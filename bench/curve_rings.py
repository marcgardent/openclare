"""A guide curve round a cube, kept or let go (PR #350): the cube of side 2,
40 x 40 quads a face, in triangles, and one closed ring round it, remeshed
at 2000 quads by the engine given; whether the ring came out as a loop of
edges, from what the engine prints.

    python3 bench/curve_rings.py --openclare <engine> --ring <name> --out DIR

A ring is `z<height>`, level at that height (on the rows of edges of the
cube when the height is a multiple of 0.05), or `tilted`, its height going
with x. Writes rings.json into DIR: kept (1 or 0), guide_edges, let_go.
"""

import argparse
import json
import math
import re
import subprocess
import sys
from pathlib import Path

SEGMENTS = 40
QUADS = 2000


def cube(path):
    """The cube of side 2, each face a grid of SEGMENTS x SEGMENTS quads cut
    into two triangles each, as an OBJ file."""
    index, vertices, triangles = {}, [], []

    def vertex(p):
        key = tuple(round(c, 9) for c in p)
        if key not in index:
            index[key] = len(vertices)
            vertices.append(key)
        return index[key]

    n = SEGMENTS
    for axis in range(3):
        for sign in (-1, 1):
            a, b = (axis + 1) % 3, (axis + 2) % 3

            def at(i, j):
                p = [0.0, 0.0, 0.0]
                p[axis], p[a], p[b] = sign, -1 + 2 * i / n, -1 + 2 * j / n
                return vertex(p)

            for i in range(n):
                for j in range(n):
                    quad = [at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)]
                    if sign < 0:
                        quad.reverse()
                    triangles += [(quad[0], quad[1], quad[2]), (quad[0], quad[2], quad[3])]
    lines = [f"v {x} {y} {z}" for x, y, z in vertices] + [f"f {a + 1} {b + 1} {c + 1}" for a, b, c in triangles]
    path.write_text("\n".join(lines) + "\n")


def ring(name, points=64):
    """The points of the ring `name`, round the cube on its surface."""
    out = []
    for k in range(points):
        turn = 2 * math.pi * k / points
        x, y = math.cos(turn), math.sin(turn)
        s = max(abs(x), abs(y))
        x, y = x / s, y / s
        out.append((x, y, 0.4 * x if name == "tilted" else float(name[1:])))
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--openclare", required=True, help="the engine's command line")
    parser.add_argument("--ring", required=True, help="z<height> or tilted")
    parser.add_argument("--out", required=True)
    options = parser.parse_args()
    out = Path(options.out)
    out.mkdir(parents=True, exist_ok=True)
    cube(out / "cube.obj")
    (out / "ring.txt").write_text("".join(f"{x} {y} {z}\n" for x, y, z in ring(options.ring)) + "closed\n")
    run = subprocess.run(
        [options.openclare, "remesh", str(out / "cube.obj"), str(out / "mesh.obj"), "--quads", str(QUADS),
         "--curves", str(out / "ring.txt")],
        capture_output=True, text=True, check=False,
    )
    (out / "remesh.log").write_text(run.stdout + run.stderr)
    if run.returncode != 0:
        print(run.stdout + run.stderr, file=sys.stderr)
        return run.returncode
    edges = re.search(r"^guides\s+(\d+) edges along the guide curves", run.stdout, re.M)
    let_go = "could not be kept as lines of edges" in run.stdout
    guide_edges = int(edges.group(1)) if edges else 0
    said = {"ring": options.ring, "kept": int(guide_edges > 0 and not let_go), "guide_edges": guide_edges,
            "let_go": int(let_go)}
    (out / "rings.json").write_text(json.dumps(said) + "\n")
    print(json.dumps(said))
    return 0


if __name__ == "__main__":
    sys.exit(main())
