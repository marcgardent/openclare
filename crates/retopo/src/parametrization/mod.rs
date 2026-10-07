//! Parametrization (stage M4): coordinates on the surface whose grid lines
//! are where the quad edges will run.
//!
//! The stage is a [`Parametrizer`]; chains get one from [`create`].

pub mod cut;
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
    pub report: Report,
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
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Seamless];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Seamless => "seamless",
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
    }
}
