//! The Batches screen (SPEC.md, §6b, amendment R3): the batches of the
//! catalogue, and for the one chosen its grid, one line per job (a cell:
//! one value of each axis), numbered in the grid's order. A cell shows the
//! batch under way's state, else its last record; cells are selected and
//! the batch is queued restricted to them.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use eframe::egui::{self, RichText};

use super::{AMBER, App, BLUE, GREEN, MUTED, RED, Tab, stopwatch};
use crate::catalogue::{Batch, CellPlan, Timeout};
use crate::files;
use crate::model::{Run, Status, duration_text};
use crate::runner::{BatchChoice, LiveGrid, LiveState};

/// The lines of a cell's log shown under the grid.
const LOG_LINES: usize = 40;

/// What the Batches screen remembers between frames.
#[derive(Default)]
pub(super) struct BatchView {
    pub batch: Option<String>,
    /// The cells selected, each its axes' values in order.
    selected: BTreeSet<Vec<(String, String)>>,
    /// The cell clicked last, shown under the grid.
    clicked: Option<Vec<(String, String)>>,
    timeout: String,
    again: bool,
    /// The report asked (R4): its table when made, or what went wrong.
    report: Option<ReportSlot>,
}

/// Where the thread that makes a report puts its table, or what went wrong.
type ReportSlot = Arc<Mutex<Option<Result<String, String>>>>;

/// What a cell of the grid shows.
struct Shown {
    text: String,
    colour: egui::Color32,
    hover: String,
    failed: bool,
    never: bool,
}

impl BatchView {
    /// The screen showing that batch's grid, nothing selected.
    pub(super) fn showing(batch: Option<String>) -> BatchView {
        BatchView { batch, ..BatchView::default() }
    }
}

impl App {
    pub(super) fn batches_tab(&mut self, ui: &mut egui::Ui) {
        let Ok(catalogue) = &self.catalogue else {
            ui.label("No catalogue: see the error above.");
            return;
        };
        let catalogue = catalogue.clone();
        egui::Panel::left("batches").resizable(true).default_size(260.0).show(ui, |ui| {
            ui.label(RichText::new("BATCHES").small().strong().color(MUTED));
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for batch in &catalogue.batches {
                    let chosen = self.batches.batch.as_deref() == Some(batch.job.name.as_str());
                    let jobs: usize = batch.axes.iter().map(|axis| axis.values.len()).product();
                    ui.horizontal(|ui| {
                        let label = ui.selectable_label(chosen, &batch.job.name).on_hover_text(&batch.job.description);
                        if label.clicked() && !chosen {
                            self.batches = BatchView { batch: Some(batch.job.name.clone()), ..BatchView::default() };
                        }
                        ui.label(RichText::new(format!("{jobs}")).color(MUTED));
                        stopwatch(ui, batch.job.timing_matters);
                    });
                }
            });
        });
        egui::CentralPanel::default().show(ui, |ui| {
            let Some(batch) = self.batches.batch.as_deref().and_then(|name| catalogue.batch(name)).cloned() else {
                ui.label(RichText::new("Choose a batch on the left: its grid shows here.").color(MUTED));
                return;
            };
            egui::ScrollArea::vertical().id_salt("batch page").auto_shrink([false; 2]).show(ui, |ui| {
                self.grid(ui, &catalogue, &batch);
            });
        });
    }

    fn grid(&mut self, ui: &mut egui::Ui, catalogue: &crate::catalogue::Catalogue, batch: &Batch) {
        let cells = match batch.expand(&[]) {
            Ok(cells) => cells,
            Err(error) => {
                ui.colored_label(RED, error);
                return;
            }
        };
        let live: Option<LiveGrid> = self.lock().grid.clone().filter(|grid| grid.batch == batch.job.name);
        let runs = self.all_runs();
        let shown: Vec<Shown> = cells.iter().map(|cell| self.shown(batch, cell, &runs, live.as_ref())).collect();

        // The header: what the batch is, then the selection and its launch.
        ui.horizontal(|ui| {
            ui.label(RichText::new(&batch.job.name).strong().size(18.0));
            stopwatch(ui, batch.job.timing_matters);
            let axes: Vec<String> =
                batch.axes.iter().map(|axis| format!("{} ({})", axis.name, axis.values.len())).collect();
            ui.label(RichText::new(format!("{} = {} jobs", axes.join(" × "), cells.len())).color(MUTED));
            if !batch.needs.is_empty() {
                ui.label(RichText::new(format!("needs {}", batch.needs.join(", "))).color(MUTED));
            }
        });
        ui.label(RichText::new(&batch.job.description).color(MUTED));
        self.dependency_table(ui, catalogue, batch, &cells, &runs);
        if let Some(grid) = &live {
            let remaining: Vec<Option<f64>> = grid
                .cells
                .iter()
                .filter_map(|cell| match cell.state {
                    LiveState::Waiting => Some(cell.estimate),
                    LiveState::Running(since) => {
                        Some(cell.estimate.map(|seconds| (seconds - since.elapsed().as_secs_f64()).max(0.0)))
                    }
                    _ => None,
                })
                .collect();
            let known: Vec<f64> = remaining.iter().flatten().copied().collect();
            let left = match (known.len(), remaining.len() - known.len()) {
                (0, _) => "no estimate yet".to_string(),
                (_, 0) => {
                    format!("about {} left", duration_text(known.iter().sum::<f64>() / f64::from(grid.width.max(1))))
                }
                (_, unknown) => format!(
                    "at least {} left, plus {unknown} jobs without an estimate",
                    duration_text(known.iter().sum::<f64>() / f64::from(grid.width.max(1)))
                ),
            };
            let done = grid
                .cells
                .iter()
                .filter(|cell| matches!(cell.state, LiveState::Ended { .. } | LiveState::Skipped))
                .count();
            let text = if grid.ended {
                format!("last launch {}: {done}/{} jobs", grid.launch, grid.cells.len())
            } else {
                format!("under way, {done}/{} jobs, {} at a time · {left}", grid.cells.len(), grid.width)
            };
            ui.colored_label(if grid.ended { MUTED } else { AMBER }, text);
            if !grid.ended {
                if !grid.phase.is_empty() {
                    ui.label(RichText::new(format!("now: {}", grid.phase)).strong());
                }
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
            }
            // The dependencies of the launch, made or found in the cache, before its jobs (R2).
            if !grid.dependencies.is_empty() {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("DEPENDENCIES").small().strong().color(MUTED));
                    for (text, state) in &grid.dependencies {
                        let (said, colour) = match state {
                            LiveState::Skipped => ("cached".to_string(), GREEN),
                            LiveState::Running(since) => {
                                (format!("… {}", duration_text(since.elapsed().as_secs_f64())), BLUE)
                            }
                            LiveState::Ended { status: Status::Ok } => ("made".to_string(), GREEN),
                            LiveState::Ended { status } => (status.text(), RED),
                            LiveState::Waiting => ("waiting".to_string(), MUTED),
                        };
                        ui.label(RichText::new(format!("{text}: {said}")).monospace().color(colour));
                    }
                });
            }
        }
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("SELECT").small().strong().color(MUTED));
            if ui.small_button("all").clicked() {
                self.batches.selected = cells.iter().map(|cell| cell.values.clone()).collect();
            }
            if ui.small_button("none").clicked() {
                self.batches.selected.clear();
            }
            if ui.small_button("failed").on_hover_text("failed or timed out, at their last record").clicked() {
                self.batches.selected = cells
                    .iter()
                    .zip(&shown)
                    .filter(|(_, shown)| shown.failed)
                    .map(|(cell, _)| cell.values.clone())
                    .collect();
            }
            if ui.small_button("never run").clicked() {
                self.batches.selected = cells
                    .iter()
                    .zip(&shown)
                    .filter(|(_, shown)| shown.never)
                    .map(|(cell, _)| cell.values.clone())
                    .collect();
            }
            ui.separator();
            ui.label("Timeout");
            let default = batch.job.timeout.map(|timeout| duration_text(timeout.seconds)).unwrap_or_default();
            ui.add(egui::TextEdit::singleline(&mut self.batches.timeout).hint_text(default).desired_width(70.0))
                .on_hover_text("for this launch: 90s, 30m, 2h; empty, the batch's");
            ui.checkbox(&mut self.batches.again, "again").on_hover_text(
                "run again the jobs done before (same command, commit, tools and dependencies, ended ok); \
                 without it they are skipped",
            );
            let timeout = match self.batches.timeout.trim() {
                "" => Ok(None),
                text => Timeout::parse(text).map(Some),
            };
            let count = self.batches.selected.len();
            let label = if count == 0 {
                format!("Queue the batch ({})", cells.len())
            } else {
                format!("Queue the selection ({count})")
            };
            let queue = ui.add_enabled(timeout.is_ok(), egui::Button::new(RichText::new(label).strong()));
            if let Err(error) = &timeout {
                ui.colored_label(RED, error);
            }
            if queue.clicked()
                && let Ok(timeout) = timeout
            {
                let chosen: Vec<BTreeMap<String, String>> =
                    self.batches.selected.iter().map(|values| values.iter().cloned().collect()).collect();
                let choice = BatchChoice {
                    catalogue: Arc::new(catalogue.clone()),
                    cells: chosen,
                    timeout,
                    again: self.batches.again,
                };
                self.lock().add_batch(batch, choice);
                self.batches.selected.clear();
                self.tab = Tab::Queue;
            }
            if batch.report.is_some() {
                ui.separator();
                let making = self
                    .batches
                    .report
                    .as_ref()
                    .is_some_and(|report| report.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).is_none());
                let button = ui
                    .add_enabled(!making, egui::Button::new(if making { "Report…" } else { "Report" }))
                    .on_hover_text(format!(
                        "the table from the last record of each cell, nothing computed again: {}",
                        batch.report.as_deref().unwrap_or_default()
                    ));
                if button.clicked() {
                    let slot = Arc::new(Mutex::new(None));
                    self.batches.report = Some(slot.clone());
                    let (places, mirror, batch) = (self.places.clone(), self.mirror.clone(), batch.clone());
                    let (threads, context) = (self.settings.threads, ui.ctx().clone());
                    std::thread::spawn(move || {
                        let made = crate::report::make(&places, &mirror, &batch, None, None, threads)
                            .map(|made| crate::report::table(&made));
                        *slot.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(made);
                        context.request_repaint();
                    });
                }
            }
        });
        let report = self
            .batches
            .report
            .as_ref()
            .and_then(|report| report.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone());
        if let Some(report) = report {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(RichText::new("REPORT").small().strong().color(MUTED));
                if ui.small_button("close").clicked() {
                    self.batches.report = None;
                }
            });
            match report {
                Ok(table) => {
                    egui::ScrollArea::both().id_salt("report").max_height(220.0).show(ui, |ui| {
                        ui.label(RichText::new(table).monospace().size(11.5));
                    });
                }
                Err(error) => {
                    ui.colored_label(RED, error);
                }
            }
        }
        ui.separator();

        // One line per job, in the grid's order: its number, its axes' values, its state.
        egui::ScrollArea::horizontal().id_salt("cells").auto_shrink([false, true]).show(ui, |ui| {
            egui::Grid::new("cells").striped(true).spacing([14.0, 3.0]).show(ui, |ui| {
                ui.label(RichText::new("#").small().strong().color(MUTED));
                for axis in &batch.axes {
                    ui.label(RichText::new(&axis.name).small().strong().color(MUTED));
                }
                ui.label(RichText::new("state").small().strong().color(MUTED));
                ui.label(RichText::new("last run").small().strong().color(MUTED));
                ui.end_row();
                for (index, (cell, shown)) in cells.iter().zip(&shown).enumerate() {
                    let selected = self.batches.selected.contains(&cell.values);
                    let mut clicked = ui
                        .selectable_label(selected, RichText::new(format!("{}", index + 1)).monospace().color(MUTED))
                        .clicked();
                    for (_, value) in &cell.values {
                        clicked |= ui.selectable_label(selected, RichText::new(value).strong()).clicked();
                    }
                    let text = RichText::new(&shown.text).monospace().color(shown.colour);
                    clicked |= ui.selectable_label(selected, text).on_hover_text(&shown.hover).clicked();
                    let last = last_run(batch, &cell.values, &runs).map_or(String::new(), |run| run.id.clone());
                    clicked |=
                        ui.selectable_label(selected, RichText::new(last).monospace().small().color(MUTED)).clicked();
                    if clicked {
                        if selected {
                            self.batches.selected.remove(&cell.values);
                        } else {
                            self.batches.selected.insert(cell.values.clone());
                        }
                        self.batches.clicked = Some(cell.values.clone());
                    }
                    ui.end_row();
                }
            });
        });

        // The cell clicked last: its record and the end of its log.
        let Some(values) = self.batches.clicked.clone() else { return };
        ui.separator();
        let text: Vec<String> = values.iter().map(|(axis, value)| format!("{axis}={value}")).collect();
        ui.label(RichText::new(text.join(" ")).strong());
        let Some(run) = last_run(batch, &values, &runs).cloned() else {
            ui.label(RichText::new("never run").color(MUTED));
            return;
        };
        ui.horizontal(|ui| {
            ui.label(RichText::new(&run.id).monospace());
            ui.label(format!(
                "{} · {}",
                run.status.text(),
                run.seconds.map(duration_text).unwrap_or_else(|| "no time".to_string())
            ));
            if let Some(git) = &run.git {
                ui.label(
                    RichText::new(format!("{} · {}", run.checkout, &git.commit[..7.min(git.commit.len())]))
                        .color(MUTED),
                );
            }
            if ui.small_button("Open in History").clicked() {
                self.run = Some(run.id.clone());
                self.tab = Tab::History;
            }
        });
        if let Some(log) = &run.log {
            let lines = files::tail(&self.places.data.join(&log.path), LOG_LINES);
            egui::ScrollArea::vertical().id_salt("cell log").max_height(320.0).stick_to_bottom(true).show(ui, |ui| {
                for line in lines {
                    ui.label(RichText::new(line).monospace().size(11.5));
                }
            });
        }
    }

    /// The jobs the batch needs before its own (R2), one row per dependency
    /// and value it wants, one column per reference: each one's last record
    /// and whether its result is in the cache.
    fn dependency_table(
        &mut self,
        ui: &mut egui::Ui,
        catalogue: &crate::catalogue::Catalogue,
        batch: &Batch,
        cells: &[CellPlan],
        runs: &[Run],
    ) {
        let rows = dependency_rows(catalogue, batch, cells);
        if rows.is_empty() {
            return;
        }
        let references: Vec<String> =
            rows.iter().flat_map(|row| row.references.iter().cloned()).fold(Vec::new(), |mut kept, reference| {
                if !kept.contains(&reference) {
                    kept.push(reference);
                }
                kept
            });
        let jobs: usize = rows.iter().map(|row| row.references.len()).sum();
        egui::CollapsingHeader::new(
            RichText::new(format!("DEPENDENCIES: {jobs} jobs, made before the grid, once per key"))
                .small()
                .strong()
                .color(MUTED),
        )
        .id_salt(("dependencies", &batch.job.name))
        .default_open(true)
        .show(ui, |ui| {
            egui::ScrollArea::horizontal().id_salt("dependencies").show(ui, |ui| {
                egui::Grid::new("dependencies").striped(true).spacing([6.0, 2.0]).show(ui, |ui| {
                    ui.label("");
                    for reference in &references {
                        ui.label(RichText::new(reference).small().strong().color(MUTED));
                    }
                    ui.end_row();
                    for row in &rows {
                        ui.label(RichText::new(&row.text).strong());
                        for reference in &references {
                            if !row.references.contains(reference) {
                                ui.label("");
                                continue;
                            }
                            let last = runs
                                .iter()
                                .filter(|run| {
                                    run.job == row.name
                                        && &run.checkout == reference
                                        && row
                                            .wanted
                                            .iter()
                                            .all(|(name, value)| run.parameters.get(name) == Some(value))
                                })
                                .max_by_key(|run| run.started);
                            let (text, colour, hover) = self.dependency_shown(&row.name, last);
                            let response = ui.selectable_label(false, RichText::new(text).monospace().color(colour));
                            if response.on_hover_text(hover).clicked()
                                && let Some(run) = last
                            {
                                self.run = Some(run.id.clone());
                                self.tab = Tab::History;
                            }
                        }
                        ui.end_row();
                    }
                });
            });
        });
    }

    /// A dependency's cell: its last record, and whether its result is in the cache.
    fn dependency_shown(&self, name: &str, last: Option<&Run>) -> (String, egui::Color32, String) {
        let Some(run) = last else {
            return ("—".to_string(), MUTED, "never made: made at the launch".to_string());
        };
        let time = run.seconds.map(duration_text).unwrap_or_default();
        let commit = run.git.as_ref().map_or(String::new(), |git| git.commit[..7.min(git.commit.len())].to_string());
        let cached = run.status == Status::Ok
            && run.cache.as_ref().is_some_and(|key| crate::batch::cache_folder(&self.places, name, key).is_dir());
        let hover = format!(
            "last run {} · {} · {time} · commit {commit}\nkey {}\n{}\nclick: open in History",
            run.id,
            run.status.text(),
            run.cache.as_deref().unwrap_or("none"),
            if cached {
                "in the cache: not made again while the key is the same (same files at the commit, same tools)"
            } else {
                "not in the cache: made at the launch"
            }
        );
        let (text, colour) = match &run.status {
            Status::Ok if cached => (format!("cached {time}"), GREEN),
            Status::Ok => (format!("ok {time}, not cached"), AMBER),
            Status::TimedOut => ("timed out".to_string(), RED),
            Status::Failed { .. } => ("failed".to_string(), RED),
            other => (other.text(), AMBER),
        };
        (text, colour, hover)
    }

    /// The state of one cell: the batch under way's, else its last record's.
    fn shown(&self, batch: &Batch, cell: &CellPlan, runs: &[Run], live: Option<&LiveGrid>) -> Shown {
        let estimate = cell_estimate(batch, &cell.values, runs);
        let estimate_text = estimate.map(|seconds| format!("≈ {}", duration_text(seconds))).unwrap_or_default();
        let last = last_run(batch, &cell.values, runs);
        let last_text = last.map(|run| {
            format!(
                "last run {} · {} · {}{}",
                run.id,
                run.status.text(),
                run.seconds.map(duration_text).unwrap_or_else(|| "no time".to_string()),
                if run.calm.timing_valid {
                    String::new()
                } else {
                    format!(" (time not counting: {})", run.calm.reason)
                }
            )
        });
        let hover = [last_text.unwrap_or_else(|| "never run".to_string()), estimate_text.clone()]
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        let failed = last.is_some_and(|run| matches!(run.status, Status::Failed { .. } | Status::TimedOut));
        let never = last.is_none();
        if let Some(live_cell) = live.and_then(|grid| grid.cells.iter().find(|live| live.values == cell.values)) {
            let (text, colour) = match &live_cell.state {
                LiveState::Waiting if !live.is_some_and(|grid| grid.ended) => {
                    (format!("waiting {estimate_text}"), MUTED)
                }
                LiveState::Running(since) => (format!("… {}", duration_text(since.elapsed().as_secs_f64())), BLUE),
                LiveState::Skipped => {
                    let time = last.and_then(|run| run.seconds).map(duration_text).unwrap_or_default();
                    (format!("done {time}"), GREEN)
                }
                _ => return record_shown(last, hover, failed, never),
            };
            return Shown { text, colour, hover, failed, never };
        }
        record_shown(last, hover, failed, never)
    }
}

/// One row of the dependencies' table: a dependency and the values it wants
/// (the model of a source), and the references of the cells that need it.
struct DependencyRow {
    name: String,
    wanted: Vec<(String, String)>,
    text: String,
    references: Vec<String>,
}

/// The dependencies' rows of a batch, in the order of `needs` then of the grid.
fn dependency_rows(catalogue: &crate::catalogue::Catalogue, batch: &Batch, cells: &[CellPlan]) -> Vec<DependencyRow> {
    let fallback = batch.job.checkout.initial();
    let mut rows: Vec<DependencyRow> = Vec::new();
    for name in &batch.needs {
        let Some(dependency) = catalogue.dependency(name) else { continue };
        let wanted_names = dependency.wanted();
        for cell in cells {
            let wanted: Vec<(String, String)> = wanted_names
                .iter()
                .filter_map(|wanted| cell.parameters.get(wanted).map(|value| (wanted.clone(), value.clone())))
                .collect();
            let reference = cell.reference.clone().unwrap_or_else(|| fallback.clone());
            match rows.iter_mut().find(|row| &row.name == name && row.wanted == wanted) {
                Some(row) => {
                    if !row.references.contains(&reference) {
                        row.references.push(reference);
                    }
                }
                None => {
                    let mut text = name.clone();
                    for (_, value) in &wanted {
                        text.push_str(&format!(" {value}"));
                    }
                    rows.push(DependencyRow { name: name.clone(), wanted, text, references: vec![reference] });
                }
            }
        }
    }
    rows
}

/// A cell as its last record says.
fn record_shown(last: Option<&Run>, hover: String, failed: bool, never: bool) -> Shown {
    let (text, colour) = match last {
        None => ("—".to_string(), MUTED),
        Some(run) => {
            let time = run.seconds.map(duration_text).unwrap_or_default();
            match &run.status {
                Status::Ok if run.calm.timing_valid => (format!("ok {time}"), GREEN),
                Status::Ok => (format!("ok ({time})"), GREEN.gamma_multiply(0.7)),
                Status::TimedOut => ("timed out".to_string(), RED),
                Status::Failed { .. } => ("failed".to_string(), RED),
                other => (other.text(), AMBER),
            }
        }
    };
    Shown { text, colour, hover, failed, never }
}

/// The values of a record's cell, in the order of `values`.
fn same_cell(run: &Run, batch: &Batch, values: &[(String, String)]) -> bool {
    run.job == batch.job.name
        && run.cell.as_ref().is_some_and(|cell| {
            cell.values.len() == values.len() && values.iter().all(|(axis, value)| cell.values.get(axis) == Some(value))
        })
}

/// The last record of a cell, by its start.
fn last_run<'a>(batch: &Batch, values: &[(String, String)], runs: &'a [Run]) -> Option<&'a Run> {
    runs.iter().filter(|run| same_cell(run, batch, values)).max_by_key(|run| run.started)
}

/// The median time of the cell's `Ok` records (SPEC.md, §6b, R3).
fn cell_estimate(batch: &Batch, values: &[(String, String)], runs: &[Run]) -> Option<f64> {
    let mut seconds: Vec<f64> = runs
        .iter()
        .filter(|run| same_cell(run, batch, values) && run.status == Status::Ok)
        .filter_map(|run| run.seconds)
        .collect();
    if seconds.is_empty() {
        return None;
    }
    seconds.sort_by(f64::total_cmp);
    let middle = seconds.len() / 2;
    Some(if seconds.len() % 2 == 1 { seconds[middle] } else { (seconds[middle - 1] + seconds[middle]) / 2.0 })
}

/// The estimate of a batch in the queue: the cells' known medians over the
/// pool's width, and how many cells have none.
pub(super) fn batch_estimate(batch: &Batch, choice: &BatchChoice, runs: &[Run], setting: u32) -> (Option<f64>, usize) {
    let Ok(mut cells) = batch.expand(&[]) else { return (None, 0) };
    if !choice.cells.is_empty() {
        cells.retain(|cell| {
            choice.cells.iter().any(|kept| cell.values.iter().all(|(axis, value)| kept.get(axis) == Some(value)))
        });
    }
    let width = crate::batch::batch_width(&batch.job, setting);
    let known: Vec<f64> = cells.iter().filter_map(|cell| cell_estimate(batch, &cell.values, runs)).collect();
    let unknown = cells.len() - known.len();
    if known.is_empty() {
        return (None, unknown);
    }
    (Some(known.iter().sum::<f64>() / f64::from(width)), unknown)
}
