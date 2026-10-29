"""Random T-meshes that do not come from the engine, exported as the engine
exports its own, for `Check/Main.lean` (the Lean oracle) and the property
tests of the research notes.

    python3 docs/research/turns-of-a-t-mesh/experiments/property/generate.py \\
        --count 50 --seed 1 --out scratch/data-package/property/seed-1

Each instance is a random polygon mesh of a closed surface (`quadmesh.py`)
turned into an all-quad mesh by one step of Catmull–Clark, its motorcycle
graph (`tracing.py`), whole lengths drawn by adding strips of quads to the
mesh's own lengths and real lengths in eighths beside them (`walks.py`), and
the turns walked at the real lengths from the middle of every side. Then

    python3 docs/research/turns-of-a-t-mesh/lean/Check/check_exports.py \\
        scratch/data-package/property/seed-1 scratch/data-package/property/seed-1/lean-check

checks them all. `summary.json` in the output folder says what was made and
what was rejected (regions that are not rectangles), with the seed, so that a
run is made again exactly.
"""

import argparse
import json
import os
import random
import sys
import time
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import export  # noqa: E402
import quadmesh  # noqa: E402
import tracing  # noqa: E402
import walks  # noqa: E402

# The families drawn from, with their weights.
FAMILIES = [
    (("cube", 0), 1),
    (("cube", 1), 2),
    (("cube", 2), 2),
    (("torus", 6, 5, 1), 2),
    (("torus", 8, 6, 2), 2),
    (("torus", 10, 7, 3), 2),
    (("torus", 12, 9, 4), 1),
    (("hull", 6), 2),
    (("hull", 8), 2),
    (("hull", 12), 2),
    (("dual", 6), 2),
    (("dual", 8), 2),
    (("dual", 12), 2),
]


def genus(quad):
    return (2 - quad.euler()) // 2


def make(rng, kind, shares, most_lean, longest):
    poly = quadmesh.random_polygon_mesh(rng, kind)
    quad = quadmesh.catmull_clark(poly)
    T = tracing.trace(quad)
    side = max(1, int(len(quad.faces) ** 0.5))
    steps = 8 * len(T.patches)
    base = walks.Walker(T, walks.quad_lengths(T))
    base_strips = base.strips(steps, longest * side)
    basis = walks.balanced_basis(T)
    whole, real = walks.random_lengths(rng, T, base_strips, basis, whole_adds=rng.randint(2, 12),
                                       real_moves=rng.randint(1, 4))
    at_real = walks.Walker(T, real)
    turns = at_real.turns_from_shares(shares, steps, longest * side)
    # Held, as the engine holds: a real shift above 0.05, a lean below the
    # bound, each shift expression once (up to its sign).
    held = []
    for turn in turns:
        shift = turn["shift"]
        real_shift = walks.value(shift, real)
        if abs(real_shift) <= Fraction(1, 20) or abs(real_shift) / turn["around"] >= most_lean:
            continue
        negated = {a: -c for a, c in shift.items()}
        if shift not in held and negated not in held:
            held.append(shift)
    strips = walks.Walker(T, whole).strips(steps, longest * side)
    return quad, T, whole, real, turns, held, strips, basis


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--count", type=int, default=50)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--out", default=None)
    parser.add_argument("--shares", default="0.5", help="shares of each side the turns start from")
    parser.add_argument("--most-lean", type=float, default=0.12)
    parser.add_argument("--longest", type=float, default=6.0, help="in sides of the mesh")
    args = parser.parse_args()
    out = args.out or os.path.join("scratch", "data-package", "property", f"seed-{args.seed}")
    shares = [Fraction(s) for s in args.shares.split(",")]
    rng = random.Random(args.seed)
    kinds = [kind for kind, weight in FAMILIES for _ in range(weight)]
    made, rejected = [], []
    started = time.time()
    attempts = 0
    while len(made) < args.count:
        attempts += 1
        kind = rng.choice(kinds)
        instance_seed = rng.randrange(1 << 30)
        instance_rng = random.Random(instance_seed)
        try:
            quad, T, whole, real, turns, held, strips, basis = make(
                instance_rng, kind, shares, args.most_lean, args.longest)
        except tracing.NotRectangles as reason:
            rejected.append({"kind": list(kind), "seed": instance_seed, "reason": str(reason)})
            continue
        name = f"{kind[0]}-{instance_seed}"
        way = export.way_json(T, whole, real, turns, held, shares, args.most_lean, args.longest, name)
        helices = export.helices_json(strips, whole)
        export.write(out, name, way, helices)
        made.append({
            "kind": list(kind), "seed": instance_seed, "name": name, "genus": genus(quad),
            "quads": len(quad.faces), "nodes": len(T.nodes),
            "singular": sum(1 for v in T.nodes if quad.valence[v] != 4),
            "arcs": len(T.arcs), "patches": len(T.patches), "turns": len(turns), "held": len(held),
            "strips": len(strips), "shifted_turns": sum(1 for t in turns if walks.value(t["shift"], real) != 0),
            "shifted_strips": sum(1 for s in strips if walks.value(s["shift"], whole) != 0),
            "lattice_dimension": len(basis),
        })
        print(f"{name}: genus {made[-1]['genus']}, {made[-1]['patches']} patches, {made[-1]['arcs']} arcs, "
              f"{made[-1]['turns']} turns ({made[-1]['shifted_turns']} shifted), {made[-1]['strips']} strips")
    summary = {
        "experiment": "random T-meshes for the property tests",
        "seed": args.seed, "count": args.count, "attempts": attempts,
        "shares": [str(s) for s in shares], "most_lean": args.most_lean, "longest": args.longest,
        "seconds": round(time.time() - started, 1),
        "made": made, "rejected": rejected,
    }
    os.makedirs(out, exist_ok=True)
    with open(os.path.join(out, "summary.json"), "w") as f:
        json.dump(summary, f, indent=1)
    print(f"{len(made)} instances in {out}, {len(rejected)} rejected, {summary['seconds']} s")


if __name__ == "__main__":
    main()
