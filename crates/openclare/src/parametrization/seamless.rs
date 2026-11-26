//! Seamless parametrization: `(u, v)` follows the cross field and agrees
//! across every seam up to a quarter-turn rotation and a shift. No whole
//! number is imposed, so its grid lines do not close up into quads yet; it
//! is the continuous problem the integer one starts from.

use std::collections::HashSet;

use super::problem::{Constraint, Problem, Solver};
use super::{Kind, Parametrization, Parametrizer, Report, Request};
use crate::crossfield::CrossField;
use crate::error::{Error, Result};
use crate::mesh::Topology;
use crate::preprocess::WorkMesh;

#[derive(Clone, Debug, Default)]
pub struct Seamless {
    pub(super) solver: Solver,
}

/// With more guide lines than this on a piece, they are kept all together
/// or not at all: trying each alone costs a solve of the map apiece, which
/// the borders between the materials of a model can ask for by the hundred.
const SINGLE_TRIALS: usize = 8;
/// How many times guide lines are let go near the folds of the map before
/// whole lines are, and how many edges away from a fold a line is let go
/// the first time.
const NEAR_ROUNDS: usize = 5;
const NEAR_REACH: usize = 2;

impl Parametrizer for Seamless {
    fn kind(&self) -> Kind {
        Kind::Seamless
    }

    fn parametrize(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Parametrization> {
        let mut map = self.with_lines(work, field, request)?;
        // A guide line asked to lie on a grid line does not always end up
        // on one (where the field crosses it, the condition is left out):
        // such an edge is let go like the others, rather than have the
        // layout refuse the whole map for it.
        let gone: HashSet<[u32; 2]> = map.report.loose_guides.iter().copied().collect();
        let kept: Vec<[u32; 2]> = work.guide_edges.iter().copied().filter(|edge| !gone.contains(edge)).collect();
        let astray = map.off_grid(&work.mesh, &kept);
        if !astray.is_empty() {
            map.report.guides_off_grid = true;
            map.report.loose_guides.extend(astray);
            map.report.loose_guides.sort_unstable();
        }
        Ok(map)
    }
}

impl Seamless {
    /// The map of `work`, with as many of its lines on grid lines as it can
    /// keep without folding.
    fn with_lines(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Parametrization> {
        // Feature edges are kept on grid lines (which one is left free), so
        // that borders, creases and guide curves bound the rectangles the
        // map is cut into.
        let folds = match self.solve_or_folds(work, field, request)? {
            Ok(map) => return Ok(map),
            Err(folds) => folds,
        };
        let mut map = self.with_fewer_lines(work, field, request, folds.clone())?;
        map.report.folds_with_lines = folds;
        Ok(map)
    }
}

impl Seamless {
    /// The map of `work` when it folds with every line on a grid line, at
    /// the triangles `folds`: with what lines it can keep there.
    fn with_fewer_lines(
        &self,
        work: &WorkMesh,
        field: &CrossField,
        request: &Request,
        mut folds: Vec<usize>,
    ) -> Result<Parametrization> {
        // Where that folds the map, guide lines are let go first: a line
        // drawn that cannot be kept should not cost the creases theirs, nor
        // the other lines. And of a line, only what is near a fold: the
        // rest of it stays, as a line that stops there.
        if !work.guide_edges.is_empty() {
            let neighbors = Topology::build(&work.mesh).neighbors;
            let mut lighter = work.clone();
            let mut loose: Vec<[u32; 2]> = Vec::new();
            for round in 0..NEAR_ROUNDS {
                // The vertices within reach of a fold, a reach that doubles
                // each time the folds do not go away.
                let mut near: HashSet<u32> = folds.iter().flat_map(|&f| work.mesh.triangles[f]).collect();
                for _ in 0..(NEAR_REACH << round) {
                    let ring: Vec<u32> = near.iter().flat_map(|&v| neighbors[v as usize].iter().copied()).collect();
                    near.extend(ring);
                }
                let dropped: HashSet<[u32; 2]> =
                    lighter.guide_edges.iter().copied().filter(|edge| edge.iter().any(|v| near.contains(v))).collect();
                if dropped.is_empty() {
                    // The folds are not the doing of the lines left.
                    break;
                }
                lighter.feature_edges.retain(|edge| !dropped.contains(edge));
                lighter.guide_edges.retain(|edge| !dropped.contains(edge));
                loose.extend(dropped);
                match self.solve_or_folds(&lighter, field, request)? {
                    Ok(mut map) => {
                        loose.sort_unstable();
                        map.report.guides_off_grid = true;
                        map.report.loose_guides = loose;
                        return Ok(map);
                    }
                    Err(still) => folds = still,
                }
            }
        }
        // Failing that, whole lines: each is tried alone, then those that
        // can be kept together, then none.
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
            if curves.len() > 1 && curves.len() <= SINGLE_TRIALS {
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
        let uv = untangle(
            &problem,
            &constraints,
            &mut weights,
            &self.solver,
            request.stiffening_rounds,
            request.distortion_passes,
            &mut report,
        )?;
        report.flipped_first = report.flipped_before_stiffening;
        if on_lines && report.flipped > 0 {
            return Ok(None);
        }
        Ok(Some(Parametrization { set_aside: problem.set_aside(), cut: problem.cut, uv, report }))
    }

    /// The map of `work` with its feature edges on grid lines, or the
    /// triangles that fold over in the best map found with them there.
    fn solve_or_folds(
        &self,
        work: &WorkMesh,
        field: &CrossField,
        request: &Request,
    ) -> Result<std::result::Result<Parametrization, Vec<usize>>> {
        let problem = Problem::new(work, field, request.edge_length)?;
        let mut constraints = problem.seam_constraints();
        constraints.extend(problem.feature_conditions().0);
        let mut weights = vec![1.0; work.mesh.triangles.len()];
        let mut report =
            Report { seam_edges: problem.cut.seam_edges(), wedges: problem.cut.wedge_count, ..Default::default() };
        let uv = untangle(
            &problem,
            &constraints,
            &mut weights,
            &self.solver,
            request.stiffening_rounds,
            request.distortion_passes,
            &mut report,
        )?;
        report.flipped_first = report.flipped_before_stiffening;
        if report.flipped > 0 {
            return Ok(Err(problem.quality(&uv).flipped));
        }
        Ok(Ok(Parametrization { set_aside: problem.set_aside(), cut: problem.cut, uv, report }))
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
pub(super) const MAX_STIFFNESS: f64 = 1.0e6;

/// Solves, then stiffens the faces that folded over and solves again, until
/// none is left or the `rounds` run out. Then, up to `passes` times, stiffens
/// the faces of the valid map that are stretched or skewed past what is
/// tolerated (see [`Problem::distortion`]) and solves again, folds that come
/// back being fought as before. Fills in the quality part of `report`.
///
/// With `passes` at 0 this is stiffening against folds alone, as it was.
pub(super) fn untangle(
    problem: &Problem,
    constraints: &[Constraint],
    weights: &mut [f64],
    solver: &Solver,
    rounds: usize,
    passes: usize,
    report: &mut Report,
) -> Result<Vec<[f64; 2]>> {
    // Stiffening can make things worse where a fold has no cure (two
    // singularities rounded onto the same point, say): keep the best map
    // seen, and give up once the weights have grown past any use. The best
    // has the fewest folds and, against distortion, the least of it.
    struct Best {
        flipped: usize,
        excess: f64,
        uv: Vec<[f64; 2]>,
        report: Report,
        weights: Option<Vec<f64>>,
    }
    let total_area: f64 = problem.frames.area.iter().sum();
    let mut best: Option<Best> = None;
    let (mut round, mut fold_rounds, mut passes_done) = (0, 0, 0);
    let mut folded = vec![false; problem.frames.area.len()];
    loop {
        let (x, mismatch) = match problem.solve(weights, constraints, solver) {
            Ok(solved) => solved,
            // Stiffer faces make the system worse conditioned: where a pass
            // against distortion breaks the solve, the map before it stands.
            Err(Error::Numerical(_)) if passes_done > 0 => break,
            Err(other) => return Err(other),
        };
        let uv = problem.uv(&x);
        let quality = problem.quality(&uv);
        let distortion = if passes > 0 { problem.distortion(&uv) } else { Vec::new() };
        let excess = distortion.iter().zip(&problem.frames.area).map(|(d, a)| d * a).sum::<f64>()
            / total_area.max(f64::MIN_POSITIVE);
        if round == 0 {
            report.flipped_before_stiffening = quality.flipped.len();
        }
        report.flipped = quality.flipped.len();
        report.distortion_mean = quality.distortion_mean;
        report.stretch_max = quality.stretch_max;
        report.seam_error = mismatch;
        report.rounds = round;
        report.distortion_passes = passes_done;
        let better = best.as_ref().is_none_or(|kept| {
            quality.flipped.len() < kept.flipped
                || (passes > 0 && quality.flipped.len() == kept.flipped && excess < kept.excess)
        });
        if better {
            let kept_weights = (passes > 0).then(|| weights.to_vec());
            best = Some(Best {
                flipped: quality.flipped.len(),
                excess,
                uv,
                report: report.clone(),
                weights: kept_weights,
            });
        }
        for &f in &quality.flipped {
            folded[f] = true;
        }
        if !quality.flipped.is_empty() {
            let heaviest = quality.flipped.iter().map(|&f| weights[f]).fold(0.0, f64::max);
            if heaviest >= MAX_STIFFNESS {
                break;
            }
            problem.stiffen(weights, &quality.flipped);
            fold_rounds += 1;
            if fold_rounds > rounds {
                break;
            }
        } else if passes_done < passes && distortion.iter().any(|&d| d > 0.0) {
            problem.stiffen_distorted(weights, &distortion);
            passes_done += 1;
        } else {
            break;
        }
        round += 1;
    }
    let kept = best.expect("the first solve either ran or returned its error");
    // The weights go on to the solves that follow (the rounding of the
    // integer map): those of the map kept, not of a pass that did worse.
    if passes_done > 0
        && let Some(kept_weights) = kept.weights
    {
        weights.copy_from_slice(&kept_weights);
    }
    *report = kept.report;
    near_folds(problem, &kept.uv, &folded, report);
    Ok(kept.uv)
}

/// Fills in where the stretch and skew of the map `uv` lie from the faces
/// that `folded` at some round: what tells a leftover of the stiffening
/// against folds from a stretch the field asked for.
fn near_folds(problem: &Problem, uv: &[[f64; 2]], folded: &[bool], report: &mut Report) {
    let away = problem.faces_away(folded);
    let distortion = problem.distortion(uv);
    report.folded_faces = (0..folded.len()).filter(|&f| folded[f]).collect();
    report.stretch_max_face = problem.quality(uv).stretch_max_face;
    report.stretch_max_from_fold = report.stretch_max_face.and_then(|f| away[f]);
    report.distorted_faces = distortion.iter().filter(|&&d| d > 0.0).count();
    let excess = |f: usize| distortion[f] * problem.frames.area[f];
    let total: f64 = (0..distortion.len()).map(excess).sum();
    for (k, &reach) in super::NEAR_FOLDS.iter().enumerate() {
        let near = |f: &usize| distortion[*f] > 0.0 && away[*f].is_some_and(|d| d <= reach);
        report.distorted_near_folds[k] = (0..distortion.len()).filter(near).count();
        report.excess_near_folds[k] =
            (0..distortion.len()).filter(near).map(excess).sum::<f64>() / total.max(f64::MIN_POSITIVE) + 0.0;
    }
}
