//! The motorcycle graph of a seamless map.
//!
//! A trace leaves every singularity along each grid direction, all at the
//! same speed, and stops at the first trace it runs into (Eppstein et al.
//! 2008; used this way by Campen et al. 2015). What is left is a partition
//! of the surface into rectangles of the map.
//!
//! Open borders and hard edges are cuts that are there from the start (see
//! [`super::walls`]): traces stop on them, and where they turn a corner they
//! send traces out as a singularity does. That takes a map in which they run
//! along grid lines.
//!
//! Traces advance one triangle at a time, the one furthest behind first. A
//! trace is therefore stopped by whatever was laid down in its triangle
//! before it got there, which can differ from the ideal race by the time it
//! takes to cross one triangle. Any outcome of that is a valid partition,
//! which is all the next stage asks for.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::Stretch;
use super::walk::{Axis, Departure, EPSILON, Exit, Map, axis_vector};
use crate::math::{INVALID, Vec3};

/// Two events on a trace closer than this, in grid units, are one node.
const MERGE: f64 = 4.0 * EPSILON;

/// A node of the graph being built.
pub(super) struct Junction {
    pub position: Vec3,
    /// The singular vertex it sits on, if any.
    pub vertex: Option<u32>,
    /// It is a vertex where a border or a hard edge turns, ends or meets
    /// another.
    pub corner: bool,
    /// Quarter turns in a full turn round it: four, except round a
    /// singularity. (Round a corner of the border it only has to exceed the
    /// angle of the surface there; what lies beyond is no patch.)
    pub full_turn: f64,
    /// On a regular node, which of its four directions carry an arc.
    used: [bool; 4],
}

impl Junction {
    fn regular(position: Vec3) -> Self {
        Self { position, vertex: None, corner: false, full_turn: 4.0, used: [false; 4] }
    }

    /// A node on a wall: `corner` unless the wall just runs through it.
    pub(super) fn at_wall(position: Vec3, vertex: Option<u32>, corner: bool, full_turn: f64) -> Self {
        Self { position, vertex, corner, full_turn, used: [false; 4] }
    }
}

pub(super) struct Trace {
    /// Node it starts from and the key of its arc there.
    pub start: (u32, f64),
    /// Node it stopped at and the key of its arc there; `None` while it
    /// runs, and for good if it got lost.
    pub end: Option<(u32, f64)>,
    /// Nodes other traces made along it: its length at the node, the node,
    /// and the key there of the direction it runs in.
    pub through: Vec<(f64, u32, u8)>,
    pub length: f64,
    /// Where it went on the surface, one point per triangle crossed.
    pub path: Vec<Vec3>,
    /// The same in the map, one stretch per triangle crossed.
    pub stretches: Vec<Stretch>,
    /// A stretch of border or hard edge rather than a trace: there from the
    /// start.
    pub wall: bool,
    /// A stretch of open border: the surface is on its left only.
    pub border: bool,
    pub(super) face: usize,
    pub(super) point: [f64; 2],
    pub(super) axis: Axis,
    pub(super) running: bool,
    pub(super) steps: usize,
}

impl Trace {
    /// A run of wall, laid whole before the race.
    pub(super) fn wall(
        start: (u32, f64),
        end: (u32, f64),
        length: f64,
        path: Vec<Vec3>,
        stretches: Vec<Stretch>,
        border: bool,
    ) -> Self {
        Self {
            start,
            end: Some(end),
            through: Vec::new(),
            length,
            path,
            stretches,
            wall: true,
            border,
            face: 0,
            point: [0.0; 2],
            axis: 0,
            running: false,
            steps: 0,
        }
    }
}

/// A stretch of a trace within one triangle, in the coordinates of that
/// triangle.
struct Segment {
    trace: u32,
    start: [f64; 2],
    axis: Axis,
    length: f64,
    /// Length of the trace where the stretch starts.
    time: f64,
}

/// A trace that went through a regular vertex: which, its length there, and
/// the direction it left in.
#[derive(Clone, Copy)]
pub(super) struct Passage {
    pub trace: u32,
    pub time: f64,
    pub departure: Departure,
}

/// One side of a border edge or hard edge, as a stretch of cut.
#[derive(Clone, Copy)]
pub(super) struct Wall {
    pub trace: u32,
    /// Length of its trace where this half-edge starts, and whether the
    /// trace runs the way the half-edge does (1) or against it (-1).
    pub time: f64,
    pub sense: f64,
    pub length: f64,
    /// Direction of the trace in the coordinates of the face of the
    /// half-edge.
    pub axis: Axis,
}

#[derive(Default)]
pub(super) struct Outcome {
    pub junctions: Vec<Junction>,
    pub traces: Vec<Trace>,
    /// Border edges and hard edges that do not run along a grid line of the
    /// map: no rectangle can have them for a side.
    pub off_grid_edges: usize,
    /// Traces that never met another: out of length, at a border, or off the
    /// map through a degenerate triangle.
    pub lost: usize,
    /// Traces that ran into another along the same line, which leaves two
    /// arcs on top of each other.
    pub overlaps: usize,
}

struct Hit {
    distance: f64,
    segment: usize,
    crossing: bool,
}

pub(super) struct Race<'a> {
    pub(super) map: &'a Map<'a>,
    singular: Vec<u32>,
    segments: Vec<Vec<Segment>>,
    pub(super) passages: Vec<Option<Passage>>,
    /// Per half-edge, for those on a border or a hard edge.
    pub(super) walls: Vec<Option<Wall>>,
    pub(super) outcome: Outcome,
    max_length: f64,
}

/// Runs the race on `map` from the vertices flagged in `is_singular`, with
/// the half-edges flagged in `is_wall` for cuts already there. `seeds` are
/// plain vertices to start from as well, for the pieces of surface that
/// have neither.
pub(super) fn race(map: &Map, is_singular: &[bool], is_wall: &[bool], seeds: &[u32], max_length: f64) -> Outcome {
    let vertices = map.mesh.positions.len();
    let mut race = Race {
        map,
        singular: vec![INVALID; vertices],
        segments: (0..map.mesh.triangles.len()).map(|_| Vec::new()).collect(),
        passages: vec![None; vertices],
        walls: vec![None; 3 * map.mesh.triangles.len()],
        outcome: Outcome::default(),
        max_length,
    };

    // A singularity on a wall is dealt with as part of the wall: what it
    // sends out depends on the corners the wall makes there.
    let mut on_wall = vec![false; vertices];
    for h in (0..is_wall.len()).filter(|&h| is_wall[h]) {
        let triangle = map.mesh.triangles[h / 3];
        on_wall[triangle[h % 3] as usize] = true;
        on_wall[triangle[(h + 1) % 3] as usize] = true;
    }
    for v in (0..vertices as u32).filter(|&v| is_singular[v as usize] && !on_wall[v as usize]) {
        let departures = map.departures(v);
        let node = race.outcome.junctions.len() as u32;
        race.singular[v as usize] = node;
        race.outcome.junctions.push(Junction {
            position: map.mesh.positions[v as usize],
            vertex: Some(v),
            corner: false,
            full_turn: departures.len() as f64,
            used: [false; 4],
        });
        for (rank, departure) in departures.into_iter().enumerate() {
            race.start(node, rank as f64, departure);
        }
    }
    race.lay_walls(is_wall, is_singular);
    // A piece with no singularity and no wall (a ring) has nothing to be cut
    // from. Any vertex will do to start: it sends a trace along each of its
    // four grid directions, which go round the piece and stop on one
    // another.
    for &v in seeds {
        let node = race.outcome.junctions.len() as u32;
        race.outcome.junctions.push(Junction::regular(map.mesh.positions[v as usize]));
        for (rank, departure) in map.departures(v).into_iter().enumerate() {
            if rank == 0 {
                let trace = race.outcome.traces.len() as u32;
                race.passages[v as usize] = Some(Passage { trace, time: 0.0, departure });
            }
            race.start(node, rank as f64, departure);
        }
    }

    let mut queue: BinaryHeap<Reverse<(Time, u32)>> = (0..race.outcome.traces.len() as u32)
        .filter(|&id| race.outcome.traces[id as usize].running)
        .map(|id| Reverse((Time(0.0), id)))
        .collect();
    while let Some(Reverse((_, id))) = queue.pop() {
        if race.outcome.traces[id as usize].running && race.step(id as usize) {
            queue.push(Reverse((Time(race.outcome.traces[id as usize].length), id)));
        }
    }
    race.outcome.lost = race.outcome.traces.iter().filter(|trace| trace.end.is_none()).count();
    race.outcome
}

impl Race<'_> {
    /// A new trace leaving `node` along `departure`.
    pub(super) fn start(&mut self, node: u32, key: f64, (face, corner, axis): Departure) {
        self.outcome.traces.push(Trace {
            start: (node, key),
            end: None,
            through: Vec::new(),
            length: 0.0,
            path: vec![self.outcome.junctions[node as usize].position],
            stretches: Vec::new(),
            wall: false,
            border: false,
            face,
            point: self.map.uv[3 * face + corner],
            axis,
            running: true,
            steps: 0,
        });
    }

    /// Takes trace `id` across the triangle it is in. Returns whether it is
    /// still running.
    fn step(&mut self, id: usize) -> bool {
        let map = self.map;
        let (face, point, axis, length, first) = {
            let trace = &mut self.outcome.traces[id];
            trace.steps += 1;
            (trace.face, trace.point, trace.axis, trace.length, trace.steps == 1)
        };
        // A trace that long is going round in circles between two others it
        // never quite meets, or the map is broken where it is.
        // (A trace crosses a few triangles per quad edge of its length; one
        // that takes many more steps is not getting anywhere.)
        let budget = 1024 + (64.0 * self.max_length) as usize;
        if length > self.max_length || self.outcome.traces[id].steps > budget {
            return self.lose(id);
        }
        let Some((exit, target, distance)) = map.exit(face, point, axis) else {
            return self.lose(id);
        };

        if let Some(hit) = self.first_hit(id, face, point, axis, distance, first) {
            let direction = axis_vector(axis);
            let at = [point[0] + hit.distance * direction[0], point[1] + hit.distance * direction[1]];
            self.lay(id, face, point, axis, hit.distance, at);
            let (other, time, turn) = {
                let segment = &self.segments[face][hit.segment];
                let along = (segment.axis % 2) as usize;
                let run =
                    ((at[along] - segment.start[along]) * axis_vector(segment.axis)[along]).clamp(0.0, segment.length);
                (segment.trace as usize, segment.time + run, (axis + 4 - segment.axis) % 4)
            };
            self.stop_on(id, other, time, turn, face, at);
            return false;
        }

        self.lay(id, face, point, axis, distance, target);
        match exit {
            Exit::Edge { edge, t } => {
                if let Some(wall) = self.walls[3 * face + edge] {
                    // A border or a hard edge: the trace stops on it.
                    let position = self.position(face, target);
                    let time = wall.time + wall.sense * t * wall.length;
                    self.stop_at(id, wall.trace as usize, time, (axis + 4 - wall.axis) % 4, position);
                    return false;
                }
                match map.cross_edge(face, edge, t, axis) {
                    Some((next, point, axis)) => {
                        let trace = &mut self.outcome.traces[id];
                        (trace.face, trace.point, trace.axis) = (next, point, axis);
                        true
                    }
                    None => self.lose(id),
                }
            }
            Exit::Vertex { corner } => self.reach_vertex(id, face, corner, axis),
        }
    }

    /// Records the stretch of trace `id` from `point` to `target` in `face`.
    fn lay(&mut self, id: usize, face: usize, point: [f64; 2], axis: Axis, distance: f64, target: [f64; 2]) {
        let position = self.position(face, target);
        let trace = &mut self.outcome.traces[id];
        if distance > 0.0 {
            self.segments[face].push(Segment {
                trace: id as u32,
                start: point,
                axis,
                length: distance,
                time: trace.length,
            });
        }
        if distance > 0.0 {
            trace.stretches.push(Stretch {
                face: face as u32,
                start: point,
                axis,
                length: distance,
                time: trace.length,
            });
        }
        trace.length += distance;
        trace.point = target;
        trace.path.push(position);
    }

    fn lose(&mut self, id: usize) -> bool {
        self.outcome.traces[id].running = false;
        false
    }

    fn position(&self, face: usize, point: [f64; 2]) -> Vec3 {
        let weights = self.map.barycentric(face, point);
        let triangle = self.map.mesh.triangles[face];
        (0..3).map(|k| self.map.mesh.positions[triangle[k] as usize] * weights[k]).sum()
    }

    /// The first stretch already laid in `face` that the move of trace `id`
    /// from `point` along `axis` over `distance` runs into.
    fn first_hit(
        &self,
        id: usize,
        face: usize,
        point: [f64; 2],
        axis: Axis,
        distance: f64,
        first: bool,
    ) -> Option<Hit> {
        let along = (axis % 2) as usize;
        let across = 1 - along;
        let sign = axis_vector(axis)[along];
        let mut best: Option<Hit> = None;
        for (index, segment) in self.segments[face].iter().enumerate() {
            // The traces that set off from the same singularity only touch
            // at their start.
            if first && segment.time == 0.0 && segment.start == point {
                continue;
            }
            let own = segment.trace as usize == id;
            let direction = axis_vector(segment.axis);
            let hit = if segment.axis % 2 != axis % 2 {
                // Across: the stretch runs along the other coordinate.
                let reach = (segment.start[along] - point[along]) * sign;
                let run = (point[across] - segment.start[across]) * direction[across];
                let within = (-EPSILON..=distance + EPSILON).contains(&reach)
                    && (-EPSILON..=segment.length + EPSILON).contains(&run);
                within.then(|| Hit { distance: reach.clamp(0.0, distance), segment: index, crossing: true })
            } else if !own && (point[across] - segment.start[across]).abs() <= EPSILON {
                // Along the same line: they collide if they share more than
                // a point of it.
                let from = (segment.start[along] - point[along]) * sign;
                let to = from + segment.length * direction[along] * sign;
                let (low, high) = (from.min(to).max(0.0), from.max(to).min(distance));
                (high - low > EPSILON).then_some(Hit { distance: low, segment: index, crossing: false })
            } else {
                None
            };
            let Some(hit) = hit else { continue };
            // Of two hits at the same place, the crossing is the one that
            // says what the node looks like.
            let better = best.as_ref().is_none_or(|kept| {
                hit.distance < kept.distance - EPSILON
                    || (hit.distance <= kept.distance + EPSILON && hit.crossing && !kept.crossing)
            });
            if better {
                best = Some(hit);
            }
        }
        best
    }

    /// Trace `id` arrives at corner `corner` of `face`.
    fn reach_vertex(&mut self, id: usize, face: usize, corner: usize, axis: Axis) -> bool {
        let map = self.map;
        let vertex = map.mesh.triangles[face][corner];
        let node = self.singular[vertex as usize];
        if node != INVALID {
            let key = self.key_at_singularity(node, face, corner, axis);
            self.finish(id, node, key);
            return false;
        }
        if let Some(passage) = self.passages[vertex as usize] {
            // Another trace has been through this vertex, or the border
            // has: this one stops on it.
            let (theirs, _, their_axis) = passage.departure;
            let Some(carried) = map.carry_around(face, corner, axis, theirs) else {
                return self.lose(id);
            };
            let turn = (carried + 4 - their_axis) % 4;
            let position = map.mesh.positions[vertex as usize];
            self.stop_at(id, passage.trace as usize, passage.time, turn, position);
            return false;
        }
        let Some(departure) = map.enter_from_vertex(face, corner, axis) else {
            return self.lose(id);
        };
        let trace = &mut self.outcome.traces[id];
        self.passages[vertex as usize] = Some(Passage { trace: id as u32, time: trace.length, departure });
        (trace.face, trace.axis) = (departure.0, departure.2);
        trace.point = map.uv[3 * departure.0 + departure.1];
        true
    }

    /// Trace `id`, at `at` in `face`, stops on trace `other` where that one
    /// had length `time`. `turn` is the direction of `id` counted in quarter
    /// turns from that of `other`.
    fn stop_on(&mut self, id: usize, other: usize, time: f64, turn: u8, face: usize, at: [f64; 2]) {
        // The ends of a trace may be singularities, where directions are not
        // numbered from the trace but round the vertex.
        if let Some(node) = self.singular_end(other, time) {
            let weights = self.map.barycentric(face, at);
            let corner = (0..3).max_by(|&a, &b| weights[a].total_cmp(&weights[b])).unwrap();
            let axis = self.outcome.traces[id].axis;
            let key = self.key_at_singularity(node, face, corner, axis);
            self.finish(id, node, key);
            return;
        }
        let position = self.position(face, at);
        self.stop_at(id, other, time, turn, position);
    }

    /// The singular node trace `other` has at length `time`, if any.
    fn singular_end(&self, other: usize, time: f64) -> Option<u32> {
        let trace = &self.outcome.traces[other];
        // (A singularity on a wall numbers its directions as walls do.)
        let singular = |node: u32| {
            let junction = &self.outcome.junctions[node as usize];
            junction.vertex.is_some() && !junction.corner
        };
        if time <= MERGE {
            return Some(trace.start.0).filter(|&node| singular(node));
        }
        match trace.end {
            Some((node, _)) if time >= trace.length - MERGE && singular(node) => Some(node),
            _ => None,
        }
    }

    /// Key, round singular node `node`, of the arc of a trace that arrives
    /// there at corner `corner` of `face` going along `axis`.
    fn key_at_singularity(&mut self, node: u32, face: usize, corner: usize, axis: Axis) -> f64 {
        let vertex = self.outcome.junctions[node as usize].vertex.unwrap();
        // Seen from the vertex, the arc leaves the way the trace came.
        let rank = self
            .map
            .canonical(face, corner, (axis + 2) % 4)
            .and_then(|departure| self.map.departures(vertex).into_iter().position(|candidate| candidate == departure));
        // A trace leaves along every direction of a singularity; one coming
        // in along the same line should have met it head-on.
        self.outcome.overlaps += 1;
        rank.map_or(-0.5, |rank| rank as f64 + 0.25)
    }

    /// Trace `id` stops on a regular point of trace `other`, where that one
    /// had length `time`.
    fn stop_at(&mut self, id: usize, other: usize, time: f64, turn: u8, position: Vec3) {
        let found = {
            let trace = &self.outcome.traces[other];
            let through = trace.through.iter().find(|&&(at, _, _)| (at - time).abs() <= MERGE);
            match (through, trace.end) {
                (Some(&(_, node, forward)), _) => Some((node, forward)),
                (None, _) if time <= MERGE => Some((trace.start.0, trace.start.1 as u8 % 4)),
                // The key of an end is that of the arc leaving the node,
                // which points back along the trace.
                (None, Some((node, key))) if time >= trace.length - MERGE => Some((node, (key as u8 + 2) % 4)),
                _ => None,
            }
        };
        let head_on = turn.is_multiple_of(2);
        let (node, forward) = found.unwrap_or_else(|| {
            let node = self.outcome.junctions.len() as u32;
            self.outcome.junctions.push(Junction::regular(position));
            let trace = &mut self.outcome.traces[other];
            if head_on && trace.running && time >= trace.length - MERGE {
                // Two traces meeting nose to nose: both end here.
                trace.running = false;
                trace.end = Some((node, 2.0));
                self.outcome.junctions[node as usize].used[2] = true;
            } else {
                trace.through.push((time, node, 0));
                self.outcome.junctions[node as usize].used[0] = true;
                self.outcome.junctions[node as usize].used[2] = true;
            }
            (node, 0)
        });
        let key = (forward + turn + 2) % 4;
        let used = &mut self.outcome.junctions[node as usize].used[key as usize];
        if *used {
            self.outcome.overlaps += 1;
        }
        *used = true;
        self.finish(id, node, key as f64);
    }

    fn finish(&mut self, id: usize, node: u32, key: f64) {
        let trace = &mut self.outcome.traces[id];
        trace.running = false;
        trace.end = Some((node, key));
    }
}

/// A length used as a priority.
#[derive(Clone, Copy, PartialEq)]
struct Time(f64);

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
