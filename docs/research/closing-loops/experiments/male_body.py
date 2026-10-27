#!/usr/bin/env python3
"""The male body of Blender's Essentials library against its hand-made base
mesh (Section "Experiments", "Against the hand-made mesh"): openclare at the
hand-made mesh's 10,590 quads, preset organic, with closing, without it, and
with the lean bound 0.05 alone; the hand-made mesh measured the same way.

    python3 docs/research/closing-loops/experiments/male_body.py \
        [--openclare target/release/openclare] [--commit <hash>] [--source <male body .obj>]

The source is the body as the bench gives it to every remesher
(bench/out/human_male/source.obj; see figures/README.md for how to make it).
The meshes are written under --out as no_closing.obj and closing.obj, the
names figures/make_figures.sh reuses (WORK=<that directory>).
"""

import argparse
import json
import os
import time
from pathlib import Path

import common

QUADS = 10590
HANDMADE = common.ROOT / "bench" / "reference" / "handmade" / "human_male.obj"
# The lean bound 0.05 alone: ways 0, 1, 4 and 5 (`m + 2j + 4i` with j = 0).
RUNS = [
    ("closing", "openclare, closing (the engine as it is)", [], {}),
    ("no_closing", "openclare, no closing", ["--no-close-loops"], {}),
    ("lean_005", "openclare, closing with the lean bound 0.05 alone (ways 0,1,4,5)", [],
     {"OPENCLARE_CLOSE_WAYS": "0,1,4,5"}),
]
KEYS = ["valid", "faces", "quads", "irregular_vertices", "loop_edges_spiral", "loops_spiral", "loop_edges_closed",
        "loops_closed", "mean_distance", "hausdorff", "angle_deviation_mean", "loop_edges_helix",
        "loops_helix_missed", "base_complex_patches", "inverted_faces"]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    common.options(parser, "male_body")
    parser.add_argument("--source", type=Path, help="the male body (default <sources>/human_male/source.obj)")
    arguments = parser.parse_args()
    common.use(arguments)
    source = (arguments.source or common.source("human_male")).resolve()

    started, clock = common.machine(), time.monotonic()
    meshes = {}
    for name, label, flags, environment in RUNS:
        mesh, record = arguments.out / f"{name}.obj", arguments.out / f"{name}.json"
        if record.exists():
            meshes[name] = json.loads(record.read_text())
            continue
        seconds, load = None, round(os.getloadavg()[0], 2)
        if not mesh.exists():
            seconds, failure = common.remesh(source, mesh, "organic", QUADS, flags, environment,
                                             memory=common.MEMORY_LARGE, timeout=3600)
            if failure:
                raise SystemExit(f"{name}: {failure}")
        row = {"label": label, "flags": flags, "environment": environment,
               "seconds": None if seconds is None else round(seconds, 1), "load_average": load,
               "metrics": common.measure(source, mesh, QUADS, common.MEMORY_LARGE)}
        common.write_json(record, row)
        meshes[name] = row
        print(f"{name}: {100 * row['metrics']['loop_edges_spiral']:.2f}% spiral ({seconds and round(seconds)} s)",
              flush=True)
    handmade = common.measure(source, HANDMADE, QUADS, common.MEMORY_LARGE)

    # The bench's own run of the add-on chain at the hand-made count, if the
    # results file has it (bench/results/results.json, "openclare-as-handmade").
    bench = json.loads((common.ROOT / "bench" / "results" / "results.json").read_text())
    as_handmade = bench["runs"].get("human_male", {}).get("openclare-as-handmade", [])

    summary = {
        "experiment": "male body against the hand-made mesh",
        **common.header(arguments, started, [round(x, 2) for x in os.getloadavg()], time.monotonic() - clock,
                         list(meshes.values())),
        "source_md5": common.md5(source),
        "quads": QUADS,
        "openclare": {name: {"label": row["label"], "seconds": row["seconds"],
                             **{key: row["metrics"].get(key) for key in KEYS}} for name, row in meshes.items()},
        "handmade": {key: handmade.get(key) for key in KEYS},
        "bench_openclare_as_handmade": {key: as_handmade[0]["metrics"].get(key) for key in KEYS}
        if as_handmade and as_handmade[0].get("metrics") else None,
    }
    common.write_json(arguments.summary, summary)
    for name, row in [*summary["openclare"].items(), ("handmade", summary["handmade"])]:
        print(f"{name}: spiral {100 * row['loop_edges_spiral']:.2f}% ({row['loops_spiral']} loops), "
              f"closed {100 * row['loop_edges_closed']:.1f}%, distance {1000 * row['mean_distance']:.3f} per mille, "
              f"corners {row['angle_deviation_mean']:.2f} deg, helices {100 * row['loop_edges_helix']:.2f}%")
    print(f"summary: {arguments.summary}")


if __name__ == "__main__":
    main()
