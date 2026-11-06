//! The live database, `runs.jsonl` in the data folder (SPEC.md, section 4):
//! one run per line, loaded whole.

use std::collections::BTreeMap;
use std::path::Path;

use crate::model::{FORMAT, Run};

/// The runs as read, with the lines that could not be read.
#[derive(Clone, Debug, Default)]
pub struct History {
    /// Sorted by start time, one per id.
    pub runs: Vec<Run>,
    pub problems: Vec<String>,
}

impl History {
    /// A missing file is an empty history.
    pub fn read(path: &Path) -> Result<History, String> {
        match std::fs::read_to_string(path) {
            Ok(text) => Ok(History::parse(&text)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(History::default()),
            Err(error) => Err(format!("{}: {error}", path.display())),
        }
    }

    /// One run per id: of two lines with the same id (two branches merged
    /// with `merge=union`), the one edited last, else the later line.
    pub fn parse(text: &str) -> History {
        let mut by_id: BTreeMap<String, Run> = BTreeMap::new();
        let mut problems = Vec::new();
        for (index, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<Run>(line) {
                Ok(run) if run.format > FORMAT => problems.push(format!(
                    "runs.jsonl, line {}: format {} is newer than this lab's",
                    index + 1,
                    run.format
                )),
                Ok(run) => {
                    let keep = by_id.get(&run.id).is_none_or(|known| run.edited >= known.edited);
                    if keep {
                        by_id.insert(run.id.clone(), run);
                    }
                }
                Err(error) => problems.push(format!("runs.jsonl, line {}: {error}", index + 1)),
            }
        }
        let mut runs: Vec<Run> = by_id.into_values().collect();
        runs.sort_by(|a, b| a.started.cmp(&b.started).then_with(|| a.id.cmp(&b.id)));
        History { runs, problems }
    }
}

/// Written beside, then renamed: a reader sees the old file or the new one.
pub fn write_atomically(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let failed = |error: std::io::Error| format!("{}: {error}", path.display());
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder).map_err(failed)?;
    }
    let mut beside = path.as_os_str().to_owned();
    beside.push(".new");
    std::fs::write(&beside, bytes).map_err(failed)?;
    std::fs::rename(&beside, path).map_err(failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../tests/fixtures/runs.jsonl");

    #[test]
    fn the_fixture_reads_whole_and_sorted() {
        let history = History::parse(FIXTURE);
        assert!(history.problems.is_empty(), "{:?}", history.problems);
        let ids: Vec<&str> = history.runs.iter().map(|run| run.id.as_str()).collect();
        assert_eq!(ids, ["seeded-forced", "20261106-153212-quick-bench", "20261106-223912-forced-whole-lower"]);
    }

    #[test]
    fn the_line_edited_last_wins_and_a_bad_line_is_reported() {
        let mut lines: Vec<String> = FIXTURE.lines().map(str::to_string).collect();
        let mut edited: Run = serde_json::from_str(&lines[0]).unwrap();
        edited.comment = "edited on another branch".to_string();
        edited.edited = Some("2026-11-07T09:00:00+01:00".parse().unwrap());
        lines.insert(0, serde_json::to_string(&edited).unwrap());
        lines.push("{not json".to_string());
        let history = History::parse(&lines.join("\n"));
        assert_eq!(history.runs.len(), 3);
        let run = history.runs.iter().find(|run| run.id == edited.id).unwrap();
        assert_eq!(run.comment, "edited on another branch");
        assert_eq!(history.problems.len(), 1);
        assert!(history.problems[0].starts_with(&format!("runs.jsonl, line {}", lines.len())));
    }
}
