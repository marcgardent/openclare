"""Runs the add-on in a Blender without interface, on a mesh it makes.

    blender -b --factory-startup --python blender/test_addon.py

Exits with an error if anything is not as it should be.
"""

import sys
from pathlib import Path

import bpy

sys.path.insert(0, str(Path(__file__).resolve().parent))
import openclare  # noqa: E402

openclare.register()
print("ENGINE", openclare.engine.version())

bpy.ops.mesh.primitive_monkey_add()
original = bpy.context.active_object
original.location = (1.0, 2.0, 3.0)
modifier = original.modifiers.new("subdivide", "SUBSURF")
modifier.levels = 2
triangles_before = len(original.data.polygons)

bpy.context.scene.openclare.preset = "organic"
bpy.context.scene.openclare.quads = 800
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}

remeshed = bpy.context.active_object
mesh = remeshed.data
print("RESULT", remeshed.name, len(mesh.vertices), "vertices,", len(mesh.polygons), "faces")
assert remeshed is not original and remeshed.name == f"{original.name}.quads"
assert len(original.data.polygons) == triangles_before, "the original was touched"
assert tuple(remeshed.location) == (1.0, 2.0, 3.0), "the result is not where the original is"
assert all(len(polygon.vertices) == 4 for polygon in mesh.polygons), "not all quads"
assert abs(len(mesh.polygons) - 800) < 80, f"{len(mesh.polygons)} quads for 800 asked"
assert remeshed.select_get() and not original.select_get()

# The same mesh comes out whether Blender's arrays are read in place or
# copied out first.
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
source = evaluated.to_mesh()
source.calc_loop_triangles()
assert openclare.in_place(source), "this Blender does not lay its meshes out as expected"
direct, _ = openclare.quads_of(source, "organic", 800)
openclare.in_place = lambda mesh: False
copied, _ = openclare.quads_of(source, "organic", 800)
evaluated.to_mesh_clear()
assert len(direct.polygons) == len(copied.polygons) == len(mesh.polygons)
assert all(a.co == b.co for a, b in zip(direct.vertices, copied.vertices)), "in place and copied differ"
print("IN PLACE OK")

# Nothing to remesh: said, not crashed.
bpy.ops.object.empty_add()
assert not bpy.ops.object.openclare_remesh.poll()

openclare.unregister()
print("ADD-ON OK")
