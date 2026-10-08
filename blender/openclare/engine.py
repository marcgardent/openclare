"""The openclare engine, loaded from the native library that ships with the
add-on and called through its C interface (see crates/openclare-capi)."""

import ctypes
import sys
from pathlib import Path

_NAMES = {"linux": "libopenclare_capi.so", "darwin": "libopenclare_capi.dylib", "win32": "openclare_capi.dll"}
_library = None


class EngineError(RuntimeError):
    """The engine could not be loaded, or gave no mesh."""


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
    lib = ctypes.CDLL(str(path))

    pointer, size, text = ctypes.c_void_p, ctypes.c_size_t, ctypes.c_char_p
    lib.openclare_version.restype = text
    lib.openclare_remesh.restype = pointer
    lib.openclare_remesh.argtypes = [pointer, size, pointer, size, pointer, size, pointer, text, text]
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
    lib.openclare_result_free.restype = None
    _library = lib
    return lib


def version():
    return library().openclare_version().decode()


def remesh(positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count, preset, settings):
    """Remeshes into quads the triangle mesh whose arrays sit at the addresses
    given: three floats per vertex at `positions`, three corners per triangle
    at `triangles`, the vertex of each corner at `corner_vertices`. They are
    read in place.

    `settings` maps setting names to values, on top of the preset.

    Returns `(positions, sizes, corners, report)`: numpy arrays of the
    coordinates, of the number of corners of each face and of the vertex of
    each corner, and what the engine says of the run as a dictionary.
    """
    import numpy

    lib = library()
    lines = "".join(f"{key} = {value}\n" for key, value in settings.items())
    result = lib.openclare_remesh(
        positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count, None,
        preset.encode(), lines.encode(),
    )
    try:
        if not lib.openclare_result_ok(result):
            raise EngineError(lib.openclare_result_error(result).decode())
        out_positions = numpy.empty(3 * lib.openclare_result_vertex_count(result), numpy.float32)
        sizes = numpy.empty(lib.openclare_result_face_count(result), numpy.uint32)
        corners = numpy.empty(lib.openclare_result_corner_count(result), numpy.uint32)
        lib.openclare_result_copy_positions(result, out_positions.ctypes.data)
        lib.openclare_result_copy_faces(result, sizes.ctypes.data, corners.ctypes.data)
        report = {}
        for line in lib.openclare_result_report(result).decode().splitlines():
            key, _, value = line.partition(" = ")
            report[key] = value
        return out_positions, sizes, corners, report
    finally:
        lib.openclare_result_free(result)
