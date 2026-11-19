"""The baseline of PRO issue #6: Blender's Multires rebuilt from the source,
one level after the other, each level placed on the source by a Shrinkwrap.

    blender -b --factory-startup --python-exit-code 1 --python bench/multires.py --
        --source <source.obj> --cage <cage.obj> --mode <project|project-limited|nearest|openclare|addon>
        --levels <N> --out <dir> --openclare <engine> [--creases <FILE>] [--write <apply|reshape>]
        [--settings TEXT] [--samples N] [--seed S]

Blender script. The cage gets a Multires modifier (Catmull-Clark, creases on);
level after level, Multires subdivides once, a Shrinkwrap on the source after
it places the new level, and the placed positions are written into the level:

- `apply` (the default, what #6 did): the Shrinkwrap applied, which Blender
  bakes into the top level of the Multires before it;
- `reshape`: the mesh evaluated with the Shrinkwrap copied into the level by
  `multires_reshape`, the route the PRO add-on is to take.

Either way the level read back is what Multires now gives, written verbatim
(no axis conversion) as level-<N>.obj into <dir>, only the top level kept;
each level is measured on the way by `openclare detail` (the missing detail,
p99 of the distance from points spread on the source by area to the level,
in % of the diagonal; the edges longer than 4 times the median).

The modes, #6's three, with what #6 did not record chosen here:

- `project`: along the vertex normals, both ways, no limit, no face culled;
- `project-limited`: the same, no farther than one edge, the median edge of
  the level being placed;
- `nearest`: the nearest point of the source.

And the engine's own (milestone 3 of the Multires specification):

- `openclare`: no Shrinkwrap; the level Multires subdivided, with the crease
  weights Multires carried onto its edges, written out and placed by
  `openclare level` (the guarded projection), `--settings` passed to it.
  The placed positions are written into the level by reshape whatever
  `--write` says; what the engine reports of each level goes in its entry
  under `placed`.

- `addon`: the same placing, with the engine called as the add-on calls it
  (`engine.project_level`, the library of `make addon` in blender/openclare/lib),
  the level's arrays read from Blender's mesh, the source triangulated from
  Blender's: what Add Level of the PRO edition does. Each level's entry gets
  `click`, the seconds of each step of that click (subdivide, read the
  level, read the source, engine, write by reshape) and their total; the
  measure is not in it.

`--creases` gives the cage's crease weights, as `openclare remesh
--creases-out` writes them; Multires subdivides with them.

Writes <dir>/detail.json: the settings, Blender's version, and one entry per
level from 0 (the cage) to N: its faces, what `openclare detail --json`
says of it, and `gap`, the largest distance between the positions placed and
those the level holds after the write, in % of the diagonal.
"""

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

import bpy
import numpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from blender_io import activate, read_obj, write_obj  # noqa: E402


def crease_edges(mesh, path):
    """Gives the edges of `mesh` the crease weights of the file at `path`."""
    pairs, weights = [], []
    for line in Path(path).read_text().splitlines():
        fields = line.split()
        if fields:
            pairs.append((int(fields[0]) - 1, int(fields[1]) - 1))
            weights.append(float(fields[2]))
    count = len(mesh.vertices)
    ends = numpy.empty(2 * len(mesh.edges), numpy.int64)
    mesh.edges.foreach_get("vertices", ends)
    ends = numpy.sort(ends.reshape(-1, 2), axis=1)
    keys = {min(a, b) * count + max(a, b): w for (a, b), w in zip(pairs, weights)}
    values = numpy.array([keys.get(a * count + b, 0.0) for a, b in ends], numpy.float32)
    attribute = mesh.attributes.get("crease_edge") or mesh.attributes.new("crease_edge", "FLOAT", "EDGE")
    attribute.data.foreach_set("value", values)
    return int((values > 0).sum()), len(pairs)


def positions(mesh):
    flat = numpy.empty(3 * len(mesh.vertices), numpy.float64)
    mesh.vertices.foreach_get("co", flat)
    return flat.reshape(-1, 3)


def median_edge(mesh):
    points = positions(mesh)
    ends = numpy.empty(2 * len(mesh.edges), numpy.int64)
    mesh.edges.foreach_get("vertices", ends)
    ends = ends.reshape(-1, 2)
    lengths = numpy.sort(numpy.linalg.norm(points[ends[:, 0]] - points[ends[:, 1]], axis=1))
    return float(lengths[len(lengths) // 2])


def level_creases(mesh, path):
    """Writes the creased edges of `mesh` as `openclare level --creases`
    reads them, and returns how many."""
    attribute = mesh.attributes.get("crease_edge")
    if attribute is None:
        Path(path).write_text("")
        return 0
    weights = numpy.empty(len(mesh.edges), numpy.float32)
    attribute.data.foreach_get("value", weights)
    ends = numpy.empty(2 * len(mesh.edges), numpy.int64)
    mesh.edges.foreach_get("vertices", ends)
    ends = ends.reshape(-1, 2)
    creased = numpy.nonzero(weights > 0)[0]
    Path(path).write_text("".join(f"{ends[e, 0] + 1} {ends[e, 1] + 1} {weights[e]:.6g}\n" for e in creased))
    return len(creased)


def placed_by_engine(options, subdivided, out, level):
    """A copy of `subdivided` with its vertices where `openclare level` puts
    them, and what the engine said."""
    given, placed_path, creases = out / f"subdivided-{level}.obj", out / f"placed-{level}.obj", out / "creases-level.txt"
    write_obj(given, subdivided)
    level_creases(subdivided, creases)
    done = subprocess.run(
        [options.openclare, "level", options.source, str(given), str(placed_path), "--creases", str(creases),
         "--settings", f"{options.settings}; samples = 0", "--json"],
        capture_output=True, text=True,
    )
    if done.returncode != 0:
        raise SystemExit(f"openclare level {given}: {done.stderr.strip()}")
    said = json.loads(done.stdout)
    points = [line.split()[1:4] for line in placed_path.read_text().splitlines() if line.startswith("v ")]
    placed = subdivided.copy()
    placed.vertices.foreach_set("co", numpy.array(points, numpy.float32).reshape(-1))
    placed.update()
    for path in (given, placed_path, creases):
        path.unlink()
    return placed, said


def level_arrays(mesh):
    """Positions, quads and creased edges of a level, as the PRO add-on reads them."""
    points = numpy.empty(3 * len(mesh.vertices), numpy.float32)
    mesh.vertices.foreach_get("co", points)
    quads = numpy.empty(len(mesh.loops), numpy.uint32)
    mesh.loops.foreach_get("vertex_index", quads)
    attribute = mesh.attributes.get("crease_edge")
    if attribute is None:
        return points.reshape(-1, 3), quads, None, None
    weights = numpy.empty(len(mesh.edges), numpy.float32)
    attribute.data.foreach_get("value", weights)
    ends = numpy.empty(2 * len(mesh.edges), numpy.uint32)
    mesh.edges.foreach_get("vertices", ends)
    creased = weights > 0
    return points.reshape(-1, 3), quads, ends.reshape(-1, 2)[creased], weights[creased]


def source_arrays(source):
    """The source as seen, in triangles, as the PRO add-on reads it (the
    cage and the source both at the origin here)."""
    seen = source.evaluated_get(bpy.context.evaluated_depsgraph_get())
    mesh = seen.to_mesh()
    mesh.calc_loop_triangles()
    points = numpy.empty(3 * len(mesh.vertices), numpy.float32)
    mesh.vertices.foreach_get("co", points)
    triangles = numpy.empty(3 * len(mesh.loop_triangles), numpy.uint32)
    mesh.loop_triangles.foreach_get("vertices", triangles)
    seen.to_mesh_clear()
    return points.reshape(-1, 3), triangles


def placed_by_addon(options, subdivided, source, clock):
    """A copy of `subdivided` placed by the engine as the add-on calls it,
    what it said, the steps timed into `clock`."""
    sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "blender"))
    from openclare import engine

    started = time.perf_counter()
    arrays = level_arrays(subdivided)
    clock["read"] = time.perf_counter() - started
    started = time.perf_counter()
    source_positions, source_triangles = source_arrays(source)
    clock["source"] = time.perf_counter() - started
    started = time.perf_counter()
    points, _, said = engine.project_level(
        arrays[0], arrays[1], source_positions, source_triangles, arrays[2], arrays[3],
        f"{options.settings}; samples = 0",
    )
    clock["engine"] = time.perf_counter() - started
    placed = subdivided.copy()
    placed.vertices.foreach_set("co", points.reshape(-1))
    placed.update()
    return placed, {key: (int(value) if value.isdigit() else float(value)) for key, value in said.items()}


def evaluated(obj):
    """A new mesh of `obj` with its modifiers applied."""
    return bpy.data.meshes.new_from_object(obj.evaluated_get(bpy.context.evaluated_depsgraph_get()))


def measured(options, path):
    done = subprocess.run(
        [options.openclare, "detail", options.source, str(path), "--samples", str(options.samples),
         "--seed", str(options.seed), "--json"],
        capture_output=True, text=True,
    )
    if done.returncode != 0:
        raise SystemExit(f"openclare detail {path}: {done.stderr.strip()}")
    return json.loads(done.stdout)


def main():
    arguments = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--source", required=True)
    parser.add_argument("--cage", required=True)
    parser.add_argument("--mode", required=True, choices=["project", "project-limited", "nearest", "openclare", "addon"])
    parser.add_argument("--levels", type=int, required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--openclare", required=True, help="the engine's command line, for the measure")
    parser.add_argument("--creases", help="the cage's crease weights, as remesh --creases-out writes them")
    parser.add_argument("--write", default="apply", choices=["apply", "reshape"])
    parser.add_argument("--settings", default="", help="settings of openclare level, for the mode openclare")
    parser.add_argument("--samples", type=int, default=60000)
    parser.add_argument("--seed", type=int, default=1)
    options = parser.parse_args(arguments)
    out = Path(options.out)
    out.mkdir(parents=True, exist_ok=True)

    bpy.ops.wm.read_factory_settings(use_empty=True)
    source = read_obj(options.source, "source")
    cage = read_obj(options.cage, "cage")
    corners = positions(source.data)
    diagonal = float(numpy.linalg.norm(corners.max(axis=0) - corners.min(axis=0)))
    creased = None
    if options.creases:
        creased = crease_edges(cage.data, options.creases)

    multires = cage.modifiers.new("Multires", "MULTIRES")
    multires.use_creases = True
    activate(cage)
    report = {
        "source": options.source, "cage": options.cage, "mode": options.mode, "write": options.write,
        "creases": creased and {"edges_creased": creased[0], "given": creased[1]}, "settings": options.settings,
        "samples": options.samples, "seed": options.seed, "blender": bpy.app.version_string, "levels": [],
    }
    for level in range(options.levels + 1):
        gap = 0.0
        if level > 0:
            clock = {}
            started = time.perf_counter()
            bpy.ops.object.multires_subdivide(modifier="Multires", mode="CATMULL_CLARK")
            subdivided = evaluated(cage)
            clock["subdivide"] = time.perf_counter() - started
            said = None
            if options.mode == "openclare":
                placed, said = placed_by_engine(options, subdivided, out, level)
                bpy.data.meshes.remove(subdivided)
            elif options.mode == "addon":
                placed, said = placed_by_addon(options, subdivided, source, clock)
                bpy.data.meshes.remove(subdivided)
            else:
                wrap = cage.modifiers.new("Shrinkwrap", "SHRINKWRAP")
                wrap.target = source
                if options.mode == "nearest":
                    wrap.wrap_method = "NEAREST_SURFACEPOINT"
                else:
                    wrap.wrap_method = "PROJECT"
                    wrap.use_negative_direction = True
                    wrap.use_positive_direction = True
                    wrap.cull_face = "OFF"
                    if options.mode == "project-limited":
                        wrap.project_limit = median_edge(subdivided)
                bpy.data.meshes.remove(subdivided)
                placed = evaluated(cage)
            started = time.perf_counter()
            if options.write == "apply" and options.mode not in ("openclare", "addon"):
                bpy.ops.object.modifier_apply(modifier=wrap.name)
            else:
                if options.mode not in ("openclare", "addon"):
                    cage.modifiers.remove(wrap)
                temporary = bpy.data.objects.new("placed", placed)
                bpy.context.collection.objects.link(temporary)
                bpy.ops.object.select_all(action="DESELECT")
                temporary.select_set(True)
                activate(cage)
                temporary.select_set(True)
                bpy.ops.object.multires_reshape(modifier="Multires")
                bpy.data.objects.remove(temporary)
            clock["write"] = time.perf_counter() - started
            now = evaluated(cage)
            if len(now.vertices) != len(placed.vertices):
                raise SystemExit(f"level {level}: {len(now.vertices)} vertices read back, {len(placed.vertices)} placed")
            gap = float(numpy.linalg.norm(positions(now) - positions(placed), axis=1).max()) / diagonal
            bpy.data.meshes.remove(placed)
        else:
            now = evaluated(cage)
        path = out / f"level-{level}.obj"
        write_obj(path, now)
        entry = {"level": level, "faces": len(now.polygons), "vertices": len(now.vertices), "gap": 100.0 * gap}
        if level > 0 and said is not None:
            entry["placed"] = said
        if level > 0 and options.mode == "addon":
            entry["click"] = {**{k: round(v, 3) for k, v in clock.items()}, "total": round(sum(clock.values()), 3)}
        entry.update(measured(options, path))
        report["levels"].append(entry)
        print(
            f"LEVEL {level} faces {entry['faces']} missing p99 {entry['missing_p99']:.4f} % "
            f"stretched {entry['stretched']} gap {entry['gap']:.2e} %",
            flush=True,
        )
        bpy.data.meshes.remove(now)
        if level < options.levels:
            path.unlink()
    (out / "detail.json").write_text(json.dumps(report, indent=1) + "\n")


main()
