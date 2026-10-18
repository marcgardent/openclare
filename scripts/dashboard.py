#!/usr/bin/env python3
"""The dashboard of the README: pictures of the remeshers side by side, and their marks.

    python3 scripts/dashboard.py              the chart of the marks, from bench/results/scores.json
    python3 scripts/dashboard.py --gallery    and the sheet of pictures, from docs/images/gallery/
                                              (made by scripts/gallery.py)

Both are written for a light and for a dark page, into docs/images/dashboard/;
the README shows the one that goes with the reader's theme.
"""

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "bench"))
from run import MODELS  # noqa: E402

OUT = ROOT / "docs" / "images" / "dashboard"
SCORES = ROOT / "bench" / "results" / "scores.json"
GALLERY = ROOT / "docs" / "images" / "gallery"

# The remeshers, in a fixed order with a fixed colour each, validated
# (scripts of the dataviz guidance) for both themes.
TOOLS = [
    ("openclare", "openclare"),
    ("quadriflow", "QuadriFlow (Blender)"),
    ("autoremesher", "AutoRemesher"),
]
THEMES = {
    "light": {
        "surface": "#fcfcfb", "text": "#0b0b0b", "muted": "#52514e", "grid": "#e4e3df",
        "series": ["#2a78d6", "#1baf7a", "#eda100"],
    },
    "dark": {
        "surface": "#1a1a19", "text": "#ffffff", "muted": "#c3c2b7", "grid": "#3a3a37",
        "series": ["#3987e5", "#199e70", "#c98500"],
    },
}
GROUPS = [("organic", "Organic models"), ("mechanical", "Mechanical parts"), ("extra", "Assembled and generated")]
FONT = "Inter, 'Segoe UI', Helvetica, Arial, sans-serif"
# The background scripts/render.py renders on with --opaque.
RENDER_BACKGROUND = (203, 203, 203)


def averages():
    """{group: {budget: (models, {tool: mark})}}: the average overall mark of
    each remesher over the models of a group that all of them were measured on."""
    scores = json.loads(SCORES.read_text())
    group_of = {name: group for name, group, *_ in MODELS}
    table = {}
    for group, _ in GROUPS:
        table[group] = {}
        for budget in sorted(scores, key=int):
            common = [
                model
                for model, by_tool in scores[budget].items()
                if group_of.get(model) == group
                and all(by_tool.get(tool) and by_tool[tool].get("overall") is not None for tool, _ in TOOLS)
            ]
            marks = {tool: sum(scores[budget][m][tool]["overall"] for m in common) / len(common) for tool, _ in TOOLS} if common else {}
            table[group][int(budget)] = (len(common), marks)
    return table


def bar(x, top, width, bottom, colour):
    """A bar standing on `bottom`, its data end rounded by 4."""
    r = min(4.0, width / 2, max(0.0, bottom - top))
    return (f'<path d="M{x:.1f},{bottom:.1f} V{top + r:.1f} Q{x:.1f},{top:.1f} {x + r:.1f},{top:.1f} '
            f'H{x + width - r:.1f} Q{x + width:.1f},{top:.1f} {x + width:.1f},{top + r:.1f} V{bottom:.1f} Z" '
            f'fill="{colour}"/>')


def chart(theme):
    colours = THEMES[theme]
    table = averages()
    budgets = sorted(next(iter(table.values())))
    bar_width, bar_gap, group_gap, axis = 16, 2, 22, 34
    group_width = len(TOOLS) * bar_width + (len(TOOLS) - 1) * bar_gap
    panel_width = axis + len(budgets) * group_width + (len(budgets) - 1) * group_gap + 24
    width, top, plot_height = len(GROUPS) * panel_width + 24, 96, 200
    height = top + plot_height + 64
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" '
           f'font-family="{FONT}">', f'<rect width="{width}" height="{height}" rx="12" fill="{colours["surface"]}"/>']
    out.append(f'<text x="24" y="34" font-size="17" font-weight="600" fill="{colours["text"]}">'
               'Average mark out of 100, by kind of model and number of quads asked for</text>')
    # The legend, in the order of the bars.
    x = 24
    for (tool, name), colour in zip(TOOLS, colours["series"]):
        out.append(f'<rect x="{x}" y="52" width="12" height="12" rx="3" fill="{colour}"/>')
        out.append(f'<text x="{x + 18}" y="62.5" font-size="13" fill="{colours["text"]}">{name}</text>')
        x += 30 + 7.4 * len(name)
    for index, (group, label) in enumerate(GROUPS):
        left = 12 + index * panel_width
        counts = sorted({table[group][b][0] for b in budgets})
        models = f"{counts[0]} models" if len(counts) == 1 else f"{counts[0]} to {counts[-1]} models"
        out.append(f'<text x="{left + axis}" y="{top - 14}" font-size="14" font-weight="600" fill="{colours["text"]}">'
                   f'{label}<tspan font-weight="400" fill="{colours["muted"]}">&#160;&#160;·&#160;&#160;{models}</tspan></text>')
        bottom = top + plot_height
        for mark in (0, 25, 50, 75, 100):
            y = bottom - plot_height * mark / 100
            out.append(f'<line x1="{left + axis - 4}" x2="{left + panel_width - 24}" y1="{y:.1f}" y2="{y:.1f}" '
                       f'stroke="{colours["grid"] if mark else colours["muted"]}" stroke-width="1"/>')
            if mark in (0, 50, 100):
                out.append(f'<text x="{left + axis - 8}" y="{y + 4:.1f}" font-size="11" text-anchor="end" '
                           f'fill="{colours["muted"]}">{mark}</text>')
        for b, budget in enumerate(budgets):
            x0 = left + axis + b * (group_width + group_gap)
            _, marks = table[group][budget]
            for t, ((tool, _), colour) in enumerate(zip(TOOLS, colours["series"])):
                if tool not in marks:
                    continue
                x = x0 + t * (bar_width + bar_gap)
                y = bottom - plot_height * marks[tool] / 100
                out.append(bar(x, y, bar_width, bottom, colour))
                # openclare's own mark written over its bar.
                if t == 0:
                    out.append(f'<text x="{x + bar_width / 2:.1f}" y="{y - 6:.1f}" font-size="11" font-weight="600" '
                               f'text-anchor="middle" fill="{colours["text"]}">{marks[tool]:.0f}</text>')
            out.append(f'<text x="{x0 + group_width / 2:.1f}" y="{bottom + 20}" font-size="12" text-anchor="middle" '
                       f'fill="{colours["muted"]}">{budget:,}</text>'.replace(",", " "))
        out.append(f'<text x="{left + axis + (panel_width - axis - 24) / 2:.1f}" y="{bottom + 40}" font-size="11" '
                   f'text-anchor="middle" fill="{colours["muted"]}">quads asked for</text>')
    out.append("</svg>")
    path = OUT / f"marks-{theme}.svg"
    path.write_text("\n".join(out) + "\n")
    return path


def sheet(theme):
    """The pictures of scripts/gallery.py on one sheet: a row per model, a
    column per remesher, under each its quads and its mark."""
    from PIL import Image, ImageDraw, ImageFont

    colours = THEMES[theme]
    shown = json.loads((GALLERY / "gallery.json").read_text())
    cell, pad, head, caption = 360, 16, 64, 46
    width = 150 + len(TOOLS) * (cell + pad) + pad
    height = head + len(shown) * (cell + caption + pad) + pad
    page = Image.new("RGB", (width, height), colours["surface"])
    draw = ImageDraw.Draw(page)

    def font(size, bold=False):
        for name in (["DejaVuSans-Bold.ttf"] if bold else ["DejaVuSans.ttf"]):
            try:
                return ImageFont.truetype(name, size)
            except OSError:
                pass
        return ImageFont.load_default()

    for t, ((tool, name), colour) in enumerate(zip(TOOLS, colours["series"])):
        x = 150 + pad + t * (cell + pad)
        draw.rounded_rectangle([x, 24, x + 14, 38], radius=3, fill=colour)
        draw.text((x + 22, 21), name, font=font(19, True), fill=colours["text"])
    for r, (model, row) in enumerate(shown.items()):
        y = head + r * (cell + caption + pad)
        draw.text((pad, y + cell / 2 - 24), model, font=font(20, True), fill=colours["text"])
        draw.text((pad, y + cell / 2 + 4), f"{row['quads']:,} quads".replace(",", " "), font=font(15), fill=colours["muted"])
        for t, (tool, _) in enumerate(TOOLS):
            x = 150 + pad + t * (cell + pad)
            made = row["tools"].get(tool, {})
            picture = ROOT / made["picture"] if "picture" in made else None
            if picture and picture.exists():
                image = Image.open(picture)
                # A picture rendered on a transparent background is laid on
                # the grey that scripts/render.py gives an opaque one, its
                # edges blended as they were drawn: no cut-out, no fringe.
                if image.mode in ("RGBA", "LA", "P"):
                    image = image.convert("RGBA")
                    backdrop = Image.new("RGBA", image.size, RENDER_BACKGROUND + (255,))
                    image = Image.alpha_composite(backdrop, image)
                image = image.convert("RGB").resize((cell, cell), Image.Resampling.LANCZOS)
                page.paste(image, (x, y))
                note = f"{made['quads']:,} quads".replace(",", " ")
                draw.text((x, y + cell + 6), note, font=font(15), fill=colours["muted"])
                mark = f"mark {made['overall']}"
                draw.text((x + cell - draw.textlength(mark, font=font(17, True)), y + cell + 4), mark,
                          font=font(17, True), fill=colours["text"])
                if not made["valid"]:
                    draw.text((x, y + cell + 25), "not a valid quad mesh", font=font(13), fill=colours["muted"])
            else:
                draw.rectangle([x, y, x + cell, y + cell], outline=colours["grid"], width=2)
                draw.text((x + 16, y + cell / 2 - 8), "no mesh", font=font(16), fill=colours["muted"])
    path = OUT / f"gallery-{theme}.webp"
    page.save(path, quality=88)
    return path


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--gallery", action="store_true", help="the sheet of pictures too")
    options = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    for theme in THEMES:
        print("written:", chart(theme).relative_to(ROOT))
        if options.gallery:
            print("written:", sheet(theme).relative_to(ROOT))


if __name__ == "__main__":
    main()
