//! Turns of a T-mesh: where a straight way through its rectangles goes
//! round a part of the shape and comes back to where it started, and how
//! far beside itself, as a sum of lengths of arcs.
//!
//! Each rectangle has its own coordinates, its first corner at the origin,
//! its sides along them. From one rectangle to the next across an arc, the
//! coordinates turn by quarter turns and shift by what the lengths of the
//! arcs on the rims put between the two first corners. Composed all the way
//! round a turn that comes back going the same way, the turns cancel and the
//! shifts add up: across the way, that is how far the lines of quads round
//! that part come back beside themselves. Not nothing, they wind round as
//! helices (DT-062).

use crate::linalg::SymmetricBuilder;
use crate::tmesh::{HalfArc, TMesh};

/// A linear expression in the lengths of the arcs: `(arc, coefficient)`.
pub(super) type Expression = Vec<(usize, f64)>;

/// A way round, and how far across itself it comes back.
#[derive(Clone, Debug)]
pub(super) struct Turn {
    /// The shift across, in the lengths of the arcs.
    pub shift: Expression,
    /// Its length on the way, with the real lengths of the arcs.
    pub around: f64,
    /// The rectangle it starts from.
    pub patch: usize,
}

/// The direction of each side of a rectangle, counter-clockwise from the
/// first.
const DIRECTION: [[f64; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]];

fn turn_expression(quarter_turns: usize, [a, b]: &[Expression; 2]) -> [Expression; 2] {
    let negated = |row: &Expression| -> Expression { row.iter().map(|&(i, c)| (i, -c)).collect() };
    match quarter_turns % 4 {
        0 => [a.clone(), b.clone()],
        1 => [negated(b), a.clone()],
        2 => [negated(a), negated(b)],
        _ => [b.clone(), negated(a)],
    }
}

fn merged(mut row: Expression) -> Expression {
    row.sort_by_key(|&(i, _)| i);
    let mut out: Expression = Vec::with_capacity(row.len());
    for (i, c) in row {
        match out.last_mut() {
            Some((j, d)) if *j == i => *d += c,
            _ => out.push((i, c)),
        }
    }
    out.retain(|&(_, c)| c != 0.0);
    out
}

/// A rectangle's rim from its first corner: each half-arc, its side, and
/// where it starts, in numbers and as an expression of the lengths.
struct Rim {
    halves: Vec<(HalfArc, usize)>,
    start: Vec<[f64; 2]>,
    start_expression: Vec<[Expression; 2]>,
    /// Where each side starts along the rim, and its length.
    side_start: [usize; 4],
}

fn rims(tmesh: &TMesh) -> Vec<Option<Rim>> {
    tmesh
        .patches
        .iter()
        .map(|patch| {
            let side_of = patch.side_of_each()?;
            let first = (0..patch.boundary.len())
                .find(|&k| side_of[k] == 0 && side_of[(k + side_of.len() - 1) % side_of.len()] == 3)?;
            let count = patch.boundary.len();
            let mut rim =
                Rim { halves: Vec::new(), start: Vec::new(), start_expression: Vec::new(), side_start: [0; 4] };
            let (mut at, mut at_expression) = ([0.0, 0.0], [Vec::new(), Vec::new()]);
            for step in 0..count {
                let k = (first + step) % count;
                let (half, side) = (patch.boundary[k], side_of[k] as usize);
                if step == 0 || side != rim.halves.last().unwrap().1 {
                    rim.side_start[side] = step;
                }
                rim.halves.push((half, side));
                rim.start.push(at);
                rim.start_expression.push(at_expression.clone());
                let arc = half as usize / 2;
                let length = tmesh.arcs[arc].length;
                let d = DIRECTION[side];
                at = [at[0] + d[0] * length, at[1] + d[1] * length];
                let [mut u, mut v] = at_expression;
                if d[0] != 0.0 {
                    u.push((arc, d[0]));
                }
                if d[1] != 0.0 {
                    v.push((arc, d[1]));
                }
                at_expression = [merged(u), merged(v)];
            }
            Some(rim)
        })
        .collect()
}

/// Turns found by going straight across the rectangles, from each of the
/// places `offsets` (shares of its length) along each side of each, at most `steps_per_patch` times the number of
/// rectangles before giving up.
pub(super) fn turns(tmesh: &TMesh, steps_per_patch: usize, offsets: &[f64], longest: f64) -> Vec<Turn> {
    let rims = rims(tmesh);
    // Where each half-arc is on a rim: the rectangle and the place.
    let mut place = vec![None; 2 * tmesh.arcs.len()];
    for (patch, rim) in rims.iter().enumerate() {
        if let Some(rim) = rim {
            for (k, &(half, _)) in rim.halves.iter().enumerate() {
                place[half as usize] = Some((patch, k));
            }
        }
    }
    let steps = steps_per_patch * tmesh.patches.len().max(1);
    let mut found = Vec::new();
    for (start_patch, rim) in rims.iter().enumerate() {
        let Some(rim) = rim else { continue };
        for start_side in 0..4 {
            // From the middle of that side, inwards.
            let begin = rim.side_start[start_side];
            let end = if start_side == 3 { rim.halves.len() } else { rim.side_start[start_side + 1] };
            let side_length: f64 = (begin..end).map(|k| tmesh.arcs[rim.halves[k].0 as usize / 2].length).sum();
            if side_length <= 0.0 {
                continue;
            }
            for &share in offsets {
                if let Some(turn) =
                    go_round(tmesh, &rims, &place, start_patch, start_side, share * side_length, steps, longest)
                {
                    found.push(turn);
                }
            }
        }
    }
    found
}

/// Goes straight across from side `side` of rectangle `patch`, `offset`
/// along it, until it comes back in through that side going the same way,
/// or has gone further than `longest`.
#[allow(clippy::too_many_arguments)]
fn go_round(
    tmesh: &TMesh,
    rims: &[Option<Rim>],
    place: &[Option<(usize, usize)>],
    patch: usize,
    side: usize,
    offset: f64,
    steps: usize,
    longest: f64,
) -> Option<Turn> {
    let (start_patch, start_side) = (patch, side);
    let (mut patch, mut side, mut offset) = (patch, side, offset);
    let mut quarter_turns = 0usize;
    let mut around = 0.0;
    let side_length = |rim: &Rim, side: usize| -> f64 {
        let begin = rim.side_start[side];
        let end = if side == 3 { rim.halves.len() } else { rim.side_start[side + 1] };
        (begin..end).map(|k| tmesh.arcs[rim.halves[k].0 as usize / 2].length).sum()
    };
    // The way: per crossing, the rectangle left and the place on its rim,
    // the one entered and the place on its rim.
    let mut crossings: Vec<(usize, usize, usize, usize)> = Vec::new();
    for _ in 0..steps {
        let rim = rims[patch].as_ref()?;
        // Across to the opposite side, the same distance from its end.
        let opposite = (side + 2) % 4;
        let begin = rim.side_start[opposite];
        let opposite_length = side_length(rim, opposite);
        around += side_length(rim, (side + 1) % 4);
        if around > longest {
            return None;
        }
        let along = (opposite_length - offset).clamp(0.0, opposite_length);
        let end = if opposite == 3 { rim.halves.len() } else { rim.side_start[opposite + 1] };
        let mut before = 0.0;
        let mut leaving = None;
        for k in begin..end {
            let length = tmesh.arcs[rim.halves[k].0 as usize / 2].length;
            if along < before + length || k + 1 == end {
                leaving = Some((k, along - before));
                break;
            }
            before += length;
        }
        let (k, into) = leaving?;
        let (half, _) = rim.halves[k];
        let arc = &tmesh.arcs[half as usize / 2];
        if arc.border {
            return None;
        }
        let (next, k_next) = place[(half ^ 1) as usize]?;
        let next_rim = rims[next].as_ref()?;
        let next_side = next_rim.halves[k_next].1;
        let next_begin = next_rim.side_start[next_side];
        let before_twin: f64 = (next_begin..k_next).map(|m| tmesh.arcs[next_rim.halves[m].0 as usize / 2].length).sum();
        crossings.push((patch, k, next, k_next));
        quarter_turns = (quarter_turns + (next_side + 2 + 4 - opposite) % 4) % 4;
        (patch, side, offset) = (next, next_side, before_twin + (arc.length - into));
        if patch == start_patch && side == start_side && quarter_turns == 0 {
            let across = if DIRECTION[start_side][0] != 0.0 { 0 } else { 1 };
            let mut shift = shift_along(tmesh, rims, &crossings);
            return Some(Turn { shift: std::mem::take(&mut shift[across]), around, patch: start_patch });
        }
    }
    None
}

/// How the coordinates of the first rectangle of a way turn and shift to
/// reach those of the last, crossing after crossing: the shift, in the
/// lengths of the arcs.
fn shift_along(tmesh: &TMesh, rims: &[Option<Rim>], crossings: &[(usize, usize, usize, usize)]) -> [Expression; 2] {
    let mut shift: [Expression; 2] = [Vec::new(), Vec::new()];
    for &(patch, k, next, k_next) in crossings {
        let (rim, next_rim) = (rims[patch].as_ref().unwrap(), rims[next].as_ref().unwrap());
        let (half, opposite) = rim.halves[k];
        let next_side = next_rim.halves[k_next].1;
        let turn = (next_side + 2 + 4 - opposite) % 4;
        // The node where `half` starts is where its twin ends.
        let here = &rim.start_expression[k];
        let mut there = next_rim.start_expression[k_next].clone();
        let d = DIRECTION[next_side];
        let arc = half as usize / 2;
        if d[0] != 0.0 {
            there[0].push((arc, d[0]));
        }
        if d[1] != 0.0 {
            there[1].push((arc, d[1]));
        }
        let turned_here = turn_expression(turn, here);
        let turned = turn_expression(turn, &shift);
        shift = [0, 1].map(|c| {
            merged(
                turned[c]
                    .iter()
                    .copied()
                    .chain(there[c].iter().copied())
                    .chain(turned_here[c].iter().map(|&(i, v)| (i, -v)))
                    .collect(),
            )
        });
    }
    let _ = tmesh;
    shift
}

/// The value of `expression` for the lengths `x`.
pub(super) fn value(expression: &Expression, x: &[f64]) -> f64 {
    expression.iter().map(|&(i, c)| c * x[i]).sum()
}

/// `lengths` changed as little as can be, each by its `weight`, for every
/// rectangle to keep its opposite sides equal and every turn in `held` to
/// come back `aimed` beside itself: onto itself at 0.
pub(super) fn closed(
    tmesh: &TMesh,
    lengths: &[f64],
    weight: &[f64],
    held: &[&Turn],
    aimed: &[f64],
) -> Option<Vec<f64>> {
    // The conditions, one per row: the two pairs of opposite sides of each
    // rectangle, then the turns.
    let mut rows: Vec<Expression> = Vec::new();
    for patch in &tmesh.patches {
        let Some(side_of) = patch.side_of_each() else { continue };
        for pair in 0..2 {
            let row: Expression = patch
                .boundary
                .iter()
                .zip(&side_of)
                .filter(|&(_, &side)| side as usize % 2 == pair)
                .map(|(&half, &side)| (half as usize / 2, if side < 2 { 1.0 } else { -1.0 }))
                .collect();
            rows.push(merged(row));
        }
    }
    let stations = rows.len();
    rows.extend(held.iter().map(|turn| turn.shift.clone()));
    let conditions = rows.len();
    // (Experiment: lengths that would go below a floor are held at it and
    // the projection made again, until none does: the least change with
    // the bound, where clamping afterwards breaks the conditions.)
    let floor_share: Option<f64> = std::env::var("OPENCLARE_ACTIVE_SET").ok().and_then(|v| v.parse().ok());
    let floor: Vec<f64> = lengths.iter().map(|&length| floor_share.map_or(0.0, |share| share * length)).collect();
    let mut held_at: Vec<usize> = Vec::new();
    for _ in 0..ACTIVE_ROUNDS {
        let mut rows = rows.clone();
        rows.extend(held_at.iter().map(|&arc| vec![(arc, 1.0)]));
        // Least change: delta = W^-1 A^T y with (A W^-1 A^T) y = -A lengths.
        let mut columns: Vec<Vec<(usize, f64)>> = vec![Vec::new(); lengths.len()];
        for (r, row) in rows.iter().enumerate() {
            for &(arc, c) in row {
                columns[arc].push((r, c));
            }
        }
        let mut system = SymmetricBuilder::new(rows.len());
        for (arc, column) in columns.iter().enumerate() {
            system.add_squared(column, 1.0 / weight[arc]);
        }
        for r in 0..rows.len() {
            // Rows that say the same are allowed: a little on the diagonal.
            system.add(r, r, 1e-9);
        }
        let rhs: Vec<f64> = rows
            .iter()
            .enumerate()
            .map(|(r, row)| {
                let wanted = if r < stations {
                    0.0
                } else if r < conditions {
                    aimed[r - stations]
                } else {
                    floor[held_at[r - conditions]]
                };
                wanted - value(row, lengths)
            })
            .collect();
        let y = system.factorize().ok()?.solve(&rhs);
        let mut out = lengths.to_vec();
        for (arc, column) in columns.iter().enumerate() {
            out[arc] += column.iter().map(|&(r, c)| c * y[r]).sum::<f64>() / weight[arc];
        }
        let below: Vec<usize> =
            (0..out.len()).filter(|&arc| out[arc] < floor[arc] - 1e-9 && !held_at.contains(&arc)).collect();
        if floor_share.is_none() || below.is_empty() {
            return Some(out.iter().zip(&floor).map(|(&length, &least)| length.max(least)).collect());
        }
        held_at.extend(below);
    }
    None
}

/// Most rounds of holding lengths at their floor in [`closed`].
const ACTIVE_ROUNDS: usize = 30;
