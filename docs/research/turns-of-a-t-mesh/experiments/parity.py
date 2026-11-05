#!/usr/bin/env python3
"""Feet modulo 2 (Section "Combs", Proposition "Feet modulo 2"): the
hypotheses on the T-mesh (every trace leaves a vertex, every other node is a
junction met by exactly two traces, no trace ends at a vertex); for every
vertex v (a singular vertex or a node of valence four that traces leave) the
lengths chi_v, the number of feet of each arc at v, and the node function
g_v (the traces from v through a node other than a vertex); the arcs where
chi_v is not the coboundary of g_v modulo 2 and the balances it breaks
modulo 2; chi_v read by the half-sum of the balances (odd exactly at the
vertices of odd valence) and, modulo 2, by the shift form of every turn of
every way; then the boundary of the odd part of every certificate of
`../lean/Check/certificates/male-body.json`, scaled to whole numbers.

    python3 docs/research/turns-of-a-t-mesh/experiments/parity.py [<export>] \\
        [--certificates docs/research/turns-of-a-t-mesh/lean/Check/certificates/male-body.json]

Seconds, one thread. Writes results/parity.json.
"""

import argparse
import json
import math
import time
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path

import tmesh  # first: it sets the threads of the linear algebra
from combs import Feet
from geometry import Layout


def node_function(L, F, vertices, v):
    """g_v: at each node other than a vertex, the number of traces through
    it traced from v."""
    g = Counter()
    for t, arcs in F.trace_arcs.items():
        if F.origin[t] != v:
            continue
        for n in {n for a in arcs for n in L.arcs[a]["nodes"]}:
            if n not in vertices:
                g[n] += 1
    return g


def reads(form, chi):
    """The whole form at the lengths chi, modulo 2."""
    return sum(c * chi[a] for a, c in form.items() if chi.get(a)) % 2


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "parity")
    parser.add_argument("--certificates", type=Path,
                        default=tmesh.HERE.parent / "lean" / "Check" / "certificates" / "male-body.json",
                        help="the certificates checked by Lean")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, paths = tmesh.ways(arguments.export, arguments.call)
    ways = {k: tmesh.Way(p) for k, p in paths.items()}
    by_sha = {w.sha: w for w in ways.values()}

    # The T-mesh is the same in every way: its feet are read on way 1.
    way = ways[1] if 1 in ways else next(iter(ways.values()))
    for other in ways.values():
        assert [a["nodes"] for a in other.data["arcs"]] == [a["nodes"] for a in way.data["arcs"]]
    L = Layout(way)
    F = Feet(L)
    feet = {a: F.comb(a)[:2] for a in range(way.n)}
    vertices = sorted({n for n in range(len(L.nodes)) if L.singular(n)} | set(F.origin.values()))
    valence = {v: len(L.nodes[v]["halves"]) for v in vertices}
    traces_at = {n: Counter(L.arcs[h // 2]["trace"] for h in L.nodes[n]["halves"]) for n in range(len(L.nodes))}
    singular = [v for v in vertices if L.singular(v)]
    ends = {t: L.arcs[arcs[-1]]["nodes"][1] for t, arcs in F.trace_arcs.items()}
    hypotheses = {
        "vertices": len(vertices),
        "singular_vertices": len(singular),
        "vertices_of_valence": dict(sorted(Counter(valence.values()).items())),
        "junctions_not_met_by_exactly_two_traces": [
            n for n in range(len(L.nodes)) if n not in vertices and len(traces_at[n]) != 2],
        "traces_ending_at_a_singular_vertex": sorted(t for t, n in ends.items() if L.singular(n)),
        "traces_from_a_singular_vertex_ending_at_a_vertex": sorted(
            t for t, n in ends.items() if L.singular(F.origin[t]) and n in vertices),
        "traces_joining_two_vertices_of_valence_four": sorted(
            t for t, n in ends.items() if n in vertices and not L.singular(n) and not L.singular(F.origin[t])),
    }
    print(", ".join(f"{k.replace('_', ' ')}: {v}" for k, v in hypotheses.items()), flush=True)

    half = Counter()
    for row in way.rows:
        for a, c in row.items():
            half[a] += c
    assert all(c % 2 == 0 for c in half.values())
    half = {a: c // 2 for a, c in half.items() if c}

    chi = {}
    coboundary_fails, balance_fails, half_sum_odd = defaultdict(list), defaultdict(list), []
    for v in singular:
        chi[v] = {a: (c0 == v) + (c1 == v) for a, (c0, c1) in feet.items() if v in (c0, c1)}
        g = node_function(L, F, vertices, v)
        for a in range(way.n):
            n0, n1 = L.arcs[a]["nodes"]
            if (chi[v].get(a, 0) - g[n0] - g[n1]) % 2:
                coboundary_fails[v].append(a)
        for k, row in enumerate(way.rows):
            if reads(row, chi[v]):
                balance_fails[v].append(k)
        if reads(half, chi[v]):
            half_sum_odd.append(v)
    odd_valence = [v for v in singular if valence[v] % 2]
    print(f"chi_v not the coboundary of g_v at {sum(map(len, coboundary_fails.values()))} (vertex, arc) pairs; "
          f"balances broken modulo 2 at {sum(map(len, balance_fails.values()))} (vertex, balance) pairs; "
          f"the half-sum of the balances reads chi_v odd at {len(half_sum_odd)} vertices, "
          f"{'exactly' if half_sum_odd == odd_valence else 'not'} those of odd valence ({len(odd_valence)})",
          flush=True)

    turns = {}
    for k, w in ways.items():
        odd = [(t, v) for t, shift in enumerate(w.shift) for v in singular if reads(shift, chi[v])]
        turns[str(k)] = {"turns": len(w.shift), "odd_readings": odd}
        print(f"way {k}: {len(w.shift)} turns, chi_v read odd by a shift at {len(odd)} (turn, vertex) pairs",
              flush=True)

    certificates = []
    data = json.loads(arguments.certificates.read_text())
    for sha, entry in data["ways"].items():
        w = by_sha.get(sha)
        if w is None:
            raise SystemExit(f"{arguments.certificates}: way {entry['way']} ({sha[:12]}) is not in {folder}")
        for s in entry["sets"]:
            c = {a: Fraction(p, q) for a, p, q in s["c"]}
            # Whole multipliers, and the certificate scaled with them.
            scale = math.lcm(*(q for _, q in s["u"]), *(x.denominator for x in c.values()))
            whole = {a: int(x * scale) for a, x in c.items()}
            boundary = [v for v in singular if reads(whole, chi[v])]
            certificates.append({
                "way": w.number, "row": s["row"], "turns": s["turns"], "arcs": len(c), "scale": scale,
                "odd_arcs": sum(1 for x in whole.values() if x % 2), "odd_boundary": boundary,
                "feet_of_odd_valence": all(valence.get(f, 0) % 2 for a in c for f in feet[a]),
            })
    distinct = {json.dumps(s["c"]) for e in data["ways"].values() for s in e["sets"]}
    print(f"{len(certificates)} certificates ({len(distinct)} distinct forms, "
          f"{sum(1 for c in certificates if c['scale'] == 1)} with whole multipliers, the others scaled): odd part a cycle in "
          f"{sum(1 for c in certificates if not c['odd_boundary'])}; with odd arcs "
          f"{sum(1 for c in certificates if c['odd_arcs'])}; feet all of odd valence in "
          f"{sum(1 for c in certificates if c['feet_of_odd_valence'])}; at most "
          f"{max(c['odd_arcs'] for c in certificates)} odd arcs", flush=True)

    summary = {
        "experiment": "parity: chi_v, the coboundary of g_v modulo 2, read by the shifts and the certificates",
        **tmesh.record(arguments, folder, list(ways.values()), started, clock,
                       certificates=arguments.certificates.name),
        "hypotheses": hypotheses,
        "coboundary_fails": {str(v): a for v, a in coboundary_fails.items()},
        "balances_broken": {str(v): k for v, k in balance_fails.items()},
        "half_sum_odd_at": half_sum_odd,
        "half_sum_odd_exactly_at_odd_valence": half_sum_odd == odd_valence,
        "turns": turns,
        "certificates": certificates,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
