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

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **81** | 100 | 70 | 73 | 78 | 64 | 55 | 97 | 74 |
| quad-layout | **80** | 100 | 69 | 80 | 65 | 64 | 72 | 99 | 81 |
| integer-grid | **73** | 100 | 64 | 62 | 55 | 70 | 81 | 65 | 57 |
| quadremesher | **70** | 100 | 71 | 60 | 89 | 64 | 36 | 0 | 37 |
| quadremesher-uniform | **44** | 0 | 60 | 51 | 73 | 42 | — | 74 | 40 |
| instant-meshes | **42** | 80 | 40 | 57 | 0 | 2 | 37 | 38 | 84 |
| quadriflow | **0** | 0 | — | — | — | — | — | — | 100 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **85** | 100 | 67 | 81 | 87 | 84 | 74 | 95 | 70 |
| quad-layout | **84** | 100 | 64 | 81 | 86 | 83 | 64 | 94 | 67 |
| integer-grid | **78** | 100 | 63 | 68 | 86 | 81 | 72 | 58 | 47 |
| instant-meshes | **66** | 100 | 44 | 69 | 44 | 48 | 35 | 87 | 73 |
| quadremesher-uniform | **52** | 0 | 65 | 64 | 86 | 77 | — | 63 | 40 |
| quadremesher | **48** | 0 | 71 | 67 | 88 | 82 | — | 0 | 41 |
| quadriflow | **0** | 0 | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **83** | 100 | 60 | 77 | 87 | 90 | 66 | 95 | 55 |
| quad-layout | **82** | 100 | 56 | 76 | 87 | 91 | 62 | 94 | 54 |
| quadremesher | **79** | 100 | 63 | 69 | 87 | 91 | 67 | 60 | 33 |
| integer-grid | **79** | 100 | 56 | 65 | 86 | 91 | 78 | 69 | 22 |
| instant-meshes | **75** | 100 | 38 | 78 | 82 | 68 | 35 | 98 | 85 |
| quadremesher-uniform | **52** | 0 | 59 | 66 | 87 | 89 | — | 53 | 40 |
| quadriflow | **0** | 0 | — | — | — | — | — | — | 100 |
