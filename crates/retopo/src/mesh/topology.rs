use std::collections::HashMap;

use super::TriMesh;
use crate::math::INVALID;

/// Connectivity of a [`TriMesh`].
///
/// Half-edge `3 * f + k` runs from corner `k` to corner `(k + 1) % 3` of
/// triangle `f`.
#[derive(Clone, Debug, Default)]
pub struct Topology {
    /// Twin of each half-edge, or [`INVALID`] on a boundary.
    pub opposite: Vec<u32>,
    /// One-ring vertex neighbours, sorted by index.
    pub neighbors: Vec<Vec<u32>>,
    /// Directed edges used by more than one triangle (non-manifold or
    /// inconsistently wound input).
    pub non_manifold_edges: usize,
}

impl Topology {
    pub fn build(mesh: &TriMesh) -> Self {
        let half_edges = 3 * mesh.triangles.len();
        let mut by_endpoints: HashMap<(u32, u32), u32> = HashMap::with_capacity(half_edges);
        let mut non_manifold_edges = 0;
        for h in 0..half_edges {
            if by_endpoints.insert(half_edge_vertices(mesh, h), h as u32).is_some() {
                non_manifold_edges += 1;
            }
        }

        let mut neighbors = vec![Vec::new(); mesh.positions.len()];
        let opposite = (0..half_edges)
            .map(|h| {
                let (a, b) = half_edge_vertices(mesh, h);
                neighbors[a as usize].push(b);
                neighbors[b as usize].push(a);
                by_endpoints.get(&(b, a)).copied().unwrap_or(INVALID)
            })
            .collect();
        for list in &mut neighbors {
            list.sort_unstable();
            list.dedup();
        }

        Self { opposite, neighbors, non_manifold_edges }
    }

    pub fn boundary_half_edges(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.opposite.len()).filter(|&h| self.opposite[h] == INVALID)
    }

    pub fn is_closed(&self) -> bool {
        self.boundary_half_edges().next().is_none()
    }

    /// Number of undirected edges.
    pub fn edge_count(&self) -> usize {
        self.neighbors.iter().map(Vec::len).sum::<usize>() / 2
    }
}

#[inline]
pub fn half_edge_vertices(mesh: &TriMesh, h: usize) -> (u32, u32) {
    let tri = mesh.triangles[h / 3];
    (tri[h % 3], tri[(h + 1) % 3])
}
