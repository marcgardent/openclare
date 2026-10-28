#!/usr/bin/env python3
"""Which bounds of separation stand in the way of closing every held turn
(paragraph "The conflict is real but local" of Section "An exact look at
the closing system"): for each way, whether all its held turns close
together with the lengths only asked to be non-negative, and with the
bounds; then the least total amount of the bounds to give up for all to
close (a linear program: ℓ ≥ b - s, s ≥ 0, least Σ s), and the arcs whose
bound it gives up. The turns that cannot be closed even alone, and the
certificates, are those of `../../../turns-of-a-t-mesh/experiments/conflicts.py`.

    python3 docs/research/closing-loops/experiments/exact/relaxation.py [<export>]

Seconds. Writes results/relaxation.json.
"""

import argparse
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "turns-of-a-t-mesh" / "experiments"))
import tmesh  # noqa: E402  (first: it sets the threads of the linear algebra)

import numpy as np  # noqa: E402
from scipy.sparse import coo_matrix, hstack, identity, vstack  # noqa: E402


def given_up(way, forms):
    """The least Σ s with the forms closed, ℓ ≥ b - s: (total, {arc: s})."""
    n = way.n
    A = vstack([way.B, tmesh.matrix(forms, n)]).tocsr()
    eye = identity(n)
    r = tmesh.run_linprog(np.concatenate([np.zeros(n), np.ones(n)]),
                          A_eq=hstack([A, coo_matrix((A.shape[0], n))]).tocsr(), b_eq=np.zeros(A.shape[0]),
                          A_ub=hstack([-eye, -eye]).tocsr(), b_ub=-way.lower,
                          bounds=[(0, None)] * (2 * n))
    if r.status != 0:
        raise RuntimeError(r.message)
    s = r.x[n:]
    return float(s.sum()), {a: round(float(s[a]), 4) for a in range(n) if s[a] > 1e-7}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "relaxation", paper="closing-loops")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    rows = []
    for way in ways:
        forms = tmesh.held_forms(way)
        total, arcs = given_up(way, forms)
        row = {"way": way.number, "held": len(forms),
               "closable_with_lengths_nonnegative": way.closable(forms, lower=np.zeros(way.n)) is not None,
               "closable_within_the_bounds": way.closable(forms) is not None,
               "bounds_given_up": {str(a): {"amount": s, "target": round(float(way.target[a]), 3)}
                                   for a, s in sorted(arcs.items(), key=lambda item: -item[1])},
               "total_given_up": round(total, 4), "arcs_held_open": int((way.lower > 0).sum())}
        rows.append(row)
        print(f"way {way.number}: closable with ℓ ≥ 0 {row['closable_with_lengths_nonnegative']}, within the bounds "
              f"{row['closable_within_the_bounds']}; least giving up {row['total_given_up']} on arcs "
              f"{list(row['bounds_given_up'])[:8]}", flush=True)
    summary = {
        "experiment": "the bounds of separation that stand in the way",
        **tmesh.record(arguments, folder, ways, started, clock),
        "per_way": rows,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
