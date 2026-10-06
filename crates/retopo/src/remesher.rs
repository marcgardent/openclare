//! Method-independent entry point.
//!
//! Every remeshing chain (the Instant Meshes baseline today, the
//! parametrization chain for organic shapes and the patch chain for hard
//! surfaces later) is a [`Remesher`]. Callers pick one by [`Method`] and get
//! it from [`create`]; nothing outside this module names a concrete chain.

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::mesh::{PolyMesh, TriMesh};
use crate::pipeline::{self, Params, Report, Target};

/// What the user asks for, whatever the method.
#[derive(Clone, Debug)]
pub struct Request {
    pub target: Target,
    /// Dihedral angle in degrees above which an input edge is a hard edge the
    /// output must follow. `None` disables crease detection.
    pub crease_angle: Option<f64>,
    /// Make the output follow open boundaries.
    pub align_boundaries: bool,
    pub seed: u64,
}

impl Default for Request {
    fn default() -> Self {
        let params = Params::default();
        Self {
            target: params.target,
            crease_angle: params.crease_angle,
            align_boundaries: params.align_boundaries,
            seed: params.seed,
        }
    }
}

/// A complete triangle-to-quad remeshing chain.
pub trait Remesher {
    fn method(&self) -> Method;
    fn remesh(&self, input: &TriMesh, request: &Request) -> Result<(PolyMesh, Report)>;
}

/// The chains [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    /// Local orientation and position fields (Jakob et al. 2015). The
    /// reference floor the other chains are measured against.
    InstantMeshes,
}

impl Method {
    pub const ALL: &[Method] = &[Method::InstantMeshes];

    pub fn name(self) -> &'static str {
        match self {
            Method::InstantMeshes => "instant-meshes",
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        Method::ALL.iter().copied().find(|m| m.name() == name).ok_or_else(|| {
            let known: Vec<&str> = Method::ALL.iter().map(|m| m.name()).collect();
            Error::InvalidParams(format!("unknown method '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the chain for `method` with its default tuning.
pub fn create(method: Method) -> Box<dyn Remesher> {
    match method {
        Method::InstantMeshes => Box::new(InstantMeshes::default()),
    }
}

/// Tuning of the Instant Meshes chain (see [`pipeline::remesh`]).
#[derive(Clone, Debug)]
pub struct InstantMeshes {
    pub pure_quad: bool,
    pub orientation_iterations: usize,
    pub position_iterations: usize,
    pub relax_iterations: usize,
}

impl Default for InstantMeshes {
    fn default() -> Self {
        let params = Params::default();
        Self {
            pure_quad: params.pure_quad,
            orientation_iterations: params.orientation_iterations,
            position_iterations: params.position_iterations,
            relax_iterations: params.relax_iterations,
        }
    }
}

impl Remesher for InstantMeshes {
    fn method(&self) -> Method {
        Method::InstantMeshes
    }

    fn remesh(&self, input: &TriMesh, request: &Request) -> Result<(PolyMesh, Report)> {
        let params = Params {
            target: request.target,
            pure_quad: self.pure_quad,
            crease_angle: request.crease_angle,
            align_boundaries: request.align_boundaries,
            orientation_iterations: self.orientation_iterations,
            position_iterations: self.position_iterations,
            relax_iterations: self.relax_iterations,
            seed: request.seed,
        };
        pipeline::remesh(input, &params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::primitives;

    #[test]
    fn every_method_round_trips_through_its_name() {
        for &method in Method::ALL {
            assert_eq!(method.name().parse::<Method>().unwrap(), method);
            assert_eq!(create(method).method(), method);
        }
        assert!("no-such-method".parse::<Method>().is_err());
    }

    #[test]
    fn factory_built_chain_matches_the_direct_call() {
        let source = primitives::torus(1.0, 0.4, 48, 24);
        let request = Request { target: Target::QuadCount(400), ..Default::default() };
        let (via_factory, _) = create(Method::InstantMeshes).remesh(&source, &request).unwrap();
        let params = Params { target: Target::QuadCount(400), ..Default::default() };
        let (direct, _) = pipeline::remesh(&source, &params).unwrap();
        assert_eq!(via_factory.faces, direct.faces);
        assert_eq!(via_factory.positions, direct.positions);
    }
}
