#!/usr/bin/env python3
"""Scores from 0 to 100, per criterion and overall, for every model and remesher.

    python3 scripts/scores.py                      write the report (bench/results/SCORES.md)
    python3 scripts/scores.py model ogre           the scores of one model, every remesher
    python3 scripts/scores.py model ogre --target 8000 --tool openclare
    python3 scripts/scores.py mesh source.obj quads.obj --target 3000
                                                   the scores of any quad mesh against its source

The first two read what the bench measured (bench/results/results*.json, see
bench/run.py); the last one measures the mesh with `openclare measure`.

Every score is an absolute mark, not a rank: a remesher keeps its score
whichever others it is compared with. `SCALES` below says how each metric
becomes a mark, and `WEIGHTS` what each criterion counts for in the overall
score.
"""

import argparse
import json
import math
import statistics
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "bench"))
from run import DEFAULT_TARGET, MATCHED, MODELS, RESULTS, OPENCLARE  # noqa: E402

# Criteria, in the order they are shown: key, title, what a full mark means.
CRITERIA = [
    ("validity", "Validity", "all quads, manifold, no flipped or collapsed face, no hole the source lacks"),
    ("alignment", "Alignment", "edges along the principal curvature directions"),
    ("shape", "Quad shape", "corners at right angles"),
    ("fidelity", "Fidelity", "no point of the source far from the mesh, thin parts kept"),
    ("singularities", "Regularity", "no irregular vertex beyond what the topology imposes"),
    ("structure", "Structure", "few patches in the base complex: edge loops that run long"),
    ("loops", "Edge loops", "loops that close round the shape, none that spiral round it"),
    ("budget", "Budget", "the number of quads asked for"),
    ("speed", "Speed", "an answer within a second"),
]
# What each criterion counts for in the overall score, out of 100. Speed is
# shown but is not a matter of quality: it counts for nothing.
WEIGHTS = {
    "validity": 25,
    "alignment": 20,
    "shape": 15,
    "fidelity": 15,
    "singularities": 10,
    "structure": 5,
    "loops": 10,
    "budget": 10,
    "speed": 0,
}


def linear(value, full, none):
    """100 at `full`, 0 at `none`, straight in between."""
    return max(0.0, min(100.0, 100.0 * (none - value) / (none - full)))


def fidelity(metrics):
    """The Hausdorff distance, as a share of the diagonal of the bounding box;
    and, where the source has parts thinner than a quad (fingers, ears), how
    far those are from the mesh on average, in quad edges: half a quad edge
    away, they are as good as gone. The two count the same."""
    whole = linear(metrics["hausdorff"], 0.0, 0.05)
    thin = metrics.get("thin_distance")
    return whole if thin is None else 0.5 * (whole + linear(thin, 0.0, 0.5))


def loops(metrics):
    """The edge loops (DT-062): none on a spiral, which is 0 from a fifth of
    the edges on spirals; and loops that close, which is 100 from half of
    them, those from a border to a border left out (an open sheet has
    nothing else). The two count the same. `None` where the loops could not
    be followed: a mesh that is not a manifold."""
    spiral = metrics.get("loop_edges_spiral")
    if spiral is None:
        return None
    no_spiral = linear(spiral, 0.0, 0.20)
    counted = sum(metrics.get(key) or 0 for key in ("loops_closed", "loops_spiral", "loops_open"))
    if counted == 0:
        return no_spiral
    closing = linear(-(metrics.get("loops_closed") or 0) / counted, -0.5, 0.0)
    return 0.5 * (no_spiral + closing)


def per_100_quads(count, metrics):
    return 100.0 * count / max(metrics["faces"], 1)


# How a criterion is marked from the metrics of a mesh (the median over the
# seeds of a remesher). `None` when the metric does not exist for the model.
SCALES = {
    # Edges at random are 22.5 degrees off on average.
    "alignment": lambda m: None if m.get("alignment") is None else linear(m["alignment"], 0.0, 22.5),
    # Mean distance of the corners from a right angle, in degrees.
    "shape": lambda m: linear(m["angle_deviation_mean"], 0.0, 20.0),
    "fidelity": lambda m: fidelity(m),
    # Irregular vertices beyond the topological minimum (or all of them
    # where that minimum is not known), per 100 quads.
    "singularities": lambda m: linear(
        per_100_quads(m["irregular_excess"] if m.get("irregular_excess") is not None else m["irregular_vertices"], m),
        0.0,
        10.0,
    ),
    # Patches of the base complex per quad: 1 means no structure at all.
    "structure": lambda m: (
        None if m.get("base_complex_patches") is None else linear(m["base_complex_patches"] / max(m["faces"], 1), 0.0, 1.0)
    ),
    "loops": lambda m: loops(m),
    # Distance to the number of quads asked for, as a share of it.
    "budget": lambda m: None if m.get("budget_error") is None else linear(m["budget_error"], 0.0, 0.25),
}


def speed_score(seconds):
    """100 within a second, 0 at a minute, by the logarithm in between."""
    if seconds is None or seconds <= 0:
        return None
    return max(0.0, min(100.0, 100.0 * (1.0 - math.log(max(seconds, 1.0)) / math.log(60.0))))


def median(values):
    values = [v for v in values if v is not None]
    return statistics.median(values) if values else None


def scores_of(measured, runs, seconds, budget_counts=True):
    """Marks of a remesher on a model. `measured` holds the metrics of the
    runs that gave a mesh, `runs` is how many were launched. Without
    `budget_counts`, the number of quads is left out of the overall mark."""
    marks = {key: None for key, *_ in CRITERIA}
    quads = None
    if runs == 0:
        return None
    marks["validity"] = 100.0 * sum(1 for m in measured if m.get("valid")) / runs
    if measured:
        keys = set().union(*(m.keys() for m in measured))
        typical = {key: median([m.get(key) for m in measured]) for key in keys}
        if typical.get("faces"):
            for key, scale in SCALES.items():
                marks[key] = scale(typical)
        quads = typical.get("faces")
    marks["speed"] = speed_score(seconds)
    # Overall: the weighted mean of the criteria that exist for this model.
    # A remesher that gave no mesh at all has nothing but its validity: 0.
    counted = [
        (marks[key], WEIGHTS[key])
        for key in marks
        if marks[key] is not None and WEIGHTS[key] > 0 and (budget_counts or key != "budget")
    ]
    weight = sum(w for _, w in counted)
    marks["overall"] = sum(mark * w for mark, w in counted) / weight if measured and weight else 0.0
    # Not a mark: how many quads the mesh has, to read the marks by.
    marks["quads"] = quads
    return marks


def results_path(target):
    return RESULTS / ("results.json" if target == DEFAULT_TARGET else f"results-{target}.json")


def load(target):
    """{model: {tool: marks}} for one budget, from the bench results."""
    path = results_path(target)
    if not path.exists():
        return {}
    table = {}
    for model, tools in json.loads(path.read_text())["runs"].items():
        for tool, runs in tools.items():
            measured = [run["metrics"] for run in runs if run.get("metrics")]
            seconds = median([run.get("seconds") for run in runs])
            marks = scores_of(measured, len(runs), seconds, budget_counts=tool not in MATCHED)
            if marks:
                table.setdefault(model, {})[tool] = marks
    return table


def show(mark):
    return "—" if mark is None else f"{mark:.0f}"


def rows_of(tools, bold=None):
    """Markdown rows: one remesher per line, overall first, then each criterion."""
    lines = [
        "| Remesher | Quads | Overall | " + " | ".join(title for _, title, _ in CRITERIA) + " |",
        "|---|---|---|" + "---|" * len(CRITERIA),
    ]
    for tool, marks in sorted(tools.items(), key=lambda item: -item[1]["overall"]):
        name = f"**{tool}**" if tool == bold else tool
        cells = [show(marks[key]) for key, *_ in CRITERIA]
        lines.append(f"| {name} | {show(marks.get('quads'))} | **{marks['overall']:.0f}** | " + " | ".join(cells) + " |")
    return lines


def without_matched(tools):
    """The remeshers run at the budget of the bench."""
    return {tool: marks for tool, marks in tools.items() if tool not in MATCHED}


def matched_rows(table, models):
    """Markdown rows: one model per line, Quad Remesher as it is against
    openclare at the same number of quads, as `theirs / ours`."""
    theirs, ours = MATCHED
    shown = [("quads", "Quads"), ("overall", "Overall")] + [(key, title) for key, title, _ in CRITERIA if WEIGHTS[key] > 0 and key != "budget"]
    lines = ["| Model | " + " | ".join(title for _, title in shown) + " |", "|---|" + "---|" * len(shown)]
    for model in models:
        pair = table.get(model, {})
        if theirs not in pair or ours not in pair:
            continue
        cells = [f"{show(pair[theirs].get(key))} / {show(pair[ours].get(key))}" for key, _ in shown]
        lines.append(f"| {model} | " + " | ".join(cells) + " |")
    return lines


def text_table(tools):
    """The same as plain text, for the terminal."""
    width = max(len(tool) for tool in tools)
    header = f"{'':{width}}    quads  overall  " + "  ".join(f"{title[:10]:>10}" for _, title, _ in CRITERIA)
    lines = [header]
    for tool, marks in sorted(tools.items(), key=lambda item: -item[1]["overall"]):
        cells = "  ".join(f"{show(marks[key]):>10}" for key, *_ in CRITERIA)
        lines.append(f"{tool:{width}}  {show(marks.get('quads')):>7}  {marks['overall']:7.0f}  {cells}")
    return "\n".join(lines)


def averages(table, models):
    """{tool: (marks averaged over `models`, how many of them it was run on)}."""
    out = {}
    tools = sorted({tool for model in models for tool in table.get(model, {})})
    for tool in tools:
        run_on = [table[model][tool] for model in models if tool in table.get(model, {})]
        marks = {}
        for key in [key for key, *_ in CRITERIA] + ["overall", "quads"]:
            values = [m[key] for m in run_on if m.get(key) is not None]
            marks[key] = sum(values) / len(values) if values else None
        out[tool] = (marks, len(run_on))
    return out


def write_report(targets):
    groups = []
    for category in ["organic", "mechanical", "calibration", "extra"]:
        names = [name for name, kind, *_ in MODELS if kind == category]
        if names:
            groups.append((category, names))

    lines = [
        "# Scores",
        "",
        "A mark from 0 to 100 for every remesher on every model of the bench, overall and per criterion.",
        "Generated by `scripts/scores.py` (`make scores`) from the measurements of `bench/run.py`.",
        "",
        "## How the marks are made",
        "",
        "Marks are absolute: a remesher keeps its mark whichever others it is compared with.",
        "Where a remesher ran several times (one per seed), the median of each metric is marked.",
        "",
        "| Criterion | Full marks for | 100 at | 0 at | Weight |",
        "|---|---|---|---|---|",
        "| Validity | " + CRITERIA[0][2] + " | every run valid | no run valid | 25 |",
        "| Alignment | " + CRITERIA[1][2] + " | 0° off | 22.5° off (edges at random) | 20 |",
        "| Quad shape | " + CRITERIA[2][2] + " | 0° mean deviation | 20° | 15 |",
        "| Fidelity | " + CRITERIA[3][2] + " | Hausdorff distance 0; thin parts on the mesh | 5 % of the bounding diagonal; "
        "thin parts half a quad edge away | 15 |",
        "| Regularity | " + CRITERIA[4][2] + " | none | 10 per 100 quads | 10 |",
        "| Structure | " + CRITERIA[5][2] + " | — | one patch per quad | 5 |",
        "| Budget | " + CRITERIA[6][2] + " | exact | 25 % off | 10 |",
        "| Speed | " + CRITERIA[7][2] + " | 1 s | 60 s (logarithmic) | 0 |",
        "",
        "The *Quads* column is not a mark: it is how many quads the mesh has (the mean over the models, in",
        "the averages). Quad Remesher, left to itself, makes about twice the quads it is asked for, and its",
        "Budget mark says so. To compare at equal quad count, two more rows are set apart under each",
        "budget: `quadremesher-as-made` is the mesh of `quadremesher` marked against the number of quads",
        "it has, and `openclare-matched` is `openclare` asked for that same number.",
        "Budget does not count there.",
        "",
        "Fidelity is two marks that count the same where the source has parts thinner than a quad edge",
        "(fingers, ears): one for the Hausdorff distance, one for how far those parts are from the mesh",
        "on average, in quad edges. Elsewhere it is the first alone.",
        "",
        "The overall mark is the weighted mean of the criteria that exist for the model (a sphere has no",
        "direction to align with; a mesh with open borders has no topological minimum). Speed is shown but",
        "does not count: it is not a matter of quality. A remesher that gave no mesh at all scores 0.",
        "",
        "The *extra* group is outside the acceptance rule. Its models from BlenderKit cannot be",
        "redistributed (see CREDITS.md): their marks are here, the models are not. Desert Rose, the two",
        "game characters and the car are not clean surfaces; no remesher is expected to do well on them",
        "yet. The car is run as it is in its file, torn along its sharp edges by an Edge Split modifier.",
        "",
    ]
    data = {}
    for target in targets:
        table = load(target)
        if not table:
            continue
        data[str(target)] = table
        lines += [f"## {target} quads", ""]
        for category, names in groups:
            present = [name for name in names if name in table]
            if not present:
                continue
            lines += [f"### {category.capitalize()} models: average", ""]
            lines += [f"Over {', '.join(present)}.", ""]
            mean = averages(table, present)
            lines += rows_of(without_matched({tool: marks for tool, (marks, _) in mean.items()}))
            partial = [f"{tool} ({count} of {len(present)})" for tool, (_, count) in mean.items() if count < len(present)]
            if partial:
                lines += ["", "Not run on every model: " + ", ".join(partial) + "."]
            lines.append("")
        for category, names in groups:
            present = [name for name in names if all(tool in table.get(name, {}) for tool in MATCHED)]
            if not present:
                continue
            lines += [f"### {category.capitalize()} models at equal quad count", ""]
            lines += ["Quad Remesher as it is / openclare asked for as many quads; the last line is the average.", ""]
            rows = matched_rows(table, present)
            mean = {tool: marks for tool, (marks, _) in averages(table, present).items()}
            rows += matched_rows({"**average**": mean}, ["**average**"])[2:]
            lines += rows + [""]
        lines += [f"### Model by model, {target} quads", ""]
        for category, names in groups:
            for name in names:
                if name not in table:
                    continue
                lines += [f"#### {name} ({category})", ""]
                lines += rows_of(without_matched(table[name]))
                lines.append("")
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "SCORES.md").write_text("\n".join(lines))
    (RESULTS / "scores.json").write_text(json.dumps(data, indent=1, sort_keys=True))
    return data, groups


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command")
    report = commands.add_parser("report", help="write bench/results/SCORES.md and scores.json (the default)")
    report.add_argument("--targets", default="1000,3000,8000")
    model = commands.add_parser("model", help="print the scores of one model")
    model.add_argument("name")
    model.add_argument("--target", type=int, default=DEFAULT_TARGET)
    model.add_argument("--tool", help="only this remesher")
    model.add_argument("--json", action="store_true", help="print JSON instead of a table")
    mesh = commands.add_parser("mesh", help="score any quad mesh against its source")
    mesh.add_argument("source")
    mesh.add_argument("quads")
    mesh.add_argument("--target", type=int, help="number of quads that was asked for")
    mesh.add_argument("--json", action="store_true")
    options = parser.parse_args()

    if options.command == "model":
        table = load(options.target).get(options.name)
        if not table:
            sys.exit(f"no measurement of '{options.name}' at {options.target} quads (see bench/run.py)")
        if options.tool:
            if options.tool not in table:
                sys.exit(f"'{options.tool}' was not run on '{options.name}' (run: {', '.join(table)})")
            table = {options.tool: table[options.tool]}
        if options.json:
            print(json.dumps(table, indent=1, sort_keys=True))
        else:
            print(f"{options.name}, {options.target} quads\n")
            print(text_table(table))
        return

    if options.command == "mesh":
        command = [str(OPENCLARE), "measure", options.source, options.quads, "--json"]
        if options.target:
            command += ["--target", str(options.target)]
        done = subprocess.run(command, capture_output=True, text=True)
        if done.returncode != 0:
            sys.exit(done.stderr.strip().splitlines()[0] if done.stderr.strip() else "openclare measure failed")
        marks = scores_of([json.loads(done.stdout)], 1, None)
        if options.json:
            print(json.dumps(marks, indent=1, sort_keys=True))
        else:
            print(text_table({Path(options.quads).name: marks}))
        return

    targets = [int(t) for t in getattr(options, "targets", "1000,3000,8000").split(",")]
    data, groups = write_report(targets)
    print(f"written: {RESULTS / 'SCORES.md'}")
    # The headline: organic models at the reference budget.
    table = data.get(str(DEFAULT_TARGET), {})
    organic = [name for name in dict(groups).get("organic", []) if name in table]
    if organic:
        print(f"\norganic models, {DEFAULT_TARGET} quads, average\n")
        print(text_table(without_matched({tool: marks for tool, (marks, _) in averages(table, organic).items()})))


if __name__ == "__main__":
    main()
