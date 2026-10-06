"""Builds the provisional corpus with Blender, so nothing has to be downloaded.

    blender -b --python bench/make_corpus.py -- <output directory>

These shapes are synthetic stand-ins. They cover the cases the bench needs
(smooth, noisy, branching, high genus, open, sharp-edged) but they are not
real sculpts; see bench/README.md.
"""

import math
import os
import sys

import bpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from blender_io import activate, evaluated_triangles, write_obj  # noqa: E402

output = sys.argv[sys.argv.index("--") + 1]
os.makedirs(output, exist_ok=True)


def reset():
    bpy.ops.wm.read_factory_settings(use_empty=True)


def save(name, obj):
    mesh = evaluated_triangles(obj)
    write_obj(os.path.join(output, name + ".obj"), mesh)
    print(f"CORPUS {name} {len(mesh.vertices)} vertices {len(mesh.polygons)} triangles", flush=True)


def voxelize(obj, size, smooth=0):
    """Rebuilds the surface as a uniform, direction-free tessellation, the way
    a sculpting session leaves it."""
    remesh = obj.modifiers.new("voxel", "REMESH")
    remesh.mode = "VOXEL"
    remesh.voxel_size = size
    remesh.adaptivity = 0.0
    if smooth:
        relax = obj.modifiers.new("smooth", "SMOOTH")
        relax.factor = 0.5
        relax.iterations = smooth


def noise_texture(name, scale, depth):
    texture = bpy.data.textures.new(name, "CLOUDS")
    texture.noise_scale = scale
    texture.noise_depth = depth
    return texture


def displace(obj, texture, strength):
    modifier = obj.modifiers.new(texture.name, "DISPLACE")
    modifier.texture = texture
    modifier.strength = strength
    modifier.mid_level = 0.5


# Open surface in several pieces, with designed edge flow.
reset()
bpy.ops.mesh.primitive_monkey_add()
suzanne = bpy.context.active_object
suzanne.modifiers.new("subdivision", "SUBSURF").levels = 2
save("suzanne", suzanne)

# Smooth genus 0 shape with bulges and necks: overlapping balls, fused.
reset()
balls = []
for (x, y, z), radius in [((0, 0, 0), 1.0), ((1.1, 0.3, 0.2), 0.75), ((-0.9, 0.5, -0.3), 0.7),
                          ((0.2, -1.0, 0.5), 0.65), ((0.3, 0.6, 1.0), 0.6), ((1.9, 0.6, 0.5), 0.45)]:
    bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=4, radius=radius, location=(x, y, z))
    balls.append(bpy.context.active_object)
for ball in balls:
    ball.select_set(True)
bpy.ops.object.join()
blob = bpy.context.active_object
voxelize(blob, 0.025, smooth=10)
save("blob", blob)

# Noisy sculpt: large features plus fine grain.
reset()
bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=7, radius=1.0)
rock = bpy.context.active_object
displace(rock, noise_texture("large", 0.9, 2), 0.55)
displace(rock, noise_texture("grain", 0.08, 1), 0.035)
save("rock", rock)

# Branching shape: torso, neck and head, four legs, tail.
reset()
joints = [(-1.2, 0, 0.9), (-0.4, 0, 1.0), (0.5, 0, 1.0), (1.1, 0, 1.3), (1.5, 0, 1.75), (1.9, 0, 1.7),
          (-1.2, 0.35, 0.0), (-1.2, -0.35, 0.0), (0.5, 0.35, 0.0), (0.5, -0.35, 0.0), (-1.9, 0, 1.3)]
bones = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 6), (0, 7), (2, 8), (2, 9), (0, 10)]
radii = [0.42, 0.48, 0.42, 0.26, 0.3, 0.2, 0.13, 0.13, 0.13, 0.13, 0.07]
skeleton = bpy.data.meshes.new("creature")
skeleton.from_pydata(joints, bones, [])
creature = bpy.data.objects.new("creature", skeleton)
bpy.context.collection.objects.link(creature)
activate(creature)
bpy.ops.object.modifier_add(type="SKIN")
for vertex, radius in zip(skeleton.skin_vertices[0].data, radii):
    vertex.radius = (radius, radius)
creature.modifiers.new("subdivision", "SUBSURF").levels = 2
voxelize(creature, 0.02, smooth=4)
save("creature", creature)


def drilled_block(size, holes):
    bpy.ops.mesh.primitive_cube_add(size=1.0)
    block = bpy.context.active_object
    block.scale = size
    for axis, radius, offset in holes:
        bpy.ops.mesh.primitive_cylinder_add(vertices=48, radius=radius, depth=6.0, location=offset)
        drill = bpy.context.active_object
        drill.rotation_euler = {"x": (0, math.pi / 2, 0), "y": (math.pi / 2, 0, 0), "z": (0, 0, 0)}[axis]
        cut = block.modifiers.new("drill_" + axis, "BOOLEAN")
        cut.operation = "DIFFERENCE"
        cut.solver = "EXACT"
        cut.object = drill
        drill.hide_render = True
    return block


# High genus, organic: a cube drilled along its three axes, then melted.
reset()
cage = drilled_block((2.0, 2.0, 2.0), [("x", 0.55, (0, 0, 0)), ("y", 0.55, (0, 0, 0)), ("z", 0.55, (0, 0, 0))])
voxelize(cage, 0.025, smooth=12)
save("cage", cage)

# Mechanical part: flat faces, hard edges, two bores, CAD-like long triangles.
reset()
part = drilled_block((2.4, 1.4, 0.7), [("z", 0.32, (-0.55, 0, 0)), ("z", 0.2, (0.6, 0.2, 0))])
save("part", part)
