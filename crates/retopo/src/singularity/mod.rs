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
    /// usually describes a real feature. `None` puts no limit.
    pub reach: Option<f64>,
}

impl Default for Request {
    fn default() -> Self {
        Self { alignment: 0.05, tolerance: 1.0, reach: None }
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
