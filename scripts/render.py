"""Blender script: renders a mesh with its edges drawn, for the documentation.

    blender -b --python scripts/render.py -- <mesh.obj> <image.png> [options]

    --up y|z          which axis points up in the file (default z)
    --azimuth DEG     where the camera stands round the up axis (default -60)
    --elevation DEG   how far above the horizon (default 20)
    --size N          width and height of the image in pixels (default 900)
    --samples N       rendering samples (default 24)
    --no-edges        the surface alone, for a dense source

The mesh is read as it is, faces of any size, framed to fill the image, on a
transparent background. Edges are real geometry (thin dark tubes), so a quad
shows as a quad, without the diagonal a triangulating renderer would add.
"""

import math
import os
import sys

import bpy
from mathutils import Matrix, Vector

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "bench"))
from blender_io import read_obj  # noqa: E402

arguments = sys.argv[sys.argv.index("--") + 1 :]
source, target = arguments[0], arguments[1]
options = {"up": "z", "azimuth": "-60", "elevation": "20", "size": "900", "samples": "24"}
flags = set()
rest = arguments[2:]
while rest:
    name = rest.pop(0).lstrip("-")
    if name in options:
        options[name] = rest.pop(0)
    else:
        flags.add(name)

for obj in list(bpy.data.objects):
    bpy.data.objects.remove(obj)
surface = read_obj(source, "mesh")
mesh = surface.data
if options["up"] == "y":
    # Bring y up to z, the way Blender stands.
    mesh.transform(Matrix.Rotation(math.radians(90.0), 4, "X"))
# What is framed is what is seen: a file may carry vertices no face uses,
# far from the rest.
seen = sorted({index for polygon in mesh.polygons for index in polygon.vertices})
low = Vector(tuple(min(mesh.vertices[v].co[k] for v in seen) for k in range(3)))
high = Vector(tuple(max(mesh.vertices[v].co[k] for v in seen) for k in range(3)))
centre, extent = (low + high) / 2, max((high - low).length, 1e-12)
mesh.transform(Matrix.Scale(2.0 / extent, 4) @ Matrix.Translation(-centre))
for polygon in mesh.polygons:
    polygon.use_smooth = False


def material(name, colour, roughness=0.6):
    made = bpy.data.materials.new(name)
    made.use_nodes = True
    shader = made.node_tree.nodes["Principled BSDF"]
    shader.inputs["Base Color"].default_value = (*colour, 1.0)
    shader.inputs["Roughness"].default_value = roughness
    return made


mesh.materials.append(material("surface", (0.50, 0.60, 0.72)))

if "no-edges" not in flags:
    edges = bpy.data.objects.new("edges", mesh.copy())
    bpy.context.collection.objects.link(edges)
    edges.data.materials.clear()
    edges.data.materials.append(material("edges", (0.02, 0.03, 0.05), 0.9))
    lengths = [(mesh.vertices[e.vertices[0]].co - mesh.vertices[e.vertices[1]].co).length for e in mesh.edges]
    wire = edges.modifiers.new("wire", "WIREFRAME")
    # Thin against the typical edge, but never thinner than a pixel or so.
    wire.thickness = max(0.035 * sorted(lengths)[len(lengths) // 2], 2.5 / int(options["size"]))
    wire.use_replace = True
    wire.use_even_offset = False

azimuth, elevation = math.radians(float(options["azimuth"])), math.radians(float(options["elevation"]))
towards = Vector((math.cos(elevation) * math.cos(azimuth), math.cos(elevation) * math.sin(azimuth), math.sin(elevation)))
camera = bpy.data.objects.new("camera", bpy.data.cameras.new("camera"))
bpy.context.collection.objects.link(camera)
camera.data.type = "ORTHO"
camera.location = 10.0 * towards
camera.rotation_euler = (-towards).to_track_quat("-Z", "Y").to_euler()
# The mesh fits in a ball of radius 1: frame what the camera actually sees.
side, up = towards.cross(Vector((0, 0, 1))).normalized(), None
up = side.cross(towards).normalized()
across = max(abs(mesh.vertices[v].co.dot(side)) for v in seen)
upwards = max(abs(mesh.vertices[v].co.dot(up)) for v in seen)
camera.data.ortho_scale = 2.1 * max(across, upwards)

sun = bpy.data.objects.new("sun", bpy.data.lights.new("sun", "SUN"))
bpy.context.collection.objects.link(sun)
sun.data.energy = 1.6
sun.data.angle = math.radians(20.0)
sun.rotation_euler = (-(towards + 0.6 * up - 0.5 * side)).to_track_quat("-Z", "Y").to_euler()

scene = bpy.context.scene
scene.camera = camera
world = bpy.data.worlds.new("world")
world.use_nodes = True
world.node_tree.nodes["Background"].inputs["Color"].default_value = (1.0, 1.0, 1.0, 1.0)
world.node_tree.nodes["Background"].inputs["Strength"].default_value = 0.6
scene.world = world
scene.render.engine = "CYCLES"
scene.cycles.device = "CPU"
scene.cycles.samples = int(options["samples"])
scene.cycles.use_denoising = False
scene.render.film_transparent = True
scene.render.resolution_x = scene.render.resolution_y = int(options["size"])
scene.render.resolution_percentage = 100
# By the extension asked for: WebP keeps a page of pictures light.
scene.render.image_settings.file_format = "WEBP" if target.lower().endswith(".webp") else "PNG"
scene.render.image_settings.color_mode = "RGBA"
scene.render.image_settings.quality = 88
scene.render.filepath = os.path.abspath(target)
scene.view_settings.view_transform = "Standard"
bpy.ops.render.render(write_still=True)
print(f"RENDER wrote {target}")
