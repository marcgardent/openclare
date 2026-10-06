use std::collections::BTreeMap;

use super::rosy::compat_orientation;
use crate::math::{INVALID, Vec3};

/// Soft per-vertex constraints on both fields. A weight of 0 means free, 1
/// means the value is imposed.
///
/// A vertex on a feature line gets a direction (edges must run along the
/// line) and a point (the lattice must pass through the line; it stays free
/// to slide along the direction). A vertex with a point but no direction is
/// a corner: a lattice point is pinned onto it.
#[derive(Clone, Debug, Default)]
pub struct Constraints {
    pub direction: Vec<Vec3>,
    pub direction_weight: Vec<f64>,
    pub point: Vec<Vec3>,
    pub point_weight: Vec<f64>,
}

/// Turn of a feature polyline beyond which a vertex is a corner rather than a
/// point on a smooth line.
const CORNER_COS: f64 = 0.7;

impl Constraints {
    pub fn none(vertex_count: usize) -> Self {
        Self {
            direction: vec![Vec3::ZERO; vertex_count],
            direction_weight: vec![0.0; vertex_count],
            point: vec![Vec3::ZERO; vertex_count],
            point_weight: vec![0.0; vertex_count],
        }
    }

    /// Constraints that make the output follow `edges` (see
    /// [`crate::geometry::feature_edges`]).
    pub fn from_feature_edges(positions: &[Vec3], normals: &[Vec3], edges: &[[u32; 2]]) -> Self {
        let mut constraints = Self::none(positions.len());
        let mut incident: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for &[a, b] in edges {
            incident.entry(a).or_default().push(b);
            incident.entry(b).or_default().push(a);
        }

        for (&v, ends) in &incident {
            let (v, p, n) = (v as usize, positions[v as usize], normals[v as usize]);
            constraints.point[v] = p;
            constraints.point_weight[v] = 1.0;
            if let [a, b] = ends[..] {
                let incoming = (p - positions[a as usize]).normalize_or_zero();
                let outgoing = (positions[b as usize] - p).normalize_or_zero();
                let along = positions[b as usize] - positions[a as usize];
                let tangent = (along - n * n.dot(along)).normalize_or_zero();
                if incoming.dot(outgoing) > CORNER_COS && tangent != Vec3::ZERO {
                    constraints.direction[v] = tangent;
                    constraints.direction_weight[v] = 1.0;
                }
            }
        }
        constraints
    }

    pub fn is_empty(&self) -> bool {
        self.point_weight.iter().all(|&w| w == 0.0)
    }

    /// Carries the constraints to a coarser level whose vertex `c` merges the
    /// fine vertices `children[c]`.
    pub(crate) fn restrict(
        &self,
        children: &[[u32; 2]],
        fine_normals: &[Vec3],
        coarse_positions: &[Vec3],
        coarse_normals: &[Vec3],
    ) -> Self {
        let mut coarse = Self::none(children.len());
        for (c, &[a, b]) in children.iter().enumerate() {
            let a = a as usize;
            let (n, p) = (coarse_normals[c], coarse_positions[c]);
            let pair = (b != INVALID).then_some(b as usize);

            let (mut direction, mut weight) = (self.direction[a] * self.direction_weight[a], self.direction_weight[a]);
            if let Some(b) = pair
                && self.direction_weight[b] > 0.0
            {
                if weight > 0.0 {
                    let (da, db) =
                        compat_orientation(self.direction[a], fine_normals[a], self.direction[b], fine_normals[b]);
                    direction = da * self.direction_weight[a] + db * self.direction_weight[b];
                    weight = 0.5 * (weight + self.direction_weight[b]);
                } else {
                    direction = self.direction[b];
                    weight = self.direction_weight[b];
                }
            }
            let direction = (direction - n * n.dot(direction)).normalize_or_zero();
            if weight > 0.0 && direction != Vec3::ZERO {
                coarse.direction[c] = direction;
                coarse.direction_weight[c] = weight;
            }

            let (mut point, mut weight) = (self.point[a], self.point_weight[a]);
            if let Some(b) = pair
                && self.point_weight[b] > 0.0
            {
                if weight > 0.0 {
                    let total = weight + self.point_weight[b];
                    point = (point * weight + self.point[b] * self.point_weight[b]) / total;
                    weight = 0.5 * total;
                } else {
                    point = self.point[b];
                    weight = self.point_weight[b];
                }
            }
            if weight > 0.0 {
                coarse.point[c] = point - n * n.dot(point - p);
                coarse.point_weight[c] = weight;
            }
        }
        coarse
    }
}
