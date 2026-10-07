//! Cross field (stage M2): one cross per face telling which way the quad
//! edges should run, and the singularities that field implies.
//!
//! The stage is a [`CrossFieldSolver`]; chains get one from [`create`].

pub mod frames;

mod extrinsic;
mod power;

use std::f64::consts::{FRAC_PI_2, TAU};
use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::math::{INVALID, Vec3};
use crate::mesh::{Topology, TriMesh};
use crate::preprocess::WorkMesh;
pub use frames::FaceFrames;
pub(crate) use power::constraints;

/// A cross per face of a work mesh.
#[derive(Clone, Debug)]
pub struct CrossField {
    /// Angle of one arm of the cross on each face, in the face's frame (see
    /// [`FaceFrames`]), within `(-π/4, π/4]`.
    pub angles: Vec<f64>,
    /// Per half-edge `3 * f + k` towards face `g`: how many quarter turns
    /// separate the arm stored on `g` from the arm of `f` carried across,
    /// modulo 4 and kept in `-1..=2`. This is what lets a parametrization
    /// follow one arm consistently.
    pub period_jumps: Vec<i32>,
    /// Index of the field at each vertex, in quarter turns: 0 for a regular
    /// vertex, +1 where a valence-3 vertex will sit, -1 for a valence 5.
    /// Border vertices are reported as 0.
    pub indices: Vec<i8>,
}

impl CrossField {
    /// One arm of the cross of face `f`, as a unit vector.
    pub fn direction(&self, frames: &FaceFrames, f: usize) -> Vec3 {
        frames.direction(f, self.angles[f])
    }

    /// How many vertices have index +1/4, -1/4, and any other non-zero index.
    pub fn singularities(&self) -> (usize, usize, usize) {
        let count = |test: fn(i8) -> bool| self.indices.iter().filter(|&&i| test(i)).count();
        (count(|i| i == 1), count(|i| i == -1), count(|i| i != 0 && i.abs() != 1))
    }

    /// Sum of the indices, in quarter turns: `4 χ` on a closed surface.
    pub fn total_index(&self) -> i64 {
        self.indices.iter().map(|&i| i as i64).sum()
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    /// How strongly the field is pulled towards the principal curvature
    /// directions, relative to its own smoothness. 0 ignores curvature; the
    /// higher, the better the alignment and the more singularities.
    pub alignment: f64,
    pub seed: u64,
}

impl Default for Request {
    fn default() -> Self {
        Self { alignment: 0.05, seed: 1 }
    }
}

pub trait CrossFieldSolver {
    fn kind(&self) -> Kind;
    fn solve(&self, work: &WorkMesh, request: &Request) -> Result<CrossField>;
}

/// The cross field solvers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Smoothest field in the least-squares sense, with curvature alignment
    /// and hard feature constraints (after Knöppel et al. 2013, per face).
    Power,
    /// The local, extrinsic smoothing of Instant Meshes, read off per face.
    /// Kept as a point of comparison.
    Extrinsic,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Power, Kind::Extrinsic];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Power => "power",
            Kind::Extrinsic => "extrinsic",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        Kind::ALL.iter().copied().find(|k| k.name() == name).ok_or_else(|| {
            let known: Vec<&str> = Kind::ALL.iter().map(|k| k.name()).collect();
            Error::InvalidParams(format!("unknown cross field solver '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the solver of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn CrossFieldSolver> {
    match kind {
        Kind::Power => Box::new(power::Power::default()),
        Kind::Extrinsic => Box::new(extrinsic::Extrinsic::default()),
    }
}

/// The connectivity every solver needs, refusing meshes the stage cannot
/// work on: from here on the pipeline assumes an oriented manifold.
pub(crate) fn prepare(mesh: &TriMesh) -> Result<(Topology, FaceFrames)> {
    if mesh.triangles.is_empty() {
        return Err(Error::InvalidMesh("work mesh has no triangles".into()));
    }
    let topology = Topology::build(mesh);
    if topology.non_manifold_edges > 0 {
        return Err(Error::InvalidMesh(format!(
            "work mesh has {} non-manifold or inconsistently oriented edges",
            topology.non_manifold_edges
        )));
    }
    let frames = FaceFrames::build(mesh, &topology);
    Ok((topology, frames))
}

/// Wraps an angle difference to the nearest quarter turn: the count of
/// quarter turns, and what is left in `(-π/4, π/4]`.
pub(crate) fn split_quarter_turns(angle: f64) -> (i32, f64) {
    let turns = (angle / FRAC_PI_2).round();
    (turns as i32, angle - turns * FRAC_PI_2)
}

/// Derives the period jumps and vertex indices of a field given by its
/// per-face angles, matching neighbouring crosses arm to nearest arm.
fn finish(mesh: &TriMesh, topology: &Topology, frames: &FaceFrames, angles: Vec<f64>) -> CrossField {
    debug_assert_eq!(topology.opposite.len(), frames.across.len());
    build_field(mesh, frames, angles, None)
}

/// Assembles a field from per-face angles and, optionally, the period jump
/// of every half-edge. Without jumps, neighbouring crosses are matched arm
/// to nearest arm. With them the matching is imposed: that is how a field
/// keeps prescribed singularities even where it turns by more than an
/// eighth of a turn across an edge.
pub(crate) fn build_field(
    mesh: &TriMesh,
    frames: &FaceFrames,
    mut angles: Vec<f64>,
    jumps: Option<Vec<i32>>,
) -> CrossField {
    let half_edges = 3 * mesh.triangles.len();
    // Bring every angle to its representative arm. Imposed jumps count
    // quarter turns between the arms that were stored, so they move along.
    let turns: Vec<i32> = angles
        .iter_mut()
        .map(|angle| {
            let (turns, rest) = split_quarter_turns(*angle);
            *angle = rest;
            turns
        })
        .collect();
    let jump_of = |h: usize, g: usize| match &jumps {
        Some(jumps) => jumps[h] + turns[h / 3] - turns[g],
        None => split_quarter_turns(angles[g] - angles[h / 3] - frames.rotation[h]).0,
    };

    let mut period_jumps = vec![0; half_edges];
    // Around each vertex: its angle defect, plus how much the field turns
    // relative to parallel transport on the way round. Together they are a
    // whole number of quarter turns, the index of the field there.
    let mut defect = vec![TAU; mesh.positions.len()];
    let mut turning = vec![0.0; mesh.positions.len()];
    let mut on_border = vec![false; mesh.positions.len()];
    for h in 0..half_edges {
        let (f, k) = (h / 3, h % 3);
        let tri = mesh.triangles[f];
        let [p, next, previous] = [k, (k + 1) % 3, (k + 2) % 3].map(|c| mesh.positions[tri[c] as usize]);
        let (to_next, to_previous) = ((next - p).normalize_or_zero(), (previous - p).normalize_or_zero());
        defect[tri[k] as usize] -= to_next.dot(to_previous).clamp(-1.0, 1.0).acos();

        let g = frames.across[h];
        if g == INVALID {
            on_border[tri[k] as usize] = true;
            on_border[tri[(k + 1) % 3] as usize] = true;
            continue;
        }
        let jump = jump_of(h, g as usize);
        let rest = angles[g as usize] - angles[f] - frames.rotation[h] - jump as f64 * FRAC_PI_2;
        // Only meaningful modulo a full turn: keep it in -1..=2.
        period_jumps[h] = (jump + 1).rem_euclid(4) - 1;
        // Face `f` lies to the left of its half-edge, so crossing it from
        // `f` to `g` goes clockwise around the half-edge's origin.
        turning[tri[k] as usize] -= rest;
    }

    let indices = (0..mesh.positions.len())
        .map(|v| if on_border[v] { 0 } else { (4.0 * (defect[v] + turning[v]) / TAU).round() as i8 })
        .collect();
    CrossField { angles, period_jumps, indices }
}

/// Mean angle, in degrees, between the crosses and the principal curvature
/// directions of the work mesh, weighted by how trustworthy those are: 0 is
/// perfect, 22.5 is what a random field scores. `None` where the surface has
/// no direction to follow.
pub fn alignment_error(work: &WorkMesh, frames: &FaceFrames, field: &CrossField) -> Option<f64> {
    let (mut weighted, mut total) = (0.0, 0.0);
    for (f, tri) in work.mesh.triangles.iter().enumerate() {
        for &v in tri {
            let v = v as usize;
            let weight = work.confidence[v] * frames.area[f];
            if weight <= 0.0 || frames.area[f] == 0.0 {
                continue;
            }
            let (_, off) = split_quarter_turns(frames.angle_of(f, work.curvature.dir1[v]) - field.angles[f]);
            weighted += weight * off.abs().to_degrees();
            total += weight;
        }
    }
    let surface: f64 = frames.area.iter().sum::<f64>() * 3.0;
    (total > 0.05 * surface).then(|| weighted / total)
}
