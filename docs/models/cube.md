# cube

A mechanical model, with hard edges; 3 072 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/cube__source.webp) | ![QuadriFlow (Blender)](img/cube__quadriflow.webp) | ![openclare](img/cube__quad-layout-adaptive.webp) | ![Quad Remesher](img/cube__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quadremesher | **99** | 100 | 100 | 100 | 100 | 100 | 99 | 94 | 79 |
| quadremesher-uniform | **99** | 100 | 100 | 100 | 100 | 100 | 99 | 94 | 73 |
| quad-layout | **99** | 100 | 100 | 100 | 100 | 100 | 99 | 94 | 100 |
| **quad-layout-adaptive** | **99** | 100 | 99 | 98 | 100 | 100 | 99 | 94 | 100 |
| quadriflow | **99** | 100 | 99 | 100 | 95 | 100 | 99 | 94 | 100 |
| instant-meshes | **91** | 100 | 94 | 90 | 100 | 91 | 81 | 60 | 100 |
| integer-grid | **84** | 100 | 93 | 87 | 48 | 100 | 0 | 100 | 78 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **100** | 100 | 99 | 100 | 100 | 100 | 100 | 99 | 93 |
| **quad-layout-adaptive** | **99** | 100 | 98 | 98 | 100 | 100 | 100 | 99 | 95 |
| quadremesher | **99** | 100 | 99 | 100 | 100 | 100 | 100 | 89 | 73 |
| quadremesher-uniform | **99** | 100 | 99 | 100 | 100 | 100 | 100 | 89 | 73 |
| quadriflow | **97** | 100 | 99 | 99 | 90 | 100 | 100 | 87 | 100 |
| instant-meshes | **96** | 100 | 91 | 92 | 100 | 95 | 91 | 96 | 100 |
| integer-grid | **93** | 100 | 97 | 96 | 78 | 100 | 63 | 99 | 58 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **100** | 100 | 99 | 100 | 100 | 100 | 100 | 97 | 70 |
| quadremesher | **99** | 100 | 100 | 100 | 100 | 100 | 100 | 96 | 69 |
| quadremesher-uniform | **99** | 100 | 100 | 100 | 100 | 100 | 100 | 96 | 74 |
| **quad-layout-adaptive** | **99** | 100 | 99 | 99 | 100 | 100 | 100 | 97 | 69 |
| quadriflow | **99** | 100 | 98 | 98 | 98 | 100 | 100 | 96 | 67 |
| instant-meshes | **97** | 100 | 96 | 95 | 100 | 99 | 95 | 90 | 100 |
| integer-grid | **91** | 100 | 97 | 95 | 86 | 100 | 0 | 99 | 0 |
