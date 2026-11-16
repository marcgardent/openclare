//! The reflection check: where a cage will make a reflection wobble or
//! pinch once Catmull-Clark subdivides it, fault by fault, each with one
//! number to rank it by.
//!
//! Catmull-Clark puts a point at the centre of each face (the mean of its
//! vertices) and cuts the face into one quad per corner: centre, middle of
//! the edge before, vertex, middle of the edge after. Three criteria
//! follow, and not every face is concerned by each:
//!
//! 1. Flat (faces of 5 sides and more): the centre stays in the plane of
//!    the face.
//! 2. Star-shaped round its centre (4 sides and more): each corner quad
//!    keeps the winding of the face. The quad of a vertex folds as soon as
//!    one of the two triangles (centre, edge) on either side of it turns
//!    over; one much thinner than the mean is a warning.
//! 3. Kept apart from curvature (every face but a quad): the centre of a
//!    triangle or of an n-gon is a pole, so everything that touches the
//!    face lies in its plane, or its border is sharp (a crease of 1, or a
//!    border of the mesh). Otherwise it pinches at its border.
//!
//! A vertex of 3 edges, or 5 and more, is a pole too, even in a mesh all of
//! quads. The surface stays smooth there but loses its continuity of
//! curvature: a warning where the pole sits on a curved zone, since a
//! closed mesh cannot do without poles.
//!
//! The criteria, their thresholds and their crease rules are those of the
//! ngon_check add-on, but for two things: a fold is read up to 180 degrees
//! where the mesh is consistently oriented (see [`check`]), and a vertex of
//! valence 2, a point on an edge, is no pole. What this module adds is the
//! measure each fault is ranked by: an error of normal shows in a
//! reflection, so every criterion that can be is brought back to an angle,
//! in degrees, comparable across criteria and free of scale. The size of a
//! fault is not multiplied in: it tells how far the fault reaches, not how
//! strong it is. See [`Kind`] for the number each kind of fault carries.
//!
//! Every angle is measured on the cage, not on the subdivided surface: it
//! ranks faults against one another, it is not the actual wobble of the
//! reflection.

use rayon::prelude::*;

use crate::creases::Crease;
use crate::math::Vec3;
use crate::mesh::PolyMesh;

/// The thresholds of what counts as a fault, those of ngon_check.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// Largest distance of a vertex to the plane of its face, over the
    /// radius of the face (the distance from its centre to its furthest
    /// vertex), for a face of 5 sides or more to count as flat.
    pub flat_tolerance: f64,
    /// Largest fold, in degrees, between a face and what touches it, and
    /// between the faces round a pole and their mean, that is not a fault.
    pub angle_tolerance: f64,
    /// A triangle (centre, edge) smaller than this share of the mean one
    /// makes its two corner quads thin.
    pub thin_ratio: f64,
    /// Crease weight from which an edge is sharp; below it a crease still
    /// softens the edge.
    pub crease_sharp: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self { flat_tolerance: 0.002, angle_tolerance: 1.0, thin_ratio: 0.15, crease_sharp: 0.999 }
    }
}

/// What is wrong, and what number ranks it ([`Fault::value`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    /// A face of (numerically) no area: the geometry is wrong. Out of the
    /// ranking, first of all; its value is 0.
    Degenerate,
    /// A face of 4 sides or more with corner quads that fold: the geometry
    /// is wrong. Out of the ranking, right after the degenerate faces. Its
    /// value is the smallest triangle (centre, edge) over the mean one, 0
    /// or below: the most folded first.
    Folded,
    /// A face of 5 sides or more off its plane. Its value is the tilt, in
    /// degrees, of the vertex furthest off the plane seen from the centre
    /// at the radius of the face: `atan(deviation / radius)`.
    NotFlat,
    /// A triangle or an n-gon on a curved zone. Its value is the largest
    /// fold, in degrees, between the face and what touches it where its
    /// border is not sharp (see [`check`] for the corners).
    Pinch,
    /// A vertex of 3 edges, or 5 and more, on a curved zone. Its value is
    /// the largest angle, in degrees, between the normal of a face round
    /// the vertex and their mean.
    CurvedPole,
    /// A face of 4 sides or more with corner quads much thinner than the
    /// others: a warning. A thin quad does not tilt the normal of the cage,
    /// so no angle stands for it: its value is the smallest triangle
    /// (centre, edge) over the mean one, between 0 and
    /// [`Settings::thin_ratio`], and thin faces come after every fault an
    /// angle ranks, the thinnest first.
    Thin,
}

impl Kind {
    pub const ALL: [Kind; 6] =
        [Kind::Degenerate, Kind::Folded, Kind::NotFlat, Kind::Pinch, Kind::CurvedPole, Kind::Thin];

    /// A name in one word, for a report.
    pub fn name(self) -> &'static str {
        match self {
            Kind::Degenerate => "degenerate",
            Kind::Folded => "folded",
            Kind::NotFlat => "not_flat",
            Kind::Pinch => "pinch",
            Kind::CurvedPole => "curved_pole",
            Kind::Thin => "thin",
        }
    }

    /// Whether [`Fault::value`] is an angle in degrees; otherwise it is a
    /// share of the mean triangle (centre, edge), or 0.
    pub fn is_angle(self) -> bool {
        matches!(self, Kind::NotFlat | Kind::Pinch | Kind::CurvedPole)
    }

    /// The kind of repair it calls for.
    pub fn repair(self) -> Repair {
        match self {
            Kind::Folded | Kind::NotFlat | Kind::Thin => Repair::MoveVertices,
            Kind::Degenerate | Kind::Pinch | Kind::CurvedPole => Repair::ChangeTopology,
        }
    }

    /// The group a fault is ranked in, first to last: what is wrong
    /// whatever the angle, collapsed faces before folded ones; what an
    /// angle ranks; the warnings.
    fn tier(self) -> u8 {
        match self {
            Kind::Degenerate => 0,
            Kind::Folded => 1,
            Kind::NotFlat | Kind::Pinch | Kind::CurvedPole => 2,
            Kind::Thin => 3,
        }
    }
}

/// What a fault is mended by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Repair {
    /// Move vertices: flatten the face, or move its centre away from the
    /// edge it comes close to.
    MoveVertices,
    /// Change the topology: the face or the pole is in the wrong place, or
    /// its border wants a crease.
    ChangeTopology,
}

/// Where a fault is: a face or a vertex of the mesh, by its index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    Face(u32),
    Vertex(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub kind: Kind,
    pub at: Element,
    /// The sides of the face, or the valence of the vertex.
    pub sides: usize,
    /// How many parts of the element the fault concerns: vertices off the
    /// plane, corner quads folded or thin, edges and corners that pinch;
    /// the faces round a pole that bend away from their mean.
    pub count: usize,
    /// The number that ranks it, as [`Kind`] says.
    pub value: f64,
    /// Which corners and edges of the face are at fault, for a viewer to
    /// show them: the vertices off the plane, the corners whose quad folds
    /// or is thin, the edges and corners that pinch (all of those that do,
    /// even the corners [`Fault::count`] leaves out). None for a pole or a
    /// degenerate face.
    pub parts: Vec<Part>,
    /// Where to draw it.
    pub frame: Frame,
}

/// A corner or an edge of a face, by its place in the face: corner `i` is
/// its vertex `i`, edge `i` runs from its vertex `i` to its vertex `i + 1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Part {
    Corner(u32),
    Edge(u32),
}

/// The place of a fault, for a viewer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Frame {
    /// The centre of the face (the mean of its vertices), or the vertex.
    pub centre: Vec3,
    /// The unit normal of the face, or the mean normal round the vertex;
    /// zero for a degenerate face.
    pub normal: Vec3,
    /// For a face, the distance from its centre to its nearest edge, and to
    /// its furthest vertex. For a vertex, the mean length of its edges,
    /// twice.
    pub inner: f64,
    pub radius: f64,
}

impl Fault {
    /// The value, where it is an angle in degrees.
    pub fn angle(&self) -> Option<f64> {
        self.kind.is_angle().then_some(self.value)
    }

    pub fn repair(&self) -> Repair {
        self.kind.repair()
    }
}

/// Everything [`check`] finds.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Check {
    /// The faults, worst first: see [`rank`].
    pub faults: Vec<Fault>,
    /// The poles looked at, on a curved zone or not: vertices of 3 edges,
    /// or 5 and more, off the borders and off the creases.
    pub poles: usize,
}

/// How many faults of one kind, and the worst and median of their values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KindScore {
    pub count: usize,
    /// The largest angle, or the smallest share for a folded or thin face.
    pub worst: Option<f64>,
    pub median: Option<f64>,
}

/// What [`Check::score`] sums up of a mesh: for a bench, one line per
/// model.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Score {
    /// The creases the check was given, `None` when none was given at all:
    /// then only the borders of the mesh are sharp.
    pub creases: Option<usize>,
    /// The poles looked at; those on a curved zone are the
    /// [`Kind::CurvedPole`] faults.
    pub poles: usize,
    /// One for each kind, in the order of [`Kind::ALL`].
    pub kinds: [KindScore; 6],
}

impl Score {
    pub fn of(&self, kind: Kind) -> &KindScore {
        &self.kinds[kind as usize]
    }
}

impl Check {
    /// The faults counted kind by kind. `creases` is how many creases the
    /// check was given, if any was.
    pub fn score(&self, creases: Option<usize>) -> Score {
        let mut score = Score { creases, poles: self.poles, ..Default::default() };
        for kind in Kind::ALL {
            let mut values: Vec<f64> = self.faults.iter().filter(|f| f.kind == kind).map(|f| f.value).collect();
            if values.is_empty() {
                continue;
            }
            values.sort_by(f64::total_cmp);
            let n = values.len();
            let median = if n % 2 == 1 { values[n / 2] } else { 0.5 * (values[n / 2 - 1] + values[n / 2]) };
            let worst = if kind.is_angle() { values[n - 1] } else { values[0] };
            score.kinds[kind as usize] = KindScore { count: n, worst: Some(worst), median: Some(median) };
        }
        score
    }
}

/// Puts `faults` in the order to look at them: degenerate faces, then
/// folded ones, the most folded first; then what an angle ranks, the
/// largest angle first whatever its kind; then thin faces, the thinnest
/// first. Ties go by kind, then by element.
pub fn rank(faults: &mut [Fault]) {
    faults.sort_by(|a, b| {
        let by_value = if a.kind.is_angle() { b.value.total_cmp(&a.value) } else { a.value.total_cmp(&b.value) };
        a.kind.tier().cmp(&b.kind.tier()).then(by_value).then(a.kind.cmp(&b.kind)).then(a.at.cmp(&b.at))
    });
}

/// Checks every face and every vertex of `mesh`, a cage to be subdivided
/// with `creases` (edges by their two vertices, weights from 0 to 1; an
/// edge listed twice keeps its largest weight). With no creases, only the
/// borders of the mesh are sharp.
///
/// The angle a pinch gets is the largest of those that make it a fault:
/// across an edge that is not sharp, the fold between the face and a face
/// on the other side; at a corner (a vertex of the face, as ngon_check
/// looks at it), where the vertex is on fewer than two sharp edges, the
/// fold between the face and a face round the vertex, and where it is on
/// two, the tilt off the plane of the face of a sharp edge leaving the
/// face.
///
/// A fold is the angle between the two normals, up to 180 degrees, where
/// the mesh is a consistently oriented manifold: every edge on two faces
/// at most, walked one way by each. Otherwise (a face wound against its
/// neighbours, as a cage with flipped normals has, or an edge on three
/// faces) the sense of a normal tells nothing, and a fold is the angle
/// between the two lines, at most 90 degrees: one beyond 90 then reads as
/// its supplement. The tilt of a sharp edge is that of a line, either way.
pub fn check(mesh: &PolyMesh, creases: &[Crease], settings: &Settings) -> Check {
    let cage = Cage::build(mesh, creases, settings.crease_sharp);
    let tolerance = settings.angle_tolerance.to_radians();
    let mut faults: Vec<Fault> = (0..mesh.faces.len())
        .into_par_iter()
        .flat_map_iter(|f| {
            let mut found = Vec::new();
            cage.check_face(f, settings, tolerance, &mut found);
            found
        })
        .collect();
    // One entry per pole looked at, with its fault if it has one.
    let poles: Vec<Option<Fault>> = (0..mesh.positions.len())
        .into_par_iter()
        .filter_map(|v| {
            let mut looked = 0;
            let fault = cage.check_vertex(v, tolerance, &mut looked);
            (looked > 0).then_some(fault)
        })
        .collect();
    let looked = poles.len();
    faults.extend(poles.into_iter().flatten());
    rank(&mut faults);
    Check { faults, poles: looked }
}

/// Twice the area vector of a polygon (Newell's normal): its length is
/// twice the area.
fn newell(corners: &[Vec3]) -> Vec3 {
    let k = corners.len();
    let mut n = Vec3::ZERO;
    for i in 0..k {
        let (a, b) = (corners[i], corners[(i + 1) % k]);
        n.x += (a.y - b.y) * (a.z + b.z);
        n.y += (a.z - b.z) * (a.x + b.x);
        n.z += (a.x - b.x) * (a.y + b.y);
    }
    n
}

/// The distance from `p` to the segment from `a` to `b`.
fn distance_to_segment(p: Vec3, a: Vec3, b: Vec3) -> f64 {
    let ab = b - a;
    let t = if ab.length_squared() <= 1e-24 { 0.0 } else { ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) };
    (p - (a + t * ab)).length()
}

fn key(a: u32, b: u32) -> (u32, u32) {
    (a.min(b), a.max(b))
}

/// The mesh with what the criteria ask of its connectivity, in flat arrays:
/// a check runs on every change of a cage being edited.
struct Cage<'a> {
    mesh: &'a PolyMesh,
    /// The edges, by their two vertices, the smaller first, in order; the
    /// faces on edge `e` are `edge_faces[edge_offsets[e]..edge_offsets[e + 1]]`.
    edges: Vec<(u32, u32)>,
    edge_offsets: Vec<u32>,
    edge_faces: Vec<u32>,
    /// The sharp edges: borders, and creases from the sharp weight.
    sharp: Vec<bool>,
    /// The other end of each edge of each vertex, and the faces round it,
    /// likewise.
    neighbour_offsets: Vec<u32>,
    neighbours: Vec<u32>,
    round_offsets: Vec<u32>,
    round: Vec<u32>,
    /// The vertices on a border, and how many creases each is on.
    boundary: Vec<bool>,
    creased: Vec<u32>,
    /// The unit normal of each face, where it has one.
    normals: Vec<Option<Vec3>>,
    /// Whether the mesh is a consistently oriented manifold, so that the
    /// sense of its normals can be trusted.
    oriented: bool,
}

/// Offsets from counts: `counts[i]` entries for `i`, one after the other.
fn offsets(counts: impl Iterator<Item = u32>) -> Vec<u32> {
    let mut offsets = vec![0];
    for count in counts {
        offsets.push(offsets.last().unwrap() + count);
    }
    offsets
}

impl<'a> Cage<'a> {
    fn build(mesh: &'a PolyMesh, creases: &[Crease], crease_sharp: f64) -> Self {
        let vertices = mesh.positions.len();
        // Every side of every face: its edge, its face, and whether the
        // face walks it from the smaller vertex.
        let mut sides: Vec<((u32, u32), u32, bool)> = Vec::new();
        let mut round_counts = vec![0u32; vertices];
        for (f, face) in mesh.faces.iter().enumerate() {
            for k in 0..face.len() {
                let (a, b) = (face[k], face[(k + 1) % face.len()]);
                // A face that names a vertex twice is round it once.
                if !face[..k].contains(&a) {
                    round_counts[a as usize] += 1;
                }
                if a != b {
                    sides.push((key(a, b), f as u32, a < b));
                }
            }
        }
        let round_offsets = offsets(round_counts.iter().copied());
        let mut round = vec![0; *round_offsets.last().unwrap() as usize];
        let mut filled = round_offsets.clone();
        for (f, face) in mesh.faces.iter().enumerate() {
            for k in 0..face.len() {
                if !face[..k].contains(&face[k]) {
                    let at = &mut filled[face[k] as usize];
                    round[*at as usize] = f as u32;
                    *at += 1;
                }
            }
        }

        sides.sort_unstable();
        let mut edges: Vec<(u32, u32)> = Vec::new();
        let mut edge_counts = Vec::new();
        let mut oriented = true;
        for (i, &(edge, _, forward)) in sides.iter().enumerate() {
            if edges.last() == Some(&edge) {
                *edge_counts.last_mut().unwrap() += 1;
                // Two faces on an edge walk it both ways, once each.
                let (_, _, before) = sides[i - 1];
                oriented &= before != forward && *edge_counts.last().unwrap() <= 2;
            } else {
                edges.push(edge);
                edge_counts.push(1u32);
            }
        }
        let edge_offsets = offsets(edge_counts.iter().copied());
        let edge_faces: Vec<u32> = sides.iter().map(|&(_, f, _)| f).collect();

        let mut weights: Vec<((u32, u32), f64)> = creases.iter().map(|&([a, b], w)| (key(a, b), w)).collect();
        weights.sort_unstable_by(|x, y| x.0.cmp(&y.0).then(y.1.total_cmp(&x.1)));
        weights.dedup_by_key(|x| x.0);
        let mut neighbour_counts = vec![0u32; vertices];
        let (mut boundary, mut creased) = (vec![false; vertices], vec![0u32; vertices]);
        let mut sharp = Vec::with_capacity(edges.len());
        for (e, &(a, b)) in edges.iter().enumerate() {
            neighbour_counts[a as usize] += 1;
            neighbour_counts[b as usize] += 1;
            let border = edge_counts[e] == 1;
            let creased_edge =
                weights.binary_search_by(|x| x.0.cmp(&(a, b))).is_ok_and(|i| weights[i].1 >= crease_sharp);
            for v in [a, b] {
                boundary[v as usize] |= border;
                creased[v as usize] += creased_edge as u32;
            }
            sharp.push(border || creased_edge);
        }
        let neighbour_offsets = offsets(neighbour_counts.iter().copied());
        let mut neighbours = vec![0; *neighbour_offsets.last().unwrap() as usize];
        let mut filled = neighbour_offsets.clone();
        for &(a, b) in &edges {
            for (v, u) in [(a, b), (b, a)] {
                let at = &mut filled[v as usize];
                neighbours[*at as usize] = u;
                *at += 1;
            }
        }

        let normals = mesh
            .faces
            .par_iter()
            .map(|face| {
                let raw = newell(&face.iter().map(|&v| mesh.positions[v as usize]).collect::<Vec<_>>());
                (raw.length() > 1e-12).then(|| raw.normalize())
            })
            .collect();
        Self {
            mesh,
            edges,
            edge_offsets,
            edge_faces,
            sharp,
            neighbour_offsets,
            neighbours,
            round_offsets,
            round,
            boundary,
            creased,
            normals,
            oriented,
        }
    }

    /// The angle, in radians, between two unit normals `a` and `b`; where
    /// their sense cannot be trusted, between the two lines.
    fn fold(&self, a: Vec3, b: Vec3) -> f64 {
        let cosine = if self.oriented { a.dot(b) } else { a.dot(b).abs() };
        cosine.clamp(-1.0, 1.0).acos()
    }

    fn edge(&self, a: u32, b: u32) -> Option<usize> {
        self.edges.binary_search(&key(a, b)).ok()
    }

    fn is_sharp(&self, a: u32, b: u32) -> bool {
        self.edge(a, b).is_some_and(|e| self.sharp[e])
    }

    /// The faces on the edge from `a` to `b`.
    fn faces_on(&self, a: u32, b: u32) -> &[u32] {
        match self.edge(a, b) {
            Some(e) => &self.edge_faces[self.edge_offsets[e] as usize..self.edge_offsets[e + 1] as usize],
            None => &[],
        }
    }

    /// The other ends of the edges of vertex `v`.
    fn neighbours(&self, v: u32) -> &[u32] {
        &self.neighbours[self.neighbour_offsets[v as usize] as usize..self.neighbour_offsets[v as usize + 1] as usize]
    }

    /// The faces round vertex `v`.
    fn faces_round(&self, v: u32) -> &[u32] {
        &self.round[self.round_offsets[v as usize] as usize..self.round_offsets[v as usize + 1] as usize]
    }

    /// The largest fold, in radians, between `normal` and the faces other
    /// than `face` among `faces`, if one is beyond `tolerance`.
    fn bent(&self, faces: &[u32], face: usize, normal: Vec3, tolerance: f64) -> Option<f64> {
        faces
            .iter()
            .filter(|&&g| g as usize != face)
            .filter_map(|&g| self.normals[g as usize].map(|n| self.fold(n, normal)))
            .filter(|&angle| angle > tolerance)
            .reduce(f64::max)
    }

    fn check_face(&self, f: usize, settings: &Settings, tolerance: f64, faults: &mut Vec<Fault>) {
        let face = &self.mesh.faces[f];
        let k = face.len();
        if k < 3 {
            return;
        }
        let corners: Vec<Vec3> = face.iter().map(|&v| self.mesh.positions[v as usize]).collect();
        let center = corners.iter().sum::<Vec3>() / k as f64;
        let raw = newell(&corners);
        let area = raw.length() / 2.0;
        let radius = corners.iter().map(|&c| (c - center).length()).fold(0.0, f64::max);
        let inner =
            (0..k).map(|i| distance_to_segment(center, corners[i], corners[(i + 1) % k])).fold(f64::INFINITY, f64::min);
        let at = Element::Face(f as u32);
        let mut frame = Frame { centre: center, normal: Vec3::ZERO, inner, radius };
        if area <= 1e-12 || radius <= 1e-9 {
            faults.push(Fault { kind: Kind::Degenerate, at, sides: k, count: k, value: 0.0, parts: Vec::new(), frame });
            return;
        }
        let n = raw / raw.length();
        frame.normal = n;
        let corners_where = |wanted: &dyn Fn(usize) -> bool| -> Vec<Part> {
            (0..k).filter(|&i| wanted(i)).map(|i| Part::Corner(i as u32)).collect()
        };

        // 1. Flat.
        if k > 4 {
            let deviations: Vec<f64> = corners.iter().map(|&c| (c - center).dot(n).abs() / radius).collect();
            let deviation = deviations.iter().copied().fold(0.0, f64::max);
            if deviation > settings.flat_tolerance {
                let parts = corners_where(&|i| deviations[i] > settings.flat_tolerance);
                faults.push(Fault {
                    kind: Kind::NotFlat,
                    at,
                    sides: k,
                    count: parts.len(),
                    value: deviation.atan().to_degrees(),
                    parts,
                    frame,
                });
            }
        }

        // 2. Star-shaped round the centre: the signed area of each triangle
        // (centre, edge i), and the corner quad of vertex i between the
        // triangles of edges i - 1 and i.
        if k > 3 {
            let mean = area / k as f64;
            let wedges: Vec<f64> =
                (0..k).map(|i| (corners[i] - center).cross(corners[(i + 1) % k] - center).dot(n) / 2.0).collect();
            // 2 folded, 1 thin, 0 fine.
            let state = |w: f64| {
                if w <= 1e-9 * area {
                    2
                } else if w < settings.thin_ratio * mean {
                    1
                } else {
                    0
                }
            };
            let quads: Vec<u8> = (0..k).map(|i| state(wedges[(i + k - 1) % k]).max(state(wedges[i]))).collect();
            let smallest = |wanted: u8| {
                wedges.iter().filter(|&&w| state(w) == wanted).map(|&w| w / mean).fold(f64::INFINITY, f64::min)
            };
            for (kind, wanted) in [(Kind::Folded, 2), (Kind::Thin, 1)] {
                let parts = corners_where(&|i| quads[i] == wanted);
                if !parts.is_empty() {
                    let (count, value) = (parts.len(), smallest(wanted));
                    faults.push(Fault { kind, at, sides: k, count, value, parts, frame });
                }
            }
        }

        // 3. Kept apart from curvature.
        if k != 4 {
            self.check_curve(f, frame, tolerance, faults);
        }
    }

    fn check_curve(&self, f: usize, frame: Frame, tolerance: f64, faults: &mut Vec<Fault>) {
        let n = frame.normal;
        let face = &self.mesh.faces[f];
        let k = face.len();
        let position = |v: u32| self.mesh.positions[v as usize];

        // Edge i runs from vertex i to vertex i + 1.
        let edges: Vec<Option<f64>> = (0..k)
            .map(|i| {
                let (a, b) = (face[i], face[(i + 1) % k]);
                if a == b || self.is_sharp(a, b) {
                    return None;
                }
                self.bent(self.faces_on(a, b), f, n, tolerance)
            })
            .collect();

        let corners: Vec<Option<f64>> = face
            .iter()
            .map(|&v| {
                let sharps: Vec<u32> = self.neighbours(v).iter().copied().filter(|&u| self.is_sharp(v, u)).collect();
                match sharps.len() {
                    // A corner: the vertex does not move.
                    3.. => None,
                    // The vertex follows the curve of its two sharp edges.
                    // Those of the face itself are for criterion 1.
                    2 => sharps
                        .iter()
                        .filter(|u| !face.contains(u))
                        .filter_map(|&u| {
                            let d = position(u) - position(v);
                            (d.length() > 1e-12).then(|| (d.dot(n).abs() / d.length()).min(1.0).asin())
                        })
                        .filter(|&angle| angle > tolerance)
                        .reduce(f64::max),
                    _ => self.bent(self.faces_round(v), f, n, tolerance),
                }
            })
            .collect();

        let bent_edges = edges.iter().filter(|e| e.is_some()).count();
        // A corner counts where neither of its edges does already.
        let bad_corners =
            (0..k).filter(|&i| corners[i].is_some() && edges[(i + k - 1) % k].is_none() && edges[i].is_none()).count();
        if bent_edges + bad_corners == 0 {
            return;
        }
        let angle = edges.iter().chain(&corners).flatten().copied().fold(0.0, f64::max);
        let parts = (0..k)
            .filter(|&i| edges[i].is_some())
            .map(|i| Part::Edge(i as u32))
            .chain((0..k).filter(|&i| corners[i].is_some()).map(|i| Part::Corner(i as u32)))
            .collect();
        faults.push(Fault {
            kind: Kind::Pinch,
            at: Element::Face(f as u32),
            sides: k,
            count: bent_edges + bad_corners,
            value: angle.to_degrees(),
            parts,
            frame,
        });
    }

    /// The pole fault of vertex `v`, if it is a pole on a curved zone;
    /// `poles` counts the poles looked at.
    ///
    /// A vertex on a border of the mesh is not looked at, nor one on two
    /// creases or more: they follow the rules of curves, not those of the
    /// surface. Nor is a vertex of valence 2 (or 1): a point on an edge,
    /// which subdivision rounds off like the edge, not a singularity of the
    /// surface.
    fn check_vertex(&self, v: usize, tolerance: f64, poles: &mut usize) -> Option<Fault> {
        let faces = self.faces_round(v as u32);
        let valence = self.neighbours(v as u32).len();
        if faces.is_empty() || valence < 3 || valence == 4 || self.boundary[v] || self.creased[v] >= 2 {
            return None;
        }
        let normals: Vec<Vec3> = faces.iter().filter_map(|&f| self.normals[f as usize]).collect();
        if normals.is_empty() {
            return None;
        }
        *poles += 1;
        let sum: Vec3 = normals.iter().sum();
        let mean = if sum.length() > 0.0 { sum.normalize() } else { normals[0] };
        let angles: Vec<f64> = normals.iter().map(|&n| self.fold(n, mean)).collect();
        let count = angles.iter().filter(|&&a| a > tolerance).count();
        let centre = self.mesh.positions[v];
        let size =
            self.neighbours(v as u32).iter().map(|&u| (self.mesh.positions[u as usize] - centre).length()).sum::<f64>()
                / valence as f64;
        (count > 0).then(|| Fault {
            kind: Kind::CurvedPole,
            at: Element::Vertex(v as u32),
            sides: valence,
            count,
            value: angles.iter().copied().fold(0.0, f64::max).to_degrees(),
            parts: Vec::new(),
            frame: Frame { centre, normal: mean, inner: size, radius: size },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn of_kind(check: &Check, kind: Kind) -> Vec<&Fault> {
        check.faults.iter().filter(|f| f.kind == kind).collect()
    }

    /// A grid of 5 by 5 vertices, column `i` and row `j` at `place(i, j)`,
    /// made of quads but for its 2 by 2 middle, one octagon of the eight
    /// vertices round the middle one (left out of every face). Returns the
    /// mesh and the index of the octagon.
    fn grid_with_octagon(place: impl Fn(usize, usize) -> Vec3) -> (PolyMesh, usize) {
        let index = |i: usize, j: usize| (j * 5 + i) as u32;
        let mut mesh = PolyMesh::default();
        for j in 0..5 {
            for i in 0..5 {
                mesh.positions.push(place(i, j));
            }
        }
        for j in 0..4 {
            for i in 0..4 {
                if (1..3).contains(&i) && (1..3).contains(&j) {
                    continue;
                }
                mesh.faces.push(vec![index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
            }
        }
        let ring = [(1, 1), (2, 1), (3, 1), (3, 2), (3, 3), (2, 3), (1, 3), (1, 2)];
        mesh.faces.push(ring.iter().map(|&(i, j)| index(i, j)).collect());
        let octagon = mesh.faces.len() - 1;
        (mesh, octagon)
    }

    #[test]
    fn a_flat_octagon_in_a_flat_grid_has_no_fault() {
        let (mesh, _) = grid_with_octagon(|i, j| Vec3::new(i as f64, j as f64, 0.0));
        let check = check(&mesh, &[], &Settings::default());
        assert!(check.faults.is_empty(), "{:?}", check.faults);
        // The four middles of the octagon's sides have lost their edge to
        // the middle vertex: valence 3, on a plane.
        assert_eq!(check.poles, 4);
    }

    #[test]
    fn the_same_octagon_on_a_cylinder_pinches_by_the_turn_to_its_neighbours() {
        // Column i at the angle i α round the y axis, radius r; rows h apart.
        // A quad between columns i and i + 1 is a flat rectangle facing the
        // angle (i + 1/2) α. The octagon spans columns 1 to 3 and is mirror
        // symmetric about the angle 2 α and about row 2, so its normal faces
        // the angle 2 α: it turns by 3/2 α to the quads either side of it,
        // and 1/2 α to those above and below.
        let (alpha, r, h) = (10f64.to_radians(), 1.0, 0.2);
        let (mesh, octagon) = grid_with_octagon(|i, j| {
            let angle = i as f64 * alpha;
            Vec3::new(r * angle.sin(), j as f64 * h, r * angle.cos())
        });
        let check = check(&mesh, &[], &Settings::default());

        let pinch = of_kind(&check, Kind::Pinch);
        assert_eq!(pinch.len(), 1);
        assert_eq!(pinch[0].at, Element::Face(octagon as u32));
        assert!(close(pinch[0].value, (1.5 * alpha).to_degrees()), "{}", pinch[0].value);
        // All eight edges fold, by 15 or 5 degrees, beyond the tolerance,
        // and every corner touches a quad that does.
        assert_eq!(pinch[0].count, 8);
        let edges: Vec<Part> = (0..8).map(Part::Edge).collect();
        let corners: Vec<Part> = (0..8).map(Part::Corner).collect();
        assert_eq!(pinch[0].parts, [edges, corners].concat());
        // Drawn round the centre of the octagon, facing the angle 2 α.
        let facing = Vec3::new((2.0 * alpha).sin(), 0.0, (2.0 * alpha).cos());
        assert!((pinch[0].frame.normal.dot(facing).abs() - 1.0).abs() < 1e-12);

        // Off its plane: towards the axis, the six vertices of columns 1
        // and 3 sit at r cos α, the two of column 2 at r. The centre is at
        // (6 r cos α + 2 r) / 8, so column 2 is the furthest off, by
        // 3 r (1 - cos α) / 4, and a corner the furthest from the centre.
        let deviation = 3.0 * r * (1.0 - alpha.cos()) / 4.0;
        let radius = ((r * alpha.sin()).powi(2) + h * h + (r * (1.0 - alpha.cos()) / 4.0).powi(2)).sqrt();
        let not_flat = of_kind(&check, Kind::NotFlat);
        assert_eq!(not_flat.len(), 1);
        assert!(close(not_flat[0].value, (deviation / radius).atan().to_degrees()), "{}", not_flat[0].value);
        assert_eq!(not_flat[0].count, 8);

        // The four valence 3 vertices. Those of rows 1 and 3, column 2, have
        // the octagon (facing 2 α) between two quads facing 3/2 α and 5/2 α:
        // their mean faces 2 α, 1/2 α from the quads. Those of column 1 and
        // 3, row 2, have the octagon and two quads facing 1/2 α (or 7/2 α):
        // their mean is at the angle φ of 2 (sin, cos)(α/2) + (sin, cos)(2α).
        let phi =
            (2.0 * (alpha / 2.0).sin() + (2.0 * alpha).sin()).atan2(2.0 * (alpha / 2.0).cos() + (2.0 * alpha).cos());
        let side = (phi - alpha / 2.0).max(2.0 * alpha - phi).to_degrees();
        let poles = of_kind(&check, Kind::CurvedPole);
        assert_eq!(check.poles, 4);
        assert_eq!(poles.len(), 4);
        let mut values: Vec<f64> = poles.iter().map(|p| p.value).collect();
        values.sort_by(f64::total_cmp);
        let half = (alpha / 2.0).to_degrees();
        for (value, expected) in values.iter().zip([half, half, side, side]) {
            assert!(close(*value, expected), "{value} against {expected}");
        }

        // Nothing else, and the largest angle first.
        assert_eq!(check.faults.len(), 6);
        assert_eq!(check.faults[0].kind, Kind::Pinch);
        assert_eq!(check.faults[5].kind, Kind::NotFlat);
    }

    #[test]
    fn a_crease_leaving_a_face_off_its_plane_pinches_it_at_that_corner() {
        // The flat grid, the vertex below the octagon's corner (1, 1)
        // raised so that the edge from the corner down to it climbs at 20
        // degrees off the plane, rows 1 apart. Creases along row 1 from
        // column 1 on, and down from the corner: the corner is on two sharp
        // edges, one of which leaves the octagon, out of its plane.
        let rise = 20f64.to_radians();
        let (mesh, octagon) = grid_with_octagon(|i, j| {
            let z = if (i, j) == (1, 0) { rise.tan() } else { 0.0 };
            Vec3::new(i as f64, j as f64, z)
        });
        let index = |i: u32, j: u32| j * 5 + i;
        let creases = [(1, 1, 2, 1), (2, 1, 3, 1), (3, 1, 4, 1), (1, 0, 1, 1)]
            .map(|(a, b, c, d)| ([index(a, b), index(c, d)], 1.0));
        let check = check(&mesh, &creases, &Settings::default());
        let pinch = of_kind(&check, Kind::Pinch);
        assert_eq!(pinch.len(), 1, "{:?}", check.faults);
        assert_eq!(pinch[0].at, Element::Face(octagon as u32));
        assert_eq!(pinch[0].count, 1);
        assert_eq!(pinch[0].parts, [Part::Corner(0)]);
        assert!(close(pinch[0].value, 20.0), "{}", pinch[0].value);
        // The rest of the grid is flat but for the two quads under the
        // raised vertex, which no criterion of a quad looks at; the only
        // pole that sees them, (2, 1), is on two creases.
        assert_eq!(check.faults.len(), 1, "{:?}", check.faults);
    }

    fn cube() -> PolyMesh {
        let positions = (0..8).map(|v| Vec3::new((v & 1) as f64, ((v >> 1) & 1) as f64, (v >> 2) as f64)).collect();
        let faces = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
        PolyMesh { positions, faces: faces.iter().map(|f| f.to_vec()).collect() }
    }

    fn all_edges(mesh: &PolyMesh, weight: f64) -> Vec<Crease> {
        mesh.edge_uses().keys().map(|&(a, b)| ([a, b], weight)).collect()
    }

    #[test]
    fn a_cube_corner_is_a_curved_pole_unless_it_is_creased() {
        // Three faces at right angles: their mean is (1, 1, 1) / √3, at
        // acos(1 / √3) from each.
        let mesh = cube();
        let corner = (1.0 / 3f64.sqrt()).acos().to_degrees();
        let bare = check(&mesh, &[], &Settings::default());
        assert_eq!(bare.poles, 8);
        assert_eq!(bare.faults.len(), 8);
        for fault in &bare.faults {
            assert_eq!((fault.kind, fault.sides, fault.count), (Kind::CurvedPole, 3, 3));
            assert!(close(fault.value, corner), "{}", fault.value);
        }
        // A crease that only softens is not sharp.
        assert_eq!(check(&mesh, &all_edges(&mesh, 0.5), &Settings::default()), bare);
        // Creased all round, a corner is on three creases: not looked at.
        let creased = check(&mesh, &all_edges(&mesh, 1.0), &Settings::default());
        assert_eq!((creased.poles, creased.faults.len()), (0, 0));
    }

    /// The unit cube with its corner (1, 1, 1) cut off by a triangle
    /// through the points `t` from it along its three edges; returns the
    /// mesh, the triangle and the three pentagons it cut.
    fn cut_cube(t: f64) -> (PolyMesh, usize, [usize; 3]) {
        let mut positions: Vec<Vec3> =
            (0..7).map(|v| Vec3::new((v & 1) as f64, ((v >> 1) & 1) as f64, (v >> 2) as f64)).collect();
        positions.extend([Vec3::new(1.0 - t, 1.0, 1.0), Vec3::new(1.0, 1.0 - t, 1.0), Vec3::new(1.0, 1.0, 1.0 - t)]);
        let (a, b, c) = (7, 8, 9);
        let faces = vec![
            vec![0, 2, 3, 1],
            vec![0, 1, 5, 4],
            vec![0, 4, 6, 2],
            vec![4, 5, b, a, 6],
            vec![2, 6, a, c, 3],
            vec![1, 3, c, b, 5],
            vec![a, b, c],
        ];
        (PolyMesh { positions, faces }, 6, [3, 4, 5])
    }

    #[test]
    fn a_triangle_on_a_cube_corner_pinches_unless_its_border_is_creased() {
        let (mesh, triangle, pentagons) = cut_cube(0.5);
        let corner = (1.0 / 3f64.sqrt()).acos().to_degrees();

        // The triangle faces (1, 1, 1) / √3, the pentagons an axis each:
        // acos(1 / √3) between them. The pentagons meet the quads, and each
        // other, at right angles.
        let bare = check(&mesh, &[], &Settings::default());
        let pinch = of_kind(&bare, Kind::Pinch);
        assert_eq!(pinch.len(), 4);
        for fault in &pinch {
            match fault.at {
                Element::Face(f) if f as usize == triangle => {
                    assert_eq!(fault.count, 3);
                    assert!(close(fault.value, corner), "{}", fault.value);
                }
                Element::Face(f) if pentagons.contains(&(f as usize)) => {
                    assert_eq!(fault.count, 5);
                    assert!(close(fault.value, 90.0), "{}", fault.value);
                }
                at => panic!("pinch at {at:?}"),
            }
        }
        // Every vertex is of valence 3. A corner of the cube has three faces
        // at right angles; a corner of the triangle has the triangle and two
        // pentagons, facing d = (1, 1, 1) / √3, y and z say: their sum is
        // s = (1/√3, 1 + 1/√3, 1 + 1/√3), the furthest from it is y.
        let poles = of_kind(&bare, Kind::CurvedPole);
        assert_eq!((bare.poles, poles.len()), (10, 10));
        let third = 1.0 / 3f64.sqrt();
        let s = Vec3::new(third, 1.0 + third, 1.0 + third);
        let cut = (s.y / s.length()).acos().to_degrees();
        for fault in &poles {
            let Element::Vertex(v) = fault.at else { panic!() };
            let expected = if v >= 7 { cut } else { corner };
            assert!(close(fault.value, expected), "vertex {v}: {} against {expected}", fault.value);
        }
        assert_eq!(bare.faults.len(), 14);

        // The triangle's border creased: the triangle is kept apart (each of
        // its corners is on two creases, both its own edges), the pentagons
        // still fold onto the quads and each other, and the corners of the
        // triangle are no longer poles looked at.
        let border = [([7, 8], 1.0), ([8, 9], 1.0), ([9, 7], 1.0)];
        let ringed = check(&mesh, &border, &Settings::default());
        let pinch = of_kind(&ringed, Kind::Pinch);
        assert_eq!(pinch.len(), 3);
        for fault in &pinch {
            assert!(matches!(fault.at, Element::Face(f) if pentagons.contains(&(f as usize))));
            assert_eq!(fault.count, 4);
            assert!(close(fault.value, 90.0));
        }
        assert_eq!((ringed.poles, of_kind(&ringed, Kind::CurvedPole).len()), (7, 7));

        // Creased all round: nothing.
        let creased = check(&mesh, &all_edges(&mesh, 1.0), &Settings::default());
        assert_eq!((creased.poles, creased.faults.len()), (0, 0), "{:?}", creased.faults);
    }

    /// Five quads round a vertex at the top of a sphere of radius `rho`:
    /// spokes at the polar angle `beta`, every 72 degrees, and between two
    /// spokes a vertex at the polar angle `gamma`, half way round. With
    /// `rho` infinite, the same on the plane z = 0, the polar angles read
    /// as their sines.
    fn five_round(rho: Option<f64>, beta: f64, gamma: f64) -> PolyMesh {
        let at = |polar: f64, azimuth: f64| match rho {
            Some(rho) => rho * Vec3::new(polar.sin() * azimuth.cos(), polar.sin() * azimuth.sin(), polar.cos()),
            None => Vec3::new(polar.sin() * azimuth.cos(), polar.sin() * azimuth.sin(), 0.0),
        };
        let step = 72f64.to_radians();
        let mut positions = vec![at(0.0, 0.0)];
        for k in 0..5 {
            positions.push(at(beta, k as f64 * step));
            positions.push(at(gamma, (k as f64 + 0.5) * step));
        }
        let spoke = |k: u32| 1 + 2 * (k % 5);
        let faces = (0..5).map(|k| vec![0, spoke(k), spoke(k) + 1, spoke(k + 1)]).collect();
        PolyMesh { positions, faces }
    }

    #[test]
    fn a_valence_five_pole_is_a_fault_on_a_sphere_not_on_a_plane() {
        let (beta, gamma) = (20f64.to_radians(), 30f64.to_radians());
        let flat = check(&five_round(None, beta, gamma), &[], &Settings::default());
        assert_eq!((flat.poles, flat.faults.len()), (1, 0), "{:?}", flat.faults);

        // A quad's normal is along the cross product of its diagonals. One
        // runs from the pole P to the far vertex o, in the plane of the
        // azimuth half way between the spokes; the other, from spoke to
        // spoke, is level and across that plane. So the normal lies in that
        // plane, square to o - P = ρ (sin γ, cos γ - 1): at the polar angle
        // atan((1 - cos γ) / sin γ) = γ / 2. By symmetry the mean of the
        // five is the axis: every face is γ / 2 from it.
        let sphere = check(&five_round(Some(3.0), beta, gamma), &[], &Settings::default());
        assert_eq!((sphere.poles, sphere.faults.len()), (1, 1), "{:?}", sphere.faults);
        let pole = &sphere.faults[0];
        assert_eq!((pole.kind, pole.at, pole.sides, pole.count), (Kind::CurvedPole, Element::Vertex(0), 5, 5));
        assert!(close(pole.value, 15.0), "{}", pole.value);
        // Drawn at the pole, facing up, as wide as its spokes are long:
        // the chord 2 ρ sin(β / 2).
        let spoke = 2.0 * 3.0 * (beta / 2.0).sin();
        assert!(close(pole.frame.normal.z, 1.0) && close(pole.frame.radius, spoke), "{:?}", pole.frame);
    }

    #[test]
    fn a_fold_beyond_a_right_angle_reads_as_itself_where_the_mesh_is_oriented() {
        // A triangle in the plane z = 0 facing up, and across its edge on
        // the x axis a triangle whose far vertex is turned by φ about that
        // axis: its normal is turned by φ from the first.
        let fold = 120f64.to_radians();
        let far = Vec3::new(0.5, -fold.cos(), fold.sin());
        let positions = vec![Vec3::ZERO, Vec3::X, Vec3::new(0.5, 1.0, 0.0), far];
        let oriented = PolyMesh { positions: positions.clone(), faces: vec![vec![0, 1, 2], vec![1, 0, 3]] };
        let pinch = |mesh: &PolyMesh| {
            let check = check(mesh, &[], &Settings::default());
            check.faults.iter().filter(|f| f.kind == Kind::Pinch).map(|f| f.value).fold(0.0, f64::max)
        };
        assert!(close(pinch(&oriented), 120.0), "{}", pinch(&oriented));
        // The second triangle wound the other way: the senses cannot be
        // trusted, and the fold reads as its supplement.
        let flipped = PolyMesh { positions, faces: vec![vec![0, 1, 2], vec![0, 1, 3]] };
        assert!(close(pinch(&flipped), 60.0), "{}", pinch(&flipped));
    }

    #[test]
    fn a_vertex_of_valence_two_is_not_a_pole() {
        // The flat grid with its octagon, and one more vertex in the middle
        // of the edge from (1, 0) to (1, 1), between two quads of the bottom
        // row and used by both: it has two edges, and is a point on that
        // edge. The two quads become flat pentagons with one vertex on a
        // straight side.
        let (mut mesh, _) = grid_with_octagon(|i, j| Vec3::new(i as f64, j as f64, 0.0));
        let (a, b) = (1u32, 6u32);
        let middle = mesh.positions.len() as u32;
        mesh.positions.push(Vec3::new(1.0, 0.5, 0.0));
        for face in &mut mesh.faces {
            for k in 0..face.len() {
                let (u, w) = (face[k], face[(k + 1) % face.len()]);
                if (u, w) == (a, b) || (u, w) == (b, a) {
                    face.insert(k + 1, middle);
                    break;
                }
            }
        }
        let check = check(&mesh, &[], &Settings::default());
        // As before: the four middles of the octagon's sides, and no fault.
        assert_eq!(check.poles, 4);
        assert!(check.faults.is_empty(), "{:?}", check.faults);
        // Raised off the plane, the vertex makes its two pentagons pinch
        // and leave their plane, but it is still no pole.
        mesh.positions[middle as usize].z = 0.2;
        let raised = super::check(&mesh, &[], &Settings::default());
        assert_eq!(raised.poles, 4);
        assert!(raised.faults.iter().all(|f| f.at != Element::Vertex(middle)), "{:?}", raised.faults);
        assert!(raised.faults.iter().any(|f| f.kind == Kind::Pinch));
    }

    /// A regular pentagon of circumradius `r` in the plane z = 0, its
    /// first vertex on the y axis raised by `h`.
    fn raised_pentagon(r: f64, h: f64) -> PolyMesh {
        let positions = (0..5)
            .map(|i| {
                let angle = (90.0 + 72.0 * i as f64).to_radians();
                Vec3::new(r * angle.cos(), r * angle.sin(), if i == 0 { h } else { 0.0 })
            })
            .collect();
        PolyMesh { positions, faces: vec![(0..5).collect()] }
    }

    #[test]
    fn a_pentagon_off_its_plane_tilts_by_its_furthest_vertex() {
        // Newell's normal of the pentagon, by its sums: (0, -2 h r sin 72,
        // 5 r² sin 72), along (0, -2 h, 5 r), of length L = √(4 h² + 25 r²).
        // The centre is (0, 0, h / 5). Off the plane, the raised vertex is
        // at (0, r, 4 h / 5) from the centre: 2 h r / L, the furthest (the
        // others are at h r (2 cos 72 + 1) / L and h r (2 cos 36 - 1) / L),
        // and the furthest from the centre too, at √(r² + 16 h² / 25).
        let (r, h): (f64, f64) = (1.0, 0.1);
        let deviation = 2.0 * h * r / (4.0 * h * h + 25.0 * r * r).sqrt();
        let radius = (r * r + 16.0 * h * h / 25.0).sqrt();
        let check = check(&raised_pentagon(r, h), &[], &Settings::default());
        assert_eq!(check.faults.len(), 1, "{:?}", check.faults);
        let fault = &check.faults[0];
        assert_eq!((fault.kind, fault.sides, fault.repair()), (Kind::NotFlat, 5, Repair::MoveVertices));
        assert!(close(fault.value, (deviation / radius).atan().to_degrees()), "{}", fault.value);
        // Off by more than 0.2 %: the raised vertex (0.0400 of the radius),
        // the two beside it (0.0323) and the two across (0.0123).
        assert_eq!(fault.parts, (0..5).map(Part::Corner).collect::<Vec<_>>());
        // Its border is the mesh's: no pinch, alone as it is.

        // Raised by 0.004, it is off by 0.0016 of its radius at most, within
        // the 0.2 % tolerated.
        let within = super::check(&raised_pentagon(r, 0.004), &[], &Settings::default());
        assert!(within.faults.is_empty(), "{:?}", within.faults);
    }

    /// A dart: tips (0, 3), (-2, -1) and (2, -1), its notch at (0, a).
    fn dart(a: f64) -> PolyMesh {
        let positions = vec![
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(-2.0, -1.0, 0.0),
            Vec3::new(0.0, a, 0.0),
            Vec3::new(2.0, -1.0, 0.0),
        ];
        PolyMesh { positions, faces: vec![vec![0, 1, 2, 3]] }
    }

    #[test]
    fn a_dart_folds_when_its_centre_passes_its_notch() {
        // Area 6 - 2 a, centre (0, (1 + a) / 4). The triangles (centre, edge)
        // either side of the notch have the signed area (1 - 3 a) / 4: below
        // zero once a > 1/3. With a = 1, -1/2 against a mean of 1: the
        // corner quads of the notch and of both lower tips fold.
        let folded = check(&dart(1.0), &[], &Settings::default());
        assert_eq!(folded.faults.len(), 1, "{:?}", folded.faults);
        let fault = &folded.faults[0];
        assert_eq!((fault.kind, fault.count, fault.repair()), (Kind::Folded, 3, Repair::MoveVertices));
        assert!(close(fault.value, -0.5), "{}", fault.value);
        assert_eq!(fault.parts, [Part::Corner(1), Part::Corner(2), Part::Corner(3)]);
        // Centre (0, 1/2). The nearest edges are those from the lower tips to
        // the notch, along (1, 1), at |(2, 3/2) × (1, 1)| / √2 = 1 / (2 √2);
        // the furthest vertices the three tips, at 5/2.
        let inner = 1.0 / (2.0 * 2f64.sqrt());
        assert!(close(fault.frame.inner, inner) && close(fault.frame.radius, 2.5), "{:?}", fault.frame);

        // With a = 0.2: 0.1 against a mean of 5.6 / 4 = 1.4, a share of
        // 1/14, under the 0.15 of a thin quad.
        let thin = check(&dart(0.2), &[], &Settings::default());
        assert_eq!(thin.faults.len(), 1, "{:?}", thin.faults);
        let fault = &thin.faults[0];
        assert_eq!((fault.kind, fault.count), (Kind::Thin, 3));
        assert!(close(fault.value, 1.0 / 14.0), "{}", fault.value);
        assert_eq!(fault.angle(), None);
    }

    #[test]
    fn faults_are_ranked_folded_first_then_by_angle_then_thin() {
        let fault = |kind, v, value| Fault {
            kind,
            at: Element::Vertex(v),
            sides: 3,
            count: 1,
            value,
            parts: Vec::new(),
            frame: Frame::default(),
        };
        let mut faults = vec![
            fault(Kind::Thin, 0, 0.1),
            fault(Kind::NotFlat, 1, 2.0),
            fault(Kind::Pinch, 2, 12.0),
            fault(Kind::Folded, 3, -0.1),
            fault(Kind::CurvedPole, 4, 30.0),
            fault(Kind::Thin, 5, 0.01),
            fault(Kind::Folded, 6, -0.5),
            fault(Kind::Degenerate, 7, 0.0),
        ];
        rank(&mut faults);
        let order: Vec<u32> = faults.iter().map(|f| if let Element::Vertex(v) = f.at { v } else { 99 }).collect();
        assert_eq!(order, [7, 6, 3, 4, 2, 1, 5, 0]);

        let score = Check { faults, poles: 9 }.score(None);
        assert_eq!(score.of(Kind::Folded), &KindScore { count: 2, worst: Some(-0.5), median: Some(-0.3) });
        assert_eq!(score.of(Kind::Pinch), &KindScore { count: 1, worst: Some(12.0), median: Some(12.0) });
        assert_eq!(score.of(Kind::Thin).worst, Some(0.01));
        assert_eq!(score.poles, 9);
    }

    #[test]
    fn a_square_face_is_fine_however_it_bends() {
        // A quad's centre is a regular point: a quad off its plane, next to
        // a fold, is no fault of its own (only criterion 2 is its).
        let positions = vec![
            Vec3::ZERO,
            Vec3::X,
            Vec3::new(1.0, 1.0, 0.3),
            Vec3::Y,
            Vec3::new(0.0, 2.0, 1.0),
            Vec3::new(1.0, 2.0, 1.0),
        ];
        let mesh = PolyMesh { positions, faces: vec![vec![0, 1, 2, 3], vec![3, 2, 5, 4]] };
        let check = check(&mesh, &[], &Settings::default());
        assert!(check.faults.is_empty(), "{:?}", check.faults);
    }
}
