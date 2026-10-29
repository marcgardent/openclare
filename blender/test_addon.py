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
# A bare mesh: no UV layer, no seam; the only edges marked sharp are those
# of its shading, By Angle (checked on a cube below).
assert len(mesh.uv_layers) == 0 and not any(edge.use_seam for edge in mesh.edges)
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

# The presets of the shape: the add-on ships one, Default, which gives every
# setting of the shape its default value; the others are those a user saved.
import os

shipped = os.path.join(openclare.ADDON_FOLDER, "presets", openclare.SHAPE_PRESETS)
assert shipped in bpy.utils.preset_paths(openclare.SHAPE_PRESETS), bpy.utils.preset_paths(openclare.SHAPE_PRESETS)
assert sorted(f for f in os.listdir(shipped) if f.endswith(".py")) == ["Default.py"], os.listdir(shipped)
properties = openclare.OpenclareSettings.bl_rna.properties
settings.quads_around, settings.shading, settings.use_marked_sharp = 11, "FLAT", True
with bpy.context.temp_override(object=original):
    assert bpy.ops.script.execute_preset(
        filepath=os.path.join(shipped, "Default.py"), menu_idname="OPENCLARE_PT_shape_presets"
    ) == {"FINISHED"}
for name in openclare.SHAPE_NAMES:
    wanted = properties[name].default
    got = getattr(settings, name)
    assert got == wanted or abs(got - wanted) < 1e-6, (name, got, wanted)
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
        assert "settings.shading = 'BY_ANGLE'" in written, written
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


import math  # noqa: E402
import time  # noqa: E402

# Marks: on a shape with hard edges, those edges marked sharp and creased.
bpy.ops.mesh.primitive_cube_add()
cube = bpy.context.active_object
cube.modifiers.new("subdivide", "SUBSURF").subdivision_type = "SIMPLE"
cube.modifiers["subdivide"].levels = 4
alone(cube)
shape(settings, "hard-surface")
settings.quads = 600
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
sharp = sum(edge.use_edge_sharp for edge in mesh.edges)
creases = [item.value for item in mesh.attributes["crease_edge"].data]
creased = sum(1 for value in creases if value > 0)
print("MARK cube:", len(mesh.polygons), "quads,", sharp, "sharp edges,", creased, "creased")
assert sharp >= 12 and sharp % 12 == 0, f"{sharp} sharp edges on a cube"
# The same edges are creased, all the way, and they are the sharp ones.
assert creased == sharp and all(value in (0.0, 1.0) for value in creases)
assert all((edge.use_edge_sharp) == (creases[edge.index] == 1.0) for edge in mesh.edges)

# With the boxes unticked, the result is a bare mesh.
alone(cube)
settings.export_sharp_edges = settings.export_creases = False
# Shaded smooth: the shading by angle marks the edges of the cube itself.
settings.shading = "SMOOTH"
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
assert not any(edge.use_edge_sharp for edge in mesh.edges)
assert "crease_edge" not in mesh.attributes
settings.export_sharp_edges = settings.export_creases = True
settings.shading = "BY_ANGLE"
shape(settings, "organic")
settings.quads = 800

# Curves: two circles drawn round a ball, each held on it by a Shrinkwrap.
# One is put in the Guide category and the other in the Sharp one, with the
# buttons of the panel. The result has an edge loop on each, and the edges of
# the second are marked sharp and creased.
bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=5, radius=1.0, location=(4.0, 0.0, 0.0))
ball = bpy.context.active_object


def circle_at(height):
    bpy.ops.curve.primitive_bezier_circle_add(radius=1.1, location=(4.0, 0.0, height))
    circle = bpy.context.active_object
    circle.data.resolution_u = 16
    circle.modifiers.new("on the ball", "SHRINKWRAP").target = ball
    return circle


def with_the_ball(circle):
    """The circle selected, and the ball last: the active object, whose
    panel the buttons are on."""
    for other in bpy.context.selected_objects:
        other.select_set(False)
    circle.select_set(True)
    ball.select_set(True)
    bpy.context.view_layer.objects.active = ball


def assign(circle, category):
    with_the_ball(circle)
    assert bpy.ops.object.openclare_curves_assign(category=category) == {"FINISHED"}


loop, ridge = circle_at(0.3), circle_at(-0.3)
alone(ball)
assign(loop, "guide")
assign(ridge, "sharp")
guides, sharps = settings.guide_curves, settings.sharp_curves
assert guides is not None and sharps is not None and guides != sharps
assert list(guides.objects) == [loop] and list(sharps.objects) == [ridge]


def coloured(drawn, colour):
    return all(abs(got - wanted) < 1e-6 for got, wanted in zip(drawn.color, colour))


assert coloured(loop, openclare.CATEGORY["guide"][1]) and coloured(ridge, openclare.CATEGORY["sharp"][1])
assert not coloured(loop, openclare.CATEGORY["sharp"][1])

# Moving a curve to the other category takes it out of the first.
assign(loop, "sharp")
assert list(guides.objects) == [] and set(sharps.objects) == {loop, ridge}
assign(loop, "guide")
assert list(guides.objects) == [loop] and list(sharps.objects) == [ridge]

# Select and Deselect act on the curves of one category.
alone(ball)
assert bpy.ops.object.openclare_curves_select(category="sharp", select=True) == {"FINISHED"}
assert ridge.select_get() and not loop.select_get()
assert bpy.context.view_layer.objects.active == ball, "the ball is no longer active: its panel would go"
assert bpy.ops.object.openclare_curves_select(category="sharp", select=False) == {"FINISHED"}
assert not ridge.select_get()

alone(ball)
settings.quads, settings.adaptive_size = 600, 0.0
curves = openclare.curves_of(guides, ball)
assert len(curves) == 1 and curves[0][1] and not curves[0][2], "one closed guide curve expected"
assert all(abs(float((point ** 2).sum()) ** 0.5 - 1.0) < 0.02 for point in curves[0][0]), "the circle is not on the ball"
assert openclare.curves_of(sharps, ball, sharp=True)[0][2]
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
# The Shrinkwrap brought each circle to the nearest points of the ball, a
# circle of latitude; the loop is at its height.
height = sum(float(point[2]) for point in curves[0][0]) / len(curves[0][0])
on_the_loop = [vertex.index for vertex in mesh.vertices if abs(vertex.co.z - height) < 0.01]
on_the_ridge = [vertex.index for vertex in mesh.vertices if abs(vertex.co.z + height) < 0.01]
print("CURVES a loop of", len(on_the_loop), "vertices at height %.3f," % height, len(on_the_ridge), "on the ridge")
assert len(on_the_loop) >= 12 and len(on_the_ridge) >= 12, "the curves have no loop of edges"
# Sharp and creased along the ridge, and nowhere else.
sharp = [edge for edge in mesh.edges if edge.use_edge_sharp]
weights = mesh.attributes["crease_edge"].data
assert len(sharp) >= 12, f"{len(sharp)} sharp edges"
assert all(set(edge.vertices) <= set(on_the_ridge) for edge in sharp), "a sharp edge off the ridge"
assert all(weights[edge.index].value == 1.0 for edge in sharp), "a sharp edge is not creased in full"
assert sum(1 for weight in weights if weight.value > 0) == len(sharp)

# Under a scale that is not the same on every axis the engine is given the
# ball stretched, and the curves stretched with it: the loops stay on them.
ball.scale = (1.0, 1.0, 2.0)
bpy.context.view_layer.update()
curves = openclare.curves_of(guides, ball)
height = sum(float(point[2]) for point in curves[0][0]) / len(curves[0][0])
alone(ball)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
stretched = [vertex.index for vertex in mesh.vertices if abs(vertex.co.z - height) < 0.01]
print("CURVES STRETCHED a loop of", len(stretched), "vertices at height %.3f" % height)
assert len(stretched) >= 12, "the guide curve is lost under a scale of 1, 1, 2"
ball.scale = (1.0, 1.0, 1.0)
bpy.context.view_layer.update()

# Remove takes a curve out of its category and gives it back its colour.
with_the_ball(ridge)
assert bpy.ops.object.openclare_curves_remove(category="guide") == {"FINISHED"}
assert list(sharps.objects) == [ridge], "taken out of a category it was not in"
assert bpy.ops.object.openclare_curves_remove(category="sharp") == {"FINISHED"}
assert list(sharps.objects) == [] and ridge.users_collection and coloured(ridge, openclare.PLAIN)
alone(ball)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert not any(edge.use_edge_sharp for edge in bpy.context.active_object.data.edges)
settings.guide_curves = settings.sharp_curves = None

# A curve drawn on the ball with the drawing tool's logic: a stroke round it
# at a height of 0.5, closed on its start, made into a curve of the Guide
# category. The result has a loop of edges on it, with no Shrinkwrap.
from openclare import trace  # noqa: E402

settings.guide_curves = None
settings.quads, settings.adaptive_size = 600, 0.0
height = 0.5
ring = (1.0 - height ** 2) ** 0.5
stroke = trace.Trace((ring, 0.0, height), 0.02, 0.05)
for degrees in range(3, 360, 3):
    stroke.move((ring * math.cos(math.radians(degrees)), ring * math.sin(math.radians(degrees)), height))
points, closed = stroke.curve()
assert closed, "the stroke round the ball did not close"
drawn = openclare.drawn_curve(bpy.context, ball, points, closed, "guide", 0.02 * 2 * 3 ** 0.5)
assert list(settings.guide_curves.objects) == [drawn] and coloured(drawn, openclare.CATEGORY["guide"][1])
# Few control points: a ring of 268 points of the stroke is two Bezier
# segments; the curve is a child of the ball, held on it by a Shrinkwrap.
spline = drawn.data.splines[0]
assert spline.type == "BEZIER" and spline.use_cyclic_u and len(spline.bezier_points) <= 4, len(spline.bezier_points)
assert drawn.parent == ball and drawn.modifiers[openclare.SHRINKWRAP].target == ball
curves = openclare.curves_of(settings.guide_curves, ball)
assert len(curves) == 1 and curves[0][1], "the drawn curve is not read back as one closed curve"
off = max(abs(float(sum(x * x for x in point)) ** 0.5 - 1.0) for point in curves[0][0])
assert off < 0.01, f"the drawn curve is {off:.4f} off the ball"
alone(ball)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
on_it = [vertex.index for vertex in mesh.vertices if abs(vertex.co.z - height) < 0.01]
print("DRAWN a stroke of", len(points), "points round the ball, a loop of", len(on_it), "vertices on it")
assert len(on_it) >= 12, "the drawn curve has no loop of edges"

# The Draw Curve tool: in the toolbar of the 3D viewport in Object Mode, with
# its operator. The mouse cannot be driven here; what it relies on can.
from bl_ui.space_toolsystem_common import ToolSelectPanelHelper  # noqa: E402

toolbar = ToolSelectPanelHelper._tool_class_from_space_type("VIEW_3D")
tools = [item for item in ToolSelectPanelHelper._tools_flatten(toolbar.tools_from_context(bpy.context, mode="OBJECT"))
         if item is not None]
assert any(item.idname == "openclare.draw_curve" for item in tools), "no Draw Curve tool in the toolbar"
# Cut Volume is the other tool of the same button.
groups = [group for group in toolbar.tools_from_context(bpy.context, mode="OBJECT") if isinstance(group, tuple)]
assert any({"openclare.draw_curve", "openclare.cut_volume"} <= {getattr(item, "idname", None) for item in group}
           for group in groups), "Draw Curve and Cut Volume are not one button"
assert hasattr(bpy.ops.object, "openclare_cut_volume")
assert hasattr(bpy.ops.object, "openclare_draw_curve")
# The tools draw on the active object if it is a selected mesh; otherwise
# they cannot, and show it by their cursor, which the timer sets through the
# tool system of Blender: what it relies on is there.
bpy.ops.object.select_all(action="DESELECT")
ball.select_set(True)
bpy.context.view_layer.objects.active = ball
assert openclare.draw.target(bpy.context) == ball and not openclare.draw.disabled_in(bpy.context.view_layer)
ball.select_set(False)
assert openclare.draw.target(bpy.context) is None and openclare.draw.disabled_in(bpy.context.view_layer)
bpy.context.view_layer.objects.active = drawn
drawn.select_set(True)
assert openclare.draw.target(bpy.context) is None and openclare.draw.disabled_in(bpy.context.view_layer)
drawn.select_set(False)
ball.select_set(True)
bpy.context.view_layer.objects.active = ball
from bl_ui.space_toolsystem_common import _activate_by_item  # noqa: E402,F401

item = next(item for item in tools if item.idname == "openclare.cut_volume")
assert item._replace(cursor=openclare.draw.DISABLED).cursor == "STOP"
assert "STOP" in {e.identifier for e in bpy.types.Window.bl_rna.functions["cursor_set"].parameters["cursor"].enum_items}
assert openclare.draw.follow_cursor() == openclare.draw.LOOK_FOR_CURSOR
draw_properties = bpy.ops.object.openclare_draw_curve.get_rna_type().properties
assert draw_properties["rope"].default == 30.0 and draw_properties["snap"].default == "NONE"
assert draw_properties["snap_radius"].default == 15.0
assert [draw_properties[name].default for name in ("width", "spacing", "back_band", "close", "fit")] == [8.0, 5.0, 30.0, 15.0, 5.0]
# The settings of the stroke are in the popover of the header, as those of
# Blender's own tools: drawn without fault in a layout of a panel.
draw_tool = next(item for item in tools if item.idname == "openclare.draw_curve")
assert "extra" in __import__("inspect").signature(draw_tool.draw_settings).parameters
assert {item.identifier for item in draw_properties["snap"].enum_items} == {"NONE", "RIDGE", "VALLEY"}
# The snap on a roof with a sharp ridge at x = 0.123, the screen seen from
# above (a pixel is 0.01): the mouse 7 pixels from the ridge, within a radius
# of 15, is moved onto it by Ridge, and stays where it is by Valley.
from mathutils.bvhtree import BVHTree  # noqa: E402

roof_tree = BVHTree.FromPolygons(
    [(-1.0, -1.0, 0.0), (0.123, -1.0, 1.0), (0.123, 1.0, 1.0), (-1.0, 1.0, 0.0), (1.0, -1.0, 0.0), (1.0, 1.0, 0.0)],
    [(0, 1, 2, 3), (1, 4, 5, 2)],
)


def from_above(context, owner, tree, mouse):
    location, normal, _, _ = tree.ray_cast((mouse[0] * 0.01, mouse[1] * 0.01, 5.0), (0.0, 0.0, -1.0))
    return (None, None) if location is None else (tuple(location), tuple(normal))


surface_at = openclare.draw.surface_at
openclare.draw.surface_at = from_above
try:
    ridge = openclare.draw.snapped(None, None, roof_tree, (5.0, 0.0), (1.0, 0.0), "RIDGE", 15.0)
    under = openclare.draw.snapped(None, None, roof_tree, (5.0, 0.0), (1.0, 0.0), "VALLEY", 15.0)
    # A gutter, its valley at x = 0.123: found by Valley.
    gutter_tree = BVHTree.FromPolygons(
        [(-1.0, -1.0, 1.0), (0.123, -1.0, 0.0), (0.123, 1.0, 0.0), (-1.0, 1.0, 1.0), (1.0, -1.0, 1.0), (1.0, 1.0, 1.0)],
        [(0, 1, 2, 3), (1, 4, 5, 2)],
    )
    valley = openclare.draw.snapped(None, None, gutter_tree, (5.0, 0.0), (1.0, 0.0), "VALLEY", 15.0)
finally:
    openclare.draw.surface_at = surface_at
assert abs(ridge[0] - 0.123) < 1e-3 and abs(ridge[2] - 1.0) < 1e-3, ridge
assert abs(valley[0] - 0.123) < 1e-3 and abs(valley[2]) < 1e-3, valley
assert abs(under[0] - 0.05) < 1e-6, under
print("SNAP onto the ridge at x = %.4f, into the valley at %.4f, for 0.123; Valley on the ridge leaves the mouse" % (ridge[0], valley[0]))
# F sets the radius of the snap, in the tool of Draw Curve.
assert hasattr(bpy.ops.object, "openclare_snap_radius")
assert any(entry[0] == "object.openclare_snap_radius" and entry[1]["type"] == "F"
           for entry in openclare.draw.OPENCLARE_TL_draw_curve.bl_keymap)
print("TARGET the selected active mesh; a curve active, or nothing selected: the cursor says no")
# The surface the strokes are laid on: a ray from outside the ball reaches it
# at its radius; the tree is kept while the shape stays as it is.
tree = openclare.draw.surface_tree(bpy.context, ball)
location, _, _, _ = tree.ray_cast((0.0, 0.0, 5.0), (0.0, 0.0, -1.0))
assert location is not None and abs(location.z - 1.0) < 0.02, location
assert openclare.draw.surface_tree(bpy.context, ball) is tree
# The tree is built again when the shape changes, and only then.
ball.data.vertices[0].co.z += 0.01
ball.data.update()
assert openclare.draw.surface_tree(bpy.context, ball) is not tree, "the tree outlived a change of the shape"
ball.data.vertices[0].co.z -= 0.01
ball.data.update()
tree = openclare.draw.surface_tree(bpy.context, ball)
assert openclare.draw.surface_tree(bpy.context, ball) is tree
assert abs(openclare.draw.step_of(ball) - 0.01 * 2 * 3 ** 0.5) < 1e-3, openclare.draw.step_of(ball)
# The curve drawn above is one that can be taken up again, and its points set
# anew (as a stroke taken up and drawn on).
found = openclare.draw.drawn_curves(bpy.context, ball, settings.guide_curves)
assert len(found) == 1 and found[0][0] == drawn and found[0][2], found
openclare.bezier_spline(drawn.data, points[:100], False, 0.02)
assert drawn.data.splines[0].type == "BEZIER" and not drawn.data.splines[0].use_cyclic_u
# The curve moves with the ball.
ball.location.x += 1.0
bpy.context.view_layer.update()
assert abs(drawn.matrix_world.translation.x - ball.matrix_world.translation.x) < 1e-6
ball.location.x -= 1.0
bpy.context.view_layer.update()
print("DRAWN as", len(spline.bezier_points), "control points, a child of the ball, held on it")
assert drawn.show_in_front, "a drawn curve is hidden half behind the object"

# What Cut Volume makes of a cut: the plane of a line across the ball at a
# height of -0.4 cuts it in one closed loop, made a closed curve of few
# control points on it.
surface = openclare.draw.cut_surface(bpy.context, ball)
assert openclare.draw.cut_surface(bpy.context, ball) is surface, "the triangles were read again"
loops = openclare.draw.cut_loops(surface, (0.0, 0.0, -0.4), (0.0, 0.0, 1.0))
loop, gap = openclare.cut.nearest_loop(loops, (0.0, -0.9, -0.4))
assert len(loops) == 1 and loop[1] and gap < 0.05, [(len(p), c) for p, c in loops]
# The same cut made by the thread of Cut Volume, away from Blender.
cutter = openclare.draw.Cutter(surface, 0.05)
cutter.ask(1, ((0.0, 0.0, -0.4), (0.0, 0.0, 1.0)), (0.0, -0.9, -0.4))
for _ in range(500):
    done = cutter.take()
    if done is not None:
        break
    time.sleep(0.01)
cutter.close()
assert done is not None and done[0] == 1 and done[1][1] is None and len(done[1][0]) == len(loop[0]), done
step = openclare.draw.step_of(ball)
ring = openclare.trace.resampled([tuple(point) for point in loop[0]], step, closed=True)
belt = openclare.drawn_curve(bpy.context, ball, ring, True, "guide", step)
assert belt.data.splines[0].use_cyclic_u and len(belt.data.splines[0].bezier_points) <= 4
print("CUT a loop of", len(loop[0]), "points round the ball, made", len(belt.data.splines[0].bezier_points), "control points")
# A copy of the ball (Shift+D) shares its collections; a curve drawn on the
# copy is its own, and each takes only its own curves when remeshed.
assert settings.guide_curves.name.startswith(f"{ball.name} guide curves"), settings.guide_curves.name
bpy.ops.object.select_all(action="DESELECT")
ball.select_set(True)
bpy.context.view_layer.objects.active = ball
bpy.ops.object.duplicate()
copy = bpy.context.active_object
copy.location.x += 5.0
assert copy.openclare.guide_curves == settings.guide_curves
openclare.drawn_curve(bpy.context, copy, ring, True, "guide", step)
bpy.context.view_layer.update()
assert len(openclare.curve_objects(settings.guide_curves)) == 3, openclare.curve_objects(settings.guide_curves)
assert len(openclare.curve_objects(settings.guide_curves, ball)) == 2 and len(openclare.curve_objects(settings.guide_curves, copy)) == 1
print("COPY the ball and its copy share a collection of 3 curves: 2 taken for the ball, 1 for the copy")
bpy.data.objects.remove(copy)
settings.guide_curves = None
settings.quads, settings.adaptive_size = 800, 50.0

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
# Shaded By Angle by default: smooth, but sharp across the edges of the cube,
# and only there; Flat and Smooth as they say.
mesh = result.data
sharp = [edge for edge in mesh.edges if edge.use_edge_sharp]
assert all(face.use_smooth for face in mesh.polygons) and len(sharp) >= 12, len(sharp)
faces_of = {}
for face in mesh.polygons:
    for key in face.edge_keys:
        faces_of.setdefault(key, []).append(face.normal)
angle = second.openclare.hard_edge_angle
for edge in mesh.edges:
    normals = faces_of[edge.key]
    bent = len(normals) == 2 and normals[0].angle(normals[1]) > angle + 1e-3
    straight = len(normals) == 2 and normals[0].angle(normals[1]) < angle - 1e-3
    assert not (bent and not edge.use_edge_sharp) and not (straight and edge.use_edge_sharp), edge.key
openclare.shade(mesh, "FLAT", second.openclare.hard_edge_angle)
assert not any(face.use_smooth for face in mesh.polygons)
openclare.shade(mesh, "SMOOTH", second.openclare.hard_edge_angle)
assert all(face.use_smooth for face in mesh.polygons)
print("SHADING By Angle: %d sharp edges on the cube of %d quads" % (len(sharp), len(mesh.polygons)))
openclare.shade(mesh, "BY_ANGLE", second.openclare.hard_edge_angle)
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

# Symmetry: Suzanne is the same on both sides of the plane across her own X
# axis. Asked to, the quads come out the same on both sides, wherever the
# object is and however it is turned; with the marked equator of the ball
# above, and its materials, the same.
import mathutils


def mirror_image_missing(result, axis):
    """How many vertices of `result` have no vertex at their mirror image
    across the plane through its origin across `axis`, in its own axes."""
    tree = mathutils.kdtree.KDTree(len(result.data.vertices))
    for vertex in result.data.vertices:
        tree.insert(vertex.co, vertex.index)
    tree.balance()
    missing = 0
    for vertex in result.data.vertices:
        image = vertex.co.copy()
        image[axis] = -image[axis]
        if tree.find(image)[2] > 1e-5:
            missing += 1
    return missing


bpy.ops.mesh.primitive_monkey_add(location=(72.0, 3.0, -1.0), rotation=(0.3, 0.0, 1.1))
monkey = bpy.context.active_object
monkey.modifiers.new("subdivide", "SUBSURF").levels = 1
alone(monkey)
settings.quads, settings.use_marked_sharp, settings.symmetry = 1500, False, "X"
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
result = bpy.context.active_object
assert mirror_image_missing(result, 0) == 0
assert int(result[openclare.REPORT].get("off_the_plane", "0")) == 0
plain_faces = len(result.data.polygons)
# Across Y Suzanne is not the same on both sides: refused, and said why.
alone(monkey)
settings.symmetry = "Y"
try:
    bpy.ops.object.openclare_remesh()
    raise AssertionError("a model not the same on both sides was taken as symmetric")
except RuntimeError as error:
    assert "not the same on both sides" in str(error), error

alone(ball)
settings.symmetry, settings.use_marked_sharp, settings.quads = "X", True, 800
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
result = bpy.context.active_object
assert mirror_image_missing(result, 0) == 0
assert on_equator(result) >= 30, on_equator(result)
settings.symmetry = "NONE"
print("SYMMETRY:", plain_faces, "quads on Suzanne, each vertex with its image; the marked equator kept across X")

# Extensions (see extend): what an add-on that includes this one adds to a
# run. One asks the engine for the grid UV and lays it on the result; one
# fails, and the result stays; Remesh Again hands them the result in place.
class GridUV(openclare.extend.Extension):
    def asks(self, original):
        return {"uv_layers": ("grid",)}

    def made(self, context, original, result, made, report):
        layer = result.data.uv_layers.new(name="grid", do_init=False)
        layer.data.foreach_set("uv", made.uv["grid"])
        result["extended"] = result.get("extended", 0) + 1
        report["grid"] = len(made.uv["grid"]) // 2


class Failing(openclare.extend.Extension):
    def made(self, context, original, result, made, report):
        raise RuntimeError("on purpose")


grid_uv, failing = GridUV(), Failing()
openclare.extend.add(grid_uv)
openclare.extend.add(failing)
alone(ball)
settings.quads = 400
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
result = bpy.context.active_object
assert "grid" in result.data.uv_layers and result["extended"] == 1
assert int(result[openclare.REPORT]["grid"]) == len(result.data.loops)
assert bpy.ops.object.openclare_remesh_again() == {"FINISHED"}
assert bpy.context.active_object == result and result["extended"] == 2 and "grid" in result.data.uv_layers
openclare.extend.remove(failing)


class Greedy(openclare.extend.Extension):
    def asks(self, original):
        return {"curves": ()}


openclare.extend.add(Greedy())
alone(ball)
try:
    bpy.ops.object.openclare_remesh()
    raise AssertionError("an extension took an argument openclare gives")
except RuntimeError as error:
    assert "curves" in str(error), error
openclare.extend.EXTENSIONS.clear()
alone(ball)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert len(bpy.context.active_object.data.uv_layers) == 0
print("EXTENSIONS the grid UV asked and laid, kept on Remesh Again; a failing one passed by; none, no UV")

openclare.unregister()
# The drawing goes with the add-on.
assert preview.state.handle is None and preview.state.shown is None
assert not bpy.app.timers.is_registered(preview.tick)
assert not hasattr(bpy.types.WindowManager, "openclare_show_hard_edges")
print("ADD-ON OK")
