//! Final vertex placement: smooth the extracted mesh while keeping it on the
//! source surface, and its hard edges on the source's hard edges.

use crate::field::Constraints;
use crate::math::Vec3;
use crate::mesh::PolyMesh;
use crate::spatial::Bvh;

/// Feature lines and corners of the source that the output must not drift
/// away from.
pub struct Features {
    lines: Bvh,
    corners: Bvh,
    /// Output vertices closer than this to a line (resp. corner) belong to it.
    snap_distance: f64,
}

impl Features {
    /// `edges` and `constraints` as produced for the field solvers; the
    /// corners are the constrained vertices without a direction.
    pub fn new(positions: &[Vec3], edges: &[[u32; 2]], constraints: &Constraints, snap_distance: f64) -> Option<Self> {
        if edges.is_empty() {
            return None;
        }
        let segments = edges.iter().map(|&[a, b]| {
            let (a, b) = (positions[a as usize], positions[b as usize]);
            [a, b, b]
        });
        let corners = (0..positions.len())
            .filter(|&v| constraints.point_weight[v] > 0.0 && constraints.direction_weight[v] == 0.0)
            .map(|v| [positions[v]; 3]);
        Some(Self {
            lines: Bvh::from_triangles(segments.collect()),
            corners: Bvh::from_triangles(corners.collect()),
            snap_distance,
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// Slides on the surface.
    Free,
    /// Slides along a feature line.
    Line,
    /// Does not move.
    Pinned,
}

/// Laplacian smoothing constrained to the source: free vertices are projected
/// back onto the surface after every pass, vertices on a feature line onto
/// that line, and corners stay put.
pub fn relax(mesh: &mut PolyMesh, surface: &Bvh, features: Option<&Features>, iterations: usize) {
    let n = mesh.positions.len();
    let mut neighbors = vec![Vec::new(); n];
    let mut on_border = vec![false; n];
    for (&(a, b), &count) in &mesh.edge_uses() {
        neighbors[a as usize].push(b);
        neighbors[b as usize].push(a);
        if count != 2 {
            on_border[a as usize] = true;
            on_border[b as usize] = true;
        }
    }

    let on_surface = |p: Vec3| surface.closest_point(p).map_or(p, |hit| hit.point);
    let on_line = |p: Vec3| features.and_then(|f| f.lines.closest_point(p)).map_or(p, |hit| hit.point);

    let mut kind = vec![Kind::Free; n];
    if let Some(features) = features {
        let limit = features.snap_distance * features.snap_distance;
        // Each corner captures the one vertex nearest to it.
        let mut captured: Vec<Option<(f64, usize)>> = Vec::new();
        for v in 0..n {
            let p = mesh.positions[v];
            if features.lines.closest_point(p).is_some_and(|hit| hit.distance_squared < limit) {
                kind[v] = Kind::Line;
                mesh.positions[v] = on_line(p);
            }
            if let Some(hit) = features.corners.closest_point(p).filter(|hit| hit.distance_squared < limit) {
                let corner = hit.triangle as usize;
                if captured.len() <= corner {
                    captured.resize(corner + 1, None);
                }
                if captured[corner].is_none_or(|(best, _)| hit.distance_squared < best) {
                    captured[corner] = Some((hit.distance_squared, v));
                }
            }
        }
        for (_, v) in captured.into_iter().flatten() {
            kind[v] = Kind::Pinned;
            mesh.positions[v] = features.corners.closest_point(mesh.positions[v]).unwrap().point;
        }
    }
    for v in 0..n {
        // A border that follows no feature is a hole of the extraction (or a
        // boundary we were told not to align with): leave its shape alone.
        if on_border[v] && kind[v] == Kind::Free {
            kind[v] = Kind::Pinned;
        }
    }

    for _ in 0..iterations {
        let smoothed: Vec<Vec3> = (0..n)
            .map(|v| {
                let p = mesh.positions[v];
                let mean_of = |keep: &dyn Fn(u32) -> bool| {
                    let (sum, count) = neighbors[v]
                        .iter()
                        .filter(|&&nb| keep(nb))
                        .fold((Vec3::ZERO, 0usize), |(s, c), &nb| (s + mesh.positions[nb as usize], c + 1));
                    (count > 0).then(|| sum / count as f64)
                };
                match kind[v] {
                    Kind::Pinned => p,
                    Kind::Free => mean_of(&|_| true).map_or(p, |mean| on_surface(0.5 * (p + mean))),
                    // Only its neighbours along the line pull it; the others
                    // would drag it across.
                    Kind::Line => {
                        let along = |nb: u32| kind[nb as usize] != Kind::Free;
                        let is_inner = neighbors[v].iter().filter(|&&nb| along(nb)).count() >= 2;
                        match mean_of(&along).filter(|_| is_inner) {
                            Some(mean) => on_line(0.5 * (p + mean)),
                            None => p,
                        }
                    }
                }
            })
            .collect();
        mesh.positions = smoothed;
    }

    for v in 0..n {
        if kind[v] == Kind::Free {
            mesh.positions[v] = on_surface(mesh.positions[v]);
        }
    }
}
