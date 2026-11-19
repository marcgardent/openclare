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
        pointer, size,
        pointer, pointer, pointer, size,
        pointer, pointer, size,
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
    lib.openclare_result_curve_edge_count.restype = size
    lib.openclare_result_curve_edge_count.argtypes = [pointer]
    lib.openclare_result_copy_curve_edges.restype = None
    lib.openclare_result_copy_curve_edges.argtypes = [pointer, pointer, pointer]
    lib.openclare_result_pole_count.restype = size
    lib.openclare_result_pole_count.argtypes = [pointer]
    lib.openclare_result_copy_poles.restype = None
    lib.openclare_result_copy_poles.argtypes = [pointer, pointer, pointer, pointer, pointer, pointer]
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
    lib.openclare_project_level.restype = pointer
    lib.openclare_project_level.argtypes = [
        pointer, size, pointer, size, pointer, pointer, size, pointer, size, pointer, pointer, size, pointer, size,
        ctypes.c_char_p,
    ]
    lib.openclare_level_ok.restype = ctypes.c_int32
    lib.openclare_level_error.restype = lib.openclare_level_report.restype = text
    lib.openclare_level_ok.argtypes = lib.openclare_level_error.argtypes = [pointer]
    lib.openclare_level_report.argtypes = lib.openclare_level_free.argtypes = [pointer]
    lib.openclare_level_copy_positions.restype = lib.openclare_level_copy_status.restype = None
    lib.openclare_level_free.restype = None
    lib.openclare_level_copy_positions.argtypes = lib.openclare_level_copy_status.argtypes = [pointer, pointer]
    lib.openclare_level_copy_missing.restype = lib.openclare_level_copy_verdict.restype = None
    lib.openclare_level_copy_missing.argtypes = lib.openclare_level_copy_verdict.argtypes = [pointer, pointer]
    lib.openclare_analyse.restype = pointer
    lib.openclare_analyse.argtypes = [pointer, size, pointer, size, ctypes.c_char_p, ctypes.c_char_p, pointer]
    lib.openclare_analysis_ok.restype = ctypes.c_int32
    lib.openclare_analysis_error.restype = text
    lib.openclare_analysis_ok.argtypes = lib.openclare_analysis_error.argtypes = [pointer]
    double = ctypes.POINTER(ctypes.c_double)
    lib.openclare_analysis_proposal.restype = ctypes.c_int32
    lib.openclare_analysis_proposal.argtypes = [pointer, double, double]
    lib.openclare_analysis_count.restype = ctypes.c_int32
    lib.openclare_analysis_count.argtypes = [pointer, ctypes.c_double, ctypes.c_double, double, double]
    lib.openclare_analysis_vertices.restype = ctypes.c_int32
    lib.openclare_analysis_vertices.argtypes = [pointer, ctypes.c_double, ctypes.c_double, pointer, pointer]
    lib.openclare_analysis_vertices_for.restype = ctypes.c_int32
    lib.openclare_analysis_vertices_for.argtypes = [pointer, ctypes.c_double, pointer]
    lib.openclare_analysis_free.restype = None
    lib.openclare_analysis_free.argtypes = [pointer]
    lib.openclare_hard_edges.restype = pointer
    lib.openclare_hard_edges.argtypes = [pointer, size, pointer, size, pointer, size, pointer, text, text]
    lib.openclare_hard_edges_ok.restype = ctypes.c_int32
    lib.openclare_hard_edges_error.restype = text
    for edges in ("kept", "left_out"):
        count = getattr(lib, f"openclare_hard_edges_{edges}_count")
        copy = getattr(lib, f"openclare_hard_edges_copy_{edges}")
        count.restype, count.argtypes = size, [pointer]
        copy.restype, copy.argtypes = None, [pointer, pointer]
    lib.openclare_hard_edges_ok.argtypes = lib.openclare_hard_edges_error.argtypes = [pointer]
    lib.openclare_hard_edges_free.restype = None
    lib.openclare_hard_edges_free.argtypes = [pointer]
    lib.openclare_hard_edge_finder.restype = pointer
    lib.openclare_hard_edge_finder.argtypes = [pointer, size, pointer, size, pointer, size]
    lib.openclare_hard_edge_finder_ok.restype = ctypes.c_int32
    lib.openclare_hard_edge_finder_error.restype = text
    lib.openclare_hard_edge_finder_ok.argtypes = lib.openclare_hard_edge_finder_error.argtypes = [pointer]
    lib.openclare_hard_edge_finder_find.restype = pointer
    lib.openclare_hard_edge_finder_find.argtypes = [pointer, pointer, text, text]
    lib.openclare_hard_edge_finder_free.restype = None
    lib.openclare_hard_edge_finder_free.argtypes = [pointer]
    double = ctypes.c_double
    lib.openclare_reflection_check.restype = pointer
    lib.openclare_reflection_check.argtypes = [
        pointer, size, pointer, size, pointer, size, pointer, pointer, size, double, double, double,
    ]
    lib.openclare_reflection_ok.restype = ctypes.c_int32
    lib.openclare_reflection_error.restype = text
    for count in ("fault", "pole", "part"):
        function = getattr(lib, f"openclare_reflection_{count}_count")
        function.restype, function.argtypes = size, [pointer]
    lib.openclare_reflection_ok.argtypes = lib.openclare_reflection_error.argtypes = [pointer]
    lib.openclare_reflection_copy_faults.restype = None
    lib.openclare_reflection_copy_faults.argtypes = [pointer] * 7
    lib.openclare_reflection_copy_frames.restype = None
    lib.openclare_reflection_copy_frames.argtypes = [pointer] * 5
    lib.openclare_reflection_copy_parts.restype = None
    lib.openclare_reflection_copy_parts.argtypes = [pointer] * 4
    lib.openclare_reflection_free.restype = None
    lib.openclare_reflection_free.argtypes = [pointer]
    lib.openclare_reflection_repair.restype = pointer
    lib.openclare_reflection_repair.argtypes = [
        pointer, size, pointer, size, pointer, size, pointer, pointer, size, pointer, size,
        double, double, double, double, double,
    ]
    lib.openclare_repair_ok.restype = ctypes.c_int32
    lib.openclare_repair_error.restype = text
    lib.openclare_repair_ok.argtypes = lib.openclare_repair_error.argtypes = [pointer]
    for count in ("vertex", "moved", "crease", "left"):
        function = getattr(lib, f"openclare_repair_{count}_count")
        function.restype, function.argtypes = size, [pointer]
    lib.openclare_repair_largest_displacement.restype = double
    lib.openclare_repair_largest_displacement.argtypes = [pointer, pointer]
    lib.openclare_repair_repair_count.restype = size
    lib.openclare_repair_repair_count.argtypes = [pointer, pointer]
    lib.openclare_repair_copy_positions.restype = None
    lib.openclare_repair_copy_positions.argtypes = [pointer, pointer]
    lib.openclare_repair_copy_score.restype = size
    lib.openclare_repair_copy_score.argtypes = [pointer, ctypes.c_uint32, pointer, pointer, pointer]
    lib.openclare_repair_copy_creases.restype = None
    lib.openclare_repair_copy_creases.argtypes = [pointer] * 8
    lib.openclare_repair_copy_left.restype = None
    lib.openclare_repair_copy_left.argtypes = [pointer] * 7
    lib.openclare_repair_free.restype = None
    lib.openclare_repair_free.argtypes = [pointer]
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
    weight per edge), `curve_edges` (the same, and the index of the curve
    each runs along), None when not asked for; `face_groups`, the group
    of the input each face lies in, None when the input came without groups;
    and `poles`, the irregular vertices of the result away from its borders:
    the point of the input each is on (three floats each), its valence, and
    the index of the pole given that it answers, -1 for a free one."""

    def __init__(self):
        self.positions = self.sizes = self.corners = None
        self.report, self.uv = {}, {}
        self.seams = self.hard_edges = self.creases = self.curve_edges = self.face_groups = None
        self.poles = None


def remesh(
    positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count, preset, settings,
    density=None, on_stage=None, uv_layers=(), seams=False, hard_edges=False, creases=False, curves=(),
    groups=None, edges=None, curve_edges=False, poles=(),
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

    `edges` is None or a numpy array of unsigned 32-bit vertices, two per
    edge: edges of the mesh the result keeps as hard edges whatever their
    angle, as if a sharp curve ran through them (those marked sharp, say).

    `poles` are poles placed by hand: for each, its point (three floats) and
    the valence, 3 or 5, the vertex of the result there is to have. The
    engine places the others freely.

    `on_stage`, if given, is called with the name of each stage as it begins
    ("cross field", or "piece 3 of 12: cross field"); the run stops if it
    returns something true, and `EngineError` says so.

    `uv_layers` names the layers of texture coordinates wanted, among
    "grid" and "atlas"; `seams` asks for the edges the atlas is cut along,
    `hard_edges` for those that run along a hard edge of the input,
    `creases` for those to keep sharp under subdivision, with their weights,
    `curve_edges` for those that run along a guide curve, with the index of
    the curve in `curves`.

    Returns a `Remeshed`.
    """
    import numpy

    lib = library()
    lines = "".join(f"{key} = {value}\n" for key, value in settings.items())
    if density is not None and (density.dtype != numpy.float32 or len(density) != vertex_count):
        raise EngineError(f"{len(density)} weights of type {density.dtype} for {vertex_count} vertices")
    if edges is not None and (edges.dtype != numpy.uint32 or len(edges) % 2):
        raise EngineError(f"{len(edges)} vertices of type {edges.dtype} for edges, two each")
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
    # The curves of fewer than two points are left out; the index of each
    # curve given to the engine is kept, to name it as the caller does.
    kept_curves = [index for index, curve in enumerate(curves) if len(curve[0]) >= 2]
    curves = [curves[index] for index in kept_curves]
    if curves:
        curve_points = numpy.concatenate([points for points, _, _ in curves])
        curve_sizes = numpy.array([len(points) for points, _, _ in curves], numpy.uint32)
        curve_flags = numpy.array(
            [CURVE_CLOSED * bool(closed) + CURVE_SHARP * bool(sharp) for _, closed, sharp in curves], numpy.uint8
        )
        curve_arguments = (curve_points.ctypes.data, curve_sizes.ctypes.data, curve_flags.ctypes.data, len(curves))
    else:
        curve_arguments = (None, None, None, 0)

    if poles:
        pole_points = numpy.array([tuple(point) for point, _ in poles], numpy.float32).ravel()
        pole_valences = numpy.array([valence for _, valence in poles], numpy.uint8)
        pole_arguments = (pole_points.ctypes.data, pole_valences.ctypes.data, len(poles))
    else:
        pole_arguments = (None, None, 0)

    # Kept in a variable: the engine calls it for as long as the run lasts.
    callback = ON_STAGE(told) if on_stage else ON_STAGE()
    result = lib.openclare_remesh(
        positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count,
        None if density is None else density.ctypes.data,
        None if groups is None else groups.ctypes.data,
        None if edges is None else edges.ctypes.data, 0 if edges is None else len(edges) // 2,
        *curve_arguments,
        *pole_arguments,
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
        count = lib.openclare_result_pole_count(result)
        out.poles = (
            numpy.empty(3 * count, numpy.float32), numpy.empty(count, numpy.uint8), numpy.empty(count, numpy.int32)
        )
        lib.openclare_result_copy_poles(
            result, None, None, out.poles[0].ctypes.data, out.poles[1].ctypes.data, out.poles[2].ctypes.data
        )
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
        if curve_edges:
            count = lib.openclare_result_curve_edge_count(result)
            out.curve_edges = (numpy.empty(2 * count, numpy.uint32), numpy.empty(count, numpy.uint32))
            lib.openclare_result_copy_curve_edges(result, out.curve_edges[0].ctypes.data, out.curve_edges[1].ctypes.data)
            out.curve_edges = (out.curve_edges[0], numpy.array(kept_curves, numpy.uint32)[out.curve_edges[1]])
        return out
    finally:
        lib.openclare_result_free(result)


def hard_edges(
    positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count, preset, settings, density=None
):
    """The hard edges a remesh with the same arguments would follow, found
    by the engine without remeshing, by the same code and at the size of the
    quads it would make: the arguments are those of `remesh`, read in place.

    Returns two numpy arrays of unsigned 32-bit vertices, two per edge: the
    edges kept (sharper than the angle of the hard edges and still sharp at
    the size of the quads; open borders are not among them), and those
    sharper than the angle left out (the grain of a sculpt)."""
    import numpy

    lib = library()
    lines = "".join(f"{key} = {value}\n" for key, value in settings.items())
    if density is not None and (density.dtype != numpy.float32 or len(density) != vertex_count):
        raise EngineError(f"{len(density)} weights of type {density.dtype} for {vertex_count} vertices")
    found = lib.openclare_hard_edges(
        positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count,
        None if density is None else density.ctypes.data, preset.encode(), lines.encode(),
    )
    return _edges_of(lib, found)


def _edges_of(lib, found):
    """The edges kept and left out of the handle `found`, which is freed."""
    import numpy

    try:
        if not lib.openclare_hard_edges_ok(found):
            raise EngineError(lib.openclare_hard_edges_error(found).decode())
        kept = numpy.empty(2 * lib.openclare_hard_edges_kept_count(found), numpy.uint32)
        left_out = numpy.empty(2 * lib.openclare_hard_edges_left_out_count(found), numpy.uint32)
        lib.openclare_hard_edges_copy_kept(found, kept.ctypes.data)
        lib.openclare_hard_edges_copy_left_out(found, left_out.ctypes.data)
        return kept, left_out
    finally:
        lib.openclare_hard_edges_free(found)


class HardEdgeFinder:
    """One mesh whose hard edges are found again and again under other
    settings, as `hard_edges` finds them: what depends on the mesh alone is
    done once, by the engine, which keeps it. Used by one thread at a time."""

    def __init__(self, positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count):
        """The arguments are the first six of `hard_edges`, read in place
        and copied by the engine."""
        self._lib = library()
        self.vertices = vertex_count
        self._handle = self._lib.openclare_hard_edge_finder(
            positions, vertex_count, triangles, triangle_count, corner_vertices, corner_count
        )
        if not self._lib.openclare_hard_edge_finder_ok(self._handle):
            error = self._lib.openclare_hard_edge_finder_error(self._handle).decode()
            self._lib.openclare_hard_edge_finder_free(self._handle)
            self._handle = None
            raise EngineError(error)

    def __del__(self):
        if getattr(self, "_handle", None):
            self._lib.openclare_hard_edge_finder_free(self._handle)
            self._handle = None

    def find(self, preset, settings, density=None):
        """What `hard_edges` gives for the mesh with these arguments."""
        import numpy

        if density is not None and (density.dtype != numpy.float32 or len(density) != self.vertices):
            raise EngineError(f"{len(density)} weights of type {density.dtype} for {self.vertices} vertices")
        lines = "".join(f"{key} = {value}\n" for key, value in settings.items())
        found = self._lib.openclare_hard_edge_finder_find(
            self._handle, None if density is None else density.ctypes.data, preset.encode(), lines.encode()
        )
        return _edges_of(self._lib, found)


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


def project_level(positions, quads, source_positions, source_triangles, crease_edges=None, crease_weights=None,
                  settings="", cage_face=None, cage_faces=0, previous_missing=None):
    """Places a subdivision level on its source, as the Multires of the PRO
    edition does after each subdivision: each vertex onto the side of the
    source that faces its way, no farther than its reach, a vertex inside a
    creased line onto the source's hard edges, faces that fold taken back.

    The level is numpy arrays: `positions` (three 32-bit floats per vertex)
    and `quads` (four unsigned 32-bit vertices per face), with its creased
    edges if any, `crease_edges` (two vertices each) and `crease_weights`
    (a 32-bit float each). The source is `source_positions` and
    `source_triangles`. `settings` is `key = value` lines (reach, far,
    stretch, tilt, crease_angle, samples, seed, target, stall, stuck);
    `samples = 0` skips the measure.

    `cage_face`, for each face of the level the face of the cage it was
    subdivided from, of `cage_faces`, has the missing detail measured over
    each cage face too; `previous_missing` is what that gave for the level
    below, None for the first level.

    Returns the placed positions (vertices × 3, 32-bit floats), what became
    of each vertex (a byte each: 0 moved, 1 onto a hard edge, 2 stopped by
    the reach, 3 nothing found, 4 taken back, 5 held), and what the engine
    says, as a dictionary of texts (`advice`: done, more or denser). With
    `cage_face` and the level measured, the dictionary also holds
    `cage_missing`, what the level misses over each cage face (32-bit
    floats, % of the diagonal), and `cage_verdict`, what each calls for (a
    byte each: 0 below the target, 1 another level helps, 2 more quads round
    it in the cage).
    """
    import numpy

    def flat(array, kind, per):
        array = numpy.ascontiguousarray(array, dtype=kind).reshape(-1)
        if len(array) % per:
            raise EngineError(f"{len(array)} numbers, not a multiple of {per}")
        return array

    level, faces = flat(positions, numpy.float32, 3), flat(quads, numpy.uint32, 4)
    source, triangles = flat(source_positions, numpy.float32, 3), flat(source_triangles, numpy.uint32, 3)
    edges = flat(numpy.zeros(0) if crease_edges is None else crease_edges, numpy.uint32, 2)
    weights = flat(numpy.zeros(0) if crease_weights is None else crease_weights, numpy.float32, 1)
    if len(weights) != len(edges) // 2:
        raise EngineError(f"{len(edges) // 2} creased edges and {len(weights)} weights")
    vertices = len(level) // 3
    owners = None if cage_face is None else flat(cage_face, numpy.uint32, 1)
    if owners is not None and len(owners) != len(faces) // 4:
        raise EngineError(f"{len(owners)} cage faces named for {len(faces) // 4} faces")
    below = None if previous_missing is None else flat(previous_missing, numpy.float32, 1)
    if below is not None and (owners is None or len(below) != cage_faces):
        raise EngineError(f"the level below is measured over {len(below)} cage faces, not {cage_faces}")
    lib = library()
    placed = lib.openclare_project_level(
        level.ctypes.data, vertices, faces.ctypes.data, len(faces) // 4,
        edges.ctypes.data if len(edges) else None, weights.ctypes.data if len(weights) else None, len(weights),
        None if owners is None else owners.ctypes.data, cage_faces if owners is not None else 0,
        None if below is None else below.ctypes.data,
        source.ctypes.data, len(source) // 3, triangles.ctypes.data, len(triangles) // 3, settings.encode(),
    )
    try:
        if not lib.openclare_level_ok(placed):
            raise EngineError(lib.openclare_level_error(placed).decode())
        report = dict(line.split(" = ", 1) for line in lib.openclare_level_report(placed).decode().splitlines())
        out = numpy.empty(3 * vertices, dtype=numpy.float32)
        status = numpy.empty(vertices, dtype=numpy.uint8)
        lib.openclare_level_copy_positions(placed, out.ctypes.data)
        lib.openclare_level_copy_status(placed, status.ctypes.data)
        if owners is not None and "advice" in report:
            report["cage_missing"] = numpy.zeros(cage_faces, dtype=numpy.float32)
            report["cage_verdict"] = numpy.zeros(cage_faces, dtype=numpy.uint8)
            lib.openclare_level_copy_missing(placed, report["cage_missing"].ctypes.data)
            lib.openclare_level_copy_verdict(placed, report["cage_verdict"].ctypes.data)
    finally:
        lib.openclare_level_free(placed)
    return out.reshape(-1, 3), status, report


class Analysis:
    """What a shape calls for, measured once by the engine: how many quads a
    largest and a smallest quad make of it is then told at once, as often as
    wanted, in the units of the arrays it was given."""

    def __init__(self, positions, triangles, preset, settings, density=None):
        """`positions`: a numpy array of three 32-bit floats per vertex;
        `triangles`: three unsigned 32-bit vertices per triangle; `settings`
        maps setting names to values, on top of the `preset`; `density` is
        None or one 32-bit float per vertex."""
        import numpy

        self._lib = library()
        self._positions = numpy.ascontiguousarray(positions, dtype=numpy.float32).reshape(-1)
        self._triangles = numpy.ascontiguousarray(triangles, dtype=numpy.uint32).reshape(-1)
        self.vertices = len(self._positions) // 3
        if density is not None and (density.dtype != numpy.float32 or len(density) != self.vertices):
            raise EngineError(f"{len(density)} weights of type {density.dtype} for {self.vertices} vertices")
        lines = "".join(f"{key} = {value}\n" for key, value in settings.items())
        self._handle = self._lib.openclare_analyse(
            self._positions.ctypes.data, self.vertices, self._triangles.ctypes.data, len(self._triangles) // 3,
            preset.encode(), lines.encode(), None if density is None else density.ctypes.data,
        )
        if not self._lib.openclare_analysis_ok(self._handle):
            error = self._lib.openclare_analysis_error(self._handle).decode()
            self._lib.openclare_analysis_free(self._handle)
            self._handle = None
            raise EngineError(error)

    def __del__(self):
        if getattr(self, "_handle", None):
            self._lib.openclare_analysis_free(self._handle)
            self._handle = None

    def proposal(self):
        """A largest and a smallest quad to start from."""
        largest, smallest = ctypes.c_double(), ctypes.c_double()
        if not self._lib.openclare_analysis_proposal(self._handle, ctypes.byref(largest), ctypes.byref(smallest)):
            raise EngineError("no proposal")
        return largest.value, smallest.value

    def count(self, largest, smallest):
        """How many quads, and the share of the surface (0 to 1) that calls
        for quads smaller than the smallest."""
        quads, below = ctypes.c_double(), ctypes.c_double()
        if not self._lib.openclare_analysis_count(
            self._handle, float(largest), float(smallest), ctypes.byref(quads), ctypes.byref(below)
        ):
            raise EngineError(f"quads from {smallest} to {largest} cannot be counted")
        return quads.value, below.value

    def at_vertices(self, largest, smallest):
        """For each vertex: the edge its quads would have (not a number where
        nothing could be measured), and whether its shape calls for smaller
        ones than the smallest. Two numpy arrays."""
        import numpy

        sizes = numpy.empty(self.vertices, numpy.float32)
        coarser = numpy.empty(self.vertices, numpy.uint8)
        if not self._lib.openclare_analysis_vertices(
            self._handle, float(largest), float(smallest), sizes.ctypes.data, coarser.ctypes.data
        ):
            raise EngineError(f"quads from {smallest} to {largest} cannot be sized")
        return sizes, coarser.astype(bool)

    def at_vertices_for(self, quads):
        """For each vertex: the edge its quads would have with `quads` quads
        asked for, as a number and not as sizes (not a number where nothing
        could be measured). A numpy array."""
        import numpy

        sizes = numpy.empty(self.vertices, numpy.float32)
        if not self._lib.openclare_analysis_vertices_for(self._handle, float(quads), sizes.ctypes.data):
            raise EngineError(f"{quads} quads cannot be sized")
        return sizes


# The kinds of fault of the reflection check, in the engine's order
# (OPENCLARE_FAULT_* in openclare.h).
FAULT_KINDS = ("degenerate", "folded", "not_flat", "pinch", "curved_pole", "thin")


class Faults:
    """What the reflection check finds, worst first, as numpy arrays of one
    entry per fault: `kinds` (indices into `FAULT_KINDS`), `on_vertex`
    (true for a pole, false for a face), `indices` (of the face or the
    vertex), `sides` (of the face, or valence of the vertex), `counts` (how
    many of its parts the fault concerns), `values` (an angle in degrees for
    not_flat, pinch and curved_pole; the smallest triangle (centre, edge)
    over the mean one for folded and thin; 0 for degenerate); where to draw
    each: `centres` and `normals` (three floats each), `inner` and `radius`
    (for a face, the distance from its centre to its nearest edge and to its
    furthest vertex; for a vertex, the mean length of its edges); the parts
    of each, those of fault i from `offsets[i]` to `offsets[i + 1]` in
    `part_edges` (true for an edge, false for a corner) and `part_places`
    (the place of the corner, or of the edge from that corner, in its face);
    and `poles`, how many poles were looked at."""

    def parts(self, i):
        """The corners and the edges of fault `i`: two lists of places in its
        face."""
        span = slice(int(self.offsets[i]), int(self.offsets[i + 1]))
        places, edges = self.part_places[span], self.part_edges[span]
        return [int(p) for p in places[~edges]], [int(p) for p in places[edges]]


def reflection_check(positions, sizes, corners, crease_edges=None, crease_weights=None,
                     flat_tolerance=-1.0, angle_tolerance=-1.0, thin_ratio=-1.0):
    """The reflection check of a cage (see openclare.reflection in the
    engine): `positions`, three floats per vertex; `sizes`, the number of
    corners of each face; `corners`, the vertex of each corner, face after
    face; `crease_edges` and `crease_weights`, two vertices and one weight
    per creased edge, or None. A tolerance below zero takes the engine's
    default (0.2 % of the radius off the plane, 1 degree, 0.15 of the mean
    triangle). Returns `Faults`."""
    import numpy

    lib = library()
    positions = numpy.ascontiguousarray(positions, numpy.float32).reshape(-1)
    sizes = numpy.ascontiguousarray(sizes, numpy.uint32).reshape(-1)
    corners = numpy.ascontiguousarray(corners, numpy.uint32).reshape(-1)
    if crease_edges is not None and len(crease_edges):
        crease_edges = numpy.ascontiguousarray(crease_edges, numpy.uint32).reshape(-1)
        crease_weights = numpy.ascontiguousarray(crease_weights, numpy.float32).reshape(-1)
        if len(crease_edges) != 2 * len(crease_weights):
            raise EngineError(f"{len(crease_edges)} vertices for {len(crease_weights)} creased edges, two each")
        creases = (crease_edges.ctypes.data, crease_weights.ctypes.data, len(crease_weights))
    else:
        creases = (None, None, 0)
    found = lib.openclare_reflection_check(
        positions.ctypes.data, len(positions) // 3, sizes.ctypes.data, len(sizes), corners.ctypes.data, len(corners),
        *creases, float(flat_tolerance), float(angle_tolerance), float(thin_ratio),
    )
    try:
        if not lib.openclare_reflection_ok(found):
            raise EngineError(lib.openclare_reflection_error(found).decode())
        n, parts = lib.openclare_reflection_fault_count(found), lib.openclare_reflection_part_count(found)
        out = Faults()
        out.poles = lib.openclare_reflection_pole_count(found)
        out.kinds, elements = numpy.empty(n, numpy.uint8), numpy.empty(n, numpy.uint8)
        out.indices, out.sides, out.counts = (numpy.empty(n, numpy.uint32) for _ in range(3))
        out.values = numpy.empty(n, numpy.float64)
        lib.openclare_reflection_copy_faults(
            found, out.kinds.ctypes.data, elements.ctypes.data, out.indices.ctypes.data, out.sides.ctypes.data,
            out.counts.ctypes.data, out.values.ctypes.data,
        )
        out.on_vertex = elements == 1
        out.centres, out.normals = numpy.empty((n, 3), numpy.float32), numpy.empty((n, 3), numpy.float32)
        out.inner, out.radius = numpy.empty(n, numpy.float32), numpy.empty(n, numpy.float32)
        lib.openclare_reflection_copy_frames(
            found, out.centres.ctypes.data, out.normals.ctypes.data, out.inner.ctypes.data, out.radius.ctypes.data
        )
        out.offsets = numpy.empty(n + 1, numpy.uint32)
        part_kinds, out.part_places = numpy.empty(parts, numpy.uint8), numpy.empty(parts, numpy.uint32)
        lib.openclare_reflection_copy_parts(
            found, out.offsets.ctypes.data, part_kinds.ctypes.data, out.part_places.ctypes.data
        )
        out.part_edges = part_kinds == 1
        return out
    finally:
        lib.openclare_reflection_free(found)


# Why a fault is left after a reflection repair, in the engine's order
# (OPENCLARE_REASON_* in openclare.h); and its three scores
# (OPENCLARE_REPAIR_*).
REPAIR_REASONS = ("change_topology", "too_far", "would_worsen", "proposed_crease", "outside_selection",
                  "out_of_passes")
REPAIR_STAGES = ("before", "after", "with_creases")


class Repair:
    """What the reflection repair gives back, as numpy arrays: `positions`
    (one row of three floats per vertex, numbered as given; a vertex not
    moved is as given); `moved`, `largest` (the furthest displacement),
    `share` (the largest displacement over the shortest edge of its vertex),
    `repairs` and `passes`; `scores`, for each of `REPAIR_STAGES`, a tuple
    (counts, worst, median, poles), the first three of one entry per kind of
    `FAULT_KINDS` (NaN where there is none); the creases proposed:
    `crease_edges` (two vertices each), `crease_folds` (degrees), and the
    fault each is for, `crease_kinds`, `crease_on_vertex`, `crease_indices`,
    `crease_values`, `crease_cured`; the faults left after the moves:
    `left_kinds`, `left_on_vertex`, `left_indices`, `left_sides`,
    `left_values` and `left_reasons` (indices into `REPAIR_REASONS`)."""


def reflection_repair(positions, sizes, corners, crease_edges=None, crease_weights=None, within=None,
                      flat_tolerance=-1.0, angle_tolerance=-1.0, thin_ratio=-1.0, bound=-1.0, crease_fold=-1.0):
    """The reflection repair of a cage in place (see openclare.reflection.repair
    in the engine), given as to `reflection_check`, with `within` the faces
    to repair, or None for all of them; `bound` is the largest displacement
    of a vertex as a share of its shortest edge, `crease_fold` the fold in
    degrees from which an edge may get a crease. A setting below zero takes
    the engine's default (0.1, 30 degrees). Returns `Repair`."""
    import numpy

    lib = library()
    positions = numpy.ascontiguousarray(positions, numpy.float32).reshape(-1)
    sizes = numpy.ascontiguousarray(sizes, numpy.uint32).reshape(-1)
    corners = numpy.ascontiguousarray(corners, numpy.uint32).reshape(-1)
    if crease_edges is not None and len(crease_edges):
        crease_edges = numpy.ascontiguousarray(crease_edges, numpy.uint32).reshape(-1)
        crease_weights = numpy.ascontiguousarray(crease_weights, numpy.float32).reshape(-1)
        if len(crease_edges) != 2 * len(crease_weights):
            raise EngineError(f"{len(crease_edges)} vertices for {len(crease_weights)} creased edges, two each")
        creases = (crease_edges.ctypes.data, crease_weights.ctypes.data, len(crease_weights))
    else:
        creases = (None, None, 0)
    if within is not None:
        within = numpy.ascontiguousarray(within, numpy.uint32).reshape(-1)
        chosen = (within.ctypes.data if len(within) else None, len(within))
        if not len(within):
            raise EngineError("no face to repair")
    else:
        chosen = (None, 0)
    found = lib.openclare_reflection_repair(
        positions.ctypes.data, len(positions) // 3, sizes.ctypes.data, len(sizes), corners.ctypes.data, len(corners),
        *creases, *chosen, float(flat_tolerance), float(angle_tolerance), float(thin_ratio), float(bound),
        float(crease_fold),
    )
    try:
        if not lib.openclare_repair_ok(found):
            raise EngineError(lib.openclare_repair_error(found).decode())
        out = Repair()
        n = lib.openclare_repair_vertex_count(found)
        out.positions = numpy.empty((n, 3), numpy.float32)
        lib.openclare_repair_copy_positions(found, out.positions.ctypes.data)
        out.moved = lib.openclare_repair_moved_count(found)
        share, passes = ctypes.c_double(), ctypes.c_size_t()
        out.largest = lib.openclare_repair_largest_displacement(found, ctypes.byref(share))
        out.share = share.value
        out.repairs = lib.openclare_repair_repair_count(found, ctypes.byref(passes))
        out.passes = passes.value
        out.scores = {}
        for stage, name in enumerate(REPAIR_STAGES):
            counts = numpy.empty(len(FAULT_KINDS), numpy.uint32)
            worst, median = numpy.empty(len(FAULT_KINDS)), numpy.empty(len(FAULT_KINDS))
            poles = lib.openclare_repair_copy_score(
                found, stage, counts.ctypes.data, worst.ctypes.data, median.ctypes.data
            )
            out.scores[name] = (counts, worst, median, poles)
        c = lib.openclare_repair_crease_count(found)
        out.crease_edges = numpy.empty((c, 2), numpy.uint32)
        out.crease_folds, out.crease_values = numpy.empty(c), numpy.empty(c)
        out.crease_kinds, elements, cured = (numpy.empty(c, numpy.uint8) for _ in range(3))
        out.crease_indices = numpy.empty(c, numpy.uint32)
        lib.openclare_repair_copy_creases(
            found, out.crease_edges.ctypes.data, out.crease_folds.ctypes.data, out.crease_kinds.ctypes.data,
            elements.ctypes.data, out.crease_indices.ctypes.data, out.crease_values.ctypes.data, cured.ctypes.data,
        )
        out.crease_on_vertex, out.crease_cured = elements == 1, cured == 1
        n = lib.openclare_repair_left_count(found)
        out.left_kinds, elements, out.left_reasons = (numpy.empty(n, numpy.uint8) for _ in range(3))
        out.left_indices, out.left_sides = numpy.empty(n, numpy.uint32), numpy.empty(n, numpy.uint32)
        out.left_values = numpy.empty(n)
        lib.openclare_repair_copy_left(
            found, out.left_kinds.ctypes.data, elements.ctypes.data, out.left_indices.ctypes.data,
            out.left_sides.ctypes.data, out.left_values.ctypes.data, out.left_reasons.ctypes.data,
        )
        out.left_on_vertex = elements == 1
        return out
    finally:
        lib.openclare_repair_free(found)
