//! End-to-end remeshing.

use std::time::{Duration, Instant};

use crate::error::{Error, Result};
use crate::extract::{LatticeGraph, extract_faces};
use crate::field::{Constraints, orientation, position};
use crate::geometry::{feature_edges, vertex_areas, vertex_normals};
use crate::hierarchy::{Graph, Hierarchy, Level};
use crate::math::Rng;
use crate::mesh::subdivide::split_long_edges;
use crate::mesh::{MeshStats, PolyMesh, Topology, TriMesh};
use crate::relax::{Features, relax};
use crate::spatial::Bvh;

/// How dense the output should be.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    /// Approximate number of quads.
    QuadCount(usize),
    /// Edge length, in the units of the input mesh.
    EdgeLength(f64),
    /// The largest and the smallest edge a quad may have, in the units of
    /// the input mesh: the shape says which where, and the number of quads
    /// is what that makes (see [`crate::sizing`]).
    SizeRange { largest: f64, smallest: f64 },
}

#[derive(Clone, Debug)]
pub struct Params {
    pub target: Target,
    /// Output only quads, by solving at twice the edge length and splitting
    /// every face around its centre. When false, the raw quad-dominant mesh
    /// is returned (mostly quads, with triangles and pentagons where the
    /// lattice has defects).
    pub pure_quad: bool,
    /// Dihedral angle in degrees above which an input edge is a hard edge the
    /// output must follow. `None` disables crease detection.
    pub crease_angle: Option<f64>,
    /// Make the output follow open boundaries.
    pub align_boundaries: bool,
    /// Gauss-Seidel sweeps per hierarchy level for each field.
    pub orientation_iterations: usize,
    pub position_iterations: usize,
    /// Smoothing passes on the final mesh (each followed by a projection onto
    /// the input surface).
    pub relax_iterations: usize,
    pub seed: u64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            target: Target::QuadCount(2000),
            pure_quad: true,
            crease_angle: None,
            align_boundaries: true,
            orientation_iterations: 6,
            position_iterations: 6,
            relax_iterations: 2,
            seed: 1,
        }
    }
}

/// What happened during a [`remesh`] call.
#[derive(Clone, Debug, Default)]
pub struct Report {
    /// Edge length of the output.
    pub edge_length: f64,
    /// Vertices of the (possibly subdivided) mesh the fields were solved on.
    pub working_vertices: usize,
    pub hierarchy_levels: usize,
    pub feature_edges: usize,
    /// Orientation-field singularities of index +1/4 and -1/4.
    pub singularities: (usize, usize),
    /// Loops of the extracted graph left without a face (see
    /// [`crate::extract::FaceExtraction::open_loops`]).
    pub open_loops: usize,
    /// Triangles left inverted in the parametrization, for the chains that
    /// have one: each is a likely defect in the mesh.
    pub map_inversions: usize,
    /// The chain asked for could not do the job, and another one did.
    pub fell_back: bool,
    /// Separate pieces the input was made of, for the chains that mesh each
    /// on its own; how many of them went to the other chain, and how many
    /// could not be meshed at all and are missing from the output.
    pub pieces: usize,
    pub pieces_fell_back: usize,
    pub pieces_failed: usize,
    /// Pieces left out because their share of the quads was too small (see
    /// [`crate::RemeshConfig::skip_pieces_under`]).
    pub pieces_skipped: usize,
    /// Pieces smaller than a quad that no chain made anything of, and whose
    /// bounding box stands for them in the mesh.
    pub pieces_boxed: usize,
    /// Pieces thin all along that were given more quads than their share
    /// by area, for an edge no longer than they are thick, and how many
    /// quads that moved to them from the other pieces: the number asked
    /// for in all is kept.
    pub thin_pieces: usize,
    pub quads_for_thin_pieces: usize,
    /// When the size of the quads was asked for, between a largest and a
    /// smallest (see [`Target::SizeRange`]): the two edges as they were
    /// used, how many quads the shape calls for between them, and the share
    /// of the surface that would call for quads smaller than the smallest.
    /// The edges are larger than asked when that number is more than
    /// [`crate::RemeshConfig::most_quads`].
    pub largest_edge: f64,
    pub smallest_edge: f64,
    pub quads_wanted: f64,
    pub below_smallest: f64,
    /// Why pieces went to the other chain, with how many for each reason.
    pub fallback_causes: Vec<(&'static str, usize)>,
    /// Why guide lines were let go, wholly or in part, with how many pieces
    /// for each reason.
    pub line_causes: Vec<(&'static str, usize)>,
    /// Edges of the output, as their two vertices, that run along a hard
    /// edge of the input, or along a curve drawn as one.
    pub hard_edges: Vec<[u32; 2]>,
    /// Edges of the output that run along a guide curve, those drawn as
    /// hard edges left out.
    pub guide_edges: Vec<[u32; 2]>,
    /// Those of the hard edges that run along a curve drawn as one.
    pub sharp_curve_edges: Vec<[u32; 2]>,
    /// Every edge of the output that runs along a curve, guide or sharp,
    /// with the index of that curve among those given: what a host marks
    /// as the curve says (sharp, crease, seam).
    pub curve_edges: Vec<([u32; 2], u32)>,
    /// Places where a guide curve could not be laid as one unbroken line.
    pub guide_gaps: usize,
    /// Pieces on which the guide curves could not be kept as lines of edges:
    /// there the edges only run along them, or not at all.
    pub guides_let_go: usize,
    /// Edges of the output that should stay sharp when it is subdivided,
    /// with how sharp, from 0 to 1 (see [`crate::creases`]). The hard edges
    /// are among them, at 1.
    pub creases: Vec<crate::creases::Crease>,
    /// For each face of the output, the group of the input it lies in, when
    /// the input came with groups (see [`crate::Guides::groups`]); empty
    /// otherwise.
    /// What mending the input took, when it had defects: vertices of torn
    /// borders welded to another at the same place, and edges with more
    /// than two faces whose faces were taken apart. Both 0 for an input
    /// left as it was.
    pub welded_vertices: usize,
    pub edges_taken_apart: usize,
    pub face_groups: Vec<u32>,
    /// How many faces of the output lie across a border between two groups
    /// of the input: none where every border is a line of edges.
    pub faces_across_groups: usize,
    /// With a plane of symmetry: how many vertices of the half kept were
    /// put on the plane from farther than 0.6 of the edge of the
    /// quads, where the line of edges on it was let go. 0 when the half
    /// was cut cleanly along it.
    pub off_the_plane: usize,
    pub stats: MeshStats,
    pub timings: Vec<(&'static str, Duration)>,
}

impl Report {
    /// Adds what was done on one piece to what was done on the others.
    pub(crate) fn absorb(&mut self, piece: Report) {
        self.working_vertices += piece.working_vertices;
        self.feature_edges += piece.feature_edges;
        self.singularities.0 += piece.singularities.0;
        self.singularities.1 += piece.singularities.1;
        self.open_loops += piece.open_loops;
        self.map_inversions += piece.map_inversions;
        self.pieces_fell_back += piece.fell_back as usize;
        self.hard_edges.extend(piece.hard_edges);
        self.creases.extend(piece.creases);
        self.guide_edges.extend(piece.guide_edges);
        self.sharp_curve_edges.extend(piece.sharp_curve_edges);
        self.curve_edges.extend(piece.curve_edges);
        self.guide_gaps += piece.guide_gaps;
        self.guides_let_go += piece.guides_let_go;
        // Sizes asked for: the same for every piece unless one had to make
        // its quads larger; the share below the smallest is of all the
        // quads called for.
        if piece.quads_wanted > 0.0 {
            let together = self.quads_wanted + piece.quads_wanted;
            self.below_smallest =
                (self.below_smallest * self.quads_wanted + piece.below_smallest * piece.quads_wanted) / together;
            self.quads_wanted = together;
            self.largest_edge = self.largest_edge.max(piece.largest_edge);
            self.smallest_edge = if self.smallest_edge > 0.0 {
                self.smallest_edge.min(piece.smallest_edge)
            } else {
                piece.smallest_edge
            };
        }
        for (cause, count) in piece.line_causes {
            match self.line_causes.iter_mut().find(|(name, _)| *name == cause) {
                Some((_, known)) => *known += count,
                None => self.line_causes.push((cause, count)),
            }
        }
        for (cause, count) in piece.fallback_causes {
            match self.fallback_causes.iter_mut().find(|(name, _)| *name == cause) {
                Some((_, total)) => *total += count,
                None => self.fallback_causes.push((cause, count)),
            }
        }
        for (stage, time) in piece.timings {
            match self.timings.iter_mut().find(|(name, _)| *name == stage) {
                Some((_, total)) => *total += time,
                None => self.timings.push((stage, time)),
            }
        }
    }
}

/// Remeshes `input` into a field-aligned quad mesh.
pub fn remesh(input: &TriMesh, params: &Params) -> Result<(PolyMesh, Report)> {
    input.validate()?;
    let source = input.cleaned();
    if source.triangles.is_empty() {
        return Err(Error::InvalidMesh("mesh has no non-degenerate triangle".into()));
    }

    let edge_length = match params.target {
        Target::EdgeLength(length) => length,
        // This chain makes quads of one size: between the two.
        Target::SizeRange { largest, smallest } => (largest * smallest).sqrt(),
        Target::QuadCount(0) => return Err(Error::InvalidParams("target quad count is zero".into())),
        Target::QuadCount(count) => (source.surface_area() / count as f64).sqrt(),
    };
    if !(edge_length.is_finite() && edge_length > 0.0) {
        return Err(Error::InvalidParams(format!("edge length {edge_length} is not positive")));
    }
    // The centre split of `pure_quad` halves every edge.
    let scale = if params.pure_quad { 2.0 * edge_length } else { edge_length };

    let mut report = Report { edge_length, ..Default::default() };
    let mut clock = Instant::now();
    let mut lap = |report: &mut Report, stage: &'static str| {
        report.timings.push((stage, clock.elapsed()));
        clock = Instant::now();
    };

    // The lattice is sampled at the vertices: a few of them per cell are
    // needed to recover it.
    let working = split_long_edges(&source, 0.5 * scale);
    let topology = Topology::build(&working);
    let normals = vertex_normals(&working);
    let features =
        feature_edges(&working, &topology, params.crease_angle.map(f64::to_radians), params.align_boundaries);
    let constraints = Constraints::from_feature_edges(&working.positions, &normals, &features);
    let guide = Features::new(&working.positions, &features, &constraints, 0.35 * edge_length);
    report.working_vertices = working.positions.len();
    report.feature_edges = features.len();
    lap(&mut report, "preprocess");

    let finest = Level::new(
        working.positions.clone(),
        normals,
        vertex_areas(&working),
        Graph::uniform(&topology.neighbors),
        constraints,
    );
    let mut hierarchy = Hierarchy::build(finest);
    report.hierarchy_levels = hierarchy.levels.len();
    lap(&mut report, "hierarchy");

    let mut rng = Rng::new(params.seed);
    orientation::solve(&mut hierarchy, params.orientation_iterations, &mut rng);
    let finest = hierarchy.finest();
    let singular = orientation::singularities(&working, &finest.normals, &finest.orientation);
    report.singularities =
        (singular.iter().filter(|s| s.index > 0).count(), singular.iter().filter(|s| s.index < 0).count());
    lap(&mut report, "orientation field");

    position::solve(&mut hierarchy, scale, params.position_iterations, &mut rng);
    lap(&mut report, "position field");

    let mut graph = LatticeGraph::from_fields(hierarchy.finest(), scale);
    graph.remove_spurious_vertices();
    graph.remove_redundant_edges(scale);
    graph.prune_dangling_edges();
    graph.sort_counter_clockwise();
    let extraction = extract_faces(&graph, 8);
    report.open_loops = extraction.open_loops;
    let mut mesh = extraction.mesh;
    mesh.dissolve_valence_two();
    if params.pure_quad {
        mesh = mesh.split_to_quads();
    }
    lap(&mut report, "extraction");

    relax(&mut mesh, &Bvh::build(&source), guide.as_ref(), params.relax_iterations);
    lap(&mut report, "relaxation");

    report.stats = mesh.stats();
    Ok((mesh, report))
}
