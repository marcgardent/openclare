"""Runs Quad Remesher (Exoside) on an OBJ.

    blender -b --python bench/quadremesher.py -- <in.obj> <out.obj> <faces> <uniform 0|1> <hard edges 0|1>

Quad Remesher is commercial software: this needs its Blender add-on installed
and a valid (trial or paid) licence, and does nothing to work around either.
It calls the add-on's own start / poll / finish functions, exactly what its
"Remesh It" button does on a timer, because a modal operator cannot run in
background mode.

Prints one "QUADREMESHER <status> <seconds>" line.
"""

import os
import sys
import time

import addon_utils
import bpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from blender_io import activate, read_obj, write_obj  # noqa: E402

ADDON = "quad_remesher_1_4"
TIMEOUT_SECONDS = 900

arguments = sys.argv[sys.argv.index("--") + 1:]
source_path, output_path, faces = arguments[0], arguments[1], int(arguments[2])
uniform, hard_edges = arguments[3] == "1", arguments[4] == "1"


class Driver:
    """Stands in for the add-on's operator instance."""

    def __init__(self, operators):
        self.the_input_object = None
        self.openclareFilename = ""
        self.IsRemeshing = False
        self.progressData = operators.QRCheckProgressData()
        self.messages = []

    def report(self, level, message):
        self.messages.append(f"{'/'.join(sorted(level))}: {message}")


def finish(status, start):
    print(f"QUADREMESHER {status} {time.perf_counter() - start:.3f}", flush=True)
    sys.exit(0)


def slug(text):
    return "_".join(str(text).split())[:160] or "unknown"


bpy.ops.wm.read_factory_settings(use_empty=True)
start = time.perf_counter()
addon_utils.enable("io_scene_fbx", default_set=False)
if addon_utils.enable(ADDON, default_set=False) is None:
    finish("error:add-on_not_installed", start)
operators = sys.modules[ADDON].qr_operators

settings = bpy.context.scene.qremesher
settings.target_count = faces
# Out of the box Quad Remesher makes quads smaller where the surface curves
# and lets the count drift to do so. "Uniform" turns both off, for a
# like-for-like comparison with remeshers that only do one quad size.
settings.adaptive_size = 0 if uniform else 50
settings.adapt_quad_count = not uniform
settings.autodetect_hard_edges = hard_edges
settings.use_normals = False
settings.use_materials = False
settings.use_vertex_color = False
settings.symmetry_x = settings.symmetry_y = settings.symmetry_z = False
settings.hide_input = False

source = read_obj(source_path)
activate(source)
driver = Driver(operators)
start = time.perf_counter()
operators.doRemeshing_Start(driver, bpy.context)
if not driver.IsRemeshing:
    finish("error:" + slug("; ".join(driver.messages) or "engine did not start"), start)

while True:
    time.sleep(0.25)
    progress, text, _ = driver.progressData.get_progress_status()
    if progress == 2:
        break
    if progress == -2:
        finish("error:licence_or_EULA_needs_attention_in_the_Blender_interface", start)
    if progress is not None and progress < 0 and progress != -11:
        finish("error:" + slug(text or progress), start)
    if time.perf_counter() - start > TIMEOUT_SECONDS:
        driver.progressData.RemeshingProcess.terminate()
        finish("error:timeout", start)

seconds_start = start
operators.doRemeshing_Finish(driver, bpy.context)
result = bpy.context.view_layer.objects.active
if result is None or result == source or result.type != "MESH":
    finish("error:no_result_imported", seconds_start)
mesh = result.data.copy()
mesh.transform(result.matrix_world)
write_obj(output_path, mesh)
finish("ok", seconds_start)
