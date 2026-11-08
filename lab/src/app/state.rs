//! The State screen (SPEC.md, section 11): the last run of each job, on a tag,
//! off-tag or orphan, the tag filter and the machine filter.

use std::collections::BTreeSet;

use eframe::egui::{self, RichText};

use super::{AMBER, App, MUTED, Tab, duration_cell, place_style, stopwatch};
use crate::model::{Paper, Run, Source};
use crate::state::{Class, class, last_runs, place, tag_of};

/// The filters of the screen.
#[derive(Default)]
pub(super) struct StateView {
    /// None: all runs; else the name of a tag.
    tag: Option<String>,
    /// None: every machine; else the runs one machine recorded (SPEC.md, §4b).
    machine: Option<String>,
    show: Show,
    checked: BTreeSet<String>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Show {
    #[default]
    All,
    OnTag,
    OffTag,
    Orphan,
    Never,
    UnderLoad,
}

fn load1(run: &Run) -> Option<f64> {
    run.load.as_ref().map(|load| load.at_start.load[0])
}

impl App {
    pub(super) fn state_tab(&mut self, ui: &mut egui::Ui) {
        let Ok(catalogue) = &self.catalogue else {
            ui.label("No catalogue: see the error above.");
            return;
        };
        let catalogue = catalogue.clone();
        let runs = self.all_runs();
        let tags = self.tags.clone();
        let reach = self.reach.clone();
        let chosen = self.state_view.tag.clone();
        let machine = self.state_view.machine.clone();
        let origin = &self.origin;
        let last = last_runs(&runs, |run| {
            chosen.as_ref().is_none_or(|name| tag_of(run, &tags).is_some_and(|tag| &tag.name == name))
                && machine.as_ref().is_none_or(|machine| origin.get(&run.id) == Some(machine))
        });

        ui.horizontal(|ui| {
            ui.label("Machine");
            let this = self.machines.first().cloned().unwrap_or_default();
            let text = match &machine {
                None => "every machine".to_string(),
                Some(name) if name == &this => format!("{name} (this one)"),
                Some(name) => name.clone(),
            };
            egui::ComboBox::from_id_salt("machine filter").selected_text(text).width(200.0).show_ui(ui, |ui| {
                ui.selectable_value(&mut self.state_view.machine, None, "every machine");
                for name in self.machines.clone() {
                    let label = if name == this { format!("{name} (this one)") } else { name.clone() };
                    ui.selectable_value(&mut self.state_view.machine, Some(name), label);
                }
            });
            ui.add_space(10.0);
            ui.label("Tag");
            let text = chosen.clone().unwrap_or_else(|| "All runs: the last run of each job".to_string());
            egui::ComboBox::from_id_salt("tag filter").selected_text(text).width(320.0).show_ui(ui, |ui| {
                ui.selectable_value(&mut self.state_view.tag, None, "All runs: the last run of each job");
                for tag in &tags {
                    let short: String = tag.commit.chars().take(7).collect();
                    ui.selectable_value(&mut self.state_view.tag, Some(tag.name.clone()), format!("{} · {short} · {}", tag.name, tag.date));
                }
            });
            ui.label(
                RichText::new("A run is on a tag when the one hash it records is the tag's; else it is off-tag, or orphan when no branch nor tag of the remote reaches it; neither can back a paper.")
                    .color(MUTED),
            );
        });
        ui.add_space(6.0);

        let names: Vec<&str> = catalogue.jobs.iter().map(|job| job.name.as_str()).collect();
        let count = |wanted: Class| {
            names.iter().filter(|name| last.get(**name).is_some_and(|run| class(run, &tags, &reach) == wanted)).count()
        };
        let (on_tag, off_tag, orphan) = (count(Class::OnTag), count(Class::OffTag), count(Class::Orphan));
        let (green, blue, amber) = (place_style("v").0, place_style("off-tag").0, place_style("orphan").0);
        let missing: Vec<String> =
            names.iter().filter(|name| !last.contains_key(**name)).map(|name| name.to_string()).collect();
        ui.horizontal(|ui| {
            let big = |ui: &mut egui::Ui, count: usize, colour, text: &str| {
                ui.label(RichText::new(count.to_string()).size(26.0).strong().color(colour));
                ui.label(RichText::new(text).color(MUTED));
                ui.add_space(14.0);
            };
            match &chosen {
                None => {
                    big(ui, on_tag, green, "last run on a tag");
                    big(ui, off_tag, blue, "last run off-tag");
                    big(ui, orphan, amber, "last run orphan");
                    let never = match &machine {
                        Some(name) => format!("never run on {name}"),
                        None => "never run".to_string(),
                    };
                    big(ui, missing.len(), ui.visuals().text_color(), &never);
                }
                Some(name) => {
                    big(ui, on_tag, green, &format!("with a run on {name}"));
                    big(ui, missing.len(), ui.visuals().text_color(), &format!("without a run on {name}"));
                }
            }
            ui.label(RichText::new(format!("of {} jobs", names.len())).color(MUTED));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let label = match &chosen {
                    None => format!("Queue the {} never run", missing.len()),
                    Some(name) => format!("Queue the {} without a run on {name}", missing.len()),
                };
                if ui.add_enabled(!missing.is_empty(), egui::Button::new(label)).clicked() {
                    for name in &missing {
                        self.add_to_queue(name);
                    }
                    self.tab = Tab::Queue;
                }
                let checked: Vec<String> = self.state_view.checked.iter().cloned().collect();
                if ui
                    .add_enabled(
                        !checked.is_empty(),
                        egui::Button::new(format!("Queue the selection ({})", checked.len())),
                    )
                    .clicked()
                {
                    for name in &checked {
                        self.add_to_queue(name);
                    }
                    self.state_view.checked.clear();
                    self.tab = Tab::Queue;
                }
            });
        });
        ui.horizontal(|ui| {
            for (show, text) in [
                (Show::All, "All"),
                (Show::OnTag, "Last run on a tag"),
                (Show::OffTag, "Last run off-tag"),
                (Show::Orphan, "Last run orphan"),
                (Show::Never, if chosen.is_some() { "No run on this tag" } else { "Never run" }),
                (Show::UnderLoad, "Started under load"),
            ] {
                ui.selectable_value(&mut self.state_view.show, show, text);
            }
        });
        ui.separator();

        let show = self.state_view.show;
        let keep = |run: Option<&&Run>| match (show, run) {
            (Show::All, _) => true,
            (Show::OnTag, Some(run)) => class(run, &tags, &reach) == Class::OnTag,
            (Show::OffTag, Some(run)) => class(run, &tags, &reach) == Class::OffTag,
            (Show::Orphan, Some(run)) => class(run, &tags, &reach) == Class::Orphan,
            (Show::Never, None) => true,
            (Show::UnderLoad, Some(run)) => load1(run).is_some_and(|load| load >= 1.0),
            _ => false,
        };
        let groups = [
            (Some(Paper::B), "Paper B · Turns of a T-mesh"),
            (Some(Paper::A), "Paper A · Closing edge loops"),
            (None, "Builds, tests, bench tools"),
        ];
        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            for (paper, title) in groups {
                let jobs: Vec<&crate::catalogue::Job> = catalogue
                    .jobs
                    .iter()
                    .filter(|job| job.paper == paper)
                    .filter(|job| keep(last.get(&job.name)))
                    .collect();
                if jobs.is_empty() {
                    continue;
                }
                ui.add_space(10.0);
                ui.label(RichText::new(title).strong().size(15.0));
                egui::Grid::new(("state", title)).striped(true).num_columns(7).spacing([14.0, 6.0]).show(ui, |ui| {
                    for heading in ["", "Job", "Last run", "Date", "Duration", "Load at start", "Read from"] {
                        ui.label(RichText::new(heading).small().strong().color(MUTED));
                    }
                    ui.end_row();
                    for job in jobs {
                        let mut checked = self.state_view.checked.contains(&job.name);
                        if ui.checkbox(&mut checked, "").changed() {
                            if checked {
                                self.state_view.checked.insert(job.name.clone());
                            } else {
                                self.state_view.checked.remove(&job.name);
                            }
                        }
                        ui.horizontal(|ui| {
                            ui.label(&job.name);
                            stopwatch(ui, job.timing_matters);
                        });
                        match last.get(&job.name) {
                            Some(run) => {
                                let place = place(run, &tags, &reach);
                                let (colour, hint) = place_style(&place);
                                ui.label(RichText::new(place).monospace().color(colour)).on_hover_text(hint);
                                ui.monospace(run.started.format("%Y-%m-%d").to_string());
                                duration_cell(ui, run, job.timing_matters);
                                match load1(run) {
                                    Some(load) => {
                                        let colour = if load >= 1.0 { AMBER } else { ui.visuals().text_color() };
                                        ui.label(RichText::new(format!("{load:.2}")).monospace().color(colour));
                                    }
                                    None => {
                                        ui.label("");
                                    }
                                }
                                let from = match &run.source {
                                    Source::Seeded { from } => from.clone(),
                                    Source::App => format!("lab, run {}", run.id),
                                    Source::Manual => "typed by hand".to_string(),
                                };
                                ui.label(RichText::new(from).monospace().small().color(MUTED));
                            }
                            None => {
                                let text = match &chosen {
                                    None => "never run".to_string(),
                                    Some(name) => format!("no run on {name}"),
                                };
                                ui.label(RichText::new(text).color(MUTED));
                                for _ in 0..4 {
                                    ui.label("");
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
            }
        });
    }
}
