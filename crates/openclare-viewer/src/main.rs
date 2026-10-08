//! The Rerun viewer, built in: opens the recordings that
//! `openclare remesh --record` writes, with nothing else to install and the
//! version that goes with them.

use std::fs::File;
use std::io::BufReader;
use std::process::ExitCode;

use rerun::external::re_log_encoding::DecoderApp;

const USAGE: &str = "usage: openclare-view <FILE.rrd>...

Opens recordings written by `openclare remesh --record <FILE.rrd>`.";

fn main() -> ExitCode {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() || paths.iter().any(|path| path.starts_with('-')) {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    match view(&paths) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn view(paths: &[String]) -> Result<(), String> {
    let mut messages = Vec::new();
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
    rerun::native_viewer::show(token, messages).map_err(|error| error.to_string())
}
