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
use crate::meshing;
use crate::parametrization;
use crate::pipeline::{self, Params, Report, Target};
use crate::preprocess::{self, WorkMesh};
use crate::quantization;
use crate::relax::{Features, Laplacian, Relaxer, Squaring};
use crate::singularity;
use crate::sizing;
use crate::spatial::Bvh;
use crate::tmesh;

/// What the user drew on the input to steer the result, as opposed to the
/// settings, which do not depend on the model.
#[derive(Clone, Copy, Default)]
pub struct Guides<'a> {
    /// A density painted on the vertices of the input, as one weight each:
    /// 0 for the least dense, 1 for the most. How far apart the two are is
    /// a setting, [`RemeshConfig::density_contrast`]. A vertex that was not
    /// painted is simply at 0.
    pub density: Option<&'a [f64]>,
}

/// A complete triangle-to-quad remeshing chain.
pub trait Remesher {
    fn method(&self) -> Method;

    fn remesh(&self, input: &TriMesh, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
        self.remesh_guided(input, &Guides::default(), config)
    }

    /// The same, steered by `guides`. A chain is free to ignore those it has
    /// no use for.
    fn remesh_guided(&self, input: &TriMesh, guides: &Guides, config: &RemeshConfig) -> Result<(PolyMesh, Report)>;
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
    /// Cross field, singularity simplification, seamless map, T-mesh,
    /// quantization, and one grid of quads per rectangle of the T-mesh.
    QuadLayout,
}

impl Method {
    pub const ALL: &[Method] = &[Method::InstantMeshes, Method::IntegerGrid, Method::QuadLayout];

    pub fn name(self) -> &'static str {
        match self {
            Method::InstantMeshes => "instant-meshes",
            Method::IntegerGrid => "integer-grid",
            Method::QuadLayout => "quad-layout",
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
        Method::QuadLayout => Box::new(QuadLayout::default()),
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

    fn remesh_guided(&self, input: &TriMesh, _: &Guides, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
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

    fn remesh_guided(&self, input: &TriMesh, _: &Guides, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
        let mut clock = Stopwatch::start();
        let mut report = Report::default();
        let size = WorkSize { per_quad: self.triangles_per_quad, least: self.min_triangles, most: self.max_triangles };
        // Its extraction counts on quads of one size: no sizing.
        let Fields { work, field, edge_length, .. } = fields(input, config, &size, None, &mut report, &mut clock)?;

        let map_request = parametrization::Request { edge_length, stiffening_rounds: self.stiffening_rounds };
        let map =
            parametrization::create(parametrization::Kind::IntegerGrid).parametrize(&work, &field, &map_request)?;
        report.map_inversions = map.report.flipped;
        clock.lap(&mut report, "parametrization");

        let mut graph = graph_from_parametrization(&work.mesh, &work.normals, &map.uv);
        graph.remove_spurious_vertices();
        graph.prune_dangling_edges();
        graph.sort_counter_clockwise();
        let extraction = extract_faces(&graph, 8);
        report.open_loops = extraction.open_loops;
        let mut mesh = extraction.mesh;
        mesh.dissolve_valence_two();
        mesh.repair_to_quads();
        clock.lap(&mut report, "extraction");

        settle(&mut mesh, input, &work, edge_length, None, &Laplacian { iterations: self.relax_iterations });
        clock.lap(&mut report, "relaxation");

        report.stats = mesh.stats();
        Ok((mesh, report))
    }
}

/// Tuning of the quad-layout chain: what the configuration does not cover.
#[derive(Clone, Debug)]
pub struct QuadLayout {
    /// Triangles of the work mesh per quad asked for, and the bounds it is
    /// kept within.
    pub triangles_per_quad: f64,
    pub min_triangles: usize,
    pub max_triangles: usize,
    /// How close to the number of quads asked for is close enough.
    pub budget_tolerance: f64,
    /// Final placement of the vertices.
    pub relaxer: Squaring,
}

impl Default for QuadLayout {
    fn default() -> Self {
        Self {
            triangles_per_quad: 10.0,
            min_triangles: 20_000,
            max_triangles: 120_000,
            budget_tolerance: 0.02,
            relaxer: Squaring::default(),
        }
    }
}

impl Remesher for QuadLayout {
    fn method(&self) -> Method {
        Method::QuadLayout
    }

    fn remesh_guided(&self, input: &TriMesh, guides: &Guides, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
        let mut clock = Stopwatch::start();
        let mut report = Report::default();
        let size = WorkSize { per_quad: self.triangles_per_quad, least: self.min_triangles, most: self.max_triangles };
        let Fields { work, field, edge_length, quads } =
            fields(input, config, &size, Some(guides), &mut report, &mut clock)?;

        let layout = tmesh::create(tmesh::Kind::Motorcycle).build(&work, &field, &tmesh::Request { edge_length })?;
        report.map_inversions = layout.tmesh.report.map_inversions;
        clock.lap(&mut report, "layout");
        // A piece of surface with neither singularity nor border (a torus)
        // has no layout; a layout that went wrong cannot be meshed from, nor
        // can vertices be read off a map that folds over. In all those
        // cases the provisional chain takes over.
        let layout_report = &layout.tmesh.report;
        let unusable =
            layout_report.uncut_triangles > 0 || layout_report.map_inversions > 0 || !layout_report.is_valid();
        if layout.tmesh.patches.is_empty() || unusable {
            return fall_back(input, config);
        }

        let request =
            quantization::Request { quads: Some((quads.round() as usize).max(1)), tolerance: self.budget_tolerance };
        let quantized = quantization::create(quantization::Kind::WalkCancelling).quantize(&layout.tmesh, &request)?;
        clock.lap(&mut report, "quantization");
        if !quantized.report.is_valid() || quantized.report.quads == 0 {
            return fall_back(input, config);
        }

        let (mut mesh, meshed) = meshing::create(meshing::Kind::PatchGrid).mesh(&work, &layout, &quantized)?;
        clock.lap(&mut report, "meshing");

        // The layout knows which vertices lie on a border or a hard edge.
        let held = Some((meshed.on_feature, meshed.on_corner));
        settle(&mut mesh, input, &work, edge_length, held, &self.relaxer);
        clock.lap(&mut report, "relaxation");

        report.stats = mesh.stats();
        Ok((mesh, report))
    }
}

/// Hands the job to the integer-grid chain, and says so in the report.
fn fall_back(input: &TriMesh, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
    let (mesh, mut report) = IntegerGrid::default().remesh(input, config)?;
    report.fell_back = true;
    Ok((mesh, report))
}

/// How large a work mesh a chain computes on.
struct WorkSize {
    per_quad: f64,
    least: usize,
    most: usize,
}

/// What the stages common to the field-based chains produce.
struct Fields {
    work: WorkMesh,
    field: crossfield::CrossField,
    /// Edge length of the quads to come, and how many that makes.
    edge_length: f64,
    quads: f64,
}

/// Preprocessing, sizing, cross field and singularity simplification.
/// `guides` is given by the chains whose quads may vary in size.
fn fields(
    input: &TriMesh,
    config: &RemeshConfig,
    size: &WorkSize,
    guides: Option<&Guides>,
    report: &mut Report,
    clock: &mut Stopwatch,
) -> Result<Fields> {
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
    report.edge_length = edge_length;

    let triangles = ((size.per_quad * quads) as usize).clamp(size.least, size.most);
    let work_request =
        preprocess::Request { triangles, crease_angle: config.crease_angle, curvature_scale: Some(edge_length) };
    let mut work = preprocess::create(preprocess::Kind::Isotropic).preprocess(input, &work_request)?;
    report.working_vertices = work.mesh.positions.len();
    report.feature_edges = work.feature_edges.len();
    clock.lap(report, "preprocess");

    if let Some(guides) = guides {
        let painted =
            guides.density.map(|weights| sizing::Painted { source: input, weights, contrast: config.density_contrast });
        let request = sizing::Request { edge_length, adaptivity: config.adaptivity, painted };
        work.sizing = sizing::create(sizing::Kind::Curvature).size(&work, &request)?;
        clock.lap(report, "sizing");
    }

    let field_request = crossfield::Request { alignment: config.alignment, seed: config.seed };
    let mut field = crossfield::create(crossfield::Kind::Power).solve(&work, &field_request)?;
    clock.lap(report, "cross field");

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
    clock.lap(report, "singularities");
    Ok(Fields { work, field, edge_length, quads })
}

/// Relaxes `mesh` on the input surface. Hard edges and corners of the work
/// mesh are what its vertices snap to; the surface itself is the input.
/// `held` lists the vertices known to lie on feature lines and on corners;
/// without it they are told by distance.
fn settle(
    mesh: &mut PolyMesh,
    input: &TriMesh,
    work: &WorkMesh,
    edge_length: f64,
    held: Option<(Vec<u32>, Vec<u32>)>,
    relaxer: &dyn Relaxer,
) {
    let constraints =
        crate::field::Constraints::from_feature_edges(&work.mesh.positions, &work.normals, &work.feature_edges);
    // A vertex belongs to a feature line when it is within a third of an
    // edge of it: of the shortest edges there are, where sizes vary.
    let finest = work.sizing.iter().copied().fold(1.0, f64::min);
    let mut guide = Features::new(&work.mesh.positions, &work.feature_edges, &constraints, 0.35 * finest * edge_length);
    if let Some((on_lines, on_corners)) = held {
        guide = guide.map(|features| features.with_members(on_lines, on_corners));
    }
    relaxer.relax(mesh, &Bvh::build(input), guide.as_ref());
}

/// Times the stages of a chain.
struct Stopwatch(Instant);

impl Stopwatch {
    fn start() -> Self {
        Self(Instant::now())
    }

    fn lap(&mut self, report: &mut Report, stage: &'static str) {
        report.timings.push((stage, self.0.elapsed()));
        self.0 = Instant::now();
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
