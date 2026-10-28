#!/usr/bin/env python3
"""The rings of the strips of one way's mesh (paragraph "Whole lengths are not
what is missing", from "Walking every strip"): the ways round of the strips
of the way's whole lengths, those shifted, those leaning less than 0.12;
the rings kept by one round of the constraint rounds (the strips leaning
less than 0.12, the closed ones first then the longest, each kept if those
kept still close together in real lengths within the bounds); the arcs every
real solution closing them sets to zero (implicit zeros of the cone, one
linear program), with a Farkas certificate for each (the least Σ|u|, grouped
into distinct certificates); whole lengths closing them once those zeros are
given as upper bounds, and, with --without-zeros S, without them (HiGHS
stopped after S seconds); the least mean deviation from the target of real
lengths closing them; and a subset that closes within a mean deviation
--subset-cap and the number of quads to first order, the most such rings (a
program, one binary per ring, stopped after --subset-seconds) or, with
--subset greedy, chosen greedily in the same order; with its least mean
deviation and its number of quads.

    python3 docs/research/closing-loops/experiments/exact/rings.py [<export>] [--way 2] \\
        [--subset-cap 0.40] [--subset most|greedy] [--without-zeros 600]

Writes results/rings.json, and under --out `rings.json` (the rings kept) and
`subset.json` (the subset), which lattice_rounding.py reads. A few minutes;
--without-zeros adds its limit.
"""

import argparse
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "turns-of-a-t-mesh" / "experiments"))
import tmesh  # noqa: E402  (first: it sets the threads of the linear algebra)

import numpy as np  # noqa: E402
from scipy.optimize import Bounds, LinearConstraint  # noqa: E402
from scipy.sparse import coo_matrix, csr_matrix, hstack, identity, vstack  # noqa: E402


def implicit_zeros(way, forms):
    """Arcs zero on all of {ℓ ≥ 0 : Bℓ = 0, Sℓ = 0}: max Σ z, z ≤ ℓ, z ≤ 1."""
    n = way.n
    A = vstack([way.B, tmesh.matrix(forms, n)]).tocsr()
    eye = identity(n)
    constraints = [LinearConstraint(hstack([A, coo_matrix((A.shape[0], n))]).tocsr(), 0, 0),
                   LinearConstraint(hstack([-eye, eye]).tocsr(), -np.inf, 0)]
    result, _ = tmesh.run_milp(np.concatenate([np.zeros(n), -np.ones(n)]), constraints,
                               Bounds(np.zeros(2 * n), np.concatenate([np.full(n, np.inf), np.ones(n)])),
                               np.zeros(2 * n))
    return [a for a in range(n) if result.x[n + a] < 0.5]


def certificates(way, forms, zeros):
    """For each implicit zero not yet explained, the least Σ|u| with
    c = Σ u s + λᵀB ≥ 0 and c_a ≥ 1: the rings it uses and the arcs it forces."""
    n, m = way.n, len(forms)
    S, B = tmesh.matrix(forms, n), way.B
    rows = hstack([S.T, -S.T, B.T]).tocsr()
    p = B.shape[0]
    found, todo = [], list(zeros)
    while todo:
        a = todo.pop(0)
        lower = np.zeros(n)
        lower[a] = 1
        result, _ = tmesh.run_milp(np.concatenate([np.ones(2 * m), np.zeros(p)]),
                                   [LinearConstraint(rows, lower, np.inf)],
                                   Bounds(np.concatenate([np.zeros(2 * m), np.full(p, -np.inf)]), np.inf),
                                   np.zeros(2 * m + p))
        if result.x is None:
            found.append({"arc": a, "certificate": None})
            continue
        u = result.x[:m] - result.x[m:2 * m]
        c = rows @ result.x
        support = [b for b in range(n) if c[b] > 1e-7]
        used = [k for k in range(m) if abs(u[k]) > 1e-7]
        found.append({"arc": a, "rings": used, "multipliers": [round(float(u[k]), 4) for k in used],
                      "forced": support, "forced_target": round(float(way.target[support].sum()), 3),
                      "at": [round(float(v), 3) for v in way.data["arcs_at"][a]]})
        todo = [b for b in todo if b not in support]
    return found


def nearest_plain(way, forms, extra=()):
    """The least mean |ℓ - t| of real lengths closing the forms within the bounds."""
    result, x, _ = way.nearest(forms, weighted=False, extra=extra)
    return x


def area_row(way, n_columns):
    """Q(t) + ∇Q(t)·(ℓ - t) ≥ Q(t): the number of quads kept to first order."""
    g = way.quads_gradient(way.target)
    row = np.concatenate([g, np.zeros(n_columns - way.n)])
    return LinearConstraint(csr_matrix(row[None, :]), g @ way.target, np.inf)


def capped_area(way, forms, cap):
    """Real lengths closing the forms within the bounds, mean |ℓ - t| ≤ cap,
    the number of quads at least the target's to first order; or None."""
    n = way.n
    budget = LinearConstraint(csr_matrix(np.concatenate([np.zeros(n), np.ones(2 * n)])[None, :]), 0, cap * n)
    result, x, _ = way.nearest(forms, weighted=False, extra=(budget, area_row(way, 3 * n)))
    return x


def most_within(way, forms, cap, seconds):
    """The most of the forms closable together within the bounds, mean
    |ℓ - t| ≤ cap and the number of quads at least the target's to first
    order: a binary per ring, big M (as the table's program)."""
    n, m = way.n, len(forms)
    t = way.target
    S = tmesh.matrix(forms, n)
    big = 200.0
    eye = identity(n)

    def zeros(rows):
        return coo_matrix((rows, n))

    g = way.quads_gradient(t)
    constraints = [
        LinearConstraint(hstack([way.B, zeros(way.B.shape[0]), zeros(way.B.shape[0]),
                                 coo_matrix((way.B.shape[0], m))]).tocsr(), 0, 0),
        LinearConstraint(hstack([eye, -eye, eye, coo_matrix((n, m))]).tocsr(), t, t),
        LinearConstraint(hstack([S, zeros(m), zeros(m), identity(m) * big]).tocsr(), -np.inf, big),
        LinearConstraint(hstack([-S, zeros(m), zeros(m), identity(m) * big]).tocsr(), -np.inf, big),
        LinearConstraint(csr_matrix(np.concatenate([np.zeros(n), np.ones(2 * n), np.zeros(m)])[None, :]), 0, cap * n),
        LinearConstraint(csr_matrix(np.concatenate([g, np.zeros(2 * n + m)])[None, :]), g @ t, np.inf),
    ]
    bounds = Bounds(np.concatenate([way.lower, np.zeros(2 * n + m)]),
                    np.concatenate([np.full(3 * n, np.inf), np.ones(m)]))
    result, took = tmesh.run_milp(np.concatenate([np.zeros(3 * n), -np.ones(m)]), constraints, bounds,
                                  np.concatenate([np.zeros(3 * n), np.ones(m)]), seconds)
    chosen = [] if result.x is None else [k for k in range(m) if result.x[3 * n + k] > 0.5]
    return chosen, result, took


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "rings", paper="closing-loops")
    parser.add_argument("--way", type=int, default=2, help="the way whose mesh is walked (default 2)")
    parser.add_argument("--most-lean", type=float, default=0.12)
    parser.add_argument("--subset-cap", type=float, default=0.40, help="mean deviation allowed to the subset")
    parser.add_argument("--subset", choices=["most", "greedy"], default="most",
                        help="the subset: the most rings within the cap (a program), or chosen greedily")
    parser.add_argument("--subset-seconds", type=float, default=600.0, help="time limit of the subset's program")
    parser.add_argument("--without-zeros", type=float, help="seconds for whole lengths without the zeros given")
    arguments = parser.parse_args()
    arguments.ways = [arguments.way]
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, (way,) = tmesh.load(arguments)
    strips = way.strips(None)
    leaning = way.strips(arguments.most_lean)
    out = {"ways_round": len(strips), "shifted": sum(1 for s in strips if abs(s["pitch"]) >= 0.5),
           "leaning_below": len(leaning), "most_lean": arguments.most_lean}
    # One round of the constraint rounds: the closed first, then the longest.
    order = sorted(leaning, key=lambda s: (abs(s["pitch"]) >= 0.5, -s["around"]))
    kept, refused = [], 0
    for strip in order:
        if way.closable([s["shift"] for s in kept] + [strip["shift"]]) is not None:
            kept.append(strip)
        else:
            refused += 1
    forms = [s["shift"] for s in kept]
    out.update(rings_kept=len(kept), rings_kept_up_to_sign=len({tmesh.key(f) for f in forms}), refused=refused)
    print(f"way {way.number}: {out['ways_round']} ways round, {out['shifted']} shifted, {out['leaning_below']} "
          f"leaning < {arguments.most_lean}; {len(kept)} rings closable together ({out['rings_kept_up_to_sign']} "
          f"strips)", flush=True)
    (arguments.out / "rings.json").write_text(json.dumps([[[a, c] for a, c in f.items()] for f in forms]) + "\n")

    zeros = implicit_zeros(way, forms)
    found = certificates(way, forms, zeros)
    distinct = {tuple(item["rings"]) for item in found if item.get("rings") is not None}
    out["implicit_zeros"] = {"arcs": len(zeros), "held_open_among_them": int(sum(way.lower[a] > 0 for a in zeros)),
                             "target_total": round(float(way.target[zeros].sum()), 3),
                             "target_largest": round(float(way.target[zeros].max()), 3) if zeros else 0,
                             "certificates": len(found), "distinct_ring_sets": len(distinct),
                             "rings_per_certificate": sorted(len(k) for k in distinct), "list": found}
    print(f"   {len(zeros)} implicit zeros, {len(distinct)} distinct certificates", flush=True)

    upper = np.full(way.n, np.inf)
    upper[zeros] = 0
    A = vstack([way.B, tmesh.matrix(forms, way.n)]).tocsr()
    result, took = tmesh.run_milp(np.zeros(way.n), [LinearConstraint(A, 0, 0)], Bounds(way.lower, upper),
                                  np.ones(way.n), 600)
    x = None if result.x is None else np.rint(result.x).astype(np.int64)
    out["whole_with_zeros_given"] = {"found": x is not None and way.closes([], list(x)) and all(
        sum(c * int(x[a]) for a, c in f.items()) == 0 for f in forms), "seconds": round(took, 1)}
    print(f"   whole lengths with the zeros given: {out['whole_with_zeros_given']}", flush=True)
    if arguments.without_zeros:
        result, took = tmesh.run_milp(np.zeros(way.n), [LinearConstraint(A, 0, 0)],
                                      Bounds(way.lower, np.inf), np.ones(way.n), arguments.without_zeros)
        out["whole_without_zeros"] = {"found": result.x is not None, "message": result.message,
                                      "seconds": round(took, 1)}
        print(f"   whole lengths without them: {out['whole_without_zeros']}", flush=True)

    x = nearest_plain(way, forms)
    out["least_mean_deviation"] = way.deviation(x) if x is not None else None
    out["least_mean_deviation_quads"] = way.quads(x) if x is not None else None
    out["engine"] = {"deviation": way.deviation(way.lengths), "quads": way.quads(way.lengths)}
    call = folder.parent / f"{folder.name}.json"
    if call.exists():
        data = json.loads(call.read_text())
        out["engine"].update(plain_deviation=data.get("plain_deviation"), cap=data.get("most_deviation"))
    print(f"   least mean deviation closing them: {out['least_mean_deviation']:.3f}", flush=True)

    # A subset within the cap and the number of quads to first order:
    # chosen greedily in the same order, or the most by a program.
    if arguments.subset == "greedy":
        subset = []
        for strip in kept:
            if capped_area(way, [s["shift"] for s in subset] + [strip["shift"]], arguments.subset_cap) is not None:
                subset.append(strip)
        how = {"chosen": "greedily"}
    else:
        chosen, result, took = most_within(way, forms, arguments.subset_cap, arguments.subset_seconds)
        subset = [kept[k] for k in chosen]
        how = {"chosen": "the most, by a program", "message": result.message, "seconds": round(took, 1),
               "dual_bound": getattr(result, "mip_dual_bound", None)}
    subset_forms = [s["shift"] for s in subset]
    x = nearest_plain(way, subset_forms, extra=(area_row(way, 3 * way.n),))
    out["subset"] = {"cap": arguments.subset_cap, "rings": len(subset), **how,
                     "least_mean_deviation_with_quads_to_first_order": way.deviation(x) if x is not None else None,
                     "quads": way.quads(x) if x is not None else None}
    (arguments.out / "subset.json").write_text(json.dumps([[[a, c] for a, c in f.items()] for f in subset_forms])
                                               + "\n")
    print(f"   subset within {arguments.subset_cap}: {len(subset)} rings, least mean deviation "
          f"{out['subset']['least_mean_deviation_with_quads_to_first_order']}, quads {out['subset']['quads']}",
          flush=True)
    summary = {
        "experiment": "the rings of the strips of one way's mesh",
        **tmesh.record(arguments, folder, [way], started, clock, way=way.number),
        **out,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
