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

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 1018 | **86** | 100 | 52 | 89 | 97 | 84 | 93 | 93 | 85 |
| quadremesher | 1013 | **82** | 100 | 51 | 78 | 97 | 82 | 64 | 95 | 73 |
| quad-layout | 976 | **80** | 100 | 45 | 90 | 71 | 84 | 91 | 90 | 97 |
| quadremesher-uniform | 924 | **80** | 100 | 51 | 77 | 95 | 83 | 68 | 70 | 76 |
| quadriflow | 943 | **77** | 100 | 40 | 81 | 84 | 77 | 76 | 77 | 100 |
| integer-grid | 959 | **74** | 100 | 43 | 76 | 66 | 83 | 60 | 84 | 45 |
| instant-meshes | 1036 | **60** | 100 | 35 | 57 | 59 | 0 | 39 | 86 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 1018 | **85** | 100 | 52 | 89 | 97 | 84 | 93 | 98 | 100 |
| quadremesher-as-made | 1013 | **81** | 100 | 51 | 78 | 97 | 82 | 64 | 100 | 73 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 2978 | **88** | 100 | 53 | 89 | 99 | 95 | 90 | 97 | 79 |
| quad-layout | 3049 | **87** | 100 | 47 | 93 | 98 | 95 | 97 | 93 | 77 |
| integer-grid | 2976 | **86** | 100 | 48 | 87 | 98 | 95 | 97 | 97 | 66 |
| quadremesher | 2815 | **84** | 100 | 52 | 85 | 98 | 94 | 82 | 75 | 75 |
| quadremesher-uniform | 2815 | **84** | 100 | 52 | 85 | 98 | 94 | 82 | 75 | 74 |
| quadriflow | 2890 | **77** | 80 | 42 | 87 | 89 | 93 | 79 | 85 | 74 |
| instant-meshes | 2938 | **70** | 100 | 38 | 69 | 81 | 33 | 39 | 92 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 2828 | **87** | 100 | 53 | 91 | 99 | 94 | 94 | 98 | 73 |
| quadremesher-as-made | 2815 | **85** | 100 | 52 | 85 | 98 | 94 | 82 | 100 | 75 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 8098 | **89** | 100 | 54 | 95 | 99 | 98 | 99 | 95 | 51 |
| quad-layout | 8078 | **88** | 100 | 47 | 94 | 99 | 98 | 95 | 96 | 53 |
| integer-grid | 7824 | **87** | 100 | 47 | 92 | 99 | 98 | 98 | 91 | 42 |
| quadremesher | 7353 | **83** | 100 | 52 | 82 | 99 | 98 | 90 | 68 | 76 |
| quadremesher-uniform | 7353 | **83** | 100 | 52 | 82 | 99 | 98 | 90 | 68 | 76 |
| instant-meshes | 7732 | **74** | 100 | 40 | 77 | 90 | 57 | 39 | 87 | 80 |
| quadriflow | 7983 | **73** | 60 | 41 | 87 | 90 | 96 | 85 | 99 | 48 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 7279 | **88** | 100 | 53 | 94 | 99 | 98 | 96 | 96 | 58 |
| quadremesher-as-made | 7353 | **85** | 100 | 52 | 82 | 99 | 98 | 90 | 100 | 76 |
