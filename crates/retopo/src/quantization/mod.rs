//! Quantization (stage M6): a whole length for every arc of the T-mesh.
//!
//! The quad mesh to come has a whole number of edges along every arc. Those
//! numbers must agree on the opposite sides of each rectangle, stay close to
//! the real lengths, add up to the number of quads asked for, and leave the
//! structure intact: an arc may shrink to nothing, which closes up a sliver
//! between two cuts, but not so that two singularities land on each other
//! or a tube is pinched shut.
//!
//! The stage is a [`Quantizer`]; chains get one from [`create`].

mod flow;
mod lattice;

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::math::Rng;
use crate::tmesh::TMesh;
use flow::{Cost, Network};
pub use lattice::{Lattice, Source};

#[derive(Clone, Debug)]
pub struct Request {
    /// Number of quads to aim for. `None` keeps the scale of the T-mesh,
    /// whose lengths are already counted in quad edges.
    pub quads: Option<usize>,
    /// How far from that number is close enough, as a fraction of it.
    pub tolerance: f64,
}

impl Default for Request {
    fn default() -> Self {
        Self { quads: None, tolerance: 0.02 }
    }
}

/// What a quantizer achieved.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    pub arcs: usize,
    /// Quads the lengths add up to, and how far that is from the number
    /// asked for, as a fraction of it.
    pub quads: usize,
    pub budget_error: Option<f64>,
    /// Factor applied to the real lengths to get there.
    pub scale: f64,
    /// Arcs given no length, and rectangles left with no area.
    pub zero_arcs: usize,
    pub collapsed_patches: usize,
    /// Arcs that had to be kept from shrinking to nothing for the structure
    /// to stay intact.
    pub held_arcs: usize,
    /// Mean and largest distance between a whole length and the scaled real
    /// one, in quad edges.
    pub deviation_mean: f64,
    pub deviation_max: f64,
    /// The opposite sides of every rectangle add up to the same.
    pub balanced: bool,
    /// The quad mesh the lengths stand for is a manifold with the topology
    /// of the surface, and its irregular vertices are the singularities of
    /// the T-mesh: no two of them merged, nothing pinched.
    pub separated: bool,
    /// Improving moves made by the solver, over all its runs.
    pub moves: usize,
}

impl Report {
    pub fn is_valid(&self) -> bool {
        self.balanced && self.separated
    }
}

/// The whole lengths of a T-mesh.
#[derive(Clone, Debug)]
pub struct Quantization {
    /// Per arc of the T-mesh, in quad edges.
    pub lengths: Vec<u32>,
    pub report: Report,
}

impl Quantization {
    /// Connectivity of the quad mesh these lengths stand for on `tmesh`.
    pub fn lattice(&self, tmesh: &TMesh) -> Lattice {
        let lengths: Vec<i64> = self.lengths.iter().map(|&length| length as i64).collect();
        lattice::build(tmesh, &lengths).lattice
    }

    /// Width and height of each patch of `tmesh` under these lengths: its
    /// first two sides.
    pub fn patch_sizes(&self, tmesh: &TMesh) -> Vec<[u32; 2]> {
        tmesh
            .patches
            .iter()
            .map(|patch| {
                let mut size = [0; 2];
                if let Some(side_of) = patch.side_of_each() {
                    for (&half, &side) in patch.boundary.iter().zip(&side_of) {
                        if side < 2 {
                            size[side as usize] += self.lengths[half as usize / 2];
                        }
                    }
                }
                size
            })
            .collect()
    }
}

pub trait Quantizer {
    fn kind(&self) -> Kind;
    fn quantize(&self, tmesh: &TMesh, request: &Request) -> Result<Quantization>;
}

/// The quantizers [`create`] knows how to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Minimum-deviation flow on the strip network (after Heistermann et al.
    /// 2023), solved approximately by cancelling improving walks.
    WalkCancelling,
}

impl Kind {
    pub const ALL: &[Kind] = &[Kind::WalkCancelling];

    pub fn name(self) -> &'static str {
        match self {
            Kind::WalkCancelling => "walk-cancelling",
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
            Error::InvalidParams(format!("unknown quantizer '{name}' (known: {})", known.join(", ")))
        })
    }
}

/// Builds the quantizer of that kind with its default tuning.
pub fn create(kind: Kind) -> Box<dyn Quantizer> {
    match kind {
        Kind::WalkCancelling => Box::new(WalkCancelling::default()),
    }
}

#[derive(Clone, Debug)]
pub struct WalkCancelling {
    /// Most changes of scale tried to reach the number of quads asked for.
    pub scale_rounds: usize,
    /// Most rounds of holding arcs open to keep the structure intact.
    pub separation_rounds: usize,
    /// Spread of the preference given to each arc, as a fraction of its
    /// length, so that equal arcs do not all change together.
    pub jitter: f64,
}

impl Default for WalkCancelling {
    fn default() -> Self {
        Self { scale_rounds: 24, separation_rounds: 200, jitter: 0.01 }
    }
}

impl Quantizer for WalkCancelling {
    fn kind(&self) -> Kind {
        Kind::WalkCancelling
    }

    fn quantize(&self, tmesh: &TMesh, request: &Request) -> Result<Quantization> {
        if tmesh.patches.iter().any(|patch| patch.sides.is_none()) {
            return Err(Error::InvalidMesh("the T-mesh is not a partition into rectangles".into()));
        }
        if request.quads == Some(0) || !(request.tolerance.is_finite() && request.tolerance >= 0.0) {
            return Err(Error::InvalidParams("quantization needs a positive number of quads and tolerance".into()));
        }
        let arcs = tmesh.arcs.len();
        let network = strip_network(tmesh);
        let exact: Vec<f64> = tmesh.arcs.iter().map(|arc| arc.length).collect();
        // Arcs of equal length (a cube has nothing else) would all gain or
        // lose an edge at the same scale, and the number of quads would jump
        // by whole layers. A slight, fixed preference per arc spreads them out.
        let real: Vec<f64> = exact
            .iter()
            .enumerate()
            .map(|(arc, length)| length * (1.0 + self.jitter * (Rng::new(arc as u64).next_f64() - 0.5)))
            .collect();

        let mut x = vec![0i64; arcs];
        let mut lower = vec![0i64; arcs];
        let mut report = Report { arcs, scale: 1.0, separated: true, ..Default::default() };
        // The best assignment seen: lengths, scale, quads.
        let mut best: Option<(Vec<i64>, f64, usize)> = None;
        let mut scale = 1.0;
        // Scales known to give too few quads and too many.
        let (mut too_small, mut too_large) = (None::<f64>, None::<f64>);
        let rounds = if request.quads.is_some() { self.scale_rounds } else { 1 };
        for _ in 0..rounds {
            let target: Vec<f64> = real.iter().map(|length| scale * length).collect();
            // Relative to the length: one edge more or less matters on a
            // short arc, not on a long one.
            let weight: Vec<f64> = target.iter().map(|length| 1.0 / (length + 1.0)).collect();
            let mut separated = false;
            for _ in 0..self.separation_rounds {
                let cost = Cost { target: &target, weight: &weight, lower: &lower };
                report.moves += network.improve(&mut x, &cost).moves;
                // The lengths are sound if the quad mesh they stand for is;
                // where it is not, something closed up that must not.
                let defects = lattice::build(tmesh, &x).defects(tmesh);
                if defects.reopen.is_empty() {
                    separated = defects.is_empty();
                    break;
                }
                for arc in defects.reopen {
                    lower[arc] = 1;
                }
            }
            report.separated &= separated;

            let quads = count_quads(tmesh, &x);
            let wanted = request.quads.unwrap_or(quads);
            let miss = |count: usize| count.abs_diff(wanted);
            if best.as_ref().is_none_or(|(_, _, kept)| miss(quads) < miss(*kept)) {
                best = Some((x.clone(), scale, quads));
            }
            if miss(quads) as f64 <= request.tolerance * wanted as f64 {
                break;
            }
            if quads < wanted {
                too_small = Some(scale);
            } else {
                too_large = Some(scale);
            }
            scale = match (too_small, too_large) {
                (Some(low), Some(high)) => 0.5 * (low + high),
                // Quads go as the square of the scale; with none at all,
                // anything larger is a start.
                _ if quads == 0 => 2.0 * scale,
                _ => scale * (wanted as f64 / quads as f64).sqrt(),
            };
        }

        let (x, scale, quads) = best.unwrap_or((x, 1.0, 0));
        report.scale = scale;
        report.quads = quads;
        report.budget_error = request.quads.map(|wanted| quads.abs_diff(wanted) as f64 / wanted as f64);
        report.balanced = network.is_balanced(&x);
        let built = lattice::build(tmesh, &x);
        report.separated &= built.defects(tmesh).is_empty() && built.euler() == tmesh.report.euler;
        report.zero_arcs = x.iter().filter(|&&length| length == 0).count();
        report.held_arcs = lower.iter().zip(&x).filter(|&(&bound, &length)| bound > 0 && length == bound).count();
        let deviations: Vec<f64> =
            x.iter().zip(&exact).map(|(&length, real)| (length as f64 - scale * real).abs()).collect();
        report.deviation_mean = deviations.iter().sum::<f64>() / arcs.max(1) as f64;
        report.deviation_max = deviations.iter().fold(0.0, |a, &b| a.max(b));

        let quantization = Quantization { lengths: x.iter().map(|&length| length as u32).collect(), report };
        let collapsed = quantization.patch_sizes(tmesh).iter().filter(|size| size[0] == 0 || size[1] == 0).count();
        Ok(Quantization { report: Report { collapsed_patches: collapsed, ..quantization.report }, ..quantization })
    }
}

/// The strip network of `tmesh`: one station per pair of opposite sides of
/// each patch, each arc tied to the side it lies on in the patch on its
/// left and in the patch on its right.
fn strip_network(tmesh: &TMesh) -> Network {
    let mut ends = vec![[None; 2]; tmesh.arcs.len()];
    for (index, patch) in tmesh.patches.iter().enumerate() {
        let Some(side_of) = patch.side_of_each() else { continue };
        for (&half, &side) in patch.boundary.iter().zip(&side_of) {
            let station = 2 * index as u32 + side as u32 % 2;
            ends[half as usize / 2][half as usize % 2] = Some(2 * station + side as u32 / 2);
        }
    }
    Network::new(ends)
}

fn count_quads(tmesh: &TMesh, x: &[i64]) -> usize {
    tmesh
        .patches
        .iter()
        .map(|patch| {
            let mut size = [0i64; 2];
            if let Some(side_of) = patch.side_of_each() {
                for (&half, &side) in patch.boundary.iter().zip(&side_of) {
                    if side < 2 {
                        size[side as usize] += x[half as usize / 2];
                    }
                }
            }
            (size[0] * size[1]) as usize
        })
        .sum()
}
