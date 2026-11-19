"""The report of the batch multires-projection (milestone 3 of the Multires
specification): the engine's guarded projection, level after level, judged
against the acceptance criteria of the milestone, from the records lab wrote
for its cells and the detail.json each job left in its folder.

    python3 bench/multires_projection_report.py <records.json> [--out DIR]

The criteria, per row, at every level from L1 to L4 (missing detail is the
p99 in % of the diagonal; #6 is PRO issue #6's table, given to two decimals):

- creature, rock: missing at most Project's in #6 plus 0.01 point;
- part with the cage's crease weights: at most limited Project with creases
  in #6 (plus 0.005, half its last digit);
- ogre: no stretched edge at any level, and missing below 0.18 at L4;
- the other rows are shown, not judged.

Then what became of the vertices of each level. Writes report.md into
`--out` (the folder of records.json by default) and prints it.
"""

import argparse
import json
from pathlib import Path

# PRO issue #6 (2026-10-24, Blender 5.2.2), missing p99 from the cage to L4.
PROJECT = {"creature": [0.16, 0.09, 0.03, 0.01, 0.00], "rock": [0.41, 0.25, 0.10, 0.04, 0.02]}
LIMITED_CREASES = [0.37, 0.11, 0.04, 0.01, 0.00]
OGRE_L4 = 0.18


def criterion(row):
    """For a row, a function of (level, entry) giving (bound text, passed),
    or None when the row is not judged."""
    model = row.split("-")[0]
    if row in ("creature-openclare", "rock-openclare"):
        bound = PROJECT[model]
        return lambda level, e: (f"≤ {bound[level] + 0.01:.2f}", e["missing_p99"] <= bound[level] + 0.01 + 1e-12)
    if row == "part-openclare-creases":
        return lambda level, e: (
            f"≤ {LIMITED_CREASES[level] + 0.005:.3f}",
            e["missing_p99"] <= LIMITED_CREASES[level] + 0.005 + 1e-12,
        )
    if row == "ogre-openclare":

        def ogre(level, e):
            fine = e["stretched"] == 0 and (level < 4 or e["missing_p99"] < OGRE_L4)
            return ("0 stretched" + (f", < {OGRE_L4}" if level == 4 else ""), fine)

        return ogre
    return None


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records")
    parser.add_argument("--out")
    options = parser.parse_args()
    records = json.loads(Path(options.records).read_text())

    lines = [
        f"# {records['batch']}: the guarded projection against milestone 3",
        "",
        "Missing detail, p99 in % of the diagonal, and stretched edges, per level; "
        "the bound of the milestone beside each judged cell, `✗` where it is not met.",
        "",
        "| row | cage | L1 | L2 | L3 | L4 |",
        "|---|---|---|---|---|---|",
    ]
    placed_lines = []
    judged = failed = 0
    for cell in records["cells"]:
        row = cell["values"]["row"]
        detail = None
        if cell.get("status") == "ok":
            try:
                detail = json.loads((Path(cell["out"]) / "detail.json").read_text())
            except (OSError, ValueError):
                detail = None
        if detail is None:
            lines.append(f"| {row} | " + " | ".join([cell.get("status") or "never run"] * 5) + " |")
            continue
        judge = criterion(row)
        texts = []
        for entry in detail["levels"]:
            text = f"{entry['missing_p99']:.3f}, {entry['stretched']} str."
            if judge and entry["level"] > 0:
                bound, fine = judge(entry["level"], entry)
                judged += 1
                failed += not fine
                text += f" ({bound}{'' if fine else ' ✗'})"
            texts.append(text)
            if "placed" in entry:
                said = entry["placed"]
                counts = ", ".join(f"{k} {v}" for k, v in said.items() if k not in ("vertices", "seconds") and v)
                placed_lines.append(f"| {row} | L{entry['level']} | {said['vertices']} | {counts} |")
        lines.append(f"| {row} | " + " | ".join(texts) + " |")
    lines += ["", f"{judged - failed} of {judged} judged cells meet the milestone.", ""]
    lines += ["What became of the vertices.", "", "| row | level | vertices | statuses |", "|---|---|---|---|"]
    lines += placed_lines
    text = "\n".join(lines) + "\n"
    out = Path(options.out) if options.out else Path(options.records).parent
    (out / "report.md").write_text(text)
    print(text, end="")


if __name__ == "__main__":
    main()
