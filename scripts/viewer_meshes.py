#!/usr/bin/env python3
"""The meshes of the website's 3D viewer, from the bench's outputs.

    python3 scripts/viewer_meshes.py [--models ogre ...] [--budgets 3000 ...] [--out bench/out]

Reads bench/out/<model>/<budget>/<remesher>_<seed>.obj, made by the bench on this
machine, and writes docs/models/mesh/<model>/<remesher>-<budget>.ocqm with an
index.json beside them, which site/build.py turns into a viewer on the model's
page.

- Only openclare, QuadriFlow and AutoRemesher are read: never Quad Remesher,
  whose outputs stay local (its licence).
- Never the source: it is too heavy to load, and the page keeps its pictures.
- A model whose licence does not allow passing it on (CREDITS.md: BlenderKit
  Royalty Free) is left out: a mesh one can download passes the model on, even
  remeshed.
- The mesh shown is the one of the bench's last run, bench/results/results*.json:
  for a remesher run with several seeds (QuadriFlow), the seed whose number of
  faces is the median the summary gives. A file whose number of faces is not
  that of its run is refused: it would come from another run.

The .ocqm format, little-endian, made for quads (they are never triangulated):

    char[4]  "OCQM"
    uint32   version (1)
    uint32   flags (bit 0: indices on 16 bits, else 32)
    uint32   vertices, faces, corners (the sum of the faces' sizes)
    float32  minimum x, y, z; maximum x, y, z (the box of the positions)
    uint16   x, y, z of each vertex, quantized over the box
    uint8    the size of each face (4 for a quad), then zeros to a multiple of 4
    uint16 or uint32   the vertices of each face, in order
"""

import argparse
import json
import re
import struct
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))
from pages import VIEWS, Y_UP  # noqa: E402  the view of the pictures, kept by the viewer

REMESHERS = ("openclare", "quadriflow", "autoremesher")
BUDGETS = ("1000", "3000", "8000")
RESULTS = {"1000": "results-1000.json", "3000": "results.json", "8000": "results-8000.json"}


def withheld_models():
    """The models of CREDITS.md that may not be passed on."""
    names = set()
    for line in (ROOT / "CREDITS.md").read_text(encoding="utf-8").splitlines():
        if line.startswith("| `") and "BlenderKit Royalty Free" in line:
            names.update(re.findall(r"`([a-z_0-9]+)`", line.split("|")[1]))
    return names


def read_obj(path):
    """Positions and faces (lists of 0-based vertex indices) of an OBJ."""
    positions, faces = [], []
    with open(path, encoding="utf-8") as obj:
        for line in obj:
            if line.startswith("v "):
                positions.append(tuple(float(value) for value in line.split()[1:4]))
            elif line.startswith("f "):
                face = []
                for corner in line.split()[1:]:
                    index = int(corner.split("/")[0])
                    face.append(index - 1 if index > 0 else len(positions) + index)
                faces.append(face)
    return positions, faces


def encode(positions, faces):
    low = [min(p[axis] for p in positions) for axis in range(3)]
    high = [max(p[axis] for p in positions) for axis in range(3)]
    span = [(high[axis] - low[axis]) or 1.0 for axis in range(3)]
    corners = sum(len(face) for face in faces)
    short = len(positions) <= 0xFFFF
    out = bytearray(b"OCQM")
    out += struct.pack("<4I", 1, 1 if short else 0, len(positions), len(faces))
    out += struct.pack("<I", corners)
    out += struct.pack("<6f", *low, *high)
    for p in positions:
        out += struct.pack("<3H", *(round((p[axis] - low[axis]) / span[axis] * 0xFFFF) for axis in range(3)))
    out += bytes(len(face) for face in faces)
    out += bytes(-len(out) % 4)
    kind = "H" if short else "I"
    for face in faces:
        out += struct.pack(f"<{len(face)}{kind}", *face)
    return bytes(out)


def last_run(budget):
    """The results of the bench's last run at a budget, and its date."""
    path = ROOT / "bench/results" / RESULTS[budget]
    date = subprocess.run(["git", "log", "-1", "--format=%cs", "--", str(path)], cwd=ROOT,
                          capture_output=True, text=True, check=True).stdout.strip()
    return json.loads(path.read_text(encoding="utf-8")), date


def chosen_run(results, model, remesher):
    """The seed and number of faces of the run shown: the median one."""
    summary = results["summary"].get(model, {}).get(remesher)
    if not summary or not summary.get("produced"):
        return None
    for run in results["runs"][model][remesher]:
        if run.get("metrics") and run["metrics"]["faces"] == summary["faces"]:
            return run["seed"], summary["faces"]
    return None


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--models", nargs="*", help="the models (default: every one with outputs)")
    parser.add_argument("--budgets", nargs="*", default=list(BUDGETS))
    parser.add_argument("--out", type=Path, default=ROOT / "bench/out", help="the bench's outputs (default: bench/out)")
    args = parser.parse_args()
    runs = {budget: last_run(budget) for budget in args.budgets}
    withheld = withheld_models()
    models = args.models or sorted(p.name for p in args.out.iterdir() if p.is_dir())
    failed = False
    for model in models:
        if model in withheld:
            print(f"{model}: left out, its licence does not allow passing it on")
            continue
        entries = []
        for budget in args.budgets:
            results, date = runs[budget]
            for remesher in REMESHERS:
                chosen = chosen_run(results, model, remesher)
                if chosen is None:
                    continue
                seed, expected = chosen
                obj = args.out / model / budget / f"{remesher}_{seed}.obj"
                if not obj.exists():
                    print(f"{model} {budget} {remesher}: {obj.name} missing")
                    failed = True
                    continue
                positions, faces = read_obj(obj)
                if len(faces) != expected:
                    print(f"{model} {budget} {remesher}: {len(faces)} faces, the run of {date} says {expected}: refused")
                    failed = True
                    continue
                name = f"{remesher}-{budget}.ocqm"
                target = ROOT / "docs/models/mesh" / model / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(encode(positions, faces))
                entries.append({"remesher": remesher, "budget": int(budget), "file": name,
                                "faces": len(faces), "seed": seed, "run": date})
                print(f"{model} {budget} {remesher}: seed {seed}, {len(faces)} faces, {target.stat().st_size / 1000:.0f} kB")
        if entries:
            azimuth, elevation = VIEWS.get(model, (-60, 20))
            view = {"up": "y" if model in Y_UP else "z", "azimuth": azimuth, "elevation": elevation}
            index = {"model": model, "view": view, "meshes": entries}
            (ROOT / "docs/models/mesh" / model / "index.json").write_text(json.dumps(index, indent=1) + "\n", encoding="utf-8")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
