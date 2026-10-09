"""Blender script: makes the scenes of the bench that are made from a file as it is.

    blender -b --factory-startup --python bench/make_scenes.py -- [--models a,b] [--refresh]

A scene of the bench is a .blend that holds one object to remesh and the
settings of the add-on, saved with the scene (see bench/make_scene.py, which
makes one from an object of a .blend, with its modifiers and its curves).
Here the object is a plain mesh, named after the model:

- a model of bench/corpus/<model>.obj: its vertices and faces as they are in
  the file, into bench/scenes/<model>.blend;
- a BlenderKit model, bench/corpus-large/blenderkit/<model>.blend: every mesh
  object of the file with its modifiers and its place in the scene applied,
  in triangles, all in one mesh, into bench/corpus-large/scenes/<model>.blend.
  Neither is tracked: mind the licence of the asset (see CREDITS.md).

The settings: the object, 3000 quads, the preset `hard-surface` for the
models `MODELS` of run.py says have hard edges, `organic` for the others.
A scene that is there and newer than its file is left alone, unless
`--refresh` is given. The scenes made by hand (`AUTHORED` in run.py) are not
made here.
"""

import argparse
import sys
from pathlib import Path

import bpy

BENCH = Path(__file__).resolve().parent
sys.path.insert(0, str(BENCH))
sys.path.insert(0, str(BENCH.parent / "blender"))
import openclare  # noqa: E402
from blender_io import activate, evaluated_triangles, read_obj  # noqa: E402
from run import AUTHORED, DEFAULT_TARGET, LOCAL, LOCAL_MODELS, MODELS, obj_path, scene_path  # noqa: E402

# How much the size of the quads follows the curvature, in per cent, whatever
# the preset: what the bench has always run `openclare` with. (Left to
# itself the preset `hard-surface` sets it to 0, which is `openclare-uniform`.)
ADAPTIVE_SIZE = 50.0


def empty_file():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    # No copy of the file it replaces left beside a scene (.blend1).
    bpy.context.preferences.filepaths.save_version = 0


def merged(blend, name):
    """One mesh object out of every mesh object of the file `blend`, as it
    is seen there, in a file otherwise empty."""
    bpy.ops.wm.open_mainfile(filepath=str(blend), use_scripts=False)
    vertices, faces = [], []
    for part in bpy.data.objects:
        if part.type != "MESH":
            continue
        mesh = evaluated_triangles(part)
        offset = len(vertices)
        vertices.extend(tuple(vertex.co) for vertex in mesh.vertices)
        faces.extend([offset + index for index in polygon.vertices] for polygon in mesh.polygons)
    empty_file()
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(vertices, [], faces)
    mesh.update()
    made = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(made)
    return made


def make(model, sharp):
    target = scene_path(model)
    if model in LOCAL_MODELS:
        made = merged(LOCAL / "blenderkit" / f"{model}.blend", model)
    else:
        empty_file()
        made = read_obj(obj_path(model), model)
    activate(made)
    settings = bpy.context.scene.openclare
    settings.source = made
    settings.preset = "hard-surface" if sharp else "organic"
    settings.quads = DEFAULT_TARGET
    settings.adaptive_size = ADAPTIVE_SIZE
    target.parent.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(target), compress=True)
    mesh = made.data
    print(f"SCENE wrote {target.relative_to(BENCH.parent)}: {len(mesh.vertices)} vertices, "
          f"{len(mesh.polygons)} faces, preset {settings.preset}", flush=True)


def main():
    parser = argparse.ArgumentParser(prog="blender -b --factory-startup --python bench/make_scenes.py --",
                                     description=__doc__.splitlines()[0])
    parser.add_argument("--models", help="those only (default: every one that has its file)")
    parser.add_argument("--refresh", action="store_true", help="make again the scenes that are already there")
    options = parser.parse_args(sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else [])
    wanted = options.models.split(",") if options.models else None
    known = [name for name, *_ in MODELS]
    unknown = [name for name in wanted or [] if name not in known or name in AUTHORED]
    if unknown:
        sys.exit(f"no scene to make here for: {', '.join(unknown)} (made here: "
                 f"{', '.join(name for name in known if name not in AUTHORED)})")

    openclare.register()
    for model, _, sharp, _ in MODELS:
        if model in AUTHORED or (wanted is not None and model not in wanted):
            continue
        origin = LOCAL / "blenderkit" / f"{model}.blend" if model in LOCAL_MODELS else obj_path(model)
        target = scene_path(model)
        if not origin.exists():
            # Said only of what was asked for: a local model not fetched is no news.
            if wanted is not None:
                sys.exit(f"{model}: {origin.relative_to(BENCH.parent)} is missing")
            continue
        if target.exists() and target.stat().st_mtime >= origin.stat().st_mtime and not options.refresh:
            print(f"SCENE {target.relative_to(BENCH.parent)} is already there", flush=True)
            continue
        make(model, sharp)


if __name__ == "__main__":
    main()
