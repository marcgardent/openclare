"""Renders a quad mesh with its edge loops coloured by kind, in Blender.

    blender -b --factory-startup --python render_loops.py -- <mesh.obj> <out.png> <view>

<view> is one of the VIEWS below. The edge loops are sorted by loops.py, the
port of `openclare measure` (Section 5 of the paper): closed loops in dark
blue, spiral loops in vermillion, drawn wider and with a dark rim (to tell them
from closed loops in greyscale too), loops that stop at an
irregular vertex in grey. The surface is light grey, shaded by a fixed light
baked into vertex colours; Workbench renders everything flat, so the colours
in the image are exactly those below, whatever the view. Each edge is drawn
as a thin ribbon facing the orthographic camera, moved slightly towards it
so that the surface does not hide it.
"""

import math
import os
import sys

import bpy
import numpy as np

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from loops import edge_loops, read_obj  # noqa: E402

# Orthographic views of the male body (Z up, facing -Y; its left is +X).
# centre (x, z), width of the view, resolution in pixels (x, y), line width
# in pixels.
VIEWS = {
    "overview": {"centre": (0.0, 0.84), "width": 0.95, "resolution": (620, 1150), "line": 1.6},
    "wrist": {"centre": (0.365, 0.855), "width": 0.22, "resolution": (600, 800), "line": 3.0},
    "eye": {"centre": (-0.032, 1.575), "width": 0.12, "resolution": (600, 600), "line": 3.0},
}

SURFACE = (0.86, 0.86, 0.86)
COLOURS = {
    "closed": (0x0B / 255, 0x3C / 255, 0x8C / 255),  # dark blue
    "spiral": (0xD5 / 255, 0x5E / 255, 0x00 / 255),  # vermillion (Okabe-Ito)
    "open": (0.55, 0.55, 0.55),  # grey
    "border": (0.55, 0.55, 0.55),
    "none": (0.55, 0.55, 0.55),  # edges on no loop: those of the few triangles of the hand-made mesh
}
RIM = (0x5A / 255, 0x24 / 255, 0x00 / 255)  # dark brown rim of the spiral loops
# Width of each kind, as a multiple of the view's line width, and its order
# in depth (higher is nearer the camera, drawn over the others).
WIDTH = {"closed": 1.0, "spiral": 2.0, "open": 0.8, "border": 0.8, "none": 0.8}
DEPTH = {"closed": 2, "spiral": 3, "open": 1, "border": 1, "none": 1}

VIEW_DIRECTION = np.array([0.0, 1.0, 0.0])  # from the camera into the scene
LIGHT = np.array([-0.35, -1.0, 0.55])  # towards the light: front, upper left


def srgb_to_linear(c):
    c = np.asarray(c, dtype=float)
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def add_mesh(name, positions, faces, colours):
    """A mesh object with one colour per face corner (linear RGB)."""
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(positions.tolist(), [], faces)
    mesh.update()
    attribute = mesh.color_attributes.new("colour", "FLOAT_COLOR", "CORNER")
    rgba = np.ones((len(colours), 4))
    rgba[:, :3] = colours
    attribute.data.foreach_set("color", rgba.ravel())
    mesh.color_attributes.active_color = attribute
    obj = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(obj)
    return obj


def surface(positions, faces):
    normals = np.zeros_like(positions)
    for polygon in faces:
        p = positions[polygon]
        # Newell's normal, area weighted.
        n = np.cross(p, np.roll(p, -1, axis=0)).sum(axis=0)
        normals[polygon] += n
    normals /= np.maximum(np.linalg.norm(normals, axis=1, keepdims=True), 1e-12)
    light = LIGHT / np.linalg.norm(LIGHT)
    # Two-sided: the meshes need not agree on which way their faces point.
    shade = 0.55 + 0.45 * np.abs(normals @ light)
    vertex_colours = shade[:, None] * np.array(SURFACE)[None, :]
    corners = np.array([vertex_colours[v] for polygon in faces for v in polygon])
    return add_mesh("surface", positions, faces, srgb_to_linear(corners))


def ribbons(positions, edges, kind, half_width, bias, colour=None, extension=None):
    """One quad per edge, in the plane facing the camera, extended by its
    half width at both ends so that consecutive edges join."""
    a = positions[[e[0] for e in edges]]
    b = positions[[e[1] for e in edges]]
    along = b - a
    along /= np.maximum(np.linalg.norm(along, axis=1, keepdims=True), 1e-12)
    side = np.cross(along, VIEW_DIRECTION)
    side /= np.maximum(np.linalg.norm(side, axis=1, keepdims=True), 1e-12)
    extension = half_width if extension is None else extension
    a = a - along * extension - VIEW_DIRECTION * bias
    b = b + along * extension - VIEW_DIRECTION * bias
    side *= half_width
    corners = np.stack([a - side, b - side, b + side, a + side], axis=1).reshape(-1, 3)
    faces = [[4 * k, 4 * k + 1, 4 * k + 2, 4 * k + 3] for k in range(len(edges))]
    colours = np.tile(srgb_to_linear(colour or COLOURS[kind]), (len(corners), 1))
    return add_mesh("loops_" + kind, corners, faces, colours)


def main():
    arguments = sys.argv[sys.argv.index("--") + 1:]
    mesh_path, out_path, view_name = arguments[0], arguments[1], arguments[2]
    view = VIEWS[view_name]

    bpy.ops.wm.read_factory_settings(use_empty=True)
    scene = bpy.context.scene
    positions, faces = read_obj(mesh_path)
    positions = np.array(positions)
    result = edge_loops(len(positions), faces)
    if result is None:
        sys.exit(f"{mesh_path}: not a consistently oriented manifold, no loops to colour")
    summary, kind_of_edge = result

    surface(positions, faces)
    width, (res_x, res_y) = view["width"], view["resolution"]
    pixel = width / res_x
    edge_length = np.mean([np.linalg.norm(positions[a] - positions[b]) for a, b in kind_of_edge])
    for polygon in faces:
        for k in range(len(polygon)):
            a, b = polygon[k], polygon[(k + 1) % len(polygon)]
            kind_of_edge.setdefault((min(a, b), max(a, b)), "none")
    for kind in ("none", "open", "border", "closed", "spiral"):
        edges = [edge for edge, k in kind_of_edge.items() if k == kind]
        if edges:
            half_width = 0.5 * view["line"] * WIDTH[kind] * pixel
            bias = 0.25 * edge_length * DEPTH[kind]
            if kind == "spiral":
                # A dark rim, one pixel wide, so that spiral loops stand out
                # from closed ones in greyscale too, by more than their width.
                ribbons(positions, edges, "spiral_rim", half_width + 0.9 * view["line"] * pixel, 0.9 * bias, RIM,
                        extension=half_width)
            ribbons(positions, edges, kind, half_width, bias)

    camera_data = bpy.data.cameras.new("camera")
    camera_data.type = "ORTHO"
    camera_data.ortho_scale = width * max(1.0, res_y / res_x)
    camera_data.clip_start, camera_data.clip_end = 0.01, 10.0
    camera = bpy.data.objects.new("camera", camera_data)
    scene.collection.objects.link(camera)
    x, z = view["centre"]
    camera.location = (x, -3.0, z)
    camera.rotation_euler = (math.pi / 2, 0.0, 0.0)  # looking along +Y, Z up
    scene.camera = camera

    scene.render.engine = "BLENDER_WORKBENCH"
    scene.display.shading.light = "FLAT"
    scene.display.shading.color_type = "VERTEX"
    scene.display.render_aa = "32"
    scene.view_settings.view_transform = "Standard"
    scene.world = bpy.data.worlds.new("world")
    scene.world.color = (1.0, 1.0, 1.0)
    scene.render.film_transparent = False
    scene.render.resolution_x, scene.render.resolution_y = res_x, res_y
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGB"
    scene.render.filepath = os.path.abspath(out_path)
    bpy.ops.render.render(write_still=True)
    print("RENDERED", out_path, summary, flush=True)


main()
