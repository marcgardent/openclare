"""The quad mesh of a T-mesh at whole balanced lengths: the integer grid of
each rectangle, glued along the arcs. Built refined by two, as the Lean
model of Gauss–Bonnet does: a vertex at every half-integer point of every
rectangle, so that the dual line of a strip (at a half-integer offset) and
the sides of the rectangles are both paths of the fine grid. The coarse quad
graph is the fine vertices with whole coordinates.

Coordinates in a rectangle are those of its frame: side 0 from `(0, 0)` to
`(w, 0)`, side 1 up to `(w, h)`, side 2 back to `(0, h)`, side 3 down to the
origin; in the fine grid they are doubled, `x` in `0..2w`, `y` in `0..2h`.
"""

from fractions import Fraction

from walks import DIRECTION, Rim


class Grid:
    def __init__(self, tmesh, whole):
        self.T = tmesh
        self.whole = [int(x) for x in whole]
        self.rims = [Rim(patch["rim"], [Fraction(x) for x in self.whole]) for patch in tmesh.patches]
        self.size = []
        for rim in self.rims:
            w = int(rim.side_length(0, self.whole))
            h = int(rim.side_length(1, self.whole))
            self.size.append((w, h))
        # Fine vertices: (patch, x, y) -> id, with union-find over the arcs.
        self.index = {}
        self.parent = []
        for p, (w, h) in enumerate(self.size):
            for x in range(2 * w + 1):
                for y in range(2 * h + 1):
                    self.index[(p, x, y)] = len(self.parent)
                    self.parent.append(len(self.parent))
        # Where each half starts on its rim and which way it runs, in fine units.
        self.half_place = {}
        for p, rim in enumerate(self.rims):
            for k, (half, side) in enumerate(rim.halves):
                start = rim.start[k]
                self.half_place[half] = (p, 2 * int(start[0]), 2 * int(start[1]), DIRECTION[side])
        for a, arc in enumerate(tmesh.arcs):
            length = 2 * self.whole[a]
            p, x0, y0, d0 = self.half_place[2 * a]
            q, x1, y1, d1 = self.half_place[2 * a + 1]
            for t in range(length + 1):
                u = self.index[(p, x0 + t * d0[0], y0 + t * d0[1])]
                v = self.index[(q, x1 + (length - t) * d1[0], y1 + (length - t) * d1[1])]
                self.union(u, v)
        self.vertex = {}      # (patch, x, y) -> glued vertex id
        self.representative = {}
        for key, i in self.index.items():
            r = self.find(i)
            if r not in self.representative:
                self.representative[r] = len(self.representative)
            self.vertex[key] = self.representative[r]
        self.vertices = len(self.representative)
        # Fine faces, counter-clockwise in the frame of their rectangle.
        self.faces = []
        self.face_patch = []
        for p, (w, h) in enumerate(self.size):
            for x in range(2 * w):
                for y in range(2 * h):
                    self.faces.append((self.vertex[(p, x, y)], self.vertex[(p, x + 1, y)],
                                       self.vertex[(p, x + 1, y + 1)], self.vertex[(p, x, y + 1)]))
                    self.face_patch.append(p)
        self.edges = set()
        for face in self.faces:
            for i in range(4):
                a, b = face[i], face[(i + 1) % 4]
                self.edges.add((min(a, b), max(a, b)))
        # The nodes, as fine vertices, and the cone points.
        self.node_vertex = []
        for n in range(len(tmesh.nodes)):
            self.node_vertex.append(None)
        for a, arc in enumerate(tmesh.arcs):
            p, x0, y0, d0 = self.half_place[2 * a]
            length = 2 * self.whole[a]
            self.node_vertex[arc["nodes"][0]] = self.vertex[(p, x0, y0)]
            self.node_vertex[arc["nodes"][1]] = self.vertex[(p, x0 + length * d0[0], y0 + length * d0[1])]
        self.cones = {}       # fine vertex -> valence, for the singular nodes
        quad = tmesh.quad
        for n, v in enumerate(tmesh.nodes):
            if quad.valence[v] != 4:
                self.cones[self.node_vertex[n]] = quad.valence[v]

    def union(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra != rb:
            self.parent[max(ra, rb)] = min(ra, rb)

    def find(self, a):
        while self.parent[a] != a:
            self.parent[a] = self.parent[self.parent[a]]
            a = self.parent[a]
        return a

    def euler(self):
        return self.vertices - len(self.edges) + len(self.faces)

    def coarse_vertices(self):
        return {v for (p, x, y), v in self.vertex.items() if x % 2 == 0 and y % 2 == 0}

    def coarse_edges(self):
        """The edges of the quad graph (whole lengths), as pairs of fine vertices."""
        out = set()
        for p, (w, h) in enumerate(self.size):
            for x in range(0, 2 * w + 1, 2):
                for y in range(0, 2 * h + 1, 2):
                    v = self.vertex[(p, x, y)]
                    if x + 2 <= 2 * w:
                        u = self.vertex[(p, x + 2, y)]
                        out.add((min(u, v), max(u, v)))
                    if y + 2 <= 2 * h:
                        u = self.vertex[(p, x, y + 2)]
                        out.add((min(u, v), max(u, v)))
        return out

    def two_colouring(self):
        """A proper 2-colouring of the quad graph, or None with an odd cycle
        (the graph is bipartite on a sphere, not always elsewhere)."""
        adjacency = {}
        for (u, v) in self.coarse_edges():
            adjacency.setdefault(u, []).append(v)
            adjacency.setdefault(v, []).append(u)
        colour = {}
        for start in adjacency:
            if start in colour:
                continue
            colour[start] = 0
            queue = [start]
            while queue:
                u = queue.pop()
                for v in adjacency[u]:
                    if v not in colour:
                        colour[v] = 1 - colour[u]
                        queue.append(v)
                    elif colour[v] == colour[u]:
                        return None
        return colour

    def turn_curve(self, turn):
        """The closed curve of a turn at these lengths as a cycle of fine
        vertices: the walk from its half-integer offset, then the closing
        segment along the first side back to the start. The turn's offset must
        be a half-integer and its crossings those of the walk at these
        lengths."""
        T = self.T
        p, side, offset = turn["patch"], turn["side"], turn["offset"]
        assert offset.denominator == 2, "the curve needs a half-integer offset"
        path = []

        def along(p, side, offset):
            """The fine vertices of the walk across rectangle p from `offset`
            on `side`, from that side to the opposite one."""
            w, h = self.size[p]
            o = int(2 * offset)
            out = []
            if side == 0:
                out = [(o, y) for y in range(0, 2 * h + 1)]
            elif side == 2:
                out = [(2 * w - o, y) for y in range(2 * h, -1, -1)]
            elif side == 1:
                out = [(x, o) for x in range(2 * w, -1, -1)]
            else:
                out = [(x, 2 * h - o) for x in range(0, 2 * w + 1)]
            return [self.vertex[(p, x, y)] for (x, y) in out]

        rim = self.rims[p]
        here = (p, side, offset)
        for (q, half, nxt, into) in turn["crossings"]:
            assert q == here[0]
            segment = along(*here)
            path.extend(segment[:-1])
            next_rim = self.rims[nxt]
            k_next = next(k for k, (hh, _) in enumerate(next_rim.halves) if hh == half ^ 1)
            next_side = next_rim.halves[k_next][1]
            before_twin = sum((Fraction(self.whole[next_rim.halves[m][0] // 2])
                               for m in range(next_rim.side_start[next_side], k_next)), Fraction(0))
            here = (nxt, next_side, before_twin + (self.whole[half // 2] - into))
        # Back on the first side at here[2]: the closing segment along it.
        assert here[0] == p and here[1] == side
        end, start = int(2 * here[2]), int(2 * offset)
        w, h = self.size[p]
        step = 1 if start > end else -1
        for o in range(end, start, step):
            if side == 0:
                key = (o, 0)
            elif side == 1:
                key = (2 * w, o)
            elif side == 2:
                key = (2 * w - o, 2 * h)
            else:
                key = (0, 2 * h - o)
            path.append(self.vertex[(p, key[0], key[1])])
        return path


def regions(grid, curves):
    """The regions the closed curves (cycles of fine vertices) cut the surface
    into: the components of the fine faces not crossing them, each with its
    Euler characteristic as a surface with boundary and the cone points
    strictly inside it."""
    cut = set()
    for curve in curves:
        for i, a in enumerate(curve):
            b = curve[(i + 1) % len(curve)]
            cut.add((min(a, b), max(a, b)))
    on_curve = {v for curve in curves for v in curve}
    face_of_edge = {}
    for f, face in enumerate(grid.faces):
        for i in range(4):
            a, b = face[i], face[(i + 1) % 4]
            face_of_edge.setdefault((min(a, b), max(a, b)), []).append(f)
    region = [None] * len(grid.faces)
    out = []
    for seed in range(len(grid.faces)):
        if region[seed] is not None:
            continue
        k = len(out)
        region[seed] = k
        queue, faces = [seed], [seed]
        while queue:
            f = queue.pop()
            face = grid.faces[f]
            for i in range(4):
                a, b = face[i], face[(i + 1) % 4]
                e = (min(a, b), max(a, b))
                if e in cut:
                    continue
                for g in face_of_edge[e]:
                    if region[g] is None:
                        region[g] = k
                        queue.append(g)
                        faces.append(g)
        vertices, edges = set(), set()
        for f in faces:
            face = grid.faces[f]
            vertices.update(face)
            for i in range(4):
                a, b = face[i], face[(i + 1) % 4]
                edges.add((min(a, b), max(a, b)))
        inside = {v: k_ for v, k_ in grid.cones.items() if v in vertices and v not in on_curve}
        out.append({"faces": len(faces), "euler": len(vertices) - len(edges) + len(faces),
                    "cones": inside, "index": sum(4 - k_ for k_ in inside.values())})
    return out


def self_crossings(curve):
    """The vertices a closed curve visits more than once."""
    seen, twice = set(), set()
    for v in curve:
        if v in seen:
            twice.add(v)
        seen.add(v)
    return twice
