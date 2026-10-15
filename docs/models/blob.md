# blob

An organic model; 124 004 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/blob__source.webp) | ![QuadriFlow (Blender)](img/blob__quadriflow.webp) | ![AutoRemesher](img/blob__autoremesher.webp) | ![openclare](img/blob__openclare.webp) | ![Quad Remesher](img/blob__quadremesher.webp) |
| 124 004 triangles | 2 796 quads | 1 165 quads | 3 005 quads | 3 370 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/blob__quadremesher.webp) | ![openclare, as many quads](img/blob__openclare-matched.webp) |
| 3 370 quads | 3 392 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 0 % | 3 % | 97 % | 0 % | 1 | 410 |
| AutoRemesher | 17 % | 0 % | 83 % | 0 % | 0 | 222 |
| openclare | 17 % | 22 % | 61 % | 0 % | 3 | 560 |
| Quad Remesher | 23 % | 0 % | 77 % | 0 % | 0 | 655 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 990 | **84** | 100 | 80 | 71 | 87 | 84 | 20 | 96 | 77 |
| openclare-uniform | 1026 | **82** | 100 | 77 | 68 | 79 | 84 | 35 | 90 | 75 |
| quadremesher | 1362 | **76** | 100 | 85 | 66 | 85 | 87 | 62 | 0 | 36 |
| quadremesher-uniform | 878 | **76** | 100 | 78 | 68 | 62 | 79 | 51 | 51 | 37 |
| integer-grid | 867 | **70** | 100 | 67 | 52 | 65 | 82 | 31 | 47 | 65 |
| instant-meshes | 1036 | **61** | 100 | 21 | 62 | 70 | 34 | 0 | 86 | 87 |
| autoremesher | 400 | **60** | 100 | 61 | 38 | 66 | 60 | 21 | 0 | 100 |
| quadriflow | 941 | **57** | 80 | 20 | 54 | 71 | 70 | 0 | 76 | 84 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1362 | **85** | 100 | 85 | 66 | 85 | 87 | 62 | 100 | 36 |
| openclare-matched | 1340 | **83** | 100 | 78 | 71 | 91 | 88 | 14 | 94 | 85 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3005 | **87** | 100 | 78 | 72 | 95 | 95 | 32 | 99 | 74 |
| openclare-uniform | 3021 | **85** | 100 | 76 | 69 | 94 | 95 | 22 | 97 | 67 |
| quadremesher-uniform | 2755 | **83** | 100 | 80 | 73 | 85 | 94 | 51 | 67 | 39 |
| quadremesher | 3370 | **83** | 100 | 83 | 71 | 92 | 95 | 46 | 51 | 32 |
| integer-grid | 2507 | **78** | 100 | 72 | 61 | 90 | 93 | 62 | 34 | 66 |
| instant-meshes | 3090 | **68** | 100 | 30 | 71 | 80 | 56 | 1 | 88 | 87 |
| quadriflow | 2781 | **57** | 60 | 22 | 56 | 83 | 89 | 3 | 71 | 83 |
| autoremesher | 1165 | **45** | 0 | 72 | 53 | 87 | 78 | — | 0 | 98 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 3370 | **86** | 100 | 82 | 71 | 92 | 95 | 46 | 100 | 32 |
| openclare-matched | 3392 | **85** | 100 | 78 | 72 | 94 | 95 | 33 | 97 | 73 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 7911 | **88** | 100 | 82 | 72 | 94 | 98 | 42 | 96 | 39 |
| **openclare** | 7979 | **86** | 100 | 76 | 71 | 96 | 98 | 15 | 99 | 51 |
| openclare-uniform | 8068 | **85** | 100 | 74 | 69 | 96 | 98 | 16 | 97 | 55 |
| quadremesher-uniform | 7035 | **84** | 100 | 80 | 72 | 93 | 98 | 59 | 52 | 38 |
| integer-grid | 7124 | **80** | 100 | 74 | 63 | 95 | 98 | 21 | 56 | 42 |
| autoremesher | 2795 | **74** | 100 | 74 | 54 | 94 | 93 | 58 | 0 | 76 |
| instant-meshes | 8164 | **72** | 100 | 33 | 76 | 90 | 67 | 0 | 92 | 85 |
| quadriflow | 7590 | **62** | 60 | 34 | 61 | 90 | 96 | 2 | 80 | 65 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 7911 | **87** | 100 | 82 | 72 | 94 | 98 | 42 | 100 | 39 |
| openclare-matched | 7949 | **85** | 100 | 77 | 72 | 97 | 98 | 15 | 98 | 52 |
