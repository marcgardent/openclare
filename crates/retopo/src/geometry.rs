//! Per-vertex differential quantities and feature detection.

use crate::math::{INVALID, Vec3};
use crate::mesh::topology::half_edge_vertices;
use crate::mesh::{Topology, TriMesh};

/// Angle-weighted vertex normals (independent of how the surface happens to
/// be triangulated, unlike area weighting).
pub fn vertex_normals(mesh: &TriMesh) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; mesh.positions.len()];
    for (f, tri) in mesh.triangles.iter().enumerate() {
        let corners = mesh.corners(f);
        let face_normal = mesh.area_vector(f).normalize_or_zero();
        for k in 0..3 {
            let e0 = (corners[(k + 1) % 3] - corners[k]).normalize_or_zero();
            let e1 = (corners[(k + 2) % 3] - corners[k]).normalize_or_zero();
            let angle = e0.dot(e1).clamp(-1.0, 1.0).acos();
            normals[tri[k] as usize] += face_normal * angle;
        }
    }
    for n in &mut normals {
        *n = n.normalize_or_zero();
    }
    normals
}

/// Barycentric dual areas: each vertex owns a third of its incident triangles.
pub fn vertex_areas(mesh: &TriMesh) -> Vec<f64> {
    let mut areas = vec![0.0; mesh.positions.len()];
    for (f, tri) in mesh.triangles.iter().enumerate() {
        let third = mesh.area_vector(f).length() / 3.0;
        for &v in tri {
            areas[v as usize] += third;
        }
    }
    areas
}

/// Edges the output must follow: open boundaries and, when `crease_angle`
/// (radians) is given, edges whose dihedral angle exceeds it.
pub fn feature_edges(
    mesh: &TriMesh,
    topology: &Topology,
    crease_angle: Option<f64>,
    boundaries: bool,
) -> Vec<[u32; 2]> {
    let cos_threshold = crease_angle.map(f64::cos);
    let mut edges = Vec::new();
    for h in 0..topology.opposite.len() {
        let twin = topology.opposite[h];
        let (a, b) = half_edge_vertices(mesh, h);
        if twin == INVALID {
            if boundaries {
                edges.push([a, b]);
            }
        } else if h < twin as usize
            && let Some(cos_threshold) = cos_threshold
        {
            let n0 = mesh.area_vector(h / 3).normalize_or_zero();
            let n1 = mesh.area_vector(twin as usize / 3).normalize_or_zero();
            if n0 != Vec3::ZERO && n1 != Vec3::ZERO && n0.dot(n1) < cos_threshold {
                edges.push([a, b]);
            }
        }
    }
    edges
}
