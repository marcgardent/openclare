//! The versions of the tools a run depends on (SPEC.md, §6b): Blender,
//! rustc and Python, asked in the checkout; Lean, the toolchain the
//! checkout's `lean-toolchain` names, read and not run: elan would download
//! a toolchain it lacks. A tool not found is left out.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// How long a `--version` may take: Blender starts in about a second.
const PATIENCE: Duration = Duration::from_secs(60);

/// The map a record keeps, read in `checkout`.
pub fn read(checkout: &Path) -> BTreeMap<String, String> {
    let mut tools = BTreeMap::new();
    let asked = [("blender", "blender"), ("rustc", "rustc"), ("python", "python3")];
    for (name, program) in asked {
        if let Some(version) = output(checkout, program, &["--version"]).as_deref().and_then(version_in) {
            tools.insert(name.to_string(), version);
        }
    }
    if let Some(version) = lean_toolchain(checkout) {
        tools.insert("lean".to_string(), version);
    }
    tools
}

/// The first line of `program --version` run in `directory`, or None.
fn output(directory: &Path, program: &str, arguments: &[&str]) -> Option<String> {
    let mut child = Command::new(program)
        .args(arguments)
        .current_dir(directory)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let begun = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if begun.elapsed() < PATIENCE => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
    let mut text = String::new();
    std::io::Read::read_to_string(&mut child.stdout.take()?, &mut text).ok()?;
    text.lines().find(|line| !line.trim().is_empty()).map(str::to_string)
}

/// The version number in a first line: "Blender 5.3.0", "rustc 1.95.0
/// (59807616e 2026-04-14)", "Python 3.14.4" give "5.3.0", "1.95.0", "3.14.4".
pub fn version_in(line: &str) -> Option<String> {
    line.split_whitespace()
        .find(|word| word.chars().next().is_some_and(|c| c.is_ascii_digit()) && word.contains('.'))
        .map(str::to_string)
}

/// The Lean toolchain of the checkout: the first tracked `lean-toolchain`,
/// "leanprover/lean4:v4.35.0" giving "v4.35.0".
fn lean_toolchain(checkout: &Path) -> Option<String> {
    let listed = Command::new("git")
        .arg("-C")
        .arg(checkout)
        .args(["ls-files", "--", "lean-toolchain", "*/lean-toolchain"])
        .output()
        .ok()?;
    let path = String::from_utf8_lossy(&listed.stdout).lines().next()?.to_string();
    toolchain_version(&std::fs::read_to_string(checkout.join(path)).ok()?)
}

pub fn toolchain_version(text: &str) -> Option<String> {
    let line = text.lines().map(str::trim).find(|line| !line.is_empty())?;
    Some(line.rsplit(':').next().unwrap_or(line).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_read_from_their_first_lines() {
        assert_eq!(version_in("Blender 5.3.0").as_deref(), Some("5.3.0"));
        assert_eq!(version_in("rustc 1.95.0 (59807616e 2026-04-14)").as_deref(), Some("1.95.0"));
        assert_eq!(version_in("Python 3.14.4").as_deref(), Some("3.14.4"));
        assert_eq!(version_in("no number here"), None);
        assert_eq!(toolchain_version("leanprover/lean4:v4.35.0\n").as_deref(), Some("v4.35.0"));
        assert_eq!(toolchain_version("\n"), None);
    }

    /// This repository: Lean from its file, Python and rustc on any machine that builds lab.
    #[test]
    fn the_tools_of_this_checkout() {
        let tools = read(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap());
        assert!(tools.contains_key("rustc"), "{tools:?}");
        assert!(tools.get("lean").is_some_and(|lean| lean.starts_with('v')), "{tools:?}");
    }
}
