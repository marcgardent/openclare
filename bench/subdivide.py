"""Catmull-Clark subdivision of an OBJ with Blender, written back as triangles.

    blender -b --python bench/subdivide.py -- <in.obj> <out.obj> <levels>
"""

import os
import sys

import bpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from blender_io import evaluated_triangles, read_obj, write_obj  # noqa: E402

source, output, levels = sys.argv[sys.argv.index("--") + 1:]
bpy.ops.wm.read_factory_settings(use_empty=True)
obj = read_obj(source)
if int(levels) > 0:
    modifier = obj.modifiers.new("subdivision", "SUBSURF")
    modifier.levels = int(levels)
mesh = evaluated_triangles(obj)
write_obj(output, mesh)
print(f"SUBDIVIDED {len(mesh.vertices)} vertices {len(mesh.polygons)} triangles", flush=True)
