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
use sha2::{Digest, Sha256};

use crate::catalogue::{Batch, Catalogue, Dependency, Job as CatalogueJob, OUT, Timeout, render};
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

/// The folder of the dependencies' results, under lab's data (R2).
pub const CACHE: &str = "cache";

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
    /// The cells kept, each its axes' values; empty: every cell (R3, the selection of the grid).
    pub cells: Vec<BTreeMap<String, String>>,
    /// Run again the jobs already done (R3).
    pub again: bool,
}

/// A cell as planned: its axes' values and its estimate.
pub type PlannedCell = (Vec<(String, String)>, Option<f64>);

/// What the pool tells the window of the cells of a launch (R3).
#[derive(Clone, Debug)]
pub enum Progress {
    /// The cells of the launch, in order: their axes' values and estimates.
    Planned {
        launch: String,
        width: u32,
        cells: Vec<PlannedCell>,
    },
    Started(usize),
    /// Its record written.
    Ended {
        index: usize,
        status: Status,
    },
    /// Done before: the grid shows that record.
    Skipped {
        index: usize,
    },
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
    /// The dependencies made for the launch (R2): (text, status, run id).
    pub dependencies: Vec<(String, Status, String)>,
    /// Cells done before, not run again (R3): (text, the record's id).
    pub skipped: Vec<(String, String)>,
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
        if !self.skipped.is_empty() {
            counts.insert("done before".to_string(), self.skipped.len());
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
    /// Where the state of each cell goes, for the grid (R3).
    pub progress: &'a mut dyn FnMut(Progress),
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
    timeout: Option<Duration>,
    stop_sent: Option<Instant>,
    killed: bool,
    timed_out: bool,
    stopped: bool,
    during: During,
}

/// The id of a job: the start time, the batch's name, the cell's values.
pub fn run_id(started: &DateTime<FixedOffset>, batch: &str, values: &[(String, String)]) -> String {
    let mut id = format!("{}-{batch}", started.format("%Y%m%d-%H%M%S"));
    for (_, value) in values {
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

/// A job the pool runs: a cell of the batch, or a dependency to make (R2).
struct Planned<'a> {
    job: &'a CatalogueJob,
    /// "model=ogre budget=3000 ref=main", "engine 3f2a1b2c on main".
    text: String,
    /// The values its id and its `cell` carry.
    values: Vec<(String, String)>,
    /// Its placeholders: the cell's parameters and axes.
    parameters: BTreeMap<String, String>,
    target: String,
    /// Placeholders given by the launch: name -> (executed, recorded), `{engine}` -> (its path, "{engine}").
    given: BTreeMap<String, (String, String)>,
    /// Why its command does not run: its prepare or a dependency failed.
    blocked: Option<String>,
    dependencies: BTreeMap<String, String>,
    /// A dependency's own run: the key, its folder moved into the cache when it ends Ok.
    cache: Option<String>,
    timeout: Option<f64>,
    /// A cell done before (R3): the id of its record.
    done: Option<String>,
    estimate: Option<f64>,
}

/// Runs a launch of `batch` to its end; `catalogue` holds the dependencies it needs.
pub fn run(context: &mut Context, catalogue: &Catalogue, batch: &Batch, options: &Options) -> Result<Summary, String> {
    let job = &batch.job;
    let mut cells = batch.expand(&options.only)?;
    if !options.cells.is_empty() {
        cells.retain(|cell| {
            options.cells.iter().any(|kept| cell.values.iter().all(|(axis, value)| kept.get(axis) == Some(value)))
        });
    }
    let has_ref_axis = batch.axis(crate::catalogue::REFERENCE_AXIS).is_some();
    if has_ref_axis && options.reference.is_some() {
        return Err(format!("{} has a ref axis: give its references with --axis ref=…", job.name));
    }
    let needs = batch
        .needs
        .iter()
        .map(|name| {
            catalogue.dependency(name).ok_or_else(|| format!("{} needs {name}, not in the catalogue", job.name))
        })
        .collect::<Result<Vec<&Dependency>, String>>()?;
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

    let history: Vec<Run> =
        crate::store::History::read(&context.places.runs()).map(|history| history.runs).unwrap_or_default();
    let mut ids: BTreeSet<String> = history.iter().map(|run| run.id.clone()).collect();

    // The dependencies, by key, before anything else; then which cell has which.
    let mut cell_keys: Vec<BTreeMap<String, Result<String, String>>> = vec![BTreeMap::new(); cells.len()];
    let mut made: BTreeMap<(String, String), bool> = BTreeMap::new();
    for dependency in &needs {
        let name = &dependency.job.name;
        let mut plans: Vec<Planned> = Vec::new();
        let mut said: BTreeSet<String> = BTreeSet::new();
        for (index, cell) in cells.iter().enumerate() {
            let Some(Ok(place)) = places.get(&targets[index]) else { continue };
            let key = dependency_key(dependency, place, &cell.parameters, options.threads);
            match &key {
                Err(error) => {
                    if said.insert(error.clone()) {
                        (context.say)(format!("{name}: {error}; the cells that need it fail"));
                    }
                }
                Ok(key) if !made.contains_key(&(name.clone(), key.clone())) => {
                    let cached = cache_folder(context.places, name, key).is_dir();
                    made.insert((name.clone(), key.clone()), cached);
                    let wanted: Vec<(String, String)> = dependency
                        .wanted()
                        .into_iter()
                        .filter_map(|wanted| cell.parameters.get(&wanted).map(|value| (wanted, value.clone())))
                        .collect();
                    let mut text = format!("{name} {}", &key[..8]);
                    for (wanted, value) in &wanted {
                        text.push_str(&format!(" {wanted}={value}"));
                    }
                    text.push_str(&format!(" on {}", place.reference));
                    if cached {
                        (context.say)(format!("{text}: cached"));
                    } else {
                        plans.push(Planned {
                            job: &dependency.job,
                            text,
                            values: wanted,
                            parameters: cell.parameters.clone(),
                            target: targets[index].clone(),
                            given: BTreeMap::new(),
                            blocked: None,
                            dependencies: BTreeMap::new(),
                            cache: Some(key.clone()),
                            timeout: dependency.job.timeout.map(|timeout| timeout.seconds),
                            done: None,
                            estimate: None,
                        });
                    }
                }
                Ok(_) => {}
            }
            cell_keys[index].insert(name.clone(), key);
        }
        if !plans.is_empty() {
            let dependency_threads = dependency.job.effective_threads(options.threads);
            let dependency_width = pool_width(options.threads, dependency_threads);
            (context.say)(format!("{name}: {} to make, {dependency_width} at a time", plans.len()));
            let mut not_run = Vec::new();
            let mut skipped = Vec::new();
            let stopped = context.stopped;
            let mut quiet = Pool { report: false, skipped: &mut skipped };
            let records = pool(
                context,
                plans,
                &places,
                options,
                &launch,
                dependency_width,
                &mut ids,
                &mut not_run,
                stopped,
                &mut quiet,
            );
            summary.dependencies.extend(records);
        }
        for ((dependency_name, key), present) in made.iter_mut() {
            if dependency_name == name {
                *present = cache_folder(context.places, name, key).is_dir();
            }
        }
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

    // The cells.
    let mut plans = Vec::with_capacity(cells.len());
    for (index, cell) in cells.iter().enumerate() {
        let mut given = BTreeMap::new();
        let mut dependencies = BTreeMap::new();
        let mut blocked = match places.get(&targets[index]) {
            Some(Ok(place)) if place.prepare_failed() => Some("its checkout's prepare failed".to_string()),
            _ => None,
        };
        for dependency in &needs {
            let name = &dependency.job.name;
            match cell_keys[index].get(name) {
                Some(Ok(key)) => {
                    dependencies.insert(name.clone(), key.clone());
                    let path = cache_folder(context.places, name, key).join(&dependency.provides);
                    given.insert(name.clone(), (path.to_string_lossy().into_owned(), format!("{{{name}}}")));
                    if !made.get(&(name.clone(), key.clone())).copied().unwrap_or(false) {
                        blocked.get_or_insert(format!("its dependency {name} {} failed", &key[..8]));
                    }
                }
                Some(Err(error)) => {
                    given.insert(name.clone(), (format!("{{{name}}}"), format!("{{{name}}}")));
                    blocked.get_or_insert(format!("its dependency {name}: {error}"));
                }
                None => {
                    given.insert(name.clone(), (format!("{{{name}}}"), format!("{{{name}}}")));
                }
            }
        }
        let mut plan = Planned {
            job,
            text: cell.text(),
            values: cell.values.clone(),
            parameters: cell.parameters.clone(),
            target: targets[index].clone(),
            given,
            blocked,
            dependencies,
            cache: None,
            timeout: Some(timeout),
            done: None,
            estimate: None,
        };
        if let Some(Ok(place)) = places.get(&plan.target)
            && let Ok((command, environment)) = recorded_texts(&plan, place, options.threads)
        {
            let same_key = |run: &&Run| {
                run.job == job.name
                    && run.command == command
                    && run.environment == environment
                    && run.directory == job.directory
            };
            let same_commit = |run: &&Run| run.git.as_ref().is_some_and(|git| git.commit == place.git.commit);
            plan.done = batch
                .skip_done
                .then(|| history.iter().rev())
                .into_iter()
                .flatten()
                .filter(same_key)
                .filter(same_commit)
                .find(|run| {
                    run.status == Status::Ok && run.tools == place.tools && run.dependencies == plan.dependencies
                })
                .map(|run| run.id.clone());
            let ok: Vec<&Run> = history
                .iter()
                .filter(|run| run.job == job.name && run.status == Status::Ok && run.seconds.is_some())
                .collect();
            let values: BTreeMap<String, String> = plan.values.iter().cloned().collect();
            let same_cell = |run: &&Run| run.cell.as_ref().is_some_and(|cell| cell.values == values);
            plan.estimate = median(ok.iter().copied().filter(same_key).filter(same_commit))
                .or_else(|| median(ok.iter().copied().filter(same_key)))
                .or_else(|| median(ok.iter().copied().filter(same_cell)));
        }
        plans.push(plan);
    }
    let to_run = |plan: &&Planned| plan.done.is_none() || options.again;
    let known: Vec<f64> = plans.iter().filter(to_run).filter_map(|plan| plan.estimate).collect();
    if !known.is_empty() {
        let total: f64 = known.iter().sum::<f64>() / f64::from(width);
        let unknown = plans.iter().filter(to_run).count() - known.len();
        (context.say)(format!(
            "about {} with a pool of {width}{}",
            crate::model::duration_text(total),
            if unknown == 0 { String::new() } else { format!(", plus {unknown} jobs without an estimate") }
        ));
    }
    (context.progress)(Progress::Planned {
        launch: launch.clone(),
        width,
        cells: plans.iter().map(|plan| (plan.values.clone(), plan.estimate)).collect(),
    });
    let stopped = context.stopped;
    let mut cells_pool = Pool { report: true, skipped: &mut summary.skipped };
    let records = pool(
        context,
        plans,
        &places,
        options,
        &launch,
        width,
        &mut ids,
        &mut summary.not_run,
        stopped,
        &mut cells_pool,
    );
    summary.records = records;
    if let Some((watch, _)) = &context.monitor {
        monitor::lock(watch).our_last_end = Some(Instant::now());
    }
    Ok(summary)
}

/// The median of some durations, None for none.
fn median<'a>(runs: impl Iterator<Item = &'a Run>) -> Option<f64> {
    let mut seconds: Vec<f64> = runs.filter_map(|run| run.seconds).collect();
    if seconds.is_empty() {
        return None;
    }
    seconds.sort_by(f64::total_cmp);
    let middle = seconds.len() / 2;
    Some(if seconds.len() % 2 == 1 { seconds[middle] } else { (seconds[middle - 1] + seconds[middle]) / 2.0 })
}

/// The command and environment a plan's record keeps: placeholders filled,
/// `{repo}`, `{out}` and the dependencies' paths kept, local paths scrubbed.
fn recorded_texts(plan: &Planned, place: &Place, setting: u32) -> Result<(String, BTreeMap<String, String>), String> {
    let job = plan.job;
    let mut recorded = job.values(setting, &place.git.short, &place.reference, None, &plan.parameters);
    recorded.insert(OUT.to_string(), format!("{{{OUT}}}"));
    for (name, (_, kept)) in &plan.given {
        recorded.insert(name.clone(), kept.clone());
    }
    let scrub = Scrub::new(&place.root);
    let command = scrub.text(&render(&job.command, &recorded)?);
    let environment =
        job.environment_for(&recorded)?.into_iter().map(|(name, value)| (name, scrub.text(&value))).collect();
    Ok((command, environment))
}

/// What a pool tells besides its lines: the cells' progress, and the cells done before.
struct Pool<'s> {
    report: bool,
    skipped: &'s mut Vec<(String, String)>,
}

/// `<lab>/cache/<name>/<key>/`.
pub fn cache_folder(places: &Places, name: &str, key: &str) -> PathBuf {
    places.data.join(CACHE).join(name).join(key)
}

/// The key of a dependency for a cell (SPEC.md, §6b, R2): its name, its
/// command as recorded, the git objects under each path of its key at the
/// checkout's commit (and the local changes of a local folder), the
/// versions of its tools; the first 16 hex digits of their SHA-256.
fn dependency_key(
    dependency: &Dependency,
    place: &Place,
    parameters: &BTreeMap<String, String>,
    setting: u32,
) -> Result<String, String> {
    let mut values = dependency.job.values(setting, &place.git.short, &place.reference, None, parameters);
    values.insert(OUT.to_string(), format!("{{{OUT}}}"));
    let mut hasher = Sha256::new();
    hasher.update(format!("{}\n{}\n", dependency.job.name, render(&dependency.job.command, &values)?));
    for path in &dependency.key {
        let path = render(path, &values)?;
        let listed = git(&place.root, &["ls-tree", "-r", "HEAD", "--", &path])?;
        if listed.trim().is_empty() {
            return Err(format!("key: {path} names nothing at {}", place.git.short));
        }
        hasher.update(format!("{path}\n{listed}"));
        if place.local {
            hasher.update(git(&place.root, &["diff", "HEAD", "--", &path])?);
        }
    }
    for tool in &dependency.tools {
        let version = place.tools.get(tool).map_or("none", String::as_str);
        hasher.update(format!("{tool}={version}\n"));
    }
    let digest = hasher.finalize();
    Ok(digest.iter().take(8).map(|byte| format!("{byte:02x}")).collect())
}

/// The standard output of a git command in `root`.
fn git(root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|error| format!("git: {error}"))?;
    if !output.status.success() {
        return Err(format!("git {}: {}", arguments.join(" "), String::from_utf8_lossy(&output.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Runs the planned jobs `width` at a time, in their order, to their ends;
/// returns their records (text, status, run id). `stopped` ends those under
/// way and starts no more.
#[allow(clippy::too_many_arguments)]
fn pool(
    context: &mut Context,
    plans: Vec<Planned>,
    places: &BTreeMap<String, Result<Place, String>>,
    options: &Options,
    launch: &str,
    width: u32,
    ids: &mut BTreeSet<String>,
    not_run: &mut Vec<String>,
    stopped: &dyn Fn() -> bool,
    tell: &mut Pool,
) -> Vec<(String, Status, String)> {
    let mut records = Vec::new();
    let mut next = 0;
    let mut running: Vec<Job> = Vec::new();
    let count = plans.len();
    let digits = count.to_string().len();
    loop {
        let stopping = stopped();
        while !stopping && running.len() < width as usize && next < count {
            let index = next;
            next += 1;
            let plan = &plans[index];
            if let Some(run) = &plan.done
                && !options.again
            {
                (context.say)(format!("[{:>digits$}/{count}] done before  {}  · {run}", index + 1, plan.text));
                tell.skipped.push((plan.text.clone(), run.clone()));
                if tell.report {
                    (context.progress)(Progress::Skipped { index });
                }
                continue;
            }
            let place = match places.get(&plan.target) {
                Some(Ok(place)) => place,
                _ => {
                    not_run.push(plan.text.clone());
                    continue;
                }
            };
            let started = Local::now().fixed_offset();
            let mut id = run_id(&started, &plan.job.name, &plan.values);
            if let Some(key) = &plan.cache {
                id.push('-');
                id.push_str(&key[..8]);
            }
            let base = id.clone();
            let mut suffix = 1;
            while ids.contains(&id) {
                suffix += 1;
                id = format!("{base}-{suffix}");
            }
            ids.insert(id.clone());
            match begin(context, plan, place, options, launch, id, started, width) {
                Ok(mut started_job) => {
                    started_job.index = index;
                    if started_job.child.is_none() {
                        // Its prepare or a dependency failed: recorded at once, without running.
                        let status = Status::Failed { code: None, signal: None };
                        finish(context, started_job, status.clone(), None, &mut records, digits, count);
                        if tell.report {
                            (context.progress)(Progress::Ended { index, status });
                        }
                    } else {
                        let estimate = plan
                            .estimate
                            .map(|seconds| format!("  ≈ {}", crate::model::duration_text(seconds)))
                            .unwrap_or_default();
                        (context.say)(format!(
                            "[{:>digits$}/{count}] start  {}{estimate}",
                            index + 1,
                            started_job.text
                        ));
                        if tell.report {
                            (context.progress)(Progress::Started(index));
                        }
                        running.push(started_job);
                    }
                }
                Err(error) => {
                    (context.say)(format!("[{:>digits$}/{count}] {}: {error}", index + 1, plan.text));
                    not_run.push(plan.text.clone());
                }
            }
        }
        if stopping {
            while next < count {
                not_run.push(plans[next].text.clone());
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
            if job.stop_sent.is_none()
                && let Some(limit) = job.timeout
                && job.started.elapsed() >= limit
            {
                log_line(
                    &mut job.log,
                    &format!(
                        "past its timeout of {}: SIGTERM to the process group",
                        crate::model::duration_text(limit.as_secs_f64())
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
            let index = job.index;
            finish(context, job, status.clone(), Some(seconds), &mut records, digits, count);
            if tell.report {
                (context.progress)(Progress::Ended { index, status });
            }
        }
        std::thread::sleep(TICK);
    }
    records
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
/// its command launched; not launched when it is blocked.
#[allow(clippy::too_many_arguments)]
fn begin(
    context: &mut Context,
    plan: &Planned,
    place: &Place,
    options: &Options,
    launch: &str,
    id: String,
    started: DateTime<FixedOffset>,
    width: u32,
) -> Result<Job, String> {
    let job = plan.job;
    let out_relative = format!("{OUT}/{id}");
    let out = context.places.data.join(&out_relative);
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).map_err(|error| format!("{}: {error}", out.display()))?;
    let root = place.root.to_string_lossy().into_owned();
    let mut executed = job.values(options.threads, &place.git.short, &place.reference, Some(&root), &plan.parameters);
    executed.insert(OUT.to_string(), out.to_string_lossy().into_owned());
    for (name, (now, _)) in &plan.given {
        executed.insert(name.clone(), now.clone());
    }
    let (recorded_command, recorded_environment) = recorded_texts(plan, place, options.threads)?;
    let scrub = Scrub::new(&place.root);
    let command = render(&job.command, &executed)?;
    let environment = job.environment_for(&executed)?;
    let directory = place.root.join(render(&job.directory, &executed)?);
    let log_relative = format!("logs/{id}.log");
    let log_path = context.places.data.join(&log_relative);
    std::fs::create_dir_all(log_path.parent().expect("logs/ has a parent")).map_err(|error| error.to_string())?;
    let mut log = File::create(&log_path).map_err(|error| format!("{}: {error}", log_path.display()))?;
    log_line(&mut log, &format!("{}: {} · launch {launch} · {}", job.name, plan.text, place.reference));
    log_line(&mut log, &format!("checkout: {} at {} in {}", place.reference, place.git.commit, place.root.display()));
    let before = context.monitor.as_ref().map(|(watch, calm)| {
        let calm = calm.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
        monitor::lock(watch).verdict(&calm)
    });
    // The other jobs of the pool are ours: `Ours` is calm for the one that starts.
    let calm_before =
        before.as_ref().is_some_and(|verdict| matches!(verdict.calmness, Calmness::Calm | Calmness::Ours));
    let cell_values: BTreeMap<String, String> = plan.values.iter().cloned().collect();
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
        command: recorded_command,
        environment: recorded_environment,
        parameters: plan.parameters.iter().map(|(name, value)| (name.clone(), scrub.text(value))).collect(),
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
        timeout_seconds: plan.timeout,
        tools: place.tools.clone(),
        dependencies: plan.dependencies.clone(),
        cache: plan.cache.clone(),
    };
    let mut started_job = Job {
        index: 0,
        text: plan.text.clone(),
        child: None,
        started: Instant::now(),
        since: SystemTime::now(),
        run,
        log,
        out: out_relative,
        root: place.root.clone(),
        globs: job.outputs.clone(),
        timeout: plan.timeout.map(Duration::from_secs_f64),
        stop_sent: None,
        killed: false,
        timed_out: false,
        stopped: false,
        during: During::default(),
    };
    if let Some(why) = &plan.blocked {
        log_line(&mut started_job.log, &format!("{why}: the command does not run"));
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

/// The record of a job that ended, written at once, and its line said. A
/// dependency that ended Ok has its folder moved into the cache first.
fn finish(
    context: &mut Context,
    mut job: Job,
    status: Status,
    seconds: Option<f64>,
    records: &mut Vec<(String, Status, String)>,
    digits: usize,
    count: usize,
) {
    let data = &context.places.data;
    if status == Status::Ok
        && let Some(key) = job.run.cache.clone()
    {
        let folder = cache_folder(context.places, &job.run.job, &key);
        let moved = if folder.is_dir() {
            // Another launch made it first: its folder wins.
            let _ = std::fs::remove_dir_all(data.join(&job.out));
            Ok(())
        } else {
            std::fs::create_dir_all(folder.parent().expect("cache/<name> has a parent"))
                .and_then(|_| std::fs::rename(data.join(&job.out), &folder))
        };
        match moved {
            Ok(()) => {
                job.out =
                    folder.strip_prefix(data).map(Path::to_path_buf).unwrap_or(folder).to_string_lossy().into_owned()
            }
            Err(error) => log_line(&mut job.log, &format!("moving its folder into the cache: {error}")),
        }
    }
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
    records.push((job.text.clone(), status, run.id.clone()));
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
    use crate::catalogue::CellPlan;
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
        let mut progress = |_: Progress| {};
        let mut context = Context {
            places,
            mirror: &mirror,
            machine: crate::machine::machine("test"),
            monitor: None,
            stopped: &stopped,
            say: &mut say,
            progress: &mut progress,
        };
        let options = Options {
            reference: Some(places.home.clone().unwrap().to_string_lossy().into_owned()),
            threads: 6,
            grace: Duration::from_secs(1),
            ..options
        };
        let begun = Instant::now();
        let summary = run(&mut context, &catalogue, &batch, &options).unwrap();
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

    const NEEDS: &str = r#"
        [[dependency]]
        name = "thing"
        category = "build"
        command = "echo made {model} > {out}/thing.txt"
        key = ["lab/Cargo.toml"]
        provides = "thing.txt"
        threads = 1

        [[batch]]
        name = "uses"
        category = "quick-bench"
        needs = ["thing"]
        command = "cat {thing} > {out}/got.txt"
        threads = 1
        [batch.axes]
        model = ["a", "b"]
        budget = [1, 2]
        "#;

    #[test]
    fn a_dependency_is_made_once_per_key_and_found_in_the_cache_the_second_time() {
        let places = places("needs");
        let (summary, lines) = launch(&places, NEEDS, Options::default());
        assert_eq!(summary.counts(), "4 jobs: 4 ok", "{lines:#?}");
        assert_eq!(summary.dependencies.len(), 2, "one per model, shared by its two budgets");
        let runs = History::read(&places.runs()).unwrap().runs;
        let made: Vec<&Run> = runs.iter().filter(|run| run.job == "thing").collect();
        assert_eq!(made.len(), 2);
        let key = made[0].cache.clone().unwrap();
        assert_eq!(key.len(), 16);
        assert!(made[0].outputs[0].path.starts_with(&format!("cache/thing/{key}/")), "{:?}", made[0].outputs);
        let cell = runs.iter().find(|run| run.job == "uses" && run.parameters["model"] == "a").unwrap();
        assert_eq!(cell.command, "cat {thing} > {out}/got.txt", "the path is kept out of the command key");
        let got = cell.outputs.iter().find(|output| output.path.ends_with("got.txt")).unwrap();
        assert_eq!(std::fs::read_to_string(places.data.join(&got.path)).unwrap(), "made a\n");
        assert_eq!(cell.dependencies["thing"].len(), 16);
        // The second launch makes nothing, and skips the jobs done (R3).
        let (summary, lines) = launch(&places, NEEDS, Options::default());
        assert_eq!(summary.counts(), "4 jobs: 4 done before");
        assert!(summary.all_ok());
        assert!(summary.dependencies.is_empty(), "{lines:#?}");
        assert_eq!(lines.iter().filter(|line| line.ends_with(": cached")).count(), 2, "{lines:#?}");
        let runs = History::read(&places.runs()).unwrap().runs;
        assert_eq!(runs.iter().filter(|run| run.job == "thing").count(), 2);
        assert_eq!(runs.len(), 6, "no new record");
        // Again, on one cell chosen: it runs, with an estimate from its first run.
        let chosen = Options {
            again: true,
            cells: vec![[("model".to_string(), "b".to_string()), ("budget".to_string(), "2".to_string())].into()],
            ..Options::default()
        };
        let (summary, lines) = launch(&places, NEEDS, chosen);
        assert_eq!(summary.counts(), "1 jobs: 1 ok", "{lines:#?}");
        assert!(lines.iter().any(|line| line.contains("start  model=b budget=2  ≈")), "{lines:#?}");
        assert!(lines.iter().any(|line| line.starts_with("about ")), "{lines:#?}");
    }

    #[test]
    fn a_failed_dependency_fails_the_cells_that_need_it() {
        let places = places("needs-failed");
        let text =
            NEEDS.replace("echo made {model} > {out}/thing.txt", "test {model} = a && echo made > {out}/thing.txt");
        let (summary, lines) = launch(&places, &text, Options::default());
        assert_eq!(summary.counts(), "4 jobs: 2 ok, 2 failed", "{lines:#?}");
        let runs = History::read(&places.runs()).unwrap().runs;
        for run in runs.iter().filter(|run| run.job == "uses" && run.parameters["model"] == "b") {
            assert!(run.seconds.is_none(), "the command did not run");
        }
        assert!(
            !places
                .data
                .join(CACHE)
                .join("thing")
                .read_dir()
                .unwrap()
                .any(|entry| { std::fs::read_to_string(entry.unwrap().path().join("thing.txt")).is_err() }),
            "a failed dependency leaves nothing in the cache"
        );
        // A key that names nothing: no dependency runs, its cells fail.
        let places = places_named("needs-nothing");
        let text = NEEDS.replace("lab/Cargo.toml", "lab/no-such-file");
        let (summary, lines) = launch(&places, &text, Options::default());
        assert_eq!(summary.counts(), "4 jobs: 4 failed", "{lines:#?}");
        assert!(summary.dependencies.is_empty());
        assert!(lines.iter().any(|line| line.contains("key: lab/no-such-file names nothing")), "{lines:#?}");
    }

    fn places_named(name: &str) -> Places {
        places(name)
    }

    /// A batch that says `skip_done = false` runs its jobs done again.
    #[test]
    fn a_batch_without_skip_runs_its_jobs_again() {
        let places = places("no-skip");
        let text = r#"
            [[batch]]
            name = "reads-scratch"
            category = "build"
            command = "echo {model} > {out}/said.txt"
            skip_done = false
            threads = 1
            [batch.axes]
            model = ["a", "b"]
            "#;
        let (first, _) = launch(&places, text, Options::default());
        assert_eq!(first.counts(), "2 jobs: 2 ok");
        let (second, _) = launch(&places, text, Options::default());
        assert_eq!(second.counts(), "2 jobs: 2 ok", "run again, not skipped");
        assert_eq!(History::read(&places.runs()).unwrap().runs.len(), 4);
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
            run_id(&started, "pole-pairs-115-remesh", &cell.values),
            "20261113-221403-pole-pairs-115-remesh-ogre-3000-pull-301"
        );
        assert_eq!(pool_width(6, 1), 6);
        assert_eq!(pool_width(6, 4), 1);
        assert_eq!(pool_width(2, 8), 1, "at least one");
    }
}
