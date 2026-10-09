"""Blender script: writes one object of the open file as the bench takes its models.

    blender -b file.blend --python bench/export_object.py -- <object> <model.obj> [--without EDGE_SPLIT,BOOLEAN]

The object is taken as it is seen, modifiers applied, in triangles; those of
the kinds named after `--without` are left out (nothing is saved). Next to
the OBJ goes `<model>.groups`, the material of each triangle as one whole
number per line, for `openclare remesh --groups`.
"""

import sys
from pathlib import Path

import bpy

arguments = sys.argv[sys.argv.index("--") + 1 :]
name, target = arguments[0], Path(arguments[1])
without = arguments[arguments.index("--without") + 1].split(",") if "--without" in arguments else []
wanted = bpy.data.objects[name]
for modifier in wanted.modifiers:
    if modifier.type in without:
        modifier.show_viewport = False
for scene in bpy.data.scenes:
    if wanted.name in scene.objects:
        bpy.context.window.scene = scene
        break
evaluated = wanted.evaluated_get(bpy.context.evaluated_depsgraph_get())
mesh = evaluated.to_mesh()
mesh.calc_loop_triangles()
with open(target, "w") as out:
    for vertex in mesh.vertices:
        out.write("v %.7f %.7f %.7f\n" % tuple(vertex.co))
    for triangle in mesh.loop_triangles:
        out.write("f %d %d %d\n" % tuple(index + 1 for index in triangle.vertices))
with open(target.with_suffix(".groups"), "w") as out:
    names = [slot.material.name if slot.material else "none" for slot in evaluated.material_slots]
    out.write("# the material of each triangle; in order: " + ", ".join(names) + "\n")
    for triangle in mesh.loop_triangles:
        out.write("%d\n" % triangle.material_index)
print(f"EXPORT wrote {target}: {len(mesh.vertices)} vertices, {len(mesh.loop_triangles)} triangles, {len(names)} materials")
evaluated.to_mesh_clear()
