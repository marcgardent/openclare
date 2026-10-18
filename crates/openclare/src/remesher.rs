//! Method-independent entry point.
//!
//! Every remeshing chain (the Instant Meshes baseline, the parametrization
//! chain for organic shapes, the patch chain for hard surfaces later) is a
//! [`Remesher`]. Callers pick one by [`Method`] and get it from [`create`];
//! nothing outside this module names a concrete chain.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, mpsc};
use std::time::Instant;

use crate::config::RemeshConfig;
use crate::creases::{Crease, Creases};
use crate::crossfield;
use crate::error::{Error, Result};
use crate::extract::{extract_faces, grid::graph_from_parametrization};
use crate::fitting;
use crate::math::Vec3;
use crate::mesh::{PolyMesh, TriMesh};
use crate::meshing;
use crate::parametrization;
use crate::pipeline::{self, Params, Report, Target};
use crate::preprocess::{self, GuideCurve, WorkMesh};
use crate::quantization;
use crate::recorder::{self, Recorder, Shape, Silent};
use crate::relax::{Features, Laplacian, Relaxer, Squaring};
use crate::singularity;
use crate::thickness;
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
    /// Curves drawn on the input for the quad mesh to have lines of edges
    /// along: an edge loop round an eye, a line down the middle of a back.
    pub curves: &'a [GuideCurve],
    /// Edges of the input for the quad mesh to have lines of edges along,
    /// each given by where its two ends are.
    pub lines: &'a [[Vec3; 2]],
    /// A group for each triangle of the input: its material, say. Where two
    /// groups meet the quad mesh has a line of edges, so that no face
    /// straddles two of them, and the report says which group each face of
    /// the quad mesh lies in ([`Report::face_groups`]).
    pub groups: Option<&'a [u32]>,
}

/// A piece with less than this share of the area of the model is left out.
const NEGLIGIBLE_AREA: f64 = 1e-12;

/// The edges of `mesh` between two triangles of different `groups`, each by
/// where its two ends are.
fn group_borders(mesh: &TriMesh, groups: &[u32]) -> Vec<[Vec3; 2]> {
    let mut sides: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    for (tri, &group) in mesh.triangles.iter().zip(groups) {
        for k in 0..3 {
            let (a, b) = (tri[k], tri[(k + 1) % 3]);
            let seen = sides.entry((a.min(b), a.max(b))).or_insert((0, group));
            // Two faces of different groups: a border. A third face, or two
            // of the same group, and it is none.
            seen.0 += if seen.0 == 0 || (seen.0 == 1 && seen.1 != group) { 1 } else { 2 };
            if seen.0 == 1 {
                seen.1 = group;
            }
        }
    }
    let mut borders: Vec<(u32, u32)> =
        sides.into_iter().filter(|&(_, (count, _))| count == 2).map(|(edge, _)| edge).collect();
    borders.sort_unstable();
    borders.into_iter().map(|(a, b)| [mesh.positions[a as usize], mesh.positions[b as usize]]).collect()
}

/// A complete triangle-to-quad remeshing chain.
pub trait Remesher {
    fn method(&self) -> Method;

    fn remesh(&self, input: &TriMesh, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
        self.remesh_guided(input, &Guides::default(), config)
    }

    /// The same, steered by `guides`. A chain is free to ignore those it has
    /// no use for.
    fn remesh_guided(&self, input: &TriMesh, guides: &Guides, config: &RemeshConfig) -> Result<(PolyMesh, Report)> {
        self.remesh_recorded(input, guides, config, &mut Silent)
    }

    /// The same, showing `recorder` what each stage makes on the way. What
    /// comes out does not depend on who listens.
    fn remesh_recorded(
        &self,
        input: &TriMesh,
        guides: &Guides,
        config: &RemeshConfig,
        recorder: &mut dyn Recorder,
    ) -> Result<(PolyMesh, Report)>;
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

    fn remesh_recorded(
        &self,
        input: &TriMesh,
        _: &Guides,
        config: &RemeshConfig,
        _: &mut dyn Recorder,
    ) -> Result<(PolyMesh, Report)> {
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

    fn remesh_recorded(
        &self,
        input: &TriMesh,
        _: &Guides,
        config: &RemeshConfig,
        recorder: &mut dyn Recorder,
    ) -> Result<(PolyMesh, Report)> {
        let mut clock = Stopwatch::start();
        let mut report = Report::default();
        let size = WorkSize { per_quad: self.triangles_per_quad, least: self.min_triangles, most: self.max_triangles };
        // Its extraction counts on quads of one size: no sizing.
        let Fields { work, field, edge_length, .. } =
            fields(input, config, &size, None, &mut report, &mut clock, recorder)?;

        recorder.begin("parametrization")?;
        let map_request = parametrization::Request { edge_length, stiffening_rounds: self.stiffening_rounds };
        let map =
            parametrization::create(parametrization::Kind::IntegerGrid).parametrize(&work, &field, &map_request)?;
        report.map_inversions = map.report.flipped;
        clock.lap(&mut report, "parametrization");

        recorder.begin("extraction")?;
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
        recorder.record("extraction/quads", Shape::Polygons(&mesh));

        let relaxer = Laplacian { iterations: self.relax_iterations };
        settle(&mut mesh, input, &work, edge_length, None, &relaxer, config.volume_fit, recorder)?
            .into_report(&mut report);
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

    fn remesh_recorded(
        &self,
        input: &TriMesh,
        guides: &Guides,
        config: &RemeshConfig,
        recorder: &mut dyn Recorder,
    ) -> Result<(PolyMesh, Report)> {
        config.validate()?;
        input.validate()?;
        let expect = |what: &str, given: Option<usize>, wanted: usize, of: &str| match given {
            Some(given) if given != wanted => Err(Error::InvalidParams(format!("{given} {what} for {wanted} {of}"))),
            _ => Ok(()),
        };
        expect("weights painted", guides.density.map(<[f64]>::len), input.positions.len(), "vertices")?;
        expect("groups", guides.groups.map(<[u32]>::len), input.triangles.len(), "triangles")?;

        // Borders torn apart are stitched first: the pieces are those of the
        // surface, not of the way it was written down.
        let mended = input.stitched();
        let (stitched, density, groups) = match &mended {
            Some(mended) => (
                &mended.mesh,
                guides
                    .density
                    .map(|density| mended.vertex_was.iter().map(|&v| density[v as usize]).collect::<Vec<f64>>()),
                guides
                    .groups
                    .map(|groups| mended.triangle_was.iter().map(|&f| groups[f as usize]).collect::<Vec<u32>>()),
            ),
            None => (input, guides.density.map(<[f64]>::to_vec), guides.groups.map(<[u32]>::to_vec)),
        };
        let mut lines = guides.lines.to_vec();
        if let Some(groups) = &groups {
            lines.extend(group_borders(stitched, groups));
        }
        let guides = &Guides { density: density.as_deref(), lines: &lines, groups: None, ..*guides };
        let (mesh, mut report) = self.remesh_stitched(stitched, guides, config, recorder)?;
        if let Some(mended) = &mended {
            (report.welded_vertices, report.edges_taken_apart) = (mended.welded, mended.taken_apart);
        }
        if let Some(groups) = &groups {
            // Each face is of the group of the triangle under its middle.
            // One whose corners, seen from a little inside it, are not all
            // of that group lies across a border between two.
            let surface = Bvh::build(stitched);
            let group_at = |p: Vec3| surface.closest_point(p).map_or(0, |hit| groups[hit.triangle as usize]);
            for face in &mesh.faces {
                let middle = face.iter().map(|&v| mesh.positions[v as usize]).sum::<Vec3>() / face.len() as f64;
                let group = group_at(middle);
                let inside = |&v: &u32| group_at(middle.lerp(mesh.positions[v as usize], 0.8));
                report.faces_across_groups += face.iter().any(|v| inside(v) != group) as usize;
                report.face_groups.push(group);
            }
        }
        Ok((mesh, report))
    }
}

impl QuadLayout {
    /// The chain on a surface whose torn borders are stitched.
    fn remesh_stitched(
        &self,
        input: &TriMesh,
        guides: &Guides,
        config: &RemeshConfig,
        recorder: &mut dyn Recorder,
    ) -> Result<(PolyMesh, Report)> {
        let pieces = input.pieces();
        if pieces.len() <= 1 {
            let (mesh, mut report) = self.remesh_piece(input, guides, config, self.min_triangles, recorder)?;
            report.pieces = 1;
            report.pieces_fell_back = report.fell_back as usize;
            return Ok((mesh, report));
        }

        // Each piece is meshed on its own, with its share of the quads, so
        // that one piece the layout cannot handle does not drag the others
        // to the other chain with it.
        let area = input.surface_area();
        // With sizes asked for, each piece is given them as they are and
        // makes the quads its shape calls for: the shares only tell which
        // pieces are too small to be meshed at all.
        let sized = matches!(config.target, Target::SizeRange { .. });
        let (quads, edge_length) = match config.target {
            Target::QuadCount(0) => return Err(Error::InvalidParams("target quad count is zero".into())),
            Target::QuadCount(count) => (count as f64, (area / count as f64).sqrt()),
            Target::EdgeLength(length) if length.is_finite() && length > 0.0 => (area / (length * length), length),
            Target::EdgeLength(length) => {
                return Err(Error::InvalidParams(format!("edge length {length} is not positive")));
            }
            Target::SizeRange { largest, smallest } => {
                check_sizes(largest, smallest)?;
                (area / (largest * smallest), (largest * smallest).sqrt())
            }
        };
        if let Some(density) = guides.density
            && density.len() != input.positions.len()
        {
            return Err(Error::InvalidParams(format!(
                "{} weights painted for {} vertices",
                density.len(),
                input.positions.len()
            )));
        }
        let mut shares = shares_of(&pieces, guides, config, quads);
        // A piece thin all along is given the quads its thickness calls
        // for, out of the number asked for: the others have that many fewer,
        // and keep half of the number between them whatever is called for.
        let (mut thin_pieces, mut quads_for_thin_pieces) = (0, 0);
        if !sized {
            let needs: Vec<(usize, usize, usize)> = pieces
                .iter()
                .zip(&shares)
                .enumerate()
                .filter_map(|(index, ((piece, _), share))| {
                    let share = (*share)?;
                    quads_for_thickness(piece, share, edge_length, config).map(|needed| (index, share, needed))
                })
                .collect();
            let called_for: usize = needs.iter().map(|&(_, share, needed)| needed - share).sum();
            let others: usize = shares.iter().flatten().sum::<usize>() - needs.iter().map(|&(_, share, _)| share).sum::<usize>();
            let given = called_for.min(others).min((quads / 2.0) as usize);
            if given > 0 {
                let part = given as f64 / called_for as f64;
                let left = 1.0 - given as f64 / others as f64;
                for (index, share) in shares.iter_mut().enumerate() {
                    let Some(share) = share else { continue };
                    match needs.iter().find(|&&(thin, ..)| thin == index) {
                        Some(&(_, had, needed)) => {
                            *share = had + ((needed - had) as f64 * part).round() as usize;
                            let note = format!("thin all along: {share} quads for its thickness, not {had}");
                            recorder::Piece::new(recorder, index, pieces.len()).record("notes/thin", Shape::Note(&note));
                            thin_pieces += 1;
                            quads_for_thin_pieces += *share - had;
                        }
                        None => *share = ((*share as f64 * left).round() as usize).max(1),
                    }
                }
            }
        }

        let mut merged = PolyMesh::default();
        let mut report =
            Report { edge_length, pieces: pieces.len(), thin_pieces, quads_for_thin_pieces, ..Default::default() };
        // One piece through the chain, told to `recorder` under its name.
        let one = |index: usize, share: usize, recorder: &mut dyn Recorder| {
            let (piece, origin) = &pieces[index];
            let density: Option<Vec<f64>> =
                guides.density.map(|density| origin.iter().map(|&v| density[v as usize]).collect());
            let piece_guides = Guides { density: density.as_deref(), ..*guides };
            let target = if sized { config.target } else { Target::QuadCount(share) };
            let piece_config = RemeshConfig { target, ..config.clone() };
            // A work mesh in proportion: a button does not need the
            // triangles a body does.
            let least = ((self.min_triangles as f64 * piece.surface_area() / area) as usize).max(MIN_PIECE_TRIANGLES);
            let mut recorder = recorder::Piece::new(recorder, index, pieces.len());
            self.remesh_piece(piece, &piece_guides, &piece_config, least, &mut recorder)
        };
        let asked: Vec<(usize, usize)> =
            shares.iter().enumerate().filter_map(|(index, share)| share.map(|share| (index, share))).collect();
        report.pieces_skipped = pieces.len() - asked.len();
        let meshed = each_piece(&asked, recorder, one)?;

        let mut first_error = None;
        for (&(index, _), done) in asked.iter().zip(meshed) {
            let mut recorder = recorder::Piece::new(recorder, index, pieces.len());
            // A piece no chain made anything of, and smaller than a quad of
            // the mesh asked for: a screw, a rivet. Its box stands for it,
            // where it would otherwise be missing.
            let failed = match &done {
                Ok((mesh, _)) if mesh.faces.is_empty() => Some("came out with no face".to_string()),
                Err(error) => Some(error.to_string()),
                Ok(_) => None,
            };
            if let Some(why) = &failed
                && let Some(stand_in) = box_of(&pieces[index].0, edge_length)
            {
                let offset = merged.positions.len() as u32;
                merged.positions.extend(stand_in.positions);
                merged.faces.extend(stand_in.faces.into_iter().map(|face| face.into_iter().map(|v| v + offset).collect()));
                report.pieces_boxed += 1;
                recorder.record("notes/boxed", Shape::Note(&format!("its box stands for it: {why}")));
                continue;
            }
            match done {
                // A piece that comes out with no face at all is as good as
                // not meshed.
                Ok((mesh, _)) if mesh.faces.is_empty() => {
                    report.pieces_failed += 1;
                    recorder.record("notes/not meshed", Shape::Note("came out with no face"));
                }
                Ok((mesh, mut done)) => {
                    let offset = merged.positions.len() as u32;
                    let listed = done.hard_edges.iter_mut().chain(done.guide_edges.iter_mut());
                    let listed = listed.chain(done.sharp_curve_edges.iter_mut());
                    for edge in listed.chain(done.creases.iter_mut().map(|(edge, _)| edge)) {
                        *edge = edge.map(|v| v + offset);
                    }
                    merged.positions.extend(mesh.positions);
                    merged
                        .faces
                        .extend(mesh.faces.into_iter().map(|face| face.into_iter().map(|v| v + offset).collect()));
                    report.absorb(done);
                }
                Err(error) => {
                    report.pieces_failed += 1;
                    recorder.record("notes/not meshed", Shape::Note(&error.to_string()));
                    first_error.get_or_insert(error);
                }
            }
        }
        if let (true, Some(error)) = (merged.faces.is_empty(), first_error) {
            return Err(error);
        }
        report.fell_back = report.pieces_fell_back == report.pieces;
        report.stats = merged.stats();
        Ok((merged, report))
    }
}

/// Fewest triangles the work mesh of one piece among several is given.
const MIN_PIECE_TRIANGLES: usize = 1500;

/// Fewest quads a piece is asked for: one for an open sheet; for a closed
/// shape, what it cannot do without, a cube for a ball and four rings of
/// four for anything with a hole through it.
fn least_quads(piece: &TriMesh) -> usize {
    let topology = crate::mesh::Topology::build(piece);
    if !topology.is_closed() {
        return 1;
    }
    let euler = piece.positions.len() as i64 - topology.edge_count() as i64 + piece.triangles.len() as i64;
    if euler >= 2 { 6 } else { 16 }
}

/// The bounding box of a piece as a mesh of six quads, facing out, if the
/// piece is smaller than `edge` from corner to corner: what stands for a
/// piece that small when nothing else could be made of it. A flat piece
/// gives the one quad it lies in, facing the way the piece does.
fn box_of(piece: &TriMesh, edge: f64) -> Option<PolyMesh> {
    let (low, high) =
        piece.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(low, high), &p| (low.min(p), high.max(p)));
    let size = high - low;
    if piece.positions.is_empty() || size.length() >= edge || size.length() <= 0.0 {
        return None;
    }
    let corner = |k: usize| Vec3::new(
        if k & 1 == 0 { low.x } else { high.x },
        if k & 2 == 0 { low.y } else { high.y },
        if k & 4 == 0 { low.z } else { high.z },
    );
    // Each face with its normal pointing out: the corners are numbered by
    // which side they are on along x (1), y (2) and z (4).
    const FACES: [[u32; 4]; 6] =
        [[0, 4, 6, 2], [1, 3, 7, 5], [0, 1, 5, 4], [2, 6, 7, 3], [0, 2, 3, 1], [4, 5, 7, 6]];
    let flat = (0..3).find(|&axis| size.to_array()[axis] <= 1e-9 * size.length());
    let faces: Vec<Vec<u32>> = match flat {
        None => FACES.iter().map(|face| face.to_vec()).collect(),
        Some(axis) => {
            // Its two faces across that axis are one and the same: the one
            // that faces as the piece does is kept.
            let facing: Vec3 = (0..piece.triangles.len()).map(|f| piece.area_vector(f)).fold(Vec3::ZERO, |a, b| a + b);
            let side = if facing.to_array()[axis] < 0.0 { 2 * axis } else { 2 * axis + 1 };
            vec![FACES[side].to_vec()]
        }
    };
    let mut made = PolyMesh::default();
    let mut number = [u32::MAX; 8];
    for face in faces {
        let face = face
            .into_iter()
            .map(|k| {
                // On a flat piece the two sides are at the same place.
                let k = flat.map_or(k, |axis| k & !(1 << axis)) as usize;
                if number[k] == u32::MAX {
                    number[k] = made.positions.len() as u32;
                    made.positions.push(corner(k));
                }
                number[k]
            })
            .collect();
        made.faces.push(face);
    }
    Some(made)
}

/// How many pieces are meshed at a time: as many as the machine has cores,
/// or what `OPENCLARE_THREADS` says.
pub(crate) fn threads() -> usize {
    let asked = std::env::var("OPENCLARE_THREADS").ok().and_then(|text| text.parse::<usize>().ok());
    asked.unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |count| count.get())).max(1)
}

/// What a piece meshed on another thread tells of its stages: passed on to
/// the thread that was asked for the mesh, which alone speaks to whoever
/// follows the run, and which says back when the run is to stop.
struct Relay<'a> {
    stages: mpsc::Sender<String>,
    stopped: &'a AtomicBool,
}

impl Recorder for Relay<'_> {
    fn is_listening(&self) -> bool {
        false
    }

    fn record(&mut self, _: &str, _: Shape<'_>) {}

    fn begin(&mut self, stage: &str) -> Result<()> {
        if self.stopped.load(Ordering::Relaxed) {
            return Err(Error::Cancelled);
        }
        // Nobody to tell any more: the run is ending.
        let _ = self.stages.send(stage.to_string());
        Ok(())
    }
}

/// What the chain makes of one piece.
type Meshed = Result<(PolyMesh, Report)>;

/// Runs `one` on each of the pieces `asked` for (its index and its share of
/// the quads) and gives what each made, in their order. The pieces do not
/// depend on one another: they are meshed several at a time, on as many
/// threads as [`threads`] says, and the result is the same as one after the
/// other. They are taken one after the other when the recorder keeps what
/// the stages show (shapes cannot be passed from a thread to another), and
/// when there is one thread or one piece.
///
/// The stages of every piece are told to `recorder` from this thread, as
/// they come. An error from it stops the run: the pieces under way end at
/// their next stage, and the error is what is given back.
fn each_piece<F>(asked: &[(usize, usize)], recorder: &mut dyn Recorder, one: F) -> Result<Vec<Meshed>>
where
    F: Fn(usize, usize, &mut dyn Recorder) -> Meshed + Sync,
{
    let threads = threads().min(asked.len());
    if threads <= 1 || recorder.is_listening() {
        let mut meshed = Vec::with_capacity(asked.len());
        for &(index, share) in asked {
            let done = one(index, share, recorder);
            if matches!(done, Err(Error::Cancelled)) {
                return Err(Error::Cancelled);
            }
            meshed.push(done);
        }
        return Ok(meshed);
    }

    let slots: Vec<Mutex<Option<Meshed>>> = asked.iter().map(|_| Mutex::new(None)).collect();
    let (next, stopped) = (AtomicUsize::new(0), AtomicBool::new(false));
    let (stages, told) = mpsc::channel();
    let mut refused = None;
    std::thread::scope(|scope| {
        for _ in 0..threads {
            let mut relay = Relay { stages: stages.clone(), stopped: &stopped };
            let (slots, next, one) = (&slots, &next, &one);
            scope.spawn(move || {
                loop {
                    // The largest pieces first: `asked` is in that order.
                    let turn = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&(index, share)) = asked.get(turn) else { break };
                    let done = one(index, share, &mut relay);
                    *slots[turn].lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(done);
                }
            });
        }
        // Only the threads at work hold the channel now: it ends with them.
        drop(stages);
        for stage in told {
            if refused.is_none()
                && let Err(error) = recorder.begin(&stage)
            {
                stopped.store(true, Ordering::Relaxed);
                refused = Some(error);
            }
        }
    });
    if let Some(error) = refused {
        return Err(error);
    }
    Ok(slots
        .into_iter()
        .map(|slot| slot.into_inner().unwrap_or_else(|poisoned| poisoned.into_inner()).unwrap_or(Err(Error::Cancelled)))
        .collect())
}

/// Fewest quads that go round a piece thin all along, whatever the preset
/// says of thin parts: with fewer than four a wire has no section.
const LEAST_AROUND: f64 = 4.0;

/// How many times smaller than the size asked for the quads of a thin piece
/// may get. Thinner than that, a piece keeps quads longer than it is thick:
/// a guard on what a model of hairs would otherwise ask for.
const THIN_PIECE_RANGE: f64 = 16.0;

/// The quads a piece needs for an edge no longer than it is thick, when that
/// is more than the `share` it has: `None` for a piece thick enough for its
/// quads, which is every piece but a ring, a strap, a wire. `nominal` is the
/// edge of the quads asked for over the whole model.
fn quads_for_thickness(piece: &TriMesh, share: usize, nominal: f64, config: &RemeshConfig) -> Option<usize> {
    if share == 0 {
        return None;
    }
    let area = piece.surface_area();
    let around = config.quads_around.max(LEAST_AROUND);
    let edge = (area / share as f64).sqrt();
    let girth = thickness::typical_girth(piece, around * edge);
    let wanted = (girth / around).max(nominal / THIN_PIECE_RANGE);
    // Not thin, or thick beyond the reach (an infinite girth).
    if wanted.is_nan() || wanted >= edge {
        return None;
    }
    let needed = (area / (wanted * wanted)).ceil() as usize;
    (needed > share).then_some(needed)
}

/// How many quads each piece is asked for, out of `quads` in all: in
/// proportion to its area, weighted by the painted density if there is one,
/// and never less than a piece can be meshed with. `None` for a piece that
/// the configuration says to leave out as too small.
fn shares_of(pieces: &[(TriMesh, Vec<u32>)], guides: &Guides, config: &RemeshConfig, quads: f64) -> Vec<Option<usize>> {
    let contrast = config.density_contrast;
    let weights: Vec<f64> = pieces
        .iter()
        .map(|(piece, origin)| match guides.density {
            None => piece.surface_area(),
            // A weight of 1 calls for `contrast` times the quads of a
            // weight of 0, between pieces as within one.
            Some(density) => (0..piece.triangles.len())
                .map(|f| {
                    let painted: f64 = piece.triangles[f].iter().map(|&v| density[origin[v as usize] as usize]).sum();
                    piece.area_vector(f).length() * contrast.powf(painted / 3.0)
                })
                .sum(),
        })
        .collect();
    let total: f64 = weights.iter().sum();
    let fair: Vec<f64> = weights.iter().map(|weight| quads * weight / total.max(f64::MIN_POSITIVE)).collect();
    // A piece with next to no area is no surface to mesh: the sliver two
    // parts leave between them, say. It is left out whatever the setting.
    let area: f64 = pieces.iter().map(|(piece, _)| piece.surface_area()).sum();
    let kept: Vec<bool> = fair
        .iter()
        .zip(pieces)
        .map(|(&share, (piece, _))| share >= config.skip_pieces_under && piece.surface_area() > NEGLIGIBLE_AREA * area)
        .collect();
    // A piece left out has no floor and takes nothing.
    let floors: Vec<usize> =
        pieces.iter().zip(&kept).map(|((piece, _), &kept)| if kept { least_quads(piece) } else { 0 }).collect();
    let mut shares: Vec<usize> = fair
        .iter()
        .zip(&floors)
        .zip(&kept)
        .map(|((&share, &floor), &kept)| if kept { (share.round() as usize).max(floor) } else { 0 })
        .collect();
    // What the small pieces took beyond their due comes off the others, as
    // far as their own floors allow.
    let asked = quads.round() as usize;
    let mut excess = shares.iter().sum::<usize>().saturating_sub(asked);
    let spare: usize = shares.iter().zip(&floors).map(|(share, floor)| share - floor).sum();
    if excess > 0 && spare > 0 {
        let cut = (excess as f64 / spare as f64).min(1.0);
        for (share, &floor) in shares.iter_mut().zip(&floors) {
            let less = (((*share - floor) as f64) * cut).floor() as usize;
            let less = less.min(excess);
            *share -= less;
            excess -= less;
        }
    }
    shares.into_iter().zip(kept).map(|(share, kept)| kept.then_some(share)).collect()
}

impl QuadLayout {
    /// The chain on one connected piece. `least` is the fewest triangles its
    /// work mesh is given.
    fn remesh_piece(
        &self,
        input: &TriMesh,
        guides: &Guides,
        config: &RemeshConfig,
        least: usize,
        recorder: &mut dyn Recorder,
    ) -> Result<(PolyMesh, Report)> {
        let (mesh, report) = self.remesh_piece_once(input, guides, config, least, recorder)?;
        if config.simplicity == crate::config::SIMPLICITY {
            return Ok((mesh, report));
        }
        // Singularities cancelled from further or closer than the presets
        // do give another layout, and one of them now and then folds a quad
        // over (the body of a man at 10 695 quads, with a reach anywhere
        // from 3.2 to 3.7 edges, and not at 3, 3.5 or 4). Where the mesh
        // has a fold, the presets' balance is meshed too, and the mesh with
        // fewer folds kept: simplicity never costs the soundness of the mesh.
        let surface = Bvh::build(input);
        let folded = crate::metrics::folded_faces(input, &surface, &mesh);
        if folded == 0 {
            return Ok((mesh, report));
        }
        let balanced = RemeshConfig { simplicity: crate::config::SIMPLICITY, ..config.clone() };
        let (other, mut other_report) = self.remesh_piece_once(input, guides, &balanced, least, recorder)?;
        if crate::metrics::folded_faces(input, &surface, &other) < folded {
            let note = format!("simplicity left as the presets have it: it folded {folded} quads over");
            recorder.record("notes/simplicity", Shape::Note(&note));
            other_report.pieces_at_preset_simplicity = 1;
            return Ok((other, other_report));
        }
        Ok((mesh, report))
    }

    /// The chain on one connected piece, once.
    fn remesh_piece_once(
        &self,
        input: &TriMesh,
        guides: &Guides,
        config: &RemeshConfig,
        least: usize,
        recorder: &mut dyn Recorder,
    ) -> Result<(PolyMesh, Report)> {
        let mut clock = Stopwatch::start();
        let mut report = Report::default();
        let size = WorkSize { per_quad: self.triangles_per_quad, least, most: self.max_triangles };
        let Fields { work, field, edge_length, quads } =
            fields(input, config, &size, Some(guides), &mut report, &mut clock, recorder)?;

        recorder.begin("layout")?;
        let layout = tmesh::create(tmesh::Kind::Motorcycle).build(&work, &field, &tmesh::Request { edge_length })?;
        report.map_inversions = layout.tmesh.report.map_inversions;
        report.guides_let_go = layout.map.report.guides_off_grid as usize;
        if layout.map.report.guides_off_grid {
            let whole = layout.map.report.loose_guides.len() == work.guide_edges.len();
            report.line_causes = vec![(
                if whole { "the map folds over with them, whole" } else { "the map folds over near part of them" },
                1,
            )];
        }
        clock.lap(&mut report, "layout");
        recorder::show_map(recorder, &work, &layout.map);
        recorder::show_layout(recorder, &work, &layout.tmesh);
        // A piece of surface with neither singularity nor border (a torus)
        // has no layout; a layout that went wrong cannot be meshed from, nor
        // can vertices be read off a map that folds over. In all those
        // cases the provisional chain takes over.
        let layout_report = &layout.tmesh.report;
        let cause = if layout.tmesh.patches.is_empty() || layout_report.uncut_triangles > 0 {
            Some("nothing to cut the surface from (no singularity, border or hard edge)")
        } else if layout_report.map_inversions > 0 {
            Some("the map folds over")
        } else if layout_report.off_grid_edges > 0 {
            Some("a border or hard edge is off the grid lines of the map")
        } else if layout_report.lost_traces > 0 || layout_report.overlaps > 0 {
            Some("a trace got lost")
        } else if !layout_report.is_valid() {
            Some("the layout is not made of rectangles")
        } else {
            None
        };
        if let Some(cause) = cause {
            // With guide curves, the first thing to try is to do without
            // them: the layout without the lines drawn is still better than
            // the other chain.
            if !guides.curves.is_empty() || !guides.lines.is_empty() {
                let free = Guides { curves: &[], lines: &[], ..*guides };
                recorder.record("notes/lines", Shape::Note(&format!("done again without its lines: {cause}")));
                let (mesh, mut report) = self.remesh_piece(input, &free, config, least, recorder)?;
                report.guides_let_go = 1;
                report.line_causes = vec![(cause, 1)];
                return Ok((mesh, report));
            }
            return fall_back(input, config, least, cause, recorder);
        }

        recorder.begin("quantization")?;
        let levers = config.levers();
        let request = quantization::Request {
            quads: Some((quads.round() as usize).max(1)),
            tolerance: self.budget_tolerance,
            close_turns: config.close_loops,
            deviation_growth: levers.deviation_growth,
            deviation_slack: levers.deviation_slack,
        };
        let quantized = quantization::create(quantization::Kind::WalkCancelling).quantize(&layout.tmesh, &request)?;
        clock.lap(&mut report, "quantization");
        recorder::show_lengths(recorder, &work, &layout.tmesh, &quantized);
        if !quantized.report.is_valid() || quantized.report.quads == 0 {
            if !guides.curves.is_empty() || !guides.lines.is_empty() {
                let free = Guides { curves: &[], lines: &[], ..*guides };
                let note = "done again without its lines: no sound whole lengths for the layout";
                recorder.record("notes/lines", Shape::Note(note));
                let (mesh, mut report) = self.remesh_piece(input, &free, config, least, recorder)?;
                report.guides_let_go = 1;
                report.line_causes = vec![("no sound whole lengths for the layout", 1)];
                return Ok((mesh, report));
            }
            return fall_back(input, config, least, "no sound whole lengths for the layout", recorder);
        }

        recorder.begin("meshing")?;
        let (mut mesh, meshed) = meshing::create(meshing::Kind::PatchGrid).mesh(&work, &layout, &quantized)?;
        clock.lap(&mut report, "meshing");
        recorder.record("meshing/quads", Shape::Polygons(&mesh));

        // The layout knows which vertices lie on a border or a hard edge.
        let held = Some((meshed.on_feature, meshed.on_corner));
        let mut lines = settle(&mut mesh, input, &work, edge_length, held, &self.relaxer, config.volume_fit, recorder)?;
        clock.lap(&mut report, "relaxation");

        // Closing the loops round a part may ask a strip for a row of quads
        // it has no room for, which folds over. Where the mesh has a fold,
        // the lengths the plain search found are meshed too, and the mesh
        // with fewer folds kept.
        if quantized.report.turns_held > 0 {
            let surface = Bvh::build(input);
            let folded = crate::metrics::folded_faces(input, &surface, &mesh);
            let plain = quantization::Request { close_turns: false, ..request };
            let plain = match folded {
                0 => None,
                _ => Some(quantization::create(quantization::Kind::WalkCancelling).quantize(&layout.tmesh, &plain)?),
            };
            if let Some(plain) = plain.filter(|plain| plain.report.is_valid() && plain.report.quads > 0) {
                let (mut other, meshed) = meshing::create(meshing::Kind::PatchGrid).mesh(&work, &layout, &plain)?;
                let held = Some((meshed.on_feature, meshed.on_corner));
                let other_lines =
                    settle(&mut other, input, &work, edge_length, held, &self.relaxer, config.volume_fit, recorder)?;
                if crate::metrics::folded_faces(input, &surface, &other) < folded {
                    let note = format!("loops left as they were: closing them folded {folded} quads over");
                    recorder.record("notes/loops", Shape::Note(&note));
                    (mesh, lines) = (other, other_lines);
                }
            }
            clock.lap(&mut report, "folds");
        }
        lines.into_report(&mut report);

        report.stats = mesh.stats();
        Ok((mesh, report))
    }
}

/// Hands the job to the integer-grid chain, and says so in the report.
fn fall_back(
    input: &TriMesh,
    config: &RemeshConfig,
    least: usize,
    cause: &'static str,
    recorder: &mut dyn Recorder,
) -> Result<(PolyMesh, Report)> {
    recorder.record("notes/fallback", Shape::Note(&format!("meshed by the integer-grid chain instead: {cause}")));
    let chain = IntegerGrid { min_triangles: least, ..Default::default() };
    let (mesh, mut report) = chain.remesh_recorded(input, &Guides::default(), config, recorder)?;
    report.fell_back = true;
    report.fallback_causes = vec![(cause, 1)];
    Ok((mesh, report))
}

/// How large a work mesh a chain computes on.
struct WorkSize {
    per_quad: f64,
    least: usize,
    most: usize,
}

/// What a shape calls for, measured once: how many quads any largest and
/// smallest quad make of it is then told at once, without meshing anything.
pub struct Analysis {
    /// One for each piece of the shape.
    pieces: Vec<sizing::Demand>,
    /// For each vertex of the shape, where it lies on the work mesh of its
    /// piece: the piece, a triangle of its work mesh and how much of each
    /// corner. None for a vertex of a piece that was not looked at.
    at_input: Vec<Option<(u32, [u32; 3], [f64; 3])>>,
}

impl Analysis {
    /// Measures `input` under `config`, of which the adaptivity, the quads
    /// round a thin part and the contrast of the paint count, and not the
    /// target: no size is set yet. Curvature and thickness are read at the
    /// scale of the quads of a mesh of a few thousand.
    pub fn of(input: &TriMesh, guides: &Guides, config: &RemeshConfig) -> Result<Self> {
        config.validate()?;
        input.validate()?;
        // (As it is given: borders that the chain would stitch are borders
        // here, which makes a torn shell thinner than it is.)
        let chain = QuadLayout::default();
        let all = input.surface_area();
        let reference = (all / REFERENCE_QUADS).sqrt();
        if !(reference.is_finite() && reference > 0.0) {
            return Err(Error::InvalidMesh("a shape without a surface".into()));
        }
        let mut pieces = Vec::new();
        let mut at_input = vec![None; input.positions.len()];
        for (piece, origin) in input.pieces() {
            let area = piece.surface_area();
            let triangles = ((chain.triangles_per_quad * area / (reference * reference)) as usize)
                .clamp(MIN_PIECE_TRIANGLES, chain.max_triangles);
            let request = preprocess::Request {
                triangles,
                crease_angle: config.crease_angle,
                curvature_scale: Some(reference),
                edge_angle: config.edge_angle,
                ..Default::default()
            };
            // A piece no work mesh can be made of asks for nothing: it will
            // not be meshed either.
            let Ok(work) = preprocess::create(preprocess::Kind::Isotropic).preprocess(&piece, &request) else {
                continue;
            };
            let density: Option<Vec<f64>> = guides
                .density
                .filter(|density| density.len() == input.positions.len())
                .map(|density| origin.iter().map(|&v| density[v as usize]).collect());
            let painted = density.as_deref().map(|weights| sizing::Painted {
                source: &piece,
                weights,
                contrast: config.density_contrast,
            });
            let request = sizing::Request {
                edge_length: reference,
                bounds: None,
                adaptivity: config.adaptivity,
                painted,
                quads_around: config.quads_around,
            };
            let demand = sizing::Demand::measure(&sizing::Curvature::default(), &work, &request)?;
            // Each vertex of the piece reads the work mesh where it is nearest.
            let surface = Bvh::build(&work.mesh);
            let index = pieces.len() as u32;
            for (v, &at) in origin.iter().enumerate() {
                let Some(hit) = surface.closest_point(piece.positions[v]) else { continue };
                let corners = work.mesh.triangles[hit.triangle as usize];
                let shares = barycentric_of(corners.map(|c| work.mesh.positions[c as usize]), hit.point);
                at_input[at as usize] = Some((index, corners, shares));
            }
            pieces.push(demand);
        }
        if pieces.is_empty() {
            return Err(Error::InvalidMesh("no piece of the shape could be looked at".into()));
        }
        Ok(Self { pieces, at_input })
    }

    /// For each vertex of the shape, the edge its quads would have with
    /// quads between `smallest` and `largest`, and whether its shape calls
    /// for smaller ones than the smallest there. A vertex of a piece that
    /// was not looked at has no size (not a number).
    pub fn at_vertices(&self, largest: f64, smallest: f64) -> (Vec<f64>, Vec<bool>) {
        let reading = sizing::Reading::between(largest, smallest);
        let sizes: Vec<Vec<f64>> = self.pieces.iter().map(|piece| piece.sizes(&reading)).collect();
        let asked: Vec<Vec<f64>> = self.pieces.iter().map(|piece| piece.asks_at_vertices(&reading)).collect();
        let read = |values: &[Vec<f64>], (piece, corners, shares): &(u32, [u32; 3], [f64; 3])| -> f64 {
            (0..3).map(|k| shares[k] * values[*piece as usize][corners[k] as usize]).sum()
        };
        self.at_input
            .iter()
            .map(|at| match at {
                Some(at) => (read(&sizes, at), read(&asked, at) < smallest),
                None => (f64::NAN, false),
            })
            .unzip()
    }

    /// The area of the surface.
    pub fn area(&self) -> f64 {
        self.pieces.iter().map(sizing::Demand::area).sum()
    }

    /// How many quads there would be with quads between `smallest` and
    /// `largest`.
    pub fn quads(&self, largest: f64, smallest: f64) -> f64 {
        let reading = sizing::Reading::between(largest, smallest);
        self.pieces.iter().map(|piece| piece.quads(&reading)).sum()
    }

    /// The share of the surface that calls for quads smaller than
    /// `smallest`, with `largest` for the largest.
    pub fn below(&self, largest: f64, smallest: f64) -> f64 {
        let reading = sizing::Reading::between(largest, smallest);
        let under: f64 = self.pieces.iter().map(|piece| piece.below(&reading) * piece.area()).sum();
        under / self.area().max(f64::MIN_POSITIVE)
    }

    /// A largest and a smallest quad to start from. The largest is that of
    /// a mesh of [`PROPOSED_QUADS`] quads of one size: how coarse the whole
    /// is to be is a matter of taste, and of what the mesh is for. The
    /// smallest comes down by steps of [`PROPOSED_STEP`] as long as each
    /// is worth it: as long as it gives what it calls for to at least
    /// [`PROPOSED_YIELD`] of a point of the surface for each per cent of
    /// quads it adds. A surface bumpy all over stops early, a smooth one
    /// with a few tight bends goes down to them. Thin parts come first, when
    /// they are parts: fingers, a wire, under [`PROPOSED_THIN`] of the
    /// surface. The smallest is then no larger than what nineteen twentieths
    /// of their surface calls for. A shape thin over much more of it (a
    /// shell full of beams and rims) is thin by nature, and that is left to
    /// the steps. The smallest is never less than [`PROPOSED_RANGE`] times
    /// less than the largest.
    pub fn proposal(&self) -> (f64, f64) {
        let largest = (self.area() / PROPOSED_QUADS).sqrt();
        let floor = largest / PROPOSED_RANGE;
        let mut smallest = largest;
        let (mut quads, mut below) = (self.quads(largest, smallest), self.below(largest, smallest));
        while smallest / PROPOSED_STEP >= floor {
            let finer = smallest / PROPOSED_STEP;
            let (more, less) = (self.quads(largest, finer), self.below(largest, finer));
            let (gained, cost) = (100.0 * (below - less), 100.0 * (more / quads - 1.0));
            if gained < PROPOSED_YIELD * cost {
                break;
            }
            (smallest, quads, below) = (finer, more, less);
        }
        let mut thin: Vec<(f64, f64)> = self.pieces.iter().flat_map(|piece| piece.thin_parts()).collect();
        thin.sort_by(|a, b| a.0.total_cmp(&b.0));
        let all: f64 = thin.iter().map(|&(_, area)| area).sum();
        if all > PROPOSED_THIN * self.area() {
            thin.clear();
        }
        let mut under = 0.0;
        for (size, area) in thin {
            under += area;
            if under > 0.05 * all {
                smallest = smallest.min(size);
                break;
            }
        }
        (largest, smallest.clamp(floor, largest))
    }
}

/// How much of each corner of a triangle a point of it is made of.
fn barycentric_of([a, b, c]: [Vec3; 3], p: Vec3) -> [f64; 3] {
    let (ab, ac, ap) = (b - a, c - a, p - a);
    let (d00, d01, d11, d20, d21) = (ab.dot(ab), ab.dot(ac), ac.dot(ac), ap.dot(ab), ap.dot(ac));
    let det = d00 * d11 - d01 * d01;
    if det.abs() < 1e-300 {
        return [1.0, 0.0, 0.0];
    }
    let v = (d11 * d20 - d01 * d21) / det;
    let w = (d00 * d21 - d01 * d20) / det;
    [1.0 - v - w, v, w]
}

/// Quads of the mesh at whose scale a shape is looked at before any size is
/// set.
const REFERENCE_QUADS: f64 = 4000.0;
/// What [`Analysis::proposal`] starts from.
pub const PROPOSED_QUADS: f64 = 1500.0;
pub const PROPOSED_STEP: f64 = 1.5;
pub const PROPOSED_YIELD: f64 = 0.25;
pub const PROPOSED_THIN: f64 = 0.1;
pub const PROPOSED_RANGE: f64 = 16.0;

/// The density painted on `input`, if any, as the sizer takes it.
fn painted_of<'a>(input: &'a TriMesh, guides: &Guides<'a>, contrast: f64) -> Option<sizing::Painted<'a>> {
    guides.density.map(|weights| sizing::Painted { source: input, weights, contrast })
}

/// Whether `largest` and `smallest` are the sizes of quads they are given as.
fn check_sizes(largest: f64, smallest: f64) -> Result<()> {
    if smallest.is_finite() && smallest > 0.0 && largest.is_finite() && largest >= smallest {
        return Ok(());
    }
    Err(Error::InvalidParams(format!("quads from {smallest} to {largest}: not a smallest and a largest size")))
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
    recorder: &mut dyn Recorder,
) -> Result<Fields> {
    config.validate()?;
    input.validate()?;
    let area = input.surface_area();
    let (quads, edge_length, bounds) = match config.target {
        Target::QuadCount(0) => return Err(Error::InvalidParams("target quad count is zero".into())),
        Target::QuadCount(count) => (count as f64, (area / count as f64).sqrt(), None),
        Target::EdgeLength(length) if length.is_finite() && length > 0.0 => (area / (length * length), length, None),
        Target::EdgeLength(length) => {
            return Err(Error::InvalidParams(format!("edge length {length} is not positive")));
        }
        // The size is asked for: the shape is looked at first, on a work
        // mesh of its own, for the number of quads that makes. The rest is
        // then done as for that number.
        Target::SizeRange { largest, smallest } => {
            check_sizes(largest, smallest)?;
            recorder.begin("analysis")?;
            let between = (largest * smallest).sqrt();
            let triangles = ((size.per_quad * area / (between * between)) as usize).clamp(size.least, size.most);
            let request = preprocess::Request {
                triangles,
                crease_angle: config.crease_angle,
                curvature_scale: Some(between),
                edge_angle: config.edge_angle,
                ..Default::default()
            };
            let looked_at = preprocess::create(preprocess::Kind::Isotropic).preprocess(input, &request)?;
            let asked = sizing::Request {
                edge_length: between,
                bounds: Some((largest, smallest)),
                adaptivity: config.adaptivity,
                painted: guides.and_then(|guides| painted_of(input, guides, config.density_contrast)),
                quads_around: config.quads_around,
            };
            let demand = sizing::Demand::measure(&sizing::Curvature::default(), &looked_at, &asked)?;
            let reading = sizing::Reading::between(largest, smallest);
            let wanted = demand.quads(&reading).max(1.0);
            // More than may be made: every quad larger, by as much as it takes.
            let most = (config.most_quads.max(1)) as f64;
            let larger = if wanted > most { (wanted / most).sqrt() } else { 1.0 };
            let quads = wanted.min(most);
            (report.largest_edge, report.smallest_edge) = (largest * larger, smallest * larger);
            (report.quads_wanted, report.below_smallest) = (wanted, demand.below(&reading));
            clock.lap(report, "analysis");
            (quads, (area / quads).sqrt(), Some((largest * larger, smallest * larger)))
        }
    };
    report.edge_length = edge_length;

    recorder.begin("preprocessing")?;
    let triangles = ((size.per_quad * quads) as usize).clamp(size.least, size.most);
    let work_request = preprocess::Request {
        triangles,
        crease_angle: config.crease_angle,
        curvature_scale: Some(edge_length),
        curves: guides.map_or(&[], |guides| guides.curves),
        lines: guides.map_or(&[], |guides| guides.lines),
        edge_angle: config.edge_angle,
    };
    let mut work = preprocess::create(preprocess::Kind::Isotropic).preprocess(input, &work_request)?;
    report.working_vertices = work.mesh.positions.len();
    report.feature_edges = work.feature_edges.len();
    report.guide_gaps = work.guide_gaps;
    clock.lap(report, "preprocess");
    recorder::show_work(recorder, &work);

    if let Some(guides) = guides {
        recorder.begin("sizing")?;
        let request = sizing::Request {
            edge_length,
            bounds,
            adaptivity: config.adaptivity,
            painted: painted_of(input, guides, config.density_contrast),
            quads_around: config.quads_around,
        };
        work.sizing = sizing::create(sizing::Kind::Curvature).size(&work, &request)?;
        clock.lap(report, "sizing");
        recorder::show_sizing(recorder, &work);
    }

    recorder.begin("cross field")?;
    let field_request = crossfield::Request { alignment: config.alignment, seed: config.seed };
    let mut field = crossfield::create(crossfield::Kind::Power).solve(&work, &field_request)?;
    clock.lap(report, "cross field");
    recorder::show_field(recorder, "cross-field", &work, &field);

    let reach = config.levers().singularity_reach;
    if reach > 0.0 {
        recorder.begin("singularities")?;
        let request = singularity::Request {
            alignment: config.alignment,
            tolerance: config.singularity_tolerance,
            reach: Some(reach * edge_length),
        };
        field = singularity::create(singularity::Kind::PairCancellation).optimize(&work, &field, &request)?.0;
    }
    let (positive, negative, _) = field.singularities();
    report.singularities = (positive, negative);
    clock.lap(report, "singularities");
    recorder::show_field(recorder, "singularities", &work, &field);
    Ok(Fields { work, field, edge_length, quads })
}

/// Relaxes `mesh` on the input surface, then fits it to that surface by
/// `volume_fit` (see [`fitting`]). Hard edges and corners of the work mesh
/// are what its vertices snap to; the surface itself is the input. `held`
/// lists the vertices known to lie on feature lines and on corners; without
/// it they are told by distance. Returns the edges of the mesh that ended up
/// along a hard edge, those to crease with their weights, and those along a
/// guide curve.
#[allow(clippy::too_many_arguments)]
fn settle(
    mesh: &mut PolyMesh,
    input: &TriMesh,
    work: &WorkMesh,
    edge_length: f64,
    held: Option<(Vec<u32>, Vec<u32>)>,
    relaxer: &dyn Relaxer,
    volume_fit: f64,
    recorder: &mut dyn Recorder,
) -> Result<Lines> {
    let constraints =
        crate::field::Constraints::from_feature_edges(&work.mesh.positions, &work.normals, &work.feature_edges);
    // A vertex belongs to a feature line when it is within a third of an
    // edge of it: of the shortest edges there are, where sizes vary.
    let finest = work.sizing.iter().copied().fold(1.0, f64::min);
    let mut guide = Features::new(&work.mesh.positions, &work.feature_edges, &constraints, 0.35 * finest * edge_length);
    if let Some((on_lines, on_corners)) = held {
        guide = guide.map(|features| features.with_members(on_lines, on_corners));
    }
    recorder.begin("relaxation")?;
    let surface = Bvh::build(input);
    relaxer.relax(mesh, &surface, guide.as_ref());
    if volume_fit > 0.0 {
        recorder.record("relaxation/quads", Shape::Polygons(mesh));
        recorder.begin("fitting")?;
        fitting::create(fitting::Kind::LeastSquares).fit(mesh, input, &surface, guide.as_ref(), volume_fit);
    }
    // A line drawn by the user is a line of edges, not a hard edge, unless
    // it was drawn as one: it is listed apart, and neither marked sharp nor
    // creased for being there.
    let mut hard_edges = guide.map_or_else(Vec::new, |features| features.edges_along(mesh));
    let (mut guide_edges, mut sharp_curve_edges) = (Vec::new(), Vec::new());
    if !work.guide_edges.is_empty() {
        // Each edge along a line is of the kind of the edge of the work
        // mesh nearest to its middle: those lines are what the vertices
        // were held on, where the curves as they were drawn are only near.
        let positions = &work.mesh.positions;
        let lines: Vec<[Vec3; 3]> = work
            .feature_edges
            .iter()
            .map(|&[a, b]| [positions[a as usize], positions[b as usize]])
            .map(|[a, b]| [a, b, b])
            .collect();
        let lines = Bvh::from_triangles(lines);
        let drawn: HashSet<[u32; 2]> = work.guide_edges.iter().copied().collect();
        let sharp: HashSet<[u32; 2]> = work.sharp_guide_edges.iter().copied().collect();
        let of_the_shape: HashSet<[u32; 2]> = work.shape_guide_edges.iter().copied().collect();
        let mut kept = Vec::with_capacity(hard_edges.len());
        for edge in hard_edges {
            let middle = 0.5 * (mesh.positions[edge[0] as usize] + mesh.positions[edge[1] as usize]);
            let nearest = lines.closest_point(middle).map(|hit| work.feature_edges[hit.triangle as usize]);
            match nearest {
                Some(line) if sharp.contains(&line) => {
                    sharp_curve_edges.push(edge);
                    kept.push(edge);
                }
                // A sharp edge of the shape that could be followed is as
                // hard an edge as one that had to be.
                Some(line) if of_the_shape.contains(&line) => kept.push(edge),
                Some(line) if drawn.contains(&line) => guide_edges.push(edge),
                _ => kept.push(edge),
            }
        }
        hard_edges = kept;
    }
    let creases = Creases::default().mark(mesh, input, &surface, &hard_edges);
    Ok(Lines { hard_edges, creases, guide_edges, sharp_curve_edges })
}

/// The edges of a finished mesh that are worth telling apart.
struct Lines {
    /// Along a hard edge of the input.
    hard_edges: Vec<[u32; 2]>,
    /// To keep sharp under subdivision, with their weights.
    creases: Vec<Crease>,
    /// Along a guide curve that is not drawn as a hard edge.
    guide_edges: Vec<[u32; 2]>,
    /// Those of the hard edges that are along a curve drawn as one.
    sharp_curve_edges: Vec<[u32; 2]>,
}

impl Lines {
    fn into_report(self, report: &mut Report) {
        (report.hard_edges, report.creases, report.guide_edges) = (self.hard_edges, self.creases, self.guide_edges);
        report.sharp_curve_edges = self.sharp_curve_edges;
    }
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
    fn the_box_of_a_small_piece_faces_out_and_a_flat_one_gives_one_quad() {
        use crate::mesh::primitives;
        // A cube two across, squashed to 2 by 1 by 0.5.
        let mut piece = primitives::cube(2);
        for p in &mut piece.positions {
            *p = Vec3::new(p.x, 0.5 * p.y, 0.25 * p.z);
        }
        assert!(box_of(&piece, 2.0).is_none(), "larger than a quad: no box stands for it");
        let stand_in = box_of(&piece, 3.0).unwrap();
        assert_eq!((stand_in.positions.len(), stand_in.faces.len()), (8, 6));
        // Facing out: the volume its faces enclose is that of the box.
        let volume: f64 = (stand_in.faces.iter())
            .map(|face| {
                let [a, b, c, d] = [0, 1, 2, 3].map(|k| stand_in.positions[face[k] as usize]);
                (a.dot(b.cross(c)) + a.dot(c.cross(d))) / 6.0
            })
            .sum();
        assert!((volume - 2.0 * 1.0 * 0.5).abs() < 1e-9, "volume {volume}");

        // A flat sheet facing up: the one quad it lies in, facing up.
        let sheet = primitives::plane(2);
        let stand_in = box_of(&sheet, 10.0).unwrap();
        assert_eq!((stand_in.positions.len(), stand_in.faces.len()), (4, 1));
        let [a, b, c] = [0, 1, 2].map(|k| stand_in.positions[stand_in.faces[0][k] as usize]);
        let up = sheet.area_vector(0).normalize();
        assert!((b - a).cross(c - a).normalize().dot(up) > 0.99);
    }

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
