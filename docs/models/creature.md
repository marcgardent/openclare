# creature

An organic model; 78 124 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/creature__source.webp) | ![QuadriFlow (Blender)](img/creature__quadriflow.webp) | ![openclare](img/creature__quad-layout-adaptive.webp) | ![Quad Remesher](img/creature__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **84** | 100 | 75 | 73 | 90 | 68 | 65 | 100 | 87 |
| quadremesher-uniform | **80** | 100 | 79 | 65 | 87 | 67 | 63 | 71 | 47 |
| quad-layout | **80** | 100 | 64 | 76 | 84 | 81 | 20 | 86 | 87 |
| quadremesher | **76** | 100 | 79 | 68 | 93 | 72 | 51 | 17 | 39 |
| instant-meshes | **70** | 100 | 47 | 69 | 62 | 57 | 17 | 94 | 100 |
| integer-grid | **49** | 0 | 64 | 64 | 74 | 74 | 28 | 68 | 69 |
| quadriflow | **47** | 80 | 34 | 37 | 19 | 90 | 0 | 25 | 96 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quadremesher | **86** | 100 | 77 | 69 | 95 | 88 | 50 | 92 | 41 |
| quad-layout | **85** | 100 | 69 | 72 | 95 | 89 | 57 | 95 | 80 |
| **quad-layout-adaptive** | **84** | 100 | 74 | 75 | 95 | 89 | 4 | 94 | 88 |
| quadremesher-uniform | **82** | 100 | 76 | 67 | 95 | 86 | 52 | 63 | 46 |
| integer-grid | **78** | 100 | 69 | 59 | 94 | 87 | 65 | 46 | 59 |
| instant-meshes | **76** | 100 | 46 | 77 | 93 | 66 | 7 | 91 | 100 |
| quadriflow | **71** | 100 | 43 | 66 | 84 | 94 | 0 | 56 | 88 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **87** | 100 | 75 | 72 | 97 | 96 | 59 | 91 | 37 |
| quad-layout | **86** | 100 | 73 | 66 | 97 | 96 | 64 | 94 | 42 |
| quadremesher | **82** | 100 | 70 | 69 | 98 | 96 | 55 | 59 | 38 |
| quadremesher-uniform | **82** | 100 | 70 | 69 | 97 | 96 | 52 | 60 | 48 |
| instant-meshes | **78** | 100 | 47 | 80 | 97 | 75 | 9 | 88 | 100 |
| quadriflow | **71** | 80 | 39 | 70 | 94 | 98 | 0 | 92 | 57 |
| integer-grid | **54** | 0 | 72 | 56 | 95 | 94 | 64 | 46 | 22 |
