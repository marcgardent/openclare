#!/usr/bin/env python3
"""The numbers of Figures `fig:overview` and `fig:closeups` (the caption of
fig:overview, and the table of figures/README.md), read from the measures
figures/make_figures.sh leaves in its work directory, into
results/figures.json.

    WORK=<directory> OPENCLARE=<binary> COMMIT=<hash> docs/research/closing-loops/figures/make_figures.sh
    python3 docs/research/closing-loops/experiments/figure_numbers.py <directory> [--commit <hash>]
"""

import argparse
import json
from pathlib import Path

import common

MESHES = ["no_closing", "closing", "handmade", "quadriflow"]
KEYS = ["valid", "faces", "quads", "irregular_vertices", "inverted_faces", "loop_edges_spiral", "loops_spiral",
        "loop_edges_closed", "loops_closed", "mean_distance", "hausdorff", "angle_deviation_mean",
        "loop_edges_helix", "loops_helix_missed"]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("work", type=Path, help="the work directory of make_figures.sh")
    parser.add_argument("--commit", help="the commit of the engine (default: this clone's)")
    parser.add_argument("--summary", type=Path, default=common.RESULTS / "figures.json")
    arguments = parser.parse_args()
    meshes = {name: json.loads((arguments.work / f"{name}.json").read_text()) for name in MESHES}
    summary = {
        "experiment": "figures fig:overview and fig:closeups",
        "engine": {"commit": arguments.commit or common.commit()},
        "meshes": {name: {key: metrics.get(key) for key in KEYS} for name, metrics in meshes.items()},
    }
    common.write_json(arguments.summary, summary)
    for name, row in summary["meshes"].items():
        print(f"{name}: {row['quads']} quads, spiral {100 * row['loop_edges_spiral']:.2f}% ({row['loops_spiral']}), "
              f"closed {100 * row['loop_edges_closed']:.1f}%, Hausdorff {1000 * row['hausdorff']:.2f}, "
              f"inverted {row['inverted_faces']}")


if __name__ == "__main__":
    main()
