//! What the stages of a chain show of their work, to whoever listens.
//!
//! A chain hands its intermediate results (work mesh, cross field, layout,
//! ...) to a [`Recorder`] as plain shapes under a path, `stage/layer`. The
//! library knows no viewer and depends on none: [`Silent`] listens to
//! nothing and is what every chain gets unless told otherwise; the viewers
//! live in crates of their own and implement the trait.
//!
//! A viewer may show one stage at a time, taking away what the stage before
//! showed. So a stage records all it takes to read it, the surface its
//! lines and points lie on included, and no two surfaces at the same place.

use crate::crossfield::{self, CrossField};
use crate::error::Result;
use crate::math::Vec3;
use crate::mesh::{PolyMesh, TriMesh};
use crate::parametrization::Parametrization;
use crate::preprocess::WorkMesh;
use crate::quantization::Quantization;
use crate::tmesh::{NodeKind, TMesh};

/// Red, green, blue.
pub type Colour = [u8; 3];

/// One thing to look at.
#[derive(Clone, Copy)]
pub enum Shape<'a> {
    /// A triangle mesh, with one value per vertex if it carries a quantity.
    Surface { mesh: &'a TriMesh, values: Option<&'a [f64]> },
    /// A polygon mesh, whose edges matter as much as its faces.
    Polygons(&'a PolyMesh),
    /// Polylines.
    Lines { strips: &'a [Vec<Vec3>], colour: Colour },
    /// Points, with one label each if they have something to say.
    Points { positions: &'a [Vec3], labels: Option<&'a [String]>, colour: Colour },
    /// A remark on what happened.
    Note(&'a str),
}

/// Receives what the stages show, in the order they run.
pub trait Recorder {
    /// False when nothing is kept: the stages then do not prepare what they
    /// would have shown.
    fn is_listening(&self) -> bool {
        true
    }

    /// `path` is `stage/layer`, followed by `/piece-NNN` when the input is
    /// made of several pieces meshed apart. A stage is over when another
    /// one records; what is under `notes` belongs to no stage.
    fn record(&mut self, path: &str, shape: Shape<'_>);

    /// Told when a stage begins, by a name meant to be read ("cross field",
    /// or "piece 3 of 12: cross field"), whether anything is kept or not:
    /// this is how a host follows a run. An error stops the chain there,
    /// which is how a host cancels one ([`crate::Error::Cancelled`]).
    fn begin(&mut self, _stage: &str) -> Result<()> {
        Ok(())
    }
}

/// The recorder that keeps nothing.
#[derive(Clone, Copy, Debug, Default)]
pub struct Silent;

impl Recorder for Silent {
    fn is_listening(&self) -> bool {
        false
    }

    fn record(&mut self, _: &str, _: Shape<'_>) {}
}

/// Files what one piece among several shows under its own name.
pub(crate) struct Piece<'a> {
    recorder: &'a mut dyn Recorder,
    name: String,
    /// "piece 3 of 12", for those who follow the run.
    place: String,
}

impl<'a> Piece<'a> {
    /// The piece of that `index` among `count`.
    pub(crate) fn new(recorder: &'a mut dyn Recorder, index: usize, count: usize) -> Self {
        Self { recorder, name: format!("piece-{index:03}"), place: format!("piece {} of {count}", index + 1) }
    }
}

impl Recorder for Piece<'_> {
    fn is_listening(&self) -> bool {
        self.recorder.is_listening()
    }

    fn record(&mut self, path: &str, shape: Shape<'_>) {
        self.recorder.record(&format!("{path}/{}", self.name), shape);
    }

    fn begin(&mut self, stage: &str) -> Result<()> {
        self.recorder.begin(&format!("{}: {stage}", self.place))
    }
}

/// Layers of one kind: the name, what belongs to it, its colour.
type Layers<T> = [(&'static str, fn(T) -> bool, Colour); 3];

const VALENCE_3: Colour = [60, 160, 255];
const VALENCE_5: Colour = [255, 80, 60];
const ODD_VALENCE: Colour = [255, 0, 255];
const CROSSES: Colour = [210, 210, 210];
const HARD_EDGES: Colour = [255, 140, 40];
const BORDERS: Colour = [80, 220, 255];
const CUTS: Colour = [255, 225, 80];
const CORNERS: Colour = [120, 230, 120];
const JUNCTIONS: Colour = [245, 245, 245];
const U_LINES: Colour = [255, 170, 70];
const V_LINES: Colour = [90, 200, 255];
const FOLDS: Colour = [255, 40, 40];
const FIRST_FOLDS: Colour = [255, 0, 255];
const LINES_KEPT: Colour = [90, 230, 110];
const LINES_LET_GO: Colour = [150, 110, 255];

/// The surface of the work mesh under `stage`, for what the stage draws to
/// lie on.
fn show_surface(recorder: &mut dyn Recorder, stage: &str, work: &WorkMesh) {
    recorder.record(&format!("{stage}/surface"), Shape::Surface { mesh: &work.mesh, values: None });
}

/// The hard edges of the work mesh under `stage`, if it has any.
fn show_hard_edges(recorder: &mut dyn Recorder, stage: &str, work: &WorkMesh) {
    if !work.feature_edges.is_empty() {
        let position = |v: u32| work.mesh.positions[v as usize];
        let edges: Vec<Vec<Vec3>> = work.feature_edges.iter().map(|&[a, b]| vec![position(a), position(b)]).collect();
        recorder.record(&format!("{stage}/hard-edges"), Shape::Lines { strips: &edges, colour: HARD_EDGES });
    }
}

/// The work mesh and its hard edges, then how much it bends.
pub(crate) fn show_work(recorder: &mut dyn Recorder, work: &WorkMesh) {
    if !recorder.is_listening() {
        return;
    }
    show_surface(recorder, "work", work);
    show_hard_edges(recorder, "work", work);
    let bend: Vec<f64> =
        (0..work.mesh.positions.len()).map(|v| work.curvature.k1[v].abs().max(work.curvature.k2[v].abs())).collect();
    recorder.record("curvature/bend", Shape::Surface { mesh: &work.mesh, values: Some(&bend) });
    show_hard_edges(recorder, "curvature", work);
}

/// How long the edges are asked to be, relative to the mean.
pub(crate) fn show_sizing(recorder: &mut dyn Recorder, work: &WorkMesh) {
    if recorder.is_listening() {
        recorder.record("sizing/edge-length", Shape::Surface { mesh: &work.mesh, values: Some(&work.sizing) });
        show_hard_edges(recorder, "sizing", work);
    }
}

/// A cross field under `stage`: one cross per face, and its singularities
/// by the valence the vertex there will have.
pub(crate) fn show_field(recorder: &mut dyn Recorder, stage: &str, work: &WorkMesh, field: &CrossField) {
    if !recorder.is_listening() {
        return;
    }
    let mesh = &work.mesh;
    show_surface(recorder, stage, work);
    if let Ok((_, frames)) = crossfield::prepare(mesh) {
        let mut crosses = Vec::with_capacity(2 * mesh.triangles.len());
        for f in 0..mesh.triangles.len() {
            let [a, b, c] = mesh.corners(f);
            let centre = (a + b + c) / 3.0;
            let reach = 0.1 * ((b - a).length() + (c - b).length() + (a - c).length());
            let arm = field.direction(&frames, f) * reach;
            let other = frames.normal[f].cross(arm);
            crosses.push(vec![centre - arm, centre + arm]);
            crosses.push(vec![centre - other, centre + other]);
        }
        recorder.record(&format!("{stage}/crosses"), Shape::Lines { strips: &crosses, colour: CROSSES });
    }
    let kinds: Layers<i8> = [
        ("valence-3", |index| index == 1, VALENCE_3),
        ("valence-5", |index| index == -1, VALENCE_5),
        ("valence-other", |index| index != 0 && index.abs() != 1, ODD_VALENCE),
    ];
    for (name, test, colour) in kinds {
        let positions: Vec<Vec3> =
            (0..mesh.positions.len()).filter(|&v| test(field.indices[v])).map(|v| mesh.positions[v]).collect();
        if !positions.is_empty() {
            recorder.record(&format!("{stage}/{name}"), Shape::Points { positions: &positions, labels: None, colour });
        }
    }
}

/// The lines of the T-mesh under `stage`, on the surface they run on: its
/// cuts, borders and hard edges.
fn show_cuts(recorder: &mut dyn Recorder, stage: &str, work: &WorkMesh, layout: &TMesh) {
    show_surface(recorder, stage, work);
    let lines: Layers<(bool, bool)> = [
        ("cuts", |(border, feature)| !border && !feature, CUTS),
        ("borders", |(border, _)| border, BORDERS),
        ("hard-edges", |(border, feature)| feature && !border, HARD_EDGES),
    ];
    for (name, test, colour) in lines {
        let strips: Vec<Vec<Vec3>> = layout
            .traces
            .iter()
            .filter(|trace| test((trace.border, trace.feature)) && trace.points.len() > 1)
            .map(|trace| trace.points.clone())
            .collect();
        if !strips.is_empty() {
            recorder.record(&format!("{stage}/{name}"), Shape::Lines { strips: &strips, colour });
        }
    }
}

/// The map the layout is cut from: its grid lines over the surface, the
/// triangles it folds over, and what became of the lines it was to keep.
pub(crate) fn show_map(recorder: &mut dyn Recorder, work: &WorkMesh, map: &Parametrization) {
    if !recorder.is_listening() {
        return;
    }
    let stage = "map";
    show_surface(recorder, stage, work);
    let mesh = &work.mesh;
    let corners = |f: usize| mesh.triangles[f].map(|v| mesh.positions[v as usize]);

    // The lines where u, then v, is a whole number: the edges of the quads
    // to come, were the map already made of whole numbers.
    for (axis, name, colour) in [(0, "u-lines", U_LINES), (1, "v-lines", V_LINES)] {
        let mut strips: Vec<Vec<Vec3>> = Vec::new();
        for f in 0..mesh.triangles.len() {
            let (points, values) = (corners(f), [0, 1, 2].map(|k| map.uv[3 * f + k][axis]));
            let (low, high) = (
                values.iter().copied().fold(f64::INFINITY, f64::min),
                values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            );
            // A triangle stretched over hundreds of lines is one the map
            // lost; its lines would only hide the others.
            if !(low.is_finite() && high.is_finite()) || high - low > 50.0 {
                continue;
            }
            for level in low.ceil() as i64..=high.floor() as i64 {
                let level = level as f64;
                let crossings: Vec<Vec3> = (0..3)
                    .filter_map(|k| {
                        let (a, b) = (values[k], values[(k + 1) % 3]);
                        let crosses = (a - level) * (b - level) < 0.0 || (a == level && b != level);
                        crosses.then(|| points[k].lerp(points[(k + 1) % 3], (level - a) / (b - a)))
                    })
                    .collect();
                if crossings.len() == 2 {
                    strips.push(crossings);
                }
            }
        }
        if !strips.is_empty() {
            recorder.record(&format!("{stage}/{name}"), Shape::Lines { strips: &strips, colour });
        }
    }

    // The triangles the map turns over or squashes flat, at the end, and
    // those that folded when every line was put on a grid line.
    let outline = |f: usize| {
        let [a, b, c] = corners(f);
        vec![a, b, c, a]
    };
    let turned = |f: usize| {
        let [a, b, c] = [0, 1, 2].map(|k| map.uv[3 * f + k]);
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) <= 0.0
    };
    let folds: Vec<Vec<Vec3>> = (0..mesh.triangles.len()).filter(|&f| turned(f)).map(outline).collect();
    if !folds.is_empty() {
        recorder.record(&format!("{stage}/folded-or-flat"), Shape::Lines { strips: &folds, colour: FOLDS });
    }
    let first: Vec<Vec<Vec3>> = map.report.folds_with_lines.iter().map(|&f| outline(f)).collect();
    if !first.is_empty() {
        let shape = Shape::Lines { strips: &first, colour: FIRST_FOLDS };
        recorder.record(&format!("{stage}/folded-with-every-line"), shape);
    }

    // The lines: creases and borders, and the guide lines kept or let go.
    let edge = |&[a, b]: &[u32; 2]| vec![mesh.positions[a as usize], mesh.positions[b as usize]];
    let guides: std::collections::HashSet<[u32; 2]> = work.guide_edges.iter().copied().collect();
    let loose: std::collections::HashSet<[u32; 2]> = map.report.loose_guides.iter().copied().collect();
    type Kind<'a> = &'a dyn Fn(&[u32; 2]) -> bool;
    let layers: [(&str, Colour, Kind); 3] = [
        ("creases-and-borders", HARD_EDGES, &|edge| !guides.contains(edge)),
        ("lines-kept", LINES_KEPT, &|edge| guides.contains(edge) && !loose.contains(edge)),
        ("lines-let-go", LINES_LET_GO, &|edge| loose.contains(edge)),
    ];
    for (name, colour, test) in layers {
        let strips: Vec<Vec<Vec3>> = work.feature_edges.iter().filter(|edge| test(edge)).map(edge).collect();
        if !strips.is_empty() {
            recorder.record(&format!("{stage}/{name}"), Shape::Lines { strips: &strips, colour });
        }
    }
    let report = &map.report;
    let note = format!(
        "{} triangles folded at the first solve, {} at the end after {} stiffening rounds; {} folded with every \
         line on a grid line; {} of {} guide edges let go{}",
        report.flipped_first,
        report.flipped,
        report.rounds,
        report.folds_with_lines.len(),
        report.loose_guides.len(),
        work.guide_edges.len(),
        if report.features_off_grid { "; creases and borders off the grid lines too" } else { "" },
    );
    recorder.record("notes/map", Shape::Note(&note));
    // The lines that are walls of the layout and do not lie on a grid line,
    // by kind: guide lines still kept, and the others, creases and borders.
    let walls: Vec<[u32; 2]> = work.feature_edges.iter().copied().filter(|edge| !loose.contains(edge)).collect();
    let astray = map.off_grid(mesh, &walls);
    if !astray.is_empty() {
        let of_guides = astray.iter().filter(|edge| guides.contains(*edge)).count();
        let strips: Vec<Vec<Vec3>> = astray.iter().map(edge).collect();
        recorder.record(&format!("{stage}/off-the-grid"), Shape::Lines { strips: &strips, colour: [255, 255, 255] });
        let note = format!(
            "{} edges that are walls of the layout are off the grid lines of the map: {} of guide lines, {} of creases and borders",
            astray.len(),
            of_guides,
            astray.len() - of_guides,
        );
        recorder.record("notes/map off the grid", Shape::Note(&note));
    }
    // What the layout will refuse the map for, and how much of it the map
    // could not see: the faces it had set aside.
    let turned = map.turned_over();
    if !turned.is_empty() {
        let aside = turned.iter().filter(|&&f| map.set_aside[f]).count();
        let note = format!(
            "{} triangles are turned over in the map as the layout counts them, {} of them among the {} the map \
             had set aside",
            turned.len(),
            aside,
            map.set_aside.iter().filter(|&&aside| aside).count(),
        );
        recorder.record("notes/map turned", Shape::Note(&note));
    }
    // Where the map folds with every line on a grid line: next to a place
    // where the lines turn, meet or end, or along a plain stretch of them?
    if !report.folds_with_lines.is_empty() {
        let mut along: Vec<Vec<u32>> = vec![Vec::new(); mesh.positions.len()];
        for &[a, b] in &work.feature_edges {
            along[a as usize].push(b);
            along[b as usize].push(a);
        }
        let turning = |v: usize| match along[v][..] {
            [a, b] => {
                let (here, a, b) = (mesh.positions[v], mesh.positions[a as usize], mesh.positions[b as usize]);
                (a - here).normalize_or_zero().dot((b - here).normalize_or_zero()) > -0.87
            }
            [] => false,
            _ => true,
        };
        let joints: Vec<Vec3> = (0..along.len()).filter(|&v| turning(v)).map(|v| mesh.positions[v]).collect();
        let on_lines: Vec<Vec3> =
            (0..along.len()).filter(|&v| !along[v].is_empty()).map(|v| mesh.positions[v]).collect();
        let reach = 2.0 * mesh.edge_length_stats().1;
        let near = |points: &[Vec3], f: usize| {
            let middle = corners(f).iter().sum::<Vec3>() / 3.0;
            points.iter().any(|p| p.distance(middle) <= reach)
        };
        // A triangle with its three corners on lines has nowhere to go when
        // two lines run side by side a triangle apart: how many are such.
        let on_a_line = |v: u32| !along[v as usize].is_empty();
        let feature: std::collections::HashSet<(u32, u32)> =
            work.feature_edges.iter().map(|&[a, b]| (a.min(b), a.max(b))).collect();
        let sides_on_lines = |f: usize| {
            let tri = mesh.triangles[f];
            (0..3).filter(|&k| feature.contains(&(tri[k].min(tri[(k + 1) % 3]), tri[k].max(tri[(k + 1) % 3])))).count()
        };
        let between = report
            .folds_with_lines
            .iter()
            .filter(|&&f| mesh.triangles[f].iter().all(|&v| on_a_line(v)) && sides_on_lines(f) <= 1)
            .count();
        let pinched = report.folds_with_lines.iter().filter(|&&f| sides_on_lines(f) >= 2).count();
        let at_joints = report.folds_with_lines.iter().filter(|&&f| near(&joints, f)).count();
        let at_lines = report.folds_with_lines.iter().filter(|&&f| near(&on_lines, f)).count();
        let note = format!(
            "of the {} triangles folded with every line on a grid line, {} are within two edges of a place where \
             lines turn by more than 30 degrees, meet or end ({} such places), {} within two edges of a line at all; \
             {} span from one line to another a triangle away (three corners on lines, at most one side along one), \
             {} have two sides along lines",
            report.folds_with_lines.len(),
            at_joints,
            joints.len(),
            at_lines,
            between,
            pinched,
        );
        recorder.record("notes/map folds", Shape::Note(&note));
    }
}

/// The T-mesh: its lines, and its nodes by kind.
pub(crate) fn show_layout(recorder: &mut dyn Recorder, work: &WorkMesh, layout: &TMesh) {
    if !recorder.is_listening() {
        return;
    }
    show_cuts(recorder, "layout", work, layout);
    let nodes: Layers<NodeKind> = [
        ("singularities", |kind| matches!(kind, NodeKind::Singularity { .. }), VALENCE_5),
        ("corners", |kind| kind == NodeKind::Corner, CORNERS),
        ("junctions", |kind| kind == NodeKind::Junction, JUNCTIONS),
    ];
    for (name, test, colour) in nodes {
        let positions: Vec<Vec3> =
            layout.nodes.iter().filter(|node| test(node.kind)).map(|node| node.position).collect();
        if !positions.is_empty() {
            recorder.record(&format!("layout/{name}"), Shape::Points { positions: &positions, labels: None, colour });
        }
    }
}

/// The whole length each arc of the T-mesh was given, written at its middle.
pub(crate) fn show_lengths(recorder: &mut dyn Recorder, work: &WorkMesh, layout: &TMesh, quantized: &Quantization) {
    if !recorder.is_listening() {
        return;
    }
    show_cuts(recorder, "quantization", work, layout);
    let position = |node: u32| layout.nodes[node as usize].position;
    let middles: Vec<Vec3> =
        layout.arcs.iter().map(|arc| 0.5 * (position(arc.nodes[0]) + position(arc.nodes[1]))).collect();
    let labels: Vec<String> = quantized.lengths.iter().map(|length| length.to_string()).collect();
    if middles.len() == labels.len() && !middles.is_empty() {
        let shape = Shape::Points { positions: &middles, labels: Some(&labels), colour: CUTS };
        recorder.record("quantization/lengths", shape);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Paths(Vec<String>);

    impl Recorder for Paths {
        fn record(&mut self, path: &str, _: Shape<'_>) {
            self.0.push(path.to_string());
        }
    }

    #[test]
    fn a_piece_files_what_it_shows_under_its_own_name() {
        let mut paths = Paths(Vec::new());
        Piece::new(&mut paths, 7, 9).record("layout/cuts", Shape::Note(""));
        assert_eq!(paths.0, ["layout/cuts/piece-007"]);
        assert!(Piece::new(&mut paths, 0, 1).is_listening());
        assert!(!Piece::new(&mut Silent, 0, 1).is_listening());
    }
}
