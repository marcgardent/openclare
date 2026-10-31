#!/usr/bin/env python3
"""The pictures of the use cards of the home page, as SVG drawn from code.

    python3 scripts/use_art.py

Writes site/assets/uses/<use>.svg, 480 x 240, in the site's colours through
its CSS variables, like the banners of the articles
(scripts/article_banners.py), whose drawing helpers it borrows. site/build.py
puts each inline in the card <div class="card" data-art="<use>">. They are
decorative, hidden from screen readers.
"""

import math
import random
from pathlib import Path

from article_banners import style

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "site/assets/uses"
W, H = 480, 240


def svg(body):
    grid = "".join(f'<path d="M{x} 0V{H}"/>' for x in range(0, W + 1, 24)) + "".join(f'<path d="M0 {y}H{W}"/>' for y in range(0, H + 1, 24))
    return (f'<svg class="use-art" viewBox="0 0 {W} {H}" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false">'
            f'<g style="stroke: var(--grid); stroke-width: 1">{grid}</g>{body}</svg>\n')


def path(points, closed=False):
    return "M" + "L".join(f"{x:.1f} {y:.1f}" for x, y in points) + ("Z" if closed else "")


def sculpt():
    """A mouth: a grid of quads laid across it, blind to it (left), and loops
    that go round it, closed, the way an animator wants them (right)."""
    def contour(cx, cy, s, k):
        # The outline of the lips, grown by s: the upper lip with its bow,
        # the lower one fuller. k runs round it, 0 to 1.
        a = math.tau * k
        x = math.cos(a)
        if math.sin(a) >= 0:
            y = -16 * math.sin(a) * (1 - 0.45 * math.exp(-(x / 0.22) ** 2))
        else:
            y = -20 * math.sin(a)
        grow = 1 + 0.55 * (s - 1)
        return cx + 50 * s * x, cy + y * s * grow

    def mouth(cx, cy, colour):
        outline = path([contour(cx, cy, 1, k / 96) for k in range(96)], True)
        line = path([(cx + 50 * x, cy + 3 - 5 * (1 - x * x)) for x in (j / 20 - 1 for j in range(41))])
        return f'<path style="{style(colour, 3.2)}" d="{outline} {line}"/>'

    cx, cy = 116, 122
    lines = [path([(cx - 96 + 16 * i, 40), (cx - 96 + 16 * i, 204)]) for i in range(13)]
    lines += [path([(cx - 96, 40 + 16.4 * j), (cx + 96, 40 + 16.4 * j)]) for j in range(11)]
    b = f'<path style="{style("--slate", 1.6)}" d="{" ".join(lines)}"/>'
    b += mouth(cx, cy, "--ink")
    b += f'<path style="{style("--ink", 3)}" d="M222 122H252M238 110L252 122L238 134"/>'
    # Right: rings round the mouth, joined by spokes into quads.
    cx = 368
    rings = (1.0, 1.35, 1.7, 2.0)
    b += "".join(f'<path style="{style("--teal", 2.4)}" d="{path([contour(cx, cy, s, k / 96) for k in range(96)], True)}"/>' for s in rings[1:])
    spokes = [path([contour(cx, cy, rings[0], k / 16), contour(cx, cy, rings[-1], k / 16)]) for k in range(16)]
    b += f'<path style="{style("--teal", 1.8)}" d="{" ".join(spokes)}"/>'
    b += mouth(cx, cy, "--ink")
    return svg(b)


def loops():
    """A bent arm: the loops of an automatic remesh wind round it as a spiral
    (left), openclare's close into rings, closer together at the elbow
    (right)."""
    def frame(x0, t):
        # Where the arm is at t (shoulder 0, wrist 1), its direction, its
        # normal and its radius: a smooth bend round the elbow.
        p0, p1, p2 = (x0 + 20, 62), (x0 + 70, 212), (x0 + 190, 122)
        x = (1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * p1[0] + t * t * p2[0]
        y = (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * p1[1] + t * t * p2[1]
        dx = 2 * (1 - t) * (p1[0] - p0[0]) + 2 * t * (p2[0] - p1[0])
        dy = 2 * (1 - t) * (p1[1] - p0[1]) + 2 * t * (p2[1] - p1[1])
        n = math.hypot(dx, dy)
        return x, y, dx / n, dy / n, 24 - 8 * t

    def around(x0, t, phase):
        # A point on the arm's surface, seen a little from the side.
        x, y, dx, dy, r = frame(x0, t)
        return x - dy * r * math.cos(phase) + dx * 0.3 * r * math.sin(phase), y + dx * r * math.cos(phase) + dy * 0.3 * r * math.sin(phase)

    def outline(x0):
        ts = [k / 60 for k in range(61)]
        return path([around(x0, t, 0) for t in ts] + [around(x0, t, math.pi) for t in reversed(ts)], True)

    b = ""
    for x0 in (14, 250):
        b += f'<path style="{style("--ink", 2.5, "--surface")}" d="{outline(x0)}"/>'
    # Left: one line winding from the shoulder to the wrist.
    turns = 9
    helix = [around(14, k / 900, k / 900 * turns * math.tau) for k in range(901)]
    b += f'<path style="{style("--hot", 3)}" d="{path(helix)}"/>'
    b += f'<path style="{style("--ink", 3)}" d="M222 130H252M238 118L252 130L238 142"/>'
    # Right: closed rings, closer together where the arm bends.
    for t in (0.04, 0.16, 0.27, 0.36, 0.43, 0.49, 0.55, 0.62, 0.71, 0.82, 0.94):
        b += f'<path style="{style("--teal", 3)}" d="{path([around(250, t, k / 40 * math.tau) for k in range(40)], True)}"/>'
    return svg(b)


def mechanical():
    """A block with a chamfer: the sharp edges kept as lines of edges, in vermillion."""
    def iso(x, y, z):
        return 240 + (x - y) * 0.87, 128 + (x + y) * 0.5 - z
    s = 70
    top = [iso(-s, -s, s), iso(s, -s, s), iso(s, s, s), iso(-s, s, s)]
    b = ""
    # faces with quad lines
    faces = [([iso(-s, s, -s / 2), iso(s, s, -s / 2), iso(s, s, s), iso(-s, s, s)], "--surface"),
             ([iso(s, -s, -s / 2), iso(s, s, -s / 2), iso(s, s, s), iso(s, -s, s)], "--tint"),
             (top, "--surface")]
    for pts, fill in faces:
        b += f'<path style="{style("--ink", 2, fill)}" d="{path(pts, True)}"/>'
    lines = []
    for t in range(1, 6):
        u = -s + 2 * s * t / 6
        lines += [path([iso(u, -s, s), iso(u, s, s)]), path([iso(-s, u, s), iso(s, u, s)])]
        lines.append(path([iso(u, s, -s / 2), iso(u, s, s)]))
        lines.append(path([iso(s, u, -s / 2), iso(s, u, s)]))
    for t in range(1, 3):
        z = -s / 2 + 1.5 * s * t / 3
        lines += [path([iso(-s, s, z), iso(s, s, z)]), path([iso(s, -s, z), iso(s, s, z)])]
    b += f'<path style="{style("--slate", 1.5)}" d="{" ".join(lines)}"/>'
    sharp = [path([iso(-s, s, s), iso(s, s, s), iso(s, -s, s)]), path([iso(s, s, s), iso(s, s, -s / 2)])]
    b += f'<path style="{style("--hot", 5)}" d="{" ".join(sharp)}"/>'
    return svg(b)


def scan():
    """A noisy triangle soup on the left, a clean patch of quads on the right."""
    rng = random.Random(3)
    b = ""
    pts = {(i, j): (40 + i * 30 + rng.uniform(-9, 9), 50 + j * 30 + rng.uniform(-9, 9)) for i in range(6) for j in range(6)}
    tris = []
    for i in range(5):
        for j in range(5):
            a, c, d, e = pts[i, j], pts[i + 1, j], pts[i + 1, j + 1], pts[i, j + 1]
            tris += [path([a, c, d], True), path([a, d, e], True)] if rng.random() < 0.5 else [path([a, c, e], True), path([c, d, e], True)]
    b += f'<path style="{style("--slate", 1.8)}" d="{" ".join(tris)}"/>'
    b += f'<path style="{style("--ink", 3)}" d="M222 120H262M248 108L262 120L248 132"/>'
    q = []
    for k in range(6):
        q.append(path([(290 + k * 30, 50), (290 + k * 30, 200)]))
        q.append(path([(290, 50 + k * 30), (440, 50 + k * 30)]))
    b += f'<path style="{style("--teal", 2.5)}" d="{" ".join(q)}"/>'
    return svg(b)


def detail():
    """Smaller quads where the detail is: a grid that tightens towards one corner."""
    def f(t):
        return t ** 1.8
    xs = [60 + 360 * f(i / 14) for i in range(15)]
    ys = [30 + 180 * f(j / 7) for j in range(8)]
    lines = [path([(x, ys[0]), (x, ys[-1])]) for x in xs] + [path([(xs[0], y), (xs[-1], y)]) for y in ys]
    b = f'<path style="{style("--ink", 1.8)}" d="{" ".join(lines)}"/>'
    b += f'<circle cx="70" cy="40" r="34" style="{style("--hot", 3)}; stroke-dasharray: 6 6"/>'
    return svg(b)


def seams():
    """A seam drawn once on the dense source (left), found again by two
    remeshes of different sizes, as a line of their edges (right)."""
    def curve(t):
        # The seam, from top to bottom of a panel.
        return 18 * math.sin(t * math.pi * 1.2)

    def panel(x0, w, cols, rows, colour, width, seam_at=None):
        # A patch whose vertical lines bend like the seam.
        lines = [path([(x0 + w * i / cols + curve(j / 40), 40 + 160 * j / 40) for j in range(41)]) for i in range(cols + 1)]
        lines += [path([(x0 + w * i / cols + curve(j / rows), 40 + 160 * j / rows) for i in range(cols + 1)]) for j in range(rows + 1)]
        out = f'<path style="{style(colour, width)}" d="{" ".join(lines)}"/>'
        if seam_at is not None:
            out += f'<path style="{style("--hot", 5)}" d="{path([(x0 + w * seam_at + curve(j / 40), 40 + 160 * j / 40) for j in range(41)])}"/>'
        return out

    b = panel(24, 140, 28, 32, "--slate", 0.8)
    b += f'<path style="{style("--hot", 5)}" d="{path([(24 + 70 + curve(j / 40), 40 + 160 * j / 40) for j in range(41)])}"/>'
    b += f'<path style="{style("--ink", 3)}" d="M190 120H226M212 108L226 120L212 132"/>'
    b += panel(244, 96, 4, 6, "--teal", 2, 0.5)
    b += panel(360, 96, 8, 12, "--teal", 1.6, 0.5)
    return svg(b)


def strip(lower, upper):
    """Triangles between two rows of points, each ordered by x."""
    out, i, j = [], 0, 0
    while i < len(lower) - 1 or j < len(upper) - 1:
        if j == len(upper) - 1 or (i < len(lower) - 1 and lower[i + 1][0] <= upper[j + 1][0]):
            out.append((lower[i], lower[i + 1], upper[j]))
            i += 1
        else:
            out.append((lower[i], upper[j + 1], upper[j]))
            j += 1
    return out


def triangles():
    """The same cloth hanging from a rail and swaying: uneven triangles can
    bend it only at their few vertices, in kinks (left), even ones in a
    smooth wave (right)."""
    W, H = 180, 160

    def drape(x, y, x0):
        t = (y - 40) / H
        u = (x - x0) / W
        return x, y + 14 * t * math.sin(u * 1.5 * math.tau)

    def sheet(rows, x0):
        rows = [[(x0 + x, 40 + y) for x, y in row] for row in rows]
        tris = [t for a, b in zip(rows, rows[1:]) for t in strip(a, b)]
        return [tuple(drape(x, y, x0) for x, y in t) for t in tris]

    rng = random.Random(3)
    # Left: few vertices, unevenly spread.
    coarse = []
    for j, y in enumerate((0, 50, 85, 160)):
        inner = sorted(rng.uniform(15, W - 15) for _ in range((2, 3, 1, 2)[j]))
        coarse.append([(0, y)] + [(x, y + (rng.uniform(-10, 10) if 0 < j < 3 else 0)) for x in inner] + [(W, y)])
    out = f'<path style="{style("--ink", 1.8)}" d="{" ".join(path(t, True) for t in sheet(coarse, 30))}"/>'
    out += f'<path style="{style("--ink", 3)}" d="M226 120H262M248 108L262 120L248 132"/>'
    # Right: even triangles, rows offset by half a side, cut straight at the sides.
    side = 20
    h = side * math.sqrt(3) / 2
    even = []
    for j in range(round(H / h) + 1):
        y, offset = min(j * h, H), (j % 2) * side / 2
        xs = [0.0] + [offset + k * side for k in range(10) if 0 < offset + k * side < W] + [float(W)]
        even.append([(x, y) for x in xs])
    out += f'<path style="{style("--teal", 1.8)}" d="{" ".join(path(t, True) for t in sheet(even, 276))}"/>'
    out += f'<path style="{style("--hot", 4)}" d="M22 40H218M268 40H464"/>'
    return svg(out)


def paint():
    """The same stroke painted on vertex colours: a jagged blot on a sculpt's
    uneven triangles (left), a clean band on quads of one size (right). The
    stroke asked for is the dashed curve."""
    def wave(x, x0):
        return 120 + 36 * math.sin((x - x0) / 32)

    def painted(points, x0):
        cx, cy = sum(x for x, _ in points) / len(points), sum(y for _, y in points) / len(points)
        return abs(cy - wave(cx, x0)) < 16

    rng = random.Random(7)
    # Left: columns narrow at one end and wide at the other, jittered, each
    # cell cut in two: the uneven triangles of a sculpt.
    cols = [30 + 180 * (i / 5) ** 2.2 for i in range(6)]
    rows = [40 + 160 * j / 4 for j in range(5)]
    grid = [[(x + (rng.uniform(-5, 5) if 0 < i < 5 else 0), y + (rng.uniform(-10, 10) if 0 < j < 4 else 0))
             for j, y in enumerate(rows)] for i, x in enumerate(cols)]
    tris = []
    for i in range(5):
        for j in range(4):
            a, b, c, d = grid[i][j], grid[i + 1][j], grid[i + 1][j + 1], grid[i][j + 1]
            tris += [(a, b, c), (a, c, d)] if (i + j) % 2 else [(a, b, d), (b, c, d)]
    out = "".join(f'<path style="{style(None, 0, "--hot", 0.85)}" d="{path(t, True)}"/>' for t in tris if painted(t, 30))
    out += f'<path style="{style("--ink", 1.5)}" d="{" ".join(path(t, True) for t in tris)}"/>'
    out += f'<path style="{style("--ink", 2.5)}; stroke-dasharray: 6 6" d="{path([(x, wave(x, 30)) for x in range(30, 211, 4)])}"/>'
    out += f'<path style="{style("--ink", 3)}" d="M226 120H262M248 108L262 120L248 132"/>'
    # Right: quads of one size.
    xs, ys = [276 + 12 * i for i in range(16)], [40 + 12 * j for j in range(14)]
    quads = [[(xs[i], ys[j]), (xs[i + 1], ys[j]), (xs[i + 1], ys[j + 1]), (xs[i], ys[j + 1])] for i in range(15) for j in range(13)]
    out += "".join(f'<path style="{style(None, 0, "--hot", 0.85)}" d="{path(q, True)}"/>' for q in quads if painted(q, 276))
    out += f'<path style="{style("--teal", 1.5)}" d="{" ".join(path(q, True) for q in quads)}"/>'
    out += f'<path style="{style("--ink", 2.5)}; stroke-dasharray: 6 6" d="{path([(x, wave(x, 276)) for x in range(276, 457, 4)])}"/>'
    return svg(out)


USES = {"sculpt": sculpt, "loops": loops, "mechanical": mechanical, "scan": scan, "detail": detail, "seams": seams, "triangles": triangles, "paint": paint}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, draw in USES.items():
        (OUT / f"{name}.svg").write_text(draw())
        print(f"written: site/assets/uses/{name}.svg")


if __name__ == "__main__":
    main()
