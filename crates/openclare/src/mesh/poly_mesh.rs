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

    /// Turns the faces that are not quads into quads where that only takes
    /// moving edges, not vertices: two odd faces sharing an edge are merged,
    /// and any face with an even number of sides is cut into quads from one
    /// of its corners. Odd faces with no odd neighbour are left as they are.
    /// Returns how many non-quad faces remain.
    pub fn repair_to_quads(&mut self) -> usize {
        // Merge pairs of odd faces across the edge they share.
        let mut across: HashMap<(u32, u32), usize> = HashMap::new();
        for (f, face) in self.faces.iter().enumerate() {
            if face.len() % 2 == 1 {
                for k in 0..face.len() {
                    across.insert((face[k], face[(k + 1) % face.len()]), f);
                }
            }
        }
        let mut merged = vec![false; self.faces.len()];
        let mut additions = Vec::new();
        for f in 0..self.faces.len() {
            if merged[f] || self.faces[f].len().is_multiple_of(2) {
                continue;
            }
            let face = &self.faces[f];
            let n = face.len();
            let partner = (0..n).find_map(|k| {
                let g = *across.get(&(face[(k + 1) % n], face[k]))?;
                (g != f && !merged[g]).then_some((k, g))
            });
            let Some((k, g)) = partner else { continue };
            let other = &self.faces[g];
            let m = other.len();
            // The shared edge runs a -> b in `face` and b -> a in `other`.
            let (a, b) = (face[k], face[(k + 1) % n]);
            let at = (0..m).find(|&j| other[j] == b && other[(j + 1) % m] == a).unwrap();
            // Round `face` from b back to a, then round `other` from a to b,
            // leaving out the shared edge itself.
            let mut polygon: Vec<u32> = (1..=n).map(|i| face[(k + i) % n]).collect();
            polygon.extend((2..m).map(|j| other[(at + j) % m]));
            let simple = {
                let mut sorted = polygon.clone();
                sorted.sort_unstable();
                sorted.windows(2).all(|w| w[0] != w[1])
            };
            if simple {
                merged[f] = true;
                merged[g] = true;
                additions.push(polygon);
            }
        }
        let mut faces: Vec<Vec<u32>> =
            self.faces.drain(..).zip(merged).filter(|(_, gone)| !gone).map(|(face, _)| face).collect();
        faces.extend(additions);

        // Cut even faces of six sides or more into quads, fanning from the
        // corner that leaves the shortest diagonals.
        let mut out = Vec::with_capacity(faces.len());
        for face in faces {
            let n = face.len();
            if n < 6 || n % 2 == 1 {
                out.push(face);
                continue;
            }
            // The fan from a corner uses the diagonals to every second
            // vertex after its neighbour: 3, 5, ... steps round the face.
            let reach = |start: usize| -> f64 {
                (3..n - 1)
                    .step_by(2)
                    .map(|j| {
                        self.positions[face[start] as usize].distance(self.positions[face[(start + j) % n] as usize])
                    })
                    .sum()
            };
            let start = (0..n).min_by(|&a, &b| reach(a).total_cmp(&reach(b))).unwrap();
            for i in 0..(n - 2) / 2 {
                out.push(vec![
                    face[start],
                    face[(start + 2 * i + 1) % n],
                    face[(start + 2 * i + 2) % n],
                    face[(start + 2 * i + 3) % n],
                ]);
            }
        }
        self.faces = out;
        self.faces.iter().filter(|face| face.len() != 4).count()
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

    /// Removes the pairs of faces with the same corners the other way round:
    /// a lone loop of edges traced as a face from either side, two faces
    /// back to back that enclose nothing, one of which faces against the
    /// surface whatever is done. Returns how many pairs went.
    pub fn drop_back_to_back(&mut self) -> usize {
        let reversed = |face: &[u32], other: &[u32]| {
            let n = face.len();
            other.len() == n
                && other
                    .iter()
                    .position(|&v| v == face[0])
                    .is_some_and(|at| (0..n).all(|i| other[(at + n - i) % n] == face[i]))
        };
        let mut seen: HashMap<Vec<u32>, usize> = HashMap::new();
        let mut dropped = vec![false; self.faces.len()];
        let mut pairs = 0;
        for (f, face) in self.faces.iter().enumerate() {
            let mut corners = face.clone();
            corners.sort_unstable();
            match seen.get(&corners) {
                Some(&g) if reversed(face, &self.faces[g]) => {
                    (dropped[f], dropped[g]) = (true, true);
                    seen.remove(&corners);
                    pairs += 1;
                }
                _ => {
                    seen.insert(corners, f);
                }
            }
        }
        if pairs > 0 {
            let mut f = 0;
            self.faces.retain(|_| {
                f += 1;
                !dropped[f - 1]
            });
            self.compact();
        }
        pairs
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
    fn odd_pairs_and_even_polygons_become_quads() {
        // A triangle and a pentagon glued along 0-1, next to a hexagon.
        let positions = (0..12).map(|i| Vec3::new((i as f64).cos(), (i as f64).sin(), 0.0)).collect();
        let mut mesh = PolyMesh {
            positions,
            faces: vec![vec![0, 1, 2], vec![1, 0, 3, 4, 5], vec![6, 7, 8, 9, 10, 11], vec![0, 2, 6, 7]],
        };
        assert_eq!(mesh.repair_to_quads(), 0);
        // Triangle + pentagon -> one hexagon -> two quads; hexagon -> two quads.
        assert_eq!(mesh.faces.len(), 5);
        assert!(mesh.faces.iter().all(|face| face.len() == 4));
        let mut used: Vec<u32> = mesh.faces.iter().flatten().copied().collect();
        used.sort_unstable();
        used.dedup();
        assert_eq!(used.len(), 12, "no vertex may be lost");

        // A lone triangle has nothing to pair with.
        let mut lone = PolyMesh { positions: mesh.positions.clone(), faces: vec![vec![0, 1, 2], vec![2, 1, 3, 4]] };
        assert_eq!(lone.repair_to_quads(), 1);
    }

    #[test]
    fn faces_back_to_back_go_and_the_rest_stays() {
        let positions = (0..6).map(|i| Vec3::new(i as f64, (i * i) as f64, 0.0)).collect();
        // A triangle traced from both sides, and a quad next to its twin
        // with the same corners in another order, which is not its back.
        let mut mesh = PolyMesh {
            positions,
            faces: vec![vec![0, 1, 2], vec![3, 2, 4, 5], vec![1, 0, 2], vec![3, 5, 2, 4], vec![2, 1, 0]],
        };
        assert_eq!(mesh.drop_back_to_back(), 1);
        assert_eq!(mesh.faces.len(), 3);
        assert_eq!(mesh.positions.len(), 6);
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
