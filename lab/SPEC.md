# lab: the record and the launcher of openclare's computations

Status: **draft for validation**. No code is written before Marc validates this document.
French mirror: `SPEC.fr.md`.

Amendment of 2026-11-08, **draft for validation**: lab becomes a program of its own, installed on
the machine, which fetches the repository, checks out the commit asked for and runs the job there; one
memory per machine in `~/.openclare-lab`, met in git on a branch of that machine (§1, §2, §3, §4, §4b,
§5, §6, §7.3, §8.1, §10b, §11, §12, §13, §14, §15, §17).

Amendment R1 of 2026-11-13, **validated by Marc the same day**: a job is one computation, timed and judged by lab;
a batch is a grid of jobs, declared as `[[batch]]` with axes and expanded by lab, which runs it in a pool,
one record per job, each job cut at its timeout, the versions of the tools in every record; without a
window first (§1, §3, §5, §6b, §14, §15, §17). The grid in the window, the reruns, the cached
dependencies and the reports are R2 to R4 (§15).

Amendment R2 of 2026-11-13, asked by Marc the same day ("finish the milestones"): the engine of a commit
and the source of a scene are dependencies, `[[dependency]]`, made once per key in `<lab>/cache/` and
shared by every batch that needs them (§3, §5, §6b, §15, §17).

Amendment R3 of 2026-11-13, asked the same day: the window queues a batch as one item and a sequence
may name batches; a Batches screen shows the grid, its cells selected and queued again; a job already
done is skipped unless asked again; an estimate per job (§6b, §11, §14, §15).

Amendment R4 of 2026-11-13, asked the same day: a batch's `report`, a short script that reads the
outputs of its jobs and makes the table, run again in seconds without computing anything (§6b, §14,
§15).

## 1. Purpose

Every long computation of the project (benches, test suites, Lean builds and oracle runs,
the papers' experiments) is launched from one desktop application, which keeps one record per
run: what ran, on which commit, on which machine, under which load, how long it took, and which
version of which paper it backs. Timings stop living in logs, PR bodies and memory notes.

The application does six things: it keeps the database of runs (§4), the catalogue of jobs
(§6), a queue that runs them one at a time (§8), a progress estimate from history (§10), a
monitor of the machine with a "calm" indicator (§9), and a history view with export (§11).

Out of scope: launching a job on another machine from here (each machine runs its own lab, and
their records meet in git, §4b), running two items of the queue at once (the jobs of one batch run
side by side in its pool, §6b), scheduling recurrent jobs, reading the results of a run (the scripts'
summaries stay what they are).

## 2. Name and place

- Folder `lab/` at the root of the repository, its own Cargo workspace (its own `Cargo.toml`,
  `Cargo.lock` and `target/`), not a member of the engine's workspace: engine builds and CI are
  unchanged.
- Package `openclare-lab`, binary `lab`. Installed on the machine by `make lab-install` at the root
  of the repository (§14); `cargo run --release` from `lab/` while developing it. The root's
  `make install` stays the Blender add-on's.
- lab owns a **mirror** of the repository, `~/.openclare-lab/repo.git`, fetched from GitHub
  (`marcgardent/openclare`, or the `remote` setting) before every queue and on request. It no longer
  needs the clone it was started from: the first start creates the mirror, from that clone when there is
  one (fast, offline, its LFS objects linked in), else from GitHub. The catalogue it shows is the
  `lab/jobs.toml` of `main` in the mirror; `--catalogue <file>` reads another, while one is being written.
- A job runs on a **reference**: a tag (`v0.7.0`), a branch (`main`, `worktree-borders`), a pull
  request (`pull/301`) or a hash. lab resolves it in the mirror and checks that commit out in
  `~/.openclare-lab/checkouts/<short hash>/`, a detached worktree of the mirror, one per commit,
  reused by every run on that commit. The catalogue the job is rendered from is the `lab/jobs.toml`
  of that commit; a commit older than lab takes the catalogue of `main`.
- The escape hatch for an experiment not committed: a job run in a folder given by hand (a **local**
  checkout). Its record is off-tag, dirty if the tree differs from HEAD, and orphan if its commit is
  not pushed (§10b).

## 3. Files

| File | Tracked | Written by | Content |
|---|---|---|---|
| `lab/jobs.toml` | yes | Marc by hand | the catalogue (§6), read at the commit a job runs on |
| `lab/runs.jsonl` | yes | the app, on Marc's request ("Copy history to the repo") | the published snapshot of the history |
| `<lab>/runs.jsonl` | no | the app | the live memory of this machine, one run per line |
| `<lab>/logs/<run id>.log` | no | the job (its stdout and stderr) | the log of each run |
| `<lab>/out/<run id>/` | no | the job, through `{out}` | the files of one job of a batch (§6b) |
| `<lab>/cache/<dependency>/<key>/` | no | a dependency's job | what a dependency made, shared by the batches that need it (§6b, R2) |
| `<lab>/refused.jsonl` | no | the app | the records that still name a local path (section 4) |
| `<lab>/state.json` | no | the app | the queue in progress and the run under way, for recovery |
| `<lab>/settings.toml` | no | the app | remote, machine name, push policy, thresholds, thread warning, policies (§12) |
| `<lab>/repo.git/` | no | git | the mirror of the repository (§2) |
| `<lab>/checkouts/<short hash>/` | no | git, the prepares, the jobs | one detached worktree per commit run (§2), with its `target/` and `.lake/` |
| `<lab>/machines/<machine>.jsonl` | no | the app | the memory of each other machine, as last fetched (§4b) |

`<lab>` is `~/.openclare-lab`, one folder for everything lab keeps on the machine; `--data <folder>`
moves it (the tests use `tests/fixtures`). The first start finds the `~/.local/share/openclare-lab` and
`~/.config/openclare-lab` of the earlier version and moves their files into `<lab>` once. The app shows
the path in its "About" box.

## 4. Storage

**Format: JSON Lines, one run per line, sorted by start time.** A few hundred rows fit in memory
and load in milliseconds; a line is readable by `jq`, by Python without a library, and in a diff.
SQLite through `rusqlite` would add indexes and transactions this volume does not need, and a
binary file git can neither diff nor merge. Editing a comment rewrites the file atomically
(written beside, then renamed): the git diff of the snapshot is that one line.

**The live file is outside the work tree; git holds a snapshot.** If the live database were
`lab/runs.jsonl` in a checkout, that checkout would carry a local change all the
time: `git pull` would refuse to run each time a PR changed the file, and the `dirty` flag of
every run made there would be true because of the database itself. So the app writes
`<lab>/runs.jsonl`, one file for the machine whatever checkout or worktree a job runs in, and
"Copy history to the repo" writes the union of the machines' memories (§4b), sorted, to `lab/runs.jsonl` of the checkout
Marc picks, to be committed with the PR that cites the runs. `lab/.gitattributes` declares
`runs.jsonl merge=union`: two branches that each add runs merge without conflict; the loader
then keeps one line per run id (the one with the latest `edited`). The tracked snapshot is also
the backup of the live file; the app keeps the last five versions of the live file
(`runs.jsonl.1` to `.5`) as well.

**No absolute path is written to a record.** Paths are relative to the root of the checkout the
job ran in, and that checkout is named by the reference asked for (`v0.7.0`, `main`, `pull/301`, a hash), or
`local` for a folder given by hand (§2). Before writing, the app replaces the root of the checkout by `{repo}`
and the home folder by `~` in the command, the environment and the paths, then refuses to write a
record that still matches the home-folder patterns of `hygiene.yml` (Linux, macOS and Windows)
or contains `/tmp/`, and shows the offending field; that record is kept in `<lab>/refused.jsonl`,
never copied into the repository. The environment recorded is the variables
the app sets (§7), not the whole inherited environment.

## 4b. One memory per machine, met in git

- Each machine has its own live memory, `<lab>/runs.jsonl`, and its own **branch** in the repository,
  `lab/runs-<machine>` (`<machine>` is the `machine` setting, the hostname by default). The branch
  holds one file, `runs.jsonl`, the copy of the live memory, and shares no history with `main` (an
  orphan branch, as `gh-pages` is). Only that machine writes to it: every push is a fast-forward, there
  is never a conflict nor a race between two machines.
- **Push**: after every run, by the `push` setting (`ask`, the default: a button in the top bar with the
  count of records not yet pushed; `auto`; `never`). What is pushed is the union of the branch and the
  live memory, so that a memory wiped on the machine loses nothing. The push needs a key with write
  access to the repository: Marc's on his machines, a deploy key limited to the repository on a machine
  that runs at night (Q18).
- **Read**: at every opening and after every fetch, lab reads the `lab/runs-*` branches of the mirror
  into `<lab>/machines/` and unions them with the live memory of this machine: one line per run id,
  the latest `edited` wins, the rule §4 already has. A comment or a paper edited on another machine
  therefore reaches this one at the next fetch; the same record edited on two machines keeps the later
  edit. A record of another machine edited here is added to this machine's memory, and reaches every
  machine as the latest.
- **Publication**: "Copy history to the repo" writes that union, sorted, to `lab/runs.jsonl` in a
  checkout Marc picks, to be committed with the PR that cites the runs; `main` still changes by PR
  only.
- The tags and the repository's summaries (§13) are read from the mirror (`git tag`,
  `git show main:<file>`, `git log -1 main -- <file>`; the remote's branches are the mirror's own), never
  from a work tree. The State screen therefore depends on no checkout: the same on every machine after
  the same fetch.

## 5. Data model

```rust
/// One run, one line of runs.jsonl.
struct Run {
    format: u32,                    // 1
    id: String,                     // "20261106-221403-forced-whole-lower", unique
    job: String,                    // catalogue name at the time of the run
    source: Source,                 // App | Seeded { from: String } | Manual
    queue: Option<String>,          // id of the queue it ran in
    started: DateTime<FixedOffset>, // RFC 3339 with offset
    ended: Option<DateTime<FixedOffset>>,
    seconds: Option<f64>,           // the job alone, the prepare excluded
    status: Status,
    prepare: Option<Prepare>,
    command: String,                // as executed, placeholders filled, {repo} kept
    environment: BTreeMap<String, String>, // the variables the app set
    parameters: BTreeMap<String, String>,  // the job's parameters as chosen
    checkout: String,               // the reference asked for, or "local" (§2)
    directory: String,              // working directory, relative to that checkout
    // The four below are None only in a seeded or typed run: it carries what is known.
    git: Option<Git>,
    threads: Option<Threads>,
    memory_limit_gb: Option<u32>,
    machine: Option<Machine>,
    load: Option<Load>,
    calm: Calm,
    comment: String,                // editable afterwards
    papers: Vec<PaperRef>,          // editable afterwards
    log: Option<FileRef>,           // the log, outside the repo
    outputs: Vec<FileRef>,          // files the run produced
    edited: Option<DateTime<FixedOffset>>, // last change of comment or papers
    // R1 (§6b); absent in the records written before it.
    cell: Option<Cell>,             // the batch and the cell this job is; None for a [[job]]
    timeout_seconds: Option<f64>,   // the limit the job ran under; None: no limit
    tools: BTreeMap<String, String>, // "blender" -> "5.3.0", "rustc" -> "1.95.0", "python" -> "3.14.4", "lean" -> "v4.35.0"
    // R2 (§6b); absent before it.
    dependencies: BTreeMap<String, String>, // a cell's: dependency -> key, {"engine": "3f2a…", "source": "9c01…"}
    cache: Option<String>,          // a dependency's own run: the key it made
}

enum Status { Ok, Failed { code: Option<i32>, signal: Option<i32> }, Stopped, Interrupted, TimedOut }
// Stopped: by Marc. Interrupted: the app ended during the run (§8.5). TimedOut: killed by lab at its
// timeout (§6b), `seconds` then the time until the kill.

struct Cell {
    launch: String,                    // "20261113-221403-pole-pairs-115-remesh": one launch of the batch
    values: BTreeMap<String, String>,  // axis -> value: {"budget": "3000", "model": "ogre", "ref": "12a6675"}
    pool: u32,                         // jobs of the batch running side by side (§6b)
}

struct Prepare { command: String, seconds: f64, status: Status, skipped_because: Option<String> }

struct Git { commit: String /* 40 hex */, dirty: bool, branch: Option<String>, pushed: bool }
// dirty: tracked files differ from HEAD, lab/runs.jsonl excluded; always false in a checkout of the mirror.
// branch: the reference asked for. pushed: the commit is reachable from a branch or a tag of the remote (§10b).

struct Threads { setting: u32, effective: u32 } // effective: what the job uses (§7)

struct Machine {
    hostname: String, cpu: String, cores: u32, threads: u32,
    memory_bytes: u64, gpu: Option<Gpu>, kernel: String,
    label: String, // from settings, e.g. "i7-7700K 4c/8t 2017, 62 GB"
}
struct Gpu { name: String, memory_bytes: u64 }

struct Sample {
    load: [f64; 3],                // load averages 1, 5, 15
    cpu_percent: f64,              // whole machine, 100 = all threads busy
    foreign_cpu_percent: f64,      // processes that are not the run's
    memory_used_bytes: u64,
    vram_used_bytes: Option<u64>, gpu_percent: Option<f64>,
}
struct Load {
    at_start: Sample, at_end: Option<Sample>,
    during: Option<During>,        // over the samples of the run
}
struct During { samples: u32, foreign_cpu_mean: f64, foreign_cpu_peak: f64,
                load1_max: f64, memory_used_peak: u64, vram_used_peak: Option<u64> }

struct Calm {
    before: bool,          // the machine was calm during the window before the start (§9)
    during: bool,          // foreign load stayed under the thresholds during the run
    forced: bool,          // Marc started it anyway (§7.2)
    timing_valid: bool,    // before && during && !forced
    reason: String,        // why not, in words: "foreign CPU 34 % at 23:12 (lake)"
}

enum Paper { A, B } // A: closing-loops, B: turns-of-a-t-mesh
struct PaperRef { paper: Paper, label: String } // "v0.7.0", "2026-11-06", "tab:forced"

struct FileRef { path: String, bytes: u64, sha256: Option<String> } // sha256 up to 1 GB
```

```rust
/// One job of the catalogue, a [[job]] table of jobs.toml.
struct Job {
    name: String,                  // unique, kebab-case
    description: String,
    category: Category,  // Build | TestSuite | QuickBench | Bench | LeanBuild | LeanOracle | Experiment
    command: String,               // shell line with placeholders
    prepare: Option<String>,       // run before the command, in the same checkout
    checkout: CheckoutChoice,      // Ask | Reference(String)
    directory: String,             // default "."
    parameters: BTreeMap<String, String>, // name -> default, editable per queue item
    environment: BTreeMap<String, String>,
    threads: ThreadUse,            // Setting | Fixed(u32)
    memory_limit_gb: Option<u32>,
    timing_matters: bool,
    outputs: Vec<String>,          // globs relative to the checkout
    paper: Option<Paper>,          // default paper of its runs
    tags: Vec<String>,
    timeout: Option<Timeout>,      // R1: none by default for a [[job]] (Q20)
}

/// A grid of jobs, a [[batch]] table of jobs.toml (§6b): every field of Job, and its axes.
struct Batch {
    job: Job,                       // the template, the axes' placeholders in it; its timeout 30 minutes when absent
    axes: Vec<Axis>,                // in the order written; the last varies fastest
}
struct Timeout { seconds: f64 }    // written "90s", "30m", "2h"
struct Axis { name: String, values: Vec<AxisValue> }
struct AxisValue { value: String, placeholders: BTreeMap<String, String> } // "cage", {"preset": "hard-surface"}

/// R2: made once per key and shared, a [[dependency]] table (§6b): the fields of Job but
/// prepare, summary and outputs, and:
struct Dependency {
    job: Job,
    key: Vec<String>,               // paths of the checkout, placeholders allowed: what the result depends on
    tools: Vec<String>,             // the tools whose versions are in the key: "rustc", "blender"
    provides: String,               // the file {<name>} names in the cache folder: "openclare", "source.obj"
}
// Batch gains `needs: Vec<String>`, the dependencies its cells use.

/// A named, ordered list of jobs, a [[sequence]] table: a NIGHT.md as it lists its stages.
struct Sequence { name: String, description: String, jobs: Vec<String> }
```

## 6. The catalogue

`lab/jobs.toml`, edited in the Catalogue screen or by hand; the app reloads it when the file
changes on disk. Example:

```toml
[[job]]
name = "forced-whole-lower"
description = "Table tab:forced: tau_P(H) with the cap, the window and whole lengths"
category = "experiment"
command = "python3 docs/research/turns-of-a-t-mesh/experiments/forced.py {export} --whole-lower --seconds 900 --budget 1800 --jobs {threads}"
checkout = "ask"
parameters = { export = "scratch/data-package/turns-of-a-t-mesh/male-body" }
threads = "setting"
memory_limit_gb = 16
timing_matters = true
outputs = ["docs/research/turns-of-a-t-mesh/experiments/results/forced.json"]
paper = "B"
tags = ["paper-B-data", "stage-4"]
```

Placeholders: `{threads}` (the effective thread count), `{repo}` (the root of the checkout the
job runs in), `{commit}` (its short hash), `{ref}` (the reference asked for), and the job's own
`{parameters}`. An unknown
placeholder is an error shown in the catalogue, not at run time. `checkout` is the reference the job
runs on, `main` when the field is absent, or `"ask"`: Marc picks it on the item of the queue.

The catalogue is seeded with the jobs the repository has today. Durations are what the
documents say, not measured by the app; "?" where nothing is written.

| Job | Category | Command (shortened) | Prepare | Threads | Timing matters | Written duration |
|---|---|---|---|---|---|---|
| `build-release` | build | `cargo build --release` | | setting | no | ? |
| `addon` | build | `make addon` | | setting | no | ? |
| `test-release` | test suite | `cargo test --release` (40 GB cap) | | setting | no | ~15 min (Marc) |
| `addon-test` | test suite | `make addon-test` | | setting | no | ? |
| `quick-bench` | quick bench | `python3 bench/quick.py --models {models} --target {target} --jobs {threads}`, `OPENCLARE_THREADS=1` | `cargo build --release && make addon` | setting (Q8) | no | a couple of minutes (quick.py) |
| `three-budgets` | quick bench | `python3 bench/budgets.py --engine '{engine}' ...`: the twelve models at 2970, 3000, 3030 quads, metrics and valences; `{engine}` a commit to build, or empty for the one built here; checkout asked | `cargo build --release` | setting, one engine thread per run | no | ? |
| `bench` | bench | `python3 bench/run.py --target {target} --tools {tools}` | `cargo build --release && make addon` | ? (Q8) | yes | an hour (quick.py docstring); about 3 h in all (Marc) |
| `remeasure` | bench | `make remeasure` | | setting | no | ? |
| `scores` | bench | `make scores` | | 1 | no | seconds |
| `lean-build` | Lean build | `make proofs NOTE=turns-of-a-t-mesh` | | ? (Q9) | no | ? |
| `lean-oracle-seed-1` | Lean oracle | `generate.py --count 50 --seed 1`, then `check_exports.py ... --jobs {threads}` | `make proofs NOTE=turns-of-a-t-mesh` | setting | yes | ~3 s per instance (property/README) |
| `lean-oracle-doubles` | Lean oracle | `generate.py --count 50 --seed 1 --borders`, then `check_exports.py ... --jobs {threads}`, on branch `worktree-borders` (PR #271) | `make proofs NOTE=turns-of-a-t-mesh` | setting | yes | ~18 h of one core in all (memory note) |
| `lean-check-export` | Lean oracle | `make lean-check EXPORT={export}/export JOBS={threads}` | | setting | yes | ~15 min, 3 jobs (NIGHT.md) |
| `export-male-body` | experiment (B) | `export.py --commit e13fadb --engine as-is --source scratch/torso/source.obj` (24 GB cap) | `cargo build --release` | fixed 8 | yes | ~2 min (NIGHT.md) |
| `census`, `lattice`, `conflicts`, `pairs`, `combs`, `fox`, `forced-bounds` | experiment (B) | one job per script of NIGHT.md stage 3 | | fixed 1 | yes | 15 min in all (NIGHT.md) |
| `forced-whole-lower` | experiment (B) | above | | setting | yes | 1 to 4 h (NIGHT.md) |
| `forced-sound`, `holonomy`, `strict`, `certify` | experiment (B) | NIGHT.md stage 4 | | setting, fixed 1, fixed 4 | yes | 0.5 to 1 h each (NIGHT.md) |
| `conjugacy`, `parity`, `gadget` | experiment (B) | experiments/README.md | | fixed 1 | yes | 626 s under load, ?, 352 s under load |
| `twelve-models` | experiment (A) | `twelve_models.py --jobs 1`, the engine on the setting | `cargo build --release` | setting | yes | 529 s from load 1.5 (results) |
| `ablation`, `male-body`, `figures`, `structure`, `against-remeshers`, `compare` | experiment (A) | closing-loops/experiments/README.md | `cargo build --release` where the engine runs | setting or 1 | yes, but `compare`, `against-remeshers` | 1406 s and 368 s under load (results) |
| `exact-quick`, `most-closable`, `rings`, `exact-remesh`, `end-to-end` | experiment (A) | closing-loops/experiments/exact/NIGHT.md | | fixed 1 to 4 | yes | minutes to 4 h (exact/NIGHT.md) |

The comparison against other remeshers is the `bench` job with every tool (`--tools` default);
`against-remeshers` only reads `bench/results/results.json`.

Seeded sequences: `paper-B-data` (NIGHT.md stages 1 to 4 in order), `paper-A-exact`
(exact/NIGHT.md 1 to 4), `before-merge` (`build-release`, `test-release`, `quick-bench`).

## 6b. Batches (amendment R1)

A **job** is one computation: one command, one process group, one time, one log, one record. A
**batch** is a grid of jobs: a command template and axes, which lab expands into one job per cell. The
words are those of the interface. A script that runs 150 computations inside one `[[job]]` is a black
box to lab: no progress, one time for everything, and the first computation that hangs loses the
others (the `pole-pairs-115` job, 2026-11-13). A batch is the same work with lab as its pool and
its stopwatch.

### The catalogue

```toml
[[batch]]
name = "pole-pairs-115-remesh"
description = "Ticket #115 before and after: the twelve models of bench/budgets.py at three budgets, on main and on 12a6675."
category = "quick-bench"
prepare = "cargo build --release && make addon && python3 -c 'import sys; sys.path.insert(0, \"bench\"); import run; [run.source_path(m) for m in sys.argv[1:]]' creature blob suzanne spot pig ogre bob jerry blub rock cage part"
command = "target/release/openclare remesh bench/out/{model}/source.obj {out}/mesh.obj --preset {preset} --quads {budget} && target/release/openclare measure bench/out/{model}/source.obj {out}/mesh.obj --target {budget} --json > {out}/measure.json"
parameters = { preset = "organic" }
threads = 1
memory_limit_gb = 32
timeout = "30m"
timing_matters = true

[batch.axes]
model = ["creature", "blob", "suzanne", "spot", "pig", "ogre", "bob", "jerry", "blub", "rock",
         { value = "cage", preset = "hard-surface" }, { value = "part", preset = "hard-surface" }]
budget = [2970, 3000, 3030]
ref = ["main", "12a6675"]
```

- A `[[batch]]` has every field of a `[[job]]` (§6), plus `axes` and `timeout`.
- **Axes.** Each axis is a placeholder of the command and the prepare, with the list of its values
  (strings or numbers). A value may be a table: its `value` is the cell's, its other keys
  override, for that cell only, the batch's `parameters` of the same name (above, `{preset}`;
  a key that is not a parameter is a problem of the catalogue). The grid
  is the product of the axes, expanded in the order they are written, the last varying fastest: the
  axis compared goes last, so that its values run side by side under the same load (above, main and
  12a6675 of one model and budget follow each other).
- **`ref`** is the one axis with a meaning: the reference each cell runs on, resolved and checked out
  as in §2, one checkout per commit. A batch without it runs on its `checkout`.
- **The prepare** may name the built-in placeholders and the parameters, not the other axes: it
  runs once per checkout, not per cell.
- **One catalogue for the whole grid.** Every cell is rendered from the catalogue the batch was taken
  from (`main` of the mirror, or `--catalogue`), not from the `jobs.toml` of each cell's commit: a
  comparison across commits runs the same command on each.
- **`{out}`**, a new built-in placeholder: `<lab>/out/<run id>/`, a folder made empty before the job
  starts. Every file in it goes into the record's `outputs`, with size and SHA-256. Like `{repo}`,
  the record keeps `{out}` unrendered in `command`, so that two runs of the same cell have the same
  command key (§10).
- An existing **`[[job]]` is a batch without axes**: one cell, run as today, its record without a
  `cell`. A computation whose steps depend on each other (one reads another's result, one random
  stream, one optimisation) stays one job; the jobs that were batches in a script are split, one job
  per unit, their summary rebuilt by the batch's report (R4). Done on 2026-11-13:

  | Batch | One job per | Its report |
  |---|---|---|
  | `lean-oracle-doubles`, `lean-oracle-seed-1` | instance (`index`); the generation, one random stream, is the prepare | — |
  | `lean-check-export` | way file (`index`) | — |
  | `lattice`, `holonomy`, `conjugacy`, `certify` | way | `experiments/merge.py` |
  | `forced-bounds`, `forced-whole-lower`, `forced-valid-bounds`, `forced-valid` | family (8 held, the strips of way 2): bounds, cap and size of a family read each other | `merge.py` |
  | `strict` | row of the table of conflicts | `merge.py` |
  | `twelve-models`, `ablation`, `end-to-end`, `end-to-end-time-5`, `end-to-end-time-300` | (line or variant ×) model × budget | `closing-loops/experiments/collect.py`, then the script reads the runs back |
  | `end-to-end-male-body`, `male-body`, `exact-remesh`, `forced-valid-remesh`, `most-closable` | variant, run or way | `collect.py` |
  | `three-budgets`, `pole-pairs-115-remesh`, `pole-pairs-115-tmesh` | model × budget × ref (or pairs) | `bench/before_after.py` |
  | `pole-pairs-115-male-body` | side (`ref`): the export, then conflicts on it | — |
  | `quick-bench`, `bench`, `remeasure` | model, model × tool, budget | `bench/run.py --merge` (bench) |

  `rings` split in two jobs: the rings with their rounding (the rounding reads the subset the rings
  chose), and `local-search`, which reads the export alone. Stay one job, by the rule or for being
  seconds: the builds and test suites, `lean-build`, `export-male-body`, `census`, `conflicts` (its
  table groups the ways), `pairs`, `combs`, `fox` (its relations pool every way), `forced-sound` (one
  family), `parity`, `gadget` (one random stream), `scores`, `figures`, `structure`,
  `against-remeshers`, `compare`, `exact-relaxation`. The batches of the papers read the export of
  the male body in `scratch/`, outside their key: `skip_done = false`.

### Dependencies (amendment R2)

A **dependency** is a computation many jobs use and whose result is the same for many of them: the
engine of a commit, the source of a scene. Declared once as `[[dependency]]`, it is made once per
**key**, kept in `<lab>/cache/<name>/<key>/`, and shared by every batch and every launch that needs
it. A second launch of the witness builds and exports nothing.

```toml
[[dependency]]
name = "engine"
description = "The engine's command line, release."
category = "build"
command = "cargo build --release && cp target/release/openclare {out}/"
key = ["crates", "Cargo.toml", "Cargo.lock"]
tools = ["rustc"]
provides = "openclare"

[[dependency]]
name = "source"
description = "A scene of the bench as the add-on sees it, exported by Blender."
category = "quick-bench"
command = "blender -b --factory-startup bench/scenes/{model}.blend --python-exit-code 1 --python bench/blender_run.py -- --out {out} --no-remesh"
key = ["bench/scenes/{model}.blend", "bench/blender_run.py", "bench/blender_engine.py", "blender/openclare"]
tools = ["blender"]
provides = "source.obj"
threads = 1

[[batch]]
name = "pole-pairs-115-remesh"
needs = ["engine", "source"]
command = "{engine} remesh {source} {out}/mesh.obj --preset {preset} --quads {budget} && {engine} measure {source} {out}/mesh.obj --target {budget} --json > {out}/measure.json"
# … the rest as above, without the prepare
```

- A `[[dependency]]` has the fields of a `[[job]]` but `prepare`, `summary` and `outputs` (its files
  are those of its `{out}`), plus `key`, `tools` and `provides`. Its threads are the setting unless
  it says otherwise; it has no timeout unless it sets one. It is not queued alone nor named in a
  sequence: a batch that needs it makes it.
- **Its placeholders**: the built-in ones, `{out}`, its parameters, and those of the cell that needs
  it (`{model}` above). A batch that needs a dependency naming a placeholder its cells lack is a
  problem of the catalogue.
- **The key**: the first 16 hex digits of a SHA-256 over the dependency's name, its command rendered
  (`{out}` and `{repo}` kept), for each path of `key` rendered, the git objects under it at the
  checkout's commit (`git ls-tree -r`; for a local folder, the text of `git diff HEAD` on those paths
  as well, new untracked files not seen), and the versions of the `tools` named (§6b, step 2). A
  path that names nothing at that commit fails the dependency. So two commits whose engine sources
  are the same share one engine, and a scene that did not change is not exported again, whatever
  the commit.
- **`needs`**, in a batch: the dependencies of its cells. `{engine}` in the command is the path of
  the file `provides` in the cache folder of that cell's engine. The record keeps `{engine}`
  unrendered, as `{out}`, so the command keys of two commits stay equal, and gives the keys in
  `dependencies`.
- **When**: after the tools are read and before the prepares (step 3 below). The dependencies of
  every cell are rendered and gathered by key; those already in the cache are said and skipped
  ("engine 3f2a… cached"); the others run in the order of `needs`, one pool per dependency
  (`setting / threads` side by side), each a job with its own record: `job` the dependency's name,
  `cell` its placeholders, `cache` its key, its log in `logs/`. Its `{out}` is `<lab>/out/<run id>/`;
  when it ends `Ok` the folder is moved to `<lab>/cache/<name>/<key>/` (a folder another launch put
  there first wins, ours is removed), and its outputs are recorded there. A failed dependency fails
  the cells that need it without running them, one record each, as a failed prepare does; the other
  cells go on.
- Nothing removes the cache on its own (an engine is about 20 MB, a source a few MB); the folder is
  removed by hand (Q23).

### The run of a batch

1. **Expand** the grid; an axis restricted by hand (`--axis model=ogre,pig`) keeps those values only.
   Resolve each `ref` (an unknown reference runs on `main`, as in §8.1).
2. **Tools**, read once per checkout: `blender --version`, `rustc --version` and `python3 --version`
   run in the checkout, their version number kept; Lean is the toolchain named by the checkout's
   `lean-toolchain` file, read and not run (elan would download a toolchain it lacks). A tool not
   found is left out of the map. Every record gets the map of its checkout, a `[[job]]`'s as well:
   until now nothing recorded that Blender is 5.3.0.
3. **Dependencies** (R2, above), then the **prepares**, one checkout after the other, before any job: the prepare of §7.3, once per checkout
   (`.lab-prepared`), with the whole thread setting (not the job's `threads`). Its time and status go
   into the `prepare` of every job of that checkout. A failed prepare fails the jobs of its checkout
   without running them, one record each; the other checkouts go on.
4. **The pool.** `threads` is what one job uses. The pool runs `max(1, setting / threads)` jobs side
   by side (by day, the setting 6 and one engine thread: six jobs), in the grid's order, each started
   as §8.1 starts a run: its own process group, memory cap and log `<lab>/logs/<run id>.log`. The
   comfort limit (§7.1) is checked on the pool's width times `threads`. A batch whose **timing
   matters runs one job at a time** (Marc, 2026-11-13): six jobs side by side on 4 cores and 8 threads
   slow each other, and the monitor counts them as ours, so their times would pass for calm.
5. **One record per job, written when it ends**: `job` is the batch's name, `parameters` the cell's
   placeholders, `checkout` its reference, `cell` the launch, the axis values and the pool's width.
   The run id is the start time, the batch's name and the cell's values (`20261113-221403-pole-pairs-
   115-remesh-ogre-3000-12a6675`), with `-2`, `-3` if it is taken. A batch that dies loses the jobs
   under way, never those already done.
6. **Timeout**, per job: `timeout` in the batch (`"90s"`, `"30m"`, `"2h"`), 30 minutes when absent;
   `--timeout` overrides it for one launch. Past it, lab ends the job as Stop does (§8.3, SIGTERM,
   then SIGKILL to the group): status `TimedOut`. A failed or timed-out job is a failed cell; the
   batch goes on. The policies of §8.4 apply between the items of a queue, not inside a batch.
7. **Calm**, per job, by §9, the foreign CPU being the processes outside the batch (the other jobs of
   the pool are ours); load1 is not checked while the batch runs, as after one of our runs. A batch
   whose timing matters starts its pool once the monitor has a full window (60 s), else its first
   jobs would have no calm verdict. The time
   of a job run in a pool of six is a time in a pool of six: `cell.pool` says so. A batch whose timing
   matters has a pool of one, its sides of a comparison still interleaved (Q21).

**Threads in the code, not in the pool** (Marc, 2026-11-13). A job is given the threads it may use
and its code uses them: lab sets `OPENCLARE_THREADS`, `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and
`CARGO_BUILD_JOBS` to them (§7.1), and every batch that runs the engine takes the thread setting
(`threads` left out), one job at a time when its timing matters. The scripts that call the engine pass
`OPENCLARE_THREADS` on (they dropped it until then, so the engine took the whole machine);
`end_to_end.py --cpus` caps it to the processors given. HiGHS follows the same setting
(`HIGHS_THREADS`, no longer forced to 1 in the catalogue): the papers' Python batches take the thread
setting too. Only the Lean checks, one instance a process, stay on one thread.

**Times in the papers** (Marc, 2026-11-13). A paper cites the time of each unit (a way, a family, a
model at a budget), from a job run alone (`cell.pool` 1) on a calm machine (`timing_valid`), with the
hardware and its threads (`threads.effective`); and for a stage, the sum of those times, which does not
depend on how many jobs ran at once. Never the wall time of a parallel pool. The merged summaries give both:
the units' own `seconds`, and `cells_seconds_sum` beside `wall_seconds` (the longest job).
8. **The end**: a line per status (`72 jobs: 71 ok, 1 timed out`), the failed cells named with their
   logs; exit code 0 when every job is `Ok`, else 1. Stop (Ctrl-C without a window) ends the jobs
   under way, recorded `Stopped`; the jobs not started get no record.

Without a window (§14), `lab run <batch>` prints one line per job that starts and ends (`[37/72] ok
41.2 s  model=ogre budget=3000 ref=12a6675`), not the logs, which are several at once. In R1 the
window lists the batches in the catalogue and their records in the history; R3 queues them (below).

### The grid, reruns and the queue (amendment R3)

- **A batch in the queue.** The Queue screen lists the batches beside the jobs, and a sequence may name
  batches as well as jobs. A batch is one item of the queue: at its turn the runner runs it as above
  (dependencies, prepares, pool), then goes to the next item. Its item reads `37/72` while it runs;
  the policies of §8.4 apply after it when one of its jobs did not end `Ok`. Stop ends the jobs under
  way (`Stopped`), as Ctrl-C does without a window; the jobs not started get no record. The lines of
  the launch, those `lab run` prints, go to `logs/<launch>.log`: the log the Queue screen shows for it.
- **The Batches screen**, a sixth tab: the batches on the left; for the one chosen, its grid. One line
  per job, numbered in the grid's order: each axis's value, its state, its last run (a cross table
  read as one job per row). A cell shows
  its state: running (elapsed and estimate), waiting in the queue (its estimate), its last record
  (`ok 41 s`, `failed`, `timed out`, the time muted when it does not count, §7.2), or `—`, never run.
  Hovering it gives its run, commit, calm and estimate; a click selects it, and the cell clicked last
  shows its record and the end of its log under the grid. The grid of a launch shows from its start;
  above it, what the launch does now (the checkouts, making a dependency, a prepare, the wait for the
  monitor, the jobs) and each dependency, cached, under way or made. The Queue screen's log follows
  the job that runs, a dependency or a prepare included, not the launch's lines. Above the grid, at
  rest as during a launch, the table of the jobs the batch needs (R2): one row per dependency and value
  it wants (`source creature`), one column per reference, each its last record and whether its result
  is in the cache; a click opens the record in History.
- **Selecting and running again.** Cells are selected one by one, or "failed", "never run", "all",
  "none"; a timeout for this launch (the batch's by default); "Queue the selection" puts the batch in
  the queue restricted to those cells. Without a window, `--axis` restricts the grid as in R1.
- **A job already done is skipped.** A cell whose last `Ok` record has the same command key (§10),
  commit, tools' versions and dependencies' keys is not run again: no new record, the line says
  `done before` with that record's id, and the grid keeps showing it. A record whose time does not
  count is done all the same: its result is there. "Again" (`--again` without a window) runs them
  anyway. A batch whose jobs read files that their command, commit and dependencies do not name (the
  export of the male body in `scratch/`) says `skip_done = false`: its jobs always run.
- **An estimate per job**: the median time of the `Ok` records of the same batch with the same
  command key and commit, else the same command key, else the same values of the axes. `lab run`
  gives it on the line of a job that starts (`≈ 41 s`), and at the start the launch's total divided by
  the pool's width; the grid gives each cell's and the time left of the batch under way.
- **Data**: a queue item gains `batch: Option<BatchChoice>`, `BatchChoice { cells: Vec<BTreeMap<String,
  String>> /* empty: all */, timeout: Option<Timeout>, again: bool }`; it runs the batch from the
  catalogue the window shows.

### Reports (amendment R4)

- A batch may have a **`report`**: a command, run in a checkout like a job, that reads the records
  of its cells and their files, and writes a table. It computes nothing again: run again, it gives
  the same table in seconds. Placeholders: the built-in ones, the batch's parameters, `{records}` and
  `{out}`; not the axes.

  ```toml
  report = "python3 bench/before_after.py {records} --side ref --out {out}"
  ```

- **`lab report <batch>`** takes, for each cell of the grid, its last record (`--launch <id>`: of that
  launch only), and writes them to `<lab>/reports/<batch>/<time>/records.json`: the batch, its axes,
  and per cell its values, run id, status, seconds, whether the time counts, commit, reference, and the
  absolute paths of its `{out}` folder and of its outputs (`null` for a cell never run). Then it runs
  the report in that folder's checkout: the batch's `checkout`, or `--ref <reference or folder>`; its
  `{out}` is the same folder, its output in `report.log` there, and `lab report` prints it. A report
  that runs more than 10 minutes is ended. No record: a report is made again from the records at will.
  A launch of a batch that has a report ends with it, without a window as in the window (its table in
  the launch's log): a night of a sequence leaves its tables and its merged summaries ready.
- The Batches screen has a **Report** button beside the grid of a batch that has one; the table
  shows under the grid when it is done.
- One script serves the comparisons, `bench/before_after.py`: the axis compared (`--side`, `ref` by
  default), the file each job wrote (`--file`, `measure.json` by default), the keys compared, their
  mean or their sum over the other axes but one (`--by`, `budget` by default); the time is lab's, with
  how many of the times count.

## 7. Rules the app enforces

### 7.1 Threads

- One global **thread setting**, shown in the top bar. Each job declares how it uses threads:
  `setting` (it gets the setting) or `Fixed(n)` (it uses n whatever the setting, as the
  export, which takes the whole machine). The **effective** count is what the rules check.
- The app sets, for every job, `OPENCLARE_THREADS`, `RAYON_NUM_THREADS`, `RUST_TEST_THREADS`,
  `CARGO_BUILD_JOBS` and `HIGHS_THREADS` (since 2026-11-13: the papers' scripts give it to HiGHS) to
  the effective count, and fills `{threads}` (for `-P`, `--jobs`, `JOBS=`). A job's own
  `environment` overrides them.
- **Comfort limit.** Above 6 threads (tunable, §12) the queue item shows an amber warning: the
  machine is slow to use meanwhile. It never blocks: Marc decides when he launches. The app
  cannot see the threads of other sessions on the machine: the monitor's foreign load (§9)
  shows them.
- The queue runs one job at a time, so a bench comparison and the test suite never overlap
  when both are launched from the app.

### 7.2 Timing that matters

A job with `timing_matters = true` is the kind whose duration goes into a paper. What makes its
time honest is an idle machine, not the hour: Marc launches when he knows he is not using it, by
day as well as by night. Proposal (Q2):

- It **starts at once**, like every job (decision of 2026-11-09: lab warns, it never keeps a Run
  from starting). A run begun on a machine not calm has `before = false`, so `timing_valid = false`,
  its reason recorded ("not calm at the start: foreign CPU 18 %"), and the message band says so.
- With `wait = true` under `[calm]` in the settings, it waits at the head of the queue for the machine
  to be calm (§9): the item shows "waiting for calm since 15:04 (foreign CPU 18 %)", it starts once
  the machine has been calm for the whole window, and the other jobs of the queue wait for it only
  when they come after it. "Start anyway" starts it at once: `forced = true`, so `timing_valid =
  false`, the run counts, its time does not.
- Every run, whatever the job, gets its `Calm` verdict; a timing is reported only from a run with
  `timing_valid = true`.
- The screens say which it is. A stopwatch ⏱ stands beside the name of a job whose timing matters,
  wherever the job is shown (State, Queue, History, Catalogue, the run under way, the wait for calm);
  a job without it runs under any load and its duration is an order of magnitude, not a measurement.
  For a job with the stopwatch, the duration of a run is plain only when `timing_valid`: muted when its
  calm was not measured (a run read from the repository or typed by hand), struck through when the
  machine was not calm or the run was forced, the reason on hover.

### 7.3 Prepare before the job

- The prepare command runs before the job, in the same checkout, with the same thread setting;
  its time and status are kept in `prepare`, apart from the job's `seconds`.
- The same prepare on the same checkout runs once for all queues: it writes its command in
  `.lab-prepared` at the root of the checkout, and is skipped while that file names it
  (`skipped_because = "done for forced-bounds on 2026-11-06"`). A local checkout, which may change
  between two queues, runs it once per queue.
- A failed prepare fails the job; the job's command does not run.
- So `cargo build --release` always comes before a bench: `cargo test --release` does not rebuild
  `target/release/openclare`, and the bench jobs carry the build in their prepare.

### 7.4 Memory cap

`memory_limit_gb` sets `RLIMIT_AS` (soft and hard) in the child before `exec`, as `ulimit -v`
does. Seeded values follow the project's rules: 32 GB for the engine, Blender and Lean, 40 GB for
`cargo test`, the values of NIGHT.md where it gives one.

## 8. The runner

### 8.1 Starting a run

The app fetches the mirror, resolves the reference, adds the checkout of that commit if none exists
(`git worktree add --detach <lab>/checkouts/<short hash> <hash>`), writes `state.json` (the queue, the
item started), takes the start sample (§9), reads git (`rev-parse HEAD`, `status --porcelain
--untracked-files=no`, the reference as `branch`, `pushed` by `branch -r --contains` and
`tag --contains`), then spawns `bash -c "<command>"` in that checkout:

- in a new process group (`setpgid(0, 0)` in `pre_exec`), with the memory cap;
- stdout and stderr both redirected to `<lab>/logs/<run id>.log`, opened by the app and handed to
  the child: the log keeps being written if the app dies;
- stdin from `/dev/null`;
- the environment inherited, plus the variables of §7.1 and the job's own.

A reference the mirror does not know (a branch deleted after its merge, a typo) does not stop the item:
it runs on `main`, the message band and the log say which reference was asked and which commit ran, the
record names `main`, and the queue item showed "unknown, main instead" beforehand. lab warns, it never
keeps a run from starting (2026-11-09).

The log is tailed in the app: the last 2,000 lines, refreshed every half second, ANSI colours
stripped, search in the log.

### 8.2 The end of a run

When the shell exits: end sample, status from the wait status (code or signal), outputs (the
job's globs, plus the tracked files `git status` shows changed between start and end, each with
size and SHA-256), the record appended to `runs.jsonl`, the next item started. Processes of the
group still alive after the shell exits are listed in the log and killed (§8.3).

### 8.3 Stop

The Stop button sends SIGTERM to the process group, then SIGKILL to the group after 10 seconds
(tunable). The record gets `Stopped`. What happens to the rest of the queue is a setting (Q3):
**pause and ask** (default proposed: a dialog "Continue the queue / Cancel the rest"), or cancel the
rest.

### 8.4 Failure

A failed job (non-zero exit or signal not sent by the app): same setting as Stop, separately
(Q3): pause and ask, continue, or cancel the rest.

### 8.5 Closing the window, crash

- Closing the window during a run asks: "Stop the run and quit" or "Keep the window open"
  (default). There is no detached mode: the record needs the app alive (Q4).
- If the app dies (crash, logout, reboot), `state.json` says which run was under way. At the next
  start the app looks for the process group: if it is gone, the run is recorded `Interrupted`,
  its end the last modification of the log; if it is still alive, the app offers to wait for it
  (end time known, exit code not: `Interrupted` too) or to kill it. The rest of the queue is shown
  paused.

## 9. Monitoring and the calm indicator

A sampler thread reads, every 2 seconds (tunable, 1 to 5): CPU % in total and per thread, load
averages 1/5/15 and RAM from /proc (Linux only; `sysinfo` would add nothing here); VRAM, GPU % through
`nvidia-smi --query-gpu=memory.used,memory.total,utilization.gpu --format=csv,noheader,nounits`
(every 4 seconds; "n/a" when the command is missing or fails); and the CPU of each process. The
processes of the run are those of its process group and their descendants (by parent pid, which
also catches children that left the group); the **foreign CPU** is the rest. The last 10 minutes
are kept in memory for the sparklines and the window of the indicator.

The indicator, always visible in the top bar:

| State | When | Text |
|---|---|---|
| green **machine calme** | no run of ours, and over the last 60 s: foreign CPU mean < 5 %, every sample < 25 %, load1 < 1.0 | "calme depuis 14 min" |
| amber **occupée par notre run** | a run of ours, and over the last 60 s foreign CPU mean < 5 %, every sample < 25 % | "forced-whole-lower, 4 threads" |
| red **chargée par autre chose** | the foreign thresholds are crossed, run or not; or no run and load1 ≥ 1.0 | "lake 212 %, cargo 95 %" (the three heaviest foreign processes) |

For 60 seconds after the app starts, the indicator is grey ("measuring"), the window not being full.

Load average decays over minutes and includes our own run: right after one of our runs ends, load1
stays high for a while though nothing else runs. So load1 is checked only when no run of ours ended
in the last 5 minutes (tunable); in that case the foreign CPU alone decides. CPU % are percentages of
the whole machine (8 threads: 12.5 % is one thread busy).

Thresholds, window and sample period are in `settings.toml` and editable in the Machine screen; the
values above are a first proposal to tune on real runs (Q5).

## 10. Progress estimation

The **command key** of a run is its rendered command, its job's environment and its directory
(the checkout excluded). The estimate of a job is the median of `seconds` over its runs with
`status = Ok`:

1. with the same command key and the same commit;
2. else with the same command key at any commit;
3. else over all the job's runs, those read from the repository included (section 13);
4. else "no estimate".

An estimate always says what it stands on: "≈ 2 h 38 min 24 · 1 run, load 0.43". Runs under load are
used, and say so: they are often all there is (Marc, 2026-11-06).

The queue shows, for each item, its estimate, the **running total** from the start of the queue and the
time it should end. An item without an estimate makes the total read "at least …, plus n jobs without
an estimate". During a run: elapsed / estimate and a progress bar; past the estimate, "over the
estimate by 12 min". A finished run is a new point for the next estimate.

## 10b. Versions: on a tag or off-tag

- A run is classed by **one hash**: the one it records. That is the commit of the checkout for a run
  launched by lab, the commit written in the summary for a result of the repository; for a result that
  records none, the commit that brought it into git (`git log -1 -- <file>`).
- That hash is a tag's (`git tag -l 'v*'` in the mirror): the run is **on that tag** and can back a
  paper. Otherwise it is **off-tag**, shown with its hash, and cannot back a paper.
- That hash is reachable from no branch nor tag of the remote (`pushed = false`): the run is **orphan**,
  shown "orphan <hash>", and cannot back a paper either: nobody can check out what ran. A run of a
  local checkout is orphan until its commit is pushed.
- A run whose duration was not measured shows "no time".

## 11. Screens

One window, a top bar and five tabs. Labels in English, as the code; French labels are possible with a
switch (Q11). Layout, from the mockup:

### Top bar

The mirror (time of the last fetch, a Fetch button), the records not yet pushed (a Push button, §4b),
the calm indicator, CPU and load1 at a glance,
the thread setting (a spinner from 1 to 8, amber above the comfort limit).

### State (the screen the app opens on)

- One line per job, grouped by paper, the stopwatch ⏱ beside a job whose timing matters, with its
  **Last run**: marked with its tag ("v0.7.0") or "off-tag <hash>", then its date, duration ("no time"
  if not measured; muted or struck when the job's timing matters and this time is not valid, §7.2),
  load at start and the file it was read from; or "never run".
- Counts at the top: last run on a tag, off-tag, orphan, never run.
- The runs shown are the union of the machines' memories (§4b) and the repository's summaries (§13),
  read from the mirror: the screen depends on no checkout. A **Machine** filter keeps one machine's
  runs.
- A **Tag** filter: "All runs" (the last run of each job, wherever it ran), or one tag: only the last run
  of each job on that tag, the others "no run on this tag".
- Jobs checked here, or all those never run, go to the queue in one click.

### Queue

- **Left, the catalogue**: jobs grouped by category, a search box, for each a checkbox, its estimate,
  the stopwatch ⏱ if its timing matters; the sequences above them (checking one adds its jobs in order).
- **Middle, the queue**: the items in order, reordered with a **drag handle**, each with its parameters
  (editable), reference (a tag, a branch, a pull request, a hash, or a local folder; whether the run
  will be on a tag, off-tag or orphan; whether that commit has a prepared checkout already), effective
  threads, estimate, running total and end time (section 10); below: the comment and the
  paper version (A or B, a label with suggestions from past labels) applied to every run of the queue,
  the policies on stop and on failure, the queue's ETA, Run.
- **Right, the run under way**: job, item n of m, elapsed / estimate with the bar, ETA of the job and
  of the queue, Stop, the log tail.

### History

- A table of every run: date, job (with the stopwatch ⏱ when its timing matters), duration (muted or
  struck when that time is not valid, §7.2), commit (short, dirty mark), threads, calm (✓, ✗ with the
  reason as a tooltip, "forced"), status, comment, papers. Sorted by any column; filtered by job,
  commit, paper and label, dates, status, "timing-valid only".
- A detail pane for the selected run: full command, environment, parameters, checkout and directory,
  machine, load at start, during and at end, outputs with hashes, the log (opened in the pane, or in
  the system's editor); the comment and the papers editable there.
- "Export selection": Markdown table or CSV, to the clipboard or a file. The Markdown carries the
  hardware line the papers need, built from the runs: "Intel Core i7-7700K (4 cores, 8 threads,
  2017, 62 GB), idle (load1 0.12 at start, foreign CPU under 5 %)", and says when the selection
  mixes machines, loads or commits.
- "Record a past run": a form to type a run that the app did not launch (a time from a PR body):
  `source = Manual`, the fields Marc knows, the rest empty.
- "Copy history to the repo" (§4).

### Catalogue

The list of jobs and sequences, a form to edit the one selected (every field of `Job`), the command
rendered with the current setting as a preview, "Duplicate", "New", "Delete" (the history keeps the
runs of a deleted job). Saving rewrites `jobs.toml` keeping the order of the jobs; comments in the
file are lost when it is saved from the app (Q10).

### Machine

CPU total and per thread (bars and a 10-minute sparkline), load 1/5/15, RAM used/total, VRAM and GPU
%, the heaviest processes with "ours" or "foreign", the indicator with the reason, the thresholds
editable; the checkouts of the mirror with their size, and a Clean button that removes those off any
tag with their `target/` and `.lake/` (nothing is removed on its own, Q16).

## 12. Settings

`<lab>/settings.toml`: the remote (`marcgardent/openclare`), the machine name (the hostname by default:
the branch `lab/runs-<machine>` and the exports), the push policy (`ask`, `auto`, `never`), the comfort limit of threads (6, Q1), calm thresholds
and window and whether a timed job waits for calm (`wait`, off by default), sample period, delay between SIGTERM and SIGKILL, the policies on stop and on failure, the
machine label used in exports (default proposed "i7-7700K 4c/8t 2017, 62 GB", checked against what is
detected: the app warns if the CPU model or the memory differ).

## 13. The repository's results in the history

At every opening and after every fetch, lab reads the summaries `main` holds in the mirror
(`git show`, `git log -1 main -- <file>`, never a work tree) and shows each as a run "from the
repository", never copied into `runs.jsonl`. A job names its summary in `jobs.toml` (`summary`, a path, with
`#key` for a record nested in another file, as the export inside the papers' summaries). A summary gives:
the hash it records (else the commit that brought it, section 10b), the date of that commit (the time of day
is not known), `wall_seconds` or `seconds`, the processor and memory, the load averages at start and at end.
Its calm cannot be known (nothing else was sampled): `timing_valid = false`, reason "not measured (from the
repository)". `bench/results/*.json` and a few others record no duration: "no time".

## 14. Technical choices

- Rust 1.95 (the toolchain here), edition 2024.
- `eframe` / `egui` 0.36 with `egui_extras` (tables) and `egui_plot` (sparklines): immediate mode suits a
  dashboard redrawn every second, and it is mature on Linux (X11 and Wayland). The cargo cache here holds
  `egui` 0.36.2, `egui_extras` 0.36.2 and `egui_plot` 0.37.0, but not `eframe`: the first build downloads
  it and its dependencies from crates.io (Q12).
- CPU, RAM, load and processes read from /proc; `nix` 0.31 (process groups, signals, `setrlimit`),
  `serde`, `serde_json`, `toml`, `chrono`, `sha2`, `glob`, `dirs`. The GPU through the command
  `nvidia-smi`, no NVML binding.
- Threads: the UI; the sampler; the runner (spawn, wait, end of run); a file watcher on `jobs.toml` (a check
  of the modification time every 2 s, no `notify` crate). Shared state behind a `Mutex`, the UI redrawn every
  second and on every event.
- Pure modules without egui (records, catalogue, rendering of commands, estimation, calm verdict, export),
  with unit tests; the UI calls them.
- Own builds light: `CARGO_BUILD_JOBS=4` while developing.

**Install.** `make lab-install` at the root of the repository: `cargo install --path lab --root
~/.local` puts the binary at `~/.local/bin/lab`; `lab/openclare-lab.desktop` goes to
`~/.local/share/applications/` and `site/assets/icon.svg` to
`~/.local/share/icons/hicolor/scalable/apps/openclare-lab.svg`, so KDE lists "openclare lab" in its
menu; `update-desktop-database` runs when present. `make lab-uninstall` removes the three. Nothing is
installed outside the home folder.

**Headless.** `lab run <job> [--ref <reference>] [--force] [--threads <n>]` runs one job without a
window, with the same runner, monitor and records, its log on stdout, and exits with the job's status;
the reference defaults to the job's own, `--force` starts it although the machine is not calm when the
`wait` setting makes it wait (its time then not counting). `lab fetch` and `lab status` do what their names say; `lab push` fetches, then
pushes the live memory to this machine's branch (§4b), whatever the `push` setting says, and
`lab run … --push` does so after the run. A systemd timer or a shell on another machine can call them
(Q4 of §17 answered). The window stays the place where one watches.
R1: `lab run <batch> [--axis <name>=<value>,…]… [--timeout <duration>] [--threads <n>]` runs a batch
(§6b); `--ref` gives the reference of a batch without a `ref` axis; `--axis` and `--timeout` are for a
batch only, a job's own timeout being its `timeout` in `jobs.toml`.

**CI.** A workflow `lab.yml`, run only when `lab/**` or itself changes (the same `paths` filter and
`changes` job as `lean.yml`): `cargo build --release --locked` and `cargo test --locked` in `lab/`, on
`ubuntu-24.04` with the system libraries eframe needs. It costs a few minutes of GitHub's time, none of
the machine's, and keeps the app building when Rust or a dependency moves. `hygiene.yml` already covers
`lab/runs.jsonl`.

## 15. Milestones

Each is one or a few commits on the branch, then a stop for Marc's review.

| | Content | Checked by |
|---|---|---|
| M1 | the app opens, reads `jobs.toml` (seeded with §6) and `runs.jsonl`, shows the catalogue and the history table; `settings.toml` | opening it; unit tests of reading and rendering |
| M1b (optional) | `lab seed` from the results summaries | the seeded rows against the JSON files |
| M2 | queue, run and stop, log tail, prepare, memory cap, records written, `state.json` and recovery | a queue of `sleep 5`, `false`, a `sleep 60` stopped, the app killed during a run |
| M3 | sampler, Machine screen, calm indicator, `Calm` in the records, the rules of §7 | the indicator watched while a `yes > /dev/null` runs outside the app |
| M4 | estimates and ETAs | unit tests on made-up histories; a queue of `sleep` jobs run twice |
| M5 | History detail, editing, export Markdown and CSV, record a past run, copy to the repo, `lab/README.md` and `README.fr.md`, CI; the PR on Marc's word | an export pasted into a draft table |

Status on 2026-11-06: M1 to M5 done, with the State screen and the tag rule (sections 10b, 11, 13)
asked for after M2, and the repository's summaries read at every opening in place of M1b. Not done:
editing the catalogue in the app (edit `jobs.toml` by hand), French labels (Q11).

The amendment of 2026-11-08 adds three, each one PR, this document first:

| | Content | Checked by |
|---|---|---|
| M6 | `~/.openclare-lab` as the one folder, the files of the earlier folders moved once; `make lab-install` and `lab-uninstall`, the desktop entry and the icon | the entry in the KDE menu opens lab with the history still there |
| M7 | the mirror, one checkout per commit, prepares once per checkout, `{ref}`, `pushed`, orphan runs; the catalogue, the tags and the summaries read from the mirror, so that no clone is needed; the checkouts and the Clean button in the Machine screen; `lab run`, `fetch`, `status` without a window | a job queued on `v0.7.0` and on `main`; `lab run` from a shell, the window closed |
| M8 | the branches `lab/runs-<machine>`, push by policy, `lab push` and `lab run --push`, the union read; the Machine filter of the State screen | two memories (a second machine name with `--data`) pushed and read back; the screen the same from both folders |

Status on 2026-11-08: M6, M7 and M8 done; `lab push` went from M7 to M8, where the branches it pushes are.

The amendment R1 of 2026-11-13 starts the reform of jobs and batches (§6b), four milestones, each its
own review, the batch of ticket #115 the witness of each:

| | Content | Checked by |
|---|---|---|
| R1 | `[[batch]]`, the grid expanded into jobs, the pool, one record per job, a timeout per job, the tools' versions; without a window | `lab run pole-pairs-115-remesh`: 72 records, ogre at 3000 quads on 12a6675 timed out if it passes 30 minutes, the others done |
| R2 | dependencies as jobs, cached by key: the engine of a commit, the source of a scene, shared across batches | a second launch builds and exports nothing |
| R3 | the grid in the window (rows, columns, a cell's state, its log), selecting cells and running them again; a job already done (same command key, commit and tools) skipped unless forced; ETA per job | ogre alone run again with a longer timeout |
| R4 | reports: a short script reads the jobs' outputs and makes the table, run again in seconds | the before-and-after table of #115, nothing computed again |

Status on 2026-11-13: R1 coded (`src/batch.rs`, `[[batch]]` in `src/catalogue.rs`, `src/tools.rs`), unit
tests on made-up grids; checked by hand on 4 jobs of the witness, the full 72 left to Marc. R2 coded
(`[[dependency]]` in `src/catalogue.rs`, the cache in `src/batch.rs`), unit tests on a made-up
dependency; the witness restricted to blob at 3000 quads on main and 12a6675: the first launch built two
engines and exported one source (the scene is the same on both commits), the second built and exported
nothing. R3 coded (`src/app/batches.rs`, the batch item of `src/runner.rs`, the skip and the estimates in
`src/batch.rs`); a unit test runs a batch item of the queue on chosen cells, then again (skipped); the
Batches screen looked at on the witness's records and on a made-up batch under way. The witness of R3,
ogre run again alone with a longer timeout from the grid, is left to Marc. R4 coded (`src/report.rs`,
`lab report`, the Report button, `bench/before_after.py`); a unit test makes a report twice from a
made-up batch, the same table and no record; the witness's report made from the records of the R1 and
R2 checks (blob and ogre at 3000 quads). The full table of #115 waits for Marc's run of the 72 jobs.

## 16. What cannot be verified here

- **VRAM**: `nvidia-smi` answers on this machine (GeForce RTX 5070, 12,227 MiB, read on 2026-11-06), so the
  query of §9 works; the GPU % of a process is not read, so GPU load is not split into ours and foreign.
- **Threads of a job**: the app sets the variables; whether each script honours them is not checked (Q8, Q9).
- **Cost of the app itself**: measured on 2026-11-06 at 15 % of one thread (about 2 % of the machine),
  window open on the Machine tab, machine loaded by other work; it counts as foreign CPU.
- **Calm thresholds**: a first proposal, to tune on real runs.
- **Wayland or X11**: not checked which session Marc runs; eframe supports both.

## 17. Open questions for Marc

1. **Comfort limit**: an amber warning above 6 threads, never blocking: enough?
2. **Timing that matters, machine not calm**: wait for calm at the head of the queue (proposed), or
   only warn and start? Answered on 2026-11-09: warn and start, the time not counting; the wait is
   the `wait` calm setting, off by default (§7.2).
3. **Rest of the queue after Stop, after a failure**: pause and ask (proposed for both), cancel, continue?
4. **Closing the window during a run**: ask (proposed) and no detached mode, or a headless runner `lab run`
   that survives the window (a later milestone)?
5. **Calm thresholds**: foreign CPU mean < 5 % and peak < 25 % over 60 s, load1 < 1.0: right orders of
   magnitude?
6. **Sleep**: does the machine suspend itself when nobody touches it? If so the app must hold it
   awake (`systemd-inhibit`) during a queue of several hours.
7. **Estimates from loaded runs**: shown but kept out of the ETA (proposed), or used with a mark?
8. **Threads of the bench**: how many threads does `bench/run.py` use (Blender, the engine, the other
   remeshers)? Is `OPENCLARE_THREADS` honoured inside Blender through the add-on?
9. **Threads of Lake**: `lake build` does not take `-j` as far as I know; is the Lean build to be capped,
   and how?
10. **jobs.toml**: edited mostly by hand (then the app keeps its comments by editing it with
    `toml_edit`), or mostly in the app (plain rewrite)?
11. **Language of the interface**: English as the code, or French?
12. **Downloading crates**: the first build fetches `eframe` and its dependencies from crates.io. Allowed?
13. **Live database outside the repo, snapshot in git** (§4): agreed, or the database itself tracked in the
    home checkout?
14. **Name**: `lab` for the folder and the binary, `openclare-lab` for the package?

Answered by the amendment of 2026-11-08: Q4 (`lab run` without a window, M7) and Q13 (the live memory
outside the repository, met in git, §4b). New:

15. **Push**: `ask` by default, a button with the count of records not yet pushed; or `auto` after every
    run?
16. **Checkouts kept**: the tags' always, the last five other commits, the rest removed with their
    `target/` and `.lake/` when a queue ends: right? A `.lake/packages` shared by a link between
    checkouts of the same toolchain, to spare the Mathlib cache?
17. **Machine name**: the hostname, or a name set by hand (`i7-7700K`)?
18. **Night machine**: a deploy key limited to the repository, with write access, for its pushes?
19. **Where everything lives**: the mirror and the checkouts in `~/.openclare-lab` beside the memory,
    several GB; or the memory alone there and the checkouts elsewhere (`--checkouts <folder>`)?

Answered on 2026-11-08: Q15 `ask`; Q16 everything kept, a Clean button in the Machine screen; Q17 the
hostname; Q18 no night machine, Marc's key alone; Q19 everything in `~/.openclare-lab`.

Decided on 2026-11-13 (the reform's proposal): remesh and measure are one job; a job is cut after
30 minutes by default, each batch may change it; "job" and "batch" in the interface; the reform in a
session and a branch of its own. New:

Q20 to Q22 answered on 2026-11-13: as proposed (no timeout for a `[[job]]` by default, the pool
filling the thread setting, Lean's version from `lean-toolchain`); Q21 revised the same day: a batch
whose timing matters runs one job at a time.

20. **Timeout of a `[[job]]`**: none by default (proposed), since many of today's jobs are whole
    batches in a script (`lean-oracle-doubles`, about 18 h; `forced-whole-lower`, 1 to 4 h) and 30
    minutes would kill them; or 30 minutes for every job, each long one setting its own?
21. **Pool and timing**: six one-thread jobs side by side on 4 cores and 8 threads slow each other
    down. Proposed: the pool fills the thread setting, `cell.pool` recorded, and a comparison is fair
    because its sides run interleaved in the same pool. Or a batch whose timing matters runs one job
    at a time, slower but each time alone? Answered again by Marc later on 2026-11-13: one job at a
    time for a batch whose timing matters; the others fill the pool.
22. **Lean's version**: the toolchain named by `lean-toolchain` (proposed, nothing run); or
    `lean --version` through elan, at the risk of a download?

New with R2:

23. **The cache**: kept until removed by hand (proposed; an engine is about 20 MB a commit), or
    cleaned like the checkouts, by a button of the Machine screen?
