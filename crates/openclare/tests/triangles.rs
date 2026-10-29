//! The triangles chain (DT-179): the isotropic triangle mesh as the result,
//! for simulation, 3D printing and collision.

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::{GuideCurve, Guides, Method, PolyMesh, RemeshConfig, Target};

fn triangles(count: usize) -> RemeshConfig {
    RemeshConfig { method: Method::Triangles, ..RemeshConfig::for_organic(Target::QuadCount(count)) }
}

/// The smallest angle of each face, in degrees.
fn least_angles(mesh: &PolyMesh) -> Vec<f64> {
    mesh.faces
        .iter()
        .map(|face| {
            let p: Vec<Vec3> = face.iter().map(|&v| mesh.positions[v as usize]).collect();
            (0..3)
                .map(|k| {
                    let (a, b) = (p[(k + 1) % 3] - p[k], p[(k + 2) % 3] - p[k]);
                    a.angle_between(b).to_degrees()
                })
                .fold(180.0, f64::min)
        })
        .collect()
}

#[test]
fn a_ball_comes_out_as_closed_triangles_close_to_equilateral() {
    let source = primitives::icosphere(4);
    let config = triangles(3000);
    let (mesh, report) = config.remesh_guided(&source, &Guides::default()).unwrap();
    assert!(mesh.faces.iter().all(|face| face.len() == 3));
    let made = mesh.faces.len() as f64;
    assert!((made / 3000.0 - 1.0).abs() < 0.1, "{made} triangles for 3000");
    let stats = mesh.stats();
    assert_eq!((stats.boundary_edges, stats.non_manifold_edges), (0, 0));
    assert_eq!(report.crossing_faces, 0);
    let angles = least_angles(&mesh);
    let mean = angles.iter().sum::<f64>() / angles.len() as f64;
    let worst = angles.iter().copied().fold(180.0, f64::min);
    assert!(mean > 50.0 && worst > 20.0, "least angle {mean:.1} on average, {worst:.1} at worst");
    assert!(measure(&source, &mesh, None).hausdorff < 0.01);
}

#[test]
fn the_hard_edges_of_a_box_are_kept_and_creased() {
    let source = primitives::cube(4);
    let config = RemeshConfig { method: Method::Triangles, ..RemeshConfig::for_hard_surface(Target::QuadCount(2000)) };
    let (mesh, report) = config.remesh_guided(&source, &Guides::default()).unwrap();
    // The twelve edges of the cube, each a line of edges of the result.
    let on_an_edge = |p: Vec3| [p.x, p.y, p.z].iter().filter(|c| (c.abs() - 1.0).abs() < 1e-6).count() >= 2;
    assert!(report.hard_edges.len() >= 12 * 8, "{} hard edges", report.hard_edges.len());
    for &[a, b] in &report.hard_edges {
        assert!(on_an_edge(mesh.positions[a as usize]) && on_an_edge(mesh.positions[b as usize]));
    }
    assert!(report.creases.iter().filter(|(_, weight)| *weight == 1.0).count() >= report.hard_edges.len());
}

#[test]
fn a_curve_drawn_on_a_ball_is_a_line_of_edges_of_the_triangles() {
    let source = primitives::icosphere(4);
    let height: f64 = 0.35;
    let radius = (1.0 - height * height).sqrt();
    let points = (0..48)
        .map(|k| {
            let turn = std::f64::consts::TAU * k as f64 / 48.0;
            Vec3::new(radius * turn.cos(), radius * turn.sin(), height)
        })
        .collect();
    let curves = [GuideCurve { points, closed: true, sharp: false }];
    let (mesh, report) =
        triangles(3000).remesh_guided(&source, &Guides { curves: &curves, ..Default::default() }).unwrap();
    assert!(report.curve_edges.len() >= 40, "{} edges along the curve", report.curve_edges.len());
    for &([a, b], curve) in &report.curve_edges {
        assert_eq!(curve, 0);
        for v in [a, b] {
            assert!((mesh.positions[v as usize].z - height).abs() < 2e-3, "{:?}", mesh.positions[v as usize]);
        }
    }
    assert_eq!(report.curve_edges.len(), report.guide_edges.len());
}
