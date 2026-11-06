//! lab: the record and the launcher of openclare's computations (SPEC.md).
//!
//!     cargo run --release [-- options]
//!
//! `--data <folder>` reads and writes the runs in another folder than
//! `$XDG_DATA_HOME/openclare-lab` (for example `tests/fixtures`, to see the
//! history on the made-up runs of the tests). `--tab <queue|history|
//! catalogue|machine>` opens on that tab, `--queue <job or sequence>` puts
//! it in the queue (repeatable), `--run <id>` selects that run in the
//! history, `--start` presses Run (or goes on with a queue left paused).
//! `--screenshot <file.ppm>` writes a picture of the window, not of the
//! screen, `--after <seconds>` seconds after opening, and closes it, even
//! during a run, as an app that dies would.

mod app;
mod catalogue;
mod files;
mod home;
mod machine;
mod model;
mod runner;
mod settings;
mod state;
mod store;

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut data = None;
    let mut opening = app::Opening::default();
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        if argument == "--start" {
            opening.start = true;
            continue;
        }
        match (argument.as_str(), arguments.next()) {
            ("--data", Some(folder)) => data = Some(PathBuf::from(folder)),
            ("--tab", Some(name)) if app::Tab::named(&name).is_some() => opening.tab = app::Tab::named(&name),
            ("--queue", Some(name)) => opening.queue.push(name),
            ("--run", Some(id)) => opening.run = Some(id),
            ("--screenshot", Some(file)) => opening.screenshot = Some(PathBuf::from(file)),
            ("--after", Some(seconds)) if seconds.parse::<f64>().is_ok() => {
                opening.after = seconds.parse().unwrap_or(0.0)
            }
            _ => {
                eprintln!(
                    "usage: lab [--data <folder>] [--tab <name>] [--queue <job>]... [--start] [--run <id>] [--screenshot <file.ppm> [--after <seconds>]]"
                );
                return ExitCode::from(2);
            }
        }
    }
    let start = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let places = match home::Places::find(&start, data) {
        Ok(places) => places,
        Err(error) => {
            eprintln!("lab: {error}");
            return ExitCode::FAILURE;
        }
    };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("lab · openclare")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([960.0, 600.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        "openclare-lab",
        options,
        Box::new(|creation| {
            creation.egui_ctx.set_theme(eframe::egui::Theme::Dark);
            Ok(Box::new(app::App::new(places, opening, creation.egui_ctx.clone())))
        }),
    );
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("lab: {error}");
            ExitCode::FAILURE
        }
    }
}
