//! Isotropic remeshing (Botsch & Kobbelt, "A Remeshing Approach to
//! Multiresolution Modeling", 2004): repeat "split the long edges, collapse
//! the short ones, flip towards valence 6, relax, project back" until the
//! triangles are all about the same size and close to equilateral.
//!
//! The solvers downstream discretise differential operators on this mesh;
//! slivers and wild size variations are what make them inaccurate. The few
//! slivers the loop leaves (a vertex pulled onto the edge between two
//! others) are then removed one by one.

use rayon::prelude::*;

use super::dynamic_mesh::DynamicMesh;
use super::{Refinement, SLIVER_ANGLE};
use crate::math::Vec3;
use crate::spatial::Bvh;

/// What the remeshed surface must stay on.
pub(crate) struct Target<'a> {
    /// The source surface.
    pub surface: &'a Bvh,
    /// Face normals of the source, to keep projections on the right sheet.
    pub face_normals: &'a [Vec3],
    /// The source feature lines, as a [`Bvh`] of segments; `None` if it has none.
    pub features: Option<&'a Bvh>,
}

/// Remeshes towards edges of length `edge_length`, with no more than `most`
/// triangles: a length that calls for more is not one the surface can be
/// tiled with (a sliver with next to no area and a long outline asks for
/// edges of next to no length, and splitting towards them never ends).
/// Where `refinement` asks for it, the edges are shorter than that.
pub(crate) fn remesh(
    mesh: &mut DynamicMesh,
    target: &Target,
    edge_length: f64,
    iterations: usize,
    most: usize,
    refinement: Option<&Refinement>,
) {
    for _ in 0..iterations {
        // The edge asked for at each vertex, read again as they have moved;
        // a vertex a split makes takes what its edge asked for.
        let mut lengths: Vec<f64> = match refinement {
            Some(refinement) => (0..mesh.vertex_count())
                .into_par_iter()
                .map(|v| edge_length * refinement.factor(mesh.positions[v]))
                .collect(),
            None => vec![edge_length; mesh.vertex_count()],
        };
        split_long_edges(mesh, &mut lengths, most);
        collapse_short_edges(mesh, &lengths);
        equalize_valences(mesh);
        relax(mesh, target, edge_length, refinement.is_some().then_some(&lengths[..]));
        remove_slivers(mesh);
    }
}

/// Splits the edges much longer than `edge_length`, nothing else: a coarse
/// mesh made fine enough for a curve to be laid into it.
pub(crate) fn split_to(mesh: &mut DynamicMesh, edge_length: f64, most: usize) {
    let mut lengths = vec![edge_length; mesh.vertex_count()];
    split_long_edges(mesh, &mut lengths, most);
}

fn length(mesh: &DynamicMesh, a: u32, b: u32) -> f64 {
    mesh.positions[a as usize].distance(mesh.positions[b as usize])
}

/// The edge asked for between `a` and `b`: the shorter of what is asked at
/// its ends, so that the edges round a place made finer are not collapsed
/// into the coarser ones, which would wear it away from its rim.
fn asked(lengths: &[f64], a: u32, b: u32) -> f64 {
    lengths[a as usize].min(lengths[b as usize])
}

fn split_long_edges(mesh: &mut DynamicMesh, lengths: &mut Vec<f64>, most: usize) {
    let high = |lengths: &[f64], a: u32, b: u32| 4.0 / 3.0 * asked(lengths, a, b);
    // A split halves an edge but creates two more across the neighbouring
    // triangles, which may be long too: go on until none is left.
    for _ in 0..16 {
        let long: Vec<(u32, u32)> =
            mesh.edges().into_iter().filter(|&(a, b)| length(mesh, a, b) > high(lengths, a, b)).collect();
        if long.is_empty() {
            break;
        }
        for (a, b) in long {
            if mesh.triangle_count() >= most {
                return;
            }
            if mesh.has_edge(a, b) && length(mesh, a, b) > high(lengths, a, b) {
                let middle = 0.5 * (mesh.positions[a as usize] + mesh.positions[b as usize]);
                let between = asked(lengths, a, b);
                let new = mesh.split(a, b, middle) as usize;
                if lengths.len() <= new {
                    lengths.resize(new + 1, between);
                }
                lengths[new] = between;
            }
        }
    }
}

fn collapse_short_edges(mesh: &mut DynamicMesh, lengths: &[f64]) {
    for (a, b) in mesh.edges() {
        if !mesh.has_edge(a, b) || length(mesh, a, b) >= 0.8 * asked(lengths, a, b) {
            continue;
        }
        // Either end may go, as long as the survivor gets no long edge out of it.
        for (from, to) in [(a, b), (b, a)] {
            let position = mesh.positions[to as usize];
            let stays_short = mesh
                .neighbors(from)
                .into_iter()
                .all(|n| mesh.positions[n as usize].distance(position) <= 4.0 / 3.0 * asked(lengths, n, to));
            if stays_short && mesh.can_collapse(from, to, position) {
                mesh.collapse(from, to, position);
                break;
            }
        }
    }
}

/// Flips every edge whose flip brings its four vertices closer to the ideal
/// valence (6 inside, 4 on a border): that is what makes triangles
/// equilateral on average.
fn equalize_valences(mesh: &mut DynamicMesh) {
    let deviation = |mesh: &DynamicMesh, v: u32, change: i64| {
        let ideal = if mesh.is_boundary_vertex(v) { 4 } else { 6 };
        (mesh.valence(v) as i64 + change - ideal).abs()
    };
    for (a, b) in mesh.edges() {
        if !mesh.has_edge(a, b) {
            continue;
        }
        let Some((c, d)) = mesh.can_flip(a, b) else { continue };
        let before: i64 = [a, b, c, d].iter().map(|&v| deviation(mesh, v, 0)).sum();
        let after: i64 =
            deviation(mesh, a, -1) + deviation(mesh, b, -1) + deviation(mesh, c, 1) + deviation(mesh, d, 1);
        if after < before {
            mesh.flip(a, b);
        }
    }
}

/// Rids the mesh of its slivers (#31) as Botsch and Kobbelt's remeshing
/// does: a needle, one edge much shorter than the other two, loses that edge
/// to a collapse; a cap, one angle nearly a straight one, has the edge
/// facing that angle flipped, or its corner laid onto that edge where no
/// flip will do. Feature lines are respected: a collapse only slides a
/// vertex of one along it (or makes one of two stretches of line that run
/// side by side), a flip never turns one, and a corner laid onto one stays
/// on it.
fn remove_slivers(mesh: &mut DynamicMesh) {
    let sliver = SLIVER_ANGLE.to_radians();
    for _ in 0..8 {
        let slivers: Vec<[u32; 3]> = mesh.triangles().filter(|&tri| mesh.min_angle_of(tri) < sliver).collect();
        let mut removed = 0;
        for tri in slivers {
            if mesh.has_triangle(tri) && mesh.min_angle_of(tri) < sliver && remove_sliver(mesh, tri) {
                removed += 1;
            }
        }
        if removed == 0 {
            break;
        }
    }
}

/// Above this angle, in degrees, the largest corner of a sliver makes it a
/// cap rather than a needle.
const CAP_ANGLE: f64 = 150.0;

fn remove_sliver(mesh: &mut DynamicMesh, tri: [u32; 3]) -> bool {
    let position = |v: u32| mesh.positions[v as usize];
    // The edge facing each corner, shortest first.
    let mut edges: Vec<(f64, u32, u32, u32)> = (0..3)
        .map(|k| {
            let (corner, a, b) = (tri[k], tri[(k + 1) % 3], tri[(k + 2) % 3]);
            (position(a).distance(position(b)), a, b, corner)
        })
        .collect();
    edges.sort_by(|x, y| x.0.total_cmp(&y.0));
    let (_, a, b, apex) = edges[2];
    let (u, w) =
        ((position(a) - position(apex)).normalize_or_zero(), (position(b) - position(apex)).normalize_or_zero());
    let largest = u.dot(w).clamp(-1.0, 1.0).acos().to_degrees();

    let needle = |mesh: &mut DynamicMesh| {
        let (_, p, q, _) = edges[0];
        for (from, to) in [(p, q), (q, p)] {
            let at = mesh.positions[to as usize];
            if mesh.can_merge_lines(from, to, at) {
                mesh.collapse(from, to, at);
                return true;
            }
        }
        false
    };
    let cap = |mesh: &mut DynamicMesh| {
        if mesh.can_flip_sliver(a, b) {
            mesh.flip(a, b);
            return true;
        }
        lay_onto(mesh, a, b, apex)
    };
    if largest > CAP_ANGLE { cap(mesh) || needle(mesh) } else { needle(mesh) || cap(mesh) }
}

/// Lays `apex`, the corner of a cap, onto its long side `a-b`: the side is
/// split where the apex is nearest to it, and the apex merged into the new
/// vertex. That is how a cap along a feature line, which no flip may turn,
/// goes: the line keeps its course and gains a vertex. Nothing changes if
/// the merge cannot be made.
fn lay_onto(mesh: &mut DynamicMesh, a: u32, b: u32, apex: u32) -> bool {
    if mesh.feature_degree(apex) > 0 {
        return false;
    }
    let (start, end) = (mesh.positions[a as usize], mesh.positions[b as usize]);
    let along = end - start;
    let length_squared = along.length_squared();
    if length_squared == 0.0 {
        return false;
    }
    let t = ((mesh.positions[apex as usize] - start).dot(along) / length_squared).clamp(0.0, 1.0);
    let foot = start + along * t;
    let middle = mesh.split(a, b, foot);
    if mesh.can_collapse(apex, middle, foot) {
        mesh.collapse(apex, middle, foot);
        true
    } else {
        // Undone: merged into `a`, which does not move, the new vertex
        // gives back the side and the two triangles it was split from.
        mesh.collapse(middle, a, start);
        false
    }
}

/// Moves every vertex towards the centre of its neighbours, within its
/// tangent plane, then back onto the source. Vertices of a feature line only
/// move along it, corners not at all.
///
/// Where the edges asked for vary (`lengths`), each neighbour counts for the
/// inverse of the edge asked for at it: the vertices then settle as far
/// apart as asked, where plain centres would even the edges out.
fn relax(mesh: &mut DynamicMesh, target: &Target, edge_length: f64, lengths: Option<&[f64]>) {
    // Where each vertex would go, side by side; the moves are then made in
    // turn.
    let moved: Vec<Option<Vec3>> = (0..mesh.vertex_count() as u32)
        .into_par_iter()
        .map(|v| {
            if !mesh.is_live_vertex(v) || mesh.is_corner(v) {
                return None;
            }
            let p = mesh.positions[v as usize];
            let line = mesh.feature_neighbors(v);
            match line[..] {
                [] => {
                    let neighbors = mesh.neighbors(v);
                    let center = match lengths {
                        None => {
                            neighbors.iter().map(|&n| mesh.positions[n as usize]).sum::<Vec3>() / neighbors.len() as f64
                        }
                        Some(lengths) => {
                            let weight = |n: u32| 1.0 / lengths[n as usize];
                            neighbors.iter().map(|&n| mesh.positions[n as usize] * weight(n)).sum::<Vec3>()
                                / neighbors.iter().map(|&n| weight(n)).sum::<f64>()
                        }
                    };
                    let normal = mesh.normal(v);
                    let tangential = center + normal * normal.dot(p - center);
                    Some(project(target, tangential, normal, edge_length))
                }
                [x, y] => {
                    let center = 0.5 * (mesh.positions[x as usize] + mesh.positions[y as usize]);
                    let slid = 0.5 * (p + center);
                    Some(target.features.and_then(|f| f.closest_point(slid)).map_or(p, |hit| hit.point))
                }
                _ => None,
            }
        })
        .collect();

    // Gauss-Seidel application: each move is checked against the positions
    // its neighbours actually have by then, so no fold can slip through.
    for (v, position) in moved.into_iter().enumerate() {
        if let Some(position) = position
            && mesh.move_keeps_orientation(v as u32, position, u32::MAX)
        {
            mesh.positions[v] = position;
        }
    }
}

/// Closest point of the source that faces the same way as `normal`, so that a
/// vertex near a thin wall is not pulled through to its other side.
fn project(target: &Target, p: Vec3, normal: Vec3, edge_length: f64) -> Vec3 {
    let same_side = |t: u32| target.face_normals[t as usize].dot(normal) > 0.0;
    target
        .surface
        .closest_point_where(p, 2.0 * edge_length, same_side)
        .or_else(|| target.surface.closest_point(p))
        .map_or(p, |hit| hit.point)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::{Topology, TriMesh};

    /// Two sheets folded at a right angle along a crease on the x axis,
    /// each a grid of 3 by 2 squares cut in two, with a cap along the
    /// crease (a vertex 0.002 off it, between its vertices at x = 1 and 2)
    /// and a needle (a vertex of the crease 0.01 from the one at x = 2).
    fn folded_with_slivers() -> (TriMesh, Vec<[u32; 2]>) {
        let mut positions = Vec::new();
        // Row 0 is the crease; rows 1 and 2 are on the flat sheet (y), rows
        // 3 and 4 on the folded one (-z).
        for row in 0..5 {
            for x in 0..4 {
                let x = x as f64;
                positions.push(match row {
                    0 => Vec3::new(x, 0.0, 0.0),
                    1 | 2 => Vec3::new(x, row as f64, 0.0),
                    _ => Vec3::new(x, 0.0, -(row as f64 - 2.0)),
                });
            }
        }
        let at = |row: u32, x: u32| 4 * row + x;
        let apex = positions.len() as u32;
        positions.push(Vec3::new(1.5, 0.002, 0.0));
        let near = positions.len() as u32;
        positions.push(Vec3::new(2.01, 0.0, 0.0));

        let mut triangles = Vec::new();
        // The flat sheet faces +z, the folded one +y.
        for (inner, outer) in [(0, 1), (1, 2)] {
            for x in 0..3 {
                let (a, b, c, d) = (at(inner, x), at(inner, x + 1), at(outer, x + 1), at(outer, x));
                if inner == 0 && x == 1 {
                    // The cell on the crease between x = 1 and 2, as a fan
                    // round the apex: its first triangle is the cap.
                    triangles.extend([[a, b, apex], [b, c, apex], [c, d, apex], [d, a, apex]]);
                } else if inner == 0 && x == 2 {
                    triangles.extend([[a, near, c], [near, b, c], [a, c, d]]);
                } else {
                    triangles.extend([[a, b, c], [a, c, d]]);
                }
            }
        }
        for (inner, outer) in [(0, 3), (3, 4)] {
            for x in 0..3 {
                let (a, b, c, d) = (at(inner, x), at(inner, x + 1), at(outer, x + 1), at(outer, x));
                if inner == 0 && x == 2 {
                    triangles.extend([[a, c, near], [near, c, b], [a, d, c]]);
                } else {
                    triangles.extend([[a, c, b], [a, d, c]]);
                }
            }
        }
        let mesh = TriMesh { positions, triangles };

        let topology = Topology::build(&mesh);
        let mut features: Vec<[u32; 2]> = topology
            .boundary_half_edges()
            .map(|h| {
                let tri = mesh.triangles[h / 3];
                [tri[h % 3], tri[(h + 1) % 3]]
            })
            .collect();
        features.extend([[at(0, 0), at(0, 1)], [at(0, 1), at(0, 2)], [at(0, 2), near], [near, at(0, 3)]]);
        (mesh, features)
    }

    #[test]
    fn slivers_along_a_crease_are_removed_and_the_crease_kept() {
        let (source, features) = folded_with_slivers();
        let sliver = SLIVER_ANGLE.to_radians();
        let mut mesh = DynamicMesh::new(&source, &features);
        assert_eq!(mesh.triangles().filter(|&tri| mesh.min_angle_of(tri) < sliver).count(), 3);
        let borders_before = Topology::build(&source).boundary_half_edges().count();

        remove_slivers(&mut mesh);

        assert_eq!(mesh.triangles().filter(|&tri| mesh.min_angle_of(tri) < sliver).count(), 0);
        let (result, features) = mesh.finish();
        let topology = Topology::build(&result);
        assert_eq!(topology.non_manifold_edges, 0);
        assert_eq!(topology.boundary_half_edges().count(), borders_before);
        // Every vertex is still on one of the two sheets, and the crease
        // is still a line of feature edges along the x axis, from end to end.
        for p in &result.positions {
            assert!(p.z.abs() < 1e-12 || p.y.abs() < 1e-12, "{p:?} is off the sheets");
        }
        let on_crease = |v: u32| {
            let p = result.positions[v as usize];
            p.y.abs() < 1e-12 && p.z.abs() < 1e-12
        };
        let crease: Vec<[u32; 2]> = features.into_iter().filter(|&[a, b]| on_crease(a) && on_crease(b)).collect();
        let length: f64 =
            crease.iter().map(|&[a, b]| result.positions[a as usize].distance(result.positions[b as usize])).sum();
        assert!((length - 3.0).abs() < 1e-9, "the crease is {length} long in feature edges");
    }
}
