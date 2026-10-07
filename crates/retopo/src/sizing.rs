//! Sizing: how large the quads should be, place by place.
//!
//! The result is one factor per vertex of the work mesh, by which the edge
//! length asked for is multiplied there. It comes from two sources that
//! simply multiply: the shape (smaller quads where the surface bends more)
//! and what the user painted (a density per vertex of the input). The field
//! is then smoothed, since the map can only follow a size that changes
//! gently, and scaled so that the number of quads stays the one asked for.
//!
//! The stage is a [`Sizer`]; chains get one from [`create`].

use std::fmt;
use std::str::FromStr;

use crate::curvature::TensorField;
use crate::error::{Error, Result};
use crate::mesh::{Topology, TriMesh};
use crate::preprocess::WorkMesh;
use crate::spatial::Bvh;

/// A density painted on the input: `density[v]` multiplies the number of
/// quads per unit of area around vertex `v` of `source`; 1 leaves it alone.
#[derive(Clone, Copy)]
pub struct Painted<'a> {
    pub source: &'a TriMesh,
    pub density: &'a [f64],
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
    /// The same for the painted density, as a factor on the density.
    pub painted_range: f64,
    /// Length, in edges asked for, over which curvature is read: details
    /// smaller than that do not call for smaller quads.
    pub reading_scale: f64,
    /// Length, in edges asked for, over which the size is smoothed.
    pub smoothing_scale: f64,
}

impl Default for Curvature {
    fn default() -> Self {
        Self { shape_range: 3.0, painted_range: 16.0, reading_scale: 0.5, smoothing_scale: 1.5 }
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
        if request.adaptivity == 0.0 && request.painted.is_none() {
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
            if painted.density.len() != painted.source.positions.len() {
                return Err(Error::InvalidParams(format!(
                    "{} densities painted for {} vertices",
                    painted.density.len(),
                    painted.source.positions.len()
                )));
            }
            if painted.density.iter().any(|d| !(d.is_finite() && *d > 0.0)) {
                return Err(Error::InvalidParams("painted densities must be positive numbers".into()));
            }
            // Each vertex of the work mesh reads the paint at the nearest
            // point of the input. Density counts quads per area: the edge
            // length goes as its inverse square root.
            let surface = Bvh::build(painted.source);
            let limit = self.painted_range.ln();
            for v in 0..vertices {
                let Some(hit) = surface.closest_point(mesh.positions[v]) else { continue };
                let triangle = painted.source.triangles[hit.triangle as usize];
                let weights = barycentric(painted.source.corners(hit.triangle as usize), hit.point);
                let log_density: f64 =
                    (0..3).map(|k| weights[k] * painted.density[triangle[k] as usize].ln().clamp(-limit, limit)).sum();
                log_scale[v] -= 0.5 * log_density;
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

        // Same number of quads as with the uniform size: a region of area A
        // holds A / (h s)² of them.
        let mut scale: Vec<f64> = log_scale.into_iter().map(f64::exp).collect();
        let density = weighted_mean(&scale.iter().map(|s| 1.0 / (s * s)).collect::<Vec<_>>(), &work.areas);
        if density.is_finite() && density > 0.0 {
            let correction = density.sqrt();
            for s in &mut scale {
                *s *= correction;
            }
        }
        Ok(scale)
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
