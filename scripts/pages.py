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
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "bench"))
sys.path.insert(0, str(ROOT / "scripts"))
from run import (  # noqa: E402
    DEFAULT_TARGET,
    LOCAL_MODELS,
    MODELS,
    OUTPUT,
    TOOLS,
    available_models,
    faces_of,
    quadremesher_reference,
    source_path,
)
from scores import MATCHED, load, rows_of, without_matched  # noqa: E402

PAGES = ROOT / "docs" / "models"
# The remeshers pictured, with the name they go by on the page.
PICTURED = [
    ("quadriflow", "QuadriFlow (Blender)"),
    ("openclare", "openclare"),
    ("quadremesher", "Quad Remesher"),
]
# Quad Remesher makes more quads than it is asked for. Pictured next to its
# mesh, in a section of its own: openclare asked for as many quads as that.
EQUAL = ("openclare-matched", "openclare, as many quads")
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
# Models no picture of which is to be published. None at the moment.
UNPICTURED = set()
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


def spaced(count):
    return f"{count:,}".replace(",", " ")


def mesh_of(tool, model, target, directory):
    """Where the mesh a picture was rendered from is kept, to count its
    quads when the pictures are not made again."""
    if tool == "quadremesher":
        return quadremesher_reference(model, target, uniform=False)
    return directory / f"{tool}_1.obj"


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
        cells = [f"{table[tool]['overall']:.0f}" if tool in table else "—" for tool, _ in PICTURED]
        index.append(f"| [{model}]({model}.md) | {category} | " + " | ".join(cells) + " |")
        if model not in wanted:
            continue

        source = source_path(model)
        triangles = f"{spaced(triangles_of(source))} triangles"
        # What is shown: the label of each picture, the picture, and how
        # many faces the mesh in it has.
        shown, equal = [], []
        directory = OUTPUT / "pages" / model
        if model not in UNPICTURED and not options.no_render:
            picture = pictures / f"{model}__source.webp"
            if render(source, picture, model, edges=triangles_of(source) <= DENSE):
                shown.append(("Source", picture, triangles))
            directory.mkdir(parents=True, exist_ok=True)
            for tool, label in PICTURED + [EQUAL]:
                path, _, failure, *_ = TOOLS[tool](model, sharp, boundary, options.target, [1], directory)[1]
                picture = pictures / f"{model}__{tool}.webp"
                if path and render(path, picture, model):
                    (equal if tool == EQUAL[0] else shown).append((label, picture, f"{spaced(faces_of(path))} quads"))
                else:
                    picture.unlink(missing_ok=True)
                    print(f"{model}: no picture for {tool} ({failure or 'render failed'})", file=sys.stderr)
        elif model not in UNPICTURED:
            if (pictures / f"{model}__source.webp").exists():
                shown.append(("Source", pictures / f"{model}__source.webp", triangles))
            for tool, label in PICTURED + [EQUAL]:
                picture, mesh = pictures / f"{model}__{tool}.webp", mesh_of(tool, model, options.target, directory)
                if picture.exists():
                    caption = f"{spaced(faces_of(mesh))} quads" if mesh.exists() else ""
                    (equal if tool == EQUAL[0] else shown).append((label, picture, caption))
        # Next to ours at equal quad count, the mesh of Quad Remesher again.
        theirs = [entry for entry in shown if entry[0] == "Quad Remesher"]
        equal = theirs + equal if theirs and equal else []

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
        if equal:
            lines += [
                "## At equal quad count",
                "",
                f"Quad Remesher makes more quads than the {options.target} it is asked for. "
                "Next to its mesh, openclare asked for as many quads as that mesh has.",
                "",
            ] + gallery(equal)
        if not shown and model in UNPICTURED:
            lines += ["No picture of this one is published.", ""]
        lines += ["## Marks", "", "From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).", ""]
        for target in (1000, 3000, 8000):
            table = marks[target].get(model)
            if table:
                lines += [f"### {target} quads", ""] + rows_of(without_matched(table), bold="openclare") + [""]
                pair = {tool: table[tool] for tool in MATCHED if tool in table}
                if len(pair) == len(MATCHED):
                    lines += ["At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.", ""]
                    lines += rows_of(pair) + [""]
        (PAGES / f"{model}.md").write_text("\n".join(lines))
        print(f"{model}: {len(shown)} pictures", flush=True)

    (PAGES / "README.md").write_text("\n".join(index) + "\n")
    print(f"written: {PAGES}")


if __name__ == "__main__":
    main()
