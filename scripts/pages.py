#!/usr/bin/env python3
"""One page per model of the bench: pictures of what each remesher makes of it, and its marks.

    python3 scripts/pages.py [--models ogre,blob] [--target 3000] [--no-render]

Writes docs/models/<model>.md, the pictures under docs/models/img/, and an
index, docs/models/README.md. The marks come from scripts/scores.py, hence
from what the bench measured; the pictures are rendered here, with Blender
(scripts/render.py), from one run of each remesher at `--target` quads.

Needs `blender` on the PATH and a release build of `openclare`. The meshes are
made again if they are not there (bench/out/pages), so this can take a
while the first time.
"""

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "bench"))
sys.path.insert(0, str(ROOT / "scripts"))
from run import (  # noqa: E402
    DEFAULT_TARGET,
    HANDMADE,
    LOCAL_MODELS,
    MODELS,
    OUTPUT,
    RESULTS,
    TOOLS,
    available_models,
    faces_of,
    memory_cap,
    run,
    source_path,
)
from scores import load, rows_of  # noqa: E402

PAGES = ROOT / "docs" / "models"
# The remeshers pictured, with the name they go by on the page.
PICTURED = [
    # Only the models that have a mesh modelled by hand (see bench/run.py).
    ("handmade", "Modelled by hand"),
    ("quadriflow", "QuadriFlow (Blender)"),
    ("autoremesher", "AutoRemesher"),
    ("openclare", "openclare"),
]
# Models that come y-up, and those best looked at from somewhere else than
# the default (azimuth, elevation).
Y_UP = {"spot", "bob", "blub", "pig", "jerry", "rose", "bridge"}
VIEWS = {
    "ogre": (-70, 10),
    "ogre_hp": (-70, 10),
    "terrain": (-60, 38),
    "venturi": (-125, 25),
    "cube": (35, 28),
    "lblock": (35, 28),
    "cylinder": (35, 28),
    "part": (35, 28),
}
# Models no picture of which is to be published. None at the moment.
UNPICTURED = set()
# A source with more triangles than this is shown without its edges.
DENSE = 20_000


def render(mesh, picture, model, edges=True, opaque=False):
    azimuth, elevation = VIEWS.get(model, (-60, 20))
    command = ["blender", "-b", "--factory-startup", "--python", str(ROOT / "scripts" / "render.py"), "--"]
    command += [str(mesh), str(picture)]
    command += ["--up", "y" if model in Y_UP else "z", "--azimuth", str(azimuth), "--elevation", str(elevation)]
    command += ["--size", "600"]
    if not edges:
        command.append("--no-edges")
    if opaque:
        command.append("--opaque")
    done = run(command, memory=memory_cap(model))
    return done.returncode == 0 and picture.exists()


def triangles_of(path):
    with open(path) as lines:
        return sum(1 for line in lines if line.startswith("f "))


def spaced(count):
    return f"{count:,}".replace(",", " ")


def mesh_of(tool, model, target, directory):
    """Where the mesh a picture was rendered from is kept, to count its
    quads when the pictures are not made again."""
    if tool == "handmade":
        return HANDMADE / f"{model}.obj"
    return directory / f"{tool}_1.obj"


def edge_loops(model):
    """A table of the edge loops of each remesher's mesh at the budget of
    the bench: the share of the edges on loops that close, spiral, stop at
    an irregular vertex, or run from border to border (DT-062)."""
    path = RESULTS / "results.json"
    if not path.exists():
        return []
    summary = json.loads(path.read_text()).get("summary", {}).get(model, {})
    tools = [(tool, label) for tool, label in PICTURED if tool in summary]
    if not any(summary[tool].get("loop_edges_closed") is not None for tool, _ in tools):
        return []
    lines = [
        "## Edge loops",
        "",
        f"At {DEFAULT_TARGET} quads asked for. Each loop is followed straight on through every regular vertex: it "
        "closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs "
        "from a border to a border. Share of the edges on each, those of borders and of faces that are not "
        "quads left out; \"—\" where the mesh is not a manifold.",
        "",
        "| | closed | spiral | stops | border to border | spirals | longest that stops |",
        "|---|---|---|---|---|---|---|",
    ]
    for tool, label in tools:
        row = summary[tool]
        if row.get("loop_edges_closed") is None:
            lines.append(f"| {label} | — | — | — | — | — | — |")
            continue
        shares = [f"{100.0 * (row.get(key) or 0.0):.0f} %" for key in
                  ("loop_edges_closed", "loop_edges_spiral", "loop_edges_open", "loop_edges_border")]
        lines.append(f"| {label} | " + " | ".join(shares)
                     + f" | {row.get('loops_spiral') or 0:.0f} | {row.get('longest_open_loop') or 0:.0f} |")
    return lines + [""]


def lines_kept(model, target):
    """A sentence on what the engine says of the lines of the model (curves
    drawn on it, borders between its materials), if its scene has any."""
    path = RESULTS / ("results.json" if target == DEFAULT_TARGET else f"results-{target}.json")
    runs = json.loads(path.read_text())["runs"].get(model, {}).get("openclare") if path.exists() else None
    said = runs[0].get("engine") if runs else None
    if not said or not (said.get("guide_edges") or said.get("sharp_curve_edges") or said.get("faces_across_groups")):
        return []
    parts = [f"{said.get('guide_edges', 0)} edges laid along the guide curves and the borders between materials"]
    if said.get("sharp_curve_edges"):
        parts.append(f"{said['sharp_curve_edges']} along the sharp curves")
    if said.get("faces_across_groups"):
        parts.append(f"{said['faces_across_groups']} faces left across a border")
    if said.get("guides_let_go"):
        parts.append(f"lines let go on {said['guides_let_go']} pieces")
    return ["Lines of the scene, as openclare reports them: " + ", ".join(parts) + ".", ""]


def gallery(shown):
    """Markdown: the pictures side by side, with what each is and how many
    faces it has."""
    lines = ["| " + " | ".join(label for label, _, _ in shown) + " |", "|" + "---|" * len(shown)]
    lines += ["| " + " | ".join(f"![{label}](img/{picture.name})" for label, picture, _ in shown) + " |"]
    lines += ["| " + " | ".join(caption for _, _, caption in shown) + " |", ""]
    return lines


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--models", default=",".join(available_models()))
    parser.add_argument("--target", type=int, default=DEFAULT_TARGET, help="quads asked for in the pictures")
    parser.add_argument("--no-render", action="store_true", help="only write the pages, with the pictures already there")
    options = parser.parse_args()
    wanted = options.models.split(",")
    pictures = PAGES / "img"
    pictures.mkdir(parents=True, exist_ok=True)
    marks = {target: load(target) for target in (1000, 3000, 8000)}

    index = [
        "# Models of the bench",
        "",
        "One page per model: what each remesher makes of it, and its marks from 0 to 100.",
        "Written by `scripts/pages.py` (`make pages`). How the marks are made: [SCORES.md](../../bench/results/SCORES.md).",
        "Authors, sources and licences of the models: [CREDITS.md](../../CREDITS.md).",
        "",
        f"Overall mark at {DEFAULT_TARGET} quads:",
        "",
        "| Model | Kind | " + " | ".join(label for _, label in PICTURED) + " |",
        "|---|---|" + "---|" * len(PICTURED),
    ]
    for model, category, sharp, boundary in MODELS:
        if model not in available_models():
            continue
        table = marks[DEFAULT_TARGET].get(model, {})
        # Not measured yet, and no page asked for: nothing to link to.
        if not table and model not in wanted and not (PAGES / f"{model}.md").exists():
            continue
        cells = [f"{table[tool]['overall']:.0f}" if tool in table else "—" for tool, _ in PICTURED]
        index.append(f"| [{model}]({model}.md) | {category} | " + " | ".join(cells) + " |")
        if model not in wanted:
            continue

        source = source_path(model)
        triangles = f"{spaced(triangles_of(source))} triangles"
        # What is shown: the label of each picture, the picture, and how
        # many faces the mesh in it has.
        shown = []
        directory = OUTPUT / "pages" / model
        if model not in UNPICTURED and not options.no_render:
            picture = pictures / f"{model}__source.webp"
            if render(source, picture, model, edges=triangles_of(source) <= DENSE):
                shown.append(("Source", picture, triangles))
            directory.mkdir(parents=True, exist_ok=True)
            for tool, label in PICTURED:
                runs = TOOLS[tool](model, sharp, boundary, options.target, [1], directory)
                if 1 not in runs:
                    # A tool with nothing for this model (no mesh modelled by hand).
                    continue
                path, _, failure, *_ = runs[1]
                picture = pictures / f"{model}__{tool}.webp"
                if path and render(path, picture, model):
                    shown.append((label, picture, f"{spaced(faces_of(path))} quads"))
                else:
                    picture.unlink(missing_ok=True)
                    print(f"{model}: no picture for {tool} ({failure or 'render failed'})", file=sys.stderr)
        elif model not in UNPICTURED:
            if (pictures / f"{model}__source.webp").exists():
                shown.append(("Source", pictures / f"{model}__source.webp", triangles))
            for tool, label in PICTURED:
                picture, mesh = pictures / f"{model}__{tool}.webp", mesh_of(tool, model, options.target, directory)
                if picture.exists():
                    caption = f"{spaced(faces_of(mesh))} quads" if mesh.exists() else ""
                    shown.append((label, picture, caption))

        lines = [f"# {model}", ""]
        kind = f"{'An' if category[0] in 'aeiou' else 'A'} {category} model"
        kind += ", with hard edges" if sharp else ""
        kind += ", with open borders" if boundary else ""
        lines += [f"{kind}; {triangles} in the source."]
        if model in LOCAL_MODELS:
            lines += ["Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`)."]
        lines += ["Author, source and licence: [CREDITS.md](../../CREDITS.md).", ""]
        if shown:
            lines += [f"## At {options.target} quads asked for", ""] + gallery(shown)
        if not shown and model in UNPICTURED:
            lines += ["No picture of this one is published.", ""]
        lines += edge_loops(model)
        lines += ["## Marks", "", "From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).", ""]
        for target in (1000, 3000, 8000):
            table = marks[target].get(model)
            if table:
                lines += [f"### {target} quads", ""] + rows_of(table, bold="openclare") + [""]
                lines += lines_kept(model, target)
        (PAGES / f"{model}.md").write_text("\n".join(lines))
        print(f"{model}: {len(shown)} pictures", flush=True)

    (PAGES / "README.md").write_text("\n".join(index) + "\n")
    print(f"written: {PAGES}")


if __name__ == "__main__":
    main()
