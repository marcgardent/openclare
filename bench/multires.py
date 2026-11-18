"""The baseline of PRO issue #6: Blender's Multires rebuilt from the source,
one level after the other, each level placed on the source by a Shrinkwrap.

    blender -b --factory-startup --python-exit-code 1 --python bench/multires.py --
        --source <source.obj> --cage <cage.obj> --mode <project|project-limited|nearest>
        --levels <N> --out <dir> --openclare <engine> [--creases <FILE>] [--write <apply|reshape>]
        [--samples N] [--seed S]

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
    parser.add_argument("--mode", required=True, choices=["project", "project-limited", "nearest"])
    parser.add_argument("--levels", type=int, required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--openclare", required=True, help="the engine's command line, for the measure")
    parser.add_argument("--creases", help="the cage's crease weights, as remesh --creases-out writes them")
    parser.add_argument("--write", default="apply", choices=["apply", "reshape"])
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
        "creases": creased and {"edges_creased": creased[0], "given": creased[1]},
        "samples": options.samples, "seed": options.seed, "blender": bpy.app.version_string, "levels": [],
    }
    for level in range(options.levels + 1):
        gap = 0.0
        if level > 0:
            bpy.ops.object.multires_subdivide(modifier="Multires", mode="CATMULL_CLARK")
            subdivided = evaluated(cage)
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
            if options.write == "apply":
                bpy.ops.object.modifier_apply(modifier=wrap.name)
            else:
                cage.modifiers.remove(wrap)
                temporary = bpy.data.objects.new("placed", placed)
                bpy.context.collection.objects.link(temporary)
                bpy.ops.object.select_all(action="DESELECT")
                temporary.select_set(True)
                activate(cage)
                temporary.select_set(True)
                bpy.ops.object.multires_reshape(modifier="Multires")
                bpy.data.objects.remove(temporary)
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
