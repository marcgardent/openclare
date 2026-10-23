//! Creases: the edges that should stay sharp when the mesh is subdivided.

use std::collections::HashMap;

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::{RemeshConfig, Target, TriMesh};

/// A lens: two spherical caps of radius `bulge` back to back, meeting along
/// the unit circle in a fold with no hard edge marked on it by an angle
/// threshold when the preset keeps none.
fn lens(bulge: f64) -> TriMesh {
    let mut mesh = primitives::icosphere(5);
    let depth = (bulge * bulge - 1.0).sqrt();
    for p in &mut mesh.positions {
        // Each half of the ball onto its cap, centred on the other side.
        let side = if p.z >= 0.0 { 1.0 } else { -1.0 };
        let flat = Vec3::new(p.x, p.y, 0.0);
        let radius = flat.length().min(1.0);
        let height = (bulge * bulge - radius * radius).sqrt() - depth;
        *p = Vec3::new(p.x, p.y, side * height);
        if radius >= 1.0 {
            *p = flat.normalize() * 1.0;
        }
    }
    mesh
}

#[test]
fn hard_edges_are_creased_all_the_way() {
    let source = primitives::cube(8);
    let (mesh, report) = RemeshConfig::for_hard_surface(Target::QuadCount(600)).remesh(&source).unwrap();
    let creases: HashMap<[u32; 2], f64> = report.creases.iter().copied().collect();
    assert!(!report.hard_edges.is_empty());
    for edge in &report.hard_edges {
        assert_eq!(creases.get(edge), Some(&1.0));
    }
    // And nothing else on a cube: its faces are flat.
    assert_eq!(creases.len(), report.hard_edges.len(), "{} creases on {} edges", creases.len(), mesh.edge_uses().len());
}

#[test]
fn a_smooth_shape_has_no_crease() {
    for source in [primitives::icosphere(4), primitives::torus(1.0, 0.4, 64, 32)] {
        let (_, report) = RemeshConfig::for_organic(Target::QuadCount(400)).remesh(&source).unwrap();
        assert!(report.creases.is_empty(), "{:?}", &report.creases[..report.creases.len().min(5)]);
    }
}

#[test]
fn a_fold_of_the_source_is_creased_where_edges_run_along_it() {
    // Caps of radius 1.25: they meet at an angle of about 106 degrees.
    let source = lens(1.25);
    let config = RemeshConfig { edge_angle: None, ..RemeshConfig::for_organic(Target::QuadCount(600)) };
    let (mesh, report) = config.remesh(&source).unwrap();
    assert!(report.hard_edges.is_empty(), "without an edge angle, no hard edge is kept");
    let on_the_rim = |v: u32| {
        let p = mesh.positions[v as usize];
        p.z.abs() < 0.05 && (p.truncate().length() - 1.0).abs() < 0.08
    };
    let rim: Vec<f64> =
        report.creases.iter().filter(|&&([a, b], _)| on_the_rim(a) && on_the_rim(b)).map(|&(_, w)| w).collect();
    let elsewhere = report.creases.len() - rim.len();
    assert!(rim.len() >= 8, "{} creased edges on the rim, {elsewhere} elsewhere", rim.len());
    assert!(rim.iter().sum::<f64>() / rim.len() as f64 > 0.5, "weights on the rim: {rim:?}");
    assert!(elsewhere <= rim.len() / 2, "{} creased edges on the rim, {elsewhere} elsewhere", rim.len());
}
