//! lab: the record and the launcher of openclare's computations (SPEC.md).
//!
//!     lab [options]                      the window
//!     lab [options] run <job> [--ref <reference>] [--force] [--threads <n>] [--push]
//!     lab [options] run <batch> [--axis <name>=<value>,…]… [--timeout <duration>] [--again] [--ref <reference>] [--force] [--threads <n>] [--push]
//!     lab [options] report <batch> [--launch <id>] [--ref <reference>]
//!     lab [options] fetch
//!     lab [options] push
//!     lab [options] status
//!
//! `--data <folder>` uses another folder than `~/.openclare-lab` (for
//! example `tests/fixtures`, to see the history on the made-up runs of the
//! tests); without it, the first start moves the files of the earlier
//! folders, `~/.local/share/openclare-lab` and `~/.config/openclare-lab`,
//! into `~/.openclare-lab` once. `--catalogue <file>` reads the catalogue
//! from that file in place of `lab/jobs.toml` at `main` of the mirror.
//!
//! The window takes `--tab <state|queue|batches|history|catalogue|machine>`,
//! `--queue <job, batch or sequence>` (repeatable), `--run <id>` to select that
//! run in the history, `--batch <name>` to show that batch's grid, `--start` to press Run (or go on with a queue left paused), and
//! `--screenshot <file.ppm> [--after <seconds>]`, a picture of the window,
//! not of the screen, taken that many seconds after opening, then the window
//! closes, even during a run, as an app that dies would.
//!
//! `run` runs one job without a window and exits with its status (§14):
//! on the reference given, else the job's own; `--force` starts it although
//! the machine is not calm when the `wait` calm setting makes it wait, its
//! time then not counting; `--push` pushes the
//! live memory to this machine's branch after the run, whatever the setting.
//! `run` on a batch (§6b) runs its grid, one line per job that starts and
//! ends; `--axis model=ogre,pig` keeps those values of an axis (any reference
//! for `ref`), `--timeout 20s` replaces the timeout of each of its jobs, and
//! a job done before (same command, commit, tools and dependencies, ended ok)
//! is skipped unless `--again`; it exits 0 when every job is ok.
//! `report` writes the last record of each cell of a batch to records.json and
//! runs the batch's `report` on it (§6b, R4), printing its table.
//! `fetch` fetches the mirror, creating it if need be. `push` fetches, then
//! pushes the live memory. `status` says where things stand.

mod app;
mod batch;
mod catalogue;
mod cpus;
mod desktop;
mod files;
mod headless;
mod home;
mod machine;
mod machines;
mod mirror;
mod model;
mod monitor;
mod report;
mod runner;
mod settings;
mod state;
mod store;
mod tools;

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: lab [--data <folder>] [--catalogue <file>] [--tab <name>] [--queue <job>]... [--start] [--run <id>] [--batch <name>] [--screenshot <file.ppm> [--after <seconds>]]
       lab [--data <folder>] [--catalogue <file>] run <job or batch> [--ref <reference>] [--force] [--threads <n>] [--push] [--timeout <90s|30m|2h>] [--again] [--axis <name>=<value>,...]...
       lab [--data <folder>] [--catalogue <file>] report <batch> [--launch <id>] [--ref <reference or folder>]
       lab [--data <folder>] fetch
       lab [--data <folder>] push
       lab [--data <folder>] status";

enum Command {
    Window,
    Run(headless::RunArguments),
    Report { batch: String, launch: Option<String>, reference: Option<String> },
    Fetch,
    Push,
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
            ("--again", Some(Command::Run(run))) => run.again = true,
            ("--push", Some(Command::Run(run))) => run.push = true,
            ("run", None) => match arguments.next() {
                Some(job) => {
                    command = Some(Command::Run(headless::RunArguments {
                        job,
                        reference: None,
                        force: false,
                        threads: None,
                        push: false,
                        only: Vec::new(),
                        timeout: None,
                        again: false,
                    }))
                }
                None => return usage(),
            },
            ("report", None) => match arguments.next() {
                Some(batch) => command = Some(Command::Report { batch, launch: None, reference: None }),
                None => return usage(),
            },
            ("fetch", None) => command = Some(Command::Fetch),
            ("push", None) => command = Some(Command::Push),
            ("status", None) => command = Some(Command::Status),
            _ => match (argument.as_str(), arguments.next(), &mut command) {
                ("--data", Some(folder), _) => data = Some(PathBuf::from(folder)),
                ("--catalogue", Some(file), _) => catalogue = Some(PathBuf::from(file)),
                ("--ref", Some(reference), Some(Command::Run(run))) => run.reference = Some(reference),
                ("--ref", Some(given), Some(Command::Report { reference, .. })) => *reference = Some(given),
                ("--launch", Some(given), Some(Command::Report { launch, .. })) => *launch = Some(given),
                ("--threads", Some(count), Some(Command::Run(run))) if count.parse::<u32>().is_ok_and(|n| n > 0) => {
                    run.threads = count.parse().ok()
                }
                ("--timeout", Some(text), Some(Command::Run(run))) => match catalogue::Timeout::parse(&text) {
                    Ok(timeout) => run.timeout = Some(timeout),
                    Err(error) => {
                        eprintln!("lab: {error}");
                        return usage();
                    }
                },
                ("--axis", Some(text), Some(Command::Run(run))) => match text.split_once('=') {
                    Some((name, values)) if !values.is_empty() => {
                        run.only.push((name.to_string(), values.split(',').map(str::to_string).collect()))
                    }
                    _ => return usage(),
                },
                ("--tab", Some(name), None) if app::Tab::named(&name).is_some() => opening.tab = app::Tab::named(&name),
                ("--queue", Some(name), None) => opening.queue.push(name),
                ("--run", Some(id), None) => opening.run = Some(id),
                ("--batch", Some(name), None) => opening.batch = Some(name),
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
    // `lab status | head` ends quietly, as a shell command does.
    // Safety: the default disposition of SIGPIPE, set before any thread starts.
    unsafe {
        let _ = nix::sys::signal::signal(nix::sys::signal::Signal::SIGPIPE, nix::sys::signal::SigHandler::SigDfl);
    }
    match command.unwrap_or(Command::Window) {
        Command::Run(arguments) => headless::run(&places, arguments),
        Command::Report { batch, launch, reference } => {
            headless::report(&places, &batch, launch.as_deref(), reference.as_deref())
        }
        Command::Fetch => headless::fetch(&places),
        Command::Push => headless::push(&places),
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
