"""Runs the reflection check of the add-on in a Blender without interface, on
cages whose answer is known by construction.

    blender -b --factory-startup --python blender/test_reflection.py

Exits with an error if anything is not as it should be.
"""

import math
import sys
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
    report = reflection.refresh(bpy.context)
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
assert listed == [("N-gon, 5 sides · pinch 90°", "MESH_GRID")] * 3, listed
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

# Without a Catmull-Clark modifier, nothing is checked.
plain = cage("plain cube", [corner_of(v) for v in range(8)], CUBE)
plain.modifiers.clear()
bpy.ops.object.mode_set(mode="EDIT")
assert reflection.checked_object(bpy.context) is None and reflection.refresh(bpy.context) is None
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
