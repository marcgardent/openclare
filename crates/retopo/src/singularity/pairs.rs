//! Greedy cancellation of opposite singularities.
//!
//! A +1/4 and a -1/4 singularity joined by a path of edges can be made to
//! annihilate by shifting the period jump of every edge on the path: the
//! index leaves one end, passes through and cancels at the other. The field
//! is then re-smoothed, and the change is kept only if that did not cost too
//! much energy. Pairs are tried nearest first.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::angle_field::AngleField;
use super::{Kind, Report, Request, SingularityOptimizer};
use crate::crossfield::{CrossField, FaceFrames, prepare};
use crate::error::Result;
use crate::math::INVALID;
use crate::mesh::Topology;
use crate::preprocess::WorkMesh;

#[derive(Clone, Debug)]
pub struct PairCancellation {
    /// Most cancellations to try: each costs a solve.
    pub max_trials: usize,
}

impl Default for PairCancellation {
    fn default() -> Self {
        Self { max_trials: 2000 }
    }
}

impl SingularityOptimizer for PairCancellation {
    fn kind(&self) -> Kind {
        Kind::PairCancellation
    }

    fn optimize(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<(CrossField, Report)> {
        let (topology, frames) = prepare(&work.mesh)?;
        let graph = Graph::new(work, &topology);
        let mut angles = AngleField::new(work, &topology, &frames, field, request.alignment);
        let everything: Vec<u32> = (0..work.mesh.triangles.len() as u32).collect();
        let total_energy = |angles: &mut AngleField| {
            angles.select(&everything, true);
            let energy = angles.energy(&everything).total();
            angles.select(&everything, false);
            energy
        };
        let mut report = Report {
            singularities_before: graph.singular_vertices(&angles).len(),
            energy_before: total_energy(&mut angles),
            ..Default::default()
        };

        report.split = graph.split_higher_order(&mut angles, &frames);

        // Candidates are gathered in rounds: each positive singularity with
        // its few nearest negative ones, all tried nearest first. A new round
        // follows as long as the last one removed something, since that frees
        // singularities to pair up differently.
        let mut rejected: HashSet<(u32, u32)> = HashSet::new();
        'rounds: loop {
            let mut progressed = false;
            for path in graph.candidates(&angles, request.reach, &rejected) {
                if report.tried >= self.max_trials {
                    break 'rounds;
                }
                let (positive, negative) = (path[0], *path.last().unwrap());
                // Either end may have gone in an earlier cancellation of this round.
                if graph.index(&angles, positive) <= 0 || graph.index(&angles, negative) >= 0 {
                    continue;
                }
                report.tried += 1;
                // The field is only re-smoothed around the path: elsewhere it
                // stays exactly what the cross field stage computed.
                let region = graph.faces_around(&path, &frames);
                angles.select(&region, true);
                let before = angles.energy(&region);
                let saved = angles.snapshot(&region);
                graph.shift_along(&mut angles, &path, 1);
                angles.solve(&region);
                let after = angles.energy(&region);
                // With two singularities fewer the field is usually smoother;
                // what has to be watched is that it does not let go of its
                // targets.
                let allowance = request.tolerance * before.total();
                let affordable =
                    after.total() <= before.total() + allowance && after.alignment <= before.alignment + allowance;
                if affordable {
                    report.cancelled += 1;
                    progressed = true;
                } else {
                    graph.shift_along(&mut angles, &path, -1);
                    angles.restore(&region, saved);
                    rejected.insert((positive, negative));
                }
                angles.select(&region, false);
            }
            if !progressed {
                break;
            }
        }

        if report.cancelled == 0 && report.split == 0 {
            report.singularities_after = report.singularities_before;
            report.energy_after = report.energy_before;
            return Ok((field.clone(), report));
        }
        report.energy_after = total_energy(&mut angles);
        report.singularities_after = graph.singular_vertices(&angles).len();
        Ok((angles.into_field(), report))
    }
}

/// The vertex graph of the work mesh, restricted to interior edges.
struct Graph<'a> {
    work: &'a WorkMesh,
    topology: &'a Topology,
    /// Half-edge from `a` to `b`, for every directed edge.
    half_edge: HashMap<(u32, u32), u32>,
    /// Half-edges leaving each vertex.
    outgoing: Vec<Vec<u32>>,
}

impl<'a> Graph<'a> {
    fn new(work: &'a WorkMesh, topology: &'a Topology) -> Self {
        let mesh = &work.mesh;
        let mut half_edge = HashMap::with_capacity(3 * mesh.triangles.len());
        let mut outgoing = vec![Vec::new(); mesh.positions.len()];
        for (f, tri) in mesh.triangles.iter().enumerate() {
            for k in 0..3 {
                let h = (3 * f + k) as u32;
                half_edge.insert((tri[k], tri[(k + 1) % 3]), h);
                outgoing[tri[k] as usize].push(h);
            }
        }
        Self { work, topology, half_edge, outgoing }
    }

    fn index(&self, angles: &AngleField, v: u32) -> i32 {
        if angles.on_border[v as usize] { 0 } else { angles.index(v as usize, &self.outgoing[v as usize]) }
    }

    fn singular_vertices(&self, angles: &AngleField) -> Vec<(u32, i32)> {
        (0..self.work.mesh.positions.len() as u32)
            .map(|v| (v, self.index(angles, v)))
            .filter(|&(_, index)| index != 0)
            .collect()
    }

    /// An edge both of whose sides exist, as its half-edge from `a` to `b`.
    fn interior_half_edge(&self, a: u32, b: u32) -> Option<u32> {
        let h = *self.half_edge.get(&(a, b))?;
        (self.topology.opposite[h as usize] != INVALID).then_some(h)
    }

    /// Moves a quarter turn of index along `path`, from its first vertex to
    /// its last (`amount = 1`) or back (`amount = -1`).
    fn shift_along(&self, angles: &mut AngleField, path: &[u32], amount: i32) {
        for step in path.windows(2) {
            let h = self.interior_half_edge(step[0], step[1]).expect("paths only use interior edges");
            angles.shift(h as usize, amount);
        }
    }

    /// A singularity of two quarter turns or more is spread over its
    /// neighbours, one quarter turn each, so that only simple ones remain.
    fn split_higher_order(&self, angles: &mut AngleField, frames: &FaceFrames) -> usize {
        let mut split = 0;
        for (v, index) in self.singular_vertices(angles) {
            let mut excess = index.abs() - 1;
            for &neighbor in &self.topology.neighbors[v as usize] {
                if excess == 0 {
                    break;
                }
                let free = !angles.on_border[neighbor as usize] && self.index(angles, neighbor) == 0;
                if !free || self.interior_half_edge(v, neighbor).is_none() {
                    continue;
                }
                // A positive index is pushed out to the neighbour; a negative
                // one is made up by pulling a positive quarter in from it.
                let path = if index > 0 { [v, neighbor] } else { [neighbor, v] };
                self.shift_along(angles, &path, 1);
                let region = self.faces_around(&path, frames);
                angles.select(&region, true);
                angles.solve(&region);
                angles.select(&region, false);
                excess -= 1;
                split += 1;
            }
        }
        split
    }

    /// The faces within a few rings of `path`: enough for the field to
    /// absorb a change made along it.
    fn faces_around(&self, path: &[u32], frames: &FaceFrames) -> Vec<u32> {
        let rings = 6 + 2 * path.len();
        let mut seen: HashSet<u32> = HashSet::new();
        let mut faces: Vec<u32> = Vec::new();
        for &v in path {
            for &h in &self.outgoing[v as usize] {
                if seen.insert(h / 3) {
                    faces.push(h / 3);
                }
            }
        }
        let mut ring_start = 0;
        for _ in 0..rings {
            let ring_end = faces.len();
            for i in ring_start..ring_end {
                let f = faces[i] as usize;
                for h in 3 * f..3 * f + 3 {
                    let g = frames.across[h];
                    if g != INVALID && seen.insert(g) {
                        faces.push(g);
                    }
                }
            }
            ring_start = ring_end;
        }
        faces
    }

    /// Every positive singularity paired with its nearest negative ones, as
    /// paths of vertices from the positive to the negative end, shortest
    /// first. If its closest partners cannot be afforded, farther ones will
    /// not be either, so only a few are proposed.
    fn candidates(&self, angles: &AngleField, reach: Option<f64>, rejected: &HashSet<(u32, u32)>) -> Vec<Vec<u32>> {
        const PARTNERS: usize = 3;
        let singular: HashMap<u32, i32> = self.singular_vertices(angles).into_iter().collect();
        // In a fixed order, so that ties do not depend on hashing.
        let mut positive: Vec<u32> = singular.iter().filter(|&(_, &index)| index > 0).map(|(&v, _)| v).collect();
        positive.sort_unstable();

        let limit = reach.unwrap_or(f64::INFINITY);
        let mut found: Vec<(f64, Vec<u32>)> = positive
            .into_iter()
            .flat_map(|start| self.nearest_negatives(angles, start, &singular, limit, rejected, PARTNERS))
            .collect();
        found.sort_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        found.into_iter().map(|(_, path)| path).collect()
    }

    /// Dijkstra from `start` over interior edges, up to `limit`, collecting
    /// the first `count` negative singularities met. Other singular or border
    /// vertices are not walked through.
    fn nearest_negatives(
        &self,
        angles: &AngleField,
        start: u32,
        singular: &HashMap<u32, i32>,
        limit: f64,
        rejected: &HashSet<(u32, u32)>,
        count: usize,
    ) -> Vec<(f64, Vec<u32>)> {
        let positions = &self.work.mesh.positions;
        let mut found = Vec::new();
        let mut distance: HashMap<u32, f64> = HashMap::from([(start, 0.0)]);
        let mut previous: HashMap<u32, u32> = HashMap::new();
        // Distances are never negative, so their bit patterns order like they do.
        let mut queue = BinaryHeap::from([Reverse((0u64, start))]);
        while let Some(Reverse((bits, v))) = queue.pop() {
            let reached = f64::from_bits(bits);
            if reached > distance[&v] {
                continue;
            }
            if singular.get(&v).is_some_and(|&index| index < 0) && !rejected.contains(&(start, v)) {
                let mut path = vec![v];
                while let Some(&before) = previous.get(path.last().unwrap()) {
                    path.push(before);
                }
                path.reverse();
                found.push((reached, path));
                if found.len() == count {
                    break;
                }
            }
            if v != start && (singular.contains_key(&v) || angles.on_border[v as usize]) {
                continue;
            }
            for &next in &self.topology.neighbors[v as usize] {
                if self.interior_half_edge(v, next).is_none() {
                    continue;
                }
                let through = reached + positions[v as usize].distance(positions[next as usize]);
                if through < limit && distance.get(&next).is_none_or(|&known| through < known) {
                    distance.insert(next, through);
                    previous.insert(next, v);
                    queue.push(Reverse((through.to_bits(), next)));
                }
            }
        }
        found
    }
}
