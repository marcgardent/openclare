//! Preprocessing (stage M1): from the raw input to the *work mesh*, the
//! well-shaped, moderately sized triangle mesh every later stage computes on,
//! together with its curvature.
//!
//! The stage is a [`Preprocessor`]; chains get one from [`create`] and never
//! name a concrete implementation.

mod decimate;
mod dynamic_mesh;
mod imprint;
mod isotropic;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use crate::curvature::{Curvature, TensorField};
use crate::error::{Error, Result};
use crate::geometry::{feature_edges, vertex_areas, vertex_normals};
use crate::math::{INVALID, Vec3};
use crate::mesh::{Topology, TriMesh};
use crate::spatial::Bvh;
use dynamic_mesh::DynamicMesh;
pub use imprint::GuideCurve;

/// What the solvers work on.
#[derive(Clone, Debug)]
pub struct WorkMesh {
    pub mesh: TriMesh,
    pub normals: Vec<Vec3>,
    pub areas: Vec<f64>,
    /// Edges the output must follow: creases, open boundaries and guide
    /// curves.
    pub feature_edges: Vec<[u32; 2]>,
    /// The guide curves as they were laid on the surface, stretch by
    /// stretch: what tells a line drawn by the user from a hard edge.
    pub guide_lines: Vec<[Vec3; 2]>,
    /// Places where a guide curve could not be laid as one unbroken line.
    pub guide_gaps: usize,
    /// Those of the feature edges that are along a guide curve: lines to
    /// follow if they can be, where a crease or a border must be.
    pub guide_edges: Vec<[u32; 2]>,
    /// Those of the guide edges that are along a curve drawn as a hard edge:
    /// the edges of the output along them are to come out as hard edges,
    /// not only as a line of edges.
    pub sharp_guide_edges: Vec<[u32; 2]>,
    /// Principal curvatures per vertex, read at [`Request::curvature_scale`].
    pub curvature: Curvature,
    /// How much the principal directions can be trusted, per vertex: 0 where
    /// the surface is flat or equally curved every way, towards 1 where one
    /// direction clearly dominates.
    pub confidence: Vec<f64>,
    /// Factor on the edge length asked for, per vertex: below 1 where quads
    /// should be smaller. All ones unless a chain sets it (see
    /// [`crate::sizing`]).
    pub sizing: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Request<'a> {
    /// Size of the work mesh.
    pub triangles: usize,
    /// Dihedral angle in degrees above which an edge is a crease to keep.
    /// Open boundaries are always kept.
    pub crease_angle: Option<f64>,
    /// Length over which curvature is averaged; typically the edge length of
    /// the quads to come, since smaller detail cannot be followed anyway.
    /// `None` reads it at the scale of the work mesh itself.
    pub curvature_scale: Option<f64>,
    /// Curves drawn on the surface, to keep as lines of edges.
    pub curves: &'a [GuideCurve],
    /// Edges of the source to keep as lines of edges, each given by where
    /// its two ends are. Those that are no edge of the source are ignored.
    pub lines: &'a [[Vec3; 2]],
}

impl Default for Request<'_> {
    fn default() -> Self {
        Self { triangles: 100_000, crease_angle: None, curvature_scale: None, curves: &[], lines: &[] }
    }
}

pub trait Preprocessor {
    fn kind(&self) -> Kind;
    fn preprocess(&self, source: &TriMesh, request: &Request) -> Result<WorkMesh>;
}

/// The preprocessors [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Clean, decimate if far too dense, remesh isotropically.
    Isotropic,
    /// Clean only: for inputs that are already good work meshes.
    Passthrough,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Isotropic, Kind::Passthrough];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Isotropic => "isotropic",
            Kind::Passthrough => "passthrough",
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
            Error::InvalidParams(format!("unknown preprocessor '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the preprocessor of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn Preprocessor> {
    match kind {
        Kind::Isotropic => Box::new(Isotropic::default()),
        Kind::Passthrough => Box::new(Passthrough),
    }
}

/// Decimation by quadric error where the input is far denser than needed,
/// then isotropic remeshing to the requested size.
#[derive(Clone, Debug)]
pub struct Isotropic {
    pub iterations: usize,
    /// Inputs with more than this many times the requested triangles are
    /// first decimated down to that many times.
    pub decimate_above: f64,
}

impl Default for Isotropic {
    fn default() -> Self {
        Self { iterations: 6, decimate_above: 2.0 }
    }
}

/// The work mesh is never let grow beyond this many times the triangles
/// asked for, or those it starts from if there are more. Splitting comes
/// before collapsing, so a source of long thin triangles goes well above
/// the count asked for on its way to it: the bound has to leave room for
/// that, and only stop what would never come back down.
const MOST_TRIANGLES: usize = 32;

impl Preprocessor for Isotropic {
    fn kind(&self) -> Kind {
        Kind::Isotropic
    }

    fn preprocess(&self, source: &TriMesh, request: &Request) -> Result<WorkMesh> {
        if request.triangles == 0 {
            return Err(Error::InvalidParams("work mesh size is zero".into()));
        }
        let source = cleaned(source)?;
        let features = source_features(&source, request);
        let surface = Bvh::build(&source);
        let face_normals: Vec<Vec3> =
            (0..source.triangles.len()).map(|f| source.area_vector(f).normalize_or_zero()).collect();
        let mut mesh = DynamicMesh::new(&source, &features);
        let ceiling = (self.decimate_above * request.triangles as f64) as usize;
        if mesh.triangle_count() > ceiling {
            decimate::decimate(&mut mesh, ceiling);
        }
        // Guide curves go in once the mesh is light, and before it is evened
        // out, which then treats them as it does creases.
        let laid = imprint::imprint(&mut mesh, &surface, &face_normals, request.curves);

        // What the vertices of a line are held on: the creases of the source
        // and the curves as laid.
        let mut segments: Vec<[Vec3; 3]> = features
            .iter()
            .map(|&[a, b]| {
                let (a, b) = (source.positions[a as usize], source.positions[b as usize]);
                [a, b, b]
            })
            .collect();
        segments.extend(laid.segments.iter().map(|&[a, b]| [a, b, b]));
        let lines = (!segments.is_empty()).then(|| Bvh::from_triangles(segments));

        // Edge of the equilateral triangle that tiles the surface `triangles`
        // times, stretched a little: remeshing settles on edges slightly
        // shorter than asked (measured: about 13 % too many triangles without).
        let edge_length = 1.06 * (4.0 * source.surface_area() / (3.0f64.sqrt() * request.triangles as f64)).sqrt();
        let target = isotropic::Target { surface: &surface, face_normals: &face_normals, features: lines.as_ref() };
        // Remeshing settles within a few per cent of the size asked for;
        // dozens of times that is a surface it cannot tile at that length.
        let most = MOST_TRIANGLES * request.triangles.max(mesh.triangle_count());
        isotropic::remesh(&mut mesh, &target, edge_length, self.iterations, most);
        if !laid.segments.is_empty() || !request.lines.is_empty() {
            imprint::unpinch(&mut mesh);
        }

        let (mesh, features) = mesh.finish();
        let mut work = work_mesh(mesh, features, request);
        if !laid.segments.is_empty() || !request.lines.is_empty() {
            // What is kept as a line without being a border or a crease can
            // only come from a curve, or from a line asked for.
            let mut sides: HashMap<(u32, u32), Vec<Vec3>> = HashMap::new();
            for (f, tri) in work.mesh.triangles.iter().enumerate() {
                for k in 0..3 {
                    let (a, b) = (tri[k], tri[(k + 1) % 3]);
                    sides.entry((a.min(b), a.max(b))).or_default().push(work.mesh.area_vector(f).normalize_or_zero());
                }
            }
            let sharp = request.crease_angle.map(|degrees| degrees.to_radians().cos());
            work.guide_edges = work
                .feature_edges
                .iter()
                .copied()
                .filter(|&[a, b]| match sides.get(&(a.min(b), a.max(b))).map(Vec::as_slice) {
                    Some([one, other]) => sharp.is_none_or(|cos| one.dot(*other) > cos),
                    _ => false,
                })
                .collect();
        }
        if laid.sharp.contains(&true) {
            // An edge is of the kind of the stretch nearest to its middle.
            let stretches = Bvh::from_triangles(laid.segments.iter().map(|&[a, b]| [a, b, b]).collect());
            let middle = |[a, b]: [u32; 2]| 0.5 * (work.mesh.positions[a as usize] + work.mesh.positions[b as usize]);
            work.sharp_guide_edges = work
                .guide_edges
                .iter()
                .copied()
                .filter(|&edge| {
                    stretches.closest_point(middle(edge)).is_some_and(|hit| laid.sharp[hit.triangle as usize])
                })
                .collect();
        }
        work.guide_lines = laid.segments;
        work.guide_gaps = laid.gaps;
        Ok(work)
    }
}

/// Leaves the triangulation as it is. Guide curves are not laid.
#[derive(Clone, Copy, Debug)]
pub struct Passthrough;

impl Preprocessor for Passthrough {
    fn kind(&self) -> Kind {
        Kind::Passthrough
    }

    fn preprocess(&self, source: &TriMesh, request: &Request) -> Result<WorkMesh> {
        let mesh = cleaned(source)?;
        let features = source_features(&mesh, request);
        Ok(work_mesh(mesh, features, request))
    }
}

/// Welds coincident vertices and drops degenerate triangles: exported meshes
/// routinely split vertices along UV seams or hard edges, which would leave
/// the surface in disconnected pieces.
///
/// Only vertices on an open border are welded. Those are the ones a seam
/// splits; inside a closed shell there is nothing to stitch, and merging
/// two shells that merely touch would wreck both.
fn cleaned(source: &TriMesh) -> Result<TriMesh> {
    source.validate()?;
    let mut on_border = vec![false; source.positions.len()];
    for h in Topology::build(source).boundary_half_edges() {
        let tri = source.triangles[h / 3];
        on_border[tri[h % 3] as usize] = true;
        on_border[tri[(h + 1) % 3] as usize] = true;
    }

    let (min, max) =
        source.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let cell = ((max - min).length() * 1e-9).max(f64::MIN_POSITIVE);
    let mut first_at: HashMap<[i64; 3], u32> = HashMap::new();
    let remap: Vec<u32> = (0..source.positions.len())
        .map(|v| {
            if !on_border[v] {
                return v as u32;
            }
            let cell_of = ((source.positions[v] - min) / cell).round().to_array().map(|c| c as i64);
            *first_at.entry(cell_of).or_insert(v as u32)
        })
        .collect();
    // Triangles that lost a corner to welding go; flat ones with three
    // distinct corners stay, because removing them would open a slit in the
    // surface (remeshing collapses them properly later).
    let mut compact = vec![u32::MAX; source.positions.len()];
    let mut welded = TriMesh::default();
    for tri in &source.triangles {
        let [a, b, c] = tri.map(|v| remap[v as usize]);
        if a == b || b == c || c == a {
            continue;
        }
        welded.triangles.push([a, b, c].map(|v| {
            if compact[v as usize] == u32::MAX {
                compact[v as usize] = welded.positions.len() as u32;
                welded.positions.push(source.positions[v as usize]);
            }
            compact[v as usize]
        }));
    }
    if welded.surface_area() == 0.0 {
        return Err(Error::InvalidMesh("mesh has no non-degenerate triangle".into()));
    }
    Ok(welded)
}

fn source_features(mesh: &TriMesh, request: &Request) -> Vec<[u32; 2]> {
    let topology = Topology::build(mesh);
    let mut features = feature_edges(mesh, &topology, request.crease_angle.map(f64::to_radians), true);
    if !request.lines.is_empty() {
        // The edges asked for, found by where their ends are: the vertices
        // have other numbers here than where the lines were told.
        let at = |p: Vec3| p.to_array().map(f64::to_bits);
        let asked: HashSet<[[u64; 3]; 2]> =
            request.lines.iter().flat_map(|&[a, b]| [[at(a), at(b)], [at(b), at(a)]]).collect();
        let known: HashSet<(u32, u32)> = features.iter().map(|&[a, b]| (a.min(b), a.max(b))).collect();
        for (h, &opposite) in topology.opposite.iter().enumerate() {
            let tri = mesh.triangles[h / 3];
            let (a, b) = (tri[h % 3], tri[(h + 1) % 3]);
            // Each inner edge once, from the lower of its two half-edges.
            let inner_once = opposite != INVALID && (h as u32) < opposite;
            if inner_once
                && !known.contains(&(a.min(b), a.max(b)))
                && asked.contains(&[at(mesh.positions[a as usize]), at(mesh.positions[b as usize])])
            {
                features.push([a, b]);
            }
        }
    }
    features
}

fn work_mesh(mesh: TriMesh, feature_edges: Vec<[u32; 2]>, request: &Request) -> WorkMesh {
    let normals = vertex_normals(&mesh);
    let areas = vertex_areas(&mesh);
    let topology = Topology::build(&mesh);

    // n smoothing passes reach about sqrt(n) edges.
    let edge = mesh.edge_length_stats().1;
    let (min, max) =
        mesh.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    // A scale larger than a tenth of the piece (a ring asked for six quads)
    // would average the curvature of the whole of it into nothing.
    let widest = 0.1 * (max - min).length();
    let passes =
        request.curvature_scale.map_or(0, |scale| ((scale.min(widest) / edge).powi(2).round() as usize).min(400));
    let curvature = TensorField::estimate(&mesh, &normals, &areas)
        .smoothed(&topology.neighbors, &normals, &areas, passes)
        .principal(&normals);

    // A direction only counts where the surface bends more than the model as
    // a whole does (a sphere of its size): flatter regions have no direction
    // worth following, whatever the ratio of their two tiny curvatures.
    let negligible = 2.0 / (max - min).length().max(f64::MIN_POSITIVE);
    let confidence = (0..mesh.positions.len())
        .map(|v| curvature.anisotropy(v) / (curvature.k1[v].abs() + curvature.k2[v].abs() + negligible))
        .collect();

    let sizing = vec![1.0; mesh.positions.len()];
    WorkMesh {
        mesh,
        normals,
        areas,
        feature_edges,
        guide_lines: Vec::new(),
        guide_gaps: 0,
        guide_edges: Vec::new(),
        sharp_guide_edges: Vec::new(),
        curvature,
        confidence,
        sizing,
    }
}

/// Shape statistics of a triangle mesh.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quality {
    /// Mean over the triangles of their smallest angle, in degrees (60 for
    /// equilateral triangles).
    pub min_angle_mean: f64,
    /// The smallest angle that 99 % of the triangles exceed.
    pub min_angle_p01: f64,
    /// Longest edge over shortest edge.
    pub edge_ratio: f64,
}

pub fn quality(mesh: &TriMesh) -> Quality {
    let mut min_angles: Vec<f64> = (0..mesh.triangles.len())
        .map(|f| {
            let p = mesh.corners(f);
            (0..3)
                .map(|k| {
                    let (a, b) = (p[(k + 1) % 3] - p[k], p[(k + 2) % 3] - p[k]);
                    a.normalize_or_zero().dot(b.normalize_or_zero()).clamp(-1.0, 1.0).acos().to_degrees()
                })
                .fold(f64::INFINITY, f64::min)
        })
        .collect();
    min_angles.sort_unstable_by(f64::total_cmp);
    let (shortest, _, longest) = mesh.edge_length_stats();
    Quality {
        min_angle_mean: min_angles.iter().sum::<f64>() / min_angles.len().max(1) as f64,
        min_angle_p01: min_angles.get(min_angles.len() / 100).copied().unwrap_or(0.0),
        edge_ratio: if shortest > 0.0 { longest / shortest } else { f64::INFINITY },
    }
}
