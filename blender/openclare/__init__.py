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


# The two kinds of curves drawn on a model, each with its collection and the
# colour its curves take in the viewport: name, label, what it does, colour
# of the objects, colour tag of the collection (the nearest Blender has).
# A run takes the curves of both; a curve is of one kind only.
CATEGORIES = [
    (
        "guide",
        "Guide",
        "Curves the result has a line of edges along, and nothing more: edge loops round an eye, a limb, a muscle",
        (0.0, 0.85, 1.0, 1.0),
        "COLOR_05",
    ),
    (
        "sharp",
        "Sharp",
        "Curves the result has a line of hard edges along: marked sharp and creased, "
        "as if the surface had a crease there",
        (0.6, 0.25, 1.0, 1.0),
        "COLOR_06",
    ),
]
CATEGORY = {name: (label, colour, tag) for name, label, _, colour, tag in CATEGORIES}
# What a curve goes back to when it leaves its category.
PLAIN = (1.0, 1.0, 1.0, 1.0)


# What the engine said of the last run of the operator, name by name: for a
# script that drives the add-on (bench/blender_run.py). Empty until a run
# ends well.
last_report = {}


def preset_changed(settings, context):
    settings.adaptive_size = ADAPTIVE_SIZE[settings.preset]


def is_a_mesh(settings, candidate):
    return candidate.type == "MESH"


def object_to_remesh(context):
    """The object the panel is about: the one chosen in it, or failing that
    the active object, if it is a mesh; None otherwise."""
    chosen = context.scene.openclare.source or context.active_object
    return chosen if chosen is not None and chosen.type == "MESH" else None


class OpenclareSettings(bpy.types.PropertyGroup):
    source: bpy.props.PointerProperty(
        name="Object",
        type=bpy.types.Object,
        poll=is_a_mesh,
        description=(
            "The object to remesh, taken as it is seen, modifiers applied. "
            "Leave empty to remesh the active object, whichever it is at the time"
        ),
    )
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
    guide_curves: bpy.props.PointerProperty(
        name="Guide Curves",
        type=bpy.types.Collection,
        description=(
            "Collection of curves drawn on the model: the result has a line of edges along each. "
            "Curves are taken with their modifiers, so a Shrinkwrap keeps them on the model. "
            "Leave empty for none"
        ),
    )
    sharp_curves: bpy.props.PointerProperty(
        name="Sharp Curves",
        type=bpy.types.Collection,
        description=(
            "Collection of curves drawn on the model: the result has a line of hard edges along each, "
            "marked sharp and creased. Curves are taken with their modifiers, so a Shrinkwrap keeps them "
            "on the model. Leave empty for none"
        ),
    )
    export_grid_uv: bpy.props.BoolProperty(
        name="Grid UV",
        description=(
            "Give the result a UV layer where every quad is a unit square: stripes and checkers follow the "
            "edges. Parts of it overlap, so it is for patterns that repeat, not for painting"
        ),
        default=True,
    )
    export_atlas_uv: bpy.props.BoolProperty(
        name="Atlas UV",
        description=(
            "Give the result a UV layer made of islands that never overlap, packed into the unit square, "
            "to paint or bake on, and mark its seams on the edges"
        ),
        default=True,
    )
    export_sharp_edges: bpy.props.BoolProperty(
        name="Sharp Edges",
        description=(
            "Mark as sharp the edges of the result that follow a hard edge of the original "
            "(only presets that keep hard edges, Hard surface, find any) or a curve of the Sharp category"
        ),
        default=True,
    )
    export_materials: bpy.props.BoolProperty(
        name="Materials",
        description=(
            "Give each quad of the result the material of the surface it lies on, and keep the borders "
            "between materials as lines of edges where that can be done. Has no effect on an object of "
            "one material"
        ),
        default=True,
    )
    export_creases: bpy.props.BoolProperty(
        name="Creases",
        description=(
            "Give a crease weight to the edges that should stay sharp when the result is subdivided: "
            "those along a hard edge of the original or a curve of the Sharp category, and those where "
            "the original folds sharply. "
            "A Subdivision Surface modifier reads them"
        ),
        default=True,
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


# Names of the layers of texture coordinates on the mesh made.
UV_NAMES = {"grid": "openclare grid", "atlas": "openclare atlas"}


def edge_keys(mesh, pairs):
    """One number per edge, whichever way round its two vertices are given:
    for the edges of `mesh`, and for those given as `pairs` of vertices."""
    import numpy

    ends = numpy.empty(2 * len(mesh.edges), numpy.int64)
    mesh.edges.foreach_get("vertices", ends)
    ends = numpy.sort(ends.reshape(-1, 2), axis=1)
    wanted = numpy.sort(pairs.astype(numpy.int64).reshape(-1, 2), axis=1)
    count = len(mesh.vertices)
    return ends[:, 0] * count + ends[:, 1], wanted[:, 0] * count + wanted[:, 1]


def flag_edges(mesh, attribute, pairs):
    """Sets the boolean `attribute` of the edges of `mesh` on those given as
    `pairs` of vertices, off on the others."""
    import numpy

    of_mesh, wanted = edge_keys(mesh, pairs)
    flags = numpy.isin(of_mesh, wanted)
    mesh.edges.foreach_set(attribute, flags)
    return int(flags.sum())


def crease_edges(mesh, pairs, weights):
    """Gives the edges of `mesh` given as `pairs` of vertices their crease
    `weights`, what a Subdivision Surface modifier reads; the others get 0."""
    import numpy

    of_mesh, wanted = edge_keys(mesh, pairs)
    order = numpy.argsort(wanted)
    found = numpy.searchsorted(wanted[order], of_mesh)
    found[found >= len(wanted)] = 0
    values = numpy.zeros(len(of_mesh), numpy.float32)
    if len(wanted):
        match = wanted[order][found] == of_mesh
        values[match] = weights[order][found[match]]
    attribute = mesh.attributes.get("crease_edge") or mesh.attributes.new("crease_edge", "FLOAT", "EDGE")
    attribute.data.foreach_set("value", values)
    return int((values > 0).sum())


def curve_objects(collection):
    """The curve objects of `collection`, those of the collections inside it
    included; none if there is no collection."""
    return [] if collection is None else [drawn for drawn in collection.all_objects if drawn.type == "CURVE"]


def curves_of(collection, target, sharp=False, but=()):
    """The curves of `collection` as the engine takes them: for each stretch
    of each curve object, its points in the space of the object `target`,
    whether it closes on itself, and `sharp`: whether the edges along it are
    to be hard edges. The objects in `but` are left out. Curves are taken as
    they are seen, modifiers applied: a Shrinkwrap is what keeps a curve on
    a model."""
    import numpy

    depsgraph = bpy.context.evaluated_depsgraph_get()
    into_target = target.matrix_world.inverted()
    curves = []
    for drawn in curve_objects(collection):
        if drawn in but:
            continue
        evaluated = drawn.evaluated_get(depsgraph)
        mesh = evaluated.to_mesh()
        if mesh is None:
            continue
        matrix = into_target @ evaluated.matrix_world
        points = [matrix @ vertex.co for vertex in mesh.vertices]
        # A curve comes out as chains of edges, one per stretch.
        joined = [[] for _ in points]
        for edge in mesh.edges:
            a, b = edge.vertices
            joined[a].append(b)
            joined[b].append(a)
        evaluated.to_mesh_clear()
        seen = [False] * len(points)
        ends = [v for v in range(len(points)) if len(joined[v]) == 1]
        for start in ends + list(range(len(points))):
            if seen[start] or not joined[start]:
                continue
            chain, previous, current = [start], None, start
            seen[start] = True
            while True:
                onward = [v for v in joined[current] if v != previous and not seen[v]]
                if not onward:
                    break
                previous, current = current, onward[0]
                seen[current] = True
                chain.append(current)
            closed = len(chain) > 2 and chain[0] in joined[chain[-1]]
            curves.append((numpy.array([tuple(points[v]) for v in chain], numpy.float32), closed, sharp))
    return curves


def quads_of(
    source, preset, settings, density=None, on_stage=None, uv_layers=(), sharp_edges=False, creases=False, curves=(),
    materials=False,
):
    """A new mesh datablock: `source`, an evaluated mesh, remeshed into quads
    with the engine's `settings` on top of `preset`, and `density` if given
    (one weight per vertex of `source`). `on_stage`, if given, is told the
    name of each stage as it begins.

    `uv_layers` names the layers of texture coordinates to put on the mesh
    ("grid", "atlas"); with the atlas come its seams, marked on the edges.
    `sharp_edges` marks as sharp the edges that follow a hard edge of
    `source`. `creases` gives a crease weight to the edges that should stay
    sharp when the mesh is subdivided. `curves` are guide curves, as
    `curves_of` gives them, in the space of `source`. `materials` gives the
    mesh the materials of `source`, each face that of the surface it lies
    on, with the borders between them kept as lines of edges."""
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
    groups = None
    if materials and len(source.materials) > 1:
        # The material of each triangle; an object all of one is left alone.
        groups = numpy.empty(triangles, numpy.int32)
        source.loop_triangles.foreach_get("material_index", groups)
        groups = groups.astype(numpy.uint32) if groups.min() != groups.max() else None
    made = engine.remesh(
        addresses[0], vertices, addresses[1], triangles, addresses[2], corners, preset, settings, density, on_stage,
        uv_layers=uv_layers, seams="atlas" in uv_layers, hard_edges=sharp_edges, creases=creases, curves=curves,
        groups=groups,
    )
    del kept
    positions, sizes, corner_vertices, report = made.positions, made.sizes, made.corners, made.report

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
    for name in uv_layers:
        layer = mesh.uv_layers.new(name=UV_NAMES[name], do_init=False)
        layer.data.foreach_set("uv", made.uv[name])
    if "atlas" in uv_layers:
        # The atlas is the one to paint on: it is the one Blender shows.
        mesh.uv_layers.active = mesh.uv_layers[UV_NAMES["atlas"]]
        mesh.uv_layers[UV_NAMES["atlas"]].active_render = True
        report["seams"] = flag_edges(mesh, "use_seam", made.seams)
    if made.face_groups is not None:
        for material in source.materials:
            mesh.materials.append(material)
        mesh.polygons.foreach_set("material_index", made.face_groups.astype(numpy.int32))
        report["materials"] = len(numpy.unique(made.face_groups))
    if sharp_edges:
        report["sharp_edges"] = flag_edges(mesh, "use_edge_sharp", made.hard_edges)
    if creases:
        report["creases"] = crease_edges(mesh, *made.creases)
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
        return object_to_remesh(context) is not None and context.mode == "OBJECT"

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
        original = object_to_remesh(context)
        self.drawn, self.stages = 0.0, []
        last_report.clear()
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
            layers = [name for name in ("grid", "atlas") if getattr(settings, f"export_{name}_uv")]
            # A curve put in both collections by hand is a sharp one.
            sharp = curve_objects(settings.sharp_curves)
            curves = curves_of(settings.sharp_curves, original, sharp=True)
            curves += curves_of(settings.guide_curves, original, but=sharp)
            mesh, report = quads_of(
                source, settings.preset, engine_settings, density, on_stage, layers,
                settings.export_sharp_edges, settings.export_creases, curves, settings.export_materials,
            )
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

        last_report.update(report)
        seconds = float(report.get("seconds", 0.0))
        said = f"openclare: {report.get('quads', '?')} quads in {seconds:.1f} s"
        if "materials" in report:
            said += f", {report['materials']} materials"
        welded, apart = int(report.get("welded_vertices", 0)), int(report.get("edges_taken_apart", 0))
        if welded or apart:
            # Said, because the result then has borders the object did not show.
            said += f"; the mesh had defects and was mended ({welded} vertices welded along torn borders"
            said += f", {apart} edges with more than two faces taken apart)"
        if curves:
            said += f", {report.get('guide_edges', '0')} edges along the guide curves"
            if sharp:
                said += f", {report.get('sharp_curve_edges', '0')} along the sharp curves"
            if report.get("guide_gaps", "0") != "0":
                said += f" ({report['guide_gaps']} breaks)"
        # Said as a warning: the result then lacks part of the object.
        failed = int(report.get("pieces_failed", 0))
        if failed:
            said += f"; {failed} of {report.get('pieces', '?')} pieces could not be meshed and are missing"
        self.report({"WARNING"} if failed else {"INFO"}, said)
        return {"FINISHED"}


def category_collection(context, name, create=False):
    """The collection of the category `name`; made and put in the scene if
    there is none yet and `create` says so."""
    settings = context.scene.openclare
    collection = getattr(settings, f"{name}_curves")
    if collection is None and create:
        label, _, tag = CATEGORY[name]
        collection = bpy.data.collections.new(f"openclare {label.lower()} curves")
        collection.color_tag = tag
        context.scene.collection.children.link(collection)
        setattr(settings, f"{name}_curves", collection)
    return collection


def show_object_colours(context):
    """Has the 3D viewports of the screen draw wires in the colour of their
    object, which is how the two categories are told apart."""
    for area in context.screen.areas if context.screen else ():
        if area.type == "VIEW_3D":
            area.spaces.active.shading.wireframe_color_type = "OBJECT"


class CurvesOperator:
    """What the four buttons of a curve category have in common."""

    bl_options = {"REGISTER", "UNDO"}

    category: bpy.props.EnumProperty(
        name="Category",
        items=[(name, label, description) for name, label, description, _, _ in CATEGORIES],
        default="guide",
        options={"HIDDEN"},
    )

    @classmethod
    def poll(cls, context):
        return context.mode == "OBJECT"


class OPENCLARE_OT_curves_assign(CurvesOperator, bpy.types.Operator):
    """Move the selected curves to this category and give them its colour. A curve is in one category only"""

    bl_idname = "object.openclare_curves_assign"
    bl_label = "Assign"

    def execute(self, context):
        chosen = [drawn for drawn in context.selected_objects if drawn.type == "CURVE"]
        if not chosen:
            self.report({"WARNING"}, "openclare: no curve is selected")
            return {"CANCELLED"}
        collection = category_collection(context, self.category, create=True)
        label, colour, _ = CATEGORY[self.category]
        for drawn in chosen:
            # Moved, not linked as well: a curve that stayed in another
            # collection too would still show when its category is hidden.
            for other in drawn.users_collection:
                if other != collection:
                    other.objects.unlink(drawn)
            if drawn.name not in collection.objects:
                collection.objects.link(drawn)
            drawn.color = colour
        show_object_colours(context)
        self.report({"INFO"}, f"openclare: {len(chosen)} curve(s) in the {label} category")
        return {"FINISHED"}


class OPENCLARE_OT_curves_remove(CurvesOperator, bpy.types.Operator):
    """Take the selected curves out of this category: back to the collection of the scene, in their plain colour"""

    bl_idname = "object.openclare_curves_remove"
    bl_label = "Remove"

    def execute(self, context):
        collection = category_collection(context, self.category)
        chosen = [drawn for drawn in context.selected_objects if drawn.type == "CURVE"]
        chosen = [drawn for drawn in chosen if collection is not None and drawn.name in collection.objects]
        for drawn in chosen:
            collection.objects.unlink(drawn)
            # A curve must stay somewhere in the scene.
            if not drawn.users_collection:
                context.scene.collection.objects.link(drawn)
            drawn.color = PLAIN
        self.report({"INFO"}, f"openclare: {len(chosen)} curve(s) taken out")
        return {"FINISHED"}


class OPENCLARE_OT_curves_select(CurvesOperator, bpy.types.Operator):
    """Select the curves of this category"""

    bl_idname = "object.openclare_curves_select"
    bl_label = "Select"

    select: bpy.props.BoolProperty(default=True, options={"HIDDEN"})

    @classmethod
    def description(cls, context, properties):
        return "Select the curves of this category" if properties.select else "Deselect the curves of this category"

    def execute(self, context):
        for drawn in curve_objects(category_collection(context, self.category)):
            # Only what this view layer shows can be selected.
            if drawn.name in context.view_layer.objects and (drawn.visible_get() or not self.select):
                drawn.select_set(self.select)
                if self.select:
                    context.view_layer.objects.active = drawn
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
        layout.prop(settings, "source")
        if settings.source is None:
            # Said, so that it is plain what the button will act on.
            active = object_to_remesh(context)
            layout.label(text=f"Active object: {active.name}" if active else "No mesh is active", icon="INFO")
        layout.prop(settings, "preset")
        layout.prop(settings, "quads")
        layout.prop(settings, "adaptive_size", slider=True)
        chosen = object_to_remesh(context)
        if chosen is not None:
            column = layout.column(align=True)
            column.prop_search(settings, "density_group", chosen, "vertex_groups", icon="GROUP_VERTEX")
            factor = column.row()
            factor.enabled = bool(settings.density_group)
            factor.prop(settings, "density_factor")
        layout.operator(OPENCLARE_OT_remesh.bl_idname, icon="MOD_REMESH")


class OPENCLARE_PT_curves(bpy.types.Panel):
    """Curves drawn on the model for the result to have edges along"""

    bl_label = "Curves"
    bl_idname = "OPENCLARE_PT_curves"
    bl_parent_id = "OPENCLARE_PT_panel"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"
    bl_category = "openclare"

    def draw(self, context):
        layout = self.layout
        settings = context.scene.openclare
        # One box per category, both always there: a run takes the curves
        # of both.
        for name, label, _, _, tag in CATEGORIES:
            box = layout.box()
            count = len(curve_objects(getattr(settings, f"{name}_curves")))
            curves = f"{count} curve" + ("" if count == 1 else "s")
            box.label(text=f"{label}: {curves}", icon=tag.replace("COLOR", "COLLECTION_COLOR"))
            box.prop(settings, f"{name}_curves", text="")
            column = box.column(align=True)
            row = column.row(align=True)
            row.operator(OPENCLARE_OT_curves_assign.bl_idname).category = name
            row.operator(OPENCLARE_OT_curves_remove.bl_idname).category = name
            row = column.row(align=True)
            for text, select in (("Select", True), ("Deselect", False)):
                button = row.operator(OPENCLARE_OT_curves_select.bl_idname, text=text)
                button.category, button.select = name, select


class OPENCLARE_PT_export(bpy.types.Panel):
    """What the result carries besides its quads"""

    bl_label = "Export"
    bl_idname = "OPENCLARE_PT_export"
    bl_parent_id = "OPENCLARE_PT_panel"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"
    bl_category = "openclare"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        settings = context.scene.openclare
        column = layout.column(heading="UV Layers")
        column.prop(settings, "export_grid_uv", text="Grid")
        column.prop(settings, "export_atlas_uv", text="Atlas")
        column = layout.column(heading="Surface")
        column.prop(settings, "export_materials", text="Materials")
        column = layout.column(heading="Edges")
        column.prop(settings, "export_sharp_edges", text="Sharp")
        column.prop(settings, "export_creases", text="Creases")


CLASSES = (
    OpenclareSettings,
    OPENCLARE_OT_remesh,
    OPENCLARE_OT_curves_assign,
    OPENCLARE_OT_curves_remove,
    OPENCLARE_OT_curves_select,
    OPENCLARE_PT_panel,
    OPENCLARE_PT_curves,
    OPENCLARE_PT_export,
)


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Scene.openclare = bpy.props.PointerProperty(type=OpenclareSettings)


def unregister():
    del bpy.types.Scene.openclare
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
