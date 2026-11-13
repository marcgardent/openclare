"""The report of a batch that compares the values of one axis (lab, SPEC.md
§6b, R4): reads the records lab wrote for its cells and the file each job
left in its folder, and computes nothing again.

    python3 bench/before_after.py <records.json> [--side ref] [--by budget]
        [--file measure.json] [--keys k1,k2] [--sum] [--out DIR]

The axis `--side` is compared (its values in the order of the grid, main
then 12a6675); `--by` gives one line per value of that axis. A cell counts
when the job of every side of it ended ok: the means (or with `--sum` the
sums) are over those pairs, so that both sides are measured on the same
cells. A boolean key (`valid`) counts its true values. The time is lab's,
with how many of those times count (a calm machine, SPEC.md §7.2). Then one
line per cell, each side's status and keys.

Writes report.md into `--out` (the folder of records.json by default) and
prints it.
"""

import argparse
import json
import math
import sys
from pathlib import Path

MEASURE_KEYS = "valid,hausdorff,mean_distance,loop_edges_spiral,irregular_vertices"


def number(text):
    if text is None:
        return "—"
    if isinstance(text, float):
        if math.isnan(text):
            return "—"
        return f"{text:.4g}"
    return str(text)


def read(cell, name):
    """What the job of a cell wrote, or None."""
    if cell.get("run") is None or cell.get("status") != "ok":
        return None
    path = Path(cell["out"]) / name
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError):
        return None


def value(data, key):
    found = data.get(key)
    if isinstance(found, bool):
        return int(found)
    return found if isinstance(found, (int, float)) else None


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records")
    parser.add_argument("--side", default="ref", help="the axis compared")
    parser.add_argument("--by", default="budget", help="one line per value of this axis")
    parser.add_argument("--file", default="measure.json", help="the JSON each job wrote in its folder")
    parser.add_argument("--keys", default=MEASURE_KEYS)
    parser.add_argument("--sum", action="store_true", help="sums over the cells, not means")
    parser.add_argument("--out")
    options = parser.parse_args()

    records = json.loads(Path(options.records).read_text())
    axes, cells = records["axes"], records["cells"]
    for axis in (options.side, options.by):
        if axis not in axes:
            sys.exit(f"{records['batch']} has no axis {axis} (its axes: {', '.join(axes)})")
    keys = [key for key in options.keys.split(",") if key]
    order = lambda axis: list(dict.fromkeys(cell["values"][axis] for cell in cells))
    sides, bys = order(options.side), order(options.by)
    others = [axis for axis in axes if axis != options.side]

    # The cells of each combination of the other axes, one per side.
    groups = {}
    for cell in cells:
        key = tuple(cell["values"][axis] for axis in others)
        groups.setdefault(key, {})[cell["values"][options.side]] = cell

    arrow = " → ".join
    how = "sum" if options.sum else "mean"
    lines = [
        f"# {records['batch']}: {arrow(sides)}",
        "",
        f"{how.capitalize()}s over the cells ok on every side ({options.file}); the time is lab's.",
        "",
        "| " + " | ".join([options.by, "pairs"] + keys + ["seconds", "times counting"]) + " |",
        "|" + "---|" * (len(keys) + 4),
    ]
    detail = []
    for by in bys:
        pairs = []
        for key_values, per_side in groups.items():
            if dict(zip(others, key_values))[options.by] != by:
                continue
            read_all = {side: read(per_side.get(side, {}), options.file) for side in sides}
            if all(data is not None for data in read_all.values()):
                pairs.append((per_side, read_all))
        row = [by, str(len(pairs))]
        for key in keys + ["seconds"]:
            texts = []
            for side in sides:
                if key == "seconds":
                    found = [per_side[side]["seconds"] for per_side, _ in pairs if per_side[side].get("seconds") is not None]
                else:
                    found = [value(data[side], key) for _, data in pairs]
                    found = [item for item in found if item is not None]
                boolean = key != "seconds" and any(isinstance(data[side].get(key), bool) for _, data in pairs)
                if not found:
                    texts.append("—")
                elif options.sum or boolean:
                    texts.append(number(sum(found)))
                else:
                    texts.append(number(sum(found) / len(found)))
            row.append(arrow(texts))
        counting = [str(sum(1 for per_side, _ in pairs if per_side[side].get("timing_valid"))) for side in sides]
        row.append(", ".join(counting))
        lines.append("| " + " | ".join(row) + " |")

    lines += ["", "| " + " | ".join(others + ["status"] + keys + ["seconds"]) + " |", "|" + "---|" * (len(others) + len(keys) + 2)]
    for key_values, per_side in groups.items():
        statuses, columns = [], {key: [] for key in keys + ["seconds"]}
        for side in sides:
            cell = per_side.get(side, {})
            statuses.append(cell.get("status") or "—")
            data = read(cell, options.file) or {}
            for key in keys:
                columns[key].append(number(value(data, key)))
            columns["seconds"].append(number(cell.get("seconds")))
        detail.append("| " + " | ".join(list(key_values) + [arrow(statuses)] + [arrow(columns[key]) for key in keys + ["seconds"]]) + " |")
    lines += detail
    text = "\n".join(lines) + "\n"
    out = Path(options.out) if options.out else Path(options.records).parent
    (out / "report.md").write_text(text)
    print(text, end="")


if __name__ == "__main__":
    main()
