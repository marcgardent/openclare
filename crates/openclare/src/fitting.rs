//! Fitting: the last touch on where the vertices sit, for the mesh to keep
//! the volume of the shape.
//!
//! Relaxation leaves every vertex on the source surface. Wherever that
//! surface bulges, the faces then run under it, flat between their corners:
//! the mesh is inscribed in the shape and smaller than it, by a few per cent
//! at low budgets and much more on a thin part (six faces round a finger
//! hold 83 % of its section). Fitting moves the vertices off the surface,
//! along their normals, until the faces run through the surface rather than
//! under it: as much of the mesh above as below.
//!
//! Artists subdivide the mesh, and what they see then is its Catmull-Clark
//! limit surface, which runs inside the faces wherever the shape bulges.
//! [`Kind::LimitSurface`] fits that surface instead of the faces.
//!
//! The stage is a [`Fitter`]; chains get one from [`create`].

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::math::Vec3;
use crate::mesh::{PolyMesh, TriMesh};
use crate::relax::{Features, Hold, classify};
use crate::spatial::Bvh;
use crate::subdivision::{self, Stencil};

pub trait Fitter {
    fn kind(&self) -> Kind;
    /// Moves the vertices of `mesh`, which lie on `source`, so that it fits
    /// `source` on the whole. `surface` is the hierarchy over the triangles
    /// of `source`. Vertices that `features` holds, and those of a border,
    /// stay where they are. `amount` is how far to go: 0 leaves the mesh as
    /// it is, 1 goes all the way.
    fn fit(&self, mesh: &mut PolyMesh, source: &TriMesh, surface: &Bvh, features: Option<&Features>, amount: f64);
}

/// The fitters [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The faces as close to the surface as they can be on the whole, in the
    /// least-squares sense, the vertices moving along their normals.
    LeastSquares,
    /// The same, for the limit surface of the mesh rather than its faces:
    /// the surface a Catmull-Clark subdivision converges to, which is what
    /// an artist sees once the mesh is subdivided.
    LimitSurface,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::LeastSquares, Kind::LimitSurface];

    pub fn name(self) -> &'static str {
        match self {
            Kind::LeastSquares => "least-squares",
            Kind::LimitSurface => "limit-surface",
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
            Error::InvalidParams(format!("unknown fitter '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the fitter of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn Fitter> {
    match kind {
        Kind::LeastSquares => Box::new(LeastSquares::default()),
        Kind::LimitSurface => Box::new(LeastSquares { limit_surface: true, ..LeastSquares::default() }),
    }
}

#[derive(Clone, Debug)]
pub struct LeastSquares {
    /// Rounds of measuring how far the faces are from the surface and moving
    /// the vertices to make up for it.
    pub rounds: usize,
    /// Points looked at on each quad: this many along each of its sides.
    /// On the limit surface, the middles of the sixteen quads two
    /// subdivisions make of it, whatever this says.
    pub samples: usize,
    /// Farthest a vertex may end up from where it started, as a share of the
    /// shortest edge it is on. Six faces round a finger call for an eighth
    /// of an edge; much more than that is not a matter of volume.
    pub reach: f64,
    /// Most a face may turn, in radians.
    pub tilt: f64,
    /// Fit the limit surface of the mesh rather than its faces.
    pub limit_surface: bool,
}

impl Default for LeastSquares {
    fn default() -> Self {
        Self { rounds: 8, samples: 4, reach: 0.25, tilt: 20f64.to_radians(), limit_surface: false }
    }
}

impl LeastSquares {
    /// The points looked at, each with the quad it is on and as weights on
    /// the vertices of the mesh. Only quads are looked at.
    fn points(&self, mesh: &PolyMesh) -> Vec<(u32, Stencil)> {
        let quads = (0..mesh.faces.len() as u32).filter(|&f| mesh.faces[f as usize].len() == 4);
        if !self.limit_surface {
            // The middles of a grid of cells, between the corners.
            let along: Vec<f64> = (0..self.samples).map(|i| (i as f64 + 0.5) / self.samples as f64).collect();
            return quads
                .flat_map(|f| {
                    let face = &mesh.faces[f as usize];
                    let along = &along;
                    along.iter().flat_map(move |&u| {
                        along.iter().map(move |&v| {
                            let shares = [(1.0 - u) * (1.0 - v), u * (1.0 - v), u * v, (1.0 - u) * v];
                            (f, (0..4).map(|k| (face[k], shares[k])).collect())
                        })
                    })
                })
                .collect();
        }
        // On the limit surface: the middles of the quads of the mesh
        // subdivided twice, their corners taken to the limit. Two
        // subdivisions are what the bench looks at, and what Blender shows
        // by default.
        let vertices = mesh.positions.len();
        let once = subdivision::refine(&mesh.faces, vertices);
        let twice = subdivision::refine(&once.faces, once.rows.len());
        let limit = subdivision::limit(&twice.faces, twice.rows.len());
        let on_mesh =
            subdivision::compose(&subdivision::compose(&limit, &twice.rows, once.rows.len()), &once.rows, vertices);
        let middles: Vec<Stencil> = twice.faces.iter().map(|face| face.iter().map(|&v| (v, 0.25)).collect()).collect();
        let middles = subdivision::compose(&middles, &on_mesh, vertices);
        let quad = |f: u32| mesh.faces[f as usize].len() == 4;
        twice.parent.iter().map(|&p| once.parent[p as usize]).zip(middles).filter(|&(f, _)| quad(f)).collect()
    }
}

impl Fitter for LeastSquares {
    fn kind(&self) -> Kind {
        if self.limit_surface { Kind::LimitSurface } else { Kind::LeastSquares }
    }

    fn fit(&self, mesh: &mut PolyMesh, source: &TriMesh, surface: &Bvh, features: Option<&Features>, amount: f64) {
        if amount.is_nan() || amount <= 0.0 || mesh.faces.is_empty() {
            return;
        }
        let vertices = mesh.positions.len();
        let (hold, neighbors) = classify(mesh, features);
        let start = mesh.positions.clone();
        // How far each vertex may go.
        let leash: Vec<f64> = (0..vertices)
            .map(|v| {
                let shortest =
                    neighbors[v].iter().map(|&nb| start[v].distance(start[nb as usize])).fold(f64::INFINITY, f64::min);
                if shortest.is_finite() { self.reach * shortest } else { 0.0 }
            })
            .collect();

        let points = self.points(mesh);
        for _ in 0..self.rounds {
            let normals = vertex_normals(mesh);
            let mut pull = vec![0.0; vertices];
            let mut weight = vec![0.0; vertices];
            // For each quad, which way it faces and how wide it is.
            let quads: Vec<(Vec3, f64)> = mesh
                .faces
                .iter()
                .map(|face| {
                    if face.len() != 4 {
                        return (Vec3::ZERO, 0.0);
                    }
                    let corners: [Vec3; 4] = std::array::from_fn(|k| mesh.positions[face[k] as usize]);
                    let facing = (corners[2] - corners[0]).cross(corners[3] - corners[1]);
                    (facing, (corners[2] - corners[0]).length().max((corners[3] - corners[1]).length()))
                })
                .collect();
            for (face, stencil) in &points {
                // Nothing farther than the face is wide is this face's to fit.
                let (facing, width) = quads[*face as usize];
                let point = subdivision::apply(stencil, &mesh.positions);
                // The sheet of the source that faces the same way: on a thin
                // part the nearest one may be the other side.
                let same_side = |t: u32| source.area_vector(t as usize).dot(facing) > 0.0;
                let Some(hit) = surface.closest_point_where(point, width, same_side) else { continue };
                let off = hit.point - point;
                for &(corner, share) in stencil {
                    let corner = corner as usize;
                    pull[corner] += share * off.dot(normals[corner]);
                    weight[corner] += share;
                }
            }
            for v in (0..vertices).filter(|&v| hold[v] == Hold::Free && weight[v] > 0.0) {
                let moved = mesh.positions[v] + normals[v] * (pull[v] / weight[v]);
                let from_start = moved - start[v];
                let distance = from_start.length();
                mesh.positions[v] =
                    if distance > leash[v] { start[v] + from_start * (leash[v] / distance) } else { moved };
            }
        }
        if amount < 1.0 {
            for v in 0..vertices {
                mesh.positions[v] = start[v] + (mesh.positions[v] - start[v]) * amount;
            }
        }

        // No face may come out folded that did not go in folded, nor tilted
        // by much: fitting moves vertices in and out, which hardly turns a
        // face, and one that turns has been pulled towards another sheet of
        // the source. Where that happens the vertices of the face go back to
        // where they were, and so on while that upsets another.
        for _ in 0..FOLD_PASSES {
            let mut back = vec![false; vertices];
            for face in &mesh.faces {
                let (was, now) = (facing(&start, face), facing(&mesh.positions, face));
                let tilted = was.dot(now) < self.tilt.cos() * was.length() * now.length();
                let folded =
                    turns(&start, face).iter().zip(turns(&mesh.positions, face)).any(|(&was, now)| was && !now);
                if tilted || folded {
                    for &v in face {
                        back[v as usize] = true;
                    }
                }
            }
            if !back.contains(&true) {
                break;
            }
            for v in (0..vertices).filter(|&v| back[v]) {
                mesh.positions[v] = start[v];
            }
        }
    }
}

/// Passes of undoing the moves that fold a face.
const FOLD_PASSES: usize = 8;

/// For each corner of a face, whether it turns the way the face does on the
/// whole.
fn turns(positions: &[Vec3], face: &[u32]) -> Vec<bool> {
    let at = |k: usize| positions[face[k % face.len()] as usize];
    let whole = facing(positions, face);
    (0..face.len()).map(|k| (at(k + 1) - at(k)).cross(at(k + face.len() - 1) - at(k)).dot(whole) > 0.0).collect()
}

/// Which way a face faces: its area as a vector.
fn facing(positions: &[Vec3], face: &[u32]) -> Vec3 {
    let at = |k: usize| positions[face[k % face.len()] as usize];
    let centre: Vec3 = (0..face.len()).map(at).sum::<Vec3>() / face.len() as f64;
    (0..face.len()).map(|k| (at(k) - centre).cross(at(k + 1) - centre)).sum()
}

/// Unit normals at the vertices of a polygon mesh, each face weighing in by
/// its area.
fn vertex_normals(mesh: &PolyMesh) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; mesh.positions.len()];
    for face in &mesh.faces {
        let centre: Vec3 = face.iter().map(|&v| mesh.positions[v as usize]).sum::<Vec3>() / face.len() as f64;
        let mut area = Vec3::ZERO;
        for k in 0..face.len() {
            let (a, b) = (mesh.positions[face[k] as usize], mesh.positions[face[(k + 1) % face.len()] as usize]);
            area += (a - centre).cross(b - centre);
        }
        for &v in face {
            normals[v as usize] += area;
        }
    }
    normals.into_iter().map(Vec3::normalize_or_zero).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_fitter_round_trips_through_its_name() {
        for &kind in Kind::ALL {
            assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
            assert_eq!(create(kind).kind(), kind);
        }
        assert!("no-such-fitter".parse::<Kind>().is_err());
    }
}
