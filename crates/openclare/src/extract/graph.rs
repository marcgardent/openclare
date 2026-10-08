use crate::field::rosy::{compat_orientation, compat_position_index};
use crate::hierarchy::Level;
use crate::math::{Vec3, tangent_frame};

/// The quad mesh as a graph embedded on the surface: one vertex per lattice
/// point, one edge per unit lattice step.
#[derive(Clone, Debug, Default)]
pub struct LatticeGraph {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    /// Number of input vertices collapsed into each vertex; 0 once removed.
    pub members: Vec<u32>,
    pub adjacency: Vec<Vec<u32>>,
}

impl LatticeGraph {
    /// Classifies every input edge by the integer offset between the lattice
    /// points of its two ends: zero means "same output vertex" (collapse),
    /// one unit step means "output edge", anything else is dropped.
    pub fn from_fields(level: &Level, scale: f64) -> Self {
        let n = level.len();
        let inv_scale = 1.0 / scale;
        let (v, nrm, q, o) = (&level.positions, &level.normals, &level.orientation, &level.origin);

        let mut collapses: Vec<(f64, u32, u32)> = Vec::new();
        let mut unit_edges: Vec<(u32, u32)> = Vec::new();
        for i in 0..n {
            for link in level.graph.neighbors(i) {
                let j = link.id as usize;
                if j <= i {
                    continue;
                }
                let (qi, qj) = compat_orientation(q[i], nrm[i], q[j], nrm[j]);
                let (ci, cj, error) =
                    compat_position_index(v[i], nrm[i], qi, o[i], v[j], nrm[j], qj, o[j], scale, inv_scale);
                match ((ci[0] - cj[0]).abs(), (ci[1] - cj[1]).abs()) {
                    (0, 0) => collapses.push((error, i as u32, j as u32)),
                    (1, 0) | (0, 1) => unit_edges.push((i as u32, j as u32)),
                    _ => {}
                }
            }
        }

        // Collapse most reliable matches first, and never merge two clusters
        // already joined by a lattice edge: they are distinct lattice points.
        let mut sets = DisjointSets::new(n);
        let mut cluster_links: Vec<Vec<u32>> = vec![Vec::new(); n];
        for &(a, b) in &unit_edges {
            cluster_links[a as usize].push(b);
            cluster_links[b as usize].push(a);
        }
        collapses.sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
        for &(_, i, j) in &collapses {
            let (a, b) = (sets.find(i), sets.find(j));
            if a == b {
                continue;
            }
            let (small, large) =
                if cluster_links[a as usize].len() < cluster_links[b as usize].len() { (a, b) } else { (b, a) };
            if cluster_links[small as usize].iter().any(|&k| sets.find(k) == large) {
                continue;
            }
            let root = sets.union(a, b);
            let absorbed = if root == a { b } else { a };
            let moved = std::mem::take(&mut cluster_links[absorbed as usize]);
            cluster_links[root as usize].extend(moved);
        }

        // Number the clusters and average their members. Members sitting
        // close to their lattice point are the most trustworthy.
        let mut cluster = vec![u32::MAX; n];
        let mut graph = LatticeGraph::default();
        let mut weights: Vec<f64> = Vec::new();
        for i in 0..n {
            let root = sets.find(i as u32) as usize;
            if cluster[root] == u32::MAX {
                cluster[root] = graph.positions.len() as u32;
                graph.positions.push(Vec3::ZERO);
                graph.normals.push(Vec3::ZERO);
                graph.members.push(0);
                weights.push(0.0);
            }
            let c = cluster[root] as usize;
            cluster[i] = c as u32;
            let w = (-o[i].distance_squared(v[i]) * inv_scale * inv_scale * 9.0).exp();
            graph.positions[c] += o[i] * w;
            graph.normals[c] += nrm[i] * w;
            graph.members[c] += 1;
            weights[c] += w;
        }
        for c in 0..graph.positions.len() {
            graph.positions[c] /= weights[c];
            graph.normals[c] = graph.normals[c].normalize_or_zero();
        }

        graph.adjacency = vec![Vec::new(); graph.positions.len()];
        for &(i, j) in &unit_edges {
            let (a, b) = (cluster[i as usize], cluster[j as usize]);
            if a != b {
                graph.adjacency[a as usize].push(b);
                graph.adjacency[b as usize].push(a);
            }
        }
        for list in &mut graph.adjacency {
            list.sort_unstable();
            list.dedup();
        }
        graph
    }

    pub fn has_edge(&self, a: u32, b: u32) -> bool {
        self.adjacency[a as usize].contains(&b)
    }

    fn add_edge(&mut self, a: u32, b: u32) {
        if a != b && !self.has_edge(a, b) {
            self.adjacency[a as usize].push(b);
            self.adjacency[b as usize].push(a);
        }
    }

    fn remove_edge(&mut self, a: u32, b: u32) {
        self.adjacency[a as usize].retain(|&v| v != b);
        self.adjacency[b as usize].retain(|&v| v != a);
    }

    fn remove_vertex(&mut self, v: u32) {
        for nb in std::mem::take(&mut self.adjacency[v as usize]) {
            self.adjacency[nb as usize].retain(|&x| x != v);
        }
        self.members[v as usize] = 0;
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency.iter().map(Vec::len).sum::<usize>() / 2
    }

    /// Drops clusters backed by far fewer input vertices than average: they
    /// are noise around position singularities, not lattice points.
    pub fn remove_spurious_vertices(&mut self) {
        let live = self.members.iter().filter(|&&m| m > 0).count().max(1);
        let average = self.members.iter().sum::<u32>() as f64 / live as f64;
        for v in 0..self.members.len() {
            if self.members[v] > 0 && (self.members[v] as f64) < 0.1 * average {
                self.remove_vertex(v as u32);
            }
        }
    }

    /// Height of `i` above segment `j-k` if `i` sits almost on it.
    fn height_over_edge(&self, i: u32, j: u32, k: u32, threshold: f64) -> Option<f64> {
        let (pi, pj, pk) = (self.positions[i as usize], self.positions[j as usize], self.positions[k as usize]);
        let (a, b, c) = (pj.distance(pk), pi.distance(pj), pi.distance(pk));
        if a <= b.max(c) {
            return None;
        }
        let s = 0.5 * (a + b + c);
        let height = 2.0 * (s * (s - a) * (s - b) * (s - c)).max(0.0).sqrt() / a;
        (height < threshold).then_some(height)
    }

    /// Cleans up around T-junctions: when a vertex `i` lies on an edge `j-k`
    /// it is either merged into the end it nearly coincides with, or the edge
    /// is rerouted through it (`j-i-k`).
    pub fn remove_redundant_edges(&mut self, scale: f64) {
        let threshold = 0.3 * scale;
        for _ in 0..64 {
            let mut candidates: Vec<(f64, u32, u32, u32)> = Vec::new();
            for i in 0..self.adjacency.len() as u32 {
                for &j in &self.adjacency[i as usize] {
                    for &k in &self.adjacency[j as usize] {
                        if k != i
                            && let Some(height) = self.height_over_edge(i, j, k, threshold)
                        {
                            candidates.push((height, i, j, k));
                        }
                    }
                }
            }
            candidates.sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then((a.1, a.2, a.3).cmp(&(b.1, b.2, b.3))));

            let mut changed = false;
            for &(_, i, j, k) in &candidates {
                // Earlier fixes in this pass may have invalidated the candidate.
                if !self.has_edge(i, j) || !self.has_edge(j, k) || self.height_over_edge(i, j, k, threshold).is_none() {
                    continue;
                }
                let pi = self.positions[i as usize];
                let to_j = pi.distance(self.positions[j as usize]);
                let to_k = pi.distance(self.positions[k as usize]);
                if to_j < threshold || to_k < threshold {
                    self.merge_into(i, if to_j < threshold { j } else { k });
                } else {
                    self.remove_edge(j, k);
                    self.add_edge(i, k);
                }
                changed = true;
            }
            if !changed {
                break;
            }
        }
    }

    fn merge_into(&mut self, from: u32, into: u32) {
        let (f, t) = (from as usize, into as usize);
        let (wf, wt) = (self.members[f] as f64, self.members[t] as f64);
        self.positions[t] = (self.positions[t] * wt + self.positions[f] * wf) / (wt + wf);
        self.normals[t] = (self.normals[t] * wt + self.normals[f] * wf).normalize_or(self.normals[t]);
        self.members[t] += self.members[f];
        let neighbors = self.adjacency[f].clone();
        self.remove_vertex(from);
        for nb in neighbors {
            self.add_edge(into, nb);
        }
    }

    /// Repeatedly removes edges that lead to a dead end: they cannot bound a
    /// face.
    pub fn prune_dangling_edges(&mut self) {
        let mut stack: Vec<u32> =
            (0..self.adjacency.len() as u32).filter(|&v| self.adjacency[v as usize].len() == 1).collect();
        while let Some(v) = stack.pop() {
            if let [nb] = self.adjacency[v as usize][..] {
                self.remove_edge(v, nb);
                if self.adjacency[nb as usize].len() == 1 {
                    stack.push(nb);
                }
            }
        }
    }

    /// Orders every neighbour list counter-clockwise around the vertex
    /// normal, which turns the graph into a rotation system (a combinatorial
    /// embedding) from which faces can be traced.
    pub fn sort_counter_clockwise(&mut self) {
        for v in 0..self.adjacency.len() {
            let (s, t) = tangent_frame(self.normals[v]);
            let p = self.positions[v];
            let mut keyed: Vec<(f64, u32)> = self.adjacency[v]
                .iter()
                .map(|&nb| {
                    let d = self.positions[nb as usize] - p;
                    (t.dot(d).atan2(s.dot(d)), nb)
                })
                .collect();
            keyed.sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            self.adjacency[v] = keyed.into_iter().map(|(_, nb)| nb).collect();
        }
    }
}

struct DisjointSets {
    parent: Vec<u32>,
    size: Vec<u32>,
}

impl DisjointSets {
    fn new(n: usize) -> Self {
        Self { parent: (0..n as u32).collect(), size: vec![1; n] }
    }

    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let grandparent = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = grandparent;
            x = grandparent;
        }
        x
    }

    /// Unites the sets rooted at `a` and `b`; returns the surviving root.
    fn union(&mut self, a: u32, b: u32) -> u32 {
        let (root, child) = if self.size[a as usize] >= self.size[b as usize] { (a, b) } else { (b, a) };
        self.parent[child as usize] = root;
        self.size[root as usize] += self.size[child as usize];
        root
    }
}
