#!/usr/bin/env python3
"""One page per model of the bench: pictures of what each remesher makes of it, and its marks.

    python3 scripts/pages.py [--models ogre,blob] [--target 3000] [--no-render]

Writes docs/models/<model>.md, the pictures under docs/models/img/, and an
index, docs/models/README.md. The marks come from scripts/scores.py, hence
from what the bench measured; the pictures are rendered here, with Blender
(scripts/render.py), from one run of each remesher at `--target` quads.

Needs `blender` on the PATH and a release build of `retopo`. The meshes are
made again if they are not there (bench/out/pages), so this can take a
while the first time.
"""

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "bench"))
sys.path.insert(0, str(ROOT / "scripts"))
from run import DEFAULT_TARGET, LOCAL_MODELS, MODELS, OUTPUT, TOOLS, available_models, source_path  # noqa: E402
from scores import load, rows_of  # noqa: E402

PAGES = ROOT / "docs" / "models"
# The remeshers pictured, with the name they go by on the page.
PICTURED = [
    ("quadriflow", "QuadriFlow (Blender)"),
    ("quad-layout-adaptive", "openclare"),
    ("quadremesher", "Quad Remesher"),
]
# Models that come y-up, and those best looked at from somewhere else than
# the default (azimuth, elevation).
Y_UP = {"spot", "bob", "blub", "pig", "jerry", "rose", "bridge"}
VIEWS = {
    "ogre": (-70, 10),
    "terrain": (-60, 38),
    "cube": (35, 28),
    "lblock": (35, 28),
    "cylinder": (35, 28),
    "part": (35, 28),
}
# Pictures of these are not published: a fan creation after a game.
UNPICTURED = {"marine"}
# A source with more triangles than this is shown without its edges.
DENSE = 20_000


def render(mesh, picture, model, edges=True):
    azimuth, elevation = VIEWS.get(model, (-60, 20))
    command = ["blender", "-b", "--python", str(ROOT / "scripts" / "render.py"), "--", str(mesh), str(picture)]
    command += ["--up", "y" if model in Y_UP else "z", "--azimuth", str(azimuth), "--elevation", str(elevation)]
    command += ["--size", "600"]
    if not edges:
        command.append("--no-edges")
    done = subprocess.run(command, capture_output=True, text=True)
    return done.returncode == 0 and picture.exists()


def triangles_of(path):
    with open(path) as lines:
        return sum(1 for line in lines if line.startswith("f "))


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
        cells = [f"{table[tool]['overall']:.0f}" if tool in table else "—" for tool, _ in PICTURED]
        index.append(f"| [{model}]({model}.md) | {category} | " + " | ".join(cells) + " |")
        if model not in wanted:
            continue

        source = source_path(model)
        shown = []
        if model not in UNPICTURED and not options.no_render:
            picture = pictures / f"{model}__source.webp"
            if render(source, picture, model, edges=triangles_of(source) <= DENSE):
                shown.append(("Source", picture))
            directory = OUTPUT / "pages" / model
            directory.mkdir(parents=True, exist_ok=True)
            for tool, label in PICTURED:
                path, _, failure = TOOLS[tool](model, sharp, boundary, options.target, [1], directory)[1]
                picture = pictures / f"{model}__{tool}.webp"
                if path and render(path, picture, model):
                    shown.append((label, picture))
                else:
                    picture.unlink(missing_ok=True)
                    print(f"{model}: no picture for {tool} ({failure or 'render failed'})", file=sys.stderr)
        elif model not in UNPICTURED:
            for name, label in [("source", "Source")] + PICTURED:
                picture = pictures / f"{model}__{name}.webp"
                if picture.exists():
                    shown.append((label, picture))

        lines = [f"# {model}", ""]
        kind = f"{'An' if category[0] in 'aeiou' else 'A'} {category} model"
        kind += ", with hard edges" if sharp else ""
        kind += ", with open borders" if boundary else ""
        triangles = f"{triangles_of(source):,}".replace(",", " ")
        lines += [f"{kind}; {triangles} triangles in the source."]
        if model in LOCAL_MODELS:
            lines += ["Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`)."]
        lines += ["Author, source and licence: [CREDITS.md](../../CREDITS.md).", ""]
        if shown:
            lines += [f"## At {options.target} quads", ""]
            lines += ["| " + " | ".join(label for label, _ in shown) + " |", "|" + "---|" * len(shown)]
            lines += ["| " + " | ".join(f"![{label}](img/{picture.name})" for label, picture in shown) + " |", ""]
        elif model in UNPICTURED:
            lines += ["No picture of this one is published.", ""]
        lines += ["## Marks", "", "From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).", ""]
        for target in (1000, 3000, 8000):
            table = marks[target].get(model)
            if table:
                lines += [f"### {target} quads", ""] + rows_of(table, bold="quad-layout-adaptive") + [""]
        (PAGES / f"{model}.md").write_text("\n".join(lines))
        print(f"{model}: {len(shown)} pictures", flush=True)

    (PAGES / "README.md").write_text("\n".join(index) + "\n")
    print(f"written: {PAGES}")


if __name__ == "__main__":
    main()
