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

use std::collections::HashMap;

use super::dynamic_mesh::DynamicMesh;
use crate::math::Vec3;
use crate::spatial::Bvh;

/// A polyline on the surface for the quad mesh to have edges along.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuideCurve {
    pub points: Vec<Vec3>,
    /// Whether the last point joins the first.
    pub closed: bool,
    /// Whether the edges along it are hard edges of the result, as if the
    /// surface had a crease there, and not only a line of edges.
    pub sharp: bool,
}

/// What laying the curves did.
#[derive(Clone, Debug, Default)]
pub(crate) struct Laid {
    /// The stretches of the curves, on the surface: what the vertices of
    /// the lines are held on from here on.
    pub segments: Vec<[Vec3; 2]>,
    /// For each stretch, whether it belongs to a curve drawn as a hard edge.
    pub sharp: Vec<bool>,
    /// Places where two points of a curve in a row ended up with no edge
    /// between them: the line is broken there.
    pub gaps: usize,
}

/// The shortest way from `from` to `to` along edges of `mesh`, through
/// vertices no farther than `reach` from either and that are not among
/// `taken` (the line must not come back on itself); `None` if there is none.
fn bridge(mesh: &DynamicMesh, from: u32, to: u32, reach: f64, taken: &[u32]) -> Option<Vec<u32>> {
    let (start, goal) = (mesh.positions[from as usize], mesh.positions[to as usize]);
    if start.distance(goal) > reach {
        return None;
    }
    let mut best: HashMap<u32, (f64, u32)> = HashMap::from([(from, (0.0, from))]);
    let mut open = vec![from];
    while !open.is_empty() {
        // Few vertices are in reach: the nearest is found by looking.
        let (at, &v) = open.iter().enumerate().min_by(|a, b| best[a.1].0.total_cmp(&best[b.1].0))?;
        open.swap_remove(at);
        if v == to {
            let mut way = vec![to];
            while *way.last()? != from {
                way.push(best[way.last()?].1);
            }
            way.reverse();
            return Some(way);
        }
        let so_far = best[&v].0;
        for next in mesh.neighbors(v) {
            let p = mesh.positions[next as usize];
            let length = so_far + mesh.positions[v as usize].distance(p);
            let within = p.distance(start) <= reach && p.distance(goal) <= reach;
            let within = within && (next == to || !taken.contains(&next));
            if within && best.get(&next).is_none_or(|&(known, _)| length < known) {
                best.insert(next, (length, v));
                if !open.contains(&next) {
                    open.push(next);
                }
            }
        }
    }
    None
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

        // Which side of the curve each vertex near it is on, told from the
        // stretch nearest to it: how far to that side, and how far along the
        // curve, in stretches. Told stretch by stretch instead, with a plane
        // each, the edges that pass where two stretches meet at an angle
        // would be seen twice or not at all.
        let stretches: Vec<(Vec3, Vec3, Vec3, Vec3, f64)> = ends
            .windows(2)
            .filter_map(|pair| {
                let ((from, facing_from), (to, facing_to)) = (pair[0], pair[1]);
                let (along, length) = ((to - from).normalize_or_zero(), from.distance(to));
                let facing = (facing_from + facing_to).normalize_or_zero();
                let across = along.cross(facing).normalize_or_zero();
                (length > 1e-9 * spacing && across.length_squared() > 0.5)
                    .then_some((from, along, across, facing, length))
            })
            .collect();
        if stretches.is_empty() {
            continue;
        }
        laid.segments.extend(stretches.iter().map(|&(from, along, _, _, length)| [from, from + along * length]));
        laid.sharp.resize(laid.segments.len(), curve.sharp);

        let reach = 2.0 * spacing;
        let mut place: HashMap<u32, (f64, f64, f64)> = HashMap::new();
        for (index, &(from, along, across, facing, length)) in stretches.iter().enumerate() {
            let to = from + along * length;
            let (low, high) = (cell_of(from.min(to) - Vec3::splat(reach)), cell_of(from.max(to) + Vec3::splat(reach)));
            for x in low[0]..=high[0] {
                for y in low[1]..=high[1] {
                    for z in low[2]..=high[2] {
                        for &v in cells.get(&[x, y, z]).map_or(&[][..], Vec::as_slice) {
                            if !mesh.is_live_vertex(v) || mesh.normal(v).dot(facing) < 0.2 {
                                continue;
                            }
                            let offset = mesh.positions[v as usize] - from;
                            let run = offset.dot(along).clamp(0.0, length);
                            let away = (offset - along * run).length();
                            if away < reach && place.get(&v).is_none_or(|&(known, ..)| away < known) {
                                place.insert(v, (away, offset.dot(across), index as f64 + run / length));
                            }
                        }
                    }
                }
            }
        }
        // An edge crosses the curve where its two ends are on either side,
        // about as far along it; past the ends of an open curve nothing does.
        let last = stretches.len() as f64;
        let mut crossings: Vec<(f64, u32, u32, f64)> = Vec::new();
        for (&a, &(_, side_a, along_a)) in &place {
            for b in mesh.neighbors(a) {
                let Some(&(_, side_b, along_b)) = place.get(&b) else { continue };
                if a > b || (side_a > 0.0) == (side_b > 0.0) {
                    continue;
                }
                let mut apart = (along_a - along_b).abs();
                if curve.closed {
                    apart = apart.min(last - apart);
                }
                let t = side_a / (side_a - side_b);
                let mut at = along_a + t * (along_b - along_a);
                if curve.closed && (along_a - along_b).abs() > 0.5 * last {
                    // The two ends are on either side of where the curve closes.
                    let (first, second) =
                        if along_a < along_b { (along_a + last, along_b) } else { (along_a, along_b + last) };
                    at = (first + t * (second - first)).rem_euclid(last);
                }
                let beyond = !curve.closed && (at <= 0.0 || at >= last);
                if apart < 3.0 && !beyond {
                    crossings.push((at, a, b, t));
                }
            }
        }
        crossings.sort_by(|x, y| x.0.total_cmp(&y.0).then((x.1, x.2).cmp(&(y.1, y.2))));

        // The vertices the curve goes through, in order.
        let mut path: Vec<u32> = Vec::new();
        for (_, a, b, t) in crossings {
            if !mesh.has_edge(a, b) {
                continue;
            }
            let point = mesh.positions[a as usize].lerp(mesh.positions[b as usize], t);
            // Close to a vertex, the curve goes through it rather than next
            // to it, if it can be brought onto the curve: one that is held,
            // or that would fold a triangle, is left alone and the edge is
            // split after all.
            let close = if t < SNAP { Some(a) } else { Some(b).filter(|_| t > 1.0 - SNAP) };
            let moved = close.filter(|&v| {
                !path.contains(&v) && mesh.feature_degree(v) == 0 && mesh.move_keeps_orientation(v, point, u32::MAX)
            });
            let vertex = match (moved, close) {
                (Some(v), _) => {
                    mesh.positions[v as usize] = point;
                    v
                }
                // Already on the line a little earlier: the same vertex.
                (None, Some(v)) if path.last() == Some(&v) => v,
                _ => {
                    let v = mesh.split(a, b, point);
                    cells.entry(cell_of(point)).or_default().push(v);
                    v
                }
            };
            if path.last() != Some(&vertex) {
                path.push(vertex);
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
                continue;
            }
            // A crossing was missed between the two (the surface turns
            // sharply there, or the stretches met at an odd angle): the line
            // goes on along the edges that are there, the shortest way.
            match bridge(mesh, a, b, 3.0 * spacing, &path) {
                Some(way) => {
                    for pair in way.windows(2) {
                        mesh.mark_feature(pair[0], pair[1]);
                    }
                    touched.extend(way);
                }
                None => laid.gaps += 1,
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

/// Does away with the triangles that a line runs round: two edges of the
/// line in a row and, closing them into a triangle, the chord between their
/// far ends. Such a triangle is bound to be flat once the line is straight
/// in the map, and the chord sends astray whatever is traced along the line.
/// The chord is turned to join the middle of the two edges to the vertex
/// that faced it, wherever that keeps the surface as it is.
pub(crate) fn unpinch(mesh: &mut DynamicMesh) {
    for _ in 0..4 {
        let mut turned = 0;
        for v in 0..mesh.vertex_count() as u32 {
            if !mesh.is_live_vertex(v) {
                continue;
            }
            let [a, b] = mesh.feature_neighbors(v)[..] else { continue };
            if mesh.has_edge(a, b) && !mesh.is_feature(a, b) && mesh.can_flip(a, b).is_some() {
                mesh.flip(a, b);
                turned += 1;
            }
        }
        if turned == 0 {
            break;
        }
    }
}
