//! The queue and its runner (SPEC.md, section 8).
//!
//! The window and the runner share the queue behind a mutex. The runner is a
//! thread of its own, so a queue goes on while the window is not drawn. It
//! runs one job at a time: its prepare, then its command, each `bash -c` in
//! a process group of its own, stdout and stderr in the run's log; then it
//! writes the record. `state.json` says, at every step, what is under way,
//! so that a run left behind by an app that died can be recorded.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use chrono::{DateTime, FixedOffset, Local};
use nix::sys::resource::{Resource, setrlimit};
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;
use serde::{Deserialize, Serialize};

use crate::catalogue::{Catalogue, Job, local_path, render};
use crate::files;
use crate::home::{GitState, Places, Worktree};
use crate::machine;
use crate::model::{Calm, FORMAT, FileRef, Git, Machine, PaperRef, Prepare, Run, Source, Status, Threads};
use crate::settings::Policy;

/// How often the runner looks at its child.
const TICK: Duration = Duration::from_millis(200);

/// What a run records of its calm until the monitor exists (milestone M3).
const NOT_MEASURED: &str = "not measured: the monitor comes with milestone M3";

#[derive(Clone, Debug)]
pub struct Item {
    pub key: u64,
    pub job: Job,
    /// The values chosen for the job's parameters.
    pub parameters: BTreeMap<String, String>,
    pub checkout: Worktree,
    pub state: ItemState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemState {
    Waiting,
    Running,
    Done { run: String, status: Status, seconds: Option<f64> },
    Cancelled,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Idle,
    Running,
    /// Waiting for Marc: why.
    Paused(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Prepare,
    Command,
}

/// What every run of a queue shares, fixed when Run is pressed.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct QueueSettings {
    pub id: String,
    pub comment: String,
    pub papers: Vec<PaperRef>,
    pub threads: u32,
    pub on_failure: Option<Policy>,
    pub grace_seconds: u32,
}

/// The run under way, as the window shows it.
#[derive(Clone, Debug)]
pub struct Current {
    pub key: u64,
    pub job: String,
    pub phase: Phase,
    pub started: Instant,
    pub log: PathBuf,
    pub stopping: bool,
}

/// A run of an app that died, whose processes still live.
#[derive(Clone, Debug)]
pub struct Orphan {
    pub run: Run,
    pub group: i32,
    /// Marc's answer: wait for it, or kill it.
    pub decision: Option<OrphanDecision>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrphanDecision {
    Wait,
    Kill,
}

#[derive(Debug, Default)]
pub struct Shared {
    pub items: Vec<Item>,
    pub mode: Mode,
    pub queue: QueueSettings,
    pub current: Option<Current>,
    /// Stop asked, and what to do with the rest of the queue then.
    pub stop: Option<Policy>,
    /// What went wrong, for the window to show.
    pub messages: Vec<String>,
    pub orphan: Option<Orphan>,
    next_key: u64,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Idle
    }
}

impl Shared {
    pub fn add(&mut self, job: Job, checkout: Worktree) -> u64 {
        self.next_key += 1;
        let parameters = job.parameters.clone();
        self.items.push(Item { key: self.next_key, job, parameters, checkout, state: ItemState::Waiting });
        self.next_key
    }

    /// Run the items waiting, with these settings.
    pub fn start(&mut self, queue: QueueSettings) {
        self.queue = queue;
        self.mode = Mode::Running;
    }

    pub fn cancel_waiting(&mut self) {
        for item in &mut self.items {
            if item.state == ItemState::Waiting {
                item.state = ItemState::Cancelled;
            }
        }
    }

    pub fn running(&self) -> bool {
        self.current.is_some() || self.orphan.is_some()
    }
}

/// The file that says what is under way.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub queue: QueueSettings,
    pub items: Vec<SavedItem>,
    pub current: Option<SavedCurrent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedItem {
    pub job: String,
    pub parameters: BTreeMap<String, String>,
    pub checkout: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedCurrent {
    /// The record as begun: what is known at the start.
    pub run: Run,
    pub group: i32,
}

fn state_path(places: &Places) -> PathBuf {
    places.data.join("state.json")
}

/// The record of a run, written to runs.jsonl; to refused.jsonl if it still
/// names a local path once scrubbed, so that it is kept but never copied
/// into the repository.
pub fn write_record(places: &Places, run: &Run) -> Result<(), String> {
    let line = serde_json::to_string(run).map_err(|error| error.to_string())?;
    if let Some(path) = local_path(&line) {
        files::append_line(&places.data.join("refused.jsonl"), &line)?;
        return Err(format!(
            "the record of {} names a local path ({path}): kept in refused.jsonl, not in runs.jsonl",
            run.id
        ));
    }
    files::append_line(&places.runs(), &line)
}

/// A run left behind by an app that ended during it (SPEC.md, section 8.5):
/// recorded `Interrupted` if its processes are gone, else handed to Marc;
/// the items that were waiting come back, the queue paused.
pub fn recover(places: &Places, catalogue: Option<&Catalogue>, worktrees: &[Worktree], shared: &mut Shared) {
    let Ok(text) = std::fs::read_to_string(state_path(places)) else { return };
    let state: State = match serde_json::from_str(&text) {
        Ok(state) => state,
        Err(error) => {
            shared.messages.push(format!("state.json: {error}"));
            return;
        }
    };
    let mut why = None;
    if let Some(current) = state.current {
        if group_alive(current.group) {
            why = Some(format!("The last session ended during {}, whose processes still run.", current.run.job));
            shared.orphan = Some(Orphan { run: current.run, group: current.group, decision: None });
        } else {
            let mut run = current.run;
            let log = run.log.as_ref().map(|log| places.data.join(&log.path));
            let ended =
                log.as_deref().and_then(|log| std::fs::metadata(log).and_then(|metadata| metadata.modified()).ok());
            run.ended = ended.map(|ended| DateTime::<Local>::from(ended).fixed_offset());
            run.status = Status::Interrupted;
            run.calm.reason = format!("the app ended during the run; {NOT_MEASURED}");
            if let Err(error) = write_record(places, &run) {
                shared.messages.push(error);
            }
            why = Some(format!("The last session ended during {}: recorded as interrupted.", run.job));
        }
    }
    for saved in state.items {
        let job = catalogue.and_then(|catalogue| catalogue.job(&saved.job)).cloned();
        let checkout = worktrees.iter().find(|worktree| worktree.relative == saved.checkout).cloned();
        match (job, checkout) {
            (Some(job), Some(checkout)) => {
                let key = shared.add(job, checkout);
                if let Some(item) = shared.items.iter_mut().find(|item| item.key == key) {
                    item.parameters = saved.parameters;
                }
            }
            _ => {
                shared.messages.push(format!("state.json: {} in {} cannot be queued again", saved.job, saved.checkout))
            }
        }
    }
    if why.is_some() || !shared.items.is_empty() {
        shared.queue = state.queue;
        shared.mode = Mode::Paused(why.unwrap_or_else(|| "The last session ended with jobs waiting.".to_string()));
    }
    // state.json stays: the runner writes it again at its first step. If the
    // app dies before, the next one reads it again, and the record of an
    // interrupted run, written again under the same id, is read once.
}

fn group_alive(group: i32) -> bool {
    group > 0 && killpg(Pid::from_raw(group), None).is_ok()
}

fn signal(group: i32, signal: Signal) {
    if group > 0 {
        let _ = killpg(Pid::from_raw(group), signal);
    }
}

/// Marc stops a recovered orphan: SIGTERM, then SIGKILL after the grace.
fn kill_group(group: i32, grace: Duration) {
    signal(group, Signal::SIGTERM);
    let asked = Instant::now();
    while group_alive(group) && asked.elapsed() < grace {
        std::thread::sleep(TICK);
    }
    signal(group, Signal::SIGKILL);
}

/// A job under way, held by the runner thread.
struct Running {
    key: u64,
    /// None until the first launch.
    child: Option<Child>,
    phase: Phase,
    phase_started: Instant,
    run: Run,
    /// The root of the checkout, and the working directory in it.
    root: PathBuf,
    directory: PathBuf,
    command: String,
    environment: BTreeMap<String, String>,
    memory_limit_gb: Option<u32>,
    log: File,
    outputs: Vec<String>,
    since: SystemTime,
    stop_sent: Option<Instant>,
    killed: bool,
    prepare_key: Option<String>,
}

pub struct Runner {
    shared: Arc<Mutex<Shared>>,
    places: Places,
    machine: Machine,
    repaint: Box<dyn Fn() + Send>,
    running: Option<Running>,
    /// The prepares done in this session: checkout, commit, command.
    prepared: Vec<(String, String)>,
    ids: BTreeSet<String>,
    /// What state.json holds, as last written: None for no file.
    written: Option<Option<String>>,
}

impl Runner {
    pub fn new(shared: Arc<Mutex<Shared>>, places: Places, machine: Machine, repaint: Box<dyn Fn() + Send>) -> Runner {
        let ids = crate::store::History::read(&places.runs())
            .map(|history| history.runs.into_iter().map(|run| run.id).collect())
            .unwrap_or_default();
        Runner { shared, places, machine, repaint, running: None, prepared: Vec::new(), ids, written: None }
    }

    /// The runner's thread: a step every tick, for as long as the app lives.
    pub fn spawn(mut self) {
        std::thread::Builder::new()
            .name("runner".to_string())
            .spawn(move || {
                loop {
                    self.step();
                    std::thread::sleep(TICK);
                }
            })
            .expect("the runner thread starts");
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Shared> {
        self.shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn message(&self, text: String) {
        self.lock().messages.push(text);
        (self.repaint)();
    }

    /// One look: the run under way, else the orphan, else the next item.
    pub fn step(&mut self) {
        self.look();
        self.save_state();
    }

    fn look(&mut self) {
        if self.running.is_some() {
            self.watch();
            return;
        }
        if self.watch_orphan() {
            return;
        }
        let next = {
            let mut shared = self.lock();
            if shared.mode != Mode::Running {
                return;
            }
            let queue = shared.queue.clone();
            match shared.items.iter_mut().find(|item| item.state == ItemState::Waiting) {
                Some(item) => {
                    item.state = ItemState::Running;
                    Some((item.clone(), queue))
                }
                None => {
                    shared.mode = Mode::Idle;
                    None
                }
            }
        };
        match next {
            Some((item, queue)) => {
                if let Err(error) = self.begin(&item, &queue) {
                    self.message(format!("{}: {error}", item.job.name));
                    self.settle(
                        item.key,
                        ItemState::Cancelled,
                        Some(Policy::PauseAndAsk),
                        &format!("{} could not start.", item.job.name),
                    );
                }
            }
            None => self.save_state(),
        }
        (self.repaint)();
    }

    fn unique_id(&mut self, started: &DateTime<FixedOffset>, job: &str) -> String {
        let base = format!("{}-{job}", started.format("%Y%m%d-%H%M%S"));
        let mut id = base.clone();
        let mut count = 1;
        while self.ids.contains(&id) {
            count += 1;
            id = format!("{base}-{count}");
        }
        self.ids.insert(id.clone());
        id
    }

    fn begin(&mut self, item: &Item, queue: &QueueSettings) -> Result<(), String> {
        let job = &item.job;
        let checkout = item.checkout.path.clone();
        let git = GitState::read(&checkout)?;
        let started = Local::now().fixed_offset();
        let id = self.unique_id(&started, &job.name);
        let threads = job.effective_threads(queue.threads);
        let recorded = job.values(queue.threads, &git.short, None, &item.parameters);
        let executed = job.values(queue.threads, &git.short, Some(&checkout.to_string_lossy()), &item.parameters);
        let command = render(&job.command, &executed)?;
        let environment = job.environment_for(&executed)?;
        let directory = checkout.join(render(&job.directory, &executed)?);
        let log_relative = format!("logs/{id}.log");
        let log_path = self.places.data.join(&log_relative);
        std::fs::create_dir_all(log_path.parent().expect("logs/ has a parent")).map_err(|error| error.to_string())?;
        let log = File::create(&log_path).map_err(|error| format!("{}: {error}", log_path.display()))?;
        let scrub = Scrub::new(&checkout);
        let run = Run {
            format: FORMAT,
            id: id.clone(),
            job: job.name.clone(),
            source: Source::App,
            queue: Some(queue.id.clone()),
            started,
            ended: None,
            seconds: None,
            status: Status::Interrupted,
            prepare: None,
            command: scrub.text(&render(&job.command, &recorded)?),
            environment: job
                .environment_for(&recorded)?
                .into_iter()
                .map(|(name, value)| (name, scrub.text(&value)))
                .collect(),
            parameters: item.parameters.iter().map(|(name, value)| (name.clone(), scrub.text(value))).collect(),
            checkout: item.checkout.relative.clone(),
            directory: job.directory.clone(),
            git: Some(Git { commit: git.commit.clone(), dirty: git.dirty, branch: git.branch.clone() }),
            threads: Some(Threads { setting: queue.threads, effective: threads }),
            memory_limit_gb: job.memory_limit_gb,
            machine: Some(self.machine.clone()),
            load: Some(crate::model::Load { at_start: machine::sample(), at_end: None, during: None }),
            calm: Calm {
                before: false,
                during: false,
                forced: false,
                timing_valid: false,
                reason: NOT_MEASURED.to_string(),
            },
            comment: queue.comment.clone(),
            papers: if queue.papers.is_empty() {
                job.paper.map(|paper| vec![PaperRef { paper, label: String::new() }]).unwrap_or_default()
            } else {
                queue.papers.clone()
            },
            log: Some(FileRef { path: log_relative, bytes: 0, sha256: None }),
            outputs: Vec::new(),
            edited: None,
        };
        let mut running = Running {
            key: item.key,
            child: None,
            phase: Phase::Command,
            phase_started: Instant::now(),
            run,
            root: checkout.clone(),
            directory,
            command,
            environment,
            memory_limit_gb: job.memory_limit_gb,
            log,
            outputs: job.outputs.clone(),
            since: SystemTime::now(),
            stop_sent: None,
            killed: false,
            prepare_key: None,
        };
        let prepare = match &job.prepare {
            Some(prepare) => Some((render(prepare, &executed)?, scrub.text(&render(prepare, &recorded)?))),
            None => None,
        };
        match prepare {
            Some((executed, recorded)) => {
                let key = format!("{}\n{}\n{executed}", item.checkout.relative, git.commit);
                let done =
                    self.prepared.iter().find(|(done, _)| !git.dirty && done == &key).map(|(_, why)| why.clone());
                match done {
                    Some(why) => {
                        log_line(&mut running.log, &format!("prepare skipped: {why}"));
                        running.run.prepare = Some(Prepare {
                            command: recorded,
                            seconds: 0.0,
                            status: Status::Ok,
                            skipped_because: Some(why),
                        });
                        self.launch(&mut running, Phase::Command)?;
                    }
                    None => {
                        running.prepare_key = Some(key);
                        running.run.prepare = Some(Prepare {
                            command: recorded,
                            seconds: 0.0,
                            status: Status::Ok,
                            skipped_because: None,
                        });
                        let command = std::mem::replace(&mut running.command, executed);
                        self.launch(&mut running, Phase::Prepare)?;
                        running.command = command;
                    }
                }
            }
            None => self.launch(&mut running, Phase::Command)?,
        }
        {
            let mut shared = self.lock();
            shared.current = Some(Current {
                key: item.key,
                job: job.name.clone(),
                phase: running.phase,
                started: Instant::now(),
                log: log_path,
                stopping: false,
            });
        }
        self.running = Some(running);
        self.save_state();
        Ok(())
    }

    /// Starts the prepare or the command of a run; `running.command` is
    /// what is executed.
    fn launch(&self, running: &mut Running, phase: Phase) -> Result<(), String> {
        let word = if phase == Phase::Prepare { "prepare" } else { "command" };
        log_line(&mut running.log, &format!("{word}: {}", running.command));
        let output = running.log.try_clone().map_err(|error| error.to_string())?;
        let errors = running.log.try_clone().map_err(|error| error.to_string())?;
        let mut command = Command::new("bash");
        command
            .arg("-c")
            .arg(&running.command)
            .current_dir(&running.directory)
            .envs(&running.environment)
            .stdin(Stdio::null())
            .stdout(output)
            .stderr(errors)
            .process_group(0);
        if let Some(gigabytes) = running.memory_limit_gb {
            // As `ulimit -v`, whose unit is the kilobyte: 32 GB is 32,000,000 kB.
            let bytes = u64::from(gigabytes) * 1_000_000 * 1024;
            // Safety: only setrlimit, async-signal-safe, between fork and exec.
            unsafe {
                command.pre_exec(move || setrlimit(Resource::RLIMIT_AS, bytes, bytes).map_err(std::io::Error::from));
            }
        }
        running.child = Some(command.spawn().map_err(|error| format!("bash: {error}"))?);
        running.phase = phase;
        running.phase_started = Instant::now();
        Ok(())
    }

    fn watch(&mut self) {
        let (stop, grace) = {
            let shared = self.lock();
            (shared.stop, Duration::from_secs(u64::from(shared.queue.grace_seconds.max(1))))
        };
        let running = self.running.as_mut().expect("a run under way");
        let Some(child) = running.child.as_mut() else { return };
        let group = child.id() as i32;
        if stop.is_some() {
            match running.stop_sent {
                None => {
                    log_line(&mut running.log, "stopped by Marc: SIGTERM to the process group");
                    signal(group, Signal::SIGTERM);
                    running.stop_sent = Some(Instant::now());
                    // The field, not self.lock(): self.running is borrowed.
                    if let Some(current) =
                        &mut self.shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).current
                    {
                        current.stopping = true;
                    }
                }
                Some(sent) if !running.killed && sent.elapsed() >= grace => {
                    log_line(
                        &mut running.log,
                        &format!("still running after {} s: SIGKILL to the group", grace.as_secs()),
                    );
                    signal(group, Signal::SIGKILL);
                    running.killed = true;
                }
                Some(_) => {}
            }
        }
        let status = match child.try_wait() {
            Ok(Some(status)) => status,
            Ok(None) => return,
            Err(error) => {
                log_line(&mut running.log, &format!("waiting for the shell failed: {error}"));
                return;
            }
        };
        let seconds = running.phase_started.elapsed().as_secs_f64();
        if group_alive(group) {
            log_line(
                &mut running.log,
                "processes of the group still alive after the shell ended: SIGTERM, then SIGKILL",
            );
            kill_group(group, grace);
        }
        log_line(&mut running.log, &format!("{} after {seconds:.1} s", describe(status)));
        let stopped = stop.is_some();
        if running.phase == Phase::Prepare {
            let prepare = running.run.prepare.as_mut().expect("a prepare was begun");
            prepare.seconds = seconds;
            prepare.status = status_of(status, stopped);
            if status.success() && !stopped {
                if let Some(key) = running.prepare_key.take() {
                    let why = format!("done for {} at {}", running.run.job, Local::now().format("%H:%M"));
                    self.prepared.push((key, why));
                }
                let mut running = self.running.take().expect("a run under way");
                match self.launch(&mut running, Phase::Command) {
                    Ok(()) => {
                        if let Some(current) = &mut self.lock().current {
                            current.phase = Phase::Command;
                        }
                        self.running = Some(running);
                        self.save_state();
                    }
                    Err(error) => {
                        self.running = Some(running);
                        self.finish(Status::Failed { code: None, signal: None }, None, &error);
                    }
                }
                return;
            }
            let why = if stopped { "stopped during its prepare".to_string() } else { "its prepare failed".to_string() };
            self.finish(status_of(status, stopped), None, &why);
            return;
        }
        self.finish(status_of(status, stopped), Some(seconds), "");
    }

    /// The record written, the item settled, the policy applied.
    fn finish(&mut self, status: Status, seconds: Option<f64>, why: &str) {
        let mut running = self.running.take().expect("a run under way");
        let ended = Local::now().fixed_offset();
        let root = running.root.clone();
        let run = &mut running.run;
        run.ended = Some(ended);
        run.seconds = seconds;
        run.status = status.clone();
        if let Some(load) = &mut run.load {
            load.at_end = Some(machine::sample());
        }
        let mut outputs: BTreeSet<String> = BTreeSet::new();
        for pattern in &running.outputs {
            outputs.extend(files::glob(&root, pattern).into_iter().map(|path| path.to_string_lossy().into_owned()));
        }
        // And the tracked files the run changed: those git shows modified, written since it started.
        outputs.extend(changed(&root));
        run.outputs = outputs
            .iter()
            .filter(|path| !path.ends_with("runs.jsonl"))
            .filter_map(|path| files::output(&root, Path::new(path), running.since))
            .collect();
        if let Some(log) = &mut run.log {
            log.bytes = std::fs::metadata(self.places.data.join(&log.path)).map(|metadata| metadata.len()).unwrap_or(0);
        }
        if !why.is_empty() {
            log_line(&mut running.log, why);
        }
        if let Err(error) = write_record(&self.places, run) {
            self.message(error);
        }
        let policy = {
            let shared = self.lock();
            match &status {
                Status::Ok => None,
                Status::Stopped => shared.stop.or(Some(Policy::PauseAndAsk)),
                _ => shared.queue.on_failure.or(Some(Policy::PauseAndAsk)),
            }
        };
        let summary = match &status {
            Status::Stopped => format!("{} was stopped.", run.job),
            Status::Ok => String::new(),
            other => format!(
                "{} {}{}.",
                run.job,
                other.text(),
                if why.is_empty() { String::new() } else { format!(": {why}") }
            ),
        };
        let done = ItemState::Done { run: run.id.clone(), status, seconds };
        self.settle(running.key, done, policy, &summary);
    }

    fn settle(&mut self, key: u64, state: ItemState, policy: Option<Policy>, summary: &str) {
        {
            let mut shared = self.lock();
            if let Some(item) = shared.items.iter_mut().find(|item| item.key == key) {
                item.state = state;
            }
            shared.current = None;
            shared.stop = None;
            match policy {
                None | Some(Policy::Continue) => {}
                Some(Policy::PauseAndAsk) => {
                    if shared.items.iter().any(|item| item.state == ItemState::Waiting) {
                        shared.mode = Mode::Paused(summary.to_string());
                    }
                }
                Some(Policy::CancelTheRest) => {
                    shared.cancel_waiting();
                    shared.mode = Mode::Idle;
                }
            }
        }
        self.save_state();
        (self.repaint)();
    }

    /// The orphan of a dead app, once Marc said what to do with it. True
    /// while it holds the queue.
    fn watch_orphan(&mut self) -> bool {
        let (orphan, grace) = {
            let shared = self.lock();
            (shared.orphan.clone(), Duration::from_secs(u64::from(shared.queue.grace_seconds.max(1))))
        };
        let Some(orphan) = orphan else { return false };
        let status = match orphan.decision {
            None => return true,
            Some(OrphanDecision::Wait) if group_alive(orphan.group) => return true,
            Some(OrphanDecision::Wait) => Status::Interrupted,
            Some(OrphanDecision::Kill) => {
                kill_group(orphan.group, grace);
                Status::Stopped
            }
        };
        let mut run = orphan.run;
        run.status = status;
        run.ended = Some(Local::now().fixed_offset());
        run.calm.reason = format!("the app ended during the run, its exit status is not known; {NOT_MEASURED}");
        if let Err(error) = write_record(&self.places, &run) {
            self.message(error);
        }
        self.lock().orphan = None;
        self.save_state();
        (self.repaint)();
        false
    }

    /// state.json as the queue is now: written when it changed, removed
    /// when nothing waits and nothing runs.
    fn save_state(&mut self) {
        let state = {
            let shared = self.lock();
            State {
                queue: shared.queue.clone(),
                items: shared
                    .items
                    .iter()
                    .filter(|item| item.state == ItemState::Waiting)
                    .map(|item| SavedItem {
                        job: item.job.name.clone(),
                        parameters: item.parameters.clone(),
                        checkout: item.checkout.relative.clone(),
                    })
                    .collect(),
                current: self
                    .running
                    .as_ref()
                    .map(|running| SavedCurrent {
                        run: running.run.clone(),
                        group: running.child.as_ref().map_or(0, |child| child.id() as i32),
                    })
                    .or_else(|| {
                        shared
                            .orphan
                            .as_ref()
                            .map(|orphan| SavedCurrent { run: orphan.run.clone(), group: orphan.group })
                    }),
            }
        };
        let path = state_path(&self.places);
        let empty = state.current.is_none() && state.items.is_empty();
        let text = (!empty).then(|| serde_json::to_string_pretty(&state).expect("a state serializes"));
        if self.written.as_ref() == Some(&text) {
            return;
        }
        let result = match &text {
            None => match std::fs::remove_file(&path) {
                Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
                    Err(format!("{}: {error}", path.display()))
                }
                _ => Ok(()),
            },
            Some(text) => crate::store::write_atomically(&path, text.as_bytes()),
        };
        match result {
            Ok(()) => self.written = Some(text),
            Err(error) => self.message(error),
        }
    }
}

fn log_line(log: &mut File, text: &str) {
    let _ = writeln!(log, "== lab {}: {text}", Local::now().format("%H:%M:%S"));
}

fn describe(status: ExitStatus) -> String {
    match (status.code(), status.signal()) {
        (Some(code), _) => format!("exit {code}"),
        (None, Some(signal)) => format!("killed by signal {signal}"),
        _ => "ended".to_string(),
    }
}

fn status_of(status: ExitStatus, stopped: bool) -> Status {
    if stopped {
        return Status::Stopped;
    }
    match (status.code(), status.signal()) {
        (Some(0), _) => Status::Ok,
        (code, signal) => Status::Failed { code, signal },
    }
}

/// The tracked files that differ from HEAD in a checkout.
fn changed(checkout: &Path) -> BTreeSet<String> {
    let output =
        Command::new("git").arg("-C").arg(checkout).args(["status", "--porcelain", "--untracked-files=no"]).output();
    let Ok(output) = output else { return BTreeSet::new() };
    String::from_utf8_lossy(&output.stdout).lines().filter_map(|line| line.get(3..)).map(str::to_string).collect()
}

/// Local paths written as a record keeps them: the checkout as `{repo}`,
/// the home folder as `~`.
struct Scrub {
    checkout: String,
    home: Option<String>,
}

impl Scrub {
    fn new(checkout: &Path) -> Scrub {
        Scrub {
            checkout: checkout.to_string_lossy().into_owned(),
            home: dirs::home_dir().map(|home| home.to_string_lossy().into_owned()),
        }
    }

    fn text(&self, text: &str) -> String {
        let mut text = text.replace(&self.checkout, "{repo}");
        if let Some(home) = &self.home {
            text = text.replace(home.as_str(), "~");
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalogue::Catalogue;
    use crate::store::History;

    /// A data folder of its own under target/, empty.
    fn places(name: &str) -> Places {
        let lab = Path::new(env!("CARGO_MANIFEST_DIR"));
        let data = lab.join("target").join("test-data").join(name);
        let _ = std::fs::remove_dir_all(&data);
        std::fs::create_dir_all(&data).unwrap();
        Places { home: lab.parent().unwrap().to_path_buf(), data: data.clone(), config: data }
    }

    fn jobs(text: &str) -> Catalogue {
        let catalogue = Catalogue::parse(text).unwrap();
        assert!(catalogue.problems.is_empty(), "{:?}", catalogue.problems);
        catalogue
    }

    fn home(places: &Places) -> Worktree {
        Worktree { path: places.home.clone(), relative: ".".to_string(), branch: None }
    }

    /// Runs the queue until the runner has nothing left to do; `act` may
    /// change the queue at every step, as the window would.
    fn run(places: &Places, catalogue: &Catalogue, names: &[&str], act: impl Fn(&mut Shared)) -> Arc<Mutex<Shared>> {
        let shared = Arc::new(Mutex::new(Shared::default()));
        {
            let mut queue = shared.lock().unwrap();
            for name in names {
                queue.add(catalogue.job(name).unwrap().clone(), home(places));
            }
            queue.start(QueueSettings {
                id: "test".to_string(),
                comment: "a test".to_string(),
                threads: 3,
                grace_seconds: 1,
                on_failure: Some(Policy::Continue),
                ..QueueSettings::default()
            });
        }
        let mut runner = Runner::new(shared.clone(), places.clone(), machine::machine("test"), Box::new(|| {}));
        let begun = Instant::now();
        loop {
            runner.step();
            {
                let mut queue = shared.lock().unwrap();
                act(&mut queue);
                if queue.mode != Mode::Running && runner.running.is_none() {
                    break;
                }
            }
            assert!(begun.elapsed() < Duration::from_secs(30), "the queue took too long");
            std::thread::sleep(Duration::from_millis(20));
        }
        shared
    }

    const JOBS: &str = r#"
        [[job]]
        name = "fine"
        category = "build"
        command = "echo threads $RAYON_NUM_THREADS {threads} in {repo}; ulimit -v"
        prepare = "echo prepared"
        memory_limit_gb = 2
        [[job]]
        name = "also-fine"
        category = "build"
        command = "true"
        prepare = "echo prepared"
        [[job]]
        name = "broken"
        category = "build"
        command = "exit 3"
        [[job]]
        name = "broken-prepare"
        category = "build"
        command = "echo never"
        prepare = "false"
        [[job]]
        name = "long"
        category = "build"
        command = "sleep 30"
        [[job]]
        name = "stubborn"
        category = "build"
        command = "trap '' TERM; sleep 30"
    "#;

    #[test]
    fn a_queue_records_each_run() {
        let places = places("queue");
        let catalogue = jobs(JOBS);
        run(&places, &catalogue, &["fine", "also-fine", "broken", "broken-prepare"], |_| {});
        let history = History::read(&places.runs()).unwrap();
        assert!(history.problems.is_empty(), "{:?}", history.problems);
        let runs = &history.runs;
        assert_eq!(runs.len(), 4);
        let statuses: Vec<String> = runs.iter().map(|run| run.status.text()).collect();
        assert_eq!(statuses, ["ok", "ok", "failed (3)", "failed (1)"]);
        // {repo} kept in the record, filled in the command executed.
        let fine = &runs[0];
        assert_eq!(fine.command, "echo threads $RAYON_NUM_THREADS 3 in {repo}; ulimit -v");
        assert_eq!(fine.environment["RAYON_NUM_THREADS"], "3");
        assert_eq!(fine.comment, "a test");
        assert!(fine.seconds.is_some() && fine.git.is_some() && fine.machine.is_some());
        let log = std::fs::read_to_string(places.data.join(&fine.log.as_ref().unwrap().path)).unwrap();
        assert!(log.contains("prepared"), "{log}");
        assert!(log.contains(&format!("threads 3 3 in {}", places.home.display())), "{log}");
        assert!(log.contains("2000000"), "the memory cap is ulimit -v 2000000: {log}");
        // The same prepare on the same commit runs once, if the checkout is clean.
        let also = runs[1].prepare.as_ref().unwrap();
        let dirty = runs[1].git.as_ref().unwrap().dirty;
        assert_eq!(also.skipped_because.is_some(), !dirty, "{also:?}");
        assert!(runs[3].seconds.is_none(), "the command of a failed prepare does not run");
        assert_eq!(runs[3].prepare.as_ref().unwrap().status, Status::Failed { code: Some(1), signal: None });
        assert!(!places.data.join("state.json").exists(), "nothing is under way any more");
    }

    #[test]
    fn stop_terms_then_kills_and_pauses_the_rest() {
        let places = places("stop");
        let catalogue = jobs(JOBS);
        let begun = Instant::now();
        let shared = run(&places, &catalogue, &["stubborn", "fine"], |queue| {
            let started =
                queue.current.as_ref().is_some_and(|current| current.started.elapsed() > Duration::from_millis(300));
            if started && queue.stop.is_none() {
                queue.stop = Some(Policy::PauseAndAsk);
            }
        });
        // The shell ignores SIGTERM: SIGKILL after the grace of one second.
        assert!(begun.elapsed() >= Duration::from_secs(1) && begun.elapsed() < Duration::from_secs(10));
        let queue = shared.lock().unwrap();
        assert!(matches!(queue.mode, Mode::Paused(_)), "{:?}", queue.mode);
        assert_eq!(queue.items[1].state, ItemState::Waiting, "the rest waits for Marc");
        let history = History::read(&places.runs()).unwrap();
        assert_eq!(history.runs.len(), 1, "the stopped run is recorded");
        assert_eq!(history.runs[0].status, Status::Stopped);
        let log = std::fs::read_to_string(places.data.join(&history.runs[0].log.as_ref().unwrap().path)).unwrap();
        assert!(log.contains("SIGTERM") && log.contains("SIGKILL"), "{log}");
    }

    #[test]
    fn a_run_left_behind_is_recorded_interrupted() {
        let places = places("recover");
        let catalogue = jobs(JOBS);
        // A queue stopped in the middle of `long`, as if the app had died:
        // its state written, its process group killed behind its back.
        let shared = Arc::new(Mutex::new(Shared::default()));
        {
            let mut queue = shared.lock().unwrap();
            queue.add(catalogue.job("long").unwrap().clone(), home(&places));
            queue.add(catalogue.job("fine").unwrap().clone(), home(&places));
            queue.start(QueueSettings {
                id: "test".to_string(),
                threads: 2,
                grace_seconds: 1,
                ..QueueSettings::default()
            });
        }
        let mut runner = Runner::new(shared.clone(), places.clone(), machine::machine("test"), Box::new(|| {}));
        runner.step();
        let mut child = runner.running.as_mut().unwrap().child.take().unwrap();
        let group = child.id() as i32;
        assert!(places.data.join("state.json").exists());
        std::mem::forget(runner);
        kill_group(group, Duration::from_secs(1));
        // Reaped, as init reaps the children of an app that died.
        child.wait().unwrap();
        let mut again = Shared::default();
        let worktrees = vec![home(&places)];
        recover(&places, Some(&catalogue), &worktrees, &mut again);
        assert!(matches!(again.mode, Mode::Paused(_)), "{:?}", again.mode);
        assert_eq!(again.items.len(), 1, "fine waits again");
        assert_eq!(again.items[0].job.name, "fine");
        let history = History::read(&places.runs()).unwrap();
        assert_eq!(history.runs.len(), 1);
        assert_eq!(history.runs[0].status, Status::Interrupted);
        assert!(history.runs[0].ended.is_some());
        // The file stays until a runner writes what is left: fine, waiting.
        let again = Arc::new(Mutex::new(again));
        let mut runner = Runner::new(again.clone(), places.clone(), machine::machine("test"), Box::new(|| {}));
        runner.step();
        let state: State =
            serde_json::from_str(&std::fs::read_to_string(places.data.join("state.json")).unwrap()).unwrap();
        assert!(state.current.is_none());
        assert_eq!(state.items.len(), 1);
        again.lock().unwrap().cancel_waiting();
        runner.step();
        assert!(!places.data.join("state.json").exists(), "nothing waits any more");
    }

    #[test]
    fn a_record_that_names_a_local_path_is_kept_apart() {
        let places = places("refused");
        let mut run: Run =
            serde_json::from_str(include_str!("../tests/fixtures/runs.jsonl").lines().next().unwrap()).unwrap();
        run.parameters.insert("export".to_string(), ["", "home", "someone", "export"].join("/"));
        assert!(write_record(&places, &run).unwrap_err().contains("refused.jsonl"));
        assert!(!places.runs().exists());
        assert!(places.data.join("refused.jsonl").exists());
    }
}
