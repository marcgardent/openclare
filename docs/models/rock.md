# rock

An organic model; 81 920 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/rock__source.webp) | ![QuadriFlow (Blender)](img/rock__quadriflow.webp) | ![openclare](img/rock__openclare.webp) | ![Quad Remesher](img/rock__quadremesher.webp) |
| 81 920 triangles | 3 036 quads | 2 984 quads | 3 539 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/rock__quadremesher.webp) | ![openclare, as many quads](img/rock__openclare-matched.webp) |
| 3 539 quads | 3 589 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 981 | **71** | 100 | 25 | 75 | 85 | 82 | 0 | 92 | 92 |
| openclare-uniform | 978 | **71** | 100 | 24 | 71 | 82 | 82 | 22 | 91 | 94 |
| quadriflow | 947 | **65** | 100 | 0 | 69 | 81 | 100 | 1 | 79 | 99 |
| integer-grid | 898 | **64** | 100 | 28 | 53 | 78 | 80 | 0 | 59 | 59 |
| instant-meshes | 966 | **63** | 100 | 0 | 70 | 79 | 67 | 6 | 86 | 96 |
| quadremesher | 1290 | **55** | 100 | 40 | 45 | 80 | 36 | 0 | 0 | 19 |
| quadremesher-uniform | 837 | **54** | 100 | 36 | 40 | 75 | 9 | 0 | 35 | 32 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1298 | **69** | 100 | 26 | 72 | 85 | 80 | 0 | 98 | 89 |
| quadremesher-as-made | 1290 | **61** | 100 | 38 | 45 | 80 | 36 | 0 | 100 | 19 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2984 | **74** | 100 | 30 | 75 | 91 | 81 | 0 | 98 | 80 |
| openclare-uniform | 2968 | **73** | 100 | 27 | 73 | 91 | 86 | 0 | 96 | 85 |
| quadriflow | 3036 | **68** | 100 | 0 | 71 | 89 | 100 | 6 | 91 | 92 |
| integer-grid | 2687 | **67** | 100 | 30 | 56 | 91 | 84 | 0 | 58 | 53 |
| instant-meshes | 2898 | **65** | 100 | 0 | 74 | 89 | 71 | 0 | 86 | 99 |
| quadremesher | 3539 | **63** | 100 | 37 | 50 | 90 | 70 | 0 | 28 | 26 |
| quadremesher-uniform | 2531 | **62** | 100 | 35 | 47 | 88 | 63 | 0 | 37 | 31 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3589 | **71** | 100 | 31 | 73 | 91 | 81 | 0 | 94 | 68 |
| quadremesher-as-made | 3539 | **67** | 100 | 35 | 50 | 90 | 70 | 0 | 100 | 26 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7940 | **74** | 100 | 27 | 73 | 95 | 84 | 0 | 97 | 45 |
| openclare-uniform | 7943 | **73** | 100 | 26 | 70 | 94 | 86 | 0 | 97 | 55 |
| integer-grid | 7393 | **68** | 100 | 29 | 54 | 93 | 86 | 0 | 70 | 14 |
| instant-meshes | 7792 | **68** | 100 | 0 | 79 | 94 | 80 | 0 | 90 | 91 |
| quadremesher | 7071 | **66** | 100 | 23 | 51 | 92 | 92 | 0 | 54 | 29 |
| quadriflow | 7342 | **65** | 100 | 1 | 69 | 86 | 100 | 0 | 67 | 62 |
| quadremesher-uniform | 6767 | **64** | 100 | 23 | 51 | 91 | 91 | 0 | 38 | 29 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 7061 | **71** | 100 | 28 | 73 | 93 | 85 | 0 | 99 | 55 |
| quadremesher-as-made | 7071 | **67** | 100 | 26 | 51 | 92 | 92 | 0 | 100 | 29 |
