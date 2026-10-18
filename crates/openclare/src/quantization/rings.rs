//! Rings chosen by a solver from outside: the slow quality mode.
//!
//! The engine closes the strips of quads that go round a part by walks on
//! the strip network, a few at a time. Which of them to close together is a
//! hard choice (choosing the turns is NP-hard, the research note); an exact
//! solver given time closes more of those that keep their walk. The core
//! only poses the problem and hands it to a [`RingSelector`]: it depends on
//! no solver, and [`NoRingSelector`], the default, chooses nothing. A
//! selector built on a solver lives in a crate of its own
//! (`openclare-highs`), and the caller injects it through
//! [`crate::config::RemeshConfig::ring_selector`].
//!
//! The lengths the solver gives are not taken as they are: a few rounds,
//! each solved again with the strips the lengths of the round before make,
//! give lengths that are added to those the engine chooses among by its
//! own count of spirals. The fewest spirals are kept, so the mode is never
//! worse than the engine by that count.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use super::tubes;
use super::{CUT_LEAN, CUT_LONGEST, Lengths, count_quads, lattice};
use crate::tmesh::TMesh;

/// A strip of quads to bring back onto itself: a ring.
#[derive(Clone, Debug, PartialEq)]
pub struct Ring {
    /// How far beside itself the strip comes back, in the lengths of the
    /// arcs, as `(arc, coefficient)`: closed where it is nothing.
    pub shift: Vec<(usize, f64)>,
    /// What closing it is worth: its length round, in quad edges.
    pub weight: f64,
    /// What keeps it the same strip, crossing after crossing: each
    /// `(expression, constant)` asks `expression . x + constant >= 0` of the
    /// lengths `x`. A ring counts as closed only where they all hold.
    pub walk: Vec<(Vec<(usize, f64)>, f64)>,
}

/// Whole lengths for the arcs of a T-mesh, to be chosen by a
/// [`RingSelector`]. Every expression is a list of `(arc, coefficient)`.
#[derive(Clone, Debug, PartialEq)]
pub struct RingProblem {
    /// Rows that whole lengths bring to nothing: the opposite sides of
    /// every rectangle add up to the same.
    pub balances: Vec<Vec<(usize, f64)>>,
    /// Per rectangle, the arcs of its first side and of its second: it has
    /// as many quads as the product of their sums.
    pub sides: Vec<[Vec<usize>; 2]>,
    /// What each whole length aims for: the real length at the scale.
    pub target: Vec<f64>,
    /// Lengths to start from and stay near: the engine's, or those of the
    /// round before. They meet every condition but perhaps the closing of
    /// rings.
    pub reference: Vec<i64>,
    /// The least and the most length of each arc.
    pub lower: Vec<i64>,
    pub upper: Vec<i64>,
    /// Most mean distance between the whole lengths and their targets.
    pub most_deviation: f64,
    /// Quads asked for, and how far from that number is close enough.
    pub quads: usize,
    pub quads_allowed: usize,
    pub rings: Vec<Ring>,
    /// Time each solve may take.
    pub time_limit: Duration,
}

impl RingProblem {
    /// Quads that the lengths `x` make.
    pub fn count_quads(&self, x: &[i64]) -> usize {
        self.sides
            .iter()
            .map(|[first, second]| {
                let sum = |arcs: &Vec<usize>| arcs.iter().map(|&arc| x[arc]).sum::<i64>();
                (sum(first) * sum(second)).max(0) as usize
            })
            .sum()
    }

    /// Whether `x` balances every rectangle, keeps within the bounds, the
    /// deviation and the number of quads.
    pub fn admits(&self, x: &[i64]) -> bool {
        let value = |row: &Vec<(usize, f64)>| row.iter().map(|&(arc, c)| c * x[arc] as f64).sum::<f64>();
        let arcs = self.target.len();
        let deviation: f64 = x.iter().zip(&self.target).map(|(&length, target)| (length as f64 - target).abs()).sum();
        x.len() == arcs
            && self.balances.iter().all(|row| value(row).abs() < 0.5)
            && x.iter()
                .zip(self.lower.iter().zip(&self.upper))
                .all(|(length, (low, high))| low <= length && length <= high)
            && deviation <= self.most_deviation * arcs as f64 + 1e-9
            && self.count_quads(x).abs_diff(self.quads) <= self.quads_allowed
    }
}

impl Ring {
    /// Whether the lengths `x` close this ring, keeping its walk.
    pub fn closed_by(&self, x: &[i64]) -> bool {
        let value = |row: &Vec<(usize, f64)>| row.iter().map(|&(arc, c)| c * x[arc] as f64).sum::<f64>();
        value(&self.shift).abs() < 0.5 && self.walk.iter().all(|(row, constant)| value(row) + constant >= -1e-9)
    }
}

/// Chooses whole lengths that close rings: the interface the slow quality
/// mode calls, implemented outside the core.
pub trait RingSelector: Send + Sync {
    /// A name, for reports and for telling selectors apart.
    fn name(&self) -> &str;

    /// Whether it ever chooses anything: when not, the rounds are not even
    /// posed.
    fn is_mute(&self) -> bool {
        false
    }

    /// Whole lengths that `problem` admits ([`RingProblem::admits`]),
    /// closing as much weight of its rings as can be found in the time
    /// given (each counted only where its walk holds), then, with those
    /// closed, as near their targets as can be, and near the reference.
    /// `None` when none was found.
    fn select(&self, problem: &RingProblem) -> Option<Vec<i64>>;
}

/// The selector that chooses nothing: the engine's own lengths stay.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoRingSelector;

impl RingSelector for NoRingSelector {
    fn name(&self) -> &str {
        "none"
    }

    fn is_mute(&self) -> bool {
        true
    }

    fn select(&self, _: &RingProblem) -> Option<Vec<i64>> {
        None
    }
}

/// A selector handed to the engine. Two are equal when they have the same
/// name, so that a configuration holding one can still be compared.
#[derive(Clone)]
pub struct Selector(pub Arc<dyn RingSelector>);

impl Selector {
    pub fn new(selector: impl RingSelector + 'static) -> Self {
        Self(Arc::new(selector))
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self::new(NoRingSelector)
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Selector({})", self.0.name())
    }
}

impl PartialEq for Selector {
    fn eq(&self, other: &Self) -> bool {
        self.0.name() == other.0.name()
    }
}

/// What the slow quality mode asks of the quantization: the selector and
/// the time each of its solves may take.
#[derive(Clone, Debug, PartialEq)]
pub struct Slow {
    pub selector: Selector,
    pub time_limit: Duration,
}

/// Rounds of rings chosen by the selector, after the lengths the engine
/// kept.
pub(super) const ROUNDS: usize = 3;
/// Times the lengths of a round are solved again with the arcs the quad
/// mesh they stand for pinched held open.
const TRIES: usize = 6;
/// How far from its target, in quad edges, a length may go to close rings,
/// besides as far as the reference already is.
const SLACK: f64 = 4.0;

/// Lengths of the rounds of the slow quality mode, from the lengths `start`
/// the engine kept: in each, the selector closes rings among the strips of
/// quads the real lengths would close (leaning less than [`CUT_LEAN`]) that
/// the lengths of every round so far make, within the balances, the bounds,
/// the deviation `most_deviation` and the quads `wanted` give or take
/// `allowed`. Each round's lengths, with the arcs held open for them, and
/// the number of rings they close keeping their walk.
#[allow(clippy::too_many_arguments)]
pub(super) fn rounds(
    tmesh: &TMesh,
    real: &[f64],
    start: &Lengths,
    wanted: usize,
    allowed: usize,
    most_deviation: f64,
    slow: &Slow,
) -> Vec<(Vec<i64>, Vec<i64>, usize)> {
    let debug = std::env::var_os("OPENCLARE_DEBUG_TUBES").is_some();
    let (x, scale, _, lower) = start;
    let target: Vec<f64> = real.iter().map(|length| scale * length).collect();
    let mut balances = Vec::new();
    let mut sides = Vec::new();
    for patch in &tmesh.patches {
        let Some(side_of) = patch.side_of_each() else { continue };
        let mut first_two = [Vec::new(), Vec::new()];
        for pair in 0..2 {
            let mut row = Vec::new();
            for (&half, &side) in patch.boundary.iter().zip(&side_of) {
                if side as usize % 2 == pair {
                    row.push((half as usize / 2, if side < 2 { 1.0 } else { -1.0 }));
                }
                if side as usize == pair {
                    first_two[pair].push(half as usize / 2);
                }
            }
            balances.push(row);
        }
        sides.push(first_two);
    }
    // The rings, each once whichever way round, with the walk it had where
    // it was first found.
    let mut rings: Vec<Ring> = Vec::new();
    let mut known = std::collections::HashSet::new();
    let mut add_strips = |rings: &mut Vec<Ring>, x: &[i64]| {
        let lengths: Vec<f64> = x.iter().map(|&length| length as f64).collect();
        let side = (count_quads(tmesh, x) as f64).sqrt().max(1.0);
        for (strip, walk) in tubes::lattice_walks(tmesh, &lengths, 4, CUT_LONGEST * side) {
            let field = scale * tubes::value(&strip.shift, real);
            if field.abs() > CUT_LEAN * strip.around || !known.insert(super::ring(&strip.shift)) {
                continue;
            }
            rings.push(Ring { shift: strip.shift, weight: strip.around, walk });
        }
    };
    add_strips(&mut rings, x);
    let mut reference = x.clone();
    let mut lower: Vec<i64> = lower.iter().map(|&bound| bound.max(0)).collect();
    let mut out = Vec::new();
    for round in 0..ROUNDS {
        let mut tries = 0;
        let chosen = loop {
            tries += 1;
            let (low, high): (Vec<i64>, Vec<i64>) = target
                .iter()
                .zip(&reference)
                .zip(&lower)
                .map(|((&target, &reference), &bound)| {
                    let low = bound.max(((target.floor() - SLACK) as i64).min(reference)).max(0);
                    (low, ((target.ceil() + SLACK) as i64).max(reference).max(low))
                })
                .unzip();
            let problem = RingProblem {
                balances: balances.clone(),
                sides: sides.clone(),
                target: target.clone(),
                reference: reference.clone(),
                lower: low,
                upper: high,
                most_deviation,
                quads: wanted,
                quads_allowed: allowed,
                rings: rings.clone(),
                time_limit: slow.time_limit,
            };
            let clock = std::time::Instant::now();
            let chosen = slow.selector.0.select(&problem).filter(|x| problem.admits(x));
            if debug {
                let closed = chosen.as_ref().map(|x| rings.iter().filter(|ring| ring.closed_by(x)).count());
                eprintln!(
                    "  slow round {round} try {tries}: {} rings, {} closed by the selector, {:.1} s",
                    rings.len(),
                    closed.map_or("none".to_string(), |closed| closed.to_string()),
                    clock.elapsed().as_secs_f64()
                );
            }
            // No lengths found: those of the round before stay.
            let chosen = chosen.unwrap_or_else(|| reference.clone());
            let defects = lattice::build(tmesh, &chosen).defects(tmesh);
            if defects.is_empty() || tries >= TRIES {
                break chosen;
            }
            for arc in defects.reopen {
                lower[arc] = lower[arc].max(1);
            }
        };
        let closed = rings.iter().filter(|ring| ring.closed_by(&chosen)).count();
        out.push((chosen.clone(), lower.clone(), closed));
        add_strips(&mut rings, &chosen);
        reference = chosen;
    }
    out
}
