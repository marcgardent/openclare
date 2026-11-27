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

# With a rope the head lags the mouse: the head stops 20 degrees short of
# the start, the mouse is on it. It closes, by the mouse, and the gap from
# the head to the start is laid down in steps.
lagging = trace.Trace(on_ball(0, 0), STEP, NEAR)
drawn(lagging, along_equator(0, 340, every=3.0))
assert not lagging.closed
lagging.aim(on_ball(359, 0))
points, shut = lagging.curve()
assert lagging.closed and shut and abs(len(points) - count) < 10, (len(points), count)
gaps = [trace.distance(a, b) for a, b in zip(points, points[1:] + points[:1])]
assert max(gaps) < 1.01 * STEP, max(gaps)
lagging.aim(on_ball(300, 0))
assert not lagging.closed
print("CLOSED BY THE MOUSE the head 20 degrees behind, the gap filled in", len(points) - len(lagging.points), "points")

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
# Move: a point of the equator taken 0.2 north, a radius of 0.5 along it.
equator = [on_ball(lon, 0) for lon in range(0, 360, 2)]
assert trace.grabbed([(equator, True)], on_ball(41, 0.5), NEAR) in {(0, 20), (0, 21)}
assert trace.grabbed([(equator, True)], on_ball(41, 30), NEAR) is None
held = 0
shifted = trace.moved(equator, True, held, (0.0, 0.0, 0.2), 0.5)
lifts = [b[2] - a[2] for a, b in zip(equator, shifted)]
assert abs(lifts[0] - 0.2) < 1e-12
# The falloff: the less the farther, both ways round (the start of a closed
# curve has neighbours on both sides), nothing past the radius.
side = [lifts[k] for k in range(0, 20)]
assert all(a >= b for a, b in zip(side, side[1:])) and abs(lifts[1] - lifts[-1]) < 1e-12
far = [k for k, point in enumerate(equator) if min(trace.along_from(equator, True, held)[k], 9) >= 0.5]
assert far and all(lifts[k] == 0.0 for k in far)
# Along the curve, not across: a hairpin moved by one leg leaves the other.
pin = [(x / 10, 0.0, 0.0) for x in range(30)] + [(3.0 - x / 10, 0.1, 0.0) for x in range(30)]
bent = trace.moved(pin, False, 5, (0.0, 0.0, 1.0), 0.6)
assert bent[5][2] == 1.0 and all(point[2] == 0.0 for point in bent[30:])
print("MOVED a point of the equator 0.2 north, its neighbours within 0.5 the less the farther; a hairpin by one leg")

# Smooth: a line with a bend 0.3 high and 1 long in its middle is rubbed
# flat by a brush as wide as the bend; its ends stay, and what the brush
# does not reach stays.
line = [(x / 20, 0.3 * math.exp(-((x / 20 - 2.0) / 0.3) ** 2), 0.0) for x in range(81)]
rubbed = line
for _ in range(10):
    rubbed = trace.smoothed(rubbed, False, trace.brushed(rubbed, (2.0, 0.0, 0.0), 1.0), 1.0)
height = max(y for _, y, _ in rubbed)
assert height < 0.1, height
assert rubbed[0] == line[0] and rubbed[-1] == line[-1] and rubbed[:15] == line[:15]
# A shake of 0.05 from one point to the next goes in one stroke of the brush.
shaky = [(x / 20, 0.05 * (-1) ** x, 0.0) for x in range(81)]
calm = trace.smoothed(shaky, False, [1.0] * 81, 0.3)
assert max(abs(y) for _, y, _ in calm[5:-5]) < 0.03
# A circle of radius 1 rubbed all round with a brush of 0.3 draws in a
# little, as a mesh under Blender's Smooth brush.
circle = [(math.cos(a), math.sin(a), 0.0) for a in (2 * math.pi * k / 120 for k in range(120))]
drawn_in = trace.smoothed(circle, True, [1.0] * 120, 0.3)
radius = sum(math.hypot(x, y) for x, y, _ in drawn_in) / 120
assert 0.98 < radius < 1.0, radius
print("SMOOTH a bend 0.3 high rubbed down to %.3f in 10 strokes; a shake halved in one; a circle draws in to %.4f"
      % (height, radius))

# Laid: a loop round a cube of side 2, smoothed, cuts its corners through
# the volume; laid on the surface again, no stretch leaves it.
def on_cube(point):
    x, y, z = (min(1.0, max(-1.0, c)) for c in point)
    inside = max(abs(x), abs(y), abs(z)) < 1.0
    if not inside:
        return (x, y, z)
    axis = max(range(3), key=lambda k: abs((x, y, z)[k]))
    out = [x, y, z]
    out[axis] = 1.0 if out[axis] >= 0.0 else -1.0
    return tuple(out)


def worst(points, closed):
    pairs = zip(points, points[1:] + (points[:1] if closed else []))
    return max(trace.distance(m, on_cube(m)) for m in (tuple(0.5 * (u + v) for u, v in zip(a, b)) for a, b in pairs))


square = [on_cube((2 * math.cos(a), 2 * math.sin(a), 0.0)) for a in (2 * math.pi * k / 40 for k in range(40))]
rounded = [on_cube(point) for point in trace.smoothed(square, True, [1.0] * 40, 1.0, 1.0)]
assert worst(rounded, True) > 0.05, worst(rounded, True)
kept = trace.laid(rounded, True, on_cube, 0.001)
assert worst(kept, True) <= 0.01 and len(rounded) < len(kept) < 2 * len(rounded), (worst(kept, True), len(kept))
print("LAID a loop round a cube, smoothed, cut %.3f into it; laid again in %d points, %.4f at most"
      % (worst(rounded, True), len(kept), worst(kept, True)))

# Push: a line along x, the brush of 0.5 coming from below up to 0.1 under
# it: the line goes up ahead of the brush, round it, its points in their
# order; what the brush does not reach stays. With the centre on the
# line, it goes the way the brush goes.
flat = [(x / 20 - 2.0, 0.0, 0.0) for x in range(81)]
up = (0.0, 1.0, 0.0)
bulged = flat
for _ in range(20):
    bulged = trace.pushed(bulged, False, (0.0, -0.1, 0.0), 0.5, 0.5, up)
assert all(a[0] < b[0] for a, b in zip(bulged, bulged[1:])), "the points of the line crossed"
assert min(math.dist(point, (0.0, -0.1, 0.0)) for point in bulged) > 0.49
assert all(point == flat[k] for k, point in enumerate(bulged) if abs(flat[k][0]) >= 0.5)
assert max(y for _, y, _ in bulged) > 0.35
centred = trace.pushed(flat, False, (0.0, 0.0, 0.0), 0.5, 1.0, up)
assert all(y >= 0.0 for _, y, _ in centred) and max(y for _, y, _ in centred) > 0.4
down = trace.pushed(flat, False, (0.0, 0.0, 0.0), 0.5, 1.0, (0.0, -1.0, 0.0))
assert all(y <= 0.0 for _, y, _ in down)
print("PUSHED a line ahead of the brush to %.3f, round it, its points in order; from on it, the way the brush goes"
      % max(y for _, y, _ in bulged))

print("TRACE all checks passed")
