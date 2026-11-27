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
"""

import bpy

from . import experimental

# How wide the lines of the loops are drawn, in pixels.
LINE_WIDTH = 3.0


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
    import numpy

    loop = mesh.openclare_loops.items.add()
    loop.name = name or f"Loop {len(mesh.openclare_loops.items)}"
    flat = numpy.asarray(points, numpy.float32).reshape(-1)
    for _ in range(len(flat) // 3):
        loop.points.add()
    loop.points.foreach_set("co", flat)
    loop.closed = bool(closed)
    loop.sharp, loop.crease, loop.seam = bool(marks[0]), float(marks[1]), bool(marks[2])
    redraw_views()
    return loop


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
    gpu.state.blend_set("NONE")
    gpu.state.depth_test_set("NONE")


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

handle = None


def register():
    global handle
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Mesh.openclare_loops = bpy.props.PointerProperty(type=OpenclareLoops)
    handle = bpy.types.SpaceView3D.draw_handler_add(draw_loops, (), "WINDOW", "POST_VIEW")


def unregister():
    global handle
    if handle is not None:
        bpy.types.SpaceView3D.draw_handler_remove(handle, "WINDOW")
        handle = None
    del bpy.types.Mesh.openclare_loops
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
