"""Runs Optimize for Subdivision Surface, the option of a remesh for a
result to be subdivided (#330), in a Blender without interface.

    blender -b --factory-startup --python blender/test_subdivision.py

Exits with an error if anything is not as it should be.
"""

import sys
from pathlib import Path

import bpy

sys.path.insert(0, str(Path(__file__).resolve().parent))
import openclare  # noqa: E402
from openclare import experimental  # noqa: E402

openclare.register()
print("ENGINE", openclare.engine.version())


def alone(target):
    for other in bpy.context.selected_objects:
        other.select_set(False)
    bpy.context.view_layer.objects.active = target
    target.select_set(True)


def given(target):
    """The settings the engine would be given for `target`."""
    evaluated = target.evaluated_get(bpy.context.evaluated_depsgraph_get())
    source = evaluated.to_mesh()
    try:
        return openclare.engine_settings_of(target, evaluated, source, write=False)[0]
    finally:
        evaluated.to_mesh_clear()


# Off by default; on, the engine is told, and only for quads or n-gons.
bpy.ops.mesh.primitive_cube_add()
box = bpy.context.active_object
settings = box.openclare
assert not settings.for_subdivision
assert "for_subdivision" not in given(box)
settings.for_subdivision = True
assert given(box)["for_subdivision"] == "true"
settings.output = "TRIANGLES"
assert "for_subdivision" not in given(box)
settings.output = "QUADS"
# Hidden with the experimental features, the engine is given its default.
experimental.assumed = False
assert "for_subdivision" not in given(box)
experimental.assumed = True
print("SETTINGS for_subdivision given only when asked for, shown, and not of triangles")

# A cube: its corners on its creased edges, the result has the creases
# whatever Carry says, and the panel of the result says what the repair did.
settings.quads, settings.export_creases = 600, False
alone(box)
assert bpy.ops.object.openclare_remesh() == {"FINISHED"}
result = bpy.context.active_object
assert result is not box and all(len(polygon.vertices) == 4 for polygon in result.data.polygons)
creases = result.data.attributes["crease_edge"]
full = sum(1 for value in creases.data if value.value == 1.0)
# The twelve edges of the cube, each cut into several edges of the result.
assert full >= 12, full
told = result[openclare.REPORT]
said = {key: told[key] for key in told.keys()}
assert int(said["subdivision_folded_after"]) == 0 and int(said["subdivision_moved"]) == 0, said
lines = openclare.report_lines(told)
assert any(line.startswith("For subdivision: folded faces 0 to 0, thin 0 to 0") for line, _ in lines), lines
print("CUBE", len(result.data.polygons), "quads,", full, "edges fully creased; the repair said")

# The Output panel shows the checkbox and that creases are always carried.
logged = []


class Drawn:
    """A stand-in layout that logs what is drawn on it."""

    def __init__(self, log):
        self.log = log
        self.active = True

    def __getattr__(self, name):
        if name in ("row", "column", "box", "split"):
            return lambda *args, **kwargs: self
        if name == "prop":
            return lambda owner, key, **kwargs: self.log.append(("prop", key, kwargs.get("icon", "NONE")))
        if name == "label":
            return lambda **kwargs: self.log.append(("label", kwargs.get("text", ""), kwargs.get("icon", "NONE")))
        if name == "prop_enum":
            return lambda owner, key, value, **kwargs: self.log.append(("prop_enum", key, value))
        return lambda *args, **kwargs: self


openclare.OPENCLARE_PT_output.draw_settings(None, bpy.context, Drawn(logged), box, settings)
assert ("prop", "for_subdivision", experimental.ICON) in logged, logged
assert ("label", "Creases: always, for a subdivision", "CHECKMARK") in logged, logged
logged.clear()
experimental.assumed = False
openclare.OPENCLARE_PT_output.draw_settings(None, bpy.context, Drawn(logged), box, settings)
assert all(entry[1] != "for_subdivision" for entry in logged), logged
experimental.assumed = True
print("PANEL the checkbox, experimental, and the creases always carried")

openclare.unregister()
print("SUBDIVISION OK")
