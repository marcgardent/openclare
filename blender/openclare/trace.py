"""A curve drawn on the model with the mouse, as the drawing tool keeps it
while the button is down: what moving forward, moving back, closing on the
start and taking an end again do to the points.

Nothing here knows Blender, the mouse or the screen. The tool hands in
points already laid on the surface (in the space of the object) and reads
back the points to draw and, once the stroke is over, the curve to make.
So the whole behaviour is tested without a window (tests/test_trace.py).

The rules, for a head moving to a new point:
- Forward: the stretch from the head to the point is laid down in steps of
  `step`, so a fast stroke is as dense as a slow one.
- Back: a point near the stroke (within `back`) just behind the head,
  within `reach` along it, takes the stroke back to there. Going back along the stroke undoes it
  bit by bit; crossing an older part of the stroke, further back than
  `reach`, does not, so a curve may cross itself.
- Closing: the curve is closed if the button is let go with the mouse near
  the start, once the stroke is longer than `closing_length`. While the
  button is down nothing closes: the stroke may pass by its start and go
  on; `closed` tells whether letting go now would close it, for the ribbon
  to show it.
"""

import math


def distance(a, b):
    return math.dist(a, b)


def between(a, b, t):
    return tuple(x + t * (y - x) for x, y in zip(a, b))


class Trace:
    """The points of one curve being drawn, from its start to its head.

    `step` is the spacing of the points, `near` how close a point must be to
    the stroke to touch it (the radius of the brush), `reach` how far behind
    the head, along the stroke, moving back still undoes it. All three are
    lengths in the space of the object."""

    def __init__(self, start, step, near, reach=None, closing_length=None, back=None):
        self.points = [tuple(start)]
        # Where the mouse is: the head lags it by less than a step.
        self.mouse = tuple(start)
        self.step, self.near = step, near
        # How near the stroke a point must be to take it back: wider than
        # `near` if given, so that going back is easy.
        self.back = near if back is None else back
        self.reach = 3.0 * self.back if reach is None else reach
        # A small scribble near its start is not a loop.
        self.closing_length = 4.0 * near if closing_length is None else closing_length

    @property
    def head(self):
        return self.points[-1]

    @property
    def closed(self):
        """Whether the curve would be closed if the button were let go now."""
        return (
            len(self.points) > 2
            and distance(self.mouse, self.points[0]) <= self.near
            and self.length() >= self.closing_length
        )

    def length(self):
        """Along the stroke, from the start to the head."""
        return sum(distance(a, b) for a, b in zip(self.points, self.points[1:]))

    def behind(self, point):
        """The index of the point of the stroke `point` is back on, if it is
        on the stroke within `reach` behind the head; else None. Back means
        nearer that point than the head, by half a step: a point ahead of
        the head is always nearer the head than anything behind it."""
        along, found, gap = 0.0, None, math.inf
        for index in range(len(self.points) - 2, -1, -1):
            along += distance(self.points[index], self.points[index + 1])
            if along > self.reach + self.step:
                break
            here = distance(point, self.points[index])
            if here < gap:
                found, gap = index, here
        if found is None or gap > self.back or gap + self.step / 2.0 >= distance(point, self.head):
            return None
        return found

    def move(self, point):
        """The head moves to `point`: the stroke goes forward or back.
        Returns what happened: "forward", "back" or "still" (too close to
        the head to add a step)."""
        point = self.mouse = tuple(point)
        back = self.behind(point)
        if back is not None:
            del self.points[back + 1:]
            return "back"
        gap = distance(self.head, point)
        if gap < self.step:
            return "still"
        start, count = self.head, int(gap // self.step)
        self.points.extend(between(start, point, (k + 1) * self.step / gap) for k in range(count))
        return "forward"

    def turned(self):
        """The stroke drawn the other way: its start becomes its head, so it
        can be taken up again from there."""
        self.points.reverse()
        return self

    def curve(self):
        """What the tool makes of it once the stroke is over: its points and
        whether it closes on itself; None if it is too short to be a curve."""
        if len(self.points) < 2:
            return None
        return list(self.points), self.closed


def end_near(curves, point, near):
    """The curve and the end of it that `point` is on, to take up a curve
    again: (index of the curve, True for its start, False for its end), the
    nearest within `near`, or None. Closed curves have no end."""
    best = None
    for index, (points, closed) in enumerate(curves):
        if closed or not points:
            continue
        for at_start, end in ((True, points[0]), (False, points[-1])):
            gap = distance(point, end)
            if gap <= near and (best is None or gap < best[0]):
                best = (gap, index, at_start)
    return None if best is None else best[1:]


def resampled(points, step, closed=False):
    """The polyline through `points` again, with a point every `step` along
    it (and its last point), as a stroke is laid down."""
    points = [tuple(point) for point in points] + ([tuple(points[0])] if closed else [])
    out, carried = [points[0]], 0.0
    for a, b in zip(points, points[1:]):
        gap = distance(a, b)
        at = step - carried
        while at <= gap:
            out.append(between(a, b, at / gap))
            at += step
        carried = gap - (at - step)
    if closed:
        if len(out) > 1 and distance(out[-1], out[0]) < step / 2:
            out.pop()
    elif distance(out[-1], points[-1]) > 1e-9:
        out.append(points[-1])
    return out


def taken_up(points, closed, at_start, step, near, reach=None, closing_length=None, back=None):
    """A `Trace` of an existing curve, its head at the end chosen; its points
    laid again every `step`, as if it had been drawn."""
    trace = Trace(points[0], step, near, reach, closing_length, back)
    trace.points = resampled(points, step, closed)
    if at_start:
        trace.turned()
    trace.mouse = trace.head
    return trace


def touched(curves, point, near):
    """The index of the curve `point` is on (any of its points within
    `near`), the nearest, or None: the curve to delete under the brush."""
    best = None
    for index, (points, _) in enumerate(curves):
        for at in points:
            gap = distance(point, at)
            if gap <= near and (best is None or gap < best[0]):
                best = (gap, index)
    return None if best is None else best[1]


def pulled(head, mouse, rope):
    """Where the point that draws goes when the mouse is at `mouse`, the
    point being at `head` (pixels of the screen) and tied to the mouse by a
    rope of `rope` pixels: it stays while the rope is slack, and is pulled
    towards the mouse, the rope taut, once it is not. A rope of 0 is none."""
    gap = math.hypot(mouse[0] - head[0], mouse[1] - head[1])
    if gap <= rope:
        return tuple(head)
    if rope <= 0.0:
        return tuple(mouse)
    t = (gap - rope) / gap
    return (head[0] + t * (mouse[0] - head[0]), head[1] + t * (mouse[1] - head[1]))


def crease(samples, kind, least_angle):
    """The index of the sample on the ridge (`kind` "RIDGE") or in the
    valley ("VALLEY") that `samples` cross, or None if they cross none.
    `samples` are (point, normal) of the surface along a line across the
    stroke, in order. At each sample the surface turns by the angle between
    the normals of the samples on either side of it, seen from outside the
    object: outward (a ridge, convex) when the normals part along the line,
    inward (a valley, concave) when they meet. The
    crease is the run of samples that turn the way asked; it counts if it
    turns by `least_angle` (radians) in all, so that a rounded fold whose
    turn is spread over the run counts as much as a sharp edge; the sample
    of the run that turns the most is on it."""
    turns = [0.0] * len(samples)
    for index in range(1, len(samples) - 1):
        (before, normal_before), (after, normal_after) = samples[index - 1], samples[index + 1]
        if before is None or after is None or samples[index][0] is None:
            continue
        cosine = max(-1.0, min(1.0, sum(a * b for a, b in zip(normal_before, normal_after))))
        parting = sum((b - a) * (y - x) for a, b, x, y in zip(normal_before, normal_after, before, after))
        if (parting > 0.0) == (kind == "RIDGE"):
            turns[index] = math.acos(cosine)
    best = max(range(len(turns)), key=lambda index: turns[index])
    if turns[best] <= 0.0:
        return None
    first = last = best
    while first > 0 and turns[first - 1] > 0.0:
        first -= 1
    while last < len(turns) - 1 and turns[last + 1] > 0.0:
        last += 1
    # Each turn is measured over two steps of the line: the run turns by
    # half their sum.
    return best if sum(turns[first:last + 1]) / 2.0 >= least_angle else None


def edge_between(sample_at, low, high, rounds=8):
    """Where the surface turns between the offsets `low` and `high` along a
    line across the stroke, found by halves: `sample_at(offset)` gives the
    (point, normal) of the surface there. Each half keeps the side whose
    normal the middle is not like, so that on a sharp edge the point found
    is on the edge, `rounds` halvings near. The point there, or None if a
    ray misses."""
    point_low, normal_low = sample_at(low)
    point_high, normal_high = sample_at(high)
    if point_low is None or point_high is None:
        return None
    for _ in range(rounds):
        middle = (low + high) / 2.0
        _, normal = sample_at(middle)
        if normal is None:
            return None
        like_low = sum(a * b for a, b in zip(normal, normal_low))
        like_high = sum(a * b for a, b in zip(normal, normal_high))
        if like_low >= like_high:
            low = middle
        else:
            high = middle
    return sample_at((low + high) / 2.0)[0]
