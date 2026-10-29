"""The loops a plane cuts on a mesh, without Blender: shapes whose answer is
known (a capped cylinder, two of them side by side, a bent tube, a tube with
a hole in it), as the Cut Volume tool would cut them. Run with
python3 tests/test_cut.py (make test-trace)."""

import importlib.util
import math
import pathlib

import numpy

HERE = pathlib.Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("cut", HERE / "openclare" / "cut.py")
cut = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(cut)


def cylinder(radius=0.5, height=4.0, around=48, along=40, centre=(0.0, 0.0), axis_points=None, holes=()):
    """A tube along z, capped at both ends; `axis_points`, if given, bends
    its axis (one centre per ring); `holes`, faces (ring, segment) left out."""
    positions, triangles = [], []
    for ring in range(along + 1):
        z = height * ring / along
        cx, cy = centre if axis_points is None else axis_points(z)
        for k in range(around):
            a = 2 * math.pi * k / around
            positions.append((cx + radius * math.cos(a), cy + radius * math.sin(a), z))
    for ring in range(along):
        for k in range(around):
            if (ring, k) in holes:
                continue
            a, b = ring * around + k, ring * around + (k + 1) % around
            c, d = a + around, b + around
            triangles += [(a, b, d), (a, d, c)]
    if not holes:
        for ring, flip in ((0, True), (along, False)):
            cx, cy = centre if axis_points is None else axis_points(height * ring / along)
            middle = len(positions)
            positions.append((cx, cy, height * ring / along))
            for k in range(around):
                a, b = ring * around + k, ring * around + (k + 1) % around
                triangles.append((middle, b, a) if flip else (middle, a, b))
    return numpy.array(positions), numpy.array(triangles)


def joined(*meshes):
    positions, triangles, offset = [], [], 0
    for p, t in meshes:
        positions.append(p)
        triangles.append(t + offset)
        offset += len(p)
    return numpy.concatenate(positions), numpy.concatenate(triangles)


# The plane of two rays of a perspective view, from an eye in front of the
# tube, through its left and right sides at height 2: horizontal.
eye = numpy.array([0.0, -10.0, 2.0])
plane = cut.plane_through_rays(eye, (-0.8, 10.0, 0.0), eye, (0.8, 10.0, 0.0))
point, normal = plane
assert abs(abs(normal[2]) - 1.0) < 1e-9, normal
# Orthographic: two rays of one direction.
_, flat = cut.plane_through_rays((-1.0, -10.0, 2.0), (0.0, 1.0, 0.0), (1.0, -10.0, 2.0), (0.0, 1.0, 0.0))
assert abs(abs(flat[2]) - 1.0) < 1e-9, flat
# A stroke of no length says no plane.
assert cut.plane_through_rays(eye, (0.0, 1.0, 0.0), eye, (0.0, 1.0, 0.0)) is None

# A capped cylinder cut across: one closed loop, a circle of its radius.
positions, triangles = cylinder()
loops = cut.plane_loops(positions, triangles, point, normal)
assert len(loops) == 1 and loops[0][1], [(len(p), c) for p, c in loops]
radii = numpy.linalg.norm(loops[0][0][:, :2], axis=1)
assert radii.min() > 0.49 and radii.max() <= 0.5 + 1e-9 and numpy.allclose(loops[0][0][:, 2], 2.0)
length = cut.length(loops[0][0], True)
assert abs(length - 2 * math.pi * 0.5) < 0.01, length
print("CYLINDER one loop of", len(loops[0][0]), "points, %.4f long for %.4f" % (length, math.pi))

# Two cylinders side by side, both cut by the plane: the loop kept is the
# one under the stroke.
other = cylinder(centre=(2.0, 0.0))
positions2, triangles2 = joined(cylinder(), other)
loops = cut.plane_loops(positions2, triangles2, point, normal)
assert len(loops) == 2 and all(closed for _, closed in loops)
kept, gap = cut.nearest_loop(loops, (0.0, -0.5, 2.0))
assert gap < 0.05 and numpy.linalg.norm(kept[0][:, :2], axis=1).max() <= 0.5 + 1e-9
print("TWO CYLINDERS two loops, the one under the stroke kept")

# A tube bent like an arm at the elbow, cut across its upper part along a
# plane tilted as the stroke was: a closed loop round it.
bent = cylinder(axis_points=lambda z: (0.0, 0.0) if z < 2.0 else (0.6 * (z - 2.0), 0.0))
tilted = numpy.array([0.3, 0.0, 1.0]) / numpy.linalg.norm([0.3, 0.0, 1.0])
loops = cut.plane_loops(*bent, (0.9, 0.0, 3.5), tilted)
kept, gap = cut.nearest_loop(loops, (0.9, -0.5, 3.5))
assert kept is not None and kept[1], [(len(p), c) for p, c in loops]
print("ELBOW a closed loop of", len(kept[0]), "points round the bent tube")

# A tube with a hole where the plane goes: the cut does not go round, it is
# open, and is no loop round a volume.
holed = cylinder(holes={(20, k) for k in range(10, 14)} | {(19, k) for k in range(10, 14)})
loops = cut.plane_loops(*holed, (0.0, 0.0, 2.0 + 1e-3), (0.0, 0.0, 1.0))
assert loops and not any(closed for _, closed in loops), [(len(p), c) for p, c in loops]
print("HOLE the cut through the hole is open:", [(len(p), c) for p, c in loops])

# The triangles the plane may cross, found from spheres round them, give the
# very loops all the triangles give, from far fewer.
for mesh, plane in (((positions, triangles), (point, normal)), ((positions2, triangles2), (point, normal)),
                    (bent, ((0.9, 0.0, 3.5), tilted)), (holed, ((0.0, 0.0, 2.0 + 1e-3), (0.0, 0.0, 1.0)))):
    near = cut.crossed(cut.bounds(*mesh, size=64), *plane)
    whole, few = cut.plane_loops(*mesh, *plane), cut.plane_loops(mesh[0], near, *plane)
    assert len(near) < len(mesh[1]) / 4, (len(near), len(mesh[1]))
    assert len(whole) == len(few), (len(whole), len(few))
    # The same points, from wherever the chaining of each starts.
    for (a, closed_a), (b, closed_b) in zip(sorted(whole, key=lambda l: len(l[0])), sorted(few, key=lambda l: len(l[0]))):
        apart = max(numpy.linalg.norm(b - point_a, axis=1).min() for point_a in a)
        assert closed_a == closed_b and len(a) == len(b) and apart < 1e-12, (closed_a, closed_b, len(a), len(b), apart)
print("CROSSED the same loops from a few of the triangles")

# A plane that misses the mesh: nothing.
assert cut.plane_loops(positions, triangles, (0.0, 0.0, 10.0), (0.0, 0.0, 1.0)) == []
print("CUT all checks passed")
