//! The monitor of the machine and the calm indicator (SPEC.md, section 9).
//!
//! A thread samples /proc every few seconds: CPU in all and per thread, the
//! CPU of each process, load averages and memory; and `nvidia-smi` for the
//! GPU. The processes of the run under way are its process group and their
//! descendants; the rest of the CPU is foreign. Their CPU is counted whole,
//! that of the processes born and ended between two samples included (see
//! `our_ticks`). The calm verdict is drawn from the last window of samples.

use std::collections::{BTreeMap, VecDeque};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::settings::CalmSettings;

/// The samples kept, for the sparklines and the window of the verdict.
const KEPT: Duration = Duration::from_secs(600);

/// Clock ticks per second of /proc/<pid>/stat (USER_HZ, 100 on Linux).
const TICKS: f64 = 100.0;

/// The processes of the machine, by pid: name, parent, process group, CPU
/// ticks of their own, CPU ticks of their children reaped.
type Processes = BTreeMap<i32, (String, i32, i32, u64, u64)>;

#[derive(Clone, Debug)]
pub struct Point {
    pub at: Instant,
    /// Percent of the whole machine; 100 is every thread busy.
    pub cpu: f64,
    pub per_thread: Vec<f64>,
    /// The run's processes, and the others, in percent of the whole machine.
    pub ours: f64,
    pub foreign: f64,
    /// With job processors set (SPEC.md, §7.5): the foreign CPU on them, in
    /// percent of them; what the calm is judged on.
    pub foreign_jobs: Option<f64>,
    pub load: [f64; 3],
    pub memory_used: u64,
    pub memory_total: u64,
    pub vram: Option<(u64, u64)>,
    pub gpu: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Process {
    pub name: String,
    /// Percent of one thread.
    pub cpu: f64,
    pub ours: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calmness {
    /// The window is not full yet.
    Measuring,
    Calm,
    /// Busy with our run only.
    Ours,
    /// Something else loads the machine.
    Busy,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Verdict {
    pub calmness: Calmness,
    pub text: String,
}

/// What the sampler shares with the window and the runner.
#[derive(Debug, Default)]
pub struct Watch {
    pub points: VecDeque<Point>,
    /// The heaviest processes at the last sample.
    pub heaviest: Vec<Process>,
    /// The process groups of the runs under way, set by the runner: one, or
    /// the jobs of a batch's pool (SPEC.md, §6b).
    pub groups: Vec<i32>,
    /// When a run of ours last ended.
    pub our_last_end: Option<Instant>,
    /// When the machine was last seen busy by something else.
    pub last_busy: Option<Instant>,
    pub started: Option<Instant>,
}

impl Watch {
    pub fn latest(&self) -> Option<&Point> {
        self.points.back()
    }

    /// The calm verdict over the last window (SPEC.md, section 9).
    pub fn verdict(&self, calm: &CalmSettings) -> Verdict {
        let now = Instant::now();
        let window = Duration::from_secs(u64::from(calm.window_seconds.max(2)));
        let Some(started) = self.started else {
            return Verdict { calmness: Calmness::Measuring, text: "measuring".to_string() };
        };
        if now.duration_since(started) < window {
            let done = now.duration_since(started).as_secs();
            return Verdict {
                calmness: Calmness::Measuring,
                text: format!("measuring · {done} s of {}", window.as_secs()),
            };
        }
        let points: Vec<&Point> = self.points.iter().filter(|point| now.duration_since(point.at) <= window).collect();
        if points.is_empty() {
            return Verdict { calmness: Calmness::Measuring, text: "measuring".to_string() };
        }
        // Pinned, the calm is judged on the job processors alone; the rest is the desktop's.
        // The sampler measured the job processors when there are some.
        let job_cpus = crate::cpus::job_cpus(&calm.job_cpus).unwrap_or_default();
        let pinned = !job_cpus.is_empty() && points.iter().any(|point| point.foreign_jobs.is_some());
        let foreign = |point: &&Point| if pinned { point.foreign_jobs.unwrap_or(point.foreign) } else { point.foreign };
        let mean = points.iter().map(foreign).sum::<f64>() / points.len() as f64;
        let peak = points.iter().map(foreign).fold(0.0, f64::max);
        let place =
            if pinned { format!(" on the job processors {}", crate::cpus::text(&job_cpus)) } else { String::new() };
        let foreign_names = || {
            let names: Vec<String> = self
                .heaviest
                .iter()
                .filter(|process| !process.ours && process.cpu >= 5.0)
                .take(3)
                .map(|process| format!("{} {:.0} %", process.name, process.cpu))
                .collect();
            if names.is_empty() { String::new() } else { format!(" · {}", names.join(", ")) }
        };
        if mean >= calm.foreign_cpu_mean_max || peak >= calm.foreign_cpu_peak_max {
            return Verdict {
                calmness: Calmness::Busy,
                text: format!("foreign CPU{place} mean {mean:.1} %, peak {peak:.1} %{}", foreign_names()),
            };
        }
        let ours = !self.groups.is_empty();
        let load1 = points.last().map_or(0.0, |point| point.load[0]);
        let after_ours = self.our_last_end.is_some_and(|end| {
            now.duration_since(end) < Duration::from_secs(60 * u64::from(calm.load1_ignored_minutes))
        });
        // load1 counts the whole machine: pinned, the desktop's share of it does not matter.
        if !pinned && !ours && !after_ours && load1 >= calm.load1_max {
            return Verdict { calmness: Calmness::Busy, text: format!("load1 {load1:.2}{}", foreign_names()) };
        }
        if ours {
            return Verdict {
                calmness: Calmness::Ours,
                text: format!("busy with our run · foreign CPU{place} {mean:.1} %"),
            };
        }
        let since = self.last_busy.map_or(now.duration_since(started), |busy| now.duration_since(busy));
        Verdict {
            calmness: Calmness::Calm,
            text: format!("calm for {}", crate::model::duration_text(since.as_secs_f64().floor())),
        }
    }
}

/// The counters of /proc/stat: per CPU, (busy, all) ticks.
fn cpu_ticks() -> Vec<(u64, u64)> {
    let text = std::fs::read_to_string("/proc/stat").unwrap_or_default();
    text.lines()
        .filter(|line| line.starts_with("cpu"))
        .map(|line| {
            let values: Vec<u64> = line.split_whitespace().skip(1).filter_map(|value| value.parse().ok()).collect();
            let all: u64 = values.iter().take(8).sum();
            let idle = values.get(3).copied().unwrap_or(0) + values.get(4).copied().unwrap_or(0);
            (all - idle, all)
        })
        .collect()
}

/// Each process: (pid, (name, parent, group, ticks used)).
fn processes() -> Processes {
    let mut found = BTreeMap::new();
    let Ok(entries) = std::fs::read_dir("/proc") else { return found };
    for entry in entries.flatten() {
        let Ok(pid) = entry.file_name().to_string_lossy().parse::<i32>() else { continue };
        let Ok(text) = std::fs::read_to_string(entry.path().join("stat")) else { continue };
        // "pid (name) state ppid pgrp ... utime stime ...": the name may hold spaces.
        let (Some(open), Some(close)) = (text.find('('), text.rfind(')')) else { continue };
        let name = text[open + 1..close].to_string();
        let fields: Vec<&str> = text[close + 1..].split_whitespace().collect();
        let number = |index: usize| fields.get(index).and_then(|value| value.parse::<i64>().ok()).unwrap_or(0);
        // After the name: state 0, ppid 1, pgrp 2, ... utime 11, stime 12, cutime 13, cstime 14.
        let own = (number(11) + number(12)) as u64;
        found.insert(pid, (name, number(1) as i32, number(2) as i32, own, (number(13) + number(14)) as u64));
    }
    found
}

fn gpu() -> Option<((u64, u64), f64)> {
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=memory.used,memory.total,utilization.gpu", "--format=csv,noheader,nounits"])
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let text = String::from_utf8_lossy(&output.stdout);
    let values: Vec<f64> = text.lines().next()?.split(',').filter_map(|value| value.trim().parse().ok()).collect();
    let mebibyte = 1u64 << 20;
    Some((((values.first()? * mebibyte as f64) as u64, (values.get(1)? * mebibyte as f64) as u64), *values.get(2)?))
}

fn memory() -> (u64, u64) {
    let text = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let field = |name: &str| -> u64 {
        text.lines()
            .find(|line| line.starts_with(name))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0)
            * 1024
    };
    let total = field("MemTotal:");
    (total.saturating_sub(field("MemAvailable:")), total)
}

fn load() -> [f64; 3] {
    let text = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let values: Vec<f64> = text.split_whitespace().take(3).filter_map(|value| value.parse().ok()).collect();
    [0, 1, 2].map(|index| values.get(index).copied().unwrap_or(0.0))
}

/// The processes of the groups and their descendants.
fn ours(groups: &[i32], all: &Processes) -> Vec<i32> {
    if groups.is_empty() {
        return Vec::new();
    }
    let mut found: Vec<i32> =
        all.iter().filter(|(_, (_, _, pgrp, _, _))| groups.contains(pgrp)).map(|(pid, _)| *pid).collect();
    let mut grew = true;
    while grew {
        grew = false;
        for (pid, (_, parent, _, _, _)) in all {
            if !found.contains(pid) && found.contains(parent) {
                found.push(*pid);
                grew = true;
            }
        }
    }
    found
}

/// The CPU ticks of the children this process reaped (its cutime and cstime).
fn reaped_ticks() -> u64 {
    let text = std::fs::read_to_string("/proc/self/stat").unwrap_or_default();
    let fields: Vec<&str> = text.rfind(')').map_or(Vec::new(), |close| text[close + 1..].split_whitespace().collect());
    let number = |index: usize| fields.get(index).and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    number(13) + number(14)
}

/// Our runs' CPU between two samples, in ticks. Each process of ours alive
/// counts its time and that of its children reaped, against the sample
/// before: a process born between the two counts whole, one that ended hands
/// its time to its parent, which reaped it. A run's first process ends
/// reaped by lab itself: `reaped`, lab's reaped ticks over the same span,
/// counts only when one did, lab's other children (git) taking a few ticks.
/// Before, only the processes alive at both samples counted, and what ours
/// did at their start and end went to the foreign CPU.
fn our_ticks(before: &Processes, before_ours: &[i32], now: &Processes, ours: &[i32], reaped: u64) -> u64 {
    let whole = |all: &Processes, pids: &[i32]| -> u64 {
        pids.iter().filter_map(|pid| all.get(pid)).map(|(_, _, _, own, children)| own + children).sum()
    };
    let me = std::process::id() as i32;
    let run_ended = before_ours
        .iter()
        .any(|pid| before.get(pid).is_some_and(|(_, parent, ..)| *parent == me) && !now.contains_key(pid));
    (whole(now, ours) + if run_ended { reaped } else { 0 }).saturating_sub(whole(before, before_ours))
}

/// The sampler's thread, for as long as the app lives.
pub fn spawn(
    watch: Arc<Mutex<Watch>>,
    calm: Arc<Mutex<CalmSettings>>,
    period: Duration,
    repaint: Box<dyn Fn() + Send>,
) {
    std::thread::Builder::new()
        .name("monitor".to_string())
        .spawn(move || {
            let threads = cpu_ticks().len().saturating_sub(1).max(1) as f64;
            let mut before_cpu = cpu_ticks();
            let mut before_processes = processes();
            let mut before_ours = ours(&lock(&watch).groups.clone(), &before_processes);
            let mut before_at = Instant::now();
            // nvidia-smi is a child of lab too: its ticks are not a run's.
            let mut gpu_ticks = reaped_ticks();
            let mut gpu_reading = gpu();
            gpu_ticks = reaped_ticks() - gpu_ticks;
            let mut before_reaped = reaped_ticks() - gpu_ticks;
            let mut count: u64 = 0;
            lock(&watch).started = Some(Instant::now());
            loop {
                std::thread::sleep(period);
                count += 1;
                let now = Instant::now();
                let seconds = now.duration_since(before_at).as_secs_f64().max(0.1);
                let cpu = cpu_ticks();
                let percent = |index: usize| -> f64 {
                    match (before_cpu.get(index), cpu.get(index)) {
                        (Some((busy0, all0)), Some((busy1, all1))) if all1 > all0 => {
                            100.0 * (busy1 - busy0) as f64 / (all1 - all0) as f64
                        }
                        _ => 0.0,
                    }
                };
                let total = percent(0);
                let per_thread: Vec<f64> = (1..cpu.len()).map(percent).collect();
                let processes = processes();
                let groups = lock(&watch).groups.clone();
                let ours_pids = ours(&groups, &processes);
                let mut heaviest: Vec<Process> = processes
                    .iter()
                    .filter_map(|(pid, (name, _, _, ticks, _))| {
                        // Born since the sample before: all its time is of this span.
                        let ticks0 = before_processes.get(pid).map_or(0, |(_, _, _, ticks0, _)| *ticks0);
                        let used = 100.0 * ticks.saturating_sub(ticks0) as f64 / TICKS / seconds;
                        (used > 0.0).then(|| Process { name: name.clone(), cpu: used, ours: ours_pids.contains(pid) })
                    })
                    .collect();
                heaviest.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
                let reaped = reaped_ticks() - gpu_ticks;
                let ours_ticks = our_ticks(
                    &before_processes,
                    &before_ours,
                    &processes,
                    &ours_pids,
                    reaped.saturating_sub(before_reaped),
                );
                // Percent of one thread.
                let ours_cpu = 100.0 * ours_ticks as f64 / TICKS / seconds;
                let ours_percent = (ours_cpu / threads).max(0.0);
                let foreign = (total - ours_percent).max(0.0);
                // The job processors: their busy time less ours (pinned to them), per processor.
                let job_cpus = {
                    let calm = calm.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    crate::cpus::job_cpus(&calm.job_cpus).unwrap_or_default()
                };
                let foreign_jobs = (!job_cpus.is_empty()).then(|| {
                    let busy: f64 = job_cpus.iter().filter_map(|cpu| per_thread.get(*cpu)).sum();
                    ((busy - ours_cpu) / job_cpus.len() as f64).max(0.0)
                });
                heaviest.truncate(12);
                if count % 2 == 0 {
                    let start = reaped_ticks();
                    gpu_reading = gpu();
                    gpu_ticks += reaped_ticks().saturating_sub(start);
                }
                let (memory_used, memory_total) = memory();
                let point = Point {
                    at: now,
                    cpu: total,
                    per_thread,
                    ours: ours_percent,
                    foreign,
                    foreign_jobs,
                    load: load(),
                    memory_used,
                    memory_total,
                    vram: gpu_reading.map(|(vram, _)| vram),
                    gpu: gpu_reading.map(|(_, percent)| percent),
                };
                {
                    let mut watch = lock(&watch);
                    watch.points.push_back(point);
                    while watch.points.front().is_some_and(|point| now.duration_since(point.at) > KEPT) {
                        watch.points.pop_front();
                    }
                    watch.heaviest = heaviest;
                    let calm = calm.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
                    if watch.verdict(&calm).calmness == Calmness::Busy {
                        watch.last_busy = Some(now);
                    }
                }
                before_cpu = cpu;
                before_processes = processes;
                before_ours = ours_pids;
                before_reaped = reaped;
                before_at = now;
                repaint();
            }
        })
        .expect("the monitor thread starts");
}

pub fn lock(watch: &Arc<Mutex<Watch>>) -> std::sync::MutexGuard<'_, Watch> {
    watch.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(seconds_ago: u64, foreign: f64, load1: f64) -> Point {
        Point {
            at: Instant::now() - Duration::from_secs(seconds_ago),
            cpu: foreign,
            per_thread: Vec::new(),
            ours: 0.0,
            foreign,
            foreign_jobs: None,
            load: [load1, 0.0, 0.0],
            memory_used: 0,
            memory_total: 1,
            vram: None,
            gpu: None,
        }
    }

    fn watch(points: Vec<Point>) -> Watch {
        Watch { points: points.into(), started: Some(Instant::now() - Duration::from_secs(120)), ..Watch::default() }
    }

    #[test]
    fn the_three_states() {
        let calm = CalmSettings::default();
        let quiet = watch((0..30).map(|second| point(second * 2, 1.0, 0.2)).collect());
        assert_eq!(quiet.verdict(&calm).calmness, Calmness::Calm);
        let mut ours = watch((0..30).map(|second| point(second * 2, 1.0, 3.0)).collect());
        ours.groups = vec![1];
        assert_eq!(ours.verdict(&calm).calmness, Calmness::Ours, "load1 counts our run: not looked at");
        let busy = watch((0..30).map(|second| point(second * 2, if second == 3 { 40.0 } else { 1.0 }, 0.2)).collect());
        assert_eq!(busy.verdict(&calm).calmness, Calmness::Busy, "one sample above the peak");
        let loaded = watch((0..30).map(|second| point(second * 2, 1.0, 1.5)).collect());
        assert_eq!(loaded.verdict(&calm).calmness, Calmness::Busy, "load1 with no run of ours");
        let mut after = watch((0..30).map(|second| point(second * 2, 1.0, 1.5)).collect());
        after.our_last_end = Some(Instant::now() - Duration::from_secs(60));
        assert_eq!(after.verdict(&calm).calmness, Calmness::Calm, "load1 ignored after our run");
        let fresh = Watch { started: Some(Instant::now()), ..Watch::default() };
        assert_eq!(fresh.verdict(&calm).calmness, Calmness::Measuring);
    }

    #[test]
    fn this_machine_is_read() {
        assert!(cpu_ticks().len() >= 2);
        let all = processes();
        let me = std::process::id() as i32;
        assert!(all.contains_key(&me));
        let group = all[&me].2;
        assert!(ours(&[group], &all).contains(&me));
        assert!(memory().1 > 0);
    }

    #[test]
    fn short_processes_of_ours_are_counted_whole() {
        use std::os::unix::process::CommandExt;
        // A run of short busy processes, each born and ended between two samples.
        let folder = std::env::temp_dir().join(format!("lab-our-ticks-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let end = folder.join("end");
        let script = format!(
            "for n in $(seq 25); do sh -c 'i=0; while [ $i -lt 20000 ]; do i=$((i+1)); done'; done; cat /proc/$$/stat > {}",
            end.display()
        );
        let before_reaped = reaped_ticks();
        let mut before = processes();
        let mut child = std::process::Command::new("sh").arg("-c").arg(&script).process_group(0).spawn().unwrap();
        let group = child.id() as i32;
        let mut before_ours = ours(&[group], &before);
        let mut last_reaped = before_reaped;
        let mut counted = 0;
        loop {
            std::thread::sleep(Duration::from_millis(150));
            let done = child.try_wait().unwrap().is_some();
            let now = processes();
            let ours_now = ours(&[group], &now);
            let reaped = reaped_ticks();
            counted += our_ticks(&before, &before_ours, &now, &ours_now, reaped - last_reaped);
            (before, before_ours, last_reaped) = (now, ours_now, reaped);
            if done {
                break;
            }
        }
        // The run's own word: the shell's ticks and its children's, read as it ended.
        let text = std::fs::read_to_string(&end).unwrap();
        let fields: Vec<u64> =
            text[text.rfind(')').unwrap() + 1..].split_whitespace().map(|value| value.parse().unwrap_or(0)).collect();
        let truth = fields[11] + fields[12] + fields[13] + fields[14];
        std::fs::remove_dir_all(&folder).unwrap();
        assert!(truth >= 20, "the run took {truth} ticks: too short to tell");
        assert!(counted * 10 >= truth * 9, "counted {counted} ticks of {truth}");
        assert!(counted <= last_reaped - before_reaped + 5, "counted {counted} ticks, more than this process reaped");
    }
}
