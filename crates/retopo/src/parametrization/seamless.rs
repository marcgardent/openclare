//! Seamless parametrization: coordinates `(u, v)` on the cut-open surface
//! whose gradients follow the two arms of the combed cross field, and which
//! agree across every seam up to a quarter-turn rotation and a translation.
//!
//! This is the second step of Bommes et al., "Mixed-Integer Quadrangulation"
//! (2009), before any integer is imposed. With the cross of each face scaled
//! to the target edge length, the integer grid lines of `(u, v)` are where
//! the quad edges will run.
//!
//! The energy is the squared difference between the gradient of `(u, v)` and
//! the cross, summed over the faces; the seam conditions are linear equality
//! constraints. They are imposed by the augmented Lagrangian method, which
//! keeps the system symmetric positive definite: one Cholesky factorization
//! serves every iteration.

use super::cut::{CutMesh, next_corner};
use super::{Kind, Parametrization, Parametrizer, Report, Request};
use crate::crossfield::{CrossField, prepare};
use crate::error::{Error, Result};
use crate::linalg::SymmetricBuilder;
use crate::math::INVALID;
use crate::preprocess::WorkMesh;

#[derive(Clone, Debug)]
pub struct Seamless {
    /// Weight of the seam conditions relative to the energy. They end up
    /// exact whatever its value; a higher one gets there in fewer iterations.
    pub penalty: f64,
    /// Seam mismatch, in parameter units, below which the conditions count
    /// as met.
    pub seam_tolerance: f64,
    pub max_iterations: usize,
}

impl Default for Seamless {
    fn default() -> Self {
        Self { penalty: 1.0e4, seam_tolerance: 1.0e-9, max_iterations: 30 }
    }
}

/// One linear condition `sum of coefficient * unknown = 0`.
type Row = Vec<(usize, f64)>;

impl Parametrizer for Seamless {
    fn kind(&self) -> Kind {
        Kind::Seamless
    }

    fn parametrize(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Parametrization> {
        if !(request.edge_length.is_finite() && request.edge_length > 0.0) {
            return Err(Error::InvalidParams(format!("edge length {} is not positive", request.edge_length)));
        }
        let mesh = &work.mesh;
        let (topology, frames) = prepare(mesh)?;
        let cut = CutMesh::build(mesh, &topology, field);
        let faces = mesh.triangles.len();
        let unknowns = 2 * cut.wedge_count;
        let scale = 1.0 / request.edge_length;

        // The triangle of each face in the plane of its combed cross: the
        // parametrization we are after is as close to these as it can be.
        let ideal: Vec<[[f64; 2]; 3]> = (0..faces)
            .map(|f| {
                let angle = field.angles[f] + cut.turns[f] as f64 * std::f64::consts::FRAC_PI_2;
                let (u_arm, v_arm) = (frames.direction(f, angle), frames.normal[f].cross(frames.direction(f, angle)));
                mesh.corners(f).map(|p| [p.dot(u_arm) * scale, p.dot(v_arm) * scale])
            })
            .collect();

        let constraints = self.constraints(work, &topology, &cut);
        let mut weights = vec![1.0; faces];
        let mut report = Report { seam_edges: cut.seam_edges(), wedges: cut.wedge_count, ..Default::default() };
        let mut uv = Vec::new();

        for round in 0..=request.stiffening_rounds {
            let x = self.solve(&cut, &frames.area, &ideal, &weights, &constraints, unknowns, &mut report)?;
            uv = (0..3 * faces).map(|c| [x[2 * cut.wedge[c] as usize], x[2 * cut.wedge[c] as usize + 1]]).collect();

            let quality = Quality::measure(&uv, &ideal, &frames.area);
            if round == 0 {
                report.flipped_first = quality.flipped.len();
            }
            report.flipped = quality.flipped.len();
            report.distortion_mean = quality.distortion_mean;
            report.stretch_max = quality.stretch_max;
            report.rounds = round;
            if quality.flipped.is_empty() {
                break;
            }
            // Make the faces that folded over, and the ones around them,
            // stiffer, and try again.
            for &f in &quality.flipped {
                weights[f] *= 4.0;
                for h in 3 * f..3 * f + 3 {
                    let g = frames.across[h];
                    if g != INVALID {
                        weights[g as usize] *= 2.0;
                    }
                }
            }
        }

        Ok(Parametrization { cut, uv, report })
    }
}

impl Seamless {
    /// The seam conditions, plus one pinned vertex per connected piece to
    /// remove the freedom of translating the whole map.
    fn constraints(&self, work: &WorkMesh, topology: &crate::mesh::Topology, cut: &CutMesh) -> Vec<Row> {
        let faces = work.mesh.triangles.len();
        let (u, v) =
            (|corner: usize| 2 * cut.wedge[corner] as usize, |corner: usize| 2 * cut.wedge[corner] as usize + 1);
        let mut rows = Vec::new();

        for h in 0..3 * faces {
            let t = topology.opposite[h] as usize;
            if !cut.seam[h] || t < h {
                continue;
            }
            // The edge runs a -> b in face f and b -> a in face g. Written as
            // complex numbers z = u + i v, the map must satisfy
            //   z_g(a) - z_g(b) = (-i)^j (z_f(a) - z_f(b)),
            // j being the quarter turns from the cross of f to that of g.
            let (fa, fb, ga, gb) = (h, next_corner(h), next_corner(t), t);
            let (du, dv): (Row, Row) = (vec![(u(fa), 1.0), (u(fb), -1.0)], vec![(v(fa), 1.0), (v(fb), -1.0)]);
            let negate = |row: &Row| row.iter().map(|&(i, c)| (i, -c)).collect::<Row>();
            let (rotated_u, rotated_v) = match cut.transition[h] {
                0 => (du, dv),
                1 => (dv, negate(&du)),
                2 => (negate(&du), negate(&dv)),
                _ => (negate(&dv), du),
            };
            for (mut row, plus, minus) in [(rotated_u, u(ga), u(gb)), (rotated_v, v(ga), v(gb))] {
                for entry in &mut row {
                    entry.1 = -entry.1;
                }
                row.push((plus, 1.0));
                row.push((minus, -1.0));
                rows.push(merged(row));
            }
        }

        // One pin per connected piece of the surface: the first corner of
        // its first face.
        let mut visited = vec![false; faces];
        let mut stack = Vec::new();
        for start in 0..faces {
            if visited[start] {
                continue;
            }
            rows.push(vec![(u(3 * start), 1.0)]);
            rows.push(vec![(v(3 * start), 1.0)]);
            visited[start] = true;
            stack.push(start);
            while let Some(f) = stack.pop() {
                for h in 3 * f..3 * f + 3 {
                    let t = topology.opposite[h];
                    if t != INVALID && !visited[t as usize / 3] {
                        visited[t as usize / 3] = true;
                        stack.push(t as usize / 3);
                    }
                }
            }
        }
        rows
    }

    /// Minimises the energy under the constraints, by the augmented
    /// Lagrangian method.
    #[allow(clippy::too_many_arguments)]
    fn solve(
        &self,
        cut: &CutMesh,
        areas: &[f64],
        ideal: &[[[f64; 2]; 3]],
        weights: &[f64],
        constraints: &[Row],
        unknowns: usize,
        report: &mut Report,
    ) -> Result<Vec<f64>> {
        let mut matrix = SymmetricBuilder::new(unknowns);
        let mut rhs = vec![0.0; unknowns];
        let mut diagonal_sum = 0.0;
        for (f, corners) in ideal.iter().enumerate() {
            let area = areas[f];
            if area <= 0.0 {
                continue;
            }
            // Gradients of the three hat functions, in the plane of the cross.
            let ideal_area = signed_area(corners);
            if ideal_area <= 0.0 {
                continue;
            }
            let gradient: [[f64; 2]; 3] = std::array::from_fn(|i| {
                let (a, b) = (corners[(i + 1) % 3], corners[(i + 2) % 3]);
                [-(b[1] - a[1]) / (2.0 * ideal_area), (b[0] - a[0]) / (2.0 * ideal_area)]
            });
            let weight = weights[f] * area;
            for i in 0..3 {
                let wi = 2 * cut.wedge[3 * f + i] as usize;
                // The target gradients are (1, 0) for u and (0, 1) for v.
                rhs[wi] += weight * gradient[i][0];
                rhs[wi + 1] += weight * gradient[i][1];
                for j in 0..3 {
                    let wj = 2 * cut.wedge[3 * f + j] as usize;
                    let value = weight * (gradient[i][0] * gradient[j][0] + gradient[i][1] * gradient[j][1]);
                    matrix.add(wi, wj, value);
                    matrix.add(wi + 1, wj + 1, value);
                    if i == j {
                        diagonal_sum += 2.0 * value;
                    }
                }
            }
        }
        // The penalty is set relative to the typical stiffness of the energy,
        // whatever the units of the mesh and the weights of the faces.
        let penalty = self.penalty * (diagonal_sum / unknowns.max(1) as f64).max(f64::MIN_POSITIVE);
        for row in constraints {
            matrix.add_squared(row, penalty);
        }
        for i in 0..unknowns {
            matrix.add(i, i, 1e-12 * penalty);
        }
        let factorization = matrix.factorize()?;

        let apply = |row: &Row, x: &[f64]| row.iter().map(|&(i, c)| c * x[i]).sum::<f64>();
        let mut multipliers = vec![0.0; constraints.len()];
        let mut x = vec![0.0; unknowns];
        for _ in 0..self.max_iterations {
            let mut b = rhs.clone();
            for (row, &multiplier) in constraints.iter().zip(&multipliers) {
                for &(i, c) in row {
                    b[i] -= multiplier * c;
                }
            }
            x = factorization.solve(&b);
            let mut worst = 0.0f64;
            for (row, multiplier) in constraints.iter().zip(&mut multipliers) {
                let violation = apply(row, &x);
                worst = worst.max(violation.abs());
                *multiplier += penalty * violation;
            }
            report.seam_error = worst;
            if worst <= self.seam_tolerance {
                return Ok(x);
            }
        }
        Err(Error::Numerical(format!(
            "seam conditions not met after {} iterations (mismatch {:.3e})",
            self.max_iterations, report.seam_error
        )))
    }
}

/// Sums the coefficients of repeated unknowns and drops those that cancel.
fn merged(mut row: Row) -> Row {
    row.sort_unstable_by_key(|&(i, _)| i);
    let mut out: Row = Vec::with_capacity(row.len());
    for (i, c) in row {
        match out.last_mut() {
            Some(last) if last.0 == i => last.1 += c,
            _ => out.push((i, c)),
        }
    }
    out.retain(|&(_, c)| c != 0.0);
    out
}

fn signed_area(corners: &[[f64; 2]; 3]) -> f64 {
    let (a, b, c) = (corners[0], corners[1], corners[2]);
    0.5 * ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]))
}

/// How far the map of each face is from its ideal, a rigid copy of the face.
struct Quality {
    /// Faces mapped with the wrong orientation, or squashed flat.
    flipped: Vec<usize>,
    /// Area-weighted mean of `|J - I|`, `J` being the map from the ideal
    /// triangle to the actual one.
    distortion_mean: f64,
    /// Largest ratio between the two stretches of a face.
    stretch_max: f64,
}

impl Quality {
    fn measure(uv: &[[f64; 2]], ideal: &[[[f64; 2]; 3]], areas: &[f64]) -> Self {
        let mut quality = Quality { flipped: Vec::new(), distortion_mean: 0.0, stretch_max: 1.0 };
        let mut total_area = 0.0;
        for (f, corners) in ideal.iter().enumerate() {
            let ideal_area = signed_area(corners);
            if ideal_area <= 0.0 || areas[f] <= 0.0 {
                continue;
            }
            let actual = [uv[3 * f], uv[3 * f + 1], uv[3 * f + 2]];
            // J maps the edges of the ideal triangle onto the actual ones.
            let edge = |t: &[[f64; 2]; 3], k: usize| [t[k][0] - t[0][0], t[k][1] - t[0][1]];
            let (p1, p2, q1, q2) = (edge(corners, 1), edge(corners, 2), edge(&actual, 1), edge(&actual, 2));
            let det = p1[0] * p2[1] - p1[1] * p2[0];
            let j = [
                [(q1[0] * p2[1] - q2[0] * p1[1]) / det, (q2[0] * p1[0] - q1[0] * p2[0]) / det],
                [(q1[1] * p2[1] - q2[1] * p1[1]) / det, (q2[1] * p1[0] - q1[1] * p2[0]) / det],
            ];
            let jacobian = j[0][0] * j[1][1] - j[0][1] * j[1][0];
            // A sliver in parameter space is as useless as a fold.
            if jacobian <= 1e-3 {
                quality.flipped.push(f);
            }
            let off = ((j[0][0] - 1.0).powi(2) + j[0][1].powi(2) + j[1][0].powi(2) + (j[1][1] - 1.0).powi(2)).sqrt();
            quality.distortion_mean += areas[f] * off;
            total_area += areas[f];

            // Singular values of J, from its Frobenius norm and determinant.
            let norm = j[0][0].powi(2) + j[0][1].powi(2) + j[1][0].powi(2) + j[1][1].powi(2);
            let gap = (norm * norm - 4.0 * jacobian * jacobian).max(0.0).sqrt();
            let (largest, smallest) = (((norm + gap) / 2.0).sqrt(), ((norm - gap) / 2.0).max(0.0).sqrt());
            if jacobian > 1e-3 && smallest > 0.0 {
                quality.stretch_max = quality.stretch_max.max(largest / smallest);
            }
        }
        quality.distortion_mean /= total_area.max(f64::MIN_POSITIVE);
        quality
    }
}
