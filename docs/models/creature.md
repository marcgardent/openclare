# creature

An organic model; 78 124 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/creature__source.webp) | ![QuadriFlow (Blender)](img/creature__quadriflow.webp) | ![openclare](img/creature__openclare.webp) | ![Quad Remesher](img/creature__quadremesher.webp) |
| 78 124 triangles | 2 779 quads | 2 955 quads | 2 941 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/creature__quadremesher.webp) | ![openclare, as many quads](img/creature__openclare-matched.webp) |
| 2 941 quads | 2 894 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1001 | **85** | 100 | 75 | 72 | 94 | 68 | 65 | 100 | 74 |
| quadremesher-uniform | 928 | **80** | 100 | 79 | 65 | 87 | 67 | 63 | 71 | 47 |
| openclare-uniform | 965 | **80** | 100 | 64 | 75 | 90 | 81 | 20 | 86 | 84 |
| quadremesher | 1208 | **76** | 100 | 79 | 68 | 93 | 72 | 51 | 17 | 39 |
| instant-meshes | 994 | **70** | 100 | 47 | 69 | 62 | 57 | 17 | 94 | 99 |
| integer-grid | 920 | **49** | 0 | 64 | 63 | 74 | 74 | 28 | 68 | 62 |
| quadriflow | 812 | **47** | 80 | 34 | 37 | 19 | 90 | 0 | 25 | 97 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1207 | **84** | 100 | 74 | 74 | 93 | 73 | 61 | 100 | 88 |
| quadremesher-as-made | 1208 | **83** | 100 | 79 | 68 | 93 | 72 | 51 | 100 | 39 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 2941 | **86** | 100 | 77 | 69 | 95 | 88 | 50 | 92 | 41 |
| openclare-uniform | 3041 | **85** | 100 | 69 | 71 | 96 | 89 | 57 | 95 | 75 |
| **openclare** | 2955 | **84** | 100 | 74 | 75 | 96 | 89 | 4 | 94 | 76 |
| quadremesher-uniform | 2719 | **82** | 100 | 76 | 67 | 95 | 86 | 52 | 63 | 46 |
| integer-grid | 2597 | **79** | 100 | 69 | 59 | 96 | 87 | 65 | 46 | 56 |
| instant-meshes | 3070 | **76** | 100 | 46 | 77 | 93 | 66 | 7 | 91 | 100 |
| quadriflow | 2667 | **71** | 100 | 43 | 66 | 84 | 94 | 0 | 56 | 87 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 2941 | **85** | 100 | 77 | 69 | 95 | 88 | 50 | 100 | 41 |
| openclare-matched | 2894 | **85** | 100 | 75 | 74 | 96 | 89 | 37 | 94 | 80 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7825 | **87** | 100 | 75 | 72 | 98 | 96 | 59 | 91 | 38 |
| openclare-uniform | 7884 | **87** | 100 | 73 | 66 | 98 | 96 | 64 | 94 | 43 |
| quadremesher | 7186 | **82** | 100 | 70 | 69 | 98 | 96 | 55 | 59 | 38 |
| quadremesher-uniform | 7207 | **82** | 100 | 70 | 69 | 97 | 96 | 52 | 60 | 48 |
| instant-meshes | 8234 | **78** | 100 | 47 | 80 | 97 | 75 | 9 | 88 | 89 |
| quadriflow | 8170 | **71** | 80 | 39 | 70 | 94 | 98 | 0 | 92 | 59 |
| integer-grid | 6922 | **54** | 0 | 72 | 56 | 95 | 94 | 64 | 46 | 22 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 7168 | **86** | 100 | 75 | 74 | 98 | 96 | 40 | 99 | 46 |
| quadremesher-as-made | 7186 | **85** | 100 | 70 | 69 | 98 | 96 | 55 | 100 | 38 |
