//! The processors the jobs run on (SPEC.md, §7.5): a list such as `1-3,5-7`
//! in the settings, grouped by physical core as /sys tells it, cut into
//! slots of whole cores, one job per slot; and the affinity of a process.

use std::collections::BTreeSet;

/// The processors in `text`: `1-3,5-7`, `0,4`; empty is none (no pinning).
pub fn parse(text: &str) -> Result<Vec<usize>, String> {
    let mut cpus = BTreeSet::new();
    for part in text.split(',').map(str::trim).filter(|part| !part.is_empty()) {
        let (low, high) = match part.split_once('-') {
            Some((low, high)) => (low.trim(), high.trim()),
            None => (part, part),
        };
        let number = |text: &str| text.parse::<usize>().map_err(|_| format!("job CPUs: \"{part}\" is not a number"));
        let (low, high) = (number(low)?, number(high)?);
        if low > high {
            return Err(format!("job CPUs: {part} goes down"));
        }
        cpus.extend(low..=high);
    }
    Ok(cpus.into_iter().collect())
}

/// `1-3,5-7` for 1, 2, 3, 5, 6, 7.
pub fn text(cpus: &[usize]) -> String {
    let mut parts = Vec::new();
    let mut index = 0;
    while index < cpus.len() {
        let start = cpus[index];
        let mut end = start;
        while index + 1 < cpus.len() && cpus[index + 1] == end + 1 {
            index += 1;
            end = cpus[index];
        }
        parts.push(if start == end { start.to_string() } else { format!("{start}-{end}") });
        index += 1;
    }
    parts.join(",")
}

/// The processors of this machine, from /sys (0 to n-1 when it says nothing).
pub fn online() -> Vec<usize> {
    std::fs::read_to_string("/sys/devices/system/cpu/online")
        .ok()
        .and_then(|text| parse(text.trim()).ok())
        .filter(|cpus| !cpus.is_empty())
        .unwrap_or_else(|| (0..std::thread::available_parallelism().map_or(1, usize::from)).collect())
}

/// The processor's twins on its physical core (itself included), from /sys.
fn siblings(cpu: usize) -> Vec<usize> {
    std::fs::read_to_string(format!("/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list"))
        .ok()
        .and_then(|text| parse(text.trim()).ok())
        .filter(|siblings| siblings.contains(&cpu))
        .unwrap_or_else(|| vec![cpu])
}

/// The processors grouped by physical core, in order: `1-3,5-7` on a
/// 4-core 8-thread machine is [1,5], [2,6], [3,7].
pub fn cores(cpus: &[usize]) -> Vec<Vec<usize>> {
    cores_by(cpus, siblings)
}

fn cores_by(cpus: &[usize], siblings: impl Fn(usize) -> Vec<usize>) -> Vec<Vec<usize>> {
    let mut cores: Vec<Vec<usize>> = Vec::new();
    for &cpu in cpus {
        if cores.iter().any(|core| core.contains(&cpu)) {
            continue;
        }
        let core: Vec<usize> = siblings(cpu).into_iter().filter(|twin| cpus.contains(twin)).collect();
        cores.push(core);
    }
    cores
}

/// The slots of a pool: `threads` whole cores each, as many as fit (one at
/// least, all the cores when a job wants more than there are).
pub fn slots(cores: &[Vec<usize>], threads: u32) -> Vec<Vec<usize>> {
    let per_slot = (threads.max(1) as usize).min(cores.len().max(1));
    cores
        .chunks(per_slot)
        .filter(|chunk| chunk.len() == per_slot)
        .map(|chunk| {
            let mut cpus: Vec<usize> = chunk.iter().flatten().copied().collect();
            cpus.sort_unstable();
            cpus
        })
        .collect()
}

/// The settings' text checked against this machine: its processors, or why not.
pub fn checked(text: &str) -> Result<Vec<usize>, String> {
    let cpus = parse(text)?;
    let online = online();
    if let Some(missing) = cpus.iter().find(|cpu| !online.contains(cpu)) {
        return Err(format!("job CPUs: this machine has no processor {missing} (it has {})", self::text(&online)));
    }
    Ok(cpus)
}

/// The physical cores of this machine, as the Machine screen shows them: "core 0: 0,4 · core 1: 1,5".
pub fn layout() -> String {
    cores(&online())
        .iter()
        .enumerate()
        .map(|(index, core)| {
            format!("core {index}: {}", core.iter().map(usize::to_string).collect::<Vec<_>>().join(","))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_list_is_read_and_written_back() {
        assert_eq!(parse("1-3,5-7").unwrap(), vec![1, 2, 3, 5, 6, 7]);
        assert_eq!(parse(" 0, 4 ").unwrap(), vec![0, 4]);
        assert_eq!(parse("").unwrap(), Vec::<usize>::new());
        assert!(parse("3-1").is_err());
        assert!(parse("a").is_err());
        assert_eq!(text(&[1, 2, 3, 5, 6, 7]), "1-3,5-7");
        assert_eq!(text(&[0, 4]), "0,4");
        assert_eq!(text(&[]), "");
    }

    /// A 4-core 8-thread machine: core k holds k and k + 4.
    fn twins(cpu: usize) -> Vec<usize> {
        vec![cpu % 4, cpu % 4 + 4]
    }

    #[test]
    fn cores_and_slots_take_whole_cores() {
        let cores = cores_by(&[1, 2, 3, 5, 6, 7], twins);
        assert_eq!(cores, vec![vec![1, 5], vec![2, 6], vec![3, 7]]);
        assert_eq!(slots(&cores, 1), vec![vec![1, 5], vec![2, 6], vec![3, 7]], "one job per core");
        assert_eq!(slots(&cores, 3), vec![vec![1, 2, 3, 5, 6, 7]], "one job on the three");
        assert_eq!(slots(&cores, 4), vec![vec![1, 2, 3, 5, 6, 7]], "more threads than cores: all of them");
        assert_eq!(slots(&cores, 2), vec![vec![1, 2, 5, 6]], "the third core is left idle");
        // A twin left out stays out: the core is the processors given.
        assert_eq!(cores_by(&[1, 2], twins), vec![vec![1], vec![2]]);
    }

    #[test]
    fn this_machine_is_read() {
        let online = online();
        assert!(!online.is_empty());
        assert_eq!(cores(&online).iter().map(Vec::len).sum::<usize>(), online.len());
        assert!(checked(&text(&online)).is_ok());
        assert!(checked("100000").is_err());
    }
}
