"""What the experiment scripts of the closing-loops paper share: where the
clone, the engine and the sources are, running the engine under a memory cap,
measuring a mesh, and the record of the machine a run was made on.

Every script is run from anywhere. The engine is a parameter: `--openclare`
(default `target/release/openclare` of this clone, or `$OPENCLARE`) and
`--commit`, the commit it was built from, recorded with the numbers (default
the commit of this clone). Raw output goes under the directory given with
`--out` (default under `scratch/data-package/`, which is not committed), and
the summary into `results/` next to this file.
"""

import hashlib
import json
import os
import platform
import resource
import statistics
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
RESULTS = HERE / "results"
DATA = ROOT / "scratch" / "data-package" / "closing-loops"
OPENCLARE = Path(os.environ.get("OPENCLARE", ROOT / "target" / "release" / "openclare"))
# Sources of the bench, as it gives them to every remesher (bench/run.py).
SOURCES = Path(os.environ.get("SOURCES", ROOT / "bench" / "out"))

# The twelve development models and their presets (Appendix "Provenance").
TWELVE = [
    ("creature", "organic"),
    ("blob", "organic"),
    ("suzanne", "organic"),
    ("spot", "organic"),
    ("pig", "organic"),
    ("ogre", "organic"),
    ("bob", "organic"),
    ("jerry", "organic"),
    ("blub", "organic"),
    ("rock", "organic"),
    ("cage", "hard-surface"),
    ("part", "hard-surface"),
]
# The budgets of the protocol. The share of spiral edges is chaotic on one
# model, a change of one per cent of the budget moving it as much as a change
# of the method: each model is run at 21 budgets, 2,900 to 3,100 quads by 10,
# so that a mean is over the chaos and not three draws of it (#222).
BUDGETS = list(range(2900, 3101, 10))

# The measures averaged: those the paper quotes, then the independent
# measures of structure (DT-137), which the selection of the engine does not
# look at.
MEASURES = [
    "loop_edges_spiral",
    "loop_edges_closed",
    "loops_spiral",
    "mean_distance",
    "hausdorff",
    "angle_deviation_mean",
    "loop_edges_helix",
    "loops_helix_missed",
    "base_complex_patches",
    "irregular_vertices",
]

# Memory caps of the engine, in KB (scratch/AGENT-RULES.md).
MEMORY = 12_000_000
MEMORY_LARGE = 24_000_000


def source(model):
    return SOURCES / model / "source.obj"


def md5(path):
    digest = hashlib.md5()
    with open(path, "rb") as data:
        for block in iter(lambda: data.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def options(parser, group):
    """The options every script takes; sets the engine used."""
    parser.add_argument("--openclare", type=Path, default=OPENCLARE, help="the openclare binary")
    parser.add_argument("--commit", help="the commit the binary was built from (default: this clone's)")
    parser.add_argument("--sources", type=Path, default=SOURCES,
                        help="the bench's sources, <model>/source.obj (default bench/out)")
    parser.add_argument("--out", type=Path, default=DATA / group, help="where the raw output goes")
    parser.add_argument("--summary", type=Path, default=RESULTS / f"{group}.json", help="the summary written")


def listed(text):
    """The values of an option given as `a,b` (or as several words): the
    words, in order, empty ones left out."""
    return [word for part in (text if isinstance(text, list) else [text]) for word in part.split(",") if word]


def budgets_option(parser, default=BUDGETS):
    """`--budgets 2900,3000` (or `2900 3000`): the budgets run, by default those
    of the protocol."""
    parser.add_argument("--budgets", nargs="+", default=None,
                        help="only these budgets, as 2900,3000 (default: the 21 of the protocol)")
    parser.set_defaults(budgets_default=default)


def budgets(arguments):
    """The budgets of `budgets_option`."""
    if arguments.budgets is None:
        return list(arguments.budgets_default)
    return [int(word) for word in listed(arguments.budgets)]


def models(text, known=None):
    """The models of `--models a,b` among `known` (default the twelve), in
    the order of `known`: [(model, preset)]."""
    known = TWELVE if known is None else known
    if text is None:
        return list(known)
    asked = listed(text)
    unknown = [name for name in asked if name not in dict(known)]
    if unknown:
        raise SystemExit(f"unknown model {', '.join(unknown)}: {', '.join(name for name, _ in known)}")
    return [(name, preset) for name, preset in known if name in asked]


def use(arguments):
    """Takes the engine and the sources given on the command line."""
    global OPENCLARE, SOURCES
    OPENCLARE = arguments.openclare.resolve()
    SOURCES = arguments.sources.resolve()
    if not OPENCLARE.exists():
        raise SystemExit(f"no engine at {OPENCLARE}: cargo build --release, or --openclare <binary>")
    arguments.commit = arguments.commit or commit()
    arguments.out.mkdir(parents=True, exist_ok=True)


def commit():
    """The commit of the clone, with a mark if its tracked files differ."""
    head = subprocess.run(["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, capture_output=True, text=True)
    dirty = subprocess.run(["git", "status", "--porcelain", "--untracked-files=no"], cwd=ROOT,
                           capture_output=True, text=True)
    return head.stdout.strip() + ("+changes" if dirty.stdout.strip() else "")


def engine_version():
    """What tells the binary apart: the start of its SHA-256."""
    if VERSIONS.get(OPENCLARE):
        return VERSIONS[OPENCLARE]
    digest = hashlib.sha256()
    with open(OPENCLARE, "rb") as data:
        for block in iter(lambda: data.read(1 << 20), b""):
            digest.update(block)
    VERSIONS[OPENCLARE] = "sha256:" + digest.hexdigest()[:16]
    return VERSIONS[OPENCLARE]


VERSIONS = {}


def machine():
    """The hardware and its load when a run starts: times are only read with
    them (a shared machine gives orders of magnitude only)."""
    model = ""
    try:
        for line in open("/proc/cpuinfo"):
            if line.startswith("model name"):
                model = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    memory = ""
    try:
        for line in open("/proc/meminfo"):
            if line.startswith("MemTotal"):
                memory = f"{int(line.split()[1]) / 1024 / 1024:.1f} GiB"
                break
    except OSError:
        pass
    return {
        "processor": model or platform.processor(),
        "threads": os.cpu_count(),
        "memory": memory,
        "load_average": [round(load, 2) for load in os.getloadavg()],
    }


def capped(kilobytes):
    def limit():
        resource.setrlimit(resource.RLIMIT_AS, (kilobytes * 1024, kilobytes * 1024))

    return limit


def remesh(source_path, out, preset, quads, flags=(), environment=None, memory=MEMORY, timeout=1800):
    """`openclare remesh`, with the experiment options in `environment`.
    Returns (seconds, failure or None)."""
    command = [str(OPENCLARE), "remesh", str(source_path), str(out), "--preset", preset, "--quads", str(quads),
               *flags]
    # The experiment's own switches only: those inherited are dropped, but for
    # OPENCLARE_THREADS, the threads lab allows the job (the engine takes the
    # whole machine without it).
    env = {key: value for key, value in os.environ.items()
           if not key.startswith("OPENCLARE_") or key == "OPENCLARE_THREADS"}
    env.update(environment or {})
    clock = time.monotonic()
    try:
        done = subprocess.run(command, env=env, capture_output=True, text=True, timeout=timeout,
                              preexec_fn=capped(memory))
    except subprocess.TimeoutExpired:
        return time.monotonic() - clock, f"timeout after {timeout} s"
    seconds = time.monotonic() - clock
    if done.returncode != 0 or not Path(out).exists():
        said = (done.stderr.strip().splitlines() or ["no output"])[-1]
        return seconds, f"exit {done.returncode}: {said}"
    return seconds, None


def measure(source_path, mesh, target, memory=MEMORY):
    """`openclare measure --json`, as a dictionary."""
    done = subprocess.run([str(OPENCLARE), "measure", str(source_path), str(mesh), "--target", str(target), "--json"],
                          capture_output=True, text=True, preexec_fn=capped(memory))
    if done.returncode != 0:
        raise RuntimeError(f"measure {mesh}: {done.stderr.strip()}")
    return json.loads(done.stdout)


def means(rows, measures=MEASURES):
    """The mean of each measure over the runs that have it."""
    out = {}
    for key in measures:
        values = [row["metrics"][key] for row in rows if row.get("metrics") and row["metrics"].get(key) is not None]
        out[key] = statistics.mean(values) if values else None
    return out


def spread(rows, key="loop_edges_spiral", draws=4000, seed=1):
    """How a share is spread over the runs: its mean, its median, the share
    of runs where it is 0, the 95 % bootstrap interval of the mean, and the
    number of models where it is ever above 0. The bootstrap draws the
    models with replacement, then the runs of each drawn model: the runs of
    one model are not independent of each other."""
    import random

    by_model = {}
    for row in rows:
        if row.get("metrics") and row["metrics"].get(key) is not None:
            by_model.setdefault(row["model"], []).append(row["metrics"][key])
    values = [value for runs in by_model.values() for value in runs]
    if not values:
        return None
    rng, means = random.Random(seed), []
    for _ in range(draws):
        drawn = []
        for model in rng.choices(list(by_model), k=len(by_model)):
            drawn += rng.choices(by_model[model], k=len(by_model[model]))
        means.append(statistics.mean(drawn))
    means.sort()
    return {
        "runs": len(values),
        "mean": statistics.mean(values),
        "interval_95": [means[int(0.025 * draws)], means[int(0.975 * draws) - 1]],
        "median": statistics.median(values),
        "share_of_runs_at_zero": sum(value == 0 for value in values) / len(values),
        "models_above_zero": sum(any(value > 0 for value in runs) for runs in by_model.values()),
        "models": len(by_model),
    }


def paired(before, after, key="loop_edges_spiral"):
    """Run by run (same model, same budget), on how many runs `after` has a
    lower share than `before`, a higher one, the same; and the two-sided
    sign test on the runs where it changes."""
    from math import comb

    def shares(rows):
        return {(row["model"], row["quads"]): row["metrics"][key] for row in rows
                if row.get("metrics") and row["metrics"].get(key) is not None}

    first, second = shares(before), shares(after)
    changes = [second[run] - first[run] for run in first.keys() & second.keys()]
    lower, higher = sum(change < 0 for change in changes), sum(change > 0 for change in changes)
    changed, fewer = lower + higher, min(lower, higher)
    p = 1.0 if changed == 0 else min(1.0, 2 * sum(comb(changed, i) for i in range(fewer + 1)) / 2 ** changed)
    return {"runs": len(changes), "lower": lower, "higher": higher, "same": len(changes) - changed,
            "sign_test_p": p}


def write_json(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=1, sort_keys=False) + "\n")


def run_one(name, model, preset, quads, directory, flags=(), environment=None, memory=MEMORY):
    """One run and its measure, kept in `directory` (`<name>.obj`,
    `<name>.json`); a run already there is read back, not made again."""
    mesh, record = directory / f"{name}.obj", directory / f"{name}.json"
    if record.exists():
        return json.loads(record.read_text())
    load = round(os.getloadavg()[0], 2)
    seconds, failure = remesh(source(model), mesh, preset, quads, flags, environment, memory)
    row = {"model": model, "preset": preset, "quads": quads, "flags": list(flags), "environment": environment or {},
           "seconds": round(seconds, 2), "load_average": load, "failure": failure, "metrics": None,
           "build": engine_version()}
    if failure is None:
        row["metrics"] = measure(source(model), mesh, quads, memory)
    write_json(record, row)
    return row


def batch(jobs, workers):
    """Runs `jobs`, (name, model, preset, quads, directory, flags,
    environment) each, `workers` at a time; returns their rows in order."""
    from concurrent.futures import ThreadPoolExecutor

    with ThreadPoolExecutor(max_workers=workers) as pool:
        futures = [pool.submit(run_one, *job) for job in jobs]
        rows = []
        for job, future in zip(jobs, futures):
            row = future.result()
            print(f"{job[0]}: {'failed, ' + row['failure'] if row['failure'] else 'ok'} ({row['seconds']} s)",
                  flush=True)
            rows.append(row)
    return rows


def header(arguments, started, finished_load, seconds, rows=()):
    """What the summary says of how it was made. `seconds` is the wall time
    of this call of the script (short if its runs were read back); the sum of
    the times of the runs themselves is kept with them."""
    # The runs read back may have been made by another binary (a run of
    # lab's batch, merged): the builds they say made them.
    builds = sorted({row["build"] for row in rows if row.get("build")})
    if builds and builds != [engine_version()]:
        print(f"warning: runs made by {', '.join(builds)}, the summary's engine is {engine_version()}", flush=True)
    return {
        "engine": {"binary": "openclare", "commit": arguments.commit, "build": engine_version(),
                   "builds_of_the_runs": builds},
        "machine": {**started, "load_average_at_end": finished_load},
        "wall_seconds": round(seconds, 1),
        "run_seconds_total": round(sum(row.get("seconds") or 0 for row in rows), 1),
        "load_average_during_runs": [min(loads), max(loads)] if (loads := [row["load_average"] for row in rows
                                                                          if row.get("load_average") is not None])
        else None,
    }
