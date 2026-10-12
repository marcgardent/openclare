"""Runs the add-on in a Blender without interface, on a mesh it makes.

    blender -b --factory-startup --python blender/test_addon.py

Exits with an error if anything is not as it should be.
"""

import sys
from pathlib import Path

import bpy

sys.path.insert(0, str(Path(__file__).resolve().parent))
import openclare  # noqa: E402

openclare.register()
print("ENGINE", openclare.engine.version())

bpy.ops.mesh.primitive_monkey_add()
original = bpy.context.active_object
original.location = (1.0, 2.0, 3.0)
modifier = original.modifiers.new("subdivide", "SUBSURF")
modifier.levels = 2
triangles_before = len(original.data.polygons)

bpy.context.scene.openclare.preset = "organic"
bpy.context.scene.openclare.quads = 800
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}

remeshed = bpy.context.active_object
mesh = remeshed.data
print("RESULT", remeshed.name, len(mesh.vertices), "vertices,", len(mesh.polygons), "faces")
assert remeshed is not original and remeshed.name == f"{original.name}.quads"
assert len(original.data.polygons) == triangles_before, "the original was touched"
assert tuple(remeshed.location) == (1.0, 2.0, 3.0), "the result is not where the original is"
assert all(len(polygon.vertices) == 4 for polygon in mesh.polygons), "not all quads"
assert abs(len(mesh.polygons) - 800) < 80, f"{len(mesh.polygons)} quads for 800 asked"
assert remeshed.select_get() and not original.select_get()
# What the engine said of the run is kept, for a script to read.
said = openclare.last_report
assert int(said["quads"]) == len(mesh.polygons) and int(said["pieces"]) == 3, said
assert float(said["seconds"]) > 0.0 and said["guide_edges"] == "0", said

# Each stage is announced as it begins, and a host can stop the run.
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
source = evaluated.to_mesh()
stages = []
openclare.quads_of(source, "organic", {"quads": 800}, on_stage=lambda stage: stages.append(stage))
print("STAGES", ", ".join(stages))
# Suzanne is a head and two eyes: three pieces, each with its stages in
# their order, those of one piece among those of the others when the pieces
# are meshed at the same time.
for piece in ("piece 1 of 3", "piece 2 of 3", "piece 3 of 3"):
    own = [stage for stage in stages if stage.startswith(piece)]
    assert own[0] == f"{piece}: preprocessing" and own[-1] == f"{piece}: fitting" and len(own) >= 8, stages
assert len(stages) == sum(stage.startswith("piece ") for stage in stages), stages
assert "piece 2 of 3: cross field" in stages
assert openclare.worth_a_line("cross field") and openclare.worth_a_line("piece 2 of 3: layout")
assert openclare.worth_a_line("piece 42 of 820: preprocessing") and not openclare.worth_a_line("piece 42 of 820: layout")
assert not openclare.worth_a_line("piece 43 of 820: preprocessing")
try:
    openclare.quads_of(source, "organic", {"quads": 800}, on_stage=lambda stage: stage.endswith("layout"))
    raise AssertionError("the run was not stopped")
except openclare.engine.EngineError as error:
    assert str(error) == "stopped on request", error
evaluated.to_mesh_clear()

# The same mesh comes out whether Blender's arrays are read in place or
# copied out first.
evaluated = original.evaluated_get(bpy.context.evaluated_depsgraph_get())
source = evaluated.to_mesh()
source.calc_loop_triangles()
assert openclare.in_place(source), "this Blender does not lay its meshes out as expected"
direct, _ = openclare.quads_of(source, "organic", {"quads": 800})
really_in_place = openclare.in_place
openclare.in_place = lambda mesh: False
copied, _ = openclare.quads_of(source, "organic", {"quads": 800})
openclare.in_place = really_in_place
evaluated.to_mesh_clear()
assert len(direct.polygons) == len(copied.polygons) == len(mesh.polygons)
assert all(a.co == b.co for a, b in zip(direct.vertices, copied.vertices)), "in place and copied differ"
print("IN PLACE OK")

# A painted vertex group: with the top of the head at 1 and the rest at 0,
# and a factor of 9, the top gets most of the quads.
def share_on_top(remeshed):
    polygons = remeshed.data.polygons
    return sum(1 for polygon in polygons if polygon.center.z > 0.0) / len(polygons)

for other in bpy.context.selected_objects:
    other.select_set(False)
bpy.context.view_layer.objects.active = original
original.select_set(True)
group = original.vertex_groups.new(name="more here")
group.add([vertex.index for vertex in original.data.vertices if vertex.co.z > 0.0], 1.0, "REPLACE")
settings = bpy.context.scene.openclare
settings.adaptive_size = 0.0
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
unpainted = share_on_top(bpy.context.active_object)

bpy.context.active_object.select_set(False)
bpy.context.view_layer.objects.active = original
original.select_set(True)
settings.density_group, settings.density_factor = "more here", 9.0
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
painted = bpy.context.active_object
print("DENSITY share of the quads on top: %.2f unpainted, %.2f painted" % (unpainted, share_on_top(painted)))
assert share_on_top(painted) > unpainted + 0.1, "the painted weights changed nothing"
assert abs(len(painted.data.polygons) - 800) < 80, "the paint changed the number of quads"

# A group that is not there is said, and a preset brings its own adaptive size.
bpy.context.active_object.select_set(False)
bpy.context.view_layer.objects.active = original
original.select_set(True)
settings.density_group = "no such group"
try:
    bpy.ops.object.openclare_remesh()
    raise AssertionError("a vertex group that does not exist went unnoticed")
except RuntimeError as error:
    assert "no such group" in str(error)
assert not openclare.last_report, "a run that failed left a report"
settings.density_group = ""
settings.preset = "hard-surface"
assert settings.adaptive_size == 0.0
settings.preset = "organic"
assert settings.adaptive_size == 50.0

# Export: the result carries its two UV layers, the seams of the atlas and,
# on a shape with hard edges, those edges marked sharp.
def alone(target):
    for other in bpy.context.selected_objects:
        other.select_set(False)
    bpy.context.view_layer.objects.active = target
    target.select_set(True)

alone(original)
settings.preset = "organic"
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
assert [layer.name for layer in mesh.uv_layers] == ["openclare grid", "openclare atlas"], list(mesh.uv_layers.keys())
assert mesh.uv_layers.active.name == "openclare atlas"
atlas = [tuple(item.uv) for item in mesh.uv_layers["openclare atlas"].data]
grid = [tuple(item.uv) for item in mesh.uv_layers["openclare grid"].data]
assert len(atlas) == len(grid) == len(mesh.loops)
assert all(0.0 <= c <= 1.0 for uv in atlas for c in uv), "the atlas leaves the unit square"
assert all(c == round(c) for uv in grid for c in uv), "the grid is not on whole numbers"
seams = sum(edge.use_seam for edge in mesh.edges)
assert seams > 0 and not any(edge.use_edge_sharp for edge in mesh.edges)
print("EXPORT organic: 2 UV layers,", seams, "seam edges, no sharp edge")

bpy.ops.mesh.primitive_cube_add()
cube = bpy.context.active_object
cube.modifiers.new("subdivide", "SUBSURF").subdivision_type = "SIMPLE"
cube.modifiers["subdivide"].levels = 4
alone(cube)
settings.preset, settings.quads = "hard-surface", 600
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
sharp = sum(edge.use_edge_sharp for edge in mesh.edges)
creases = [item.value for item in mesh.attributes["crease_edge"].data]
creased = sum(1 for value in creases if value > 0)
print("EXPORT cube:", len(mesh.polygons), "quads,", sharp, "sharp edges,", creased, "creased")
assert sharp >= 12 and sharp % 12 == 0, f"{sharp} sharp edges on a cube"
# The same edges are creased, all the way, and they are the sharp ones.
assert creased == sharp and all(value in (0.0, 1.0) for value in creases)
assert all((edge.use_edge_sharp) == (creases[edge.index] == 1.0) for edge in mesh.edges)

# With the boxes unticked, the result is a bare mesh.
alone(cube)
settings.export_grid_uv = settings.export_atlas_uv = settings.export_sharp_edges = settings.export_creases = False
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
assert len(mesh.uv_layers) == 0 and not any(edge.use_seam or edge.use_edge_sharp for edge in mesh.edges)
assert "crease_edge" not in mesh.attributes
settings.export_grid_uv = settings.export_atlas_uv = settings.export_sharp_edges = settings.export_creases = True
settings.preset, settings.quads = "organic", 800

# Curves: two circles drawn round a ball, each held on it by a Shrinkwrap.
# One is put in the Guide category and the other in the Sharp one, with the
# buttons of the panel. The result has an edge loop on each, and the edges of
# the second are marked sharp and creased.
bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=5, radius=1.0, location=(4.0, 0.0, 0.0))
ball = bpy.context.active_object


def circle_at(height):
    bpy.ops.curve.primitive_bezier_circle_add(radius=1.1, location=(4.0, 0.0, height))
    circle = bpy.context.active_object
    circle.data.resolution_u = 16
    circle.modifiers.new("on the ball", "SHRINKWRAP").target = ball
    return circle


def assign(circle, category):
    alone(circle)
    assert bpy.ops.object.openclare_curves_assign(category=category) == {"FINISHED"}


loop, ridge = circle_at(0.3), circle_at(-0.3)
assign(loop, "guide")
assign(ridge, "sharp")
guides, sharps = settings.guide_curves, settings.sharp_curves
assert guides is not None and sharps is not None and guides != sharps
assert list(guides.objects) == [loop] and list(sharps.objects) == [ridge]


def coloured(drawn, colour):
    return all(abs(got - wanted) < 1e-6 for got, wanted in zip(drawn.color, colour))


assert coloured(loop, openclare.CATEGORY["guide"][1]) and coloured(ridge, openclare.CATEGORY["sharp"][1])
assert not coloured(loop, openclare.CATEGORY["sharp"][1])

# Moving a curve to the other category takes it out of the first.
assign(loop, "sharp")
assert list(guides.objects) == [] and set(sharps.objects) == {loop, ridge}
assign(loop, "guide")
assert list(guides.objects) == [loop] and list(sharps.objects) == [ridge]

# Select and Deselect act on the curves of one category.
alone(ball)
assert bpy.ops.object.openclare_curves_select(category="sharp", select=True) == {"FINISHED"}
assert ridge.select_get() and not loop.select_get()
assert bpy.ops.object.openclare_curves_select(category="sharp", select=False) == {"FINISHED"}
assert not ridge.select_get()

alone(ball)
settings.quads, settings.adaptive_size = 600, 0.0
curves = openclare.curves_of(guides, ball)
assert len(curves) == 1 and curves[0][1] and not curves[0][2], "one closed guide curve expected"
assert all(abs(float((point ** 2).sum()) ** 0.5 - 1.0) < 0.02 for point in curves[0][0]), "the circle is not on the ball"
assert openclare.curves_of(sharps, ball, sharp=True)[0][2]
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
# The Shrinkwrap brought each circle to the nearest points of the ball, a
# circle of latitude; the loop is at its height.
height = sum(float(point[2]) for point in curves[0][0]) / len(curves[0][0])
on_the_loop = [vertex.index for vertex in mesh.vertices if abs(vertex.co.z - height) < 0.01]
on_the_ridge = [vertex.index for vertex in mesh.vertices if abs(vertex.co.z + height) < 0.01]
print("CURVES a loop of", len(on_the_loop), "vertices at height %.3f," % height, len(on_the_ridge), "on the ridge")
assert len(on_the_loop) >= 12 and len(on_the_ridge) >= 12, "the curves have no loop of edges"
# Sharp and creased along the ridge, and nowhere else.
sharp = [edge for edge in mesh.edges if edge.use_edge_sharp]
weights = mesh.attributes["crease_edge"].data
assert len(sharp) >= 12, f"{len(sharp)} sharp edges"
assert all(set(edge.vertices) <= set(on_the_ridge) for edge in sharp), "a sharp edge off the ridge"
assert all(weights[edge.index].value == 1.0 for edge in sharp), "a sharp edge is not creased in full"
assert sum(1 for weight in weights if weight.value > 0) == len(sharp)

# Remove takes a curve out of its category and gives it back its colour.
alone(ridge)
assert bpy.ops.object.openclare_curves_remove(category="guide") == {"FINISHED"}
assert list(sharps.objects) == [ridge], "taken out of a category it was not in"
assert bpy.ops.object.openclare_curves_remove(category="sharp") == {"FINISHED"}
assert list(sharps.objects) == [] and ridge.users_collection and coloured(ridge, openclare.PLAIN)
alone(ball)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert not any(edge.use_edge_sharp for edge in bpy.context.active_object.data.edges)
settings.guide_curves = settings.sharp_curves = None
settings.quads, settings.adaptive_size = 800, 50.0

# Materials: a flat grid whose left third is of another material. The result
# has both, each quad that of the surface under it, and no quad across the
# border between the two.
bpy.ops.mesh.primitive_grid_add(x_subdivisions=24, y_subdivisions=24, size=2.0, location=(8.0, 0.0, 0.0))
sheet = bpy.context.active_object
for name in ("body", "trim"):
    sheet.data.materials.append(bpy.data.materials.new(name))
border = -1.0 + 2.0 * 8 / 24
for polygon in sheet.data.polygons:
    polygon.material_index = int(polygon.center.x < border)
alone(sheet)
settings.quads, settings.adaptive_size = 400, 0.0
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
mesh = bpy.context.active_object.data
assert [material.name for material in mesh.materials] == ["body", "trim"]
used = {polygon.material_index for polygon in mesh.polygons}
assert used == {0, 1}, f"materials used: {used}"
for polygon in mesh.polygons:
    xs = [mesh.vertices[v].co.x for v in polygon.vertices]
    left, right = all(x <= border + 1e-5 for x in xs), all(x >= border - 1e-5 for x in xs)
    assert left or right, f"a quad from {min(xs):.3f} to {max(xs):.3f} is across the border at {border:.3f}"
    assert polygon.material_index == int(left), "a quad has the material of the other side"
print("MATERIALS", len(mesh.polygons), "quads,", sum(p.material_index for p in mesh.polygons), "of the second material")
# Not asked for: the result has no material.
settings.export_materials = False
alone(sheet)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert len(bpy.context.active_object.data.materials) == 0
settings.export_materials = True
settings.quads, settings.adaptive_size = 800, 50.0

# A ring of wire beside a ball, in one object. The ring is thin all along:
# asked for a number of quads, the engine gives it those its thickness
# calls for out of that number.
bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=4, radius=1.0, location=(12.0, 0.0, 0.0))
body = bpy.context.active_object
bpy.ops.mesh.primitive_torus_add(
    major_radius=0.5, minor_radius=0.01, major_segments=96, minor_segments=12, location=(15.0, 0.0, 0.0)
)
wire = bpy.context.active_object
alone(body)
wire.select_set(True)
bpy.ops.object.join()
body = bpy.context.active_object
settings.quads, settings.adaptive_size = 600, 0.0
alone(body)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
kept = len(bpy.context.active_object.data.polygons)
# The number asked for is kept; the wire has the quads its thickness calls
# for, taken from the ball, which keeps half of the number.
assert openclare.last_report["thin_pieces"] == "1", openclare.last_report
assert 540 <= kept <= 660, kept
assert 250 <= int(openclare.last_report["quads_for_thin_pieces"]) <= 330, openclare.last_report
on_the_wire = sum(1 for polygon in bpy.context.active_object.data.polygons if polygon.center.x > 2.0)
assert on_the_wire > 250, on_the_wire

# Sizes asked for instead of a number: set from the object when none is,
# given to the engine as lengths of the mesh, and the number of quads is
# what the shape calls for.
alone(body)
assert settings.target == "COUNT" and settings.largest_quad == 0.0
settings.target = "SIZE"
largest, smallest = openclare.sizes_from(body)
assert abs(settings.largest_quad - largest) < 1e-6 and abs(settings.smallest_quad - smallest) < 1e-6
assert abs(largest / smallest - 8.0) < 1e-6 and 0.05 < largest < 0.2, (largest, smallest)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
sized = bpy.context.active_object
report = dict(openclare.last_report)
assert "quads_asked" not in report, report
assert abs(float(report["largest_asked"]) - largest) < 1e-6, report
made, wanted = len(sized.data.polygons), float(report["quads_wanted"])
assert wanted > 1000 and abs(made / wanted - 1.0) < 0.15, (made, wanted)
# Twice the size, the object scaled by two: the same mesh is asked for.
body.scale = (2.0, 2.0, 2.0)
bpy.context.view_layer.update()
settings.largest_quad, settings.smallest_quad = 2.0 * largest, 2.0 * smallest
alone(body)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert abs(float(openclare.last_report["largest_asked"]) - largest) < 1e-6, openclare.last_report
assert len(bpy.context.active_object.data.polygons) == made
body.scale = (1.0, 1.0, 1.0)
bpy.context.view_layer.update()
# Larger quads make fewer of them.
settings.largest_quad, settings.smallest_quad = 2.0 * largest, 2.0 * smallest
alone(body)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert len(bpy.context.active_object.data.polygons) < 0.5 * made
settings.target, settings.largest_quad, settings.smallest_quad = "COUNT", 0.0, 0.0
settings.quads, settings.adaptive_size = 800, 50.0

# Relief: the height of the original above the result subdivided, baked on
# its atlas, moves the subdivided surface back onto the original. A ball of
# radius 1: subdivided, its quad mesh shrinks inside it; with the relief it
# is on the ball again.
import numpy
from mathutils import Matrix

bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=7, radius=1.0, location=(20.0, 0.0, 0.0))
globe = bpy.context.active_object
settings.quads, settings.adaptive_size = 300, 0.0
alone(globe)
assert not bpy.ops.object.openclare_bake_relief.poll(), "nothing was remeshed from the ball itself"
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
quads = bpy.context.active_object
assert quads.openclare_source == globe and bpy.ops.object.openclare_bake_relief.poll()
settings.relief_levels, settings.relief_resolution = 2, "512"
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert int(openclare.last_report["covered"]) > 50_000 and openclare.last_report["astray"] == "0", openclare.last_report
relief = quads.modifiers["openclare relief"]
assert quads.modifiers["openclare subdivision"].levels == 2 and relief.texture.image.size[0] == 512
relief_strength = relief.strength


def off_the_ball():
    """How far the vertices of the result, as it is shown, are from the ball at most."""
    depsgraph = bpy.context.evaluated_depsgraph_get()
    depsgraph.update()
    shown = quads.evaluated_get(depsgraph).to_mesh()
    radii = numpy.array([vertex.co.length for vertex in shown.vertices])
    quads.evaluated_get(depsgraph).to_mesh_clear()
    return float(numpy.abs(radii - 1.0).max())


with_relief = off_the_ball()
relief.show_viewport = False
without = off_the_ball()
relief.show_viewport = True
assert without > 0.01 and with_relief < 0.2 * without, (without, with_relief)
# Baked again, it is the same relief updated, not another on top.
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert [modifier.name for modifier in quads.modifiers] == ["openclare subdivision", "openclare relief"]
assert abs(off_the_ball() - with_relief) < 1e-5

# The original moved aside, turned and scaled, as one does to see the two
# side by side: the relief is the same, the two still lie on one another
# each in its own space.
globe.location, globe.rotation_euler, globe.scale = (27.0, 3.0, -1.0), (0.4, 0.2, 1.0), (1.5, 1.5, 1.5)
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert openclare.last_report["astray"] == "0", openclare.last_report
assert abs(off_the_ball() - with_relief) < 1e-5
# The mesh of the original edited since, it is no longer under the result:
# no relief is made of that, and the one there is left as it is.
globe.data.transform(Matrix.Translation((3.0, 0.0, 0.0)))
try:
    bpy.ops.object.openclare_bake_relief()
    raise AssertionError("a relief was baked from an original that is elsewhere")
except RuntimeError as error:
    assert "out of reach" in str(error), error
assert abs(off_the_ball() - with_relief) < 1e-5
globe.data.transform(Matrix.Translation((-3.0, 0.0, 0.0)))

# As the displacement of the material instead: no Displace modifier, the
# height map in the material of the result, and adaptive subdivision for a
# renderer that dices to the pixel.
settings.relief_apply = "MATERIAL"
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert [modifier.name for modifier in quads.modifiers] == ["openclare subdivision"]
assert quads.modifiers["openclare subdivision"].use_adaptive_subdivision
material = quads.data.materials[0]
nodes = material.node_tree.nodes
moved = next(node for node in nodes if node.type == "OUTPUT_MATERIAL").inputs["Displacement"].links[0].from_node
assert moved.name == "openclare displacement" and material.displacement_method == "DISPLACEMENT"
assert moved.inputs["Midlevel"].default_value == 0.5
assert abs(moved.inputs["Scale"].default_value - relief_strength) < 1e-3 * relief_strength, moved.inputs["Scale"].default_value
read = moved.inputs["Height"].links[0].from_node
assert read.image == bpy.data.images[f"{quads.name} height"] and read.image.colorspace_settings.name == "Non-Color"
assert read.inputs["Vector"].links[0].from_node.uv_map == "openclare atlas"
assert abs(off_the_ball() - without) < 1e-6, "nothing but the material moves the surface now"
# Baked again: the same three nodes, brought up to date.
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert sorted(node.name for node in nodes if node.name.startswith("openclare")) == [
    "openclare atlas", "openclare displacement", "openclare height"
]

# A material the result shares with another object is copied first, and one
# that has a displacement of its own is left as it is.
shared = bpy.data.materials.new("shared")
globe.data.materials.append(shared)
quads.data.materials[0] = shared
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert quads.data.materials[0] != shared and globe.data.materials[0] == shared
assert shared.node_tree is None or "openclare height" not in shared.node_tree.nodes
own = bpy.data.materials.new("own")
own.use_nodes = True
bump = own.node_tree.nodes.new("ShaderNodeDisplacement")
out = next(node for node in own.node_tree.nodes if node.type == "OUTPUT_MATERIAL")
own.node_tree.links.new(bump.outputs["Displacement"], out.inputs["Displacement"])
quads.data.materials[0] = own
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert openclare.last_report["materials_left_alone"] == "1"
assert out.inputs["Displacement"].links[0].from_node == bump and "openclare height" not in own.node_tree.nodes

# Back to the modifier: the material is given back what it had, the surface
# is on the ball again.
quads.data.materials[0] = material
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
settings.relief_apply = "MODIFIER"
assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
assert not any(node.name.startswith("openclare") for node in nodes) and material.displacement_method == "BUMP"
assert not quads.modifiers["openclare subdivision"].use_adaptive_subdivision
assert abs(off_the_ball() - with_relief) < 1e-5

# Written as a file too: read back, it holds the heights themselves in a
# float format, and the heights between 0 and 1 in an integer one.
import tempfile

heights, said = openclare.relief_of(bpy.context, quads, globe, 2, 512)
span = max(abs(float(said["low"])), abs(float(said["high"])))


def read_back(path):
    image = bpy.data.images.load(path)
    image.colorspace_settings.name = "Non-Color"
    values = numpy.empty(512 * 512 * image.channels, numpy.float32)
    image.pixels.foreach_get(values)
    bpy.data.images.remove(image)
    return values.reshape(512, 512, -1)[..., 0]


with tempfile.TemporaryDirectory() as folder:
    settings.relief_directory = folder
    for kind, extension, tolerance in (("EXR32", "exr", 1e-7), ("EXR16", "exr", 1e-3 * span), ("PNG16", "png", 1.0 / 65535)):
        settings.relief_format = kind
        assert bpy.ops.object.openclare_bake_relief() == {"FINISHED"}
        path = openclare.last_report["file"]
        assert path.startswith(folder) and path.endswith(f"_height.{extension}"), path
        midlevel, scale = float(openclare.last_report["file_midlevel"]), float(openclare.last_report["file_scale"])
        expected = numpy.nan_to_num(heights, nan=0.0) if midlevel == 0.0 else numpy.where(
            numpy.isnan(heights), 0.5, 0.5 + heights / (2.0 * span)
        )
        assert (midlevel, abs(scale - (1.0 if midlevel == 0.0 else 2.0 * span)) < 1e-9) in ((0.0, True), (0.5, True))
        worst = float(numpy.abs(read_back(path) - expected).max())
        assert worst <= tolerance, (kind, worst, tolerance)
        assert kind[:3].lower() in quads["openclare_height_file"]
# Beside a .blend that was never saved there is nowhere to write: said.
settings.relief_directory = "//textures/"
try:
    bpy.ops.object.openclare_bake_relief()
    raise AssertionError("a file was written beside a .blend that has no place")
except RuntimeError as error:
    assert "save the .blend file first" in str(error), error
settings.relief_format, settings.relief_directory = "NONE", "//textures/"
settings.quads, settings.adaptive_size = 800, 50.0

# Nothing to remesh: said, not crashed.
bpy.ops.object.empty_add()
assert not bpy.ops.object.openclare_remesh.poll()

# Unless an object is chosen in the panel: it is the one remeshed, whatever
# is active, and only a mesh can be chosen.
settings.source = sheet
assert bpy.ops.object.openclare_remesh.poll()
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
assert bpy.context.active_object.name.startswith(sheet.name) and bpy.context.active_object != sheet
assert len(bpy.context.active_object.data.polygons) > 300
settings.source = None
bpy.ops.object.empty_add()
assert not bpy.ops.object.openclare_remesh.poll()

# The engine is loaded from a copy named after what it holds: an add-on
# updated in a Blender that has already used it does not get the engine it
# had in memory back, by its path, in place of the new one.
import tempfile
from pathlib import Path

with tempfile.TemporaryDirectory() as folder:
    installed = Path(folder) / "libopenclare_test.so"
    installed.write_bytes(b"the engine as it was")
    before = openclare.engine._own_copy(installed)
    assert before.read_bytes() == b"the engine as it was" and openclare.engine._own_copy(installed) == before
    installed.write_bytes(b"the engine after an update")
    after = openclare.engine._own_copy(installed)
    assert after != before and after.read_bytes() == b"the engine after an update"
    assert not before.exists(), "the copy of the version before is left behind"
    after.unlink()

openclare.unregister()
print("ADD-ON OK")
