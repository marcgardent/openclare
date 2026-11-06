//! Catmull-Clark subdivision as linear maps: each point it makes, a vertex
//! of the refined mesh or a point of the limit surface, as weights on the
//! vertices of the mesh it starts from.
//!
//! The rules are those of Blender's Subdivision Surface modifier with its
//! defaults, which is what an artist gets: smooth everywhere, a border
//! subdivided as a cubic B-spline, the corners of a border smoothed with
//! it, and an edge of more than two faces kept sharp. Creases are not
//! looked at.
//!
//! The limit position of a vertex where only quads meet is the mask of
//! Halstead, Kass and DeRose, "Efficient, Fair Interpolation using
//! Catmull-Clark Surfaces" (SIGGRAPH 1993): for valence `n`,
//! `(n² v + 4 Σ e + Σ d) / (n (n + 5))`, over its `n` neighbours along an
//! edge `e` and the `n` corners `d` facing it across a quad. On a regular
//! grid that is the bicubic B-spline: 4/9, 1/9 and 1/36. On a border it is
//! the cubic B-spline of the border, `(a + 4 v + b) / 6`.

use std::collections::HashMap;

use crate::math::Vec3;

/// A point made of the vertices of a mesh: each with its weight. The
/// weights add up to 1 and none is negative.
pub type Stencil = Vec<(u32, f64)>;

/// Where `stencil` puts its point when the vertices are at `positions`.
pub fn apply(stencil: &Stencil, positions: &[Vec3]) -> Vec3 {
    stencil.iter().map(|&(v, w)| positions[v as usize] * w).sum()
}

/// One step of subdivision.
#[derive(Clone, Debug)]
pub struct Refined {
    /// The quads of the refined mesh, turning the way their face did. The
    /// quads of a face of `n` corners are `n` in a row, from its first
    /// corner on.
    pub faces: Vec<Vec<u32>>,
    /// Each vertex of the refined mesh as weights on the vertices of the
    /// mesh refined: first one for each vertex, then one for each face,
    /// then one for each edge.
    pub rows: Vec<Stencil>,
    /// For each quad of the refined mesh, the face it is part of.
    pub parent: Vec<u32>,
}

/// Subdivides the mesh of `vertices` vertices and these faces once.
pub fn refine(faces: &[Vec<u32>], vertices: usize) -> Refined {
    let around = Around::of(faces, vertices);
    let edges = around.edges.len();
    let face_point = |f: usize| -> Stencil {
        let corners = faces[f].len() as f64;
        faces[f].iter().map(|&v| (v, 1.0 / corners)).collect()
    };
    let mut rows = Vec::with_capacity(vertices + faces.len() + edges);
    for v in 0..vertices {
        let row = match around.role(v) {
            Role::Smooth => {
                // (n - 2)/n of the vertex, 1/n² of each neighbour and of
                // each face point around it.
                let n = around.vertex_edges[v].len() as f64;
                let mut sum = Sum::default();
                sum.add(v as u32, (n - 2.0) / n);
                for &e in &around.vertex_edges[v] {
                    sum.add(around.other_end(e, v), 1.0 / (n * n));
                }
                for &f in &around.vertex_faces[v] {
                    sum.add_all(&face_point(f as usize), 1.0 / (n * n));
                }
                sum.done()
            }
            Role::Border(a, b) => Sum::of(&[(a, 0.125), (v as u32, 0.75), (b, 0.125)]),
            Role::Corner => vec![(v as u32, 1.0)],
        };
        rows.push(row);
    }
    for f in 0..faces.len() {
        rows.push(face_point(f));
    }
    for (e, &[a, b]) in around.edges.iter().enumerate() {
        let row = match around.edge_faces[e].as_slice() {
            // A quarter of each end and of each face point.
            &[f, g] => {
                let mut sum = Sum::default();
                sum.add(a, 0.25);
                sum.add(b, 0.25);
                sum.add_all(&face_point(f as usize), 0.25);
                sum.add_all(&face_point(g as usize), 0.25);
                sum.done()
            }
            _ => Sum::of(&[(a, 0.5), (b, 0.5)]),
        };
        rows.push(row);
    }

    let edge_point = |a: u32, b: u32| (vertices + faces.len()) as u32 + around.edge(a, b);
    let mut refined = Vec::new();
    let mut parent = Vec::new();
    for (f, face) in faces.iter().enumerate() {
        let n = face.len();
        for k in 0..n {
            let (before, here, after) = (face[(k + n - 1) % n], face[k], face[(k + 1) % n]);
            refined.push(vec![here, edge_point(here, after), (vertices + f) as u32, edge_point(before, here)]);
            parent.push(f as u32);
        }
    }
    Refined { faces: refined, rows, parent }
}

/// Where each vertex of the mesh goes on the limit surface, as weights on
/// the vertices of the mesh.
pub fn limit(faces: &[Vec<u32>], vertices: usize) -> Vec<Stencil> {
    if faces.iter().all(|face| face.len() == 4) {
        return limit_of_quads(faces, vertices);
    }
    // The mask holds where quads alone meet: once refined, they do, and a
    // vertex keeps its limit through subdivision.
    let once = refine(faces, vertices);
    let finer = limit_of_quads(&once.faces, once.rows.len());
    compose(&finer[..vertices], &once.rows, vertices)
}

fn limit_of_quads(faces: &[Vec<u32>], vertices: usize) -> Vec<Stencil> {
    let around = Around::of(faces, vertices);
    (0..vertices)
        .map(|v| match around.role(v) {
            Role::Smooth => {
                let n = around.vertex_edges[v].len() as f64;
                let scale = 1.0 / (n * (n + 5.0));
                let mut sum = Sum::default();
                sum.add(v as u32, n * n * scale);
                for &e in &around.vertex_edges[v] {
                    sum.add(around.other_end(e, v), 4.0 * scale);
                }
                for &f in &around.vertex_faces[v] {
                    let face = &faces[f as usize];
                    let k = face.iter().position(|&c| c as usize == v).unwrap();
                    sum.add(face[(k + 2) % 4], scale);
                }
                sum.done()
            }
            Role::Border(a, b) => Sum::of(&[(a, 1.0 / 6.0), (v as u32, 4.0 / 6.0), (b, 1.0 / 6.0)]),
            Role::Corner => vec![(v as u32, 1.0)],
        })
        .collect()
}

/// The rows of `outer`, which weigh vertices that `inner` makes, as
/// weights on the `vertices` vertices `inner` weighs.
pub fn compose(outer: &[Stencil], inner: &[Stencil], vertices: usize) -> Vec<Stencil> {
    let mut weights = vec![0.0; vertices];
    let mut touched = Vec::new();
    outer
        .iter()
        .map(|row| {
            for &(middle, w) in row {
                for &(v, u) in &inner[middle as usize] {
                    if weights[v as usize] == 0.0 {
                        touched.push(v);
                    }
                    weights[v as usize] += w * u;
                }
            }
            touched.sort_unstable();
            let composed = touched.iter().map(|&v| (v, std::mem::take(&mut weights[v as usize]))).collect();
            touched.clear();
            composed
        })
        .collect()
}

/// Weights gathered vertex by vertex.
#[derive(Default)]
struct Sum(HashMap<u32, f64>);

impl Sum {
    fn of(terms: &[(u32, f64)]) -> Stencil {
        let mut sum = Sum::default();
        for &(v, w) in terms {
            sum.add(v, w);
        }
        sum.done()
    }

    fn add(&mut self, v: u32, w: f64) {
        *self.0.entry(v).or_insert(0.0) += w;
    }

    fn add_all(&mut self, stencil: &Stencil, scale: f64) {
        for &(v, w) in stencil {
            self.add(v, w * scale);
        }
    }

    fn done(self) -> Stencil {
        let mut row: Stencil = self.0.into_iter().collect();
        row.sort_unstable_by_key(|&(v, _)| v);
        row
    }
}

/// What the rules do with a vertex.
enum Role {
    /// Inside the surface, its edges and faces in one fan.
    Smooth,
    /// On a border, between these two neighbours along it.
    Border(u32, u32),
    /// Kept where it is: where more than two sharp edges meet, or two
    /// fans of faces, or none.
    Corner,
}

/// The edges of a mesh and what is around each vertex.
struct Around {
    edges: Vec<[u32; 2]>,
    edge_faces: Vec<Vec<u32>>,
    index: HashMap<(u32, u32), u32>,
    vertex_edges: Vec<Vec<u32>>,
    vertex_faces: Vec<Vec<u32>>,
}

impl Around {
    fn of(faces: &[Vec<u32>], vertices: usize) -> Self {
        let mut around = Around {
            edges: Vec::new(),
            edge_faces: Vec::new(),
            index: HashMap::new(),
            vertex_edges: vec![Vec::new(); vertices],
            vertex_faces: vec![Vec::new(); vertices],
        };
        for (f, face) in faces.iter().enumerate() {
            for k in 0..face.len() {
                let (a, b) = (face[k], face[(k + 1) % face.len()]);
                around.vertex_faces[a as usize].push(f as u32);
                let key = (a.min(b), a.max(b));
                let e = *around.index.entry(key).or_insert_with(|| {
                    around.edges.push([key.0, key.1]);
                    around.edge_faces.push(Vec::new());
                    around.vertex_edges[a as usize].push(around.edges.len() as u32 - 1);
                    around.vertex_edges[b as usize].push(around.edges.len() as u32 - 1);
                    around.edges.len() as u32 - 1
                });
                around.edge_faces[e as usize].push(f as u32);
            }
        }
        around
    }

    fn edge(&self, a: u32, b: u32) -> u32 {
        self.index[&(a.min(b), a.max(b))]
    }

    fn other_end(&self, e: u32, v: usize) -> u32 {
        let [a, b] = self.edges[e as usize];
        if a as usize == v { b } else { a }
    }

    fn role(&self, v: usize) -> Role {
        let sharp: Vec<u32> =
            self.vertex_edges[v].iter().copied().filter(|&e| self.edge_faces[e as usize].len() != 2).collect();
        match sharp.as_slice() {
            [] if !self.vertex_faces[v].is_empty() && self.vertex_edges[v].len() == self.vertex_faces[v].len() => {
                Role::Smooth
            }
            &[e, f] if self.vertex_edges[v].len() == self.vertex_faces[v].len() + 1 => {
                Role::Border(self.other_end(e, v), self.other_end(f, v))
            }
            _ => Role::Corner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::primitives;

    fn weight_of(row: &Stencil, v: u32) -> f64 {
        row.iter().find(|&&(u, _)| u == v).map_or(0.0, |&(_, w)| w)
    }

    /// Where `levels` subdivisions put the first `vertices` vertices: close
    /// to the limit after enough of them.
    fn refined_positions(faces: &[Vec<u32>], positions: &[Vec3], levels: usize) -> Vec<Vec3> {
        let (mut faces, mut positions) = (faces.to_vec(), positions.to_vec());
        for _ in 0..levels {
            let once = refine(&faces, positions.len());
            positions = once.rows.iter().map(|row| apply(row, &positions)).collect();
            faces = once.faces;
        }
        positions
    }

    fn assert_limit_is_where_subdivision_goes(faces: &[Vec<u32>], positions: &[Vec3], what: &str) {
        let stencils = limit(faces, positions.len());
        let far = refined_positions(faces, positions, 7);
        for v in 0..positions.len() {
            let row = &stencils[v];
            assert!((row.iter().map(|&(_, w)| w).sum::<f64>() - 1.0).abs() < 1e-12, "{what} {v}");
            assert!(row.iter().all(|&(_, w)| w >= 0.0), "{what} {v}");
            let gap = apply(row, positions).distance(far[v]);
            assert!(gap < 1e-4, "{what}: vertex {v} has its limit {gap} from where subdivision takes it");
        }
    }

    #[test]
    fn on_a_regular_grid_the_limit_is_the_bicubic_b_spline() {
        // A torus of quads: every vertex has valence 4 and no border.
        let torus = primitives::torus_quads(2.0, 0.5, 8, 6);
        let stencils = limit(&torus.faces, torus.positions.len());
        let around = Around::of(&torus.faces, torus.positions.len());
        for (v, row) in stencils.iter().enumerate() {
            assert_eq!(row.len(), 9);
            assert!((weight_of(row, v as u32) - 4.0 / 9.0).abs() < 1e-15);
            for &e in &around.vertex_edges[v] {
                assert!((weight_of(row, around.other_end(e, v)) - 1.0 / 9.0).abs() < 1e-15);
            }
            let diagonal = row.iter().filter(|&&(_, w)| (w - 1.0 / 36.0).abs() < 1e-15).count();
            assert_eq!(diagonal, 4);
        }
        assert_limit_is_where_subdivision_goes(&torus.faces, &torus.positions, "torus");
    }

    #[test]
    fn the_corners_of_a_cube_go_half_way_in() {
        // Valence 3: 9/24 of the corner, 4/24 of each neighbour, 1/24 of
        // each corner across a face. The cube from -1 to 1 has its corners
        // at (±1/2, ±1/2, ±1/2) on the limit surface.
        let cube = primitives::cube_quads(1);
        assert_eq!(cube.positions.len(), 8);
        let stencils = limit(&cube.faces, 8);
        for (v, row) in stencils.iter().enumerate() {
            assert_eq!(row.len(), 7);
            assert!((weight_of(row, v as u32) - 9.0 / 24.0).abs() < 1e-15);
            let corner = cube.positions[v];
            let half = corner.abs().max_element();
            let limit_point = apply(row, &cube.positions);
            assert!((limit_point - corner * 0.5).length() < 1e-12 * half, "{limit_point:?} for {corner:?}");
        }
        assert_limit_is_where_subdivision_goes(&cube.faces, &cube.positions, "cube");
        // Finer, with valences 3 and 4 and quads of two sizes.
        let finer = primitives::cube_quads(3);
        assert_limit_is_where_subdivision_goes(&finer.faces, &finer.positions, "cube of 54 quads");
    }

    #[test]
    fn a_border_is_a_cubic_b_spline_and_its_corners_are_smoothed() {
        let plane = primitives::plane_quads(4);
        assert_limit_is_where_subdivision_goes(&plane.faces, &plane.positions, "plane");
        let stencils = limit(&plane.faces, plane.positions.len());
        let around = Around::of(&plane.faces, plane.positions.len());
        for v in 0..plane.positions.len() {
            if let Role::Border(a, b) = around.role(v) {
                let mut expected = vec![(v as u32, 4.0 / 6.0), (a, 1.0 / 6.0), (b, 1.0 / 6.0)];
                expected.sort_unstable_by_key(|&(u, _)| u);
                assert_eq!(stencils[v], expected);
            }
        }
    }

    #[test]
    fn faces_other_than_quads_are_refined_first() {
        // A square pyramid: four triangles and a quad, and a pentagon
        // closing a prism.
        let pyramid = vec![vec![0, 1, 2, 3], vec![1, 0, 4], vec![2, 1, 4], vec![3, 2, 4], vec![0, 3, 4]];
        let positions = vec![
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.5),
        ];
        assert_limit_is_where_subdivision_goes(&pyramid, &positions, "pyramid");
        let ring: Vec<Vec3> = (0..5)
            .map(|k| {
                let angle = k as f64 * std::f64::consts::TAU / 5.0;
                Vec3::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect();
        let mut prism: Vec<Vec3> = ring.iter().map(|&p| p - Vec3::Z).collect();
        prism.extend(ring.iter().map(|&p| p + Vec3::Z));
        let mut faces = vec![vec![4, 3, 2, 1, 0], vec![5, 6, 7, 8, 9]];
        faces.extend((0..5u32).map(|k| vec![k, (k + 1) % 5, (k + 1) % 5 + 5, k + 5]));
        assert_limit_is_where_subdivision_goes(&faces, &prism, "prism");
    }

    #[test]
    fn a_vertex_keeps_its_limit_through_subdivision() {
        let cube = primitives::cube_quads(2);
        let once = refine(&cube.faces, cube.positions.len());
        let finer: Vec<Vec3> = once.rows.iter().map(|row| apply(row, &cube.positions)).collect();
        let (coarse, fine) = (limit(&cube.faces, cube.positions.len()), limit(&once.faces, finer.len()));
        for v in 0..cube.positions.len() {
            assert!(apply(&coarse[v], &cube.positions).distance(apply(&fine[v], &finer)) < 1e-12);
        }
        // Composed, the two steps are one map.
        let composed = compose(&fine, &once.rows, cube.positions.len());
        for (row, fine_row) in composed.iter().zip(&fine) {
            assert!(apply(row, &cube.positions).distance(apply(fine_row, &finer)) < 1e-12);
        }
        assert_eq!(once.parent.len(), once.faces.len());
        assert!(once.faces.iter().all(|face| face.len() == 4));
    }
}
