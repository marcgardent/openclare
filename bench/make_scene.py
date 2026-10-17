"""Blender script: makes a scene of the bench out of objects of another file.

    blender -b --factory-startup --python bench/make_scene.py -- <source.blend> <scene.blend>
        --object NAME [--scene NAME] [--guides COLLECTION] [--sharp COLLECTION]
        [--density GROUP] [--factor F] [--preset NAME] [--quads N] [--adaptive PERCENT]

A scene of the bench is a .blend that holds one object to remesh, as it is in
the file it comes from (modifiers, materials, vertex groups), the curves drawn
on it if any, and the settings of the add-on, saved with the scene: the file
alone says how it is to be run. Only the object, the objects its modifiers
use and the curves are taken from the source; nothing else of it is copied,
and the source is not written to.
"""

import sys
from pathlib import Path

import bpy

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "blender"))
import openclare  # noqa: E402

arguments = sys.argv[sys.argv.index("--") + 1 :]
source, target = arguments[0], arguments[1]
options = {arguments[k].lstrip("-"): arguments[k + 1] for k in range(2, len(arguments) - 1, 2)}

# What the file Blender starts with holds, to be taken out afterwards.
default_objects, default_collections = list(bpy.data.objects), list(bpy.data.collections)

# What to take: the object, and the curves of the two collections.
with bpy.data.libraries.load(source, link=False) as (there, here):
    wanted_collections = [name for name in (options.get("guides"), options.get("sharp")) if name]
    missing = [name for name in [options["object"]] if name not in there.objects]
    missing += [name for name in wanted_collections if name not in there.collections]
    if missing:
        sys.exit(f"{source} has no {', '.join(missing)}")
    here.objects = [options["object"]]
    here.collections = wanted_collections

openclare.register()
scene = bpy.context.scene
# The scene starts empty: nothing of the default one is kept. What came
# with the object without being asked for (the cutter of a boolean) stays.
for existing in default_objects:
    bpy.data.objects.remove(existing)
for existing in default_collections:
    bpy.data.collections.remove(existing)
taken = bpy.data.objects[options["object"]]
scene.collection.objects.link(taken)
# What its modifiers use comes with it (the cutter of a boolean, say), and
# must be in the scene for them to work, out of the way of the renders.
for modifier in taken.modifiers:
    used = getattr(modifier, "object", None) or getattr(modifier, "target", None)
    if used is not None and used.name not in scene.collection.all_objects:
        scene.collection.objects.link(used)
        used.hide_render = True
# The object is the active one, and keeps its settings.
bpy.context.view_layer.objects.active = taken
settings = taken.openclare
for kind in ("guides", "sharp"):
    if options.get(kind):
        collection = bpy.data.collections[options[kind]]
        scene.collection.children.link(collection)
        setattr(settings, "guide_curves" if kind == "guides" else "sharp_curves", collection)
if options.get("density"):
    if options["density"] not in taken.vertex_groups:
        sys.exit(f"{options['object']} has no vertex group {options['density']}")
    settings.density_group = options["density"]
    settings.density_factor = float(options.get("factor", 4.0))
for name, value in openclare.SHAPES[options.get("preset", "organic")].items():
    setattr(settings, name, value)
settings.quads = int(options.get("quads", 3000))
if "adaptive" in options:
    settings.adaptive_size = float(options["adaptive"])

# Nothing kept that the scene does not use.
bpy.ops.outliner.orphans_purge(do_local_ids=True, do_linked_ids=True, do_recursive=True)
bpy.ops.wm.save_as_mainfile(filepath=str(Path(target).resolve()), compress=True)
meshes = [o for o in scene.collection.all_objects if o.type == "MESH"]
curves = [o for o in scene.collection.all_objects if o.type == "CURVE"]
print(f"SCENE wrote {target}: {taken.name} ({len(taken.data.vertices)} vertices, {[m.type for m in taken.modifiers]}), "
      f"{len(meshes)} meshes, {len(curves)} curves, {len(bpy.data.materials)} materials, {len(bpy.data.images)} images")
