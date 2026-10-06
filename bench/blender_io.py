"""Plain OBJ in and out of Blender, without the importer's axis conversions.

Every tool of the bench must see exactly the same coordinates, so vertices
and faces are copied verbatim instead of going through Blender's OBJ add-on.
"""

import bmesh
import bpy


def read_obj(path, name="source"):
    vertices, faces = [], []
    with open(path) as lines:
        for line in lines:
            tokens = line.split()
            if not tokens:
                continue
            if tokens[0] == "v":
                vertices.append(tuple(float(x) for x in tokens[1:4]))
            elif tokens[0] == "f":
                faces.append([int(t.split("/")[0]) - 1 for t in tokens[1:]])
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(vertices, [], faces)
    mesh.update()
    obj = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(obj)
    return obj


def write_obj(path, mesh):
    with open(path, "w") as out:
        for v in mesh.vertices:
            out.write(f"v {v.co.x!r} {v.co.y!r} {v.co.z!r}\n")
        for polygon in mesh.polygons:
            out.write("f " + " ".join(str(i + 1) for i in polygon.vertices) + "\n")


def evaluated_triangles(obj):
    """A new triangulated mesh with all of `obj`'s modifiers applied."""
    depsgraph = bpy.context.evaluated_depsgraph_get()
    mesh = bpy.data.meshes.new_from_object(obj.evaluated_get(depsgraph))
    mesh.transform(obj.matrix_world)
    bm = bmesh.new()
    bm.from_mesh(mesh)
    bmesh.ops.triangulate(bm, faces=bm.faces)
    bmesh.ops.remove_doubles(bm, verts=bm.verts, dist=1e-7)
    bm.to_mesh(mesh)
    bm.free()
    return mesh


def activate(obj):
    for other in bpy.data.objects:
        other.select_set(False)
    obj.select_set(True)
    bpy.context.view_layer.objects.active = obj
