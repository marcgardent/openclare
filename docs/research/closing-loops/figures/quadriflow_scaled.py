"""QuadriFlow, as bench/quadriflow.py runs it, on a mesh scaled up first.

    blender -b --factory-startup --python quadriflow_scaled.py -- <in.obj> <out.obj> <faces> <seed> <scale>

Blender refuses to run QuadriFlow on a mesh with an edge whose ends are
within 1e-4 of each other in every coordinate (it reports the mesh as not
manifold). The male body source, 1.35 million triangles 1.7 units tall, has
such edges, which is why the bench records QuadriFlow as cancelled on it.
QuadriFlow does not depend on the scale (it is asked for a number of faces),
so the mesh is scaled by <scale> about the origin, remeshed with the
parameters of bench/quadriflow.py, and scaled back.
"""

import os
import sys

import bpy

REPOSITORY = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", ".."))
sys.path.insert(0, os.path.join(REPOSITORY, "bench"))
from blender_io import activate, read_obj, write_obj  # noqa: E402

arguments = sys.argv[sys.argv.index("--") + 1:]
source_path, out_path, faces, seed, scale = arguments[0], arguments[1], int(arguments[2]), int(arguments[3]), float(
    arguments[4])

bpy.ops.wm.read_factory_settings(use_empty=True)
obj = read_obj(source_path)
obj.data.transform([[scale, 0, 0, 0], [0, scale, 0, 0], [0, 0, scale, 0], [0, 0, 0, 1]])
obj.data.update()
activate(obj)
result = bpy.ops.object.quadriflow_remesh(
    use_mesh_symmetry=False,
    use_preserve_sharp=False,
    use_preserve_boundary=False,
    preserve_attributes=False,
    smooth_normals=False,
    mode="FACES",
    target_faces=faces,
    seed=seed,
)
if "FINISHED" not in result:
    sys.exit(f"QUADRIFLOW failed: {result}")
obj.data.transform([[1 / scale, 0, 0, 0], [0, 1 / scale, 0, 0], [0, 0, 1 / scale, 0], [0, 0, 0, 1]])
write_obj(out_path, obj.data)
print(f"QUADRIFLOW ok {len(obj.data.polygons)} faces", flush=True)
