//! One grid of quads per rectangle of the T-mesh.
//!
//! The connectivity is that of the quantization. A vertex on an arc is
//! placed along the trace the arc is a piece of; a vertex inside a rectangle
//! is placed by starting from the rim and walking the seamless map inwards.
//! The whole grid of a rectangle is thereby stretched over its real extent:
//! where the whole lengths differ from the real ones, grid lines bend at the
//! rims, which is left to the relaxation that follows.

use super::{Kind, Mesher, Report};
use crate::error::{Error, Result};
use crate::math::Vec3;
use crate::mesh::{PolyMesh, Topology};
use crate::preprocess::WorkMesh;
use crate::quantization::{Quantization, Source};
use crate::tmesh::walk::Map;
use crate::tmesh::{HalfArc, Layout, NodeKind, TMesh};

pub struct PatchGrid;

impl Mesher for PatchGrid {
    fn kind(&self) -> Kind {
        Kind::PatchGrid
    }

    fn mesh(&self, work: &WorkMesh, layout: &Layout, quantization: &Quantization) -> Result<(PolyMesh, Report)> {
        let tmesh = &layout.tmesh;
        if !quantization.report.is_valid() || quantization.lengths.len() != tmesh.arcs.len() {
            return Err(Error::InvalidMesh("the quantization does not stand for a sound quad mesh".into()));
        }
        let lattice = quantization.lattice(tmesh);
        let topology = Topology::build(&work.mesh);
        let cut = &layout.map.cut;
        let map = Map::new(&work.mesh, &topology, &layout.map.uv, &cut.seam, &cut.transition);
        let rims: Vec<Option<Rim>> =
            (0..tmesh.patches.len()).map(|patch| Rim::new(tmesh, &quantization.lengths, patch)).collect();

        let mut report = Report { vertices: lattice.sources.len(), quads: lattice.quads.len(), ..Default::default() };
        let node_on_feature =
            |node: u32| tmesh.nodes[node as usize].arcs.iter().any(|&half| tmesh.arcs[half as usize / 2].feature);
        // Each vertex goes where its points are on average. But some points
        // outrank the others: those on a border or a hard edge keep the
        // vertex on it, and a singularity or a corner keeps it where it is.
        const FREE: u8 = 0;
        const ON_FEATURE: u8 = 1;
        const PINNED: u8 = 2;
        let mut sum = vec![(Vec3::ZERO, 0.0); lattice.sources.len()];
        let mut rank = vec![FREE; lattice.sources.len()];
        let mut corner = vec![false; lattice.sources.len()];
        for (&source, &vertex) in lattice.points.iter().zip(&lattice.point_vertex) {
            let vertex = vertex as usize;
            let (position, standing) = match source {
                Source::Node(node) => {
                    let data = &tmesh.nodes[node as usize];
                    let walled = node_on_feature(node);
                    corner[vertex] |= data.kind == NodeKind::Corner || (data.kind != NodeKind::Junction && walled);
                    let standing = match data.kind {
                        NodeKind::Junction if walled => ON_FEATURE,
                        NodeKind::Junction => FREE,
                        _ => PINNED,
                    };
                    (data.position, standing)
                }
                Source::Arc { arc, step } => {
                    let data = &tmesh.arcs[arc as usize];
                    let along = step as f64 / quantization.lengths[arc as usize] as f64;
                    let time = data.span[0] + along * (data.span[1] - data.span[0]);
                    let position = match tmesh.traces[data.trace as usize].locate(time) {
                        Some((face, point, _)) => map.position(face, point),
                        None => {
                            report.interpolated += 1;
                            let [from, to] = data.nodes.map(|node| tmesh.nodes[node as usize].position);
                            from.lerp(to, along)
                        }
                    };
                    (position, if data.feature { ON_FEATURE } else { FREE })
                }
                Source::Patch { patch, i, j } => {
                    let rim = rims[patch as usize].as_ref().expect("a patch with a grid is a rectangle");
                    let (u, v) = rim.real(i as usize, j as usize);
                    let position = match rim.read(tmesh, &map, u, v) {
                        Some(position) => position,
                        None => {
                            report.interpolated += 1;
                            rim.between(tmesh, &map, i as usize, j as usize)
                        }
                    };
                    (position, FREE)
                }
            };
            // The first pinned point wins; otherwise only the points of the
            // highest standing seen so far are averaged.
            if standing > rank[vertex] {
                rank[vertex] = standing;
                sum[vertex] = (Vec3::ZERO, 0.0);
            }
            if standing == rank[vertex] && !(standing == PINNED && sum[vertex].1 > 0.0) {
                sum[vertex].0 += position;
                sum[vertex].1 += 1.0;
            }
        }
        for vertex in 0..rank.len() as u32 {
            if corner[vertex as usize] {
                report.on_corner.push(vertex);
            } else if rank[vertex as usize] == ON_FEATURE {
                report.on_feature.push(vertex);
            }
        }
        let positions = sum.into_iter().map(|(total, count)| total / f64::max(count, 1.0)).collect();
        let faces = lattice.quads.iter().map(|quad| quad.to_vec()).collect();
        Ok((PolyMesh { positions, faces }, report))
    }
}

/// The rim of a rectangle, laid out for reading points inside it.
struct Rim {
    /// Per side, its half-arcs with the real distance from the start of the
    /// side at which each begins.
    halves: [Vec<(HalfArc, f64)>; 4],
    /// Real length of each side.
    real: [f64; 4],
    /// Per side, the real distance from its start of each grid point on it.
    marks: [Vec<f64>; 4],
}

impl Rim {
    fn new(tmesh: &TMesh, lengths: &[u32], patch: usize) -> Option<Self> {
        let patch = &tmesh.patches[patch];
        let side_of = patch.side_of_each()?;
        let count = patch.boundary.len();
        let start = (0..count).find(|&at| side_of[at] == 0 && side_of[(at + count - 1) % count] == 3)?;
        let mut rim = Rim { halves: Default::default(), real: [0.0; 4], marks: Default::default() };
        for at in (0..count).map(|step| (start + step) % count) {
            let (half, side) = (patch.boundary[at], side_of[at] as usize);
            let (real, whole) = (tmesh.arcs[half as usize / 2].length, lengths[half as usize / 2]);
            let from = rim.real[side];
            if rim.marks[side].is_empty() {
                rim.marks[side].push(0.0);
            }
            rim.marks[side].extend((1..=whole).map(|step| from + real * step as f64 / whole as f64));
            rim.halves[side].push((half, from));
            rim.real[side] += real;
        }
        let (w, h) = (rim.marks[0].len(), rim.marks[1].len());
        (rim.marks[2].len() == w && rim.marks[3].len() == h).then_some(rim)
    }

    /// Real coordinates, from the first corner along the first two sides,
    /// of grid point `(i, j)`: between where its grid lines leave the rim.
    fn real(&self, i: usize, j: usize) -> (f64, f64) {
        let (w, h) = (self.marks[0].len() - 1, self.marks[1].len() - 1);
        let (across, up) = (i as f64 / w as f64, j as f64 / h as f64);
        // The far sides run backwards round the rim.
        let u = (1.0 - up) * self.marks[0][i] + up * (self.real[2] - self.marks[2][w - i]);
        let v = (1.0 - across) * (self.real[3] - self.marks[3][h - j]) + across * self.marks[1][j];
        (u.clamp(0.0, self.real[0]), v.clamp(0.0, self.real[1]))
    }

    /// The point of a side at real distance `along` from its start: the
    /// triangle, the point in its coordinates, and the direction into the
    /// rectangle there.
    fn on_side(&self, tmesh: &TMesh, side: usize, along: f64) -> Option<(usize, [f64; 2], u8)> {
        let halves = &self.halves[side];
        let after = halves.partition_point(|&(_, from)| from <= along);
        let &(half, from) = halves.get(after.max(1) - 1)?;
        let arc = &tmesh.arcs[half as usize / 2];
        let forward = half.is_multiple_of(2);
        let run = (along - from).clamp(0.0, arc.length);
        let time = if forward { arc.span[0] + run } else { arc.span[1] - run };
        let (face, point, axis) = tmesh.traces[arc.trace as usize].locate(time)?;
        // The rectangle is on the left of its rim.
        Some((face, point, (axis + if forward { 1 } else { 3 }) % 4))
    }

    /// The point of the surface at real coordinates `(u, v)` of the
    /// rectangle: `u` along the first side, then `v` straight in.
    fn read(&self, tmesh: &TMesh, map: &Map, u: f64, v: f64) -> Option<Vec3> {
        let (face, point, inwards) = self.on_side(tmesh, 0, u)?;
        let (face, point) = map.walk(face, point, inwards, v)?;
        Some(map.position(face, point))
    }

    /// Grid point `(i, j)` put between the four points where its grid lines
    /// leave the rim, for when the map cannot be read.
    fn between(&self, tmesh: &TMesh, map: &Map, i: usize, j: usize) -> Vec3 {
        let (w, h) = (self.marks[0].len() - 1, self.marks[1].len() - 1);
        let on = |side: usize, mark: usize| {
            self.on_side(tmesh, side, self.marks[side][mark]).map(|(face, point, _)| map.position(face, point))
        };
        let ends = [(on(0, i), on(2, w - i), j as f64 / h as f64), (on(3, h - j), on(1, j), i as f64 / w as f64)];
        let mut sum = Vec3::ZERO;
        let mut count = 0.0;
        for (near, far, along) in ends {
            if let (Some(near), Some(far)) = (near, far) {
                sum += near.lerp(far, along);
                count += 1.0;
            }
        }
        sum / f64::max(count, 1.0)
    }
}
