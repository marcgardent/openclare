//! The quad-layout chain end to end: T-mesh, quantization, and one grid of
//! quads per rectangle, placed by reading the seamless map.

use openclare::crossfield;
use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::meshing::{self, Kind};
use openclare::metrics::measure;
use openclare::preprocess;
use openclare::quantization;
use openclare::spatial::Bvh;
use openclare::tmesh;
use openclare::{Method, RemeshConfig, Target};

fn config(target: usize) -> RemeshConfig {
    let config = RemeshConfig::for_organic(Target::QuadCount(target));
    assert_eq!(config.method, Method::QuadLayout);
    config
}

#[test]
fn a_sphere_comes_out_closed_all_quads_and_on_budget() {
    let source = primitives::icosphere(4);
    for wanted in [600, 2500] {
        let (mesh, report) = config(wanted).remesh(&source).unwrap();
        let metrics = measure(&source, &mesh, Some(wanted));
        assert!(metrics.is_valid(), "{metrics:?}");
        assert!(!report.fell_back);
        assert!(metrics.budget_error.unwrap() <= 0.05, "{} faces for {wanted}", metrics.faces);
        // Its irregular vertices are the singularities of the field.
        assert_eq!(metrics.irregular_vertices, report.singularities.0 + report.singularities.1);
        assert!(metrics.hausdorff < 0.01, "{metrics:?}");
    }
}

#[test]
fn a_small_layout_of_more_rectangles_than_quads_is_still_quantized() {
    // A sphere's layout has at least the six rectangles of a cube: four
    // quads cannot be shared out among them, but so small a layout is
    // quantized in no time, and the nearest it comes is kept.
    let source = primitives::icosphere(4);
    let (mesh, report) = config(4).remesh(&source).unwrap();
    assert!(!report.fell_back, "{:?}", report.fallback_causes);
    assert!(!mesh.faces.is_empty());
    assert!(mesh.faces.iter().all(|face| face.len() == 4));
}

#[test]
fn a_cube_keeps_its_edges_and_corners() {
    let source = primitives::cube(8);
    let (mesh, _) = RemeshConfig::for_hard_surface(Target::QuadCount(600)).remesh(&source).unwrap();
    let metrics = measure(&source, &mesh, Some(600));
    assert!(metrics.is_valid(), "{metrics:?}");
    assert_eq!(metrics.irregular_vertices, 8);
    assert_eq!(metrics.base_complex_patches, Some(6));
    assert!(metrics.budget_error.unwrap() <= 0.05, "{} faces", metrics.faces);
    assert!(metrics.hausdorff < 1e-6, "{metrics:?}");
    // The eight corners are vertices of the mesh.
    for corner in 0..8 {
        let p = Vec3::new(
            if corner & 1 == 0 { -1.0 } else { 1.0 },
            if corner & 2 == 0 { -1.0 } else { 1.0 },
            if corner & 4 == 0 { -1.0 } else { 1.0 },
        );
        assert!(mesh.positions.iter().any(|q| q.distance(p) < 1e-6), "no vertex on corner {p}");
    }
}

#[test]
fn a_flat_sheet_comes_out_as_its_exact_grid() {
    let source = primitives::plane(24);
    let (mesh, _) = config(400).remesh(&source).unwrap();
    let stats = mesh.stats();
    assert_eq!((stats.faces, stats.quads, stats.irregular_vertices), (400, 400, 0));
    assert_eq!(stats.vertices, 21 * 21);
    // A regular grid of side 0.1, up to the border.
    for p in &mesh.positions {
        for coordinate in [p.x, p.y] {
            let steps = (coordinate + 1.0) / 0.1;
            assert!((steps - steps.round()).abs() < 1e-3, "vertex off the grid: {p}");
        }
        assert!(p.z.abs() < 1e-9);
    }
}

#[test]
fn a_torus_comes_out_as_one_regular_grid() {
    // Following the curvature, the field of a torus has no singularity:
    // the layout is then cut from an arbitrary vertex.
    let source = primitives::torus(1.0, 0.4, 96, 48);
    let (mesh, report) = config(2000).remesh(&source).unwrap();
    assert!(!report.fell_back);
    let metrics = measure(&source, &mesh, Some(2000));
    assert!(metrics.is_valid(), "{metrics:?}");
    assert_eq!(metrics.irregular_vertices, 0);
    assert_eq!(metrics.base_complex_patches, Some(1));
    assert!(metrics.budget_error.unwrap() <= 0.05, "{} faces", metrics.faces);
}

#[test]
fn vertices_are_read_off_the_map_onto_the_surface() {
    // The mesher on its own, before any smoothing: every vertex already
    // lies on the work mesh, and none had to be guessed.
    let source = primitives::torus(1.0, 0.4, 96, 48);
    let request = preprocess::Request { triangles: 8000, ..Default::default() };
    let work = preprocess::create(preprocess::Kind::Isotropic).preprocess(&source, &request).unwrap();
    let field_request = crossfield::Request { alignment: 0.0, ..Default::default() };
    let field = crossfield::create(crossfield::Kind::Power).solve(&work, &field_request).unwrap();
    let layout =
        tmesh::create(tmesh::Kind::Motorcycle).build(&work, &field, &tmesh::Request { edge_length: 0.1 }).unwrap();
    assert!(layout.tmesh.report.is_valid());
    let wanted = quantization::Request { quads: Some(1500), ..Default::default() };
    let quantized = quantization::create(quantization::Kind::WalkCancelling).quantize(&layout.tmesh, &wanted).unwrap();

    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        assert_eq!(meshing::create(kind).kind(), kind);
    }
    let (mesh, report) = meshing::create(Kind::PatchGrid).mesh(&work, &layout, &quantized).unwrap();
    assert_eq!((report.quads, report.interpolated), (quantized.report.quads, 0));
    assert_eq!(mesh.faces.len(), report.quads);
    assert_eq!(mesh.positions.len(), report.vertices);
    let surface = Bvh::build(&work.mesh);
    for &p in &mesh.positions {
        let off = surface.closest_point(p).unwrap().distance_squared.sqrt();
        // Vertices where several points merged sit at their average, a
        // little off; the others are on the surface to rounding.
        assert!(off < 0.02, "a vertex {off} off the surface");
    }
    let stats = mesh.stats();
    assert_eq!((stats.quads, stats.non_manifold_edges, stats.boundary_edges), (stats.faces, 0, 0));
    assert_eq!(stats.euler_characteristic(), 0);
}

#[test]
fn two_sheets_that_touch_at_one_point_are_meshed_all_the_same() {
    // Two squares sharing a single corner: round that vertex the surface is
    // two separate fans, each with its own pair of border edges.
    let mut mesh = primitives::plane(12);
    let shared = mesh.positions.iter().position(|p| p.x == 1.0 && p.y == 1.0).unwrap() as u32;
    let first = mesh.positions.len() as u32;
    let other = primitives::plane(12);
    let twin = other.positions.iter().position(|p| p.x == -1.0 && p.y == -1.0).unwrap() as u32;
    mesh.positions.extend(other.positions.iter().map(|&p| p + Vec3::new(2.0, 2.0, 0.0)));
    let moved = |v: u32| if v == twin { shared } else { first + v };
    mesh.triangles.extend(other.triangles.iter().map(|tri| tri.map(moved)));

    let (quads, _) = config(800).remesh(&mesh).unwrap();
    let stats = quads.stats();
    assert_eq!(stats.quads, stats.faces);
    assert!(stats.faces > 600 && stats.faces < 1000, "{} faces", stats.faces);
    assert_eq!(stats.non_manifold_edges, 0);
}

#[test]
fn the_chain_is_deterministic_and_checks_its_configuration() {
    let sphere = primitives::icosphere(4);
    let settings = config(800);
    let (first, second) = (settings.remesh(&sphere).unwrap().0, settings.remesh(&sphere).unwrap().0);
    assert_eq!(first.faces, second.faces);
    assert_eq!(first.positions, second.positions);
    assert!(RemeshConfig { alignment: -1.0, ..settings.clone() }.remesh(&sphere).is_err());
    assert!(RemeshConfig { target: Target::QuadCount(0), ..settings }.remesh(&sphere).is_err());
}
