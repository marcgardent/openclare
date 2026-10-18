# blub

An organic model; 106 640 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/blub__source.webp) | ![QuadriFlow (Blender)](img/blub__quadriflow.webp) | ![AutoRemesher](img/blub__autoremesher.webp) | ![openclare](img/blub__openclare.webp) | ![Quad Remesher](img/blub__quadremesher.webp) |
| 106 640 triangles | 2 770 quads | 1 343 quads | 2 981 quads | 3 889 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/blub__quadremesher.webp) | ![openclare, as many quads](img/blub__openclare-matched.webp) |
| 3 889 quads | 3 935 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 33 % | 64 % | 0 % | 13 | 225 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 35 % | 11 % | 54 % | 0 % | 4 | 164 |
| Quad Remesher | 25 % | 20 % | 54 % | 0 % | 6 | 201 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 989 | **95** | 100 | 96 | 100 | 81 | 100 | 64 | 100 | 96 | 72 |
| openclare-uniform | 990 | **92** | 100 | 94 | 100 | 64 | 100 | 67 | 100 | 96 | 77 |
| quadremesher-uniform | 892 | **85** | 100 | 100 | 98 | 49 | 100 | 51 | — | 57 | 38 |
| integer-grid | 888 | **84** | 95 | 92 | 96 | 48 | 100 | — | — | 55 | 73 |
| quadremesher | 1638 | **81** | 100 | 100 | 100 | 88 | 100 | 49 | 49 | 0 | 35 |
| instant-meshes | 996 | **60** | 60 | 37 | 100 | 36 | 76 | 2 | — | 94 | 97 |
| quadriflow | 847 | **51** | 60 | 43 | 88 | 31 | 100 | 7 | 19 | 39 | 83 |
| autoremesher | 604 | **35** | 0 | 55 | 86 | 18 | 54 | — | 42 | 0 | 92 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1626 | **97** | 100 | 100 | 100 | 85 | 100 | 73 | 100 | 97 | 70 |
| quadremesher-as-made | 1638 | **95** | 100 | 100 | 100 | 85 | 100 | 49 | — | 100 | 35 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2981 | **92** | 100 | 100 | 100 | 81 | 100 | 62 | 74 | 97 | 62 |
| openclare-uniform | 2963 | **92** | 100 | 100 | 100 | 72 | 100 | 76 | 77 | 95 | 57 |
| integer-grid | 2680 | **89** | 100 | 100 | 100 | 68 | 100 | 66 | — | 57 | 56 |
| quadremesher-uniform | 2703 | **88** | 100 | 100 | 100 | 69 | 100 | 24 | — | 60 | 39 |
| quadremesher | 3889 | **81** | 100 | 100 | 100 | 87 | 100 | 48 | 50 | 0 | 35 |
| instant-meshes | 3036 | **78** | 100 | 49 | 100 | 60 | 100 | 4 | — | 95 | 95 |
| autoremesher | 1343 | **53** | 0 | 82 | 93 | 65 | 100 | — | — | 0 | 95 |
| quadriflow | 2698 | **48** | 60 | 45 | 84 | 13 | 100 | 4 | 0 | 60 | 87 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3935 | **98** | 100 | 100 | 100 | 88 | 100 | 83 | 100 | 95 | 58 |
| quadremesher-as-made | 3889 | **94** | 100 | 100 | 100 | 84 | 100 | 48 | — | 100 | 35 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 8160 | **98** | 100 | 100 | 100 | 96 | 100 | 92 | 100 | 92 | 46 |
| integer-grid | 7057 | **91** | 100 | 100 | 95 | 91 | 100 | 58 | — | 53 | 27 |
| **openclare** | 7884 | **90** | 100 | 100 | 100 | 87 | 100 | 75 | 50 | 94 | 46 |
| quadremesher-uniform | 6943 | **89** | 100 | 100 | 100 | 84 | 100 | 29 | — | 47 | 37 |
| quadremesher | 8513 | **87** | 100 | 100 | 100 | 91 | 100 | 27 | 50 | 74 | 33 |
| instant-meshes | 8100 | **84** | 100 | 54 | 100 | 88 | 100 | 2 | — | 95 | 96 |
| autoremesher | 3438 | **82** | 98 | 100 | 100 | 83 | 100 | — | 54 | 0 | 67 |
| quadriflow | 7229 | **44** | 0 | 50 | 86 | 56 | 100 | 13 | 19 | 61 | 69 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 8601 | **96** | 100 | 100 | 100 | 93 | 100 | 85 | 82 | 96 | 41 |
| quadremesher-as-made | 8513 | **95** | 100 | 100 | 100 | 91 | 100 | 27 | — | 100 | 33 |
