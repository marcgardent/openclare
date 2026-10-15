//! Seamless parametrization: `(u, v)` follows the cross field and agrees
//! across every seam up to a quarter-turn rotation and a shift. No whole
//! number is imposed, so its grid lines do not close up into quads yet; it
//! is the continuous problem the integer one starts from.

use std::collections::HashSet;

use super::problem::{Constraint, Problem, Solver};
use super::{Kind, Parametrization, Parametrizer, Report, Request};
use crate::crossfield::CrossField;
use crate::error::Result;
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
        let uv = untangle(&problem, &constraints, &mut weights, &self.solver, request.stiffening_rounds, &mut report)?;
        report.flipped_first = report.flipped_before_stiffening;
        let uv = self.closing(&problem, &mut constraints, &mut weights, request, &mut report, uv)?;
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
        let uv = untangle(&problem, &constraints, &mut weights, &self.solver, request.stiffening_rounds, &mut report)?;
        report.flipped_first = report.flipped_before_stiffening;
        let uv = self.closing(&problem, &mut constraints, &mut weights, request, &mut report, uv)?;
        if report.flipped > 0 {
            return Ok(Err(problem.quality(&uv).flipped));
        }
        Ok(Ok(Parametrization { set_aside: problem.set_aside(), cut: problem.cut, uv, report }))
    }
}

/// Lines started from about this many points are followed to find the
/// turns of the map; one that comes back within the gate, in quad edges,
/// across its way, has made a turn, and shifted if by more than the least.
const TUBE_SAMPLES: usize = 600;
const TUBE_GATE: f64 = 3.0;
const TUBE_SHIFT: f64 = 0.05;
/// A turn whose shift is more than this share of its way round leans too
/// much to be a part the lines go round (some 3 degrees): it goes round
/// singularities, whose places decide its shift, and holding it fights them.
const TUBE_LEAN: f64 = 0.05;
/// Turns held at a time, the least leaning first, and rounds of finding
/// them: a map held on some may shift on others.
const TUBES_AT_ONCE: usize = 16;
const TUBE_ROUNDS: usize = 4;
/// Seconds the closing may take, at most, beyond the first solve.
const TUBE_SECONDS: f64 = 20.0;

impl Seamless {
    /// The map solved again with the turns it makes held closed, if the
    /// request asks for it: where a grid line goes round a part and comes
    /// back beside itself, its shift once round is held at nothing, and the
    /// lines close into rings (DT-062). Turns are held a few at a time; a
    /// batch that folds the map is given up, and so is what is left once
    /// the time is out.
    fn closing(
        &self,
        problem: &Problem,
        constraints: &mut Vec<Constraint>,
        weights: &mut [f64],
        request: &Request,
        report: &mut Report,
        mut uv: Vec<[f64; 2]>,
    ) -> Result<Vec<[f64; 2]>> {
        if !request.close_tubes {
            return Ok(uv);
        }
        let started = std::time::Instant::now();
        let set_aside = problem.set_aside();
        let faces = problem.work.mesh.triangles.len();
        let area: f64 = (0..faces)
            .map(|f| {
                let [a, b, c] = [uv[3 * f], uv[3 * f + 1], uv[3 * f + 2]];
                0.5 * ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs()
            })
            .sum();
        let longest = 4.0 * area.sqrt();
        let debug = std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some();
        let gate = std::env::var("OPENCLARE_TUBE_GATE").ok().and_then(|g| g.parse().ok()).unwrap_or(TUBE_GATE);
        let samples = std::env::var("OPENCLARE_TUBE_SAMPLES").ok().and_then(|g| g.parse().ok()).unwrap_or(TUBE_SAMPLES);
        let lean = std::env::var("OPENCLARE_TUBE_LEAN").ok().and_then(|g| g.parse().ok()).unwrap_or(TUBE_LEAN);
        // Lines that went round the same part came back shifted by the same
        // amount: one condition holds them all. Those tried and given up are
        // not tried again.
        let mut tried: HashSet<(u8, i64)> = HashSet::new();
        for round in 0..TUBE_ROUNDS {
            if started.elapsed().as_secs_f64() > TUBE_SECONDS {
                break;
            }
            let clock = std::time::Instant::now();
            let found = crate::tmesh::tubes::tubes_of(
                &problem.work.mesh, &uv, &problem.cut, &set_aside, samples, longest, gate,
            );
            let traced = clock.elapsed().as_secs_f64();
            let mut candidates: Vec<&crate::tmesh::tubes::Tube> = found
                .iter()
                .filter(|tube| {
                    tube.shift.abs() > TUBE_SHIFT && tube.shift.abs() <= lean * tube.around && !tube.crossings.is_empty()
                })
                .collect();
            candidates.sort_by(|a, b| (a.shift.abs() / a.around).total_cmp(&(b.shift.abs() / b.around)));
            let mut batch = Vec::new();
            for tube in candidates {
                let same = (tube.axis % 2, (tube.shift.abs() * 1e4).round() as i64);
                if !tried.insert(same) {
                    continue;
                }
                report.tube_shift = report.tube_shift.max(tube.shift.abs());
                let [u_shift, v_shift] = problem.period(&tube.crossings);
                batch.push(Constraint::zero(if tube.axis % 2 == 0 { v_shift } else { u_shift }));
                if batch.len() == TUBES_AT_ONCE {
                    break;
                }
            }
            if batch.is_empty() {
                break;
            }
            let (folds_before, kept_report, kept_weights) = (report.flipped, report.clone(), weights.to_vec());
            let added = batch.len();
            constraints.extend(batch);
            let clock = std::time::Instant::now();
            let held = untangle(problem, constraints, weights, &self.solver, request.stiffening_rounds, report)?;
            if debug {
                eprintln!(
                    "tubes round {round}: {} turns, {added} held; traced in {traced:.1} s, solved in {:.1} s, \
                     {} folds (were {folds_before})",
                    found.len(),
                    clock.elapsed().as_secs_f64(),
                    report.flipped,
                );
            }
            if report.flipped > folds_before {
                // That batch folds the map: given up, and the map as it was.
                constraints.truncate(constraints.len() - added);
                *report = kept_report;
                weights.copy_from_slice(&kept_weights);
                continue;
            }
            report.tubes_closed += added;
            uv = held;
        }
        Ok(uv)
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
