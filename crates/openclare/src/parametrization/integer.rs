//! Integer-grid parametrization by greedy rounding.
//!
//! For the grid lines of `(u, v)` to close up into quads, some quantities
//! must be whole numbers: the coordinates of the singularities, the shifts
//! across the seams, the grid line each feature edge lies on. Finding the
//! best whole numbers is a hard combinatorial problem; this takes the cheap
//! way of Bommes et al., "Mixed-Integer Quadrangulation" (2009): solve
//! without them, round the quantities that are closest to a whole number,
//! solve again with those held, and repeat until none is left.
//!
//! Each rounding is locally the least harmful, but nothing looks ahead: the
//! result is valid when it works, not optimal, and it can fail. It is the
//! shortcut to a first complete mesh, to be replaced by quantization.
//!
//! One consequence of not looking ahead: the quantities are tied together by
//! parity (a singularity sits at the fixed point of the quarter turn around
//! it, which halves a shift), so the last ones can come out forced onto half
//! numbers. To stay clear of that, the problem is solved on a grid twice as
//! coarse, where halves are allowed, and the map is doubled at the end: the
//! free choices then land on even lines, the forced ones on any line.

use super::problem::{Constraint, Problem, Row, Solver, evaluate};
use super::seamless::untangle;
use super::{Kind, Parametrization, Parametrizer, Report, Request};
use crate::crossfield::CrossField;
use crate::error::{Error, Result};
use crate::preprocess::WorkMesh;

#[derive(Clone, Debug)]
pub struct IntegerGrid {
    pub(super) solver: Solver,
    /// Share of the quantities still free that is rounded in one go. More
    /// means fewer solves and worse choices.
    pub batch: f64,
    /// A quantity farther than this from a whole number is never rounded in
    /// a batch, only alone when it is the closest one left.
    pub batch_reach: f64,
}

impl Default for IntegerGrid {
    fn default() -> Self {
        Self { solver: Solver::default(), batch: 0.25, batch_reach: 0.2 }
    }
}

/// Distance below which a quantity is taken to be a whole number already,
/// because the ones fixed so far imply it.
const SETTLED: f64 = 1e-6;

impl Parametrizer for IntegerGrid {
    fn kind(&self) -> Kind {
        Kind::IntegerGrid
    }

    fn parametrize(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Parametrization> {
        // Feature edges on grid lines are worth having, but rounding
        // sometimes cannot reconcile them with everything else. A map that
        // only follows them through the field beats no map at all: the final
        // vertices are still snapped onto them.
        match self.round(work, field, request, true) {
            Err(Error::Numerical(_)) if !work.feature_edges.is_empty() => {
                let mut map = self.round(work, field, request, false)?;
                map.report.features_off_grid = true;
                Ok(map)
            }
            outcome => outcome,
        }
    }
}

impl IntegerGrid {
    fn round(&self, work: &WorkMesh, field: &CrossField, request: &Request, features: bool) -> Result<Parametrization> {
        // Twice as coarse: see the module documentation.
        let problem = Problem::new(work, field, 2.0 * request.edge_length)?;
        let (on_lines, mut expressions) = if features { problem.feature_conditions() } else { Default::default() };
        let mut constraints = problem.seam_constraints();
        constraints.extend(on_lines);
        expressions.extend(problem.integer_expressions(field));
        // Both faces of a crease, and both ends of an edge, often name the
        // same quantity.
        expressions.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        expressions.dedup();

        let mut weights = vec![1.0; work.mesh.triangles.len()];
        let mut report =
            Report { seam_edges: problem.cut.seam_edges(), wedges: problem.cut.wedge_count, ..Default::default() };

        // Start from the continuous solution, already rid of its folds.
        untangle(&problem, &constraints, &mut weights, &self.solver, request.stiffening_rounds, &mut report)?;
        report.flipped_first = report.flipped_before_stiffening;
        let (mut x, _) = problem.solve(&weights, &constraints, &self.solver)?;

        let mut free: Vec<Row> = expressions;
        while !free.is_empty() {
            let mut ranked: Vec<(f64, usize)> = free
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    let value = evaluate(row, &x);
                    ((value - value.round()).abs(), i)
                })
                .collect();
            ranked.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

            let settled = ranked.iter().take_while(|(distance, _)| *distance <= SETTLED).count();
            let wanted = ((self.batch * (free.len() - settled) as f64).ceil() as usize).max(1);
            let in_reach = ranked[settled..].iter().take_while(|(distance, _)| *distance <= self.batch_reach).count();
            let mut take = settled + wanted.min(in_reach.max(1)).min(free.len() - settled);

            // Quantities rounded together may contradict one another; if so,
            // fall back to rounding the closest one alone.
            let rounded = loop {
                let batch: Vec<Constraint> = ranked[..take]
                    .iter()
                    .map(|&(_, i)| Constraint { row: free[i].clone(), target: evaluate(&free[i], &x).round() })
                    .collect();
                let mut trial = constraints.clone();
                trial.extend(batch.iter().cloned());
                match problem.solve(&weights, &trial, &self.solver) {
                    Ok((solution, _)) => break Some((solution, batch)),
                    Err(Error::Numerical(_)) if take > settled + 1 => take = settled + (take - settled) / 2,
                    Err(Error::Numerical(_)) if take > 1 => take = 1,
                    Err(Error::Numerical(_)) => break None,
                    Err(other) => return Err(other),
                }
            };
            let Some((solution, batch)) = rounded else {
                // What is left cannot be rounded: fine if the choices made so
                // far pin every one of them on a half.
                let on_halves = ranked.iter().all(|(distance, _)| (distance - 0.5).abs() <= SETTLED);
                if on_halves {
                    report.integers += free.len();
                    break;
                }
                return Err(Error::Numerical(format!(
                    "no consistent whole number for one of the {} quantities left",
                    free.len()
                )));
            };
            x = solution;
            constraints.extend(batch);
            report.integers += take;

            let mut taken: Vec<usize> = ranked[..take].iter().map(|&(_, i)| i).collect();
            taken.sort_unstable_by(|a, b| b.cmp(a));
            for i in taken {
                free.swap_remove(i);
            }
        }

        // Whole numbers bend the map: folds may be back.
        let mut uv =
            untangle(&problem, &constraints, &mut weights, &self.solver, request.stiffening_rounds, &mut report)?;
        for point in &mut uv {
            *point = [2.0 * point[0], 2.0 * point[1]];
        }
        report.seam_error *= 2.0;
        Ok(Parametrization { cut: problem.cut, uv, report })
    }
}
