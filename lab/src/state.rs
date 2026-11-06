//! Where each job stands (SPEC.md, sections 10, 10b and 13): the tags of the
//! repository, the runs its summaries record, the last run of each job, on a
//! tag or off-tag, and the estimate of a job from its runs.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use chrono::{DateTime, FixedOffset};
use serde_json::Value;

use crate::catalogue::{Catalogue, Job};
use crate::model::{Calm, FORMAT, Git, Load, Machine, Run, Sample, Source, Status};

/// The reason a run read from the repository gives for its calm.
pub const FROM_REPOSITORY: &str = "not measured (from the repository)";

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    /// The commit it points at, 40 hexadecimal digits.
    pub commit: String,
    /// "2026-10-28".
    pub date: String,
}

fn git(home: &Path, arguments: &[&str]) -> Option<String> {
    let output = Command::new("git").arg("-C").arg(home).args(arguments).output().ok()?;
    output.status.success().then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// The tags `v*`, newest first.
pub fn tags(home: &Path) -> Vec<Tag> {
    let Some(text) = git(home, &["tag", "-l", "v*", "--sort=-creatordate"]) else { return Vec::new() };
    text.lines()
        .filter_map(|name| {
            let commit = git(home, &["rev-parse", &format!("{name}^{{commit}}")])?;
            let date = git(home, &["log", "-1", "--format=%ad", "--date=short", &commit]).unwrap_or_default();
            Some(Tag { name: name.to_string(), commit, date })
        })
        .collect()
}

/// The tag a run is on, if the one hash it records is a tag's.
pub fn tag_of<'a>(run: &Run, tags: &'a [Tag]) -> Option<&'a Tag> {
    let commit = &run.git.as_ref()?.commit;
    tags.iter().find(|tag| !commit.is_empty() && tag.commit.starts_with(commit.as_str()))
}

/// "v0.7.0", or "off-tag 45ae56e", or "no hash".
pub fn place(run: &Run, tags: &[Tag]) -> String {
    match (tag_of(run, tags), &run.git) {
        (Some(tag), _) => tag.name.clone(),
        (None, Some(git)) => format!("off-tag {}", git.commit.chars().take(7).collect::<String>()),
        (None, None) => "no hash".to_string(),
    }
}

/// The runs the repository's summaries record, one per job that names one.
pub fn repository_runs(home: &Path, catalogue: &Catalogue) -> (Vec<Run>, Vec<String>) {
    let mut runs = Vec::new();
    let mut problems = Vec::new();
    for job in &catalogue.jobs {
        let Some(summary) = &job.summary else { continue };
        match repository_run(home, job, summary) {
            Ok(Some(run)) => runs.push(run),
            Ok(None) => {}
            Err(error) => problems.push(format!("{}: {error}", job.name)),
        }
    }
    (runs, problems)
}

/// The run a summary records; None if the file is not there.
fn repository_run(home: &Path, job: &Job, summary: &str) -> Result<Option<Run>, String> {
    let (path, key) = match summary.split_once('#') {
        Some((path, key)) => (path, Some(key)),
        None => (summary, None),
    };
    let Ok(text) = std::fs::read_to_string(home.join(path)) else { return Ok(None) };
    let whole: Value = serde_json::from_str(&text).map_err(|error| format!("{path}: {error}"))?;
    let record = match key {
        Some(key) => find_key(&whole, key).ok_or_else(|| format!("{path}: no {key} in it"))?,
        None => &whole,
    };
    // The commit that brought the file: its date, and its hash if the record names none.
    let brought = git(home, &["log", "-1", "--format=%H %aI", "--", path])
        .filter(|line| !line.is_empty())
        .ok_or_else(|| format!("{path}: not in git"))?;
    let (brought_hash, brought_date) = brought.split_once(' ').ok_or("an odd git log line")?;
    let started: DateTime<FixedOffset> =
        DateTime::parse_from_rfc3339(brought_date).map_err(|error| error.to_string())?;
    let recorded = ["engine/commit", "export_record/engine/commit", "commit"]
        .iter()
        .find_map(|pointer| string_at(record, pointer))
        .and_then(|short| git(home, &["rev-parse", "--verify", "--quiet", &format!("{short}^{{commit}}")]));
    let commit = recorded.unwrap_or_else(|| brought_hash.to_string());
    let seconds = ["wall_seconds", "seconds"].iter().find_map(|name| record.get(*name)?.as_f64());
    let machine = record.get("machine");
    let averages = |name: &str| -> Option<[f64; 3]> {
        let values = machine?.get(name)?.as_array()?;
        Some([0, 1, 2].map(|index| values.get(index).and_then(Value::as_f64).unwrap_or(0.0)))
    };
    let sample = |load: [f64; 3]| Sample {
        load,
        cpu_percent: None,
        foreign_cpu_percent: None,
        memory_used_bytes: None,
        vram_used_bytes: None,
        gpu_percent: None,
    };
    let load = averages("load_average").map(|start| Load {
        at_start: sample(start),
        at_end: averages("load_average_at_end").map(sample),
        during: None,
    });
    let machine = machine.map(|machine| Machine {
        hostname: String::new(),
        cpu: machine.get("processor").and_then(Value::as_str).unwrap_or_default().to_string(),
        cores: 0,
        threads: machine.get("threads").and_then(Value::as_u64).unwrap_or(0) as u32,
        memory_bytes: machine.get("memory").and_then(Value::as_str).and_then(gibibytes).unwrap_or(0),
        gpu: None,
        kernel: String::new(),
        label: String::new(),
    });
    Ok(Some(Run {
        format: FORMAT,
        id: format!("repository:{summary}"),
        job: job.name.clone(),
        source: Source::Seeded { from: summary.to_string() },
        queue: None,
        started,
        ended: None,
        seconds,
        status: Status::Ok,
        prepare: None,
        command: String::new(),
        environment: BTreeMap::new(),
        parameters: BTreeMap::new(),
        checkout: ".".to_string(),
        directory: ".".to_string(),
        git: Some(Git { commit, dirty: false, branch: None }),
        threads: None,
        memory_limit_gb: None,
        machine,
        load,
        calm: Calm {
            before: false,
            during: false,
            forced: false,
            timing_valid: false,
            reason: FROM_REPOSITORY.to_string(),
        },
        comment: record.get("experiment").and_then(Value::as_str).unwrap_or_default().to_string(),
        papers: job.paper.map(|paper| vec![crate::model::PaperRef { paper, label: String::new() }]).unwrap_or_default(),
        log: None,
        outputs: Vec::new(),
        edited: None,
    }))
}

/// The first object under `key`, at any depth (the export record sits in each summary).
fn find_key<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Object(map) => map.get(key).or_else(|| map.values().find_map(|inner| find_key(inner, key))),
        Value::Array(items) => items.iter().find_map(|inner| find_key(inner, key)),
        _ => None,
    }
}

fn string_at<'a>(value: &'a Value, pointer: &str) -> Option<&'a str> {
    let mut at = value;
    for part in pointer.split('/') {
        at = at.get(part)?;
    }
    at.as_str().filter(|text| !text.is_empty())
}

/// "62.7 GiB" in bytes.
fn gibibytes(text: &str) -> Option<u64> {
    let number: f64 = text.split_whitespace().next()?.parse().ok()?;
    Some((number * (1u64 << 30) as f64) as u64)
}

/// The last run of each job, among `runs`, kept if `keep` says so.
pub fn last_runs<'a>(runs: &'a [Run], keep: impl Fn(&Run) -> bool) -> BTreeMap<String, &'a Run> {
    let mut last: BTreeMap<String, &Run> = BTreeMap::new();
    for run in runs.iter().filter(|run| keep(run)) {
        if last.get(&run.job).is_none_or(|known| run.started >= known.started) {
            last.insert(run.job.clone(), run);
        }
    }
    last
}

/// What an estimate stands on (SPEC.md, section 10).
#[derive(Clone, Debug, PartialEq)]
pub struct Estimate {
    pub seconds: f64,
    pub runs: usize,
    /// The largest load1 at start among them, when known.
    pub load: Option<f64>,
}

impl Estimate {
    pub fn basis(&self) -> String {
        let runs = if self.runs == 1 { "1 run".to_string() } else { format!("median of {} runs", self.runs) };
        match self.load {
            Some(load) => format!("{runs}, load up to {load:.2}"),
            None => runs,
        }
    }
}

/// The median duration of the job's successful runs: same command and commit,
/// else same command, else any run of the job.
pub fn estimate(job: &str, command: &str, commit: &str, runs: &[Run]) -> Option<Estimate> {
    let timed: Vec<&Run> =
        runs.iter().filter(|run| run.job == job && run.status == Status::Ok && run.seconds.is_some()).collect();
    let same_commit = |run: &&Run| run.git.as_ref().is_some_and(|git| git.commit == commit);
    let levels: [Box<dyn Fn(&&Run) -> bool>; 3] = [
        Box::new(|run: &&Run| run.command == command && same_commit(run)),
        Box::new(|run: &&Run| run.command == command),
        Box::new(|_: &&Run| true),
    ];
    for level in levels {
        let chosen: Vec<&Run> = timed.iter().copied().filter(|run| level(run)).collect();
        if chosen.is_empty() {
            continue;
        }
        let mut seconds: Vec<f64> = chosen.iter().filter_map(|run| run.seconds).collect();
        seconds.sort_by(f64::total_cmp);
        let middle = seconds.len() / 2;
        let median =
            if seconds.len() % 2 == 1 { seconds[middle] } else { (seconds[middle - 1] + seconds[middle]) / 2.0 };
        let load = chosen.iter().filter_map(|run| run.load.as_ref().map(|load| load.at_start.load[0])).reduce(f64::max);
        return Some(Estimate { seconds: median, runs: chosen.len(), load });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
    }

    fn catalogue() -> Catalogue {
        Catalogue::read(&home().join("lab").join("jobs.toml")).unwrap()
    }

    /// The repository as it is: v0.7.0 is e13fadb, and its summaries are known
    /// (checked by hand on 2026-11-06).
    #[test]
    fn the_repository_runs_are_classed_by_their_hash() {
        let tags = tags(&home());
        let v070 = tags.iter().find(|tag| tag.name == "v0.7.0").expect("the tag v0.7.0");
        assert!(v070.commit.starts_with("e13fadb"));
        let (runs, problems) = repository_runs(&home(), &catalogue());
        assert!(problems.is_empty(), "{problems:?}");
        let run = |job: &str| runs.iter().find(|run| run.job == job).unwrap_or_else(|| panic!("no run of {job}"));
        // The hash the summary records: on v0.7.0.
        let forced = run("forced-whole-lower");
        assert_eq!(place(forced, &tags), "v0.7.0");
        assert_eq!(forced.seconds, Some(9503.5));
        assert_eq!(forced.load.as_ref().unwrap().at_start.load, [0.43, 1.77, 4.27]);
        // The export, nested in the summaries.
        let export = run("export-male-body");
        assert_eq!(place(export, &tags), "v0.7.0");
        assert_eq!(export.seconds, Some(187.3));
        // No hash recorded: the commit that brought it, off-tag.
        assert_eq!(place(run("gadget"), &tags), "off-tag 45ae56e");
        assert_eq!(run("certify").seconds, None);
        assert!(!runs.iter().any(|run| run.job == "lean-oracle-doubles"));
        let on_tag = runs.iter().filter(|run| tag_of(run, &tags).is_some()).count();
        assert_eq!((on_tag, runs.len() - on_tag), (24, 7));
    }

    #[test]
    fn the_last_run_and_the_estimate() {
        let (runs, _) = repository_runs(&home(), &catalogue());
        let last = last_runs(&runs, |_| true);
        assert_eq!(last.len(), 31);
        let estimate = estimate("holonomy", "python3 something", "0000000", &runs).unwrap();
        assert_eq!(estimate.seconds, 721.1);
        assert_eq!(estimate.basis(), "1 run, load up to 2.23");
        assert!(super::estimate("lean-oracle-doubles", "", "", &runs).is_none());
    }
}
