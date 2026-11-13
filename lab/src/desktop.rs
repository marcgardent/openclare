//! The desktop kept off the job processors (SPEC.md, §7.5): while lab runs
//! something, every process of the user still allowed on every processor is
//! pinned to the others, the desktop's; once nothing runs, they get every
//! processor back. A process started meanwhile inherits its parent's
//! processors, so it comes back with the others.
//!
//! What lab may not move itself, it asks `sudo -n` for, never a password:
//! kwin and a few desktop processes hold a capability the kernel will not let
//! a plain process change the processors of, and the system's services are
//! root's. The commands are those of the whitelist of SPEC.md, §7.5; without
//! it they fail at once and the window says what was left.

use std::collections::BTreeSet;
use std::process::{Command, Stdio};

use chrono::{DateTime, Local};
use nix::sched::{CpuSet, sched_setaffinity};
use nix::unistd::Pid;

/// What lab last did to the desktop, for the window to show.
#[derive(Clone, Debug, PartialEq)]
pub enum Desktop {
    /// Pinned to these processors: what moved, since when.
    Pinned { cpus: String, moved: Moved, since: DateTime<Local> },
    /// Given every processor back: what moved, when, and why.
    Freed { moved: Moved, at: DateTime<Local>, why: String },
}

/// What a pin or a free did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Moved {
    /// Processes lab moved itself.
    pub processes: usize,
    /// Processes lab may not move (kwin), moved by `sudo -n taskset`.
    pub by_sudo: usize,
    /// The system's services (system.slice) moved too, by `sudo -n systemctl`.
    pub services: bool,
    /// The processes of the user left behind, by name.
    pub left: Vec<String>,
}

impl Moved {
    pub fn any(&self) -> bool {
        self.processes + self.by_sudo > 0 || self.services || !self.left.is_empty()
    }

    /// "283 processes, 4 by sudo, the services too; kwin_wayland left".
    pub fn text(&self) -> String {
        let mut text = format!("{} processes", self.processes + self.by_sudo);
        if self.by_sudo > 0 {
            text.push_str(&format!(", {} by sudo", self.by_sudo));
        }
        if self.services {
            text.push_str(", the services too");
        }
        if !self.left.is_empty() {
            text.push_str(&format!("; {} left", self.left.join(", ")));
        }
        text
    }
}

/// The desktop's processors: those of the machine the jobs do not take.
pub fn cpus(job_cpus: &[usize]) -> Vec<usize> {
    crate::cpus::online().into_iter().filter(|cpu| !job_cpus.contains(cpu)).collect()
}

/// The tests never move the machine's processes: where sudo needs no password
/// (CI), the services held to the desktop's processors would hold the jobs
/// started by the tests there too, and they could not be pinned to theirs.
const TESTING: bool = cfg!(test);

/// Pins to `desktop` every process of this user allowed on every processor,
/// and the system's services.
pub fn pin(desktop: &[usize]) -> Moved {
    let online: Vec<usize> = crate::cpus::online();
    if desktop.is_empty() || desktop.len() == online.len() || TESTING {
        return Moved::default();
    }
    let all: BTreeSet<usize> = online.iter().copied().collect();
    let text = crate::cpus::text(desktop);
    let mut moved = Moved { processes: self::moved(&all, desktop, &|_| true), ..Moved::default() };
    for (pid, name) in of_user_on(&all, &|_| true) {
        if sudo(&["/usr/bin/taskset", "-apc", &text, &pid.to_string()]) {
            moved.by_sudo += 1;
        } else {
            moved.left.push(name);
        }
    }
    moved.services = services() == desktop
        || sudo(&["/usr/bin/systemctl", "set-property", "--runtime", "system.slice", &format!("AllowedCPUs={text}")]);
    moved
}

/// Gives every processor back to the processes of this user pinned to
/// `desktop`, and to the system's services when lab pinned them.
pub fn free(desktop: &[usize]) -> Moved {
    let online = crate::cpus::online();
    if desktop.is_empty() || desktop.len() == online.len() || TESTING {
        return Moved::default();
    }
    let pinned: BTreeSet<usize> = desktop.iter().copied().collect();
    let text = crate::cpus::text(&online);
    let mut moved = Moved { processes: self::moved(&pinned, &online, &|_| true), ..Moved::default() };
    for (pid, name) in of_user_on(&pinned, &|_| true) {
        if sudo(&["/usr/bin/taskset", "-apc", &text, &pid.to_string()]) {
            moved.by_sudo += 1;
        } else {
            moved.left.push(name);
        }
    }
    if services() == desktop {
        moved.services = sudo(&["/usr/bin/systemctl", "set-property", "--runtime", "system.slice", "AllowedCPUs="]);
        if !moved.services {
            moved.left.push("the services".to_string());
        }
    }
    moved
}

/// A command run by `sudo -n`: true when it ran and succeeded. Never asks for
/// a password; a command off the whitelist fails at once.
fn sudo(arguments: &[&str]) -> bool {
    Command::new("sudo")
        .arg("-n")
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// The processors the system's services are held to (none when not held);
/// systemd says them "0 4".
fn services() -> Vec<usize> {
    Command::new("systemctl")
        .args(["show", "system.slice", "--property", "AllowedCPUs", "--value"])
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| crate::cpus::parse(&String::from_utf8_lossy(&output.stdout).replace(' ', ",")).ok())
        .unwrap_or_default()
}

/// The processes of this user allowed on exactly `from` (and kept by
/// `only`): pid and name.
fn of_user_on(from: &BTreeSet<usize>, only: &dyn Fn(i32) -> bool) -> Vec<(i32, String)> {
    let Some(uid) = std::fs::read_to_string("/proc/self/status").ok().and_then(|status| real_uid(&status)) else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir("/proc") else { return Vec::new() };
    let mut found = Vec::new();
    for entry in entries.flatten() {
        let Some(pid) = entry.file_name().to_str().and_then(|name| name.parse::<i32>().ok()) else { continue };
        if !only(pid) {
            continue;
        }
        let Ok(status) = std::fs::read_to_string(entry.path().join("status")) else { continue };
        if real_uid(&status) != Some(uid) {
            continue;
        }
        let field = |name: &str| status.lines().find_map(|line| line.strip_prefix(name)).map(str::trim);
        let allowed: BTreeSet<usize> = field("Cpus_allowed_list:")
            .and_then(|text| crate::cpus::parse(text).ok())
            .unwrap_or_default()
            .into_iter()
            .collect();
        if &allowed == from {
            found.push((pid, field("Name:").unwrap_or("?").to_string()));
        }
    }
    found
}

/// The processes of this user allowed on exactly `from` (and kept by
/// `only`), every thread of them set to `to`; how many processes moved.
fn moved(from: &BTreeSet<usize>, to: &[usize], only: &dyn Fn(i32) -> bool) -> usize {
    let mut set = CpuSet::new();
    for &cpu in to {
        if set.set(cpu).is_err() {
            return 0;
        }
    }
    let mut count = 0;
    for (pid, _) in of_user_on(from, only) {
        // Every thread of it: the affinity is a thread's.
        let mut any = false;
        if let Ok(tasks) = std::fs::read_dir(format!("/proc/{pid}/task")) {
            for task in tasks.flatten() {
                if let Some(tid) = task.file_name().to_str().and_then(|name| name.parse::<i32>().ok()) {
                    any |= sched_setaffinity(Pid::from_raw(tid), &set).is_ok();
                }
            }
        } else {
            any = sched_setaffinity(Pid::from_raw(pid), &set).is_ok();
        }
        count += usize::from(any);
    }
    count
}

/// The real user id of a /proc/<pid>/status.
fn real_uid(status: &str) -> Option<u32> {
    status.lines().find_map(|line| line.strip_prefix("Uid:"))?.split_whitespace().next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The processors a process is allowed on, from /proc.
    fn allowed(pid: u32) -> Vec<usize> {
        let status = std::fs::read_to_string(format!("/proc/{pid}/status")).unwrap();
        let line = status.lines().find_map(|line| line.strip_prefix("Cpus_allowed_list:")).unwrap();
        crate::cpus::parse(line.trim()).unwrap()
    }

    #[test]
    fn a_process_is_pinned_to_the_desktop_then_given_every_processor_back() {
        let online = crate::cpus::online();
        if online.len() < 2 {
            return;
        }
        // One process of ours, the others left alone: pinned as `pin` does, freed as `free` does.
        let desktop = vec![online[0]];
        let mut child = std::process::Command::new("sleep").arg("5").spawn().unwrap();
        let pid = child.id() as i32;
        let only = move |candidate: i32| candidate == pid;
        let all: BTreeSet<usize> = online.iter().copied().collect();
        // Allowed everywhere, as a process of the desktop, wherever the tests were started from (a
        // shell of a desktop that lab pinned is on its processors).
        assert_eq!(moved(&allowed(child.id()).into_iter().collect(), &online, &only), 1);
        assert_eq!(of_user_on(&all, &only), vec![(pid, "sleep".to_string())]);
        assert_eq!(moved(&all, &desktop, &only), 1);
        assert_eq!(allowed(child.id()), desktop, "pinned to the desktop");
        assert_eq!(moved(&all, &desktop, &only), 0, "pinned already: not allowed everywhere any more");
        assert_eq!(moved(&desktop.iter().copied().collect(), &online, &only), 1);
        assert_eq!(allowed(child.id()), online, "every processor back");
        let _ = child.kill();
        let _ = child.wait();
        assert_eq!(cpus(&online[1..]), desktop);
    }

    #[test]
    fn what_moved_is_told() {
        let moved = Moved { processes: 283, by_sudo: 4, services: true, left: vec![] };
        assert_eq!(moved.text(), "287 processes, 4 by sudo, the services too");
        let left = Moved { processes: 283, left: vec!["kwin_wayland".to_string()], ..Moved::default() };
        assert_eq!(left.text(), "283 processes; kwin_wayland left");
        assert!(!Moved::default().any());
    }
}
