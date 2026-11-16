//! Singularity simplification (stage M3): fewer irregular vertices for the
//! same shape, by removing the pairs of opposite singularities a cross field
//! can do without.
//!
//! The stage is a [`SingularityOptimizer`]; chains get one from [`create`].

mod angle_field;
mod pairs;

use std::fmt;
use std::str::FromStr;

use crate::crossfield::CrossField;
use crate::error::{Error, Result};
use crate::preprocess::WorkMesh;

#[derive(Clone, Debug)]
pub struct Request {
    /// Pull towards curvature directions, as in the cross field stage: the
    /// field is re-smoothed under the same objective after every change.
    pub alignment: f64,
    /// What one cancellation may cost: how much the energy of the field
    /// around the pair (smoothness plus misfit to the curvature directions)
    /// may rise, as a fraction of what it was. 0 only accepts changes that
    /// are free; at 1 and above nearly every pair within reach goes.
    pub tolerance: f64,
    /// Farthest apart, along the surface, two singularities may be to be
    /// considered a pair. This is the main control: a pair closer than a quad
    /// or two is noise the mesh could not express anyway, while a distant one
    /// usually describes a real feature: beyond reach, a pair only goes if
    /// the field is several times smoother without it, a twist and not a
    /// feature. `None` puts no limit.
    pub reach: Option<f64>,
    /// The singularities left after the cancellations slide towards
    /// flatter places, where the field affords it (see [`Slide`]). `None`
    /// leaves them where they are.
    pub slide: Option<Slide>,
}

impl Default for Request {
    fn default() -> Self {
        Self { alignment: 0.05, tolerance: 1.0, reach: None, slide: None }
    }
}

/// An experiment for a mesh to be subdivided (#330): each simple
/// singularity slides to a place within reach where a pole costs less
/// ([`Bend`]), if the field round the way affords it. Measured on the
/// bench, it lowers the angle the reflection check gives the poles, and
/// pays for it in alignment, spirals and thin faces (ALGORITHM.md, "A
/// result to be subdivided"): it is no part of any setting, and only runs
/// when `OPENCLARE_SLIDE` asks for it ([`Slide::from_env`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slide {
    /// The edge of the quads to come.
    pub edge_length: f64,
    /// How far a singularity may go, in quad edges.
    pub reach: f64,
    /// The most a pole may cost where it goes, as a share of where it is.
    pub gain: f64,
    /// How much the energy of the field round the way may rise, as a share
    /// of what it was; and its part that pulls towards the curvature
    /// directions (infinite: not looked at apart).
    pub tolerance: f64,
    pub alignment_tolerance: f64,
    /// Places tried, the cheapest first.
    pub tries: usize,
}

impl Slide {
    /// The slide `OPENCLARE_SLIDE` asks for, for quads of `edge_length`:
    /// `reach,gain,tolerance,alignment_tolerance,tries` (`inf` for no
    /// bound), or `on` for `3,0.95,0.25,inf,3`, the tuning of the
    /// measures. `None` when it is not set or cannot be read.
    pub fn from_env(edge_length: f64) -> Option<Self> {
        let text = std::env::var("OPENCLARE_SLIDE").ok()?;
        let text = if text.trim() == "on" { "3,0.95,0.25,inf,3".to_string() } else { text };
        let values: Vec<f64> = text.split(',').map(|value| value.trim().parse().ok()).collect::<Option<_>>()?;
        let [reach, gain, tolerance, alignment_tolerance, tries] = values[..] else { return None };
        Some(Self { edge_length, reach, gain, tolerance, alignment_tolerance, tries: tries as usize })
    }
}

/// What a pole is charged when the mesh is to be subdivided: how far the
/// surface turns over one quad round it, in radians.
///
/// The reflection check gives a pole the largest angle between the normals
/// of the faces round it and their mean; on a smooth surface that angle
/// grows as the curvature across those faces, in the direction the surface
/// bends most, times their size. So a pole at vertex `v` of the work mesh
/// is charged the largest principal curvature (read at the scale of the
/// quads) met within one quad edge of it, times that edge (`edge_length`
/// times the sizing at `v`). The largest within reach and not the value at
/// `v`: the faces round a pole reach one edge out, and a pole on the rim of
/// a curved zone, its curvature averaged with the flat beside it, has faces
/// on the curve all the same.
pub struct Bend<'a> {
    work: &'a WorkMesh,
    edge_length: f64,
    neighbours: Vec<Vec<u32>>,
    most: Vec<f64>,
}

impl<'a> Bend<'a> {
    pub fn new(work: &'a WorkMesh, edge_length: f64) -> Self {
        let mesh = &work.mesh;
        let mut neighbours = vec![Vec::new(); mesh.positions.len()];
        for tri in &mesh.triangles {
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                if !neighbours[a as usize].contains(&b) {
                    neighbours[a as usize].push(b);
                }
                if !neighbours[b as usize].contains(&a) {
                    neighbours[b as usize].push(a);
                }
            }
        }
        let curvature = &work.curvature;
        let most = (0..mesh.positions.len())
            .map(|v| {
                let k1 = curvature.k1.get(v).map_or(0.0, |k| k.abs());
                k1.max(curvature.k2.get(v).map_or(0.0, |k| k.abs()))
            })
            .collect();
        Self { work, edge_length, neighbours, most }
    }

    /// The edge of the quads at `v`.
    pub fn edge(&self, v: usize) -> f64 {
        self.work.sizing.get(v).copied().unwrap_or(1.0) * self.edge_length
    }

    /// The charge of a pole at `v`, in radians.
    pub fn at(&self, v: usize) -> f64 {
        let positions = &self.work.mesh.positions;
        let (centre, reach) = (positions[v], self.edge(v));
        // Every vertex joined to `v` through vertices within reach.
        let mut seen = std::collections::HashSet::from([v as u32]);
        let mut queue = vec![v as u32];
        let mut most = self.most[v];
        while let Some(u) = queue.pop() {
            for &w in &self.neighbours[u as usize] {
                if positions[w as usize].distance(centre) <= reach && seen.insert(w) {
                    queue.push(w);
                    most = most.max(self.most[w as usize]);
                }
            }
        }
        most * reach
    }
}

/// What an optimizer did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    pub singularities_before: usize,
    pub singularities_after: usize,
    /// Pairs of opposite singularities removed.
    pub cancelled: usize,
    /// Singularities of index beyond a quarter turn split into simple ones.
    pub split: usize,
    /// Cancellations tried, accepted or not.
    pub tried: usize,
    /// Singularities slid towards a flatter place ([`Request::slide`]), and
    /// the slides tried.
    pub slid: usize,
    pub slides_tried: usize,
    pub energy_before: f64,
    pub energy_after: f64,
}

pub trait SingularityOptimizer {
    fn kind(&self) -> Kind;
    fn optimize(&self, work: &WorkMesh, field: &CrossField, request: &Request) -> Result<(CrossField, Report)>;
}

/// The optimizers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Leaves the field as the cross field stage made it.
    Keep,
    /// Splits higher-order singularities, then cancels opposite pairs
    /// greedily, nearest first, as long as the field does not suffer.
    PairCancellation,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::Keep, Kind::PairCancellation];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Keep => "keep",
            Kind::PairCancellation => "pair-cancellation",
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
            Error::InvalidParams(format!("unknown singularity optimizer '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the optimizer of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn SingularityOptimizer> {
    match kind {
        Kind::Keep => Box::new(Keep),
        Kind::PairCancellation => Box::new(pairs::PairCancellation::default()),
    }
}

/// The optimizer that does nothing.
#[derive(Clone, Copy, Debug)]
pub struct Keep;

impl SingularityOptimizer for Keep {
    fn kind(&self) -> Kind {
        Kind::Keep
    }

    fn optimize(&self, _work: &WorkMesh, field: &CrossField, _request: &Request) -> Result<(CrossField, Report)> {
        let (positive, negative, other) = field.singularities();
        let count = positive + negative + other;
        let report = Report { singularities_before: count, singularities_after: count, ..Default::default() };
        Ok((field.clone(), report))
    }
}
