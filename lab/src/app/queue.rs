//! The queue screen (SPEC.md, section 11): the catalogue to pick from, the
//! queue with its settings, the run under way and its log.

use std::time::{Duration, Instant};

use eframe::egui::{self, RichText};

use super::{AMBER, App, BLUE, GREEN, MUTED, RED, STOPWATCH, command_lines, place_style, stopwatch};
use crate::catalogue::render;
use crate::catalogue::{Category, CheckoutChoice, Job};
use crate::files;
use crate::mirror::Target;
use crate::model::{Paper, PaperRef, Status, duration_text};
use crate::runner::{BatchChoice, ItemState, LiveGrid, LiveState, Mode, OrphanDecision, Phase, QueueSettings, Shared};
use crate::settings::Policy;
use crate::state::{Estimate, estimate};

/// How often the log under view is read again.
const LOG_EVERY: Duration = Duration::from_millis(500);
/// The lines of the log kept on the screen.
const LOG_LINES: usize = 2000;

fn policy_text(policy: Policy) -> &'static str {
    match policy {
        Policy::PauseAndAsk => "pause and ask",
        Policy::Continue => "continue",
        Policy::CancelTheRest => "cancel the rest",
    }
}

impl App {
    /// A job, a batch (all its cells), or every job and batch of a sequence,
    /// at the end of the queue.
    pub(super) fn add_to_queue(&mut self, name: &str) {
        let Ok(catalogue) = &self.catalogue else { return };
        let names = match catalogue.sequences.iter().find(|sequence| sequence.name == name) {
            Some(sequence) => sequence.jobs.clone(),
            None => vec![name.to_string()],
        };
        let shared_catalogue = std::sync::Arc::new(catalogue.clone());
        let mut problems = Vec::new();
        let mut shared = self.shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        for name in names {
            if let Some(job) = catalogue.job(&name) {
                shared.add(job.clone(), job.checkout.initial());
            } else if let Some(batch) = catalogue.batch(&name) {
                let choice =
                    BatchChoice { catalogue: shared_catalogue.clone(), cells: Vec::new(), timeout: None, again: false };
                shared.add_batch(batch, choice);
            } else {
                problems.push(format!("no job nor batch named {name}"));
            }
        }
        shared.messages.extend(problems);
    }

    /// What the runs of the queue share, as the screen sets it now.
    pub(super) fn queue_settings(&self) -> QueueSettings {
        let papers =
            self.paper.map(|paper| vec![PaperRef { paper, label: self.label.trim().to_string() }]).unwrap_or_default();
        QueueSettings {
            id: chrono::Local::now().format("%Y%m%d-%H%M%S").to_string(),
            comment: self.comment.trim().to_string(),
            papers,
            threads: self.settings.threads,
            on_failure: Some(self.settings.on_failure),
            grace_seconds: self.settings.stop_grace_seconds,
        }
    }

    pub(super) fn queue_tab(&mut self, ui: &mut egui::Ui) {
        let Ok(catalogue) = &self.catalogue else {
            ui.label("No catalogue: see the error above.");
            return;
        };
        let catalogue = catalogue.clone();
        let waiting: Vec<String> = {
            let shared = self.lock();
            shared
                .items
                .iter()
                .filter(|item| item.state == ItemState::Waiting)
                .map(|item| item.job.name.clone())
                .collect()
        };
        let mut add: Vec<String> = Vec::new();
        let mut remove: Vec<String> = Vec::new();
        egui::Panel::left("catalogue").resizable(true).default_size(300.0).show(ui, |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search jobs"));
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                ui.add_space(6.0);
                ui.label(RichText::new("SEQUENCES").small().strong().color(MUTED));
                for sequence in &catalogue.sequences {
                    let mut all = !sequence.jobs.is_empty() && sequence.jobs.iter().all(|name| waiting.contains(name));
                    let text = format!("{}  ({} jobs)", sequence.name, sequence.jobs.len());
                    if ui.checkbox(&mut all, text).on_hover_text(&sequence.description).changed() {
                        if all {
                            add.push(sequence.name.clone());
                        } else {
                            remove.extend(sequence.jobs.iter().cloned());
                        }
                    }
                }
                let search = self.search.to_lowercase();
                let batches: Vec<_> = catalogue
                    .batches
                    .iter()
                    .filter(|batch| {
                        search.is_empty()
                            || batch.job.name.contains(&search)
                            || batch.job.description.to_lowercase().contains(&search)
                    })
                    .collect();
                if !batches.is_empty() {
                    ui.add_space(8.0);
                    ui.label(RichText::new("BATCHES").small().strong().color(MUTED));
                    for batch in batches {
                        let mut checked = waiting.contains(&batch.job.name);
                        let jobs: usize = batch.axes.iter().map(|axis| axis.values.len()).product();
                        ui.horizontal(|ui| {
                            let text = format!("{}  ({jobs} jobs)", batch.job.name);
                            let hover = format!(
                                "{}\n\nEvery cell; choose some in the Batches screen.",
                                batch.job.description
                            );
                            if ui.checkbox(&mut checked, text).on_hover_text(hover).changed() {
                                if checked {
                                    add.push(batch.job.name.clone());
                                } else {
                                    remove.push(batch.job.name.clone());
                                }
                            }
                            stopwatch(ui, batch.job.timing_matters);
                        });
                    }
                }
                for category in Category::ALL {
                    let jobs: Vec<&Job> = catalogue
                        .jobs
                        .iter()
                        .filter(|job| job.category == category)
                        .filter(|job| {
                            search.is_empty()
                                || job.name.contains(&search)
                                || job.description.to_lowercase().contains(&search)
                        })
                        .collect();
                    if jobs.is_empty() {
                        continue;
                    }
                    ui.add_space(8.0);
                    ui.label(RichText::new(category.title().to_uppercase()).small().strong().color(MUTED));
                    for job in jobs {
                        let mut checked = waiting.contains(&job.name);
                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut checked, &job.name).on_hover_text(&job.description).changed() {
                                if checked {
                                    add.push(job.name.clone());
                                } else {
                                    remove.push(job.name.clone());
                                }
                            }
                            stopwatch(ui, job.timing_matters);
                        });
                    }
                }
                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!(
                        "{STOPWATCH} its duration counts: a run under load reports no time (with the calm setting \
                         `wait`, the job waits for a calm machine). Without the mark, the duration is not a measurement."
                    ))
                    .small()
                    .color(MUTED),
                );
            });
        });
        for name in add {
            self.add_to_queue(&name);
        }
        if !remove.is_empty() {
            self.lock().items.retain(|item| item.state != ItemState::Waiting || !remove.contains(&item.job.name));
        }
        egui::Panel::right("run").resizable(true).default_size(560.0).show(ui, |ui| self.run_panel(ui));
        egui::CentralPanel::default().show(ui, |ui| self.queue_panel(ui));
    }

    fn queue_panel(&mut self, ui: &mut egui::Ui) {
        let shared = self.shared.clone();
        let mut shared = shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let count = shared.items.len();
        let waiting = shared.items.iter().filter(|item| item.state == ItemState::Waiting).count();
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("QUEUE · {count} JOBS")).small().strong().color(MUTED));
            let mode = match &shared.mode {
                Mode::Idle => "idle".to_string(),
                Mode::Running => format!("running, {} threads", shared.queue.threads),
                Mode::Paused(_) => "paused".to_string(),
            };
            ui.label(RichText::new(mode).color(MUTED));
            let finished =
                shared.items.iter().any(|item| matches!(item.state, ItemState::Done { .. } | ItemState::Cancelled));
            if finished && ui.small_button("Clear the finished").clicked() {
                shared.items.retain(|item| matches!(item.state, ItemState::Waiting | ItemState::Running));
            }
        });
        if let Some(orphan) = shared.orphan.clone() {
            banner(ui, RED, |ui| {
                ui.label(format!(
                    "{} still runs from the last session (process group {}): its exit status cannot be known.",
                    orphan.run.job, orphan.group
                ));
                match orphan.decision {
                    None => {
                        ui.horizontal(|ui| {
                            if ui.button("Wait for it to end").clicked() {
                                shared.orphan.as_mut().expect("the orphan").decision = Some(OrphanDecision::Wait);
                            }
                            if ui.button("Kill it").clicked() {
                                shared.orphan.as_mut().expect("the orphan").decision = Some(OrphanDecision::Kill);
                            }
                        });
                    }
                    Some(decision) => {
                        ui.label(if decision == OrphanDecision::Wait {
                            "Waiting for it to end."
                        } else {
                            "Killing it."
                        });
                    }
                }
            });
        }
        if let Mode::Paused(why) = shared.mode.clone() {
            banner(ui, AMBER, |ui| {
                ui.label(why);
                ui.horizontal(|ui| {
                    let free = shared.orphan.is_none();
                    if ui.add_enabled(free && waiting > 0, egui::Button::new("Continue the queue")).clicked() {
                        shared.mode = Mode::Running;
                    }
                    if ui.button("Cancel the rest").clicked() {
                        shared.cancel_waiting();
                        shared.mode = Mode::Idle;
                    }
                });
            });
        }
        if let Some(why) = shared.waiting_for_calm.clone() {
            banner(ui, RED, |ui| {
                ui.label(format!("{STOPWATCH} {why}"));
                ui.horizontal(|ui| {
                    if ui.button("Start anyway (its time will not count)").clicked()
                        && let Some(item) = shared.items.iter_mut().find(|item| item.state == ItemState::Waiting)
                    {
                        item.forced = true;
                    }
                    if ui.button("Pause the queue").clicked() {
                        shared.mode = Mode::Paused("Paused while waiting for a calm machine.".to_string());
                        shared.waiting_for_calm = None;
                    }
                });
            });
        }
        // A queue recovered before Run was pressed has no thread count of its own yet.
        let threads = if shared.mode == Mode::Idle || shared.queue.threads == 0 {
            self.settings.threads
        } else {
            shared.queue.threads
        };
        let timings = self.timings(&shared, threads);
        let mut drops: Vec<(u64, usize)> = Vec::new();
        let mut removed: Vec<u64> = Vec::new();
        if let Some(Some(last)) = timings.iter().rev().find(|timing| timing.is_some()) {
            ui.label(RichText::new(last.total_text(true)).strong());
        }
        let references = self.references.clone();
        let resolutions: Vec<Option<Result<super::Resolution, String>>> = shared
            .items
            .iter()
            .map(|item| {
                matches!(item.state, ItemState::Waiting | ItemState::Running).then(|| self.resolution(&item.target))
            })
            .collect();
        let shared_grid = shared.grid.clone();
        let height = ui.available_height() - 210.0;
        egui::ScrollArea::vertical().max_height(height.max(120.0)).auto_shrink([false, true]).show(ui, |ui| {
            for index in 0..shared.items.len() {
                let item = &mut shared.items[index];
                let frame = egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    let editable = item.state == ItemState::Waiting;
                    ui.horizontal(|ui| {
                        if editable {
                            handle(ui, item.key);
                        } else {
                            ui.add_space(14.0);
                        }
                        ui.label(RichText::new(format!("{} · {}", index + 1, item.job.name)).strong());
                        stopwatch(ui, item.job.timing_matters);
                        let (text, colour) = state_text(&item.state);
                        ui.colored_label(colour, text);
                        let ref_axis = item.batch.as_ref().is_some_and(|choice| {
                            choice
                                .catalogue
                                .batch(&item.job.name)
                                .is_some_and(|batch| batch.axis(crate::catalogue::REFERENCE_AXIS).is_some())
                        });
                        if ref_axis {
                            ui.label(RichText::new("on its ref axis").color(MUTED));
                        } else if editable {
                            ui.label(RichText::new("on").color(MUTED));
                            ui.add(
                                egui::TextEdit::singleline(&mut item.target)
                                    .hint_text("tag, branch, pull/<n>, hash, or a folder")
                                    .desired_width(150.0)
                                    .font(egui::TextStyle::Monospace),
                            )
                            .on_hover_text(
                                "a tag (v0.7.0), a branch (main), a pull request (pull/301) or a hash, resolved in the mirror; \
                                 or a folder on this machine (/…, ~/…), the escape hatch for an experiment not committed",
                            );
                            ui.menu_button("…", |ui| {
                                for reference in &references {
                                    if ui.button(reference).clicked() {
                                        item.target = reference.clone();
                                        ui.close();
                                    }
                                }
                            });
                            if item.job.checkout == CheckoutChoice::Ask && item.target == item.job.checkout.initial() {
                                ui.colored_label(AMBER, "pick its reference").on_hover_text(
                                    "the catalogue says checkout = \"ask\": the job is to be run where you say",
                                );
                            }
                        } else {
                            ui.label(RichText::new(format!("on {}", item.target)).monospace().color(MUTED));
                        }
                        let effective = item.job.effective_threads(threads);
                        let plural = if effective == 1 { "thread" } else { "threads" };
                        let colour = if effective > self.settings.comfort_threads { AMBER } else { MUTED };
                        ui.colored_label(colour, format!("{effective} {plural}"));
                        match resolutions.get(index).cloned().flatten() {
                            Some(Ok(resolution)) => {
                                let place = match (&resolution.tag, resolution.pushed) {
                                    (Some(tag), _) => tag.clone(),
                                    (None, true) => format!("off-tag {}", resolution.short),
                                    (None, false) => format!("orphan {}", resolution.short),
                                };
                                let (colour, hint) = place_style(&place);
                                ui.colored_label(colour, place).on_hover_text(hint);
                                if resolution.dirty {
                                    ui.colored_label(AMBER, "modified").on_hover_text("tracked files differ from HEAD");
                                }
                                let ready = if resolution.checked_out { "checkout ready" } else { "to check out" };
                                ui.label(RichText::new(ready).color(MUTED));
                            }
                            Some(Err(error)) if item.target.trim() != crate::mirror::MAIN => {
                                ui.colored_label(AMBER, format!("unknown, {} instead", crate::mirror::MAIN))
                                    .on_hover_text(format!("{error}; the run will be on {}", crate::mirror::MAIN));
                            }
                            Some(Err(error)) => {
                                ui.colored_label(RED, "unknown").on_hover_text(error);
                            }
                            None => {}
                        }
                        if editable {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("remove").clicked() {
                                    removed.push(item.key);
                                }
                            });
                        }
                    });
                    if let Some(Some(timing)) = timings.get(index) {
                        ui.horizontal(|ui| {
                            ui.add_space(14.0);
                            match &timing.estimate {
                                Some(estimate) => {
                                    ui.label(
                                        RichText::new(format!("≈ {}", duration_text(estimate.seconds))).monospace(),
                                    );
                                    ui.label(RichText::new(estimate.basis()).color(MUTED));
                                }
                                None => {
                                    ui.label(RichText::new("no estimate").color(MUTED));
                                }
                            }
                            ui.label(RichText::new(format!("· {}", timing.total_text(false))).monospace());
                        });
                    }
                    if let Some(choice) = &item.batch {
                        let cells = if choice.cells.is_empty() {
                            "every cell".to_string()
                        } else {
                            format!("{} cells chosen", choice.cells.len())
                        };
                        let timeout = choice
                            .timeout
                            .map(|timeout| format!(" · timeout {}", duration_text(timeout.seconds)))
                            .unwrap_or_default();
                        let again = if choice.again { " · again: the jobs done run again" } else { "" };
                        let mut line = format!("batch · {cells}{timeout}{again}");
                        if item.state == ItemState::Running
                            && let Some(grid) = &shared_grid
                        {
                            let ended = grid
                                .cells
                                .iter()
                                .filter(|cell| matches!(cell.state, LiveState::Ended { .. } | LiveState::Skipped))
                                .count();
                            line.push_str(&format!(" · {ended}/{}", grid.cells.len()));
                        }
                        ui.horizontal(|ui| {
                            ui.add_space(14.0);
                            ui.label(RichText::new(line).color(MUTED));
                        });
                        return;
                    }
                    if editable && !item.parameters.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            for (name, value) in item.parameters.iter_mut() {
                                ui.label(RichText::new(name.as_str()).color(MUTED));
                                ui.add(
                                    egui::TextEdit::singleline(value)
                                        .desired_width(480.0)
                                        .font(egui::TextStyle::Monospace),
                                );
                            }
                        });
                    }
                    let commit = match resolutions.get(index) {
                        Some(Some(Ok(resolution))) => resolution.short.clone(),
                        _ => "{commit}".to_string(),
                    };
                    let reference = Target::parse(&item.target).recorded();
                    let values = item.job.values(threads, &commit, &reference, None, &item.parameters);
                    command_lines(ui, &item.job, &values);
                });
                let response = frame.response;
                if item.state == ItemState::Waiting {
                    if response.dnd_hover_payload::<u64>().is_some() {
                        let rect = response.rect;
                        ui.painter().hline(rect.x_range(), rect.top() - 2.0, egui::Stroke::new(2.0, BLUE));
                    }
                    if let Some(dragged) = response.dnd_release_payload::<u64>() {
                        drops.push((*dragged, index));
                    }
                }
            }
        });
        shared.items.retain(|item| !removed.contains(&item.key));
        // A job dropped on a waiting one takes its place; only waiting jobs move.
        for (dragged, target) in drops {
            let Some(from) =
                shared.items.iter().position(|item| item.key == dragged && item.state == ItemState::Waiting)
            else {
                continue;
            };
            let moved = shared.items.remove(from);
            let target = target.min(shared.items.len());
            shared.items.insert(target, moved);
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Comment");
            ui.add(
                egui::TextEdit::singleline(&mut self.comment)
                    .desired_width(ui.available_width())
                    .hint_text("for every run of this queue"),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Paper");
            ui.radio_value(&mut self.paper, None, "none");
            ui.radio_value(&mut self.paper, Some(Paper::A), "A");
            ui.radio_value(&mut self.paper, Some(Paper::B), "B");
            ui.add_enabled(
                self.paper.is_some(),
                egui::TextEdit::singleline(&mut self.label).hint_text("v0.7.0 · tab:forced").desired_width(220.0),
            );
            ui.label("On failure");
            let mut failure = self.settings.on_failure;
            egui::ComboBox::from_id_salt("on failure").selected_text(policy_text(failure)).show_ui(ui, |ui| {
                for policy in [Policy::PauseAndAsk, Policy::Continue, Policy::CancelTheRest] {
                    ui.selectable_value(&mut failure, policy, policy_text(policy));
                }
            });
            if failure != self.settings.on_failure {
                self.settings.on_failure = failure;
                if let Err(error) = self.settings.save(&self.places.settings()) {
                    self.settings_error = Some(error);
                }
            }
        });
        ui.horizontal(|ui| {
            let ready = shared.mode == Mode::Idle && waiting > 0 && shared.orphan.is_none();
            let run = ui
                .add_enabled(ready, egui::Button::new(RichText::new("Run").strong()).min_size(egui::vec2(90.0, 28.0)));
            if run.clicked() {
                shared.start(self.queue_settings());
            }
            let note = format!("{waiting} waiting, one at a time, {} threads for the setting", self.settings.threads);
            ui.label(RichText::new(note).color(MUTED));
            if self.settings.threads > self.settings.comfort_threads {
                ui.colored_label(
                    AMBER,
                    format!("above {}: the machine will be slow to use", self.settings.comfort_threads),
                );
            }
        });
    }

    fn run_panel(&mut self, ui: &mut egui::Ui) {
        let (current, position, count, estimate, timed_current) = {
            let arc = self.shared.clone();
            let shared = arc.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let current = shared.current.clone();
            let position =
                current.as_ref().and_then(|current| shared.items.iter().position(|item| item.key == current.key));
            let threads = shared.queue.threads;
            let estimate = position
                .and_then(|index| self.timings(&shared, threads).swap_remove(index))
                .and_then(|timing| timing.estimate);
            let timed_current =
                position.and_then(|index| shared.items.get(index)).is_some_and(|item| item.job.timing_matters);
            (current, position, shared.items.len(), estimate, timed_current)
        };
        match &current {
            Some(current) => {
                ui.label(
                    RichText::new(format!("RUNNING · ITEM {} OF {count}", position.map_or(0, |index| index + 1)))
                        .small()
                        .strong()
                        .color(MUTED),
                );
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&current.job).strong().size(20.0));
                    stopwatch(ui, timed_current);
                });
                let grid = self.lock().grid.clone().filter(|grid| grid.batch == current.job && !grid.ended);
                if let Some(grid) = &grid {
                    let count =
                        |wanted: fn(&LiveState) -> bool| grid.cells.iter().filter(|cell| wanted(&cell.state)).count();
                    let ended = count(|state| matches!(state, LiveState::Ended { .. } | LiveState::Skipped));
                    let running = count(|state| matches!(state, LiveState::Running(_)));
                    let failed =
                        count(|state| matches!(state, LiveState::Ended { status, .. } if *status != Status::Ok));
                    // The dependencies being made: what runs before the jobs, each with its time.
                    let making: Vec<(&String, f64)> = grid
                        .dependencies
                        .iter()
                        .filter_map(|(text, state)| match state {
                            LiveState::Running(since) => Some((text, since.elapsed().as_secs_f64())),
                            _ => None,
                        })
                        .collect();
                    if making.is_empty() {
                        if !grid.phase.is_empty() {
                            ui.label(RichText::new(format!("now: {}", grid.phase)).strong());
                        }
                    } else {
                        let runs = self.all_runs();
                        for (text, seconds) in &making {
                            ui.label(RichText::new(format!("now: making {text}, before the jobs")).strong());
                            let name = text.split_whitespace().next().unwrap_or_default();
                            let mut made: Vec<f64> = runs
                                .iter()
                                .filter(|run| run.job == name && run.status == Status::Ok)
                                .filter_map(|run| run.seconds)
                                .collect();
                            made.sort_by(f64::total_cmp);
                            let estimate = made.get(made.len() / 2).copied();
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(duration_text(*seconds)).strong().size(16.0));
                                let said = match estimate {
                                    Some(estimate) => format!(
                                        "/ ≈ {} (median of {} {name} made before)",
                                        duration_text(estimate),
                                        made.len()
                                    ),
                                    None => format!("no {name} made before: no estimate"),
                                };
                                ui.label(RichText::new(said).color(MUTED));
                            });
                            if let Some(estimate) = estimate {
                                ui.add(
                                    egui::ProgressBar::new((seconds / estimate.max(1.0)).min(1.0) as f32).fill(AMBER),
                                );
                            }
                        }
                        ui.label(RichText::new("its log below; the dependencies in the Batches screen").color(MUTED));
                        ui.add_space(4.0);
                    }
                    ui.label(format!(
                        "jobs: {ended}/{} done, {running} running, {failed} not ok; the list in the Batches screen",
                        grid.cells.len()
                    ));
                    if !grid.cells.is_empty() {
                        ui.add(
                            egui::ProgressBar::new(ended as f32 / grid.cells.len() as f32)
                                .fill(BLUE)
                                .text(format!("{ended}/{}", grid.cells.len())),
                        );
                    }
                    launch_jobs(ui, grid, &self.all_runs());
                }
                let phase = match (current.stopping, current.phase) {
                    (true, _) => "stopping".to_string(),
                    (false, Phase::Prepare) => "its prepare".to_string(),
                    (false, Phase::Command) if grid.is_some() => "the whole launch".to_string(),
                    (false, Phase::Command) => "its command".to_string(),
                };
                let elapsed = current.started.elapsed().as_secs_f64();
                // A batch's item has no estimate of its own: what is left comes from its cells'.
                let estimate = if grid.is_some() { None } else { estimate };
                let phase = match &grid {
                    Some(grid) => format!("{phase} · {}", time_left(grid)),
                    None => phase,
                };
                ui.horizontal(|ui| {
                    ui.label(RichText::new(duration_text(elapsed)).strong().size(16.0));
                    match &estimate {
                        Some(estimate) => {
                            ui.label(
                                RichText::new(format!(
                                    "/ ≈ {} · {} · {phase}",
                                    duration_text(estimate.seconds),
                                    estimate.basis()
                                ))
                                .color(MUTED),
                            );
                        }
                        None => {
                            let said = if grid.is_some() {
                                format!("elapsed · {phase}")
                            } else {
                                format!("elapsed · no estimate · {phase}")
                            };
                            ui.label(RichText::new(said).color(MUTED));
                        }
                    }
                });
                if let Some(estimate) = &estimate {
                    let fraction = (elapsed / estimate.seconds.max(1.0)).min(1.0) as f32;
                    ui.add(egui::ProgressBar::new(fraction).fill(AMBER));
                    if elapsed > estimate.seconds {
                        ui.colored_label(
                            AMBER,
                            format!("over the estimate by {}", duration_text(elapsed - estimate.seconds)),
                        );
                    }
                }
                ui.horizontal(|ui| {
                    let stop = ui.add_enabled(!current.stopping, egui::Button::new(RichText::new("Stop").color(RED)));
                    if stop.clicked() {
                        self.stop_dialog = Some(self.settings.on_stop);
                    }
                    ui.label(
                        RichText::new(format!(
                            "SIGTERM to the group, SIGKILL after {} s",
                            self.settings.stop_grace_seconds
                        ))
                        .color(MUTED),
                    );
                });
                if self.log.path.as_ref() != Some(&current.log) {
                    self.log = super::LogView { path: Some(current.log.clone()), ..Default::default() };
                }
                ui.ctx().request_repaint_after(LOG_EVERY);
            }
            None => {
                ui.label(RichText::new("NOTHING RUNNING").small().strong().color(MUTED));
                if self.log.path.is_some() {
                    ui.label(RichText::new("The log of the last run:").color(MUTED));
                }
            }
        }
        let Some(path) = self.log.path.clone() else { return };
        if self.log.read.is_none_or(|read| read.elapsed() >= LOG_EVERY) {
            self.log.lines = files::tail(&path, LOG_LINES);
            self.log.read = Some(Instant::now());
        }
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new("LOG").small().strong().color(MUTED));
            ui.label(
                RichText::new(path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default())
                    .monospace()
                    .color(MUTED),
            );
            ui.add(egui::TextEdit::singleline(&mut self.log.search).hint_text("Find in the log").desired_width(160.0));
        });
        let search = self.log.search.clone();
        egui::ScrollArea::both().stick_to_bottom(true).auto_shrink([false; 2]).show(ui, |ui| {
            for line in self.log.lines.iter().filter(|line| search.is_empty() || line.contains(&search)) {
                let colour = if line.starts_with("== lab") { GREEN } else { ui.visuals().text_color() };
                ui.label(RichText::new(line).monospace().size(11.5).color(colour));
            }
        });
    }

    /// Stop asked from the run panel: what then of the rest of the queue.
    pub(super) fn stop_dialog(&mut self, context: &egui::Context) {
        let Some(mut then) = self.stop_dialog else { return };
        let (job, waiting) = {
            let shared = self.lock();
            let waiting = shared.items.iter().filter(|item| item.state == ItemState::Waiting).count();
            (shared.current.as_ref().map(|current| current.job.clone()), waiting)
        };
        let Some(job) = job else {
            self.stop_dialog = None;
            return;
        };
        let mut open = true;
        egui::Window::new("Stop the run").collapsible(false).resizable(false).show(context, |ui| {
            ui.label(format!(
                "Stop {job}? SIGTERM to its process group now, SIGKILL after {} s. The run is recorded as stopped, with its log.",
                self.settings.stop_grace_seconds
            ));
            ui.label(format!("Then the {waiting} jobs left in the queue:"));
            ui.radio_value(&mut then, Policy::PauseAndAsk, "pause, and ask me");
            ui.radio_value(&mut then, Policy::CancelTheRest, "cancel them");
            ui.horizontal(|ui| {
                if ui.button("Keep running").clicked() {
                    open = false;
                }
                if ui.button(RichText::new("Stop").color(RED)).clicked() {
                    self.lock().stop = Some(then);
                    open = false;
                }
            });
        });
        self.stop_dialog = open.then_some(then);
    }
}

/// What an item of the queue is expected to take, and the queue up to it.
pub(super) struct Timing {
    estimate: Option<Estimate>,
    /// The estimated seconds from now to the end of this item.
    total: f64,
    /// Items up to this one without an estimate.
    unknown: usize,
}

impl Timing {
    /// "total 12 min 02 · ends 15:44", or with jobs without an estimate,
    /// "at least …, plus 2 jobs without an estimate · ends after …".
    fn total_text(&self, queue: bool) -> String {
        let ends = (chrono::Local::now() + chrono::Duration::seconds(self.total.round() as i64)).format("%H:%M");
        let what = if queue { "queue" } else { "total" };
        match self.unknown {
            0 => format!("{what} {} · ends {ends}", duration_text(self.total)),
            unknown => {
                let jobs = if unknown == 1 { "job" } else { "jobs" };
                format!(
                    "{what} at least {}, plus {unknown} {jobs} without an estimate · ends after {ends}",
                    duration_text(self.total)
                )
            }
        }
    }
}

/// The drag handle of a waiting item: six dots, its key as the payload.
fn handle(ui: &mut egui::Ui, key: u64) {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(12.0, 18.0), egui::Sense::drag());
    let colour = if response.hovered() { ui.visuals().strong_text_color() } else { MUTED };
    for row in 0..3 {
        for column in 0..2 {
            let centre = rect.min + egui::vec2(3.0 + 6.0 * column as f32, 4.0 + 5.0 * row as f32);
            ui.painter().circle_filled(centre, 1.5, colour);
        }
    }
    response.on_hover_text("drag to reorder").dnd_set_drag_payload(key);
}

impl App {
    /// For each item still to run, its estimate and the running total from
    /// now (SPEC.md, section 10); None for an item done.
    fn timings(&mut self, shared: &Shared, threads: u32) -> Vec<Option<Timing>> {
        let runs = self.all_runs();
        let job_cpus = crate::cpus::job_cpus(&self.settings.calm.job_cpus).unwrap_or_default();
        let elapsed = shared.current.as_ref().map(|current| (current.key, current.started.elapsed().as_secs_f64()));
        let mut total = 0.0;
        let mut unknown = 0;
        shared
            .items
            .iter()
            .map(|item| {
                if !matches!(item.state, ItemState::Waiting | ItemState::Running) {
                    return None;
                }
                if let Some(choice) = &item.batch {
                    let found = choice
                        .catalogue
                        .batch(&item.job.name)
                        .map(|batch| super::batches::batch_estimate(batch, choice, &runs, threads, &job_cpus));
                    let (seconds, without) = found.unwrap_or((None, 0));
                    let estimate = seconds.map(|seconds| Estimate { seconds, runs: 1, load: None });
                    match &estimate {
                        Some(estimate) => {
                            let spent = elapsed.filter(|(key, _)| *key == item.key).map_or(0.0, |(_, seconds)| seconds);
                            total += (estimate.seconds - spent).max(0.0);
                        }
                        None => unknown += 1,
                    }
                    unknown += usize::from(without > 0 && estimate.is_some());
                    return Some(Timing { estimate, total, unknown });
                }
                let (commit, short) = match self.resolution(&item.target) {
                    Ok(resolution) => (resolution.commit, resolution.short),
                    Err(_) => (String::new(), "{commit}".to_string()),
                };
                let reference = Target::parse(&item.target).recorded();
                let values = item.job.values(threads, &short, &reference, None, &item.parameters);
                let command = render(&item.job.command, &values).unwrap_or_default();
                let estimate = estimate(&item.job.name, &command, &commit, &runs);
                match &estimate {
                    Some(estimate) => {
                        let spent = elapsed.filter(|(key, _)| *key == item.key).map_or(0.0, |(_, seconds)| seconds);
                        total += (estimate.seconds - spent).max(0.0);
                    }
                    None => unknown += 1,
                }
                Some(Timing { estimate, total, unknown })
            })
            .collect()
    }
}

fn state_text(state: &ItemState) -> (String, egui::Color32) {
    match state {
        ItemState::Waiting => ("waiting".to_string(), MUTED),
        ItemState::Running => ("running".to_string(), AMBER),
        ItemState::Cancelled => ("cancelled".to_string(), MUTED),
        ItemState::Done { status, seconds, .. } => {
            let time = seconds.map(|seconds| format!(" · {}", duration_text(seconds))).unwrap_or_default();
            let colour = match status {
                Status::Ok => GREEN,
                Status::Stopped | Status::Interrupted => AMBER,
                Status::Failed { .. } | Status::TimedOut => RED,
            };
            (format!("{}{time}", status.text()), colour)
        }
    }
}

fn banner(ui: &mut egui::Ui, colour: egui::Color32, add: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(colour.gamma_multiply(0.12))
        .stroke(egui::Stroke::new(1.0, colour.gamma_multiply(0.5)))
        .inner_margin(8)
        .corner_radius(4)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add(ui);
        });
    ui.add_space(6.0);
}

/// What is left of a launch, from the estimates of its cells not ended, over its pool's width.
fn time_left(grid: &LiveGrid) -> String {
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
    let left = known.iter().sum::<f64>() / f64::from(grid.width.max(1));
    match (known.len(), remaining.len() - known.len()) {
        (0, 0) => "nothing left".to_string(),
        (0, _) => "no estimate of what is left".to_string(),
        (_, 0) => format!("about {} left", duration_text(left)),
        (_, unknown) => format!("at least {} left, plus {unknown} jobs without an estimate", duration_text(left)),
    }
}

/// One line per job of the launch: those under way, then those ended, the last first.
fn launch_jobs(ui: &mut egui::Ui, grid: &LiveGrid, runs: &[crate::model::Run]) {
    let text =
        |values: &[(String, String)]| values.iter().map(|(_, value)| value.as_str()).collect::<Vec<_>>().join(" · ");
    let mut ended: Vec<&crate::model::Run> = runs
        .iter()
        .filter(|run| run.job == grid.batch && run.cell.as_ref().is_some_and(|cell| cell.launch == grid.launch))
        .collect();
    ended.sort_by_key(|run| std::cmp::Reverse(run.started));
    egui::ScrollArea::vertical().id_salt("launch jobs").max_height(240.0).show(ui, |ui| {
        egui::Grid::new("launch jobs").striped(true).spacing([10.0, 2.0]).show(ui, |ui| {
            for (index, cell) in grid.cells.iter().enumerate() {
                let LiveState::Running(since) = cell.state else { continue };
                ui.label(RichText::new(format!("{}", index + 1)).monospace().color(MUTED));
                ui.label(text(&cell.values));
                ui.label(RichText::new(format!("… {}", duration_text(since.elapsed().as_secs_f64()))).color(BLUE));
                ui.label(
                    RichText::new(
                        cell.estimate.map_or(String::new(), |seconds| format!("≈ {}", duration_text(seconds))),
                    )
                    .color(MUTED),
                );
                ui.end_row();
            }
            for run in ended {
                let Some(cell) = &run.cell else { continue };
                let index = grid
                    .cells
                    .iter()
                    .position(|live| live.values.iter().all(|(axis, value)| cell.values.get(axis) == Some(value)));
                ui.label(
                    RichText::new(index.map_or("?".to_string(), |index| format!("{}", index + 1)))
                        .monospace()
                        .color(MUTED),
                );
                let values: Vec<(String, String)> = index
                    .map(|index| grid.cells[index].values.clone())
                    .unwrap_or_else(|| cell.values.clone().into_iter().collect());
                ui.label(text(&values));
                let colour = if run.status == Status::Ok { GREEN } else { RED };
                ui.label(RichText::new(run.status.text()).color(colour));
                let time = run.seconds.map(duration_text).unwrap_or_default();
                if run.calm.timing_valid {
                    ui.label(time);
                } else {
                    ui.label(RichText::new(format!("{time} (not calm)")).color(MUTED))
                        .on_hover_text(format!("the time does not count: {}", run.calm.reason));
                }
                ui.end_row();
            }
        });
    });
}
