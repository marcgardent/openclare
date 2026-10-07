//! Borders and hard edges as cuts that are there before the race starts.
//!
//! A *wall* is an edge the quad mesh must have edges along: an open border,
//! or an edge marked as a feature of the shape. Walls are laid as traces, one
//! per straight run, between the vertices where they turn, end or meet: the
//! *corners*, which become nodes.
//!
//! Round a vertex, its walls cut the surface into sectors. In the map each
//! sector spans a whole number of quarter turns: two where a wall just runs
//! on, one at a convex corner, three or more where it turns into the surface.
//! A sector of `k` quarter turns sends out `k - 1` traces, one along each
//! grid direction strictly inside it, exactly as a singularity does all
//! round. A singularity that sits on a wall is covered by the same rule: its
//! sectors simply do not add up to four.

use super::Stretch;
use super::motorcycle::{Junction, Passage, Race, Trace, Wall};
use super::walk::{Axis, EPSILON};
use crate::math::INVALID;

/// A wall edge seen from one of its ends.
#[derive(Clone, Copy)]
struct Spoke {
    /// The half-edge leaving the vertex along it, in the face counter-
    /// clockwise of it; `None` if that side is off the surface.
    out: Option<usize>,
    /// The half-edge arriving at the vertex along it, in the face clockwise
    /// of it; `None` if that side is off the surface.
    back: Option<usize>,
    /// The vertex at its other end.
    other: u32,
    /// Quarter turns the sector counter-clockwise of it spans in the map; 0
    /// if there is none.
    turns: usize,
    /// Quarter turns from the first spoke of the vertex to this one: what
    /// orders the arcs round the node.
    key: usize,
}

impl Spoke {
    /// One number per wall edge, whichever end it is seen from.
    fn edge(&self) -> usize {
        self.out.unwrap_or(usize::MAX).min(self.back.unwrap_or(usize::MAX))
    }
}

impl Race<'_> {
    /// Lays every wall as a trace and starts the traces their corners send
    /// out. `is_wall` flags half-edges; `is_singular`, vertices.
    pub(super) fn lay_walls(&mut self, is_wall: &[bool], is_singular: &[bool]) {
        let map = self.map;
        let twin = &map.topology.opposite;
        let vertices = map.mesh.positions.len();
        let start_of = |h: usize| map.mesh.triangles[h / 3][h % 3];
        let end_of = |h: usize| map.mesh.triangles[h / 3][(h % 3 + 1) % 3];
        // Direction of a half-edge in the map, its length along that
        // direction and how far it strays from it.
        let run = |h: usize| {
            let (a, b) = (map.uv[h], map.uv[3 * (h / 3) + (h % 3 + 1) % 3]);
            let d = [b[0] - a[0], b[1] - a[1]];
            let along = if d[0].abs() >= d[1].abs() { 0 } else { 1 };
            let axis = (along + if d[along] < 0.0 { 2 } else { 0 }) as Axis;
            (axis, d[along].abs(), d[1 - along].abs())
        };

        // A wall half-edge to start from at each vertex: the one along the
        // border if there is a border, since the sectors then begin there.
        let mut first = vec![usize::MAX; vertices];
        for h in (0..is_wall.len()).filter(|&h| is_wall[h]) {
            let v = start_of(h) as usize;
            if first[v] == usize::MAX || twin[h] == INVALID {
                first[v] = h;
            }
        }

        // The spokes of each vertex, counter-clockwise.
        let mut spokes: Vec<Vec<Spoke>> = vec![Vec::new(); vertices];
        let mut on_border = vec![false; vertices];
        for v in (0..vertices).filter(|&v| first[v] != usize::MAX) {
            let mut h = first[v];
            let mut key = 0;
            loop {
                // The sector counter-clockwise of `h`, up to the next wall.
                let (mut face, mut corner) = (h / 3, h % 3);
                let mut angle = 0.0;
                let arriving = loop {
                    let [a, b, c] = std::array::from_fn(|k| map.uv[3 * face + (corner + k) % 3]);
                    let (to_next, to_previous) = ([b[0] - a[0], b[1] - a[1]], [c[0] - a[0], c[1] - a[1]]);
                    let turn = to_next[0] * to_previous[1] - to_next[1] * to_previous[0];
                    let reach = to_next[0] * to_previous[0] + to_next[1] * to_previous[1];
                    // A triangle the map lays flat has a corner of half a
                    // turn, whichever way rounding tips it.
                    let flat = turn.abs() <= 1e-9 * to_next[0].hypot(to_next[1]) * to_previous[0].hypot(to_previous[1]);
                    angle += if flat { turn.abs() } else { turn }.atan2(reach);
                    // Counter-clockwise round the vertex is across the edge
                    // arriving at it.
                    let arriving = 3 * face + (corner + 2) % 3;
                    if is_wall[arriving] || twin[arriving] == INVALID {
                        break arriving;
                    }
                    (face, corner) = (twin[arriving] as usize / 3, twin[arriving] as usize % 3);
                };
                let turns = (angle / std::f64::consts::FRAC_PI_2).round().max(1.0) as usize;
                let back = Some(twin[h]).filter(|&t| t != INVALID).map(|t| t as usize);
                spokes[v].push(Spoke { out: Some(h), back, other: end_of(h), turns, key });
                key += turns;
                let next = twin[arriving];
                if next == INVALID {
                    // The far side of the border: a spoke with no sector.
                    on_border[v] = true;
                    spokes[v].push(Spoke { out: None, back: Some(arriving), other: start_of(arriving), turns: 0, key });
                    break;
                }
                if next as usize == first[v] {
                    break;
                }
                h = next as usize;
            }
        }

        // A vertex a wall merely runs through is no corner.
        let runs_through = |v: usize| {
            let around = &spokes[v];
            around.len() == 2 && around[0].turns == 2 && (on_border[v] || around[1].turns == 2)
        };
        let mut node = vec![INVALID; vertices];
        let new_node = |race: &mut Self, v: usize, corner: bool| {
            let turns: usize = spokes[v].iter().map(|spoke| spoke.turns).sum();
            // Past a border there is no surface, only room for the rim of
            // the hole to close.
            let full_turn = (turns + if on_border[v] { 2 } else { 0 }) as f64;
            race.outcome.junctions.push(Junction::at_wall(
                map.mesh.positions[v],
                (is_singular[v] && !on_border[v]).then_some(v as u32),
                corner,
                full_turn,
            ));
            race.outcome.junctions.len() as u32 - 1
        };
        for v in (0..vertices).filter(|&v| !spokes[v].is_empty() && !runs_through(v)) {
            node[v] = new_node(self, v, true);
        }

        // One trace per run of wall from a corner to the next. A border is
        // only ever followed with the surface on its left.
        let mut used = vec![false; is_wall.len()];
        let mut starts: Vec<(usize, usize)> = Vec::new();
        for v in (0..vertices).filter(|&v| node[v] != INVALID) {
            starts.extend((0..spokes[v].len()).map(|index| (v, index)));
        }
        let mut pending = 0;
        loop {
            let Some(&(from, index)) = starts.get(pending) else {
                // What is left are closed walls without a corner: each needs
                // some vertex to hang its single arc on.
                let Some(v) = (0..vertices).find(|&v| {
                    node[v] == INVALID && spokes[v].iter().any(|spoke| spoke.out.is_some() && !used[spoke.edge()])
                }) else {
                    break;
                };
                node[v] = new_node(self, v, false);
                starts.push((v, 0));
                continue;
            };
            pending += 1;
            if spokes[from][index].out.is_none() || used[spokes[from][index].edge()] {
                continue;
            }

            let id = self.outcome.traces.len() as u32;
            let (mut path, mut stretches, mut length) = (Vec::new(), Vec::new(), 0.0);
            let mut border = true;
            let (mut v, mut spoke) = (from, spokes[from][index]);
            let end_key = loop {
                used[spoke.edge()] = true;
                let other = spoke.other as usize;
                // The trace is told from the face on its left, or from the
                // one on its right where the left is off the surface.
                let (seen_from, forward) = match spoke.out {
                    Some(h) => (h, true),
                    None => (spoke.back.expect("a spoke has a side"), false),
                };
                let (axis, along, _) = run(seen_from);
                let axis = if forward { axis } else { (axis + 2) % 4 };
                let corner = if forward { seen_from % 3 } else { (seen_from % 3 + 1) % 3 };
                let face = seen_from / 3;
                stretches.push(Stretch {
                    face: face as u32,
                    start: map.uv[3 * face + corner],
                    axis,
                    length: along,
                    time: length,
                });
                if self.passages[v].is_none() {
                    self.passages[v] = Some(Passage { trace: id, time: length, departure: (face, corner, axis) });
                }
                for (half, sense) in [(spoke.out, 1.0), (spoke.back, -1.0)] {
                    let Some(half) = half else { continue };
                    let (own_axis, own_length, astray) = run(half);
                    if astray > EPSILON.max(1e-3 * own_length) {
                        self.outcome.off_grid_edges += 1;
                    }
                    let time = if sense > 0.0 { length } else { length + along };
                    let axis = if sense > 0.0 { own_axis } else { (own_axis + 2) % 4 };
                    self.walls[half] = Some(Wall { trace: id, time, sense, length: own_length, axis });
                }
                border &= spoke.back.is_none();
                path.push(map.mesh.positions[v]);
                length += along;

                // On to the same edge seen from its other end.
                let arrival = spokes[other]
                    .iter()
                    .position(|candidate| candidate.other as usize == v && candidate.edge() == spoke.edge())
                    .expect("a wall edge has two ends");
                if node[other] != INVALID {
                    if self.passages[other].is_none() {
                        self.passages[other] =
                            Some(Passage { trace: id, time: length, departure: (face, corner, axis) });
                    }
                    path.push(map.mesh.positions[other]);
                    break (other, spokes[other][arrival].key);
                }
                // No corner there: the wall has exactly one other spoke.
                (v, spoke) = (other, spokes[other][1 - arrival]);
            };
            self.outcome.traces.push(Trace::wall(
                (node[from], spokes[from][index].key as f64),
                (node[end_key.0], end_key.1 as f64),
                length,
                path,
                stretches,
                border,
            ));
        }

        // The traces of the corners: one along each grid direction strictly
        // inside a sector.
        for v in (0..vertices).filter(|&v| node[v] != INVALID) {
            for spoke in &spokes[v] {
                let Some(h) = spoke.out else { continue };
                let (axis, _, _) = run(h);
                for quarter in 1..spoke.turns {
                    if let Some(departure) = map.enter_from_vertex(h / 3, h % 3, (axis + quarter as Axis) % 4) {
                        self.start(node[v], (spoke.key + quarter) as f64, departure);
                    }
                }
            }
        }
    }
}
