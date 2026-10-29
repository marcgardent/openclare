"""Quad meshes of closed surfaces made without the engine: a polygon mesh (a
grid on a torus, a cube, the convex hull of random points on a sphere or its
dual), a few random moves that turn two quads into a triangle and a pentagon,
then one step of Catmull–Clark. The result is an all-quad mesh whose
irregular vertices are the centres of the faces that are not quads (valence
the degree of the face) and the original vertices whose degree is not four.

Python standard library, plus `numpy` and `scipy` for the convex hull.
"""

import math
import random

import numpy as np
from scipy.spatial import ConvexHull


class PolygonMesh:
    """Faces as lists of vertex indices, every face counter-clockwise when
    seen from outside, every edge shared by exactly two faces."""

    def __init__(self, positions, faces):
        self.positions = [tuple(p) for p in positions]
        self.faces = [list(f) for f in faces]

    def check(self):
        seen = {}
        for k, face in enumerate(self.faces):
            if len(face) < 3 or len(set(face)) != len(face):
                raise ValueError(f"face {k} is degenerate: {face}")
            for i, a in enumerate(face):
                b = face[(i + 1) % len(face)]
                if (a, b) in seen:
                    raise ValueError(f"edge {a}->{b} runs the same way in faces {seen[(a, b)]} and {k}")
                seen[(a, b)] = k
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
        edges = sum(len(f) for f in self.faces) // 2
        return len(self.positions) - edges + len(self.faces)

    def tri_pent_move(self, rng):
        """Replaces two adjacent quads `p q r s` and `q p t u` (sharing the edge
        `p q`) by the triangle `q r s` and the pentagon `s p t u q`: after
        Catmull–Clark, two 3–5 pairs. Returns False when no edge allows it."""
        where = {}
        for k, face in enumerate(self.faces):
            for i, a in enumerate(face):
                where[(a, face[(i + 1) % len(face)])] = (k, i)
        candidates = [(a, b) for (a, b), (k, i) in where.items()
                      if a < b and len(self.faces[k]) == 4 and len(self.faces[where[(b, a)][0]]) == 4]
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


class QuadMesh:
    """An all-quad mesh of a closed oriented surface, with its half-edges."""

    def __init__(self, positions, faces):
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
        for (a, b) in self.where:
            if (b, a) not in self.where:
                raise ValueError(f"edge {a}-{b} is on a border")
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
        return len(self.positions) - len(self.where) // 2 + len(self.faces)


def catmull_clark(mesh):
    """One step of Catmull–Clark on a polygon mesh, in connectivity: a vertex
    per original vertex, edge and face; positions are only there to draw."""
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
    return QuadMesh(positions, faces)


def random_polygon_mesh(rng, kind):
    """One of the families: ('torus', w, h, moves), ('cube', moves),
    ('box', a, b, c, moves), ('hull', count), ('dual', count)."""
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
    else:
        raise ValueError(kind)
    for _ in range(moves):
        if not mesh.tri_pent_move(rng):
            break
    return mesh.check()
