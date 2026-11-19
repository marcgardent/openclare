"""The report of the batch multires-advice (milestone 5 of the Multires
specification): what the engine advises after each level, as Add Level of
the PRO edition asks it, judged against the acceptance criteria of the
milestone, from the records lab wrote for its cells and the detail.json and
advice-<N>.json each job left in its folder.

    python3 bench/multires_advice_report.py <records.json> [--out DIR]

The criteria, with the default target (missing p99 below 0.05 % of the
diagonal, decision 2):

- creature says "done" at L2 and not before;
- rock says "done" at L3 and not before;
- ogre: at every level, the cage faces told to want more quads ("denser")
  cover at most 5 % of the cage's area (decision 4).

Writes report.md into `--out` (the folder of records.json by default) and
prints it.
"""

import argparse
import json
from pathlib import Path

DONE_AT = {"creature": 2, "rock": 3}
DENSER_AREA = 0.05


def cage_areas(path):
    """The area of each face of the OBJ at `path`, in its order."""
    positions, areas = [], []
    for line in Path(path).read_text().splitlines():
        fields = line.split()
        if not fields:
            continue
        if fields[0] == "v":
            positions.append(tuple(float(x) for x in fields[1:4]))
        elif fields[0] == "f":
            corners = [positions[int(f.split("/")[0]) - 1] for f in fields[1:]]
            # Half the cross product of the diagonals: the area of a planar
            # quad, close to it for one that is not.
            if len(corners) == 4:
                a, b, c, d = corners
                u = [c[i] - a[i] for i in range(3)]
                v = [d[i] - b[i] for i in range(3)]
            else:
                a, b, c = corners[:3]
                u = [b[i] - a[i] for i in range(3)]
                v = [c[i] - a[i] for i in range(3)]
            cross = (u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0])
            areas.append(0.5 * sum(x * x for x in cross) ** 0.5)
    return areas


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records")
    parser.add_argument("--out")
    options = parser.parse_args()
    records = json.loads(Path(options.records).read_text())

    lines = [
        f"# {records['batch']}: the advice of each level against milestone 5",
        "",
        "Per level: missing detail p99 (% of the diagonal), the advice, and the cage faces told to want "
        "more quads with their share of the cage's area; `✗` where the milestone is not met.",
        "",
        "| row | L1 | L2 | L3 | L4 | verdict |",
        "|---|---|---|---|---|---|",
    ]
    judged = failed = 0
    for cell in records["cells"]:
        row = cell["values"]["row"]
        model = row.split("-")[0]
        detail = None
        if cell.get("status") == "ok":
            try:
                detail = json.loads((Path(cell["out"]) / "detail.json").read_text())
            except (OSError, ValueError):
                detail = None
        if detail is None:
            lines.append(f"| {row} | " + " | ".join([cell.get("status") or "never run"] * 5) + " |")
            continue
        areas = cage_areas(detail["cage"])
        total = sum(areas)
        texts, fine = [], True
        for entry in detail["levels"][1:]:
            level = entry["level"]
            verdict = json.loads((Path(cell["out"]) / f"advice-{level}.json").read_text())["verdict"]
            share = sum(a for a, v in zip(areas, verdict) if v == 2) / total
            text = f"{entry['missing_p99']:.3f}, {entry['advice']}, {entry['denser_faces']} ({share:.1%})"
            if model in DONE_AT and (entry["advice"] == "done") != (level >= DONE_AT[model]):
                fine, text = False, text + " ✗"
            if model == "ogre" and share > DENSER_AREA:
                fine, text = False, text + " ✗"
            texts.append(text)
        if model in DONE_AT or model == "ogre":
            judged += 1
            failed += not fine
            criterion = f"done at L{DONE_AT[model]}" if model in DONE_AT else f"denser ≤ {DENSER_AREA:.0%} of the area"
            texts.append(f"{criterion}: {'met' if fine else 'not met'}")
        else:
            texts.append("not judged")
        lines.append(f"| {row} | " + " | ".join(texts) + " |")
    lines += ["", f"{judged - failed} of {judged} judged rows meet the milestone."]
    text = "\n".join(lines) + "\n"
    out = Path(options.out) if options.out else Path(options.records).parent
    (out / "report.md").write_text(text)
    print(text, end="")


if __name__ == "__main__":
    main()
