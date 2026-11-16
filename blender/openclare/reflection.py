"""The reflection check (#328): on the cage being edited, under a Catmull-Clark
Subdivision Surface, the faults that will make a reflection pinch or wobble
once it is subdivided, worst first, each ranked by one number.

The criteria are the engine's (`openclare::reflection`, called through
`engine.reflection_check`): none is worked out here. This module takes the
cage out of Edit Mode, keeps the faults found until Blender says something
changed, lists the N worst in a panel and draws them in the 3D viewport,
with one colour per kind of fault:

- folded corner quads (red) and thin ones (orange): a dashed star from the
  centre of the face to the corners whose quad folds or is thin;
- a face off its plane (yellow): a circle flat round its centre, and each
  vertex off the plane joined to where it falls on the plane;
- a pinch (violet): the edges that pinch, dashed, and the corners that do;
- a pole on a curved zone (blue): a flat ring round the vertex;
- a face of no area (magenta): a point at its centre.

The angles are measured on the cage, not on the subdivided surface: they
rank faults against one another, they are not the actual wobble of the
reflection.

The faults are found again when Blender signals a change (an edit, a
selection, an undo) or the settings change, as long as a check of the cage
takes less than `AUTO_SECONDS`; a cage that takes longer is checked on
demand, with the Check button. Out of X-ray, each point drawn is tested for
being seen (see `Visibility`); a segment is drawn if both its ends are.
"""

import time
from math import cos, pi, radians, sin

import bmesh
import bpy
from bpy.app.handlers import persistent

from . import engine, experimental

# One colour per kind of fault, the same in the viewport and in the panel's
# words.
COLOURS = {
    "degenerate": (1.00, 0.20, 0.80, 1.0),
    "folded": (1.00, 0.25, 0.25, 1.0),
    "not_flat": (1.00, 0.90, 0.25, 1.0),
    "pinch": (0.75, 0.25, 0.65, 1.0),
    "curved_pole": (0.30, 0.70, 1.00, 1.0),
    "thin": (1.00, 0.65, 0.15, 1.0),
}
CENTRE = (1.0, 1.0, 1.0, 1.0)
# What each kind of fault is mended by, and the icon that says it.
REPAIRS = {
    "degenerate": "topology",
    "folded": "move",
    "not_flat": "move",
    "pinch": "topology",
    "curved_pole": "topology",
    "thin": "move",
}
REPAIR_ICONS = {"move": "EMPTY_ARROWS", "topology": "MESH_GRID"}
REPAIR_WORDS = {"move": "move vertices", "topology": "change the topology"}

# A check of the cage refreshed on every change only if it took less than
# this, in seconds; beyond it, on demand.
AUTO_SECONDS = 0.05
DASH_PX = 8          # length of a dash on screen
CIRCLE_SEGMENTS = 48
MAX_RAYS = 1000      # points tested for being seen, per view
# The mesh the cage is copied to, to be read in a few calls rather than
# vertex by vertex; used by no object.
SCRATCH = ".openclare reflection check"

_generation = 0      # raised on every change Blender signals
_reports = {}        # pointer of the mesh -> Report
_handles = []
_timer_set = False


class Report:
    """The faults of one cage, as the engine found them (`faults`, an
    `engine.Faults`), with what drawing and listing them asks for: the
    positions of the cage (a numpy array of three floats per vertex, in the
    space of the object), the vertices of each face (`starts`, where the
    corners of each face start in `corners`), the key it was found under, and
    how long finding it took, in seconds."""

    def __init__(self, faults, positions, sizes, corners, key, seconds):
        import numpy

        self.faults, self.positions, self.corners, self.key, self.seconds = faults, positions, corners, key, seconds
        self.starts = numpy.concatenate(([0], numpy.cumsum(sizes, dtype=numpy.int64)))

    def face(self, index):
        """The vertices of face `index`, in order."""
        return self.corners[self.starts[index]:self.starts[index + 1]]

    def __len__(self):
        return len(self.faults.kinds)


def settings_of(scene):
    return scene.openclare_reflection


def subsurf_modifiers(owner):
    return [m for m in owner.modifiers if m.type == "SUBSURF" and m.subdivision_type == "CATMULL_CLARK"]


def uses_creases(owner):
    """Whether the creases of the mesh count: they do unless a Catmull-Clark
    modifier of the object has Use Creases off."""
    return all(m.use_creases for m in subsurf_modifiers(owner))


def checked_object(context):
    """The object whose cage is checked: the one in Edit Mode, if it is a mesh
    under a Catmull-Clark Subdivision Surface."""
    owner = context.edit_object
    if owner is None or owner.type != "MESH" or not subsurf_modifiers(owner):
        return None
    return owner


def key_of(owner, settings):
    return (_generation, round(settings.flat_tolerance, 6), round(settings.angle_tolerance, 9), uses_creases(owner))


def scratch_mesh():
    mesh = bpy.data.meshes.get(SCRATCH)
    if mesh is None or mesh.users:
        mesh = bpy.data.meshes.new(SCRATCH)
    return mesh


def cage_arrays(owner, creases=True):
    """The cage of `owner`, in Edit Mode, as the engine takes it: positions,
    the number of corners of each face, the vertex of each corner, and the
    creased edges with their weights (from the attribute crease_edge), or
    None and None when `creases` is false or there are none. Vertices and
    faces are numbered as the edit mesh numbers them."""
    import numpy

    mesh = scratch_mesh()
    bmesh.from_edit_mesh(owner.data).to_mesh(mesh)
    positions = numpy.empty(3 * len(mesh.vertices), numpy.float32)
    mesh.vertices.foreach_get("co", positions)
    sizes = numpy.empty(len(mesh.polygons), numpy.uint32)
    mesh.polygons.foreach_get("loop_total", sizes)
    corners = numpy.empty(len(mesh.loops), numpy.uint32)
    mesh.loops.foreach_get("vertex_index", corners)
    pairs = weights = None
    crease = mesh.attributes.get("crease_edge")
    if creases and crease is not None and crease.domain == "EDGE" and len(mesh.edges):
        weights = numpy.empty(len(mesh.edges), numpy.float32)
        crease.data.foreach_get("value", weights)
        ends = numpy.empty(2 * len(mesh.edges), numpy.uint32)
        mesh.edges.foreach_get("vertices", ends)
        keep = weights > 0.0
        pairs, weights = ends.reshape(-1, 2)[keep].ravel(), weights[keep]
    return positions, sizes, corners, pairs, weights


def check(owner, settings):
    """Checks the cage of `owner` afresh and keeps what was found. Returns
    the `Report`."""
    start = time.perf_counter()
    positions, sizes, corners, pairs, weights = cage_arrays(owner, uses_creases(owner))
    faults = engine.reflection_check(
        positions, sizes, corners, pairs, weights,
        flat_tolerance=settings.flat_tolerance / 100.0, angle_tolerance=settings.angle_tolerance * 180.0 / pi,
    )
    report = Report(faults, positions.reshape(-1, 3), sizes, corners, key_of(owner, settings),
                    time.perf_counter() - start)
    _reports[owner.data.as_pointer()] = report
    return report


def report_of(owner, settings):
    """What is known of the cage of `owner`: its `Report` and whether it is
    up to date, or (None, False)."""
    report = _reports.get(owner.data.as_pointer())
    if report is None:
        return None, False
    return report, report.key == key_of(owner, settings)


def automatic(report):
    """Whether a cage is checked again on every change: one that was quick
    to check."""
    return report is None or report.seconds < AUTO_SECONDS


def fault_name(report, i):
    """What the panel says of fault `i`: "N-gon, 6 sides · pinch 12°"."""
    faults = report.faults
    kind = engine.FAULT_KINDS[faults.kinds[i]]
    sides, value, count = int(faults.sides[i]), float(faults.values[i]), int(faults.counts[i])
    if faults.on_vertex[i]:
        what = f"Pole, {sides} edges"
    else:
        what = {3: "Triangle", 4: "Quad"}.get(sides, f"N-gon, {sides} sides")

    def degrees(angle):
        return f"{angle:.1f}°" if angle < 10.0 else f"{angle:.0f}°"

    def quads(n):
        return f"{n} corner quad{'s' if n > 1 else ''}"

    detail = {
        "degenerate": "no area",
        "folded": f"{quads(count)} fold",
        "not_flat": f"not flat {degrees(value)}",
        "pinch": f"pinch {degrees(value)}",
        "curved_pole": f"curved {degrees(value)}",
        "thin": f"{quads(count)} thin",
    }[kind]
    return f"{what} · {detail}"


def rows(report, worst):
    """The faults the panel lists: for each of the `worst` first, its index,
    its words and the icon of its repair."""
    out = []
    for i in range(min(worst, len(report))):
        kind = engine.FAULT_KINDS[report.faults.kinds[i]]
        out.append((i, fault_name(report, i), REPAIR_ICONS[REPAIRS[kind]]))
    return out


# ---------------------------------------------------------------- refresh


def refresh(context=None):
    """Checks the cage in Edit Mode again if it is shown, out of date and
    quick to check. Returns the report, or None."""
    context = context or bpy.context
    scene = getattr(context, "scene", None)
    if scene is None or not settings_of(scene).show or not experimental.shown():
        return None
    owner = checked_object(context)
    if owner is None:
        return None
    settings = settings_of(scene)
    report, current = report_of(owner, settings)
    if current:
        return report
    if not automatic(report):
        return report
    return check(owner, settings)


def _refresh_soon():
    global _timer_set
    _timer_set = False
    try:
        if refresh() is not None:
            tag_redraw()
    except (ReferenceError, RuntimeError, ValueError, engine.EngineError) as error:
        print(f"openclare: reflection check: {error}")
    return None


def _schedule():
    global _timer_set
    if not _timer_set and not bpy.app.background:
        _timer_set = True
        bpy.app.timers.register(_refresh_soon, first_interval=0.0)


@persistent
def _changed(_scene=None, depsgraph=None):
    """Blender signals a change. Copying the cage to the scratch mesh signals
    one with nothing in it: that one is not a change of the cage."""
    global _generation
    if depsgraph is not None and not len(depsgraph.updates):
        return
    _generation += 1
    _schedule()


@persistent
def _undone(*_args):
    global _generation
    _generation += 1
    _schedule()


@persistent
def _loaded(*_args):
    _reports.clear()
    _undone()


def settings_changed(_settings, context):
    # The tolerances are in the key of what was found: changed, it is out of
    # date and found again.
    _schedule()
    tag_redraw(context)


def tag_redraw(context=None):
    wm = (context or bpy.context).window_manager
    for window in wm.windows:
        for area in window.screen.areas if window.screen else ():
            if area.type in {"VIEW_3D", "PROPERTIES"}:
                area.tag_redraw()


# ---------------------------------------------------------------- settings


class OpenclareReflectionSettings(bpy.types.PropertyGroup):
    """The reflection check of the scene: whether it is shown, how many
    faults, and what counts as one."""

    show: bpy.props.BoolProperty(
        name="Reflection Check",
        description="List and draw what of the cage in Edit Mode will make a reflection pinch or wobble once "
                    "Catmull-Clark subdivides it. Angles are measured on the cage, not on the subdivided surface",
        default=False,
        update=settings_changed,
    )
    worst: bpy.props.IntProperty(
        name="Show the N Worst",
        description="How many faults are listed and drawn, the worst first",
        default=20,
        min=1,
        soft_max=200,
        update=settings_changed,
    )
    flat_tolerance: bpy.props.FloatProperty(
        name="Off the Plane",
        description="How far a vertex of an n-gon may be from its plane, as a share of the radius of the face, "
                    "for the face to count as flat",
        default=0.2,
        min=0.0,
        soft_max=5.0,
        precision=2,
        subtype="PERCENTAGE",
        update=settings_changed,
    )
    angle_tolerance: bpy.props.FloatProperty(
        name="Fold",
        description="Largest fold between a triangle or an n-gon and what touches it, and between the faces round "
                    "a pole, that is no fault",
        default=radians(1.0),
        min=0.0,
        soft_max=radians(20.0),
        subtype="ANGLE",
        update=settings_changed,
    )


# ---------------------------------------------------------------- operators


def frame_selected(context):
    """Frames the selection in the first 3D view of the window; False where
    there is none (a Blender without interface)."""
    screen = context.window.screen if context.window else None
    for area in screen.areas if screen else ():
        if area.type != "VIEW_3D":
            continue
        region = next((r for r in area.regions if r.type == "WINDOW"), None)
        if region is None:
            continue
        with context.temp_override(area=area, region=region):
            bpy.ops.view3d.view_selected()
        return True
    return False


class OPENCLARE_OT_reflection_check(bpy.types.Operator):
    """Check the cage again: what of it will make a reflection pinch or wobble once subdivided"""

    bl_idname = "mesh.openclare_reflection_check"
    bl_label = "Check"
    bl_options = {"REGISTER"}

    @classmethod
    def poll(cls, context):
        return checked_object(context) is not None

    def execute(self, context):
        report = check(checked_object(context), settings_of(context.scene))
        tag_redraw(context)
        self.report({"INFO"}, f"{len(report)} faults, {1000.0 * report.seconds:.0f} ms")
        return {"FINISHED"}


class OPENCLARE_OT_reflection_select(bpy.types.Operator):
    """Select this face or vertex, and frame the view on it"""

    bl_idname = "mesh.openclare_reflection_select"
    bl_label = "Select Fault"
    bl_options = {"REGISTER", "UNDO"}

    index: bpy.props.IntProperty(name="Fault", description="Its place in the list, the worst first", min=0)

    @classmethod
    def poll(cls, context):
        return checked_object(context) is not None

    def execute(self, context):
        owner = checked_object(context)
        settings = settings_of(context.scene)
        report, current = report_of(owner, settings)
        if report is None or not current:
            # The numbering of the cage may have changed: found afresh.
            report = check(owner, settings)
        if self.index >= len(report):
            self.report({"WARNING"}, "No such fault: the cage was checked again")
            return {"CANCELLED"}
        on_vertex = bool(report.faults.on_vertex[self.index])
        index = int(report.faults.indices[self.index])
        context.tool_settings.mesh_select_mode = (True, False, False) if on_vertex else (False, False, True)
        bpy.ops.mesh.select_all(action="DESELECT")
        bm = bmesh.from_edit_mesh(owner.data)
        elements = bm.verts if on_vertex else bm.faces
        elements.ensure_lookup_table()
        element = elements[index]
        element.select_set(True)
        bm.select_history.clear()
        bm.select_history.add(element)
        if not on_vertex:
            bm.faces.active = element
        bmesh.update_edit_mesh(owner.data, loop_triangles=False, destructive=False)
        frame_selected(context)
        return {"FINISHED"}


# ---------------------------------------------------------------- panel


class OPENCLARE_PT_reflection(bpy.types.Panel):
    """What of the cage will make a reflection pinch or wobble under
    Catmull-Clark, the worst first"""

    bl_label = "Reflection Check"
    bl_idname = "OPENCLARE_PT_reflection"
    bl_space_type = "PROPERTIES"
    bl_region_type = "WINDOW"
    bl_context = "data"
    bl_options = {"DEFAULT_CLOSED"}

    @classmethod
    def poll(cls, context):
        owner = getattr(context, "object", None)
        return (
            owner is not None and owner.type == "MESH" and bool(subsurf_modifiers(owner)) and experimental.shown()
        )

    def draw_header(self, context):
        self.layout.prop(settings_of(context.scene), "show", text="")

    def draw(self, context):
        draw_panel(self.layout, context)


def draw_panel(layout, context):
    """The panel, on `layout`: the settings, then the faults found."""
    settings = settings_of(context.scene)
    layout.label(text="Angles are measured on the cage, not on the subdivided surface", icon=experimental.ICON)
    column = layout.column(align=True)
    column.prop(settings, "worst")
    column.prop(settings, "flat_tolerance")
    column.prop(settings, "angle_tolerance")
    if not settings.show:
        return
    owner = checked_object(context)
    if owner is None:
        layout.label(text="In Edit Mode, the faults of the cage are listed here", icon="INFO")
        return
    # Found again after each change by a timer, not here: a panel is drawn,
    # it does not change data.
    report, current = report_of(owner, settings)
    if report is None or not (current or automatic(report)):
        row = layout.row()
        row.operator(OPENCLARE_OT_reflection_check.bl_idname, icon="FILE_REFRESH")
        if report is not None:
            row.label(text=f"Out of date: a check takes {1000.0 * report.seconds:.0f} ms", icon="TIME")
    if report is None:
        return
    if not uses_creases(owner):
        layout.label(text="Use Creases is off: only borders are sharp", icon="INFO")
    listed = rows(report, settings.worst)
    layout.label(text=f"{len(listed)} shown out of {len(report)}; {report.faults.poles} poles looked at")
    box = layout.column(align=True)
    for index, words, icon in listed:
        box.operator(OPENCLARE_OT_reflection_select.bl_idname, text=words, icon=icon, emboss=False).index = index
    if listed:
        legend = layout.row(align=True)
        for repair in ("move", "topology"):
            legend.label(text=REPAIR_WORDS[repair], icon=REPAIR_ICONS[repair])


# ---------------------------------------------------------------- overlay


def dashes(a, b, colour, region, view):
    """The segment from `a` to `b` cut in dashes about DASH_PX pixels long
    on screen (ten of them where it cannot be projected)."""
    from bpy_extras.view3d_utils import location_3d_to_region_2d

    pa = location_3d_to_region_2d(region, view, a) if region is not None else None
    pb = location_3d_to_region_2d(region, view, b) if region is not None else None
    px = (pb - pa).length if pa is not None and pb is not None else 20 * DASH_PX
    n = max(1, int(px / (2 * DASH_PX)))
    step = 1.0 / (2 * n)
    return [(a.lerp(b, 2 * i * step), a.lerp(b, (2 * i + 1) * step), colour) for i in range(n)]


def circle(centre, normal, radius, colour):
    from mathutils import Vector

    normal = Vector(normal)
    if normal.length < 1e-12:
        return []
    u = normal.orthogonal().normalized()
    v = normal.cross(u).normalized()
    points = [centre + radius * (cos(t) * u + sin(t) * v)
              for t in (2 * pi * i / CIRCLE_SEGMENTS for i in range(CIRCLE_SEGMENTS))]
    return [(points[i], points[(i + 1) % CIRCLE_SEGMENTS], colour) for i in range(CIRCLE_SEGMENTS)]


class SeeAll:
    """In X-ray, and with no view to test against, everything is seen."""

    def sees(self, owner, co):
        return True


class Visibility:
    """Says whether a point of the cage is seen from the eye, out of X-ray.

    A ray goes from the eye to the point: it is hidden if the ray hits
    something before. In Edit Mode, scene.ray_cast hits the cage of the
    object edited, not its subdivided surface: the cage hides itself
    exactly, and other objects hide it by their surface. One ray per point,
    kept while neither the view nor the mesh changes."""

    def __init__(self, context):
        self.region, self.view = context.region, context.region_data
        self.scene = context.scene
        self.depsgraph = context.evaluated_depsgraph_get()
        self.rays = 0
        self.known = {}

    def sees(self, owner, co):
        """`co` in the space of the object."""
        key = (owner.name, tuple(round(x, 6) for x in co))
        if key not in self.known:
            self.known[key] = self._sees(owner, co)
        return self.known[key]

    def _sees(self, owner, co):
        from bpy_extras.view3d_utils import location_3d_to_region_2d, region_2d_to_origin_3d, region_2d_to_vector_3d

        p = owner.matrix_world @ co
        p2 = location_3d_to_region_2d(self.region, self.view, p)
        if p2 is None:
            return False   # behind the eye
        if not (0 <= p2.x <= self.region.width and 0 <= p2.y <= self.region.height):
            return True    # off screen: the GPU clips it, no ray needed
        if self.rays >= MAX_RAYS:
            return False   # out of rays: what is not tested is not drawn
        self.rays += 1
        origin = region_2d_to_origin_3d(self.region, self.view, p2)
        direction = region_2d_to_vector_3d(self.region, self.view, p2)
        reach = (p - origin).dot(direction)
        eps = max(1e-3 * reach, 1e-5)
        hit, *_ = self.scene.ray_cast(self.depsgraph, origin, direction, distance=max(0.0, reach - eps))
        return not hit


_views = {}   # pointer of the region -> (key of the view, Visibility)


def xray(space):
    shading = space.shading
    return shading.show_xray_wireframe if shading.type == "WIREFRAME" else shading.show_xray


def visibility(context):
    if xray(context.space_data):
        return SeeAll()
    region, view = context.region, context.region_data
    key = (_generation, tuple(map(tuple, view.perspective_matrix)), region.width, region.height)
    pointer = region.as_pointer()
    cached = _views.get(pointer)
    if cached is not None and cached[0] == key:
        return cached[1]
    seen = Visibility(context)
    _views[pointer] = (key, seen)
    return seen


def overlay(owner, report, worst, seen=None, region=None, view=None):
    """What is drawn for the `worst` first faults of `report` on `owner`:
    lines (pairs of points with a colour), points at the vertices at fault,
    and points at the centres of faces, all in world space. `seen` tells
    whether a point of the cage is seen (`SeeAll` by default); `region` and
    `view` size the dashes."""
    from mathutils import Vector

    seen = seen or SeeAll()
    faults = report.faults
    matrix = owner.matrix_world
    lines, vertices, centres = [], [], []
    for i in range(min(worst, len(report))):
        kind = engine.FAULT_KINDS[faults.kinds[i]]
        colour = COLOURS[kind]
        centre_local = Vector(faults.centres[i].tolist())
        normal_local = Vector(faults.normals[i].tolist())
        centre = matrix @ centre_local
        if faults.on_vertex[i]:
            if seen.sees(owner, centre_local):
                ring = circle(centre_local, normal_local, 0.3 * float(faults.radius[i]), colour)
                lines.extend((matrix @ a, matrix @ b, c) for a, b, c in ring)
            continue
        face = report.face(int(faults.indices[i]))
        local = [Vector(report.positions[v].tolist()) for v in face]
        world = [matrix @ p for p in local]
        corners, edges = faults.parts(i)
        if kind == "degenerate":
            if seen.sees(owner, centre_local):
                centres.append((centre, colour))
            continue
        if kind in ("folded", "thin"):
            if seen.sees(owner, centre_local):
                for k in corners:
                    if seen.sees(owner, local[k]):
                        lines.extend(dashes(centre, world[k], colour, region, view))
                centres.append((centre, colour))
        if kind == "not_flat":
            if seen.sees(owner, centre_local):
                inner, radius = float(faults.inner[i]), float(faults.radius[i])
                size = 0.4 * inner if inner > 0.1 * radius else 0.15 * radius
                lines.extend((matrix @ a, matrix @ b, c) for a, b, c in circle(centre_local, normal_local, size, colour))
            for k in corners:
                if seen.sees(owner, local[k]):
                    on_plane = local[k] - (local[k] - centre_local).dot(normal_local) * normal_local
                    lines.append((world[k], matrix @ on_plane, colour))
        if kind == "pinch":
            for k in edges:
                j = (k + 1) % len(face)
                if seen.sees(owner, local[k]) and seen.sees(owner, local[j]):
                    lines.extend(dashes(world[k], world[j], colour, region, view))
        vertices.extend((world[k], colour) for k in corners if seen.sees(owner, local[k]))
    return lines, vertices, centres


def _lines(segments, width):
    import gpu
    from gpu_extras.batch import batch_for_shader

    if not segments:
        return
    region = bpy.context.region
    shader = gpu.shader.from_builtin("POLYLINE_SMOOTH_COLOR")
    positions, colours = [], []
    for a, b, c in segments:
        positions += [a, b]
        colours += [c, c]
    batch = batch_for_shader(shader, "LINES", {"pos": positions, "color": colours})
    shader.bind()
    shader.uniform_float("viewportSize", (region.width, region.height))
    shader.uniform_float("lineWidth", width)
    batch.draw(shader)


def _points(points, size):
    import gpu
    from gpu_extras.batch import batch_for_shader

    if not points:
        return
    shader = gpu.shader.from_builtin("POINT_FLAT_COLOR")
    batch = batch_for_shader(shader, "POINTS", {"pos": [p for p, _ in points], "color": [c for _, c in points]})
    gpu.state.point_size_set(size)
    batch.draw(shader)


def draw_view():
    """Draws the worst faults of the cage in Edit Mode, when the check is
    shown, the overlays are, and what was found is up to date."""
    import gpu

    context = bpy.context
    space = context.space_data
    if space is None or not space.overlay.show_overlays or not experimental.shown():
        return
    settings = settings_of(context.scene)
    owner = checked_object(context) if settings.show else None
    if owner is None:
        return
    report, current = report_of(owner, settings)
    if report is None or not current:
        return
    lines, vertices, centres = overlay(
        owner, report, settings.worst, visibility(context), context.region, context.region_data
    )
    scale = context.preferences.system.ui_scale
    gpu.state.blend_set("ALPHA")
    gpu.state.depth_test_set("NONE")
    _lines(lines, 3.0 * scale)
    _points(vertices, 9 * scale)
    _points(centres, 6 * scale)
    gpu.state.point_size_set(1.0)
    gpu.state.blend_set("NONE")


# ---------------------------------------------------------------- registration

CLASSES = (
    OpenclareReflectionSettings,
    OPENCLARE_OT_reflection_check,
    OPENCLARE_OT_reflection_select,
    OPENCLARE_PT_reflection,
)

HANDLERS = (
    (bpy.app.handlers.depsgraph_update_post, _changed),
    (bpy.app.handlers.undo_post, _undone),
    (bpy.app.handlers.redo_post, _undone),
    (bpy.app.handlers.load_post, _loaded),
)


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Scene.openclare_reflection = bpy.props.PointerProperty(type=OpenclareReflectionSettings)
    for handlers, handler in HANDLERS:
        handlers.append(handler)
    _handles.append(bpy.types.SpaceView3D.draw_handler_add(draw_view, (), "WINDOW", "POST_VIEW"))


def unregister():
    for handle in _handles:
        bpy.types.SpaceView3D.draw_handler_remove(handle, "WINDOW")
    _handles.clear()
    for handlers, handler in HANDLERS:
        if handler in handlers:
            handlers.remove(handler)
    if bpy.app.timers.is_registered(_refresh_soon):
        bpy.app.timers.unregister(_refresh_soon)
    _reports.clear()
    _views.clear()
    del bpy.types.Scene.openclare_reflection
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
