"""The drawing of a curve, without Blender: strokes made of points on a unit
sphere, as the tool would lay them on a ball, handed to `trace` the way the
mouse moves. Run with python3 tests/test_trace.py (make test-trace)."""

import importlib.util
import math
import pathlib
import random

HERE = pathlib.Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("trace", HERE / "openclare" / "trace.py")
trace = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(trace)

STEP, NEAR = 0.02, 0.05


def on_ball(longitude, latitude):
    """A point of the unit sphere, angles in degrees."""
    lon, lat = math.radians(longitude), math.radians(latitude)
    return (math.cos(lat) * math.cos(lon), math.cos(lat) * math.sin(lon), math.sin(lat))


def along_equator(start, end, every=2.0):
    """The mouse moving along the equator from longitude `start` to `end`,
    one point every `every` degrees (a fast stroke when it is large)."""
    count = max(1, round(abs(end - start) / every))
    return [on_ball(start + (end - start) * k / count, 0.0) for k in range(1, count + 1)]


def drawn(trace_, path):
    return [trace_.move(point) for point in path]


def spacing(points):
    return [trace.distance(a, b) for a, b in zip(points, points[1:])]


# Forward: a fast stroke (a mouse point every 10 degrees, 0.17 apart) is laid
# down in steps, as dense as a slow one.
stroke = trace.Trace(on_ball(0, 0), STEP, NEAR)
assert set(drawn(stroke, along_equator(0, 60, every=10.0))) == {"forward"}
gaps = spacing(stroke.points)
assert max(gaps) <= STEP + 1e-9 and min(gaps) > 0.5 * STEP, (min(gaps), max(gaps))
assert abs(stroke.length() - math.radians(60)) < 0.02, stroke.length()
assert all(abs(trace.distance(point, (0, 0, 0)) - 1.0) < 0.01 for point in stroke.points), "off the ball"
print("FORWARD", len(stroke.points), "points, %.3f to %.3f apart" % (min(gaps), max(gaps)))

# The mouse held still, or shaking less than a step, adds nothing.
before = len(stroke.points)
assert stroke.move(stroke.head) == "still"
assert stroke.move(on_ball(60.3, 0.2)) == "still" and len(stroke.points) == before

# Back: going back along the stroke undoes it to where the mouse stops, then
# forward again lays the stroke anew from there.
events = drawn(stroke, along_equator(60, 40, every=1.0))
assert "back" in events and "forward" not in events, events
assert abs(stroke.length() - math.radians(40)) < 0.03, math.degrees(stroke.length())
drawn(stroke, [on_ball(40, latitude) for latitude in range(1, 21)])
assert abs(stroke.head[2] - math.sin(math.radians(20))) < 0.01, "the stroke did not turn north"
print("BACK the stroke undone to 40 degrees, then on to the north:", len(stroke.points), "points")

# A wider band to go back in: a mouse twice as far from the stroke as the
# brush still takes it back, and only with that band.
wide = trace.Trace(on_ball(0, 0), STEP, NEAR, back=2.5 * NEAR)
narrow = trace.Trace(on_ball(0, 0), STEP, NEAR)
drawn(wide, along_equator(0, 60))
drawn(narrow, along_equator(0, 60))
aside = on_ball(52, math.degrees(2.0 * NEAR))
assert wide.move(aside) == "back" and narrow.move(aside) != "back"
print("BACK within a band of", 2.5, "brushes, not of one")

# Shaking along the way: noise of a quarter of a step does not undo the
# stroke, it ends as long as the path drawn.
random.seed(1)
shaky = trace.Trace(on_ball(0, 0), STEP, NEAR)
for point in along_equator(0, 90, every=0.5):
    noisy = tuple(x + random.uniform(-STEP / 4, STEP / 4) for x in point)
    shaky.move(noisy)
assert abs(shaky.length() - math.radians(90)) < 0.1, math.degrees(shaky.length())

# Crossing an older part of the stroke, far behind the head, is not going
# back: a curve may cross itself. Here a hook: east along the equator, north,
# west, then south across the first stretch.
hook = trace.Trace(on_ball(0, 0), STEP, NEAR)
path = along_equator(0, 30) + [on_ball(30, lat) for lat in range(2, 21, 2)]
path += [on_ball(lon, 20) for lon in range(28, 13, -2)] + [on_ball(15, lat) for lat in range(18, -21, -2)]
assert "back" not in drawn(hook, path)
assert hook.head[2] < -0.3 and not hook.closed, "the hook did not go on south across itself"
print("CROSSING the hook crosses its first stretch and goes on:", len(hook.points), "points")

# Closing: round the equator, the stroke would close as the mouse comes back
# to its start, but nothing closes while the button is down: it may go on
# past the start, and closes only if let go near it.
loop = trace.Trace(on_ball(0, 0), STEP, NEAR)
events = drawn(loop, along_equator(0, 358, every=3.0))
assert loop.closed and set(events) <= {"forward", "still"}, events[-5:]
count = len(loop.points)
assert abs(count - 2 * math.pi / STEP) < 10, count
assert loop.curve()[1], "let go at its start, the loop is not closed"
# Going on past the start draws on: let go there, the curve is open.
onward = trace.Trace(on_ball(0, 0), STEP, NEAR)
drawn(onward, along_equator(0, 358, every=3.0) + along_equator(358, 400, every=3.0))
assert not onward.closed and not onward.curve()[1] and len(onward.points) > count
# Back along it from the start, it is open again.
drawn(loop, along_equator(355, 340, every=1.0))
assert not loop.closed and not loop.curve()[1]
print("CLOSED round the equator in", count, "points when let go at its start; open past it, or back from it")

# A small scribble near its start does not close.
scribble = trace.Trace(on_ball(0, 0), STEP, NEAR)
drawn(scribble, [on_ball(2, 0), on_ball(3, 1), on_ball(2, 2), on_ball(0.5, 1.0), on_ball(0.2, 0.2)])
assert not scribble.closed

# Taking a curve up again by either end: the nearest end within reach; by its
# start the stroke is turned, so it goes on from there.
curves = [stroke.curve(), loop.curve(), hook.curve()]
first, last = stroke.points[0], stroke.points[-1]
assert trace.end_near(curves, first, NEAR) == (0, True)
assert trace.end_near(curves, last, NEAR) == (0, False)
assert trace.end_near(curves, on_ball(0, -60), NEAR) is None
again = trace.taken_up(*stroke.curve(), True, STEP, NEAR)
assert again.head == first and again.points[0] == last
drawn(again, along_equator(0, -20))
assert again.points[-1][1] < 0 and again.points[0] == last, "not taken up from its start"
print("TAKEN UP by its start and drawn on to -20 degrees:", len(again.points), "points")

# A closed curve has no end to take up; the brush on a curve finds it.
closed = trace.Trace(on_ball(0, 0), STEP, NEAR)
drawn(closed, along_equator(0, 358, every=3.0))
assert trace.end_near([closed.curve()], on_ball(0, 0), NEAR) is None
assert trace.touched(curves, on_ball(15, 10), NEAR) == 2
assert trace.touched(curves, on_ball(0, -60), NEAR) is None

# A polyline of uneven points laid again a step apart.
even = trace.resampled([(0, 0, 0), (0.05, 0, 0), (0.051, 0, 0), (0.2, 0, 0)], STEP)
assert len(even) == 11 and max(spacing(even)) <= STEP + 1e-9 and trace.distance(even[-1], (0.2, 0, 0)) < 1e-9, even

# Too short to be a curve.
assert trace.Trace(on_ball(0, 0), STEP, NEAR).curve() is None
# The rope: the point that draws stays while the rope is slack, and is pulled
# along it, taut, once the mouse is farther; a rope of 0 is none.
assert trace.pulled((0.0, 0.0), (20.0, 0.0), 30.0) == (0.0, 0.0)
assert trace.pulled((0.0, 0.0), (50.0, 0.0), 30.0) == (20.0, 0.0)
head = trace.pulled((0.0, 0.0), (30.0, 40.0), 30.0)
assert abs(math.hypot(30.0 - head[0], 40.0 - head[1]) - 30.0) < 1e-9 and abs(head[1] / head[0] - 40.0 / 30.0) < 1e-9
assert trace.pulled((0.0, 0.0), (5.0, 5.0), 0.0) == (5.0, 5.0)
# The shake of a hand smaller than the rope does not move it.
random.seed(3)
head = (0.0, 0.0)
for _ in range(200):
    head = trace.pulled(head, (random.uniform(-10, 10), random.uniform(-10, 10)), 30.0)
assert head == (0.0, 0.0)
print("ROPE slack: the point stays; taut: it follows the mouse at the length of the rope")

# The crease: samples across a roof (a ridge at x = 0), across a gutter (a
# valley at x = 0), across a plane; the ridge is found by RIDGE only, the
# valley by VALLEY only, the plane by neither.
roof = [((x, 0.0, -abs(x)), ((1.0 if x > 0 else -1.0 if x < 0 else 0.0) / math.sqrt(2), 0.0, 1.0 / math.sqrt(2)) if x else (0.0, 0.0, 1.0))
        for x in (k * 0.1 for k in range(-5, 6))]
gutter = [((x, 0.0, abs(x)), ((-1.0 if x > 0 else 1.0 if x < 0 else 0.0) / math.sqrt(2), 0.0, 1.0 / math.sqrt(2)) if x else (0.0, 0.0, 1.0))
          for x in (k * 0.1 for k in range(-5, 6))]
plane = [((k * 0.1, 0.0, 0.0), (0.0, 0.0, 1.0)) for k in range(-5, 6)]
least = math.radians(8.0)
assert trace.crease(roof, "RIDGE", least) == 5 and trace.crease(roof, "VALLEY", least) is None
assert trace.crease(gutter, "VALLEY", least) == 5 and trace.crease(gutter, "RIDGE", least) is None
assert trace.crease(plane, "RIDGE", least) is None and trace.crease(plane, "VALLEY", least) is None
# A ray that missed (None) is stepped over.
assert trace.crease([(None, None)] + roof[1:], "RIDGE", least) == 5
print("CREASE the ridge of a roof is a ridge, the bottom of a gutter a valley, a plane neither")

# A rounded valley, as the folds of a sculpt: no two rays turn by 8 degrees,
# the band turns by 17 in all; its bottom is found, within a step.
def valley(depth, bottom):
    samples = []
    for x in (k * 0.03 for k in range(-5, 6)):
        slope = 2.0 * depth * (x - bottom)
        length = math.hypot(slope, 1.0)
        samples.append(((x, 0.0, depth * (x - bottom) ** 2), (-slope / length, 0.0, 1.0 / length)))
    return samples

found = trace.crease(valley(0.5, 0.04), "VALLEY", least)
assert found is not None and abs(found * 0.03 - 0.15 - 0.04) <= 0.03, found
assert trace.crease(valley(0.5, 0.04), "RIDGE", least) is None
assert trace.crease(valley(0.05, 0.04), "VALLEY", least) is None, "a valley of 2 degrees in all is no crease"
print("VALLEY rounded, 17 degrees across the band: its bottom found at sample", found)
# On a sharp edge the crease found by halves is on the edge: a roof whose
# ridge is at x = 0.123, sampled from -0.5 to 0.5.
def roof_at(x):
    side = 1.0 if x > 0.123 else -1.0
    return (x, 0.0, -abs(x - 0.123)), (side / math.sqrt(2), 0.0, 1.0 / math.sqrt(2))

edge = trace.edge_between(roof_at, -0.1, 0.3, rounds=12)
assert abs(edge[0] - 0.123) < 1e-3, edge
print("EDGE found by halves at x = %.4f for 0.123" % edge[0])
print("TRACE all checks passed")
