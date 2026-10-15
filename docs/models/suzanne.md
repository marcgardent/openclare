# suzanne

An organic model, with open borders; 15 744 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/suzanne__source.webp) | ![QuadriFlow (Blender)](img/suzanne__quadriflow.webp) | ![AutoRemesher](img/suzanne__autoremesher.webp) | ![openclare](img/suzanne__openclare.webp) | ![Quad Remesher](img/suzanne__quadremesher.webp) |
| 15 744 triangles | 3 012 quads | 1 255 quads | 2 986 quads | 4 310 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/suzanne__quadremesher.webp) | ![openclare, as many quads](img/suzanne__openclare-matched.webp) |
| 4 310 quads | 4 306 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 2 % | 15 % | 80 % | 0 % | 12 | 233 |
| AutoRemesher | 9 % | 15 % | 77 % | 0 % | 8 | 82 |
| openclare | 10 % | 21 % | 61 % | 8 % | 7 | 126 |
| Quad Remesher | 16 % | 12 % | 65 % | 7 % | 7 | 207 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1002 | **75** | 100 | 49 | 76 | 73 | 56 | 40 | — | 99 | 95 |
| **openclare** | 994 | **73** | 100 | 54 | 72 | 82 | 52 | 44 | 43 | 98 | 88 |
| quadremesher-uniform | 922 | **66** | 100 | 57 | 57 | 70 | 26 | 13 | — | 69 | 68 |
| quadremesher | 1790 | **61** | 100 | 62 | 56 | 84 | 62 | 14 | 17 | 0 | 63 |
| integer-grid | 976 | **41** | 0 | 48 | 55 | 51 | 48 | 25 | — | 90 | 56 |
| quadriflow | 1005 | **33** | 0 | 30 | 52 | 45 | 33 | 1 | 33 | 96 | 100 |
| instant-meshes | 958 | **24** | 0 | 21 | 49 | 26 | 0 | 3 | — | 83 | 100 |
| autoremesher | 357 | **12** | 0 | 19 | 21 | 0 | 33 | 0 | 36 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1771 | **78** | 100 | 53 | 76 | 87 | 72 | 52 | — | 96 | 92 |
| quadremesher-as-made | 1790 | **72** | 100 | 60 | 56 | 83 | 62 | 14 | — | 100 | 63 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2980 | **79** | 100 | 54 | 78 | 82 | 82 | 17 | — | 97 | 82 |
| **openclare** | 2986 | **74** | 100 | 56 | 78 | 85 | 81 | 33 | 14 | 98 | 79 |
| integer-grid | 2757 | **73** | 100 | 54 | 64 | 75 | 81 | 33 | — | 68 | 48 |
| quadremesher-uniform | 2628 | **70** | 100 | 54 | 61 | 73 | 73 | 30 | — | 50 | 72 |
| instant-meshes | 3070 | **46** | 40 | 24 | 65 | 58 | 31 | 2 | — | 91 | 100 |
| quadremesher | 4310 | **44** | 0 | 61 | 61 | 88 | 81 | 42 | 35 | 0 | 70 |
| quadriflow | 3030 | **37** | 0 | 32 | 57 | 41 | 82 | 5 | 16 | 96 | 71 |
| autoremesher | 1255 | **26** | 0 | 41 | 44 | 38 | 45 | — | 21 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 4306 | **81** | 100 | 56 | 78 | 89 | 87 | 56 | — | 100 | 59 |
| quadremesher-as-made | 4310 | **49** | 0 | 60 | 61 | 86 | 81 | 42 | — | 100 | 70 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 8040 | **83** | 100 | 56 | 73 | 91 | 92 | 56 | — | 98 | 56 |
| quadremesher-uniform | 7317 | **78** | 100 | 56 | 65 | 90 | 90 | 54 | — | 66 | 61 |
| integer-grid | 7376 | **77** | 100 | 56 | 62 | 90 | 91 | 42 | — | 69 | 0 |
| quadremesher | 9246 | **75** | 100 | 62 | 65 | 93 | 92 | 56 | 59 | 38 | 50 |
| **openclare** | 8103 | **75** | 100 | 56 | 70 | 94 | 90 | 41 | 10 | 95 | 45 |
| quadriflow | 7983 | **71** | 100 | 37 | 70 | 86 | 93 | 22 | 26 | 97 | 53 |
| instant-meshes | 8098 | **70** | 100 | 26 | 74 | 88 | 57 | 3 | — | 95 | 100 |
| autoremesher | 2647 | **29** | 0 | 46 | 53 | 36 | 68 | — | 9 | 0 | 96 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 9246 | **81** | 100 | 62 | 65 | 93 | 92 | 56 | — | 100 | 50 |
| openclare-matched | 9380 | **80** | 100 | 58 | 70 | 95 | 91 | 36 | — | 94 | 50 |
