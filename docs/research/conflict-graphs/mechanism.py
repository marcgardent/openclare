#!/usr/bin/env python3
"""What makes two rings conflict once their walks are kept (level V of
analyse.py: balances, bounds of separation, walks; no box).

For every conflicting pair {A, B} of level V, which walks it needs: those of
A alone, of B alone, or both (a conflict that needs neither is one of the
bounds of separation, the research note's kind); and how far closing one
forces the other's shift: the least |shift_A| over the lengths that close B
and keep both walks (by two LPs). A value of one or more says that closing
B leaves A at least one row beside itself.

Usage: mechanism.py <supplement.json> <instance directory> [piece...]
"""

import json
import os
import sys

import numpy as np
from scipy import sparse
from scipy.optimize import linprog

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyse  # noqa: E402


def feasible(inst, closed, walks):
    """Lengths of level S closing `closed`, keeping the walks of `walks`."""
    n = inst.n
    eq = [inst.balance] + ([inst.shift[sorted(closed)]] if closed else [])
    a_eq = sparse.vstack(eq).tocsr()
    ub = [-inst.walk[k] for k in walks if inst.walk[k].shape[0]]
    b_ub = [inst.walk_constant[k] for k in walks if inst.walk[k].shape[0]]
    result = linprog(
        np.zeros(n),
        A_ub=sparse.vstack(ub).tocsr() if ub else None,
        b_ub=np.concatenate(b_ub) if ub else None,
        A_eq=a_eq,
        b_eq=np.zeros(a_eq.shape[0]),
        bounds=[(s, None) for s in inst.separation],
        method="highs",
    )
    return result.status == 0


def least_shift(inst, a, b):
    """The least |shift_a| over lengths of level V closing b, keeping the
    walks of a and b."""
    if not feasible(inst, [b], [a, b]):
        # Closing b cannot keep a's walk at all.
        return np.inf
    a_eq = sparse.vstack([inst.balance, inst.shift[[b]]]).tocsr()
    ub = [-inst.walk[k] for k in (a, b) if inst.walk[k].shape[0]]
    b_ub = [inst.walk_constant[k] for k in (a, b) if inst.walk[k].shape[0]]
    values = []
    for sign in (1, -1):
        result = linprog(
            sign * inst.shift[a].toarray().ravel(),
            A_ub=sparse.vstack(ub).tocsr() if ub else None,
            b_ub=np.concatenate(b_ub) if ub else None,
            A_eq=a_eq,
            b_eq=np.zeros(a_eq.shape[0]),
            bounds=[(s, None) for s in inst.separation],
            method="highs",
        )
        # The polyhedron is not empty: anything but an optimum is unbounded.
        values.append(sign * result.fun if result.status == 0 else -sign * np.inf)
    # The least and the most of shift_a.
    low, high = values
    # The shift ranges over [low, high]; the least |shift| is 0 if 0 is inside.
    if low > 0:
        return low
    if high < 0:
        return -high
    return 0.0


def main():
    supplement = {r["instance"]: r for r in json.load(open(sys.argv[1]))}
    directory = sys.argv[2]
    pieces = sys.argv[3:] or list(supplement)
    for piece in pieces:
        record = supplement.get(piece)
        if not record:
            continue
        v = [entry for entry in record["levels"] if entry["level"] == "V"][0]
        if not v["pairs"]:
            continue
        inst = analyse.Instance(os.path.join(directory, piece + ".json"))
        kinds = {"neither": 0, "one": 0, "both": 0}
        forcing = []
        for a, b in v["pairs"]:
            if not feasible(inst, [a, b], []):
                kinds["neither"] += 1
            elif not feasible(inst, [a, b], [a]) or not feasible(inst, [a, b], [b]):
                kinds["one"] += 1
            else:
                kinds["both"] += 1
            forcing.append(min(least_shift(inst, a, b), least_shift(inst, b, a)))
        forcing = np.array(forcing)
        print(
            f"{piece}: {len(v['pairs'])} pairs at level V; walks needed: {kinds}; "
            f"least forced shift (the smaller of the two ways): min {forcing.min():.3f}, median {np.median(forcing):.3f}, "
            f"share >= 1: {(forcing >= 1 - 1e-6).mean():.2f}, share >= 0.5: {(forcing >= 0.5 - 1e-6).mean():.2f}, "
            f"share where closing one cannot keep the other's walk: {np.isinf(forcing).mean():.2f}",
            flush=True,
        )


if __name__ == "__main__":
    main()
