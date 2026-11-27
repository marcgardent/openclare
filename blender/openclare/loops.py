"""The loops of a retopo project (docs/LOOP_EDITING.md): lines on the surface
of the source along which the quad mesh is to have a line of edges, kept as
data of its mesh, as the poles are (`poles`), and not as curve objects.

A loop is a list of points in the space of the object, laid on its surface,
whether it closes on itself, and the marks of the edges of the result along
it: sharp, crease, seam, as a curve drawn on the mesh has them. The engine
takes it as it takes a curve (`engine.remesh`, `curves`) and lays each point
on the nearest point of the surface, so that a loop the artist sculpted
away from is laid on the surface again at the next remesh.

The curves drawn so far are still read; Convert Curves to Loops makes them
loops, one per stretch, and takes the curve objects away.

Each loop is drawn with the quads along it (`loop_quads`): a row either
side, of the size the project asks for, `h(p)` (`SizeField`): the sizes of
the engine's analysis when one holds for the object as it is (Analyse),
else one size for the whole surface. The quads are worked out away from the
draw, by a timer, and kept while the loops, the shape and the sizes stay
the same.
"""

import bpy

from . import experimental, loop_quads

# How wide the lines of the loops are drawn, in pixels, and the rows of
# quads along them.
LINE_WIDTH = 3.0
ROW_WIDTH = 1.5
# Red, for quads that cannot be laid and the ends that need a pole.
RED = (0.9, 0.15, 0.15, 1.0)
RED_FILL = (0.9, 0.15, 0.15, 0.3)
END_SIZE = 10.0


def redraw_views(_settings=None, _context=None):
    for window in bpy.context.window_manager.windows:
        for area in window.screen.areas if window.screen else ():
            if area.type == "VIEW_3D":
                area.tag_redraw()


class OpenclareLoopPoint(bpy.types.PropertyGroup):
    """A point of a loop, in the space of the object."""

    co: bpy.props.FloatVectorProperty(options=set(), name="Position", size=3, subtype="XYZ")


class OpenclareLoop(bpy.types.PropertyGroup):
    """A loop of the project: a line the quad mesh is to have edges along."""

    points: bpy.props.CollectionProperty(type=OpenclareLoopPoint, options=set())
    closed: bpy.props.BoolProperty(
        options=set(), name="Closed", description="The last point joins the first", update=redraw_views
    )
    sharp: bpy.props.BoolProperty(
        options=set(), name="Sharp", description="Mark the edges of the result along this loop sharp",
        update=redraw_views,
    )
    crease: bpy.props.FloatProperty(
        options=set(),
        name="Crease",
        description="Crease weight of the edges of the result along this loop: 1 keeps them sharp under subdivision",
        default=0.0,
        min=0.0,
        max=1.0,
        update=redraw_views,
    )
    seam: bpy.props.BoolProperty(
        options=set(), name="Seam", description="Mark the edges of the result along this loop as seams",
        update=redraw_views,
    )


class OpenclareLoops(bpy.types.PropertyGroup):
    """The loops of the retopo project of a mesh."""

    items: bpy.props.CollectionProperty(type=OpenclareLoop, options=set())
    show: bpy.props.BoolProperty(
        options=set(),
        name="Show Loops",
        description="Draw the loops on the object and on its results, in the colour of their marks",
        default=True,
        update=redraw_views,
    )


def marks_of(loop):
    return (loop.sharp, loop.crease, loop.seam)


def points_of(loop):
    """The points of `loop`, a numpy array of three 32-bit floats each."""
    import numpy

    flat = numpy.empty(3 * len(loop.points), numpy.float32)
    loop.points.foreach_get("co", flat)
    return flat.reshape(-1, 3)


def add(mesh, points, closed, marks, name=None):
    """A new loop of `mesh` through `points` (in the space of the object),
    with the marks (sharp, crease, seam). Returns it."""
    loop = mesh.openclare_loops.items.add()
    loop.name = name or f"Loop {len(mesh.openclare_loops.items)}"
    loop.sharp, loop.crease, loop.seam = bool(marks[0]), float(marks[1]), bool(marks[2])
    set_points(loop, points, closed)
    return loop


def set_points(loop, points, closed):
    """`loop` made to go through `points` (in the space of the object)."""
    import numpy

    flat = numpy.asarray(points, numpy.float32).reshape(-1)
    loop.points.clear()
    for _ in range(len(flat) // 3):
        loop.points.add()
    loop.points.foreach_set("co", flat)
    loop.closed = bool(closed)
    redraw_views()


def loops_of(mesh):
    """The loops of `mesh` as the engine takes the curves, and their marks,
    as `curves_of` gives those of the curve objects: for each loop its
    points in the space of the object, whether it closes and whether it is
    sharp; alongside, its marks (sharp, crease, seam). Loops of fewer than
    two points are left out."""
    curves, marks = [], []
    for loop in mesh.openclare_loops.items:
        if len(loop.points) < 2:
            continue
        curves.append((points_of(loop), loop.closed and len(loop.points) > 2, loop.sharp))
        marks.append(marks_of(loop))
    return curves, marks


class SizeField:
    """The size the project asks for, `h(p)`, on the surface of `owner` as
    it is seen, in the units of its mesh: from the engine's analysis when
    one holds for the object as it is, at the vertex nearest a point; else
    one size everywhere, the Largest asked for with Sizes, or the edge of as
    many equal squares as the quads asked for. Also the borders of the
    surface, which crowd the rows of quads as other loops do."""

    def __init__(self, context, owner):
        import math

        import numpy

        from . import ANALYSES, analysis_key, scale_of, sizes_asked

        settings = owner.openclare
        mesh = owner.evaluated_get(context.evaluated_depsgraph_get()).data
        scale = scale_of(owner)
        area = numpy.empty(len(mesh.polygons), numpy.float64)
        mesh.polygons.foreach_get("area", area)
        by_size = settings.target == "SIZE" and settings.largest_quad > 0.0
        self.uniform = settings.largest_quad / scale if by_size else math.sqrt(float(area.sum()) / max(settings.quads, 1))
        self.scale, self.sizes = scale, None
        corners = numpy.empty(len(mesh.loops), numpy.int64)
        mesh.loops.foreach_get("vertex_index", corners)
        self.positions = numpy.empty(3 * len(mesh.vertices), numpy.float64)
        mesh.vertices.foreach_get("co", self.positions)
        self.positions = self.positions.reshape(-1, 3)
        self.starts = numpy.empty(len(mesh.polygons), numpy.int64)
        mesh.polygons.foreach_get("loop_start", self.starts)
        self.totals = numpy.empty(len(mesh.polygons), numpy.int64)
        mesh.polygons.foreach_get("loop_total", self.totals)
        self.corners = corners
        held = ANALYSES.get(owner.name)
        if held is not None and held["key"] == analysis_key(settings, owner, mesh):
            if by_size:
                self.sizes = sizes_asked(settings, held["analysis"], scale)[0]
            else:
                self.sizes = held["analysis"].at_vertices_for(settings.quads)
            seen = self.sizes[numpy.isfinite(self.sizes)]
            self.span = (float(seen.min()), float(seen.max())) if len(seen) else (self.uniform, self.uniform)
        # The borders: the edges of one face, as their middles and directions.
        edges = numpy.empty(len(mesh.loops), numpy.int64)
        mesh.loops.foreach_get("edge_index", edges)
        faces = numpy.bincount(edges, minlength=len(mesh.edges))
        ends = numpy.empty(2 * len(mesh.edges), numpy.int64)
        mesh.edges.foreach_get("vertices", ends)
        ends = ends.reshape(-1, 2)[faces == 1]
        a, b = self.positions[ends[:, 0]], self.positions[ends[:, 1]]
        self.borders = (0.5 * (a + b), b - a)

    def at(self, polygon, point):
        """The size at `point`, on the polygon of that index."""
        if self.sizes is None or polygon is None:
            return self.uniform
        start, total = self.starts[polygon], self.totals[polygon]
        vertices = self.corners[start : start + total]
        offsets = self.positions[vertices] - point
        nearest = vertices[int((offsets * offsets).sum(axis=1).argmin())]
        size = float(self.sizes[nearest])
        return size if size == size and size > 0.0 else self.uniform

    def said(self):
        """The size, in words, in the units of the scene."""
        if self.sizes is None:
            return f"Quad size {self.uniform * self.scale:.4g} everywhere: Analyse for the sizes of the shape"
        low, high = self.span
        return f"Quad size {low * self.scale:.4g} to {high * self.scale:.4g}, from the analysis"


# The quads along the loops of the object shown, and the size field they were
# laid with: each kept with what it was made from.
QUADS = {"key": None, "made": None, "pending": False, "field_key": None, "field": None}


def field_key(owner):
    from . import ANALYSES, preview

    settings = owner.openclare
    held = ANALYSES.get(owner.name)
    return (
        owner.name, settings.target, settings.quads, round(settings.largest_quad, 9),
        round(settings.smallest_quad, 9), id(held) if held is not None else None,
        preview.generation_of(owner.name), len(owner.data.vertices), tuple(round(x, 6) for x in owner.matrix_world.to_scale()),
    )


def quads_key(owner):
    loops_seen = tuple(
        (len(loop.points), loop.closed, round(float(points_of(loop).sum()), 6) if len(loop.points) else 0.0)
        for loop in owner.data.openclare_loops.items
    )
    return field_key(owner), loops_seen


def size_field(context, owner):
    """The size field of `owner`, made again only when what it depends on
    changed."""
    key = field_key(owner)
    if QUADS["field_key"] != key or QUADS["field"] is None:
        QUADS["field"], QUADS["field_key"] = SizeField(context, owner), key
    return QUADS["field"]


def quads_along(context, owner):
    """The quads along each loop of `owner` (`loop_quads.row`), in the order
    of its loops, laid on its surface; None for a loop of fewer than two
    points."""
    import numpy
    from mathutils import Vector

    from . import draw

    field = size_field(context, owner)
    tree = draw.surface_tree(context, owner)

    def laid(points):
        found = [tree.find_nearest(Vector(point)) for point in points]
        at = numpy.array([tuple(hit[0]) if hit[0] is not None else tuple(p) for hit, p in zip(found, points)])
        normals = numpy.array([tuple(hit[1]) if hit[1] is not None else (0.0, 0.0, 1.0) for hit in found])
        return at, normals, [hit[2] for hit in found]

    placed = []
    for loop in owner.data.openclare_loops.items:
        if len(loop.points) < 2:
            placed.append(None)
            continue
        points, closed = points_of(loop).astype(numpy.float64), loop.closed and len(loop.points) > 2
        on, _, polygons = laid(points)
        sizes = numpy.array([field.at(polygon, point) for polygon, point in zip(polygons, on)])
        stations, at, _ = loop_quads.stations(on, sizes, closed)
        stations, normals, _ = laid(stations)
        placed.append((stations, normals, at, closed))
    made = []
    for index, here in enumerate(placed):
        if here is None:
            made.append(None)
            continue
        stations, normals, at, closed = here
        reach = float(at.max())
        low, high = stations.min(axis=0) - reach, stations.max(axis=0) + reach
        others = []
        for other, there in enumerate(placed):
            if there is not None and other != index:
                others.append((there[0], loop_quads.tangents(there[0], there[3])))
        middles, directions = field.borders
        inside = ((middles >= low) & (middles <= high)).all(axis=1)
        if inside.any():
            others.append((middles[inside], directions[inside]))
        row = loop_quads.row(stations, normals, at, closed, others)
        row["left"] = laid(row["left"])[0]
        row["right"] = laid(row["right"])[0]
        made.append(row)
    return made


def refresh():
    """The timer: the quads along the loops of the object shown worked out
    again if what they were made from changed, and the views redrawn."""
    QUADS["pending"] = False
    owner = owner_shown(bpy.context)
    if owner is None:
        return None
    key = quads_key(owner)
    if key != QUADS["key"]:
        try:
            QUADS["made"] = quads_along(bpy.context, owner)
        except (ReferenceError, RuntimeError, ValueError) as error:
            print(f"openclare: the quads along the loops could not be laid: {error}")
            QUADS["made"] = None
        QUADS["key"] = key
        redraw_views()
    return None


def quads_shown(owner):
    """The quads along the loops of `owner` as they were last worked out if
    they still hold, else None, and the timer asked to work them out."""
    if QUADS["key"] == quads_key(owner):
        return QUADS["made"]
    if not QUADS["pending"]:
        QUADS["pending"] = True
        bpy.app.timers.register(refresh, first_interval=0.0)
    return None


def owner_shown(context):
    """The object whose loops are drawn: the active one, or the object the
    active result was made from."""
    active = context.active_object
    if active is None:
        return None
    source = getattr(active, "openclare_source", None)
    owner = source if source is not None else active
    if owner.type != "MESH" or not owner.data.openclare_loops.items:
        return None
    return owner


def draw_loops():
    """Draws the loops of the object shown, in the 3D view, on it and on the
    active result made from it when that is elsewhere, a little towards the
    viewer so that the surface does not hide them."""
    import gpu
    from gpu_extras.batch import batch_for_shader
    from mathutils import Vector

    from . import curve_colour

    context = bpy.context
    owner = owner_shown(context)
    if owner is None or not owner.data.openclare_loops.show or not experimental.shown():
        return
    view = context.region_data
    if view is None:
        return
    places = [owner.matrix_world] if owner.visible_get() else []
    active = context.active_object
    if active is not owner and active.visible_get() and active.matrix_world not in places:
        places.append(active.matrix_world)
    if not places:
        return
    eye = view.view_matrix.inverted().translation
    backwards = view.view_rotation @ Vector((0.0, 0.0, 1.0))
    lift = 0.002 * max(owner.dimensions)

    shader = gpu.shader.from_builtin("POLYLINE_UNIFORM_COLOR")
    shader.uniform_float("viewportSize", (context.region.width, context.region.height))
    shader.uniform_float("lineWidth", LINE_WIDTH)
    gpu.state.depth_test_set("LESS_EQUAL")
    gpu.state.blend_set("ALPHA")
    for loop in owner.data.openclare_loops.items:
        if len(loop.points) < 2:
            continue
        local = [Vector(point) for point in points_of(loop)]
        if loop.closed:
            local.append(local[0])
        shader.uniform_float("color", curve_colour(*marks_of(loop)))
        for matrix in places:
            lifted = []
            for point in local:
                point = matrix @ point
                towards = (eye - point).normalized() if view.is_perspective else backwards
                lifted.append(point + towards * lift)
            batch_for_shader(shader, "LINE_STRIP", {"pos": lifted}).draw(shader)
    rows = quads_shown(owner)
    if rows:
        draw_rows(rows, owner, places, eye, backwards, view.is_perspective, lift, shader)
    gpu.state.blend_set("NONE")
    gpu.state.depth_test_set("NONE")


def draw_rows(rows, owner, places, eye, backwards, perspective, lift, shader):
    """The rows of quads along the loops: their edges in the colour of the
    loop, the quads that cannot be laid filled in red, a red dot at each
    open end."""
    import gpu
    from gpu_extras.batch import batch_for_shader
    from mathutils import Vector

    from . import curve_colour

    flat = gpu.shader.from_builtin("UNIFORM_COLOR")
    dots = gpu.shader.from_builtin("POINT_UNIFORM_COLOR")
    shader.uniform_float("lineWidth", ROW_WIDTH)
    for loop, row in zip(owner.data.openclare_loops.items, rows):
        if row is None:
            continue
        for matrix in places:

            def seen(points):
                out = []
                for point in points:
                    point = matrix @ Vector(tuple(point))
                    towards = (eye - point).normalized() if perspective else backwards
                    out.append(point + towards * lift)
                return out

            stations, left, right = seen(row["stations"]), seen(row["left"]), seen(row["right"])
            count = len(stations)
            following = [(k + 1) % count for k in range(len(row["red_left"]))]
            lines = []
            for k in range(count):
                lines += [left[k], stations[k], stations[k], right[k]]
            for k, n in enumerate(following):
                lines += [left[k], left[n], right[k], right[n]]
            colour = curve_colour(*marks_of(loop))
            shader.uniform_float("color", (colour[0], colour[1], colour[2], 0.7))
            batch_for_shader(shader, "LINES", {"pos": lines}).draw(shader)
            red, edges = [], []
            for side, flags in ((left, row["red_left"]), (right, row["red_right"])):
                for k, n in enumerate(following):
                    if flags[k]:
                        red += [stations[k], stations[n], side[n], stations[k], side[n], side[k]]
                        edges += [stations[k], side[k], side[k], side[n], side[n], stations[n]]
            if red:
                flat.uniform_float("color", RED_FILL)
                batch_for_shader(flat, "TRIS", {"pos": red}).draw(flat)
                shader.uniform_float("color", RED)
                batch_for_shader(shader, "LINES", {"pos": edges}).draw(shader)
            if row["ends"]:
                gpu.state.point_size_set(END_SIZE)
                dots.uniform_float("color", RED)
                batch_for_shader(dots, "POINTS", {"pos": seen(row["ends"])}).draw(dots)
                gpu.state.point_size_set(1.0)
    shader.uniform_float("lineWidth", LINE_WIDTH)


def draw_counts():
    """The number of edges round each closed loop, written beside it."""
    import blf
    from bpy_extras import view3d_utils
    from mathutils import Vector

    context = bpy.context
    owner = owner_shown(context)
    if owner is None or not owner.data.openclare_loops.show or not experimental.shown() or not owner.visible_get():
        return
    rows = quads_shown(owner)
    if not rows or context.region_data is None:
        return
    font = 0
    blf.size(font, 14.0 * context.preferences.system.ui_scale)
    for row in rows:
        if row is None or not row["closed"]:
            continue
        at = view3d_utils.location_3d_to_region_2d(
            context.region, context.region_data, owner.matrix_world @ Vector(tuple(row["right"][0]))
        )
        if at is None:
            continue
        blf.color(font, 1.0, 1.0, 1.0, 1.0)
        blf.position(font, at.x + 6.0, at.y + 6.0, 0.0)
        blf.draw(font, f"{row['edges']}")


def row_text(row):
    """What the panel says of the quads along one loop."""
    if row is None:
        return ""
    said = f"{row['edges']} round" if row["closed"] else f"{row['edges']} along"
    red = int(row["red_left"].sum() + row["red_right"].sum())
    if red:
        said += f", {red} red"
    if row["ends"]:
        said += ", 2 ends"
    return said


class OPENCLARE_OT_curves_to_loops(bpy.types.Operator):
    """Make the curves drawn on this object loops of it, one per stretch,
    their marks kept, and take the curve objects away"""

    bl_idname = "object.openclare_curves_to_loops"
    bl_label = "Convert Curves to Loops"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        from . import curve_objects, object_to_remesh

        owner = object_to_remesh(context)
        return owner is not None and owner.type == "MESH" and bool(curve_objects(owner))

    def execute(self, context):
        from . import curve_objects, curves_of, object_to_remesh

        owner = object_to_remesh(context)
        curves, marks = curves_of(owner)
        for (points, closed, _), marked in zip(curves, marks):
            add(owner.data, points, closed, marked)
        drawn = curve_objects(owner)
        for curve in drawn:
            bpy.data.objects.remove(curve)
        self.report({"INFO"}, f"openclare: {len(curves)} loops made of {len(drawn)} curves")
        return {"FINISHED"}


class OPENCLARE_OT_loops_clear(bpy.types.Operator):
    """Forget the loops of this object"""

    bl_idname = "object.openclare_loops_clear"
    bl_label = "Clear"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        return owner_shown(context) is not None

    def execute(self, context):
        owner_shown(context).data.openclare_loops.items.clear()
        redraw_views()
        return {"FINISHED"}


def marks_text(loop):
    """The marks of a loop, in a few words."""
    said = [name for name, on in (("sharp", loop.sharp), ("seam", loop.seam)) if on]
    if loop.crease > 0.0:
        said.append(f"crease {loop.crease:.2f}")
    return ", ".join(said) or "line of edges"


def summary(loops):
    """What the panel says of the loops of a project."""
    items = loops.items
    closed = sum(loop.closed for loop in items)
    return f"{len(items)} loops: {closed} closed, {len(items) - closed} open"


CLASSES = (
    OpenclareLoopPoint,
    OpenclareLoop,
    OpenclareLoops,
    OPENCLARE_OT_curves_to_loops,
    OPENCLARE_OT_loops_clear,
)

handles = []


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Mesh.openclare_loops = bpy.props.PointerProperty(type=OpenclareLoops)
    handles.append(bpy.types.SpaceView3D.draw_handler_add(draw_loops, (), "WINDOW", "POST_VIEW"))
    handles.append(bpy.types.SpaceView3D.draw_handler_add(draw_counts, (), "WINDOW", "POST_PIXEL"))


def unregister():
    for handle in handles:
        bpy.types.SpaceView3D.draw_handler_remove(handle, "WINDOW")
    handles.clear()
    if bpy.app.timers.is_registered(refresh):
        bpy.app.timers.unregister(refresh)
    QUADS.update(key=None, made=None, pending=False, field_key=None, field=None)
    del bpy.types.Mesh.openclare_loops
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
