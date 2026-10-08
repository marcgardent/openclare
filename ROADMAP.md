# openclare — roadmap

What matters among what is left to do, in the order proposed. Written on 8 October 2026.
The full list of shortcuts and known limits is in [BACKLOG.md](BACKLOG.md) (tickets `DT-nnn`);
the specification and what each finished milestone delivered are in
[ALGORITHME.md](ALGORITHME.md).

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
| 1 | **Volume fitting** (under way) | With its vertices on the surface a mesh is inscribed in the shape: the ogre lost 3.5 % of its volume at 1000 quads | The bench confirms what the side measurements show (99.8 to 100 % of the volume kept) with no model losing its validity | DT-037 |
| 2 | **Alignment** | The largest gap left with Quad Remesher: edges are 7.7 to 8.4 degrees off the curvature directions, against 6.4 to 7.5. It is what an artist sees first in an edge flow | Median alignment at or under Quad Remesher's at the three budgets, validity untouched | DT-021, DT-025, DT-013, DT-033 |
| 3 | **Fewer irregular vertices, longer edge loops** | The weakest marks of the chain (structure about 40 out of 100; regularity 50 at 1000 quads), and thin parts made it worse: 155 irregular vertices on the ogre at 3000 quads for 88 before | Irregular vertices back to what they were before the thin-part work, with the fingers kept; fewer patches in the base complex | DT-020, DT-007, DT-005, DT-038 |
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
models. It comes after the quality tracks by the rule above; it could come earlier if trying it
on real work matters more than the last points of alignment.

| Order | Track | Done when | Tickets |
|---|---|---|---|
| 8 | **C library and Blender add-on** (milestone 10): remesh from Blender without copying memory, density read from Weight Paint, hard edges from the marked ones | A sculpt remeshed from Blender's interface with the presets and a painted density | J10, DT-026 |
| 9 | **Rename** the crates and the executable from `retopo` to `openclare`, with the add-on | Nothing is called `retopo` any more | DT-015 |
| 10 | **Symmetry**: a symmetric model gives a symmetric mesh, with an edge loop on the plane | Mesh and mirror image coincide on the symmetric models of the bench | J9 bis, DT-028 |
| 11 | **Guide curves**: edge loops where the artist draws them, in a way that stays simple in Blender (face sets or annotation strokes rather than marked seams) | An edge loop follows a stroke drawn on the ogre | DT-024 |

## 4. Beyond the quad mesh

Later, and each to be confirmed when its turn comes: seams and UVs (with milestone 10),
automatic creases for subdivision (milestone 12), optimal normals per corner (milestone 13),
Multires levels with the detail of the sculpt reprojected (milestone 14).

## 5. The project itself

| What | Why now or later | Tickets |
|---|---|---|
| **Sculpts of the author in the bench, with their Quad Remesher references** | The references can only be made while the trial licence of Quad Remesher lasts: this one has a deadline | DT-017 |
| **Documents in English** | New text is in English since 8 October 2026; `ALGORITHME.md`, `BACKLOG.md` and the `README.md` files still hold French | DT-016 |
| **Public release**: continuous integration, contribution guide, a tagged version | Before the add-on is announced, not before it exists | — |
| **Rerun**: patches as coloured regions, the seamless map, comparison of two runs; move to the current version | As the quality tracks need them, the map first | DT-035 |

## What is not planned

- The patch chain for hard surfaces (QuadWild): walls in the T-mesh do the job (DT-030).
- An exact integer-grid map and extraction: the direct route from the T-mesh replaced it
  (DT-001).
- Embedding the Rerun viewer in what is distributed: it stays a tool for those who work on the
  chain.
