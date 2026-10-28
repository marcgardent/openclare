#!/usr/bin/env python3
"""What an exact selection of the rings buys on the bench (paragraph "Closing
the most rings, end to end" of Section "An exact look at the closing
system"), on the engine of one commit: its slow quality mode (`--quality
slow`, HiGHS choosing which rings to close, three rounds, the lengths found
added to the candidates the engine chooses among by its own count of
spirals) against the engine without it and against other draws of its
per-arc preference, on the twelve models at the three budgets (36 runs per
variant) and on the male body. The variants:

- `draws-0`: the engine before the other draws (`OPENCLARE_EXTRA_DRAWS=0`);
- `draws-3`: three other draws added the same way (the control);
- `engine`: the engine as it is (twelve other draws);
- `slow-draws-0`: the slow mode with its default work limit, no other draw;
- `slow-time-5`: the slow mode stopped by a time limit of 5 seconds a solve
  (`--slow-work-limit 0 --slow-time-limit 5`), no other draw;
- `slow-time-300`: the same with 300 seconds, the time limit alone;
- `slow-work-W`: the work limit W (`--slow-work-limit W`), no other draw;
- `slow`: the slow mode as a user gets it (with the other draws).

Each run logs, with `OPENCLARE_SLOW_LOG`, every solve of HiGHS (with or
without a solution), every round (whether it returned the lengths it
started from) and every piece (whose lengths go first); the summary counts
them. A time limit makes the outcome depend on the machine: run those
variants alone on an idle machine, with the processors fixed (--cpus, which
the engine and HiGHS inherit; HiGHS's own threads are not set by the engine)
and stated.

    python3 docs/research/closing-loops/experiments/exact/end_to_end.py --openclare <binary> --commit <hash> \\
        [--variants draws-0,draws-3,slow-draws-0,slow-time-5] [--jobs 1] [--cpus 0-3] [--male-body]

Runs are kept under --out (scratch/data-package/closing-loops/end_to_end/
<variant>/) and read back when there. Hours: 36 runs a variant, seconds to
minutes each. Writes results/end_to_end.json.
"""

import argparse
import os
import re
import sys
import time
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "turns-of-a-t-mesh" / "experiments"))
import common  # noqa: E402

VARIANTS = {
    "draws-0": ([], {"OPENCLARE_EXTRA_DRAWS": "0"}),
    "draws-3": ([], {"OPENCLARE_EXTRA_DRAWS": "3"}),
    "engine": ([], {}),
    "slow-draws-0": (["--quality", "slow"], {"OPENCLARE_EXTRA_DRAWS": "0"}),
    "slow-time-5": (["--quality", "slow", "--slow-work-limit", "0", "--slow-time-limit", "5"],
                    {"OPENCLARE_EXTRA_DRAWS": "0"}),
    "slow-time-300": (["--quality", "slow", "--slow-work-limit", "0", "--slow-time-limit", "300"],
                      {"OPENCLARE_EXTRA_DRAWS": "0"}),
    "slow": (["--quality", "slow"], {}),
}
MALE = ("human_male", "organic", 10590)


def variant(name):
    if name in VARIANTS:
        return VARIANTS[name]
    work = re.fullmatch(r"slow-work-([0-9.]+)", name)
    if work:
        return ["--quality", "slow", "--slow-work-limit", work.group(1)], {"OPENCLARE_EXTRA_DRAWS": "0"}
    raise SystemExit(f"unknown variant {name}: {sorted(VARIANTS)} or slow-work-<W>")


def read_log(path):
    """The counts of a slow log: solves with and without a solution, rounds
    that returned the lengths they started from, pieces whose first lengths
    are the selector's."""
    count = Counter()
    if not path.exists():
        return dict(count)
    for line in path.read_text().splitlines():
        # Each line: the number of the process, a word, what it says.
        parts = line.split(" ", 2)
        kind = parts[1] if len(parts) > 1 else ""
        if kind == "solve":
            count["solves"] += 1
            count["solves without lengths"] += "none closed by the selector" in line
        elif kind == "highs":
            count["HiGHS solves"] += 1
            count["HiGHS solves without a solution"] += "solution false" in line
            # HiGHS model status: 7 optimal, 13 time limit, 14 iteration
            # limit (the others are listed by their number).
            status = re.search(r"status (\d+)", line)
            if status:
                count[f"HiGHS status {status.group(1)}"] += 1
        elif kind == "round":
            count["rounds"] += 1
            count["rounds returning the lengths they started from"] += line.rstrip().endswith("same as start true")
        elif kind == "piece":
            count["pieces"] += 1
            count["pieces whose first lengths are the selector's"] += "first selector" in line
        elif kind == "draws":
            count["pieces with other draws"] += 1
            count["pieces whose first lengths are a draw's"] += "first draw" in line
    return dict(count)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    common.options(parser, "end_to_end")
    parser.set_defaults(summary=Path(__file__).resolve().parent / "results" / "end_to_end.json",
                        out=common.DATA / "end_to_end")
    parser.add_argument("--variants", default="draws-0,draws-3,engine,slow-draws-0,slow-time-5,slow-time-300")
    parser.add_argument("--jobs", type=int, default=1, help="runs at a time (1 for the time-limited variants)")
    parser.add_argument("--cpus", help="processors the runs may use, as 0-3 or 0,1 (default: all)")
    parser.add_argument("--male-body", action="store_true", help="also the male body at 10,590 quads")
    arguments = parser.parse_args()
    common.use(arguments)
    if arguments.cpus:
        cpus = set()
        for part in arguments.cpus.split(","):
            low, _, high = part.partition("-")
            cpus.update(range(int(low), int(high or low) + 1))
        os.sched_setaffinity(0, cpus)
    started, clock = common.machine(), time.monotonic()
    started["processors_allowed"] = sorted(os.sched_getaffinity(0))
    out = {}
    for name in arguments.variants.split(","):
        flags, environment = variant(name)
        directory = arguments.out / name
        directory.mkdir(parents=True, exist_ok=True)
        jobs = []
        cases = [(model, preset, quads) for quads in common.BUDGETS for model, preset in common.TWELVE]
        if arguments.male_body:
            cases.append(MALE)
        for model, preset, quads in cases:
            run = f"{model}-{quads}"
            log = directory / f"{run}.slow.log"
            jobs.append((run, model, preset, quads, directory, tuple(flags),
                         {**environment, "OPENCLARE_SLOW_LOG": str(log.resolve())},
                         common.MEMORY_LARGE if model == MALE[0] else common.MEMORY))
        rows = common.batch(jobs, arguments.jobs)
        twelve = [row for row in rows if row["model"] != MALE[0]]
        logs = Counter()
        for job in jobs:
            logs.update(read_log(directory / f"{job[0]}.slow.log"))
        out[name] = {
            "flags": flags, "environment": environment,
            "runs": len(twelve), "valid": sum(1 for row in twelve if row["metrics"] and row["metrics"]["valid"]),
            "failures": [f"{row['model']} {row['quads']}: {row['failure']}" for row in rows if row["failure"]],
            "mean": common.means(twelve),
            "seconds_total": round(sum(row["seconds"] for row in twelve), 1),
            "slow_log": dict(logs),
            "per_run": [{"model": row["model"], "quads": row["quads"], "seconds": row["seconds"],
                         "loop_edges_spiral": row["metrics"]["loop_edges_spiral"] if row["metrics"] else None}
                        for row in rows],
            "male_body": next(({key: row["metrics"].get(key) for key in common.MEASURES}
                               for row in rows if row["model"] == MALE[0] and row["metrics"]), None),
        }
        mean = out[name]["mean"]
        print(f"{name}: spiral edges {100 * mean['loop_edges_spiral']:.2f}%, closed {100 * mean['loop_edges_closed']:.1f}%, "
              f"mean distance {1000 * mean['mean_distance']:.3f} per mille, {out[name]['valid']}/{len(twelve)} valid; "
              f"{dict(logs)}", flush=True)
    summary = {
        "experiment": "an exact selection of the rings, end to end (the slow quality mode)",
        **common.header(arguments, started, [round(x, 2) for x in os.getloadavg()], time.monotonic() - clock),
        "variants": out,
    }
    common.write_json(arguments.summary, summary)
    print(f"summary: {arguments.summary}")


if __name__ == "__main__":
    main()
