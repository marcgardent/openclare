//! Symmetry: a model the same on both sides of a plane gets a quad mesh the
//! same on both sides of it.
//!
//! The plane goes through the origin of the model, across one of its axes.
//! The model is cut along it, the half on the positive side is remeshed with
//! the cut as an open border, and the quad mesh is mirrored across the
//! plane, the vertices on the plane shared by both halves (DT-028).

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

/// The part of `mesh` on the positive side of the plane across `axis`: the
/// triangles that cross it cut along it, the vertices on it exactly on it.
/// A vertex closer to the plane than `snap` is taken to be on it, so that a
/// model built on the plane is not cut into slivers next to it.
pub fn half(mesh: &TriMesh, axis: Axis, snap: f64) -> TriMesh {
    let side: Vec<f64> =
        mesh.positions.iter().map(|&p| axis.side(p)).map(|d| if d.abs() <= snap { 0.0 } else { d }).collect();
    let mut out = TriMesh::default();
    // Each kept vertex of the input, and each cut edge, once.
    let mut kept: HashMap<u32, u32> = HashMap::new();
    let mut cuts: HashMap<(u32, u32), u32> = HashMap::new();
    let mut vertex = |out: &mut TriMesh, v: u32| -> u32 {
        *kept.entry(v).or_insert_with(|| {
            let p = mesh.positions[v as usize];
            out.positions.push(if side[v as usize] == 0.0 { axis.onto(p) } else { p });
            out.positions.len() as u32 - 1
        })
    };
    for tri in &mesh.triangles {
        let mut polygon: Vec<u32> = Vec::with_capacity(4);
        for k in 0..3 {
            let (a, b) = (tri[k], tri[(k + 1) % 3]);
            let (da, db) = (side[a as usize], side[b as usize]);
            if da >= 0.0 {
                polygon.push(vertex(&mut out, a));
            }
            if (da > 0.0 && db < 0.0) || (da < 0.0 && db > 0.0) {
                let key = (a.min(b), a.max(b));
                let at = *cuts.entry(key).or_insert_with(|| {
                    let (pa, pb) = (mesh.positions[a as usize], mesh.positions[b as usize]);
                    let t = da / (da - db);
                    out.positions.push(axis.onto(pa + (pb - pa) * t));
                    out.positions.len() as u32 - 1
                });
                polygon.push(at);
            }
        }
        // A triangle that only touches the plane is on the other side.
        if polygon.len() < 3 || tri.iter().all(|&v| side[v as usize] <= 0.0) {
            continue;
        }
        for k in 1..polygon.len() - 1 {
            out.triangles.push([polygon[0], polygon[k], polygon[k + 1]]);
        }
    }
    out
}

/// `mesh` and its mirror image across the plane, as one mesh: the vertices
/// of its borders within `snap` of the plane are put on it and shared by
/// both halves. (Only those: a vertex inside the mesh near the plane is
/// mirrored like any other.)
pub fn mirrored(mesh: &PolyMesh, axis: Axis, snap: f64) -> PolyMesh {
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
    PolyMesh { positions, faces }
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

/// Checks that `mesh` has something on both sides of the plane.
pub(crate) fn check(mesh: &TriMesh, axis: Axis) -> Result<()> {
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
    Ok(())
}
