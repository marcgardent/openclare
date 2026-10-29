"""The fitting of a stroke by a Bezier curve, without Blender: strokes whose
best curve is known (a line, a circle, an L, an S, a stroke with the shake
of a hand), checked for how few control points they get and how close the
curve stays. Run with python3 tests/test_fit.py (make test-trace)."""

import importlib.util
import math
import pathlib
import random

HERE = pathlib.Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("fit", HERE / "openclare" / "fit.py")
fit = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(fit)

STEP = 0.02
TOLERANCE = STEP


def report(name, stroke, closed, curve):
    gap = fit.deviation(stroke, curve, closed)
    print(f"{name}: {len(stroke)} points of the stroke, {len(curve)} control points, farthest {gap:.4f}")
    assert gap <= TOLERANCE * 1.05, (name, gap)
    return len(curve)


# A straight line: its two ends.
line = [(k * STEP, 0.0, 0.0) for k in range(60)]
assert report("LINE", line, False, fit.fit(line, False, TOLERANCE)) == 2

# A circle of radius 1, closed: two cubics hold it to about 1.8 % of its
# radius, within the tolerance of 2 %, so two control points; four would
# hold it to 0.03 %.
circle = [(math.cos(a), math.sin(a), 0.0) for a in (2 * math.pi * k / 314 for k in range(314))]
count = report("CIRCLE", circle, True, fit.fit(circle, True, TOLERANCE))
assert count == 2, count

# An open arc of a quarter circle: one cubic.
arc = [(math.cos(a), math.sin(a), 0.0) for a in (0.5 * math.pi * k / 80 for k in range(81))]
assert report("ARC", arc, False, fit.fit(arc, False, TOLERANCE)) == 2

# An L: its corner is kept as a control point with handles of its own, at
# the point of the stroke nearest the corner (a step from it).
ell = [(0.0, 1.0 - k * STEP, 0.0) for k in range(50)] + [(k * STEP, 0.0, 0.0) for k in range(1, 50)]
curve = fit.fit(ell, False, TOLERANCE)
assert report("L", ell, False, curve) == 3
corner = curve[1]
assert corner[3] and fit.norm(fit.sub(corner[0], (0.0, 0.0, 0.0))) <= STEP * 1.01, corner

# An S of two half turns: a control point near its inflection, few in all.
s_curve = [(math.sin(t), t / 2, 0.0) for t in (2 * math.pi * k / 200 for k in range(201))]
count = report("S", s_curve, False, fit.fit(s_curve, False, TOLERANCE))
assert count <= 6, count

# The shake of a hand, a third of the tolerance, on a circle: the curve
# follows the circle, not the shake, and stays as sparse.
random.seed(2)
shaky = [tuple(x + random.uniform(-TOLERANCE / 3, TOLERANCE / 3) for x in point) for point in circle]
count = report("SHAKY CIRCLE", shaky, True, fit.fit(shaky, True, TOLERANCE))
assert count <= 4, count

# A square drawn closed: its four corners, and nothing else.
side = 40
square = ([(k / side, 0.0, 0.0) for k in range(side)] + [(1.0, k / side, 0.0) for k in range(side)]
          + [(1.0 - k / side, 1.0, 0.0) for k in range(side)] + [(0.0, 1.0 - k / side, 0.0) for k in range(side)])
curve = fit.fit(square, True, TOLERANCE)
assert report("SQUARE", square, True, curve) == 4 and all(point[3] for point in curve)

# A curve in space: a turn of a helix.
helix = [(math.cos(a), math.sin(a), a / 6) for a in (2 * math.pi * k / 300 for k in range(301))]
count = report("HELIX", helix, False, fit.fit(helix, False, TOLERANCE))
assert count <= 6, count
print("FIT all checks passed")
