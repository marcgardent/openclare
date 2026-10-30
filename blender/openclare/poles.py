"""The poles of a retopo project (DT-181): the irregular vertices of the last
result, kept on the mesh of the object it was made from, drawn on it, and
those the artist touched given to the next remesh, which places the others
freely.

A pole is a point in the space of the mesh, a valence (3 or 5), and whether
it was touched: moved, added or pinned by hand. The first remesh of an
object fills its poles; each remesh after keeps the touched ones as they
are and takes the free ones from its result.
"""

import bpy

# Colours of the poles drawn: a valence 3 warm, a valence 5 cool; a touched
# one has a light ring round it.
COLOURS = {3: (0.95, 0.55, 0.12, 1.0), 5: (0.25, 0.6, 1.0, 1.0)}
RING = (1.0, 1.0, 1.0, 1.0)
SIZE, RING_SIZE = 9.0, 14.0


def redraw_views(_settings=None, _context=None):
    for window in bpy.context.window_manager.windows:
        for area in window.screen.areas if window.screen else ():
            if area.type == "VIEW_3D":
                area.tag_redraw()


class OpenclarePole(bpy.types.PropertyGroup):
    """A pole of the project."""

    position: bpy.props.FloatVectorProperty(options=set(), name="Position", size=3, subtype="XYZ")
    valence: bpy.props.IntProperty(options=set(), name="Valence", min=3, max=5, default=3)
    touched: bpy.props.BoolProperty(
        options=set(),
        name="Touched",
        description="Placed by hand: the next remesh has a pole of this valence here",
        update=redraw_views,
    )


class OpenclarePoles(bpy.types.PropertyGroup):
    """The poles of the retopo project of a mesh."""

    items: bpy.props.CollectionProperty(type=OpenclarePole, options=set())
    show: bpy.props.BoolProperty(
        options=set(),
        name="Show Poles",
        description="Draw the poles of the last result on the object: valence 3 orange, 5 blue, touched ringed",
        default=True,
        update=redraw_views,
    )


def placed(mesh, stretch=None):
    """The touched poles of `mesh`, as the engine takes them: the point
    (stretched as the positions are, see `stretch_of`) and the valence."""
    import numpy

    poles = []
    for pole in mesh.openclare_poles.items:
        if pole.touched:
            point = numpy.array(pole.position, numpy.float32)
            poles.append((point * numpy.float32(stretch) if stretch is not None else point, pole.valence))
    return poles


def refresh(mesh, made, stretch=None):
    """The poles of `mesh` after a run that `made` its result: the touched
    ones kept as they are, the free ones those of the result. Returns how
    many of the touched ones the result answers."""
    import numpy

    items = mesh.openclare_poles.items
    for index in reversed(range(len(items))):
        if not items[index].touched:
            items.remove(index)
    if made.poles is None:
        return 0
    points, valences, imposed = made.poles
    points = points.reshape(-1, 3)
    if stretch is not None:
        points = points / numpy.asarray(stretch, numpy.float32)
    for point, valence, answers in zip(points, valences, imposed):
        if answers < 0:
            pole = items.add()
            pole.position, pole.valence = tuple(float(x) for x in point), int(valence)
    return len(set(int(k) for k in imposed if k >= 0))


def owner_shown(context):
    """The object whose poles are drawn: the active one, or the object the
    active result was made from."""
    active = context.active_object
    if active is None:
        return None
    source = getattr(active, "openclare_source", None)
    owner = source if source is not None else active
    if owner.type != "MESH" or not owner.data.openclare_poles.items:
        return None
    return owner


def draw():
    """Draws the poles of the object shown, in the 3D view, a little towards
    the viewer so that the surface does not hide them."""
    import gpu
    from gpu_extras.batch import batch_for_shader

    context = bpy.context
    owner = owner_shown(context)
    if owner is None or not owner.data.openclare_poles.show or not owner.visible_get():
        return
    view = context.region_data
    if view is None:
        return
    from mathutils import Vector

    eye = view.view_matrix.inverted().translation
    # In an orthographic view, every point looks the same way.
    backwards = view.view_rotation @ Vector((0.0, 0.0, 1.0))
    matrix = owner.matrix_world
    lift = 0.004 * max(owner.dimensions)
    groups = {(3, False): [], (5, False): [], (3, True): [], (5, True): []}
    for pole in owner.data.openclare_poles.items:
        point = matrix @ Vector(pole.position)
        towards = (eye - point).normalized() if view.is_perspective else backwards
        point = point + towards * lift
        groups[(pole.valence if pole.valence in (3, 5) else 3, pole.touched)].append(point)

    shader = gpu.shader.from_builtin("POINT_UNIFORM_COLOR")
    gpu.state.depth_test_set("LESS_EQUAL")
    gpu.state.blend_set("ALPHA")
    ringed = groups[(3, True)] + groups[(5, True)]
    if ringed:
        gpu.state.point_size_set(RING_SIZE)
        shader.uniform_float("color", RING)
        batch_for_shader(shader, "POINTS", {"pos": ringed}).draw(shader)
    gpu.state.point_size_set(SIZE)
    for (valence, _), points in groups.items():
        if points:
            shader.uniform_float("color", COLOURS[valence])
            batch_for_shader(shader, "POINTS", {"pos": points}).draw(shader)
    gpu.state.point_size_set(1.0)
    gpu.state.blend_set("NONE")
    gpu.state.depth_test_set("NONE")


class OPENCLARE_OT_poles_free(bpy.types.Operator):
    """Let the next remesh place every pole of this object freely: none is
    touched any more"""

    bl_idname = "object.openclare_poles_free"
    bl_label = "Free All"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        owner = owner_shown(context)
        return owner is not None and any(pole.touched for pole in owner.data.openclare_poles.items)

    def execute(self, context):
        for pole in owner_shown(context).data.openclare_poles.items:
            pole.touched = False
        return {"FINISHED"}


class OPENCLARE_OT_poles_clear(bpy.types.Operator):
    """Forget the poles of this object, touched or not: the next remesh starts
    the project again"""

    bl_idname = "object.openclare_poles_clear"
    bl_label = "Clear"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        return owner_shown(context) is not None

    def execute(self, context):
        owner_shown(context).data.openclare_poles.items.clear()
        redraw_views()
        return {"FINISHED"}


def summary(poles):
    """What the panel says of the poles of a project."""
    items = poles.items
    threes = sum(pole.valence == 3 for pole in items)
    touched = sum(pole.touched for pole in items)
    return f"{len(items)} poles: {threes} of valence 3, {len(items) - threes} of 5; {touched} touched"


CLASSES = (OpenclarePole, OpenclarePoles, OPENCLARE_OT_poles_free, OPENCLARE_OT_poles_clear)

handle = None


def register():
    global handle
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Mesh.openclare_poles = bpy.props.PointerProperty(type=OpenclarePoles)
    handle = bpy.types.SpaceView3D.draw_handler_add(draw, (), "WINDOW", "POST_VIEW")


def unregister():
    global handle
    if handle is not None:
        bpy.types.SpaceView3D.draw_handler_remove(handle, "WINDOW")
        handle = None
    del bpy.types.Mesh.openclare_poles
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
