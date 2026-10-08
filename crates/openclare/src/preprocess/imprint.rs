//! Lays guide curves into a mesh as lines of edges.
//!
//! A guide curve is a polyline drawn on the surface, along which the quad
//! mesh should have a line of edges. The rest of the chain already knows how
//! to follow lines of edges of the work mesh: creases and borders. So a
//! curve is cut into the mesh: every edge it crosses is split where it
//! crosses, the pieces of the curve between two crossings become edges, and
//! those are marked to be kept like a crease.
//!
//! The curve is taken a short stretch at a time. Each stretch stands in a
//! plane, that of the stretch and of the direction the surface faces there;
//! the edges of the mesh that go through that plane, between the two ends
//! of the stretch and close to it, are the ones it crosses.

use std::collections::{HashMap, HashSet};

use super::dynamic_mesh::DynamicMesh;
use crate::math::Vec3;
use crate::spatial::Bvh;

/// A polyline on the surface for the quad mesh to have edges along.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuideCurve {
    pub points: Vec<Vec3>,
    /// Whether the last point joins the first.
    pub closed: bool,
}

/// What laying the curves did.
#[derive(Clone, Debug, Default)]
pub(crate) struct Laid {
    /// The stretches of the curves, on the surface: what the vertices of
    /// the lines are held on from here on.
    pub segments: Vec<[Vec3; 2]>,
    /// Places where two points of a curve in a row ended up with no edge
    /// between them: the line is broken there.
    pub gaps: usize,
}

/// A crossing nearer than this to an end of the edge, as a share of its
/// length, uses the vertex there instead of making a new one next to it.
const SNAP: f64 = 0.2;

/// Cuts `curves` into `mesh`, whose shape is `surface` (with `face_normals`
/// for its triangles), and marks them as lines to keep.
pub(crate) fn imprint(mesh: &mut DynamicMesh, surface: &Bvh, face_normals: &[Vec3], curves: &[GuideCurve]) -> Laid {
    let mut laid = Laid::default();
    if curves.iter().all(|curve| curve.points.len() < 2) {
        return laid;
    }
    let edges = mesh.edges();
    let length_of =
        |mesh: &DynamicMesh, a: u32, b: u32| mesh.positions[a as usize].distance(mesh.positions[b as usize]);
    let spacing = edges.iter().map(|&(a, b)| length_of(mesh, a, b)).sum::<f64>() / edges.len().max(1) as f64;
    if !(spacing.is_finite() && spacing > 0.0) {
        return laid;
    }

    // The vertices by where they are, to find those near a stretch.
    let cell_of = |p: Vec3| -> [i64; 3] { (p / (2.0 * spacing)).floor().to_array().map(|c| c as i64) };
    let mut cells: HashMap<[i64; 3], Vec<u32>> = HashMap::new();
    for v in (0..mesh.vertex_count() as u32).filter(|&v| mesh.is_live_vertex(v)) {
        cells.entry(cell_of(mesh.positions[v as usize])).or_default().push(v);
    }

    let mut touched: Vec<u32> = Vec::new();
    for curve in curves.iter().filter(|curve| curve.points.len() >= 2) {
        // Stretches about as long as an edge, each end back on the surface.
        let mut ends: Vec<(Vec3, Vec3)> = Vec::new();
        let count = curve.points.len() + curve.closed as usize;
        for k in 0..count - 1 {
            let (from, to) = (curve.points[k], curve.points[(k + 1) % curve.points.len()]);
            let pieces = (from.distance(to) / spacing).ceil().max(1.0) as usize;
            for step in 0..pieces {
                let point = from.lerp(to, step as f64 / pieces as f64);
                if let Some(hit) = surface.closest_point(point) {
                    ends.push((hit.point, face_normals[hit.triangle as usize]));
                }
            }
        }
        if !curve.closed
            && let Some(hit) = surface.closest_point(*curve.points.last().expect("at least two points"))
        {
            ends.push((hit.point, face_normals[hit.triangle as usize]));
        }
        if curve.closed
            && let Some(&first) = ends.first()
        {
            ends.push(first);
        }

        // The vertices the curve goes through, in order.
        let mut path: Vec<u32> = Vec::new();
        for pair in ends.windows(2) {
            let ((from, facing_from), (to, facing_to)) = (pair[0], pair[1]);
            let (along, length) = ((to - from).normalize_or_zero(), from.distance(to));
            let facing = (facing_from + facing_to).normalize_or_zero();
            let across = along.cross(facing).normalize_or_zero();
            if length < 1e-9 * spacing || across.length_squared() < 0.5 {
                continue;
            }
            laid.segments.push([from, to]);

            // Edges with an end near the stretch.
            let reach = 0.5 * length + 2.0 * spacing;
            let (low, high) = (cell_of(from.min(to) - Vec3::splat(reach)), cell_of(from.max(to) + Vec3::splat(reach)));
            let mut near: HashSet<(u32, u32)> = HashSet::new();
            for x in low[0]..=high[0] {
                for y in low[1]..=high[1] {
                    for z in low[2]..=high[2] {
                        for &v in cells.get(&[x, y, z]).map_or(&[][..], Vec::as_slice) {
                            if mesh.is_live_vertex(v) {
                                near.extend(mesh.neighbors(v).into_iter().map(|w| (v.min(w), v.max(w))));
                            }
                        }
                    }
                }
            }
            // Where each goes through the plane of the stretch, if it does
            // between its two ends and on this sheet of the surface.
            let side = |p: Vec3| (p - from).dot(across);
            let mut crossings: Vec<(f64, u32, u32, f64)> = Vec::new();
            for (a, b) in near {
                let (p, q) = (mesh.positions[a as usize], mesh.positions[b as usize]);
                let (sp, sq) = (side(p), side(q));
                if (sp > 0.0) == (sq > 0.0) {
                    continue;
                }
                let t = sp / (sp - sq);
                let point = p.lerp(q, t);
                let (run, off) = ((point - from).dot(along), (point - from).dot(facing));
                let faces_alike = mesh.normal(a).dot(facing) > 0.2 && mesh.normal(b).dot(facing) > 0.2;
                if (-1e-9 * length..length).contains(&run) && off.abs() < spacing && faces_alike {
                    crossings.push((run, a, b, t));
                }
            }
            crossings.sort_by(|x, y| x.0.total_cmp(&y.0).then((x.1, x.2).cmp(&(y.1, y.2))));

            for (_, a, b, t) in crossings {
                if !mesh.has_edge(a, b) {
                    continue;
                }
                let point = mesh.positions[a as usize].lerp(mesh.positions[b as usize], t);
                // Close to a vertex, the curve goes through it rather than
                // next to it, if it can be brought onto the curve: one that
                // is held, or that would fold a triangle, is left alone and
                // the edge is split after all.
                let close = if t < SNAP { Some(a) } else { Some(b).filter(|_| t > 1.0 - SNAP) };
                let moved =
                    close.filter(|&v| mesh.feature_degree(v) == 0 && mesh.move_keeps_orientation(v, point, u32::MAX));
                let vertex = match moved {
                    Some(v) => {
                        mesh.positions[v as usize] = point;
                        v
                    }
                    None => {
                        let v = mesh.split(a, b, point);
                        cells.entry(cell_of(point)).or_default().push(v);
                        v
                    }
                };
                if path.last() != Some(&vertex) {
                    path.push(vertex);
                }
            }
        }
        if curve.closed && path.len() > 2 && path.first() == path.last() {
            path.pop();
        }

        let links = path.len() - (!curve.closed || path.len() < 3) as usize;
        for k in 0..links.min(path.len()) {
            let (a, b) = (path[k], path[(k + 1) % path.len()]);
            if a == b {
                continue;
            }
            if mesh.has_edge(a, b) {
                mesh.mark_feature(a, b);
            } else {
                laid.gaps += 1;
            }
        }
        touched.extend(path);
    }
    // Where curves end, meet or cross a crease, a corner. Along a curve,
    // none: two crossings a hair apart can make it look as if it turned
    // sharply there, and a vertex held for that would keep the kink.
    for v in touched {
        mesh.refresh_corner(v);
        mesh.let_slide(v);
    }
    laid
}
