"""The quads along a loop (docs/LOOP_EDITING.md, section 5): one row of quads
either side of it, of the size the project asks for, and where they cannot
be laid.

Nothing here knows Blender: the add-on hands in the points of a loop, the
size h asked for at each and the direction the surface faces there, and
reads back where the quads are and which are red. So the rules are tested
without a window (blender/test_loop_quads.py).

- Stations: the loop is cut where the quads' edges cross it, one every h
  along it, h taken where it falls: the n-th station is where the integral
  of ds / h along the loop reaches n, scaled so that the stations close up
  on a closed loop. Their number is the count of edges along the loop.
- The row: at each station an edge across the loop, h to either side of it
  on the surface (the side is the normal crossed with the tangent: left of
  the way the loop runs, seen from outside).
- Red, for a quad of one side: a bend tighter than h on that side (the row
  inside the bend folds); something closer than h on that side and running
  alongside, within 45 degrees (another loop, a border, the loop itself
  further along); a loop that crosses at a steeper angle meets this one at
  a vertex and is not red.
- An open loop has two ends: each needs a pole where the line stops.
"""

import numpy

# How close to parallel something must run to crowd a row: within 45 degrees.
ALONGSIDE = 0.5 ** 0.5
# Stations of the loop itself nearer than this, along it, are its neighbours
# and do not crowd it.
OWN_REACH = 2


def unit(vectors):
    lengths = numpy.linalg.norm(vectors, axis=-1, keepdims=True)
    return numpy.divide(vectors, lengths, out=numpy.zeros_like(vectors), where=lengths > 0.0)


def stations(points, sizes, closed):
    """Where the edges across the loop fall: (positions, size at each,
    count of edges along the loop). `points` (n, 3), `sizes` (n,)."""
    points = numpy.asarray(points, numpy.float64)
    sizes = numpy.asarray(sizes, numpy.float64)
    if closed:
        points, sizes = numpy.vstack([points, points[:1]]), numpy.append(sizes, sizes[0])
    lengths = numpy.linalg.norm(numpy.diff(points, axis=0), axis=1)
    keep = numpy.append(True, lengths > 0.0)
    points, sizes, lengths = points[keep], sizes[keep], lengths[lengths > 0.0]
    if len(points) < 2:
        return points[:1], sizes[:1], 0
    # In edges asked for: how many quads of the size there fit along it.
    along = numpy.append(0.0, numpy.cumsum(lengths / (0.5 * (sizes[:-1] + sizes[1:]))))
    total = along[-1]
    count = max(3 if closed else 1, int(round(total)))
    at = numpy.linspace(0.0, total, count + 1)
    if closed:
        at = at[:-1]
    placed = numpy.column_stack([numpy.interp(at, along, points[:, k]) for k in range(3)])
    return placed, numpy.interp(at, along, sizes), count


def tangents(positions, closed):
    if closed:
        return unit(numpy.roll(positions, -1, axis=0) - numpy.roll(positions, 1, axis=0))
    if len(positions) < 2:
        return numpy.zeros_like(positions)
    return unit(numpy.gradient(positions, axis=0))


def sides(positions, normals, closed):
    """The direction to the left of the loop at each station, on the
    surface."""
    return unit(numpy.cross(numpy.asarray(normals, numpy.float64), tangents(positions, closed)))


def tight(positions, normals, sizes, closed):
    """For each station, the side a bend tighter than the size there is on:
    +1 left, -1 right, 0 none."""
    count = len(positions)
    found = numpy.zeros(count, int)
    for k in range(count):
        if not closed and (k == 0 or k == count - 1):
            continue
        before, after = positions[k] - positions[k - 1], positions[(k + 1) % count] - positions[k]
        a, b = numpy.linalg.norm(before), numpy.linalg.norm(after)
        if a == 0.0 or b == 0.0:
            continue
        turn = numpy.arccos(numpy.clip(numpy.dot(before, after) / (a * b), -1.0, 1.0))
        if turn > 1e-9 and 0.5 * (a + b) / turn < sizes[k]:
            found[k] = 1 if numpy.dot(numpy.cross(before, after), normals[k]) > 0.0 else -1
    return found


def crowded(positions, sizes, left, closed, others, own=True):
    """For each station, whether something runs alongside closer than the
    size there, on the left and on the right: two arrays of booleans.
    `others` is a list of (points, directions) of other lines (loops,
    borders); the loop's own stations further along it count too."""
    tangent = tangents(positions, closed)
    count = len(positions)
    on_left, on_right = numpy.zeros(count, bool), numpy.zeros(count, bool)
    lines = list(others)
    if own and count > 2 * OWN_REACH + 1:
        lines.append((positions, tangent, True))
    for line in lines:
        points, directions = numpy.asarray(line[0], numpy.float64), unit(numpy.asarray(line[1], numpy.float64))
        if not len(points):
            continue
        offset = points[None, :, :] - positions[:, None, :]
        near = numpy.linalg.norm(offset, axis=2) < sizes[:, None]
        near &= numpy.abs(directions @ tangent.T).T > ALONGSIDE
        if len(line) > 2:
            # The loop itself: its neighbours along it are not something else.
            apart = numpy.abs(numpy.arange(count)[:, None] - numpy.arange(count)[None, :])
            if closed:
                apart = numpy.minimum(apart, count - apart)
            near &= apart > OWN_REACH
        side = numpy.einsum("ijk,ik->ij", offset, left)
        on_left |= (near & (side > 0.0)).any(axis=1)
        on_right |= (near & (side <= 0.0)).any(axis=1)
    return on_left, on_right


def row(positions, normals, sizes, closed, others=()):
    """The quads along one loop, its stations given (`stations`) with the
    normal of the surface at each: a dict of the stations, the far corners
    to the left and to the right (to be laid on the surface by the caller),
    the count of edges along it, for each quad of each side whether it is
    red (quad k from station k to the next), and the open ends."""
    positions = numpy.asarray(positions, numpy.float64)
    sizes = numpy.asarray(sizes, numpy.float64)
    count = len(positions)
    left = sides(positions, normals, closed)
    bends = tight(positions, numpy.asarray(normals, numpy.float64), sizes, closed)
    near_left, near_right = crowded(positions, sizes, left, closed, others)
    red_left_at, red_right_at = near_left | (bends > 0), near_right | (bends < 0)
    quads = count if closed else count - 1
    following = (numpy.arange(quads) + 1) % max(count, 1)
    red_left = red_left_at[:quads] | red_left_at[following]
    red_right = red_right_at[:quads] | red_right_at[following]
    return {
        "stations": positions,
        "left": positions + sizes[:, None] * left,
        "right": positions - sizes[:, None] * left,
        "closed": closed,
        "edges": count if closed else count - 1,
        "red_left": red_left,
        "red_right": red_right,
        "ends": [] if closed or count == 0 else [positions[0], positions[-1]],
    }
