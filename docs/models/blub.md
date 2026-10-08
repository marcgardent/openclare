# blub

An organic model; 106 640 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/blub__source.webp) | ![QuadriFlow (Blender)](img/blub__quadriflow.webp) | ![openclare](img/blub__quad-layout-adaptive.webp) | ![Quad Remesher](img/blub__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **78** | 100 | 62 | 75 | 61 | 68 | 59 | 99 | 93 |
| quad-layout | **77** | 100 | 62 | 76 | 57 | 67 | 67 | 93 | 92 |
| quadremesher | **72** | 100 | 78 | 66 | 84 | 68 | 49 | 0 | 35 |
| quadremesher-uniform | **67** | 100 | 69 | 58 | 41 | 50 | 51 | 57 | 38 |
| instant-meshes | **45** | 60 | 24 | 62 | 26 | 26 | 2 | 94 | 95 |
| quadriflow | **42** | 60 | 27 | 50 | 22 | 62 | 7 | 39 | 88 |
| integer-grid | **37** | 0 | 58 | 57 | 22 | 55 | — | 65 | 74 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **84** | 100 | 68 | 76 | 83 | 85 | 65 | 99 | 71 |
| quad-layout | **83** | 100 | 65 | 74 | 79 | 88 | 57 | 99 | 82 |
| quadremesher-uniform | **76** | 100 | 69 | 64 | 77 | 86 | 24 | 60 | 39 |
| quadremesher | **75** | 100 | 77 | 68 | 90 | 88 | 48 | 0 | 35 |
| instant-meshes | **67** | 100 | 31 | 72 | 69 | 51 | 4 | 95 | 94 |
| integer-grid | **48** | 0 | 63 | 57 | 71 | 83 | — | 54 | 57 |
| quadriflow | **45** | 60 | 29 | 46 | 19 | 84 | 4 | 60 | 82 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **86** | 100 | 71 | 70 | 93 | 94 | 63 | 98 | 53 |
| quad-layout | **85** | 100 | 69 | 67 | 91 | 95 | 58 | 100 | 55 |
| quadremesher | **82** | 100 | 72 | 68 | 91 | 95 | 27 | 74 | 33 |
| integer-grid | **78** | 100 | 68 | 56 | 89 | 94 | 58 | 53 | 31 |
| quadremesher-uniform | **76** | 100 | 66 | 62 | 84 | 94 | 29 | 47 | 37 |
| instant-meshes | **73** | 100 | 35 | 78 | 88 | 66 | 2 | 95 | 92 |
| quadriflow | **38** | 0 | 32 | 48 | 56 | 91 | 13 | 61 | 74 |
