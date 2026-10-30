"""openclare: automatic quad retopology, as a Blender add-on (Community
edition).

A panel in the Properties editor, on the tab of the mesh of an object
(Object Data), remeshes the object into quads with the settings it keeps.
The result is a new object, next to the original, which is left as it is;
shown, the same panel says what it was made from and how, and remeshes it
again. A toggle beside the angle of the hard edges shows in the 3D viewport
the edges the quads will follow, and those left out (see `preview`). Curves
drawn on the model, with the tools of the toolbar (see `draw`) or by hand,
give the result lines of edges, or of hard edges, along them.
"""

import math
import os
import time

import bpy
import mathutils
from bl_operators.presets import AddPresetBase
from bl_ui.utils import PresetPanel

from . import draw, engine, extend, poles, preview

PRESETS = [
    ("organic", "Organic", "Sculpts and characters: edges follow the shape closely, quads are smaller where it bends"),
    ("animation", "Animation", "Meshes meant to deform: fewer irregular vertices, at some cost in fidelity"),
    ("hard-surface", "Hard surface", "Mechanical parts: edges sharper than 30 degrees are kept exactly"),
]
# The shapes the engine's presets of these names stand for, as the values of
# the settings they set. The add-on ships no preset since 0.6: with hard edges
# followed by default they hardly differ from the defaults. Files of 0.1,
# which named one of these, are still given its values.
SHAPES = {
    "organic": dict(adaptive_size=50.0, quads_around=6, use_hard_edges=True, hard_edge_angle=math.radians(30.0),
                    follow_curvature=0.05, fewer_irregular=2.0, volume_fit=1.0),
    "animation": dict(adaptive_size=50.0, quads_around=6, use_hard_edges=True, hard_edge_angle=math.radians(30.0),
                      follow_curvature=0.05, fewer_irregular=3.0, volume_fit=1.0),
    "hard-surface": dict(adaptive_size=0.0, quads_around=0, use_hard_edges=True, hard_edge_angle=math.radians(30.0),
                         follow_curvature=0.02, fewer_irregular=2.0, volume_fit=1.0),
}
# The settings a preset of the shape holds, in the order of the panel.
SHAPE_NAMES = [*SHAPES["organic"], "use_marked_sharp", "shading"]
# Where the presets of the shape are: the one the add-on ships, Default
# (presets/ here), and those a user saves, under presets/ in Blender's own
# folder.
SHAPE_PRESETS = "openclare_shape"
ADDON_FOLDER = os.path.dirname(os.path.abspath(__file__))
# What the engine is given under these values: the shapes are said in full.
BASE_PRESET = "organic"


# The colour a curve drawn on a model takes in the viewport, by the marks it
# gives the edges along it: a seam, else sharp or creased, else none (a line
# of edges and nothing more).
CURVE_COLOURS = {"seam": (1.0, 0.35, 0.2, 1.0), "hard": (0.6, 0.25, 1.0, 1.0), "line": (0.0, 0.85, 1.0, 1.0)}


def curve_colour(sharp, crease, seam):
    return CURVE_COLOURS["seam" if seam else "hard" if sharp or crease > 0.0 else "line"]

# What the engine said of the last run of the operator, name by name: for a
# script that drives the add-on. Empty until a run ends well.
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


# Settings that are not copied from one object to another: what is shown,
# and the object files of 0.1 named in the settings of their scene.
NOT_COPIED = {"rna_type", "name", "source", "show_sizes"}


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
    owner = settings.id_data
    if isinstance(owner, bpy.types.Object) and owner.type == "MESH":
        settings.largest_quad, settings.smallest_quad = sizes_from(owner)


# Shapes analysed: by object name, the engine's analysis, what it was made
# from (to tell when it no longer holds), and the scale of the object then.
ANALYSES = {}
# The last count asked of an analysis, which the panel draws again and again.
COUNTED = {}
# The preview of the sizes: the object coloured, the colour attributes that
# were its active and rendered ones, and how the 3D views coloured objects.
PREVIEW = {}
SIZES_ATTRIBUTE = "openclare sizes"
# Sequential colours of the sizes, light for the largest quads to dark for the
# smallest, and the colour of what is coarser than its shape asks.
SIZE_RAMP = ["#cde2fb", "#9ec5f4", "#6da7ec", "#3987e5", "#256abf", "#184f95", "#0d366b"]
COARSER = "#e34948"


def analysis_key(settings, owner, evaluated_mesh):
    """What an analysis depends on: the mesh as seen and the settings that
    say what the shape calls for."""
    vertices = len(evaluated_mesh.vertices)
    corner = tuple(round(c, 5) for c in evaluated_mesh.vertices[0].co) if vertices else ()
    return (owner.name, vertices, len(evaluated_mesh.polygons), corner, tuple(shape_of(settings).items()),
            settings.density_group, round(settings.density_factor, 3))


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
    """Colours the object with the sizes asked for now."""
    import numpy

    owner = bpy.data.objects.get(PREVIEW.get("of", ""))
    held = ANALYSES.get(PREVIEW.get("of", ""))
    if owner is None or held is None or owner.type != "MESH":
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
    if PREVIEW.get("nearest") is not None:
        colours = colours[PREVIEW["nearest"]]
    mesh = owner.data
    if len(colours) != len(mesh.vertices):
        return
    attribute = mesh.color_attributes.get(SIZES_ATTRIBUTE) or mesh.color_attributes.new(
        SIZES_ATTRIBUTE, "FLOAT_COLOR", "POINT"
    )
    attribute.data.foreach_set("color", colours.astype(numpy.float32).ravel())
    mesh.color_attributes.active_color_name = SIZES_ATTRIBUTE
    mesh.color_attributes.default_color_name = SIZES_ATTRIBUTE
    mesh.update()


def stop_preview(context, untick=True):
    """Takes the colours of the sizes away: the colour attributes of the
    object are as they were, and the 3D views colour as they did. Its box
    Show Sizes is unticked, unless `untick` says not to."""
    owner = bpy.data.objects.get(PREVIEW.pop("of", ""))
    active, default = PREVIEW.pop("active", ""), PREVIEW.pop("default", "")
    PREVIEW.pop("nearest", None)
    if owner is not None and owner.type == "MESH":
        colours = owner.data.color_attributes
        if SIZES_ATTRIBUTE in colours:
            colours.remove(colours[SIZES_ATTRIBUTE])
        if active in colours:
            colours.active_color_name = active
        if default in colours:
            colours.default_color_name = default
        owner.data.update()
        if untick and owner.openclare.show_sizes:
            owner.openclare.show_sizes = False
    for space, colour_type in PREVIEW.pop("views", []):
        try:
            space.shading.color_type = colour_type
        except ReferenceError:
            pass


def nearest_seen(mesh, seen):
    """For each vertex of `mesh`, the nearest vertex of `seen`, the mesh as
    its modifiers make it: a numpy array, or None when the two have as many
    vertices, each then being its own."""
    import numpy
    from mathutils.kdtree import KDTree

    if len(seen.vertices) == len(mesh.vertices):
        return None
    tree = KDTree(len(seen.vertices))
    for vertex in seen.vertices:
        tree.insert(vertex.co, vertex.index)
    tree.balance()
    return numpy.array([tree.find(vertex.co)[1] for vertex in mesh.vertices])


def start_preview(context, settings, owner):
    """Colours the object with its sizes, in a colour attribute of its own
    made the active and the rendered one; those that were are kept to be
    given back."""
    stop_preview(context)
    colours = owner.data.color_attributes
    PREVIEW.update(
        of=owner.name, scale=scale_of(owner), active=colours.active_color_name or "",
        default=colours.default_color_name or "", nearest=None,
    )
    # The sizes are known at the vertices of the mesh as seen. Where its
    # modifiers make other vertices, each of the object takes the nearest.
    evaluated = owner.evaluated_get(context.evaluated_depsgraph_get())
    seen = evaluated.to_mesh()
    try:
        PREVIEW["nearest"] = nearest_seen(owner.data, seen)
    finally:
        evaluated.to_mesh_clear()
    # The 3D views show the colour attribute meanwhile ("VERTEX", shown as
    # Attribute).
    PREVIEW["views"] = []
    for area in context.screen.areas if context.screen else []:
        if area.type == "VIEW_3D":
            space = area.spaces.active
            PREVIEW["views"].append((space, space.shading.color_type))
            space.shading.color_type = "VERTEX"
    colour_sizes(settings)


@bpy.app.handlers.persistent
def before_saving(*_):
    """No colours of the sizes in the file saved: taken away, then put back."""
    PREVIEW["saved"] = PREVIEW.get("of")
    if PREVIEW["saved"]:
        views = PREVIEW.get("views", [])
        stop_preview(bpy.context, untick=False)
        PREVIEW["views_saved"] = views


@bpy.app.handlers.persistent
def after_saving(*_):
    owner = bpy.data.objects.get(PREVIEW.pop("saved", None) or "")
    views = PREVIEW.pop("views_saved", [])
    if owner is not None:
        start_preview(bpy.context, owner.openclare, owner)
        # The views were given back their colouring before saving: what they
        # had before the preview is what to give back at the end.
        PREVIEW["views"] = views


@bpy.app.handlers.persistent
def after_loading(*_):
    """Another file: what was analysed and shown belonged to the last one."""
    PREVIEW.clear()
    ANALYSES.clear()
    COUNTED.clear()
    for scene in bpy.data.scenes:
        adopt_scene_settings(scene)
    for owner in bpy.data.objects:
        adopt_preset(owner.openclare)
        if owner.openclare.show_sizes:
            owner.openclare.show_sizes = False


def sizes_changed(settings, context):
    """The sizes moved: the preview follows."""
    if PREVIEW.get("of") and PREVIEW["of"] == settings.id_data.name:
        colour_sizes(settings)


def show_sizes_changed(settings, context):
    if not settings.show_sizes:
        stop_preview(context)
        return
    owner = settings.id_data
    if not isinstance(owner, bpy.types.Object) or owner.type != "MESH" or owner.mode == "EDIT" \
            or analysis_of(context, settings, owner) is None:
        # Nothing to show before the object is analysed as it is, nor while
        # it is edited.
        settings.show_sizes = False
        return
    start_preview(context, settings, owner)


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
    output: bpy.props.EnumProperty(
        options=set(),
        name="Output",
        description="What the object is remeshed into",
        items=[
            ("QUADS", "Quads", "Quads, with edge loops that follow the shape: to model, sculpt on, animate"),
            (
                "TRIANGLES",
                "Triangles",
                "Triangles of one size or as the shape asks, close to equilateral: for simulation, 3D printing, "
                "collision",
            ),
        ],
        default="QUADS",
    )
    triangles_for: bpy.props.EnumProperty(
        options=set(),
        name="For",
        description="What the triangles are for, which says what makes them",
        items=[
            (
                "SIMULATION",
                "Simulation",
                "Triangles close to equilateral, made by openclare: for cloth and soft bodies, an even surface",
            ),
            (
                "SHAPE",
                "Shape",
                "Triangles that keep closest to the shape, stretched where it needs them, made by Blender's "
                "Decimate (Collapse): for printing and collision meshes. Hard edges, curves and materials are not "
                "followed",
            ),
        ],
        default="SIMULATION",
    )
    target: bpy.props.EnumProperty(
        options=set(),
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
        options=set(),
        name="Quads",
        description="Number of quads asked for",
        default=3000,
        min=1,
        soft_min=100,
        soft_max=50_000,
    )
    largest_quad: bpy.props.FloatProperty(
        options=set(),
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
        options=set(),
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
        options=set(),
        name="Show Sizes",
        description=(
            "Colour the object with the size its quads will have: light where they are largest, dark where "
            "they are smallest, red where its shape calls for quads smaller than the smallest. A colour "
            "attribute of its own, made the active and rendered one meanwhile, shown in Solid view by "
            "attribute; taken away when unticked, the colour attributes of the object as they were"
        ),
        default=False,
        update=show_sizes_changed,
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
            "Edges of the object sharper than the angle, and still sharp at the size of the quads, are "
            "followed by the quads wherever they can, and kept crisp"
        ),
        default=True,
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
    use_marked_sharp: bpy.props.BoolProperty(
        options=set(),
        name="Marked Sharp",
        description=(
            "Edges of the object marked sharp (Mark Sharp, or a Smooth by Angle modifier) are followed by "
            "the quads and kept as hard edges, whatever their angle and size"
        ),
        default=False,
    )
    shading: bpy.props.EnumProperty(
        options=set(),
        name="Shading",
        description="How the faces of the result are shaded",
        items=[
            (
                "BY_ANGLE", "By Angle",
                "Smooth, but sharp across the edges sharper than the angle of the hard edges: a sculpt stays "
                "smooth, a mechanical part keeps its edges crisp",
            ),
            ("SMOOTH", "Smooth", "Every face smooth"),
            ("FLAT", "Flat", "Every face flat"),
        ],
        default="BY_ANGLE",
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
    density_group: bpy.props.StringProperty(
        options=set(),
        name="Density",
        description=(
            "Vertex group painted with the density wanted (Weight Paint): quads are smallest where the weight "
            "is 1, largest where it is 0. Leave empty for none"
        ),
    )
    density_factor: bpy.props.FloatProperty(
        options=set(),
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
    shape_adaptive_size: bpy.props.FloatProperty(
        options={"HIDDEN"},
        name="Adaptive Size Written",
        description=(
            "The Adaptive Size Density from Shape read the shape with before it switched it off: what it reads "
            "the shape with again while Adaptive Size stays off and its group is the Density"
        ),
        subtype="PERCENTAGE",
        default=0.0,
        min=0.0,
        max=100.0,
    )
    symmetry: bpy.props.EnumProperty(
        options=set(),
        name="Symmetry",
        description=(
            "The quads come out the same on both sides of the plane through the origin of the object across "
            "this axis, in the object's own axes. The object must be the same on both sides"
        ),
        items=[
            ("NONE", "None", "No plane of symmetry"),
            ("X", "X", "The plane across the object's X axis"),
            ("Y", "Y", "The plane across the object's Y axis"),
            ("Z", "Z", "The plane across the object's Z axis"),
        ],
        default="NONE",
    )
    export_materials: bpy.props.BoolProperty(
        options=set(),
        name="Materials",
        description=(
            "Give each quad of the result the material of the surface it lies on, and keep the borders "
            "between materials as lines of edges where that can be done. Has no effect on an object of "
            "one material"
        ),
        default=True,
    )
    export_sharp_edges: bpy.props.BoolProperty(
        options=set(),
        name="Sharp Edges",
        description=(
            "Mark as sharp the edges of the result that follow a hard edge of the original "
            "(see Hard Edges and Marked Sharp). The edges along a curve drawn on it take the marks of the curve"
        ),
        default=True,
    )
    export_creases: bpy.props.BoolProperty(
        options=set(),
        name="Creases",
        description=(
            "Give a crease weight to the edges that should stay sharp when the result is subdivided: "
            "those along a hard edge of the original, and those where "
            "the original folds sharply. "
            "A Subdivision Surface modifier reads them"
        ),
        default=True,
    )



def curve_mesh_changed(settings, context):
    """The mesh of a curve set from its panel: the curve is made its child
    where it is, and held on it."""
    drawn, mesh = settings.id_data, settings.mesh
    if not isinstance(drawn, bpy.types.Object) or drawn.parent == mesh:
        return
    where = drawn.matrix_world.copy()
    drawn.parent = mesh
    if mesh is not None:
        drawn.matrix_parent_inverse = mesh.matrix_world.inverted()
    drawn.matrix_world = where
    wrap = drawn.modifiers.get(SHRINKWRAP)
    if wrap is not None:
        wrap.target = mesh


def curve_marks_changed(settings, context):
    drawn = settings.id_data
    if isinstance(drawn, bpy.types.Object):
        drawn.color = curve_colour(settings.sharp, settings.crease, settings.seam)


class OpenclareCurveSettings(bpy.types.PropertyGroup):
    """What a curve drawn on a mesh gives the result: a line of edges along
    it, always, and the marks of those edges."""

    mesh: bpy.props.PointerProperty(
        options=set(),
        name="Mesh",
        type=bpy.types.Object,
        poll=is_a_mesh,
        description=(
            "The mesh the result of which has a line of edges along this curve: its parent. Set here, the "
            "curve is made its child where it is, and its Shrinkwrap holds it on it"
        ),
        update=curve_mesh_changed,
    )
    sharp: bpy.props.BoolProperty(
        options=set(),
        name="Sharp",
        description="Mark the edges of the result along this curve sharp",
        update=curve_marks_changed,
    )
    crease: bpy.props.FloatProperty(
        options=set(),
        name="Crease",
        description=(
            "Crease weight of the edges of the result along this curve, which a Subdivision Surface modifier "
            "reads: 1 keeps them sharp, 0 leaves them smooth"
        ),
        subtype="FACTOR",
        default=0.0,
        min=0.0,
        max=1.0,
        update=curve_marks_changed,
    )
    seam: bpy.props.BoolProperty(
        options=set(),
        name="Seam",
        description="Mark the edges of the result along this curve as seams, for the UVs to be cut along it",
        update=curve_marks_changed,
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


def stretch_of(owner):
    """The scale of `owner` along its own three axes when they differ: what its
    mesh is multiplied by before the engine sees it, for the quads to come out
    square in the scene, and divided by after. None when the scale is the same
    along all three, which only changes the sizes asked for (see
    `engine_settings_of`)."""
    import numpy

    scale = [abs(factor) for factor in owner.matrix_world.to_scale()]
    if max(scale) - min(scale) <= 1e-6 * max(scale):
        return None
    return numpy.array(scale, numpy.float32)


def marked_sharp(mesh):
    """The edges of `mesh` marked sharp, two vertices each, as unsigned 32-bit
    numbers; None if there are none."""
    import numpy

    attribute = mesh.attributes.get("sharp_edge")
    if attribute is None or attribute.domain != "EDGE" or not len(mesh.edges):
        return None
    sharp = numpy.zeros(len(mesh.edges), bool)
    attribute.data.foreach_get("value", sharp)
    if not sharp.any():
        return None
    ends = numpy.empty(2 * len(mesh.edges), numpy.int32)
    mesh.edges.foreach_get("vertices", ends)
    return ends.reshape(-1, 2)[sharp].astype(numpy.uint32).ravel()


def engine_settings_of(original, evaluated, source, write=True):
    """What the engine is given for the object `original` with its settings:
    the settings on top of `BASE_PRESET`, and the weights of the density
    painted, if any, on `source`, the mesh of `evaluated`, `original` as it
    is seen. Sizes asked for and none set are set from the object, and kept
    in its settings unless `write` is false."""
    settings = original.openclare
    engine_settings = {"quads": settings.quads, **shape_of(settings)}
    if settings.target == "SIZE":
        # Lengths of the scene, where the engine is given the mesh as the
        # object holds it: before its scale, when the same along all axes.
        largest, smallest = settings.largest_quad, settings.smallest_quad
        if largest <= 0.0:
            largest, smallest = sizes_from(original)
            if write:
                settings.largest_quad, settings.smallest_quad = largest, smallest
        # A scale that differs along the axes is applied to the mesh itself.
        uniform = stretch_of(original) is None
        scale = sum(abs(factor) for factor in original.matrix_world.to_scale()) / 3.0 if uniform else 1.0
        scale = scale or 1.0
        smallest = min(smallest, largest) or largest / SIZE_RANGE
        del engine_settings["quads"]
        engine_settings["largest_quad"] = largest / scale
        engine_settings["smallest_quad"] = smallest / scale
    if settings.symmetry != "NONE":
        engine_settings["symmetry"] = settings.symmetry.lower()
    if settings.output == "TRIANGLES":
        engine_settings["method"] = "triangles"
    density = None
    if settings.density_group:
        density = weights_of(evaluated, source, settings.density_group)
        engine_settings["density_contrast"] = settings.density_factor
    return engine_settings, density


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


def curve_objects(owner):
    """The curves drawn on `owner`: its children that are curves."""
    return [drawn for drawn in owner.children if drawn.type == "CURVE"]


def curves_of(owner):
    """The curves of `owner` as the engine takes them, and their marks: for
    each stretch of each curve, its points in the space of `owner`, whether
    it closes on itself and whether it is sharp; and alongside, for each, its
    marks (sharp, crease, seam). Curves are taken as they are seen,
    modifiers applied: a Shrinkwrap is what keeps a curve on a model."""
    import numpy

    depsgraph = bpy.context.evaluated_depsgraph_get()
    into_owner = owner.matrix_world.inverted()
    curves, marks = [], []
    for drawn in curve_objects(owner):
        told = drawn.openclare_curve
        marked = (told.sharp, told.crease, told.seam)
        evaluated = drawn.evaluated_get(depsgraph)
        mesh = evaluated.to_mesh()
        if mesh is None:
            continue
        matrix = into_owner @ evaluated.matrix_world
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
            curves.append((numpy.array([tuple(points[v]) for v in chain], numpy.float32), closed, told.sharp))
            marks.append(marked)
    return curves, marks


def mark_curve_edges(mesh, curve_edges, marks, report):
    """Gives the edges of `mesh` along each curve the marks of that curve:
    `curve_edges` as the engine gives them (the edges, and the index of the
    curve of each), `marks` the (sharp, crease, seam) of each curve. Sharp
    and seam add to the marks there; the crease of a curve is the weight of
    its edges, whatever was there."""
    import numpy

    pairs, which = curve_edges
    if not len(which):
        return
    sharp, crease, seam = (numpy.array([m[k] for m in marks], numpy.float32)[which] for k in range(3))
    of_mesh, wanted = edge_keys(mesh, pairs)
    order = numpy.argsort(of_mesh)
    found = order[numpy.searchsorted(of_mesh[order], wanted)]
    for attribute, key, on in (("use_edge_sharp", "curve_sharp_edges", sharp > 0), ("use_seam", "curve_seams", seam > 0)):
        if on.any():
            flags = numpy.zeros(len(mesh.edges), bool)
            mesh.edges.foreach_get(attribute, flags)
            flags[found[on]] = True
            mesh.edges.foreach_set(attribute, flags)
            report[key] = int(on.sum())
    attribute = mesh.attributes.get("crease_edge")
    if attribute is None and (crease > 0).any():
        attribute = mesh.attributes.new("crease_edge", "FLOAT", "EDGE")
    if attribute is not None:
        values = numpy.zeros(len(mesh.edges), numpy.float32)
        attribute.data.foreach_get("value", values)
        values[found] = crease
        attribute.data.foreach_set("value", values)


def quads_of(
    source, preset, settings, density=None, on_stage=None, materials=False, curves=(), sharp_edges=False,
    creases=False,
):
    """A new mesh datablock: `source`, an evaluated mesh, remeshed into quads
    with the engine's `settings` on top of `preset`, and `density` if given
    (one weight per vertex of `source`), and the report of the run.
    `on_stage`, if given, is told the name of each stage as it begins.
    `materials` gives the mesh the materials of `source`, each face that of
    the surface it lies on, with the borders between them kept as lines of
    edges. `curves` are guide curves, as `curves_of` gives them, in the space
    of `source`. `sharp_edges` marks as sharp the edges that follow a hard
    edge of `source` or a sharp curve; `creases` gives a crease weight to
    the edges that should stay sharp when the mesh is subdivided. The three
    steps, for a run that does not wait: `engine_input`, `engine_run`,
    `mesh_of`."""
    given = engine_input(
        source, preset, settings, density, materials, curves=curves, sharp_edges=sharp_edges, creases=creases
    )
    return mesh_of(engine_run(given, on_stage), given)


class EngineInput:
    """What a run of the engine is given, read from Blender beforehand: the
    run itself touches nothing of Blender, and can go on in a thread of its
    own when the arrays are copies."""


def engine_input(
    source, preset, settings, density=None, materials=False, copy=False, sharp=False, stretch=None, curves=(),
    sharp_edges=False, creases=False, asked=None, curve_marks=(), poles=(),
):
    """The `EngineInput` of `quads_of`. With `copy`, the arrays of `source`
    are copied out, so that the run does not depend on `source` living as
    long as it does; otherwise they are read in place where they can be.
    `sharp` gives the engine the edges marked sharp, kept as hard edges;
    `stretch`, if given, the three factors the positions are multiplied by
    (see `stretch_of`), which copies them, and the curves with them.
    `asked` is what more the engine is to give, for an add-on that includes
    this one (see `extend`). `poles` are the poles placed by hand, as
    `poles.placed` gives them."""
    import numpy

    source.calc_loop_triangles()
    vertices, corners, triangles = len(source.vertices), len(source.loops), len(source.loop_triangles)
    if triangles == 0:
        raise engine.EngineError("the object has no faces")
    if not copy and stretch is None and in_place(source):
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
        if stretch is not None:
            kept[0].reshape(-1, 3)[:] *= stretch
        addresses = tuple(array.ctypes.data for array in kept)
    groups = None
    if materials and len(source.materials) > 1:
        # The material of each triangle; an object all of one is left alone.
        groups = numpy.empty(triangles, numpy.int32)
        source.loop_triangles.foreach_get("material_index", groups)
        groups = groups.astype(numpy.uint32) if groups.min() != groups.max() else None
    given = EngineInput()
    given.counts = (vertices, triangles, corners)
    given.addresses, given.kept, given.groups = addresses, kept, groups
    given.preset, given.settings, given.density, given.curves = preset, settings, density, list(curves)
    given.curve_marks, given.poles = list(curve_marks), list(poles)
    given.sharp_edges, given.creases, given.asked = sharp_edges, creases, dict(asked or {})
    given.edges = marked_sharp(source) if sharp else None
    given.stretch = stretch
    if stretch is not None:
        given.curves = [(points * numpy.float32(stretch), closed, sharp) for points, closed, sharp in curves]
    # The materials the faces of the result take, kept as references.
    given.materials = list(source.materials)
    return given


def engine_run(given, on_stage=None):
    """Runs the engine on an `EngineInput`; `on_stage` as in `quads_of`.
    Nothing of Blender is touched: with copied arrays, this can run in a
    thread of its own."""
    vertices, triangles, corners = given.counts
    return engine.remesh(
        given.addresses[0], vertices, given.addresses[1], triangles, given.addresses[2], corners, given.preset,
        given.settings, given.density, on_stage, hard_edges=given.sharp_edges, creases=given.creases,
        curves=given.curves, groups=given.groups, edges=given.edges, curve_edges=bool(given.curve_marks),
        poles=given.poles, **given.asked,
    )


def mesh_of(made, given):
    """The mesh datablock of what a run made, with what `given` asked for
    (materials, sharp edges, creases), and the report of the run."""
    import numpy

    mesh = bpy.data.meshes.new("openclare")
    positions = made.positions
    if given.stretch is not None:
        positions = (positions.reshape(-1, 3) / given.stretch).astype(numpy.float32).ravel()
    mesh.vertices.add(len(positions) // 3)
    mesh.vertices.foreach_set("co", positions)
    mesh.loops.add(len(made.corners))
    mesh.loops.foreach_set("vertex_index", made.corners.astype(numpy.int32))
    mesh.polygons.add(len(made.sizes))
    starts = numpy.zeros(len(made.sizes), numpy.int32)
    numpy.cumsum(made.sizes[:-1], out=starts[1:])
    mesh.polygons.foreach_set("loop_start", starts)
    mesh.polygons.foreach_set("loop_total", made.sizes.astype(numpy.int32))
    mesh.update(calc_edges=True)
    mesh.validate()
    report = made.report
    if made.face_groups is not None:
        for material in given.materials:
            mesh.materials.append(material)
        mesh.polygons.foreach_set("material_index", made.face_groups.astype(numpy.int32))
        report["materials"] = len(numpy.unique(made.face_groups))
    if given.sharp_edges:
        report["sharp_edges"] = flag_edges(mesh, "use_edge_sharp", made.hard_edges)
    if given.creases:
        report["creases"] = crease_edges(mesh, *made.creases)
    if made.curve_edges is not None:
        mark_curve_edges(mesh, made.curve_edges, given.curve_marks, report)
    return mesh, report


def shade(mesh, shading, angle):
    """The faces of `mesh` shaded as the setting `shading` says: flat,
    smooth, or smooth but sharp across the edges sharper than `angle` and
    those marked sharp already (along a curve drawn as sharp, say)."""
    import numpy

    if shading == "FLAT":
        mesh.shade_flat()
        return
    marked = numpy.zeros(len(mesh.edges), bool)
    mesh.edges.foreach_get("use_edge_sharp", marked)
    mesh.shade_smooth()
    if shading == "BY_ANGLE":
        # Blender sets the marks of the angle afresh: those there before
        # are added back.
        mesh.set_sharp_from_angle(angle=angle)
        by_angle = numpy.zeros(len(mesh.edges), bool)
        mesh.edges.foreach_get("use_edge_sharp", by_angle)
        mesh.edges.foreach_set("use_edge_sharp", by_angle | marked)


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

        owner = object_to_remesh(context)
        adopt_scene_settings(context.scene)
        settings = owner.openclare
        adopt_preset(settings)
        evaluated = owner.evaluated_get(context.evaluated_depsgraph_get())
        mesh = evaluated.to_mesh()
        try:
            mesh.calc_loop_triangles()
            positions = numpy.empty(3 * len(mesh.vertices), numpy.float32)
            mesh.vertices.foreach_get("co", positions)
            triangles = numpy.empty(3 * len(mesh.loop_triangles), numpy.uint32)
            mesh.loop_triangles.foreach_get("vertices", triangles)
            engine_settings = shape_of(settings)
            density = None
            if settings.density_group:
                density = weights_of(evaluated, mesh, settings.density_group)
                engine_settings["density_contrast"] = settings.density_factor
            key = analysis_key(settings, owner, mesh)
            started = time.perf_counter()
            analysis = engine.Analysis(positions, triangles, BASE_PRESET, engine_settings, density)
            seconds = time.perf_counter() - started
        except engine.EngineError as error:
            self.report({"ERROR"}, f"openclare: {error}")
            return {"CANCELLED"}
        finally:
            evaluated.to_mesh_clear()
        stop_preview(context)
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


# The vertex group Density from Shape writes.
SHAPE_DENSITY = "openclare density"
# Weights are written in steps of this much: one call to Blender per value.
WEIGHT_STEP = 1.0 / 1000.0


def weights_from_sizes(sizes, largest, factor):
    """The painted density that makes quads of the edges `sizes` where the
    largest would be `largest`, with a `factor` (Density's Factor, the
    engine's `density_contrast`): one weight per size, from 0 to 1.

    The engine reads a weight w as a density `factor`**w times that of the
    weight 0, per unit of area (sizing.rs, `Demand::measure`): the edge of
    the quads is multiplied by `factor`**(-w / 2), the inverse square root.
    An edge s where the weight 0 has `largest` is then the weight
    w = 2 ln(largest / s) / ln(factor), kept between 0 and 1: no smaller
    than `largest` / sqrt(`factor`) can be painted. A size not known (not a
    number) is the weight 0. The engine takes the least weight painted for
    the weight 0, so only how the weights differ counts."""
    import numpy

    if not factor > 1.0:
        raise engine.EngineError("a Factor of 1 gives the paint no effect: raise it")
    sizes = numpy.nan_to_num(numpy.asarray(sizes, numpy.float64), nan=largest)
    weights = 2.0 * numpy.log(largest / numpy.maximum(sizes, 1e-30)) / math.log(factor)
    return numpy.clip(weights, 0.0, 1.0).astype(numpy.float32)


def write_weights(owner, name, weights):
    """Writes `weights`, one per vertex of `owner`, as its vertex group
    `name`: made if it is not there, emptied first if it is. Weights are
    rounded to `WEIGHT_STEP`, for Blender to be given each value once."""
    import numpy

    group = owner.vertex_groups.get(name)
    everyone = list(range(len(owner.data.vertices)))
    if group is None:
        group = owner.vertex_groups.new(name=name)
    else:
        group.remove(everyone)
    steps = numpy.rint(numpy.asarray(weights, numpy.float64) / WEIGHT_STEP).astype(numpy.int64)
    for step in numpy.unique(steps):
        group.add(numpy.flatnonzero(steps == step).tolist(), float(step * WEIGHT_STEP), "REPLACE")
    return group


class OPENCLARE_OT_density_from_shape(bpy.types.Operator):
    """Write the density the shape calls for (Adaptive Size, thin parts) as a vertex group, to paint on top in
    Weight Paint: made the Density, with Adaptive Size switched off for the shape not to count twice"""

    bl_idname = "object.openclare_density_from_shape"
    bl_label = "Density from Shape"
    bl_options = {"REGISTER", "UNDO"}

    @classmethod
    def poll(cls, context):
        owner = object_to_remesh(context)
        return owner is not None and source_of(owner) is None and owner.mode != "EDIT"

    def execute(self, context):
        """The shape as it is seen, modifiers applied, is analysed as Analyse
        does, without any density painted, and its sizes read at each vertex
        as a run would size it: between the sizes asked for (Sizes), the
        largest being the weight 0, or for the number asked for (Number), the
        largest found being the weight 0. They are written as weights
        (`weights_from_sizes`) into the vertex group `SHAPE_DENSITY`, which
        becomes the Density, and Adaptive Size is switched off. Quads Around
        Thin Parts stays as it is: a thin part is a bound on the size, which
        counts once however many times it is given, and keeps what is thinner
        than the Factor can paint. Read again while the group is the Density
        and Adaptive Size is off, the shape is read with the Adaptive Size
        that was switched off."""
        import numpy

        owner = object_to_remesh(context)
        adopt_scene_settings(context.scene)
        settings = owner.openclare
        adopt_preset(settings)
        factor = settings.density_factor
        if factor <= 1.0:
            self.report({"ERROR"}, "openclare: a Factor of 1 gives the paint no effect: raise it")
            return {"CANCELLED"}
        shape = shape_of(settings)
        adaptive = settings.adaptive_size
        if adaptive == 0.0 and settings.density_group == SHAPE_DENSITY and settings.shape_adaptive_size > 0.0:
            # Written before by this button, which switched it off: read
            # with what it was.
            adaptive = settings.shape_adaptive_size
            shape["adaptivity"] = round(adaptive / 100.0, 6)
        evaluated = owner.evaluated_get(context.evaluated_depsgraph_get())
        mesh = evaluated.to_mesh()
        try:
            mesh.calc_loop_triangles()
            positions = numpy.empty(3 * len(mesh.vertices), numpy.float32)
            mesh.vertices.foreach_get("co", positions)
            triangles = numpy.empty(3 * len(mesh.loop_triangles), numpy.uint32)
            mesh.loop_triangles.foreach_get("vertices", triangles)
            analysis = engine.Analysis(positions, triangles, BASE_PRESET, shape)
            nearest = nearest_seen(owner.data, mesh)
        except engine.EngineError as error:
            self.report({"ERROR"}, f"openclare: {error}")
            return {"CANCELLED"}
        finally:
            evaluated.to_mesh_clear()
        if settings.target == "SIZE" and settings.largest_quad > 0.0:
            scale = scale_of(owner)
            largest = settings.largest_quad / scale
            smallest = (min(settings.smallest_quad, settings.largest_quad) or settings.largest_quad / SIZE_RANGE) / scale
            sizes, _ = analysis.at_vertices(largest, smallest)
        else:
            sizes = analysis.at_vertices_for(settings.quads)
            largest = float(numpy.nanmax(sizes)) if numpy.isfinite(sizes).any() else 1.0
        weights = weights_from_sizes(sizes, largest, factor)
        # What the shape asks for and the Factor cannot paint: quads smaller
        # than the largest over its square root.
        beyond = float(numpy.mean(numpy.nan_to_num(sizes, nan=largest) < 0.999 * largest / math.sqrt(factor)))
        enough = (largest / float(numpy.nanmin(sizes))) ** 2 if beyond > 0.0 else factor
        if nearest is not None:
            weights = weights[nearest]
        write_weights(owner, SHAPE_DENSITY, weights)
        if settings.density_group != SHAPE_DENSITY or settings.adaptive_size > 0.0:
            settings.shape_adaptive_size = adaptive
        settings.density_group = SHAPE_DENSITY
        settings.adaptive_size = 0.0
        message = (
            f"openclare: '{SHAPE_DENSITY}' written from the shape (Adaptive Size {adaptive:.0f} %), "
            f"weights {weights.min():.2f} to {weights.max():.2f}; it is the Density now, and Adaptive Size is off"
        )
        if beyond >= 0.005:
            message += f"; {100.0 * beyond:.0f} % of it asks for smaller quads than the Factor gives"
            if enough <= OpenclareSettings.bl_rna.properties["density_factor"].hard_max:
                message += f" (a Factor of {math.ceil(enough)} would give them all)"
        self.report({"INFO"}, message)
        return {"FINISHED"}


# The custom property of a result where what the engine said of it is kept.
REPORT = "openclare_report"


def decimating(settings):
    """Whether `settings` ask for Blender's Decimate rather than the engine."""
    return settings.output == "TRIANGLES" and settings.triangles_for == "SHAPE"


def decimated(context, original):
    """A mesh datablock: `original` as it is seen, cut down to the number of
    triangles of its settings by Blender's Decimate in Collapse mode, with
    its symmetry, and its painted density kept denser (a weight of 1 is
    kept, 0 goes first: Decimate reads them the other way round); and the
    report of the run, said as the engine's is."""
    import time

    settings = original.openclare
    started = time.perf_counter()
    depsgraph = context.evaluated_depsgraph_get()
    mesh = bpy.data.meshes.new_from_object(original.evaluated_get(depsgraph))
    work = bpy.data.objects.new("openclare decimate", mesh)
    context.scene.collection.objects.link(work)
    try:
        before = sum(len(polygon.vertices) - 2 for polygon in mesh.polygons)
        modifier = work.modifiers.new("decimate", "DECIMATE")
        modifier.decimate_type = "COLLAPSE"
        modifier.ratio = min(1.0, settings.quads / max(before, 1))
        modifier.use_collapse_triangulate = True
        if settings.symmetry != "NONE":
            modifier.use_symmetry, modifier.symmetry_axis = True, settings.symmetry
        if settings.density_group and settings.density_group in work.vertex_groups:
            modifier.vertex_group, modifier.invert_vertex_group = settings.density_group, True
        made = bpy.data.meshes.new_from_object(work.evaluated_get(context.evaluated_depsgraph_get()))
    finally:
        bpy.data.objects.remove(work)
        bpy.data.meshes.remove(mesh)
    # Faces of more than three sides that Collapse left are cut too.
    import bmesh

    cut = bmesh.new()
    cut.from_mesh(made)
    bmesh.ops.triangulate(cut, faces=[face for face in cut.faces if len(face.verts) > 3])
    cut.to_mesh(made)
    cut.free()
    report = {
        "faces": len(made.polygons),
        "quads": 0,
        "quads_asked": settings.quads,
        "method": "decimate",
        "seconds": round(time.perf_counter() - started, 3),
    }
    return made, report


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
        stop_preview(context)
        last_report.clear()
        # With its modifiers applied: what is seen is what is remeshed.
        evaluated = original.evaluated_get(context.evaluated_depsgraph_get())
        source = evaluated.to_mesh()
        try:
            engine_settings, density = engine_settings_of(original, evaluated, source)
            self.curves, marks = curves_of(original)
            given = engine_input(
                source, BASE_PRESET, engine_settings, density, settings.export_materials, copy,
                sharp=settings.use_marked_sharp, stretch=stretch_of(original), curves=self.curves, curve_marks=marks,
                sharp_edges=settings.export_sharp_edges, creases=settings.export_creases,
                asked=extend.asked(original), poles=poles.placed(original.data, stretch_of(original)),
            )
        except (engine.EngineError, ValueError) as error:
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
        if decimating(original.openclare):
            return self.decimate(context, original)
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
        return self.finish(context, original, made, given)

    def finish(self, context, original, made, given):
        """Makes the object of what the engine `made` from `original`, next
        to it and the active one, and says how it went. Returns it."""
        mesh, report = mesh_of(made, given)
        # The poles of the project: the touched ones kept, the free ones
        # those of this result. Triangles have none to keep.
        report["poles_placed"] = len(given.poles)
        report["poles_answered"] = 0
        if original.openclare.output != "TRIANGLES":
            report["poles_answered"] = poles.refresh(original.data, made, given.stretch)
        # For the extensions, once the result is in place (`extended`).
        self.made_from, self.engine_made = original, made
        settings = original.openclare
        shade(mesh, settings.shading, settings.hard_edge_angle)
        kind = ".tris" if settings.output == "TRIANGLES" else ".quads"
        mesh.name = f"{original.data.name}{kind}"
        remeshed = bpy.data.objects.new(f"{original.name}{kind}", mesh)
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
        said = f"openclare: {made_of(report)} in {seconds:.1f} s"
        if "materials" in report:
            said += f", {report['materials']} materials"
        if self.curves:
            along = int(report.get("guide_edges", 0)) + int(report.get("sharp_curve_edges", 0))
            said += f", {along} edges along the curves"
            if report.get("guide_gaps", "0") != "0":
                said += f" ({report['guide_gaps']} breaks)"
        welded, apart = int(report.get("welded_vertices", 0)), int(report.get("edges_taken_apart", 0))
        if welded or apart:
            # Said, because the result then has borders the object did not show.
            said += f"; the mesh had defects and was mended ({welded} vertices welded along torn borders"
            said += f", {apart} edges with more than two faces taken apart)"
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
        plainer, let_go = int(report.get("pieces_through_fallback", 0)), int(report.get("guides_let_go", 0))
        if plainer:
            said += f"; {plainer} piece(s) meshed by the plainer method, with more irregular vertices"
        if let_go:
            said += f"; hard edges or lines let go on {let_go} piece(s), which could not be meshed with them"
        off_plane = int(report.get("off_the_plane", 0))
        if off_plane:
            said += "; the line on the plane of symmetry was let go, and the halves meet across their quads"
        # Said as a warning: the result then lacks part of the object, or is
        # far from what was asked.
        failed = int(report.get("pieces_failed", 0))
        if failed:
            said += f"; {failed} of {report.get('pieces', '?')} pieces could not be meshed and are missing"
        short = falls_short(report)
        if short:
            said += f"; {short}"
        placed, answered = int(report["poles_placed"]), int(report["poles_answered"])
        if placed:
            said += f"; {answered} of the {placed} poles placed by hand"
        crossing = int(report.get("crossing_faces", 0))
        if crossing:
            said += f"; {crossing} triangles cut through another one: mend them before printing"
        self.report({"WARNING"} if failed or short or off_plane or crossing else {"INFO"}, said)
        return remeshed

    def decimate(self, context, original):
        """`run`, by Blender's Decimate instead of the engine (see
        `decimated`): the result is made and said as a run's is. The
        extensions are not called: there is nothing of the engine to give
        them."""
        adopt_scene_settings(context.scene)
        settings = original.openclare
        mesh, report = decimated(context, original)
        shade(mesh, settings.shading, settings.hard_edge_angle)
        mesh.name = f"{original.data.name}.tris"
        remeshed = bpy.data.objects.new(f"{original.name}.tris", mesh)
        remeshed.matrix_world = original.matrix_world
        remeshed.openclare_source = original
        remeshed[REPORT] = {key: str(value) for key, value in report.items()}
        for collection in original.users_collection or [context.collection]:
            collection.objects.link(remeshed)
        for other in context.selected_objects:
            other.select_set(False)
        remeshed.select_set(True)
        context.view_layer.objects.active = remeshed
        last_report.clear()
        last_report.update(report)
        self.made_from, self.engine_made = original, None
        self.report({"INFO"}, f"openclare: {made_of(report)} by Blender's Decimate (Collapse) in {report['seconds']} s")
        return remeshed

    def extended(self, context, result):
        """Hands `result`, in place, to the extensions (see `extend`), with
        what the engine made of the original; what they add to the report
        is kept on the result. One that fails is said, and passed by."""
        if not extend.EXTENSIONS or self.engine_made is None:
            return
        report = {key: result[REPORT][key] for key in result[REPORT].keys()} if REPORT in result else {}
        for extension in extend.EXTENSIONS:
            try:
                extension.made(context, self.made_from, result, self.engine_made, report)
            except Exception as error:  # noqa: BLE001 - whatever it is, the result stays
                self.report({"WARNING"}, f"openclare: {type(extension).__name__} failed: {error}")
        result[REPORT] = {key: str(value) for key, value in report.items()}
        last_report.update(report)


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
        if decimating(original.openclare):
            # Blender's own, and quick: no thread.
            return self.done(context, self.decimate(context, original))
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
            made = self.finish(context, self.original, job.made, job.given)
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
        if made is None:
            return {"CANCELLED"}
        self.extended(context, made)
        return {"FINISHED"}


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
            self.extended(context, made)
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
        self.extended(context, result)
        return {"FINISHED"}


# How far a drawn curve may stray from the stroke it is fitted on, in steps
# of the stroke (a few pixels of the viewport each, where it was drawn): the
# curve keeps as few control points as that allows.
FIT_STEPS = 1.0
# The points of each Bezier segment of a drawn curve as it is seen, and read
# by the engine: enough for a long segment to lie on the surface.
DRAWN_RESOLUTION = 24
SHRINKWRAP = "openclare on the surface"


def bezier_spline(data, points, closed, tolerance):
    """The one spline of the curve `data` made the Bezier curve of fewest
    control points within `tolerance` of `points`: smooth joints with
    aligned handles, corners with handles of their own."""
    from . import fit

    fitted = fit.fit(points, closed, tolerance)
    data.splines.clear()
    spline = data.splines.new("BEZIER")
    spline.bezier_points.add(len(fitted) - 1)
    for at, (point, left, right, corner) in zip(spline.bezier_points, fitted):
        at.handle_left_type = at.handle_right_type = "FREE"
        at.co, at.handle_left, at.handle_right = point, left, right
        if not corner:
            at.handle_left_type = at.handle_right_type = "ALIGNED"
    spline.use_cyclic_u = closed
    spline.resolution_u = DRAWN_RESOLUTION
    return spline


def drawn_curve(context, owner, points, closed, marks, tolerance):
    """A curve drawn on `owner`, fitted on the stroke `points` given in the
    space of `owner` (as the drawing tool lays them on its surface): a Bezier
    curve of few control points, child of `owner` so that it moves with it
    and is taken when it is remeshed, held on its surface by a Shrinkwrap so
    that moving its points keeps it there, and giving the edges along it
    `marks`, (sharp, crease, seam)."""
    data = bpy.data.curves.new("openclare curve", "CURVE")
    data.dimensions = "3D"
    bezier_spline(data, points, closed, tolerance)
    drawn = bpy.data.objects.new(data.name, data)
    drawn.parent = owner
    drawn.matrix_parent_inverse.identity()
    drawn.matrix_basis.identity()
    wrap = drawn.modifiers.new(SHRINKWRAP, "SHRINKWRAP")
    wrap.target = owner
    wrap.wrap_method = "NEAREST_SURFACEPOINT"
    # On the curve as it is seen, not on its control points: off, the whole
    # curve lies on the surface, not only its control points.
    wrap.use_apply_on_spline = False
    for collection in owner.users_collection or [context.scene.collection]:
        collection.objects.link(drawn)
    told = drawn.openclare_curve
    told.mesh = owner
    told.sharp, told.crease, told.seam = marks
    drawn.color = curve_colour(*marks)
    # Seen through the object it lies on, which hides half of a loop.
    drawn.show_in_front = True
    show_object_colours(context)
    return drawn


def show_object_colours(context):
    """Has the 3D viewports of the screen draw wires in the colour of their
    object, which is how the two categories are told apart."""
    for area in context.screen.areas if context.screen else ():
        if area.type == "VIEW_3D":
            area.spaces.active.shading.wireframe_color_type = "OBJECT"


class OPENCLARE_OT_curves_select(bpy.types.Operator):
    """Select the curves drawn on this object"""

    bl_idname = "object.openclare_curves_select"
    bl_label = "Select Curves"
    bl_options = {"REGISTER", "UNDO"}

    select: bpy.props.BoolProperty(default=True, options={"HIDDEN"})

    @classmethod
    def poll(cls, context):
        return object_to_remesh(context) is not None and context.mode == "OBJECT"

    @classmethod
    def description(cls, context, properties):
        return "Select the curves drawn on this object" if properties.select else "Deselect them"

    def execute(self, context):
        for drawn in curve_objects(object_to_remesh(context)):
            # Only what this view layer shows can be selected.
            # The object stays the active one: its panel stays shown.
            if drawn.name in context.view_layer.objects and (drawn.visible_get() or not self.select):
                drawn.select_set(self.select)
        return {"FINISHED"}


def marks_text(told):
    """The marks of a curve, in a few words."""
    said = [name for name, on in (("sharp", told.sharp), ("seam", told.seam)) if on]
    if told.crease > 0.0:
        said.append(f"crease {told.crease:.2f}")
    return ", ".join(said) or "line of edges"


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
        settings = owner.openclare
        layout.prop(settings, "output", expand=True)
        decimate = decimating(settings)
        if settings.output == "TRIANGLES":
            layout.prop(settings, "triangles_for", expand=True)
            if decimate:
                # The best there is for it is Blender's own: it is used.
                layout.label(text="By Blender's Decimate (Collapse): closest to the shape", icon="MOD_DECIM")
        target = layout.row()
        target.active = not decimate
        target.prop(settings, "target", expand=True)
        if settings.target == "SIZE" and not decimate:
            layout.operator(OPENCLARE_OT_analyse.bl_idname, icon="VIEWZOOM")
            sizes = layout.column(align=True)
            sizes.prop(settings, "largest_quad")
            sizes.prop(settings, "smallest_quad")
            held = analysis_of(context, settings, owner)
            if held is None:
                note = "Analyse to see how many quads these sizes make"
                if owner.name in ANALYSES:
                    note = "The object or its settings changed: analyse again"
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
                    # Triangles of the edge of a quad: four of them over the
                    # square root of three in its area.
                    made = f"{quads:,.0f} quads" if settings.output == "QUADS" else f"{quads / 0.4330127:,.0f} triangles"
                    column.label(text=f"About {made}".replace(",", " "), icon="MESH_GRID")
                    if below >= 0.005:
                        column.label(text=f"{100.0 * below:.0f} % coarser than its shape asks", icon="ERROR")
                layout.prop(settings, "show_sizes")
        else:
            layout.prop(settings, "quads", text="Triangles" if settings.output == "TRIANGLES" else "Quads")
        column = layout.column(align=True)
        column.prop_search(settings, "density_group", owner, "vertex_groups", icon="GROUP_VERTEX")
        factor = column.row()
        factor.enabled = bool(settings.density_group)
        factor.active = not decimate
        factor.prop(settings, "density_factor")
        column.operator(OPENCLARE_OT_density_from_shape.bl_idname, icon="MOD_VERTEX_WEIGHT")
        # What Decimate does not follow.
        followed = layout.column()
        followed.active = not decimate
        followed.prop(settings, "export_materials")
        layout = followed
        marks = layout.column(heading="Mark", align=True)
        marks.prop(settings, "export_sharp_edges", text="Sharp")
        marks.prop(settings, "export_creases", text="Creases")
        layout = self.layout
        layout.prop(settings, "symmetry", expand=True)
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


# A result with fewer quads than this share of those asked for is said to fall
# short: the engine keeps to the number within a few per cent otherwise.
SHORT_OF = 0.75


def of_triangles(told):
    """Whether what the engine said is of a result of triangles."""
    return int(told.get("quads", 0)) == 0 and int(told.get("faces", 0)) > 0


def made_of(told):
    """How many faces a result has, and of what: "1200 quads"."""
    if of_triangles(told):
        return f"{told.get('faces', '?')} triangles"
    return f"{told.get('quads', '?')} quads"


def falls_short(told):
    """What to say of a result with far fewer faces than were asked for, or
    nothing."""
    kind = "triangles" if of_triangles(told) else "quads"
    asked, made = int(told.get("quads_asked", 0)), int(told.get("faces" if kind == "triangles" else "quads", 0))
    if asked and made < SHORT_OF * asked:
        return f"only {made} {kind} of the {asked} asked for: part of the object could not be meshed as asked"
    return ""


def report_lines(told):
    """What the panel of a result says of how it was made: lines of text,
    each with its icon."""
    told = {key: told[key] for key in told.keys()}
    lines = []
    seconds = float(told.get("seconds", 0.0))
    lines.append((f"{made_of(told)} in {seconds:.1f} s", "MESH_GRID"))
    if told.get("method") == "decimate":
        lines.append(("By Blender's Decimate (Collapse)", "MOD_DECIM"))
    crossing = int(told.get("crossing_faces", 0))
    if crossing:
        lines.append((f"{crossing} triangles cut through another one", "ERROR"))
    wanted, below = float(told.get("quads_wanted", 0.0)), float(told.get("below_smallest", 0.0))
    if wanted > 0.0:
        lines.append((f"The shape called for {wanted:,.0f}".replace(",", " "), "INFO"))
        if below >= 0.005:
            lines.append((f"{100.0 * below:.0f} % coarser than its shape asks", "ERROR"))
    failed, boxed = int(told.get("pieces_failed", 0)), int(told.get("pieces_boxed", 0))
    if failed:
        lines.append((f"{failed} of {told.get('pieces', '?')} pieces could not be meshed", "ERROR"))
    if falls_short(told):
        lines.append((f"Only {told.get('quads')} of the {told.get('quads_asked')} quads asked for", "ERROR"))
    if boxed:
        lines.append((f"{boxed} part(s) smaller than a quad are boxes", "ERROR"))
    plainer, let_go = int(told.get("pieces_through_fallback", 0)), int(told.get("guides_let_go", 0))
    if plainer:
        lines.append((f"{plainer} piece(s) by the plainer method", "INFO"))
    if let_go:
        lines.append((f"Hard edges let go on {let_go} piece(s)", "INFO"))
    if int(told.get("off_the_plane", 0)):
        lines.append(("The halves meet across their quads", "ERROR"))
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
        # Decimate reads none of the shape.
        layout.active = not decimating(settings)
        layout.prop(settings, "adaptive_size", slider=True)
        layout.prop(settings, "quads_around")
        column = layout.column(heading="Hard Edges")
        row = column.row(align=True)
        row.prop(settings, "use_hard_edges", text="")
        angle = row.row(align=True)
        angle.active = settings.use_hard_edges
        angle.prop(settings, "hard_edge_angle", text="")
        # The preview of what the setting does, in the 3D viewport, and
        # where it is: a ring that turns while it is being computed, a tick
        # once what is drawn is for the settings as they are, and in words
        # below.
        wm = context.window_manager
        shown = wm.openclare_show_hard_edges
        row.prop(wm, "openclare_show_hard_edges", text="", icon="HIDE_OFF" if shown else "HIDE_ON")
        if shown and preview.is_about(object_to_remesh(context)):
            status = preview.state.status
            if status in preview.BUSY:
                # As wide as an icon, not a share of the row.
                ring = row.row()
                ring.ui_units_x = 1.0
                ring.progress(factor=preview.ring_factor(), type="RING", text="")
            elif status == preview.CURRENT:
                row.label(text="", icon="CHECKMARK")
            elif status == preview.FAILED:
                row.label(text="", icon="ERROR")
            said = preview.status_text()
            if said:
                column.label(text=said)
        layout.prop(settings, "use_marked_sharp")
        # What only the quads have: the triangles follow no direction.
        quads_only = layout.column()
        quads_only.active = settings.output == "QUADS"
        quads_only.prop(settings, "follow_curvature")
        quads_only.prop(settings, "fewer_irregular")
        quads_only.prop(settings, "volume_fit", slider=True)
        layout.prop(settings, "shading", expand=True)


class OPENCLARE_PT_curves(DataPanel, bpy.types.Panel):
    """Curves drawn on the model for the result to have edges along"""

    bl_label = "Curves"
    bl_idname = "OPENCLARE_PT_curves"
    bl_parent_id = "OPENCLARE_PT_panel"
    bl_options = {"DEFAULT_CLOSED"}

    @classmethod
    def poll(cls, context):
        return is_source(context)

    def draw(self, context):
        layout = self.layout
        owner = object_to_remesh(context)
        drawn = curve_objects(owner)
        if not drawn:
            layout.label(text="Draw Curve or Cut Volume in the toolbar (T), or parent a curve here", icon="INFO")
            return
        column = layout.column(align=True)
        for curve in drawn:
            row = column.row(align=True)
            row.label(text=curve.name, icon="OUTLINER_OB_CURVE")
            row.label(text=marks_text(curve.openclare_curve))
        row = layout.row(align=True)
        for text, select in (("Select", True), ("Deselect", False)):
            row.operator(OPENCLARE_OT_curves_select.bl_idname, text=text).select = select


class OPENCLARE_PT_poles(DataPanel, bpy.types.Panel):
    """The poles of the retopo project: those of the last result, and those
    touched by hand, which the next remesh keeps"""

    bl_label = "Poles"
    bl_idname = "OPENCLARE_PT_poles"
    bl_parent_id = "OPENCLARE_PT_panel"
    bl_options = {"DEFAULT_CLOSED"}

    @classmethod
    def poll(cls, context):
        return is_source(context)

    def draw(self, context):
        layout = self.layout
        project = object_to_remesh(context).data.openclare_poles
        if not project.items:
            layout.label(text="Remesh once: the poles of the result are kept here", icon="INFO")
            return
        layout.label(text=poles.summary(project))
        layout.prop(project, "show")
        row = layout.row(align=True)
        row.operator(poles.OPENCLARE_OT_poles_free.bl_idname)
        row.operator(poles.OPENCLARE_OT_poles_clear.bl_idname)


class OPENCLARE_PT_curve(DataPanel, bpy.types.Panel):
    """For a curve drawn on a mesh: which mesh, and the marks of the edges
    of its result along the curve"""

    bl_label = "openclare"
    bl_idname = "OPENCLARE_PT_curve"
    bl_order = -1

    @classmethod
    def poll(cls, context):
        drawn = getattr(context, "object", None)
        return drawn is not None and drawn.type == "CURVE"

    def draw_header(self, context):
        self.layout.label(text="", icon="MOD_REMESH")

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False
        drawn = context.object
        told = drawn.openclare_curve
        layout.prop(told, "mesh")
        if drawn.parent != told.mesh:
            # Parented otherwise (Object Properties, Ctrl+P): the parent is
            # what a run goes by.
            parent = drawn.parent.name if drawn.parent is not None else "nothing"
            layout.label(text=f"Its parent is {parent}: that is the mesh it guides", icon="INFO")
        column = layout.column(heading="Mark")
        column.prop(told, "sharp")
        column.prop(told, "seam")
        layout.prop(told, "crease", slider=True)


CLASSES = (
    OpenclareSettings,
    OPENCLARE_OT_analyse,
    OPENCLARE_OT_density_from_shape,
    OPENCLARE_OT_remesh,
    OPENCLARE_OT_remesh_again,
    OPENCLARE_OT_stop,
    OPENCLARE_OT_select_original,
    OPENCLARE_OT_shape_preset_add,
    OPENCLARE_PT_shape_presets,
    OpenclareCurveSettings,
    OPENCLARE_OT_curves_select,
    OPENCLARE_PT_panel,
    OPENCLARE_PT_shape,
    OPENCLARE_PT_curves,
    OPENCLARE_PT_poles,
    OPENCLARE_PT_curve,
)


HANDLERS = (
    (bpy.app.handlers.save_pre, before_saving),
    (bpy.app.handlers.save_post, after_saving),
    (bpy.app.handlers.load_post, after_loading),
)


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    preview.register()
    draw.register()
    poles.register()
    bpy.types.Object.openclare = bpy.props.PointerProperty(type=OpenclareSettings)
    bpy.types.Object.openclare_curve = bpy.props.PointerProperty(type=OpenclareCurveSettings)
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
    preview.unregister()
    draw.unregister()
    poles.unregister()
    bpy.utils.unregister_preset_path(ADDON_FOLDER)
    for handlers, handler in HANDLERS:
        if handler in handlers:
            handlers.remove(handler)
    stop_preview(bpy.context)
    ANALYSES.clear()
    del bpy.types.Object.openclare_source
    del bpy.types.Object.openclare_curve
    del bpy.types.Object.openclare
    del bpy.types.Scene.openclare
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
