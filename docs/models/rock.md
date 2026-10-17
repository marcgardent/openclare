# rock

An organic model; 81 920 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/rock__source.webp) | ![QuadriFlow (Blender)](img/rock__quadriflow.webp) | ![AutoRemesher](img/rock__autoremesher.webp) | ![openclare](img/rock__openclare.webp) | ![Quad Remesher](img/rock__quadremesher.webp) |
| 81 920 triangles | 3 036 quads | 2 412 quads | 2 984 quads | 3 539 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/rock__quadremesher.webp) | ![openclare, as many quads](img/rock__openclare-matched.webp) |
| 3 539 quads | 3 589 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 0 % | 57 % | 43 % | 0 % | 5 | 528 |
| AutoRemesher | 0 % | 27 % | 73 % | 0 % | 4 | 317 |
| openclare | 0 % | 2 % | 98 % | 0 % | 1 | 222 |
| Quad Remesher | 0 % | 10 % | 90 % | 0 % | 6 | 145 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 978 | **71** | 100 | 24 | 71 | 82 | 82 | 22 | — | 91 | 78 |
| **openclare** | 981 | **65** | 100 | 25 | 75 | 85 | 82 | 0 | 0 | 92 | 87 |
| integer-grid | 898 | **64** | 100 | 28 | 53 | 78 | 80 | 0 | — | 59 | 59 |
| instant-meshes | 966 | **63** | 100 | 0 | 70 | 79 | 67 | 6 | — | 86 | 96 |
| quadriflow | 947 | **60** | 100 | 0 | 69 | 81 | 100 | 1 | 0 | 79 | 98 |
| autoremesher | 778 | **56** | 100 | 14 | 63 | 72 | 97 | 35 | 6 | 11 | 100 |
| quadremesher | 1290 | **54** | 100 | 40 | 45 | 80 | 36 | 0 | 41 | 0 | 19 |
| quadremesher-uniform | 837 | **54** | 100 | 36 | 40 | 75 | 9 | 0 | — | 35 | 32 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1298 | **69** | 100 | 26 | 72 | 85 | 80 | 0 | — | 98 | 87 |
| quadremesher-as-made | 1290 | **61** | 100 | 38 | 45 | 80 | 36 | 0 | — | 100 | 19 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2968 | **73** | 100 | 27 | 73 | 91 | 86 | 0 | — | 96 | 75 |
| **openclare** | 2984 | **71** | 100 | 30 | 75 | 91 | 81 | 0 | 44 | 98 | 74 |
| integer-grid | 2687 | **67** | 100 | 30 | 56 | 91 | 84 | 0 | — | 58 | 53 |
| instant-meshes | 2898 | **65** | 100 | 0 | 74 | 89 | 71 | 0 | — | 86 | 99 |
| quadremesher-uniform | 2531 | **62** | 100 | 35 | 47 | 88 | 63 | 0 | — | 37 | 31 |
| quadriflow | 3036 | **62** | 100 | 0 | 71 | 89 | 100 | 6 | 0 | 91 | 95 |
| quadremesher | 3539 | **60** | 100 | 37 | 50 | 90 | 70 | 0 | 24 | 28 | 26 |
| autoremesher | 2412 | **56** | 100 | 17 | 63 | 85 | 95 | 0 | 0 | 22 | 85 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3589 | **71** | 100 | 31 | 73 | 91 | 81 | 0 | — | 94 | 71 |
| quadremesher-as-made | 3539 | **67** | 100 | 35 | 50 | 90 | 70 | 0 | — | 100 | 26 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7943 | **73** | 100 | 26 | 70 | 94 | 86 | 0 | — | 97 | 57 |
| **openclare** | 7940 | **71** | 100 | 27 | 73 | 95 | 84 | 0 | 40 | 97 | 52 |
| integer-grid | 7393 | **68** | 100 | 29 | 54 | 93 | 86 | 0 | — | 70 | 14 |
| instant-meshes | 7792 | **68** | 100 | 0 | 79 | 94 | 80 | 0 | — | 90 | 91 |
| quadremesher-uniform | 6767 | **64** | 100 | 23 | 51 | 91 | 91 | 0 | — | 38 | 29 |
| quadremesher | 7071 | **62** | 100 | 23 | 51 | 92 | 92 | 0 | 29 | 54 | 29 |
| quadriflow | 7342 | **59** | 100 | 1 | 69 | 86 | 100 | 0 | 0 | 67 | 65 |
| autoremesher | 5842 | **57** | 100 | 17 | 65 | 92 | 96 | 14 | 6 | 0 | 71 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 7061 | **71** | 100 | 28 | 73 | 93 | 85 | 0 | — | 99 | 54 |
| quadremesher-as-made | 7071 | **67** | 100 | 26 | 51 | 92 | 92 | 0 | — | 100 | 29 |
