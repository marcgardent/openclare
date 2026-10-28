#!/usr/bin/env python3
"""Any combination of turns, on an export (Section "Any combination of turns",
paragraph "On the male body"):

- the Fox–Burgers formula on every turn: the surface cut along a spanning
  tree of the nodes (the arcs a breadth-first tree of the patches does not
  cross) is one chart; each arc of the tree is a jump, written by the
  recursion of Proposition "Burgers sums" (a) as a word in the rotations
  about the singular vertices (the conventions are tried, one kept only if it
  holds at every node); every turn's holonomy is then a word, and its
  translation the sum over the vertices of Fox derivatives times their
  Burgers vectors, checked against the walk modulo the balances;
- Corollary "Checkerboard rule for any combination" on every turn and on
  every set of the table of `conflicts.py` (primitive whole multipliers): at
  the whole lengths, Σ u s has the parity of Σ w(c) colour(c) over the
  singular vertices c of odd valence, w the winding number of the cycle
  Σ u γ round c and colour(c) the parity of the developed place of c;
- the twisted windings of every set of at most --largest turns: the vertices
  where they are not zero, and the formula Σ_v ⟨W_v, (I - R_v) p_v⟩ = Σ u s
  modulo the balances;
- the twisted homology of the surface without its singular vertices and the
  rank of the Burgers pairing on it (Proposition "The Burgers pairing");
- with --relations K, K relations among the winding numbers of single turns
  that their translations break in every choice of quarter-turn frames (no
  formula linear in the windings gives the holonomy).

    python3 docs/research/turns-of-a-t-mesh/experiments/fox.py [<export>] [--relations 3]

Genus zero only (the arcs off the tree of patches must be a tree). Minutes.
Writes results/fox.json.
"""

import argparse
import itertools
import json
import time
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path

import tmesh  # first: it sets the threads of the linear algebra

import numpy as np
from geometry import DIRECTION, Dense, Layout, Motion, dense, form_value, rotate_dense, shift_form


def patch_charts(L, base=0):
    """The motion from the frame of each patch to the frame of patch `base`,
    along a breadth-first tree of the patches (symbolic)."""
    charts = {base: Motion()}
    queue = [base]
    while queue:
        p = queue.pop(0)
        for half, side in L.rims[p]:
            q = L.place[half ^ 1][0]
            if q not in charts:
                charts[q] = charts[p].after(L.crossing(half ^ 1))
                queue.append(q)
    return charts


def same(L, A, B):
    """Two motions (Dense) equal modulo the balances."""
    return A.k == B.k and L.same_mod_balances(A.t0, B.t0) and L.same_mod_balances(A.t1, B.t1)


class Fox:
    """The one chart of the surface cut along a tree of the nodes, and the
    words of its jumps (genus zero)."""

    def __init__(self, L, base=0):
        self.L, self.base, self.n = L, base, len(L.arcs)
        self.charts = patch_charts(L, base)
        crossed, seen, queue = set(), {base}, [base]
        while queue:
            p = queue.pop(0)
            for half, side in L.rims[p]:
                q = L.place[half ^ 1][0]
                if q not in seen:
                    seen.add(q)
                    crossed.add(half // 2)
                    queue.append(q)
        self.tree = [a for a in range(self.n) if a not in crossed]
        self.in_tree = set(self.tree)
        if len(self.tree) != len(L.nodes) - 1:
            raise SystemExit("the arcs off the tree of patches are not a tree of the nodes: genus zero only")
        self.D = {p: Dense.of(m, self.n) for p, m in self.charts.items()}
        self.J = {}
        for a in self.tree:
            for h in (2 * a, 2 * a + 1):
                P, Q = L.place[h][0], L.place[h ^ 1][0]
                self.J[h] = self.D[Q].after(Dense.of(L.crossing(h), self.n)).after(self.D[P].inverse())
        self.corners = defaultdict(list)
        for p, rim in enumerate(L.rims):
            for k, (half, side) in enumerate(rim):
                place = self.charts[p].apply(L.start[p][k])
                self.corners[L.tail(half)].append((dense(place[0], self.n), dense(place[1], self.n)))
        self.root_tree()

    def root_tree(self):
        L = self.L
        adjacent = defaultdict(list)
        for a in self.tree:
            n0, n1 = L.arcs[a]["nodes"]
            adjacent[n0].append(a)
            adjacent[n1].append(a)
        start = L.tail(L.rims[self.base][0][0])
        seen, queue = {start}, [start]
        v = start
        while queue:
            v = queue.pop(0)
            if L.singular(v):
                break
            for a in adjacent[v]:
                n0, n1 = L.arcs[a]["nodes"]
                w = n1 if n0 == v else n0
                if w not in seen:
                    seen.add(w)
                    queue.append(w)
        root = v
        parent = {root: None}
        order = [root]
        for v in order:
            for a in adjacent[v]:
                n0, n1 = L.arcs[a]["nodes"]
                w = n1 if n0 == v else n0
                if w not in parent:
                    parent[w] = 2 * a if n0 == v else 2 * a + 1
                    order.append(w)
        self.parent, self.order = parent, order
        self.children = {}
        for v in order:
            halves = L.nodes[v]["halves"]
            back = None if parent[v] is None else parent[v] ^ 1
            tree_halves = [h for h in halves if h // 2 in self.in_tree and h != back]
            if back is not None:
                i = halves.index(back)
                turned = halves[i + 1:] + halves[:i]
                tree_halves = [h for h in turned if h in tree_halves]
            self.children[v] = tree_halves

    def conventions(self):
        for use_twin in (False, True):
            for reverse in (False, True):
                for after in (False, True):
                    ok, words, places = self.recurse(use_twin, reverse, after)
                    if ok:
                        self.convention = {"twin": use_twin, "reverse": reverse, "after": after}
                        self.use_twin, self.words, self.places = use_twin, words, places
                        return True
        return False

    def generator(self, h, use_twin):
        return self.J[h ^ 1] if use_twin else self.J[h]

    def recurse(self, use_twin, reverse, after):
        L = self.L
        words, places = {}, {}
        identity = Dense.identity(self.n)
        for v in reversed(self.order):
            h = self.parent[v]
            if h is None:
                continue
            kids = list(reversed(self.children[v])) if reverse else self.children[v]
            composed, word = identity, []
            for c in kids:
                composed = self.generator(c, use_twin).after(composed)
                word = words[c] + word
            J = self.generator(h, use_twin)
            X = composed.inverse().after(J) if not after else J.after(composed.inverse())
            if not L.singular(v):
                if not same(L, X, identity):
                    return False, None, None
                words[h] = word
                continue
            j = 4 - L.nodes[v]["angle"]
            found = None
            for p in self.corners[v]:
                for jj in (j, -j):
                    if same(L, X, Dense.rotation_about(jj, *p)):
                        found = (p, jj)
                        break
                if found:
                    break
            if found is None:
                return False, None, None
            places[v] = found
            words[h] = (word + [(v, 1)]) if not after else ([(v, 1)] + word)
        return True, words, places

    def away(self, h):
        return self.parent.get(self.L.head(h)) == h

    def turn_word(self, turn):
        word = []
        for (p, half, q, into) in turn["crossings"]:
            if half // 2 not in self.in_tree:
                continue
            if self.use_twin:
                w, sign = (self.words[half ^ 1], 1) if self.away(half ^ 1) else (self.words[half], -1)
            else:
                w, sign = (self.words[half], 1) if self.away(half) else (self.words[half ^ 1], -1)
            letters = w if sign == 1 else [(v, -e) for v, e in reversed(w)]
            word = letters + word
        return word

    def motion_of_word(self, word):
        M = Dense.identity(self.n)
        for v, e in reversed(word):
            p, j = self.places[v]
            M = Dense.rotation_about(e * j, *p).after(M)
        return M

    def fox(self, word):
        """For each vertex, its Fox derivative as a0 + a1 R + a2 R² + a3 R³."""
        D = defaultdict(lambda: [0, 0, 0, 0])
        k = 0
        for v, e in word:
            p, j = self.places[v]
            if e == 1:
                D[v][k % 4] += 1
                k += j
            else:
                k -= j
                D[v][k % 4] -= 1
        return dict(D)

    def burgers(self, v):
        """(I - R^j) p_v."""
        (p0, p1), j = self.places[v]
        r0, r1 = rotate_dense(j, p0, p1)
        return p0 - r0, p1 - r1

    def translation(self, D):
        t0, t1 = np.zeros(self.n, np.int64), np.zeros(self.n, np.int64)
        for v, coefficients in D.items():
            b0, b1 = self.burgers(v)
            for k, a in enumerate(coefficients):
                if a:
                    r0, r1 = rotate_dense(k, b0, b1)
                    t0, t1 = t0 + a * r0, t1 + a * r1
        return t0, t1

    def across(self, turn):
        """The across direction of the exported shift in the one chart."""
        side = turn["side"]
        e2 = DIRECTION[(side + 1 + (-1 if side in (0, 1) else 1)) % 4]
        k = self.charts[turn["patch"]].k
        a, b = e2
        for _ in range(k):
            a, b = -b, a
        return (a, b)


def colours(L, fox, x):
    """The parity of the developed place of every node at whole lengths x."""
    colour, disagree = {}, 0
    for p, rim in enumerate(L.rims):
        M = fox.charts[p]
        for k, (half, side) in enumerate(rim):
            v = L.tail(half)
            place = M.apply(L.start[p][k])
            c = int(form_value(place[0], x) + form_value(place[1], x)) % 2
            if v in colour and colour[v] != c:
                disagree += 1
            colour.setdefault(v, c)
    return colour, disagree


def windings(L, turn_ids, weights):
    """The winding numbers of Σ u γ round every node (the turns as walked,
    no handedness), zero on the region holding the most nodes."""
    regions, sides, find, node_cell = L.split(turn_ids)
    owner = defaultdict(list)
    for r, tid in enumerate(turn_ids):
        for patch, a, b in L.chords(L.turns[tid])[0]:
            owner[patch].append(r)

    def offset(cell):
        patch, signs = cell
        return sum(weights[owner[patch][i]] for i, inside in enumerate(signs) if not inside)

    graph = defaultdict(list)
    for a, b, half, lo, hi in L.last_edges:
        graph[a[0]].append((b[0], offset(b) - offset(a)))
        graph[b[0]].append((a[0], offset(a) - offset(b)))
    big = max((k for k in regions if k is not None), key=lambda k: len(regions[k]))
    start = next(cell for n, cell, p, k in L.last_corners if find(cell) == big)
    base = {start[0]: -offset(start)}
    queue, bad = [start[0]], 0
    while queue:
        p = queue.pop()
        for q, delta in graph[p]:
            value = base[p] - delta
            if q not in base:
                base[q] = value
                queue.append(q)
            elif base[q] != value:
                bad += 1
    winding = {}
    for v, cell, p, k in L.last_corners:
        w = base[p] + offset(cell)
        if v in winding and winding[v] != w:
            bad += 1
        winding.setdefault(v, w)
    return winding, bad


def checkerboard(L, ids, weights, colour):
    """Σ u s at whole lengths, and the parity the corollary predicts."""
    x = L.whole
    total = sum(int(u) * int(form_value(shift_form(L.turns[t]), x)) for t, u in zip(ids, weights))
    winding, bad = windings(L, ids, weights)
    predicted = sum(int(winding[v]) * colour[v] for v in winding
                    if L.singular(v) and L.nodes[v]["angle"] % 2 == 1) % 2
    return (total - predicted) % 2 == 0, bad


def twisted(L, fox, ids, weights):
    """The non-zero twisted windings W_v of Σ u γ, and whether
    Σ_v ⟨W_v, (I - R_v) p_v⟩ = Σ u s modulo the balances."""
    n = len(L.arcs)
    W = defaultdict(lambda: np.zeros(2, dtype=np.int64))
    combination = np.zeros(n, np.int64)
    for t, u in zip(ids, weights):
        turn = L.turns[t]
        f = fox.across(turn)
        for v, coefficients in fox.fox(fox.turn_word(turn)).items():
            vector = np.zeros(2, dtype=np.int64)
            for k, a in enumerate(coefficients):
                if a:
                    r = f
                    for _ in range((-k) % 4):
                        r = (-r[1], r[0])
                    vector += a * np.array(r, dtype=np.int64)
            W[v] += int(u) * vector
        combination += int(u) * dense(shift_form(turn), n)
    total = np.zeros(n, np.int64)
    for v, vector in W.items():
        b0, b1 = fox.burgers(v)
        total += vector[0] * b0 + vector[1] * b1
    nonzero = {v: [int(a) for a in vector] for v, vector in W.items() if vector.any()}
    # Pairs of a vertex of valence 3 and one of valence 5 whose contributions
    # add up, modulo the balances, to a multiple of a single arc length (a
    # dislocation, Proposition "Dislocations" (ii)).
    N = L.nullspace()
    single = []
    for v, w in itertools.combinations(sorted(nonzero), 2):
        if sorted((L.nodes[v]["angle"], L.nodes[w]["angle"])) != [3, 5]:
            continue
        part = np.zeros(n)
        for u in (v, w):
            b0, b1 = fox.burgers(u)
            part += W[u][0] * b0 + W[u][1] * b1
        p = part @ N
        for a in range(n):
            q = N[a]
            size = q @ q
            if size < 1e-12:
                continue
            c = (p @ q) / size
            if abs(c - round(c)) < 1e-6 and round(c) != 0 and np.abs(p - round(c) * q).max() < 1e-8:
                single.append({"vertices": [v, w], "arc": a, "times": int(round(c))})
                break
    return nonzero, L.same_mod_balances(total, combination), single


def pairing(L, fox):
    """Dimension of the twisted cycles modulo boundaries, and the rank of the
    Burgers pairing on them (Proposition "The Burgers pairing")."""
    from scipy.linalg import null_space

    N = L.nullspace()
    vertices = sorted(fox.places)
    columns, cycles = [], []
    R = np.array([[0, -1], [1, 0]])
    for v in vertices:
        b0, b1 = fox.burgers(v)
        columns += [b0.astype(float) @ N, b1.astype(float) @ N]
        j = fox.places[v][1]
        cycles.append((np.eye(2) - np.linalg.matrix_power(R, j % 4)).T)
    S = np.array(columns).T
    Z = null_space(np.hstack(cycles))
    values = np.linalg.svd(S @ Z, compute_uv=False)
    rank = int((values > 1e-8 * values[0]).sum())
    # The surface without its n singular vertices is a wedge of the n - 1
    # loops of the generators (the root's loop is their product): its twisted
    # homology is the twisted cycles, of dimension 2(n - 1) - 2.
    return {"singular_vertices": len(vertices) + 1, "generators": len(vertices),
            "twisted_homology_dimension": Z.shape[1], "pairing_rank": rank, "kernel": Z.shape[1] - rank}


def relations(layouts, wanted, most_terms=6):
    """Relations among the winding numbers of single turns that their
    translations break in every choice of quarter-turn frames: for each turn,
    the sparsest combination of the others (distinct windings and shifts)
    with the same winding numbers round every singular vertex (least L1, a
    linear program); kept if its coefficients are whole, it has at most
    `most_terms` terms, and Σ c_r R^{j_r} T_r is not zero modulo the balances
    for any frames j_r. Stops after `wanted` such relations."""
    vectors, translations, info, seen = [], [], [], set()
    for number, (L, fox, colour) in layouts.items():
        singular = [v for v in range(len(L.nodes)) if L.singular(v)]
        N = L.nullspace()
        for tid, turn in enumerate(L.turns):
            winding, bad = windings(L, [tid], [1])
            if bad:
                continue
            v = np.array([winding[s] for s in singular], dtype=float)
            v -= v[0]
            M = L.walk(turn)
            key = (tuple(v.astype(int)), tuple(np.round(dense(shift_form(turn), len(L.arcs)) @ N, 6)))
            if key in seen:
                continue
            seen.add(key)
            vectors.append(v)
            translations.append((M.t0, M.t1, N))
            info.append((number, tid))
    W = np.array(vectors)
    found = []
    for target in range(len(W)):
        others = [i for i in range(len(W)) if i != target and not np.array_equal(W[i], W[target])]
        A = W[others].T
        m = len(others)
        r = tmesh.run_linprog(np.ones(2 * m), A_eq=np.hstack([A, -A]), b_eq=W[target], bounds=(0, None))
        if r.status != 0:
            continue
        c = r.x[:m] - r.x[m:]
        support = [(others[j], c[j]) for j in range(m) if abs(c[j]) > 1e-9]
        if len(support) + 1 > most_terms or any(abs(x - round(x)) > 1e-9 for _, x in support):
            continue
        terms = [(target, 1)] + [(j, -int(round(x))) for j, x in support]
        best = None
        N = translations[terms[0][0]][2]
        for frames in itertools.product(range(4), repeat=len(terms) - 1):
            turned = [rotate_dense(j, *translations[i][:2]) for (i, _), j in zip(terms, (0,) + frames)]
            t0 = sum(cf * r[0] for (_, cf), r in zip(terms, turned))
            t1 = sum(cf * r[1] for (_, cf), r in zip(terms, turned))
            residual = max(np.abs(t0.astype(float) @ N).max(), np.abs(t1.astype(float) @ N).max())
            best = residual if best is None else min(best, residual)
        if best is not None and best > 1e-6:
            found.append({"turns": [{"way": info[i][0], "turn": info[i][1], "coefficient": cf} for i, cf in terms],
                          "smallest_residual_over_frames": float(best)})
            if len(found) >= wanted:
                break
    return found


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "fox")
    parser.add_argument("--table", type=Path, default=tmesh.HERE / "results" / "conflicts.json",
                        help="the summary of conflicts.py")
    parser.add_argument("--largest", type=int, default=4, help="twisted windings of sets of at most this size")
    parser.add_argument("--pairing-way", type=int, default=1, help="the way whose T-mesh the pairing is read on")
    parser.add_argument("--relations", type=int, default=0,
                        help="look for this many relations of windings that the translations break")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    rows, total, layouts = [], Counter(), {}
    for way in ways:
        tick = time.monotonic()
        L = Layout(way)
        fox = Fox(L)
        if not fox.conventions():
            raise SystemExit(f"way {way.number}: no convention of the recursion holds")
        colour, disagree = colours(L, fox, L.whole)
        tally = Counter()
        for tid, turn in enumerate(L.turns):
            word = fox.turn_word(turn)
            M = fox.motion_of_word(word)
            P0 = fox.D[turn["patch"]]
            expected = P0.after(L.walk(turn)).after(P0.inverse())
            t0, t1 = fox.translation(fox.fox(word))
            tally["turns"] += 1
            tally["Fox–Burgers formula"] += same(L, M, expected) and L.same_mod_balances(t0, expected.t0) \
                and L.same_mod_balances(t1, expected.t1)
            ok, bad = checkerboard(L, [tid], [1], colour)
            tally["checkerboard rule for the turn"] += ok
            tally["winding inconsistencies"] += bad
        layouts[way.number] = (L, fox, colour)
        total.update(tally)
        rows.append({"way": way.number, "convention": fox.convention, "tree_arcs": len(fox.tree),
                     "colour_disagreements": disagree, **{k: int(v) for k, v in tally.items()},
                     "seconds": round(time.monotonic() - tick, 1)})
        print(f"way {way.number}: {rows[-1]}", flush=True)
    sets = []
    if arguments.table.exists():
        table = json.loads(arguments.table.read_text())
        for i, row in enumerate(table["table"]):
            number, turns = next(iter(row["turns_in_each_way"].items()))
            number = int(number)
            if number not in layouts:
                continue
            L, fox, colour = layouts[number]
            ids = [L.held[h] for h in turns]
            weights = [Fraction(u) for u in row["primitive_multipliers"]]
            ok, bad = checkerboard(L, ids, weights, colour)
            entry = {"row": i + 1, "way": number, "turns": turns, "multipliers": row["primitive_multipliers"],
                     "checkerboard_rule": ok, "winding_inconsistencies": bad}
            if len(ids) <= arguments.largest:
                nonzero, formula, single = twisted(L, fox, ids, weights)
                entry.update(twisted_windings=nonzero, twisted_formula=formula, dislocations=single)
            sets.append(entry)
        print(f"{len(sets)} sets: checkerboard rule on {sum(s['checkerboard_rule'] for s in sets)}", flush=True)
    broken = None
    if arguments.relations:
        broken = relations(layouts, arguments.relations)
        print(f"relations of windings that the translations break: {broken}", flush=True)
    pairing_out = None
    if arguments.pairing_way in layouts:
        L, fox, _ = layouts[arguments.pairing_way]
        pairing_out = {"way": arguments.pairing_way, **pairing(L, fox)}
        print(f"pairing: {pairing_out}", flush=True)
    summary = {
        "experiment": "any combination of turns: the Fox–Burgers formula, the checkerboard rule, the pairing",
        **tmesh.record(arguments, folder, ways, started, clock, table=arguments.table.name),
        "total": {k: int(v) for k, v in total.items()},
        "per_way": rows,
        "sets": sets,
        "pairing": pairing_out,
        "relations_broken_by_the_translations": broken,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
