use openclare::crossfield::{self, CrossField, FaceFrames, alignment_error};
use openclare::math::Vec3;
use openclare::mesh::{Topology, primitives};
use openclare::preprocess::{self, WorkMesh};
use openclare::reflection::{Element, Kind as Fault, Settings, check};
use openclare::singularity::{Bend, Kind, Report, Request, Slide, create};
use openclare::{PolyMesh, TriMesh};

fn as_is(source: &TriMesh) -> WorkMesh {
    preprocess::create(preprocess::Kind::Passthrough).preprocess(source, &Default::default()).unwrap()
}

fn field(work: &WorkMesh, alignment: f64) -> CrossField {
    let request = crossfield::Request { alignment, ..Default::default() };
    crossfield::create(crossfield::Kind::Power).solve(work, &request).unwrap()
}

fn cancel(work: &WorkMesh, field: &CrossField, alignment: f64, tolerance: f64) -> (CrossField, Report) {
    cancel_within(work, field, alignment, tolerance, None)
}

fn cancel_within(
    work: &WorkMesh,
    field: &CrossField,
    alignment: f64,
    tolerance: f64,
    reach: Option<f64>,
) -> (CrossField, Report) {
    let request = Request { alignment, tolerance, reach, slide: None };
    create(Kind::PairCancellation).optimize(work, field, &request).unwrap()
}

fn count(field: &CrossField) -> usize {
    let (positive, negative, other) = field.singularities();
    positive + negative + other
}

/// A sphere with bumps: plenty of curvature directions to follow, hence
/// plenty of singularities once the field is told to follow them.
fn bumpy_sphere() -> WorkMesh {
    let mut mesh = primitives::icosphere(5);
    for p in &mut mesh.positions {
        *p *= 1.0 + 0.06 * (7.0 * p.x).sin() * (9.0 * p.y).sin() * (5.0 * p.z).cos();
    }
    let request = preprocess::Request { triangles: 12_000, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(&mesh, &request).unwrap()
}

#[test]
fn a_torus_can_shed_every_singularity() {
    // The smoothest field on a fat torus has eight pairs (see the cross
    // field tests), none of which the topology requires.
    let work = as_is(&primitives::torus(1.0, 0.4, 64, 32));
    let smoothest = field(&work, 0.0);
    assert_eq!(smoothest.singularities(), (8, 8, 0));

    let (cleaned, report) = cancel(&work, &smoothest, 0.0, 10.0);
    assert_eq!(cleaned.singularities(), (0, 0, 0));
    assert_eq!((report.singularities_before, report.singularities_after, report.cancelled), (16, 0, 8));
    assert_eq!(cleaned.total_index(), 0);
}

#[test]
fn singularities_the_topology_requires_are_left_alone() {
    let work = as_is(&primitives::icosphere(4));
    let smoothest = field(&work, 0.0);
    let (cleaned, report) = cancel(&work, &smoothest, 0.0, 10.0);
    assert_eq!(cleaned.singularities(), (8, 0, 0));
    assert_eq!(report.tried, 0);
}

#[test]
fn nearby_pairs_go_for_nearly_the_same_alignment() {
    let work = bumpy_sphere();
    let frames = FaceFrames::build(&work.mesh, &Topology::build(&work.mesh));
    let alignment = 0.3;
    let before = field(&work, alignment);
    // The bumps are about 0.4 apart: pairs closer than that are the ones
    // sitting on a single bump.
    let (after, report) = cancel_within(&work, &before, alignment, 1.0, Some(0.3));

    assert!(count(&before) > 20, "the test shape should be busy: {}", count(&before));
    assert!(count(&after) < count(&before), "{} -> {}", count(&before), count(&after));
    assert_eq!(report.singularities_after, count(&after));
    assert_eq!(count(&before) - count(&after), 2 * report.cancelled);
    // The topology still gets its due, with simple singularities only.
    let (positive, negative, other) = after.singularities();
    assert_eq!((positive - negative, other), (8, 0));
    assert_eq!(after.total_index(), 8);

    let (error_before, error_after) =
        (alignment_error(&work, &frames, &before).unwrap(), alignment_error(&work, &frames, &after).unwrap());
    // The field gives up some alignment for it, but stays far from random (22.5).
    assert!(error_after < error_before + 4.0, "alignment {error_before} -> {error_after} degrees");
}

#[test]
fn reach_and_tolerance_both_limit_what_is_removed() {
    let work = bumpy_sphere();
    let before = field(&work, 0.3);
    let removed = |tolerance: f64, reach: Option<f64>| {
        let (after, _) = cancel_within(&work, &before, 0.3, tolerance, reach);
        assert_eq!(after.total_index(), 8);
        count(&before) - count(&after)
    };
    let everything = removed(10.0, None);
    assert_eq!(everything, count(&before) - 8, "with no limit only the eight required ones stay");
    assert!(removed(10.0, Some(0.3)) < everything);
    assert!(removed(0.0, None) < everything);
    assert_eq!(removed(10.0, Some(0.0)), 0);
}

#[test]
fn a_field_with_nothing_to_remove_comes_back_untouched() {
    let work = bumpy_sphere();
    let before = field(&work, 0.3);
    let (after, report) = cancel_within(&work, &before, 0.3, 1.0, Some(0.0));
    assert_eq!((report.cancelled, report.split), (0, 0));
    assert_eq!(after.angles, before.angles);
    assert_eq!(after.period_jumps, before.period_jumps);
}

#[test]
fn the_result_is_a_consistent_field() {
    let work = bumpy_sphere();
    let topology = Topology::build(&work.mesh);
    let (after, _) = cancel(&work, &field(&work, 0.3), 0.3, 0.05);
    for h in 0..after.period_jumps.len() {
        let sum = after.period_jumps[h] + after.period_jumps[topology.opposite[h] as usize];
        assert_eq!(sum.rem_euclid(4), 0);
    }
    assert!(after.angles.iter().all(|a| a.abs() <= std::f64::consts::FRAC_PI_4 + 1e-12));
}

#[test]
fn optimizers_are_reachable_by_name_and_keep_changes_nothing() {
    let work = as_is(&primitives::torus(1.0, 0.4, 32, 16));
    let before = field(&work, 0.0);
    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        assert_eq!(create(kind).kind(), kind);
    }
    assert!("nope".parse::<Kind>().is_err());

    let (kept, report) = create(Kind::Keep).optimize(&work, &before, &Request::default()).unwrap();
    assert_eq!(kept.angles, before.angles);
    assert_eq!(kept.indices, before.indices);
    assert_eq!(report.cancelled, 0);

    // Same input, same output.
    let (first, _) = cancel(&work, &before, 0.0, 1.0);
    let (second, _) = cancel(&work, &before, 0.0, 1.0);
    assert_eq!(first.angles, second.angles);
    assert_eq!(first.indices, second.indices);
}

/// The vertex of `positions` nearest to `point`.
fn nearest(positions: &[Vec3], point: Vec3) -> usize {
    (0..positions.len()).min_by(|&a, &b| positions[a].distance(point).total_cmp(&positions[b].distance(point))).unwrap()
}

/// The angle the reflection check gives the pole at `vertex` of `mesh`,
/// when it finds it on a curved zone.
fn pole_angle(mesh: &PolyMesh, vertex: usize) -> Option<f64> {
    check(mesh, &[], &Settings::default())
        .faults
        .iter()
        .find(|fault| fault.kind == Fault::CurvedPole && fault.at == Element::Vertex(vertex as u32))
        .map(|fault| fault.value)
}

#[test]
fn a_pole_costs_what_the_reflection_check_gives_it_on_an_ellipsoid() {
    // The spheroid x² + y² + (z / 2)² = 1. Its principal curvatures at a
    // point of polar angle t (x = sin t, z = 2 cos t) are 2 / D^(3/2) along
    // the meridian and 2 / D^(1/2) along the parallel, D = cos² t + 4 sin² t:
    // 2 and 2 at the tip (t = 0); at cos t = 1/3, where D = 33 / 9, 0.285
    // and 1.044. A pole is charged the largest curvature within one quad
    // edge of it: 2 at the tip; at cos t = 1/3, with quads of edge 0.1, the
    // largest is 0.1 / √D = 0.052 radians of t towards the tip, where
    // D = 3.561 and the curvature along the parallel 2 / √D = 1.060. A pole
    // at the tip costs 2 / 1.060 = 1.89 times as much.
    let spheroid = |p: Vec3| Vec3::new(p.x, p.y, 2.0 * p.z);
    let tip = Vec3::new(0.0, 0.0, 2.0);
    let side = Vec3::new(8f64.sqrt() / 3.0, 0.0, 2.0 / 3.0);

    // The cost, read on a fine triangulation, for quads of edge 0.1.
    let mut sphere = primitives::icosphere(5);
    for p in &mut sphere.positions {
        *p = spheroid(*p);
    }
    let work = as_is(&sphere);
    let edge = 0.1;
    let bend = Bend::new(&work, edge);
    let at_tip = bend.at(nearest(&work.mesh.positions, tip));
    let at_side = bend.at(nearest(&work.mesh.positions, side));
    assert!((at_tip / edge - 2.0).abs() < 0.2, "{}", at_tip / edge);
    assert!((at_side / edge - 1.060).abs() < 0.1, "{}", at_side / edge);

    // The check, on a quad sphere on the spheroid, turned so that a corner
    // of its cube, (1, 1, 1), is the tip, and another, (1, 1, -1), at
    // cos t = 1/3: two poles of valence 3.
    let mut cage = primitives::cube_quads(12);
    let ez = Vec3::new(1.0, 1.0, 1.0).normalize();
    let other = Vec3::new(1.0, 1.0, -1.0).normalize();
    let ex = (other - ez * other.dot(ez)).normalize();
    let ey = ez.cross(ex);
    for p in &mut cage.positions {
        let q = p.normalize();
        *p = spheroid(Vec3::new(q.dot(ex), q.dot(ey), q.dot(ez)));
    }
    let (tip_pole, side_pole) = (nearest(&cage.positions, tip), nearest(&cage.positions, side));
    assert!(cage.positions[tip_pole].distance(tip) < 1e-9 && cage.positions[side_pole].distance(side) < 1e-9);
    let (check_tip, check_side) = (pole_angle(&cage, tip_pole).unwrap(), pole_angle(&cage, side_pole).unwrap());
    // Both rank the tip first, by a like margin.
    assert!(check_tip > 1.5 * check_side, "{check_tip} {check_side}");
    assert!(at_tip > 1.5 * at_side);
}

#[test]
fn a_pole_on_the_flat_of_a_cube_costs_nothing_and_on_its_corner_much() {
    // On the flat of a face the surface does not bend, and the check finds
    // no pole on a curved zone there (see the reflection tests: a valence 5
    // on a plane is no fault). At a corner the check gives a pole of
    // valence 3 acos(1 / √3) = 54.7 degrees, and the surface bends most.
    let work = as_is(&primitives::cube(8));
    let positions = &work.mesh.positions;
    let bend = Bend::new(&work, 0.25);
    let flat = bend.at(nearest(positions, Vec3::new(0.0, 0.0, 1.0)));
    let corner = bend.at(nearest(positions, Vec3::new(1.0, 1.0, 1.0)));
    assert!(flat < 1e-9, "{flat}");
    assert!(corner > 0.1, "{corner}");
    let cage = primitives::cube_quads(4);
    let at_corner = pole_angle(&cage, nearest(&cage.positions, Vec3::new(1.0, 1.0, 1.0))).unwrap();
    assert!((at_corner - (1.0 / 3f64.sqrt()).acos().to_degrees()).abs() < 1e-9, "{at_corner}");
}

#[test]
fn on_an_ellipsoid_the_poles_slide_from_its_tips_towards_its_waist() {
    // The field puts its eight poles where the curvature of the spheroid
    // x² + y² + (z / 2)² = 1 gathers, round its two tips; the surface bends
    // there by 2, at the waist by 1 (see above). Slid, they go down
    // towards the waist, each by at most a few quads, and stay eight.
    let mut sphere = primitives::icosphere(5);
    for p in &mut sphere.positions {
        p.z *= 2.0;
    }
    let work = as_is(&sphere);
    let before = field(&work, 0.05);
    let edge = 0.12;
    // The tuning the bench was measured with, the alignment not held apart.
    let tuning = Slide {
        edge_length: edge,
        reach: 3.0,
        gain: 0.95,
        tolerance: 0.25,
        alignment_tolerance: f64::INFINITY,
        tries: 3,
    };
    let slide = |slide| {
        let request = Request { alignment: 0.05, tolerance: 1.0, reach: Some(0.0), slide };
        create(Kind::PairCancellation).optimize(&work, &before, &request).unwrap()
    };
    let height = |field: &CrossField| -> f64 {
        let singular: Vec<usize> = (0..field.indices.len()).filter(|&v| field.indices[v] != 0).collect();
        singular.iter().map(|&v| work.mesh.positions[v].z.abs()).sum::<f64>() / singular.len() as f64
    };
    let (kept, report) = slide(None);
    assert_eq!((report.slid, kept.angles == before.angles), (0, true));
    let (slid, report) = slide(Some(tuning));
    assert_eq!(slid.singularities(), (8, 0, 0));
    assert!(report.slid >= 4, "{report:?}");
    assert!(height(&before) > 1.85, "the poles are not round the tips: {}", height(&before));
    assert!(height(&slid) < height(&before) - 0.05, "{} -> {}", height(&before), height(&slid));
}
