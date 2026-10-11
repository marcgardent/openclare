# openclare — backlog of technical debt

One ticket per shortcut, approximation, fallback or item of the specification
([ALGORITHM.md](ALGORITHM.md)) that was not done. Nothing is deleted: a ticket that is settled
becomes *closed*, with the milestone that settled it. Numbers are never used again.

This file does not hold the roadmap (the milestones are in ALGORITHM.md, section 4; what is left
to do today is in [ROADMAP.md](ROADMAP.md)), only what was left behind on the way.

**Severity**: *high* = limits a result the acceptance rule is about; *medium* = limits quality
or robustness outside the corpus; *low* = tidiness, comfort, theoretical guarantee.

**Labels**: J*n* is milestone *n* (from the French *jalon*); Q*n* a quality track; T*n* a tools
track. "DT" stands for *dette technique*, technical debt.

## Overview

| Ticket | Subject | Born at | Severity | State |
|---|---|---|---|---|
| DT-001 | Constrained integer-grid map and exact extraction, set aside | J7 | medium | open |
| DT-002 | Hard edges missing from the T-mesh | J5 | high (hard surface) | **closed at J9** |
| DT-003 | A piece with neither singularity nor border: no T-mesh | J5 | medium | **closed (Q1)** |
| DT-004 | Approximate quantization solver | J6 | low | open |
| DT-005 | Arcs kept open: narrow strips of quads | J6 | medium | open |
| DT-006 | Quad budget: no finer than a strip | J6 | low | open |
| DT-007 | Approximate race of the T-mesh, T-mesh not simplified | J5 | low | open |
| DT-008 | Tracing in floating point, with no formal guarantee | J5 | low | open |
| DT-009 | The provisional `integer-grid` chain still in the repository | J4 bis | low | open |
| DT-010 | Lines of defence 2 and 3 against inversions not written | J4 | medium | open |
| DT-011 | Very stretched faces in the seamless map | J4 | low | open |
| DT-012 | The `hard-surface` preset on the organic chain | J4 bis | high (hard surface) | **closed at J9** |
| DT-013 | Preset values not tuned on the corpus | J4 bis | medium | open |
| DT-014 | Visualization of fields and structures | J2 | low | **closed (T1, T2)**, DT-035 remains |
| DT-015 | Code still named `retopo` | J4 bis | low | **closed at J10** |
| DT-016 | Documents in French, to translate | J0 | low | **closed (9 October 2026)** |
| DT-017 | Corpus: the author's sculpts to add before the trial licence ends | J0 | medium | open |
| DT-018 | Parts thinner than a quad poorly rendered at low budgets | J7 | high | open (fingers dealt with at Q2; thin sheets remain) |
| DT-019 | Rudimentary relaxation: the shape of the quads levels off | J7 | high | **closed at J8** |
| DT-020 | Base complex doubled by T-junctions | J7 | medium | open |
| DT-021 | Noisy scan (rock): the field follows the noise | J7 | medium | open |
| DT-022 | Comparison references incomplete at 1000 and 8000 quads | J7 | low | **closed at J8** |
| DT-023 | Relaxation: cost, and sizes of quads | J8 | low | open |
| DT-024 | Guide curves (rank 4) not started | J8 | medium | **closed (9 October 2026)**, DT-044 remains |
| DT-025 | Density: reach of singularities and reading of curvature still at the nominal scale | J8 | low | open (local reach done at Q2; curvature remains) |
| DT-026 | Painted density: reading Blender's Weight Paint to do (Blender bridge) | J8 | medium | **closed at J10** |
| DT-028 | Symmetry: missing, Quad Remesher has it | J8 | low | open, after quality |
| DT-029 | Stretched quads on the wall of a cylinder | J9 | medium | open |
| DT-030 | Patch chain (QuadWild) not written | J9 | low | open |
| DT-031 | Triangles of nearly no area left by preprocessing | J9 | medium | open |
| DT-032 | Models made of many separate pieces: the whole chain falls back | J9 | high | **closed (Q1)**, DT-034 remains |
| DT-034 | Small pieces: a few still go through the fallback, or do not come out of it | Q1 | medium | open |
| DT-035 | Rerun visualization: what is not shown, version behind | T2 | low | open |
| DT-036 | Deep pockets and cavities closed over by the mesh | Q2 | medium | open |
| DT-037 | Volume: the mesh is inscribed in the shape | Q2 | high | **closed (Q2 bis)**, DT-039 remains |
| DT-038 | Thin parts: limits of the bound on size | Q2 | medium | open |
| DT-039 | Fitting: limits, and the volume after subdivision | Q2 bis | medium | open |
| DT-040 | Improving a mesh from a dry run: tried for density, the rest to do | idea of the author | medium | open |
| DT-041 | Blender add-on, first version: what it does not do | J10 | medium | open |
| DT-042 | UV layers unfolded from the quad mesh: limits | J10 | medium | open |
| DT-043 | Creases: limits | J12 | low | open |
| DT-044 | Guide curves: not kept everywhere | guide curves | high | open |
| DT-045 | Sculpts of millions of triangles: most of the time goes to preprocessing | guide curves | medium | open |
| DT-046 | Add-on: a button to write the density worked out from curvature as weights on the source | idea of the author | medium | open |
| DT-047 | Borders between materials: mostly let go on a real model | materials | high | open |
| DT-048 | The work mesh has triangles of one size: what is finer is lost before anything else runs | materials | high | open |
| DT-049 | The bench has no model with curves or a painted density | materials | medium | open |
| DT-050 | A sliver with next to no area took all the memory of the machine | bench in Blender | high | **closed (9 October 2026)**; no guard on memory in the engine itself |
| DT-051 | Baking the sculpt onto the result: normal map, height and ambient occlusion, read through the engine's own correspondence | idea of the author | medium | open, after the lines (DT-047, DT-048) |
| DT-052 | Preprocessing welded back what the mending took apart: the body of a model not meshed, and the result passed for valid | bench | high | **closed (11 October 2026)**; a result that lacks most of its model still passes for valid |
| DT-033 | Terrains and scans: no tool aligns, the field has nothing to follow | J9 | medium | open |
| DT-027 | Guards on computing time for pathological inputs | J8 | low | open |

## Tickets

### DT-001 — Constrained integer-grid map and exact extraction, set aside

- **Born at** milestone 7, by decision of 7 October 2026 (the "direct" route).
- **What was planned**: M7 (solve a (u, v) map with whole transitions, constrained by the
  quantized lengths, with no inverted triangle) then M8 (exact extraction, on the principle of
  QEx).
- **What is done instead**: the connectivity of the mesh comes straight from the quantization;
  each vertex is placed by reading the seamless map in its rectangle, then relaxed.
- **What is lost**: the integer map spreads the distortion over the whole surface; the direct
  route stretches each rectangle on its own, which leaves breaks at the borders between
  rectangles that only relaxation softens. No global UVs consistent with the mesh (useful for
  J10).
- **To take up again if** the quality measured at milestone 7 (metrics A and F) stays behind
  Quad Remesher because of those breaks, or for the export of UVs (J10) at the latest.

### DT-002 — Hard edges missing from the T-mesh

- **Born at** milestone 5.
- **Finding**: hard edges are kept on isolines of the map, but they are not cuts of the T-mesh.
  So nothing makes a hard edge carry an arc, and quantization cannot give it a whole
  coordinate. On the cube the traces follow them by themselves; that is not guaranteed
  elsewhere.
- **Consequence**: on a mechanical part, edges of the mesh may not fall on the hard edge; only
  relaxation brings them back to it.
- **To do**: treat hard edges like open borders (cuts that are there from the start, corners
  that send out traces). At milestone 9 at the latest.
- **Closed at milestone 9**: borders and hard edges are "walls" of the T-mesh. The mesh has an
  edge along every hard edge and a vertex on every corner, checked to 1e-6 on an L-shaped
  block; a cylinder keeps its two circular rims.

### DT-003 — A piece with neither singularity nor border: no T-mesh

- **Born at** milestone 5.
- **Finding**: a torus whose field has no singularity has no trace, hence neither T-mesh nor
  quantization.
- **Fallback at the time** (milestone 7): as soon as one piece of the surface has neither
  singularity nor border, the whole `quad-layout` chain hands over to `integer-grid` (DT-009),
  and the report says so. A model in several pieces of which one is a torus thus loses all the
  benefit.
- **To do**: quantize the two periods apart, or lay an arbitrary cut; treat each piece on its
  own.
- **Closed at track Q1**: a piece with neither singularity nor wall is cut from one of its
  vertices, any, from which four traces set off. The torus comes out as one regular grid from
  the main chain, with no fallback. This case, which I took for marginal, is in fact common:
  on a game character, 92 pieces out of 164 were rings.

### DT-004 — Approximate quantization solver

- **Born at** milestone 6.
- **What was planned**: Bi-MDF with exact refinement by matching.
- **What is done**: local search by improving walks. On small networks it finds the exact
  optimum in 58 cases out of 60.
- **To take up again if** visibly sub-optimal quantizations show up.

### DT-005 — Arcs kept open: narrow strips of quads

- **Born at** milestone 6. Decision of 7 October 2026: singularities are kept apart.
- **Finding**: so as not to merge two singularities, some arcs get a minimum length of 1 when
  their real length is far less (29 arcs over the 12 organic models at 3000 quads). The result
  is strips one quad wide, too narrow.
- **Lead**: allow two close opposite singularities to merge (Lyon et al. 2021), as an option a
  preset can set.
- **Seen at milestone 7**: these strips give squashed quads that two passes of relaxation do
  not quite open (a few non-convex quads on jerry). The same strips along borders were removed
  by letting a singularity become a border corner and two neighbouring border corners merge;
  between singularities the rule stays.

### DT-006 — Quad budget: no finer than a strip

- **Born at** milestone 6.
- **Finding**: the number of quads is set by one common scale factor; the smallest possible
  change is a whole strip, that is 2 to 4 % at 1000 quads. Median gap 1.4 %, worst case 3.9 %.
- **Lead**: a budget term in the objective (Marinov et al. 2026).

### DT-007 — Approximate race of the T-mesh, T-mesh not simplified

- **Born at** milestone 5.
- **Finding**: traces advance triangle by triangle, the least advanced first; the order of the
  meetings may differ from the ideal race by a triangle. No trace is shortened or extended to
  simplify the partition.
- **Consequence**: a valid partition but not the simplest; a quarter of the rectangles are
  slivers that quantization closes.
- **Lead**: choosing which traces to extend (Lyon et al. 2021).

### DT-008 — Tracing in floating point, with no formal guarantee

- **Born at** milestone 5.
- **What was considered**: exact or rational predicates.
- **What is done**: floating point and a single tolerance of 1e-6. No failure on the corpus (48
  cases). A lost trace makes the partition invalid, and it is reported.
- **To take up again if** a model outside the corpus makes tracing fail.

### DT-009 — The provisional `integer-grid` chain still in the repository

- **Born at** milestone 4 bis. The specification planned to remove greedy rounding at
  milestone 7.
- **Finding**: it still serves as a fallback (DT-003) and as a point of comparison. Since
  milestone 7 it is no longer the chain of the presets.
- **To do**: remove it, with its extraction by cells, when the new chain replaces it
  everywhere.

### DT-010 — Lines of defence 2 and 3 against inversions not written

- **Born at** milestone 4.
- **What was planned**: the validity constraints of *Integer-Grid Maps*, then a construction
  with a guarantee (Campen et al. 2019) as a last resort.
- **What is done**: stiffening alone. No inverted triangle on the corpus.
- **Consequence**: outside the corpus, an inversion that cannot be undone would make the T-mesh
  invalid with no fallback.

### DT-011 — Very stretched faces in the seamless map

- **Born at** milestone 4.
- **Finding**: near some singularities, triangles are valid but squashed (stretch up to 1000).
- **Consequence**: the map is read with little precision there; to watch in the placement of
  vertices (J7).

### DT-012 — The `hard-surface` preset on the organic chain

- **Born at** milestone 4 bis.
- **Finding**: the preset exists, but it goes through the organic chain, for want of a chain
  for hard surfaces. See also DT-002.
- **Closed at milestone 9**: the one chain now guarantees hard edges; the preset no longer
  needs another chain. The patch chain planned at first is followed apart (DT-030).

### DT-013 — Preset values not tuned on the corpus

- **Born at** milestone 4 bis.
- **Finding**: `organic`, `animation` and `hard-surface` have reasonable values, not measured
  ones.
- **To do**: a tuning campaign once the whole chain is stable.

### DT-014 — Visualization of fields and structures

- **Born at** milestone 2. Put off by decision ("we shall see later").
- **Finding**: fields, traces and patches can only be looked at through OBJ files of lines and
  renders made by hand.
- **Track T1 (8 October 2026)**: the final meshes now have real renders (Blender,
  `scripts/render.py`) and a page per model (`make pages`). The stages in between (field,
  traces, patches) still have no visualization.
- **Track T2 (8 October 2026)**: the stages in between can be looked at in Rerun
  (`make view MODEL=ogre`). **Closed**; what is still missing is followed by DT-035.

### DT-015 — Code still named `retopo`

- **Born at** milestone 4 bis. Decision of 7 October 2026: renaming at milestone 10.
- **Finding**: crates, executable and scripts of the bench carry the old name.
- **Closed at milestone 10 (8 October 2026)**: crates, executable, scripts and documents are
  named `openclare`. The folder of the repository on the author's machine keeps its name.

### DT-016 — Documents in French, to translate

- **Born at** milestone 0, by decision ("for the docs we shall translate at the end").
- **Finding**: `ALGORITHME.md`, `BACKLOG.md`, the `README.md` files and `docs/` are in French;
  the code and its comments are in English.
- **8 October 2026**: the author asks for documents to be written in English from now on, so
  that there is less to translate at the end. `CREDITS.md` is translated.
- **Closed on 9 October 2026**, at the author's request: every document is in English.
  `ALGORITHME.md` is now `ALGORITHM.md`, and the pictures named after French words are
  renamed. The labels J*n* (for *jalon*, milestone) and DT (for *dette technique*) are kept:
  tickets are never renumbered. The models generated by the bench (`docs/models`,
  `bench/results`) were in English already.

### DT-017 — Corpus: the author's sculpts to add

- **Born at** milestone 0. The answer at the time: "yes, later".
- **Finding**: the Quad Remesher references can only be made while the trial licence is valid.
  Any model added after it expires will have no reference.
- **To do**: provide the sculpts, then run `bench/make_references.py`.

### DT-018 — Parts thinner than a quad poorly rendered at low budgets

- **Born at** milestone 7.
- **Finding**: when a part of the shape is thinner than the quad asked for (the ears of the
  pig at 1000 quads), the vertices of its two sides end up at nearly the same place. Two faces
  of the pig are counted inverted at 1000 quads: it is the only invalid case of the 36
  measured. The ogre keeps at 3000 quads a Hausdorff distance of 3 % of the diagonal
  (QuadriFlow: 3.7 %), probably for the same reason; to check.
- **Lead**: adaptive density (rank 2 of the constraints, J8); failing that, detection of thin
  parts and placement of the vertices on their median line.
- **Remark of the author, 8 October 2026**: the fingers and ears of the ogre are a real problem
  for every remesher. The requirement to hold: keep at least the volume of those parts. It
  became quality track Q2.
- **At track Q2 (8 October 2026)**: a measure of thickness now bounds the size of the quads so
  that six of them go round a thin part (`quads_around`), and the singularities of a finger
  are no longer cancelled. On the ogre at 3000 quads the distance over the thin parts goes
  from 30 to 9 % of a quad edge; the hands have fingers. **Still open**: thin sheets (ears,
  cloth), which the measure of girth leaves aside on purpose and whose rim stays rough; and
  details smaller than a quad (the tail of `spot` at 1000 quads is rendered less well than
  before).
- **At milestone 8**: with the adaptive density of the preset, the pig is valid at 1000 quads.
  But the ears of the ogre stay its largest distance to the source: curvature alone does not
  see that a part is thin. A measure of local thickness in the sizing field would be needed.

### DT-019 — Rudimentary relaxation: the shape of the quads levels off

- **Born at** milestone 7.
- **Finding**: relaxation is Laplacian smoothing followed by a projection. Two passes improve
  the shape; more wears fine details away (the Hausdorff distance of the ogre goes from 27 to
  46 per mille between 0 and 25 passes) and ends up inverting a face. Metric F stays behind
  QuadriFlow on rock, creature and spot, and behind Quad Remesher everywhere.
- **Consequence**: it was the first brake on perceived quality.
- **Closed at milestone 8**: a new relaxation, `squaring` (each quad drawn towards its
  rectangle, tangential moves, a vertex held within half an edge of where it started, moves
  that turn a quad over refused). At 3000 quads the mean angle deviation goes from 7.8 to 5.4
  degrees (Quad Remesher: 7.2) and the Hausdorff distance from 9.1 to 8.1 per mille. The F
  threshold is met on the 12 models. The old smoothing stays available (`laplacian`) for the
  two other chains.

### DT-020 — Base complex doubled by T-junctions

- **Born at** milestone 7.
- **Finding**: 1886 patches in the median against 951 for the greedy rounding of milestone 4
  bis. Two neighbouring rectangles of the T-mesh do not necessarily have their lines facing
  each other; so the separatrices of the final mesh do not join up. The C threshold stays met
  against QuadriFlow (except on rock), not the aim (half of QuadriFlow).
- **Lead**: simplifying the T-mesh (DT-007), or a term aligning the junctions in the
  quantization.

### DT-021 — Noisy scan (rock): the field follows the noise

- **Born at** milestone 7 (visible since milestone 2).
- **Finding**: on the raw scan, 50 irregular vertices against 8 for QuadriFlow, alignment at
  16 degrees, quads of a poorer shape. The field aligns on a curvature that is partly noise.
- **Lead**: read curvature at a wider scale on noisy surfaces, or lower the alignment where
  confidence is weak; tuning of the presets (DT-013).

### DT-022 — Comparison references incomplete at 1000 and 8000 quads

- **Born at** milestone 7.
- **Finding**: QuadriFlow was run on the whole corpus at 3000 quads only; at 8000 there is one
  model, at 1000 none. So the acceptance rule is only established at 3000 quads.
- **Closed at milestone 8**: every tool was run on the whole corpus at 1000 and 8000 quads
  (the Quad Remesher references existed already). The acceptance rule is met in the median at
  the three budgets; the detail per model is in `bench/results/ACCEPTANCE.md`. A reservation:
  these two campaigns ran while I was compiling and testing, so their computing times are a
  little overestimated.

### DT-023 — Relaxation: cost, and sizes of quads

- **Born at** milestone 8.
- **Finding**: sixty passes, each with a projection onto the source per vertex: 1.7 s on the
  ogre at 8000 quads, a sixth of the total time. Besides, the quads keep the sizes that
  quantization gives them; the variation of area goes up a little (0.34 to 0.38 on blob).
- **Seen too**: at 8000 quads the sixty passes are no longer quite enough (mean angle deviation
  of 6.1 degrees against 5.4 at 3000 quads), and openclare is slower there than QuadriFlow
  (6.8 s against 4.1 s in the median).
- **Leads**: a number of passes that depends on the size of the mesh, or coarse to fine;
  projecting only one pass in several; stopping when nothing moves any more; projecting onto
  the work mesh rather than the source; a larger share of smoothing where two neighbouring
  rectangles have different densities.

### DT-024 — Guide curves (rank 4) not started

- **Born at** milestone 8 under the title "Adaptive density and constraints of rank 3 to 5 not
  started"; **reduced** in the same milestone.
- **Done since**: adaptive density (rank 2) and painted density (rank 5), by one sizing field
  (sheet M1 bis). Quad Remesher by default keeps an edge on fidelity at 1000 and 3000 quads,
  but it goes over the budget asked for by 71 % and 37 % there.
- **Left**: rank 4, guide curves drawn on the surface. Rank 3 (hard edges) is followed by
  DT-002 and DT-012.
- **Lead**: a guide curve is a feature edge the user draws; it would come in through `Guides`,
  like painted density, and be treated like a hard edge.
- **Finding of 7 October 2026**: Quad Remesher (Blender add-on 1.4, checked) has no guide
  curves. It guides edge loops by what the mesh already carries: borders between materials,
  split normals, hard edges detected by angle. A far cheaper variant of rank 4 would then be
  **edges marked by the user** in Blender, which would go through the same mechanism as the
  hard edges of milestone 9.
- **Objection of 7 October 2026**: marking edges is close to impossible on a mesh of high
  density, and free curves may make for a heavy interface. (Since milestone 9 the mechanism
  exists on the engine side: a line to follow is a "wall". What is left is to bring in through
  `Guides` walls named by the user.) So the question is first that of the gesture in Blender.
  Leads to discuss, from the most natural on a sculpt to the heaviest: the borders between
  *Face Sets* of Sculpt mode (zones are painted, their borders become loops); an annotation
  stroke drawn on the surface; curve objects. Nothing is decided.
- **Closed on 9 October 2026**: curve objects it is, drawn in Blender and held on the model by
  a Shrinkwrap, as the author proposed (sheet "Guide curves" of the specification). They come
  in through `Guides` and are treated like hard edges. What is not reliable yet is DT-044.

### DT-025 — Density: reach of singularities and reading of curvature still at the nominal scale

- **Born at** milestone 8.
- **Finding**: where quads are smaller, two settings are still counted in nominal edges: the
  reach of the cancellation of pairs of singularities (M3), which thus cancels too much in
  dense zones and not enough in loose ones, and the scale at which the cross field reads
  curvature (M2).
- **Consequence**: in a zone painted very dense, details the quads could follow are not seen
  by the field.
- **To do**: make these two lengths local.
- **At track Q2**: the reach of the cancellation is now counted in local quads (the path
  between two singularities is measured in the metric of the sizing field). The scale at which
  curvature is read is left.

### DT-026 — Painted density: reading Blender's Weight Paint to do (Blender bridge)

- **Born at** milestone 8.
- **Done on the engine side**: painted density is a weight per vertex, from 0 (lowest density)
  to 1 (highest density), with a contrast setting (`density_contrast`). Convention set on
  7 October 2026 to match Weight Paint: an unpainted vertex is worth 0 and serves as the base
  level, there is no neutral value to know.
- **Left at the time**: nothing read a vertex group of Blender yet; on the command line, the
  weights come from a text file (`--density`).
- **Closed at milestone 10 (8 October 2026)**: the add-on lets the user pick a vertex group of
  the object and a factor (how many times more quads a weight of 1 gets than a weight of 0),
  and hands its weights to the engine.

### DT-027 — Guards on computing time for pathological inputs

- **Born at** milestone 8, on Desert Rose (outside acceptance).
- **Finding**: on a surface where the map does not unfold, traces went round on the spot for
  more than a quarter of an hour; their number of steps is now bounded by the length they are
  allowed, and a folded map sends straight to the fallback chain. But this fallback
  (`integer-grid`) still takes more than two minutes on this model at 8000 quads, and the chain
  has no overall time limit.
- **To do**: with the repair preprocessing (J11); until then, bound the fallback too.

### DT-028 — Symmetry: missing, Quad Remesher has it

- **Born at** milestone 8, reading again the options of the Blender add-on of Quad Remesher
  (1.4).
- **Finding**: Quad Remesher offers a symmetry along X, Y or Z. The specification of openclare
  does not plan for it, and nothing guarantees today that a symmetric model gives a symmetric
  mesh.
- **Lead**: remesh one half with a border laid on the plane of symmetry, then mirror it; open
  borders are handled already.
- **Decision of 7 October 2026**: wanted. Entered in the roadmap as milestone 9 bis.
- **Decision of 8 October 2026**: it is a side feature, which does not move quality forward;
  it comes after the work on quality.

### DT-029 — Stretched quads on the wall of a cylinder

- **Born at** milestone 9.
- **Finding**: on a cylinder with flat ends, the quads of the wall are 1.3 times wider than
  they are high. The number of lines that run along the wall is the one that comes out of the
  two discs, and a disc, whose rings shrink towards the centre, wants fewer of them than the
  wall. The map settles between the two.
- **Leads**: place the four singularities of each disc closer to the rim (the zone in rings is
  then thin); or weight the map by the area concerned on each side.

### DT-030 — Patch chain (QuadWild) not written

- **Born at** milestone 9, by decision of 7 October 2026.
- **What was planned**: a second chain for hard surfaces, on the principle of QuadWild: trace
  patches from the field without going through a map, then fill them with patterns.
- **What is done instead**: hard edges are cuts of the T-mesh of the existing chain (sheet M5).
  On the four parts of the corpus, edges and corners are recovered exactly.
- **What is lost**: the patch chain does not depend on a map with no inverted triangle; it was
  also the fallback planned if parametrization failed (section 6). So that fallback does not
  exist: today `integer-grid` stands in for it (DT-009).
- **To take up again if** real mechanical parts make the map or the T-mesh fail.

### DT-031 — Triangles of nearly no area left by preprocessing

- **Born at** milestone 9; the defect dates from milestone 1.
- **Finding**: so as not to open a slit, isotropic remeshing keeps flat triangles along hard
  edges (area of the order of 1e-18). They made the seamless map collapse in some
  configurations (cylinder, `hard-surface` preset).
- **Same family**: a triangle with two sides on the same hard edge (it cuts a slight angle of
  the line) is bound to be flattened in the map; it made the map give up holding the hard
  edges, and skewed the count of angles around vertices.
- **Dealt with downstream** at milestone 9: parametrization, the count of inverted triangles
  and the computation of angles around a wall put up with them.
- **To do**: remove them at the source, by an edge flip or collapse that respects hard edges.

### DT-032 — Models made of many separate pieces: the whole chain falls back

- **Born at** milestone 9, on two game characters fetched from BlenderKit (see CREDITS.md).
- **Finding**: `mahaveer` is made of 164 separate open pieces (clothes, jewels), `marine` of
  820, with 305 edges of three faces besides. The specification says that several separate
  pieces are "accepted, each piece is treated on its own": this is not what the code does.
  Everything is treated as one block; one piece without a valid T-mesh is enough for the whole
  model to go to the fallback chain, which takes 84 seconds on `mahaveer` and returns an
  invalid mesh (17 pieces out of 164, budget missed by 28 %). `marine` is refused at once, with
  a clear diagnosis, for its non-manifold edges: it is the behaviour planned until repair
  (J11).
- **Consequence**: assembled models, common in production, do not go through.
- **To do**: treat each piece for itself (budget shared in proportion to area, fallback piece
  by piece), ignore or group the pieces smaller than a quad. Merging pieces that go through
  one another into one skin stays the business of milestone 11.
- **Closed at track Q1**: each piece is meshed for itself, with its share of the quads (in
  proportion to area, weighted by painted density, with a minimum per piece), a work mesh to
  its measure, and its own fallback if needed. On `mahaveer`: 15 seconds instead of 84, 150
  pieces out of 164 through the main chain, budget held to 1.8 %. What is left is followed by
  DT-034.

### DT-033 — Terrains and scans: no tool aligns, the field has nothing to follow

- **Born at** milestone 9, on a mountain terrain of 543,557 triangles (BlenderKit).
- **Finding**: openclare gives a valid mesh there, on budget, with three times fewer irregular
  vertices than Quad Remesher (56 against 167) and a comparable fidelity. But alignment is at
  18.6 degrees, close to chance (22.5), as for every tool measured (QuadriFlow 20.2, Quad
  Remesher 18.6): an eroded relief has no curvature directions that are stable at the scale of
  a quad. Same family as the scan `rock` (DT-021).
- **Lead**: on these surfaces the alignment metric does not say much; what would count would
  be to follow ridge and valley lines, to be detected as lines to follow rather than by
  curvature point by point.

### DT-034 — Small pieces: a few still go through the fallback, or do not come out of it

- **Born at** track Q1, on `mahaveer` (164 pieces) and on Desert Rose (96 shells).
- **Finding**: on `mahaveer` at 8000 quads, 6 pieces still go through the fallback chain and 8
  come out of it with no face; the mesh keeps 9 faces that are not quads. On the rose, 12
  shells out of 96 go through the fallback. The causes, which the report now details piece by
  piece: the map folds (most often), no sound whole lengths, a border off the isolines, a lost
  trace.
- **Besides**: a piece whose fair share is less than a quad still gets the minimum (6 quads
  for a ball, 16 for a ring). On a model with very many small pieces, these minimums weigh on
  the budget. **Decided on 8 October 2026**: this minimum is kept by default, and the setting
  `skip_pieces_under` lets one leave aside the pieces whose share is below a threshold.
- **To do**: take these causes up one by one; make the fallback itself safer on very small
  pieces, or do without it.
- **Mending the input, 9 October 2026**: before a model is split into pieces, borders torn
  apart are stitched and faces that share an edge in threes are taken apart there
  (`TriMesh::stitched`). The author's car, exported with a modifier that splits edges for
  shading, went from 373 pieces to 17 and from 1787 quads out of 8000 to 6995; a low-poly
  game character with every hard edge split (`wolf`) comes out all in quads. Faces taken
  apart stay apart: the result has a border where the input had an edge with three faces.
- **Two assembled hard-surface models, 9 October 2026** (BlenderKit, proposed by the author;
  see CREDITS.md), `hard-surface` preset, 8000 quads. Both come out invalid, and far worse
  than the characters above. `f16`, a fighter jet in 296 pieces: 187 pieces not meshed at
  all, 57 through the fallback, 140 faces that are not quads, 36 seconds. `camera`, a film
  camera in 42 pieces: 9 not meshed, 15 through the fallback, and 351 seconds, nearly all of
  them in the fallback (DT-027). The first cause by far is again "the map folds over". Parts
  of such models are thin plates, discs and rings with hard edges all round: the case to
  study first. Neither has Quad Remesher references nor a page yet.

### DT-035 — Rerun visualization: what is not shown, version behind

- **Born at** track T2.
- **What is not shown**:
  - the patches of the global structure as coloured regions: their borders (the cuts) are
    seen, not their surface;
  - the seamless map was not shown until 9 October 2026. It is now, as a stage of its own
    before the layout: its grid lines, the triangles it folds or flattens, those that folded
    when every line was put on a grid line, and the guide lines kept and let go. Not shown
    yet: the map of the attempts in between, and that of the fallback chain;
  - lost traces and overlaps, which the T-mesh counts without keeping their position;
  - the comparison between remeshers: a recording shows one run only.
- **Approximations**: the whole length of an arc is written at the middle of the chord between
  its two nodes, not at the middle of the arc; quads are displayed with smooth shading.
- **One stage at a time** (asked for on 8 October 2026, done): along the `step` timeline each
  stage clears the one before; the `stack` timeline keeps everything. Still, the stages of a
  model in several pieces go by piece after piece, with no overall view of one stage over all
  the pieces.
- **Version**: Rerun 0.36.3, the last that builds with Rust 1.95; the most recent (0.38.1) asks
  for Rust 1.96. The viewer of 0.36 also demands exact versions of `wasm-bindgen` and its
  neighbours, pinned in `Cargo.lock`.
- **Weight**: `Cargo.lock` goes up to a thousand packages, which only the two Rerun crates
  use. First build in release mode: about 5 minutes for `openclare` with `--features rerun`
  (31 MB instead of 17), about 17 minutes for `openclare-view` (265 MB). The debug information
  of the dependencies was removed from the release profile for that: with it, `openclare-view`
  weighed 1.5 GB.
- **Embedded viewer not checked by eye**: `openclare-view` starts and stays open without an
  error, but it is in the web viewer of Rerun that the recording was looked at.
- **Measured times**: during a recording, the time spent writing the layers of a stage is
  counted in the next stage.

### DT-036 — Deep pockets and cavities closed over by the mesh

- **Born at** track Q2, looking for where the Hausdorff distance of the ogre comes from.
- **Finding**: at 3000 quads, the point of the source farthest from the mesh (1.6 quad edges)
  is not on a finger but in two small pockets behind the eyes, 0.15 wide and 0.35 deep, which
  the mesh closes over. Quad Remesher, at twice the budget, follows them (0.36 edge). As long
  as they are there, the Hausdorff distance of the ogre (27 per mille) says nothing of its
  hands.
- **Lead**: it is the hollow counterpart of thin parts. The same measure, with rays cast
  outwards, would give the width of a cavity; it is left to decide whether to follow it or to
  plug it cleanly.

### DT-037 — Volume: the mesh is inscribed in the shape

- **Born at** track Q2. High priority: the author holds the preservation of volumes to be very
  important.
- **Finding**: the vertices of the mesh are laid on the surface. Wherever it bulges, the faces
  then pass below it, and the volume goes down: 99.6 % of the volume of the ogre at 8000
  quads, 98.8 % at 3000, 96.5 % at 1000; between 97 and 98 % at 1000 quads on the other closed
  models. On a thin part it is far more: a finger with six faces has only 83 % of the section
  of the finger, with four faces 64 %. A Catmull-Clark subdivision, the normal use of such a
  mesh, takes away more volume still.
- **Lead**: after relaxation, adjust the vertices along the normal so that the mesh passes
  through the middle of the surface rather than below it (least squares on the distance from
  the source to the mesh, not from the vertices to the source); as an option, aim at the limit
  surface of the subdivision. The measure `volume_ratio` of the bench is there to check it.
- **Closed at track Q2 bis (8 October 2026)**: a last stage, fitting, does this
  (`fitting.rs`, setting `volume_fit`). Median volume kept over the bench: 100.0 % at the three
  budgets; the ogre 99.8 % at 1000 quads and 100.0 % above. What it leaves open is DT-039.

### DT-038 — Thin parts: limits of the bound on size

- **Born at** track Q2.
- **No ceiling on the share of the budget** that thin parts take: on a model made mostly of
  thin parts, the rest becomes very coarse. The only guard is the floor on size (a quarter of
  the size asked for).
- **The floor is tuned on one case**: at a sixth, one or two faces of `jerry` turned over on
  its ears at 1000 quads; at a quarter, they do not. Nothing guarantees that this holds
  everywhere.
- **More irregular vertices**: the ogre goes from 88 to 155 at 3000 quads, `jerry` from 102 to
  162. The `animation` preset, which wants few irregular vertices, keeps the same setting as
  `organic` for now; to look at again with DT-013.
- **The scores do not count thin parts**: the scale of `SCORES.md` sees what they cost
  (regularity) and not what they bring. A decision to take: add a criterion, or make it part
  of fidelity. (Decided since: part of fidelity.)
- **Thickness is not shown** in the Rerun recording; only its effect on the sizing field is
  seen there.
- **The measure**: seven rays across and four chords per vertex, the median over the
  neighbourhood. It costs 0.3 to 0.5 second. At the tip of a finger, the thickness across is
  overestimated by about a third (the central ray runs along the finger).
- **`hard-surface` preset**: the bound is switched off there, untried. A mechanical part with
  thin rods might need it.

### DT-039 — Fitting: limits, and the volume after subdivision

- **Born at** track Q2 bis.
- **After subdivision the volume is lost again.** Fitting makes the quad mesh keep the volume
  of the shape. A Catmull-Clark subdivision, the normal use of such a mesh, rounds it off and
  shrinks it. To keep the volume there, the fit should aim the limit surface of the
  subdivision at the source, not the faces of the mesh. **Measured at milestone 12**
  (`bench/creases.py` subdivides with Blender): the ogre at 1000 quads keeps 99.8 % of its
  volume as a quad mesh and 96.4 % once subdivided twice; 98.8 % at 3000 quads. The other
  closed organic models lose 2 to 3 % at 1000 quads the same way.
- **Rims stay inscribed**: vertices on hard edges, corners and borders do not move. The rim of
  a cylinder is still a polygon inside its circle; it could slide outwards in the plane of the
  cap.
- **The two guards are tuned on the bench**: a vertex goes no farther than a quarter of its
  shortest edge, and a face tilted by more than 20 degrees is put back. The second came from
  one face of Desert Rose; nothing says these values hold everywhere.
- **Only quads are looked at**: a face that is not a quad (the fallback chain leaves a few)
  asks for nothing, and its corners follow their other faces.
- **Vertices off the surface**: by design, but a later stage that expects them on the source
  (a shrinkwrap in Blender, say) will undo the fit. To say in the documentation of the add-on.

### DT-040 — Improving a mesh from a dry run

- **Born** on 8 October 2026, from an idea of the author: run the chain once to mark the mesh,
  turn the marks into one or several weights, and run again.
- **Tried for density** (a script outside the repository): after each round, a density is
  painted where the mesh was far from the source, and the next round uses it. On the ogre and
  the pig at 3000 quads, three rounds, contrast 4:

  | | Ogre, round 0 → best round | Pig, round 0 → best round |
  |---|---|---|
  | Distance to the mesh, 99th centile, share of a quad edge | 26.9 → 22.4 % | 17.4 → 15.3 % |
  | Mean angle deviation | 6.3 → 7.6 degrees | 5.3 → 6.2 degrees |
  | Irregular vertices | 155 → 183 | 102 → 104 |
  | Hausdorff distance | 28.4 → 25.6 ‰ | 10.0 → 8.2 ‰, then 11.8 the round after |

- **What it says**: density driven by the error gains 10 to 15 % on the worst distances, costs
  a degree of quad shape, and swings from one round to the next. Curvature and thickness
  already put the density about where it belongs, and every round redraws the whole mesh, so
  the marks of one round only half predict the next.
- **Where the idea should pay**, none of it written yet:
  - *keep the best of several rounds*: results vary from run to run, the marks exist in the
    library (`metrics::measure`), so marking a few runs and keeping the best is nearly free;
  - *a validity loop*: when a round gives an inverted face or a piece through the fallback,
    run again with what went wrong corrected there (smaller quads, another tolerance);
  - *a weight for alignment*: where edges follow the shape badly, pull the cross field harder
    towards curvature there. Alignment is the largest gap left with Quad Remesher.
- **Decided on 8 October 2026**: a quality track of its own, after the first version of the
  Blender add-on, starting with the best of several rounds and the validity loop.

### DT-041 — Blender add-on, first version: what it does not do

- **Born at** milestone 10, first step (8 October 2026), by decision: the least that runs
  first.
- **Blender waits while the engine runs**, and cannot be made to stop it. Since 8 October 2026
  the engine tells its host each stage as it begins and stops if the host says so
  (`Recorder::begin`, `on_stage` in the C interface), and the add-on shows the stage in the
  status bar, on the cursor and in the Info log, asking Blender to redraw each time. What is
  missing is a way for the user to say stop: Blender reads no key while an operator runs. The
  operator would have to run the engine aside, on a copy of the mesh (reading Blender's
  arrays in place from another thread is not safe while the user can change the scene).
- **The redraw in the middle of a run is a workaround** (`wm.redraw_timer`), which Blender's
  documentation advises against. Seen to work on Blender 5.2.2 on Linux (a picture of the
  window taken during a run shows the stage in the status bar); not tried elsewhere.
- **Few settings shown**: preset, quads, adaptive size, a vertex group for density and its
  factor (added on 8 October 2026 at the author's request). Quads round thin parts and volume
  fitting exist in the engine and pass through the interface as text; the panel does not show
  them.
- **Vertex group weights are read one vertex at a time** in Python: Blender offers no array of
  them. About 0.2 second for 100,000 vertices, so a couple of seconds per million.
- **Export, added on 8 October 2026** at the author's request: a section of the panel with
  boxes to tick for the two UV layers and for marking sharp the edges that follow a hard edge
  of the original. The limits of the UV layers are in DT-042.
- **Curve categories, added on 9 October 2026** at the author's request: guide curves (cyan)
  and sharp curves (violet), each in its collection, with buttons to assign, remove, select
  and deselect. What it leaves aside:
  - the two colours are fixed, and only *Assign* gives them: a curve dropped into one of the
    two collections by hand is used but keeps its colour;
  - *Assign* sets the 3D viewports of the screen to draw wires in the colour of their object
    (*Wire Color: Object*), without which the colours would not show; it is not set back;
  - a curve lying exactly on the surface is half hidden by it whatever its colour: an offset
    on the Shrinkwrap, or *In Front* on the curve, is left to the user;
  - *Remove* puts a curve back in the collection of the scene, not where it came from;
  - a sharp curve is let go in the same cases as a guide curve (DT-044), and then leaves no
    hard edge at all.
- **Still to do**: reading the edges *marked* sharp in Blender as hard edges to keep. Today
  only the angle between faces makes a hard edge (preset *Hard surface*, 30 degrees); the
  engine has no way yet to be given a list of edges.
- **Linux, 64-bit Intel or AMD only.** The archive carries one native library; Windows and
  macOS need their own builds, and a way to make them (continuous integration).
- **Reading in place relies on how Blender lays a mesh out in memory**: positions as a packed
  array of three floats, corner vertices and triangle corners as packed integers. Checked on
  Blender 5.2.2 against the documented accessors; true since Blender 4.0 as far as its
  release notes say, not checked on 4.2 to 5.1. If a version changed it the add-on would read
  nonsense rather than fail; so the add-on compares a few values with the documented accessors
  first, and copies the arrays out the documented way if they do not match. That other way is
  tested, but only by forcing it.
- **Tried without interface only.** `make addon-test` runs the operator in a Blender started
  with `-b`; the archive is accepted by Blender's own validation. Nobody has installed it and
  pressed the button yet.
- **The result is a bare mesh**: no material, no vertex group, no custom attribute carried
  over from the original.
- **In the local coordinates of the object**: a non-uniform scale on the object is not applied
  before remeshing, so quads that are square in local space are not in the scene.

### DT-042 — UV layers unfolded from the quad mesh: limits

- **Born at** milestone 10 (8 October 2026), with the Export section of the add-on.
- **What they are**: both layers are read off the quad mesh itself, by laying one quad on a
  unit square and unfolding its neighbours next to it (`uv.rs`). It works on any mesh of
  quads, pieces through the fallback chain included; it does not use the map or the patches
  the chain worked with.
- **Not what was first described.** The atlas was announced as one rectangular island per
  rectangular patch of the layout. It is made of islands grown from the quads instead, each
  kept within a square of its own: far fewer islands (53 on the ogre at 8000 quads, where the
  layout has several hundred patches), but with ragged outlines.
- **Every quad takes the same room**, whatever its size on the surface. Where quads are four
  times smaller (thin parts, painted density), the texture is four times finer there, and
  coarser elsewhere.
- **The atlas is filled at 50 to 60 %** (60 % on the ogre at 3000 quads). Islands are odd
  shapes fitted cell by cell; a model in hundreds of pieces falls to 25 % (`marine`).
- **Many seams**: about a third of the edges on the ogre (1034 of 5898 at 3000 quads). Islands
  are cut where the unfolding meets itself, which leaves cuts inside them. Seams placed where
  they are least seen (creases, hidden sides) is another problem, not started.
- **The grid layer overlaps itself**, by nature, and its coordinates leave the unit square
  (they count quads). Fine for a pattern that repeats; a renderer that expects coordinates
  between 0 and 1 needs the texture set to repeat.
- **Faces that are not quads** (the fallback chain leaves a few) get a small island of their
  own.
- **Seen as a drawing of the coordinates only**: the two layers of the ogre were read back
  from Blender and plotted; nobody has yet looked at a texture on the model.

### DT-043 — Creases: limits

- **Born at** milestone 12.
- **Little effect on organic shapes.** The rule finds few folds there (124 edges on the ogre at
  3000 quads) and the subdivided mesh is hardly closer to the source for it. Either such
  shapes have few folds worth a crease, or the rule misses them; the bench cannot tell which.
  A picture of a subdivided nail or eyelid, with and without, would.
- **A fold is only creased where edges run along it.** Nothing here moves edges onto a fold:
  with the organic preset a fold the cross field did not follow is crossed by edges, and no
  edge gets its weight. Hard edges (preset *Hard surface*) and guide curves are what put
  edges on a line.
- **Thresholds set by hand**: no weight under 30 degrees, full weight from 70, at least 60 %
  of the turn close to the edge. Tried on a lens and on the bench, not tuned.
- **A partial weight is Blender's**: the engine gives a number from 0 to 1 and Blender turns
  it into a sharpness for the subdivision in its own way. Another host may read the same
  number otherwise.
- **Borders get no weight**: how a border is subdivided is a setting of the modifier.
- **The `part` model at 1000 quads** stays at 15 per mille after subdivision, creases or not:
  its holes have few quads round them and subdivision shrinks them.
- **No setting**: the weights are always computed (it costs a few queries per edge) and the
  add-on has a box to write them or not. The thresholds are not exposed.

### DT-044 — Guide curves: not kept everywhere

- **Born** with guide curves, 9 October 2026. Priority high: a curve that is let go is a
  feature that did not work for the user who drew it.
- **Which curves are kept is not predictable enough.** On the ogre at 3000 quads a line down
  the front or the back is kept, two rings out of three round the belly and the arms are, and
  the profile of the face and a line through the crotch are not. During the work the same
  curve went from kept to let go and back with changes that had nothing to do with it. What
  decides is one or two triangles of the work mesh along the line: a single fold of the map
  that stiffening cannot undo is enough to let a curve go.
- **Small loops need enough quads**: a loop under about nine quad edges round is let go (the
  eyes of the ogre at 3000 quads; kept at 8000).
- **First try on the author's own scene (9 October 2026)**: the ogre as he sculpted it (2.3
  million vertices), with a density painted on the head, three loops drawn round the eyes and
  the mouth as guides and a line along the belt as a sharp curve, each held by a Shrinkwrap,
  at 12,000 quads and adaptive size at 100 %. All four are kept, with no break: 86 edges
  along the loops, 21 hard edges along the belt. That try showed a mistake in what the report
  says, since put right: 30 of the edges along the loops were taken for hard edges, hence
  marked sharp and creased. An edge was told to be along a guide curve by its distance to the
  curve as it was drawn, and the line the mesh follows strays from that on a small loop; it is
  now told by the line of the work mesh nearest to it.
- **Curves that touch are one**: letting go is decided curve by curve, but two curves that
  cross hang together and go together. A long curve with one bad stretch is lost whole, where
  it could be kept up to that stretch.
- **The fallback chain ignores curves**, and the fallback of the fallback is to do the piece
  again without them.
- **A curve is cut into the work mesh by its crossings**: its own bends inside a triangle are
  lost, which is nothing on a dense sculpt and too much on a model of a few large triangles.
  An open curve loses up to a triangle at each end.
- **Cost**: each curve tried alone is one more solve of the map, about half a second each on
  the ogre.
- **Only curve objects** are read by the add-on: not the edges of a mesh object, not
  annotation strokes, not face set borders. The edges along the curves come back in the
  report, but the add-on does not mark them on the result.
- **Not measured on the bench**: no model of the bench has curves, so no metric says whether
  the edges along a curve are where they should be; the tests check it on a sphere and on a
  flat sheet.
- **Where to look next**: the triangles along a laid line. Evening the mesh out again along
  the line (collapsing the short edges crossings leave), rather than making up for them in the
  map and in the layout as now, should make the outcome depend less on luck.

### DT-045 — Sculpts of millions of triangles: most of the time goes to preprocessing

- **Born on** 9 October 2026, on the author's own ogre: 4.7 million triangles, straight from
  Blender through the add-on's library.
- **Finding**: 72 seconds for 12,000 quads, of which 53 in preprocessing (bringing the sculpt
  down to a work mesh), 6 in the layout, 3 each in sizing, quantization, relaxation and
  fitting. The bench never sees this: its ogre is the same sculpt already brought down to
  200,000 triangles. Blender is frozen for the whole of it (DT-041).
- **Leads**: measure where the 53 seconds go (welding, decimation, the hierarchy over the
  source for projections); decimate in a first coarse pass by clustering vertices before the
  error-driven one; do the passes on several threads. Relaxation and fitting project onto the
  full source too, and could project onto a lighter copy of it.

### DT-046 — Add-on: a button to write the density worked out from curvature as weights on the source

- **Born on** 9 October 2026, an idea of the author.
- **What is wanted**: a button of the add-on that writes, as a vertex group of the dense
  source, the density the engine works out by itself from the shape (the sizing field of
  *Adaptive Size*, thin parts included). The user then paints on top of it in Weight Paint to
  add detail where he wants more, without painting again what was already handled, and gives
  that group back as *Density*.
- **What it takes**: the sizing field lives on the work mesh, not on the source; it has to be
  read back at each vertex of the source (closest point), turned into a weight from 0 to 1
  that means the same as the painted density (the inverse of what `density_contrast` does),
  and given out by the C interface without running the rest of the chain. And a run with that
  group as *Density* should then switch *Adaptive Size* off, or the shape would count twice.

### DT-047 — Borders between materials: mostly let go on a real model

- **Born on** 9 October 2026, with the materials of a model given to the engine as groups of
  triangles (sheet "Materials, and mending the input" of the specification).
- **Finding**: on a flat sheet in two materials the border is a line of edges and no quad is
  across it. On the author's car (`venturi`, thirteen materials, 8000 quads) every quad gets
  the material of the surface under it, but the borders are let go on 6 pieces out of 17
  with the `hard-surface` preset, and no edge is along them; 22 edges with `organic`. A quad
  across a border has the material of its middle, so the borders of the result are jagged.
- **Why**: the map folds with all the lines of a piece at once, and the lines of a piece are
  then kept all together or not at all. Trying each alone, as for guide curves, is one solve
  of the map per line: with the borders of thirteen materials that ran for more than twenty
  minutes, and was capped at 8 lines per piece. It is DT-044 on a larger scale.
- **Leads**: find the lines that make the map fold from the triangles that fold, rather than
  by trying each; keep a line up to the stretch that cannot be kept; where a border is let
  go, move the vertices next to it onto it after the fact.
- **First step, 9 October 2026** (track "lines kept"). Three things, and what they showed:
  - *Letting go near the folds.* When the map folds with every line on a grid line, the lines
    are now let go only within a few edges of the folded triangles, a reach that doubles up to
    five times, before whole lines are tried. A line can thus be kept in part. On the car with
    the `organic` preset: 61 edges along the borders instead of 22. Little, then.
  - *A measure*: the report counts the faces that lie across a border between two groups
    (`faces_across_groups`). On the car at 8000 quads: 1456 of 8450 with `organic`, 1055 of
    6995 with `hard-surface`.
  - *Why lines are let go*, piece by piece (`Report::line_causes`, printed by the command
    line). With `hard-surface` the five pieces that lose their borders are the five whose map
    folds without any border either: their creases are what the map cannot keep, the
    materials have nothing to do with it. With `organic`, two pieces lose them whole and three
    in part.
- **A lead tried and dropped the same day**: bringing the size of the quads down to the gap
  where two lines run closer than a quad is wide (stripes, window frames), the way it comes
  down round thin parts. Written, measured on the car, and taken out again: 52 edges along
  the borders instead of 61, 1471 faces across instead of 1456, and no better at 20,000
  quads (148 edges, 2305 faces across). Lines too close together are not what makes the map
  fold here.
- **What is left to look at**, and it has to be looked at rather than guessed: where exactly
  the map folds on the body of the car. The likeliest cause now is the corners. A border that
  turns by a quarter turn sits on two grid lines; one that turns by 45 or 120 degrees, or
  three borders that meet at odd angles, need an irregular vertex exactly there, and nothing
  puts one there. The recording for Rerun does not show the map yet (DT-035); it should, for
  this.
- **With the map shown and counted, the same day** (`--notes` on the command line prints what
  the stages remark on; the recording shows it). On the body of the car, 668 triangles fold
  when every line is on a grid line. Only 199 of them are within two edges of a place where
  the lines turn by more than 30 degrees, meet or end; 638 are within two edges of a line. So
  corners are a third of it at most: the map folds along plain stretches of the lines too.
  And the body does not come out of the main chain at all, borders between materials or not:
  done again without them, its map has no fold, but the layout finds a border off the grid
  lines and the whole body goes to the fallback chain, which knows nothing of lines. The 61
  edges along borders are all on the small pieces. The open borders of the body, some of them
  made by taking faces apart along edges with three faces, are the first thing to look at in
  the recording.
- **The car exported as its author meant it, the same day**: with the mirror and the
  subdivision applied and without the two modifiers that tore it (one splits edges for
  shading, the other is a boolean that leaves 66 edges with three or four faces), it is one
  closed surface, and everything above reads differently. With the `organic` preset at 8000
  quads: 17 pieces all through the main chain, 8789 quads and nothing else, 982 edges along
  the borders between materials and 717 faces across one, in 39 seconds. Letting lines go
  near the folds only is what does it: on the body 858 guide edges out of 3046 are let go,
  where all of them were before. Left: those 858, a third of them at places where the lines
  turn or meet; 1337 irregular vertices, many more than the shape asks for; and the
  `hard-surface` preset, which finds creases all over a smooth subdivided body at 30 degrees
  and sends most pieces to the fallback chain. The first export, torn, is no longer in the
  corpus; what it showed of mending stays true (DT-034).
- **Not the size of the model, but luck, measured the same day.** The author asked whether
  the chain is sensitive to the size of a model, his car being tiny (0.18 long). It is not:
  the ogre gives 155 irregular vertices and a valid mesh at every size from a millionth to a
  million times its own, and the car scaled by an exact power of two (1/1024, 8, 1024) gives
  the same map, fold for fold. But scaled by 3, by 0.7, or by 1.0000001, the body of the car
  loses every one of its 3046 guide edges, where it kept 2188 of them: a change in the last
  digits of the coordinates decides it. The 982 edges along the borders reported above are
  one outcome among others, not a result to count on. Letting go near the folds either
  settles in its five rounds or does not, and then everything goes. That is the first thing
  to make steady (DT-044), before corners.
- **Another lead tried and dropped, 9 October 2026**: a line the map lets go still holds the
  cross field, so the field was worked out again without the lines let go, and the layout
  with it, up to three times. That makes the outcome steady, and steadily poor: 68 to 90
  edges along the borders at every scaling of the car, the lucky ones included. Each release
  changes the field, the map folds somewhere else, and more lines go.
- **Three leads dropped in a day** (the gap between lines, the size of the work triangles,
  the field released): enough guessing. The map is now recorded for Rerun, which its author
  can look at and the one writing this cannot. The next step is to give the latter eyes too:
  the layers of the recording written as pictures, to see where the body of the car folds
  before anything else is changed.
- **Also left**: the bench passes the groups of a model to the chain but keeps nothing of what
  the report says about them; the command line writes the group of each face next to the
  mesh, not into it.

### DT-048 — The work mesh has triangles of one size: what is finer is lost before anything else runs

- **Born on** 9 October 2026, from what the author saw in the recording of his car: the fine
  edges of the model are gone after preprocessing.
- **Finding**: no vertices are merged by distance (the only weld is of border vertices at the
  very same place, a billionth of the diagonal apart at most). What removes fine edges is the
  isotropic remeshing: it gives the work mesh triangles of one size and collapses whatever is
  shorter. On the car, 80,000 triangles make edges of 1.45e-3 where a quarter of the edges of
  the source are less than half that and 7 % less than a fifth: the narrow strips that round
  its panels off. The work mesh ends up 1.2 % of the diagonal away from the source at worst,
  0.2 % when edges sharper than 30 degrees are kept as creases; but then the map cannot keep
  that many creases (DT-047).
- **Leads**: triangles whose size follows the shape (smaller where the source bends within
  less than a triangle, as the sizing field already does for the quads); or the source left
  as it is where it is already fine and well shaped enough; a bound on how far the work mesh
  may stray from the source, checked while it is made.
- **The first lead, written, measured and taken out again the same day.** The isotropic
  remeshing was given a size per place, from how tightly the source bends (a chord may stray
  by a twentieth of the edge length), between a quarter and twice the one size, for the same
  number of triangles. On the car it is worse: the work mesh strays by 2.1 % of the diagonal
  instead of 1.2 %. What the car loses is not narrow rounded strips but hard edges, each a
  single edge between two large faces, and no size of triangle keeps those: only keeping them
  as creases does (0.2 %). Elsewhere it is neither better nor worse: on the ogre at 3000 quads
  the Hausdorff distance is halved (28 to 13 per mille, the pockets behind the eyes, DT-036);
  on the pig it goes up (10 to 17) and `jerry` comes out invalid.
- **A lead that holds, 9 October 2026**, after pictures of the map taken by the author in
  Rerun: the spoiler of his car came out serrated, its edges chewed by the remeshing of the
  work mesh. A new setting, `edge_angle` (`--edge-angle`): edges of the input sharper than it
  are kept as lines of the work mesh, so its shape stays crisp, and the quads follow them
  where the map can, as it does for guide curves; they are not creases to keep at any cost,
  nor hard edges of the result. On the car as it is seen, at 60 degrees and 8000 quads: the
  spoiler, the air intakes and the lines of the body come out straight; 486 irregular
  vertices instead of 886; mean distance to the source 1.60 per mille instead of 2.01. And a
  cube under the `organic` preset comes out exact (a test). What it costs: more of the small
  pieces of the car fail (7 through the fallback chain and 6 not meshed, against 1 and 1),
  and the run takes 121 seconds instead of 45. Off by default, in no preset, not in the
  add-on: those small pieces first.
- **Acute tips, the same evening**, from two more pictures of the author: the air intakes of
  the car, where two edges run into one another at a sharp angle. Two lines that meet at
  less than a quarter turn or so cannot both be lines of a grid of quads. With `edge_angle`
  at 60 degrees the body of the car had 4837 guide edges, its map folded on 1191 triangles
  with all of them on grid lines, every one was let go, and the body went to the fallback
  chain. Now the lines are let go beforehand within a quad edge and a half of any place
  where two meet at less than 50 degrees: 127 triangles fold instead of 1191, 417 guide
  edges are let go out of 3707, and the car takes 76 seconds instead of 121, with 3 pieces
  through the fallback chain and 4 not meshed instead of 7 and 6. The body still ends in the
  fallback chain: its map has no fold by its own count, and the layout finds some. That
  disagreement between the two counts is the next thing to look at.
- **One count of folds, 10 October 2026.** Measured first: on the body of the car the layout
  found 4 triangles turned over in a map that counted none, and all 4 were among the 59 the
  map had set aside (slivers, and triangles laid along a line), which weigh nothing in its
  solve and which it did not look at. The map and the layout now share one count
  (`Parametrization::turned_over`), so the map stiffens round such a triangle and lets the
  line next to it go. That removed the reason the body was refused for, and showed the next:
  2 edges of its borders are not on a grid line, and a wall off the grid lines is still all
  or nothing. Two things done on the way: a guide edge that ends up off the grid lines is
  let go like the others, and round an edge with more than two faces, the faces that go
  straight on from one another stay joined (two sheets that cross stay two sheets; the car
  goes from 23 pieces to 22).
- **Where that leaves the car** with `edge_angle` at 60 degrees: 4 pieces through the
  fallback chain and 3 not meshed, the body among the first; about 1000 edges along lines;
  130 seconds, more than before, for the map now works harder on a piece the layout then
  refuses for something else. The outcome is at least steady: 967 to 1005 edges along lines
  and the same pieces lost when the car is scaled by 3, by 0.7 or by 1.0000001, where a
  change in the last digits used to decide everything. The models of the bench and the
  author's ogre with its curves come out as before.
- **Next**: those 2 border edges. A border the map did not put on a grid line should cost the
  stretch of border, not the piece.
- **What the car asks for instead**: its hard edges kept. At 30 degrees they are 9058 edges of
  the source in 179 connected lines, 5577 quad edges long in all for a mesh of 8000 quads,
  with 1300 junctions and 246 loose ends. Short stray bits are not the trouble (33 quad edges
  in all). It is a net of lines nearly as dense as the mesh asked for, which is DT-047 again:
  how many lines the map can keep.

### DT-049 — The bench has no model with curves or a painted density

- **Born on** 9 October 2026, at a question of the author: how is what was said of curves,
  densities and materials measured, and can it be run again?
- **Finding**: materials, yes: the car is in the corpus with the material of each triangle
  beside it (`venturi.groups`, written by `bench/export_object.py`), and the bench passes it
  on. Curves and painted densities, no. The tests draw their own on spheres and flat sheets.
  What was said of the ogre with curves cut out of it, and of the author's own scene (his
  sculpt of 4.7 million triangles, four curves and a painted group), came from files copied
  out of Blender into a scratch folder, in no repository, and from no `.blend` kept.
- **To do**: the same sidecars for those as for materials, in the formats the command line
  already reads (`<model>.curves`, `<model>.density`), written by `bench/export_object.py`
  from a collection of curves and a vertex group; a model of the corpus that has them, the
  ogre with its author's curves being the obvious one; and what the report says of lines and
  borders kept among what the bench stores.
- **Done on 9 October 2026, another way than the sidecars**: the bench runs from scenes. A
  model is a `.blend` that holds the object to remesh, with its modifiers, its materials, its
  vertex groups and the curves drawn on it, and the settings of the add-on saved with the
  scene; `openclare` and `openclare-matched` run on it through the add-on, in Blender, as its
  button does (`bench/blender_run.py`), and the bench only sets the number of quads. Nothing is
  copied out of Blender by hand any more: the file that is measured is the file that is kept.
  - `ogre_hp` is a model of the bench: the sculpt of 4.7 million triangles with its author's
    three guide curves, his sharp curve and his painted density (`bench/scenes/ogre_hp.blend`,
    made by `bench/make_scene.py`). At 12,000 quads asked for: 12,110 quads, valid, 418
    irregular vertices, 86 edges along the guide curves, 21 along the sharp one, no line let
    go, in 86 seconds; Hausdorff distance 7.3 per mille, mean angle deviation 6.3 degrees.
  - `venturi` is the car as it is in its file, modifiers and all, instead of an OBJ exported
    without two of them and a `.groups` file beside it.
  - The other models are scenes too, made from their OBJ file as it is
    (`bench/make_scenes.py`, `make scenes`); the tools that do not read a scene are given the
    object as the add-on sees it, exported by the same script (`bench/out/<model>/source.obj`).
  - With each run of the add-on the bench stores what the engine says of its lines (`engine`
    in `results*.json`): `guide_edges`, `sharp_curve_edges`, `faces_across_groups`,
    `welded_vertices`, `edges_taken_apart`, `guides_let_go`.
- **What the move showed: the chain is sensitive to the last digits of its input.** Blender
  stores positions in single precision; the OBJ files of the corpus were in double. On the
  same numbers the add-on and the command line make the same mesh, face for face (checked on
  the cube, Suzanne, the pig and the ogre). But at 3000 quads the pig makes 3031 quads from
  its OBJ in double precision, 3054 from the same positions rounded to single precision (a
  move of 1.2e-7 at most), and 2943 from those written with nine digits and read back (a move
  of a few billionths); the ogre 3019, 2949 and 3057; Suzanne 2986, 2986 and 2983. The irregular
  vertices hardly move (102, 104, 104 on the pig; 155 each time on the ogre); the cube, whose
  coordinates are exact either way, does not move at all. So two or three per cent on the
  number of quads is noise, and what is stored in `bench/results` (measured from the OBJ
  files, before this) will not come out the same to the unit from the scenes.
- **The car cannot be run as it is.** On the scene with its four modifiers (40,764 triangles,
  15,110 border edges where Edge Split tore it, 334 edges of three faces or more where the
  boolean cut) a run of the add-on grew to 53 GB of memory and the system killed it. Since
  then every process the bench starts is capped (`MEMORY_CAP` in `bench/run.py`, 12 GB, 24 for
  `ogre_hp`), `bench/blender_run.py` caps itself, and a run that hits the cap is reported as
  `out of memory`. No figure is stored for the car; where the memory goes is not known yet.
- **The car is in the reports since 10 October 2026**, the memory having been found (DT-050):
  measured at 1000, 3000 and 8000 quads with every tool, Quad Remesher references made, a
  page with pictures (`docs/models/venturi.md`), and on that page what the engine says of
  the lines of the scene. As the add-on runs it (*Organic*, sharp edges not kept as lines):
  836, 3049 and 8352 faces, none valid, for two to five faces that are not quads or one that
  is turned over; 16, 29 and 71 edges along the borders between materials, 348, 846 and 1470
  faces across one; the blade of the spoiler is lost at 3000 quads. QuadriFlow gives up, the
  fallback chain refuses the mesh, and Quad Remesher keeps the surface torn (11,146 border
  edges, 379 parts, 14,193 faces for 3000 asked) but is by far the closest to the shape. The
  only valid meshes of the three tables are ours at 8000 quads: with quads of one size, and
  asked for as many quads as Quad Remesher made (19,050).
- **Left**:
  - The scenes are in git since 9 October 2026, through Git LFS, and the OBJ files they were
    made from are gone. There is no remote yet: what LFS stores is on one machine.
  - **Done on 11 October 2026**: `ogre_hp`, `f16`, `camera` and `wolf` are measured with
    every tool, with their Quad Remesher references and their pages, and the chains of this
    repository were measured again on every model from its scene, pictures included. The
    marks hardly move (organic models: 71, 76 and 80 at the three budgets, for 70, 76 and
    81). What the other tools gave on the models that were in the bench before the scenes
    was not measured again: QuadriFlow and Instant Meshes ran on the OBJ files in double
    precision, and the Quad Remesher references were made from them.
  - What that run showed besides: on `ogre_hp` at 3000 quads the curves are let go, no edge
    along them, where 11 and 5 are kept at 1000 quads and 70 and 14 at 8000 (DT-044); Desert
    Rose at 8000 quads has two faces turned over out of 8484, where it had none; the body of
    `wolf` was not meshed at all (DT-052, closed).
  - The other tools get the object of a scene, not its curves nor its painted density:
    `openclare-uniform` (the command line) is given the materials (`source.groups`) and
    nothing else, QuadriFlow and Quad Remesher the triangles alone. On `ogre_hp` the comparison
    is not on equal terms. The sidecars first thought of (`.curves`, `.density`) would give
    the command line the rest.
  - The scenes made from an OBJ file carry an adaptive size of 50 % whatever the preset,
    which is what the bench always ran `openclare` with; the add-on, left to itself, sets 0 %
    with *Hard surface*. On the models with hard edges the bench does not measure what the
    preset gives out of the box.
  - The BlenderKit scenes are one mesh, every object of the file merged with its modifiers
    applied, as the OBJ files were: no materials, no modifiers, nothing of what a scene can
    hold.
  - What the engine says of its lines is stored with each run, not shown in `RESULTS.md` nor
    marked in `SCORES.md`.
  - The object as the add-on sees it is written as text: 255 MB and 21 seconds for
    `ogre_hp`, and 94 seconds to measure a result against it.

### DT-050 — A sliver with next to no area took all the memory of the machine

- **Born on** 9 October 2026, the hard way: a first run of the add-on on the author's car as
  it is seen, its boolean included, grew to 53 GB in six minutes on a machine of 62 GB with no
  swap, and the system killed Blender.
- **Cause**: the boolean leaves, among the 23 pieces of the car, a sliver with a long outline
  and next to no area. The work mesh of a piece is given triangles sized from its area: next
  to no area, triangles of next to no size, and the splitting of the long edges towards that
  size never ended. It did not show on the same car read from a file written with seven
  digits: the sliver came out differently.
- **Closed the same day**, twice over: the work mesh is never let grow beyond thirty-two times
  the triangles asked for (or those it starts from; splitting comes before collapsing, so a
  source of long thin triangles goes well above the count on its way to it), and a piece with less than a million
  millionth of the area of the model is left out. The run that took 53 GB now takes 49
  seconds and 2.2 GB, in Blender, on the same scene. A test has a strip of no area next to a
  ball; it runs out of a 4 GB allowance without the fix.
- **Around it**: every process the bench starts is now refused memory beyond 12 GB (24 for the
  sculpt of 2.3 million vertices), and reports a run that hits that as out of memory instead
  of taking the machine down.
- **Left**: the engine itself has no such allowance, and an add-on running inside a user's
  Blender is not protected by the bench's. A count of what a stage is about to allocate,
  refused beyond a share of the memory there is, would be. And 2.2 GB for a car of 40,000
  triangles is a lot: where it goes has not been looked at.

### DT-051 — Baking the sculpt onto the result: normal map, height and ambient occlusion

- **Born on** 10 October 2026, a request of the author: a "clever" algorithm that bakes a
  normal map properly. Decided the same day: the engine does it, not Blender's baker set up
  by the add-on; three maps come out (tangent-space normals, height, ambient occlusion); and
  its turn comes after the lines on a real model (DT-047, DT-048).
- **Why the engine**: the usual bake (*Selected to Active*) throws a ray from each texel of
  the low-poly mesh and takes what it hits first within one distance, or within a cage, set
  for the whole model. Too short and rays miss; too long and they go through to the other
  side of a finger or into the next fold; a cage has to be made and mended by hand. The
  engine has what those tools lack: every quad of its mesh was read from a map of the
  surface, so it knows where on the sculpt each quad sits before any ray is thrown.
- **What it takes**:
  - For each texel of the atlas the add-on already exports (one island per rectangular
    patch): its point on the quad, the point of the source that goes with it, and there the
    normal of the source, its height above the quad and its occlusion. The correspondence is
    exact on the work mesh before relaxation and fitting move the vertices; after them it is
    a starting point, to be finished by a short search bounded by what is known locally, not
    by one distance for the model. How good that starting point is has not been measured.
  - The detail is on the source, not on the work mesh (DT-048): the 4.7 million triangles
    have to be read, not their 200,000 triangle stand-in.
  - Tangent space as the renderer builds it (MikkTSpace), from the shading normals of the
    result as it is exported: sharp edges split, creases, and a choice to make on whether a
    mesh meant to be subdivided is baked as it is or as its subdivided surface.
  - Several samples per texel, and the islands grown by a few texels so that nothing bleeds
    between patches; a resolution asked of the user.
  - Height as a signed distance along the normal of the quad, written with the range that
    turns it back into a length. Ambient occlusion on the source, which is the costly one:
    many rays per texel against millions of triangles.
  - No image library in the core: the maps leave the C interface as arrays of numbers, the
    add-on makes Blender images of them; a format for the command line is to be chosen.
- **Done when**: on `ogre_hp`, the result shaded with its normal map is closer to the sculpt
  than with a bake by Blender at its default settings (mean angle between the two shadings,
  measured texel by texel), no texel is left unfilled, and no seam shows between islands.
- **Near it**: optimal normals per corner (milestone 13) and Multires levels with the detail
  reprojected (milestone 14) need the same correspondence.

### DT-052 — Preprocessing welded back what the mending took apart

- **Born on** 11 October 2026, from the bench: on `wolf` (a BlenderKit character of 14,797
  triangles, in eight pieces) the add-on gave 145 quads for 3000 asked and 360 for 8000, and
  the mesh was counted valid. The Hausdorff distance, 406 per mille of the diagonal, was the
  only figure that said the body was not there.
- **Cause**: the wolf has two edges with more than two faces, which the mending of the input
  takes apart, each face keeping the edge as a border of its own. The cleaning at the start
  of preprocessing then welded every pair of coincident border vertices, without the guard
  the mending has, and joined them again. The work mesh came out with 3 to 9 edges of more
  than two faces (more as it got finer), the cross field refused it, and the piece was
  dropped without a word: an error of one piece among several is only told when every piece
  fails. Below 2500 quads the coarser work mesh happened to lose those edges.
- **Closed the same day**: the cleaning is the mending itself (`TriMesh::stitched`), and
  nothing else. A piece that is not meshed says why in a note (`notes/not meshed`). The wolf
  gives 752, 2466 and 6901 faces at the three budgets, its body through the fallback chain
  for a border off the grid lines of the map, the blocker of the car (DT-047). The fallback
  chain run on a whole model now mends it too: it meshes the car, which it refused.
- **Left**:
  - A result that lacks most of its model is still counted valid, by the engine and by the
    bench alike: nothing compares the area meshed with the area asked for. The add-on said
    nothing of it either; it now warns of the pieces that could not be meshed and are missing.
  - The body of the wolf goes through the fallback chain, and takes its time there: 457
    seconds for 13,000 quads (DT-027).
