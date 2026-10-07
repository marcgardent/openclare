//! Whole numbers on the arcs of a strip network, as close to given targets
//! as the network allows.
//!
//! The network is that of the quad strips to come. A strip crosses a
//! rectangle from one side to the opposite one, so the numbers on the arcs
//! of two opposite sides must add up to the same: a *station* is one such
//! pair of sides, and each arc is tied at either end to one side of a
//! station (or to nothing, on an open border, where strips simply end).
//! Strips have no direction, which makes this a flow in a *bidirected*
//! graph: the problem Heistermann et al. 2023 call minimum-deviation flow.
//!
//! This solver is the approximate one. It starts from a balanced assignment
//! and improves it by walks along which arcs are alternately raised and
//! lowered so that every station stays balanced: through a station from one
//! side to the other, two arcs move the same way; turning back on the same
//! side, they move opposite ways. A closed walk that lowers the cost is
//! applied, until none is found. Walks are searched as negative cycles in a
//! graph of (arc, raise or lower, end reached), with the cost of each move
//! taken alone, so a walk that needs the same arc twice can be misjudged:
//! the result is a local optimum, usually the global one on small cases.

/// Which side of which station an arc end is tied to: `2 * station + side`.
pub(super) type Side = u32;

pub(super) struct Network {
    /// Per arc, the side each of its two ends is tied to; `None` for an end
    /// on an open border.
    ends: Vec<[Option<Side>; 2]>,
    /// Per side, the arc ends tied to it.
    tied: Vec<Vec<(u32, u8)>>,
    /// Arc ends tied to nothing.
    loose: Vec<(u32, u8)>,
}

/// What an assignment costs: per arc, `weight * (x - target)^2`, plus a
/// prohibitive price for every unit below `lower`.
pub(super) struct Cost<'a> {
    pub target: &'a [f64],
    pub weight: &'a [f64],
    pub lower: &'a [i64],
}

/// Price of one unit below a lower bound: more than any walk could save.
const SHORTFALL: f64 = 1.0e7;

/// Cost differences below this are rounding noise.
const NOISE: f64 = 1.0e-9;

impl Cost<'_> {
    pub fn of(&self, arc: usize, x: i64) -> f64 {
        if x < 0 {
            return f64::INFINITY;
        }
        let off = x as f64 - self.target[arc];
        let missing = (self.lower[arc] - x).max(0) as f64;
        self.weight[arc] * off * off + SHORTFALL * missing * missing
    }

    #[cfg(test)]
    pub fn total(&self, x: &[i64]) -> f64 {
        x.iter().enumerate().map(|(arc, &value)| self.of(arc, value)).sum()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Effort {
    /// Walks applied.
    pub moves: usize,
    /// Walks found that turned out not to lower the cost.
    pub misjudged: usize,
}

impl Network {
    pub fn new(ends: Vec<[Option<Side>; 2]>) -> Self {
        let sides = ends.iter().flatten().flatten().map(|&side| side as usize + 1).max().unwrap_or(0);
        // Sides come in pairs: make room for the partner of the last one.
        let mut tied = vec![Vec::new(); sides + sides % 2];
        let mut loose = Vec::new();
        for (arc, pair) in ends.iter().enumerate() {
            for (end, side) in pair.iter().enumerate() {
                match side {
                    Some(side) => tied[*side as usize].push((arc as u32, end as u8)),
                    None => loose.push((arc as u32, end as u8)),
                }
            }
        }
        Self { ends, tied, loose }
    }

    pub fn arcs(&self) -> usize {
        self.ends.len()
    }

    /// Whether the two sides of every station carry the same total.
    pub fn is_balanced(&self, x: &[i64]) -> bool {
        self.tied.chunks(2).all(|pair| {
            let total = |side: &Vec<(u32, u8)>| side.iter().map(|&(arc, _)| x[arc as usize]).sum::<i64>();
            total(&pair[0]) == total(&pair[1])
        })
    }

    /// Improves the balanced assignment `x` until no walk lowers its cost.
    pub fn improve(&self, x: &mut [i64], cost: &Cost) -> Effort {
        let mut effort = Effort::default();
        let largest = cost.target.iter().fold(1.0f64, |a, &b| a.max(b));
        // Big steps first: most of the way is covered in few moves.
        let mut step = 1i64 << (largest.log2().floor() as u32).min(40);
        while step >= 1 {
            self.improve_by(x, cost, step, &mut effort);
            step /= 2;
        }
        effort
    }

    /// The state of a walk: it has just moved `arc` (up if `raise`) and
    /// stands at its end `end`.
    fn state(arc: u32, raise: bool, end: u8) -> usize {
        4 * arc as usize + 2 * raise as usize + end as usize
    }

    fn improve_by(&self, x: &mut [i64], cost: &Cost, step: i64, effort: &mut Effort) {
        let hub = 4 * self.arcs();
        // Moves that were tried in a walk and found wanting, set aside so
        // that the search does not come back to the same walk forever.
        let mut barred = vec![false; hub + 1];
        loop {
            // What it costs to enter each state: the change on its arc.
            let mut entry = vec![0.0; hub + 1];
            for arc in 0..self.arcs() {
                let now = cost.of(arc, x[arc]);
                for raise in [false, true] {
                    let moved = cost.of(arc, x[arc] + if raise { step } else { -step }) - now;
                    for end in 0..2 {
                        let state = Self::state(arc as u32, raise, end);
                        entry[state] = if barred[state] { f64::INFINITY } else { moved };
                    }
                }
            }
            let Some(walk) = self.negative_walk(&entry) else { return };

            let mut change = vec![0i64; self.arcs()];
            for &state in walk.iter().filter(|&&state| state != hub) {
                change[state / 4] += if state / 2 % 2 == 1 { step } else { -step };
            }
            let saved: f64 = walk
                .iter()
                .filter(|&&state| state != hub)
                .map(|&state| state / 4)
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .map(|arc| cost.of(arc, x[arc] + change[arc]) - cost.of(arc, x[arc]))
                .sum();
            if saved < -NOISE {
                for (arc, delta) in change.into_iter().enumerate() {
                    x[arc] += delta;
                }
                effort.moves += 1;
            } else {
                // Judged on single moves, it looked better than it is. Bar
                // its most expensive move and look again.
                let worst =
                    walk.iter().copied().filter(|&state| state != hub).max_by(|&a, &b| entry[a].total_cmp(&entry[b]));
                effort.misjudged += 1;
                match worst {
                    Some(state) => barred[state] = true,
                    None => return,
                }
            }
        }
    }

    /// The states reachable in one move from `state`, each one a way of
    /// keeping the station just reached balanced.
    fn for_each_next(&self, state: usize, mut visit: impl FnMut(usize)) {
        let hub = 4 * self.arcs();
        if state == hub {
            // From outside the surface, a strip may come in at any border.
            for &(arc, end) in &self.loose {
                visit(Self::state(arc, false, 1 - end));
                visit(Self::state(arc, true, 1 - end));
            }
            return;
        }
        let (arc, raise, end) = ((state / 4) as u32, state / 2 % 2 == 1, (state % 2) as u8);
        let Some(side) = self.ends[arc as usize][end as usize] else {
            visit(hub);
            return;
        };
        // On through the station: the arc taken on the far side moves the
        // same way. Back on the same side: it moves the other way.
        for (through, raise) in [(side ^ 1, raise), (side, !raise)] {
            for &(next, from) in &self.tied[through as usize] {
                if (next, from) != (arc, end) {
                    visit(Self::state(next, raise, 1 - from));
                }
            }
        }
    }

    /// A closed walk whose entry costs add up to less than zero, if the
    /// search finds one (Bellman-Ford from everywhere at once).
    fn negative_walk(&self, entry: &[f64]) -> Option<Vec<usize>> {
        let states = entry.len();
        let mut distance = vec![0.0f64; states];
        let mut previous = vec![usize::MAX; states];
        let mut active: Vec<usize> = (0..states).collect();
        let mut queued = vec![false; states];
        for _ in 0..=states {
            let mut next_active = Vec::new();
            for &state in &active {
                let base = distance[state];
                self.for_each_next(state, |next| {
                    let candidate = base + entry[next];
                    if candidate < distance[next] - NOISE {
                        distance[next] = candidate;
                        previous[next] = state;
                        if !queued[next] {
                            queued[next] = true;
                            next_active.push(next);
                        }
                    }
                });
            }
            if next_active.is_empty() {
                return None;
            }
            for &state in &next_active {
                queued[state] = false;
            }
            if let Some(walk) = cycle_among(&previous, &next_active) {
                return Some(walk);
            }
            active = next_active;
        }
        None
    }
}

/// A cycle in the `previous` links, looked for from the states in `from`.
fn cycle_among(previous: &[usize], from: &[usize]) -> Option<Vec<usize>> {
    // 0: not seen; otherwise the number of the search that reached it.
    let mut seen = vec![0u32; previous.len()];
    for (search, &start) in from.iter().enumerate() {
        let search = search as u32 + 1;
        let mut state = start;
        while state != usize::MAX && seen[state] == 0 {
            seen[state] = search;
            state = previous[state];
        }
        if state != usize::MAX && seen[state] == search {
            // Came back onto this very search: `state` is on a cycle.
            let mut walk = vec![state];
            let mut at = previous[state];
            while at != state {
                walk.push(at);
                at = previous[at];
            }
            return Some(walk);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Rng;

    /// Best balanced assignment with every arc in `0..=top`, by trying all.
    fn brute_force(network: &Network, cost: &Cost, top: i64) -> f64 {
        let arcs = network.arcs();
        let mut x = vec![0i64; arcs];
        let mut best = f64::INFINITY;
        loop {
            if network.is_balanced(&x) {
                best = best.min(cost.total(&x));
            }
            let mut at = 0;
            while at < arcs && x[at] == top {
                x[at] = 0;
                at += 1;
            }
            if at == arcs {
                return best;
            }
            x[at] += 1;
        }
    }

    fn solve(network: &Network, cost: &Cost) -> Vec<i64> {
        let mut x = vec![0; network.arcs()];
        network.improve(&mut x, cost);
        assert!(network.is_balanced(&x));
        x
    }

    #[test]
    fn a_ring_of_stations_takes_one_number_all_round() {
        // Three stations in a ring: arc i runs from the far side of station
        // i to the near side of station i + 1.
        let network = Network::new(vec![[Some(1), Some(2)], [Some(3), Some(4)], [Some(5), Some(0)]]);
        let (target, weight, lower) = ([2.6, 3.4, 2.9], [1.0; 3], [0; 3]);
        let x = solve(&network, &Cost { target: &target, weight: &weight, lower: &lower });
        assert_eq!(x, [3, 3, 3]);
    }

    #[test]
    fn a_lower_bound_is_met_whatever_the_targets_say() {
        let network = Network::new(vec![[Some(1), Some(2)], [Some(3), Some(4)], [Some(5), Some(0)]]);
        let (target, weight) = ([0.2, 0.1, 0.3], [1.0; 3]);
        let free = solve(&network, &Cost { target: &target, weight: &weight, lower: &[0; 3] });
        assert_eq!(free, [0, 0, 0]);
        let held = solve(&network, &Cost { target: &target, weight: &weight, lower: &[0, 1, 0] });
        assert_eq!(held, [1, 1, 1]);
    }

    #[test]
    fn strips_end_on_open_borders() {
        // One station between two border arcs, as across a sheet: the two
        // arcs carry the same number, anything else being free.
        let network = Network::new(vec![[None, Some(0)], [Some(1), None]]);
        let (target, weight, lower) = ([4.4, 4.9], [1.0; 2], [0; 2]);
        let x = solve(&network, &Cost { target: &target, weight: &weight, lower: &lower });
        assert_eq!(x, [5, 5]);
    }

    #[test]
    fn a_side_split_in_two_shares_out_its_total() {
        // One side of the station carries arc 0, the other arcs 1 and 2
        // (a T-junction); all three end on borders.
        let network = Network::new(vec![[None, Some(0)], [Some(1), None], [Some(1), None]]);
        let (target, weight, lower) = ([5.2, 1.7, 3.5], [1.0; 3], [0; 3]);
        let x = solve(&network, &Cost { target: &target, weight: &weight, lower: &lower });
        assert_eq!(x[0], x[1] + x[2]);
        assert_eq!(x, [5, 2, 3]);
    }

    #[test]
    fn small_random_networks_are_solved_to_the_optimum() {
        let mut rng = Rng::new(7);
        let (mut solved, mut total) = (0, 0);
        for _ in 0..60 {
            // Three stations, seven arcs tied at random, some to a border.
            let stations = 3;
            let ends: Vec<[Option<Side>; 2]> = (0..7)
                .map(|_| {
                    std::array::from_fn(|_| {
                        let pick = (rng.next_f64() * (2 * stations + 1) as f64) as u32;
                        (pick < 2 * stations).then_some(pick)
                    })
                })
                .collect();
            let network = Network::new(ends);
            let target: Vec<f64> = (0..7).map(|_| 3.0 * rng.next_f64()).collect();
            let weight: Vec<f64> = (0..7).map(|_| 0.5 + rng.next_f64()).collect();
            let lower = vec![0; 7];
            let cost = Cost { target: &target, weight: &weight, lower: &lower };
            let x = solve(&network, &cost);
            let best = brute_force(&network, &cost, 5);
            total += 1;
            if cost.total(&x) <= best + 1e-9 {
                solved += 1;
            }
        }
        // A local search: nearly always the optimum, not provably always.
        println!("optimum found on {solved} of {total} networks");
        assert!(solved * 10 >= total * 9, "{solved} of {total}");
    }
}
