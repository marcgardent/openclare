#!/usr/bin/env python3
"""Ticket #115 before and after: the pole pairs joined, or left to the race.

    python3 bench/pole_pairs.py [--before main] [--parts tmesh,remesh,male-body] [--jobs 2]

The engine built in this checkout (target/release/openclare: `cargo build
--release` first) is measured against the one of `--before`, built from `git
archive` as bench/budgets.py does. Three parts:

- tmesh: `openclare tmesh --json` of the twelve models at `--tmesh-budgets`,
  with the pairs joined and with `--no-pairs`, on this checkout's engine: the
  patches, thin patches, short arcs and T-junctions quantization is given.
- remesh: bench/budgets.py on both engines at `--budgets`: validity,
  Hausdorff, mean distance, spiral share, irregular vertices and seconds,
  each a mean over the twelve models, budget by budget.
- male-body: the export of paper B (export.py) on both engines, then
  conflicts.py on each: the infeasible sets of held turns, of which the left
  wrist's was one arc 0.08 quad edges long (research note, section 7).

The sources are read from `--sources`, by default bench/out of the main
checkout, which lab's checkouts do not have: run it on a local checkout of
the main clone, or give the folder. Everything goes to
scratch/pole-pairs/<commit>/, the summary to scratch/pole-pairs/<commit>.json.
"""

import argparse
import json
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

import budgets

ROOT = budgets.ROOT
EXPERIMENTS = ROOT / "docs" / "research" / "turns-of-a-t-mesh" / "experiments"
COUNTS = ("patches", "thin_patches", "short_arcs", "t_junctions", "arcs", "pole_pairs_joined", "valid")
MEANS = ("hausdorff", "mean_distance", "loop_edges_spiral", "irregular_vertices")


def resolved(ref):
    """`ref`, else origin/main where the clone has it, else main (lab's mirror)."""
    for candidate in [ref] if ref else ["origin/main", "main"]:
        if subprocess.run(["git", "rev-parse", "--verify", "-q", candidate], cwd=ROOT, capture_output=True).returncode == 0:
            return candidate
    sys.exit(f"pole_pairs: no commit named {ref or 'origin/main or main'}")


def tmesh(engine, source, model, budget, joined):
    command = [str(engine), "tmesh", str(source), "--quads", str(budget), "--simplify", "2", "--json"]
    if model in budgets.HARD_SURFACE:
        command += ["--crease-angle", "30"]
    if not joined:
        command.append("--no-pairs")
    done = subprocess.run(command, capture_output=True, text=True, timeout=900)
    record = {"model": model, "budget": budget, "pairs": joined}
    if done.returncode != 0:
        record["failure"] = done.stderr.strip().splitlines()[-1:] or ["no message"]
        return record
    report = json.loads(done.stdout)
    record.update({key: report[key] for key in (*COUNTS, "seconds") if key in report})
    return record


def tmesh_table(records, models, budgets_):
    print("\nT-mesh, summed over the models: without the pairs -> with them")
    for budget in budgets_:
        sums = {}
        for joined in (False, True):
            rows = [r for r in records if r["budget"] == budget and r["pairs"] == joined and "failure" not in r]
            sums[joined] = {key: sum(int(r.get(key, 0)) for r in rows) for key in COUNTS}
            sums[joined]["runs"] = len(rows)
        print(f"  {budget:5}  " + ", ".join(f"{key} {sums[False][key]} -> {sums[True][key]}" for key in (*COUNTS, "runs")))
    failed = [f"{r['model']} {r['budget']}" for r in records if "failure" in r]
    if failed:
        print("  failed: " + ", ".join(failed))


def remesh_table(sides, budgets_):
    print("\nFinal mesh, means over the models: before -> after")
    for budget in budgets_:
        cells = []
        for key in (*MEANS, "seconds"):
            means = []
            for side in ("before", "after"):
                values = [r["seconds"] if key == "seconds" else r["metrics"][key]
                          for r in sides[side] if r["budget"] == budget and "metrics" in r]
                means.append(sum(values) / len(values) if values else float("nan"))
            cells.append(f"{key} {means[0]:.4g} -> {means[1]:.4g}")
        valid = [sum(1 for r in sides[side] if r["budget"] == budget and r.get("metrics", {}).get("valid"))
                 for side in ("before", "after")]
        print(f"  {budget:5}  valid {valid[0]} -> {valid[1]}, " + ", ".join(cells))


def male_body(engine, commit, source, out, threads):
    """The export on `engine`, then conflicts.py on it: the summary of conflicts.py."""
    export = out / "male-body"
    environment = {**os.environ, "RAYON_NUM_THREADS": str(threads), "HIGHS_THREADS": "1"}
    subprocess.run([sys.executable, EXPERIMENTS / "export.py", "--openclare", engine, "--commit", commit,
                    "--source", source, "--out", export], check=True, env=environment)
    summary = out / "conflicts.json"
    subprocess.run([sys.executable, EXPERIMENTS / "conflicts.py", export, "--out", out / "conflicts",
                    "--summary", summary], check=True, env=environment)
    return json.loads(summary.read_text())


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--before", default="", help="the commit to compare with; default origin/main, else main")
    parser.add_argument("--parts", default="tmesh,remesh,male-body")
    parser.add_argument("--models", default=",".join(budgets.MODELS))
    parser.add_argument("--tmesh-budgets", default="1000,3000,8000")
    parser.add_argument("--budgets", default="2970,3000,3030", help="of the final mesh, as bench/budgets.py")
    parser.add_argument("--jobs", type=int, default=2, help="runs at a time, one engine thread each")
    parser.add_argument("--sources", default="", help="default: bench/out of the main checkout")
    options = parser.parse_args()
    parts, models = options.parts.split(","), options.models.split(",")
    sources = Path(options.sources) if options.sources else budgets.home_checkout() / "bench" / "out"
    wanted = models + (["human_male"] if "male-body" in parts else [])
    missing = [model for model in wanted if not (sources / model / "source.obj").exists()]
    if missing:
        sys.exit(f"pole_pairs: no source.obj under {sources} for {', '.join(missing)}: "
                 "run on a local checkout of the main clone, or give --sources")

    after = ROOT / "target" / "release" / "openclare"
    commit = budgets.git("rev-parse", "--short", "HEAD")
    before, before_commit = budgets.engine_at(resolved(options.before))
    out = ROOT / "scratch" / "pole-pairs" / commit
    out.mkdir(parents=True, exist_ok=True)
    summary = {"after": commit, "before": before_commit}
    print(f"before {before_commit}, after {commit} (this checkout)", flush=True)

    with ThreadPoolExecutor(options.jobs) as pool:
        if "tmesh" in parts:
            tmesh_budgets = [int(b) for b in options.tmesh_budgets.split(",")]
            work = [(m, b, j) for b in tmesh_budgets for m in models for j in (False, True)]
            summary["tmesh"] = list(pool.map(lambda w: tmesh(after, sources / w[0] / "source.obj", *w), work))
            tmesh_table(summary["tmesh"], models, tmesh_budgets)
        if "remesh" in parts:
            remesh_budgets = [int(b) for b in options.budgets.split(",")]
            summary["remesh"] = {}
            for side, engine in (("before", before), ("after", after)):
                meshes = out / f"remesh-{side}"
                meshes.mkdir(exist_ok=True)
                work = [(m, b) for b in remesh_budgets for m in models]
                summary["remesh"][side] = list(pool.map(lambda w: budgets.run(engine, sources, meshes, *w), work))
            remesh_table(summary["remesh"], remesh_budgets)
    if "male-body" in parts:
        summary["male_body"] = {}
        for side, engine, hash_ in (("before", before, before_commit), ("after", after, commit)):
            conflicts = male_body(engine, hash_, sources / "human_male" / "source.obj", out / f"male-body-{side}",
                                  options.jobs)
            summary["male_body"][side] = {"per_way": conflicts["per_way"], "table": conflicts["table"]}
        print("\nMale body, infeasible sets of held turns: before -> after: "
              f"{len(summary['male_body']['before']['table'])} -> {len(summary['male_body']['after']['table'])}")
        for side in ("before", "after"):
            for row in summary["male_body"][side]["table"]:
                print(f"  {side:6} {json.dumps(row)[:200]}")

    path = ROOT / "scratch" / "pole-pairs" / f"{commit}.json"
    path.write_text(json.dumps(summary, indent=1) + "\n")
    print(f"\nwritten to {path.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
