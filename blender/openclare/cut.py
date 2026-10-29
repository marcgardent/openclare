"""The loops a plane cuts on a triangle mesh: what the Cut Volume tool makes
a curve of, the loop round a limb where the user drew across it.

Each triangle the plane crosses gives a segment between two of its edges;
the segments are chained edge to edge into polylines. On a closed surface
every one of them closes; one that ends on a border or a hole is open, and
is not a loop round a volume. Nothing here knows Blender: positions are a
numpy array of (n, 3), triangles one of (m, 3) indices.
"""

import numpy


def plane_through_rays(origin_a, direction_a, origin_b, direction_b):
    """The plane through two rays of the view (one per end of the stroke):
    a point of it and its unit normal, or None when the rays are one line
    (a stroke too short to say a plane). In perspective the rays share the
    eye; in an orthographic view they share their direction: one formula
    holds both."""
    origin_a, direction_a = numpy.asarray(origin_a, float), numpy.asarray(direction_a, float)
    origin_b, direction_b = numpy.asarray(origin_b, float), numpy.asarray(direction_b, float)
    normal = numpy.cross(direction_a, origin_b + direction_b - origin_a)
    length = numpy.linalg.norm(normal)
    if length < 1e-9 * max(numpy.linalg.norm(direction_a), 1e-12) * max(numpy.linalg.norm(origin_b + direction_b - origin_a), 1e-12):
        return None
    return origin_a, normal / length


class Bounds:
    """The triangles of a mesh in an order that keeps neighbours together,
    in groups of `size`, each triangle and each group with a sphere about it
    that holds it: what `crossed` looks at, made once per shape."""

    def __init__(self, triangles, centres, radii, group_centres, group_radii, size):
        self.triangles, self.centres, self.radii = triangles, centres, radii
        self.group_centres, self.group_radii, self.size = group_centres, group_radii, size


def spread_bits(values):
    """The 10 low bits of each value, two zero bits put between each: one
    axis of a Morton code."""
    x = values.astype(numpy.uint32) & numpy.uint32(0x3FF)
    x = (x | (x << 16)) & numpy.uint32(0x030000FF)
    x = (x | (x << 8)) & numpy.uint32(0x0300F00F)
    x = (x | (x << 4)) & numpy.uint32(0x030C30C3)
    return (x | (x << 2)) & numpy.uint32(0x09249249)


def bounds(positions, triangles, size=1024, chunk=1 << 20):
    """The `Bounds` of the mesh: the triangles sorted along a Morton curve
    of their centres, so that each group of `size` is a small part of the
    surface."""
    positions = numpy.asarray(positions, numpy.float32)
    triangles = numpy.asarray(triangles)
    centres = numpy.empty((len(triangles), 3), numpy.float32)
    radii = numpy.empty(len(triangles), numpy.float32)
    for first in range(0, len(triangles), chunk):
        corners = positions[triangles[first:first + chunk]]
        centres[first:first + chunk] = middle = corners.mean(axis=1)
        radii[first:first + chunk] = numpy.sqrt(((corners - middle[:, None]) ** 2).sum(axis=2).max(axis=1))
    low = centres.min(axis=0)
    cells = (centres - low) * numpy.float32(1023.0 / max(float((centres.max(axis=0) - low).max()), 1e-30))
    code = spread_bits(cells[:, 0]) | (spread_bits(cells[:, 1]) << 1) | (spread_bits(cells[:, 2]) << 2)
    order = numpy.argsort(code, kind="stable")
    triangles, centres, radii = triangles[order], centres[order], radii[order]
    starts = numpy.arange(0, len(triangles), size)
    counts = numpy.diff(numpy.append(starts, len(triangles))).astype(numpy.float32)
    group_centres = numpy.add.reduceat(centres, starts, axis=0) / counts[:, None]
    group = numpy.repeat(numpy.arange(len(starts)), counts.astype(int))
    reach = numpy.linalg.norm(centres - group_centres[group], axis=1) + radii
    group_radii = numpy.maximum.reduceat(reach, starts)
    return Bounds(triangles, centres, radii, group_centres, group_radii, size)


def crossed(bounds, point, normal):
    """The triangles the plane may cross: those whose sphere it meets, in
    the groups whose sphere it meets. All those it crosses are among them,
    so that `plane_loops` of these finds what it finds of all; a margin
    keeps the rounding of 32-bit floats from leaving one out."""
    normal = numpy.asarray(normal, numpy.float32)
    offset = numpy.float32(numpy.dot(point, normal))
    margin, slack = numpy.float32(1.001), numpy.float32(1e-6)
    groups = numpy.flatnonzero(numpy.abs(bounds.group_centres @ normal - offset) <= bounds.group_radii * margin + slack)
    if len(groups) == 0:
        return bounds.triangles[:0]
    near = (groups[:, None] * bounds.size + numpy.arange(bounds.size)).ravel()
    near = near[near < len(bounds.triangles)]
    height = bounds.centres[near] @ normal - offset
    return bounds.triangles[near[numpy.abs(height) <= bounds.radii[near] * margin + slack]]


def plane_loops(positions, triangles, point, normal):
    """The polylines where the plane through `point` with `normal` cuts the
    mesh: a list of (points, closed), points an array of (k, 3). Only the
    vertices of `triangles` are looked at, so that a few triangles of a big
    mesh (`crossed`) are cut at the cost of a few."""
    triangles = numpy.asarray(triangles)
    if len(triangles) == 0:
        return []
    # The vertices of `triangles` only, numbered again in the same order.
    used, triangles = numpy.unique(triangles, return_inverse=True)
    triangles = triangles.reshape(-1, 3)
    positions = numpy.asarray(positions, float)[used]
    side = (positions - numpy.asarray(point, float)) @ numpy.asarray(normal, float)
    # A vertex on the plane counts on the positive side, so that a segment
    # never ends on a vertex.
    above = side >= 0.0
    crossed = above[triangles].sum(axis=1)
    crossed = triangles[(crossed == 1) | (crossed == 2)]
    if len(crossed) == 0:
        return []
    # The edges of each crossed triangle the plane crosses, as sorted pairs.
    edges = numpy.stack([crossed[:, [0, 1]], crossed[:, [1, 2]], crossed[:, [2, 0]]], axis=1)
    cut = above[edges[..., 0]] != above[edges[..., 1]]
    pairs = numpy.sort(edges[cut].reshape(-1, 2), axis=1).reshape(-1, 2, 2)
    # Each edge crossed once is one point of the cut.
    unique, index = numpy.unique(pairs.reshape(-1, 2), axis=0, return_inverse=True)
    index = index.reshape(-1, 2)
    a, b = positions[unique[:, 0]], positions[unique[:, 1]]
    t = side[unique[:, 0]] / (side[unique[:, 0]] - side[unique[:, 1]])
    points = a + t[:, None] * (b - a)
    # Chain the segments: each point has one segment (a border) or two.
    neighbours = [[] for _ in range(len(unique))]
    for first, second in index.tolist():
        neighbours[first].append(second)
        neighbours[second].append(first)
    seen = [False] * len(unique)
    loops = []
    ends = [at for at, near in enumerate(neighbours) if len(near) == 1]
    for start in ends + list(range(len(unique))):
        if seen[start]:
            continue
        chain, previous, current = [start], None, start
        seen[start] = True
        while True:
            onward = [at for at in neighbours[current] if at != previous and not seen[at]]
            if not onward:
                break
            previous, current = current, onward[0]
            seen[current] = True
            chain.append(current)
        closed = len(chain) > 2 and chain[0] in neighbours[chain[-1]] and all(len(neighbours[at]) == 2 for at in chain)
        loop = points[chain]
        # A plane through vertices meets each twice, by two of its edges.
        keep = numpy.ones(len(loop), bool)
        keep[1:] = numpy.linalg.norm(numpy.diff(loop, axis=0), axis=1) > 1e-12
        if closed and len(loop) > 1 and numpy.linalg.norm(loop[-1] - loop[0]) <= 1e-12:
            keep[-1] = False
        loops.append((loop[keep], closed))
    return loops


def nearest_loop(loops, target):
    """The loop that passes nearest `target` (a point of the surface under
    the stroke), and its distance to it; (None, inf) if there is none."""
    best, gap = None, numpy.inf
    target = numpy.asarray(target, float)
    for loop in loops:
        distance = numpy.linalg.norm(loop[0] - target, axis=1).min()
        if distance < gap:
            best, gap = loop, distance
    return best, gap


def length(points, closed):
    points = numpy.asarray(points, float)
    steps = numpy.linalg.norm(numpy.diff(points, axis=0), axis=1).sum()
    return steps + (numpy.linalg.norm(points[0] - points[-1]) if closed else 0.0)
