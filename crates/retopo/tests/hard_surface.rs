//! Hard edges: the quad mesh has edges along every one of them, and a
//! vertex on every corner.

use retopo::math::Vec3;
use retopo::mesh::primitives;
use retopo::metrics::measure;
use retopo::{PolyMesh, RemeshConfig, Target, TriMesh};

fn remesh(source: &TriMesh, quads: usize) -> PolyMesh {
    let config = RemeshConfig::for_hard_surface(Target::QuadCount(quads));
    let (mesh, report) = config.remesh(source).unwrap();
    assert!(!report.fell_back);
    let metrics = measure(source, &mesh, Some(quads));
    assert!(metrics.is_valid(), "{metrics:?}");
    assert!(metrics.budget_error.unwrap() <= 0.05, "{} faces for {quads}", metrics.faces);
    mesh
}

/// Distance from `p` to the nearest edge of `mesh`.
fn distance_to_edges(mesh: &PolyMesh, p: Vec3) -> f64 {
    let mut nearest = f64::INFINITY;
    for face in &mesh.faces {
        for k in 0..face.len() {
            let (a, b) = (mesh.positions[face[k] as usize], mesh.positions[face[(k + 1) % face.len()] as usize]);
            let along = ((p - a).dot(b - a) / (b - a).length_squared()).clamp(0.0, 1.0);
            nearest = nearest.min(p.distance(a + along * (b - a)));
        }
    }
    nearest
}

fn has_vertex_at(mesh: &PolyMesh, p: Vec3) -> bool {
    mesh.positions.iter().any(|q| q.distance(p) < 1e-6)
}

/// Outline of the L of [`primitives::l_block`], counter-clockwise.
const L: [[f64; 2]; 6] = [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [1.0, 1.0], [1.0, 2.0], [0.0, 2.0]];

#[test]
fn an_l_block_keeps_every_edge_and_corner() {
    let source = primitives::l_block(8);
    for quads in [400, 1800] {
        let mesh = remesh(&source, quads);
        let metrics = measure(&source, &mesh, Some(quads));
        // Ten corners that stick out, two that turn in: nothing else.
        assert_eq!(metrics.irregular_vertices, 12);
        assert!(metrics.hausdorff < 1e-6, "{metrics:?}");
        for z in [0.0, 1.0] {
            for k in 0..6 {
                let (a, b) = (L[k], L[(k + 1) % 6]);
                assert!(has_vertex_at(&mesh, Vec3::new(a[0], a[1], z)), "no vertex on corner {a:?} at z = {z}");
                // The edge of the L from this corner to the next, and the
                // upright edge at this corner.
                for step in 0..=20 {
                    let t = step as f64 / 20.0;
                    let on_outline = Vec3::new(a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]), z);
                    assert!(distance_to_edges(&mesh, on_outline) < 1e-6, "no mesh edge along {on_outline}");
                    let upright = Vec3::new(a[0], a[1], t);
                    assert!(distance_to_edges(&mesh, upright) < 1e-6, "no mesh edge along {upright}");
                }
            }
        }
    }
}

#[test]
fn a_cylinder_keeps_its_two_rims() {
    let source = primitives::cylinder(12);
    let mesh = remesh(&source, 1200);
    let metrics = measure(&source, &mesh, Some(1200));
    assert_eq!(metrics.irregular_vertices, 8);
    assert!(metrics.hausdorff < 0.005, "{metrics:?}");
    // The rims of the source are polygons of 48 sides, not quite regular.
    // The mesh has a closed loop of edges on each: vertices on the polygon,
    // each joined to two others of the loop.
    let inner = 0.99;
    for z in [-1.0, 1.0] {
        let on_rim = |p: Vec3| (p.z - z).abs() < 1e-9 && p.x.hypot(p.y) > inner - 1e-6 && p.x.hypot(p.y) < 1.0 + 1e-9;
        let rim: Vec<usize> = (0..mesh.positions.len()).filter(|&v| on_rim(mesh.positions[v])).collect();
        assert!(rim.len() >= 24, "{} vertices on the rim at z = {z}", rim.len());
        let mut links = vec![std::collections::BTreeSet::new(); mesh.positions.len()];
        for face in &mesh.faces {
            for k in 0..face.len() {
                let (a, b) = (face[k] as usize, face[(k + 1) % face.len()] as usize);
                if on_rim(mesh.positions[a]) && on_rim(mesh.positions[b]) {
                    links[a].insert(b);
                    links[b].insert(a);
                }
            }
        }
        for &v in &rim {
            assert_eq!(links[v].len(), 2, "rim vertex {} is not on a loop", mesh.positions[v]);
        }
    }
}

#[test]
fn two_hard_edges_close_together_keep_a_row_of_quads_between_them() {
    // The L block flattened into a plate a quarter of a quad thick: its
    // upper and lower outlines must not close up on each other.
    let mut source = primitives::l_block(8);
    for p in &mut source.positions {
        p.z *= 0.02;
    }
    let mesh = remesh(&source, 1000);
    let metrics = measure(&source, &mesh, Some(1000));
    assert!(metrics.hausdorff < 1e-6, "{metrics:?}");
    for z in [0.0, 0.02] {
        for corner in L {
            assert!(has_vertex_at(&mesh, Vec3::new(corner[0], corner[1], z)), "no vertex on {corner:?} at z = {z}");
        }
    }
    // Round the plate, one row of quads as tall as it is thick.
    let upright = mesh.faces.iter().filter(|face| {
        let z: Vec<f64> = face.iter().map(|&v| mesh.positions[v as usize].z).collect();
        z.iter().any(|&z| z < 1e-9) && z.iter().any(|&z| z > 0.02 - 1e-9)
    });
    assert!(upright.count() >= 8);
}
