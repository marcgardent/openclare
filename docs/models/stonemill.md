# stonemill

An extra model; 14 942 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/stonemill__source.webp) | ![QuadriFlow (Blender)](img/stonemill__quadriflow.webp) | ![AutoRemesher](img/stonemill__autoremesher.webp) | ![openclare](img/stonemill__openclare.webp) |
| 14 942 triangles | 2 635 quads | 1 809 quads | 3 008 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 2 % | 5 % | 92 % | 0 % | 12 | 92 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 3 % | 2 % | 94 % | 0 % | 6 | 156 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1000 | **71** | 100 | 61 | 89 | 44 | 100 | 12 | 28 | 100 | 70 |
| **openclare** | 987 | **42** | 0 | 51 | 63 | 58 | 52 | 2 | 37 | 95 | 40 |
| quadriflow | 938 | **33** | 0 | 51 | 73 | 12 | 9 | 1 | 44 | 75 | 100 |
| autoremesher | 703 | **25** | 0 | 37 | 77 | 0 | 46 | — | — | 0 | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2976 | **80** | 100 | 64 | 100 | 60 | 100 | 31 | 56 | 97 | 62 |
| **openclare** | 3008 | **79** | 100 | 66 | 100 | 66 | 94 | 6 | 54 | 99 | 49 |
| quadriflow | 2635 | **50** | 40 | 60 | 92 | 16 | 72 | 1 | 44 | 51 | 76 |
| integer-grid | 2538 | **42** | 0 | 59 | 95 | 0 | 100 | — | — | 38 | 45 |
| instant-meshes | 2664 | **41** | 20 | 57 | 91 | 0 | 57 | 3 | — | 55 | 100 |
| autoremesher | 1809 | **34** | 0 | 53 | 81 | 0 | 95 | — | — | 0 | 100 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7948 | **85** | 100 | 76 | 100 | 63 | 100 | 40 | 75 | 97 | 43 |
| **openclare** | 8161 | **81** | 100 | 75 | 100 | 73 | 100 | 18 | 47 | 92 | 35 |
| quadriflow | 6712 | **56** | 40 | 63 | 92 | 58 | 99 | 3 | 35 | 36 | 53 |
| autoremesher | 4327 | **32** | 0 | 61 | 84 | 0 | 100 | — | 4 | 0 | 73 |
