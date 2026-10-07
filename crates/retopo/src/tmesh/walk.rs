//! Walking along grid directions of a seamless map.
//!
//! Inside a triangle the map is linear, so a line of constant `u` or `v` is a
//! straight segment. Crossing an edge that is not a seam changes nothing;
//! crossing a seam turns the direction by the transition of that seam. Through
//! a vertex, the line continues in whichever triangle around it contains its
//! direction.

use crate::math::INVALID;
use crate::mesh::{Topology, TriMesh};

/// A grid direction: 0 is +u, 1 is +v, 2 is -u, 3 is -v, each a quarter turn
/// counter-clockwise from the last.
pub(crate) type Axis = u8;

/// Lengths below this, in grid units, are taken to be zero. Well above what
/// the map is accurate to (its seams close to about 1e-10), well below the
/// size of anything in it: a trace this close to a vertex goes through it,
/// two traces this close to one line are on the same line.
pub(crate) const EPSILON: f64 = 1e-6;

pub(crate) fn axis_vector(axis: Axis) -> [f64; 2] {
    [[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]][axis as usize % 4]
}

#[inline]
fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

#[inline]
fn cross(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

#[inline]
fn dot(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

/// Where a ray leaves its triangle.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Exit {
    /// Through the edge leaving corner `edge`, at fraction `t` along it.
    Edge { edge: usize, t: f64 },
    /// Through corner `corner`.
    Vertex { corner: usize },
}

/// A direction leaving a vertex: the face it enters, the corner the vertex
/// has there, and the direction in the coordinates of that face.
pub(crate) type Departure = (usize, usize, Axis);

pub(crate) struct Map<'a> {
    pub mesh: &'a TriMesh,
    pub topology: &'a Topology,
    /// Coordinates of corner `3 * f + k`.
    pub uv: &'a [[f64; 2]],
    pub seam: &'a [bool],
    pub transition: &'a [u8],
    /// One corner of each vertex, to start walking around it.
    some_corner: Vec<u32>,
}

impl<'a> Map<'a> {
    pub fn new(
        mesh: &'a TriMesh,
        topology: &'a Topology,
        uv: &'a [[f64; 2]],
        seam: &'a [bool],
        transition: &'a [u8],
    ) -> Self {
        let mut some_corner = vec![INVALID; mesh.positions.len()];
        for (corner, &v) in mesh.triangles.iter().flatten().enumerate() {
            some_corner[v as usize] = corner as u32;
        }
        Self { mesh, topology, uv, seam, transition, some_corner }
    }

    pub fn corners(&self, f: usize) -> [[f64; 2]; 3] {
        [self.uv[3 * f], self.uv[3 * f + 1], self.uv[3 * f + 2]]
    }

    /// Signed area of face `f` in the map.
    pub fn area(&self, f: usize) -> f64 {
        let [a, b, c] = self.corners(f);
        0.5 * cross(sub(b, a), sub(c, a))
    }

    /// Barycentric coordinates of `p` in the triangle of face `f`.
    pub fn barycentric(&self, f: usize, p: [f64; 2]) -> [f64; 3] {
        let [a, b, c] = self.corners(f);
        let area = cross(sub(b, a), sub(c, a));
        if area.abs() < 1e-300 {
            return [1.0, 0.0, 0.0];
        }
        let v = cross(sub(p, a), sub(c, a)) / area;
        let w = cross(sub(b, a), sub(p, a)) / area;
        [1.0 - v - w, v, w]
    }

    /// The same direction on the other side of half-edge `h`.
    pub fn axis_across(&self, h: usize, axis: Axis) -> Axis {
        if self.seam[h] { (axis + 4 - self.transition[h] % 4) % 4 } else { axis }
    }

    /// Where the line through `p` along `axis` leaves face `f`: how, the
    /// point, and how far from `p` that is. `p` is in the triangle or on its
    /// rim. `None` for a triangle squashed flat along the line.
    pub fn exit(&self, f: usize, p: [f64; 2], axis: Axis) -> Option<(Exit, [f64; 2], f64)> {
        let corners = self.corners(f);
        let along = (axis % 2) as usize;
        let across = 1 - along;
        let sign = axis_vector(axis)[along];

        // The line meets the rim of the triangle twice; the way out is the
        // further of the two along the direction.
        let mut best: Option<(Exit, [f64; 2], f64)> = None;
        for edge in 0..3 {
            let (a, b) = (corners[edge], corners[(edge + 1) % 3]);
            let rise = b[across] - a[across];
            if rise == 0.0 {
                continue;
            }
            let t = (p[across] - a[across]) / rise;
            let length = (b[0] - a[0]).hypot(b[1] - a[1]);
            // Fractions are judged against the length of the edge, so that
            // "at a corner" means the same on long and short edges.
            let margin = EPSILON / length.max(EPSILON);
            if !(-margin..=1.0 + margin).contains(&t) {
                continue;
            }
            let (exit, point) = if t <= margin {
                (Exit::Vertex { corner: edge }, a)
            } else if t >= 1.0 - margin {
                (Exit::Vertex { corner: (edge + 1) % 3 }, b)
            } else {
                (Exit::Edge { edge, t }, [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])])
            };
            let distance = (point[along] - p[along]) * sign;
            if best.as_ref().is_none_or(|(_, _, d)| distance > *d) {
                best = Some((exit, point, distance));
            }
        }
        best.map(|(exit, point, distance)| (exit, point, distance.max(0.0)))
    }

    /// The point of the surface at `p` in the coordinates of face `f`.
    pub fn position(&self, f: usize, p: [f64; 2]) -> crate::math::Vec3 {
        let weights = self.barycentric(f, p);
        let triangle = self.mesh.triangles[f];
        (0..3).map(|k| self.mesh.positions[triangle[k] as usize] * weights[k]).sum()
    }

    /// Goes `distance` from `p` in face `f` along `axis`: the face and the
    /// point reached. `None` if the way leaves the surface or gets stuck.
    pub fn walk(&self, f: usize, p: [f64; 2], axis: Axis, distance: f64) -> Option<(usize, [f64; 2])> {
        let (mut face, mut point, mut axis, mut left) = (f, p, axis, distance);
        // Far more triangles than any straight way crosses.
        for _ in 0..self.mesh.triangles.len().max(64) {
            let (exit, _, reach) = self.exit(face, point, axis)?;
            if reach >= left {
                let direction = axis_vector(axis);
                return Some((face, [point[0] + left * direction[0], point[1] + left * direction[1]]));
            }
            left -= reach;
            match exit {
                Exit::Edge { edge, t } => (face, point, axis) = self.cross_edge(face, edge, t, axis)?,
                Exit::Vertex { corner } => {
                    let (next, at, carried) = self.enter_from_vertex(face, corner, axis)?;
                    (face, point, axis) = (next, self.uv[3 * next + at], carried);
                }
            }
        }
        None
    }

    /// Crosses the edge leaving corner `edge` of face `f` at fraction `t`:
    /// the face entered, the point in its coordinates, and the direction
    /// there. `None` on a border.
    pub fn cross_edge(&self, f: usize, edge: usize, t: f64, axis: Axis) -> Option<(usize, [f64; 2], Axis)> {
        let h = 3 * f + edge;
        let twin = self.topology.opposite[h];
        if twin == INVALID {
            return None;
        }
        // The edge runs the other way in the face across.
        let twin = twin as usize;
        let g = twin / 3;
        let (start, end) = (self.uv[twin], self.uv[3 * g + (twin % 3 + 1) % 3]);
        let s = 1.0 - t;
        let point = [start[0] + s * (end[0] - start[0]), start[1] + s * (end[1] - start[1])];
        Some((g, point, self.axis_across(h, axis)))
    }

    /// Which side of the edge leaving corner `corner` of face `f` the
    /// direction `axis` lies on, seen from that corner: positive counter-
    /// clockwise of it, zero along it, negative clockwise or backwards.
    ///
    /// Axis vectors have entries 0 and ±1, so the sign is exact; the triangle
    /// on the other side of the edge asks the same question through
    /// [`Self::in_sector`] and gets the same answer. That is what makes the
    /// sectors round a vertex share out the directions without overlap or gap.
    fn side_of_first_edge(&self, f: usize, corner: usize, axis: Axis) -> f64 {
        let corners = self.corners(f);
        let first = sub(corners[(corner + 1) % 3], corners[corner]);
        let direction = axis_vector(axis);
        let side = cross(first, direction);
        if side == 0.0 && dot(first, direction) < 0.0 { -1.0 } else { side }
    }

    /// The corner sector counter-clockwise of `(f, corner)` round their
    /// vertex, and `axis` carried to it. `None` at a border.
    fn next_around(&self, f: usize, corner: usize, axis: Axis) -> Option<Departure> {
        // Counter-clockwise round the vertex is across the edge arriving at it.
        let h = 3 * f + (corner + 2) % 3;
        let twin = self.topology.opposite[h];
        (twin != INVALID).then(|| (twin as usize / 3, twin as usize % 3, self.axis_across(h, axis)))
    }

    /// The corner sector clockwise of `(f, corner)`, and `axis` carried to it.
    fn previous_around(&self, f: usize, corner: usize, axis: Axis) -> Option<Departure> {
        let h = 3 * f + corner;
        let twin = self.topology.opposite[h];
        (twin != INVALID).then(|| (twin as usize / 3, (twin as usize % 3 + 1) % 3, self.axis_across(h, axis)))
    }

    /// Does the direction `axis` leaving corner `corner` of face `f` enter
    /// that triangle? Its sector is half-open: a direction along its first
    /// edge is in, one along its second edge belongs to the next triangle
    /// round the vertex.
    pub fn in_sector(&self, f: usize, corner: usize, axis: Axis) -> bool {
        if self.side_of_first_edge(f, corner, axis) < 0.0 {
            return false;
        }
        match self.next_around(f, corner, axis) {
            Some((g, at, carried)) => {
                let corners = self.corners(g);
                let first = sub(corners[(at + 1) % 3], corners[at]);
                // Strictly clockwise of the second edge, and not pointing
                // backwards along it.
                cross(first, axis_vector(carried)) < 0.0
            }
            None => {
                let corners = self.corners(f);
                let second = sub(corners[(corner + 2) % 3], corners[corner]);
                let direction = axis_vector(axis);
                let side = cross(direction, second);
                side > 0.0 || (side == 0.0 && dot(direction, second) > 0.0)
            }
        }
    }

    /// The corner sectors round the vertex of `(f, corner)`, starting there
    /// and going counter-clockwise, each with `axis` carried to it. At a
    /// border vertex the sectors clockwise of the start come last.
    pub fn around(&self, f: usize, corner: usize, axis: Axis) -> Vec<Departure> {
        let mut out = vec![(f, corner, axis)];
        let mut at = (f, corner, axis);
        loop {
            match self.next_around(at.0, at.1, at.2) {
                Some(next) if next.0 == f => return out,
                Some(next) => {
                    out.push(next);
                    at = next;
                }
                None => break,
            }
        }
        at = (f, corner, axis);
        while let Some(previous) = self.previous_around(at.0, at.1, at.2) {
            out.push(previous);
            at = previous;
        }
        out
    }

    /// `axis`, given in the coordinates of `(f, corner)`, in those of face
    /// `target` round the same vertex. Only meaningful round a regular
    /// vertex, where the way round does not matter.
    pub fn carry_around(&self, f: usize, corner: usize, axis: Axis, target: usize) -> Option<Axis> {
        self.around(f, corner, axis).into_iter().find(|&(g, _, _)| g == target).map(|(_, _, carried)| carried)
    }

    /// The triangle round the vertex at corner `corner` of face `f` that the
    /// direction `axis` (in the coordinates of `f`) enters.
    pub fn enter_from_vertex(&self, f: usize, corner: usize, axis: Axis) -> Option<Departure> {
        self.around(f, corner, axis).into_iter().find(|&(g, at, carried)| self.in_sector(g, at, carried))
    }

    /// Every grid direction leaving vertex `v`, counter-clockwise round it.
    /// A regular vertex has four; a singular one of index `k` has `4 - k`.
    pub fn departures(&self, v: u32) -> Vec<Departure> {
        let start = self.some_corner[v as usize] as usize;
        let mut out = Vec::new();
        for (f, corner, _) in self.around(start / 3, start % 3, 0) {
            // Within one triangle, counter-clockwise is by increasing angle
            // from its first edge.
            let corners = self.corners(f);
            let first = sub(corners[(corner + 1) % 3], corners[corner]);
            let mut inside: Vec<(f64, Axis)> = (0..4)
                .filter(|&axis| self.in_sector(f, corner, axis))
                .map(|axis| (cross(first, axis_vector(axis)).atan2(dot(first, axis_vector(axis))), axis))
                .collect();
            inside.sort_by(|a, b| a.0.total_cmp(&b.0));
            out.extend(inside.into_iter().map(|(_, axis)| (f, corner, axis)));
        }
        out
    }

    /// The departure that the direction `axis`, known to leave corner
    /// `corner` of face `f` within the triangle or along one of its two
    /// edges there, really is.
    pub fn canonical(&self, f: usize, corner: usize, axis: Axis) -> Option<Departure> {
        if self.in_sector(f, corner, axis) {
            return Some((f, corner, axis));
        }
        let neighbours = [self.next_around(f, corner, axis), self.previous_around(f, corner, axis)];
        neighbours.into_iter().flatten().find(|&(g, at, carried)| self.in_sector(g, at, carried))
    }
}
