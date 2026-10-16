//! Symmetry: a model the same on both sides of a plane through its origin
//! gets a quad mesh the same on both sides of it, closed where the model
//! is, without a seam (DT-028).

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
    assert!(metrics.budget_error.unwrap() <= 0.06, "{axis}: {} faces for {quads}", metrics.faces);
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
fn half_keeps_the_positive_side_cut_exactly_on_the_plane() {
    let cube = primitives::cube(5);
    let half = symmetry::half(&cube, Axis::X, 0.0);
    assert!(half.positions.iter().all(|p| p.x >= 0.0));
    // Half the surface: the cut falls in the middle of a row of triangles.
    let ratio = half.surface_area() / cube.surface_area();
    assert!((ratio - 0.5).abs() < 1e-9, "{ratio}");
    // Its border is the square of the cut, on the plane.
    let mut uses = std::collections::HashMap::new();
    for tri in &half.triangles {
        for k in 0..3 {
            let (a, b) = (tri[k], tri[(k + 1) % 3]);
            *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
        }
    }
    let border: Vec<Vec3> = uses
        .iter()
        .filter(|&(_, &count)| count == 1)
        .flat_map(|(&(a, b), _)| [half.positions[a as usize], half.positions[b as usize]])
        .collect();
    assert!(!border.is_empty() && border.iter().all(|p| p.x == 0.0));
}

#[test]
fn a_model_on_one_side_of_the_plane_is_refused() {
    let mut sphere = primitives::icosphere(3);
    for p in &mut sphere.positions {
        p.x += 5.0;
    }
    assert!(config(500, Axis::X).remesh(&sphere).is_err());
}
