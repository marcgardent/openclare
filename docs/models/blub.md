# blub

An organic model; 106 640 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/blub__source.webp) | ![QuadriFlow (Blender)](img/blub__quadriflow.webp) | ![openclare](img/blub__openclare.webp) | ![Quad Remesher](img/blub__quadremesher.webp) |
| 106 640 triangles | 2 770 quads | 2 973 quads | 3 889 quads |

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
| openclare-uniform | 983 | **77** | 100 | 62 | 76 | 59 | 67 | 67 | 93 | 89 |
| **openclare** | 989 | **77** | 100 | 59 | 71 | 76 | 58 | 47 | 96 | 75 |
| quadremesher | 1638 | **73** | 100 | 78 | 66 | 88 | 68 | 49 | 0 | 35 |
| quadremesher-uniform | 892 | **68** | 100 | 69 | 58 | 49 | 50 | 51 | 57 | 38 |
| instant-meshes | 996 | **47** | 60 | 24 | 62 | 36 | 26 | 2 | 94 | 97 |
| quadriflow | 847 | **43** | 60 | 27 | 50 | 31 | 62 | 7 | 39 | 90 |
| integer-grid | 912 | **40** | 0 | 59 | 58 | 36 | 55 | — | 65 | 59 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1638 | **80** | 100 | 76 | 66 | 85 | 68 | 49 | 100 | 35 |
| openclare-matched | 1647 | **78** | 100 | 64 | 75 | 78 | 74 | 35 | 98 | 83 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2973 | **83** | 100 | 67 | 76 | 85 | 83 | 55 | 96 | 69 |
| openclare-uniform | 2993 | **83** | 100 | 66 | 74 | 82 | 88 | 57 | 99 | 76 |
| quadremesher | 3889 | **75** | 100 | 77 | 68 | 87 | 88 | 48 | 0 | 35 |
| quadremesher-uniform | 2703 | **74** | 100 | 69 | 64 | 69 | 86 | 24 | 60 | 39 |
| instant-meshes | 3036 | **66** | 100 | 31 | 72 | 60 | 51 | 4 | 95 | 95 |
| integer-grid | 2652 | **47** | 0 | 63 | 57 | 66 | 83 | — | 54 | 46 |
| quadriflow | 2698 | **44** | 60 | 29 | 46 | 13 | 84 | 4 | 60 | 85 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3911 | **83** | 100 | 67 | 74 | 91 | 89 | 50 | 98 | 66 |
| quadremesher-as-made | 3889 | **82** | 100 | 76 | 68 | 84 | 88 | 48 | 100 | 35 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7994 | **86** | 100 | 71 | 69 | 95 | 94 | 45 | 100 | 49 |
| openclare-uniform | 7990 | **85** | 100 | 69 | 67 | 93 | 95 | 58 | 100 | 47 |
| quadremesher | 8513 | **82** | 100 | 72 | 68 | 91 | 95 | 27 | 74 | 33 |
| integer-grid | 7057 | **78** | 100 | 68 | 56 | 91 | 94 | 58 | 53 | 28 |
| quadremesher-uniform | 6943 | **76** | 100 | 66 | 62 | 84 | 94 | 29 | 47 | 37 |
| instant-meshes | 8100 | **73** | 100 | 35 | 78 | 88 | 66 | 2 | 95 | 96 |
| quadriflow | 7229 | **38** | 0 | 32 | 48 | 56 | 91 | 13 | 61 | 68 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 8369 | **85** | 100 | 71 | 69 | 95 | 94 | 67 | 93 | 48 |
| quadremesher-as-made | 8513 | **82** | 100 | 72 | 68 | 91 | 95 | 27 | 100 | 33 |
