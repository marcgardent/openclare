#!/usr/bin/env python3
"""The meshes and pictures of the dashboard of the README: the remeshers at the same number of quads.

    python3 scripts/gallery.py [--models ogre,pig] [--target 3000] [--no-render]

Every remesher is asked for the same number of quads (`--target`, or
`--quads ogre=6000,pig=4500` for some models): openclare and QuadriFlow once,
and AutoRemesher, which does not keep to a number, again until it comes
within a tenth of it (three tries at most). Each mesh is measured against the model, marked (scripts/scores.py)
and rendered from the same point of view as its page (scripts/pages.py), into
docs/images/gallery/, with what was found in gallery.json.
scripts/dashboard.py then puts them on one sheet (`make dashboard` does both).
"""

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "bench"))
sys.path.insert(0, str(ROOT / "scripts"))
from pages import render  # noqa: E402
from run import (  # noqa: E402
    MODELS,
    OUTPUT,
    addon,
    autoremesher,
    ensure_corpus,
    faces_of,
    measure,
    quadriflow,
)
from scores import scores_of  # noqa: E402

GALLERY = ROOT / "docs" / "images" / "gallery"
# A sculpt, a smooth shape, a mechanical part, a generated building.
MODELS_SHOWN = ["ogre", "pig", "part", "stonemill"]
# The remeshers, in the order of the columns, with the name they go by.
SHOWN = [
    ("openclare", "openclare"),
    ("quadriflow", "QuadriFlow (Blender)"),
    ("autoremesher", "AutoRemesher"),
]
# How close to the number AutoRemesher is to come, and in how many tries.
CLOSE_ENOUGH = 0.1
TRIES = 3


def quads_in(path):
    return faces_of(path) if path and Path(path).exists() else None


def relative(path):
    """`path` from the root of the clone, so that no file written to git names the home folder of
    whoever ran the script; the bare file name for a mesh outside the clone."""
    path = Path(path).resolve()
    return path.relative_to(ROOT).as_posix() if path.is_relative_to(ROOT) else path.name


def made_by(tool, model, sharp, boundary, count, directory):
    """(mesh or None, seconds, why not) of `tool` asked for `count` quads."""
    if tool == "openclare":
        path, seconds, failure, *_ = addon("gallery")(model, sharp, boundary, count, [1], directory)[1]
        return path, seconds, failure
    if tool == "quadriflow":
        return quadriflow(model, sharp, boundary, count, [1], directory)[1]
    # AutoRemesher makes what it makes of a number: asked again for as much
    # more or less as it fell short or over.
    asked, best = count, (None, 0.0, "not run")
    for _ in range(TRIES):
        path, seconds, failure = autoremesher(model, sharp, boundary, asked, [1], directory)[1]
        made = quads_in(path)
        if made is None:
            return best if best[0] else (None, seconds, failure)
        kept = directory / f"autoremesher_{asked}.obj"
        Path(path).replace(kept)
        if best[0] is None or abs(made - count) < abs(quads_in(best[0]) - count):
            best = (kept, seconds, None)
        if abs(made - count) <= CLOSE_ENOUGH * count:
            break
        asked = max(1, round(asked * count / made))
    return best


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--models", default=",".join(MODELS_SHOWN))
    parser.add_argument("--target", type=int, default=3000, help="quads asked of every remesher")
    parser.add_argument("--quads", default="", help="another number for some models: ogre=6000,pig=4500")
    parser.add_argument("--no-render", action="store_true", help="measure and write the table, with the pictures there")
    options = parser.parse_args()
    wanted = options.models.split(",")
    ensure_corpus(wanted)
    GALLERY.mkdir(parents=True, exist_ok=True)
    kind = {name: (sharp, boundary) for name, _, sharp, boundary in MODELS}
    # The models not asked for keep what they had.
    record = GALLERY / "gallery.json"
    shown = json.loads(record.read_text()) if record.exists() else {}
    asked = {name: int(count) for name, count in (pair.split("=") for pair in options.quads.split(",") if pair)}
    for model in wanted:
        sharp, boundary = kind[model]
        count = asked.get(model, options.target)
        directory = OUTPUT / "gallery" / model
        directory.mkdir(parents=True, exist_ok=True)
        row = {"quads": count, "tools": {}}
        for tool, _ in SHOWN:
            path, seconds, failure = made_by(tool, model, sharp, boundary, count, directory)
            if path is None:
                print(f"{model} {tool}: {failure}")
                row["tools"][tool] = {"failure": failure}
                continue
            metrics = measure(model, path, count)
            marks = scores_of([metrics], 1, seconds)
            picture = GALLERY / f"{model}__{tool}.webp"
            if not options.no_render:
                render(path, picture, model, opaque=True)
            row["tools"][tool] = {
                "quads": metrics.get("faces"),
                "valid": bool(metrics.get("valid")),
                "overall": round(marks["overall"]),
                "seconds": round(seconds, 1),
                "picture": picture.relative_to(ROOT).as_posix(),
                "mesh": relative(path),
            }
            print(f"{model:10} {tool:13} {metrics.get('faces'):>6} quads  mark {marks['overall']:5.1f}  {seconds:7.1f} s")
        shown[model] = row
    (GALLERY / "gallery.json").write_text(json.dumps(shown, indent=1) + "\n")
    print("written:", (GALLERY / "gallery.json").relative_to(ROOT))


if __name__ == "__main__":
    main()
