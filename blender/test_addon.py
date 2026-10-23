"""Runs the add-on in a Blender without interface, on a mesh it makes.

    blender -b --factory-startup --python blender/test_addon.py

Exits with an error if anything is not as it should be.
"""

import sys
import tempfile
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

original.openclare.quads = 800


def shape(settings, name):
    """Gives `settings` the values of the shape `name` the add-on ships."""
    for key, value in openclare.SHAPES[name].items():
        setattr(settings, key, value)


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
# A bare mesh: no UV layer, no edge marked.
assert len(mesh.uv_layers) == 0 and not any(edge.use_seam or edge.use_edge_sharp for edge in mesh.edges)
# What the engine said of the run is kept, for a script to read.
said = openclare.last_report
assert int(said["quads"]) == len(mesh.polygons) and int(said["pieces"]) == 3, said
assert float(said["seconds"]) > 0.0, said

# Each stage is announced as it begins, and a host can stop the run.
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
source = evaluated.to_mesh()
stages = []
openclare.quads_of(source, "organic", {"quads": 800}, on_stage=lambda stage: stages.append(stage))
print("STAGES", ", ".join(stages))
# Suzanne is a head and two eyes: three pieces, each with its stages in
# their order, those of one piece among those of the others when the pieces
# are meshed at the same time.
for piece in ("piece 1 of 3", "piece 2 of 3", "piece 3 of 3"):
    own = [stage for stage in stages if stage.startswith(piece)]
    assert own[0] == f"{piece}: preprocessing" and own[-1] == f"{piece}: fitting" and len(own) >= 8, stages
assert len(stages) == sum(stage.startswith("piece ") for stage in stages), stages
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

# Clicked in Blender, the engine runs in a thread of its own on arrays read
# beforehand (what the modal operators do; their events need a window, so
# the thread is driven here by hand). The mesh read is let go at once: the
# run no longer needs it. It makes the same mesh, tells its stages, and
# stops when asked.
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
source = evaluated.to_mesh()
given = openclare.engine_input(source, "organic", {"quads": 800}, copy=True)
evaluated.to_mesh_clear()
job = openclare.Run(given)
job.thread.start()
job.thread.join()
assert job.error is None, job.error
apart, _ = openclare.mesh_of(job.made, given)
assert all(a.co == b.co for a, b in zip(direct.vertices, apart.vertices)), "the run in a thread differs"
new = job.news()
assert new and new == job.stages and job.stage == new[-1] and job.done == 1.0, (new, job.done)
assert job.news() == [], "stages told twice"
job = openclare.Run(given)
job.stop.set()
job.thread.start()
job.thread.join()
assert job.made is None and str(job.error) == "stopped on request", job.error
assert openclare.progress_of([]) == 0.0 and openclare.progress_of(["cross field"]) == 1 / openclare.STAGES
assert openclare.progress_of(["piece 2 of 4: preprocessing", "piece 1 of 4: cross field"]) == 2 / (4 * 9)
# Run from a script, even as a click would, the operator waits for the
# engine: here, in the background, there is no window to come back to.
assert bpy.ops.object.openclare_remesh.poll() and openclare.RUNNING is None
print("IN A THREAD OK")

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
settings = original.openclare
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

# A group that is not there is said.
bpy.context.active_object.select_set(False)
bpy.context.view_layer.objects.active = original
original.select_set(True)
settings.density_group = "no such group"
try:
    bpy.ops.object.openclare_remesh()
    raise AssertionError("a vertex group that does not exist went unnoticed")
except RuntimeError as error:
    assert "no such group" in str(error)
assert not openclare.last_report, "a run that failed left a report"
settings.density_group = ""

# The presets of the shape: those the add-on ships set what the engine's
# presets of the same names are, and are applied the way the menu of the
# panel applies them.
import os

shipped = os.path.join(openclare.ADDON_FOLDER, "presets", openclare.SHAPE_PRESETS)
assert shipped in bpy.utils.preset_paths(openclare.SHAPE_PRESETS), bpy.utils.preset_paths(openclare.SHAPE_PRESETS)
files = {"organic": "Organic.py", "animation": "Animation.py", "hard-surface": "Hard_Surface.py"}
assert sorted(f for f in os.listdir(shipped) if f.endswith(".py")) == sorted(files.values()), os.listdir(shipped)
for name, file in files.items():
    with bpy.context.temp_override(object=original):
        assert bpy.ops.script.execute_preset(
            filepath=os.path.join(shipped, file), menu_idname="OPENCLARE_PT_shape_presets"
        ) == {"FINISHED"}
    for key, value in openclare.SHAPES[name].items():
        assert abs(getattr(settings, key) - value) < 1e-6, (name, key, getattr(settings, key), value)
# What the engine is given: the hard edges at 30 degrees, or none.
shape(settings, "organic")
assert openclare.shape_of(settings)["edge_angle"] == 30.0 and openclare.shape_of(settings)["alignment"] == 0.05
settings.use_hard_edges = False
assert openclare.shape_of(settings)["edge_angle"] == "none"
shape(settings, "hard-surface")
assert openclare.shape_of(settings)["edge_angle"] == 30.0 and openclare.shape_of(settings)["quads_around"] == 0
# Saved, removed; those of the add-on cannot be.
with tempfile.TemporaryDirectory() as folder:
    saved = os.path.join(folder, "presets", openclare.SHAPE_PRESETS)
    real_resource = bpy.utils.user_resource
    bpy.utils.user_resource = lambda kind, path="", create=False: (
        os.makedirs(os.path.join(folder, path), exist_ok=True) or os.path.join(folder, path)
    )
    try:
        settings.quads_around = 11
        with bpy.context.temp_override(object=original):
            assert bpy.ops.object.openclare_shape_preset_add(name="Eleven Around") == {"FINISHED"}
        written = open(os.path.join(saved, "Eleven_Around.py")).read()
        assert "settings.quads_around = 11" in written and "settings = bpy.context.object.openclare" in written
        assert "settings.quads " not in written and "settings.export" not in written, written
    finally:
        bpy.utils.user_resource = real_resource
# No setting takes keyframes.
assert not any(p.is_animatable for p in openclare.OpenclareSettings.bl_rna.properties if p.identifier != "rna_type")
shape(settings, "organic")


def alone(target):
    """Makes `target` the one selected and active object. A mesh is given
    the settings of the last one: the sections below go on from what was
    set before, each object keeping its own being checked apart."""
    global settings
    for other in bpy.context.selected_objects:
        other.select_set(False)
    bpy.context.view_layer.objects.active = target
    target.select_set(True)
    if target.type == "MESH":
        if settings.id_data != target:
            openclare.copy_settings(settings, target.openclare)
        settings = target.openclare


# Materials: a flat grid whose left third is of another material. The result
# has both, each quad that of the surface under it, and no quad across the
# border between the two.
bpy.ops.mesh.primitive_grid_add(x_subdivisions=24, y_subdivisions=24, size=2.0, location=(8.0, 0.0, 0.0))
sheet = bpy.context.active_object
for name in ("body", "trim"):
    sheet.data.materials.append(bpy.data.materials.new(name))
border = -1.0 + 2.0 * 8 / 24
for polygon in sheet.data.polygons:
    polygon.material_index = int(polygon.center.x < border)
alone(sheet)
settings.quads, settings.adaptive_size = 400, 0.0
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
assert [material.name for material in mesh.materials] == ["body", "trim"]
used = {polygon.material_index for polygon in mesh.polygons}
assert used == {0, 1}, f"materials used: {used}"
for polygon in mesh.polygons:
    xs = [mesh.vertices[v].co.x for v in polygon.vertices]
    left, right = all(x <= border + 1e-5 for x in xs), all(x >= border - 1e-5 for x in xs)
    assert left or right, f"a quad from {min(xs):.3f} to {max(xs):.3f} is across the border at {border:.3f}"
    assert polygon.material_index == int(left), "a quad has the material of the other side"
print("MATERIALS", len(mesh.polygons), "quads,", sum(p.material_index for p in mesh.polygons), "of the second material")
# Not asked for: the result has no material.
settings.export_materials = False
alone(sheet)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert len(bpy.context.active_object.data.materials) == 0
settings.export_materials = True
settings.quads, settings.adaptive_size = 800, 50.0

# A ring of wire beside a ball, in one object. The ring is thin all along:
# asked for a number of quads, the engine gives it those its thickness
# calls for out of that number.
bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=4, radius=1.0, location=(12.0, 0.0, 0.0))
body = bpy.context.active_object
bpy.ops.mesh.primitive_torus_add(
    major_radius=0.5, minor_radius=0.01, major_segments=96, minor_segments=12, location=(15.0, 0.0, 0.0)
)
wire = bpy.context.active_object
alone(body)
wire.select_set(True)
bpy.ops.object.join()
body = bpy.context.active_object
settings.quads, settings.adaptive_size = 600, 0.0
alone(body)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
kept = len(bpy.context.active_object.data.polygons)
# The number asked for is kept; the wire has the quads its thickness calls
# for, taken from the ball, which keeps half of the number.
assert openclare.last_report["thin_pieces"] == "1", openclare.last_report
assert 540 <= kept <= 660, kept
assert 250 <= int(openclare.last_report["quads_for_thin_pieces"]) <= 330, openclare.last_report
on_the_wire = sum(1 for polygon in bpy.context.active_object.data.polygons if polygon.center.x > 2.0)
assert on_the_wire > 250, on_the_wire

# Sizes asked for instead of a number: set from the object when none is,
# given to the engine as lengths of the mesh, and the number of quads is
# what the shape calls for.
alone(body)
assert settings.target == "COUNT" and settings.largest_quad == 0.0
settings.target = "SIZE"
largest, smallest = openclare.sizes_from(body)
assert abs(settings.largest_quad - largest) < 1e-6 and abs(settings.smallest_quad - smallest) < 1e-6
assert abs(largest / smallest - 8.0) < 1e-6 and 0.05 < largest < 0.2, (largest, smallest)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
sized = bpy.context.active_object
report = dict(openclare.last_report)
assert "quads_asked" not in report, report
assert abs(float(report["largest_asked"]) - largest) < 1e-6, report
made, wanted = len(sized.data.polygons), float(report["quads_wanted"])
assert wanted > 1000 and abs(made / wanted - 1.0) < 0.15, (made, wanted)
# Twice the size, the object scaled by two: the same mesh is asked for.
body.scale = (2.0, 2.0, 2.0)
bpy.context.view_layer.update()
settings.largest_quad, settings.smallest_quad = 2.0 * largest, 2.0 * smallest
alone(body)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert abs(float(openclare.last_report["largest_asked"]) - largest) < 1e-6, openclare.last_report
assert len(bpy.context.active_object.data.polygons) == made
body.scale = (1.0, 1.0, 1.0)
bpy.context.view_layer.update()
# Larger quads make fewer of them.
settings.largest_quad, settings.smallest_quad = 2.0 * largest, 2.0 * smallest
alone(body)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert len(bpy.context.active_object.data.polygons) < 0.5 * made
settings.target, settings.largest_quad, settings.smallest_quad = "COUNT", 0.0, 0.0

# Nothing to remesh: said, not crashed.
bpy.ops.object.empty_add()
assert not bpy.ops.object.openclare_remesh.poll()

# Each object keeps its own settings.
bpy.ops.mesh.primitive_cube_add(location=(30.0, 0.0, 0.0))
first = bpy.context.active_object
bpy.ops.mesh.primitive_cube_add(location=(33.0, 0.0, 0.0))
second = bpy.context.active_object
first.openclare.quads, second.openclare.quads = 200, 500
shape(first.openclare, "hard-surface")
assert second.openclare.quads_around == 6 and first.openclare.quads == 200

# The panel is in the Properties editor, on the tab of the mesh: for an
# object to remesh, its settings; for a result, what it comes from.
panels = (openclare.OPENCLARE_PT_panel, openclare.OPENCLARE_PT_shape)
assert all((p.bl_space_type, p.bl_context) == ("PROPERTIES", "data") for p in panels)
assert not hasattr(bpy.types, "VIEW3D_PT_openclare") and all(p.bl_region_type == "WINDOW" for p in panels)
alone(second)
second.openclare.quads = 500
assert openclare.is_source(bpy.context) and openclare.OPENCLARE_PT_shape.poll(bpy.context)
assert not bpy.ops.object.openclare_remesh_again.poll()
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
result = bpy.context.active_object
assert openclare.source_of(result) == second and not openclare.is_source(bpy.context)
assert not openclare.OPENCLARE_PT_shape.poll(bpy.context)
# How it went is kept on the result, and said by its panel.
told = result[openclare.REPORT]
assert int(told["quads"]) == len(result.data.polygons), dict(told)
lines = openclare.report_lines(told)
assert lines[0][0].startswith(f"{len(result.data.polygons)} quads in "), lines

# Remeshed again, with the settings the original has now, in place: the
# same object, name and place, a new mesh.
made_before = len(result.data.polygons)
result.location.x += 1.0
second.openclare.quads = 1500
objects_before = len(bpy.data.objects)
assert bpy.ops.object.openclare_remesh_again() == {"FINISHED"}
assert bpy.context.active_object == result and len(bpy.data.objects) == objects_before
assert len(result.data.polygons) > 2 * made_before and int(result[openclare.REPORT]["quads"]) == len(result.data.polygons)
assert result.location.x == 34.0
assert result.data.name == f"{second.data.name}.quads", result.data.name
# And its original selected from it.
assert bpy.ops.object.openclare_select_original() == {"FINISHED"}
assert bpy.context.active_object == second and not result.select_get()

# A scene of openclare 0.1 kept the settings in the scene, naming the
# object: on opening, that object takes them, and the scene keeps nothing.
legacy = bpy.context.scene.openclare
legacy.source, legacy.quads, legacy.preset, legacy.adaptive_size = first, 1234, "animation", 20.0
openclare.adopt_scene_settings(bpy.context.scene)
# The preset it named is turned into its values; its adaptive size is its own.
assert (first.openclare.quads, first.openclare.fewer_irregular, first.openclare.adaptive_size) == (1234, 3.0, 20.0)
assert not first.openclare.is_property_set("preset")
assert second.openclare.quads == 1500, "an object not named took the settings"
assert not legacy.is_property_set("quads") and legacy.source is None
assert bpy.context.view_layer.objects.active == first
# Naming none: each mesh that has no settings of its own takes them.
bpy.ops.mesh.primitive_cube_add(location=(36.0, 0.0, 0.0))
plain = bpy.context.active_object
legacy.quads = 4321
openclare.adopt_scene_settings(bpy.context.scene)
assert plain.openclare.quads == 4321 and first.openclare.quads == 1234 and second.openclare.quads == 1500
assert result.openclare.quads != 4321, "a result took settings to remesh"
print("PANEL OK")

# The engine is loaded from a copy named after what it holds: an add-on
# updated in a Blender that has already used it does not get the engine it
# had in memory back, by its path, in place of the new one.
import tempfile
from pathlib import Path

with tempfile.TemporaryDirectory() as folder:
    installed = Path(folder) / "libopenclare_test.so"
    installed.write_bytes(b"the engine as it was")
    before = openclare.engine._own_copy(installed)
    assert before.read_bytes() == b"the engine as it was" and openclare.engine._own_copy(installed) == before
    installed.write_bytes(b"the engine after an update")
    after = openclare.engine._own_copy(installed)
    assert after != before and after.read_bytes() == b"the engine after an update"
    assert not before.exists(), "the copy of the version before is left behind"
    after.unlink()

openclare.unregister()
print("ADD-ON OK")
