# openclare — roadmap

What matters among what is left to do. Written on 8 October 2026 and reordered the same day:
the author chose to start the Blender add-on now (section 3), with its UV layers, and to keep
the quality tracks for after its first version.
The full list of shortcuts and known limits is in [BACKLOG.md](BACKLOG.md) (tickets `DT-nnn`);
the specification and what each finished milestone delivered are in
[ALGORITHM.md](ALGORITHM.md).

## Where the project stands

The chain runs end to end: triangles in, quads out, with sizing by the shape, painted density,
hard edges, separate pieces, thin parts and (being measured) volume fitting.

Average mark of the organic models, out of 100 ([SCORES.md](bench/results/SCORES.md), before
volume fitting):

| Quads asked for | openclare | Quad Remesher | QuadriFlow (Blender) |
|---|---|---|---|
| 1000 | 70 | 64 | 40 |
| 3000 | 76 | 63 | 46 |
| 8000 | 81 | 72 | 50 |

The goal "better than Blender's quad remesher" is met on the bench. Quad Remesher is ahead on
three points, which are what the quality tracks below are about: **alignment** of the edges on
the shape, **regularity** at low budgets, and **fidelity** when it is allowed twice the quads.
And nobody can use openclare yet without a command line.

The rule since 8 October 2026: what improves the mesh comes before side features.

## 1. Quality: close the gaps that are measured

Each track ends with the bench run at 1000, 3000 and 8000 quads and a stop for review.

| Order | Track | Why it matters | Done when | Tickets |
|---|---|---|---|---|
| 1 | **Volume fitting** (done, awaiting review) | With its vertices on the surface a mesh is inscribed in the shape: the ogre lost 3.5 % of its volume at 1000 quads | Done: median volume kept 100.0 % at the three budgets, the ogre 99.8 % at 1000 quads, no model losing its validity. Left: the volume after subdivision | DT-037 (closed), DT-039 |
| 2 | **Alignment** | The largest gap left with Quad Remesher: edges are 7.7 to 8.4 degrees off the curvature directions, against 6.4 to 7.5. It is what an artist sees first in an edge flow | Median alignment at or under Quad Remesher's at the three budgets, validity untouched | DT-021, DT-025, DT-013, DT-033 |
| 3 | **Fewer irregular vertices, longer edge loops** | The weakest marks of the chain (structure about 40 out of 100; regularity 50 at 1000 quads), and thin parts made it worse: 155 irregular vertices on the ogre at 3000 quads for 88 before | Irregular vertices back to what they were before the thin-part work, with the fingers kept; fewer patches in the base complex | DT-020, DT-007, DT-005, DT-038 |
| 3 bis | **Improving a mesh from a dry run**: mark a first run, keep the best of several, run again where one went wrong, and weigh alignment where it is poor | Density driven by the error was tried and gains little (10 to 15 % on the worst distances); the other uses are not tried | A run that gives an invalid mesh is caught and corrected without the user; the best of three runs beats one on the marks | DT-040 |
| 4 | **What a mesh still loses of the shape** | Ears and thin sheets stay rough at their rim; deep pockets are closed over (the Hausdorff distance of the ogre comes from two pockets behind its eyes, not from its hands) | Thin-part distance of `jerry` under 10 % of a quad edge; Hausdorff of the ogre at 3000 quads under 10 per mille | DT-018, DT-036 |

## 2. Robustness: inputs that are not clean

Three models of the bench are still not meshed validly: Desert Rose (shells that cross), and
the two game characters (hundreds of small pieces, non-manifold edges). Real models look like
them more often than like a clean sculpt.

| Order | Track | Done when | Tickets |
|---|---|---|---|
| 5 | **Repair before meshing**: merge the pieces that cross, mend non-manifold edges (milestone 11) | Desert Rose meshed as one valid skin at the three budgets; `marine` accepted | J11, DT-031 |
| 6 | **Small pieces**: no piece falls back to the provisional chain or comes out empty | `mahaveer` valid, every piece through the main chain | DT-034, DT-009, DT-010 |
| 7 | **Guards**: a bound on computing time whatever the input | No input of the bench, or made to break it, runs longer than a stated limit | DT-027 |

## 3. Use: openclare inside Blender

This is what turns the engine into a tool, and what will bring feedback from other people's
models. **Next up**, by decision of 8 October 2026, in two steps: first the least that runs
(rename, C library called from Python with `ctypes`, a panel with a preset, a number of quads
and a button, the result as a new object); then density from Weight Paint, hard edges from the
marked ones, and two UV layers: the map the engine uses as it is (every quad a unit square,
islands that overlap) and an atlas where each rectangular patch is an island of its own.

| Order | Track | Done when | Tickets |
|---|---|---|---|
| 8 | **C library and Blender add-on** (milestone 10): remesh from Blender without copying memory, density read from Weight Paint, hard edges from the marked ones, UV layers | Done, awaiting review: `make addon`; preset, quads, adaptive size, density from a vertex group; each stage announced; export of two UV layers and of sharp edges. Left: reading the edges marked sharp, stopping a run, Windows and macOS | J10, DT-041, DT-042 |
| 9 | **Rename** the crates and the executable from `retopo` to `openclare` | Done on 8 October 2026 | DT-015 (closed) |
| 10 | **Symmetry**: a symmetric model gives a symmetric mesh, with an edge loop on the plane | Mesh and mirror image coincide on the symmetric models of the bench | J9 bis, DT-028 |
| 11 | **Guide curves**: edge loops where the artist draws them, as curves held on the model by a Shrinkwrap | First version done on 9 October 2026, awaiting review: exact on simple shapes, kept on part of the curves tried on the ogre and let go on the others. Done when a curve drawn where a mesh of quads can follow it is kept, every time | DT-024 (closed), DT-044 |

## 4. Beyond the quad mesh

Done ahead of their turn, at the author's request: two UV layers and their seams (milestone
10, DT-042) and crease weights for subdivision (milestone 12, DT-043).

Later, and each to be confirmed when its turn comes: optimal normals per corner (milestone
13), Multires levels with the detail of the sculpt reprojected (milestone 14).

## 5. The project itself

| What | Why now or later | Tickets |
|---|---|---|
| **Sculpts of the author in the bench, with their Quad Remesher references** | The references can only be made while the trial licence of Quad Remesher lasts: this one has a deadline | DT-017 |
| **Documents in English** | **Done on 9 October 2026**: every document is in English | DT-016 |
| **Public release**: continuous integration, contribution guide, a tagged version | Before the add-on is announced, not before it exists | — |
| **Rerun**: patches as coloured regions, the seamless map, comparison of two runs; move to the current version | As the quality tracks need them, the map first | DT-035 |

## What is not planned

- The patch chain for hard surfaces (QuadWild): walls in the T-mesh do the job (DT-030).
- An exact integer-grid map and extraction: the direct route from the T-mesh replaced it
  (DT-001).
- Embedding the Rerun viewer in what is distributed: it stays a tool for those who work on the
  chain.
