//! Showing the stages of a chain to a recorder: what is shown, and that
//! being watched changes nothing to the result.

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::recorder::{Recorder, Shape};
use openclare::{Guides, RemeshConfig, Target, TriMesh};

/// Keeps the path of everything it is shown, and counts what is in it.
#[derive(Default)]
struct Paths {
    seen: Vec<String>,
    elements: usize,
}

impl Recorder for Paths {
    fn record(&mut self, path: &str, shape: Shape<'_>) {
        self.seen.push(path.to_string());
        self.elements += match shape {
            Shape::Surface { mesh, values } => {
                assert!(values.is_none_or(|values| values.len() == mesh.positions.len()), "{path}");
                mesh.triangles.len()
            }
            Shape::Polygons(mesh) => mesh.faces.len(),
            Shape::Lines { strips, .. } => {
                assert!(strips.iter().all(|strip| strip.len() > 1), "{path}");
                strips.len()
            }
            Shape::Points { positions, labels, .. } => {
                assert!(labels.is_none_or(|labels| labels.len() == positions.len()), "{path}");
                positions.len()
            }
            Shape::Note(text) => text.len(),
        };
    }
}

fn config(quads: usize) -> RemeshConfig {
    RemeshConfig { adaptivity: 0.0, ..RemeshConfig::for_organic(Target::QuadCount(quads)) }
}

#[test]
fn a_chain_shows_its_stages_in_the_order_they_run() {
    let mesh = primitives::icosphere(4);
    let mut paths = Paths::default();
    let (quads, _) = config(600).remesh_recorded(&mesh, &Guides::default(), &mut paths).unwrap();
    // What is under `notes` belongs to no stage.
    let stages: Vec<&str> =
        paths.seen.iter().map(|path| path.split('/').next().unwrap()).filter(|&stage| stage != "notes").collect();
    let mut order = stages.clone();
    order.dedup();
    assert_eq!(
        order,
        [
            "input",
            "work",
            "curvature",
            "sizing",
            "cross-field",
            "singularities",
            "map",
            "layout",
            "quantization",
            "meshing",
            "relaxation",
            "result"
        ]
    );
    for layer in ["work/surface", "cross-field/crosses", "singularities/valence-3", "layout/cuts", "result/quads"] {
        assert!(paths.seen.iter().any(|path| path == layer), "{layer} is missing from {:?}", paths.seen);
    }
    // A viewer may show one stage at a time: what a stage draws on the
    // surface comes with that surface.
    for stage in ["cross-field", "singularities", "layout", "quantization"] {
        let surface = format!("{stage}/surface");
        assert!(paths.seen.contains(&surface), "{surface} is missing from {:?}", paths.seen);
    }
    assert!(paths.elements > quads.faces.len());
}

/// Keeps the names of the stages as they begin, and stops the chain when
/// one of them is `stop_at`.
#[derive(Default)]
struct Stages {
    begun: Vec<String>,
    stop_at: Option<&'static str>,
}

impl Recorder for Stages {
    fn is_listening(&self) -> bool {
        false
    }

    fn record(&mut self, _: &str, _: Shape<'_>) {}

    fn begin(&mut self, stage: &str) -> openclare::Result<()> {
        self.begun.push(stage.to_string());
        if self.stop_at.is_some_and(|stop| stage.ends_with(stop)) { Err(openclare::Error::Cancelled) } else { Ok(()) }
    }
}

#[test]
fn a_host_is_told_each_stage_as_it_begins() {
    let mesh = primitives::icosphere(4);
    let mut stages = Stages::default();
    config(600).remesh_recorded(&mesh, &Guides::default(), &mut stages).unwrap();
    assert_eq!(
        stages.begun,
        [
            "preprocessing",
            "sizing",
            "cross field",
            "singularities",
            "layout",
            "quantization",
            "meshing",
            "relaxation",
            "fitting"
        ]
    );
}

#[test]
fn a_host_can_stop_a_run_between_two_stages() {
    let mesh = primitives::icosphere(4);
    let mut stages = Stages { stop_at: Some("layout"), ..Default::default() };
    let stopped = config(600).remesh_recorded(&mesh, &Guides::default(), &mut stages);
    assert!(matches!(stopped, Err(openclare::Error::Cancelled)));
    assert_eq!(stages.begun.last().map(String::as_str), Some("layout"));
}

#[test]
fn the_stages_of_a_piece_say_which_piece() {
    let mut mesh = TriMesh::default();
    for offset in [Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)] {
        let (piece, first) = (primitives::icosphere(3), mesh.positions.len() as u32);
        mesh.positions.extend(piece.positions.iter().map(|&p| p + offset));
        mesh.triangles.extend(piece.triangles.iter().map(|tri| tri.map(|v| v + first)));
    }
    let mut stages = Stages::default();
    config(600).remesh_recorded(&mesh, &Guides::default(), &mut stages).unwrap();
    assert_eq!(stages.begun.first().map(String::as_str), Some("piece 1 of 2: preprocessing"));
    assert!(stages.begun.contains(&"piece 2 of 2: fitting".to_string()), "{:?}", stages.begun);
}

#[test]
fn being_watched_changes_nothing_to_the_result() {
    let mesh = primitives::icosphere(4);
    let (silent, _) = config(600).remesh(&mesh).unwrap();
    let (watched, _) = config(600).remesh_recorded(&mesh, &Guides::default(), &mut Paths::default()).unwrap();
    assert_eq!(silent.faces, watched.faces);
    assert_eq!(silent.positions, watched.positions);
}

#[test]
fn each_piece_shows_its_stages_under_its_own_name() {
    let mut mesh = TriMesh::default();
    for offset in [Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)] {
        let (piece, first) = (primitives::icosphere(3), mesh.positions.len() as u32);
        mesh.positions.extend(piece.positions.iter().map(|&p| p + offset));
        mesh.triangles.extend(piece.triangles.iter().map(|tri| tri.map(|v| v + first)));
    }
    let mut paths = Paths::default();
    config(600).remesh_recorded(&mesh, &Guides::default(), &mut paths).unwrap();
    for layer in ["layout/cuts/piece-000", "layout/cuts/piece-001", "input/surface", "result/quads"] {
        assert!(paths.seen.iter().any(|path| path == layer), "{layer} is missing from {:?}", paths.seen);
    }
}
