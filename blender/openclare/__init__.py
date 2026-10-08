"""openclare: automatic quad retopology, as a Blender add-on.

A panel in the sidebar of the 3D viewport (tab "openclare") remeshes the
active object into quads. The result is a new object, next to the original,
which is left as it is.
"""

import bpy

from . import engine

PRESETS = [
    ("organic", "Organic", "Sculpts and characters: edges follow the shape closely, quads are smaller where it bends"),
    ("animation", "Animation", "Meshes meant to deform: fewer irregular vertices, at some cost in fidelity"),
    ("hard-surface", "Hard surface", "Mechanical parts: edges sharper than 30 degrees are kept exactly"),
]
# How much the size of the quads follows the curvature in each preset, in
# per cent: what the slider goes back to when the preset changes.
ADAPTIVE_SIZE = {"organic": 50.0, "animation": 50.0, "hard-surface": 0.0}


def preset_changed(settings, context):
    settings.adaptive_size = ADAPTIVE_SIZE[settings.preset]


class OpenclareSettings(bpy.types.PropertyGroup):
    preset: bpy.props.EnumProperty(
        name="Preset", description="What the mesh is for", items=PRESETS, default="organic", update=preset_changed
    )
    quads: bpy.props.IntProperty(
        name="Quads", description="Number of quads asked for", default=3000, min=1, soft_min=100, soft_max=50_000
    )
    adaptive_size: bpy.props.FloatProperty(
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
    density_group: bpy.props.StringProperty(
        name="Density",
        description=(
            "Vertex group painted with the density wanted (Weight Paint): quads are smallest where the weight "
            "is 1, largest where it is 0. Leave empty for none"
        ),
    )
    density_factor: bpy.props.FloatProperty(
        name="Factor",
        description=(
            "How many times more quads a weight of 1 gets than a weight of 0, for the same area. "
            "At 4, their edges are half as long; at 1 the paint has no effect"
        ),
        default=4.0,
        min=1.0,
        soft_max=16.0,
        max=100.0,
    )


def weights_of(evaluated, mesh, name):
    """The weights of the vertex group `name` on `mesh`, the evaluated mesh
    of the object `evaluated`: one 32-bit float per vertex, 0 where the group
    has nothing."""
    import numpy

    group = evaluated.vertex_groups.get(name)
    if group is None:
        raise engine.EngineError(f"the object has no vertex group named '{name}'")
    weights = numpy.zeros(len(mesh.vertices), numpy.float32)
    for vertex in mesh.vertices:
        for membership in vertex.groups:
            if membership.group == group.index:
                weights[vertex.index] = membership.weight
    return weights


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


def quads_of(source, preset, settings, density=None, on_stage=None):
    """A new mesh datablock: `source`, an evaluated mesh, remeshed into quads
    with the engine's `settings` on top of `preset`, and `density` if given
    (one weight per vertex of `source`). `on_stage`, if given, is told the
    name of each stage as it begins."""
    import numpy

    source.calc_loop_triangles()
    vertices, corners, triangles = len(source.vertices), len(source.loops), len(source.loop_triangles)
    if triangles == 0:
        raise engine.EngineError("the object has no faces")
    if in_place(source):
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
    positions, sizes, corner_vertices, report = engine.remesh(
        addresses[0], vertices, addresses[1], triangles, addresses[2], corners, preset, settings, density, on_stage
    )
    del kept

    mesh = bpy.data.meshes.new("openclare")
    mesh.vertices.add(len(positions) // 3)
    mesh.vertices.foreach_set("co", positions)
    mesh.loops.add(len(corner_vertices))
    mesh.loops.foreach_set("vertex_index", corner_vertices.astype(numpy.int32))
    mesh.polygons.add(len(sizes))
    starts = numpy.zeros(len(sizes), numpy.int32)
    numpy.cumsum(sizes[:-1], out=starts[1:])
    mesh.polygons.foreach_set("loop_start", starts)
    mesh.polygons.foreach_set("loop_total", sizes.astype(numpy.int32))
    mesh.update(calc_edges=True)
    mesh.validate()
    return mesh, report


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


class OPENCLARE_OT_remesh(bpy.types.Operator):
    """Remesh the active object into quads, as a new object next to it"""

    bl_idname = "object.openclare_remesh"
    bl_label = "Remesh to Quads"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        active = context.active_object
        return active is not None and active.type == "MESH" and context.mode == "OBJECT"

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

    def execute(self, context):
        settings = context.scene.openclare
        original = context.active_object
        self.drawn, self.stages = 0.0, []
        context.window_manager.progress_begin(0, STAGES)
        self.say(context, f"reading '{original.name}'")
        # With its modifiers applied: what is seen is what is remeshed.
        evaluated = original.evaluated_get(context.evaluated_depsgraph_get())
        source = evaluated.to_mesh()

        def on_stage(stage):
            self.stages.append(stage)
            self.say(context, stage, min(len(self.stages), STAGES))
            return False

        try:
            engine_settings = {"quads": settings.quads, "adaptivity": settings.adaptive_size / 100.0}
            density = None
            if settings.density_group:
                density = weights_of(evaluated, source, settings.density_group)
                engine_settings["density_contrast"] = settings.density_factor
            mesh, report = quads_of(source, settings.preset, engine_settings, density, on_stage)
        except engine.EngineError as error:
            self.report({"ERROR"}, f"openclare: {error}")
            return {"CANCELLED"}
        finally:
            evaluated.to_mesh_clear()
            context.window_manager.progress_end()
            context.workspace.status_text_set(None)

        mesh.name = f"{original.data.name}.quads"
        remeshed = bpy.data.objects.new(f"{original.name}.quads", mesh)
        remeshed.matrix_world = original.matrix_world
        for collection in original.users_collection or [context.collection]:
            collection.objects.link(remeshed)
        for other in context.selected_objects:
            other.select_set(False)
        remeshed.select_set(True)
        context.view_layer.objects.active = remeshed

        seconds = float(report.get("seconds", 0.0))
        self.report({"INFO"}, f"openclare: {report.get('quads', '?')} quads in {seconds:.1f} s")
        return {"FINISHED"}


class OPENCLARE_PT_panel(bpy.types.Panel):
    bl_label = "openclare"
    bl_idname = "OPENCLARE_PT_panel"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"
    bl_category = "openclare"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        settings = context.scene.openclare
        layout.prop(settings, "preset")
        layout.prop(settings, "quads")
        layout.prop(settings, "adaptive_size", slider=True)
        active = context.active_object
        if active is not None and active.type == "MESH":
            column = layout.column(align=True)
            column.prop_search(settings, "density_group", active, "vertex_groups", icon="GROUP_VERTEX")
            factor = column.row()
            factor.enabled = bool(settings.density_group)
            factor.prop(settings, "density_factor")
        layout.operator(OPENCLARE_OT_remesh.bl_idname, icon="MOD_REMESH")


CLASSES = (OpenclareSettings, OPENCLARE_OT_remesh, OPENCLARE_PT_panel)


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Scene.openclare = bpy.props.PointerProperty(type=OpenclareSettings)


def unregister():
    del bpy.types.Scene.openclare
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
