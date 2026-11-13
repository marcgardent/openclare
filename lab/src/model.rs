//! The record of one run: one line of `runs.jsonl` (SPEC.md, section 5).
//!
//! A run launched by the app fills every field. A run seeded from a summary
//! or typed by hand fills those that are known: the others are `None`.

use std::collections::BTreeMap;
use std::fmt;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// The version of the format of a line, written in every record.
pub const FORMAT: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Run {
    pub format: u32,
    /// "20261106-221403-forced-whole-lower", unique.
    pub id: String,
    /// The name of the job in the catalogue at the time of the run.
    pub job: String,
    pub source: Source,
    #[serde(default)]
    pub queue: Option<String>,
    pub started: DateTime<FixedOffset>,
    #[serde(default)]
    pub ended: Option<DateTime<FixedOffset>>,
    /// The job alone, the prepare excluded.
    #[serde(default)]
    pub seconds: Option<f64>,
    pub status: Status,
    #[serde(default)]
    pub prepare: Option<Prepare>,
    /// As executed, placeholders filled but `{repo}`.
    pub command: String,
    /// The variables the app set, not the inherited environment.
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, String>,
    /// The reference asked for (a tag, a branch, `pull/<n>`, a hash), or
    /// "local" for a folder given by hand (SPEC.md, §2).
    pub checkout: String,
    /// The working directory, relative to that checkout.
    pub directory: String,
    #[serde(default)]
    pub git: Option<Git>,
    #[serde(default)]
    pub threads: Option<Threads>,
    #[serde(default)]
    pub memory_limit_gb: Option<u32>,
    #[serde(default)]
    pub machine: Option<Machine>,
    #[serde(default)]
    pub load: Option<Load>,
    pub calm: Calm,
    /// Editable afterwards.
    #[serde(default)]
    pub comment: String,
    /// Editable afterwards.
    #[serde(default)]
    pub papers: Vec<PaperRef>,
    /// The log, outside the repository.
    #[serde(default)]
    pub log: Option<FileRef>,
    #[serde(default)]
    pub outputs: Vec<FileRef>,
    /// The last change of the comment or the papers.
    #[serde(default)]
    pub edited: Option<DateTime<FixedOffset>>,
    /// The batch and the cell this job is (SPEC.md, §6b); None for a `[[job]]`.
    /// The three fields below are left out of the line when empty, so that the
    /// records written before them read and write back unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell: Option<Cell>,
    /// The limit the job ran under; None: no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<f64>,
    /// The versions of the tools in its checkout: "blender" -> "5.3.0",
    /// "rustc", "python", "lean"; a tool not found is left out.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub tools: BTreeMap<String, String>,
}

/// One cell of one launch of a batch (SPEC.md, §6b).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    /// "20261113-221403-pole-pairs-115-remesh": the launch, shared by its jobs.
    pub launch: String,
    /// Axis -> value: {"budget": "3000", "model": "ogre", "ref": "12a6675"}.
    pub values: BTreeMap<String, String>,
    /// The jobs of the batch that ran side by side.
    pub pool: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Source {
    App,
    /// From a summary tracked in the repository, `from` its path.
    Seeded {
        from: String,
    },
    /// Typed by hand: a run the app did not launch.
    Manual,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Status {
    Ok,
    Failed {
        #[serde(default)]
        code: Option<i32>,
        #[serde(default)]
        signal: Option<i32>,
    },
    /// Stopped by Marc.
    Stopped,
    /// The app ended during the run.
    Interrupted,
    /// Killed by lab at its timeout (SPEC.md, §6b); `seconds` is the time until the kill.
    TimedOut,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prepare {
    pub command: String,
    pub seconds: f64,
    pub status: Status,
    #[serde(default)]
    pub skipped_because: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Git {
    /// 40 hexadecimal digits when the app records it; a seeded run may
    /// carry the short hash its summary gives.
    pub commit: String,
    /// Tracked files differ from HEAD, `lab/runs.jsonl` excluded.
    pub dirty: bool,
    /// The reference asked for (SPEC.md, §5); what `git branch` said, for a
    /// local checkout.
    #[serde(default)]
    pub branch: Option<String>,
    /// The commit is reachable from a branch, a tag or a pull request of the
    /// remote, as the mirror knew when the run started (§10b). A record from
    /// before M7 says nothing: false.
    #[serde(default)]
    pub pushed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Threads {
    pub setting: u32,
    /// What the job uses (SPEC.md, section 7.1).
    pub effective: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    pub hostname: String,
    pub cpu: String,
    pub cores: u32,
    pub threads: u32,
    pub memory_bytes: u64,
    #[serde(default)]
    pub gpu: Option<Gpu>,
    #[serde(default)]
    pub kernel: String,
    /// From the settings, e.g. "i7-7700K 4c/8t 2017, 62 GB".
    #[serde(default)]
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gpu {
    pub name: String,
    pub memory_bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    /// Load averages over 1, 5 and 15 minutes.
    pub load: [f64; 3],
    /// Of the whole machine: 100 is every thread busy.
    #[serde(default)]
    pub cpu_percent: Option<f64>,
    /// The processes that are not the run's.
    #[serde(default)]
    pub foreign_cpu_percent: Option<f64>,
    #[serde(default)]
    pub memory_used_bytes: Option<u64>,
    #[serde(default)]
    pub vram_used_bytes: Option<u64>,
    #[serde(default)]
    pub gpu_percent: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Load {
    pub at_start: Sample,
    #[serde(default)]
    pub at_end: Option<Sample>,
    #[serde(default)]
    pub during: Option<During>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct During {
    pub samples: u32,
    pub foreign_cpu_mean: f64,
    pub foreign_cpu_peak: f64,
    pub load1_max: f64,
    pub memory_used_peak: u64,
    #[serde(default)]
    pub vram_used_peak: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Calm {
    /// The machine was calm over the window before the start.
    pub before: bool,
    /// The foreign load stayed under the thresholds during the run.
    pub during: bool,
    /// Started although the machine was not calm.
    pub forced: bool,
    /// `before && during && !forced`: only then is the time reported.
    pub timing_valid: bool,
    /// Why not, in words.
    #[serde(default)]
    pub reason: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Paper {
    /// Closing edge loops (docs/research/closing-loops).
    A,
    /// Turns of a T-mesh (docs/research/turns-of-a-t-mesh).
    B,
}

impl fmt::Display for Paper {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Paper::A => "A",
            Paper::B => "B",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaperRef {
    pub paper: Paper,
    /// "v0.7.0", "2026-11-06", "tab:forced".
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileRef {
    /// Relative to the checkout of the run, or to the data folder for a log.
    pub path: String,
    pub bytes: u64,
    /// For files up to 1 GB.
    #[serde(default)]
    pub sha256: Option<String>,
}

impl Run {
    pub fn short_commit(&self) -> Option<String> {
        self.git.as_ref().map(|git| {
            let short: String = git.commit.chars().take(7).collect();
            if git.dirty { format!("{short}-dirty") } else { short }
        })
    }

    pub fn papers_text(&self) -> String {
        let papers: Vec<String> = self.papers.iter().map(|paper| format!("{} {}", paper.paper, paper.label)).collect();
        papers.join(", ")
    }
}

impl Status {
    pub fn text(&self) -> String {
        match self {
            Status::Ok => "ok".to_string(),
            Status::Failed { code: Some(code), .. } => format!("failed ({code})"),
            Status::Failed { signal: Some(signal), .. } => format!("failed (signal {signal})"),
            Status::Failed { .. } => "failed".to_string(),
            Status::Stopped => "stopped".to_string(),
            Status::Interrupted => "interrupted".to_string(),
            Status::TimedOut => "timed out".to_string(),
        }
    }
}

/// A duration as the screens show it: "3.6 s", "24 s", "2 min 04",
/// "2 h 38 min 51".
pub fn duration_text(seconds: f64) -> String {
    if seconds < 10.0 {
        return format!("{seconds:.1} s");
    }
    let whole = seconds.round() as u64;
    if whole < 60 {
        return format!("{whole} s");
    }
    let (hours, minutes, rest) = (whole / 3600, whole / 60 % 60, whole % 60);
    if hours == 0 { format!("{minutes} min {rest:02}") } else { format!("{hours} h {minutes:02} min {rest:02}") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_read_as_the_screens_show_them() {
        assert_eq!(duration_text(0.6), "0.6 s");
        assert_eq!(duration_text(24.1), "24 s");
        assert_eq!(duration_text(124.0), "2 min 04");
        assert_eq!(duration_text(9503.5), "2 h 38 min 24");
    }

    #[test]
    fn a_status_reads_back_as_written() {
        let failed = Status::Failed { code: Some(1), signal: None };
        let text = serde_json::to_string(&failed).unwrap();
        assert_eq!(text, r#"{"kind":"failed","code":1,"signal":null}"#);
        assert_eq!(serde_json::from_str::<Status>(r#"{"kind":"failed","code":1}"#).unwrap(), failed);
        assert_eq!(serde_json::from_str::<Status>(r#"{"kind":"ok"}"#).unwrap(), Status::Ok);
    }
}
