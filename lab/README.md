# lab

The record and the launcher of openclare's computations: every bench, test suite, Lean build and
experiment of the papers is launched from here, and each run is recorded with its hash, machine, load,
calm verdict and the paper it backs. The specification is `SPEC.md` (French: `SPEC.fr.md`).

```sh
cd lab && cargo run --release
```

lab finds the repository it sits in. It runs on Linux (it reads /proc); `nvidia-smi`, if present, gives
the GPU.

## The screens

- **State** opens first: the last run of each job, marked with its tag (`v0.7.0`) or `off-tag <hash>`,
  its date, its duration (`no time` when it was not measured) and the load at its start. A run is on a
  tag when the one hash it records is the tag's; an off-tag run cannot back a paper. The Tag filter
  shows the last run of each job on one tag only. Jobs checked, or all those never run, go to the
  queue in one click.
- **Queue**: jobs and sequences picked from the catalogue, reordered with their handle, their parameters
  and checkout edited; each shows its estimate (the median of the job's past runs, with how many and
  under which load), the running total and the time it should end. Run starts them one at a time, each
  in a process group of its own; Stop sends SIGTERM, then SIGKILL. A job whose timing matters waits for
  a calm machine; "Start anyway" starts it, its time then not counted.
- **History**: every run lab recorded and every result the repository's summaries hold, sortable and
  filtered; the detail of a run; its comment and paper edited; the runs shown copied as Markdown (with
  the hardware line) or CSV; a past run typed by hand; the history copied into the repository.
- **Catalogue**: the jobs of `jobs.toml` and the commands as they would run. Edit `jobs.toml` by hand:
  lab reads it again within two seconds.
- **Machine**: CPU in all and per thread, load, memory, VRAM, the heaviest processes (ours or foreign),
  the calm indicator and its thresholds.

The calm indicator, in the top bar: green when, over the last 60 s, the foreign CPU stays under 5 % in
mean and 25 % in every sample and load1 under 1.0 (load1 is not looked at for 5 minutes after a run of
ours); amber when busy with our run alone; red when something else loads the machine.

## The files

| File | Tracked | What |
|---|---|---|
| `lab/jobs.toml` | yes | the catalogue: commands, prepares, threads, memory caps, the summary each job writes |
| `lab/runs.jsonl` | yes | a snapshot of the history, written by "Copy history to the repo" |
| `~/.local/share/openclare-lab/runs.jsonl` | no | the live history, one run per line |
| `~/.local/share/openclare-lab/logs/` | no | the log of each run |
| `~/.local/share/openclare-lab/state.json` | no | the queue under way, to recover a run left by an app that died |
| `~/.config/openclare-lab/settings.toml` | no | thread setting, comfort limit, calm thresholds, policies |

No record names a local path: the checkout is written `{repo}`, the home folder `~`; a record that still
names one goes to `refused.jsonl` instead.

## Options

`--data <folder>` reads and writes the runs elsewhere (`tests/fixtures` shows the history on made-up
runs); `--tab <state|queue|history|catalogue|machine>`; `--queue <job or sequence>` (repeatable);
`--start` presses Run; `--run <id>` selects a run; `--screenshot <file.ppm> [--after <seconds>]`
writes a picture of the window itself, never of the screen, then closes it.

## Tests

```sh
cd lab && cargo test --release
```

They run real processes (`sleep`, `false`, a shell that ignores SIGTERM), read this repository's tags and
summaries, and read /proc. CI builds and tests lab when `lab/` changes (`.github/workflows/lab.yml`).
