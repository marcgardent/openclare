//! Isotropic remeshing (Botsch & Kobbelt, "A Remeshing Approach to
//! Multiresolution Modeling", 2004): repeat "split the long edges, collapse
//! the short ones, flip towards valence 6, relax, project back" until the
//! triangles are all about the same size and close to equilateral.
//!
//! The solvers downstream discretise differential operators on this mesh;
//! slivers and wild size variations are what make them inaccurate.

use rayon::prelude::*;

use super::Refinement;
use super::dynamic_mesh::DynamicMesh;
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
