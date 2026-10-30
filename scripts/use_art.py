#!/usr/bin/env python3
"""The pictures of the use cards of the home page, as SVG drawn from code.

    python3 scripts/use_art.py

Writes site/assets/uses/<use>.svg, 480 x 240, in the site's colours through
its CSS variables, like the banners of the articles
(scripts/article_banners.py), whose drawing helpers it borrows. site/build.py
puts each inline in the card <div class="card" data-art="<use>">. They are
decorative, hidden from screen readers.
"""

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
    """A head: loops round the eyes and the mouth, closed."""
    cx, cy = 240, 124
    b = f'<ellipse cx="{cx}" cy="{cy}" rx="78" ry="98" style="{style("--ink", 3, "--surface")}"/>'
    # rings round the face, closed
    for rx, ry in [(62, 80), (44, 60)]:
        b += f'<ellipse cx="{cx}" cy="{cy + 6}" rx="{rx}" ry="{ry}" style="{style("--slate", 2)}"/>'
    for ex in (cx - 28, cx + 28):
        for r in (18, 11):
            b += f'<ellipse cx="{ex}" cy="{cy - 14}" rx="{r}" ry="{r * 0.7:.1f}" style="{style("--teal", 3)}"/>'
    for r in (26, 16):
        b += f'<ellipse cx="{cx}" cy="{cy + 44}" rx="{r}" ry="{r * 0.45:.1f}" style="{style("--teal", 3)}"/>'
    b += f'<path style="{style("--slate", 2)}" d="M{cx} {cy - 26}V{cy + 26}"/>'
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
    """A curve drawn on a patch becomes a seam, and the patch unfolds along it."""
    b = ""
    q = []
    for k in range(7):
        q.append(path([(40 + k * 30, 40), (40 + k * 30, 200)]))
    for k in range(6):
        q.append(path([(40, 40 + k * 32), (220, 40 + k * 32)]))
    b += f'<path style="{style("--slate", 2)}" d="{" ".join(q)}"/>'
    b += f'<path style="{style("--hot", 5)}" d="M130 40V200"/>'
    b += f'<path style="{style("--ink", 3)}" d="M236 120H276M262 108L276 120L262 132"/>'
    for dx in (300, 380):
        q = [path([(dx + k * 15, 40), (dx + k * 15, 200)]) for k in range(5)] + [path([(dx, 40 + k * 32), (dx + 60, 40 + k * 32)]) for k in range(6)]
        b += f'<path style="{style("--teal", 2)}" d="{" ".join(q)}"/>'
    b += f'<path style="{style("--hot", 5)}" d="M360 40V200M380 40V200"/>'
    return svg(b)


def triangles():
    """Even triangles for cloth (left) and fewer, larger ones for printing (right)."""
    t = []
    for j in range(5):
        for i in range(7):
            x, y = 30 + i * 28 + (j % 2) * 14, 50 + j * 28
            t.append(path([(x, y), (x + 28, y), (x + 14, y + 28)], True))
    b = f'<path style="{style("--teal", 2)}" d="{" ".join(t)}"/>'
    pts = [(270, 50), (330, 60), (420, 48), (450, 120), (430, 190), (350, 190), (280, 185), (300, 120), (370, 115)]
    tris = [(0, 1, 7), (1, 8, 7), (1, 2, 8), (2, 3, 8), (3, 4, 8), (4, 5, 8), (5, 6, 7), (5, 7, 8), (6, 7, 0)]
    b += f'<path style="{style("--ink", 2, "--surface")}" d="{" ".join(path([pts[a], pts[c], pts[d]], True) for a, c, d in tris)}"/>'
    return svg(b)


USES = {"sculpt": sculpt, "mechanical": mechanical, "scan": scan, "detail": detail, "seams": seams, "triangles": triangles}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, draw in USES.items():
        (OUT / f"{name}.svg").write_text(draw())
        print(f"written: site/assets/uses/{name}.svg")


if __name__ == "__main__":
    main()
