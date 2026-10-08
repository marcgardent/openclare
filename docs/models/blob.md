# blob

An organic model; 124 004 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/blob__source.webp) | ![QuadriFlow (Blender)](img/blob__quadriflow.webp) | ![openclare](img/blob__quad-layout-adaptive.webp) | ![Quad Remesher](img/blob__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **83** | 100 | 80 | 71 | 83 | 84 | 20 | 96 | 88 |
| quad-layout | **81** | 100 | 77 | 68 | 72 | 84 | 35 | 90 | 85 |
| quadremesher | **76** | 100 | 85 | 66 | 85 | 87 | 62 | 0 | 36 |
| quadremesher-uniform | **76** | 100 | 78 | 68 | 62 | 79 | 51 | 51 | 37 |
| integer-grid | **69** | 100 | 66 | 52 | 57 | 82 | 31 | 47 | 68 |
| instant-meshes | **61** | 100 | 21 | 62 | 70 | 34 | 0 | 86 | 92 |
| quadriflow | **57** | 80 | 20 | 54 | 71 | 70 | 0 | 76 | 83 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **86** | 100 | 78 | 72 | 93 | 95 | 32 | 99 | 78 |
| quad-layout | **85** | 100 | 76 | 69 | 91 | 95 | 22 | 97 | 78 |
| quadremesher-uniform | **83** | 100 | 80 | 73 | 85 | 94 | 51 | 67 | 39 |
| quadremesher | **83** | 100 | 83 | 71 | 92 | 95 | 46 | 51 | 32 |
| integer-grid | **77** | 100 | 72 | 60 | 85 | 93 | 62 | 34 | 63 |
| instant-meshes | **68** | 100 | 30 | 71 | 80 | 56 | 1 | 88 | 87 |
| quadriflow | **57** | 60 | 22 | 56 | 83 | 89 | 3 | 71 | 76 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quadremesher | **88** | 100 | 82 | 72 | 94 | 98 | 42 | 96 | 39 |
| **quad-layout-adaptive** | **86** | 100 | 76 | 71 | 96 | 98 | 15 | 99 | 53 |
| quad-layout | **85** | 100 | 74 | 69 | 95 | 98 | 16 | 97 | 56 |
| quadremesher-uniform | **84** | 100 | 80 | 72 | 93 | 98 | 59 | 52 | 38 |
| integer-grid | **80** | 100 | 74 | 63 | 94 | 98 | 21 | 56 | 37 |
| instant-meshes | **72** | 100 | 33 | 76 | 90 | 67 | 0 | 92 | 89 |
| quadriflow | **62** | 60 | 34 | 61 | 90 | 96 | 2 | 80 | 67 |
