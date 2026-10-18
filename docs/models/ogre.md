# ogre

An organic model; 199 764 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/ogre__source.webp) | ![QuadriFlow (Blender)](img/ogre__quadriflow.webp) | ![AutoRemesher](img/ogre__autoremesher.webp) | ![openclare](img/ogre__openclare.webp) |
| 199 764 triangles | 3 049 quads | 1 081 quads | 2 965 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 2 % | 10 % | 89 % | 0 % | 14 | 250 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 16 % | 8 % | 76 % | 0 % | 19 | 135 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1019 | **87** | 100 | 82 | 100 | 52 | 100 | 62 | 87 | 92 | 57 |
| **openclare** | 1009 | **80** | 100 | 77 | 91 | 65 | 51 | 47 | 78 | 96 | 61 |
| integer-grid | 802 | **70** | 95 | 73 | 79 | 31 | 91 | — | — | 21 | 62 |
| instant-meshes | 932 | **51** | 80 | 25 | 90 | 0 | 44 | 8 | — | 73 | 76 |
| autoremesher | 311 | **41** | 87 | 37 | 69 | 0 | 34 | — | 15 | 0 | 100 |
| quadriflow | 891 | **38** | 60 | 11 | 64 | 0 | 78 | 0 | 27 | 56 | 71 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2965 | **83** | 100 | 75 | 100 | 65 | 99 | 32 | 65 | 95 | 38 |
| openclare-uniform | 3053 | **82** | 100 | 69 | 100 | 47 | 100 | 46 | 81 | 93 | 45 |
| integer-grid | 2465 | **71** | 98 | 71 | 94 | 14 | 100 | — | — | 29 | 49 |
| instant-meshes | 2874 | **57** | 80 | 27 | 100 | 4 | 80 | 3 | — | 83 | 76 |
| quadriflow | 2935 | **41** | 20 | 22 | 76 | 13 | 100 | 3 | 37 | 83 | 71 |
| autoremesher | 1081 | **33** | 0 | 43 | 86 | 19 | 72 | — | — | 0 | 95 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7979 | **88** | 100 | 82 | 100 | 87 | 100 | 32 | 69 | 99 | 26 |
| openclare-uniform | 8010 | **87** | 100 | 76 | 100 | 76 | 100 | 34 | 79 | 100 | 31 |
| integer-grid | 6967 | **80** | 99 | 75 | 94 | 49 | 100 | — | — | 48 | 13 |
| instant-meshes | 7836 | **45** | 0 | 33 | 100 | 34 | 95 | 2 | — | 92 | 77 |
| autoremesher | 4622 | **43** | 0 | 66 | 100 | 22 | 100 | — | 38 | 0 | 67 |
| quadriflow | 7763 | **43** | 0 | 33 | 79 | 51 | 100 | 5 | 28 | 88 | 62 |
