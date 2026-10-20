//! Quality measures of a quad mesh against the surface it was made from.
//!
//! This is the yardstick of the project: any remesher's output (ours,
//! Blender's QuadriFlow, anything that writes polygons) goes through
//! [`measure`], so that "better" is a number rather than an impression.

use std::collections::HashMap;
use std::fmt::Write;

use rayon::prelude::*;

use crate::curvature::TensorField;
use crate::geometry::{vertex_areas, vertex_normals};
use crate::math::{INVALID, Vec3};
use crate::mesh::topology::half_edge_vertices;
use crate::mesh::{PolyMesh, Topology, TriMesh};
use crate::spatial::{Bvh, closest_point_on_triangle};

/// Everything [`measure`] reports. Lengths are relative to the diagonal of
/// the source's bounding box, angles are in degrees.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Metrics {
    pub vertices: usize,
    pub faces: usize,
    pub quads: usize,

    // Validity.
    /// Edges shared by more than two faces.
    pub non_manifold_edges: usize,
    /// Edges whose two faces wind in opposite senses.
    pub misoriented_edges: usize,
    pub boundary_edges: usize,
    pub source_boundary_edges: usize,
    /// Faces whose normal opposes the source surface underneath.
    pub inverted_faces: usize,
    /// Faces of (numerically) zero area.
    pub degenerate_faces: usize,
    pub components: usize,
    pub source_components: usize,

    // Singularities.
    /// Interior vertices of valence other than 4.
    pub irregular_vertices: usize,
    /// Fewest irregular vertices the topology allows (`4 |χ|` per closed
    /// component); `None` when the mesh is open or not pure quad.
    pub topological_minimum: Option<usize>,

    // Structure.
    /// Patches cut out by the separatrices traced from every irregular
    /// vertex. Misplaced singularities and spiralling edge loops blow this
    /// up. `None` unless the mesh is a consistently oriented pure quad
    /// manifold.
    pub base_complex_patches: Option<usize>,
    /// The edge loops: how many close, stop, or spiral. `None` unless the
    /// mesh is a consistently oriented manifold.
    pub edge_loops: Option<EdgeLoops>,

    // Alignment.
    /// Mean angle between edges and the nearest principal curvature
    /// direction of the source, weighted by anisotropy: 0 is perfect, 22.5 is
    /// what random edges score. `None` where the source has no preferred
    /// direction (sphere, plane).
    pub alignment: Option<f64>,

    // Quad shape.
    pub angle_deviation_mean: f64,
    pub angle_deviation_p95: f64,
    /// Longest over shortest side.
    pub aspect_mean: f64,
    pub aspect_p95: f64,
    /// Standard deviation of the face areas over their mean.
    pub area_variation: f64,

    // Fidelity.
    /// Two-sided Hausdorff distance (sampled at vertices and face centres).
    pub hausdorff: f64,
    pub mean_distance: f64,
    /// Share of the vertices of the source that sit on a thin part: one the
    /// shape is thinner across than one quad edge.
    pub thin_share: f64,
    /// Mean distance from those vertices to the mesh, in quad edges: how
    /// much of fingers, ears and the like is lost. `None` where the source
    /// has next to nothing that thin.
    pub thin_distance: Option<f64>,
    /// Volume of the mesh over that of the source, both closed.
    pub volume_ratio: Option<f64>,

    // Sharp edges.
    /// Length of the sharp edges of the source that still show at the size
    /// of the quads, in quad edges: see [`SharpEdges`].
    pub feature_edges_length: f64,
    /// Runs of those edges from an end or a junction to the next.
    pub feature_chains: usize,
    /// Share of that length along which an edge of the mesh runs and is
    /// sharp itself. `None` where the source has less than
    /// [`MIN_FEATURE_LENGTH`] quad edges of it.
    pub feature_edges_kept: Option<f64>,

    // Budget.
    /// `|faces - target| / target`.
    pub budget_error: Option<f64>,
}

impl Metrics {
    /// All quads, manifold, consistently wound, no inverted or collapsed
    /// face, and no hole the source does not have.
    pub fn is_valid(&self) -> bool {
        self.faces > 0
            && self.quads == self.faces
            && self.non_manifold_edges == 0
            && self.misoriented_edges == 0
            && self.inverted_faces == 0
            && self.degenerate_faces == 0
            && (self.source_boundary_edges > 0 || self.boundary_edges == 0)
    }

    /// Irregular vertices beyond what the topology imposes.
    pub fn irregular_excess(&self) -> Option<i64> {
        self.topological_minimum.map(|minimum| self.irregular_vertices as i64 - minimum as i64)
    }

    pub fn to_json(&self) -> String {
        fn optional<T: ToString>(value: Option<T>) -> String {
            value.map_or("null".into(), |v| v.to_string())
        }
        let fields: [(&str, String); 30] = [
            ("valid", self.is_valid().to_string()),
            ("vertices", self.vertices.to_string()),
            ("faces", self.faces.to_string()),
            ("quads", self.quads.to_string()),
            ("non_manifold_edges", self.non_manifold_edges.to_string()),
            ("misoriented_edges", self.misoriented_edges.to_string()),
            ("boundary_edges", self.boundary_edges.to_string()),
            ("source_boundary_edges", self.source_boundary_edges.to_string()),
            ("inverted_faces", self.inverted_faces.to_string()),
            ("degenerate_faces", self.degenerate_faces.to_string()),
            ("components", self.components.to_string()),
            ("source_components", self.source_components.to_string()),
            ("irregular_vertices", self.irregular_vertices.to_string()),
            ("topological_minimum", optional(self.topological_minimum)),
            ("irregular_excess", optional(self.irregular_excess())),
            ("base_complex_patches", optional(self.base_complex_patches)),
            ("alignment", optional(self.alignment)),
            ("angle_deviation_mean", self.angle_deviation_mean.to_string()),
            ("angle_deviation_p95", self.angle_deviation_p95.to_string()),
            ("aspect_mean", self.aspect_mean.to_string()),
            ("aspect_p95", self.aspect_p95.to_string()),
            ("area_variation", self.area_variation.to_string()),
            ("hausdorff", self.hausdorff.to_string()),
            ("mean_distance", self.mean_distance.to_string()),
            ("thin_share", self.thin_share.to_string()),
            ("thin_distance", optional(self.thin_distance)),
            ("volume_ratio", optional(self.volume_ratio)),
            ("feature_edges_length", self.feature_edges_length.to_string()),
            ("feature_chains", self.feature_chains.to_string()),
            ("feature_edges_kept", optional(self.feature_edges_kept)),
        ];
        let mut json = String::from("{");
        for (name, value) in &fields {
            write!(json, "\"{name}\": {value}, ").unwrap();
        }
        let loops = self.edge_loops.as_ref();
        let pitch_shares = |l: &EdgeLoops| {
            let shares: Vec<String> = l.helix_pitch_shares.iter().map(f64::to_string).collect();
            format!("[{}]", shares.join(", "))
        };
        let loop_fields: [(&str, String); 15] = [
            ("loops_closed", optional(loops.map(|l| l.closed))),
            ("loops_spiral", optional(loops.map(|l| l.spiral))),
            ("loops_open", optional(loops.map(|l| l.open))),
            ("loops_border", optional(loops.map(|l| l.border))),
            ("loop_edges_closed", optional(loops.map(|l| l.closed_share))),
            ("loop_edges_spiral", optional(loops.map(|l| l.spiral_share))),
            ("loop_edges_open", optional(loops.map(|l| l.open_share))),
            ("loop_edges_border", optional(loops.map(|l| l.border_share))),
            ("longest_open_loop", optional(loops.map(|l| l.longest_open))),
            ("loops_helix", optional(loops.map(|l| l.helix))),
            ("loop_edges_helix", optional(loops.map(|l| l.helix_share))),
            ("loops_helix_missed", optional(loops.map(|l| l.helix_missed))),
            ("loop_edges_helix_missed", optional(loops.map(|l| l.helix_missed_share))),
            ("helix_pitch_max", optional(loops.map(EdgeLoops::helix_pitch_max))),
            ("loop_edges_helix_by_pitch", optional(loops.map(pitch_shares))),
        ];
        for (name, value) in &loop_fields {
            write!(json, "\"{name}\": {value}, ").unwrap();
        }
        write!(json, "\"budget_error\": {}}}", optional(self.budget_error)).unwrap();
        json
    }
}

/// The edge loops of `mesh` alone, without a surface to measure it against:
/// `None` unless it is a consistently oriented manifold. Only its
/// connectivity is read.
pub fn edge_loops(mesh: &PolyMesh) -> Option<EdgeLoops> {
    let connectivity = Connectivity::build(mesh);
    let manifold = connectivity.uses.iter().all(|&(_, count)| count <= 2) && connectivity.misoriented == 0;
    manifold.then(|| connectivity.edge_loops(mesh))
}

/// Measures `mesh` against the `source` surface it approximates.
/// `target_faces` is the face count that was asked for, when known.
pub fn measure(source: &TriMesh, mesh: &PolyMesh, target_faces: Option<usize>) -> Metrics {
    let mut metrics = Metrics {
        vertices: mesh.positions.len(),
        faces: mesh.faces.len(),
        quads: mesh.faces.iter().filter(|f| f.len() == 4).count(),
        budget_error: target_faces.filter(|&n| n > 0).map(|n| (mesh.faces.len() as f64 - n as f64).abs() / n as f64),
        ..Default::default()
    };

    let source_topology = Topology::build(source);
    metrics.source_boundary_edges = source_topology.boundary_half_edges().count();
    metrics.source_components = count_components(source.positions.len(), source.triangles.iter().map(|t| t.as_slice()));
    if mesh.faces.is_empty() || source.triangles.is_empty() {
        return metrics;
    }

    let (min, max) =
        source.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let diagonal = (max - min).length().max(f64::MIN_POSITIVE);
    let surface = Bvh::build(source);

    let connectivity = Connectivity::build(mesh);
    connectivity.validity(mesh, &mut metrics);
    connectivity.singularities(mesh, &mut metrics);
    if metrics.quads == metrics.faces && metrics.non_manifold_edges == 0 && metrics.misoriented_edges == 0 {
        metrics.base_complex_patches = Some(connectivity.base_complex_patches(mesh));
    }
    if metrics.non_manifold_edges == 0 && metrics.misoriented_edges == 0 {
        metrics.edge_loops = Some(connectivity.edge_loops(mesh));
    }

    orientation(source, &surface, mesh, diagonal, &mut metrics);
    let target_edge = match target_faces {
        Some(n) if n > 0 => (source.surface_area() / n as f64).sqrt(),
        _ => mean_edge_length(mesh, &connectivity),
    };
    metrics.alignment = alignment(source, &source_topology, &surface, mesh, &connectivity, target_edge);
    quad_shape(mesh, &mut metrics);
    fidelity(source, &surface, mesh, diagonal, &mut metrics);
    thin_parts(source, mesh, target_edge, &mut metrics);
    if target_edge.is_finite() && target_edge > 0.0 {
        let sharp = SharpEdges::default().measure_with(source, &source_topology, mesh, &connectivity, target_edge);
        metrics.feature_edges_length = sharp.length;
        metrics.feature_chains = sharp.chains;
        metrics.feature_edges_kept = (sharp.length >= MIN_FEATURE_LENGTH).then(|| sharp.kept / sharp.length);
    }
    if metrics.source_boundary_edges == 0 && metrics.boundary_edges == 0 && metrics.non_manifold_edges == 0 {
        let volume = source.signed_volume();
        if volume.abs() > 0.0 {
            metrics.volume_ratio = Some((mesh.triangulated().signed_volume() / volume).abs());
        }
    }
    metrics
}

/// The edge loops of a mesh, each followed straight on through every
/// regular vertex. Each edge is on one loop; the edges of a border are on
/// none. Where the mesh has other faces than quads, a vertex on one stops
/// the loops, and the edges of those faces are on none.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EdgeLoops {
    /// Loops that come back to where they started: rings.
    pub closed: usize,
    /// Loops met again, the same way round, by a strip of quads that leaves
    /// them across, within [`SPIRAL_ORDER`] quads: a helix goes round and
    /// comes back beside itself a few quads further (two or more helices
    /// wound together come back beside one another), where a ring has
    /// another ring. Whether they close or stop, they are counted here only.
    pub spiral: usize,
    /// Loops that stop, at an irregular vertex at one end at least.
    pub open: usize,
    /// Loops that run from a border to a border.
    pub border: usize,
    /// The share of the edges, those of borders left out, on each kind.
    pub closed_share: f64,
    pub spiral_share: f64,
    pub open_share: f64,
    pub border_share: f64,
    /// Edges of the longest loop that stops at an irregular vertex.
    pub longest_open: usize,
    /// Loops that are helices, of any pitch, whatever else they are. The
    /// engine does not choose its meshes by this count, unlike `spiral`.
    ///
    /// A strip of quads leaving a loop across, followed as far as it goes,
    /// may cross the loop again the same way round: there a pass of the loop
    /// lies beside it, that many quads across. One such pass makes no helix,
    /// since a long loop on a large surface comes back near itself somewhere.
    /// A loop is a helix when it lies beside itself along a whole turn: from
    /// an edge to the pass beside it, every edge has a pass of the loop
    /// beside it (one edge being beside another when either is beside the
    /// other). The loop has then gone round at least twice, side by side.
    /// The pass must also be further along the loop than across: a turn
    /// mostly across is the strip's, not the loop's (round a twisted torus,
    /// the rings are met again one ring along, but across the whole tube),
    /// whereas a straight helix round a tube comes back beside itself less
    /// than half as far across as along. A strip that goes round and comes
    /// back to the edge it left is no pass: that is a ring on a torus.
    ///
    /// The pitch of a helix is the fewest quads across among those passes:
    /// a q-helix in the sense of Bommes et al. (2011), going round a turn
    /// being the same as stepping that many quads across. It is exact on a
    /// tube; round a twisted torus, one helix may come back beside itself
    /// only after several turns, and its pitch is then that of the nearest
    /// pass, not the shift of one turn. A loop that comes back beside itself
    /// for less than a turn before it stops is no helix here, where `spiral`
    /// counts it if the pass is close enough.
    pub helix: usize,
    pub helix_share: f64,
    /// Helices that `spiral` misses: of pitch above [`SPIRAL_ORDER`].
    pub helix_missed: usize,
    pub helix_missed_share: f64,
    /// The share of the edges on helices of each pitch, from pitch 1 up to
    /// the largest met.
    pub helix_pitch_shares: Vec<f64>,
}

impl EdgeLoops {
    /// The largest pitch of a helix, 0 without any.
    pub fn helix_pitch_max(&self) -> usize {
        self.helix_pitch_shares.len()
    }
}

/// Most quads a strip leaving a loop across may cross to meet it again for
/// the loop to be a spiral: as many helices wound together. Rings are met
/// again only round a whole torus, or over a cap the other way round.
pub const SPIRAL_ORDER: usize = 4;

/// Fewest thin vertices for their mean distance to mean something.
const THIN_SAMPLES: usize = 20;

/// How well the parts of the source thinner than one quad edge are kept.
fn thin_parts(source: &TriMesh, mesh: &PolyMesh, quad_edge: f64, metrics: &mut Metrics) {
    if !(quad_edge.is_finite() && quad_edge > 0.0) {
        return;
    }
    let normals = crate::geometry::vertex_normals(source);
    let across = crate::thickness::measure(source, &normals, quad_edge).across;
    let output = Bvh::build(&mesh.triangulated());
    let distances: Vec<Option<f64>> =
        source
            .positions
            .par_iter()
            .enumerate()
            .map(|(v, &p)| {
                if across[v] < quad_edge {
                    output.closest_point(p).map(|hit| hit.distance_squared.sqrt())
                } else {
                    None
                }
            })
            .collect();
    let (mut sum, mut count) = (0.0, 0usize);
    for distance in distances.into_iter().flatten() {
        sum += distance;
        count += 1;
    }
    metrics.thin_share = count as f64 / source.positions.len().max(1) as f64;
    if count >= THIN_SAMPLES {
        metrics.thin_distance = Some(sum / count as f64 / quad_edge);
    }
}

/// Fewest quad edges of sharp edges on the source for the share kept to
/// mean something: below, a few stray edges would make the whole mark.
pub const MIN_FEATURE_LENGTH: f64 = 10.0;

/// The sharp edges of a source at the size of the quads, and how much of
/// them a quad mesh keeps.
///
/// A dihedral angle alone cannot tell a machined edge from the grain of a
/// sculpt: both turn sharply from one triangle to the next. Scale tells
/// them apart. A mesh of quads `h` wide can only show a crease that is
/// still there when the surface is looked at `h` wide, so an edge of the
/// source is a feature edge when:
///
/// - its two triangles meet at more than `angle` degrees;
/// - the surface round its middle, within `reach` quad edges, still turns by
///   more than `angle`: cut in two by the plane through the edge halfway
///   between its two triangles, the mean normals of the two halves are that
///   far apart;
/// - that turn happens at the edge, not across the whole width: the two
///   halves of the surface within a quarter of the reach turn by `focus` of
///   it at least. A tube a few quads round turns by as much across `h`, but
///   evenly, and a quad mesh is right to round it;
/// - the line it is on spans `shortest` quad edges at least: a nick or a
///   wart is no edge to follow. Feature edges whose ends are within `gap`
///   quad edges of each other are on one line, since a crease of a sculpt
///   zigzags across the triangles and some of its edges turn by less.
///
/// Two creases side by side, less than `beside` quad edges apart (the two
/// sides of a groove or of a thin plank), are one at the size of the quads,
/// and their length counts once.
///
/// A point of a feature edge is kept when an edge of the quad mesh runs
/// along it: within `tolerance` quad edges, `parallel` degrees at most from
/// its direction, and sharp itself, its two faces meeting at `sharpness` of
/// the angle the source turns by there at least. A mesh that cuts across
/// the edge, or bevels it with two rows of half the angle, does not keep it.
#[derive(Clone, Debug)]
pub struct SharpEdges {
    pub angle: f64,
    pub reach: f64,
    pub focus: f64,
    pub shortest: f64,
    pub gap: f64,
    pub beside: f64,
    pub tolerance: f64,
    pub parallel: f64,
    pub sharpness: f64,
}

impl Default for SharpEdges {
    fn default() -> Self {
        Self {
            angle: 30.0,
            reach: 0.5,
            focus: 0.6,
            shortest: 2.0,
            gap: 0.25,
            beside: 0.5,
            tolerance: 0.25,
            parallel: 30.0,
            sharpness: 0.6,
        }
    }
}

/// What [`SharpEdges::measure`] finds, lengths in quad edges, two creases
/// side by side counted once.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SharpEdgesKept {
    /// Length of the feature edges of the source.
    pub length: f64,
    /// Runs of feature edges from an end or a junction to the next; a ring
    /// with neither is one.
    pub chains: usize,
    /// Length of them that the mesh keeps.
    pub kept: f64,
}

/// An edge of the source that is a feature at the size of the quads, and
/// the angle its surface turns by at that size, in degrees.
struct FeatureEdge {
    ends: [u32; 2],
    angle: f64,
}

impl SharpEdges {
    /// Finds the feature edges of `source` for quads `quad_edge` long, and
    /// how much of them `mesh` keeps.
    pub fn measure(&self, source: &TriMesh, mesh: &PolyMesh, quad_edge: f64) -> SharpEdgesKept {
        self.measure_with(source, &Topology::build(source), mesh, &Connectivity::build(mesh), quad_edge)
    }

    fn measure_with(
        &self,
        source: &TriMesh,
        topology: &Topology,
        mesh: &PolyMesh,
        connectivity: &Connectivity,
        quad_edge: f64,
    ) -> SharpEdgesKept {
        let features = self.features(source, topology, quad_edge);
        let (length, kept) = self.kept(source, &features, mesh, connectivity, quad_edge);
        SharpEdgesKept { length: length / quad_edge, chains: chains(&features), kept: kept / quad_edge }
    }

    fn features(&self, source: &TriMesh, topology: &Topology, quad_edge: f64) -> Vec<FeatureEdge> {
        let normals: Vec<Vec3> =
            (0..source.triangles.len()).map(|t| source.area_vector(t).normalize_or_zero()).collect();
        let cos_angle = self.angle.to_radians().cos();
        // Each inner edge once, from the lower of its two half-edges.
        let creased: Vec<usize> = (0..topology.opposite.len())
            .filter(|&h| {
                let twin = topology.opposite[h];
                twin != INVALID
                    && h < twin as usize
                    && normals[h / 3] != Vec3::ZERO
                    && normals[twin as usize / 3] != Vec3::ZERO
                    && normals[h / 3].dot(normals[twin as usize / 3]) < cos_angle
            })
            .collect();
        let candidates: Vec<FeatureEdge> = creased
            .par_iter()
            .filter_map(|&h| {
                let angle = self.turn_at_scale(source, topology, h, quad_edge)?;
                let (a, b) = half_edge_vertices(source, h);
                Some(FeatureEdge { ends: [a, b], angle })
            })
            .collect();

        // The lines, and the edges on those that span too little dropped.
        let gap = self.gap * quad_edge;
        let mut cells: HashMap<[i64; 3], Vec<(usize, Vec3)>> = HashMap::new();
        for (i, edge) in candidates.iter().enumerate() {
            for v in edge.ends {
                let p = source.positions[v as usize];
                cells.entry(cell_of(p, gap)).or_default().push((i, p));
            }
        }
        let mut lines = Components::new(candidates.len());
        for (i, edge) in candidates.iter().enumerate() {
            for v in edge.ends {
                let p = source.positions[v as usize];
                for cell in cells_around(cell_of(p, gap)) {
                    for &(j, q) in cells.get(&cell).into_iter().flatten() {
                        if p.distance(q) <= gap {
                            lines.union(i as u32, j as u32);
                        }
                    }
                }
            }
        }
        let mut bounds: HashMap<u32, (Vec3, Vec3)> = HashMap::new();
        for (i, edge) in candidates.iter().enumerate() {
            let line = bounds.entry(lines.find(i as u32)).or_insert((Vec3::INFINITY, Vec3::NEG_INFINITY));
            for v in edge.ends {
                line.0 = line.0.min(source.positions[v as usize]);
                line.1 = line.1.max(source.positions[v as usize]);
            }
        }
        let spans = |i: usize| {
            let (low, high) = bounds[&lines.find(i as u32)];
            low.distance(high) >= self.shortest * quad_edge
        };
        let keep: Vec<bool> = (0..candidates.len()).map(spans).collect();
        candidates.into_iter().zip(keep).filter_map(|(edge, keep)| keep.then_some(edge)).collect()
    }

    /// The angle the surface turns by across the edge of half-edge `h`, seen
    /// `reach` quad edges wide, in degrees; `None` unless that is more than
    /// `angle` and happens at the edge.
    fn turn_at_scale(&self, source: &TriMesh, topology: &Topology, h: usize, quad_edge: f64) -> Option<f64> {
        let (a, b) = half_edge_vertices(source, h);
        let (p, q) = (source.positions[a as usize], source.positions[b as usize]);
        let middle = 0.5 * (p + q);
        let along = (q - p).try_normalize()?;
        let sides = [h / 3, topology.opposite[h] as usize / 3];
        let center = |t: usize| source.corners(t).into_iter().sum::<Vec3>() / 3.0;
        // Straight away from the edge into each of its triangles; the plane
        // that cuts the surface in two runs halfway between.
        let away = |t: usize| {
            let to = center(t) - middle;
            (to - along * to.dot(along)).try_normalize()
        };
        let cut = (away(sides[0])? - away(sides[1])?).try_normalize()?;

        let (far, near) = (self.reach * quad_edge, 0.25 * self.reach * quad_edge);
        // Area-weighted normals of each half, within `far` and `near`.
        let mut sums = [[Vec3::ZERO; 2]; 2];
        let mut seen: std::collections::HashSet<usize> = sides.into_iter().collect();
        let mut stack: Vec<usize> = sides.to_vec();
        while let Some(t) = stack.pop() {
            let side = match sides.iter().position(|&s| s == t) {
                Some(side) => side,
                None => (cut.dot(center(t) - middle) < 0.0) as usize,
            };
            let [x, y, z] = source.corners(t);
            let distance = closest_point_on_triangle(middle, x, y, z).distance(middle);
            let area = source.area_vector(t);
            sums[0][side] += area;
            if distance <= near || sides.contains(&t) {
                sums[1][side] += area;
            }
            for k in 0..3 {
                let twin = topology.opposite[3 * t + k];
                if twin == INVALID {
                    continue;
                }
                let next = twin as usize / 3;
                if seen.contains(&next) {
                    continue;
                }
                let [x, y, z] = source.corners(next);
                if closest_point_on_triangle(middle, x, y, z).distance(middle) <= far {
                    seen.insert(next);
                    stack.push(next);
                }
            }
        }
        let angle = |pair: [Vec3; 2]| {
            let (u, v) = (pair[0].try_normalize()?, pair[1].try_normalize()?);
            Some(u.dot(v).clamp(-1.0, 1.0).acos().to_degrees())
        };
        let (wide, close) = (angle(sums[0])?, angle(sums[1])?);
        (wide > self.angle && close >= self.focus * wide).then_some(wide)
    }

    /// The length of the feature edges, those beside a sharper one left
    /// out, and the length of them along which an edge of `mesh` runs, sharp
    /// enough.
    fn kept(
        &self,
        source: &TriMesh,
        features: &[FeatureEdge],
        mesh: &PolyMesh,
        connectivity: &Connectivity,
        quad_edge: f64,
    ) -> (f64, f64) {
        // Points along the feature edges no further apart than the
        // tolerance: where, which way, the length each stands for, and the
        // angle of its edge.
        let spacing = self.tolerance * quad_edge;
        let mut samples: Vec<(Vec3, Vec3, f64, f64)> = Vec::new();
        for feature in features {
            let [p, q] = feature.ends.map(|v| source.positions[v as usize]);
            let Some(along) = (q - p).try_normalize() else { continue };
            let count = (p.distance(q) / spacing).ceil().max(1.0) as usize;
            for i in 0..count {
                let point = p.lerp(q, (i as f64 + 0.5) / count as f64);
                samples.push((point, along, p.distance(q) / count as f64, feature.angle));
            }
        }

        // Which of them an edge of the mesh runs along.
        let cos_parallel = self.parallel.to_radians().cos();
        let edges = self.sharp_edges_of(mesh, connectivity);
        let alone: Vec<bool> = samples
            .par_iter()
            .map(|&(point, along, _, angle)| {
                edges.as_ref().is_some_and(|(edges, shape)| {
                    let runs_along = |s: u32| {
                        let (direction, dihedral) = shape[s as usize];
                        direction.dot(along).abs() >= cos_parallel && dihedral >= self.sharpness * angle
                    };
                    edges.closest_point_where(point, spacing, runs_along).is_some()
                })
            })
            .collect();

        // Two creases side by side, closer than `beside` quad edges, are one
        // at the size of the quads (the two sides of a groove or of a thin
        // plank): a mesh can run along one of them only. The sharpest point
        // first stands for those beside it, and is kept if any of them is.
        let radius = self.beside * quad_edge;
        let mut order: Vec<usize> = (0..samples.len()).collect();
        order.sort_by(|&i, &j| samples[j].3.total_cmp(&samples[i].3).then(i.cmp(&j)));
        let mut standing: HashMap<[i64; 3], Vec<usize>> = HashMap::new();
        let mut kept_by = vec![false; samples.len()];
        let mut owner = vec![usize::MAX; samples.len()];
        for &i in &order {
            let (point, along, _, _) = samples[i];
            let first = cells_around(cell_of(point, radius))
                .flat_map(|key| standing.get(&key).into_iter().flatten().copied())
                .find(|&j| {
                    let (other, direction, _, _) = samples[j];
                    let offset = point - other;
                    let ahead = offset.dot(direction);
                    let across = (offset - direction * ahead).length();
                    // Beside it, not ahead on the same line; and further than
                    // the tolerance, within which a mesh keeps both.
                    direction.dot(along).abs() >= cos_parallel
                        && ahead.abs() <= 0.5 * spacing
                        && across > spacing
                        && across <= radius
                });
            match first {
                Some(j) => owner[i] = j,
                None => {
                    owner[i] = i;
                    standing.entry(cell_of(point, radius)).or_default().push(i);
                }
            }
        }
        for i in 0..samples.len() {
            kept_by[owner[i]] |= alone[i];
        }
        let (mut length, mut kept) = (0.0, 0.0);
        for i in (0..samples.len()).filter(|&i| owner[i] == i) {
            length += samples[i].2;
            if kept_by[i] {
                kept += samples[i].2;
            }
        }
        (length, kept)
    }

    /// The inner edges of `mesh` sharp enough for some feature, as
    /// segments, with their direction and the angle their two faces meet at.
    fn sharp_edges_of(&self, mesh: &PolyMesh, connectivity: &Connectivity) -> Option<(Bvh, Vec<(Vec3, f64)>)> {
        let least = self.sharpness * self.angle;
        let mut segments = Vec::new();
        let mut shape = Vec::new();
        for h in 0..connectivity.origin.len() {
            let twin = connectivity.twin[h];
            if twin == INVALID || h > twin as usize {
                continue;
            }
            let faces = [connectivity.face[h], connectivity.face[twin as usize]];
            let [n, m] = faces.map(|f| area_vector(mesh, &mesh.faces[f as usize]).normalize_or_zero());
            if n == Vec3::ZERO || m == Vec3::ZERO {
                continue;
            }
            let dihedral = n.dot(m).clamp(-1.0, 1.0).acos().to_degrees();
            let (a, b) =
                (mesh.positions[connectivity.origin[h] as usize], mesh.positions[connectivity.head(h as u32) as usize]);
            let Some(direction) = (b - a).try_normalize() else { continue };
            if dihedral >= least {
                segments.push([a, b, b]);
                shape.push((direction, dihedral));
            }
        }
        (!segments.is_empty()).then(|| (Bvh::from_triangles(segments), shape))
    }
}

/// The cell of a grid of cubes `size` wide that `p` is in.
fn cell_of(p: Vec3, size: f64) -> [i64; 3] {
    (p / size).floor().to_array().map(|c| c as i64)
}

/// A cell and the 26 around it.
fn cells_around([x, y, z]: [i64; 3]) -> impl Iterator<Item = [i64; 3]> {
    (-1..=1).flat_map(move |i| (-1..=1).flat_map(move |j| (-1..=1).map(move |k| [x + i, y + j, z + k])))
}

/// Runs of `edges` from an end or a junction to the next, a ring with
/// neither counting as one: each run has two ends at vertices joined to
/// other than two edges, save the rings.
fn chains(edges: &[FeatureEdge]) -> usize {
    let mut degree: HashMap<u32, usize> = HashMap::new();
    for edge in edges {
        for v in edge.ends {
            *degree.entry(v).or_insert(0) += 1;
        }
    }
    let ends: usize = degree.values().filter(|&&d| d != 2).sum();
    // Rings: joined sets whose vertices are all on two edges.
    let mut index: HashMap<u32, u32> = HashMap::new();
    for &v in degree.keys() {
        let next = index.len() as u32;
        index.entry(v).or_insert(next);
    }
    let mut sets = Components::new(index.len());
    for edge in edges {
        sets.union(index[&edge.ends[0]], index[&edge.ends[1]]);
    }
    let mut open: HashMap<u32, bool> = HashMap::new();
    for (v, &d) in &degree {
        *open.entry(sets.find(index[v])).or_insert(false) |= d != 2;
    }
    ends / 2 + open.values().filter(|&&o| !o).count()
}

/// Half-edge view of a polygon mesh. Half-edge `h` leaves `origin[h]` along
/// its face; `twin` is [`INVALID`] on a boundary.
struct Connectivity {
    origin: Vec<u32>,
    next: Vec<u32>,
    face: Vec<u32>,
    twin: Vec<u32>,
    /// Number of faces on each undirected edge, keyed by sorted endpoints.
    uses: Vec<((u32, u32), u32)>,
    /// Undirected edges used twice in the same direction.
    misoriented: usize,
    valence: Vec<u32>,
    on_boundary: Vec<bool>,
}

impl Connectivity {
    fn build(mesh: &PolyMesh) -> Self {
        let (mut origin, mut next, mut face) = (Vec::new(), Vec::new(), Vec::new());
        for (f, polygon) in mesh.faces.iter().enumerate() {
            let base = origin.len();
            for (k, &v) in polygon.iter().enumerate() {
                origin.push(v);
                next.push((base + (k + 1) % polygon.len()) as u32);
                face.push(f as u32);
            }
        }

        let head = |h: usize| origin[next[h] as usize];
        let mut directed: HashMap<(u32, u32), (u32, u32)> = HashMap::with_capacity(origin.len());
        for h in 0..origin.len() {
            let entry = directed.entry((origin[h], head(h))).or_insert((h as u32, 0));
            entry.1 += 1;
        }
        let twin =
            (0..origin.len()).map(|h| directed.get(&(head(h), origin[h])).map_or(INVALID, |&(twin, _)| twin)).collect();

        let uses: Vec<((u32, u32), u32)> = mesh.edge_uses().into_iter().collect();
        let mut valence = vec![0; mesh.positions.len()];
        let mut on_boundary = vec![false; mesh.positions.len()];
        let mut misoriented = 0;
        for &((a, b), count) in &uses {
            valence[a as usize] += 1;
            valence[b as usize] += 1;
            if count == 1 {
                on_boundary[a as usize] = true;
                on_boundary[b as usize] = true;
            }
            let same_way = |key| directed.get(&key).is_some_and(|&(_, n)| n == 2);
            if count == 2 && (same_way((a, b)) || same_way((b, a))) {
                misoriented += 1;
            }
        }
        Self { origin, next, face, twin, uses, misoriented, valence, on_boundary }
    }

    fn head(&self, h: u32) -> u32 {
        self.origin[self.next[h as usize] as usize]
    }

    /// Interior vertices want four edges, boundary ones three.
    fn is_irregular(&self, v: usize) -> bool {
        self.valence[v] != if self.on_boundary[v] { 3 } else { 4 }
    }

    fn validity(&self, mesh: &PolyMesh, metrics: &mut Metrics) {
        metrics.non_manifold_edges = self.uses.iter().filter(|&&(_, count)| count > 2).count();
        metrics.boundary_edges = self.uses.iter().filter(|&&(_, count)| count == 1).count();
        metrics.misoriented_edges = self.misoriented;
        metrics.components = count_components(mesh.positions.len(), mesh.faces.iter().map(Vec::as_slice));
    }

    fn singularities(&self, mesh: &PolyMesh, metrics: &mut Metrics) {
        metrics.irregular_vertices = (0..mesh.positions.len())
            .filter(|&v| self.valence[v] > 0 && !self.on_boundary[v] && self.valence[v] != 4)
            .count();

        // In a closed pure quad mesh the valence defects add up to 4 χ per
        // component, so at least 4 |χ| vertices have to be irregular.
        let closed = metrics.boundary_edges == 0 && metrics.non_manifold_edges == 0;
        if !closed || metrics.quads != metrics.faces {
            return;
        }
        let mut sets = Components::new(mesh.positions.len());
        for &((a, b), _) in &self.uses {
            sets.union(a, b);
        }
        // Euler characteristic V - E + F, accumulated per component.
        let mut euler: HashMap<u32, i64> = HashMap::new();
        for v in 0..mesh.positions.len() {
            if self.valence[v] > 0 {
                *euler.entry(sets.find(v as u32)).or_insert(0) += 1;
            }
        }
        for &((a, _), _) in &self.uses {
            *euler.entry(sets.find(a)).or_insert(0) -= 1;
        }
        for polygon in &mesh.faces {
            *euler.entry(sets.find(polygon[0])).or_insert(0) += 1;
        }
        metrics.topological_minimum = Some(euler.values().map(|chi| 4 * chi.unsigned_abs() as usize).sum());
    }

    /// The half-edge straight on from `h`, through its head: across the
    /// edge between the two faces ahead. None at an irregular vertex, on a
    /// border or on a face that is not a quad (`odd`), where a loop stops.
    fn straight_on(&self, h: u32, odd: &[bool]) -> Option<u32> {
        let v = self.head(h) as usize;
        if self.is_irregular(v) || self.on_boundary[v] || odd[v] {
            return None;
        }
        let across = self.twin[self.next[h as usize] as usize];
        (across != INVALID).then(|| self.next[across as usize])
    }

    /// Follows every edge loop and sorts them: closed, spiral, open (at an
    /// irregular vertex) or from border to border.
    fn edge_loops(&self, mesh: &PolyMesh) -> EdgeLoops {
        let halves = self.origin.len();
        let is_quad: Vec<bool> = mesh.faces.iter().map(|polygon| polygon.len() == 4).collect();
        let mut odd = vec![false; mesh.positions.len()];
        for polygon in mesh.faces.iter().filter(|polygon| polygon.len() != 4) {
            for &v in polygon {
                odd[v as usize] = true;
            }
        }
        let quad_on_both_sides =
            |h: usize| is_quad[self.face[h] as usize] && is_quad[self.face[self.twin[h] as usize] as usize];
        let mut loop_of = vec![INVALID; halves];
        // Of the two halves of each edge on a loop, the one that runs the
        // way the loop was first followed.
        let mut forward = vec![false; halves];
        // Where each edge is along its loop, counted from the start; those
        // met going backward are numbered below zero, then shifted.
        let mut position = vec![0i64; halves];
        // Per loop: its edges, whether it closed, and whether an end is on
        // an irregular vertex.
        let mut loops: Vec<(usize, bool, bool)> = Vec::new();
        let mut behind_start: Vec<i64> = Vec::new();
        let end_on_pole = |h: u32| !self.on_boundary[self.head(h) as usize];
        for start in 0..halves as u32 {
            if loop_of[start as usize] != INVALID
                || self.twin[start as usize] == INVALID
                || !quad_on_both_sides(start as usize)
            {
                continue;
            }
            let id = loops.len() as u32;
            let (mut edges, mut closed, mut pole) = (0usize, false, false);
            let (mut ahead, mut behind) = (0i64, 0i64);
            // Forward from the start; then, if it did not close, backward
            // from beyond its other end.
            let backward = self.twin[start as usize];
            for first in [Some(start), None] {
                let mut h = match first {
                    Some(h) => h,
                    None if closed => break,
                    None => match self.straight_on(backward, &odd) {
                        Some(on) => on,
                        None => {
                            pole |= end_on_pole(backward);
                            break;
                        }
                    },
                };
                loop {
                    if loop_of[h as usize] == id {
                        closed = true;
                        break;
                    }
                    loop_of[h as usize] = id;
                    loop_of[self.twin[h as usize] as usize] = id;
                    let along = if first.is_some() { h } else { self.twin[h as usize] };
                    forward[along as usize] = true;
                    let at = if first.is_some() {
                        ahead += 1;
                        ahead - 1
                    } else {
                        behind += 1;
                        -behind
                    };
                    position[h as usize] = at;
                    position[self.twin[h as usize] as usize] = at;
                    edges += 1;
                    match self.straight_on(h, &odd) {
                        Some(on) => h = on,
                        None => {
                            pole |= end_on_pole(h);
                            break;
                        }
                    }
                }
            }
            loops.push((edges, closed, pole));
            behind_start.push(behind);
        }
        for h in 0..halves {
            if loop_of[h] != INVALID {
                position[h] += behind_start[loop_of[h] as usize];
            }
        }
        // A strip of quads leaving a loop across that meets it again the
        // same way round, within a few quads: that loop spirals. (The edge
        // across a quad from `e` is next(next(e)), running back; its twin
        // runs the same way as `e`.)
        let mut spiral = vec![false; loops.len()];
        for h in 0..halves {
            let id = loop_of[h];
            if id == INVALID || !is_quad[self.face[h] as usize] {
                continue;
            }
            let mut e = h;
            for _ in 0..SPIRAL_ORDER {
                let opposite = self.next[self.next[e] as usize] as usize;
                let beyond = self.twin[opposite];
                if loop_of[opposite] == id {
                    spiral[id as usize] |= beyond != INVALID && forward[beyond as usize] == forward[h];
                    break;
                }
                if beyond == INVALID || !is_quad[self.face[beyond as usize] as usize] {
                    break;
                }
                e = beyond as usize;
            }
        }
        let pitch = self.helices(&is_quad, &loop_of, &forward, &position, &loops);
        let mut counted = EdgeLoops::default();
        let all: usize = loops.iter().map(|l| l.0).sum();
        for (k, &(edges, _, _)) in loops.iter().enumerate() {
            let Some(pitch) = pitch[k] else { continue };
            let share = edges as f64 / all.max(1) as f64;
            counted.helix += 1;
            counted.helix_share += share;
            if !spiral[k] {
                counted.helix_missed += 1;
                counted.helix_missed_share += share;
            }
            if counted.helix_pitch_shares.len() < pitch {
                counted.helix_pitch_shares.resize(pitch, 0.0);
            }
            counted.helix_pitch_shares[pitch - 1] += share;
        }
        for (k, &(edges, closed, pole)) in loops.iter().enumerate() {
            let (count, share) = if spiral[k] {
                (&mut counted.spiral, &mut counted.spiral_share)
            } else if closed {
                (&mut counted.closed, &mut counted.closed_share)
            } else if pole {
                counted.longest_open = counted.longest_open.max(edges);
                (&mut counted.open, &mut counted.open_share)
            } else {
                (&mut counted.border, &mut counted.border_share)
            };
            *count += 1;
            *share += edges as f64 / all.max(1) as f64;
        }
        counted
    }

    /// The pitch of each loop that is a helix, `None` for the others: see
    /// [`EdgeLoops::helix`]. `position` is where each half-edge's edge is
    /// along its loop, from 0.
    fn helices(
        &self,
        is_quad: &[bool],
        loop_of: &[u32],
        forward: &[bool],
        position: &[i64],
        loops: &[(usize, bool, bool)],
    ) -> Vec<Option<usize>> {
        let halves = self.origin.len();
        let in_quad = |h: u32| is_quad[self.face[h as usize] as usize];
        // Across a quad from the half-edge `e` it is entered by, to the half
        // of the next quad on the far side; and back.
        let step = |e: u32| {
            let beyond = self.twin[self.next[self.next[e as usize] as usize] as usize];
            (beyond != INVALID && in_quad(beyond)).then_some(beyond)
        };
        let step_back = |e: u32| {
            let twin = self.twin[e as usize];
            (twin != INVALID && in_quad(twin)).then(|| self.next[self.next[twin as usize] as usize])
        };

        // For each half-edge on a loop, the first edge of the same loop the
        // strip of quads it enters crosses, and how many quads on. Each
        // strip is followed once: the next of each loop along it is found
        // walking it backward, twice round when it closes.
        let mut met: Vec<Option<(usize, u32)>> = vec![None; halves];
        let mut seen = vec![false; halves];
        let mut last = vec![(usize::MAX, 0usize); loops.len()];
        let mut strip = Vec::new();
        for (count, seed) in (0..halves as u32).enumerate() {
            if seen[seed as usize] || !in_quad(seed) {
                continue;
            }
            let (mut first, mut closes) = (seed, false);
            while let Some(before) = step_back(first) {
                if before == seed {
                    closes = true;
                    break;
                }
                first = before;
            }
            strip.clear();
            let mut e = first;
            loop {
                seen[e as usize] = true;
                strip.push(e);
                match step(e) {
                    Some(after) if after != first => e = after,
                    _ => break,
                }
            }
            let n = strip.len();
            for i in (0..if closes { 2 * n } else { n }).rev() {
                let e = strip[i % n];
                let id = loop_of[e as usize];
                if id == INVALID {
                    continue;
                }
                let (stamp, j) = last[id as usize];
                // Round a closed strip back to itself is not a meeting.
                if i < n && stamp == count && j != i + n {
                    met[e as usize] = Some((j - i, strip[j % n]));
                }
                last[id as usize] = (count, i);
            }
        }

        // Per loop and position along it, the fewest quads across to a pass
        // of itself the same way round, and where along the loop that pass is.
        let mut beside: Vec<Vec<Option<(usize, usize)>>> = loops.iter().map(|l| vec![None; l.0]).collect();
        for h in 0..halves {
            let Some((quads, other)) = met[h] else { continue };
            let id = loop_of[h] as usize;
            if forward[other as usize] != forward[h] {
                continue;
            }
            let (edges, closed, _) = loops[id];
            let gap = (position[other as usize] - position[h]).unsigned_abs() as usize;
            let along = if closed { gap.min(edges - gap) } else { gap };
            if along <= quads {
                continue;
            }
            // Each of the two is beside the other.
            let (here, there) = (position[h] as usize, position[other as usize] as usize);
            for (at, pass) in [(here, there), (there, here)] {
                let cell = &mut beside[id][at];
                if cell.is_none_or(|(fewest, _)| quads < fewest) {
                    *cell = Some((quads, pass));
                }
            }
        }

        beside
            .iter()
            .zip(loops)
            .map(|(cells, &(edges, closed, _))| {
                // Positions with a pass beside them before each, twice round
                // for a closed loop.
                let mut before = vec![0usize; 2 * edges + 1];
                for k in 0..2 * edges {
                    before[k + 1] = before[k] + cells[k % edges].is_some() as usize;
                }
                let mut pitch: Option<usize> = None;
                for (here, cell) in cells.iter().enumerate() {
                    let Some((quads, there)) = *cell else { continue };
                    // The turn from here to the pass beside it (the short way
                    // round a closed loop), all of it beside the loop.
                    let (from, length) = if !closed {
                        (here.min(there), here.abs_diff(there))
                    } else if (there + edges - here) % edges <= edges / 2 {
                        (here, (there + edges - here) % edges)
                    } else {
                        (there, (here + edges - there) % edges)
                    };
                    if before[from + length + 1] - before[from] == length + 1 {
                        pitch = Some(pitch.map_or(quads, |p| p.min(quads)));
                    }
                }
                pitch
            })
            .collect()
    }

    /// Traces a separatrix out of every irregular vertex along each of its
    /// edges, going straight through regular vertices until it reaches
    /// another irregular vertex or a boundary, then counts the regions the
    /// separatrices enclose.
    fn base_complex_patches(&self, mesh: &PolyMesh) -> usize {
        let mut separatrix = vec![false; self.origin.len()];
        for start in 0..self.origin.len() as u32 {
            if !self.is_irregular(self.origin[start as usize] as usize) {
                continue;
            }
            let mut h = start;
            // A straight walk is reversible, so it cannot loop without coming
            // back to its irregular start; the bound is only a safety net.
            for _ in 0..self.origin.len() {
                separatrix[h as usize] = true;
                if self.twin[h as usize] != INVALID {
                    separatrix[self.twin[h as usize] as usize] = true;
                }
                let v = self.head(h) as usize;
                if self.is_irregular(v) || self.on_boundary[v] {
                    break;
                }
                // Straight on at a valence-4 vertex: skip one edge.
                let across = self.twin[self.next[h as usize] as usize];
                if across == INVALID {
                    break;
                }
                h = self.next[across as usize];
            }
        }

        let mut patch = vec![INVALID; mesh.faces.len()];
        let mut first_half_edge = Vec::with_capacity(mesh.faces.len());
        let mut offset = 0u32;
        for polygon in &mesh.faces {
            first_half_edge.push(offset);
            offset += polygon.len() as u32;
        }
        let mut patches = 0;
        let mut stack = Vec::new();
        for seed in 0..mesh.faces.len() {
            if patch[seed] != INVALID {
                continue;
            }
            patch[seed] = patches;
            stack.push(seed);
            while let Some(f) = stack.pop() {
                for k in 0..mesh.faces[f].len() as u32 {
                    let h = (first_half_edge[f] + k) as usize;
                    let twin = self.twin[h];
                    if separatrix[h] || twin == INVALID {
                        continue;
                    }
                    let neighbor = self.face[twin as usize] as usize;
                    if patch[neighbor] == INVALID {
                        patch[neighbor] = patches;
                        stack.push(neighbor);
                    }
                }
            }
            patches += 1;
        }
        patches as usize
    }
}

/// Disjoint sets over vertex indices.
struct Components {
    parent: Vec<u32>,
}

impl Components {
    fn new(n: usize) -> Self {
        Self { parent: (0..n as u32).collect() }
    }

    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let grandparent = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = grandparent;
            x = grandparent;
        }
        x
    }

    fn union(&mut self, a: u32, b: u32) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b {
            self.parent[a.max(b) as usize] = a.min(b);
        }
    }
}

/// Connected components among the vertices that some face uses.
fn count_components<'a>(vertex_count: usize, faces: impl Iterator<Item = &'a [u32]>) -> usize {
    let mut sets = Components::new(vertex_count);
    let mut used = vec![false; vertex_count];
    for face in faces {
        for &v in face {
            used[v as usize] = true;
            sets.union(face[0], v);
        }
    }
    (0..vertex_count as u32).filter(|&v| used[v as usize] && sets.find(v) == v).count()
}

/// Area-weighted normal of a polygon (Newell): half the sum of the cross
/// products of consecutive corners.
fn area_vector(mesh: &PolyMesh, face: &[u32]) -> Vec3 {
    let mut sum = Vec3::ZERO;
    for k in 0..face.len() {
        sum += mesh.positions[face[k] as usize].cross(mesh.positions[face[(k + 1) % face.len()] as usize]);
    }
    0.5 * sum
}

fn centroid(mesh: &PolyMesh, face: &[u32]) -> Vec3 {
    face.iter().map(|&v| mesh.positions[v as usize]).sum::<Vec3>() / face.len() as f64
}

/// Faces of `mesh` that face away from `source` all round them, or have no
/// area: the ones [`measure`] counts as inverted or degenerate.
pub fn folded_faces(source: &TriMesh, surface: &Bvh, mesh: &PolyMesh) -> usize {
    let (min, max) =
        source.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let diagonal = (max - min).length().max(f64::MIN_POSITIVE);
    let mut metrics = Metrics::default();
    orientation(source, surface, mesh, diagonal, &mut metrics);
    metrics.inverted_faces + metrics.degenerate_faces
}

fn orientation(source: &TriMesh, surface: &Bvh, mesh: &PolyMesh, diagonal: f64, metrics: &mut Metrics) {
    let negligible = 1e-12 * diagonal * diagonal;
    for face in &mesh.faces {
        let normal = area_vector(mesh, face);
        if normal.length() < negligible {
            metrics.degenerate_faces += 1;
            continue;
        }
        // A face is inverted when the source around it only faces the other
        // way. Looking at the single closest triangle would be wrong where
        // two sheets of the source touch (a thin wall, intersecting parts):
        // the closest one may well belong to the other sheet.
        let center = centroid(mesh, face);
        let reach = face.iter().map(|&v| mesh.positions[v as usize].distance(center)).fold(0.0, f64::max);
        let agrees = |t: u32| normal.dot(source.area_vector(t as usize)) > 0.0;
        let inverted = match surface.closest_point_where(center, reach, |_| true) {
            Some(_) => surface.closest_point_where(center, reach, agrees).is_none(),
            // Nothing that close: the face is off the surface, which is a
            // fidelity problem; judge its orientation by the nearest triangle.
            None => surface.closest_point(center).is_some_and(|hit| !agrees(hit.triangle)),
        };
        metrics.inverted_faces += inverted as usize;
    }
}

fn mean_edge_length(mesh: &PolyMesh, connectivity: &Connectivity) -> f64 {
    let total: f64 = connectivity
        .uses
        .iter()
        .map(|&((a, b), _)| mesh.positions[a as usize].distance(mesh.positions[b as usize]))
        .sum();
    total / connectivity.uses.len().max(1) as f64
}

/// Below this share of anisotropic curvature the source has no direction
/// worth aligning to.
const MIN_RELATIVE_ANISOTROPY: f64 = 0.1;

fn alignment(
    source: &TriMesh,
    topology: &Topology,
    surface: &Bvh,
    mesh: &PolyMesh,
    connectivity: &Connectivity,
    target_edge: f64,
) -> Option<f64> {
    let normals = vertex_normals(source);
    let areas = vertex_areas(source);
    // A quad mesh can only follow shapes of its own size: look at the
    // curvature of the source at the scale of one quad, not at the scale of
    // its (possibly much finer, possibly noisy) triangles.
    let source_edge = source.edge_length_stats().1;
    let passes = ((target_edge / source_edge).powi(2).round() as usize).min(400);
    let curvature = TensorField::estimate(source, &normals, &areas)
        .smoothed(&topology.neighbors, &normals, &areas, passes)
        .principal(&normals);

    let (mut weighted_angle, mut weight_sum) = (0.0, 0.0);
    let (mut anisotropy_sum, mut magnitude_sum) = (0.0, 0.0);
    for &((a, b), _) in &connectivity.uses {
        let (pa, pb) = (mesh.positions[a as usize], mesh.positions[b as usize]);
        let Some(hit) = surface.closest_point(0.5 * (pa + pb)) else { continue };
        let nearest = source.triangles[hit.triangle as usize]
            .into_iter()
            .min_by(|&x, &y| {
                let distance = |v: u32| source.positions[v as usize].distance_squared(hit.point);
                distance(x).total_cmp(&distance(y))
            })
            .unwrap() as usize;

        let normal = normals[nearest];
        let edge = pb - pa;
        let Some(direction) = (edge - normal * normal.dot(edge)).try_normalize() else { continue };
        let length = edge.length();
        anisotropy_sum += length * curvature.anisotropy(nearest);
        magnitude_sum += length * (curvature.k1[nearest].abs() + curvature.k2[nearest].abs());

        // Angle to the nearest arm of the principal cross, in [0°, 45°].
        let to_first = direction.dot(curvature.dir1[nearest]).abs().min(1.0).acos().to_degrees();
        let weight = length * curvature.anisotropy(nearest);
        weighted_angle += weight * to_first.min(90.0 - to_first);
        weight_sum += weight;
    }
    (weight_sum > 0.0 && anisotropy_sum > MIN_RELATIVE_ANISOTROPY * magnitude_sum).then(|| weighted_angle / weight_sum)
}

fn quad_shape(mesh: &PolyMesh, metrics: &mut Metrics) {
    let (mut deviations, mut aspects, mut areas) = (Vec::new(), Vec::new(), Vec::new());
    for face in &mesh.faces {
        areas.push(area_vector(mesh, face).length());
        if face.len() != 4 {
            continue;
        }
        let corner = |k: usize| mesh.positions[face[k % 4] as usize];
        let (mut shortest, mut longest) = (f64::INFINITY, 0.0f64);
        for k in 0..4 {
            let (to_previous, to_next) = (corner(k + 3) - corner(k), corner(k + 1) - corner(k));
            let length = to_next.length();
            shortest = shortest.min(length);
            longest = longest.max(length);
            if let (Some(a), Some(b)) = (to_previous.try_normalize(), to_next.try_normalize()) {
                deviations.push((a.dot(b).clamp(-1.0, 1.0).acos().to_degrees() - 90.0).abs());
            }
        }
        if shortest > 0.0 {
            aspects.push(longest / shortest);
        }
    }

    (metrics.angle_deviation_mean, metrics.angle_deviation_p95) = mean_and_p95(&mut deviations);
    (metrics.aspect_mean, metrics.aspect_p95) = mean_and_p95(&mut aspects);
    let mean_area = areas.iter().sum::<f64>() / areas.len() as f64;
    let variance = areas.iter().map(|a| (a - mean_area).powi(2)).sum::<f64>() / areas.len() as f64;
    metrics.area_variation = if mean_area > 0.0 { variance.sqrt() / mean_area } else { 0.0 };
}

fn mean_and_p95(values: &mut [f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    values.sort_unstable_by(f64::total_cmp);
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let rank = ((0.95 * values.len() as f64).ceil() as usize).clamp(1, values.len());
    (mean, values[rank - 1])
}

fn fidelity(source: &TriMesh, surface: &Bvh, mesh: &PolyMesh, diagonal: f64, metrics: &mut Metrics) {
    let (mut worst, mut sum, mut count) = (0.0f64, 0.0, 0usize);
    let mut record = |distance_squared: f64| {
        let distance = distance_squared.sqrt();
        worst = worst.max(distance);
        sum += distance;
        count += 1;
    };

    // From the quad mesh to the source ...
    // (The queries side by side, the distances added up in order: the same
    // sums as one at a time.)
    let samples: Vec<Vec3> =
        mesh.positions.iter().copied().chain(mesh.faces.iter().map(|face| centroid(mesh, face))).collect();
    let there: Vec<Option<f64>> =
        samples.par_iter().map(|&sample| surface.closest_point(sample).map(|hit| hit.distance_squared)).collect();
    there.into_iter().flatten().for_each(&mut record);
    // ... and back, which is what catches a missing piece.
    let output = Bvh::build(&mesh.triangulated());
    let mut referenced = vec![false; source.positions.len()];
    for tri in &source.triangles {
        for &v in tri {
            referenced[v as usize] = true;
        }
    }
    let back: Vec<Option<f64>> = source
        .positions
        .par_iter()
        .enumerate()
        .map(|(v, &p)| if referenced[v] { output.closest_point(p).map(|hit| hit.distance_squared) } else { None })
        .collect();
    back.into_iter().flatten().for_each(&mut record);

    metrics.hausdorff = worst / diagonal;
    metrics.mean_distance = sum / count.max(1) as f64 / diagonal;
}
