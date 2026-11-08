//! One memory per machine, met in git (SPEC.md, §4b): the branches
//! `lab/runs-*` of the mirror read into `<lab>/machines/<machine>.jsonl`,
//! their union with the live memory of this machine, the records of this
//! machine not yet pushed, and the push itself.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::home::Places;
use crate::mirror::Mirror;
use crate::model::Run;
use crate::store::History;

/// The folder of the other machines' memories.
pub fn folder(places: &Places) -> PathBuf {
    places.data.join("machines")
}

/// The file of a machine's memory, as last fetched.
pub fn file(places: &Places, machine: &str) -> PathBuf {
    folder(places).join(format!("{machine}.jsonl"))
}

/// Writes the memory of every machine the mirror has a branch for into
/// `<lab>/machines/`, and removes the file of a machine whose branch is gone:
/// the folder says what the repository said at the last fetch. Returns the
/// machines, by name.
pub fn refresh(places: &Places, mirror: &Mirror) -> Result<Vec<String>, String> {
    if !mirror.exists() {
        return Ok(known(places));
    }
    let machines = mirror.machines();
    let folder = folder(places);
    for machine in &machines {
        if let Some(text) = mirror.runs_of(machine) {
            let mut text = text;
            if !text.ends_with('\n') {
                text.push('\n');
            }
            crate::store::write_atomically(&file(places, machine), text.as_bytes())?;
        }
    }
    for machine in known(places) {
        if !machines.contains(&machine) {
            let _ = std::fs::remove_file(folder.join(format!("{machine}.jsonl")));
        }
    }
    Ok(machines)
}

/// The machines whose memory the folder holds, by name.
pub fn known(places: &Places) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(folder(places)) else { return Vec::new() };
    let mut machines: Vec<String> = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            name.strip_suffix(".jsonl").map(str::to_string)
        })
        .collect();
    machines.sort();
    machines
}

/// The memories met: one run per id, the latest `edited` wins, this
/// machine's line on a tie.
#[derive(Clone, Debug, Default)]
pub struct Union {
    pub history: History,
    /// The machine each run comes from: the one whose line was kept.
    pub origin: BTreeMap<String, String>,
    /// Every machine met, this one first.
    pub machines: Vec<String>,
}

/// The union of the other machines' memories and this machine's live one.
pub fn union(places: &Places, this: &str) -> Result<Union, String> {
    let mut sources: Vec<(String, History)> = Vec::new();
    for machine in known(places) {
        if machine == this {
            // The live memory says what this machine knows; its branch lags behind it.
            continue;
        }
        let history = History::read(&file(places, &machine))?;
        sources.push((machine, history));
    }
    sources.push((this.to_string(), History::read(&places.runs())?));
    Ok(merge(sources))
}

fn merge(sources: Vec<(String, History)>) -> Union {
    let mut by_id: BTreeMap<String, (Run, String)> = BTreeMap::new();
    let mut problems = Vec::new();
    let mut machines = Vec::new();
    for (machine, history) in sources {
        problems.extend(history.problems.into_iter().map(|problem| format!("{machine}: {problem}")));
        for run in history.runs {
            let keep = by_id.get(&run.id).is_none_or(|(known, _)| run.edited >= known.edited);
            if keep {
                by_id.insert(run.id.clone(), (run, machine.clone()));
            }
        }
        machines.push(machine);
    }
    machines.rotate_right(1);
    let mut origin = BTreeMap::new();
    let mut runs = Vec::new();
    for (id, (run, machine)) in by_id {
        origin.insert(id, machine);
        runs.push(run);
    }
    runs.sort_by(|a, b| a.started.cmp(&b.started).then_with(|| a.id.cmp(&b.id)));
    Union { history: History { runs, problems }, origin, machines }
}

/// The runs of the live memory the branch of this machine does not have as
/// they are: new, or edited since the last push.
pub fn unpushed(places: &Places, mirror: &Mirror, machine: &str) -> Result<usize, String> {
    let live = History::read(&places.runs())?;
    let pushed = mirror.runs_of(machine).map(|text| History::parse(&text)).unwrap_or_default();
    let lines =
        |runs: &[Run]| -> Vec<String> { runs.iter().filter_map(|run| serde_json::to_string(run).ok()).collect() };
    let known = lines(&pushed.runs);
    Ok(lines(&live.runs).into_iter().filter(|line| !known.contains(line)).count())
}

/// Pushes the live memory of this machine to its branch (SPEC.md, §4b),
/// united with what the branch holds already so that a memory wiped on the
/// machine loses nothing. Says what was pushed.
pub fn push(places: &Places, mirror: &Mirror, machine: &str) -> Result<String, String> {
    let live = History::read(&places.runs())?;
    let before = mirror.runs_of(machine).map(|text| History::parse(&text)).unwrap_or_default();
    let count = live.runs.len();
    let united = merge(vec![(machine.to_string(), before), (machine.to_string(), live)]).history;
    let mut text = String::new();
    for run in &united.runs {
        let line = serde_json::to_string(run).map_err(|error| error.to_string())?;
        if let Some(path) = crate::catalogue::local_path(&line) {
            return Err(format!("run {} names a local path ({path}): nothing pushed", run.id));
        }
        text.push_str(&line);
        text.push('\n');
    }
    let message = format!("lab: the memory of {machine}, {} runs", united.runs.len());
    let said = mirror.push_runs(machine, &text, &message)?;
    if united.runs.len() > count {
        return Ok(format!("{said}, {} of them from the branch alone", united.runs.len() - count));
    }
    Ok(said)
}

/// The union written as `lab/runs.jsonl` of a clone, to commit with the PR
/// that cites the runs (SPEC.md, §4b).
pub fn publish(history: &History, clone: &Path) -> Result<usize, String> {
    if !clone.join("lab").is_dir() {
        return Err(format!("{}: not a clone with a lab/ folder", clone.display()));
    }
    let mut text = String::new();
    for run in &history.runs {
        let line = serde_json::to_string(run).map_err(|error| error.to_string())?;
        if let Some(path) = crate::catalogue::local_path(&line) {
            return Err(format!("run {} names a local path ({path}): nothing copied", run.id));
        }
        text.push_str(&line);
        text.push('\n');
    }
    crate::store::write_atomically(&clone.join("lab").join("runs.jsonl"), text.as_bytes())?;
    Ok(history.runs.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../tests/fixtures/runs.jsonl");

    fn places(name: &str) -> Places {
        let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join("test-data").join(name);
        let _ = std::fs::remove_dir_all(&data);
        std::fs::create_dir_all(&data).unwrap();
        Places { home: None, data, catalogue: None }
    }

    fn edited(line: &str, comment: &str, when: &str) -> String {
        let mut run: Run = serde_json::from_str(line).unwrap();
        run.comment = comment.to_string();
        run.edited = Some(when.parse().unwrap());
        serde_json::to_string(&run).unwrap()
    }

    #[test]
    fn the_memories_unite_and_the_latest_edit_wins() {
        let places = places("union");
        let lines: Vec<&str> = FIXTURE.lines().collect();
        // This machine has the three runs; night has the first, edited later, and one of its own.
        std::fs::write(places.runs(), FIXTURE).unwrap();
        std::fs::create_dir_all(folder(&places)).unwrap();
        let own = lines[0].replace("20261106-223912-forced-whole-lower", "20261107-010000-forced-whole-lower");
        assert_ne!(own, lines[0]);
        let night = format!("{}\n{own}\n", edited(lines[0], "edited at night", "2026-11-07T09:00:00+01:00"));
        std::fs::write(file(&places, "night"), night).unwrap();
        // A stale file of this machine's own branch is not read: the live memory is.
        std::fs::write(file(&places, "box"), "{not json\n").unwrap();
        let union = union(&places, "box").unwrap();
        assert!(union.history.problems.is_empty(), "{:?}", union.history.problems);
        assert_eq!(union.machines, ["box", "night"]);
        assert_eq!(union.history.runs.len(), 4);
        let forced = union.history.runs.iter().find(|run| run.id == "20261106-223912-forced-whole-lower").unwrap();
        assert_eq!(forced.comment, "edited at night");
        assert_eq!(union.origin[&forced.id], "night");
        assert_eq!(union.origin["20261106-153212-quick-bench"], "box");
        assert_eq!(union.origin["20261107-010000-forced-whole-lower"], "night");
        assert_eq!(known(&places), ["box", "night"]);
    }

    #[test]
    fn the_branches_are_read_into_the_folder_and_the_live_memory_is_pushed() {
        let places = places("machines");
        let mirror = crate::mirror::tests::mirror_at_head("machines-mirror");
        let remote = crate::mirror::tests::remote_of(&mirror);
        assert!(refresh(&places, &mirror).unwrap().is_empty());
        std::fs::write(places.runs(), FIXTURE).unwrap();
        assert_eq!(unpushed(&places, &mirror, "box").unwrap(), 3, "nothing pushed yet");
        let said = push(&places, &mirror, "box").unwrap();
        assert_eq!(said, "created lab/runs-box with 3 runs");
        assert_eq!(unpushed(&places, &mirror, "box").unwrap(), 0);
        // An edit here is one record to push; a record pushed from the branch alone is kept.
        let lines: Vec<&str> = FIXTURE.lines().collect();
        let mut text = format!("{}\n{}\n", lines[0], lines[1]);
        text.push_str(&edited(lines[2], "edited here", "2026-11-08T10:00:00+01:00"));
        text.push('\n');
        std::fs::write(places.runs(), &text).unwrap();
        assert_eq!(unpushed(&places, &mirror, "box").unwrap(), 1);
        std::fs::write(places.runs(), format!("{}\n", lines[1])).unwrap();
        assert_eq!(
            push(&places, &mirror, "box").unwrap(),
            "pushed 3 runs to lab/runs-box, 2 of them from the branch alone"
        );
        // Another machine's branch at the remote comes into the folder with a fetch.
        let at_remote = std::process::Command::new("git")
            .args(["-C", &remote.to_string_lossy(), "rev-parse", "lab/runs-box"])
            .output()
            .unwrap();
        let hash = String::from_utf8_lossy(&at_remote.stdout).trim().to_string();
        std::process::Command::new("git")
            .args(["-C", &remote.to_string_lossy(), "update-ref", "refs/heads/lab/runs-night", &hash])
            .status()
            .unwrap();
        mirror.fetch(None).unwrap();
        assert_eq!(refresh(&places, &mirror).unwrap(), ["box", "night"]);
        assert_eq!(History::read(&file(&places, "night")).unwrap().runs.len(), 3);
        let union = union(&places, "box").unwrap();
        assert_eq!(union.history.runs.len(), 3, "the live memory and night's, one line per id");
        assert_eq!(union.origin["20261106-153212-quick-bench"], "box", "this machine's line on a tie");
        // The branch gone at the remote: its file goes.
        std::process::Command::new("git")
            .args(["-C", &remote.to_string_lossy(), "update-ref", "-d", "refs/heads/lab/runs-night"])
            .status()
            .unwrap();
        mirror.fetch(None).unwrap();
        assert_eq!(refresh(&places, &mirror).unwrap(), ["box"]);
        assert!(!file(&places, "night").exists());
        // Published: the union, in a clone.
        let clone = places.data.join("clone");
        std::fs::create_dir_all(clone.join("lab")).unwrap();
        assert_eq!(publish(&union.history, &clone).unwrap(), 3);
        assert_eq!(History::read(&clone.join("lab").join("runs.jsonl")).unwrap().runs.len(), 3);
    }
}
