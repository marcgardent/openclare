//! Multi-resolution vertex graph.
//!
//! Local smoothing only propagates information one ring per sweep and gets
//! stuck in poor local minima (lots of spurious singularities) when run on
//! the full-resolution mesh alone. Solving coarse-to-fine on a hierarchy of
//! progressively merged vertices fixes both the speed and the quality.

use crate::field::Constraints;
use crate::math::{INVALID, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Link {
    pub id: u32,
    pub weight: f64,
}

/// Weighted adjacency in compressed sparse row form.
#[derive(Clone, Debug)]
pub struct Graph {
    offsets: Vec<u32>,
    links: Vec<Link>,
}

impl Graph {
    pub fn from_lists(lists: &[Vec<Link>]) -> Self {
        let mut offsets = Vec::with_capacity(lists.len() + 1);
        let mut links = Vec::with_capacity(lists.iter().map(Vec::len).sum());
        offsets.push(0);
        for list in lists {
            links.extend_from_slice(list);
            offsets.push(links.len() as u32);
        }
        Self { offsets, links }
    }

    /// Unit weight on every edge.
    pub fn uniform(neighbors: &[Vec<u32>]) -> Self {
        let lists: Vec<Vec<Link>> =
            neighbors.iter().map(|list| list.iter().map(|&id| Link { id, weight: 1.0 }).collect()).collect();
        Self::from_lists(&lists)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.offsets.len() - 1
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn neighbors(&self, i: usize) -> &[Link] {
        &self.links[self.offsets[i] as usize..self.offsets[i + 1] as usize]
    }
}

/// One resolution of the hierarchy: an oriented point cloud with adjacency,
/// plus the two fields being solved for.
#[derive(Clone, Debug)]
pub struct Level {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub areas: Vec<f64>,
    pub graph: Graph,
    pub constraints: Constraints,
    /// Orientation field: one representative unit tangent of the cross.
    pub orientation: Vec<Vec3>,
    /// Position field: a point of the local lattice, in the tangent plane.
    pub origin: Vec<Vec3>,
    /// The one or two vertices of the next finer level merged into each
    /// vertex (second is [`INVALID`] when unpaired). Empty on level 0.
    pub children: Vec<[u32; 2]>,
}

impl Level {
    pub fn new(
        positions: Vec<Vec3>,
        normals: Vec<Vec3>,
        areas: Vec<f64>,
        graph: Graph,
        constraints: Constraints,
    ) -> Self {
        let n = positions.len();
        Self {
            orientation: vec![Vec3::ZERO; n],
            origin: positions.clone(),
            positions,
            normals,
            areas,
            graph,
            constraints,
            children: Vec::new(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// Merges adjacent vertex pairs, most similar normals first. Returns
    /// `None` when nothing can be merged any more.
    fn coarsen(&self) -> Option<Level> {
        let n = self.len();
        let mut candidates: Vec<(f64, u32, u32)> = Vec::new();
        for i in 0..n {
            for link in self.graph.neighbors(i) {
                let j = link.id as usize;
                if j <= i {
                    continue;
                }
                let (ai, aj) = (self.areas[i], self.areas[j]);
                let ratio = if ai > aj { ai / aj } else { aj / ai };
                candidates.push((self.normals[i].dot(self.normals[j]) * ratio, i as u32, j as u32));
            }
        }
        // Ties broken by index so the hierarchy is deterministic.
        candidates.sort_unstable_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));

        let mut parent = vec![INVALID; n];
        let mut children: Vec<[u32; 2]> = Vec::with_capacity(n / 2 + 1);
        for &(_, i, j) in &candidates {
            if parent[i as usize] == INVALID && parent[j as usize] == INVALID {
                parent[i as usize] = children.len() as u32;
                parent[j as usize] = children.len() as u32;
                children.push([i, j]);
            }
        }
        for i in 0..n {
            if parent[i] == INVALID {
                parent[i] = children.len() as u32;
                children.push([i as u32, INVALID]);
            }
        }
        if children.len() == n {
            return None;
        }

        let m = children.len();
        let mut positions = Vec::with_capacity(m);
        let mut normals = Vec::with_capacity(m);
        let mut areas = Vec::with_capacity(m);
        let mut lists: Vec<Vec<Link>> = Vec::with_capacity(m);
        for (c, pair) in children.iter().enumerate() {
            let members = || pair.iter().filter(|&&v| v != INVALID).map(|&v| v as usize);
            let area: f64 = members().map(|v| self.areas[v]).sum();
            let position = if area > 0.0 {
                members().map(|v| self.positions[v] * self.areas[v]).sum::<Vec3>() / area
            } else {
                members().map(|v| self.positions[v]).sum::<Vec3>() / members().count() as f64
            };
            let normal = members().map(|v| self.normals[v] * self.areas[v]).sum::<Vec3>();
            positions.push(position);
            normals.push(normal.try_normalize().unwrap_or(self.normals[pair[0] as usize]));
            areas.push(area);

            let mut list: Vec<Link> = members()
                .flat_map(|v| self.graph.neighbors(v))
                .map(|link| Link { id: parent[link.id as usize], weight: link.weight })
                .filter(|link| link.id as usize != c)
                .collect();
            list.sort_unstable_by_key(|link| link.id);
            list.dedup_by(|next, kept| {
                let same = next.id == kept.id;
                if same {
                    kept.weight += next.weight;
                }
                same
            });
            lists.push(list);
        }

        let constraints = self.constraints.restrict(&children, &self.normals, &positions, &normals);
        let mut level = Level::new(positions, normals, areas, Graph::from_lists(&lists), constraints);
        level.children = children;
        Some(level)
    }
}

/// Levels from finest (`levels[0]`, the input vertices) to coarsest.
#[derive(Clone, Debug)]
pub struct Hierarchy {
    pub levels: Vec<Level>,
}

impl Hierarchy {
    pub fn build(finest: Level) -> Self {
        let mut levels = vec![finest];
        while levels.len() < 64 && levels.last().unwrap().len() > 1 {
            match levels.last().unwrap().coarsen() {
                Some(coarser) => levels.push(coarser),
                None => break,
            }
        }
        Self { levels }
    }

    pub fn finest(&self) -> &Level {
        &self.levels[0]
    }

    /// Runs `visit(coarse, fine)` for each adjacent pair, coarsest first.
    pub(crate) fn for_each_coarse_to_fine(&mut self, mut visit: impl FnMut(&mut Level, Option<&mut Level>)) {
        for l in (0..self.levels.len()).rev() {
            let (finer, coarser) = self.levels.split_at_mut(l);
            visit(&mut coarser[0], finer.last_mut());
        }
    }
}
