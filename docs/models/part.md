# part

A mechanical model, with hard edges; 406 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/part__source.webp) | ![QuadriFlow (Blender)](img/part__quadriflow.webp) | ![openclare](img/part__quad-layout-adaptive.webp) | ![Quad Remesher](img/part__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **86** | 100 | 52 | 89 | 97 | 84 | 93 | 93 | 97 |
| quadremesher | **82** | 100 | 51 | 78 | 97 | 82 | 64 | 95 | 73 |
| quad-layout | **80** | 100 | 45 | 90 | 70 | 84 | 91 | 90 | 93 |
| quadremesher-uniform | **80** | 100 | 51 | 77 | 95 | 83 | 68 | 70 | 76 |
| quadriflow | **77** | 100 | 40 | 81 | 84 | 77 | 76 | 77 | 100 |
| integer-grid | **74** | 100 | 43 | 76 | 60 | 83 | 60 | 84 | 43 |
| instant-meshes | **60** | 100 | 35 | 57 | 59 | 0 | 39 | 86 | 100 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **88** | 100 | 53 | 89 | 99 | 95 | 90 | 97 | 83 |
| quad-layout | **87** | 100 | 47 | 93 | 98 | 95 | 97 | 93 | 85 |
| integer-grid | **86** | 100 | 48 | 87 | 98 | 95 | 97 | 97 | 62 |
| quadremesher | **84** | 100 | 52 | 85 | 98 | 94 | 82 | 75 | 75 |
| quadremesher-uniform | **84** | 100 | 52 | 85 | 98 | 94 | 82 | 75 | 74 |
| quadriflow | **77** | 80 | 42 | 87 | 89 | 93 | 79 | 85 | 74 |
| instant-meshes | **70** | 100 | 38 | 69 | 81 | 33 | 39 | 92 | 100 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **89** | 100 | 54 | 95 | 99 | 98 | 99 | 95 | 59 |
| quad-layout | **88** | 100 | 47 | 94 | 99 | 98 | 95 | 96 | 59 |
| integer-grid | **87** | 100 | 47 | 92 | 99 | 98 | 98 | 91 | 33 |
| quadremesher | **83** | 100 | 52 | 82 | 99 | 98 | 90 | 68 | 76 |
| quadremesher-uniform | **83** | 100 | 52 | 82 | 99 | 98 | 90 | 68 | 76 |
| instant-meshes | **74** | 100 | 40 | 77 | 90 | 57 | 39 | 87 | 78 |
| quadriflow | **73** | 60 | 41 | 87 | 90 | 96 | 85 | 99 | 54 |
