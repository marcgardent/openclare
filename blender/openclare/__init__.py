"""openclare: automatic quad retopology, as a Blender add-on.

A panel in the sidebar of the 3D viewport (tab "openclare") remeshes the
active object into quads. The result is a new object, next to the original,
which is left as it is.
"""

import time

import bpy
import mathutils

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



# How many of its largest quads go along the diagonal of the box of an object
# when the sizes are set from it, and how many times smaller the smallest is.
QUADS_ALONG = 40.0
SIZE_RANGE = 8.0


def sizes_from(owner):
    """The largest and the smallest quad to start from for an object, in the
    units of the scene: by its size alone."""
    corners = [owner.matrix_world @ mathutils.Vector(corner) for corner in owner.bound_box]
    diagonal = max((a - b).length for a in corners for b in corners)
    largest = diagonal / QUADS_ALONG
    return largest, largest / SIZE_RANGE


def target_changed(settings, context):
    """Sizes asked for and none set: those the object gives."""
    if settings.target != "SIZE" or settings.largest_quad > 0.0:
        return
    owner = settings.source or object_to_remesh(context)
    if owner is not None:
        settings.largest_quad, settings.smallest_quad = sizes_from(owner)


# Shapes analysed: by object name, the engine's analysis, what it was made
# from (to tell when it no longer holds), and the scale of the object then.
ANALYSES = {}
# The last count asked of an analysis, which the panel draws again and again.
COUNTED = {}
# The preview of the sizes: the copy shown, the object hidden for it, and how
# the 3D views coloured their objects before.
PREVIEW = {}
# Sequential colours of the sizes, light for the largest quads to dark for the
# smallest, and the colour of what is coarser than its shape asks.
SIZE_RAMP = ["#cde2fb", "#9ec5f4", "#6da7ec", "#3987e5", "#256abf", "#184f95", "#0d366b"]
COARSER = "#e34948"


def analysis_key(settings, owner, evaluated_mesh):
    """What an analysis depends on: the mesh as seen and the settings that
    say what the shape calls for."""
    vertices = len(evaluated_mesh.vertices)
    corner = tuple(round(c, 5) for c in evaluated_mesh.vertices[0].co) if vertices else ()
    return (owner.name, vertices, len(evaluated_mesh.polygons), corner, settings.preset,
            round(settings.adaptive_size, 3), settings.density_group, round(settings.density_factor, 3))


def scale_of(owner):
    """How long a unit of the mesh is in the scene."""
    return sum(abs(factor) for factor in owner.matrix_world.to_scale()) / 3.0 or 1.0


def analysis_of(context, settings, owner):
    """The analysis of `owner` as it is now, if it was made: (analysis,
    scale), or None."""
    held = ANALYSES.get(owner.name)
    if held is None:
        return None
    evaluated = owner.evaluated_get(context.evaluated_depsgraph_get())
    mesh = evaluated.to_mesh()
    try:
        current = analysis_key(settings, owner, mesh)
    finally:
        evaluated.to_mesh_clear()
    return (held["analysis"], scale_of(owner)) if held["key"] == current else None


def counted(analysis, scale, largest, smallest):
    """How many quads and how much is coarser than asked, for sizes in the
    units of the scene; the last answer is kept."""
    asked = (id(analysis), round(largest, 9), round(smallest, 9), scale)
    if COUNTED.get("asked") != asked:
        COUNTED["asked"], COUNTED["answer"] = asked, analysis.count(largest / scale, smallest / scale)
    return COUNTED["answer"]


def linear(hex_colour):
    """A colour written as #rrggbb, in the linear values Blender keeps."""
    channels = [int(hex_colour[k : k + 2], 16) / 255.0 for k in (1, 3, 5)]
    return [c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4 for c in channels] + [1.0]


def colour_sizes(settings):
    """Colours the preview with the sizes asked for now."""
    import numpy

    shown = PREVIEW.get("object")
    held = ANALYSES.get(PREVIEW.get("of", ""))
    if shown is None or held is None or shown.name not in bpy.data.objects:
        return
    scale = PREVIEW["scale"]
    largest = settings.largest_quad / scale
    smallest = min(settings.smallest_quad / scale, largest) or largest / SIZE_RANGE
    sizes, coarser = held["analysis"].at_vertices(largest, smallest)
    ramp = numpy.array([linear(colour) for colour in SIZE_RAMP], numpy.float32)
    span = max(numpy.log(largest / smallest), 1e-9)
    # From light (largest) to dark (smallest), as a logarithm of the size.
    t = numpy.clip(numpy.log(largest / numpy.nan_to_num(sizes, nan=largest)) / span, 0.0, 1.0) * (len(ramp) - 1)
    low = numpy.floor(t).astype(int).clip(0, len(ramp) - 2)
    part = (t - low)[:, None]
    colours = ramp[low] * (1.0 - part) + ramp[low + 1] * part
    colours[coarser] = linear(COARSER)
    mesh = shown.data
    attribute = mesh.color_attributes.get("openclare sizes") or mesh.color_attributes.new(
        "openclare sizes", "FLOAT_COLOR", "POINT"
    )
    attribute.data.foreach_set("color", colours.astype(numpy.float32).ravel())
    mesh.color_attributes.active_color = attribute
    mesh.update()


def stop_preview(context):
    """Takes the preview away and shows the object again."""
    shown, hidden = PREVIEW.pop("object", None), PREVIEW.pop("hidden", None)
    PREVIEW.pop("of", None)
    if shown is not None and shown.name in bpy.data.objects:
        mesh = shown.data
        bpy.data.objects.remove(shown)
        if mesh.users == 0:
            bpy.data.meshes.remove(mesh)
    if hidden is not None and hidden.name in bpy.data.objects:
        hidden.hide_set(False)
    for space, colour_type in PREVIEW.pop("views", []):
        try:
            space.shading.color_type = colour_type
        except ReferenceError:
            pass


def start_preview(context, settings, owner):
    """Shows a copy of the object as seen, coloured with its sizes, in its
    place, and hides the object meanwhile."""
    stop_preview(context)
    evaluated = owner.evaluated_get(context.evaluated_depsgraph_get())
    mesh = bpy.data.meshes.new_from_object(evaluated)
    mesh.name = f"{owner.data.name}.sizes"
    shown = bpy.data.objects.new(f"{owner.name}.sizes", mesh)
    shown.matrix_world = owner.matrix_world
    for collection in owner.users_collection:
        collection.objects.link(shown)
    owner.hide_set(True)
    PREVIEW.update(object=shown, hidden=owner, of=owner.name, scale=scale_of(owner))
    # The 3D views show the colour attribute meanwhile ("VERTEX", shown as
    # Attribute).
    PREVIEW["views"] = []
    for area in context.screen.areas if context.screen else []:
        if area.type == "VIEW_3D":
            space = area.spaces.active
            PREVIEW["views"].append((space, space.shading.color_type))
            space.shading.color_type = "VERTEX"
    colour_sizes(settings)


def sizes_changed(settings, context):
    """The sizes moved: the preview follows."""
    if PREVIEW.get("object") is not None:
        colour_sizes(settings)


def show_sizes_changed(settings, context):
    if not settings.show_sizes:
        stop_preview(context)
        return
    owner = object_to_remesh(context)
    if owner is None or analysis_of(context, settings, owner) is None:
        # Nothing to show before the object is analysed as it is.
        settings.show_sizes = False
        return
    start_preview(context, settings, owner)


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
    target: bpy.props.EnumProperty(
        name="Ask For",
        description="What says how dense the mesh is",
        items=[
            (
                "COUNT",
                "Number",
                "A number of quads: the shape says where they are smaller, and the number is kept",
            ),
            (
                "SIZE",
                "Sizes",
                "The largest and the smallest quad: the shape says which where, and the number of quads is "
                "what that makes",
            ),
        ],
        default="COUNT",
        update=target_changed,
    )
    quads: bpy.props.IntProperty(
        name="Quads",
        description="Number of quads asked for",
        default=3000,
        min=1,
        soft_min=100,
        soft_max=50_000,
    )
    largest_quad: bpy.props.FloatProperty(
        name="Largest",
        description=(
            "Edge of the largest quads, in the units of the scene: what the shape is meshed with wherever "
            "it asks for nothing smaller. 0 to have it set from the size of the object"
        ),
        default=0.0,
        min=0.0,
        subtype="DISTANCE",
        precision=4,
        update=sizes_changed,
    )
    smallest_quad: bpy.props.FloatProperty(
        name="Smallest",
        description=(
            "Edge of the smallest quads, in the units of the scene: thin parts and tight bends get quads "
            "down to that size, and what would call for smaller comes out coarser than its shape asks"
        ),
        default=0.0,
        min=0.0,
        subtype="DISTANCE",
        precision=4,
        update=sizes_changed,
    )
    show_sizes: bpy.props.BoolProperty(
        name="Show Sizes",
        description=(
            "Colour the object with the size its quads will have: light where they are largest, dark where "
            "they are smallest, red where its shape calls for quads smaller than the smallest. Shown on a "
            "copy, in Solid view by attribute; the object itself is hidden meanwhile and left as it is"
        ),
        default=False,
        update=show_sizes_changed,
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
    relief_resolution: bpy.props.EnumProperty(
        name="Resolution",
        description="Width and height of the height map, in texels",
        items=[(str(size), f"{size} x {size}", "") for size in (512, 1024, 2048, 4096)],
        default="2048",
    )
    relief_levels: bpy.props.IntProperty(
        name="Levels",
        description=(
            "How many times the quad mesh is subdivided under the relief: the height map is measured from "
            "that surface, creases included, and moves its vertices"
        ),
        default=2,
        min=0,
        max=5,
    )
    relief_apply: bpy.props.EnumProperty(
        name="Apply As",
        description="How the height map moves the surface",
        items=[
            (
                "MODIFIER", "Modifier",
                "A Displace modifier after the Subdivision Surface: shown in every view and by every renderer, "
                "and applied or exported as geometry; the mesh is really subdivided, and carries no more "
                "detail than its vertices allow",
            ),
            (
                "MATERIAL", "Material",
                "The displacement of the material, with adaptive subdivision: Cycles dices the surface to the "
                "size of a pixel when it renders, for more detail and less memory; nothing moves in Solid view",
            ),
        ],
        default="MODIFIER",
    )
    relief_format: bpy.props.EnumProperty(
        name="File",
        description="Write the height map as a file too, to use it elsewhere",
        items=[
            ("NONE", "None", "The image is kept in the .blend file only"),
            (
                "EXR32", "OpenEXR 32-bit",
                "The heights themselves, in the unit of the object, as 32-bit floats: midlevel 0, scale 1",
            ),
            (
                "EXR16", "OpenEXR 16-bit",
                "The heights themselves, in the unit of the object, as half floats: midlevel 0, scale 1",
            ),
            (
                "PNG16", "PNG 16-bit",
                "The heights brought between 0 and 1: midlevel 0.5, and the scale that the report gives",
            ),
        ],
        default="NONE",
    )
    relief_directory: bpy.props.StringProperty(
        name="Folder",
        description="Where the file is written; // is the folder of the .blend file",
        subtype="DIR_PATH",
        default="//textures/",
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


class OPENCLARE_OT_analyse(bpy.types.Operator):
    """Measure what the shape asks for, and propose a largest and a smallest quad: how many quads any sizes
    make is then shown at once"""

    bl_idname = "object.openclare_analyse"
    bl_label = "Analyse"
    bl_options = {"REGISTER"}

    @classmethod
    def poll(cls, context):
        return object_to_remesh(context) is not None

    def execute(self, context):
        import numpy

        settings = context.scene.openclare
        owner = object_to_remesh(context)
        evaluated = owner.evaluated_get(context.evaluated_depsgraph_get())
        mesh = evaluated.to_mesh()
        try:
            mesh.calc_loop_triangles()
            positions = numpy.empty(3 * len(mesh.vertices), numpy.float32)
            mesh.vertices.foreach_get("co", positions)
            triangles = numpy.empty(3 * len(mesh.loop_triangles), numpy.uint32)
            mesh.loop_triangles.foreach_get("vertices", triangles)
            engine_settings = {"adaptivity": settings.adaptive_size / 100.0}
            density = None
            if settings.density_group:
                density = weights_of(evaluated, mesh, settings.density_group)
                engine_settings["density_contrast"] = settings.density_factor
            key = analysis_key(settings, owner, mesh)
            started = time.perf_counter()
            analysis = engine.Analysis(positions, triangles, settings.preset, engine_settings, density)
            seconds = time.perf_counter() - started
        except engine.EngineError as error:
            self.report({"ERROR"}, f"openclare: {error}")
            return {"CANCELLED"}
        finally:
            evaluated.to_mesh_clear()
        stop_preview(context)
        settings.show_sizes = False
        ANALYSES[owner.name] = {"analysis": analysis, "key": key}
        scale = scale_of(owner)
        largest, smallest = analysis.proposal()
        settings.target = "SIZE"
        settings.largest_quad, settings.smallest_quad = largest * scale, smallest * scale
        quads, below = analysis.count(largest, smallest)
        self.report(
            {"INFO"},
            f"openclare: '{owner.name}' analysed in {seconds:.1f} s; proposed: about {quads:.0f} quads, "
            f"{100.0 * below:.0f} % of it coarser than its shape asks",
        )
        return {"FINISHED"}


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
        if PREVIEW.get("object") is not None:
            settings.show_sizes = False
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
            if settings.target == "SIZE":
                # Lengths of the scene, where the engine is given the mesh
                # as the object holds it: before its scale.
                if settings.largest_quad <= 0.0:
                    settings.largest_quad, settings.smallest_quad = sizes_from(original)
                scale = sum(abs(factor) for factor in original.matrix_world.to_scale()) / 3.0 or 1.0
                smallest = min(settings.smallest_quad, settings.largest_quad) or settings.largest_quad / SIZE_RANGE
                del engine_settings["quads"]
                engine_settings["largest_quad"] = settings.largest_quad / scale
                engine_settings["smallest_quad"] = smallest / scale
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
        # Kept, for what is baked onto the result later to be read from it.
        remeshed.openclare_source = original
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
        thin, moved = int(report.get("thin_pieces", 0)), int(report.get("quads_for_thin_pieces", 0))
        if thin:
            said += f"; {moved} of them moved to {thin} thin part(s), to keep their shape"
        wanted, below = float(report.get("quads_wanted", 0.0)), float(report.get("below_smallest", 0.0))
        if wanted > 0.0:
            said += f"; the shape called for {wanted:.0f}"
            if below >= 0.005:
                said += f", and {100.0 * below:.0f} % of it for quads smaller than the smallest"
        boxed = int(report.get("pieces_boxed", 0))
        if boxed:
            said += f"; {boxed} part(s) smaller than a quad could not be meshed and are boxes"
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


# The modifiers the relief is made of, by name: found again and updated when
# it is baked again.
SUBDIVISION, RELIEF = "openclare subdivision", "openclare relief"
# How far from the subdivided result the sculpt is looked for, in mean edges
# of the quad mesh: the result lies that close to it.
RELIEF_REACH = 1.0


def relief_of(context, target, source, levels, resolution):
    """The height of `source` above `target` subdivided `levels` times, on
    the atlas of `target`: an array of `resolution` rows of as many floats
    from the bottom up, in the unit of `target`, and what the engine says
    of the bake."""
    import numpy

    atlas = UV_NAMES["atlas"]
    if atlas not in target.data.uv_layers:
        raise engine.EngineError(f"'{target.name}' has no '{atlas}' UV layer: remesh with Export > UV Layers > Atlas")
    subdivision = target.modifiers.get(SUBDIVISION) or target.modifiers.new(SUBDIVISION, "SUBSURF")
    subdivision.levels = subdivision.render_levels = levels
    subdivision.show_viewport = levels > 0
    relief = target.modifiers.get(RELIEF)
    shown = relief.show_viewport if relief else None
    if relief:
        # What is measured is the surface under the relief, not with it.
        relief.show_viewport = False
    try:
        depsgraph = context.evaluated_depsgraph_get()
        depsgraph.update()
        surface = target.evaluated_get(depsgraph).to_mesh()
        surface.calc_loop_triangles()
        count, faces = len(surface.vertices), len(surface.loop_triangles)
        positions, normals = numpy.empty(3 * count, numpy.float32), numpy.empty(3 * count, numpy.float32)
        surface.vertices.foreach_get("co", positions)
        surface.vertex_normals.foreach_get("vector", normals)
        triangles, corners = numpy.empty(3 * faces, numpy.int32), numpy.empty(3 * faces, numpy.int32)
        surface.loop_triangles.foreach_get("vertices", triangles)
        surface.loop_triangles.foreach_get("loops", corners)
        uvs = numpy.empty(2 * len(surface.loops), numpy.float32)
        surface.uv_layers[atlas].data.foreach_get("uv", uvs)
        uvs = uvs.reshape(-1, 2)[corners]
        target.evaluated_get(depsgraph).to_mesh_clear()

        sculpt = source.evaluated_get(depsgraph).to_mesh()
        sculpt.calc_loop_triangles()
        sculpt_positions = numpy.empty(3 * len(sculpt.vertices), numpy.float32)
        sculpt.vertices.foreach_get("co", sculpt_positions)
        sculpt_triangles = numpy.empty(3 * len(sculpt.loop_triangles), numpy.int32)
        sculpt.loop_triangles.foreach_get("vertices", sculpt_triangles)
        source.evaluated_get(depsgraph).to_mesh_clear()
    finally:
        if relief:
            relief.show_viewport = shown
    # Each in its own space, where the two lie on one another whatever was
    # done to the objects since: the result was made in the space of the
    # original, and the original is often moved aside to see them both.

    edges = numpy.empty(2 * len(target.data.edges), numpy.int32)
    target.data.edges.foreach_get("vertices", edges)
    corners_of = numpy.empty(3 * len(target.data.vertices), numpy.float32)
    target.data.vertices.foreach_get("co", corners_of)
    ends = corners_of.reshape(-1, 3)[edges.reshape(-1, 2)]
    reach = RELIEF_REACH * float(numpy.linalg.norm(ends[:, 0] - ends[:, 1], axis=1).mean())
    heights, report = engine.bake_height(
        positions, normals, triangles.astype(numpy.uint32), uvs, sculpt_positions,
        sculpt_triangles.astype(numpy.uint32), resolution, resolution, reach,
    )
    covered, astray = int(report["covered"]), int(report["astray"])
    if 2 * astray > covered:
        # Nothing to make a relief of: better none than one made of guesses.
        raise engine.EngineError(
            f"'{source.name}' is out of reach under {100.0 * astray / max(covered, 1):.0f} % of '{target.name}': "
            "the result no longer lies on its original (was the mesh of one of them edited, or its scale applied?)"
        )
    return heights, report


# The nodes of a material that carry the relief, by name, likewise.
HEIGHT_NODES = ("openclare atlas", "openclare height", "openclare displacement")
# What a height map written as a file holds, and how it is to be read: the
# format and depth Blender is asked for, the extension, the midlevel.
RELIEF_FILES = {"EXR32": ("OPEN_EXR", "32", "exr", 0.0), "EXR16": ("OPEN_EXR", "16", "exr", 0.0), "PNG16": ("PNG", "16", "png", 0.5)}


def relief_in_materials(target, image, span):
    """Plugs `image`, a height map with no height at one half and `span`
    either way, into the displacement of the materials of `target`. Returns
    how many materials were left alone because they have a displacement of
    their own."""
    atlas, height, displacement = HEIGHT_NODES
    mesh = target.data
    if not any(mesh.materials):
        made = bpy.data.materials.new(f"{target.name} relief")
        if len(mesh.materials):
            mesh.materials[0] = made
        else:
            mesh.materials.append(made)
    left_alone = 0
    for index, material in enumerate(mesh.materials):
        if material is None:
            continue
        if material.users > 1 and height not in (material.node_tree.nodes if material.node_tree else ()):
            # Shared with another object, the original most likely: the
            # result gets a copy of its own, or the other would be moved by a
            # map made for this mesh.
            material = material.copy()
            mesh.materials[index] = material
        material.use_nodes = True
        nodes, links = material.node_tree.nodes, material.node_tree.links
        output = next((node for node in nodes if node.type == "OUTPUT_MATERIAL" and node.is_active_output), None)
        if output is None:
            output = nodes.new("ShaderNodeOutputMaterial")
        socket = output.inputs["Displacement"]
        if socket.is_linked and socket.links[0].from_node.name != displacement:
            left_alone += 1
            continue
        where = nodes.get(atlas) or nodes.new("ShaderNodeUVMap")
        where.name, where.uv_map = atlas, UV_NAMES["atlas"]
        read = nodes.get(height) or nodes.new("ShaderNodeTexImage")
        read.name, read.image, read.extension = height, image, "EXTEND"
        move = nodes.get(displacement) or nodes.new("ShaderNodeDisplacement")
        move.name, move.space = displacement, "OBJECT"
        move.inputs["Midlevel"].default_value, move.inputs["Scale"].default_value = 0.5, 2.0 * span
        where.location, read.location, move.location = (-700, -400), (-500, -400), (-200, -400)
        links.new(where.outputs["UV"], read.inputs["Vector"])
        links.new(read.outputs["Color"], move.inputs["Height"])
        links.new(move.outputs["Displacement"], socket)
        # The surface itself is moved, and only moved: shading what is finer
        # than its vertices on top ("Displacement and Bump") draws a dark
        # line along every seam of the atlas, and a surface diced to the
        # pixel has little left that is finer.
        material.displacement_method = "DISPLACEMENT"
    return left_alone


def relief_out_of_materials(target):
    """Takes out of the materials of `target` what `relief_in_materials` put
    there."""
    for material in target.data.materials:
        if material is None or material.node_tree is None:
            continue
        nodes = material.node_tree.nodes
        ours = [nodes[name] for name in HEIGHT_NODES if name in nodes]
        if ours:
            for node in ours:
                nodes.remove(node)
            material.displacement_method = "BUMP"


def write_relief(heights, span, target, kind, directory):
    """Writes the height map as a file: the heights as they are for a float
    format, brought between 0 and 1 for an integer one. Returns the path,
    the midlevel and the scale it is to be read with."""
    import os

    import numpy

    file_format, depth, extension, midlevel = RELIEF_FILES[kind]
    if directory.startswith("//") and not bpy.data.filepath:
        raise engine.EngineError("save the .blend file first, or choose a folder that is not beside it (//)")
    folder = bpy.path.abspath(directory)
    os.makedirs(folder, exist_ok=True)
    path = os.path.join(folder, f"{bpy.path.clean_name(target.name)}_height.{extension}")
    if midlevel == 0.0:
        values, scale = numpy.nan_to_num(heights, nan=0.0), 1.0
    else:
        values, scale = numpy.where(numpy.isnan(heights), 0.5, 0.5 + heights / (2.0 * span)), 2.0 * span
    rows, columns = heights.shape
    pixels = numpy.ones((rows, columns, 4), numpy.float32)
    pixels[..., :3] = values.astype(numpy.float32)[..., None]
    # Written by Blender, as numbers and not as colours: one channel, at the
    # depth asked for, through a scene of its own so that the settings of
    # the user's are not touched.
    image = bpy.data.images.new("openclare height file", columns, rows, alpha=False, float_buffer=True, is_data=True)
    scene = bpy.data.scenes.new("openclare height file")
    try:
        image.pixels.foreach_set(pixels.reshape(-1))
        wanted = scene.render.image_settings
        wanted.file_format, wanted.color_mode, wanted.color_depth = file_format, "BW", depth
        if file_format == "OPEN_EXR":
            wanted.exr_codec = "ZIP"
        image.save_render(path, scene=scene)
    finally:
        bpy.data.images.remove(image)
        bpy.data.scenes.remove(scene)
    return path, midlevel, scale


class OPENCLARE_OT_bake_relief(bpy.types.Operator):
    """Bake the relief of the original onto the result, as a height map on its atlas that displaces it"""

    bl_idname = "object.openclare_bake_relief"
    bl_label = "Bake Height Map"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        target = context.active_object
        return (
            target is not None and target.type == "MESH" and context.mode == "OBJECT"
            and target.openclare_source is not None and target.openclare_source.type == "MESH"
        )

    def execute(self, context):
        import numpy

        settings = context.scene.openclare
        target, resolution = context.active_object, int(settings.relief_resolution)
        try:
            heights, report = relief_of(context, target, target.openclare_source, settings.relief_levels, resolution)
        except engine.EngineError as error:
            self.report({"ERROR"}, f"openclare: {error}")
            return {"CANCELLED"}

        # As an image: grey at half for no height, the greatest height either
        # way at black or white, which is what a displacement reads.
        span = max(abs(float(report["low"])), abs(float(report["high"])), 1e-12)
        grey = numpy.where(numpy.isnan(heights), 0.5, 0.5 + heights / (2.0 * span)).astype(numpy.float32)
        name = f"{target.name} height"
        image = bpy.data.images.get(name)
        if image is None or tuple(image.size) != (resolution, resolution):
            if image is not None:
                bpy.data.images.remove(image)
            image = bpy.data.images.new(name, resolution, resolution, alpha=False, float_buffer=True, is_data=True)
        pixels = numpy.ones((resolution, resolution, 4), numpy.float32)
        pixels[..., :3] = grey[..., None]
        image.pixels.foreach_set(pixels.reshape(-1))
        image.update()
        image.pack()

        # One way of moving the surface at a time: both would move it twice.
        subdivision, left_alone = target.modifiers[SUBDIVISION], 0
        if settings.relief_apply == "MODIFIER":
            relief_out_of_materials(target)
            subdivision.use_adaptive_subdivision = False
            texture = bpy.data.textures.get(name) or bpy.data.textures.new(name, "IMAGE")
            texture.image = image
            texture.extension = "EXTEND"
            relief = target.modifiers.get(RELIEF) or target.modifiers.new(RELIEF, "DISPLACE")
            relief.texture, relief.texture_coords, relief.uv_layer = texture, "UV", UV_NAMES["atlas"]
            relief.direction, relief.space = "NORMAL", "LOCAL"
            relief.mid_level, relief.strength = 0.5, 2.0 * span
            relief.show_viewport = True
        else:
            if RELIEF in target.modifiers:
                target.modifiers.remove(target.modifiers[RELIEF])
            # Diced to the size of a pixel by a renderer that can.
            subdivision.use_adaptive_subdivision = True
            left_alone = relief_in_materials(target, image, span)

        last_report.clear()
        last_report.update(report)
        last_report.update(applied_as=settings.relief_apply, materials_left_alone=str(left_alone))
        covered, astray = int(report["covered"]), int(report["astray"])
        said = f"openclare: height map of {resolution} x {resolution} in {float(report['seconds']):.1f} s"
        said += f", from {float(report['low']):.4g} to {float(report['high']):.4g}"
        if left_alone:
            said += f"; {left_alone} material(s) have a displacement of their own and were left as they are"
        if settings.relief_format != "NONE":
            try:
                path, midlevel, scale = write_relief(heights, span, target, settings.relief_format, settings.relief_directory)
            except (engine.EngineError, OSError, RuntimeError) as error:
                self.report({"ERROR"}, f"openclare: the height map is baked, but its file could not be written: {error}")
                return {"CANCELLED"}
            last_report.update(file=path, file_midlevel=str(midlevel), file_scale=repr(scale))
            target["openclare_height_file"] = f"{bpy.path.basename(path)}: midlevel {midlevel:g}, scale {scale:.6g}"
            said += f"; written to {path}, to read with midlevel {midlevel:g} and scale {scale:.6g}"
        if astray:
            # Said: there the relief is a guess, the sculpt being farther
            # from the result than a quad is long.
            said += f"; the original was out of reach under {100.0 * astray / max(covered, 1):.1f} % of the texels"
        self.report({"WARNING"} if astray > covered // 20 else {"INFO"}, said)
        return {"FINISHED"}


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
        layout.prop(settings, "target", expand=True)
        if settings.target == "SIZE":
            layout.operator(OPENCLARE_OT_analyse.bl_idname, icon="VIEWZOOM")
            sizes = layout.column(align=True)
            sizes.prop(settings, "largest_quad")
            sizes.prop(settings, "smallest_quad")
            owner = object_to_remesh(context)
            held = analysis_of(context, settings, owner) if owner is not None else None
            if held is None:
                note = "Analyse to see how many quads these sizes make" if owner is not None else ""
                if owner is not None and owner.name in ANALYSES:
                    note = "The object or its settings changed: analyse again"
                if note:
                    layout.label(text=note, icon="INFO")
            elif settings.largest_quad > 0.0:
                analysis, scale = held
                smallest = min(settings.smallest_quad, settings.largest_quad) or settings.largest_quad / SIZE_RANGE
                try:
                    quads, below = counted(analysis, scale, settings.largest_quad, smallest)
                except engine.EngineError:
                    quads, below = None, None
                if quads is not None:
                    column = layout.column(align=True)
                    column.label(text=f"About {quads:,.0f} quads".replace(",", " "), icon="MESH_GRID")
                    if below >= 0.005:
                        column.label(text=f"{100.0 * below:.0f} % coarser than its shape asks", icon="ERROR")
                layout.prop(settings, "show_sizes")
        else:
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


class OPENCLARE_PT_relief(bpy.types.Panel):
    """The relief of the original, baked onto the result"""

    bl_label = "Relief"
    bl_idname = "OPENCLARE_PT_relief"
    bl_parent_id = "OPENCLARE_PT_panel"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"
    bl_category = "openclare"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        settings = context.scene.openclare
        target = context.active_object
        source = getattr(target, "openclare_source", None)
        if source is None:
            layout.label(text="Select a result of Remesh to Quads", icon="INFO")
        else:
            layout.label(text=f"From: {source.name}", icon="MESH_DATA")
        layout.prop(settings, "relief_levels")
        layout.prop(settings, "relief_resolution")
        layout.prop(settings, "relief_apply")
        layout.prop(settings, "relief_format")
        folder = layout.row()
        folder.enabled = settings.relief_format != "NONE"
        folder.prop(settings, "relief_directory")
        layout.operator(OPENCLARE_OT_bake_relief.bl_idname, icon="RNDCURVE")
        written = target.get("openclare_height_file") if target is not None else None
        if written:
            # How the file last written is to be read, where it is used.
            layout.label(text=written, icon="FILE_IMAGE")


CLASSES = (
    OpenclareSettings,
    OPENCLARE_OT_analyse,
    OPENCLARE_OT_remesh,
    OPENCLARE_OT_bake_relief,
    OPENCLARE_OT_curves_assign,
    OPENCLARE_OT_curves_remove,
    OPENCLARE_OT_curves_select,
    OPENCLARE_PT_panel,
    OPENCLARE_PT_curves,
    OPENCLARE_PT_export,
    OPENCLARE_PT_relief,
)


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Scene.openclare = bpy.props.PointerProperty(type=OpenclareSettings)
    bpy.types.Object.openclare_source = bpy.props.PointerProperty(
        name="Original",
        description="The object this one was remeshed from: what is baked onto it is read from there",
        type=bpy.types.Object,
    )


def unregister():
    if bpy.context.screen is not None:
        stop_preview(bpy.context)
    ANALYSES.clear()
    del bpy.types.Object.openclare_source
    del bpy.types.Scene.openclare
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
