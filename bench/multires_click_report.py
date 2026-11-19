"""The report of the batch multires-click (milestone 4 of the Multires
specification): the seconds of one click of Add Level, level after level,
step by step, from the records lab wrote for its cells and the detail.json
each job left in its folder.

    python3 bench/multires_click_report.py <records.json> [--out DIR]

Writes report.md into `--out` (the folder of records.json by default) and
prints it.
"""

import argparse
import json
from pathlib import Path

STEPS = ["subdivide", "read", "source", "engine", "write", "total"]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records")
    parser.add_argument("--out")
    options = parser.parse_args()
    records = json.loads(Path(options.records).read_text())
    lines = [
        f"# {records['batch']}: one click of Add Level, in seconds",
        "",
        "| row | level | quads | " + " | ".join(STEPS) + " | missing p99 | stretched |",
        "|---|---|---|" + "---|" * (len(STEPS) + 2),
    ]
    for cell in records["cells"]:
        row = cell["values"]["row"]
        try:
            detail = json.loads((Path(cell["out"]) / "detail.json").read_text())
        except (OSError, ValueError, KeyError, TypeError):
            lines.append(f"| {row} | {cell.get('status') or 'never run'} |" + " |" * (len(STEPS) + 3))
            continue
        for entry in detail["levels"][1:]:
            click = entry.get("click", {})
            times = " | ".join(f"{click[step]:.3f}" if step in click else "" for step in STEPS)
            lines.append(
                f"| {row} | L{entry['level']} | {entry['faces']} | {times} | "
                f"{entry['missing_p99']:.3f} | {entry['stretched']} |"
            )
    text = "\n".join(lines) + "\n"
    out = Path(options.out) if options.out else Path(options.records).parent
    (out / "report.md").write_text(text)
    print(text, end="")


if __name__ == "__main__":
    main()
