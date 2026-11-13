# lab

The record and the launcher of openclare's computations: every bench, test suite, Lean build and
experiment of the papers is launched from here, and each run is recorded with its hash, machine, load,
calm verdict and the paper it backs. The specification is `SPEC.md` (French: `SPEC.fr.md`).

```sh
make lab-install
```

from the root of the repository puts the binary in `~/.local/bin/lab` and an entry "openclare lab" in
the desktop menu (KDE, and whatever reads `~/.local/share/applications`); `make lab-uninstall` undoes
it. While developing lab, `cd lab && cargo run --release`.

lab needs no clone. It keeps a **mirror** of the repository in `~/.openclare-lab/repo.git`, created at
the first start (from the clone it is started in when there is one, its LFS objects linked in; else from
GitHub over SSH) and fetched before every queue and with the Fetch button. Each job runs on a
**reference**, a tag (`v0.7.0`), a branch (`main`), a pull request (`pull/301`) or a hash, resolved in
the mirror and checked out once in `~/.openclare-lab/checkouts/<hash>/`; a folder on the machine (`~/…`)
is the escape hatch for an experiment not committed, its runs recorded "local". The catalogue shown is
the `lab/jobs.toml` of `main`. Each machine keeps its own memory and pushes it to its branch
`lab/runs-<machine>` of the repository (the Push button of the top bar, or `push = "auto"` in the
settings); every machine reads the others' branches at each fetch, so the screens show every run,
wherever it ran. It runs on Linux (it reads /proc); `nvidia-smi`, if present, gives the GPU.

## The screens

- **State** opens first: the last run of each job, marked with its tag (`v0.7.0`) or `off-tag <hash>`,
  its date, its duration (`no time` when it was not measured) and the load at its start. A run is on a
  tag when the one hash it records is the tag's; else it is off-tag, or `orphan <hash>` when no branch
  nor tag of the remote reaches that commit; neither can back a paper. The Tag filter
  shows the last run of each job on one tag only; the Machine filter, the runs one machine
  recorded. Jobs checked, or all those never run, go to the
  queue in one click. A stopwatch ⏱ beside a job says its timing matters:
  its duration is shown plain only when the run's time is valid, muted when its calm was not
  measured, struck when the machine was not calm. A job without the stopwatch runs under any load; its
  duration is an order of magnitude.
- **Queue**: jobs and sequences picked from the catalogue, reordered with their handle, their parameters
  and reference edited (… lists the tags and the branches), each saying whether it will be on a tag,
  off-tag or orphan and whether its commit is checked out already; each shows its estimate (the median
  of the job's past runs, with how many and under which load), the running total and the time it should
  end. Run fetches the mirror and starts them one at a time, each in a process group of its own, the
  prepare once per checkout; Stop sends SIGTERM, then SIGKILL. A job whose timing matters (⏱) starts at
  once like the others, and its time does not count when the machine was not calm; with `wait = true`
  under `[calm]` in the settings it waits for a calm machine, and "Start anyway" starts it. A reference
  the mirror does not know runs on `main`, with a warning: lab warns, it never holds a Run back.
  With `job_cpus = "1-3,5-7"` under `[calm]` (or "Job processors" on the Machine screen), the jobs are
  pinned to those processors, one per slot of whole cores, and the calm is judged on them alone;
  `lab/pin-desktop.sh 0,4` keeps the desktop on core 0 (SPEC.md §7.5).
- **Batches**: the jobs of a batch, one line each, numbered (its axes' values, its state, its last run);
  each its state under way or its last record (`ok 41 s`, `failed`, `timed out`, `—`
  never run), its estimate on hover, its record and the end of its log once clicked. Cells are selected
  (one by one, or the failed ones, those never run, all) and the batch is queued restricted to them,
  with a timeout for that launch.
- **History**: every run lab recorded and every result the repository's summaries hold, sortable and
  filtered; the detail of a run; its comment and paper edited; the runs shown copied as Markdown (with
  the hardware line) or CSV; a past run typed by hand; the history of every machine copied into the
  repository.
- **Catalogue**: the jobs of `jobs.toml` and the commands as they would run. Edit `jobs.toml` by hand:
  the mirror's is read again at the next fetch, a file given with `--catalogue` within two seconds.
- **Machine**: CPU in all and per thread, load, memory, VRAM, the heaviest processes (ours or foreign),
  the calm indicator and its thresholds; the checkouts of the mirror, their size on request, and Clean,
  which removes those off any tag with what was built in them (nothing is removed on its own).

The calm indicator, in the top bar: green when, over the last 60 s, the foreign CPU stays under 5 % in
mean and 25 % in every sample and load1 under 1.0 (load1 is not looked at for 5 minutes after a run of
ours); amber when busy with our run alone; red when something else loads the machine.

## The files

| File | Tracked | What |
|---|---|---|
| `lab/jobs.toml` | yes | the catalogue: commands, prepares, threads, memory caps, the summary each job writes |
| `lab/runs.jsonl` | yes | a snapshot of the history, written by "Copy history to the repo" |
| `~/.openclare-lab/runs.jsonl` | no | the live history of this machine, one run per line |
| `~/.openclare-lab/logs/` | no | the log of each run |
| `~/.openclare-lab/state.json` | no | the queue under way, to recover a run left by an app that died |
| `~/.openclare-lab/settings.toml` | no | the remote, the machine name, the push policy, thread setting, comfort limit, calm thresholds, policies |
| `~/.openclare-lab/repo.git/` | no | the mirror of the repository |
| `~/.openclare-lab/machines/<machine>.jsonl` | no | the memory of each machine, as its branch `lab/runs-<machine>` had it at the last fetch |
| `~/.openclare-lab/checkouts/<hash>/` | no | one detached checkout per commit run, with its `target/`, its `.lake/` and `.lab-prepared`, the prepares done in it |

`~/.openclare-lab` is the one folder of everything lab keeps on the machine. The first start moves the
files of the earlier folders, `~/.local/share/openclare-lab` and `~/.config/openclare-lab`, into it once.

No record names a local path: the checkout is written `{repo}`, the home folder `~`; a record that still
names one goes to `refused.jsonl` instead.

## Options

`--data <folder>` uses another folder than `~/.openclare-lab` (`tests/fixtures` shows the history on
made-up runs); `--catalogue <file>` reads the catalogue from that file in place of the mirror's;
`--tab <state|queue|history|catalogue|machine>`; `--queue <job or sequence>` (repeatable);
`--start` presses Run; `--run <id>` selects a run; `--screenshot <file.ppm> [--after <seconds>]`
writes a picture of the window itself, never of the screen, then closes it.

## Without a window

```sh
lab run build-release --ref v0.7.0
```

runs one job with the same runner, monitor and records, its log on stdout, and exits with the job's
status; `--ref` defaults to the job's own reference, `--force` starts it although the machine is not calm
when the `wait` setting makes it wait (its time then not counting), `--threads <n>` sets the thread count, `--push` pushes the memory of this
machine to its branch after the run, whatever the setting. Ctrl-C stops the run and records it stopped.
`lab fetch` creates or fetches the mirror and reads the other machines' memories; `lab push` fetches,
then pushes this machine's memory; `lab status` says where the mirror, the checkouts, the machines, the
queue and the history stand.

## Batches

A `[[batch]]` of `jobs.toml` is a grid of jobs, one computation each (SPEC.md, §6b):

```sh
lab run pole-pairs-115-remesh
lab run pole-pairs-115-remesh --axis model=ogre --axis budget=3000 --timeout 1h
```

runs every cell of the grid, `setting / threads` jobs at a time (one at a time when its timing matters,
⏱: a time for a paper is that of a job alone), each with its own log, folder
(`{out}`), timeout (30 minutes unless the batch sets one) and calm verdict, each recorded as it ends; a
failed or timed-out job is a failed cell and the batch goes on. It prints one line per job that starts
and ends, then the counts, and exits 0 when every job is ok. `--axis` keeps some values of an axis (any
reference for `ref`), `--timeout` replaces each job's for this launch. The window lists the batches in
its catalogue but does not queue them yet.

A batch that `needs` a `[[dependency]]` (the engine of a commit, the source of a scene) has it made once
per key, before its jobs, and kept in `~/.openclare-lab/cache/<name>/<key>/`; the next launches, of
this batch or another, find it there. The key reads the dependency's command, the git objects of its
`key` paths and the versions of its `tools`, so two commits with the same engine sources share one
engine. Nothing empties the cache on its own: remove the folder by hand.

A job already done (same command, commit, tools and dependencies, ended ok) is skipped, said `done
before`; `--again` runs it anyway. The line of a job that starts gives its estimate, the median of its
cell's past times. In the window, a batch is one item of the queue (a sequence may name batches), its
lines in `logs/<launch>.log`, its cells in the Batches screen as they start and end.

```sh
lab report pole-pairs-115-remesh
```

writes the last record of each cell of the batch to `~/.openclare-lab/reports/<batch>/<time>/records.json`
and runs the batch's `report` on it, which reads the files the jobs left and prints the table: computed
again in seconds, nothing run again. `bench/before_after.py` is the report of the comparisons (#115:
main against 12a6675, means over the cells ok on both sides). The Batches screen has a Report button.

## Tests

```sh
cd lab && cargo test --release
```

They run real processes (`sleep`, `false`, a shell that ignores SIGTERM), read this repository's tags and
summaries, and read /proc. CI builds and tests lab when `lab/` changes (`.github/workflows/lab.yml`).
