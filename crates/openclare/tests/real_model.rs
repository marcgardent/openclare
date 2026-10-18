//! A real model through the chain: the ogre of the project, brought down to
//! 19 480 triangles (`assets/ogre.obj`). Arms, fingers, ears and folds: what
//! spheres and cubes do not have.
//!
//! What must hold whatever the tuning is checked as it is: a closed mesh of
//! quads, of the right number, by the main chain. The bounds on quality are
//! of another kind: some way above what the engine gives today (in the
//! comments), they say when it got clearly worse, not that it is good.

use openclare::mesh::obj::read_obj;
use openclare::metrics::measure;
use openclare::{RemeshConfig, Target, TriMesh};

fn ogre() -> TriMesh {
    let mesh = read_obj(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/ogre.obj")).unwrap();
    assert_eq!((mesh.positions.len(), mesh.triangles.len()), (9742, 19480));
    assert_eq!(mesh.pieces().len(), 1);
    mesh
}

#[test]
fn the_ogre_comes_out_closed_all_quads_on_budget_and_close_to_itself() {
    let source = ogre();
    // The quads asked for, then the most allowed of: the distance to the
    // source furthest and on average (of the diagonal of its box), the
    // angle off the directions of curvature and the angle of the corners off
    // a right angle (degrees). Measured on 12 October 2026: 0.029, 0.0028,
    // 8.8 and 10.5 at 600 quads; 0.010, 0.0009, 10.5 and 6.8 at 2 000.
    for (quads, furthest, on_average, off_curvature, off_square) in
        [(600, 0.045, 0.0045, 13.0, 14.0), (2000, 0.02, 0.0015, 13.0, 9.0)]
    {
        let (mesh, report) = RemeshConfig::for_organic(Target::QuadCount(quads)).remesh(&source).unwrap();
        let metrics = measure(&source, &mesh, Some(quads));
        assert!(metrics.is_valid(), "{quads}: {metrics:?}");
        assert_eq!((report.pieces, report.pieces_failed, report.pieces_boxed), (1, 0, 0), "{quads}");
        assert!(!report.fell_back, "{quads}: through the fallback chain");
        assert_eq!((metrics.quads, metrics.boundary_edges, metrics.components), (metrics.faces, 0, 1), "{quads}");
        // A body without a handle: as many vertices as faces, and two more.
        assert_eq!(mesh.positions.len(), mesh.faces.len() + 2, "{quads}");
        assert!(metrics.budget_error.unwrap() <= 0.05, "{} faces for {quads}", metrics.faces);
        assert_eq!(metrics.irregular_vertices, report.singularities.0 + report.singularities.1, "{quads}");
        let kept = metrics.volume_ratio.unwrap();
        assert!((kept - 1.0).abs() < 0.015, "{quads}: volume kept {kept}");

        assert!(metrics.hausdorff < furthest, "{quads}: {} away at the furthest", metrics.hausdorff);
        assert!(metrics.mean_distance < on_average, "{quads}: {} away on average", metrics.mean_distance);
        let alignment = metrics.alignment.unwrap();
        assert!(alignment < off_curvature, "{quads}: {alignment} degrees off the curvature");
        assert!(metrics.angle_deviation_mean < off_square, "{quads}: corners {} degrees off", metrics.angle_deviation_mean);
    }
}

#[test]
fn the_ogre_keeps_its_thin_parts_when_asked_to() {
    // Fingers and ears are thinner than a quad at these budgets. With quads
    // round thin parts (the default) they are meshed finer, and the mesh is
    // much closer to the shape there: measured on 12 October 2026, 0.063
    // quad edges away against 0.130 at 600 quads, 0.051 against 0.150 at
    // 1 000. Those quads are not spent elsewhere: at 600 the mesh is a fifth
    // farther on average over the whole shape, which is not asked to be
    // better here, only not to be much worse.
    let source = ogre();
    for quads in [600, 1000] {
        let kept = RemeshConfig::for_organic(Target::QuadCount(quads));
        assert_eq!(kept.quads_around, 6.0);
        let dropped = RemeshConfig { quads_around: 0.0, ..kept.clone() };
        let (with, _) = kept.remesh(&source).unwrap();
        let (without, _) = dropped.remesh(&source).unwrap();
        let (with, without) = (measure(&source, &with, Some(quads)), measure(&source, &without, Some(quads)));
        assert!(with.is_valid() && without.is_valid(), "{quads}");
        assert!(with.thin_share > 0.02, "{quads}: {} of the ogre is thin", with.thin_share);
        let (near, far) = (with.thin_distance.unwrap(), without.thin_distance.unwrap());
        assert!(near < 0.6 * far, "{quads}: thin parts {near} quad edges away with, {far} without");
        assert!(with.mean_distance < 1.35 * without.mean_distance, "{quads}: {} then {}", without.mean_distance, with.mean_distance);
    }
}

#[test]
fn a_simpler_ogre_has_no_more_irregular_vertices_and_is_still_all_quads() {
    // Simplicity from the regular end to the simple one, the presets' in
    // between: never more irregular vertices, and as sound a mesh of quads.
    // Measured on 18 October 2026 at 1 000 quads: 142, 104, 86 and 62.
    let source = ogre();
    let mut counts = Vec::new();
    for simplicity in [0.0, openclare::config::SIMPLICITY, 0.5, 1.0] {
        let config = RemeshConfig { simplicity, ..RemeshConfig::for_organic(Target::QuadCount(1000)) };
        let (mesh, report) = config.remesh(&source).unwrap();
        let metrics = measure(&source, &mesh, Some(1000));
        assert!(metrics.is_valid(), "{simplicity}: {metrics:?}");
        assert!(!report.fell_back, "{simplicity}: through the fallback chain");
        assert_eq!(metrics.quads, metrics.faces, "{simplicity}");
        counts.push(metrics.irregular_vertices);
    }
    eprintln!("irregular vertices: {counts:?}");
    assert!(counts.windows(2).all(|pair| pair[1] <= pair[0]), "irregular vertices {counts:?}");
    assert!(counts[3] < counts[0], "irregular vertices {counts:?}");
}
