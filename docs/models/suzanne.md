# suzanne

An organic model, with open borders; 15 744 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/suzanne__source.webp) | ![QuadriFlow (Blender)](img/suzanne__quadriflow.webp) | ![AutoRemesher](img/suzanne__autoremesher.webp) | ![openclare](img/suzanne__openclare.webp) | ![Quad Remesher](img/suzanne__quadremesher.webp) |
| 15 744 triangles | 3 012 quads | 1 255 quads | 2 981 quads | 4 310 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/suzanne__quadremesher.webp) | ![openclare, as many quads](img/suzanne__openclare-matched.webp) |
| 4 310 quads | 4 311 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 2 % | 15 % | 80 % | 0 % | 12 | 233 |
| AutoRemesher | 9 % | 15 % | 77 % | 0 % | 8 | 82 |
| openclare | 19 % | 16 % | 55 % | 11 % | 6 | 119 |
| Quad Remesher | 16 % | 12 % | 65 % | 7 % | 7 | 207 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1010 | **87** | 100 | 75 | 100 | 72 | 100 | 41 | 86 | 96 | 85 |
| **openclare** | 994 | **87** | 100 | 84 | 100 | 82 | 100 | 44 | 59 | 98 | 93 |
| quadremesher-uniform | 922 | **83** | 100 | 89 | 96 | 70 | 76 | 13 | — | 69 | 68 |
| quadremesher | 1790 | **74** | 100 | 96 | 96 | 84 | 100 | 14 | 23 | 0 | 63 |
| integer-grid | 976 | **57** | 0 | 75 | 94 | 51 | 98 | 25 | — | 90 | 56 |
| quadriflow | 1005 | **46** | 0 | 47 | 90 | 45 | 83 | 1 | 38 | 96 | 100 |
| instant-meshes | 958 | **37** | 0 | 32 | 87 | 26 | 48 | 3 | — | 83 | 100 |
| autoremesher | 357 | **24** | 0 | 29 | 54 | 0 | 83 | 0 | 40 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1790 | **90** | 100 | 94 | 96 | 83 | 100 | 14 | — | 100 | 63 |
| openclare-matched | 1771 | **89** | 100 | 82 | 100 | 87 | 100 | 52 | 78 | 96 | 87 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3025 | **87** | 100 | 84 | 100 | 84 | 100 | 40 | 58 | 97 | 84 |
| **openclare** | 2981 | **86** | 100 | 87 | 100 | 86 | 100 | 51 | 45 | 97 | 76 |
| integer-grid | 2757 | **86** | 100 | 83 | 100 | 75 | 100 | 33 | — | 68 | 48 |
| quadremesher-uniform | 2628 | **84** | 100 | 85 | 100 | 73 | 100 | 30 | — | 50 | 72 |
| instant-meshes | 3070 | **59** | 40 | 38 | 100 | 58 | 81 | 2 | — | 91 | 100 |
| quadremesher | 4310 | **58** | 0 | 95 | 100 | 88 | 100 | 42 | 52 | 0 | 70 |
| quadriflow | 3030 | **47** | 0 | 49 | 96 | 41 | 100 | 5 | 21 | 96 | 71 |
| autoremesher | 1255 | **41** | 0 | 64 | 82 | 38 | 95 | — | 30 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 4311 | **89** | 100 | 87 | 100 | 88 | 100 | 57 | 70 | 100 | 69 |
| quadremesher-as-made | 4310 | **65** | 0 | 94 | 100 | 86 | 100 | 42 | — | 100 | 70 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-uniform | 7317 | **90** | 100 | 87 | 100 | 90 | 100 | 54 | — | 66 | 61 |
| integer-grid | 7376 | **90** | 100 | 86 | 100 | 90 | 100 | 42 | — | 69 | 0 |
| **openclare** | 7925 | **89** | 100 | 88 | 100 | 94 | 100 | 58 | 57 | 96 | 50 |
| quadremesher | 9246 | **89** | 100 | 96 | 100 | 93 | 100 | 56 | 85 | 38 | 50 |
| openclare-uniform | 8061 | **89** | 100 | 85 | 100 | 91 | 100 | 53 | 61 | 97 | 58 |
| instant-meshes | 8098 | **81** | 100 | 40 | 100 | 88 | 100 | 3 | — | 95 | 100 |
| quadriflow | 7983 | **78** | 100 | 57 | 100 | 86 | 100 | 22 | 32 | 97 | 53 |
| autoremesher | 2647 | **42** | 0 | 72 | 92 | 36 | 100 | — | 17 | 0 | 96 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 9246 | **95** | 100 | 96 | 100 | 93 | 100 | 56 | — | 100 | 50 |
| openclare-matched | 9149 | **91** | 100 | 90 | 100 | 95 | 100 | 61 | 71 | 96 | 49 |
