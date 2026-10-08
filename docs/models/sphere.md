# sphere

A calibration model; 5 120 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/sphere__source.webp) | ![QuadriFlow (Blender)](img/sphere__quadriflow.webp) | ![openclare](img/sphere__quad-layout-adaptive.webp) | ![Quad Remesher](img/sphere__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 1010 | **89** | 100 | — | 77 | 98 | 100 | 0 | 96 | 92 |
| quad-layout | 1010 | **89** | 100 | — | 77 | 98 | 100 | 0 | 96 | 100 |
| quadriflow | 950 | **87** | 100 | — | 77 | 97 | 100 | 1 | 80 | 100 |
| quadremesher | 915 | **84** | 100 | — | 70 | 96 | 98 | 24 | 66 | 62 |
| quadremesher-uniform | 915 | **84** | 100 | — | 70 | 96 | 98 | 24 | 66 | 68 |
| integer-grid | 923 | **82** | 100 | — | 58 | 98 | 100 | 0 | 69 | 83 |
| instant-meshes | 1066 | **81** | 100 | — | 74 | 97 | 66 | 0 | 74 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 900 | **88** | 100 | — | 80 | 96 | 98 | 6 | 93 | 90 |
| quadremesher-as-made | 915 | **87** | 100 | — | 70 | 96 | 98 | 24 | 100 | 62 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout | 3000 | **95** | 100 | — | 80 | 99 | 92 | 99 | 100 | 99 |
| **quad-layout-adaptive** | 3000 | **95** | 100 | — | 80 | 99 | 92 | 99 | 100 | 97 |
| integer-grid | 3000 | **91** | 100 | — | 58 | 99 | 92 | 99 | 100 | 72 |
| quadriflow | 3107 | **88** | 100 | — | 78 | 99 | 100 | 12 | 86 | 86 |
| quadremesher | 2740 | **85** | 100 | — | 70 | 98 | 99 | 35 | 65 | 69 |
| quadremesher-uniform | 2740 | **85** | 100 | — | 70 | 98 | 99 | 35 | 65 | 70 |
| instant-meshes | 3090 | **84** | 100 | — | 79 | 99 | 71 | 0 | 88 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 2706 | **94** | 100 | — | 81 | 99 | 91 | 99 | 95 | 95 |
| quadremesher-as-made | 2740 | **88** | 100 | — | 70 | 98 | 99 | 35 | 100 | 69 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout | 8035 | **88** | 100 | — | 75 | 99 | 98 | 0 | 98 | 60 |
| **quad-layout-adaptive** | 8031 | **88** | 100 | — | 75 | 99 | 98 | 0 | 98 | 56 |
| instant-meshes | 8190 | **86** | 100 | — | 83 | 99 | 77 | 0 | 90 | 100 |
| integer-grid | 7528 | **84** | 100 | — | 65 | 99 | 98 | 0 | 76 | 29 |
| quadremesher | 7088 | **83** | 100 | — | 62 | 99 | 100 | 30 | 54 | 65 |
| quadremesher-uniform | 7088 | **83** | 100 | — | 62 | 99 | 100 | 30 | 54 | 65 |
| quadriflow | 7736 | **75** | 60 | — | 75 | 99 | 99 | 0 | 87 | 58 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 7022 | **87** | 100 | — | 74 | 99 | 99 | 0 | 96 | 60 |
| quadremesher-as-made | 7088 | **87** | 100 | — | 62 | 99 | 100 | 30 | 100 | 65 |
