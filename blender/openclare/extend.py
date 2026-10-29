"""What an add-on that includes this one adds to it: the PRO edition, which
wraps the Community add-on and takes the result on through the pipeline
(UVs, bakes, export). Without such an add-on, nothing here is used.

Its panels hang under the panel of this add-on (`bl_parent_id` set to
"OPENCLARE_PT_panel") and need nothing from here. Its share of a run of
Remesh to Quads or Remesh Again goes through an `Extension`, added with
`add`:

- `asks(original)`, on Blender's thread before the run: what more the
  engine is to give, as keyword arguments of `engine.remesh` (`uv_layers`,
  `seams`). Those this add-on gives itself cannot be asked.
- `made(context, original, result, made, report)`, on Blender's thread once
  the result is made and in place: what it does with the result, given what
  the engine `made` (its UV layers, its seams). It may add to `report`, the
  dictionary of what the run said, kept on the result.

An extension that fails does not take the result away: the run says so, and
the result stays as this add-on made it.
"""

# The keyword arguments of `engine.remesh` this add-on gives itself.
OWN = {"density", "on_stage", "hard_edges", "creases", "curves", "groups", "edges"}

EXTENSIONS = []


class Extension:
    """An extension that asks nothing and does nothing: what to derive from."""

    def asks(self, original):
        return {}

    def made(self, context, original, result, made, report):
        pass


def add(extension):
    """Has the runs to come go through `extension`, after those added before."""
    if extension not in EXTENSIONS:
        EXTENSIONS.append(extension)


def remove(extension):
    if extension in EXTENSIONS:
        EXTENSIONS.remove(extension)


def asked(original):
    """What the extensions ask of the engine for a run on `original`."""
    found = {}
    for extension in EXTENSIONS:
        asks = extension.asks(original)
        taken = OWN & set(asks)
        if taken:
            raise ValueError(f"{type(extension).__name__} asks for {', '.join(sorted(taken))}, given by openclare")
        found.update(asks)
    return found
