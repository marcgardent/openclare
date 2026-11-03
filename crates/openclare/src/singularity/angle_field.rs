//! A cross field written as one angle per face with the period jump of
//! every edge held fixed.
//!
//! In this form the singularities are decided by the jumps alone, and the
//! smoothest field that has them is the solution of a linear system in the
//! angles. Editing a jump therefore moves a singularity, and re-solving
//! shows what the field has to pay for it.

use std::f64::consts::{FRAC_PI_2, TAU};

use crate::crossfield::{
    CrossField, FaceFrames, angle_defects, build_field, constraints, split_quarter_turns, vertex_index,
};
use crate::math::INVALID;
use crate::mesh::Topology;
use crate::preprocess::WorkMesh;

#[derive(Clone, Copy, Debug)]
pub(super) struct Energy {
    pub smoothness: f64,
    pub alignment: f64,
}

impl Energy {
    pub fn total(self) -> f64 {
        self.smoothness + self.alignment
    }
}

pub(super) struct AngleField<'a> {
    work: &'a WorkMesh,
    frames: &'a FaceFrames,
    twin: &'a [u32],
    /// Per half-edge: quarter turns between the arms stored on its two
    /// faces; opposite on the twin.
    jumps: Vec<i32>,
    /// Per half-edge towards `g` from `f`: what `angle[g] - angle[f]` should
    /// be for the crosses to agree. Opposite on the twin.
    expected: Vec<f64>,
    /// Per face: how strongly it is tied to its target cross, and that
    /// target as a complex number (`e^{4i angle}`, any arm).
    strength: Vec<f64>,
    target_cross: Vec<f64>,
    /// The arm of the target each face currently aims at.
    target: Vec<f64>,
    pub angles: Vec<f64>,
    /// Which faces the current local solve works on, and their position
    /// among its unknowns.
    in_region: Vec<bool>,
    slot: Vec<u32>,
    defect: Vec<f64>,
    pub on_border: Vec<bool>,
}

impl<'a> AngleField<'a> {
    pub fn new(
        work: &'a WorkMesh,
        topology: &'a Topology,
        frames: &'a FaceFrames,
        field: &CrossField,
        alignment: f64,
    ) -> Self {
        let mesh = &work.mesh;
        let half_edges = 3 * mesh.triangles.len();
        let twin = &topology.opposite[..];

        // The field stores jumps modulo a full turn; here they must be the
        // actual integers, so that the angles on both sides nearly agree.
        let mut jumps = vec![0; half_edges];
        for h in 0..half_edges {
            let t = twin[h];
            if t == INVALID || (t as usize) < h {
                continue;
            }
            let (f, g) = (h / 3, t as usize / 3);
            let stored = field.period_jumps[h];
            let rest = field.angles[g] - field.angles[f] - frames.transport(h) - stored as f64 * FRAC_PI_2;
            jumps[h] = stored + 4 * (rest / TAU).round() as i32;
            jumps[t as usize] = -jumps[h];
        }

        let (strength, target_complex) = constraints(work, frames, alignment);
        let target_cross = target_complex.iter().map(|z| z.arg() / 4.0).collect();

        let (defect, on_border) = angle_defects(mesh, frames);

        let mut this = Self {
            work,
            frames,
            twin,
            jumps,
            expected: vec![0.0; half_edges],
            strength,
            target_cross,
            target: vec![0.0; mesh.triangles.len()],
            angles: field.angles.clone(),
            in_region: vec![false; mesh.triangles.len()],
            slot: vec![0; mesh.triangles.len()],
            defect,
            on_border,
        };
        for h in 0..half_edges {
            this.refresh_expected(h);
        }
        let all: Vec<u32> = (0..mesh.triangles.len() as u32).collect();
        this.aim_targets(&all);
        this
    }

    fn refresh_expected(&mut self, h: usize) {
        let t = self.twin[h];
        if t == INVALID {
            return;
        }
        // Computed from one side only, so that the two are exact opposites.
        let (low, high) = (h.min(t as usize), h.max(t as usize));
        let value = self.frames.transport(low) + self.jumps[low] as f64 * FRAC_PI_2;
        self.expected[low] = value;
        self.expected[high] = -value;
    }

    /// Points the given faces at the arm of their target nearest to their
    /// angle. Returns whether any changed.
    fn aim_targets(&mut self, faces: &[u32]) -> bool {
        let mut changed = false;
        for &f in faces {
            let f = f as usize;
            let (_, off) = split_quarter_turns(self.angles[f] - self.target_cross[f]);
            let aim = self.angles[f] - off;
            changed |= (aim - self.target[f]).abs() > 1e-9;
            self.target[f] = aim;
        }
        changed
    }

    /// The two parts of what [`Self::solve`] minimises, over the given
    /// faces: how much the field turns from face to face (edges leading out
    /// of the set included), and how far it is from its targets.
    pub fn energy(&self, faces: &[u32]) -> Energy {
        let (mut smoothness, mut alignment) = (0.0, 0.0);
        for &f in faces {
            let f = f as usize;
            for h in 3 * f..3 * f + 3 {
                let g = self.frames.across[h];
                // An edge between two faces of the set is met from both; count it once.
                if g == INVALID || (self.in_region[g as usize] && g as usize > f) {
                    continue;
                }
                let off = self.angles[g as usize] - self.angles[f] - self.expected[h];
                smoothness += self.frames.weight[h] * off * off;
            }
            let off = self.angles[f] - self.target[f];
            alignment += self.strength[f] * off * off;
        }
        Energy { smoothness, alignment }
    }

    /// Marks `faces` as the set [`Self::energy`] and [`Self::solve`] work on.
    pub fn select(&mut self, faces: &[u32], selected: bool) {
        for &f in faces {
            self.in_region[f as usize] = selected;
        }
    }

    /// Finds the angles of least energy on the selected `faces` for the
    /// current jumps, the rest of the field staying as it is.
    pub fn solve(&mut self, faces: &[u32]) {
        // A face may come to aim at another arm of its target once the field
        // has moved; a couple of rounds settle it.
        for _ in 0..3 {
            self.solve_for_current_targets(faces);
            if !self.aim_targets(faces) {
                break;
            }
        }
    }

    fn solve_for_current_targets(&mut self, faces: &[u32]) {
        let n = faces.len();
        let frames = self.frames;
        // Position of each selected face among the unknowns.
        for (slot, &f) in faces.iter().enumerate() {
            self.slot[f as usize] = slot as u32;
        }
        let slot = &self.slot;
        let in_region = &self.in_region;
        let unknown = |g: u32| g != INVALID && in_region[g as usize];

        let mut diagonal = Vec::with_capacity(n);
        let mut rhs = Vec::with_capacity(n);
        for &f in faces {
            let f = f as usize;
            let mut pull = 0.0;
            for h in 3 * f..3 * f + 3 {
                let g = frames.across[h];
                if g == INVALID {
                    continue;
                }
                pull += frames.weight[h] * self.expected[h];
                // A neighbour outside the set is a fixed value, not an unknown.
                if !in_region[g as usize] {
                    pull -= frames.weight[h] * self.angles[g as usize];
                }
            }
            diagonal.push(self.strength[f] + frames.weight[3 * f..3 * f + 3].iter().sum::<f64>() + 1e-9);
            rhs.push(self.strength[f] * self.target[f] - pull);
        }
        let apply = |x: &[f64], out: &mut [f64]| {
            for (i, &f) in faces.iter().enumerate() {
                let f = f as usize;
                let mut value = diagonal[i] * x[i];
                for h in 3 * f..3 * f + 3 {
                    let g = frames.across[h];
                    if unknown(g) {
                        value -= frames.weight[h] * x[slot[g as usize] as usize];
                    }
                }
                out[i] = value;
            }
        };
        let dot = |a: &[f64], b: &[f64]| a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>();

        // Conjugate gradients with diagonal preconditioning.
        let mut x: Vec<f64> = faces.iter().map(|&f| self.angles[f as usize]).collect();
        let mut scratch = vec![0.0; n];
        apply(&x, &mut scratch);
        let mut residual: Vec<f64> = rhs.iter().zip(&scratch).map(|(b, a)| b - a).collect();
        let mut z: Vec<f64> = residual.iter().zip(&diagonal).map(|(r, d)| r / d).collect();
        let mut direction = z.clone();
        let mut rz = dot(&residual, &z);
        let scale: f64 = rhs.iter().zip(&diagonal).map(|(b, d)| b * b / d).sum();
        let stop = 1e-14 * scale.max(1e-300);
        for _ in 0..20_000 {
            if rz <= stop {
                break;
            }
            apply(&direction, &mut scratch);
            let step = rz / dot(&direction, &scratch);
            for i in 0..n {
                x[i] += step * direction[i];
                residual[i] -= step * scratch[i];
                z[i] = residual[i] / diagonal[i];
            }
            let next = dot(&residual, &z);
            let ratio = next / rz;
            rz = next;
            for i in 0..n {
                direction[i] = z[i] + ratio * direction[i];
            }
        }
        for (i, &f) in faces.iter().enumerate() {
            self.angles[f as usize] = x[i];
        }
    }

    /// Index of the field at interior vertex `v`, in quarter turns: the one
    /// [`build_field`] gives, which the map and the T-mesh realize. It only
    /// depends on the jumps, not on the angles, and not on the drift that
    /// `expected` holds (see [`vertex_index`]).
    pub fn index(&self, v: usize, outgoing: &[u32]) -> i32 {
        vertex_index(self.defect[v], self.frames, outgoing.iter().map(|&h| (h as usize, self.jumps[h as usize])))
    }

    /// Moves one quarter turn of index from the origin of half-edge `h` to
    /// its head (`amount = 1`), or back (`amount = -1`).
    pub fn shift(&mut self, h: usize, amount: i32) {
        let t = self.twin[h] as usize;
        self.jumps[h] -= amount;
        self.jumps[t] += amount;
        self.refresh_expected(h);
    }

    /// Saves what a tentative change on `faces` may alter.
    pub fn snapshot(&self, faces: &[u32]) -> Vec<(f64, f64)> {
        faces.iter().map(|&f| (self.angles[f as usize], self.target[f as usize])).collect()
    }

    pub fn restore(&mut self, faces: &[u32], saved: Vec<(f64, f64)>) {
        for (&f, (angle, target)) in faces.iter().zip(saved) {
            self.angles[f as usize] = angle;
            self.target[f as usize] = target;
        }
    }

    pub fn into_field(self) -> CrossField {
        build_field(&self.work.mesh, self.frames, self.angles, Some(self.jumps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crossfield::{self, prepare_sized};
    use crate::mesh::primitives;
    use crate::parametrization;
    use crate::preprocess;

    /// The half-edges out of each vertex.
    fn outgoing(work: &WorkMesh) -> Vec<Vec<u32>> {
        let mut outgoing = vec![Vec::new(); work.mesh.positions.len()];
        for (f, tri) in work.mesh.triangles.iter().enumerate() {
            for k in 0..3 {
                outgoing[tri[k] as usize].push((3 * f + k) as u32);
            }
        }
        outgoing
    }

    /// The index at `v` as the optimizer counted it before ticket #279, the
    /// drift of the sizing taken in.
    fn index_with_the_drift(angles: &AngleField, v: usize, outgoing: &[u32]) -> i32 {
        let turning: f64 = outgoing.iter().map(|&h| angles.expected[h as usize]).sum();
        (4.0 * (angles.defect[v] + turning) / TAU).round() as i32
    }

    /// The index the seamless map realizes at `v`: a full turn less the
    /// angles of its triangles there in the map, in quarter turns.
    fn index_of_the_map(work: &WorkMesh, map: &parametrization::Parametrization, v: u32) -> i32 {
        let mut angles = 0.0;
        for (f, tri) in work.mesh.triangles.iter().enumerate() {
            for k in (0..3).filter(|&k| tri[k] == v) {
                let [p, next, previous] = [k, (k + 1) % 3, (k + 2) % 3].map(|c| map.uv[3 * f + c]);
                let (a, b) = ([next[0] - p[0], next[1] - p[1]], [previous[0] - p[0], previous[1] - p[1]]);
                angles += (a[0] * b[1] - a[1] * b[0]).atan2(a[0] * b[0] + a[1] * b[1]);
            }
        }
        (4.0 * (TAU - angles) / TAU).round() as i32
    }

    #[test]
    fn the_optimizer_counts_the_index_the_map_realizes_where_the_sizing_turns_the_field() {
        // A flat grid whose quads are to be much smaller at its centre than
        // around: the drift summed round the centre is -2.67 rad, more than
        // an eighth of a turn, as at jerry's spike (ticket #279).
        let segments = 12;
        let mesh = primitives::plane(segments);
        let centre = (segments / 2) * (segments + 2);
        let mut work =
            preprocess::create(preprocess::Kind::Passthrough).preprocess(&mesh, &Default::default()).unwrap();
        work.sizing[centre as usize] = (-0.8f64).exp();
        let field = crossfield::create(crossfield::Kind::Power)
            .solve(&work, &crossfield::Request { alignment: 0.0, ..Default::default() })
            .unwrap();
        let (topology, frames) = prepare_sized(&work).unwrap();
        let angles = AngleField::new(&work, &topology, &frames, &field, 0.0);

        let outgoing = outgoing(&work);
        let drift: f64 = outgoing[centre as usize].iter().map(|&h| frames.drift[h as usize]).sum();
        assert!(drift < -2.6, "drift round the centre {drift}");
        for v in (0..mesh.positions.len()).filter(|&v| !angles.on_border[v]) {
            assert_eq!(angles.index(v, &outgoing[v]), field.indices[v] as i32, "vertex {v}");
        }
        // The count the optimizer made before: the drift taken in.
        let old = index_with_the_drift(&angles, centre as usize, &outgoing[centre as usize]);
        let index = field.indices[centre as usize] as i32;
        assert_ne!(old, index, "the configuration tells the two counts apart");
        // Nor is it the sum of the jumps modulo 4, which glue the map's charts.
        let jumps: i32 = outgoing[centre as usize].iter().map(|&h| field.period_jumps[h as usize]).sum();
        assert_eq!((index - jumps).rem_euclid(4), 0);
        assert_ne!((old - jumps).rem_euclid(4), 0);

        let request = parametrization::Request { edge_length: 0.25, stiffening_rounds: 40, conditions: Vec::new() };
        let map =
            parametrization::create(parametrization::Kind::Seamless).parametrize(&work, &field, &request).unwrap();
        println!("centre: index {index}, with the drift {old}, drift {drift:.3}");
        assert_eq!(index_of_the_map(&work, &map, centre), index);
    }

    /// The faces round jerry's spike as the optimizer received them at 3000
    /// quads, the sizing and the field's angles with them, and the spike.
    fn jerry_spike() -> (WorkMesh, Vec<f64>, usize) {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/jerry_spike.obj")).unwrap();
        let mesh = crate::mesh::obj::parse_obj(text.as_bytes()).unwrap();
        let mut work =
            preprocess::create(preprocess::Kind::Passthrough).preprocess(&mesh, &Default::default()).unwrap();
        let values = |prefix: &str| -> Vec<f64> {
            text.lines()
                .filter_map(|line| line.strip_prefix(prefix))
                .map(|value| value.trim().parse().unwrap())
                .collect()
        };
        work.sizing = values("# sizing ");
        let angles = values("# angle ");
        assert_eq!((work.sizing.len(), angles.len()), (work.mesh.positions.len(), work.mesh.triangles.len()));
        (work, angles, values("# centre ")[0] as usize - 1)
    }

    #[test]
    fn at_jerrys_spike_the_optimizer_sees_the_index_the_map_has_and_splits_it() {
        let (work, angles, spike) = jerry_spike();
        let (topology, frames) = prepare_sized(&work).unwrap();
        let field = build_field(&work.mesh, &frames, angles, None);
        let outgoing = outgoing(&work);
        let drift: f64 = outgoing[spike].iter().map(|&h| frames.drift[h as usize]).sum();
        // A doublet for the map: the T-mesh would give it 2 arcs, the quad
        // mesh a vertex of valence 2.
        assert_eq!(field.indices[spike], 2, "drift round the spike {drift}");
        let optimizer = AngleField::new(&work, &topology, &frames, &field, 0.05);
        assert_eq!(index_with_the_drift(&optimizer, spike, &outgoing[spike]), 1, "what the optimizer used to see");
        for v in (0..work.mesh.positions.len()).filter(|&v| !optimizer.on_border[v]) {
            assert_eq!(optimizer.index(v, &outgoing[v]), field.indices[v] as i32, "vertex {v}");
        }

        let request = crate::singularity::Request { alignment: 0.05, tolerance: 1.0, reach: None };
        let (optimized, report) = crate::singularity::create(crate::singularity::Kind::PairCancellation)
            .optimize(&work, &field, &request)
            .unwrap();
        assert!(report.split >= 1, "{report:?}");
        assert!(optimized.indices.iter().all(|index| index.abs() < 2), "{:?}", optimized.singularities());
    }

    #[test]
    fn the_optimizer_sees_the_poles_the_user_placed() {
        // Imposed poles are made by the drift alone: the count with the
        // drift taken in did not see them.
        let mesh = primitives::icosphere(3);
        let mut work =
            preprocess::create(preprocess::Kind::Passthrough).preprocess(&mesh, &Default::default()).unwrap();
        work.imposed = vec![(0, 1), (100, -1)];
        let field = crossfield::create(crossfield::Kind::Power)
            .solve(&work, &crossfield::Request { alignment: 0.0, ..Default::default() })
            .unwrap();
        let (topology, frames) = prepare_sized(&work).unwrap();
        let angles = AngleField::new(&work, &topology, &frames, &field, 0.0);
        let outgoing = outgoing(&work);
        for &(pole, index) in &work.imposed {
            let pole = pole as usize;
            assert_eq!(field.indices[pole], index);
            assert_eq!(angles.index(pole, &outgoing[pole]), index as i32);
            assert_eq!(index_with_the_drift(&angles, pole, &outgoing[pole]), 0);
        }
        for v in 0..mesh.positions.len() {
            assert_eq!(angles.index(v, &outgoing[v]), field.indices[v] as i32, "vertex {v}");
        }
    }
}
