#!/usr/bin/env python3
"""The banners of the website's articles, as SVG drawn from code.

    python3 scripts/article_banners.py

Writes site/assets/articles/<article>.svg, 1200 x 360. The pictures are drawn
in the site's colours through its CSS variables (--ink, --teal, --hot, ...),
so site/build.py puts them inline in the pages: they follow the light and
the dark theme. They are decorative, hidden from screen readers.
"""

import math
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "site/assets/articles"
W, H = 1200, 360


def svg(body):
    grid = "".join(f'<path d="M{x} 0V{H}"/>' for x in range(0, W + 1, 40))
    grid += "".join(f'<path d="M0 {y}H{W}"/>' for y in range(0, H + 1, 40))
    return (
        f'<svg class="banner" viewBox="0 0 {W} {H}" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false" '
        f'preserveAspectRatio="xMidYMid slice">'
        f'<g style="stroke: var(--grid); stroke-width: 1">{grid}</g>{body}</svg>\n'
    )


def style(stroke="--ink", width=2.5, fill="none", opacity=None):
    parts = [f"stroke: var({stroke})" if stroke else "stroke: none", f"stroke-width: {width}",
             f"fill: var({fill})" if fill != "none" else "fill: none", "stroke-linecap: round", "stroke-linejoin: round"]
    if opacity is not None:
        parts.append(f"fill-opacity: {opacity}")
    return "; ".join(parts)


def warped_grid(x0, y0, cols, rows, size, warp, stroke="--ink", width=2, accent=None):
    """A patch of quads, gently bent: the motif of the site's logo."""
    def point(i, j):
        x = x0 + i * size + warp * math.sin(j * 0.7 + i * 0.3)
        y = y0 + j * size + warp * math.sin(i * 0.6 - j * 0.2)
        return x, y

    paths = []
    for j in range(rows + 1):
        paths.append("M" + "L".join(f"{x:.1f} {y:.1f}" for x, y in (point(i, j) for i in range(cols + 1))))
    for i in range(cols + 1):
        paths.append("M" + "L".join(f"{x:.1f} {y:.1f}" for x, y in (point(i, j) for j in range(rows + 1))))
    out = ""
    if accent:
        i, j = accent
        corners = [point(i, j), point(i + 1, j), point(i + 1, j + 1), point(i, j + 1)]
        out += f'<path style="{style(None, 0, "--hot")}" d="M' + "L".join(f"{x:.1f} {y:.1f}" for x, y in corners) + 'Z"/>'
    out += f'<path style="{style(stroke, width)}" d="{" ".join(paths)}"/>'
    return out


def theory():
    """Loops that close: along one tube, a spiral (left) becomes closed rings (right)."""
    cx0, cx1, cy, ry, rx = 150, 1050, 180, 105, 24
    split = 610
    body = f'<path style="{style("--ink", 3)}" d="M{cx0} {cy - ry}H{cx1}M{cx0} {cy + ry}H{cx1}"/>'
    # The spiral, three turns on the left half: the front solid, the back faint.
    turns, x0, x1 = 3, cx0 + 20, split - 40
    front, back = [], []
    for k in range(361):
        t = k / 360 * turns * 2 * math.pi
        x = x0 + (x1 - x0) * k / 360 + rx * math.sin(t)
        y = cy - ry * math.cos(t)
        (front if math.cos(t) * 0 + math.sin(t) >= 0 else back).append((k, x, y))

    def runs(points):
        segments, last = [], None
        for k, x, y in points:
            if last is None or k != last + 1:
                segments.append([])
            segments[-1].append((x, y))
            last = k
        return " ".join("M" + "L".join(f"{x:.1f} {y:.1f}" for x, y in s) for s in segments if len(s) > 1)

    body += f'<path style="{style("--hot", 3)}; stroke-dasharray: 2 10; opacity: 0.85" d="{runs(back)}"/>'
    body += f'<path style="{style("--hot", 6)}" d="{runs(front)}"/>'
    # Closed rings on the right half.
    for x in range(split + 50, cx1 - 30, 85):
        body += f'<ellipse cx="{x}" cy="{cy}" rx="{rx}" ry="{ry}" style="{style("--teal", 5)}"/>'
    body += f'<path style="{style("--ink", 3)}" d="M{split - 22} {cy}H{split + 8}M{split - 4} {cy - 12}L{split + 8} {cy}L{split - 4} {cy + 12}"/>'
    body += f'<ellipse cx="{cx0}" cy="{cy}" rx="{rx}" ry="{ry}" style="{style("--ink", 3)}"/>'
    body += f'<ellipse cx="{cx1}" cy="{cy}" rx="{rx}" ry="{ry}" style="{style("--ink", 3, "--surface")}"/>'
    return svg(body)


def results():
    """The bench: bars of marks beside a patch of quads."""
    body = ""
    base, x = 300, 120
    groups = [(82, 41, 33), (86, 45, 52), (90, 44, 60)]
    for group in groups:
        for value, colour in zip(group, ("--teal", "--other", "--rule")):
            h = value * 2.3
            body += f'<rect x="{x}" y="{base - h:.0f}" width="38" height="{h:.0f}" style="{style("--ink", 2, colour)}"/>'
            x += 46
        x += 46
    body += f'<path style="{style("--ink", 3)}" d="M100 {base}H{x - 30}"/>'
    body += f'<path style="{style("--hot", 3)}; stroke-dasharray: 8 8" d="M100 {base - 200}H{x - 30}"/>'
    body += warped_grid(720, 70, 9, 5, 44, 7, accent=(5, 2))
    return svg(body)


def research():
    """The papers and their proofs: a page of text beside a tree of steps that ends in a check."""
    body = f'<rect x="150" y="40" width="260" height="290" style="{style("--ink", 3, "--surface")}"/>'
    body += f'<path style="{style("--ink", 3)}" d="M180 80H360"/>'
    lines = " ".join(f"M180 {y}H{380 if (y // 22) % 4 else 320}" for y in range(120, 300, 22))
    body += f'<path style="{style("--slate", 3)}" d="{lines}"/>'
    body += f'<rect x="180" y="210" width="90" height="56" style="{style("--teal", 2.5)}"/>'
    nodes = [(560, 90), (560, 270), (720, 60), (720, 180), (720, 300), (880, 120), (880, 240), (1040, 180)]
    edges = [(0, 2), (0, 3), (1, 3), (1, 4), (2, 5), (3, 5), (3, 6), (4, 6), (5, 7), (6, 7)]
    body += f'<path style="{style("--ink", 3)}" d="' + " ".join(f"M{nodes[a][0]} {nodes[a][1]}L{nodes[b][0]} {nodes[b][1]}" for a, b in edges) + '"/>'
    for k, (x, y) in enumerate(nodes[:-1]):
        body += f'<circle cx="{x}" cy="{y}" r="14" style="{style("--ink", 3, "--surface")}"/>'
    x, y = nodes[-1]
    body += f'<circle cx="{x}" cy="{y}" r="34" style="{style("--teal", 4, "--surface")}"/>'
    body += f'<path style="{style("--teal", 7)}" d="M{x - 15} {y}L{x - 4} {y + 12}L{x + 17} {y - 13}"/>'
    return svg(body)


def retopology():
    """The tools: a shelf of tiles, one of them the quad patch."""
    body = ""
    for row in range(2):
        for col in range(6):
            x, y = 130 + col * 160, 50 + row * 150
            ours = (row, col) == (0, 2)
            body += f'<rect x="{x}" y="{y}" width="120" height="120" rx="10" style="{style("--teal" if ours else "--ink", 4 if ours else 2.5, "--surface")}"/>'
            kind = (row * 6 + col) % 4
            if ours:
                body += warped_grid(x + 20, y + 20, 4, 4, 20, 2.5, accent=(2, 1))
            elif kind == 0:
                body += f'<path style="{style("--slate", 3)}" d="M{x+20} {y+100}L{x+60} {y+20}L{x+100} {y+100}ZM{x+40} {y+60}H{x+80}"/>'
            elif kind == 1:
                body += f'<path style="{style("--slate", 3)}" d="M{x+20} {y+90}C{x+40} {y+20} {x+80} {y+20} {x+100} {y+60}"/>'
                body += f'<circle cx="{x+100}" cy="{y+60}" r="6" style="{style("--hot", 2, "--hot")}"/>'
            elif kind == 2:
                body += f'<circle cx="{x+60}" cy="{y+60}" r="36" style="{style("--slate", 3)}"/><path style="{style("--slate", 3)}" d="M{x+24} {y+60}H{x+96}M{x+60} {y+24}V{y+96}"/>'
            else:
                body += f'<path style="{style("--slate", 3)}" d="M{x+25} {y+30}H{x+95}M{x+25} {y+60}H{x+95}M{x+25} {y+90}H{x+70}"/>'
    return svg(body)


def gear(cx, cy, r, teeth, colour):
    points = []
    for k in range(teeth * 4):
        a = 2 * math.pi * k / (teeth * 4)
        radius = r + (12 if k % 4 in (1, 2) else 0)
        points.append((cx + radius * math.cos(a), cy + radius * math.sin(a)))
    d = "M" + "L".join(f"{x:.1f} {y:.1f}" for x, y in points) + "Z"
    return f'<path style="{style(colour, 3.5, "--surface")}" d="{d}"/><circle cx="{cx}" cy="{cy}" r="{r * 0.35:.0f}" style="{style(colour, 3.5)}"/>'


def engines():
    """The engines: triangles in, a solver turning, quads out."""
    body = ""
    tris = []
    for j in range(4):
        for i in range(5):
            x, y = 110 + i * 46 + (j % 2) * 23, 90 + j * 46
            tris.append(f"M{x} {y}L{x + 46} {y}L{x + 23} {y + 46}Z")
    body += f'<path style="{style("--slate", 2.5)}" d="{" ".join(tris)}"/>'
    body += f'<path style="{style("--ink", 3)}" d="M400 180H470M452 166L470 180L452 194"/>'
    body += gear(590, 160, 70, 10, "--teal") + gear(705, 245, 46, 8, "--ink")
    body += f'<path style="{style("--ink", 3)}" d="M790 180H860M842 166L860 180L842 194"/>'
    body += warped_grid(890, 90, 5, 4, 46, 3)
    return svg(body)


def platforms():
    """Where the tools are sold: one package sent to several stores."""
    body = ""
    cx, cy = 600, 180
    stores = [(220, 80), (220, 280), (390, 40), (810, 40), (980, 80), (980, 280), (810, 320), (390, 320)]
    body += f'<path style="{style("--rule", 3)}; stroke-dasharray: 10 8" d="' + " ".join(f"M{cx} {cy}L{x} {y}" for x, y in stores) + '"/>'
    for k, (x, y) in enumerate(stores):
        colour = "--teal" if k in (0, 4) else "--ink"
        body += f'<rect x="{x - 34}" y="{y - 26}" width="68" height="52" rx="6" style="{style(colour, 3, "--surface")}"/>'
        body += f'<path style="{style(colour, 3)}" d="M{x - 34} {y - 8}H{x + 34}M{x - 14} {y + 26}V{y + 6}H{x + 14}V{y + 26}"/>'
    # The package: an isometric box with the quad motif on its face.
    s = 70
    top = f"M{cx} {cy - s}L{cx + s} {cy - s / 2}L{cx} {cy}L{cx - s} {cy - s / 2}Z"
    left = f"M{cx - s} {cy - s / 2}L{cx} {cy}L{cx} {cy + s}L{cx - s} {cy + s / 2}Z"
    right = f"M{cx + s} {cy - s / 2}L{cx} {cy}L{cx} {cy + s}L{cx + s} {cy + s / 2}Z"
    body += f'<path style="{style("--ink", 3.5, "--surface")}" d="{left}"/><path style="{style("--ink", 3.5, "--tint")}" d="{right}"/>'
    body += f'<path style="{style("--ink", 3.5, "--hot", 0.85)}" d="{top}"/>'
    body += f'<path style="{style("--ink", 2.5)}" d="M{cx + s / 2} {cy - s / 4}V{cy + s * 0.75}M{cx} {cy + s / 2}L{cx + s} {cy}"/>'
    return svg(body)


def glossary():
    """The words: a patch of quads with a pole, a loop and a quad pointed at, each with its entry."""
    x0, y0, size = 100, 30, 60
    body = warped_grid(x0, y0, 10, 5, size, 6, accent=(7, 3))
    # An edge loop along a row, and a pole where five edges meet.
    loop = " ".join(f"{'M' if i == 0 else 'L'}{x0 + i * size + 6 * math.sin(2 * 0.7 + i * 0.3):.1f} "
                    f"{y0 + 2 * size + 6 * math.sin(i * 0.6 - 0.4):.1f}" for i in range(11))
    body += f'<path style="{style("--teal", 6)}" d="{loop}"/>'
    pole = (x0 + 3 * size + 6 * math.sin(4 * 0.7 + 3 * 0.3), y0 + 4 * size + 6 * math.sin(3 * 0.6 - 4 * 0.2))
    body += f'<circle cx="{pole[0]:.1f}" cy="{pole[1]:.1f}" r="11" style="{style("--ink", 3, "--hot")}"/>'
    # The entries, each a card with a word and a line of text, tied to what it names.
    quad = (x0 + 7.5 * size, y0 + 3.5 * size)
    cards = [(80, "--teal", (x0 + 10 * size + 6 * math.sin(2 * 0.7 + 10 * 0.3), y0 + 2 * size + 6 * math.sin(10 * 0.6 - 0.4))),
             (170, "--hot", quad), (260, "--ink", pole)]
    for y, colour, (tx, ty) in cards:
        body += f'<path style="{style("--rule", 2.5)}; stroke-dasharray: 6 6" d="M{tx:.1f} {ty:.1f}L850 {y + 28}"/>'
        body += f'<rect x="850" y="{y}" width="240" height="56" style="{style(colour, 3, "--surface")}"/>'
        body += f'<path style="{style(colour, 5)}" d="M870 {y + 20}H950"/>'
        body += f'<path style="{style("--slate", 3)}" d="M870 {y + 38}H1060"/>'
    return svg(body)


def blender_mesh(x, y, height):
    """The Blender logo remeshed in quads (site/assets/brand/blender_mesh.svg,
    from scripts/blender_mesh_logo.py), in its own colours."""
    source = (ROOT / "site/assets/brand/blender_mesh.svg").read_text(encoding="utf-8")
    box = re.search(r'viewBox="([^"]*)"', source).group(1)
    inner = source[source.index(">") + 1:source.rindex("</svg>")]
    _, _, w, h = map(float, box.split())
    return f'<svg x="{x}" y="{y}" width="{height * w / h:.1f}" height="{height}" viewBox="{box}">{inner}</svg>'


def blender():
    """Built into Blender: the logo in quads, then one shape remeshed by a voxel grid and by a field."""
    body = blender_mesh(50, 30, 300)
    body += f'<path style="{style("--ink", 3)}" d="M452 180H512M494 166L512 180L494 194"/>'
    # The blob on a grid along the axes, its cells kept as stair steps.
    cx, cy, r = 680, 180, 110
    inside = lambda x, y: ((x - cx) / r) ** 2 + ((y - cy) / (r * 0.8)) ** 2 < 1
    cells = [f"M{x} {y}h40v40h-40Z" for x in range(560, 800, 40) for y in range(60, 300, 40) if inside(x + 20, y + 20)]
    body += f'<path style="{style("--slate", 2.5, "--surface")}" d="{" ".join(cells)}"/>'
    body += f'<ellipse cx="{cx}" cy="{cy}" rx="{r}" ry="{r * 0.8:.0f}" style="{style("--hot", 3)}; stroke-dasharray: 8 7"/>'
    # The same outline, quads aligned with it: rings and spokes.
    cx = 1000
    rings = " ".join(f'M{cx + r * k:.1f} {cy}' + "".join(f'L{cx + r * k * math.cos(a):.1f} {cy + r * 0.8 * k * math.sin(a):.1f}'
                     for a in (2 * math.pi * t / 48 for t in range(1, 49))) for k in (0.35, 0.6, 0.82, 1.0))
    spokes = " ".join(f'M{cx + r * 0.35 * math.cos(a):.1f} {cy + r * 0.28 * math.sin(a):.1f}L{cx + r * math.cos(a):.1f} {cy + r * 0.8 * math.sin(a):.1f}'
                      for a in (2 * math.pi * t / 16 for t in range(16)))
    body += f'<path style="{style("--teal", 3)}" d="{rings} {spokes}"/>'
    body += f'<ellipse cx="{cx}" cy="{cy}" rx="{r * 0.35:.0f}" ry="{r * 0.28:.0f}" style="{style("--ink", 3, "--hot", 0.85)}"/>'
    return svg(body)


def pipeline():
    """The pipeline in Blender: the logo, then dense triangles, quads, and the quads with a seam and the detail baked on them."""
    body = blender_mesh(24, 74, 212)
    tris = []
    for j in range(7):
        for i in range(8):
            x, y = 330 + i * 24 + (j % 2) * 12, 96 + j * 24
            tris.append(f"M{x} {y}L{x + 24} {y}L{x + 12} {y + 24}Z")
    body += f'<path style="{style("--slate", 1.5)}" d="{" ".join(tris)}"/>'
    for x in (552, 842):
        body += f'<path style="{style("--ink", 3)}" d="M{x} 180H{x + 40}M{x + 22} 166L{x + 40} 180L{x + 22} 194"/>'
    body += warped_grid(616, 100, 5, 4, 40, 3, stroke="--teal")
    body += f'<path style="{style(None, 0, "--hot", 0.35)}" d="M910 116Q960 150 1010 122T1106 128V256Q1060 276 1010 254T910 256Z"/>'
    body += warped_grid(906, 100, 5, 4, 40, 3, stroke="--teal")
    body += f'<path style="{style("--hot", 5)}" d="M1026 100V260"/>'
    return svg(body)


BANNERS = {"theory": theory, "results": results, "research": research,
           "retopology": retopology, "engines": engines, "platforms": platforms,
           "glossary": glossary, "blender": blender, "pipeline": pipeline}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, draw in BANNERS.items():
        (OUT / f"{name}.svg").write_text(draw(), encoding="utf-8")
        print(f"site/assets/articles/{name}.svg")


if __name__ == "__main__":
    main()
