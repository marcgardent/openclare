//! Tubes of a seamless map: where a grid line goes round a part of the
//! shape and comes back beside itself.
//!
//! On a limb, a line of constant `v` goes round it. If the map has no shift
//! across that turn, the line closes into a ring; otherwise it comes back a
//! little higher or lower and goes round again, a helix, and so does every
//! line of the layout cut from the map, and every edge loop of the mesh
//! quantized from it (DT-062). This finds such turns and measures their
//! shift, in quad edges.

use std::collections::HashSet;

use super::walk::{Axis, EPSILON, Exit, Map, axis_vector};
use crate::mesh::{Topology, TriMesh};
use crate::parametrization::{CutMesh, Parametrization};
use crate::preprocess::WorkMesh;

/// A grid line that went round and came back beside where it started.
#[derive(Clone, Debug, PartialEq)]
pub struct Tube {
    /// Face and point it started from, in the coordinates of that face, and
    /// the direction it went.
    pub face: usize,
    pub start: [f64; 2],
    pub axis: Axis,
    /// How far it went before coming back, in quad edges: once round.
    pub around: f64,
    /// How far beside its start it came back, across its direction, in
    /// quad edges: 0 for a ring.
    pub shift: f64,
    /// The seams it crossed on the way, in order: each the half-edge of the
    /// face it left. What the map turns and shifts by there makes the shift.
    pub crossings: Vec<usize>,
}

/// Lines started from about `samples` points spread over the surface, along
/// both directions, each followed for at most `longest` quad edges; one
/// that comes back within `gate` quad edges of its start, across, is a turn.
pub fn tubes(work: &WorkMesh, map: &Parametrization, samples: usize, longest: f64, gate: f64) -> Vec<Tube> {
    tubes_of(&work.mesh, &map.uv, &map.cut, &map.set_aside, samples, longest, gate)
}

/// The same, from the parts of a map.
pub(crate) fn tubes_of(
    mesh: &TriMesh,
    uv: &[[f64; 2]],
    cut: &CutMesh,
    set_aside: &[bool],
    samples: usize,
    longest: f64,
    gate: f64,
) -> Vec<Tube> {
    let topology = Topology::build(mesh);
    let walk = Map::new(mesh, &topology, uv, &cut.seam, &cut.transition);
    let faces = mesh.triangles.len();
    let stride = (faces / samples.max(1)).max(1);
    let mut found = Vec::new();
    for face in (0..faces).step_by(stride) {
        if set_aside[face] || walk.area(face) <= 0.0 {
            continue;
        }
        let [a, b, c] = walk.corners(face);
        let start = [(a[0] + b[0] + c[0]) / 3.0, (a[1] + b[1] + c[1]) / 3.0];
        for axis in [0, 1] {
            if let Some(tube) = turn(&walk, face, start, axis, longest, gate) {
                found.push(tube);
            }
        }
    }
    found
}

/// The faces a short segment across `axis` through `start` passes, both
/// ways up to `gate`, without crossing a seam: they share the coordinates
/// of `face`.
fn gate_faces(walk: &Map, face: usize, start: [f64; 2], axis: Axis, gate: f64) -> HashSet<usize> {
    let mut faces = HashSet::from([face]);
    for across in [(axis + 1) % 4, (axis + 3) % 4] {
        let (mut f, mut p, mut left) = (face, start, gate);
        while left > 0.0 {
            let Some((exit, _, reach)) = walk.exit(f, p, across) else { break };
            if reach >= left {
                break;
            }
            left -= reach;
            let Exit::Edge { edge, t } = exit else { break };
            if walk.seam[3 * f + edge] {
                break;
            }
            let Some((g, q, _)) = walk.cross_edge(f, edge, t, across) else { break };
            faces.insert(g);
            (f, p) = (g, q);
        }
    }
    faces
}

/// Follows the line through `start` along `axis`: the turn it makes, if it
/// comes back through the gate beside its start.
fn turn(walk: &Map, face: usize, start: [f64; 2], axis: Axis, longest: f64, gate: f64) -> Option<Tube> {
    let gate_faces = gate_faces(walk, face, start, axis, gate);
    let along = (axis % 2) as usize;
    let across = 1 - along;
    let sign = axis_vector(axis)[along];
    let (mut f, mut p, mut a, mut travelled) = (face, start, axis, 0.0);
    let mut crossings = Vec::new();
    // Far more triangles than a line crosses on its way round.
    for _ in 0..walk.mesh.triangles.len().max(64) {
        let (exit, out, reach) = walk.exit(f, p, a)?;
        // Back among the faces of the gate, going the same way, and level
        // with the start along the line on the way through this face.
        if travelled > 2.0 * gate && a == axis && gate_faces.contains(&f) {
            let (from, to) = (p[along] * sign, out[along] * sign);
            let level = start[along] * sign;
            let shift = p[across] - start[across];
            if from - EPSILON <= level && level <= to + EPSILON && shift.abs() <= gate {
                return Some(Tube { face, start, axis, around: travelled + level - from, shift, crossings });
            }
        }
        travelled += reach;
        if travelled > longest {
            return None;
        }
        match exit {
            Exit::Edge { edge, t } => {
                if walk.seam[3 * f + edge] {
                    crossings.push(3 * f + edge);
                }
                (f, p, a) = walk.cross_edge(f, edge, t, a)?;
            }
            Exit::Vertex { corner } => {
                // Round the vertex counter-clockwise to the sector the line
                // goes on in, the seams crossed on the way with it.
                let (mut g, mut at, mut carried) = (f, corner, a);
                let mut turns = 0;
                while !walk.in_sector(g, at, carried) {
                    let h = 3 * g + (at + 2) % 3;
                    if walk.seam[h] {
                        crossings.push(h);
                    }
                    (g, at, carried) = walk.next_around(g, at, carried)?;
                    turns += 1;
                    if turns > 64 {
                        return None;
                    }
                }
                (f, p, a) = (g, walk.uv[3 * g + at], carried);
            }
        }
    }
    None
}
