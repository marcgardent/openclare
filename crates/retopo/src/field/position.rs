//! 4-PoSy position field: where the quad edges sit.
//!
//! Given the orientation field, every vertex carries a square lattice of
//! spacing `scale` in its tangent plane. Smoothing makes neighbouring
//! lattices coincide, so that they all describe the same global grid, whose
//! points become the vertices of the quad mesh.

use super::rosy::{compat_position, lattice_round};
use crate::hierarchy::{Hierarchy, Level};
use crate::math::{INVALID, Rng, tangent_frame};

/// Solves the position field on every level, coarsest first. The orientation
/// field must already be solved.
pub fn solve(hierarchy: &mut Hierarchy, scale: f64, iterations: usize, rng: &mut Rng) {
    for level in &mut hierarchy.levels {
        randomize(level, scale, rng);
    }
    hierarchy.for_each_coarse_to_fine(|level, finer| {
        let mut order: Vec<u32> = (0..level.len() as u32).collect();
        rng.shuffle(&mut order);
        smooth(level, scale, iterations, &order);
        if let Some(finer) = finer {
            prolong(level, finer);
        }
    });
}

fn randomize(level: &mut Level, scale: f64, rng: &mut Rng) {
    for i in 0..level.len() {
        let (s, t) = tangent_frame(level.normals[i]);
        let (x, y) = (rng.next_f64() - 0.5, rng.next_f64() - 0.5);
        level.origin[i] = level.positions[i] + (s * x + t * y) * scale;
    }
}

/// Gauss-Seidel sweeps: each lattice origin becomes the running average of
/// its neighbours' nearest lattice points, then snaps back to the lattice
/// point closest to its own vertex.
pub fn smooth(level: &mut Level, scale: f64, iterations: usize, order: &[u32]) {
    let inv_scale = 1.0 / scale;
    let Level { positions, normals, graph, constraints, orientation, origin, .. } = level;
    for _ in 0..iterations {
        for &i in order {
            let i = i as usize;
            let (p, n, q) = (positions[i], normals[i], orientation[i]);
            let mut sum = origin[i];
            let mut weight_sum = 0.0;
            for link in graph.neighbors(i) {
                let j = link.id as usize;
                let (a, b) = compat_position(
                    p,
                    n,
                    q,
                    sum,
                    positions[j],
                    normals[j],
                    orientation[j],
                    origin[j],
                    scale,
                    inv_scale,
                );
                sum = (a * weight_sum + b * link.weight) / (weight_sum + link.weight);
                weight_sum += link.weight;
                sum -= n * n.dot(sum - p);
            }

            let w = constraints.point_weight[i];
            if w > 0.0 {
                // Pull the lattice onto the constraint point, except along the
                // constrained direction where it stays free to slide.
                let along = constraints.direction[i] * constraints.direction_weight[i].min(1.0);
                let mut pull = constraints.point[i] - sum;
                pull -= along * along.dot(pull);
                sum += pull * w;
                sum -= n * n.dot(sum - p);
            }
            if sum.is_finite() {
                origin[i] = lattice_round(sum, q, n, p, scale, inv_scale);
            }
        }
    }
}

/// Initialises the finer level from the solution of the coarser one.
fn prolong(coarse: &Level, fine: &mut Level) {
    for (c, children) in coarse.children.iter().enumerate() {
        for &child in children.iter().filter(|&&v| v != INVALID) {
            let child = child as usize;
            let (n, p) = (fine.normals[child], fine.positions[child]);
            let o = coarse.origin[c];
            fine.origin[child] = o - n * n.dot(o - p);
        }
    }
}
