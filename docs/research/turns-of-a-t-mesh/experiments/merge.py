#!/usr/bin/env python3
"""The summary of a script split into a batch of lab (lab/SPEC.md, section
6b, "Reports (amendment R4)"), made again from its cells: each job ran the
script on one unit (a way, a family, a row) and wrote its summary to
`{out}/<name>.json`; this reads them, in the order of the cells in
records.json, and writes the one summary the script writes unsplit.

    python3 docs/research/turns-of-a-t-mesh/experiments/merge.py <records.json> --file <name>.json
        --mode concat|holonomy|conjugacy|certify [--pairs results/pairs.json] [--out DIR]

The head (the export, its record, the solver, the parameters) is the first
cell's, with every key that differs between cells named in `head_differs`;
`ways` is the union of the cells' ways; `machine` is the first cell's (each
cell's load is in `merged_from`); `wall_seconds` is the longest cell's, and
`cells_seconds_sum` the sum over the cells. The body, by `--mode`:

- `concat` (lattice.py, forced.py, strict.py): the lists `per_way`, `table`,
  `per_family` and `rows` concatenated;
- `holonomy`: `total` summed, `extremes` the least or the most by key,
  `dislocation_annuli` and `per_way` concatenated;
- `conjugacy`: `presentations` united, `rows` in the order of `--pairs`
  (pairs.py's summary) when it has them all, else in the order of the cells;
- `certify` (`../lean/Check/certify.py`, which writes certificates.json, not
  a summary): the `ways` of the certificates united, by the SHA-256 of the way
  file, as `../lean/Check/certificates/male-body.json` was made.

A cell counts when its job ended ok and its file reads. `complete` is false
when one does not (and, with forced.py, when a cell's own summary is not
complete); `missing` lists those cells. Standard library only; computes
nothing again. Writes DIR/<name> (DIR: the folder of records.json by default).
"""

import argparse
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent

# The keys every mode merges itself, never compared as head.
MERGED = {"ways", "machine", "wall_seconds", "complete"}
BODY = {
    "concat": ["per_way", "table", "per_family", "rows"],
    "holonomy": ["total", "extremes", "dislocation_annuli", "per_way"],
    "conjugacy": ["presentations", "rows"],
    "certify": ["ways"],
}


def read(cell, name):
    """What the job of a cell wrote, or the reason there is nothing."""
    if cell.get("run") is None:
        return None, "never run"
    if cell.get("status") != "ok":
        return None, cell.get("status") or "no status"
    path = Path(cell["out"]) / name
    try:
        return json.loads(path.read_text()), None
    except OSError:
        return None, f"no {name}"
    except ValueError:
        return None, f"{name} does not read as JSON"


def label(cell):
    values = ", ".join(f"{axis}={value}" for axis, value in (cell.get("values") or {}).items())
    return f"{values or 'the cell'} (run {cell.get('run')}, {cell.get('status') or 'never run'})"


def concatenated(summaries, key):
    out = []
    for summary in summaries:
        out += summary.get(key, [])
    return out


def united(summaries, key, differs):
    """The dicts `key` of the summaries in one, the first cell's value kept
    where two cells give one key different values."""
    out = {}
    for summary in summaries:
        for name, value in summary.get(key, {}).items():
            if name in out and out[name] != value:
                differs.add(f"{key}.{name}")
            out.setdefault(name, value)
    return out


def merge_concat(summaries, differs, arguments):
    return {key: concatenated(summaries, key) for key in BODY["concat"]
            if isinstance(summaries[0].get(key), list)}


def merge_holonomy(summaries, differs, arguments):
    total, extremes = {}, {}
    for summary in summaries:
        for key, value in summary.get("total", {}).items():
            total[key] = total.get(key, 0) + value
        for key, value in summary.get("extremes", {}).items():
            extremes[key] = (min if "least" in key else max)(extremes.get(key, value), value)
    return {"total": dict(sorted(total.items())), "extremes": extremes,
            "dislocation_annuli": concatenated(summaries, "dislocation_annuli"),
            "per_way": concatenated(summaries, "per_way")}


def merge_conjugacy(summaries, differs, arguments):
    rows = concatenated(summaries, "rows")
    try:
        pairs = json.loads(arguments.pairs.read_text())["rows"]
    except (OSError, ValueError, KeyError):
        pairs = []
    place = {(row["way"], tuple(row["turns"])): i for i, row in enumerate(pairs)}
    if rows and all((row["way"], tuple(row["turns"])) in place for row in rows):
        rows.sort(key=lambda row: place[(row["way"], tuple(row["turns"]))])
    else:
        print(f"rows in the order of the cells: {arguments.pairs.name} does not have them all")
    presentations = united(summaries, "presentations", differs)
    # The ways in the order the script meets them, that of the rows.
    order = list(dict.fromkeys(str(row["way"]) for row in rows))
    order += [way for way in presentations if way not in order]
    return {"presentations": {way: presentations[way] for way in order if way in presentations},
            "rows": rows, "_order": order}


def merge_certify(summaries, differs, arguments):
    return {"ways": united(summaries, "ways", differs)}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records", type=Path, help="the records.json lab wrote for the batch")
    parser.add_argument("--file", required=True, help="the file each job wrote in its {out}, as lattice.json")
    parser.add_argument("--mode", required=True, choices=sorted(BODY), help="how the cells are merged")
    parser.add_argument("--pairs", type=Path, default=HERE / "results" / "pairs.json",
                        help="with --mode conjugacy: the summary of pairs.py, for the order of the rows")
    parser.add_argument("--out", type=Path, help="where the merged file goes (default: the folder of records.json)")
    arguments = parser.parse_args()
    records = json.loads(arguments.records.read_text())
    cells, summaries, missing = [], [], []
    for cell in records["cells"]:
        summary, reason = read(cell, arguments.file)
        machine = (summary or {}).get("machine") or {}
        cells.append({"values": cell.get("values"), "run": cell.get("run"), "status": cell.get("status"),
                      "commit": cell.get("commit"), "checkout": cell.get("checkout"),
                      "seconds": cell.get("seconds"), "timing_valid": cell.get("timing_valid"),
                      "read": summary is not None,
                      **({"wall_seconds": summary.get("wall_seconds"),
                          "load_average": machine.get("load_average"),
                          "load_average_at_end": machine.get("load_average_at_end")}
                         if summary is not None and arguments.mode != "certify" else {})})
        if summary is None:
            missing.append({"values": cell.get("values"), "run": cell.get("run"), "status": cell.get("status"),
                            "reason": reason})
            print(f"missing: {label(cell)}: {reason}")
        else:
            summaries.append(summary)
    if not summaries:
        sys.exit(f"no cell of {arguments.records} left a {arguments.file}: nothing written")

    differs = set()
    first = summaries[0]
    body = globals()[f"merge_{arguments.mode}"](summaries, differs, arguments)
    order = body.pop("_order", None)
    for key, value in first.items():
        if key in MERGED or key in body or key == "note":
            continue
        if any(summary.get(key) != value for summary in summaries[1:]):
            differs.add(key)
    complete = not missing and all(summary.get("complete", True) for summary in summaries)
    merged_from = {"batch": records.get("batch"), "axes": records.get("axes"), "launch": records.get("launch"),
                   "file": arguments.file, "mode": arguments.mode, "cells": cells}

    if arguments.mode == "certify":
        notes = list(dict.fromkeys(summary.get("note", "") for summary in summaries if summary.get("note")))
        merged = {"note": "; ".join(notes), **body}
    else:
        ways = united(summaries, "ways", differs)
        if order is not None:
            ways = {way: ways[way] for way in order + [w for w in ways if w not in order] if way in ways}
        walls = [summary.get("wall_seconds") or 0.0 for summary in summaries]
        merged = {}
        for key, value in first.items():
            if key == "experiment":
                merged[key] = value
                merged["complete"] = complete
            elif key == "complete":
                continue
            elif key == "ways":
                merged[key] = ways
            elif key == "wall_seconds":
                merged[key] = max(walls)
                merged["cells_seconds_sum"] = round(sum(walls), 1)
            elif key in body:
                merged[key] = body.pop(key)
            else:
                merged[key] = value
        merged.update(body)
        merged.setdefault("complete", complete)
    merged["complete"] = complete
    if missing:
        merged["missing"] = missing
    if differs:
        merged["head_differs"] = sorted(differs)
        print(f"differs between cells (the first cell's kept): {', '.join(sorted(differs))}")
    merged["merged_from"] = merged_from

    out = (arguments.out or arguments.records.parent) / arguments.file
    out.parent.mkdir(parents=True, exist_ok=True)
    if arguments.mode == "certify":
        out.write_text(json.dumps(merged, separators=(",", ":")))
    else:
        out.write_text(json.dumps(merged, indent=1) + "\n")
    print(f"{len(summaries)} of {len(cells)} cells merged into {out}"
          f"{'' if complete else ' (not complete)'}")


if __name__ == "__main__":
    main()
