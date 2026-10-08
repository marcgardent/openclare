//! Sizing: how large the quads should be, place by place.
//!
//! The result is one factor per vertex of the work mesh, by which the edge
//! length asked for is multiplied there. It comes from two sources that
//! simply multiply: the shape (smaller quads where the surface bends more)
//! and what the user painted (a weight per vertex of the input). The field
//! is then smoothed, since the map can only follow a size that changes
//! gently, and scaled so that the number of quads stays the one asked for.
//!
//! On top of that comes a bound: a thin part, a finger say, gets quads small
//! enough for a given number of them to go round it, without which it comes
//! out flat and its volume is lost. The other quads grow to make up for it.
//!
//! The stage is a [`Sizer`]; chains get one from [`create`].

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
    /// Edge length of the quads asked for.
    pub edge_length: f64,
    /// How much the size follows the shape: 0 keeps every quad the same
    /// size; at 1 the error to the surface is evened out, quads being as
    /// small as the surface bends.
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
    /// Largest factor the shape alone may put between the edge length asked
    /// for and the local one, either way.
    pub shape_range: f64,
    /// Length, in edges asked for, over which curvature is read: details
    /// smaller than that do not call for smaller quads.
    pub reading_scale: f64,
    /// Length, in edges asked for, over which the size is smoothed.
    pub smoothing_scale: f64,
    /// Largest factor by which thin parts may make quads smaller than asked
    /// for: thinner than that, a part is beyond what the budget can render.
    /// At 6, quads that small on the thinnest sheets (the ears of a face)
    /// came out inverted now and then; at 4 fingers are kept just as well.
    pub thin_range: f64,
    /// How fast the size may grow away from a thin part: by this much of an
    /// edge asked for, per edge asked for travelled.
    pub gradation: f64,
}

impl Default for Curvature {
    fn default() -> Self {
        Self { shape_range: 3.0, reading_scale: 0.5, smoothing_scale: 1.5, thin_range: 4.0, gradation: 0.5 }
    }
}

impl Sizer for Curvature {
    fn kind(&self) -> Kind {
        Kind::Curvature
    }

    fn size(&self, work: &WorkMesh, request: &Request) -> Result<Vec<f64>> {
        if !(0.0..=1.0).contains(&request.adaptivity) {
            return Err(Error::InvalidParams(format!("adaptivity {} is not between 0 and 1", request.adaptivity)));
        }
        let mesh = &work.mesh;
        let vertices = mesh.positions.len();
        if !(request.quads_around.is_finite() && request.quads_around >= 0.0) {
            return Err(Error::InvalidParams(format!(
                "quads around a thin part: {} is not a number of quads",
                request.quads_around
            )));
        }
        if request.adaptivity == 0.0 && request.painted.is_none() && request.quads_around == 0.0 {
            return Ok(vec![1.0; vertices]);
        }
        let topology = Topology::build(mesh);
        let work_edge = mesh.edge_length_stats().1.max(f64::MIN_POSITIVE);
        // n smoothing passes reach about sqrt(n) edges.
        let passes_over = |length: f64| ((length / work_edge).powi(2).round() as usize).clamp(1, 400);

        // Logarithm of the factor: the two sources add up.
        let mut log_scale = vec![0.0; vertices];
        if request.adaptivity > 0.0 {
            let bending = TensorField::estimate(mesh, &work.normals, &work.areas)
                .smoothed(
                    &topology.neighbors,
                    &work.normals,
                    &work.areas,
                    passes_over(self.reading_scale * request.edge_length),
                )
                .principal(&work.normals);
            // A chord of length l on a surface of curvature k strays from it
            // by k l² / 8: for the same error everywhere, l goes as 1/√k.
            // Curvature below what the edge length can feel does not count.
            let floor = 0.05 / request.edge_length;
            let strongest: Vec<f64> =
                (0..vertices).map(|v| bending.k1[v].abs().max(bending.k2[v].abs()) + floor).collect();
            let typical = weighted_mean(&strongest.iter().map(|k| k.ln()).collect::<Vec<_>>(), &work.areas).exp();
            let limit = self.shape_range.ln();
            for v in 0..vertices {
                let even_error = 0.5 * (typical / strongest[v]).ln();
                log_scale[v] += (request.adaptivity * even_error).clamp(-limit, limit);
            }
        }
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
            // the inverse square root of that. Where the level sits does not
            // matter: the whole field is rescaled below.
            let surface = Bvh::build(painted.source);
            let full = painted.contrast.ln();
            for v in 0..vertices {
                let Some(hit) = surface.closest_point(mesh.positions[v]) else { continue };
                let triangle = painted.source.triangles[hit.triangle as usize];
                let share = barycentric(painted.source.corners(hit.triangle as usize), hit.point);
                let weight: f64 = (0..3).map(|k| share[k] * painted.weights[triangle[k] as usize]).sum();
                log_scale[v] -= 0.5 * weight * full;
            }
        }

        // The map can only follow a size that changes gently.
        for _ in 0..passes_over(self.smoothing_scale * request.edge_length) {
            let previous = log_scale.clone();
            for v in 0..vertices {
                let (mut sum, mut weight) = (previous[v] * work.areas[v], work.areas[v]);
                for &nb in &topology.neighbors[v] {
                    sum += previous[nb as usize] * work.areas[nb as usize];
                    weight += work.areas[nb as usize];
                }
                if weight > 0.0 {
                    log_scale[v] = sum / weight;
                }
            }
        }

        let mut scale: Vec<f64> = log_scale.into_iter().map(f64::exp).collect();
        keep_the_count(&mut scale, &work.areas);

        if request.quads_around > 0.0 {
            // Beyond this, a part is thick enough whatever the size there.
            let reach = request.quads_around * request.edge_length * self.shape_range;
            let girth = thickness::measure(mesh, &work.normals, reach).girth;
            let smallest = 1.0 / self.thin_range;
            let most: Vec<f64> =
                girth.iter().map(|g| (g / (request.quads_around * request.edge_length)).max(smallest)).collect();
            // Making the thin parts finer makes the rest coarser to keep the
            // count, and that moves the bound in turn: a few rounds settle it.
            for _ in 0..THIN_ROUNDS {
                for v in 0..vertices {
                    scale[v] = scale[v].min(most[v]);
                }
                grade(&mut scale, mesh, &topology, self.gradation / request.edge_length);
                keep_the_count(&mut scale, &work.areas);
            }
        }
        Ok(scale)
    }
}

/// Rounds of bounding the size on thin parts and restoring the count.
const THIN_ROUNDS: usize = 6;

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

/// Lowers the factors so that none grows faster than `rate` per unit of
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
