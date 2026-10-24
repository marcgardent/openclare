"""What the Hard Edges setting does, shown over the object in the 3D viewport
while a toggle of the panel is on: in solid lines the edges the quads will
follow, sharper than the angle and still sharp at the size of the quads
asked for; in faint lines those sharper than the angle that the engine
leaves out, too small for those quads (the grain of a sculpt).

The engine finds them by the code the remesh finds them with, at the size
of the quads it would make, on the mesh it would be given: the object with
its modifiers. That runs in a thread of its own, once the settings, the
object and its shape have stayed as they are for a moment. The mesh is read
once per shape and handed to an engine finder that keeps what depends on the
shape alone, so that a change of the settings only costs what depends on
them. Nothing is written into the mesh or the file: the toggle belongs to
the window manager, which is not saved, and the lines are drawn, never made.

What is drawn is always said for what it is: while a change waits or is
being computed, the lines drawn are dimmed and the panel shows a turning
ring; once the edges of the latest change are in, the lines are drawn in
full and the panel says how many. A result that comes back for a change
older than the latest is dropped.
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
# Lines of a change that is not computed yet: their opacity multiplied by
# this, and a pixel thinner.
STALE_OPACITY = 0.3
STALE_THINNER = 1.0
# How long the settings and the object stay as they are before the edges are
# found again, in seconds: after a change of shape, which comes in a stream
# while one sculpts or edits and calls for the mesh to be read again; and
# after a change of the settings only, the mesh read already.
QUIET = 0.25
QUIET_SETTINGS = 0.1
# How often the object and its settings are looked at, in seconds: when all
# is up to date, and while a change waits or is computed, when the ring of
# the panel turns and the result is taken as soon as it is in.
TICK = 0.1
BUSY_TICK = 0.05
# A computation that lasts longer than this, in seconds, is said in the
# corner of the 3D viewport too.
SAID_IN_VIEWPORT_AFTER = 0.5
# How long the ring of the panel takes to fill, in seconds.
RING_TURN = 1.0
# How far the lines are brought towards the eye, not to sink into the
# surface they lie on: what Blender brings its own wires forward by.
OFFSET = 1.0

# The settings of an object the edges found depend on: those that set the
# size of the quads, and the angle.
WATCHED = (
    "target", "quads", "largest_quad", "smallest_quad", "adaptive_size", "quads_around",
    "use_hard_edges", "hard_edge_angle", "use_marked_sharp", "density_group", "density_factor",
)

# Where the preview is.
OFF = "off"  # the toggle is off, or no object to show
WAITING = "waiting"  # a change is waited out before computing
COMPUTING = "computing"  # the engine is finding the edges
CURRENT = "current"  # what is drawn is for the object and settings as they are
FAILED = "failed"  # the engine could not find them: the error says why
BUSY = (WAITING, COMPUTING)


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


class Shape:
    """The mesh of an object as the engine was given it, for one shape of the
    object: the finder that keeps what depends on it, where its vertices are,
    to draw the edges at, and its edges marked sharp (None if none), which a
    remesh keeps whatever the settings when asked to."""

    def __init__(self, key, finder, positions, marked):
        self.key, self.finder, self.positions, self.marked = key, finder, positions, marked


class Job:
    """The edges of one object for one change, found by the engine in a
    thread of its own, on what was read beforehand: the thread touches
    nothing of Blender. `shape` is the shape already given to the engine, if
    it is the one; otherwise `given` holds the arrays of the mesh, of which
    the thread makes it."""

    def __init__(self, generation, name, shape_key, shape, given, settings, density, marked):
        import threading

        self.generation, self.name, self.shape_key = generation, name, shape_key
        self.shape, self.given = shape, given
        self.settings, self.density, self.with_marked = settings, density, marked
        self.found = self.error = None
        self.thread = threading.Thread(target=self.work, name="openclare hard edges", daemon=True)

    def work(self):
        from . import BASE_PRESET, engine

        try:
            if self.shape is None:
                given = self.given
                vertices, triangles, corners = given.counts
                finder = engine.HardEdgeFinder(
                    given.addresses[0], vertices, given.addresses[1], triangles, given.addresses[2], corners
                )
                positions = given.kept[0].reshape(-1, 3)
                if given.stretch is not None:
                    # Drawn where the object has them, before its scale.
                    positions = positions / given.stretch
                self.shape = Shape(self.shape_key, finder, positions, given.edges)
                self.given = None
            kept, left_out = self.shape.finder.find(BASE_PRESET, self.settings, self.density)
            if self.with_marked and self.shape.marked is not None:
                kept, left_out = with_marked(kept, left_out, self.shape.marked)
            self.found = Shown(self.name, kept, left_out, self.shape.positions)
        except Exception as error:  # noqa: BLE001
            self.error = error


def with_marked(kept, left_out, marked):
    """The edges kept and left out once those `marked` sharp are kept too:
    all as two vertices per edge."""
    import numpy

    def keys(edges):
        pairs = numpy.sort(edges.reshape(-1, 2).astype(numpy.int64), axis=1)
        return pairs[:, 0] << 32 | pairs[:, 1]

    known = keys(kept)
    added = marked.reshape(-1, 2)[~numpy.isin(keys(marked), known)]
    still_out = left_out.reshape(-1, 2)[~numpy.isin(keys(left_out), keys(marked))]
    return numpy.concatenate([kept, added.ravel()]).astype(numpy.uint32), still_out.ravel().astype(numpy.uint32)


class State:
    """Where the preview is: its status, what is drawn and whether it is
    out of date, the draw handlers, the object and settings last seen and
    since when, the change waited for, the job under way and the shape last
    given to the engine."""

    def __init__(self):
        self.status = OFF
        self.shown = self.handle = self.text_handle = self.job = self.shape = None
        self.stale = False
        self.key, self.changed_at, self.pending = None, 0.0, False
        # One more for each change seen: a job is for the change it was
        # started for, and what it finds is drawn only if no change came
        # after.
        self.generation = 0
        # Since when the preview is busy, waiting or computing.
        self.busy_since = None
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


def shape_key_of(owner):
    """Which object, and which of its shapes: a scale that differs along the
    axes is applied to the mesh the engine is given, and makes another."""
    from . import stretch_of

    stretch = stretch_of(owner)
    shape = state.generations.get(owner.name, 0), None if stretch is None else tuple(stretch)
    return owner.name, owner.as_pointer(), shape


def key_of(owner):
    """What the edges of `owner` depend on: which object, its shape, and its
    settings; its scale too where sizes are asked for, in the units of the
    scene."""
    settings = owner.openclare
    key = shape_key_of(owner) + tuple(getattr(settings, name) for name in WATCHED)
    if settings.target == "SIZE":
        key += tuple(owner.matrix_world.to_scale())
    return key


def step(now):
    """One look at the object and its settings, at time `now` (in seconds):
    a change is waited out for a moment, then the edges are found again in
    the background, and drawn once found. Returns when to look again, or
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
        state.generation += 1
        if key is None:
            # No object to show: the shape given to the engine goes with
            # what it holds.
            state.shape = None
            set_status(OFF, now)
        else:
            set_status(WAITING, now)
            make_stale()
    job = state.job
    if job is not None:
        if job.thread.is_alive():
            return keep_turning(now)
        state.job = None
        if job.shape is not None and key is not None and job.shape.key == key[:3]:
            # The mesh the job gave the engine is still the object's: kept,
            # whatever the job was for.
            state.shape = job.shape
        if job.generation == state.generation:
            if job.error is not None:
                state.error = str(job.error)
                hide()
                set_status(FAILED, now)
            else:
                state.error = None
                show(job.found)
                set_status(CURRENT, now)
        # Otherwise for a change older than the latest: dropped, the latest
        # is pending.
    if state.pending and state.job is None:
        quiet = QUIET_SETTINGS if state.shape is not None and state.shape.key == key[:3] else QUIET
        if now - state.changed_at >= quiet:
            state.pending = False
            start(owner, key, now)
    if state.status in BUSY:
        return keep_turning(now)
    return TICK


def keep_turning(now):
    """While busy: the ring of the panel turned, the viewport told of a long
    computation. Returns when to look again."""
    redraw("PROPERTIES")
    if state.text_handle is None and state.busy_since is not None and now - state.busy_since >= SAID_IN_VIEWPORT_AFTER:
        state.text_handle = bpy.types.SpaceView3D.draw_handler_add(draw_text, (), "WINDOW", "POST_PIXEL")
        redraw("VIEW_3D")
    return BUSY_TICK


def set_status(status, now):
    """Changes the status, and what shows it: the panel, and once idle, the
    text of the viewport goes."""
    was_busy = state.status in BUSY
    state.status = status
    if status in BUSY:
        if not was_busy:
            state.busy_since = now
    else:
        state.busy_since = None
        if state.text_handle is not None:
            bpy.types.SpaceView3D.draw_handler_remove(state.text_handle, "WINDOW")
            state.text_handle = None
            redraw("VIEW_3D")
    redraw("PROPERTIES")


def start(owner, key, now):
    """Has the engine find the edges of `owner` in a thread of its own: on the
    shape it was given already if that is still the object's; otherwise on the
    object with its modifiers, read as the remesh reads it."""
    from . import BASE_PRESET, engine, engine_input, engine_settings_of, stretch_of

    shape = state.shape if state.shape is not None and state.shape.key == key[:3] else None
    if shape is None:
        # The shape before is of no more use: its memory goes.
        state.shape = None
    evaluated = owner.evaluated_get(bpy.context.evaluated_depsgraph_get())
    # The mesh is read for a new shape, or for the weights of the density
    # painted on it.
    source = evaluated.to_mesh() if shape is None or owner.openclare.density_group else None
    try:
        engine_settings, density = engine_settings_of(owner, evaluated, source, write=False)
        given = None if shape is not None else engine_input(
            source, BASE_PRESET, engine_settings, density, copy=True, sharp=True, stretch=stretch_of(owner)
        )
    except engine.EngineError as error:
        state.error = str(error)
        hide()
        set_status(FAILED, now)
        return
    finally:
        if source is not None:
            evaluated.to_mesh_clear()
    marked = owner.openclare.use_marked_sharp
    state.job = Job(state.generation, owner.name, key[:3], shape, given, engine_settings, density, marked)
    state.job.thread.start()
    set_status(COMPUTING, now)


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
    """Nothing drawn, nothing waited for, no shape kept. A job under way ends
    by itself, and what it finds is dropped."""
    hide()
    state.job, state.key, state.pending, state.error, state.shape = None, None, False, None, None
    state.generation += 1
    set_status(OFF, 0.0)


def stop():
    """Nothing drawn, nothing looked at any more."""
    clear()
    if bpy.app.timers.is_registered(tick):
        bpy.app.timers.unregister(tick)


def show(found):
    """Draws `found`, over the object it was found for, in full."""
    state.shown, state.stale = found, False
    if state.handle is None:
        state.handle = bpy.types.SpaceView3D.draw_handler_add(draw, (), "WINDOW", "POST_VIEW")
    redraw("VIEW_3D")


def make_stale():
    """What is drawn is no longer for the object and settings as they are:
    dimmed until the edges of the latest change are in."""
    if state.shown is not None and not state.stale:
        state.stale = True
        redraw("VIEW_3D")


def hide():
    """Draws nothing any more."""
    state.shown, state.stale = None, False
    if state.handle is not None:
        bpy.types.SpaceView3D.draw_handler_remove(state.handle, "WINDOW")
        state.handle = None
        redraw("VIEW_3D")


def redraw(kind):
    """Has the areas of `kind` ("VIEW_3D", "PROPERTIES") drawn again."""
    window_manager = getattr(bpy.context, "window_manager", None)
    for window in window_manager.windows if window_manager is not None else ():
        for area in window.screen.areas if window.screen else ():
            if area.type == kind:
                area.tag_redraw()


def ring_factor():
    """How full the ring of the panel is drawn now: it fills, and starts
    again, while the preview is busy."""
    return (time.monotonic() / RING_TURN) % 1.0


def status_text():
    """What the panel says of the preview, short."""
    if state.status in BUSY:
        return "Computing…"
    if state.status == FAILED:
        return state.error or "The hard edges could not be found"
    if state.status == CURRENT and state.shown is not None:
        kept, left_out = len(state.shown.kept) // 2, len(state.shown.left_out) // 2
        return f"{kept:,} followed, {left_out:,} left out"
    return ""


def is_about(owner):
    """Whether the status is that of `owner`."""
    return state.key is not None and owner is not None and state.key[0] == owner.name


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
    draw_lines(shown, owner.matrix_world, (region.width, region.height), view.view_distance, state.stale)


def draw_text():
    """Says in a corner of the 3D viewport being drawn that the edges drawn
    are being computed again."""
    import blf

    region = bpy.context.region
    if region is None:
        return
    # Clear of the toolbar, when the regions are drawn over the viewport.
    left = 0
    if bpy.context.preferences.system.use_region_overlap:
        for other in bpy.context.area.regions:
            if other.type == "TOOLS" and other.width > 1:
                left = other.width
    scale = bpy.context.preferences.system.ui_scale
    font = 0
    blf.size(font, 11.0 * scale)
    blf.color(font, 1.0, 1.0, 1.0, 0.6)
    blf.position(font, left + 12.0 * scale, 12.0 * scale, 0.0)
    blf.draw(font, "openclare: computing hard edges…")


def colours(stale):
    """The colour and the width of the edges left out, then of those kept:
    in full, or dimmed when `stale`."""
    if not stale:
        return (LEFT_OUT_COLOUR, LEFT_OUT_WIDTH), (KEPT_COLOUR, KEPT_WIDTH)
    dimmed = lambda colour: colour[:3] + (colour[3] * STALE_OPACITY,)  # noqa: E731
    return (
        (dimmed(LEFT_OUT_COLOUR), LEFT_OUT_WIDTH - STALE_THINNER),
        (dimmed(KEPT_COLOUR), KEPT_WIDTH - STALE_THINNER),
    )


def draw_lines(shown, matrix_world, viewport_size, view_distance, stale=False):
    """Draws the edges of `shown`, placed by `matrix_world`, with the view and
    the projection in place, on a viewport of `viewport_size` pixels whose
    eye is `view_distance` from the point it turns round: hidden behind the
    surface, brought a little towards the eye not to sink into it; dimmed if
    `stale`."""
    import gpu
    from gpu_extras.batch import batch_for_shader

    shader = gpu.shader.from_builtin("POLYLINE_UNIFORM_COLOR")
    if shown.batches is None:
        # Left out first, kept over them.
        shown.batches = [
            batch_for_shader(shader, "LINES", {"pos": points}) if len(points) else None
            for points in (shown.left_out_points, shown.kept_points)
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
            for batch, (colour, width) in zip(shown.batches, colours(stale)):
                if batch is None:
                    continue
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
