//! Flat parts as n-gons: an output apart from the quads, for a mesh with
//! fewer faces that subdivides the same.
//!
//! A face of many sides subdivides as the quads it stands for when it is
//! flat, cut off from the rest of the surface by sharp edges, and seen
//! whole from the middle of its corners, where the subdivision puts its
//! point, with room to spare: the subdivision then makes nothing but
//! points of its plane, inside its outline. A convex face always is. So the quads of a flat part of the mesh are merged where all
//! three hold:
//!
//! - the part is everything a walk across the edges reaches without
//!   crossing a full crease or leaving the mesh, and all of it lies in one
//!   plane. A flat that runs into a curve without a fold between them is
//!   left as quads, the whole part;
//! - if one face can stand for the part, it becomes that face: the face
//!   of a cube, an L. Otherwise its quads are merged two faces at a time
//!   as long as a face can stand for the two: the part round a hole comes
//!   out as sectors. A face can when it is convex or, if it is not, when
//!   its subdivision, made here in its plane, does not fold (see
//!   [`subdivides`]);
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
    /// Least crease weight that cuts a flat part off from its neighbours.
    pub sharp: f64,
    /// Least turn of a crease at a vertex, in degrees, for it to bend
    /// there; and of an outline for it not to be convex.
    pub bend: f64,
}

impl Default for FlatNgons {
    fn default() -> Self {
        Self { flatness: 1e-4, sharp: 0.999, bend: 0.1 }
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

    /// Whether the outline, a face with a full crease all round it, would
    /// subdivide without folding (see [`subdivides`]), three levels deep.
    fn subdivides(&self, positions: &[Vec3], outline: &[u32], corner: &[bool]) -> bool {
        let points: Vec<(f64, f64)> = outline.iter().map(|&v| self.flat(positions[v as usize])).collect();
        subdivides(&points, corner, 3)
    }

    fn off(&self, p: Vec3) -> f64 {
        (p - self.origin).dot(self.normal).abs()
    }

    fn flat(&self, p: Vec3) -> (f64, f64) {
        ((p - self.origin).dot(self.across.0), (p - self.origin).dot(self.across.1))
    }

    /// Whether the outline is convex, to within `slack` radians at each
    /// corner and twice that over all of them, turns once, and sees all of
    /// itself well from the mean of its vertices, where the subdivision
    /// puts the point of the face: no edge seen under a twentieth of the
    /// angle the edges are seen under on average.
    fn convex(&self, positions: &[Vec3], outline: &[u32], slack: f64) -> bool {
        let points: Vec<(f64, f64)> = outline.iter().map(|&v| self.flat(positions[v as usize])).collect();
        let n = points.len();
        let centre = points.iter().fold((0.0, 0.0), |(x, y), p| (x + p.0 / n as f64, y + p.1 / n as f64));
        let (mut turned, mut inwards) = (0.0, 0.0);
        let mut seen = Vec::with_capacity(n);
        for k in 0..n {
            let (p, q, r) = (points[k], points[(k + 1) % n], points[(k + 2) % n]);
            let (u, v) = ((q.0 - p.0, q.1 - p.1), (r.0 - q.0, r.1 - q.1));
            let turn = (u.0 * v.1 - u.1 * v.0).atan2(u.0 * v.0 + u.1 * v.1);
            if turn < -slack {
                return false;
            }
            turned += turn;
            inwards -= turn.min(0.0);
            seen.push((p.0 - centre.0) * (q.1 - centre.1) - (p.1 - centre.1) * (q.0 - centre.0));
        }
        let least = 0.05 * seen.iter().sum::<f64>() / n as f64;
        inwards <= 2.0 * slack && (turned - std::f64::consts::TAU).abs() < 0.5 && seen.iter().all(|&area| area > least)
    }
}

/// Whether a flat face with a full crease all round it subdivides without
/// folding: its Catmull-Clark subdivision is made, `levels` times over, in
/// its plane, and no quad of it may turn the other way. `points` are its
/// corners, turning counter-clockwise, and `corner` says which of them do
/// not move: those three creases or more meet at, and those with no other
/// edge than the two of the outline, which Blender's subdivision holds
/// where they are (found by comparing with it). The others slide along
/// the outline. A face cut off by creases is subdivided on its own,
/// whatever lies round it, so this is the whole of what happens to it.
fn subdivides(points: &[(f64, f64)], corner: &[bool], levels: usize) -> bool {
    type Point = (f64, f64);
    let mean = |points: &mut dyn Iterator<Item = Point>| {
        let (mut sum, mut count) = ((0.0, 0.0), 0.0);
        for p in points {
            sum = (sum.0 + p.0, sum.1 + p.1);
            count += 1.0;
        }
        (sum.0 / count, sum.1 / count)
    };
    let area = |points: &[Point], face: &[usize]| -> f64 {
        (0..face.len())
            .map(|k| {
                let (p, q) = (points[face[k]], points[face[(k + 1) % face.len()]]);
                0.5 * (p.0 * q.1 - p.1 * q.0)
            })
            .sum()
    };
    let n = points.len();
    let mut points = points.to_vec();
    let mut corner = corner.to_vec();
    let mut faces: Vec<Vec<usize>> = vec![(0..n).collect()];
    let mut sharp: HashSet<(usize, usize)> = (0..n).map(|k| (k.min((k + 1) % n), k.max((k + 1) % n))).collect();
    let whole = area(&points, &faces[0]);
    if whole <= 0.0 {
        return false;
    }
    for _ in 0..levels {
        let vertices = points.len();
        let middles: Vec<Point> = faces.iter().map(|face| mean(&mut face.iter().map(|&v| points[v]))).collect();
        // The edges, in the order the faces meet them, and the faces of
        // each.
        let mut index: HashMap<(usize, usize), usize> = HashMap::new();
        let mut edges: Vec<((usize, usize), Vec<usize>)> = Vec::new();
        let mut at: Vec<Vec<usize>> = vec![Vec::new(); vertices];
        let mut round: Vec<Vec<usize>> = vec![Vec::new(); vertices];
        for (f, face) in faces.iter().enumerate() {
            for k in 0..face.len() {
                let (a, b) = (face[k], face[(k + 1) % face.len()]);
                let edge = (a.min(b), a.max(b));
                let e = *index.entry(edge).or_insert_with(|| {
                    edges.push((edge, Vec::new()));
                    at[a].push(edges.len() - 1);
                    at[b].push(edges.len() - 1);
                    edges.len() - 1
                });
                edges[e].1.push(f);
                round[a].push(f);
            }
        }
        let mut next: Vec<Point> = Vec::with_capacity(vertices + faces.len() + edges.len());
        for v in 0..vertices {
            let p = points[v];
            let far = |e: usize| if edges[e].0.0 == v { edges[e].0.1 } else { edges[e].0.0 };
            let creases: Vec<usize> = at[v].iter().copied().filter(|&e| sharp.contains(&edges[e].0)).collect();
            next.push(if corner[v] || creases.len() > 2 {
                p
            } else if let &[a, b] = creases.as_slice() {
                let (a, b) = (points[far(a)], points[far(b)]);
                ((a.0 + 6.0 * p.0 + b.0) / 8.0, (a.1 + 6.0 * p.1 + b.1) / 8.0)
            } else {
                let valence = at[v].len() as f64;
                let f = mean(&mut round[v].iter().map(|&f| middles[f]));
                let r = mean(&mut at[v].iter().map(|&e| {
                    let q = points[far(e)];
                    (0.5 * (p.0 + q.0), 0.5 * (p.1 + q.1))
                }));
                (
                    (f.0 + 2.0 * r.0 + (valence - 3.0) * p.0) / valence,
                    (f.1 + 2.0 * r.1 + (valence - 3.0) * p.1) / valence,
                )
            });
        }
        next.extend(&middles);
        for ((a, b), beside) in &edges {
            let (p, q) = (points[*a], points[*b]);
            next.push(match beside.as_slice() {
                &[f, g] if !sharp.contains(&(*a, *b)) => {
                    let (f, g) = (middles[f], middles[g]);
                    ((p.0 + q.0 + f.0 + g.0) / 4.0, (p.1 + q.1 + f.1 + g.1) / 4.0)
                }
                _ => (0.5 * (p.0 + q.0), 0.5 * (p.1 + q.1)),
            });
        }
        let on = |a: usize, b: usize| vertices + faces.len() + index[&(a.min(b), a.max(b))];
        let mut quads = Vec::with_capacity(4 * faces.len());
        for (f, face) in faces.iter().enumerate() {
            for k in 0..face.len() {
                let (before, v, after) = (face[(k + face.len() - 1) % face.len()], face[k], face[(k + 1) % face.len()]);
                quads.push(vec![v, on(v, after), vertices + f, on(before, v)]);
            }
        }
        sharp = sharp
            .iter()
            .flat_map(|&(a, b)| {
                let e = on(a, b);
                [(a, e), (b, e)]
            })
            .collect();
        corner.resize(next.len(), false);
        points = next;
        faces = quads;
        // Where the vertices end up if the subdivision went on for ever:
        // what Blender shows by default (Use Limit Surface). The faces are
        // all quads now; a vertex inside them goes to the mask of Halstead,
        // Kass and DeRose, one on the outline to the cubic B-spline of the
        // outline, and a corner stays.
        let mut ends: Vec<Vec<usize>> = vec![Vec::new(); points.len()];
        for &(a, b) in &sharp {
            ends[a].push(b);
            ends[b].push(a);
        }
        let mut beside = vec![((0.0, 0.0), (0.0, 0.0), 0.0); points.len()];
        for face in &faces {
            for k in 0..4 {
                let (edge, facing, count) = &mut beside[face[k]];
                for neighbour in [face[(k + 1) % 4], face[(k + 3) % 4]] {
                    *edge = (edge.0 + 0.5 * points[neighbour].0, edge.1 + 0.5 * points[neighbour].1);
                }
                *facing = (facing.0 + points[face[(k + 2) % 4]].0, facing.1 + points[face[(k + 2) % 4]].1);
                *count += 1.0;
            }
        }
        let limit: Vec<Point> = (0..points.len())
            .map(|v| {
                let p = points[v];
                match *ends[v].as_slice() {
                    _ if corner[v] => p,
                    [a, b] => {
                        let (a, b) = (points[a], points[b]);
                        ((a.0 + 4.0 * p.0 + b.0) / 6.0, (a.1 + 4.0 * p.1 + b.1) / 6.0)
                    }
                    [] => {
                        let (edge, facing, n) = beside[v];
                        let all = n * (n + 5.0);
                        ((n * n * p.0 + 4.0 * edge.0 + facing.0) / all, (n * n * p.1 + 4.0 * edge.1 + facing.1) / all)
                    }
                    _ => p,
                }
            })
            .collect();
        let least = 1e-12 * whole;
        if faces.iter().any(|face| area(&points, face) <= least || area(&limit, face) <= least) {
            return false;
        }
    }
    true
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

        // The edges at each vertex, by their other end, and those of them
        // that are sharp whatever is merged: the full creases, and the
        // edges with one face only.
        let mut edges_at: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut sharp_at: HashMap<u32, Vec<u32>> = HashMap::new();
        for (&(a, b), _) in owner.iter().filter(|&(&(a, b), _)| a < b || !owner.contains_key(&(b, a))) {
            let one_face = across(&owner, (a, b)).is_none() || !owner.contains_key(&(b, a));
            for (v, w) in [(a, b), (b, a)] {
                edges_at.entry(v).or_default().push(w);
                if one_face || cut.contains(&key(a, b)) {
                    sharp_at.entry(v).or_default().push(w);
                }
            }
        }
        // Whether a face may be made with this outline: convex, or found to
        // subdivide without folding. Which of its vertices are corners,
        // where a third crease ends, depends on the faces made round it,
        // not all known yet: it must hold with the fewest there can be,
        // those the creases of the mesh make, and with the most, every
        // vertex another edge is left at. `gone` are the edges merged
        // away, `inner` those this face would merge away.
        let bend = self.bend.to_radians();
        let may_be = |plane: &Plane, outline: &[u32], gone: &HashSet<Edge>, inner: &HashSet<Edge>| -> bool {
            if plane.convex(positions, outline, bend) {
                return true;
            }
            let n = outline.len();
            let third = |k: usize, among: &HashMap<u32, Vec<u32>>| {
                let (before, v, after) = (outline[(k + n - 1) % n], outline[k], outline[(k + 1) % n]);
                among.get(&v).is_some_and(|ends| {
                    ends.iter().any(|&w| {
                        w != before && w != after && !gone.contains(&key(v, w)) && !inner.contains(&key(v, w))
                    })
                })
            };
            // A vertex slides along the outline only if another edge ends
            // there and none that does is a crease.
            let fewest: Vec<bool> = (0..n).map(|k| third(k, &sharp_at) || !third(k, &edges_at)).collect();
            plane.subdivides(positions, outline, &fewest) && plane.subdivides(positions, outline, &vec![true; n])
        };

        let mut alive = vec![true; faces.len()];
        let mut made = vec![false; faces.len()];
        let mut planes: Vec<Option<Plane>> = vec![None; parts.len()];
        let mut gone: HashSet<Edge> = HashSet::new();
        for members in &parts {
            if members.len() < 2 {
                continue;
            }
            let Some(plane) = self.plane_of(positions, &faces, members, tolerance) else { continue };
            planes[part[members[0]]] = Some(plane.clone());

            // All of it in one face, if its outline is convex.
            let inside = |f: usize| part[f] == part[members[0]];
            let mut outline = Vec::new();
            let mut inner: HashSet<Edge> = HashSet::new();
            let mut whole = true;
            for &f in members {
                for (a, b) in steps(&faces[f]) {
                    match across(&owner, (a, b)).filter(|&g| inside(g)) {
                        Some(_) => {
                            whole &= !kept.contains(&key(a, b));
                            inner.insert(key(a, b));
                        }
                        None => outline.push((a, b)),
                    }
                }
            }
            if whole && let Some(outline) = single_loop(&outline).filter(|o| may_be(&plane, o, &gone, &inner)) {
                for &f in &members[1..] {
                    alive[f] = false;
                }
                faces[members[0]] = outline;
                made[members[0]] = true;
                continue;
            }

            // Otherwise two faces at a time, as long as the two make a
            // face that may be.
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
                            let inner: HashSet<Edge> =
                                steps(&faces[f]).filter(shared).map(|(a, b)| key(a, b)).collect();
                            may_be(&plane, &outline, &gone, &inner).then_some((g, outline))
                        });
                        let Some((g, outline)) = joined else { break };
                        for edge in steps(&faces[g]) {
                            owner.insert(edge, f);
                        }
                        let merged: Vec<Edge> =
                            steps(&faces[f]).filter(|&(a, b)| owner.get(&(b, a)) == Some(&f)).collect();
                        for (a, b) in merged {
                            owner.remove(&(a, b));
                            owner.remove(&(b, a));
                            gone.insert(key(a, b));
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
                let none = HashSet::new();
                if without.iter().all(|outline| outline.len() >= 3 && may_be(&plane, outline, &gone, &none)) {
                    for (f, outline) in users.into_iter().zip(without) {
                        faces[f] = outline;
                    }
                }
            }
        }

        // The faces made are now all known, and so are the corners of
        // each: every edge of a face made is a crease. A face found to fold
        // after all sends its whole part back to its quads, and the others
        // are looked at again, their corners being fewer for it.
        loop {
            let mut creases_at: HashMap<u32, HashSet<u32>> = HashMap::new();
            let mut ends_at: HashMap<u32, HashSet<u32>> = HashMap::new();
            for (&v, ends) in &sharp_at {
                creases_at.entry(v).or_default().extend(ends);
            }
            for f in (0..faces.len()).filter(|&f| alive[f]) {
                for (a, b) in steps(&faces[f]) {
                    ends_at.entry(a).or_default().insert(b);
                    ends_at.entry(b).or_default().insert(a);
                    if made[f] {
                        creases_at.entry(a).or_default().insert(b);
                        creases_at.entry(b).or_default().insert(a);
                    }
                }
            }
            let folds = |f: usize| {
                let (outline, plane) = (&faces[f], planes[part[f]].as_ref().unwrap());
                let n = outline.len();
                let corner: Vec<bool> = (0..n)
                    .map(|k| {
                        let (before, after) = (outline[(k + n - 1) % n], outline[(k + 1) % n]);
                        let third = |among: &HashMap<u32, HashSet<u32>>| {
                            among[&outline[k]].iter().any(|&w| w != before && w != after)
                        };
                        third(&creases_at) || !third(&ends_at)
                    })
                    .collect();
                !plane.convex(positions, outline, bend) && !plane.subdivides(positions, outline, &corner)
            };
            let back: HashSet<usize> =
                (0..faces.len()).filter(|&f| alive[f] && made[f] && folds(f)).map(|f| part[f]).collect();
            if back.is_empty() {
                break;
            }
            for &f in back.iter().flat_map(|&p| &parts[p]) {
                faces[f] = mesh.faces[f].clone();
                (alive[f], made[f]) = (true, false);
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

/// What lies across an edge of the outline of a [`Lid`]: what is left of
/// the input.
const LEFT: usize = usize::MAX;

/// A flat part of the input taken out of it before the chain: its plane,
/// its outline as the input has it (vertices of the input, and where they
/// are), what lies across each edge of it, from each vertex to the next,
/// another lid or what is left of the input, and the group of its
/// triangles.
#[derive(Clone, Debug)]
struct Lid {
    plane: Plane,
    outline: Vec<u32>,
    places: Vec<Vec3>,
    beside: Vec<usize>,
    group: u32,
}

/// An input with flat parts taken out (see [`FlatNgons::open`]).
#[derive(Clone, Debug)]
pub struct Opened {
    /// What is left of the input, open where the parts were: nothing at
    /// all for a shape made of such parts only, a cube.
    pub mesh: TriMesh,
    /// The vertex and the triangle of the input each of those of `mesh`
    /// was.
    pub vertices: Vec<u32>,
    pub triangles: Vec<u32>,
    lids: Vec<Lid>,
    tolerance: f64,
    bend: f64,
}

impl FlatNgons {
    /// `input` without those of its flat parts that one face can stand
    /// for, so that the chain does not mesh them at all: the quads round a
    /// cylinder are then those its side calls for, not those its caps can
    /// take, and a cube needs no chain. Such a part lies in one plane and
    /// has for its outline one convex loop of edges folded by more than
    /// `angle` degrees. `None` if there is none.
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
                (closed && flat && one_group && plane.convex(positions, &outline, self.bend.to_radians())).then(|| {
                    let places = outline.iter().map(|&v| positions[v as usize]).collect();
                    Lid { plane, outline, places, beside: Vec::new(), group }
                })
            };
            lids.push(lid());
        }
        // What lies across each edge of each outline. Two parts back to
        // back, with all of their outline in common, are a sheet with no
        // inside: left in.
        for l in 0..lids.len() {
            let Some(outline) = lids[l].as_ref().map(|lid| lid.outline.clone()) else { continue };
            let beside: Vec<usize> = steps(&outline).map(|(a, b)| part[owner[&(b, a)]]).collect();
            if beside.iter().all(|&other| other == beside[0]) && lids[beside[0]].is_some() {
                lids[l] = None;
                continue;
            }
            lids[l].as_mut().unwrap().beside = beside;
        }
        let mut number = vec![LEFT; lids.len()];
        let mut kept = 0;
        for (l, _) in lids.iter().enumerate().filter(|(_, lid)| lid.is_some()) {
            number[l] = kept;
            kept += 1;
        }
        if kept == 0 {
            return None;
        }
        let out: Vec<bool> = lids.iter().map(Option::is_some).collect();
        let mut opened = Opened {
            mesh: TriMesh::default(),
            vertices: Vec::new(),
            triangles: Vec::new(),
            lids: lids.into_iter().flatten().collect(),
            tolerance,
            bend: self.bend.to_radians(),
        };
        for lid in &mut opened.lids {
            for other in &mut lid.beside {
                *other = number[*other];
            }
        }
        let mut became = vec![u32::MAX; positions.len()];
        for (t, triangle) in input.triangles.iter().enumerate().filter(|&(t, _)| !out[part[t]]) {
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
    /// face on each of the parts taken out. Along what was left, a face
    /// takes the vertices of the border the chain made there. Between two
    /// parts taken out the edge is straight, and has no vertex but its two
    /// ends: where the neighbour of a part changes, a vertex of the border
    /// if the chain made one there, one of the input otherwise. What
    /// `report` says of the mesh is said of the result. `None` if a face
    /// cannot be made so, flat and convex: the mesh is then not to be
    /// used.
    pub(crate) fn close(&self, mut mesh: PolyMesh, report: &mut crate::pipeline::Report) -> Option<PolyMesh> {
        // The border of the mesh, walked the other way: as the faces that
        // close it turn.
        let walked: HashSet<Edge> = mesh.faces.iter().flat_map(|face| steps(face)).collect();
        let mut after: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for &(a, b) in walked.iter().filter(|&&(a, b)| !walked.contains(&(b, a))) {
            after.entry(b).or_default().push(a);
        }

        // The ends of the stretches of each outline: where what lies
        // across changes. One on what is left is a vertex of the border.
        let mut on_the_border: BTreeMap<u32, (Vec3, bool)> = BTreeMap::new();
        for lid in &self.lids {
            let n = lid.outline.len();
            for k in (0..n).filter(|&k| lid.beside[k] != lid.beside[(k + n - 1) % n]) {
                let left = lid.beside[k] == LEFT || lid.beside[(k + n - 1) % n] == LEFT;
                on_the_border.entry(lid.outline[k]).or_insert((lid.places[k], false)).1 |= left;
            }
        }
        let mut ends: HashMap<u32, u32> = HashMap::new();
        let mut used: HashSet<u32> = HashSet::new();
        for (&v, &(place, left)) in &on_the_border {
            let vertex = if left {
                let nearest = after.keys().copied().min_by(|&a, &b| {
                    let far = |v: u32| mesh.positions[v as usize].distance_squared(place);
                    far(a).total_cmp(&far(b))
                })?;
                if mesh.positions[nearest as usize].distance(place) > 10.0 * self.tolerance || !used.insert(nearest) {
                    return None;
                }
                nearest
            } else {
                mesh.positions.push(place);
                (mesh.positions.len() - 1) as u32
            };
            ends.insert(v, vertex);
        }
        let is_end: HashSet<u32> = ends.values().copied().collect();

        // A whole border, for an outline with what is left all round it.
        let mut borders: Vec<Vec<u32>> = Vec::new();
        let mut seen: HashSet<u32> = HashSet::new();
        for (&start, next) in &after {
            if next.len() != 1 || seen.contains(&start) {
                continue;
            }
            let mut border = vec![start];
            let mut at = next[0];
            while at != start && border.len() <= after.len() {
                border.push(at);
                at = match after.get(&at).map(Vec::as_slice) {
                    Some(&[next]) => next,
                    _ => start,
                };
            }
            if steps(&border).all(|(a, b)| after[&a].contains(&b)) {
                seen.extend(&border);
                borders.push(border);
            }
        }
        let mut taken = vec![false; borders.len()];

        let mut creased: HashSet<Edge> = HashSet::new();
        for lid in &self.lids {
            let n = lid.outline.len();
            let flat = |v: &u32| lid.plane.off(mesh.positions[*v as usize]) <= self.tolerance;
            let stops: Vec<usize> = (0..n).filter(|&k| ends.contains_key(&lid.outline[k])).collect();
            let face = if stops.is_empty() {
                // The border in the plane of the lid whose middle is inside
                // its outline.
                let inside = |p: Vec3| {
                    let p = lid.plane.flat(p);
                    (0..n).all(|k| {
                        let (a, b) = (lid.plane.flat(lid.places[k]), lid.plane.flat(lid.places[(k + 1) % n]));
                        (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0) >= 0.0
                    })
                };
                let fits = |border: &Vec<u32>| {
                    let middle = border.iter().map(|&v| mesh.positions[v as usize]).sum::<Vec3>();
                    border.iter().all(flat) && inside(middle / border.len() as f64)
                };
                let mut fitting = (0..borders.len()).filter(|&b| fits(&borders[b]));
                let (b, None) = (fitting.next()?, fitting.next()) else { return None };
                if std::mem::replace(&mut taken[b], true) {
                    return None;
                }
                borders[b].clone()
            } else {
                let mut face = Vec::new();
                for (i, &k) in stops.iter().enumerate() {
                    let (from, to) = (ends[&lid.outline[k]], ends[&lid.outline[stops[(i + 1) % stops.len()]]]);
                    face.push(from);
                    if lid.beside[k] != LEFT {
                        continue;
                    }
                    // Along the border of what is left, to the next end.
                    let run = after.get(&from)?.iter().find_map(|&first| {
                        let mut run = Vec::new();
                        let mut at = first;
                        while at != to {
                            if is_end.contains(&at) || !flat(&at) || run.len() > after.len() {
                                return None;
                            }
                            run.push(at);
                            at = match after.get(&at).map(Vec::as_slice) {
                                Some(&[next]) => next,
                                _ => return None,
                            };
                        }
                        Some(run)
                    })?;
                    face.extend(run);
                }
                face
            };
            let once: HashSet<u32> = face.iter().copied().collect();
            // Convex, or found to subdivide without folding, whether the
            // vertices the chain made on it slide along the outline or not.
            let stops: Vec<bool> = face.iter().map(|v| is_end.contains(v)).collect();
            let sound = lid.plane.convex(&mesh.positions, &face, self.bend)
                || (lid.plane.subdivides(&mesh.positions, &face, &stops)
                    && lid.plane.subdivides(&mesh.positions, &face, &vec![true; face.len()]));
            if face.len() < 3 || once.len() != face.len() || !sound {
                return None;
            }
            for (a, b) in steps(&face) {
                if creased.insert(key(a, b)) {
                    report.creases.push(([a, b], 1.0));
                    report.hard_edges.push([a, b]);
                }
            }
            if !report.face_groups.is_empty() {
                report.face_groups.push(lid.group);
            }
            mesh.faces.push(face);
        }
        let stats = mesh.stats();
        if stats.non_manifold_edges > 0 {
            return None;
        }
        report.merged_faces += self.lids.len();
        report.stats = stats;
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
    fn an_l_is_one_face_where_that_subdivides_and_two_rectangles_where_not() {
        // Three cubes in an L: the middle of the corners of an L-shaped
        // face is too close to its inward corner, and it is cut in two.
        let block = blocks_quads(&[[0, 0, 0], [1, 0, 0], [0, 1, 0]], 3);
        let done = FlatNgons::default().merge(&block, &folds(&block), &[], &[]);
        assert_eq!(done.mesh.faces.len(), 6 + 2 * 2);
        assert_eq!(done.mesh.stats().euler_characteristic(), 2);
        assert_eq!(done.mesh.stats().non_manifold_edges, 0);
        // No vertex is left away from the creases.
        let creased: HashSet<u32> = folds(&done.mesh).iter().flat_map(|(edge, _)| *edge).collect();
        assert_eq!(done.mesh.positions.len(), creased.len());
    }

    #[test]
    fn the_subdivision_of_a_face_is_followed_in_its_plane() {
        let free = |points: &[(f64, f64)]| subdivides(points, &vec![false; points.len()], 3);
        let held = |points: &[(f64, f64)]| subdivides(points, &vec![true; points.len()], 3);
        // A square, and an L wide enough for the middle of its corners to be
        // well inside it.
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let l = [(0.0, 0.0), (3.0, 0.0), (3.0, 2.0), (2.0, 2.0), (2.0, 3.0), (0.0, 3.0)];
        assert_eq!([free(&square), held(&square), free(&l)], [true; 3]);
        // With so few corners, and the inward one held where it is, the
        // same L folds.
        assert!(!held(&l));
        // The L of three squares: that middle is its inward corner itself.
        let thin = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
        assert_eq!([free(&thin), held(&thin)], [false; 2]);
        // A U: the middle of its corners is in the gap between its arms.
        let u = [(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (2.0, 3.0), (2.0, 1.0), (1.0, 1.0), (1.0, 3.0), (0.0, 3.0)];
        assert!(!free(&u) && !held(&u));
        // Turning the other way is no face at all.
        let mut back = square;
        back.reverse();
        assert!(!free(&back));
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
    fn the_caps_of_a_cylinder_are_taken_out_and_all_of_a_cube() {
        let cylinder = crate::mesh::primitives::cylinder(12);
        let opened = FlatNgons::default().open(&cylinder, 30.0, None).unwrap();
        assert_eq!(opened.lids.len(), 2);
        // Left: the side, open at both ends, every vertex of it kept.
        assert!(opened.mesh.positions.len() < cylinder.positions.len());
        assert!(
            opened.triangles.iter().all(|&t| { cylinder.corners(t as usize).iter().any(|p| p.z.abs() < 1.0 - 1e-9) })
        );
        assert_eq!(opened.vertices.len(), opened.mesh.positions.len());
        // A cube is nothing but such parts: nothing is left for the chain,
        // and its faces are closed from its corners alone.
        let cube = FlatNgons::default().open(&cube_quads(4).triangulated(), 30.0, None).unwrap();
        assert_eq!((cube.lids.len(), cube.mesh.triangles.len()), (6, 0));
        let mut report = crate::pipeline::Report::default();
        let closed = cube.close(PolyMesh::default(), &mut report).unwrap();
        assert_eq!((closed.positions.len(), closed.faces.len(), report.creases.len()), (8, 6, 12));
        assert!(closed.faces.iter().all(|face| face.len() == 4));
        assert_eq!((closed.stats().euler_characteristic(), closed.stats().boundary_edges), (2, 0));
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
