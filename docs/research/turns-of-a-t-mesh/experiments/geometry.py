"""The geometry of turns on an exported T-mesh, for `holonomy.py` and
`pairs.py`: patch frames, crossing maps, turns as curves, the regions their
curves cut the surface into, development along a spanning tree of cells,
the holonomy of a walk and of a contour, monotone paths of arcs, and the
winding numbers of a weighted cycle of turns.

It reads the T-mesh the export carries (`nodes`, `arcs`, `patches`; ALGORITHM.md,
M6, "Experiment options") and each turn's `crossings`.

- Patch frames: each patch's rim from its first corner (as the turns read
  it); the place of each rim node as numbers (for given lengths) or as
  sparse integer forms in the arc lengths.
- Crossing maps between the frames of two patches across an arc:
  x -> R^j x + t, R the quarter turn, t a pair of forms.
- Turns as curves: each patch mapped to the unit square, each side by the
  share of its length (the lengths the turns were walked with); a passage of
  a turn is the straight chord between where it enters and where it leaves
  (in the first patch, from where the walk comes back to where it first
  leaves: the closing segment, made straight). Chords cut a square into
  convex cells, each named by its side of every chord; cells joined across
  the pieces of arcs between crossing points make the regions.
"""

import itertools
import sys
from collections import defaultdict
from fractions import Fraction

import numpy as np

DIRECTION = [(1, 0), (0, 1), (-1, 0), (0, -1)]


def rotate(k, v):
    """R^k v, R the quarter turn (a, b) -> (-b, a)."""
    a, b = v
    for _ in range(k % 4):
        a, b = -b, a
    return (a, b)


# Sparse integer forms in the arc lengths: {arc: coefficient}.
def form_add(*forms):
    out = defaultdict(int)
    for f in forms:
        for a, c in f.items():
            out[a] += c
    return {a: c for a, c in out.items() if c != 0}


def form_scale(f, s):
    return {a: c * s for a, c in f.items() if c * s != 0}


def form_value(f, x):
    return sum(c * x[a] for a, c in f.items())


def vector_add(u, v):
    return (form_add(u[0], v[0]), form_add(u[1], v[1]))


def vector_sub(u, v):
    return (form_add(u[0], form_scale(v[0], -1)), form_add(u[1], form_scale(v[1], -1)))


def vector_rotate(k, v):
    a, b = v
    for _ in range(k % 4):
        a, b = form_scale(b, -1), a
    return (a, b)


class Motion:
    """x -> R^k x + t, t a pair of forms."""

    def __init__(self, k=0, t=({}, {})):
        self.k = k % 4
        self.t = t

    def apply(self, p):
        return vector_add(vector_rotate(self.k, p), self.t)

    def after(self, other):
        """self o other."""
        return Motion(self.k + other.k, vector_add(vector_rotate(self.k, other.t), self.t))

    def inverse(self):
        k = (-self.k) % 4
        return Motion(k, vector_rotate(k, (form_scale(self.t[0], -1), form_scale(self.t[1], -1))))


def rotation_about(j, p):
    """The rotation R^j about the point p."""
    return Motion(j, vector_sub(p, vector_rotate(j, p)))


def dense(form, n):
    """A form as a vector of its n coefficients."""
    v = np.zeros(n, dtype=np.int64)
    for a, c in form.items():
        v[a] += c
    return v


def rotate_dense(k, a, b):
    for _ in range(k % 4):
        a, b = -b, a
    return a, b


class Dense:
    """A motion x -> R^k x + t with t a pair of coefficient vectors: the same
    algebra as Motion, fast on long compositions."""

    def __init__(self, k, t0, t1):
        self.k, self.t0, self.t1 = k % 4, t0, t1

    @staticmethod
    def of(motion, n):
        return Dense(motion.k, dense(motion.t[0], n), dense(motion.t[1], n))

    @staticmethod
    def identity(n):
        return Dense(0, np.zeros(n, dtype=np.int64), np.zeros(n, dtype=np.int64))

    @staticmethod
    def rotation_about(j, p0, p1):
        r0, r1 = rotate_dense(j, p0, p1)
        return Dense(j, p0 - r0, p1 - r1)

    def after(self, other):
        r0, r1 = rotate_dense(self.k, other.t0, other.t1)
        return Dense(self.k + other.k, r0 + self.t0, r1 + self.t1)

    def inverse(self):
        k = (-self.k) % 4
        a, b = rotate_dense(k, -self.t0, -self.t1)
        return Dense(k, a, b)


def shift_form(turn):
    return {int(a): int(round(c)) for a, c in turn["shift"] if round(c) != 0}


class Layout:
    """One way file of an export, as a T-mesh with turns."""

    def __init__(self, way):
        d = way.data
        if "nodes" not in d or (d["turns"] and "crossings" not in d["turns"][0]):
            raise SystemExit(f"{way.path}: the export has no T-mesh (an engine before the hook carried it)")
        self.way = way
        self.data = d
        self.nodes = d["nodes"]
        self.arcs = d["arcs"]
        self.rims = [[tuple(h) for h in rim] for rim in d["patches"]]
        self.turns = d["turns"]
        self.exact = np.array([a["length"] for a in self.arcs])
        self.whole = np.array(d["lengths"], dtype=np.int64)
        self.place = {}
        for p, rim in enumerate(self.rims):
            for k, (half, side) in enumerate(rim):
                self.place[half] = (p, k)
        # The place of the start of each rim half, as a pair of forms.
        self.start = []
        for rim in self.rims:
            at = ({}, {})
            starts = []
            for half, side in rim:
                starts.append(at)
                d0, d1 = DIRECTION[side]
                arc = half // 2
                at = (form_add(at[0], {arc: d0} if d0 else {}), form_add(at[1], {arc: d1} if d1 else {}))
            starts.append(at)
            self.start.append(starts)
        self.held = {t["held_index"]: i for i, t in enumerate(self.turns) if t["held"]}
        self._null = None

    # --- nodes and halves ---------------------------------------------------------------

    def tail(self, half):
        """The node a half-arc starts from."""
        a = self.arcs[half // 2]["nodes"]
        return a[0] if half % 2 == 0 else a[1]

    def head(self, half):
        return self.tail(half ^ 1)

    def index(self, node):
        return 4 - self.nodes[node]["angle"]

    def singular(self, node):
        return self.nodes[node]["kind"] == "singularity"

    def side_range(self, patch, side):
        return [k for k, (h, s) in enumerate(self.rims[patch]) if s == side]

    # --- crossing maps ----------------------------------------------------------------------

    def crossing(self, half):
        """The motion from the frame of the patch holding `half` to the frame
        of the patch holding its twin."""
        p, k = self.place[half]
        q, k2 = self.place[half ^ 1]
        s, s2 = self.rims[p][k][1], self.rims[q][k2][1]
        j = (s2 + 2 - s) % 4
        a = self.start[p][k]
        # The head of the twin in q is the tail of `half`: the start of the
        # next rim half in q.
        b = self.start[q][(k2 + 1) % len(self.rims[q])]
        return Motion(j, vector_sub(b, vector_rotate(j, a)))

    def walk(self, turn):
        """The composed crossing maps along a turn's walk (a Dense motion)."""
        n = len(self.arcs)
        M = Dense.identity(n)
        for (p, half, q, into) in turn["crossings"]:
            M = Dense.of(self.crossing(half), n).after(M)
        return M

    # --- turns as curves ------------------------------------------------------------------

    def tau(self, patch, k, along, x):
        """The place on the boundary of the unit square, in [0, 4), of the
        point `along` from the tail of rim half k of the patch."""
        rim = self.rims[patch]
        side = rim[k][1]
        ks = self.side_range(patch, side)
        before = sum(x[rim[j][0] // 2] for j in ks if j < k)
        total = sum(x[rim[j][0] // 2] for j in ks)
        return side + (before + along) / total

    def chords(self, turn, x=None):
        """The chords of a turn, (patch, tau in, tau out), and where it
        crosses arcs, (arc, distance from node 0)."""
        x = self.exact if x is None else x
        out, points = [], []
        patch = turn["patch"]
        entry = None
        for (p, half, q, into) in turn["crossings"]:
            k = self.place[half][1]
            out.append([p, entry, self.tau(p, k, into, x)])
            arc = half // 2
            length = x[arc]
            points.append((arc, into if half % 2 == 0 else length - into))
            q2, k2 = self.place[half ^ 1]
            entry = self.tau(q, k2, length - into, x)
            patch = q
        out[0][1] = entry
        return [tuple(c) for c in out], points

    @staticmethod
    def inside(t, a, b):
        """t strictly within the counter-clockwise boundary interval from a to b."""
        if a < b:
            return a < t < b
        return t > a or t < b

    def regions(self, turn_ids, x=None):
        """Union-find over the cells cut by the curves of the turns: (find,
        the cell of each node, None for a node on a curve)."""
        x = self.exact if x is None else x
        chords = defaultdict(list)
        points = defaultdict(list)
        for tid in turn_ids:
            cs, pts = self.chords(self.turns[tid], x)
            for c in cs:
                chords[c[0]].append((c[1], c[2]))
            for arc, t in pts:
                points[arc].append(t)
        self.last_chords = chords
        parent = {}
        self.last_edges = []
        self.last_corners = []

        def find(c):
            parent.setdefault(c, c)
            while parent[c] != c:
                parent[c] = parent[parent[c]]
                c = parent[c]
            return c

        def union(a, b):
            ra, rb = find(a), find(b)
            if ra != rb:
                parent[ra] = rb

        def cell(patch, t):
            return (patch, tuple(self.inside(t, a, b) for a, b in chords.get(patch, [])))

        self.cell_at = cell
        for arc in range(len(self.arcs)):
            length = x[arc]
            ts = sorted(points.get(arc, []))
            ends = [0.0] + ts + [length]
            for a, b in zip(ends, ends[1:]):
                if b - a < 1e-12:
                    continue
                m = 0.5 * (a + b)
                p, k = self.place[2 * arc]
                q, k2 = self.place[2 * arc + 1]
                ca, cb = cell(p, self.tau(p, k, m, x)), cell(q, self.tau(q, k2, length - m, x))
                union(ca, cb)
                self.last_edges.append((ca, cb, 2 * arc, a, b))
        node_cell = {}
        for p, rim in enumerate(self.rims):
            for k, (half, side) in enumerate(rim):
                n = self.tail(half)
                c0 = cell(p, self.tau(p, k, 1e-9, x))
                self.last_corners.append((n, c0, p, k))
                c = find(c0)
                if n in node_cell and node_cell[n] != c:
                    node_cell[n] = None
                elif n not in node_cell:
                    node_cell[n] = c
        return find, node_cell

    def split(self, turn_ids, x=None):
        """The regions of the surface cut by the curves ({root: nodes}), and
        for each turn the regions along it."""
        x = self.exact if x is None else x
        find, node_cell = self.regions(turn_ids, x)
        regions = defaultdict(list)
        for n, c in node_cell.items():
            regions[c].append(n)
        sides = []
        for tid in turn_ids:
            cs, _ = self.chords(self.turns[tid], x)
            touched = set()
            for patch, a, b in cs:
                for t in (a, b):
                    for delta in (-1e-9, 1e-9):
                        touched.add(find(self.cell_at(patch, (t + delta) % 4)))
            sides.append(touched)
        return regions, sides, find, node_cell

    def crossings_between(self, t1, t2, x=None):
        """Transverse crossings of the curves of two turns (of one with itself
        if t1 == t2): chords in one patch whose ends interleave."""
        x = self.exact if x is None else x
        c1, _ = self.chords(self.turns[t1], x)
        c2, _ = self.chords(self.turns[t2], x)
        count = 0
        for i, (p, a, b) in enumerate(c1):
            for j, (q, c, d) in enumerate(c2):
                if p != q or (t1 == t2 and j <= i):
                    continue
                if self.inside(c, a, b) != self.inside(d, a, b):
                    count += 1
        return count

    # --- forms modulo the balances ------------------------------------------------------

    def nullspace(self):
        """A basis of the balanced lengths (floating point)."""
        if self._null is None:
            from scipy.linalg import null_space
            self._null = null_space(self.way.B.toarray())
        return self._null

    def same_mod_balances(self, f, g):
        """Whether the forms f and g (dictionaries or coefficient vectors) agree
        on every balanced length."""
        n = len(self.arcs)
        v = (f if isinstance(f, np.ndarray) else dense(f, n)) - (g if isinstance(g, np.ndarray) else dense(g, n))
        return float(np.abs(v.astype(float) @ self.nullspace()).max()) < 1e-8

    # --- nodes developed ------------------------------------------------------------------

    def node_text(self, n):
        node = self.nodes[n]
        tag = f"valence {node['angle']}" if node["kind"] == "singularity" else node["kind"]
        p = node["at"]
        return f"{n} ({tag}, at {p[0]:+.3f} {p[1]:+.3f} {p[2]:+.3f})"


def develop(L, root, find, base=None):
    """The motion from the frame of each cell of a region to the frame of
    its base cell, along a breadth-first spanning tree of its cells (the
    region cut into a disc)."""
    edges = defaultdict(list)
    for a, b, half, *_ in L.last_edges:
        if find(a) == root and a != b:
            edges[a].append((b, half))
            edges[b].append((a, half ^ 1))
    cells = list(edges)
    if base is None:
        base = min(cells)
    motion = {base: Motion()}
    queue = [base]
    while queue:
        a = queue.pop(0)
        for b, half in edges[a]:
            if b not in motion:
                motion[b] = motion[a].after(L.crossing(half ^ 1))
                queue.append(b)
    return motion


def developed_nodes(L, root, find, motion):
    """The developed place (a pair of forms) of each node of the region,
    through the first corner cell met in the tree."""
    out = {}
    for n, cell, p, k in L.last_corners:
        if n in out or cell not in motion:
            continue
        out[n] = motion[cell].apply(L.start[p][k])
    return out


def contour_order(L, turn, root, find, x):
    """The singular nodes of a disc bounded by a simple turn, in the order of
    their first visit and of their last visit by the contour of the disc cut
    along a breadth-first tree of its cells (from where the walk closes),
    each with its developed place at its first visit."""
    patch, t_in, t_out = L.chords(turn, x)[0][0]
    edges = defaultdict(list)
    elements = defaultdict(list)
    for e, (a, b, half, lo, hi) in enumerate(L.last_edges):
        if find(a) != root:
            continue
        arc = half // 2
        p, k = L.place[2 * arc]
        q, k2 = L.place[2 * arc + 1]
        mid = 0.5 * (lo + hi)
        elements[a].append((L.tau(p, k, mid, x), "piece", e, b))
        elements[b].append((L.tau(q, k2, x[arc] - mid, x), "piece", e, a))
        if a != b:
            edges[a].append((b, half, e))
            edges[b].append((a, half ^ 1, e))
    for n, cell, p, k in L.last_corners:
        if find(cell) == root:
            elements[cell].append((L.tau(p, k, 1e-9, x), "corner", n, (p, k)))
    for c in elements:
        elements[c].sort(key=lambda element: element[0])
    after_in = L.cell_at(patch, (t_in + 1e-9) % 4)
    if find(after_in) == root:
        base, start = after_in, t_in
    else:
        base, start = L.cell_at(patch, (t_out + 1e-9) % 4), t_out
    motion = {base: Motion()}
    parent_edge = {base: None}
    queue = [base]
    while queue:
        a = queue.pop(0)
        for b, half, e in edges[a]:
            if b not in motion:
                motion[b] = motion[a].after(L.crossing(half ^ 1))
                parent_edge[b] = e
                queue.append(b)
    first, last, first_time = {}, {}, {}
    clock = [0]
    visited = {base}

    def visit(cell, entry):
        elements_here = elements[cell]
        if entry is None:
            i0 = next((i for i, el in enumerate(elements_here) if el[0] > start), 0)
            order = elements_here[i0:] + elements_here[:i0]
        else:
            i0 = next(i for i, el in enumerate(elements_here) if el[1] == "piece" and el[2] == entry)
            order = elements_here[i0 + 1:] + elements_here[:i0]
        for el in order:
            if el[1] == "corner":
                n = el[2]
                if L.singular(n):
                    clock[0] += 1
                    if n not in first:
                        first_time[n] = clock[0]
                        p, k = el[3]
                        first[n] = motion[cell].apply(L.start[p][k])
                    last[n] = clock[0]
            else:
                e, other = el[2], el[3]
                if other != cell and parent_edge.get(other) == e and other not in visited:
                    visited.add(other)
                    visit(other, e)

    sys.setrecursionlimit(max(sys.getrecursionlimit(), 100000))
    visit(base, None)
    by_first = sorted(first_time, key=lambda n: first_time[n])
    by_last = sorted(last, key=lambda n: last[n])
    return by_first, by_last, first


# --- monotone paths of arcs ----------------------------------------------------------------


def next_ccw(L, half):
    """The half-arc leaving the same node a turn counter-clockwise after
    `half`, and the angle between them in quarter turns."""
    a = L.arcs[half // 2]
    P = a["patches"][0] if half % 2 == 0 else a["patches"][1]
    rim = L.rims[P]
    k = next(k for k, (h, s) in enumerate(rim) if h == half)
    g, side_g = rim[k - 1]
    angle = 2 if side_g == rim[k][1] else 1
    return g ^ 1, angle


def directions_at(L, node, half, direction):
    """The directions of every half-arc leaving `node`, given that of one."""
    out = {half: direction % 4}
    h, d = half, direction
    for _ in range(len(L.nodes[node]["halves"]) - 1):
        h, angle = next_ccw(L, h)
        d = d + angle
        out[h] = d % 4
    return out


def staircases(L, a, b, depth):
    """Monotone paths of arcs from node a to node b through T-junctions only,
    every arc in one of two directions a quarter turn apart: [(directions,
    arcs)]."""
    found = []
    start = L.nodes[a]["halves"][0]
    dirs_a = directions_at(L, a, start, 0)
    for low in range(4):
        pair = {low, (low + 1) % 4}

        def go(node, dirs, path, visited):
            if len(path) > depth:
                return
            for h, d in dirs.items():
                if d not in pair:
                    continue
                m = L.head(h)
                if m in visited:
                    continue
                if m == b:
                    found.append((sorted(pair), path + [h // 2]))
                    continue
                if L.singular(m):
                    continue
                go(m, directions_at(L, m, h ^ 1, d + 2), path + [h // 2], visited | {m})

        go(a, dirs_a, [], {a})
    return found


def straight(L, start, arcs):
    """Whether every arc of the path from `start` runs in one direction."""
    node, first, dirs = start, None, None
    for arc in arcs:
        half = 2 * arc if L.arcs[arc]["nodes"][0] == node else 2 * arc + 1
        if first is None:
            dirs = directions_at(L, node, half, 0)
            first = 0
        if dirs[half] != first:
            return False
        node = L.head(half)
        dirs = directions_at(L, node, half ^ 1, dirs[half] + 2)
    return True


# --- winding numbers ---------------------------------------------------------------------


def windings(L, turn_ids, weights):
    """The winding numbers round every region of the cycle Σ u_r γ_r (each
    turn weighted by its multiplier times its handedness: the exported shift
    reads across e1 turned clockwise from sides 0 and 1, counter-clockwise
    from sides 2 and 3), zero on the largest region. Each turn may be taken
    either way round: the orientations putting the fewest singular nodes
    under a non-zero winding are kept. Returns ({root: winding}, regions,
    inconsistencies)."""
    hand = [1 if L.turns[t]["side"] in (0, 1) else -1 for t in turn_ids]
    u0 = [Fraction(w) * h for w, h in zip(weights, hand)]
    regions, sides, find, node_cell = L.split(turn_ids)
    owner = defaultdict(list)
    for r, tid in enumerate(turn_ids):
        for patch, a, b in L.chords(L.turns[tid])[0]:
            owner[patch].append(r)

    def wind(u):
        def offset(cell):
            patch, signs = cell
            return sum(u[owner[patch][i]] for i, inside in enumerate(signs) if not inside)

        graph = defaultdict(list)
        for a, b, half, lo, hi in L.last_edges:
            graph[a[0]].append((b[0], offset(b) - offset(a)))
            graph[b[0]].append((a[0], offset(a) - offset(b)))
        big = max(regions, key=lambda k: len(regions[k]))
        start_cell = next(cell for n, cell, p, k in L.last_corners if find(cell) == big)
        base = {start_cell[0]: -offset(start_cell)}
        queue = [start_cell[0]]
        inconsistencies = 0
        while queue:
            p = queue.pop()
            for q, delta in graph[p]:
                value = base[p] - delta
                if q not in base:
                    base[q] = value
                    queue.append(q)
                elif base[q] != value:
                    inconsistencies += 1
        region_winding = {}
        for n, cell, p, k in L.last_corners:
            w = base[p] + offset(cell)
            root = find(cell)
            if root in region_winding and region_winding[root] != w:
                inconsistencies += 1
            region_winding[root] = w
        return region_winding, inconsistencies

    best = None
    for flips in itertools.product((1, -1), repeat=len(turn_ids) - 1):
        u = [u0[0]] + [w * f for w, f in zip(u0[1:], flips)]
        region_winding, inconsistencies = wind(u)
        weight = sum(len([v for v in regions[r] if L.singular(v)]) for r, w in region_winding.items() if w != 0)
        if best is None or weight < best[0]:
            best = (weight, region_winding, inconsistencies)
    return best[1], regions, best[2]
