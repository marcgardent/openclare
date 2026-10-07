//! Meshing (stages M7 and M8): from the quantized layout to a quad mesh on
//! the surface.
//!
//! The stage is a [`Mesher`]; chains get one from [`create`].

mod patch_grid;

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::mesh::PolyMesh;
use crate::preprocess::WorkMesh;
use crate::quantization::Quantization;
use crate::tmesh::Layout;

/// What a mesher did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    pub vertices: usize,
    pub quads: usize,
    /// Vertices that could not be placed by reading the map and were put
    /// between their neighbours on the rim of their patch instead.
    pub interpolated: usize,
}

pub trait Mesher {
    fn kind(&self) -> Kind;
    /// The quad mesh of `quantization` on `layout`, its vertices on the
    /// work mesh.
    fn mesh(&self, work: &WorkMesh, layout: &Layout, quantization: &Quantization) -> Result<(PolyMesh, Report)>;
}

/// The meshers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// One grid per rectangle of the T-mesh, each vertex placed by reading
    /// the seamless map inside its rectangle.
    PatchGrid,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::PatchGrid];

    pub fn name(self) -> &'static str {
        match self {
            Kind::PatchGrid => "patch-grid",
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
            Error::InvalidParams(format!("unknown mesher '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the mesher of that kind.
pub fn create(kind: Kind) -> Box<dyn Mesher> {
    match kind {
        Kind::PatchGrid => Box::new(patch_grid::PatchGrid),
    }
}
