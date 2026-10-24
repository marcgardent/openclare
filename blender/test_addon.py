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
    """Gives `settings` the values of the shape `name` of the engine."""
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

# The presets of the shape: the add-on ships none since 0.6, and the menu of
# the panel only lists those a user saved.
import os

addon_folder = os.path.dirname(os.path.abspath(openclare.__file__))
assert not os.path.exists(os.path.join(addon_folder, "presets")), "presets are shipped again"
assert addon_folder not in [os.path.dirname(os.path.dirname(path)) for path in bpy.utils.preset_paths(openclare.SHAPE_PRESETS)]
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
        assert "settings.use_marked_sharp = False" in written, written
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

# The hard edges shown in the viewport. Drawing needs a window, which there
# is not here: what is drawn is checked, and the steps the timer takes are
# taken by hand, the background thread waited for.
preview = openclare.preview
wm = bpy.context.window_manager
assert not wm.openclare_show_hard_edges and preview.state.handle is None
bpy.ops.mesh.primitive_cube_add(location=(40.0, 0.0, 0.0))
box = bpy.context.active_object
alone(box)
settings.target, settings.quads, settings.use_hard_edges = "COUNT", 3000, True
wm.openclare_show_hard_edges = True


def preview_settles(start=0.0):
    """Takes the steps of the timer until the edges of the object shown are
    found and drawn, or nothing is to be: what a change waits out, the run
    in the background, the drawing. Checks the status on the way: waiting,
    computing, up to date; the lines drawn before dimmed until then."""
    now = start
    preview.step(now)
    assert preview.state.job is None, "the edges were looked for without waiting"
    assert preview.state.status == preview.WAITING, preview.state.status
    assert preview.state.shown is None or preview.state.stale, "lines out of date drawn in full"
    for _ in range(20):
        now += preview.TICK
        preview.step(now)
        if preview.state.job is None and preview.state.status == preview.FAILED:
            # Refused before the engine was called.
            return
        if preview.state.job is not None:
            assert now - start >= preview.QUIET_SETTINGS
            assert preview.state.status == preview.COMPUTING, preview.state.status
            assert preview.status_text() == "Computing\u2026"
            preview.state.job.thread.join()
            assert preview.step(now + 0.01) == preview.TICK, "still busy once the edges are in"
            assert preview.state.job is None and not preview.state.pending
            assert preview.state.status in (preview.CURRENT, preview.FAILED), preview.state.status
            assert not preview.state.stale
            return
    raise AssertionError("the edges were never looked for")


def edges_of(pairs):
    return {tuple(sorted(pair)) for pair in pairs.reshape(-1, 2).tolist()}


preview_settles()
shown = preview.state.shown
assert shown is not None and shown.name == box.name and preview.state.handle is not None, preview.state.error
# The twelve edges of the cube, in the vertices of the object.
cube_edges = {tuple(sorted(edge.vertices)) for edge in box.data.edges}
assert edges_of(shown.kept) == cube_edges and len(shown.left_out) == 0, (edges_of(shown.kept), cube_edges)
assert len(shown.kept_points) == 24 and all(
    abs(abs(c) - 1.0) < 1e-6 for point in shown.kept_points for c in point
), "the lines are not on the edges of the cube"
# Drawn off screen, where this Blender can draw without a window: over a
# grey cube seen from above a corner, the edges kept come out cyan, the
# three at the back hidden behind the faces, those in front not sunk into
# them.
import math

import gpu
import mathutils


def cyan_pixels(with_faces, stale=False):
    import math

    import numpy
    from gpu_extras.batch import batch_for_shader

    size, eye = 200, 6.0
    offscreen = gpu.types.GPUOffScreen(size, size)
    view = mathutils.Matrix.Translation((0.0, 0.0, -eye)) @ (
        mathutils.Matrix.Rotation(math.radians(-55.0), 4, "X") @ mathutils.Matrix.Rotation(math.radians(-45.0), 4, "Z")
    )
    near, far, focal = 0.1, 100.0, 1.0 / math.tan(math.radians(25.0))
    projection = mathutils.Matrix(
        (
            (focal, 0.0, 0.0, 0.0),
            (0.0, focal, 0.0, 0.0),
            (0.0, 0.0, (far + near) / (near - far), 2.0 * far * near / (near - far)),
            (0.0, 0.0, -1.0, 0.0),
        )
    )
    with offscreen.bind():
        framebuffer = gpu.state.active_framebuffer_get()
        framebuffer.clear(color=(0.2, 0.2, 0.2, 1.0), depth=1.0)
        with gpu.matrix.push_pop(), gpu.matrix.push_pop_projection():
            gpu.matrix.load_matrix(view)
            gpu.matrix.load_projection_matrix(projection)
            if with_faces:
                mesh = box.data
                mesh.calc_loop_triangles()
                corners = [mesh.vertices[v].co[:] for triangle in mesh.loop_triangles for v in triangle.vertices]
                faces = gpu.shader.from_builtin("UNIFORM_COLOR")
                batch = batch_for_shader(faces, "TRIS", {"pos": corners})
                gpu.state.depth_test_set("LESS_EQUAL")
                gpu.state.depth_mask_set(True)
                faces.bind()
                faces.uniform_float("color", (0.6, 0.6, 0.6, 1.0))
                batch.draw(faces)
            preview.draw_lines(preview.state.shown, mathutils.Matrix.Identity(4), (size, size), eye, stale)
        pixels = numpy.array(framebuffer.read_color(0, 0, size, size, 4, 0, "FLOAT").to_list()).reshape(-1, 4)
    offscreen.free()
    gpu.state.depth_test_set("NONE")
    return int(((pixels[:, 0] < 0.3) & (pixels[:, 1] > 0.6) & (pixels[:, 2] > 0.8)).sum())


try:
    gpu.init()
    drawable = True
except Exception as error:  # noqa: BLE001
    print("PREVIEW not drawn off screen:", error)
    drawable = False
if drawable:
    alone_lines, over_faces = cyan_pixels(False), cyan_pixels(True)
    print("PREVIEW drawn: cyan pixels", alone_lines, "alone,", over_faces, "over the faces")
    assert over_faces > 300, over_faces
    assert over_faces < 0.85 * alone_lines, (over_faces, alone_lines)
    # Out of date, the lines are dimmed: hardly any pixel comes out as cyan
    # as they are in full.
    dimmed = cyan_pixels(False, stale=True)
    print("PREVIEW drawn dimmed: cyan pixels", dimmed)
    assert dimmed < 0.2 * alone_lines, (dimmed, alone_lines)

# Up to date: said with the counts, nothing redrawn while nothing happens,
# nothing said in the viewport.
assert preview.state.status == preview.CURRENT and preview.state.text_handle is None
assert preview.status_text() == "12 followed, 0 left out", preview.status_text()
redrawn = []
real_redraw, preview.redraw = preview.redraw, redrawn.append
for moment in range(5):
    assert preview.step(5.0 + 0.1 * moment) == preview.TICK
preview.redraw = real_redraw
assert redrawn == [], redrawn
# The shape read once: a change of the settings is found on the shape the
# engine was given, after a shorter pause.
shape_given = preview.state.shape
assert shape_given is not None and shape_given.key == preview.shape_key_of(box)

# A change while the edges of the one before are being computed: those are
# dropped when they come, never drawn as up to date, and the latest change
# is computed after them.
settings.hard_edge_angle = math.radians(60.0)
preview.step(6.0)
preview.step(6.2)
assert preview.state.status == preview.COMPUTING and preview.state.job is not None
assert preview.state.stale, "the lines of the angle before are not dimmed"
older = preview.state.job
settings.quads = 30
older.thread.join()
preview.step(6.25)
assert older.found is not None and preview.state.job is None
assert preview.state.status == preview.WAITING and preview.state.stale, "edges of an older change shown"
assert preview.state.shape is shape_given
preview.step(6.3)
assert preview.state.job is None, "the latest change not waited out"
preview.step(6.4)
assert preview.state.status == preview.COMPUTING and preview.state.job is not older
assert preview.state.job.shape is shape_given, "the mesh read again for a change of the settings"
preview.state.job.thread.join()
preview.step(6.45)
assert preview.state.status == preview.CURRENT and not preview.state.stale
settings.hard_edge_angle, settings.quads = math.radians(30.0), 3000

# A computation that lasts is said in the viewport too, and no longer once
# done.
settings.hard_edge_angle = math.radians(45.0)
preview.step(8.0)
preview.step(8.0 + preview.SAID_IN_VIEWPORT_AFTER)
assert preview.state.status == preview.COMPUTING and preview.state.text_handle is not None
preview.state.job.thread.join()
preview.step(8.6)
assert preview.state.status == preview.CURRENT and preview.state.text_handle is None
settings.hard_edge_angle = math.radians(30.0)
preview_settles(9.5)



class Drawn:
    """A layout that keeps what is drawn on it: what the panel says of the
    preview is read off it, there being no window here."""

    def __init__(self, log):
        self.log = log

    def row(self, **_):
        return Drawn(self.log)

    column = row

    def prop(self, owner, name, **options):
        self.log.append(("prop", name, options.get("icon")))

    def label(self, text="", icon="NONE"):
        self.log.append(("label", text, icon))

    def progress(self, **options):
        parameters = {p.identifier for p in bpy.types.UILayout.bl_rna.functions["progress"].parameters}
        assert set(options) <= parameters, options
        self.log.append(("progress", options.get("type"), options.get("text")))


def panel_says():
    import types

    log = []
    openclare.OPENCLARE_PT_shape.draw(types.SimpleNamespace(layout=Drawn(log)), bpy.context)
    return [entry for entry in log if entry[0] != "prop" or entry[1] == "openclare_show_hard_edges"]


assert panel_says() == [("prop", "openclare_show_hard_edges", "HIDE_OFF"), ("label", "", "CHECKMARK"),
                        ("label", "12 followed, 0 left out", "NONE")], panel_says()
settings.hard_edge_angle = math.radians(50.0)
preview.step(9.6)
assert panel_says()[1:] == [("progress", "RING", ""), ("label", "Computing\u2026", "NONE")], panel_says()
settings.hard_edge_angle = math.radians(30.0)

# What goes wrong is said, and nothing drawn; set right, all is drawn again.
settings.density_group = "no such group"
preview_settles(9.7)
assert preview.state.status == preview.FAILED and preview.state.shown is None and preview.state.handle is None
assert "no such group" in preview.status_text(), preview.status_text()
assert panel_says()[1] == ("label", "", "ERROR") and "no such group" in panel_says()[2][1], panel_says()
settings.density_group = ""
preview_settles(9.9)
assert preview.state.status == preview.CURRENT and edges_of(preview.state.shown.kept) == cube_edges

# Hard edges turned off: nothing kept, after the pause.
settings.use_hard_edges = False
preview_settles(10.0)
assert len(preview.state.shown.kept) == 0 and len(preview.state.shown.left_out) == 0
settings.use_hard_edges = True
preview_settles(20.0)
assert edges_of(preview.state.shown.kept) == cube_edges
# An object with modifiers: the mesh they make is the one read, as for the
# remesh, and the edges are of its vertices.
alone(original)
preview_settles(30.0)
shown = preview.state.shown
assert shown is not None and shown.name == original.name, preview.state.error
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
vertices = len(evaluated.to_mesh().vertices)
evaluated.to_mesh_clear()
assert all(int(v) < vertices for v in list(shown.kept) + list(shown.left_out)), vertices
print("PREVIEW of the head:", len(shown.kept) // 2, "edges kept,", len(shown.left_out) // 2, "left out")
# Another object made the active one: what was drawn goes at once.
alone(box)
preview.step(40.0)
assert preview.state.shown is None and preview.state.handle is None
# The object deleted while shown: nothing drawn, nothing left behind.
preview_settles(40.0)
assert preview.state.handle is not None
bpy.data.objects.remove(box)
preview.step(60.0)
assert preview.state.shown is None and preview.state.handle is None
# Turned off: nothing drawn; on again for what follows.
settings = original.openclare
alone(original)
preview_settles(70.0)
wm.openclare_show_hard_edges = False
assert preview.state.shown is None and preview.state.handle is None
assert not bpy.app.timers.is_registered(preview.tick)
wm.openclare_show_hard_edges = True
assert bpy.app.timers.is_registered(preview.tick)
preview_settles(80.0)
assert preview.state.handle is not None
print("PREVIEW OK")

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

# Edges marked sharp: a smooth sphere whose equator is marked. Asked to, the
# quads follow it whatever its angle, and the preview shows it as kept.
bpy.ops.mesh.primitive_uv_sphere_add(segments=48, ring_count=24, location=(60.0, 0.0, 0.0))
ball = bpy.context.active_object
equator = [edge.index for edge in ball.data.edges
           if all(abs(ball.data.vertices[v].co.z) < 1e-5 for v in edge.vertices)]
assert len(equator) == 48, len(equator)
for index in equator:
    ball.data.edges[index].use_edge_sharp = True
alone(ball)
settings.target, settings.quads, settings.use_hard_edges = "COUNT", 800, False


def on_equator(result):
    return sum(1 for vertex in result.data.vertices if abs(vertex.co.z) < 1e-4)


settings.use_marked_sharp = False
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
plain = on_equator(bpy.context.active_object)
alone(ball)
settings.use_marked_sharp = True
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
followed = on_equator(bpy.context.active_object)
assert plain < 10 <= 30 <= followed, (plain, followed)
evaluated = ball.evaluated_get(bpy.context.evaluated_depsgraph_get())
marked = openclare.marked_sharp(evaluated.to_mesh())
evaluated.to_mesh_clear()
assert marked is not None and len(marked) == 2 * 48
import numpy

kept, left_out = preview.with_marked(numpy.array([0, 1], numpy.uint32), numpy.array([2, 3, 4, 5], numpy.uint32),
                                     numpy.array([3, 2, 7, 8], numpy.uint32))
assert sorted(kept.reshape(-1, 2).tolist()) == [[0, 1], [3, 2], [7, 8]] and left_out.tolist() == [4, 5], (kept, left_out)
print("MARKED SHARP: the equator held by", followed, "vertices, against", plain, "without")

# A scale that differs along the axes: the quads come out square in the
# scene, not stretched along the axis scaled most.
bpy.ops.mesh.primitive_cube_add(location=(66.0, 0.0, 0.0))
tall = bpy.context.active_object
tall.modifiers.new("subdivide", "SUBSURF").subdivision_type = "SIMPLE"
tall.modifiers["subdivide"].levels = 4
tall.scale = (1.0, 1.0, 3.0)
alone(tall)
settings.use_marked_sharp, settings.use_hard_edges, settings.quads = False, True, 700
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
result = bpy.context.active_object
world = [result.matrix_world @ vertex.co for vertex in result.data.vertices]
ratios = []
for polygon in result.data.polygons:
    sides = [(world[polygon.vertices[k]] - world[polygon.vertices[(k + 1) % 4]]).length for k in range(4)]
    ratios.append(max(sides) / min(sides))
ratios.sort()
assert ratios[len(ratios) // 2] < 1.5, f"quads stretched {ratios[len(ratios) // 2]:.2f} times in the scene"
assert abs(max(v.z for v in world) - (0.0 + 3.0)) < 0.05, "the result is not where the scaled cube is"
print(f"STRETCH: median quad {ratios[len(ratios) // 2]:.2f} long for wide under a scale of 1, 1, 3")

# A result far short of the number asked for is said, as a warning and in
# the panel of the result.
told = {"quads_asked": "3000", "quads": "164", "pieces": "3", "pieces_failed": "0", "seconds": "1.0"}
assert "only 164 quads of the 3000" in openclare.falls_short(told)
assert any("Only 164" in line for line, _ in openclare.report_lines(told))
assert not openclare.falls_short({"quads_asked": "3000", "quads": "2900"})
assert not openclare.falls_short({"largest_asked": "0.1", "quads": "164"})
print("FALLS SHORT OK")

openclare.unregister()
# The drawing goes with the add-on.
assert preview.state.handle is None and preview.state.shown is None
assert not bpy.app.timers.is_registered(preview.tick)
assert not hasattr(bpy.types.WindowManager, "openclare_show_hard_edges")
print("ADD-ON OK")
