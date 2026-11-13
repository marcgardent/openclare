//! lab without a window (SPEC.md, §14): `lab run <job> --ref <reference>`
//! runs one job with the same runner, monitor and records as the window,
//! its log streamed to stdout, and exits with the job's status; `lab fetch`
//! fetches the mirror, creating it if need be; `lab status` says where the
//! mirror, the checkouts, the queue and the history stand.

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use nix::sys::signal::{SigHandler, Signal, signal};

use crate::catalogue::Catalogue;
use crate::home::Places;
use crate::mirror::{MAIN, Mirror};
use crate::model::Status;
use crate::monitor::Watch;
use crate::runner::{ItemState, Mode, QueueSettings, Runner, Shared, State};
use crate::settings::{Policy, Push, Settings};
use crate::store::History;

/// What `lab run` was asked.
pub struct RunArguments {
    pub job: String,
    pub reference: Option<String>,
    /// Start although the machine is not calm, when the `wait` calm setting
    /// makes a timed job wait: the time will not count.
    pub force: bool,
    pub threads: Option<u32>,
    /// Push the live memory to this machine's branch after the run, whatever the setting.
    pub push: bool,
    /// Of a batch: the axes kept to some of their values (SPEC.md, §6b).
    pub only: Vec<(String, Vec<String>)>,
    /// The timeout of each job, replaced for this launch.
    pub timeout: Option<crate::catalogue::Timeout>,
    /// Of a batch: run again the jobs already done (SPEC.md, §6b, R3).
    pub again: bool,
}

/// Set by the SIGINT handler: the run is stopped and recorded, then lab exits.
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

extern "C" fn on_interrupt(_: nix::libc::c_int) {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

/// The catalogue as the window reads it: the file given by hand, else
/// `lab/jobs.toml` at `main` of the mirror, else the clone lab is started in.
fn catalogue(places: &Places, mirror: &Mirror) -> Result<Catalogue, String> {
    if let Some(file) = &places.catalogue {
        return Catalogue::read(file);
    }
    if mirror.exists()
        && let Some(text) = mirror.show(MAIN, "lab/jobs.toml")
    {
        return Catalogue::parse(&text).map_err(|error| format!("lab/jobs.toml at {MAIN} of the mirror: {error}"));
    }
    match places.catalogue_file() {
        Some(file) => Catalogue::read(&file),
        None => Err("no catalogue: run `lab fetch` first, or give one with --catalogue".to_string()),
    }
}

fn settings(places: &Places) -> Settings {
    Settings::load(&places.settings()).unwrap_or_else(|error| {
        eprintln!("lab: {error}: the default settings");
        Settings::default()
    })
}

/// `lab report <batch>` (SPEC.md, §6b, R4): its records written, its report
/// run, its table printed; nothing computed again.
pub fn report(places: &Places, batch: &str, launch: Option<&str>, reference: Option<&str>) -> ExitCode {
    let settings = settings(places);
    let mirror = Mirror::new(&places.data, &settings.remote);
    let catalogue = match catalogue(places, &mirror) {
        Ok(catalogue) => catalogue,
        Err(error) => {
            eprintln!("lab: {error}");
            return ExitCode::from(2);
        }
    };
    let Some(batch) = catalogue.batch(batch) else {
        eprintln!("lab: no batch named {batch} in the catalogue");
        return ExitCode::from(2);
    };
    match crate::report::make(places, &mirror, batch, launch, reference, settings.threads) {
        Ok(made) => {
            println!("{}", crate::report::table(&made));
            eprintln!("lab: the report and its records in {}", made.folder.display());
            if made.ok { ExitCode::SUCCESS } else { ExitCode::from(1) }
        }
        Err(error) => {
            eprintln!("lab: {error}");
            ExitCode::from(2)
        }
    }
}

/// `lab fetch`: the mirror created if need be, then fetched, and the other
/// machines' memories read into `<lab>/machines/` (SPEC.md, §4b).
pub fn fetch(places: &Places) -> ExitCode {
    let settings = settings(places);
    let mirror = Mirror::new(&places.data, &settings.remote);
    match mirror.fetch(places.seed().as_deref()) {
        Ok(said) => {
            println!("{said}");
            println!(
                "{} tags, {} branches, {} checkouts",
                mirror.tags().len(),
                mirror.branches().len(),
                mirror.checkouts().len()
            );
            match crate::machines::refresh(places, &mirror) {
                Ok(machines) if machines.is_empty() => println!("no machine has pushed its memory yet"),
                Ok(machines) => println!("machines: {}", machines.join(" ")),
                Err(error) => eprintln!("lab: machines: {error}"),
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("lab: {error}");
            ExitCode::FAILURE
        }
    }
}

/// `lab push`: the mirror fetched, then the live memory pushed to this
/// machine's branch (SPEC.md, §4b), whatever the `push` setting says.
pub fn push(places: &Places) -> ExitCode {
    let settings = settings(places);
    let mirror = Mirror::new(&places.data, &settings.remote);
    let result = mirror.fetch(places.seed().as_deref()).and_then(|said| {
        let _ = crate::machines::refresh(places, &mirror);
        crate::machines::push(places, &mirror, &settings.machine).map(|pushed| format!("{said}; {pushed}"))
    });
    match result {
        Ok(said) => {
            println!("{said}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("lab: {error}");
            ExitCode::FAILURE
        }
    }
}

/// `lab status`: the mirror, the checkouts, what is under way, the history.
pub fn status(places: &Places) -> ExitCode {
    let settings = settings(places);
    let mirror = Mirror::new(&places.data, &settings.remote);
    println!("folder    {}", places.data.display());
    let unpushed = crate::machines::unpushed(places, &mirror, &settings.machine).unwrap_or(0);
    let policy = match settings.push {
        Push::Ask => "on request",
        Push::Auto => "after every run",
        Push::Never => "never",
    };
    println!(
        "machine   {} · branch {} · {unpushed} runs not pushed · push {policy}",
        settings.machine,
        crate::mirror::runs_branch(&settings.machine)
    );
    let machines = mirror.machines();
    if !machines.is_empty() {
        println!("machines  {}", machines.join(" "));
    }
    if mirror.exists() {
        let fetched = mirror
            .last_fetch()
            .map(|at| chrono::DateTime::<chrono::Local>::from(at).format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "never".to_string());
        let tags = mirror.tags();
        println!("mirror    {} ← {} · fetched {fetched}", mirror.path.display(), mirror.url);
        println!("tags      {}", tags.iter().map(|tag| tag.name.as_str()).collect::<Vec<_>>().join(" "));
        let checkouts = mirror.checkouts();
        println!("checkouts {}", if checkouts.is_empty() { "none".to_string() } else { checkouts.len().to_string() });
        for checkout in &checkouts {
            let short: String = checkout.head.chars().take(7).collect();
            let place = match tags.iter().find(|tag| tag.commit == checkout.head) {
                Some(tag) => tag.name.clone(),
                None if mirror.pushed(&checkout.head) => format!("off-tag {short}"),
                None => format!("orphan {short}"),
            };
            let prepared = std::fs::read_to_string(checkout.path.join(crate::runner::PREPARED))
                .map(|text| text.lines().count())
                .unwrap_or(0);
            println!("  {} · {place} · {prepared} prepares done", checkout.path.display());
        }
    } else {
        println!("mirror    none yet: `lab fetch` creates it from {}", mirror.url);
    }
    match std::fs::read_to_string(places.data.join("state.json")) {
        Ok(text) => match serde_json::from_str::<State>(&text) {
            Ok(state) => {
                match &state.current {
                    Some(current) => println!(
                        "under way {} on {} since {} (process group {})",
                        current.run.job,
                        current.run.checkout,
                        current.run.started.format("%H:%M:%S"),
                        current.group
                    ),
                    None => println!("under way nothing"),
                }
                if !state.items.is_empty() {
                    let names: Vec<String> =
                        state.items.iter().map(|item| format!("{} on {}", item.job, item.checkout)).collect();
                    println!("waiting   {}", names.join(", "));
                }
            }
            Err(error) => println!("state.json: {error}"),
        },
        Err(_) => println!("under way nothing"),
    }
    match History::read(&places.runs()) {
        Ok(history) => {
            let last = history.runs.last().map(|run| {
                format!(
                    "{} on {} at {}, {}",
                    run.job,
                    run.checkout,
                    run.started.format("%Y-%m-%d %H:%M"),
                    run.status.text()
                )
            });
            println!("history   {} runs · last: {}", history.runs.len(), last.unwrap_or_else(|| "none".to_string()));
            for problem in &history.problems {
                println!("  {problem}");
            }
        }
        Err(error) => println!("history   {error}"),
    }
    ExitCode::SUCCESS
}

/// `lab run`: one job, its log on stdout, the exit code of the job.
pub fn run(places: &Places, arguments: RunArguments) -> ExitCode {
    let settings = settings(places);
    let mirror = Mirror::new(&places.data, &settings.remote);
    let catalogue = match catalogue(places, &mirror) {
        Ok(catalogue) => catalogue,
        Err(error) => {
            eprintln!("lab: {error}");
            return ExitCode::from(2);
        }
    };
    if let Some(batch) = catalogue.batch(&arguments.job).cloned() {
        return run_batch(places, &settings, mirror, &catalogue, &batch, arguments);
    }
    let Some(job) = catalogue.job(&arguments.job).cloned() else {
        eprintln!("lab: no job nor batch named {} in the catalogue", arguments.job);
        return ExitCode::from(2);
    };
    if !arguments.only.is_empty() {
        eprintln!("lab: {} is a job, without axes: --axis is for a batch", job.name);
        return ExitCode::from(2);
    }
    if arguments.timeout.is_some() {
        eprintln!("lab: {} is a job: --timeout is for a batch; a job's own is `timeout` in jobs.toml", job.name);
        return ExitCode::from(2);
    }
    if arguments.again {
        eprintln!("lab: {} is a job: --again is for a batch, whose jobs done before are skipped", job.name);
        return ExitCode::from(2);
    }
    let target = match (&arguments.reference, &job.checkout) {
        (Some(reference), _) => reference.clone(),
        (None, crate::catalogue::CheckoutChoice::Ask) => {
            eprintln!("lab: {} says checkout = \"ask\": give --ref <reference>", job.name);
            return ExitCode::from(2);
        }
        (None, choice) => choice.initial(),
    };
    let threads = arguments.threads.unwrap_or(settings.threads);
    let mut shared = Shared::default();
    // A run left behind by an earlier lab is recorded first (SPEC.md, §8.5); a
    // queue left waiting is not taken up here: the window shows it.
    crate::runner::recover(places, Some(&catalogue), &mut shared);
    for message in shared.messages.drain(..) {
        eprintln!("lab: {message}");
    }
    if shared.orphan.is_some() {
        eprintln!(
            "lab: a run of an earlier session still has processes alive: open the window to decide what to do with it"
        );
        return ExitCode::FAILURE;
    }
    shared.items.clear();
    let key = shared.add(job.clone(), target.clone());
    if arguments.force
        && let Some(item) = shared.items.iter_mut().find(|item| item.key == key)
    {
        item.forced = true;
    }
    shared.start(QueueSettings {
        id: chrono::Local::now().format("%Y%m%d-%H%M%S").to_string(),
        comment: String::new(),
        papers: Vec::new(),
        threads,
        on_failure: Some(Policy::CancelTheRest),
        grace_seconds: settings.stop_grace_seconds,
    });
    let shared = Arc::new(Mutex::new(shared));
    let watch = Arc::new(Mutex::new(Watch::default()));
    let calm = Arc::new(Mutex::new(settings.calm.clone()));
    let period = Duration::from_secs(u64::from(settings.sample_seconds.clamp(1, 5)));
    crate::monitor::spawn(watch.clone(), calm.clone(), period, Box::new(|| {}));
    let machine = crate::machine::machine(&settings.machine_label);
    let push = if arguments.push { Push::Auto } else { settings.push };
    let mut runner = Runner::new(shared.clone(), places.clone(), mirror, machine, Box::new(|| {}))
        .with_monitor(watch, calm)
        .with_push(push, settings.machine.clone());
    // Safety: the handler only stores a flag.
    unsafe {
        let _ = signal(Signal::SIGINT, SigHandler::Handler(on_interrupt));
        let _ = signal(Signal::SIGTERM, SigHandler::Handler(on_interrupt));
    }
    eprintln!("lab: {} on {target}, {threads} threads", job.name);
    let mut said: Vec<String> = Vec::new();
    let mut tail: Option<(PathBuf, std::fs::File, u64)> = None;
    let begun = Instant::now();
    let mut stdout = std::io::stdout();
    loop {
        runner.step();
        if INTERRUPTED.swap(false, Ordering::SeqCst) {
            let mut shared = shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if shared.current.is_some() {
                eprintln!(
                    "lab: interrupted: stopping the run, SIGTERM then SIGKILL after {} s",
                    settings.stop_grace_seconds
                );
                shared.stop = Some(Policy::CancelTheRest);
            } else {
                shared.cancel_waiting();
                shared.mode = Mode::Idle;
            }
        }
        let (done, messages, waiting, activity, log) = {
            let mut shared = shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let mut messages: Vec<String> = shared.messages.drain(..).collect();
            if let Some((_, said)) = shared.pushed.take() {
                messages.push(said);
            }
            let done = shared.mode != Mode::Running && shared.current.is_none();
            let log = shared.current.as_ref().map(|current| current.log.clone());
            (done, messages, shared.waiting_for_calm.clone(), shared.activity.clone(), log)
        };
        for message in messages {
            eprintln!("lab: {message}");
        }
        for line in [waiting, activity].into_iter().flatten() {
            if !said.contains(&line) {
                eprintln!("lab: {line}");
                said.push(line);
            }
        }
        // The log, as it grows.
        if let Some(path) = log {
            if tail.as_ref().is_none_or(|(known, _, _)| known != &path)
                && let Ok(file) = std::fs::File::open(&path)
            {
                tail = Some((path, file, 0));
            }
            if let Some((_, file, read)) = &mut tail {
                let mut bytes = Vec::new();
                if file.seek(SeekFrom::Start(*read)).is_ok()
                    && file.read_to_end(&mut bytes).is_ok()
                    && !bytes.is_empty()
                {
                    *read += bytes.len() as u64;
                    let _ = stdout.write_all(&bytes);
                    let _ = stdout.flush();
                }
            }
        }
        if done && runner_idle(&runner) {
            break;
        }
        if begun.elapsed() > Duration::from_secs(30 * 24 * 3600) {
            eprintln!("lab: giving up after thirty days");
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    let shared = shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let item = shared.items.iter().find(|item| item.key == key);
    match item.map(|item| &item.state) {
        Some(ItemState::Done { run, status, seconds }) => {
            let time =
                seconds.map(|seconds| format!(" in {}", crate::model::duration_text(seconds))).unwrap_or_default();
            eprintln!("lab: {} {}{time} · run {run}", job.name, status.text());
            match status {
                Status::Ok => ExitCode::SUCCESS,
                Status::Failed { code: Some(code), .. } => ExitCode::from((*code).clamp(1, 255) as u8),
                _ => ExitCode::FAILURE,
            }
        }
        Some(ItemState::Cancelled) => {
            eprintln!("lab: {} did not run", job.name);
            ExitCode::FAILURE
        }
        other => {
            eprintln!("lab: {} ended in an odd state: {other:?}", job.name);
            ExitCode::FAILURE
        }
    }
}

/// `lab run <batch>`: its grid in a pool, one line per job that starts and
/// ends, the counts at the end; 0 when every job is ok (SPEC.md, §6b).
fn run_batch(
    places: &Places,
    settings: &Settings,
    mirror: Mirror,
    catalogue: &Catalogue,
    batch: &crate::catalogue::Batch,
    arguments: RunArguments,
) -> ExitCode {
    let problems: Vec<&String> = catalogue
        .problems
        .iter()
        .filter(|problem| {
            problem.starts_with(&format!("batch {}:", batch.job.name))
                || batch.needs.iter().any(|need| problem.starts_with(&format!("dependency {need}:")))
        })
        .collect();
    if !problems.is_empty() {
        for problem in problems {
            eprintln!("lab: {problem}");
        }
        return ExitCode::from(2);
    }
    let mut shared = Shared::default();
    crate::runner::recover(places, Some(catalogue), &mut shared);
    for message in shared.messages.drain(..) {
        eprintln!("lab: {message}");
    }
    if shared.orphan.is_some() {
        eprintln!(
            "lab: a run of an earlier session still has processes alive: open the window to decide what to do with it"
        );
        return ExitCode::FAILURE;
    }
    let threads = arguments.threads.unwrap_or(settings.threads);
    let job_threads = batch.job.effective_threads(threads);
    let width = crate::batch::pool_width(threads, job_threads);
    if width * job_threads > settings.comfort_threads {
        eprintln!(
            "lab: {width} jobs of {job_threads} threads: {} threads, above the comfort limit of {}",
            width * job_threads,
            settings.comfort_threads
        );
    }
    let watch = Arc::new(Mutex::new(Watch::default()));
    let calm = Arc::new(Mutex::new(settings.calm.clone()));
    let period = Duration::from_secs(u64::from(settings.sample_seconds.clamp(1, 5)));
    crate::monitor::spawn(watch.clone(), calm.clone(), period, Box::new(|| {}));
    // Safety: the handler only stores a flag.
    unsafe {
        let _ = signal(Signal::SIGINT, SigHandler::Handler(on_interrupt));
        let _ = signal(Signal::SIGTERM, SigHandler::Handler(on_interrupt));
    }
    let options_reference = arguments.reference.clone();
    let options = crate::batch::Options {
        only: arguments.only,
        reference: arguments.reference,
        timeout: arguments.timeout,
        threads,
        grace: Duration::from_secs(u64::from(settings.stop_grace_seconds.max(1))),
        forced: arguments.force,
        comment: String::new(),
        papers: Vec::new(),
        cells: Vec::new(),
        again: arguments.again,
    };
    let stopped = || INTERRUPTED.load(Ordering::SeqCst);
    let mut say = |line: String| eprintln!("lab: {line}");
    let mut context = crate::batch::Context {
        places,
        mirror: &mirror,
        machine: crate::machine::machine(&settings.machine_label),
        monitor: Some((watch, calm)),
        stopped: &stopped,
        say: &mut say,
        progress: &mut |_| {},
    };
    let summary = match crate::batch::run(&mut context, catalogue, batch, &options) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("lab: {error}");
            return ExitCode::from(2);
        }
    };
    eprintln!("lab: {} · launch {}", summary.counts(), summary.launch);
    for (cell, status, id) in &summary.records {
        if *status != Status::Ok {
            eprintln!(
                "lab:   {} {cell} · log {}",
                status.text(),
                places.data.join("logs").join(format!("{id}.log")).display()
            );
        }
    }
    for cell in &summary.not_run {
        eprintln!("lab:   not run {cell}");
    }
    // A launch ends with its batch's report, on the last record of every cell (R4).
    if batch.report.is_some() && !INTERRUPTED.load(Ordering::SeqCst) {
        // On the reference the batch ran on, its report beside its jobs' files.
        match crate::report::make(places, &mirror, batch, None, options_reference.as_deref(), threads) {
            Ok(made) => {
                println!("{}", crate::report::table(&made));
                eprintln!("lab: the report in {}", made.folder.display());
            }
            Err(error) => eprintln!("lab: report: {error}"),
        }
    }
    if arguments.push || settings.push == Push::Auto {
        match crate::machines::push(places, &mirror, &settings.machine) {
            Ok(said) => eprintln!("lab: {said}"),
            Err(error) => eprintln!("lab: push: {error}"),
        }
    }
    if summary.all_ok() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

fn runner_idle(runner: &Runner) -> bool {
    runner.idle()
}
