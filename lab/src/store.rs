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

/// The live file rewritten with `run` in place of the line of the same id,
/// the five previous versions kept as runs.jsonl.1 to .5 (SPEC.md, section 4).
/// A run of another machine edited here is added: the edit reaches the union
/// of every machine as the latest (§4b).
pub fn replace(path: &Path, run: &Run) -> Result<(), String> {
    let mut history = History::read(path)?;
    match history.runs.iter_mut().find(|known| known.id == run.id) {
        Some(known) => *known = run.clone(),
        None => history.runs.push(run.clone()),
    }
    write_all(path, &history.runs)
}

/// All the runs, one per line, sorted, the previous versions kept.
pub fn write_all(path: &Path, runs: &[Run]) -> Result<(), String> {
    let mut text = String::new();
    for run in runs {
        text.push_str(&serde_json::to_string(run).map_err(|error| error.to_string())?);
        text.push('\n');
    }
    if path.exists() {
        let numbered = |count: u32| {
            let mut name = path.as_os_str().to_owned();
            name.push(format!(".{count}"));
            std::path::PathBuf::from(name)
        };
        for count in (1..5).rev() {
            let _ = std::fs::rename(numbered(count), numbered(count + 1));
        }
        std::fs::copy(path, numbered(1)).map_err(|error| format!("{}: {error}", path.display()))?;
    }
    write_atomically(path, text.as_bytes())
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
    fn an_edit_rewrites_its_line_and_keeps_the_previous_version() {
        let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join("test-data").join("store");
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("runs.jsonl");
        std::fs::write(&path, FIXTURE).unwrap();
        let mut run = History::parse(FIXTURE).runs[1].clone();
        run.comment = "changed".to_string();
        replace(&path, &run).unwrap();
        let again = History::read(&path).unwrap();
        assert_eq!(again.runs.len(), 3);
        assert_eq!(again.runs[1].comment, "changed");
        assert_eq!(std::fs::read_to_string(folder.join("runs.jsonl.1")).unwrap(), FIXTURE);
        // A run of another machine, edited here, is added.
        run.id = "elsewhere".to_string();
        replace(&path, &run).unwrap();
        assert_eq!(History::read(&path).unwrap().runs.len(), 4);
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
