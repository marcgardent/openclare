#!/usr/bin/env python3
"""Turns models from Keenan Crane's 3D Model Repository into corpus sources.

    python3 bench/import_crane.py

Expects the archives unpacked under bench/corpus-large/crane/<archive name>/
(https://www.cs.cmu.edu/~kmcrane/Projects/ModelRepository/). The repository
states: "you can do whatever you like with the data without asking for
permission"; the Pittsburgh Bridge scan is released into the public domain.

The characters come as coarse subdivision cages, which is not what a remesher
is fed in practice: they are subdivided into dense smooth surfaces first, then
brought to a common size like any other dense input.
"""

import subprocess
import sys
from pathlib import Path

BENCH = Path(__file__).resolve().parent
RETOPO = BENCH.parent / "target" / "release" / "retopo"
CRANE = BENCH / "corpus-large" / "crane"
CORPUS = BENCH / "corpus"

# name, file in the archive, subdivision levels, preprocessing (kind, triangles)
MODELS = [
    ("spot", "spot/spot/spot_quadrangulated.obj", 2, ("isotropic", 100_000)),
    ("bob", "bob/bob/bob_quad.obj", 2, ("isotropic", 100_000)),
    ("blub", "blub/blub/blub_quadrangulated.obj", 2, ("isotropic", 100_000)),
    ("pig", "pig/pig/pig_body.obj", 2, ("isotropic", 100_000)),
    # A head with its back removed: an open organic surface, kept as it is.
    ("jerry", "ogre/bs_rest.obj", 0, ("passthrough", 0)),
    # 96 intersecting closed shells.
    ("rose", "desertrose/tri_326064.obj", 0, ("isotropic", 150_000)),
    # A raw scan with open borders and slivers: its triangulation is the test.
    ("bridge", "pittsburghbridge/PittsburghBridge/PittsburghBridge.obj", 0, ("passthrough", 0)),
]


def run(command):
    done = subprocess.run([str(c) for c in command], capture_output=True, text=True)
    if done.returncode != 0:
        sys.exit(f"{' '.join(str(c) for c in command)}\n{done.stdout}\n{done.stderr}")
    return done.stdout


for name, relative, levels, (kind, triangles) in MODELS:
    source = CRANE / relative
    if not source.exists():
        sys.exit(f"missing {source}")
    if levels:
        dense = CRANE / f"{name}_dense.obj"
        run(["blender", "-b", "--python", BENCH / "subdivide.py", "--", source, dense, levels])
        source = dense
    command = [RETOPO, "preprocess", source, CORPUS / f"{name}.obj", "--kind", kind]
    if triangles:
        command += ["--triangles", triangles]
    report = run(command)
    keep = [line for line in report.splitlines() if line.split()[0] in ("triangles", "topology", "fidelity")]
    print(f"== {name}\n" + "\n".join(keep), flush=True)
