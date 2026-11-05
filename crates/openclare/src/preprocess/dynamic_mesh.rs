//! Triangle mesh that can be edited in place: edge split, collapse and flip.
//!
//! Connectivity is kept as "which triangles touch each vertex". That is all
//! the three operations need, it stays valid whatever the order they are
//! applied in, and unlike a half-edge structure it tolerates the odd
//! non-manifold edge of a real-world input (such edges are simply left alone).

use std::collections::HashSet;

use crate::math::Vec3;
use crate::mesh::TriMesh;

const DEAD: [u32; 3] = [u32::MAX; 3];

/// How far a vertex of a feature line may sit off the segment that replaces
/// it when it is collapsed, relative to the length of that segment.
const FEATURE_DEVIATION: f64 = 0.05;
/// Cosine of the turn beyond which a vertex of a feature line is a corner.
const CORNER_COS: f64 = 0.7;
/// A move is rejected when it turns a triangle's normal by more than this
/// (cosine): it would fold the surface.
const MIN_NORMAL_AGREEMENT: f64 = 0.2;

#[inline]
fn key(a: u32, b: u32) -> (u32, u32) {
    (a.min(b), a.max(b))
}

pub(crate) struct DynamicMesh {
    pub positions: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    /// Live triangles around each vertex.
    incident: Vec<Vec<u32>>,
    /// Edges that must survive as edges: creases and open boundaries.
    features: HashSet<(u32, u32)>,
    /// Vertices where feature lines meet or turn sharply: they never move.
    corner: Vec<bool>,
    live: usize,
}

impl DynamicMesh {
    pub fn new(mesh: &TriMesh, feature_edges: &[[u32; 2]]) -> Self {
        let mut incident = vec![Vec::new(); mesh.positions.len()];
        for (t, tri) in mesh.triangles.iter().enumerate() {
            for &v in tri {
                incident[v as usize].push(t as u32);
            }
        }
        let mut mesh = Self {
            positions: mesh.positions.clone(),
            triangles: mesh.triangles.clone(),
            incident,
            features: feature_edges.iter().map(|&[a, b]| key(a, b)).collect(),
            corner: vec![false; mesh.positions.len()],
            live: mesh.triangles.len(),
        };
        for v in 0..mesh.positions.len() as u32 {
            mesh.refresh_corner(v);
        }
        mesh
    }

    /// Makes the edge `a-b` one that must survive as an edge. The corners
    /// at its ends are to be looked at again ([`Self::refresh_corner`]).
    pub fn mark_feature(&mut self, a: u32, b: u32) {
        self.features.insert(key(a, b));
    }

    /// Says that `v`, on a feature line that merely runs through it, is no
    /// corner however the line turns there: it may slide along the line.
    pub fn let_slide(&mut self, v: u32) {
        if self.feature_degree(v) == 2 {
            self.corner[v as usize] = false;
        }
    }

    /// Decides again whether `v` is a corner: where feature lines meet, end,
    /// or turn sharply.
    pub fn refresh_corner(&mut self, v: u32) {
        let p = self.positions[v as usize];
        self.corner[v as usize] = match self.feature_neighbors(v)[..] {
            [] => false,
            [x, y] => {
                let incoming = (p - self.positions[x as usize]).normalize_or_zero();
                let outgoing = (self.positions[y as usize] - p).normalize_or_zero();
                incoming.dot(outgoing) < CORNER_COS
            }
            _ => true,
        };
    }

    pub fn is_corner(&self, v: u32) -> bool {
        self.corner[v as usize]
    }

    pub fn triangle_count(&self) -> usize {
        self.live
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    pub fn is_live_vertex(&self, v: u32) -> bool {
        !self.incident[v as usize].is_empty()
    }

    /// The live triangles.
    pub fn triangles(&self) -> impl Iterator<Item = [u32; 3]> + '_ {
        self.triangles.iter().copied().filter(|tri| *tri != DEAD)
    }

    /// Every undirected edge, once.
    pub fn edges(&self) -> Vec<(u32, u32)> {
        let mut edges: Vec<(u32, u32)> = self
            .triangles
            .iter()
            .filter(|tri| **tri != DEAD)
            .flat_map(|tri| (0..3).map(move |k| key(tri[k], tri[(k + 1) % 3])))
            .collect();
        edges.sort_unstable();
        edges.dedup();
        edges
    }

    /// Triangles on edge `a-b` (the first two) and how many there are in all.
    fn edge_triangles(&self, a: u32, b: u32) -> ([u32; 2], usize) {
        let mut found = [u32::MAX; 2];
        let mut count = 0;
        for &t in &self.incident[a as usize] {
            if self.triangles[t as usize].contains(&b) {
                if count < 2 {
                    found[count] = t;
                }
                count += 1;
            }
        }
        (found, count)
    }

    /// Is `a-b-c` a live triangle, whichever way round?
    pub fn has_triangle(&self, [a, b, c]: [u32; 3]) -> bool {
        self.incident[a as usize].iter().any(|&t| {
            let tri = self.triangles[t as usize];
            tri.contains(&b) && tri.contains(&c)
        })
    }

    pub fn has_edge(&self, a: u32, b: u32) -> bool {
        self.incident[a as usize].iter().any(|&t| self.triangles[t as usize].contains(&b))
    }

    /// The corner of `t` that is neither `a` nor `b`.
    fn opposite(&self, t: u32, a: u32, b: u32) -> u32 {
        self.triangles[t as usize].into_iter().find(|&v| v != a && v != b).unwrap()
    }

    pub fn neighbors(&self, v: u32) -> Vec<u32> {
        let mut out: Vec<u32> = Vec::with_capacity(8);
        for &t in &self.incident[v as usize] {
            for w in self.triangles[t as usize] {
                if w != v && !out.contains(&w) {
                    out.push(w);
                }
            }
        }
        out
    }

    pub fn is_feature(&self, a: u32, b: u32) -> bool {
        self.features.contains(&key(a, b))
    }

    /// 0 for a free vertex, 2 on a feature line, anything else is a corner.
    pub fn feature_degree(&self, v: u32) -> usize {
        if self.features.is_empty() {
            return 0;
        }
        self.neighbors(v).into_iter().filter(|&n| self.is_feature(v, n)).count()
    }

    /// The feature neighbours of `v`.
    pub fn feature_neighbors(&self, v: u32) -> Vec<u32> {
        self.neighbors(v).into_iter().filter(|&n| self.is_feature(v, n)).collect()
    }

    fn area_vector_of(&self, [a, b, c]: [u32; 3]) -> Vec3 {
        let (pa, pb, pc) = (self.positions[a as usize], self.positions[b as usize], self.positions[c as usize]);
        0.5 * (pb - pa).cross(pc - pa)
    }

    /// Area-weighted normal at `v`.
    pub fn normal(&self, v: u32) -> Vec3 {
        self.incident[v as usize]
            .iter()
            .map(|&t| self.area_vector_of(self.triangles[t as usize]))
            .sum::<Vec3>()
            .normalize_or_zero()
    }

    /// Would moving `v` to `position` keep every triangle around it facing
    /// the same way? Triangles that also contain `ignore` are skipped (they
    /// are about to disappear).
    pub fn move_keeps_orientation(&self, v: u32, position: Vec3, ignore: u32) -> bool {
        self.incident[v as usize].iter().all(|&t| {
            let tri = self.triangles[t as usize];
            if tri.contains(&ignore) {
                return true;
            }
            let before = self.area_vector_of(tri);
            let corner = |w: u32| if w == v { position } else { self.positions[w as usize] };
            let after = 0.5 * (corner(tri[1]) - corner(tri[0])).cross(corner(tri[2]) - corner(tri[0]));
            let (Some(before), Some(after)) = (before.try_normalize(), after.try_normalize()) else {
                // Already degenerate triangles cannot get worse; new ones must not appear.
                return before.length_squared() == 0.0;
            };
            before.dot(after) > MIN_NORMAL_AGREEMENT
        })
    }

    /// Inserts a vertex at `at` on edge `a-b`, splitting the triangles on it.
    pub fn split(&mut self, a: u32, b: u32, at: Vec3) -> u32 {
        let m = self.positions.len() as u32;
        self.positions.push(at);
        self.incident.push(Vec::new());
        self.corner.push(false);

        let on_edge: Vec<u32> =
            self.incident[a as usize].iter().copied().filter(|&t| self.triangles[t as usize].contains(&b)).collect();
        for t in on_edge {
            let tri = self.triangles[t as usize];
            let c = self.opposite(t, a, b);
            // Same winding for both halves: one keeps `a`, the other keeps `b`.
            let with_a = tri.map(|v| if v == b { m } else { v });
            let with_b = tri.map(|v| if v == a { m } else { v });
            self.triangles[t as usize] = with_a;
            self.incident[b as usize].retain(|&x| x != t);
            self.incident[m as usize].push(t);

            let fresh = self.triangles.len() as u32;
            self.triangles.push(with_b);
            for v in [m, b, c] {
                self.incident[v as usize].push(fresh);
            }
            self.live += 1;
        }

        if self.features.remove(&key(a, b)) {
            self.features.insert(key(a, m));
            self.features.insert(key(m, b));
        }
        m
    }

    /// Can `from` be merged into `to`, which then moves to `position`,
    /// without changing the topology, folding the surface or damaging a
    /// feature line?
    pub fn can_collapse(&self, from: u32, to: u32, position: Vec3) -> bool {
        self.collapse_check(from, to, position, false)
    }

    /// As [`Self::can_collapse`], and also where `from` is on a stretch of
    /// feature line that runs alongside one through `to` between the same
    /// two vertices: the collapse then makes the two stretches one, `from`
    /// staying on a line. Two lines that close are what a sliver between
    /// them is made of, where they meet at an acute angle.
    pub fn can_merge_lines(&self, from: u32, to: u32, position: Vec3) -> bool {
        self.collapse_check(from, to, position, true)
    }

    fn collapse_check(&self, from: u32, to: u32, position: Vec3, merge_lines: bool) -> bool {
        let (shared, count) = self.edge_triangles(from, to);
        if count == 0 || count > 2 || self.corner[from as usize] {
            return false;
        }

        // Link condition: the only vertices adjacent to both ends are the
        // ones facing the edge. Anything else would be pinched together.
        let around_from = self.neighbors(from);
        let around_to = self.neighbors(to);
        if around_from.iter().filter(|v| around_to.contains(v)).count() != count {
            return false;
        }
        // The vertices facing the edge each lose a triangle and must keep a
        // proper fan afterwards.
        for &t in &shared[..count] {
            let facing = self.opposite(t, from, to);
            let is_interior = self.neighbors(facing).len() == self.incident[facing as usize].len();
            if self.incident[facing as usize].len() <= if is_interior { 3 } else { 1 } {
                return false;
            }
        }

        // A vertex on a feature line may only slide along it, and only where
        // the line is straight enough; corners stay.
        match self.feature_neighbors(from)[..] {
            [] => {}
            [x, y] if x == to || y == to => {
                let other = if x == to { y } else { x };
                let (start, end, p) = (self.positions[other as usize], position, self.positions[from as usize]);
                let along = end - start;
                let length_squared = along.length_squared();
                if length_squared == 0.0 {
                    return false;
                }
                let foot = start + along * ((p - start).dot(along) / length_squared).clamp(0.0, 1.0);
                if foot.distance_squared(p) > FEATURE_DEVIATION * FEATURE_DEVIATION * length_squared {
                    return false;
                }
            }
            [x, y] if merge_lines && self.is_feature(to, x) && self.is_feature(to, y) => {}
            _ => return false,
        }

        self.move_keeps_orientation(from, position, to) && self.move_keeps_orientation(to, position, from)
    }

    /// Merges `from` into `to` and moves `to` to `position`. Only call after
    /// [`Self::can_collapse`].
    pub fn collapse(&mut self, from: u32, to: u32, position: Vec3) {
        for x in self.feature_neighbors(from) {
            self.features.remove(&key(from, x));
            if x != to {
                self.features.insert(key(to, x));
            }
        }

        for t in std::mem::take(&mut self.incident[from as usize]) {
            let tri = self.triangles[t as usize];
            if tri.contains(&to) {
                for v in tri {
                    self.incident[v as usize].retain(|&x| x != t);
                }
                self.triangles[t as usize] = DEAD;
                self.live -= 1;
            } else {
                self.triangles[t as usize] = tri.map(|v| if v == from { to } else { v });
                self.incident[to as usize].push(t);
            }
        }
        self.positions[to as usize] = position;
    }

    /// If edge `a-b` can be flipped, the two vertices facing it (which the
    /// flipped edge would join).
    pub fn can_flip(&self, a: u32, b: u32) -> Option<(u32, u32)> {
        let (shared, c, d) = self.flippable(a, b)?;
        // Only across a (nearly) flat pair, and only if the new pair still
        // faces the same way: otherwise the flip would change the shape.
        let n0 = self.area_vector_of(self.triangles[shared[0] as usize]).try_normalize()?;
        let n1 = self.area_vector_of(self.triangles[shared[1] as usize]).try_normalize()?;
        if n0.dot(n1) < 0.8 {
            return None;
        }
        let [first, second] = self.flipped(shared[0], a, b, c, d);
        let m0 = self.area_vector_of(first).try_normalize()?;
        let m1 = self.area_vector_of(second).try_normalize()?;
        let mean = (n0 + n1).normalize_or_zero();
        (m0.dot(mean) > 0.5 && m1.dot(mean) > 0.5).then_some((c, d))
    }

    /// Can edge `a-b`, which a sliver lies on, be flipped to rid the mesh
    /// of it? The normal of a sliver means nothing, so the pair is not asked
    /// to be flat: the new triangles are to face the way the pair does as a
    /// whole (which the other triangle of it says), and their smallest angle
    /// is to be larger than that of the pair. Nor may a new triangle have two
    /// sides on feature lines, as where a line ends or two meet at a tip:
    /// held straight in the map, the lines leave it no room there, and the
    /// map folds over (the chain then lets every line go, #31).
    pub fn can_flip_sliver(&self, a: u32, b: u32) -> bool {
        let Some((shared, c, d)) = self.flippable(a, b) else { return false };
        let pair = shared.map(|t| self.triangles[t as usize]);
        let Some(mean) = (self.area_vector_of(pair[0]) + self.area_vector_of(pair[1])).try_normalize() else {
            return false;
        };
        let new = self.flipped(shared[0], a, b, c, d);
        let faces_the_same_way =
            new.iter().all(|&tri| self.area_vector_of(tri).try_normalize().is_some_and(|m| m.dot(mean) > 0.5));
        let smallest =
            |tris: [[u32; 3]; 2]| tris.map(|tri| self.min_angle_of(tri)).into_iter().fold(f64::INFINITY, f64::min);
        let on_lines =
            |[x, y, z]: [u32; 3]| [(x, y), (y, z), (z, x)].into_iter().filter(|&(u, w)| self.is_feature(u, w)).count();
        faces_the_same_way && smallest(new) > smallest(pair) && new.iter().all(|&tri| on_lines(tri) < 2)
    }

    /// The two triangles on edge `a-b` and the vertices facing it, if it is
    /// an edge a flip may turn: no feature, two triangles, the flipped edge
    /// not already there, and ends with triangles to spare.
    fn flippable(&self, a: u32, b: u32) -> Option<([u32; 2], u32, u32)> {
        if self.is_feature(a, b) {
            return None;
        }
        let (shared, count) = self.edge_triangles(a, b);
        if count != 2 {
            return None;
        }
        let (c, d) = (self.opposite(shared[0], a, b), self.opposite(shared[1], a, b));
        if c == d || self.has_edge(c, d) {
            return None;
        }
        // Both ends lose a triangle.
        for v in [a, b] {
            let is_interior = self.neighbors(v).len() == self.incident[v as usize].len();
            if self.incident[v as usize].len() <= if is_interior { 3 } else { 2 } {
                return None;
            }
        }
        Some((shared, c, d))
    }

    /// Smallest angle of a triangle, in radians.
    pub fn min_angle_of(&self, tri: [u32; 3]) -> f64 {
        let p = tri.map(|v| self.positions[v as usize]);
        (0..3)
            .map(|k| {
                let (u, w) = (p[(k + 1) % 3] - p[k], p[(k + 2) % 3] - p[k]);
                u.normalize_or_zero().dot(w.normalize_or_zero()).clamp(-1.0, 1.0).acos()
            })
            .fold(f64::INFINITY, f64::min)
    }

    /// The two triangles that replace the pair on `a-b`, with `c` facing the
    /// edge in `first` and `d` in the other.
    fn flipped(&self, first: u32, a: u32, b: u32, c: u32, d: u32) -> [[u32; 3]; 2] {
        // Orient so that `first` reads a -> b -> c.
        let tri = self.triangles[first as usize];
        let at = tri.iter().position(|&v| v == a).unwrap();
        let (a, b) = if tri[(at + 1) % 3] == b { (a, b) } else { (b, a) };
        [[a, d, c], [d, b, c]]
    }

    /// Replaces edge `a-b` by the edge joining the two vertices facing it.
    /// Only call after [`Self::can_flip`].
    pub fn flip(&mut self, a: u32, b: u32) {
        let (shared, _) = self.edge_triangles(a, b);
        let (c, d) = (self.opposite(shared[0], a, b), self.opposite(shared[1], a, b));
        let replacement = self.flipped(shared[0], a, b, c, d);
        for (&t, tri) in shared.iter().zip(replacement) {
            for v in self.triangles[t as usize] {
                self.incident[v as usize].retain(|&x| x != t);
            }
            self.triangles[t as usize] = tri;
            for v in tri {
                self.incident[v as usize].push(t);
            }
        }
    }

    /// Number of edges at `v`.
    pub fn valence(&self, v: u32) -> usize {
        self.neighbors(v).len()
    }

    pub fn is_boundary_vertex(&self, v: u32) -> bool {
        self.neighbors(v).len() != self.incident[v as usize].len()
    }

    /// The compact mesh and its feature edges.
    pub fn finish(self) -> (TriMesh, Vec<[u32; 2]>) {
        let mut remap = vec![u32::MAX; self.positions.len()];
        let mut mesh = TriMesh::default();
        for tri in self.triangles.iter().filter(|tri| **tri != DEAD) {
            mesh.triangles.push(tri.map(|v| {
                if remap[v as usize] == u32::MAX {
                    remap[v as usize] = mesh.positions.len() as u32;
                    mesh.positions.push(self.positions[v as usize]);
                }
                remap[v as usize]
            }));
        }
        let mut features: Vec<[u32; 2]> = self
            .features
            .iter()
            .filter(|(a, b)| remap[*a as usize] != u32::MAX && remap[*b as usize] != u32::MAX)
            .map(|&(a, b)| [remap[a as usize], remap[b as usize]])
            .collect();
        features.sort_unstable();
        (mesh, features)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::{Topology, primitives};

    fn check_closed_manifold(mesh: &TriMesh, euler: i64) {
        let topology = Topology::build(mesh);
        assert!(topology.is_closed());
        assert_eq!(topology.non_manifold_edges, 0);
        let chi = mesh.positions.len() as i64 - topology.edge_count() as i64 + mesh.triangles.len() as i64;
        assert_eq!(chi, euler);
    }

    #[test]
    fn split_collapse_and_flip_keep_a_closed_manifold() {
        let source = primitives::icosphere(2);
        let volume = source.signed_volume();
        let mut mesh = DynamicMesh::new(&source, &[]);

        for (a, b) in mesh.edges().into_iter().step_by(3) {
            let middle = 0.5 * (mesh.positions[a as usize] + mesh.positions[b as usize]);
            mesh.split(a, b, middle);
        }
        let (after_split, _) = mesh_clone(&mesh).finish();
        check_closed_manifold(&after_split, 2);
        assert!((after_split.signed_volume() - volume).abs() < 1e-9, "splitting must not move the surface");

        let mut flips = 0;
        for (a, b) in mesh.edges() {
            if mesh.can_flip(a, b).is_some() {
                mesh.flip(a, b);
                flips += 1;
            }
        }
        assert!(flips > 0);
        let mut collapses = 0;
        for (a, b) in mesh.edges().into_iter().step_by(5) {
            if mesh.is_live_vertex(a) && mesh.has_edge(a, b) && mesh.can_collapse(a, b, mesh.positions[b as usize]) {
                mesh.collapse(a, b, mesh.positions[b as usize]);
                collapses += 1;
            }
        }
        assert!(collapses > 0);
        let (result, _) = mesh.finish();
        check_closed_manifold(&result, 2);
        assert!(result.signed_volume() > 0.5 * volume);
    }

    fn mesh_clone(mesh: &DynamicMesh) -> DynamicMesh {
        DynamicMesh {
            positions: mesh.positions.clone(),
            triangles: mesh.triangles.clone(),
            incident: mesh.incident.clone(),
            features: mesh.features.clone(),
            corner: mesh.corner.clone(),
            live: mesh.live,
        }
    }

    #[test]
    fn a_tetrahedron_cannot_be_collapsed_further() {
        let tetrahedron = TriMesh {
            positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y, Vec3::Z],
            triangles: vec![[0, 2, 1], [0, 1, 3], [1, 2, 3], [0, 3, 2]],
        };
        let mesh = DynamicMesh::new(&tetrahedron, &[]);
        for (a, b) in mesh.edges() {
            assert!(!mesh.can_collapse(a, b, mesh.positions[b as usize]));
            assert!(mesh.can_flip(a, b).is_none());
        }
    }

    #[test]
    fn feature_lines_only_shorten_along_themselves_and_keep_their_corners() {
        // A flat square: its border is a feature with four corners.
        let source = primitives::plane(4);
        let topology = Topology::build(&source);
        let border: Vec<[u32; 2]> = topology
            .boundary_half_edges()
            .map(|h| {
                let tri = source.triangles[h / 3];
                [tri[h % 3], tri[(h + 1) % 3]]
            })
            .collect();
        let mesh = DynamicMesh::new(&source, &border);
        let at = |x: f64, y: f64| {
            source.positions.iter().position(|p| (p.x - x).abs() < 1e-9 && (p.y - y).abs() < 1e-9).unwrap() as u32
        };
        let (corner, on_side, next_on_side, inside) = (at(-1.0, -1.0), at(-0.5, -1.0), at(0.0, -1.0), at(-0.5, -0.5));
        let target = |v: u32| mesh.positions[v as usize];

        assert_eq!(mesh.feature_degree(corner), 2);
        assert!(mesh.is_corner(corner) && !mesh.is_corner(on_side));
        assert_eq!(mesh.feature_degree(inside), 0);
        // Along the side: allowed. Off the side into the interior: not.
        assert!(mesh.can_collapse(on_side, next_on_side, target(next_on_side)));
        assert!(!mesh.can_collapse(on_side, inside, target(inside)));
        // An interior vertex may merge into the border.
        assert!(mesh.can_collapse(inside, on_side, target(on_side)));
        // Sliding a corner along a side would cut it: the turn is not straight.
        assert!(!mesh.can_collapse(corner, on_side, target(on_side)));
        assert!(mesh.can_flip(corner, on_side).is_none());
    }

    #[test]
    fn a_sliver_flip_never_closes_a_triangle_on_two_lines() {
        // A cap: c a hair above the long edge a-b. Its flip to c-d would
        // close a-d-c, two of whose sides are the lines that meet at a.
        let at = |x: f64, y: f64| Vec3::new(x, y, 0.0);
        let source = TriMesh {
            positions: vec![
                at(0.0, 0.0),
                at(2.0, 0.0),
                at(1.0, 0.05),
                at(1.0, -1.0),
                at(1.0, 1.0),
                at(-1.0, 0.0),
                at(3.0, 0.0),
            ],
            triangles: vec![[0, 1, 2], [1, 0, 3], [0, 2, 4], [2, 1, 4], [5, 0, 4], [5, 3, 0], [1, 6, 4], [1, 3, 6]],
        };
        let (a, b, c, d) = (0, 1, 2, 3);
        assert!(DynamicMesh::new(&source, &[]).can_flip_sliver(a, b));
        assert!(!DynamicMesh::new(&source, &[[a, c], [a, d]]).can_flip_sliver(a, b));
    }
}
