"""The reflection check (#328): on the cage being edited, under a Catmull-Clark
Subdivision Surface, the faults that will make a reflection pinch or wobble
once it is subdivided, worst first, each ranked by one number.

The criteria are the engine's (`openclare::reflection`, called through
`engine.reflection_check`): none is worked out here. This module reads the
cage out of Edit Mode, keeps the faults found until the cage changes, lists
the N worst in a panel and draws them in the 3D viewport, with one colour
per kind of fault:

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

When the check runs. Only an edit of the geometry of the cage, as Blender
signals it (an update of the object or of its mesh with its geometry
changed), an undo, or a change of the settings makes what was found out of
date; a selection, another object, a material do not. The cage is then
looked at once it has been still for `SETTLE_SECONDS`, if a check of it
takes less than `AUTO_SECONDS`, else on demand with the Check button. A
look reads the cage from the edit mesh and writes nothing: no data, no
mesh, no property, so nothing it does can be signalled back as a change.
And a look only ever brings what was found up to date: it never makes it
out of date, so no look can lead to another. When the cage read is the one
already checked (a selection made through the edit mesh is signalled as a
change of its geometry), the engine is not called again. With the toggle
off, or the experimental features hidden, the handlers and the draw read
the toggle and return: nothing is looked at, nothing scheduled, and what
was found is taken as out of date once the check is on again.

While out of date, nothing is drawn. The overlay is built once per result
and per view, and drawn from what was built otherwise; while the view
moves, the last one built is drawn as it is, and once the view has been
still for `SETTLE_SECONDS`, it is built again for that view, out of X-ray
with each point tested for being seen (see `Visibility`).

With Blender's Developer Extras on, the panel says in one line what the
check has cost (see `Stats`).

Repair in Place (#329) is the engine's repair (`openclare::reflection::repair`,
through `engine.reflection_repair`), run by its operator only: it moves
vertices of the cage and, if asked, sets the creases it proposes, in one
undo step, on the selected faces if any is selected, else on the whole cage.
What it did is kept for the panel (see `RepairSummary`) until another
repair, an undo or a file loaded; no handler, no draw and no check reads or
runs anything of it.
"""

import time
from itertools import chain
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

# How long, in seconds, the cage, or the view, must have been still before
# it is looked at again: a drag runs no check, an orbit casts no ray.
SETTLE_SECONDS = 0.2
# A cage is looked at again by itself only if a check of it takes less than
# this, in seconds; beyond it, on demand. Before its first check, a cage is
# reckoned at `SECONDS_PER_FACE` (reading it, about 1.5 microseconds a face,
# and the engine, about 0.6 on 4 threads, measured on an i7-7700K): a cage
# of up to some 25 000 faces is kept up to date by itself.
AUTO_SECONDS = 0.05
SECONDS_PER_FACE = 2e-6
DASH_PX = 8          # length of a dash on screen
CIRCLE_SEGMENTS = 48
MAX_RAYS = 1000      # points tested for being seen, per build of the overlay

_generation = 0      # raised by an edit of the cage; never by this module's work
_last_change = 0.0   # when it was last raised, or a look last asked for
_pending = False     # a look is due
_timer_set = False
_view_timer_set = False
_looking = False
_missed = False      # a change may have gone by while the check was off
_reports = {}        # pointer of the mesh -> Report
_drawn = {}          # pointer of the region -> Drawn
_handles = []


class Stats:
    """What the check has cost since the add-on was enabled, kept in memory
    and shown with Developer Extras."""

    def __init__(self):
        self.looks = 0           # the cage read
        self.checks = 0          # the engine called
        self.unchanged = 0       # read, and found the one already checked
        self.read_seconds = 0.0  # of the last look
        self.engine_seconds = 0.0
        self.builds = 0          # the overlay built
        self.build_seconds = 0.0
        self.rays = 0            # cast by the last build
        self.rays_total = 0

    def line(self):
        return (f"{self.checks} checks of {self.looks} looks ({self.unchanged} unchanged); last read "
                f"{1000 * self.read_seconds:.1f} ms, engine {1000 * self.engine_seconds:.1f} ms; overlay built "
                f"{self.builds}×, last {1000 * self.build_seconds:.1f} ms, {self.rays} rays ({self.rays_total} in all)")


STATS = Stats()


class Report:
    """The faults of one cage, as the engine found them (`faults`, an
    `engine.Faults`), with what drawing and listing them asks for: the arrays
    the cage was read as (positions, three floats per vertex in the space of
    the object; sizes and corners of the faces; creased edges), the
    `generation` and the `tolerances` it was found under, and how long
    reading and checking it took, in seconds."""

    def __init__(self, faults, arrays, tolerances, seconds):
        import numpy

        self.faults, self.arrays, self.tolerances, self.seconds = faults, arrays, tolerances, seconds
        self.generation = _generation
        positions, sizes, corners = arrays[:3]
        self.positions, self.corners = positions.reshape(-1, 3), corners
        self.starts = numpy.concatenate(([0], numpy.cumsum(sizes, dtype=numpy.int64)))
        self._rows = {}

    def face(self, index):
        """The vertices of face `index`, in order."""
        return self.corners[self.starts[index]:self.starts[index + 1]]

    def same_cage(self, arrays):
        """Whether `arrays` read the cage this report was found on."""
        import numpy

        def same(a, b):
            if a is None or b is None:
                return a is None and b is None
            return a.shape == b.shape and numpy.array_equal(a, b)

        return all(same(a, b) for a, b in zip(self.arrays, arrays))

    def rows(self, worst):
        """What the panel lists, made once per number shown: for each of
        the `worst` first faults, its index, its words and the icon of its
        repair."""
        if worst not in self._rows:
            out = []
            for i in range(min(worst, len(self))):
                kind = engine.FAULT_KINDS[self.faults.kinds[i]]
                out.append((i, fault_name(self, i), REPAIR_ICONS[REPAIRS[kind]]))
            self._rows[worst] = out
        return self._rows[worst]

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
    owner = getattr(context, "edit_object", None)
    if owner is None or owner.type != "MESH" or not subsurf_modifiers(owner):
        return None
    return owner


def tolerances_of(owner, settings):
    return (round(settings.flat_tolerance, 6), round(settings.angle_tolerance, 9), uses_creases(owner))


def cage_arrays(owner, creases=True):
    """The cage of `owner`, in Edit Mode, as the engine takes it: positions,
    the number of corners of each face, the vertex of each corner, and the
    creased edges with their weights (from the layer crease_edge), or None
    and None when `creases` is false or there are none. Vertices and faces
    are numbered as the edit mesh numbers them.

    Read element by element from the edit mesh, which costs about a
    microsecond a face, and writes nothing: a copy to a mesh would be
    quicker, but a write to Blender's data is signalled as an update, and
    nothing this module does must be."""
    import numpy

    bm = bmesh.from_edit_mesh(owner.data)
    # The indices of the edit mesh, written into it and nowhere else: what
    # Blender itself does before reading it.
    bm.verts.index_update()
    positions = numpy.fromiter(chain.from_iterable(v.co for v in bm.verts), numpy.float32, 3 * len(bm.verts))
    sizes = numpy.fromiter((len(f.verts) for f in bm.faces), numpy.uint32, len(bm.faces))
    corners = numpy.fromiter((v.index for f in bm.faces for v in f.verts), numpy.uint32, int(sizes.sum()))
    pairs = weights = None
    layer = bm.edges.layers.float.get("crease_edge") if creases else None
    if layer is not None:
        found = [(e.verts[0].index, e.verts[1].index, e[layer]) for e in bm.edges if e[layer] > 0.0]
        if found:
            pairs = numpy.array([(a, b) for a, b, _ in found], numpy.uint32).ravel()
            weights = numpy.array([w for _, _, w in found], numpy.float32)
    return positions, sizes, corners, pairs, weights


def check(owner, settings, previous=None):
    """Reads the cage of `owner` and checks it, unless it is the one
    `previous` was found on under the same tolerances: then `previous` is
    brought up to date. Returns the `Report`, which is kept."""
    start = time.perf_counter()
    tolerances = tolerances_of(owner, settings)
    arrays = cage_arrays(owner, tolerances[2])
    read = time.perf_counter()
    STATS.looks += 1
    STATS.read_seconds = read - start
    if previous is not None and previous.tolerances == tolerances and previous.same_cage(arrays):
        STATS.unchanged += 1
        previous.generation = _generation
        return previous
    faults = engine.reflection_check(
        *arrays, flat_tolerance=settings.flat_tolerance / 100.0, angle_tolerance=settings.angle_tolerance * 180.0 / pi,
    )
    STATS.checks += 1
    STATS.engine_seconds = time.perf_counter() - read
    report = Report(faults, arrays, tolerances, time.perf_counter() - start)
    _reports[owner.data.as_pointer()] = report
    return report


def report_of(owner, settings):
    """What is known of the cage of `owner`: its `Report` and whether it is
    up to date, or (None, False)."""
    report = _reports.get(owner.data.as_pointer())
    if report is None:
        return None, False
    return report, report.generation == _generation and report.tolerances == tolerances_of(owner, settings)


def affordable(owner, report):
    """Whether a cage is looked at by itself: one that was quick to check,
    or, never checked, that is small enough to be."""
    if report is not None:
        return report.seconds < AUTO_SECONDS
    return len(bmesh.from_edit_mesh(owner.data).faces) * SECONDS_PER_FACE < AUTO_SECONDS


def fault_name(report, i):
    """What the panel says of fault `i`: "N-gon, 6 sides · pinch 12°"."""
    faults = report.faults
    return describe(engine.FAULT_KINDS[faults.kinds[i]], bool(faults.on_vertex[i]), int(faults.sides[i]),
                    float(faults.values[i]), int(faults.counts[i]))


def describe(kind, on_vertex, sides, value, count=None):
    """A fault in words: what it is on, and what is wrong; `count`, the corner
    quads that fold or are thin, if known."""
    if on_vertex:
        what = f"Pole, {sides} edges"
    else:
        what = {3: "Triangle", 4: "Quad"}.get(sides, f"N-gon, {sides} sides")

    def degrees(angle):
        return f"{angle:.1f}°" if angle < 10.0 else f"{angle:.0f}°"

    def quads(n):
        if n is None:
            return "corner quads"
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
    return report.rows(worst)


# ---------------------------------------------------------------- repair in place

# Why a fault is left after a repair, in the panel's words.
REASON_WORDS = {
    "change_topology": "needs a change of topology",
    "too_far": "would move a vertex beyond the bound",
    "would_worsen": "would worsen the score",
    "proposed_crease": "a proposed crease cures it",
    "outside_selection": "outside the selection",
    "out_of_passes": "not tried: out of passes",
}


class RepairSummary:
    """What the last repair of a cage did, for the panel to show: where
    (`faces`, how many were asked for, or None for the whole cage), how many
    vertices moved and how far, the creases proposed and set, the score
    before and after per kind, and the faults left with their reason. Made
    once, by the operator; kept until another repair, an undo or a file
    loaded."""

    def __init__(self, result, faces, set_creases):
        self.faces, self.set_creases = faces, set_creases
        self.moved, self.largest, self.share = result.moved, result.largest, result.share
        self.proposed = len(result.crease_edges)
        after = "with_creases" if set_creases else "after"
        self.table = [
            (kind, int(result.scores["before"][0][k]), int(result.scores[after][0][k]))
            for k, kind in enumerate(engine.FAULT_KINDS)
        ]
        self.left = []
        for i in range(len(result.left_kinds)):
            reason = engine.REPAIR_REASONS[result.left_reasons[i]]
            if set_creases and reason == "proposed_crease":
                continue
            kind = engine.FAULT_KINDS[result.left_kinds[i]]
            words = describe(kind, bool(result.left_on_vertex[i]), int(result.left_sides[i]),
                             float(result.left_values[i]))
            self.left.append((words, REASON_WORDS[reason]))

    def headline(self):
        where = "the whole cage" if self.faces is None else f"{self.faces} selected faces"
        vertices = "1 vertex" if self.moved == 1 else f"{self.moved} vertices"
        moved = (f"{vertices} moved, the furthest by {100.0 * self.share:.1f} % of its shortest edge"
                 if self.moved else "no vertex moved")
        creases = f"{self.proposed} creases proposed" + (", set" if self.set_creases and self.proposed else "")
        return f"On {where}: {moved}; {creases}"


_repairs = {}        # pointer of the mesh -> RepairSummary


def repair(owner, settings, bound, set_creases, crease_fold):
    """Repairs the cage of `owner` in place, in Edit Mode (see
    `openclare::reflection::repair` in the engine): the selected faces if
    any is, else the whole cage. Moves the vertices the engine moved, sets
    the creases it proposes if `set_creases`, and keeps the summary. `bound`
    is a share of the shortest edge of a vertex, `crease_fold` an angle in
    radians. Returns the `RepairSummary`."""
    import numpy

    arrays = cage_arrays(owner, uses_creases(owner))
    positions = arrays[0].reshape(-1, 3)
    bm = bmesh.from_edit_mesh(owner.data)
    chosen = [i for i, face in enumerate(bm.faces) if face.select]
    result = engine.reflection_repair(
        *arrays, within=chosen or None, flat_tolerance=settings.flat_tolerance / 100.0,
        angle_tolerance=settings.angle_tolerance * 180.0 / pi, bound=bound, crease_fold=crease_fold * 180.0 / pi,
    )
    moved = numpy.nonzero((result.positions != positions).any(axis=1))[0]
    bm.verts.ensure_lookup_table()
    for v in moved:
        bm.verts[int(v)].co = result.positions[v].tolist()
    set_count = 0
    if set_creases and len(result.crease_edges):
        layer = bm.edges.layers.float.get("crease_edge") or bm.edges.layers.float.new("crease_edge")
        for a, b in result.crease_edges:
            edge = bm.edges.get((bm.verts[int(a)], bm.verts[int(b)]))
            if edge is not None:
                edge[layer] = 1.0
                set_count += 1
    if len(moved) or set_count:
        bmesh.update_edit_mesh(owner.data)
    summary = RepairSummary(result, len(chosen) if chosen else None, set_creases)
    _repairs[owner.data.as_pointer()] = summary
    return summary


# ---------------------------------------------------------------- looking again


def look(context=None, force=False):
    """Brings what was found of the cage in Edit Mode up to date, if the
    check is shown and the cage quick enough to check, or `force`. Returns
    the report, or None. Never makes anything out of date."""
    global _pending, _looking
    context = context or bpy.context
    _pending = False
    scene = getattr(context, "scene", None)
    if scene is None or not active(scene):
        return None
    owner = checked_object(context)
    if owner is None:
        return None
    settings = settings_of(scene)
    report, current = report_of(owner, settings)
    if current or not (force or affordable(owner, report)):
        return report
    _looking = True
    try:
        report = check(owner, settings, report)
    finally:
        _looking = False
    tag_redraw(context)
    return report


def _wait():
    """The timer: looks once the cage has been still for SETTLE_SECONDS."""
    global _timer_set
    left = SETTLE_SECONDS - (time.monotonic() - _last_change)
    if left > 1e-3:
        return left
    _timer_set = False
    try:
        look()
    except (ReferenceError, RuntimeError, ValueError, engine.EngineError) as error:
        print(f"openclare: reflection check: {error}")
    return None


def _ask_for_look():
    """A look is due: one, once the cage has been still for SETTLE_SECONDS,
    however many times this is asked meanwhile."""
    global _pending, _timer_set, _last_change
    _pending = True
    _last_change = time.monotonic()
    if not _timer_set and not bpy.app.background:
        _timer_set = True
        bpy.app.timers.register(_wait, first_interval=SETTLE_SECONDS)


def _cage_changed():
    global _generation
    _generation += 1
    _ask_for_look()


def edits_a_cage(depsgraph, owner=None):
    """Whether an update Blender signals is an edit of the geometry of a
    cage: of the object `owner` or of its mesh, or of a mesh something was
    found on (edited in Object Mode, say, between two sessions of Edit
    Mode), with its geometry changed."""
    for update in depsgraph.updates:
        if not update.is_updated_geometry:
            continue
        original = update.id.original
        if owner is not None and (original == owner or original == owner.data):
            return True
        mesh = original.data if isinstance(original, bpy.types.Object) else original
        if isinstance(mesh, bpy.types.Mesh) and mesh.as_pointer() in _reports:
            return True
    return False


def active(scene):
    """Whether the check is on: its toggle, and the experimental features
    shown. Read first by every handler and by the draw, which return at once
    when it is not."""
    settings = getattr(scene, "openclare_reflection", None)
    return settings is not None and settings.show and experimental.shown()


def _catch_up():
    """What was found may have been made out of date while the check was off:
    taken as such, and looked at."""
    global _missed
    _missed = False
    _cage_changed()


@persistent
def _changed(_scene=None, depsgraph=None):
    """Blender signals an update. Only an edit of the geometry of a cage
    counts, and none while this module is looking. While the check is off,
    nothing is looked at, and the next time it is on, everything found is
    out of date."""
    global _missed
    if _looking or depsgraph is None:
        return
    if not active(bpy.context.scene):
        _missed = True
        return
    if _missed or edits_a_cage(depsgraph, checked_object(bpy.context)):
        _catch_up()


@persistent
def _undone(*_args):
    global _missed
    # What the last repair did no longer describes the cage.
    _repairs.clear()
    if not active(bpy.context.scene):
        _missed = True
        return
    _catch_up()


@persistent
def _loaded(*_args):
    global _missed
    _reports.clear()
    _drawn.clear()
    _repairs.clear()
    _missed = True


def settings_changed(_settings, context):
    # The toggle and the tolerances: what was found may be out of date, and
    # is looked at again (and found the same, if the cage is).
    if active(context.scene):
        _catch_up()
    tag_redraw(context)


def worst_changed(_settings, context):
    # Show the N Worst only redraws.
    tag_redraw(context)


def tag_redraw(context=None, kinds=("VIEW_3D", "PROPERTIES")):
    wm = (context or bpy.context).window_manager
    for window in wm.windows:
        for area in window.screen.areas if window.screen else ():
            if area.type in kinds:
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
        update=worst_changed,
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
    # Those of Repair in Place: read when it runs, never by a check.
    repair_bound: bpy.props.FloatProperty(
        name="Bound",
        description="How far Repair in Place may move a vertex, as a share of the shortest of its edges: a repair "
                    "that needs more is refused and listed",
        default=10.0,
        min=0.0,
        soft_max=50.0,
        precision=1,
        subtype="PERCENTAGE",
    )
    repair_creases: bpy.props.BoolProperty(
        name="Apply the Proposed Creases",
        description="Set a full crease on each edge Repair in Place proposes: edges already folding by Sharp From "
                    "or more, where a crease cures a pinch or a pole on a curved zone. Off, they are only counted",
        default=False,
    )
    crease_fold: bpy.props.FloatProperty(
        name="Sharp From",
        description="Fold between the two faces of an edge from which it is a sharp edge of the shape, where Repair "
                    "in Place may propose a crease",
        default=radians(30.0),
        min=0.0,
        max=pi,
        subtype="ANGLE",
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
        owner = checked_object(context)
        settings = settings_of(context.scene)
        report = check(owner, settings, report_of(owner, settings)[0])
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
            # The numbering of the cage may have changed: read afresh.
            report = check(owner, settings, report)
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
        # Signalled as an edit of the geometry: the cage is read again once
        # still, found the same, and not checked again.
        bmesh.update_edit_mesh(owner.data, loop_triangles=False, destructive=False)
        frame_selected(context)
        return {"FINISHED"}


class OPENCLARE_OT_reflection_repair(bpy.types.Operator):
    """Experimental. Move vertices of the cage, every vertex, edge and face kept, to flatten its n-gons and make the faces whose corner quads fold or are thin star-shaped again; propose creases on edges already sharp. Each repair is kept only if the reflection check scores better. The selected faces if any, else the whole cage"""

    bl_idname = "mesh.openclare_reflection_repair"
    bl_label = "Repair in Place"
    bl_options = {"REGISTER", "UNDO"}

    bound: bpy.props.FloatProperty(
        name="Bound",
        description="How far a vertex may move, as a share of the shortest of its edges",
        default=10.0,
        min=0.0,
        soft_max=50.0,
        precision=1,
        subtype="PERCENTAGE",
    )
    apply_creases: bpy.props.BoolProperty(
        name="Apply the Proposed Creases",
        description="Set a full crease on each edge proposed",
        default=False,
    )
    crease_fold: bpy.props.FloatProperty(
        name="Sharp From",
        description="Fold between the two faces of an edge from which a crease may be proposed on it",
        default=radians(30.0),
        min=0.0,
        max=pi,
        subtype="ANGLE",
    )

    @classmethod
    def poll(cls, context):
        return checked_object(context) is not None

    def execute(self, context):
        owner = checked_object(context)
        summary = repair(owner, settings_of(context.scene), self.bound / 100.0, self.apply_creases, self.crease_fold)
        tag_redraw(context)
        self.report({"INFO"}, summary.headline())
        if self.apply_creases and summary.proposed and not uses_creases(owner):
            self.report({"WARNING"}, "Use Creases is off on the Subdivision Surface: the creases set do not count")
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
    """The panel, on `layout`: the settings, then the faults found. It reads
    what was found and changes nothing: no look, no check."""
    settings = settings_of(context.scene)
    layout.label(text="Angles are measured on the cage, not on the subdivided surface", icon=experimental.ICON)
    column = layout.column(align=True)
    column.prop(settings, "worst")
    column.prop(settings, "flat_tolerance")
    column.prop(settings, "angle_tolerance")
    if context.preferences.view.show_developer_ui:
        layout.label(text=STATS.line())
    if not settings.show:
        return
    if _missed:
        # Back on: what was found is taken as out of date and looked at
        # again (a timer, not here).
        _catch_up()
    owner = checked_object(context)
    if owner is None:
        layout.label(text="In Edit Mode, the faults of the cage are listed here", icon="INFO")
        return
    report, current = report_of(owner, settings)
    if report is None or not (current or _pending):
        row = layout.row()
        row.operator(OPENCLARE_OT_reflection_check.bl_idname, icon="FILE_REFRESH")
        if report is not None:
            row.label(text=f"Out of date: a check takes {1000.0 * report.seconds:.0f} ms", icon="TIME")
    if report is not None:
        if not uses_creases(owner):
            layout.label(text="Use Creases is off: only borders are sharp", icon="INFO")
        listed = report.rows(settings.worst)
        layout.label(text=f"{len(listed)} shown out of {len(report)}; {report.faults.poles} poles looked at")
        box = layout.column(align=True)
        for index, words, icon in listed:
            box.operator(OPENCLARE_OT_reflection_select.bl_idname, text=words, icon=icon, emboss=False).index = index
        if listed:
            legend = layout.row(align=True)
            for mends in ("move", "topology"):
                legend.label(text=REPAIR_WORDS[mends], icon=REPAIR_ICONS[mends])
    draw_repair(layout, owner, settings)


def draw_repair(layout, owner, settings):
    """The Repair in Place block: its settings, its button, and what the last
    repair of this cage did. Reads what was kept, runs nothing."""
    box = layout.box()
    box.label(text="Repair in Place: the selected faces, or the whole cage", icon=experimental.ICON)
    column = box.column(align=True)
    column.prop(settings, "repair_bound")
    column.prop(settings, "repair_creases")
    column.prop(settings, "crease_fold")
    button = box.operator(OPENCLARE_OT_reflection_repair.bl_idname, icon="MODIFIER")
    button.bound, button.apply_creases = settings.repair_bound, settings.repair_creases
    button.crease_fold = settings.crease_fold
    summary = _repairs.get(owner.data.as_pointer())
    if summary is None:
        return
    box.label(text=summary.headline())
    after = "after, creases set" if summary.set_creases else "after"
    header = box.row()
    for text in ("", "before", after):
        header.label(text=text)
    for kind, before, now in summary.table:
        if before or now:
            row = box.row()
            row.label(text=kind.replace("_", " "))
            row.label(text=str(before))
            row.label(text=str(now))
    box.label(text=f"{len(summary.left)} left" + (", the worst first:" if summary.left else ""))
    rows_left = box.column(align=True)
    for words, reason in summary.left[:settings.worst]:
        rows_left.label(text=f"{words}: {reason}")


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
    """In X-ray, while the view moves, and with no view to test against,
    everything is seen."""

    rays = 0

    def sees(self, owner, co):
        return True


class Visibility:
    """Says whether a point of the cage is seen from the eye, out of X-ray.

    A ray goes from the eye to the point: it is hidden if the ray hits
    something before. In Edit Mode, scene.ray_cast hits the cage of the
    object edited, not its subdivided surface: the cage hides itself
    exactly, and other objects hide it by their surface. One ray per point,
    at most MAX_RAYS for one build of the overlay. The depsgraph is the one
    the view drew with, taken as it is: asking for it evaluated could make
    Blender evaluate it, from inside a draw."""

    def __init__(self, context):
        self.region, self.view = context.region, context.region_data
        self.scene = context.scene
        self.depsgraph = context.view_layer.depsgraph
        self.rays = 0
        self.known = {}

    def sees(self, owner, co):
        """`co` in the space of the object."""
        key = tuple(round(x, 6) for x in co)
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


def xray(space):
    shading = space.shading
    return shading.show_xray_wireframe if shading.type == "WIREFRAME" else shading.show_xray


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


class Drawn:
    """The overlay built for one region: from which report, how many faults,
    for which view and the matrix of the object, what (`segments`, as
    `overlay` gives them) and its GPU batches once made; and when the view
    last moved."""

    def __init__(self, report, worst, view, segments, moved_at, rays):
        self.report, self.worst, self.built_for, self.segments = report, worst, view, segments
        self.view, self.moved_at, self.rays = view, moved_at, rays
        self.batches = None


def overlay_for(key, owner, report, worst, view, seen, region=None, region_view=None, now=None):
    """The overlay of region `key` for `report` seen from `view` (anything
    that changes when the view or the object moves), built only when the
    report, the number shown, or a view still for SETTLE_SECONDS is not what
    it was built for; `seen()` makes what tells which points are seen.
    Returns the `Drawn` and whether it was built now."""
    now = time.monotonic() if now is None else now
    drawn = _drawn.get(key)
    if drawn is not None and drawn.view != view:
        drawn.view, drawn.moved_at = view, now
    moving = drawn is not None and now - drawn.moved_at < SETTLE_SECONDS
    if drawn is not None and drawn.report is report and drawn.worst == worst and (moving or drawn.built_for == view):
        return drawn, False
    start = time.perf_counter()
    looker = SeeAll() if moving else seen()
    segments = overlay(owner, report, worst, looker, region, region_view)
    drawn = Drawn(report, worst, view if not moving else None, segments,
                  drawn.moved_at if drawn is not None else now - SETTLE_SECONDS, looker.rays)
    drawn.view = view
    _drawn[key] = drawn
    STATS.builds += 1
    STATS.build_seconds = time.perf_counter() - start
    STATS.rays = looker.rays
    STATS.rays_total += looker.rays
    return drawn, True


def _view_settled():
    global _view_timer_set
    _view_timer_set = False
    tag_redraw(kinds=("VIEW_3D",))
    return None


def _batches(segments):
    import gpu
    from gpu_extras.batch import batch_for_shader

    lines, vertices, centres = segments
    out = []
    if lines:
        shader = gpu.shader.from_builtin("POLYLINE_SMOOTH_COLOR")
        positions = [p for a, b, _ in lines for p in (a, b)]
        colours = [c for _, _, c in lines for _ in (0, 1)]
        out.append(("lines", shader, batch_for_shader(shader, "LINES", {"pos": positions, "color": colours})))
    shader = gpu.shader.from_builtin("POINT_FLAT_COLOR")
    for name, points in (("vertices", vertices), ("centres", centres)):
        if points:
            batch = batch_for_shader(shader, "POINTS", {"pos": [p for p, _ in points], "color": [c for _, c in points]})
            out.append((name, shader, batch))
    return out


def draw_view():
    """Draws the worst faults of the cage in Edit Mode, when the check is
    shown, the overlays are, and what was found is up to date: from the
    batches already built, unless they are for another report or view."""
    global _view_timer_set
    import gpu

    context = bpy.context
    if not active(context.scene):
        return
    space = context.space_data
    if space is None or not space.overlay.show_overlays:
        return
    if _missed:
        _catch_up()
        return
    settings = settings_of(context.scene)
    owner = checked_object(context)
    if owner is None:
        return
    report, current = report_of(owner, settings)
    if report is None or not current:
        return
    region, region_view = context.region, context.region_data
    see_all = xray(space)
    view = (tuple(map(tuple, region_view.perspective_matrix)), region.width, region.height, see_all,
            tuple(map(tuple, owner.matrix_world)))
    drawn, built = overlay_for(
        region.as_pointer(), owner, report, settings.worst, view,
        SeeAll if see_all else (lambda: Visibility(context)), region, region_view,
    )
    if drawn.built_for != view and not _view_timer_set:
        # Drawn as last built while the view moves; built again once it stops.
        _view_timer_set = True
        bpy.app.timers.register(_view_settled, first_interval=SETTLE_SECONDS)
    if built or drawn.batches is None:
        drawn.batches = _batches(drawn.segments)
    scale = context.preferences.system.ui_scale
    gpu.state.blend_set("ALPHA")
    gpu.state.depth_test_set("NONE")
    for name, shader, batch in drawn.batches:
        shader.bind()
        if name == "lines":
            shader.uniform_float("viewportSize", (region.width, region.height))
            shader.uniform_float("lineWidth", 3.0 * scale)
        else:
            gpu.state.point_size_set((9 if name == "vertices" else 6) * scale)
        batch.draw(shader)
    gpu.state.point_size_set(1.0)
    gpu.state.blend_set("NONE")


# ---------------------------------------------------------------- registration

CLASSES = (
    OpenclareReflectionSettings,
    OPENCLARE_OT_reflection_check,
    OPENCLARE_OT_reflection_select,
    OPENCLARE_OT_reflection_repair,
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
    global _timer_set, _view_timer_set
    for handle in _handles:
        bpy.types.SpaceView3D.draw_handler_remove(handle, "WINDOW")
    _handles.clear()
    for handlers, handler in HANDLERS:
        if handler in handlers:
            handlers.remove(handler)
    for timer in (_wait, _view_settled):
        if bpy.app.timers.is_registered(timer):
            bpy.app.timers.unregister(timer)
    _timer_set = _view_timer_set = False
    _reports.clear()
    _drawn.clear()
    _repairs.clear()
    del bpy.types.Scene.openclare_reflection
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
