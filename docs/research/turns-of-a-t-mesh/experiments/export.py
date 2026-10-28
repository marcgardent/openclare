#!/usr/bin/env python3
"""Makes the export the other scripts read: the engine remeshes the male body
(or another source) with `OPENCLARE_EXPORT_TURNS`, and the mesh is measured.

    python3 docs/research/turns-of-a-t-mesh/experiments/export.py \\
        [--openclare target/release/openclare] [--commit <hash>] [--engine as-is|before-cuts] \\
        [--source bench/out/human_male/source.obj] [--quads 10590] [--preset organic] [--out <dir>]

`--engine as-is` (the default) is the engine of the commit as published, the
one the papers' numbers are made on. `before-cuts`, an aside for comparison
with the drafts' first export, sets the experiment options that switch off
the stages the engine of that export did not have (ALGORITHM.md, M6,
"Experiment options"): no rounds of constraint generation, no active set, no
floor of the deviation cap, no fold guard, no other draws. The rest of the
engine is that of the commit, so this is not the old engine (field, T-mesh
and work mesh have changed since).

Writes, under --out (default scratch/data-package/turns-of-a-t-mesh/male-body,
or male-body-before-cuts): `export/` (the engine's files), `mesh.obj`,
`run.log`, `measure.json` (`openclare measure --json`) and `record.json`
(the command, the options, the commit and build of the engine, the MD5 of
the source, the machine and the time), which the summaries of the other
scripts copy. A minute or two on the male body; nothing is done if
`record.json` is there already.
"""

import argparse
import json
import os
import subprocess
import time
from pathlib import Path

import tmesh

ENGINES = {
    "as-is": {},
    "before-cuts": {"OPENCLARE_NO_CUT_ROUNDS": "1", "OPENCLARE_NO_ACTIVE_SET": "1",
                    "OPENCLARE_NO_DEVIATION_FLOOR": "1", "OPENCLARE_NO_FOLD_GUARD": "1",
                    "OPENCLARE_EXTRA_DRAWS": "0"},
}
MEASURES = ("valid", "quads", "loop_edges_spiral", "loops_spiral", "loop_edges_closed", "mean_distance",
            "hausdorff", "angle_deviation_mean", "irregular_vertices", "inverted_faces")


def run(binary, source, quads, preset, options, out, memory=tmesh.common.MEMORY_LARGE, flags=()):
    """`openclare remesh` with the experiment options `options` (the turns
    exported to out/export), then `openclare measure`: (seconds, measure),
    the log in out/run.log."""
    out.mkdir(parents=True, exist_ok=True)
    environment = {key: value for key, value in os.environ.items() if not key.startswith("OPENCLARE_")}
    environment.update({"OPENCLARE_EXPORT_TURNS": str((out / "export").resolve()), **options})
    command = [str(Path(binary).resolve()), "remesh", str(Path(source).resolve()), str(out / "mesh.obj"),
               "--preset", preset, "--quads", str(quads), *flags]
    clock = time.monotonic()
    with open(out / "run.log", "w") as log:
        done = subprocess.run(command, env=environment, stdout=log, stderr=subprocess.STDOUT,
                              preexec_fn=tmesh.common.capped(memory))
    seconds = time.monotonic() - clock
    if done.returncode != 0:
        raise RuntimeError(f"the engine failed ({done.returncode}): see {out / 'run.log'}")
    tmesh.common.OPENCLARE = Path(binary).resolve()
    measure = tmesh.common.measure(source, out / "mesh.obj", quads, memory)
    (out / "measure.json").write_text(json.dumps(measure, indent=1) + "\n")
    return seconds, measure


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--openclare", type=Path, default=tmesh.common.OPENCLARE, help="the openclare binary")
    parser.add_argument("--commit", help="the commit the binary was built from (default: this clone's)")
    parser.add_argument("--engine", choices=sorted(ENGINES), default="as-is")
    parser.add_argument("--source", type=Path, default=tmesh.ROOT / "bench" / "out" / "human_male" / "source.obj")
    parser.add_argument("--quads", type=int, default=10590)
    parser.add_argument("--preset", default="organic")
    parser.add_argument("--out", type=Path)
    parser.add_argument("--memory", type=int, default=tmesh.common.MEMORY_LARGE, help="memory cap of the engine, KB")
    arguments = parser.parse_args()
    out = arguments.out or tmesh.DATA / "turns-of-a-t-mesh" / (
        "male-body" if arguments.engine == "as-is" else f"male-body-{arguments.engine}")
    if (out / "record.json").exists():
        print(f"{out / 'record.json'} exists: nothing to do")
        return
    if not arguments.openclare.exists():
        raise SystemExit(f"no engine at {arguments.openclare}: cargo build --release, or --openclare <binary>")
    tmesh.common.OPENCLARE = arguments.openclare.resolve()
    commit = arguments.commit or tmesh.common.commit()
    started = tmesh.common.machine()
    seconds, measure = run(arguments.openclare, arguments.source, arguments.quads, arguments.preset,
                           ENGINES[arguments.engine], out, arguments.memory)
    folder, ways = tmesh.ways(out / "export")
    record = {
        "experiment": "export of the turns of the closing",
        "engine": {"commit": commit, "build": tmesh.common.engine_version(), "options": arguments.engine,
                   "environment": ENGINES[arguments.engine]},
        "command": ["openclare", "remesh", "<source>", "mesh.obj", "--preset", arguments.preset,
                    "--quads", str(arguments.quads)],
        "source": {"path": arguments.source.name, "md5": tmesh.common.md5(arguments.source)},
        "machine": {**started, "load_average_at_end": [round(x, 2) for x in os.getloadavg()]},
        "seconds": round(seconds, 1),
        "tmesh": folder.name,
        "ways": sorted(ways),
        "measure": {key: measure.get(key) for key in MEASURES},
    }
    tmesh.common.write_json(out / "record.json", record)
    print(f"{folder.name}: ways {sorted(ways)}, {measure.get('quads')} quads, spiral edges "
          f"{100 * measure['loop_edges_spiral']:.2f}% ({seconds:.0f} s); record: {out / 'record.json'}")


if __name__ == "__main__":
    main()
