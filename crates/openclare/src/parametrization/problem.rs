//! What the parametrizers share: the cut-open surface, the energy that pulls
//! `(u, v)` towards the cross field, the linear conditions it can be put
//! under, and the solver.
//!
//! The energy is the squared difference between the gradient of `(u, v)` and
//! the two arms of the combed cross, summed over the faces (the second step
//! of Bommes et al., "Mixed-Integer Quadrangulation", 2009). Conditions are
//! linear equalities, imposed by the augmented Lagrangian method: that keeps
//! the system symmetric positive definite, and one Cholesky factorization
//! serves every iteration.

use std::collections::HashMap;
use std::f64::consts::FRAC_PI_2;

use super::cut::{CutMesh, next_corner};
use crate::crossfield::{CrossField, FaceFrames, prepare};
use crate::error::{Error, Result};
use crate::linalg::SymmetricBuilder;
use crate::math::INVALID;
use crate::mesh::Topology;
use crate::preprocess::WorkMesh;

/// A linear expression in the unknowns: `(index, coefficient)` pairs.
pub(super) type Row = Vec<(usize, f64)>;

/// The condition `row . x = target`.
#[derive(Clone, Debug)]
pub(super) struct Constraint {
    pub row: Row,
    pub target: f64,
}

impl Constraint {
    pub fn zero(row: Row) -> Self {
        Self { row, target: 0.0 }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Solver {
    /// Weight of the conditions relative to the energy. They end up exact
    /// whatever its value; a higher one gets there in fewer iterations.
    pub penalty: f64,
    /// Mismatch, in parameter units, below which the conditions count as met.
    pub tolerance: f64,
    pub max_iterations: usize,
    /// Iterations allowed without the mismatch halving before the
    /// conditions are declared contradictory.
    pub patience: usize,
}

impl Default for Solver {
    fn default() -> Self {
        Self { penalty: 1.0e4, tolerance: 1.0e-9, max_iterations: 400, patience: 12 }
    }
}

/// A face with less than this share of the mean area is a sliver.
const SLIVER: f64 = 1e-9;

pub(super) struct Problem<'a> {
    pub work: &'a WorkMesh,
    pub topology: Topology,
    pub frames: FaceFrames,
    pub cut: CutMesh,
    /// The triangle of each face laid in the plane of its combed cross, in
    /// units of the edge length wanted there: what its map should be a copy
    /// of.
    pub ideal: Vec<[[f64; 2]; 3]>,
    /// Arms of the combed cross of each face.
    pub arms: Vec<[crate::math::Vec3; 2]>,
    pub unknowns: usize,
}

impl<'a> Problem<'a> {
    pub fn new(work: &'a WorkMesh, field: &CrossField, edge_length: f64) -> Result<Self> {
        if !(edge_length.is_finite() && edge_length > 0.0) {
            return Err(Error::InvalidParams(format!("edge length {edge_length} is not positive")));
        }
        let mesh = &work.mesh;
        let (topology, mut frames) = prepare(mesh)?;
        // Slivers with next to no area (the preprocessing leaves some along
        // hard edges) have no shape for the map to copy, and gradients large
        // enough to drown everything else: they count as having no area.
        let typical = frames.area.iter().sum::<f64>() / frames.area.len().max(1) as f64;
        for area in &mut frames.area {
            if *area < SLIVER * typical {
                *area = 0.0;
            }
        }
        let cut = CutMesh::build(mesh, &topology, field);
        let scale = 1.0 / edge_length;
        let arms: Vec<[crate::math::Vec3; 2]> = (0..mesh.triangles.len())
            .map(|f| {
                let u_arm = frames.direction(f, field.angles[f] + cut.turns[f] as f64 * FRAC_PI_2);
                [u_arm, frames.normal[f].cross(u_arm)]
            })
            .collect();
        let ideal = (0..mesh.triangles.len())
            .map(|f| {
                // Where quads are to be smaller, the map runs faster: the
                // local edge length is the unit.
                let local: f64 = mesh.triangles[f].iter().map(|&v| work.sizing[v as usize].ln()).sum::<f64>() / 3.0;
                let scale = scale / local.exp();
                mesh.corners(f).map(|p| [p.dot(arms[f][0]) * scale, p.dot(arms[f][1]) * scale])
            })
            .collect();
        let unknowns = 2 * cut.wedge_count;
        let mut problem = Self { work, topology, frames, cut, ideal, arms, unknowns };
        // A triangle with two edges on the same feature line (it cuts a
        // slight bend of the line) has all three corners on one grid line
        // once that line is: it is bound to be flat in the map. Like a
        // sliver, it is not something to fit, nor a fold to fight.
        let mut held = vec![[0u8; 2]; mesh.triangles.len()];
        for (f, _, along_u) in problem.edges_on_lines() {
            held[f][along_u as usize] += 1;
        }
        for (f, count) in held.into_iter().enumerate() {
            if count[0] >= 2 || count[1] >= 2 {
                problem.frames.area[f] = 0.0;
            }
        }
        Ok(problem)
    }

    /// Index of the `u` unknown of a corner; its `v` follows.
    #[inline]
    pub fn u(&self, corner: usize) -> usize {
        2 * self.cut.wedge[corner] as usize
    }

    #[inline]
    pub fn v(&self, corner: usize) -> usize {
        self.u(corner) + 1
    }

    /// For seam half-edge `h` (a -> b in face f, b -> a across in face g):
    /// the two expressions `z_g(x) - (-i)^j z_f(x)` at corner pair `(in f, in
    /// g)`, as real and imaginary parts. `j` is the transition of `h`.
    fn across(&self, h: usize, in_f: usize, in_g: usize) -> [Row; 2] {
        let (fu, fv) = ((self.u(in_f), 1.0), (self.v(in_f), 1.0));
        let negated = |(i, c): (usize, f64)| (i, -c);
        // (-i)^j (u + i v), as (real part, imaginary part).
        let (real, imaginary) = match self.cut.transition[h] {
            0 => (fu, fv),
            1 => (fv, negated(fu)),
            2 => (negated(fu), negated(fv)),
            _ => (negated(fv), fu),
        };
        [merged(vec![(self.u(in_g), 1.0), negated(real)]), merged(vec![(self.v(in_g), 1.0), negated(imaginary)])]
    }

    /// The seam conditions (across a seam the map may only turn by the
    /// transition and shift), plus one pinned corner per connected piece to
    /// remove the freedom of translating the whole map.
    pub fn seam_constraints(&self) -> Vec<Constraint> {
        let faces = self.work.mesh.triangles.len();
        let mut constraints = Vec::new();
        for h in 0..3 * faces {
            let t = self.topology.opposite[h] as usize;
            if !self.cut.seam[h] || t < h {
                continue;
            }
            // The shift is the same at both ends of the edge.
            let at_a = self.across(h, h, next_corner(t));
            let at_b = self.across(h, next_corner(h), t);
            for (a, b) in at_a.into_iter().zip(at_b) {
                let difference = a.into_iter().chain(b.into_iter().map(|(i, c)| (i, -c))).collect();
                constraints.push(Constraint::zero(merged(difference)));
            }
        }

        let mut visited = vec![false; faces];
        let mut stack = Vec::new();
        for start in 0..faces {
            if visited[start] {
                continue;
            }
            constraints.push(Constraint::zero(vec![(self.u(3 * start), 1.0)]));
            constraints.push(Constraint::zero(vec![(self.v(3 * start), 1.0)]));
            visited[start] = true;
            stack.push(start);
            while let Some(f) = stack.pop() {
                for h in 3 * f..3 * f + 3 {
                    let t = self.topology.opposite[h];
                    if t != INVALID && !visited[t as usize / 3] {
                        visited[t as usize / 3] = true;
                        stack.push(t as usize / 3);
                    }
                }
            }
        }
        constraints
    }

    /// Coordinates that must be whole numbers for the grid lines to close up
    /// into quads: both coordinates of every singular vertex, and the shift
    /// across every seam that no singularity already decides.
    pub fn integer_expressions(&self, field: &CrossField) -> Vec<Row> {
        let mesh = &self.work.mesh;
        let mut expressions = Vec::new();
        let mut seen = vec![false; self.cut.wedge_count];
        for (corner, &vertex) in mesh.triangles.iter().flatten().enumerate() {
            let wedge = self.cut.wedge[corner] as usize;
            if field.indices[vertex as usize] != 0 && !std::mem::replace(&mut seen[wedge], true) {
                expressions.push(vec![(self.u(corner), 1.0)]);
                expressions.push(vec![(self.v(corner), 1.0)]);
            }
        }

        // Seam edges are grouped into stretches that share one shift: two
        // edges meeting at a vertex do when they see the same pair of copies
        // of it. A stretch that touches a singularity gets its shift from the
        // (whole) coordinates of that singularity.
        let seam_edges: Vec<usize> =
            (0..self.cut.seam.len()).filter(|&h| self.cut.seam[h] && h < self.topology.opposite[h] as usize).collect();
        let mut stretch: Vec<usize> = (0..seam_edges.len()).collect();
        fn find(stretch: &mut [usize], mut x: usize) -> usize {
            while stretch[x] != x {
                stretch[x] = stretch[stretch[x]];
                x = stretch[x];
            }
            x
        }
        let mut by_copies: HashMap<(u32, u32), usize> = HashMap::new();
        let ends = |h: usize| {
            let t = self.topology.opposite[h] as usize;
            [(h, next_corner(t)), (next_corner(h), t)]
        };
        for (e, &h) in seam_edges.iter().enumerate() {
            for (in_f, in_g) in ends(h) {
                // Which side of the seam is called `f` depends on the edge,
                // so the pair is stored unordered.
                let (one, other) = (self.cut.wedge[in_f], self.cut.wedge[in_g]);
                let copies = (one.min(other), one.max(other));
                if let Some(&other) = by_copies.get(&copies) {
                    let (a, b) = (find(&mut stretch, e), find(&mut stretch, other));
                    stretch[a.max(b)] = a.min(b);
                } else {
                    by_copies.insert(copies, e);
                }
            }
        }
        let mut decided = vec![false; seam_edges.len()];
        for (e, &h) in seam_edges.iter().enumerate() {
            let tri = mesh.triangles[h / 3];
            if [tri[h % 3], tri[(h + 1) % 3]].iter().any(|&v| field.indices[v as usize] != 0) {
                decided[find(&mut stretch, e)] = true;
            }
        }
        for (e, &h) in seam_edges.iter().enumerate() {
            if find(&mut stretch, e) == e && !decided[e] {
                let (in_f, in_g) = ends(h)[0];
                expressions.extend(self.across(h, in_f, in_g));
            }
        }
        expressions
    }

    /// Feature edges (creases and open borders) must lie on grid lines, so
    /// that quad edges run along them. Returns the conditions that keep each
    /// one on a line (one coordinate constant along it) and the expressions
    /// (that coordinate) that must then be whole numbers.
    pub fn feature_conditions(&self) -> (Vec<Constraint>, Vec<Row>) {
        let (mut constraints, mut expressions) = (Vec::new(), Vec::new());
        for (f, k, along_u) in self.edges_on_lines() {
            let (here, there) = (3 * f + k, 3 * f + (k + 1) % 3);
            // Running along u, it is v that stays constant, and conversely.
            let constant = |corner: usize| if along_u { self.v(corner) } else { self.u(corner) };
            constraints.push(Constraint::zero(merged(vec![(constant(here), 1.0), (constant(there), -1.0)])));
            expressions.push(vec![(constant(here), 1.0)]);
        }
        (constraints, expressions)
    }

    /// The feature edges the field follows, each as seen from a face: the
    /// face, the corner the edge leaves from, and whether it runs along `u`
    /// (otherwise along `v`).
    fn edges_on_lines(&self) -> Vec<(usize, usize, bool)> {
        let mesh = &self.work.mesh;
        let on_feature: std::collections::HashSet<(u32, u32)> =
            self.work.feature_edges.iter().map(|&[a, b]| (a.min(b), a.max(b))).collect();
        let mut out = Vec::new();
        if on_feature.is_empty() {
            return out;
        }
        for (f, tri) in mesh.triangles.iter().enumerate() {
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                if !on_feature.contains(&(a.min(b), a.max(b))) {
                    continue;
                }
                let edge = (mesh.positions[b as usize] - mesh.positions[a as usize]).normalize_or_zero();
                let (along_u, along_v) = (edge.dot(self.arms[f][0]).abs(), edge.dot(self.arms[f][1]).abs());
                // Where the field does not follow the edge (a corner triangle
                // left free, a face too flat to have a direction), forcing it
                // onto a grid line would fold the map.
                if along_u.max(along_v) < 0.95 {
                    continue;
                }
                out.push((f, k, along_u > along_v));
            }
        }
        out
    }

    /// Minimises the energy under the constraints. Returns the unknowns and
    /// the worst mismatch left on a constraint.
    pub fn solve(&self, weights: &[f64], constraints: &[Constraint], solver: &Solver) -> Result<(Vec<f64>, f64)> {
        let mut matrix = SymmetricBuilder::new(self.unknowns);
        let mut rhs = vec![0.0; self.unknowns];
        let mut diagonal_sum = 0.0;
        for (f, corners) in self.ideal.iter().enumerate() {
            let ideal_area = signed_area(corners);
            if self.frames.area[f] <= 0.0 || ideal_area <= 0.0 {
                continue;
            }
            // Gradients of the three hat functions, in the plane of the cross.
            let gradient: [[f64; 2]; 3] = std::array::from_fn(|i| {
                let (a, b) = (corners[(i + 1) % 3], corners[(i + 2) % 3]);
                [-(b[1] - a[1]) / (2.0 * ideal_area), (b[0] - a[0]) / (2.0 * ideal_area)]
            });
            let weight = weights[f] * self.frames.area[f];
            for i in 0..3 {
                let wi = self.u(3 * f + i);
                // The target gradients are (1, 0) for u and (0, 1) for v.
                rhs[wi] += weight * gradient[i][0];
                rhs[wi + 1] += weight * gradient[i][1];
                for j in 0..3 {
                    let wj = self.u(3 * f + j);
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
        let penalty = solver.penalty * (diagonal_sum / self.unknowns.max(1) as f64).max(f64::MIN_POSITIVE);
        for constraint in constraints {
            matrix.add_squared(&constraint.row, penalty);
        }
        for i in 0..self.unknowns {
            matrix.add(i, i, 1e-12 * penalty);
        }
        let factorization = matrix.factorize()?;

        let mut multipliers = vec![0.0; constraints.len()];
        let mut worst = f64::INFINITY;
        // Consistent conditions are met at a steady geometric rate, slow
        // when some faces are very stiff; contradictory ones stall. So the
        // loop runs as long as it makes progress.
        let (mut reference, mut since_progress) = (f64::INFINITY, 0);
        for iteration in 0..solver.max_iterations {
            let mut b = rhs.clone();
            for (constraint, &multiplier) in constraints.iter().zip(&multipliers) {
                for &(i, c) in &constraint.row {
                    b[i] += (penalty * constraint.target - multiplier) * c;
                }
            }
            let x = factorization.solve(&b);
            worst = 0.0;
            for (constraint, multiplier) in constraints.iter().zip(&mut multipliers) {
                let violation = evaluate(&constraint.row, &x) - constraint.target;
                worst = worst.max(violation.abs());
                *multiplier += penalty * violation;
            }
            if worst <= solver.tolerance {
                return Ok((x, worst));
            }
            if worst < 0.5 * reference {
                (reference, since_progress) = (worst, 0);
            } else {
                since_progress += 1;
            }
            if since_progress >= solver.patience {
                return Err(Error::Numerical(format!(
                    "conditions cannot all be met (mismatch stuck at {worst:.3e} after {} iterations)",
                    iteration + 1
                )));
            }
        }
        Err(Error::Numerical(format!(
            "conditions not met after {} iterations (mismatch {worst:.3e})",
            solver.max_iterations
        )))
    }

    /// The coordinates of every corner, from the unknowns.
    pub fn uv(&self, x: &[f64]) -> Vec<[f64; 2]> {
        (0..self.cut.wedge.len()).map(|c| [x[self.u(c)], x[self.v(c)]]).collect()
    }

    /// Makes the faces that folded over, and the ones around them, stiffer.
    pub fn stiffen(&self, weights: &mut [f64], flipped: &[usize]) {
        for &f in flipped {
            weights[f] *= 4.0;
            for h in 3 * f..3 * f + 3 {
                let g = self.frames.across[h];
                if g != INVALID {
                    weights[g as usize] *= 2.0;
                }
            }
        }
    }

    pub fn quality(&self, uv: &[[f64; 2]]) -> Quality {
        Quality::measure(uv, &self.ideal, &self.frames.area)
    }
}

pub(super) fn evaluate(row: &Row, x: &[f64]) -> f64 {
    row.iter().map(|&(i, c)| c * x[i]).sum()
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
pub(super) struct Quality {
    /// Faces mapped with the wrong orientation, or squashed flat.
    pub flipped: Vec<usize>,
    /// Area-weighted mean of `|J - I|`, `J` being the map from the ideal
    /// triangle to the actual one.
    pub distortion_mean: f64,
    /// Largest ratio between the two stretches of a face.
    pub stretch_max: f64,
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
