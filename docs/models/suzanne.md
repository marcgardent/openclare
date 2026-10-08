# suzanne

An organic model, with open borders; 15 744 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/suzanne__source.webp) | ![QuadriFlow (Blender)](img/suzanne__quadriflow.webp) | ![openclare](img/suzanne__quad-layout-adaptive.webp) | ![Quad Remesher](img/suzanne__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **75** | 100 | 55 | 72 | 76 | 52 | 44 | 98 | 84 |
| quad-layout | **73** | 100 | 50 | 76 | 62 | 56 | 40 | 99 | 100 |
| quadremesher-uniform | **65** | 100 | 57 | 57 | 65 | 26 | 13 | 69 | 68 |
| quadremesher | **65** | 100 | 62 | 56 | 79 | 62 | 14 | 0 | 63 |
| integer-grid | **37** | 0 | 47 | 54 | 26 | 48 | 25 | 90 | 65 |
| quadriflow | **28** | 0 | 30 | 52 | 10 | 33 | 1 | 96 | 100 |
| instant-meshes | **20** | 0 | 21 | 49 | 0 | 0 | 3 | 83 | 100 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **80** | 100 | 56 | 78 | 82 | 81 | 33 | 98 | 73 |
| quad-layout | **79** | 100 | 54 | 79 | 82 | 82 | 17 | 97 | 83 |
| integer-grid | **74** | 100 | 54 | 64 | 77 | 81 | 33 | 68 | 49 |
| quadremesher-uniform | **71** | 100 | 54 | 61 | 81 | 73 | 30 | 50 | 72 |
| instant-meshes | **46** | 40 | 24 | 65 | 60 | 31 | 2 | 91 | 100 |
| quadremesher | **45** | 0 | 61 | 61 | 86 | 81 | 42 | 0 | 70 |
| quadriflow | **35** | 0 | 32 | 57 | 15 | 82 | 5 | 96 | 71 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **82** | 100 | 56 | 73 | 90 | 92 | 56 | 98 | 62 |
| **quad-layout-adaptive** | **81** | 100 | 57 | 70 | 92 | 90 | 41 | 95 | 41 |
| quadremesher-uniform | **78** | 100 | 56 | 65 | 90 | 90 | 54 | 66 | 61 |
| integer-grid | **77** | 100 | 56 | 62 | 89 | 91 | 42 | 69 | 0 |
| quadremesher | **77** | 100 | 62 | 65 | 93 | 92 | 56 | 38 | 50 |
| quadriflow | **76** | 100 | 37 | 70 | 86 | 93 | 22 | 97 | 54 |
| instant-meshes | **70** | 100 | 26 | 74 | 88 | 57 | 3 | 95 | 100 |
