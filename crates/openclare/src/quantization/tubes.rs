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
    /// How far beside its start it came back, measured on the way.
    pub measured: f64,
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
            let first = (0..patch.boundary.len()).find(|&k| side_of[k] == 0 && side_of[(k + side_of.len() - 1) % side_of.len()] == 3)?;
            let count = patch.boundary.len();
            let mut rim = Rim { halves: Vec::new(), start: Vec::new(), start_expression: Vec::new(), side_start: [0; 4] };
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

/// Turns found by going straight across the rectangles, from the middle of
/// each side of each, at most `steps_per_patch` times the number of
/// rectangles before giving up.
pub(super) fn turns(tmesh: &TMesh, steps_per_patch: usize) -> Vec<Turn> {
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
            if let Some(turn) = go_round(tmesh, &rims, &place, start_patch, start_side, 0.5 * side_length, steps) {
                found.push(turn);
            }
        }
    }
    found
}

/// Goes straight across from side `side` of rectangle `patch`, `offset`
/// along it, until it comes back in through that side going the same way.
fn go_round(
    tmesh: &TMesh,
    rims: &[Option<Rim>],
    place: &[Option<(usize, usize)>],
    patch: usize,
    side: usize,
    offset: f64,
    steps: usize,
) -> Option<Turn> {
    let (start_patch, start_side, start_offset) = (patch, side, offset);
    let (mut patch, mut side, mut offset) = (patch, side, offset);
    // How the coordinates of the start rectangle have turned and shifted to
    // reach those of the one the way is in.
    let mut quarter_turns = 0usize;
    let mut shift: [Expression; 2] = [Vec::new(), Vec::new()];
    let mut around = 0.0;
    for _ in 0..steps {
        let rim = rims[patch].as_ref()?;
        // Across to the opposite side, the same distance from its end.
        let opposite = (side + 2) % 4;
        let begin = rim.side_start[opposite];
        let end = if opposite == 3 { rim.halves.len() } else { rim.side_start[opposite + 1] };
        let lengths: Vec<f64> = (begin..end).map(|k| tmesh.arcs[rim.halves[k].0 as usize / 2].length).collect();
        let opposite_length: f64 = lengths.iter().sum();
        let across_rim = {
            let b = rim.side_start[(side + 1) % 4];
            let e = if (side + 1) % 4 == 3 { rim.halves.len() } else { rim.side_start[(side + 1) % 4 + 1] };
            (b..e).map(|k| tmesh.arcs[rim.halves[k].0 as usize / 2].length).sum::<f64>()
        };
        around += across_rim;
        let along = (opposite_length - offset).clamp(0.0, opposite_length);
        // The half-arc of the opposite side the way leaves through.
        let mut before = 0.0;
        let mut leaving = None;
        for (k, &length) in lengths.iter().enumerate() {
            if along < before + length || k + 1 == lengths.len() {
                leaving = Some((begin + k, along - before));
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
        let twin = half ^ 1;
        let (next, k_next) = place[twin as usize]?;
        let next_rim = rims[next].as_ref()?;
        let next_side = next_rim.halves[k_next].1;
        // Into the next rectangle: the twin runs the other way.
        let length = arc.length;
        let next_begin = next_rim.side_start[next_side];
        let before_twin: f64 =
            (next_begin..k_next).map(|m| tmesh.arcs[next_rim.halves[m].0 as usize / 2].length).sum();
        let next_offset = before_twin + (length - into);
        // The coordinates of the next rectangle from those of this one: the
        // node where `half` starts is where its twin ends.
        let turn = (next_side + 2 + 4 - opposite) % 4;
        let here = &rim.start_expression[k];
        let mut there = next_rim.start_expression[k_next].clone();
        let d = DIRECTION[next_side];
        let arc_index = half as usize / 2;
        if d[0] != 0.0 {
            there[0].push((arc_index, d[0]));
        }
        if d[1] != 0.0 {
            there[1].push((arc_index, d[1]));
        }
        let turned_here = turn_expression(turn, here);
        let step: [Expression; 2] = [
            merged(there[0].iter().copied().chain(turned_here[0].iter().map(|&(i, c)| (i, -c))).collect()),
            merged(there[1].iter().copied().chain(turned_here[1].iter().map(|&(i, c)| (i, -c))).collect()),
        ];
        let turned = turn_expression(turn, &shift);
        shift = [
            merged(turned[0].iter().copied().chain(step[0].iter().copied()).collect()),
            merged(turned[1].iter().copied().chain(step[1].iter().copied()).collect()),
        ];
        quarter_turns = (quarter_turns + turn) % 4;
        (patch, side, offset) = (next, next_side, next_offset);
        if patch == start_patch && side == start_side && quarter_turns == 0 {
            // Across the way is along the side it went in through.
            let across = if DIRECTION[start_side][0] != 0.0 { 0 } else { 1 };
            return Some(Turn { shift: std::mem::take(&mut shift[across]), around, measured: offset - start_offset });
        }
    }
    None
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
    rows.extend(held.iter().map(|turn| turn.shift.clone()));
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
    let stations = rows.len() - held.len();
    let rhs: Vec<f64> = rows
        .iter()
        .enumerate()
        .map(|(r, row)| if r < stations { 0.0 } else { aimed[r - stations] } - value(row, lengths))
        .collect();
    let y = system.factorize().ok()?.solve(&rhs);
    let mut out = lengths.to_vec();
    for (arc, column) in columns.iter().enumerate() {
        let change: f64 = column.iter().map(|&(r, c)| c * y[r]).sum::<f64>() / weight[arc];
        out[arc] = (out[arc] + change).max(0.0);
    }
    Some(out)
}
