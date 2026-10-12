//! Sizing: how large the quads should be, place by place.
//!
//! The shape says it. At each point it calls for a size, by one rule: so
//! many quads to go round. Round a thin part, a finger say, that is its
//! girth over that number, without which it comes out flat and its volume
//! is lost. Where the surface bends more tightly than the largest quad goes
//! round in a given number, quads get smaller for the error to the surface
//! to stay even. What the user painted multiplies it. The size is that, kept
//! between a smallest and a largest quad: the largest wherever the shape
//! asks for nothing smaller, the smallest where it would ask for less. It is
//! then smoothed and graded, since the map can only follow a size that
//! changes gently.
//!
//! The number of quads is what that makes, and no setting of its own. Asked
//! for a number, the sizer sets the bounds from the edge of as many quads of
//! one size and looks for how much a bend may ask, that gives the number:
//! the same rule, read the other way.
//!
//! What a chain is given is one factor per vertex of the work mesh, by which
//! the edge length of as many quads of one size is multiplied there.
//!
//! The stage is a [`Sizer`]; chains get one from [`create`]. [`Demand`] is
//! what the shape calls for before any bound: measured once, it tells at
//! once how many quads any pair of bounds makes.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt;
use std::str::FromStr;

use crate::curvature::TensorField;
use crate::error::{Error, Result};
use crate::mesh::{Topology, TriMesh};
use crate::preprocess::WorkMesh;
use crate::spatial::Bvh;
use crate::thickness;

/// A density painted on the input, as weights: 0 where `source` should be
/// least dense, 1 where it should be most, anything in between in between.
#[derive(Clone, Copy)]
pub struct Painted<'a> {
    pub source: &'a TriMesh,
    /// One weight per vertex of `source`, in `0..=1`.
    pub weights: &'a [f64],
    /// How many times more quads per unit of area a weight of 1 gets than a
    /// weight of 0.
    pub contrast: f64,
}

#[derive(Clone, Copy)]
pub struct Request<'a> {
    /// Edge length of as many quads of one size as there are to be: what
    /// the factors are of, and what sets the number of quads.
    pub edge_length: f64,
    /// The largest and the smallest edge a quad may have, in that order.
    /// Without, they are set from `edge_length`, and what a bend may ask is
    /// looked for that makes the number of quads it stands for (see
    /// [`Curvature::coarsest`]).
    pub bounds: Option<(f64, f64)>,
    /// How much the size follows the way the surface bends: 0 not at all;
    /// at 1 the error to the surface is evened out, quads being as small
    /// as the surface bends.
    pub adaptivity: f64,
    pub painted: Option<Painted<'a>>,
    /// Fewest quads that should go round a thin part; 0 not to care.
    pub quads_around: f64,
}

pub trait Sizer {
    fn kind(&self) -> Kind;
    /// The factor on the edge length at each vertex of `work`.
    fn size(&self, work: &WorkMesh, request: &Request) -> Result<Vec<f64>>;
}

/// The sizers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// From the curvature of the surface and the painted density.
    Curvature,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Curvature];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Curvature => "curvature",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        Kind::ALL.iter().copied().find(|k| k.name() == name).ok_or_else(|| {
            let known: Vec<&str> = Kind::ALL.iter().map(|k| k.name()).collect();
            Error::InvalidParams(format!("unknown sizer '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the sizer of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn Sizer> {
    match kind {
        Kind::Curvature => Box::new(Curvature::default()),
    }
}

#[derive(Clone, Debug)]
pub struct Curvature {
    /// Quads to a full turn of the tightest bend that counts as flat: one
    /// that the largest quad goes round in that many. Tighter, a bend gets
    /// smaller quads.
    pub quads_a_turn: f64,
    /// When a number of quads is asked for and not their size: how many
    /// times larger than the edge of as many quads of one size the largest
    /// may be, and how many times smaller the smallest. Thinner than that
    /// last one allows, a part is beyond what the number can render.
    pub coarsest: f64,
    pub finest: f64,
    /// Length, in edges asked for, over which curvature is read: details
    /// smaller than that do not call for smaller quads.
    pub reading_scale: f64,
    /// Length, in edges asked for, over which the size is smoothed.
    pub smoothing_scale: f64,
    /// How far across a part is looked for its other side, in edges asked
    /// for and for each quad wanted round it: thicker than that, a part is
    /// thick enough whatever the size there.
    pub thin_reach: f64,
    /// How fast the size may grow away from a smaller one: by this much of
    /// a length, per length travelled.
    pub gradation: f64,
}

impl Default for Curvature {
    fn default() -> Self {
        Self { quads_a_turn: 12.0, coarsest: 3.0, finest: 4.0, reading_scale: 0.5, smoothing_scale: 1.5, thin_reach: 3.0, gradation: 0.5 }
    }
}

/// What the shape calls for at each vertex of a work mesh, before any bound
/// on the size of the quads. Measuring it is the long part; what a pair of
/// bounds makes of it is then told at once.
pub struct Demand<'a> {
    work: &'a WorkMesh,
    topology: Topology,
    /// The circle the surface bends along, over the quads to a turn: the
    /// largest quad that goes round it in that many. Infinite where
    /// nothing bends.
    bend: Vec<f64>,
    /// The edge a thin part calls for: its girth over the quads wanted
    /// round it. Infinite where nothing is thin.
    thin: Vec<f64>,
    /// What the paint multiplies the size by, the weight 0 being 1.
    paint: Vec<f64>,
    adaptivity: f64,
    /// How many times the size is smoothed over the neighbours.
    passes: usize,
    gradation: f64,
}

impl<'a> Demand<'a> {
    /// Measures what `work` calls for under `request`, whose `edge_length`
    /// says about how large the quads are to be: curvature is read, the
    /// size smoothed and thin parts looked for at that scale.
    pub fn measure(tuning: &Curvature, work: &'a WorkMesh, request: &Request) -> Result<Self> {
        if !(0.0..=1.0).contains(&request.adaptivity) {
            return Err(Error::InvalidParams(format!("adaptivity {} is not between 0 and 1", request.adaptivity)));
        }
        if !(request.quads_around.is_finite() && request.quads_around >= 0.0) {
            return Err(Error::InvalidParams(format!(
                "quads around a thin part: {} is not a number of quads",
                request.quads_around
            )));
        }
        if !(request.edge_length.is_finite() && request.edge_length > 0.0) {
            return Err(Error::InvalidParams(format!("edge length {} is not positive", request.edge_length)));
        }
        let mesh = &work.mesh;
        let vertices = mesh.positions.len();
        let topology = Topology::build(mesh);
        let work_edge = mesh.edge_length_stats().1.max(f64::MIN_POSITIVE);
        // n smoothing passes reach about sqrt(n) edges.
        let passes_over = |length: f64| ((length / work_edge).powi(2).round() as usize).clamp(1, 400);

        let mut bend = vec![f64::INFINITY; vertices];
        if request.adaptivity > 0.0 {
            let bending = TensorField::estimate(mesh, &work.normals, &work.areas)
                .smoothed(
                    &topology.neighbors,
                    &work.normals,
                    &work.areas,
                    passes_over(tuning.reading_scale * request.edge_length),
                )
                .principal(&work.normals);
            for v in 0..vertices {
                let strongest = bending.k1[v].abs().max(bending.k2[v].abs());
                // A circle of radius 1 / k, in so many quads.
                bend[v] = std::f64::consts::TAU / (strongest * tuning.quads_a_turn);
            }
        }

        let mut thin = vec![f64::INFINITY; vertices];
        if request.quads_around > 0.0 {
            let reach = request.quads_around * request.edge_length * tuning.thin_reach;
            let girth = thickness::measure(mesh, &work.normals, reach).girth;
            for v in 0..vertices {
                thin[v] = girth[v] / request.quads_around;
            }
        }

        let mut paint = vec![1.0; vertices];
        if let Some(painted) = request.painted {
            if painted.weights.len() != painted.source.positions.len() {
                return Err(Error::InvalidParams(format!(
                    "{} weights painted for {} vertices",
                    painted.weights.len(),
                    painted.source.positions.len()
                )));
            }
            if painted.weights.iter().any(|w| !(0.0..=1.0).contains(w)) {
                return Err(Error::InvalidParams("painted weights must lie between 0 and 1".into()));
            }
            if !(painted.contrast.is_finite() && painted.contrast >= 1.0) {
                return Err(Error::InvalidParams(format!("density contrast {} is not 1 or more", painted.contrast)));
            }
            // Each vertex of the work mesh reads the paint at the nearest
            // point of the input. A weight of 1 means `contrast` times the
            // quads per area of a weight of 0, and the edge length goes as
            // the inverse square root of that. Only the difference between
            // weights counts: the least painted is the weight 0.
            let surface = Bvh::build(painted.source);
            let full = painted.contrast.ln();
            let least = painted.weights.iter().copied().fold(f64::INFINITY, f64::min);
            for v in 0..vertices {
                let Some(hit) = surface.closest_point(mesh.positions[v]) else { continue };
                let triangle = painted.source.triangles[hit.triangle as usize];
                let share = barycentric(painted.source.corners(hit.triangle as usize), hit.point);
                let weight: f64 = (0..3).map(|k| share[k] * painted.weights[triangle[k] as usize]).sum();
                paint[v] = (-0.5 * (weight - least).max(0.0) * full).exp();
            }
        }

        Ok(Self {
            work,
            topology,
            bend,
            thin,
            paint,
            adaptivity: request.adaptivity,
            passes: passes_over(tuning.smoothing_scale * request.edge_length),
            gradation: tuning.gradation,
        })
    }

    /// The edge length at each vertex for quads between `smallest` and
    /// `largest`. `flat` is the edge a bend is measured against: one that
    /// calls for that much gets it, a tighter one an edge between the two,
    /// by the adaptivity. With sizes asked for it is the largest quad: what
    /// calls for that much or more is flat.
    pub fn sizes(&self, flat: f64, largest: f64, smallest: f64) -> Vec<f64> {
        let (mesh, areas) = (&self.work.mesh, &self.work.areas);
        let vertices = mesh.positions.len();
        // What the bend and the paint ask, as a logarithm, smoothed: the map
        // can only follow a size that changes gently.
        let mut asked: Vec<f64> =
            (0..vertices).map(|v| self.by_bend(v, flat).min(largest).ln() + self.paint[v].ln()).collect();
        let level = asked.first().copied().unwrap_or(0.0);
        if asked.iter().any(|&a| a != level) {
            for _ in 0..self.passes {
                let previous = asked.clone();
                for v in 0..vertices {
                    let (mut sum, mut weight) = (previous[v] * areas[v], areas[v]);
                    for &nb in &self.topology.neighbors[v] {
                        sum += previous[nb as usize] * areas[nb as usize];
                        weight += areas[nb as usize];
                    }
                    if weight > 0.0 {
                        asked[v] = sum / weight;
                    }
                }
            }
        }
        // A thin part is not smoothed away: its size is a bound, and the
        // others come down to it gently.
        let mut sizes: Vec<f64> =
            (0..vertices).map(|v| asked[v].exp().min(self.thin[v]).clamp(smallest, largest)).collect();
        let least = sizes.iter().copied().fold(f64::INFINITY, f64::min);
        if sizes.iter().any(|&size| size > least) {
            grade(&mut sizes, mesh, &self.topology, self.gradation);
        }
        sizes
    }

    /// The edge the bend at `v` calls for, measured against `flat`. A chord
    /// of length l on a circle of radius r strays from it by l² / 8r: for
    /// the same error everywhere l goes as √r, which is half way, as a
    /// logarithm, between `flat` and the edge of so many quads to a turn.
    /// That is an adaptivity of 1; less is nearer `flat`. A bend that asks
    /// for `flat` gets it; a gentler one more, as far as the largest quad
    /// allows.
    fn by_bend(&self, v: usize, flat: f64) -> f64 {
        if self.adaptivity == 0.0 { flat } else { flat * (self.bend[v] / flat).powf(0.5 * self.adaptivity) }
    }

    /// How many quads that makes: a region of area A holds A / s² of them.
    pub fn quads(&self, flat: f64, largest: f64, smallest: f64) -> f64 {
        self.sizes(flat, largest, smallest).iter().zip(&self.work.areas).map(|(size, area)| area / (size * size)).sum()
    }

    /// The share of the surface that calls for quads smaller than
    /// `smallest`: what comes out coarser than its shape asks.
    pub fn below(&self, flat: f64, smallest: f64) -> f64 {
        let (mut under, mut all) = (0.0, 0.0);
        for (v, &area) in self.work.areas.iter().enumerate() {
            let wanted = self.by_bend(v, flat).min(self.thin[v]) * self.paint[v];
            under += if wanted < smallest { area } else { 0.0 };
            all += area;
        }
        if all > 0.0 { under / all } else { 0.0 }
    }

    /// The edge a bend is to be measured against for quads between
    /// `smallest` and `largest` to be `quads` in all, or as near as those
    /// bounds allow: the tighter the measure, the more quads.
    pub fn flat_for(&self, quads: f64, largest: f64, smallest: f64) -> f64 {
        let (mut low, mut high) = (smallest.ln() - 12.0, largest.ln() + 12.0);
        for _ in 0..48 {
            let middle = 0.5 * (low + high);
            if self.quads(middle.exp(), largest, smallest) > quads {
                low = middle;
            } else {
                high = middle;
            }
        }
        (0.5 * (low + high)).exp()
    }
}

impl Sizer for Curvature {
    fn kind(&self) -> Kind {
        Kind::Curvature
    }

    fn size(&self, work: &WorkMesh, request: &Request) -> Result<Vec<f64>> {
        let demand = Demand::measure(self, work, request)?;
        let vertices = work.mesh.positions.len();
        let (largest, smallest) = match request.bounds {
            Some((largest, smallest)) => {
                if !(smallest.is_finite() && smallest > 0.0 && largest.is_finite() && largest >= smallest) {
                    return Err(Error::InvalidParams(format!(
                        "quads from {smallest} to {largest}: not a smallest and a largest size"
                    )));
                }
                (largest, smallest)
            }
            None => {
                if request.adaptivity == 0.0 && request.painted.is_none() && request.quads_around == 0.0 {
                    return Ok(vec![1.0; vertices]);
                }
                (request.edge_length * self.coarsest, request.edge_length / self.finest)
            }
        };
        let flat = match request.bounds {
            Some(_) => largest,
            None => {
                let area: f64 = work.areas.iter().sum();
                demand.flat_for(area / (request.edge_length * request.edge_length), largest, smallest)
            }
        };
        let mut scale: Vec<f64> =
            demand.sizes(flat, largest, smallest).into_iter().map(|size| size / request.edge_length).collect();
        // The factors are of the edge of as many quads of one size: what is
        // left of a difference is taken out, the work mesh being another
        // than the one the number was counted on, or the bounds too tight
        // for the number asked for.
        keep_the_count(&mut scale, &work.areas);
        Ok(scale)
    }
}

/// Scales the factors so that the number of quads is the one the uniform
/// size gives: a region of area A holds A / (h s)² of them.
fn keep_the_count(scale: &mut [f64], areas: &[f64]) {
    let density = weighted_mean(&scale.iter().map(|s| 1.0 / (s * s)).collect::<Vec<_>>(), areas);
    if density.is_finite() && density > 0.0 {
        let correction = density.sqrt();
        for s in scale {
            *s *= correction;
        }
    }
}

/// Lowers the sizes so that none grows faster than `rate` per unit of
/// length away from a smaller one: the small ones are kept as they are and
/// the others follow, where smoothing would wash a small part out.
fn grade(scale: &mut [f64], mesh: &TriMesh, topology: &Topology, rate: f64) {
    // Factors are positive, so their bit patterns order like they do.
    let mut queue: BinaryHeap<Reverse<(u64, u32)>> =
        (0..scale.len()).map(|v| Reverse((scale[v].to_bits(), v as u32))).collect();
    while let Some(Reverse((bits, v))) = queue.pop() {
        let here = f64::from_bits(bits);
        if here > scale[v as usize] {
            continue;
        }
        for &next in &topology.neighbors[v as usize] {
            let allowed = here + rate * mesh.positions[v as usize].distance(mesh.positions[next as usize]);
            if allowed < scale[next as usize] {
                scale[next as usize] = allowed;
                queue.push(Reverse((allowed.to_bits(), next)));
            }
        }
    }
}

fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    let total: f64 = weights.iter().sum();
    if total > 0.0 { values.iter().zip(weights).map(|(v, w)| v * w).sum::<f64>() / total } else { 0.0 }
}

/// Barycentric coordinates of a point of a triangle.
fn barycentric([a, b, c]: [crate::math::Vec3; 3], p: crate::math::Vec3) -> [f64; 3] {
    let (ab, ac, ap) = (b - a, c - a, p - a);
    let (d00, d01, d11, d20, d21) = (ab.dot(ab), ab.dot(ac), ac.dot(ac), ap.dot(ab), ap.dot(ac));
    let det = d00 * d11 - d01 * d01;
    if det.abs() < 1e-300 {
        return [1.0, 0.0, 0.0];
    }
    let v = (d11 * d20 - d01 * d21) / det;
    let w = (d00 * d21 - d01 * d20) / det;
    [1.0 - v - w, v, w]
}
