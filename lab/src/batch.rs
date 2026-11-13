//! The run of a batch (SPEC.md, §6b): the grid expanded into jobs, the
//! checkouts of its references, the tools' versions read once per checkout,
//! the prepares one checkout after the other, then the jobs in a pool, each
//! in its own process group with its own log, folder, timeout and calm,
//! each recorded in `runs.jsonl` as it ends. A failed or timed-out job is a
//! failed cell; the batch goes on.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use chrono::{DateTime, FixedOffset, Local};
use nix::sys::signal::Signal;

use crate::catalogue::{Batch, CellPlan, OUT, Timeout, render};
use crate::files;
use crate::home::{GitState, Places};
use crate::mirror::{Mirror, Target};
use crate::model::{Calm, Cell, FORMAT, FileRef, Git, Machine, PaperRef, Prepare, Run, Source, Status, Threads};
use crate::monitor::{self, Calmness, Watch};
use crate::runner::{
    During, NOT_MEASURED, PREPARED, Scrub, describe, group_alive, kill_group, log_line, prepared_in, remember_prepared,
    resolve_checkout, signal, spawn_shell, status_of, write_record,
};
use crate::settings::CalmSettings;

/// How often the pool looks at its jobs.
const TICK: Duration = Duration::from_millis(100);

/// What a launch of a batch was asked.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Axes restricted by hand: `--axis model=ogre,pig`.
    pub only: Vec<(String, Vec<String>)>,
    /// The reference of a batch without a `ref` axis; its `checkout` else.
    pub reference: Option<String>,
    /// The batch's timeout replaced for this launch.
    pub timeout: Option<Timeout>,
    /// The thread setting: the pool's width is it divided by the job's threads.
    pub threads: u32,
    /// Between SIGTERM and SIGKILL.
    pub grace: Duration,
    /// Started although the machine is not calm: the times will not count.
    pub forced: bool,
    pub comment: String,
    pub papers: Vec<PaperRef>,
}

/// What the launch did, for the last lines and the exit code.
#[derive(Debug, Default)]
pub struct Summary {
    pub launch: String,
    pub jobs: usize,
    /// The records written: (cell, status, run id).
    pub records: Vec<(String, Status, String)>,
    /// Cells that got no record: not started (stopped before) or whose checkout could not be had.
    pub not_run: Vec<String>,
}

impl Summary {
    pub fn all_ok(&self) -> bool {
        self.not_run.is_empty() && self.records.iter().all(|(_, status, _)| *status == Status::Ok)
    }

    /// "72 jobs: 71 ok, 1 timed out".
    pub fn counts(&self) -> String {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for (_, status, _) in &self.records {
            *counts.entry(kind(status)).or_default() += 1;
        }
        if !self.not_run.is_empty() {
            counts.insert("not run".to_string(), self.not_run.len());
        }
        let mut parts: Vec<String> = Vec::new();
        if let Some(ok) = counts.remove("ok") {
            parts.push(format!("{ok} ok"));
        }
        parts.extend(counts.into_iter().map(|(kind, count)| format!("{count} {kind}")));
        format!("{} jobs: {}", self.jobs, parts.join(", "))
    }
}

/// A status without its code: what the counts group by.
fn kind(status: &Status) -> String {
    match status {
        Status::Failed { .. } => "failed".to_string(),
        other => other.text(),
    }
}

/// The places, the mirror, the machine and the monitor a launch uses.
pub struct Context<'a> {
    pub places: &'a Places,
    pub mirror: &'a Mirror,
    pub machine: Machine,
    pub monitor: Option<(Arc<Mutex<Watch>>, Arc<Mutex<CalmSettings>>)>,
    /// True once Marc asked to stop (Ctrl-C without a window).
    pub stopped: &'a dyn Fn() -> bool,
    /// Where the lines of progress go.
    pub say: &'a mut dyn FnMut(String),
}

/// A checkout the batch runs in, with what every job there records.
struct Place {
    root: PathBuf,
    git: GitState,
    pushed: bool,
    /// The reference as recorded: the one asked, "main" for an unknown one, "local".
    reference: String,
    local: bool,
    tools: BTreeMap<String, String>,
    prepare: Option<Prepare>,
}

impl Place {
    fn prepare_failed(&self) -> bool {
        self.prepare.as_ref().is_some_and(|prepare| prepare.status != Status::Ok)
    }
}

/// A job of the pool under way.
struct Job {
    index: usize,
    text: String,
    child: Option<Child>,
    started: Instant,
    since: SystemTime,
    run: Run,
    log: File,
    out: String,
    /// The checkout and the batch's `outputs` globs in it.
    root: PathBuf,
    globs: Vec<String>,
    timeout: Duration,
    stop_sent: Option<Instant>,
    killed: bool,
    timed_out: bool,
    stopped: bool,
    during: During,
}

/// The id of a job: the start time, the batch's name, the cell's values.
pub fn run_id(started: &DateTime<FixedOffset>, batch: &str, cell: &CellPlan) -> String {
    let mut id = format!("{}-{batch}", started.format("%Y%m%d-%H%M%S"));
    for (_, value) in &cell.values {
        let word: String = value
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '_' { c.to_ascii_lowercase() } else { '-' })
            .collect();
        id.push('-');
        id.push_str(word.trim_matches('-'));
    }
    id
}

/// The jobs of a pool: as many as the setting holds of the job's threads, at least one.
pub fn pool_width(setting: u32, job_threads: u32) -> u32 {
    (setting / job_threads.max(1)).max(1)
}

/// Runs a launch of `batch` to its end.
pub fn run(context: &mut Context, batch: &Batch, options: &Options) -> Result<Summary, String> {
    let job = &batch.job;
    let cells = batch.expand(&options.only)?;
    let has_ref_axis = batch.axis(crate::catalogue::REFERENCE_AXIS).is_some();
    if has_ref_axis && options.reference.is_some() {
        return Err(format!("{} has a ref axis: give its references with --axis ref=…", job.name));
    }
    let fallback = match (&options.reference, &job.checkout) {
        (Some(reference), _) => reference.clone(),
        (None, crate::catalogue::CheckoutChoice::Ask) if !has_ref_axis => {
            return Err(format!("{} says checkout = \"ask\": give --ref <reference>", job.name));
        }
        (None, choice) => choice.initial(),
    };
    let targets: Vec<String> =
        cells.iter().map(|cell| cell.reference.clone().unwrap_or_else(|| fallback.clone())).collect();
    let timeout = options.timeout.or(job.timeout).map_or(crate::catalogue::BATCH_TIMEOUT_SECONDS, |t| t.seconds);
    let threads = job.effective_threads(options.threads);
    let width = pool_width(options.threads, threads);
    let begun = Local::now().fixed_offset();
    let launch = format!("{}-{}", begun.format("%Y%m%d-%H%M%S"), job.name);
    let mut summary = Summary { launch: launch.clone(), jobs: cells.len(), ..Summary::default() };
    (context.say)(format!(
        "{}: {} jobs, {width} at a time of {threads} thread{} each, timeout {} each",
        job.name,
        cells.len(),
        if threads == 1 { "" } else { "s" },
        crate::model::duration_text(timeout)
    ));

    // The mirror is fetched once, when a cell runs on a reference.
    if targets.iter().any(|target| matches!(Target::parse(target), Target::Reference(_))) {
        (context.say)("fetching the mirror".to_string());
        if let Err(error) = context.mirror.fetch(context.places.seed().as_deref()) {
            (context.say)(format!("mirror: {error}"));
        }
    }

    // One place per distinct target, in the order of the grid.
    let mut places: BTreeMap<String, Result<Place, String>> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    for target in &targets {
        if places.contains_key(target) {
            continue;
        }
        order.push(target.clone());
        let place = resolve_checkout(context.mirror, target, &|_| {}).map(|(root, git, as_run, note)| {
            if let Some(note) = note {
                (context.say)(format!("{target}: {note}"));
            }
            let pushed = context.mirror.pushed(&git.commit);
            (context.say)(format!("{target}: {} in {}, reading the tools' versions", git.short, root.display()));
            let tools = crate::tools::read(&root);
            let listed: Vec<String> = tools.iter().map(|(name, version)| format!("{name} {version}")).collect();
            (context.say)(format!(
                "{target}: {}",
                if listed.is_empty() { "no tool found".to_string() } else { listed.join(", ") }
            ));
            Place {
                local: matches!(as_run, Target::Local(_)),
                reference: as_run.recorded(),
                root,
                git,
                pushed,
                tools,
                prepare: None,
            }
        });
        if let Err(error) = &place {
            (context.say)(format!("{target}: {error}; its cells do not run"));
        }
        places.insert(target.clone(), place);
    }

    // The prepares, one checkout after the other, before any job.
    if let Some(prepare) = &job.prepare {
        for target in &order {
            let Some(Ok(place)) = places.get_mut(target) else { continue };
            if (context.stopped)() {
                break;
            }
            let outcome = run_prepare(context, batch, prepare, place, options, &launch)?;
            place.prepare = Some(outcome);
        }
    }

    // A batch whose times matter starts once the monitor has a full window.
    if job.timing_matters
        && let Some((watch, calm)) = &context.monitor
    {
        let calm = calm.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
        let waited = Instant::now();
        let mut said = false;
        while monitor::lock(watch).verdict(&calm).calmness == Calmness::Measuring
            && waited.elapsed() < Duration::from_secs(u64::from(calm.window_seconds) + 10)
            && !(context.stopped)()
        {
            if !said {
                (context.say)("the monitor measures the machine before the first jobs".to_string());
                said = true;
            }
            std::thread::sleep(TICK);
        }
    }

    // The pool.
    let mut ids: BTreeSet<String> = crate::store::History::read(&context.places.runs())
        .map(|history| history.runs.into_iter().map(|run| run.id).collect())
        .unwrap_or_default();
    let mut next = 0;
    let mut running: Vec<Job> = Vec::new();
    let count = cells.len();
    let digits = count.to_string().len();
    loop {
        let stopping = (context.stopped)();
        while !stopping && running.len() < width as usize && next < count {
            let index = next;
            next += 1;
            let cell = &cells[index];
            let place = match places.get(&targets[index]) {
                Some(Ok(place)) => place,
                _ => {
                    summary.not_run.push(cell.text());
                    continue;
                }
            };
            let started = Local::now().fixed_offset();
            let mut id = run_id(&started, &job.name, cell);
            let base = id.clone();
            let mut suffix = 1;
            while ids.contains(&id) {
                suffix += 1;
                id = format!("{base}-{suffix}");
            }
            ids.insert(id.clone());
            match begin(context, batch, cell, place, options, &launch, id, started, width, timeout) {
                Ok(mut started_job) => {
                    started_job.index = index;
                    if started_job.child.is_none() {
                        // Its prepare failed: recorded at once, without running.
                        finish(
                            context,
                            started_job,
                            Status::Failed { code: None, signal: None },
                            None,
                            &mut summary,
                            digits,
                            count,
                        );
                    } else {
                        (context.say)(format!("[{:>digits$}/{count}] start  {}", index + 1, started_job.text));
                        running.push(started_job);
                    }
                }
                Err(error) => {
                    (context.say)(format!("[{:>digits$}/{count}] {}: {error}", index + 1, cell.text()));
                    summary.not_run.push(cell.text());
                }
            }
        }
        if stopping {
            while next < count {
                summary.not_run.push(cells[next].text());
                next += 1;
            }
        }
        if running.is_empty() && next >= count {
            break;
        }
        // Each job looked at: the monitor's point, its timeout, Stop, its end.
        let mut ended = Vec::new();
        for (position, job) in running.iter_mut().enumerate() {
            if let Some((watch, _)) = &context.monitor {
                job.during.take(&monitor::lock(watch));
            }
            let Some(child) = job.child.as_mut() else { continue };
            let group = child.id() as i32;
            if job.stop_sent.is_none() && stopping {
                log_line(&mut job.log, "stopped by Marc: SIGTERM to the process group");
                signal(group, Signal::SIGTERM);
                job.stop_sent = Some(Instant::now());
                job.stopped = true;
            }
            if job.stop_sent.is_none() && job.started.elapsed() >= job.timeout {
                log_line(
                    &mut job.log,
                    &format!(
                        "past its timeout of {}: SIGTERM to the process group",
                        crate::model::duration_text(job.timeout.as_secs_f64())
                    ),
                );
                signal(group, Signal::SIGTERM);
                job.stop_sent = Some(Instant::now());
                job.timed_out = true;
            }
            if let Some(sent) = job.stop_sent
                && !job.killed
                && sent.elapsed() >= options.grace
            {
                log_line(
                    &mut job.log,
                    &format!("still running after {} s: SIGKILL to the group", options.grace.as_secs()),
                );
                signal(group, Signal::SIGKILL);
                job.killed = true;
            }
            match child.try_wait() {
                Ok(Some(status)) => {
                    let seconds = job.started.elapsed().as_secs_f64();
                    if group_alive(group) {
                        log_line(
                            &mut job.log,
                            "processes of the group still alive after the shell ended: SIGTERM, then SIGKILL",
                        );
                        kill_group(group, options.grace);
                    }
                    log_line(&mut job.log, &format!("{} after {seconds:.1} s", describe(status)));
                    let recorded = if job.stopped {
                        Status::Stopped
                    } else if job.timed_out {
                        Status::TimedOut
                    } else {
                        status_of(status, false)
                    };
                    ended.push((position, recorded, seconds, group));
                }
                Ok(None) => {}
                Err(error) => log_line(&mut job.log, &format!("waiting for the shell failed: {error}")),
            }
        }
        for (position, status, seconds, group) in ended.into_iter().rev() {
            let job = running.remove(position);
            if let Some((watch, _)) = &context.monitor {
                monitor::lock(watch).groups.retain(|known| *known != group);
            }
            finish(context, job, status, Some(seconds), &mut summary, digits, count);
        }
        std::thread::sleep(TICK);
    }
    if let Some((watch, _)) = &context.monitor {
        monitor::lock(watch).our_last_end = Some(Instant::now());
    }
    Ok(summary)
}

/// The values of a cell's placeholders: the job's, the cell's, and `{out}`.
fn values(
    batch: &Batch,
    place: &Place,
    parameters: &BTreeMap<String, String>,
    setting: u32,
    repo: Option<&str>,
    out: &str,
) -> BTreeMap<String, String> {
    let mut values = batch.job.values(setting, &place.git.short, &place.reference, repo, parameters);
    values.insert(OUT.to_string(), out.to_string());
    values
}

/// The prepare of one checkout, once (`.lab-prepared`), with the whole
/// thread setting; its log `logs/<launch>-prepare-<short hash>.log`.
fn run_prepare(
    context: &mut Context,
    batch: &Batch,
    prepare: &str,
    place: &Place,
    options: &Options,
    launch: &str,
) -> Result<Prepare, String> {
    let job = &batch.job;
    let scrub = Scrub::new(&place.root);
    let mut executed = batch.job.values(
        options.threads,
        &place.git.short,
        &place.reference,
        Some(&place.root.to_string_lossy()),
        &job.parameters,
    );
    executed.insert("threads".to_string(), options.threads.to_string());
    let mut recorded_values = executed.clone();
    recorded_values.insert("repo".to_string(), "{repo}".to_string());
    let line = render(prepare, &executed)?;
    let recorded = scrub.text(&render(prepare, &recorded_values)?);
    let done = if place.local { None } else { prepared_in(&place.root, &line) };
    if let Some(why) = done {
        (context.say)(format!("{}: prepare skipped, {why}", place.reference));
        return Ok(Prepare { command: recorded, seconds: 0.0, status: Status::Ok, skipped_because: Some(why) });
    }
    let relative = format!("logs/{launch}-prepare-{}.log", place.git.short);
    let path = context.places.data.join(&relative);
    std::fs::create_dir_all(path.parent().expect("logs/ has a parent")).map_err(|error| error.to_string())?;
    let mut log = File::create(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    log_line(&mut log, &format!("checkout: {} at {} in {}", place.reference, place.git.commit, place.root.display()));
    log_line(&mut log, &format!("prepare: {line}"));
    (context.say)(format!("{}: prepare, its log {}", place.reference, path.display()));
    let environment = job.environment_for(&executed)?;
    let directory = place.root.join(render(&job.directory, &executed)?);
    let mut child = spawn_shell(&line, &directory, &environment, job.memory_limit_gb, &log)?;
    let group = child.id() as i32;
    if let Some((watch, _)) = &context.monitor {
        monitor::lock(watch).groups = vec![group];
    }
    let begun = Instant::now();
    let mut stop_sent: Option<Instant> = None;
    let status = loop {
        if (context.stopped)() && stop_sent.is_none() {
            log_line(&mut log, "stopped by Marc: SIGTERM to the process group");
            signal(group, Signal::SIGTERM);
            stop_sent = Some(Instant::now());
        }
        if let Some(sent) = stop_sent
            && sent.elapsed() >= options.grace
        {
            signal(group, Signal::SIGKILL);
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => std::thread::sleep(TICK),
            Err(error) => return Err(format!("waiting for the prepare: {error}")),
        }
    };
    if let Some((watch, _)) = &context.monitor {
        monitor::lock(watch).groups.clear();
    }
    if group_alive(group) {
        kill_group(group, options.grace);
    }
    let seconds = begun.elapsed().as_secs_f64();
    log_line(&mut log, &format!("{} after {seconds:.1} s", describe(status)));
    let status = status_of(status, stop_sent.is_some());
    if status == Status::Ok {
        if !place.local
            && let Err(error) = remember_prepared(&place.root, &job.name, &line)
        {
            log_line(&mut log, &format!("{PREPARED}: {error}"));
        }
        (context.say)(format!("{}: prepare done in {}", place.reference, crate::model::duration_text(seconds)));
    } else {
        (context.say)(format!(
            "{}: prepare {}, its cells fail; log {}",
            place.reference,
            status.text(),
            path.display()
        ));
    }
    Ok(Prepare { command: recorded, seconds, status, skipped_because: None })
}

/// A job begun: its folder and log made, its record as known at the start,
/// its command launched; not launched when its checkout's prepare failed.
#[allow(clippy::too_many_arguments)]
fn begin(
    context: &mut Context,
    batch: &Batch,
    cell: &CellPlan,
    place: &Place,
    options: &Options,
    launch: &str,
    id: String,
    started: DateTime<FixedOffset>,
    width: u32,
    timeout: f64,
) -> Result<Job, String> {
    let job = &batch.job;
    let out_relative = format!("{OUT}/{id}");
    let out = context.places.data.join(&out_relative);
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).map_err(|error| format!("{}: {error}", out.display()))?;
    let root = place.root.to_string_lossy().into_owned();
    let executed = values(batch, place, &cell.parameters, options.threads, Some(&root), &out.to_string_lossy());
    let recorded = values(batch, place, &cell.parameters, options.threads, None, "{out}");
    let scrub = Scrub::new(&place.root);
    let command = render(&job.command, &executed)?;
    let environment = job.environment_for(&executed)?;
    let directory = place.root.join(render(&job.directory, &executed)?);
    let log_relative = format!("logs/{id}.log");
    let log_path = context.places.data.join(&log_relative);
    std::fs::create_dir_all(log_path.parent().expect("logs/ has a parent")).map_err(|error| error.to_string())?;
    let mut log = File::create(&log_path).map_err(|error| format!("{}: {error}", log_path.display()))?;
    log_line(&mut log, &format!("batch: {} · launch {launch} · {}", job.name, cell.text()));
    log_line(&mut log, &format!("checkout: {} at {} in {}", place.reference, place.git.commit, place.root.display()));
    let before = context.monitor.as_ref().map(|(watch, calm)| {
        let calm = calm.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
        monitor::lock(watch).verdict(&calm)
    });
    // The other jobs of the pool are ours: `Ours` is calm for the one that starts.
    let calm_before =
        before.as_ref().is_some_and(|verdict| matches!(verdict.calmness, Calmness::Calm | Calmness::Ours));
    let mut cell_values: BTreeMap<String, String> = BTreeMap::new();
    for (axis, value) in &cell.values {
        cell_values.insert(axis.clone(), value.clone());
    }
    let run = Run {
        format: FORMAT,
        id: id.clone(),
        job: job.name.clone(),
        source: Source::App,
        queue: None,
        started,
        ended: None,
        seconds: None,
        status: Status::Interrupted,
        prepare: place.prepare.clone(),
        command: scrub.text(&render(&job.command, &recorded)?),
        environment: job
            .environment_for(&recorded)?
            .into_iter()
            .map(|(name, value)| (name, scrub.text(&value)))
            .collect(),
        parameters: cell.parameters.iter().map(|(name, value)| (name.clone(), scrub.text(value))).collect(),
        checkout: place.reference.clone(),
        directory: job.directory.clone(),
        git: Some(Git {
            commit: place.git.commit.clone(),
            dirty: place.git.dirty,
            branch: place.git.branch.clone(),
            pushed: place.pushed,
        }),
        threads: Some(Threads { setting: options.threads, effective: job.effective_threads(options.threads) }),
        memory_limit_gb: job.memory_limit_gb,
        machine: Some(context.machine.clone()),
        load: Some(crate::model::Load { at_start: crate::machine::sample(), at_end: None, during: None }),
        calm: Calm {
            before: calm_before,
            during: false,
            forced: options.forced,
            timing_valid: false,
            reason: match &before {
                None => NOT_MEASURED.to_string(),
                Some(_) if calm_before => String::new(),
                Some(verdict) => format!("not calm at the start: {}", verdict.text),
            },
        },
        comment: options.comment.clone(),
        papers: if options.papers.is_empty() {
            job.paper.map(|paper| vec![PaperRef { paper, label: String::new() }]).unwrap_or_default()
        } else {
            options.papers.clone()
        },
        log: Some(FileRef { path: log_relative, bytes: 0, sha256: None }),
        outputs: Vec::new(),
        edited: None,
        cell: Some(Cell { launch: launch.to_string(), values: cell_values, pool: width }),
        timeout_seconds: Some(timeout),
        tools: place.tools.clone(),
    };
    let mut started_job = Job {
        index: 0,
        text: cell.text(),
        child: None,
        started: Instant::now(),
        since: SystemTime::now(),
        run,
        log,
        out: out_relative,
        root: place.root.clone(),
        globs: job.outputs.clone(),
        timeout: Duration::from_secs_f64(timeout),
        stop_sent: None,
        killed: false,
        timed_out: false,
        stopped: false,
        during: During::default(),
    };
    if place.prepare_failed() {
        log_line(&mut started_job.log, "its checkout's prepare failed: the command does not run");
        return Ok(started_job);
    }
    log_line(&mut started_job.log, &format!("command: {command}"));
    let child = spawn_shell(&command, &directory, &environment, job.memory_limit_gb, &started_job.log)?;
    if let Some((watch, _)) = &context.monitor {
        monitor::lock(watch).groups.push(child.id() as i32);
    }
    started_job.child = Some(child);
    started_job.started = Instant::now();
    Ok(started_job)
}

/// The record of a job that ended, written at once, and its line said.
fn finish(
    context: &mut Context,
    mut job: Job,
    status: Status,
    seconds: Option<f64>,
    summary: &mut Summary,
    digits: usize,
    count: usize,
) {
    let run = &mut job.run;
    run.ended = Some(Local::now().fixed_offset());
    run.seconds = seconds;
    run.status = status.clone();
    if let Some(load) = &mut run.load {
        load.at_end = Some(crate::machine::sample());
    }
    if let Some((_, calm)) = &context.monitor {
        let calm = calm.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
        job.during.apply(run, &calm);
    }
    // Its folder's files, then the batch's globs in the checkout written since it started.
    let data = &context.places.data;
    let mut outputs: Vec<FileRef> = files_under(&data.join(&job.out))
        .into_iter()
        .filter_map(|path| path.strip_prefix(data).ok().map(Path::to_path_buf))
        .filter_map(|relative| files::output(data, &relative, job.since))
        .collect();
    for pattern in &job.globs {
        outputs.extend(
            files::glob(&job.root, pattern).iter().filter_map(|path| files::output(&job.root, path, job.since)),
        );
    }
    run.outputs = outputs;
    if let Some(log) = &mut run.log {
        log.bytes = std::fs::metadata(data.join(&log.path)).map(|metadata| metadata.len()).unwrap_or(0);
    }
    if let Err(error) = write_record(context.places, run) {
        (context.say)(error);
    }
    let time = seconds.map(|seconds| format!(" {}", crate::model::duration_text(seconds))).unwrap_or_default();
    let mut line = format!("[{:>digits$}/{count}] {}{time}  {}", job.index + 1, status.text(), job.text);
    if status != Status::Ok
        && let Some(log) = &run.log
    {
        line.push_str(&format!("  · log {}", data.join(&log.path).display()));
    }
    (context.say)(line);
    summary.records.push((job.text.clone(), status, run.id.clone()));
}

/// Every file under a folder, sorted.
fn files_under(folder: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut folders = vec![folder.to_path_buf()];
    while let Some(folder) = folders.pop() {
        let Ok(entries) = std::fs::read_dir(&folder) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                folders.push(path);
            } else {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalogue::Catalogue;
    use crate::store::History;

    /// A data folder of its own under target/, empty.
    fn places(name: &str) -> Places {
        let lab = Path::new(env!("CARGO_MANIFEST_DIR"));
        let data = lab.join("target").join("test-data").join(format!("batch-{name}"));
        let _ = std::fs::remove_dir_all(&data);
        std::fs::create_dir_all(&data).unwrap();
        Places { home: Some(lab.parent().unwrap().to_path_buf()), data, catalogue: None }
    }

    /// The batch run on this repository as a local folder, without a monitor.
    fn launch(places: &Places, text: &str, options: Options) -> (Summary, Vec<String>) {
        let catalogue = Catalogue::parse(text).unwrap();
        assert!(catalogue.problems.is_empty(), "{:?}", catalogue.problems);
        let batch = catalogue.batches[0].clone();
        let mirror = Mirror::new(&places.data, "nowhere/none");
        let mut lines = Vec::new();
        let stopped = || false;
        let mut say = |line: String| lines.push(line);
        let mut context = Context {
            places,
            mirror: &mirror,
            machine: crate::machine::machine("test"),
            monitor: None,
            stopped: &stopped,
            say: &mut say,
        };
        let options = Options {
            reference: Some(places.home.clone().unwrap().to_string_lossy().into_owned()),
            threads: 6,
            grace: Duration::from_secs(1),
            ..options
        };
        let begun = Instant::now();
        let summary = run(&mut context, &batch, &options).unwrap();
        assert!(begun.elapsed() < Duration::from_secs(30), "the batch took too long");
        (summary, lines)
    }

    const GRID: &str = r#"
        [[batch]]
        name = "grid"
        category = "quick-bench"
        prepare = "echo prepared with {threads}"
        command = "echo {model} {budget} $OPENCLARE_THREADS > {out}/said.txt; test {model} != bad && sleep {pause}"
        parameters = { pause = "0" }
        threads = 2
        timeout = "2s"
        [batch.axes]
        model = ["good", "bad", { value = "slow", pause = "30" }]
        budget = [1, 2]
        "#;

    #[test]
    fn a_batch_records_one_line_per_job_and_goes_on_past_a_failure() {
        let places = places("grid");
        let begun = Instant::now();
        let (summary, lines) = launch(&places, GRID, Options::default());
        assert!(begun.elapsed() < Duration::from_secs(10), "the slow jobs ran side by side and were cut at 2 s");
        assert_eq!(summary.counts(), "6 jobs: 2 ok, 2 failed, 2 timed out", "{lines:#?}");
        assert!(!summary.all_ok());
        let history = History::read(&places.runs()).unwrap();
        assert!(history.problems.is_empty(), "{:?}", history.problems);
        let runs = &history.runs;
        assert_eq!(runs.len(), 6);
        let good =
            runs.iter().find(|run| run.parameters["model"] == "good" && run.parameters["budget"] == "2").unwrap();
        assert_eq!(good.status, Status::Ok);
        assert_eq!(good.job, "grid");
        assert_eq!(good.checkout, "local");
        assert!(good.id.ends_with("-grid-good-2"), "{}", good.id);
        assert_eq!(good.command, "echo good 2 $OPENCLARE_THREADS > {out}/said.txt; test good != bad && sleep 0");
        assert_eq!(good.environment["OPENCLARE_THREADS"], "2", "the job's threads");
        assert_eq!(good.timeout_seconds, Some(2.0));
        let cell = good.cell.as_ref().unwrap();
        assert_eq!(cell.pool, 3, "6 threads of setting, 2 a job");
        assert_eq!(cell.values["model"], "good");
        assert_eq!(cell.values["budget"], "2");
        assert!(good.tools.contains_key("rustc"), "{:?}", good.tools);
        let said = good.outputs.iter().find(|output| output.path.ends_with("said.txt")).unwrap();
        assert_eq!(said.path, format!("out/{}/said.txt", good.id));
        assert_eq!(std::fs::read_to_string(places.data.join(&said.path)).unwrap(), "good 2 2\n");
        let prepare = good.prepare.as_ref().unwrap();
        assert_eq!(prepare.command, "echo prepared with 6", "the prepare has the whole setting");
        assert_eq!(prepare.status, Status::Ok);
        let slow = runs.iter().find(|run| run.parameters["model"] == "slow").unwrap();
        assert_eq!(slow.status, Status::TimedOut);
        assert_eq!(slow.parameters["pause"], "30", "the value's own placeholder");
        assert!(slow.seconds.unwrap() < 5.0);
        let log = std::fs::read_to_string(places.data.join(&slow.log.as_ref().unwrap().path)).unwrap();
        assert!(log.contains("past its timeout of 2.0 s"), "{log}");
        let bad = runs.iter().find(|run| run.parameters["model"] == "bad").unwrap();
        assert_eq!(bad.status, Status::Failed { code: Some(1), signal: None });
        assert!(lines.iter().any(|line| line.starts_with("[2/6] start")), "{lines:#?}");
        // A restricted launch, the timeout replaced: one cell, the same id plus a suffix if taken.
        let only = Options {
            only: vec![("model".to_string(), vec!["slow".to_string()]), ("budget".to_string(), vec!["1".to_string()])],
            timeout: Some(Timeout { seconds: 1.0 }),
            ..Options::default()
        };
        let (summary, _) = launch(&places, GRID, only);
        assert_eq!(summary.counts(), "1 jobs: 1 timed out");
        let runs = History::read(&places.runs()).unwrap().runs;
        assert_eq!(runs.len(), 7);
        assert_eq!(runs[6].timeout_seconds, Some(1.0));
    }

    #[test]
    fn a_failed_prepare_fails_its_cells_without_running_them() {
        let places = places("prepare");
        let text = GRID.replace("echo prepared with {threads}", "false");
        let (summary, _) = launch(&places, &text, Options::default());
        assert_eq!(summary.counts(), "6 jobs: 6 failed");
        let runs = History::read(&places.runs()).unwrap().runs;
        assert_eq!(runs.len(), 6);
        for run in &runs {
            assert!(run.seconds.is_none(), "the command did not run");
            assert_eq!(run.prepare.as_ref().unwrap().status, Status::Failed { code: Some(1), signal: None });
            assert!(run.outputs.is_empty());
        }
    }

    #[test]
    fn ids_widths_and_counts() {
        let started = DateTime::parse_from_rfc3339("2026-11-13T22:14:03+01:00").unwrap();
        let cell = CellPlan {
            values: vec![
                ("model".to_string(), "ogre".to_string()),
                ("budget".to_string(), "3000".to_string()),
                ("ref".to_string(), "pull/301".to_string()),
            ],
            parameters: BTreeMap::new(),
            reference: Some("pull/301".to_string()),
        };
        assert_eq!(
            run_id(&started, "pole-pairs-115-remesh", &cell),
            "20261113-221403-pole-pairs-115-remesh-ogre-3000-pull-301"
        );
        assert_eq!(pool_width(6, 1), 6);
        assert_eq!(pool_width(6, 4), 1);
        assert_eq!(pool_width(2, 8), 1, "at least one");
    }
}
