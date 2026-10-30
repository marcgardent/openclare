#!/usr/bin/env python3
"""The figure of the home page's fingers: the right hand of the male body.

    python3 scripts/fingers_figure.py [--blender blender]

Writes docs/images/fingers.webp, three panels side by side: the source,
openclare asked at 1,000 quads and Instant Meshes asked at 3,000, seed 1 of the
bench. It reads the bench's own results, bench/out/human_male/source.obj,
1000/openclare_1.obj and 3000/instant-meshes_1.obj, which `bench/run.py` makes.

Each mesh is cut to the same box round the right forearm and hand, and the box's
two corners are kept as specks too small to see, so that scripts/render.py, which
frames what it sees, frames the three the same: a hand that is missing shows as
missing, not enlarged. The panels are rendered from the back of the hand.
"""

import argparse
import subprocess
import tempfile
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
BENCH = ROOT / "bench/out/human_male"
TARGET = ROOT / "docs/images/fingers.webp"
PANELS = (("source.obj", False), ("1000/openclare_1.obj", True), ("3000/instant-meshes_1.obj", True))
# Round the right forearm and hand, in the bench's units, z up.
LOW, HIGH = (0.24, -0.20, 0.70), (0.46, 0.06, 1.02)
SIZE, PAD = 700, 20


def cut(source, target):
    """The faces of `source` whose centre is in the box, and the box's corners."""
    vertices, faces = [], []
    for line in source.read_text().splitlines():
        if line.startswith("v "):
            vertices.append(tuple(map(float, line.split()[1:4])))
        elif line.startswith("f "):
            faces.append([int(word.split("/")[0]) for word in line.split()[1:]])
    inside = lambda face: all(LOW[k] <= sum(vertices[i - 1][k] for i in face) / len(face) <= HIGH[k] for k in range(3))
    kept = [face for face in faces if inside(face)]
    used = sorted({i for face in kept for i in face})
    index = {old: new + 1 for new, old in enumerate(used)}
    lines = ["v %f %f %f" % vertices[i - 1] for i in used]
    for corner in (LOW, HIGH):
        lines += ["v %f %f %f" % (corner[0] + d, corner[1], corner[2] + d / 2) for d in (0, 1e-6, 2e-6)]
    lines += ["f " + " ".join(str(index[i]) for i in face) for face in kept]
    n = len(used)
    lines += [f"f {n + 1} {n + 2} {n + 3}", f"f {n + 4} {n + 5} {n + 6}"]
    target.write_text("\n".join(lines) + "\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--blender", default="blender")
    arguments = parser.parse_args()
    with tempfile.TemporaryDirectory() as work:
        images = []
        for number, (name, edges) in enumerate(PANELS):
            mesh, picture = Path(work) / f"{number}.obj", Path(work) / f"{number}.png"
            cut(BENCH / name, mesh)
            command = [arguments.blender, "-b", "--python", str(ROOT / "scripts/render.py"), "--",
                       str(mesh), str(picture), "--azimuth", "0", "--elevation", "10", "--size", str(SIZE)]
            subprocess.run(command + ([] if edges else ["--no-edges"]), check=True, capture_output=True)
            images.append(Image.open(picture).convert("RGBA"))
    # One crop for the three, round what any of them shows.
    boxes = [image.getbbox() for image in images]
    box = (max(0, min(b[0] for b in boxes) - PAD), max(0, min(b[1] for b in boxes) - PAD),
           min(SIZE, max(b[2] for b in boxes) + PAD), min(SIZE, max(b[3] for b in boxes) + PAD))
    images = [image.crop(box) for image in images]
    width = images[0].width
    figure = Image.new("RGBA", (width * len(images), images[0].height), (0, 0, 0, 0))
    for number, image in enumerate(images):
        figure.alpha_composite(image, (number * width, 0))
    figure.save(TARGET, quality=88)
    print(f"written: {TARGET.relative_to(ROOT)}, {figure.width} x {figure.height}")


if __name__ == "__main__":
    main()
