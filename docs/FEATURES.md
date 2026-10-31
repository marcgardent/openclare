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
| *Ask For: Number* | stable | |
| *Ask For: Sizes* (*Largest*, *Smallest*) | experimental | sizes proposed from a hidden constant, fingers dropped on a body (#253) |
| *Analyse* | experimental | the same (#253) |
| *Show Sizes* | experimental | shows those sizes (#253) |
| *Density from Shape* | experimental | reads the shape with those sizes (#253) |
| *Adaptive Size* | stable | |
| *Quads Around Thin Parts* | experimental | the bound on size has known limits (#38); parts thinner than a quad at low budgets (#18) |
| *Hard Edges*, *Angle* | stable | |
| *Marked Sharp* | stable | |
| *Shading* | stable | |
| *Follow Curvature* | experimental | default not tuned on the corpus (#13); noisy scans and terrains (#21, #33) |
| *Fewer Irregular Vertices* | experimental | a reach between 3.2 and 3.7 folds a quad on the male body (#72) |
| *Volume Fit* | experimental | the volume after subdivision is not kept (#39) |
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
| The poles of the last result, kept on the object and shown; those touched imposed on the next remesh | experimental | new on 31 October 2026, not yet measured on the bench nor tried by artists (#249) |
| The *Poles* tool: drag, add (3, 5), pin (P), remove (X), *Free All*, *Clear* | experimental | the same (#249) |

## Platforms

| Feature | State | What it lacks |
|---|---|---|
| The add-on on Linux | stable | |
| The add-on on Windows and macOS | experimental | built and tested as a library, never run in Blender (#110) |

## The command line

`openclare remesh` and the other commands ([USAGE.md](../USAGE.md)).

| Feature | State | What it lacks |
|---|---|---|
| `remesh` with `--quads`, `--seed`, `--notes` | stable | |
| `--preset`, `--config`, `--save-config` | stable | |
| The presets shipped, `organic`, `animation`, `hard-surface` (`presets`) | experimental | their values are not tuned on the corpus (#13) |
| `--adaptivity`, `--crease-angle`, `--density`, `--density-contrast`, `--symmetry` | stable | |
| `--alignment` | experimental | as *Follow Curvature* (#13, #21, #33) |
| `--singularity-reach`, `--singularity-tolerance` | experimental | as *Fewer Irregular Vertices* (#72) |
| `--volume-fit` | experimental | as *Volume Fit* (#39) |
| `--quads-around` | experimental | as *Quads Around Thin Parts* (#38, #18) |
| `--largest`, `--smallest`, `--most-quads`, `analyse` | experimental | as *Ask For: Sizes* (#253) |
| `--edge-length` | experimental | older than sizes, not on the bench |
| `--groups`: the group of each quad | stable | |
| `--groups`: a line of edges between two | experimental | as *Materials* (#47) |
| `--curves` | experimental | as *Draw Curve* (#195, #196, #44) |
| `--poles`, `--poles-out` | experimental | as the poles of the add-on (#249) |
| `--edge-angle` | experimental | sharp edges of the work mesh, off by default, not yet the add-on's |
| `--method triangles` | experimental | as *Output: Triangles* (#226) |
| `--method integer-grid`, `--method instant-meshes` | experimental | other chains, kept for comparison (#9) |
| `--quality slow`, `--slow-work-limit`, `--slow-time-limit` | experimental | a research track, on the command line only (#138, #144) |
| `--skip-pieces-under`, `--no-close-loops` | experimental | tools to look into a run |
| `measure`, `layout`, `primitive` | stable | |
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
