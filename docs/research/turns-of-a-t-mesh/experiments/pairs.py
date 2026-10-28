#!/usr/bin/env python3
"""The geometry of the conflicts found by `conflicts.py` (Table `tab:pairs`,
the paragraphs "A separatrix that winds" and "Larger conflicts" of Section
"Conflicts between turns"). For each row of its table (in the first way
listed):

- the curves: simple or not, how often they cross themselves and each other;
- the winding numbers of the cycle Σ u_r γ_r (primitive multipliers) round
  the regions the curves cut the surface into, with the singular vertices of
  the regions where it is not zero ("between the turns");
- staircases (Proposition "Staircases"): monotone paths of arcs, through
  T-junctions only and at most --depth arcs long, between two of those
  singular vertices, whose length equals the combination of the shifts
  modulo the balances; where the held arc of the certificate lies (on the
  staircase, or reached by the balances);
- the mechanism this suggests: a dislocation (a 3–5 pair joined by the
  staircase), an exchange (two vertices of one valence), or none found;
- for a turn that cannot be closed alone (Proposition "A separatrix that
  winds"): a separatrix σ of a singular vertex v that stops on another, τ,
  at a distance δ(ℓ) from v along τ with shift ≡ ±δ modulo the balances,
  and the arcs of δ equal, modulo the balances, to an arc held open.

    python3 docs/research/turns-of-a-t-mesh/experiments/pairs.py [<export>] \\
        [--table docs/research/turns-of-a-t-mesh/experiments/results/conflicts.json] [--largest 4] [--depth 8]

Identities are checked in floating point, on the balanced lengths (a basis
of them); homotopies are not checked. Seconds to minutes a row. Writes
results/pairs.json.
"""

import argparse
import itertools
import json
import time
from collections import defaultdict
from fractions import Fraction
from pathlib import Path

import tmesh  # first: it sets the threads of the linear algebra

import numpy as np
from geometry import Layout, dense, shift_form, staircases, windings


def traces(L):
    """Each trace as the list of its arcs from where it starts."""
    out = defaultdict(list)
    for a, arc in enumerate(L.arcs):
        out[arc["trace"]].append(a)
    for t in out:
        out[t].sort(key=lambda a: L.arcs[a]["span"][0])
    return out


def separatrix(L, way, shift, bounded):
    """Separatrices σ, τ of one singular vertex v, σ stopping on τ at δ(ℓ)
    from v, with the shift ≡ ±δ modulo the balances."""
    n = len(L.arcs)
    s = dense(shift, n)
    chains = traces(L)
    starting = defaultdict(list)
    for t, arcs in chains.items():
        first = L.arcs[arcs[0]]
        if L.singular(first["nodes"][0]):
            starting[first["nodes"][0]].append(t)
    found = []
    for v, ts in starting.items():
        for sigma, tau in itertools.permutations(ts, 2):
            end = L.arcs[chains[sigma][-1]]["nodes"][1]
            delta = []
            for a in chains[tau]:
                delta.append(a)
                if L.arcs[a]["nodes"][1] == end:
                    break
            else:
                continue
            form = dense({a: 1 for a in delta}, n)
            for sign in (1, -1):
                if L.same_mod_balances(s, sign * form):
                    equal = [[a, b] for a in delta for b in bounded if L.same_mod_balances({a: 1}, {b: 1})]
                    found.append({"vertex": v, "valence": L.nodes[v]["angle"], "sigma": sigma, "tau": tau,
                                  "sigma_length": round(float(sum(L.exact[a] for a in chains[sigma])), 3),
                                  "delta_arcs": delta, "delta": round(float(sum(L.exact[a] for a in delta)), 3),
                                  "sign": sign, "delta_arc_equal_to_a_held_arc": equal})
    return found


def describe(L, way, row, depth):
    turns = row["turns"]
    ids = [way.held_index[h] for h in turns]
    u = [Fraction(x) for x in row["primitive_multipliers"]]
    n = len(L.arcs)
    combination = sum((int(w) * dense(shift_form(L.turns[t]), n) for w, t in zip(u, ids)), np.zeros(n, np.int64))
    form = {int(a): Fraction(x) for a, x in row["form"].items()}
    bounded = row["bounded"]
    out = {"way": int(next(iter(row["turns_in_each_way"]))), "turns": turns, "multipliers": row["multipliers"],
           "primitive_multipliers": row["primitive_multipliers"], "bounded": bounded,
           "self_crossings": [L.crossings_between(t, t) for t in ids],
           "mutual_crossings": {f"{a}-{b}": L.crossings_between(ids[i], ids[j])
                                for (i, a), (j, b) in itertools.combinations(enumerate(turns), 2)},
           "around": [round(float(L.turns[t]["around"]), 2) for t in ids]}
    out["simple"] = [c == 0 for c in out["self_crossings"]]
    if len(ids) == 1:
        out["separatrices"] = separatrix(L, way, shift_form(L.turns[ids[0]]), bounded)
        regions, sides, find, node_cell = L.split(ids)
        keys = [k for k in regions if k is not None]
        small = min(keys, key=lambda k: sum(1 for v in regions[k] if L.singular(v)))
        out["smaller_side"] = {"index": sum(L.index(v) for v in regions[small]),
                               "singular": [[v, L.nodes[v]["angle"]] for v in regions[small] if L.singular(v)]}
        return out
    winding, regions, inconsistent = windings(L, ids, u)
    between = []
    for root, w in sorted(winding.items(), key=lambda item: -abs(item[1])):
        if w == 0:
            continue
        singular = [v for v in regions[root] if L.singular(v)]
        between.append({"winding": str(w), "index": sum(L.index(v) for v in regions[root]),
                        "singular": [[v, L.nodes[v]["angle"]] for v in singular] if len(singular) <= 12
                        else f"{len(singular)} singular vertices"})
    out["regions"] = len(regions)
    out["winding_inconsistencies"] = inconsistent
    out["between"] = between
    vertices = [v for region in between if isinstance(region["singular"], list) for v, _ in region["singular"]]
    found = []
    if len(vertices) <= 12:
        for a, b in itertools.combinations(sorted(set(vertices)), 2):
            for directions, arcs in staircases(L, a, b, depth):
                length = dense({arc: 1 for arc in arcs}, n)
                for sign in (1, -1):
                    if L.same_mod_balances(sign * combination, length):
                        found.append({"from": a, "to": b, "valences": [L.nodes[a]["angle"], L.nodes[b]["angle"]],
                                      "arcs": arcs, "length": round(float(sum(L.exact[c] for c in arcs)), 3),
                                      "held_arc_on_it": [c for c in arcs if way.b[c] > 0], "sign": sign})
    seen, unique = set(), []
    for item in sorted(found, key=lambda item: len(item["arcs"])):
        key = tuple(sorted(item["arcs"]))
        if key not in seen:
            seen.add(key)
            unique.append(item)
    out["staircases"] = unique[:6]
    if unique:
        best = unique[0]
        out["mechanism"] = "dislocation" if sorted(best["valences"]) == [3, 5] else "exchange"
        out["held_arc"] = "on the staircase" if best["held_arc_on_it"] else "by the balances"
    else:
        out["mechanism"] = "no staircase found"
    out["certificate_form"] = {str(a): str(x) for a, x in sorted(form.items())}
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "pairs")
    parser.add_argument("--table", type=Path,
                        default=tmesh.HERE / "results" / "conflicts.json", help="the summary of conflicts.py")
    parser.add_argument("--largest", type=int, default=4, help="rows of at most this many turns")
    parser.add_argument("--depth", type=int, default=8, help="the longest staircase searched, in arcs, less one")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    table = json.loads(arguments.table.read_text())
    folder, paths = tmesh.ways(arguments.export, arguments.call)
    for number, sha in table["ways"].items():
        if int(number) not in paths or tmesh.sha256(paths[int(number)]) != sha:
            raise SystemExit(f"{arguments.table} was not made from this export (way {number})")
    layouts, ways, rows = {}, {}, []
    for row in table["table"]:
        if row["size"] > arguments.largest:
            continue
        number = int(next(iter(row["turns_in_each_way"])))
        if number not in layouts:
            ways[number] = tmesh.Way(paths[number])
            layouts[number] = Layout(ways[number])
        tick = time.monotonic()
        out = describe(layouts[number], ways[number], row, arguments.depth)
        out["ways"] = row["ways"]
        out["seconds"] = round(time.monotonic() - tick, 1)
        rows.append(out)
        if "separatrices" in out:
            text = "; ".join(f"σ {s['sigma']} of {s['vertex']} (valence {s['valence']}), {s['sigma_length']} long, "
                             f"stops on τ {s['tau']} at δ {s['delta']} = arcs {s['delta_arcs']}, shift = "
                             f"{'+' if s['sign'] > 0 else '-'}δ; held arc equal {s['delta_arc_equal_to_a_held_arc']}"
                             for s in out["separatrices"]) or "no separatrix found"
            print(f"way {out['way']} turn {out['turns']}: {text}", flush=True)
        else:
            stair = out["staircases"][0] if out["staircases"] else None
            print(f"way {out['way']} turns {out['turns']} u {out['primitive_multipliers']}: self-crossings "
                  f"{out['self_crossings']}, crossings {out['mutual_crossings']}; between {out['between']}; "
                  + (f"staircase {stair['arcs']} from {stair['from']} to {stair['to']} (valences "
                     f"{stair['valences']}), {stair['length']}, held arc {out['held_arc']}: {out['mechanism']}"
                     if stair else "no staircase found") + f" ({out['seconds']} s)", flush=True)
    summary = {
        "experiment": "the geometry of the conflicts (Table tab:pairs)",
        **tmesh.record(arguments, folder, list(ways.values()), started, clock, depth=arguments.depth,
                       table=str(arguments.table.name)),
        "rows": rows,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
