//! Per-face tangent frames and the connection between neighbouring faces.
//!
//! A direction on a face is an angle measured from the face's first edge,
//! counter-clockwise about its normal. To compare directions on two faces
//! that share an edge, one is carried across by unfolding the pair flat
//! around that edge: its angle changes by a fixed amount, the *rotation* of
//! the half-edge.

use crate::math::{INVALID, Vec3};
use crate::mesh::{Topology, TriMesh};

pub struct FaceFrames {
    /// First axis of each face: its first edge, normalised.
    pub axis: Vec<Vec3>,
    /// Second axis: normal x first axis.
    pub side: Vec<Vec3>,
    pub normal: Vec<Vec3>,
    pub area: Vec<f64>,
    /// Face on the other side of half-edge `3 * f + k`, or [`INVALID`].
    pub across: Vec<u32>,
    /// What to add to an angle on face `f` to express the same direction on
    /// the face across half-edge `3 * f + k`.
    pub rotation: Vec<f64>,
    /// Smoothness weight of each half-edge (0 on a border).
    pub weight: Vec<f64>,
}

impl FaceFrames {
    pub fn build(mesh: &TriMesh, topology: &Topology) -> Self {
        let faces = mesh.triangles.len();
        let (mut axis, mut side, mut normal, mut area) = (
            Vec::with_capacity(faces),
            Vec::with_capacity(faces),
            Vec::with_capacity(faces),
            Vec::with_capacity(faces),
        );
        for f in 0..faces {
            let [a, b, c] = mesh.corners(f);
            let area_vector = 0.5 * (b - a).cross(c - a);
            // A flat triangle keeps zero axes: it then carries no weight.
            let n = area_vector.normalize_or_zero();
            let e = if n == Vec3::ZERO { Vec3::ZERO } else { (b - a).normalize_or_zero() };
            axis.push(e);
            side.push(n.cross(e));
            normal.push(n);
            area.push(area_vector.length());
        }

        let mut frames = Self {
            axis,
            side,
            normal,
            area,
            across: vec![INVALID; 3 * faces],
            rotation: vec![0.0; 3 * faces],
            weight: vec![0.0; 3 * faces],
        };
        for h in 0..3 * faces {
            let twin = topology.opposite[h];
            if twin == INVALID {
                continue;
            }
            let (f, g) = (h / 3, twin as usize / 3);
            frames.across[h] = g as u32;
            let tri = mesh.triangles[f];
            let edge = mesh.positions[tri[(h + 1) % 3] as usize] - mesh.positions[tri[h % 3] as usize];
            // Wrapped to (-π, π], so that the two sides of an edge are exact opposites.
            let turn = frames.angle_of(g, edge) - frames.angle_of(f, edge);
            frames.rotation[h] = turn.sin().atan2(turn.cos());
            let total_area = frames.area[f] + frames.area[g];
            if frames.area[f] > 0.0 && frames.area[g] > 0.0 {
                // Edge length over the distance between the two faces, up to
                // a constant: the usual weight of a face-based Laplacian.
                frames.weight[h] = 3.0 * edge.length_squared() / total_area;
            }
        }
        frames
    }

    pub fn len(&self) -> usize {
        self.area.len()
    }

    pub fn is_empty(&self) -> bool {
        self.area.is_empty()
    }

    /// Angle of the tangent part of `direction` in the frame of face `f`.
    #[inline]
    pub fn angle_of(&self, f: usize, direction: Vec3) -> f64 {
        self.side[f].dot(direction).atan2(self.axis[f].dot(direction))
    }

    /// Unit vector at `angle` in the frame of face `f`.
    #[inline]
    pub fn direction(&self, f: usize, angle: f64) -> Vec3 {
        let (sin, cos) = angle.sin_cos();
        self.axis[f] * cos + self.side[f] * sin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::primitives;

    #[test]
    fn carrying_a_direction_across_an_edge_and_back_changes_nothing() {
        let mesh = primitives::icosphere(2);
        let topology = Topology::build(&mesh);
        let frames = FaceFrames::build(&mesh, &topology);
        for h in 0..3 * mesh.triangles.len() {
            let back = topology.opposite[h] as usize;
            let round_trip = frames.rotation[h] + frames.rotation[back];
            assert!((round_trip.sin()).abs() < 1e-12 && round_trip.cos() > 0.0);
            assert!((frames.weight[h] - frames.weight[back]).abs() < 1e-12);
        }
    }

    #[test]
    fn on_a_flat_mesh_a_carried_direction_stays_the_same_vector() {
        let mesh = primitives::plane(3);
        let topology = Topology::build(&mesh);
        let frames = FaceFrames::build(&mesh, &topology);
        let direction = Vec3::new(0.6, 0.8, 0.0);
        for h in 0..3 * mesh.triangles.len() {
            let g = frames.across[h];
            if g == INVALID {
                continue;
            }
            let carried = frames.direction(g as usize, frames.angle_of(h / 3, direction) + frames.rotation[h]);
            assert!(carried.distance(direction) < 1e-12);
        }
    }
}
