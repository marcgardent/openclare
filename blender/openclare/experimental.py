"""The features the add-on calls experimental, and the toggle of its
preferences that shows them or hides them.

A feature is experimental by the rule of docs/FEATURES.md: not yet tested
enough, or with an issue of severity high or medium open against it. Shown,
each carries the icon EXPERIMENTAL; hidden, its settings are gone from the
panels, its tools from the toolbar, and the engine is given the value the
setting has by default, not the one the object keeps: what runs is what the
stable features make.
"""

import bpy

ICON = "EXPERIMENTAL"
# The settings of an object (OpenclareSettings) that are experimental, and
# the value the engine is given while they are hidden; None is the default of
# the setting. Triangles for the simulation are experimental and those for
# the shape (Blender's Decimate) are not: hidden, triangles are for the shape.
SETTINGS = {
    "target": "COUNT",
    "triangles_for": "SHAPE",
    "quads_around": None,
    "follow_curvature": None,
    "fewer_irregular": None,
    "volume_fit": None,
}
# Whether they are shown where the preferences cannot be read: the add-on
# registered by a script rather than enabled (the tests).
assumed = True


def shown():
    """Whether the experimental features are shown: the toggle of the
    add-on's preferences."""
    addon = bpy.context.preferences.addons.get(__package__)
    if addon is None or addon.preferences is None:
        return assumed
    return addon.preferences.experimental


def value(settings, name):
    """The value the engine is given for the setting `name` of `settings`:
    its own, or while experimental features are hidden and it is one of
    them, the one of `SETTINGS`."""
    if name not in SETTINGS or shown():
        return getattr(settings, name)
    hidden = SETTINGS[name]
    return settings.bl_rna.properties[name].default if hidden is None else hidden


class Effective:
    """The settings of an object as the engine is to take them: read through
    `value`, written as they are."""

    def __init__(self, settings):
        object.__setattr__(self, "settings", settings)

    def __getattr__(self, name):
        return value(self.settings, name)

    def __setattr__(self, name, given):
        setattr(self.settings, name, given)


def effective(settings):
    return settings if isinstance(settings, Effective) else Effective(settings)


# The tools of the toolbar that are experimental: registered while shown.
# Each module gives a function that registers its tools, or unregisters them.
TOOLS = []
registered = False


def apply():
    """The tools in the toolbar as `shown` says, and every area redrawn."""
    global registered
    wanted = shown()
    if wanted != registered:
        for tools in TOOLS:
            tools(wanted)
        registered = wanted
    try:
        windows = bpy.context.window_manager.windows
    except AttributeError:  # Blender starting: nothing drawn yet
        return
    for window in windows:
        for area in window.screen.areas:
            area.tag_redraw()


def toggled(preferences, context):
    apply()


class OpenclarePreferences(bpy.types.AddonPreferences):
    bl_idname = __package__

    experimental: bpy.props.BoolProperty(
        name="Experimental Features",
        description=(
            "Show the features that are not stable yet, marked with this icon: Ask For Sizes and Analyse, "
            "Quads Around Thin Parts, Follow Curvature, Fewer Irregular Vertices, Volume Fit, triangles for "
            "simulation, the curves and their tools, the poles and their tool. Hidden, they are left out of "
            "the panels and the toolbar, and a remesh gives them their default values"
        ),
        default=True,
        update=toggled,
    )

    def draw(self, context):
        self.layout.prop(self, "experimental", icon=ICON)


def register(*tools):
    TOOLS[:] = tools
    bpy.utils.register_class(OpenclarePreferences)
    apply()


def unregister():
    global registered
    if registered:
        for tools in TOOLS:
            tools(False)
        registered = False
    TOOLS.clear()
    bpy.utils.unregister_class(OpenclarePreferences)
