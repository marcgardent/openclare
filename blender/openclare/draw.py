"""The Draw Curve and Cut Volume tools, in the toolbar of the 3D viewport
(T) in Object Mode, one button: a guide or sharp curve drawn with the mouse
straight on a mesh, instead of a curve object held on it by a Shrinkwrap by
hand. The mesh drawn on is the active object, selected; when the active
object is not a selected mesh, the tools show the cursor of a thing that
cannot be done, and a click does nothing. A curve is a child of the mesh it
was drawn on, and only its own curves are taken when it is remeshed, though
a copy of it (Shift+D) shares its collections.

The left button held down draws a stroke on the surface, as `trace` keeps
it: forward, back along the stroke to undo it, closed if let go at its
start (the ribbon closes to show it); pressed near an end of a curve drawn
before, it takes that curve up again from there. A ribbon is drawn over the
surface while the stroke goes on; on release the stroke becomes a curve of the category chosen in the
header of the tool (`drawn_curve`), one step of undo. Esc or the right
button drops the stroke under way.

The point that draws is pulled by the mouse on a rope (`trace.pulled`), as
the stabilizer of the annotation: the shake of the hand is left out and the
turns are rounded; the rope is drawn. With a snap chosen in the header, the
point keeps to the ridge or the valley that a few rays
across the stroke find near it (`trace.crease`), as far as a radius drawn
as a circle; F sets the radius with the mouse, as it does that of a brush.

Cut Volume draws a straight line across a limb, as a knife would, from one
side of it to the other: the plane of the view through the line cuts the
object, and the loop of that cut under the line is the curve, all round the
limb, the hidden side too. The loop is shown while the line is drawn, so
that what is let go is what was seen; a cut that does not go round (through
a hole, off a border) or that finds nothing under the line makes no curve,
and says why.

The surface is the object as it is seen, modifiers applied, found by a ray
from the mouse into a BVH tree of it. The tree is built at the first stroke
on a shape and kept while the shape stays the same, so the strokes after
the first start at once; Cut Volume reads the triangles too, at its first
cut. The cut of the line is made away from the mouse, by a thread of its
own: on a big mesh the line follows the mouse at full speed and the loop
follows it as fast as it is cut.

With OPENCLARE_TIMINGS=1 in the environment, each stroke prints to the
console how long it took to start, how long the tool took for each event
of the mouse, how far apart the frames of the viewport were, and for Cut
Volume how long each cut took.
"""

import os
import threading
import time

import bpy
import mathutils
from bpy_extras import view3d_utils
from mathutils.bvhtree import BVHTree

from . import cut, preview, trace

# The spacing of the points of a stroke, in pixels of the viewport where it
# starts, so that the ribbon follows the mouse however near the eye is; the
# brush (how near the stroke a point must be to touch it, and the mouse to
# its start to close it) is so many pixels wide.
STEP_PIXELS = 5.0
BRUSH_PIXELS = 15.0
# How far from the stroke the mouse may be and still take it back.
BACK_PIXELS = 30.0
# In how many rays a side the snap looks across the stroke, as far as its
# radius, and how sharp a crease must be for the stroke to keep to it.
SNAP_RAYS = 5
SNAP_HALVINGS = 8
SNAP_ANGLE = 0.14  # 8 degrees
# How far a loop of Cut Volume may pass from the surface under the line, as
# a share of the diagonal of the box of the object.
STEP_SHARE = 0.01
# The width of the ribbon, in pixels, and of the dots at its ends.
RIBBON_WIDTH = 8.0
END_SIZE = 10.0
# How far the ribbon is brought towards the eye, not to sink into the
# surface: as the preview of the hard edges does.
OFFSET = 4.0

# What was read of the surface last: the key it was read for, its tree, and
# for Cut Volume its `CutSurface`.
SURFACE = {"key": None, "tree": None, "cut": None}
# How many positions the key of the shape samples.
KEY_SAMPLES = 64


def shape_key(owner, mesh):
    """What tells the shape of `owner` changed, its evaluated mesh being
    `mesh`, at no cost on a big mesh: the changes of shape counted by the
    preview, the sizes of the mesh, and a few of its positions."""
    count = len(mesh.vertices)
    sample = tuple(tuple(mesh.vertices[index].co) for index in range(0, count, max(1, count // KEY_SAMPLES)))
    return owner.name, count, len(mesh.polygons), preview.generation_of(owner.name), sample


def surface_tree(context, owner):
    """The BVH tree of `owner` as it is seen, in its own space; built again
    only when its shape changed."""
    depsgraph = context.evaluated_depsgraph_get()
    key = shape_key(owner, owner.evaluated_get(depsgraph).data)
    if SURFACE["key"] != key or SURFACE["tree"] is None:
        SURFACE.update(key=key, tree=BVHTree.FromObject(owner, depsgraph), cut=None)
    return SURFACE["tree"]


class CutSurface:
    """What Cut Volume cuts: positions (64-bit floats) and triangles, and the
    spheres round the triangles (`cut.bounds`), made the first time they are
    needed, in whichever thread needs them first: on a big mesh, the thread
    of the cut, while the mouse moves on."""

    def __init__(self, positions, triangles):
        self.positions, self.triangles = positions, triangles
        self.bounds, self.lock = None, threading.Lock()

    def spheres(self):
        with self.lock:
            if self.bounds is None:
                self.bounds = cut.bounds(self.positions, self.triangles)
        return self.bounds


def cut_surface(context, owner):
    """The `CutSurface` of `owner` as it is seen: read once per shape, with
    the tree."""
    import numpy

    surface_tree(context, owner)
    if SURFACE["cut"] is None:
        mesh = owner.evaluated_get(context.evaluated_depsgraph_get()).data
        positions = numpy.empty(3 * len(mesh.vertices), numpy.float32)
        mesh.vertices.foreach_get("co", positions)
        mesh.calc_loop_triangles()
        triangles = numpy.empty(3 * len(mesh.loop_triangles), numpy.int32)
        mesh.loop_triangles.foreach_get("vertices", triangles)
        SURFACE["cut"] = CutSurface(positions.reshape(-1, 3).astype(numpy.float64), triangles.reshape(-1, 3))
    return SURFACE["cut"]


def cut_loops(surface, point, normal):
    """The loops the plane cuts on `surface` (`cut_surface`), from the
    triangles it may cross only."""
    near = cut.crossed(surface.spheres(), point, normal)
    return cut.plane_loops(surface.positions, near, point, normal)


TIMINGS = bool(os.environ.get("OPENCLARE_TIMINGS"))


class Timings:
    """How long a stroke took, where: kept always, printed with TIMINGS."""

    def __init__(self, tool):
        self.tool, self.began, self.prepared = tool, time.perf_counter(), 0.0
        self.events, self.frames = [], []

    def ready(self):
        self.prepared = time.perf_counter() - self.began

    def event(self, began):
        self.events.append(time.perf_counter() - began)

    def frame(self):
        self.frames.append(time.perf_counter())

    def report(self, cuts=()):
        if not TIMINGS:
            return
        apart = [b - a for a, b in zip(self.frames, self.frames[1:])]
        parts = [f"started in {1000 * self.prepared:.0f} ms", spread("events", self.events),
                 spread("frames apart", apart)]
        if cuts:
            parts.append(spread("cuts", cuts))
        print(f"openclare timings, {self.tool}: " + "; ".join(parts))


def spread(name, values):
    if not values:
        return f"no {name}"
    return f"{len(values)} {name}, {1000 * sum(values) / len(values):.1f} ms mean, {1000 * max(values):.1f} ms max"


def pixel_at(context, owner, point):
    """How long a pixel of the viewport is at `point`, in the space of
    `owner`, where `point` is."""
    region, view = context.region, context.region_data
    world = owner.matrix_world @ mathutils.Vector(point)
    at = view3d_utils.location_3d_to_region_2d(region, view, world)
    if at is None:
        # Behind the eye: the size of the object says it.
        return step_of(owner) / STEP_PIXELS
    beside = view3d_utils.region_2d_to_location_3d(region, view, (at.x + 1.0, at.y), world)
    return max((owner.matrix_world.inverted() @ beside - mathutils.Vector(point)).length, 1e-9)


def step_of(owner):
    """A hundredth of the size of `owner`, in its space."""
    corners = [mathutils.Vector(corner) for corner in owner.bound_box]
    diagonal = max((a - b).length for a in corners for b in corners)
    return max(diagonal * STEP_SHARE, 1e-6)


def surface_at(context, owner, tree, mouse):
    """The point of the surface of `owner` under `mouse` (region pixels) and
    its normal, in the space of `owner`; (None, None) off the object."""
    region, view = context.region, context.region_data
    origin = view3d_utils.region_2d_to_origin_3d(region, view, mouse)
    direction = view3d_utils.region_2d_to_vector_3d(region, view, mouse)
    into = owner.matrix_world.inverted()
    start = into @ origin
    toward = (into.to_3x3() @ direction).normalized()
    location, normal, _, _ = tree.ray_cast(start, toward)
    return (None, None) if location is None else (tuple(location), tuple(normal))


def hit(context, owner, tree, mouse):
    """The point of the surface of `owner` under `mouse` (region pixels), in
    the space of `owner`, or None off the object."""
    return surface_at(context, owner, tree, mouse)[0]


def snapped(context, owner, tree, mouse, across, kind, radius):
    """The point of the surface under `mouse`, moved onto the ridge (`kind`
    "RIDGE") or into the valley ("VALLEY") that the rays either side of
    it, along `across` (a unit vector of the screen) as far as `radius`
    pixels, cross; the point under `mouse` if they cross none. The rays
    are as many whatever the radius: a wider one looks for wider creases."""
    samples = []
    for k in range(-SNAP_RAYS, SNAP_RAYS + 1):
        offset = k * radius / SNAP_RAYS
        samples.append(surface_at(context, owner, tree, (mouse[0] + offset * across[0], mouse[1] + offset * across[1])))
    index = trace.crease(samples, kind, SNAP_ANGLE)
    if index is None:
        return samples[SNAP_RAYS][0]
    # The crease is between the rays either side of the one found: looked
    # for by halves, so that on a sharp edge the point is on the edge.

    def sample_at(offset):
        return surface_at(context, owner, tree, (mouse[0] + offset * across[0], mouse[1] + offset * across[1]))

    spacing = radius / SNAP_RAYS
    offset = (index - SNAP_RAYS) * spacing
    edge = trace.edge_between(sample_at, offset - spacing, offset + spacing, SNAP_HALVINGS)
    return samples[index][0] if edge is None else edge


def target(context):
    """The mesh the tools draw on: the active object, if it is a mesh and
    selected; None otherwise."""
    owner = context.active_object
    return owner if owner is not None and owner.type == "MESH" and owner.select_get() else None


# The cursor of the tools when there is nothing to draw on, and the cursor
# last given to the tool of each window: (idname of the tool, disabled).
DISABLED = "STOP"
CURSORS = {}
LOOK_FOR_CURSOR = 0.2


def disabled_in(view_layer):
    """Whether the tools have nothing to draw on in `view_layer`."""
    owner = view_layer.objects.active
    return owner is None or owner.type != "MESH" or not owner.select_get(view_layer=view_layer)


def follow_cursor():
    """The timer that gives the tools of each window the cursor of a thing
    that cannot be done while the active object is not a selected mesh, and
    theirs again once it is. Blender gives a tool one cursor, set when it
    is made active: the tool is made active again with the other one."""
    from bl_ui.space_toolsystem_common import ToolSelectPanelHelper, _activate_by_item

    window_manager = getattr(bpy.context, "window_manager", None)
    for window in window_manager.windows if window_manager is not None else ():
        tool = window.workspace.tools.from_space_view3d_mode("OBJECT", create=False)
        if tool is None or tool.idname not in TOOL_IDS or window.view_layer is None:
            CURSORS.pop(window.as_pointer(), None)
            continue
        disabled = disabled_in(window.view_layer)
        if CURSORS.get(window.as_pointer()) == (tool.idname, disabled):
            continue
        try:
            with bpy.context.temp_override(window=window):
                tools = ToolSelectPanelHelper._tool_class_from_space_type("VIEW_3D")
                item, index = tools._tool_get_by_id(bpy.context, tool.idname)
                if item is None:
                    continue
                cursor = DISABLED if disabled else (item.cursor or "DEFAULT")
                _activate_by_item(bpy.context, "VIEW_3D", item._replace(cursor=cursor), index)
        except Exception:  # noqa: BLE001 - only the cursor is lost, if Blender changes its tool system
            pass
        CURSORS[window.as_pointer()] = (tool.idname, disabled)
    return LOOK_FOR_CURSOR


def drawn_curves(context, owner, collection):
    """The curves of `collection` drawn on `owner` that can be taken up
    again: children of `owner` of one spline each. A list of (object, points
    as the curve is seen on the surface, in the space of `owner`, closed)."""
    found = []
    if collection is None:
        return found
    depsgraph = context.evaluated_depsgraph_get()
    for drawn in collection.all_objects:
        if drawn.type != "CURVE" or drawn.parent != owner or len(drawn.data.splines) != 1:
            continue
        evaluated = drawn.evaluated_get(depsgraph)
        mesh = evaluated.to_mesh()
        if mesh is None:
            continue
        into = owner.matrix_world.inverted() @ evaluated.matrix_world
        points = [tuple(into @ vertex.co) for vertex in mesh.vertices]
        evaluated.to_mesh_clear()
        closed = drawn.data.splines[0].use_cyclic_u
        if closed and len(points) > 1 and trace.distance(points[0], points[-1]) < 1e-9:
            points.pop()
        found.append((drawn, points, closed))
    return found


def nothing_to_draw_on(mouse):
    """A click of the tools with no selected mesh to draw on: it selects what
    is under the mouse, as a click does, so that a mesh can be chosen, and
    is the tool's: a drag moves nothing."""
    bpy.ops.view3d.select(location=(int(mouse[0]), int(mouse[1])))
    return {"CANCELLED"}


class OPENCLARE_OT_draw_curve(bpy.types.Operator):
    """Draw a curve on the object: forward to draw, back along it to undo, let go at its start to close
    it; from an end of a curve to take it up again"""

    bl_idname = "object.openclare_draw_curve"
    bl_label = "Draw Curve"
    bl_options = {"REGISTER", "UNDO"}

    category: bpy.props.EnumProperty(
        name="Category",
        description="What the curve drawn is",
        items=[
            ("guide", "Guide", "A line of edges along it"),
            ("sharp", "Sharp", "A line of hard edges along it, marked sharp and creased"),
        ],
        default="guide",
    )
    rope: bpy.props.FloatProperty(
        name="Rope",
        description=(
            "The point that draws is pulled by the mouse on a rope this long, in pixels: the shake of the hand "
            "is left out and the turns are rounded. 0 draws under the mouse"
        ),
        subtype="PIXEL",
        default=30.0,
        min=0.0,
        soft_max=100.0,
    )
    snap: bpy.props.EnumProperty(
        name="Snap",
        description="Keep the stroke to a crease near it",
        items=[
            ("NONE", "None", "The stroke goes where the mouse goes"),
            ("RIDGE", "Ridge", "The stroke keeps to the ridge near it: a crest, a bump, an edge that stands out (convex)"),
            ("VALLEY", "Valley", "The stroke keeps to the valley near it: a fold, a hollow, an edge that goes in (concave)"),
        ],
        default="NONE",
    )
    width: bpy.props.FloatProperty(
        name="Width",
        description="How wide the ribbon of the stroke is drawn, in pixels",
        subtype="PIXEL",
        default=RIBBON_WIDTH,
        min=1.0,
        soft_max=30.0,
    )
    spacing: bpy.props.FloatProperty(
        name="Spacing",
        description="How far apart the points of the stroke are laid, in pixels where it starts",
        subtype="PIXEL",
        default=STEP_PIXELS,
        min=1.0,
        soft_max=30.0,
    )
    back_band: bpy.props.FloatProperty(
        name="Back",
        description="How far from the stroke the point that draws may be and still take it back, in pixels",
        subtype="PIXEL",
        default=BACK_PIXELS,
        min=1.0,
        soft_max=100.0,
    )
    close: bpy.props.FloatProperty(
        name="Close",
        description=(
            "How near its start the stroke must be let go to close it, and how near an end of a curve a press "
            "takes the curve up again, in pixels"
        ),
        subtype="PIXEL",
        default=BRUSH_PIXELS,
        min=1.0,
        soft_max=100.0,
    )
    fit: bpy.props.FloatProperty(
        name="Fit",
        description=(
            "How far the curve made may stray from the stroke, in pixels where it starts: more, and the curve "
            "has fewer control points"
        ),
        subtype="PIXEL",
        default=STEP_PIXELS,
        min=0.5,
        soft_max=30.0,
    )
    snap_radius: bpy.props.FloatProperty(
        name="Radius",
        description="How far either side of the stroke the snap looks for a crease, in pixels: drawn as a circle",
        subtype="PIXEL",
        default=15.0,
        min=3.0,
        soft_max=100.0,
    )

    @classmethod
    def poll(cls, context):
        # Always there in the viewport, so that no click of the tool falls
        # through to Blender's own, which would move the object dragged.
        return context.area is not None and context.area.type == "VIEW_3D"

    def invoke(self, context, event):
        from . import CATEGORY, category_collection

        self.timings = Timings("Draw Curve")
        mouse = (event.mouse_region_x, event.mouse_region_y)
        self.owner = target(context)
        if self.owner is None:
            return nothing_to_draw_on(mouse)
        self.tree = surface_tree(context, self.owner)
        start = hit(context, self.owner, self.tree, mouse)
        if start is None:
            # Off the object: the click is the tool's, and does nothing.
            return {"CANCELLED"}
        pixel = pixel_at(context, self.owner, start)
        self.pixel = pixel
        step, near, back = self.spacing * pixel, self.close * pixel, self.back_band * pixel
        collection = category_collection(context, self.category, owner=self.owner)
        self.taken = None
        curves = drawn_curves(context, self.owner, collection)
        end = trace.end_near([(points, closed) for _, points, closed in curves], start, near)
        if end is not None:
            index, at_start = end
            self.taken, points, closed = curves[index]
            self.stroke = trace.taken_up(points, closed, at_start, step, near, back=back)
        else:
            self.stroke = trace.Trace(start, step, near, back=back)
        self.colour = CATEGORY[self.category][1]
        # The point that draws, on the screen, and the mouse that pulls it.
        self.head = self.mouse = mouse
        self.handles = (
            bpy.types.SpaceView3D.draw_handler_add(self.draw, (context,), "WINDOW", "POST_VIEW"),
            bpy.types.SpaceView3D.draw_handler_add(self.draw_pull, (context,), "WINDOW", "POST_PIXEL"),
        )
        context.window_manager.modal_handler_add(self)
        context.region.tag_redraw()
        self.timings.ready()
        return {"RUNNING_MODAL"}

    def modal(self, context, event):
        began = time.perf_counter()
        try:
            return self.step(context, event)
        finally:
            self.timings.event(began)

    def step(self, context, event):
        if event.type == "MOUSEMOVE":
            self.mouse = (event.mouse_region_x, event.mouse_region_y)
            head = trace.pulled(self.head, self.mouse, self.rope)
            pull = (head[0] - self.head[0], head[1] - self.head[1])
            self.head = head
            changed = False
            if pull != (0.0, 0.0):
                if self.snap == "NONE":
                    point = hit(context, self.owner, self.tree, head)
                else:
                    length = (pull[0] ** 2 + pull[1] ** 2) ** 0.5
                    point = snapped(context, self.owner, self.tree, head, (-pull[1] / length, pull[0] / length), self.snap,
                                    self.snap_radius)
                changed = point is not None and self.stroke.move(point) != "still"
            # Only the region of the viewport is drawn again, and only when
            # the ribbon, the rope or the circle of the snap changed.
            if changed or self.rope > 0.0 or (self.snap != "NONE" and pull != (0.0, 0.0)):
                context.region.tag_redraw()
            return {"RUNNING_MODAL"}
        if event.type == "LEFTMOUSE" and event.value == "RELEASE":
            return self.finish(context)
        if event.type in {"ESC", "RIGHTMOUSE"}:
            self.stop(context)
            return {"CANCELLED"}
        return {"RUNNING_MODAL"}

    def finish(self, context):
        from . import bezier_spline, drawn_curve

        self.stop(context)
        made = self.stroke.curve()
        if made is None:
            return {"CANCELLED"}
        points, closed = made
        tolerance = self.fit * self.pixel
        if self.taken is not None:
            bezier_spline(self.taken.data, points, closed, tolerance)
        else:
            drawn_curve(context, self.owner, points, closed, self.category, tolerance)
        return {"FINISHED"}

    def stop(self, context):
        for handle in self.handles:
            bpy.types.SpaceView3D.draw_handler_remove(handle, "WINDOW")
        context.region.tag_redraw()
        self.timings.report()

    def draw(self, context):
        self.timings.frame()
        draw_stroke(context, self.owner.matrix_world, self.stroke.points, self.stroke.closed, self.colour, self.width)

    def draw_pull(self, context):
        """The rope, from the point that draws to the mouse, and the point;
        the circle of the snap round the point."""
        import math

        import gpu
        from gpu_extras.batch import batch_for_shader

        rope, snap = self.rope > 0.0, self.snap != "NONE"
        if not rope and not snap:
            return
        colour = self.colour[:3] + (0.6,)
        line = gpu.shader.from_builtin("POLYLINE_UNIFORM_COLOR")
        gpu.state.blend_set("ALPHA")
        line.bind()
        line.uniform_float("viewportSize", (context.region.width, context.region.height))
        line.uniform_float("lineWidth", 1.5)
        line.uniform_float("color", colour)
        if rope:
            batch_for_shader(line, "LINES", {"pos": [(*self.head, 0.0), (*self.mouse, 0.0)]}).draw(line)
        if snap:
            circle = [
                (self.head[0] + self.snap_radius * math.cos(a), self.head[1] + self.snap_radius * math.sin(a), 0.0)
                for a in (2.0 * math.pi * k / 48 for k in range(48))
            ]
            batch_for_shader(line, "LINE_LOOP", {"pos": circle}).draw(line)
        if not rope:
            gpu.state.blend_set("NONE")
            return
        dot = gpu.shader.from_builtin("POINT_UNIFORM_COLOR")
        point = batch_for_shader(dot, "POINTS", {"pos": [(*self.head, 0.0)]})
        gpu.state.point_size_set(6.0)
        dot.bind()
        dot.uniform_float("color", colour)
        point.draw(dot)
        gpu.state.point_size_set(1.0)
        gpu.state.blend_set("NONE")


def draw_stroke(context, matrix_world, points, closed, colour, width=RIBBON_WIDTH):
    """The ribbon of a stroke over the surface, hidden behind it, and a dot
    at each end of a stroke that is not closed."""
    import gpu
    from gpu_extras.batch import batch_for_shader

    if len(points) < 2:
        return
    view = context.region_data
    projection = gpu.matrix.get_projection_matrix().copy()
    if projection[3][3] > 0.5:
        projection[2][3] -= 0.00001 * OFFSET * view.view_distance
    else:
        projection[2][3] -= 0.0005 * OFFSET
    size = (context.region.width, context.region.height)
    line = gpu.shader.from_builtin("POLYLINE_UNIFORM_COLOR")
    ribbon = batch_for_shader(line, "LINE_LOOP" if closed else "LINE_STRIP", {"pos": points})
    gpu.state.depth_test_set("LESS_EQUAL")
    gpu.state.depth_mask_set(False)
    gpu.state.blend_set("ALPHA")
    with gpu.matrix.push_pop_projection():
        gpu.matrix.load_projection_matrix(projection)
        with gpu.matrix.push_pop():
            gpu.matrix.multiply_matrix(matrix_world)
            line.bind()
            line.uniform_float("viewportSize", size)
            line.uniform_float("lineWidth", width)
            line.uniform_float("color", colour)
            ribbon.draw(line)
            if not closed:
                dot = gpu.shader.from_builtin("POINT_UNIFORM_COLOR")
                ends = batch_for_shader(dot, "POINTS", {"pos": [points[0], points[-1]]})
                gpu.state.point_size_set(END_SIZE)
                dot.bind()
                dot.uniform_float("color", colour)
                ends.draw(dot)
    gpu.state.point_size_set(1.0)
    gpu.state.blend_set("NONE")
    gpu.state.depth_mask_set(True)
    gpu.state.depth_test_set("NONE")


def draw_tool_properties(context):
    """The settings of Draw Curve in the header of its tool, or None."""
    tool = context.workspace.tools.from_space_view3d_mode("OBJECT", create=False)
    return None if tool is None else tool.operator_properties(OPENCLARE_OT_draw_curve.bl_idname)


class OPENCLARE_OT_snap_radius(bpy.types.Operator):
    """Set the radius of the snap with the mouse, as F does the radius of a brush"""

    bl_idname = "object.openclare_snap_radius"
    bl_label = "Snap Radius"
    # A setting of the tool, not of the scene: no step of undo.
    bl_options = {"REGISTER"}

    @classmethod
    def poll(cls, context):
        return context.area is not None and context.area.type == "VIEW_3D" and draw_tool_properties(context) is not None

    def invoke(self, context, event):
        self.properties_of_tool = draw_tool_properties(context)
        self.before = self.properties_of_tool.snap_radius
        mouse = (event.mouse_region_x, event.mouse_region_y)
        # The circle is drawn with its edge under the mouse, as Blender's.
        self.centre = (mouse[0] - self.before, mouse[1])
        self.mouse = mouse
        self.handle = bpy.types.SpaceView3D.draw_handler_add(self.draw, (context,), "WINDOW", "POST_PIXEL")
        context.window_manager.modal_handler_add(self)
        self.say(context)
        return {"RUNNING_MODAL"}

    def radius(self):
        gap = ((self.mouse[0] - self.centre[0]) ** 2 + (self.mouse[1] - self.centre[1]) ** 2) ** 0.5
        properties = type(self.properties_of_tool).bl_rna.properties["snap_radius"]
        return min(max(gap, properties.hard_min), properties.hard_max)

    def say(self, context):
        context.area.header_text_set(f"Snap Radius: {self.radius():.0f} px")
        context.region.tag_redraw()

    def modal(self, context, event):
        if event.type == "MOUSEMOVE":
            self.mouse = (event.mouse_region_x, event.mouse_region_y)
            self.properties_of_tool.snap_radius = self.radius()
            self.say(context)
            return {"RUNNING_MODAL"}
        if event.type in {"LEFTMOUSE", "RET", "NUMPAD_ENTER", "SPACE"} and event.value == "PRESS":
            self.properties_of_tool.snap_radius = self.radius()
            self.stop(context)
            return {"FINISHED"}
        if event.type in {"ESC", "RIGHTMOUSE"} and event.value == "PRESS":
            self.properties_of_tool.snap_radius = self.before
            self.stop(context)
            return {"CANCELLED"}
        return {"RUNNING_MODAL"}

    def stop(self, context):
        bpy.types.SpaceView3D.draw_handler_remove(self.handle, "WINDOW")
        context.area.header_text_set(None)
        context.region.tag_redraw()

    def draw(self, context):
        import math

        import gpu
        from gpu_extras.batch import batch_for_shader

        radius = self.radius()
        circle = [
            (self.centre[0] + radius * math.cos(a), self.centre[1] + radius * math.sin(a), 0.0)
            for a in (2.0 * math.pi * k / 64 for k in range(64))
        ]
        line = gpu.shader.from_builtin("POLYLINE_UNIFORM_COLOR")
        gpu.state.blend_set("ALPHA")
        line.bind()
        line.uniform_float("viewportSize", (context.region.width, context.region.height))
        line.uniform_float("lineWidth", 1.5)
        line.uniform_float("color", (1.0, 1.0, 1.0, 0.8))
        batch_for_shader(line, "LINE_LOOP", {"pos": circle}).draw(line)
        gpu.state.blend_set("NONE")


class OPENCLARE_TL_draw_curve(bpy.types.WorkSpaceTool):
    """The tool in the toolbar: the left button draws with the operator above,
    whose category is set in the header of the tool."""

    bl_space_type = "VIEW_3D"
    bl_context_mode = "OBJECT"
    bl_idname = "openclare.draw_curve"
    bl_label = "Draw Curve (openclare)"
    bl_description = (
        "Draw guide or sharp curves straight on the selected mesh: forward to draw, back along the stroke "
        "to undo, let go at its start to close it, from an end of a curve to take it up again"
    )
    bl_icon = "ops.curve.draw"
    bl_widget = None
    bl_keymap = (
        (OPENCLARE_OT_draw_curve.bl_idname, {"type": "LEFTMOUSE", "value": "PRESS"}, None),
        (OPENCLARE_OT_snap_radius.bl_idname, {"type": "F", "value": "PRESS"}, None),
    )

    def draw_settings(context, layout, tool, *, extra=False):
        properties = tool.operator_properties(OPENCLARE_OT_draw_curve.bl_idname)
        if extra:
            # The popover "..." of the header, as Blender's own tools have.
            for name in ("width", "spacing", "back_band", "close", "fit"):
                layout.prop(properties, name)
            return
        layout.prop(properties, "category", expand=True)
        layout.prop(properties, "rope")
        layout.prop(properties, "snap", expand=True)
        if properties.snap != "NONE":
            layout.prop(properties, "snap_radius")
        layout.popover("TOPBAR_PT_tool_settings_extra", text="...")


# How many points along the line drawn are looked under for the surface the
# loop must pass by; the most a loop may pass from it, in steps; how far the
# mouse moves, in pixels, before the line is cut again; and how often the
# tool looks for a cut made, in seconds.
LINE_SAMPLES = 32
UNDER_STEPS = 3.0
MOVE_PIXELS = 2.0
LOOK_EVERY = 0.02


def cut_plane(context, owner, tree, start, end):
    """The plane of the view through the line from `start` to `end` (region
    pixels) and the point of the surface under the line nearest its middle,
    in the space of `owner`: (plane, under, None), or (None, None, why there
    is none)."""
    region, view = context.region, context.region_data
    into = owner.matrix_world.inverted()
    rays = []
    for mouse in (start, end):
        origin = view3d_utils.region_2d_to_origin_3d(region, view, mouse)
        direction = view3d_utils.region_2d_to_vector_3d(region, view, mouse)
        rays += [tuple(into @ origin), tuple((into.to_3x3() @ direction).normalized())]
    plane = cut.plane_through_rays(*rays)
    if plane is None:
        return None, None, "draw a line across the limb"
    for k in sorted(range(LINE_SAMPLES + 1), key=lambda k: abs(k - LINE_SAMPLES / 2)):
        t = k / LINE_SAMPLES
        under = hit(context, owner, tree, (start[0] + t * (end[0] - start[0]), start[1] + t * (end[1] - start[1])))
        if under is not None:
            return plane, under, None
    return None, None, "the line does not cross the object"


def loop_under(surface, plane, under, farthest):
    """The loop of the cut of `surface` (`cut_surface`) by `plane` that
    passes by `under`, no farther than `farthest`: (points, None), or (None,
    why there is none). Nothing of Blender: it runs in the thread of the
    cut."""
    loop, gap = cut.nearest_loop(cut_loops(surface, *plane), under)
    if loop is None or gap > farthest:
        return None, "the cut finds no surface under the line"
    if not loop[1]:
        return None, "the cut does not go round: it runs into a hole or a border"
    return [tuple(point) for point in loop[0]], None


def cut_loop(context, owner, start, end):
    """The loop round the volume under the line from `start` to `end`
    (region pixels): (points in the space of `owner`, None), or (None, why
    there is none). At once, in the thread of Blender."""
    plane, under, why = cut_plane(context, owner, surface_tree(context, owner), start, end)
    if plane is None:
        return None, why
    return loop_under(cut_surface(context, owner), plane, under, UNDER_STEPS * step_of(owner))


class Cutter:
    """Cuts the latest line asked for, in a thread of its own: numpy only,
    nothing of Blender. A line asked for while one is cut waits, and only
    the latest waits."""

    def __init__(self, surface, farthest):
        self.surface, self.farthest = surface, farthest
        self.lock, self.wake = threading.Lock(), threading.Event()
        self.asked = self.done = None
        self.open, self.times = True, []
        self.thread = threading.Thread(target=self.run, daemon=True)
        self.thread.start()

    def ask(self, serial, plane, under):
        with self.lock:
            self.asked = (serial, plane, under)
        self.wake.set()

    def take(self):
        """The latest cut made since the last taken: (serial, (points, why)),
        or None."""
        with self.lock:
            done, self.done = self.done, None
        return done

    def close(self):
        self.open = False
        self.wake.set()

    def run(self):
        try:
            self.surface.spheres()
        except Exception:  # noqa: BLE001 - made again, and said, at the first cut
            pass
        while True:
            self.wake.wait()
            self.wake.clear()
            if not self.open:
                return
            with self.lock:
                asked, self.asked = self.asked, None
            if asked is None:
                continue
            serial, plane, under = asked
            began = time.perf_counter()
            try:
                made = loop_under(self.surface, plane, under, self.farthest)
            except Exception as error:  # noqa: BLE001 - said in the header, not lost in a thread
                made = None, f"the cut failed: {error}"
            self.times.append(time.perf_counter() - began)
            with self.lock:
                self.done = (serial, made)


class OPENCLARE_OT_cut_volume(bpy.types.Operator):
    """Draw a line across a limb, from one side to the other: the curve goes all round it, where the line cut
    it, the hidden side too"""

    bl_idname = "object.openclare_cut_volume"
    bl_label = "Cut Volume"
    bl_options = {"REGISTER", "UNDO"}

    category: bpy.props.EnumProperty(
        name="Category",
        description="What the curve made is",
        items=[
            ("guide", "Guide", "A line of edges along it"),
            ("sharp", "Sharp", "A line of hard edges along it, marked sharp and creased"),
        ],
        default="guide",
    )

    @classmethod
    def poll(cls, context):
        return OPENCLARE_OT_draw_curve.poll(context)

    def invoke(self, context, event):
        from . import CATEGORY

        self.timings = Timings("Cut Volume")
        self.start = self.end = self.asked_end = (event.mouse_region_x, event.mouse_region_y)
        self.owner = target(context)
        if self.owner is None:
            return nothing_to_draw_on(self.start)
        self.tree = surface_tree(context, self.owner)
        self.surface = cut_surface(context, self.owner)
        self.farthest = UNDER_STEPS * step_of(self.owner)
        self.cutter = Cutter(self.surface, self.farthest)
        # Each line asked for has a number: the loop shown is that of the
        # line `shown`, the latest asked for is `serial`.
        self.serial = self.shown = 0
        self.loop, self.why, self.header = None, None, None
        self.colour = CATEGORY[self.category][1]
        self.handles = (
            bpy.types.SpaceView3D.draw_handler_add(self.draw_loop, (context,), "WINDOW", "POST_VIEW"),
            bpy.types.SpaceView3D.draw_handler_add(self.draw_line, (context,), "WINDOW", "POST_PIXEL"),
        )
        self.timer = context.window_manager.event_timer_add(LOOK_EVERY, window=context.window)
        context.window_manager.modal_handler_add(self)
        self.timings.ready()
        return {"RUNNING_MODAL"}

    def modal(self, context, event):
        began = time.perf_counter()
        try:
            return self.step(context, event)
        finally:
            if event.type != "TIMER":
                self.timings.event(began)

    def step(self, context, event):
        if event.type == "MOUSEMOVE":
            self.end = (event.mouse_region_x, event.mouse_region_y)
            if distance_2d(self.end, self.asked_end) >= MOVE_PIXELS:
                self.ask(context)
            context.region.tag_redraw()
            return {"RUNNING_MODAL"}
        if event.type == "TIMER":
            done = self.cutter.take()
            if done is not None and done[0] > self.shown:
                self.show(context, done[0], *done[1])
            return {"RUNNING_MODAL"}
        if event.type == "LEFTMOUSE" and event.value == "RELEASE":
            if self.shown != self.serial or self.asked_end != self.end:
                # The loop shown is not yet that of the line let go: cut now.
                plane, under, why = cut_plane(context, self.owner, self.tree, self.start, self.end)
                self.loop, self.why = (None, why) if plane is None else loop_under(self.surface, plane, under, self.farthest)
            self.stop(context)
            if self.loop is None:
                self.report({"WARNING"}, f"openclare: no curve made: {self.why or 'draw a line across the limb'}")
                return {"CANCELLED"}
            return self.make(context)
        if event.type in {"ESC", "RIGHTMOUSE"}:
            self.stop(context)
            return {"CANCELLED"}
        return {"RUNNING_MODAL"}

    def ask(self, context):
        """The line as it is now, to be cut: what can be said at once (no
        plane, nothing under the line) is shown at once, the cut is asked
        of the thread."""
        self.serial += 1
        self.asked_end = self.end
        plane, under, why = cut_plane(context, self.owner, self.tree, self.start, self.end)
        if plane is None:
            self.show(context, self.serial, None, why)
        else:
            self.cutter.ask(self.serial, plane, under)

    def show(self, context, serial, loop, why):
        self.shown, self.loop, self.why = serial, loop, why
        header = f"Cut Volume: {why}" if why else "Cut Volume"
        if header != self.header:
            self.header = header
            context.area.header_text_set(header)
        context.region.tag_redraw()

    def make(self, context):
        from . import FIT_STEPS, drawn_curve

        middle = tuple(sum(point[k] for point in self.loop) / len(self.loop) for k in range(3))
        step = STEP_PIXELS * pixel_at(context, self.owner, middle)
        points = trace.resampled(self.loop, step, closed=True)
        drawn_curve(context, self.owner, points, True, self.category, FIT_STEPS * step)
        return {"FINISHED"}

    def stop(self, context):
        self.cutter.close()
        context.window_manager.event_timer_remove(self.timer)
        bpy.types.SpaceView3D.draw_handler_remove(self.handles[0], "WINDOW")
        bpy.types.SpaceView3D.draw_handler_remove(self.handles[1], "WINDOW")
        context.area.header_text_set(None)
        context.region.tag_redraw()
        self.timings.report(self.cutter.times)

    def draw_loop(self, context):
        self.timings.frame()
        if self.loop is not None:
            draw_stroke(context, self.owner.matrix_world, self.loop, True, self.colour)

    def draw_line(self, context):
        import gpu
        from gpu_extras.batch import batch_for_shader

        line = gpu.shader.from_builtin("POLYLINE_UNIFORM_COLOR")
        batch = batch_for_shader(line, "LINES", {"pos": [(*self.start, 0.0), (*self.end, 0.0)]})
        gpu.state.blend_set("ALPHA")
        line.bind()
        line.uniform_float("viewportSize", (context.region.width, context.region.height))
        line.uniform_float("lineWidth", 2.0)
        line.uniform_float("color", self.colour[:3] + (0.6 if self.loop is None else 1.0,))
        batch.draw(line)
        gpu.state.blend_set("NONE")


def distance_2d(a, b):
    return ((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2) ** 0.5


class OPENCLARE_TL_cut_volume(bpy.types.WorkSpaceTool):
    """The other tool of the same button: a line across a limb makes the curve
    all round it."""

    bl_space_type = "VIEW_3D"
    bl_context_mode = "OBJECT"
    bl_idname = "openclare.cut_volume"
    bl_label = "Cut Volume (openclare)"
    bl_description = (
        "Draw a line across a limb, from one side to the other: a guide or sharp curve goes all round it where "
        "the line cut it. The loop is shown while the line is drawn"
    )
    bl_icon = "ops.mesh.knife_tool"
    bl_widget = None
    bl_keymap = (
        (OPENCLARE_OT_cut_volume.bl_idname, {"type": "LEFTMOUSE", "value": "PRESS"}, None),
    )

    def draw_settings(context, layout, tool):
        properties = tool.operator_properties(OPENCLARE_OT_cut_volume.bl_idname)
        layout.prop(properties, "category", expand=True)


TOOL_IDS = {OPENCLARE_TL_draw_curve.bl_idname, OPENCLARE_TL_cut_volume.bl_idname}


def register():
    bpy.utils.register_class(OPENCLARE_OT_draw_curve)
    bpy.utils.register_class(OPENCLARE_OT_cut_volume)
    bpy.utils.register_class(OPENCLARE_OT_snap_radius)
    bpy.utils.register_tool(OPENCLARE_TL_draw_curve, separator=True, group=True)
    bpy.utils.register_tool(OPENCLARE_TL_cut_volume, after={OPENCLARE_TL_draw_curve.bl_idname})
    if not bpy.app.timers.is_registered(follow_cursor):
        bpy.app.timers.register(follow_cursor, first_interval=LOOK_FOR_CURSOR, persistent=True)


def unregister():
    if bpy.app.timers.is_registered(follow_cursor):
        bpy.app.timers.unregister(follow_cursor)
    CURSORS.clear()
    bpy.utils.unregister_tool(OPENCLARE_TL_cut_volume)
    bpy.utils.unregister_tool(OPENCLARE_TL_draw_curve)
    bpy.utils.unregister_class(OPENCLARE_OT_snap_radius)
    bpy.utils.unregister_class(OPENCLARE_OT_cut_volume)
    bpy.utils.unregister_class(OPENCLARE_OT_draw_curve)
    SURFACE.update(key=None, tree=None, cut=None)
