"""Puts four rendered panels side by side, each labelled with its numbers.

    python3 compose.py <out.png> <numbers.json> <panel.png> <mesh.obj> <title> [<panel.png> <mesh.obj> <title>]...

The numbers under each panel (quads, share of edges on spiral loops, of the
whole mesh whatever the view) are measured on <mesh.obj> by loops.py, the port of
`openclare measure`; they are also written to <numbers.json>. A legend of the
colours closes the figure.
"""

import json
import os
import sys

from PIL import Image, ImageDraw, ImageFont

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from loops import edge_loops, read_obj  # noqa: E402

FONT = "/usr/share/texmf/fonts/opentype/public/lm/lmroman10-regular.otf"
FALLBACK_FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf"
# Must match render_loops.py.
LEGEND = [
    ((0x0B, 0x3C, 0x8C), 1.0, "closed loop"),
    ((0xD5, 0x5E, 0x00), 2.0, "spiral loop"),
    ((140, 140, 140), 0.8, "loop stopping at an irregular vertex"),
]


def font(size):
    for path in (FONT, FALLBACK_FONT):
        if os.path.exists(path):
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def main():
    arguments = sys.argv[1:]
    out_path, numbers_path, triples = arguments[0], arguments[1], arguments[2:]
    panels, numbers = [], []
    for k in range(0, len(triples), 3):
        panel_path, mesh_path, title = triples[k:k + 3]
        positions, faces = read_obj(mesh_path)
        summary, _ = edge_loops(len(positions), faces)
        summary["mesh"] = os.path.basename(mesh_path)
        summary["title"] = title
        numbers.append(summary)
        panels.append(Image.open(panel_path).convert("RGB"))

    width = sum(p.width for p in panels)
    height = max(p.height for p in panels)
    size = max(28, width // 60)
    title_font, number_font = font(size), font(int(size * 0.85))
    line_gap = int(size * 1.25)
    legend_height = int(size * 1.6)
    canvas = Image.new("RGB", (width, height + 3 * line_gap + int(size * 0.6) + legend_height), "white")
    draw = ImageDraw.Draw(canvas)
    x = 0
    for k, (panel, summary) in enumerate(zip(panels, numbers)):
        canvas.paste(panel, (x, 0))
        letter = "abcdefgh"[k]
        others = summary["faces"] - summary["quads"]
        faces = f"{summary['quads']:,} quads" + (f" + {others} triangles" if others else "")
        lines = [
            f"({letter}) {summary['title']}",
            f"{100 * summary['loop_edges_spiral']:.1f}% of edges on spiral loops",
            faces,
        ]
        for j, (text, face) in enumerate(zip(lines, (title_font, number_font, number_font))):
            text_width = draw.textlength(text, font=face)
            draw.text((x + (panel.width - text_width) / 2, height + int(size * 0.3) + j * line_gap), text,
                      fill="black", font=face)
        x += panel.width

    # Legend: a short stroke of each colour, then its name, centred.
    legend_font = font(int(size * 0.85))
    stroke = int(size * 1.6)
    stroke_width = max(2, size // 9)
    items = [(colour, w, name, draw.textlength(name, font=legend_font)) for colour, w, name in LEGEND]
    gap = size
    total = sum(stroke + size // 3 + text_width + gap for *_, text_width in items) - gap
    x = (width - total) / 2
    y = canvas.height - legend_height + legend_height // 2
    for colour, w, name, text_width in items:
        if name == "spiral loop":  # with its dark rim, as render_loops.py draws it
            draw.line([(x, y), (x + stroke, y)], fill=(0x5A, 0x24, 0x00), width=max(2, int(stroke_width * w)) + 4)
        draw.line([(x, y), (x + stroke, y)], fill=colour, width=max(2, int(stroke_width * w)))
        x += stroke + size // 3
        draw.text((x, y), name, fill="black", font=legend_font, anchor="lm")
        x += text_width + gap
    canvas.save(out_path, optimize=True)
    with open(numbers_path, "w") as out:
        json.dump(numbers, out, indent=1)
    print(out_path, os.path.getsize(out_path), "bytes")


if __name__ == "__main__":
    main()
