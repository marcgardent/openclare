"""openclare: automatic quad retopology, as a Blender add-on.

A panel in the Properties editor, on the tab of the mesh of an object
(Object Data), remeshes the object into quads with the settings it keeps.
The result is a new object, next to the original, which is left as it is;
shown, the same panel says what it was made from and how, remeshes it again
and bakes the relief of the original onto it.
"""

import math
import time

import os

import bpy
import mathutils
from bl_operators.presets import AddPresetBase
from bl_ui.utils import PresetPanel

from . import engine

PRESETS = [
    ("organic", "Organic", "Sculpts and characters: edges follow the shape closely, quads are smaller where it bends"),
    ("animation", "Animation", "Meshes meant to deform: fewer irregular vertices, at some cost in fidelity"),
    ("hard-surface", "Hard surface", "Mechanical parts: edges sharper than 30 degrees are kept exactly"),
]
# How much the size of the quads follows the curvature in each preset, in
# per cent: what the slider goes back to when the preset changes.
ADAPTIVE_SIZE = {"organic": 50.0, "animation": 50.0, "hard-surface": 0.0}
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


def start_preview(context, settings, owner):
    """Colours the object with its sizes, in a colour attribute of its own
    made the active and the rendered one; those that were are kept to be
    given back."""
    import numpy
    from mathutils.kdtree import KDTree

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
        if len(seen.vertices) != len(owner.data.vertices):
            tree = KDTree(len(seen.vertices))
            for vertex in seen.vertices:
                tree.insert(vertex.co, vertex.index)
            tree.balance()
            PREVIEW["nearest"] = numpy.array([tree.find(vertex.co)[1] for vertex in owner.data.vertices])
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
    guide_curves: bpy.props.PointerProperty(
        options=set(),
        name="Guide Curves",
        type=bpy.types.Collection,
        description=(
            "Collection of curves drawn on the model: the result has a line of edges along each. "
            "Curves are taken with their modifiers, so a Shrinkwrap keeps them on the model. "
            "Leave empty for none"
        ),
    )
    sharp_curves: bpy.props.PointerProperty(
        options=set(),
        name="Sharp Curves",
        type=bpy.types.Collection,
        description=(
            "Collection of curves drawn on the model: the result has a line of hard edges along each, "
            "marked sharp and creased. Curves are taken with their modifiers, so a Shrinkwrap keeps them "
            "on the model. Leave empty for none"
        ),
    )
    export_grid_uv: bpy.props.BoolProperty(
        options=set(),
        name="Grid UV",
        description=(
            "Give the result a UV layer where every quad is a unit square: stripes and checkers follow the "
            "edges. Parts of it overlap, so it is for patterns that repeat, not for painting"
        ),
        default=True,
    )
    relief_resolution: bpy.props.EnumProperty(
        options=set(),
        name="Resolution",
        description="Width and height of the height map, in texels",
        items=[(str(size), f"{size} x {size}", "") for size in (512, 1024, 2048, 4096)],
        default="2048",
    )
    relief_levels: bpy.props.IntProperty(
        options=set(),
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
        options=set(),
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
        options=set(),
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
        options=set(),
        name="Folder",
        description="Where the file is written; // is the folder of the .blend file",
        subtype="DIR_PATH",
        default="//textures/",
    )
    export_atlas_uv: bpy.props.BoolProperty(
        options=set(),
        name="Atlas UV",
        description=(
            "Give the result a UV layer made of islands that never overlap, packed into the unit square, "
            "to paint or bake on, and mark its seams on the edges"
        ),
        default=True,
    )
    export_sharp_edges: bpy.props.BoolProperty(
        options=set(),
        name="Sharp Edges",
        description=(
            "Mark as sharp the edges of the result that follow a hard edge of the original "
            "(only presets that keep hard edges, Hard surface, find any) or a curve of the Sharp category"
        ),
        default=True,
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
    export_creases: bpy.props.BoolProperty(
        options=set(),
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
    materials=False, copy=False,
):
    """A new mesh datablock: `source`, an evaluated mesh, remeshed into quads
    with the engine's `settings` on top of `preset`, and `density` if given
    (one weight per vertex of `source`). `on_stage`, if given, is told the
    name of each stage as it begins. The three steps, for a run that does
    not wait: `engine_input`, `engine_run`, `mesh_of`.

    `uv_layers` names the layers of texture coordinates to put on the mesh
    ("grid", "atlas"); with the atlas come its seams, marked on the edges.
    `sharp_edges` marks as sharp the edges that follow a hard edge of
    `source`. `creases` gives a crease weight to the edges that should stay
    sharp when the mesh is subdivided. `curves` are guide curves, as
    `curves_of` gives them, in the space of `source`. `materials` gives the
    mesh the materials of `source`, each face that of the surface it lies
    on, with the borders between them kept as lines of edges."""
    given = engine_input(source, preset, settings, density, uv_layers, sharp_edges, creases, curves, materials, copy)
    return mesh_of(engine_run(given, on_stage), given)


class EngineInput:
    """What a run of the engine is given, read from Blender beforehand: the
    run itself touches nothing of Blender, and can go on in a thread of its
    own when the arrays are copies."""


def engine_input(
    source, preset, settings, density=None, uv_layers=(), sharp_edges=False, creases=False, curves=(),
    materials=False, copy=False,
):
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
    groups = None
    if materials and len(source.materials) > 1:
        # The material of each triangle; an object all of one is left alone.
        groups = numpy.empty(triangles, numpy.int32)
        source.loop_triangles.foreach_get("material_index", groups)
        groups = groups.astype(numpy.uint32) if groups.min() != groups.max() else None
    given = EngineInput()
    given.counts = (vertices, triangles, corners)
    given.addresses, given.kept, given.groups = addresses, kept, groups
    given.preset, given.settings, given.density, given.curves = preset, settings, density, curves
    given.uv_layers, given.sharp_edges, given.creases = uv_layers, sharp_edges, creases
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
        given.settings, given.density, on_stage, uv_layers=given.uv_layers, seams="atlas" in given.uv_layers,
        hard_edges=given.sharp_edges, creases=given.creases, curves=given.curves, groups=given.groups,
    )


def mesh_of(made, given):
    """The mesh datablock of what a run made, with what `given` asked for,
    and the report of the run."""
    import numpy

    uv_layers, sharp_edges, creases = given.uv_layers, given.sharp_edges, given.creases
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
        for material in given.materials:
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


# The custom property of a result where what the engine said of it is kept.
REPORT = "openclare_report"


class Remeshing:
    """What the two buttons that remesh have in common: the run, and what
    is said of it while it goes.

    Clicked in Blender, the engine runs in a thread of its own and Blender
    stays free: the panel shows a bar, Esc stops the run. From a script or
    in the background, the operator waits for the engine, as it always did."""

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
            engine_settings = {"quads": settings.quads, **shape_of(settings)}
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
            self.sharp = curve_objects(settings.sharp_curves)
            self.curves = curves_of(settings.sharp_curves, original, sharp=True)
            self.curves += curves_of(settings.guide_curves, original, but=self.sharp)
            given = engine_input(
                source, BASE_PRESET, engine_settings, density, layers, settings.export_sharp_edges,
                settings.export_creases, self.curves, settings.export_materials, copy,
            )
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
        return self.finish(context, original, made, given)

    def finish(self, context, original, made, given):
        """Makes the object of what the engine `made` from `original`, next
        to it and the active one, and says how it went. Returns it."""
        mesh, report = mesh_of(made, given)
        curves, sharp = self.curves, self.sharp
        mesh.name = f"{original.data.name}.quads"
        remeshed = bpy.data.objects.new(f"{original.name}.quads", mesh)
        remeshed.matrix_world = original.matrix_world
        # Kept, for what is baked onto the result later to be read from it.
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
        return {"FINISHED"} if made is not None else {"CANCELLED"}


class OPENCLARE_OT_remesh_again(Background, Remeshing, bpy.types.Operator):
    """Remesh the original of this result again, with the settings it has now, in place of this result. Its
    relief goes: bake it again"""

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
        # The relief was baked on the atlas of the mesh that went.
        for name in (RELIEF, SUBDIVISION):
            if name in result.modifiers:
                result.modifiers.remove(result.modifiers[name])
        if "openclare_height_file" in result:
            del result["openclare_height_file"]
        for other in context.selected_objects:
            other.select_set(False)
        result.select_set(True)
        context.view_layer.objects.active = result
        return {"FINISHED"}


def category_collection(context, name, create=False):
    """The collection of the category `name` of the object the panel is
    about; made and put in the scene if there is none yet and `create` says
    so."""
    settings = object_to_remesh(context).openclare
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
        return source_of(object_to_remesh(context)) is not None and context.mode == "OBJECT"

    def execute(self, context):
        import numpy

        target = object_to_remesh(context)
        settings = target.openclare
        resolution = int(settings.relief_resolution)
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
        return object_to_remesh(context) is not None and context.mode == "OBJECT"


class OPENCLARE_OT_curves_assign(CurvesOperator, bpy.types.Operator):
    """Move the selected curves to this category of the object and give them its colour. A curve is in one category
    only"""

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
            # The object stays the active one: its panel stays shown.
            if drawn.name in context.view_layer.objects and (drawn.visible_get() or not self.select):
                drawn.select_set(self.select)
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
        settings = owner.openclare
        layout.prop(settings, "target", expand=True)
        if settings.target == "SIZE":
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
                    column.label(text=f"About {quads:,.0f} quads".replace(",", " "), icon="MESH_GRID")
                    if below >= 0.005:
                        column.label(text=f"{100.0 * below:.0f} % coarser than its shape asks", icon="ERROR")
                layout.prop(settings, "show_sizes")
        else:
            layout.prop(settings, "quads")
        column = layout.column(align=True)
        column.prop_search(settings, "density_group", owner, "vertex_groups", icon="GROUP_VERTEX")
        factor = column.row()
        factor.enabled = bool(settings.density_group)
        factor.prop(settings, "density_factor")
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
    wanted, below = float(told.get("quads_wanted", 0.0)), float(told.get("below_smallest", 0.0))
    if wanted > 0.0:
        lines.append((f"The shape called for {wanted:,.0f}".replace(",", " "), "INFO"))
        if below >= 0.005:
            lines.append((f"{100.0 * below:.0f} % coarser than its shape asks", "ERROR"))
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
        settings = object_to_remesh(context).openclare
        layout.label(text="Select curves, then this object last", icon="INFO")
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


class OPENCLARE_PT_export(DataPanel, bpy.types.Panel):
    """What the result carries besides its quads"""

    bl_label = "Export"
    bl_idname = "OPENCLARE_PT_export"
    bl_parent_id = "OPENCLARE_PT_panel"
    bl_options = {"DEFAULT_CLOSED"}

    @classmethod
    def poll(cls, context):
        return is_source(context)

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False
        settings = object_to_remesh(context).openclare
        column = layout.column(heading="UV Layers")
        column.prop(settings, "export_grid_uv", text="Grid")
        column.prop(settings, "export_atlas_uv", text="Atlas")
        column = layout.column(heading="Surface")
        column.prop(settings, "export_materials", text="Materials")
        column = layout.column(heading="Edges")
        column.prop(settings, "export_sharp_edges", text="Sharp")
        column.prop(settings, "export_creases", text="Creases")


class OPENCLARE_PT_relief(DataPanel, bpy.types.Panel):
    """The relief of the original, baked onto the result"""

    bl_label = "Relief"
    bl_idname = "OPENCLARE_PT_relief"
    bl_parent_id = "OPENCLARE_PT_panel"

    @classmethod
    def poll(cls, context):
        return source_of(object_to_remesh(context)) is not None

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False
        target = object_to_remesh(context)
        settings = target.openclare
        layout.prop(settings, "relief_levels")
        layout.prop(settings, "relief_resolution")
        layout.prop(settings, "relief_apply")
        layout.prop(settings, "relief_format")
        folder = layout.row()
        folder.enabled = settings.relief_format != "NONE"
        folder.prop(settings, "relief_directory")
        layout.operator(OPENCLARE_OT_bake_relief.bl_idname, icon="RNDCURVE")
        written = target.get("openclare_height_file")
        if written:
            # How the file last written is to be read, where it is used.
            layout.label(text=written, icon="FILE_IMAGE")


CLASSES = (
    OpenclareSettings,
    OPENCLARE_OT_analyse,
    OPENCLARE_OT_remesh,
    OPENCLARE_OT_remesh_again,
    OPENCLARE_OT_stop,
    OPENCLARE_OT_select_original,
    OPENCLARE_OT_bake_relief,
    OPENCLARE_OT_curves_assign,
    OPENCLARE_OT_curves_remove,
    OPENCLARE_OT_curves_select,
    OPENCLARE_OT_shape_preset_add,
    OPENCLARE_PT_shape_presets,
    OPENCLARE_PT_panel,
    OPENCLARE_PT_shape,
    OPENCLARE_PT_curves,
    OPENCLARE_PT_export,
    OPENCLARE_PT_relief,
)


# The add-on's own presets are in presets/ here.
ADDON_FOLDER = os.path.dirname(os.path.abspath(__file__))
HANDLERS = (
    (bpy.app.handlers.save_pre, before_saving),
    (bpy.app.handlers.save_post, after_saving),
    (bpy.app.handlers.load_post, after_loading),
)


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Object.openclare = bpy.props.PointerProperty(type=OpenclareSettings)
    # Where files of 0.1 kept the settings: read once, then given to objects.
    bpy.types.Scene.openclare = bpy.props.PointerProperty(type=OpenclareSettings)
    bpy.types.Object.openclare_source = bpy.props.PointerProperty(
        name="Original",
        description="The object this one was remeshed from: what is baked onto it is read from there",
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
    stop_preview(bpy.context)
    ANALYSES.clear()
    del bpy.types.Object.openclare_source
    del bpy.types.Object.openclare
    del bpy.types.Scene.openclare
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
