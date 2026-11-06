//! Where things are: the home checkout, the data and config folders
//! (SPEC.md, sections 2 and 3), and what git says of a checkout.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The folders of one session of the app.
#[derive(Clone, Debug)]
pub struct Places {
    /// The clone `lab/` sits in.
    pub home: PathBuf,
    /// `runs.jsonl`, `logs/`, `state.json`.
    pub data: PathBuf,
    /// `settings.toml`.
    pub config: PathBuf,
}

impl Places {
    /// The home checkout is the first folder holding `lab/jobs.toml` from
    /// `start` up, else the one this binary was built from.
    pub fn find(start: &Path, data: Option<PathBuf>) -> Result<Places, String> {
        let built_from = Path::new(env!("CARGO_MANIFEST_DIR")).parent().map(Path::to_path_buf);
        let home = start
            .ancestors()
            .find(|folder| folder.join("lab").join("jobs.toml").is_file())
            .map(Path::to_path_buf)
            .or(built_from.filter(|folder| folder.join("lab").join("jobs.toml").is_file()))
            .ok_or("no lab/jobs.toml here or above: run lab from the repository")?;
        let data = data
            .or_else(|| dirs::data_dir().map(|folder| folder.join("openclare-lab")))
            .ok_or("no data folder ($XDG_DATA_HOME)")?;
        let config = dirs::config_dir()
            .map(|folder| folder.join("openclare-lab"))
            .ok_or("no config folder ($XDG_CONFIG_HOME)")?;
        Ok(Places { home, data, config })
    }

    pub fn jobs(&self) -> PathBuf {
        self.home.join("lab").join("jobs.toml")
    }

    pub fn runs(&self) -> PathBuf {
        self.data.join("runs.jsonl")
    }

    pub fn settings(&self) -> PathBuf {
        self.config.join("settings.toml")
    }
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
        let git = GitState::read(&places.home).unwrap();
        assert_eq!(git.commit.len(), 40);
        assert!(git.commit.starts_with(&git.short));
        let worktrees = worktrees(&places.home).unwrap();
        assert_eq!(worktrees[0].relative, ".");
        assert_eq!(worktrees[0].branch, git.branch);
        assert_eq!(worktrees[0].head, git.commit);
    }
}
