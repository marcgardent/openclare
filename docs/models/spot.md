# spot

An organic model; 100 250 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/spot__source.webp) | ![QuadriFlow (Blender)](img/spot__quadriflow.webp) | ![openclare](img/spot__quad-layout-adaptive.webp) | ![Quad Remesher](img/spot__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **73** | 100 | 66 | 65 | 68 | 39 | 27 | 96 | 77 |
| quad-layout | **72** | 100 | 58 | 64 | 75 | 43 | 22 | 96 | 76 |
| quadremesher | **67** | 100 | 72 | 59 | 83 | 53 | 28 | 0 | 30 |
| quadremesher-uniform | **66** | 100 | 68 | 55 | 69 | 23 | 29 | 53 | 36 |
| quadriflow | **61** | 100 | 19 | 62 | 59 | 47 | 6 | 89 | 91 |
| integer-grid | **60** | 100 | 51 | 44 | 46 | 44 | 18 | 63 | 66 |
| instant-meshes | **51** | 100 | 21 | 63 | 10 | 14 | 3 | 94 | 99 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **81** | 100 | 65 | 72 | 85 | 79 | 44 | 97 | 63 |
| quad-layout | **81** | 100 | 61 | 72 | 87 | 80 | 43 | 94 | 75 |
| quadremesher | **76** | 100 | 72 | 68 | 87 | 79 | 30 | 36 | 28 |
| quadremesher-uniform | **74** | 100 | 66 | 65 | 79 | 71 | 21 | 58 | 34 |
| integer-grid | **73** | 100 | 59 | 55 | 85 | 77 | 33 | 58 | 50 |
| quadriflow | **66** | 100 | 19 | 64 | 65 | 80 | 2 | 95 | 82 |
| instant-meshes | **64** | 100 | 24 | 72 | 61 | 52 | 2 | 94 | 95 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quadremesher | **84** | 100 | 66 | 72 | 91 | 91 | 42 | 98 | 30 |
| **quad-layout-adaptive** | **83** | 100 | 66 | 68 | 95 | 91 | 38 | 95 | 36 |
| quad-layout | **83** | 100 | 65 | 68 | 94 | 91 | 33 | 98 | 41 |
| quadremesher-uniform | **79** | 100 | 63 | 70 | 88 | 90 | 47 | 64 | 30 |
| integer-grid | **76** | 100 | 62 | 55 | 93 | 90 | 48 | 54 | 11 |
| instant-meshes | **71** | 100 | 25 | 78 | 86 | 66 | 2 | 95 | 97 |
| quadriflow | **67** | 80 | 25 | 69 | 89 | 91 | 1 | 94 | 71 |
