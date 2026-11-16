//! The reflection repair (#329): a cage made by hand cleaned up in place,
//! judged by the score of the reflection check before and after.
//!
//! The repair keeps every vertex, edge and face. It moves vertices, to
//! flatten the n-gons off their plane and to make the faces whose corner
//! quads fold or are thin star-shaped round their centre again; and it
//! proposes creases on the edges that are already sharp edges of the shape,
//! where they cure a pinch or a pole on a curved zone. A crease is only
//! proposed: whether it is set is the caller's choice.
//!
//! The repairs fight one another: flattening a face bends the faces round
//! it, a crease cures a pinch but makes an edge sharp. So every repair is
//! tried, the check run again where it changed something, and the repair
//! kept only if the result is better by [`Badness::compare`], an order that
//! no trade between kinds of fault can cheat (see [`Badness`]).
//!
//! Moves. A vertex is shared, so no face is moved alone, and a move must
//! not undo what is right round it. Each vertex of the face moves only in
//! the directions that keep flat the n-gons round it that are flat (square
//! to their normals): free where there is none, in a plane where there is
//! one, along a line where there are two, not at all where there are three.
//! Within that:
//!
//! - an n-gon off its plane is projected onto the plane through its centre
//!   square to its normal, every vertex at once; or, where all its vertices
//!   but one lie in a plane, that one alone is brought into it, which is the
//!   repair of a vertex pulled out by mistake;
//! - a face whose corner quads fold or are thin gets the smallest move (in
//!   the sum of the squares of the displacements), in its own plane, that
//!   brings every triangle (centre, edge) to a given share of the mean one,
//!   the centre moving with the vertices; the constraint is not linear, so
//!   it is linearised and solved again until it holds (Gauss-Newton), each
//!   step by the normal equations of the smallest move.
//!
//! No vertex moves further from where it was given than
//! [`RepairSettings::bound`] times the shortest of its edges: a repair that
//! needs more is refused, and the fault is left with
//! [`Reason::TooFar`].
//!
//! Passes. A pass runs the check, then tries the faults that moving
//! vertices mends, in the order of [`super::rank`], each against the cage as
//! the repairs before it in the pass left it. The repair stops after a pass
//! that kept nothing, or after [`RepairSettings::passes`] passes. Every
//! repair kept makes the badness strictly smaller, and everything runs one
//! repair after the other in a fixed order: the result is the same from one
//! run to the next.
//!
//! What cannot be repaired in place, a pinch or a pole on a curved zone, is
//! left as it is, with [`Reason::ChangeTopology`], and so is an n-gon off
//! its plane that would still pinch once flat: it is on a curved zone, what
//! takes it off its plane is the shape, and a flattening is refused even
//! where the score would take it (an octagon on a cylinder, flattened,
//! pinches less, because the cylinder is flattened under it).
//!
//! A second run on the result changes nothing where every repair kept
//! mended its fault whole. Where one stopped at the bound part way (a fold
//! mended, the face left thin), a second run, whose bound is on the cage it
//! is given, may go a bound further.
//!
//! Every angle is that of the check, measured on the cage, not on the
//! subdivided surface: "repaired" means by that measure.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use super::{Cage, Element, Fault, Kind, Repair, Score, Settings, check, key, newell};
use crate::creases::Crease;
use crate::linalg::solve_dense;
use crate::math::{Vec3, tangent_frame};
use crate::mesh::PolyMesh;

/// Differences of a sum of angles, in degrees, and of a sum of shares,
/// below which two badnesses are the same: rounding, not a change.
const ANGLE_EPSILON: f64 = 1e-6;
const SHARE_EPSILON: f64 = 1e-9;
/// A normal closer than this to the span of others (the sine of the angle,
/// about one degree) is taken as in it: two faces in the same plane hold a
/// vertex once.
const SAME_PLANE: f64 = 0.0175;
/// Steps of the Gauss-Newton solve that makes a face star-shaped.
const STEPS: usize = 50;

#[derive(Clone, Debug, PartialEq)]
pub struct RepairSettings {
    /// What counts as a fault.
    pub check: Settings,
    /// The largest displacement of a vertex, as a share of the shortest of
    /// its edges in the cage given.
    pub bound: f64,
    /// The fold, in degrees, between the two faces of an edge from which it
    /// is a sharp edge of the shape, where a crease may be proposed.
    pub crease_fold: f64,
    /// The most passes.
    pub passes: usize,
}

impl Default for RepairSettings {
    fn default() -> Self {
        Self { check: Settings::default(), bound: 0.1, crease_fold: 30.0, passes: 8 }
    }
}

/// How bad a cage is, by what the reflection check finds, in the terms the
/// repair is judged by. Each field is a sum over faults, so that the
/// badness of a cage is the sum of those of its parts, and a change is
/// judged on the faces and vertices it touches alone.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Badness {
    /// Faces of no area.
    pub degenerate: usize,
    /// Faces whose corner quads fold.
    pub folded: usize,
    /// The sum of the angles, in degrees, of the pinches and the poles on a
    /// curved zone.
    pub curved: f64,
    /// The sum of the angles, in degrees, of the n-gons off their plane.
    pub not_flat: f64,
    /// Thin faces, and the sum of what each lacks to the thin ratio.
    pub thin: usize,
    pub thinness: f64,
}

impl Badness {
    pub fn of<'a>(faults: impl IntoIterator<Item = &'a Fault>, thin_ratio: f64) -> Self {
        let mut badness = Self::default();
        for fault in faults {
            match fault.kind {
                Kind::Degenerate => badness.degenerate += 1,
                Kind::Folded => badness.folded += 1,
                Kind::Pinch | Kind::CurvedPole => badness.curved += fault.value,
                Kind::NotFlat => badness.not_flat += fault.value,
                Kind::Thin => {
                    badness.thin += 1;
                    badness.thinness += (thin_ratio - fault.value).max(0.0);
                }
            }
        }
        badness
    }

    /// The order the repair is judged by, lexicographic, worse last: faces
    /// of no area; folded faces; the angles of what moving vertices does not
    /// mend (pinches, poles on a curved zone); the angles of the n-gons off
    /// their plane; thin faces, then how thin.
    ///
    /// Counts come before angles, so a folded face is never traded for a
    /// smaller angle elsewhere, however large: a fold is the subdivided
    /// surface turned over, wrong whatever its size. The angles of a pinch
    /// and of a pole come before those of a face off its plane: these are
    /// what the topology is to blame for, and a move that makes them worse
    /// to flatten a face is flattening the shape round it, or bending it
    /// further. Thin faces come last, as in the check: a warning.
    pub fn compare(&self, other: &Self) -> Ordering {
        compare_terms(&self.terms(), &other.terms())
    }

    /// The fields in the order they are compared in, counts as numbers.
    fn terms(&self) -> [f64; 6] {
        [self.degenerate as f64, self.folded as f64, self.curved, self.not_flat, self.thin as f64, self.thinness]
    }

    pub fn is_better_than(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Less
    }
}

/// Why a fault is left after the repair.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reason {
    /// A pinch, a pole on a curved zone, a face of no area, or an n-gon off
    /// its plane on a curved zone: moving vertices does not mend it.
    ChangeTopology,
    /// Mending it would move a vertex further than the bound.
    TooFar,
    /// Every move tried would make the score worse, or none keeps flat the
    /// n-gons round it that are.
    WouldWorsen,
    /// A proposed crease cures it (gone once they are set).
    ProposedCrease,
    /// Not on the faces the repair was asked for.
    OutsideSelection,
    /// It appeared in the last pass, which left no pass to try it.
    OutOfPasses,
}

impl Reason {
    pub const ALL: [Reason; 6] = [
        Reason::ChangeTopology,
        Reason::TooFar,
        Reason::WouldWorsen,
        Reason::ProposedCrease,
        Reason::OutsideSelection,
        Reason::OutOfPasses,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Reason::ChangeTopology => "change_topology",
            Reason::TooFar => "too_far",
            Reason::WouldWorsen => "would_worsen",
            Reason::ProposedCrease => "proposed_crease",
            Reason::OutsideSelection => "outside_selection",
            Reason::OutOfPasses => "out_of_passes",
        }
    }
}

/// A fault left, and why.
#[derive(Clone, Debug, PartialEq)]
pub struct Left {
    pub fault: Fault,
    pub reason: Reason,
}

/// A crease the repair proposes, and what for.
#[derive(Clone, Debug, PartialEq)]
pub struct ProposedCrease {
    /// The edge, by its two vertices, the smaller first.
    pub edge: [u32; 2],
    /// The fold, in degrees, between its two faces.
    pub fold: f64,
    /// The fault it is proposed for, as it was before: a pinch or a pole on
    /// a curved zone, and its angle in degrees.
    pub kind: Kind,
    pub at: Element,
    pub value: f64,
    /// Whether that fault is gone once every proposed crease is set.
    pub cured: bool,
}

/// What [`repair`] gives back.
#[derive(Clone, Debug, PartialEq)]
pub struct Repaired {
    /// The positions of the vertices, numbered as given; those not moved
    /// are the very numbers given.
    pub positions: Vec<Vec3>,
    /// How many vertices moved, how far the furthest did, and the largest
    /// displacement over the shortest edge of its vertex.
    pub moved: usize,
    pub largest_displacement: f64,
    pub largest_share: f64,
    /// The creases proposed, in the order they were found; none is set.
    pub creases: Vec<ProposedCrease>,
    /// The score of the cage given; after the moves, with the creases
    /// given; and after the moves with the proposed creases set too.
    pub before: Score,
    pub after: Score,
    pub with_creases: Score,
    /// The same, as the repair judges them.
    pub badness_before: Badness,
    pub badness_after: Badness,
    pub badness_with_creases: Badness,
    /// The faults after the moves, with the creases given, worst first,
    /// each with why it is left.
    pub left: Vec<Left>,
    /// The passes run, and the repairs kept.
    pub passes: usize,
    pub repairs: usize,
}

/// A move: vertices and where they go.
type Move = Vec<(u32, Vec3)>;

/// Repairs `mesh` in place, to be subdivided with `creases`: the faults of
/// the reflection check that moving vertices mends are mended where it
/// makes the score better, and creases are proposed where they cure a pinch
/// or a pole on a curved zone (see the module). With `within`, only those
/// faces are repaired: only their vertices move, only their edges get a
/// crease; the rest of the cage counts in the score all the same.
pub fn repair(mesh: &PolyMesh, creases: &[Crease], within: Option<&[u32]>, settings: &RepairSettings) -> Repaired {
    let checking = &settings.check;
    let thin = checking.thin_ratio;
    let tolerance = checking.angle_tolerance.to_radians();
    let before = check(mesh, creases, checking);

    let mut cage = Cage::build(mesh, creases, checking.crease_sharp);
    let vertices = mesh.positions.len();
    // The faces asked for, and the vertices that may move.
    let mut chosen = vec![within.is_none(); mesh.faces.len()];
    let mut movable = vec![within.is_none(); vertices];
    for &f in within.unwrap_or(&[]) {
        if let Some(face) = mesh.faces.get(f as usize) {
            chosen[f as usize] = true;
            for &v in face {
                movable[v as usize] = true;
            }
        }
    }
    // How far each vertex may go: a share of the shortest of its edges.
    let shortest: Vec<f64> = (0..vertices as u32)
        .map(|v| {
            let p = mesh.positions[v as usize];
            cage.neighbours(v).iter().map(|&u| (mesh.positions[u as usize] - p).length()).fold(f64::INFINITY, f64::min)
        })
        .collect();
    let reach: Vec<f64> = shortest.iter().map(|&l| if l.is_finite() { settings.bound * l } else { 0.0 }).collect();

    let mut outcome: BTreeMap<(u32, Kind), Reason> = BTreeMap::new();
    let (mut passes, mut repairs) = (0, 0);
    while passes < settings.passes {
        passes += 1;
        let found = cage.check(checking);
        let wanted: Vec<(u32, Kind)> = found
            .faults
            .iter()
            .filter(|f| f.kind.repair() == Repair::MoveVertices)
            .filter_map(|f| match f.at {
                Element::Face(face) if chosen[face as usize] => Some((face, f.kind)),
                _ => None,
            })
            .collect();
        let mut kept = 0;
        for (f, kind) in wanted {
            let mut own = Vec::new();
            cage.check_face(f as usize, checking, tolerance, &mut own);
            if !own.iter().any(|fault| fault.kind == kind) {
                // Mended on the way by a repair of another face.
                outcome.remove(&(f, kind));
                continue;
            }
            let candidates: Vec<Option<Move>> = match kind {
                Kind::NotFlat => cage.flattenings(f as usize, checking),
                Kind::Folded => vec![
                    cage.straightened(f as usize, 1.1 * thin, checking),
                    cage.straightened(f as usize, 0.2 * thin, checking),
                ],
                _ => vec![cage.straightened(f as usize, 1.1 * thin, checking)],
            };
            let (mut best, mut too_far, mut curved_zone): (Option<([f64; 6], f64, Move)>, bool, bool) =
                (None, false, false);
            for candidate in candidates.into_iter().flatten() {
                let candidate: Move = candidate.into_iter().filter(|&(v, p)| p != cage.positions[v as usize]).collect();
                if candidate.is_empty() || candidate.iter().any(|&(v, _)| !movable[v as usize]) {
                    continue;
                }
                let displacement = |&(v, p): &(u32, Vec3)| (p - mesh.positions[v as usize]).length();
                if candidate.iter().any(|m| displacement(m) > reach[m.0 as usize] * (1.0 + 1e-9)) {
                    too_far = true;
                    continue;
                }
                let moved: Vec<u32> = candidate.iter().map(|&(v, _)| v).collect();
                let (faces, corners) = cage.region(&moved);
                let was = cage.badness(&faces, &corners, checking, tolerance);
                let undo = cage.move_to(&candidate);
                let now = cage.badness(&faces, &corners, checking, tolerance);
                // Flat, and still pinching: the face is on a curved zone,
                // and what took it off its plane is the shape.
                let pinches = kind == Kind::NotFlat && {
                    let mut own = Vec::new();
                    cage.check_face(f as usize, checking, tolerance, &mut own);
                    own.iter().any(|fault| fault.kind == Kind::Pinch)
                };
                cage.move_to(&undo);
                if pinches {
                    curved_zone = true;
                    continue;
                }
                if !now.is_better_than(&was) {
                    continue;
                }
                // Candidates touch different faces: they are compared by
                // what they change, then by the furthest they move a vertex.
                let gain = change(&now, &was);
                let largest = candidate.iter().map(displacement).fold(0.0, f64::max);
                let better = best.as_ref().is_none_or(|(other, other_largest, _)| {
                    compare_terms(&gain, other).then(largest.total_cmp(other_largest)) == Ordering::Less
                });
                if better {
                    best = Some((gain, largest, candidate));
                }
            }
            match best {
                Some((_, _, chosen_move)) => {
                    cage.move_to(&chosen_move);
                    outcome.remove(&(f, kind));
                    kept += 1;
                }
                None => {
                    // A move beyond the bound might have mended it: that
                    // before the curved zone the others found.
                    let reason = if too_far {
                        Reason::TooFar
                    } else if curved_zone {
                        Reason::ChangeTopology
                    } else {
                        Reason::WouldWorsen
                    };
                    outcome.insert((f, kind), reason);
                }
            }
        }
        repairs += kept;
        if kept == 0 {
            break;
        }
    }

    let positions = cage.positions.to_vec();
    let repaired = PolyMesh { positions, faces: mesh.faces.clone() };
    let after = check(&repaired, creases, checking);

    // Creases, on the cage as the moves left it, fault by fault: the sharp
    // edges of a pinched face or round a pole on a curved zone, kept for
    // that fault if they make the score better.
    let crease_fold = settings.crease_fold.to_radians();
    let mut proposed: Vec<ProposedCrease> = Vec::new();
    for fault in &after.faults {
        let edges: Vec<(u32, u32)> = match fault.at {
            Element::Face(f) if fault.kind == Kind::Pinch && chosen[f as usize] => {
                let face = &mesh.faces[f as usize];
                (0..face.len()).map(|i| key(face[i], face[(i + 1) % face.len()])).collect()
            }
            Element::Vertex(v) if fault.kind == Kind::CurvedPole && movable[v as usize] => cage
                .neighbours(v)
                .iter()
                .map(|&u| key(v, u))
                .filter(|&(a, b)| cage.chosen_edge(a, b, &chosen))
                .collect(),
            _ => continue,
        };
        let mut group: Vec<(usize, f64)> = Vec::new();
        for (a, b) in edges {
            let Some(e) = cage.edge(a, b) else { continue };
            if cage.sharp[e] || group.iter().any(|&(other, _)| other == e) {
                continue;
            }
            if let Some(fold) = cage.edge_fold(e)
                && fold >= crease_fold
            {
                group.push((e, fold));
            }
        }
        if group.is_empty() {
            continue;
        }
        let ends: Vec<u32> = group.iter().flat_map(|&(e, _)| [cage.edges[e].0, cage.edges[e].1]).collect();
        let (faces, corners) = cage.crease_region(&ends);
        let was = cage.badness(&faces, &corners, checking, tolerance);
        for &(e, _) in &group {
            cage.set_sharp(e, true);
        }
        let now = cage.badness(&faces, &corners, checking, tolerance);
        if now.is_better_than(&was) {
            for &(e, fold) in &group {
                let (a, b) = cage.edges[e];
                proposed.push(ProposedCrease {
                    edge: [a, b],
                    fold: fold.to_degrees(),
                    kind: fault.kind,
                    at: fault.at,
                    value: fault.value,
                    cured: false,
                });
            }
        } else {
            for &(e, _) in &group {
                cage.set_sharp(e, false);
            }
        }
    }
    let all_creases: Vec<Crease> = creases.iter().copied().chain(proposed.iter().map(|c| (c.edge, 1.0))).collect();
    let with_creases = check(&repaired, &all_creases, checking);
    let gone = |kind: Kind, at: Element| !with_creases.faults.iter().any(|f| f.kind == kind && f.at == at);
    for crease in &mut proposed {
        crease.cured = gone(crease.kind, crease.at);
    }

    // What is left, and why.
    let left = after
        .faults
        .iter()
        .map(|fault| {
            let inside = match fault.at {
                Element::Face(f) => chosen[f as usize],
                Element::Vertex(v) => movable[v as usize],
            };
            let reason = if !proposed.is_empty() && gone(fault.kind, fault.at) {
                Reason::ProposedCrease
            } else if !inside {
                Reason::OutsideSelection
            } else if fault.kind.repair() == Repair::ChangeTopology {
                Reason::ChangeTopology
            } else {
                let Element::Face(f) = fault.at else { unreachable!("a vertex fault is mended by the topology") };
                outcome.get(&(f, fault.kind)).copied().unwrap_or(Reason::OutOfPasses)
            };
            Left { fault: fault.clone(), reason }
        })
        .collect();

    let (mut moved, mut largest_displacement, mut largest_share) = (0, 0.0f64, 0.0f64);
    for (v, (&now, &was)) in repaired.positions.iter().zip(&mesh.positions).enumerate() {
        if now != was {
            moved += 1;
            let distance = (now - was).length();
            largest_displacement = largest_displacement.max(distance);
            largest_share = largest_share.max(distance / shortest[v]);
        }
    }
    let given = Some(creases.len());
    Repaired {
        moved,
        largest_displacement,
        largest_share,
        badness_before: Badness::of(&before.faults, thin),
        badness_after: Badness::of(&after.faults, thin),
        badness_with_creases: Badness::of(&with_creases.faults, thin),
        before: before.score(given),
        after: after.score(given),
        with_creases: with_creases.score(Some(all_creases.len())),
        creases: proposed,
        left,
        passes,
        repairs,
        positions: repaired.positions,
    }
}

/// The order of [`Badness::compare`] on its terms, or on changes of them:
/// the first term that differs by more than rounding decides.
fn compare_terms(a: &[f64; 6], b: &[f64; 6]) -> Ordering {
    const EPSILONS: [f64; 6] = [0.5, 0.5, ANGLE_EPSILON, ANGLE_EPSILON, 0.5, SHARE_EPSILON];
    for i in 0..6 {
        if (a[i] - b[i]).abs() > EPSILONS[i] {
            return a[i].total_cmp(&b[i]);
        }
    }
    Ordering::Equal
}

/// What a change does to the badness, term by term: negative where it
/// mends.
fn change(now: &Badness, was: &Badness) -> [f64; 6] {
    let (now, was) = (now.terms(), was.terms());
    std::array::from_fn(|i| now[i] - was[i])
}

/// An orthonormal basis of the directions square to every one of `normals`
/// (unit vectors): three, two, one or none.
fn allowed(normals: &[Vec3]) -> Vec<Vec3> {
    let mut span: Vec<Vec3> = Vec::new();
    for &n in normals {
        let mut u = n;
        for &s in &span {
            u -= u.dot(s) * s;
        }
        if u.length() > SAME_PLANE {
            span.push(u.normalize());
        }
        if span.len() == 3 {
            break;
        }
    }
    match span.len() {
        0 => vec![Vec3::X, Vec3::Y, Vec3::Z],
        1 => {
            let (s, t) = tangent_frame(span[0]);
            vec![s, t]
        }
        2 => vec![span[0].cross(span[1]).normalize()],
        _ => Vec::new(),
    }
}

/// `d` projected onto the span of `basis`.
fn project(basis: &[Vec3], d: Vec3) -> Vec3 {
    basis.iter().map(|&b| d.dot(b) * b).sum()
}

impl Cage<'_> {
    /// The normal of face `g` if it is an n-gon of 5 sides or more that is
    /// flat by the check: one a move must keep flat.
    fn flat_normal(&self, g: usize, settings: &Settings) -> Option<Vec3> {
        let face = &self.faces[g];
        if face.len() < 5 {
            return None;
        }
        let n = self.normals[g]?;
        let corners: Vec<Vec3> = face.iter().map(|&v| self.positions[v as usize]).collect();
        let centre = corners.iter().sum::<Vec3>() / corners.len() as f64;
        let radius = corners.iter().map(|&c| (c - centre).length()).fold(0.0, f64::max);
        let deviation = corners.iter().map(|&c| (c - centre).dot(n).abs()).fold(0.0, f64::max);
        (radius > 1e-9 && deviation <= settings.flat_tolerance * radius).then_some(n)
    }

    /// The directions vertex `v` may move in, square to `own` (if any) and
    /// to the flat n-gons round it other than `face`.
    fn freedom(&self, v: u32, face: usize, own: Option<Vec3>, settings: &Settings) -> Vec<Vec3> {
        let normals: Vec<Vec3> = own
            .into_iter()
            .chain(
                self.faces_round(v)
                    .iter()
                    .filter(|&&g| g as usize != face)
                    .filter_map(|&g| self.flat_normal(g as usize, settings)),
            )
            .collect();
        allowed(&normals)
    }

    /// Vertex `v` brought into the plane through `centre` square to `n`,
    /// within the directions it may move in; None if it cannot be.
    fn onto_plane(&self, v: u32, face: usize, centre: Vec3, n: Vec3, settings: &Settings) -> Option<Vec3> {
        let p = self.positions[v as usize];
        let off = (p - centre).dot(n);
        if off == 0.0 {
            return Some(p);
        }
        let d = project(&self.freedom(v, face, None, settings), n);
        // A move along d changes the distance to the plane by d·n = |d|².
        if d.length_squared() < 0.01 {
            return None;
        }
        Some(p - off / d.length_squared() * d)
    }

    /// The ways of flattening face `f`: all its vertices onto its plane, and
    /// for each vertex out of the plane of the others, that vertex alone
    /// into it.
    fn flattenings(&self, f: usize, settings: &Settings) -> Vec<Option<Move>> {
        let face = &self.faces[f];
        let k = face.len();
        let corners: Vec<Vec3> = face.iter().map(|&v| self.positions[v as usize]).collect();
        let mut out = Vec::new();
        let raw = newell(&corners);
        if raw.length() <= 1e-12 {
            return out;
        }
        let (n, centre) = (raw.normalize(), corners.iter().sum::<Vec3>() / k as f64);
        out.push(face.iter().map(|&v| self.onto_plane(v, f, centre, n, settings).map(|p| (v, p))).collect());
        for i in 0..k {
            let others: Vec<Vec3> = (0..k).filter(|&j| j != i).map(|j| corners[j]).collect();
            let raw = newell(&others);
            if raw.length() <= 1e-12 {
                continue;
            }
            let (n, centre) = (raw.normalize(), others.iter().sum::<Vec3>() / (k - 1) as f64);
            let radius = others.iter().map(|&c| (c - centre).length()).fold(0.0, f64::max);
            let deviation = others.iter().map(|&c| (c - centre).dot(n).abs()).fold(0.0, f64::max);
            if deviation <= 0.5 * settings.flat_tolerance * radius {
                out.push(self.onto_plane(face[i], f, centre, n, settings).map(|p| vec![(face[i], p)]));
            }
        }
        out
    }

    /// The smallest move of the vertices of face `f`, in its plane and
    /// keeping the flat n-gons round them flat, that brings every triangle
    /// (centre, edge) to at least `share` of the mean one; None if the
    /// solve does not get there.
    fn straightened(&self, f: usize, share: f64, settings: &Settings) -> Option<Move> {
        let face = &self.faces[f];
        let k = face.len();
        let n = self.normals[f]?;
        // The projection onto the directions each vertex may move in.
        let freedoms: Vec<Vec<Vec3>> = face.iter().map(|&v| self.freedom(v, f, Some(n), settings)).collect();
        let mut p: Vec<Vec3> = face.iter().map(|&v| self.positions[v as usize]).collect();
        // Aim a little above the share, so as not to end on its edge.
        let aim = 1.05 * share;
        for _ in 0..STEPS {
            let centre = p.iter().sum::<Vec3>() / k as f64;
            let wedges: Vec<f64> =
                (0..k).map(|i| 0.5 * (p[i] - centre).cross(p[(i + 1) % k] - centre).dot(n)).collect();
            let area: f64 = wedges.iter().sum();
            if area <= 0.0 {
                return None;
            }
            let mean = area / k as f64;
            let short: Vec<usize> = (0..k).filter(|&i| wedges[i] < share * mean).collect();
            if short.is_empty() {
                return Some(face.iter().copied().zip(p).collect());
            }
            // The gradient of the area, by vertex: the sum of those of the
            // wedges, where the moves of the centre cancel out.
            let area_gradient: Vec<Vec3> =
                (0..k).map(|j| 0.5 * (p[(j + 1) % k] - p[(j + k - 1) % k]).cross(n)).collect();
            // The gradient of wedge i less `aim` times the mean, by vertex.
            let gradients: Vec<Vec<Vec3>> = short
                .iter()
                .map(|&i| {
                    let (a, b) = (p[i] - centre, p[(i + 1) % k] - centre);
                    let edge = 0.5 * (p[(i + 1) % k] - p[i]).cross(n) / k as f64;
                    (0..k)
                        .map(|j| {
                            let mut g = -edge - aim / k as f64 * area_gradient[j];
                            if j == i {
                                g += 0.5 * b.cross(n);
                            }
                            if j == (i + 1) % k {
                                g += 0.5 * n.cross(a);
                            }
                            project(&freedoms[j], g)
                        })
                        .collect()
                })
                .collect();
            let m = short.len();
            let mut normal = vec![0.0; m * m];
            for r in 0..m {
                for c in 0..m {
                    normal[r * m + c] = (0..k).map(|j| gradients[r][j].dot(gradients[c][j])).sum();
                }
            }
            let trace: f64 = (0..m).map(|r| normal[r * m + r]).sum();
            if trace <= 0.0 {
                return None;
            }
            for r in 0..m {
                normal[r * m + r] += 1e-12 * trace / m as f64;
            }
            let rhs: Vec<f64> = short.iter().map(|&i| aim * mean - wedges[i]).collect();
            let y = solve_dense(m, &normal, &rhs).ok()?;
            for (j, point) in p.iter_mut().enumerate() {
                *point += (0..m).map(|r| y[r] * gradients[r][j]).sum::<Vec3>();
            }
        }
        None
    }

    /// Sets the positions of `moves`, and the normals of the faces round
    /// them; returns the move back.
    fn move_to(&mut self, moves: &[(u32, Vec3)]) -> Move {
        let positions = self.positions.to_mut();
        let undo = moves.iter().map(|&(v, p)| (v, std::mem::replace(&mut positions[v as usize], p))).collect();
        let mut faces: Vec<u32> = moves.iter().flat_map(|&(v, _)| self.faces_round(v).iter().copied()).collect();
        faces.sort_unstable();
        faces.dedup();
        for f in faces {
            self.normals[f as usize] = self.normal_of(f as usize);
        }
        undo
    }

    /// What a move of the vertices `moved` can change the check of: the
    /// faces sharing a vertex with a face round one of them (whose pinch
    /// reads the normals of those), and the vertices of the faces round
    /// them (whose pole reads those normals).
    fn region(&self, moved: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut round: Vec<u32> = moved.iter().flat_map(|&v| self.faces_round(v).iter().copied()).collect();
        round.sort_unstable();
        round.dedup();
        let mut corners: Vec<u32> = round.iter().flat_map(|&f| self.faces[f as usize].iter().copied()).collect();
        corners.sort_unstable();
        corners.dedup();
        let mut faces: Vec<u32> = corners.iter().flat_map(|&v| self.faces_round(v).iter().copied()).collect();
        faces.sort_unstable();
        faces.dedup();
        (faces, corners)
    }

    /// What a crease on edges ending at `ends` can change the check of: the
    /// faces round those vertices, and the vertices themselves.
    fn crease_region(&self, ends: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut corners = ends.to_vec();
        corners.sort_unstable();
        corners.dedup();
        let mut faces: Vec<u32> = corners.iter().flat_map(|&v| self.faces_round(v).iter().copied()).collect();
        faces.sort_unstable();
        faces.dedup();
        (faces, corners)
    }

    /// The badness of the faults of `faces` and `vertices`.
    fn badness(&self, faces: &[u32], vertices: &[u32], settings: &Settings, tolerance: f64) -> Badness {
        let mut faults = Vec::new();
        for &f in faces {
            self.check_face(f as usize, settings, tolerance, &mut faults);
        }
        let mut looked = 0;
        faults.extend(vertices.iter().filter_map(|&v| self.check_vertex(v as usize, tolerance, &mut looked)));
        Badness::of(&faults, settings.thin_ratio)
    }

    /// The fold, in radians, between the two faces of edge `e`, if it has
    /// two with a normal.
    fn edge_fold(&self, e: usize) -> Option<f64> {
        let faces = &self.edge_faces[self.edge_offsets[e] as usize..self.edge_offsets[e + 1] as usize];
        let [a, b] = faces else { return None };
        Some(self.fold(self.normals[*a as usize]?, self.normals[*b as usize]?))
    }

    /// Whether the edge from `a` to `b` is on one of the faces `chosen`.
    fn chosen_edge(&self, a: u32, b: u32, chosen: &[bool]) -> bool {
        self.faces_on(a, b).iter().any(|&f| chosen[f as usize])
    }

    /// Makes edge `e` sharp by a crease, or takes the crease back.
    fn set_sharp(&mut self, e: usize, sharp: bool) {
        if self.sharp[e] == sharp {
            return;
        }
        self.sharp[e] = sharp;
        let (a, b) = self.edges[e];
        for v in [a, b] {
            if sharp {
                self.creased[v as usize] += 1;
            } else {
                self.creased[v as usize] -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflection::tests::{cut_cube, dart, grid_with_octagon};

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn kinds(left: &[Left]) -> Vec<(Kind, Reason)> {
        left.iter().map(|l| (l.fault.kind, l.reason)).collect()
    }

    /// Runs the repair on what it gave back: nothing more is done.
    fn again(mesh: &PolyMesh, creases: &[Crease], settings: &RepairSettings, once: &Repaired) {
        let second = PolyMesh { positions: once.positions.clone(), faces: mesh.faces.clone() };
        let twice = repair(&second, creases, None, settings);
        assert_eq!(twice.positions, once.positions);
        assert_eq!((twice.moved, twice.repairs), (0, 0));
        assert_eq!(twice.creases, once.creases);
        assert_eq!(kinds(&twice.left), kinds(&once.left));
    }

    #[test]
    fn the_order_puts_folds_before_any_angle_and_the_topology_before_flatness() {
        let none = Badness::default();
        // One folded face is worse than any sum of angles.
        let folded = Badness { folded: 1, ..none };
        let bent = Badness { curved: 1000.0, not_flat: 1000.0, thin: 50, ..none };
        assert!(bent.is_better_than(&folded) && !folded.is_better_than(&bent));
        // A degree more of pinch is not paid for by ten less off the plane.
        let flattened = Badness { curved: 11.0, not_flat: 0.0, ..none };
        let as_was = Badness { curved: 10.0, not_flat: 10.0, ..none };
        assert!(as_was.is_better_than(&flattened));
        // Thin faces last: none of them is worth a hundredth of a degree.
        let thin = Badness { thin: 3, thinness: 0.4, ..none };
        assert!(thin.is_better_than(&Badness { not_flat: 0.01, ..none }));
        // Rounding is no change, either way.
        let rounded = Badness { curved: 10.0 + 1e-9, not_flat: 10.0 - 1e-9, ..none };
        assert_eq!(rounded.compare(&as_was), Ordering::Equal);
        assert!(!rounded.is_better_than(&as_was) && !as_was.is_better_than(&rounded));
        // A badness is a sum over faults.
        let fault = |kind, value| Fault {
            kind,
            at: Element::Face(0),
            sides: 5,
            count: 1,
            value,
            parts: Vec::new(),
            frame: Default::default(),
        };
        let faults = [
            fault(Kind::Pinch, 12.0),
            fault(Kind::CurvedPole, 3.0),
            fault(Kind::NotFlat, 2.0),
            fault(Kind::Thin, 0.05),
        ];
        let badness = Badness::of(&faults, 0.15);
        assert_eq!((badness.curved, badness.not_flat, badness.thin), (15.0, 2.0, 1));
        assert!(close(badness.thinness, 0.1));
    }

    /// The flat grid of 5 by 5 vertices with its octagon, and one more
    /// vertex on the edge from (1, 0) to (1, 1), raised by `h`: the two
    /// quads either side are pentagons, flat but for that vertex. Returns
    /// the mesh and the vertex.
    fn raised_middle(h: f64) -> (PolyMesh, u32) {
        let (mut mesh, _) = grid_with_octagon(|i, j| Vec3::new(i as f64, j as f64, 0.0));
        let (a, b) = (1u32, 6u32);
        let middle = mesh.positions.len() as u32;
        mesh.positions.push(Vec3::new(1.0, 0.5, h));
        for face in &mut mesh.faces {
            for k in 0..face.len() {
                let (u, w) = (face[k], face[(k + 1) % face.len()]);
                if (u, w) == (a, b) || (u, w) == (b, a) {
                    face.insert(k + 1, middle);
                    break;
                }
            }
        }
        (mesh, middle)
    }

    #[test]
    fn a_vertex_raised_out_of_a_flat_surrounding_is_brought_back() {
        // Its two pentagons are off their plane, and pinch. All their
        // vertices but the raised one lie on z = 0: it alone is brought
        // into that plane, straight down by h, and the grid is flat again.
        // Its edges are √(1/4 + h²) long: h is 0.080 of them, within the
        // bound of 0.1.
        let h: f64 = 0.04;
        let (mesh, middle) = raised_middle(h);
        let settings = RepairSettings::default();
        let out = repair(&mesh, &[], None, &settings);
        assert_eq!(out.before.of(Kind::NotFlat).count, 2);
        assert!(out.before.of(Kind::Pinch).count > 0);
        assert_eq!(out.moved, 1);
        assert_eq!(out.positions[middle as usize], Vec3::new(1.0, 0.5, 0.0));
        assert!(close(out.largest_displacement, h));
        assert!(close(out.largest_share, h / (0.25 + h * h).sqrt()));
        assert_eq!(&out.positions[..middle as usize], &mesh.positions[..middle as usize]);
        // Tilt 0, nothing left.
        assert!(out.after.kinds.iter().all(|k| k.count == 0), "{:?}", out.after);
        assert_eq!(out.badness_after, Badness::default());
        assert!(out.left.is_empty() && out.creases.is_empty());
        again(&mesh, &[], &settings, &out);

        // Asked for the quads round them only, the pentagons are left.
        let quads: Vec<u32> = (0..mesh.faces.len() as u32).filter(|&f| mesh.faces[f as usize].len() == 4).collect();
        let outside = repair(&mesh, &[], Some(&quads), &settings);
        assert_eq!(outside.positions, mesh.positions);
        assert!(outside.left.iter().all(|l| l.reason == Reason::OutsideSelection), "{:?}", kinds(&outside.left));
    }

    #[test]
    fn a_repair_that_needs_more_than_the_bound_is_refused() {
        // With a bound of 0.04 of the shortest edge. The raised vertex alone
        // needs h / √(1/4 + h²) = 0.080. All five vertices onto the plane of
        // the pentagon: from the pentagon (0, 0, 0), (1, 0, 0), (1, 1/2, h),
        // (1, 1, 0), (0, 1, 0), its Newell normal is along (-h, 0, 2), its
        // centre at (0.6, 0.5, h / 5); the raised vertex is (0.4, 0, 4 h / 5)
        // from it, 1.2 h / √(h² + 4) off the plane, 0.048 of its shortest
        // edge. Both too far.
        let h: f64 = 0.04;
        assert!(1.2 * h / (h * h + 4.0).sqrt() / (0.25 + h * h).sqrt() > 0.04);
        let (mesh, _) = raised_middle(h);
        let settings = RepairSettings { bound: 0.04, ..Default::default() };
        let out = repair(&mesh, &[], None, &settings);
        assert_eq!(out.positions, mesh.positions);
        assert_eq!((out.moved, out.largest_displacement), (0, 0.0));
        assert_eq!(out.after, out.before);
        let not_flat: Vec<Reason> =
            out.left.iter().filter(|l| l.fault.kind == Kind::NotFlat).map(|l| l.reason).collect();
        assert_eq!(not_flat, [Reason::TooFar, Reason::TooFar]);
        again(&mesh, &[], &settings, &out);

        // With 0.06, the plane of the pentagon is within the bound, but
        // tilted by atan(h / 2), more than a degree, against the flat grid:
        // flat, and pinching. The vertex alone is still too far. Left as too
        // far, not as wanting a change of topology: a larger bound mends it.
        assert!((h / 2.0).atan().to_degrees() > 1.0);
        let settings = RepairSettings { bound: 0.06, ..Default::default() };
        let out = repair(&mesh, &[], None, &settings);
        assert_eq!(out.positions, mesh.positions);
        let not_flat: Vec<Reason> =
            out.left.iter().filter(|l| l.fault.kind == Kind::NotFlat).map(|l| l.reason).collect();
        assert_eq!(not_flat, [Reason::TooFar, Reason::TooFar]);
    }

    #[test]
    fn a_folded_dart_is_made_star_shaped_in_its_plane() {
        // The dart of the check, notch at (0, 1): the centre (0, 1/2) is on
        // the wrong side of the edges from the lower tips to the notch, by
        // 1 / (2 √2). A move of the vertices by d at most moves the centre
        // and those edges by d at most: some vertex must move by 1 / (4 √2)
        // or more. Its shortest edges are √8 long.
        let mesh = dart(1.0);
        let star_shaped = |p: &[Vec3], bound: f64, out: &Repaired| {
            // In its plane, and mirror symmetric as it was.
            assert!(p.iter().all(|q| q.z == 0.0));
            assert!(p[0].x.abs() < 1e-12 && p[2].x.abs() < 1e-12);
            assert!((p[1].x + p[3].x).abs() < 1e-12 && close(p[1].y, p[3].y));
            // Star-shaped round the mean of its vertices.
            let centre = p.iter().sum::<Vec3>() / 4.0;
            for i in 0..4 {
                assert!((p[i] - centre).cross(p[(i + 1) % 4] - centre).z > 0.0, "{p:?}");
            }
            assert!(out.largest_displacement >= 1.0 / (4.0 * 2f64.sqrt()));
            for (now, was) in p.iter().zip(&mesh.positions) {
                assert!((*now - *was).length() <= bound * 8f64.sqrt() * (1.0 + 1e-9));
            }
        };
        // With a bound of 0.3, neither folded nor thin.
        let settings = RepairSettings { bound: 0.3, ..Default::default() };
        let out = repair(&mesh, &[], None, &settings);
        assert_eq!(out.before.of(Kind::Folded).count, 1);
        assert!(out.after.kinds.iter().all(|k| k.count == 0), "{:?}", out.after);
        star_shaped(&out.positions, 0.3, &out);
        again(&mesh, &[], &settings, &out);

        // With 0.2, the fold is mended and the face left thin: making it
        // wider too would go beyond the bound. A second run starts from
        // there with a bound of its own, and goes further: the bound is on
        // the cage each run is given.
        let settings = RepairSettings { bound: 0.2, ..Default::default() };
        let out = repair(&mesh, &[], None, &settings);
        assert_eq!(out.after.of(Kind::Folded).count, 0);
        assert_eq!(kinds(&out.left), [(Kind::Thin, Reason::TooFar)]);
        star_shaped(&out.positions, 0.2, &out);
        let second = PolyMesh { positions: out.positions.clone(), faces: mesh.faces.clone() };
        assert!(repair(&second, &[], None, &settings).moved > 0);

        // With the default bound, 0.1, the solve finds no move within it:
        // left as it is.
        let out = repair(&mesh, &[], None, &RepairSettings::default());
        assert_eq!(out.positions, mesh.positions);
        assert_eq!(kinds(&out.left), [(Kind::Folded, Reason::TooFar)]);
    }

    #[test]
    fn an_octagon_on_a_cylinder_is_not_flattened_and_wants_a_change_of_topology() {
        // The octagon of the check on a cylinder: off its plane by the turn
        // of the cylinder, and pinched. Flattened, it would still turn by
        // about 3/2 α to the quads either side: it is on a curved zone.
        let (alpha, r, h) = (10f64.to_radians(), 1.0, 0.2);
        let (mesh, octagon) = grid_with_octagon(|i, j| {
            let angle = i as f64 * alpha;
            Vec3::new(r * angle.sin(), j as f64 * h, r * angle.cos())
        });
        let settings = RepairSettings::default();
        let out = repair(&mesh, &[], None, &settings);
        assert_eq!(out.positions, mesh.positions);
        assert_eq!(out.after, out.before);
        let left = kinds(&out.left);
        assert!(left.contains(&(Kind::NotFlat, Reason::ChangeTopology)), "{left:?}");
        assert!(left.contains(&(Kind::Pinch, Reason::ChangeTopology)), "{left:?}");
        assert!(left.iter().all(|&(_, reason)| reason == Reason::ChangeTopology), "{left:?}");
        // The flattening, forced: flat, and still pinching.
        let cage = Cage::build(&mesh, &[], 0.999);
        let flattened = cage.flattenings(octagon, &settings.check)[0].clone().expect("every vertex is free");
        let mut forced = mesh.clone();
        for (v, p) in flattened {
            forced.positions[v as usize] = p;
        }
        let found = check(&forced, &[], &settings.check);
        let own: Vec<Kind> =
            found.faults.iter().filter(|f| f.at == Element::Face(octagon as u32)).map(|f| f.kind).collect();
        assert_eq!(own, [Kind::Pinch]);
        again(&mesh, &[], &settings, &out);
    }

    #[test]
    fn a_triangle_on_a_cube_corner_gets_creases_on_its_border() {
        // The cube with a corner cut off by a triangle. The edges of the
        // cube creased, but for the triangle's three: the triangle pinches,
        // and so do the pentagons across its edges. Each of its edges is a
        // fold of acos(1 / √3) between the triangle and a pentagon, beyond
        // 30 degrees: proposed, and once set, the cage is creased all round
        // and nothing is left.
        let (mesh, triangle, _) = cut_cube(0.5);
        let rim = [(7, 8), (8, 9), (7, 9)];
        let given: Vec<Crease> =
            mesh.edge_uses().keys().filter(|edge| !rim.contains(edge)).map(|&(a, b)| ([a, b], 1.0)).collect();
        assert_eq!(given.len(), 12);
        let settings = RepairSettings::default();
        let out = repair(&mesh, &given, None, &settings);
        assert_eq!(out.positions, mesh.positions);
        let mut edges: Vec<[u32; 2]> = out.creases.iter().map(|c| c.edge).collect();
        edges.sort();
        assert_eq!(edges, [[7, 8], [7, 9], [8, 9]]);
        let corner = (1.0 / 3f64.sqrt()).acos().to_degrees();
        for crease in &out.creases {
            assert!(close(crease.fold, corner), "{crease:?}");
            assert_eq!(crease.kind, Kind::Pinch);
            // Each is proposed for the pinch of the pentagon it borders,
            // found first, the worst: at the corners of the triangle the
            // pentagon meets the next one, on no sharp edge between them,
            // at a right angle.
            assert!(matches!(crease.at, Element::Face(3..=5)), "{crease:?}");
            assert!(crease.cured && close(crease.value, 90.0), "{crease:?}");
        }
        let pinch = out.left.iter().find(|l| l.fault.at == Element::Face(triangle as u32)).expect("it pinches");
        assert_eq!((pinch.fault.kind, pinch.reason), (Kind::Pinch, Reason::ProposedCrease));
        assert!(out.with_creases.kinds.iter().all(|k| k.count == 0), "{:?}", out.with_creases);
        assert_eq!((out.with_creases.poles, out.with_creases.creases), (0, Some(15)));
        assert_eq!(out.badness_with_creases, Badness::default());
        // Proposed, not set: the score of the cage as it is does not move.
        assert_eq!(out.after, out.before);
        again(&mesh, &given, &settings, &out);

        // With no crease given, every edge is a fold of 90 degrees or of
        // acos(1 / √3): all fifteen are proposed, for the pinches and for
        // the poles at the corners of the cube, and with them nothing is
        // left either.
        let bare = repair(&mesh, &[], None, &settings);
        assert_eq!(bare.creases.len(), 15);
        assert!(bare.creases.iter().all(|c| close(c.fold, 90.0) || close(c.fold, corner)));
        assert!(bare.with_creases.kinds.iter().all(|k| k.count == 0), "{:?}", bare.with_creases);
        assert!(bare.left.iter().all(|l| l.reason == Reason::ProposedCrease), "{:?}", kinds(&bare.left));
        // A stricter fold leaves the triangle's edges out.
        let strict = repair(&mesh, &given, None, &RepairSettings { crease_fold: 60.0, ..Default::default() });
        assert!(strict.creases.is_empty());
    }

    #[test]
    fn a_cage_with_no_fault_comes_back_as_it_was() {
        let (mesh, _) = grid_with_octagon(|i, j| Vec3::new(0.1 * i as f64, 0.3 * j as f64, 0.0));
        let out = repair(&mesh, &[], None, &RepairSettings::default());
        let bits = |p: &Vec3| p.to_array().map(f64::to_bits);
        assert!(out.positions.iter().zip(&mesh.positions).all(|(a, b)| bits(a) == bits(b)));
        assert_eq!((out.moved, out.repairs, out.passes), (0, 0, 1));
        assert!(out.left.is_empty() && out.creases.is_empty());
        assert_eq!(out.before, out.after);
    }
}
