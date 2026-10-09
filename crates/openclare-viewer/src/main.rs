//! The Rerun viewer, built in: opens the recordings that
//! `openclare remesh --record` writes, with nothing else to install and the
//! version that goes with them.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::ExitCode;

use rerun::external::re_log_encoding::DecoderApp;
use rerun::external::re_log_types::LogMsg;
use rerun::external::re_sdk_types::blueprint::components::PanelState;
use rerun::external::re_viewer::{self, external::re_log_channel, external::re_log_types};

const USAGE: &str = "usage: openclare-view [--screenshot <FILE.png>] [--size <W>x<H>] <FILE.rrd>...

Opens recordings written by `openclare remesh --record <FILE.rrd>`.

    --screenshot <FILE.png>   open the window, take a picture of it once the recording is shown, and quit
    --size <W>x<H>            size of the window in points (default 1600x1000); the picture has twice as many pixels";

fn main() -> ExitCode {
    let mut paths: Vec<String> = Vec::new();
    let (mut screenshot, mut size) = (None::<PathBuf>, [1600.0f32, 1000.0]);
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--screenshot" => match arguments.next() {
                Some(path) => screenshot = Some(path.into()),
                None => return usage(),
            },
            "--size" => {
                let given = arguments.next().unwrap_or_default();
                let parsed: Vec<f32> = given.split('x').filter_map(|side| side.parse().ok()).collect();
                match parsed[..] {
                    [width, height] if width > 0.0 && height > 0.0 => size = [width, height],
                    _ => return usage(),
                }
            }
            other if other.starts_with('-') => return usage(),
            _ => paths.push(argument),
        }
    }
    if paths.is_empty() {
        return usage();
    }
    match view(&paths, screenshot, size) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn usage() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

fn view(paths: &[String], screenshot: Option<PathBuf>, size: [f32; 2]) -> Result<(), String> {
    let mut messages: Vec<LogMsg> = Vec::new();
    for path in paths {
        let file = File::open(path).map_err(|error| format!("{path}: {error}"))?;
        for message in DecoderApp::decode_lazy(BufReader::new(file)) {
            messages.push(message.map_err(|error| format!("{path}: {error}"))?);
        }
    }
    if messages.is_empty() {
        return Err("nothing recorded in there".into());
    }
    // This is the main thread, as the viewer asks.
    let token = rerun::MainThreadToken::i_promise_i_am_on_the_main_thread();
    // What `rerun::native_viewer::show` does, with options of our own: a
    // picture taken and the window closed, when one is asked for.
    let picture = screenshot.is_some();
    let mut startup = re_viewer::StartupOptions {
        screenshot_to_path_then_quit: screenshot,
        resolution_in_points: Some(size),
        hide_welcome_screen: true,
        // A picture is of the recording, not of what was open last time.
        persist_state: !picture,
        ..Default::default()
    };
    if picture {
        // And of what is recorded, not of the panels round it.
        let hidden = Some(PanelState::Hidden);
        let panels = &mut startup.panel_state_overrides;
        (panels.top, panels.blueprint, panels.selection, panels.time) = (hidden, hidden, hidden, hidden);
    }
    let source = re_log_types::StoreSource::RustSdk { rustc_version: "unknown".into(), llvm_version: "unknown".into() };
    let runtime = tokio::runtime::Runtime::new().map_err(|error| error.to_string())?;
    let (sender, receiver) = re_log_channel::log_channel(re_log_channel::LogSource::Sdk);
    for message in messages {
        sender.send(message.into()).ok();
    }
    let backend = startup.force_wgpu_backend.clone();
    re_viewer::run_native_app(
        token,
        Box::new(move |context| {
            let mut app = re_viewer::App::new(
                token,
                re_viewer::build_info(),
                re_viewer::AppEnvironment::from_store_source(&source),
                startup,
                context,
                None,
                re_viewer::AsyncRuntimeHandle::new_native(runtime.handle().clone()),
            );
            app.add_log_receiver(receiver);
            Ok(Box::new(app))
        }),
        backend.as_deref(),
    )
    .map_err(|error| error.to_string())
}
