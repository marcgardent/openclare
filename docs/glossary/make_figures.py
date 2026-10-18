"""Draw the pictures of docs/GLOSSARY.md.

Run from anywhere:  python3 docs/glossary/make_figures.py [name ...]
With no name, every picture is drawn again. Needs numpy, matplotlib and Pillow.

The pictures that show a quad mesh with poles are computed, not drawn by hand: a
regular grid is changed by edge rotations (each one creates two pairs of a
valence-3 and a valence-5 vertex), the pairs are moved apart by further rotations,
and the mesh is laid out by Laplacian smoothing. The shifts printed on them are
measured by walking the edge loops of that mesh.
"""

import io
import os
import sys
from collections import defaultdict

import numpy as np
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon, FancyArrowPatch, Circle, Rectangle
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = "/home/marcgardent/retopology/scratch"

# One palette for every picture.
INK = "#1f2933"
MUTED = "#5b6470"
GRID = "#9aa3ad"
LIGHT = "#eef2f7"
BLUE = "#2563b0"  # rings, closed loops
BLUE_LIGHT = "#bcd3ee"
RED = "#d23c32"  # helices, spirals
RED_LIGHT = "#f4c2bd"
ORANGE = "#e8890c"  # valence 3
PURPLE = "#7c3aad"  # valence 5
GREEN = "#14934a"  # what is measured: shift, staircase, Burgers vector
PATCH_COLOURS = ["#dbe8f6", "#e2f1de", "#fbf0cf", "#f6dde3", "#e7e0f4", "#d8efee"]

plt.rcParams.update(
    {
        "font.family": "DejaVu Sans",
        "font.size": 15,
        "axes.edgecolor": INK,
        "text.color": INK,
        "figure.facecolor": "white",
        "savefig.facecolor": "white",
    }
)


def save(fig, name, colours=256):
    """Save as an opaque PNG with a reduced palette, to keep the repository light."""
    buffer = io.BytesIO()
    fig.savefig(buffer, format="png", dpi=100, facecolor="white")
    plt.close(fig)
    buffer.seek(0)
    image = Image.open(buffer).convert("RGB")
    # Pure white stays pure white: quantize the rest, then put white back.
    pixels = np.asarray(image).copy()
    white = (pixels >= 252).all(axis=2)
    pixels[white] = 255
    image = Image.fromarray(pixels).quantize(colors=colours, method=Image.Quantize.MEDIANCUT,
                                              dither=Image.Dither.NONE)
    palette = image.getpalette()
    index = np.asarray(image)
    if white.any():
        slot = int(np.bincount(index[white]).argmax())
        palette[3 * slot : 3 * slot + 3] = [255, 255, 255]
        image.putpalette(palette)
    path = os.path.join(HERE, name)
    image.save(path, optimize=True)
    print(f"{name}: {image.size[0]}x{image.size[1]}, {os.path.getsize(path) // 1024} KB")


def bare(ax):
    ax.set_aspect("equal")
    ax.axis("off")


def pole_marker(ax, point, valence, size=15, zorder=10):
    if valence == 3:
        ax.plot(*point, marker="^", ms=size, color=ORANGE, mec="white", mew=1.5, zorder=zorder)
    elif valence == 5:
        ax.plot(*point, marker="p", ms=size * 1.05, color=PURPLE, mec="white", mew=1.5, zorder=zorder)
    else:
        ax.plot(*point, marker="o", ms=size * 0.7, color=INK, mec="white", mew=1.5, zorder=zorder)


def arrow(ax, start, end, colour=INK, width=1.8, style="-|>", rad=0.0, scale=16, zorder=8, ls="-"):
    ax.add_patch(
        FancyArrowPatch(
            start,
            end,
            arrowstyle=style,
            mutation_scale=scale,
            color=colour,
            lw=width,
            connectionstyle=f"arc3,rad={rad}",
            zorder=zorder,
            linestyle=ls,
        )
    )


def label(ax, xy, text, colour=INK, size=15, ha="center", va="center", box=True, **kw):
    bbox = dict(boxstyle="round,pad=0.25", fc="white", ec="none", alpha=0.9) if box else None
    ax.text(*xy, text, color=colour, fontsize=size, ha=ha, va=va, bbox=bbox, zorder=20, **kw)


# ----------------------------------------------------------------------------
# A small quad mesh toolkit
# ----------------------------------------------------------------------------


def grid(width, height, periodic=False):
    """Quad grid of width x height quads; periodic glues the left and right sides."""
    columns = width if periodic else width + 1

    def vertex(i, j):
        return j * columns + (i % width if periodic else i)

    faces = []
    for j in range(height):
        for i in range(width):
            faces.append([vertex(i, j), vertex(i + 1, j), vertex(i + 1, j + 1), vertex(i, j + 1)])
    positions = np.array([[i, j] for j in range(height + 1) for i in range(columns)], float)
    return faces, positions


def edges_of(faces):
    edges = defaultdict(list)
    for index, face in enumerate(faces):
        for k in range(4):
            edges[frozenset((face[k], face[(k + 1) % 4]))].append(index)
    return edges


def valences(faces, count):
    result = np.zeros(count, int)
    for edge in edges_of(faces):
        for vertex in edge:
            result[vertex] += 1
    return result


def rotate_edge(faces, first, second, choice, edges=None, with_diagonals=False):
    """Rotate the edge shared by two quads: their union, a hexagon, is split again
    along one of its two other diagonals. Returns None if that is not possible."""
    a, b = faces[first], faces[second]
    for k in range(4):
        p0, p1 = a[k], a[(k + 1) % 4]
        if p0 in b and p1 in b and b[(b.index(p1) + 1) % 4] == p0:
            break
    else:
        return None
    ib = b.index(p1)
    hexagon = [p1, a[(k + 2) % 4], a[(k + 3) % 4], p0, b[(ib + 2) % 4], b[(ib + 3) % 4]]
    if len(set(hexagon)) != 6:
        return None
    h = hexagon
    if choice == 0:
        new = [h[1], h[2], h[3], h[4]], [h[4], h[5], h[0], h[1]]
        diagonal = (h[1], h[4])
    else:
        new = [h[2], h[3], h[4], h[5]], [h[5], h[0], h[1], h[2]]
        diagonal = (h[2], h[5])
    if frozenset(diagonal) in (edges_of(faces) if edges is None else edges):
        return None
    result = [list(face) for face in faces]
    result[first], result[second] = new
    if with_diagonals:
        return result, (p0, p1), diagonal
    return result


def rotation_system(faces):
    """next_around[v][a] = b: around v, b follows a counter-clockwise."""
    next_around = defaultdict(dict)
    for face in faces:
        for k in range(4):
            next_around[face[k]][face[(k + 1) % 4]] = face[(k - 1) % 4]
    return next_around


def go_straight(next_around, u, v):
    """From u through the regular vertex v, the next vertex straight on."""
    around = next_around[v]
    if len(around) != 4 or u not in around:
        return None
    return around[around[u]]


def turn_left(next_around, u, v):
    around = next_around[v]
    return around[around[around[u]]]


def lay_out(faces, positions, fixed, period=None, iterations=4000):
    """Laplacian smoothing with fixed vertices; period wraps x for a tube."""
    pairs = np.array(sorted({tuple(sorted(edge)) for edge in edges_of(faces)}))
    count = len(positions)
    degree = np.bincount(pairs.ravel(), minlength=count).astype(float)
    free = np.ones(count, bool)
    free[list(fixed)] = False
    result = positions.copy()
    for _ in range(iterations):
        difference = result[pairs[:, 1]] - result[pairs[:, 0]]
        if period:
            difference[:, 0] -= period * np.round(difference[:, 0] / period)
        total = np.zeros_like(result)
        np.add.at(total, pairs[:, 0], difference)
        np.add.at(total, pairs[:, 1], -difference)
        step = 0.95 * total / degree[:, None]
        result[free] += step[free]
    return result


class Mesh:
    """A quad mesh being changed by edge rotations; the vertex positions of the
    starting grid are kept as combinatorial coordinates."""

    def __init__(self, width, height, periodic=False):
        self.width, self.height, self.periodic = width, height, periodic
        self.faces, self.grid_positions = grid(width, height, periodic)
        self.count = len(self.grid_positions)

    def face(self, i, j):
        return j * self.width + i

    def inner(self, vertex):
        x, y = self.grid_positions[vertex]
        if self.periodic:
            return 0 < y < self.height
        return 0 < x < self.width and 0 < y < self.height

    def distance(self, a, b):
        d = self.grid_positions[a] - self.grid_positions[b]
        if self.periodic:
            d[0] -= self.width * round(d[0] / self.width)
        return float(np.hypot(*d))

    def poles(self, faces=None):
        v = getattr(self, "trial_valences", None)
        if v is None:
            v = valences(self.faces if faces is None else faces, self.count)
        return [(x, int(v[x])) for x in range(self.count) if self.inner(x) and v[x] != 4]

    def improve(self, score, steps):
        """Greedy edge rotations near the poles that raise the score."""
        current = score(self, self.faces)
        for _ in range(steps):
            best = None
            near = [x for x, _ in self.poles()]
            edges = edges_of(self.faces)
            base = valences(self.faces, self.count)
            for edge, pair in edges.items():
                if len(pair) != 2:
                    continue
                if min(self.distance(e, p) for e in edge for p in near) > 2.5:
                    continue
                for choice in (0, 1):
                    rotated = rotate_edge(self.faces, pair[0], pair[1], choice, edges, with_diagonals=True)
                    if rotated is None:
                        continue
                    candidate, old, new = rotated
                    trial = base.copy()
                    trial[list(old)] -= 1
                    trial[list(new)] += 1
                    self.trial_valences = trial
                    value = score(self, candidate)
                    self.trial_valences = None
                    if value is not None and (best is None or value > best[0]):
                        best = (value, candidate)
            if best is None or best[0] <= current:
                break
            current, self.faces = best

    def lay_out(self):
        fixed = [x for x in range(self.count) if not self.inner(x)]
        return lay_out(self.faces, self.grid_positions, fixed, self.width if self.periodic else None)

    def loop_shift(self, row):
        """Walk the edge loop leaving column 0 at this row to the right, once round
        the tube. Returns (number of edges, rows gained) or None if it meets a pole."""
        around = rotation_system(self.faces)
        u, v = row * self.width, row * self.width + 1
        path = [u, v]
        while True:
            w = go_straight(around, u, v)
            if w is None:
                return None
            u, v = v, w
            path.append(v)
            if int(self.grid_positions[v][0]) == 0:
                return len(path) - 1, int(self.grid_positions[v][1]) - row, path


def pole_pairs_score(rows, tight=1.5, diagonal_bonus=0.0):
    """Score for moving the pole pairs, each a valence 3 next to a valence 5, to the
    given rows (one pair per row, taken from the bottom up, in each column group)."""

    def score(mesh, faces):
        poles = mesh.poles(faces)
        if len(poles) != 2 * len(rows):
            return None
        poles.sort(key=lambda p: mesh.grid_positions[p[0]][1])
        total = 0.0
        for k, row in enumerate(rows):
            pair = poles[2 * k : 2 * k + 2]
            if {v for _, v in pair} != {3, 5}:
                return None
            gap = mesh.distance(pair[0][0], pair[1][0])
            if gap > tight:
                return None
            middle = np.mean([mesh.grid_positions[x][1] for x, _ in pair])
            total -= abs(middle - row) - diagonal_bonus * gap
        return total

    return score


def draw_faces(ax, mesh, positions, keep=None, colour=GRID, width=0.9, period=None):
    for face in mesh.faces:
        points = positions[face].copy()
        if period:
            points[:, 0] = points[0, 0] + (points[:, 0] - points[0, 0] + period / 2) % period - period / 2
        if keep is not None and not keep(points):
            continue
        closed = np.vstack([points, points[:1]])
        ax.plot(closed[:, 0], closed[:, 1], "-", color=colour, lw=width, zorder=2)


def unwrap_path(points, period):
    """Unwrap x along a path on a tube so that it is continuous."""
    result = points.copy()
    for k in range(1, len(result)):
        d = result[k, 0] - result[k - 1, 0]
        result[k, 0] = result[k - 1, 0] + d - period * round(d / period)
    return result


FIGURES = {}


def figure(function):
    FIGURES[function.__name__] = function
    return function


# ----------------------------------------------------------------------------
# 1. Meshes
# ----------------------------------------------------------------------------


@figure
def valence_poles():
    fig, axes = plt.subplots(1, 3, figsize=(12, 4.9))
    titles = {
        4: "Regular vertex\n4 edges meet",
        3: "Pole of valence 3\n3 edges meet (index +1)",
        5: "Pole of valence 5\n5 edges meet (index −1)",
    }
    n = 3
    for ax, k in zip(axes, (4, 3, 5)):
        start = 0.0 if k == 4 else np.pi / 2
        directions = [
            np.array([np.cos(start + 2 * np.pi * i / k), np.sin(start + 2 * np.pi * i / k)]) for i in range(k)
        ]
        for i in range(k):
            a, b = directions[i], directions[(i + 1) % k]
            for t in range(n + 1):
                for u, w in ((a, b), (b, a)):
                    p, q = t * u, t * u + n * w
                    ax.plot([p[0], q[0]], [p[1], q[1]], "-", color=GRID, lw=1.3, zorder=1)
        for i, d in enumerate(directions):
            ax.plot([0, d[0]], [0, d[1]], "-", color=INK, lw=4.5, solid_capstyle="round", zorder=5)
        colour = {4: INK, 3: ORANGE, 5: PURPLE}[k]
        pole_marker(ax, (0, 0), k, size=22)
        ax.text(0, -5.55, titles[k], ha="center", va="top", fontsize=16, color=colour if k != 4 else INK)
        ax.set_xlim(-5.2, 5.2)
        ax.set_ylim(-7.4, 5.2)
        bare(ax)
    fig.subplots_adjust(left=0.01, right=0.99, top=0.99, bottom=0.01, wspace=0.05)
    save(fig, "valence_poles.png")


def cylinder_sketch(ax, bands, rows=8, row_height=0.4, radius=1.0, tilt=0.3, centre=(0, 0)):
    """An opaque cylinder in oblique view with strips drawn on it.
    bands: list of (z of the lower edge as a function of t, t start, t end, colour)."""
    cx, cy = centre
    height = rows * row_height

    def project(t, z):
        return cx + radius * np.cos(t), cy + z + tilt * np.sin(t)

    t = np.linspace(-np.pi, 0, 80)
    x, y = project(t, 0)
    xt, yt = project(t[::-1], height)
    ax.add_patch(Polygon(np.c_[np.r_[x, xt], np.r_[y, yt]], fc="#f3f5f8", ec="none", zorder=1))
    full = np.linspace(-np.pi, np.pi, 160)
    ax.add_patch(Polygon(np.c_[project(full, height)], fc="#fafbfc", ec=INK, lw=1.6, zorder=3))
    for m in range(rows + 1):
        ax.plot(*project(t, m * row_height), "-", color=GRID, lw=0.8, zorder=2)
    for k in range(16):
        angle = -np.pi + (k + 0.5) * np.pi / 8
        if np.sin(angle) < 0:
            ax.plot(*project(np.array([angle, angle]), np.array([0, height])), "-", color=GRID, lw=0.8, zorder=2)
    for lower, t0, t1, colour in bands:
        ts = np.linspace(t0, t1, 400)
        zs = lower(ts)
        sin = np.sin(ts)
        front = sin < 0
        # split into runs of front and back
        start = 0
        for k in range(1, len(ts) + 1):
            if k == len(ts) or front[k] != front[start]:
                seg = slice(start, k + 1 if k < len(ts) else k)
                tt, zz = ts[seg], zs[seg]
                xa, ya = project(tt, zz)
                xb, yb = project(tt[::-1], zz[::-1] + row_height)
                if front[start]:
                    ax.add_patch(Polygon(np.c_[np.r_[xa, xb], np.r_[ya, yb]], fc=colour, ec=colour, lw=1.0, alpha=0.95, zorder=4))
                else:
                    ax.add_patch(Polygon(np.c_[np.r_[xa, xb], np.r_[ya, yb]], fc=colour, ec="none", alpha=0.22, zorder=0.5))
                start = k
    ax.plot(*project(np.array([-np.pi, -np.pi]), np.array([0, height])), color=INK, lw=1.6, zorder=6)
    ax.plot(*project(np.array([0, 0]), np.array([0, height])), color=INK, lw=1.6, zorder=6)
    ax.plot(*project(t, 0), color=INK, lw=1.6, zorder=6)


@figure
def ring_helix():
    fig = plt.figure(figsize=(12, 8.4))
    rows, width, row_height = 6, 10, 1.0
    for column, helix in enumerate((False, True)):
        colour = RED if helix else BLUE
        top = fig.add_axes([0.03 + 0.5 * column, 0.50, 0.44, 0.42])
        if helix:
            bands = [
                (lambda t: 0.8 + 0.4 * (t + np.pi) / (2 * np.pi), -np.pi, np.pi, RED),
                (lambda t: 0.8 + 0.4 * (t + np.pi) / (2 * np.pi), np.pi, 3 * np.pi, RED_LIGHT),
            ]
        else:
            bands = [(lambda t: 1.6 + 0 * t, -np.pi, np.pi, BLUE)]
        cylinder_sketch(top, bands)
        top.set_xlim(-2.2, 2.2)
        top.set_ylim(-0.45, 3.65)
        bare(top)
        title = "Helix (spiral), pitch 1" if helix else "Ring (closed loop)"
        fig.text(0.25 + 0.5 * column, 0.94, title, ha="center", fontsize=19, color=colour, weight="bold")

        ax = fig.add_axes([0.02 + 0.5 * column, 0.02, 0.47, 0.46])
        slope = 1.0 / width if helix else 0.0
        ax.add_patch(Rectangle((0, 0), width, rows, fc="white", ec=INK, lw=1.8, zorder=3, fill=False))
        for i in range(width + 1):
            ax.plot([i, i], [0, rows], color=GRID, lw=1.0, zorder=1)
        for j in range(-1 if helix else 0, rows + 1):
            xs = np.array([0.0, width])
            ys = j + slope * xs
            if helix:
                # clip the slanted line to the rectangle
                x0 = 0 if j >= 0 else -j / slope
                x1 = width if j + 1 <= rows else (rows - j) / slope
                xs = np.array([x0, x1])
                ys = j + slope * xs
            ax.plot(xs, ys, color=GRID, lw=1.0, zorder=1)
        laps = 2 if helix else 1
        for lap in range(laps):
            base = 2 + lap
            shade = colour if lap == 0 else (RED_LIGHT if helix else BLUE_LIGHT)
            corners = [(0, base), (width, base + slope * width), (width, base + 1 + slope * width), (0, base + 1)]
            ax.add_patch(Polygon(corners, fc=shade, ec="none", alpha=0.9, zorder=2))
        arrow(ax, (0.3, 2.5), (width - 0.3, 2.5 + slope * (width - 0.6)), colour="white", width=2.4, scale=18)
        # glued sides
        for x in (0, width):
            for y in (0.5, rows - 0.5):
                ax.text(x, y, "▶", ha="center", va="center", fontsize=13, color=MUTED, zorder=6)
        ax.text(width / 2, -0.5, "left and right sides are glued: the tube cut open", ha="center", va="top",
                fontsize=14, color=MUTED)
        if helix:
            label(ax, (width + 0.25, 3.5), "comes back\n1 row higher", colour=RED, ha="left", size=14)
            ax.annotate("", xy=(-0.25, 3.5), xytext=(-0.25, 2.5),
                        arrowprops=dict(arrowstyle="-|>", color=GREEN, lw=2.2, mutation_scale=16))
            label(ax, (-0.4, 3.0), "pitch 1", colour=GREEN, ha="right", size=14, box=False)
        else:
            label(ax, (width + 0.25, 2.5), "comes back\nto its start", colour=BLUE, ha="left", size=14)
        if helix:
            label(ax, (1.6, 3.65), "next turn", colour=RED, size=12, box=False)
        ax.set_xlim(-2.2, width + 3.7)
        ax.set_ylim(-1.4, rows + 0.4)
        bare(ax)
    save(fig, "ring_helix.png")


def copy_render(source, name, width, colours=256):
    """Copy a render of the engine's results, smaller."""
    path = os.path.join(SCRATCH, source)
    if not os.path.exists(path):
        print(f"{name}: source {path} missing, kept as it is")
        return
    image = Image.open(path).convert("RGB")
    height = round(image.size[1] * width / image.size[0])
    image = image.resize((width, height), Image.Resampling.LANCZOS)
    image = image.quantize(colors=colours, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE)
    out = os.path.join(HERE, name)
    image.save(out, optimize=True)
    print(f"{name}: {width}x{height}, {os.path.getsize(out) // 1024} KB")


def before_after(source, name, boxes, titles, height_inches):
    """Crop the two panels of a before/after render and label them."""
    path = os.path.join(SCRATCH, source)
    if not os.path.exists(path):
        print(f"{name}: source {path} missing, kept as it is")
        return
    image = Image.open(path).convert("RGB")
    fig, axes = plt.subplots(1, 2, figsize=(12, height_inches))
    for ax, box, (title, colour) in zip(axes, boxes, titles):
        ax.imshow(image.crop(box), interpolation="lanczos")
        ax.set_title(title, fontsize=18, color=colour, weight="bold")
        ax.axis("off")
    fig.subplots_adjust(left=0.005, right=0.995, top=0.93, bottom=0.005, wspace=0.02)
    save(fig, name, colours=256)


@figure
def cylinder_result():
    before_after(
        "images/cyl-compare.png",
        "cylinder_result.png",
        [(160, 110, 760, 820), (1076, 110, 1676, 820)],
        [("Before: spirals (red)", RED), ("After: no spiral left (blue: closed loops)", BLUE)],
        6.8,
    )


@figure
def male_result():
    before_after(
        "images/male-compare.png",
        "male_result.png",
        [(180, 90, 720, 870), (1096, 90, 1636, 870)],
        [("Before: spirals (yellow to red)", ORANGE), ("After: far fewer spirals", BLUE)],
        8.4,
    )


@figure
def simplicity():
    copy_render("simplicity-work/male-simplicity.png", "simplicity.png", 1600)


# ----------------------------------------------------------------------------
# 2. The cross field
# ----------------------------------------------------------------------------


def cross(ax, centre, angle, size, colour=INK, width=2.2, zorder=5):
    for k in range(2):
        a = angle + k * np.pi / 2
        d = size * np.array([np.cos(a), np.sin(a)])
        ax.plot([centre[0] - d[0], centre[0] + d[0]], [centre[1] - d[1], centre[1] + d[1]], "-",
                color=colour, lw=width, solid_capstyle="round", zorder=zorder)


@figure
def cross_field():
    fig, (left, right) = plt.subplots(1, 2, figsize=(12, 5.0), gridspec_kw={"width_ratios": [1, 1.2]})
    # A bent shape: the crosses follow its bend.
    inner_radius, outer_radius, a0, a1 = 2.0, 4.0, np.radians(22), np.radians(158)
    t = np.linspace(a0, a1, 100)
    outline = np.r_[np.c_[outer_radius * np.cos(t), outer_radius * np.sin(t)],
                    np.c_[inner_radius * np.cos(t[::-1]), inner_radius * np.sin(t[::-1])]]
    left.add_patch(Polygon(outline, fc=LIGHT, ec=INK, lw=2, zorder=1))
    for r in np.linspace(inner_radius, outer_radius, 7)[1:-1]:
        left.plot(r * np.cos(t), r * np.sin(t), "-", color=BLUE_LIGHT, lw=1.2, zorder=2)
    for a in np.linspace(a0, a1, 15)[1:-1]:
        left.plot([inner_radius * np.cos(a), outer_radius * np.cos(a)],
                  [inner_radius * np.sin(a), outer_radius * np.sin(a)], "-", color=BLUE_LIGHT, lw=1.2, zorder=2)
    for r in (2.5, 3.0, 3.5):
        count = int(r * (a1 - a0) / 0.62)
        for a in np.linspace(a0, a1, count + 2)[1:-1]:
            cross(left, (r * np.cos(a), r * np.sin(a)), a, 0.19)
    left.text(0, -0.55, "the crosses follow the bend of the shape;\nthe quad edges (light lines) will run along them",
              ha="center", va="center", fontsize=14, color=MUTED)
    left.set_xlim(-4.4, 4.4)
    left.set_ylim(-1.3, 4.2)
    bare(left)
    left.set_title("Cross field", fontsize=18, color=INK, weight="bold")

    # A singularity: going once round it, the cross comes back turned by a quarter turn.
    radius = 3.0
    right.add_patch(Circle((0, 0), radius, fc=LIGHT, ec=INK, lw=2, zorder=0))
    for k in range(3):
        a = 2 * np.pi * k / 3
        right.plot([0, radius * np.cos(a)], [0, radius * np.sin(a)], "-", color=ORANGE, lw=2.5, zorder=1)
    for r in (0.85, 2.45):
        count = int(2 * np.pi * r / 0.62)
        for a in np.linspace(0, 2 * np.pi, count, endpoint=False) + 0.2:
            cross(right, (r * np.cos(a), r * np.sin(a)), a / 4, 0.19, colour=MUTED, width=1.8)
    loop = 1.65
    start = np.pi / 4
    t = np.linspace(start + 0.12, start + 2 * np.pi - 0.12, 200)
    right.plot(loop * np.cos(t), loop * np.sin(t), "--", color=GREEN, lw=1.6, zorder=2)
    arrow(right, (loop * np.cos(start + 2.6), loop * np.sin(start + 2.6)),
          (loop * np.cos(start + 2.75), loop * np.sin(start + 2.75)), colour=GREEN, scale=18)
    for a in start + np.linspace(0, 2 * np.pi, 9)[:-1]:
        p = (loop * np.cos(a), loop * np.sin(a))
        cross(right, p, a / 4, 0.24, colour=INK, width=2.0)
        d = 0.24 * np.array([np.cos(a / 4), np.sin(a / 4)])
        right.plot([p[0], p[0] + d[0]], [p[1], p[1] + d[1]], "-", color=GREEN, lw=4.5,
                   solid_capstyle="round", zorder=6)
    # the marked arm after one full round, back at the start
    p = np.array([loop * np.cos(start), loop * np.sin(start)])
    after = start / 4 + np.pi / 2
    d = 0.24 * np.array([np.cos(after), np.sin(after)])
    right.plot([p[0], p[0] + d[0]], [p[1], p[1] + d[1]], "-", color=GREEN, lw=4.5, alpha=0.45,
               solid_capstyle="round", zorder=6)
    arrow(right, p + 0.34 * np.array([np.cos(start / 4), np.sin(start / 4)]),
          p + 0.34 * np.array([np.cos(after), np.sin(after)]), colour=GREEN, rad=0.6, scale=12, width=1.5)
    label(right, p + np.array([0.15, 0.55]), "start", colour=GREEN, size=12, box=False)
    label(right, (3.15, 1.2), "go once round:\nthe green arm\ncomes back turned\nby a quarter turn",
          colour=GREEN, ha="left", size=13)
    pole_marker(right, (0, 0), 3, size=17)
    label(right, (-3.3, -1.9), "singularity:\nbecomes a pole of\nvalence 3, with 3\nlines leaving it", colour=ORANGE,
          ha="right", size=13)
    right.set_xlim(-6.4, 6.6)
    right.set_ylim(-3.2, 3.2)
    bare(right)
    right.set_title("Singularity of the field", fontsize=18, color=INK, weight="bold")
    fig.subplots_adjust(left=0.01, right=0.99, top=0.91, bottom=0.01, wspace=0.04)
    save(fig, "cross_field.png")


# ----------------------------------------------------------------------------
# 4. The layout
# ----------------------------------------------------------------------------


def trace_separatrices(mesh):
    """Motorcycle traces: every separatrix advances one edge per step and stops on
    anything already drawn, on a pole or on the border. Returns the paths."""
    around = rotation_system(mesh.faces)
    poles = {x for x, _ in mesh.poles()}
    neighbours = defaultdict(set)
    for face in mesh.faces:
        for k in range(4):
            neighbours[face[k]].add(face[(k + 1) % 4])
            neighbours[face[(k + 1) % 4]].add(face[k])
    occupied = set(poles)
    traces = [{"path": [p, q], "alive": True} for p in sorted(poles) for q in sorted(neighbours[p])]
    claims = defaultdict(list)
    for trace in traces:
        claims[trace["path"][-1]].append(trace)
    while claims:
        for vertex, claimants in claims.items():
            if vertex in occupied or not mesh.inner(vertex) or vertex in poles:
                for trace in claimants:
                    trace["alive"] = False
            elif len(claimants) > 1:
                # two traces reach a free vertex together: head on, they merge and
                # both stop; at a right angle, the first one goes on
                first = claimants[0]
                for trace in claimants[1:]:
                    trace["alive"] = False
                    if go_straight(around, trace["path"][-2], vertex) == first["path"][-2]:
                        first["alive"] = False
            occupied.add(vertex)
        claims = defaultdict(list)
        for trace in traces:
            if trace["alive"]:
                u, v = trace["path"][-2], trace["path"][-1]
                trace["path"].append(go_straight(around, u, v))
                claims[trace["path"][-1]].append(trace)
    return [trace["path"] for trace in traces]


def planar_pairs_mesh():
    mesh = Mesh(20, 13)
    mesh.faces = rotate_edge(mesh.faces, mesh.face(9, 6), mesh.face(9, 7), 0)
    mesh.improve(pole_pairs_score([3.5, 9.5], tight=1.01), 10)
    return mesh


@figure
def tmesh():
    mesh = planar_pairs_mesh()
    positions = mesh.lay_out()
    traces = trace_separatrices(mesh)
    poles = dict(mesh.poles())
    walls = set()
    for edge, pair in edges_of(mesh.faces).items():
        if len(pair) == 1:
            walls.add(edge)
    cut = set(walls)
    for path in traces:
        for a, b in zip(path, path[1:]):
            cut.add(frozenset((a, b)))
    # patches: faces joined across edges that are not on a trace
    edge_faces = edges_of(mesh.faces)
    patch = [-1] * len(mesh.faces)
    count = 0
    for start in range(len(mesh.faces)):
        if patch[start] >= 0:
            continue
        stack = [start]
        patch[start] = count
        while stack:
            f = stack.pop()
            face = mesh.faces[f]
            for k in range(4):
                edge = frozenset((face[k], face[(k + 1) % 4]))
                if edge in cut:
                    continue
                for g in edge_faces[edge]:
                    if patch[g] < 0:
                        patch[g] = count
                        stack.append(g)
        count += 1
    # colour the patches so that neighbours differ
    touching = defaultdict(set)
    for edge, pair in edge_faces.items():
        if len(pair) == 2 and patch[pair[0]] != patch[pair[1]]:
            touching[patch[pair[0]]].add(patch[pair[1]])
            touching[patch[pair[1]]].add(patch[pair[0]])
    colour_of = {}
    for p in range(count):
        used = {colour_of.get(q) for q in touching[p]}
        colour_of[p] = next(c for c in range(len(PATCH_COLOURS)) if c not in used)
    fig, ax = plt.subplots(figsize=(12, 8.3))
    for f, face in enumerate(mesh.faces):
        ax.add_patch(Polygon(positions[face], fc=PATCH_COLOURS[colour_of[patch[f]]], ec=GRID, lw=0.7, zorder=1))
    for edge in walls:
        a, b = tuple(edge)
        ax.plot(*positions[[a, b]].T, "-", color=INK, lw=3.0, zorder=3)
    for path in traces:
        ax.plot(*positions[path].T, "-", color=INK, lw=3.0, zorder=4, solid_capstyle="round")
    # nodes: poles, ends of traces (T-junctions), border hits
    ends = {path[-1] for path in traces}
    junctions = [v for v in ends if mesh.inner(v) and v not in poles]
    for v in ends:
        if v not in poles:
            ax.plot(*positions[v], "o", ms=9, color=INK, mec="white", mew=1.5, zorder=6)
    for v, k in poles.items():
        pole_marker(ax, positions[v], k, size=18)
    # an arc: from the upper valence-5 pole to the first node along its trace to the left
    upper_five = max((v for v, k in poles.items() if k == 5), key=lambda v: positions[v][1])
    leftward = min((t for t in traces if t[0] == upper_five), key=lambda t: positions[t[-1]][0])
    stop = next(k for k, v in enumerate(leftward) if k > 0 and v in ends)
    arc = leftward[: stop + 1]
    ax.plot(*positions[arc].T, "-", color=GREEN, lw=6, zorder=5, solid_capstyle="round")
    middle = positions[arc[len(arc) // 2]]
    label(ax, middle + np.array([0, 0.75]), "arc: a piece of trace between two nodes", colour=GREEN, size=15)
    junction = positions[arc[-1]]
    label(ax, junction + np.array([-3.2, -1.7]), "T-junction:\na trace stops\non another", size=15)
    arrow(ax, junction + np.array([-2.0, -1.0]), junction + np.array([-0.25, -0.2]), width=1.8)
    upward = max((t for t in traces if t[0] == upper_five), key=lambda t: positions[t[-1]][1])
    point = positions[upward[2]]
    label(ax, point + np.array([4.3, 0.3]), "separatrix:\na line leaving a pole", size=15)
    arrow(ax, point + np.array([2.3, 0.2]), point + np.array([0.2, 0.0]), width=1.8)
    label(ax, (9.0, 6.4), "patch:\na rectangle\nof quads", size=17, colour=BLUE)
    lower_three = min((v for v, k in poles.items() if k == 3), key=lambda v: positions[v][1])
    point = positions[lower_three]
    label(ax, point + np.array([-2.9, 1.9]), "pole: a node too", size=15)
    arrow(ax, point + np.array([-2.0, 1.5]), point + np.array([-0.3, 0.3]), width=1.8)
    label(ax, (16.2, 1.4), "border (a wall): traces stop on it", size=15)
    arrow(ax, (16.2, 1.0), (16.2, 0.1), width=1.8)
    ax.set_xlim(-0.6, 20.6)
    ax.set_ylim(-0.6, 13.6)
    bare(ax)
    fig.subplots_adjust(left=0.01, right=0.99, top=0.99, bottom=0.01)
    save(fig, "tmesh.png")


# ----------------------------------------------------------------------------
# 5. Quantization
# ----------------------------------------------------------------------------


def three_patches(ax, w1, w2, left, upper, lower, quads, text, colour=INK):
    """A patch on the left; on its right two stacked patches meeting it at a T-junction.
    left: height of the left patch; upper, lower: heights of the right patches."""
    boxes = [(0, 0, w1, left), (w1, lower, w2, upper), (w1, 0, w2, lower)]
    for k, (x, y, w, h) in enumerate(boxes):
        ax.add_patch(Rectangle((x, y), w, h, fc=PATCH_COLOURS[k], ec=INK, lw=2.4, zorder=2))
        if quads:
            for i in range(1, int(round(w))):
                ax.plot([x + i, x + i], [y, y + h], "-", color=GRID, lw=1.0, zorder=3)
            for j in range(1, int(round(h))):
                ax.plot([x, x + w], [y + j, y + j], "-", color=GRID, lw=1.0, zorder=3)
    ax.plot(w1, lower, "o", ms=8, color=INK, zorder=5)
    size = 15
    label(ax, (-0.35, left / 2), text(left), ha="right", size=size, colour=colour, box=False)
    label(ax, (w1 / 2, left + 0.3), text(w1), size=size, colour=colour, box=False)
    label(ax, (w1 / 2, -0.35), text(w1), size=size, colour=colour, box=False)
    label(ax, (w1 + w2 / 2, lower + upper + 0.3), text(w2), size=size, colour=colour, box=False)
    label(ax, (w1 + w2 / 2, -0.35), text(w2), size=size, colour=colour, box=False)
    label(ax, (w1 + w2 + 0.3, lower + upper / 2), text(upper), ha="left", size=size, colour=colour, box=False)
    label(ax, (w1 + w2 + 0.3, lower / 2), text(lower), ha="left", size=size, colour=colour, box=False)


@figure
def quantization():
    fig, axes = plt.subplots(1, 3, figsize=(12, 4.0))
    real = lambda v: f"{v:.1f}"
    whole = lambda v: f"{int(round(v))}"
    three_patches(axes[0], 3.4, 5.7, 4.8, 2.4, 2.4, False, real)
    axes[0].set_title("The lengths the field asks for\n(real numbers)", fontsize=15)
    three_patches(axes[1], 3, 6, 5, 2, 2, True, whole)
    axes[1].plot([3, 3], [4, 5], "-", color=RED, lw=5, zorder=6, solid_capstyle="round")
    label(axes[1], (4.5, -1.15), "left side 5 \u2260 right side 2 + 2", colour=RED, size=15, box=False)
    axes[1].set_title("Rounding each length alone:\nthe quads do not fit", fontsize=15, color=RED)
    three_patches(axes[2], 3, 6, 5, 2, 3, True, whole)
    label(axes[2], (4.5, -1.15), "left side 5 = right side 2 + 3", colour=GREEN, size=15, box=False)
    axes[2].set_title("Quantization: whole numbers,\nopposite sides equal (balance)", fontsize=15, color=GREEN)
    for ax in axes:
        ax.set_xlim(-1.3, 10.3)
        ax.set_ylim(-1.6, 5.6)
        bare(ax)
    fig.subplots_adjust(left=0.01, right=0.99, top=0.85, bottom=0.0, wspace=0.06)
    save(fig, "quantization.png")


# ----------------------------------------------------------------------------
# 6. Turns
# ----------------------------------------------------------------------------


def quads_box(ax, x, y, w, h, fill, edge=INK, alpha=1.0, lw=2.2, zorder=2, ls="-"):
    ax.add_patch(Rectangle((x, y), w, h, fc=fill, ec="none", alpha=alpha, zorder=zorder))
    for i in range(1, w):
        ax.plot([x + i, x + i], [y, y + h], "-", color=GRID, lw=1.0, alpha=alpha, zorder=zorder + 0.1)
    for j in range(1, h):
        ax.plot([x, x + w], [y + j, y + j], "-", color=GRID, lw=1.0, alpha=alpha, zorder=zorder + 0.1)
    ax.add_patch(Rectangle((x, y), w, h, fc="none", ec=edge, lw=lw, alpha=alpha, zorder=zorder + 0.3, ls=ls))


@figure
def turn_shift():
    fig, axes = plt.subplots(2, 1, figsize=(12, 8.2), gridspec_kw={"height_ratios": [8.0, 8.9]})
    for ax, shift in zip(axes, (0, 1)):
        colour = BLUE if shift == 0 else RED
        quads_box(ax, 0, 0, 4, 6, PATCH_COLOURS[0])
        quads_box(ax, 4, 0, 5, 2, PATCH_COLOURS[1])
        quads_box(ax, 4, 2, 5, 4, PATCH_COLOURS[2])
        quads_box(ax, 9, 0, 3, 4, PATCH_COLOURS[3])
        quads_box(ax, 9, 4, 3, 2, PATCH_COLOURS[4])
        # the first patch again, past the cut, as the tube brings it back
        quads_box(ax, 12, -shift, 4, 6, PATCH_COLOURS[0], alpha=0.55, lw=1.6, ls="--")
        for x, y, name in ((2, 5.4, "patch A"), (14, 5.4 - shift, "patch A again")):
            ax.text(x, y, name, ha="center", va="center", fontsize=15, color=MUTED, zorder=7,
                    bbox=dict(boxstyle="round,pad=0.15", fc="white", ec="none", alpha=0.8))
        ax.plot([12, 12], [-0.5 - shift * 0.9, 6.6], "-", color=INK, lw=1.2, zorder=6)
        ax.text(12, 6.75, "cut of the tube", ha="center", va="bottom", fontsize=13, color=MUTED)
        # the strip followed by the walk
        ax.add_patch(Rectangle((0, 2), 16, 1, fc=colour, alpha=0.85, ec="none", zorder=4))
        # where it started, seen in the copy of A
        ax.add_patch(Rectangle((12, 2 - shift), 4, 1, fc="none", ec=INK, lw=2.2, ls=(0, (4, 3)), zorder=5))
        arrow(ax, (0.2, 2.5), (15.6, 2.5), colour="white", width=2.6, scale=20)
        ax.text(-0.3, 2.5, "start", ha="right", va="center", fontsize=14, color=colour)
        if shift == 0:
            label(ax, (16.4, 2.5), "back in A at its start:\nshift 0, a ring", colour=BLUE, ha="left", size=15)
        else:
            ax.annotate("", xy=(16.25, 2.5), xytext=(16.25, 1.5),
                        arrowprops=dict(arrowstyle="-|>", color=GREEN, lw=2.6, mutation_scale=18))
            label(ax, (16.5, 2.0), "back in A one row\nhigher: shift 1,\na helix of pitch 1", colour=RED,
                  ha="left", size=15)
            ax.text(14, 2.5 - shift, "its start", ha="center", va="center", fontsize=12, color=INK, zorder=7)
        ax.set_xlim(-1.6, 21.2)
        ax.set_ylim(-0.7 - shift * 0.9, 7.3)
        bare(ax)
    fig.subplots_adjust(left=0.01, right=0.99, top=0.99, bottom=0.01, hspace=0.06)
    save(fig, "turn_shift.png")


def crystal_with_dislocation(columns=14, rows=12, poisson=0.3):
    """Atoms of a square crystal with one edge dislocation at the origin, placed by the
    displacement field of linear elasticity, and their bonds."""
    atoms, row_of = [], []
    for j in range(-rows // 2, rows // 2):
        for i in range(-columns // 2, columns // 2):
            x, y = i + 0.5, j + 0.5
            r2 = x * x + y * y
            theta = np.arctan2(y, x)
            ux = (theta + x * y / (2 * (1 - poisson) * r2)) / (2 * np.pi)
            uy = -((1 - 2 * poisson) / (4 * (1 - poisson)) * np.log(r2) + (x * x - y * y) / (4 * (1 - poisson) * r2)) / (2 * np.pi)
            atoms.append((x + ux, y + uy))
            row_of.append(j)
    atoms = np.array(atoms)
    row_of = np.array(row_of)
    right, up = {}, {}
    rows_list = sorted(set(row_of))
    members = {j: sorted(np.flatnonzero(row_of == j), key=lambda a: atoms[a][0]) for j in rows_list}
    for j in rows_list:
        for a, b in zip(members[j], members[j][1:]):
            right[a] = b
    for j in rows_list[:-1]:
        below, above = members[j], members[j + 1]
        for a in below:
            b = min(above, key=lambda c: abs(atoms[c][0] - atoms[a][0]))
            back = min(below, key=lambda c: abs(atoms[c][0] - atoms[b][0]))
            if back == a and abs(atoms[b][0] - atoms[a][0]) < 0.5:
                up[a] = b
    return atoms, right, up


def dislocated_plane_mesh():
    mesh = Mesh(30, 20)
    mesh.faces = rotate_edge(mesh.faces, mesh.face(14, 10), mesh.face(15, 10), 0)

    def spread(mesh, faces):
        poles = mesh.poles(faces)
        if sorted(v for _, v in poles) != [3, 3, 5, 5]:
            return None
        threes = [x for x, v in poles if v == 3]
        fives = [x for x, v in poles if v == 5]
        for t in threes:
            if min(mesh.distance(t, f) for f in fives) > 1.5:
                return None
        return mesh.distance(*threes) + mesh.distance(*fives)

    mesh.improve(spread, 7)
    return mesh


def mesh_circuit(mesh, start, first, legs):
    """Walk along edges: legs[k] edges straight on, then a left turn."""
    around = rotation_system(mesh.faces)
    path = [start, first]
    for k, length in enumerate(legs):
        for _ in range(length - 1):
            path.append(go_straight(around, path[-2], path[-1]))
        if k + 1 < len(legs):
            path.append(turn_left(around, path[-2], path[-1]))
    return path


@figure
def burgers():
    fig, (left, right) = plt.subplots(1, 2, figsize=(12, 6.6))
    atoms, right_of, up_of = crystal_with_dislocation()
    inside = lambda p: abs(p[0]) <= 5.75 and abs(p[1]) <= 5.75
    for a, b in list(right_of.items()) + list(up_of.items()):
        if inside(atoms[a]) and inside(atoms[b]):
            left.plot(*atoms[[a, b]].T, "-", color=GRID, lw=1.4, zorder=1)
    left_of = {b: a for a, b in right_of.items()}
    down_of = {b: a for a, b in up_of.items()}
    # the extra half row: a column of atoms that ends at the core, with no partner beyond
    core = [a for a in range(len(atoms)) if np.hypot(*atoms[a]) < 1.6]
    half = []
    for a in core:
        if a not in up_of and atoms[a][1] < 0:
            step = down_of
        elif a not in down_of and atoms[a][1] > 0:
            step = up_of
        else:
            continue
        while a is not None:
            half.append(a)
            a = step.get(a)
    shown = [a for a in range(len(atoms)) if inside(atoms[a])]
    left.plot(*atoms[shown].T, "o", ms=8, color="#c9d0d8", mec=GRID, zorder=2)
    half = [a for a in half if inside(atoms[a])]
    left.plot(*atoms[half].T, "o", ms=9, color=INK, zorder=3)
    start = min(range(len(atoms)), key=lambda a: np.hypot(*(atoms[a] - (-3.5, -3.5))))
    path = [start]
    for step, count in ((right_of, 6), (up_of, 6), (left_of, 6), (down_of, 6)):
        for _ in range(count):
            path.append(step[path[-1]])
    left.plot(*atoms[path].T, "-", color=INK, lw=3, zorder=4)
    left.plot(*atoms[start], "s", ms=11, color=INK, zorder=5)
    arrow(left, atoms[path[-1]], atoms[start], colour=GREEN, width=3.5, scale=22, zorder=6)
    print("crystal circuit gap", (atoms[start] - atoms[path[-1]]).round(2))
    label(left, (0, -6.7), "from the square: 6 steps right, 6 up, 6 left, 6 down", size=13)
    label(left, (atoms[start][0] - 0.35, atoms[start][1] - 0.75), "misses by one step:\nthe Burgers vector",
          colour=GREEN, size=13, ha="center", va="top")
    top_of_half = atoms[half][np.argmax(np.abs(atoms[half][:, 1]))]
    label(left, top_of_half + np.array([0, 0.55 * np.sign(top_of_half[1])]), "extra half row", size=13)
    left.set_xlim(-6.6, 6.6)
    left.set_ylim(-7.3, 6.6)
    bare(left)
    left.set_title("A crystal with a dislocation", fontsize=17, weight="bold")

    mesh = dislocated_plane_mesh()
    positions = mesh.lay_out()
    poles = mesh.poles()
    draw_faces(right, mesh, positions)
    index = {tuple(mesh.grid_positions[v]): v for v in range(mesh.count)}
    path = mesh_circuit(mesh, index[(11.0, 8.0)], index[(12.0, 8.0)], [8, 8, 8, 8])
    right.plot(*positions[path].T, "-", color=INK, lw=3, zorder=4)
    right.plot(*positions[path[0]], "s", ms=11, color=INK, zorder=5)
    arrow(right, positions[path[-1]], positions[path[0]], colour=GREEN, width=3.5, scale=22, zorder=6)
    print("quad circuit gap", mesh.grid_positions[path[0]] - mesh.grid_positions[path[-1]])
    for v, k in poles:
        pole_marker(right, positions[v], k, size=16)
    s = positions[path[0]]
    label(right, (15.0, 5.3), "from the square: 8 edges straight on, turn left, and again", size=13)
    label(right, (s[0] - 0.4, s[1] + 0.35), "misses by\none edge across\nand one along", colour=GREEN,
          size=13, ha="right", va="bottom")
    right.set_xlim(7.4, 22.6)
    right.set_ylim(4.4, 18.6)
    bare(right)
    right.set_title("The same in a quad mesh: a 3/5 pair", fontsize=17, weight="bold")
    fig.subplots_adjust(left=0.01, right=0.99, top=0.93, bottom=0.01, wspace=0.04)
    save(fig, "burgers.png")


def draw_tube(ax, mesh, positions):
    """Draw a tube cut open: faces across the cut are drawn on both sides."""
    w = mesh.width
    for face in mesh.faces:
        points = positions[face].copy()
        points[:, 0] = points[0, 0] + (points[:, 0] - points[0, 0] + w / 2) % w - w / 2
        for offset in (-w, 0, w):
            closed = np.vstack([points, points[:1]])
            ax.plot(closed[:, 0] + offset, closed[:, 1], "-", color=GRID, lw=0.9, zorder=2)


def tube_poles(ax, mesh, positions, top):
    for v, k in mesh.poles():
        if positions[v][1] < top:
            for offset in (-mesh.width, 0, mesh.width):
                pole_marker(ax, positions[v] + (offset, 0), k, size=17)


def tube_loop(ax, mesh, positions, row, colour):
    edges, shift, path = mesh.loop_shift(row)
    points = unwrap_path(positions[path], mesh.width)
    ax.plot(*points.T, "-", color=colour, lw=4, zorder=5, solid_capstyle="round")
    ax.plot(*points[0], "o", ms=9, color=colour, zorder=6)
    return edges, shift, points


def glue_marks(ax, width, bottom, top):
    for x in (0, width):
        ax.plot([x, x], [bottom, top], "--", color=MUTED, lw=1.2, zorder=3)


def stacked_pairs_tube():
    mesh = Mesh(16, 13, periodic=True)
    mesh.faces = rotate_edge(mesh.faces, mesh.face(8, 6), mesh.face(8, 7), 0)
    mesh.improve(pole_pairs_score([3, 10], tight=1.5, diagonal_bonus=0.01), 25)
    return mesh


def tube_axes(fig, width, top, box):
    """Axes showing exactly the tube cut open, [0, width] x [0, top]."""
    ax = fig.add_axes(box)
    ax.set_xlim(0, width)
    ax.set_ylim(0, top)
    ax.set_aspect("equal")
    ax.axis("off")
    for x in (0, width):
        ax.plot([x, x], [0, top], "--", color=MUTED, lw=1.6, zorder=8, clip_on=False)
    ax.plot([0, width], [0, 0], "-", color=INK, lw=2.0, zorder=8, clip_on=False)
    return ax


@figure
def staircase():
    mesh = stacked_pairs_tube()
    positions = mesh.lay_out()
    top, w = 8.0, mesh.width
    fig = plt.figure(figsize=(12, 6.4))
    ax = tube_axes(fig, w, top, [0.09, 0.11, 0.66, 0.76])
    draw_tube(ax, mesh, positions)
    lower = tube_loop(ax, mesh, positions, 1, BLUE)
    upper = tube_loop(ax, mesh, positions, 5, RED)
    print("lower turn", lower[:2], "upper turn", upper[:2])
    poles = dict(mesh.poles())
    low = sorted(poles, key=lambda v: positions[v][1])[:2]
    face = next(f for f in mesh.faces if low[0] in f and low[1] in f)
    k0, k1 = face.index(low[0]), face.index(low[1])
    corner = face[(k0 + 1) % 4] if (k0 + 2) % 4 == k1 else face[(k1 + 1) % 4]
    stair = positions[[low[0], corner, low[1]]]
    tube_poles(ax, mesh, positions, top)
    ax.plot(*stair.T, "-", color=GREEN, lw=7, zorder=7, solid_capstyle="round")
    label(ax, stair[1] + np.array([3.6, -0.2]), "staircase: 2 edges\nfrom the 5 to the 3", colour=GREEN, size=14)
    # the shifts, read on the cut
    ax.annotate("", xy=(w + 0.4, upper[2][-1][1]), xytext=(w + 0.4, upper[2][0][1]),
                arrowprops=dict(arrowstyle="-|>", color=GREEN, lw=2.6, mutation_scale=18), annotation_clip=False)
    ax.text(w + 0.9, (upper[2][0][1] + upper[2][-1][1]) / 2, f"comes back {upper[1]} rows\nhigher: shift {upper[1]}",
            color=RED, ha="left", va="center", fontsize=14)
    ax.text(w + 0.9, lower[2][0][1], f"comes back to\nits start: shift {lower[1]}", color=BLUE, ha="left",
            va="center", fontsize=14)
    ax.text(-0.5, upper[2][0][1], "turn 2", color=RED, ha="right", va="center", fontsize=14)
    ax.text(-0.5, lower[2][0][1], "turn 1", color=BLUE, ha="right", va="center", fontsize=14)
    fig.text(0.47, 0.95, f"shift of turn 2 \u2212 shift of turn 1 = {upper[1]} \u2212 {lower[1]} = {upper[1] - lower[1]}"
             " = length of the staircase", ha="center", va="center", fontsize=16, color=INK)
    fig.text(0.42, 0.05, "a tube cut open: the dashed sides are glued", ha="center", va="center", fontsize=13,
             color=MUTED)
    save(fig, "staircase.png")


def chain_tube():
    """A tube with three 3/5 pairs of the same orientation stacked up the tube; their
    opposite pairs are moved far up, out of the picture."""
    mesh = Mesh(24, 26, periodic=True)
    columns, lows, high = [4, 12, 20], [4, 9, 14], 21
    for k, (column, low) in enumerate(zip(columns, lows)):
        row = (low + high) // 2
        mesh.faces = rotate_edge(mesh.faces, mesh.face(column, row), mesh.face(column, row + 1), 0)

        def score(mesh, faces, k=k, column=column, low=low):
            poles = mesh.poles(faces)
            if len(poles) != 4 * (k + 1):
                return None
            near = []
            for x, v in poles:
                d = abs((mesh.grid_positions[x][0] - column + mesh.width / 2) % mesh.width - mesh.width / 2)
                if d <= 3:
                    near.append((x, v))
            if len(near) != 4:
                return None
            near.sort(key=lambda p: mesh.grid_positions[p[0]][1])
            total = 0.0
            for pair, target in ((near[:2], low), (near[2:], high)):
                if {v for _, v in pair} != {3, 5}:
                    return None
                gap = mesh.distance(pair[0][0], pair[1][0])
                if gap > 1.5:
                    return None
                middle = np.mean([mesh.grid_positions[x][1] for x, _ in pair])
                total -= abs(middle - target - 0.5) - 0.01 * gap
            return total

        mesh.improve(score, 30)
    return mesh


@figure
def forced_helices():
    mesh = chain_tube()
    positions = mesh.lay_out()
    top, w = 19.6, mesh.width
    fig = plt.figure(figsize=(12, 8.4))
    ax = tube_axes(fig, w, top, [0.1, 0.07, 0.62, 0.76])
    draw_tube(ax, mesh, positions)
    tube_poles(ax, mesh, positions, top)
    for v, k in mesh.poles():
        if positions[v][1] < top and k == 5:
            three = next(u for u, j in mesh.poles() if j == 3 and mesh.distance(u, v) < 1.5)
            face = next(f for f in mesh.faces if v in f and three in f)
            k0, k1 = face.index(v), face.index(three)
            corner = face[(k0 + 1) % 4] if (k0 + 2) % 4 == k1 else face[(k1 + 1) % 4]
            ax.plot(*positions[[v, corner, three]].T, "-", color=GREEN, lw=6, zorder=7, solid_capstyle="round")
    for row, number in ((2, 1), (5, 2), (8, 3), (13, 4)):
        edges, shift, points = tube_loop(ax, mesh, positions, row, BLUE if row == 2 else RED)
        colour = BLUE if shift == 0 else RED
        kind = "a ring" if shift == 0 else "a helix"
        ax.text(-0.5, points[0][1], f"turn {number}", color=colour, ha="right", va="center", fontsize=14)
        ax.text(w + 0.6, points[-1][1], f"shift {shift}: {kind}", color=colour, ha="left", va="center",
                fontsize=15)
        print("turn", number, "row", row, "edges", edges, "shift", shift)
    fig.text(0.41, 0.94, "Each 3/5 pair between two turns adds the length of its staircase (green)\n"
             "to the shift, always the same way: at most one turn can be a ring.",
             ha="center", va="center", fontsize=15, color=INK)
    fig.text(0.41, 0.035, "a limb cut open: the dashed sides are glued", ha="center", va="center", fontsize=13,
             color=MUTED)
    save(fig, "forced_helices.png")


@figure
def winding_separatrix():
    fig = plt.figure(figsize=(12, 5.4))
    # the wrist in 3D
    ax = fig.add_axes([0.0, 0.02, 0.33, 0.86])
    radius, tilt, height = 1.0, 0.3, 3.0
    project = lambda t, z: (radius * np.cos(t), z + tilt * np.sin(t))
    t = np.linspace(-np.pi, 0, 80)
    xs, ys = project(t, 0)
    xt, yt = project(t[::-1], height)
    ax.add_patch(Polygon(np.c_[np.r_[xs, xt], np.r_[ys, yt]], fc="#f3f5f8", ec="none", zorder=1))
    full = np.linspace(-np.pi, np.pi, 160)
    ax.add_patch(Polygon(np.c_[project(full, height)], fc="#fafbfc", ec=INK, lw=1.6, zorder=3))
    ax.plot(*project(t, 0), color=INK, lw=1.6, zorder=6)
    for x in (-1, 1):
        ax.plot([x, x], [0, height], color=INK, lw=1.6, zorder=6)
    start, gap = 1.2, 0.42
    ts = np.linspace(-np.pi / 2, 3 * np.pi / 2, 400)
    zs = start + gap * (ts + np.pi / 2) / (2 * np.pi)
    front = np.sin(ts) < 0
    px, py = project(ts, zs)
    ax.plot(np.where(front, px, np.nan), np.where(front, py, np.nan), "-", color=PURPLE, lw=3.2, zorder=5)
    ax.plot(np.where(~front, px, np.nan), np.where(~front, py, np.nan), ":", color=PURPLE, lw=2.0, alpha=0.5, zorder=2)
    v = project(-np.pi / 2, start)
    q = project(-np.pi / 2, start + gap)
    ax.plot([v[0], v[0]], [0.35, height - 0.35], "-", color=PURPLE, lw=2.0, zorder=4)
    pole_marker(ax, v, 5, size=15)
    ax.plot(*q, "o", ms=8, color=INK, zorder=8)
    label(ax, (0.0, -0.75), "the wrist", size=14, colour=MUTED, box=False)
    ax.set_xlim(-1.6, 1.6)
    ax.set_ylim(-1.0, 3.5)
    bare(ax)

    # the same cut open
    ax = fig.add_axes([0.35, 0.02, 0.64, 0.94])
    width, rows, gap = 10.0, 5.0, 1.0
    v = np.array([3.0, 2.2])
    ax.add_patch(Rectangle((0, 0), width, rows, fc="#f5f7fa", ec="none", zorder=0))
    for x in (0, width):
        ax.plot([x, x], [0, rows], "--", color=MUTED, lw=1.6, zorder=1)
    # the separatrices of the valence-5 pole: one to the right, one up, three short ones
    ax.plot([v[0], width], [v[1], v[1]], "-", color=PURPLE, lw=3.2, zorder=4)
    ax.plot([0, v[0]], [v[1] + gap, v[1] + gap], "-", color=PURPLE, lw=3.2, zorder=4)
    ax.plot([v[0], v[0]], [v[1], rows - 0.2], "-", color=PURPLE, lw=2.2, zorder=3)
    for angle in (165, 238, 305):
        a = np.radians(angle)
        ax.plot([v[0], v[0] + 0.75 * np.cos(a)], [v[1], v[1] + 0.75 * np.sin(a)], "-", color=PURPLE, lw=2.2,
                alpha=0.55, zorder=3)
    pole_marker(ax, v, 5, size=17)
    q = v + (0, gap)
    ax.plot(*q, "o", ms=9, color=INK, zorder=8)
    # the turn round the wrist, just below the separatrix
    y = v[1] - 0.6
    ax.plot([0, width], [y, y], "--", color=RED, lw=2.6, zorder=5)
    ax.plot([0, 1.6], [y + gap, y + gap], "--", color=RED, lw=2.6, zorder=5)
    arrow(ax, (width - 1.2, y), (width - 0.15, y), colour=RED, scale=16, width=2.2)
    ax.annotate("", xy=(v[0] + 0.3, q[1]), xytext=(v[0] + 0.3, v[1]),
                arrowprops=dict(arrowstyle="<|-|>", color=GREEN, lw=2.4, mutation_scale=14))
    ax.text(v[0] + 0.5, v[1] + 0.5 * gap, "gap", color=GREEN, ha="left", va="center", fontsize=14)
    ax.annotate("", xy=(-0.3, y + gap), xytext=(-0.3, y),
                arrowprops=dict(arrowstyle="-|>", color=GREEN, lw=2.4, mutation_scale=14), annotation_clip=False)
    ax.text(-0.5, y + gap / 2, "shift\n= gap", color=GREEN, ha="right", va="center", fontsize=14)
    ax.text(7.0, v[1] + 0.2, "separatrix: leaves the pole\nand goes once round the wrist", color=PURPLE,
            ha="center", va="bottom", fontsize=14)
    ax.text(0.2, rows - 0.25, "it lands on its own\npole's line, just\nbeside the pole", color=PURPLE,
            ha="left", va="top", fontsize=14)
    ax.text(6.4, y - 0.2, "the turn round the wrist comes back\nshifted by the same gap: a helix", color=RED,
            ha="center", va="top", fontsize=14)
    ax.text(width / 2, -0.25, "cut open: the dashed sides are glued", ha="center", va="top", fontsize=13,
            color=MUTED)
    ax.set_xlim(-1.9, width + 0.4)
    ax.set_ylim(-0.9, rows + 0.2)
    bare(ax)
    save(fig, "winding_separatrix.png")


# ----------------------------------------------------------------------------
# Spheres and parity
# ----------------------------------------------------------------------------


def cube_sphere(n):
    """Quads of a cube with n x n quads per side, pushed onto the sphere.
    Returns a list of (corner points on the sphere, corner whole coordinates, side)."""
    quads = []
    for axis in range(3):
        for sign in (-1, 1):
            others = [a for a in range(3) if a != axis]
            for i in range(n):
                for j in range(n):
                    corners = []
                    for di, dj in ((0, 0), (1, 0), (1, 1), (0, 1)):
                        whole = [0, 0, 0]
                        whole[axis] = n if sign > 0 else 0
                        whole[others[0]] = i + di
                        whole[others[1]] = j + dj
                        corners.append(whole)
                    whole = np.array(corners)
                    cube = 2.0 * whole / n - 1.0
                    sphere = cube / np.linalg.norm(cube, axis=1)[:, None]
                    quads.append((sphere, whole, (axis, sign), (i, j)))
    return quads


def view_rotation(tilt=24, turn=-38):
    a, b = np.radians(tilt), np.radians(turn)
    rx = np.array([[1, 0, 0], [0, np.cos(a), -np.sin(a)], [0, np.sin(a), np.cos(a)]])
    ry = np.array([[np.cos(b), 0, np.sin(b)], [0, 1, 0], [-np.sin(b), 0, np.cos(b)]])
    return rx @ ry


def draw_sphere(ax, n, fill, ring=None, dots=False, poles=True):
    """fill(side) gives a colour; ring(side, i, j) says if a quad is on the highlighted ring."""
    rotation = view_rotation()
    light = np.array([-0.4, 0.6, 0.7])
    light /= np.linalg.norm(light)
    drawn = []
    for sphere, whole, side, (i, j) in cube_sphere(n):
        p = sphere @ rotation.T
        centre = p.mean(axis=0)
        if centre[2] <= 0:
            continue
        drawn.append((centre[2], p, whole, side, i, j))
    drawn.sort(key=lambda d: d[0])
    for depth, p, whole, side, i, j in drawn:
        normal = p.mean(axis=0) / np.linalg.norm(p.mean(axis=0))
        shade = 0.82 + 0.18 * max(0.0, float(normal @ light))
        colour = np.array(matplotlib.colors.to_rgb(fill(side)))
        if ring is not None and ring(side, i, j):
            colour = np.array(matplotlib.colors.to_rgb(BLUE))
        colour = np.clip(colour * shade, 0, 1)
        # subdivide the edges so that they follow the sphere
        outline = []
        for k in range(4):
            a, b = p[k], p[(k + 1) % 4]
            for t in np.linspace(0, 1, 6, endpoint=False):
                q = (1 - t) * a + t * b
                outline.append(q / np.linalg.norm(q))
        outline = np.array(outline)
        ax.add_patch(Polygon(outline[:, :2], fc=colour, ec=INK, lw=0.8, zorder=2 + depth))
    seen = {}
    for depth, p, whole, side, i, j in drawn:
        for k in range(4):
            key = tuple(whole[k])
            if p[k][2] > 0.05:
                seen[key] = p[k]
    if dots:
        for key, point in seen.items():
            black = sum(key) % 2 == 0
            ax.plot(*point[:2], "o", ms=7, color=INK if black else "white", mec=INK, mew=1.3, zorder=10)
    if poles:
        for key, point in seen.items():
            if all(c in (0, n) for c in key):
                pole_marker(ax, point[:2], 3, size=18, zorder=12)
    ax.add_patch(Circle((0, 0), 1.0, fc="none", ec=INK, lw=1.6, zorder=11))


@figure
def quadsphere():
    fig, ax = plt.subplots(figsize=(12, 6.2))
    sides = {(0, -1): 0, (0, 1): 1, (1, -1): 2, (1, 1): 3, (2, -1): 4, (2, 1): 5}
    n = 6
    draw_sphere(ax, n, lambda side: PATCH_COLOURS[sides[side]],
                ring=lambda side, i, j: side[0] != 1 and (i if side[0] == 0 else j) == n // 2)
    label(ax, (1.35, 0.75), "8 poles of valence 3:\nthe corners of a cube", colour=ORANGE, ha="left", size=16)
    label(ax, (1.35, 0.0), "6 patches", colour=INK, ha="left", size=16)
    label(ax, (1.35, -0.6), "every loop is a ring\n(one shown in blue)", colour=BLUE, ha="left", size=16)
    ax.set_xlim(-2.6, 3.4)
    ax.set_ylim(-1.15, 1.15)
    bare(ax)
    fig.subplots_adjust(left=0, right=1, top=1, bottom=0)
    save(fig, "quadsphere.png")


@figure
def parity():
    fig, (left, right) = plt.subplots(1, 2, figsize=(12, 5.9))
    n = 4
    draw_sphere(left, n, lambda side: "#f1f4f8",
                ring=lambda side, i, j: side[0] != 1 and (i if side[0] == 0 else j) == n // 2,
                dots=True, poles=False)
    left.set_xlim(-1.25, 1.25)
    left.set_ylim(-1.3, 1.25)
    bare(left)
    left.set_title("Closed surface: the corners take two colours,\nneighbours always different, "
                   "so a ring has\nan even number of quads (here 16)", fontsize=14)
    # a ring of five quads round a hole
    angles = np.pi / 2 + 2 * np.pi * np.arange(5) / 5
    inner = np.c_[np.cos(angles), np.sin(angles)]
    outer = 2.3 * inner
    right.add_patch(Polygon(inner, fc="#5b6470", ec=INK, lw=2, zorder=1))
    right.text(0, 0, "hole\n(an eye)", ha="center", va="center", fontsize=13, color="white", zorder=2)
    for k in range(5):
        quad = [inner[k], inner[(k + 1) % 5], outer[(k + 1) % 5], outer[k]]
        right.add_patch(Polygon(quad, fc=BLUE_LIGHT, ec=INK, lw=2, zorder=1))
    colours = ["black", "white", "black", "white", "black"]
    for k in range(5):
        for points, flip in ((inner, False), (outer, True)):
            black = (colours[k] == "black") != flip
            right.plot(*points[k], "o", ms=13, color=INK if black else "white", mec=INK, mew=1.6, zorder=5)
    for points in (inner, outer):
        right.plot(*points[[4, 0]].T, "-", color=RED, lw=5, zorder=4)
    label(right, (0.0, 2.75), "two neighbours of the same colour", colour=RED, size=14)
    right.set_xlim(-3.2, 3.2)
    right.set_ylim(-2.4, 3.1)
    bare(right)
    right.set_title("With a hole, the colours cannot alternate:\na ring of 5 quads is possible",
                    fontsize=14)
    fig.subplots_adjust(left=0.01, right=0.99, top=0.84, bottom=0.01, wspace=0.05)
    save(fig, "parity.png")


@figure
def checkerboard():
    fig, axes = plt.subplots(1, 3, figsize=(12, 4.9))

    def lattice(ax, xs, ys):
        for x in xs:
            for y in ys:
                black = (x + y) % 2 == 0
                ax.plot(x, y, "o", ms=10, color=INK if black else "white", mec=INK, mew=1.4, zorder=3)

    def pole_on_point(ax, point, valence):
        black = (point[0] + point[1]) % 2 == 0
        ax.plot(*point, "o", ms=30, color=INK if black else "white", mec=INK, mew=1.6, zorder=4)
        pole_marker(ax, point, valence, size=16, zorder=6)

    ax = axes[0]
    lattice(ax, range(-3, 4), range(-2, 4))
    centre, q = np.array([0, 0]), np.array([2, 1])
    turned = centre + np.array([-(q - centre)[1], (q - centre)[0]])
    pole_on_point(ax, centre, 3)
    for point in (q, turned):
        ax.plot(*point, "o", ms=17, color="white", mec=GREEN, mew=3, zorder=5)
    arrow(ax, q + (0.05, 0.25), turned + (0.25, 0.1), colour=GREEN, rad=0.45, width=2.4, scale=18)
    ax.set_title("A quarter turn about a whole\npoint keeps the colours", fontsize=14)

    for ax, partner, verdict, colour in (
        (axes[1], (1, 0), "1 on a white point: odd\nso the two turns cannot\nboth be rings", RED),
        (axes[2], (1, 1), "0 on a white point: even\nso this rule does not\nforbid two rings", BLUE),
    ):
        lattice(ax, range(-3, 4), range(-2, 4))
        pole_on_point(ax, (0, 0), 3)
        pole_on_point(ax, partner, 5)
        for y, name in ((-1.5, "turn 1"), (2.5, "turn 2")):
            ax.plot([-3.4, 3.4], [y, y], "--", color=MUTED, lw=2, zorder=2)
            ax.text(3.45, y, name, ha="left", va="center", fontsize=12, color=MUTED)
        label(ax, (0.0, -3.0), verdict, colour=colour, size=13, va="top", box=False)
    axes[1].set_title("A 3/5 pair side by side:\ncount the poles on white points", fontsize=14)
    axes[2].set_title("A 3/5 pair across a quad:\ncount the poles on white points", fontsize=14)
    for ax in axes:
        ax.set_xlim(-3.6, 4.6)
        ax.set_ylim(-5.0, 3.6)
        bare(ax)
    label(axes[0], (0.0, -3.0), "the pole sits on a whole point;\nthe green point and its turned\ncopy have the same colour",
          colour=GREEN, size=13, va="top", box=False)
    fig.subplots_adjust(left=0.01, right=0.99, top=0.86, bottom=0.01, wspace=0.08)
    save(fig, "checkerboard.png")


# ----------------------------------------------------------------------------
# 8. Charts of the popular summary of the research note
# ----------------------------------------------------------------------------


@figure
def forced_vs_engine():
    """Turns held for closing on the male body at 10,590 quads, per way: those the
    singularities force to stay helices (note, table "forced helices", bounds of
    separation) against those the engine leaves shifted."""
    ways = list(range(8))
    held = [48, 78, 52, 86, 84, 160, 88, 169]
    forced = [1, 5, 1, 4, 1, 6, 1, 9]
    forced_text = ["1", "5", "1", "4", "1", "6", "1", "8-9"]
    engine = [13, 39, 16, 44, 49, 115, 44, 123]
    fig, ax = plt.subplots(figsize=(12, 6.2))
    height = 0.36
    for k, way in enumerate(ways):
        y = len(ways) - 1 - k
        ax.barh(y, held[k], height=0.86, color=LIGHT, edgecolor="none", zorder=1)
        ax.barh(y + height / 2 + 0.02, engine[k], height=height, color=RED_LIGHT, edgecolor=RED, lw=1.0, zorder=2)
        ax.barh(y - height / 2 - 0.02, forced[k], height=height, color=RED, edgecolor="none", zorder=2)
        ax.text(engine[k] + 1.5, y + height / 2 + 0.02, str(engine[k]), va="center", fontsize=12, color=INK)
        ax.text(forced[k] + 1.5, y - height / 2 - 0.02, forced_text[k], va="center", fontsize=12, color=INK,
                fontweight="bold")
        ax.text(held[k] + 1.5, y, f"of {held[k]} held", va="center", fontsize=11, color=MUTED)
    ax.set_yticks(range(len(ways)))
    ax.set_yticklabels([f"way {w}" for w in reversed(ways)], fontsize=13)
    ax.set_xlim(0, 190)
    ax.set_xlabel("number of turns", fontsize=13, color=MUTED)
    for side in ("top", "right", "left"):
        ax.spines[side].set_visible(False)
    ax.spines["bottom"].set_color(GRID)
    ax.tick_params(axis="x", colors=MUTED, labelsize=12)
    ax.tick_params(axis="y", length=0)
    handles = [
        Rectangle((0, 0), 1, 1, color=RED),
        Rectangle((0, 0), 1, 1, facecolor=RED_LIGHT, edgecolor=RED),
        Rectangle((0, 0), 1, 1, color=LIGHT),
    ]
    ax.legend(handles, ["helices forced by the singularities", "helices left by the engine",
                        "turns held for closing"], loc="upper right", fontsize=12, frameon=False)
    ax.set_title("Male body, 10,590 quads: forced helices against those the engine leaves",
                 fontsize=15, pad=12)
    fig.subplots_adjust(left=0.09, right=0.98, top=0.91, bottom=0.10)
    save(fig, "forced_vs_engine.png")


@figure
def spiral_results():
    """Share of edges on spiral loops: the 12-model batch over three budgets
    (PR #69) and the male body (note and PR #69)."""
    fig, axes = plt.subplots(1, 2, figsize=(12, 4.6), gridspec_kw={"width_ratios": [1, 1.25]})

    def bars(ax, names, values, colours, texts, title, top):
        x = np.arange(len(names))
        ax.bar(x, values, width=0.58, color=colours, edgecolor="none", zorder=2)
        for k, value in enumerate(values):
            ax.text(x[k], value + top * 0.02, texts[k], ha="center", va="bottom", fontsize=14,
                    fontweight="bold", color=INK)
        ax.set_xticks(x)
        ax.set_xticklabels(names, fontsize=12)
        ax.set_ylim(0, top)
        ax.set_title(title, fontsize=14, pad=10)
        for side in ("top", "right", "left"):
            ax.spines[side].set_visible(False)
        ax.spines["bottom"].set_color(GRID)
        ax.set_yticks([])
        ax.tick_params(axis="x", length=0)

    bars(axes[0], ["first version\nof closing", "today"], [7.55, 4.54], [RED_LIGHT, BLUE],
         ["7.55%", "4.54%"], "12 models, 2,970 / 3,000 / 3,030 quads", 9.0)
    bars(axes[1], ["first version\nof closing", "today", "made by hand"], [2.47, 1.59, 2.5],
         [RED_LIGHT, BLUE, GRID], ["2.47%", "1.15-1.59%", "2.5%"], "Male body, 10,590 quads", 3.0)
    fig.suptitle("Share of edges on spiral loops (lower is better)", fontsize=15, y=0.99)
    fig.subplots_adjust(left=0.02, right=0.99, top=0.80, bottom=0.16, wspace=0.12)
    save(fig, "spiral_results.png")


# ----------------------------------------------------------------------------
# Main
# ----------------------------------------------------------------------------

if __name__ == "__main__":
    names = sys.argv[1:] or list(FIGURES)
    for name in names:
        FIGURES[name]()
