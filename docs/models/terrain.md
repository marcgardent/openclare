# terrain

An extra model, with open borders; 543 557 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/terrain__source.webp) | ![QuadriFlow (Blender)](img/terrain__quadriflow.webp) | ![AutoRemesher](img/terrain__autoremesher.webp) | ![openclare](img/terrain__openclare.webp) | ![Quad Remesher](img/terrain__quadremesher.webp) |
| 543 557 triangles | 2 775 quads | 919 quads | 2 967 quads | 4 677 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/terrain__quadremesher.webp) | ![openclare, as many quads](img/terrain__openclare-matched.webp) |
| 4 677 quads | 4 619 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 0 % | 0 % | 53 % | 47 % | 0 | 46 |
| AutoRemesher | 0 % | 0 % | 68 % | 32 % | 0 | 27 |
| openclare | 0 % | 2 % | 81 % | 18 % | 1 | 108 |
| Quad Remesher | 0 % | 0 % | 85 % | 15 % | 0 | 105 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1000 | **66** | 100 | 22 | 70 | 54 | 62 | 35 | 100 | 48 |
| openclare-uniform | 980 | **66** | 100 | 20 | 71 | 53 | 65 | 49 | 92 | 48 |
| integer-grid | 896 | **58** | 100 | 20 | 50 | 41 | 62 | 64 | 58 | 47 |
| quadriflow | 872 | **57** | 100 | 14 | 66 | 40 | 50 | 65 | 49 | 27 |
| quadremesher | 1541 | **46** | 100 | 16 | 44 | 70 | 1 | 6 | 0 | 27 |
| instant-meshes | 574 | **42** | 100 | 16 | 57 | 5 | 30 | 31 | 0 | 47 |
| quadremesher-uniform | 878 | **22** | 0 | 11 | 46 | 42 | 0 | — | 51 | 25 |
| autoremesher | 308 | **0** | 0 | 0 | 0 | 0 | 3 | — | 0 | 85 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1512 | **62** | 100 | 18 | 72 | 49 | 72 | 32 | 92 | 47 |
| quadremesher-as-made | 1541 | **50** | 100 | 16 | 44 | 67 | 1 | 6 | 100 | 27 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2968 | **67** | 100 | 17 | 69 | 54 | 81 | 43 | 96 | 48 |
| **openclare** | 2967 | **66** | 100 | 17 | 67 | 61 | 67 | 32 | 96 | 46 |
| quadriflow | 2717 | **59** | 100 | 10 | 68 | 43 | 59 | 70 | 62 | 27 |
| integer-grid | 2502 | **56** | 100 | 17 | 53 | 40 | 78 | 60 | 34 | 35 |
| quadremesher-uniform | 2597 | **54** | 100 | 17 | 59 | 43 | 36 | 36 | 46 | 27 |
| instant-meshes | 1753 | **42** | 100 | 6 | 59 | 13 | 34 | 31 | 0 | 47 |
| quadremesher | 4677 | **29** | 0 | 19 | 52 | 73 | 51 | — | 0 | 25 |
| autoremesher | 919 | **20** | 0 | 10 | 67 | 15 | 51 | — | 0 | 76 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 4619 | **63** | 100 | 19 | 65 | 68 | 69 | 16 | 95 | 39 |
| quadremesher-as-made | 4677 | **31** | 0 | 18 | 52 | 69 | 51 | — | 100 | 25 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7941 | **69** | 100 | 21 | 66 | 65 | 84 | 31 | 97 | 41 |
| **openclare** | 8014 | **68** | 100 | 22 | 66 | 73 | 71 | 9 | 99 | 37 |
| quadriflow | 6963 | **60** | 100 | 7 | 71 | 49 | 73 | 70 | 48 | 23 |
| quadremesher-uniform | 7183 | **59** | 100 | 18 | 60 | 48 | 65 | 39 | 59 | 22 |
| integer-grid | 6773 | **57** | 100 | 21 | 49 | 46 | 82 | 37 | 39 | 7 |
| instant-meshes | 4807 | **37** | 80 | 5 | 63 | 8 | 43 | 25 | 0 | 44 |
| quadremesher | 11819 | **32** | 0 | 20 | 57 | 68 | 75 | 30 | 0 | 21 |
| autoremesher | 2975 | **22** | 0 | 7 | 64 | 26 | 64 | — | 0 | 64 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 11855 | **65** | 100 | 22 | 68 | 73 | 73 | 11 | 99 | 33 |
| quadremesher-as-made | 11819 | **34** | 0 | 19 | 57 | 62 | 75 | 30 | 100 | 21 |
