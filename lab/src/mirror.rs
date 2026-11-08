//! The mirror of the repository, `<lab>/repo.git` (SPEC.md, sections 2 and
//! 8.1): fetched from GitHub, it resolves the reference a job runs on, holds
//! one detached checkout per commit under `<lab>/checkouts/`, and answers what
//! the repository knows without any clone: its tags, its branches, a file at
//! `main`, whether a commit is pushed.
//!
//! Every git command runs on the runner's thread or on the window's; the two
//! never fetch at once, since only the runner fetches (runner.rs).

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

/// The branch the catalogue and the summaries are read from (SPEC.md, §2, §13).
pub const MAIN: &str = "main";

/// The repository, as the `remote` setting names it by default.
pub const DEFAULT_REMOTE: &str = "marcgardent/openclare";

/// A fetch that takes longer is given up.
const FETCH_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Clone, Debug, PartialEq)]
pub struct Mirror {
    /// `<lab>/repo.git`, a bare mirror.
    pub path: PathBuf,
    /// `<lab>/checkouts`, one folder per commit checked out.
    pub checkouts: PathBuf,
    /// What git fetches from.
    pub url: String,
}

/// Where a job runs: a reference resolved in the mirror, or a folder given by
/// hand, the escape hatch for an experiment not committed (SPEC.md, §2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// A tag, a branch, `pull/<n>` or a hash.
    Reference(String),
    Local(PathBuf),
}

/// A checkout the mirror knows, from `git worktree list`.
#[derive(Clone, Debug, PartialEq)]
pub struct Checkout {
    pub path: PathBuf,
    /// The commit checked out, 40 hexadecimal digits.
    pub head: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    /// The commit it points at, 40 hexadecimal digits.
    pub commit: String,
    /// "2026-10-28".
    pub date: String,
}

/// `owner/name` is a repository of GitHub, fetched over SSH with Marc's key
/// (Q18); anything else, a URL or a path, is taken as it is.
pub fn url_of(remote: &str) -> String {
    let remote = remote.trim();
    let github = remote.matches('/').count() == 1
        && !remote.contains(':')
        && !remote.starts_with(['/', '.', '~'])
        && !remote.ends_with(".git");
    if github { format!("git@github.com:{remote}.git") } else { remote.to_string() }
}

impl Target {
    /// A folder when the text is a path (`/…`, `~/…`, `./…`), else a reference.
    pub fn parse(text: &str) -> Target {
        let text = text.trim();
        if let Some(rest) = text.strip_prefix("~/").or_else(|| (text == "~").then_some(""))
            && let Some(home) = dirs::home_dir()
        {
            return Target::Local(if rest.is_empty() { home } else { home.join(rest) });
        }
        if text.starts_with('/') || text.starts_with("./") || text.starts_with("../") {
            return Target::Local(PathBuf::from(text));
        }
        Target::Reference(text.to_string())
    }

    /// As a record names it: the reference, or "local" (SPEC.md, §5).
    pub fn recorded(&self) -> String {
        match self {
            Target::Reference(reference) => reference.clone(),
            Target::Local(_) => "local".to_string(),
        }
    }
}

impl Mirror {
    pub fn new(lab: &Path, remote: &str) -> Mirror {
        Mirror { path: lab.join("repo.git"), checkouts: lab.join("checkouts"), url: url_of(remote) }
    }

    pub fn exists(&self) -> bool {
        self.path.join("HEAD").is_file()
    }

    /// When it was last fetched: the time of FETCH_HEAD.
    pub fn last_fetch(&self) -> Option<SystemTime> {
        std::fs::metadata(self.path.join("FETCH_HEAD")).and_then(|metadata| metadata.modified()).ok()
    }

    /// Creates the mirror if it is not there, from `seed` (the git folder of a
    /// clone on this machine, fast and offline) when given, else from the
    /// remote; then fetches it, pruning what the remote no longer has. Says
    /// what was done.
    pub fn fetch(&self, seed: Option<&Path>) -> Result<String, String> {
        let mut said = Vec::new();
        if !self.exists() {
            let parent = self.path.parent().ok_or("the mirror has no parent folder")?;
            std::fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
            let target = self.path.to_string_lossy().into_owned();
            match seed.filter(|seed| seed.join("HEAD").is_file()) {
                Some(seed) => {
                    let seed_text = seed.to_string_lossy().into_owned();
                    git_at(parent, &["clone", "--quiet", "--mirror", &seed_text, &target], FETCH_TIMEOUT)?;
                    self.git(&["remote", "set-url", "origin", &self.url])?;
                    // The LFS objects the clone has (the bench's scenes), linked in: the
                    // checkouts of the mirror share them, and download none of them again.
                    let linked = link_tree(&seed.join("lfs").join("objects"), &self.path.join("lfs").join("objects"))
                        .unwrap_or(0);
                    said.push(format!("mirror created from the clone at {seed_text}, {linked} LFS objects linked"));
                }
                None => {
                    git_at(parent, &["clone", "--quiet", "--mirror", &self.url, &target], FETCH_TIMEOUT)?;
                    said.push(format!("mirror created from {}", self.url));
                }
            }
        }
        let before = self.refs();
        git_at(&self.path, &["fetch", "--quiet", "--prune", "origin"], FETCH_TIMEOUT)?;
        let after = self.refs();
        let new = after.iter().filter(|(name, hash)| before.get(*name) != Some(hash)).count();
        let gone = before.keys().filter(|name| !after.contains_key(*name)).count();
        said.push(match (new, gone) {
            (0, 0) => format!("fetched from {}: nothing new", self.url),
            (new, 0) => format!("fetched from {}: {new} references moved or new", self.url),
            (new, gone) => format!("fetched from {}: {new} references moved or new, {gone} gone", self.url),
        });
        Ok(said.join("; "))
    }

    /// Every reference of the mirror and the commit it points at.
    fn refs(&self) -> std::collections::BTreeMap<String, String> {
        self.git(&["for-each-ref", "--format=%(refname) %(objectname)"])
            .unwrap_or_default()
            .lines()
            .filter_map(|line| line.split_once(' ').map(|(name, hash)| (name.to_string(), hash.to_string())))
            .collect()
    }

    /// The commit a reference names: a tag, a branch, `pull/<n>` or a hash,
    /// 40 hexadecimal digits.
    pub fn resolve(&self, reference: &str) -> Result<String, String> {
        let reference = reference.trim();
        if reference.is_empty() {
            return Err("no reference: a tag, a branch, pull/<n> or a hash".to_string());
        }
        if !self.exists() {
            return Err("no mirror yet: Fetch creates it".to_string());
        }
        let candidates = match reference.strip_prefix("pull/") {
            Some(number) => vec![format!("refs/pull/{number}/head")],
            None => vec![
                format!("refs/tags/{reference}"),
                format!("refs/heads/{reference}"),
                format!("refs/remotes/origin/{reference}"),
                reference.to_string(),
            ],
        };
        for candidate in candidates {
            if let Ok(hash) = self.git(&["rev-parse", "--verify", "--quiet", &format!("{candidate}^{{commit}}")])
                && hash.len() == 40
            {
                return Ok(hash);
            }
        }
        Err(format!("{reference}: not a tag, a branch, a pull request nor a commit the mirror knows"))
    }

    /// Whether the commit is reachable from a branch, a tag or a pull request
    /// of the remote (SPEC.md, §10b): false when the mirror does not know it.
    pub fn pushed(&self, hash: &str) -> bool {
        if hash.is_empty() || !self.exists() {
            return false;
        }
        let refs = ["refs/heads", "refs/tags", "refs/remotes", "refs/pull"];
        self.git(&["for-each-ref", "--count=1", "--contains", hash, refs[0], refs[1], refs[2], refs[3]])
            .is_ok_and(|found| !found.is_empty())
    }

    /// The tags `v*`, newest first.
    pub fn tags(&self) -> Vec<Tag> {
        let Ok(text) = self.git(&["tag", "-l", "v*", "--sort=-creatordate"]) else { return Vec::new() };
        text.lines()
            .filter_map(|name| {
                let commit = self.git(&["rev-parse", &format!("{name}^{{commit}}")]).ok()?;
                let date = self.git(&["log", "-1", "--format=%ad", "--date=short", &commit]).unwrap_or_default();
                Some(Tag { name: name.to_string(), commit, date })
            })
            .collect()
    }

    /// The branches of the remote, `main` first, then by name.
    pub fn branches(&self) -> Vec<String> {
        let Ok(text) = self.git(&["for-each-ref", "--format=%(refname:short)", "refs/heads"]) else {
            return Vec::new();
        };
        let mut branches: Vec<String> = text.lines().map(str::to_string).filter(|name| !name.is_empty()).collect();
        branches.sort_by_key(|name| (name != MAIN, name.clone()));
        branches
    }

    /// A file as it is at a reference, `git show <reference>:<path>`; None if
    /// the reference or the file is not there.
    pub fn show(&self, reference: &str, path: &str) -> Option<String> {
        let hash = self.resolve(reference).ok()?;
        self.git(&["show", &format!("{hash}:{path}")]).ok()
    }

    /// The commit that last touched a file at a reference, and when: "<hash>", "<RFC 3339>".
    pub fn brought(&self, reference: &str, path: &str) -> Option<(String, String)> {
        let hash = self.resolve(reference).ok()?;
        let line =
            self.git(&["log", "-1", "--format=%H %aI", &hash, "--", path]).ok().filter(|line| !line.is_empty())?;
        line.split_once(' ').map(|(hash, date)| (hash.to_string(), date.to_string()))
    }

    /// The checkouts the mirror has, with the commit each holds.
    pub fn checkouts(&self) -> Vec<Checkout> {
        let Ok(text) = self.git(&["worktree", "list", "--porcelain"]) else { return Vec::new() };
        let mut found = Vec::new();
        for block in text.split("\n\n") {
            let mut path = None;
            let mut head = String::new();
            let mut bare = false;
            for line in block.lines() {
                if let Some(rest) = line.strip_prefix("worktree ") {
                    path = Some(PathBuf::from(rest));
                } else if let Some(rest) = line.strip_prefix("HEAD ") {
                    head = rest.to_string();
                } else if line == "bare" {
                    bare = true;
                }
            }
            if let Some(path) = path.filter(|path| !bare && path.is_dir()) {
                found.push(Checkout { path, head });
            }
        }
        found
    }

    /// The checkout holding `hash`: the one there already, else a new detached
    /// worktree `<lab>/checkouts/<short hash>` (SPEC.md, §8.1).
    pub fn checkout(&self, hash: &str) -> Result<PathBuf, String> {
        if hash.len() != 40 {
            return Err(format!("{hash}: not a full hash"));
        }
        let _ = self.git(&["worktree", "prune"]);
        if let Some(found) = self.checkouts().into_iter().find(|checkout| checkout.head == hash) {
            return Ok(found.path);
        }
        std::fs::create_dir_all(&self.checkouts).map_err(|error| format!("{}: {error}", self.checkouts.display()))?;
        let mut length = 7;
        while self.checkouts.join(&hash[..length]).exists() && length < 40 {
            length += 1;
        }
        let path = self.checkouts.join(&hash[..length]);
        let target = path.to_string_lossy().into_owned();
        git_at(&self.path, &["worktree", "add", "--quiet", "--detach", &target, hash], FETCH_TIMEOUT)?;
        Ok(path)
    }

    /// Removes a checkout and what was built in it.
    pub fn remove_checkout(&self, path: &Path) -> Result<(), String> {
        let target = path.to_string_lossy().into_owned();
        self.git(&["worktree", "remove", "--force", &target])?;
        if path.exists() {
            std::fs::remove_dir_all(path).map_err(|error| format!("{}: {error}", path.display()))?;
        }
        Ok(())
    }

    fn git(&self, arguments: &[&str]) -> Result<String, String> {
        git_at(&self.path, arguments, Duration::from_secs(60))
    }
}

/// A git command in `folder`, given up after `timeout`. Its output is read
/// as it comes, on threads of its own: a `git show` of a large file would
/// otherwise fill the pipe and wait for ever.
fn git_at(folder: &Path, arguments: &[&str], timeout: Duration) -> Result<String, String> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(folder)
        .args(arguments)
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("git: {error}"))?;
    fn read<R: Read + Send + 'static>(pipe: Option<R>) -> std::thread::JoinHandle<Vec<u8>> {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut bytes);
            }
            bytes
        })
    }
    let stdout = read(child.stdout.take());
    let stderr = read(child.stderr.take());
    let begun = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if begun.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("git {}: gave up after {} s", arguments.join(" "), timeout.as_secs()));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(error) => return Err(format!("git {}: {error}", arguments.join(" "))),
        }
    };
    let stdout = stdout.join().unwrap_or_default();
    let stderr = stderr.join().unwrap_or_default();
    if !status.success() {
        let shown: Vec<&str> = arguments.iter().copied().filter(|argument| !argument.starts_with("--quiet")).collect();
        return Err(format!("git {}: {}", shown.join(" "), String::from_utf8_lossy(&stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&stdout).trim().to_string())
}

/// The files of a folder linked (hard links, else copied) under another:
/// the LFS objects of the clone, so that no checkout downloads them again.
fn link_tree(from: &Path, to: &Path) -> std::io::Result<u64> {
    let mut count = 0;
    if from.is_dir() {
        std::fs::create_dir_all(to)?;
        for entry in std::fs::read_dir(from)? {
            let entry = entry?;
            count += link_tree(&entry.path(), &to.join(entry.file_name()))?;
        }
    } else if !to.exists() {
        if std::fs::hard_link(from, to).is_err() {
            std::fs::copy(from, to)?;
        }
        count += 1;
    }
    Ok(count)
}

/// The bytes under a folder, every file counted once.
pub fn size(path: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(path) else {
        return std::fs::symlink_metadata(path).map(|metadata| metadata.len()).unwrap_or(0);
    };
    entries
        .flatten()
        .map(|entry| {
            let kind = entry.file_type();
            if kind.as_ref().is_ok_and(|kind| kind.is_dir()) {
                size(&entry.path())
            } else {
                entry.metadata().map(|metadata| metadata.len()).unwrap_or(0)
            }
        })
        .sum()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// The git folder of this repository, the seed of the mirrors of the tests.
    pub(crate) fn seed() -> PathBuf {
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let common =
            git_at(repository, &["rev-parse", "--path-format=absolute", "--git-common-dir"], Duration::from_secs(10))
                .expect("this repository has a git folder");
        PathBuf::from(common)
    }

    /// A mirror of this repository under target/, created once per test
    /// folder, with the repository itself as the remote.
    pub(crate) fn mirror(name: &str) -> Mirror {
        let lab = Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join("test-data").join(name);
        let _ = std::fs::remove_dir_all(&lab);
        std::fs::create_dir_all(&lab).unwrap();
        let seed = seed();
        let mirror = Mirror::new(&lab, &seed.to_string_lossy());
        let said = mirror.fetch(Some(&seed)).unwrap();
        assert!(said.contains("mirror created from the clone") && said.contains("LFS objects linked"), "{said}");
        mirror
    }

    /// The same, with `main` moved to the commit this checkout is at: the tests
    /// on the repository's summaries read them as this commit has them, not as
    /// the clone's `main`, which may lag behind.
    pub(crate) fn mirror_at_head(name: &str) -> Mirror {
        let mirror = mirror(name);
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let head = git_at(repository, &["rev-parse", "HEAD"], Duration::from_secs(10)).expect("a HEAD");
        mirror.git(&["update-ref", "refs/heads/main", &head]).expect("main moved");
        mirror
    }

    #[test]
    fn a_remote_is_a_github_repository_or_a_url() {
        assert_eq!(url_of(DEFAULT_REMOTE), "git@github.com:marcgardent/openclare.git");
        assert_eq!(url_of("https://github.com/x/y.git"), "https://github.com/x/y.git");
        assert_eq!(url_of("git@github.com:x/y.git"), "git@github.com:x/y.git");
        assert_eq!(url_of("/somewhere/repo.git"), "/somewhere/repo.git");
    }

    #[test]
    fn a_target_is_a_reference_or_a_folder() {
        assert_eq!(Target::parse(" v0.7.0 "), Target::Reference("v0.7.0".to_string()));
        assert_eq!(Target::parse("pull/301").recorded(), "pull/301");
        assert_eq!(Target::parse("/x/y"), Target::Local(PathBuf::from("/x/y")));
        assert_eq!(Target::parse("./here").recorded(), "local");
        assert_eq!(Target::parse("~/clone"), Target::Local(dirs::home_dir().unwrap().join("clone")));
    }

    /// The repository as it is: v0.7.0 is e13fadb (checked by hand on 2026-11-06).
    #[test]
    fn the_mirror_resolves_references_and_checks_a_commit_out() {
        let mirror = mirror("mirror");
        assert!(mirror.exists() && mirror.last_fetch().is_some());
        let v070 = mirror.resolve("v0.7.0").unwrap();
        assert!(v070.starts_with("e13fadb"), "{v070}");
        assert_eq!(mirror.resolve(&v070[..9]).unwrap(), v070, "a short hash resolves");
        assert!(mirror.resolve(MAIN).is_ok(), "main, as a branch of the mirror or of its origin");
        assert!(mirror.resolve("no-such-reference").is_err());
        assert!(mirror.resolve("").is_err());
        assert!(mirror.pushed(&v070));
        assert!(!mirror.pushed("0123456789abcdef0123456789abcdef01234567"));
        let tags = mirror.tags();
        assert!(tags.iter().any(|tag| tag.name == "v0.7.0" && tag.commit == v070 && tag.date.starts_with("2026")));
        assert_eq!(mirror.branches().first().map(String::as_str), Some(MAIN));
        assert!(mirror.show(MAIN, "README.md").unwrap().contains("openclare"));
        assert!(mirror.show(MAIN, "no/such/file").is_none());
        // A file larger than a pipe's buffer, read whole (forced.json, 769 KB on 2026-11-08).
        let large = mirror.show(MAIN, "docs/research/turns-of-a-t-mesh/experiments/results/forced.json").unwrap();
        assert!(large.len() > 65_536 && large.trim_end().ends_with('}'), "{} bytes", large.len());
        let (brought, date) = mirror.brought(MAIN, "README.md").unwrap();
        assert_eq!(brought.len(), 40);
        assert!(date.starts_with("20"), "{date}");
        // One checkout per commit, reused.
        assert!(mirror.checkouts().is_empty());
        let path = mirror.checkout(&v070).unwrap();
        assert_eq!(path, mirror.checkouts.join("e13fadb"));
        assert!(path.join("Cargo.toml").is_file());
        assert_eq!(mirror.checkout(&v070).unwrap(), path);
        let listed = mirror.checkouts();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].head, v070);
        let state = crate::home::GitState::read(&path).unwrap();
        assert_eq!(state.commit, v070);
        assert!(!state.dirty && state.branch.is_none() && state.worktree);
        assert!(size(&path) > 1_000_000);
        // Fetching again changes nothing.
        assert!(mirror.fetch(None).unwrap().contains("nothing new"));
        mirror.remove_checkout(&path).unwrap();
        assert!(!path.exists() && mirror.checkouts().is_empty());
    }
}
