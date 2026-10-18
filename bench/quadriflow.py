"""Runs QuadriFlow, the remesher built into Blender, on an OBJ, once per seed.

    blender -b --python bench/quadriflow.py -- <in.obj> <out prefix> <faces> <sharp 0|1> <boundary 0|1> <seed>...

Writes <out prefix>_<seed>.obj and prints one "QUADRIFLOW <seed> <status> <seconds>" line per run.
"""

import os
import sys
import time

import bpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from blender_io import activate, read_obj, write_obj  # noqa: E402

arguments = sys.argv[sys.argv.index("--") + 1:]
source_path, prefix, faces, sharp, boundary = arguments[0], arguments[1], int(arguments[2]), arguments[3], arguments[4]
seeds = [int(s) for s in arguments[5:]]

bpy.ops.wm.read_factory_settings(use_empty=True)
for seed in seeds:
    obj = read_obj(source_path, f"source_{seed}")
    activate(obj)
    start = time.perf_counter()
    try:
        result = bpy.ops.object.quadriflow_remesh(
            # On by default; it would mirror the mesh and distort the comparison.
            use_mesh_symmetry=False,
            use_preserve_sharp=sharp == "1",
            use_preserve_boundary=boundary == "1",
            preserve_attributes=False,
            smooth_normals=False,
            mode="FACES",
            target_faces=faces,
            seed=seed,
        )
        status = "ok" if "FINISHED" in result else "cancelled"
    except RuntimeError as error:
        status = "error:" + str(error).strip().replace("\n", " ")[:120].replace(" ", "_")
    seconds = time.perf_counter() - start
    if status == "ok":
        write_obj(f"{prefix}_{seed}.obj", obj.data)
    print(f"QUADRIFLOW {seed} {status} {seconds:.3f}", flush=True)
    bpy.data.objects.remove(obj)
