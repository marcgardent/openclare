# blub

An organic model; 106 640 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/blub__source.webp) | ![QuadriFlow (Blender)](img/blub__quadriflow.webp) | ![AutoRemesher](img/blub__autoremesher.webp) | ![openclare](img/blub__openclare.webp) | ![Quad Remesher](img/blub__quadremesher.webp) |
| 106 640 triangles | 2 770 quads | 1 343 quads | 3 030 quads | 3 889 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/blub__quadremesher.webp) | ![openclare, as many quads](img/blub__openclare-matched.webp) |
| 3 889 quads | 3 871 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 33 % | 64 % | 0 % | 13 | 225 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 36 % | 13 % | 51 % | 0 % | 8 | 170 |
| Quad Remesher | 25 % | 20 % | 54 % | 0 % | 6 | 201 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1027 | **89** | 100 | 98 | 100 | 65 | 100 | 55 | 79 | 89 | 83 |
| **openclare** | 984 | **85** | 100 | 93 | 100 | 76 | 100 | 46 | 43 | 94 | 81 |
| quadremesher-uniform | 892 | **85** | 100 | 100 | 98 | 49 | 100 | 51 | — | 57 | 38 |
| integer-grid | 888 | **84** | 95 | 92 | 96 | 48 | 100 | — | — | 55 | 73 |
| quadremesher | 1638 | **81** | 100 | 100 | 100 | 88 | 100 | 49 | 49 | 0 | 35 |
| instant-meshes | 996 | **60** | 60 | 37 | 100 | 36 | 76 | 2 | — | 94 | 97 |
| quadriflow | 847 | **51** | 60 | 43 | 88 | 31 | 100 | 7 | 19 | 39 | 83 |
| autoremesher | 604 | **35** | 0 | 55 | 86 | 18 | 54 | — | 42 | 0 | 92 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1668 | **95** | 100 | 100 | 100 | 82 | 100 | 61 | 100 | 93 | 81 |
| quadremesher-as-made | 1638 | **95** | 100 | 100 | 100 | 85 | 100 | 49 | — | 100 | 35 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3009 | **95** | 100 | 100 | 100 | 82 | 100 | 76 | 89 | 99 | 78 |
| **openclare** | 3030 | **92** | 100 | 100 | 100 | 84 | 100 | 58 | 70 | 96 | 68 |
| integer-grid | 2680 | **89** | 100 | 100 | 100 | 68 | 100 | 66 | — | 57 | 56 |
| quadremesher-uniform | 2703 | **88** | 100 | 100 | 100 | 69 | 100 | 24 | — | 60 | 39 |
| quadremesher | 3889 | **81** | 100 | 100 | 100 | 87 | 100 | 48 | 50 | 0 | 35 |
| instant-meshes | 3036 | **78** | 100 | 49 | 100 | 60 | 100 | 4 | — | 95 | 95 |
| autoremesher | 1343 | **53** | 0 | 82 | 93 | 65 | 100 | — | — | 0 | 95 |
| quadriflow | 2698 | **48** | 60 | 45 | 84 | 13 | 100 | 4 | 0 | 60 | 87 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 3889 | **94** | 100 | 100 | 100 | 84 | 100 | 48 | — | 100 | 35 |
| openclare-matched | 3871 | **92** | 100 | 100 | 100 | 90 | 100 | 67 | 62 | 98 | 67 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7961 | **92** | 100 | 100 | 100 | 94 | 100 | 78 | 50 | 98 | 52 |
| integer-grid | 7057 | **91** | 100 | 100 | 95 | 91 | 100 | 58 | — | 53 | 27 |
| **openclare** | 7851 | **91** | 100 | 100 | 100 | 95 | 100 | 71 | 50 | 93 | 47 |
| quadremesher-uniform | 6943 | **89** | 100 | 100 | 100 | 84 | 100 | 29 | — | 47 | 37 |
| quadremesher | 8513 | **87** | 100 | 100 | 100 | 91 | 100 | 27 | 50 | 74 | 33 |
| instant-meshes | 8100 | **84** | 100 | 54 | 100 | 88 | 100 | 2 | — | 95 | 96 |
| autoremesher | 3438 | **82** | 98 | 100 | 100 | 83 | 100 | — | 54 | 0 | 67 |
| quadriflow | 7229 | **44** | 0 | 50 | 86 | 56 | 100 | 13 | 19 | 61 | 69 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 8513 | **95** | 100 | 100 | 100 | 91 | 100 | 27 | — | 100 | 33 |
| openclare-matched | 8570 | **93** | 100 | 100 | 100 | 94 | 100 | 77 | 65 | 97 | 43 |
