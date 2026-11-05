"""Can turns of a T-mesh of genus zero carry the reduction of Theorem
"Choosing the turns is NP-hard"? (#216)

    python3 -I docs/research/turns-of-a-t-mesh/experiments/gadget.py --count 60 --seed 1 \\
        --families box:2:2:6:2,box:3:3:8:3,box:3:3:10:6,box:4:4:10:8,box:3:3:12:10,cube:6,hull:12,dual:12

Three checks on the graphs K4, the triangular prism, the cube and the
pentagonal prism (planar, every vertex of degree three).

1. Differences. When the shifts of a set of turns are bound only by
   differences, `x_γ − x_γ' ≥ c` (Corollary "Chains of dislocations": the
   shifts of consecutive turns of a chain differ by a staircase held at one at
   least), a set closes exactly when the difference system with its shifts at
   zero is feasible (Bellman–Ford in whole numbers). For every orientation of
   the edges of a graph, with `x_u − x_v ≥ 1` on each arc `u → v`, the
   closable sets are compared with the independent sets; and the same for one
   chain, its turns in a row with the sign of each staircase. The implication
   classes of Gallai (Golumbic, Theorem 5.4) say again which graphs have such
   an orientation (the comparability graphs).
2. The reduction's own instance (bidirected balances, Lemma
   `closable_iff_independent`) on the same graphs, exactly: whole lengths
   closing every maximal independent set, and a Farkas certificate for every
   edge.
3. Random T-meshes of genus zero (the spheres of `property/generate.py`: cubes,
   boxes, hulls and their duals, rebuilt from the seed): `P` the balanced
   lengths with every arc at least one, `H` the strips of the whole lengths
   and the turns walked at the real lengths with a non-zero shift form, each
   form once up to sign (as `property/certificates.py`). The pairs of turns
   that cannot close together within `P` make the graph of conflicts on the
   turns closable alone; it is searched for the graphs above as induced
   subgraphs. A set of turns found is checked exactly: whole lengths in `P`
   closing each maximal independent set (`Check/certify.py`, `closing`), and
   an exact certificate for each edge (`certificate`, `valid`). Then its
   closable sets are the independent sets of the graph. The graph of
   conflicts is also tested for being a comparability graph.

What it does not do: build a T-mesh for a given graph (a gadget); a set found
on a random instance shows that such sets exist, not that every graph has one.
Floating point (HiGHS through SciPy) decides the conflicts during the search;
only the sets reported are checked exactly.
"""

import argparse
import itertools
import json
import os
import platform
import random
import sys
import time
from fractions import Fraction

import numpy as np
import scipy
from scipy.optimize import linprog
from scipy.sparse import coo_matrix, csr_matrix, hstack, identity, vstack

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "property"))
sys.path.insert(0, os.path.join(HERE, "..", "lean", "Check"))

import certify  # noqa: E402
import export  # noqa: E402
import generate  # noqa: E402
import tracing  # noqa: E402

GRAPHS = {
    "K4": (4, [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
    "prism": (6, [(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3), (0, 3), (1, 4), (2, 5)]),
    "cube": (8, [(a, b) for a in range(8) for b in range(a + 1, 8) if bin(a ^ b).count("1") == 1]),
    "pentagonal prism": (10, [(i, (i + 1) % 5) for i in range(5)] + [(5 + i, 5 + (i + 1) % 5) for i in range(5)]
                         + [(i, i + 5) for i in range(5)]),
}

SPHERES = [kind for kind, weight in generate.FAMILIES for _ in range(weight)
           if kind[0] in ("cube", "box", "hull", "dual")]


# --- graphs -------------------------------------------------------------------------------

def adjacency(n, edges):
    adjacent = [set() for _ in range(n)]
    for a, b in edges:
        adjacent[a].add(b)
        adjacent[b].add(a)
    return adjacent


def independent_sets(n, edges):
    adjacent = adjacency(n, edges)
    out = []
    for mask in range(1 << n):
        members = [v for v in range(n) if mask >> v & 1]
        if all(b not in adjacent[a] for a, b in itertools.combinations(members, 2)):
            out.append(frozenset(members))
    return out


def maximal(sets):
    return [s for s in sets if not any(s < t for t in sets)]


def feasible_differences(n, arcs, zero):
    """Whether x_u − x_v ≥ w on every arc (u, v, w) has a solution with x = 0
    on `zero`: the vertices of `zero` are made one, then Bellman–Ford for the
    heaviest walks; a positive cycle means no solution. Whole numbers only."""
    node = list(range(n))
    root = min(zero) if zero else None
    for v in zero:
        node[v] = root
    distance = {v: 0 for v in set(node)}
    contracted = [(node[u], node[v], w) for u, v, w in arcs]
    for _ in range(len(distance)):
        changed = False
        for u, v, w in contracted:
            # x_u ≥ x_v + w: the heaviest walk into u.
            if distance[v] + w > distance[u]:
                distance[u] = distance[v] + w
                changed = True
        if not changed:
            return True
    return False


def same_closable_sets(n, arcs, independent):
    """Whether the closable sets of the difference system are exactly the
    independent sets, every subset tested, smallest first (a difference stops
    the test)."""
    for size in range(n + 1):
        for members in itertools.combinations(range(n), size):
            if feasible_differences(n, arcs, set(members)) != (frozenset(members) in independent):
                return False
    return True


def comparability(n, edges):
    """Gallai's implication classes: G is a comparability graph if and only if
    no class holds an oriented edge and its reverse (Golumbic, Theorem 5.4)."""
    adjacent = adjacency(n, edges)
    oriented = [(a, b) for a, b in edges] + [(b, a) for a, b in edges]
    parent = {e: e for e in oriented}

    def find(e):
        while parent[e] != e:
            parent[e] = parent[parent[e]]
            e = parent[e]
        return e

    for a in range(n):
        for b, c in itertools.combinations(sorted(adjacent[a]), 2):
            if c not in adjacent[b]:
                # a b Γ a c, and b a Γ c a.
                parent[find((a, b))] = find((a, c))
                parent[find((b, a))] = find((c, a))
    return all(find((a, b)) != find((b, a)) for a, b in edges)


def differences(name, n, edges):
    independent = set(independent_sets(n, edges))
    working = []
    for mask in range(1 << len(edges)):
        arcs = [((a, b) if not mask >> k & 1 else (b, a)) + (1,) for k, (a, b) in enumerate(edges)]
        if same_closable_sets(n, arcs, independent):
            working.append([[u, v] for u, v, _ in arcs])
    # One chain: turns 0..n−1 in a row, consecutive shifts differing by a
    # staircase of sign ε_i held at one at least; which graphs of conflicts.
    chain_graphs = set()
    for signs in itertools.product((1, -1), repeat=n - 1):
        arcs = [((i, i + 1, 1) if s > 0 else (i + 1, i, 1)) for i, s in enumerate(signs)]
        pairs = frozenset((i, j) for i, j in itertools.combinations(range(n), 2)
                          if not feasible_differences(n, arcs, {i, j}))
        chain_graphs.add(pairs)
    chain = any(isomorphic(n, edges, sorted(g)) for g in chain_graphs)
    return {"graph": name, "vertices": n, "edges": len(edges), "independent_sets": len(independent),
            "orientations": 1 << len(edges), "orientations_whose_closable_sets_are_the_independent_sets":
            len(working), "example": working[0] if working else None,
            "comparability_graph_by_implication_classes": comparability(n, edges),
            "one_chain_realizes_it": chain}


def bipartite(host, vertices):
    colour = {}
    for start in vertices:
        if start in colour:
            continue
        colour[start] = 0
        queue = [start]
        while queue:
            a = queue.pop()
            for b in host[a]:
                if b not in colour:
                    colour[b] = 1 - colour[a]
                    queue.append(b)
                elif colour[b] == colour[a]:
                    return False
    return True


def isomorphic(n, edges, other):
    if len(edges) != len(other):
        return False
    target = set(map(frozenset, edges))
    degrees = sorted(len(s) for s in adjacency(n, edges))
    if degrees != sorted(len(s) for s in adjacency(n, other)):
        return False
    for permutation in itertools.permutations(range(n)):
        if all(frozenset((permutation[a], permutation[b])) in target for a, b in other):
            return True
    return False


# --- the reduction's own instance ----------------------------------------------------------

def reduction(name, n, edges):
    """Arcs y_{u,e}, y_{v,e}, d_e, d'_e per edge; rows y_u + y_v − d, y_u + y_v − d',
    d − d'; bounds d, d' ≥ 1; shift_u = Σ_{e ∋ u} y_{u,e}. Exact witnesses and
    certificates."""
    arcs = lambda k: (4 * k, 4 * k + 1, 4 * k + 2, 4 * k + 3)
    rows, lower = [], [0, 0, 1, 1] * len(edges)
    for k in range(len(edges)):
        yu, yv, d, d2 = arcs(k)
        rows += [{yu: 1, yv: 1, d: -1}, {yu: 1, yv: 1, d2: -1}, {d: 1, d2: -1}]
    shift = [{} for _ in range(n)]
    for k, (a, b) in enumerate(edges):
        shift[a][arcs(k)[0]] = 1
        shift[b][arcs(k)[1]] = 1
    independent = independent_sets(n, edges)
    checked = 0
    for Y in maximal(independent):
        length = []
        for k, (a, b) in enumerate(edges):
            yu, yv = (0, 1) if a in Y else (1, 0)
            length += [yu, yv, 1, 1]
        assert all(sum(c * length[x] for x, c in row.items()) == 0 for row in rows)
        assert all(x >= b for x, b in zip(length, lower))
        assert all(sum(c * length[x] for x, c in shift[u].items()) == 0 for u in Y)
        checked += 1
    for k, (a, b) in enumerate(edges):
        # shift_a + shift_b − (first row of e) = d_e + the other y: non-negative, d_e held.
        c = {}
        for form in (shift[a], shift[b]):
            for x, v in form.items():
                c[x] = c.get(x, 0) + v
        for x, v in rows[3 * k].items():
            c[x] = c.get(x, 0) - v
        assert all(v >= 0 for v in c.values()) and sum(v * lower[x] for x, v in c.items()) > 0
    return {"graph": name, "maximal_independent_sets_closed_exactly": checked,
            "edges_with_exact_certificate": len(edges)}


# --- random T-meshes of genus zero ---------------------------------------------------------

def instance(seed, kind, work, shares):
    rng = random.Random(seed)
    quad, T, whole, real, turns, held, strips, basis = generate.make(rng, kind, shares, 0.12, 6.0)
    forms = []
    for turn in strips + turns:
        shift = turn["shift"]
        if not shift:
            continue
        negated = {a: -c for a, c in shift.items()}
        if shift not in forms and negated not in forms:
            forms.append(shift)
    name = f"{kind[0]}-{seed}"
    way = export.way_json(T, whole, real, strips + turns, forms, shares, 0.12, 6.0, name, lower=1)
    folder = export.write(work, name, way, export.helices_json(strips, whole))
    return name, quad, T, certify.Way(os.path.join(folder, "way-0.json"))


def zero_sets(W, H, rounds, rng):
    """Sets of turns closed together by one length each: the turns at zero after
    minimising Σ w_γ |shift_γ| over P with random weights (floating point)."""
    n, k = W.n, len(H)
    S = W.H(H)
    A = vstack([hstack([W.B, coo_matrix((W.B.shape[0], 2 * k))]),
                hstack([S, -identity(k), identity(k)])]).tocsr()
    bounds = list(zip(W.b, [None] * n)) + [(0, None)] * (2 * k)
    out = []
    for _ in range(rounds):
        w = np.array([rng.choice([1.0, 1.0, 4.0, 16.0]) for _ in range(k)])
        r = linprog(np.concatenate([np.zeros(n), w, w]), A_eq=A, b_eq=np.zeros(A.shape[0]), bounds=bounds,
                    method="highs")
        if r.status != 0:
            continue
        values = S @ r.x[:n]
        out.append({H[i] for i in range(k) if abs(values[i]) <= 1e-7})
    return out


def conflicts(W, H, rng, rounds):
    alone = [k for k in H if W.feasible([k])]
    together = set()
    for zero in zero_sets(W, alone, rounds, rng):
        for a, b in itertools.combinations(sorted(zero), 2):
            together.add((a, b))
    edges = set()
    tested = 0
    for a, b in itertools.combinations(sorted(alone), 2):
        if (a, b) in together:
            continue
        tested += 1
        if not W.feasible([a, b]):
            edges.add((a, b))
    return alone, edges, tested


def induced(pattern_n, pattern_edges, host, vertices, limit):
    """Induced copies of the pattern in the host graph (adjacency sets), by
    backtracking, at most `limit` of them, each vertex set once."""
    pattern = adjacency(pattern_n, pattern_edges)
    order = [0]
    while len(order) < pattern_n:
        rest = [v for v in range(pattern_n) if v not in order]
        order.append(max(rest, key=lambda v: len(pattern[v] & set(order))))
    found, seen = [], set()

    def extend(assigned):
        if len(found) >= limit:
            return
        if len(assigned) == pattern_n:
            key = frozenset(assigned.values())
            if key not in seen:
                seen.add(key)
                found.append([assigned[v] for v in range(pattern_n)])
            return
        v = order[len(assigned)]
        used = set(assigned.values())
        for h in vertices:
            if h in used:
                continue
            if all((h in host[assigned[u]]) == (u in pattern[v]) for u in assigned):
                assigned[v] = h
                extend(assigned)
                del assigned[v]

    extend({})
    return found


def check_exactly(W, n, edges, turns):
    """Whole lengths closing each maximal independent set, an exact certificate
    for each edge: then the closable subsets of `turns` are the independent sets."""
    report = {"maximal_independent_sets": [], "edges": []}
    for Y in maximal(independent_sets(n, edges)):
        chosen = [turns[v] for v in sorted(Y)]
        lengths = W.closing(chosen, seconds=20) if chosen else list(W.engine)
        report["maximal_independent_sets"].append({"vertices": sorted(Y), "closed_exactly":
                                                   lengths is not None and W.closes(chosen, lengths)})
    for a, b in edges:
        pair = [turns[a], turns[b]]
        found = W.certificate(pair)
        exact = found is not None and W.valid(pair, *found)
        report["edges"].append({"ends": [a, b], "certificate_exact": exact,
                                "multipliers": [str(x) for x in found[0]] if found else None,
                                "arcs": sorted(found[1]) if found else None})
    report["holds"] = (all(x["closed_exactly"] for x in report["maximal_independent_sets"])
                       and all(x["certificate_exact"] for x in report["edges"]))
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--count", type=int, default=60, help="random spheres searched")
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--rounds", type=int, default=40, help="programs with random weights per instance")
    parser.add_argument("--keep", type=int, default=3, help="sets checked exactly per graph")
    parser.add_argument("--families", default=None,
                        help="the spheres drawn from, as 'kind:size:...' separated by commas, e.g. "
                             "'box:3:3:10:6,cube:4'; default: the spheres of generate.py")
    parser.add_argument("--work", default=os.path.join("scratch", "data-package", "turns-of-a-t-mesh", "gadget"))
    parser.add_argument("--summary", default=os.path.join(HERE, "results", "gadget.json"))
    args = parser.parse_args()
    started = time.time()
    summary = {"experiment": "#216: differences, the reduction's instance, and the graphs of conflicts of "
                             "random T-meshes of genus zero against K4, the prism, the cube and the pentagonal prism",
               "machine": {"platform": platform.platform(), "python": platform.python_version(),
                           "numpy": np.__version__, "scipy": scipy.__version__,
                           "load_at_start": list(os.getloadavg())}}
    summary["differences"] = [differences(name, n, edges) for name, (n, edges) in GRAPHS.items()]
    summary["reduction"] = [reduction(name, n, edges) for name, (n, edges) in GRAPHS.items()]
    for row in summary["differences"]:
        print(f"{row['graph']}: {row['orientations_whose_closable_sets_are_the_independent_sets']} of "
              f"{row['orientations']} orientations, comparability {row['comparability_graph_by_implication_classes']}, "
              f"one chain {row['one_chain_realizes_it']}")

    spheres = SPHERES
    if args.families:
        spheres = [tuple([p[0]] + [int(x) for x in p[1:]]) for p in (t.split(":") for t in args.families.split(","))]
    rng = random.Random(args.seed)
    shares = [Fraction(1, 2)]
    instances, rejected = [], 0
    found = {name: [] for name in GRAPHS}
    while len(instances) < args.count:
        kind = rng.choice(spheres)
        seed = rng.randrange(1 << 30)
        try:
            name, quad, T, W = instance(seed, kind, args.work, shares)
        except tracing.NotRectangles:
            rejected += 1
            continue
        assert generate.genus(quad) == 0
        alone, edges, tested = conflicts(W, list(W.held), random.Random(seed), args.rounds)
        host = {k: set() for k in alone}
        for a, b in edges:
            host[a].add(b)
            host[b].add(a)
        vertices = sorted(alone)
        record = {"name": name, "kind": list(kind), "seed": seed, "arcs": W.n, "turns_in_H": len(W.held),
                  "closable_alone": len(alone), "pairs_tested_by_a_program": tested, "conflicts": len(edges),
                  "comparability": comparability(len(vertices), [(vertices.index(a), vertices.index(b))
                                                                 for a, b in edges]) if vertices else True,
                  "bipartite": bipartite(host, vertices),
                  "triangles": sum(1 for a, b in edges for c in host[a] & host[b] if c > max(a, b)),
                  "induced": {}}
        for gname, (n, gedges) in GRAPHS.items():
            copies = induced(n, gedges, host, vertices, 1000)
            record["induced"][gname] = len(copies)
            for turns in copies:
                if len(found[gname]) >= args.keep:
                    break
                check = check_exactly(W, n, gedges, turns)
                found[gname].append({"instance": name, "kind": list(kind), "seed": seed, "turns": turns,
                                     "forms": [{str(a): c for a, c in sorted(W.shift[t].items())} for t in turns],
                                     **check})
        instances.append(record)
        print(f"{name}: {len(alone)} turns closable alone, {len(edges)} conflicts, comparability "
              f"{record['comparability']}, bipartite {record['bipartite']}, {record['triangles']} triangles, "
              f"induced {record['induced']}", flush=True)
    summary.update({
        "seed": args.seed, "count": args.count, "rejected": rejected, "bound": "every arc at least one",
        "families": [list(k) for k in dict.fromkeys(spheres)], "rounds": args.rounds,
        "seconds": round(time.time() - started, 1),
        "instances_with_a_conflict": sum(1 for i in instances if i["conflicts"]),
        "graphs_of_conflicts_not_comparability": sum(1 for i in instances if not i["comparability"]),
        "graphs_of_conflicts_not_bipartite": sum(1 for i in instances if not i["bipartite"]),
        "instances_with_a_triangle": sum(1 for i in instances if i["triangles"]),
        "instances_with_an_induced_copy": {g: sum(1 for i in instances if i["induced"][g]) for g in GRAPHS},
        "checked_exactly": {g: [{k: v for k, v in x.items() if k in ("instance", "turns", "holds")}
                                for x in found[g]] for g in GRAPHS},
        "found": found, "instances": instances,
    })
    os.makedirs(os.path.dirname(args.summary), exist_ok=True)
    with open(args.summary, "w") as f:
        json.dump(summary, f, indent=1)
    for key in ("instances_with_a_conflict", "graphs_of_conflicts_not_comparability",
                "graphs_of_conflicts_not_bipartite", "instances_with_a_triangle",
                "instances_with_an_induced_copy", "checked_exactly"):
        print(f"{key}: {summary[key]}")
    print(f"{len(instances)} spheres, {rejected} rejected, {summary['seconds']} s, summary in {args.summary}")


if __name__ == "__main__":
    main()
