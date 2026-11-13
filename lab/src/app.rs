//! The window (SPEC.md, section 11): a top bar and four tabs. The queue
//! screen is in `queue.rs`; the runner it drives, in `runner.rs`.

mod history;
mod machine;
mod queue;
mod state;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use eframe::egui::{self, Color32, RichText};
use egui_extras::{Column, TableBuilder};

use crate::catalogue::{Catalogue, Category, Job, ThreadUse, render};
use crate::home::{GitState, Places};
use crate::mirror::{MAIN, Mirror, Target};
use crate::model::{Paper, Run, Source, duration_text};
use crate::monitor::{Calmness, Watch};
use crate::runner::{Runner, Shared, recover};
use crate::settings::{Policy, Push, Settings};
use crate::state::{Reach, Tag, place, repository_runs};
use crate::store::History;

const AMBER: Color32 = Color32::from_rgb(255, 210, 122);
const RED: Color32 = Color32::from_rgb(255, 176, 176);
const GREEN: Color32 = Color32::from_rgb(143, 240, 176);
const MUTED: Color32 = Color32::from_rgb(154, 161, 171);
const BLUE: Color32 = Color32::from_rgb(169, 196, 255);

/// The mark of a job whose timing matters (SPEC.md, section 7.2): beside its
/// name wherever the job is shown. A job without it runs under any load and
/// its duration is an order of magnitude, not a measurement.
pub(crate) const STOPWATCH: &str = "⏱";
const STOPWATCH_HINT: &str =
    "its duration counts: it waits for a calm machine, and a run under load does not report a time";

/// The stopwatch beside a job's name, when its timing matters.
fn stopwatch(ui: &mut egui::Ui, timing_matters: bool) {
    if timing_matters {
        ui.label(RichText::new(STOPWATCH).color(AMBER)).on_hover_text(STOPWATCH_HINT);
    }
}

/// A run's duration (SPEC.md, section 7.2): "no time" when not measured. For
/// a job whose timing matters, the time is plain only when it is valid: muted
/// when its calm was not measured (a run read from the repository or typed),
/// struck through when the machine was not calm or the run was forced.
fn duration_cell(ui: &mut egui::Ui, run: &Run, timing_matters: bool) {
    let text = RichText::new(run.seconds.map_or("no time".to_string(), duration_text)).monospace();
    if !timing_matters || run.seconds.is_none() || run.calm.timing_valid {
        ui.label(text);
    } else if !matches!(run.source, Source::App) {
        ui.label(text.color(MUTED)).on_hover_text("its calm was not measured: an order of magnitude, not a timing");
    } else {
        let why = if run.calm.reason.is_empty() { "not calm".to_string() } else { run.calm.reason.clone() };
        ui.label(text.strikethrough().color(MUTED)).on_hover_text(format!("time not reported: {why}"));
    }
}

/// How often the files are looked at again.
const RELOAD: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    State,
    Queue,
    History,
    Catalogue,
    Machine,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortBy {
    Started,
    Job,
    Duration,
    Commit,
    Status,
}

#[derive(Default)]
struct Filter {
    job: Option<String>,
    paper: Option<Paper>,
    commit: String,
    text: String,
    valid_only: bool,
}

/// The checkouts of the mirror and their sizes, measured on a thread of their own.
type CheckoutSizes = Arc<Mutex<Option<Vec<(PathBuf, u64)>>>>;

/// What the mirror says of a reference typed on an item of the queue.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Resolution {
    /// 40 hexadecimal digits.
    pub commit: String,
    pub short: String,
    pub tag: Option<String>,
    pub pushed: bool,
    /// A local folder: tracked files differ from HEAD.
    pub dirty: bool,
    /// A checkout of that commit exists already (always, for a local folder).
    pub checked_out: bool,
}

pub struct App {
    places: Places,
    mirror: Mirror,
    settings: Settings,
    settings_error: Option<String>,
    catalogue: Result<Catalogue, String>,
    /// Where the catalogue was read from, for the screens.
    catalogue_source: String,
    /// The memories met (SPEC.md, §4b): this machine's live one and the other
    /// machines' as last fetched, one line per run id.
    history: Result<History, String>,
    /// The machine each run comes from.
    origin: BTreeMap<String, String>,
    /// Every machine met, this one first.
    machines: Vec<String>,
    /// The records of this machine its branch does not have yet.
    unpushed: usize,
    modified: (Option<SystemTime>, Option<SystemTime>),
    last_look: Instant,
    tab: Tab,
    about: bool,
    // Queue tab: the queue the runner runs, shared with it.
    shared: Arc<Mutex<Shared>>,
    // The monitor's samples, and the calm thresholds it and the runner read.
    watch: Arc<Mutex<Watch>>,
    calm: Arc<Mutex<crate::settings::CalmSettings>>,
    /// The tags and the branches of the mirror, to pick a reference from.
    references: Vec<String>,
    /// What each reference typed resolves to, as the mirror said last.
    resolutions: BTreeMap<String, Result<Resolution, String>>,
    /// Whether each commit the runs name is pushed, as the mirror said last.
    reach: Reach,
    /// The mirror changes the runner counted, as last read.
    mirror_seen: u64,
    search: String,
    comment: String,
    paper: Option<Paper>,
    label: String,
    /// The Stop dialog is open, with what to do with the rest then.
    stop_dialog: Option<Policy>,
    /// The window was asked to close during a run.
    closing: bool,
    quit_after_stop: bool,
    log: LogView,
    // History tab: the edit of the run selected, and the form of a past run.
    edit: Option<history::Edit>,
    past: Option<history::PastRun>,
    // Catalogue tab.
    job: Option<String>,
    // History tab.
    filter: Filter,
    sort: (SortBy, bool),
    run: Option<String>,
    // State tab: the tags and the runs the repository's summaries record, from the mirror.
    tags: Vec<Tag>,
    repository: Vec<Run>,
    repository_problems: Vec<String>,
    state_view: state::StateView,
    /// The clone "Copy history to the repo" writes into.
    copy_target: String,
    /// Machine tab: the checkouts of the mirror and their sizes, measured apart.
    checkout_sizes: CheckoutSizes,
    clean_dialog: bool,
    /// Where to write a picture of the window, then close it.
    screenshot: Option<Screenshot>,
}

/// The log shown beside the queue: the run under way's, else the last one's.
#[derive(Default)]
struct LogView {
    path: Option<PathBuf>,
    read: Option<Instant>,
    lines: Vec<String>,
    search: String,
}

/// How the window opens: from the command line (main.rs).
#[derive(Default)]
pub struct Opening {
    pub tab: Option<Tab>,
    /// Jobs or sequences put in the queue.
    pub queue: Vec<String>,
    /// The run selected in the history.
    pub run: Option<String>,
    pub screenshot: Option<std::path::PathBuf>,
    /// Seconds before the picture is taken.
    pub after: f64,
    /// Press Run, or go on with a queue left paused.
    pub start: bool,
    /// What main.rs has to say, shown with the runner's messages.
    pub messages: Vec<String>,
}

/// A picture of the window itself, never of the screen, written as a PPM;
/// then the window closes, even during a run, as an app that dies.
struct Screenshot {
    path: std::path::PathBuf,
    at: Instant,
    frames: u32,
    asked: bool,
}

impl Tab {
    pub fn named(name: &str) -> Option<Tab> {
        match name {
            "state" => Some(Tab::State),
            "queue" => Some(Tab::Queue),
            "history" => Some(Tab::History),
            "catalogue" => Some(Tab::Catalogue),
            "machine" => Some(Tab::Machine),
            _ => None,
        }
    }
}

fn modified(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|metadata| metadata.modified()).ok()
}

impl App {
    pub fn new(places: Places, opening: Opening, context: egui::Context) -> App {
        let (settings, settings_error) = match Settings::load(&places.settings()) {
            Ok(settings) => (settings, None),
            Err(error) => (Settings::default(), Some(error)),
        };
        let mirror = Mirror::new(&places.data, &settings.remote);
        let (catalogue, catalogue_source) = read_catalogue(&places, &mirror);
        let mut shared = Shared::default();
        recover(&places, catalogue.as_ref().ok(), &mut shared);
        shared.messages.extend(opening.messages.iter().cloned());
        // The mirror is created at the first start (SPEC.md, §2).
        shared.fetch_requested = !mirror.exists();
        let shared = Arc::new(Mutex::new(shared));
        let machine = crate::machine::machine(&settings.machine_label);
        let watch = Arc::new(Mutex::new(Watch::default()));
        let calm = Arc::new(Mutex::new(settings.calm.clone()));
        let period = Duration::from_secs(u64::from(settings.sample_seconds.clamp(1, 5)));
        let repaint = context.clone();
        crate::monitor::spawn(watch.clone(), calm.clone(), period, Box::new(move || repaint.request_repaint()));
        Runner::new(
            shared.clone(),
            places.clone(),
            mirror.clone(),
            machine,
            Box::new(move || context.request_repaint()),
        )
        .with_monitor(watch.clone(), calm.clone())
        .with_push(settings.push, settings.machine.clone())
        .spawn();
        let copy_target = places.home.as_ref().map(|home| home.to_string_lossy().into_owned()).unwrap_or_default();
        let mut app = App {
            catalogue,
            catalogue_source,
            history: Ok(History::default()),
            origin: BTreeMap::new(),
            machines: Vec::new(),
            unpushed: 0,
            modified: (places.catalogue_file().as_deref().and_then(modified), modified(&places.runs())),
            places,
            mirror,
            settings,
            settings_error,
            last_look: Instant::now(),
            tab: opening.tab.unwrap_or(Tab::State),
            about: false,
            shared,
            watch,
            calm,
            references: Vec::new(),
            resolutions: BTreeMap::new(),
            reach: Reach::new(),
            mirror_seen: 0,
            search: String::new(),
            comment: String::new(),
            paper: None,
            label: String::new(),
            stop_dialog: None,
            closing: false,
            quit_after_stop: false,
            log: LogView::default(),
            edit: None,
            past: None,
            job: None,
            filter: Filter::default(),
            sort: (SortBy::Started, false),
            run: opening.run,
            tags: Vec::new(),
            repository: Vec::new(),
            repository_problems: Vec::new(),
            state_view: state::StateView::default(),
            copy_target,
            checkout_sizes: Arc::new(Mutex::new(None)),
            clean_dialog: false,
            screenshot: opening.screenshot.map(|path| Screenshot {
                path,
                at: Instant::now() + Duration::from_secs_f64(opening.after.max(0.0)),
                frames: 0,
                asked: false,
            }),
        };
        for name in &opening.queue {
            app.add_to_queue(name);
        }
        if opening.start {
            let settings = app.queue_settings();
            let mut shared = app.lock();
            match shared.mode {
                crate::runner::Mode::Idle => shared.start(settings),
                _ if shared.orphan.is_none() => shared.mode = crate::runner::Mode::Running,
                _ => {}
            }
        }
        app.read_repository();
        app.job = app.catalogue.as_ref().ok().and_then(|catalogue| catalogue.jobs.first()).map(|job| job.name.clone());
        app
    }

    /// Reads again what changed: a catalogue file edited by hand, runs.jsonl
    /// written by another process, the mirror fetched or a checkout added.
    fn look_again(&mut self) {
        let moved = self.lock().mirror_changes;
        if moved != self.mirror_seen {
            self.mirror_seen = moved;
            self.read_repository();
            return;
        }
        if self.last_look.elapsed() < RELOAD {
            return;
        }
        self.last_look = Instant::now();
        let now = (self.places.catalogue_file().as_deref().and_then(modified), modified(&self.places.runs()));
        if now.0 != self.modified.0 && self.places.catalogue_file().is_some() {
            (self.catalogue, self.catalogue_source) = read_catalogue(&self.places, &self.mirror);
        }
        if now.1 != self.modified.1 {
            self.read_history();
            self.reach_all();
        }
        self.modified = now;
    }

    /// The memories met, read again (SPEC.md, §4b): the other machines' as
    /// the folder has them, this machine's live one, and what of it is not
    /// pushed yet.
    pub(crate) fn read_history(&mut self) {
        match crate::machines::union(&self.places, &self.settings.machine) {
            Ok(union) => {
                self.history = Ok(union.history);
                self.origin = union.origin;
                self.machines = union.machines;
            }
            Err(error) => self.history = Err(error),
        }
        self.unpushed = crate::machines::unpushed(&self.places, &self.mirror, &self.settings.machine).unwrap_or(0);
        self.modified.1 = modified(&self.places.runs());
    }

    /// Everything read from the mirror, again: the catalogue at `main`, the
    /// tags, the branches, the repository's runs, what the references of the
    /// queue resolve to, which commits are pushed.
    fn read_repository(&mut self) {
        if let Err(error) = crate::machines::refresh(&self.places, &self.mirror) {
            self.lock().messages.push(format!("machines: {error}"));
        }
        self.read_history();
        (self.catalogue, self.catalogue_source) = read_catalogue(&self.places, &self.mirror);
        self.tags = self.mirror.tags();
        self.references = self.tags.iter().map(|tag| tag.name.clone()).collect();
        self.references.extend(self.mirror.branches());
        if let Ok(catalogue) = &self.catalogue {
            (self.repository, self.repository_problems) = repository_runs(&self.mirror, catalogue);
        }
        self.resolutions.clear();
        self.reach.clear();
        self.reach_all();
    }

    /// Whether each commit the runs name is pushed, asked of the mirror once per commit.
    fn reach_all(&mut self) {
        if !self.mirror.exists() {
            return;
        }
        let commits: Vec<String> = self
            .history
            .as_ref()
            .map(|history| {
                history.runs.iter().filter_map(|run| run.git.as_ref().map(|git| git.commit.clone())).collect()
            })
            .unwrap_or_default();
        for commit in commits {
            if !self.reach.contains_key(&commit) {
                let pushed = self.mirror.pushed(&commit);
                self.reach.insert(commit, pushed);
            }
        }
    }

    /// What a target typed on an item resolves to, asked of the mirror once per text.
    pub(crate) fn resolution(&mut self, text: &str) -> Result<Resolution, String> {
        if let Some(known) = self.resolutions.get(text) {
            return known.clone();
        }
        let resolution = match Target::parse(text) {
            Target::Local(path) => GitState::read(&path).map(|git| Resolution {
                tag: self.tags.iter().find(|tag| tag.commit == git.commit).map(|tag| tag.name.clone()),
                pushed: self.mirror.pushed(&git.commit),
                dirty: git.dirty,
                checked_out: true,
                short: git.short,
                commit: git.commit,
            }),
            Target::Reference(reference) => self.mirror.resolve(&reference).map(|commit| Resolution {
                short: commit.chars().take(7).collect(),
                tag: self.tags.iter().find(|tag| tag.commit == commit).map(|tag| tag.name.clone()),
                pushed: self.mirror.pushed(&commit),
                dirty: false,
                checked_out: self.mirror.checkouts().iter().any(|checkout| checkout.head == commit),
                commit,
            }),
        };
        self.resolutions.insert(text.to_string(), resolution.clone());
        resolution
    }

    /// The runs lab recorded and those the repository's summaries record.
    fn all_runs(&self) -> Vec<Run> {
        let mut runs = self.history.as_ref().map(|history| history.runs.clone()).unwrap_or_default();
        runs.extend(self.repository.iter().cloned());
        runs
    }

    /// The names of the jobs whose timing matters, for the runs' rows.
    fn timed_jobs(&self) -> std::collections::BTreeSet<String> {
        self.catalogue
            .as_ref()
            .map(|catalogue| {
                catalogue
                    .jobs
                    .iter()
                    .chain(catalogue.batches.iter().map(|batch| &batch.job))
                    .filter(|job| job.timing_matters)
                    .map(|job| job.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The short hash of `main`, for the previews of the catalogue.
    fn commit(&mut self) -> String {
        self.resolution(MAIN).map(|resolution| resolution.short).unwrap_or_else(|_| "unknown".to_string())
    }

    fn set_threads(&mut self, threads: u32) {
        if threads != self.settings.threads {
            self.settings.threads = threads;
            if let Err(error) = self.settings.save(&self.places.settings()) {
                self.settings_error = Some(error);
            }
        }
    }
}

/// The catalogue, and where it comes from (SPEC.md, §2): the file given by
/// hand, else `lab/jobs.toml` at `main` of the mirror, else, while there is
/// no mirror, the clone lab was started in.
fn read_catalogue(places: &Places, mirror: &Mirror) -> (Result<Catalogue, String>, String) {
    if let Some(file) = &places.catalogue {
        return (Catalogue::read(file), file.display().to_string());
    }
    if mirror.exists() {
        let source = format!("lab/jobs.toml at {MAIN} of the mirror");
        let read = match mirror.show(MAIN, "lab/jobs.toml") {
            Some(text) => Catalogue::parse(&text).map_err(|error| format!("{source}: {error}")),
            None => Err(format!("{MAIN} of the mirror has no lab/jobs.toml")),
        };
        return (read, source);
    }
    match places.catalogue_file() {
        Some(file) => (Catalogue::read(&file), format!("{} (no mirror yet)", file.display())),
        None => (Err("no catalogue yet: the mirror is being created".to_string()), "none yet".to_string()),
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.look_again();
        ui.ctx().request_repaint_after(RELOAD);
        self.closing_window(ui.ctx());
        egui::Panel::top("top").show(ui, |ui| self.top_bar(ui));
        egui::CentralPanel::default().show(ui, |ui| {
            self.problems(ui);
            self.messages(ui);
            match self.tab {
                Tab::State => self.state_tab(ui),
                Tab::Queue => self.queue_tab(ui),
                Tab::History => self.history_tab(ui),
                Tab::Catalogue => self.catalogue_tab(ui),
                Tab::Machine => self.machine_tab(ui),
            }
        });
        if self.about {
            let mut open = true;
            egui::Window::new("About lab").open(&mut open).resizable(false).show(ui.ctx(), |ui| {
                ui.label("The record and the launcher of openclare's computations (lab/SPEC.md).");
                egui::Grid::new("places").num_columns(2).show(ui, |ui| {
                    let home = self.places.home.as_ref().map_or("none".to_string(), |home| home.display().to_string());
                    for (name, text) in [
                        ("Folder", self.places.data.display().to_string()),
                        ("Mirror", format!("{} ← {}", self.mirror.path.display(), self.mirror.url)),
                        ("Checkouts", self.mirror.checkouts.display().to_string()),
                        ("Catalogue", self.catalogue_source.clone()),
                        ("Runs", self.places.runs().display().to_string()),
                        (
                            "Machine",
                            format!(
                                "{} · branch {} · {} runs not pushed",
                                self.settings.machine,
                                crate::mirror::runs_branch(&self.settings.machine),
                                self.unpushed
                            ),
                        ),
                        ("Machines", crate::machines::folder(&self.places).display().to_string()),
                        ("Settings", self.places.settings().display().to_string()),
                        ("Started in", home),
                    ] {
                        ui.label(RichText::new(name).color(MUTED));
                        ui.monospace(text);
                        ui.end_row();
                    }
                });
            });
            self.about = open;
        }
        self.stop_dialog(ui.ctx());
        self.past_run_window(ui.ctx());
        self.take_screenshot(ui.ctx());
    }
}

impl App {
    /// Closing the window during a run asks first (SPEC.md, section 8.5):
    /// the record needs the app open.
    fn closing_window(&mut self, context: &egui::Context) {
        let running = self.lock().running();
        if self.quit_after_stop && !running {
            context.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        let closing = context.input(|input| input.viewport().close_requested());
        if closing && running && !self.quit_after_stop && self.screenshot.is_none() {
            context.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.closing = true;
        }
        if !self.closing {
            return;
        }
        let job =
            self.lock().current.as_ref().map(|current| current.job.clone()).unwrap_or_else(|| "a job".to_string());
        egui::Window::new("A run is under way").collapsible(false).resizable(false).show(context, |ui| {
            ui.label(format!("{job} is running. Its record needs the app open: closing stops it."));
            ui.horizontal(|ui| {
                if ui.button("Keep the window open").clicked() {
                    self.closing = false;
                }
                if ui.button(RichText::new("Stop the run and quit").color(RED)).clicked() {
                    self.lock().stop = Some(Policy::CancelTheRest);
                    self.quit_after_stop = true;
                    self.closing = false;
                }
            });
        });
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Shared> {
        self.shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// What the runner reports, until dismissed.
    fn messages(&mut self, ui: &mut egui::Ui) {
        let messages = self.lock().messages.clone();
        if messages.is_empty() {
            return;
        }
        egui::Frame::new().fill(Color32::from_rgb(46, 26, 28)).inner_margin(8).corner_radius(4).show(ui, |ui| {
            ui.set_width(ui.available_width());
            for message in &messages {
                ui.colored_label(RED, message);
            }
            if ui.small_button("Dismiss").clicked() {
                self.lock().messages.clear();
            }
        });
        ui.add_space(6.0);
    }
}

impl App {
    fn take_screenshot(&mut self, context: &egui::Context) {
        let Some(screenshot) = &mut self.screenshot else { return };
        context.request_repaint();
        screenshot.frames += 1;
        if screenshot.frames >= 10 && Instant::now() >= screenshot.at && !screenshot.asked {
            screenshot.asked = true;
            context.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
        }
        let image = context.input(|input| {
            input.raw.events.iter().find_map(|event| match event {
                egui::Event::Screenshot { image, .. } => Some(image.clone()),
                _ => None,
            })
        });
        if let Some(image) = image {
            let [width, height] = image.size;
            let mut bytes = format!("P6\n{width} {height}\n255\n").into_bytes();
            for pixel in &image.pixels {
                bytes.extend_from_slice(&[pixel.r(), pixel.g(), pixel.b()]);
            }
            if let Err(error) = std::fs::write(&screenshot.path, bytes) {
                eprintln!("lab: {}: {error}", screenshot.path.display());
            }
            context.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl App {
    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("lab").strong().size(18.0));
            ui.add_space(8.0);
            let (activity, fetched, pushed) = {
                let shared = self.lock();
                (shared.activity.clone(), shared.fetched.clone(), shared.pushed.clone())
            };
            match &activity {
                Some(activity) => {
                    ui.spinner();
                    ui.label(RichText::new(format!("mirror: {activity}…")).color(AMBER));
                }
                None if !self.mirror.exists() => {
                    ui.colored_label(RED, "no mirror: Fetch creates it");
                }
                None => {
                    let when = match &fetched {
                        Some((at, said)) => {
                            ui.label(RichText::new(format!("mirror: fetched {}", at.format("%H:%M"))).color(MUTED))
                                .on_hover_text(said);
                            None
                        }
                        None => self.mirror.last_fetch().map(chrono::DateTime::<chrono::Local>::from),
                    };
                    if let Some(at) = when {
                        let text = format!("mirror: fetched {}", at.format("%Y-%m-%d %H:%M"));
                        ui.label(RichText::new(text).color(MUTED)).on_hover_text(self.mirror.url.clone());
                    }
                }
            }
            if ui
                .add_enabled(activity.is_none(), egui::Button::new("Fetch"))
                .on_hover_text(format!("git fetch --prune from {}", self.mirror.url))
                .clicked()
            {
                self.lock().fetch_requested = true;
            }
            // The records not yet pushed to this machine's branch (SPEC.md, §4b).
            let branch = crate::mirror::runs_branch(&self.settings.machine);
            match self.settings.push {
                Push::Never => {
                    ui.label(RichText::new(format!("{} not pushed (setting: never)", self.unpushed)).color(MUTED))
                        .on_hover_text(format!("push = \"never\" in settings.toml: nothing goes to {branch}"));
                }
                policy => {
                    let hint = match &pushed {
                        Some((at, said)) => format!("to {branch}; last: {said} at {}", at.format("%H:%M")),
                        None => format!("git push to {branch}, the memory of this machine"),
                    };
                    let label = if policy == Push::Auto && self.unpushed == 0 {
                        "Push (auto)".to_string()
                    } else {
                        format!("Push ({})", self.unpushed)
                    };
                    if ui
                        .add_enabled(activity.is_none() && self.mirror.exists(), egui::Button::new(label))
                        .on_hover_text(hint)
                        .clicked()
                    {
                        self.lock().push_requested = true;
                    }
                }
            }
            ui.add_space(8.0);
            let verdict = crate::monitor::lock(&self.watch).verdict(&self.settings.calm);
            let (title, colour) = match verdict.calmness {
                Calmness::Measuring => ("Measuring", MUTED),
                Calmness::Calm => ("Calm machine", GREEN),
                Calmness::Ours => ("Busy with our run", AMBER),
                Calmness::Busy => ("Loaded by something else", RED),
            };
            // The first clause in the bar; the whole verdict on hover.
            let short = verdict.text.split(" · ").next().unwrap_or_default().to_string();
            let response = egui::Frame::new()
                .fill(colour.gamma_multiply(0.18))
                .stroke(egui::Stroke::new(1.0, colour.gamma_multiply(0.6)))
                .corner_radius(12)
                .inner_margin(egui::Margin::symmetric(10, 3))
                .show(ui, |ui| {
                    let text = if verdict.calmness == Calmness::Measuring {
                        short.clone()
                    } else {
                        format!("{title} · {short}")
                    };
                    ui.label(RichText::new(text).color(colour).strong())
                })
                .response;
            response.on_hover_text(&verdict.text);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("About").clicked() {
                    self.about = true;
                }
                let most = std::thread::available_parallelism().map_or(8, |count| count.get() as u32);
                let mut threads = self.settings.threads;
                ui.add(egui::DragValue::new(&mut threads).range(1..=most));
                self.set_threads(threads);
                ui.label("Threads");
                if self.settings.threads > self.settings.comfort_threads {
                    ui.colored_label(
                        AMBER,
                        format!("above {}: the machine will be slow to use", self.settings.comfort_threads),
                    );
                }
            });
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Tab::State, "State");
            ui.selectable_value(&mut self.tab, Tab::Queue, "Queue");
            ui.selectable_value(&mut self.tab, Tab::History, "History");
            ui.selectable_value(&mut self.tab, Tab::Catalogue, "Catalogue");
            ui.selectable_value(&mut self.tab, Tab::Machine, "Machine");
        });
        ui.add_space(4.0);
    }

    /// What could not be read, above every tab.
    fn problems(&self, ui: &mut egui::Ui) {
        let mut problems: Vec<String> = Vec::new();
        match &self.catalogue {
            Ok(catalogue) => problems.extend(catalogue.problems.iter().map(|problem| format!("jobs.toml, {problem}"))),
            Err(error) => problems.push(error.clone()),
        }
        match &self.history {
            Ok(history) => problems.extend(history.problems.iter().cloned()),
            Err(error) => problems.push(error.clone()),
        }
        problems.extend(self.settings_error.iter().cloned());
        problems.extend(self.repository_problems.iter().map(|problem| format!("repository summary, {problem}")));
        if problems.is_empty() {
            return;
        }
        egui::Frame::new().fill(Color32::from_rgb(46, 26, 28)).inner_margin(8).corner_radius(4).show(ui, |ui| {
            ui.set_width(ui.available_width());
            for problem in &problems {
                ui.colored_label(RED, problem);
            }
        });
        ui.add_space(6.0);
    }

    fn catalogue_tab(&mut self, ui: &mut egui::Ui) {
        let Ok(catalogue) = &self.catalogue else {
            ui.label("No catalogue: see the error above.");
            return;
        };
        let catalogue = catalogue.clone();
        egui::Panel::left("jobs").resizable(true).default_size(280.0).show(ui, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for category in Category::ALL {
                    let jobs: Vec<&Job> = catalogue.jobs.iter().filter(|job| job.category == category).collect();
                    if jobs.is_empty() {
                        continue;
                    }
                    ui.add_space(8.0);
                    ui.label(RichText::new(category.title().to_uppercase()).small().strong().color(MUTED));
                    for job in jobs {
                        let selected = self.job.as_deref() == Some(job.name.as_str());
                        ui.horizontal(|ui| {
                            if ui.selectable_label(selected, &job.name).clicked() {
                                self.job = Some(job.name.clone());
                            }
                            stopwatch(ui, job.timing_matters);
                        });
                    }
                }
                if !catalogue.batches.is_empty() {
                    ui.add_space(8.0);
                    ui.label(RichText::new("BATCHES").small().strong().color(MUTED));
                    for batch in &catalogue.batches {
                        let axes: Vec<String> =
                            batch.axes.iter().map(|axis| format!("{} ({})", axis.name, axis.values.len())).collect();
                        let jobs: usize = batch.axes.iter().map(|axis| axis.values.len()).product();
                        ui.horizontal(|ui| {
                            ui.label(&batch.job.name).on_hover_text(format!(
                                "{}\n\n{jobs} jobs: {}.\nRun it with `lab run {}`; the window does not queue a batch yet (R3).",
                                batch.job.description,
                                axes.join(" × "),
                                batch.job.name
                            ));
                            stopwatch(ui, batch.job.timing_matters);
                        });
                    }
                }
                ui.add_space(8.0);
                ui.label(RichText::new("SEQUENCES").small().strong().color(MUTED));
                for sequence in &catalogue.sequences {
                    ui.label(&sequence.name).on_hover_text(format!(
                        "{}\n\n{}",
                        sequence.description,
                        sequence.jobs.join(", ")
                    ));
                }
            });
        });
        egui::CentralPanel::default().show(ui, |ui| {
            let Some(job) = self.job.as_deref().and_then(|name| catalogue.job(name)) else {
                ui.label("Pick a job on the left.");
                return;
            };
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&job.name).strong().size(20.0));
                    stopwatch(ui, job.timing_matters);
                });
                ui.label(&job.description);
                ui.add_space(8.0);
                let threads = match job.threads {
                    ThreadUse::Setting => format!("the setting ({})", self.settings.threads),
                    ThreadUse::Fixed(count) => format!("{count}, fixed"),
                };
                let mut rows = vec![
                    ("Category", category_word(job.category).to_string()),
                    ("Checkout", job.checkout.text()),
                    ("Directory", job.directory.clone()),
                    ("Threads", threads),
                    ("Memory cap", job.memory_limit_gb.map_or("none".to_string(), |gb| format!("{gb} GB per process"))),
                    (
                        "Timing matters",
                        if job.timing_matters {
                            format!("{STOPWATCH} yes: it waits for a calm machine; a run under load reports no time")
                        } else {
                            "no: it runs under any load; its duration is not a measurement".to_string()
                        },
                    ),
                    ("Paper", job.paper.map_or("none".to_string(), |paper| paper.to_string())),
                    ("Tags", job.tags.join(", ")),
                    ("Outputs", job.outputs.join("  ")),
                ];
                for (name, value) in &job.parameters {
                    rows.push(("Parameter", format!("{name} = {value}")));
                }
                egui::Grid::new("job").num_columns(2).spacing([16.0, 4.0]).show(ui, |ui| {
                    for (name, value) in rows {
                        ui.label(RichText::new(name).color(MUTED));
                        ui.label(value);
                        ui.end_row();
                    }
                });
                ui.add_space(8.0);
                let commit = self.commit();
                ui.label(
                    RichText::new(format!(
                        "AS IT WOULD RUN, WITH {} THREADS, ON {} AT {commit}",
                        self.settings.threads,
                        job.checkout.initial()
                    ))
                    .small()
                    .strong()
                    .color(MUTED),
                );
                let values = job.values(self.settings.threads, &commit, &job.checkout.initial(), None, &BTreeMap::new());
                command_lines(ui, job, &values);
                match job.environment_for(&values) {
                    Ok(environment) => {
                        let text: Vec<String> =
                            environment.iter().map(|(name, value)| format!("{name}={value}")).collect();
                        ui.label(RichText::new(text.join(" ")).monospace().color(MUTED));
                    }
                    Err(error) => {
                        ui.colored_label(RED, error);
                    }
                }
                ui.add_space(12.0);
                ui.label(
                    RichText::new(format!(
                        "Read from {}. Edit jobs.toml by hand; a catalogue file is read again within two seconds, the mirror's at the next fetch. Editing here is not in M1.",
                        self.catalogue_source
                    ))
                    .color(MUTED),
                );
            });
        });
    }

    fn history_tab(&mut self, ui: &mut egui::Ui) {
        let runs: Vec<Run> = self.all_runs();
        if runs.is_empty() {
            ui.add_space(20.0);
            ui.label(format!(
                "No run recorded yet in {}, and none in the repository's summaries.",
                self.places.runs().display()
            ));
            return;
        }
        let mut jobs: Vec<String> = runs.iter().map(|run| run.job.clone()).collect();
        jobs.sort();
        jobs.dedup();
        ui.horizontal_wrapped(|ui| {
            ui.label("Job");
            egui::ComboBox::from_id_salt("job")
                .selected_text(self.filter.job.clone().unwrap_or_else(|| "all".to_string()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.filter.job, None, "all");
                    for job in &jobs {
                        ui.selectable_value(&mut self.filter.job, Some(job.clone()), job);
                    }
                });
            ui.label("Paper");
            egui::ComboBox::from_id_salt("paper")
                .selected_text(self.filter.paper.map_or("any".to_string(), |paper| paper.to_string()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.filter.paper, None, "any");
                    ui.selectable_value(&mut self.filter.paper, Some(Paper::A), "A");
                    ui.selectable_value(&mut self.filter.paper, Some(Paper::B), "B");
                });
            ui.label("Commit");
            ui.add(egui::TextEdit::singleline(&mut self.filter.commit).desired_width(80.0));
            ui.label("Text");
            ui.add(
                egui::TextEdit::singleline(&mut self.filter.text)
                    .hint_text("comment, command, label")
                    .desired_width(180.0),
            );
            ui.checkbox(&mut self.filter.valid_only, "timing-valid only");
        });
        let text = self.filter.text.to_lowercase();
        let mut shown: Vec<&Run> = runs
            .iter()
            .filter(|run| self.filter.job.as_ref().is_none_or(|job| &run.job == job))
            .filter(|run| {
                self.filter.paper.is_none_or(|paper| run.papers.iter().any(|reference| reference.paper == paper))
            })
            .filter(|run| {
                self.filter.commit.is_empty()
                    || run.git.as_ref().is_some_and(|git| git.commit.starts_with(self.filter.commit.trim()))
            })
            .filter(|run| !self.filter.valid_only || run.calm.timing_valid)
            .filter(|run| {
                text.is_empty()
                    || run.comment.to_lowercase().contains(&text)
                    || run.command.to_lowercase().contains(&text)
                    || run.papers_text().to_lowercase().contains(&text)
            })
            .collect();
        let (by, ascending) = self.sort;
        shown.sort_by(|a, b| {
            let order = match by {
                SortBy::Started => a.started.cmp(&b.started),
                SortBy::Job => a.job.cmp(&b.job),
                SortBy::Duration => a.seconds.unwrap_or(-1.0).total_cmp(&b.seconds.unwrap_or(-1.0)),
                SortBy::Commit => a.short_commit().cmp(&b.short_commit()),
                SortBy::Status => a.status.text().cmp(&b.status.text()),
            };
            if ascending { order } else { order.reverse() }
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{} of {} runs", shown.len(), runs.len())).color(MUTED));
            self.history_tools(ui, &shown);
        });

        let timed = self.timed_jobs();
        let selected = self.run.clone();
        let detail = selected.as_ref().and_then(|id| runs.iter().find(|run| &run.id == id)).cloned();
        if let Some(run) = &detail {
            egui::Panel::right("run").resizable(true).default_size(460.0).show(ui, |ui| {
                egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    let origin = self.origin.get(&run.id).cloned();
                    run_detail(ui, run, &self.places.data, timed.contains(&run.job), origin.as_deref());
                    self.edit_panel(ui, run);
                });
            });
        }
        let tags = self.tags.clone();
        let reach = self.reach.clone();
        let mut sort = self.sort;
        let mut clicked = None;
        egui::CentralPanel::default().show(ui, |ui| {
            let header = |ui: &mut egui::Ui, sort: &mut (SortBy, bool), name: &str, by: Option<SortBy>| {
                let Some(by) = by else {
                    ui.strong(name);
                    return;
                };
                let mark = if sort.0 == by { if sort.1 { " (up)" } else { " (down)" } } else { "" };
                if ui.add(egui::Button::new(RichText::new(format!("{name}{mark}")).strong()).frame(false)).clicked() {
                    *sort = (by, if sort.0 == by { !sort.1 } else { by != SortBy::Started && by != SortBy::Duration });
                }
            };
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .sense(egui::Sense::click())
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().at_least(120.0))
                .column(Column::auto().at_least(140.0))
                .column(Column::auto().at_least(90.0))
                .column(Column::auto().at_least(80.0))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto().at_least(70.0))
                .column(Column::auto().at_least(120.0))
                .column(Column::remainder())
                .header(22.0, |mut row| {
                    for (name, by) in [
                        ("Started", Some(SortBy::Started)),
                        ("Job", Some(SortBy::Job)),
                        ("Duration", Some(SortBy::Duration)),
                        ("Run on", Some(SortBy::Commit)),
                        ("Thr", None),
                        ("Calm", None),
                        ("Status", Some(SortBy::Status)),
                        ("Papers", None),
                        ("Comment", None),
                    ] {
                        row.col(|ui| header(ui, &mut sort, name, by));
                    }
                })
                .body(|body| {
                    body.rows(20.0, shown.len(), |mut row| {
                        let run = shown[row.index()];
                        row.set_selected(selected.as_deref() == Some(run.id.as_str()));
                        let seeded = !matches!(run.source, Source::App);
                        row.col(|ui| {
                            ui.monospace(run.started.format("%Y-%m-%d %H:%M").to_string());
                        });
                        row.col(|ui| {
                            ui.label(&run.job);
                            stopwatch(ui, timed.contains(&run.job));
                            if seeded {
                                ui.label(RichText::new(source_word(&run.source)).small().color(MUTED));
                            }
                        });
                        row.col(|ui| {
                            duration_cell(ui, run, timed.contains(&run.job));
                        });
                        row.col(|ui| {
                            let place = place(run, &tags, &reach);
                            let (colour, hint) = place_style(&place);
                            ui.colored_label(colour, RichText::new(place).monospace()).on_hover_text(hint);
                        });
                        row.col(|ui| {
                            ui.monospace(run.threads.map_or("?".to_string(), |threads| threads.effective.to_string()));
                        });
                        row.col(|ui| {
                            let (mark, colour) = calm_mark(run);
                            ui.colored_label(colour, mark).on_hover_text(&run.calm.reason);
                        });
                        row.col(|ui| {
                            ui.label(run.status.text());
                        });
                        row.col(|ui| {
                            ui.label(run.papers_text());
                        });
                        row.col(|ui| {
                            ui.label(&run.comment);
                        });
                        if row.response().clicked() {
                            clicked = Some(run.id.clone());
                        }
                    });
                });
        });
        self.sort = sort;
        if let Some(id) = clicked {
            self.run = if self.run.as_ref() == Some(&id) { None } else { Some(id) };
        }
    }
}

/// The command and the prepare of a job with these values, or why not.
fn command_lines(ui: &mut egui::Ui, job: &Job, values: &BTreeMap<String, String>) {
    if let Some(prepare) = &job.prepare {
        match render(prepare, values) {
            Ok(text) => ui.label(RichText::new(format!("prepare: {text}")).monospace().color(MUTED)),
            Err(error) => ui.colored_label(RED, error),
        };
    }
    match render(&job.command, values) {
        Ok(text) => ui.label(RichText::new(text).monospace()),
        Err(error) => ui.colored_label(RED, error),
    };
}

/// The colour of a run's place, and why it matters (SPEC.md, §10b).
pub(crate) fn place_style(place: &str) -> (Color32, &'static str) {
    if place.starts_with("orphan") {
        (AMBER, "its commit is reachable from no branch nor tag of the remote: nobody can check out what ran")
    } else if place.starts_with("off-tag") {
        (BLUE, "a run on a tag can back a paper; an off-tag run cannot")
    } else if place.starts_with("no hash") {
        (MUTED, "the record names no commit")
    } else {
        (GREEN, "on a tag: it can back a paper")
    }
}

fn category_word(category: Category) -> &'static str {
    match category {
        Category::Build => "build",
        Category::TestSuite => "test suite",
        Category::QuickBench => "quick bench",
        Category::Bench => "bench",
        Category::LeanBuild => "Lean build",
        Category::LeanOracle => "Lean oracle",
        Category::Experiment => "experiment",
    }
}

fn source_word(source: &Source) -> &'static str {
    match source {
        Source::App => "",
        Source::Seeded { .. } => "repository",
        Source::Manual => "typed",
    }
}

/// "yes" timing valid, "forced", "no" not calm, "?" not measured.
fn calm_mark(run: &Run) -> (&'static str, Color32) {
    if run.calm.timing_valid {
        ("yes", GREEN)
    } else if run.calm.forced {
        ("forced", AMBER)
    } else if !matches!(run.source, Source::App) {
        ("?", MUTED)
    } else {
        ("no", RED)
    }
}

fn run_detail(ui: &mut egui::Ui, run: &Run, data: &Path, timing_matters: bool, origin: Option<&str>) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(&run.job).strong().size(18.0));
        stopwatch(ui, timing_matters);
        let (mark, colour) = calm_mark(run);
        ui.colored_label(
            colour,
            format!("{} · {}", run.status.text(), if run.calm.timing_valid { "timing valid" } else { mark }),
        );
    });
    ui.label(RichText::new(&run.id).monospace().color(MUTED));
    ui.add_space(6.0);
    ui.label(RichText::new(&run.command).monospace());
    ui.add_space(6.0);
    let gigabytes = |bytes: u64| format!("{:.1} GiB", bytes as f64 / (1u64 << 30) as f64);
    let mut rows: Vec<(&str, String)> = Vec::new();
    let ended = run.ended.map(|ended| format!(" to {}", ended.format("%Y-%m-%d %H:%M:%S"))).unwrap_or_default();
    let seconds = run.seconds.map(|seconds| format!(" · {}", duration_text(seconds))).unwrap_or_default();
    rows.push(("When", format!("{}{ended}{seconds}", run.started.format("%Y-%m-%d %H:%M:%S"))));
    if let Source::Seeded { from } = &run.source {
        rows.push(("Read from", format!("{from} (the date is that of the commit that brought it)")));
    }
    if !run.environment.is_empty() {
        let text: Vec<String> = run.environment.iter().map(|(name, value)| format!("{name}={value}")).collect();
        rows.push(("Environment", text.join(" ")));
    }
    for (name, value) in &run.parameters {
        rows.push(("Parameter", format!("{name} = {value}")));
    }
    let memory = run.memory_limit_gb.map(|gb| format!(" · memory cap {gb} GB")).unwrap_or_default();
    rows.push(("Checkout", format!("{} · directory {}{memory}", run.checkout, run.directory)));
    if let Some(origin) = origin {
        rows.push(("Recorded by", format!("{origin} (its branch {})", crate::mirror::runs_branch(origin))));
    }
    if let Some(git) = &run.git {
        let branch = git.branch.clone().unwrap_or_default();
        let pushed = if git.pushed { "pushed" } else { "not pushed when it ran" };
        rows.push((
            "Git",
            format!("{} {} {branch} · {pushed}", git.commit, if git.dirty { "modified" } else { "clean" }),
        ));
    }
    if let Some(threads) = run.threads {
        rows.push(("Threads", format!("{} (setting {})", threads.effective, threads.setting)));
    }
    if let Some(machine) = &run.machine {
        let gpu = machine
            .gpu
            .as_ref()
            .map(|gpu| format!(" · {} {}", gpu.name, gigabytes(gpu.memory_bytes)))
            .unwrap_or_default();
        // A summary of the repository records no host nor cores: say only what is known.
        let mut parts: Vec<String> = Vec::new();
        if !machine.hostname.is_empty() {
            parts.push(machine.hostname.clone());
        }
        parts.push(machine.cpu.clone());
        match machine.cores {
            0 => parts.push(format!("{} threads", machine.threads)),
            cores => parts.push(format!("{cores} cores, {} threads", machine.threads)),
        }
        parts.push(format!("{}{gpu}", gigabytes(machine.memory_bytes)));
        rows.push(("Machine", parts.join(" · ")));
    }
    if let Some(load) = &run.load {
        let sample = |sample: &crate::model::Sample| {
            let mut text = format!("load {:.2} {:.2} {:.2}", sample.load[0], sample.load[1], sample.load[2]);
            if let Some(cpu) = sample.cpu_percent {
                text.push_str(&format!(" · CPU {cpu:.1} %"));
            }
            if let Some(foreign) = sample.foreign_cpu_percent {
                text.push_str(&format!(" · foreign {foreign:.1} %"));
            }
            if let Some(memory) = sample.memory_used_bytes {
                text.push_str(&format!(" · RAM {}", gigabytes(memory)));
            }
            text
        };
        rows.push(("At start", sample(&load.at_start)));
        if let Some(during) = &load.during {
            rows.push((
                "During",
                format!(
                    "foreign CPU mean {:.1} %, peak {:.1} % · load1 max {:.2} · RAM peak {}",
                    during.foreign_cpu_mean,
                    during.foreign_cpu_peak,
                    during.load1_max,
                    gigabytes(during.memory_used_peak)
                ),
            ));
        }
        if let Some(end) = &load.at_end {
            rows.push(("At end", sample(end)));
        }
    }
    let calm = &run.calm;
    let yes = |value: bool| if value { "yes" } else { "no" };
    let mut verdict = format!(
        "before {} · during {} · {}",
        yes(calm.before),
        yes(calm.during),
        if calm.forced { "forced" } else { "not forced" }
    );
    if !calm.reason.is_empty() {
        verdict.push_str(&format!(" · {}", calm.reason));
    }
    rows.push(("Calm", verdict));
    for output in &run.outputs {
        let hash = output
            .sha256
            .as_ref()
            .map(|hash| format!(" · {}…{}", &hash[..4.min(hash.len())], &hash[hash.len().saturating_sub(4)..]))
            .unwrap_or_default();
        rows.push(("Output", format!("{} · {} bytes{hash}", output.path, output.bytes)));
    }
    if let Some(log) = &run.log {
        rows.push(("Log", format!("{} · {} bytes", log.path, log.bytes)));
    }
    rows.push(("Papers", run.papers_text()));
    rows.push(("Comment", run.comment.clone()));
    egui::Grid::new("run").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
        for (name, value) in rows {
            ui.label(RichText::new(name).color(MUTED));
            ui.add(egui::Label::new(value).wrap());
            ui.end_row();
        }
    });
    ui.add_space(8.0);
    if let Some(log) = &run.log
        && ui.button("Open the log").clicked()
    {
        let _ = std::process::Command::new("xdg-open").arg(data.join(&log.path)).spawn();
    }
}
