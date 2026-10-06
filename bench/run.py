#!/usr/bin/env python3
"""Runs every remesher on every model of the corpus and tabulates the metrics.

    python3 bench/run.py [--models a,b] [--tools a,b] [--target N] [--seeds 1,2,3]

Needs `blender` on the PATH (for QuadriFlow and the provisional corpus) and a
release build of `retopo`. Results go to bench/results/.
"""

import argparse
import json
import statistics
import subprocess
import sys
from pathlib import Path

BENCH = Path(__file__).resolve().parent
ROOT = BENCH.parent
RETOPO = ROOT / "target" / "release" / "retopo"
CORPUS = BENCH / "corpus"
OUTPUT = BENCH / "out"
RESULTS = BENCH / "results"
# Outputs of tools that cannot be re-run at will (a licence that expires).
REFERENCE = BENCH / "reference"

# name, category, has hard edges, has open boundaries
MODELS = [
    ("blob", "organic", False, False),
    ("rock", "organic", False, False),
    ("creature", "organic", False, False),
    ("cage", "organic", False, False),
    ("suzanne", "organic", False, True),
    # The project mascot: a real 4.7 million triangle sculpt, brought down to
    # 200 000 triangles by `retopo preprocess` (see bench/README.md).
    ("ogre", "organic", False, False),
    # From Keenan Crane's model repository (see bench/import_crane.py).
    ("spot", "organic", False, False),
    ("bob", "organic", False, False),
    ("blub", "organic", False, False),
    ("pig", "organic", False, False),
    ("jerry", "organic", False, True),
    ("rose", "organic", False, False),
    ("bridge", "organic", False, True),
    ("part", "mechanical", True, False),
    ("cube", "mechanical", True, False),
    ("sphere", "calibration", False, False),
    ("torus", "calibration", False, False),
]
PRIMITIVES = {"cube", "sphere", "torus"}
DEFAULT_TARGET = 3000
CREASE_ANGLE = "30"

# metric key, label, format, lower is better
METRICS = [
    ("irregular_vertices", "Irregular vertices (S)", "{:.0f}", True),
    ("irregular_excess", "… of which above the minimum", "{:.0f}", True),
    ("base_complex_patches", "Base complex patches (C)", "{:.0f}", True),
    ("alignment", "Alignment, degrees (A)", "{:.2f}", True),
    ("angle_deviation_mean", "Mean angle deviation, degrees (F)", "{:.2f}", True),
    ("angle_deviation_p95", "Angle deviation, 95th percentile (F)", "{:.2f}", True),
    ("aspect_mean", "Mean aspect ratio (F)", "{:.3f}", True),
    ("area_variation", "Area variation (F)", "{:.3f}", True),
    ("hausdorff", "Hausdorff, ‰ of the diagonal (H)", "{:.2f}", True),
    ("mean_distance", "Mean distance, ‰ (H)", "{:.3f}", True),
    ("budget_error", "Off budget, % (N)", "{:.1f}", True),
    ("seconds", "Time, seconds (T)", "{:.2f}", True),
]
SCALE = {"hausdorff": 1000.0, "mean_distance": 1000.0, "budget_error": 100.0}
# The metrics the acceptance rule is about.
QUALITY = ["irregular_vertices", "base_complex_patches", "alignment", "angle_deviation_mean", "aspect_mean"]


def run(command, timeout=1800):
    return subprocess.run([str(c) for c in command], capture_output=True, text=True, timeout=timeout)


def source_path(model):
    return CORPUS / f"{model}.obj"


def ensure_corpus(models):
    CORPUS.mkdir(exist_ok=True)
    for model in models:
        if model in PRIMITIVES and not source_path(model).exists():
            run([RETOPO, "primitive", model, source_path(model)])
    if any(not source_path(model).exists() for model in models):
        run(["blender", "-b", "--python", BENCH / "make_corpus.py", "--", CORPUS])
    missing = [model for model in models if not source_path(model).exists()]
    if missing:
        sys.exit(f"missing corpus models: {', '.join(missing)}")


def quadriflow(model, sharp, boundary, target, seeds, directory):
    """Returns {seed: (path or None, seconds, failure reason)}."""
    prefix = directory / "quadriflow"
    done = run(["blender", "-b", "--python", BENCH / "quadriflow.py", "--", source_path(model), prefix, target,
                int(sharp), int(boundary), *seeds])
    runs = {}
    for line in done.stdout.splitlines():
        if line.startswith("QUADRIFLOW "):
            _, seed, status, seconds = line.split()
            path = Path(f"{prefix}_{seed}.obj")
            runs[int(seed)] = (path if status == "ok" else None, float(seconds), None if status == "ok" else status)
    for seed in seeds:
        runs.setdefault(seed, (None, 0.0, "no output from Blender"))
    return runs


def quadremesher_reference(model, target, uniform):
    setting = "uniform" if uniform else "default"
    return REFERENCE / "quadremesher" / f"{model}__{target}__{setting}.obj"


def quadremesher(uniform, refresh=False):
    """Quad Remesher is deterministic (no seed) and needs a licence, so each
    result is computed once and kept in bench/reference; later runs read it
    back, with or without the software installed."""

    def tool(model, sharp, boundary, target, seeds, directory):
        path = quadremesher_reference(model, target, uniform)
        if path.exists() and not refresh:
            return {seeds[0]: (path, None, None)}
        path.parent.mkdir(parents=True, exist_ok=True)
        fresh = directory / path.name
        fresh.unlink(missing_ok=True)
        done = run(["blender", "-b", "--python", BENCH / "quadremesher.py", "--", source_path(model), fresh, target,
                    int(uniform), int(sharp)])
        line = next((l for l in done.stdout.splitlines() if l.startswith("QUADREMESHER ")), None)
        if line is None:
            return {seeds[0]: (None, None, "no output from Blender")}
        _, status, seconds = line.split()
        if status != "ok":
            return {seeds[0]: (None, None, status)}
        fresh.replace(path)
        path.with_suffix(".seconds").write_text(seconds + "\n")
        return {seeds[0]: (path, None, None)}

    def timed(*arguments):
        runs = tool(*arguments)
        for seed, (path, _, failure) in runs.items():
            record = path.with_suffix(".seconds") if path else None
            seconds = float(record.read_text()) if record and record.exists() else 0.0
            runs[seed] = (path, seconds, failure)
        return runs

    return timed


def retopo(method):
    def tool(model, sharp, boundary, target, seeds, directory):
        runs = {}
        for seed in seeds:
            path = directory / f"{method}_{seed}.obj"
            command = [RETOPO, "remesh", source_path(model), path, "--method", method, "--quads", target, "--seed", seed]
            if sharp:
                command += ["--crease-angle", CREASE_ANGLE]
            done = run(command)
            seconds = next((float(l.split()[1]) for l in done.stdout.splitlines() if l.startswith("seconds")), 0.0)
            failed = done.returncode != 0
            runs[seed] = (None if failed else path, seconds, done.stderr.strip().splitlines()[0] if failed else None)
        return runs

    return tool


# Every remesher under test. A new chain only needs a line here.
TOOLS = {
    "quadriflow": quadriflow,
    "instant-meshes": retopo("instant-meshes"),
    # Commercial yardstick; needs the add-on and a licence (see bench/README.md).
    "quadremesher": quadremesher(uniform=False),
    "quadremesher-uniform": quadremesher(uniform=True),
}


def measure(model, path, target):
    done = run([RETOPO, "measure", source_path(model), path, "--target", target, "--json"])
    if done.returncode != 0:
        return None
    return json.loads(done.stdout)


def median(values):
    values = [v for v in values if v is not None]
    return statistics.median(values) if values else None


def summarize(runs):
    measured = [r for r in runs if r.get("metrics")]
    summary = {
        "runs": len(runs),
        "produced": len(measured),
        "valid": sum(1 for r in measured if r["metrics"]["valid"]),
        "failures": sorted({r["failure"] for r in runs if r.get("failure")}),
    }
    for key, *_ in METRICS:
        values = [r["seconds"] if key == "seconds" else r["metrics"].get(key) for r in measured]
        summary[key] = median(values)
    return summary


def cell(summary, key, pattern):
    value = summary.get(key)
    return "—" if value is None else pattern.format(value * SCALE.get(key, 1.0))


def report(results, target, seeds, tools):
    lines = [
        "# Bench results",
        "",
        f"Requested budget: {target} quads. Median over {len(seeds)} seeds; Quad Remesher has no seed and "
        "runs once. Lower is better for every metric; \"—\" means it could not be measured.",
        "",
    ]
    for model, category, *_ in MODELS:
        if model not in results:
            continue
        lines += [f"## {model} ({category})", "", "| | " + " | ".join(tools) + " |", "|---|" + "---|" * len(tools)]
        row = [f"{results[model][t]['valid']}/{results[model][t]['runs']}" for t in tools]
        lines.append("| **Valid meshes (V)** | " + " | ".join(row) + " |")
        for key, label, pattern, _ in METRICS:
            lines.append(f"| {label} | " + " | ".join(cell(results[model][t], key, pattern) for t in tools) + " |")
        failures = [f"{t}: {', '.join(results[model][t]['failures'])}" for t in tools if results[model][t]["failures"]]
        if failures:
            lines += ["", "Failures: " + "; ".join(failures) + "."]
        lines.append("")

    if len(tools) >= 2:
        lines += ["## Summary", "", "Number of models on which each tool has the best median, calibration "
                  "shapes excluded. Ties count for each.", ""]
        for category in ("organic", "mechanical"):
            names = [m for m, c, *_ in MODELS if c == category and m in results]
            if not names:
                continue
            lines += [f"### {category} ({len(names)} models)", "", "| Metric | " + " | ".join(tools) + " |",
                      "|---|" + "---|" * len(tools)]
            for key in QUALITY:
                label = next(l for k, l, *_ in METRICS if k == key)
                wins = dict.fromkeys(tools, 0)
                for model in names:
                    values = {t: results[model][t].get(key) for t in tools}
                    known = [v for v in values.values() if v is not None]
                    for t in tools:
                        if known and values[t] == min(known):
                            wins[t] += 1
                lines.append(f"| {label} | " + " | ".join(str(wins[t]) for t in tools) + " |")
            valid = [f"{sum(results[m][t]['valid'] for m in names)}/{sum(results[m][t]['runs'] for m in names)}" for t in tools]
            lines += ["| Valid meshes (V) | " + " | ".join(valid) + " |", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--models", default=",".join(m[0] for m in MODELS))
    parser.add_argument("--tools", default=",".join(TOOLS))
    parser.add_argument("--target", type=int, default=DEFAULT_TARGET)
    parser.add_argument("--seeds", default="1,2,3,4,5")
    options = parser.parse_args()
    models = options.models.split(",")
    tools = options.tools.split(",")
    seeds = [int(s) for s in options.seeds.split(",")]

    if not RETOPO.exists():
        sys.exit("build retopo first: cargo build --release")
    ensure_corpus(models)
    RESULTS.mkdir(exist_ok=True)

    # One pair of files per budget; a partial run (some models, some tools)
    # updates them in place instead of discarding what was measured before.
    suffix = "" if options.target == DEFAULT_TARGET else f"-{options.target}"
    data_path, report_path = RESULTS / f"results{suffix}.json", RESULTS / f"RESULTS{suffix}.md"
    raw = {}
    if data_path.exists():
        previous = json.loads(data_path.read_text())
        if previous.get("seeds") == seeds:
            raw = previous["runs"]

    for model, _, sharp, boundary in MODELS:
        if model not in models:
            continue
        directory = OUTPUT / model
        directory.mkdir(parents=True, exist_ok=True)
        raw.setdefault(model, {})
        for tool in tools:
            runs = []
            for seed, (path, seconds, failure) in sorted(TOOLS[tool](model, sharp, boundary, options.target, seeds, directory).items()):
                metrics = measure(model, path, options.target) if path else None
                runs.append({"seed": seed, "seconds": seconds, "failure": failure, "metrics": metrics})
            raw[model][tool] = runs
            summary = summarize(runs)
            print(f"{model:10} {tool:15} valid {summary['valid']}/{summary['runs']}  "
                  f"irregular {cell(summary, 'irregular_vertices', '{:.0f}'):>5}  "
                  f"patches {cell(summary, 'base_complex_patches', '{:.0f}'):>6}  "
                  f"alignment {cell(summary, 'alignment', '{:.2f}'):>6}", flush=True)

    # Report every tool that has results, in the order they are declared.
    reported = [t for t in TOOLS if all(t in by_tool for by_tool in raw.values())]
    results = {model: {t: summarize(raw[model][t]) for t in reported} for model in raw}
    data_path.write_text(json.dumps({"target": options.target, "seeds": seeds, "runs": raw, "summary": results}, indent=1))
    report_path.write_text(report(results, options.target, seeds, reported))
    print(f"\nwritten: {report_path}")


if __name__ == "__main__":
    main()
