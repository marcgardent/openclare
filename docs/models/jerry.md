# jerry

An organic model, with open borders; 39 856 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/jerry__source.webp) | ![QuadriFlow (Blender)](img/jerry__quadriflow.webp) | ![openclare](img/jerry__openclare.webp) | ![Quad Remesher](img/jerry__quadremesher.webp) |
| 39 856 triangles | 3 015 quads | 3 055 quads | 6 872 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/jerry__quadremesher.webp) | ![openclare, as many quads](img/jerry__openclare-matched.webp) |
| 6 872 quads | 6 872 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1000 | **64** | 100 | 34 | 61 | 41 | 49 | 40 | 100 | 85 |
| **openclare** | 1005 | **61** | 100 | 32 | 55 | 61 | 17 | 12 | 98 | 81 |
| integer-grid | 830 | **47** | 100 | 26 | 46 | 11 | 37 | 28 | 32 | 12 |
| quadriflow | 900 | **43** | 80 | 18 | 49 | 22 | 23 | 20 | 60 | 100 |
| quadremesher | 3093 | **35** | 0 | 46 | 46 | 88 | 41 | — | 0 | 54 |
| instant-meshes | 886 | **31** | 60 | 14 | 45 | 8 | 0 | 4 | 54 | 100 |
| quadremesher-uniform | 876 | **24** | 0 | 36 | 38 | 32 | 0 | — | 50 | 42 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3071 | **62** | 100 | 37 | 65 | 57 | 44 | 13 | 97 | 72 |
| quadremesher-as-made | 3093 | **38** | 0 | 43 | 46 | 83 | 41 | — | 100 | 54 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3055 | **67** | 100 | 40 | 66 | 63 | 47 | 18 | 93 | 69 |
| openclare-uniform | 3000 | **67** | 100 | 37 | 68 | 38 | 71 | 32 | 100 | 86 |
| quadremesher-uniform | 2573 | **58** | 100 | 40 | 50 | 66 | 30 | 19 | 43 | 53 |
| integer-grid | 2617 | **53** | 100 | 34 | 52 | 7 | 64 | 25 | 49 | 45 |
| quadriflow | 3122 | **53** | 80 | 20 | 44 | 38 | 67 | 29 | 84 | 82 |
| quadremesher | 6872 | **40** | 0 | 51 | 52 | 88 | 70 | — | 0 | 53 |
| instant-meshes | 2959 | **29** | 20 | 18 | 57 | 11 | 10 | 0 | 95 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 6872 | **70** | 100 | 45 | 69 | 77 | 62 | 17 | 100 | 57 |
| quadremesher-as-made | 6872 | **44** | 0 | 50 | 52 | 86 | 70 | — | 100 | 53 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8102 | **73** | 100 | 45 | 70 | 81 | 66 | 13 | 95 | 51 |
| openclare-uniform | 7873 | **73** | 100 | 42 | 73 | 62 | 79 | 32 | 94 | 57 |
| quadremesher | 13714 | **67** | 100 | 53 | 59 | 92 | 82 | 15 | 0 | 45 |
| quadriflow | 7781 | **59** | 80 | 22 | 61 | 44 | 86 | 36 | 89 | 56 |
| quadremesher-uniform | 6947 | **42** | 0 | 44 | 58 | 75 | 64 | — | 47 | 48 |
| integer-grid | 7340 | **40** | 0 | 41 | 57 | 45 | 77 | — | 67 | 12 |
| instant-meshes | 7933 | **37** | 20 | 17 | 67 | 32 | 38 | 2 | 97 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 13714 | **74** | 100 | 52 | 59 | 91 | 82 | 15 | 100 | 45 |
| openclare-matched | 13810 | **45** | 0 | 48 | 72 | 80 | 74 | 14 | 97 | 37 |
