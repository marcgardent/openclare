# sphere

A calibration model; 5 120 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/sphere__source.webp) | ![QuadriFlow (Blender)](img/sphere__quadriflow.webp) | ![AutoRemesher](img/sphere__autoremesher.webp) | ![openclare](img/sphere__openclare.webp) | ![Quad Remesher](img/sphere__quadremesher.webp) |
| 5 120 triangles | 3 253 quads | 2 086 quads | 3 000 quads | 2 740 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/sphere__quadremesher.webp) | ![openclare, as many quads](img/sphere__openclare-matched.webp) |
| 2 740 quads | 2 706 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 6 % | 70 % | 15 % | 0 % | 5 | 276 |
| AutoRemesher | 2 % | 74 % | 24 % | 0 % | 6 | 298 |
| openclare | 90 % | 0 % | 10 % | 0 % | 0 | 10 |
| Quad Remesher | 17 % | 69 % | 14 % | 0 % | 6 | 158 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1010 | **89** | 100 | — | 77 | 98 | 100 | 0 | — | 96 | 100 |
| quadremesher-uniform | 915 | **84** | 100 | — | 70 | 96 | 98 | 24 | — | 66 | 68 |
| integer-grid | 923 | **82** | 100 | — | 58 | 98 | 100 | 0 | — | 69 | 86 |
| instant-meshes | 1066 | **81** | 100 | — | 74 | 97 | 66 | 0 | — | 74 | 100 |
| **openclare** | 1010 | **79** | 100 | — | 77 | 98 | 100 | 0 | 0 | 96 | 100 |
| quadriflow | 950 | **77** | 100 | — | 77 | 97 | 100 | 1 | 0 | 80 | 100 |
| quadremesher | 915 | **76** | 100 | — | 70 | 96 | 98 | 24 | 11 | 66 | 62 |
| autoremesher | 768 | **68** | 100 | — | 74 | 94 | 100 | 0 | 0 | 7 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 900 | **88** | 100 | — | 80 | 96 | 98 | 6 | — | 93 | 96 |
| quadremesher-as-made | 915 | **87** | 100 | — | 70 | 96 | 98 | 24 | — | 100 | 62 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3000 | **95** | 100 | — | 80 | 99 | 92 | 99 | 97 | 100 | 94 |
| openclare-uniform | 3000 | **95** | 100 | — | 80 | 99 | 92 | 99 | — | 100 | 86 |
| integer-grid | 3000 | **91** | 100 | — | 58 | 99 | 92 | 99 | — | 100 | 84 |
| quadremesher-uniform | 2740 | **85** | 100 | — | 70 | 98 | 99 | 35 | — | 65 | 70 |
| instant-meshes | 3090 | **84** | 100 | — | 79 | 99 | 71 | 0 | — | 88 | 100 |
| quadremesher | 2740 | **80** | 100 | — | 70 | 98 | 99 | 35 | 41 | 65 | 69 |
| quadriflow | 3107 | **79** | 100 | — | 78 | 99 | 100 | 12 | 6 | 86 | 86 |
| autoremesher | 2086 | **66** | 100 | — | 62 | 97 | 99 | 4 | 6 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 2706 | **94** | 100 | — | 81 | 99 | 91 | 99 | — | 95 | 90 |
| quadremesher-as-made | 2740 | **88** | 100 | — | 70 | 98 | 99 | 35 | — | 100 | 69 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7984 | **89** | 100 | — | 78 | 99 | 98 | 0 | — | 99 | 62 |
| instant-meshes | 8190 | **86** | 100 | — | 83 | 99 | 77 | 0 | — | 90 | 100 |
| integer-grid | 7568 | **85** | 100 | — | 70 | 99 | 98 | 0 | — | 78 | 37 |
| quadremesher-uniform | 7088 | **83** | 100 | — | 62 | 99 | 100 | 30 | — | 54 | 65 |
| **openclare** | 8074 | **79** | 100 | — | 77 | 99 | 98 | 0 | 0 | 96 | 56 |
| quadremesher | 7088 | **78** | 100 | — | 62 | 99 | 100 | 30 | 40 | 54 | 65 |
| autoremesher | 6520 | **70** | 100 | — | 71 | 98 | 97 | 4 | 4 | 26 | 84 |
| quadriflow | 7736 | **66** | 60 | — | 75 | 99 | 99 | 0 | 0 | 87 | 63 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 7088 | **87** | 100 | — | 62 | 99 | 100 | 30 | — | 100 | 65 |
| openclare-matched | 7223 | **86** | 100 | — | 70 | 99 | 99 | 0 | — | 92 | 59 |
