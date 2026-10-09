use std::collections::{HashMap, HashSet};

use crate::error::{Error, Result};
use crate::math::{INVALID, Vec3};

/// A mesh mended by [`TriMesh::stitched`], and what was done to it.
#[derive(Clone, Debug)]
pub struct Mended {
    pub mesh: TriMesh,
    /// The vertex of the mesh as it was each vertex of this one comes from.
    pub vertex_was: Vec<u32>,
    /// The same for the triangles.
    pub triangle_was: Vec<u32>,
    /// Vertices of torn borders welded into another at the same place.
    pub welded: usize,
    /// Edges with more than two faces, whose faces were taken apart there.
    pub taken_apart: usize,
}

/// Indexed triangle mesh: the input representation of the remesher.
#[derive(Clone, Debug, Default)]
pub struct TriMesh {
    pub positions: Vec<Vec3>,
    pub triangles: Vec<[u32; 3]>,
}

impl TriMesh {
    #[inline]
    pub fn corners(&self, f: usize) -> [Vec3; 3] {
        let [a, b, c] = self.triangles[f];
        [self.positions[a as usize], self.positions[b as usize], self.positions[c as usize]]
    }

    /// Face normal scaled by the face area.
    #[inline]
    pub fn area_vector(&self, f: usize) -> Vec3 {
        let [a, b, c] = self.corners(f);
        0.5 * (b - a).cross(c - a)
    }

    pub fn surface_area(&self) -> f64 {
        (0..self.triangles.len()).map(|f| self.area_vector(f).length()).sum()
    }

    /// Signed enclosed volume; positive when a closed mesh is wound outward.
    pub fn signed_volume(&self) -> f64 {
        (0..self.triangles.len())
            .map(|f| {
                let [a, b, c] = self.corners(f);
                a.dot(b.cross(c)) / 6.0
            })
            .sum()
    }

    /// `(min, mean, max)` over all triangle sides.
    pub fn edge_length_stats(&self) -> (f64, f64, f64) {
        let (mut min, mut sum, mut max) = (f64::INFINITY, 0.0, 0.0f64);
        for f in 0..self.triangles.len() {
            let c = self.corners(f);
            for k in 0..3 {
                let len = c[k].distance(c[(k + 1) % 3]);
                min = min.min(len);
                max = max.max(len);
                sum += len;
            }
        }
        let count = (3 * self.triangles.len()).max(1) as f64;
        (min, sum / count, max)
    }

    pub fn validate(&self) -> Result<()> {
        if self.triangles.is_empty() {
            return Err(Error::InvalidMesh("mesh has no triangles".into()));
        }
        if let Some(i) = self.positions.iter().position(|p| !p.is_finite()) {
            return Err(Error::InvalidMesh(format!("vertex {i} is not finite")));
        }
        let n = self.positions.len() as u32;
        for (f, tri) in self.triangles.iter().enumerate() {
            if tri.iter().any(|&v| v >= n) {
                return Err(Error::InvalidMesh(format!("triangle {f} references a missing vertex")));
            }
        }
        Ok(())
    }

    /// Drops degenerate triangles (repeated index or zero area) and the
    /// vertices nothing references any more.
    /// The mesh with its open borders stitched where they coincide: what a
    /// modifier that splits edges for shading, or a format that writes each
    /// face with vertices of its own, tore apart. With it come the vertex
    /// of this mesh each of its vertices was, and the triangle of this mesh
    /// each of its triangles was. `None` if there was nothing to mend: a mesh
    /// is only touched for a defect it is found to have, borders that
    /// coincide or an edge with more than two faces.
    ///
    /// Only vertices on an open border are welded: inside a closed shell
    /// there is nothing to stitch. And a weld that would leave an edge with
    /// more than two faces is not made: three sheets that meet along a line
    /// are better left as three.
    ///
    /// Where the mesh itself has an edge with more than two faces, those
    /// faces are taken apart along it, each keeping the edge as a border of
    /// its own: what comes out is a surface everywhere, in more pieces if
    /// need be, where the mesh as it was could not be worked on at all.
    pub fn stitched(&self) -> Option<Mended> {
        let key = |a: u32, b: u32| (a.min(b), a.max(b));
        // A closed surface, every edge with its two faces and every vertex
        // in use, is the common case and the largest meshes: told at once.
        let mut edges: Vec<(u32, u32)> =
            self.triangles.iter().flat_map(|&[a, b, c]| [key(a, b), key(b, c), key(c, a)]).collect();
        edges.sort_unstable();
        let in_pairs = edges.chunks(2).all(|pair| pair.len() == 2 && pair[0] == pair[1] && pair[0].0 != pair[0].1);
        let no_third = edges.chunks(2).zip(edges.chunks(2).skip(1)).all(|(pair, next)| pair[0] != next[0]);
        let mut used = vec![false; self.positions.len()];
        for &(a, b) in &edges {
            used[a as usize] = true;
            used[b as usize] = true;
        }
        if in_pairs && no_third && used.iter().all(|&used| used) {
            return None;
        }
        drop(edges);

        let uses = |remap: &[u32]| {
            let mut uses: HashMap<(u32, u32), u32> = HashMap::new();
            for tri in &self.triangles {
                let [a, b, c] = tri.map(|v| remap[v as usize]);
                if a != b && b != c && c != a {
                    for (a, b) in [(a, b), (b, c), (c, a)] {
                        *uses.entry(key(a, b)).or_default() += 1;
                    }
                }
            }
            uses
        };
        let own: Vec<u32> = (0..self.positions.len() as u32).collect();
        let before = uses(&own);
        let mut welds = vec![false; self.positions.len()];
        for (&(a, b), _) in before.iter().filter(|&(_, &count)| count == 1) {
            welds[a as usize] = true;
            welds[b as usize] = true;
        }

        let (min, max) =
            self.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
        let cell = ((max - min).length() * 1e-9).max(f64::MIN_POSITIVE);
        let mut remap = own.clone();
        for _ in 0..4 {
            let mut first_at: HashMap<[i64; 3], u32> = HashMap::new();
            for v in 0..self.positions.len() {
                remap[v] = v as u32;
                if welds[v] {
                    let at = ((self.positions[v] - min) / cell).round().to_array().map(|c| c as i64);
                    remap[v] = *first_at.entry(at).or_insert(v as u32);
                }
            }
            // Edges that now have more faces than two, and did not before.
            let mut spoilt: HashSet<u32> = HashSet::new();
            for (&(a, b), _) in uses(&remap).iter().filter(|&(edge, &count)| count > 2 && !before.contains_key(edge)) {
                spoilt.extend([a, b]);
            }
            if spoilt.is_empty() {
                break;
            }
            for v in 0..self.positions.len() {
                welds[v] &= !spoilt.contains(&remap[v]);
            }
        }

        // The triangles kept, and their corners joined into vertices across
        // the edges that have two faces, and across those only: an edge with
        // more is left as a border of each of its faces.
        let kept: Vec<(u32, [u32; 3])> = (self.triangles.iter().enumerate())
            .map(|(f, tri)| (f as u32, tri.map(|v| remap[v as usize])))
            .filter(|&(_, [a, b, c])| a != b && b != c && c != a)
            .collect();
        let mut sides: HashMap<(u32, u32), Vec<(u32, u32)>> = HashMap::new();
        for (t, &(_, tri)) in kept.iter().enumerate() {
            for k in 0..3 {
                let corners = (3 * t as u32 + k, 3 * t as u32 + (k + 1) % 3);
                sides.entry(key(tri[k as usize], tri[(k as usize + 1) % 3])).or_default().push(corners);
            }
        }
        let mut parent: Vec<u32> = (0..3 * kept.len() as u32).collect();
        fn find(parent: &mut [u32], mut x: u32) -> u32 {
            while parent[x as usize] != x {
                parent[x as usize] = parent[parent[x as usize] as usize];
                x = parent[x as usize];
            }
            x
        }
        let vertex_at = |corner: u32| kept[corner as usize / 3].1[corner as usize % 3];
        for faces in sides.values().filter(|faces| faces.len() == 2) {
            let ((a, b), (c, d)) = (faces[0], faces[1]);
            // The same two vertices, whichever way round each face has them.
            let (c, d) = if vertex_at(a) == vertex_at(c) { (c, d) } else { (d, c) };
            for (one, other) in [(a, c), (b, d)] {
                let (one, other) = (find(&mut parent, one), find(&mut parent, other));
                parent[one.max(other) as usize] = one.min(other);
            }
        }
        let mut compact = vec![INVALID; parent.len()];
        let (mut stitched, mut vertex_was, mut triangle_was) = (TriMesh::default(), Vec::new(), Vec::new());
        for (t, &(f, tri)) in kept.iter().enumerate() {
            stitched.triangles.push([0, 1, 2].map(|k| {
                let root = find(&mut parent, 3 * t as u32 + k) as usize;
                if compact[root] == INVALID {
                    compact[root] = stitched.positions.len() as u32;
                    stitched.positions.push(self.positions[tri[k as usize] as usize]);
                    vertex_was.push(tri[k as usize]);
                }
                compact[root]
            }));
            triangle_was.push(f);
        }
        // Nothing welded, nothing dropped, nothing taken apart: the mesh is
        // whole as it is, and is better left with its own numbering.
        let whole =
            remap == own && kept.len() == self.triangles.len() && stitched.positions.len() == self.positions.len();
        let welded = remap.iter().zip(&own).filter(|(to, own)| to != own).count();
        let taken_apart = sides.values().filter(|faces| faces.len() > 2).count();
        (!whole).then_some(Mended { mesh: stitched, vertex_was, triangle_was, welded, taken_apart })
    }

    /// The connected pieces of the mesh, largest first: each as a mesh of
    /// its own, with the vertex of this mesh each of its vertices was.
    pub fn pieces(&self) -> Vec<(TriMesh, Vec<u32>)> {
        // Disjoint sets over the vertices: a triangle joins its three.
        let mut parent: Vec<u32> = (0..self.positions.len() as u32).collect();
        fn find(parent: &mut [u32], mut x: u32) -> u32 {
            while parent[x as usize] != x {
                parent[x as usize] = parent[parent[x as usize] as usize];
                x = parent[x as usize];
            }
            x
        }
        for tri in &self.triangles {
            let root = find(&mut parent, tri[0]);
            for &v in &tri[1..] {
                let other = find(&mut parent, v);
                parent[other as usize] = root;
            }
        }
        let mut piece_of = vec![INVALID; self.positions.len()];
        let mut pieces: Vec<(TriMesh, Vec<u32>)> = Vec::new();
        let mut local = vec![INVALID; self.positions.len()];
        for tri in &self.triangles {
            let root = find(&mut parent, tri[0]) as usize;
            if piece_of[root] == INVALID {
                piece_of[root] = pieces.len() as u32;
                pieces.push((TriMesh::default(), Vec::new()));
            }
            let (mesh, origin) = &mut pieces[piece_of[root] as usize];
            mesh.triangles.push(tri.map(|v| {
                if local[v as usize] == INVALID {
                    local[v as usize] = mesh.positions.len() as u32;
                    mesh.positions.push(self.positions[v as usize]);
                    origin.push(v);
                }
                local[v as usize]
            }));
        }
        pieces.sort_by(|a, b| b.0.surface_area().total_cmp(&a.0.surface_area()));
        pieces
    }

    pub fn cleaned(&self) -> TriMesh {
        let mut remap = vec![INVALID; self.positions.len()];
        let mut out = TriMesh::default();
        for (f, tri) in self.triangles.iter().enumerate() {
            let [a, b, c] = *tri;
            if a == b || b == c || c == a || self.area_vector(f).length_squared() == 0.0 {
                continue;
            }
            out.triangles.push(tri.map(|v| {
                if remap[v as usize] == INVALID {
                    remap[v as usize] = out.positions.len() as u32;
                    out.positions.push(self.positions[v as usize]);
                }
                remap[v as usize]
            }));
        }
        out
    }
}
