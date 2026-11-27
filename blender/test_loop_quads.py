"""The quads along a loop, without Blender: circles and lines on a plane,
handed to `loop_quads` as the add-on would. Run with python3
blender/test_loop_quads.py (make test-trace)."""

import importlib.util
import math
import pathlib

import numpy

HERE = pathlib.Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("loop_quads", HERE / "openclare" / "loop_quads.py")
loop_quads = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(loop_quads)

UP = (0.0, 0.0, 1.0)


def circle(radius, count=360, centre=(0.0, 0.0)):
    turns = numpy.linspace(0.0, 2.0 * math.pi, count, endpoint=False)
    return numpy.column_stack([centre[0] + radius * numpy.cos(turns), centre[1] + radius * numpy.sin(turns),
                               numpy.zeros(count)])


def line(y, x0=0.0, x1=10.0, count=200):
    xs = numpy.linspace(x0, x1, count)
    return numpy.column_stack([xs, numpy.full(count, y), numpy.zeros(count)])


def quads_of(points, size, closed, others=()):
    sizes = numpy.full(len(points), size)
    placed, at, count = loop_quads.stations(points, sizes, closed)
    normals = numpy.tile(UP, (len(placed), 1))
    return loop_quads.row(placed, normals, at, closed, others), count


# A circle of radius 1, quads of 0.1: 2π / 0.1 = 62.8, so 63 edges round it,
# the stations evenly spread and on the circle.
made, count = quads_of(circle(1.0), 0.1, True)
assert count == made["edges"] == 63, count
gaps = numpy.linalg.norm(numpy.diff(numpy.vstack([made["stations"], made["stations"][:1]]), axis=0), axis=1)
assert gaps.max() - gaps.min() < 1e-3 and abs(gaps.mean() - 2 * math.pi / 63) < 1e-3, (gaps.min(), gaps.max())
# The left side of a circle run counterclockwise is inside it, h away.
assert numpy.allclose(numpy.linalg.norm(made["left"][:, :2], axis=1), 0.9, atol=1e-3)
assert numpy.allclose(numpy.linalg.norm(made["right"][:, :2], axis=1), 1.1, atol=1e-3)
assert not made["red_left"].any() and not made["red_right"].any() and made["ends"] == []
print("CIRCLE", count, "edges round a circle of radius 1 at a size of 0.1")

# A size that changes along: twice as large on half the loop, fewer edges.
points = circle(1.0)
sizes = numpy.where(points[:, 0] > 0.0, 0.2, 0.1)
_, _, varied = loop_quads.stations(points, sizes, True)
assert 44 <= varied <= 50, varied

# A circle smaller than the size: the inside row folds, red on the left.
made, count = quads_of(circle(0.08), 0.1, True)
assert count == 5 and made["red_left"].all() and not made["red_right"].any(), (count, made["red_left"])
print("TIGHT a circle of radius 0.08 at a size of 0.1: red inside")

# An open line: its edges, its two ends.
made, count = quads_of(line(0.0), 0.5, False)
assert count == made["edges"] == 20 and len(made["ends"]) == 2
assert numpy.allclose(made["ends"][0], (0.0, 0.0, 0.0)) and numpy.allclose(made["ends"][1], (10.0, 0.0, 0.0))

# Two lines 0.3 apart, quads of 0.5: crowded on the side facing the other,
# not on the other side; 0.8 apart, not crowded.
other = line(0.3)
for gap, red in ((0.3, True), (0.8, False)):
    other = line(gap)
    made, _ = quads_of(line(0.0), 0.5, False, [(other, numpy.gradient(other, axis=0))])
    assert made["red_left"].any() == red and not made["red_right"].any(), (gap, made["red_left"])
# A line crossing at a right angle meets it at a vertex: not red.
across = numpy.column_stack([numpy.full(200, 5.0), numpy.linspace(-3.0, 3.0, 200), numpy.zeros(200)])
made, _ = quads_of(line(0.0), 0.5, False, [(across, numpy.gradient(across, axis=0))])
assert not made["red_left"].any() and not made["red_right"].any()
print("CROWDED two lines 0.3 apart at a size of 0.5 red between them; 0.8 apart or crossing, not")

# A loop that comes back alongside itself, a hairpin 0.2 wide: crowded
# where its two legs run side by side.
hairpin = numpy.vstack([line(0.0, 0.0, 5.0, 100), line(0.2, 5.0, 0.0, 100)])
made, _ = quads_of(hairpin, 0.5, False)
assert made["red_left"].any() or made["red_right"].any()
print("HAIRPIN a loop running back along itself 0.2 away is red")
print("loop quads: all good")
