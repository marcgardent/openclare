#!/usr/bin/env python3
"""A local search over whole lengths near the engine's (paragraph "The
heuristic leaves most of the gap": "a local search over whole lengths
within the same cap and size closes 38 more strips of the engine's mesh than
the engine does"): within |ℓ_a - e_a| ≤ --radius of the engine's lengths e,
the balances, the bounds of separation, the engine's cap on the mean
deviation and its window of sizes (to first order round e, the exact count
checked), the fewest of a family left shifted: the strips of the way's mesh
leaning less than 0.12 (or its held turns), each once up to sign. Within the
box the big M of each turn is tight. With --sound, every arc the engine keeps
open is held at length one or more too.

    python3 docs/research/closing-loops/experiments/exact/local_search.py [<export>] \\
        [--way 2] [--family strips] [--radius 2] [--seconds 1200] [--sound]

The lengths found are written as `call-<name>.txt` under --out, for the
engine to mesh (`OPENCLARE_LENGTHS`; remesh.py), whose mesh says how much
it spirals. Up to --seconds (one thread). Writes results/local_search.json.
"""

import argparse
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "turns-of-a-t-mesh" / "experiments"))
import tmesh  # noqa: E402  (first: it sets the threads of the linear algebra)

import numpy as np  # noqa: E402
from forced import OLD_EXPORT, Family  # noqa: E402
from scipy.optimize import Bounds, LinearConstraint  # noqa: E402
from scipy.sparse import coo_matrix, diags, hstack  # noqa: E402


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "local_search", paper="closing-loops")
    parser.add_argument("--way", type=int, default=2)
    parser.add_argument("--family", choices=["strips", "held"], default="strips")
    parser.add_argument("--radius", type=int, default=2)
    parser.add_argument("--seconds", type=float, default=1200.0)
    parser.add_argument("--sound", action="store_true", help="keep open every arc the engine keeps open")
    parser.add_argument("--cap", type=float)
    parser.add_argument("--wanted", type=int)
    parser.add_argument("--allowed", type=int)
    arguments = parser.parse_args()
    arguments.ways = [arguments.way]
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, (way,) = tmesh.load(arguments)
    call = folder.parent / f"{folder.name}.json"
    engine_data = json.loads(call.read_text()) if call.exists() and folder.name.startswith("call-") else {}
    cap = arguments.cap or engine_data.get("most_deviation") or OLD_EXPORT["cap"]
    wanted = arguments.wanted or engine_data.get("wanted") or OLD_EXPORT["wanted"]
    allowed = arguments.allowed or engine_data.get("allowed") or OLD_EXPORT["allowed"]
    forms = [s["shift"] for s in way.strips(0.12)] if arguments.family == "strips" else tmesh.held_forms(way)
    family = Family(way, arguments.family, forms, cap, wanted, allowed, arguments.sound)
    n, m, R = family.n, family.m, arguments.radius
    e = way.lengths.astype(float)
    S = family.S
    big = np.asarray(abs(S).sum(axis=1)).ravel() * R + np.abs(S @ e) + 1
    constraints, lb, ub, width = family.blocks(window=(e, wanted - allowed + 20, wanted + allowed - 20))
    constraints = [LinearConstraint(hstack([c.A, coo_matrix((c.A.shape[0], m))]).tocsr(), c.lb, c.ub)
                   for c in constraints]
    zero = coo_matrix((m, width - n))
    constraints += [LinearConstraint(hstack([S, zero, -diags(big)]).tocsr(), -np.inf, 0),
                    LinearConstraint(hstack([S, zero, diags(big)]).tocsr(), 0, np.inf)]
    lb = np.concatenate([np.maximum(lb[:n], e - R), lb[n:], np.zeros(m)])
    ub = np.concatenate([np.minimum(ub[:n], e + R), ub[n:], np.ones(m)])
    cost = np.concatenate([np.zeros(width), np.ones(m)])
    integrality = np.concatenate([np.ones(n), np.zeros(width - n), np.ones(m)])
    result, took = tmesh.run_milp(cost, constraints, Bounds(lb, ub), integrality, arguments.seconds, mip_rel_gap=0)
    out = {"family": arguments.family, "turns": m, "radius": R, "sound": arguments.sound, "cap": cap,
           "wanted": wanted, "allowed": allowed, "engine_left_shifted": len(family.engine_shifted()),
           "message": result.message, "seconds": round(took, 1),
           "bound_within_the_box": getattr(result, "mip_dual_bound", None)}
    if result.x is not None:
        x = np.rint(result.x[:n]).astype(np.int64)
        check = family.check(x)
        out.update(left_shifted=len(check["shifted"]), quads=check["quads"], deviation=check["deviation"],
                   admissible=check["admissible"], arcs_moved=int((x != way.lengths).sum()),
                   closed_more_than_the_engine=out["engine_left_shifted"] - len(check["shifted"]))
        name = folder.name if folder.name.startswith("call-") else "lengths"
        (arguments.out / f"{name}.txt").write_text(tmesh.lengths_text(x, way.scale))
        out["file"] = f"{name}.txt"
    print(f"way {way.number}, {arguments.family}: the engine leaves {out['engine_left_shifted']} of {m} shifted; "
          f"within {R} of its lengths: {out.get('left_shifted')} ({out['message']}, {out['seconds']} s)")
    summary = {
        "experiment": "a local search over whole lengths near the engine's",
        **tmesh.record(arguments, folder, [way], started, clock, way=way.number),
        **out,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
