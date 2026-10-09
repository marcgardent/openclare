# The reference chain (Instant Meshes)

> This document describes the chain that was **written first**, kept as the floor the others
> are measured against. What the project aims at is described in
> [ALGORITHM.md](../ALGORITHM.md), which replaces the roadmap at the bottom of this page.

## The problem

From any triangulated surface (a sculpt, a scan, CAD), make a mesh of quads:

1. whose edges **follow the shape** (curvature, hard edges, borders);
2. of a **size that is asked for** (a target number of faces);
3. with **as few irregular vertices as possible** (valence other than 4), placed where the
   shape calls for them;
4. that **stays on the original surface**.

Every serious method splits this into two questions: *which way do the edges go?* then *where
do they pass?* This is the "field-aligned" family, and the backbone of this project.

```
triangles ─► cleaning ─► hierarchy ─► orientation ─► position ─► extraction ─► relaxation ─► quads
             subdivision              field          field
```

## 1. Orientation field (`field::orientation`)

At each vertex a **cross** is sought: four tangent directions at 90 degrees from one another,
with nothing to tell them apart (4-RoSy symmetry). It is stored as a single unit tangent vector
`q`; the three other arms are its rotations about the normal.

The field must be **smooth**: two neighbouring vertices should carry nearly the same cross.
The difficulty is the symmetry: to compare two crosses, one must first choose which arms go
together. `rosy::compat_orientation` takes the pair of arms best aligned *in 3D* (the
"extrinsic" energy), which has a precious side effect: the field aligns by itself on folds and
on curvature directions, without their being computed.

It is solved by Gauss-Seidel sweeps: each cross is replaced by the mean of its neighbours
(each brought to the compatible arm), projected back into the tangent plane and normalized.

Where the field cannot be smooth, **singularities** appear: going round a triangle, the cross
has turned by a quarter turn. These are the future vertices of valence 3 (index +¼) and 5
(index −¼). Their sum is set by the topology (Poincaré-Hopf):
`#valence3 − #valence5 = 4 χ`, that is 8 for a sphere and 0 for a torus. The test
`orientation_field_obeys_poincare_hopf` checks it.

## 2. Multi-resolution hierarchy (`hierarchy`)

Local smoothing only carries information one neighbourhood per pass, and freezes in poor local
minima, with dozens of needless singularities. So a pyramid is built: at each level,
neighbouring vertices are merged two by two (the closest normals first) until one is left. The
coarsest level is solved, its solution copied to the finer one, refined, and so on. As a
result, a sphere gets its 8 singularities, the minimum.

## 3. Position field (`field::position`)

With the cross and an edge length `s`, each vertex carries a **square grid** of step `s` in its
tangent plane, represented by one of its points `o`. Smoothing this field means making
neighbouring grids coincide so that they describe one global grid, whose nodes will be the
vertices of the quads.

The principle is the same as for orientation, with a symmetry by whole translations: to
compare two grids, `rosy::compat_position` looks for the two nodes closest to one another
around the middle of the edge (`middle_point`, the point both close to the two vertices and in
their two tangent planes, to cancel curvature out). After each averaging, `o` is brought back
to the node closest to its vertex.

## 4. Constraints (`field::constraints`)

Open borders and hard edges (dihedral angle above a threshold) impose:

- a **direction**: the cross has an arm along the line;
- a **point**: the grid passes through the line, while staying free to slide along it;
- at **corners** (a line that turns sharply, or more than two lines that meet): a node of the
  grid is pinned on the corner.

The constraints are carried up to the coarse levels of the hierarchy.

## 5. Extraction (`extract`)

For each edge of the input mesh, the whole offset between the grid nodes of its two ends is
computed (`rosy::compat_position_index`):

- `(0, 0)`: the same node, the two vertices are **merged** (union-find, the most reliable
  matches first, never merging two groups already joined by an edge);
- `(±1, 0)` or `(0, ±1)`: adjacent nodes, this is an **edge** of the quad mesh;
- anything else (a diagonal, a jump): ignored.

This gives a graph embedded in the surface. After cleaning (stray vertices, T-junctions,
dangling edges), the neighbours of each vertex are sorted counter-clockwise about the normal.
"Arrive by `u`, leave by the neighbour before `u`" is then a permutation of the directed edges
whose cycles are exactly the faces. Cycles that pass twice through the same vertex are cut at
their pinch points.

The result is **quad-dominant**: quads, plus triangles and pentagons where the grid has
defects. For **pure quads**, the problem is solved at scale `2s` and each face is then cut
around its centre (an n-gon gives n quads).

## 6. Relaxation (`relax`)

Laplacian smoothing followed by a projection: free vertices onto the source surface (BVH,
`spatial`), vertices close to a feature line onto that line, corners stay put. This is what
makes hard edges crisp in the result.

## Known limits

- **Position singularities.** The position field is solved locally: nothing guarantees that it
  is globally consistent. Where it is not, triangle and pentagon pairs appear, hence pairs of
  valence 3 and valence 5 vertices that serve no purpose (3 to 6 % of the vertices today). This
  is the structural limit of Instant Meshes and the main gap with Quad Remesher.
- **Hard edges.** Normals are averaged across a fold: the field is less reliable there than
  elsewhere, and a defect sometimes stays along an edge.
- **Uniform density.** No size adapted to curvature.
- **Input.** Assumed to be roughly manifold and consistently oriented; no welding of duplicate
  vertices, no repair.
- **Holes.** On open or very noisy meshes, a loop of more than 8 edges stays open (the report
  counts them: `open_loops`).

## Roadmap

As it was written with this chain, by impact on quality. It is kept for the record: the plan
that replaced it is in [ALGORITHM.md](../ALGORITHM.md), and what is left to do today in
[ROADMAP.md](../ROADMAP.md).

1. **Global parametrization with integers**, to remove position singularities. Two ways:
   - *QuadriFlow*: keep our two fields and make the whole offsets consistent by a minimum-cost
     flow. The shortest path from the current code.
   - *QuadWild*: trace the separatrices of the cross field to cut the surface into patches,
     then choose the number of quads per side (quantization by integer program or BiMDF).
     Heavier, but this is what gives the clean edge loops expected in modelling.
2. **A guided cross field**: principal curvature directions as soft constraints, strokes drawn
   by the user, an intrinsic solver (Knöppel et al.) as an alternative, merging of close pairs
   of singularities.
3. **Clean hard edges**: normals kept apart on each side of a fold, smoothing groups, UV seams
   and material borders as feature lines.
4. **Adaptive density**: a sizing field driven by curvature or painted.
5. **Symmetry**: detection of a plane of symmetry and a symmetric result.
6. **Robustness**: welding, non-manifold input, tiny components, hole filling.
7. **Speed**: parallel Gauss-Seidel by graph colouring (rayon), SIMD.
8. **Integration**: Blender add-on, C API, PLY and glTF formats.
9. **Measurement**: a bench comparing the number of singularities, the distortion of the quads
   and the Hausdorff distance with Instant Meshes, QuadriFlow and QuadWild.

## References

The papers are freely available on their authors' pages.

| Paper | What is taken from it | Reference code (licence) |
|---|---|---|
| Jakob, Tarini, Panozzo, Sorkine-Hornung, *Instant Field-Aligned Meshes*, SIGGRAPH Asia 2015 | stages 1 to 5, implemented here | `wjakob/instant-meshes` (BSD-3) |
| Huang, Zhou, Niessner, Shewchuk, Guibas, *QuadriFlow*, SGP 2018 | global consistency by a flow | `hjwdzh/QuadriFlow` (MIT) |
| Pietroni, Nuvoli, Alderighi, Cignoni, Tarini, *Reliable Feature-Line Driven Quad-Remeshing*, SIGGRAPH 2021 | patches driven by hard edges | `nicopietroni/quadwild` (GPL-3) |
| Heistermann, Warnett, Bommes, *Min-Deviation-Flow in Bi-directed Graphs for T-Mesh Quantization*, SIGGRAPH 2023 | quantization without a commercial solver | `cgg-bern/quadwild-bimdf` (GPL-3) |
| Bommes, Zimmer, Kobbelt, *Mixed-Integer Quadrangulation*, SIGGRAPH 2009 | the reference formulation | CoMISo (GPL) |
| Bommes, Campen, Ebke, Alliez, Kobbelt, *Integer-Grid Maps for Reliable Quad Meshing*, SIGGRAPH 2013 | guarantees of validity | — |
| Ebke, Bommes, Campen, Kobbelt, *QEx: Robust Quad Mesh Extraction*, SIGGRAPH Asia 2013 | robust extraction | `libQEx` (GPL-3) |
| Campen, Bommes, Kobbelt, *Quantized Global Parametrization*, SIGGRAPH Asia 2015 | robust quantization | — |
| Knöppel, Crane, Pinkall, Schröder, *Globally Optimal Direction Fields*, SIGGRAPH 2013 | cross field by eigenvalues | Directional (MPL-2) |
| Bommes et al., *Quad-Mesh Generation and Processing: A Survey*, 2013 | overview | — |
| Vaxman et al., *Directional Field Synthesis, Design, and Processing*, 2016 | overview of fields | — |

The code of this chain is written from the Instant Meshes paper. The project is under
GPL-3.0-or-later; how existing code may be reused is set out in
[ALGORITHM.md](../ALGORITHM.md).
