//! Symmetry: a model the same on both sides of a plane through its origin
//! gets a quad mesh the same on both sides of it, closed where the model
//! is, the quads along the plane as good as the others (DT-028).

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::symmetry::{self, Axis};
use openclare::{RemeshConfig, Target, TriMesh};

fn config(quads: usize, axis: Axis) -> RemeshConfig {
    RemeshConfig { symmetry: Some(axis), ..RemeshConfig::for_organic(Target::QuadCount(quads)) }
}

fn diagonal(mesh: &TriMesh) -> f64 {
    let (low, high) =
        mesh.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    (high - low).length()
}

/// Remeshes `source` symmetric across `axis` and checks the result: a
/// valid quad mesh, closed as the source is, its own mirror image, and near
/// the number of quads asked for.
fn symmetric(source: &TriMesh, axis: Axis, quads: usize) {
    let (mesh, _) = config(quads, axis).remesh(source).unwrap();
    let metrics = measure(source, &mesh, Some(quads));
    assert!(metrics.is_valid(), "{axis}: {metrics:?}");
    assert_eq!(metrics.boundary_edges, 0, "{axis}: the halves are not sewn together");
    let off = symmetry::asymmetry(&mesh, axis);
    assert!(off <= 1e-9 * diagonal(source), "{axis}: a vertex has no image, {off} away");
    // The whole model is meshed for that number; its positive side need not
    // hold exactly half.
    assert!(metrics.budget_error.unwrap() <= 0.08, "{axis}: {} faces for {quads}", metrics.faces);
}

#[test]
fn primitives_come_out_symmetric_across_each_axis() {
    let shapes = [
        ("sphere", primitives::icosphere(4)),
        ("torus", primitives::torus(1.0, 0.35, 64, 24)),
        ("cube", primitives::cube(16)),
        ("cylinder", primitives::cylinder(32)),
    ];
    for (name, shape) in &shapes {
        for &axis in Axis::ALL {
            eprintln!("{name} across {axis}");
            symmetric(shape, axis, 1000);
        }
    }
}

#[test]
fn the_plane_is_drawn_into_the_model_as_a_ring_of_edges() {
    let cube = primitives::cube(5);
    let (drawn, lines) = symmetry::split(&cube, Axis::X, 0.0);
    // Nothing added nor taken away: the same surface, cut into more pieces.
    let ratio = drawn.surface_area() / cube.surface_area();
    assert!((ratio - 1.0).abs() < 1e-9, "{ratio}");
    // The plane meets the cube in a square of side 2: its edges are drawn.
    let length: f64 = lines.iter().map(|[a, b]| a.distance(*b)).sum();
    assert!((length - 8.0).abs() < 1e-9, "{length}");
    assert!(lines.iter().flatten().all(|p| p.x == 0.0));
    // And each is an edge of the drawn mesh.
    let edges: std::collections::HashSet<(u32, u32)> = drawn
        .triangles
        .iter()
        .flat_map(|tri| (0..3).map(move |k| (tri[k].min(tri[(k + 1) % 3]), tri[k].max(tri[(k + 1) % 3]))))
        .collect();
    let at = |p: Vec3| drawn.positions.iter().position(|&q| q == p).unwrap() as u32;
    assert!(lines.iter().all(|&[a, b]| edges.contains(&(at(a).min(at(b)), at(a).max(at(b))))));
}

#[test]
fn a_model_on_one_side_of_the_plane_is_refused() {
    let mut sphere = primitives::icosphere(3);
    for p in &mut sphere.positions {
        p.x += 5.0;
    }
    assert!(config(500, Axis::X).remesh(&sphere).is_err());
}
