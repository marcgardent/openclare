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

use crate::linalg::{Factorization, SymmetricBuilder, solve_dense};
use crate::tmesh::{HalfArc, TMesh};

/// A linear expression in the lengths of the arcs: `(arc, coefficient)`.
pub(super) type Expression = Vec<(usize, f64)>;

/// A way round, and how far across itself it comes back.
#[derive(Clone, Debug)]
pub(super) struct Turn {
    /// The shift across, in the lengths of the arcs.
    pub shift: Expression,
    /// The other component of the same motion: how far along the way it
    /// comes back, in the lengths of the arcs. With lengths that balance
    /// every rectangle, it is `around` up to its sign.
    pub along: Expression,
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

fn rims(tmesh: &TMesh, lengths: &[f64]) -> Vec<Option<Rim>> {
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
                let length = lengths[arc];
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
    let lengths: Vec<f64> = tmesh.arcs.iter().map(|arc| arc.length).collect();
    turns_with(
        tmesh,
        &lengths,
        steps_per_patch,
        &|side_length| offsets.iter().map(|share| share * side_length).collect(),
        longest,
    )
}

/// Every turn of the quad mesh that whole `lengths` make: from the middle of
/// every quad edge along every side of every rectangle, at the lengths
/// themselves, so that each way is a strip of quads of that mesh and none
/// is missed. Each way found once.
pub(super) fn lattice_turns(tmesh: &TMesh, lengths: &[f64], steps_per_patch: usize, longest: f64) -> Vec<Turn> {
    let middles = |side_length: f64| (0..side_length.round() as usize).map(|k| k as f64 + 0.5).collect();
    let mut found = turns_with(tmesh, lengths, steps_per_patch, &middles, longest);
    let mut seen = std::collections::HashSet::new();
    found.retain(|turn| seen.insert(turn.shift.iter().map(|&(arc, c)| (arc, c.round() as i64)).collect::<Vec<_>>()));
    found
}

/// Turns from the places `starts` gives along each side, from its length.
fn turns_with(
    tmesh: &TMesh,
    lengths: &[f64],
    steps_per_patch: usize,
    starts: &dyn Fn(f64) -> Vec<f64>,
    longest: f64,
) -> Vec<Turn> {
    let rims = rims(tmesh, lengths);
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
            let side_length: f64 = (begin..end).map(|k| lengths[rim.halves[k].0 as usize / 2]).sum();
            if side_length <= 0.0 {
                continue;
            }
            for offset in starts(side_length) {
                if let Some(turn) =
                    go_round(tmesh, lengths, &rims, &place, start_patch, start_side, offset, steps, longest)
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
    lengths: &[f64],
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
        (begin..end).map(|k| lengths[rim.halves[k].0 as usize / 2]).sum()
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
            let length = lengths[rim.halves[k].0 as usize / 2];
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
        let before_twin: f64 = (next_begin..k_next).map(|m| lengths[next_rim.halves[m].0 as usize / 2]).sum();
        crossings.push((patch, k, next, k_next));
        quarter_turns = (quarter_turns + (next_side + 2 + 4 - opposite) % 4) % 4;
        (patch, side, offset) = (next, next_side, before_twin + (lengths[half as usize / 2] - into));
        if patch == start_patch && side == start_side && quarter_turns == 0 {
            let across = if DIRECTION[start_side][0] != 0.0 { 0 } else { 1 };
            let [first, second] = shift_along(tmesh, rims, &crossings);
            let (shift, along) = if across == 0 { (first, second) } else { (second, first) };
            // Each crossing turns by quarter turns and shifts by sums of
            // lengths, so every coefficient is a whole number (the shift of a
            // turn is linear, proven in the research note).
            debug_assert!(
                shift.iter().chain(&along).all(|&(_, c)| c.fract() == 0.0),
                "a turn from rectangle {start_patch} has a coefficient that is not whole: {shift:?} / {along:?}"
            );
            return Some(Turn { shift, along, around, patch: start_patch });
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

/// Per rectangle, whether the piece of surface it is on is a sphere: closed,
/// with no border, and of genus zero. The pieces are the rectangles joined
/// across their arcs; each has the Euler characteristic of its nodes, arcs
/// and rectangles, 2 for a closed sphere. Only there does the parity of
/// turns hold: an open cylinder three quads round has a ring of three quads,
/// and so does a torus (the research note, Remark "Borders").
pub(super) fn on_spheres(tmesh: &TMesh) -> Vec<bool> {
    let patches = tmesh.patches.len();
    let mut parent: Vec<usize> = (0..patches).collect();
    fn root(parent: &mut [usize], mut at: usize) -> usize {
        while parent[at] != at {
            parent[at] = parent[parent[at]];
            at = parent[at];
        }
        at
    }
    // The rectangle on each side of each arc, where there is one.
    let mut sides = vec![[None::<usize>; 2]; tmesh.arcs.len()];
    for (index, patch) in tmesh.patches.iter().enumerate() {
        for &half in &patch.boundary {
            sides[half as usize / 2][half as usize % 2] = Some(index);
        }
    }
    for pair in &sides {
        if let [Some(a), Some(b)] = *pair {
            let (a, b) = (root(&mut parent, a), root(&mut parent, b));
            parent[a] = b;
        }
    }
    let piece: Vec<usize> = (0..patches).map(|index| root(&mut parent, index)).collect();
    let mut euler = vec![0i64; patches];
    let mut open = vec![false; patches];
    let mut nodes = std::collections::HashSet::new();
    for &index in &piece {
        euler[index] += 1;
    }
    for (arc, pair) in sides.iter().enumerate() {
        let Some(index) = pair[0].or(pair[1]) else { continue };
        let index = piece[index];
        euler[index] -= 1;
        if pair[0].is_none() || pair[1].is_none() || tmesh.arcs[arc].border {
            open[index] = true;
        }
        for node in tmesh.arcs[arc].nodes {
            if nodes.insert(node) {
                euler[index] += 1;
            }
        }
    }
    piece.iter().map(|&index| !open[index] && euler[index] == 2).collect()
}

/// What breaks a statement proven for turns, at the whole lengths `x`
/// balancing every rectangle: on a sphere (`sphere`, per rectangle, from
/// [`on_spheres`]), the two components of a turn add up to an even number
/// (the research note, Proposition "Parity of a turn on a sphere"); if
/// `walked`, the turns were found at `x` itself, and the length along each
/// is its way round. Each turn that breaks one, and what. Empty when all
/// hold.
pub(super) fn broken_by(turns: &[Turn], sphere: &[bool], x: &[f64], walked: bool) -> Vec<(usize, String)> {
    let mut broken = Vec::new();
    for (index, turn) in turns.iter().enumerate() {
        let (shift, along) = (value(&turn.shift, x), value(&turn.along, x));
        if sphere[turn.patch] && (shift + along).rem_euclid(2.0) != 0.0 {
            broken.push((
                index,
                format!(
                    "turn {index} from rectangle {}: shift {shift} and along {along} add up to an odd number on a sphere",
                    turn.patch
                ),
            ));
        }
        if walked && along.abs() != turn.around {
            broken.push((
                index,
                format!(
                    "turn {index} from rectangle {}: along {along} is not its way round {}",
                    turn.patch, turn.around
                ),
            ));
        }
    }
    broken
}

/// `lengths` changed as little as can be, each by its `weight`, for every
/// rectangle to keep its opposite sides equal and every turn held to come
/// back where it is aimed beside itself: onto itself at 0.
///
/// The least change is `W^-1 A^T y` with `(A W^-1 A^T) y = -A lengths`, one
/// row of `A` per condition. The rows of the rectangles are the same for
/// every set of turns held, and the most of them: their part of the system
/// is factorized once, and the rows of the turns are reduced against it into
/// a small dense system of their own, the Schur complement, solved anew for
/// each set. Arcs held at nothing are no longer free: they are taken out of
/// the system, which is factorized again without them.
pub(super) struct Closing {
    /// The rows of the rectangles: two pairs of opposite sides each.
    stations: Vec<Expression>,
    /// The lengths, those of the arcs held at nothing at nothing.
    lengths: Vec<f64>,
    weight: Vec<f64>,
    /// Per arc, how freely it changes: the inverse of its weight, nothing
    /// for an arc held.
    spread: Vec<f64>,
    /// Per arc, the rows of the rectangles it is on, with its coefficient.
    columns: Vec<Vec<(usize, f64)>>,
    factorization: Factorization,
    /// The multipliers of the rectangles alone: `S^-1 (-A_s lengths)`.
    base: Vec<f64>,
}

/// A row added to those of the rectangles, reduced against them.
struct Reduced {
    row: Expression,
    /// `A_s W^-1 row^T`, over the rows of the rectangles.
    coupling: Expression,
    /// `S^-1` of that.
    solved: Vec<f64>,
}

/// The rows of turns added to those of the rectangles, each reduced once,
/// and what each pair of them gives in the small system.
pub(super) struct Rows<'a> {
    closing: &'a Closing,
    rows: Vec<Reduced>,
    /// Row after row, the entries of the small system between them.
    gram: Vec<Vec<f64>>,
}

impl Closing {
    pub fn new(tmesh: &TMesh, lengths: &[f64], weight: &[f64]) -> Option<Self> {
        let mut stations: Vec<Expression> = Vec::new();
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
                stations.push(merged(row));
            }
        }
        if cfg!(debug_assertions) {
            let broken = unbalanced_columns(tmesh, &stations);
            assert!(broken.is_empty(), "the balances break the handshake: {broken:?}");
        }
        Self::with_held(stations, lengths, weight, &[])
    }

    /// The same with the arcs `held` at nothing as well.
    fn with_held(stations: Vec<Expression>, lengths: &[f64], weight: &[f64], held: &[usize]) -> Option<Self> {
        let mut lengths = lengths.to_vec();
        let mut spread: Vec<f64> = weight.iter().map(|w| 1.0 / w).collect();
        for &arc in held {
            lengths[arc] = 0.0;
            spread[arc] = 0.0;
        }
        let mut columns: Vec<Vec<(usize, f64)>> = vec![Vec::new(); lengths.len()];
        for (r, row) in stations.iter().enumerate() {
            for &(arc, c) in row {
                columns[arc].push((r, c));
            }
        }
        let mut system = SymmetricBuilder::new(stations.len());
        for (arc, column) in columns.iter().enumerate() {
            system.add_squared(column, spread[arc]);
        }
        for r in 0..stations.len() {
            // Rows that say the same are allowed: a little on the diagonal.
            system.add(r, r, 1e-9);
        }
        let factorization = system.factorize().ok()?;
        let base = factorization.solve(&stations.iter().map(|row| -value(row, &lengths)).collect::<Vec<_>>());
        Some(Self { stations, lengths, weight: weight.to_vec(), spread, columns, factorization, base })
    }

    fn reduce(&self, row: Expression) -> Reduced {
        let mut coupling = Vec::new();
        for &(arc, c) in &row {
            coupling.extend(self.columns[arc].iter().map(|&(r, d)| (r, c * d * self.spread[arc])));
        }
        let coupling = merged(coupling);
        let mut dense = vec![0.0; self.base.len()];
        for &(r, v) in &coupling {
            dense[r] = v;
        }
        let solved = self.factorization.solve(&dense);
        Reduced { row, coupling, solved }
    }
}

impl<'a> Rows<'a> {
    pub fn new(closing: &'a Closing) -> Self {
        Self { closing, rows: Vec::new(), gram: Vec::new() }
    }

    /// Adds a row, reduced; its index.
    pub fn push(&mut self, row: Expression) -> usize {
        let closing = self.closing;
        let reduced = closing.reduce(merged(row));
        let entry = |other: &Reduced| -> f64 {
            sparse_dot(&reduced.row, &other.row, &closing.spread)
                - reduced.coupling.iter().map(|&(r, v)| v * other.solved[r]).sum::<f64>()
        };
        let mut entries: Vec<f64> = self.rows.iter().map(entry).collect();
        entries.push(entry(&reduced) + 1e-9);
        self.rows.push(reduced);
        self.gram.push(entries);
        self.rows.len() - 1
    }

    fn entry(&self, i: usize, j: usize) -> f64 {
        if j <= i { self.gram[i][j] } else { self.gram[j][i] }
    }

    /// The lengths changed as little as can be for the rows `active` to
    /// come back to `aimed`. If `pin`, arcs it would shorten below nothing
    /// are held at nothing and the change looked for again, until none is
    /// asked for less; otherwise they are cut off there.
    pub fn closed(&self, active: &[usize], aimed: &[f64], pin: bool) -> Option<Vec<f64>> {
        let (mut out, mut below) = self.solve(active, aimed)?;
        let mut held: Vec<usize> = Vec::new();
        let closing = self.closing;
        for _ in 0..if pin { PIN_ROUNDS } else { 0 } {
            if below.is_empty() {
                break;
            }
            held.extend(below);
            let Some(pinned) = Closing::with_held(closing.stations.clone(), &closing.lengths, &closing.weight, &held)
            else {
                break;
            };
            let mut rows = Rows::new(&pinned);
            let again: Vec<usize> = active.iter().map(|&i| rows.push(self.rows[i].row.clone())).collect();
            (out, below) = rows.solve(&again, aimed)?;
        }
        Some(out)
    }

    /// The least change for the rows `active`, and the arcs it asks for
    /// less than nothing.
    fn solve(&self, active: &[usize], aimed: &[f64]) -> Option<(Vec<f64>, Vec<usize>)> {
        let closing = self.closing;
        let size = active.len();
        let mut matrix = vec![0.0; size * size];
        for (a, &i) in active.iter().enumerate() {
            for (b, &j) in active.iter().enumerate() {
                matrix[a * size + b] = self.entry(i, j);
            }
        }
        let rhs: Vec<f64> = active
            .iter()
            .zip(aimed)
            .map(|(&i, &aim)| {
                let row = &self.rows[i];
                aim - value(&row.row, &closing.lengths)
                    - row.coupling.iter().map(|&(r, v)| v * closing.base[r]).sum::<f64>()
            })
            .collect();
        let y = solve_dense(size, &matrix, &rhs).ok()?;
        let mut stations = closing.base.clone();
        for (&i, &multiplier) in active.iter().zip(&y) {
            for (s, &v) in stations.iter_mut().zip(&self.rows[i].solved) {
                *s -= multiplier * v;
            }
        }
        let mut change: Vec<f64> =
            closing.columns.iter().map(|column| column.iter().map(|&(r, c)| c * stations[r]).sum::<f64>()).collect();
        for (&i, &multiplier) in active.iter().zip(&y) {
            for &(arc, c) in &self.rows[i].row {
                change[arc] += c * multiplier;
            }
        }
        let mut out = closing.lengths.clone();
        let mut below = Vec::new();
        for (arc, change) in change.iter().enumerate() {
            let change = change * closing.spread[arc];
            if out[arc] + change < -1e-9 {
                below.push(arc);
            }
            out[arc] = (out[arc] + change).max(0.0);
        }
        Some((out, below))
    }
}

/// Arcs whose column in the balances `stations` of the rectangles of `tmesh`
/// is not what the handshake needs (the research note, Lemma "Handshake"):
/// each half of the arc on a rim gives one entry `+1` or `-1`, two of them
/// on one row merged, so its whole entries have a size of at most the
/// number of its halves on rims and a sum of the same parity. An arc with
/// both halves on rims, every arc of a closed surface, has an even sum:
/// `B x` then adds up to an even number for every whole `x`, and a balance
/// system whose right-hand side adds up to an odd number has no whole
/// solution. Empty when all hold.
pub(super) fn unbalanced_columns(tmesh: &TMesh, stations: &[Expression]) -> Vec<usize> {
    let mut halves = vec![0i64; tmesh.arcs.len()];
    for patch in tmesh.patches.iter().filter(|patch| patch.side_of_each().is_some()) {
        for &half in &patch.boundary {
            halves[half as usize / 2] += 1;
        }
    }
    let (mut sum, mut size, mut whole) = (vec![0i64; halves.len()], vec![0i64; halves.len()], vec![true; halves.len()]);
    for row in stations {
        for &(arc, c) in row {
            whole[arc] &= c.fract() == 0.0;
            sum[arc] += c as i64;
            size[arc] += c.abs() as i64;
        }
    }
    (0..halves.len())
        .filter(|&arc| !whole[arc] || size[arc] > halves[arc] || (sum[arc] - halves[arc]).rem_euclid(2) != 0)
        .collect()
}

/// `a W^-1 b^T` for two rows sorted by arc, `W^-1` given as `spread`.
fn sparse_dot(a: &Expression, b: &Expression, spread: &[f64]) -> f64 {
    let (mut i, mut j, mut sum) = (0, 0, 0.0);
    while i < a.len() && j < b.len() {
        match a[i].0.cmp(&b[j].0) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                sum += a[i].1 * b[j].1 * spread[a[i].0];
                i += 1;
                j += 1;
            }
        }
    }
    sum
}

/// Most rounds of holding at zero the arcs the least change shortens below.
const PIN_ROUNDS: usize = 20;
