#!/usr/bin/env python3
"""Question "Are conflicts sums of local dislocations?" read strictly (Section
"Combs", paragraph "Question ..., read loosely, is true and empty"): is the
certificate of each set of the table of `conflicts.py` a non-negative
combination, modulo the balances, of the strict local forms?

- diagonal walks between two singular vertices of odd valence (staircases,
  weight one on every step) and closed diagonal walks (caps);
- closed axial walks (caps), and axial walks from a singular vertex of odd
  valence back to itself (winding separatrices), counting the steps along
  one direction;
- with --loose, axial walks between two odd vertices, with weight two.

These forms are infinitely many, but each is the length of a walk in a
graph whose steps carry the arc lengths: the linear program min c · x over
balanced x in [-1, 1]^n with every form generated so far non-negative is
solved, and shortest-path searches (Bellman–Ford for closed walks, Johnson
for the others) add every form negative at its solution, until there is none
(x separates c from the cone: the reading is refuted for c) or the optimum
is 0 (c is in the cone, its decomposition read from the duals). A separating
x is then made exact: pushed by a thousandth of the real lengths, its free
arcs (Proposition "Free arcs") rounded to multiples of 10⁻⁸, the others
solved in rational arithmetic from the balances, scaled to whole numbers;
the certificate's value there is exact, and the search for a negative form is
run again in integer arithmetic.

    python3 docs/research/turns-of-a-t-mesh/experiments/strict.py [<export>] [--rows 1,2,7] [--loose]

Seconds to minutes a set (the largest sets longer). Writes results/strict.json.
"""

import argparse
import json
import math
import time
from collections import defaultdict
from fractions import Fraction
from pathlib import Path

import tmesh  # first: it sets the threads of the linear algebra

import numpy as np
from geometry import Layout, directions_at
from scipy.sparse import csr_matrix, vstack
from scipy.sparse.csgraph import shortest_path


class Mesh:
    """What the walk graphs need of a way: its layout and balances."""

    def __init__(self, way):
        self.way = way
        self.L = Layout(way)
        self.n = way.n
        self.B = way.B

    def singular(self, v):
        return self.L.singular(v)

    def valence(self, v):
        return self.L.nodes[v]["angle"]


def turning(M):
    """For each half whose head is a T-junction, the halves that leave it
    and the turn to each (0 straight, 1 left, 3 right, in quarter turns)."""
    L = M.L
    out = {}
    for h in range(2 * M.n):
        m = L.head(h)
        if M.singular(m):
            continue
        directions = directions_at(L, m, h ^ 1, 2)
        out[h] = [(g, directions[g] % 4) for g in L.nodes[m]["halves"] if g != h ^ 1]
    return out


class WalkGraph:
    """Walks along arcs through T-junctions only. `diagonal`: steps in two
    directions a quarter turn apart (phases: chirality and which of the two),
    each arc counted once; `axial`: steps never against a direction e (phases
    -1, 0, +1), the steps along e counted. A state is (half, phase), weighted
    by what its arc adds to the form."""

    def __init__(self, M, kind):
        self.M, self.kind = M, kind
        L = M.L
        halves = 2 * M.n
        self.phases = [(+1, 0), (+1, 1), (-1, 0), (-1, 1)] if kind == "diagonal" else [-1, 0, 1]
        self.np = len(self.phases)
        self.nstates = halves * self.np
        src, dst = [], []
        for h, following in turning(M).items():
            for pi, phase in enumerate(self.phases):
                for g, t in following:
                    q = self.step(phase, t)
                    if q is None:
                        continue
                    src.append(h * self.np + pi)
                    dst.append(g * self.np + self.phases.index(q))
        self.src, self.dst = np.array(src), np.array(dst)
        self.coef = np.zeros(self.nstates)
        self.arc = np.zeros(self.nstates, dtype=int)
        for h in range(halves):
            for pi, phase in enumerate(self.phases):
                s = h * self.np + pi
                self.arc[s] = h // 2
                self.coef[s] = 1.0 if kind == "diagonal" else (1.0 if phase == 0 else 0.0)
        self.tail = np.array([L.tail(s // self.np) for s in range(self.nstates)])
        self.head = np.array([L.head(s // self.np) for s in range(self.nstates)])
        odd = np.array([M.singular(v) and M.valence(v) % 2 == 1 for v in range(len(L.nodes))])
        self.start = np.array([s for s in range(self.nstates) if odd[self.tail[s]] and self.first(s)])
        self.end = np.array([s for s in range(self.nstates) if odd[self.head[s]]])
        self.exact = None

    def first(self, s):
        phase = self.phases[s % self.np]
        return phase[1] == 0 if self.kind == "diagonal" else True

    def step(self, phase, t):
        t = {0: 0, 1: 1, 3: -1}[t]
        if self.kind == "diagonal":
            chirality, p = phase
            d = p * chirality + t
            if chirality == 1 and d in (0, 1):
                return (chirality, d)
            if chirality == -1 and d in (0, -1):
                return (chirality, -d)
            return None
        d = phase + t
        return d if d in (-1, 0, 1) else None

    def weights(self, x):
        """Whole weights: the lengths in millionths, plus one unit per counted
        step (so that a walk is negative only beyond rounding); or, once
        `exact` is set, the whole lengths themselves."""
        if self.exact is not None:
            return self.coef * self.exact[self.arc].astype(float)
        return np.round(self.coef * x[self.arc] * 1e6) + self.coef

    def matrix(self, x, sources=None):
        w = self.weights(x)
        src, dst, data = [self.src], [self.dst], [w[self.dst]]
        n = self.nstates
        for i, states in enumerate(sources or []):
            src.append(np.full(len(states), n + i))
            dst.append(np.asarray(states))
            data.append(w[np.asarray(states)])
        n += len(sources or [])
        return csr_matrix((np.concatenate(data), (np.concatenate(src), np.concatenate(dst))), shape=(n, n))

    def form(self, states):
        f = {}
        for s in states:
            c = self.coef[s]
            if c:
                f[int(self.arc[s])] = f.get(int(self.arc[s]), 0) + c
        return f


def find_cycle(predecessor):
    n = len(predecessor)
    colour = np.zeros(n, dtype=np.int8)
    for s0 in range(n):
        if colour[s0] or predecessor[s0] < 0:
            continue
        path, s = [], s0
        while s >= 0 and colour[s] == 0:
            colour[s] = 1
            path.append(s)
            s = predecessor[s]
        if s >= 0 and colour[s] == 1:
            cycle = path[path.index(s):]
            for p in path:
                colour[p] = 2
            return list(reversed(cycle))
        for p in path:
            colour[p] = 2
    return None


def negative_cycle(G, x, rounds=4000):
    """A closed walk of negative weight (Bellman–Ford from a source joined to
    every state), or None."""
    w = G.weights(x)
    distance = np.zeros(G.nstates)
    predecessor = np.full(G.nstates, -1)
    for it in range(rounds):
        candidate = distance[G.src] + w[G.dst]
        order = np.lexsort((candidate, G.dst))
        d_sorted, c_sorted, s_sorted = G.dst[order], candidate[order], G.src[order]
        firsts = np.r_[True, d_sorted[1:] != d_sorted[:-1]]
        bd, bc, bs = d_sorted[firsts], c_sorted[firsts], s_sorted[firsts]
        better = bc < distance[bd]
        if not better.any():
            return None
        distance[bd[better]] = bc[better]
        predecessor[bd[better]] = bs[better]
        if it % 20 == 19:
            cycle = find_cycle(predecessor)
            if cycle is not None:
                return cycle
    return find_cycle(predecessor)


def walk_from(predecessor, source, target):
    out, s = [], target
    while s != source and s >= 0:
        out.append(s)
        s = predecessor[s]
    return list(reversed(out))


class Separator:
    def __init__(self, M, loose):
        self.M, self.loose = M, loose
        self.D = WalkGraph(M, "diagonal")
        self.A = WalkGraph(M, "axial")
        L = M.L
        self.odd = [v for v in range(len(L.nodes)) if M.singular(v) and M.valence(v) % 2]
        self.A_start = {v: [s for s in self.A.start if self.A.tail[s] == v] for v in self.odd}
        self.A_end_at = {v: [s for s in self.A.end if self.A.head[s] == v] for v in self.odd}

    def violated(self, x, limit=300):
        """Forms negative at x: [(kind, form)]."""
        out = []
        for G, name in ((self.D, "closed diagonal walk"), (self.A, "closed axial walk")):
            cycle = negative_cycle(G, x)
            if cycle is not None:
                out.append((name, G.form(cycle)))
        if out:
            return out
        D = self.D
        source = D.nstates
        distance, predecessor = shortest_path(D.matrix(x, [list(D.start)]), method="J", indices=[source],
                                              return_predecessors=True)
        distance, predecessor = distance[0], predecessor[0]
        for s in sorted(D.end, key=lambda s: distance[s]):
            if distance[s] >= 0 or len(out) >= limit:
                break
            out.append(("staircase", D.form(walk_from(predecessor, source, s))))
        A = self.A
        sources = [self.A_start[v] for v in self.odd]
        indices = [A.nstates + i for i in range(len(self.odd))]
        distance, predecessor = shortest_path(A.matrix(x, sources), method="J", indices=indices,
                                              return_predecessors=True)
        for i, v in enumerate(self.odd):
            targets = self.A_end_at[v] if not self.loose else list(A.end)
            for s in targets:
                if distance[i, s] < 0 and len(out) < limit:
                    f = A.form(walk_from(predecessor[i], indices[i], s))
                    w = 1 if A.head[s] == v else 2
                    out.append(("winding separatrix" if w == 1 else "axial pair", {a: w * c for a, c in f.items()}))
        return out


def separate(M, c, loose, rounds=400):
    """('in the cone' | 'separated' | 'undecided', optimum, forms, x)."""
    n = M.n
    sep = Separator(M, loose)
    forms, rows = [], []
    r = None
    for _ in range(rounds):
        arguments = {}
        if rows:
            arguments = {"A_ub": vstack([csr_matrix(-np.array(rows))]), "b_ub": np.zeros(len(rows))}
        r = tmesh.run_linprog(c, A_eq=M.B, b_eq=np.zeros(M.B.shape[0]), bounds=(-1, 1), **arguments)
        if r.status != 0:
            raise RuntimeError(r.message)
        if r.fun > -1e-9:
            return "in the cone", r, forms, r.x
        new = sep.violated(r.x)
        if not new:
            return "separated", r, forms, r.x
        for kind, f in new:
            g = np.zeros(n)
            for a, k in f.items():
                g[a] += k
            rows.append(g)
            forms.append((kind, f))
    return "undecided", r, forms, r.x


def representative(way, combination):
    """A non-negative representative c = combination + Bᵀμ ≥ 0 of least total,
    exact (checked with the rows of the balances), or None."""
    n = way.n
    g = np.zeros(n)
    for a, w in combination.items():
        g[a] = float(w)
    Bt = way.B.T.tocsr()
    r = tmesh.run_linprog(np.asarray(Bt.sum(axis=0)).ravel(), A_ub=-Bt, b_ub=g, bounds=(None, None))
    if r.status != 0:
        return None
    c = g + Bt @ r.x
    for q in (12, 60, 1000, 100000):
        rep = {a: Fraction(float(c[a])).limit_denominator(q) for a in range(n) if c[a] > 1e-7}
        d = [Fraction(0)] * n
        for a, x in rep.items():
            d[a] += x
        for a, x in combination.items():
            d[a] -= x
        if way.in_rowspace(d) is not None:
            return rep
    return None


def free_arcs(way):
    """The arcs of a spanning tree of the bidirected graph of the balances
    plus one closing an unbalanced cycle (Proposition "Free arcs"), and the
    two (row, sign) ends of every arc."""
    B = way.B.tocsc()
    ends = []
    for a in range(way.n):
        column = B.getcol(a)
        ends.append(((column.indices[0], int(column.data[0])), (column.indices[1], int(column.data[1]))))
    m = way.B.shape[0]
    adjacent = defaultdict(list)
    for a, ((r0, s0), (r1, s1)) in enumerate(ends):
        adjacent[r0].append((r1, a))
        adjacent[r1].append((r0, a))
    parent, order, tree, sign = {0: None}, [0], set(), {0: 1}
    for u in order:
        for v, a in adjacent[u]:
            if v not in parent:
                parent[v] = (u, a)
                (r0, s0), (r1, s1) = ends[a]
                sign[v] = sign[u] * (-s0 * s1)
                tree.add(a)
                order.append(v)
    extra = next(a for a, ((r0, s0), (r1, s1)) in enumerate(ends)
                 if a not in tree and sign[r0] * sign[r1] * (-s0 * s1) != 1)
    return ends, tree | {extra}, m


def solve_exact(way, ends, T, m, free):
    """The lengths of the arcs of T from those of the free arcs, Bℓ = 0, exact."""
    x = dict(free)
    rhs = defaultdict(Fraction)
    unknown = defaultdict(list)
    for a, ((r0, s0), (r1, s1)) in enumerate(ends):
        if a in T:
            unknown[r0].append((a, s0))
            unknown[r1].append((a, s1))
        else:
            rhs[r0] -= s0 * x[a]
            rhs[r1] -= s1 * x[a]
    stack = [r for r in range(m) if len(unknown.get(r, [])) == 1]
    while stack:
        r = stack.pop()
        if len(unknown.get(r, [])) != 1:
            continue
        a, s = unknown[r][0]
        x[a] = rhs[r] / s
        unknown[r] = []
        (r0, s0), (r1, s1) = ends[a]
        other, so = (r1, s1) if r0 == r else (r0, s0)
        rhs[other] -= so * x[a]
        unknown[other] = [(b, t) for b, t in unknown[other] if b != a]
        if len(unknown[other]) == 1:
            stack.append(other)
    cycle = [r for r in unknown if len(unknown[r]) == 2]
    if cycle:
        r0 = cycle[0]
        a0, _ = unknown[r0][0]
        value = {a0: (Fraction(1), Fraction(0))}
        r, a = r0, a0
        while True:
            (q0, t0), (q1, t1) = ends[a]
            following, sn = (q1, t1) if q0 == r else (q0, t0)
            if following == r0:
                break
            b, sb = [(b, t) for b, t in unknown[following] if b != a][0]
            p, q = value[a]
            value[b] = (-sn * p / sb, (rhs[following] - sn * q) / sb)
            r, a = following, b
        (b1, t1), (b2, t2) = unknown[r0]
        p = t1 * value[b1][0] + t2 * value[b2][0]
        q = t1 * value[b1][1] + t2 * value[b2][1]
        t = (rhs[r0] - q) / p
        for b, (pp, qq) in value.items():
            x[b] = pp * t + qq
    return x


def exact_witness(M, rep, x, loose):
    """A separating x made exact: (value of the certificate there, whole
    lengths' denominator, forms still negative in integer arithmetic)."""
    way = M.way
    x = x + 1e-3 * M.L.exact
    ends, T, m = free_arcs(way)
    free = {a: Fraction(round(x[a] * 1e8), 10 ** 8) for a in range(way.n) if a not in T}
    xe = solve_exact(way, ends, T, m, free)
    D = math.lcm(*[v.denominator for v in xe.values()])
    whole = np.array([int(xe[a] * D) for a in range(way.n)], dtype=np.int64)
    balanced = all(sum(c * int(whole[a]) for a, c in row.items()) == 0 for row in way.rows)
    value = sum(Fraction(v) * int(whole[a]) for a, v in rep.items()) / D
    sep = Separator(M, loose)
    for G in (sep.D, sep.A):
        G.exact = whole
    negative = sep.violated(whole.astype(float), limit=5)
    return {"balanced": balanced, "certificate_value": float(value), "certificate_value_exact": str(value),
            "negative": value < 0, "forms_negative_at_it": len(negative),
            "largest_length": float(np.abs(whole).max() / D)}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "strict")
    parser.add_argument("--table", type=Path, default=tmesh.HERE / "results" / "conflicts.json",
                        help="the summary of conflicts.py")
    parser.add_argument("--rows", type=lambda text: [int(k) for k in text.split(",")],
                        help="rows of the table, as 1,2,7 (default all)")
    parser.add_argument("--loose", action="store_true", help="also axial walks between two odd vertices")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, paths = tmesh.ways(arguments.export, arguments.call)
    table = json.loads(arguments.table.read_text())
    meshes, out = {}, []
    for i, row in enumerate(table["table"]):
        if arguments.rows and i + 1 not in arguments.rows:
            continue
        number, turns = next(iter(row["turns_in_each_way"].items()))
        number = int(number)
        if number not in meshes:
            meshes[number] = Mesh(tmesh.Way(paths[number]))
        M = meshes[number]
        way = M.way
        ids = [way.held_index[h] for h in turns]
        u = [Fraction(x) for x in row["primitive_multipliers"]]
        combination = defaultdict(Fraction)
        for t, w in zip(ids, u):
            for a, k in way.shift[t].items():
                combination[a] += w * k
        sign = 1
        rep = representative(way, combination)
        if rep is None:
            sign = -1
            rep = representative(way, {a: -x for a, x in combination.items()})
        tick = time.monotonic()
        c = np.zeros(way.n)
        for a, w in rep.items():
            c[a] = float(w)
        verdict, result, forms, x = separate(M, c, arguments.loose)
        entry = {"row": i + 1, "way": number, "turns": turns, "multipliers": row["primitive_multipliers"],
                 "sign": sign, "verdict": verdict, "optimum": float(result.fun), "forms_generated": len(forms),
                 "seconds": round(time.monotonic() - tick, 1)}
        if verdict == "in the cone" and forms:
            duals = -result.ineqlin.marginals
            entry["decomposition"] = [{"weight": round(float(duals[k]), 4), "kind": forms[k][0],
                                       "form": {str(a): v for a, v in sorted(forms[k][1].items())}}
                                      for k in range(len(forms)) if duals[k] > 1e-9]
        if verdict == "separated":
            entry["exact_witness"] = exact_witness(M, rep, x, arguments.loose)
        out.append(entry)
        print(f"row {i + 1}, way {number}, turns {turns}: {verdict}, optimum {result.fun:+.4f}, "
              f"{len(forms)} forms" + (f"; exact witness {entry['exact_witness']}" if "exact_witness" in entry
                                       else "") + f" ({entry['seconds']} s)", flush=True)
    summary = {
        "experiment": "the question of local dislocations read strictly",
        **tmesh.record(arguments, folder, [m.way for m in meshes.values()], started, clock, loose=arguments.loose,
                       table=arguments.table.name),
        "rows": out,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
