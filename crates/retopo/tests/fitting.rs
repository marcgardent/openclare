//! Fitting: the mesh keeps the volume of the shape instead of being
//! inscribed in it.

use retopo::mesh::primitives;
use retopo::metrics::measure;
use retopo::{RemeshConfig, Target, TriMesh};

fn remesh(source: &TriMesh, quads: usize, volume_fit: f64) -> (retopo::PolyMesh, retopo::metrics::Metrics) {
    let config = RemeshConfig { adaptivity: 0.0, volume_fit, ..RemeshConfig::for_organic(Target::QuadCount(quads)) };
    let (mesh, _) = config.remesh(source).unwrap();
    let metrics = measure(source, &mesh, Some(quads));
    (mesh, metrics)
}

#[test]
fn with_its_vertices_on_the_surface_a_mesh_is_smaller_than_the_shape() {
    let source = primitives::icosphere(5);
    let (_, inscribed) = remesh(&source, 300, 0.0);
    let kept = inscribed.volume_ratio.unwrap();
    assert!(kept < 0.985, "volume kept: {kept}");
}

#[test]
fn a_fitted_mesh_keeps_the_volume() {
    for (name, source, quads) in
        [("ball", primitives::icosphere(5), 300), ("ring", primitives::torus(1.0, 0.4, 96, 48), 400)]
    {
        let (on_surface, inscribed) = remesh(&source, quads, 0.0);
        let (fitted, fit) = remesh(&source, quads, 1.0);
        assert!(fit.is_valid(), "{name}");
        // The same mesh, its vertices moved.
        assert_eq!(fitted.faces, on_surface.faces, "{name}");
        let (before, after) = (inscribed.volume_ratio.unwrap(), fit.volume_ratio.unwrap());
        assert!((after - 1.0).abs() < 0.004, "{name}: volume kept {after} (it was {before})");
        assert!((after - 1.0).abs() < 0.3 * (before - 1.0).abs(), "{name}: volume kept {after} (it was {before})");
        // No farther from the shape for that: closer on the whole.
        assert!(
            fit.mean_distance < inscribed.mean_distance,
            "{name}: {} then {}",
            inscribed.mean_distance,
            fit.mean_distance
        );
        assert!(fit.hausdorff < 1.05 * inscribed.hausdorff, "{name}: {} then {}", inscribed.hausdorff, fit.hausdorff);
    }
}

#[test]
fn half_way_is_half_way() {
    let source = primitives::icosphere(5);
    let (on_surface, _) = remesh(&source, 300, 0.0);
    let (half, _) = remesh(&source, 300, 0.5);
    let (fitted, _) = remesh(&source, 300, 1.0);
    for v in 0..on_surface.positions.len() {
        let middle = 0.5 * (on_surface.positions[v] + fitted.positions[v]);
        assert!(half.positions[v].distance(middle) < 1e-12);
    }
}

#[test]
fn flat_faces_and_hard_edges_are_left_where_they_are() {
    let source = primitives::cube(8);
    let base = RemeshConfig::for_hard_surface(Target::QuadCount(600));
    assert_eq!(base.volume_fit, 1.0);
    let (fitted, _) = base.remesh(&source).unwrap();
    let (exact, _) = RemeshConfig { volume_fit: 0.0, ..base }.remesh(&source).unwrap();
    assert_eq!(fitted.faces, exact.faces);
    for v in 0..exact.positions.len() {
        assert!(fitted.positions[v].distance(exact.positions[v]) < 1e-9);
    }
}
