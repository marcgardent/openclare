#!/usr/bin/env python3
"""Table `tab:dev` as an ablation on one engine: each line is the same
binary with the experiment options of `crates/openclare/src/quantization/
experiment.rs` (ALGORITHM.md, M6, "Experiment options") turning stages of
the closing off, one more stage on per line. The twelve development models,
at the budgets given (default the three of the protocol), means over the runs.

    python3 docs/research/closing-loops/experiments/ablation.py \
        [--openclare target/release/openclare] [--commit <hash>] [--budgets 3000] [--jobs 2]

One stage is added per line. The paper's table has four lines, two of which
add two stages at once (leaning ways and aimed repair; longer turns and the
deviation cap): here they are split, and the stages the table leaves out
follow, one at a time, up to the whole engine, which is the engine with no
option set (checked: its meshes are compared to those of a
plain run, byte for byte, when --check is given).
"""

import argparse
import filecmp
import os
import time

import common

# Every stage the switches turn off, as the variables that do it.
OFF = {
    "OPENCLARE_NO_AIMED_REPAIR": "1",
    "OPENCLARE_NO_DEVIATION_CAP": "1",
    "OPENCLARE_NO_DEVIATION_FLOOR": "1",
    "OPENCLARE_NO_CUT_ROUNDS": "1",
    "OPENCLARE_NO_ACTIVE_SET": "1",
    "OPENCLARE_NO_FOLD_GUARD": "1",
}


def without(*names):
    return {key: value for key, value in OFF.items() if key not in names}


# (key, line, the row of the paper's table it stands for or None, flags,
# environment). Ways: `m + 2j + 4i`, m the starting shares, j the lean bound,
# i the length bound; 0,1 is the first closing (lean 0.05, turns up to 2 sides),
# 0,1,2,3 adds the leaning ways (lean 0.12), all eight add the longer turns.
LINES = [
    ("no_closing", "no closing", "no closing", ["--no-close-loops"], {}),
    ("first_closing", "closing, one way kept by spirals (ways 0,1)", "closing, one way kept by spirals", [],
     {**OFF, "OPENCLARE_CLOSE_WAYS": "0,1"}),
    ("leaning", "+ leaning ways (0,1,2,3)", None, [], {**OFF, "OPENCLARE_CLOSE_WAYS": "0,1,2,3"}),
    ("leaning_aimed", "+ aimed repair", "+ leaning ways, aimed repair", [],
     {**without("OPENCLARE_NO_AIMED_REPAIR"), "OPENCLARE_CLOSE_WAYS": "0,1,2,3"}),
    ("longer", "+ longer turns (all eight ways)", None, [], without("OPENCLARE_NO_AIMED_REPAIR")),
    ("longer_cap", "+ deviation cap, without its floor", "+ longer turns, deviation cap", [],
     without("OPENCLARE_NO_AIMED_REPAIR", "OPENCLARE_NO_DEVIATION_CAP")),
    ("cut_rounds", "+ constraint rounds", None, [],
     without("OPENCLARE_NO_AIMED_REPAIR", "OPENCLARE_NO_DEVIATION_CAP", "OPENCLARE_NO_CUT_ROUNDS")),
    ("active_set", "+ active set", None, [],
     without("OPENCLARE_NO_AIMED_REPAIR", "OPENCLARE_NO_DEVIATION_CAP", "OPENCLARE_NO_CUT_ROUNDS",
             "OPENCLARE_NO_ACTIVE_SET")),
    ("deviation_floor", "+ floor of the deviation cap", None, [], {"OPENCLARE_NO_FOLD_GUARD": "1"}),
    ("engine", "+ fold guard: the whole engine, no option set", None, [], {}),
]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    common.options(parser, "ablation")
    parser.add_argument("--budgets", type=int, nargs="+", default=common.BUDGETS)
    parser.add_argument("--jobs", type=int, default=2, help="runs at a time (each engine run is parallel too)")
    parser.add_argument("--check", type=str,
                        help="directory of plain runs (<model>-<quads>.obj, e.g. twelve_models' output) that the "
                             "last line must equal byte for byte")
    arguments = parser.parse_args()
    common.use(arguments)

    started, clock = common.machine(), time.monotonic()
    lines, all_rows = [], []
    for key, label, paper, flags, environment in LINES:
        directory = arguments.out / key
        directory.mkdir(parents=True, exist_ok=True)
        jobs = [(f"{model}-{quads}", model, preset, quads, directory, flags, environment)
                for quads in arguments.budgets for model, preset in common.TWELVE]
        rows = common.batch(jobs, arguments.jobs)
        all_rows += rows
        mean = common.means(rows)
        lines.append({
            "key": key,
            "line": label,
            "paper_row": paper,
            "flags": flags,
            "environment": environment,
            "runs": len(rows),
            "valid": sum(1 for row in rows if row["metrics"] and row["metrics"]["valid"]),
            "failures": [f"{row['model']} {row['quads']}: {row['failure']}" for row in rows if row["failure"]],
            "spiral_loops_total": sum(row["metrics"]["loops_spiral"] or 0 for row in rows if row["metrics"]),
            "mean": mean,
            "mean_at_3000": common.means([row for row in rows if row["quads"] == 3000]) if 3000 in arguments.budgets
            else None,
            "run_seconds": round(sum(row["seconds"] or 0 for row in rows), 1),
        })
        print(f"{label}: spiral {100 * mean['loop_edges_spiral']:.2f}%, hausdorff {1000 * mean['hausdorff']:.2f}, "
              f"corners {mean['angle_deviation_mean']:.2f}", flush=True)

    identical = None
    if arguments.check:
        plain = common.Path(arguments.check)
        names = [f"{model}-{quads}.obj" for quads in arguments.budgets for model, _ in common.TWELVE]
        identical = sum(1 for name in names
                        if (plain / name).exists()
                        and filecmp.cmp(plain / name, arguments.out / "engine" / name, shallow=False))
        identical = f"{identical} of {len(names)}"
        print(f"whole engine against plain runs: {identical} identical")

    summary = {
        "experiment": "table tab:dev as an ablation on one engine",
        **common.header(arguments, started, [round(x, 2) for x in os.getloadavg()], time.monotonic() - clock,
                         all_rows),
        "budgets": arguments.budgets,
        "whole_engine_identical_to_plain_runs": identical,
        "lines": lines,
    }
    common.write_json(arguments.summary, summary)
    print(f"summary: {arguments.summary}")


if __name__ == "__main__":
    main()
