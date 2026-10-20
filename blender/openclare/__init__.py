"""openclare: automatic quad retopology, as a Blender add-on (Community
edition).

A panel in the Properties editor, on the tab of the mesh of an object
(Object Data), remeshes the object into quads with the settings it keeps:
a number of quads, and how they follow the shape, which presets set. The
result is a new object, next to the original, which is left as it is;
shown, the same panel says what it was made from and how, and remeshes it
again.
"""

import math
import os

import bpy
from bl_operators.presets import AddPresetBase
from bl_ui.utils import PresetPanel

from . import engine

PRESETS = [
    ("organic", "Organic", "Sculpts and characters: edges follow the shape closely, quads are smaller where it bends"),
    ("animation", "Animation", "Meshes meant to deform: fewer irregular vertices, at some cost in fidelity"),
    ("hard-surface", "Hard surface", "Mechanical parts: edges sharper than 30 degrees are kept exactly"),
]
# The shapes the add-on ships, as the values of the settings they set: what
# the engine's presets of the same names are. The preset files of the add-on
# (presets/openclare_shape) say the same; files of 0.1, which named one of
# these, are given its values.
SHAPES = {
    "organic": dict(adaptive_size=50.0, quads_around=6, use_hard_edges=False, hard_edge_angle=math.radians(30.0),
                    follow_curvature=0.05, fewer_irregular=2.0, volume_fit=1.0),
    "animation": dict(adaptive_size=50.0, quads_around=6, use_hard_edges=False, hard_edge_angle=math.radians(30.0),
                      follow_curvature=0.05, fewer_irregular=3.0, volume_fit=1.0),
    "hard-surface": dict(adaptive_size=0.0, quads_around=0, use_hard_edges=True, hard_edge_angle=math.radians(30.0),
                         follow_curvature=0.02, fewer_irregular=2.0, volume_fit=1.0),
}
# The settings a preset of the shape holds, in the order of the panel.
SHAPE_NAMES = list(SHAPES["organic"])
# Where the presets of the shape are, under presets/: those of the add-on,
# which cannot be removed, and those saved in Blender's own folder.
SHAPE_PRESETS = "openclare_shape"
# What the engine is given under these values: the shapes are said in full.
BASE_PRESET = "organic"


# What the engine said of the last run of the operator, name by name: for a
# script that drives the add-on (bench/blender_run.py). Empty until a run
# ends well.
last_report = {}


def is_a_mesh(settings, candidate):
    return candidate.type == "MESH"


def shown_object(context):
    """The object the panel and its buttons are about: the one the
    Properties editor shows, or the active one where there is no such
    editor."""
    return getattr(context, "object", None) or context.active_object


def object_to_remesh(context):
    """The object the panel is about, if it is a mesh; None otherwise."""
    chosen = shown_object(context)
    return chosen if chosen is not None and chosen.type == "MESH" else None


def source_of(owner):
    """What `owner` was remeshed from, if it is a result of the add-on whose
    original is still there; None otherwise."""
    source = getattr(owner, "openclare_source", None) if owner is not None else None
    return source if source is not None and source.type == "MESH" else None


# Settings that are not copied from one object to another: the object files
# of 0.1 named in the settings of their scene.
NOT_COPIED = {"rna_type", "name", "source"}


def copy_settings(given, taken):
    """Gives `taken` the settings of `given`. The preset named by settings
    of 0.1 goes with them, and is turned into its values."""
    names = [p.identifier for p in given.bl_rna.properties if p.identifier not in NOT_COPIED]
    for name in names:
        if name != "preset" or given.is_property_set("preset"):
            setattr(taken, name, getattr(given, name))
    adopt_preset(taken)


def adopt_scene_settings(scene):
    """Files of 0.1 kept one set of settings in the scene: the object it
    named, or failing that each mesh object that has none of its own and is
    not a result, takes them. Done once: the scene then keeps nothing."""
    legacy = scene.openclare
    names = [p.identifier for p in legacy.bl_rna.properties if p.identifier not in {"rna_type", "name"}]
    if not any(legacy.is_property_set(name) for name in names):
        return
    named = legacy.source if legacy.source is not None and legacy.source.type == "MESH" else None
    takers = [named] if named is not None else [
        candidate for candidate in scene.objects
        if candidate.type == "MESH" and source_of(candidate) is None
        and not any(candidate.openclare.is_property_set(name) for name in names)
    ]
    for taker in takers:
        copy_settings(legacy, taker.openclare)
    if named is not None and scene.view_layers:
        # What was remeshed whichever object was active: made so.
        scene.view_layers[0].objects.active = named
    for name in names:
        legacy.property_unset(name)


def shape_of(settings):
    """The settings of the shape as the engine takes them."""
    return {
        "adaptivity": round(settings.adaptive_size / 100.0, 6),
        "quads_around": settings.quads_around,
        "edge_angle": round(math.degrees(settings.hard_edge_angle), 4) if settings.use_hard_edges else "none",
        "alignment": round(settings.follow_curvature, 6),
        "singularity_reach": round(settings.fewer_irregular, 6),
        "volume_fit": round(settings.volume_fit, 6),
    }


def adopt_preset(settings):
    """Settings of 0.1 that named a preset: given the values of its shape,
    but for the adaptive size, which they kept of their own; the name is
    then forgotten."""
    if not settings.is_property_set("preset"):
        return
    for name, value in SHAPES[settings.preset].items():
        if name != "adaptive_size":
            setattr(settings, name, value)
    settings.property_unset("preset")


@bpy.app.handlers.persistent
def after_loading(*_):
    """Another file: the settings it kept as 0.1 did are given to its
    objects."""
    for scene in bpy.data.scenes:
        adopt_scene_settings(scene)
    for owner in bpy.data.objects:
        adopt_preset(owner.openclare)


class OpenclareSettings(bpy.types.PropertyGroup):
    source: bpy.props.PointerProperty(
        options=set(),
        name="Object",
        type=bpy.types.Object,
        poll=is_a_mesh,
        description=(
            "Read only in the settings of a scene saved by openclare 0.1: the object those settings are given to"
        ),
    )
    # Read only in files of 0.1, which named a preset of the engine: turned
    # into the values of the shape it stood for (see `adopt_preset`).
    preset: bpy.props.EnumProperty(
        options=set(),
        name="Preset", description="The preset of files saved by openclare 0.1", items=PRESETS, default="organic"
    )
    quads: bpy.props.IntProperty(
        options=set(),
        name="Quads",
        description="Number of quads asked for: the shape says where they are smaller, and the number is kept",
        default=3000,
        min=1,
        soft_min=100,
        soft_max=50_000,
    )
    adaptive_size: bpy.props.FloatProperty(
        options=set(),
        name="Adaptive Size",
        description=(
            "How much the size of the quads follows the curvature: at 0 % they are all the same size; "
            "towards 100 % they get smaller where the shape bends and larger where it is flat, "
            "for the same number in total"
        ),
        subtype="PERCENTAGE",
        default=50.0,
        min=0.0,
        max=100.0,
    )
    quads_around: bpy.props.IntProperty(
        options=set(),
        name="Quads Around Thin Parts",
        description=(
            "Fewest quads that go round a thin part, a finger or the tip of an ear: smaller quads there and "
            "larger elsewhere, for the part to keep its volume instead of coming out flat. 0 does not look "
            "at thickness"
        ),
        default=6,
        min=0,
        soft_max=16,
    )
    use_hard_edges: bpy.props.BoolProperty(
        options=set(),
        name="Hard Edges",
        description=(
            "Edges of the object sharper than the angle are followed by the quads wherever they can, and "
            "kept crisp: for mechanical parts"
        ),
        default=False,
    )
    hard_edge_angle: bpy.props.FloatProperty(
        options=set(),
        name="Angle",
        description="How sharp an edge of the object must be to be followed",
        subtype="ANGLE",
        default=math.radians(30.0),
        min=0.0,
        max=math.pi,
    )
    follow_curvature: bpy.props.FloatProperty(
        options=set(),
        name="Follow Curvature",
        description=(
            "How strongly the edges are drawn along the directions in which the shape bends, against how "
            "smoothly they flow. 0 ignores the curvature; higher follows the shape more closely, with more "
            "irregular vertices"
        ),
        default=0.05,
        min=0.0,
        soft_max=0.2,
        step=0.1,
        precision=3,
    )
    fewer_irregular: bpy.props.FloatProperty(
        options=set(),
        name="Fewer Irregular Vertices",
        description=(
            "Irregular vertices closer than this many quads to one of the opposite kind are cancelled with "
            "it. 0 keeps them all; higher gives fewer, and edges that follow the shape a little less: for "
            "meshes meant to deform"
        ),
        default=2.0,
        min=0.0,
        soft_max=6.0,
        step=50,
        precision=1,
    )
    volume_fit: bpy.props.FloatProperty(
        options=set(),
        name="Volume Fit",
        description=(
            "How far the vertices leave the surface, at the very end, for the mesh to keep the volume of the "
            "shape. 0 leaves them on the surface, where the faces run under it wherever it bulges; 1 puts "
            "as much of the mesh above the surface as below"
        ),
        subtype="FACTOR",
        default=1.0,
        min=0.0,
        max=1.0,
    )


def in_place(mesh):
    """Whether the arrays of `mesh` sit in memory the way the engine reads
    them: packed floats for the positions, packed integers for the corners.
    True of Blender since 4.0; checked here on a few values rather than
    assumed, since nothing promises it."""
    import ctypes

    def at(pointer, kind, index):
        return ctypes.cast(pointer, ctypes.POINTER(kind))[index]

    last = len(mesh.vertices) - 1
    positions = mesh.vertices[0].as_pointer()
    for index in {0, last // 2, last}:
        if any(at(positions, ctypes.c_float, 3 * index + k) != mesh.vertices[index].co[k] for k in range(3)):
            return False
    corner_vertices = mesh.loops[0].as_pointer()
    last = len(mesh.loops) - 1
    if any(at(corner_vertices, ctypes.c_int32, i) != mesh.loops[i].vertex_index for i in {0, last // 2, last}):
        return False
    triangles = mesh.loop_triangles[0].as_pointer()
    last = len(mesh.loop_triangles) - 1
    return all(
        at(triangles, ctypes.c_int32, 3 * i + k) == mesh.loop_triangles[i].loops[k]
        for i in {0, last // 2, last}
        for k in range(3)
    )


def quads_of(source, preset, settings, on_stage=None):
    """A new mesh datablock: `source`, an evaluated mesh, remeshed into quads
    with the engine's `settings` on top of `preset`, and the report of the
    run. `on_stage`, if given, is told the name of each stage as it begins.
    The three steps, for a run that does not wait: `engine_input`,
    `engine_run`, `mesh_of`."""
    return mesh_of(engine_run(engine_input(source, preset, settings), on_stage))


class EngineInput:
    """What a run of the engine is given, read from Blender beforehand: the
    run itself touches nothing of Blender, and can go on in a thread of its
    own when the arrays are copies."""


def engine_input(source, preset, settings, copy=False):
    """The `EngineInput` of `quads_of`. With `copy`, the arrays of `source`
    are copied out, so that the run does not depend on `source` living as
    long as it does; otherwise they are read in place where they can be."""
    import numpy

    source.calc_loop_triangles()
    vertices, corners, triangles = len(source.vertices), len(source.loops), len(source.loop_triangles)
    if triangles == 0:
        raise engine.EngineError("the object has no faces")
    if not copy and in_place(source):
        # Blender's own arrays, read where they are: nothing is copied here.
        addresses = (
            source.vertices[0].as_pointer(), source.loop_triangles[0].as_pointer(), source.loops[0].as_pointer()
        )
        kept = None
    else:
        # A Blender that lays its meshes out otherwise: copy them out the
        # documented way.
        kept = (
            numpy.empty(3 * vertices, numpy.float32),
            numpy.empty(3 * triangles, numpy.int32),
            numpy.empty(corners, numpy.int32),
        )
        source.vertices.foreach_get("co", kept[0])
        source.loop_triangles.foreach_get("loops", kept[1])
        source.loops.foreach_get("vertex_index", kept[2])
        addresses = tuple(array.ctypes.data for array in kept)
    given = EngineInput()
    given.counts = (vertices, triangles, corners)
    given.addresses, given.kept = addresses, kept
    given.preset, given.settings = preset, settings
    return given


def engine_run(given, on_stage=None):
    """Runs the engine on an `EngineInput`; `on_stage` as in `quads_of`.
    Nothing of Blender is touched: with copied arrays, this can run in a
    thread of its own."""
    vertices, triangles, corners = given.counts
    return engine.remesh(
        given.addresses[0], vertices, given.addresses[1], triangles, given.addresses[2], corners, given.preset,
        given.settings, on_stage=on_stage,
    )


def mesh_of(made):
    """The mesh datablock of what a run made, and the report of the run."""
    import numpy

    mesh = bpy.data.meshes.new("openclare")
    mesh.vertices.add(len(made.positions) // 3)
    mesh.vertices.foreach_set("co", made.positions)
    mesh.loops.add(len(made.corners))
    mesh.loops.foreach_set("vertex_index", made.corners.astype(numpy.int32))
    mesh.polygons.add(len(made.sizes))
    starts = numpy.zeros(len(made.sizes), numpy.int32)
    numpy.cumsum(made.sizes[:-1], out=starts[1:])
    mesh.polygons.foreach_set("loop_start", starts)
    mesh.polygons.foreach_set("loop_total", made.sizes.astype(numpy.int32))
    mesh.update(calc_edges=True)
    mesh.validate()
    return mesh, made.report


def worth_a_line(message):
    """Whether a stage goes to the Info log as well as to the status bar. The
    stages of a model in a few pieces all do; of hundreds of pieces, only
    the start of one piece now and then, not to flood the log."""
    import re

    piece = re.match(r"piece (\d+) of (\d+): (.*)", message)
    if piece is None:
        return True
    index, count, stage = int(piece[1]), int(piece[2]), piece[3]
    if count <= 5:
        return True
    return stage == "preprocessing" and (index - 1) % max(count // 20, 1) == 0


# Stages of a run on a model in one piece, for the progress shown.
STAGES = 9

# The custom property of a result where what the engine said of it is kept.
REPORT = "openclare_report"


class Remeshing:
    """What the two buttons that remesh have in common: the run, and what
    is said of it while it goes.

    Clicked in Blender, the engine runs in a thread of its own and Blender
    stays free: the panel shows a bar, Esc stops the run. From a script or
    in the background, the operator waits for the engine."""

    def say(self, context, message, done=0):
        """Shows where the run is: in the status bar and in the Info log, and
        on the cursor as stages out of `STAGES`. Blender is busy in this
        operator and would not redraw by itself, so it is asked to."""
        import time

        context.workspace.status_text_set(f"openclare: {message}...")
        context.window_manager.progress_update(done)
        if worth_a_line(message):
            self.report({"INFO"}, f"openclare: {message}")
        now = time.monotonic()
        if not bpy.app.background and now - self.drawn > 0.05:
            self.drawn = now
            try:
                bpy.ops.wm.redraw_timer(type="DRAW_WIN_SWAP", iterations=1)
            except RuntimeError:
                pass

    def begin(self, context, original, copy):
        """Reads `original` and its settings into what the engine is given.
        Returns it with a function to call once the engine is done with the
        evaluated mesh (at once with `copy`), or None if it could not be
        read, which is reported."""
        adopt_scene_settings(context.scene)
        settings = original.openclare
        adopt_preset(settings)
        last_report.clear()
        # With its modifiers applied: what is seen is what is remeshed.
        evaluated = original.evaluated_get(context.evaluated_depsgraph_get())
        source = evaluated.to_mesh()
        try:
            given = engine_input(source, BASE_PRESET, {"quads": settings.quads, **shape_of(settings)}, copy)
        except engine.EngineError as error:
            evaluated.to_mesh_clear()
            self.report({"ERROR"}, f"openclare: {error}")
            return None
        if copy:
            evaluated.to_mesh_clear()
            return given, lambda: None
        return given, evaluated.to_mesh_clear

    def run(self, context, original):
        """Remeshes `original` with its settings into a new object next to
        it, made the active one, and says how it went, waiting for the
        engine. Returns the new object, or None if it could not be made."""
        self.drawn, self.stages = 0.0, []
        context.window_manager.progress_begin(0, STAGES)
        self.say(context, f"reading '{original.name}'")

        def on_stage(stage):
            self.stages.append(stage)
            self.say(context, stage, min(len(self.stages), STAGES))
            return False

        try:
            begun = self.begin(context, original, copy=False)
            if begun is None:
                return None
            given, release = begun
            try:
                made = engine_run(given, on_stage)
            except engine.EngineError as error:
                self.report({"ERROR"}, f"openclare: {error}")
                return None
            finally:
                release()
        finally:
            context.window_manager.progress_end()
            context.workspace.status_text_set(None)
        return self.finish(context, original, made)

    def finish(self, context, original, made):
        """Makes the object of what the engine `made` from `original`, next
        to it and the active one, and says how it went. Returns it."""
        mesh, report = mesh_of(made)
        mesh.name = f"{original.data.name}.quads"
        remeshed = bpy.data.objects.new(f"{original.name}.quads", mesh)
        remeshed.matrix_world = original.matrix_world
        # Kept, for the result to be remeshed again and its original found.
        remeshed.openclare_source = original
        # And how it went, for the panel of the result to tell.
        remeshed[REPORT] = {key: str(value) for key, value in report.items()}
        for collection in original.users_collection or [context.collection]:
            collection.objects.link(remeshed)
        for other in context.selected_objects:
            other.select_set(False)
        remeshed.select_set(True)
        context.view_layer.objects.active = remeshed

        last_report.update(report)
        seconds = float(report.get("seconds", 0.0))
        said = f"openclare: {report.get('quads', '?')} quads in {seconds:.1f} s"
        welded, apart = int(report.get("welded_vertices", 0)), int(report.get("edges_taken_apart", 0))
        if welded or apart:
            # Said, because the result then has borders the object did not show.
            said += f"; the mesh had defects and was mended ({welded} vertices welded along torn borders"
            said += f", {apart} edges with more than two faces taken apart)"
        thin, moved = int(report.get("thin_pieces", 0)), int(report.get("quads_for_thin_pieces", 0))
        if thin:
            said += f"; {moved} of them moved to {thin} thin part(s), to keep their shape"
        boxed = int(report.get("pieces_boxed", 0))
        if boxed:
            said += f"; {boxed} part(s) smaller than a quad could not be meshed and are boxes"
        # Said as a warning: the result then lacks part of the object.
        failed = int(report.get("pieces_failed", 0))
        if failed:
            said += f"; {failed} of {report.get('pieces', '?')} pieces could not be meshed and are missing"
        self.report({"WARNING"} if failed else {"INFO"}, said)
        return remeshed


class Run:
    """A run of the engine going on in a thread of its own. The thread only
    calls the engine; what it says of its stages is queued, and read on
    Blender's main thread, the only one that may touch Blender."""

    def __init__(self, given):
        import queue
        import threading

        self.given = given
        self.told = queue.SimpleQueue()
        self.stop = threading.Event()
        self.made = self.error = None
        # Read on the main thread, for the panel: where the run is.
        self.stages, self.stage, self.done = [], "starting", 0.0
        self.thread = threading.Thread(target=self.work, name="openclare", daemon=True)

    def work(self):
        try:
            self.made = engine_run(self.given, self.on_stage)
        except Exception as error:  # noqa: BLE001
            self.error = error

    def on_stage(self, stage):
        self.told.put(stage)
        return self.stop.is_set()

    def news(self):
        """The stages begun since last asked, in order; `stage` and `done`
        are brought up to date."""
        import queue

        new = []
        while True:
            try:
                new.append(self.told.get_nowait())
            except queue.Empty:
                break
        if new:
            self.stages += new
            self.stage, self.done = new[-1], progress_of(self.stages)
        return new


def progress_of(stages):
    """How far a run is, from 0 to 1: the stages begun out of `STAGES` for
    each piece. The pieces are meshed at the same time, so the count, not
    the piece last heard of, says how far it is."""
    import re

    if not stages:
        return 0.0
    piece = re.match(r"piece \d+ of (\d+): ", stages[-1])
    count = int(piece[1]) if piece else 1
    return min(len(stages) / (count * STAGES), 1.0)


# The run going on in the background, if any: one at a time.
RUNNING = None


def redraw_panels():
    """The Properties editors redrawn, for the bar of the panel to move."""
    for window in bpy.context.window_manager.windows:
        for area in window.screen.areas if window.screen else ():
            if area.type == "PROPERTIES":
                area.tag_redraw()


class Background:
    """The operators that remesh, run without waiting for the engine when
    clicked: `target` is the object to remesh, `done` what follows once its
    result is made."""

    def invoke(self, context, event):
        global RUNNING
        if bpy.app.background:
            return self.execute(context)
        try:
            engine.library()
        except engine.EngineError as error:
            self.report({"ERROR"}, f"openclare: {error}")
            return {"CANCELLED"}
        original = self.target(context)
        begun = self.begin(context, original, copy=True)
        if begun is None:
            return {"CANCELLED"}
        self.original, (given, _) = original, begun
        self.job = RUNNING = Run(given)
        self.workspace = context.workspace
        self.workspace.status_text_set(f"openclare: remeshing '{original.name}'... (Esc to stop)")
        manager = context.window_manager
        self.timer = manager.event_timer_add(0.1, window=context.window)
        manager.modal_handler_add(self)
        self.job.thread.start()
        redraw_panels()
        return {"RUNNING_MODAL"}

    def modal(self, context, event):
        job = self.job
        if event.type == "ESC" and event.value == "PRESS":
            job.stop.set()
            self.workspace.status_text_set("openclare: stopping...")
            return {"RUNNING_MODAL"}
        # Read at each tick of a timer, ours or another's; every other event
        # goes on to Blender, which stays free.
        if event.type != "TIMER":
            return {"PASS_THROUGH"}
        for stage in job.news():
            if not job.stop.is_set():
                self.workspace.status_text_set(f"openclare: {stage}... (Esc to stop)")
            if worth_a_line(stage):
                self.report({"INFO"}, f"openclare: {stage}")
        redraw_panels()
        if job.thread.is_alive():
            return {"PASS_THROUGH"}
        self.end(context)
        if job.stop.is_set():
            self.report({"WARNING"}, "openclare: stopped, nothing made")
            return {"CANCELLED"}
        if job.error is not None:
            self.report({"ERROR"}, f"openclare: {job.error}")
            return {"CANCELLED"}
        try:
            made = self.finish(context, self.original, job.made)
        except ReferenceError:
            self.report({"ERROR"}, "openclare: the object was removed while it was being remeshed")
            return {"CANCELLED"}
        return self.done(context, made)

    def cancel(self, context):
        # Blender ends the operator (a file loaded, Blender closed): the
        # engine is asked to stop, and what it makes is dropped.
        self.job.stop.set()
        self.end(context)

    def end(self, context):
        global RUNNING
        context.window_manager.event_timer_remove(self.timer)
        RUNNING = None
        try:
            self.workspace.status_text_set(None)
        except ReferenceError:
            pass
        redraw_panels()

    def execute(self, context):
        return self.done(context, self.run(context, self.target(context)))


class OPENCLARE_OT_stop(bpy.types.Operator):
    """Stop the remeshing under way: nothing is made"""

    bl_idname = "object.openclare_stop"
    bl_label = "Stop"

    @classmethod
    def poll(cls, context):
        return RUNNING is not None and not RUNNING.stop.is_set()

    def execute(self, context):
        RUNNING.stop.set()
        return {"FINISHED"}


def draw_remesh(layout, operator, icon):
    """The button that remeshes, or, while a run goes on, how far it is and
    a button to stop it."""
    if RUNNING is None:
        layout.operator(operator, icon=icon)
        return
    row = layout.row(align=True)
    text = "Stopping..." if RUNNING.stop.is_set() else RUNNING.stage.capitalize()
    row.progress(factor=RUNNING.done, type="BAR", text=text)
    row.operator(OPENCLARE_OT_stop.bl_idname, text="", icon="CANCEL")


class OPENCLARE_OT_remesh(Background, Remeshing, bpy.types.Operator):
    """Remesh the object into quads, as a new object next to it"""

    bl_idname = "object.openclare_remesh"
    bl_label = "Remesh to Quads"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        return object_to_remesh(context) is not None and context.mode == "OBJECT" and RUNNING is None

    def target(self, context):
        return object_to_remesh(context)

    def done(self, context, made):
        return {"FINISHED"} if made is not None else {"CANCELLED"}


class OPENCLARE_OT_remesh_again(Background, Remeshing, bpy.types.Operator):
    """Remesh the original of this result again, with the settings it has now, in place of this result"""

    bl_idname = "object.openclare_remesh_again"
    bl_label = "Remesh Again"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        return source_of(object_to_remesh(context)) is not None and context.mode == "OBJECT" and RUNNING is None

    def target(self, context):
        self.result = object_to_remesh(context)
        return source_of(self.result)

    def done(self, context, made):
        if made is None:
            return {"CANCELLED"}
        result = self.result
        try:
            result.data
        except ReferenceError:
            # The result was removed during the run: the new one stays.
            return {"FINISHED"}
        # The new mesh goes into the result, which keeps its name, place,
        # settings and what else it was given; the new object goes.
        previous, mesh = result.data, made.data
        result.data = mesh
        result[REPORT] = made[REPORT]
        bpy.data.objects.remove(made)
        if previous.users == 0:
            name = previous.name
            bpy.data.meshes.remove(previous)
            mesh.name = name
        for other in context.selected_objects:
            other.select_set(False)
        result.select_set(True)
        context.view_layer.objects.active = result
        return {"FINISHED"}


class DataPanel:
    """Where the panels are: in the Properties editor, on the tab of the
    mesh of the object."""

    bl_space_type = "PROPERTIES"
    bl_region_type = "WINDOW"
    bl_context = "data"


def is_source(context):
    """The object shown is one to remesh: a mesh that is not a result."""
    owner = object_to_remesh(context)
    return owner is not None and source_of(owner) is None


class OPENCLARE_PT_panel(DataPanel, bpy.types.Panel):
    """For an object to remesh: how; for a result: what it was made from
    and how it went"""

    bl_label = "openclare"
    bl_idname = "OPENCLARE_PT_panel"
    # Before Animation and Custom Properties. Blender keeps the order a file
    # saved for the panels it knew, and puts the others after: dragged up
    # by its handle, the panel stays there.
    bl_order = -1

    @classmethod
    def poll(cls, context):
        return object_to_remesh(context) is not None

    def draw_header(self, context):
        self.layout.label(text="", icon="MOD_REMESH")

    def draw(self, context):
        owner = object_to_remesh(context)
        if source_of(owner) is None:
            self.draw_source(context, owner)
        else:
            self.draw_result(context, owner)

    def draw_source(self, context, owner):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False
        layout.prop(owner.openclare, "quads")
        draw_remesh(layout, OPENCLARE_OT_remesh.bl_idname, "MOD_REMESH")

    def draw_result(self, context, owner):
        layout = self.layout
        source = source_of(owner)
        row = layout.row(align=True)
        row.label(text=f"Remeshed from {source.name}", icon="MESH_DATA")
        row.operator(OPENCLARE_OT_select_original.bl_idname, text="", icon="RESTRICT_SELECT_OFF")
        told = owner.get(REPORT)
        if told:
            box = layout.box().column(align=True)
            for line, icon in report_lines(told):
                box.label(text=line, icon=icon)
        draw_remesh(layout, OPENCLARE_OT_remesh_again.bl_idname, "FILE_REFRESH")


def report_lines(told):
    """What the panel of a result says of how it was made: lines of text,
    each with its icon."""
    told = {key: told[key] for key in told.keys()}
    lines = []
    quads, seconds = told.get("quads", "?"), float(told.get("seconds", 0.0))
    lines.append((f"{quads} quads in {seconds:.1f} s", "MESH_GRID"))
    failed, boxed = int(told.get("pieces_failed", 0)), int(told.get("pieces_boxed", 0))
    if failed:
        lines.append((f"{failed} of {told.get('pieces', '?')} pieces could not be meshed", "ERROR"))
    if boxed:
        lines.append((f"{boxed} part(s) smaller than a quad are boxes", "ERROR"))
    welded, apart = int(told.get("welded_vertices", 0)), int(told.get("edges_taken_apart", 0))
    if welded or apart:
        lines.append((f"Mended: {welded} vertices welded, {apart} edges taken apart", "ERROR"))
    return lines


class OPENCLARE_OT_select_original(bpy.types.Operator):
    """Select the object this result was remeshed from, and make it the active one"""

    bl_idname = "object.openclare_select_original"
    bl_label = "Select Original"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        source = source_of(object_to_remesh(context))
        return source is not None and source.name in context.view_layer.objects and context.mode == "OBJECT"

    def execute(self, context):
        source = source_of(object_to_remesh(context))
        for other in context.selected_objects:
            other.select_set(False)
        source.hide_set(False)
        source.select_set(True)
        context.view_layer.objects.active = source
        return {"FINISHED"}


class OPENCLARE_OT_shape_preset_add(AddPresetBase, bpy.types.Operator):
    """Save the settings of the shape as a preset, or remove a preset that was saved"""

    bl_idname = "object.openclare_shape_preset_add"
    bl_label = "Add Shape Preset"
    preset_menu = "OPENCLARE_PT_shape_presets"
    preset_subdir = SHAPE_PRESETS
    preset_defines = ["settings = bpy.context.object.openclare"]
    preset_values = [f"settings.{name}" for name in SHAPE_NAMES]


class OPENCLARE_PT_shape_presets(PresetPanel, bpy.types.Panel):
    """The presets of the shape, in the header of its block"""

    bl_label = "Shape Presets"
    preset_subdir = SHAPE_PRESETS
    preset_operator = "script.execute_preset"
    preset_add_operator = OPENCLARE_OT_shape_preset_add.bl_idname


class OPENCLARE_PT_shape(DataPanel, bpy.types.Panel):
    """How the quads follow the shape: what a preset sets"""

    bl_label = "Shape"
    bl_idname = "OPENCLARE_PT_shape"
    bl_parent_id = "OPENCLARE_PT_panel"

    @classmethod
    def poll(cls, context):
        return is_source(context)

    def draw_header_preset(self, context):
        OPENCLARE_PT_shape_presets.draw_panel_header(self.layout)

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False
        settings = object_to_remesh(context).openclare
        layout.prop(settings, "adaptive_size", slider=True)
        layout.prop(settings, "quads_around")
        column = layout.column(heading="Hard Edges")
        row = column.row(align=True)
        row.prop(settings, "use_hard_edges", text="")
        angle = row.row(align=True)
        angle.active = settings.use_hard_edges
        angle.prop(settings, "hard_edge_angle", text="")
        layout.prop(settings, "follow_curvature")
        layout.prop(settings, "fewer_irregular")
        layout.prop(settings, "volume_fit", slider=True)


CLASSES = (
    OpenclareSettings,
    OPENCLARE_OT_remesh,
    OPENCLARE_OT_remesh_again,
    OPENCLARE_OT_stop,
    OPENCLARE_OT_select_original,
    OPENCLARE_OT_shape_preset_add,
    OPENCLARE_PT_shape_presets,
    OPENCLARE_PT_panel,
    OPENCLARE_PT_shape,
)


# The add-on's own presets are in presets/ here.
ADDON_FOLDER = os.path.dirname(os.path.abspath(__file__))
HANDLERS = ((bpy.app.handlers.load_post, after_loading),)


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Object.openclare = bpy.props.PointerProperty(type=OpenclareSettings)
    # Where files of 0.1 kept the settings: read once, then given to objects.
    bpy.types.Scene.openclare = bpy.props.PointerProperty(type=OpenclareSettings)
    bpy.types.Object.openclare_source = bpy.props.PointerProperty(
        name="Original",
        description="The object this one was remeshed from: what Remesh Again remeshes",
        type=bpy.types.Object,
    )
    for handlers, handler in HANDLERS:
        handlers.append(handler)
    bpy.utils.register_preset_path(ADDON_FOLDER)
    # The file open now, when Blender lets it be read (a script, not the
    # start of Blender: there, the file is read after, and the handler does
    # it).
    try:
        for scene in bpy.data.scenes:
            adopt_scene_settings(scene)
        for owner in bpy.data.objects:
            adopt_preset(owner.openclare)
    except AttributeError:
        pass


def unregister():
    if RUNNING is not None:
        RUNNING.stop.set()
    bpy.utils.unregister_preset_path(ADDON_FOLDER)
    for handlers, handler in HANDLERS:
        if handler in handlers:
            handlers.remove(handler)
    del bpy.types.Object.openclare_source
    del bpy.types.Object.openclare
    del bpy.types.Scene.openclare
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
