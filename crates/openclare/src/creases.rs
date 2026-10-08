//! Creases: which edges of the quad mesh should stay sharp when it is
//! subdivided, and how sharp.
//!
//! A quad mesh is made to be subdivided, and subdivision rounds everything
//! off: the edges of a cube, the rim of a nail, the fold of an eyelid. A
//! crease weight on an edge, from 0 to 1, tells the subdivision to keep it
//! sharp, wholly or in part. Two things give an edge a weight here:
//!
//! - it runs along a hard edge of the input the chain kept: weight 1;
//! - the source turns sharply right at the edge. The surface is looked at on
//!   either side, close to the edge and at the middle of the two faces. If
//!   most of the turn between the two faces happens close to the edge, the
//!   source has a fold there, and the weight grows with the angle of the
//!   fold. Where the surface turns evenly from one face to the other it is
//!   just curved, and subdivision is right to round it.

use crate::math::Vec3;
use crate::mesh::{PolyMesh, TriMesh};
use crate::spatial::Bvh;

/// An edge of the mesh, by its two vertices, and its crease weight in
/// `0..=1`.
pub type Crease = ([u32; 2], f64);

#[derive(Clone, Debug)]
pub struct Creases {
    /// How close to the edge the source is looked at, as a share of the way
    /// to the middle of the face.
    pub near: f64,
    /// Fold of the source, in degrees, below which an edge gets no weight,
    /// and from which it gets all of it.
    pub soft: f64,
    pub sharp: f64,
    /// Least share of the turn between the two faces that must happen close
    /// to the edge for it to be a fold rather than a curve.
    pub focus: f64,
    /// Points looked at along the edge, on each side.
    pub samples: usize,
}

impl Default for Creases {
    fn default() -> Self {
        Self { near: 0.2, soft: 30.0, sharp: 70.0, focus: 0.6, samples: 3 }
    }
}

impl Creases {
    /// The edges of `mesh` that deserve a crease, with their weights.
    /// `surface` is the hierarchy over the triangles of `source`, and
    /// `hard_edges` the edges known to run along a hard edge of it.
    pub fn mark(&self, mesh: &PolyMesh, source: &TriMesh, surface: &Bvh, hard_edges: &[[u32; 2]]) -> Vec<Crease> {
        let key = |a: u32, b: u32| (a.min(b), a.max(b));
        let hard: std::collections::HashSet<(u32, u32)> = hard_edges.iter().map(|&[a, b]| key(a, b)).collect();

        // The two faces of each inner edge, with the edge as each walks it.
        let mut sides: std::collections::BTreeMap<(u32, u32), Vec<usize>> = Default::default();
        for (f, face) in mesh.faces.iter().enumerate() {
            for k in 0..face.len() {
                sides.entry(key(face[k], face[(k + 1) % face.len()])).or_default().push(f);
            }
        }
        let middle_of = |f: usize| -> Vec3 {
            let face = &mesh.faces[f];
            face.iter().map(|&v| mesh.positions[v as usize]).sum::<Vec3>() / face.len() as f64
        };
        let facing_of = |f: usize| -> Vec3 {
            let (face, centre) = (&mesh.faces[f], middle_of(f));
            let at = |k: usize| mesh.positions[face[k % face.len()] as usize] - centre;
            (0..face.len()).map(|k| at(k).cross(at(k + 1))).sum()
        };
        // Which way the source faces under a point of face `f`.
        let source_normal = |point: Vec3, f: usize, reach: f64| -> Option<Vec3> {
            let facing = facing_of(f);
            let same_side = |t: u32| source.area_vector(t as usize).dot(facing) > 0.0;
            let hit = surface.closest_point_where(point, reach, same_side)?;
            Some(source.area_vector(hit.triangle as usize).normalize_or_zero())
        };

        let mut creases = Vec::new();
        for (&(a, b), faces) in &sides {
            if hard.contains(&(a, b)) {
                creases.push(([a, b], 1.0));
                continue;
            }
            let &[f, g] = faces.as_slice() else { continue };
            let (p, q) = (mesh.positions[a as usize], mesh.positions[b as usize]);
            let along = (q - p).normalize_or_zero();
            // Mean direction the source faces on each side, near and far.
            let mut turned = [[Vec3::ZERO; 2]; 2];
            let mut seen = true;
            for (side, &face) in [f, g].iter().enumerate() {
                let inward = middle_of(face) - 0.5 * (p + q);
                // Straight away from the edge, in the face.
                let away = inward - along * inward.dot(along);
                let reach = away.length().max(p.distance(q));
                for step in 0..self.samples {
                    let on_edge = p.lerp(q, (step as f64 + 1.0) / (self.samples as f64 + 1.0));
                    for (which, share) in [self.near, 1.0].into_iter().enumerate() {
                        match source_normal(on_edge + away * share, face, reach) {
                            Some(normal) => turned[which][side] += normal,
                            None => seen = false,
                        }
                    }
                }
            }
            if !seen {
                continue;
            }
            let angle = |pair: [Vec3; 2]| pair[0].normalize_or_zero().angle_between(pair[1].normalize_or_zero());
            let (near, far) = (angle(turned[0]).to_degrees(), angle(turned[1]).to_degrees());
            if !(near.is_finite() && far.is_finite()) || near < self.focus * far {
                continue;
            }
            let weight = ((near - self.soft) / (self.sharp - self.soft)).clamp(0.0, 1.0);
            if weight > 0.0 {
                creases.push(([a, b], weight));
            }
        }
        creases
    }
}
