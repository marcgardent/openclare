//! The Machine screen (SPEC.md, section 11): CPU, load, memory and GPU, the
//! heaviest processes, the calm indicator with its thresholds, and the
//! checkouts of the mirror with their size and the Clean button.

use std::path::PathBuf;

use eframe::egui::{self, Color32, RichText};

use super::{AMBER, App, GREEN, MUTED, RED, place_style};
use crate::monitor::{Calmness, Point, lock};

/// A line of the last ten minutes, the oldest on the left.
fn sparkline(ui: &mut egui::Ui, values: &[f64], most: f64, colour: Color32, limit: Option<f64>) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width().min(520.0), 70.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, Color32::from_gray(60)), egui::StrokeKind::Inside);
    let y = |value: f64| rect.bottom() - (value / most).clamp(0.0, 1.0) as f32 * rect.height();
    if let Some(limit) = limit {
        painter.hline(rect.x_range(), y(limit), egui::Stroke::new(1.0, GREEN.gamma_multiply(0.5)));
    }
    if values.len() < 2 {
        return;
    }
    let step = rect.width() / (values.len() - 1) as f32;
    let points: Vec<egui::Pos2> = values
        .iter()
        .enumerate()
        .map(|(index, value)| egui::pos2(rect.left() + step * index as f32, y(*value)))
        .collect();
    painter.add(egui::Shape::line(points, egui::Stroke::new(1.5, colour)));
}

fn bar(ui: &mut egui::Ui, fraction: f64, text: String, colour: Color32) {
    ui.add(egui::ProgressBar::new(fraction.clamp(0.0, 1.0) as f32).fill(colour).text(text).desired_width(360.0));
}

fn gibibytes(bytes: u64) -> String {
    format!("{:.1} GiB", bytes as f64 / (1u64 << 30) as f64)
}

impl App {
    pub(super) fn machine_tab(&mut self, ui: &mut egui::Ui) {
        let (points, heaviest, verdict, group) = {
            let watch = lock(&self.watch);
            let points: Vec<Point> = watch.points.iter().cloned().collect();
            (points, watch.heaviest.clone(), watch.verdict(&self.settings.calm), !watch.groups.is_empty())
        };
        let Some(last) = points.last().cloned() else {
            ui.add_space(20.0);
            ui.label("Measuring: the first sample comes in a few seconds.");
            return;
        };
        ui.columns(3, |columns| {
            let ui = &mut columns[0];
            ui.label(RichText::new("CPU").small().strong().color(MUTED));
            ui.label(RichText::new(format!("{:.0} %", last.cpu)).size(26.0).strong());
            ui.label(RichText::new(format!("ours {:.0} % · foreign {:.0} %", last.ours, last.foreign)).color(MUTED));
            let foreign: Vec<f64> = points.iter().map(|point| point.foreign).collect();
            sparkline(ui, &foreign, 100.0, RED, Some(self.settings.calm.foreign_cpu_mean_max));
            ui.label(
                RichText::new("foreign CPU, last 10 minutes · green: the threshold of the mean").small().color(MUTED),
            );
            ui.add_space(8.0);
            ui.label(RichText::new("PER THREAD").small().strong().color(MUTED));
            for (index, percent) in last.per_thread.iter().enumerate() {
                bar(ui, percent / 100.0, format!("cpu {index} · {percent:.0} %"), Color32::from_rgb(217, 119, 123));
            }

            let ui = &mut columns[1];
            ui.label(RichText::new("LOAD AND MEMORY").small().strong().color(MUTED));
            ui.label(
                RichText::new(format!("load {:.2} · {:.2} · {:.2}", last.load[0], last.load[1], last.load[2]))
                    .monospace()
                    .size(16.0),
            );
            let loads: Vec<f64> = points.iter().map(|point| point.load[0]).collect();
            let most = loads.iter().copied().fold(2.0, f64::max);
            sparkline(ui, &loads, most, Color32::from_rgb(159, 182, 217), Some(self.settings.calm.load1_max));
            ui.add_space(6.0);
            bar(
                ui,
                last.memory_used as f64 / last.memory_total.max(1) as f64,
                format!("RAM {} / {}", gibibytes(last.memory_used), gibibytes(last.memory_total)),
                Color32::from_rgb(127, 156, 201),
            );
            match (last.vram, last.gpu) {
                (Some((used, total)), Some(gpu)) => bar(
                    ui,
                    used as f64 / total.max(1) as f64,
                    format!("VRAM {} / {} · GPU {gpu:.0} %", gibibytes(used), gibibytes(total)),
                    Color32::from_rgb(127, 156, 201),
                ),
                _ => {
                    ui.label(RichText::new("VRAM n/a: nvidia-smi did not answer").color(MUTED));
                }
            }
            ui.add_space(8.0);
            ui.label(RichText::new("HEAVIEST PROCESSES").small().strong().color(MUTED));
            egui::Grid::new("processes").num_columns(3).striped(true).show(ui, |ui| {
                for process in heaviest.iter().take(10) {
                    ui.monospace(&process.name);
                    ui.monospace(format!("{:.0} %", process.cpu));
                    if process.ours {
                        ui.colored_label(AMBER, "ours");
                    } else {
                        ui.colored_label(MUTED, "foreign");
                    }
                    ui.end_row();
                }
            });
            ui.label(RichText::new("CPU of one thread; lab itself counts as foreign.").small().color(MUTED));

            let ui = &mut columns[2];
            ui.label(RichText::new("CALM INDICATOR").small().strong().color(MUTED));
            let colour = match verdict.calmness {
                Calmness::Calm => GREEN,
                Calmness::Ours => AMBER,
                Calmness::Busy => RED,
                Calmness::Measuring => MUTED,
            };
            ui.colored_label(colour, &verdict.text);
            if group {
                ui.label(RichText::new("a run of ours is under way: load1 is not looked at").color(MUTED));
            }
            ui.add_space(8.0);
            let mut calm = self.settings.calm.clone();
            let mut comfort = self.settings.comfort_threads;
            egui::Grid::new("thresholds").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                ui.label("Window, seconds");
                ui.add(egui::DragValue::new(&mut calm.window_seconds).range(10..=600));
                ui.end_row();
                ui.label("Foreign CPU, mean below %");
                ui.add(egui::DragValue::new(&mut calm.foreign_cpu_mean_max).range(0.5..=100.0).speed(0.5));
                ui.end_row();
                ui.label("Foreign CPU, every sample below %");
                ui.add(egui::DragValue::new(&mut calm.foreign_cpu_peak_max).range(1.0..=100.0).speed(0.5));
                ui.end_row();
                ui.label("load1 below, no run of ours");
                ui.add(egui::DragValue::new(&mut calm.load1_max).range(0.1..=8.0).speed(0.05));
                ui.end_row();
                ui.label("load1 ignored after our run, minutes");
                ui.add(egui::DragValue::new(&mut calm.load1_ignored_minutes).range(0..=30));
                ui.end_row();
                ui.label("Comfort limit, threads");
                ui.add(egui::DragValue::new(&mut comfort).range(1..=64));
                ui.end_row();
            });
            if calm != self.settings.calm || comfort != self.settings.comfort_threads {
                self.settings.calm = calm.clone();
                self.settings.comfort_threads = comfort;
                *self.calm.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = calm;
                if let Err(error) = self.settings.save(&self.places.settings()) {
                    self.settings_error = Some(error);
                }
            }
            ui.label(
                RichText::new(format!(
                    "Sampled every {} s (settings.toml, sample_seconds; read at start).",
                    self.settings.sample_seconds
                ))
                .small()
                .color(MUTED),
            );
            ui.add_space(14.0);
            self.checkouts_panel(ui);
        });
    }

    /// The checkouts of the mirror, their size measured on a thread of its
    /// own, and the Clean button that removes those off any tag (Q16:
    /// nothing is removed on its own).
    fn checkouts_panel(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("CHECKOUTS OF THE MIRROR").small().strong().color(MUTED));
        let checkouts = self.mirror.checkouts();
        if checkouts.is_empty() {
            ui.label(RichText::new("none yet: the first run on a reference checks its commit out").color(MUTED));
            return;
        }
        let sizes = self.checkout_sizes.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
        let in_use: Option<PathBuf> = {
            let shared = self.lock();
            shared.current.as_ref().and_then(|current| {
                shared.items.iter().find(|item| item.key == current.key).map(|item| item.target.clone())
            })
        }
        .and_then(|target| self.resolution(&target).ok())
        .and_then(|resolution| checkouts.iter().find(|checkout| checkout.head == resolution.commit))
        .map(|checkout| checkout.path.clone());
        let mut off_tag: Vec<PathBuf> = Vec::new();
        let gibibytes = |bytes: u64| format!("{:.1} GiB", bytes as f64 / (1u64 << 30) as f64);
        egui::Grid::new("checkouts").num_columns(4).striped(true).spacing([12.0, 4.0]).show(ui, |ui| {
            for checkout in &checkouts {
                let short: String = checkout.head.chars().take(7).collect();
                let tag = self.tags.iter().find(|tag| tag.commit == checkout.head);
                let place = match tag {
                    Some(tag) => tag.name.clone(),
                    None if self.mirror.pushed(&checkout.head) => format!("off-tag {short}"),
                    None => format!("orphan {short}"),
                };
                if tag.is_none() && in_use.as_ref() != Some(&checkout.path) {
                    off_tag.push(checkout.path.clone());
                }
                let (colour, hint) = place_style(&place);
                ui.monospace(
                    checkout.path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(),
                );
                ui.colored_label(colour, place).on_hover_text(hint);
                let size = sizes.as_ref().and_then(|sizes| sizes.iter().find(|(path, _)| path == &checkout.path));
                ui.monospace(size.map_or("…".to_string(), |(_, bytes)| gibibytes(*bytes)));
                let prepared = std::fs::read_to_string(checkout.path.join(crate::runner::PREPARED))
                    .map(|text| text.lines().count())
                    .unwrap_or(0);
                let mut notes = Vec::new();
                if prepared > 0 {
                    notes.push(format!("{prepared} prepare{} done", if prepared == 1 { "" } else { "s" }));
                }
                if in_use.as_ref() == Some(&checkout.path) {
                    notes.push("a run under way".to_string());
                }
                ui.label(RichText::new(notes.join(" · ")).color(MUTED));
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Measure the sizes").clicked() {
                let paths: Vec<PathBuf> = checkouts.iter().map(|checkout| checkout.path.clone()).collect();
                let sizes = self.checkout_sizes.clone();
                let context = ui.ctx().clone();
                std::thread::spawn(move || {
                    let measured: Vec<(PathBuf, u64)> =
                        paths.into_iter().map(|path| (path.clone(), crate::mirror::size(&path))).collect();
                    *sizes.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(measured);
                    context.request_repaint();
                });
            }
            let total: Option<u64> = sizes
                .as_ref()
                .map(|sizes| sizes.iter().filter(|(path, _)| off_tag.contains(path)).map(|(_, bytes)| bytes).sum());
            let label = match total {
                Some(bytes) => {
                    format!("Clean: remove the {} checkouts off any tag ({})", off_tag.len(), gibibytes(bytes))
                }
                None => format!("Clean: remove the {} checkouts off any tag", off_tag.len()),
            };
            if ui.add_enabled(!off_tag.is_empty(), egui::Button::new(label)).clicked() {
                self.clean_dialog = true;
            }
        });
        if self.clean_dialog {
            let mut open = true;
            egui::Window::new("Clean the checkouts").collapsible(false).resizable(false).show(ui.ctx(), |ui| {
                ui.label(format!(
                    "Remove {} checkouts off any tag, with what was built in them (target/, .lake/)? The tags' checkouts and a run under way stay. A removed commit is checked out again at its next run.",
                    off_tag.len()
                ));
                for path in &off_tag {
                    ui.monospace(path.display().to_string());
                }
                ui.horizontal(|ui| {
                    if ui.button("Keep them").clicked() {
                        open = false;
                    }
                    if ui.button(RichText::new("Remove").color(RED)).clicked() {
                        for path in &off_tag {
                            if let Err(error) = self.mirror.remove_checkout(path) {
                                self.lock().messages.push(format!("clean: {error}"));
                            }
                        }
                        self.lock().mirror_changes += 1;
                        *self.checkout_sizes.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
                        open = false;
                    }
                });
            });
            self.clean_dialog = open;
        }
    }
}
