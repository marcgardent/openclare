//! One configuration for a whole remeshing run, and the presets that ship
//! with the project.
//!
//! A preset is nothing more than a named configuration: every field stays
//! open to adjustment, and a configuration can be written to a small text
//! file and read back, which is all "saving a preset" takes.

use std::fmt::Write;

use crate::error::{Error, Result};
use crate::pipeline::Target;
use crate::remesher::{Guides, Method};

#[derive(Clone, Debug, PartialEq)]
pub struct RemeshConfig {
    /// Which chain does the work.
    pub method: Method,
    /// How dense the output should be.
    pub target: Target,
    /// How strongly edges are pulled towards the principal curvature
    /// directions, relative to the smoothness of their flow. 0 ignores
    /// curvature; higher follows the shape more closely, at the cost of more
    /// irregular vertices.
    pub alignment: f64,
    /// Opposite singularities closer than this many quad edges are
    /// cancelled. 0 keeps them all; higher gives fewer irregular vertices
    /// and edges that follow the shape a little less.
    pub singularity_reach: f64,
    /// What one such cancellation may cost the field around it, as a
    /// fraction of its energy. A safeguard more than a control.
    pub singularity_tolerance: f64,
    /// Dihedral angle in degrees above which an input edge is a hard edge the
    /// output must follow. `None` keeps no hard edge.
    pub crease_angle: Option<f64>,
    /// How much the size of the quads follows the shape: 0 keeps them all
    /// the same size; towards 1 they get smaller where the surface bends
    /// more, larger where it is flat, for the same number in total.
    pub adaptivity: f64,
    /// With a painted density: how many times more quads per unit of area
    /// the places painted at 1 get than those left at 0. At 4, their edges
    /// are half as long.
    pub density_contrast: f64,
    pub seed: u64,
}

impl Default for RemeshConfig {
    /// The `organic` preset at 2000 quads.
    fn default() -> Self {
        Self::for_organic(Target::QuadCount(2000))
    }
}

/// A configuration the project ships and documents.
pub struct Preset {
    pub name: &'static str,
    pub summary: &'static str,
    build: fn(Target) -> RemeshConfig,
}

impl Preset {
    pub fn config(&self, target: Target) -> RemeshConfig {
        (self.build)(target)
    }
}

pub const PRESETS: &[Preset] = &[
    Preset {
        name: "organic",
        summary: "sculpts and characters: edges follow the shape closely, quads are smaller where it bends",
        build: RemeshConfig::for_organic,
    },
    Preset {
        name: "animation",
        summary: "meshes meant to deform: fewer irregular vertices, at some cost in fidelity",
        build: RemeshConfig::for_animation,
    },
    Preset {
        name: "hard-surface",
        summary: "mechanical parts: edges sharper than 30 degrees are kept exactly",
        build: RemeshConfig::for_hard_surface,
    },
];

impl RemeshConfig {
    pub fn for_organic(target: Target) -> Self {
        Self {
            method: Method::QuadLayout,
            target,
            alignment: 0.05,
            singularity_reach: 2.0,
            singularity_tolerance: 1.0,
            crease_angle: None,
            adaptivity: 0.5,
            density_contrast: 4.0,
            seed: 1,
        }
    }

    pub fn for_animation(target: Target) -> Self {
        Self { singularity_reach: 3.0, ..Self::for_organic(target) }
    }

    pub fn for_hard_surface(target: Target) -> Self {
        // Hard edges lead; curvature only settles what they leave open. And
        // a part is cut to one size: its edges, sharp as they are, call for
        // no smaller quads.
        Self { alignment: 0.02, crease_angle: Some(30.0), adaptivity: 0.0, ..Self::for_organic(target) }
    }

    /// The shipped preset of that name.
    pub fn preset(name: &str, target: Target) -> Result<Self> {
        PRESETS.iter().find(|preset| preset.name == name).map(|preset| preset.config(target)).ok_or_else(|| {
            let known: Vec<&str> = PRESETS.iter().map(|preset| preset.name).collect();
            Error::InvalidParams(format!("unknown preset '{name}' (known: {})", known.join(", ")))
        })
    }

    /// Runs the configured chain on `input`.
    pub fn remesh(&self, input: &crate::TriMesh) -> Result<(crate::PolyMesh, crate::pipeline::Report)> {
        self.remesh_guided(input, &Guides::default())
    }

    /// Runs the configured chain on `input`, with what the user drew on it.
    pub fn remesh_guided(
        &self,
        input: &crate::TriMesh,
        guides: &Guides,
    ) -> Result<(crate::PolyMesh, crate::pipeline::Report)> {
        crate::remesher::create(self.method).remesh_guided(input, guides, self)
    }

    pub fn validate(&self) -> Result<()> {
        let non_negative = [
            ("alignment", self.alignment),
            ("singularity_reach", self.singularity_reach),
            ("singularity_tolerance", self.singularity_tolerance),
        ];
        for (name, value) in non_negative {
            if !(value.is_finite() && value >= 0.0) {
                return Err(Error::InvalidParams(format!("{name} must be a non-negative number, not {value}")));
            }
        }
        if !(0.0..=1.0).contains(&self.adaptivity) {
            return Err(Error::InvalidParams(format!("adaptivity must be between 0 and 1, not {}", self.adaptivity)));
        }
        if !(self.density_contrast.is_finite() && self.density_contrast >= 1.0) {
            return Err(Error::InvalidParams(format!(
                "density_contrast must be 1 or more, not {}",
                self.density_contrast
            )));
        }
        match self.crease_angle {
            Some(angle) if !(angle.is_finite() && angle > 0.0 && angle < 180.0) => {
                Err(Error::InvalidParams(format!("crease_angle must be between 0 and 180 degrees, not {angle}")))
            }
            _ => Ok(()),
        }
    }

    /// The configuration as `key = value` lines, one per field.
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        writeln!(text, "method = {}", self.method).unwrap();
        match self.target {
            Target::QuadCount(count) => writeln!(text, "quads = {count}"),
            Target::EdgeLength(length) => writeln!(text, "edge_length = {length}"),
        }
        .unwrap();
        writeln!(text, "alignment = {}", self.alignment).unwrap();
        writeln!(text, "singularity_reach = {}", self.singularity_reach).unwrap();
        writeln!(text, "singularity_tolerance = {}", self.singularity_tolerance).unwrap();
        match self.crease_angle {
            Some(angle) => writeln!(text, "crease_angle = {angle}"),
            None => writeln!(text, "crease_angle = none"),
        }
        .unwrap();
        writeln!(text, "adaptivity = {}", self.adaptivity).unwrap();
        writeln!(text, "density_contrast = {}", self.density_contrast).unwrap();
        writeln!(text, "seed = {}", self.seed).unwrap();
        text
    }

    /// Reads what [`Self::to_text`] writes. Fields left out keep their
    /// default; blank lines and lines starting with `#` are ignored.
    pub fn from_text(text: &str) -> Result<Self> {
        let mut config = Self::default();
        for (number, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let error = |message: String| Error::Parse { line: number + 1, message };
            let (key, value) = line.split_once('=').ok_or_else(|| error("expected 'key = value'".into()))?;
            let (key, value) = (key.trim(), value.trim());
            let number_of = |what: &str| value.parse::<f64>().map_err(|_| error(format!("'{value}' is not a {what}")));
            match key {
                "method" => config.method = value.parse().map_err(|e: Error| error(e.to_string()))?,
                "quads" => {
                    let count = value.parse().map_err(|_| error(format!("'{value}' is not a quad count")))?;
                    config.target = Target::QuadCount(count);
                }
                "edge_length" => config.target = Target::EdgeLength(number_of("length")?),
                "alignment" => config.alignment = number_of("number")?,
                "singularity_reach" => config.singularity_reach = number_of("number")?,
                "singularity_tolerance" => config.singularity_tolerance = number_of("number")?,
                "crease_angle" if value == "none" => config.crease_angle = None,
                "crease_angle" => config.crease_angle = Some(number_of("number of degrees, nor 'none'")?),
                "adaptivity" => config.adaptivity = number_of("number")?,
                "density_contrast" => config.density_contrast = number_of("number")?,
                "seed" => config.seed = value.parse().map_err(|_| error(format!("'{value}' is not a seed")))?,
                other => return Err(error(format!("unknown setting '{other}'"))),
            }
        }
        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_is_valid_and_survives_a_round_trip_through_text() {
        for preset in PRESETS {
            for target in [Target::QuadCount(3000), Target::EdgeLength(0.125)] {
                let config = preset.config(target);
                config.validate().unwrap();
                assert_eq!(RemeshConfig::from_text(&config.to_text()).unwrap(), config);
                assert_eq!(RemeshConfig::preset(preset.name, target).unwrap(), config);
            }
        }
        assert!(RemeshConfig::preset("nope", Target::QuadCount(1)).is_err());
    }

    #[test]
    fn a_saved_configuration_can_be_edited_by_hand() {
        let text = "# my own preset\n\nmethod = instant-meshes\nquads = 500\ncrease_angle = 45\n";
        let config = RemeshConfig::from_text(text).unwrap();
        assert_eq!(config.method, Method::InstantMeshes);
        assert_eq!(config.target, Target::QuadCount(500));
        assert_eq!(config.crease_angle, Some(45.0));
        // What the file does not mention keeps its default.
        assert_eq!(config.alignment, RemeshConfig::default().alignment);
    }

    #[test]
    fn mistakes_in_a_file_are_reported_with_their_line() {
        for (text, line) in [("quads = many", 1), ("alignment = 0.1\nsharpness = 3", 2), ("just words", 1)] {
            match RemeshConfig::from_text(text) {
                Err(Error::Parse { line: reported, .. }) => assert_eq!(reported, line, "{text:?}"),
                other => panic!("{text:?} should not parse: {other:?}"),
            }
        }
        assert!(RemeshConfig::from_text("alignment = -1").is_err());
        assert!(RemeshConfig::from_text("crease_angle = 200").is_err());
        assert!(RemeshConfig::from_text("adaptivity = 1.5").is_err());
        assert!(RemeshConfig::from_text("density_contrast = 0.5").is_err());
    }
}
