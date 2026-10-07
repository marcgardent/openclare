//! Relaxation (stage M9): final vertex placement. The mesh is evened out
//! while staying on the source surface, its hard edges on the source's hard
//! edges.
//!
//! The stage is a [`Relaxer`]; chains get one from [`create`].

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
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

pub trait Relaxer {
    fn kind(&self) -> Kind;
    /// Moves the vertices of `mesh` on `surface`, holding those that belong
    /// to `features` on them.
    fn relax(&self, mesh: &mut PolyMesh, surface: &Bvh, features: Option<&Features>);
}

/// The relaxers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Every vertex towards the mean of its neighbours. Simple, but it wears
    /// thin parts away if run for long.
    Laplacian,
    /// Every quad towards the rectangle that fits it best, each vertex kept
    /// near where it started.
    Squaring,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Laplacian, Kind::Squaring];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Laplacian => "laplacian",
            Kind::Squaring => "squaring",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        Kind::ALL.iter().copied().find(|k| k.name() == name).ok_or_else(|| {
            let known: Vec<&str> = Kind::ALL.iter().map(|k| k.name()).collect();
            Error::InvalidParams(format!("unknown relaxer '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the relaxer of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn Relaxer> {
    match kind {
        Kind::Laplacian => Box::new(Laplacian::default()),
        Kind::Squaring => Box::new(Squaring::default()),
    }
}

/// Tuning of [`relax`].
#[derive(Clone, Debug)]
pub struct Laplacian {
    pub iterations: usize,
}

impl Default for Laplacian {
    fn default() -> Self {
        Self { iterations: 2 }
    }
}

impl Relaxer for Laplacian {
    fn kind(&self) -> Kind {
        Kind::Laplacian
    }

    fn relax(&self, mesh: &mut PolyMesh, surface: &Bvh, features: Option<&Features>) {
        relax(mesh, surface, features, self.iterations);
    }
}

/// What a vertex is allowed to do.
#[derive(Clone, Copy, PartialEq)]
enum Hold {
    /// Slides on the surface.
    Free,
    /// Slides along a feature line.
    Line,
    /// Does not move.
    Pinned,
}

/// Sorts the vertices of `mesh` by what they may do, snapping those that
/// belong to a feature line or corner onto it. Returns that and the
/// neighbours of each vertex.
fn classify(mesh: &mut PolyMesh, features: Option<&Features>) -> (Vec<Hold>, Vec<Vec<u32>>) {
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

    let mut kind = vec![Hold::Free; n];
    if let Some(features) = features {
        let limit = features.snap_distance * features.snap_distance;
        // Each corner captures the one vertex nearest to it.
        let mut captured: Vec<Option<(f64, usize)>> = Vec::new();
        for v in 0..n {
            let p = mesh.positions[v];
            if let Some(hit) = features.lines.closest_point(p).filter(|hit| hit.distance_squared < limit) {
                kind[v] = Hold::Line;
                mesh.positions[v] = hit.point;
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
            kind[v] = Hold::Pinned;
            mesh.positions[v] = features.corners.closest_point(mesh.positions[v]).unwrap().point;
        }
    }
    for v in 0..n {
        // A border that follows no feature is a hole of the extraction (or a
        // boundary we were told not to align with): leave its shape alone.
        if on_border[v] && kind[v] == Hold::Free {
            kind[v] = Hold::Pinned;
        }
    }
    (kind, neighbors)
}

/// Laplacian smoothing constrained to the source: free vertices are projected
/// back onto the surface after every pass, vertices on a feature line onto
/// that line, and corners stay put.
pub fn relax(mesh: &mut PolyMesh, surface: &Bvh, features: Option<&Features>, iterations: usize) {
    let n = mesh.positions.len();
    let (kind, neighbors) = classify(mesh, features);
    let on_surface = |p: Vec3| surface.closest_point(p).map_or(p, |hit| hit.point);
    let on_line = |p: Vec3| features.and_then(|f| f.lines.closest_point(p)).map_or(p, |hit| hit.point);

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
                    Hold::Pinned => p,
                    Hold::Free => mean_of(&|_| true).map_or(p, |mean| on_surface(0.5 * (p + mean))),
                    // Only its neighbours along the line pull it; the others
                    // would drag it across.
                    Hold::Line => {
                        let along = |nb: u32| kind[nb as usize] != Hold::Free;
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
        if kind[v] == Hold::Free {
            mesh.positions[v] = on_surface(mesh.positions[v]);
        }
    }
}

/// Tuning of [`square_up`].
#[derive(Clone, Debug)]
pub struct Squaring {
    pub iterations: usize,
    /// How far a vertex may end up from where it started, in times the mean
    /// length of its edges at the start. This is what keeps thin parts and
    /// tips from being worn away, pass after pass.
    pub reach: f64,
    /// Share of plain smoothing (towards the mean of the neighbours) mixed
    /// into the pull towards rectangles: it evens out the sizes of the quads,
    /// which the rectangles alone leave as they are.
    pub smoothing: f64,
    /// Fraction of the way to its goal a vertex goes in one pass.
    pub step: f64,
    /// A move may not leave a quad round the vertex with a corner flatter
    /// than this (the sine of its angle), unless it was flatter before.
    pub flattest: f64,
}

impl Default for Squaring {
    fn default() -> Self {
        Self { iterations: 60, reach: 0.5, smoothing: 0.25, step: 0.6, flattest: 0.3 }
    }
}

impl Relaxer for Squaring {
    fn kind(&self) -> Kind {
        Kind::Squaring
    }

    fn relax(&self, mesh: &mut PolyMesh, surface: &Bvh, features: Option<&Features>) {
        square_up(mesh, surface, features, self);
    }
}

/// Relaxation that makes quads rectangular rather than merely even.
///
/// Each quad proposes, for its four corners, the rectangle that fits it
/// best: same centre, same plane, sides the mean of its opposite sides. A
/// vertex goes towards the mean of what its quads propose, sliding on the
/// surface (the move is made tangent, then projected), and never further
/// than [`Squaring::reach`] from where it started. A move that would flatten
/// or fold a quad is not made. Feature lines and corners hold their vertices
/// as in [`relax`].
pub fn square_up(mesh: &mut PolyMesh, surface: &Bvh, features: Option<&Features>, tuning: &Squaring) {
    let n = mesh.positions.len();
    let (kind, neighbors) = classify(mesh, features);
    let on_surface = |p: Vec3| surface.closest_point(p).map_or(p, |hit| hit.point);
    let on_line = |p: Vec3| features.and_then(|f| f.lines.closest_point(p)).map_or(p, |hit| hit.point);

    let origin = mesh.positions.clone();
    let radius: Vec<f64> = (0..n)
        .map(|v| {
            let total: f64 = neighbors[v].iter().map(|&nb| origin[v].distance(origin[nb as usize])).sum();
            tuning.reach * total / neighbors[v].len().max(1) as f64
        })
        .collect();
    let mut faces_of = vec![Vec::new(); n];
    for (face, corners) in mesh.faces.iter().enumerate() {
        for &v in corners {
            faces_of[v as usize].push(face as u32);
        }
    }

    for _ in 0..tuning.iterations {
        // What every quad would have its corners be, summed per vertex,
        // and the normal of the mesh at each vertex.
        let mut proposed = vec![(Vec3::ZERO, 0.0); n];
        let mut normal = vec![Vec3::ZERO; n];
        for corners in &mesh.faces {
            let points: Vec<Vec3> = corners.iter().map(|&v| mesh.positions[v as usize]).collect();
            let face_normal = polygon_normal(&points);
            for &v in corners {
                normal[v as usize] += face_normal;
            }
            let Some(rectangle) = fitted_rectangle(&points) else { continue };
            for (&v, corner) in corners.iter().zip(rectangle) {
                proposed[v as usize].0 += corner;
                proposed[v as usize].1 += 1.0;
            }
        }

        let goals: Vec<Option<Vec3>> = (0..n)
            .map(|v| {
                let p = mesh.positions[v];
                if kind[v] == Hold::Pinned || proposed[v].1 == 0.0 {
                    return None;
                }
                let along_line = |nb: u32| kind[v] == Hold::Free || kind[nb as usize] != Hold::Free;
                let (sum, count) = neighbors[v]
                    .iter()
                    .filter(|&&nb| along_line(nb))
                    .fold((Vec3::ZERO, 0.0), |(s, c), &nb| (s + mesh.positions[nb as usize], c + 1.0));
                let rectangle = proposed[v].0 / proposed[v].1;
                let goal = if count > 0.0 { rectangle.lerp(sum / count, tuning.smoothing) } else { rectangle };
                // Slide: what points out of the surface is dropped.
                let mut pull = goal - p;
                let out = normal[v].normalize_or_zero();
                pull -= out * pull.dot(out);
                let mut moved = p + tuning.step * pull;
                let from_start = moved - origin[v];
                if from_start.length() > radius[v] {
                    moved = origin[v] + from_start * (radius[v] / from_start.length());
                }
                Some(if kind[v] == Hold::Line { on_line(moved) } else { on_surface(moved) })
            })
            .collect();

        // One vertex at a time, so that each move is judged against the
        // quads as they now are.
        for v in 0..n {
            let Some(goal) = goals[v] else { continue };
            let worst = |mesh: &PolyMesh| {
                faces_of[v].iter().map(|&face| flattest_corner(mesh, face as usize)).fold(f64::INFINITY, f64::min)
            };
            let before = worst(mesh);
            let kept = std::mem::replace(&mut mesh.positions[v], goal);
            if worst(mesh) < before.min(tuning.flattest) {
                mesh.positions[v] = kept;
            }
        }
    }
}

/// Unit normal of a polygon that need not be flat.
fn polygon_normal(points: &[Vec3]) -> Vec3 {
    let count = points.len();
    (0..count)
        .map(|k| (points[(k + 1) % count] - points[k]).cross(points[(k + count - 1) % count] - points[k]))
        .sum::<Vec3>()
        .normalize_or_zero()
}

/// The rectangle closest to a quad: its centre, its plane, sides the mean
/// of the quad's opposite sides. `None` for anything but a proper quad.
fn fitted_rectangle(points: &[Vec3]) -> Option<[Vec3; 4]> {
    let &[a, b, c, d] = points else { return None };
    let centre = 0.25 * (a + b + c + d);
    let (width, height) = (0.5 * (a.distance(b) + d.distance(c)), 0.5 * (a.distance(d) + b.distance(c)));
    let along = ((b - a) + (c - d)).try_normalize()?;
    let up = ((d - a) + (c - b)).try_normalize()?;
    // The orthonormal pair closest to those two directions: turn each by
    // the same angle, away from or towards the other.
    let (sum, difference) = ((along + up).try_normalize()?, (along - up).try_normalize()?);
    let along = (sum + difference) * std::f64::consts::FRAC_1_SQRT_2;
    let up = (sum - difference) * std::f64::consts::FRAC_1_SQRT_2;
    let (half_width, half_height) = (0.5 * width * along, 0.5 * height * up);
    Some([
        centre - half_width - half_height,
        centre + half_width - half_height,
        centre + half_width + half_height,
        centre - half_width + half_height,
    ])
}

/// The sine of the flattest corner of a face, negative if it folds over.
fn flattest_corner(mesh: &PolyMesh, face: usize) -> f64 {
    let corners = &mesh.faces[face];
    let points: Vec<Vec3> = corners.iter().map(|&v| mesh.positions[v as usize]).collect();
    let normal = polygon_normal(&points);
    let count = points.len();
    (0..count)
        .map(|k| {
            let (next, previous) = (points[(k + 1) % count] - points[k], points[(k + count - 1) % count] - points[k]);
            let scale = next.length() * previous.length();
            if scale > 0.0 { next.cross(previous).dot(normal) / scale } else { -1.0 }
        })
        .fold(f64::INFINITY, f64::min)
}
