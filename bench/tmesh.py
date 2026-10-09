#!/usr/bin/env python3
"""Builds the T-mesh of every model of the corpus and tabulates what comes out.

    python3 bench/tmesh.py [--models a,b] [--targets 1000,3000,8000]

Needs a release build of `openclare`. The table goes to bench/results/TMESH.md.
"""

import argparse
import json
import sys

from run import AUTHORED, CREASE_ANGLE, LOCAL_MODELS, MODELS, RESULTS, OPENCLARE, memory_cap, run, source_path

# Desert Rose is not a manifold surface: it waits for the repair stage.
SKIPPED = {"rose"}


def tmesh(model, target, hard_edges):
    command = [str(OPENCLARE), "tmesh", str(source_path(model)), "--quads", str(target), "--simplify", "2", "--json"]
    if hard_edges:
        command += ["--crease-angle", CREASE_ANGLE]
    done = run(command, memory=memory_cap(model))
    if done.returncode != 0:
        return {"error": done.stderr.strip().splitlines()[0] if done.stderr.strip() else "failed"}
    return json.loads(done.stdout)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--models", help="comma-separated subset of the corpus")
    parser.add_argument("--targets", default="1000,3000,8000", help="quad counts the map is scaled for")
    args = parser.parse_args()
    wanted = set(args.models.split(",")) if args.models else None
    targets = [int(t) for t in args.targets.split(",")]

    lines = [
        "# T-mesh of the corpus",
        "",
        "Motorcycle graph of the seamless map, after cancelling singularity pairs closer than 2 quad edges.",
        "*Inverted* counts the triangles folded over in the map the traces run on.",
        "*Thin* counts the rectangles narrower than half a quad edge, *short* the arcs shorter than that.",
        "",
        "| Model | Quads | Inverted | Singularities | Traces | Lost | Nodes | Arcs | Patches | Rectangles | Thin | Short | Valid | Seconds |",
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|",
    ]
    valid = total = 0
    for model, _, hard_edges, _ in MODELS:
        # The scenes made by hand are more than a mesh: only if asked for.
        if model in SKIPPED or model in LOCAL_MODELS or (wanted and model not in wanted) or (
            model in AUTHORED and not wanted
        ):
            continue
        for target in targets:
            r = tmesh(model, target, hard_edges)
            total += 1
            if "error" in r:
                lines.append(f"| {model} | {target} | failed: {r['error']} |")
                print(f"{model:10} {target:6} FAILED {r['error']}", file=sys.stderr)
                continue
            valid += r["valid"]
            note = "yes" if r["valid"] else "**no**"
            if r["features_off_grid"]:
                note += " (features off the grid lines)"
            lines.append(
                f"| {model} | {target} | {r['map_inversions']} | {r['singularities']} | {r['traces']} | {r['lost_traces']} | {r['nodes']} "
                f"| {r['arcs']} | {r['patches']} | {r['rectangles']} | {r['thin_patches']} | {r['short_arcs']} "
                f"| {note} | {r['seconds']:.2f} |"
            )
            print(f"{model:10} {target:6} {r['patches']:5} patches, valid: {r['valid']}", file=sys.stderr)
    lines += ["", f"Valid: {valid} of {total}.", ""]
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "TMESH.md").write_text("\n".join(lines))
    print(f"valid: {valid} of {total}", file=sys.stderr)


if __name__ == "__main__":
    main()
