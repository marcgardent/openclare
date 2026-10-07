//! Seamless parametrization: `(u, v)` follows the cross field and agrees
//! across every seam up to a quarter-turn rotation and a shift. No whole
//! number is imposed, so its grid lines do not close up into quads yet; it
//! is the continuous problem the integer one starts from.

use super::problem::{Constraint, Problem, Solver};
use super::{Kind, Parametrization, Parametrizer, Report, Request};
use crate::crossfield::CrossField;
use crate::error::Result;
use crate::preprocess::WorkMesh;

#[derive(Clone, Debug, Default)]
pub struct Seamless {
    pub(super) solver: Solver,
}

impl Parametrizer for Seamless {
    fn kind(&self) -> Kind {
        Kind::Seamless
    }

    fn parametrize(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Parametrization> {
        let problem = Problem::new(work, field, request.edge_length)?;
        let constraints = problem.seam_constraints();
        let mut weights = vec![1.0; work.mesh.triangles.len()];
        let mut report =
            Report { seam_edges: problem.cut.seam_edges(), wedges: problem.cut.wedge_count, ..Default::default() };
        let uv = untangle(&problem, &constraints, &mut weights, &self.solver, request.stiffening_rounds, &mut report)?;
        report.flipped_first = report.flipped_before_stiffening;
        Ok(Parametrization { cut: problem.cut, uv, report })
    }
}

/// Beyond this weight a face is as stiff as it usefully gets.
const MAX_STIFFNESS: f64 = 1.0e6;

/// Solves, then stiffens the faces that folded over and solves again, until
/// none is left or the rounds run out. Fills in the quality part of `report`.
pub(super) fn untangle(
    problem: &Problem,
    constraints: &[Constraint],
    weights: &mut [f64],
    solver: &Solver,
    rounds: usize,
    report: &mut Report,
) -> Result<Vec<[f64; 2]>> {
    // Stiffening can make things worse where a fold has no cure (two
    // singularities rounded onto the same point, say): keep the best map
    // seen, and give up once the weights have grown past any use.
    let mut best: Option<(usize, Vec<[f64; 2]>, Report)> = None;
    for round in 0..=rounds {
        let (x, mismatch) = problem.solve(weights, constraints, solver)?;
        let uv = problem.uv(&x);
        let quality = problem.quality(&uv);
        if round == 0 {
            report.flipped_before_stiffening = quality.flipped.len();
        }
        report.flipped = quality.flipped.len();
        report.distortion_mean = quality.distortion_mean;
        report.stretch_max = quality.stretch_max;
        report.seam_error = mismatch;
        report.rounds = round;
        if best.as_ref().is_none_or(|(fewest, ..)| quality.flipped.len() < *fewest) {
            best = Some((quality.flipped.len(), uv, report.clone()));
        }
        let heaviest = quality.flipped.iter().map(|&f| weights[f]).fold(0.0, f64::max);
        if quality.flipped.is_empty() || heaviest >= MAX_STIFFNESS {
            break;
        }
        problem.stiffen(weights, &quality.flipped);
    }
    let (_, uv, kept) = best.expect("at least one round ran");
    *report = kept;
    Ok(uv)
}
