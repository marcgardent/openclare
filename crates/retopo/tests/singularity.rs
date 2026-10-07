use retopo::TriMesh;
use retopo::crossfield::{self, CrossField, FaceFrames, alignment_error};
use retopo::mesh::{Topology, primitives};
use retopo::preprocess::{self, WorkMesh};
use retopo::singularity::{Kind, Report, Request, create};

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
    let request = Request { alignment, tolerance, reach };
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
