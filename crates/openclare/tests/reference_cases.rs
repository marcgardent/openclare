//! Reference cases (DT-116): simple shapes whose best layout is known by
//! hand, with no algorithm involved. Each stage of the engine is held to
//! that answer; this file holds the T-mesh stage.
//!
//! The known answers, written by hand:
//!
//! | shape  | poles            | nodes | arcs | patches | T-junctions |
//! |--------|------------------|-------|------|---------|-------------|
//! | cube   | 8 of valence 3   | 8     | 12   | 6       | 0           |
//! | sphere | 8 of valence 3   | 8     | 12   | 6       | 0           |
//! | torus  | none             | 1     | 2    | 1       | 0           |
//!
//! On the cube and the sphere the poles sit at the corners of a cube and
//! each edge is one separatrix from pole to pole (DT-115). The torus has no
//! pole: it is cut along one loop of each family, through one vertex.
//!
//! The whole chain is held to the same answer on the sphere: a quad mesh of
//! six patches, its eight irregular vertices at the corners of a cube.

use openclare::TriMesh;
use openclare::crossfield;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::preprocess;
use openclare::tmesh::{self, TMesh};
use openclare::{RemeshConfig, Target};

/// The T-mesh of `source` made by the quad-layout chain, for about `quads`
/// quads. (On a work mesh of under three triangles per quad, the field of
/// the sphere is too coarse for its poles to come within half a quad edge
/// of each other's lines.)
fn layout(source: &TriMesh, quads: f64) -> TMesh {
    let edge_length = (source.surface_area() / quads).sqrt();
    let request = preprocess::Request { triangles: 20000, curvature_scale: Some(edge_length), ..Default::default() };
    let work = preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap();
    let request = crossfield::Request { alignment: 0.05, ..Default::default() };
    let field = crossfield::create(crossfield::Kind::Power).solve(&work, &request).unwrap();
    let builder = tmesh::create(tmesh::Kind::Motorcycle);
    let tmesh = builder.build(&work, &field, &tmesh::Request { edge_length, pairs: tmesh::Pairs::All }).unwrap().tmesh;
    assert!(tmesh.report.is_valid(), "{:?}", tmesh.report);
    tmesh
}

/// Nodes, arcs, patches, T-junctions, and the patches and arcs under half a
/// quad edge.
fn counts(tmesh: &TMesh) -> [usize; 6] {
    let report = &tmesh.report;
    [report.nodes, report.arcs, report.patches, report.t_junctions, report.thin_patches, report.short_arcs]
}

#[test]
fn the_cube_is_cut_along_its_twelve_edges() {
    for quads in [600.0, 3000.0] {
        let tmesh = layout(&primitives::cube(8), quads);
        assert_eq!(tmesh.report.singularities, 8);
        assert_eq!(counts(&tmesh), [8, 12, 6, 0, 0, 0], "{quads} quads: {:?}", tmesh.report);
    }
}

#[test]
fn the_sphere_is_cut_like_a_cube() {
    for quads in [600.0, 3000.0] {
        let tmesh = layout(&primitives::icosphere(5), quads);
        assert_eq!(tmesh.report.singularities, 8);
        assert_eq!(counts(&tmesh), [8, 12, 6, 0, 0, 0], "{quads} quads: {:?}", tmesh.report);
    }
}

#[test]
fn the_torus_is_one_patch() {
    for quads in [600.0, 3000.0] {
        let tmesh = layout(&primitives::torus(1.0, 0.4, 48, 24), quads);
        assert_eq!(tmesh.report.singularities, 0);
        assert_eq!(counts(&tmesh), [1, 2, 1, 0, 0, 0], "{quads} quads: {:?}", tmesh.report);
    }
}

#[test]
fn the_sphere_is_meshed_like_a_cube() {
    let source = primitives::icosphere(5);
    for quads in [600, 3000] {
        let (mesh, report) = RemeshConfig::for_organic(Target::QuadCount(quads)).remesh(&source).unwrap();
        let metrics = measure(&source, &mesh, Some(quads));
        assert!(metrics.is_valid(), "{metrics:?}");
        assert!(!report.fell_back, "{quads} quads: {:?}", report.fallback_causes);
        assert_eq!(metrics.irregular_vertices, 8, "{quads} quads");
        assert_eq!(metrics.base_complex_patches, Some(6), "{quads} quads");
    }
}
