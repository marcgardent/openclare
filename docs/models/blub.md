# blub

An organic model; 106 640 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/blub__source.webp) | ![QuadriFlow (Blender)](img/blub__quadriflow.webp) | ![AutoRemesher](img/blub__autoremesher.webp) | ![openclare](img/blub__openclare.webp) |
| 106 640 triangles | 2 770 quads | 1 343 quads | 2 981 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 33 % | 64 % | 0 % | 13 | 225 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 35 % | 11 % | 54 % | 0 % | 4 | 164 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 989 | **95** | 100 | 96 | 100 | 81 | 100 | 64 | 100 | 96 | 72 |
| openclare-uniform | 990 | **92** | 100 | 94 | 100 | 64 | 100 | 67 | 100 | 96 | 77 |
| integer-grid | 888 | **84** | 95 | 92 | 96 | 48 | 100 | — | — | 55 | 73 |
| instant-meshes | 996 | **60** | 60 | 37 | 100 | 36 | 76 | 2 | — | 94 | 97 |
| quadriflow | 847 | **51** | 60 | 43 | 88 | 31 | 100 | 7 | 19 | 39 | 83 |
| autoremesher | 604 | **35** | 0 | 55 | 86 | 18 | 54 | — | 42 | 0 | 92 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2981 | **92** | 100 | 100 | 100 | 81 | 100 | 62 | 74 | 97 | 62 |
| openclare-uniform | 2963 | **92** | 100 | 100 | 100 | 72 | 100 | 76 | 77 | 95 | 57 |
| integer-grid | 2680 | **89** | 100 | 100 | 100 | 68 | 100 | 66 | — | 57 | 56 |
| instant-meshes | 3036 | **78** | 100 | 49 | 100 | 60 | 100 | 4 | — | 95 | 95 |
| autoremesher | 1343 | **53** | 0 | 82 | 93 | 65 | 100 | — | — | 0 | 95 |
| quadriflow | 2698 | **48** | 60 | 45 | 84 | 13 | 100 | 4 | 0 | 60 | 87 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 8160 | **98** | 100 | 100 | 100 | 96 | 100 | 92 | 100 | 92 | 46 |
| integer-grid | 7057 | **91** | 100 | 100 | 95 | 91 | 100 | 58 | — | 53 | 27 |
| **openclare** | 7884 | **90** | 100 | 100 | 100 | 87 | 100 | 75 | 50 | 94 | 46 |
| instant-meshes | 8100 | **84** | 100 | 54 | 100 | 88 | 100 | 2 | — | 95 | 96 |
| autoremesher | 3438 | **82** | 98 | 100 | 100 | 83 | 100 | — | 54 | 0 | 67 |
| quadriflow | 7229 | **44** | 0 | 50 | 86 | 56 | 100 | 13 | 19 | 61 | 69 |
