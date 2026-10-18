# jerry

An organic model, with open borders; 39 856 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/jerry__source.webp) | ![QuadriFlow (Blender)](img/jerry__quadriflow.webp) | ![AutoRemesher](img/jerry__autoremesher.webp) | ![openclare](img/jerry__openclare.webp) |
| 39 856 triangles | 3 015 quads | 1 102 quads | 3 012 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 1 % | 82 % | 17 % | 1 | 155 |
| AutoRemesher | 0 % | 13 % | 87 % | 0 % | 6 | 64 |
| openclare | 3 % | 0 % | 87 % | 10 % | 0 | 103 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1020 | **76** | 100 | 52 | 100 | 41 | 100 | 52 | 59 | 92 | 87 |
| **openclare** | 1007 | **70** | 100 | 51 | 90 | 58 | 57 | 2 | 51 | 97 | 73 |
| integer-grid | 867 | **62** | 100 | 41 | 81 | 9 | 92 | 24 | — | 47 | 69 |
| quadriflow | 900 | **55** | 80 | 28 | 87 | 22 | 73 | 20 | 50 | 60 | 100 |
| instant-meshes | 886 | **41** | 60 | 22 | 82 | 8 | 28 | 4 | — | 54 | 100 |
| autoremesher | 305 | **22** | 48 | 14 | 38 | 0 | 42 | — | 0 | 0 | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3012 | **77** | 100 | 55 | 100 | 64 | 94 | 25 | 53 | 98 | 55 |
| openclare-uniform | 3045 | **77** | 100 | 56 | 100 | 48 | 100 | 43 | 58 | 94 | 72 |
| integer-grid | 2595 | **66** | 100 | 53 | 91 | 2 | 100 | 40 | — | 46 | 51 |
| quadriflow | 3122 | **62** | 80 | 32 | 81 | 38 | 100 | 29 | 50 | 84 | 87 |
| instant-meshes | 2959 | **42** | 20 | 29 | 97 | 11 | 60 | 0 | — | 95 | 100 |
| autoremesher | 1102 | **23** | 0 | 31 | 57 | 0 | 74 | — | 18 | 0 | 92 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8098 | **81** | 100 | 68 | 100 | 78 | 100 | 13 | 53 | 95 | 39 |
| openclare-uniform | 7970 | **80** | 100 | 63 | 100 | 62 | 100 | 32 | 58 | 98 | 48 |
| integer-grid | 7345 | **77** | 100 | 63 | 97 | 44 | 100 | 29 | — | 67 | 12 |
| quadriflow | 7781 | **66** | 80 | 34 | 100 | 44 | 100 | 36 | 47 | 89 | 56 |
| instant-meshes | 7933 | **49** | 20 | 27 | 100 | 32 | 88 | 2 | — | 97 | 100 |
| autoremesher | 2740 | **38** | 0 | 47 | 87 | 25 | 100 | — | — | 0 | 78 |
