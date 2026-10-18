# rose

An organic model; 140 336 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/rose__source.webp) | ![QuadriFlow (Blender)](img/rose__quadriflow.webp) | ![AutoRemesher](img/rose__autoremesher.webp) | ![openclare](img/rose__openclare.webp) |
| 140 336 triangles | 2 208 quads | 636 quads | 3 624 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 3 % | 18 % | 80 % | 1 % | 82 | 38 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | — | — | — | — | — | — |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 511 | **17** | 0 | 46 | 46 | 2 | 0 | — | — | 0 | 56 |
| openclare-uniform | 1562 | **12** | 0 | 62 | 0 | 0 | 0 | — | 3 | 0 | 64 |
| quadriflow | 494 | **10** | 0 | 25 | 37 | 0 | 0 | 8 | 4 | 0 | 81 |
| instant-meshes | 530 | **10** | 0 | 27 | 29 | 0 | 0 | 6 | — | 0 | 88 |
| autoremesher | 157 | **6** | 0 | 29 | 0 | 0 | 0 | — | — | 0 | 100 |
| **openclare** | 1463 | **1** | 0 | — | 0 | 0 | 0 | — | 4 | 0 | 64 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3268 | **28** | 0 | 52 | 37 | 39 | 0 | — | 19 | 64 | 65 |
| **openclare** | 3624 | **24** | 0 | 55 | 37 | 32 | 0 | — | — | 17 | 43 |
| integer-grid | 1923 | **24** | 0 | 44 | 69 | 10 | 19 | — | — | 0 | 32 |
| quadriflow | 2085 | **15** | 0 | 29 | 50 | 11 | 0 | 10 | 9 | 0 | 81 |
| instant-meshes | 2226 | **14** | 0 | 24 | 55 | 4 | 0 | 12 | — | 0 | 82 |
| autoremesher | 636 | **5** | 0 | 21 | 0 | 0 | 0 | — | — | 0 | 96 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8422 | **67** | 100 | 69 | 70 | 73 | 27 | 26 | 32 | 79 | 24 |
| openclare-uniform | 8226 | **43** | 0 | 66 | 75 | 70 | 1 | — | 26 | 89 | 50 |
| integer-grid | 6584 | **38** | 0 | 58 | 87 | 13 | 63 | — | — | 29 | 0 |
| instant-meshes | 6932 | **22** | 0 | 28 | 76 | 2 | 0 | 7 | — | 47 | 86 |
| quadriflow | 6758 | **20** | 0 | 30 | 60 | 6 | 14 | 3 | 10 | 38 | 69 |
| autoremesher | 3828 | **13** | 0 | 29 | 42 | 0 | 5 | — | — | 0 | 85 |
