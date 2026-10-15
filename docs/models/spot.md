# spot

An organic model; 100 250 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/spot__source.webp) | ![QuadriFlow (Blender)](img/spot__quadriflow.webp) | ![AutoRemesher](img/spot__autoremesher.webp) | ![openclare](img/spot__openclare.webp) | ![Quad Remesher](img/spot__quadremesher.webp) |
| 100 250 triangles | 2 964 quads | 1 284 quads | 2 951 quads | 3 478 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/spot__quadremesher.webp) | ![openclare, as many quads](img/spot__openclare-matched.webp) |
| 3 478 quads | 3 436 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 0 % | 23 % | 77 % | 0 % | 10 | 299 |
| AutoRemesher | 10 % | 6 % | 84 % | 0 % | 3 | 90 |
| openclare | 22 % | 15 % | 63 % | 0 % | 5 | 176 |
| Quad Remesher | 15 % | 17 % | 68 % | 0 % | 7 | 156 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 990 | **72** | 100 | 58 | 64 | 74 | 43 | 22 | — | 96 | 85 |
| **openclare** | 986 | **69** | 100 | 67 | 65 | 69 | 39 | 24 | 29 | 94 | 82 |
| quadremesher-uniform | 882 | **66** | 100 | 68 | 55 | 70 | 23 | 29 | — | 53 | 36 |
| quadremesher | 1497 | **65** | 100 | 72 | 59 | 79 | 53 | 28 | 49 | 0 | 30 |
| integer-grid | 910 | **62** | 100 | 53 | 44 | 57 | 40 | 24 | — | 64 | 63 |
| quadriflow | 1024 | **56** | 100 | 19 | 62 | 50 | 47 | 6 | 16 | 89 | 90 |
| instant-meshes | 1016 | **52** | 100 | 21 | 63 | 17 | 14 | 3 | — | 94 | 80 |
| autoremesher | 404 | **18** | 0 | 25 | 54 | 8 | 3 | — | 39 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1514 | **74** | 100 | 65 | 69 | 68 | 60 | 36 | — | 95 | 75 |
| quadremesher-as-made | 1497 | **74** | 100 | 71 | 59 | 77 | 53 | 28 | — | 100 | 30 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2995 | **82** | 100 | 61 | 72 | 91 | 79 | 41 | — | 99 | 73 |
| **openclare** | 2951 | **78** | 100 | 65 | 74 | 91 | 79 | 44 | 41 | 93 | 65 |
| quadremesher-uniform | 2686 | **72** | 100 | 66 | 65 | 68 | 71 | 21 | — | 58 | 34 |
| quadremesher | 3478 | **71** | 100 | 72 | 68 | 85 | 79 | 30 | 29 | 36 | 28 |
| integer-grid | 2659 | **70** | 100 | 58 | 52 | 76 | 77 | 25 | — | 55 | 54 |
| instant-meshes | 3044 | **60** | 100 | 24 | 72 | 31 | 52 | 2 | — | 94 | 95 |
| quadriflow | 2964 | **55** | 100 | 19 | 64 | 32 | 80 | 2 | 0 | 95 | 86 |
| autoremesher | 1284 | **53** | 100 | 46 | 56 | 33 | 53 | 20 | 49 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3436 | **81** | 100 | 65 | 72 | 93 | 82 | 46 | — | 95 | 63 |
| quadremesher-as-made | 3478 | **80** | 100 | 72 | 68 | 87 | 79 | 30 | — | 100 | 28 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7967 | **83** | 100 | 65 | 68 | 96 | 91 | 33 | — | 98 | 41 |
| quadremesher | 7965 | **79** | 100 | 66 | 72 | 91 | 91 | 42 | 35 | 98 | 30 |
| quadremesher-uniform | 7274 | **79** | 100 | 63 | 70 | 88 | 90 | 47 | — | 64 | 30 |
| **openclare** | 8093 | **78** | 100 | 66 | 68 | 96 | 91 | 38 | 25 | 95 | 36 |
| integer-grid | 7084 | **77** | 100 | 62 | 55 | 95 | 90 | 48 | — | 54 | 10 |
| instant-meshes | 8098 | **71** | 100 | 25 | 78 | 86 | 66 | 2 | — | 95 | 95 |
| quadriflow | 7874 | **62** | 80 | 25 | 69 | 89 | 91 | 1 | 3 | 94 | 68 |
| autoremesher | 2895 | **43** | 0 | 56 | 62 | 88 | 74 | — | 37 | 0 | 78 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 8007 | **82** | 100 | 67 | 70 | 96 | 91 | 34 | — | 98 | 40 |
| quadremesher-as-made | 7965 | **82** | 100 | 66 | 72 | 91 | 91 | 42 | — | 100 | 30 |
