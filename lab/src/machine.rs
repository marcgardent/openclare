//! What a record says of the machine: its fixed facts, read once, and a
//! sample of its load at the start and the end of a run. Read from /proc and
//! `nvidia-smi`; the CPU percentages need two readings apart and come with
//! the monitor (milestone M3).

use std::process::Command;

use crate::model::{Gpu, Machine, Sample};

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// The value of `name` in a "name : value" file such as /proc/cpuinfo.
fn field<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        (key.trim() == name).then(|| value.trim())
    })
}

/// A size of /proc/meminfo ("62793516 kB") in bytes.
fn kilobytes(text: &str, name: &str) -> Option<u64> {
    field(text, name)?.split_whitespace().next()?.parse::<u64>().ok().map(|kb| kb * 1024)
}

/// The hostname, the default name of this machine (SPEC.md, §12, Q17);
/// "machine" if /proc does not say.
pub fn hostname() -> String {
    let name = read("/proc/sys/kernel/hostname").trim().to_string();
    if name.is_empty() { "machine".to_string() } else { name }
}

pub fn machine(label: &str) -> Machine {
    let cpuinfo = read("/proc/cpuinfo");
    let meminfo = read("/proc/meminfo");
    Machine {
        hostname: hostname(),
        cpu: field(&cpuinfo, "model name").unwrap_or("unknown").to_string(),
        cores: field(&cpuinfo, "cpu cores").and_then(|cores| cores.parse().ok()).unwrap_or(0),
        threads: cpuinfo.lines().filter(|line| line.starts_with("processor")).count() as u32,
        memory_bytes: kilobytes(&meminfo, "MemTotal").unwrap_or(0),
        gpu: gpu(),
        kernel: read("/proc/sys/kernel/osrelease").trim().to_string(),
        label: label.to_string(),
    }
}

/// The first GPU `nvidia-smi` lists; none if the command is missing or fails.
fn gpu() -> Option<Gpu> {
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=name,memory.total", "--format=csv,noheader,nounits"])
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let text = String::from_utf8_lossy(&output.stdout);
    let (name, mebibytes) = text.lines().next()?.rsplit_once(',')?;
    Some(Gpu { name: name.trim().to_string(), memory_bytes: mebibytes.trim().parse::<u64>().ok()? << 20 })
}

/// The load averages and the memory in use now.
pub fn sample() -> Sample {
    let averages: Vec<f64> =
        read("/proc/loadavg").split_whitespace().take(3).filter_map(|value| value.parse().ok()).collect();
    let meminfo = read("/proc/meminfo");
    let used =
        kilobytes(&meminfo, "MemTotal").zip(kilobytes(&meminfo, "MemAvailable")).map(|(total, free)| total - free);
    Sample {
        load: [0, 1, 2].map(|index| averages.get(index).copied().unwrap_or(0.0)),
        cpu_percent: None,
        foreign_cpu_percent: None,
        memory_used_bytes: used,
        vram_used_bytes: None,
        gpu_percent: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_machine_reads() {
        let machine = machine("label");
        assert!(machine.threads > 0 && machine.memory_bytes > 0, "{machine:?}");
        let sample = sample();
        assert!(sample.memory_used_bytes.is_some_and(|used| used < machine.memory_bytes));
    }

    #[test]
    fn fields_read_with_their_spaces() {
        assert_eq!(field("model name\t: Intel(R) Core(TM) i7-7700K\ncpu cores\t: 4\n", "cpu cores"), Some("4"));
        assert_eq!(kilobytes("MemTotal:       62793516 kB\n", "MemTotal"), Some(62793516 * 1024));
    }
}
