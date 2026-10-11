# bob

An organic model; 102 756 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/bob__source.webp) | ![QuadriFlow (Blender)](img/bob__quadriflow.webp) | ![openclare](img/bob__openclare.webp) | ![Quad Remesher](img/bob__quadremesher.webp) |
| 102 756 triangles | 2 871 quads | 2 950 quads | 2 920 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/bob__quadremesher.webp) | ![openclare, as many quads](img/bob__openclare-matched.webp) |
| 2 920 quads | 2 947 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 993 | **88** | 100 | 81 | 78 | 90 | 82 | 76 | 97 | 94 |
| **openclare** | 991 | **87** | 100 | 81 | 75 | 90 | 80 | 67 | 96 | 90 |
| quadremesher | 954 | **85** | 100 | 86 | 73 | 85 | 78 | 58 | 82 | 38 |
| quadremesher-uniform | 893 | **82** | 100 | 84 | 69 | 85 | 76 | 74 | 57 | 32 |
| integer-grid | 861 | **78** | 100 | 81 | 73 | 62 | 79 | 79 | 44 | 74 |
| quadriflow | 972 | **64** | 100 | 11 | 60 | 75 | 71 | 1 | 89 | 91 |
| instant-meshes | 1006 | **55** | 100 | 7 | 64 | 43 | 31 | 0 | 90 | 98 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 951 | **87** | 100 | 83 | 76 | 91 | 79 | 81 | 99 | 73 |
| quadremesher-as-made | 954 | **85** | 100 | 86 | 73 | 85 | 78 | 58 | 100 | 38 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2950 | **92** | 100 | 84 | 84 | 97 | 93 | 92 | 93 | 79 |
| openclare-uniform | 3042 | **90** | 100 | 82 | 84 | 96 | 93 | 58 | 94 | 79 |
| integer-grid | 2642 | **85** | 100 | 81 | 76 | 95 | 92 | 73 | 52 | 55 |
| quadremesher | 2920 | **85** | 100 | 78 | 72 | 96 | 90 | 18 | 89 | 30 |
| quadremesher-uniform | 2696 | **80** | 100 | 77 | 70 | 95 | 89 | 5 | 59 | 30 |
| quadriflow | 2928 | **69** | 100 | 14 | 68 | 86 | 89 | 1 | 90 | 83 |
| instant-meshes | 3040 | **66** | 100 | 8 | 73 | 85 | 57 | 0 | 95 | 91 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 2947 | **90** | 100 | 82 | 82 | 96 | 93 | 69 | 96 | 80 |
| quadremesher-as-made | 2920 | **84** | 100 | 79 | 72 | 96 | 90 | 18 | 100 | 30 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7968 | **93** | 100 | 84 | 82 | 99 | 97 | 84 | 98 | 46 |
| **openclare** | 8014 | **91** | 100 | 83 | 81 | 98 | 98 | 64 | 99 | 47 |
| quadremesher | 6896 | **80** | 100 | 76 | 68 | 97 | 95 | 30 | 45 | 26 |
| quadremesher-uniform | 6832 | **80** | 100 | 74 | 66 | 97 | 96 | 45 | 42 | 31 |
| instant-meshes | 8072 | **70** | 100 | 10 | 78 | 94 | 70 | 0 | 96 | 92 |
| quadriflow | 8263 | **62** | 60 | 23 | 68 | 94 | 96 | 0 | 87 | 65 |
| integer-grid | 7052 | **61** | 0 | 82 | 75 | 93 | 97 | 78 | 53 | 32 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 6770 | **90** | 100 | 84 | 82 | 98 | 97 | 60 | 93 | 55 |
| quadremesher-as-made | 6896 | **84** | 100 | 76 | 68 | 97 | 95 | 30 | 100 | 26 |
