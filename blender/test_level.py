"""Runs the engine's placing of a subdivision level in a Blender without
interface, the round trip the Multires of the PRO edition makes: Multires
subdivides a cage, the engine places the new level on the source, reshape
writes the positions into the level.

    blender -b --factory-startup --python-exit-code 1 --python blender/test_level.py

Exits with an error if anything is not as it should be.
"""

import sys
from pathlib import Path

import bpy
import numpy

sys.path.insert(0, str(Path(__file__).resolve().parent))
import openclare  # noqa: E402
from openclare import engine  # noqa: E402

openclare.register()
print("ENGINE", engine.version())


def check(condition, what):
    if not condition:
        raise SystemExit(f"FAILED: {what}")
    print("ok", what)


def evaluated(obj):
    return bpy.data.meshes.new_from_object(obj.evaluated_get(bpy.context.evaluated_depsgraph_get()))


def arrays(mesh):
    positions = numpy.empty(3 * len(mesh.vertices), numpy.float32)
    mesh.vertices.foreach_get("co", positions)
    corners = numpy.empty(len(mesh.loops), numpy.uint32)
    mesh.loops.foreach_get("vertex_index", corners)
    return positions.reshape(-1, 3), corners


bpy.ops.wm.read_factory_settings(use_empty=True)

# The source: a ball of radius 1, in triangles.
bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=5, radius=1.0)
ball = bpy.context.active_object
source_positions, source_triangles = arrays(ball.data)

# The cage: a cube inside it, its edges creased, under a Multires.
bpy.ops.mesh.primitive_cube_add(size=1.0)
cage = bpy.context.active_object
crease = cage.data.attributes.new("crease_edge", "FLOAT", "EDGE")
crease.data.foreach_set("value", numpy.ones(len(cage.data.edges), numpy.float32))
multires = cage.modifiers.new("Multires", "MULTIRES")
multires.use_creases = True
bpy.ops.object.multires_subdivide(modifier="Multires", mode="CATMULL_CLARK")

level = evaluated(cage)
positions, quads = arrays(level)
check(len(quads) == 4 * len(level.polygons), "the level is all quads")
weights = numpy.empty(len(level.edges), numpy.float32)
level.attributes["crease_edge"].data.foreach_get("value", weights)
ends = numpy.empty(2 * len(level.edges), numpy.uint32)
level.edges.foreach_get("vertices", ends)
creased = weights > 0
check(int(creased.sum()) == 24, "Multires carries the creases onto the level")

# Placed without creases (a ball has no hard edge to go to): every vertex
# goes out onto the ball.
placed, status, report = engine.project_level(positions, quads, source_positions, source_triangles,
                                              settings="reach = 4\nsamples = 0")
check(int(report["moved"]) == len(positions) and (status == 0).all(), "every vertex is moved")
radius = numpy.linalg.norm(placed, axis=1)
check(numpy.abs(radius - 1.0).max() < 0.01, "every vertex is on the ball")

# With the creases, and a ball has no hard edge: the vertices inside the
# creased lines find none and go on as the others; the cube's corners hold.
_, status, report = engine.project_level(positions, quads, source_positions, source_triangles,
                                         ends.reshape(-1, 2)[creased], weights[creased], "reach = 4\nsamples = 0")
check(int(report["held"]) == 8, "the corners of the creased lines hold")

# Written into the level by reshape, and read back.
target = level.copy()
target.vertices.foreach_set("co", placed.reshape(-1))
target.update()
temporary = bpy.data.objects.new("placed", target)
bpy.context.collection.objects.link(temporary)
bpy.ops.object.select_all(action="DESELECT")
temporary.select_set(True)
cage.select_set(True)
bpy.context.view_layer.objects.active = cage
bpy.ops.object.multires_reshape(modifier="Multires")
back, _ = arrays(evaluated(cage))
check(numpy.abs(back - placed).max() < 1e-5, "reshape writes the placed positions into the level")

try:
    engine.project_level(positions, quads, source_positions, source_triangles, settings="speed = 3")
    check(False, "a wrong setting is refused")
except engine.EngineError as error:
    check("speed" in str(error), "a wrong setting is refused")
print("ALL GOOD")
