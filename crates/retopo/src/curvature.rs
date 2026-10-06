//! Principal curvatures of a triangle mesh.
//!
//! Follows Rusinkiewicz, "Estimating Curvatures and Their Derivatives on
//! Triangle Meshes" (2004): the second fundamental form is fitted per face
//! from how the vertex normals change along its edges, then averaged onto the
//! vertices. Tensors are kept as 3x3 matrices in world space, so averaging
//! them (onto vertices, or over a neighbourhood) needs no frame alignment.

use glam::{DMat3, DQuat};

use crate::math::{Vec3, tangent_frame};
use crate::mesh::TriMesh;

/// Second fundamental form per vertex, as a symmetric matrix acting on
/// tangent vectors. Convex regions are positive when normals point outward.
#[derive(Clone, Debug)]
pub struct TensorField {
    pub tensors: Vec<DMat3>,
}

/// Principal curvatures and directions per vertex, ordered so that
/// `|k1| >= |k2|`. `dir1`, `dir2` and the normal form a right-handed frame.
#[derive(Clone, Debug)]
pub struct Curvature {
    pub k1: Vec<f64>,
    pub k2: Vec<f64>,
    pub dir1: Vec<Vec3>,
    pub dir2: Vec<Vec3>,
}

impl Curvature {
    /// How strongly the surface prefers one direction: 0 where it is locally
    /// a sphere or a plane (principal directions are then meaningless).
    pub fn anisotropy(&self, v: usize) -> f64 {
        (self.k1[v] - self.k2[v]).abs()
    }
}

#[inline]
fn outer(a: Vec3, b: Vec3) -> DMat3 {
    DMat3::from_cols(a * b.x, a * b.y, a * b.z)
}

impl TensorField {
    /// `normals` and `areas` are per vertex (see [`crate::geometry`]).
    pub fn estimate(mesh: &TriMesh, normals: &[Vec3], areas: &[f64]) -> Self {
        let mut tensors = vec![DMat3::ZERO; mesh.positions.len()];
        for (f, tri) in mesh.triangles.iter().enumerate() {
            let p = mesh.corners(f);
            // Edge k is opposite corner k.
            let edges = [p[2] - p[1], p[0] - p[2], p[1] - p[0]];
            let face_normal = edges[0].cross(edges[1]);
            let (Some(t), Some(n)) = (edges[0].try_normalize(), face_normal.try_normalize()) else { continue };
            let b = n.cross(t);

            // Least squares for II = [[a, b], [b, c]] in the frame (t, b):
            // II * edge = change of normal along the edge, for the 3 edges.
            let mut lhs = DMat3::ZERO;
            let mut rhs = Vec3::ZERO;
            for k in 0..3 {
                let (u, v) = (edges[k].dot(t), edges[k].dot(b));
                let dn = normals[tri[(k + 2) % 3] as usize] - normals[tri[(k + 1) % 3] as usize];
                let (dnu, dnv) = (dn.dot(t), dn.dot(b));
                lhs += DMat3::from_cols(
                    Vec3::new(u * u, u * v, 0.0),
                    Vec3::new(u * v, u * u + v * v, u * v),
                    Vec3::new(0.0, u * v, v * v),
                );
                rhs += Vec3::new(dnu * u, dnu * v + dnv * u, dnv * v);
            }
            if lhs.determinant().abs() < 1e-300 {
                continue;
            }
            let fit = lhs.inverse() * rhs;
            let tensor = outer(t, t) * fit.x + (outer(t, b) + outer(b, t)) * fit.y + outer(b, b) * fit.z;

            let third = mesh.area_vector(f).length() / 3.0;
            for &v in tri {
                if areas[v as usize] > 0.0 {
                    tensors[v as usize] += tensor * (third / areas[v as usize]);
                }
            }
        }
        Self { tensors }
    }

    /// Area-weighted averaging with the one-ring, `iterations` times. Curvature
    /// only means something at a scale: `n` passes look at roughly `sqrt(n)`
    /// edge lengths around each vertex.
    ///
    /// A neighbour's tensor lives in the neighbour's tangent plane, so it is
    /// rotated into the plane of the vertex before being added; otherwise the
    /// bending of the surface itself would leak into the average.
    pub fn smoothed(&self, neighbors: &[Vec<u32>], normals: &[Vec3], areas: &[f64], iterations: usize) -> Self {
        // The rotations only depend on the normals: compute them once.
        let transports: Vec<Vec<DMat3>> = (0..self.tensors.len())
            .map(|v| {
                let rotate = |nb: &u32| DMat3::from_quat(DQuat::from_rotation_arc(normals[*nb as usize], normals[v]));
                let usable =
                    normals[v] != Vec3::ZERO && neighbors[v].iter().all(|&nb| normals[nb as usize] != Vec3::ZERO);
                if usable {
                    neighbors[v].iter().map(rotate).collect()
                } else {
                    vec![DMat3::IDENTITY; neighbors[v].len()]
                }
            })
            .collect();

        let mut tensors = self.tensors.clone();
        for _ in 0..iterations {
            let previous = tensors.clone();
            for v in 0..tensors.len() {
                let mut sum = previous[v] * areas[v];
                let mut weight = areas[v];
                for (&nb, rotation) in neighbors[v].iter().zip(&transports[v]) {
                    sum += *rotation * previous[nb as usize] * rotation.transpose() * areas[nb as usize];
                    weight += areas[nb as usize];
                }
                if weight > 0.0 {
                    tensors[v] = sum * (1.0 / weight);
                }
            }
        }
        Self { tensors }
    }

    /// Diagonalises each tensor in the tangent plane of its vertex.
    pub fn principal(&self, normals: &[Vec3]) -> Curvature {
        let n = self.tensors.len();
        let mut out =
            Curvature { k1: vec![0.0; n], k2: vec![0.0; n], dir1: vec![Vec3::ZERO; n], dir2: vec![Vec3::ZERO; n] };
        for v in 0..n {
            let normal = normals[v];
            if normal == Vec3::ZERO {
                continue;
            }
            let (s, t) = tangent_frame(normal);
            let tensor = self.tensors[v];
            let (ku, kuv, kv) = (s.dot(tensor * s), s.dot(tensor * t), t.dot(tensor * t));
            let angle = 0.5 * (2.0 * kuv).atan2(ku - kv);
            let (sin, cos) = angle.sin_cos();
            let dir = s * cos + t * sin;
            let ka = ku * cos * cos + 2.0 * kuv * sin * cos + kv * sin * sin;
            let kb = ku + kv - ka;
            let (k1, k2, dir1) = if ka.abs() >= kb.abs() { (ka, kb, dir) } else { (kb, ka, normal.cross(dir)) };
            out.k1[v] = k1;
            out.k2[v] = k2;
            out.dir1[v] = dir1;
            out.dir2[v] = normal.cross(dir1);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{vertex_areas, vertex_normals};
    use crate::mesh::primitives;

    fn curvature_of(mesh: &TriMesh) -> (Curvature, Vec<Vec3>) {
        let normals = vertex_normals(mesh);
        let field = TensorField::estimate(mesh, &normals, &vertex_areas(mesh));
        (field.principal(&normals), normals)
    }

    #[test]
    fn unit_sphere_has_curvature_one_everywhere() {
        let (curvature, _) = curvature_of(&primitives::icosphere(4));
        for v in 0..curvature.k1.len() {
            assert!((curvature.k1[v] - 1.0).abs() < 0.03, "k1 = {}", curvature.k1[v]);
            assert!((curvature.k2[v] - 1.0).abs() < 0.03, "k2 = {}", curvature.k2[v]);
        }
    }

    #[test]
    fn torus_matches_the_analytic_curvatures_and_directions() {
        let (major, minor) = (1.0, 0.4);
        let mesh = primitives::torus(major, minor, 128, 64);
        let (curvature, _) = curvature_of(&mesh);
        for (v, p) in mesh.positions.iter().enumerate() {
            // Around the tube the curvature is 1/minor; along the ring it is
            // cos(angle) / distance to the axis, smaller in magnitude.
            let radial = Vec3::new(p.x, p.y, 0.0).normalize();
            let ring_tangent = Vec3::Z.cross(radial);
            let cos = (radial.dot(*p) - major) / minor;
            let along_ring = cos / radial.dot(*p);

            assert!((curvature.k1[v] - 1.0 / minor).abs() < 0.05 / minor, "k1 = {}", curvature.k1[v]);
            assert!((curvature.k2[v] - along_ring).abs() < 0.05, "k2 = {} vs {along_ring}", curvature.k2[v]);
            assert!(curvature.dir1[v].dot(ring_tangent).abs() < 0.03, "dir1 not around the tube");
        }
    }

    #[test]
    fn smoothing_leaves_a_uniform_field_unchanged() {
        let mesh = primitives::icosphere(3);
        let normals = vertex_normals(&mesh);
        let areas = vertex_areas(&mesh);
        let neighbors = crate::mesh::Topology::build(&mesh).neighbors;
        let field = TensorField::estimate(&mesh, &normals, &areas).smoothed(&neighbors, &normals, &areas, 25);
        let curvature = field.principal(&normals);
        assert!(curvature.k1.iter().all(|k| (k - 1.0).abs() < 0.03));
        assert!(curvature.k2.iter().all(|k| (k - 1.0).abs() < 0.03));
    }
}
