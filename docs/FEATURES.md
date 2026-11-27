# Features

*[Version française](FEATURES.fr.md)*

Every feature of openclare, in the Blender add-on, on the command line and as a library, and how
far it can be relied on. What is left to do with each is in the [roadmap](../ROADMAP.md).

State on 31 October 2026, add-on 0.18.0. In the add-on, the experimental features carry the
icon of a flask, and *Experimental Features* in its preferences hides them all.

## Stable or experimental

A feature is **stable** when all of the following hold:

- it is on `main`, and so in the next release;
- a test covers it, the engine's or the add-on's run in Blender;
- it does what it says on the bench, or on a case whose answer is known;
- no issue of severity *high* or *medium* is open against it.

Otherwise it is **experimental**: it runs and can be used, but it may change, and the issue
named beside it says what it lacks. Issues about the whole chain rather than one feature (small
pieces, deep pockets, sculpts of millions of triangles) are listed apart, under
[Limits of the chain](#limits-of-the-chain): they bound every feature.

## The add-on: remeshing

The panel *openclare* of the Properties editor, on the tab of the mesh.

| Feature | State | What it lacks |
|---|---|---|
| *Remesh to Quads*, *Remesh Again*, *Stop*, *Select Original* | stable | |
| *Output: Quads* | stable | |
| *Output: Triangles*, for *Simulation* | experimental | a few triangles cut through another one on 3 models of 12 ([TRIANGLES.md](TRIANGLES.md), #226) |
| *Output: Triangles*, for *Shape* (Blender's *Decimate*) | stable | follows no hard edge, curve or material, as the panel says |
| *Output: N-gons* | experimental | flat parts as faces of many sides, creased all round; measured on six mechanical shapes and a half cylinder, three levels of subdivision; not with curves, poles or a symmetry for the parts that take no quads |
| *Ask For: Number* | stable | |
| *Ask For: Sizes* (*Largest*, *Smallest*) | experimental | the largest is the user's and the fingers are kept (#253); not measured on the bench yet (batch `analyse-sizes`) |
| *Analyse* | experimental | the same (#253) |
| *Show Sizes* | experimental | shows those sizes (#253) |
| *Density from Shape* | experimental | reads the shape with those sizes (#253) |
| *Adaptive Size* | stable | |
| *Quads per Turn* | experimental | with *Ask For: Sizes* only; a cylinder with flat caps gets about two thirds of the quads asked round it (all of them with *Output: N-gons*) |
| *Quads Around Thin Parts* | experimental | the bound on size has known limits (#38); parts thinner than a quad at low budgets (#18) |
| *Hard Edges*, *Angle* | stable | |
| *Marked Sharp* | stable | |
| *Shading* | stable | |
| *Follow Curvature* | experimental | default not tuned on the corpus (#13); noisy scans and terrains (#21, #33) |
| *Fewer Irregular Vertices* | experimental | a reach between 3.2 and 3.7 folds a quad on the male body (#72) |
| *Volume Fit* | experimental | the volume after subdivision is not kept; `--fitter limit-surface` keeps it, not yet in the add-on (#39) |
| *Density* (a vertex group) and *Factor* | stable | |
| *Symmetry* | stable | |
| *Materials*: each quad the material under it | stable | |
| *Materials*: a line of edges between two | experimental | mostly let go on a real model (#47) |
| *Sharp Edges*, *Creases* on the result | stable | |
| *Shape Presets*: *Default* and one's own | stable | |

## The add-on: curves and poles

The tools of the 3D viewport, in Object Mode.

| Feature | State | What it lacks |
|---|---|---|
| *Draw Curve*, with its snap (*Ridge*, *Valley*, radius on F) | experimental | curves drawn on a cube leave no edges (#195); the curves drawn are not counted in the sizes and the budget (#196); not kept everywhere (#44) |
| *Cut Volume* | experimental | makes the same curves (#195, #196, #44) |
| A curve's *Mesh*, *Sharp*, *Crease*, *Seam* | experimental | a mark is only as good as the line of edges it is on (#195) |
| *Select Curves*, *Deselect Curves* | stable | |
| The loops of the object, kept on its mesh and drawn on it and on its results; *Convert Curves to Loops*, *Clear*; sent to the engine with the curves | experimental | new on 27 November 2026, milestone 1 of [LOOP_EDITING.md](LOOP_EDITING.md): no tool edits them yet, and they are held no better than the curves (#195, #196, #44) |
| The poles of the last result, kept on the object and shown; those touched imposed on the next remesh | experimental | new on 31 October 2026, not yet measured on the bench nor tried by artists (#249) |
| The *Poles* tool: drag, add (3, 5), pin (P), remove (X), *Free All*, *Clear* | experimental | the same (#249) |

## The add-on: reflection check

The panel *Reflection Check* of the Properties editor, on the tab of the mesh, for an object under a Catmull-Clark *Subdivision Surface*, in Edit Mode.

| Feature | State | What it lacks |
|---|---|---|
| The faults of the cage, worst first, one angle each: n-gons off their plane, triangles and n-gons that pinch, poles on a curved zone, corner quads that fold or are thin; *Show the N Worst*, the tolerances, a row selects its face or vertex | experimental | new on 16 November 2026, not yet tried on a hand-made car body; the angles are measured on the cage, not on the subdivided surface (#328) |
| The overlay of the N worst in the 3D viewport, one colour per kind | experimental | the same; not drawn while the cage is being edited, until it has been still for 0.2 s; a cage of more than about 25 000 faces is checked on demand, with *Check* (#328); nothing is read while a move, a slide or another modal operator runs, only once it has ended; out of X-ray a fault is hidden by the cage alone (its faces hidden in Edit Mode left out), not by another object nor by what a modifier adds, the other half of a Mirror say |
| *Repair in Place*: vertices moved to flatten the n-gons and straighten the folded and thin corner quads, every vertex, edge and face kept; creases proposed on edges folding by *Sharp From* (30°) or more, set only with *Apply the Proposed Creases*; no vertex moved beyond *Bound* (10 % of its shortest edge); each repair kept only if the score gets better; the selected faces, or the whole cage; one undo step; the score before and after per kind, and what is left with its reason | experimental | new on 16 November 2026, not yet tried on a hand-made car body; "repaired" means by the check, whose angles are measured on the cage, not on the subdivided surface; an n-gon or a pole on a curved zone is listed, not repaired (#329) |

## Platforms

| Feature | State | What it lacks |
|---|---|---|
| The add-on on Linux | stable | |
| The add-on on Windows | stable | installed and run in a Blender for Windows on 31 October 2026; stopping a run with Esc not tried there |
| The add-on on macOS | experimental | built and tested as a library, never run in Blender; signed ad hoc, not notarized (#110) |

## The command line

`openclare remesh` and the other commands ([USAGE.md](../USAGE.md)).

| Feature | State | What it lacks |
|---|---|---|
| `remesh` with `--quads`, `--seed`, `--notes` | stable | |
| `--preset`, `--config`, `--save-config` | stable | |
| The presets shipped, `organic`, `animation`, `hard-surface` (`presets`) | experimental | their values are not tuned on the corpus (#13) |
| `--adaptivity`, `--crease-angle`, `--density`, `--density-contrast`, `--symmetry` | stable | |
| `--quads-a-turn` | experimental | as *Quads per Turn* |
| `--alignment` | experimental | as *Follow Curvature* (#13, #21, #33) |
| `--singularity-reach`, `--singularity-tolerance` | experimental | as *Fewer Irregular Vertices* (#72) |
| `--volume-fit` | experimental | as *Volume Fit* (#39) |
| `--fitter limit-surface` | experimental | off by default, on the command line only; creases not taken into account (#39) |
| `--quads-around` | experimental | as *Quads Around Thin Parts* (#38, #18) |
| `--largest`, `--smallest`, `--most-quads`, `analyse` | experimental | as *Ask For: Sizes* (#253) |
| `--edge-length` | experimental | older than sizes, not on the bench |
| `--groups`: the group of each quad | stable | |
| `--groups`: a line of edges between two | experimental | as *Materials* (#47) |
| `--curves` | experimental | as *Draw Curve* (#195, #196, #44) |
| `--poles`, `--poles-out` | experimental | as the poles of the add-on (#249) |
| `--faces ngons`, `--creases-out` | experimental | as *Output: N-gons* |
| `--edge-angle` | experimental | sharp edges of the work mesh, off by default, not yet the add-on's |
| `--method triangles` | experimental | as *Output: Triangles* (#226) |
| `--method integer-grid`, `--method instant-meshes` | experimental | other chains, kept for comparison (#9) |
| `--quality slow`, `--slow-work-limit`, `--slow-time-limit` | experimental | a research track, on the command line only (#138, #144) |
| `--skip-pieces-under`, `--no-close-loops` | experimental | tools to look into a run |
| `--merge-singularities` | experimental | a 3 and a 5 merged across a strip of no width, off by default (#171) |
| `measure`, `layout`, `primitive` | stable | |
| `repair`: the reflection repair of a cage in place, `--bound`, `--crease-fold`, `--apply-creases`, `--creases`, `--creases-out` | experimental | as *Repair in Place* (#329) |
| `measure`: the reflection check, `--creases` | experimental | as *Reflection Check*; an OBJ carries no crease: without `--creases`, only borders are sharp (#328) |
| `field`, `parametrize`, `tmesh`, `quantize`, `preprocess` | experimental | stages of the chain one at a time: their options follow the chain |
| `--record`, `--record-still`, `openclare-view` (Rerun) | experimental | a version of Rerun behind, stages not shown (#35) |

## Libraries

| Feature | State | What it lacks |
|---|---|---|
| The C library, `openclare-capi`, in each release | stable | `quality = slow` is the standard quality through it |
| The Rust library, `openclare` | experimental | version 0: its interface changes from one version to the next |

## Limits of the chain

What bounds every feature, and is not one feature's to fix. Severity *high* first.

- Small separate pieces: a few still go through the fallback chain, or do not come out of it
  (#34).
- Deep pockets and cavities are closed over by the mesh (#36).
- Sculpts of millions of triangles: most of the time goes to preprocessing (#45).
- The work mesh has triangles of one size: what is finer is lost first (#48).
- Triangles of nearly no area left by preprocessing (#31).
- Narrow strips of quads where arcs are kept open (#5), and a base complex doubled by
  T-junctions (#20).
- Inversions: two of the three lines of defence are not written (#10).
- No bound on computing time for inputs made to break it (#27).
