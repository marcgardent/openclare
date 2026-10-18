# towermill

An extra model; 14 998 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/towermill__source.webp) | ![QuadriFlow (Blender)](img/towermill__quadriflow.webp) | ![AutoRemesher](img/towermill__autoremesher.webp) | ![openclare](img/towermill__openclare.webp) | ![Quad Remesher](img/towermill__quadremesher.webp) |
| 14 998 triangles | 2 755 quads | 1 088 quads | 2 304 quads | 16 107 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/towermill__quadremesher.webp) | ![openclare, as many quads](img/towermill__openclare-matched.webp) |
| 16 107 quads | 12 530 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 1 % | 11 % | 88 % | 0 % | 20 | 61 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | — | — | — | — | — | — |
| Quad Remesher | — | — | — | — | — | — |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1930 | **47** | 100 | 39 | 2 | 34 | 100 | 0 | 40 | 0 | 0 |
| quadremesher | 6083 | **45** | 0 | 88 | 88 | 64 | 67 | — | 16 | 0 | 0 |
| quadriflow | 866 | **31** | 0 | 63 | 62 | 27 | 0 | 2 | 33 | 46 | 100 |
| **openclare** | 3818 | **19** | 0 | 26 | 25 | 37 | 0 | 0 | 47 | 0 | 0 |
| autoremesher | 405 | **6** | 0 | 30 | 0 | 0 | 0 | — | — | 0 | 96 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3060 | **65** | 100 | 55 | 67 | 32 | 100 | 6 | 32 | 92 | 12 |
| quadremesher | 16107 | **52** | 0 | 82 | 94 | 61 | 100 | — | — | 0 | 0 |
| quadremesher-uniform | 2897 | **47** | 0 | 74 | 92 | 37 | 21 | — | — | 86 | 0 |
| integer-grid | 2395 | **43** | 0 | 67 | 89 | 22 | 89 | — | — | 19 | 38 |
| **openclare** | 2304 | **41** | 0 | 68 | 87 | 22 | 81 | — | — | 7 | 20 |
| quadriflow | 2724 | **36** | 0 | 64 | 77 | 34 | 11 | 2 | 26 | 63 | 72 |
| instant-meshes | 1904 | **16** | 0 | 43 | 33 | 16 | 0 | 0 | — | 0 | 100 |
| autoremesher | 1088 | **14** | 0 | 43 | 33 | 0 | 0 | — | — | 0 | 85 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 16107 | **54** | 0 | 74 | 94 | 47 | 100 | — | — | 100 | 0 |
| openclare-matched | 12530 | **53** | 0 | 72 | 100 | 36 | 100 | — | — | 11 | 0 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 35806 | **50** | 0 | 85 | 100 | 73 | 100 | — | 17 | 0 | 0 |
| openclare-uniform | 5861 | **45** | 0 | 74 | 95 | 21 | 100 | — | — | 0 | 0 |
| quadriflow | 7180 | **43** | 0 | 76 | 92 | 49 | 63 | 3 | 5 | 59 | 52 |
| autoremesher | 2664 | **24** | 0 | 56 | 39 | 10 | 40 | — | — | 0 | 65 |
| **openclare** | — | **0** | 0 | — | — | — | — | — | — | — | — |
