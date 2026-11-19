"""The density Refine the Cage of the PRO edition paints (milestone 6 of the
Multires specification), from a run of `multires.py --mode addon --advice`:
one weight per vertex of the source, and the quad count raised to match.

    blender -b --factory-startup --python-exit-code 1 --python bench/multires_refine.py --
        --run <dir> --source <source.obj> --quads <N> --out <dir> [--factor F] [--target T]

Blender script. Each vertex of the source takes the cage face its closest
point on the run's top level lies over (Multires keeps the 4^L faces of a
cage face together, in the order of the cage), and that face's weight:
what its part of the top level missed, m in % of the diagonal, against the
target, on the scale of the density factor, w = clamp(log(m / target) /
log(factor), 0, 1) (the missing detail of a smooth surface falls as the
square of its edges: a face missing m wants a density m / target times as
high). The quads asked rise by what that density adds, the area of the
source weighted by factor^w over its area, so that the rest of the cage
keeps its density.

Writes <out>/weights.txt (one weight per vertex of the source, as `openclare
remesh --density` reads it) and <out>/quads.txt (the quads to ask for).
"""

import argparse
import json
import math
import sys
from pathlib import Path

import bpy
import numpy
from mathutils.bvhtree import BVHTree

sys.path.insert(0, str(Path(__file__).resolve().parent))
from blender_io import read_obj  # noqa: E402


def main():
    arguments = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--run", required=True, help="the folder of a run of multires.py --mode addon --advice")
    parser.add_argument("--source", required=True)
    parser.add_argument("--quads", type=int, required=True, help="the quads the cage of the run was asked for")
    parser.add_argument("--out", required=True)
    parser.add_argument("--factor", type=float, default=4.0, help="the density factor (--density-contrast)")
    parser.add_argument("--target", type=float, default=0.05, help="the engine's target, % of the diagonal")
    options = parser.parse_args(arguments)
    run, out = Path(options.run), Path(options.out)
    out.mkdir(parents=True, exist_ok=True)

    detail = json.loads((run / "detail.json").read_text())
    cage = detail["levels"][0]["faces"]
    top = detail["levels"][-1]["level"]
    missing = numpy.maximum(numpy.array(json.loads((run / f"advice-{top}.json").read_text())["missing"]), 1e-12)
    per_face = numpy.clip(numpy.log(missing / options.target) / math.log(options.factor), 0.0, 1.0)

    bpy.ops.wm.read_factory_settings(use_empty=True)
    level = read_obj(run / f"level-{top}.obj", "level").data
    source = read_obj(options.source, "source").data
    per = len(level.polygons) // cage
    tree = BVHTree.FromPolygons([v.co for v in level.vertices], [p.vertices for p in level.polygons])
    weights = numpy.array([per_face[tree.find_nearest(v.co)[2] // per] for v in source.vertices])

    source.calc_loop_triangles()
    triangles = numpy.empty(3 * len(source.loop_triangles), numpy.int32)
    source.loop_triangles.foreach_get("vertices", triangles)
    areas = numpy.empty(len(source.loop_triangles), numpy.float64)
    source.loop_triangles.foreach_get("area", areas)
    vertex_areas = numpy.zeros(len(source.vertices))
    numpy.add.at(vertex_areas, triangles.reshape(-1, 3), (areas / 3.0)[:, None])
    raised = float((vertex_areas * options.factor**weights).sum() / vertex_areas.sum())
    quads = max(options.quads, round(options.quads * raised))

    (out / "weights.txt").write_text("\n".join(f"{w:.4f}" for w in weights) + "\n")
    (out / "quads.txt").write_text(f"{quads}\n")
    print(f"REFINE {numpy.mean(weights > 0):.2%} of the source weighted, quads {options.quads} -> {quads}", flush=True)


main()
