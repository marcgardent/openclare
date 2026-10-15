# cylinder

A mechanical model, with hard edges; 3 584 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/cylinder__source.webp) | ![QuadriFlow (Blender)](img/cylinder__quadriflow.webp) | ![AutoRemesher](img/cylinder__autoremesher.webp) | ![openclare](img/cylinder__openclare.webp) | ![Quad Remesher](img/cylinder__quadremesher.webp) |
| 3 584 triangles | 2 892 quads | 1 306 quads | 3 040 quads | 2 458 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/cylinder__quadremesher.webp) | ![openclare, as many quads](img/cylinder__openclare-matched.webp) |
| 2 458 quads | 2 496 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 33 % | 17 % | 49 % | 0 % | 2 | 769 |
| AutoRemesher | 75 % | 0 % | 25 % | 0 % | 0 | 178 |
| openclare | 41 % | 0 % | 59 % | 0 % | 0 | 737 |
| Quad Remesher | 42 % | 39 % | 19 % | 0 % | 2 | 345 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 990 | **96** | 100 | 95 | 87 | 97 | 100 | 99 | 96 | 100 |
| **openclare** | 1002 | **94** | 100 | 92 | 81 | 97 | 100 | 81 | 99 | 100 |
| integer-grid | 900 | **91** | 100 | 93 | 82 | 97 | 100 | 95 | 60 | 33 |
| quadriflow | 995 | **89** | 100 | 82 | 78 | 92 | 100 | 62 | 93 | 100 |
| quadremesher | 782 | **85** | 100 | 93 | 80 | 95 | 100 | 82 | 13 | 74 |
| quadremesher-uniform | 782 | **85** | 100 | 93 | 80 | 95 | 100 | 82 | 13 | 74 |
| instant-meshes | 994 | **76** | 80 | 78 | 75 | 69 | 68 | 42 | 96 | 100 |
| autoremesher | 612 | **73** | 100 | 76 | 56 | 71 | 100 | 73 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 764 | **95** | 100 | 94 | 87 | 96 | 100 | 93 | 91 | 100 |
| quadremesher-as-made | 782 | **93** | 100 | 93 | 80 | 95 | 100 | 82 | 100 | 74 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3040 | **95** | 100 | 92 | 86 | 98 | 100 | 82 | 95 | 84 |
| openclare-uniform | 2988 | **94** | 100 | 93 | 82 | 98 | 100 | 76 | 98 | 85 |
| quadriflow | 2900 | **89** | 100 | 81 | 78 | 96 | 100 | 65 | 87 | 91 |
| quadremesher | 2458 | **88** | 100 | 93 | 84 | 97 | 100 | 83 | 28 | 76 |
| quadremesher-uniform | 2458 | **88** | 100 | 93 | 84 | 97 | 100 | 83 | 28 | 78 |
| integer-grid | 2533 | **88** | 100 | 93 | 79 | 98 | 100 | 75 | 38 | 74 |
| instant-meshes | 3100 | **82** | 100 | 76 | 79 | 93 | 73 | 2 | 87 | 100 |
| autoremesher | 1306 | **77** | 100 | 79 | 59 | 81 | 100 | 95 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 2496 | **95** | 100 | 94 | 87 | 98 | 100 | 82 | 94 | 84 |
| quadremesher-as-made | 2458 | **94** | 100 | 93 | 84 | 97 | 100 | 83 | 100 | 76 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7980 | **96** | 100 | 94 | 88 | 99 | 100 | 82 | 99 | 62 |
| **openclare** | 7899 | **96** | 100 | 94 | 88 | 99 | 100 | 82 | 95 | 60 |
| integer-grid | 7276 | **92** | 100 | 94 | 87 | 99 | 100 | 82 | 64 | 50 |
| quadremesher | 6922 | **89** | 100 | 92 | 78 | 99 | 100 | 88 | 46 | 64 |
| quadremesher-uniform | 6922 | **89** | 100 | 92 | 78 | 99 | 100 | 88 | 46 | 63 |
| instant-meshes | 8024 | **84** | 80 | 85 | 85 | 96 | 80 | 38 | 98 | 100 |
| autoremesher | 2565 | **52** | 0 | 82 | 67 | 89 | 96 | — | 0 | 92 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 7007 | **96** | 100 | 94 | 89 | 99 | 100 | 82 | 95 | 67 |
| quadremesher-as-made | 6922 | **94** | 100 | 92 | 78 | 99 | 100 | 88 | 100 | 64 |
