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

from . import experimental

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


def draw_poles():
    """Draws the poles of the object shown, in the 3D view, a little towards
    the viewer so that the surface does not hide them."""
    import gpu
    from gpu_extras.batch import batch_for_shader

    context = bpy.context
    owner = owner_shown(context)
    if owner is None or not owner.data.openclare_poles.show or not owner.visible_get() or not experimental.shown():
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


# How near the mouse, in pixels, a pole must be drawn for the tool to take it.
PICK_PIXELS = 15.0


def under_mouse(context, owner, mouse):
    """The index of the pole of `owner` the mouse (region pixels) is on, or
    None: among those drawn within `PICK_PIXELS` of it, the nearest to the
    point of the surface under it, so that one on the far side is not taken
    through the object."""
    from bpy_extras import view3d_utils
    from mathutils import Vector

    from . import draw

    region, view = context.region, context.region_data
    hit = draw.hit(context, owner, draw.surface_tree(context, owner), mouse)
    near = []
    for index, pole in enumerate(owner.data.openclare_poles.items):
        at = view3d_utils.location_3d_to_region_2d(region, view, owner.matrix_world @ Vector(pole.position))
        if at is not None and (at - Vector(mouse)).length <= PICK_PIXELS:
            depth = (Vector(pole.position) - Vector(hit)).length if hit is not None else 0.0
            near.append((depth, (at - Vector(mouse)).length, index))
    return min(near)[2] if near else None


def mouse_of(event):
    return (event.mouse_region_x, event.mouse_region_y)


class OPENCLARE_OT_pole_move(bpy.types.Operator):
    """Drag a pole along the surface: the next remesh has a pole of its
    valence where it is let go"""

    bl_idname = "object.openclare_pole_move"
    bl_label = "Move Pole"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        return context.area is not None and context.area.type == "VIEW_3D" and owner_shown(context) is not None

    def invoke(self, context, event):
        from . import draw

        self.owner = owner_shown(context)
        self.index = under_mouse(context, self.owner, mouse_of(event))
        if self.index is None:
            return {"CANCELLED", "PASS_THROUGH"}
        pole = self.owner.data.openclare_poles.items[self.index]
        self.before = (tuple(pole.position), pole.touched)
        self.tree = draw.surface_tree(context, self.owner)
        context.window_manager.modal_handler_add(self)
        return {"RUNNING_MODAL"}

    def modal(self, context, event):
        from . import draw

        pole = self.owner.data.openclare_poles.items[self.index]
        if event.type == "MOUSEMOVE":
            point = draw.hit(context, self.owner, self.tree, mouse_of(event))
            if point is not None:
                pole.position, pole.touched = point, True
                context.region.tag_redraw()
        elif event.type == "LEFTMOUSE" and event.value == "RELEASE":
            pole.touched = True
            return {"FINISHED"}
        elif event.type in {"RIGHTMOUSE", "ESC"}:
            pole.position, pole.touched = self.before
            context.region.tag_redraw()
            return {"CANCELLED"}
        return {"RUNNING_MODAL"}


class OPENCLARE_OT_pole_add(bpy.types.Operator):
    """Place a pole on the surface under the mouse, touched: the next remesh
    has a vertex of this valence there, and balances it with the others"""

    bl_idname = "object.openclare_pole_add"
    bl_label = "Add Pole"
    bl_options = {"REGISTER", "UNDO"}

    valence: bpy.props.IntProperty(name="Valence", min=3, max=5, default=3, options={"SKIP_SAVE"})

    @classmethod
    def poll(cls, context):
        return context.area is not None and context.area.type == "VIEW_3D" and owner_shown(context) is not None

    def invoke(self, context, event):
        from . import draw

        if self.valence not in (3, 5):
            self.report({"ERROR"}, "openclare: a pole has a valence of 3 or 5")
            return {"CANCELLED"}
        owner = owner_shown(context)
        point = draw.hit(context, owner, draw.surface_tree(context, owner), mouse_of(event))
        if point is None:
            return {"CANCELLED"}
        pole = owner.data.openclare_poles.items.add()
        pole.position, pole.valence, pole.touched = point, self.valence, True
        redraw_views()
        return {"FINISHED"}


class OPENCLARE_OT_pole_remove(bpy.types.Operator):
    """Remove the pole under the mouse: the next remesh places its poles
    without it"""

    bl_idname = "object.openclare_pole_remove"
    bl_label = "Remove Pole"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        return context.area is not None and context.area.type == "VIEW_3D" and owner_shown(context) is not None

    def invoke(self, context, event):
        owner = owner_shown(context)
        index = under_mouse(context, owner, mouse_of(event))
        if index is None:
            return {"CANCELLED"}
        owner.data.openclare_poles.items.remove(index)
        redraw_views()
        return {"FINISHED"}


class OPENCLARE_OT_pole_pin(bpy.types.Operator):
    """Pin the pole under the mouse where it is, or let it free: a pinned
    pole is kept by the next remesh, a free one placed again"""

    bl_idname = "object.openclare_pole_pin"
    bl_label = "Pin or Free Pole"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        return context.area is not None and context.area.type == "VIEW_3D" and owner_shown(context) is not None

    def invoke(self, context, event):
        owner = owner_shown(context)
        index = under_mouse(context, owner, mouse_of(event))
        if index is None:
            return {"CANCELLED"}
        pole = owner.data.openclare_poles.items[index]
        pole.touched = not pole.touched
        return {"FINISHED"}


class OPENCLARE_TL_poles(bpy.types.WorkSpaceTool):
    """The tool in the toolbar: the poles of the last result, edited on the
    object before the next remesh."""

    bl_space_type = "VIEW_3D"
    bl_context_mode = "OBJECT"
    bl_idname = "openclare.poles"
    bl_label = "Poles (openclare)"
    bl_description = (
        "Experimental. Edit the poles of the last result on the object: drag one along the surface, 3 or 5 to add one "
        "under the mouse, P to pin or free one, X to remove one. Those touched are kept by the next remesh, "
        "which places the others round them"
    )
    bl_icon = "ops.generic.select_circle"
    bl_widget = None
    bl_keymap = (
        (OPENCLARE_OT_pole_move.bl_idname, {"type": "LEFTMOUSE", "value": "PRESS"}, None),
        (OPENCLARE_OT_pole_add.bl_idname, {"type": "THREE", "value": "PRESS"}, {"properties": [("valence", 3)]}),
        (OPENCLARE_OT_pole_add.bl_idname, {"type": "FIVE", "value": "PRESS"}, {"properties": [("valence", 5)]}),
        (OPENCLARE_OT_pole_pin.bl_idname, {"type": "P", "value": "PRESS"}, None),
        (OPENCLARE_OT_pole_remove.bl_idname, {"type": "X", "value": "PRESS"}, None),
        (OPENCLARE_OT_pole_remove.bl_idname, {"type": "DEL", "value": "PRESS"}, None),
    )

    def draw_settings(context, layout, tool):
        owner = owner_shown(context)
        if owner is None:
            layout.label(text="Remesh the object once to have its poles")
            return
        layout.label(text=summary(owner.data.openclare_poles))
        layout.prop(owner.data.openclare_poles, "show")
        layout.operator(OPENCLARE_OT_poles_free.bl_idname)


def summary(poles):
    """What the panel says of the poles of a project."""
    items = poles.items
    threes = sum(pole.valence == 3 for pole in items)
    touched = sum(pole.touched for pole in items)
    return f"{len(items)} poles: {threes} of valence 3, {len(items) - threes} of 5; {touched} touched"


CLASSES = (
    OpenclarePole,
    OpenclarePoles,
    OPENCLARE_OT_poles_free,
    OPENCLARE_OT_poles_clear,
    OPENCLARE_OT_pole_move,
    OPENCLARE_OT_pole_add,
    OPENCLARE_OT_pole_remove,
    OPENCLARE_OT_pole_pin,
)

handle = None


def tools(shown):
    """The Poles tool put in the toolbar or taken out: it is experimental
    (see `experimental`)."""
    if shown:
        bpy.utils.register_tool(OPENCLARE_TL_poles, separator=True)
    else:
        bpy.utils.unregister_tool(OPENCLARE_TL_poles)


def register():
    global handle
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Mesh.openclare_poles = bpy.props.PointerProperty(type=OpenclarePoles)
    handle = bpy.types.SpaceView3D.draw_handler_add(draw_poles, (), "WINDOW", "POST_VIEW")


def unregister():
    global handle
    if handle is not None:
        bpy.types.SpaceView3D.draw_handler_remove(handle, "WINDOW")
        handle = None
    del bpy.types.Mesh.openclare_poles
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
