//! Experiment options of the closing of loops (DT-089), for the research
//! papers under `docs/research`: each experiment of theirs runs on the
//! engine as it is, with some of these set. Switches turn off one stage of
//! the closing at a time; files let the system it works on be read outside
//! the engine, and lengths worked out there be given back to it.
//!
//! They are read from the environment, once per quantization request, by
//! [`Experiment::from_env`]; ALGORITHM.md (M6, "Experiment options") lists
//! them. None is set by default, and then the engine is what it is without
//! them, to the byte.
//!
//! Files are named after the T-mesh they belong to, since the pieces of a
//! model are quantized in parallel: `call-<arcs>-<patches>-<length>`, its
//! number of arcs, of rectangles, and the total length of its arcs in
//! thousandths ([`call_name`]).

use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::tmesh::TMesh;

/// How many ways of closing the engine tries ([`super::CLOSE_WAYS`]).
const WAYS: usize = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct Experiment {
    /// `OPENCLARE_EXPORT_TURNS=<directory>`: the system each T-mesh's
    /// closing works on, written there as JSON.
    pub export: Option<PathBuf>,
    /// `OPENCLARE_LENGTHS=<directory>`: whole lengths read from there and
    /// taken as they are, for a T-mesh or for one way of closing.
    pub lengths: Option<PathBuf>,
    /// `OPENCLARE_AIMED=<directory>`: for one way of closing, the lengths
    /// the search aims at and the turns closed, read from there instead of
    /// the projection's.
    pub aimed: Option<PathBuf>,
    /// `OPENCLARE_JITTER_SEED=<n>`: another draw of the slight preference
    /// given to each arc. 0, the default, is the engine's own.
    pub jitter_seed: u64,
    /// `OPENCLARE_EXTRA_DRAWS=<n>`: the number of other draws of the per-arc
    /// preference (the next seeds) whose lengths join the candidates where
    /// the engine's still spiral. `None`, the default, is the engine's
    /// number; 0 is none, the engine before them.
    pub extra_draws: Option<usize>,
    /// `OPENCLARE_CLOSE_WAYS=<i,j,...>`: the ways of closing tried, by their
    /// place in the list of ways (0 to 7). `None`: all of them.
    pub ways: Option<Vec<usize>>,
    /// Off with `OPENCLARE_NO_CUT_ROUNDS`: the rounds of closing by cuts
    /// after each way.
    pub cut_rounds: bool,
    /// Off with `OPENCLARE_NO_AIMED_REPAIR`: the walks aimed at each turn
    /// held and still shifted after the repair.
    pub aimed_repair: bool,
    /// On with `OPENCLARE_CORRIDORS`: the turns kept are kept in their
    /// corridor, by the repair and by rounds of constraint generation
    /// (DT-138). Off by default: on the 12-model batch it did not lower the
    /// share of spiral edges.
    pub corridors: bool,
    /// Off with `OPENCLARE_NO_ACTIVE_SET`: arcs the least change would take
    /// below their floor are held there and the change looked for again;
    /// without it, they are cut off at the floor afterwards.
    pub active_set: bool,
    /// `OPENCLARE_ACTIVE_SET_FLOOR=<share>`: that floor, as a share of the
    /// real length of each arc. 0, the default, is nothing.
    pub active_set_floor: f64,
    /// Off with `OPENCLARE_NO_DEVIATION_CAP`: the most the lengths closing
    /// loops may be further from the real ones than the plain search's.
    pub deviation_cap: bool,
    /// Off with `OPENCLARE_NO_DEVIATION_FLOOR`: the floor of that cap, the
    /// fixed amount further they may always be.
    pub deviation_floor: bool,
    /// Off with `OPENCLARE_NO_FOLD_GUARD`: lengths closing loops may not take
    /// the vertices further off the folds of the surface than the plain
    /// search left them.
    pub fold_guard: bool,
}

impl Default for Experiment {
    fn default() -> Self {
        Self {
            export: None,
            lengths: None,
            aimed: None,
            jitter_seed: 0,
            extra_draws: None,
            ways: None,
            cut_rounds: true,
            aimed_repair: true,
            corridors: false,
            active_set: true,
            active_set_floor: 0.0,
            deviation_cap: true,
            deviation_floor: true,
            fold_guard: true,
        }
    }
}

impl Experiment {
    /// The options set in the environment; a value that cannot be read is
    /// an error rather than an experiment quietly run without it.
    pub fn from_env() -> Result<Self> {
        Self::from_lookup(|name| std::env::var_os(name).map(|value| value.to_string_lossy().into_owned()))
    }

    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self> {
        let invalid =
            |name: &str, value: &str, what: &str| Error::InvalidParams(format!("{name}={value}: expected {what}"));
        let directory = |name: &str| lookup(name).filter(|value| !value.is_empty()).map(PathBuf::from);
        let off = |name: &str| lookup(name).is_some();
        let mut out = Self {
            export: directory("OPENCLARE_EXPORT_TURNS"),
            lengths: directory("OPENCLARE_LENGTHS"),
            aimed: directory("OPENCLARE_AIMED"),
            cut_rounds: !off("OPENCLARE_NO_CUT_ROUNDS"),
            aimed_repair: !off("OPENCLARE_NO_AIMED_REPAIR"),
            corridors: lookup("OPENCLARE_CORRIDORS").is_some(),
            active_set: !off("OPENCLARE_NO_ACTIVE_SET"),
            deviation_cap: !off("OPENCLARE_NO_DEVIATION_CAP"),
            deviation_floor: !off("OPENCLARE_NO_DEVIATION_FLOOR"),
            fold_guard: !off("OPENCLARE_NO_FOLD_GUARD"),
            ..Self::default()
        };
        if let Some(value) = lookup("OPENCLARE_JITTER_SEED") {
            out.jitter_seed =
                value.trim().parse().map_err(|_| invalid("OPENCLARE_JITTER_SEED", &value, "a whole number"))?;
        }
        if let Some(value) = lookup("OPENCLARE_EXTRA_DRAWS") {
            out.extra_draws =
                Some(value.trim().parse().map_err(|_| invalid("OPENCLARE_EXTRA_DRAWS", &value, "a whole number"))?);
        }
        if let Some(value) = lookup("OPENCLARE_ACTIVE_SET_FLOOR") {
            out.active_set_floor = value
                .trim()
                .parse()
                .ok()
                .filter(|share: &f64| (0.0..=1.0).contains(share))
                .ok_or_else(|| invalid("OPENCLARE_ACTIVE_SET_FLOOR", &value, "a share from 0 to 1"))?;
        }
        if let Some(value) = lookup("OPENCLARE_CLOSE_WAYS") {
            let ways: Option<Vec<usize>> = value
                .split(|c: char| c == ',' || c.is_whitespace())
                .filter(|part| !part.is_empty())
                .map(|part| part.parse().ok().filter(|&way| way < WAYS))
                .collect();
            out.ways = Some(ways.ok_or_else(|| invalid("OPENCLARE_CLOSE_WAYS", &value, "ways from 0 to 7"))?);
        }
        Ok(out)
    }

    /// Whether the way of closing at `index` in the list of ways is tried.
    pub(super) fn tries(&self, index: usize) -> bool {
        self.ways.as_ref().is_none_or(|ways| ways.contains(&index))
    }

    /// Whole lengths for the whole of `tmesh` (`call-<name>.txt`): a first
    /// line with the scale (empty for the plain search's), then a length per
    /// arc. `None` if there is no such file or it has not a length per arc.
    pub(super) fn call_lengths(&self, tmesh: &TMesh) -> Option<(Option<f64>, Vec<i64>)> {
        let text = read(self.lengths.as_deref()?, &format!("{}.txt", call_name(tmesh)))?;
        let mut lines = text.lines();
        let scale = lines.next().and_then(|line| line.trim().parse().ok());
        let x = whole(lines.flat_map(|line| line.split_whitespace()));
        (x.len() == tmesh.arcs.len()).then_some((scale, x))
    }

    /// Whole lengths for the way of closing `way` of `tmesh`
    /// (`call-<name>/way-<way>-integer.txt`), a length per arc, in place of
    /// those of its search and repair.
    pub(super) fn way_lengths(&self, tmesh: &TMesh, way: usize) -> Option<Vec<i64>> {
        let name = format!("{}/way-{way}-integer.txt", call_name(tmesh));
        let x = whole(read(self.lengths.as_deref()?, &name)?.split_whitespace());
        (x.len() == tmesh.arcs.len()).then_some(x)
    }

    /// For the way of closing `way` of `tmesh` (`call-<name>/aimed-<way>.txt`):
    /// the lengths the search aims at, at the scale of the real ones (a first
    /// line with one per arc, or empty for the projection's), and the turns
    /// closed, by their place among those the way holds (a second line).
    pub(super) fn aimed(&self, tmesh: &TMesh, way: usize) -> Option<(Option<Vec<f64>>, Vec<usize>)> {
        let text = read(self.aimed.as_deref()?, &format!("{}/aimed-{way}.txt", call_name(tmesh)))?;
        let mut lines = text.lines();
        let values: Vec<f64> =
            lines.next().unwrap_or("").split_whitespace().filter_map(|value| value.parse().ok()).collect();
        let closed: Vec<usize> =
            lines.next().unwrap_or("").split_whitespace().filter_map(|value| value.parse().ok()).collect();
        if values.len() == tmesh.arcs.len() {
            Some((Some(values), closed))
        } else if values.is_empty() {
            Some((None, closed))
        } else {
            None
        }
    }
}

/// (Experiment.) Appends `line` to the file `OPENCLARE_SLOW_LOG` names,
/// after the number of the process: what the slow quality mode and its
/// solver did, for runs whose output is not kept. Nothing when it is not
/// set.
pub fn slow_log(line: &str) {
    use std::io::Write;
    let Some(path) = std::env::var_os("OPENCLARE_SLOW_LOG").filter(|path| !path.is_empty()) else { return };
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        // One write per line, so that pieces quantized in parallel do not
        // mix their lines.
        let _ = file.write_all(format!("{} {line}\n", std::process::id()).as_bytes());
    }
}

/// The name of the files of `tmesh`: `call-<arcs>-<patches>-<length>`.
pub(super) fn call_name(tmesh: &TMesh) -> String {
    let length: f64 = tmesh.arcs.iter().map(|arc| arc.length).sum();
    format!("call-{}-{}-{}", tmesh.arcs.len(), tmesh.patches.len(), (length * 1000.0).round() as i64)
}

fn read(directory: &Path, name: &str) -> Option<String> {
    std::fs::read_to_string(directory.join(name)).ok()
}

/// Numbers read as whole lengths, rounded.
fn whole<'a>(values: impl Iterator<Item = &'a str>) -> Vec<i64> {
    values.filter_map(|value| value.parse::<f64>().ok()).map(|value| value.round() as i64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(pairs: &[(&str, &str)]) -> Result<Experiment> {
        let pairs: Vec<(String, String)> = pairs.iter().map(|&(k, v)| (k.to_string(), v.to_string())).collect();
        Experiment::from_lookup(|name| pairs.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone()))
    }

    #[test]
    fn nothing_set_is_the_default() {
        assert_eq!(read(&[]).unwrap(), Experiment::default());
    }

    #[test]
    fn options_are_read() {
        let experiment = read(&[
            ("OPENCLARE_CLOSE_WAYS", "0, 2,4"),
            ("OPENCLARE_NO_CUT_ROUNDS", "1"),
            ("OPENCLARE_ACTIVE_SET_FLOOR", "0.5"),
            ("OPENCLARE_JITTER_SEED", "3"),
            ("OPENCLARE_EXTRA_DRAWS", "2"),
            ("OPENCLARE_EXPORT_TURNS", "out"),
            ("OPENCLARE_CORRIDORS", "1"),
        ])
        .unwrap();
        assert!(experiment.corridors);
        assert_eq!(experiment.ways, Some(vec![0, 2, 4]));
        assert!(experiment.tries(2) && !experiment.tries(1));
        assert!(!experiment.cut_rounds && experiment.aimed_repair);
        assert_eq!((experiment.active_set_floor, experiment.jitter_seed, experiment.extra_draws), (0.5, 3, Some(2)));
        assert_eq!(experiment.export, Some(PathBuf::from("out")));
        assert_eq!(read(&[("OPENCLARE_CLOSE_WAYS", "")]).unwrap().ways, Some(Vec::new()));
    }

    #[test]
    fn values_that_cannot_be_read_are_refused() {
        assert!(read(&[("OPENCLARE_CLOSE_WAYS", "8")]).is_err());
        assert!(read(&[("OPENCLARE_CLOSE_WAYS", "two")]).is_err());
        assert!(read(&[("OPENCLARE_JITTER_SEED", "-1")]).is_err());
        assert!(read(&[("OPENCLARE_EXTRA_DRAWS", "three")]).is_err());
        assert!(read(&[("OPENCLARE_ACTIVE_SET_FLOOR", "2")]).is_err());
    }
}
