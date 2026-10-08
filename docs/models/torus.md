# torus

A calibration model; 9 216 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/torus__source.webp) | ![QuadriFlow (Blender)](img/torus__quadriflow.webp) | ![openclare](img/torus__quad-layout-adaptive.webp) | ![Quad Remesher](img/torus__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **97** | 100 | 99 | 88 | 96 | 100 | 100 | 97 | 99 |
| quad-layout | **97** | 100 | 99 | 88 | 96 | 100 | 100 | 97 | 89 |
| integer-grid | **94** | 100 | 97 | 87 | 96 | 100 | 100 | 68 | 100 |
| quadremesher | **94** | 100 | 97 | 88 | 95 | 100 | 100 | 68 | 64 |
| quadremesher-uniform | **94** | 100 | 97 | 88 | 95 | 100 | 100 | 68 | 77 |
| quadriflow | **73** | 100 | 39 | 61 | 94 | 81 | 1 | 89 | 100 |
| instant-meshes | **70** | 100 | 61 | 57 | 92 | 15 | 12 | 78 | 100 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **97** | 100 | 97 | 93 | 98 | 100 | 100 | 94 | 84 |
| quad-layout | **97** | 100 | 97 | 93 | 98 | 100 | 100 | 94 | 90 |
| integer-grid | **94** | 100 | 97 | 93 | 98 | 100 | 100 | 63 | 82 |
| quadremesher | **83** | 100 | 69 | 74 | 97 | 96 | 20 | 77 | 62 |
| quadremesher-uniform | **83** | 100 | 69 | 74 | 97 | 96 | 20 | 77 | 74 |
| instant-meshes | **73** | 100 | 37 | 72 | 97 | 59 | 0 | 90 | 100 |
| quadriflow | **56** | 20 | 55 | 49 | 96 | 90 | 0 | 97 | 89 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **98** | 100 | 97 | 95 | 99 | 100 | 100 | 99 | 66 |
| quad-layout | **98** | 100 | 97 | 95 | 99 | 100 | 100 | 99 | 69 |
| integer-grid | **95** | 100 | 97 | 95 | 99 | 100 | 100 | 64 | 54 |
| quadremesher | **86** | 100 | 81 | 82 | 99 | 97 | 19 | 70 | 55 |
| quadremesher-uniform | **86** | 100 | 81 | 82 | 99 | 97 | 19 | 70 | 57 |
| instant-meshes | **82** | 100 | 72 | 74 | 99 | 67 | 1 | 98 | 100 |
| quadriflow | **58** | 40 | 49 | 63 | 99 | 98 | 1 | 43 | 54 |
