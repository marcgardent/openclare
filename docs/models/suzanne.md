# suzanne

An organic model, with open borders; 15 744 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/suzanne__source.webp) | ![QuadriFlow (Blender)](img/suzanne__quadriflow.webp) | ![AutoRemesher](img/suzanne__autoremesher.webp) | ![openclare](img/suzanne__openclare.webp) |
| 15 744 triangles | 3 012 quads | 1 255 quads | 2 987 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 2 % | 15 % | 80 % | 0 % | 12 | 233 |
| AutoRemesher | 9 % | 15 % | 77 % | 0 % | 8 | 82 |
| openclare | 19 % | 10 % | 61 % | 10 % | 3 | 170 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 985 | **87** | 100 | 79 | 100 | 80 | 100 | 41 | 72 | 94 | 81 |
| openclare-uniform | 1005 | **86** | 100 | 75 | 100 | 66 | 100 | 42 | 81 | 98 | 79 |
| integer-grid | 976 | **57** | 0 | 75 | 94 | 51 | 98 | 25 | — | 90 | 56 |
| quadriflow | 1005 | **46** | 0 | 47 | 90 | 45 | 83 | 1 | 38 | 96 | 100 |
| instant-meshes | 958 | **37** | 0 | 32 | 87 | 26 | 48 | 3 | — | 83 | 100 |
| autoremesher | 357 | **24** | 0 | 29 | 54 | 0 | 83 | 0 | 40 | 0 | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2982 | **91** | 100 | 83 | 100 | 82 | 100 | 51 | 92 | 98 | 67 |
| **openclare** | 2987 | **89** | 100 | 86 | 100 | 86 | 100 | 50 | 68 | 98 | 57 |
| integer-grid | 2757 | **86** | 100 | 83 | 100 | 75 | 100 | 33 | — | 68 | 48 |
| instant-meshes | 3070 | **59** | 40 | 38 | 100 | 58 | 81 | 2 | — | 91 | 100 |
| quadriflow | 3030 | **47** | 0 | 49 | 96 | 41 | 100 | 5 | 21 | 96 | 71 |
| autoremesher | 1255 | **41** | 0 | 64 | 82 | 38 | 95 | — | 30 | 0 | 100 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7889 | **90** | 100 | 87 | 100 | 93 | 100 | 64 | 63 | 94 | 48 |
| integer-grid | 7376 | **90** | 100 | 86 | 100 | 90 | 100 | 42 | — | 69 | 0 |
| openclare-uniform | 7884 | **89** | 100 | 84 | 100 | 88 | 100 | 55 | 65 | 94 | 53 |
| instant-meshes | 8098 | **81** | 100 | 40 | 100 | 88 | 100 | 3 | — | 95 | 100 |
| quadriflow | 7983 | **78** | 100 | 57 | 100 | 86 | 100 | 22 | 32 | 97 | 53 |
| autoremesher | 2647 | **42** | 0 | 72 | 92 | 36 | 100 | — | 17 | 0 | 96 |
