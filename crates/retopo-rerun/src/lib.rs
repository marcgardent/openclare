//! Writes what the retopo chains show of their work as a Rerun recording, a
//! `.rrd` file to open in the Rerun viewer (<https://rerun.io>).
//!
//! The library itself knows nothing of Rerun: it talks to a
//! [`retopo::recorder::Recorder`], and this crate is one.

use std::collections::HashSet;
use std::path::Path;

use rerun::{Clear, Color, LineStrips3D, Mesh3D, Points3D, Radius, RecordingStream, RecordingStreamBuilder, TextLog};
use retopo::math::Vec3;
use retopo::recorder::{Colour, Recorder, Shape};
use retopo::{PolyMesh, TriMesh};

/// Colour of a surface that carries no quantity, and of the faces of a
/// polygon mesh.
const SURFACE: Colour = [150, 165, 185];
const FACES: Colour = [170, 190, 215];
const EDGES: Colour = [25, 30, 40];

/// From the least to the most of a quantity.
const RAMP: [Colour; 5] = [[68, 1, 84], [59, 82, 139], [33, 145, 140], [94, 201, 98], [253, 231, 37]];

/// Timeline along which each stage takes the place of the one before.
const STEP: &str = "step";
/// Timeline along which the stages pile up.
const STACK: &str = "stack";
/// What is recorded under this belongs to no stage.
const NOTES: &str = "notes";

/// A recording being written to a file.
///
/// Each stage of the chain is one tick of two timelines. Along `step`, a
/// stage takes the place of the one before: moving along it shows the chain
/// at work, one task at a time. Along `stack`, everything shown so far
/// stays, to lay one stage over another.
pub struct RerunRecorder {
    stream: RecordingStream,
    stage: String,
    step: i64,
    /// The first thing that went wrong: a recorder cannot stop the chain.
    failure: Option<String>,
}

impl RerunRecorder {
    /// Starts a recording in the file at `path`, replacing it.
    pub fn create(path: &Path) -> Result<Self, String> {
        let stream = RecordingStreamBuilder::new("openclare")
            .save(path)
            .map_err(|error| format!("cannot record to {}: {error}", path.display()))?;
        Ok(Self { stream, stage: String::new(), step: -1, failure: None })
    }

    /// Writes what is left to write. Fails if anything could not be recorded.
    pub fn finish(mut self) -> Result<(), String> {
        if let Err(error) = self.stream.flush_blocking() {
            self.failure.get_or_insert(error.to_string());
        }
        self.failure.map_or(Ok(()), Err)
    }

    fn log(&mut self, path: &str, shape: &impl rerun::AsComponents) {
        if let Err(error) = self.stream.log(path, shape) {
            self.failure.get_or_insert(format!("{path}: {error}"));
        }
    }
}

impl Recorder for RerunRecorder {
    fn record(&mut self, path: &str, shape: Shape<'_>) {
        let stage = path.split('/').next().unwrap_or(path);
        if stage != self.stage && stage != NOTES {
            self.step += 1;
            self.stream.set_time_sequence(STEP, self.step);
            if !self.stage.is_empty() {
                // The stage before leaves, along `step` only.
                self.stream.disable_timeline(STACK);
                let before = std::mem::take(&mut self.stage);
                self.log(&before, &Clear::recursive());
            }
            self.stream.set_time_sequence(STACK, self.step);
            self.stage = stage.to_string();
        }
        match shape {
            Shape::Surface { mesh, values } => {
                let colours: Vec<Color> = match values {
                    Some(values) => ramp(values),
                    None => vec![colour(SURFACE); mesh.positions.len()],
                };
                self.log(path, &surface(mesh, colours));
            }
            Shape::Polygons(mesh) => {
                let (faces, edges) = polygons(mesh);
                self.log(&format!("{path}/faces"), &faces);
                self.log(&format!("{path}/edges"), &edges);
            }
            Shape::Lines { strips, colour: tint } => {
                let lines = LineStrips3D::new(strips.iter().map(|strip| strip.iter().map(|&p| point(p))))
                    .with_colors([colour(tint)])
                    .with_radii([Radius::new_ui_points(1.0)]);
                self.log(path, &lines);
            }
            Shape::Points { positions, labels, colour: tint } => {
                let mut points = Points3D::new(positions.iter().map(|&p| point(p)))
                    .with_colors([colour(tint)])
                    .with_radii([Radius::new_ui_points(if labels.is_some() { 2.0 } else { 5.0 })]);
                if let Some(labels) = labels {
                    points = points.with_labels(labels.iter().map(String::as_str));
                }
                self.log(path, &points);
            }
            Shape::Note(text) => self.log(path, &TextLog::new(text)),
        }
    }
}

fn point(p: Vec3) -> [f32; 3] {
    [p.x as f32, p.y as f32, p.z as f32]
}

fn colour([r, g, b]: Colour) -> Color {
    Color::from_rgb(r, g, b)
}

/// One colour per value, from the 2nd to the 98th percentile so that a few
/// extreme values do not flatten the rest.
fn ramp(values: &[f64]) -> Vec<Color> {
    let mut sorted: Vec<f64> = values.iter().copied().filter(|value| value.is_finite()).collect();
    sorted.sort_by(f64::total_cmp);
    let at = |share: f64| sorted.get(((sorted.len() as f64 - 1.0) * share).round() as usize).copied().unwrap_or(0.0);
    let (low, high) = (at(0.02), at(0.98));
    let span = (high - low).max(f64::MIN_POSITIVE);
    values
        .iter()
        .map(|&value| {
            let t = if value.is_finite() { ((value - low) / span).clamp(0.0, 1.0) } else { 0.0 };
            let along = t * (RAMP.len() - 1) as f64;
            let from = (along.floor() as usize).min(RAMP.len() - 2);
            let mix = along - from as f64;
            let channel =
                |k: usize| (RAMP[from][k] as f64 * (1.0 - mix) + RAMP[from + 1][k] as f64 * mix).round() as u8;
            Color::from_rgb(channel(0), channel(1), channel(2))
        })
        .collect()
}

/// Normals at the vertices, each face weighing in by its area.
fn normals(positions: &[Vec3], triangles: &[[u32; 3]]) -> Vec<[f32; 3]> {
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for triangle in triangles {
        let [a, b, c] = triangle.map(|v| positions[v as usize]);
        let area = (b - a).cross(c - a);
        for &v in triangle {
            normals[v as usize] += area;
        }
    }
    normals.into_iter().map(|normal| point(normal.normalize_or_zero())).collect()
}

fn surface(mesh: &TriMesh, colours: Vec<Color>) -> Mesh3D {
    Mesh3D::new(mesh.positions.iter().map(|&p| point(p)))
        .with_triangle_indices(mesh.triangles.iter().copied())
        .with_vertex_normals(normals(&mesh.positions, &mesh.triangles))
        .with_vertex_colors(colours)
}

/// The faces of a polygon mesh as triangles, and its edges, each once.
fn polygons(mesh: &PolyMesh) -> (Mesh3D, LineStrips3D) {
    let mut triangles = Vec::new();
    let mut edges = HashSet::new();
    for face in &mesh.faces {
        for k in 1..face.len().saturating_sub(1) {
            triangles.push([face[0], face[k], face[k + 1]]);
        }
        for k in 0..face.len() {
            let (a, b) = (face[k], face[(k + 1) % face.len()]);
            edges.insert((a.min(b), a.max(b)));
        }
    }
    let mut edges: Vec<(u32, u32)> = edges.into_iter().collect();
    edges.sort_unstable();
    let faces = Mesh3D::new(mesh.positions.iter().map(|&p| point(p)))
        .with_vertex_normals(normals(&mesh.positions, &triangles))
        .with_vertex_colors(vec![colour(FACES); mesh.positions.len()])
        .with_triangle_indices(triangles);
    let position = |v: u32| point(mesh.positions[v as usize]);
    let edges = LineStrips3D::new(edges.into_iter().map(|(a, b)| [position(a), position(b)]))
        .with_colors([colour(EDGES)])
        .with_radii([Radius::new_ui_points(1.0)]);
    (faces, edges)
}
