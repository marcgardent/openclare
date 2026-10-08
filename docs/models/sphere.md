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

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **88** | 100 | — | 77 | 97 | 100 | 0 | 96 | 90 |
| quad-layout | **88** | 100 | — | 77 | 97 | 100 | 0 | 96 | 100 |
| quadriflow | **87** | 100 | — | 77 | 97 | 100 | 1 | 80 | 100 |
| quadremesher | **84** | 100 | — | 70 | 96 | 98 | 24 | 66 | 62 |
| quadremesher-uniform | **84** | 100 | — | 70 | 96 | 98 | 24 | 66 | 68 |
| integer-grid | **81** | 100 | — | 58 | 96 | 100 | 0 | 69 | 95 |
| instant-meshes | **81** | 100 | — | 74 | 97 | 66 | 0 | 74 | 100 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **95** | 100 | — | 80 | 98 | 92 | 99 | 100 | 91 |
| **quad-layout-adaptive** | **95** | 100 | — | 80 | 98 | 92 | 99 | 100 | 96 |
| integer-grid | **91** | 100 | — | 58 | 99 | 92 | 99 | 100 | 89 |
| quadriflow | **88** | 100 | — | 78 | 99 | 100 | 12 | 86 | 86 |
| quadremesher | **85** | 100 | — | 70 | 98 | 99 | 35 | 65 | 69 |
| quadremesher-uniform | **85** | 100 | — | 70 | 98 | 99 | 35 | 65 | 70 |
| instant-meshes | **84** | 100 | — | 79 | 99 | 71 | 0 | 88 | 100 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **88** | 100 | — | 75 | 99 | 98 | 0 | 98 | 62 |
| **quad-layout-adaptive** | **88** | 100 | — | 75 | 99 | 98 | 0 | 98 | 50 |
| instant-meshes | **86** | 100 | — | 83 | 99 | 77 | 0 | 90 | 100 |
| integer-grid | **84** | 100 | — | 65 | 99 | 98 | 0 | 76 | 34 |
| quadremesher | **83** | 100 | — | 62 | 99 | 100 | 30 | 54 | 65 |
| quadremesher-uniform | **83** | 100 | — | 62 | 99 | 100 | 30 | 54 | 65 |
| quadriflow | **75** | 60 | — | 75 | 99 | 99 | 0 | 87 | 62 |
