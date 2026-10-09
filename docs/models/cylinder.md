# cylinder

A mechanical model, with hard edges; 3 584 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/cylinder__source.webp) | ![QuadriFlow (Blender)](img/cylinder__quadriflow.webp) | ![openclare](img/cylinder__openclare.webp) | ![Quad Remesher](img/cylinder__quadremesher.webp) |
| 3 584 triangles | 2 892 quads | 3 040 quads | 2 458 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/cylinder__quadremesher.webp) | ![openclare, as many quads](img/cylinder__openclare-matched.webp) |
| 2 458 quads | 2 494 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 990 | **96** | 100 | 95 | 87 | 97 | 100 | 99 | 96 | 100 |
| **openclare** | 968 | **93** | 100 | 92 | 81 | 97 | 100 | 81 | 87 | 92 |
| quadriflow | 995 | **89** | 100 | 82 | 78 | 92 | 100 | 62 | 93 | 100 |
| integer-grid | 892 | **89** | 100 | 91 | 78 | 97 | 100 | 80 | 57 | 85 |
| quadremesher | 782 | **85** | 100 | 93 | 80 | 95 | 100 | 82 | 13 | 74 |
| quadremesher-uniform | 782 | **85** | 100 | 93 | 80 | 95 | 100 | 82 | 13 | 74 |
| instant-meshes | 994 | **76** | 80 | 78 | 75 | 69 | 68 | 42 | 96 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 764 | **95** | 100 | 94 | 87 | 96 | 100 | 93 | 91 | 100 |
| quadremesher-as-made | 782 | **93** | 100 | 93 | 80 | 95 | 100 | 82 | 100 | 74 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3040 | **95** | 100 | 93 | 86 | 98 | 100 | 85 | 95 | 83 |
| openclare-uniform | 2984 | **94** | 100 | 93 | 82 | 98 | 100 | 76 | 98 | 87 |
| quadriflow | 2900 | **89** | 100 | 81 | 78 | 96 | 100 | 65 | 87 | 86 |
| integer-grid | 2640 | **88** | 100 | 93 | 76 | 98 | 98 | 72 | 52 | 72 |
| quadremesher | 2458 | **88** | 100 | 93 | 84 | 97 | 100 | 83 | 28 | 76 |
| quadremesher-uniform | 2458 | **88** | 100 | 93 | 84 | 97 | 100 | 83 | 28 | 78 |
| instant-meshes | 3100 | **82** | 100 | 76 | 79 | 93 | 73 | 2 | 87 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 2458 | **94** | 100 | 93 | 84 | 97 | 100 | 83 | 100 | 76 |
| openclare-matched | 2494 | **94** | 100 | 92 | 83 | 98 | 100 | 81 | 94 | 96 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 8006 | **97** | 100 | 94 | 88 | 99 | 100 | 98 | 100 | 59 |
| **openclare** | 7899 | **96** | 100 | 94 | 89 | 99 | 100 | 82 | 95 | 56 |
| integer-grid | 7324 | **93** | 100 | 94 | 87 | 99 | 100 | 82 | 66 | 49 |
| quadremesher | 6922 | **89** | 100 | 92 | 78 | 99 | 100 | 88 | 46 | 64 |
| quadremesher-uniform | 6922 | **89** | 100 | 92 | 78 | 99 | 100 | 88 | 46 | 63 |
| instant-meshes | 8024 | **84** | 80 | 85 | 85 | 96 | 80 | 38 | 98 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 7007 | **96** | 100 | 94 | 89 | 99 | 100 | 82 | 95 | 66 |
| quadremesher-as-made | 6922 | **94** | 100 | 92 | 78 | 99 | 100 | 88 | 100 | 64 |
