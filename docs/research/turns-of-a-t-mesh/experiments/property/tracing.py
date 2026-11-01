"""The motorcycle graph of a quad mesh (Eppstein, Goodrich, Kim and Tamstorf,
2008): from every irregular vertex a trace leaves along each of its edges;
all advance one edge per step, straight on through regular vertices, and a
trace stops at the first vertex that is irregular or already on a trace;
when several reach the same free vertex at once, the lowest numbered goes on
and the others stop. The traces cut the surface into rectangles of quads: the
T-mesh, in the engine's conventions (`crates/openclare/src/tmesh/mod.rs`,
`quantization/export.rs`):

* a node is an irregular vertex or a vertex where a trace stops (three or
  more trace edges); `halves` lists the half-arcs leaving it counter-clockwise;
* an arc is a maximal straight run of trace edges between two nodes; half
  `2a` runs from `nodes[0]` to `nodes[1]` with `patches[0]` on its left, half
  `2a + 1` the other way with `patches[1]` on its left;
* a patch is a region of quads; its rim lists its halves counter-clockwise
  from its first corner, and the sides `0, 1, 2, 3` are the runs between
  its four corners, in rim order.

A region that is not a rectangle (not a disc, a corner that is not convex,
more or fewer than four corners, an arc with the region on both sides) makes
the instance unusable; `trace` raises `NotRectangles` and says why, and the
generator counts how often that happens.

On the double of a surface with borders (`quadmesh.double`) the border is
traced from the start, as the engine cuts along an open border: its edges
are trail edges of the marker `BORDER`, its vertices occupied, so that every
trace reaching it stops there, and its vertices of valence `2a != 4` start
traces as any singular vertex does. `mirrored` then reads the involution the
mirror induces on the T-mesh, or raises `NotRectangles` if there is none.
"""

# The trace id of the border edges in `trail` (no trace started them).
BORDER = -1


class NotRectangles(Exception):
    pass


class TMesh:
    def __init__(self, quad):
        self.quad = quad
        self.nodes = []       # vertex index of each node
        self.node_of = {}     # vertex -> node
        self.arcs = []        # dicts: nodes, edges (directed quad edges from nodes[0]), patches, sides
        self.patches = []     # dicts: faces, rim [(half, side)], corners
        self.trail = set()    # undirected trace edges (min, max)


def motorcycles(quad, border_edges=()):
    """The trace edges, as a dict from undirected quad edge to the traces
    (motorcycles) that ran it; the vertices where a trace stopped; and per
    trace its start vertex and where it stopped. The `border_edges`
    (undirected) are trail edges of `BORDER` from the start and their
    vertices occupied."""
    singular = [v for v in range(len(quad.positions)) if quad.valence[v] != 4]
    occupied = set(singular)
    trail = {}
    for (a, b) in sorted(border_edges):
        trail[(min(a, b), max(a, b))] = [BORDER]
        occupied.update((a, b))
    stopped = set()
    live = []
    starts, ends, first_edges = [], [], []
    for v in singular:
        for (a, b) in quad.star(v):
            live.append((len(starts), (a, b)))
            starts.append(v)
            ends.append(None)
            first_edges.append((a, b))
    while live:
        arrivals = {}
        for (m, (a, b)) in live:
            arrivals.setdefault(b, []).append(m)
        next_live = []
        for (m, (a, b)) in live:
            trail.setdefault((min(a, b), max(a, b)), []).append(m)
            if b in occupied or arrivals[b][0] != m:
                stopped.add(b)
                ends[m] = b
                continue
            occupied.add(b)
            next_live.append((m, quad.straight(a, b)))
        # A vertex reached by several at once is occupied from now on.
        for b, ms in arrivals.items():
            if len(ms) > 1:
                occupied.add(b)
                stopped.add(b)
        live = next_live
    return trail, stopped, starts, ends, first_edges


def trace(quad, border_edges=()):
    trail, stopped, starts, ends, first_edges = motorcycles(quad, border_edges)
    T = TMesh(quad)
    T.trace_of_edge = trail
    T.trace_start = starts
    T.trace_end = ends
    T.trace_first_edge = first_edges
    trail_edges_at = {}
    for (a, b) in trail:
        trail_edges_at.setdefault(a, []).append(b)
        trail_edges_at.setdefault(b, []).append(a)
    for v in range(len(quad.positions)):
        if quad.valence[v] != 4 or len(trail_edges_at.get(v, [])) >= 3:
            T.node_of[v] = len(T.nodes)
            T.nodes.append(v)
    for v, ends in trail_edges_at.items():
        if v not in T.node_of and len(ends) != 2:
            raise NotRectangles(f"vertex {v} on {len(ends)} trace edges is not a node")

    # The arcs: from each node along each trace edge, straight on to the next node.
    arc_of_edge = {}
    for n0, v in enumerate(T.nodes):
        for (a, b) in quad.star(v):
            if (min(a, b), max(a, b)) not in trail or (a, b) in arc_of_edge:
                continue
            edges = [(a, b)]
            while b not in T.node_of:
                a, b = quad.straight(a, b)
                edges.append((a, b))
            k = len(T.arcs)
            T.arcs.append({"nodes": [n0, T.node_of[b]], "edges": edges, "patches": [None, None], "sides": [None, None]})
            for (p, q) in edges:
                arc_of_edge[(p, q)] = (k, 0)
                arc_of_edge[(q, p)] = (k, 1)
    T.arc_of_edge = arc_of_edge
    for (a, b) in trail:
        if (a, b) not in arc_of_edge:
            # A closed run of trace edges without a node (a border circle no
            # trace reaches).
            raise NotRectangles(f"the trace edge {a}-{b} is on a loop without a node")

    # The patches: regions of quads joined across the edges that are not traces.
    region = [None] * len(quad.faces)
    for seed in range(len(quad.faces)):
        if region[seed] is not None:
            continue
        k = len(T.patches)
        region[seed], queue, faces = k, [seed], [seed]
        while queue:
            f = queue.pop()
            face = quad.faces[f]
            for i in range(4):
                a, b = face[i], face[(i + 1) % 4]
                if (min(a, b), max(a, b)) in trail:
                    continue
                g, _ = quad.where[(b, a)]
                if region[g] is None:
                    region[g] = k
                    queue.append(g)
                    faces.append(g)
        T.patches.append({"faces": faces})

    for k, patch in enumerate(T.patches):
        boundary = set()
        for f in patch["faces"]:
            face = quad.faces[f]
            for i in range(4):
                a, b = face[i], face[(i + 1) % 4]
                if (min(a, b), max(a, b)) in trail:
                    g, _ = quad.where[(b, a)]
                    if region[g] == k:
                        raise NotRectangles(f"patch {k} lies on both sides of the trace edge {a}-{b}")
                    boundary.add((a, b))
        # Walk the rim with the patch on the left; the fan of the patch's
        # quads at each vertex gives its angle there.
        start = min(boundary)
        rim_edges, angles = [], []
        edge = start
        while True:
            rim_edges.append(edge)
            a, b = edge
            fan = 1
            c, d = quad.next_in_face(a, b)
            while (c, d) not in boundary:
                g, _ = quad.where[(d, c)]
                if region[g] != k:
                    raise NotRectangles(f"patch {k}: the rim leaves the region at {b}")
                c, d = quad.next_in_face(d, c)
                fan += 1
            angles.append(fan)
            edge = (c, d)
            if edge == start:
                break
        if len(rim_edges) != len(boundary):
            raise NotRectangles(f"patch {k} has more than one rim: not a disc")
        # Group the rim edges into half-arcs, in rim order, with the angle at
        # the end of each half (at a node); a rim vertex that is not a node
        # must be straight.
        halves, half_angles = [], []
        for (a, b), fan in zip(rim_edges, angles):
            arc, direction = arc_of_edge[(a, b)]
            half = 2 * arc + direction
            if not halves or halves[-1] != half:
                if halves and T.arcs[halves[-1] // 2]["nodes"][1 - halves[-1] % 2] != T.node_of.get(a):
                    raise NotRectangles(f"patch {k}: half {halves[-1]} does not end at a node")
                halves.append(half)
                half_angles.append(None)
            if b in T.node_of:
                if fan not in (1, 2):
                    raise NotRectangles(f"patch {k}: angle {fan} quarter turns at vertex {b}")
                half_angles[-1] = fan
            elif fan != 2:
                raise NotRectangles(f"patch {k}: angle {fan} at the vertex {b}, which is not a node")
        if rim_edges[0][0] not in T.node_of and halves[0] == halves[-1] and len(halves) > 1:
            # The rim started inside an arc: its two pieces are one half, whose
            # angle is the one recorded at the end of the first piece.
            halves.pop()
            half_angles.pop()
        if halves[0] == halves[-1] and len(halves) > 1:
            raise NotRectangles(f"patch {k}: the rim starts inside an arc it holds twice")
        if any(angle is None for angle in half_angles):
            raise NotRectangles(f"patch {k}: a half without an angle")
        if half_angles.count(1) != 4:
            raise NotRectangles(f"patch {k} has {half_angles.count(1)} corners")
        if len(set(halves)) != len(halves):
            raise NotRectangles(f"patch {k} holds an arc twice")
        # Sides from the first corner: the rim starts right after the corner
        # at the lowest node, so that each corner closes a side.
        corners = [i for i, angle in enumerate(half_angles) if angle == 1]
        first = min(corners, key=lambda i: T.arcs[halves[i] // 2]["nodes"][1 - halves[i] % 2])
        order = [(first + 1 + step) % len(halves) for step in range(len(halves))]
        rim, side = [], 0
        for i in order:
            rim.append((halves[i], side))
            if half_angles[i] == 1:
                side += 1
        assert side == 4
        patch["rim"] = rim
        for half, side in rim:
            arc = T.arcs[half // 2]
            arc["patches"][half % 2] = k
            arc["sides"][half % 2] = side
    for a, arc in enumerate(T.arcs):
        if None in arc["patches"]:
            raise NotRectangles(f"arc {a} is on the rim of one patch only")
    return T


def mirrored(T, mirror, mirror_face):
    """The involution a mirror of the quad mesh (`mirror` on vertices,
    `mirror_face` on faces, orientation reversed) induces on the T-mesh: per
    node, arc and patch its image, and per arc whether the image runs the
    same way (0, from its first node) or back (1). Raises `NotRectangles`
    ("not symmetric: ...") when the image of a node is not a node, of an arc
    not an arc, of a patch not a patch, or the maps are not involutions."""
    node = []
    for n, v in enumerate(T.nodes):
        if mirror[v] not in T.node_of:
            raise NotRectangles(f"not symmetric: the image of node {n} is not a node")
        node.append(T.node_of[mirror[v]])
    arc, direction = [], []
    for k, a in enumerate(T.arcs):
        images = {T.arc_of_edge.get((mirror[p], mirror[q])) for (p, q) in a["edges"]}
        if len(images) != 1 or None in images:
            raise NotRectangles(f"not symmetric: the image of arc {k} is not an arc")
        (image, way), = images
        if len(T.arcs[image]["edges"]) != len(a["edges"]):
            raise NotRectangles(f"not symmetric: the image of arc {k} is a part of arc {image}")
        arc.append(image)
        direction.append(way)
    patch_of_face = {}
    for p, P in enumerate(T.patches):
        for f in P["faces"]:
            patch_of_face[f] = p
    patch = []
    for p, P in enumerate(T.patches):
        images = {patch_of_face[mirror_face[f]] for f in P["faces"]}
        if len(images) != 1 or len(T.patches[next(iter(images))]["faces"]) != len(P["faces"]):
            raise NotRectangles(f"not symmetric: the image of patch {p} is not a patch")
        patch.append(images.pop())
    for name, m in (("node", node), ("arc", arc), ("patch", patch)):
        if any(m[m[i]] != i for i in range(len(m))):
            raise NotRectangles(f"not symmetric: the {name} map is not an involution")
    # Orientation reversed: the patch on the left of an arc goes to the right
    # of its image, run the same way.
    for k, a in enumerate(T.arcs):
        left, right = (patch[p] for p in a["patches"])
        if T.arcs[arc[k]]["patches"] != ([right, left] if direction[k] == 0 else [left, right]):
            raise NotRectangles(f"not symmetric: the patches of arc {k} do not go to those of its image")
    return node, arc, direction, patch


def node_halves(T):
    """The half-arcs leaving each node, counter-clockwise."""
    out = []
    for n, v in enumerate(T.nodes):
        halves = []
        for (a, b) in T.quad.star(v):
            for k, arc in enumerate(T.arcs):
                if arc["edges"][0] == (a, b):
                    halves.append(2 * k)
                elif arc["edges"][-1] == (b, a):
                    halves.append(2 * k + 1)
        out.append(halves)
    return out
