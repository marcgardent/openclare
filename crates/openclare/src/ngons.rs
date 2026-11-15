//! Flat parts as n-gons: an output apart from the quads, for a mesh with
//! fewer faces that subdivides the same.
//!
//! A face of many sides subdivides as the quads it stands for when it is
//! flat, convex, and cut off from the rest of the surface by sharp edges:
//! the subdivision then makes nothing but points of its plane, inside its
//! outline. So the quads of a flat part of the mesh are merged where all
//! three hold:
//!
//! - the part is everything a walk across the edges reaches without
//!   crossing a full crease or leaving the mesh, and all of it lies in one
//!   plane. A flat that runs into a curve without a fold between them is
//!   left as quads, the whole part;
//! - if its outline is convex, the part becomes one face. Otherwise its
//!   quads are merged two faces at a time as long as what they make is
//!   convex: an L comes out as rectangles, the part round a hole as
//!   sectors;
//! - nothing is merged across an edge the report has something to say of
//!   (a guide curve, a border between two groups).
//!
//! Before that, the flat parts one face can stand for, the cap of a
//! cylinder say, are taken out of the input, so that the chain does not
//! mesh them at all, and the border it leaves there is closed by one face
//! (see [`FlatNgons::open`]).
//!
//! The vertices of the outline are all kept, those on a straight side too:
//! the faces across the crease use them. The vertices left between two
//! merged faces, with nothing but these two faces round them, are removed.
//!
//! Every edge of a merged face that has a face on its other side is a full
//! crease of the result, those between two faces of the same flat part as
//! well: in a plane a crease changes nothing of the shape, and without it
//! the subdivision pulls the line between two large faces towards the
//! middle of each, out of the part where it is not convex.
//!
//! A vertex where three creases meet does not move when the mesh is
//! subdivided, and one on a crease slides along it. So where a part is cut
//! in several faces, a cut must not end on a crease that bends, the rim of
//! a round hole say, or the rim would keep a corner there: the quads that
//! touch such a rim are left as they are.

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::creases::Crease;
use crate::math::{Vec3, tangent_frame};
use crate::mesh::{PolyMesh, TriMesh};

#[derive(Clone, Debug)]
pub struct FlatNgons {
    /// How far from its plane a vertex of a flat part may lie, as a share
    /// of the diagonal of the mesh.
    pub flatness: f64,
    /// How far a corner of a face may turn inwards and the face still be
    /// taken as convex, in degrees: the lines of a grid are not quite
    /// straight.
    pub slack: f64,
    /// Least crease weight that cuts a flat part off from its neighbours.
    pub sharp: f64,
    /// Least turn of a crease at a vertex, in degrees, for it to bend
    /// there.
    pub bend: f64,
}

impl Default for FlatNgons {
    fn default() -> Self {
        Self { flatness: 1e-4, slack: 3.0, sharp: 0.999, bend: 0.1 }
    }
}

/// What [`FlatNgons::merge`] made of a mesh.
#[derive(Clone, Debug, Default)]
pub struct Merged {
    pub mesh: PolyMesh,
    /// The vertex of `mesh` each vertex of the mesh merged became,
    /// `u32::MAX` for those removed.
    pub vertices: Vec<u32>,
    /// A face of the mesh merged that each face of `mesh` is, or is made
    /// of.
    pub faces: Vec<u32>,
    /// Whether each face of `mesh` was made by merging others.
    pub made: Vec<bool>,
}

type Edge = (u32, u32);

fn key(a: u32, b: u32) -> Edge {
    (a.min(b), a.max(b))
}

fn steps(face: &[u32]) -> impl Iterator<Item = Edge> + '_ {
    (0..face.len()).map(|k| (face[k], face[(k + 1) % face.len()]))
}

/// The one loop the directed edges make, each vertex met once, if they do.
fn single_loop(edges: &[Edge]) -> Option<Vec<u32>> {
    let mut next = HashMap::with_capacity(edges.len());
    for &(a, b) in edges {
        if next.insert(a, b).is_some() {
            return None;
        }
    }
    let start = edges.first()?.0;
    let mut outline = vec![start];
    let mut at = *next.get(&start)?;
    while at != start {
        if outline.len() > edges.len() {
            return None;
        }
        outline.push(at);
        at = *next.get(&at)?;
    }
    (outline.len() == edges.len() && outline.len() >= 3).then_some(outline)
}

/// A flat part: its plane, as a point, and two directions that turn the
/// way its faces do.
#[derive(Clone, Debug)]
struct Plane {
    origin: Vec3,
    normal: Vec3,
    across: (Vec3, Vec3),
}

impl Plane {
    fn new(origin: Vec3, normal: Vec3) -> Self {
        Self { origin, normal, across: tangent_frame(normal) }
    }

    fn off(&self, p: Vec3) -> f64 {
        (p - self.origin).dot(self.normal).abs()
    }

    fn flat(&self, p: Vec3) -> (f64, f64) {
        ((p - self.origin).dot(self.across.0), (p - self.origin).dot(self.across.1))
    }

    /// Whether the outline is convex, to within `slack` radians at each
    /// corner, turns once, and sees all of itself from the mean of its
    /// vertices, where the subdivision puts the point of the face.
    fn convex(&self, positions: &[Vec3], outline: &[u32], slack: f64) -> bool {
        let points: Vec<(f64, f64)> = outline.iter().map(|&v| self.flat(positions[v as usize])).collect();
        let n = points.len();
        let centre = points.iter().fold((0.0, 0.0), |(x, y), p| (x + p.0 / n as f64, y + p.1 / n as f64));
        let mut turned = 0.0;
        for k in 0..n {
            let (p, q, r) = (points[k], points[(k + 1) % n], points[(k + 2) % n]);
            let (u, v) = ((q.0 - p.0, q.1 - p.1), (r.0 - q.0, r.1 - q.1));
            let turn = (u.0 * v.1 - u.1 * v.0).atan2(u.0 * v.0 + u.1 * v.1);
            let seen = (p.0 - centre.0) * (q.1 - centre.1) - (p.1 - centre.1) * (q.0 - centre.0);
            if turn < -slack || seen <= 0.0 {
                return false;
            }
            turned += turn;
        }
        (turned - std::f64::consts::TAU).abs() < 0.5
    }
}

impl FlatNgons {
    /// `mesh` with the quads of its flat parts merged. `creases` are those
    /// of the mesh, and nothing is merged across the edges of `held`, nor
    /// between two faces of different `groups` (one per face, or none).
    pub fn merge(&self, mesh: &PolyMesh, creases: &[Crease], held: &[[u32; 2]], groups: &[u32]) -> Merged {
        let positions = &mesh.positions;
        let mut faces: Vec<Vec<u32>> = mesh.faces.clone();
        let (low, high) =
            positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
        let tolerance = self.flatness * (high - low).length();

        // The face on the left of each edge as it is walked; an edge walked
        // twice the same way has no place in a surface.
        let mut owner: HashMap<Edge, usize> = HashMap::new();
        let mut broken: HashSet<Edge> = HashSet::new();
        for (f, face) in faces.iter().enumerate() {
            for (a, b) in steps(face) {
                if owner.insert((a, b), f).is_some() {
                    broken.insert(key(a, b));
                }
            }
        }
        let across = |owner: &HashMap<Edge, usize>, (a, b): Edge| -> Option<usize> {
            if broken.contains(&key(a, b)) { None } else { owner.get(&(b, a)).copied() }
        };
        let cut: HashSet<Edge> =
            creases.iter().filter(|(_, weight)| *weight >= self.sharp).map(|&([a, b], _)| key(a, b)).collect();
        // Edges nothing is merged across: every crease, sharp or not, and
        // those held.
        let mut kept: HashSet<Edge> = creases.iter().map(|&([a, b], _)| key(a, b)).collect();
        kept.extend(held.iter().map(|&[a, b]| key(a, b)));
        if !groups.is_empty() {
            for (&(a, b), &f) in &owner {
                if across(&owner, (a, b)).is_some_and(|g| groups[f] != groups[g]) {
                    kept.insert(key(a, b));
                }
            }
        }

        // The parts: what a walk reaches without crossing a full crease.
        let mut part = vec![usize::MAX; faces.len()];
        let mut parts: Vec<Vec<usize>> = Vec::new();
        for seed in 0..faces.len() {
            if part[seed] != usize::MAX {
                continue;
            }
            part[seed] = parts.len();
            let mut members = vec![seed];
            let mut head = 0;
            while head < members.len() {
                let f = members[head];
                head += 1;
                for (a, b) in steps(&faces[f]) {
                    if cut.contains(&key(a, b)) {
                        continue;
                    }
                    if let Some(g) = across(&owner, (a, b)).filter(|&g| part[g] == usize::MAX) {
                        part[g] = parts.len();
                        members.push(g);
                    }
                }
            }
            parts.push(members);
        }

        // The vertices where a crease bends: two full creases meet there,
        // and not in a straight line.
        let mut creased: HashMap<u32, Vec<u32>> = HashMap::new();
        for &(a, b) in &cut {
            creased.entry(a).or_default().push(b);
            creased.entry(b).or_default().push(a);
        }
        let bends = |v: u32| {
            creased.get(&v).is_some_and(|ends| {
                let &[a, b] = ends.as_slice() else { return false };
                let (p, q, r) = (positions[a as usize], positions[v as usize], positions[b as usize]);
                (q - p).angle_between(r - q) > self.bend.to_radians()
            })
        };
        let on_a_bend: Vec<bool> = faces.iter().map(|face| face.iter().any(|&v| bends(v))).collect();

        let mut alive = vec![true; faces.len()];
        let mut made = vec![false; faces.len()];
        for members in &parts {
            if members.len() < 2 {
                continue;
            }
            let Some(plane) = self.plane_of(positions, &faces, members, tolerance) else { continue };
            let slack = self.slack.to_radians();

            // All of it in one face, if its outline is convex.
            let inside = |f: usize| part[f] == part[members[0]];
            let mut outline = Vec::new();
            let mut whole = true;
            for &f in members {
                for (a, b) in steps(&faces[f]) {
                    match across(&owner, (a, b)).filter(|&g| inside(g)) {
                        Some(_) => whole &= !kept.contains(&key(a, b)),
                        None => outline.push((a, b)),
                    }
                }
            }
            if whole && let Some(outline) = single_loop(&outline).filter(|o| plane.convex(positions, o, slack)) {
                for &f in &members[1..] {
                    alive[f] = false;
                }
                faces[members[0]] = outline;
                made[members[0]] = true;
                continue;
            }

            // Otherwise two faces at a time, as long as the two make a
            // convex one.
            let mut changed = true;
            while changed {
                changed = false;
                for &f in members.iter().filter(|&&f| !on_a_bend[f]) {
                    while alive[f] {
                        let joined = steps(&faces[f]).filter(|&(a, b)| !kept.contains(&key(a, b))).find_map(|edge| {
                            let g =
                                across(&owner, edge).filter(|&g| g != f && alive[g] && inside(g) && !on_a_bend[g])?;
                            let theirs: HashSet<Edge> = steps(&faces[g]).collect();
                            let ours: HashSet<Edge> = steps(&faces[f]).collect();
                            let shared = |&(a, b): &Edge| theirs.contains(&(b, a));
                            if steps(&faces[f]).filter(shared).any(|(a, b)| kept.contains(&key(a, b))) {
                                return None;
                            }
                            let around: Vec<Edge> = steps(&faces[f])
                                .filter(|edge| !shared(edge))
                                .chain(steps(&faces[g]).filter(|&(a, b)| !ours.contains(&(b, a))))
                                .collect();
                            let outline = single_loop(&around)?;
                            plane.convex(positions, &outline, slack).then_some((g, outline))
                        });
                        let Some((g, outline)) = joined else { break };
                        for edge in steps(&faces[g]) {
                            owner.insert(edge, f);
                        }
                        let gone: Vec<Edge> =
                            steps(&faces[f]).filter(|&(a, b)| owner.get(&(b, a)) == Some(&f)).collect();
                        for (a, b) in gone {
                            owner.remove(&(a, b));
                            owner.remove(&(b, a));
                        }
                        alive[g] = false;
                        faces[f] = outline;
                        made[f] = true;
                        changed = true;
                    }
                }
            }

            // The vertices left on the line between two of the faces made.
            let mut around: BTreeMap<u32, Vec<Edge>> = BTreeMap::new();
            for &f in members.iter().filter(|&&f| alive[f]) {
                for (a, b) in steps(&faces[f]) {
                    around.entry(a).or_default().push(key(a, b));
                    around.entry(b).or_default().push(key(a, b));
                }
            }
            for (v, mut edges) in around {
                edges.sort_unstable();
                // Two edges, each with a face on either side.
                let two = edges.len() == 4 && edges[0] == edges[1] && edges[2] == edges[3] && edges[1] != edges[2];
                if !two || edges.iter().any(|edge| kept.contains(edge)) {
                    continue;
                }
                let users: Vec<usize> =
                    members.iter().copied().filter(|&f| alive[f] && faces[f].contains(&v)).collect();
                let without: Vec<Vec<u32>> =
                    users.iter().map(|&f| faces[f].iter().copied().filter(|&corner| corner != v).collect()).collect();
                if without.iter().all(|outline| outline.len() >= 3 && plane.convex(positions, outline, slack)) {
                    for (f, outline) in users.into_iter().zip(without) {
                        faces[f] = outline;
                    }
                }
            }
        }

        // The vertices no face uses any more are left out.
        let mut vertices = vec![u32::MAX; positions.len()];
        let mut merged = Merged::default();
        for f in (0..faces.len()).filter(|&f| alive[f]) {
            let face = faces[f]
                .iter()
                .map(|&v| {
                    if vertices[v as usize] == u32::MAX {
                        vertices[v as usize] = merged.mesh.positions.len() as u32;
                        merged.mesh.positions.push(positions[v as usize]);
                    }
                    vertices[v as usize]
                })
                .collect();
            merged.mesh.faces.push(face);
            merged.faces.push(f as u32);
            merged.made.push(made[f]);
        }
        merged.vertices = vertices;
        merged
    }

    /// The plane of the faces, if they all lie in one and face the same
    /// way.
    fn plane_of(&self, positions: &[Vec3], faces: &[Vec<u32>], members: &[usize], tolerance: f64) -> Option<Plane> {
        let facing = |f: usize| -> Vec3 {
            let face = &faces[f];
            let first = positions[face[0] as usize];
            (1..face.len() - 1)
                .map(|k| (positions[face[k] as usize] - first).cross(positions[face[k + 1] as usize] - first))
                .sum()
        };
        let normal = members.iter().map(|&f| facing(f)).sum::<Vec3>().try_normalize()?;
        let corners = || members.iter().flat_map(|&f| faces[f].iter().map(|&v| positions[v as usize]));
        let origin = corners().sum::<Vec3>() / corners().count() as f64;
        let flat = corners().all(|p| (p - origin).dot(normal).abs() <= tolerance)
            && members.iter().all(|&f| facing(f).dot(normal) > 0.0);
        flat.then(|| Plane::new(origin, normal))
    }
}

/// A flat part of the input taken out of it before the chain: its plane,
/// its outline as the input has it, and the group of its triangles.
#[derive(Clone, Debug)]
struct Lid {
    plane: Plane,
    outline: Vec<Vec3>,
    group: u32,
}

/// An input with flat parts taken out (see [`FlatNgons::open`]).
#[derive(Clone, Debug)]
pub struct Opened {
    /// What is left of the input, open where the parts were.
    pub mesh: TriMesh,
    /// The vertex and the triangle of the input each of those of `mesh`
    /// was.
    pub vertices: Vec<u32>,
    pub triangles: Vec<u32>,
    lids: Vec<Lid>,
    tolerance: f64,
    slack: f64,
}

impl FlatNgons {
    /// `input` without those of its flat parts that one face can stand
    /// for, so that the chain does not mesh them at all: the quads round a
    /// cylinder are then those its side calls for, not those its caps can
    /// take. Such a part lies in one plane, has for its outline one convex
    /// loop of edges folded by more than `angle` degrees, and no neighbour
    /// that is such a part itself: a cube is left whole. `None` if there
    /// is none.
    pub fn open(&self, input: &TriMesh, angle: f64, groups: Option<&[u32]>) -> Option<Opened> {
        let positions = &input.positions;
        let (low, high) =
            positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
        let tolerance = self.flatness * (high - low).length();
        let facing: Vec<Vec3> = (0..input.triangles.len()).map(|t| input.area_vector(t).normalize_or_zero()).collect();
        let mut owner: HashMap<Edge, usize> = HashMap::new();
        let mut broken: HashSet<Edge> = HashSet::new();
        for (t, triangle) in input.triangles.iter().enumerate() {
            for (a, b) in steps(triangle) {
                if owner.insert((a, b), t).is_some() {
                    broken.insert(key(a, b));
                }
            }
        }
        // The triangle across an edge, and whether the surface folds there.
        let across = |(a, b): Edge| owner.get(&(b, a)).copied().filter(|_| !broken.contains(&key(a, b)));
        let folded = |t: usize, u: usize| facing[t].dot(facing[u]) < angle.to_radians().cos();

        let mut part = vec![usize::MAX; input.triangles.len()];
        let mut lids: Vec<Option<Lid>> = Vec::new();
        for seed in 0..input.triangles.len() {
            if part[seed] != usize::MAX {
                continue;
            }
            part[seed] = lids.len();
            let mut members = vec![seed];
            let mut outline = Vec::new();
            let mut closed = true;
            let mut head = 0;
            while head < members.len() {
                let t = members[head];
                head += 1;
                for edge in steps(&input.triangles[t]) {
                    match across(edge) {
                        None => closed = false,
                        Some(u) if folded(t, u) => outline.push(edge),
                        Some(u) if part[u] == usize::MAX => {
                            part[u] = lids.len();
                            members.push(u);
                        }
                        Some(_) => {}
                    }
                }
            }
            let lid = || -> Option<Lid> {
                let normal = members.iter().map(|&t| input.area_vector(t)).sum::<Vec3>().try_normalize()?;
                let corners = || members.iter().flat_map(|&t| input.corners(t));
                let plane = Plane::new(corners().sum::<Vec3>() / (3 * members.len()) as f64, normal);
                let flat = corners().all(|p| plane.off(p) <= tolerance)
                    && members.iter().all(|&t| facing[t].dot(normal) > 0.0);
                let group = groups.map_or(0, |groups| groups[members[0]]);
                let one_group = groups.is_none_or(|groups| members.iter().all(|&t| groups[t] == group));
                let outline = single_loop(&outline)?;
                (closed && flat && one_group && plane.convex(positions, &outline, self.bend.to_radians()))
                    .then(|| Lid { plane, outline: outline.iter().map(|&v| positions[v as usize]).collect(), group })
            };
            lids.push(lid());
        }
        // Two such parts side by side are both left in: the edge between
        // them would have no face of the chain to give it its vertices.
        let mut alone: Vec<bool> = lids.iter().map(Option::is_some).collect();
        for (&edge, &t) in &owner {
            if across(edge).is_some_and(|u| part[u] != part[t] && lids[part[u]].is_some()) {
                alone[part[t]] = false;
            }
        }
        if !alone.contains(&true) || alone.iter().all(|&out| out) {
            return None;
        }

        let mut opened = Opened {
            mesh: TriMesh::default(),
            vertices: Vec::new(),
            triangles: Vec::new(),
            lids: lids.into_iter().zip(&alone).filter_map(|(lid, &out)| lid.filter(|_| out)).collect(),
            tolerance,
            slack: self.slack.to_radians(),
        };
        let mut became = vec![u32::MAX; positions.len()];
        for (t, triangle) in input.triangles.iter().enumerate().filter(|&(t, _)| !alone[part[t]]) {
            opened.mesh.triangles.push(triangle.map(|v| {
                if became[v as usize] == u32::MAX {
                    became[v as usize] = opened.mesh.positions.len() as u32;
                    opened.mesh.positions.push(positions[v as usize]);
                    opened.vertices.push(v);
                }
                became[v as usize]
            }));
            opened.triangles.push(t as u32);
        }
        Some(opened)
    }
}

impl Opened {
    /// `mesh`, made by the chain from what was left of the input, with one
    /// face on each of the parts taken out: the border the chain left
    /// there, walked the other way. What `report` says of the mesh is said
    /// of the result. `None` if a part has no border of its own that is
    /// flat and convex: the mesh is then not to be used.
    pub(crate) fn close(&self, mut mesh: PolyMesh, report: &mut crate::pipeline::Report) -> Option<PolyMesh> {
        let walked: HashSet<Edge> = mesh.faces.iter().flat_map(|face| steps(face)).collect();
        let mut after: HashMap<u32, u32> = HashMap::new();
        for &(a, b) in walked.iter().filter(|&&(a, b)| !walked.contains(&(b, a))) {
            // Walked the other way, the border turns as the lid faces.
            if after.insert(b, a).is_some() {
                return None;
            }
        }
        let mut borders: Vec<Vec<u32>> = Vec::new();
        let mut starts: Vec<u32> = after.keys().copied().collect();
        starts.sort_unstable();
        let mut seen: HashSet<u32> = HashSet::new();
        for start in starts {
            if !seen.insert(start) {
                continue;
            }
            let mut border = vec![start];
            let mut at = after[&start];
            while at != start {
                if !seen.insert(at) {
                    return None;
                }
                border.push(at);
                at = *after.get(&at)?;
            }
            borders.push(border);
        }

        let mut taken = vec![false; borders.len()];
        for lid in &self.lids {
            // The border in the plane of the lid whose middle is inside
            // its outline.
            let n = lid.outline.len();
            let inside = |p: Vec3| {
                let p = lid.plane.flat(p);
                (0..n).all(|k| {
                    let (a, b) = (lid.plane.flat(lid.outline[k]), lid.plane.flat(lid.outline[(k + 1) % n]));
                    (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0) >= 0.0
                })
            };
            let fits = |border: &Vec<u32>| {
                let at = |&v: &u32| mesh.positions[v as usize];
                border.iter().all(|v| lid.plane.off(at(v)) <= self.tolerance)
                    && inside(border.iter().map(at).sum::<Vec3>() / border.len() as f64)
            };
            let mut fitting = (0..borders.len()).filter(|&b| fits(&borders[b]));
            let (b, None) = (fitting.next()?, fitting.next()) else { return None };
            if std::mem::replace(&mut taken[b], true) || !lid.plane.convex(&mesh.positions, &borders[b], self.slack) {
                return None;
            }
            for (a, b) in steps(&borders[b]) {
                report.creases.push(([a, b], 1.0));
                report.hard_edges.push([a, b]);
            }
            if !report.face_groups.is_empty() {
                report.face_groups.push(lid.group);
            }
            mesh.faces.push(borders[b].clone());
        }
        report.merged_faces += self.lids.len();
        report.stats = mesh.stats();
        Some(mesh)
    }
}

/// `mesh` with its flat parts as n-gons, and what `report` says of it said
/// of the result.
pub(crate) fn flat_parts(mesh: &PolyMesh, report: &mut crate::pipeline::Report) -> PolyMesh {
    let mut held = report.hard_edges.clone();
    held.extend(&report.guide_edges);
    held.extend(&report.sharp_curve_edges);
    held.extend(report.curve_edges.iter().map(|&(edge, _)| edge));
    let done = FlatNgons::default().merge(mesh, &report.creases, &held, &report.face_groups);

    // A pole whose vertex is gone is no longer one.
    let nearest = |p: Vec3| {
        (0..mesh.positions.len())
            .min_by(|&a, &b| mesh.positions[a].distance_squared(p).total_cmp(&mesh.positions[b].distance_squared(p)))
    };
    report.poles.retain(|pole| nearest(pole.position).is_none_or(|v| done.vertices[v] != u32::MAX));

    let carried = |[a, b]: [u32; 2]| -> Option<[u32; 2]> {
        let edge = [done.vertices[a as usize], done.vertices[b as usize]];
        (!edge.contains(&u32::MAX)).then_some(edge)
    };
    for edges in [&mut report.hard_edges, &mut report.guide_edges, &mut report.sharp_curve_edges] {
        *edges = edges.iter().filter_map(|&edge| carried(edge)).collect();
    }
    report.curve_edges = report.curve_edges.iter().filter_map(|&(edge, curve)| Some((carried(edge)?, curve))).collect();
    report.creases = report.creases.iter().filter_map(|&(edge, weight)| Some((carried(edge)?, weight))).collect();
    // A full crease all round each face made, but where the mesh ends.
    let uses = done.mesh.edge_uses();
    let mut round: BTreeMap<Edge, ()> = BTreeMap::new();
    for (face, _) in done.mesh.faces.iter().zip(&done.made).filter(|(_, made)| **made) {
        round.extend(steps(face).map(|(a, b)| (key(a, b), ())).filter(|(edge, ())| uses[edge] == 2));
    }
    for ([a, b], weight) in &mut report.creases {
        if round.remove(&key(*a, *b)).is_some() {
            *weight = 1.0;
        }
    }
    report.creases.extend(round.into_keys().map(|(a, b)| ([a, b], 1.0)));
    if !report.face_groups.is_empty() {
        report.face_groups = done.faces.iter().map(|&f| report.face_groups[f as usize]).collect();
    }
    report.merged_faces += done.made.iter().filter(|made| **made).count();
    report.stats = done.mesh.stats();
    done.mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::primitives::{blocks_quads, cube_quads, plane_quads};

    /// A full crease on every edge whose two faces do not face the same
    /// way.
    fn folds(mesh: &PolyMesh) -> Vec<Crease> {
        let facing = |face: &Vec<u32>| {
            let at = |k: usize| mesh.positions[face[k] as usize];
            (1..face.len() - 1).map(|k| (at(k) - at(0)).cross(at(k + 1) - at(0))).sum::<Vec3>().normalize()
        };
        let mut sides: BTreeMap<Edge, Vec<Vec3>> = BTreeMap::new();
        for face in &mesh.faces {
            for (a, b) in steps(face) {
                sides.entry(key(a, b)).or_default().push(facing(face));
            }
        }
        sides
            .into_iter()
            .filter(|(_, n)| n.len() == 2 && n[0].dot(n[1]) < 0.9)
            .map(|((a, b), _)| ([a, b], 1.0))
            .collect()
    }

    #[test]
    fn a_flat_sheet_is_one_face_with_every_vertex_of_its_border() {
        let done = FlatNgons::default().merge(&plane_quads(5), &[], &[], &[]);
        assert_eq!(done.mesh.faces.len(), 1);
        assert_eq!((done.mesh.faces[0].len(), done.mesh.positions.len()), (20, 20));
    }

    #[test]
    fn a_cube_is_six_faces_and_keeps_the_vertices_of_its_edges() {
        let cube = cube_quads(4);
        let done = FlatNgons::default().merge(&cube, &folds(&cube), &[], &[]);
        assert_eq!(done.mesh.faces.len(), 6);
        assert!(done.mesh.faces.iter().all(|face| face.len() == 16));
        // 8 corners and 3 vertices inside each of the 12 edges.
        assert_eq!(done.mesh.positions.len(), 8 + 12 * 3);
        assert_eq!(done.mesh.stats().euler_characteristic(), 2);
    }

    #[test]
    fn without_creases_a_cube_is_left_as_it_is() {
        let cube = cube_quads(4);
        let done = FlatNgons::default().merge(&cube, &[], &[], &[]);
        assert_eq!((done.mesh.faces.len(), done.made.contains(&true)), (cube.faces.len(), false));
    }

    #[test]
    fn a_crease_that_is_not_full_does_not_cut_a_part_off() {
        let cube = cube_quads(4);
        let soft: Vec<Crease> = folds(&cube).into_iter().map(|(edge, _)| (edge, 0.5)).collect();
        let done = FlatNgons::default().merge(&cube, &soft, &[], &[]);
        assert!(!done.made.contains(&true));
    }

    #[test]
    fn an_l_is_cut_into_rectangles() {
        let block = blocks_quads(&[[0, 0, 0], [1, 0, 0], [0, 1, 0]], 3);
        let done = FlatNgons::default().merge(&block, &folds(&block), &[], &[]);
        // Six rectangles round the L, and its two L-shaped faces in two
        // rectangles each.
        assert_eq!(done.mesh.faces.len(), 6 + 2 * 2);
        assert_eq!(done.mesh.stats().euler_characteristic(), 2);
        assert_eq!(done.mesh.stats().non_manifold_edges, 0);
        // No vertex is left with two faces only, away from the creases.
        let creased: HashSet<u32> = folds(&done.mesh).iter().flat_map(|(edge, _)| *edge).collect();
        assert_eq!(done.mesh.positions.len(), creased.len());
    }

    #[test]
    fn nothing_is_merged_across_a_held_edge_or_two_groups() {
        let sheet = plane_quads(4);
        // The line of edges at x = 0, from border to border.
        let held: Vec<[u32; 2]> = (0..4).map(|j| [j * 5 + 2, (j + 1) * 5 + 2]).collect();
        let done = FlatNgons::default().merge(&sheet, &[], &held, &[]);
        assert_eq!(done.mesh.faces.len(), 2);
        let groups: Vec<u32> = (0..16).map(|f| (f % 4 >= 2) as u32).collect();
        let done = FlatNgons::default().merge(&sheet, &[], &[], &groups);
        assert_eq!(done.mesh.faces.len(), 2);
        assert_ne!(groups[done.faces[0] as usize], groups[done.faces[1] as usize]);
    }

    #[test]
    fn the_caps_of_a_cylinder_are_taken_out_and_a_cube_is_left_whole() {
        let cylinder = crate::mesh::primitives::cylinder(12);
        let opened = FlatNgons::default().open(&cylinder, 30.0, None).unwrap();
        assert_eq!(opened.lids.len(), 2);
        // Left: the side, open at both ends, every vertex of it kept.
        assert!(opened.mesh.positions.len() < cylinder.positions.len());
        assert!(
            opened.triangles.iter().all(|&t| { cylinder.corners(t as usize).iter().any(|p| p.z.abs() < 1.0 - 1e-9) })
        );
        assert_eq!(opened.vertices.len(), opened.mesh.positions.len());
        assert!(FlatNgons::default().open(&cube_quads(4).triangulated(), 30.0, None).is_none());
        // Two groups on a cap: it is left in.
        let groups: Vec<u32> = (0..cylinder.triangles.len() as u32).map(|t| t % 2).collect();
        assert!(FlatNgons::default().open(&cylinder, 30.0, Some(&groups)).is_none());
    }

    #[test]
    fn a_bent_sheet_is_left_as_quads() {
        let mut sheet = plane_quads(4);
        for p in &mut sheet.positions {
            p.z = 0.2 * p.x * p.x;
        }
        let done = FlatNgons::default().merge(&sheet, &[], &[], &[]);
        assert!(!done.made.contains(&true));
    }
}
