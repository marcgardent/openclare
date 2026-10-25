//! Symmetry: a model the same on both sides of a plane gets a quad mesh the
//! same on both sides of it.
//!
//! The plane goes through the origin of the model, across one of its axes.
//! It is drawn into the model, as a line of edges for the quads to follow;
//! the whole model is remeshed, the faces on the positive side kept, and
//! mirrored across the plane, the vertices on it shared by both halves
//! (DT-028).
//!
//! Remeshing only the half, with the cut as an open border, takes half the
//! work but fails on a detailed sculpt: where the plane runs through the
//! lips or a nostril, the border turns back on itself, and no layout of
//! rectangles follows it.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::math::Vec3;
use crate::mesh::{PolyMesh, TriMesh};

/// The axis the plane of symmetry is across.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const ALL: &[Axis] = &[Axis::X, Axis::Y, Axis::Z];

    pub fn index(self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }
    }

    /// How far `p` is on the positive side of the plane.
    pub fn side(self, p: Vec3) -> f64 {
        p[self.index()]
    }

    /// `p` mirrored across the plane.
    pub fn mirror(self, mut p: Vec3) -> Vec3 {
        p[self.index()] = -p[self.index()];
        p
    }

    /// `p` moved onto the plane.
    pub fn onto(self, mut p: Vec3) -> Vec3 {
        p[self.index()] = 0.0;
        p
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Axis {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        Axis::ALL
            .iter()
            .copied()
            .find(|axis| axis.name().eq_ignore_ascii_case(name))
            .ok_or_else(|| Error::InvalidParams(format!("unknown axis of symmetry '{name}' (known: x, y, z)")))
    }
}

/// A mesh with the plane drawn into it (see [`split`]).
pub struct Split {
    /// The mesh: its vertices first, in their order, then those made where
    /// an edge crosses the plane.
    pub mesh: TriMesh,
    /// The edges that lie on the plane, each by where its two ends are.
    pub lines: Vec<[Vec3; 2]>,
    /// For each vertex made, the two vertices of the edge it cuts and how
    /// far along from the first: what is painted on them is shared out so.
    pub made: Vec<(u32, u32, f64)>,
    /// The triangle of the mesh each triangle comes from.
    pub triangle_was: Vec<u32>,
}

impl Split {
    /// Weights given one per vertex of the mesh, for the mesh split.
    pub fn weights(&self, weights: &[f64]) -> Vec<f64> {
        let mut out = weights.to_vec();
        out.extend(self.made.iter().map(|&(a, b, t)| weights[a as usize] * (1.0 - t) + weights[b as usize] * t));
        out
    }

    /// Values given one per triangle of the mesh, for the mesh split.
    pub fn per_triangle<T: Copy>(&self, values: &[T]) -> Vec<T> {
        self.triangle_was.iter().map(|&t| values[t as usize]).collect()
    }
}

/// `mesh` with the plane drawn into it: every triangle across it cut in two
/// along it, both sides kept, and the edges that now lie on the plane. A
/// vertex closer to the plane than `snap` is put on it.
pub fn split(mesh: &TriMesh, axis: Axis, snap: f64) -> Split {
    let side: Vec<f64> =
        mesh.positions.iter().map(|&p| axis.side(p)).map(|d| if d.abs() <= snap { 0.0 } else { d }).collect();
    let mut out = TriMesh {
        positions: mesh.positions.iter().zip(&side).map(|(&p, &d)| if d == 0.0 { axis.onto(p) } else { p }).collect(),
        triangles: Vec::with_capacity(mesh.triangles.len()),
    };
    let mut cuts: HashMap<(u32, u32), u32> = HashMap::new();
    let mut made = Vec::new();
    let mut triangle_was = Vec::with_capacity(mesh.triangles.len());
    let mut on_plane: Vec<[u32; 2]> = Vec::new();
    for (index, tri) in mesh.triangles.iter().enumerate() {
        let index = index as u32;
        // The corners in order, cut edges given their point on the plane.
        let mut ring: Vec<(u32, f64)> = Vec::with_capacity(5);
        for k in 0..3 {
            let (a, b) = (tri[k], tri[(k + 1) % 3]);
            let (da, db) = (side[a as usize], side[b as usize]);
            ring.push((a, da));
            if (da > 0.0 && db < 0.0) || (da < 0.0 && db > 0.0) {
                let at = *cuts.entry((a.min(b), a.max(b))).or_insert_with(|| {
                    let (pa, pb) = (mesh.positions[a as usize], mesh.positions[b as usize]);
                    out.positions.push(axis.onto(pa + (pb - pa) * (da / (da - db))));
                    made.push((a, b, da / (da - db)));
                    out.positions.len() as u32 - 1
                });
                ring.push((at, 0.0));
            }
        }
        let ends: Vec<u32> = ring.iter().filter(|&&(_, d)| d == 0.0).map(|&(v, _)| v).collect();
        if ring.len() == 3 {
            out.triangles.push(*tri);
            triangle_was.push(index);
            // An edge of the triangle may lie on the plane already.
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                if side[a as usize] == 0.0 && side[b as usize] == 0.0 && a < b {
                    on_plane.push([a, b]);
                }
            }
            continue;
        }
        // Each side is the run of the ring from one point on the plane to
        // the other, fanned from its first point.
        let n = ring.len();
        let starts: Vec<usize> = (0..n).filter(|&k| ring[k].1 == 0.0).collect();
        for (s, &start) in starts.iter().enumerate() {
            let end = starts[(s + 1) % starts.len()];
            let mut polygon = vec![ring[start].0];
            let mut k = (start + 1) % n;
            while k != end {
                polygon.push(ring[k].0);
                k = (k + 1) % n;
            }
            polygon.push(ring[end].0);
            for k in 1..polygon.len().saturating_sub(1) {
                out.triangles.push([polygon[0], polygon[k], polygon[k + 1]]);
                triangle_was.push(index);
            }
        }
        if let [a, b] = ends[..] {
            on_plane.push([a.min(b), a.max(b)]);
        }
    }
    on_plane.sort_unstable();
    on_plane.dedup();
    let lines = on_plane.iter().map(|&[a, b]| [out.positions[a as usize], out.positions[b as usize]]).collect();
    Split { mesh: out, lines, made, triangle_was }
}

/// The faces of `mesh` whose middle is on the positive side of the plane;
/// the vertices they share with the faces left out, and those on the
/// negative side, are put on the plane.
pub fn positive_side(mesh: &PolyMesh, axis: Axis) -> PolyMesh {
    positive_side_kept(mesh, axis).mesh
}

/// What [`positive_side_kept`] gives: the half, the vertex of the half each
/// vertex of the mesh became (`u32::MAX` for those left out), the face of
/// the mesh each face of the half is, and how far each vertex of the half
/// was moved to be put on the plane.
struct Half {
    mesh: PolyMesh,
    kept: Vec<u32>,
    faces: Vec<u32>,
    moved: Vec<f64>,
}

/// The same, and where everything went.
fn positive_side_kept(mesh: &PolyMesh, axis: Axis) -> Half {
    let middle = |face: &Vec<u32>| face.iter().map(|&v| mesh.positions[v as usize]).sum::<Vec3>() / face.len() as f64;
    let keep: Vec<bool> = mesh.faces.iter().map(|face| axis.side(middle(face)) >= 0.0).collect();
    let mut in_kept = vec![false; mesh.positions.len()];
    let mut in_dropped = vec![false; mesh.positions.len()];
    for (face, &kept) in mesh.faces.iter().zip(&keep) {
        for &v in face {
            if kept {
                in_kept[v as usize] = true;
            } else {
                in_dropped[v as usize] = true;
            }
        }
    }
    let mut index = vec![u32::MAX; mesh.positions.len()];
    let mut out = PolyMesh::default();
    let mut moved = Vec::new();
    for (v, &p) in mesh.positions.iter().enumerate() {
        if in_kept[v] {
            index[v] = out.positions.len() as u32;
            let onto = in_dropped[v] || axis.side(p) < 0.0;
            out.positions.push(if onto { axis.onto(p) } else { p });
            moved.push(if onto { axis.side(p).abs() } else { 0.0 });
        }
    }
    let mut faces = Vec::new();
    for (f, (face, &kept)) in mesh.faces.iter().zip(&keep).enumerate() {
        let corners: Vec<u32> = face.iter().map(|&v| index[v as usize]).collect();
        // A face all on the plane has no side.
        if kept && corners.iter().any(|&v| axis.side(out.positions[v as usize]) != 0.0) {
            out.faces.push(corners);
            faces.push(f as u32);
        }
    }
    Half { mesh: out, kept: index, faces, moved }
}

/// `mesh` and its mirror image across the plane, as one mesh: the vertices
/// of its borders within `snap` of the plane are put on it and shared by
/// both halves. (Only those: a vertex inside the mesh near the plane is
/// mirrored like any other.)
pub fn mirrored(mesh: &PolyMesh, axis: Axis, snap: f64) -> PolyMesh {
    mirrored_images(mesh, axis, snap).0
}

/// The same, and the vertex of the result that is the image of each vertex
/// of `mesh` (itself for those shared by both halves).
fn mirrored_images(mesh: &PolyMesh, axis: Axis, snap: f64) -> (PolyMesh, Vec<u32>) {
    let count = mesh.positions.len();
    let mut uses: HashMap<(u32, u32), u32> = HashMap::new();
    for face in &mesh.faces {
        for k in 0..face.len() {
            let (a, b) = (face[k], face[(k + 1) % face.len()]);
            *uses.entry((a.min(b), a.max(b))).or_default() += 1;
        }
    }
    let mut on_plane = vec![false; count];
    for (&(a, b), &n) in &uses {
        if n == 1 {
            for v in [a, b] {
                on_plane[v as usize] |= axis.side(mesh.positions[v as usize]).abs() <= snap;
            }
        }
    }
    let mut positions: Vec<Vec3> =
        mesh.positions.iter().zip(&on_plane).map(|(&p, &on)| if on { axis.onto(p) } else { p }).collect();
    // Where each vertex of the half has its image.
    let mut image = vec![0u32; count];
    for v in 0..count {
        image[v] = if on_plane[v] {
            v as u32
        } else {
            positions.push(axis.mirror(positions[v]));
            positions.len() as u32 - 1
        };
    }
    let mut faces = mesh.faces.clone();
    // A mirror turns the faces over: their corners go the other way round.
    faces.extend(mesh.faces.iter().map(|face| face.iter().rev().map(|&v| image[v as usize]).collect()));
    (PolyMesh { positions, faces }, image)
}

/// The half on the positive side of `mesh` and its mirror image, as one
/// mesh ([`positive_side`], then [`mirrored`]), and where the vertices of
/// `mesh` went, for what was said of its edges to be said of the result.
pub fn symmetric(mesh: &PolyMesh, axis: Axis, snap: f64) -> (PolyMesh, Carried) {
    let Half { mesh: half, kept, faces, moved } = positive_side_kept(mesh, axis);
    let (whole, image) = mirrored_images(&half, axis, snap);
    let edges = whole
        .faces
        .iter()
        .flat_map(|face| (0..face.len()).map(move |k| ordered([face[k], face[(k + 1) % face.len()]])))
        .collect();
    // The faces of the half, then their images, in the same order.
    let faces = faces.iter().chain(&faces).copied().collect();
    (whole, Carried { kept, image, edges, faces, moved })
}

fn ordered([a, b]: [u32; 2]) -> (u32, u32) {
    (a.min(b), a.max(b))
}

/// Where the vertices of a mesh went in [`symmetric`]: edges given by their
/// two vertices (hard edges, creases, edges along guides) are found again
/// in the result, each with its mirror image.
pub struct Carried {
    /// The vertex of the half each vertex of the mesh became, `u32::MAX`
    /// for those left out.
    kept: Vec<u32>,
    /// The image of each vertex of the half in the result.
    image: Vec<u32>,
    /// The edges of the result.
    edges: std::collections::HashSet<(u32, u32)>,
    /// The face of the mesh each face of the result is, or the image of.
    pub faces: Vec<u32>,
    /// How far each vertex of the half was moved to be put on the plane:
    /// nothing for those on the positive side, a little for those the
    /// line of edges on the plane held there, more where it was let go.
    pub moved: Vec<f64>,
}

impl Carried {
    /// The edges of the result that `edges` of the mesh became, with their
    /// mirror images: those left out with the negative side are gone, those
    /// on the plane are their own image, none comes twice.
    pub fn edges(&self, edges: &[[u32; 2]]) -> Vec<[u32; 2]> {
        self.carry(edges.iter().map(|&edge| (edge, ()))).into_iter().map(|(edge, ())| edge).collect()
    }

    /// The same for edges that carry a value, a crease weight say.
    pub fn carry<T: Copy>(&self, edges: impl IntoIterator<Item = ([u32; 2], T)>) -> Vec<([u32; 2], T)> {
        let mut seen = std::collections::HashSet::new();
        let mut out = Vec::new();
        for ([a, b], value) in edges {
            let (a, b) = (self.kept[a as usize], self.kept[b as usize]);
            if a == u32::MAX || b == u32::MAX {
                continue;
            }
            for edge in [[a, b], [self.image[a as usize], self.image[b as usize]]] {
                if self.edges.contains(&ordered(edge)) && seen.insert(ordered(edge)) {
                    out.push((edge, value));
                }
            }
        }
        out
    }
}

/// How far `mesh` is from its own mirror image across the plane: the
/// largest distance from the image of a vertex to the nearest vertex.
pub fn asymmetry(mesh: &PolyMesh, axis: Axis) -> f64 {
    // Points as triangles of no area.
    let grid = crate::spatial::Bvh::from_triangles(mesh.positions.iter().map(|&p| [p, p, p]).collect());
    mesh.positions
        .iter()
        .map(|&p| grid.closest_point(axis.mirror(p)).map_or(f64::INFINITY, |hit| hit.distance_squared.sqrt()))
        .fold(0.0, f64::max)
}

/// How far `mesh` is from being the same on both sides of the plane: the
/// largest distance from the mirror image of a point of its surface to its
/// surface. The points are its vertices, and the middles of its edges and
/// of its triangles: on a model of few large faces, the corners of a hole
/// on one side can fall on a flat face of the other, where the middle of
/// the wall of the hole does not.
pub fn surface_asymmetry(mesh: &TriMesh, axis: Axis) -> f64 {
    let surface = crate::spatial::Bvh::build(mesh);
    let off = |p: Vec3| surface.closest_point(axis.mirror(p)).map_or(f64::INFINITY, |hit| hit.distance_squared.sqrt());
    let corners = mesh.positions.iter().copied();
    let middles = mesh.triangles.iter().flat_map(|tri| {
        let [a, b, c] = tri.map(|v| mesh.positions[v as usize]);
        [(a + b + c) / 3.0, (a + b) / 2.0, (b + c) / 2.0, (c + a) / 2.0]
    });
    corners.chain(middles).map(off).fold(0.0, f64::max)
}

/// Checks that `mesh` has something on both sides of the plane, and is the
/// same on both sides to within `tolerance` of its diagonal: the negative
/// side is replaced by the image of the positive one, and what differs
/// would be lost.
pub(crate) fn check(mesh: &TriMesh, axis: Axis, tolerance: f64) -> Result<()> {
    let (low, high) = mesh
        .positions
        .iter()
        .map(|&p| axis.side(p))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), d| (low.min(d), high.max(d)));
    if !(low < 0.0 && high > 0.0) {
        return Err(Error::InvalidParams(format!(
            "symmetry across {axis}: the model is not on both sides of the plane through its origin"
        )));
    }
    let (low, high) =
        mesh.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let diagonal = (high - low).length();
    let off = surface_asymmetry(mesh, axis);
    if off > tolerance * diagonal {
        return Err(Error::InvalidParams(format!(
            "symmetry across {axis}: the model is not the same on both sides of the plane through its origin \
             (a point is {:.2} % of its size away from the mirror image, {:.2} % allowed)",
            100.0 * off / diagonal,
            100.0 * tolerance
        )));
    }
    Ok(())
}
