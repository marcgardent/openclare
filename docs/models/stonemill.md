# stonemill

An extra model; 14 942 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/stonemill__source.webp) | ![QuadriFlow (Blender)](img/stonemill__quadriflow.webp) | ![AutoRemesher](img/stonemill__autoremesher.webp) | ![openclare](img/stonemill__openclare.webp) | ![Quad Remesher](img/stonemill__quadremesher.webp) |
| 14 942 triangles | 2 635 quads | 1 809 quads | 3 008 quads | 14 140 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/stonemill__quadremesher.webp) | ![openclare, as many quads](img/stonemill__openclare-matched.webp) |
| 14 140 quads | 14 318 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 2 % | 5 % | 92 % | 0 % | 12 | 92 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 3 % | 2 % | 94 % | 0 % | 6 | 156 |
| Quad Remesher | 2 % | 11 % | 87 % | 0 % | 37 | 189 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1000 | **71** | 100 | 61 | 89 | 44 | 100 | 12 | 28 | 100 | 70 |
| quadremesher | 5437 | **47** | 0 | 79 | 91 | 67 | 81 | 9 | 43 | 0 | 16 |
| **openclare** | 987 | **42** | 0 | 51 | 63 | 58 | 52 | 2 | 37 | 95 | 40 |
| quadriflow | 938 | **33** | 0 | 51 | 73 | 12 | 9 | 1 | 44 | 75 | 100 |
| autoremesher | 703 | **25** | 0 | 37 | 77 | 0 | 46 | — | — | 0 | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2976 | **80** | 100 | 64 | 100 | 60 | 100 | 31 | 56 | 97 | 62 |
| **openclare** | 3008 | **79** | 100 | 66 | 100 | 66 | 94 | 6 | 54 | 99 | 49 |
| quadriflow | 2635 | **50** | 40 | 60 | 92 | 16 | 72 | 1 | 44 | 51 | 76 |
| quadremesher | 14140 | **48** | 0 | 76 | 100 | 60 | 95 | — | 27 | 0 | 11 |
| integer-grid | 2538 | **42** | 0 | 59 | 95 | 0 | 100 | — | — | 38 | 45 |
| quadremesher-uniform | 2859 | **42** | 0 | 64 | 97 | 15 | 21 | — | — | 81 | 14 |
| instant-meshes | 2664 | **41** | 20 | 57 | 91 | 0 | 57 | 3 | — | 55 | 100 |
| autoremesher | 1809 | **34** | 0 | 53 | 81 | 0 | 95 | — | — | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 14318 | **78** | 100 | 70 | 100 | 70 | 100 | 12 | 42 | 95 | 21 |
| quadremesher-as-made | 14140 | **50** | 0 | 67 | 100 | 29 | 95 | — | — | 100 | 11 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7948 | **85** | 100 | 76 | 100 | 63 | 100 | 40 | 75 | 97 | 43 |
| **openclare** | 8161 | **81** | 100 | 75 | 100 | 73 | 100 | 18 | 47 | 92 | 35 |
| quadriflow | 6712 | **56** | 40 | 63 | 92 | 58 | 99 | 3 | 35 | 36 | 53 |
| quadremesher | 33780 | **52** | 0 | 78 | 100 | 77 | 100 | 7 | 46 | 0 | 0 |
| autoremesher | 4327 | **32** | 0 | 61 | 84 | 0 | 100 | — | 4 | 0 | 73 |
