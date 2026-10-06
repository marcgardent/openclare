//! 4-RoSy orientation field: which way the quad edges run.

use std::f64::consts::TAU;

use super::rosy::{compat_orientation, compat_orientation_index};
use crate::hierarchy::{Hierarchy, Level};
use crate::math::{INVALID, Rng, Vec3, tangent_frame};
use crate::mesh::TriMesh;

/// Solves the orientation field on every level, coarsest first.
pub fn solve(hierarchy: &mut Hierarchy, iterations: usize, rng: &mut Rng) {
    for level in &mut hierarchy.levels {
        randomize(level, rng);
    }
    hierarchy.for_each_coarse_to_fine(|level, finer| {
        let mut order: Vec<u32> = (0..level.len() as u32).collect();
        rng.shuffle(&mut order);
        smooth(level, iterations, &order);
        if let Some(finer) = finer {
            prolong(level, finer);
        }
    });
}

fn randomize(level: &mut Level, rng: &mut Rng) {
    for (q, &n) in level.orientation.iter_mut().zip(&level.normals) {
        let (s, t) = tangent_frame(n);
        let angle = TAU * rng.next_f64();
        *q = s * angle.cos() + t * angle.sin();
    }
}

/// Gauss-Seidel sweeps: each cross becomes the running average of its
/// neighbours' best-matching arms, re-projected onto its tangent plane.
pub fn smooth(level: &mut Level, iterations: usize, order: &[u32]) {
    let Level { normals, graph, constraints, orientation, .. } = level;
    for _ in 0..iterations {
        for &i in order {
            let i = i as usize;
            let n = normals[i];
            let mut sum = orientation[i];
            let mut weight_sum = 0.0;
            for link in graph.neighbors(i) {
                let j = link.id as usize;
                let (a, b) = compat_orientation(sum, n, orientation[j], normals[j]);
                sum = project_unit(a * weight_sum + b * link.weight, n).unwrap_or(sum);
                weight_sum += link.weight;
            }

            let w = constraints.direction_weight[i];
            if w > 0.0 {
                let (a, b) = compat_orientation(sum, n, constraints.direction[i], n);
                sum = project_unit(a * (1.0 - w) + b * w, n).unwrap_or(sum);
            }
            orientation[i] = sum;
        }
    }
}

/// Initialises the finer level from the solution of the coarser one.
fn prolong(coarse: &Level, fine: &mut Level) {
    for (c, children) in coarse.children.iter().enumerate() {
        for &child in children.iter().filter(|&&v| v != INVALID) {
            let child = child as usize;
            if let Some(q) = project_unit(coarse.orientation[c], fine.normals[child]) {
                fine.orientation[child] = q;
            }
        }
    }
}

#[inline]
fn project_unit(v: Vec3, n: Vec3) -> Option<Vec3> {
    (v - n * n.dot(v)).try_normalize()
}

/// A triangle around which the field makes a net quarter turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Singularity {
    pub face: u32,
    /// `+1` for index +1/4 (becomes a valence-3 vertex), `-1` for index -1/4
    /// (valence 5).
    pub index: i8,
}

/// Singular triangles of a per-vertex orientation field.
///
/// By Poincaré-Hopf the indices sum to `4 χ` on a closed surface: a sphere
/// cannot do with fewer than eight valence-3 vertices, a torus needs none.
pub fn singularities(mesh: &TriMesh, normals: &[Vec3], orientation: &[Vec3]) -> Vec<Singularity> {
    let mut found = Vec::new();
    for (f, tri) in mesh.triangles.iter().enumerate() {
        let mut turns = 0;
        for k in 0..3 {
            let (i, j) = (tri[k] as usize, tri[(k + 1) % 3] as usize);
            let (a, b) = compat_orientation_index(orientation[i], normals[i], orientation[j], normals[j]);
            turns += b - a;
        }
        match turns.rem_euclid(4) {
            1 => found.push(Singularity { face: f as u32, index: 1 }),
            3 => found.push(Singularity { face: f as u32, index: -1 }),
            _ => {}
        }
    }
    found
}
