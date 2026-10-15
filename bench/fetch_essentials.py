#!/usr/bin/env python3
"""The human bodies of Blender's Essentials asset library in the bench.

    python3 bench/fetch_essentials.py [--refresh]

Blender 5 downloads them into its cache when they are first used from the
Asset Browser (Essentials > Base Meshes > Human). Each holds a base mesh,
modelled by hand, under a Multires modifier whose top level is the sculpt.
From each, two things are made:

- the scene of the bench, in bench/corpus-large/scenes: the body as Blender
  sees it, its Multires at the top level, so that the sculpt is what every
  remesher is given;
- the hand-made mesh, the base level of the Multires, in
  bench/reference/handmade: what the remeshers are held up against.

The library is CC0 (see CREDITS.md); the sculpts are large, so their scenes
stay out of the repository, and the hand-made meshes, small, go in it.
"""

import argparse
import subprocess
import sys
from pathlib import Path

BENCH = Path(__file__).resolve().parent
CACHE = Path.home() / ".cache" / "blender" / "remote-assets" / "online-essentials" / "base_meshes"
SCENES = BENCH / "corpus-large" / "scenes"
HANDMADE = BENCH / "reference" / "handmade"
# Model of the bench: file of the library and the object of the body.
BODIES = {
    "human_male": ("human_base_meshes__body-realistic-male.blend", "GEO-body_male_realistic"),
    "human_female": ("human_base_meshes__body-realistic-female.blend", "GEO-body_female_realistic"),
}

# Run inside Blender: the base level of the body, written as an OBJ.
EXPORT = """
import sys, bpy
path, name = sys.argv[-2], sys.argv[-1]
body = bpy.data.objects[name]
for modifier in body.modifiers:
    if modifier.type == "MULTIRES":
        modifier.levels = modifier.sculpt_levels = modifier.render_levels = 0
depsgraph = bpy.context.evaluated_depsgraph_get()
evaluated = body.evaluated_get(depsgraph)
mesh = evaluated.to_mesh()
with open(path, "w") as out:
    out.write("# " + name + ", base mesh of Blender's Essentials (Human Base Meshes), CC0\\n")
    for vertex in mesh.vertices:
        co = body.matrix_world @ vertex.co
        out.write(f"v {co.x:.9g} {co.y:.9g} {co.z:.9g}\\n")
    for polygon in mesh.polygons:
        out.write("f " + " ".join(str(v + 1) for v in polygon.vertices) + "\\n")
print("HANDMADE", path, len(mesh.polygons))
evaluated.to_mesh_clear()
"""


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--refresh", action="store_true", help="make again what is already there")
    options = parser.parse_args()
    SCENES.mkdir(parents=True, exist_ok=True)
    HANDMADE.mkdir(parents=True, exist_ok=True)
    for model, (file, body) in BODIES.items():
        source = CACHE / file
        if not source.exists():
            print(f"{model}: {source} is not there: add the body once from Blender's Asset Browser "
                  "(Essentials, Base Meshes), then run this again", file=sys.stderr)
            continue
        scene = SCENES / f"{model}.blend"
        if options.refresh or not scene.exists():
            subprocess.run(
                ["blender", "-b", "--factory-startup", "--python", str(BENCH / "make_scene.py"), "--",
                 str(source), str(scene), "--object", body, "--preset", "organic", "--quads", "3000"],
                check=True,
            )
        handmade = HANDMADE / f"{model}.obj"
        if options.refresh or not handmade.exists():
            subprocess.run(
                ["blender", "-b", "--factory-startup", str(source), "--python-expr", EXPORT, "--",
                 str(handmade), body],
                check=True,
            )


if __name__ == "__main__":
    main()
