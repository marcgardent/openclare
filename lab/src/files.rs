//! Files: the outputs of a run, their hashes, the tail of a log, paths made
//! relative, records appended.

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

use sha2::{Digest, Sha256};

use crate::model::FileRef;

/// Hashes are taken of files up to this size (SPEC.md, section 5).
const LARGEST_HASHED: u64 = 1 << 30;

/// The files under `root` matching `pattern`, a path relative to `root`
/// whose parts may hold `*` (any characters but `/`). No `**`.
pub fn glob(root: &Path, pattern: &str) -> Vec<PathBuf> {
    let mut found = vec![PathBuf::new()];
    for part in pattern.split('/').filter(|part| !part.is_empty() && *part != ".") {
        let mut next = Vec::new();
        for folder in &found {
            if !part.contains('*') {
                let path = folder.join(part);
                if root.join(&path).exists() {
                    next.push(path);
                }
                continue;
            }
            let Ok(entries) = std::fs::read_dir(root.join(folder)) else { continue };
            let mut names: Vec<String> =
                entries.filter_map(|entry| entry.ok()?.file_name().into_string().ok()).collect();
            names.sort();
            next.extend(names.into_iter().filter(|name| matches(part, name)).map(|name| folder.join(name)));
        }
        found = next;
    }
    found.retain(|path| !path.as_os_str().is_empty() && root.join(path).is_file());
    found
}

/// Whether `name` matches `pattern`, `*` standing for any characters.
fn matches(pattern: &str, name: &str) -> bool {
    let pieces: Vec<&str> = pattern.split('*').collect();
    let (first, last) = (pieces[0], pieces[pieces.len() - 1]);
    if pieces.len() == 1 {
        return pattern == name;
    }
    if !name.starts_with(first) || !name[first.len()..].ends_with(last) || name.len() < first.len() + last.len() {
        return false;
    }
    let mut rest = &name[first.len()..name.len() - last.len()];
    for piece in &pieces[1..pieces.len() - 1] {
        match rest.find(piece) {
            Some(at) => rest = &rest[at + piece.len()..],
            None => return false,
        }
    }
    true
}

pub fn sha256(path: &Path) -> Option<String> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 1 << 20];
    loop {
        let count = file.read(&mut buffer).ok()?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Some(hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect())
}

/// A file of the checkout `root`, written since `since`, as a record keeps it.
pub fn output(root: &Path, relative: &Path, since: SystemTime) -> Option<FileRef> {
    let metadata = std::fs::metadata(root.join(relative)).ok()?;
    if metadata.modified().ok()? < since {
        return None;
    }
    let sha256 = (metadata.len() <= LARGEST_HASHED).then(|| sha256(&root.join(relative))).flatten();
    Some(FileRef { path: relative.to_string_lossy().into_owned(), bytes: metadata.len(), sha256 })
}

/// The last `most` lines of a file, ANSI colour codes taken out.
pub fn tail(path: &Path, most: usize) -> Vec<String> {
    let Ok(mut file) = std::fs::File::open(path) else { return Vec::new() };
    let length = file.metadata().map(|metadata| metadata.len()).unwrap_or(0);
    let start = length.saturating_sub(512 * 1024);
    if file.seek(SeekFrom::Start(start)).is_err() {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    let _ = file.read_to_end(&mut bytes);
    let text = String::from_utf8_lossy(&bytes);
    let mut lines: Vec<String> = text.lines().map(without_colours).collect();
    if start > 0 && !lines.is_empty() {
        lines.remove(0);
    }
    let skip = lines.len().saturating_sub(most);
    lines.split_off(skip)
}

fn without_colours(line: &str) -> String {
    let mut clean = String::with_capacity(line.len());
    let mut characters = line.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '\u{1b}' && characters.peek() == Some(&'[') {
            for next in characters.by_ref() {
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
        } else if character != '\r' {
            clean.push(character);
        }
    }
    clean
}

/// `to` relative to `from`, both absolute: "." , "sub/dir", "../../other".
pub fn relative(from: &Path, to: &Path) -> String {
    let from: Vec<Component> = from.components().collect();
    let to: Vec<Component> = to.components().collect();
    let common = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let mut parts: Vec<String> = vec!["..".to_string(); from.len() - common];
    parts.extend(to[common..].iter().map(|part| part.as_os_str().to_string_lossy().into_owned()));
    if parts.is_empty() { ".".to_string() } else { parts.join("/") }
}

/// One line added at the end of a file, in a single write.
pub fn append_line(path: &Path, line: &str) -> Result<(), String> {
    let failed = |error: std::io::Error| format!("{}: {error}", path.display());
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder).map_err(failed)?;
    }
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path).map_err(failed)?;
    file.write_all(format!("{line}\n").as_bytes()).map_err(failed)?;
    file.sync_data().map_err(failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stars_match_within_a_part() {
        assert!(matches("*.json", "forced.json"));
        assert!(matches("seed-*", "seed-1"));
        assert!(matches("results-*.json", "results-1000.json"));
        assert!(!matches("*.json", "forced.json.new"));
        assert!(!matches("seed-*", "double-seed-1"));
        assert!(matches("a*b*c", "aXbYc") && !matches("a*b*c", "acb"));
    }

    #[test]
    fn globs_find_the_files_of_the_repository() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let sources = std::fs::read_dir(root.join("src"))
            .unwrap()
            .filter(|entry| entry.as_ref().unwrap().path().extension().is_some_and(|extension| extension == "rs"));
        assert_eq!(glob(root, "src/*.rs").len(), sources.count());
        assert_eq!(glob(root, "src/*/queue.rs"), [PathBuf::from("src/app/queue.rs")]);
        assert_eq!(glob(root, "tests/*/runs.jsonl"), [PathBuf::from("tests/fixtures/runs.jsonl")]);
        assert!(glob(root, "src").is_empty(), "a folder is not an output");
    }

    #[test]
    fn paths_are_made_relative() {
        assert_eq!(relative(Path::new("/r/a"), Path::new("/r/a")), ".");
        assert_eq!(relative(Path::new("/r/a"), Path::new("/r/a/.claude/w")), ".claude/w");
        assert_eq!(relative(Path::new("/r/a/.claude/w"), Path::new("/r/a")), "../..");
        assert_eq!(relative(Path::new("/r/a"), Path::new("/r/b")), "../b");
    }

    #[test]
    fn colours_are_taken_out() {
        assert_eq!(without_colours("\u{1b}[1;32mok\u{1b}[0m done\r"), "ok done");
    }
}
