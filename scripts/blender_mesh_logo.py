#!/usr/bin/env python3
"""The meshed Blender logo of the article banners, made light enough to inline.

    python3 scripts/blender_mesh_logo.py SOURCE.svg

SOURCE is the logo as Blender's SVG export writes it (Freestyle): a path per
face and a filled outline per edge, 1.4 MB. Writes site/assets/brand/blender_mesh.svg
with the same drawing order, the faces that follow each other in one colour
merged into one path, and each edge outline turned back into a line of the
same width; coordinates rounded to whole units, the logo being shown a third of its size.
"""

import math
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TARGET = ROOT / "site/assets/brand/blender_mesh.svg"


def points(d):
    return [tuple(float(v) for v in pair.split()) for pair in re.findall(r"[ML]\s*([-\d.]+ [-\d.]+)", d)]


def segment(outline):
    """The centre line of an edge drawn as a capsule, and its width."""
    p, q = max(((a, b) for a in outline for b in outline), key=lambda ab: math.dist(*ab))
    area = abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(outline, outline[1:] + outline[:1]))) / 2
    length = math.dist(p, q)
    r = area / (2 * length) if length else 0
    t = r / length if length else 0
    a = (p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t)
    b = (q[0] + (p[0] - q[0]) * t, q[1] + (p[1] - q[1]) * t)
    return a, b, 2 * r


def f(v):
    return str(round(v))


def main():
    source = Path(sys.argv[1]).read_text(encoding="utf-8")
    box = re.search(r'viewBox="([^"]*)"', source).group(1)
    runs = []  # (kind, colour, opacity, [path pieces], [widths])
    for layer in re.finditer(r'<g id="layer\.(Fills|Lines)[^"]*">(.*?)</g>', source, re.S):
        kind = layer.group(1)
        for path in re.finditer(r"<path ([^>]*)/>", layer.group(2)):
            attributes = dict(re.findall(r'([\w-]+)="([^"]*)"', path.group(1)))
            outline = points(attributes["d"])
            colour, opacity = attributes["fill"], attributes.get("fill-opacity", "1")
            if kind == "Fills":
                piece = "M" + "L".join(f"{f(x)} {f(y)}" for x, y in outline) + "Z"
                width = None
            else:
                a, b, width = segment(outline)
                piece = f"M{f(a[0])} {f(a[1])}L{f(b[0])} {f(b[1])}"
            if runs and runs[-1][:3] == (kind, colour, opacity):
                runs[-1][3].append(piece)
                if width:
                    runs[-1][4].append(width)
            else:
                runs.append((kind, colour, opacity, [piece], [width] if width else []))
    body = ""
    for kind, colour, opacity, pieces, widths in runs:
        if kind == "Fills":
            body += f'<path d="{"".join(pieces)}" fill="{colour}"/>'
        else:
            width = sorted(widths)[len(widths) // 2]
            body += (f'<path d="{"".join(pieces)}" fill="none" stroke="{colour}" stroke-opacity="{opacity}" '
                     f'stroke-width="{width:.2f}" stroke-linecap="round"/>')
    TARGET.parent.mkdir(parents=True, exist_ok=True)
    TARGET.write_text(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{box}">{body}</svg>\n', encoding="utf-8")
    print(f"written: {TARGET.relative_to(ROOT)}, {TARGET.stat().st_size // 1024} KB, {len(runs)} paths")


if __name__ == "__main__":
    main()
