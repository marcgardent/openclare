"""The report of the batch multires-baseline (lab, SPEC.md §6b, R4): the table
of PRO issue #6 beside the one the batch measured, from the records lab
wrote for its cells and the detail.json each job left in its folder.

    python3 bench/multires_report.py <records.json> [--out DIR]

One line per row of #6, one column per level: #6's missing detail (p99, %
of the diagonal), then the batch's. A cell is within the tolerance of the
baseline (milestone 2 of the specification) when the two differ by at most
0.01 point; one that is not is marked `!`. Then the stretched edges of each
level, #6's at L4 where it gave them; the gap of the write into the level;
and whether the levels a cell built on the way agree with the cells of those
levels (the same cage, the same steps: they must be equal).

Writes report.md into `--out` (the folder of records.json by default) and
prints it.
"""

import argparse
import json
from pathlib import Path

TOLERANCE = 0.01

# PRO issue #6 (2026-10-24, Blender 5.2.2): missing p99 from the cage to L4,
# and the stretched edges at L4 where it said them.
BASELINE = {
    "creature-project": ("creature (Project)", [0.16, 0.09, 0.03, 0.01, 0.00], None),
    "rock-project": ("rock (Project)", [0.41, 0.25, 0.10, 0.04, 0.02], None),
    "part-limited": ("part, no creases (Project, limited)", [0.37, 0.19, 0.13, 0.08, 0.04], None),
    "part-limited-creases": ("part, crease weights (Project, limited)", [0.37, 0.11, 0.04, 0.01, 0.00], None),
    "ogre-project": ("ogre, Project", [0.94, 0.53, 0.19, 0.07, 0.03], 2020),
    "ogre-nearest": ("ogre, nearest point", [0.94, 0.86, 0.61, 0.42, 0.24], 398),
    "ogre-limited": ("ogre, Project limited to one edge", [0.94, 0.56, 0.28, 0.19, 0.18], 0),
}


def read(cell):
    if cell.get("run") is None or cell.get("status") != "ok":
        return None
    try:
        return json.loads((Path(cell["out"]) / "detail.json").read_text())
    except (OSError, ValueError):
        return None


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records")
    parser.add_argument("--out")
    options = parser.parse_args()
    records = json.loads(Path(options.records).read_text())

    # Per row and level, the entry of that level in the cell that stops there,
    # and every entry of it in the cells that went higher.
    top, along, statuses, blender = {}, {}, {}, set()
    for cell in records["cells"]:
        row, level = cell["values"]["row"], int(cell["values"]["level"])
        statuses[row, level] = cell.get("status") or "never run"
        detail = read(cell)
        if detail is None:
            continue
        blender.add(detail.get("blender"))
        for entry in detail["levels"]:
            along.setdefault((row, entry["level"]), []).append(entry)
            if entry["level"] == level:
                top[row, level] = entry

    rows = list(dict.fromkeys(cell["values"]["row"] for cell in records["cells"]))
    levels = range(5)
    names = ["cage"] + [f"L{level}" for level in levels if level > 0]
    lines = [
        f"# {records['batch']}: PRO #6 → this batch",
        "",
        f"Missing detail, p99 in % of the diagonal, #6 → now; `!` past {TOLERANCE} point. "
        f"Blender {', '.join(sorted(str(b) for b in blender if b)) or '—'} (#6: 5.2.2).",
        "",
        "| row | " + " | ".join(names) + " |",
        "|---|" + "---|" * len(names),
    ]
    within = outside = 0
    for row in rows:
        title, baseline, _ = BASELINE.get(row, (row, [None] * 5, None))
        texts = []
        for level in levels:
            entry = top.get((row, level))
            old = baseline[level]
            if entry is None:
                texts.append(f"{old:.2f} → {statuses.get((row, level), '—')}")
                continue
            new = entry["missing_p99"]
            far = old is not None and abs(new - old) > TOLERANCE + 1e-12
            within += old is not None and not far
            outside += far
            texts.append(f"{old:.2f} → {new:.3f}{' !' if far else ''}")
        lines.append(f"| {title} | " + " | ".join(texts) + " |")
    lines += ["", f"{within} cells within {TOLERANCE} point of #6, {outside} outside.", ""]

    lines += [
        "Stretched edges (longer than 4 times the median edge of the level); #6 gave L4 for the ogre.",
        "",
        "| row | " + " | ".join(names) + " | #6 at L4 |",
        "|---|" + "---|" * (len(names) + 1),
    ]
    for row in rows:
        title, _, stretched = BASELINE.get(row, (row, None, None))
        texts = [str(top[row, level]["stretched"]) if (row, level) in top else "—" for level in levels]
        lines.append(f"| {title} | " + " | ".join(texts) + f" | {'—' if stretched is None else stretched} |")

    gaps = [entry["gap"] for entry in top.values()]
    spread = 0.0
    for entries in along.values():
        values = [entry["missing_p99"] for entry in entries]
        spread = max(spread, max(values) - min(values))
    lines += [
        "",
        f"Largest gap between the positions placed and the level read back: "
        f"{max(gaps) if gaps else float('nan'):.2e} % of the diagonal.",
        f"Largest difference of one level's missing p99 between the cells that built it: {spread:.2e} point.",
    ]
    text = "\n".join(lines) + "\n"
    out = Path(options.out) if options.out else Path(options.records).parent
    (out / "report.md").write_text(text)
    print(text, end="")


if __name__ == "__main__":
    main()
