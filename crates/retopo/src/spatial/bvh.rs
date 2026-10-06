use crate::math::Vec3;
use crate::mesh::TriMesh;

const LEAF_SIZE: usize = 4;

#[derive(Clone, Copy, Debug)]
struct Node {
    min: Vec3,
    max: Vec3,
    /// Leaf: first triangle slot. Inner: index of the left child, the right
    /// one follows it.
    first: u32,
    /// Triangle count for a leaf, 0 for an inner node.
    count: u32,
}

/// Result of a closest-point query.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub point: Vec3,
    pub triangle: u32,
    pub distance_squared: f64,
}

/// Bounding volume hierarchy over triangles, for projecting the output back
/// onto the source surface and its feature lines.
#[derive(Clone, Debug)]
pub struct Bvh {
    nodes: Vec<Node>,
    /// Triangle indices, grouped by leaf.
    order: Vec<u32>,
    corners: Vec<[Vec3; 3]>,
}

impl Bvh {
    pub fn build(mesh: &TriMesh) -> Self {
        Self::from_triangles((0..mesh.triangles.len()).map(|f| mesh.corners(f)).collect())
    }

    /// Builds over raw triangles. Degenerate ones are fine, which makes this
    /// usable for segments (`[a, b, b]`) and points (`[a, a, a]`) too.
    pub fn from_triangles(corners: Vec<[Vec3; 3]>) -> Self {
        let centroids: Vec<Vec3> = corners.iter().map(|c| (c[0] + c[1] + c[2]) / 3.0).collect();
        let empty = Node { min: Vec3::ZERO, max: Vec3::ZERO, first: 0, count: 0 };
        let mut bvh = Self { nodes: vec![empty], order: (0..corners.len() as u32).collect(), corners };
        bvh.build_node(0, 0, bvh.order.len(), &centroids);
        bvh
    }

    fn build_node(&mut self, node: usize, start: usize, end: usize, centroids: &[Vec3]) {
        let (mut min, mut max) = (Vec3::INFINITY, Vec3::NEG_INFINITY);
        let (mut centroid_min, mut centroid_max) = (Vec3::INFINITY, Vec3::NEG_INFINITY);
        for &t in &self.order[start..end] {
            for corner in self.corners[t as usize] {
                min = min.min(corner);
                max = max.max(corner);
            }
            centroid_min = centroid_min.min(centroids[t as usize]);
            centroid_max = centroid_max.max(centroids[t as usize]);
        }

        if end - start <= LEAF_SIZE {
            self.nodes[node] = Node { min, max, first: start as u32, count: (end - start) as u32 };
            return;
        }

        // Median split along the widest axis of the centroids.
        let axis = (centroid_max - centroid_min).max_position();
        let middle = (start + end) / 2;
        self.order[start..end].select_nth_unstable_by(middle - start, |&a, &b| {
            centroids[a as usize][axis].total_cmp(&centroids[b as usize][axis])
        });

        let left = self.nodes.len();
        self.nodes[node] = Node { min, max, first: left as u32, count: 0 };
        self.nodes.push(self.nodes[node]);
        self.nodes.push(self.nodes[node]);
        self.build_node(left, start, middle, centroids);
        self.build_node(left + 1, middle, end, centroids);
    }

    /// Closest point of the surface to `p`; `None` for an empty mesh.
    pub fn closest_point(&self, p: Vec3) -> Option<Hit> {
        self.closest_point_where(p, f64::INFINITY, |_| true)
    }

    /// Closest point to `p` among the triangles `accept` lets through, no
    /// farther than `max_distance`. Lets a query stay on one sheet where
    /// several pass close to each other.
    pub fn closest_point_where(&self, p: Vec3, max_distance: f64, accept: impl Fn(u32) -> bool) -> Option<Hit> {
        if self.order.is_empty() {
            return None;
        }
        let mut best: Option<Hit> = None;
        let mut best_distance = max_distance * max_distance;
        let mut stack = vec![(0u32, self.nodes[0].distance_squared(p))];
        while let Some((index, bound)) = stack.pop() {
            if bound >= best_distance {
                continue;
            }
            let node = self.nodes[index as usize];
            if node.count > 0 {
                for &t in &self.order[node.first as usize..(node.first + node.count) as usize] {
                    let [a, b, c] = self.corners[t as usize];
                    let point = closest_point_on_triangle(p, a, b, c);
                    let distance_squared = point.distance_squared(p);
                    if distance_squared < best_distance && accept(t) {
                        best_distance = distance_squared;
                        best = Some(Hit { point, triangle: t, distance_squared });
                    }
                }
            } else {
                let (left, right) = (node.first, node.first + 1);
                let (dl, dr) =
                    (self.nodes[left as usize].distance_squared(p), self.nodes[right as usize].distance_squared(p));
                // Nearer child on top of the stack so it tightens the bound first.
                if dl < dr {
                    stack.push((right, dr));
                    stack.push((left, dl));
                } else {
                    stack.push((left, dl));
                    stack.push((right, dr));
                }
            }
        }
        best
    }
}

impl Node {
    #[inline]
    fn distance_squared(&self, p: Vec3) -> f64 {
        p.distance_squared(p.clamp(self.min, self.max))
    }
}

/// Closest point to `p` on triangle `abc` (Ericson, *Real-Time Collision
/// Detection*, 5.1.5): classify `p` against the Voronoi regions of the
/// vertices, edges and interior.
pub fn closest_point_on_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let (ab, ac, ap) = (b - a, c - a, p - a);
    let (d1, d2) = (ab.dot(ap), ac.dot(ap));
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = p - b;
    let (d3, d4) = (ab.dot(bp), ac.dot(bp));
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return a + ab * (d1 / (d1 - d3));
    }
    let cp = p - c;
    let (d5, d6) = (ab.dot(cp), ac.dot(cp));
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return a + ac * (d2 / (d2 - d6));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        return b + (c - b) * ((d4 - d3) / ((d4 - d3) + (d5 - d6)));
    }
    let sum = va + vb + vc;
    if sum == 0.0 {
        return a;
    }
    a + ab * (vb / sum) + ac * (vc / sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Rng;
    use crate::mesh::primitives;

    #[test]
    fn matches_brute_force() {
        let mesh = primitives::torus(1.0, 0.4, 24, 12);
        let bvh = Bvh::build(&mesh);
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let p = Vec3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 4.0 - 2.0;
            let brute = (0..mesh.triangles.len())
                .map(|f| {
                    let [a, b, c] = mesh.corners(f);
                    closest_point_on_triangle(p, a, b, c).distance_squared(p)
                })
                .fold(f64::INFINITY, f64::min);
            let hit = bvh.closest_point(p).unwrap();
            assert!((hit.distance_squared - brute).abs() < 1e-12);
        }
    }

    #[test]
    fn filtered_query_stays_on_the_accepted_triangles() {
        let mesh = primitives::cube(4);
        let bvh = Bvh::build(&mesh);
        let facing_up = |t: u32| mesh.area_vector(t as usize).z > 0.0;
        // Just outside the +x face: the nearest upward triangle is on top.
        let p = Vec3::new(1.05, 0.0, 0.5);
        assert!((bvh.closest_point(p).unwrap().point.x - 1.0).abs() < 1e-12);
        let hit = bvh.closest_point_where(p, f64::INFINITY, facing_up).unwrap();
        assert!((hit.point.z - 1.0).abs() < 1e-12);
        assert!(bvh.closest_point_where(p, 0.2, facing_up).is_none());
    }

    #[test]
    fn degenerate_triangles_act_as_segments_and_points() {
        let (a, b) = (Vec3::ZERO, Vec3::X);
        let p = Vec3::new(0.25, 1.0, 0.0);
        assert_eq!(closest_point_on_triangle(p, a, b, b), Vec3::new(0.25, 0.0, 0.0));
        assert_eq!(closest_point_on_triangle(Vec3::new(3.0, 1.0, 0.0), a, b, b), b);
        assert_eq!(closest_point_on_triangle(p, b, b, b), b);
    }
}
