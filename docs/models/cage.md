# cage

An organic model; 26 656 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/cage__source.webp) | ![QuadriFlow (Blender)](img/cage__quadriflow.webp) | ![AutoRemesher](img/cage__autoremesher.webp) | ![openclare](img/cage__openclare.webp) |
| 26 656 triangles | 2 885 quads | 4 039 quads | 3 021 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 18 % | 38 % | 42 % | 0 % | 20 | 86 |
| AutoRemesher | 4 % | 30 % | 66 % | 0 % | 24 | 110 |
| openclare | 54 % | 22 % | 23 % | 0 % | 6 | 69 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1008 | **96** | 100 | 100 | 100 | 78 | 100 | 85 | 97 | 97 | 92 |
| integer-grid | 970 | **93** | 100 | 100 | 100 | 65 | 100 | 90 | — | 88 | 68 |
| openclare-uniform | 1019 | **92** | 100 | 100 | 100 | 65 | 100 | 83 | 81 | 92 | 98 |
| quadriflow | 1043 | **80** | 100 | 100 | 100 | 57 | 100 | 31 | 27 | 83 | 100 |
| instant-meshes | 974 | **77** | 100 | 85 | 92 | 43 | 43 | 20 | — | 90 | 100 |
| autoremesher | 1720 | **41** | 0 | 94 | 85 | 0 | 100 | — | 25 | 0 | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 2874 | **94** | 100 | 100 | 100 | 77 | 100 | 85 | — | 83 | 48 |
| openclare-uniform | 2981 | **94** | 100 | 100 | 100 | 78 | 100 | 92 | 80 | 97 | 79 |
| instant-meshes | 2944 | **90** | 100 | 100 | 100 | 71 | 93 | 21 | — | 93 | 100 |
| **openclare** | 3021 | **90** | 100 | 100 | 100 | 80 | 100 | 81 | 50 | 97 | 73 |
| quadriflow | 2891 | **84** | 100 | 99 | 100 | 77 | 100 | 33 | 31 | 85 | 94 |
| autoremesher | 4039 | **50** | 0 | 94 | 94 | 73 | 100 | — | 6 | 0 | 79 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8013 | **99** | 100 | 100 | 100 | 91 | 100 | 98 | 100 | 99 | 47 |
| integer-grid | 7765 | **97** | 100 | 100 | 100 | 90 | 100 | 90 | — | 88 | 20 |
| openclare-uniform | 8124 | **97** | 100 | 100 | 100 | 94 | 100 | 95 | 87 | 94 | 59 |
| instant-meshes | 8050 | **91** | 100 | 94 | 100 | 80 | 100 | 11 | — | 98 | 100 |
| quadriflow | 7932 | **83** | 100 | 99 | 100 | 87 | 100 | 8 | 18 | 97 | 58 |
| autoremesher | 5928 | **82** | 99 | 100 | 100 | 85 | 100 | — | 50 | 0 | 71 |
