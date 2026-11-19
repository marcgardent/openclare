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
- `needs(original)`, on Blender's thread before the run: which of the
  outputs this add-on gives itself the extension cannot do without, among
  `NEEDABLE`, switched on for the run whatever the panel says (the PRO
  Multires needs the cage's crease weights).
- `replacing(result)`, before Remesh Again replaces the mesh of `result`
  with a new one: what that would lose, as a sentence, or nothing. Clicked
  in Blender, Remesh Again asks for a confirmation with those sentences;
  from a script it says them as warnings and goes on.

An extension that fails does not take the result away: the run says so, and
the result stays as this add-on made it.
"""

# The keyword arguments of `engine.remesh` this add-on gives itself.
OWN = {"density", "on_stage", "hard_edges", "creases", "curves", "groups", "edges"}

# Those an extension may need switched on: crease weights on the result.
NEEDABLE = {"creases"}

EXTENSIONS = []


class Extension:
    """An extension that asks nothing and does nothing: what to derive from."""

    def asks(self, original):
        return {}

    def made(self, context, original, result, made, report):
        pass

    def needs(self, original):
        return set()

    def replacing(self, result):
        return None


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


def needed(original):
    """What the extensions need switched on for a run on `original`."""
    found = set()
    for extension in EXTENSIONS:
        needs = set(extension.needs(original))
        other = needs - NEEDABLE
        if other:
            raise ValueError(f"{type(extension).__name__} needs {', '.join(sorted(other))}, which cannot be needed")
        found |= needs
    return found


def warnings(result):
    """What the extensions say Remesh Again would lose of `result`, one
    sentence each."""
    return [said for said in (extension.replacing(result) for extension in EXTENSIONS) if said]
