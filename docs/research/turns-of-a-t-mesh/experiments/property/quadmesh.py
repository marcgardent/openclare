"""Quad meshes of closed surfaces made without the engine: a polygon mesh (a
grid on a torus, a cube, the convex hull of random points on a sphere or its
dual), a few random moves that turn two quads into a triangle and a pentagon,
then one step of Catmull–Clark. The result is an all-quad mesh whose
irregular vertices are the centres of the faces that are not quads (valence
the degree of the face) and the original vertices whose degree is not four.

Surfaces with borders (a grid as a disc or a tube, the cube without a face,
a hull with holes) are subdivided the same way and then doubled: two mirror
copies glued along the borders, a closed quad mesh with a mirror involution
(`double`).

Python standard library, plus `numpy` and `scipy` for the convex hull.
"""

import math
import random

import numpy as np
from scipy.spatial import ConvexHull


# The open families: surfaces with borders, made closed by `double`.
OPEN_KINDS = ("disc", "tube", "cap", "holed")


def border_cycles(directed_edges):
    """The border of an oriented surface given by the directed edges of its
    faces: the edges without a twin, chained into cycles of vertices with the
    surface on the left, each from its lowest vertex, in order of that
    vertex. Raises if a vertex has more than two border edges."""
    out_edge, in_count = {}, {}
    for (a, b) in directed_edges:
        if (b, a) in directed_edges:
            continue
        if a in out_edge or in_count.get(b, 0):
            raise ValueError(f"vertex {a if a in out_edge else b} has more than two border edges")
        out_edge[a] = b
        in_count[b] = 1
    cycles, seen = [], set()
    for start in sorted(out_edge):
        if start in seen:
            continue
        cycle, v = [], start
        while v not in seen:
            seen.add(v)
            cycle.append(v)
            v = out_edge[v]
        assert v == start
        cycles.append(cycle)
    return cycles


class PolygonMesh:
    """Faces as lists of vertex indices, every face counter-clockwise when
    seen from outside, every edge shared by exactly two faces (or by one, on
    a border)."""

    def __init__(self, positions, faces):
        self.positions = [tuple(p) for p in positions]
        self.faces = [list(f) for f in faces]

    def check(self, closed=True):
        """With `closed`, raises on an edge with one face and returns the
        mesh; otherwise returns the border components (`border_cycles`)."""
        seen = {}
        for k, face in enumerate(self.faces):
            if len(face) < 3 or len(set(face)) != len(face):
                raise ValueError(f"face {k} is degenerate: {face}")
            for i, a in enumerate(face):
                b = face[(i + 1) % len(face)]
                if (a, b) in seen:
                    raise ValueError(f"edge {a}->{b} runs the same way in faces {seen[(a, b)]} and {k}")
                seen[(a, b)] = k
        if not closed:
            return border_cycles(seen)
        for (a, b) in seen:
            if (b, a) not in seen:
                raise ValueError(f"edge {a}-{b} has one face only: not closed")
        return self

    def degrees(self):
        degree = [0] * len(self.positions)
        for face in self.faces:
            for v in face:
                degree[v] += 1
        return degree

    def euler(self):
        edges = {(min(a, b), max(a, b)) for f in self.faces for a, b in zip(f, f[1:] + f[:1])}
        return len(self.positions) - len(edges) + len(self.faces)

    def tri_pent_move(self, rng):
        """Replaces two adjacent quads `p q r s` and `q p t u` (sharing the edge
        `p q`) by the triangle `q r s` and the pentagon `s p t u q`: after
        Catmull–Clark, two 3–5 pairs. Returns False when no edge allows it."""
        where = {}
        for k, face in enumerate(self.faces):
            for i, a in enumerate(face):
                where[(a, face[(i + 1) % len(face)])] = (k, i)
        candidates = [(a, b) for (a, b), (k, i) in where.items()
                      if a < b and (b, a) in where and len(self.faces[k]) == 4
                      and len(self.faces[where[(b, a)][0]]) == 4]
        rng.shuffle(candidates)
        for (p, q) in candidates:
            k, i = where[(p, q)]
            m, j = where[(q, p)]
            A, B = self.faces[k], self.faces[m]
            p_, q_, r, s = A[i], A[(i + 1) % 4], A[(i + 2) % 4], A[(i + 3) % 4]
            q2, p2, t, u = B[j], B[(j + 1) % 4], B[(j + 2) % 4], B[(j + 3) % 4]
            assert (p_, q_, q2, p2) == (p, q, q, p)
            if len({p, q, r, s, t, u}) != 6:
                continue
            # The edge p q goes; the diagonal q s splits the hexagon p t u q r s.
            self.faces[k] = [q, r, s]
            self.faces[m] = [s, p, t, u, q]
            return True
        return False


def torus_grid(width, height):
    """A grid of `width` by `height` quads glued into a torus."""
    positions, faces = [], []
    for j in range(height):
        for i in range(width):
            a, b = 2 * math.pi * i / width, 2 * math.pi * j / height
            positions.append(((2 + math.cos(b)) * math.cos(a), (2 + math.cos(b)) * math.sin(a), math.sin(b)))
    at = lambda i, j: (j % height) * width + (i % width)
    for j in range(height):
        for i in range(width):
            faces.append([at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)])
    return PolygonMesh(positions, faces).check()


def cube():
    positions = [(x, y, z) for z in (-1, 1) for y in (-1, 1) for x in (-1, 1)]
    faces = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]]
    return PolygonMesh(positions, faces).check()


def box(a, b, c):
    """The surface of an `a` by `b` by `c` box of unit cubes, as quads: a sphere
    with eight vertices of valence three and, for `c` large, a tube between
    two caps, where separatrices wind and dislocations sit between parallel
    turns."""
    index = {}
    positions = []

    def vertex(x, y, z):
        key = (x, y, z)
        if key not in index:
            index[key] = len(positions)
            positions.append((float(x), float(y), float(z)))
        return index[key]

    faces = []
    for x in range(a):
        for y in range(b):
            faces.append([vertex(x, y, 0), vertex(x, y + 1, 0), vertex(x + 1, y + 1, 0), vertex(x + 1, y, 0)])
            faces.append([vertex(x, y, c), vertex(x + 1, y, c), vertex(x + 1, y + 1, c), vertex(x, y + 1, c)])
    for x in range(a):
        for z in range(c):
            faces.append([vertex(x, 0, z), vertex(x + 1, 0, z), vertex(x + 1, 0, z + 1), vertex(x, 0, z + 1)])
            faces.append([vertex(x, b, z), vertex(x, b, z + 1), vertex(x + 1, b, z + 1), vertex(x + 1, b, z)])
    for y in range(b):
        for z in range(c):
            faces.append([vertex(0, y, z), vertex(0, y, z + 1), vertex(0, y + 1, z + 1), vertex(0, y + 1, z)])
            faces.append([vertex(a, y, z), vertex(a, y + 1, z), vertex(a, y + 1, z + 1), vertex(a, y, z + 1)])
    return PolygonMesh(positions, faces).check()


def sphere_hull(rng, count):
    """The convex hull of `count` random points on the unit sphere: a
    triangulation of the sphere with vertices of degree about six."""
    points = np.array([rng.gauss(0, 1) for _ in range(3 * count)]).reshape(count, 3)
    points /= np.linalg.norm(points, axis=1)[:, None]
    hull = ConvexHull(points)
    faces = []
    for simplex, equation in zip(hull.simplices, hull.equations):
        a, b, c = (points[v] for v in simplex)
        outward = np.cross(b - a, c - a) @ equation[:3] > 0
        faces.append(list(simplex) if outward else [simplex[0], simplex[2], simplex[1]])
    return PolygonMesh([tuple(p) for p in points], faces).check()


def dual(mesh):
    """The dual polygon mesh: a vertex per face, a face per vertex, the faces
    round a vertex in counter-clockwise order."""
    where = {}
    for k, face in enumerate(mesh.faces):
        for i, a in enumerate(face):
            where[(a, face[(i + 1) % len(face)])] = k
    centres = []
    for face in mesh.faces:
        p = [sum(mesh.positions[v][c] for v in face) / len(face) for c in range(3)]
        centres.append(tuple(p))
    out_edges = {}
    for (a, b) in where:
        out_edges.setdefault(a, []).append(b)
    faces = []
    for v in range(len(mesh.positions)):
        # Round v: from the face holding a -> b, the next face counter-clockwise
        # holds the edge b -> ... no: the face holding (v -> w) is followed, going
        # counter-clockwise round v, by the face holding (next vertex -> v)'s twin.
        start = out_edges[v][0]
        cycle, w = [], start
        while True:
            k = where[(v, w)]
            cycle.append(k)
            face = mesh.faces[k]
            i = face.index(v)
            w = face[(i - 1) % len(face)]
            if w == start:
                break
        faces.append(cycle)
    return PolygonMesh(centres, faces).check()


def disc_grid(width, height):
    """A grid of `width` by `height` quads: a disc, its four corners of angle
    one quarter turn."""
    positions = [(float(i), float(j), 0.0) for j in range(height + 1) for i in range(width + 1)]
    at = lambda i, j: j * (width + 1) + i
    faces = [[at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)] for j in range(height) for i in range(width)]
    mesh = PolygonMesh(positions, faces)
    mesh.check(closed=False)
    return mesh


def tube_grid(width, height):
    """A grid of `width` quads round by `height` along, glued into a
    cylinder: an annulus, two border circles."""
    positions = []
    for j in range(height + 1):
        for i in range(width):
            a = 2 * math.pi * i / width
            positions.append((math.cos(a), math.sin(a), float(j)))
    at = lambda i, j: j * width + (i % width)
    faces = [[at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)] for j in range(height) for i in range(width)]
    mesh = PolygonMesh(positions, faces)
    mesh.check(closed=False)
    return mesh


def cap():
    """The cube without its top face: a disc whose border is straight (each
    of its vertices has two faces)."""
    mesh = cube()
    mesh.faces = [face for face in mesh.faces if face != [4, 5, 7, 6]]
    mesh.check(closed=False)
    return mesh


def compact(positions, faces):
    """The mesh on the vertices its faces use, renumbered in order."""
    used = sorted({v for face in faces for v in face})
    number = {v: k for k, v in enumerate(used)}
    return PolygonMesh([positions[v] for v in used], [[number[v] for v in face] for face in faces])


def holed_hull(rng, count, holes, tries=50):
    """The convex hull of `count` random points on the sphere with the faces
    round `holes` random vertices removed: a sphere with `holes` holes. The
    vertices are drawn again until the holes do not touch (no vertex shared
    between the faces removed for different holes), the points too after
    `tries` draws."""
    while True:
        mesh = sphere_hull(rng, count)
        closed_star = [set() for _ in range(count)]
        for face in mesh.faces:
            for v in face:
                closed_star[v].update(face)
        for _ in range(tries):
            order = list(range(count))
            rng.shuffle(order)
            chosen, used = [], set()
            for v in order:
                if closed_star[v] & used:
                    continue
                chosen.append(v)
                used |= closed_star[v]
                if len(chosen) == holes:
                    break
            if len(chosen) == holes:
                faces = [face for face in mesh.faces if not set(face) & set(chosen)]
                out = compact(mesh.positions, faces)
                border = out.check(closed=False)
                assert len(border) == holes, border
                return out


class QuadMesh:
    """An all-quad mesh of an oriented surface, with its half-edges; closed
    unless `closed` is False, then `border` lists its border components
    (`border_cycles`). The walks round a vertex (`star`, `straight`) need the
    vertex off the border."""

    def __init__(self, positions, faces, closed=True):
        self.positions = positions
        self.faces = [tuple(f) for f in faces]
        if any(len(f) != 4 for f in self.faces):
            raise ValueError("not all faces are quads")
        self.where = {}
        for k, face in enumerate(self.faces):
            for i in range(4):
                edge = (face[i], face[(i + 1) % 4])
                if edge in self.where:
                    raise ValueError(f"edge {edge} twice the same way")
                self.where[edge] = (k, i)
        self.border = []
        if closed:
            for (a, b) in self.where:
                if (b, a) not in self.where:
                    raise ValueError(f"edge {a}-{b} is on a border")
        else:
            self.border = border_cycles(self.where)
        self.valence = [0] * len(positions)
        for face in self.faces:
            for v in face:
                self.valence[v] += 1

    def next_in_face(self, a, b):
        k, i = self.where[(a, b)]
        face = self.faces[k]
        return face[(i + 1) % 4], face[(i + 2) % 4]

    def prev_in_face(self, a, b):
        k, i = self.where[(a, b)]
        face = self.faces[k]
        return face[(i - 1) % 4], face[i]

    def next_ccw(self, a, b):
        """The outgoing edge after `a -> b` counter-clockwise round `a`."""
        p, _ = self.prev_in_face(a, b)
        return a, p

    def star(self, v):
        """The outgoing edges of `v`, counter-clockwise."""
        first = next((a, b) for (a, b) in self.where if a == v)
        out, edge = [], first
        while True:
            out.append(edge)
            edge = self.next_ccw(*edge)
            if edge == first:
                return out

    def straight(self, a, b):
        """At a regular vertex `b`, the edge continuing `a -> b` straight on."""
        assert self.valence[b] == 4
        edge = (b, a)
        edge = self.next_ccw(*edge)
        edge = self.next_ccw(*edge)
        return edge

    def euler(self):
        edges = {(min(a, b), max(a, b)) for (a, b) in self.where}
        return len(self.positions) - len(edges) + len(self.faces)


def catmull_clark(mesh, closed=True):
    """One step of Catmull–Clark on a polygon mesh, in connectivity: a vertex
    per original vertex, edge and face; positions are only there to draw.
    Without `closed`, the mesh may have borders: a border edge has its edge
    vertex too, and the border of the result is the border subdivided, each
    original border vertex with as many quads as it had faces, each edge
    vertex on it with two."""
    n = len(mesh.positions)
    positions = list(mesh.positions)
    edge_vertex = {}
    for face in mesh.faces:
        for i, a in enumerate(face):
            b = face[(i + 1) % len(face)]
            key = (min(a, b), max(a, b))
            if key not in edge_vertex:
                edge_vertex[key] = len(positions)
                positions.append(tuple((mesh.positions[a][c] + mesh.positions[b][c]) / 2 for c in range(3)))
    faces = []
    for face in mesh.faces:
        centre = len(positions)
        positions.append(tuple(sum(mesh.positions[v][c] for v in face) / len(face) for c in range(3)))
        for i, v in enumerate(face):
            before, after = face[(i - 1) % len(face)], face[(i + 1) % len(face)]
            e_after = edge_vertex[(min(v, after), max(v, after))]
            e_before = edge_vertex[(min(before, v), max(before, v))]
            faces.append((v, e_after, centre, e_before))
    assert n <= len(positions)
    quad = QuadMesh(positions, faces, closed=closed)
    if not closed:
        degree = mesh.degrees()
        expected, angle = set(), {}
        for cycle in mesh.check(closed=False):
            for i, a in enumerate(cycle):
                b = cycle[(i + 1) % len(cycle)]
                e = edge_vertex[(min(a, b), max(a, b))]
                expected.update({(a, e), (e, b)})
                angle[a], angle[e] = degree[a], 2
        found = {(c[i], c[(i + 1) % len(c)]) for c in quad.border for i in range(len(c))}
        assert found == expected, "the border of the subdivision is not the border subdivided"
        assert all(quad.valence[v] == a for v, a in angle.items())
    return quad


class Double:
    """A closed quad mesh made of two copies of an open one glued along the
    border: the first copy as it is, the second mirrored (every face
    reversed), one vertex for each border vertex, used by both. Vertices are
    numbered border first, then the interior of the first copy, then the
    interior of the second in the same order, and faces the first copy's then
    their mirrors in the same order, so that the second copy lists everything
    as the first does.

    `quad` the closed mesh; `mirror` the vertex involution (border vertices
    fixed) and `mirror_face` the face involution, orientation reversed;
    `copy[f]` 0 or 1; `border` the border components in the double's
    numbering, the first copy on the left; `angle[v]` the faces of the open
    mesh at a border vertex (valence `2 angle` in the double);
    `border_edges` the undirected border edges; `open` the open mesh."""


def double(open_quad):
    border_vertices = sorted(v for cycle in open_quad.border for v in cycle)
    on_border = set(border_vertices)
    interior = [v for v in range(len(open_quad.positions)) if v not in on_border]
    if any(open_quad.valence[v] == 0 for v in interior):
        raise ValueError("a vertex without a face")
    number = {}
    for v in border_vertices + interior:
        number[v] = len(number)
    second = {v: number[v] + (0 if v in on_border else len(interior)) for v in number}
    total = len(number) + len(interior)
    positions = [None] * total
    for v in number:
        positions[number[v]] = open_quad.positions[v]
        positions[second[v]] = open_quad.positions[v]
    faces = [tuple(number[v] for v in face) for face in open_quad.faces]
    faces += [(second[a], second[d], second[c], second[b]) for (a, b, c, d) in open_quad.faces]
    D = Double()
    D.open = open_quad
    D.quad = QuadMesh(positions, faces)
    count = len(open_quad.faces)
    D.mirror = list(range(total))
    for v in interior:
        D.mirror[number[v]], D.mirror[second[v]] = second[v], number[v]
    D.mirror_face = [f + count for f in range(count)] + list(range(count))
    D.copy = [0] * count + [1] * count
    D.border = [[number[v] for v in cycle] for cycle in open_quad.border]
    D.angle = {number[v]: open_quad.valence[v] for v in border_vertices}
    D.border_edges = {(min(c[i], c[(i + 1) % len(c)]), max(c[i], c[(i + 1) % len(c)]))
                      for c in D.border for i in range(len(c))}
    # The checks: an involution of the quad mesh reversing orientation, the
    # valence of a border vertex twice its angle, and the Euler
    # characteristic twice the open mesh's (the border components are circles).
    m = D.mirror
    assert all(m[m[v]] == v for v in range(total))
    for f, face in enumerate(D.quad.faces):
        for i in range(4):
            a, b = face[i], face[(i + 1) % 4]
            assert D.quad.where[(m[b], m[a])][0] == D.mirror_face[f], "the mirror does not reverse the faces"
    assert all(D.quad.valence[v] == 2 * a for v, a in D.angle.items())
    assert D.quad.euler() == 2 * open_quad.euler(), "the double has the wrong Euler characteristic"
    return D


def random_polygon_mesh(rng, kind):
    """One of the families: ('torus', w, h, moves), ('cube', moves),
    ('box', a, b, c, moves), ('hull', count), ('dual', count); with borders
    (`OPEN_KINDS`), ('disc', w, h, moves), ('tube', w, h, moves),
    ('cap', moves), ('holed', count, holes)."""
    if kind[0] == "torus":
        mesh = torus_grid(kind[1], kind[2])
        moves = kind[3]
    elif kind[0] == "cube":
        mesh = cube()
        moves = kind[1]
    elif kind[0] == "box":
        mesh = box(kind[1], kind[2], kind[3])
        moves = kind[4]
    elif kind[0] == "hull":
        return sphere_hull(rng, kind[1])
    elif kind[0] == "dual":
        return dual(sphere_hull(rng, kind[1]))
    elif kind[0] == "disc":
        mesh = disc_grid(kind[1], kind[2])
        moves = kind[3]
    elif kind[0] == "tube":
        mesh = tube_grid(kind[1], kind[2])
        moves = kind[3]
    elif kind[0] == "cap":
        mesh = cap()
        moves = kind[1]
    elif kind[0] == "holed":
        return holed_hull(rng, kind[1], kind[2])
    else:
        raise ValueError(kind)
    for _ in range(moves):
        if not mesh.tri_pent_move(rng):
            break
    if kind[0] in OPEN_KINDS:
        mesh.check(closed=False)
        return mesh
    return mesh.check()
