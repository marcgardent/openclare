"""Blender script: remeshes the object of a scene of the bench through the add-on.

    blender -b --factory-startup <scene.blend> --python-exit-code 1 --python bench/blender_run.py --
        --out <dir> [--quads N] [--name NAME] [--no-source] [--no-remesh] [--memory KB]

The scene says what is remeshed and how (see bench/make_scene.py): the object
chosen in the panel, or failing that the active one, with the settings saved
in the scene. `--quads` overrides the number of quads, and nothing else. What
runs is the button of the panel, `bpy.ops.object.openclare_remesh()`, of the
add-on of this repository (blender/openclare, with the engine `make addon`
puts in its lib/ folder), not of a copy installed in Blender.

Written into <dir>, NAME being the name of the object unless given:

    NAME.obj        the result, its faces as they are, in the space of the object
    NAME.json       what the run says: the settings, the time, the engine's report
    source.obj      the object as the add-on saw it: modifiers applied, in
                    triangles, in the space of the object. What the result is
                    measured against, and what the other tools are given
    source.groups   the material of each of those triangles, one whole number
                    per line (`openclare remesh --groups`), when the add-on
                    passes materials on; removed otherwise

`--no-source` leaves the last two as they are, `--no-remesh` writes them only.

The engine runs inside Blender, and a run that goes wrong can take all the
memory of the machine. So this process caps itself: beyond `--memory`
kilobytes of address space (12 GB unless given, as `ulimit -v`; 0 for no cap)
it is refused memory and dies, which the bench reports as out of memory.

Prints one line for the bench, `OPENCLARE ` followed by a JSON object: what
goes into NAME.json, with "status": "ok"; or "status": "error" and a
"message", and Blender then exits with an error.
"""

import argparse
import json
import resource
import sys
import time
from pathlib import Path

import bpy

ROOT = Path(__file__).resolve().parent.parent
# The settings of the add-on a run depends on, written with its report.
SETTINGS = ["preset", "quads", "preserve_volumes", "adaptive_size", "density_group", "density_factor"]
EXPORTS = ["grid_uv", "atlas_uv", "sharp_edges", "materials", "creases"]
# Lines written at a time: a sculpt has millions.
CHUNK = 200_000
# The most memory this process may ask for, in kilobytes, unless told otherwise.
MEMORY_CAP = 12_000_000


class Failure(Exception):
    """The run cannot be done; said on the line the bench reads."""


def the_addon():
    """The add-on of this repository, registered. A copy installed in this
    Blender and enabled is disabled first, for this session only: it has the
    same names, and may be older."""
    import addon_utils

    for name in list(bpy.context.preferences.addons.keys()):
        if name.split(".")[-1] == "openclare":
            addon_utils.disable(name)
    sys.path.insert(0, str(ROOT / "blender"))
    import openclare

    openclare.register()
    return openclare


def number(text):
    """A value of the engine's report as a number, if it is one."""
    for kind in (int, float):
        try:
            return kind(text)
        except ValueError:
            pass
    return text


def write_lines(out, pattern, rows):
    for start in range(0, len(rows), CHUNK):
        out.write("".join(pattern % tuple(row) for row in rows[start : start + CHUNK].tolist()))


def write_obj(path, positions, faces):
    """`positions` as an array of three floats per vertex; `faces` as an
    array of three vertices per triangle, or as a list of polygons. Written
    beside `path` first: a run cut short leaves no half file under its name."""
    partial = path.with_name(path.name + ".part")
    with open(partial, "w") as out:
        # Blender stores single precision. Each number is written in full, as
        # the double it becomes, not in the nine digits that would give the
        # same float back: the engine works in double precision, and a tool
        # that reads the file must get the very numbers the add-on hands over.
        write_lines(out, "v %r %r %r\n", positions.reshape(-1, 3))
        if isinstance(faces, list):
            out.write("".join("f " + " ".join(str(v + 1) for v in face) + "\n" for face in faces))
        else:
            write_lines(out, "f %d %d %d\n", faces.reshape(-1, 3) + 1)
    partial.replace(path)


def positions_of(mesh):
    import numpy

    positions = numpy.empty(3 * len(mesh.vertices), numpy.float32)
    mesh.vertices.foreach_get("co", positions)
    return positions


def write_source(original, settings, directory):
    """Writes the object as the add-on takes it, and the material of each of
    its triangles if the add-on passes them on. Returns what was written."""
    import numpy

    evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
    mesh = evaluated.to_mesh()
    try:
        mesh.calc_loop_triangles()
        triangles = numpy.empty(3 * len(mesh.loop_triangles), numpy.int32)
        mesh.loop_triangles.foreach_get("vertices", triangles)
        write_obj(directory / "source.obj", positions_of(mesh), triangles)
        written = {"source": str(directory / "source.obj"), "source_vertices": len(mesh.vertices),
                   "source_triangles": len(mesh.loop_triangles), "source_materials": 0}
        # As the add-on does: an object all of one material has no groups.
        groups = numpy.zeros(len(mesh.loop_triangles), numpy.int32)
        if settings.export_materials and len(mesh.materials) > 1 and len(groups):
            mesh.loop_triangles.foreach_get("material_index", groups)
        sidecar = directory / "source.groups"
        if len(groups) and groups.min() != groups.max():
            names = [material.name if material else "none" for material in mesh.materials]
            with open(sidecar, "w") as out:
                out.write("# the material of each triangle; in order: " + ", ".join(names) + "\n")
                out.write("".join("%d\n" % group for group in groups.tolist()))
            written["source_materials"] = len(numpy.unique(groups))
        else:
            sidecar.unlink(missing_ok=True)
        return written
    finally:
        evaluated.to_mesh_clear()


def main(options):
    openclare = the_addon()
    scene = bpy.context.scene
    settings = scene.openclare
    if bpy.context.mode != "OBJECT":
        bpy.ops.object.mode_set(mode="OBJECT")
    original = openclare.object_to_remesh(bpy.context)
    if original is None:
        meshes = sorted(o.name for o in scene.objects if o.type == "MESH")
        raise Failure(
            f"{bpy.data.filepath or 'the scene'} has no object to remesh: none is chosen in the openclare panel "
            f"and no mesh is active (meshes of the scene: {', '.join(meshes) or 'none'})"
        )
    if options.quads is not None:
        settings.quads = options.quads
    directory = Path(options.out).resolve()
    directory.mkdir(parents=True, exist_ok=True)
    name = options.name or original.name

    said = {"status": "ok", "scene": bpy.data.filepath, "object": original.name}
    said["settings"] = {key: getattr(settings, key) for key in SETTINGS}
    said["settings"]["export"] = [key for key in EXPORTS if getattr(settings, f"export_{key}")]
    for kind in ("guide", "sharp"):
        said["settings"][f"{kind}_curves"] = len(openclare.curve_objects(getattr(settings, f"{kind}_curves")))
    if not options.no_source:
        said.update(write_source(original, settings, directory))
    if options.no_remesh:
        return said

    start = time.perf_counter()
    try:
        done = bpy.ops.object.openclare_remesh()
    except RuntimeError as error:
        # What the operator reports as an error, or a button greyed out.
        raise Failure(" ".join(str(error).split()).removeprefix("Error: ")) from error
    said["operator_seconds"] = round(time.perf_counter() - start, 3)
    remeshed = bpy.context.view_layer.objects.active
    if "FINISHED" not in done or remeshed is None or remeshed == original:
        raise Failure(f"the operator made nothing of '{original.name}'")

    mesh = remeshed.data
    write_obj(directory / f"{name}.obj", positions_of(mesh), [list(polygon.vertices) for polygon in mesh.polygons])
    report = {key: number(value) for key, value in openclare.last_report.items()}
    said.update(result=str(directory / f"{name}.obj"), vertices=len(mesh.vertices), faces=len(mesh.polygons),
                seconds=report.get("seconds", said["operator_seconds"]), report=report)
    (directory / f"{name}.json").write_text(json.dumps(said, indent=1) + "\n")
    return said


if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="blender -b <scene.blend> --python bench/blender_run.py --",
                                     description=__doc__.splitlines()[0])
    parser.add_argument("--out", required=True, help="directory the files are written into")
    parser.add_argument("--quads", type=int, help="number of quads asked for, instead of the scene's own")
    parser.add_argument("--name", help="name of the files of the result (default: the name of the object)")
    parser.add_argument("--no-source", action="store_true", help="do not write the object as the add-on saw it")
    parser.add_argument("--no-remesh", action="store_true", help="write that object only, remesh nothing")
    parser.add_argument("--memory", type=int, default=MEMORY_CAP, metavar="KB",
                        help="cap on the memory of this process, in kilobytes (0 for none)")
    options = parser.parse_args(sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else [])
    if options.memory:
        # Below a cap this process is already under, never above.
        _, hard = resource.getrlimit(resource.RLIMIT_AS)
        wanted = options.memory * 1024
        resource.setrlimit(resource.RLIMIT_AS, (wanted if hard == resource.RLIM_INFINITY else min(wanted, hard), hard))
        # And no image of its memory kept by the system if it dies of it.
        resource.setrlimit(resource.RLIMIT_CORE, (0, resource.getrlimit(resource.RLIMIT_CORE)[1]))
    try:
        print("OPENCLARE " + json.dumps(main(options)), flush=True)
    except Failure as failure:
        print("OPENCLARE " + json.dumps({"status": "error", "message": str(failure)}), flush=True)
        print(f"error: {failure}", file=sys.stderr)
        sys.exit(1)
