"""Edge loops of a quad mesh, sorted as `openclare measure` sorts them.

    python3 loops.py <mesh.obj> [--edges <out.txt>]

A line-by-line port of `Connectivity::edge_loops` in
crates/openclare/src/metrics.rs (Section 5 of the paper): each edge loop is
followed straight on through every regular vertex; a loop is *spiral*,
whatever its ends, if a strip of quads leaving it across meets it again, the
same way round, within SPIRAL_ORDER quads; otherwise *closed* if it comes
back to its start, *open* if an end stops at an irregular vertex, *border*
if it runs from border to border. Edges of borders and of faces that are not
quads are on no loop.

Prints the counts and the shares of edges as one JSON object, with the same
names as `openclare measure --json`. With --edges, also writes one line per
edge on a loop: the two vertex indices (from 0) and its kind.

Plain Python, no dependency: Blender's Python imports it too.
"""

import json
import sys

SPIRAL_ORDER = 4
INVALID = -1
KINDS = ("closed", "spiral", "open", "border")


def read_obj(path):
    positions, faces = [], []
    with open(path) as lines:
        for line in lines:
            tokens = line.split()
            if not tokens:
                continue
            if tokens[0] == "v":
                positions.append(tuple(float(x) for x in tokens[1:4]))
            elif tokens[0] == "f":
                faces.append([int(t.split("/")[0]) - 1 for t in tokens[1:]])
    return positions, faces


def edge_loops(vertex_count, faces):
    """Returns (summary dict, {(a, b) with a < b: kind}) or None when the mesh
    is not a consistently oriented manifold (as `openclare measure`)."""
    origin, next_half, face_of = [], [], []
    for f, polygon in enumerate(faces):
        base = len(origin)
        for k, v in enumerate(polygon):
            origin.append(v)
            next_half.append(base + (k + 1) % len(polygon))
            face_of.append(f)
    halves = len(origin)

    def head(h):
        return origin[next_half[h]]

    directed = {}
    for h in range(halves):
        key = (origin[h], head(h))
        if key in directed:
            directed[key][1] += 1
        else:
            directed[key] = [h, 1]
    twin = [directed[(head(h), origin[h])][0] if (head(h), origin[h]) in directed else INVALID
            for h in range(halves)]

    uses = {}
    for polygon in faces:
        for k in range(len(polygon)):
            a, b = polygon[k], polygon[(k + 1) % len(polygon)]
            key = (min(a, b), max(a, b))
            uses[key] = uses.get(key, 0) + 1
    valence = [0] * vertex_count
    on_boundary = [False] * vertex_count
    misoriented = 0
    for (a, b), count in uses.items():
        valence[a] += 1
        valence[b] += 1
        if count == 1:
            on_boundary[a] = on_boundary[b] = True

        def same_way(key):
            return key in directed and directed[key][1] == 2

        if count == 2 and (same_way((a, b)) or same_way((b, a))):
            misoriented += 1
    if any(count > 2 for count in uses.values()) or misoriented:
        return None

    def is_irregular(v):
        return valence[v] != (3 if on_boundary[v] else 4)

    is_quad = [len(polygon) == 4 for polygon in faces]
    odd = [False] * vertex_count
    for polygon in faces:
        if len(polygon) != 4:
            for v in polygon:
                odd[v] = True

    def straight_on(h):
        v = head(h)
        if is_irregular(v) or on_boundary[v] or odd[v]:
            return None
        across = twin[next_half[h]]
        return next_half[across] if across != INVALID else None

    def quad_on_both_sides(h):
        return is_quad[face_of[h]] and is_quad[face_of[twin[h]]]

    def end_on_pole(h):
        return not on_boundary[head(h)]

    loop_of = [INVALID] * halves
    forward = [False] * halves
    loops = []  # (edges, closed, pole)
    for start in range(halves):
        if loop_of[start] != INVALID or twin[start] == INVALID or not quad_on_both_sides(start):
            continue
        loop_id = len(loops)
        edges, closed, pole = 0, False, False
        backward = twin[start]
        for first in (start, None):
            if first is not None:
                h = first
            elif closed:
                break
            else:
                h = straight_on(backward)
                if h is None:
                    pole |= end_on_pole(backward)
                    break
            while True:
                if loop_of[h] == loop_id:
                    closed = True
                    break
                loop_of[h] = loop_id
                loop_of[twin[h]] = loop_id
                along = h if first is not None else twin[h]
                forward[along] = True
                edges += 1
                on = straight_on(h)
                if on is None:
                    pole |= end_on_pole(h)
                    break
                h = on
        loops.append((edges, closed, pole))

    spiral = [False] * len(loops)
    for h in range(halves):
        loop_id = loop_of[h]
        if loop_id == INVALID or not is_quad[face_of[h]]:
            continue
        e = h
        for _ in range(SPIRAL_ORDER):
            opposite = next_half[next_half[e]]
            beyond = twin[opposite]
            if loop_of[opposite] == loop_id:
                spiral[loop_id] |= beyond != INVALID and forward[beyond] == forward[h]
                break
            if beyond == INVALID or not is_quad[face_of[beyond]]:
                break
            e = beyond

    kind_of_loop = []
    for k, (edges, closed, pole) in enumerate(loops):
        kind_of_loop.append("spiral" if spiral[k] else "closed" if closed else "open" if pole else "border")
    total = sum(loop[0] for loop in loops) or 1
    summary = {"quads": sum(is_quad), "faces": len(faces)}
    for kind in KINDS:
        summary["loops_" + kind] = sum(1 for k in kind_of_loop if k == kind)
    for kind in KINDS:
        summary["loop_edges_" + kind] = sum(l[0] for l, k in zip(loops, kind_of_loop) if k == kind) / total
    kind_of_edge = {}
    for h in range(halves):
        if loop_of[h] != INVALID:
            a, b = origin[h], head(h)
            kind_of_edge[(min(a, b), max(a, b))] = kind_of_loop[loop_of[h]]
    return summary, kind_of_edge


def main():
    arguments = sys.argv[1:]
    if not arguments or arguments[0].startswith("-"):
        sys.exit(__doc__)
    positions, faces = read_obj(arguments[0])
    result = edge_loops(len(positions), faces)
    if result is None:
        sys.exit("not a consistently oriented manifold: no loops measured")
    summary, kind_of_edge = result
    print(json.dumps(summary))
    if "--edges" in arguments:
        with open(arguments[arguments.index("--edges") + 1], "w") as out:
            for (a, b), kind in sorted(kind_of_edge.items()):
                out.write(f"{a} {b} {kind}\n")


if __name__ == "__main__":
    main()
