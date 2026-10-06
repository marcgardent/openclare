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

# name, category, has hard edges, has open boundaries
MODELS = [
    ("blob", "organique", False, False),
    ("rocher", "organique", False, False),
    ("creature", "organique", False, False),
    ("cage", "organique", False, False),
    ("suzanne", "organique", False, True),
    ("piece", "mécanique", True, False),
    ("cube", "mécanique", True, False),
    ("sphere", "étalon", False, False),
    ("torus", "étalon", False, False),
]
PRIMITIVES = {"cube", "sphere", "torus"}
CREASE_ANGLE = "30"

# metric key, label, format, lower is better
METRICS = [
    ("irregular_vertices", "Sommets irréguliers (S)", "{:.0f}", True),
    ("irregular_excess", "… dont en excès du minimum", "{:.0f}", True),
    ("base_complex_patches", "Patchs du complexe de base (C)", "{:.0f}", True),
    ("alignment", "Alignement, degrés (A)", "{:.2f}", True),
    ("angle_deviation_mean", "Écart d'angle moyen, degrés (F)", "{:.2f}", True),
    ("angle_deviation_p95", "Écart d'angle, 95ᵉ centile (F)", "{:.2f}", True),
    ("aspect_mean", "Rapport d'aspect moyen (F)", "{:.3f}", True),
    ("area_variation", "Dispersion des aires (F)", "{:.3f}", True),
    ("hausdorff", "Hausdorff, ‰ de la diagonale (H)", "{:.2f}", True),
    ("mean_distance", "Distance moyenne, ‰ (H)", "{:.3f}", True),
    ("budget_error", "Écart au budget, % (N)", "{:.1f}", True),
    ("seconds", "Temps, secondes (T)", "{:.2f}", True),
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
        "# Résultats du banc de mesure",
        "",
        f"Budget demandé : {target} quads. Médiane sur {len(seeds)} graines. "
        "Pour toutes les métriques, plus bas est meilleur ; « — » signifie non mesurable.",
        "",
    ]
    for model, category, *_ in MODELS:
        if model not in results:
            continue
        lines += [f"## {model} ({category})", "", "| | " + " | ".join(tools) + " |", "|---|" + "---|" * len(tools)]
        row = [f"{results[model][t]['valid']}/{results[model][t]['runs']}" for t in tools]
        lines.append("| **Maillages valides (V)** | " + " | ".join(row) + " |")
        for key, label, pattern, _ in METRICS:
            lines.append(f"| {label} | " + " | ".join(cell(results[model][t], key, pattern) for t in tools) + " |")
        failures = [f"{t} : {', '.join(results[model][t]['failures'])}" for t in tools if results[model][t]["failures"]]
        if failures:
            lines += ["", "Échecs : " + " ; ".join(failures) + "."]
        lines.append("")

    if len(tools) == 2:
        first, second = tools
        lines += ["## Synthèse", "", f"Nombre de modèles où chaque outil a la meilleure médiane ({first} / {second}).", "",
                  "| Métrique | organique | mécanique |", "|---|---|---|"]
        for key in QUALITY:
            label = next(l for k, l, *_ in METRICS if k == key)
            counts = {}
            for model, category, *_ in MODELS:
                if model not in results or category == "étalon":
                    continue
                a, b = results[model][first].get(key), results[model][second].get(key)
                if a is None or b is None or a == b:
                    continue
                score = counts.setdefault(category, [0, 0])
                score[0 if a < b else 1] += 1
            columns = ["{} / {}".format(*counts.get(c, [0, 0])) for c in ("organique", "mécanique")]
            lines.append(f"| {label} | " + " | ".join(columns) + " |")
        lines.append("")
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--models", default=",".join(m[0] for m in MODELS))
    parser.add_argument("--tools", default=",".join(TOOLS))
    parser.add_argument("--target", type=int, default=3000)
    parser.add_argument("--seeds", default="1,2,3,4,5")
    options = parser.parse_args()
    models = options.models.split(",")
    tools = options.tools.split(",")
    seeds = [int(s) for s in options.seeds.split(",")]

    if not RETOPO.exists():
        sys.exit("build retopo first: cargo build --release")
    ensure_corpus(models)
    RESULTS.mkdir(exist_ok=True)

    raw, results = {}, {}
    for model, _, sharp, boundary in MODELS:
        if model not in models:
            continue
        directory = OUTPUT / model
        directory.mkdir(parents=True, exist_ok=True)
        raw[model], results[model] = {}, {}
        for tool in tools:
            runs = []
            for seed, (path, seconds, failure) in sorted(TOOLS[tool](model, sharp, boundary, options.target, seeds, directory).items()):
                metrics = measure(model, path, options.target) if path else None
                runs.append({"seed": seed, "seconds": seconds, "failure": failure, "metrics": metrics})
            raw[model][tool] = runs
            results[model][tool] = summarize(runs)
            summary = results[model][tool]
            print(f"{model:10} {tool:15} valid {summary['valid']}/{summary['runs']}  "
                  f"irregular {cell(summary, 'irregular_vertices', '{:.0f}'):>5}  "
                  f"patches {cell(summary, 'base_complex_patches', '{:.0f}'):>6}  "
                  f"alignment {cell(summary, 'alignment', '{:.2f}'):>6}", flush=True)

    (RESULTS / "results.json").write_text(json.dumps({"target": options.target, "seeds": seeds, "runs": raw, "summary": results}, indent=1))
    (RESULTS / "RESULTATS.md").write_text(report(results, options.target, seeds, tools))
    print(f"\nwritten: {RESULTS / 'RESULTATS.md'}")


if __name__ == "__main__":
    main()
