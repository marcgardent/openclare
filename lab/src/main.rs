//! lab: the record and the launcher of openclare's computations (SPEC.md).
//!
//!     lab [options]                      the window
//!     lab [options] run <job> [--ref <reference>] [--force] [--threads <n>]
//!     lab [options] fetch
//!     lab [options] status
//!
//! `--data <folder>` uses another folder than `~/.openclare-lab` (for
//! example `tests/fixtures`, to see the history on the made-up runs of the
//! tests); without it, the first start moves the files of the earlier
//! folders, `~/.local/share/openclare-lab` and `~/.config/openclare-lab`,
//! into `~/.openclare-lab` once. `--catalogue <file>` reads the catalogue
//! from that file in place of `lab/jobs.toml` at `main` of the mirror.
//!
//! The window takes `--tab <state|queue|history|catalogue|machine>`, `--queue
//! <job or sequence>` (repeatable), `--run <id>` to select that run in the
//! history, `--start` to press Run (or go on with a queue left paused), and
//! `--screenshot <file.ppm> [--after <seconds>]`, a picture of the window,
//! not of the screen, taken that many seconds after opening, then the window
//! closes, even during a run, as an app that dies would.
//!
//! `run` runs one job without a window and exits with its status (§14):
//! on the reference given, else the job's own; `--force` starts it although
//! the machine is not calm, its time then not counting. `fetch` fetches the
//! mirror, creating it if need be. `status` says where things stand.

mod app;
mod catalogue;
mod files;
mod headless;
mod home;
mod machine;
mod mirror;
mod model;
mod monitor;
mod runner;
mod settings;
mod state;
mod store;

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: lab [--data <folder>] [--catalogue <file>] [--tab <name>] [--queue <job>]... [--start] [--run <id>] [--screenshot <file.ppm> [--after <seconds>]]
       lab [--data <folder>] [--catalogue <file>] run <job> [--ref <reference>] [--force] [--threads <n>]
       lab [--data <folder>] fetch
       lab [--data <folder>] status";

enum Command {
    Window,
    Run(headless::RunArguments),
    Fetch,
    Status,
}

fn main() -> ExitCode {
    let mut data = None;
    let mut catalogue = None;
    let mut opening = app::Opening::default();
    let mut command: Option<Command> = None;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match (argument.as_str(), &mut command) {
            ("--start", None) => opening.start = true,
            ("--force", Some(Command::Run(run))) => run.force = true,
            ("run", None) => match arguments.next() {
                Some(job) => {
                    command =
                        Some(Command::Run(headless::RunArguments { job, reference: None, force: false, threads: None }))
                }
                None => return usage(),
            },
            ("fetch", None) => command = Some(Command::Fetch),
            ("status", None) => command = Some(Command::Status),
            _ => match (argument.as_str(), arguments.next(), &mut command) {
                ("--data", Some(folder), _) => data = Some(PathBuf::from(folder)),
                ("--catalogue", Some(file), _) => catalogue = Some(PathBuf::from(file)),
                ("--ref", Some(reference), Some(Command::Run(run))) => run.reference = Some(reference),
                ("--threads", Some(count), Some(Command::Run(run))) if count.parse::<u32>().is_ok_and(|n| n > 0) => {
                    run.threads = count.parse().ok()
                }
                ("--tab", Some(name), None) if app::Tab::named(&name).is_some() => opening.tab = app::Tab::named(&name),
                ("--queue", Some(name), None) => opening.queue.push(name),
                ("--run", Some(id), None) => opening.run = Some(id),
                ("--screenshot", Some(file), None) => opening.screenshot = Some(PathBuf::from(file)),
                ("--after", Some(seconds), None) if seconds.parse::<f64>().is_ok() => {
                    opening.after = seconds.parse().unwrap_or(0.0)
                }
                _ => return usage(),
            },
        }
    }
    if data.is_none()
        && let Some(lab) = home::default_folder()
    {
        match home::migrate(&lab, &home::earlier_folders()) {
            Ok(said) => {
                for line in &said {
                    eprintln!("lab: {line}");
                }
                opening.messages = said;
            }
            Err(error) => {
                eprintln!("lab: {error}");
                return ExitCode::FAILURE;
            }
        }
    }
    let start = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let places = match home::Places::find(&start, data, catalogue) {
        Ok(places) => places,
        Err(error) => {
            eprintln!("lab: {error}");
            return ExitCode::FAILURE;
        }
    };
    match command.unwrap_or(Command::Window) {
        Command::Run(arguments) => headless::run(&places, arguments),
        Command::Fetch => headless::fetch(&places),
        Command::Status => headless::status(&places),
        Command::Window => window(places, opening),
    }
}

fn usage() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

fn window(places: home::Places, opening: app::Opening) -> ExitCode {
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
