//! Where each job stands (SPEC.md, sections 10, 10b and 13): the runs the
//! repository's summaries record, read from the mirror, the last run of each
//! job, on a tag, off-tag or orphan, and the estimate of a job from its runs.

use std::collections::BTreeMap;

use chrono::{DateTime, FixedOffset};
use serde_json::Value;

use crate::catalogue::{Catalogue, Job};
pub use crate::mirror::Tag;
use crate::mirror::{MAIN, Mirror};
use crate::model::{Calm, FORMAT, Git, Load, Machine, Run, Sample, Source, Status};

/// The reason a run read from the repository gives for its calm.
pub const FROM_REPOSITORY: &str = "not measured (from the repository)";

/// Whether each commit the runs name is reachable from the remote, as the
/// mirror says now (SPEC.md, §10b): a commit absent from it is not pushed.
pub type Reach = BTreeMap<String, bool>;

/// Whether a run's commit is pushed: what the mirror says of it, else what
/// the record says.
pub fn is_pushed(run: &Run, reach: &Reach) -> bool {
    run.git.as_ref().is_some_and(|git| reach.get(&git.commit).copied().unwrap_or(git.pushed))
}

/// The tag a run is on, if the one hash it records is a tag's.
pub fn tag_of<'a>(run: &Run, tags: &'a [Tag]) -> Option<&'a Tag> {
    let commit = &run.git.as_ref()?.commit;
    tags.iter().find(|tag| !commit.is_empty() && tag.commit.starts_with(commit.as_str()))
}

/// Where a run stands by the one hash it records (SPEC.md, §10b): "v0.7.0",
/// "off-tag 45ae56e", "orphan 45ae56e", or "no hash".
pub fn place(run: &Run, tags: &[Tag], reach: &Reach) -> String {
    match (tag_of(run, tags), &run.git) {
        (Some(tag), _) => tag.name.clone(),
        (None, Some(git)) => {
            let word = if is_pushed(run, reach) { "off-tag" } else { "orphan" };
            format!("{word} {}", git.commit.chars().take(7).collect::<String>())
        }
        (None, None) => "no hash".to_string(),
    }
}

/// The three classes of a run with a hash, for the counts and the filters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    OnTag,
    OffTag,
    Orphan,
}

pub fn class(run: &Run, tags: &[Tag], reach: &Reach) -> Class {
    if tag_of(run, tags).is_some() {
        Class::OnTag
    } else if is_pushed(run, reach) {
        Class::OffTag
    } else {
        Class::Orphan
    }
}

/// The runs the repository's summaries record at `main` of the mirror, one
/// per job that names one (SPEC.md, §13).
pub fn repository_runs(mirror: &Mirror, catalogue: &Catalogue) -> (Vec<Run>, Vec<String>) {
    let mut runs = Vec::new();
    let mut problems = Vec::new();
    if !mirror.exists() {
        return (runs, problems);
    }
    for job in catalogue.every_job() {
        let Some(summary) = &job.summary else { continue };
        match repository_run(mirror, job, summary) {
            Ok(Some(run)) => runs.push(run),
            Ok(None) => {}
            Err(error) => problems.push(format!("{}: {error}", job.name)),
        }
    }
    (runs, problems)
}

/// The run a summary records; None if the file is not at `main`.
fn repository_run(mirror: &Mirror, job: &Job, summary: &str) -> Result<Option<Run>, String> {
    let (path, key) = match summary.split_once('#') {
        Some((path, key)) => (path, Some(key)),
        None => (summary, None),
    };
    let Some(text) = mirror.show(MAIN, path) else { return Ok(None) };
    let whole: Value = serde_json::from_str(&text).map_err(|error| format!("{path}: {error}"))?;
    let record = match key {
        Some(key) => find_key(&whole, key).ok_or_else(|| format!("{path}: no {key} in it"))?,
        None => &whole,
    };
    // The commit that brought the file: its date, and its hash if the record names none.
    let (brought_hash, brought_date) = mirror.brought(MAIN, path).ok_or_else(|| format!("{path}: not in git"))?;
    let started: DateTime<FixedOffset> =
        DateTime::parse_from_rfc3339(&brought_date).map_err(|error| error.to_string())?;
    let recorded = ["engine/commit", "export_record/engine/commit", "commit"]
        .iter()
        .find_map(|pointer| string_at(record, pointer))
        .and_then(|short| mirror.resolve(short).ok());
    let commit = recorded.unwrap_or_else(|| brought_hash.clone());
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
        checkout: MAIN.to_string(),
        directory: ".".to_string(),
        git: Some(Git { commit, dirty: false, branch: None, pushed: true }),
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
        cell: None,
        timeout_seconds: None,
        tools: BTreeMap::new(),
        dependencies: BTreeMap::new(),
        cache: None,
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
    use std::path::Path;

    fn catalogue() -> Catalogue {
        Catalogue::read(&Path::new(env!("CARGO_MANIFEST_DIR")).join("jobs.toml")).unwrap()
    }

    /// The repository as it is: v0.7.0 is e13fadb, and its summaries are known
    /// (checked by hand on 2026-11-06).
    #[test]
    fn the_repository_runs_are_classed_by_their_hash() {
        let mirror = crate::mirror::tests::mirror_at_head("state");
        let tags = mirror.tags();
        let v070 = tags.iter().find(|tag| tag.name == "v0.7.0").expect("the tag v0.7.0");
        assert!(v070.commit.starts_with("e13fadb"));
        let (runs, problems) = repository_runs(&mirror, &catalogue());
        assert!(problems.is_empty(), "{problems:?}");
        let reach = Reach::new();
        let run = |job: &str| runs.iter().find(|run| run.job == job).unwrap_or_else(|| panic!("no run of {job}"));
        // The hash the summary records: on v0.7.0.
        let forced = run("forced-whole-lower");
        assert_eq!(place(forced, &tags, &reach), "v0.7.0");
        assert_eq!(forced.seconds, Some(9503.5));
        assert_eq!(forced.load.as_ref().unwrap().at_start.load, [0.43, 1.77, 4.27]);
        // The export, nested in the summaries.
        let export = run("export-male-body");
        assert_eq!(place(export, &tags, &reach), "v0.7.0");
        assert_eq!(export.seconds, Some(187.3));
        // No hash recorded: the commit that brought it, off-tag; orphan if the mirror lost it.
        let gadget = run("gadget");
        assert_eq!(place(gadget, &tags, &reach), "off-tag 45ae56e");
        assert_eq!(class(gadget, &tags, &reach), Class::OffTag);
        let lost: Reach = [(gadget.git.as_ref().unwrap().commit.clone(), false)].into_iter().collect();
        assert_eq!(place(gadget, &tags, &lost), "orphan 45ae56e");
        assert_eq!(class(gadget, &tags, &lost), Class::Orphan);
        assert_eq!(class(forced, &tags, &lost), Class::OnTag);
        assert_eq!(run("certify").seconds, None);
        // Counted on 2026-11-06: 24 on v0.7.0, 7 off-tag. New summaries add to them.
        let on_tag = runs.iter().filter(|run| tag_of(run, &tags).is_some()).count();
        assert!(on_tag >= 24 && runs.len() - on_tag >= 6, "{on_tag} of {}", runs.len());
    }

    #[test]
    fn the_last_run_and_the_estimate() {
        let mirror = crate::mirror::tests::mirror_at_head("estimate");
        let (runs, _) = repository_runs(&mirror, &catalogue());
        let last = last_runs(&runs, |_| true);
        assert!(last.len() >= 31);
        let estimate = estimate("holonomy", "python3 something", "0000000", &runs).unwrap();
        assert_eq!(estimate.seconds, 721.1);
        assert_eq!(estimate.basis(), "1 run, load up to 2.23");
        assert!(super::estimate("lean-oracle-doubles", "", "", &runs).is_none());
    }
}
