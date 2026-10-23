//! Hard edges: the quad mesh has edges along every one of them, and a
//! vertex on every corner.

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::{Guides, PolyMesh, RemeshConfig, Target, TriMesh};

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

#[test]
fn the_report_lists_the_edges_that_follow_a_hard_edge() {
    let source = primitives::cube(8);
    let (mesh, report) = RemeshConfig::for_hard_surface(Target::QuadCount(600)).remesh(&source).unwrap();
    // The twelve edges of the cube, ten quads long each.
    let along_an_edge_of_the_cube = |[a, b]: [u32; 2]| {
        let (p, q) = (mesh.positions[a as usize], mesh.positions[b as usize]);
        // Two coordinates at plus or minus one, for both ends, and the same ones.
        (0..3).filter(|&k| (p[k].abs() - 1.0).abs() < 1e-9 && (p[k] - q[k]).abs() < 1e-9).count() == 2
    };
    assert!(!report.hard_edges.is_empty());
    assert!(report.hard_edges.iter().all(|&edge| along_an_edge_of_the_cube(edge)));
    let all: usize = mesh.edge_uses().into_keys().filter(|&(a, b)| along_an_edge_of_the_cube([a, b])).count();
    assert_eq!(report.hard_edges.len(), all);
    assert_eq!(all % 12, 0);

    // A shape without a hard edge has none to list.
    let (_, smooth) = RemeshConfig::for_organic(Target::QuadCount(300)).remesh(&primitives::icosphere(4)).unwrap();
    assert!(smooth.hard_edges.is_empty());
}

#[test]
fn edges_kept_for_the_shape_are_followed_and_come_out_as_hard_edges() {
    // A cube under the organic preset, which keeps no crease: its edges are
    // rounded off. With an edge angle they keep the work mesh crisp and the
    // quads follow them, as lines that may be let go; where they are kept,
    // as they all are here, they are hard edges of the result like any.
    let source = primitives::cube(12);
    let crisp = RemeshConfig::for_organic(Target::QuadCount(600));
    let round = RemeshConfig { edge_angle: None, ..crisp.clone() };
    let (rounded, _) = round.remesh_guided(&source, &Guides::default()).unwrap();
    let (mesh, report) = crisp.remesh_guided(&source, &Guides::default()).unwrap();
    assert!(measure(&source, &mesh, Some(600)).is_valid());
    assert!(!report.fell_back && report.guides_let_go == 0);
    assert!(report.hard_edges.len() >= 100, "{} hard edges", report.hard_edges.len());
    assert!(report.guide_edges.is_empty() && report.creases.len() >= 100, "{} creases", report.creases.len());
    let off = |mesh: &PolyMesh| measure(&source, mesh, Some(600)).hausdorff;
    assert!(off(&mesh) < 1e-6, "the cube is {} away", off(&mesh));
    assert!(off(&rounded) > 100.0 * off(&mesh).max(1e-9), "rounded {} against crisp {}", off(&rounded), off(&mesh));
}
