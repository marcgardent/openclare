//! Guide curves: lines drawn on the surface that the quad mesh has edges
//! along.

use std::collections::HashMap;

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::preprocess::{self, Kind};
use openclare::{GuideCurve, Guides, PolyMesh, RemeshConfig, Report, Target, TriMesh};

fn remesh(source: &TriMesh, quads: usize, curves: &[GuideCurve]) -> (PolyMesh, Report) {
    let config = RemeshConfig { adaptivity: 0.0, ..RemeshConfig::for_organic(Target::QuadCount(quads)) };
    config.remesh_guided(source, &Guides { curves, ..Default::default() }).unwrap()
}

/// The vertices of `edges`, each with the vertices it is joined to.
fn joined(edges: &[[u32; 2]]) -> HashMap<u32, Vec<u32>> {
    let mut joined: HashMap<u32, Vec<u32>> = HashMap::new();
    for &[a, b] in edges {
        joined.entry(a).or_default().push(b);
        joined.entry(b).or_default().push(a);
    }
    joined
}

/// A circle of latitude on the unit sphere, at height `z`.
fn parallel(z: f64) -> GuideCurve {
    let radius = (1.0 - z * z).sqrt();
    let points = (0..48)
        .map(|k| {
            let turn = std::f64::consts::TAU * k as f64 / 48.0;
            Vec3::new(radius * turn.cos(), radius * turn.sin(), z)
        })
        .collect();
    GuideCurve { points, closed: true }
}

#[test]
fn a_curve_is_laid_into_the_work_mesh_as_one_unbroken_line() {
    let source = primitives::icosphere(5);
    let curves = [parallel(0.3)];
    let request = preprocess::Request { triangles: 8000, curves: &curves, ..Default::default() };
    let work = preprocess::create(Kind::Isotropic).preprocess(&source, &request).unwrap();
    assert_eq!(work.guide_gaps, 0);
    assert!(!work.guide_lines.is_empty());
    // A closed line: every vertex on it has two neighbours on it, and they
    // all sit on the circle.
    let line = joined(&work.feature_edges);
    assert!(line.len() > 40, "{} vertices on the line", line.len());
    for (&v, neighbours) in &line {
        let p = work.mesh.positions[v as usize];
        assert_eq!(neighbours.len(), 2, "vertex {v} at {p:?}");
        assert!((p.z - 0.3).abs() < 2e-3 && (p.length() - 1.0).abs() < 2e-3, "vertex {v} at {p:?}");
    }
    // And the mesh is still a closed surface.
    assert_eq!(work.mesh.pieces().len(), 1);
    work.mesh.validate().unwrap();
}

#[test]
fn the_quad_mesh_has_an_edge_loop_on_a_closed_curve() {
    let source = primitives::icosphere(5);
    let (free, _) = remesh(&source, 600, &[]);
    let (mesh, report) = remesh(&source, 600, &[parallel(0.3)]);
    assert!(!report.fell_back, "the layout chain gave up: {:?}", report.fallback_causes);
    assert!(measure(&source, &mesh, Some(600)).is_valid());
    assert_eq!(report.guide_gaps, 0);

    // The edges along the curve close into one loop, on the circle.
    let line = joined(&report.guide_edges);
    assert!(line.len() >= 12, "{} vertices on the loop", line.len());
    for (&v, neighbours) in &line {
        let p = mesh.positions[v as usize];
        assert_eq!(neighbours.len(), 2, "vertex {v} at {p:?}");
        assert!((p.z - 0.3).abs() < 5e-3, "vertex {v} at {p:?}");
    }
    // Without the curve nothing of the sort is there.
    let on_the_circle = |mesh: &PolyMesh| mesh.positions.iter().filter(|p| (p.z - 0.3).abs() < 5e-3).count();
    assert!(on_the_circle(&free) < line.len() / 2);
    assert_eq!(on_the_circle(&mesh), line.len());
    // A line drawn is not a hard edge: nothing to mark sharp or to crease.
    assert!(report.hard_edges.is_empty() && report.creases.is_empty());
}

#[test]
fn an_open_curve_gives_a_line_of_edges_from_end_to_end() {
    // A line across a flat sheet, off its own grid.
    let source = primitives::plane(24);
    let curve = GuideCurve { points: vec![Vec3::new(-0.61, 0.13, 0.0), Vec3::new(0.57, 0.13, 0.0)], closed: false };
    let (mesh, report) = remesh(&source, 400, &[curve]);
    assert!(measure(&source, &mesh, Some(400)).is_valid());
    let line = joined(&report.guide_edges);
    let xs: Vec<f64> = line.keys().map(|&v| mesh.positions[v as usize].x).collect();
    for &v in line.keys() {
        assert!((mesh.positions[v as usize].y - 0.13).abs() < 1e-6);
    }
    let (low, high) = (xs.iter().copied().fold(f64::INFINITY, f64::min), xs.iter().copied().fold(-1.0, f64::max));
    assert!(low < -0.5 && high > 0.45, "the line runs from {low} to {high}");
    // One line: two ends, every other vertex with two neighbours.
    assert_eq!(line.values().filter(|n| n.len() == 1).count(), 2);
    assert!(line.values().all(|n| n.len() <= 2));
}

#[test]
fn two_curves_that_cross_share_a_vertex() {
    let source = primitives::plane(24);
    let across = GuideCurve { points: vec![Vec3::new(-0.8, 0.13, 0.0), Vec3::new(0.8, 0.13, 0.0)], closed: false };
    let up = GuideCurve { points: vec![Vec3::new(-0.21, -0.8, 0.0), Vec3::new(-0.21, 0.8, 0.0)], closed: false };
    let (mesh, report) = remesh(&source, 400, &[across, up]);
    assert!(measure(&source, &mesh, Some(400)).is_valid());
    let line = joined(&report.guide_edges);
    let crossings: Vec<Vec3> =
        line.iter().filter(|(_, n)| n.len() == 4).map(|(&v, _)| mesh.positions[v as usize]).collect();
    assert_eq!(crossings.len(), 1, "{crossings:?}");
    assert!((crossings[0] - Vec3::new(-0.21, 0.13, 0.0)).length() < 1e-6, "{:?}", crossings[0]);
}

#[test]
fn a_curve_away_from_the_surface_changes_nothing() {
    let source = primitives::icosphere(4);
    let far = GuideCurve { points: vec![Vec3::new(5.0, 0.0, 0.0), Vec3::new(5.0, 1.0, 0.0)], closed: false };
    // It is brought back onto the surface, where it is one short line.
    let (mesh, report) = remesh(&source, 300, &[far]);
    assert!(measure(&source, &mesh, Some(300)).is_valid());
    assert!(report.guide_edges.len() < 12);
}
