//! Pairs of poles that one separatrix joins (DT-115).
//!
//! In a seamless map a separatrix that leaves a pole often runs on to
//! another pole and misses it by a hair. Raced as a motorcycle, it then
//! goes on past that pole and leaves a sliver of a patch along the line
//! between them; on a cube, one per edge.
//!
//! Here each separatrix is walked alone, with no race. Two walks that run
//! along the same line in opposite directions, from two poles, are the one
//! line between those poles seen from both ends. If they are close enough,
//! the pair gives a linear condition that puts both poles on that line: the
//! map is solved again with it, and the line is then laid as one arc from
//! pole to pole before any motorcycle sets off.

use super::walk::{Axis, Exit, Map, axis_vector};
use crate::parametrization::Condition;

/// How far apart two walks may be, across, for their poles to be put on one
/// line, in quad edges: under half a quad edge, the strip between them is
/// what the layout counts as thin, and what quantization would close.
const GAP: f64 = 0.5;
/// The same as a share of the distance between the poles: a snap may shear
/// the map by about 6 degrees at most.
const SLOPE: f64 = 0.1;
/// Poles closer than this along the line, in quad edges, are not paired.
const SHORTEST: f64 = 1.0;
/// How far a separatrix is walked, in times the side of a square with the
/// area of the map.
const REACH: f64 = 1.0;
/// Two walks this close are on one line already: the tracing tolerance of
/// the race, well above what the conditions are met to.
const JOINED: f64 = 1e-6;

/// What a pair must meet: the largest gap across, absolute and as a share
/// of the distance between the poles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    pub gap: f64,
    pub slope: f64,
}

impl Tolerance {
    /// The pairs that are worth a snap.
    pub const FULL: Tolerance = Tolerance { gap: GAP, slope: SLOPE };
    /// The pairs already on one line, for the race.
    pub const JOINED: Tolerance = Tolerance { gap: JOINED, slope: 1.0 };

    fn admits(&self, gap: f64, length: f64) -> bool {
        gap <= self.gap && gap <= self.slope * length && length >= SHORTEST
    }
}

/// Two separatrices along one line, from either end.
#[derive(Clone, Debug)]
pub struct Pair {
    /// Each separatrix: the vertex it leaves and the rank of its direction
    /// among the departures there.
    pub ends: [(u32, usize); 2],
    /// Distance between the two ends along the line, and the gap across, in
    /// quad edges.
    pub length: f64,
    pub gap: f64,
    /// That the two ends are on one grid line.
    pub condition: Condition,
    /// The triangles the line between the two ends crosses.
    pub faces: Vec<u32>,
}

/// A stretch of a walk within one triangle.
struct Piece {
    walk: u32,
    /// Its rank along the walk.
    index: u32,
    start: [f64; 2],
    axis: Axis,
    length: f64,
    /// Length of the walk where it starts.
    time: f64,
}

/// A change of chart along a walk: before the piece of rank `at`, from the
/// corner `from` to the corner `to` of one vertex, turning by `turn` quarter
/// turns clockwise, as the map does across a seam.
struct Jump {
    at: u32,
    from: u32,
    to: u32,
    turn: u8,
}

struct Walk {
    vertex: u32,
    rank: usize,
    /// The corner of the vertex in the face the walk leaves through.
    corner: u32,
    jumps: Vec<Jump>,
    /// The triangle of each of its pieces.
    faces: Vec<u32>,
}

/// A point of a line as a linear expression in the coordinates of the map:
/// its `u` and its `v`, each as terms (corner, coordinate, coefficient).
type Anchor = [Vec<(u32, u8, f64)>; 2];

/// The pairs of separatrices of `sources` that `tolerance` admits, shortest
/// first. `wedge` names the unknown of each corner, to tell a seam from an
/// ordinary edge round a vertex.
pub(super) fn find(
    map: &Map,
    wedge: &[u32],
    sources: &[u32],
    is_singular: &[bool],
    is_wall: &[bool],
    tolerance: Tolerance,
) -> Vec<Pair> {
    let faces = map.mesh.triangles.len();
    let side = (0..faces).map(|f| map.area(f).abs()).sum::<f64>().sqrt();
    let mut on_wall = vec![false; map.mesh.positions.len()];
    for h in (0..is_wall.len()).filter(|&h| is_wall[h]) {
        on_wall[map.mesh.triangles[h / 3][h % 3] as usize] = true;
        on_wall[map.mesh.triangles[h / 3][(h % 3 + 1) % 3] as usize] = true;
    }

    let mut pieces: Vec<Vec<Piece>> = (0..faces).map(|_| Vec::new()).collect();
    let mut walks: Vec<Walk> = Vec::new();
    for &vertex in sources {
        for (rank, (face, corner, axis)) in map.departures(vertex).into_iter().enumerate() {
            let id = walks.len() as u32;
            let mut walk =
                Walk { vertex, rank, corner: (3 * face + corner) as u32, jumps: Vec::new(), faces: Vec::new() };
            walk_one(
                map,
                wedge,
                is_singular,
                is_wall,
                &on_wall,
                REACH * side,
                (face, corner, axis),
                id,
                &mut walk,
                &mut pieces,
            );
            walks.push(walk);
        }
    }

    // For each walk, the first walk met running the other way along the
    // same line: the distance between their ends, the gap, that walk, and
    // the two pieces where they met.
    let mut first: Vec<Option<(f64, f64, u32, (usize, usize), (usize, usize))>> = vec![None; walks.len()];
    for (face, list) in pieces.iter().enumerate() {
        for (i, one) in list.iter().enumerate() {
            for (j, two) in list.iter().enumerate() {
                if one.walk == two.walk || two.axis != (one.axis + 2) % 4 {
                    continue;
                }
                let along = (one.axis % 2) as usize;
                let across = 1 - along;
                let gap = (one.start[across] - two.start[across]).abs();
                if gap > tolerance.gap {
                    continue;
                }
                // Where the two stretches overlap along the line.
                let sign = axis_vector(one.axis)[along];
                let (a, b) = (one.start[along], one.start[along] + sign * one.length);
                let (c, d) = (two.start[along], two.start[along] - sign * two.length);
                let (low, high) = (a.min(b).max(c.min(d)), a.max(b).min(c.max(d)));
                if high < low - JOINED {
                    continue;
                }
                let x = 0.5 * (low + high);
                let length = one.time + (x - one.start[along]) * sign + two.time - (x - two.start[along]) * sign;
                if !tolerance.admits(gap, length) {
                    continue;
                }
                let better = first[one.walk as usize].is_none_or(|(kept, kept_gap, ..)| {
                    length < kept - JOINED || (length <= kept + JOINED && gap < kept_gap)
                });
                if better {
                    first[one.walk as usize] = Some((length, gap, two.walk, (face, i), (face, j)));
                }
            }
        }
    }

    let mut pairs = Vec::new();
    for (id, found) in first.iter().enumerate() {
        let Some((length, gap, other, (face, i), (_, j))) = *found else { continue };
        let mutual = first[other as usize].is_some_and(|(.., back, _, _)| back as usize == id);
        if !mutual || other as usize <= id {
            continue;
        }
        let (one, two) = (&pieces[face][i], &pieces[face][j]);
        let across = 1 - (one.axis % 2);
        let mine = anchor(&walks[id], one.index);
        let theirs = anchor(&walks[other as usize], two.index);
        let mut terms: Vec<(u32, u8, f64)> = mine[across as usize].clone();
        terms.extend(
            theirs[across as usize].iter().map(|&(corner, coordinate, coefficient)| (corner, coordinate, -coefficient)),
        );
        let (walk, partner) = (&walks[id], &walks[other as usize]);
        let mut faces: Vec<u32> =
            walk.faces[..=one.index as usize].iter().chain(&partner.faces[..=two.index as usize]).copied().collect();
        faces.sort_unstable();
        faces.dedup();
        pairs.push(Pair {
            ends: [(walk.vertex, walk.rank), (partner.vertex, partner.rank)],
            length,
            gap,
            condition: Condition { terms: merged(terms), target: 0.0 },
            faces,
        });
    }
    pairs.sort_by(|a, b| a.length.total_cmp(&b.length).then(a.ends.cmp(&b.ends)));
    pairs
}

/// Walks one separatrix through the map, alone, laying its pieces and
/// noting where it changes chart. It stops at a wall, at a singular vertex,
/// or past `reach`.
#[allow(clippy::too_many_arguments)]
fn walk_one(
    map: &Map,
    wedge: &[u32],
    is_singular: &[bool],
    is_wall: &[bool],
    on_wall: &[bool],
    reach: f64,
    (face, corner, axis): (usize, usize, Axis),
    id: u32,
    walk: &mut Walk,
    pieces: &mut [Vec<Piece>],
) {
    let (mut face, mut point, mut axis, mut time, mut index) = (face, map.uv[3 * face + corner], axis, 0.0, 0u32);
    // Far more triangles than a walk of that length crosses.
    for _ in 0..1024 + 64 * reach as usize {
        if time > reach {
            return;
        }
        let Some((exit, _, distance)) = map.exit(face, point, axis) else { return };
        if distance > 0.0 {
            pieces[face].push(Piece { walk: id, index, start: point, axis, length: distance, time });
            walk.faces.push(face as u32);
            index += 1;
        }
        time += distance;
        match exit {
            Exit::Edge { edge, t } => {
                let h = 3 * face + edge;
                if is_wall[h] {
                    return;
                }
                let Some((next, at, carried)) = map.cross_edge(face, edge, t, axis) else { return };
                if map.seam[h] {
                    // The vertex the edge leaves from, seen from either side.
                    let twin = map.topology.opposite[h] as usize;
                    let to = (3 * (twin / 3) + (twin % 3 + 1) % 3) as u32;
                    walk.jumps.push(Jump { at: index, from: h as u32, to, turn: (axis + 4 - carried) % 4 });
                }
                (face, point, axis) = (next, at, carried);
            }
            Exit::Vertex { corner } => {
                let vertex = map.mesh.triangles[face][corner] as usize;
                if is_singular[vertex] || on_wall[vertex] {
                    return;
                }
                let Some((next, at, carried)) = map.enter_from_vertex(face, corner, axis) else { return };
                let (from, to) = ((3 * face + corner) as u32, (3 * next + at) as u32);
                let turn = (axis + 4 - carried) % 4;
                if turn != 0 || wedge[from as usize] != wedge[to as usize] {
                    walk.jumps.push(Jump { at: index, from, to, turn });
                }
                (face, point, axis) = (next, map.uv[3 * next + at], carried);
            }
        }
    }
}

/// A point of the line of `walk`, its vertex, in the chart of its piece of
/// rank `index`.
fn anchor(walk: &Walk, index: u32) -> Anchor {
    let mut point: Anchor = [vec![(walk.corner, 0, 1.0)], vec![(walk.corner, 1, 1.0)]];
    for jump in walk.jumps.iter().take_while(|jump| jump.at <= index) {
        // Across a seam the map turns and shifts: z' = (-i)^turn (z - z(from))
        // + z(to), with z(from) and z(to) the one vertex in either chart.
        point[0].push((jump.from, 0, -1.0));
        point[1].push((jump.from, 1, -1.0));
        let [u, v] = point;
        let negated = |row: Vec<(u32, u8, f64)>| row.into_iter().map(|(c, k, x)| (c, k, -x)).collect::<Vec<_>>();
        point = match jump.turn {
            0 => [u, v],
            1 => [v, negated(u)],
            2 => [negated(u), negated(v)],
            _ => [negated(v), u],
        };
        point[0].push((jump.to, 0, 1.0));
        point[1].push((jump.to, 1, 1.0));
    }
    point
}

/// Sums the coefficients of repeated coordinates and drops those that
/// cancel.
fn merged(mut terms: Vec<(u32, u8, f64)>) -> Vec<(u32, u8, f64)> {
    terms.sort_unstable_by_key(|&(corner, coordinate, _)| (corner, coordinate));
    let mut out: Vec<(u32, u8, f64)> = Vec::with_capacity(terms.len());
    for (corner, coordinate, coefficient) in terms {
        match out.last_mut() {
            Some(last) if (last.0, last.1) == (corner, coordinate) => last.2 += coefficient,
            _ => out.push((corner, coordinate, coefficient)),
        }
    }
    out.retain(|&(_, _, coefficient)| coefficient != 0.0);
    out
}
