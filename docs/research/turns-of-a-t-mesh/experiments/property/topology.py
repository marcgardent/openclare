"""The topology the paper assumes, computed: the developed positions of the
cone points as linear forms in the lengths (one chart along a tree of the
rectangles), the feet of the arcs (Lemma "Combs"), the separatrices that
wind and stop on a separatrix of their own vertex (Proposition "A separatrix
that winds"), and the rank of the Burgers pairing (Proposition "The Burgers
pairing"). Regions, Euler characteristics, curves and colourings are in
`grid.py`.
"""

from fractions import Fraction

import sympy

from tracing import BORDER
from walks import merged, turned


def add_forms(a, b):
    return (merged(list(a[0].items()) + list(b[0].items())), merged(list(a[1].items()) + list(b[1].items())))


def negate_forms(a):
    return ({arc: -c for arc, c in a[0].items()}, {arc: -c for arc, c in a[1].items()})


class Motion:
    """x -> R^k x + t, with t a pair of forms in the lengths."""

    def __init__(self, k=0, t=({}, {})):
        self.k, self.t = k % 4, t

    def after(self, other):
        return Motion(self.k + other.k, add_forms(turned(self.k, other.t), self.t))

    def apply(self, x):
        return add_forms(turned(self.k, x), self.t)


class Chart:
    """One chart of the surface cut along a tree: the motion from the frame
    of each rectangle to the frame of rectangle 0, symbolic in the lengths
    (`develop.py` of the Lean check, with forms in place of numbers)."""

    def __init__(self, tmesh, rims):
        self.T, self.rims = tmesh, rims
        self.place = {}
        for p, rim in enumerate(rims):
            for k, (half, _) in enumerate(rim.halves):
                self.place[half] = (p, k)
        self.D = {0: Motion()}
        self.crossed = set()
        queue = [0]
        while queue:
            p = queue.pop(0)
            for half, _ in rims[p].halves:
                q = self.place[half ^ 1][0]
                if q not in self.D:
                    self.D[q] = self.D[p].after(self.change(half ^ 1))
                    self.crossed.add(half // 2)
                    queue.append(q)
        self.tree_arcs = [a for a in range(len(tmesh.arcs)) if a not in self.crossed]

    def start_form(self, p, k):
        rim = self.rims[p]
        if k == len(rim.halves):
            return ({}, {})
        return rim.start_expression[k]

    def change(self, half):
        """The change of frame from the rectangle holding `half` to the one
        holding its twin."""
        p, k = self.place[half]
        q, k2 = self.place[half ^ 1]
        j = (self.rims[q].halves[k2][1] + 2 - self.rims[p].halves[k][1]) % 4
        a = self.start_form(p, k)
        b = self.start_form(q, k2 + 1)
        return Motion(j, add_forms(b, negate_forms(turned(j, a))))

    def node_positions(self):
        """For each node, its developed position at the start of its first
        half (one of its corners), as a pair of forms."""
        out = [None] * len(self.T.nodes)
        for a, arc in enumerate(self.T.arcs):
            for half in (2 * a, 2 * a + 1):
                n = arc["nodes"][half % 2]
                if out[n] is None:
                    p, k = self.place[half]
                    out[n] = self.D[p].apply(self.start_form(p, k))
        return out


def form_matrix(forms, n):
    M = sympy.zeros(len(forms), n)
    for r, form in enumerate(forms):
        for arc, c in form.items():
            M[r, arc] = c
    return M


def pairing_rank(tmesh, rims, basis):
    """The rank of the developed positions of the cone points, as forms on
    the balanced lengths (the image of the Burgers pairing), against the
    dimension of the balanced lengths; and the cone points of valence
    divisible by four, which the proposition excludes."""
    chart = Chart(tmesh, rims)
    positions = chart.node_positions()
    quad = tmesh.quad
    forms, multiples_of_four = [], 0
    for n, v in enumerate(tmesh.nodes):
        if quad.valence[v] == 4:
            continue
        if quad.valence[v] % 4 == 0:
            multiples_of_four += 1
        forms.extend(positions[n])
    n = len(tmesh.arcs)
    if not forms or not basis:
        return {"rank": 0, "dimension": len(basis), "multiples_of_four": multiples_of_four}
    F = form_matrix(forms, n)
    N = sympy.Matrix([[int(x) for x in v] for v in basis]).T
    return {"rank": (F * N).rank(), "dimension": len(basis), "multiples_of_four": multiples_of_four}


def connections(tmesh):
    """The traces that stop at a singular vertex (separatrix connections)."""
    quad = tmesh.quad
    return sum(1 for end in tmesh.trace_end if end is not None and quad.valence[end] != 4)


def feet(tmesh):
    """Per arc, the foot of each end: the node itself if singular, else the
    start of the single trace through it perpendicular to the arc; None
    where that trace is not unique (the hypothesis of Lemma "Combs") or is
    the border pre-traced on a double, which no trace started."""
    quad = tmesh.quad
    out = []
    # Each trail edge belongs to the trace that ran it first (two run the
    # last edge before a head-on stop; the first is the one that went on).
    first = {edge: ids[0] for edge, ids in tmesh.trace_of_edge.items()}
    for arc in tmesh.arcs:
        own = {first[(min(a, b), max(a, b))] for (a, b) in arc["edges"]}
        pair = []
        for end, vertex_edge in ((0, arc["edges"][0]), (1, arc["edges"][-1])):
            v = vertex_edge[0] if end == 0 else vertex_edge[1]
            if quad.valence[v] != 4:
                pair.append(("singular", v))
                continue
            # The trail edges at v off the line of the arc.
            line = {vertex_edge, (vertex_edge[1], vertex_edge[0])}
            along = set()
            for (p, q) in quad.star(v):
                key = (min(p, q), max(p, q))
                if key not in first or first[key] in own:
                    continue
                along.add(first[key])
            if len(along) == 1 and BORDER not in along:
                pair.append(("foot", tmesh.trace_start[next(iter(along))]))
            else:
                pair.append((None, sorted(along)))
        out.append(pair)
    return out


def winding_separatrices(tmesh):
    """Traces that stop on another trace of their own vertex, with the
    distance along that trace from the vertex to the stop, in arcs of the
    T-mesh (a sum of arc lengths at any lengths), and the first arc of the
    winding trace."""
    quad = tmesh.quad

    def arcs_along(m):
        """The arcs of trace m in order from its start, each with the vertex
        it ends at."""
        a, b = tmesh.trace_first_edge[m]
        end = tmesh.trace_end[m]
        out = []
        while True:
            arc, _ = tmesh.arc_of_edge[(a, b)]
            last = tmesh.arcs[arc]["edges"][-1][1] if tmesh.arc_of_edge[(a, b)][1] == 0 \
                else tmesh.arcs[arc]["edges"][0][0]
            out.append((arc, last))
            if last == end or len(out) > len(tmesh.arcs):
                return out
            # Straight on from the end of the arc.
            edges = tmesh.arcs[arc]["edges"]
            a, b = edges[-1] if tmesh.arc_of_edge[(a, b)][1] == 0 else (edges[0][1], edges[0][0])
            if quad.valence[b] != 4:
                return out
            a, b = quad.straight(a, b)

    out = []
    for m, end in enumerate(tmesh.trace_end):
        v = tmesh.trace_start[m]
        if end is None or end == v or quad.valence[end] != 4:
            continue
        # Which traces run through `end`, other than m, from the same vertex?
        others = set()
        for (p, q) in quad.star(end):
            key = (min(p, q), max(p, q))
            for t in tmesh.trace_of_edge.get(key, []):
                if t != m and t != BORDER and tmesh.trace_start[t] == v:
                    others.add(t)
        if len(others) != 1:
            continue
        tau = next(iter(others))
        # The arcs of tau from v up to the node at `end`.
        delta_arcs = []
        for a, last in arcs_along(tau):
            delta_arcs.append(a)
            if last == end:
                break
        else:
            continue
        first = arcs_along(m)[0][0]
        out.append({"trace": m, "vertex": v, "stop": end, "tau": tau, "delta_arcs": delta_arcs, "first_arc": first})
    return out


def beside_stations(tmesh, walker, arc, vertex):
    """The two stations half a quad from the singular `vertex` on the sides
    perpendicular to `arc` at it, one on each rectangle holding the arc: a
    walk from there runs beside the arc."""
    node = tmesh.node_of[vertex]
    out = []
    for half in (2 * arc, 2 * arc + 1):
        p, k = walker.place[half]
        rim = walker.rims[p]
        side = rim.halves[k][1]
        if tmesh.arcs[arc]["nodes"][half % 2] == node:
            # The half starts at the vertex: the previous side ends there.
            previous = (side - 1) % 4
            length = rim.side_length(previous, walker.lengths)
            out.append((p, previous, length - Fraction(1, 2)))
        else:
            # The half ends at the vertex: the next side starts there.
            following = (side + 1) % 4
            out.append((p, following, Fraction(1, 2)))
    return out
