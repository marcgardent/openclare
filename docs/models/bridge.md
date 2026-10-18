# bridge

An organic model, with open borders; 164 064 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/bridge__source.webp) | ![AutoRemesher](img/bridge__autoremesher.webp) | ![openclare](img/bridge__openclare.webp) | ![Quad Remesher](img/bridge__quadremesher.webp) |
| 164 064 triangles | 1 893 quads | 2 954 quads | 4 533 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/bridge__quadremesher.webp) | ![openclare, as many quads](img/bridge__openclare-matched.webp) |
| 4 533 quads | 4 486 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | 19 % | 8 % | 62 % | 10 % | 4 | 70 |
| openclare | 26 % | 5 % | 47 % | 23 % | 2 | 132 |
| Quad Remesher | 15 % | 8 % | 60 % | 17 % | 7 | 145 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1008 | **95** | 100 | 100 | 100 | 78 | 100 | 72 | 99 | 97 | 63 |
| **openclare** | 1014 | **95** | 100 | 100 | 100 | 88 | 100 | 68 | 89 | 94 | 70 |
| integer-grid | 919 | **93** | 100 | 100 | 100 | 80 | 100 | 81 | — | 68 | 60 |
| quadremesher-uniform | 936 | **89** | 94 | 93 | 89 | 81 | 92 | — | — | 74 | 40 |
| quadremesher | 2105 | **78** | 100 | 100 | 100 | 91 | 100 | 36 | 32 | 0 | 37 |
| instant-meshes | 845 | **58** | 80 | 62 | 96 | 1 | 52 | 37 | — | 38 | 85 |
| autoremesher | 748 | **42** | 0 | 79 | 47 | 35 | 78 | — | 70 | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 2093 | **97** | 100 | 100 | 100 | 86 | 100 | 76 | 100 | 98 | 67 |
| quadremesher-as-made | 2105 | **94** | 100 | 100 | 100 | 89 | 100 | 36 | — | 100 | 37 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2954 | **96** | 100 | 100 | 100 | 87 | 100 | 76 | 93 | 94 | 68 |
| openclare-uniform | 3020 | **94** | 100 | 100 | 100 | 78 | 100 | 72 | 89 | 97 | 69 |
| quadremesher-uniform | 2726 | **94** | 99 | 100 | 100 | 86 | 100 | — | — | 63 | 40 |
| integer-grid | 2750 | **93** | 100 | 95 | 100 | 86 | 100 | 81 | — | 67 | 42 |
| quadremesher | 4533 | **86** | 100 | 100 | 100 | 88 | 100 | — | 80 | 0 | 41 |
| instant-meshes | 2903 | **81** | 100 | 68 | 100 | 44 | 98 | 35 | — | 87 | 73 |
| autoremesher | 1893 | **80** | 93 | 92 | 100 | 70 | 100 | — | 72 | 0 | 95 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 4533 | **98** | 100 | 100 | 100 | 88 | 100 | — | — | 100 | 41 |
| openclare-matched | 4486 | **97** | 100 | 100 | 100 | 86 | 100 | 73 | 100 | 96 | 55 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8076 | **95** | 100 | 94 | 100 | 87 | 100 | 70 | 96 | 96 | 49 |
| openclare-uniform | 8034 | **95** | 100 | 88 | 100 | 87 | 100 | 76 | 100 | 98 | 46 |
| integer-grid | 7468 | **92** | 100 | 87 | 100 | 86 | 100 | 78 | — | 73 | 23 |
| quadremesher-uniform | 7054 | **91** | 100 | 92 | 100 | 87 | 100 | — | — | 53 | 40 |
| quadremesher | 8805 | **91** | 100 | 98 | 100 | 87 | 100 | 67 | 84 | 60 | 33 |
| instant-meshes | 7960 | **86** | 100 | 59 | 100 | 82 | 100 | 35 | — | 98 | 81 |
| autoremesher | 4954 | **82** | 97 | 83 | 100 | 82 | 100 | — | 82 | 0 | 72 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 8805 | **95** | 100 | 95 | 100 | 87 | 100 | 67 | — | 100 | 33 |
| openclare-matched | 8688 | **95** | 100 | 91 | 100 | 85 | 100 | 73 | 98 | 95 | 47 |
