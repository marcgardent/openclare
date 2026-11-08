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
the `lab/jobs.toml` of `main`. It runs on Linux (it reads /proc); `nvidia-smi`, if present, gives the
GPU.

## The screens

- **State** opens first: the last run of each job, marked with its tag (`v0.7.0`) or `off-tag <hash>`,
  its date, its duration (`no time` when it was not measured) and the load at its start. A run is on a
  tag when the one hash it records is the tag's; else it is off-tag, or `orphan <hash>` when no branch
  nor tag of the remote reaches that commit; neither can back a paper. The Tag filter
  shows the last run of each job on one tag only. Jobs checked, or all those never run, go to the
  queue in one click. A stopwatch ⏱ beside a job says its timing matters: it waits for a calm machine,
  and its duration is shown plain only when the run's time is valid, muted when its calm was not
  measured, struck when the machine was not calm. A job without the stopwatch runs under any load; its
  duration is an order of magnitude.
- **Queue**: jobs and sequences picked from the catalogue, reordered with their handle, their parameters
  and reference edited (… lists the tags and the branches), each saying whether it will be on a tag,
  off-tag or orphan and whether its commit is checked out already; each shows its estimate (the median
  of the job's past runs, with how many and under which load), the running total and the time it should
  end. Run fetches the mirror and starts them one at a time, each in a process group of its own, the
  prepare once per checkout; Stop sends SIGTERM, then SIGKILL. A job whose timing matters (⏱) waits
  for a calm machine; "Start anyway" starts it, its time then not counted.
- **History**: every run lab recorded and every result the repository's summaries hold, sortable and
  filtered; the detail of a run; its comment and paper edited; the runs shown copied as Markdown (with
  the hardware line) or CSV; a past run typed by hand; the history copied into the repository.
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
| `~/.openclare-lab/settings.toml` | no | the remote, thread setting, comfort limit, calm thresholds, policies |
| `~/.openclare-lab/repo.git/` | no | the mirror of the repository |
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
(its time then not counting), `--threads <n>` sets the thread count. Ctrl-C stops the run and records
it stopped. `lab fetch` creates or fetches the mirror; `lab status` says where the mirror, the checkouts,
the queue and the history stand.

## Tests

```sh
cd lab && cargo test --release
```

They run real processes (`sleep`, `false`, a shell that ignores SIGTERM), read this repository's tags and
summaries, and read /proc. CI builds and tests lab when `lab/` changes (`.github/workflows/lab.yml`).
