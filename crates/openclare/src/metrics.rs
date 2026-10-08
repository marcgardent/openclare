//! Quality measures of a quad mesh against the surface it was made from.
//!
//! This is the yardstick of the project: any remesher's output (ours,
//! Blender's QuadriFlow, anything that writes polygons) goes through
//! [`measure`], so that "better" is a number rather than an impression.

use std::collections::HashMap;
use std::fmt::Write;

use crate::curvature::TensorField;
use crate::geometry::{vertex_areas, vertex_normals};
use crate::math::{INVALID, Vec3};
use crate::mesh::{PolyMesh, Topology, TriMesh};
use crate::spatial::Bvh;

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
        let fields: [(&str, String); 27] = [
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
        ];
        let mut json = String::from("{");
        for (name, value) in &fields {
            write!(json, "\"{name}\": {value}, ").unwrap();
        }
        write!(json, "\"budget_error\": {}}}", optional(self.budget_error)).unwrap();
        json
    }
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

    orientation(source, &surface, mesh, diagonal, &mut metrics);
    let target_edge = match target_faces {
        Some(n) if n > 0 => (source.surface_area() / n as f64).sqrt(),
        _ => mean_edge_length(mesh, &connectivity),
    };
    metrics.alignment = alignment(source, &source_topology, &surface, mesh, &connectivity, target_edge);
    quad_shape(mesh, &mut metrics);
    fidelity(source, &surface, mesh, diagonal, &mut metrics);
    thin_parts(source, mesh, target_edge, &mut metrics);
    if metrics.source_boundary_edges == 0 && metrics.boundary_edges == 0 && metrics.non_manifold_edges == 0 {
        let volume = source.signed_volume();
        if volume.abs() > 0.0 {
            metrics.volume_ratio = Some((mesh.triangulated().signed_volume() / volume).abs());
        }
    }
    metrics
}

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
    let (mut sum, mut count) = (0.0, 0usize);
    for (v, &p) in source.positions.iter().enumerate() {
        if across[v] < quad_edge
            && let Some(hit) = output.closest_point(p)
        {
            sum += hit.distance_squared.sqrt();
            count += 1;
        }
    }
    metrics.thin_share = count as f64 / source.positions.len().max(1) as f64;
    if count >= THIN_SAMPLES {
        metrics.thin_distance = Some(sum / count as f64 / quad_edge);
    }
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
    let samples = mesh.positions.iter().copied().chain(mesh.faces.iter().map(|face| centroid(mesh, face)));
    for sample in samples {
        if let Some(hit) = surface.closest_point(sample) {
            record(hit.distance_squared);
        }
    }
    // ... and back, which is what catches a missing piece.
    let output = Bvh::build(&mesh.triangulated());
    let mut referenced = vec![false; source.positions.len()];
    for tri in &source.triangles {
        for &v in tri {
            referenced[v as usize] = true;
        }
    }
    for (v, &p) in source.positions.iter().enumerate() {
        if referenced[v]
            && let Some(hit) = output.closest_point(p)
        {
            record(hit.distance_squared);
        }
    }

    metrics.hausdorff = worst / diagonal;
    metrics.mean_distance = sum / count.max(1) as f64 / diagonal;
}
