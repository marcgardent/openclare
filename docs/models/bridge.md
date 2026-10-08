# bridge

An organic model, with open borders; 164 064 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | openclare | Quad Remesher |
|---|---|---|
| ![Source](img/bridge__source.webp) | ![openclare](img/bridge__quad-layout-adaptive.webp) | ![Quad Remesher](img/bridge__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout | 1003 | **82** | 100 | 69 | 79 | 78 | 64 | 72 | 99 | 76 |
| **quad-layout-adaptive** | 1008 | **82** | 100 | 70 | 72 | 88 | 64 | 55 | 97 | 80 |
| integer-grid | 912 | **76** | 100 | 64 | 61 | 73 | 70 | 81 | 65 | 56 |
| quadremesher | 2105 | **70** | 100 | 71 | 60 | 91 | 64 | 36 | 0 | 37 |
| quadremesher-uniform | 936 | **46** | 0 | 60 | 51 | 81 | 42 | — | 74 | 40 |
| instant-meshes | 845 | **43** | 80 | 40 | 57 | 1 | 2 | 37 | 38 | 85 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 2120 | **83** | 100 | 67 | 78 | 84 | 83 | 72 | 97 | 76 |
| quadremesher-as-made | 2105 | **77** | 100 | 68 | 60 | 89 | 64 | 36 | 100 | 37 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 2959 | **85** | 100 | 67 | 80 | 87 | 84 | 74 | 95 | 65 |
| quad-layout | 2952 | **84** | 100 | 64 | 80 | 86 | 83 | 64 | 94 | 59 |
| integer-grid | 2682 | **78** | 100 | 63 | 67 | 86 | 81 | 72 | 58 | 49 |
| instant-meshes | 2903 | **66** | 100 | 44 | 69 | 44 | 48 | 35 | 87 | 73 |
| quadremesher-uniform | 2726 | **52** | 0 | 65 | 64 | 86 | 77 | — | 63 | 40 |
| quadremesher | 4533 | **48** | 0 | 71 | 67 | 88 | 82 | — | 0 | 41 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 4537 | **83** | 100 | 64 | 77 | 87 | 88 | 64 | 100 | 60 |
| quadremesher-as-made | 4533 | **53** | 0 | 67 | 67 | 88 | 82 | — | 100 | 41 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 8109 | **83** | 100 | 60 | 77 | 87 | 90 | 66 | 95 | 51 |
| quad-layout | 7880 | **82** | 100 | 56 | 76 | 86 | 91 | 62 | 94 | 57 |
| quadremesher | 8805 | **79** | 100 | 63 | 69 | 87 | 91 | 67 | 60 | 33 |
| integer-grid | 7388 | **79** | 100 | 56 | 65 | 86 | 91 | 78 | 69 | 18 |
| instant-meshes | 7960 | **75** | 100 | 38 | 78 | 82 | 68 | 35 | 98 | 81 |
| quadremesher-uniform | 7054 | **52** | 0 | 59 | 66 | 87 | 89 | — | 53 | 40 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 8841 | **81** | 100 | 59 | 78 | 85 | 90 | 60 | 98 | 49 |
| quadremesher-as-made | 8805 | **81** | 100 | 61 | 69 | 87 | 91 | 67 | 100 | 33 |
