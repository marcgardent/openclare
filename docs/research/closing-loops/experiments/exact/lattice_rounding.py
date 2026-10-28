#!/usr/bin/env python3
"""Whole lengths closing a set of rings, nearest the target, found in the
lattice of closed quantizations (paragraph "Whole lengths are not what is
missing": "In whole numbers, with the reformulation of Aardal, Hurkens and
Lenstra ... stopped after 1,200 seconds"):

1. an integer basis of the whole lengths closing the rings, with the
   implicit zeros of their cone imposed first (`../../../turns-of-a-t-mesh/
   experiments/integer.py`);
2. its LLL reduction (δ = 0.99), for the plain inner product;
3. Babai's nearest plane from the real lengths closing the rings nearest the
   target with the number of quads kept to first order, and from the target;
4. the reformulation ℓ = M y, y whole: a program with no equality left, the
   least Σ|ℓ - t| with ℓ ≥ b and the number of quads at least the target's to
   first order, solved by HiGHS stopped after --seconds; its proven bound.

Mean deviations count the arcs forced to zero (their whole target). The
lengths found are written as `call-<name>.txt` under --out, for the engine
to mesh in place of its own (`OPENCLARE_LENGTHS`; remesh.py).

    python3 docs/research/closing-loops/experiments/exact/lattice_rounding.py [<export>] \\
        [--way 2] [--rings scratch/data-package/closing-loops/rings/subset.json] [--seconds 1200]

The reduction takes minutes (87 s on the drafts' old export, under load), the
program --seconds (one thread). Writes results/lattice_rounding.json.
"""

import argparse
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "turns-of-a-t-mesh" / "experiments"))
import tmesh  # noqa: E402  (first: it sets the threads of the linear algebra)

import numpy as np  # noqa: E402
from integer import Lattice, Reducer  # noqa: E402
from rings import area_row, implicit_zeros, nearest_plain  # noqa: E402
from scipy.optimize import Bounds, LinearConstraint  # noqa: E402
from scipy.sparse import csr_matrix, hstack, identity  # noqa: E402


def report(way, forms, zeros, x):
    x = np.asarray(x)
    return {"mean_deviation": way.deviation(x), "quads": way.quads(np.rint(x).astype(np.int64)),
            "below_a_bound": int((x < way.lower - 1e-9).sum()), "negative": int((x < -1e-9).sum()),
            "rings_open": sum(1 for f in forms if abs(way.value(f, x)) > 0.5),
            "balanced": all(abs(sum(c * x[a] for a, c in row.items())) < 1e-6 for row in way.rows)}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "lattice_rounding", paper="closing-loops")
    parser.add_argument("--way", type=int, default=2)
    parser.add_argument("--rings", type=Path, default=tmesh.DATA / "closing-loops" / "rings" / "subset.json",
                        help="the rings, as rings.py writes them")
    parser.add_argument("--seconds", type=float, default=1200.0, help="time limit of the program")
    arguments = parser.parse_args()
    arguments.ways = [arguments.way]
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, (way,) = tmesh.load(arguments)
    forms = [{int(a): int(c) for a, c in ring} for ring in json.loads(arguments.rings.read_text())]
    n = way.n
    target = way.target
    zeros = implicit_zeros(way, forms)
    lattice = Lattice(n)
    for a in zeros:
        lattice.impose([(a, 1)])
    for row in way.rows:
        lattice.impose(list(row.items()))
    for f in forms:
        lattice.impose(list(f.items()))
    columns = lattice.basis()
    support = sorted(set(range(n)) - set(zeros))
    where = {a: i for i, a in enumerate(support)}
    basis = np.zeros((len(columns), len(support)), dtype=np.int64)
    for j, column in enumerate(columns):
        for a, v in column:
            basis[j, where[a]] = v
    out = {"rings": len(forms), "implicit_zeros": len(zeros), "dimension": len(columns),
           "largest_entry_before": int(np.abs(basis).max()) if basis.size else 0}
    print(f"{len(forms)} rings: lattice of dimension {len(columns)} in {len(support)} arcs, {len(zeros)} forced to "
          f"zero", flush=True)
    tick = time.monotonic()
    reducer = Reducer(basis, np.ones(len(support)))
    swaps = reducer.reduce(log=lambda text: print(text, flush=True))
    reduced = reducer.B
    out["reduction"] = {"swaps": swaps, "seconds": round(time.monotonic() - tick, 1),
                        "largest_entry": int(np.abs(reduced).max()) if reduced.size else 0}
    print(f"LLL: {swaps} swaps in {out['reduction']['seconds']} s, largest entry {out['reduction']['largest_entry']}",
          flush=True)

    def whole(c):
        x = np.zeros(n, dtype=np.int64)
        x[support] = c @ reduced
        return x

    real = nearest_plain(way, forms, extra=(area_row(way, 3 * n),))
    out["real"] = report(way, forms, zeros, real)
    for name, aim in (("babai_from_the_real_point", real), ("babai_from_the_target", target)):
        out[name] = report(way, forms, zeros, whole(reducer.nearest_plane(aim[support])))

    # ℓ = M y: [y (d), p (m), q (m)], Mᵀ y - p + q = t, Mᵀ y ≥ b, quads to first order.
    d, m = len(columns), len(support)
    M = csr_matrix(reduced.T.astype(float))
    eye = identity(m)
    g = way.quads_gradient(target)
    constraints = [
        LinearConstraint(hstack([M, -eye, eye]).tocsr(), target[support], target[support]),
        LinearConstraint(hstack([M, csr_matrix((m, 2 * m))]).tocsr(), way.lower[support], np.inf),
        LinearConstraint(csr_matrix(np.concatenate([M.T @ g[support], np.zeros(2 * m)])[None, :]), g @ target, np.inf),
    ]
    result, took = tmesh.run_milp(np.concatenate([np.zeros(d), np.ones(2 * m)]), constraints,
                                  Bounds(np.concatenate([np.full(d, -np.inf), np.zeros(2 * m)]), np.inf),
                                  np.concatenate([np.ones(d), np.zeros(2 * m)]), arguments.seconds)
    forced = float(target[zeros].sum())
    program = {"message": result.message, "seconds": round(took, 1)}
    bound = getattr(result, "mip_dual_bound", None)
    if bound is not None:
        program["proven_mean_deviation_at_least"] = (bound + forced) / n
    if result.x is not None:
        x = whole(np.rint(result.x[:d]).astype(np.int64))
        program.update(report(way, forms, zeros, x))
        call = folder.name if folder.name.startswith("call-") else "lengths"
        (arguments.out / f"{call}.txt").write_text(tmesh.lengths_text(x, way.scale))
        program["file"] = f"{call}.txt"
    out["program"] = program
    print(f"real: {out['real']}\nBabai: {out['babai_from_the_real_point']}\nprogram: {program}", flush=True)
    summary = {
        "experiment": "whole lengths closing the rings, in the reduced lattice",
        **tmesh.record(arguments, folder, [way], started, clock, way=way.number, seconds=arguments.seconds,
                       rings_file=arguments.rings.name),
        **out,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
