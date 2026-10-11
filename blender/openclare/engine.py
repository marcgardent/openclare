"""The openclare engine, loaded from the native library that ships with the
add-on and called through its C interface (see crates/openclare-capi)."""

import ctypes
import sys
from pathlib import Path

# What the engine calls as each stage begins: the name of the stage and a
# pointer it was given, back; 0 to go on, anything else to stop.
ON_STAGE = ctypes.CFUNCTYPE(ctypes.c_int32, ctypes.c_char_p, ctypes.c_void_p)

_NAMES = {"linux": "libopenclare_capi.so", "darwin": "libopenclare_capi.dylib", "win32": "openclare_capi.dll"}
_library = None


class EngineError(RuntimeError):
    """The engine could not be loaded, or gave no mesh."""


def _own_copy(path):
    """A copy of the library at `path` under a name made from what it holds,
    in a folder of the user's own, which is what gets loaded.

    A library is known to the system by the path it was loaded from. When the
    add-on is updated in a Blender that has already used it, the file is
    replaced but the path is not, and asking for it again gives back the one
    already in memory: the new add-on then talks to the old engine, which
    takes its arguments for something else and makes nonsense of them. A name
    that changes with the contents is never taken for another. (And the file
    in the add-on's folder is then never held open, which lets a system that
    locks what it has loaded replace it.)"""
    import hashlib
    import os
    import shutil
    import tempfile

    held = path.read_bytes()
    digest = hashlib.sha256(held).hexdigest()[:16]
    owner = os.getuid() if hasattr(os, "getuid") else os.environ.get("USERNAME", "user")
    folder = Path(tempfile.gettempdir()) / f"openclare-engine-{owner}"
    folder.mkdir(mode=0o700, exist_ok=True)
    if hasattr(os, "getuid") and (folder.stat().st_uid != os.getuid() or folder.stat().st_mode & 0o077):
        raise EngineError(f"{folder} is not a folder of this user alone: the engine is not loaded from it")
    copy = folder / f"{digest}-{path.name}"
    if not copy.exists() or hashlib.sha256(copy.read_bytes()).hexdigest()[:16] != digest:
        # Written beside, then put in place at once: never half a library.
        with tempfile.NamedTemporaryFile(dir=folder, delete=False) as fresh:
            fresh.write(held)
        os.replace(fresh.name, copy)
    for other in folder.glob(f"*-{path.name}"):
        if other != copy:
            # Those of earlier versions; one still in use somewhere stays.
            try:
                other.unlink()
            except OSError:
                pass
    return copy


def library():
    """The native library, loaded on first use."""
    global _library
    if _library is not None:
        return _library
    name = _NAMES.get(sys.platform)
    if name is None:
        raise EngineError(f"openclare has no engine for this system ({sys.platform})")
    path = Path(__file__).parent / "lib" / name
    if not path.exists():
        raise EngineError(f"the openclare engine is missing: {path}")
    lib = ctypes.CDLL(str(_own_copy(path)))

    pointer, size, text = ctypes.c_void_p, ctypes.c_size_t, ctypes.c_char_p
    lib.openclare_version.restype = text
    lib.openclare_remesh.restype = pointer
    lib.openclare_remesh.argtypes = [
        pointer, size, pointer, size, pointer, size, pointer, pointer,
        pointer, pointer, pointer, size,
        text, text, ON_STAGE, pointer,
    ]
    lib.openclare_result_ok.restype = ctypes.c_int32
    lib.openclare_result_error.restype = text
    lib.openclare_result_report.restype = text
    for count in ("vertex", "face", "corner"):
        function = getattr(lib, f"openclare_result_{count}_count")
        function.restype, function.argtypes = size, [pointer]
    lib.openclare_result_ok.argtypes = lib.openclare_result_error.argtypes = [pointer]
    lib.openclare_result_report.argtypes = lib.openclare_result_free.argtypes = [pointer]
    lib.openclare_result_copy_positions.argtypes = [pointer, pointer]
    lib.openclare_result_copy_faces.argtypes = [pointer, pointer, pointer]
    lib.openclare_result_copy_uv.restype = ctypes.c_int32
    lib.openclare_result_copy_uv.argtypes = [pointer, ctypes.c_uint32, pointer]
    for edges in ("seam", "hard_edge"):
        count, copy = getattr(lib, f"openclare_result_{edges}_count"), getattr(lib, f"openclare_result_copy_{edges}s")
        count.restype, count.argtypes = size, [pointer]
        copy.restype, copy.argtypes = None, [pointer, pointer]
    lib.openclare_result_crease_count.restype = size
    lib.openclare_result_crease_count.argtypes = [pointer]
    lib.openclare_result_copy_creases.restype = None
    lib.openclare_result_copy_creases.argtypes = [pointer, pointer, pointer]
    lib.openclare_result_copy_face_groups.restype = ctypes.c_int32
    lib.openclare_result_copy_face_groups.argtypes = [pointer, pointer]
    lib.openclare_result_free.restype = None
    lib.openclare_bake_height.restype = pointer
    lib.openclare_bake_height.argtypes = [
        pointer, pointer, size, pointer, pointer, size, pointer, size, pointer, size,
        ctypes.c_uint32, ctypes.c_uint32, ctypes.c_float, ctypes.c_uint32,
    ]
    lib.openclare_baked_ok.restype = ctypes.c_int32
    lib.openclare_baked_error.restype = lib.openclare_baked_report.restype = text
    lib.openclare_baked_ok.argtypes = lib.openclare_baked_error.argtypes = [pointer]
    lib.openclare_baked_report.argtypes = lib.openclare_baked_free.argtypes = [pointer]
    lib.openclare_baked_copy.restype = lib.openclare_baked_free.restype = None
    lib.openclare_baked_copy.argtypes = [pointer, pointer]
    _library = lib
    return lib


def version():
    return library().openclare_version().decode()


# The layers of texture coordinates the engine can give with a mesh.
UV_LAYERS = {"grid": 0, "atlas": 1}
# What a guide curve is, as the engine's flags: its last point joins the
# first; the edges along it are hard edges of the result.
CURVE_CLOSED, CURVE_SHARP = 1, 2


class Remeshed:
    """What a run gives: `positions` (three floats per vertex), `sizes` (the
    number of corners of each face), `corners` (the vertex of each corner),
    `report` (what the engine says of the run, as a dictionary), and what was
    asked for on top: `uv` (per layer name, two floats per corner), `seams`
    and `hard_edges` (two vertices per edge), `creases` (the same, and one
    weight per edge), None when not asked for; and `face_groups`, the group
    of the input each face lies in, None when the input came without groups."""

    def __init__(self):
        self.positions = self.sizes = self.corners = None
        self.report, self.uv = {}, {}
        self.seams = self.hard_edges = self.creases = self.face_groups = None


def remesh(
    positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count, preset, settings,
    density=None, on_stage=None, uv_layers=(), seams=False, hard_edges=False, creases=False, curves=(),
    groups=None,
):
    """Remeshes into quads the triangle mesh whose arrays sit at the addresses
    given: three floats per vertex at `positions`, three corners per triangle
    at `triangles`, the vertex of each corner at `corner_vertices`. They are
    read in place.

    `settings` maps setting names to values, on top of the preset. `density`
    is None or a numpy array of one 32-bit float per vertex, from 0 where the
    quads should be largest to 1 where they should be smallest.

    `curves` are guide curves, lines drawn on the surface for the quad mesh
    to have edges along: for each, its points (a numpy array of three floats
    per point), whether its last point joins the first, and whether the
    edges along it are to be hard edges of the result.

    `groups` is None or a numpy array of one unsigned 32-bit integer per
    triangle: the material of each, say. The result has a line of edges where
    two groups meet, and says which group each of its faces lies in.

    `on_stage`, if given, is called with the name of each stage as it begins
    ("cross field", or "piece 3 of 12: cross field"); the run stops if it
    returns something true, and `EngineError` says so.

    `uv_layers` names the layers of texture coordinates wanted, among
    "grid" and "atlas"; `seams` asks for the edges the atlas is cut along,
    `hard_edges` for those that run along a hard edge of the input,
    `creases` for those to keep sharp under subdivision, with their weights.

    Returns a `Remeshed`.
    """
    import numpy

    lib = library()
    lines = "".join(f"{key} = {value}\n" for key, value in settings.items())
    if density is not None and (density.dtype != numpy.float32 or len(density) != vertex_count):
        raise EngineError(f"{len(density)} weights of type {density.dtype} for {vertex_count} vertices")
    if groups is not None and (groups.dtype != numpy.uint32 or len(groups) != triangle_count):
        raise EngineError(f"{len(groups)} groups of type {groups.dtype} for {triangle_count} triangles")
    def told(stage, _):
        # Whatever goes wrong in the host's function must not unwind into
        # the engine: the run goes on.
        try:
            return 1 if on_stage(stage.decode()) else 0
        except Exception as error:  # noqa: BLE001
            print(f"openclare: {error!r} while being told of a stage")
            return 0

    curves = [
        (numpy.ascontiguousarray(points, numpy.float32).reshape(-1, 3), closed, sharp)
        for points, closed, sharp in curves
    ]
    curves = [curve for curve in curves if len(curve[0]) >= 2]
    if curves:
        curve_points = numpy.concatenate([points for points, _, _ in curves])
        curve_sizes = numpy.array([len(points) for points, _, _ in curves], numpy.uint32)
        curve_flags = numpy.array(
            [CURVE_CLOSED * bool(closed) + CURVE_SHARP * bool(sharp) for _, closed, sharp in curves], numpy.uint8
        )
        curve_arguments = (curve_points.ctypes.data, curve_sizes.ctypes.data, curve_flags.ctypes.data, len(curves))
    else:
        curve_arguments = (None, None, None, 0)

    # Kept in a variable: the engine calls it for as long as the run lasts.
    callback = ON_STAGE(told) if on_stage else ON_STAGE()
    result = lib.openclare_remesh(
        positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count,
        None if density is None else density.ctypes.data,
        None if groups is None else groups.ctypes.data,
        *curve_arguments,
        preset.encode(), lines.encode(), callback, None,
    )
    try:
        if not lib.openclare_result_ok(result):
            raise EngineError(lib.openclare_result_error(result).decode())
        out = Remeshed()
        out.positions = numpy.empty(3 * lib.openclare_result_vertex_count(result), numpy.float32)
        out.sizes = numpy.empty(lib.openclare_result_face_count(result), numpy.uint32)
        out.corners = numpy.empty(lib.openclare_result_corner_count(result), numpy.uint32)
        lib.openclare_result_copy_positions(result, out.positions.ctypes.data)
        lib.openclare_result_copy_faces(result, out.sizes.ctypes.data, out.corners.ctypes.data)
        for line in lib.openclare_result_report(result).decode().splitlines():
            key, _, value = line.partition(" = ")
            out.report[key] = value
        for name in uv_layers:
            coordinates = numpy.empty(2 * len(out.corners), numpy.float32)
            if not lib.openclare_result_copy_uv(result, UV_LAYERS[name], coordinates.ctypes.data):
                raise EngineError(f"the engine has no layer of texture coordinates named '{name}'")
            out.uv[name] = coordinates
        if groups is not None:
            face_groups = numpy.empty(len(out.sizes), numpy.uint32)
            if lib.openclare_result_copy_face_groups(result, face_groups.ctypes.data):
                out.face_groups = face_groups
        if seams:
            out.seams = numpy.empty(2 * lib.openclare_result_seam_count(result), numpy.uint32)
            lib.openclare_result_copy_seams(result, out.seams.ctypes.data)
        if hard_edges:
            out.hard_edges = numpy.empty(2 * lib.openclare_result_hard_edge_count(result), numpy.uint32)
            lib.openclare_result_copy_hard_edges(result, out.hard_edges.ctypes.data)
        if creases:
            count = lib.openclare_result_crease_count(result)
            out.creases = (numpy.empty(2 * count, numpy.uint32), numpy.empty(count, numpy.float32))
            lib.openclare_result_copy_creases(result, out.creases[0].ctypes.data, out.creases[1].ctypes.data)
        return out
    finally:
        lib.openclare_result_free(result)


def bake_height(positions, normals, triangles, uvs, source_positions, source_triangles, width, height, reach, padding=4):
    """Bakes the relief of a detailed surface onto the texture of a plainer
    one: how far the source is from the target along the normal of the
    target, signed, for each texel.

    The target is numpy arrays: `positions` and `normals` (three 32-bit
    floats per vertex), `triangles` (three unsigned 32-bit vertices per
    triangle) and `uvs` (two 32-bit floats per corner of each triangle,
    (0, 0) at the bottom left of the texture). The source is
    `source_positions` and `source_triangles`, likewise. The source is
    looked for no farther than `reach`; `padding` texels round the faces
    are filled from their neighbours.

    Returns the heights, a numpy array of `height` rows of `width` 32-bit
    floats from the bottom row up (not a number where no face lies), and
    what the engine says of the bake, as a dictionary.
    """
    import numpy

    def flat(array, kind, per, count):
        array = numpy.ascontiguousarray(array, dtype=kind).reshape(-1)
        if len(array) != per * count:
            raise EngineError(f"{len(array)} numbers where {per} for each of {count} were expected")
        return array

    vertices, faces = len(positions.reshape(-1)) // 3, len(triangles.reshape(-1)) // 3
    source_vertices, source_faces = len(source_positions.reshape(-1)) // 3, len(source_triangles.reshape(-1)) // 3
    arrays = [
        flat(positions, numpy.float32, 3, vertices), flat(normals, numpy.float32, 3, vertices),
        flat(triangles, numpy.uint32, 3, faces), flat(uvs, numpy.float32, 6, faces),
        flat(source_positions, numpy.float32, 3, source_vertices), flat(source_triangles, numpy.uint32, 3, source_faces),
    ]
    at = [array.ctypes.data for array in arrays]
    lib = library()
    baked = lib.openclare_bake_height(
        at[0], at[1], vertices, at[2], at[3], faces, at[4], source_vertices, at[5], source_faces,
        int(width), int(height), float(reach), int(padding),
    )
    try:
        if not lib.openclare_baked_ok(baked):
            raise EngineError(lib.openclare_baked_error(baked).decode())
        report = dict(line.split(" = ", 1) for line in lib.openclare_baked_report(baked).decode().splitlines())
        heights = numpy.empty(int(width) * int(height), dtype=numpy.float32)
        lib.openclare_baked_copy(baked, heights.ctypes.data)
    finally:
        lib.openclare_baked_free(baked)
    return heights.reshape(int(height), int(width)), report
