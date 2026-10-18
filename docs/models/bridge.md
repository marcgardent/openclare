# bridge

An organic model, with open borders; 164 064 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare |
|---|---|---|
| ![Source](img/bridge__source.webp) | ![AutoRemesher](img/bridge__autoremesher.webp) | ![openclare](img/bridge__openclare.webp) |
| 164 064 triangles | 1 893 quads | 2 971 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | 19 % | 8 % | 62 % | 10 % | 4 | 70 |
| openclare | 26 % | 0 % | 40 % | 34 % | 1 | 94 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1011 | **96** | 100 | 100 | 100 | 78 | 100 | 76 | 100 | 96 | 68 |
| **openclare** | 984 | **96** | 100 | 100 | 100 | 80 | 100 | 74 | 100 | 94 | 65 |
| integer-grid | 919 | **93** | 100 | 100 | 100 | 80 | 100 | 81 | — | 68 | 60 |
| instant-meshes | 845 | **58** | 80 | 62 | 96 | 1 | 52 | 37 | — | 38 | 85 |
| autoremesher | 748 | **42** | 0 | 79 | 47 | 35 | 78 | — | 70 | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2997 | **97** | 100 | 98 | 100 | 86 | 100 | 81 | 100 | 100 | 58 |
| **openclare** | 2971 | **96** | 100 | 100 | 100 | 81 | 100 | 83 | 100 | 96 | 60 |
| integer-grid | 2750 | **93** | 100 | 95 | 100 | 86 | 100 | 81 | — | 67 | 42 |
| instant-meshes | 2903 | **81** | 100 | 68 | 100 | 44 | 98 | 35 | — | 87 | 73 |
| autoremesher | 1893 | **80** | 93 | 92 | 100 | 70 | 100 | — | 72 | 0 | 95 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7922 | **95** | 100 | 90 | 100 | 86 | 100 | 77 | 100 | 96 | 46 |
| openclare-uniform | 8041 | **95** | 100 | 86 | 100 | 87 | 100 | 80 | 100 | 98 | 33 |
| integer-grid | 7468 | **92** | 100 | 87 | 100 | 86 | 100 | 78 | — | 73 | 23 |
| instant-meshes | 7960 | **86** | 100 | 59 | 100 | 82 | 100 | 35 | — | 98 | 81 |
| autoremesher | 4954 | **82** | 97 | 83 | 100 | 82 | 100 | — | 82 | 0 | 72 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |
