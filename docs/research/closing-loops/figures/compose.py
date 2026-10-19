"""Puts four rendered panels side by side, each labelled with its numbers.

    python3 compose.py [--lang fr] <out.png> <numbers.json> <panel.png> <mesh.obj> <title> [<panel.png> <mesh.obj> <title>]...

The numbers under each panel (quads, share of edges on spiral loops, of the
whole mesh whatever the view) are measured on <mesh.obj> by loops.py, the port of
`openclare measure`; they are also written to <numbers.json>. A legend of the
colours closes the figure. With --lang fr the labels and the legend are in
French, with a decimal comma and thin spaces (for the French translation of the
paper); the titles are given as they are.
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
COLOURS = [((0x0B, 0x3C, 0x8C), 1.0), ((0xD5, 0x5E, 0x00), 2.0), ((140, 140, 140), 0.8)]
SPIRAL = 1  # the colour drawn with a dark rim, as render_loops.py draws it
# The words of each language. A thin space is written THIN: Latin Modern has no
# glyph for it, so it is drawn as a gap.
THIN = "\u2009"
TEXTS = {
    "en": {
        "legend": ["closed loop", "spiral loop", "loop stopping at an irregular vertex"],
        "spiral": "{percent}% of edges on spiral loops",
        "quads": "{quads} quads",
        "triangles": " + {count} triangles",
        "decimal": ".",
        "thousands": ",",
    },
    "fr": {
        "legend": ["boucle fermée", "boucle en spirale", "boucle qui s’arrête à un sommet irrégulier"],
        "spiral": "{percent}" + THIN + "% des arêtes en spirale",
        "quads": "{quads} quads",
        "triangles": " + {count} triangles",
        "decimal": ",",
        "thousands": THIN,
    },
}


def font(size):
    for path in (FONT, FALLBACK_FONT):
        if os.path.exists(path):
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def draw_text(draw, position, text, font, **options):
    """Draws text whose thin spaces are gaps of a sixth of an em; returns its width."""
    gap = font.size / 6
    x, y = position
    for k, piece in enumerate(text.split(THIN)):
        if k:
            x += gap
        if draw is not None:
            draw.text((x, y), piece, fill="black", font=font, **options)
        x += font.getlength(piece)
    return x - position[0]


def main():
    arguments = sys.argv[1:]
    language = "en"
    if arguments[0] == "--lang":
        language, arguments = arguments[1], arguments[2:]
    words = TEXTS[language]
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
        quads = f"{summary['quads']:,}".replace(",", words["thousands"])
        percent = f"{100 * summary['loop_edges_spiral']:.1f}".replace(".", words["decimal"])
        faces = words["quads"].format(quads=quads) + (words["triangles"].format(count=others) if others else "")
        lines = [
            f"({letter}) {summary['title']}",
            words["spiral"].format(percent=percent),
            faces,
        ]
        for j, (text, face) in enumerate(zip(lines, (title_font, number_font, number_font))):
            text_width = draw_text(None, (0, 0), text, face)
            draw_text(draw, (x + (panel.width - text_width) / 2, height + int(size * 0.3) + j * line_gap), text, face)
        x += panel.width

    # Legend: a short stroke of each colour, then its name, centred.
    legend_font = font(int(size * 0.85))
    stroke = int(size * 1.6)
    stroke_width = max(2, size // 9)
    items = [(colour, w, name, draw.textlength(name, font=legend_font))
             for (colour, w), name in zip(COLOURS, words["legend"])]
    gap = size
    total = sum(stroke + size // 3 + text_width + gap for *_, text_width in items) - gap
    x = (width - total) / 2
    y = canvas.height - legend_height + legend_height // 2
    for k, (colour, w, name, text_width) in enumerate(items):
        if k == SPIRAL:  # with its dark rim, as render_loops.py draws it
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
