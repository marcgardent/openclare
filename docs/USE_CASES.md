# Use cases

What openclare is used for, noted as each one comes up: who needs it, what it asks of the tool,
what serves it, and where that stands. The Community edition is the retopology tool; the PRO
edition is the pipeline built on it, from the remesh to the export.
A French version is in [USE_CASES.fr.md](USE_CASES.fr.md).

**State:** *done* is in the add-on of `main`, *experimental* when [FEATURES.md](FEATURES.md) says so; *review* is in a pull request; *planned* has a ticket.

## Retopology

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| A sculpt made into a mesh to animate | character artists | quads, few irregular vertices, loops round eyes, mouth and joints | the remesh; loops drawn on the model with the Loops tool, a ring round a limb with Alt, moved by a drag, the quads along them shown ([LOOP_EDITING.md](LOOP_EDITING.md)) | Community | done; the loops experimental, cutting and deleting them under way |
| A mechanical part made into clean quads | hard-surface modellers | its hard edges kept as lines of edges | Hard Edges, Marked Sharp | Community | done |
| A mesh ready for subdivision | modellers | the hard edges to stay sharp under a Subdivision Surface | crease weights on the hard edges, and on the edges along a curve as the curve says | Community | done; per curve experimental |
| A mesh that keeps the shape once subdivided | modellers, character artists | the subdivided mesh, not the quad mesh, to keep the volume and lie on the shape | `--fitter limit-surface`: the limit surface of the subdivision fitted to the shape | Community | experimental, command line only (#39) |
| Seams where the artist wants them | texture artists | the UVs cut along chosen lines | curves marked Seam: the edges along them marked as seams | Community | done, experimental |
| A scan or an AI-generated mesh made usable | scanning, generation | a clean, closed surface of quads from a noisy one | the remesh, which mends torn borders and edges of more than two faces | Community | done |
| More detail where it matters | all | smaller quads on the face, the hands | painted density; Density from Shape, sizes analysed and shown | Community | done; Density from Shape experimental (#253) |
| A lighter mesh that subdivides the same | hard-surface modellers | fewer faces on the flat parts, nothing lost under a Subdivision Surface | Output: N-gons: flat parts as faces of many sides, creased all round | Community | done, experimental |
| A cage whose reflections stay clean once subdivided | hard-surface modellers (a car body, a product shot) | where the cage will make a reflection pinch or wobble under a Subdivision Surface, the worst first | Reflection Check, in Edit Mode under a Catmull-Clark Subdivision Surface: the faults listed and drawn, one angle each, measured on the cage and not on the subdivided surface; `measure` gives the same score | Community | done, experimental (#328) |
| A hand-made cage cleaned up for its reflections, topology kept | hard-surface modellers (a car body) | n-gons flattened, folded and thin corner quads straightened, creases proposed where an edge is already sharp; what needs a change of topology listed, not hidden | Repair in Place, in the Reflection Check panel (selected faces, or the whole cage, one undo step); `openclare repair`: each repair kept only if the score of the check gets better, no vertex moved beyond a share of its shortest edge; "repaired" is by the check, measured on the cage and not on the subdivided surface | Community | done, experimental (#329) |
| Round parts with a chosen number of sides | modellers, character artists | a pipe, a limb, a column: so many quads round it, whatever its radius | Quads per Turn, with sizes asked for | Community | done, experimental |
| A mesh to paint vertex colours on | texture artists, game artists | vertices spread evenly, so the paint has the same resolution everywhere rather than blotches where the sculpt is coarse and waste where it is dense | the remesh with quads of one size (Adaptive Size at 0), or Output: Triangles, For: Simulation, for even triangles | Community | done |
| Poles where the artist wants them | character artists | a pole moved off a joint or the face, pinned, the others placed round it | the poles of the last result kept on the object, those touched given to the next remesh; the tool to move them | Community | done, experimental (#249) |
| A symmetric model | character artists | the same quads on both sides | Symmetry | Community | done |
| Materials kept apart | all | each quad of the material under it, a line of edges between two | Materials | Community | done |
| Primitives with the right poles | modellers | a quad sphere, cylinder or disc | Quad Shape menu | Community | planned (#159) |

## Triangles

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| Simulation (cloth, soft body) | animators, VFX | triangles of one size, close to equilateral | Output: Triangles, For: Simulation, openclare's triangles | Community | done, experimental (#226); compared: [TRIANGLES.md](TRIANGLES.md) |
| 3D printing | makers | a closed, clean surface of triangles, true to the source | Output: Triangles, For: Shape, Blender's Decimate (Collapse), the most faithful; crossings reported | Community | done; compared: [TRIANGLES.md](TRIANGLES.md) |
| Collision meshes | game developers | a light surface of triangles, true to the shape | Output: Triangles, For: Shape, Blender's Decimate (Collapse) | Community | done; compared: [TRIANGLES.md](TRIANGLES.md) |

## Pipeline

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| A game-ready asset in one go | game artists | quads, UVs, baked maps, levels of detail, export | the pipeline built on the remesh | PRO | planned |
| A sculpt carried over to Multires | sculptors | the remesh as the base of a Multires stack rebuilt from the source | Multires from the source; its baseline, Blender's Shrinkwrap level by level, measured by `openclare detail` (lab batch `multires-baseline`); the engine's guarded projection of a level, `openclare level` and `engine.project_level` (lab batch `multires-projection`); Add Level and Remove Level in the PRO add-on, on the Community's extension points (crease weights needed, a warning before Remesh Again), a click timed by the lab batch `multires-click`; each level measured over each cage face and advised (done, one more level, denser here), the engine's `cage_face` and `previous_missing` (lab batch `multires-advice`), the report and the map in the PRO panel; Refine the Cage, the measure turned into painted density and more quads, the cage remeshed and its levels back (lab batch `multires-refine`) | PRO | under way; baseline, projection, Add/Remove Level, the advice and the map in main, Refine the Cage in review |
| Many objects at once | studios | a collection remeshed in one run | batch remesh | PRO | planned |

## Outside Blender

| Use | Who | What it asks | What serves it | Edition | State |
|---|---|---|---|---|---|
| The engine in another program | developers | a library with a stable interface | the C library and its header, in each release | open source | done |
| Scripted or batch remeshing | pipelines, research | a command line | `openclare remesh` and the other commands ([USAGE.md](../USAGE.md)) | open source | done |
