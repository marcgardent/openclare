//! The rings of openclare's slow quality mode, chosen by HiGHS.
//!
//! The core poses the problem ([`RingProblem`]) and asks a
//! [`RingSelector`] for whole lengths; this crate is the one that solves
//! it, as a mixed-integer program, with the HiGHS solver. It is a crate of
//! its own so that the core depends on no solver: the command line tool
//! injects it when the slow quality is asked for.
//!
//! The program, for lengths `x` (whole), a switch `z` per ring:
//!
//! - every rectangle balanced, every length within its bounds;
//! - the mean of `|x - target|` at most the deviation allowed;
//! - the number of quads, a quadratic in `x`, within the budget: taken to
//!   first order around the reference, the room narrowed to 80% of what is
//!   allowed, counted exactly after the solve, and linearized again around
//!   the lengths found (with less room) while it is outside;
//! - `z = 1` only where the ring's shift is nothing and its walk holds,
//!   each by a big-M inequality.
//!
//! It is solved twice: first for the most weight of rings closed (ties
//! broken by staying near the targets and the reference), then, with those
//! rings kept closed, for the least deviation. Each solve starts from the
//! lengths of the reference (or of the first solve) where HiGHS can take
//! them, and stops at the time limit with the best found.

use std::ffi::CString;

use highs::{HighsModelStatus, RowProblem, Sense};
use openclare::quantization::{RingProblem, RingSelector};

/// The ring selector that solves the slow quality mode's program with
/// HiGHS.
#[derive(Clone, Debug)]
pub struct HighsSelector {
    /// Weight of the distance to the targets in the first solve, per quad
    /// edge, against a weight of one quad edge round per ring closed.
    pub deviation_weight: f64,
    /// Weight of the distance to the reference lengths, in both solves.
    pub stay_weight: f64,
    /// Weight of the distance to the targets in the second solve.
    pub polish_weight: f64,
    /// Share of the quads allowed that the first-order count may use.
    pub budget_share: f64,
    /// Times the count of quads is linearized again.
    pub attempts: usize,
    /// Relative gap at which a solve is done.
    pub gap: f64,
}

impl Default for HighsSelector {
    fn default() -> Self {
        Self { deviation_weight: 1e-3, stay_weight: 0.1, polish_weight: 1.0, budget_share: 0.8, attempts: 4, gap: 1e-4 }
    }
}

/// What the switches of the rings are in a solve: free, each worth its
/// weight, or fixed.
enum Rings<'a> {
    Weighted,
    Fixed(&'a [bool]),
}

/// A solution: the lengths and the switches of the rings.
struct Found {
    x: Vec<i64>,
    closed: Vec<bool>,
}

impl RingSelector for HighsSelector {
    fn name(&self) -> &str {
        "highs"
    }

    fn select(&self, problem: &RingProblem) -> Option<Vec<i64>> {
        let mut share = self.budget_share;
        let first = self.solve(problem, Rings::Weighted, None, &mut share)?;
        // With the rings found kept closed, the least deviation; the first
        // lengths where it finds none.
        let polished = self.solve(problem, Rings::Fixed(&first.closed), Some(&first), &mut share);
        Some(polished.map_or(first.x, |polished| polished.x))
    }
}

impl HighsSelector {
    /// One solve, linearized again while the quads are outside the budget.
    fn solve(&self, problem: &RingProblem, rings: Rings, start: Option<&Found>, share: &mut f64) -> Option<Found> {
        let mut around = problem.reference.clone();
        for _ in 0..self.attempts.max(1) {
            let found = self.solve_once(problem, &rings, start, &around, *share)?;
            if problem.count_quads(&found.x).abs_diff(problem.quads) <= problem.quads_allowed {
                return Some(found);
            }
            around = found.x;
            *share *= 0.7;
        }
        None
    }

    /// One solve, the quads taken to first order around `around` within
    /// `share` of the room allowed.
    fn solve_once(
        &self,
        problem: &RingProblem,
        rings: &Rings,
        start: Option<&Found>,
        around: &[i64],
        share: f64,
    ) -> Option<Found> {
        let n = problem.target.len();
        let m = problem.rings.len();
        let (deviation_weight, stay_weight) = match rings {
            Rings::Weighted => (self.deviation_weight, self.stay_weight),
            Rings::Fixed(_) => (self.polish_weight, self.stay_weight),
        };
        let (lower, upper) = (&problem.lower, &problem.upper);
        let mut model = RowProblem::default();
        let x: Vec<_> =
            (0..n).map(|arc| model.add_integer_column(0.0, lower[arc] as f64..=upper[arc] as f64)).collect();
        let over: Vec<_> = (0..n).map(|_| model.add_column(deviation_weight, 0.0..)).collect();
        let under: Vec<_> = (0..n).map(|_| model.add_column(deviation_weight, 0.0..)).collect();
        let above: Vec<_> = (0..n).map(|_| model.add_column(stay_weight, 0.0..)).collect();
        let below: Vec<_> = (0..n).map(|_| model.add_column(stay_weight, 0.0..)).collect();
        let z: Vec<_> = problem
            .rings
            .iter()
            .enumerate()
            .map(|(k, ring)| match rings {
                Rings::Weighted => model.add_integer_column(-ring.weight, 0.0..=1.0),
                Rings::Fixed(closed) => {
                    let value = if closed[k] { 1.0 } else { 0.0 };
                    model.add_integer_column(0.0, value..=value)
                }
            })
            .collect();

        for row in &problem.balances {
            model.add_row(0.0..=0.0, row.iter().map(|&(arc, c)| (x[arc], c)));
        }
        for arc in 0..n {
            let target = problem.target[arc];
            model.add_row(target..=target, [(x[arc], 1.0), (over[arc], -1.0), (under[arc], 1.0)]);
            let reference = problem.reference[arc] as f64;
            model.add_row(reference..=reference, [(x[arc], 1.0), (above[arc], -1.0), (below[arc], 1.0)]);
        }
        let most = problem.most_deviation * n as f64;
        model.add_row(..=most, over.iter().chain(&under).map(|&column| (column, 1.0)));
        // Quads to first order around `around`.
        let mut gradient = vec![0.0; n];
        let mut quads = 0.0;
        for [first, second] in &problem.sides {
            let sum = |arcs: &Vec<usize>| arcs.iter().map(|&arc| around[arc] as f64).sum::<f64>();
            let (a, b) = (sum(first), sum(second));
            quads += a * b;
            for &arc in first {
                gradient[arc] += b;
            }
            for &arc in second {
                gradient[arc] += a;
            }
        }
        let base = quads - gradient.iter().zip(around).map(|(g, &length)| g * length as f64).sum::<f64>();
        let room = share * problem.quads_allowed as f64;
        let wanted = problem.quads as f64;
        model.add_row(
            wanted - room - base..=wanted + room - base,
            gradient.iter().enumerate().filter(|&(_, &g)| g != 0.0).map(|(arc, &g)| (x[arc], g)),
        );
        // Least and most of an expression over the bounds of the lengths.
        let least = |row: &[(usize, f64)]| -> f64 {
            row.iter().map(|&(arc, c)| c * if c > 0.0 { lower[arc] } else { upper[arc] } as f64).sum()
        };
        let most_of = |row: &[(usize, f64)]| -> f64 {
            row.iter().map(|&(arc, c)| c * if c > 0.0 { upper[arc] } else { lower[arc] } as f64).sum()
        };
        for (k, ring) in problem.rings.iter().enumerate() {
            // Closed: the shift is nothing where the switch is on.
            let big = most_of(&ring.shift).max(-least(&ring.shift)) + 1.0;
            let terms = || ring.shift.iter().map(|&(arc, c)| (x[arc], c));
            model.add_row(..=big, terms().chain([(z[k], big)]));
            model.add_row(..=big, ring.shift.iter().map(|&(arc, c)| (x[arc], -c)).chain([(z[k], big)]));
            // The same walk: each condition holds where the switch is on.
            for (row, constant) in &ring.walk {
                let slack = (-(least(row) + constant)).max(0.0);
                if slack == 0.0 {
                    // It holds over all the lengths allowed.
                    continue;
                }
                model.add_row(-constant - slack.., row.iter().map(|&(arc, c)| (x[arc], c)).chain([(z[k], -slack)]));
            }
        }

        let mut model = model.try_optimise(Sense::Minimise).ok()?;
        model.make_quiet();
        model.set_option("time_limit", problem.time_limit.as_secs_f64());
        model.set_option("mip_rel_gap", self.gap);
        // A start: the lengths given, with the rings they close switched on.
        let (start_x, start_closed): (Vec<i64>, Vec<bool>) = match (start, rings) {
            (Some(found), _) => (found.x.clone(), found.closed.clone()),
            (None, Rings::Fixed(closed)) => (problem.reference.clone(), closed.to_vec()),
            (None, Rings::Weighted) => {
                let closed = problem.rings.iter().map(|ring| ring.closed_by(&problem.reference)).collect();
                (problem.reference.clone(), closed)
            }
        };
        let mut columns = Vec::with_capacity(5 * n + m);
        columns.extend(start_x.iter().map(|&length| length as f64));
        columns.extend((0..n).map(|arc| (start_x[arc] as f64 - problem.target[arc]).max(0.0)));
        columns.extend((0..n).map(|arc| (problem.target[arc] - start_x[arc] as f64).max(0.0)));
        columns.extend((0..n).map(|arc| (start_x[arc] - problem.reference[arc]).max(0) as f64));
        columns.extend((0..n).map(|arc| (problem.reference[arc] - start_x[arc]).max(0) as f64));
        columns.extend(start_closed.iter().map(|&on| if on { 1.0 } else { 0.0 }));
        // A start HiGHS cannot use is set aside by it, not an error.
        let _ = model.try_set_solution(Some(&columns), None, None, None);
        let solved = model.try_solve().ok()?;
        let usable = matches!(
            solved.status(),
            HighsModelStatus::Optimal
                | HighsModelStatus::ReachedTimeLimit
                | HighsModelStatus::ReachedIterationLimit
                | HighsModelStatus::ObjectiveBound
                | HighsModelStatus::ObjectiveTarget
                | HighsModelStatus::Unknown
        );
        if !usable || !has_solution(&solved) {
            return None;
        }
        let solution = solved.get_solution();
        let columns = solution.columns();
        let x: Vec<i64> = columns[..n].iter().map(|value| value.round() as i64).collect();
        let closed = columns[5 * n..5 * n + m].iter().map(|&value| value > 0.5).collect();
        Some(Found { x, closed })
    }
}

/// Whether HiGHS holds a feasible solution after a solve, whatever stopped
/// it.
fn has_solution(solved: &highs::SolvedModel) -> bool {
    let name = CString::new("primal_solution_status").unwrap();
    let mut status: highs_sys::HighsInt = 0;
    // SAFETY: the pointer is that of the live model; the name is a valid C
    // string; the value is written to a local of the type HiGHS expects.
    let read = unsafe { highs_sys::Highs_getIntInfoValue(solved.as_ptr(), name.as_ptr(), &mut status) };
    // kHighsStatusOk, and kSolutionStatusFeasible.
    read == 0 && status == 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use openclare::quantization::Ring;
    use std::time::Duration;

    /// One rectangle of sides `a` (arcs 0 and 2, opposite) by `b` (arcs 1
    /// and 3): balances a0 = a2 and a1 = a3, and as many quads as a0 * a1.
    fn square(target: [f64; 4], rings: Vec<Ring>) -> RingProblem {
        RingProblem {
            balances: vec![vec![(0, 1.0), (2, -1.0)], vec![(1, 1.0), (3, -1.0)]],
            sides: vec![[vec![0], vec![1]]],
            target: target.to_vec(),
            reference: target.iter().map(|t| t.round() as i64).collect(),
            lower: vec![1; 4],
            upper: vec![20; 4],
            most_deviation: 1.0,
            quads: 30,
            quads_allowed: 8,
            rings,
            time_limit: Duration::from_secs(10),
        }
    }

    #[test]
    fn lengths_found_are_admitted() {
        let problem = square([5.2, 5.8, 5.2, 5.8], Vec::new());
        let x = HighsSelector::default().select(&problem).expect("a solution");
        assert!(problem.admits(&x), "{x:?}");
    }

    #[test]
    fn a_ring_worth_closing_is_closed() {
        // A ring whose shift is a0 - a1: closed by a square of 5 by 5 or 6
        // by 6, which the budget (30 quads, give or take 8) and the
        // deviation allow; the targets alone would make 5 by 6.
        let ring = Ring { shift: vec![(0, 1.0), (1, -1.0)], weight: 10.0, walk: Vec::new() };
        let problem = square([5.2, 5.8, 5.2, 5.8], vec![ring]);
        let x = HighsSelector::default().select(&problem).expect("a solution");
        assert!(problem.admits(&x), "{x:?}");
        assert!(problem.rings[0].closed_by(&x), "{x:?}");
    }

    #[test]
    fn a_ring_whose_walk_cannot_hold_is_not_counted() {
        // Closing asks a0 = a1; its walk asks a0 >= a1 + 1: never both.
        let ring =
            Ring { shift: vec![(0, 1.0), (1, -1.0)], weight: 10.0, walk: vec![(vec![(0, 1.0), (1, -1.0)], -1.0)] };
        let problem = square([5.2, 5.8, 5.2, 5.8], vec![ring]);
        let x = HighsSelector::default().select(&problem).expect("a solution");
        assert!(problem.admits(&x));
        assert!(!problem.rings[0].closed_by(&x));
    }
}
