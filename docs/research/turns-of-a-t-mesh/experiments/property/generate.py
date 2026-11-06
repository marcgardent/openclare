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

With `--borders` the families are surfaces with borders, each made closed by
its double (`quadmesh.double`): the border traced from the start, the T-mesh
mirror-symmetric, the lengths symmetric; `call-<name>/double.json` holds the
border and the involutions, and the default output is
`scratch/data-package/property/double-seed-<seed>`.
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
import grid  # noqa: E402
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
    # Long tubes with one or two 3–5 pairs: dislocations, staircases and
    # separatrices that wind.
    (("torus", 14, 8, 1), 2),
    (("torus", 20, 6, 1), 1),
    # Tubes with several pairs: the conflicts within bounds live there; boxes
    # are the spheres with a tube.
    (("torus", 12, 9, 4), 1),
    (("torus", 14, 10, 6), 2),
    (("box", 2, 2, 6, 2), 2),
    (("box", 3, 3, 8, 3), 2),
    (("hull", 6), 2),
    (("hull", 8), 2),
    (("hull", 12), 2),
    (("dual", 6), 2),
    (("dual", 8), 2),
    (("dual", 12), 2),
]

# The families with borders (`--borders`), doubled into closed surfaces: a
# disc doubles into a sphere, a tube into a torus, a sphere with b holes into
# a surface of genus b - 1. A tube needs a move: without one no trace reaches
# its border circles.
BORDER_FAMILIES = [
    (("disc", 3, 3, 0), 1),
    (("disc", 4, 5, 1), 1),
    (("disc", 6, 6, 2), 1),
    (("tube", 3, 3, 1), 1),
    (("tube", 5, 4, 1), 1),
    (("tube", 6, 6, 2), 1),
    (("cap", 0), 1),
    (("cap", 1), 1),
    (("holed", 8, 1), 1),
    (("holed", 12, 1), 1),
    (("holed", 20, 1), 1),
    (("holed", 14, 2), 1),
    (("holed", 20, 2), 1),
    (("holed", 20, 3), 1),
]


def genus(quad):
    return (2 - quad.euler()) // 2


def random_balanced(rng, T, longest):
    """Random whole and real balanced lengths of a T-mesh, from its quad
    mesh's own."""
    side = max(1, int(len(T.quad.faces) ** 0.5))
    steps = 8 * len(T.patches)
    base = walks.Walker(T, walks.quad_lengths(T))
    base_strips = base.strips(steps, longest * side)
    basis = walks.balanced_basis(T)
    whole, real = walks.random_lengths(rng, T, base_strips, basis, whole_adds=rng.randint(2, 12),
                                       real_moves=rng.randint(1, 4))
    return whole, real, basis


def symmetric(lengths, mirror_arc):
    """`lengths + lengths ∘ mirror_arc`: balanced and positive if `lengths` is,
    and the same on an arc and its image."""
    return [x + lengths[mirror_arc[a]] for a, x in enumerate(lengths)]


def traced_double(D):
    """The T-mesh of a double with its border traced from the start, and the
    involution the mirror induces on it (`tracing.mirrored`)."""
    T = tracing.trace(D.quad, D.border_edges)
    return T, tracing.mirrored(T, D.mirror, D.mirror_face)


def make(rng, kind, shares, most_lean, longest, retrace=True):
    """One instance. With `retrace`, the quad mesh of the subdivided polygon
    mesh at random whole lengths is traced again: on the symmetric grids of
    the subdivision, traces collide head-on at the same vertices and the feet
    of the arcs are seldom defined; at random lengths they meet generically.

    A kind with borders (`quadmesh.OPEN_KINDS`) is subdivided open and
    doubled; the lengths are made symmetric, the quad mesh at the whole
    lengths is a double again (`grid.coarse_double`), and the T-mesh carries
    `double`: the `quadmesh.Double` it was traced on and the involutions
    `node`, `arc`, `direction`, `patch`. Otherwise `T.double` is None."""
    poly = quadmesh.random_polygon_mesh(rng, kind)
    D = None
    if kind[0] in quadmesh.OPEN_KINDS:
        D = quadmesh.double(quadmesh.catmull_clark(poly, closed=False))
        quad = D.quad
        T, mirror = traced_double(D)
    else:
        quad = quadmesh.catmull_clark(poly)
        T = tracing.trace(quad)
    if retrace:
        whole0, _, _ = random_balanced(rng, T, longest)
        if D is None:
            quad = grid.coarse_quad_mesh(grid.Grid(T, whole0))
            T = tracing.trace(quad)
        else:
            node, arc, direction, patch = mirror
            copy_of_patch = [D.copy[P["faces"][0]] for P in T.patches]
            D = grid.coarse_double(grid.Grid(T, symmetric(whole0, arc)), arc, direction, patch, copy_of_patch)
            quad = D.quad
            T, mirror = traced_double(D)
    side = max(1, int(len(quad.faces) ** 0.5))
    steps = 8 * len(T.patches)
    whole, real, basis = random_balanced(rng, T, longest)
    T.double = None
    if D is not None:
        node, arc, direction, patch = mirror
        whole, real = symmetric(whole, arc), symmetric(real, arc)
        T.double = {"double": D, "node": node, "arc": arc, "direction": direction, "patch": patch}
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


def double_json(T, kind, whole, real):
    """`call-<name>/double.json`: the border of the open quad mesh in the
    numbering of the double the T-mesh was traced on (each component with the
    first copy on its left, each vertex with its angle `a`, quarter turns of
    the open surface), the involutions of the double and of the T-mesh, the
    border arcs and nodes, the genus, and whether the lengths are symmetric."""
    info = T.double
    D = info["double"]
    arc = info["arc"]
    return {
        "open_kind": list(kind),
        "border": [{"vertices": cycle, "angles": [D.angle[v] for v in cycle]} for cycle in D.border],
        "mirror": D.mirror,
        "mirror_node": info["node"],
        "mirror_arc": arc,
        "mirror_patch": info["patch"],
        "border_arcs": [a for a in range(len(arc)) if arc[a] == a],
        "border_nodes": [n for n, m in enumerate(info["node"]) if m == n],
        "genus": genus(D.quad),
        "whole_symmetric": all(whole[a] == whole[arc[a]] for a in range(len(arc))),
        "real_symmetric": all(real[a] == real[arc[a]] for a in range(len(arc))),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--count", type=int, default=50)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--out", default=None)
    parser.add_argument("--shares", default="0.5", help="shares of each side the turns start from")
    parser.add_argument("--most-lean", type=float, default=0.12)
    parser.add_argument("--longest", type=float, default=6.0, help="in sides of the mesh")
    parser.add_argument("--borders", action="store_true", help="the families with borders, doubled")
    args = parser.parse_args()
    prefix = "double-seed" if args.borders else "seed"
    out = args.out or os.path.join("scratch", "data-package", "property", f"{prefix}-{args.seed}")
    shares = [Fraction(s) for s in args.shares.split(",")]
    rng = random.Random(args.seed)
    kinds = [kind for kind, weight in (BORDER_FAMILIES if args.borders else FAMILIES) for _ in range(weight)]
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
        folder = export.write(out, name, way, helices)
        made.append({
            "kind": list(kind), "seed": instance_seed, "name": name, "genus": genus(quad),
            "quads": len(quad.faces), "nodes": len(T.nodes),
            "singular": sum(1 for v in T.nodes if quad.valence[v] != 4),
            "arcs": len(T.arcs), "patches": len(T.patches), "turns": len(turns), "held": len(held),
            "strips": len(strips), "shifted_turns": sum(1 for t in turns if walks.value(t["shift"], real) != 0),
            "shifted_strips": sum(1 for s in strips if walks.value(s["shift"], whole) != 0),
            "lattice_dimension": len(basis),
        })
        if T.double is not None:
            double = double_json(T, kind, whole, real)
            with open(os.path.join(folder, "double.json"), "w") as f:
                json.dump(double, f, separators=(",", ":"))
            made[-1].update({"border_components": len(double["border"]),
                             "border_nodes": len(double["border_nodes"]),
                             "border_arcs": len(double["border_arcs"]),
                             "lengths_symmetric": double["whole_symmetric"] and double["real_symmetric"]})
        print(f"{name}: genus {made[-1]['genus']}, {made[-1]['patches']} patches, {made[-1]['arcs']} arcs, "
              f"{made[-1]['turns']} turns ({made[-1]['shifted_turns']} shifted), {made[-1]['strips']} strips")
    summary = {
        "experiment": "random T-meshes for the property tests",
        "seed": args.seed, "count": args.count, "attempts": attempts,
        "shares": [str(s) for s in shares], "most_lean": args.most_lean, "longest": args.longest,
        "seconds": round(time.time() - started, 1),
        "made": made, "rejected": rejected,
    }
    if args.borders:
        summary["families"] = "with borders, doubled"
        reasons = {}
        for r in rejected:
            reason = "not symmetric" if r["reason"].startswith("not symmetric") else "not rectangles"
            reasons[reason] = reasons.get(reason, 0) + 1
        summary["rejected_by_reason"] = reasons
    os.makedirs(out, exist_ok=True)
    with open(os.path.join(out, "summary.json"), "w") as f:
        json.dump(summary, f, indent=1)
    print(f"{len(made)} instances in {out}, {len(rejected)} rejected, {summary['seconds']} s")


if __name__ == "__main__":
    main()
