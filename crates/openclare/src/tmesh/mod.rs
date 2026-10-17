//! T-mesh (stage M5): the surface cut into rectangles of the seamless map.
//!
//! Its nodes are the singularities and the points where one cut stops on
//! another, a T-junction; its arcs are the stretches of cut between nodes,
//! each with its length in the map; its patches are what the cuts leave.
//! Giving every arc a whole length, the same on opposite sides of each
//! patch, is all that then separates the map from a quad mesh: that is the
//! next stage, quantization.
//!
//! The stage is a [`LayoutBuilder`]; chains get one from [`create`].

mod motorcycle;
pub(crate) mod walk;
mod walls;

use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

use crate::crossfield::CrossField;
use crate::error::{Error, Result};
use crate::math::{INVALID, Vec3};
use crate::mesh::Topology;
use crate::parametrization::{self, Parametrization};
use crate::preprocess::WorkMesh;

/// One direction of an arc: `2 * arc` runs from its first node to its
/// second, `2 * arc + 1` back.
pub type HalfArc = u32;

/// What a node of the T-mesh is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    /// A singularity of the field, on that vertex of the work mesh.
    Singularity { vertex: u32 },
    /// A vertex where a border or a hard edge turns, ends or meets another.
    Corner,
    /// A point where a cut stops on another one, on a border or on a hard
    /// edge.
    Junction,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub position: Vec3,
    pub kind: NodeKind,
    /// The half-arcs leaving it, counter-clockwise.
    pub arcs: Vec<HalfArc>,
}

#[derive(Clone, Debug)]
pub struct Arc {
    pub nodes: [u32; 2],
    /// Length in the map: in quad edges, not yet a whole number.
    pub length: f64,
    /// The patch on its left going from the first node to the second, and
    /// the one on its right.
    pub patches: [u32; 2],
    /// It runs along an open border, with the surface on its left: it has
    /// no patch on its right.
    pub border: bool,
    /// It runs along a border or a hard edge of the shape.
    pub feature: bool,
    /// The trace it is a piece of, and the lengths of that trace at its
    /// first and second node.
    pub trace: u32,
    pub span: [f64; 2],
}

/// A piece of a trace within one triangle of the work mesh.
#[derive(Clone, Copy, Debug)]
pub struct Stretch {
    pub face: u32,
    /// Where it starts, in the coordinates the map gives that triangle.
    pub start: [f64; 2],
    /// The grid direction it runs along there: 0 is +u, 1 is +v, 2 is -u,
    /// 3 is -v.
    pub axis: u8,
    pub length: f64,
    /// Length of the trace where the piece starts.
    pub time: f64,
}

/// A cut: a trace from a singularity or a corner, or a straight run of
/// border or hard edge.
#[derive(Clone, Debug)]
pub struct Trace {
    /// Its way on the surface, for display.
    pub points: Vec<Vec3>,
    /// Its way through the map.
    pub stretches: Vec<Stretch>,
    /// It is a run of border or hard edge.
    pub feature: bool,
    /// It is a run of open border.
    pub border: bool,
}

impl Trace {
    /// Where the trace is at length `time`: the triangle, the point in its
    /// coordinates, and the direction of the trace there.
    pub fn locate(&self, time: f64) -> Option<(usize, [f64; 2], u8)> {
        let after = self.stretches.partition_point(|stretch| stretch.time <= time);
        let stretch = self.stretches.get(after.max(1) - 1)?;
        let run = (time - stretch.time).clamp(0.0, stretch.length);
        let direction = walk::axis_vector(stretch.axis);
        let point = [stretch.start[0] + run * direction[0], stretch.start[1] + run * direction[1]];
        Some((stretch.face as usize, point, stretch.axis))
    }
}

#[derive(Clone, Debug)]
pub struct Patch {
    /// Its rim, counter-clockwise: the patch is on the left of each half-arc.
    pub boundary: Vec<HalfArc>,
    /// Interior angle at the end of each of those, in quarter turns: 1 at a
    /// corner, 2 where the rim runs straight past a T-junction.
    pub angles: Vec<u8>,
    /// Lengths of its four sides in rim order, if it is a rectangle: four
    /// corners and nothing else but straight stretches.
    pub sides: Option<[f64; 4]>,
}

/// What the builder achieved, and what quantization is in for.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    /// Triangles folded over in the map the traces were run on.
    pub map_inversions: usize,
    /// Border edges and hard edges that do not run along a grid line of the
    /// map.
    pub off_grid_edges: usize,
    /// Triangles of pieces of the surface that nothing cuts (no singularity,
    /// no border: a torus), and that the partition therefore leaves out.
    pub uncut_triangles: usize,
    pub singularities: usize,
    pub traces: usize,
    /// Traces that never met another.
    pub lost_traces: usize,
    /// Traces that ended on top of another arc instead of across it.
    pub overlaps: usize,
    pub nodes: usize,
    pub t_junctions: usize,
    pub arcs: usize,
    pub patches: usize,
    pub rectangles: usize,
    /// Largest difference between the two opposite sides of a rectangle, in
    /// quad edges.
    pub side_mismatch: f64,
    /// Area of the rectangles over the area of the map, minus one.
    pub area_error: f64,
    /// Nodes minus arcs plus patches, and what that must equal: the Euler
    /// characteristic of the surface.
    pub euler: i64,
    pub surface_euler: i64,
    pub longest_trace: f64,
    /// Arcs shorter than half a quad edge, and rectangles narrower than
    /// that: what quantization will have to collapse or widen.
    pub short_arcs: usize,
    pub thin_patches: usize,
}

impl Report {
    /// Whether the partition is one into rectangles, all of the surface and
    /// nothing else.
    pub fn is_valid(&self) -> bool {
        self.lost_traces == 0
            && self.overlaps == 0
            && self.off_grid_edges == 0
            && self.rectangles == self.patches
            && self.euler == self.surface_euler
            && self.area_error.abs() < 1e-6
            // (Lengths are read off a map solved to about 1e-9; a ten-
            // thousandth of a quad edge is still nothing.)
            && self.side_mismatch < 1e-4
    }
}

#[derive(Clone, Debug)]
pub struct TMesh {
    pub nodes: Vec<Node>,
    pub arcs: Vec<Arc>,
    pub patches: Vec<Patch>,
    pub traces: Vec<Trace>,
    pub report: Report,
}

impl TMesh {
    /// Runs the motorcycle graph of `map` from the singularities of `field`.
    /// No trace goes further than `max_trace_length` quad edges.
    pub fn trace(work: &WorkMesh, field: &CrossField, map: &Parametrization, max_trace_length: f64) -> TMesh {
        let mesh = &work.mesh;
        let topology = Topology::build(mesh);
        let walk = walk::Map::new(mesh, &topology, &map.uv, &map.cut.seam, &map.cut.transition);
        let is_singular: Vec<bool> = field.indices.iter().map(|&index| index != 0).collect();
        // Cuts that are there from the start: open borders, and the edges
        // the work mesh marks as features of the shape.
        let mut features: std::collections::HashSet<(u32, u32)> =
            work.feature_edges.iter().map(|&[a, b]| (a.min(b), a.max(b))).collect();
        // A guide curve the map could not keep on a grid line is no wall:
        // the field still runs along it, and that is all that is left of it.
        for &[a, b] in &map.report.loose_guides {
            features.remove(&(a.min(b), a.max(b)));
        }
        let is_wall: Vec<bool> = (0..3 * mesh.triangles.len())
            .map(|h| {
                let (a, b) = (mesh.triangles[h / 3][h % 3], mesh.triangles[h / 3][(h % 3 + 1) % 3]);
                topology.opposite[h] == INVALID || features.contains(&(a.min(b), a.max(b)))
            })
            .collect();
        // Pieces of the surface with neither singularity nor wall (a ring, a
        // torus) are cut from one of their vertices, any.
        let mut cut = is_singular.clone();
        for h in (0..is_wall.len()).filter(|&h| is_wall[h]) {
            cut[mesh.triangles[h / 3][h % 3] as usize] = true;
        }
        let reached = faces_reached_from(&topology, mesh, &cut);
        let mut seeds: Vec<u32> = Vec::new();
        let mut seen = reached.clone();
        for first in 0..mesh.triangles.len() {
            if seen[first] {
                continue;
            }
            seeds.push(mesh.triangles[first][0]);
            // Flag the whole piece of this face as dealt with.
            let mut stack = vec![first];
            seen[first] = true;
            while let Some(f) = stack.pop() {
                for h in 3 * f..3 * f + 3 {
                    let twin = topology.opposite[h];
                    if twin != INVALID && !seen[twin as usize / 3] {
                        seen[twin as usize / 3] = true;
                        stack.push(twin as usize / 3);
                    }
                }
            }
        }
        let outcome = motorcycle::race(&walk, &is_singular, &is_wall, &seeds, max_trace_length);

        let mut tmesh = TMesh::assemble(&outcome);
        let report = &mut tmesh.report;
        // (Faces that are flat in the map do not count: slivers it was not
        // asked to copy, and triangles with two edges on one feature line,
        // which that line flattens. A trace crosses them in no distance.)
        report.map_inversions = map.turned_over().len();
        report.surface_euler = mesh.positions.len() as i64 - topology.edge_count() as i64 + mesh.triangles.len() as i64;
        for &seed in &seeds {
            cut[seed as usize] = true;
        }
        let covered = faces_reached_from(&topology, mesh, &cut);
        report.uncut_triangles = covered.iter().filter(|&&reached| !reached).count();
        let map_area: f64 = (0..mesh.triangles.len()).filter(|&f| covered[f]).map(|f| walk.area(f)).sum();
        let rectangles: f64 = tmesh.patches.iter().filter_map(|patch| patch.sides).map(area_of).sum();
        report.area_error = if map_area > 0.0 { rectangles / map_area - 1.0 } else { 0.0 };
        tmesh
    }

    /// Nodes, arcs and patches from the traces of a race.
    fn assemble(outcome: &motorcycle::Outcome) -> TMesh {
        let mut arcs: Vec<Arc> = Vec::new();
        // Per node: its half-arcs with the key that orders them round it.
        let mut around: Vec<Vec<(f64, HalfArc)>> = vec![Vec::new(); outcome.junctions.len()];
        let mut longest_trace = 0.0f64;
        for (index, trace) in outcome.traces.iter().enumerate() {
            if trace.merged.is_some() {
                continue;
            }
            // Stops along the trace: length there, node, key of the arc
            // towards the next stop, key of the arc towards the previous one.
            let mut stops = vec![(0.0, trace.start.0, trace.start.1, f64::NAN)];
            let mut through = trace.through.clone();
            through.sort_by(|a, b| a.0.total_cmp(&b.0));
            stops.extend(
                through.iter().map(|&(at, node, forward)| (at, node, forward as f64, ((forward + 2) % 4) as f64)),
            );
            if let Some((node, key)) = trace.end {
                stops.push((trace.length, node, f64::NAN, key));
                longest_trace = longest_trace.max(trace.length);
            }
            // The tail of a lost trace, past its last stop, is dropped.
            for pair in stops.windows(2) {
                let (from, to) = (pair[0], pair[1]);
                let arc = arcs.len() as u32;
                let length = to.0 - from.0;
                arcs.push(Arc {
                    nodes: [from.1, to.1],
                    length,
                    patches: [INVALID; 2],
                    border: trace.border,
                    feature: trace.wall,
                    trace: index as u32,
                    span: [from.0, to.0],
                });
                around[from.1 as usize].push((from.2, 2 * arc));
                around[to.1 as usize].push((to.3, 2 * arc + 1));
            }
        }
        for list in &mut around {
            list.sort_by(|a, b| a.0.total_cmp(&b.0));
        }

        let full_turn: Vec<f64> = outcome.junctions.iter().map(|junction| junction.full_turn).collect();

        // Where each half-arc sits round the node it leaves.
        let mut slot = vec![0usize; 2 * arcs.len()];
        for list in &around {
            for (index, &(_, half)) in list.iter().enumerate() {
                slot[half as usize] = index;
            }
        }
        let origin = |half: HalfArc| arcs[half as usize / 2].nodes[half as usize % 2] as usize;

        // Each patch is found by walking its rim with the patch on the left:
        // at every node, the next half-arc is the first one clockwise from
        // the one just walked.
        let mut patches: Vec<Patch> = Vec::new();
        let mut patch_of = vec![INVALID; 2 * arcs.len()];
        for first in 0..2 * arcs.len() as u32 {
            if patch_of[first as usize] != INVALID {
                continue;
            }
            let (mut boundary, mut angles) = (Vec::new(), Vec::new());
            let mut half = first;
            loop {
                patch_of[half as usize] = patches.len() as u32;
                boundary.push(half);
                let back = half ^ 1;
                let node = origin(back);
                let list = &around[node];
                let at = slot[back as usize];
                let (key, next) = list[(at + list.len() - 1) % list.len()];
                let turn = (list[at].0 - key).rem_euclid(full_turn[node]);
                let angle = if list.len() == 1 || turn == 0.0 { full_turn[node] } else { turn };
                // Anything that is not a whole number of quarter turns comes
                // from traces lying on each other; 0 marks it.
                angles.push(if angle.fract() == 0.0 { angle as u8 } else { 0 });
                half = next;
                if half == first {
                    break;
                }
            }
            // Walked against a border, the rim is that of a hole of the
            // surface, not of a patch.
            if boundary.iter().any(|&half| half % 2 == 1 && arcs[half as usize / 2].border) {
                for half in boundary {
                    patch_of[half as usize] = OUTSIDE;
                }
                continue;
            }
            let sides = rectangle_sides(&boundary, &angles, &arcs);
            patches.push(Patch { boundary, angles, sides });
        }
        for (half, &patch) in patch_of.iter().enumerate() {
            arcs[half / 2].patches[half % 2] = if patch == OUTSIDE { INVALID } else { patch };
        }

        let nodes: Vec<Node> = outcome
            .junctions
            .iter()
            .zip(&around)
            .map(|(junction, list)| Node {
                position: junction.position,
                kind: match junction.vertex {
                    Some(vertex) => NodeKind::Singularity { vertex },
                    None if junction.corner => NodeKind::Corner,
                    None => NodeKind::Junction,
                },
                arcs: list.iter().map(|&(_, half)| half).collect(),
            })
            .collect();

        let mut report = Report {
            singularities: nodes.iter().filter(|node| matches!(node.kind, NodeKind::Singularity { .. })).count(),
            traces: outcome.traces.iter().filter(|trace| !trace.wall).count(),
            lost_traces: outcome.lost,
            overlaps: outcome.overlaps,
            off_grid_edges: outcome.off_grid_edges,
            nodes: nodes.len(),
            t_junctions: nodes.iter().filter(|node| node.kind == NodeKind::Junction && node.arcs.len() == 3).count(),
            arcs: arcs.len(),
            patches: patches.len(),
            rectangles: patches.iter().filter(|patch| patch.sides.is_some()).count(),
            euler: nodes.len() as i64 - arcs.len() as i64 + patches.len() as i64,
            longest_trace,
            short_arcs: arcs.iter().filter(|arc| arc.length < 0.5).count(),
            ..Default::default()
        };
        for sides in patches.iter().filter_map(|patch| patch.sides) {
            report.side_mismatch =
                report.side_mismatch.max((sides[0] - sides[2]).abs()).max((sides[1] - sides[3]).abs());
            if sides[0].min(sides[1]) < 0.5 {
                report.thin_patches += 1;
            }
        }
        let traces = outcome
            .traces
            .iter()
            .map(|trace| Trace {
                points: trace.path.clone(),
                stretches: trace.stretches.clone(),
                feature: trace.wall,
                border: trace.border,
            })
            .collect();
        TMesh { nodes, arcs, patches, traces, report }
    }
}

/// Marks the half-arcs that face a hole of the surface.
const OUTSIDE: u32 = INVALID - 1;

impl Patch {
    /// Which of the four sides each half-arc of the rim lies on, for a
    /// rectangle: the index into [`Patch::sides`].
    pub fn side_of_each(&self) -> Option<Vec<u8>> {
        self.sides?;
        let start = (self.angles.iter().position(|&angle| angle == 1)? + 1) % self.boundary.len();
        let mut side_of = vec![0; self.boundary.len()];
        let mut side = 0;
        for step in 0..self.boundary.len() {
            let at = (start + step) % self.boundary.len();
            side_of[at] = side;
            if self.angles[at] == 1 {
                side += 1;
            }
        }
        Some(side_of)
    }
}

fn area_of(sides: [f64; 4]) -> f64 {
    0.25 * (sides[0] + sides[2]) * (sides[1] + sides[3])
}

/// The four side lengths of a rim with exactly four corners and otherwise
/// straight; `None` for anything else.
fn rectangle_sides(boundary: &[HalfArc], angles: &[u8], arcs: &[Arc]) -> Option<[f64; 4]> {
    let corners = angles.iter().filter(|&&angle| angle == 1).count();
    if corners != 4 || angles.iter().any(|&angle| angle != 1 && angle != 2) {
        return None;
    }
    // Start right after a corner, so that each corner closes a side.
    let start = (angles.iter().position(|&angle| angle == 1)? + 1) % boundary.len();
    let mut sides = [0.0; 4];
    let mut side = 0;
    for step in 0..boundary.len() {
        let at = (start + step) % boundary.len();
        sides[side] += arcs[boundary[at] as usize / 2].length;
        if angles[at] == 1 {
            side += 1;
        }
    }
    Some(sides)
}

/// The faces of every connected piece of the surface holding at least one
/// flagged vertex.
fn faces_reached_from(topology: &Topology, mesh: &crate::mesh::TriMesh, flagged: &[bool]) -> Vec<bool> {
    let mut reached = vec![false; mesh.triangles.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
    for (f, triangle) in mesh.triangles.iter().enumerate() {
        if triangle.iter().any(|&v| flagged[v as usize]) && !reached[f] {
            reached[f] = true;
            queue.push_back(f);
        }
    }
    while let Some(f) = queue.pop_front() {
        for h in 3 * f..3 * f + 3 {
            let twin = topology.opposite[h];
            if twin != INVALID && !reached[twin as usize / 3] {
                reached[twin as usize / 3] = true;
                queue.push_back(twin as usize / 3);
            }
        }
    }
    reached
}

#[derive(Clone, Debug)]
pub struct Request {
    /// Edge length of the quads to come: one unit of the map on the surface.
    pub edge_length: f64,
}

/// The global structure of the quad mesh, with real lengths, and the map it
/// was read off.
pub struct Layout {
    pub tmesh: TMesh,
    pub map: Parametrization,
}

/// Builds the global structure of the quad mesh, with real lengths.
pub trait LayoutBuilder {
    fn kind(&self) -> Kind;
    fn build(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Layout>;
}

/// The layout builders [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Seamless map, then the motorcycle graph of its singularities (Campen
    /// et al. 2015). For organic shapes.
    Motorcycle,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Motorcycle];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Motorcycle => "motorcycle",
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
            Error::InvalidParams(format!("unknown layout builder '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the layout builder of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn LayoutBuilder> {
    match kind {
        Kind::Motorcycle => Box::new(Motorcycle::default()),
    }
}

#[derive(Clone, Debug)]
pub struct Motorcycle {
    /// Most stiffening rounds against inverted triangles in the map.
    pub stiffening_rounds: usize,
    /// How far a trace may go before it is given up, in times the side of a
    /// square with the area of the map.
    pub max_trace_length: f64,
}

impl Default for Motorcycle {
    fn default() -> Self {
        Self { stiffening_rounds: 40, max_trace_length: 20.0 }
    }
}

impl LayoutBuilder for Motorcycle {
    fn kind(&self) -> Kind {
        Kind::Motorcycle
    }

    fn build(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Layout> {
        let map_request =
            parametrization::Request { edge_length: request.edge_length, stiffening_rounds: self.stiffening_rounds };
        let map = parametrization::create(parametrization::Kind::Seamless).parametrize(work, field, &map_request)?;
        let side = (work.mesh.surface_area().sqrt() / request.edge_length).max(1.0);
        let tmesh = TMesh::trace(work, field, &map, self.max_trace_length * side);
        Ok(Layout { tmesh, map })
    }
}
