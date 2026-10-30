"""A stroke drawn by hand made into a Bezier curve of as few control points
as hold it within a tolerance: the retopology of a curve.

The stroke is cut at its corners first, where it turns sharply. Each run
between corners is fitted by the method of Schneider ("An Algorithm for
Automatically Fitting Digitized Curves", Graphics Gems, 1990): one cubic
segment, its handles found by least squares along the tangents at its ends
and the points placed on it again by Newton steps; where the farthest point
is further than the tolerance, the run is cut there and each half fitted the
same way. Cutting at the farthest point is not the fewest cuts, so a last
pass joins neighbouring segments again wherever one segment holds both.

The shake of the hand is taken out first by a light smoothing, so that the
tolerance goes to the shape of the stroke and not to its shake. A closed
stroke is fitted the same way from a start of its own, the point where it
turns most, with the tangents on either side of the start alike so
that the curve goes through it smoothly. Nothing here knows Blender: the
points are tuples, the curve a list of (point, left handle, right handle,
corner).
"""

import math


def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def scale(a, k):
    return tuple(x * k for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def norm(a):
    return math.sqrt(dot(a, a))


def unit(a):
    length = norm(a)
    return scale(a, 1.0 / length) if length > 1e-12 else a


def bezier(segment, t):
    p0, p1, p2, p3 = segment
    s = 1.0 - t
    return tuple(s * s * s * a + 3 * s * s * t * b + 3 * s * t * t * c + t * t * t * d for a, b, c, d in zip(p0, p1, p2, p3))


def first(segment, t):
    p0, p1, p2, p3 = segment
    s = 1.0 - t
    return tuple(3 * s * s * (b - a) + 6 * s * t * (c - b) + 3 * t * t * (d - c) for a, b, c, d in zip(p0, p1, p2, p3))


def second(segment, t):
    p0, p1, p2, p3 = segment
    return tuple(6 * (1 - t) * (c - 2 * b + a) + 6 * t * (d - 2 * c + b) for a, b, c, d in zip(p0, p1, p2, p3))


def chord_parameters(points):
    """Each point's share of the length of the run, from 0 to 1."""
    along = [0.0]
    for a, b in zip(points, points[1:]):
        along.append(along[-1] + norm(sub(b, a)))
    total = along[-1] or 1.0
    return [length / total for length in along]


def tangent_at(points, index, reach):
    """The direction of the stroke at `index`, from the points up to `reach`
    on either side that exist."""
    before = points[max(index - reach, 0)]
    after = points[min(index + reach, len(points) - 1)]
    return unit(sub(after, before))


def handles(points, parameters, start_tangent, end_tangent):
    """The cubic from the first point to the last, leaving along
    `start_tangent` and arriving along -`end_tangent`, whose handle lengths
    fit the points best by least squares (Schneider)."""
    p0, p3 = points[0], points[-1]
    c00 = c01 = c11 = x0 = x1 = 0.0
    for point, t in zip(points, parameters):
        s = 1.0 - t
        a1 = scale(start_tangent, 3 * s * s * t)
        a2 = scale(end_tangent, 3 * s * t * t)
        c00 += dot(a1, a1)
        c01 += dot(a1, a2)
        c11 += dot(a2, a2)
        rest = sub(point, add(scale(p0, s * s * s + 3 * s * s * t), scale(p3, 3 * s * t * t + t * t * t)))
        x0 += dot(a1, rest)
        x1 += dot(a2, rest)
    determinant = c00 * c11 - c01 * c01
    chord = norm(sub(p3, p0))
    alpha = beta = chord / 3.0
    if abs(determinant) > 1e-12:
        alpha = (x0 * c11 - x1 * c01) / determinant
        beta = (c00 * x1 - c01 * x0) / determinant
    # Handles of no length, or turned back, make loops: a third of the chord.
    if alpha < 1e-6 * chord or beta < 1e-6 * chord:
        alpha = beta = chord / 3.0
    return (p0, add(p0, scale(start_tangent, alpha)), add(p3, scale(end_tangent, beta)), p3)


def farthest(points, parameters, segment):
    """The largest distance of a point from where it sits on `segment`, and
    the index of that point."""
    worst, at = 0.0, len(points) // 2
    for index, (point, t) in enumerate(zip(points, parameters)):
        gap = norm(sub(bezier(segment, t), point))
        if gap > worst:
            worst, at = gap, index
    return worst, at


def placed_again(points, parameters, segment):
    """The parameters moved by one Newton step each towards the nearest
    point of `segment`."""
    moved = []
    for point, t in zip(points, parameters):
        away = sub(bezier(segment, t), point)
        d1, d2 = first(segment, t), second(segment, t)
        slope = dot(d1, d1) + dot(away, d2)
        moved.append(min(max(t - dot(away, d1) / slope, 0.0), 1.0) if abs(slope) > 1e-12 else t)
    return moved


def fit_segment(points, start_tangent, end_tangent, tolerance):
    """One cubic over `points`, and how far its farthest point is."""
    parameters = chord_parameters(points)
    segment = handles(points, parameters, start_tangent, end_tangent)
    worst, at = farthest(points, parameters, segment)
    for _ in range(6):
        if worst <= tolerance:
            break
        parameters = placed_again(points, parameters, segment)
        segment = handles(points, parameters, start_tangent, end_tangent)
        worst, at = farthest(points, parameters, segment)
    return segment, worst, at


def fit_run(points, start_tangent, end_tangent, tolerance, reach):
    """Segments over a run of `points` with no corner, each within
    `tolerance`: cut at the farthest point until they all hold."""
    if len(points) == 2:
        chord = norm(sub(points[1], points[0]))
        return [(points[0], add(points[0], scale(start_tangent, chord / 3)),
                 add(points[1], scale(end_tangent, chord / 3)), points[1])]
    segment, worst, at = fit_segment(points, start_tangent, end_tangent, tolerance)
    if worst <= tolerance:
        return [segment]
    at = min(max(at, 1), len(points) - 2)
    middle = tangent_at(points, at, reach)
    return (fit_run(points[: at + 1], start_tangent, scale(middle, -1.0), tolerance, reach)
            + fit_run(points[at:], middle, end_tangent, tolerance, reach))


def joined(segments, points_of, tolerance, reach):
    """Neighbouring segments made one wherever one cubic holds both: a pass
    that undoes the cuts the farthest point did not need."""
    merged = True
    while merged and len(segments) > 1:
        merged = False
        for index in range(len(segments) - 1):
            run = points_of[index] + points_of[index + 1][1:]
            start = unit(sub(segments[index][1], segments[index][0]))
            end = unit(sub(segments[index + 1][2], segments[index + 1][3]))
            segment, worst, _ = fit_segment(run, start, end, tolerance)
            if worst <= tolerance:
                segments[index: index + 2] = [segment]
                points_of[index: index + 2] = [run]
                merged = True
                break
    return segments


def split_points(points, segments):
    """The points of the run each segment was fitted on: from the nearest
    point of the run to the start of a segment to that of its end."""
    cuts, from_index = [0], 0
    for segment in segments[:-1]:
        end = segment[3]
        nearest = min(range(from_index, len(points)), key=lambda index: norm(sub(points[index], end)))
        cuts.append(nearest)
        from_index = nearest
    cuts.append(len(points) - 1)
    return [points[a: b + 1] for a, b in zip(cuts, cuts[1:])]


def corners(points, angle, reach):
    """The indices where the stroke turns by more than `angle` (radians),
    read over `reach` points on either side: the sharpest of each bend."""
    turns = []
    for index in range(reach, len(points) - reach):
        before = unit(sub(points[index], points[index - reach]))
        after = unit(sub(points[index + reach], points[index]))
        turns.append((math.acos(max(-1.0, min(1.0, dot(before, after)))), index))
    found = []
    for turn, index in turns:
        if turn < angle:
            continue
        # The sharpest within reach only.
        if all(turn >= other for other, near in turns if abs(near - index) <= reach):
            if not found or index - found[-1] > reach:
                found.append(index)
    return found


def smoothed(points, closed, pinned=(), passes=3):
    """The stroke with the shake of the hand taken out: each point moved to
    the mean of itself (twice) and its two neighbours, `passes` times; the
    ends of an open stroke and the points `pinned` (its corners) stay where
    they are."""
    for _ in range(passes):
        count = len(points)
        moved = []
        for index, point in enumerate(points):
            if (not closed and index in (0, count - 1)) or index in pinned:
                moved.append(point)
                continue
            before, after = points[index - 1], points[(index + 1) % count]
            moved.append(tuple((a + 2 * p + b) / 4 for a, p, b in zip(before, point, after)))
        points = moved
    return points


def fit(points, closed, tolerance, corner_angle=math.radians(70.0), reach=3):
    """The Bezier curve of a stroke: a list of control points (point, left
    handle, right handle, corner), as few as hold every point of the stroke
    within `tolerance`. A closed stroke gives a closed curve, its last
    control point joined to its first."""
    points = [tuple(point) for point in points]
    if len(points) < 2:
        return []
    if closed:
        # Start at a corner if there is one, so that it is a joint; read the
        # turns across the start too.
        count = len(points)
        sharp = sorted({index % count for index in corners(points + points[: 2 * reach], corner_angle, reach)})
        start = sharp[0] if sharp else 0
        points = points[start:] + points[: start + 1]
    last = len(points) - 1
    sharp = set(corners(points, corner_angle, reach))
    if len(points) > 4:
        points = smoothed(points, closed and not sharp, pinned=sharp | {0, last} if closed and sharp else sharp)
    smooth = set()
    if closed and not sharp:
        # A closed curve with no corner is at least two segments, joined
        # smoothly at its start and at the point farthest from it.
        middle = max(range(last), key=lambda index: norm(sub(points[index], points[0])))
        smooth = {0, middle, last}
    elif closed:
        sharp |= {0, last}
    cut = sorted({0, last} | sharp | smooth)

    def leaving(index, until):
        if index in smooth:
            return seam_tangent(index)
        return unit(sub(points[min(index + reach, until)], points[index]))

    def arriving(index, since):
        if index in smooth:
            return scale(seam_tangent(index), -1.0)
        return unit(sub(points[max(index - reach, since)], points[index]))

    def seam_tangent(index):
        if index in (0, last):
            return unit(sub(points[1], points[last - 1]))
        return tangent_at(points, index, reach)

    curve = []
    for a, b in zip(cut, cut[1:]):
        run = points[a: b + 1]
        segments = fit_run(run, leaving(a, b), arriving(b, a), tolerance, reach)
        segments = joined(segments, split_points(run, segments), tolerance, reach)
        for segment in segments:
            if curve:
                point, left, _, corner = curve[-1]
                curve[-1] = (point, left, segment[1], corner)
            else:
                curve.append((segment[0], segment[0], segment[1], a in sharp))
            ends_run = segment is segments[-1]
            curve.append((segment[3], segment[2], segment[3], ends_run and b in sharp))
    if closed:
        # The last control point is the first: its left handle comes from it.
        end = curve.pop()
        point, _, right, corner = curve[0]
        curve[0] = (point, end[1], right, corner)
    else:
        point, left, _, _ = curve[-1]
        curve[-1] = (point, left, add(point, sub(point, left)), True)
        point, _, right, _ = curve[0]
        curve[0] = (point, add(point, sub(point, right)), right, True)
    return curve


def sampled(curve, closed, per_segment=16):
    """Points along a fitted curve, for measuring it against the stroke."""
    pairs = list(zip(curve, curve[1:] + (curve[:1] if closed else [])))
    points = []
    for (p0, _, out, _), (p3, into, _, _) in pairs:
        segment = (p0, out, into, p3)
        points.extend(bezier(segment, k / per_segment) for k in range(per_segment))
    if not closed and curve:
        points.append(curve[-1][0])
    return points


def deviation(stroke, curve, closed):
    """The largest distance from a point of the stroke to the fitted curve,
    read on a fine polyline of it: what the tolerance bounds."""
    dense = sampled(curve, closed, per_segment=64)
    if closed:
        dense = dense + dense[:1]

    def to_piece(point, a, b):
        along = sub(b, a)
        length = dot(along, along)
        t = 0.0 if length < 1e-18 else min(max(dot(sub(point, a), along) / length, 0.0), 1.0)
        return norm(sub(point, add(a, scale(along, t))))

    return max(min(to_piece(point, a, b) for a, b in zip(dense, dense[1:])) for point in stroke)
