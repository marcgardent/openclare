#!/usr/bin/env python3
"""The twelve models at three budgets, with the singularities of each mesh.

    python3 bench/budgets.py [--engine origin/main] [--budgets 2970,3000,3030] [--jobs 2]

The engine remeshes each model at each budget, and `openclare measure --json`
measures the result. By default the engine is the one built in this checkout
(target/release/openclare: `cargo build --release` first); `--engine <ref>`
builds that commit instead, from `git archive`, under scratch/budgets/engines/,
so that a branch and the main line are compared from the same checkout.

Next to the metrics, each run counts the vertices of the quad mesh by valence,
interior and border apart: the histogram of the singularities' indices, an
interior vertex of valence n having index 4 - n quarter turns. One run per
model and budget: a model's Hausdorff and spiral share are chaotic, so a
change is judged on the means over the three budgets, never on one run.

The sources are read from bench/out/<model>/source.obj of the main checkout,
which a worktree does not have.
"""

import argparse
import json
import subprocess
import time
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MODELS = ["creature", "blob", "suzanne", "spot", "pig", "ogre", "bob", "jerry", "blub", "rock", "cage", "part"]
HARD_SURFACE = {"cage", "part"}


def git(*arguments):
    return subprocess.run(["git", *arguments], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()


def home_checkout():
    return Path(git("rev-parse", "--path-format=absolute", "--git-common-dir")).parent


def engine_at(ref):
    """The command line tool built from commit `ref`, and that commit's short hash."""
    commit = git("rev-parse", "--short", ref)
    place = ROOT / "scratch" / "budgets" / "engines" / commit
    tool = place / "target" / "release" / "openclare"
    if not tool.exists():
        (place / "source").mkdir(parents=True, exist_ok=True)
        archive = subprocess.run(["git", "archive", commit, "Cargo.toml", "Cargo.lock", "crates"], cwd=ROOT,
                                 capture_output=True, check=True).stdout
        subprocess.run(["tar", "-x", "-C", place / "source"], input=archive, check=True)
        subprocess.run(["cargo", "build", "--release", "-p", "openclare-cli", "--manifest-path",
                        place / "source" / "Cargo.toml", "--target-dir", place / "target"], check=True)
    return tool, commit


def valences(path):
    """Interior and border vertices of an OBJ quad mesh, counted by valence."""
    faces = []
    for line in path.read_text().splitlines():
        if line.startswith("f "):
            faces.append([int(corner.split("/")[0]) for corner in line.split()[1:]])
    uses = Counter()
    for face in faces:
        for a, b in zip(face, face[1:] + face[:1]):
            uses[(min(a, b), max(a, b))] += 1
    degree, border = Counter(), set()
    for (a, b), count in uses.items():
        degree[a] += 1
        degree[b] += 1
        if count == 1:
            border.update((a, b))
    interior = Counter(n for v, n in degree.items() if v not in border)
    on_border = Counter(n for v, n in degree.items() if v in border)
    return {str(n): interior[n] for n in sorted(interior)}, {str(n): on_border[n] for n in sorted(on_border)}


def run(engine, sources, out, model, budget):
    source = sources / model / "source.obj"
    mesh = out / f"{model}-{budget}.obj"
    preset = "hard-surface" if model in HARD_SURFACE else "organic"
    record = {"model": model, "budget": budget}
    clock = time.monotonic()
    remesh = subprocess.run([engine, "remesh", source, mesh, "--preset", preset, "--quads", str(budget)],
                            capture_output=True, text=True, timeout=900)
    record["seconds"] = time.monotonic() - clock
    if remesh.returncode != 0:
        record["failure"] = remesh.stderr.strip().splitlines()[-1:] or ["no message"]
        return record
    measured = subprocess.run([engine, "measure", source, mesh, "--target", str(budget), "--json"],
                              capture_output=True, text=True, check=True)
    record["metrics"] = json.loads(measured.stdout)
    record["interior_valences"], record["border_valences"] = valences(mesh)
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--engine", help="a commit to build and run; default: the engine built here")
    parser.add_argument("--models", default=",".join(MODELS))
    parser.add_argument("--budgets", default="2970,3000,3030")
    parser.add_argument("--jobs", type=int, default=2, help="runs at a time")
    parser.add_argument("--sources", type=Path, help="default: bench/out of the main checkout")
    parser.add_argument("--out", type=Path, help="default: scratch/budgets/<commit>.json")
    options = parser.parse_args()
    if options.engine:
        engine, commit = engine_at(options.engine)
    else:
        engine, commit = ROOT / "target" / "release" / "openclare", git("rev-parse", "--short", "HEAD")
    sources = options.sources or home_checkout() / "bench" / "out"
    out = options.out or ROOT / "scratch" / "budgets" / f"{commit}.json"
    meshes = out.with_suffix("")
    meshes.mkdir(parents=True, exist_ok=True)
    work = [(model, int(budget)) for budget in options.budgets.split(",") for model in options.models.split(",")]
    with ThreadPoolExecutor(options.jobs) as pool:
        records = list(pool.map(lambda item: run(engine, sources, meshes, *item), work))
    for record in records:
        metrics = record.get("metrics")
        if not metrics:
            print(f"{record['model']:9} {record['budget']}  failed: {record['failure']}")
            continue
        print(f"{record['model']:9} {record['budget']}  valid {metrics['valid']!s:5}  "
              f"irregular {metrics['irregular_vertices']:4}  Hausdorff {1000 * metrics['hausdorff']:6.2f}  "
              f"interior valences {record['interior_valences']}")
    out.write_text(json.dumps({"commit": commit, "runs": records}, indent=1) + "\n")
    print(f"written to {out.relative_to(ROOT) if out.is_relative_to(ROOT) else out}")


if __name__ == "__main__":
    main()
