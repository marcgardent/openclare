//! The quad mesh a quantized T-mesh stands for, as pure connectivity.
//!
//! Every arc of whole length `n` carries `n + 1` points, every rectangle of
//! `w` by `h` a grid of them. Where a length is zero, points merge: the two
//! ends of an arc, and the two long sides of a rectangle with no width. What
//! is left after merging is the quad mesh, and building it is the one sure
//! way to tell whether the lengths are sound: it must come out a manifold,
//! with the singularities of the field for its only irregular vertices.

use std::collections::{BTreeMap, VecDeque};

use crate::tmesh::{NodeKind, TMesh};

/// Where a vertex of the quad mesh comes from in the T-mesh.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Node(u32),
    /// `step` edges along the arc from its first node.
    Arc {
        arc: u32,
        step: u32,
    },
    /// `i` edges along the first side of the patch, `j` along the second.
    Patch {
        patch: u32,
        i: u32,
        j: u32,
    },
}

/// Connectivity of the quad mesh of a quantized T-mesh.
#[derive(Clone, Debug, Default)]
pub struct Lattice {
    /// One source per vertex: the first of the points merged into it.
    pub sources: Vec<Source>,
    /// Counter-clockwise, like the patches.
    pub quads: Vec<[u32; 4]>,
    /// The vertex each node of the T-mesh became.
    pub node_vertex: Vec<u32>,
}

/// A merge of two points: which, the arc to lengthen to undo it, and the
/// real size of what was closed up.
struct Merge {
    points: [u32; 2],
    arc: usize,
    size: f64,
}

/// A lattice with what it takes to trace a defect back to a length.
pub(super) struct Built {
    pub lattice: Lattice,
    /// Vertex of each point, before merging.
    vertex_of: Vec<u32>,
    merges: Vec<Merge>,
    /// Points on an open border.
    on_border: Vec<bool>,
}

pub(super) fn build(tmesh: &TMesh, x: &[i64]) -> Built {
    let nodes = tmesh.nodes.len();
    // Points: the nodes, then those strictly inside each arc, then those
    // strictly inside each patch.
    let mut first_in_arc = Vec::with_capacity(x.len());
    let mut points = nodes as u32;
    for &length in x {
        first_in_arc.push(points);
        points += (length - 1).max(0) as u32;
    }
    let along = |half: u32, step: i64| {
        let arc = half as usize / 2;
        let step = if half.is_multiple_of(2) { step } else { x[arc] - step };
        if step == 0 {
            tmesh.arcs[arc].nodes[0]
        } else if step == x[arc] {
            tmesh.arcs[arc].nodes[1]
        } else {
            first_in_arc[arc] + step as u32 - 1
        }
    };

    let mut sources: Vec<Source> = (0..nodes as u32).map(Source::Node).collect();
    for (arc, &length) in x.iter().enumerate() {
        sources.extend((1..length).map(|step| Source::Arc { arc: arc as u32, step: step as u32 }));
    }
    let mut on_border = vec![false; points as usize];
    for (arc, _) in tmesh.arcs.iter().enumerate().filter(|(_, data)| data.border) {
        for step in 0..=x[arc] {
            on_border[along(2 * arc as u32, step) as usize] = true;
        }
    }

    let mut merges = Vec::new();
    for (arc, data) in tmesh.arcs.iter().enumerate() {
        if x[arc] == 0 {
            merges.push(Merge { points: data.nodes, arc, size: data.length });
        }
    }

    let mut quads = Vec::new();
    for (index, patch) in tmesh.patches.iter().enumerate() {
        let (Some(side_of), Some(real)) = (patch.side_of_each(), patch.sides) else { continue };
        // The points along each side, corner to corner, and the arc of each
        // pair of opposite sides that is cheapest to lengthen.
        let mut sides: [Vec<u32>; 4] = Default::default();
        let mut longest: [Option<usize>; 2] = [None; 2];
        // Round the rim from the first corner, so that each side is met in
        // one piece.
        let count = patch.boundary.len();
        let start = (0..count).find(|&at| side_of[at] == 0 && side_of[(at + count - 1) % count] == 3).unwrap_or(0);
        for at in (0..count).map(|step| (start + step) % count) {
            let (half, side) = (patch.boundary[at], side_of[at]);
            let arc = half as usize / 2;
            let along_side = &mut sides[side as usize];
            if along_side.is_empty() {
                along_side.push(along(half, 0));
            }
            along_side.extend((1..=x[arc]).map(|step| along(half, step)));
            let kept = &mut longest[side as usize % 2];
            if kept.is_none_or(|other| tmesh.arcs[arc].length > tmesh.arcs[other].length) {
                *kept = Some(arc);
            }
        }
        // A side left without any arc cannot be: the rim has four corners.
        if sides.iter().any(Vec::is_empty) {
            continue;
        }
        let (w, h) = (sides[0].len() - 1, sides[1].len() - 1);
        if sides[2].len() != w + 1 || sides[3].len() != h + 1 {
            continue;
        }
        if w == 0 || h == 0 {
            // No area: the two sides that still have length fold onto each
            // other, running opposite ways round the rim.
            let (a, b, across) = if w == 0 { (1, 3, 0) } else { (0, 2, 1) };
            let Some(arc) = longest[across] else { continue };
            for (step, &point) in sides[a].iter().enumerate() {
                let facing = sides[b][sides[b].len() - 1 - step];
                merges.push(Merge { points: [point, facing], arc, size: real[across] });
            }
            continue;
        }
        let first_inside = points;
        for j in 1..h {
            for i in 1..w {
                sources.push(Source::Patch { patch: index as u32, i: i as u32, j: j as u32 });
            }
        }
        points += ((w - 1) * (h - 1)) as u32;
        on_border.resize(points as usize, false);
        let at = |i: usize, j: usize| {
            if j == 0 {
                sides[0][i]
            } else if i == w {
                sides[1][j]
            } else if j == h {
                sides[2][w - i]
            } else if i == 0 {
                sides[3][h - j]
            } else {
                first_inside + ((j - 1) * (w - 1) + (i - 1)) as u32
            }
        };
        for j in 0..h {
            for i in 0..w {
                quads.push([at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)]);
            }
        }
    }

    let mut classes = Classes::new(points as usize);
    for merge in &merges {
        classes.union(merge.points[0], merge.points[1]);
    }
    // Number the classes in order of their first point.
    let mut vertex_of = vec![u32::MAX; points as usize];
    let mut kept_sources = Vec::new();
    for point in 0..points {
        let root = classes.find(point) as usize;
        if vertex_of[root] == u32::MAX {
            vertex_of[root] = kept_sources.len() as u32;
            kept_sources.push(sources[root]);
        }
        vertex_of[point as usize] = vertex_of[root];
    }
    for quad in &mut quads {
        *quad = quad.map(|point| vertex_of[point as usize]);
    }
    let lattice = Lattice { sources: kept_sources, quads, node_vertex: vertex_of[..nodes].to_vec() };
    Built { lattice, vertex_of, merges, on_border }
}

/// What is wrong with a lattice.
#[derive(Default)]
pub(super) struct Defects {
    /// Arcs to keep from having no length, one per defect that some merge
    /// explains.
    pub reopen: Vec<usize>,
    /// Defects no merge explains: the T-mesh itself must be at fault.
    pub unexplained: usize,
}

impl Defects {
    pub fn is_empty(&self) -> bool {
        self.reopen.is_empty() && self.unexplained == 0
    }
}

impl Built {
    /// Checks that the lattice is a manifold quad mesh whose irregular
    /// vertices are the singularities of the T-mesh, each with its valence
    /// and none on a border.
    pub fn defects(&self, tmesh: &TMesh) -> Defects {
        let lattice = &self.lattice;
        let vertices = lattice.sources.len();
        let mut defects = Defects::default();
        // Whether a defect was already charged to each vertex.
        let mut blamed = vec![false; vertices];
        // Charges a defect at `vertex` to the arc `culprit` finds, once.
        let mut charge = |defects: &mut Defects, vertex: u32, culprit: &dyn Fn() -> Option<usize>| {
            if !std::mem::replace(&mut blamed[vertex as usize], true) {
                match culprit() {
                    Some(arc) => defects.reopen.push(arc),
                    None => defects.unexplained += 1,
                }
            }
        };

        // Nodes that must each stay a vertex of their own.
        let mut anchor: Vec<Option<u32>> = vec![None; vertices];
        let mut expected_valence: Vec<Option<usize>> = vec![None; vertices];
        for (node, data) in tmesh.nodes.iter().enumerate() {
            if data.kind == NodeKind::Junction {
                continue;
            }
            let vertex = lattice.node_vertex[node] as usize;
            if let Some(other) = anchor[vertex] {
                // Two of them in one vertex: reopen something between them.
                charge(&mut defects, vertex as u32, &|| {
                    self.largest_merge_between(other, |point| point == node as u32)
                });
                continue;
            }
            anchor[vertex] = Some(node as u32);
            if matches!(data.kind, NodeKind::Singularity { .. }) {
                expected_valence[vertex] = Some(data.arcs.len());
            }
        }

        // Each directed edge belongs to one quad at most.
        let mut owner: BTreeMap<(u32, u32), u32> = BTreeMap::new();
        for (index, quad) in lattice.quads.iter().enumerate() {
            for k in 0..4 {
                let (a, b) = (quad[k], quad[(k + 1) % 4]);
                if a == b || quad[(k + 2) % 4] == a || owner.insert((a, b), index as u32).is_some() {
                    charge(&mut defects, a, &|| self.largest_merge_at(a));
                    charge(&mut defects, b, &|| self.largest_merge_at(b));
                }
            }
        }

        // Round each vertex, the quads form a single fan: closed inside the
        // surface, open on a border.
        let mut corners: Vec<Vec<(u32, u32)>> = vec![Vec::new(); vertices];
        for quad in &lattice.quads {
            for k in 0..4 {
                corners[quad[k] as usize].push((quad[(k + 1) % 4], quad[(k + 3) % 4]));
            }
        }
        for vertex in 0..vertices as u32 {
            let around = &corners[vertex as usize];
            if around.is_empty() {
                // Everything round it has closed up.
                charge(&mut defects, vertex, &|| self.largest_merge_at(vertex));
                continue;
            }
            // A quad leaves along `next` and arrives along `previous`; the
            // quad across an edge arrives along the one this one leaves by.
            let unmatched =
                around.iter().filter(|&&(next, _)| !around.iter().any(|&(_, previous)| previous == next)).count();
            let border = unmatched > 0;
            let mut fan = 1;
            let mut at = around[0];
            while let Some(&following) = around.iter().find(|&&(_, previous)| previous == at.0) {
                if following == around[0] || fan > around.len() {
                    break;
                }
                fan += 1;
                at = following;
            }
            if border {
                // An open fan was entered somewhere in its middle: walk the
                // other way too.
                let mut at = around[0];
                while let Some(&preceding) = around.iter().find(|&&(next, _)| next == at.1) {
                    if preceding == around[0] || fan > around.len() {
                        break;
                    }
                    fan += 1;
                    at = preceding;
                }
            }
            let valence = around.len() + border as usize;
            let sound = fan == around.len()
                && unmatched <= 1
                && match expected_valence[vertex as usize] {
                    Some(expected) => !border && valence == expected,
                    None => border || valence == 4,
                };
            if sound {
                continue;
            }
            // A singularity on a border is undone by what brought it there.
            let on_border = |point: u32| self.on_border[point as usize];
            match anchor[vertex as usize].filter(|_| expected_valence[vertex as usize].is_some() && border) {
                Some(node) => charge(&mut defects, vertex, &|| self.largest_merge_between(node, on_border)),
                None => charge(&mut defects, vertex, &|| self.largest_merge_at(vertex)),
            }
        }
        defects.reopen.sort_unstable();
        defects.reopen.dedup();
        defects
    }

    /// Vertices minus edges plus quads.
    pub fn euler(&self) -> i64 {
        let mut edges: Vec<(u32, u32)> = self
            .lattice
            .quads
            .iter()
            .flat_map(|quad| (0..4).map(move |k| (quad[k].min(quad[(k + 1) % 4]), quad[k].max(quad[(k + 1) % 4]))))
            .collect();
        edges.sort_unstable();
        edges.dedup();
        self.lattice.sources.len() as i64 - edges.len() as i64 + self.lattice.quads.len() as i64
    }

    /// The arc undoing the largest thing closed up into `vertex`.
    fn largest_merge_at(&self, vertex: u32) -> Option<usize> {
        self.merges
            .iter()
            .filter(|merge| self.vertex_of[merge.points[0] as usize] == vertex)
            .max_by(|a, b| a.size.total_cmp(&b.size).then(b.arc.cmp(&a.arc)))
            .map(|merge| merge.arc)
    }

    /// The arc undoing the largest thing closed up on a shortest chain of
    /// merges from point `from` to a point accepted by `reached`.
    fn largest_merge_between(&self, from: u32, reached: impl Fn(u32) -> bool) -> Option<usize> {
        let vertex = self.vertex_of[from as usize];
        let mut links: BTreeMap<u32, Vec<(u32, usize)>> = BTreeMap::new();
        for (index, merge) in self.merges.iter().enumerate() {
            if self.vertex_of[merge.points[0] as usize] == vertex {
                links.entry(merge.points[0]).or_default().push((merge.points[1], index));
                links.entry(merge.points[1]).or_default().push((merge.points[0], index));
            }
        }
        let mut came_by: BTreeMap<u32, (u32, usize)> = BTreeMap::new();
        let mut queue = VecDeque::from([from]);
        let mut end = None;
        while let Some(point) = queue.pop_front() {
            if point != from && reached(point) {
                end = Some(point);
                break;
            }
            for &(other, index) in links.get(&point).map_or(&[][..], Vec::as_slice) {
                if other != from && !came_by.contains_key(&other) {
                    came_by.insert(other, (point, index));
                    queue.push_back(other);
                }
            }
        }
        let mut best: Option<&Merge> = None;
        let mut point = end?;
        while let Some(&(before, index)) = came_by.get(&point) {
            let merge = &self.merges[index];
            if best.is_none_or(|kept| merge.size > kept.size) {
                best = Some(merge);
            }
            point = before;
        }
        best.map(|merge| merge.arc)
    }
}

/// Disjoint sets of points.
struct Classes {
    parent: Vec<u32>,
}

impl Classes {
    fn new(n: usize) -> Self {
        Self { parent: (0..n as u32).collect() }
    }

    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let grandparent = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = grandparent;
            x = grandparent;
        }
        x
    }

    fn union(&mut self, a: u32, b: u32) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b {
            self.parent[a.max(b) as usize] = a.min(b);
        }
    }
}
