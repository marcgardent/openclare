# Subdivides (Catmull-Clark, level 2, with the creases the engine wrote) the quad output and the n-gon
# output of the same run, and compares them: folded faces, distance to the source, distance to each other.
# Each output has its creases next to it, as --creases-out writes them: <output>.creases for <output>.obj.
# Usage: blender -b --factory-startup --python bench/ngons.py -- <name> <source.obj> <quads.obj> <ngons.obj>
import sys
import bpy, bmesh
import numpy as np
from mathutils.bvhtree import BVHTree

name, source_path, quad_path, ngon_path = sys.argv[sys.argv.index("--") + 1:]
bpy.ops.wm.read_factory_settings(use_empty=True)


def read_obj(path):
    vertices, faces = [], []
    for line in open(path):
        if line.startswith("v "):
            vertices.append(tuple(float(x) for x in line.split()[1:4]))
        elif line.startswith("f "):
            faces.append(tuple(int(t.split("/")[0]) - 1 for t in line.split()[1:]))
    return vertices, faces


def subdivided(path):
    vertices, faces = read_obj(path)
    mesh = bpy.data.meshes.new(path)
    mesh.from_pydata(vertices, [], faces)
    mesh.update()
    obj = bpy.data.objects.new(path, mesh)
    bpy.context.scene.collection.objects.link(obj)
    bm = bmesh.new()
    bm.from_mesh(mesh)
    layer = bm.edges.layers.float.new("crease_edge")
    weights = {}
    for line in open(path[:-4] + ".creases"):
        a, b, w = line.split()
        weights[frozenset((int(a) - 1, int(b) - 1))] = float(w)
    found = 0
    for e in bm.edges:
        w = weights.get(frozenset(v.index for v in e.verts))
        if w is not None:
            e[layer] = w
            found += 1
    assert found == len(weights), (found, len(weights))
    bm.to_mesh(mesh)
    bm.free()
    m = obj.modifiers.new("s", "SUBSURF")
    m.levels = 2
    m.use_creases = True
    evaluated = obj.evaluated_get(bpy.context.evaluated_depsgraph_get())
    out = evaluated.to_mesh()
    co = np.array([v.co[:] for v in out.vertices])
    polys = [tuple(p.vertices) for p in out.polygons]
    normals = np.array([p.normal[:] for p in out.polygons])
    centres = np.array([p.center[:] for p in out.polygons])
    sides = [len(f) for f in faces]
    return co, polys, normals, centres, sides


source_vertices, source_faces = read_obj(source_path)
source = BVHTree.FromPolygons(source_vertices, source_faces)
box = np.array(source_vertices)
diagonal = float(np.linalg.norm(box.max(0) - box.min(0)))


def against_source(co, normals, centres):
    far = max(source.find_nearest(p)[3] for p in co) / diagonal
    folded = 0
    for c, n in zip(centres, normals):
        hit = source.find_nearest(c)
        folded += float(np.dot(n, hit[1])) < 0.0
    return far, folded


quads = subdivided(quad_path)
ngons = subdivided(ngon_path)
far_q, folded_q = against_source(quads[0], quads[2], quads[3])
far_n, folded_n = against_source(ngons[0], ngons[2], ngons[3])
tree_q = BVHTree.FromPolygons([tuple(p) for p in quads[0]], quads[1])
tree_n = BVHTree.FromPolygons([tuple(p) for p in ngons[0]], ngons[1])
apart = max(max(tree_q.find_nearest(p)[3] for p in ngons[0]), max(tree_n.find_nearest(p)[3] for p in quads[0])) / diagonal
many = [s for s in ngons[4] if s != 4]
print(f"RESULT {name} | faces {len(quads[4])} -> {len(ngons[4])} | merged-like faces {len(many)}, most sides {max(many, default=0)}"
      f" | folded quads {folded_q} ngons {folded_n} | to source quads {far_q:.2e} ngons {far_n:.2e} | apart {apart:.2e}")
