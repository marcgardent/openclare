//! Simplification by edge collapse under the quadric error metric
//! (Garland & Heckbert, "Surface Simplification Using Quadric Error Metrics",
//! 1997). Used to bring a very dense input down to a size the rest of the
//! preprocessing can afford, while staying close to the surface.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use glam::DMat3;

use super::dynamic_mesh::DynamicMesh;
use crate::math::Vec3;

/// Weight of the planes that pin feature edges in place, relative to the
/// ordinary face planes.
const FEATURE_WEIGHT: f64 = 1.0e3;

/// Sum of squared distances to a set of planes, as the quadratic form
/// `p^T A p + 2 b^T p + c`.
#[derive(Clone, Copy, Default)]
struct Quadric {
    /// Upper triangle of the symmetric `A`: xx, xy, xz, yy, yz, zz.
    a: [f64; 6],
    b: Vec3,
    c: f64,
}

impl Quadric {
    /// The plane through `point` with unit `normal`, weighted.
    fn plane(normal: Vec3, point: Vec3, weight: f64) -> Self {
        let d = -normal.dot(point);
        let n = normal;
        Self {
            a: [n.x * n.x, n.x * n.y, n.x * n.z, n.y * n.y, n.y * n.z, n.z * n.z].map(|v| v * weight),
            b: n * (d * weight),
            c: d * d * weight,
        }
    }

    fn add(&mut self, other: &Quadric) {
        for k in 0..6 {
            self.a[k] += other.a[k];
        }
        self.b += other.b;
        self.c += other.c;
    }

    fn matrix(&self) -> DMat3 {
        let [xx, xy, xz, yy, yz, zz] = self.a;
        DMat3::from_cols(Vec3::new(xx, xy, xz), Vec3::new(xy, yy, yz), Vec3::new(xz, yz, zz))
    }

    fn error(&self, p: Vec3) -> f64 {
        (p.dot(self.matrix() * p) + 2.0 * self.b.dot(p) + self.c).max(0.0)
    }

    /// The point of least error, when the planes pin one down.
    fn minimizer(&self) -> Option<Vec3> {
        let matrix = self.matrix();
        let scale = self.a[0] + self.a[3] + self.a[5];
        // Nearly coplanar or collinear planes leave the minimum undetermined.
        (matrix.determinant().abs() > 1e-9 * scale * scale * scale).then(|| matrix.inverse() * -self.b)
    }
}

struct Candidate {
    cost: f64,
    from: u32,
    to: u32,
    position: Vec3,
    /// Versions of both ends when the candidate was priced.
    stamps: (u32, u32),
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Candidate {}
impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Candidate {
    /// Reversed on cost so that the heap pops the cheapest; ties broken by
    /// index to keep the result deterministic.
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.total_cmp(&self.cost).then_with(|| (other.from, other.to).cmp(&(self.from, self.to)))
    }
}

/// Collapses edges, cheapest first, until `target_triangles` remain or no
/// collapse is possible any more.
pub(crate) fn decimate(mesh: &mut DynamicMesh, target_triangles: usize) {
    let mut quadrics = vertex_quadrics(mesh);
    let mut stamps = vec![0u32; mesh.vertex_count()];
    let mut heap = BinaryHeap::new();
    for (a, b) in mesh.edges() {
        heap.extend(price(mesh, &quadrics, &stamps, a, b));
    }

    while mesh.triangle_count() > target_triangles {
        let Some(candidate) = heap.pop() else { break };
        let Candidate { from, to, position, .. } = candidate;
        let current = (stamps[from as usize], stamps[to as usize]);
        if candidate.stamps != current || !mesh.has_edge(from, to) || !mesh.can_collapse(from, to, position) {
            continue;
        }

        mesh.collapse(from, to, position);
        let merged = quadrics[from as usize];
        quadrics[to as usize].add(&merged);
        stamps[to as usize] += 1;
        for neighbor in mesh.neighbors(to) {
            heap.extend(price(mesh, &quadrics, &stamps, to, neighbor));
        }
    }
}

fn vertex_quadrics(mesh: &DynamicMesh) -> Vec<Quadric> {
    let mut quadrics = vec![Quadric::default(); mesh.vertex_count()];
    for tri in mesh.triangles() {
        let corners = tri.map(|v| mesh.positions[v as usize]);
        let area_vector = 0.5 * (corners[1] - corners[0]).cross(corners[2] - corners[0]);
        let area = area_vector.length();
        let Some(normal) = area_vector.try_normalize() else { continue };
        let face = Quadric::plane(normal, corners[0], area);
        for v in tri {
            quadrics[v as usize].add(&face);
        }
        // A feature edge is held by a plane standing on it, square to the
        // face: moving off the line costs as much as leaving the surface.
        for k in 0..3 {
            let (a, b) = (tri[k], tri[(k + 1) % 3]);
            if mesh.is_feature(a, b)
                && let Some(wall) = (corners[(k + 1) % 3] - corners[k]).cross(normal).try_normalize()
            {
                let wall = Quadric::plane(wall, corners[k], FEATURE_WEIGHT * area);
                quadrics[a as usize].add(&wall);
                quadrics[b as usize].add(&wall);
            }
        }
    }
    quadrics
}

/// The collapse of edge `a-b` to consider, if any: which end goes, where the
/// survivor lands, and at what cost.
fn price(mesh: &DynamicMesh, quadrics: &[Quadric], stamps: &[u32], a: u32, b: u32) -> Option<Candidate> {
    let (pa, pb) = (mesh.positions[a as usize], mesh.positions[b as usize]);
    let mut sum = quadrics[a as usize];
    sum.add(&quadrics[b as usize]);

    let on_feature = |v: u32| mesh.feature_degree(v) > 0;
    let (from, to, position) = match (on_feature(a), on_feature(b)) {
        (false, false) => {
            let middle = 0.5 * (pa + pb);
            // The optimum is only trusted near the edge: far away it is the
            // intersection of nearly parallel planes.
            let reach = pa.distance_squared(pb);
            let best = sum
                .minimizer()
                .filter(|p| p.distance_squared(middle) <= reach)
                .into_iter()
                .chain([pa, pb, middle])
                .min_by(|p, q| sum.error(*p).total_cmp(&sum.error(*q)))?;
            (a, b, best)
        }
        // A vertex of a feature line does not leave it: the free end moves in.
        (true, false) => (b, a, pa),
        (false, true) => (a, b, pb),
        (true, true) if mesh.is_feature(a, b) => {
            if !mesh.is_corner(a) {
                (a, b, pb)
            } else {
                (b, a, pa)
            }
        }
        (true, true) => return None,
    };
    let stamps = (stamps[from as usize], stamps[to as usize]);
    Some(Candidate { cost: sum.error(position), from, to, position, stamps })
}
