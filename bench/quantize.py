#!/usr/bin/env python3
"""Quantizes the T-mesh of every model of the corpus and tabulates what comes out.

    python3 bench/quantize.py [--models a,b] [--targets 1000,3000,8000]

Needs a release build of `openclare`. The table goes to bench/results/QUANTIZATION.md.
"""

import argparse
import json
import statistics
import subprocess
import sys

from run import CREASE_ANGLE, LOCAL_MODELS, MODELS, RESULTS, OPENCLARE, source_path

# Desert Rose is not a manifold surface: it waits for the repair stage. A
# torus has no singularity, hence no T-mesh to quantize.
SKIPPED = {"rose", "torus"}


def quantize(model, target, hard_edges):
    command = [str(OPENCLARE), "quantize", str(source_path(model)), "--quads", str(target), "--simplify", "2", "--json"]
    if hard_edges:
        command += ["--crease-angle", CREASE_ANGLE]
    done = subprocess.run(command, capture_output=True, text=True)
    if done.returncode != 0:
        return {"error": done.stderr.strip().splitlines()[0] if done.stderr.strip() else "failed"}
    return json.loads(done.stdout)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--models", help="comma-separated subset of the corpus")
    parser.add_argument("--targets", default="1000,3000,8000", help="quad counts to aim for")
    args = parser.parse_args()
    wanted = set(args.models.split(",")) if args.models else None
    targets = [int(t) for t in args.targets.split(",")]

    lines = [
        "# Quantization of the corpus",
        "",
        "Whole lengths for the arcs of the T-mesh (see TMESH.md), aiming for the number of quads asked for.",
        "*Zero* counts the arcs given no length, *held* those kept from it so that the structure stays intact.",
        "*Deviation* is the mean distance, in quad edges, between a whole length and the scaled real one.",
        "",
        "| Model | Asked | Quads | Off by | Arcs | Zero | Held | Patches | Without area | Deviation | Valid | Seconds |",
        "|---|---|---|---|---|---|---|---|---|---|---|---|",
    ]
    valid = total = 0
    errors = []
    for model, _, hard_edges, _ in MODELS:
        if model in SKIPPED or model in LOCAL_MODELS or (wanted and model not in wanted):
            continue
        for target in targets:
            r = quantize(model, target, hard_edges)
            total += 1
            if "error" in r:
                lines.append(f"| {model} | {target} | failed: {r['error']} |")
                print(f"{model:10} {target:6} FAILED {r['error']}", file=sys.stderr)
                continue
            ok = r["balanced"] and r["separated"]
            valid += ok
            errors.append(r["budget_error"])
            lines.append(
                f"| {model} | {target} | {r['quads']} | {100 * r['budget_error']:.1f} % | {r['arcs']} | {r['zero_arcs']} "
                f"| {r['held_arcs']} | {r['patches']} | {r['collapsed_patches']} | {r['deviation_mean']:.2f} "
                f"| {'yes' if ok else '**no**'} | {r['seconds']:.2f} |"
            )
            print(f"{model:10} {target:6} {r['quads']:6} quads, valid: {ok}", file=sys.stderr)
    lines += [
        "",
        f"Valid: {valid} of {total}. Number of quads off by {100 * statistics.median(errors):.1f} % (median), "
        f"{100 * max(errors):.1f} % at worst.",
        "",
    ]
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "QUANTIZATION.md").write_text("\n".join(lines))
    print(lines[-2], file=sys.stderr)


if __name__ == "__main__":
    main()
