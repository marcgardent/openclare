"""Blender script: turns a BlenderKit model into an OBJ source for the bench.

    blender -b <asset.blend> --python bench/import_blenderkit.py -- <output.obj>

The .blend is what the BlenderKit add-on downloads (in Blender: search for
`asset_base_id:<id>`). Every mesh object is exported with its modifiers and
its place in the scene applied, as triangles, into one OBJ.

Mind the licence of the asset. BlenderKit's Royalty Free licence lets a
project use an asset but not pass it on as it is: such a model, and what is
remeshed from it, stays out of the repository (bench/corpus-large is not
tracked) and is credited in bench/credits.py.
"""

import os
import sys

import bpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from blender_io import evaluated_triangles  # noqa: E402

output = sys.argv[sys.argv.index("--") + 1]
vertices, faces = [], []
for obj in bpy.data.objects:
    if obj.type != "MESH":
        continue
    mesh = evaluated_triangles(obj)
    offset = len(vertices)
    vertices.extend((v.co.x, v.co.y, v.co.z) for v in mesh.vertices)
    faces.extend([offset + i for i in polygon.vertices] for polygon in mesh.polygons)
    print(f"BLENDERKIT {obj.name}: {len(mesh.vertices)} vertices, {len(mesh.polygons)} triangles")

with open(output, "w") as out:
    for x, y, z in vertices:
        # Blender stores single precision: nine digits round-trip it exactly.
        out.write(f"v {x:.9g} {y:.9g} {z:.9g}\n")
    for face in faces:
        out.write("f " + " ".join(str(i + 1) for i in face) + "\n")
print(f"BLENDERKIT wrote {output}: {len(vertices)} vertices, {len(faces)} triangles")
