# lblock

A mechanical model, with hard edges; 4 032 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/lblock__source.webp) | ![QuadriFlow (Blender)](img/lblock__quadriflow.webp) | ![openclare](img/lblock__openclare.webp) | ![Quad Remesher](img/lblock__quadremesher.webp) |
| 4 032 triangles | 2 928 quads | 2 972 quads | 3 030 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/lblock__quadremesher.webp) | ![openclare, as many quads](img/lblock__openclare-matched.webp) |
| 3 030 quads | 2 972 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 994 | **99** | 100 | 98 | 100 | 100 | 96 | 99 | 98 | 93 |
| **openclare** | 994 | **98** | 100 | 97 | 98 | 100 | 96 | 99 | 98 | 96 |
| quadremesher | 1010 | **98** | 100 | 98 | 98 | 100 | 96 | 99 | 96 | 75 |
| quadremesher-uniform | 1010 | **98** | 100 | 98 | 98 | 100 | 96 | 99 | 96 | 76 |
| instant-meshes | 902 | **93** | 100 | 95 | 93 | 100 | 91 | 91 | 61 | 100 |
| integer-grid | 992 | **92** | 100 | 92 | 90 | 73 | 96 | 97 | 97 | 73 |
| quadriflow | 930 | **91** | 100 | 94 | 94 | 81 | 96 | 91 | 72 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1010 | **99** | 100 | 98 | 98 | 100 | 96 | 99 | 100 | 75 |
| openclare-matched | 1028 | **99** | 100 | 97 | 98 | 100 | 96 | 99 | 93 | 97 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 3030 | **99** | 100 | 97 | 98 | 100 | 99 | 100 | 96 | 75 |
| quadremesher-uniform | 3030 | **99** | 100 | 97 | 98 | 100 | 99 | 100 | 96 | 74 |
| openclare-uniform | 3060 | **99** | 100 | 98 | 100 | 100 | 99 | 100 | 92 | 86 |
| **openclare** | 2972 | **98** | 100 | 96 | 97 | 100 | 99 | 100 | 96 | 82 |
| integer-grid | 2912 | **98** | 100 | 97 | 97 | 100 | 99 | 100 | 88 | 2 |
| instant-meshes | 2780 | **93** | 100 | 93 | 92 | 100 | 94 | 83 | 71 | 100 |
| quadriflow | 2928 | **89** | 80 | 95 | 96 | 79 | 99 | 97 | 90 | 92 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 3030 | **99** | 100 | 97 | 98 | 100 | 99 | 100 | 100 | 75 |
| openclare-matched | 2972 | **99** | 100 | 96 | 97 | 100 | 99 | 100 | 92 | 86 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 8064 | **99** | 100 | 98 | 100 | 100 | 100 | 100 | 97 | 72 |
| quadremesher-uniform | 8064 | **99** | 100 | 98 | 100 | 100 | 100 | 100 | 97 | 72 |
| integer-grid | 8064 | **99** | 100 | 97 | 100 | 100 | 100 | 100 | 97 | 51 |
| openclare-uniform | 8064 | **99** | 100 | 97 | 100 | 100 | 100 | 100 | 97 | 62 |
| **openclare** | 8064 | **98** | 100 | 96 | 98 | 100 | 100 | 100 | 97 | 62 |
| instant-meshes | 8000 | **98** | 100 | 95 | 96 | 100 | 98 | 91 | 100 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 8064 | **99** | 100 | 98 | 100 | 100 | 100 | 100 | 100 | 72 |
| openclare-matched | 8064 | **99** | 100 | 96 | 98 | 100 | 100 | 100 | 100 | 62 |
