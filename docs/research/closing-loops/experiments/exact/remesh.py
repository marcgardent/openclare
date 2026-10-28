#!/usr/bin/env python3
"""The meshes of Section "An exact look at the closing system": the engine
remeshes the male body with other targets for its integer search, or with
whole lengths worked out outside it, and each mesh is measured. Table
`tab:rounding` and the meshes of paragraphs "Whole lengths are not what is
missing" and "The heuristic leaves most of the gap".

Every run sets the options of --engine (`as-is` by default: the engine as
published, the one the papers' numbers are made on; `before-cuts` switches
off the stages the engine of the drafts' first export did not have, as
`export.py` does, for comparison only) and exports
its turns, from which the summary reads, way by way, the turns closed in
real lengths (`kept`) and those still shifted at the way's whole lengths,
and, for the lengths ranked first, their quads, deviation and the strips of
quads they make (ways round, shifted). The variants:

- `engine`: the engine as the options give it;
- `projection-clamped`: the projection cuts lengths below their floor off
  afterwards (`OPENCLARE_NO_ACTIVE_SET`; the same as `engine` with
  `--engine before-cuts`);
- `active-set-floor-0`, `active-set-floor-half`: the active set, with the
  floor 0 or half the real length (`OPENCLARE_ACTIVE_SET_FLOOR=0.5`);
- `aimed`: the search aims at the lengths of the linear program
  (`OPENCLARE_AIMED`, the `aimed-<way>.txt` of most_closable.py);
- `whole-<way>` and `engine-<way>`: that way alone (`OPENCLARE_CLOSE_WAYS`),
  with the exact whole lengths of most_closable.py --whole
  (`way-<way>-integer.txt`, `OPENCLARE_LENGTHS`) or with the engine's;
- `rings`: the lengths of lattice_rounding.py (`call-<name>.txt`);
- `local-search`: the lengths of local_search.py.

    python3 docs/research/closing-loops/experiments/exact/remesh.py --openclare <binary> --commit <hash> \\
        [--engine as-is] [--variants engine,active-set-floor-0,...] [--source <male body>]

A variant whose files are missing is skipped and said so. One to three
minutes a run on the male body (the whole machine). Writes the runs under
--out (default scratch/data-package/closing-loops/remesh/<variant>) and
results/remesh.json.
"""

import argparse
import json
import os
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "turns-of-a-t-mesh" / "experiments"))
import tmesh  # noqa: E402  (first: it sets the threads of the linear algebra)

import export  # noqa: E402

DATA = tmesh.DATA / "closing-loops"
VARIANTS = ["engine", "projection-clamped", "active-set-floor-0", "active-set-floor-half", "aimed", "whole",
            "rings", "local-search"]


def variants(arguments):
    """[(name, options)], and the variants skipped with why."""
    base = dict(export.ENGINES[arguments.engine])
    no_active = {k: v for k, v in base.items() if k != "OPENCLARE_NO_ACTIVE_SET"}
    out, skipped = [], {}
    asked = arguments.variants.split(",")
    for name in asked:
        if name == "engine":
            out.append((name, base))
        elif name == "projection-clamped":
            out.append((name, {**base, "OPENCLARE_NO_ACTIVE_SET": "1"}))
        elif name == "active-set-floor-0":
            out.append((name, no_active))
        elif name == "active-set-floor-half":
            out.append((name, {**no_active, "OPENCLARE_ACTIVE_SET_FLOOR": "0.5"}))
        elif name == "aimed":
            found = list(arguments.most_closable.glob("call-*/aimed-*.txt"))
            if found:
                out.append((name, {**base, "OPENCLARE_AIMED": str(arguments.most_closable.resolve())}))
            else:
                skipped[name] = f"no call-<name>/aimed-<way>.txt in {arguments.most_closable}"
        elif name == "whole":
            found = sorted(arguments.most_closable.glob("call-*/way-*-integer.txt"))
            if not found:
                skipped[name] = f"no call-<name>/way-<way>-integer.txt in {arguments.most_closable}"
            for path in found:
                way = path.name[len("way-"):-len("-integer.txt")]
                alone = {**base, "OPENCLARE_CLOSE_WAYS": way}
                out.append((f"engine-{way}", alone))
                out.append((f"whole-{way}", {**alone, "OPENCLARE_LENGTHS": str(arguments.most_closable.resolve())}))
        elif name in ("rings", "local-search"):
            folder = arguments.lattice_rounding if name == "rings" else arguments.local_search
            if list(folder.glob("call-*.txt")):
                out.append((name, {**base, "OPENCLARE_LENGTHS": str(folder.resolve())}))
            else:
                skipped[name] = f"no call-<name>.txt in {folder}"
        else:
            raise SystemExit(f"unknown variant {name}: {VARIANTS}")
    return out, skipped


def read_run(out):
    """What the run's export says: the lengths ranked first, and way by way
    the turns closed in real lengths and still shifted at whole lengths."""
    folder, paths = tmesh.ways(out / "export")
    call = folder.parent / f"{folder.name}.json"
    first = json.loads(call.read_text()) if call.exists() else {}
    strips = first.get("strips") or []
    summary = {"tmesh": folder.name,
               "first": {key: first.get(key) for key in ("way", "round", "quads", "deviation", "spirals",
                                                          "most_deviation", "turns_held", "from_selector")},
               "strips": {"ways_round": len(strips),
                          "shifted": sum(1 for s in strips if abs(s.get("pitch", 0)) >= 0.5)}}
    ways = []
    for number, path in paths.items():
        way = tmesh.Way(path)
        held = tmesh.held_forms(way)
        kept = [way.shift[k] for k in way.kept]
        ways.append({"way": number, "held": len(held), "closed_in_real_lengths": len(kept),
                     "of_those_still_shifted": way.shifted(kept, way.lengths),
                     "held_still_shifted": way.shifted(held, way.lengths), "quads": way.quads(way.lengths),
                     "spirals": way.data.get("spirals")})
    summary["ways"] = ways
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--openclare", type=Path, default=tmesh.common.OPENCLARE, help="the openclare binary")
    parser.add_argument("--commit", help="the commit the binary was built from (default: this clone's)")
    parser.add_argument("--engine", choices=sorted(export.ENGINES), default="as-is")
    parser.add_argument("--variants", default=",".join(VARIANTS))
    parser.add_argument("--source", type=Path, default=tmesh.ROOT / "bench" / "out" / "human_male" / "source.obj")
    parser.add_argument("--quads", type=int, default=10590)
    parser.add_argument("--preset", default="organic")
    parser.add_argument("--most-closable", type=Path, default=DATA / "most_closable",
                        help="the files of most_closable.py (its --out)")
    parser.add_argument("--lattice-rounding", type=Path, default=DATA / "lattice_rounding",
                        help="the files of lattice_rounding.py (its --out)")
    parser.add_argument("--local-search", type=Path, default=DATA / "local_search",
                        help="the files of local_search.py (its --out)")
    parser.add_argument("--out", type=Path, default=DATA / "remesh")
    parser.add_argument("--summary", type=Path, default=Path(__file__).resolve().parent / "results" / "remesh.json")
    arguments = parser.parse_args()
    if not arguments.openclare.exists():
        raise SystemExit(f"no engine at {arguments.openclare}: cargo build --release, or --openclare <binary>")
    tmesh.common.OPENCLARE = arguments.openclare.resolve()
    commit = arguments.commit or tmesh.common.commit()
    started, clock = tmesh.common.machine(), time.monotonic()
    runs, skipped = variants(arguments)
    for name, why in skipped.items():
        print(f"{name}: skipped, {why}", flush=True)
    rows = []
    for name, options in runs:
        out = arguments.out / name
        record = out / "record.json"
        if record.exists():
            rows.append(json.loads(record.read_text()))
            continue
        seconds, measure = export.run(arguments.openclare, arguments.source, arguments.quads, arguments.preset,
                                      options, out)
        row = {"variant": name, "options": options, "seconds": round(seconds, 1),
               "measure": {key: measure.get(key) for key in export.MEASURES}, **read_run(out)}
        tmesh.common.write_json(record, row)
        rows.append(row)
        print(f"{name}: spiral edges {100 * measure['loop_edges_spiral']:.2f}%, valid {measure['valid']}, "
              f"{measure.get('quads')} quads; first lengths from way {row['first']['way']}; strips "
              f"{row['strips']} ({seconds:.0f} s)", flush=True)
    summary = {
        "experiment": "the meshes of the exact look (Table tab:rounding and the meshes of whole lengths)",
        "engine": {"commit": commit, "build": tmesh.common.engine_version(), "options": arguments.engine},
        "source": {"path": arguments.source.name, "md5": tmesh.common.md5(arguments.source)},
        "machine": {**started, "processors_allowed": len(os.sched_getaffinity(0))},
        "wall_seconds": round(time.monotonic() - clock, 1),
        "skipped": skipped,
        "runs": rows,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
