"""openclare Triangles against Blender's Decimate, model by model, at the same
number of triangles where the method aims at one. Nothing is saved but the
JSON of the measures.

    blender -b --factory-startup --python bench/triangles_vs_decimate.py -- blender bench/results/triangles-vs-decimate-6000.json 6000 bench/out/<model>/source.obj ...
"""
import json
import math
import os
import sys
import time

import bmesh
import bpy
import numpy
from mathutils.bvhtree import BVHTree

args = sys.argv[sys.argv.index("--") + 1:]
folder, out, count, models = args[0], args[1], int(args[2]), args[3:]
sys.path.insert(0, folder)
import openclare  # noqa: E402

openclare.register()


def load(path):
    for thing in list(bpy.data.objects):
        bpy.data.objects.remove(thing)
    bpy.ops.wm.obj_import(filepath=path)
    source = bpy.context.selected_objects[0]
    bpy.context.view_layer.objects.active = source
    return source


def triangulated(obj):
    """A copy of the mesh of `obj`, modifiers applied, made of triangles."""
    depsgraph = bpy.context.evaluated_depsgraph_get()
    mesh = bpy.data.meshes.new_from_object(obj.evaluated_get(depsgraph))
    work = bmesh.new()
    work.from_mesh(mesh)
    bmesh.ops.triangulate(work, faces=work.faces[:])
    work.to_mesh(mesh)
    work.free()
    return mesh


def measure(mesh, source_tree, source_points, diagonal):
    positions = numpy.empty(3 * len(mesh.vertices))
    mesh.vertices.foreach_get("co", positions)
    positions = positions.reshape(-1, 3)
    triangles = numpy.empty(3 * len(mesh.polygons), numpy.int64)
    mesh.polygons.foreach_get("vertices", triangles)
    triangles = triangles.reshape(-1, 3)
    corners = positions[triangles]
    sides = [numpy.linalg.norm(corners[:, (k + 1) % 3] - corners[:, k], axis=1) for k in range(3)]
    sides = numpy.stack(sides, axis=1)
    angles = []
    for k in range(3):
        a, b = corners[:, (k + 1) % 3] - corners[:, k], corners[:, (k + 2) % 3] - corners[:, k]
        cos = (a * b).sum(1) / numpy.maximum(numpy.linalg.norm(a, axis=1) * numpy.linalg.norm(b, axis=1), 1e-30)
        angles.append(numpy.degrees(numpy.arccos(numpy.clip(cos, -1, 1))))
    least = numpy.min(numpy.stack(angles, 1), 1)
    ratio = sides.max(1) / numpy.maximum(sides.min(1), 1e-30)
    tree = BVHTree.FromPolygons([tuple(p) for p in positions], [tuple(t) for t in triangles])
    out_of = max(source_tree.find_nearest(tuple(p))[3] for p in positions)
    back = max(tree.find_nearest(tuple(p))[3] for p in source_points)
    work = bmesh.new()
    work.from_mesh(mesh)
    open_edges = sum(1 for e in work.edges if len(e.link_faces) == 1)
    non_manifold = sum(1 for e in work.edges if len(e.link_faces) > 2)
    work.free()
    crossing = set()
    for i, j in tree.overlap(tree):
        if i < j and not set(triangles[i]) & set(triangles[j]):
            crossing.update((i, j))
    return {
        "triangles": len(triangles),
        "least_angle_mean": float(least.mean()),
        "least_angle_p1": float(numpy.percentile(least, 1)),
        "edge_ratio_mean": float(ratio.mean()),
        "edge_ratio_p99": float(numpy.percentile(ratio, 99)),
        "hausdorff_percent": 100.0 * max(out_of, back) / diagonal,
        "open_edges": open_edges,
        "non_manifold_edges": non_manifold,
        "crossing": len(crossing),
    }


results = {}
for path in models:
    name = os.path.basename(os.path.dirname(path))
    source = load(path)
    mesh = source.data
    corners = [source.matrix_world @ v.co for v in mesh.vertices]
    diagonal = max((max(c[k] for c in corners) - min(c[k] for c in corners)) for k in range(3)) * math.sqrt(3)
    source_mesh = triangulated(source)
    source_tris = len(source_mesh.polygons)
    source_tree = BVHTree.FromPolygons([tuple(v.co) for v in source_mesh.vertices], [tuple(p.vertices) for p in source_mesh.polygons])
    step = max(1, len(source_mesh.vertices) // 40000)
    source_points = [tuple(source_mesh.vertices[i].co) for i in range(0, len(source_mesh.vertices), step)]
    row = {"source_triangles": source_tris}

    # openclare, Triangles
    settings = source.openclare
    settings.output, settings.quads = "TRIANGLES", count
    started = time.perf_counter()
    assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
    seconds = time.perf_counter() - started
    made = bpy.context.active_object
    row["openclare"] = {**measure(made.data, source_tree, source_points, diagonal), "seconds": seconds}
    bpy.data.objects.remove(made)

    def decimated(kind, **setting):
        bpy.context.view_layer.objects.active = source
        modifier = source.modifiers.new("decimate", "DECIMATE")
        modifier.decimate_type = kind
        for key, value in setting.items():
            setattr(modifier, key, value)
        started = time.perf_counter()
        result = triangulated(source)
        seconds = time.perf_counter() - started
        source.modifiers.remove(modifier)
        measured = {**measure(result, source_tree, source_points, diagonal), "seconds": seconds}
        bpy.data.meshes.remove(result)
        return measured

    row["collapse"] = decimated("COLLAPSE", ratio=min(1.0, count / source_tris))
    row["planar"] = decimated("DISSOLVE", angle_limit=math.radians(5.0))
    # Un-Subdivide undoes subdivision levels: as many as bring it nearest.
    iterations = max(1, round(math.log(max(source_tris / count, 1.0), 4)))
    row["unsubdivide"] = {**decimated("UNSUBDIV", iterations=iterations), "iterations": iterations}
    results[name] = row
    print("DONE", name, {k: (v["triangles"], round(v["hausdorff_percent"], 2)) for k, v in row.items() if isinstance(v, dict)})
    bpy.data.meshes.remove(source_mesh)
    with open(out, "w") as file:
        json.dump(results, file, indent=1)

with open(out, "w") as file:
    json.dump(results, file, indent=1)
print("WROTE", out)
