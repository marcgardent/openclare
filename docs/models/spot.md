# spot

An organic model; 100 250 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/spot__source.webp) | ![QuadriFlow (Blender)](img/spot__quadriflow.webp) | ![AutoRemesher](img/spot__autoremesher.webp) | ![openclare](img/spot__openclare.webp) | ![Quad Remesher](img/spot__quadremesher.webp) |
| 100 250 triangles | 2 964 quads | 1 284 quads | 3 008 quads | 3 478 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/spot__quadremesher.webp) | ![openclare, as many quads](img/spot__openclare-matched.webp) |
| 3 478 quads | 3 473 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 23 % | 77 % | 0 % | 10 | 299 |
| AutoRemesher | 10 % | 6 % | 84 % | 0 % | 3 | 90 |
| openclare | 32 % | 0 % | 68 % | 0 % | 0 | 239 |
| Quad Remesher | 15 % | 17 % | 68 % | 0 % | 7 | 156 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 985 | **90** | 100 | 98 | 100 | 69 | 89 | 45 | 89 | 94 | 70 |
| openclare-uniform | 1008 | **87** | 100 | 88 | 100 | 74 | 96 | 28 | 71 | 97 | 78 |
| quadremesher-uniform | 882 | **84** | 100 | 100 | 94 | 70 | 73 | 29 | — | 53 | 36 |
| quadremesher | 1497 | **81** | 100 | 100 | 99 | 79 | 100 | 28 | 67 | 0 | 30 |
| integer-grid | 910 | **79** | 100 | 82 | 81 | 57 | 90 | 24 | — | 64 | 63 |
| quadriflow | 1024 | **65** | 100 | 29 | 100 | 50 | 97 | 6 | 19 | 89 | 90 |
| instant-meshes | 1016 | **65** | 100 | 32 | 100 | 17 | 64 | 3 | — | 94 | 80 |
| autoremesher | 404 | **50** | 80 | 38 | 92 | 8 | 53 | — | 45 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1497 | **92** | 100 | 100 | 99 | 77 | 100 | 28 | — | 100 | 30 |
| openclare-matched | 1488 | **92** | 100 | 98 | 100 | 66 | 100 | 49 | 97 | 98 | 66 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3008 | **97** | 100 | 99 | 100 | 92 | 100 | 60 | 100 | 99 | 61 |
| openclare-uniform | 2976 | **95** | 100 | 93 | 100 | 91 | 100 | 57 | 99 | 97 | 65 |
| quadremesher-uniform | 2686 | **87** | 100 | 100 | 100 | 68 | 100 | 21 | — | 58 | 34 |
| integer-grid | 2659 | **85** | 100 | 91 | 91 | 76 | 100 | 25 | — | 55 | 54 |
| quadremesher | 3478 | **83** | 100 | 100 | 100 | 85 | 100 | 30 | 51 | 36 | 28 |
| instant-meshes | 3044 | **71** | 100 | 37 | 100 | 31 | 100 | 2 | — | 94 | 95 |
| autoremesher | 1284 | **69** | 100 | 72 | 95 | 33 | 100 | 20 | 65 | 0 | 100 |
| quadriflow | 2964 | **61** | 100 | 30 | 100 | 32 | 100 | 2 | 0 | 95 | 86 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3473 | **97** | 100 | 100 | 100 | 93 | 100 | 60 | 100 | 99 | 60 |
| quadremesher-as-made | 3478 | **94** | 100 | 100 | 100 | 87 | 100 | 30 | — | 100 | 28 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8003 | **98** | 100 | 100 | 100 | 95 | 100 | 67 | 98 | 100 | 41 |
| openclare-uniform | 8050 | **94** | 100 | 98 | 100 | 93 | 100 | 65 | 74 | 98 | 44 |
| quadremesher-uniform | 7274 | **91** | 100 | 97 | 100 | 88 | 100 | 47 | — | 64 | 30 |
| integer-grid | 7084 | **90** | 100 | 97 | 94 | 95 | 100 | 48 | — | 54 | 10 |
| quadremesher | 7965 | **90** | 100 | 100 | 100 | 91 | 100 | 42 | 50 | 98 | 30 |
| autoremesher | 2895 | **81** | 97 | 87 | 100 | 88 | 100 | — | 59 | 0 | 78 |
| instant-meshes | 8098 | **80** | 100 | 39 | 100 | 86 | 100 | 2 | — | 95 | 95 |
| quadriflow | 7874 | **66** | 80 | 39 | 100 | 89 | 100 | 1 | 4 | 94 | 68 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 7965 | **95** | 100 | 100 | 100 | 91 | 100 | 42 | — | 100 | 30 |
| openclare-matched | 7884 | **93** | 100 | 100 | 100 | 96 | 100 | 63 | 69 | 96 | 38 |
