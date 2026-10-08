use std::f64::consts::FRAC_PI_2;

use openclare::TriMesh;
use openclare::crossfield::{CrossField, FaceFrames, Kind, Request, alignment_error, create};
use openclare::math::Vec3;
use openclare::mesh::{Topology, primitives};
use openclare::preprocess::{self, WorkMesh};

fn work_mesh(source: &TriMesh, triangles: usize, crease_angle: Option<f64>) -> WorkMesh {
    let request = preprocess::Request { triangles, crease_angle, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap()
}

fn as_is(source: &TriMesh) -> WorkMesh {
    preprocess::create(preprocess::Kind::Passthrough).preprocess(source, &Default::default()).unwrap()
}

fn solve(kind: Kind, work: &WorkMesh, alignment: f64) -> CrossField {
    create(kind).solve(work, &Request { alignment, ..Default::default() }).unwrap()
}

fn frames_of(work: &WorkMesh) -> FaceFrames {
    FaceFrames::build(&work.mesh, &Topology::build(&work.mesh))
}

/// Angle between two crosses given by one arm each, in degrees, in [0, 45].
fn cross_angle(a: Vec3, b: Vec3, normal: Vec3) -> f64 {
    let angle = a.cross(b).dot(normal).atan2(a.dot(b));
    (angle - FRAC_PI_2 * (angle / FRAC_PI_2).round()).abs().to_degrees()
}

#[test]
fn indices_add_up_to_the_euler_characteristic_whatever_the_solver() {
    let shapes = [(as_is(&primitives::icosphere(4)), 8), (as_is(&primitives::torus(1.0, 0.4, 64, 32)), 0)];
    for (work, four_chi) in &shapes {
        for &kind in Kind::ALL {
            for alignment in [0.0, 0.2] {
                let field = solve(kind, work, alignment);
                assert_eq!(field.total_index(), *four_chi, "{kind} with alignment {alignment}");
                assert!(field.angles.iter().all(|a| a.abs() <= FRAC_PI_2 / 2.0 + 1e-12));
            }
        }
    }
}

#[test]
fn the_smoothest_field_on_a_sphere_has_the_eight_singularities_it_must_have() {
    let sphere = solve(Kind::Power, &as_is(&primitives::icosphere(4)), 0.0);
    assert_eq!(sphere.singularities(), (8, 0, 0));
}

#[test]
fn smoothest_does_not_mean_fewest_singularities() {
    // A torus needs no singularity, yet on a fat one the smoothest field
    // has some: eight where the surface bulges out and eight where it curves
    // in soak up the curvature and cost less than bending the crosses all
    // the way round. Following curvature is what removes them.
    let work = as_is(&primitives::torus(1.0, 0.4, 64, 32));
    let (positive, negative, other) = solve(Kind::Power, &work, 0.0).singularities();
    assert_eq!((positive, other), (negative, 0));
    assert!(positive > 0);
    assert_eq!(solve(Kind::Power, &work, 0.05).singularities(), (0, 0, 0));

    // A thin torus is nearly a bent cylinder: nothing to soak up.
    let thin = as_is(&primitives::torus(1.0, 0.1, 128, 16));
    assert_eq!(solve(Kind::Power, &thin, 0.0).singularities(), (0, 0, 0));
}

#[test]
fn period_jumps_are_opposite_on_the_two_sides_of_an_edge() {
    let work = as_is(&primitives::icosphere(3));
    let topology = Topology::build(&work.mesh);
    let field = solve(Kind::Power, &work, 0.0);
    for h in 0..field.period_jumps.len() {
        let sum = field.period_jumps[h] + field.period_jumps[topology.opposite[h] as usize];
        assert_eq!(sum.rem_euclid(4), 0);
    }
}

#[test]
fn alignment_follows_curvature_on_a_torus_without_adding_singularities() {
    let work = work_mesh(&primitives::torus(1.0, 0.4, 96, 48), 8000, None);
    let frames = frames_of(&work);
    let field = solve(Kind::Power, &work, 0.05);
    assert_eq!(field.singularities(), (0, 0, 0));
    let error = alignment_error(&work, &frames, &field).unwrap();
    assert!(error < 2.0, "{error} degrees off the principal directions");
}

#[test]
fn alignment_follows_curvature_on_an_ellipsoid() {
    let mut ellipsoid = primitives::icosphere(5);
    for p in &mut ellipsoid.positions {
        *p *= Vec3::new(2.0, 1.0, 0.6);
    }
    let work = work_mesh(&ellipsoid, 10000, None);
    let frames = frames_of(&work);

    let free = solve(Kind::Power, &work, 0.0);
    let aligned = solve(Kind::Power, &work, 0.2);
    let (free_error, aligned_error) =
        (alignment_error(&work, &frames, &free).unwrap(), alignment_error(&work, &frames, &aligned).unwrap());
    assert!(aligned_error < 5.0, "{aligned_error} degrees with alignment");
    assert!(aligned_error < 0.5 * free_error, "alignment should help: {free_error} -> {aligned_error}");
    assert_eq!(aligned.total_index(), 8);
}

#[test]
fn a_cube_gets_its_eight_singularities_on_its_corners() {
    let work = work_mesh(&primitives::cube(4), 6000, Some(30.0));
    let frames = frames_of(&work);
    let field = solve(Kind::Power, &work, 0.0);

    assert_eq!(field.singularities(), (8, 0, 0));
    for (v, &index) in field.indices.iter().enumerate() {
        let on_corner = work.mesh.positions[v].abs().to_array().iter().all(|c| (c - 1.0).abs() < 1e-9);
        assert_eq!(index == 1, on_corner, "vertex {v} at {}", work.mesh.positions[v]);
    }
    // Every cross has an arm along a cube axis.
    for f in 0..work.mesh.triangles.len() {
        let normal = frames.normal[f];
        let axis = [Vec3::X, Vec3::Y, Vec3::Z].into_iter().find(|a| a.dot(normal).abs() < 0.5).unwrap();
        let off = cross_angle(field.direction(&frames, f), axis, normal);
        assert!(off < 1.0, "face {f} is {off} degrees off the cube axes");
    }
}

#[test]
fn an_open_sheet_gets_a_field_parallel_to_its_border() {
    let work = work_mesh(&primitives::plane(6), 3000, None);
    let frames = frames_of(&work);
    let field = solve(Kind::Power, &work, 0.0);
    assert_eq!(field.singularities(), (0, 0, 0));
    for f in 0..work.mesh.triangles.len() {
        let off = cross_angle(field.direction(&frames, f), Vec3::X, Vec3::Z);
        assert!(off < 1.0, "face {f} is {off} degrees off the border directions");
    }
}

#[test]
fn solvers_are_reachable_by_name_deterministic_and_refuse_non_manifold_meshes() {
    let work = as_is(&primitives::torus(1.0, 0.4, 32, 16));
    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        assert_eq!(create(kind).kind(), kind);
        let (first, second) = (solve(kind, &work, 0.1), solve(kind, &work, 0.1));
        assert_eq!(first.angles, second.angles);
        assert_eq!(first.indices, second.indices);
    }
    assert!("nope".parse::<Kind>().is_err());

    // Three triangles on one edge.
    let fan = TriMesh {
        positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y, Vec3::Z, Vec3::new(0.0, -1.0, 0.0)],
        triangles: vec![[0, 1, 2], [0, 1, 3], [0, 1, 4]],
    };
    let work = as_is(&fan);
    for &kind in Kind::ALL {
        assert!(create(kind).solve(&work, &Request::default()).is_err());
    }
}
