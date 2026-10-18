#!/usr/bin/env python3
"""Checks the acceptance thresholds of ALGORITHM.md against the bench results.

    python3 bench/acceptance.py [--tool openclare] [--targets 1000,3000,8000]

Reads bench/results/results*.json (see run.py) and writes
bench/results/ACCEPTANCE.md: for the organic models, then for the mechanical
ones, medians of every tool, then, model by model, which thresholds the tool
under test meets against QuadriFlow.
"""

import argparse
import json
import statistics

from run import DEFAULT_TARGET, MODELS, RESULTS

# Desert Rose is not a manifold surface: it waits for the repair stage.
GROUPS = [
    ("organic", [name for name, category, _, _ in MODELS if category == "organic" and name != "rose"]),
    ("mechanical", [name for name, category, _, _ in MODELS if category == "mechanical"]),
]
TOOLS = [
    "quadriflow",
    "autoremesher",
    "instant-meshes",
    "integer-grid",
    "openclare-uniform",
    "openclare",
]
# key, letter, label, factor applied for display
METRICS = [
    ("irregular_vertices", "S", "Irregular vertices", 1),
    ("base_complex_patches", "C", "Base complex patches", 1),
    ("alignment", "A", "Alignment, degrees", 1),
    ("angle_deviation_mean", "F", "Mean angle deviation, degrees", 1),
    ("hausdorff", "H", "Hausdorff, per mille of the diagonal", 1000),
    ("budget_error", "N", "Off budget, %", 100),
    ("area_variation", "", "Area variation", 1),
    ("mean_distance", "", "Mean distance, per mille of the diagonal", 1000),
]


def median(values):
    values = [v for v in values if v is not None]
    return statistics.median(values) if values else None


def of_model(runs, model, tool):
    """Median of each metric over the seeds of one tool on one model, with
    how many of its runs gave a valid mesh."""
    entries = runs.get(model, {}).get(tool) or []
    if not entries:
        return None
    measured = [entry["metrics"] for entry in entries if entry["metrics"]]
    row = {key: median([m.get(key) for m in measured]) for key, *_ in METRICS}
    row["valid"] = sum(1 for m in measured if m.get("valid"))
    row["runs"] = len(entries)
    row["seconds"] = median([entry["seconds"] for entry in entries])
    return row


def thresholds(ours, theirs):
    """Which acceptance thresholds `ours` meets against QuadriFlow's `theirs`.
    A metric QuadriFlow has no value for cannot be lost."""

    def against(key, test):
        return theirs[key] is None or (ours[key] is not None and test(ours[key], theirs[key]))

    return {
        "V": ours["valid"] == ours["runs"],
        "A": against("alignment", lambda a, b: b >= 20 or a <= b - 2),
        "S": against("irregular_vertices", lambda a, b: a <= 1.5 * b),
        "C": against("base_complex_patches", lambda a, b: a <= b),
        "F": against("angle_deviation_mean", lambda a, b: a <= b),
        "H": against("hausdorff", lambda a, b: a <= 1.2 * b),
        "N": ours["budget_error"] is not None and ours["budget_error"] <= 0.05,
    }


def show(value, factor=1, digits=1):
    return "—" if value is None else f"{factor * value:.{digits}f}"


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--tool", default="openclare", help="the chain under test")
    parser.add_argument("--targets", default="1000,3000,8000")
    args = parser.parse_args()

    lines = [
        "# Acceptance",
        "",
        f"Chain under test: `{args.tool}`.",
        "The thresholds are those of ALGORITHM.md, section 0, against QuadriFlow at the same budget.",
        "They were set for the organic models; the mechanical ones are held to the same for reference.",
        "",
    ]
    for target in [int(t) for t in args.targets.split(",")]:
        suffix = "" if target == DEFAULT_TARGET else f"-{target}"
        path = RESULTS / f"results{suffix}.json"
        if not path.exists():
            continue
        runs = json.loads(path.read_text())["runs"]
        for group, models in GROUPS:
            rows = {tool: {m: of_model(runs, m, tool) for m in models} for tool in TOOLS}
            present = [tool for tool in TOOLS if any(rows[tool].values())]
            if not present:
                continue

            lines += [
                f"## {target} quads, {group} models",
                "",
                f"Models: {', '.join(models)}. Medians over those each tool was run on.",
                "",
            ]
            lines += ["| | " + " | ".join(present) + " |", "|---|" + "---|" * len(present)]
            counted = {tool: [row for row in rows[tool].values() if row] for tool in present}
            lines.append(
                "| Models run | " + " | ".join(str(len(counted[tool])) for tool in present) + " |"
            )
            lines.append(
                "| Models valid on every run (V) | "
                + " | ".join(str(sum(1 for row in counted[tool] if row["valid"] == row["runs"])) for tool in present)
                + " |"
            )
            for key, letter, label, factor in METRICS:
                digits = 0 if key in ("irregular_vertices", "base_complex_patches") else 1
                digits = 2 if key in ("area_variation", "mean_distance") else digits
                cells = [show(median([row[key] for row in counted[tool]]), factor, digits) for tool in present]
                lines.append(f"| {label}{f' ({letter})' if letter else ''} | " + " | ".join(cells) + " |")
            lines.append(
                "| Time, seconds (T) | "
                + " | ".join(show(median([row["seconds"] for row in counted[tool]]), 1, 1) for tool in present)
                + " |"
            )
            lines.append("")

            if args.tool not in present or "quadriflow" not in present:
                lines += ["No model by model check: one of the two tools has no result at this budget.", ""]
                continue
            lines += [
                "Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.",
                "",
                "| Model | V | A | S | C | F | H | N | Missed |",
                "|---|---|---|---|---|---|---|---|---|",
            ]
            met = {letter: 0 for letter in "VASCFHN"}
            compared = 0
            for model in models:
                ours, theirs = rows[args.tool][model], rows["quadriflow"][model]
                if not ours or not theirs:
                    continue
                compared += 1
                verdict = thresholds(ours, theirs)
                for letter, ok in verdict.items():
                    met[letter] += ok
                pair = lambda key, factor=1, digits=1: f"{show(ours[key], factor, digits)} / {show(theirs[key], factor, digits)}"
                lines.append(
                    f"| {model} | {ours['valid']}/{ours['runs']} / {theirs['valid']}/{theirs['runs']} | {pair('alignment')} "
                    f"| {pair('irregular_vertices', 1, 0)} | {pair('base_complex_patches', 1, 0)} "
                    f"| {pair('angle_deviation_mean')} | {pair('hausdorff', 1000)} | {show(ours['budget_error'], 100)} % "
                    f"| {''.join(letter for letter, ok in verdict.items() if not ok) or '—'} |"
                )
            lines += [
                "",
                "Thresholds met, out of "
                + f"{compared} models: "
                + ", ".join(f"{letter} {count}" for letter, count in met.items())
                + ".",
                "",
            ]
    (RESULTS / "ACCEPTANCE.md").write_text("\n".join(lines))
    print("\n".join(lines))


if __name__ == "__main__":
    main()
