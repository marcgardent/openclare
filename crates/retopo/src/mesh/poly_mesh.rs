use std::collections::{BTreeMap, HashMap};

use super::TriMesh;
use crate::math::{INVALID, Vec3};

/// Polygon mesh: the output representation of the remesher.
#[derive(Clone, Debug, Default)]
pub struct PolyMesh {
    pub positions: Vec<Vec3>,
    pub faces: Vec<Vec<u32>>,
}

/// Topological quality summary of a [`PolyMesh`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MeshStats {
    pub vertices: usize,
    pub edges: usize,
    pub faces: usize,
    pub triangles: usize,
    pub quads: usize,
    /// Faces with five sides or more.
    pub ngons: usize,
    pub boundary_edges: usize,
    pub non_manifold_edges: usize,
    /// Interior vertices whose valence is not 4.
    pub irregular_vertices: usize,
    /// Interior vertex count per valence.
    pub valences: BTreeMap<usize, usize>,
}

impl MeshStats {
    pub fn quad_ratio(&self) -> f64 {
        self.quads as f64 / self.faces.max(1) as f64
    }

    /// `V - E + F`: 2 for a sphere, 0 for a torus.
    pub fn euler_characteristic(&self) -> i64 {
        self.vertices as i64 - self.edges as i64 + self.faces as i64
    }
}

impl PolyMesh {
    /// Undirected edges with the number of faces using each. Ordered, so that
    /// whatever is derived from it does not depend on hashing.
    pub fn edge_uses(&self) -> BTreeMap<(u32, u32), u32> {
        let mut uses = BTreeMap::new();
        for face in &self.faces {
            for k in 0..face.len() {
                let (a, b) = (face[k], face[(k + 1) % face.len()]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
        uses
    }

    pub fn stats(&self) -> MeshStats {
        let uses = self.edge_uses();
        let mut valence = vec![0usize; self.positions.len()];
        let mut on_boundary = vec![false; self.positions.len()];
        let mut stats = MeshStats {
            vertices: self.positions.len(),
            edges: uses.len(),
            faces: self.faces.len(),
            ..Default::default()
        };

        for (&(a, b), &count) in &uses {
            valence[a as usize] += 1;
            valence[b as usize] += 1;
            match count {
                1 => {
                    stats.boundary_edges += 1;
                    on_boundary[a as usize] = true;
                    on_boundary[b as usize] = true;
                }
                2 => {}
                _ => stats.non_manifold_edges += 1,
            }
        }
        for face in &self.faces {
            match face.len() {
                3 => stats.triangles += 1,
                4 => stats.quads += 1,
                _ => stats.ngons += 1,
            }
        }
        for (v, &val) in valence.iter().enumerate() {
            if on_boundary[v] || val == 0 {
                continue;
            }
            *stats.valences.entry(val).or_insert(0) += 1;
            if val != 4 {
                stats.irregular_vertices += 1;
            }
        }
        stats
    }

    /// Splits every n-gon into n quads around its centroid (one linear
    /// Catmull-Clark step). The result is a pure quad mesh whatever the input.
    pub fn split_to_quads(&self) -> PolyMesh {
        let mut positions = self.positions.clone();
        let mut midpoints: HashMap<(u32, u32), u32> = HashMap::new();
        let mut faces = Vec::with_capacity(4 * self.faces.len());

        for face in &self.faces {
            let n = face.len();
            let mids: Vec<u32> = (0..n)
                .map(|k| {
                    let (a, b) = (face[k], face[(k + 1) % n]);
                    *midpoints.entry((a.min(b), a.max(b))).or_insert_with(|| {
                        positions.push(0.5 * (self.positions[a as usize] + self.positions[b as usize]));
                        (positions.len() - 1) as u32
                    })
                })
                .collect();
            let centroid = face.iter().map(|&v| self.positions[v as usize]).sum::<Vec3>() / n as f64;
            let center = positions.len() as u32;
            positions.push(centroid);
            for k in 0..n {
                faces.push(vec![face[k], mids[k], center, mids[(k + n - 1) % n]]);
            }
        }
        PolyMesh { positions, faces }
    }

    /// Removes interior vertices of valence 2: they sit in the middle of what
    /// is really a single edge between two faces. Returns how many went.
    pub fn dissolve_valence_two(&mut self) -> usize {
        let mut removed = 0;
        loop {
            let uses = self.edge_uses();
            let mut valence = vec![0u32; self.positions.len()];
            let mut interior = vec![true; self.positions.len()];
            for (&(a, b), &count) in &uses {
                for v in [a, b] {
                    valence[v as usize] += 1;
                    interior[v as usize] &= count == 2;
                }
            }
            let mut incident: Vec<Vec<u32>> = vec![Vec::new(); self.positions.len()];
            for (f, face) in self.faces.iter().enumerate() {
                for &v in face {
                    incident[v as usize].push(f as u32);
                }
            }

            // A face is edited at most once per pass so the incidences above
            // stay valid.
            let mut touched = vec![false; self.faces.len()];
            let mut changed = false;
            for v in 0..self.positions.len() {
                let [f0, f1] = incident[v][..] else { continue };
                let (f0, f1) = (f0 as usize, f1 as usize);
                let triangles = (self.faces[f0].len() == 3) as u8 + (self.faces[f1].len() == 3) as u8;
                if valence[v] != 2 || !interior[v] || touched[f0] || touched[f1] || triangles == 2 {
                    continue;
                }
                for f in [f0, f1] {
                    touched[f] = true;
                    if self.faces[f].len() == 3 {
                        // Its third side is the edge the neighbours already share.
                        self.faces[f].clear();
                    } else {
                        self.faces[f].retain(|&x| x as usize != v);
                    }
                }
                removed += 1;
                changed = true;
            }
            if !changed {
                break;
            }
            self.faces.retain(|face| !face.is_empty());
        }
        self.compact();
        removed
    }

    /// Drops the vertices no face references.
    pub fn compact(&mut self) {
        let mut remap = vec![INVALID; self.positions.len()];
        let mut positions = Vec::new();
        for face in &mut self.faces {
            for v in face.iter_mut() {
                if remap[*v as usize] == INVALID {
                    remap[*v as usize] = positions.len() as u32;
                    positions.push(self.positions[*v as usize]);
                }
                *v = remap[*v as usize];
            }
        }
        self.positions = positions;
    }

    /// Fan triangulation, for consumers that only take triangles.
    pub fn triangulated(&self) -> TriMesh {
        let mut triangles = Vec::new();
        for face in &self.faces {
            for k in 1..face.len().saturating_sub(1) {
                triangles.push([face[0], face[k], face[k + 1]]);
            }
        }
        TriMesh { positions: self.positions.clone(), triangles }
    }

    pub fn surface_area(&self) -> f64 {
        self.triangulated().surface_area()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pentagon_and_triangle() -> PolyMesh {
        let positions = (0..6).map(|i| Vec3::new(i as f64, (i * i) as f64, 0.0)).collect();
        PolyMesh { positions, faces: vec![vec![0, 1, 2, 3, 4], vec![0, 4, 5]] }
    }

    #[test]
    fn splitting_yields_one_quad_per_corner() {
        let quads = pentagon_and_triangle().split_to_quads();
        let stats = quads.stats();
        assert_eq!((stats.faces, stats.quads), (8, 8));
        // 6 corners + 7 edge midpoints + 2 centres; the shared edge is split once.
        assert_eq!(stats.vertices, 15);
        assert_eq!(stats.non_manifold_edges, 0);
    }

    #[test]
    fn interior_valence_two_vertices_are_dissolved() {
        // Two quads glued along the path 1-4-2: vertex 4 is redundant.
        let positions = (0..7).map(|i| Vec3::new(i as f64, 0.0, 0.0)).collect();
        let mut mesh = PolyMesh { positions, faces: vec![vec![0, 1, 4, 2, 3], vec![1, 5, 6, 2, 4]] };
        assert_eq!(mesh.dissolve_valence_two(), 1);
        assert_eq!(mesh.positions.len(), 6);
        assert_eq!(mesh.stats().quads, 2);
    }
}
