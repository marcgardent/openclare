//! One configuration for a whole remeshing run, and the presets that ship
//! with the project.
//!
//! A preset is nothing more than a named configuration: every field stays
//! open to adjustment, and a configuration can be written to a small text
//! file and read back, which is all "saving a preset" takes.

use std::fmt::Write;

use crate::error::{Error, Result};
use crate::pipeline::Target;
use crate::quantization::Selector;
use crate::remesher::{Guides, Method};
use crate::symmetry::{self, Axis};

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
    /// Edges of the input sharper than this dihedral angle, in degrees, keep
    /// the shape of the work mesh crisp, and the quads follow them where
    /// they can, without it being imposed as it is for [`Self::crease_angle`]:
    /// one the map cannot keep is let go. `None` for none.
    pub edge_angle: Option<f64>,
    /// How much the size of the quads follows the shape: 0 keeps them all
    /// the same size; towards 1 they get smaller where the surface bends
    /// more, larger where it is flat, for the same number in total.
    pub adaptivity: f64,
    /// Fewest quads that go round a thin part of the shape, a finger or
    /// the tip of an ear: where the quads asked for are too large for that,
    /// they are made smaller there and larger elsewhere, so that the part
    /// keeps its volume instead of coming out flat. 0 does not look at
    /// thickness.
    pub quads_around: f64,
    /// Most quads made when the size of the quads is asked for and not
    /// their number (see [`Target::SizeRange`]): where the shape calls for
    /// more, every quad is made larger, by as much as it takes.
    pub most_quads: usize,
    /// How far the vertices are moved off the surface, at the very end, for
    /// the mesh to keep the volume of the shape: with every vertex on the
    /// surface the faces run under it wherever it bulges. 0 leaves the
    /// vertices on the surface; 1 puts as much of the mesh above the surface
    /// as below.
    pub volume_fit: f64,
    /// With a painted density: how many times more quads per unit of area
    /// the places painted at 1 get than those left at 0. At 4, their edges
    /// are half as long.
    pub density_contrast: f64,
    /// On a model made of several separate pieces: a piece whose fair share
    /// of the quads is below this is left out, instead of getting the few
    /// quads no shape can do without. 0 keeps every piece.
    pub skip_pieces_under: f64,
    /// Lines of quads that go round a part are closed into rings where the
    /// whole lengths would wind them into helices (DT-062).
    pub close_loops: bool,
    /// How hard the loops are closed: by the engine alone, or, slowly, by
    /// an exact solver as well (see [`Quality`]).
    pub quality: Quality,
    /// In the slow quality mode, the time each solve may take, in seconds:
    /// a bound for safety, which depends on the load of the machine.
    pub slow_time_limit: f64,
    /// In the slow quality mode, the work each solve may do, in millions of
    /// simplex iterations times arcs: the bound that holds first in
    /// general, which does not depend on the load, so that a slow run gives
    /// the same mesh each time. 0 for the time limit alone.
    pub slow_work_limit: f64,
    /// The solver of the slow quality mode, injected by the caller: the
    /// core has none of its own. Not a setting written to text.
    pub ring_selector: Selector,
    pub seed: u64,
    /// The plane through the origin of the model across this axis is a
    /// plane of symmetry: the quad mesh is made the same on both sides of
    /// it (see [`crate::symmetry`]).
    pub symmetry: Option<Axis>,
}

/// Vertices of the input closer to the plane of symmetry than this share of
/// its diagonal are taken to be on it.
const SYMMETRY_SNAP: f64 = 1e-6;
/// A model whose mirror image is farther from its surface than this share
/// of its diagonal, somewhere, is not taken to be symmetric: its negative
/// side would be replaced by something else. Models of the bench meant to
/// be symmetric measure up to 1.5 % (the ogre, a sculpt); a wrong axis, or
/// an origin off the plane, 8 % and more.
const SYMMETRY_TOLERANCE: f64 = 0.02;
/// A vertex of the half put on the plane from farther than this share of
/// the edge of the quads was not held there by the line of edges drawn on
/// it: the line was let go, and the half is cut across its quads. Where the
/// line is kept, the relaxation leaves its vertices up to half an edge off
/// the plane (the ogre, blub); where it is let go, from 0.55 edges on (the
/// venturi across y).
const OFF_THE_PLANE: f64 = 0.6;

/// Default of [`RemeshConfig::slow_time_limit`], in seconds: a bound for
/// safety, well above what the work limit takes.
pub const SLOW_TIME_LIMIT: f64 = 300.0;
/// Default of [`RemeshConfig::slow_work_limit`]. On an Intel Core i7-7700K
/// (2017) under a load of 5 to 9, HiGHS does 0.6 to 6 million iterations
/// times arcs a second, and stops some way past the limit: a solve takes
/// from a fraction of a second to about 100 seconds, and the whole mode
/// less processor time than 20 seconds per solve took.
pub const SLOW_WORK_LIMIT: f64 = 15.0;

/// How hard the loops round the parts are closed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Quality {
    /// The engine's own search.
    #[default]
    Standard,
    /// The engine's search, then rounds of rings chosen by an exact solver
    /// ([`RemeshConfig::ring_selector`]), whose lengths are kept where they
    /// make fewer spirals. Seconds to minutes more per piece; without a
    /// solver injected, the same as the standard quality.
    Slow,
}

impl Quality {
    pub fn name(self) -> &'static str {
        match self {
            Quality::Standard => "standard",
            Quality::Slow => "slow",
        }
    }
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for Quality {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        [Quality::Standard, Quality::Slow]
            .into_iter()
            .find(|quality| quality.name() == name)
            .ok_or_else(|| Error::InvalidParams(format!("unknown quality '{name}' (known: standard, slow)")))
    }
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
        summary: "mechanical parts: edges sharper than 30 degrees are kept, exactly, wherever the mesh can",
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
            edge_angle: Some(30.0),
            adaptivity: 0.5,
            quads_around: 6.0,
            most_quads: 200_000,
            volume_fit: 1.0,
            density_contrast: 4.0,
            skip_pieces_under: 0.0,
            close_loops: true,
            quality: Quality::Standard,
            slow_time_limit: SLOW_TIME_LIMIT,
            slow_work_limit: SLOW_WORK_LIMIT,
            ring_selector: Selector::default(),
            seed: 1,
            symmetry: None,
        }
    }

    pub fn for_animation(target: Target) -> Self {
        Self { singularity_reach: 3.0, ..Self::for_organic(target) }
    }

    pub fn for_hard_surface(target: Target) -> Self {
        // Hard edges lead; curvature only settles what they leave open. And
        // a part is cut to one size: its edges, sharp as they are, call for
        // no smaller quads. Those edges are lines to follow wherever the map
        // can, and hard edges of the mesh where it does; not creases that
        // must be kept, which cost the whole piece where one cannot be: an
        // assembled model has bevels and facets closer than its quads.
        Self {
            alignment: 0.02,
            crease_angle: None,
            edge_angle: Some(30.0),
            adaptivity: 0.0,
            quads_around: 0.0,
            ..Self::for_organic(target)
        }
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
        self.remesh_recorded(input, guides, &mut crate::recorder::Silent)
    }

    /// The same, showing `recorder` the input, what each stage makes of it
    /// and the result.
    pub fn remesh_recorded(
        &self,
        input: &crate::TriMesh,
        guides: &Guides,
        recorder: &mut dyn crate::recorder::Recorder,
    ) -> Result<(crate::PolyMesh, crate::pipeline::Report)> {
        use crate::recorder::Shape;
        if let Some(axis) = self.symmetry {
            return self.remesh_symmetric(input, guides, axis, recorder);
        }
        recorder.record("input/surface", Shape::Surface { mesh: input, values: guides.density });
        let (mesh, report) = crate::remesher::create(self.method).remesh_recorded(input, guides, self, recorder)?;
        recorder.record("result/quads", Shape::Polygons(&mesh));
        Ok((mesh, report))
    }

    /// `input` remeshed with the plane across `axis` drawn into it as a line
    /// of edges, the half on its positive side kept and mirrored. What the
    /// guides say of the input is said of it with the plane drawn in: the
    /// weights painted on the vertices made there are shared out from the
    /// two ends of the edge each cuts, and each triangle cut keeps its
    /// group.
    fn remesh_symmetric(
        &self,
        input: &crate::TriMesh,
        guides: &Guides,
        axis: Axis,
        recorder: &mut dyn crate::recorder::Recorder,
    ) -> Result<(crate::PolyMesh, crate::pipeline::Report)> {
        symmetry::check(input, axis, SYMMETRY_TOLERANCE)?;
        let (low, high) = input
            .positions
            .iter()
            .fold((crate::math::Vec3::INFINITY, crate::math::Vec3::NEG_INFINITY), |(lo, hi), &p| {
                (lo.min(p), hi.max(p))
            });
        let snap = SYMMETRY_SNAP * (high - low).length();
        let split = symmetry::split(input, axis, snap);
        let density = guides.density.map(|density| split.weights(density));
        let groups = guides.groups.map(|groups| split.per_triangle(groups));
        let mut lines = guides.lines.to_vec();
        lines.extend(split.lines.iter().copied());
        let drawn = Guides {
            density: density.as_deref(),
            groups: groups.as_deref(),
            lines: &lines,
            held: &split.lines,
            ..*guides
        };
        let config = Self { symmetry: None, ..self.clone() };
        let (mesh, mut report) = config.remesh_recorded(&split.mesh, &drawn, recorder)?;
        let (mesh, carried) = symmetry::symmetric(&mesh, axis, snap);
        // What the report says of edges, by their vertices, said of the
        // result: those of the half kept, and their images.
        report.hard_edges = carried.edges(&report.hard_edges);
        report.guide_edges = carried.edges(&report.guide_edges);
        report.sharp_curve_edges = carried.edges(&report.sharp_curve_edges);
        report.curve_edges = carried.carry(report.curve_edges.iter().copied());
        report.creases = carried.carry(report.creases.iter().copied());
        if !report.face_groups.is_empty() {
            report.face_groups = carried.faces.iter().map(|&f| report.face_groups[f as usize]).collect();
        }
        let edge = (high - low).length() / (mesh.faces.len().max(1) as f64).sqrt();
        report.off_the_plane = carried.moved.iter().filter(|&&d| d > OFF_THE_PLANE * edge).count();
        report.stats = mesh.stats();
        Ok((mesh, report))
    }

    /// The hard edges of `input` the configured chain would follow, and
    /// those it would leave out, found without remeshing (see
    /// [`crate::Remesher::hard_edges`]). To find them again under other
    /// settings, a [`crate::HardEdgeFinder`] keeps what depends on the shape
    /// alone.
    pub fn hard_edges(&self, input: &crate::TriMesh, guides: &Guides) -> Result<crate::preprocess::HardEdges> {
        crate::HardEdgeFinder::new(input.clone()).find(guides, self)
    }

    pub fn validate(&self) -> Result<()> {
        let non_negative = [
            ("skip_pieces_under", self.skip_pieces_under),
            ("slow_work_limit", self.slow_work_limit),
            ("quads_around", self.quads_around),
            ("alignment", self.alignment),
            ("singularity_reach", self.singularity_reach),
            ("singularity_tolerance", self.singularity_tolerance),
        ];
        if !(self.slow_time_limit.is_finite() && self.slow_time_limit > 0.0) {
            return Err(Error::InvalidParams(format!(
                "slow_time_limit must be a positive number of seconds, not {}",
                self.slow_time_limit
            )));
        }
        for (name, value) in non_negative {
            if !(value.is_finite() && value >= 0.0) {
                return Err(Error::InvalidParams(format!("{name} must be a non-negative number, not {value}")));
            }
        }
        if !(0.0..=1.0).contains(&self.volume_fit) {
            return Err(Error::InvalidParams(format!("volume_fit must be between 0 and 1, not {}", self.volume_fit)));
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
            Target::SizeRange { largest, smallest } => {
                writeln!(text, "largest_quad = {largest}\nsmallest_quad = {smallest}")
            }
        }
        .unwrap();
        writeln!(text, "alignment = {}", self.alignment).unwrap();
        writeln!(text, "singularity_reach = {}", self.singularity_reach).unwrap();
        writeln!(text, "singularity_tolerance = {}", self.singularity_tolerance).unwrap();
        match self.edge_angle {
            Some(angle) => writeln!(text, "edge_angle = {angle}"),
            None => writeln!(text, "edge_angle = none"),
        }
        .unwrap();
        match self.crease_angle {
            Some(angle) => writeln!(text, "crease_angle = {angle}"),
            None => writeln!(text, "crease_angle = none"),
        }
        .unwrap();
        writeln!(text, "adaptivity = {}", self.adaptivity).unwrap();
        writeln!(text, "quads_around = {}", self.quads_around).unwrap();
        writeln!(text, "most_quads = {}", self.most_quads).unwrap();
        writeln!(text, "volume_fit = {}", self.volume_fit).unwrap();
        writeln!(text, "density_contrast = {}", self.density_contrast).unwrap();
        writeln!(text, "skip_pieces_under = {}", self.skip_pieces_under).unwrap();
        writeln!(text, "close_loops = {}", self.close_loops).unwrap();
        writeln!(text, "quality = {}", self.quality).unwrap();
        writeln!(text, "slow_time_limit = {}", self.slow_time_limit).unwrap();
        writeln!(text, "slow_work_limit = {}", self.slow_work_limit).unwrap();
        writeln!(text, "seed = {}", self.seed).unwrap();
        writeln!(text, "symmetry = {}", self.symmetry.map_or("none", Axis::name)).unwrap();
        text
    }

    /// Reads what [`Self::to_text`] writes. Fields left out keep their
    /// default; blank lines and lines starting with `#` are ignored.
    pub fn from_text(text: &str) -> Result<Self> {
        let mut config = Self::default();
        config.apply_text(text)?;
        Ok(config)
    }

    /// Sets the fields that `text` mentions, in the format of
    /// [`Self::to_text`], and leaves the others as they are: settings on top
    /// of a preset. Nothing is changed if a line is wrong or if the result
    /// is not a valid configuration.
    pub fn apply_text(&mut self, text: &str) -> Result<()> {
        let mut config = self.clone();
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
                // The largest first; the smallest is an eighth of it until
                // it is said.
                "largest_quad" => {
                    let largest = number_of("length")?;
                    config.target = Target::SizeRange { largest, smallest: largest / 8.0 };
                }
                "smallest_quad" => match config.target {
                    Target::SizeRange { largest, .. } => {
                        config.target = Target::SizeRange { largest, smallest: number_of("length")? };
                    }
                    _ => return Err(error("smallest_quad comes after largest_quad".to_string())),
                },
                "most_quads" => {
                    config.most_quads = value.parse().map_err(|_| error(format!("'{value}' is not a quad count")))?;
                }
                "alignment" => config.alignment = number_of("number")?,
                "singularity_reach" => config.singularity_reach = number_of("number")?,
                "singularity_tolerance" => config.singularity_tolerance = number_of("number")?,
                "edge_angle" if value == "none" => config.edge_angle = None,
                "edge_angle" => config.edge_angle = Some(number_of("number of degrees, nor 'none'")?),
                "crease_angle" if value == "none" => config.crease_angle = None,
                "crease_angle" => config.crease_angle = Some(number_of("number of degrees, nor 'none'")?),
                "adaptivity" => config.adaptivity = number_of("number")?,
                "quads_around" => config.quads_around = number_of("number")?,
                // A setting there was until sizes could be asked for: read,
                // for the files that have it, and no longer used.
                "preserve_volumes" => {}
                "volume_fit" => config.volume_fit = number_of("number")?,
                "density_contrast" => config.density_contrast = number_of("number")?,
                "skip_pieces_under" => config.skip_pieces_under = number_of("number")?,
                "close_loops" => {
                    config.close_loops = value.parse().map_err(|_| error(format!("'{value}' is not true or false")))?
                }
                "quality" => config.quality = value.parse().map_err(|e: Error| error(e.to_string()))?,
                "slow_time_limit" => config.slow_time_limit = number_of("number of seconds")?,
                "slow_work_limit" => config.slow_work_limit = number_of("number")?,
                "seed" => config.seed = value.parse().map_err(|_| error(format!("'{value}' is not a seed")))?,
                "symmetry" if value == "none" => config.symmetry = None,
                "symmetry" => config.symmetry = Some(value.parse().map_err(|e: Error| error(e.to_string()))?),
                other => return Err(error(format!("unknown setting '{other}'"))),
            }
        }
        config.validate()?;
        *self = config;
        Ok(())
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
        let slow = RemeshConfig::from_text("quality = slow\nslow_time_limit = 20\nslow_work_limit = 10\n").unwrap();
        assert_eq!((slow.quality, slow.slow_time_limit, slow.slow_work_limit), (Quality::Slow, 20.0, 10.0));
    }

    #[test]
    fn settings_can_be_laid_over_a_preset() {
        let mut config = RemeshConfig::for_hard_surface(Target::QuadCount(1000));
        config.apply_text("quads = 4000\nvolume_fit = 0.5\n").unwrap();
        assert_eq!(config.target, Target::QuadCount(4000));
        assert_eq!(config.volume_fit, 0.5);
        // What the text does not mention is still the preset's.
        assert_eq!((config.crease_angle, config.edge_angle), (None, Some(30.0)));
        // A text that is wrong changes nothing at all.
        let before = config.clone();
        assert!(config.apply_text("quads = 10\nadaptivity = 7\n").is_err());
        assert_eq!(config, before);
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
        assert!(RemeshConfig::from_text("quality = fast").is_err());
        assert!(RemeshConfig::from_text("slow_time_limit = 0").is_err());
        assert!(RemeshConfig::from_text("slow_work_limit = -1").is_err());
    }
}
