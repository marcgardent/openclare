//! The desktop kept off the job processors (SPEC.md, §7.5): while lab runs
//! something, every process of the user still allowed on every processor is
//! pinned to the others, the desktop's; once nothing runs, they get every
//! processor back. A process started meanwhile inherits its parent's
//! processors, so it comes back with the others.

use std::collections::BTreeSet;

use chrono::{DateTime, Local};
use nix::sched::{CpuSet, sched_setaffinity};
use nix::unistd::Pid;

/// What lab last did to the desktop, for the window to show.
#[derive(Clone, Debug, PartialEq)]
pub enum Desktop {
    /// Pinned to these processors: how many processes, since when.
    Pinned { cpus: String, processes: usize, since: DateTime<Local> },
    /// Given every processor back: how many processes, when, and why.
    Freed { processes: usize, at: DateTime<Local>, why: String },
}

/// The desktop's processors: those of the machine the jobs do not take.
pub fn cpus(job_cpus: &[usize]) -> Vec<usize> {
    crate::cpus::online().into_iter().filter(|cpu| !job_cpus.contains(cpu)).collect()
}

/// Pins to `desktop` every process of this user allowed on every processor;
/// how many it moved.
pub fn pin(desktop: &[usize]) -> usize {
    let online: BTreeSet<usize> = crate::cpus::online().into_iter().collect();
    if desktop.is_empty() || desktop.len() == online.len() {
        return 0;
    }
    moved(&online, desktop, &|_| true)
}

/// Gives every processor back to the processes of this user pinned to
/// `desktop`; how many it moved.
pub fn free(desktop: &[usize]) -> usize {
    let online = crate::cpus::online();
    if desktop.is_empty() || desktop.len() == online.len() {
        return 0;
    }
    moved(&desktop.iter().copied().collect(), &online, &|_| true)
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
    let Some(uid) = std::fs::read_to_string("/proc/self/status").ok().and_then(|status| real_uid(&status)) else {
        return 0;
    };
    let Ok(entries) = std::fs::read_dir("/proc") else { return 0 };
    let mut count = 0;
    for entry in entries.flatten() {
        let Some(pid) = entry.file_name().to_str().and_then(|name| name.parse::<i32>().ok()) else { continue };
        if !only(pid) {
            continue;
        }
        let Ok(status) = std::fs::read_to_string(entry.path().join("status")) else { continue };
        let field = |name: &str| status.lines().find_map(|line| line.strip_prefix(name)).map(str::trim);
        if real_uid(&status) != Some(uid) {
            continue;
        }
        let allowed: BTreeSet<usize> = field("Cpus_allowed_list:")
            .and_then(|text| crate::cpus::parse(text).ok())
            .unwrap_or_default()
            .into_iter()
            .collect();
        if &allowed != from {
            continue;
        }
        // Every thread of it: the affinity is a thread's.
        let mut any = false;
        if let Ok(tasks) = std::fs::read_dir(entry.path().join("task")) {
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
        assert_eq!(moved(&all, &desktop, &only), 1);
        assert_eq!(allowed(child.id()), desktop, "pinned to the desktop");
        assert_eq!(moved(&all, &desktop, &only), 0, "pinned already: not allowed everywhere any more");
        assert_eq!(moved(&desktop.iter().copied().collect(), &online, &only), 1);
        assert_eq!(allowed(child.id()), online, "every processor back");
        let _ = child.kill();
        let _ = child.wait();
        assert_eq!(cpus(&online[1..]), desktop);
    }
}
