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

# Each stage is announced as it begins, and a host can stop the run.
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
source = evaluated.to_mesh()
stages = []
openclare.quads_of(source, "organic", {"quads": 800}, on_stage=lambda stage: stages.append(stage))
print("STAGES", ", ".join(stages))
# Suzanne is a head and two eyes: three pieces, each with its stages.
assert stages[0] == "piece 1 of 3: preprocessing" and stages[-1] == "piece 3 of 3: fitting", stages
assert "piece 2 of 3: cross field" in stages
assert openclare.worth_a_line("cross field") and openclare.worth_a_line("piece 2 of 3: layout")
assert openclare.worth_a_line("piece 42 of 820: preprocessing") and not openclare.worth_a_line("piece 42 of 820: layout")
assert not openclare.worth_a_line("piece 43 of 820: preprocessing")
try:
    openclare.quads_of(source, "organic", {"quads": 800}, on_stage=lambda stage: stage.endswith("layout"))
    raise AssertionError("the run was not stopped")
except openclare.engine.EngineError as error:
    assert str(error) == "stopped on request", error
evaluated.to_mesh_clear()

# The same mesh comes out whether Blender's arrays are read in place or
# copied out first.
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
source = evaluated.to_mesh()
source.calc_loop_triangles()
assert openclare.in_place(source), "this Blender does not lay its meshes out as expected"
direct, _ = openclare.quads_of(source, "organic", {"quads": 800})
really_in_place = openclare.in_place
openclare.in_place = lambda mesh: False
copied, _ = openclare.quads_of(source, "organic", {"quads": 800})
openclare.in_place = really_in_place
evaluated.to_mesh_clear()
assert len(direct.polygons) == len(copied.polygons) == len(mesh.polygons)
assert all(a.co == b.co for a, b in zip(direct.vertices, copied.vertices)), "in place and copied differ"
print("IN PLACE OK")

# A painted vertex group: with the top of the head at 1 and the rest at 0,
# and a factor of 9, the top gets most of the quads.
def share_on_top(remeshed):
    polygons = remeshed.data.polygons
    return sum(1 for polygon in polygons if polygon.center.z > 0.0) / len(polygons)

for other in bpy.context.selected_objects:
    other.select_set(False)
bpy.context.view_layer.objects.active = original
original.select_set(True)
group = original.vertex_groups.new(name="more here")
group.add([vertex.index for vertex in original.data.vertices if vertex.co.z > 0.0], 1.0, "REPLACE")
settings = bpy.context.scene.openclare
settings.adaptive_size = 0.0
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
unpainted = share_on_top(bpy.context.active_object)

bpy.context.active_object.select_set(False)
bpy.context.view_layer.objects.active = original
original.select_set(True)
settings.density_group, settings.density_factor = "more here", 9.0
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
painted = bpy.context.active_object
print("DENSITY share of the quads on top: %.2f unpainted, %.2f painted" % (unpainted, share_on_top(painted)))
assert share_on_top(painted) > unpainted + 0.1, "the painted weights changed nothing"
assert abs(len(painted.data.polygons) - 800) < 80, "the paint changed the number of quads"

# A group that is not there is said, and a preset brings its own adaptive size.
bpy.context.active_object.select_set(False)
bpy.context.view_layer.objects.active = original
original.select_set(True)
settings.density_group = "no such group"
try:
    bpy.ops.object.openclare_remesh()
    raise AssertionError("a vertex group that does not exist went unnoticed")
except RuntimeError as error:
    assert "no such group" in str(error)
settings.density_group = ""
settings.preset = "hard-surface"
assert settings.adaptive_size == 0.0
settings.preset = "organic"
assert settings.adaptive_size == 50.0

# Export: the result carries its two UV layers, the seams of the atlas and,
# on a shape with hard edges, those edges marked sharp.
def alone(target):
    for other in bpy.context.selected_objects:
        other.select_set(False)
    bpy.context.view_layer.objects.active = target
    target.select_set(True)

alone(original)
settings.preset = "organic"
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
assert [layer.name for layer in mesh.uv_layers] == ["openclare grid", "openclare atlas"], list(mesh.uv_layers.keys())
assert mesh.uv_layers.active.name == "openclare atlas"
atlas = [tuple(item.uv) for item in mesh.uv_layers["openclare atlas"].data]
grid = [tuple(item.uv) for item in mesh.uv_layers["openclare grid"].data]
assert len(atlas) == len(grid) == len(mesh.loops)
assert all(0.0 <= c <= 1.0 for uv in atlas for c in uv), "the atlas leaves the unit square"
assert all(c == round(c) for uv in grid for c in uv), "the grid is not on whole numbers"
seams = sum(edge.use_seam for edge in mesh.edges)
assert seams > 0 and not any(edge.use_edge_sharp for edge in mesh.edges)
print("EXPORT organic: 2 UV layers,", seams, "seam edges, no sharp edge")

bpy.ops.mesh.primitive_cube_add()
cube = bpy.context.active_object
cube.modifiers.new("subdivide", "SUBSURF").subdivision_type = "SIMPLE"
cube.modifiers["subdivide"].levels = 4
alone(cube)
settings.preset, settings.quads = "hard-surface", 600
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
sharp = sum(edge.use_edge_sharp for edge in mesh.edges)
print("EXPORT cube:", len(mesh.polygons), "quads,", sharp, "sharp edges")
assert sharp >= 12 and sharp % 12 == 0, f"{sharp} sharp edges on a cube"

# With the boxes unticked, the result is a bare mesh.
alone(cube)
settings.export_grid_uv = settings.export_atlas_uv = settings.export_sharp_edges = False
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
assert len(mesh.uv_layers) == 0 and not any(edge.use_seam or edge.use_edge_sharp for edge in mesh.edges)
settings.export_grid_uv = settings.export_atlas_uv = settings.export_sharp_edges = True
settings.preset, settings.quads = "organic", 800

# Nothing to remesh: said, not crashed.
bpy.ops.object.empty_add()
assert not bpy.ops.object.openclare_remesh.poll()

openclare.unregister()
print("ADD-ON OK")
