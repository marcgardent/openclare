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
    /// Every point before merging, and the vertex it went into.
    pub points: Vec<Source>,
    pub point_vertex: Vec<u32>,
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
    /// Arcs to lengthen because, as things stand, a rectangle between two
    /// runs of border or hard edge has closed up and folded one onto the
    /// other.
    clashes: Vec<usize>,
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
    let mut merges = Vec::new();
    for (arc, data) in tmesh.arcs.iter().enumerate() {
        if x[arc] == 0 {
            merges.push(Merge { points: data.nodes, arc, size: data.length });
        }
    }

    let mut quads = Vec::new();
    let mut clashes = Vec::new();
    for (index, patch) in tmesh.patches.iter().enumerate() {
        let (Some(side_of), Some(real)) = (patch.side_of_each(), patch.sides) else { continue };
        // The points along each side, corner to corner, and the arc of each
        // pair of opposite sides that is cheapest to lengthen.
        let mut sides: [Vec<u32>; 4] = Default::default();
        let mut longest: [Option<usize>; 2] = [None; 2];
        // Whether each side still has some length of border or hard edge.
        let mut walled = [false; 4];
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
            walled[side as usize] |= tmesh.arcs[arc].feature && x[arc] > 0;
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
            if walled[a] && walled[b] {
                clashes.push(arc);
            }
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
    let lattice = Lattice {
        sources: kept_sources,
        quads,
        node_vertex: vertex_of[..nodes].to_vec(),
        points: sources,
        point_vertex: vertex_of.clone(),
    };
    Built { lattice, vertex_of, merges, clashes }
}

/// Largest valence a vertex inside the surface may have. The singularity
/// stage spreads every index beyond a quarter turn over free neighbours, so
/// the field asks for valences 3 and 5; an index of -2 (valence 6) is left
/// only where no free neighbour is found (the ogre has such vertices before
/// the split, ALGORITHM.md, M2), and merged singularities make 3 to 5.
/// Beyond 6 the field never asks, so a vertex there is a fault. Below 3 is
/// one as well: two quads that share two edges.
pub(super) const MOST_VALENCE: usize = 6;

/// What is wrong with a lattice.
#[derive(Default)]
pub(super) struct Defects {
    /// Arcs to keep from having no length, one per defect that some merge
    /// explains.
    pub reopen: Vec<usize>,
    /// Defects no merge explains: the T-mesh itself must be at fault.
    pub unexplained: usize,
    /// Singularities inside the surface with the valence their T-mesh node
    /// gives them, where that is not `4 - index` or not within 3 to
    /// [`MOST_VALENCE`]: the map sent out another number of arcs than the
    /// field's index asks for (#279), or the field asks for a valence no
    /// quad mesh should have. No length mends that, so [`Defects::is_empty`]
    /// leaves them out: it judges lengths, and any lengths of this T-mesh
    /// would fail. The quantization reports them as not separated.
    pub wrong_singularities: usize,
}

impl Defects {
    /// Nothing that other lengths could mend.
    pub fn is_empty(&self) -> bool {
        self.reopen.is_empty() && self.unexplained == 0
    }
}

impl Built {
    /// Checks that the lattice is a manifold quad mesh whose irregular
    /// vertices are the singularities of the T-mesh, each with the valence
    /// its index asks for unless it came to lie on a border, and no vertex
    /// inside the surface of valence under 3 or over [`MOST_VALENCE`]. With
    /// `merging`, singularities may share a vertex whose valence their
    /// indices add up to, if that is 3, 4 or 5 and the Euler characteristic
    /// is kept (ALGORITHM.md, M6, "Merging singularities").
    pub fn defects(&self, tmesh: &TMesh, merging: bool) -> Defects {
        let lattice = &self.lattice;
        let vertices = lattice.sources.len();
        let mut defects = Defects { reopen: self.clashes.clone(), unexplained: 0, wrong_singularities: 0 };
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

        // Singularities must each stay a vertex of their own. (Corners of
        // the border need not: two of them a hair apart are a step in the
        // border too small to keep, and the border is as well without it.)
        let mut singularity: Vec<Option<u32>> = vec![None; vertices];
        let mut expected_valence: Vec<Option<i64>> = vec![None; vertices];
        // The valence the node of a lone singularity gives its vertex: one
        // edge per arc.
        let mut node_valence: Vec<Option<usize>> = vec![None; vertices];
        let index_of = |node: usize| match tmesh.nodes[node].kind {
            NodeKind::Singularity { index, .. } => index as i64,
            _ => 0,
        };
        // With `merging`: the singularities come into each vertex, and the
        // valence their indices add up to.
        let mut merged: BTreeMap<u32, (Vec<u32>, i64)> = BTreeMap::new();
        for (node, data) in tmesh.nodes.iter().enumerate() {
            if !matches!(data.kind, NodeKind::Singularity { .. }) {
                continue;
            }
            let vertex = lattice.node_vertex[node] as usize;
            if let Some(other) = singularity[vertex] {
                if merging {
                    let (nodes, valence) =
                        merged.entry(vertex as u32).or_insert_with(|| (vec![other], 4 - index_of(other as usize)));
                    nodes.push(node as u32);
                    *valence -= index_of(node);
                    node_valence[vertex] = None;
                    continue;
                }
                // Two of them in one vertex: reopen something between them.
                charge(&mut defects, vertex as u32, &|| {
                    self.largest_merge_between(other, |point| point == node as u32)
                });
                continue;
            }
            singularity[vertex] = Some(node as u32);
            expected_valence[vertex] = Some(4 - index_of(node));
            node_valence[vertex] = Some(data.arcs.len());
        }
        for (&vertex, (nodes, valence)) in &merged {
            if (3..=5).contains(valence) {
                expected_valence[vertex as usize] = Some(*valence);
            } else {
                charge(&mut defects, vertex, &|| {
                    self.largest_merge_between(nodes[0], |point| nodes[1..].contains(&point))
                });
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
            let fanned = fan == around.len() && unmatched <= 1;
            // On a border any valence will do: a singularity that the
            // lengths bring onto it simply becomes a corner of the border.
            let valued = border
                || (valence as i64 == expected_valence[vertex as usize].unwrap_or(4)
                    && (3..=MOST_VALENCE).contains(&valence));
            if fanned && !valued && node_valence[vertex as usize] == Some(valence) {
                // The lengths gave it all its node has: the T-mesh is wrong.
                defects.wrong_singularities += 1;
            } else if !(fanned && valued) {
                charge(&mut defects, vertex, &|| self.largest_merge_at(vertex));
            }
        }
        // Singularities merged may not pinch a tube or a handle shut that
        // no fan shows: then each merge is undone. (Of a lattice with other
        // defects the characteristic says nothing.)
        if !merged.is_empty() && defects.is_empty() && self.euler() != tmesh.report.euler {
            for (&vertex, (nodes, _)) in &merged {
                charge(&mut defects, vertex, &|| {
                    self.largest_merge_between(nodes[0], |point| nodes[1..].contains(&point))
                });
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

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::math::Vec3;
    use crate::tmesh::{Arc, Node, Patch, Report};

    /// A T-mesh with no T-junction: each of `quads` (counter-clockwise) a
    /// rectangle, each edge an arc of real length `length` gives it, the
    /// nodes `singular` singularities of the index their arcs stand for.
    pub fn of_quads(quads: &[[u32; 4]], singular: &[u32], length: impl Fn(u32, u32) -> f64) -> TMesh {
        let nodes = quads.iter().flatten().max().map_or(0, |&n| n + 1);
        let mut arcs: Vec<Arc> = Vec::new();
        let mut index: BTreeMap<(u32, u32), usize> = BTreeMap::new();
        let mut patches = Vec::new();
        for (patch, quad) in quads.iter().enumerate() {
            let mut boundary = Vec::new();
            for k in 0..4 {
                let (a, b) = (quad[k], quad[(k + 1) % 4]);
                let arc = *index.entry((a.min(b), a.max(b))).or_insert_with(|| {
                    arcs.push(Arc {
                        nodes: [a, b],
                        length: length(a, b),
                        patches: [u32::MAX; 2],
                        border: true,
                        feature: false,
                        trace: 0,
                        span: [0.0; 2],
                    });
                    arcs.len() - 1
                });
                let forward = arcs[arc].nodes[0] == a;
                arcs[arc].patches[!forward as usize] = patch as u32;
                arcs[arc].border = arcs[arc].patches.contains(&u32::MAX);
                boundary.push(2 * arc as u32 + !forward as u32);
            }
            let sides: Vec<f64> = boundary.iter().map(|&half| arcs[half as usize / 2].length).collect();
            patches.push(Patch { boundary, angles: vec![1; 4], sides: Some([sides[0], sides[1], sides[2], sides[3]]) });
        }
        let mut node_list: Vec<Node> = (0..nodes)
            .map(|_| Node { position: Vec3::new(0.0, 0.0, 0.0), kind: NodeKind::Junction, arcs: Vec::new() })
            .collect();
        for (arc, data) in arcs.iter().enumerate() {
            node_list[data.nodes[0] as usize].arcs.push(2 * arc as u32);
            node_list[data.nodes[1] as usize].arcs.push(2 * arc as u32 + 1);
        }
        // The index of each singularity is the one its arcs stand for.
        for &node in singular {
            let data = &mut node_list[node as usize];
            data.kind = NodeKind::Singularity { vertex: node, index: 4 - data.arcs.len() as i8 };
        }
        let euler = nodes as i64 - arcs.len() as i64 + quads.len() as i64;
        let report = Report { euler, surface_euler: euler, ..Default::default() };
        TMesh { nodes: node_list, arcs, patches, traces: Vec::new(), report }
    }

    /// A grid of 3 by 3 quads on a disc, with a strip of three quads
    /// inserted along the way (0,1)-(1,1)-(1,2)-(1,3), which turns at (1,1):
    /// the grid vertex 5 is split into a 5 (itself) and a 3 (17). The four
    /// edges across the strip are returned with the T-mesh.
    pub fn disc_with_a_pair() -> (TMesh, [(u32, u32); 4]) {
        let quads = [
            [0, 1, 5, 4],
            [1, 2, 6, 5],
            [2, 3, 7, 6],
            [5, 6, 10, 9],
            [6, 7, 11, 10],
            [9, 10, 14, 13],
            [10, 11, 15, 14],
            [16, 17, 18, 8],
            [8, 18, 19, 12],
            // The strip.
            [4, 5, 17, 16],
            [5, 9, 18, 17],
            [9, 13, 19, 18],
        ];
        let across = [(4, 16), (5, 17), (9, 18), (13, 19)];
        let tmesh = of_quads(&quads, &[5, 17], |a, b| if across.contains(&(a.min(b), a.max(b))) { 0.2 } else { 1.0 });
        (tmesh, across)
    }

    /// Lengths of 1 everywhere but on the arcs `zero`.
    fn lengths(tmesh: &TMesh, zero: &[(u32, u32)]) -> Vec<i64> {
        tmesh
            .arcs
            .iter()
            .map(|arc| {
                let [a, b] = arc.nodes;
                if zero.contains(&(a.min(b), a.max(b))) { 0 } else { 1 }
            })
            .collect()
    }

    #[test]
    fn a_3_and_a_5_merge_into_a_regular_vertex_only_when_asked() {
        let (tmesh, across) = disc_with_a_pair();
        assert_eq!((tmesh.nodes[5].arcs.len(), tmesh.nodes[17].arcs.len()), (5, 3));
        // Every length 1: the quad mesh is the T-mesh, sound either way.
        let open = build(&tmesh, &lengths(&tmesh, &[]));
        assert!(open.defects(&tmesh, false).is_empty() && open.defects(&tmesh, true).is_empty());
        // The strip closed up: the grid of 3 by 3 again, the pair in one
        // vertex of valence 4.
        let closed = build(&tmesh, &lengths(&tmesh, &across));
        assert_eq!(closed.lattice.quads.len(), 9);
        assert_eq!(closed.lattice.node_vertex[5], closed.lattice.node_vertex[17]);
        assert_eq!(closed.euler(), tmesh.report.euler);
        let kept = closed.defects(&tmesh, false);
        assert!(!kept.reopen.is_empty(), "the singularities are kept apart by default");
        assert!(kept.reopen.iter().all(|&arc| {
            let [a, b] = tmesh.arcs[arc].nodes;
            across.contains(&(a.min(b), a.max(b)))
        }));
        assert!(closed.defects(&tmesh, true).is_empty());
    }

    /// A cube of six quads: eight singularities of valence 3.
    fn cube() -> TMesh {
        let quads = [[0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [1, 2, 6, 5], [2, 3, 7, 6], [3, 0, 4, 7]];
        of_quads(&quads, &[0, 1, 2, 3, 4, 5, 6, 7], |_, _| 1.0)
    }

    #[test]
    fn two_3s_are_not_merged_into_a_valence_of_2() {
        let tmesh = cube();
        // The ring of four quads round the cube closed up: the top and the
        // bottom glued face to face, every vertex of valence 2, the Euler
        // characteristic still that of a sphere.
        let closed = build(&tmesh, &lengths(&tmesh, &[(0, 4), (1, 5), (2, 6), (3, 7)]));
        assert_eq!(closed.lattice.quads.len(), 2);
        assert_eq!(closed.euler(), tmesh.report.euler);
        for merging in [false, true] {
            let defects = closed.defects(&tmesh, merging);
            assert!(!defects.reopen.is_empty() && defects.unexplained == 0, "merging {merging}");
        }
    }

    /// A disc of `n` quads round the singularity 0, of the index `index`
    /// says, or that of its arcs.
    fn fan(n: u32, index: Option<i8>) -> TMesh {
        let rim = |i: u32| 1 + i % (2 * n);
        let quads: Vec<[u32; 4]> = (0..n).map(|k| [0, rim(2 * k), rim(2 * k + 1), rim(2 * k + 2)]).collect();
        let mut tmesh = of_quads(&quads, &[0], |_, _| 1.0);
        if let (Some(index), NodeKind::Singularity { vertex, .. }) = (index, tmesh.nodes[0].kind) {
            tmesh.nodes[0].kind = NodeKind::Singularity { vertex, index };
        }
        tmesh
    }

    #[test]
    fn a_singularity_inside_has_a_valence_of_3_to_6() {
        for n in 2..=8 {
            let tmesh = fan(n, None);
            let defects = build(&tmesh, &lengths(&tmesh, &[])).defects(&tmesh, false);
            // Every length 1: no length could do better, and none is blamed.
            assert!(defects.is_empty(), "{n} quads");
            assert_eq!(defects.wrong_singularities, !(3..=MOST_VALENCE as u32).contains(&n) as usize, "{n} quads");
        }
    }

    #[test]
    fn the_valence_of_a_singularity_is_that_of_its_index_not_its_arcs() {
        // The field says a valence of 3 where the map sent out 2 arcs, or 4:
        // either way the vertex is wrong (#279, #280).
        for (n, wrong) in [(2, 1), (3, 0), (4, 1)] {
            let tmesh = fan(n, Some(1));
            let defects = build(&tmesh, &lengths(&tmesh, &[])).defects(&tmesh, false);
            assert!(defects.is_empty() && defects.wrong_singularities == wrong, "{n} quads");
        }
    }

    #[test]
    fn a_plain_vertex_of_valence_2_inside_is_blamed() {
        // Two quads sharing two edges round a vertex no singularity is on.
        let tmesh = of_quads(&[[0, 1, 2, 3], [0, 3, 4, 1]], &[], |_, _| 1.0);
        let defects = build(&tmesh, &lengths(&tmesh, &[])).defects(&tmesh, false);
        assert_eq!((defects.unexplained, defects.wrong_singularities), (1, 0));
    }
}
