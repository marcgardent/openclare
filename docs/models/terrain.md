# terrain

An extra model, with open borders; 543 557 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/terrain__source.webp) | ![QuadriFlow (Blender)](img/terrain__quadriflow.webp) | ![AutoRemesher](img/terrain__autoremesher.webp) | ![openclare](img/terrain__openclare.webp) | ![Quad Remesher](img/terrain__quadremesher.webp) |
| 543 557 triangles | 2 775 quads | 919 quads | 3 032 quads | 4 677 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/terrain__quadremesher.webp) | ![openclare, as many quads](img/terrain__openclare-matched.webp) |
| 4 677 quads | 4 676 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 0 % | 53 % | 47 % | 0 | 46 |
| AutoRemesher | 0 % | 0 % | 68 % | 32 % | 0 | 27 |
| openclare | 0 % | 2 % | 83 % | 15 % | 1 | 110 |
| Quad Remesher | 0 % | 0 % | 85 % | 15 % | 0 | 105 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1004 | **73** | 100 | 31 | 100 | 50 | 100 | 54 | 50 | 98 | 50 |
| **openclare** | 1011 | **71** | 100 | 27 | 100 | 51 | 100 | 41 | 49 | 96 | 48 |
| integer-grid | 896 | **70** | 100 | 32 | 88 | 41 | 100 | 64 | — | 58 | 47 |
| quadriflow | 872 | **66** | 100 | 23 | 100 | 40 | 100 | 65 | 50 | 49 | 27 |
| quadremesher | 1541 | **57** | 100 | 25 | 81 | 70 | 51 | 6 | 50 | 0 | 27 |
| quadremesher-uniform | 878 | **56** | 82 | 17 | 84 | 42 | 50 | — | — | 51 | 25 |
| instant-meshes | 574 | **55** | 100 | 24 | 96 | 5 | 80 | 31 | — | 0 | 47 |
| autoremesher | 308 | **10** | 0 | 1 | 0 | 0 | 53 | — | 34 | 0 | 87 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1563 | **68** | 100 | 27 | 100 | 52 | 100 | 42 | 40 | 94 | 47 |
| quadremesher-as-made | 1541 | **64** | 100 | 25 | 81 | 67 | 51 | 6 | — | 100 | 27 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3032 | **73** | 100 | 28 | 100 | 66 | 100 | 26 | 50 | 96 | 46 |
| openclare-uniform | 3029 | **73** | 100 | 30 | 100 | 54 | 100 | 51 | 50 | 96 | 44 |
| quadriflow | 2717 | **67** | 100 | 16 | 100 | 43 | 100 | 70 | 50 | 62 | 27 |
| quadremesher-uniform | 2597 | **67** | 100 | 27 | 99 | 43 | 86 | 36 | — | 46 | 27 |
| integer-grid | 2502 | **66** | 100 | 26 | 92 | 40 | 100 | 60 | — | 34 | 35 |
| quadremesher | 4677 | **66** | 98 | 29 | 91 | 73 | 100 | — | 50 | 0 | 25 |
| autoremesher | 919 | **54** | 85 | 15 | 100 | 15 | 100 | — | 50 | 0 | 84 |
| instant-meshes | 1753 | **54** | 100 | 9 | 99 | 13 | 84 | 31 | — | 0 | 47 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 4677 | **75** | 98 | 28 | 91 | 69 | 100 | — | — | 100 | 25 |
| openclare-matched | 4676 | **71** | 100 | 30 | 100 | 69 | 100 | 17 | 51 | 100 | 37 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8099 | **74** | 100 | 35 | 100 | 74 | 100 | 11 | 51 | 95 | 36 |
| openclare-uniform | 7903 | **74** | 100 | 33 | 100 | 66 | 100 | 31 | 51 | 95 | 40 |
| quadremesher-uniform | 7183 | **71** | 100 | 28 | 100 | 48 | 100 | 39 | — | 59 | 22 |
| integer-grid | 6773 | **67** | 100 | 33 | 87 | 46 | 100 | 37 | — | 39 | 7 |
| quadriflow | 6963 | **65** | 100 | 10 | 100 | 49 | 100 | 70 | 50 | 48 | 28 |
| autoremesher | 2975 | **55** | 88 | 11 | 100 | 26 | 100 | — | 50 | 0 | 65 |
| instant-meshes | 4807 | **48** | 80 | 7 | 100 | 8 | 93 | 25 | — | 0 | 44 |
| quadremesher | 11819 | **43** | 0 | 30 | 96 | 68 | 100 | 30 | 50 | 0 | 21 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 11815 | **73** | 100 | 35 | 100 | 73 | 100 | 15 | 51 | 100 | 27 |
| quadremesher-as-made | 11819 | **45** | 0 | 29 | 96 | 62 | 100 | 30 | — | 100 | 21 |
