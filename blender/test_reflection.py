"""Runs the reflection check of the add-on in a Blender without interface, on
cages whose answer is known by construction.

    blender -b --factory-startup --python blender/test_reflection.py

Exits with an error if anything is not as it should be.
"""

import math
import sys
import time
from pathlib import Path

import bmesh
import bpy

sys.path.insert(0, str(Path(__file__).resolve().parent))
import openclare  # noqa: E402
from openclare import engine, reflection  # noqa: E402

openclare.register()
print("ENGINE", engine.version())

CORNER = math.degrees(math.acos(1.0 / math.sqrt(3.0)))
# A unit cube, vertex v at (v & 1, v >> 1 & 1, v >> 2), wound outwards.
CUBE = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]]


def corner_of(v):
    return (float(v & 1), float((v >> 1) & 1), float(v >> 2))


def close(a, b, within=1e-4):
    return abs(a - b) < within


# ---------------------------------------------------------------- the engine

positions = [corner_of(v) for v in range(8)]
sizes = [4] * 6
corners = sum(CUBE, [])
found = engine.reflection_check(positions, sizes, corners)
assert found.poles == 8 and len(found.kinds) == 8, (found.poles, len(found.kinds))
assert all(engine.FAULT_KINDS[k] == "curved_pole" for k in found.kinds)
assert found.on_vertex.all() and sorted(found.indices.tolist()) == list(range(8))
assert all(close(value, CORNER) for value in found.values), found.values
edges = [0, 1, 2, 3, 4, 5, 6, 7, 0, 2, 1, 3, 4, 6, 5, 7, 0, 4, 1, 5, 2, 6, 3, 7]
creased = engine.reflection_check(positions, sizes, corners, edges, [1.0] * 12)
assert creased.poles == 0 and len(creased.kinds) == 0, (creased.poles, len(creased.kinds))
# A crease that only softens is not sharp.
soft = engine.reflection_check(positions, sizes, corners, edges, [0.5] * 12)
assert len(soft.kinds) == 8
try:
    engine.reflection_check(positions, sizes, corners[:-1])
    raise AssertionError("a face short of a corner was taken")
except engine.EngineError as error:
    assert "corners" in str(error), error
print("ENGINE ROUND TRIP ok")


# ---------------------------------------------------------------- in Blender

settings = bpy.context.scene.openclare_reflection
settings.show = True


def cage(name, points, faces, crease=None):
    """A mesh object under a Catmull-Clark Subdivision Surface, active and
    alone selected; `crease` gives every edge that weight."""
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(points, [], faces)
    mesh.update()
    if crease is not None:
        attribute = mesh.attributes.new("crease_edge", "FLOAT", "EDGE")
        attribute.data.foreach_set("value", [crease] * len(mesh.edges))
    owner = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(owner)
    owner.modifiers.new("subdivide", "SUBSURF")
    if bpy.context.object is not None and bpy.context.object.mode != "OBJECT":
        bpy.ops.object.mode_set(mode="OBJECT")
    for other in bpy.context.view_layer.objects:
        other.select_set(False)
    owner.select_set(True)
    bpy.context.view_layer.objects.active = owner
    return owner


def edited(owner):
    bpy.ops.object.mode_set(mode="EDIT")
    report = reflection.look(bpy.context)
    assert report is not None, "no check of the cage in Edit Mode"
    return report


def kinds(report):
    return [engine.FAULT_KINDS[k] for k in report.faults.kinds]


# The cube, creased all round: no pole looked at, nothing wrong.
box = cage("creased cube", [corner_of(v) for v in range(8)], CUBE, crease=1.0)
report = edited(box)
assert len(report) == 0 and report.faults.poles == 0, kinds(report)
assert reflection.report_of(box, settings) == (report, True)
# With Use Creases off, the modifier subdivides as if there were none: the
# crease list is empty, and every corner is a pole on a curved zone.
bpy.ops.object.mode_set(mode="OBJECT")
box.modifiers["subdivide"].use_creases = False
report = edited(box)
assert kinds(report) == ["curved_pole"] * 8 and all(close(v, CORNER) for v in report.faults.values)
assert reflection.fault_name(report, 0) == "Pole, 3 edges · curved 55°", reflection.fault_name(report, 0)
bpy.ops.object.mode_set(mode="OBJECT")
box.modifiers["subdivide"].use_creases = False
# The poles leave the list once the settings say a fold of 60 degrees is
# fine: the tolerance of the panel is the engine's.
settings.angle_tolerance = math.radians(60.0)
report = edited(box)
assert len(report) == 0 and report.faults.poles == 8, kinds(report)
settings.angle_tolerance = math.radians(1.0)
bpy.ops.object.mode_set(mode="OBJECT")
box.modifiers["subdivide"].use_creases = True

# The cube with its corner (1, 1, 1) cut off by a triangle through the
# middles of its edges: the triangle faces (1, 1, 1) / √3 and the pentagons
# round it an axis each, acos(1 / √3) from it; the pentagons meet the quads
# and each other at right angles.
points = [corner_of(v) for v in range(7)] + [(0.5, 1.0, 1.0), (1.0, 0.5, 1.0), (1.0, 1.0, 0.5)]
faces = [[0, 2, 3, 1], [0, 1, 5, 4], [0, 4, 6, 2], [4, 5, 8, 7, 6], [2, 6, 7, 9, 3], [1, 3, 9, 8, 5], [7, 8, 9]]
cut = cage("cut cube", points, faces)
report = edited(cut)
pinches = [i for i, kind in enumerate(kinds(report)) if kind == "pinch"]
assert len(pinches) == 4, kinds(report)
triangle = next(i for i in pinches if report.faults.indices[i] == 6)
assert close(report.faults.values[triangle], CORNER), report.faults.values[triangle]
assert all(close(report.faults.values[i], 90.0) for i in pinches if i != triangle)
assert reflection.fault_name(report, triangle) == "Triangle · pinch 55°", reflection.fault_name(report, triangle)
assert reflection.fault_name(report, 0) == "N-gon, 5 sides · pinch 90°", reflection.fault_name(report, 0)
assert report.faults.parts(triangle) == ([0, 1, 2], [0, 1, 2])
# Drawn: the three edges of the triangle dashed in the colour of a pinch,
# and its three corners.
lines, vertices, centres = reflection.overlay(cut, report, len(report))
violet = reflection.COLOURS["pinch"]
assert any(colour == violet for _, _, colour in lines) and len(vertices) >= 3
# Each of the four faces that pinch has every edge and every corner at
# fault: its edges dashed (ten dashes where there is no view to size them)
# and its corners dotted. Below them, the poles: a flat ring each.
pinched_sides = sum(len(report.face(int(report.faults.indices[i]))) for i in pinches)
dashed = [segment for segment in lines if segment[2] == violet]
assert len(dashed) == 10 * pinched_sides, (len(dashed), pinched_sides)
rings = [segment for segment in lines if segment[2] == reflection.COLOURS["curved_pole"]]
poles = kinds(report).count("curved_pole")
assert poles == 10 and len(rings) == reflection.CIRCLE_SEGMENTS * poles, (poles, len(rings))
assert len(vertices) == pinched_sides and not centres

# The panel: the counter, one row per fault, worst first, each with the
# icon of its repair.
icons = bpy.types.UILayout.bl_rna.functions["label"].parameters["icon"].enum_items.keys()
assert all(icon in icons for icon in reflection.REPAIR_ICONS.values())


class Drawn:
    """A layout that keeps what is drawn on it."""

    def __init__(self, log):
        self.log = log

    def row(self, **_):
        return Drawn(self.log)

    column = box = row

    def prop(self, owner, name, **_):
        self.log.append(("prop", name, None))

    def label(self, text="", icon="NONE"):
        self.log.append(("label", text, icon))

    def operator(self, name, text="", icon="NONE", **_):
        self.log.append(("operator", text, icon))

        class Properties:
            pass

        return Properties()


settings.worst = 3
log = []
reflection.draw_panel(Drawn(log), bpy.context)
said = [text for kind, text, _ in log if kind == "label"]
assert f"3 shown out of {len(report)}; 10 poles looked at" in said, said
listed = [(text, icon) for kind, text, icon in log if kind == "operator"]
# The rows, then the button of Repair in Place.
assert listed == [("N-gon, 5 sides · pinch 90°", "MESH_GRID")] * 3 + [("", "MODIFIER")], listed
settings.worst = 20

# A click on a row selects the face, or the vertex, and makes it active.
assert bpy.ops.mesh.openclare_reflection_select(index=triangle) == {"FINISHED"}
bm = bmesh.from_edit_mesh(cut.data)
selected = [f.index for f in bm.faces if f.select]
assert selected == [6] and bm.faces.active.index == 6, (selected, bm.faces.active)
assert tuple(bpy.context.tool_settings.mesh_select_mode) == (False, False, True)
pole = kinds(report).index("curved_pole")
assert bpy.ops.mesh.openclare_reflection_select(index=pole) == {"FINISHED"}
bm = bmesh.from_edit_mesh(cut.data)
chosen = [v.index for v in bm.verts if v.select]
assert chosen == [int(report.faults.indices[pole])], (chosen, report.faults.indices[pole])
assert tuple(bpy.context.tool_settings.mesh_select_mode) == (True, False, False)
# Selecting changes nothing of the cage: what was found is still the same.
again = reflection.check(cut, settings)
assert again.faults.indices.tolist() == report.faults.indices.tolist()
bpy.ops.object.mode_set(mode="OBJECT")

# A dart, folded: the centre of its vertices is past its notch, and the
# corner quads of the notch and of both lower tips fold. Drawn: a red star
# from the centre to those three corners.
dart = cage("dart", [(0.0, 3.0, 0.0), (-2.0, -1.0, 0.0), (0.0, 1.0, 0.0), (2.0, -1.0, 0.0)], [[0, 1, 2, 3]])
report = edited(dart)
assert kinds(report) == ["folded"] and close(report.faults.values[0], -0.5), (kinds(report), report.faults.values)
assert reflection.fault_name(report, 0) == "Quad · 3 corner quads fold"
lines, vertices, centres = reflection.overlay(dart, report, 1)
red = reflection.COLOURS["folded"]
assert len(lines) == 3 * 10 and all(colour == red for _, _, colour in lines)
assert len(centres) == 1 and len(vertices) == 3
bpy.ops.object.mode_set(mode="OBJECT")

# A regular pentagon with one vertex raised: off its plane. Drawn: a circle
# flat round its centre, and each vertex off the plane joined to the plane.
ring = [(math.cos(math.radians(90 + 72 * i)), math.sin(math.radians(90 + 72 * i)), 0.1 if i == 0 else 0.0)
        for i in range(5)]
pentagon = cage("pentagon", ring, [[0, 1, 2, 3, 4]])
report = edited(pentagon)
assert kinds(report) == ["not_flat"], kinds(report)
deviation = 0.2 / math.sqrt(0.04 + 25.0)
radius = math.sqrt(1.0 + 16.0 * 0.01 / 25.0)
assert close(report.faults.values[0], math.degrees(math.atan(deviation / radius)), 1e-3), report.faults.values[0]
lines, vertices, centres = reflection.overlay(pentagon, report, 1)
yellow = reflection.COLOURS["not_flat"]
assert len(lines) == reflection.CIRCLE_SEGMENTS + 5 and all(colour == yellow for _, _, colour in lines)
assert len(vertices) == 5
bpy.ops.object.mode_set(mode="OBJECT")

# ---------------------------------------------------------------- what starts a check
#
# Timers do not run here: what the timer would do once the cage is still,
# `reflection.look()`, is run by hand where it would be due.


class Update:
    def __init__(self, owner, geometry):
        self.id, self.is_updated_geometry = owner, geometry


class Signalled:
    """A depsgraph as an update handler sees it: its list of updates."""

    def __init__(self, *updates):
        self.updates = list(updates)


def due():
    """Runs the look the timer would run, if one is due; True if it was."""
    if not reflection._pending:
        return False
    reflection.look(bpy.context)
    return True


empty = bpy.data.objects.new("empty", None)
ball = cage("ball", [corner_of(v) for v in range(8)], CUBE)
bpy.ops.object.mode_set(mode="EDIT")
reflection.look(bpy.context)
bpy.context.view_layer.update()
reflection._pending = False
stats = reflection.STATS
generation, checks, looks = reflection._generation, stats.checks, stats.looks


def changes(action):
    """What `action` makes of the check: how many times the generation was
    raised, looks and checks run, once Blender has signalled it."""
    global generation, checks, looks
    action()
    bpy.context.view_layer.update()
    due()
    out = (reflection._generation - generation, stats.looks - looks, stats.checks - checks)
    generation, checks, looks = reflection._generation, stats.checks, stats.looks
    return out


# A selection, another object, a modifier setting: nothing.
assert changes(lambda: bpy.ops.mesh.select_all(action="SELECT")) == (0, 0, 0)
assert changes(lambda: setattr(box, "location", (4.0, 0.0, 0.0))) == (0, 0, 0)
assert changes(lambda: setattr(ball.modifiers["subdivide"], "levels", 2)) in ((0, 0, 0), (1, 1, 0))


def move():
    bm = bmesh.from_edit_mesh(ball.data)
    bm.verts.ensure_lookup_table()
    bm.verts[0].co.x += 0.25
    bmesh.update_edit_mesh(ball.data)


# A vertex moved: one check.
assert changes(move) == (1, 1, 1), changes(lambda: None)
# Our own look signals nothing back: run again and again at rest, nothing.
for _ in range(10):
    reflection.look(bpy.context)
    assert changes(lambda: None) == (0, 0, 0)


def select_through_the_edit_mesh():
    bm = bmesh.from_edit_mesh(ball.data)
    bm.verts.ensure_lookup_table()
    bm.verts[3].select_set(True)
    bmesh.update_edit_mesh(ball.data, loop_triangles=False, destructive=False)


# A selection made through the edit mesh is signalled as an edit of its
# geometry: the cage is read, found the same, and not checked again.
before = stats.unchanged
assert changes(select_through_the_edit_mesh) == (1, 1, 0)
assert stats.unchanged == before + 1

# Many signals at rest, edits of the cage or not, while still: one look at
# most, never a check, and none at all for those that are not edits.
for _ in range(20):
    reflection._changed(None, Signalled(Update(ball, False), Update(empty, True), Update(ball.data, False)))
assert not reflection._pending and changes(lambda: None) == (0, 0, 0)
for _ in range(20):
    reflection._changed(None, Signalled(Update(ball.data, True)))
assert reflection._pending and changes(lambda: None) == (20, 1, 0)
# A change signalled while this module looks is not one.
reflection._looking = True
reflection._changed(None, Signalled(Update(ball.data, True)))
reflection._looking = False
assert not reflection._pending and changes(lambda: None) == (0, 0, 0)

# A drag: many moves, the cage looked at once it is still, checked once.
for _ in range(5):
    move()
    bpy.context.view_layer.update()
assert reflection._pending and stats.checks == checks
assert changes(lambda: None)[1:] == (1, 1)

# The overlay: built once for a report and a view, the same thing given back
# after; while the view moves, what was built is drawn as it is; once the
# view has been still, built again for it.
report = reflection.look(bpy.context)
cut_report = reflection.report_of(cut, settings)[0]
builds = stats.builds
first, built = reflection.overlay_for("region", cut, cut_report, 20, "view A", reflection.SeeAll, now=100.0)
assert built and stats.builds == builds + 1
again, built = reflection.overlay_for("region", cut, cut_report, 20, "view A", reflection.SeeAll, now=100.1)
assert not built and again is first and again.segments is first.segments
moving, built = reflection.overlay_for("region", cut, cut_report, 20, "view B", reflection.SeeAll, now=101.0)
assert not built and moving is first
still, built = reflection.overlay_for("region", cut, cut_report, 20, "view B", reflection.SeeAll,
                                      now=101.0 + reflection.SETTLE_SECONDS)
assert built and still is not first and still.built_for == "view B"
fewer, built = reflection.overlay_for("region", cut, cut_report, 3, "view B", reflection.SeeAll,
                                      now=101.0 + 2 * reflection.SETTLE_SECONDS)
assert built and len(fewer.segments[0]) < len(still.segments[0])
# The rows of the panel are made once per report and number shown.
assert cut_report.rows(3) is cut_report.rows(3)

# With Developer Extras, the panel says what the check has cost.
bpy.context.preferences.view.show_developer_ui = True
log = []
reflection.draw_panel(Drawn(log), bpy.context)
assert any("checks of" in text and "rays" in text for kind, text, _ in log if kind == "label"), log
bpy.context.preferences.view.show_developer_ui = False
log = []
reflection.draw_panel(Drawn(log), bpy.context)
assert not any("checks of" in text for kind, text, _ in log if kind == "label")
print("STATS", stats.line())
# ---------------------------------------------------------------- off
#
# With the toggle off, or the experimental features hidden, the handlers and
# the draw read the toggle and return: nothing looked at, nothing scheduled.
from openclare import experimental  # noqa: E402

calls = []
real_checked, real_edits, real_look = reflection.checked_object, reflection.edits_a_cage, reflection.look
reflection.checked_object = lambda context: calls.append("checked_object") or real_checked(context)
reflection.edits_a_cage = lambda *args: calls.append("edits_a_cage") or real_edits(*args)
reflection.look = lambda *args, **kwargs: calls.append("look") or real_look(*args, **kwargs)
signals = Signalled(Update(ball, True), Update(ball.data, True), Update(empty, True))
for off in ("toggle", "experimental"):
    if off == "toggle":
        settings.show = False
    else:
        settings.show = True
        experimental.assumed = False
    reflection._pending = False
    generation, checks = reflection._generation, stats.checks
    start = time.perf_counter()
    for _ in range(10000):
        reflection._changed(None, signals)
    handler = (time.perf_counter() - start) / 10000
    start = time.perf_counter()
    for _ in range(10000):
        reflection.draw_view()
    drawing = (time.perf_counter() - start) / 10000
    reflection._undone()
    assert calls == [] and not reflection._pending and reflection._generation == generation, calls
    assert stats.checks == checks and reflection._missed
    print(f"OFF ({off}): a signal handled in {1e6 * handler:.2f} µs, a draw in {1e6 * drawing:.2f} µs, "
          "nothing scheduled")
experimental.assumed = True
reflection.checked_object, reflection.edits_a_cage, reflection.look = real_checked, real_edits, real_look
# On again: everything found is out of date and looked at; the cage is the
# same, so the engine is not called.
settings.show = False
settings.show = True
assert reflection._pending and not reflection._missed
looks, checks = stats.looks, stats.checks
due()
assert (stats.looks - looks, stats.checks - checks) == (1, 0)

# ---------------------------------------------------------------- hardening
#
# What keeps Blender safe from the check: no look under a modal operator,
# no cage given to the engine that is not sound, no error raised into
# Blender, nothing asked of Blender to tell what is seen.
from mathutils import Vector  # noqa: E402

bpy.ops.mesh.reveal()
report = reflection.check(ball, settings, None)

# What hides a point is the cage as it was read: from outside, along x, the
# near face of the cube is hit before the face x = 1 behind it.
tree = report.tree()
assert tree is report.tree()
hit = tree.ray_cast(Vector((-5.0, 0.5, 0.5)), Vector((1.0, 0.0, 0.0)), 10.0)
assert hit[0] is not None and hit[0].x < 0.9, hit   # the near face, which the moves above bent
# A face hidden in Edit Mode hides nothing: the ray goes on to the far one.
near = int(hit[2])
bm = bmesh.from_edit_mesh(ball.data)
bm.faces.ensure_lookup_table()
bm.faces[near].hide_set(True)
bmesh.update_edit_mesh(ball.data, loop_triangles=False, destructive=False)
checks = stats.checks
same = reflection.check(ball, settings, report)
assert same is report and stats.checks == checks and report.hidden is not None and report.hidden[near]
hit = report.tree().ray_cast(Vector((-5.0, 0.5, 0.5)), Vector((1.0, 0.0, 0.0)), 10.0)
assert hit[0] is not None and close(hit[0].x, 1.0, 1e-5), hit
bpy.ops.mesh.reveal()
bpy.ops.mesh.select_all(action="DESELECT")
assert not reflection.check(ball, settings, report).hide(None)

# Seen from above, with and without an eye: the middle of the top face of
# the cube is seen, the middle of its bottom face is hidden by the top, and
# is seen once the top is hidden in Edit Mode. Blender is asked for nothing:
# the view is this stand-in, and there is no window here to ask.
from mathutils import Matrix  # noqa: E402


class View:
    view_matrix = Matrix.Translation((-0.5, -0.5, -10.0))   # the eye at (0.5, 0.5, 10), looking down
    perspective_matrix = Matrix.Diagonal((0.2, 0.2, -0.01, 1.0)) @ view_matrix

    def __init__(self, is_perspective):
        self.is_perspective = is_perspective


class Region:
    width, height = 800, 600


def looker(report, is_perspective):
    stand_in = type("Context", (), {"region": Region(), "region_data": View(is_perspective)})()
    return reflection.Visibility(stand_in, report)


# The cube as it was made: the moves above had pushed its first vertex along x.
bm = bmesh.from_edit_mesh(ball.data)
bm.verts.ensure_lookup_table()
bm.verts[0].co = corner_of(0)
bmesh.update_edit_mesh(ball.data)
report = reflection.check(ball, settings, report)
ball.matrix_world = Matrix.Diagonal((2.0, 2.0, 3.0, 1.0))   # a scale that is not the same along each axis
top, bottom = Vector((0.5, 0.5, 1.0)), Vector((0.5, 0.5, 0.0))
for is_perspective in (False, True):
    seen = looker(report, is_perspective)
    assert seen.sees(ball, top) and not seen.sees(ball, bottom), is_perspective
    assert seen.rays == 2 and seen.sees(ball, top) and seen.rays == 2    # asked once for a point
bm = bmesh.from_edit_mesh(ball.data)
bm.faces.ensure_lookup_table()
upper = next(f for f in bm.faces if all(v.co.z > 0.5 for v in f.verts))
upper.hide_set(True)
bmesh.update_edit_mesh(ball.data, loop_triangles=False, destructive=False)
report = reflection.check(ball, settings, report)
for is_perspective in (False, True):
    assert looker(report, is_perspective).sees(ball, bottom), is_perspective
bpy.ops.mesh.reveal()
bpy.ops.mesh.select_all(action="DESELECT")
report = reflection.check(ball, settings, report)
ball.matrix_world = Matrix.Identity(4)


class Window:
    def __init__(self, *names):
        self.modal_operators = [type("Operator", (), {"bl_idname": name})() for name in names]


class Windows:
    def __init__(self, *windows):
        self.window_manager = type("Manager", (), {"windows": list(windows)})()


# A modal operator in any window: busy. None: not.
assert reflection.busy(Windows(Window(), Window("TRANSFORM_OT_vert_slide"))) == "TRANSFORM_OT_vert_slide"
assert reflection.busy(Windows(Window(), Window())) is None and reflection.busy() is None

# While one runs, the timer looks at nothing and comes back later; once it
# has ended, it looks.
real_busy, real_look = reflection.busy, reflection.look
looked = []
reflection.look = lambda *args, **kwargs: looked.append("look")
reflection._last_change = time.monotonic() - 10.0
reflection.busy = lambda context=None: "TRANSFORM_OT_vert_slide"
assert reflection._wait() == reflection.SETTLE_SECONDS and looked == []
reflection.busy = lambda context=None: None
assert reflection._wait() is None and looked == ["look"]


# An error in the timer or in the draw goes no further, and is said once.
def fails(*_args, **_kwargs):
    looked.append("failed")
    raise RuntimeError("on purpose")


reflection.look = fails
said = len(reflection._said)
assert reflection._wait() is None and reflection._wait() is None
assert len(reflection._said) == said + 1 and not reflection._timer_set
reflection.look, reflection.busy = real_look, real_busy
real_draw, reflection._draw = reflection._draw, fails
looked.clear()
reflection.draw_view()
reflection.draw_view()
assert looked == ["failed"], looked            # not tried again for this cage
reflection._cage_changed()
reflection.draw_view()
assert looked == ["failed", "failed"]          # tried again once it has changed
reflection._draw = real_draw
reflection.look(bpy.context)
reflection._pending = False

# A cage that is not sound is never given to the engine: a vertex at no
# finite place stops the look, and the operator reports it and is cancelled.
bm = bmesh.from_edit_mesh(ball.data)
bm.verts.ensure_lookup_table()
kept = bm.verts[0].co.x
bm.verts[0].co.x = float("nan")
checks = stats.checks
try:
    reflection.cage_arrays(ball)
    raise AssertionError("a cage with a vertex at no finite place was read")
except ValueError:
    pass
try:
    bpy.ops.mesh.openclare_reflection_check()
    raise AssertionError("the check of such a cage was not cancelled")
except RuntimeError as error:
    assert "finite" in str(error), error
try:
    bpy.ops.mesh.openclare_reflection_repair()
    raise AssertionError("the repair of such a cage was not cancelled")
except RuntimeError as error:
    assert "finite" in str(error), error
assert stats.checks == checks
bm = bmesh.from_edit_mesh(ball.data)
bm.verts.ensure_lookup_table()
bm.verts[0].co.x = kept
bmesh.update_edit_mesh(ball.data)
assert bpy.ops.mesh.openclare_reflection_check() == {"FINISHED"}

bpy.ops.object.mode_set(mode="OBJECT")
# Out of Edit Mode there is no edit mesh to read.
try:
    reflection.edit_mesh(ball)
    raise AssertionError("an edit mesh was had out of Edit Mode")
except ValueError:
    pass
print("HARDENING ok")

# Without a Catmull-Clark modifier, nothing is checked.
plain = cage("plain cube", [corner_of(v) for v in range(8)], CUBE)
plain.modifiers.clear()
bpy.ops.object.mode_set(mode="EDIT")
assert reflection.checked_object(bpy.context) is None and reflection.look(bpy.context) is None
bpy.ops.object.mode_set(mode="OBJECT")

# The overlay's shaders exist where a GPU does.
try:
    import gpu

    gpu.init()
    for name in ("POLYLINE_SMOOTH_COLOR", "POINT_FLAT_COLOR"):
        gpu.shader.from_builtin(name)
    print("SHADERS found")
except Exception as error:  # noqa: BLE001
    print("SHADERS not checked:", error)

openclare.unregister()
print("REFLECTION all good")
