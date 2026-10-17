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
    /// Improving moves made by the solver, over all its runs.
    pub moves: usize,
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

/// Writes, for analysis outside the engine, the system a way of closing
/// turns works on: the real lengths of the arcs and the whole ones found,
/// the balance of each patch, and every turn found with its shift, whether
/// it was held and whether the projection kept it. One JSON file per way,
/// numbered as the ways are tried.
#[allow(clippy::too_many_arguments)]
fn export_turns(
    directory: &std::path::Path,
    way: usize,
    tmesh: &TMesh,
    real: &[f64],
    scale: f64,
    x: &[i64],
    lower: &[i64],
    every: &[tubes::Turn],
    asked: &[tubes::Turn],
    kept: &[&tubes::Turn],
) {
    use std::fmt::Write;
    let expression = |row: &[(usize, f64)]| {
        let terms: Vec<String> = row.iter().map(|&(arc, c)| format!("[{arc},{c}]")).collect();
        format!("[{}]", terms.join(","))
    };
    let mut out = String::new();
    write!(out, "{{\"scale\":{scale},\"real\":{real:?},\"lengths\":{x:?},\"lower\":{lower:?},\"stations\":[").unwrap();
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
    out.push_str("],\"arcs_at\":[");
    for (k, arc) in tmesh.arcs.iter().enumerate() {
        let (a, b) = (tmesh.nodes[arc.nodes[0] as usize].position, tmesh.nodes[arc.nodes[1] as usize].position);
        let middle = 0.5 * (a + b);
        if k > 0 {
            out.push(',');
        }
        write!(out, "[{},{},{}]", middle.x, middle.y, middle.z).unwrap();
    }
    out.push_str("],\"turns\":[");
    for (k, turn) in every.iter().enumerate() {
        let first_arc = tmesh.patches[turn.patch].boundary[0] as usize / 2;
        let at = tmesh.nodes[tmesh.arcs[first_arc].nodes[0] as usize].position;
        let held = asked.iter().position(|h| h.shift == turn.shift).map_or(-1, |k| k as i64);
        let kept = kept.iter().any(|h| h.shift == turn.shift);
        if k > 0 {
            out.push(',');
        }
        write!(
            out,
            "{{\"shift\":{},\"around\":{},\"patch\":{},\"at\":[{},{},{}],\"held\":{},\"held_index\":{held},\"kept\":{kept}}}",
            expression(&turn.shift),
            turn.around,
            turn.patch,
            at.x,
            at.y,
            at.z,
            held >= 0
        )
        .unwrap();
    }
    out.push_str("]}");
    let _ = std::fs::create_dir_all(directory);
    let _ = std::fs::write(directory.join(format!("way-{way}.json")), out);
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

impl WalkCancelling {
    /// Whole lengths for `tmesh` with the turns `held` brought back onto
    /// themselves: the lengths aimed at changed as little as can be for
    /// that (turns that cannot all be are let go, the worst first), the
    /// search, then walks taken one at a time that shift `every` turn less.
    #[allow(clippy::too_many_arguments)]
    fn closed_search(
        &self,
        tmesh: &TMesh,
        network: &Network,
        real: &[f64],
        request: &Request,
        report: &mut Report,
        every: &[tubes::Turn],
        held: &[tubes::Turn],
        way: usize,
    ) -> (Vec<i64>, f64, usize, Vec<i64>) {
        let weight: Vec<f64> = real.iter().map(|length| 1.0 / (length + 1.0)).collect();
        let debug = std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some();
        let clock = std::time::Instant::now();
        let asked = held;
        let mut held: Vec<&tubes::Turn> = held.iter().collect();
        let mut aimed = real.to_vec();
        let mut projections = 0;
        // (Experiment: the lengths aimed at and the turns closed, worked
        // out outside the engine for this way; see export_turns.)
        if let Some(directory) = std::env::var_os("OPENCLARE_AIMED")
            && let Ok(text) = std::fs::read_to_string(std::path::Path::new(&directory).join(format!("aimed-{way}.txt")))
        {
            let mut lines = text.lines();
            let values: Vec<f64> =
                lines.next().unwrap_or("").split_whitespace().filter_map(|v| v.parse().ok()).collect();
            let closed: Vec<usize> =
                lines.next().unwrap_or("").split_whitespace().filter_map(|v| v.parse().ok()).collect();
            if values.len() == real.len() {
                aimed = values;
                held = closed.into_iter().filter_map(|k| asked.get(k)).collect();
                // Nothing left for the projection to do.
                projections = usize::MAX;
            } else if values.is_empty() {
                // Only the turns: the projection runs on those.
                held = closed.into_iter().filter_map(|k| asked.get(k)).collect();
            }
        }
        while !held.is_empty() && projections != usize::MAX {
            projections += 1;
            let Some(closed) = tubes::closed(tmesh, real, &weight, &held, &vec![0.0; held.len()]) else { break };
            let left: Vec<f64> = held.iter().map(|turn| tubes::value(&turn.shift, &closed).abs()).collect();
            let (worst, at) =
                left.iter().enumerate().fold((0.0, 0), |(w, a), (k, &v)| if v > w { (v, k) } else { (w, a) });
            if worst <= 1e-3 {
                aimed = closed;
                break;
            }
            held.remove(at);
        }
        if debug {
            eprintln!(
                "  projected {projections} times, {} turns kept, {:.1} s",
                held.len(),
                clock.elapsed().as_secs_f64()
            );
        }
        let clock = std::time::Instant::now();
        let (mut x, scale, _, lower) = self.search(tmesh, network, &aimed, request, report);
        // (Experiment: whole lengths worked out outside the engine, taken as
        // they are, with neither search nor repair.)
        if let Some(directory) = std::env::var_os("OPENCLARE_LENGTHS")
            && let Ok(text) = std::fs::read_to_string(std::path::Path::new(&directory).join(format!("way-{way}-integer.txt")))
        {
            let values: Vec<i64> = text.split_whitespace().filter_map(|v| v.parse::<f64>().ok()).map(|v| v.round() as i64).collect();
            if values.len() == x.len() {
                let quads = count_quads(tmesh, &values);
                return (values, scale, quads, lower);
            }
        }
        if debug {
            eprintln!("  searched in {:.1} s", clock.elapsed().as_secs_f64());
        }
        let clock = std::time::Instant::now();

        // One walk at a time, guided by a lean towards less shift, taken
        // only if it shifts the turns less in all, costs little, and leaves
        // the structure intact.
        let target: Vec<f64> = aimed.iter().map(|length| scale * length).collect();
        let weight: Vec<f64> = target.iter().map(|length| 1.0 / (length + 1.0)).collect();
        let left_by = |x: &[i64]| -> Vec<f64> {
            let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
            every.iter().map(|turn| tubes::value(&turn.shift, &lengths)).collect()
        };
        let mut bias = vec![0.0; real.len()];
        for (turn, shift) in every.iter().zip(left_by(&x)) {
            if shift.abs() >= 0.5 {
                for &(arc, c) in &turn.shift {
                    bias[arc] += REPAIR_LEAN * shift.signum() * c;
                }
            }
        }
        let plain = Cost { target: &target, weight: &weight, lower: &lower, bias: &[] };
        let leant = Cost { target: &target, weight: &weight, lower: &lower, bias: &bias };
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
            let sound = lattice::build(tmesh, &moved).defects(tmesh).is_empty();
            if std::env::var_os("OPENCLARE_DEBUG_REPAIR").is_some() {
                let size = change.iter().filter(|&&d| d != 0).count();
                eprintln!("    walk of {size} arcs: shift {now:.1} -> {then:.1}, cost {cost:.2}, sound {sound}");
            }
            then < now - 0.5 && cost < REPAIR_PRICE * (now - then) && sound
        };
        let mut effort = flow::Effort::default();
        network.improve_judged(&mut x, &leant, 1, REPAIR_WALKS, &mut effort, &mut judge);
        report.moves += effort.moves;
        if debug {
            eprintln!(
                "  repaired in {:.1} s: {} walks taken, {} turned down",
                clock.elapsed().as_secs_f64(),
                effort.moves,
                effort.misjudged
            );
        }
        // Then each turn held that is still shifted, one at a time: a lean
        // on its arcs alone, a walk taken if it shifts that turn less and
        // the others not more.
        let clock = std::time::Instant::now();
        let mut effort = flow::Effort::default();
        for turn in &held {
            let shift_of = |x: &[i64]| -> f64 { turn.shift.iter().map(|&(arc, c)| c * x[arc] as f64).sum() };
            let left = shift_of(&x);
            if left.abs() < 0.5 {
                continue;
            }
            let mut bias = vec![0.0; real.len()];
            for &(arc, c) in &turn.shift {
                bias[arc] += AIMED_LEAN * left.signum() * c;
            }
            let leant = Cost { target: &target, weight: &weight, lower: &lower, bias: &bias };
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
                let sound = better && lattice::build(tmesh, &moved).defects(tmesh).is_empty();
                if std::env::var_os("OPENCLARE_DEBUG_REPAIR").is_some() {
                    eprintln!(
                        "    aimed walk: turn {:.0} -> {:.0}, all {now:.0} -> {then:.0}, cost {cost:.2}, sound {sound}",
                        shift_of(x),
                        shift_of(&moved)
                    );
                }
                better && cost < REPAIR_PRICE * (now - then).max(1.0) && sound
            };
            network.improve_judged(&mut x, &leant, 1, AIMED_WALKS, &mut effort, &mut judge);
        }
        report.moves += effort.moves;
        if debug {
            eprintln!(
                "  aimed in {:.1} s: {} walks taken, {} turned down",
                clock.elapsed().as_secs_f64(),
                effort.moves,
                effort.misjudged
            );
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
        if let Some(directory) = std::env::var_os("OPENCLARE_EXPORT_TURNS") {
            export_turns(std::path::Path::new(&directory), way, tmesh, real, scale, &x, &lower, every, asked, &held);
        }
        let quads = count_quads(tmesh, &x);
        (x, scale, quads, lower)
    }

    /// Whole lengths near `real` at the scale that gives the number of quads
    /// asked for, balanced and leaving the structure intact: the lengths,
    /// the scale, the quads they make, and the arcs held from vanishing.
    fn search(
        &self,
        tmesh: &TMesh,
        network: &Network,
        real: &[f64],
        request: &Request,
        report: &mut Report,
    ) -> (Vec<i64>, f64, usize, Vec<i64>) {
        let arcs = real.len();
        let mut x = vec![0i64; arcs];
        let mut lower = vec![0i64; arcs];
        // The best assignment seen: lengths, scale, quads.
        let mut best: Option<(Vec<i64>, f64, usize)> = None;
        let mut scale = 1.0;
        // Scales known to give too few quads and too many.
        let (mut too_small, mut too_large) = (None::<f64>, None::<f64>);
        let rounds = if request.quads.is_some() { self.scale_rounds } else { 1 };
        for _ in 0..rounds {
            let target: Vec<f64> = real.iter().map(|length| scale * length).collect();
            // Relative to the length: one edge more or less matters on a
            // short arc, not on a long one.
            let weight: Vec<f64> = target.iter().map(|length| 1.0 / (length + 1.0)).collect();
            let mut separated = false;
            for _ in 0..self.separation_rounds {
                let cost = Cost { target: &target, weight: &weight, lower: &lower, bias: &[] };
                report.moves += network.improve(&mut x, &cost).moves;
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
        let network = strip_network(tmesh);
        let exact: Vec<f64> = tmesh.arcs.iter().map(|arc| arc.length).collect();
        // Arcs of equal length (a cube has nothing else) would all gain or
        // lose an edge at the same scale, and the number of quads would jump
        // by whole layers. A slight, fixed preference per arc spreads them out.
        let real: Vec<f64> = exact
            .iter()
            .enumerate()
            .map(|(arc, length)| length * (1.0 + self.jitter * (Rng::new(arc as u64).next_f64() - 0.5)))
            .collect();

        let mut report = Report { arcs, scale: 1.0, separated: true, ..Default::default() };
        // Ways round a part that the whole lengths shift wind their loops
        // into helices (DT-062). Lengths closing them are looked for in more
        // than one way; where the spirals end up is sensitive to every
        // length, so each way's quad mesh is built and its spirals counted,
        // and the one with the fewest kept, the plain search among them.
        let close = request.close_turns;
        let clock = std::time::Instant::now();
        let plain = self.search(tmesh, &network, &real, request, &mut report);
        if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
            eprintln!(
                "plain search {:.1} s, {} arcs, {} patches",
                clock.elapsed().as_secs_f64(),
                arcs,
                tmesh.patches.len()
            );
        }
        let (x, scale, quads, lower) = if close {
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
            let most_deviation = DEVIATION_GROWTH * deviation(&plain);
            let mut best = (spirals_of(&plain.0), plain);
            // Lengths that leave the structure broken are not mended by
            // closing loops, and each way would cost as long a search again.
            let sound = best.0.is_finite() && lattice::build(tmesh, &best.1.0).defects(tmesh).is_empty();
            // The ways, each with the turns it holds; the same turns held
            // twice would give the same lengths.
            let mut ways = Vec::new();
            let mut tried_held: Vec<Vec<Vec<(usize, i64)>>> = Vec::new();
            for &(offsets, lean, longest) in CLOSE_WAYS.iter().filter(|_| sound) {
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
            // Each way searched on a core of its own; the one kept is the
            // same as if they had been searched in turn.
            let tried: Vec<_> = ways
                .par_iter()
                .enumerate()
                .map(|(way, (_, found, held))| {
                    let mut own = Report::default();
                    let tried = self.closed_search(tmesh, &network, &real, request, &mut own, found, held, way);
                    (spirals_of(&tried.0), tried, own.moves)
                })
                .collect();
            for (((offsets, lean, longest), _, held), (spirals, tried, moves)) in ways.iter().zip(tried) {
                report.moves += moves;
                if std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some() {
                    eprintln!(
                        "closing from {offsets:?} leaning up to {lean}, {longest} long: spirals {:.3} deviation {:.3} quads {} (best so far {:.3})",
                        spirals,
                        deviation(&tried),
                        tried.2,
                        best.0
                    );
                }
                if spirals < best.0 - 1e-9 && tried.2.abs_diff(wanted) <= allowed && deviation(&tried) <= most_deviation
                {
                    report.turns_held = held.len();
                    best = (spirals, tried);
                }
            }
            best.1
        } else {
            plain
        };
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
/// the surface followed less closely for it.
const DEVIATION_GROWTH: f64 = 1.5;
/// The lean given to the search for turns still shifted, per unit of shift.
const REPAIR_LEAN: f64 = 0.5;
/// Most walks judged in the repair: each costs building the quad mesh.
const REPAIR_WALKS: usize = 40;
/// The lean given to the search for one turn held and still shifted, per
/// unit of its shift, and the most walks tried for it.
const AIMED_LEAN: f64 = 3.0;
const AIMED_WALKS: usize = 8;
/// Most the cost of the lengths may grow per unit of shift taken away.
const REPAIR_PRICE: f64 = 2.0;

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
