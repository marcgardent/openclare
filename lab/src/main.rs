//! lab: the record and the launcher of openclare's computations (SPEC.md).
//!
//!     cargo run --release [-- options]
//!
//! `--data <folder>` reads and writes the runs in another folder than
//! `$XDG_DATA_HOME/openclare-lab` (for example `tests/fixtures`, to see the
//! history on the made-up runs of the tests). `--tab <queue|history|
//! catalogue|machine>` opens on that tab, `--queue <job or sequence>` puts
//! it in the queue (repeatable), `--run <id>` selects that run in the
//! history. `--screenshot <file.ppm>` writes a picture of the window, not of
//! the screen, and closes it.

mod app;
mod catalogue;
mod home;
mod model;
mod settings;
mod store;

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut data = None;
    let mut opening = app::Opening::default();
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        let value = arguments.next();
        match (argument.as_str(), value) {
            ("--data", Some(folder)) => data = Some(PathBuf::from(folder)),
            ("--tab", Some(name)) if app::Tab::named(&name).is_some() => opening.tab = app::Tab::named(&name),
            ("--queue", Some(name)) => opening.queue.push(name),
            ("--run", Some(id)) => opening.run = Some(id),
            ("--screenshot", Some(file)) => opening.screenshot = Some(PathBuf::from(file)),
            _ => {
                eprintln!(
                    "usage: lab [--data <folder>] [--tab <name>] [--queue <job>]... [--run <id>] [--screenshot <file.ppm>]"
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
            Ok(Box::new(app::App::new(places, opening)))
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
