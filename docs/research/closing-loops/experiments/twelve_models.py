#!/usr/bin/env python3
"""The twelve development models at three budgets (Section "Experiments",
Appendix "Provenance", the protocol of the twelve models): 36 runs of
`openclare remesh <source> <out> --preset P --quads Q`, each measured with
`openclare measure <source> <out> --target Q --json`, and the means over the
36 runs (and over each budget) of the measures the paper quotes and of the
independent measures of structure (DT-137).

    python3 docs/research/closing-loops/experiments/twelve_models.py \
        [--openclare target/release/openclare] [--commit <hash>] [--jobs 2]

Writes the meshes and their measures under --out, and the summary to
results/twelve_models.json.
"""

import argparse
import os
import time

import common


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    common.options(parser, "twelve_models")
    parser.add_argument("--jobs", type=int, default=2, help="runs at a time (each engine run is parallel too)")
    arguments = parser.parse_args()
    common.use(arguments)

    started, clock = common.machine(), time.monotonic()
    jobs = [(f"{model}-{quads}", model, preset, quads, arguments.out, (), None)
            for quads in common.BUDGETS for model, preset in common.TWELVE]
    rows = common.batch(jobs, arguments.jobs)
    seconds = time.monotonic() - clock

    summary = {
        "experiment": "twelve models, three budgets",
        **common.header(arguments, started, [round(x, 2) for x in os.getloadavg()], seconds, rows),
        "sources": {model: common.md5(common.source(model)) for model, _ in common.TWELVE},
        "runs": len(rows),
        "valid": sum(1 for row in rows if row["metrics"] and row["metrics"]["valid"]),
        "failures": [f"{row['model']} {row['quads']}: {row['failure']}" for row in rows if row["failure"]],
        "spiral_loops_total": sum(row["metrics"]["loops_spiral"] or 0 for row in rows if row["metrics"]),
        "mean": common.means(rows),
        "mean_by_budget": {quads: common.means([row for row in rows if row["quads"] == quads])
                           for quads in common.BUDGETS},
        "per_run": [{"model": row["model"], "quads": row["quads"], "seconds": row["seconds"],
                     "valid": row["metrics"]["valid"] if row["metrics"] else None,
                     **{key: row["metrics"].get(key) if row["metrics"] else None for key in common.MEASURES}}
                    for row in rows],
    }
    common.write_json(arguments.summary, summary)
    mean = summary["mean"]
    print(f"spiral edges {100 * mean['loop_edges_spiral']:.2f}% "
          f"({', '.join(f'{100 * summary["mean_by_budget"][q]["loop_edges_spiral"]:.2f}%' for q in common.BUDGETS)}), "
          f"{summary['spiral_loops_total']} spiral loops, {summary['valid']}/{summary['runs']} valid, "
          f"closed {100 * mean['loop_edges_closed']:.1f}%, mean distance {1000 * mean['mean_distance']:.3f} per mille, "
          f"corners {mean['angle_deviation_mean']:.2f} deg; helices {100 * mean['loop_edges_helix']:.2f}%")
    print(f"summary: {arguments.summary}")


if __name__ == "__main__":
    main()
