# human_female

An organic model; 1 354 880 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | Modelled by hand | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/human_female__source.webp) | ![Modelled by hand](img/human_female__handmade.webp) | ![AutoRemesher](img/human_female__autoremesher.webp) | ![openclare](img/human_female__openclare.webp) |
| 1 354 880 triangles | 10 590 quads | 689 quads | 3 040 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | 32 % | 3 % | 66 % | 0 % | 3 | 274 |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | 12 % | 20 % | 68 % | 0 % | 18 | 46 |
| openclare | 29 % | 4 % | 67 % | 0 % | 15 | 187 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-as-handmade | 10602 | **96** | 100 | 100 | 100 | 89 | 100 | 63 | 96 | 100 | 14 |
| handmade | 10590 | **96** | 96 | 90 | 95 | 94 | 100 | — | 98 | 100 | — |
| openclare-uniform | 1003 | **92** | 100 | 100 | 100 | 46 | 100 | 70 | 100 | 99 | 37 |
| **openclare** | 982 | **80** | 100 | 92 | 97 | 66 | 48 | 44 | 53 | 93 | 31 |
| integer-grid | 869 | **79** | 100 | 100 | 100 | 9 | 100 | 68 | 76 | 48 | 35 |
| instant-meshes | 877 | **45** | 0 | 79 | 89 | 0 | 68 | 27 | 64 | 51 | 26 |
| autoremesher | 172 | **17** | 0 | 70 | 0 | 0 | 0 | — | 31 | 0 | 70 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-as-handmade | 10602 | **96** | 100 | 100 | 100 | 89 | 100 | 63 | 96 | 100 | 14 |
| handmade | 10590 | **96** | 96 | 90 | 95 | 94 | 100 | — | 98 | 100 | — |
| **openclare** | 3040 | **92** | 100 | 100 | 100 | 68 | 96 | 58 | 93 | 95 | 30 |
| openclare-uniform | 2981 | **90** | 100 | 100 | 100 | 56 | 100 | 53 | 85 | 97 | 30 |
| integer-grid | 2766 | **56** | 0 | 100 | 100 | 11 | 100 | — | 57 | 69 | 30 |
| instant-meshes | 2837 | **48** | 0 | 82 | 100 | 0 | 81 | 17 | 46 | 78 | 26 |
| autoremesher | 689 | **30** | 0 | 74 | 60 | 9 | 38 | — | 26 | 0 | 62 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-as-handmade | 10602 | **96** | 100 | 100 | 100 | 89 | 100 | 63 | 96 | 100 | 14 |
| **openclare** | 7966 | **96** | 100 | 100 | 100 | 86 | 100 | 65 | 98 | 98 | 20 |
| handmade | 10590 | **96** | 96 | 90 | 95 | 94 | 100 | — | 98 | 100 | — |
| openclare-uniform | 7870 | **92** | 100 | 100 | 100 | 69 | 100 | 64 | 88 | 94 | 22 |
| integer-grid | 7201 | **59** | 0 | 100 | 100 | 43 | 100 | — | 50 | 60 | 12 |
| instant-meshes | 7648 | **50** | 0 | 75 | 100 | 17 | 98 | 16 | 42 | 82 | 26 |
| autoremesher | 2564 | **47** | 0 | 87 | 98 | 24 | 91 | — | — | 0 | 54 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |
