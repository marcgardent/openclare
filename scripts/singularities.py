#!/usr/bin/env python3
"""What each irregular vertex of a quad mesh is for (DT-155).

    python3 scripts/singularities.py quads.obj
    python3 scripts/singularities.py quads.obj --source source.obj --adaptivity 0.5
    python3 scripts/singularities.py quads.obj --source source.obj --density weights.txt --json out.json

An irregular vertex of valence v carries (4 - v) quarter-turns of curvature
(on a border, where 3 is regular: 3 - v quarter-turns of turning). The sum
over a mesh is 2 pi chi. Each one is sorted into one of five classes, tried
in this order:

- volume: it stands alone, and the Gaussian curvature of the region it
  stands for (the vertices nearer to it than to any other lone irregular
  vertex, along the edges) is its charge, rounded to the nearest
  quarter-turn. A cube corner, the eight poles of a sphere. The curvature of
  the quad mesh itself is used: Gauss-Bonnet is then exact, the regions
  adding up to 2 pi chi (the source's is given beside it).
- boundary: the same, on a border: a corner of the border, whose turning is
  its charge.
- density: it is paired with an irregular vertex of opposite charge (a 3-5
  pair, a dislocation): each alone does not match its region, the two
  together carry no curvature. The pair is for density if the sizes asked
  for round it call for the lines it ends: the Burgers vector the size field
  asks for over the region of the pair has at least half of the pair's own
  along it. Sizes can also be followed with no singularity, by lines that
  bend or quads that stretch, so this says the pair agrees with the sizes,
  not that they need it.
- direction: it is where the field has to turn: within two quad edges of an
  end, a junction or a kink of the hard edges of the source (dihedral angle
  above --edge-angle, still that sharp half a quad edge away from the edge),
  or where the principal curvature directions, read round it, turn the same
  way as its charge (an umbilic turns them by a half-turn, so one pole is at
  best part of it). The summary gives how often regular vertices pass these
  tests: the rate they would pass by chance.
- noise: none of these. Not a proof of uselessness: what a mesh is for (to
  deform) and constraints of the whole (integer lengths, holonomy) are not
  looked at.

The Burgers vector of a pair is read by developing, quad by quad, a loop of
faces that holds the two vertices and no other irregular vertex. The loop
runs with the pair on its right, as in "Turns of a T-mesh" (a small loop
round a vertex of valence k on its right develops as x -> R^(4-k)(x - p) + p,
R the positive quarter-turn), and the vector is in the frame of the loop's
first quad: first component along its first edge, second along its last.

The sizes asked for are those of the engine's sizing (crates/openclare/src/
sizing.rs), read on the quad mesh: curvature with the share --adaptivity of
the law that evens out the error (0.5 for the organic and animation presets,
0 for hard-surface), times the painted density if one is given. Thin parts
are not looked at.

The text summary goes to the standard output; --json writes every irregular
vertex and the summary.
"""

import argparse
import json
import math
import sys
from collections import defaultdict

import numpy as np
from scipy.sparse import coo_matrix
from scipy.sparse.csgraph import dijkstra
from scipy.spatial import cKDTree

QUARTER = math.pi / 2
# The corners of a quad in its own frame, in the order of its vertices.
UNIT_CORNERS = [(0, 0), (1, 0), (1, 1), (0, 1)]
# The engine's sizing (sizing.rs, `Curvature::default`): quads round the
# tightest bend that counts as flat, how far a bend may move a size either
# way, the curvature not felt at the scale of a quad, and the smoothing.
QUADS_A_TURN = 12.0
SHAPE_RANGE = 3.0
FELT = 0.05
SMOOTHING_PASSES = 2
# How far from an end or a corner of the hard edges a vertex that turns the
# field may be, in quad edges; and the distance, in quad edges, at which a
# hard edge must still be hard to count.
FEATURE_REACH = 2.0
FEATURE_SCALE = 0.5
# Curvature directions are trusted where the principal curvatures differ by
# this much over a quad edge (the engine's `felt`).
ANISOTROPY = 0.05


def read_obj(path):
    """Vertex positions and faces (lists of vertex indices) of an OBJ file."""
    positions, faces = [], []
    with open(path) as lines:
        for line in lines:
            if line.startswith("v "):
                positions.append([float(x) for x in line.split()[1:4]])
            elif line.startswith("f "):
                faces.append([int(corner.split("/")[0]) - 1 for corner in line.split()[1:]])
    positions = np.array(positions, dtype=float)
    faces = [[corner if corner >= 0 else corner + len(positions) + 1 for corner in face] for face in faces]
    return positions, faces


def rotate(turns, vector):
    """`vector` turned by `turns` quarter-turns, counterclockwise."""
    x, y = vector
    for _ in range(turns % 4):
        x, y = -y, x
    return (x, y)


class QuadMesh:
    """A polygon mesh with its half-edges: half-edge (f, i) runs from corner i
    of face f to corner i + 1."""

    def __init__(self, positions, faces):
        self.positions = positions
        self.faces = faces
        self.vertex_count = len(positions)
        self.twin = {}
        directed = {}
        for f, face in enumerate(faces):
            for i in range(len(face)):
                directed[(face[i], face[(i + 1) % len(face)])] = (f, i)
        for (a, b), half_edge in directed.items():
            if (b, a) in directed:
                self.twin[half_edge] = directed[(b, a)]
        edges = {(min(a, b), max(a, b)) for (a, b) in directed}
        self.edges = np.array(sorted(edges), dtype=int).reshape(-1, 2)
        self.valence = np.bincount(self.edges.ravel(), minlength=self.vertex_count)
        self.on_border = np.zeros(self.vertex_count, dtype=bool)
        for (a, b) in directed:
            if (b, a) not in directed:
                self.on_border[a] = self.on_border[b] = True
        self.faces_of = [[] for _ in range(self.vertex_count)]
        for f, face in enumerate(faces):
            for v in face:
                self.faces_of[v].append(f)
        self.used = self.valence > 0
        self.quads_only = all(len(face) == 4 for face in faces)

        lengths = np.linalg.norm(positions[self.edges[:, 0]] - positions[self.edges[:, 1]], axis=1)
        self.edge_lengths = lengths
        total = np.bincount(self.edges.ravel(), weights=np.repeat(lengths, 2), minlength=self.vertex_count)
        self.local_edge = np.where(self.valence > 0, total / np.maximum(self.valence, 1), 1.0)
        self.graph = coo_matrix(
            (np.concatenate([lengths, lengths]), (np.concatenate([self.edges[:, 0], self.edges[:, 1]]),
                                                 np.concatenate([self.edges[:, 1], self.edges[:, 0]]))),
            shape=(self.vertex_count, self.vertex_count)).tocsr()
        ones = np.ones(len(self.edges))
        self.hops = coo_matrix(
            (np.concatenate([ones, ones]), (np.concatenate([self.edges[:, 0], self.edges[:, 1]]),
                                           np.concatenate([self.edges[:, 1], self.edges[:, 0]]))),
            shape=(self.vertex_count, self.vertex_count)).tocsr()
        self._angles()
        self._normals()

    def _angles(self):
        """Angle defects (2 pi minus the corner angles; on a border, pi minus
        them: the turning of the border), and the area each vertex stands
        for. A quad is cut along both diagonals and the two halves averaged:
        its corners then add up to 2 pi and Gauss-Bonnet is exact, the
        defects summing to 2 pi chi."""
        angle_sum = np.zeros(self.vertex_count)
        area = np.zeros(self.vertex_count)
        p = self.positions

        def triangle(a, b, c, weight):
            for corner, (u, w) in ((a, (b, c)), (b, (c, a)), (c, (a, b))):
                e1, e2 = p[u] - p[corner], p[w] - p[corner]
                cosine = np.dot(e1, e2) / max(np.linalg.norm(e1) * np.linalg.norm(e2), 1e-300)
                angle_sum[corner] += weight * math.acos(max(-1.0, min(1.0, cosine)))
            surface = 0.5 * np.linalg.norm(np.cross(p[b] - p[a], p[c] - p[a]))
            for corner in (a, b, c):
                area[corner] += weight * surface / 3.0

        for face in self.faces:
            if len(face) == 4:
                a, b, c, d = face
                for first, second in (((a, b, c), (a, c, d)), ((b, c, d), (b, d, a))):
                    triangle(*first, 0.5)
                    triangle(*second, 0.5)
            else:
                for k in range(1, len(face) - 1):
                    triangle(face[0], face[k], face[k + 1], 1.0)
        full = np.where(self.on_border, math.pi, 2 * math.pi)
        self.defect = np.where(self.used, full - angle_sum, 0.0)
        self.area = area

    def _normals(self):
        normals = np.zeros((self.vertex_count, 3))
        p = self.positions
        for face in self.faces:
            if len(face) == 4:
                normal = np.cross(p[face[2]] - p[face[0]], p[face[3]] - p[face[1]])
            else:
                normal = sum(np.cross(p[face[k]] - p[face[0]], p[face[k + 1]] - p[face[0]])
                             for k in range(1, len(face) - 1))
            for v in face:
                normals[v] += normal
        length = np.linalg.norm(normals, axis=1)
        self.normals = np.where(length[:, None] > 0, normals / np.maximum(length, 1e-300)[:, None], 0.0)

    def euler_characteristic(self):
        return int(np.count_nonzero(self.used)) - len(self.edges) + len(self.faces)

    def charge(self, v):
        """Quarter-turns an irregular vertex carries: 4 - valence inside, 3 -
        valence on a border."""
        return (3 if self.on_border[v] else 4) - int(self.valence[v])

    def irregular(self):
        return [v for v in range(self.vertex_count) if self.used[v] and self.charge(v) != 0]

    def ball(self, seeds, radius):
        """Vertices at most `radius` edges from any of `seeds`."""
        distance = dijkstra(self.hops, indices=list(seeds), min_only=True, limit=radius + 0.5)
        return np.flatnonzero(np.isfinite(distance))

    def faces_round(self, vertices):
        return {f for v in vertices for f in self.faces_of[v]}

    def border_walk(self, patch):
        """The faces of `patch` along its border, in order, each with the
        half-edge it is left by, and the vertices of the border; None unless
        the patch is a disc of quads. The walk keeps the patch on its left."""
        if any(len(self.faces[f]) != 4 for f in patch):
            return None
        border = [(f, i) for f in patch for i in range(4)
                  if self.twin.get((f, i), (None,))[0] not in patch]
        if not border:
            return None
        vertices = {v for f in patch for v in self.faces[f]}
        edges = {(min(a, b), max(a, b)) for f in patch for a, b in
                 zip(self.faces[f], self.faces[f][1:] + self.faces[f][:1])}
        if len(vertices) - len(edges) + len(patch) != 1:
            return None
        start = border[0]
        walk, loop = [], []
        half_edge = start
        for _ in range(4 * len(border) + 4 * len(patch)):
            f, i = half_edge
            loop.append(self.faces[f][i])
            # Turn round the head of this border half-edge, through the faces
            # of the patch, to the next border half-edge.
            f, i = f, (i + 1) % 4
            while True:
                other = self.twin.get((f, i))
                if other is None or other[0] not in patch:
                    break
                walk.append((f, i))
                f, i = other[0], (other[1] + 1) % 4
            half_edge = (f, i)
            if half_edge == start:
                break
        else:
            return None
        if len(loop) != len(border):
            return None
        return walk, loop

    def develop(self, walk):
        """Develops the faces of a closed walk quad by quad. Returns the motion
        that takes the first face onto itself after the walk, as (quarter
        turns, translation), in the frame of the first face."""
        turns, translation = 0, (0, 0)
        for f, i in walk:
            g, j = self.twin[(f, i)]
            # Corner i + 1 of f is corner j of g, corner i of f corner j + 1 of g.
            head = rotate(turns, UNIT_CORNERS[(i + 1) % 4])
            head = (head[0] + translation[0], head[1] + translation[1])
            turns = (turns + i + 2 - j) % 4
            corner = rotate(turns, UNIT_CORNERS[j])
            translation = (head[0] - corner[0], head[1] - corner[1])
        return turns, translation

    def frame(self, f):
        """Unit vectors along the first and the last edge of face f, from its
        first corner: the frame lattice vectors are read in."""
        face = self.faces[f]
        p = self.positions
        first = p[face[1]] - p[face[0]]
        last = p[face[3]] - p[face[0]]
        normal = np.cross(first, last)
        normal /= max(np.linalg.norm(normal), 1e-300)
        first /= max(np.linalg.norm(first), 1e-300)
        return first, np.cross(normal, first), normal

    def curvature_tensors(self):
        """Principal curvatures and the first principal direction at each
        vertex, from the normal curvature towards every other corner of its
        faces (a least-squares second fundamental form)."""
        p, n = self.positions, self.normals
        k1 = np.zeros(self.vertex_count)
        k2 = np.zeros(self.vertex_count)
        direction = np.zeros((self.vertex_count, 3))
        for v in range(self.vertex_count):
            others = sorted({u for f in self.faces_of[v] for u in self.faces[f]} - {v})
            if len(others) < 3:
                continue
            normal = n[v]
            if not np.all(np.isfinite(normal)) or np.linalg.norm(normal) < 0.5:
                continue
            helper = np.array([1.0, 0, 0]) if abs(normal[0]) < 0.9 else np.array([0, 1.0, 0])
            t1 = np.cross(normal, helper)
            t1 /= np.linalg.norm(t1)
            t2 = np.cross(normal, t1)
            d = p[others] - p[v]
            squared = np.maximum((d * d).sum(axis=1), 1e-300)
            bend = -2 * (d @ normal) / squared
            x, y = d @ t1, d @ t2
            length = np.maximum(np.hypot(x, y), 1e-300)
            x, y = x / length, y / length
            rows = np.stack([x * x, 2 * x * y, y * y], axis=1)
            (a, b, c), *_ = np.linalg.lstsq(rows, bend, rcond=None)
            values, vectors = np.linalg.eigh(np.array([[a, b], [b, c]]))
            k2[v], k1[v] = values
            direction[v] = vectors[0, 1] * t1 + vectors[1, 1] * t2
        return k1, k2, direction

    def cross_index(self, loop, directions):
        """Index of a cross field round a loop of vertices that goes
        counterclockwise round what it holds: its quarter-turns relative to a
        frame carried along the loop, plus the turn of that frame (the
        curvature held)."""
        p, n = self.positions, self.normals

        def carry(vector, source, target):
            axis = np.cross(n[source], n[target])
            sine, cosine = np.linalg.norm(axis), float(np.dot(n[source], n[target]))
            if sine < 1e-12:
                return vector
            axis /= sine
            angle = math.atan2(sine, cosine)
            return (vector * math.cos(angle) + np.cross(axis, vector) * math.sin(angle)
                    + axis * np.dot(axis, vector) * (1 - math.cos(angle)))

        def angle_between(a, b, normal):
            return math.atan2(np.dot(np.cross(a, b), normal), np.dot(a, b))

        turned = 0.0
        for k in range(len(loop)):
            a, b = loop[k], loop[(k + 1) % len(loop)]
            carried = carry(directions[a], a, b)
            step = angle_between(carried, directions[b], n[b])
            turned += step - QUARTER * round(step / QUARTER)
        frame = directions[loop[0]]
        for k in range(len(loop)):
            frame = carry(frame, loop[k], loop[(k + 1) % len(loop)])
        holonomy = angle_between(directions[loop[0]], frame, n[loop[0]])
        return turned, holonomy


def demanded_log_size(mesh, adaptivity, paint):
    """The logarithm of the edge length the engine's sizing asks for at each
    vertex, up to a constant: the bend, as far as `adaptivity` follows it,
    and the paint."""
    asked = np.zeros(mesh.vertex_count)
    if adaptivity > 0:
        k1, k2, _ = mesh.curvature_tensors()
        level = math.sqrt(mesh.area.sum() / max(len(mesh.faces), 1))
        strongest = np.maximum(np.abs(k1), np.abs(k2)) + FELT / level
        bend = np.log(2 * math.pi / (strongest * QUADS_A_TURN))
        flat = np.average(bend[mesh.used], weights=mesh.area[mesh.used])
        asked = np.clip(0.5 * adaptivity * (bend - flat), -math.log(SHAPE_RANGE), math.log(SHAPE_RANGE))
    if paint is not None:
        asked = asked + paint
    neighbours = [[] for _ in range(mesh.vertex_count)]
    for a, b in mesh.edges:
        neighbours[a].append(b)
        neighbours[b].append(a)
    for _ in range(SMOOTHING_PASSES):
        previous = asked.copy()
        for v in range(mesh.vertex_count):
            around = [v] + neighbours[v]
            weights = mesh.area[around]
            if weights.sum() > 0:
                asked[v] = np.dot(previous[around], weights) / weights.sum()
    return asked


def gradients(mesh, values):
    """Gradient of a field given at the vertices, per vertex: the average,
    weighted by area, of its gradient on the triangles of the faces round."""
    p = mesh.positions
    total = np.zeros((mesh.vertex_count, 3))
    weight = np.zeros(mesh.vertex_count)
    for face in mesh.faces:
        triangles = ([(face[0], face[1], face[2]), (face[0], face[2], face[3]),
                      (face[1], face[2], face[3]), (face[1], face[3], face[0])]
                     if len(face) == 4 else
                     [(face[0], face[k], face[k + 1]) for k in range(1, len(face) - 1)])
        for a, b, c in triangles:
            normal = np.cross(p[b] - p[a], p[c] - p[a])
            double_area = np.linalg.norm(normal)
            if double_area < 1e-300:
                continue
            normal /= double_area
            # The gradient of the linear interpolant on the triangle.
            gradient = (values[a] * np.cross(normal, p[c] - p[b]) + values[b] * np.cross(normal, p[a] - p[c])
                        + values[c] * np.cross(normal, p[b] - p[a])) / double_area
            for v in (a, b, c):
                total[v] += gradient * double_area
                weight[v] += double_area
    return total / np.maximum(weight, 1e-300)[:, None]


class Source:
    """The triangle mesh the quads were made from: its hard edges and the
    curvature on it."""

    def __init__(self, path, edge_angle):
        positions, faces = read_obj(path)
        triangles = np.array([face[:3] for face in faces if len(face) == 3]
                             + [[face[0], face[k], face[k + 1]] for face in faces if len(face) > 3
                                for k in range(1, len(face) - 1)], dtype=int)
        self.positions, self.triangles = positions, triangles
        normals = np.cross(positions[triangles[:, 1]] - positions[triangles[:, 0]],
                           positions[triangles[:, 2]] - positions[triangles[:, 0]])
        areas = 0.5 * np.linalg.norm(normals, axis=1)
        normals /= np.maximum(2 * areas, 1e-300)[:, None]
        # Angle defects of the source, its Gaussian curvature.
        angle_sum = np.zeros(len(positions))
        for k in range(3):
            a, b, c = triangles[:, k], triangles[:, (k + 1) % 3], triangles[:, (k + 2) % 3]
            e1, e2 = positions[b] - positions[a], positions[c] - positions[a]
            cosine = (e1 * e2).sum(axis=1) / np.maximum(np.linalg.norm(e1, axis=1) * np.linalg.norm(e2, axis=1),
                                                         1e-300)
            angle_sum += np.bincount(a, weights=np.arccos(np.clip(cosine, -1, 1)), minlength=len(positions))
        # Edges, with their faces: an edge of one face is on a border.
        directed = np.concatenate([triangles[:, [0, 1]], triangles[:, [1, 2]], triangles[:, [2, 0]]])
        owner = np.tile(np.arange(len(triangles)), 3)
        key = np.sort(directed, axis=1)
        order = np.lexsort((key[:, 1], key[:, 0]))
        key, owner = key[order], owner[order]
        same = np.all(key[1:] == key[:-1], axis=1)
        starts = np.flatnonzero(np.concatenate([[True], ~same]))
        counts = np.diff(np.concatenate([starts, [len(key)]]))
        on_border = np.zeros(len(positions), dtype=bool)
        hard = []
        limit = math.cos(math.radians(edge_angle)) if edge_angle is not None else None
        for start, count in zip(starts, counts):
            if count == 1:
                on_border[key[start]] = True
                hard.append((*key[start], owner[start], -1))
            elif count == 2 and limit is not None:
                if np.dot(normals[owner[start]], normals[owner[start + 1]]) < limit:
                    hard.append((*key[start], owner[start], owner[start + 1]))
        self.edge_angle = edge_angle
        self.face_normals, self.face_areas = normals, areas
        self.defect = np.where(on_border, math.pi, 2 * math.pi) - angle_sum
        self.vertex_normals = np.zeros((len(positions), 3))
        for k in range(3):
            np.add.at(self.vertex_normals, triangles[:, k], normals * areas[:, None])
        self.vertex_normals /= np.maximum(np.linalg.norm(self.vertex_normals, axis=1), 1e-300)[:, None]
        hard = np.array(hard, dtype=int).reshape(-1, 4)
        self.hard_edges, self.hard_edge_faces = hard[:, :2], hard[:, 2:]

    def feature_points(self, scale):
        """Where the hard edges end, meet or turn by more than 30 degrees:
        the places a field aligned with them may have to turn. A hard edge
        counts if it is still hard at `scale` (half a quad edge): the mean
        normals of the source on its two sides, within that distance of its
        middle, are still that far apart. The grain of a sculpt is not."""
        self.kept_hard_edges = 0
        if not len(self.hard_edges):
            return np.zeros((0, 3))
        p, triangles = self.positions, self.triangles
        centroids = p[triangles].mean(axis=1)
        tree = cKDTree(centroids)
        limit = math.cos(math.radians(self.edge_angle)) if self.edge_angle is not None else 1.0
        kept = []
        for (a, b), (f, g) in zip(self.hard_edges, self.hard_edge_faces):
            if g < 0:
                # A border is followed whatever its angle.
                kept.append((a, b))
                continue
            middle = 0.5 * (p[a] + p[b])
            near = np.array(sorted(set(tree.query_ball_point(middle, scale)) | {f, g}), dtype=int)
            normals = self.face_normals[near]
            first, second = self.face_normals[f], self.face_normals[g]
            across = np.cross(p[b] - p[a], first + second)
            across /= max(np.linalg.norm(across), 1e-300)
            # The faces on the same sheet: not those of the other side of a
            # thin wall.
            same_sheet = normals @ (first + second) > 0
            side = (centroids[near] - middle) @ across
            weights = self.face_areas[near][:, None] * normals
            one = weights[same_sheet & (side > 0)].sum(axis=0)
            other = weights[same_sheet & (side <= 0)].sum(axis=0)
            if np.linalg.norm(one) == 0 or np.linalg.norm(other) == 0:
                continue
            cosine = np.dot(one, other) / (np.linalg.norm(one) * np.linalg.norm(other))
            if cosine < limit:
                kept.append((a, b))
        self.kept_hard_edges = len(kept)
        around = defaultdict(list)
        for a, b in kept:
            around[a].append(b)
            around[b].append(a)
        points = []
        for v, others in around.items():
            if len(others) != 2:
                points.append(v)
                continue
            first = self.positions[others[0]] - self.positions[v]
            second = self.positions[others[1]] - self.positions[v]
            cosine = np.dot(first, second) / max(np.linalg.norm(first) * np.linalg.norm(second), 1e-300)
            # Straight on, the two edges are opposite: cosine -1.
            if cosine > -math.cos(math.radians(30)):
                points.append(v)
        return self.positions[points] if points else np.zeros((0, 3))


def voronoi(mesh, sources):
    """The nearest of `sources` to each vertex along the edges (-1 where none
    is reached)."""
    if not sources:
        return np.full(mesh.vertex_count, -1)
    _, _, nearest = dijkstra(mesh.graph, indices=sources, min_only=True, return_predecessors=True)
    return np.where(nearest >= 0, nearest, -1)


def region_sums(labels, values, count):
    keep = labels >= 0
    return np.bincount(labels[keep], weights=values[keep], minlength=count)


def classify(mesh, source=None, adaptivity=0.5, paint=None):
    """Every irregular vertex of `mesh`, with what it is for, and a summary."""
    singular = mesh.irregular()
    charge = {v: mesh.charge(v) for v in singular}
    singular_set = set(singular)
    found = {v: {"vertex": int(v)} for v in singular}

    # Pairs: opposite charges whose regions touch, that do not each match
    # their charge but carry none together. Paired vertices leave the
    # sources, the regions of the others are read again, and so on.
    paired = {}
    first_regions = None
    while True:
        lone = [v for v in singular if v not in paired]
        labels = voronoi(mesh, lone)
        curvature = region_sums(labels, mesh.defect, mesh.vertex_count)
        if first_regions is None:
            first_regions = (labels, curvature)
        matched = {v: round(curvature[v] / QUARTER) == charge[v] for v in lone}
        touching = set()
        for a, b in mesh.edges:
            la, lb = labels[a], labels[b]
            if la != lb and la >= 0 and lb >= 0:
                touching.add((min(la, lb), max(la, lb)))
        candidates = []
        for a, b in touching:
            if charge[a] + charge[b] != 0 or (matched[a] and matched[b]):
                continue
            if round((curvature[a] + curvature[b]) / QUARTER) != 0:
                continue
            candidates.append((a, b))
        if not candidates:
            break
        sources = sorted({v for pair in candidates for v in pair})
        distance = dijkstra(mesh.graph, indices=sources)
        row = {v: k for k, v in enumerate(sources)}
        candidates.sort(key=lambda pair: distance[row[pair[0]], pair[1]])
        new = 0
        for a, b in candidates:
            if a in paired or b in paired:
                continue
            paired[a], paired[b] = b, a
            for v in (a, b):
                found[v]["pair_region_curvature"] = round(float(curvature[a] + curvature[b]), 6)
            new += 1
        if not new:
            break
        # The regions the pair was found with are where its Burgers vector
        # is asked for.
        for a, b in candidates:
            if paired.get(a) == b and "pair_labels" not in found[a]:
                found[a]["pair_labels"] = found[b]["pair_labels"] = labels.copy()

    lone = [v for v in singular if v not in paired]
    labels = voronoi(mesh, lone)
    curvature = region_sums(labels, mesh.defect, mesh.vertex_count)
    size = np.bincount(labels[labels >= 0], minlength=mesh.vertex_count)
    _, first_curvature = first_regions
    if source is not None:
        tree = cKDTree(mesh.positions[mesh.used])
        used_index = np.flatnonzero(mesh.used)
        _, nearest = tree.query(source.positions, k=min(4, len(used_index)))
        nearest = np.atleast_2d(nearest)
        if nearest.shape[0] != len(source.positions):
            nearest = nearest.T
        chosen = used_index[nearest[:, 0]]
        # A thin wall: take the nearest quad vertex that faces the same way.
        for column in range(nearest.shape[1] - 1, -1, -1):
            candidate = used_index[nearest[:, column]]
            agree = (mesh.normals[candidate] * source.vertex_normals).sum(axis=1) > 0
            chosen = np.where(agree, candidate, chosen)
        source_labels = labels[chosen]
        source_curvature = region_sums(source_labels, source.defect, mesh.vertex_count)
        level = math.sqrt(mesh.area.sum() / max(len(mesh.faces), 1))
        features = source.feature_points(FEATURE_SCALE * level)
        feature_tree = cKDTree(features) if len(features) else None

    # Sizes asked for, and the Burgers vector they ask for per vertex: in
    # lattice units, (area / size) n x grad(log size) for a loop with the
    # region on its right.
    demand = demanded_log_size(mesh, adaptivity, paint)
    gradient = gradients(mesh, demand)
    asked_burgers = (mesh.area / mesh.local_edge)[:, None] * np.cross(mesh.normals, gradient)

    k1, k2, principal = mesh.curvature_tensors()
    edge_direction = np.zeros((mesh.vertex_count, 3))
    for a, b in mesh.edges:
        if not edge_direction[a].any():
            edge_direction[a] = mesh.positions[b] - mesh.positions[a]
        if not edge_direction[b].any():
            edge_direction[b] = mesh.positions[a] - mesh.positions[b]
    edge_direction -= (edge_direction * mesh.normals).sum(axis=1)[:, None] * mesh.normals
    edge_direction /= np.maximum(np.linalg.norm(edge_direction, axis=1), 1e-300)[:, None]

    def index_round(v, directions):
        """Quarter-turns a cross field makes round v, read on the border of
        the faces round its neighbours, or of its own faces where that meets
        another irregular vertex; None on a border, or where no such patch is
        a disc clear of other irregular vertices (a member of a pair whose
        partner is next to it)."""
        if mesh.on_border[v]:
            return None, None
        for patch in (mesh.faces_round(mesh.ball([v], 1)), mesh.faces_round([v])):
            walked = mesh.border_walk(patch)
            if walked is None:
                continue
            _, loop = walked
            touched = {u for f in patch for u in mesh.faces[f]}
            if any(u in singular_set for u in touched if u != v):
                continue
            held = [u for u in touched if u not in set(loop)]
            turned, holonomy = mesh.cross_index(loop, directions)
            curvature_held = float(mesh.defect[held].sum())
            holonomy += 2 * math.pi * round((curvature_held - holonomy) / (2 * math.pi))
            return round((turned + holonomy) / QUARTER), loop
        return None, None

    burgers_failures = 0
    for v in singular:
        record = found[v]
        h = float(mesh.local_edge[v])
        record.update({
            "position": [round(float(x), 6) for x in mesh.positions[v]],
            "valence": int(mesh.valence[v]),
            "border": bool(mesh.on_border[v]),
            "charge": round(charge[v] * QUARTER, 6),
            "quarter_turns": charge[v],
            "edge": round(h, 6),
        })
        if v in paired:
            record["partner"] = int(paired[v])
        else:
            record["region_curvature"] = round(float(curvature[v]), 6)
            record["region_quarter_turns"] = int(round(curvature[v] / QUARTER))
            record["region_vertices"] = int(size[v])
            # How much the quads change size over the region: on a sphere
            # the curvature a pole does not hold at its vertex is taken up
            # by the size (Bunin 2008: the Laplacian of the log of the size
            # is the curvature, less the charges).
            record["region_size_spread"] = round(float(np.std(np.log(mesh.local_edge[labels == v]))), 4)
            if source is not None:
                record["source_region_curvature"] = round(float(source_curvature[v]), 6)
        record["first_region_curvature"] = round(float(first_curvature[v]), 6)
        quad_index, _ = index_round(v, edge_direction)
        record["field_quarter_turns"] = quad_index
        reliable = None
        principal_index = None
        if not mesh.on_border[v]:
            principal_index, loop = index_round(v, principal)
            if loop is not None:
                reliable = bool(np.all(np.abs(k1[loop] - k2[loop]) * mesh.local_edge[loop] >= ANISOTROPY))
        record["principal_quarter_turns"] = principal_index
        record["principal_reliable"] = reliable
        if source is not None and feature_tree is not None:
            distance, _ = feature_tree.query(mesh.positions[v])
            record["feature_distance"] = round(float(distance) / h, 3)
        else:
            record["feature_distance"] = None

    # The Burgers vector of each pair, and what the sizes ask for over the
    # regions the pair was found with.
    done = set()
    for v in singular:
        if v not in paired or v in done:
            continue
        u = paired[v]
        done.update((u, v))
        a, b = (v, u) if charge[v] > 0 else (u, v)
        result = pair_burgers(mesh, a, b, singular_set)
        hops = dijkstra(mesh.hops, indices=[a], limit=64)[0][b]
        pair_labels = found[a].pop("pair_labels", None)
        found[b].pop("pair_labels", None)
        for w in (a, b):
            found[w]["pair_hops"] = int(hops) if np.isfinite(hops) else None
        if result is None:
            burgers_failures += 1
            for w in (a, b):
                found[w]["burgers"] = None
            continue
        burgers, face = result
        first, second, _ = mesh.frame(face)
        in_space = burgers[0] * first + burgers[1] * second
        region = np.isin(pair_labels, [a, b]) if pair_labels is not None else np.zeros(mesh.vertex_count, bool)
        asked = asked_burgers[region].sum(axis=0)
        asked_lattice = [float(np.dot(asked, first)), float(np.dot(asked, second))]
        norm_squared = burgers[0] ** 2 + burgers[1] ** 2
        agreement = float(np.dot(asked, in_space)) / norm_squared if norm_squared else 0.0
        for w in (a, b):
            found[w].update({
                "burgers": [int(burgers[0]), int(burgers[1])],
                "burgers_length": round(math.sqrt(norm_squared), 6),
                "burgers_in_space": [round(float(x), 6) for x in in_space],
                "asked_burgers": [round(x, 4) for x in asked_lattice],
                "asked_share": round(agreement, 4),
            })

    # The classes.
    for v in singular:
        record = found[v]
        record.pop("pair_labels", None)
        near_feature = record["feature_distance"] is not None and record["feature_distance"] <= FEATURE_REACH
        # An umbilic turns the principal directions by a half-turn, two
        # quarter-turns: a pole of the same sign next to one helps turn the
        # field round it.
        follows_curvature = (record["principal_reliable"] is True and record["principal_quarter_turns"]
                             and record["principal_quarter_turns"] * charge[v] > 0)
        record["near_feature"] = bool(near_feature)
        record["follows_curvature"] = bool(follows_curvature)
        if v not in paired and record["region_quarter_turns"] == charge[v]:
            kind = "boundary" if mesh.on_border[v] else "volume"
        elif v in paired and record.get("burgers") is not None and record["asked_share"] >= 0.5:
            kind = "density"
        elif near_feature or follows_curvature:
            kind = "direction"
        else:
            kind = "noise"
        record["class"] = kind

    summary = summarise(mesh, singular, found, paired, burgers_failures)
    # How often the tests of direction would pass by chance: on regular
    # vertices (curvature: every tenth of them).
    regular = np.flatnonzero(mesh.used & ~mesh.on_border & (mesh.valence == 4))
    turning = 0
    sample = regular[::10]
    for v in sample:
        principal_index, loop = index_round(v, principal)
        if loop is not None and principal_index and np.all(
                np.abs(k1[loop] - k2[loop]) * mesh.local_edge[loop] >= ANISOTROPY):
            turning += 1
    summary["share_principal_turning"] = round(turning / max(len(sample), 1), 4)
    if source is not None:
        summary["source_total_curvature"] = round(float(source.defect.sum()), 6)
        summary["hard_edges"] = int(len(source.hard_edges))
        summary["hard_edges_kept"] = int(getattr(source, "kept_hard_edges", 0))
        summary["feature_points"] = int(len(features))
        if len(features):
            regular = np.flatnonzero(mesh.used & (mesh.valence == np.where(mesh.on_border, 3, 4)))
            distance, _ = feature_tree.query(mesh.positions[regular])
            summary["share_near_feature"] = round(float(np.mean(distance / mesh.local_edge[regular]
                                                               <= FEATURE_REACH)), 4)
    return [found[v] for v in singular], summary


def pair_burgers(mesh, a, b, singular):
    """The Burgers vector of the pair (a, b), and the face its frame is that
    of: the translation of the development of a loop of faces round the
    pair, with the pair on its right. None if no disc of quads round them
    holds no other irregular vertex."""
    _, predecessors = dijkstra(mesh.hops, indices=[a], return_predecessors=True, limit=64)
    path, v = [b], b
    while v != a:
        v = predecessors[0][v]
        if v < 0:
            return None
        path.append(v)
    for radius in range(0, 4):
        patch = mesh.faces_round(mesh.ball(path, radius) if radius else path)
        walked = mesh.border_walk(patch)
        if walked is None:
            continue
        walk, loop = walked
        inside = {u for f in patch for u in mesh.faces[f]} - set(loop)
        if a not in inside or b not in inside:
            continue
        if any(u in singular for u in inside - {a, b}):
            return None
        # The walk keeps the patch on its left; the other way round, on its
        # right.
        reverse = [mesh.twin[half_edge] for half_edge in reversed(walk)]
        turns, translation = mesh.develop(reverse)
        if turns != 0:
            return None
        return translation, reverse[0][0]
    return None


def summarise(mesh, singular, found, paired, burgers_failures):
    classes = ["volume", "boundary", "density", "direction", "noise"]
    counts = {kind: sum(1 for v in singular if found[v]["class"] == kind) for kind in classes}
    inside = [v for v in singular if not mesh.on_border[v]]
    total = sum(found[v]["charge"] for v in singular)
    chi = mesh.euler_characteristic()
    lone_noise = sum(1 for v in singular if found[v]["class"] == "noise" and v not in paired)
    misplaced = sum(1 for v in singular if v not in paired and found[v]["class"] not in ("volume", "boundary"))
    return {
        "faces": len(mesh.faces),
        "quads_only": mesh.quads_only,
        "euler_characteristic": chi,
        "irregular": len(singular),
        "irregular_inside": len(inside),
        "counts": counts,
        "noise_share": round(counts["noise"] / len(singular), 4) if singular else 0.0,
        "pairs": len(paired) // 2,
        "noise_alone": lone_noise,
        "noise_in_pairs": counts["noise"] - lone_noise,
        "alone_unmatched": misplaced,
        "burgers_failures": burgers_failures,
        "total_charge": round(total, 6),
        "total_curvature": round(float(mesh.defect.sum()), 6),
        "two_pi_chi": round(2 * math.pi * chi, 6),
    }


def read_paint(path, source, mesh, contrast):
    """The painted density as what it does to the log of the size at each
    quad vertex: the weight of the nearest source vertex, as the engine
    reads it (sizing.rs)."""
    weights = np.loadtxt(path, dtype=float).ravel()
    if len(weights) != len(source.positions):
        sys.exit(f"{len(weights)} weights for {len(source.positions)} source vertices")
    _, nearest = cKDTree(source.positions).query(mesh.positions)
    return -0.5 * (weights[nearest] - weights.min()) * math.log(contrast)


def text(summary):
    counts = summary["counts"]
    lines = [
        f"{summary['irregular']} irregular vertices ({summary['irregular_inside']} inside), "
        f"{summary['pairs']} pairs, {summary['faces']} faces, chi {summary['euler_characteristic']}",
        "  " + ", ".join(f"{kind} {count}" for kind, count in counts.items())
        + f"; noise share {summary['noise_share']:.1%} ({summary['noise_alone']} alone, "
          f"{summary['noise_in_pairs']} in pairs)",
        f"  total charge {summary['total_charge']:.4f}, total curvature {summary['total_curvature']:.4f}, "
        f"2 pi chi {summary['two_pi_chi']:.4f}",
    ]
    if summary["burgers_failures"]:
        lines.append(f"  {summary['burgers_failures']} pairs whose Burgers vector could not be read")
    lines.append(f"  {summary['share_principal_turning']:.1%} of regular vertices have principal directions "
                 "that turn round them")
    if "share_near_feature" in summary:
        lines.append(f"  {summary['feature_points']} feature points; "
                     f"{summary['share_near_feature']:.1%} of regular vertices are near one")
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("quads", help="the quad mesh (OBJ)")
    parser.add_argument("--source", help="the triangle mesh it was made from (OBJ): hard edges, curvature")
    parser.add_argument("--adaptivity", type=float, default=0.5,
                        help="how far sizes follow the bends: 0.5 organic and animation (default), 0 hard-surface")
    parser.add_argument("--edge-angle", default="30",
                        help="dihedral angle above which a source edge is hard, in degrees, or none (default 30)")
    parser.add_argument("--density", help="painted density, one weight per source vertex (needs --source)")
    parser.add_argument("--density-contrast", type=float, default=4.0)
    parser.add_argument("--json", help="write every irregular vertex and the summary there ('-': standard output)")
    options = parser.parse_args()

    positions, faces = read_obj(options.quads)
    mesh = QuadMesh(positions, faces)
    edge_angle = None if options.edge_angle == "none" else float(options.edge_angle)
    source = Source(options.source, edge_angle) if options.source else None
    paint = None
    if options.density:
        if source is None:
            sys.exit("--density needs --source")
        paint = read_paint(options.density, source, mesh, options.density_contrast)
    vertices, summary = classify(mesh, source, options.adaptivity, paint)
    if options.json == "-":
        print(json.dumps({"summary": summary, "irregular": vertices}, indent=1))
        return
    if options.json:
        with open(options.json, "w") as out:
            json.dump({"summary": summary, "irregular": vertices}, out, indent=1)
    print(text(summary))


if __name__ == "__main__":
    main()
