//! Parametrization (stage M4): coordinates on the surface whose grid lines
//! are where the quad edges will run.
//!
//! The stage is a [`Parametrizer`]; chains get one from [`create`].

pub mod cut;
mod integer;
mod problem;
mod seamless;

use std::fmt;
use std::str::FromStr;

use crate::crossfield::CrossField;
use crate::error::{Error, Result};
use crate::preprocess::WorkMesh;
pub use cut::CutMesh;

#[derive(Clone, Debug)]
pub struct Request {
    /// Edge length of the quads to come: one unit of `(u, v)` on the surface.
    pub edge_length: f64,
    /// How many times to stiffen the faces that folded over and solve again.
    pub stiffening_rounds: usize,
}

/// What a parametrizer achieved.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    /// Edges on the cut.
    pub seam_edges: usize,
    /// Vertices of the cut-open mesh.
    pub wedges: usize,
    /// Faces folded over or squashed flat after the first solve.
    pub flipped_first: usize,
    /// The same at the start of the latest series of stiffening rounds.
    pub flipped_before_stiffening: usize,
    /// Quantities that were given a whole number.
    pub integers: usize,
    /// Feature edges could not be put on grid lines and were left to the
    /// field alone.
    pub features_off_grid: bool,
    /// The same for guide curves only: creases and borders are on grid
    /// lines, the curves are only followed by the field.
    pub guides_off_grid: bool,
    /// The edges of the guide curves that were let go, as their two
    /// vertices.
    pub loose_guides: Vec<[u32; 2]>,
    /// The triangles that folded over when every feature edge was put on a
    /// grid line, if that was tried and failed: where to look for what the
    /// map could not keep.
    pub folds_with_lines: Vec<usize>,
    /// The same at the end; anything but 0 means no valid mesh can be read
    /// off the map.
    pub flipped: usize,
    /// Stiffening rounds used.
    pub rounds: usize,
    /// Worst mismatch left across a seam, in parameter units.
    pub seam_error: f64,
    /// Mean deviation of the map from a rigid copy of each face (0 is ideal).
    pub distortion_mean: f64,
    /// Largest ratio between the two stretches of a face (1 is ideal).
    pub stretch_max: f64,
}

/// `(u, v)` per corner of the work mesh, on the surface cut open.
pub struct Parametrization {
    pub cut: CutMesh,
    /// Coordinates of corner `3 * f + k`.
    pub uv: Vec<[f64; 2]>,
    /// The faces the map was not asked to copy: slivers, and triangles laid
    /// along a feature line, which that line flattens. They weigh nothing in
    /// the solve.
    pub set_aside: Vec<bool>,
    pub report: Report,
}

impl Parametrization {
    /// Signed area of face `f` in the map.
    pub fn area(&self, f: usize) -> f64 {
        let [a, b, c] = [self.uv[3 * f], self.uv[3 * f + 1], self.uv[3 * f + 2]];
        0.5 * ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]))
    }

    /// Those of `edges` of `mesh` that do not lie on a grid line of the map:
    /// neither `u` nor `v` stays the same from one end to the other, in one
    /// of the faces they border. Measured as the layout does of a wall.
    pub fn off_grid(&self, mesh: &crate::mesh::TriMesh, edges: &[[u32; 2]]) -> Vec<[u32; 2]> {
        let wanted: std::collections::HashSet<(u32, u32)> = edges.iter().map(|&[a, b]| (a.min(b), a.max(b))).collect();
        let mut astray: std::collections::HashSet<(u32, u32)> = std::collections::HashSet::new();
        for (f, tri) in mesh.triangles.iter().enumerate() {
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                let edge = (a.min(b), a.max(b));
                if !wanted.contains(&edge) {
                    continue;
                }
                let (from, to) = (self.uv[3 * f + k], self.uv[3 * f + (k + 1) % 3]);
                let (du, dv) = ((to[0] - from[0]).abs(), (to[1] - from[1]).abs());
                if du.min(dv) > (1e-3 * du.max(dv)).max(1e-6) {
                    astray.insert(edge);
                }
            }
        }
        edges.iter().copied().filter(|&[a, b]| astray.contains(&(a.min(b), a.max(b)))).collect()
    }

    /// The faces turned over in the map: of negative area, beyond what a
    /// flat face may be off by. A face that is merely flat is not one: a
    /// trace of the layout crosses it in no distance. This is the one count
    /// of folds, for the map that decides what lines to keep and for the
    /// layout that is cut from it.
    pub fn turned_over(&self) -> Vec<usize> {
        let faces = self.uv.len() / 3;
        let flat = 1e-9 * (0..faces).map(|f| self.area(f).abs()).sum::<f64>() / faces.max(1) as f64;
        (0..faces).filter(|&f| self.area(f) < -flat).collect()
    }
}

pub trait Parametrizer {
    fn kind(&self) -> Kind;
    fn parametrize(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<Parametrization>;
}

/// The parametrizers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Least-squares fit of the gradients to the cross field, seamless
    /// across the cut (after Bommes et al. 2009), no integer imposed.
    Seamless,
    /// The same with singularities, seam shifts and feature edges on whole
    /// numbers, found by greedy rounding: its integer grid lines are the
    /// edges of a quad mesh. A provisional shortcut.
    IntegerGrid,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Seamless, Kind::IntegerGrid];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Seamless => "seamless",
            Kind::IntegerGrid => "integer-grid",
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
            Error::InvalidParams(format!("unknown parametrizer '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the parametrizer of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn Parametrizer> {
    match kind {
        Kind::Seamless => Box::new(seamless::Seamless::default()),
        Kind::IntegerGrid => Box::new(integer::IntegerGrid::default()),
    }
}
