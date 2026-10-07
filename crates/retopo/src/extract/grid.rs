//! Reading a quad mesh off an integer-grid parametrization.
//!
//! Every point of the surface belongs to the grid point nearest to it in
//! `(u, v)`. Refine the triangles until several vertices fall in each such
//! cell; then the vertices sharing a grid point collapse into one quad
//! vertex, and two quad vertices are joined when some triangle edge steps
//! from the cell of one into the cell of the other, one unit apart.
//!
//! This is the extraction of Instant Meshes fed with a globally consistent
//! grid. It never has to follow a grid line across a seam, which is what
//! makes the exact approach (QEx) delicate, and it shrugs off the odd folded
//! triangle; in exchange it needs the refinement, and where the map is badly
//! broken it leaves a defect instead of failing.

use std::collections::HashMap;

use super::LatticeGraph;
use crate::math::Vec3;
use crate::mesh::TriMesh;

/// Longest triangle edge, in grid units, the classification can work with.
const MAX_EDGE: f64 = 0.4;

/// Builds the quad graph of the map `uv` (coordinates of corner `3 * f + k`)
/// over `mesh`.
pub fn graph_from_parametrization(mesh: &TriMesh, normals: &[Vec3], uv: &[[f64; 2]]) -> LatticeGraph {
    let refined = Refined::new(mesh, normals, uv);
    let vertices = refined.positions.len();
    let grid_point = |corner: usize| [refined.uv[corner][0].round() as i64, refined.uv[corner][1].round() as i64];

    let mut sets = Sets::new(vertices);
    let mut steps: Vec<(u32, u32)> = Vec::new();
    for (f, tri) in refined.triangles.iter().enumerate() {
        for k in 0..3 {
            let (here, there) = (3 * f + k, 3 * f + (k + 1) % 3);
            let (p, q) = (grid_point(here), grid_point(there));
            match ((p[0] - q[0]).abs(), (p[1] - q[1]).abs()) {
                (0, 0) => sets.union(tri[k], tri[(k + 1) % 3]),
                (1, 0) | (0, 1) => steps.push((tri[k], tri[(k + 1) % 3])),
                _ => {}
            }
        }
    }

    let mut cluster = vec![u32::MAX; vertices];
    let mut graph = LatticeGraph::default();
    for v in 0..vertices as u32 {
        let root = sets.find(v) as usize;
        if cluster[root] == u32::MAX {
            cluster[root] = graph.positions.len() as u32;
            graph.positions.push(Vec3::ZERO);
            graph.normals.push(Vec3::ZERO);
            graph.members.push(0);
        }
        cluster[v as usize] = cluster[root];
        let c = cluster[root] as usize;
        graph.normals[c] += refined.normals[v as usize];
        graph.members[c] += 1;
    }

    // Each quad vertex goes where its grid point really is: inside one of
    // the triangles of its cell. Where the map is too damaged for that, the
    // members of the cell stand in, the nearest to the grid point most.
    let mut located = vec![false; graph.positions.len()];
    let mut fallback = vec![(Vec3::ZERO, 0.0); graph.positions.len()];
    for (f, tri) in refined.triangles.iter().enumerate() {
        let corners = [refined.uv[3 * f], refined.uv[3 * f + 1], refined.uv[3 * f + 2]];
        for k in 0..3 {
            let c = cluster[tri[k] as usize] as usize;
            let target = grid_point(3 * f + k).map(|n| n as f64);
            let offset = [corners[k][0] - target[0], corners[k][1] - target[1]];
            let weight = (-9.0 * (offset[0] * offset[0] + offset[1] * offset[1])).exp();
            fallback[c].0 += refined.positions[tri[k] as usize] * weight;
            fallback[c].1 += weight;
            if !located[c]
                && let Some(weights) = barycentric(&corners, target)
            {
                graph.positions[c] = (0..3).map(|i| refined.positions[tri[i] as usize] * weights[i]).sum();
                located[c] = true;
            }
        }
    }
    for c in 0..graph.positions.len() {
        if !located[c] && fallback[c].1 > 0.0 {
            graph.positions[c] = fallback[c].0 / fallback[c].1;
        }
        graph.normals[c] = graph.normals[c].normalize_or_zero();
    }

    graph.adjacency = vec![Vec::new(); graph.positions.len()];
    for (a, b) in steps {
        let (a, b) = (cluster[a as usize], cluster[b as usize]);
        if a != b {
            graph.adjacency[a as usize].push(b);
            graph.adjacency[b as usize].push(a);
        }
    }
    for list in &mut graph.adjacency {
        list.sort_unstable();
        list.dedup();
    }
    graph
}

/// Barycentric coordinates of `p` in the triangle, if it lies inside (edges
/// included).
fn barycentric(corners: &[[f64; 2]; 3], p: [f64; 2]) -> Option<[f64; 3]> {
    let [a, b, c] = *corners;
    let det = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    if det.abs() < 1e-300 {
        return None;
    }
    let v = ((p[0] - a[0]) * (c[1] - a[1]) - (p[1] - a[1]) * (c[0] - a[0])) / det;
    let w = ((b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])) / det;
    let u = 1.0 - v - w;
    (u >= -1e-9 && v >= -1e-9 && w >= -1e-9).then_some([u, v, w])
}

/// The mesh with every triangle split in four, as many times as it takes for
/// all edges to be short in `(u, v)`. Vertices stay shared; `uv` stays per
/// corner, since it differs on the two sides of a seam.
struct Refined {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    uv: Vec<[f64; 2]>,
}

impl Refined {
    fn new(mesh: &TriMesh, normals: &[Vec3], uv: &[[f64; 2]]) -> Self {
        let mut refined = Self {
            positions: mesh.positions.clone(),
            normals: normals.to_vec(),
            triangles: mesh.triangles.clone(),
            uv: uv.to_vec(),
        };
        let longest = (0..uv.len())
            .map(|c| {
                let next = 3 * (c / 3) + (c % 3 + 1) % 3;
                (uv[c][0] - uv[next][0]).hypot(uv[c][1] - uv[next][1])
            })
            .filter(|length| length.is_finite())
            .fold(0.0, f64::max);
        // A few badly stretched triangles must not make the whole mesh
        // explode: beyond four levels (256 times the triangles) they are left.
        let levels = ((longest / MAX_EDGE).log2().ceil().max(0.0) as usize).min(4);
        for _ in 0..levels {
            refined.split();
        }
        refined
    }

    fn split(&mut self) {
        let mut midpoints: HashMap<(u32, u32), u32> = HashMap::new();
        let mut triangles = Vec::with_capacity(4 * self.triangles.len());
        let mut uv = Vec::with_capacity(4 * self.uv.len());
        for (f, &tri) in self.triangles.iter().enumerate() {
            let corner_uv = [self.uv[3 * f], self.uv[3 * f + 1], self.uv[3 * f + 2]];
            let mut middle = [0u32; 3];
            let mut middle_uv = [[0.0; 2]; 3];
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                middle[k] = *midpoints.entry((a.min(b), a.max(b))).or_insert_with(|| {
                    self.positions.push(0.5 * (self.positions[a as usize] + self.positions[b as usize]));
                    self.normals.push((self.normals[a as usize] + self.normals[b as usize]).normalize_or_zero());
                    (self.positions.len() - 1) as u32
                });
                let (p, q) = (corner_uv[k], corner_uv[(k + 1) % 3]);
                middle_uv[k] = [0.5 * (p[0] + q[0]), 0.5 * (p[1] + q[1])];
            }
            for (vertices, coordinates) in [
                ([tri[0], middle[0], middle[2]], [corner_uv[0], middle_uv[0], middle_uv[2]]),
                ([middle[0], tri[1], middle[1]], [middle_uv[0], corner_uv[1], middle_uv[1]]),
                ([middle[2], middle[1], tri[2]], [middle_uv[2], middle_uv[1], corner_uv[2]]),
                ([middle[0], middle[1], middle[2]], middle_uv),
            ] {
                triangles.push(vertices);
                uv.extend(coordinates);
            }
        }
        self.triangles = triangles;
        self.uv = uv;
    }
}

struct Sets {
    parent: Vec<u32>,
}

impl Sets {
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
