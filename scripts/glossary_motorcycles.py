#!/usr/bin/env python3
"""The animations of the motorcycle traces in the glossary, as animated SVGs.

    python3 scripts/glossary_motorcycles.py

Both reuse the small quad mesh of the T-mesh picture
(docs/glossary/make_figures.py).
- docs/glossary/motorcycles.svg: on the mesh, from every pole the traces leave
  together, advance one edge per step, and stop on a trace already drawn, on a
  pole or on the border. Then the patches they cut light up.
- docs/glossary/motorcycles_field.svg: the same race in a continuous cross
  field whose singularities are the poles of the mesh.
The SVGs are shown through an <img>, so they cannot read the site's CSS
variables: they are drawn in the site's light colours on the white of the
figures, and animated with SMIL, which needs no script.
"""

import math
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "docs/glossary"))
import make_figures as mf  # noqa: E402

OUT = ROOT / "docs/glossary/motorcycles.svg"
OUT_FIELD = ROOT / "docs/glossary/motorcycles_field.svg"

# The site's palette (site/assets/style.css, light theme).
INK = "#101828"
SLATE = "#4A5565"
TEAL = "#0A8F8A"
HOT = "#D4572A"
TINTS = [(TEAL, 0.16), (HOT, 0.14), (SLATE, 0.10), (TEAL, 0.30)]

SCALE = 40  # pixels per edge of the starting grid
MARGIN = 0.6
STEP = 0.32  # seconds per edge
PAUSE = 0.8  # seconds before the traces leave
FILL = 0.6  # seconds for the patches to light up
HOLD = 2.6  # seconds the result stays
FADE = 0.5  # seconds to clear before the loop starts again


def patches(mesh, traces):
    """The patch of every face: faces joined across edges on no trace and no border."""
    edge_faces = mf.edges_of(mesh.faces)
    cut = {edge for edge, pair in edge_faces.items() if len(pair) == 1}
    for path in traces:
        cut.update(frozenset(edge) for edge in zip(path, path[1:]))
    patch = [-1] * len(mesh.faces)
    count = 0
    for start in range(len(mesh.faces)):
        if patch[start] >= 0:
            continue
        patch[start], stack = count, [start]
        while stack:
            face = mesh.faces[stack.pop()]
            for k in range(4):
                edge = frozenset((face[k], face[(k + 1) % 4]))
                if edge not in cut:
                    for g in edge_faces[edge]:
                        if patch[g] < 0:
                            patch[g] = count
                            stack.append(g)
        count += 1
    touching = defaultdict(set)
    for pair in edge_faces.values():
        if len(pair) == 2 and patch[pair[0]] != patch[pair[1]]:
            touching[patch[pair[0]]].add(patch[pair[1]])
            touching[patch[pair[1]]].add(patch[pair[0]])
    colour = {}
    for p in range(count):
        used = {colour.get(q) for q in touching[p]}
        colour[p] = next(c for c in range(len(TINTS)) if c not in used)
    return patch, colour


def discrete():
    mesh = mf.planar_pairs_mesh()
    positions = mesh.lay_out()
    traces = mf.trace_separatrices(mesh)
    poles = dict(mesh.poles())
    patch, colour = patches(mesh, traces)

    width, height = mesh.width + 2 * MARGIN, mesh.height + 2 * MARGIN

    def xy(v):
        x, y = positions[v]
        return f"{(x + MARGIN) * SCALE:.1f} {(height - MARGIN - y) * SCALE:.1f}"

    steps = max(len(path) for path in traces) - 1
    total = PAUSE + steps * STEP + FILL + HOLD + FADE

    def at(seconds):
        return f"{min(max(seconds / total, 0.0), 1.0):.4f}"

    def appear(start, length, end_fade=True):
        """An opacity animation: 0 until start, 1 after start + length, 0 again at the end."""
        times = ["0", at(start), at(start + length), at(total - FADE), "1"]
        return (f'<animate attributeName="opacity" dur="{total:.2f}s" repeatCount="indefinite" '
                f'keyTimes="{";".join(times)}" values="0;0;1;1;0"/>')

    out = []
    # the patches, lit once every trace has stopped
    lit = PAUSE + steps * STEP
    by_patch = defaultdict(list)
    for f, face in enumerate(mesh.faces):
        by_patch[patch[f]].append("M" + "L".join(xy(v) for v in face) + "Z")
    for p, faces in by_patch.items():
        fill, opacity = TINTS[colour[p]]
        out.append(f'<g opacity="0"><path fill="{fill}" fill-opacity="{opacity}" d="{"".join(faces)}"/>'
                   f'{appear(lit, FILL)}</g>')
    # the quads
    edges = mf.edges_of(mesh.faces)
    inner = "".join(f"M{xy(a)}L{xy(b)}" for a, b in (tuple(e) for e, pair in edges.items() if len(pair) == 2))
    border = "".join(f"M{xy(a)}L{xy(b)}" for a, b in (tuple(e) for e, pair in edges.items() if len(pair) == 1))
    out.append(f'<path stroke="{SLATE}" stroke-opacity="0.35" stroke-width="1" fill="none" d="{inner}"/>')
    out.append(f'<path stroke="{INK}" stroke-width="3" stroke-linecap="round" fill="none" d="{border}"/>')
    # the traces: each edge drawn during its step, as a line growing from its start
    for path in traces:
        for i, (a, b) in enumerate(zip(path, path[1:])):
            start = PAUSE + i * STEP
            (xa, ya), (xb, yb) = (map(float, xy(v).split()) for v in (a, b))
            times = ["0", at(start), at(start + STEP), at(total - FADE), "1"]
            values = [f"{xa:.1f};{xa:.1f};{xb:.1f};{xb:.1f};{xa:.1f}", f"{ya:.1f};{ya:.1f};{yb:.1f};{yb:.1f};{ya:.1f}"]
            out.append(
                f'<line x1="{xa:.1f}" y1="{ya:.1f}" x2="{xa:.1f}" y2="{ya:.1f}" stroke="{INK}" stroke-width="4" stroke-linecap="round" opacity="0">'
                + "".join(f'<animate attributeName="{name}" dur="{total:.2f}s" repeatCount="indefinite" '
                          f'keyTimes="{";".join(times)}" values="{v}"/>' for name, v in zip(("x2", "y2"), values))
                # hidden until its step: a line of length 0 with round caps is a dot
                + f'<animate attributeName="opacity" dur="{total:.2f}s" repeatCount="indefinite" calcMode="discrete" '
                  f'keyTimes="0;{at(start)};{at(total - FADE)}" values="0;1;0"/>'
                + "</line>"
            )
    # where a trace stops: a dot, when it stops
    for path in traces:
        end = path[-1]
        if end in poles:
            continue
        x, y = xy(end).split()
        out.append(f'<circle cx="{x}" cy="{y}" r="6" fill="{HOT}" stroke="#fff" stroke-width="2" opacity="0">'
                   f'{appear(PAUSE + (len(path) - 1) * STEP, 0.05)}</circle>')
    # the poles: valence 3 a triangle, valence 5 a pentagon, as in the other pictures
    for v, k in poles.items():
        x, y = map(float, xy(v).split())
        r = 11
        corners = [(x + r * math.sin(2 * math.pi * j / k), y - r * math.cos(2 * math.pi * j / k)) for j in range(k)]
        fill = HOT if k == 3 else TEAL
        out.append(f'<path fill="{fill}" stroke="#fff" stroke-width="2" d="M'
                   + "L".join(f"{cx:.1f} {cy:.1f}" for cx, cy in corners) + 'Z"/>')

    w, h = round(width * SCALE), round(height * SCALE)
    svg = (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">'
           f'<rect width="{w}" height="{h}" fill="#fff"/>' + "".join(out) + "</svg>\n")
    OUT.write_text(svg)
    print(f"{OUT.relative_to(ROOT)}: {w}x{h}, {len(traces)} traces, {steps} steps, {total:.1f} s, "
          f"{len(svg) // 1024} KB")


def field():
    """The same race in a continuous cross field: the poles of the mesh become its
    singularities, and the traces follow the field instead of the edges."""
    mesh = mf.planar_pairs_mesh()
    positions = mesh.lay_out()
    poles = [(positions[v], k) for v, k in mesh.poles()]
    # 4 theta = sum of s arg(z - p): index +1/4 at valence 3, -1/4 at valence 5
    charges = [(complex(*p), 1 if k == 3 else -1) for p, k in poles]

    def theta(z, skip=None):
        return sum(s * math.atan2((z - p).imag, (z - p).real) for j, (p, s) in enumerate(charges) if j != skip) / 4

    def heading(z, previous):
        """The direction of the cross at z closest to the previous heading."""
        t = theta(z)
        return min((t + m * math.pi / 2 for m in range(4)), key=lambda a: abs(math.remainder(a - previous, 2 * math.pi)))

    h = 0.03
    traces = []
    for j, (p, s) in enumerate(charges):
        c = 4 * theta(p, skip=j)
        for m in range(4 - s):
            a = (c + 2 * math.pi * m) / (4 - s)
            traces.append({"points": [p, p + 2 * h * complex(math.cos(a), math.sin(a))], "heading": a, "alive": True, "pole": j})
    cell = defaultdict(list)  # spatial hash of the points laid: (trace, index)

    def key(z):
        return int(z.real / (2 * h)), int(z.imag / (2 * h))

    def blocked(z, t):
        kx, ky = key(z)
        for dx in (-1, 0, 1):
            for dy in (-1, 0, 1):
                for u, i in cell[(kx + dx, ky + dy)]:
                    if abs(traces[u]["points"][i] - z) < 1.5 * h and (u != t or i < len(traces[t]["points"]) - 30):
                        return True
        return False

    for t, trace in enumerate(traces):
        for i, z in enumerate(trace["points"]):
            cell[key(z)].append((t, i))
    w0, h0 = mesh.width, mesh.height
    while any(trace["alive"] for trace in traces):
        moved = []
        for t, trace in enumerate(traces):
            if not trace["alive"]:
                continue
            z = trace["points"][-1]
            a = heading(z, trace["heading"])
            mid = z + h / 2 * complex(math.cos(a), math.sin(a))
            a = heading(mid, a)
            nxt = z + h * complex(math.cos(a), math.sin(a))
            trace["heading"] = a
            if not (0 < nxt.real < w0 and 0 < nxt.imag < h0):
                nxt = complex(min(max(nxt.real, 0), w0), min(max(nxt.imag, 0), h0))
                trace["alive"] = False
            else:
                near = [p for j, (p, _) in enumerate(charges) if abs(p - nxt) < 2 * h and j != trace["pole"]]
                if near:
                    nxt = near[0]
                    trace["alive"] = False
                elif blocked(nxt, t):
                    trace["alive"] = False
            trace["points"].append(nxt)
            moved.append(t)
        for t in moved:
            cell[key(traces[t]["points"][-1])].append((t, len(traces[t]["points"]) - 1))

    width, height = w0 + 2 * MARGIN, h0 + 2 * MARGIN

    def xy(z):
        return (z.real + MARGIN) * SCALE, (height - MARGIN - z.imag) * SCALE

    speed = 1 / STEP  # one unit (an edge of the mesh) per step, as in the discrete race
    longest = max(len(trace["points"]) for trace in traces) * h
    total = PAUSE + longest / speed + HOLD + FADE

    def at(seconds):
        return f"{min(max(seconds / total, 0.0), 1.0):.4f}"

    out = []
    # the field: a small cross every unit
    crosses = []
    for i in range(w0):
        for j in range(h0):
            z = complex(i + 0.5, j + 0.5)
            if min(abs(z - p) for p, _ in charges) < 0.4:
                continue
            x, y = xy(z)
            t = theta(z)
            for a in (t, t + math.pi / 2):
                dx, dy = 0.28 * SCALE * math.cos(a), -0.28 * SCALE * math.sin(a)
                crosses.append(f"M{x - dx:.1f} {y - dy:.1f}L{x + dx:.1f} {y + dy:.1f}")
    out.append(f'<path stroke="{SLATE}" stroke-opacity="0.45" stroke-width="1.5" stroke-linecap="round" fill="none" d="{"".join(crosses)}"/>')
    x0, y0 = xy(complex(0, h0))
    out.append(f'<rect x="{x0:.1f}" y="{y0:.1f}" width="{w0 * SCALE}" height="{h0 * SCALE}" stroke="{INK}" stroke-width="3" fill="none"/>')
    # the traces, growing at one speed: a dash that uncovers the path
    for trace in traces:
        points = trace["points"]
        d = "M" + "L".join("{:.1f} {:.1f}".format(*xy(z)) for z in points)
        end = PAUSE + (len(points) - 1) * h / speed
        times = f"0;{at(PAUSE)};{at(end)};{at(total - FADE)};1"
        out.append(f'<path d="{d}" pathLength="1" stroke="{INK}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" '
                   f'fill="none" stroke-dasharray="1 1" stroke-dashoffset="1">'
                   f'<animate attributeName="stroke-dashoffset" dur="{total:.2f}s" repeatCount="indefinite" '
                   f'keyTimes="{times}" values="1;1;0;0;1"/></path>')
        if abs(points[-1] - charges[trace["pole"]][0]) > h and not any(abs(points[-1] - p) < 1e-9 for p, _ in charges):
            x, y = xy(points[-1])
            out.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="6" fill="{HOT}" stroke="#fff" stroke-width="2" opacity="0">'
                       f'<animate attributeName="opacity" dur="{total:.2f}s" repeatCount="indefinite" calcMode="discrete" '
                       f'keyTimes="0;{at(end)};{at(total - FADE)}" values="0;1;0"/></circle>')
    for p, k in poles:
        x, y = xy(complex(*p))
        r = 11
        corners = [(x + r * math.sin(2 * math.pi * j / k), y - r * math.cos(2 * math.pi * j / k)) for j in range(k)]
        out.append(f'<path fill="{HOT if k == 3 else TEAL}" stroke="#fff" stroke-width="2" d="M'
                   + "L".join(f"{cx:.1f} {cy:.1f}" for cx, cy in corners) + 'Z"/>')
    w, hh = round(width * SCALE), round(height * SCALE)
    svg = (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {hh}" width="{w}" height="{hh}">'
           f'<rect width="{w}" height="{hh}" fill="#fff"/>' + "".join(out) + "</svg>\n")
    OUT_FIELD.write_text(svg)
    print(f"{OUT_FIELD.relative_to(ROOT)}: {w}x{hh}, {len(traces)} traces, {total:.1f} s, {len(svg) // 1024} KB")


if __name__ == "__main__":
    discrete()
    field()
