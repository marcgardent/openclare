"""What the Hard Edges setting does, shown over the object in the 3D viewport
while a toggle of the panel is on: in solid lines the edges the quads will
follow, sharper than the angle and still sharp at the size of the quads
asked for; in faint lines those sharper than the angle that the engine
leaves out, too small for those quads (the grain of a sculpt).

The engine finds them by the code the remesh finds them with, at the size
of the quads it would make, on the mesh it would be given: the object with
its modifiers. That runs in a thread of its own, once the settings, the
object and its shape have stayed as they are for `QUIET` seconds. Nothing is
written into the mesh or the file: the toggle belongs to the window manager,
which is not saved, and the lines are drawn, never made.
"""

import time

import bpy

# Blender's own colour for the edges marked sharp, a cyan that stands out on
# the default grey of the solid view, for the edges kept; and an orange red
# that is partly transparent and thinner, which reads as fainter, for those
# left out. The two are far apart in hue, for those who cannot tell red from
# green as much as for the others.
KEPT_COLOUR = (0.0, 0.8, 1.0, 1.0)
LEFT_OUT_COLOUR = (1.0, 0.35, 0.1, 0.75)
# Widths in pixels.
KEPT_WIDTH = 3.0
LEFT_OUT_WIDTH = 2.0
# How long the settings and the object stay as they are before the edges are
# found again, in seconds, and how often that is looked at.
QUIET = 0.25
TICK = 0.1
# How far the lines are brought towards the eye, not to sink into the
# surface they lie on: what Blender brings its own wires forward by.
OFFSET = 1.0

# The settings of an object the edges found depend on: those that set the
# size of the quads, and the angle.
WATCHED = (
    "target", "quads", "largest_quad", "smallest_quad", "adaptive_size", "quads_around",
    "use_hard_edges", "hard_edge_angle", "density_group", "density_factor",
)


class Shown:
    """The edges drawn for an object: its name, and for the edges kept and
    those left out, their vertices (two per edge, of the mesh the engine was
    given) and where those are in the object, three floats each."""

    def __init__(self, name, kept, left_out, positions):
        self.name = name
        self.kept, self.left_out = kept, left_out
        self.kept_points = positions[kept]
        self.left_out_points = positions[left_out]
        # Made at the first draw, where there is a GPU to make them on.
        self.batches = None


class Job:
    """The edges of one object, found by the engine in a thread of its own,
    on arrays read beforehand: the thread touches nothing of Blender."""

    def __init__(self, given, key, name):
        import threading

        self.given, self.key, self.name = given, key, name
        self.found = self.error = None
        self.thread = threading.Thread(target=self.work, name="openclare hard edges", daemon=True)

    def work(self):
        from . import engine

        given = self.given
        vertices, triangles, corners = given.counts
        try:
            kept, left_out = engine.hard_edges(
                given.addresses[0], vertices, given.addresses[1], triangles, given.addresses[2], corners,
                given.preset, given.settings, given.density,
            )
            self.found = Shown(self.name, kept, left_out, given.kept[0].reshape(-1, 3))
        except Exception as error:  # noqa: BLE001
            self.error = error


class State:
    """Where the preview is: what is drawn, the draw handler while there is
    something to draw, the object and settings last seen and since when, and
    the job under way."""

    def __init__(self):
        self.shown = self.handle = self.job = None
        self.key, self.changed_at, self.pending = None, 0.0, False
        self.error = None
        # How many times the shape of each object, by name, has changed.
        self.generations = {}


state = State()


def is_on(context=None):
    wm = (context or bpy.context).window_manager
    return bool(getattr(wm, "openclare_show_hard_edges", False))


def watched_object():
    """The object the preview is about: the active one, if it is a mesh to
    remesh (not a result of the add-on)."""
    from . import source_of

    view_layer = getattr(bpy.context, "view_layer", None)
    owner = view_layer.objects.active if view_layer is not None else None
    if owner is None or owner.type != "MESH" or source_of(owner) is not None:
        return None
    return owner


def key_of(owner):
    """What the edges of `owner` depend on: which object, its shape, and its
    settings; its scale too where sizes are asked for, in the units of the
    scene."""
    settings = owner.openclare
    key = (owner.name, owner.as_pointer(), state.generations.get(owner.name, 0))
    key += tuple(getattr(settings, name) for name in WATCHED)
    if settings.target == "SIZE":
        key += tuple(owner.matrix_world.to_scale())
    return key


def step(now):
    """One look at the object and its settings, at time `now` (in seconds):
    a change is waited out for `QUIET` seconds, then the edges are found again
    in the background, and drawn once found. Returns when to look again, or
    None once the preview is off."""
    if not is_on():
        clear()
        return None
    owner = watched_object()
    if owner is not None:
        # Brought up to date first: an object not evaluated yet would be
        # when read, and tell a change of shape that is none.
        bpy.context.evaluated_depsgraph_get()
    key = key_of(owner) if owner is not None else None
    if key != state.key:
        if state.shown is not None and (owner is None or state.shown.name != owner.name):
            # Another object, or none: what was drawn is for an object that
            # is no longer the one shown, or no longer there.
            hide()
        state.key, state.changed_at, state.pending = key, now, key is not None
    job = state.job
    if job is not None:
        if job.thread.is_alive():
            return TICK
        state.job = None
        if job.key == state.key:
            if job.error is not None:
                state.error = str(job.error)
                hide()
            else:
                state.error = None
                show(job.found)
    if state.pending and state.job is None and now - state.changed_at >= QUIET:
        state.pending = False
        start(owner, key)
    return TICK


def start(owner, key):
    """Reads `owner` with its modifiers, as the remesh does, and has the
    engine find its edges in a thread of its own."""
    from . import BASE_PRESET, engine, engine_input, engine_settings_of

    evaluated = owner.evaluated_get(bpy.context.evaluated_depsgraph_get())
    source = evaluated.to_mesh()
    try:
        engine_settings, density = engine_settings_of(owner, evaluated, source, write=False)
        given = engine_input(source, BASE_PRESET, engine_settings, density, copy=True)
    except engine.EngineError as error:
        state.error = str(error)
        hide()
        return
    finally:
        evaluated.to_mesh_clear()
    state.job = Job(given, key, owner.name)
    state.job.thread.start()


def tick():
    """The timer that keeps the preview up to date while it is on."""
    try:
        return step(time.monotonic())
    except ReferenceError:
        # The object went while it was being read: looked at again next time.
        state.key = None
        return TICK


def toggled(wm, context):
    """The toggle of the panel: on, the timer starts looking; off, what is
    drawn goes."""
    if wm.openclare_show_hard_edges:
        state.key = None
        if not bpy.app.timers.is_registered(tick):
            bpy.app.timers.register(tick, first_interval=0.0, persistent=True)
    else:
        stop()


def clear():
    """Nothing drawn, nothing waited for. A job under way ends by itself,
    and what it finds is dropped."""
    hide()
    state.job, state.key, state.pending, state.error = None, None, False, None


def stop():
    """Nothing drawn, nothing looked at any more."""
    clear()
    if bpy.app.timers.is_registered(tick):
        bpy.app.timers.unregister(tick)


def show(found):
    """Draws `found`, over the object it was found for."""
    state.shown = found
    if state.handle is None:
        state.handle = bpy.types.SpaceView3D.draw_handler_add(draw, (), "WINDOW", "POST_VIEW")
    redraw()


def hide():
    """Draws nothing any more."""
    state.shown = None
    if state.handle is not None:
        bpy.types.SpaceView3D.draw_handler_remove(state.handle, "WINDOW")
        state.handle = None
        redraw()


def redraw():
    window_manager = getattr(bpy.context, "window_manager", None)
    for window in window_manager.windows if window_manager is not None else ():
        for area in window.screen.areas if window.screen else ():
            if area.type == "VIEW_3D":
                area.tag_redraw()


def draw():
    """Draws the edges shown over their object, in the 3D viewport being
    drawn."""
    shown = state.shown
    if shown is None:
        return
    owner = bpy.data.objects.get(shown.name)
    if owner is None or not owner.visible_get():
        return
    region, view = bpy.context.region, bpy.context.region_data
    draw_lines(shown, owner.matrix_world, (region.width, region.height), view.view_distance)


def draw_lines(shown, matrix_world, viewport_size, view_distance):
    """Draws the edges of `shown`, placed by `matrix_world`, with the view and
    the projection in place, on a viewport of `viewport_size` pixels whose
    eye is `view_distance` from the point it turns round: hidden behind the
    surface, brought a little towards the eye not to sink into it."""
    import gpu
    from gpu_extras.batch import batch_for_shader

    shader = gpu.shader.from_builtin("POLYLINE_UNIFORM_COLOR")
    if shown.batches is None:
        shown.batches = [
            (batch_for_shader(shader, "LINES", {"pos": points}), colour, width)
            for points, colour, width in (
                (shown.left_out_points, LEFT_OUT_COLOUR, LEFT_OUT_WIDTH),
                (shown.kept_points, KEPT_COLOUR, KEPT_WIDTH),
            )
            if len(points)
        ]
    projection = gpu.matrix.get_projection_matrix().copy()
    # Blender's own offset of wires, made the same way: the depth moved
    # towards the eye, by a step that suits an orthographic view, and one
    # that suits a perspective.
    if projection[3][3] > 0.5:
        projection[2][3] -= 0.00001 * OFFSET * view_distance
    else:
        projection[2][3] -= 0.0005 * OFFSET
    gpu.state.depth_test_set("LESS_EQUAL")
    gpu.state.depth_mask_set(False)
    gpu.state.blend_set("ALPHA")
    with gpu.matrix.push_pop_projection():
        gpu.matrix.load_projection_matrix(projection)
        with gpu.matrix.push_pop():
            gpu.matrix.multiply_matrix(matrix_world)
            shader.bind()
            shader.uniform_float("viewportSize", viewport_size)
            for batch, colour, width in shown.batches:
                shader.uniform_float("lineWidth", width)
                shader.uniform_float("color", colour)
                batch.draw(shader)
    gpu.state.blend_set("NONE")
    gpu.state.depth_mask_set(True)
    gpu.state.depth_test_set("NONE")


@bpy.app.handlers.persistent
def shape_changed(scene, depsgraph):
    """Counts the changes of shape of each object: an edit, a modifier."""
    for update in depsgraph.updates:
        if update.is_updated_geometry and isinstance(update.id, bpy.types.Object):
            name = update.id.original.name
            state.generations[name] = state.generations.get(name, 0) + 1


@bpy.app.handlers.persistent
def file_loaded(*_):
    """Another file: what was drawn was of the objects of the one before."""
    clear()
    state.generations = {}
    if is_on() and not bpy.app.timers.is_registered(tick):
        bpy.app.timers.register(tick, first_interval=0.0, persistent=True)


HANDLERS = ((bpy.app.handlers.depsgraph_update_post, shape_changed), (bpy.app.handlers.load_post, file_loaded))


def register():
    bpy.types.WindowManager.openclare_show_hard_edges = bpy.props.BoolProperty(
        name="Show Hard Edges",
        description=(
            "Show over the object in the 3D viewport the edges the quads will follow (solid) and those "
            "sharper than the angle that are left out, too small for the quads asked for (faint). "
            "Nothing is written into the mesh"
        ),
        default=False,
        update=toggled,
    )
    for handlers, handler in HANDLERS:
        handlers.append(handler)


def unregister():
    stop()
    for handlers, handler in HANDLERS:
        if handler in handlers:
            handlers.remove(handler)
    del bpy.types.WindowManager.openclare_show_hard_edges
