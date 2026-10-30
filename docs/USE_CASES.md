# Use cases

What openclare is used for, noted as each one comes up: who needs it, what it asks of the tool,
what serves it, and where that stands. The Community edition is the retopology tool; the PRO
edition is the pipeline built on it, from the remesh to the export.
A French version is in [USE_CASES.fr.md](USE_CASES.fr.md).

**State:** *done* is in the add-on of `main`; *review* is in a pull request; *planned* has a ticket.

## Retopology

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| A sculpt made into a mesh to animate | character artists | quads, few irregular vertices, loops round eyes, mouth and joints | the remesh; curves drawn on the model, loops round a limb with Cut Volume | Community | done; curves in review (#203, #209) |
| A mechanical part made into clean quads | hard-surface modellers | its hard edges kept as lines of edges | Hard Edges, Marked Sharp | Community | done |
| A mesh ready for subdivision | modellers | the hard edges to stay sharp under a Subdivision Surface | crease weights on the hard edges, and on the edges along a curve as the curve says | Community | review (#203; per curve #209) |
| Seams where the artist wants them | texture artists | the UVs cut along chosen lines | curves marked Seam: the edges along them marked as seams | Community | review (#209) |
| A scan or an AI-generated mesh made usable | scanning, generation | a clean, closed surface of quads from a noisy one | the remesh, which mends torn borders and edges of more than two faces | Community | done |
| More detail where it matters | all | smaller quads on the face, the hands | painted density; Density from Shape, sizes analysed and shown | Community | done; the rest in review (#205) |
| Poles where the artist wants them | character artists | a pole moved off a joint or the face, pinned, the others placed round it | the poles of the last result kept on the object, those touched given to the next remesh; the tool to move them (DT-181) | Community | engine and project in review (#252, this one); the tool planned |
| A symmetric model | character artists | the same quads on both sides | Symmetry | Community | done |
| Materials kept apart | all | each quad of the material under it, a line of edges between two | Materials | Community | done |
| Primitives with the right poles | modellers | a quad sphere, cylinder or disc | Quad Shape menu | Community | planned (DT-157, #159) |

## Triangles

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| Simulation (cloth, soft body) | animators, VFX | triangles of one size, close to equilateral | Output: Triangles, For: Simulation, openclare's triangles | Community | review (#228, #229, #230); compared: [TRIANGLES.md](TRIANGLES.md) |
| 3D printing | makers | a closed, clean surface of triangles, true to the source | Output: Triangles, For: Shape, Blender's Decimate (Collapse), the most faithful; crossings reported | Community | review (#228, #229, #230); compared: [TRIANGLES.md](TRIANGLES.md) |
| Collision meshes | game developers | a light surface of triangles, true to the shape | Output: Triangles, For: Shape, Blender's Decimate (Collapse) | Community | review (#228, #229, #230); compared: [TRIANGLES.md](TRIANGLES.md) |

## Pipeline

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| A game-ready asset in one go | game artists | quads, UVs, baked maps, levels of detail, export | the pipeline built on the remesh | PRO | planned |
| A sculpt carried over to Multires | sculptors | the remesh as the base of a Multires stack rebuilt from the source | Multires from the source | PRO | planned |
| Many objects at once | studios | a collection remeshed in one run | batch remesh | PRO | planned |

## Outside Blender

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| The engine in another program | developers | a library with a stable interface | the C library and its header, in each release | open source | done |
| Scripted or batch remeshing | pipelines, research | a command line | `openclare remesh` and the other commands ([USAGE.md](../USAGE.md)) | open source | done |
