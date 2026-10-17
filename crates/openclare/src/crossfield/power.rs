//! The smoothest cross field, after Knöppel et al., "Globally Optimal
//! Direction Fields" (2013), discretised per face.
//!
//! A cross of angle `θ` is the complex number `u = e^{4iθ}`, the same for its
//! four arms. Smoothness between two faces sharing an edge is
//! `|u_g - ρ u_f|²`, where `ρ` carries `u_f` across the edge. The sum over
//! all edges is a quadratic form `u* A u`; curvature alignment and feature
//! constraints add `s_f |u_f - t_f|²` per face. The minimiser solves
//! `(A + S) u = S t`. With nothing to align to, the system has no right-hand
//! side and the smoothest field is the eigenvector of `A` for its smallest
//! eigenvalue instead.
//!
//! Each face has at most three neighbours, so `A` is applied directly from
//! the mesh and the systems are solved by conjugate gradients.

use super::{CrossField, CrossFieldSolver, FaceFrames, Kind, Request, finish, prepare_sized};
use crate::error::Result;
use crate::math::{Complex, INVALID};
use crate::preprocess::WorkMesh;

/// Weight that makes a feature constraint effectively exact.
const HARD: f64 = 1.0e6;

#[derive(Clone, Debug)]
pub struct Power {
    /// Relative residual at which the linear solves stop.
    pub tolerance: f64,
    pub max_iterations: usize,
    /// Most rounds of inverse iteration when there is nothing to align to.
    pub eigen_rounds: usize,
    /// Rounds of pulling the field back towards crosses of unit size, where
    /// hard constraints lead it (see [`System::towards_unit_crosses`]), how
    /// hard each pulls, per face of mean area, and the relative residual at
    /// which each round's solve stops.
    pub unit_rounds: usize,
    pub unit_pull: f64,
    pub unit_tolerance: f64,
}

impl Default for Power {
    fn default() -> Self {
        Self {
            tolerance: 1e-7,
            max_iterations: 20_000,
            eigen_rounds: 200,
            unit_rounds: 5,
            unit_pull: 0.01,
            unit_tolerance: 1e-3,
        }
    }
}

impl CrossFieldSolver for Power {
    fn kind(&self) -> Kind {
        Kind::Power
    }

    fn solve(&self, work: &WorkMesh, request: &Request) -> Result<CrossField> {
        let (topology, frames) = prepare_sized(work)?;
        let (strength, target) = constraints(work, &frames, request.alignment);

        let system = System::new(&frames, &strength);
        let u = if strength.iter().any(|&s| s > 0.0) {
            let rhs: Vec<Complex> = strength.iter().zip(&target).map(|(&s, &t)| t * s).collect();
            let mut u = system.solve(&rhs, &target, self.tolerance, self.max_iterations);
            // Only where hard edges lead the field. Soft alignment keeps it
            // near unit size wherever the shape bends; pulled back there too,
            // the singularities of organic shapes moved, and their loops
            // spiralled more (12 models at 3000 quads: 9.8 % of the loop
            // edges on spirals against 7.2 %, over three budgets).
            if strength.iter().any(|&s| s >= HARD) {
                system.towards_unit_crosses(&mut u, &rhs, self);
            }
            u
        } else {
            system.smoothest(self)
        };

        let angles = u.iter().map(|z| if z.norm_squared() > 0.0 { z.arg() / 4.0 } else { 0.0 }).collect();
        Ok(finish(&work.mesh, &topology, &frames, angles))
    }
}

/// Per face: how strongly it is tied to a target cross, and that target.
pub(crate) fn constraints(work: &WorkMesh, frames: &FaceFrames, alignment: f64) -> (Vec<f64>, Vec<Complex>) {
    let faces = frames.len();
    let mean_area = frames.area.iter().sum::<f64>() / faces as f64;
    let mut strength = vec![0.0; faces];
    let mut target = vec![Complex::ZERO; faces];

    // Soft: towards the principal directions of the three corners, as far as
    // they are trustworthy and agree with one another.
    if alignment > 0.0 {
        for (f, tri) in work.mesh.triangles.iter().enumerate() {
            if frames.area[f] == 0.0 {
                continue;
            }
            let mut sum = Complex::ZERO;
            for &v in tri {
                let direction = work.curvature.dir1[v as usize];
                sum += Complex::from_angle(4.0 * frames.angle_of(f, direction)) * work.confidence[v as usize];
            }
            let agreement = sum.abs() / 3.0;
            if agreement > 0.0 {
                target[f] = sum * (1.0 / sum.abs());
                strength[f] = alignment * agreement * frames.area[f] / mean_area;
            }
        }
    }

    // Hard: a face on a feature edge has an arm along it.
    if !work.feature_edges.is_empty() {
        let mut hard = vec![Complex::ZERO; faces];
        let on_feature: std::collections::HashSet<(u32, u32)> =
            work.feature_edges.iter().map(|&[a, b]| (a.min(b), a.max(b))).collect();
        for (f, tri) in work.mesh.triangles.iter().enumerate() {
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                if on_feature.contains(&(a.min(b), a.max(b))) && frames.area[f] > 0.0 {
                    let edge = work.mesh.positions[b as usize] - work.mesh.positions[a as usize];
                    hard[f] += Complex::from_angle(4.0 * frames.angle_of(f, edge));
                }
            }
        }
        for f in 0..faces {
            // Two feature edges at an odd angle cancel out: such a corner
            // triangle is left free rather than given an arbitrary cross.
            if hard[f].abs() > 0.5 {
                target[f] = hard[f] * (1.0 / hard[f].abs());
                strength[f] = HARD;
            }
        }
    }
    (strength, target)
}

/// The operator `A + S` of the module documentation, applied from the mesh.
struct System<'a> {
    frames: &'a FaceFrames,
    /// `e^{-4i transport}` per half-edge: carries the neighbour's value into
    /// the face of the half-edge.
    carry: Vec<Complex>,
    diagonal: Vec<f64>,
}

impl<'a> System<'a> {
    fn new(frames: &'a FaceFrames, strength: &[f64]) -> Self {
        let carry = (0..frames.rotation.len()).map(|h| Complex::from_angle(-4.0 * frames.transport(h))).collect();
        // The tiny shift keeps isolated or flat faces from making it singular.
        let diagonal = (0..frames.len())
            .map(|f| strength[f] + frames.weight[3 * f..3 * f + 3].iter().sum::<f64>() + 1e-12)
            .collect();
        Self { frames, carry, diagonal }
    }

    fn apply(&self, u: &[Complex], shift: &[f64], out: &mut [Complex]) {
        for f in 0..self.frames.len() {
            let mut value = u[f] * (self.diagonal[f] + shift[f]);
            for h in 3 * f..3 * f + 3 {
                let g = self.frames.across[h];
                if g != INVALID {
                    value = value - self.carry[h] * u[g as usize] * self.frames.weight[h];
                }
            }
            out[f] = value;
        }
    }

    /// Conjugate gradients with diagonal preconditioning, for
    /// `(A + S + shift) u = rhs`.
    fn conjugate_gradients(
        &self,
        rhs: &[Complex],
        shift: &[f64],
        start: &[Complex],
        tolerance: f64,
        max_iterations: usize,
    ) -> Vec<Complex> {
        let n = rhs.len();
        let dot = |a: &[Complex], b: &[Complex]| a.iter().zip(b).map(|(x, y)| (x.conj() * *y).re).sum::<f64>();
        let precondition = |r: &[Complex], z: &mut [Complex]| {
            for f in 0..n {
                z[f] = r[f] * (1.0 / (self.diagonal[f] + shift[f]));
            }
        };

        let mut u = start.to_vec();
        let mut scratch = vec![Complex::ZERO; n];
        self.apply(&u, shift, &mut scratch);
        let mut residual: Vec<Complex> = rhs.iter().zip(&scratch).map(|(b, a)| *b - *a).collect();
        let mut z = vec![Complex::ZERO; n];
        precondition(&residual, &mut z);
        let mut direction = z.clone();
        let mut rz = dot(&residual, &z);
        // Measured in the preconditioned norm, so that the huge weights of
        // hard constraints do not dominate the stopping test.
        let stop = tolerance * tolerance * rz.max(f64::MIN_POSITIVE);

        for _ in 0..max_iterations {
            if rz <= stop {
                break;
            }
            self.apply(&direction, shift, &mut scratch);
            let step = rz / dot(&direction, &scratch);
            for f in 0..n {
                u[f] += direction[f] * step;
                residual[f] = residual[f] - scratch[f] * step;
            }
            precondition(&residual, &mut z);
            let next = dot(&residual, &z);
            let ratio = next / rz;
            rz = next;
            for f in 0..n {
                direction[f] = z[f] + direction[f] * ratio;
            }
        }
        u
    }

    fn solve(&self, rhs: &[Complex], start: &[Complex], tolerance: f64, max_iterations: usize) -> Vec<Complex> {
        let no_shift = vec![0.0; rhs.len()];
        self.conjugate_gradients(rhs, &no_shift, start, tolerance, max_iterations)
    }

    /// Pulls the solution `u` of `(A + S) u = rhs` back towards crosses of
    /// unit size, a few times over: each round solves
    /// `(A + S + p M) u' = rhs + p M u/|u|`, an implicit step of a
    /// Ginzburg-Landau flow (diffusion, then back to unit size, as in the
    /// MBO schemes for cross fields of Viertel and Osting, 2019).
    ///
    /// Hard constraints are unit crosses held on lines, and the least-squares
    /// field between them is their harmonic extension: across a face whose
    /// hard edges wind round it, it shrinks towards the middle. On the flat
    /// cap of a cylinder it is `z⁴`, one zero of order four at the centre,
    /// and where it is that small its angle is decided by whatever else is
    /// left, the unevenness of the mesh: the four singularities scatter
    /// anywhere, and the lines out of them miss one another by several quads.
    /// Back at unit size, the field has them where the smoothest field of
    /// unit crosses does: on a square whose sides are lines of the field.
    fn towards_unit_crosses(&self, u: &mut Vec<Complex>, rhs: &[Complex], settings: &Power) {
        let n = self.frames.len();
        let mean_area = self.frames.area.iter().sum::<f64>() / n as f64;
        let pull: Vec<f64> = self.frames.area.iter().map(|a| settings.unit_pull * a / mean_area).collect();
        for _ in 0..settings.unit_rounds {
            let pulled: Vec<Complex> = (0..n)
                .map(|f| {
                    let size = u[f].abs();
                    let unit = if size > 0.0 { u[f] * (1.0 / size) } else { Complex::ZERO };
                    rhs[f] + unit * pull[f]
                })
                .collect();
            // Each round is a step on the way, not an end: solved roughly.
            *u = self.conjugate_gradients(&pulled, &pull, u, settings.unit_tolerance, settings.max_iterations);
        }
    }

    /// Eigenvector of `A u = λ M u` for the smallest `λ`, by inverse
    /// iteration: repeatedly solving against `A` amplifies the smoothest
    /// component of the current guess.
    fn smoothest(&self, settings: &Power) -> Vec<Complex> {
        let n = self.frames.len();
        let mean_area = self.frames.area.iter().sum::<f64>() / n as f64;
        let mass: Vec<f64> = self.frames.area.iter().map(|a| a / mean_area).collect();
        // A small shift keeps the solve well posed where a perfectly smooth
        // field exists (flat pieces), without changing which vector wins.
        let shift: Vec<f64> = mass.iter().map(|m| 1e-3 * m).collect();

        // Start from a field that is already smooth on a large scale: the
        // crosses that have an arm along a fixed direction in space.
        let mut u: Vec<Complex> = (0..n)
            .map(|f| {
                let normal = self.frames.normal[f];
                let reference = [crate::math::Vec3::X, crate::math::Vec3::Y]
                    .into_iter()
                    .max_by(|a, b| a.cross(normal).length_squared().total_cmp(&b.cross(normal).length_squared()))
                    .unwrap();
                Complex::from_angle(4.0 * self.frames.angle_of(f, reference))
            })
            .collect();

        let no_shift = vec![0.0; n];
        let mut scratch = vec![Complex::ZERO; n];
        let mut previous = f64::INFINITY;
        for round in 0..settings.eigen_rounds {
            let rhs: Vec<Complex> = u.iter().zip(&mass).map(|(z, &m)| *z * m).collect();
            u = self.conjugate_gradients(&rhs, &shift, &u, settings.tolerance, settings.max_iterations);
            let norm = u.iter().zip(&mass).map(|(z, &m)| m * z.norm_squared()).sum::<f64>().sqrt();
            if norm == 0.0 {
                break;
            }
            for z in &mut u {
                *z = *z * (n as f64).sqrt() * (1.0 / norm);
            }
            // Rayleigh quotient: the energy of the current field. Stop when it
            // no longer decreases.
            self.apply(&u, &no_shift, &mut scratch);
            let energy = u.iter().zip(&scratch).map(|(a, b)| (a.conj() * *b).re).sum::<f64>() / n as f64;
            if std::env::var_os("OPENCLARE_TRACE").is_some() {
                eprintln!("round {round}: energy {energy:.9}");
            }
            if previous - energy <= 1e-7 * energy.abs() {
                break;
            }
            previous = energy;
        }
        u
    }
}
