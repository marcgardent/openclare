//! Quantization (stage M6): a whole length for every arc of the T-mesh.
//!
//! The quad mesh to come has a whole number of edges along every arc. Those
//! numbers must agree on the opposite sides of each rectangle, stay close to
//! the real lengths, add up to the number of quads asked for, and leave the
//! structure intact: an arc may shrink to nothing, which closes up a sliver
//! between two cuts, but not so that two singularities land on each other
//! or a tube is pinched shut.
//!
//! The stage is a [`Quantizer`]; chains get one from [`create`].

mod experiment;
mod export;
mod flow;
mod lattice;
mod rings;
mod tubes;

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::math::Rng;
use crate::tmesh::TMesh;
pub use experiment::{Experiment, slow_log};
use flow::{Cost, Network};
pub use lattice::{Lattice, Source};
use rayon::prelude::*;
pub use rings::{NoRingSelector, Ring, RingProblem, RingSelector, Selector, Slow};

#[derive(Clone, Debug)]
pub struct Request {
    /// Number of quads to aim for. `None` keeps the scale of the T-mesh,
    /// whose lengths are already counted in quad edges.
    pub quads: Option<usize>,
    /// How far from that number is close enough, as a fraction of it.
    pub tolerance: f64,
    /// Lengths that close the ways round a part into rings are looked for,
    /// and kept where they make fewer spirals (DT-062).
    pub close_turns: bool,
    /// The slow quality mode: with loops closed, rounds of rings chosen by
    /// a solver from outside the core, their lengths added to those the
    /// engine chooses among. `None` for the engine alone.
    pub slow: Option<Slow>,
    /// Options for the experiments of the research papers: stages of the
    /// closing turned off, its system written out, lengths read in. None by
    /// default.
    pub experiment: Experiment,
}

impl Default for Request {
    fn default() -> Self {
        Self { quads: None, tolerance: 0.02, close_turns: true, slow: None, experiment: Experiment::default() }
    }
}

/// What a quantizer achieved.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    pub arcs: usize,
    /// Quads the lengths add up to, and how far that is from the number
    /// asked for, as a fraction of it.
    pub quads: usize,
    pub budget_error: Option<f64>,
    /// Factor applied to the real lengths to get there.
    pub scale: f64,
    /// Arcs given no length, and rectangles left with no area.
    pub zero_arcs: usize,
    pub collapsed_patches: usize,
    /// Arcs that had to be kept from shrinking to nothing for the structure
    /// to stay intact.
    pub held_arcs: usize,
    /// Mean and largest distance between a whole length and the scaled real
    /// one, in quad edges.
    pub deviation_mean: f64,
    pub deviation_max: f64,
    /// The opposite sides of every rectangle add up to the same.
    pub balanced: bool,
    /// The quad mesh the lengths stand for is a manifold with the topology
    /// of the surface, and its irregular vertices are the singularities of
    /// the T-mesh: no two of them merged, nothing pinched.
    pub separated: bool,
    /// Improving moves made by the solver, over all its runs, and the states
    /// it looked at to find them.
    pub moves: usize,
    pub scans: usize,
    /// Ways round a part found in the T-mesh, and those the lengths kept
    /// were looked for to bring back onto themselves (DT-062).
    pub turns: usize,
    pub turns_held: usize,
    /// Share of the edges of the loops of the quad mesh the lengths stand
    /// for that wind as spirals, where it was counted: when loops are closed.
    pub spirals: Option<f64>,
    /// These lengths come from a round of the slow quality mode.
    pub from_selector: bool,
}

impl Report {
    pub fn is_valid(&self) -> bool {
        self.balanced && self.separated
    }
}

/// The whole lengths of a T-mesh.
#[derive(Clone, Debug)]
pub struct Quantization {
    /// Per arc of the T-mesh, in quad edges.
    pub lengths: Vec<u32>,
    pub report: Report,
    /// Where loops were closed, the other lengths that could have been kept,
    /// the best first (the fewest spirals), and last the lengths found
    /// without closing any: what to mesh instead if these fold over.
    pub candidates: Vec<Quantization>,
}

impl Quantization {
    /// Connectivity of the quad mesh these lengths stand for on `tmesh`.
    pub fn lattice(&self, tmesh: &TMesh) -> Lattice {
        let lengths: Vec<i64> = self.lengths.iter().map(|&length| length as i64).collect();
        lattice::build(tmesh, &lengths).lattice
    }

    /// Width and height of each patch of `tmesh` under these lengths: its
    /// first two sides.
    pub fn patch_sizes(&self, tmesh: &TMesh) -> Vec<[u32; 2]> {
        tmesh
            .patches
            .iter()
            .map(|patch| {
                let mut size = [0; 2];
                if let Some(side_of) = patch.side_of_each() {
                    for (&half, &side) in patch.boundary.iter().zip(&side_of) {
                        if side < 2 {
                            size[side as usize] += self.lengths[half as usize / 2];
                        }
                    }
                }
                size
            })
            .collect()
    }
}

pub trait Quantizer {
    fn kind(&self) -> Kind;
    fn quantize(&self, tmesh: &TMesh, request: &Request) -> Result<Quantization>;
}

/// The quantizers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Minimum-deviation flow on the strip network (after Heistermann et al.
    /// 2023), solved approximately by cancelling improving walks.
    WalkCancelling,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::WalkCancelling];

    pub fn name(self) -> &'static str {
        match self {
            Kind::WalkCancelling => "walk-cancelling",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        Kind::ALL.iter().copied().find(|k| k.name() == name).ok_or_else(|| {
            let known: Vec<&str> = Kind::ALL.iter().map(|k| k.name()).collect();
            Error::InvalidParams(format!("unknown quantizer '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the quantizer of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn Quantizer> {
    match kind {
        Kind::WalkCancelling => Box::new(WalkCancelling::default()),
    }
}

#[derive(Clone, Debug)]
pub struct WalkCancelling {
    /// Most changes of scale tried to reach the number of quads asked for.
    pub scale_rounds: usize,
    /// Most rounds of holding arcs open to keep the structure intact.
    pub separation_rounds: usize,
    /// Spread of the preference given to each arc, as a fraction of its
    /// length, so that equal arcs do not all change together.
    pub jitter: f64,
}

impl Default for WalkCancelling {
    fn default() -> Self {
        Self { scale_rounds: 24, separation_rounds: 200, jitter: 0.01 }
    }
}

/// The ways round a part of `tmesh`, from these places along the sides of
/// its rectangles: every one short enough to go round one part (none is to
/// be shifted by the lengths), and those of them the real lengths shift.
fn turns_of(
    tmesh: &TMesh,
    real: &[f64],
    offsets: &[f64],
    lean: f64,
    longest: f64,
) -> (Vec<tubes::Turn>, Vec<tubes::Turn>) {
    let area: f64 = tmesh.patches.iter().filter_map(|patch| patch.sides).map(|sides| sides[0] * sides[1]).sum();
    let side = area.sqrt().max(1.0);
    let found = tubes::turns(tmesh, 4, offsets, longest * side);
    let (mut every, mut held) = (Vec::new(), Vec::new());
    let mut same_way: std::collections::HashSet<Vec<(usize, i64)>> = std::collections::HashSet::new();
    let mut same_ring = std::collections::HashSet::new();
    let debug = std::env::var_os("OPENCLARE_DEBUG_TURNS").is_some();
    for turn in found {
        let shift = tubes::value(&turn.shift, real);
        if debug {
            let first = tmesh.patches[turn.patch].boundary[0] as usize / 2;
            let at = tmesh.nodes[tmesh.arcs[first].nodes[0] as usize].position;
            eprintln!(
                "turn: round {:7.1} shift {:+7.2} lean {:.3} at ({:+.3}, {:+.3}, {:+.3}){}",
                turn.around,
                shift,
                shift.abs() / turn.around,
                at.x,
                at.y,
                at.z,
                if turn.around > longest * side {
                    " too long"
                } else if shift.abs() > lean * turn.around {
                    " leans too much"
                } else {
                    ""
                }
            );
        }
        if turn.around > longest * side || shift.abs() > lean * turn.around {
            continue;
        }
        let key: Vec<(usize, i64)> = turn.shift.iter().map(|&(arc, c)| (arc, c.round() as i64)).collect();
        if !same_way.insert(key) {
            continue;
        }
        // The same ring found again, from another rectangle or the other
        // way round, has the same shift up to its sign; two rings that merely
        // shift as much (a left and a right limb) are both held (DT-065).
        if shift.abs() > TURN_SHIFT && same_ring.insert(ring(&turn.shift)) {
            held.push(turn.clone());
        }
        every.push(turn);
    }
    (every, held)
}

/// A shift as a key that is the same for the shift taken the other way:
/// its whole coefficients, the first made positive.
fn ring(shift: &tubes::Expression) -> Vec<(usize, i64)> {
    let sign = shift.first().map_or(1.0, |&(_, c)| c.signum());
    shift.iter().map(|&(arc, c)| (arc, (sign * c).round() as i64)).collect()
}

/// Prints where each turn of `every` that `x` leaves shifted starts, and
/// how far it was shifted by the real lengths.
fn debug_turns(tmesh: &TMesh, every: &[tubes::Turn], x: &[i64], real: &[f64], scale: f64, held: &[&tubes::Turn]) {
    let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
    for turn in every {
        let left = tubes::value(&turn.shift, &lengths);
        if left.abs() < 0.5 {
            continue;
        }
        let first = tmesh.patches[turn.patch].boundary[0] as usize / 2;
        let at = tmesh.nodes[tmesh.arcs[first].nodes[0] as usize].position;
        let kept = held.iter().any(|h| std::ptr::eq(*h, turn) || h.shift == turn.shift);
        eprintln!(
            "  shifted {:+3.0}: round {:6.1} real shift {:+6.2} at ({:+.3}, {:+.3}, {:+.3}){}",
            left,
            scale * turn.around,
            scale * tubes::value(&turn.shift, real),
            at.x,
            at.y,
            at.z,
            if kept { " held" } else { "" }
        );
    }
}

/// Work the statements proven in the research note spare the repair,
/// counted over the whole process for `OPENCLARE_DEBUG_WORK`.
mod work {
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Walks of the repair turned down before the quad mesh is built.
    pub static BUILDS_SPARED: AtomicUsize = AtomicUsize::new(0);
    /// Walks aimed at one turn turned down by the parity of its length along.
    pub static WALKS_SPARED: AtomicUsize = AtomicUsize::new(0);
    /// Turns held no walk is looked for, being sign-definite at their least.
    pub static ATTEMPTS_SPARED: AtomicUsize = AtomicUsize::new(0);
    /// Strips held by the cuts that are of one sign on an arc held open:
    /// never closed, held all the same.
    pub static NEVER_CLOSED_HELD: AtomicUsize = AtomicUsize::new(0);

    pub fn count(counter: &AtomicUsize) {
        counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn summary() -> String {
        let get = |counter: &AtomicUsize| counter.load(Ordering::Relaxed);
        format!(
            "spared so far: {} quad meshes built, {} aimed walks judged, {} turns aimed at; {} strips held never closed",
            get(&BUILDS_SPARED),
            get(&WALKS_SPARED),
            get(&ATTEMPTS_SPARED),
            get(&NEVER_CLOSED_HELD)
        )
    }
}

/// In builds with debug assertions, what is proven of `turns` at the whole
/// lengths `x`, which balance every rectangle: their parity on the pieces
/// that are spheres (`sphere`, per rectangle) and, if they were `walked` at
/// `x`, their length along. A break is a bug: it panics, saying where.
fn check_turns(tmesh: &TMesh, network: &Network, turns: &[tubes::Turn], sphere: &[bool], x: &[i64], walked: bool) {
    if !cfg!(debug_assertions) {
        return;
    }
    assert!(network.is_balanced(x), "whole lengths that do not balance every rectangle");
    let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
    let broken = tubes::broken_by(turns, sphere, &lengths, walked);
    if !broken.is_empty() {
        let at = |patch: usize| {
            let first = tmesh.patches[patch].boundary[0] as usize / 2;
            tmesh.nodes[tmesh.arcs[first].nodes[0] as usize].position
        };
        let first = &turns[broken[0].0];
        panic!(
            "a proven statement on turns breaks on a T-mesh of {} rectangles and {} arcs, first at ({:+.3}, {:+.3}, {:+.3}): {}",
            tmesh.patches.len(),
            tmesh.arcs.len(),
            at(first.patch).x,
            at(first.patch).y,
            at(first.patch).z,
            broken.iter().map(|(_, what)| what.as_str()).collect::<Vec<_>>().join("; ")
        );
    }
}

/// Whole lengths, their scale, the quads they make, and the arcs held from
/// vanishing.
type Lengths = (Vec<i64>, f64, usize, Vec<i64>);

/// Walks taken one at a time on `x`, guided by a lean towards less shift,
/// taken only if they shift the turns of `every` less in all, cost little
/// away from `target`, keep the number of quads within `budget` if there is
/// one (the number asked for, and how far from it is close enough) and
/// leave the structure intact; then for each turn of `held` still shifted,
/// one at a time.
#[allow(clippy::too_many_arguments)]
fn repair(
    tmesh: &TMesh,
    network: &Network,
    pull: &[Vec<(f64, f64)>],
    target: &[f64],
    lower: &[i64],
    x: &mut [i64],
    every: &[tubes::Turn],
    held: &[&tubes::Turn],
    budget: Option<(usize, usize)>,
    sphere: &[bool],
    report: &mut Report,
) {
    let clock = std::time::Instant::now();
    let weight: Vec<f64> = target.iter().map(|length| 1.0 / (length + 1.0)).collect();
    let left_by = |x: &[i64]| -> Vec<f64> {
        let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
        every.iter().map(|turn| tubes::value(&turn.shift, &lengths)).collect()
    };
    let mut bias = vec![0.0; target.len()];
    for (turn, shift) in every.iter().zip(left_by(x)) {
        if shift.abs() >= 0.5 {
            for &(arc, c) in &turn.shift {
                bias[arc] += REPAIR_LEAN * shift.signum() * c;
            }
        }
    }
    let plain = Cost { target, weight: &weight, lower, bias: &[], folds: pull };
    let leant = Cost { target, weight: &weight, lower, bias: &bias, folds: pull };
    // Fewer quads shift every turn less: a walk may not take the number of
    // quads further from the budget than it allows, or than it already is.
    let within = |x: &[i64], moved: &[i64]| -> bool {
        budget.is_none_or(|(wanted, allowed)| {
            count_quads(tmesh, moved).abs_diff(wanted) <= allowed.max(count_quads(tmesh, x).abs_diff(wanted))
        })
    };
    let mut judge = |x: &[i64], change: &[i64]| -> bool {
        let moved: Vec<i64> = x.iter().zip(change).map(|(a, b)| a + b).collect();
        let now: f64 = left_by(x).iter().map(|w| w.abs()).sum();
        let then: f64 = left_by(&moved).iter().map(|w| w.abs()).sum();
        let cost: f64 = change
            .iter()
            .enumerate()
            .filter(|&(_, &delta)| delta != 0)
            .map(|(arc, &delta)| plain.of(arc, x[arc] + delta) - plain.of(arc, x[arc]))
            .sum();
        let sound = || within(x, &moved) && lattice::build(tmesh, &moved).defects(tmesh).is_empty();
        if std::env::var_os("OPENCLARE_DEBUG_REPAIR").is_some() {
            let size = change.iter().filter(|&&d| d != 0).count();
            let sound = sound();
            eprintln!("    walk of {size} arcs: shift {now:.1} -> {then:.1}, cost {cost:.2}, sound {sound}");
            return then < now - 0.5 && cost < REPAIR_PRICE * (now - then) && sound;
        }
        // Building the quad mesh is the dearest part of the judgement: only
        // for a walk that is taken otherwise.
        let taken = then < now - 0.5 && cost < REPAIR_PRICE * (now - then);
        if !taken {
            work::count(&work::BUILDS_SPARED);
        }
        taken && sound()
    };
    let mut effort = flow::Effort::default();
    network.improve_judged(x, &leant, 1, REPAIR_WALKS, &mut effort, &mut judge);
    report.moves += effort.moves;
    report.scans += effort.scans;
    if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
        eprintln!(
            "  repaired in {:.1} s: {} walks taken, {} turned down",
            clock.elapsed().as_secs_f64(),
            effort.moves,
            effort.misjudged
        );
    }
    // Then each turn held that is still shifted, one at a time: a lean on
    // its arcs alone, a walk taken if it shifts that turn less and the
    // others not more.
    let clock = std::time::Instant::now();
    let mut effort = flow::Effort::default();
    for turn in held {
        let shift_of = |x: &[i64]| -> f64 { turn.shift.iter().map(|&(arc, c)| c * x[arc] as f64).sum() };
        let left = shift_of(x);
        if left.abs() < 0.5 {
            continue;
        }
        // A turn whose shift has its coefficients all of one sign is never
        // shifted less than with each of its arcs at the least a walk taken
        // may leave it: then no walk can be taken for it, and none is looked
        // for (the research note, Remark "Sign-definite turns").
        let one_sign = turn.shift.iter().all(|&(_, c)| c > 0.0) || turn.shift.iter().all(|&(_, c)| c < 0.0);
        if one_sign {
            // A walk taking an arc below its bound costs more than any
            // walk taken may when every arc is at or above its bound and
            // the price of one unit below is more than all the rest.
            let rest: f64 = (0..x.len()).map(|arc| plain.of(arc, x[arc])).sum();
            let now: f64 = left_by(x).iter().map(|w| w.abs()).sum();
            let bounded = x.iter().zip(lower).all(|(&length, &bound)| length >= bound)
                && flow::SHORTFALL - rest >= REPAIR_PRICE * now.max(1.0);
            let least: f64 =
                turn.shift.iter().map(|&(arc, c)| c.abs() * if bounded { lower[arc] as f64 } else { 0.0 }).sum();
            if least >= left.abs() {
                work::count(&work::ATTEMPTS_SPARED);
                continue;
            }
        }
        // On a sphere, a turn shifted by one is closed only by a walk that
        // changes its length along by an odd number: whole and balanced,
        // the two add up to an even number before the walk and after (the
        // research note, Corollary "Parity" (c)).
        let odd_along = |change: &[i64]| -> bool {
            turn.along.iter().map(|&(arc, c)| c as i64 * change[arc]).sum::<i64>().rem_euclid(2) == 1
        };
        let parity_holds = sphere[turn.patch];
        let mut bias = vec![0.0; target.len()];
        for &(arc, c) in &turn.shift {
            bias[arc] += AIMED_LEAN * left.signum() * c;
        }
        let leant = Cost { target, weight: &weight, lower, bias: &bias, folds: pull };
        let debug_repair = std::env::var_os("OPENCLARE_DEBUG_REPAIR").is_some();
        let mut judge = |x: &[i64], change: &[i64]| -> bool {
            let moved: Vec<i64> = x.iter().zip(change).map(|(a, b)| a + b).collect();
            if parity_holds && !debug_repair && shift_of(x).abs() == 1.0 && !odd_along(change) {
                debug_assert!(
                    shift_of(&moved).abs() >= 1.0,
                    "a walk keeping the parity of its length along closed a turn shifted by one on a sphere"
                );
                work::count(&work::WALKS_SPARED);
                return false;
            }
            let now: f64 = left_by(x).iter().map(|w| w.abs()).sum();
            let then: f64 = left_by(&moved).iter().map(|w| w.abs()).sum();
            let cost: f64 = change
                .iter()
                .enumerate()
                .filter(|&(_, &delta)| delta != 0)
                .map(|(arc, &delta)| plain.of(arc, x[arc] + delta) - plain.of(arc, x[arc]))
                .sum();
            let better = shift_of(&moved).abs() < shift_of(x).abs() - 0.5 && then <= now;
            let sound = better && within(x, &moved) && lattice::build(tmesh, &moved).defects(tmesh).is_empty();
            if std::env::var_os("OPENCLARE_DEBUG_REPAIR").is_some() {
                eprintln!(
                    "    aimed walk: turn {:.0} -> {:.0}, all {now:.0} -> {then:.0}, cost {cost:.2}, sound {sound}",
                    shift_of(x),
                    shift_of(&moved)
                );
            }
            better && cost < REPAIR_PRICE * (now - then).max(1.0) && sound
        };
        network.improve_judged(x, &leant, 1, AIMED_WALKS, &mut effort, &mut judge);
    }
    report.moves += effort.moves;
    report.scans += effort.scans;
    if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
        eprintln!(
            "  aimed in {:.1} s: {} walks taken, {} turned down, {} quads",
            clock.elapsed().as_secs_f64(),
            effort.moves,
            effort.misjudged,
            count_quads(tmesh, x)
        );
    }
}

impl WalkCancelling {
    /// Whole lengths for `tmesh` with the turns `held` brought back onto
    /// themselves: the lengths aimed at changed as little as can be for
    /// that (turns that cannot all be are let go, the worst first), the
    /// search, then walks taken one at a time that shift `every` turn less.
    /// Outside the rounds of `cut`s, arcs the least change would shorten
    /// below nothing are held at nothing so that the turns stay closed; in
    /// them, the walks keep to the budget and none is aimed at a turn alone,
    /// and the search starts `from` the lengths of the round before. The
    /// least change is solved with `closing`, its part for the rectangles
    /// factorized once for all. A `way` of closing (its place in
    /// [`CLOSE_WAYS`]; none in the rounds of cuts) is what the files of the
    /// experiment options are read and written for.
    #[allow(clippy::too_many_arguments)]
    fn closed_search(
        &self,
        tmesh: &TMesh,
        network: &Network,
        pull: &[Vec<(f64, f64)>],
        real: &[f64],
        request: &Request,
        report: &mut Report,
        every: &[tubes::Turn],
        held: &[tubes::Turn],
        cut: bool,
        from: Option<&Lengths>,
        closing: &tubes::Closing,
        sphere: &[bool],
        way: Option<usize>,
    ) -> Lengths {
        let debug = std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some();
        let experiment = &request.experiment;
        let clock = std::time::Instant::now();
        let asked = held;
        let mut held: Vec<&tubes::Turn> = held.iter().collect();
        let mut aimed = real.to_vec();
        let mut projections = 0;
        // (Experiment.) The lengths aimed at and the turns closed, worked
        // out outside the engine: with the lengths, nothing is left for the
        // projection to do; without, it closes those turns alone.
        if let Some((values, closed)) = way.and_then(|way| experiment.aimed(tmesh, way)) {
            held = closed.into_iter().filter_map(|k| asked.get(k)).collect();
            if let Some(values) = values {
                aimed = values;
                projections = usize::MAX;
            }
        }
        let mut rows = tubes::Rows::new(closing);
        let mut active: Vec<usize> = held.iter().map(|turn| rows.push(turn.shift.clone())).collect();
        while !held.is_empty() && projections != usize::MAX {
            projections += 1;
            let pin = !cut && experiment.active_set;
            let Some(closed) = rows.closed(&active, &vec![0.0; held.len()], pin) else { break };
            let left: Vec<f64> = held.iter().map(|turn| tubes::value(&turn.shift, &closed).abs()).collect();
            let (worst, at) =
                left.iter().enumerate().fold((0.0, 0), |(w, a), (k, &v)| if v > w { (v, k) } else { (w, a) });
            if worst <= 1e-3 {
                aimed = closed;
                break;
            }
            held.remove(at);
            active.remove(at);
        }
        if debug {
            eprintln!(
                "  projected {projections} times, {} turns kept, {:.1} s",
                held.len(),
                clock.elapsed().as_secs_f64()
            );
        }
        let clock = std::time::Instant::now();
        let (mut x, scale, _, lower) = self.search(tmesh, network, pull, &aimed, request, report, from);
        if debug {
            eprintln!("  searched in {:.1} s", clock.elapsed().as_secs_f64());
        }
        // (Experiment.) Whole lengths worked out outside the engine, taken
        // as they are, with no repair: the search gave the scale and the
        // arcs held from vanishing.
        if let Some(imported) = way.and_then(|way| experiment.way_lengths(tmesh, way)) {
            x = imported;
        } else {
            let target: Vec<f64> = aimed.iter().map(|length| scale * length).collect();
            let (budget, aimed_at): (_, &[&tubes::Turn]) = if cut {
                let wanted = request.quads.unwrap_or_else(|| count_quads(tmesh, &x));
                (Some((wanted, (request.tolerance * wanted as f64) as usize)), &[])
            } else if !experiment.aimed_repair {
                (None, &[])
            } else {
                (None, &held)
            };
            repair(tmesh, network, pull, &target, &lower, &mut x, every, aimed_at, budget, sphere, report);
            check_turns(tmesh, network, every, sphere, &x, false);
        }
        if std::env::var_os("OPENCLARE_DEBUG_TURNS").is_some() {
            debug_turns(tmesh, every, &x, real, scale, &held);
        }
        if debug {
            let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
            let open = |turns: &mut dyn Iterator<Item = &tubes::Turn>| {
                turns.filter(|turn| tubes::value(&turn.shift, &lengths).abs() >= 0.5).count()
            };
            eprintln!(
                "  after the integer search: {} of {} kept turns, {} of {} held, {} of {} found still shifted",
                open(&mut held.iter().copied()),
                held.len(),
                open(&mut asked.iter()),
                asked.len(),
                open(&mut every.iter()),
                every.len()
            );
        }
        let quads = count_quads(tmesh, &x);
        let lengths = (x, scale, quads, lower);
        if let (Some(directory), Some(way)) = (experiment.export.as_deref(), way) {
            export::way(directory, tmesh, real, way, &lengths, every, asked, &held);
        }
        lengths
    }

    /// Closing by cuts: every strip of quads the lengths of `first` make is
    /// walked, from the middle of every quad edge along the sides of the
    /// rectangles, which finds every one that comes back; those that come
    /// back beside themselves although the real lengths bring them back to
    /// within [`CUT_LEAN`] of their way round are held as well, and the
    /// lengths looked for again, from the lengths of the round before. A few
    /// rounds, each giving lengths to choose from, with the number of turns
    /// held.
    #[allow(clippy::too_many_arguments)]
    fn cut_rounds(
        &self,
        tmesh: &TMesh,
        network: &Network,
        pull: &[Vec<(f64, f64)>],
        real: &[f64],
        request: &Request,
        report: &mut Report,
        every: &[tubes::Turn],
        held: &[tubes::Turn],
        first: &Lengths,
        closing: &tubes::Closing,
        sphere: &[bool],
    ) -> Vec<(Lengths, usize)> {
        // A way and the same way taken the other way round are one ring.
        let key = |turn: &tubes::Turn| -> Vec<(usize, i64)> {
            let sign = turn.shift.first().map_or(1.0, |&(_, c)| c.signum());
            turn.shift.iter().map(|&(arc, c)| (arc, (sign * c).round() as i64)).collect()
        };
        let (mut every, mut held) = (every.to_vec(), held.to_vec());
        let mut known_every: std::collections::HashSet<_> = every.iter().map(key).collect();
        let mut known_held: std::collections::HashSet<_> = held.iter().map(key).collect();
        let mut out = Vec::new();
        let mut current = first.clone();
        for round in 0..CUT_ROUNDS {
            let (x, scale, quads, lower) = &current;
            let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
            let side = (*quads as f64).sqrt().max(1.0);
            let mut undue = 0;
            let strips = tubes::lattice_turns(tmesh, &lengths, 4, CUT_LONGEST * side);
            check_turns(tmesh, network, &strips, sphere, x, true);
            for strip in strips {
                // How far the real lengths bring it back beside itself, in
                // quad edges: beyond the lean, it winds round as the field
                // does, and is not held.
                let field = scale * tubes::value(&strip.shift, real);
                if field.abs() > CUT_LEAN * strip.around {
                    continue;
                }
                if tubes::value(&strip.shift, &lengths).abs() >= 0.5 && known_held.insert(key(&strip)) {
                    // Of one sign and on an arc held open, it is never closed
                    // (the research note, Remark "Sign-definite turns"):
                    // counted, still held.
                    let one_sign =
                        strip.shift.iter().all(|&(_, c)| c > 0.0) || strip.shift.iter().all(|&(_, c)| c < 0.0);
                    if one_sign && strip.shift.iter().any(|&(arc, _)| lower[arc] >= 1) {
                        work::count(&work::NEVER_CLOSED_HELD);
                    }
                    undue += 1;
                    held.push(strip.clone());
                }
                if known_every.insert(key(&strip)) {
                    every.push(strip);
                }
            }
            if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                eprintln!("  cut round {round}: {undue} strips more held, {} in all", held.len());
            }
            if undue == 0 {
                break;
            }
            let next = self.closed_search(
                tmesh,
                network,
                pull,
                real,
                request,
                report,
                &every,
                &held,
                true,
                Some(&current),
                closing,
                sphere,
                None,
            );
            if next.0 == current.0 {
                break;
            }
            out.push((next.clone(), held.len()));
            current = next;
        }
        out
    }

    /// Whole lengths near `real` at the scale that gives the number of quads
    /// asked for, balanced and leaving the structure intact: the lengths,
    /// the scale, the quads they make, and the arcs held from vanishing.
    /// Arcs are drawn to put a vertex on the folds `pull` lists. The search
    /// starts from nothing, or `from` lengths found for nearly the same
    /// targets: only what changed is left to find.
    #[allow(clippy::too_many_arguments)]
    fn search(
        &self,
        tmesh: &TMesh,
        network: &Network,
        pull: &[Vec<(f64, f64)>],
        real: &[f64],
        request: &Request,
        report: &mut Report,
        from: Option<&Lengths>,
    ) -> Lengths {
        let arcs = real.len();
        // From nothing, or from lengths already found for nearly the same
        // targets: their scale, and the arcs they had to hold open.
        let (mut x, mut lower, mut scale) = match from {
            Some((x, scale, _, lower)) => (x.clone(), lower.clone(), *scale),
            None => (vec![0i64; arcs], vec![0i64; arcs], 1.0),
        };
        // The best assignment seen: lengths, scale, quads.
        let mut best: Option<(Vec<i64>, f64, usize)> = None;
        // Scales known to give too few quads and too many.
        let (mut too_small, mut too_large) = (None::<f64>, None::<f64>);
        let rounds = if request.quads.is_some() { self.scale_rounds } else { 1 };
        let (scans_before, mut scale_count, mut separation_count) = (report.scans, 0, 0);
        for _ in 0..rounds {
            scale_count += 1;
            let target: Vec<f64> = real.iter().map(|length| scale * length).collect();
            // Relative to the length: one edge more or less matters on a
            // short arc, not on a long one.
            let weight: Vec<f64> = target.iter().map(|length| 1.0 / (length + 1.0)).collect();
            let mut separated = false;
            for _ in 0..self.separation_rounds {
                separation_count += 1;
                let cost = Cost { target: &target, weight: &weight, lower: &lower, bias: &[], folds: pull };
                let effort = network.improve(&mut x, &cost);
                report.moves += effort.moves;
                report.scans += effort.scans;
                // The lengths are sound if the quad mesh they stand for is;
                // where it is not, something closed up that must not.
                let defects = lattice::build(tmesh, &x).defects(tmesh);
                if defects.reopen.is_empty() {
                    separated = defects.is_empty();
                    break;
                }
                for arc in defects.reopen {
                    lower[arc] = 1;
                }
            }
            report.separated &= separated;

            let quads = count_quads(tmesh, &x);
            let wanted = request.quads.unwrap_or(quads);
            let miss = |count: usize| count.abs_diff(wanted);
            if best.as_ref().is_none_or(|(_, _, kept)| miss(quads) < miss(*kept)) {
                best = Some((x.clone(), scale, quads));
            }
            if miss(quads) as f64 <= request.tolerance * wanted as f64 {
                break;
            }
            if quads < wanted {
                too_small = Some(scale);
            } else {
                too_large = Some(scale);
            }
            scale = match (too_small, too_large) {
                (Some(low), Some(high)) => 0.5 * (low + high),
                // Quads go as the square of the scale; with none at all,
                // anything larger is a start.
                _ if quads == 0 => 2.0 * scale,
                _ => scale * (wanted as f64 / quads as f64).sqrt(),
            };
        }

        if std::env::var_os("OPENCLARE_DEBUG_WORK").is_some() {
            eprintln!(
                "  search: {scale_count} scales, {separation_count} separation rounds, {} scans",
                report.scans - scans_before
            );
        }
        let (x, scale, quads) = best.unwrap_or((x, 1.0, 0));
        (x, scale, quads, lower)
    }
}

impl Quantizer for WalkCancelling {
    fn kind(&self) -> Kind {
        Kind::WalkCancelling
    }

    fn quantize(&self, tmesh: &TMesh, request: &Request) -> Result<Quantization> {
        if tmesh.patches.iter().any(|patch| patch.sides.is_none()) {
            return Err(Error::InvalidMesh("the T-mesh is not a partition into rectangles".into()));
        }
        if request.quads == Some(0) || !(request.tolerance.is_finite() && request.tolerance >= 0.0) {
            return Err(Error::InvalidParams("quantization needs a positive number of quads and tolerance".into()));
        }
        let arcs = tmesh.arcs.len();
        let cpu_start = process_cpu_seconds();
        let network = strip_network(tmesh);
        // Per arc, the folds of the surface it crosses: how far along it, as
        // a share of its length, and how sharp. The search is drawn to put
        // a vertex on each, for an edge of the quad mesh to run along it.
        let mut pull = vec![Vec::new(); arcs];
        for (arc, share, sharpness) in arc_folds(tmesh) {
            pull[arc].push((share, sharpness));
        }
        let exact: Vec<f64> = tmesh.arcs.iter().map(|arc| arc.length).collect();
        let experiment = &request.experiment;
        // Arcs of equal length (a cube has nothing else) would all gain or
        // lose an edge at the same scale, and the number of quads would jump
        // by whole layers. A slight, fixed preference per arc spreads them out.
        // (An experiment may draw another.)
        let seed = experiment.jitter_seed.wrapping_mul(1_000_003);
        let real: Vec<f64> = exact
            .iter()
            .enumerate()
            .map(|(arc, length)| {
                length * (1.0 + self.jitter * (Rng::new((arc as u64).wrapping_add(seed)).next_f64() - 0.5))
            })
            .collect();

        let mut report = Report { arcs, scale: 1.0, separated: true, ..Default::default() };
        // Where the parity of turns holds: the pieces that are spheres.
        let sphere = tubes::on_spheres(tmesh);
        // Ways round a part that the whole lengths shift wind their loops
        // into helices (DT-062). Lengths closing them are looked for in more
        // than one way; where the spirals end up is sensitive to every
        // length, so each way's quad mesh is built and its spirals counted,
        // and the one with the fewest kept, the plain search among them.
        let close = request.close_turns;
        let clock = std::time::Instant::now();
        let plain = self.search(tmesh, &network, &pull, &real, request, &mut report, None);
        if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
            eprintln!(
                "plain search {:.1} s, {} arcs, {} patches",
                clock.elapsed().as_secs_f64(),
                arcs,
                tmesh.patches.len()
            );
        }
        // The lengths kept, the fewest spirals first; the plain lengths last.
        let kept: Vec<Kept> = if let Some((scale, x)) = experiment.call_lengths(tmesh) {
            // (Experiment.) Whole lengths worked out outside the engine, taken
            // as they are, the plain ones after them. Counted as closing a
            // turn, so that the plain lengths are meshed instead if they fold.
            let quads = count_quads(tmesh, &x);
            let imported = (x, scale.unwrap_or(plain.1), quads, plain.3.clone());
            let plain_spirals = spirals(tmesh, &plain.0);
            vec![
                Kept { spirals: Some(spirals(tmesh, &imported.0)), held: 1, ..Kept::new(imported) },
                Kept { spirals: Some(plain_spirals), ..Kept::new(plain) },
            ]
        } else if close {
            // As near the number of quads asked for as the plain search, or
            // within the tolerance: fewer spirals do not buy a different mesh.
            let (wanted, allowed) = budget(request, &plain);
            if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                eprintln!("plain lengths: deviation {:.3}", deviation(&plain, &real));
            }
            let most_deviation = most_deviation(deviation(&plain, &real), experiment);
            // How far the vertices along the arcs are from the folds of the
            // surface they cross; closing loops may not take them further off
            // than the plain lengths left them, by more than chance would.
            let folds = arc_folds(tmesh);
            let most_off_folds = most_off_folds(&folds, &plain.0, experiment);
            if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                eprintln!(
                    "plain lengths: {} folds crossed, off them by {:.2}",
                    folds.len(),
                    off_folds(&folds, &plain.0)
                );
            }
            let plain_spirals = spirals(tmesh, &plain.0);
            // Lengths that leave the structure broken are not mended by
            // closing loops, and each way would cost as long a search again.
            let sound = plain_spirals.is_finite() && lattice::build(tmesh, &plain.0).defects(tmesh).is_empty();
            // The least change of the real lengths that closes turns, its
            // part for the rectangles solved once for every way.
            let weight: Vec<f64> = real.iter().map(|length| 1.0 / (length + 1.0)).collect();
            let closing =
                if sound { tubes::Closing::new(tmesh, &real, &weight, experiment.active_set_floor) } else { None };
            // The ways, each with the turns it holds; the same turns held
            // twice would give the same lengths.
            let mut ways = Vec::new();
            let mut tried_held: Vec<Vec<Vec<(usize, i64)>>> = Vec::new();
            for (index, &(offsets, lean, longest)) in CLOSE_WAYS.iter().enumerate().filter(|_| closing.is_some()) {
                if !experiment.tries(index) {
                    continue;
                }
                let clock = std::time::Instant::now();
                // Shifts are read at the lengths of the parametrization, which
                // keep the balances: a shift is defined modulo them, and at the
                // jittered lengths its value would depend on how it is written
                // (DT-162).
                let (found, held) = turns_of(tmesh, &exact, offsets, lean, longest);
                if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                    eprintln!(
                        "turns from {offsets:?} leaning up to {lean}, {longest} long: {} found, {} held, {:.1} s",
                        found.len(),
                        held.len(),
                        clock.elapsed().as_secs_f64()
                    );
                }
                let key: Vec<Vec<(usize, i64)>> = held
                    .iter()
                    .map(|turn| turn.shift.iter().map(|&(arc, c)| (arc, c.round() as i64)).collect())
                    .collect();
                if held.is_empty() || tried_held.contains(&key) {
                    continue;
                }
                tried_held.push(key);
                report.turns = report.turns.max(found.len());
                ways.push(((index, offsets, lean, longest), found, held));
            }
            for (_, found, _) in &ways {
                check_turns(tmesh, &network, found, &sphere, &plain.0, false);
            }
            // Each way searched on a core of its own, then, if its lengths
            // could be kept, closed by cuts a few rounds; the ones kept are
            // the same as if they had been searched in turn.
            let closing = closing.as_ref();
            let tried: Vec<_> = ways
                .par_iter()
                .map(|((index, ..), found, held)| {
                    let mut own = Report::default();
                    let tried = self.closed_search(
                        tmesh,
                        &network,
                        &pull,
                        &real,
                        request,
                        &mut own,
                        found,
                        held,
                        false,
                        None,
                        closing.unwrap(),
                        &sphere,
                        Some(*index),
                    );
                    // Only lengths that could be kept are refined by cuts;
                    // lengths given by an experiment are taken as they are.
                    let admissible = tried.2.abs_diff(wanted) <= allowed && deviation(&tried, &real) <= most_deviation;
                    let cut = if admissible && experiment.cut_rounds && experiment.way_lengths(tmesh, *index).is_none()
                    {
                        self.cut_rounds(
                            tmesh,
                            &network,
                            &pull,
                            &real,
                            request,
                            &mut own,
                            found,
                            held,
                            &tried,
                            closing.unwrap(),
                            &sphere,
                        )
                    } else {
                        Vec::new()
                    };
                    let candidates: Vec<_> = std::iter::once((tried, held.len()))
                        .chain(cut)
                        .map(|(tried, held)| (spirals(tmesh, &tried.0), tried, held))
                        .collect();
                    (candidates, own.moves, own.scans)
                })
                .collect();
            // All the lengths with fewer spirals than the plain ones, within the
            // budget, not too far from the real lengths nor off the folds.
            let mut kept: Vec<Kept> = Vec::new();
            for (((index, offsets, lean, longest), ..), (candidates, moves, scans)) in ways.iter().zip(tried) {
                report.moves += moves;
                report.scans += scans;
                for (round, (spirals, tried, held)) in candidates.into_iter().enumerate() {
                    if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                        eprintln!(
                            "closing from {offsets:?} leaning up to {lean}, {longest} long, round {round}: spirals {:.3} deviation {:.3} quads {} off folds {:.2} (plain {:.3})",
                            spirals,
                            deviation(&tried, &real),
                            tried.2,
                            off_folds(&folds, &tried.0),
                            plain_spirals
                        );
                    }
                    if spirals < plain_spirals - 1e-9
                        && tried.2.abs_diff(wanted) <= allowed
                        && deviation(&tried, &real) <= most_deviation
                        && off_folds(&folds, &tried.0) <= most_off_folds
                        && kept.iter().all(|other| other.lengths.0 != tried.0)
                    {
                        kept.push(Kept {
                            spirals: Some(spirals),
                            held,
                            way: Some((*index, round)),
                            ..Kept::new(tried)
                        });
                    }
                }
            }
            // The fewest spirals first; of equal ones, the way tried first.
            let fewest_first =
                |a: &Kept, b: &Kept| a.spirals.partial_cmp(&b.spirals).unwrap_or(std::cmp::Ordering::Equal);
            kept.sort_by(fewest_first);
            // The slow quality mode: rounds of rings chosen by the selector
            // from the lengths with the fewest spirals, each round's lengths
            // kept on the same terms as the engine's own, after them where
            // they spiral as much. Where the engine's lengths already wind no
            // loop into a spiral, the rounds cannot make fewer, and are not
            // posed: the piece is then the standard quality's to the byte.
            let fewest = kept.first().map_or(plain_spirals, |first| first.spirals.unwrap_or(plain_spirals));
            if let Some(slow) = request.slow.as_ref().filter(|slow| sound && fewest > 0.0 && !slow.selector.0.is_mute())
            {
                let (clock, cpu) = (std::time::Instant::now(), process_cpu_seconds());
                let start = kept.first().map_or(&plain, |first| &first.lengths).clone();
                let found = rings::rounds(tmesh, &real, &start, wanted, allowed, most_deviation, slow);
                let mut added = 0;
                for (round, (x, lower, closed)) in found.into_iter().enumerate() {
                    let quads = count_quads(tmesh, &x);
                    let tried = (x, start.1, quads, lower);
                    let spirals = spirals(tmesh, &tried.0);
                    slow_log(&format!(
                        "round {} {round}: spirals {spirals:.5} (engine {fewest:.5}, plain {plain_spirals:.5}), {closed} rings closed, same as start {}",
                        experiment::call_name(tmesh),
                        tried.0 == start.0
                    ));
                    if spirals < plain_spirals - 1e-9
                        && network.is_balanced(&tried.0)
                        && lattice::build(tmesh, &tried.0).defects(tmesh).is_empty()
                        && tried.2.abs_diff(wanted) <= allowed
                        && deviation(&tried, &real) <= most_deviation
                        && off_folds(&folds, &tried.0) <= most_off_folds
                        && kept.iter().all(|other| other.lengths.0 != tried.0)
                    {
                        added += 1;
                        kept.push(Kept {
                            spirals: Some(spirals),
                            held: closed.max(1),
                            from_selector: true,
                            ..Kept::new(tried)
                        });
                    }
                }
                kept.sort_by(fewest_first);
                slow_log(&format!(
                    "piece {}: {added} kept, first {} at {:.5}, {:.1} s, cpu {:.1} s",
                    experiment::call_name(tmesh),
                    if kept.first().is_some_and(|first| first.from_selector) { "selector" } else { "engine" },
                    kept.first().map_or(plain_spirals, |first| first.spirals.unwrap_or(plain_spirals)),
                    clock.elapsed().as_secs_f64(),
                    process_cpu_seconds() - cpu
                ));
                if std::env::var_os("OPENCLARE_DEBUG_WORK").is_some() {
                    eprintln!(
                        "slow quality: {} rounds by {}, {added} lengths kept, {:.1} s, cpu {:.1} s; the first is {}",
                        rings::ROUNDS,
                        slow.selector.0.name(),
                        clock.elapsed().as_secs_f64(),
                        process_cpu_seconds() - cpu,
                        if kept.first().is_some_and(|first| first.from_selector) {
                            "the selector's"
                        } else {
                            "the engine's"
                        }
                    );
                }
            } else if request.slow.is_some() && std::env::var_os("OPENCLARE_DEBUG_WORK").is_some() {
                eprintln!("slow quality: not posed, the engine's lengths spiral {fewest:.4}, sound {sound}");
            }
            // Other draws of the per-arc preference, each closed as the engine
            // closes its own, posed where the slow rounds would be (lengths
            // that still spiral) and kept on the same terms: choosing among
            // more lengths is most of what lowers the spirals (DT-089).
            let draws = experiment.extra_draws.unwrap_or(OTHER_DRAWS);
            if draws > 0 && sound && fewest > 0.0 {
                let clock = std::time::Instant::now();
                let mut added = 0;
                // Drawn side by side, looked at in the order of their seeds.
                let drawn: Vec<_> = (1..=draws as u64)
                    .into_par_iter()
                    .map(|draw| {
                        let other = Request {
                            slow: None,
                            experiment: Experiment {
                                jitter_seed: experiment.jitter_seed.wrapping_add(draw),
                                extra_draws: Some(0),
                                export: None,
                                ..experiment.clone()
                            },
                            ..request.clone()
                        };
                        // Only the lengths its closing kept: its plain ones
                        // are no other candidate.
                        let drawn = self.quantize(tmesh, &other).ok()?;
                        (drawn.report.turns_held > 0).then(|| {
                            let x: Vec<i64> = drawn.lengths.iter().map(|&length| length as i64).collect();
                            ((x, drawn.report.scale, drawn.report.quads, plain.3.clone()), drawn.report.turns_held)
                        })
                    })
                    .collect();
                // No further from the real lengths than the engine's choice:
                // on the ogre at 600 quads a draw spiralling less was 16 %
                // further from the shape on average.
                let engine_deviation =
                    kept.first().map_or(most_deviation, |first| deviation(&first.lengths, &real)).min(most_deviation);
                for (tried, held) in drawn.into_iter().flatten() {
                    if deviation(&tried, &real) > engine_deviation {
                        continue;
                    }
                    let spirals = spirals(tmesh, &tried.0);
                    if spirals < plain_spirals - 1e-9
                        && network.is_balanced(&tried.0)
                        && lattice::build(tmesh, &tried.0).defects(tmesh).is_empty()
                        && tried.2.abs_diff(wanted) <= allowed
                        && deviation(&tried, &real) <= most_deviation
                        && off_folds(&folds, &tried.0) <= most_off_folds
                        && kept.iter().all(|other| other.lengths.0 != tried.0)
                    {
                        added += 1;
                        kept.push(Kept { spirals: Some(spirals), held, ..Kept::new(tried) });
                    }
                }
                let first_before = kept.first().map(|first| first.lengths.0.clone());
                kept.sort_by(fewest_first);
                slow_log(&format!(
                    "draws {}: {added} kept, first {} at {:.5} (engine {fewest:.5}), {:.1} s",
                    experiment::call_name(tmesh),
                    if kept.first().map(|first| &first.lengths.0) == first_before.as_ref() { "engine" } else { "draw" },
                    kept.first().map_or(plain_spirals, |first| first.spirals.unwrap_or(plain_spirals)),
                    clock.elapsed().as_secs_f64()
                ));
            }
            kept.truncate(1 + MORE_CANDIDATES);
            kept.push(Kept { spirals: Some(plain_spirals), ..Kept::new(plain) });
            kept
        } else {
            vec![Kept::new(plain)]
        };
        if std::env::var_os("OPENCLARE_DEBUG_WORK").is_some() {
            eprintln!(
                "quantization work: {} moves, {} states scanned, cpu {:.2} s; {}",
                report.moves,
                report.scans,
                process_cpu_seconds() - cpu_start,
                work::summary()
            );
        }
        if let Some(directory) = experiment.export.as_deref() {
            export::call(directory, tmesh, &real, &exact, request, &kept);
        }
        let mut kept = kept.into_iter().map(|kept| {
            let (x, scale, quads, lower) = kept.lengths;
            let report = Report {
                turns_held: kept.held,
                spirals: kept.spirals,
                from_selector: kept.from_selector,
                ..report.clone()
            };
            finished(tmesh, &network, &exact, request, &x, scale, quads, &lower, report)
        });
        let first = kept.next().expect("the plain lengths are always kept");
        Ok(Quantization { candidates: kept.collect(), ..first })
    }
}

/// Lengths kept to choose from: their spirals, where they were counted, the
/// turns they held, whether a round of the slow quality mode found them,
/// and the way of closing and the round of cuts that did.
struct Kept {
    lengths: Lengths,
    spirals: Option<f64>,
    held: usize,
    from_selector: bool,
    way: Option<(usize, usize)>,
}

impl Kept {
    fn new(lengths: Lengths) -> Self {
        Self { lengths, spirals: None, held: 0, from_selector: false, way: None }
    }
}

/// Share of the edges of the loops of the quad mesh whole lengths `x` make
/// of `tmesh` that wind as spirals; infinite where the loops cannot be
/// followed.
fn spirals(tmesh: &TMesh, x: &[i64]) -> f64 {
    let lattice = lattice::build(tmesh, x).lattice;
    let mesh = crate::mesh::PolyMesh {
        positions: vec![crate::math::Vec3::ZERO; lattice.sources.len()],
        faces: lattice.quads.iter().map(|quad| quad.to_vec()).collect(),
    };
    crate::metrics::edge_loops(&mesh).map_or(f64::INFINITY, |loops| loops.spiral_share)
}

/// The number of quads lengths closing loops aim at, and how far from it
/// they may be: as near as the `plain` lengths, or within the tolerance.
fn budget(request: &Request, plain: &Lengths) -> (usize, usize) {
    let wanted = request.quads.unwrap_or(plain.2);
    (wanted, plain.2.abs_diff(wanted).max((request.tolerance * wanted as f64) as usize))
}

/// How far whole lengths are from the `real` ones at their scale, in quad
/// edges, on average.
fn deviation((x, scale, ..): &Lengths, real: &[f64]) -> f64 {
    x.iter().zip(real).map(|(&length, real)| (length as f64 - scale * real).abs()).sum::<f64>() / x.len().max(1) as f64
}

/// The most deviation lengths closing loops may have, from the `plain`
/// search's ([`DEVIATION_GROWTH`], [`DEVIATION_SLACK`]); an experiment may
/// lift the cap or its floor.
fn most_deviation(plain: f64, experiment: &Experiment) -> f64 {
    if !experiment.deviation_cap {
        f64::INFINITY
    } else if !experiment.deviation_floor {
        DEVIATION_GROWTH * plain
    } else {
        (DEVIATION_GROWTH * plain).max(plain + DEVIATION_SLACK)
    }
}

/// How far the vertices along the arcs are from the `folds` of the surface
/// they cross, in quad edges, each by how sharp the fold is, all added up.
fn off_folds(folds: &[(usize, f64, f64)], x: &[i64]) -> f64 {
    folds
        .iter()
        .map(|&(arc, share, sharpness)| {
            let at = share * x[arc] as f64;
            sharpness * (at - at.round()).abs()
        })
        .sum()
}

/// The most lengths closing loops may be off the `folds`: as far as the
/// `plain` lengths, and as much more as chance would, half an edge times
/// the square root of the number of folds; an experiment may lift it.
fn most_off_folds(folds: &[(usize, f64, f64)], plain: &[i64], experiment: &Experiment) -> f64 {
    if !experiment.fold_guard {
        return f64::INFINITY;
    }
    let chance = 0.5 * folds.iter().map(|&(.., sharpness)| sharpness * sharpness).sum::<f64>().sqrt();
    off_folds(folds, plain) + chance + 1e-9
}

/// The lengths `x` at `scale`, making `quads`, with the arcs `lower` held
/// from vanishing, and what they achieve added to `report`.
#[allow(clippy::too_many_arguments)]
fn finished(
    tmesh: &TMesh,
    network: &Network,
    exact: &[f64],
    request: &Request,
    x: &[i64],
    scale: f64,
    quads: usize,
    lower: &[i64],
    mut report: Report,
) -> Quantization {
    let arcs = x.len();
    report.scale = scale;
    report.quads = quads;
    report.budget_error = request.quads.map(|wanted| quads.abs_diff(wanted) as f64 / wanted as f64);
    report.balanced = network.is_balanced(x);
    let built = lattice::build(tmesh, x);
    report.separated &= built.defects(tmesh).is_empty() && built.euler() == tmesh.report.euler;
    report.zero_arcs = x.iter().filter(|&&length| length == 0).count();
    report.held_arcs = lower.iter().zip(x).filter(|&(&bound, &length)| bound > 0 && length == bound).count();
    let deviations: Vec<f64> =
        x.iter().zip(exact).map(|(&length, real)| (length as f64 - scale * real).abs()).collect();
    report.deviation_mean = deviations.iter().sum::<f64>() / arcs.max(1) as f64;
    report.deviation_max = deviations.iter().fold(0.0, |a, &b| a.max(b));

    let quantization =
        Quantization { lengths: x.iter().map(|&length| length as u32).collect(), report, candidates: Vec::new() };
    let collapsed = quantization.patch_sizes(tmesh).iter().filter(|size| size[0] == 0 || size[1] == 0).count();
    Quantization { report: Report { collapsed_patches: collapsed, ..quantization.report }, ..quantization }
}

/// Other draws of the per-arc preference closed where the engine's own
/// lengths still spiral, each one as long as the engine's closing: 12 lower
/// the spiral share of the batch from 5.85 % to 4.31 % and the male body's
/// from 1.65 % to 1.46 %, for four times the mean time (DT-089).
const OTHER_DRAWS: usize = 12;
/// Lengths kept besides those with the fewest spirals, to mesh instead if
/// those fold over (before the plain lengths, always kept last).
const MORE_CANDIDATES: usize = 2;
/// A way round that comes back more than this beside itself, in quad edges,
/// and no more than a share of its way round (beyond, it goes round
/// singularities rather than a part), is held to come back onto itself.
const TURN_SHIFT: f64 = 0.05;
/// The ways of closing them tried: the places along the sides of the
/// rectangles the ways round are looked for from, that share, and how many
/// times the side of the T-mesh (the side of a square of its area) a way
/// round may be. A limb whose field twists leans more than a body whose
/// field is merely tilted; on a blobby shape, loops go round the whole of
/// it. Holding more turns helps on some shapes and not on others, where
/// fewer of them can be closed together, so each way is tried.
const CLOSE_WAYS: &[(&[f64], f64, f64)] = &[
    (&[0.5], 0.05, 2.0),
    (&[0.2, 0.5, 0.8], 0.05, 2.0),
    (&[0.5], 0.12, 2.0),
    (&[0.2, 0.5, 0.8], 0.12, 2.0),
    (&[0.5], 0.05, 6.0),
    (&[0.2, 0.5, 0.8], 0.05, 6.0),
    (&[0.5], 0.12, 6.0),
    (&[0.2, 0.5, 0.8], 0.12, 6.0),
];
/// How many times further from the real lengths than the plain search's
/// the lengths closing loops may be: beyond, the quads are stretched and
/// the surface followed less closely for it. On a T-mesh whose plain lengths
/// are already close, a share of so little is too tight: they may always be
/// that much further, in quad edges.
const DEVIATION_GROWTH: f64 = 1.5;
const DEVIATION_SLACK: f64 = 0.2;
/// The lean given to the search for turns still shifted, per unit of shift.
const REPAIR_LEAN: f64 = 0.5;
/// Most walks judged in the repair: each costs building the quad mesh.
const REPAIR_WALKS: usize = 40;
/// The lean given to the search for one turn held and still shifted, per
/// unit of its shift, and the most walks tried for it.
const AIMED_LEAN: f64 = 3.0;
const AIMED_WALKS: usize = 8;
/// Rounds of closing by cuts after each way: every strip of the quad mesh
/// walked, those the real lengths would close held as well.
const CUT_ROUNDS: usize = 3;
/// How far beside itself, as a share of its way round, the real lengths may
/// bring a strip back for it to be held as a ring.
const CUT_LEAN: f64 = 0.12;
/// How many times the side of the quad mesh (the side of a square of its
/// quads) a strip walked for the cuts may go before it is given up.
const CUT_LONGEST: f64 = 6.0;
/// Fold of the surface, in degrees, from which an arc crossing it is drawn
/// to put a vertex on it, and from which it is drawn the most (as a crease
/// gets its weight; how much is [`flow::FOLD_PULL`]).
const FOLD_SOFT: f64 = 30.0;
const FOLD_SHARP: f64 = 70.0;
/// Most the cost of the lengths may grow per unit of shift taken away.
const REPAIR_PRICE: f64 = 2.0;

/// Processor time used by the whole process so far, in seconds, where the
/// system tells it (for measuring the work done, whatever the load).
fn process_cpu_seconds() -> f64 {
    let Ok(stat) = std::fs::read_to_string("/proc/self/stat") else { return 0.0 };
    // Fields after the name in parentheses: user and system time are the
    // 12th and 13th, in clock ticks.
    let fields: Vec<&str> = stat.rsplit(')').next().unwrap_or("").split_whitespace().collect();
    let ticks = |k: usize| fields.get(k).and_then(|field| field.parse::<f64>().ok()).unwrap_or(0.0);
    (ticks(11) + ticks(12)) / 100.0
}

/// The folds of the surface the arcs of `tmesh` cross: the arc, how far
/// along it, as a share of its length, and how sharp the fold is, from 0 to
/// 1 (as for a crease).
fn arc_folds(tmesh: &TMesh) -> Vec<(usize, f64, f64)> {
    let mut folds = Vec::new();
    for (index, arc) in tmesh.arcs.iter().enumerate() {
        let length = arc.span[1] - arc.span[0];
        if length <= 0.0 {
            continue;
        }
        for &(time, turn) in &tmesh.traces[arc.trace as usize].folds {
            if time > arc.span[0] && time < arc.span[1] {
                let sharpness = ((turn - FOLD_SOFT) / (FOLD_SHARP - FOLD_SOFT)).clamp(0.0, 1.0);
                folds.push((index, (time - arc.span[0]) / length, sharpness));
            }
        }
    }
    folds
}

/// The strip network of `tmesh`: one station per pair of opposite sides of
/// each patch, each arc tied to the side it lies on in the patch on its
/// left and in the patch on its right.
fn strip_network(tmesh: &TMesh) -> Network {
    let mut ends = vec![[None; 2]; tmesh.arcs.len()];
    for (index, patch) in tmesh.patches.iter().enumerate() {
        let Some(side_of) = patch.side_of_each() else { continue };
        for (&half, &side) in patch.boundary.iter().zip(&side_of) {
            let station = 2 * index as u32 + side as u32 % 2;
            ends[half as usize / 2][half as usize % 2] = Some(2 * station + side as u32 / 2);
        }
    }
    Network::new(ends)
}

fn count_quads(tmesh: &TMesh, x: &[i64]) -> usize {
    tmesh
        .patches
        .iter()
        .map(|patch| {
            let mut size = [0i64; 2];
            if let Some(side_of) = patch.side_of_each() {
                for (&half, &side) in patch.boundary.iter().zip(&side_of) {
                    if side < 2 {
                        size[side as usize] += x[half as usize / 2];
                    }
                }
            }
            (size[0] * size[1]) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::primitives;
    use crate::{crossfield, preprocess, tmesh};

    /// The T-mesh of a cube with quads of side 0.1.
    fn cube() -> TMesh {
        layout(&primitives::cube(8), Some(30.0), 0.0, 0.1)
    }

    /// The T-mesh of `source`, its field aligned by `alignment`, with quads
    /// of side `edge_length`.
    fn layout(source: &crate::TriMesh, crease_angle: Option<f64>, alignment: f64, edge_length: f64) -> TMesh {
        let request = preprocess::Request { triangles: 8000, crease_angle, ..Default::default() };
        let work = preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap();
        let field = crossfield::create(crossfield::Kind::Power)
            .solve(&work, &crossfield::Request { alignment, ..Default::default() })
            .unwrap();
        let request = tmesh::Request { edge_length };
        tmesh::create(tmesh::Kind::Motorcycle).build(&work, &field, &request).unwrap().tmesh
    }

    /// A tube open at both ends, of radius 1 and height 2, `around` by
    /// `along` squares cut in two.
    fn open_tube(around: u32, along: u32) -> crate::TriMesh {
        let mut mesh = crate::TriMesh::default();
        for j in 0..=along {
            for i in 0..around {
                let angle = std::f64::consts::TAU * i as f64 / around as f64;
                let z = 2.0 * j as f64 / along as f64 - 1.0;
                mesh.positions.push(crate::math::Vec3::new(angle.cos(), angle.sin(), z));
            }
        }
        for j in 0..along {
            for i in 0..around {
                let (a, b) = (j * around + i, j * around + (i + 1) % around);
                mesh.triangles.push([a, b, b + around]);
                mesh.triangles.push([a, b + around, a + around]);
            }
        }
        mesh
    }

    /// Every way round of the quad mesh that `quantized` makes of `tmesh`,
    /// and the lengths, as numbers.
    fn strips(tmesh: &TMesh, quantized: &Quantization) -> (Vec<tubes::Turn>, Vec<f64>) {
        let lengths: Vec<f64> = quantized.lengths.iter().map(|&length| length as f64).collect();
        (tubes::lattice_turns(tmesh, &lengths, 4, 1e9), lengths)
    }

    fn quantized(tmesh: &TMesh, quads: Option<usize>) -> Quantization {
        let request = Request { quads, ..Default::default() };
        WalkCancelling::default().quantize(tmesh, &request).unwrap()
    }

    /// Whether some way round of these, shift and length along added up, is
    /// odd: a ring of an odd number of quads, or the like.
    fn some_odd(tmesh: &TMesh, turns: &[tubes::Turn], lengths: &[f64]) -> bool {
        !tubes::broken_by(turns, &vec![true; tmesh.patches.len()], lengths, false).is_empty()
    }

    #[test]
    fn spheres_are_the_closed_pieces_of_genus_zero() {
        let cube = cube();
        assert!(tubes::on_spheres(&cube).iter().all(|&sphere| sphere));
        assert!(tubes::on_spheres(&twice(&cube)).iter().all(|&sphere| sphere), "two cubes are two spheres");
        let ball = layout(&primitives::icosphere(4), None, 0.0, 0.15);
        assert!(tubes::on_spheres(&ball).iter().all(|&sphere| sphere));
        // Genus one, or with a border: none.
        let torus = layout(&primitives::torus(1.0, 0.4, 96, 48), None, 0.0, 0.1);
        assert_eq!(torus.report.euler, 0);
        assert!(tubes::on_spheres(&torus).iter().all(|&sphere| !sphere));
        let tube = layout(&open_tube(48, 16), None, 0.0, 0.1);
        assert!(tube.arcs.iter().any(|arc| arc.border));
        assert!(tubes::on_spheres(&tube).iter().all(|&sphere| !sphere));
        let sheet = layout(&primitives::plane(24), None, 0.0, 0.1);
        assert!(tubes::on_spheres(&sheet).iter().all(|&sphere| !sphere));
    }

    #[test]
    fn on_a_sphere_every_way_round_is_even() {
        // The research note, Proposition "Parity of a turn on a sphere": at
        // whole lengths balancing every rectangle, the shift of a turn and
        // its length along add up to an even number; a ring has an even
        // number of quads round it.
        for (tmesh, budgets) in
            [(cube(), [300, 301, 1000]), (layout(&primitives::icosphere(4), None, 0.0, 0.15), [300, 333, 1000])]
        {
            let sphere = tubes::on_spheres(&tmesh);
            let real: Vec<f64> = tmesh.arcs.iter().map(|arc| arc.length).collect();
            let found = tubes::turns(&tmesh, 4, &[0.2, 0.5, 0.8], 1e9);
            assert!(!found.is_empty());
            for quads in budgets {
                let quantized = quantized(&tmesh, Some(quads));
                for candidate in std::iter::once(&quantized).chain(&quantized.candidates) {
                    let (walked, lengths) = strips(&tmesh, candidate);
                    assert!(!walked.is_empty());
                    assert_eq!(tubes::broken_by(&walked, &sphere, &lengths, true), Vec::new(), "{quads} quads");
                    // The turns found at the real lengths, taken at the whole ones.
                    assert_eq!(tubes::broken_by(&found, &sphere, &lengths, false), Vec::new(), "{quads} quads");
                }
            }
            // The coefficients are whole, and the length along is the way
            // round at the real lengths, which nearly balance.
            for turn in &found {
                assert!(turn.shift.iter().chain(&turn.along).all(|&(_, c)| c.fract() == 0.0));
                let along = tubes::value(&turn.along, &real).abs();
                assert!((along - turn.around).abs() < 1e-3 * turn.around.max(1.0), "{along} for {}", turn.around);
            }
        }
    }

    #[test]
    fn the_parity_of_ways_round_is_not_asked_of_a_torus_nor_of_an_open_tube() {
        // A torus of one grid has rings of as many quads as the grid is wide,
        // an open tube rings of as many as it is round: odd for some sizes.
        // The check is skipped there, as it must be: it would break.
        let torus = layout(&primitives::torus(1.0, 0.4, 96, 48), None, 0.05, 0.1);
        let tube = layout(&open_tube(48, 16), None, 0.0, 0.1);
        for tmesh in [torus, tube] {
            let sphere = tubes::on_spheres(&tmesh);
            assert!(sphere.iter().all(|&sphere| !sphere));
            let mut odd = false;
            for quads in [200, 230, 260, 290, 320] {
                let (walked, lengths) = strips(&tmesh, &quantized(&tmesh, Some(quads)));
                assert_eq!(tubes::broken_by(&walked, &sphere, &lengths, true), Vec::new(), "{quads} quads");
                odd |= some_odd(&tmesh, &walked, &lengths);
            }
            assert!(odd, "no ring of an odd number of quads at any size tried");
        }
    }

    /// The T-mesh of the largest piece of a model, for about `quads` quads,
    /// and whether that piece is closed.
    fn model(source: &crate::TriMesh, quads: usize) -> (TMesh, bool) {
        let mut pieces = source.pieces();
        pieces.sort_by_key(|(piece, _)| std::cmp::Reverse(piece.triangles.len()));
        let piece = &pieces[0].0;
        let closed = crate::mesh::Topology::build(piece).is_closed();
        let request = preprocess::Request { triangles: 30000, crease_angle: None, ..Default::default() };
        let work = preprocess::create(preprocess::Kind::Isotropic).preprocess(piece, &request).unwrap();
        let field = crossfield::create(crossfield::Kind::Power)
            .solve(&work, &crossfield::Request { alignment: 0.05, ..Default::default() })
            .unwrap();
        let request = tmesh::Request { edge_length: (piece.surface_area() / quads as f64).sqrt() };
        (tmesh::create(tmesh::Kind::Motorcycle).build(&work, &field, &request).unwrap().tmesh, closed)
    }

    /// What is proven of turns, checked on the model `source` at a budget
    /// of the batch: the parity where its largest piece is a sphere, the
    /// length along everywhere. Whether some way round was odd.
    fn check_model(source: &crate::TriMesh) -> (bool, bool) {
        let (tmesh, closed) = model(source, 3000);
        let sphere = tubes::on_spheres(&tmesh);
        assert!(sphere.iter().all(|&on| on == sphere[0]));
        let found = tubes::turns(&tmesh, 4, &[0.2, 0.5, 0.8], 1e9);
        let mut odd = false;
        for quads in [3000] {
            let quantized = quantized(&tmesh, Some(quads));
            for candidate in std::iter::once(&quantized).chain(&quantized.candidates) {
                let (walked, lengths) = strips(&tmesh, candidate);
                assert_eq!(tubes::broken_by(&walked, &sphere, &lengths, true), Vec::new(), "{quads} quads");
                assert_eq!(tubes::broken_by(&found, &sphere, &lengths, false), Vec::new(), "{quads} quads");
                odd |= some_odd(&tmesh, &walked, &lengths) || some_odd(&tmesh, &found, &lengths);
            }
        }
        assert_eq!(sphere[0], closed && tmesh.report.euler == 2);
        (sphere[0], odd)
    }

    #[test]
    fn the_ogre_is_a_sphere_and_its_ways_round_are_even() {
        let source = crate::mesh::obj::read_obj(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/ogre.obj")).unwrap();
        let (sphere, odd) = check_model(&source);
        assert!(sphere && !odd);
    }

    #[test]
    #[ignore = "needs the models of the bench: bench/out/<model>/source.obj, or OPENCLARE_BENCH_OUT"]
    fn models_with_borders_are_not_asked_for_the_parity() {
        let out = std::env::var("OPENCLARE_BENCH_OUT")
            .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../../bench/out").to_string());
        for name in ["suzanne", "jerry"] {
            let source = crate::mesh::obj::read_obj(format!("{out}/{name}/source.obj")).unwrap();
            let (sphere, _) = check_model(&source);
            assert!(!sphere, "{name} has borders");
        }
    }

    #[test]
    fn the_balances_keep_the_handshake() {
        // The research note, Lemma "Handshake": each arc is on one side of
        // a rectangle per half on a rim, so its column has entries +1 or -1,
        // two of them on a closed surface: the balances of whole lengths add
        // up to an even number. On a border an arc has one.
        let stations = |tmesh: &TMesh| -> Vec<tubes::Expression> {
            let weight = vec![1.0; tmesh.arcs.len()];
            let real: Vec<f64> = tmesh.arcs.iter().map(|arc| arc.length).collect();
            assert!(tubes::Closing::new(tmesh, &real, &weight, 0.0).is_some());
            let mut rows = Vec::new();
            for patch in &tmesh.patches {
                let side_of = patch.side_of_each().unwrap();
                for pair in 0..2 {
                    rows.push(
                        patch
                            .boundary
                            .iter()
                            .zip(&side_of)
                            .filter(|&(_, &side)| side as usize % 2 == pair)
                            .map(|(&half, &side)| (half as usize / 2, if side < 2 { 1.0 } else { -1.0 }))
                            .collect(),
                    );
                }
            }
            rows
        };
        let cube = cube();
        let rows = stations(&cube);
        assert_eq!(tubes::unbalanced_columns(&cube, &rows), Vec::<usize>::new());
        for seed in 0..20u64 {
            let mut rng = Rng::new(seed);
            let x: Vec<f64> = (0..cube.arcs.len()).map(|_| (rng.next_f64() * 20.0).floor() - 10.0).collect();
            let total: f64 = rows.iter().map(|row| tubes::value(row, &x)).sum();
            assert_eq!(total.rem_euclid(2.0), 0.0);
        }
        let tube = layout(&open_tube(48, 16), None, 0.0, 0.1);
        let rows = stations(&tube);
        assert_eq!(tubes::unbalanced_columns(&tube, &rows), Vec::<usize>::new());
        // An arc on the border alone: one entry, and an odd total.
        let border = tube.arcs.iter().position(|arc| arc.border).unwrap();
        let mut x = vec![0.0; tube.arcs.len()];
        x[border] = 1.0;
        let total: f64 = rows.iter().map(|row| tubes::value(row, &x)).sum();
        assert_eq!(total.rem_euclid(2.0), 1.0);
    }

    /// Two copies of `one` side by side in one T-mesh, exactly alike.
    fn twice(one: &TMesh) -> TMesh {
        let (nodes, arcs, patches, traces) =
            (one.nodes.len() as u32, one.arcs.len() as u32, one.patches.len() as u32, one.traces.len() as u32);
        let mut out = one.clone();
        out.nodes.extend(
            one.nodes.iter().map(|node| tmesh::Node {
                arcs: node.arcs.iter().map(|half| half + 2 * arcs).collect(),
                ..node.clone()
            }),
        );
        out.arcs.extend(one.arcs.iter().map(|arc| tmesh::Arc {
            nodes: arc.nodes.map(|node| node + nodes),
            patches: arc.patches.map(|patch| patch + patches),
            trace: arc.trace + traces,
            ..arc.clone()
        }));
        out.patches.extend(one.patches.iter().map(|patch| tmesh::Patch {
            boundary: patch.boundary.iter().map(|half| half + 2 * arcs).collect(),
            ..patch.clone()
        }));
        out.traces.extend(one.traces.iter().cloned());
        out
    }

    /// A selector that writes down the problems it is given, and chooses
    /// the lengths `answer` makes of each.
    struct Recording {
        problems: std::sync::Mutex<Vec<RingProblem>>,
        answer: fn(&RingProblem) -> Option<Vec<i64>>,
    }

    impl RingSelector for Recording {
        fn name(&self) -> &str {
            "recording"
        }

        fn select(&self, problem: &RingProblem) -> Option<Vec<i64>> {
            self.problems.lock().unwrap().push(problem.clone());
            (self.answer)(problem)
        }
    }

    fn slow(answer: fn(&RingProblem) -> Option<Vec<i64>>) -> (std::sync::Arc<Recording>, Slow) {
        let recording = std::sync::Arc::new(Recording { problems: Default::default(), answer });
        let selector = Selector(recording.clone());
        (recording, Slow { selector, time_limit: std::time::Duration::from_secs(1), work_limit: None })
    }

    /// The T-mesh of a sphere, whose loops the engine closes, some of them.
    fn ball() -> TMesh {
        layout(&primitives::icosphere(4), None, 0.0, 0.15)
    }

    /// The T-mesh of a thick torus, whose loops the engine leaves winding
    /// as spirals, some of them, at most numbers of quads.
    fn ring() -> TMesh {
        layout(&primitives::torus(1.0, 0.5, 64, 32), None, 0.0, 0.1)
    }

    #[test]
    fn the_walk_of_every_strip_holds_at_the_lengths_it_was_walked_at() {
        // Each condition of a walk says the strip crosses an arc half a quad
        // edge or more from either end: the strips of whole lengths cross at
        // the middles of quad edges, so they all hold, with room to spare.
        for tmesh in [cube(), ball()] {
            let quantized = quantized(&tmesh, Some(600));
            let lengths: Vec<f64> = quantized.lengths.iter().map(|&length| length as f64).collect();
            let walks = tubes::lattice_walks(&tmesh, &lengths, 4, 1e9);
            assert!(!walks.is_empty());
            assert_eq!(walks.len(), tubes::lattice_turns(&tmesh, &lengths, 4, 1e9).len());
            for (strip, conditions) in &walks {
                assert_eq!(conditions.len() % 2, 0, "two conditions per crossing");
                for (row, constant) in conditions {
                    assert!(tubes::value(row, &lengths) + constant >= -1e-9, "{strip:?}");
                }
            }
        }
    }

    #[test]
    fn without_a_selector_the_slow_quality_changes_nothing() {
        let tmesh = ball();
        for quads in [400, 600] {
            let standard = quantized(&tmesh, Some(quads));
            let mute =
                Slow { selector: Selector::default(), time_limit: std::time::Duration::from_secs(1), work_limit: None };
            let request = Request { quads: Some(quads), slow: Some(mute), ..Default::default() };
            let slow = WalkCancelling::default().quantize(&tmesh, &request).unwrap();
            assert_eq!(slow.lengths, standard.lengths);
            assert_eq!(slow.report, standard.report);
            let lengths = |q: &Quantization| q.candidates.iter().map(|c| c.lengths.clone()).collect::<Vec<_>>();
            assert_eq!(lengths(&slow), lengths(&standard));
        }
    }

    #[test]
    fn other_draws_are_posed_where_the_selector_would_be_and_never_spiral_more() {
        for (tmesh, quads) in [(cube(), 300), (ball(), 400), (ring(), 400)] {
            let without = Experiment { extra_draws: Some(0), ..Experiment::default() };
            let request = Request { quads: Some(quads), experiment: without, ..Default::default() };
            let standard = WalkCancelling::default().quantize(&tmesh, &request).unwrap();
            let experiment = Experiment { extra_draws: Some(3), ..Experiment::default() };
            let request = Request { quads: Some(quads), experiment, ..Default::default() };
            let drawn = WalkCancelling::default().quantize(&tmesh, &request).unwrap();
            assert!(drawn.report.is_valid(), "{quads} quads");
            if standard.report.spirals == Some(0.0) {
                assert_eq!(drawn.lengths, standard.lengths, "{quads} quads");
                assert_eq!(drawn.report, standard.report, "{quads} quads");
            } else {
                let spirals = |q: &Quantization| q.report.spirals.unwrap_or(f64::INFINITY);
                assert!(spirals(&drawn) <= spirals(&standard), "{quads} quads");
            }
        }
    }

    #[test]
    fn where_the_engine_leaves_no_spiral_the_selector_is_not_asked() {
        let mut without_spirals = 0;
        let mut with_spirals = 0;
        for (tmesh, quads) in [(cube(), 300), (ball(), 400), (ball(), 600), (ring(), 400)] {
            let standard = quantized(&tmesh, Some(quads));
            let (recording, same) = slow(|problem| Some(problem.reference.clone()));
            let request = Request { quads: Some(quads), slow: Some(same), ..Default::default() };
            let quantized = WalkCancelling::default().quantize(&tmesh, &request).unwrap();
            let asked = recording.problems.lock().unwrap().len();
            if standard.report.spirals == Some(0.0) {
                without_spirals += 1;
                assert_eq!(asked, 0, "{quads} quads");
                assert_eq!(quantized.lengths, standard.lengths);
                assert_eq!(quantized.report, standard.report);
            } else {
                with_spirals += 1;
                assert!(asked > 0, "{quads} quads");
            }
        }
        assert!(
            without_spirals > 0 && with_spirals > 0,
            "{without_spirals} cases without spirals, {with_spirals} with"
        );
    }

    #[test]
    fn the_selector_is_asked_from_the_engine_lengths_and_what_it_gives_is_kept_only_if_it_spirals_less() {
        // Where the engine leaves spirals: elsewhere the selector is not
        // asked.
        let tmesh = ring();
        let quads = 400;
        let standard = quantized(&tmesh, Some(quads));
        assert!(standard.report.spirals.is_some_and(|spirals| spirals > 0.0));
        // A selector that finds nothing: the rounds stay on the engine's
        // lengths, which the problem admits, and nothing changes.
        let (recording, nothing) = slow(|_| None);
        let request = Request { quads: Some(quads), slow: Some(nothing), ..Default::default() };
        let quantized = WalkCancelling::default().quantize(&tmesh, &request).unwrap();
        assert_eq!(quantized.lengths, standard.lengths);
        let problems = recording.problems.lock().unwrap();
        // Each round then starts where the one before ended, with no new
        // ring: the same problem, asked only once.
        assert_eq!(problems.len(), 1);
        let first = &problems[0];
        assert_eq!(first.reference, standard.lengths.iter().map(|&length| length as i64).collect::<Vec<_>>());
        assert!(first.admits(&first.reference));
        assert_eq!(first.count_quads(&first.reference), standard.report.quads);
        assert!(!first.rings.is_empty());
        // Walked at those lengths, every ring keeps its walk there; those
        // they bring back onto themselves are closed.
        let reference: Vec<f64> = first.reference.iter().map(|&length| length as f64).collect();
        for ring in &first.rings {
            assert!(ring.walk.iter().all(|(row, constant)| tubes::value(row, &reference) + constant >= -1e-9));
            assert_eq!(ring.closed_by(&first.reference), tubes::value(&ring.shift, &reference).abs() < 0.5);
        }
        // A selector whose lengths leave a rectangle unbalanced is not
        // listened to.
        let (_, unbalanced) = slow(|problem| {
            let mut x = problem.reference.clone();
            x[0] += 1;
            Some(x)
        });
        let request = Request { quads: Some(quads), slow: Some(unbalanced), ..Default::default() };
        let quantized = WalkCancelling::default().quantize(&tmesh, &request).unwrap();
        assert_eq!(quantized.lengths, standard.lengths);
        assert!(!quantized.report.from_selector);
        assert!(quantized.candidates.iter().all(|candidate| !candidate.report.from_selector));
    }

    #[test]
    fn two_rings_alike_are_both_held() {
        let one = cube();
        let both = twice(&one);
        // Lengths a little off, the same on both copies, for the belts
        // round each cube to come back beside themselves by as much.
        let real: Vec<f64> = both
            .arcs
            .iter()
            .enumerate()
            .map(|(arc, a)| a.length * (1.0 + 0.02 * (Rng::new((arc % one.arcs.len()) as u64).next_f64() - 0.5)))
            .collect();
        let (_, held) = turns_of(&both, &real, &[0.5], 0.05, 6.0);
        let first = held.iter().filter(|turn| turn.patch < one.patches.len()).count();
        let second = held.len() - first;
        assert!(first > 0, "no belt round the cube held");
        assert_eq!(first, second, "the belts of one cube crowd out those of the other");
        // The same belt from another face, or the other way round, is the
        // same ring: three belts round a cube.
        assert_eq!(first, 3);
    }
}
