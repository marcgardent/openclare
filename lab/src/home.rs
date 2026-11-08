//! Where things are: the home checkout and the folder of the machine,
//! `~/.openclare-lab` (SPEC.md, sections 2 and 3), and what git says of a
//! checkout.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The folders of one session of the app.
#[derive(Clone, Debug)]
pub struct Places {
    /// The clone `lab/` sits in.
    pub home: PathBuf,
    /// `~/.openclare-lab`: `runs.jsonl`, `logs/`, `state.json`, `settings.toml`.
    pub data: PathBuf,
}

impl Places {
    /// The home checkout is the first folder holding `lab/jobs.toml` from
    /// `start` up, else the one this binary was built from. The folder of
    /// the machine is `data`, else `~/.openclare-lab`.
    pub fn find(start: &Path, data: Option<PathBuf>) -> Result<Places, String> {
        let built_from = Path::new(env!("CARGO_MANIFEST_DIR")).parent().map(Path::to_path_buf);
        let home = start
            .ancestors()
            .find(|folder| folder.join("lab").join("jobs.toml").is_file())
            .map(Path::to_path_buf)
            .or(built_from.filter(|folder| folder.join("lab").join("jobs.toml").is_file()))
            .ok_or("no lab/jobs.toml here or above: run lab from the repository")?;
        let data = data.or_else(default_folder).ok_or("no home folder ($HOME)")?;
        Ok(Places { home, data })
    }

    pub fn jobs(&self) -> PathBuf {
        self.home.join("lab").join("jobs.toml")
    }

    pub fn runs(&self) -> PathBuf {
        self.data.join("runs.jsonl")
    }

    pub fn settings(&self) -> PathBuf {
        self.data.join("settings.toml")
    }
}

/// `~/.openclare-lab`, the one folder of everything lab keeps on the machine.
pub fn default_folder() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".openclare-lab"))
}

/// The folders of the versions before 2026-11-08: `$XDG_DATA_HOME/openclare-lab`
/// and `$XDG_CONFIG_HOME/openclare-lab`.
pub fn earlier_folders() -> Vec<PathBuf> {
    [dirs::data_dir(), dirs::config_dir()].into_iter().flatten().map(|folder| folder.join("openclare-lab")).collect()
}

/// Moves the files of the earlier folders into `lab` once (SPEC.md §3): a file
/// `lab` already has is left where it is, and an earlier folder left empty is
/// removed. Says what was moved and what was left.
pub fn migrate(lab: &Path, earlier: &[PathBuf]) -> Result<Vec<String>, String> {
    let mut said = Vec::new();
    for folder in earlier.iter().filter(|folder| folder.is_dir()) {
        let failed = |error: std::io::Error| format!("{}: {error}", folder.display());
        for entry in std::fs::read_dir(folder).map_err(failed)? {
            let entry = entry.map_err(failed)?.path();
            let target = lab.join(entry.file_name().unwrap_or_default());
            if target.exists() {
                said.push(format!("{} left: {} exists already", entry.display(), target.display()));
                continue;
            }
            std::fs::create_dir_all(lab).map_err(|error| format!("{}: {error}", lab.display()))?;
            move_path(&entry, &target)?;
            said.push(format!("{} moved to {}", entry.display(), target.display()));
        }
        if std::fs::read_dir(folder).map(|mut entries| entries.next().is_none()).unwrap_or(false) {
            let _ = std::fs::remove_dir(folder);
        }
    }
    Ok(said)
}

/// Renames, or copies then removes when the two are on different file systems.
fn move_path(from: &Path, to: &Path) -> Result<(), String> {
    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }
    copy_tree(from, to)?;
    if from.is_dir() { std::fs::remove_dir_all(from) } else { std::fs::remove_file(from) }
        .map_err(|error| format!("{}: {error}", from.display()))
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    let failed = |error: std::io::Error| format!("{}: {error}", from.display());
    if from.is_dir() {
        std::fs::create_dir_all(to).map_err(failed)?;
        for entry in std::fs::read_dir(from).map_err(failed)? {
            let entry = entry.map_err(failed)?;
            copy_tree(&entry.path(), &to.join(entry.file_name()))?;
        }
    } else {
        std::fs::copy(from, to).map_err(failed)?;
    }
    Ok(())
}

/// What the top bar says of a checkout.
#[derive(Clone, Debug, PartialEq)]
pub struct GitState {
    pub branch: Option<String>,
    pub commit: String,
    pub short: String,
    /// Tracked files differ from HEAD, `lab/runs.jsonl` excluded.
    pub dirty: bool,
    /// A linked worktree, not the main checkout.
    pub worktree: bool,
}

impl GitState {
    pub fn read(checkout: &Path) -> Result<GitState, String> {
        let commit = git(checkout, &["rev-parse", "HEAD"])?;
        let branch = Some(git(checkout, &["branch", "--show-current"])?).filter(|branch| !branch.is_empty());
        let status =
            git(checkout, &["status", "--porcelain", "--untracked-files=no", "--", ".", ":(exclude)lab/runs.jsonl"])?;
        let own = git(checkout, &["rev-parse", "--git-dir"])?;
        let common = git(checkout, &["rev-parse", "--git-common-dir"])?;
        Ok(GitState {
            branch,
            short: commit.chars().take(7).collect(),
            commit,
            dirty: !status.is_empty(),
            worktree: own != common,
        })
    }
}

/// A checkout a job can run in: the home checkout or one of its worktrees.
#[derive(Clone, Debug, PartialEq)]
pub struct Worktree {
    pub path: PathBuf,
    /// Relative to the home checkout, as a record names it.
    pub relative: String,
    pub branch: Option<String>,
    /// The commit checked out, when git says.
    pub head: String,
}

/// The checkouts `git worktree list` knows, the home checkout first.
pub fn worktrees(home: &Path) -> Result<Vec<Worktree>, String> {
    let text = git(home, &["worktree", "list", "--porcelain"])?;
    let mut found = Vec::new();
    for block in text.split("\n\n") {
        let mut path = None;
        let mut branch = None;
        let mut head = String::new();
        for line in block.lines() {
            if let Some(rest) = line.strip_prefix("worktree ") {
                path = Some(PathBuf::from(rest));
            } else if let Some(rest) = line.strip_prefix("HEAD ") {
                head = rest.to_string();
            } else if let Some(rest) = line.strip_prefix("branch ") {
                branch = Some(rest.trim_start_matches("refs/heads/").to_string());
            }
        }
        if let Some(path) = path.filter(|path| path.is_dir()) {
            found.push(Worktree { relative: crate::files::relative(home, &path), path, branch, head });
        }
    }
    found.sort_by_key(|worktree| worktree.relative != ".");
    Ok(found)
}

fn git(checkout: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(checkout)
        .args(arguments)
        .output()
        .map_err(|error| format!("git: {error}"))?;
    if !output.status.success() {
        return Err(format!("git {}: {}", arguments.join(" "), String::from_utf8_lossy(&output.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_home_checkout_is_found_from_inside_it() {
        let lab = Path::new(env!("CARGO_MANIFEST_DIR"));
        let places = Places::find(&lab.join("src"), Some(PathBuf::from("data"))).unwrap();
        assert_eq!(places.home, lab.parent().unwrap());
        assert_eq!(places.runs(), PathBuf::from("data").join("runs.jsonl"));
        assert_eq!(places.settings(), PathBuf::from("data").join("settings.toml"));
        let git = GitState::read(&places.home).unwrap();
        assert_eq!(git.commit.len(), 40);
        assert!(git.commit.starts_with(&git.short));
        let worktrees = worktrees(&places.home).unwrap();
        assert_eq!(worktrees[0].relative, ".");
        assert_eq!(worktrees[0].branch, git.branch);
        assert_eq!(worktrees[0].head, git.commit);
    }

    #[test]
    fn the_default_folder_is_in_the_home_folder() {
        let folder = default_folder().unwrap();
        assert_eq!(folder.file_name().unwrap(), ".openclare-lab");
        assert_eq!(folder.parent(), dirs::home_dir().as_deref());
        assert!(earlier_folders().iter().all(|earlier| earlier.ends_with("openclare-lab")));
    }

    /// The earlier folders moved into the one folder, once; what the folder
    /// already has is left; the earlier folders are removed when emptied.
    #[test]
    fn the_earlier_folders_are_moved_once() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join("test-data").join("migrate");
        let _ = std::fs::remove_dir_all(&root);
        let (lab, data, config) = (root.join("lab"), root.join("data"), root.join("config"));
        std::fs::create_dir_all(data.join("logs")).unwrap();
        std::fs::create_dir_all(&config).unwrap();
        std::fs::write(data.join("runs.jsonl"), "{}\n").unwrap();
        std::fs::write(data.join("logs").join("a.log"), "a").unwrap();
        std::fs::write(config.join("settings.toml"), "threads = 3\n").unwrap();
        let said = migrate(&lab, &[data.clone(), config.clone()]).unwrap();
        assert_eq!(said.len(), 3, "{said:?}");
        assert_eq!(std::fs::read_to_string(lab.join("runs.jsonl")).unwrap(), "{}\n");
        assert_eq!(std::fs::read_to_string(lab.join("logs").join("a.log")).unwrap(), "a");
        assert_eq!(std::fs::read_to_string(lab.join("settings.toml")).unwrap(), "threads = 3\n");
        assert!(!data.exists() && !config.exists(), "the earlier folders are gone");
        assert!(migrate(&lab, &[data.clone(), config.clone()]).unwrap().is_empty(), "nothing twice");
        // A file the folder already has is left, and the earlier folder with it.
        std::fs::create_dir_all(&data).unwrap();
        std::fs::write(data.join("runs.jsonl"), "old\n").unwrap();
        let said = migrate(&lab, &[data.clone()]).unwrap();
        assert_eq!(said.len(), 1);
        assert!(said[0].contains("left"), "{said:?}");
        assert_eq!(std::fs::read_to_string(lab.join("runs.jsonl")).unwrap(), "{}\n");
        assert_eq!(std::fs::read_to_string(data.join("runs.jsonl")).unwrap(), "old\n");
    }
}
