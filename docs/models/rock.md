# rock

An organic model; 81 920 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/rock__source.webp) | ![QuadriFlow (Blender)](img/rock__quadriflow.webp) | ![AutoRemesher](img/rock__autoremesher.webp) | ![openclare](img/rock__openclare.webp) | ![Quad Remesher](img/rock__quadremesher.webp) |
| 81 920 triangles | 3 036 quads | 2 412 quads | 3 010 quads | 3 539 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/rock__quadremesher.webp) | ![openclare, as many quads](img/rock__openclare-matched.webp) |
| 3 539 quads | 3 506 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 57 % | 43 % | 0 % | 5 | 528 |
| AutoRemesher | 0 % | 27 % | 73 % | 0 % | 4 | 317 |
| openclare | 0 % | 9 % | 91 % | 0 % | 3 | 207 |
| Quad Remesher | 0 % | 10 % | 90 % | 0 % | 6 | 145 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 898 | **75** | 100 | 43 | 92 | 78 | 100 | 0 | — | 59 | 59 |
| instant-meshes | 966 | **71** | 100 | 0 | 100 | 79 | 100 | 6 | — | 86 | 96 |
| **openclare** | 983 | **70** | 100 | 41 | 100 | 85 | 100 | 0 | 0 | 93 | 94 |
| openclare-uniform | 1012 | **70** | 100 | 38 | 100 | 84 | 100 | 0 | 2 | 95 | 87 |
| quadremesher-uniform | 837 | **68** | 100 | 56 | 77 | 75 | 59 | 0 | — | 35 | 32 |
| quadremesher | 1290 | **67** | 100 | 62 | 82 | 80 | 86 | 0 | 45 | 0 | 19 |
| quadriflow | 947 | **61** | 100 | 0 | 100 | 81 | 100 | 1 | 0 | 79 | 98 |
| autoremesher | 778 | **61** | 100 | 22 | 100 | 72 | 100 | 35 | 11 | 11 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1290 | **77** | 100 | 59 | 82 | 80 | 86 | 0 | — | 100 | 19 |
| openclare-matched | 1291 | **76** | 100 | 41 | 100 | 88 | 100 | 5 | 53 | 100 | 81 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 2687 | **78** | 100 | 46 | 95 | 91 | 100 | 0 | — | 58 | 53 |
| **openclare** | 3010 | **76** | 100 | 47 | 100 | 91 | 100 | 0 | 30 | 99 | 78 |
| quadremesher-uniform | 2531 | **75** | 100 | 54 | 85 | 88 | 100 | 0 | — | 37 | 31 |
| openclare-uniform | 3039 | **74** | 100 | 43 | 100 | 91 | 100 | 0 | 25 | 95 | 78 |
| instant-meshes | 2898 | **72** | 100 | 0 | 100 | 89 | 100 | 0 | — | 86 | 99 |
| quadremesher | 3539 | **70** | 100 | 57 | 88 | 90 | 100 | 0 | 27 | 28 | 26 |
| quadriflow | 3036 | **63** | 100 | 0 | 100 | 89 | 100 | 6 | 0 | 91 | 95 |
| autoremesher | 2412 | **61** | 100 | 26 | 100 | 85 | 100 | 0 | 0 | 22 | 85 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 3539 | **81** | 100 | 54 | 88 | 90 | 100 | 0 | — | 100 | 26 |
| openclare-matched | 3506 | **76** | 100 | 45 | 100 | 90 | 100 | 0 | 50 | 96 | 72 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 7393 | **79** | 100 | 45 | 93 | 93 | 100 | 0 | — | 70 | 14 |
| **openclare** | 8009 | **78** | 100 | 42 | 100 | 94 | 100 | 0 | 50 | 100 | 50 |
| openclare-uniform | 7913 | **78** | 100 | 41 | 100 | 94 | 100 | 0 | 50 | 96 | 52 |
| quadremesher-uniform | 6767 | **73** | 100 | 36 | 89 | 91 | 100 | 0 | — | 38 | 29 |
| instant-meshes | 7792 | **73** | 100 | 0 | 100 | 94 | 100 | 0 | — | 90 | 91 |
| quadremesher | 7071 | **69** | 100 | 36 | 89 | 92 | 100 | 0 | 33 | 54 | 29 |
| autoremesher | 5842 | **62** | 100 | 27 | 100 | 92 | 100 | 14 | 12 | 0 | 71 |
| quadriflow | 7342 | **61** | 100 | 1 | 100 | 86 | 100 | 0 | 0 | 67 | 65 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 7071 | **78** | 100 | 40 | 89 | 92 | 100 | 0 | — | 100 | 29 |
| openclare-matched | 6961 | **76** | 100 | 44 | 100 | 94 | 100 | 0 | 50 | 94 | 50 |
