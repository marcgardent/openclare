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

mod flow;
mod lattice;
mod tubes;

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::math::Rng;
use crate::tmesh::TMesh;
use flow::{Cost, Network};
pub use lattice::{Lattice, Source};
use rayon::prelude::*;

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
}

impl Default for Request {
    fn default() -> Self {
        Self { quads: None, tolerance: 0.02, close_turns: true }
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
    let mut same_shift = std::collections::HashSet::new();
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
        if shift.abs() > TURN_SHIFT && same_shift.insert((shift.abs() * 1e4).round() as i64) {
            held.push(turn.clone());
        }
        every.push(turn);
    }
    (every, held)
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
        let sound = within(x, &moved) && lattice::build(tmesh, &moved).defects(tmesh).is_empty();
        if std::env::var_os("OPENCLARE_DEBUG_REPAIR").is_some() {
            let size = change.iter().filter(|&&d| d != 0).count();
            eprintln!("    walk of {size} arcs: shift {now:.1} -> {then:.1}, cost {cost:.2}, sound {sound}");
        }
        then < now - 0.5 && cost < REPAIR_PRICE * (now - then) && sound
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
        let mut bias = vec![0.0; target.len()];
        for &(arc, c) in &turn.shift {
            bias[arc] += AIMED_LEAN * left.signum() * c;
        }
        let leant = Cost { target, weight: &weight, lower, bias: &bias, folds: pull };
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
    /// factorized once for all.
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
    ) -> Lengths {
        let debug = std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some();
        let clock = std::time::Instant::now();
        let mut held: Vec<&tubes::Turn> = held.iter().collect();
        let mut rows = tubes::Rows::new(closing);
        let mut active: Vec<usize> = held.iter().map(|turn| rows.push(turn.shift.clone())).collect();
        let mut aimed = real.to_vec();
        let mut projections = 0;
        while !held.is_empty() {
            projections += 1;
            let Some(closed) = rows.closed(&active, &vec![0.0; held.len()], !cut) else { break };
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
        let target: Vec<f64> = aimed.iter().map(|length| scale * length).collect();
        let (budget, aimed_at): (_, &[&tubes::Turn]) = if cut {
            let wanted = request.quads.unwrap_or_else(|| count_quads(tmesh, &x));
            (Some((wanted, (request.tolerance * wanted as f64) as usize)), &[])
        } else {
            (None, &held)
        };
        repair(tmesh, network, pull, &target, &lower, &mut x, every, aimed_at, budget, report);
        if std::env::var_os("OPENCLARE_DEBUG_TURNS").is_some() {
            debug_turns(tmesh, every, &x, real, scale, &held);
        }
        let quads = count_quads(tmesh, &x);
        (x, scale, quads, lower)
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
            let (x, scale, quads, _) = &current;
            let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
            let side = (*quads as f64).sqrt().max(1.0);
            let mut undue = 0;
            for strip in tubes::lattice_turns(tmesh, &lengths, 4, CUT_LONGEST * side) {
                // How far the real lengths bring it back beside itself, in
                // quad edges: beyond the lean, it winds round as the field
                // does, and is not held.
                let field = scale * tubes::value(&strip.shift, real);
                if field.abs() > CUT_LEAN * strip.around {
                    continue;
                }
                if tubes::value(&strip.shift, &lengths).abs() >= 0.5 && known_held.insert(key(&strip)) {
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
        // Arcs of equal length (a cube has nothing else) would all gain or
        // lose an edge at the same scale, and the number of quads would jump
        // by whole layers. A slight, fixed preference per arc spreads them out.
        // (Experiment: another draw of the preferences, for a control.)
        let seed: u64 = std::env::var("OPENCLARE_JITTER_SEED")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map_or(0, |seed| seed.wrapping_mul(1_000_003));
        let real: Vec<f64> = exact
            .iter()
            .enumerate()
            .map(|(arc, length)| length * (1.0 + self.jitter * (Rng::new(arc as u64 + seed).next_f64() - 0.5)))
            .collect();

        let mut report = Report { arcs, scale: 1.0, separated: true, ..Default::default() };
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
        let (x, scale, quads, lower) = if close {
            let chosen = 'close: {
                let spirals_of = |x: &[i64]| -> f64 {
                    let lattice = lattice::build(tmesh, x).lattice;
                    let mesh = crate::mesh::PolyMesh {
                        positions: vec![crate::math::Vec3::ZERO; lattice.sources.len()],
                        faces: lattice.quads.iter().map(|quad| quad.to_vec()).collect(),
                    };
                    crate::metrics::edge_loops(&mesh).map_or(f64::INFINITY, |loops| loops.spiral_share)
                };
                // As near the number of quads asked for as the plain search, or
                // within the tolerance: fewer spirals do not buy a different mesh.
                let wanted = request.quads.unwrap_or(plain.2);
                let allowed = plain.2.abs_diff(wanted).max((request.tolerance * wanted as f64) as usize);
                // How far whole lengths are from the real ones at their scale,
                // in quad edges, on average.
                let deviation = |(x, scale, ..): &(Vec<i64>, f64, usize, Vec<i64>)| -> f64 {
                    x.iter().zip(&real).map(|(&length, real)| (length as f64 - scale * real).abs()).sum::<f64>()
                        / x.len().max(1) as f64
                };
                if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                    eprintln!("plain lengths: deviation {:.3}", deviation(&plain));
                }
                let most_deviation = (DEVIATION_GROWTH * deviation(&plain)).max(deviation(&plain) + DEVIATION_SLACK);
                // How far the vertices along the arcs are from the folds of the
                // surface they cross, in quad edges, each by how sharp the fold
                // is, all added up. Closing loops may not take them further off
                // than the plain lengths left them, by more than chance would:
                // half an edge times the square root of the number of folds.
                let folds = arc_folds(tmesh);
                let off_folds = |x: &[i64]| -> f64 {
                    folds
                        .iter()
                        .map(|&(arc, share, sharpness)| {
                            let at = share * x[arc] as f64;
                            sharpness * (at - at.round()).abs()
                        })
                        .sum()
                };
                let chance = 0.5 * folds.iter().map(|&(.., sharpness)| sharpness * sharpness).sum::<f64>().sqrt();
                let most_off_folds = off_folds(&plain.0) + chance + 1e-9;
                if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                    eprintln!("plain lengths: {} folds crossed, off them by {:.2}", folds.len(), off_folds(&plain.0));
                }
                // (Experiment, not for merging: the system the lengths kept stand
                // in, written out for a selection worked out outside the engine,
                // and whole lengths read back from it, taken as they are.)
                // Pieces are quantized in parallel: each T-mesh is named by its size
                // rather than by the order of the calls.
                let _ = EXPERIMENT_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let call =
                    format!("{}-{}-{}", arcs, tmesh.patches.len(), (exact.iter().sum::<f64>() * 1000.0).round() as i64);
                let export = |chosen: &Lengths| {
                    let Some(directory) = std::env::var_os("OPENCLARE_EXPORT_TURNS") else { return };
                    let parts = ExportParts {
                        real: &real,
                        exact: &exact,
                        plain: &plain,
                        wanted,
                        allowed,
                        plain_deviation: deviation(&plain),
                        deviation: deviation(chosen),
                        most_deviation,
                        off_folds: off_folds(&chosen.0),
                        most_off_folds,
                        folds: &folds,
                        spirals: spirals_of(&chosen.0),
                    };
                    export_system(std::path::Path::new(&directory), &call, tmesh, chosen, &parts);
                };
                if let Some(directory) = std::env::var_os("OPENCLARE_LENGTHS")
                    && let Ok(text) =
                        std::fs::read_to_string(std::path::Path::new(&directory).join(format!("call-{call}.txt")))
                {
                    let mut lines = text.lines();
                    let scale: f64 = lines.next().and_then(|line| line.trim().parse().ok()).unwrap_or(plain.1);
                    let x: Vec<i64> = lines
                        .flat_map(|line| line.split_whitespace())
                        .filter_map(|v| v.parse::<f64>().ok())
                        .map(|v| v.round() as i64)
                        .collect();
                    if x.len() == arcs {
                        let quads = count_quads(tmesh, &x);
                        let imported = (x, scale, quads, plain.3.clone());
                        report.turns_held = 1;
                        export(&imported);
                        break 'close imported;
                    }
                }
                let mut best = (spirals_of(&plain.0), plain.clone());
                // Lengths that leave the structure broken are not mended by
                // closing loops, and each way would cost as long a search again.
                let sound = best.0.is_finite() && lattice::build(tmesh, &best.1.0).defects(tmesh).is_empty();
                // The least change of the real lengths that closes turns, its
                // part for the rectangles solved once for every way.
                let weight: Vec<f64> = real.iter().map(|length| 1.0 / (length + 1.0)).collect();
                let closing = if sound { tubes::Closing::new(tmesh, &real, &weight) } else { None };
                // The ways, each with the turns it holds; the same turns held
                // twice would give the same lengths.
                let mut ways = Vec::new();
                let mut tried_held: Vec<Vec<Vec<(usize, i64)>>> = Vec::new();
                for &(offsets, lean, longest) in CLOSE_WAYS.iter().filter(|_| closing.is_some()) {
                    let clock = std::time::Instant::now();
                    let (found, held) = turns_of(tmesh, &real, offsets, lean, longest);
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
                    ways.push(((offsets, lean, longest), found, held));
                }
                // Each way searched on a core of its own, then, if its lengths
                // could be kept, closed by cuts a few rounds; the one kept is the
                // same as if they had been searched in turn.
                let closing = closing.as_ref();
                let tried: Vec<_> = ways
                    .par_iter()
                    .map(|(_, found, held)| {
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
                        );
                        // Only lengths that could be kept are refined by cuts.
                        let admissible = tried.2.abs_diff(wanted) <= allowed && deviation(&tried) <= most_deviation;
                        let cut = if admissible {
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
                            )
                        } else {
                            Vec::new()
                        };
                        let candidates: Vec<_> = std::iter::once((tried, held.len()))
                            .chain(cut)
                            .map(|(tried, held)| (spirals_of(&tried.0), tried, held))
                            .collect();
                        (candidates, own.moves, own.scans)
                    })
                    .collect();
                for (((offsets, lean, longest), ..), (candidates, moves, scans)) in ways.iter().zip(tried) {
                    report.moves += moves;
                    report.scans += scans;
                    for (round, (spirals, tried, held)) in candidates.into_iter().enumerate() {
                        if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                            eprintln!(
                                "closing from {offsets:?} leaning up to {lean}, {longest} long, round {round}: spirals {:.3} deviation {:.3} quads {} off folds {:.2} (best so far {:.3})",
                                spirals,
                                deviation(&tried),
                                tried.2,
                                off_folds(&tried.0),
                                best.0
                            );
                        }
                        if spirals < best.0 - 1e-9
                            && tried.2.abs_diff(wanted) <= allowed
                            && deviation(&tried) <= most_deviation
                            && off_folds(&tried.0) <= most_off_folds
                        {
                            report.turns_held = held;
                            best = (spirals, tried);
                        }
                    }
                }
                export(&best.1);
                best.1
            };
            chosen
        } else {
            plain
        };
        if std::env::var_os("OPENCLARE_DEBUG_WORK").is_some() {
            eprintln!(
                "quantization work: {} moves, {} states scanned, cpu {:.2} s",
                report.moves,
                report.scans,
                process_cpu_seconds() - cpu_start
            );
        }
        report.scale = scale;
        report.quads = quads;
        report.budget_error = request.quads.map(|wanted| quads.abs_diff(wanted) as f64 / wanted as f64);
        report.balanced = network.is_balanced(&x);
        let built = lattice::build(tmesh, &x);
        report.separated &= built.defects(tmesh).is_empty() && built.euler() == tmesh.report.euler;
        report.zero_arcs = x.iter().filter(|&&length| length == 0).count();
        report.held_arcs = lower.iter().zip(&x).filter(|&(&bound, &length)| bound > 0 && length == bound).count();
        let deviations: Vec<f64> =
            x.iter().zip(&exact).map(|(&length, real)| (length as f64 - scale * real).abs()).collect();
        report.deviation_mean = deviations.iter().sum::<f64>() / arcs.max(1) as f64;
        report.deviation_max = deviations.iter().fold(0.0, |a, &b| a.max(b));

        let quantization = Quantization { lengths: x.iter().map(|&length| length as u32).collect(), report };
        let collapsed = quantization.patch_sizes(tmesh).iter().filter(|size| size[0] == 0 || size[1] == 0).count();
        Ok(Quantization { report: Report { collapsed_patches: collapsed, ..quantization.report }, ..quantization })
    }
}

/// (Experiment.) Calls of the quantizer with loops to close, numbered for
/// the files written and read outside the engine.
static EXPERIMENT_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// (Experiment.) What the engine knows about the lengths it chose from.
struct ExportParts<'a> {
    real: &'a [f64],
    exact: &'a [f64],
    plain: &'a Lengths,
    wanted: usize,
    allowed: usize,
    plain_deviation: f64,
    deviation: f64,
    most_deviation: f64,
    off_folds: f64,
    most_off_folds: f64,
    folds: &'a [(usize, f64, f64)],
    spirals: f64,
}

/// (Experiment, not for merging.) Writes the system the whole lengths
/// `chosen` stand in, for a selection of rings worked out outside the
/// engine: the real lengths, the balances of the patches, the bounds the
/// engine keeps to, the arcs the lattice of `chosen` asks to reopen, and
/// every strip of quads `chosen` makes with its shift expression, its pitch
/// at `chosen` and the field's shift at the scale.
fn export_system(directory: &std::path::Path, call: &str, tmesh: &TMesh, chosen: &Lengths, parts: &ExportParts) {
    use std::fmt::Write;
    let (x, scale, quads, lower) = chosen;
    let expression = |row: &[(usize, f64)]| {
        let terms: Vec<String> = row.iter().map(|&(arc, c)| format!("[{arc},{c}]")).collect();
        format!("[{}]", terms.join(","))
    };
    let mut out = String::new();
    write!(
        out,
        "{{\"call\":\"{call}\",\"scale\":{scale},\"quads\":{quads},\"wanted\":{},\"allowed\":{},\"plain_quads\":{},\"plain_scale\":{},\"plain_deviation\":{},\"deviation\":{},\"most_deviation\":{},\"off_folds\":{},\"most_off_folds\":{},\"spirals\":{},",
        parts.wanted,
        parts.allowed,
        parts.plain.2,
        parts.plain.1,
        parts.plain_deviation,
        parts.deviation,
        parts.most_deviation,
        parts.off_folds,
        parts.most_off_folds,
        if parts.spirals.is_finite() { parts.spirals } else { -1.0 }
    )
    .unwrap();
    write!(
        out,
        "\"real\":{:?},\"exact\":{:?},\"lengths\":{x:?},\"plain_lengths\":{:?},\"lower\":{lower:?},\"plain_lower\":{:?},",
        parts.real, parts.exact, parts.plain.0, parts.plain.3
    )
    .unwrap();
    let folds: Vec<String> =
        parts.folds.iter().map(|&(arc, share, sharp)| format!("[{arc},{share},{sharp}]")).collect();
    write!(out, "\"folds\":[{}],", folds.join(",")).unwrap();
    let defects = lattice::build(tmesh, x).defects(tmesh);
    write!(out, "\"reopen\":{:?},\"unexplained\":{},\"stations\":[", defects.reopen, defects.unexplained).unwrap();
    let mut first = true;
    for patch in &tmesh.patches {
        let Some(side_of) = patch.side_of_each() else { continue };
        for pair in 0..2 {
            let mut row: Vec<(usize, f64)> = Vec::new();
            for (&half, &side) in patch.boundary.iter().zip(&side_of) {
                if side as usize % 2 == pair {
                    row.push((half as usize / 2, if side < 2 { 1.0 } else { -1.0 }));
                }
            }
            if !first {
                out.push(',');
            }
            first = false;
            out.push_str(&expression(&row));
        }
    }
    out.push_str("],\"strips\":[");
    let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
    let side = (*quads as f64).sqrt().max(1.0);
    for (k, (strip, conditions)) in tubes::lattice_walks(tmesh, &lengths, 4, CUT_LONGEST * side).iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        let conditions: Vec<String> =
            conditions.iter().map(|(row, constant)| format!("[{},{constant}]", expression(row))).collect();
        write!(
            out,
            "{{\"shift\":{},\"around\":{},\"pitch\":{},\"field\":{},\"conditions\":[{}]}}",
            expression(&strip.shift),
            strip.around,
            tubes::value(&strip.shift, &lengths),
            scale * tubes::value(&strip.shift, parts.real),
            conditions.join(",")
        )
        .unwrap();
    }
    out.push_str("]}");
    let _ = std::fs::create_dir_all(directory);
    let _ = std::fs::write(directory.join(format!("call-{call}.json")), out);
}

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
