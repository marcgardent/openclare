//! The tools of the History screen (SPEC.md, section 11): the comment and the
//! papers edited afterwards, the export of the runs shown, the copy of the
//! history into the repository, and a past run typed by hand.

use std::collections::{BTreeMap, BTreeSet};

use chrono::{Local, NaiveDateTime, TimeZone};
use eframe::egui::{self, RichText};

use super::{App, MUTED, RED};
use crate::model::{Calm, FORMAT, Git, Paper, PaperRef, Run, Source, Status, duration_text};
use crate::state::{Reach, Tag, place, tag_of};

/// The comment and the paper of the run selected, as edited.
pub(super) struct Edit {
    id: String,
    comment: String,
    paper: Option<Paper>,
    label: String,
}

/// The form of a run lab did not launch.
pub(super) struct PastRun {
    job: String,
    started: String,
    hours: u32,
    minutes: u32,
    seconds: u32,
    commit: String,
    comment: String,
    error: Option<String>,
}

impl App {
    pub(super) fn history_tools(&mut self, ui: &mut egui::Ui, shown: &[&Run]) {
        if ui.button("Copy as Markdown").on_hover_text("the runs shown, with the hardware line").clicked() {
            ui.ctx().copy_text(markdown(shown, &self.tags, &self.reach, &self.settings.machine_label));
        }
        if ui.button("Copy as CSV").clicked() {
            ui.ctx().copy_text(csv(shown, &self.tags, &self.reach));
        }
        if ui.button("Record a past run").clicked() {
            let job =
                self.catalogue.as_ref().ok().and_then(|catalogue| catalogue.jobs.first()).map(|job| job.name.clone());
            self.past = Some(PastRun {
                job: job.unwrap_or_default(),
                started: Local::now().format("%Y-%m-%d %H:%M").to_string(),
                hours: 0,
                minutes: 0,
                seconds: 0,
                commit: String::new(),
                comment: String::new(),
                error: None,
            });
        }
        if ui
            .add_enabled(!self.copy_target.trim().is_empty(), egui::Button::new("Copy history to the repo"))
            .on_hover_text("writes the runs lab recorded, on every machine, into lab/runs.jsonl of the clone named beside, to commit with the PR that cites them")
            .clicked()
        {
            let message = match self.copy_to_repository() {
                Ok(count) => format!("{count} runs written to lab/runs.jsonl of {}: commit it with the PR that cites them.", self.copy_target.trim()),
                Err(error) => error,
            };
            self.lock().messages.push(message);
        }
        ui.add(
            egui::TextEdit::singleline(&mut self.copy_target)
                .hint_text("a clone of the repository")
                .desired_width(220.0)
                .font(egui::TextStyle::Monospace),
        )
        .on_hover_text("the clone \"Copy history to the repo\" writes into: its lab/runs.jsonl");
    }

    /// The union of the memories, sorted, written to lab/runs.jsonl of the
    /// clone Marc names (SPEC.md, sections 4 and 4b).
    fn copy_to_repository(&self) -> Result<usize, String> {
        let clone = match crate::mirror::Target::parse(&self.copy_target) {
            crate::mirror::Target::Local(path) => path,
            crate::mirror::Target::Reference(text) => std::path::PathBuf::from(text),
        };
        let history = self.history.as_ref().map_err(|error| error.clone())?;
        crate::machines::publish(history, &clone)
    }

    /// The comment and the papers of a run lab recorded, editable.
    pub(super) fn edit_panel(&mut self, ui: &mut egui::Ui, run: &Run) {
        ui.add_space(10.0);
        if let Source::Seeded { from } = &run.source {
            ui.label(RichText::new(format!("Read from the repository: edit {from} there.")).color(MUTED));
            return;
        }
        if self.edit.as_ref().is_none_or(|edit| edit.id != run.id) {
            let first = run.papers.first();
            self.edit = Some(Edit {
                id: run.id.clone(),
                comment: run.comment.clone(),
                paper: first.map(|reference| reference.paper),
                label: first.map(|reference| reference.label.clone()).unwrap_or_default(),
            });
        }
        let Some(edit) = &mut self.edit else { return };
        ui.label(RichText::new("COMMENT AND PAPER").small().strong().color(MUTED));
        ui.add(egui::TextEdit::multiline(&mut edit.comment).desired_rows(2).desired_width(f32::INFINITY));
        ui.horizontal(|ui| {
            ui.radio_value(&mut edit.paper, None, "none");
            ui.radio_value(&mut edit.paper, Some(Paper::A), "A");
            ui.radio_value(&mut edit.paper, Some(Paper::B), "B");
            ui.add_enabled(edit.paper.is_some(), egui::TextEdit::singleline(&mut edit.label).hint_text("label"));
        });
        let changed = edit.comment != run.comment
            || edit
                .paper
                .map(|paper| PaperRef { paper, label: edit.label.trim().to_string() })
                .into_iter()
                .collect::<Vec<_>>()
                != run.papers;
        if ui.add_enabled(changed, egui::Button::new("Save")).clicked() {
            let mut updated = run.clone();
            updated.comment = edit.comment.trim().to_string();
            updated.papers =
                edit.paper.map(|paper| PaperRef { paper, label: edit.label.trim().to_string() }).into_iter().collect();
            updated.edited = Some(Local::now().fixed_offset());
            match crate::store::replace(&self.places.runs(), &updated) {
                Ok(()) => self.read_history(),
                Err(error) => self.lock().messages.push(error),
            }
        }
    }

    pub(super) fn past_run_window(&mut self, context: &egui::Context) {
        let Some(mut past) = self.past.take() else { return };
        let jobs: Vec<String> = self
            .catalogue
            .as_ref()
            .map(|catalogue| catalogue.jobs.iter().map(|job| job.name.clone()).collect())
            .unwrap_or_default();
        let mut open = true;
        let mut save = false;
        egui::Window::new("Record a past run").open(&mut open).resizable(false).show(context, |ui| {
            ui.label(
                RichText::new("A run lab did not launch, as a PR body or a note gives it. Its calm is not known.")
                    .color(MUTED),
            );
            egui::Grid::new("past").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                ui.label("Job");
                egui::ComboBox::from_id_salt("past job").selected_text(past.job.clone()).show_ui(ui, |ui| {
                    for job in &jobs {
                        ui.selectable_value(&mut past.job, job.clone(), job);
                    }
                });
                ui.end_row();
                ui.label("Started");
                ui.add(egui::TextEdit::singleline(&mut past.started).hint_text("2026-10-29 22:10"));
                ui.end_row();
                ui.label("Duration");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut past.hours).suffix(" h"));
                    ui.add(egui::DragValue::new(&mut past.minutes).range(0..=59).suffix(" min"));
                    ui.add(egui::DragValue::new(&mut past.seconds).range(0..=59).suffix(" s"));
                });
                ui.end_row();
                ui.label("Commit");
                ui.add(egui::TextEdit::singleline(&mut past.commit).hint_text("the hash it ran on"));
                ui.end_row();
                ui.label("Comment");
                ui.add(egui::TextEdit::singleline(&mut past.comment).hint_text("where the time comes from"));
                ui.end_row();
            });
            if let Some(error) = &past.error {
                ui.colored_label(RED, error);
            }
            save = ui.button("Record").clicked();
        });
        if save {
            match self.record_past(&past) {
                Ok(()) => return,
                Err(error) => past.error = Some(error),
            }
        }
        if open {
            self.past = Some(past);
        }
    }

    fn record_past(&mut self, past: &PastRun) -> Result<(), String> {
        let naive = NaiveDateTime::parse_from_str(past.started.trim(), "%Y-%m-%d %H:%M")
            .map_err(|_| "Started: write it as 2026-10-29 22:10".to_string())?;
        let started =
            Local.from_local_datetime(&naive).single().ok_or("Started: that time does not exist here")?.fixed_offset();
        let seconds = f64::from(past.hours * 3600 + past.minutes * 60 + past.seconds);
        let typed = past.commit.trim();
        let commit = if typed.is_empty() {
            None
        } else {
            Some(self.mirror.resolve(typed).map_err(|_| format!("Commit: the mirror does not know {typed}"))?)
        };
        let run = Run {
            format: FORMAT,
            id: format!("{}-{}-typed", started.format("%Y%m%d-%H%M%S"), past.job),
            job: past.job.clone(),
            source: Source::Manual,
            queue: None,
            started,
            ended: None,
            seconds: (seconds > 0.0).then_some(seconds),
            status: Status::Ok,
            prepare: None,
            command: String::new(),
            environment: Default::default(),
            parameters: Default::default(),
            checkout: typed.to_string(),
            directory: ".".to_string(),
            git: commit.map(|commit| Git { commit, dirty: false, branch: None, pushed: true }),
            threads: None,
            memory_limit_gb: None,
            machine: None,
            load: None,
            calm: Calm {
                before: false,
                during: false,
                forced: false,
                timing_valid: false,
                reason: "not measured (typed by hand)".to_string(),
            },
            comment: past.comment.trim().to_string(),
            papers: Vec::new(),
            log: None,
            outputs: Vec::new(),
            edited: None,
            cell: None,
            timeout_seconds: None,
            tools: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            cache: None,
        };
        crate::runner::write_record(&self.places, &run)?;
        self.read_history();
        Ok(())
    }
}

fn load1(run: &Run) -> Option<f64> {
    run.load.as_ref().map(|load| load.at_start.load[0])
}

/// The runs as a Markdown table, with the hardware line a paper needs and
/// what the selection mixes.
pub(super) fn markdown(runs: &[&Run], tags: &[Tag], reach: &Reach, label: &str) -> String {
    let mut text =
        String::from("| Job | Run on | Date | Duration | Load at start | Timing |\n|---|---|---|---|---|---|\n");
    for run in runs {
        let timing = if run.calm.timing_valid { "valid" } else { "not valid" };
        text.push_str(&format!(
            "| {} | {} | {} | {} | {} | {timing} |\n",
            run.job,
            place(run, tags, reach),
            run.started.format("%Y-%m-%d"),
            run.seconds.map_or("no time".to_string(), duration_text),
            load1(run).map_or(String::new(), |load| format!("{load:.2}")),
        ));
    }
    let machines: BTreeSet<String> = runs
        .iter()
        .filter_map(|run| run.machine.as_ref())
        .map(|machine| {
            let memory = format!("{:.1} GiB", machine.memory_bytes as f64 / (1u64 << 30) as f64);
            format!("{}, {} threads, {memory}", machine.cpu, machine.threads)
        })
        .collect();
    text.push('\n');
    match machines.len() {
        0 => text.push_str("Hardware: not recorded.\n"),
        1 => text.push_str(&format!("Hardware: {} ({label}).\n", machines.iter().next().expect("one machine"))),
        _ => text.push_str(&format!("Hardware: mixed, {}.\n", machines.into_iter().collect::<Vec<_>>().join("; "))),
    }
    let loads: Vec<f64> = runs.iter().filter_map(|run| load1(run)).collect();
    if let (Some(low), Some(high)) = (loads.iter().copied().reduce(f64::min), loads.iter().copied().reduce(f64::max)) {
        text.push_str(&format!("Load: load1 {low:.2} to {high:.2} at start.\n"));
    }
    let valid = runs.iter().filter(|run| run.calm.timing_valid).count();
    text.push_str(&format!(
        "Timings valid (calm before and during, not forced): {valid} of {}; the others cannot be reported.\n",
        runs.len()
    ));
    let places: BTreeSet<String> = runs
        .iter()
        .map(|run| match tag_of(run, tags) {
            Some(tag) => tag.name.clone(),
            None if crate::state::is_pushed(run, reach) => "off-tag".to_string(),
            None => "orphan".to_string(),
        })
        .collect();
    if places.len() > 1 {
        text.push_str(&format!("Mixed: {}.\n", places.into_iter().collect::<Vec<_>>().join(", ")));
    }
    text
}

pub(super) fn csv(runs: &[&Run], tags: &[Tag], reach: &Reach) -> String {
    let quote = |text: &str| format!("\"{}\"", text.replace('"', "\"\""));
    let mut text = String::from("id,job,run_on,commit,started,seconds,load1,timing_valid,status,comment,papers\n");
    for run in runs {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            quote(&run.id),
            quote(&run.job),
            quote(&place(run, tags, reach)),
            run.git.as_ref().map_or(String::new(), |git| git.commit.clone()),
            run.started.to_rfc3339(),
            run.seconds.map_or(String::new(), |seconds| seconds.to_string()),
            load1(run).map_or(String::new(), |load| load.to_string()),
            run.calm.timing_valid,
            quote(&run.status.text()),
            quote(&run.comment),
            quote(&run.papers_text()),
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_export_says_what_the_selection_mixes() {
        let runs = crate::store::History::parse(include_str!("../../tests/fixtures/runs.jsonl")).runs;
        let shown: Vec<&Run> = runs.iter().collect();
        let tags = [Tag { name: "v0.7.0".to_string(), commit: "e13fadb0".to_string(), date: String::new() }];
        let text = markdown(&shown, &tags, &Reach::new(), "i7-7700K 4c/8t 2017, 62 GB");
        assert!(
            text.contains("| forced-whole-lower | v0.7.0 | 2026-10-31 | 2 h 38 min 24 | 0.43 | not valid |"),
            "{text}"
        );
        assert!(
            text.contains("| forced-whole-lower | off-tag 9683d14 | 2026-11-06 | 2 h 38 min 51 | 0.06 | valid |"),
            "{text}"
        );
        assert!(text.contains("(i7-7700K 4c/8t 2017, 62 GB)"), "{text}");
        assert!(text.contains("Timings valid (calm before and during, not forced): 1 of 3"), "{text}");
        assert!(text.contains("Mixed: off-tag, v0.7.0."), "{text}");
        let table = csv(&shown, &tags, &Reach::new());
        assert_eq!(table.lines().count(), 4);
        assert!(table.contains("\"A made-up run: the fixture of the tests.\""));
    }
}
