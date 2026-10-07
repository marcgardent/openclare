//! Method-independent entry point.
//!
//! Every remeshing chain (the Instant Meshes baseline, the parametrization
//! chain for organic shapes, the patch chain for hard surfaces later) is a
//! [`Remesher`]. Callers pick one by [`Method`] and get it from [`create`];
//! nothing outside this module names a concrete chain.

use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use crate::config::RemeshConfig;
use crate::crossfield;
use crate::error::{Error, Result};
use crate::extract::{extract_faces, grid::graph_from_parametrization};
use crate::mesh::{PolyMesh, TriMesh};
use crate::parametrization;
use crate::pipeline::{self, Params, Report, Target};
use crate::preprocess;
use crate::relax::{Features, relax};
use crate::singularity;
use crate::spatial::Bvh;

/// A complete triangle-to-quad remeshing chain.
pub trait Remesher {
    fn method(&self) -> Method;
    fn remesh(&self, input: &TriMesh, config: &RemeshConfig) -> Result<(PolyMesh, Report)>;
}

/// The chains [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    /// Local orientation and position fields (Jakob et al. 2015). The
    /// reference floor the other chains are measured against.
    InstantMeshes,
    /// Cross field, singularity simplification, integer-grid parametrization
    /// and extraction of its grid lines. The integers come from greedy
    /// rounding for now, which makes this chain provisional.
    IntegerGrid,
}

impl Method {
    pub const ALL: &[Method] = &[Method::InstantMeshes, Method::IntegerGrid];

    pub fn name(self) -> &'static str {
        match self {
            Method::InstantMeshes => "instant-meshes",
            Method::IntegerGrid => "integer-grid",
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
        Method::IntegerGrid => Box::new(IntegerGrid::default()),
    }
}

/// Tuning of the Instant Meshes chain (see [`pipeline::remesh`]). It reads
/// the target, the crease angle and the seed of the configuration; the
/// field settings do not apply to it.
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

    fn remesh(&self, input: &TriMesh, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
        config.validate()?;
        let params = Params {
            target: config.target,
            pure_quad: self.pure_quad,
            crease_angle: config.crease_angle,
            align_boundaries: true,
            orientation_iterations: self.orientation_iterations,
            position_iterations: self.position_iterations,
            relax_iterations: self.relax_iterations,
            seed: config.seed,
        };
        pipeline::remesh(input, &params)
    }
}

/// Tuning of the integer-grid chain: what the configuration does not cover.
#[derive(Clone, Debug)]
pub struct IntegerGrid {
    /// Triangles of the work mesh per quad asked for, and the bounds it is
    /// kept within: enough of them for the fields to be accurate, few enough
    /// for the solves to stay quick.
    pub triangles_per_quad: f64,
    pub min_triangles: usize,
    pub max_triangles: usize,
    /// Most stiffening rounds against inverted triangles in the map.
    pub stiffening_rounds: usize,
    /// Smoothing passes on the final mesh.
    pub relax_iterations: usize,
}

impl Default for IntegerGrid {
    fn default() -> Self {
        Self {
            triangles_per_quad: 10.0,
            min_triangles: 20_000,
            max_triangles: 120_000,
            stiffening_rounds: 40,
            relax_iterations: 2,
        }
    }
}

impl Remesher for IntegerGrid {
    fn method(&self) -> Method {
        Method::IntegerGrid
    }

    fn remesh(&self, input: &TriMesh, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
        config.validate()?;
        input.validate()?;
        let area = input.surface_area();
        let (quads, edge_length) = match config.target {
            Target::QuadCount(0) => return Err(Error::InvalidParams("target quad count is zero".into())),
            Target::QuadCount(count) => (count as f64, (area / count as f64).sqrt()),
            Target::EdgeLength(length) if length.is_finite() && length > 0.0 => (area / (length * length), length),
            Target::EdgeLength(length) => {
                return Err(Error::InvalidParams(format!("edge length {length} is not positive")));
            }
        };

        let mut report = Report { edge_length, ..Default::default() };
        let mut clock = Instant::now();
        let mut lap = |report: &mut Report, stage: &'static str| {
            report.timings.push((stage, clock.elapsed()));
            clock = Instant::now();
        };

        let triangles = ((self.triangles_per_quad * quads) as usize).clamp(self.min_triangles, self.max_triangles);
        let work_request =
            preprocess::Request { triangles, crease_angle: config.crease_angle, curvature_scale: Some(edge_length) };
        let work = preprocess::create(preprocess::Kind::Isotropic).preprocess(input, &work_request)?;
        report.working_vertices = work.mesh.positions.len();
        report.feature_edges = work.feature_edges.len();
        lap(&mut report, "preprocess");

        let field_request = crossfield::Request { alignment: config.alignment, seed: config.seed };
        let mut field = crossfield::create(crossfield::Kind::Power).solve(&work, &field_request)?;
        lap(&mut report, "cross field");

        if config.singularity_reach > 0.0 {
            let request = singularity::Request {
                alignment: config.alignment,
                tolerance: config.singularity_tolerance,
                reach: Some(config.singularity_reach * edge_length),
            };
            field = singularity::create(singularity::Kind::PairCancellation).optimize(&work, &field, &request)?.0;
        }
        let (positive, negative, _) = field.singularities();
        report.singularities = (positive, negative);
        lap(&mut report, "singularities");

        let map_request = parametrization::Request { edge_length, stiffening_rounds: self.stiffening_rounds };
        let map =
            parametrization::create(parametrization::Kind::IntegerGrid).parametrize(&work, &field, &map_request)?;
        report.map_inversions = map.report.flipped;
        lap(&mut report, "parametrization");

        let mut graph = graph_from_parametrization(&work.mesh, &work.normals, &map.uv);
        graph.remove_spurious_vertices();
        graph.prune_dangling_edges();
        graph.sort_counter_clockwise();
        let extraction = extract_faces(&graph, 8);
        report.open_loops = extraction.open_loops;
        let mut mesh = extraction.mesh;
        mesh.dissolve_valence_two();
        mesh.repair_to_quads();
        lap(&mut report, "extraction");

        // Hard edges and corners of the work mesh are what the final vertices
        // snap to; the surface itself is the input.
        let constraints =
            crate::field::Constraints::from_feature_edges(&work.mesh.positions, &work.normals, &work.feature_edges);
        let guide = Features::new(&work.mesh.positions, &work.feature_edges, &constraints, 0.35 * edge_length);
        relax(&mut mesh, &Bvh::build(input), guide.as_ref(), self.relax_iterations);
        lap(&mut report, "relaxation");

        report.stats = mesh.stats();
        Ok((mesh, report))
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
        let config =
            RemeshConfig { method: Method::InstantMeshes, target: Target::QuadCount(400), ..RemeshConfig::default() };
        let (via_factory, _) = config.remesh(&source).unwrap();
        let params = Params { target: Target::QuadCount(400), ..Default::default() };
        let (direct, _) = pipeline::remesh(&source, &params).unwrap();
        assert_eq!(via_factory.faces, direct.faces);
        assert_eq!(via_factory.positions, direct.positions);
    }
}
