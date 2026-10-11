# blub

An organic model; 106 640 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/blub__source.webp) | ![QuadriFlow (Blender)](img/blub__quadriflow.webp) | ![openclare](img/blub__openclare.webp) | ![Quad Remesher](img/blub__quadremesher.webp) |
| 106 640 triangles | 2 770 quads | 3 030 quads | 3 889 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/blub__quadremesher.webp) | ![openclare, as many quads](img/blub__openclare-matched.webp) |
| 3 889 quads | 3 911 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1027 | **77** | 100 | 63 | 75 | 65 | 69 | 55 | 89 | 88 |
| **openclare** | 984 | **76** | 100 | 60 | 72 | 76 | 57 | 46 | 94 | 77 |
| quadremesher | 1638 | **73** | 100 | 78 | 66 | 88 | 68 | 49 | 0 | 35 |
| quadremesher-uniform | 892 | **68** | 100 | 69 | 58 | 49 | 50 | 51 | 57 | 38 |
| instant-meshes | 996 | **47** | 60 | 24 | 62 | 36 | 26 | 2 | 94 | 97 |
| quadriflow | 847 | **43** | 60 | 27 | 50 | 31 | 62 | 7 | 39 | 90 |
| integer-grid | 888 | **41** | 0 | 59 | 57 | 48 | 61 | — | 55 | 73 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1638 | **80** | 100 | 76 | 66 | 85 | 68 | 49 | 100 | 35 |
| openclare-matched | 1663 | **78** | 100 | 64 | 75 | 80 | 75 | 36 | 94 | 77 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2948 | **83** | 100 | 65 | 74 | 82 | 88 | 70 | 93 | 74 |
| **openclare** | 3030 | **83** | 100 | 65 | 75 | 84 | 83 | 58 | 96 | 66 |
| integer-grid | 2680 | **75** | 100 | 66 | 60 | 68 | 87 | 66 | 57 | 56 |
| quadremesher | 3889 | **75** | 100 | 77 | 68 | 87 | 88 | 48 | 0 | 35 |
| quadremesher-uniform | 2703 | **74** | 100 | 69 | 64 | 69 | 86 | 24 | 60 | 39 |
| instant-meshes | 3036 | **66** | 100 | 31 | 72 | 60 | 51 | 4 | 95 | 95 |
| quadriflow | 2698 | **44** | 60 | 29 | 46 | 13 | 84 | 4 | 60 | 85 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3911 | **83** | 100 | 67 | 74 | 91 | 89 | 50 | 98 | 68 |
| quadremesher-as-made | 3889 | **82** | 100 | 76 | 68 | 84 | 88 | 48 | 100 | 35 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7994 | **86** | 100 | 71 | 69 | 95 | 94 | 45 | 100 | 53 |
| openclare-uniform | 7990 | **85** | 100 | 69 | 67 | 93 | 95 | 58 | 100 | 49 |
| quadremesher | 8513 | **82** | 100 | 72 | 68 | 91 | 95 | 27 | 74 | 33 |
| integer-grid | 7057 | **78** | 100 | 68 | 56 | 91 | 94 | 58 | 53 | 27 |
| quadremesher-uniform | 6943 | **76** | 100 | 66 | 62 | 84 | 94 | 29 | 47 | 37 |
| instant-meshes | 8100 | **73** | 100 | 35 | 78 | 88 | 66 | 2 | 95 | 96 |
| quadriflow | 7229 | **38** | 0 | 32 | 48 | 56 | 91 | 13 | 61 | 68 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 8369 | **85** | 100 | 71 | 69 | 95 | 94 | 67 | 93 | 45 |
| quadremesher-as-made | 8513 | **82** | 100 | 72 | 68 | 91 | 95 | 27 | 100 | 33 |
