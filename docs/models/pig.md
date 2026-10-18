# pig

An organic model; 100 062 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare |
|---|---|---|---|
| ![Source](img/pig__source.webp) | ![QuadriFlow (Blender)](img/pig__quadriflow.webp) | ![AutoRemesher](img/pig__autoremesher.webp) | ![openclare](img/pig__openclare.webp) |
| 100 062 triangles | 2 675 quads | 1 808 quads | 3 027 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 10 % | 90 % | 0 % | 12 | 218 |
| AutoRemesher | 11 % | 6 % | 84 % | 0 % | 5 | 113 |
| openclare | 21 % | 4 % | 75 % | 0 % | 6 | 170 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1007 | **89** | 100 | 91 | 100 | 82 | 67 | 46 | 89 | 97 | 63 |
| openclare-uniform | 980 | **63** | 0 | 72 | 99 | 56 | 100 | 44 | 92 | 92 | 74 |
| integer-grid | 877 | **51** | 0 | 65 | 96 | 44 | 99 | 48 | — | 51 | 62 |
| instant-meshes | 910 | **37** | 0 | 44 | 97 | 0 | 68 | 5 | — | 64 | 98 |
| autoremesher | 548 | **32** | 0 | 55 | 76 | 5 | 24 | — | 65 | 0 | 100 |
| quadriflow | 861 | **25** | 0 | 36 | 56 | 0 | 62 | 2 | 20 | 44 | 81 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3027 | **92** | 100 | 93 | 100 | 83 | 100 | 39 | 86 | 96 | 55 |
| openclare-uniform | 2990 | **88** | 100 | 80 | 100 | 63 | 100 | 57 | 90 | 99 | 63 |
| integer-grid | 2680 | **83** | 99 | 81 | 98 | 50 | 100 | — | — | 57 | 44 |
| autoremesher | 1808 | **66** | 85 | 77 | 95 | 21 | 95 | — | 65 | 0 | 90 |
| instant-meshes | 3016 | **46** | 0 | 43 | 100 | 25 | 87 | 3 | — | 96 | 97 |
| quadriflow | 2675 | **33** | 0 | 35 | 63 | 5 | 100 | 2 | 33 | 57 | 88 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8105 | **92** | 100 | 98 | 100 | 90 | 100 | 53 | 69 | 95 | 34 |
| openclare-uniform | 8061 | **88** | 100 | 91 | 100 | 77 | 100 | 50 | 62 | 97 | 38 |
| instant-meshes | 8116 | **75** | 80 | 52 | 100 | 67 | 100 | 4 | — | 94 | 92 |
| integer-grid | 7187 | **62** | 0 | 91 | 95 | 76 | 100 | 37 | — | 59 | 14 |
| autoremesher | 4682 | **49** | 0 | 78 | 100 | 68 | 100 | — | 20 | 0 | 63 |
| quadriflow | 6973 | **32** | 0 | 46 | 80 | 0 | 100 | 2 | 6 | 49 | 72 |
