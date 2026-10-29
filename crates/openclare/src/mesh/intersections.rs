//! Triangles of a mesh that cut through one another: what a printer cannot
//! make sense of, told before the mesh goes to it.
//!
//! The triangles are put in the cells of a grid their boxes overlap, a cell
//! about as wide as a triangle; two triangles of the same cell that share no
//! vertex cross when an edge of one goes through the other. Two that only
//! touch, or lie in one plane, are not counted.

use std::collections::HashMap;

use super::TriMesh;
use crate::math::Vec3;

/// How many triangles of `mesh` cut through another one of it.
pub fn crossing_triangles(mesh: &TriMesh) -> usize {
    let corners = |t: usize| mesh.triangles[t].map(|v| mesh.positions[v as usize]);
    let count = mesh.triangles.len();
    if count < 2 {
        return 0;
    }
    let mean_edge = mesh
        .triangles
        .iter()
        .enumerate()
        .map(|(t, _)| {
            let [a, b, c] = corners(t);
            (a.distance(b) + b.distance(c) + c.distance(a)) / 3.0
        })
        .sum::<f64>()
        / count as f64;
    let cell = mean_edge.max(f64::MIN_POSITIVE);
    let key = |p: Vec3| [(p.x / cell).floor() as i64, (p.y / cell).floor() as i64, (p.z / cell).floor() as i64];

    let mut grid: HashMap<[i64; 3], Vec<u32>> = HashMap::new();
    for t in 0..count {
        let [a, b, c] = corners(t);
        let (low, high) = (key(a.min(b).min(c)), key(a.max(b).max(c)));
        for x in low[0]..=high[0] {
            for y in low[1]..=high[1] {
                for z in low[2]..=high[2] {
                    grid.entry([x, y, z]).or_default().push(t as u32);
                }
            }
        }
    }

    let mut crossing = vec![false; count];
    let mut tried = std::collections::HashSet::new();
    for members in grid.values() {
        for (i, &s) in members.iter().enumerate() {
            for &t in &members[i + 1..] {
                let (s, t) = (s.min(t), s.max(t));
                if mesh.triangles[s as usize].iter().any(|v| mesh.triangles[t as usize].contains(v))
                    || !tried.insert((s, t))
                {
                    continue;
                }
                if cross(corners(s as usize), corners(t as usize)) {
                    crossing[s as usize] = true;
                    crossing[t as usize] = true;
                }
            }
        }
    }
    crossing.iter().filter(|&&c| c).count()
}

/// Whether an edge of either triangle goes through the inside of the other.
fn cross(p: [Vec3; 3], q: [Vec3; 3]) -> bool {
    (0..3).any(|k| through(p[k], p[(k + 1) % 3], q)) || (0..3).any(|k| through(q[k], q[(k + 1) % 3], p))
}

/// Whether the segment from `a` to `b` goes through the triangle `t`,
/// strictly inside both (Möller and Trumbore).
fn through(a: Vec3, b: Vec3, [p, q, r]: [Vec3; 3]) -> bool {
    const SLACK: f64 = 1e-9;
    let direction = b - a;
    let (e1, e2) = (q - p, r - p);
    let h = direction.cross(e2);
    let det = e1.dot(h);
    if det.abs() < SLACK * e1.length() * e2.length() * direction.length() {
        return false;
    }
    let s = a - p;
    let u = s.dot(h) / det;
    let w = s.cross(e1);
    let v = direction.dot(w) / det;
    let along = e2.dot(w) / det;
    u > SLACK && v > SLACK && u + v < 1.0 - SLACK && along > SLACK && along < 1.0 - SLACK
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::primitives;

    #[test]
    fn a_closed_sphere_has_none_and_two_crossed_triangles_are_found() {
        assert_eq!(crossing_triangles(&primitives::icosphere(3)), 0);
        // Two triangles through each other, and a third apart.
        let mesh = TriMesh {
            positions: vec![
                Vec3::new(-1.0, -1.0, 0.0),
                Vec3::new(1.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.5, 0.5, 0.0),
                Vec3::new(5.0, 0.0, 0.0),
                Vec3::new(6.0, 0.0, 0.0),
                Vec3::new(5.0, 1.0, 0.0),
            ],
            triangles: vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]],
        };
        assert_eq!(crossing_triangles(&mesh), 2);
    }
}
