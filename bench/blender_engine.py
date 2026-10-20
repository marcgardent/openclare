"""The engine, run on an object of a scene of the bench with everything it
offers: what the add-on reads of the scene (blender/openclare), and the
settings it does not have, saved in the scene by an earlier add-on or by
make_scene.py, which this module reads itself: the guide and sharp curves,
the UV layers, the edges marked sharp and the crease weights.

For Blender scripts of the bench (blender_run.py, creases.py), with the
add-on of this repository importable as `openclare`.
"""

import bpy
import openclare
from openclare import engine

# The settings the add-on does not have, and what each is when the scene
# does not say: the collections of curves, and what the result carries.
CURVES = ("guide_curves", "sharp_curves")
EXPORTS = {"export_grid_uv": True, "export_atlas_uv": True, "export_sharp_edges": True, "export_creases": True}


def setting(settings, name):
    """A setting of the object, whether the add-on has it or the scene only
    keeps it as an ID property: a collection, or None, for the curves. A
    scene saved by openclare 0.1 kept its settings in the scene, not in the
    object: the add-on gives the object those it has, and the others are
    read from the scene here."""
    if name in settings.bl_rna.properties:
        return getattr(settings, name)
    value = settings.get(name)
    if value is None:
        value = bpy.context.scene.openclare.get(name)
    if name in EXPORTS:
        return EXPORTS[name] if value is None else bool(value)
    return value


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


def run(given, uv_layers=(), hard_edges=False, creases=False, curves=()):
    """Runs the engine on `given`, an `openclare.EngineInput`, asking for
    what the add-on does not: the UV layers named ("grid", "atlas", the
    seams with the atlas), the hard edges, the crease weights, the curves
    as `curves_of` gives them. Returns what `engine.remesh` does."""
    vertices, triangles, corners = given.counts
    return engine.remesh(
        given.addresses[0], vertices, given.addresses[1], triangles, given.addresses[2], corners, given.preset,
        given.settings, given.density, uv_layers=uv_layers, seams="atlas" in uv_layers, hard_edges=hard_edges,
        creases=creases, curves=curves, groups=given.groups,
    )


def remesh(original):
    """Remeshes `original` with every setting its scene keeps. Returns the
    mesh datablock made and what the engine said of the run. Raises
    `engine.EngineError` if no mesh is made."""
    settings = original.openclare
    evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
    source = evaluated.to_mesh()
    try:
        engine_settings, density = openclare.engine_settings_of(original, evaluated, source)
        given = openclare.engine_input(
            source, openclare.BASE_PRESET, engine_settings, density, settings.export_materials
        )
        layers = [name for name in ("grid", "atlas") if setting(settings, f"export_{name}_uv")]
        # A curve put in both collections by hand is a sharp one.
        sharp = curve_objects(setting(settings, "sharp_curves"))
        curves = curves_of(setting(settings, "sharp_curves"), original, sharp=True)
        curves += curves_of(setting(settings, "guide_curves"), original, but=sharp)
        made = run(
            given, layers, setting(settings, "export_sharp_edges"), setting(settings, "export_creases"), curves
        )
    finally:
        evaluated.to_mesh_clear()
    return openclare.mesh_of(made, given)


def crease_edges(mesh, pairs, weights):
    """Gives the edges of `mesh` given as `pairs` of vertices their crease
    `weights`, what a Subdivision Surface modifier reads; the others get 0."""
    import numpy

    ends = numpy.empty(2 * len(mesh.edges), numpy.int64)
    mesh.edges.foreach_get("vertices", ends)
    ends = numpy.sort(ends.reshape(-1, 2), axis=1)
    wanted = numpy.sort(pairs.astype(numpy.int64).reshape(-1, 2), axis=1)
    count = len(mesh.vertices)
    of_mesh, wanted = ends[:, 0] * count + ends[:, 1], wanted[:, 0] * count + wanted[:, 1]
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
