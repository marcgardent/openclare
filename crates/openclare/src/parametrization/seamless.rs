//! Seamless parametrization: `(u, v)` follows the cross field and agrees
//! across every seam up to a quarter-turn rotation and a shift. No whole
//! number is imposed, so its grid lines do not close up into quads yet; it
//! is the continuous problem the integer one starts from.

use std::collections::HashSet;

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
        // Feature edges are kept on grid lines (which one is left free), so
        // that borders, creases and guide curves bound the rectangles the
        // map is cut into.
        if let Some(map) = self.solve(work, field, request, true)? {
            return Ok(map);
        }
        // Where that folds the map, guide curves are let go first: a line
        // drawn that cannot be kept should not cost the creases theirs, nor
        // the other curves. Each curve is tried alone, then those that can
        // be kept together, then none.
        if !work.guide_edges.is_empty() {
            let curves = lines_of(&work.guide_edges);
            let without = |dropped: &[&Vec<[u32; 2]>]| {
                let dropped: HashSet<[u32; 2]> = dropped.iter().flat_map(|curve| curve.iter().copied()).collect();
                let mut lighter = work.clone();
                lighter.feature_edges.retain(|edge| !dropped.contains(edge));
                lighter.guide_edges.retain(|edge| !dropped.contains(edge));
                (lighter, dropped.into_iter().collect::<Vec<_>>())
            };
            let mut fits = vec![false; curves.len()];
            if curves.len() > 1 {
                for index in 0..curves.len() {
                    let others: Vec<&Vec<[u32; 2]>> =
                        (0..curves.len()).filter(|&k| k != index).map(|k| &curves[k]).collect();
                    fits[index] = self.solve(&without(&others).0, field, request, true)?.is_some();
                }
            }
            for keep_those_that_fit in [true, false] {
                if keep_those_that_fit && !fits.contains(&true) {
                    continue;
                }
                let dropped: Vec<&Vec<[u32; 2]>> =
                    (0..curves.len()).filter(|&k| !(keep_those_that_fit && fits[k])).map(|k| &curves[k]).collect();
                let (lighter, mut loose) = without(&dropped);
                if let Some(mut map) = self.solve(&lighter, field, request, true)? {
                    loose.sort_unstable();
                    map.report.guides_off_grid = true;
                    map.report.loose_guides = loose;
                    return Ok(map);
                }
            }
        }
        // A map that only follows the lines through the field is better
        // than none.
        let mut map = self.solve(work, field, request, false)?.expect("without lines a map is always given");
        map.report.features_off_grid = true;
        map.report.guides_off_grid = !work.guide_edges.is_empty();
        map.report.loose_guides = work.guide_edges.clone();
        Ok(map)
    }
}

impl Seamless {
    /// The map of `work`: with its feature edges on grid lines if `on_lines`,
    /// and then only if that does not fold it; held by its seams alone
    /// otherwise, folds or not.
    fn solve(
        &self,
        work: &WorkMesh,
        field: &CrossField,
        request: &Request,
        on_lines: bool,
    ) -> Result<Option<Parametrization>> {
        let problem = Problem::new(work, field, request.edge_length)?;
        let mut constraints = problem.seam_constraints();
        if on_lines {
            constraints.extend(problem.feature_conditions().0);
        }
        let mut weights = vec![1.0; work.mesh.triangles.len()];
        let mut report =
            Report { seam_edges: problem.cut.seam_edges(), wedges: problem.cut.wedge_count, ..Default::default() };
        let uv = untangle(&problem, &constraints, &mut weights, &self.solver, request.stiffening_rounds, &mut report)?;
        report.flipped_first = report.flipped_before_stiffening;
        if on_lines && report.flipped > 0 {
            return Ok(None);
        }
        Ok(Some(Parametrization { cut: problem.cut, uv, report }))
    }
}

/// The edges of guide curves sorted into curves: those that hang together
/// through a vertex are one.
fn lines_of(edges: &[[u32; 2]]) -> Vec<Vec<[u32; 2]>> {
    let mut group_of: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    let mut groups: Vec<Vec<[u32; 2]>> = Vec::new();
    for &edge in edges {
        let found: Vec<usize> = edge.iter().filter_map(|v| group_of.get(v).copied()).collect();
        let into = match found[..] {
            [] => {
                groups.push(Vec::new());
                groups.len() - 1
            }
            [one] => one,
            [one, other] => {
                if one != other {
                    let moved = std::mem::take(&mut groups[other]);
                    for vertex in moved.iter().flatten() {
                        group_of.insert(*vertex, one);
                    }
                    groups[one].extend(moved);
                }
                one
            }
            _ => unreachable!("an edge has two ends"),
        };
        groups[into].push(edge);
        for vertex in edge {
            group_of.insert(vertex, into);
        }
    }
    groups.retain(|group| !group.is_empty());
    groups
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
