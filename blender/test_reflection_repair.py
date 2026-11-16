"""Runs Repair in Place, the reflection repair of the add-on, in a Blender
without interface, on cages whose answer is known by construction.

    blender -b --factory-startup --python blender/test_reflection_repair.py

Exits with an error if anything is not as it should be. A Blender without
interface has no window: the panel is drawn on a stand-in layout, and the
undo step is looked at only as far as Blender runs it there.
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

H = 0.04
# A unit cube, vertex v at (v & 1, v >> 1 & 1, v >> 2), wound outwards.
CUBE = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]]


def corner_of(v):
    return (float(v & 1), float((v >> 1) & 1), float(v >> 2))


def raised_grid(h):
    """A flat grid of 4 by 4 quads, vertex (i, j) at 5 j + i, with one more
    vertex, 25, on the edge from (1, 0) to (1, 1), raised by h: the first two
    quads are pentagons, flat but for it."""
    points = [(float(v % 5), float(v // 5), 0.0) for v in range(25)] + [(1.0, 0.5, h)]
    faces = []
    for j in range(4):
        for i in range(4):
            v = 5 * j + i
            face = [v, v + 1, v + 6, v + 5]
            if (i, j) == (0, 0):
                face.insert(2, 25)
            if (i, j) == (1, 0):
                face.insert(4, 25)
            faces.append(face)
    return points, faces


# ---------------------------------------------------------------- the engine

points, faces = raised_grid(H)
sizes = [len(f) for f in faces]
corners = sum(faces, [])
done = engine.reflection_repair(points, sizes, corners)
# The raised vertex alone comes back into the plane of the others, straight
# down by h; h is h / √(1/4 + h²) of its shortest edge.
assert done.moved == 1 and done.repairs == 1, (done.moved, done.repairs)
assert tuple(done.positions[25]) == (1.0, 0.5, 0.0), done.positions[25]
assert abs(done.largest - H) < 1e-7 and abs(done.share - H / math.sqrt(0.25 + H * H)) < 1e-6, (done.largest, done.share)
not_flat = engine.FAULT_KINDS.index("not_flat")
assert done.scores["before"][0][not_flat] == 2 and not done.scores["after"][0].any()
assert len(done.left_kinds) == 0 and len(done.crease_edges) == 0
# A cube: each of its twelve edges folds by 90 degrees, each proposed.
cube = engine.reflection_repair([corner_of(v) for v in range(8)], [4] * 6, sum(CUBE, []))
assert cube.moved == 0 and len(cube.crease_edges) == 12 and all(abs(f - 90.0) < 1e-9 for f in cube.crease_folds)
assert cube.crease_cured.all() and not cube.scores["with_creases"][0].any()
assert [engine.REPAIR_REASONS[r] for r in cube.left_reasons] == ["proposed_crease"] * 8
try:
    engine.reflection_repair(points, sizes, corners, within=[99])
    raise AssertionError("a face out of the cage was taken")
except engine.EngineError as error:
    assert "face 99" in str(error), error
print("ENGINE ROUND TRIP ok")


# ---------------------------------------------------------------- in Blender

settings = bpy.context.scene.openclare_reflection
settings.show = True


def cage(name, points, faces):
    """A mesh object under a Catmull-Clark Subdivision Surface, active and
    alone selected, in Edit Mode, every element deselected."""
    if bpy.context.object is not None and bpy.context.object.mode != "OBJECT":
        bpy.ops.object.mode_set(mode="OBJECT")
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(points, [], faces)
    mesh.update()
    owner = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(owner)
    owner.modifiers.new("subdivide", "SUBSURF")
    for other in bpy.context.view_layer.objects:
        other.select_set(False)
    owner.select_set(True)
    bpy.context.view_layer.objects.active = owner
    bpy.ops.object.mode_set(mode="EDIT")
    bpy.ops.mesh.select_all(action="DESELECT")
    return owner


def heights(owner):
    bm = bmesh.from_edit_mesh(owner.data)
    return [v.co.z for v in bm.verts]


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
        self.log.append(("operator", name, icon))

        class Properties:
            pass

        return Properties()


def panel():
    log = []
    reflection.draw_panel(Drawn(log), bpy.context)
    return [text for kind, text, _ in log if kind == "label"], [name for kind, name, _ in log if kind == "operator"]


grid = cage("raised grid", *raised_grid(H))
assert reflection.look(bpy.context) is not None
# Before any repair: the block and its button, no summary.
said, buttons = panel()
assert reflection.OPENCLARE_OT_reflection_repair.bl_idname in buttons, buttons
assert not any(text.startswith("On ") for text in said), said

# Nothing selected: the whole cage. One vertex back on z = 0, the rest as
# they were. Without interface, undo is off until a step is pushed by
# hand, and that first push only starts it: a second one is the step to go
# back to.
bpy.ops.ed.undo_push(message="undo started")
bpy.ops.ed.undo_push(message="before the repair")
assert bpy.ops.mesh.openclare_reflection_repair() == {"FINISHED"}
z = heights(grid)
assert z[25] == 0.0 and all(height == 0.0 for height in z), z
summary = reflection._repairs[grid.data.as_pointer()]
assert summary.faces is None and summary.moved == 1 and not summary.left
assert dict((kind, (before, after)) for kind, before, after in summary.table)["not_flat"] == (2, 0)
said, _ = panel()
assert any(text.startswith("On the whole cage: 1 vertex moved") for text in said), said
assert "0 left" in said, said
# The check of the cage, run again, finds nothing.
report = reflection.check(grid, settings)
assert len(report) == 0, len(report)

# Undo: one step brings the vertex back, and the summary goes, which no
# longer says what the cage is.
bpy.ops.ed.undo()
z = heights(bpy.context.edit_object)
assert abs(z[25] - H) < 1e-7 and all(height == 0.0 for height in z[:25]), z
assert not reflection._repairs
print("UNDO one step: the raised vertex back at", z[25])
bpy.ops.object.mode_set(mode="OBJECT")

# A selection: only the quads round the pentagons. Nothing moves, and the
# pentagons are left, outside the selection.
grid = cage("selected grid", *raised_grid(H))
bm = bmesh.from_edit_mesh(grid.data)
for face in bm.faces:
    face.select_set(len(face.verts) == 4)
bmesh.update_edit_mesh(grid.data)
assert bpy.ops.mesh.openclare_reflection_repair() == {"FINISHED"}
assert abs(heights(grid)[25] - H) < 1e-7
summary = reflection._repairs[grid.data.as_pointer()]
assert summary.faces == 14 and summary.moved == 0
assert summary.left and all(reason == "outside the selection" for _, reason in summary.left), summary.left
said, _ = panel()
assert any(text.startswith("On 14 selected faces: no vertex moved") for text in said), said

# A bound of 4 %: the vertex would need 8 % of its shortest edge alone, the
# plane of each pentagon 4.8 % (see the engine's tests). Refused, listed.
bpy.ops.mesh.select_all(action="DESELECT")
assert bpy.ops.mesh.openclare_reflection_repair(bound=4.0) == {"FINISHED"}
summary = reflection._repairs[grid.data.as_pointer()]
assert summary.moved == 0
reasons = [reason for words, reason in summary.left if "not flat" in words]
assert reasons == ["would move a vertex beyond the bound"] * 2, summary.left
bpy.ops.object.mode_set(mode="OBJECT")

# A cube: twelve creases proposed. Off, none is set; on, each edge gets a
# full crease, and the check finds nothing.
box = cage("cube", [corner_of(v) for v in range(8)], CUBE)
assert bpy.ops.mesh.openclare_reflection_repair() == {"FINISHED"}
summary = reflection._repairs[box.data.as_pointer()]
assert summary.proposed == 12 and not summary.set_creases
assert [reason for _, reason in summary.left] == ["a proposed crease cures it"] * 8
bm = bmesh.from_edit_mesh(box.data)
layer = bm.edges.layers.float.get("crease_edge")
assert layer is None or all(e[layer] == 0.0 for e in bm.edges)
assert bpy.ops.mesh.openclare_reflection_repair(apply_creases=True) == {"FINISHED"}
summary = reflection._repairs[box.data.as_pointer()]
assert summary.set_creases and not summary.left
assert dict((kind, after) for kind, _, after in summary.table)["curved_pole"] == 0
bm = bmesh.from_edit_mesh(box.data)
layer = bm.edges.layers.float.get("crease_edge")
assert layer is not None and all(e[layer] == 1.0 for e in bm.edges)
report = reflection.check(box, settings)
assert len(report) == 0 and report.faults.poles == 0
said, _ = panel()
assert any("12 creases proposed, set" in text for text in said), said
bpy.ops.object.mode_set(mode="OBJECT")

# Nothing of the repair runs when it is not used: the handlers only clear
# what it kept.
reflection._undone()
assert not reflection._repairs

openclare.unregister()
print("REFLECTION REPAIR all good")
