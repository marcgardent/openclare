//! Cutting the surface open and combing the cross field.
//!
//! A cross field cannot in general be told apart into "the u arm" and "the v
//! arm" over a whole surface: going round a singularity or a handle, the arms
//! swap. Cut the surface into a disk along a graph of edges through all the
//! singularities, though, and inside the disk the arms can be labelled
//! consistently ("combed"). The labels then disagree only across the cut, by
//! a known number of quarter turns per edge: the *transition*.

use std::collections::VecDeque;

use crate::crossfield::CrossField;
use crate::math::INVALID;
use crate::mesh::{Topology, TriMesh};

pub struct CutMesh {
    /// Per half-edge: whether it lies on the cut. Borders of the surface are
    /// not seams.
    pub seam: Vec<bool>,
    /// Per corner `3 * f + k`: the vertex of the cut-open mesh it belongs to.
    /// A vertex of the surface on the cut has one of these per sector.
    pub wedge: Vec<u32>,
    pub wedge_count: usize,
    /// Per face: quarter turns to add to its stored cross so that the field
    /// is combed.
    pub turns: Vec<i32>,
    /// Per half-edge on a seam, from its face to the one across: quarter
    /// turns between their combed crosses, in `0..4`.
    pub transition: Vec<u8>,
}

impl CutMesh {
    pub fn build(mesh: &TriMesh, topology: &Topology, field: &CrossField) -> Self {
        let faces = mesh.triangles.len();
        let twin = &topology.opposite;

        // A spanning tree of the faces, combing the field as it grows: each
        // new face takes the arm that continues its parent's.
        let mut turns = vec![0i32; faces];
        let mut visited = vec![false; faces];
        let mut in_tree = vec![false; 3 * faces];
        let mut queue = VecDeque::new();
        for root in 0..faces {
            if visited[root] {
                continue;
            }
            visited[root] = true;
            queue.push_back(root);
            while let Some(f) = queue.pop_front() {
                for h in 3 * f..3 * f + 3 {
                    let t = twin[h];
                    if t == INVALID || visited[t as usize / 3] {
                        continue;
                    }
                    let g = t as usize / 3;
                    visited[g] = true;
                    in_tree[h] = true;
                    in_tree[t as usize] = true;
                    turns[g] = turns[f] - field.period_jumps[h];
                    queue.push_back(g);
                }
            }
        }

        // Every edge the tree does not cross is a cut; together they already
        // open the surface into a disk. Most of them are dead ends, though:
        // trim those back, stopping at singularities (which must stay on the
        // cut) and at the border (which is a cut of its own).
        let mut seam: Vec<bool> = (0..3 * faces).map(|h| twin[h] != INVALID && !in_tree[h]).collect();
        let mut anchored: Vec<bool> = field.indices.iter().map(|&index| index != 0).collect();
        let mut degree = vec![0u32; mesh.positions.len()];
        for h in 0..3 * faces {
            let (a, b) = (mesh.triangles[h / 3][h % 3], mesh.triangles[h / 3][(h + 1) % 3]);
            if twin[h] == INVALID {
                anchored[a as usize] = true;
                anchored[b as usize] = true;
            } else if seam[h] {
                degree[a as usize] += 1;
            }
        }
        // Seam half-edges leaving each vertex, to find the last one of a dead end.
        let mut leaving: Vec<Vec<u32>> = vec![Vec::new(); mesh.positions.len()];
        for h in 0..3 * faces {
            if seam[h] {
                leaving[mesh.triangles[h / 3][h % 3] as usize].push(h as u32);
            }
        }
        let mut dead_ends: Vec<u32> =
            (0..mesh.positions.len() as u32).filter(|&v| degree[v as usize] == 1 && !anchored[v as usize]).collect();
        while let Some(v) = dead_ends.pop() {
            let Some(&h) = leaving[v as usize].iter().find(|&&h| seam[h as usize]) else { continue };
            let (h, t) = (h as usize, twin[h as usize] as usize);
            seam[h] = false;
            seam[t] = false;
            let other = mesh.triangles[h / 3][(h + 1) % 3] as usize;
            degree[v as usize] -= 1;
            degree[other] -= 1;
            if degree[other] == 1 && !anchored[other] {
                dead_ends.push(other as u32);
            }
        }

        // Corners that meet at a vertex without a seam between them are the
        // same vertex of the cut-open mesh.
        let mut sets = Corners::new(3 * faces);
        for h in 0..3 * faces {
            let t = twin[h];
            if t == INVALID || seam[h] {
                continue;
            }
            let t = t as usize;
            // h runs a -> b in its face, its twin b -> a in the other.
            sets.union(h as u32, next_corner(t) as u32);
            sets.union(next_corner(h) as u32, t as u32);
        }
        let mut wedge = vec![INVALID; 3 * faces];
        let mut wedge_count = 0;
        for corner in 0..3 * faces {
            let root = sets.find(corner as u32) as usize;
            if wedge[root] == INVALID {
                wedge[root] = wedge_count as u32;
                wedge_count += 1;
            }
            wedge[corner] = wedge[root];
        }

        let transition = (0..3 * faces)
            .map(|h| {
                if !seam[h] {
                    return 0;
                }
                let (f, g) = (h / 3, twin[h] as usize / 3);
                (field.period_jumps[h] - turns[f] + turns[g]).rem_euclid(4) as u8
            })
            .collect();

        Self { seam, wedge, wedge_count, turns, transition }
    }

    /// Number of edges on the cut.
    pub fn seam_edges(&self) -> usize {
        self.seam.iter().filter(|&&s| s).count() / 2
    }
}

/// The corner after `corner` in its face.
#[inline]
pub(super) fn next_corner(corner: usize) -> usize {
    3 * (corner / 3) + (corner % 3 + 1) % 3
}

/// Disjoint sets over corners.
struct Corners {
    parent: Vec<u32>,
}

impl Corners {
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
