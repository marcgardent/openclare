//! The report of a batch (SPEC.md, §6b, amendment R4): the last record of
//! each cell written to `records.json`, then the batch's `report` command run
//! on it in a checkout. It computes nothing again: the table comes from the
//! files the jobs left, in seconds, as often as asked.

use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use chrono::Local;
use serde_json::{Value, json};

use crate::catalogue::{Batch, OUT, RECORDS, render};
use crate::home::Places;
use crate::mirror::Mirror;
use crate::model::Run;
use crate::runner::{describe, group_alive, kill_group, log_line, resolve_checkout, spawn_shell};

/// A report that runs longer is ended.
const PATIENCE: Duration = Duration::from_secs(600);

/// What a report made: its folder, whether its command ended well, its log.
#[derive(Clone, Debug)]
pub struct Made {
    pub folder: PathBuf,
    pub ok: bool,
    pub text: String,
}

/// The records of a batch's cells, as `records.json` holds them: each cell's
/// last record, of `launch` only when given.
pub fn records(places: &Places, batch: &Batch, runs: &[Run], launch: Option<&str>) -> Result<Value, String> {
    let cells = batch.expand(&[])?;
    let mut listed = Vec::with_capacity(cells.len());
    for cell in &cells {
        let values: BTreeMap<String, String> = cell.values.iter().cloned().collect();
        let last = runs
            .iter()
            .filter(|run| run.job == batch.job.name)
            .filter(|run| run.cell.as_ref().is_some_and(|recorded| recorded.values == values))
            .filter(|run| {
                launch.is_none_or(|launch| run.cell.as_ref().is_some_and(|recorded| recorded.launch == launch))
            })
            .max_by_key(|run| run.started);
        listed.push(match last {
            None => json!({ "values": values, "run": null }),
            Some(run) => {
                let out = places.data.join(OUT).join(&run.id);
                let outputs: Vec<String> = run
                    .outputs
                    .iter()
                    .map(|output| places.data.join(&output.path).to_string_lossy().into_owned())
                    .collect();
                json!({
                    "values": values,
                    "run": run.id,
                    "status": run.status.text(),
                    "seconds": run.seconds,
                    "timing_valid": run.calm.timing_valid,
                    "commit": run.git.as_ref().map(|git| git.commit.clone()),
                    "checkout": run.checkout,
                    "pool": run.cell.as_ref().map(|cell| cell.pool),
                    "out": out.to_string_lossy(),
                    "outputs": outputs,
                })
            }
        });
    }
    Ok(json!({
        "batch": batch.job.name,
        "axes": batch.axes.iter().map(|axis| axis.name.clone()).collect::<Vec<_>>(),
        "launch": launch,
        "cells": listed,
    }))
}

/// Makes the report of `batch`: its records written, its command run in the
/// checkout of `reference` (the batch's `checkout` when None).
pub fn make(
    places: &Places,
    mirror: &Mirror,
    batch: &Batch,
    launch: Option<&str>,
    reference: Option<&str>,
    setting: u32,
) -> Result<Made, String> {
    let Some(report) = &batch.report else {
        return Err(format!("{} has no report", batch.job.name));
    };
    let runs = crate::store::History::read(&places.runs()).map(|history| history.runs)?;
    let folder =
        places.data.join("reports").join(&batch.job.name).join(Local::now().format("%Y%m%d-%H%M%S").to_string());
    std::fs::create_dir_all(&folder).map_err(|error| format!("{}: {error}", folder.display()))?;
    let records_path = folder.join("records.json");
    let text = serde_json::to_string_pretty(&records(places, batch, &runs, launch)?).expect("records serialize");
    std::fs::write(&records_path, text + "\n").map_err(|error| format!("{}: {error}", records_path.display()))?;
    let target = reference.map(str::to_string).unwrap_or_else(|| batch.job.checkout.initial());
    let (root, git, as_run, _) = resolve_checkout(mirror, &target, &|_| {})?;
    let mut values =
        batch.job.values(setting, &git.short, &as_run.recorded(), Some(&root.to_string_lossy()), &batch.job.parameters);
    values.insert(OUT.to_string(), folder.to_string_lossy().into_owned());
    values.insert(RECORDS.to_string(), records_path.to_string_lossy().into_owned());
    let line = render(report, &values)?;
    let log_path = folder.join("report.log");
    let mut log = File::create(&log_path).map_err(|error| format!("{}: {error}", log_path.display()))?;
    let mut child = spawn_shell(&line, &root, &batch.job.environment_for(&values)?, None, &[], &log)?;
    let group = child.id() as i32;
    let begun = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if begun.elapsed() < PATIENCE => std::thread::sleep(Duration::from_millis(50)),
            Ok(None) => break None,
            Err(error) => return Err(format!("waiting for the report: {error}")),
        }
    };
    if status.is_none() || group_alive(group) {
        kill_group(group, Duration::from_secs(2));
    }
    let ok = status.is_some_and(|status| status.success());
    match status {
        Some(status) => {
            log_line(&mut log, &format!("{} after {:.1} s", describe(status), begun.elapsed().as_secs_f64()))
        }
        None => log_line(&mut log, "ended after 10 minutes"),
    }
    let text = std::fs::read_to_string(&log_path).unwrap_or_default();
    Ok(Made { folder, ok, text })
}

/// The table a report wrote, `report.md` in its folder, else its log.
pub fn table(made: &Made) -> String {
    std::fs::read_to_string(made.folder.join("report.md")).unwrap_or_else(|_| made.text.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::{Context, Options, Progress, run};
    use crate::catalogue::Catalogue;
    use std::path::Path;

    /// A batch run, then its report twice: the same table, nothing run again.
    #[test]
    fn a_report_reads_the_records_and_runs_nothing_again() {
        let lab = Path::new(env!("CARGO_MANIFEST_DIR"));
        let data = lab.join("target").join("test-data").join("report");
        let _ = std::fs::remove_dir_all(&data);
        std::fs::create_dir_all(&data).unwrap();
        let places = Places { home: Some(lab.parent().unwrap().to_path_buf()), data, catalogue: None };
        let text = r#"
            [[batch]]
            name = "sums"
            category = "build"
            command = "echo $(( {a} * 10 + {b} )) > {out}/value.txt"
            report = "python3 -c 'import json, sys; cells = json.load(open(sys.argv[1]))[\"cells\"]; print(sum(int(open(c[\"out\"] + \"/value.txt\").read()) for c in cells if c[\"run\"]))' {records} > {out}/report.md"
            threads = 1
            [batch.axes]
            a = [1, 2]
            b = [3, 4]
            "#;
        let catalogue = Catalogue::parse(text).unwrap();
        assert!(catalogue.problems.is_empty(), "{:?}", catalogue.problems);
        let batch = catalogue.batches[0].clone();
        let mirror = Mirror::new(&places.data, "nowhere/none");
        let home = places.home.clone().unwrap().to_string_lossy().into_owned();
        let stopped = || false;
        let mut say = |_: String| {};
        let mut progress = |_: Progress| {};
        let mut context = Context {
            places: &places,
            mirror: &mirror,
            machine: crate::machine::machine("test"),
            monitor: None,
            stopped: &stopped,
            say: &mut say,
            progress: &mut progress,
        };
        let options =
            Options { reference: Some(home.clone()), threads: 4, grace: Duration::from_secs(1), ..Options::default() };
        let summary = run(&mut context, &catalogue, &batch, &options).unwrap();
        assert_eq!(summary.counts(), "4 jobs: 4 ok");
        let first = make(&places, &mirror, &batch, None, Some(&home), 4).unwrap();
        assert!(first.ok, "{}", first.text);
        assert_eq!(table(&first).trim(), (13 + 14 + 23 + 24).to_string());
        let records: Value =
            serde_json::from_str(&std::fs::read_to_string(first.folder.join("records.json")).unwrap()).unwrap();
        assert_eq!(records["cells"].as_array().unwrap().len(), 4);
        assert_eq!(records["axes"], json!(["a", "b"]));
        let runs = crate::store::History::read(&places.runs()).unwrap().runs.len();
        std::thread::sleep(Duration::from_millis(1100));
        let second = make(&places, &mirror, &batch, None, Some(&home), 4).unwrap();
        assert_eq!(table(&second), table(&first));
        assert_eq!(crate::store::History::read(&places.runs()).unwrap().runs.len(), runs, "no record");
        // A launch that holds no record of a cell: that cell is null.
        let none = records_of(&places, &batch, Some("no-such-launch"));
        assert!(none["cells"].as_array().unwrap().iter().all(|cell| cell["run"].is_null()));
    }

    fn records_of(places: &Places, batch: &Batch, launch: Option<&str>) -> Value {
        let runs = crate::store::History::read(&places.runs()).unwrap().runs;
        records(places, batch, &runs, launch).unwrap()
    }
}
