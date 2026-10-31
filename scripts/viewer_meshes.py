#!/usr/bin/env python3
"""The meshes of the website's 3D viewer, from the bench's outputs.

    python3 scripts/viewer_meshes.py [--models ogre ...] [--budgets 3000 ...] [--out bench/out]

Reads bench/out/<model>/<budget>/<remesher>_<seed>.obj, made by the bench on this
machine, and writes docs/models/mesh/<model>/<remesher>-<budget>.ocqm with an
index.json beside them, which site/build.py turns into a viewer on the model's
page. The poles and edge loops the viewer draws are those the bench counts:
`openclare layout` gives them (target/release/openclare, built by cargo build
--release), and a mesh whose numbers of poles and loops of each kind are not
those of its run is refused.

- Only openclare, QuadriFlow, AutoRemesher, Instant Meshes (Jakob's program)
  and QuadWild-BiMDF are read, and the meshes modelled by hand of
  bench/reference/handmade: never Quad Remesher, whose outputs stay local (its
  licence).
- Never the source: it is too heavy to load, and the page keeps its pictures.
- A model whose licence does not allow passing it on (CREDITS.md: BlenderKit
  Royalty Free) is left out: a mesh one can download passes the model on, even
  remeshed.
- The mesh shown is the one of the bench's last run, bench/results/results*.json:
  for a remesher run with several seeds (QuadriFlow), the seed whose number of
  faces is the median the summary gives. A file whose number of faces is not
  that of its run is refused: it would come from another run.
- The date of a run is that of the last commit of its results file; a mesh whose
  geometry has not changed keeps the date index.json gave it, as a partial run of
  other remeshers recommits that file.

The .ocqm format, little-endian, made for quads (they are never triangulated):

    char[4]  "OCQM"
    uint32   version (2)
    uint32   flags (bit 0: indices on 16 bits, else 32)
    uint32   vertices, faces, corners (the sum of the faces' sizes), edges
    float32  minimum x, y, z; maximum x, y, z (the box of the positions)
    uint16   x, y, z of each vertex, quantized over the box
    uint8    the size of each face (4 for a quad), then zeros to a multiple of 4
    uint16 or uint32   the vertices of each face, in order
    uint8    of each vertex: its valence if it is a pole (an irregular vertex
             off the borders), else 0
    uint8    of each edge, the edges in the order they first come going round
             the faces in order: 0 on no loop (a border), 1 on a loop that
             closes, 2 a spiral, 3 a loop that stops at a pole, 4 one that runs
             from a border to a border
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

ENGINE = ROOT / "target" / "release" / "openclare"
KINDS = ("closed", "spiral", "open", "border")  # the codes 1 to 4 of an edge
REMESHERS = ("openclare", "quadriflow", "autoremesher", "instant-meshes-jakob", "quadwild-bimdf")
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


def layout(obj, faces):
    """The valence of each pole, the code of each edge in the order of the
    format, and how many loops of each kind, from `openclare layout`; None where
    the loops are not measured (a mesh that is not a manifold)."""
    out = subprocess.run([str(ENGINE), "layout", str(obj)], capture_output=True, text=True, check=True).stdout
    found = json.loads(out)
    order = {}
    for face in faces:
        for i, a in enumerate(face):
            b = face[(i + 1) % len(face)]
            order.setdefault((min(a, b), max(a, b)), len(order))
    codes = bytearray(len(order))
    for code, kind in enumerate(KINDS, 1):
        pairs = found[kind] or []
        for a, b in zip(pairs[::2], pairs[1::2]):
            codes[order[(min(a, b), max(a, b))]] = code
    return dict(found["poles"]), bytes(codes), found["loops"]


def agrees(metrics, poles, loops):
    """Whether a layout counts the poles and loops of the run's metrics."""
    if len(poles) != metrics.get("irregular_vertices"):
        return False
    if loops is None:
        return metrics.get("loops_closed") is None
    return all(loops[kind] == metrics.get(f"loops_{kind}") for kind in KINDS)


def encode(positions, faces, poles, codes):
    low = [min(p[axis] for p in positions) for axis in range(3)]
    high = [max(p[axis] for p in positions) for axis in range(3)]
    span = [(high[axis] - low[axis]) or 1.0 for axis in range(3)]
    corners = sum(len(face) for face in faces)
    short = len(positions) <= 0xFFFF
    out = bytearray(b"OCQM")
    out += struct.pack("<4I", 2, 1 if short else 0, len(positions), len(faces))
    out += struct.pack("<2I", corners, len(codes))
    out += struct.pack("<6f", *low, *high)
    for p in positions:
        out += struct.pack("<3H", *(round((p[axis] - low[axis]) / span[axis] * 0xFFFF) for axis in range(3)))
    out += bytes(len(face) for face in faces)
    out += bytes(-len(out) % 4)
    kind = "H" if short else "I"
    for face in faces:
        out += struct.pack(f"<{len(face)}{kind}", *face)
    out += bytes(min(poles.get(v, 0), 255) for v in range(len(positions)))
    out += codes
    return bytes(out)


def geometry(data):
    """The positions and faces of an .ocqm of either version, to tell whether a
    mesh has changed whatever the version."""
    version, flags, vertices, faces, corners = struct.unpack_from("<5I", data, 4)
    start = 24 if version == 1 else 28
    body = start + 24 + 6 * vertices + faces
    body += -body % 4 + corners * (2 if flags & 1 else 4)
    return struct.pack("<4I", flags, vertices, faces, corners) + data[start:body]


def last_run(budget):
    """The results of the bench's last run at a budget, and its date."""
    path = ROOT / "bench/results" / RESULTS[budget]
    date = subprocess.run(["git", "log", "-1", "--format=%cs", "--", str(path)], cwd=ROOT,
                          capture_output=True, text=True, check=True).stdout.strip()
    return json.loads(path.read_text(encoding="utf-8")), date


def previous_dates(model):
    """The run date of each file of the model's index.json, if it has one."""
    path = ROOT / "docs/models/mesh" / model / "index.json"
    if not path.exists():
        return {}
    return {m["file"]: m["run"] for m in json.loads(path.read_text(encoding="utf-8"))["meshes"]}


def chosen_run(results, model, remesher):
    """The seed, number of faces and metrics of the run shown: the median one."""
    summary = results["summary"].get(model, {}).get(remesher)
    if not summary or not summary.get("produced"):
        return None
    for run in results["runs"][model][remesher]:
        if run.get("metrics") and run["metrics"]["faces"] == summary["faces"]:
            return run["seed"], summary["faces"], run["metrics"]
    return None


def write_mesh(model, name, obj, chosen, dated, date):
    """Writes the .ocqm of a run's mesh and returns its entry of index.json, or
    None, said why, when the mesh is not that of the run."""
    seed, expected, metrics = chosen
    positions, faces = read_obj(obj)
    if len(faces) != expected:
        print(f"{model} {name}: {len(faces)} faces, the run of {date} says {expected}: refused")
        return None
    poles, codes, loops = layout(obj, faces)
    if not agrees(metrics, poles, loops):
        print(f"{model} {name}: its poles or loops are not those the run of {date} counted: refused")
        return None
    target = ROOT / "docs/models/mesh" / model / name
    target.parent.mkdir(parents=True, exist_ok=True)
    data = encode(positions, faces, poles, codes)
    same = target.exists() and geometry(target.read_bytes()) == geometry(data)
    target.write_bytes(data)
    valences = {}
    for valence in poles.values():
        valences[str(valence)] = valences.get(str(valence), 0) + 1
    print(f"{model} {name}: {len(faces)} faces, {len(poles)} poles, {target.stat().st_size / 1000:.0f} kB")
    return {"file": name, "faces": len(faces), "quads": metrics["quads"], "vertices": len(positions),
            "poles": dict(sorted(valences.items(), key=lambda item: int(item[0]))),
            "loops": loops and {kind: loops[kind] for kind in ("closed", "spiral")},
            "seed": seed, "run": dated.get(name, date) if same else date}


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
        dated = previous_dates(model)
        for budget in args.budgets:
            results, date = runs[budget]
            for remesher in REMESHERS:
                chosen = chosen_run(results, model, remesher)
                if chosen is None:
                    continue
                obj = args.out / model / budget / f"{remesher}_{chosen[0]}.obj"
                if not obj.exists():
                    print(f"{model} {budget} {remesher}: {obj.name} missing")
                    failed = True
                    continue
                entry = write_mesh(model, f"{remesher}-{budget}.ocqm", obj, chosen, dated, date)
                failed |= entry is None
                if entry:
                    entries.append({"remesher": remesher, "budget": int(budget), **entry})
        handmade = ROOT / "bench/reference/handmade" / f"{model}.obj"
        if handmade.exists():
            # The same mesh at every budget: shown once, checked against the run at 3,000.
            results, date = runs.get("3000") or last_run("3000")
            chosen = chosen_run(results, model, "handmade")
            entry = chosen and write_mesh(model, "handmade.ocqm", handmade, chosen, dated, date)
            if chosen is None:
                print(f"{model} handmade: no run of {date}: refused")
            failed |= not entry
            if entry:
                entries.append({"remesher": "handmade", "budget": None, **entry, "seed": None})
        if entries:
            azimuth, elevation = VIEWS.get(model, (-60, 20))
            view = {"up": "y" if model in Y_UP else "z", "azimuth": azimuth, "elevation": elevation}
            index = {"model": model, "view": view, "meshes": entries}
            (ROOT / "docs/models/mesh" / model / "index.json").write_text(json.dumps(index, indent=1) + "\n", encoding="utf-8")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
