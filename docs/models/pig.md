# pig

An organic model; 100 062 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/pig__source.webp) | ![QuadriFlow (Blender)](img/pig__quadriflow.webp) | ![openclare](img/pig__openclare.webp) | ![Quad Remesher](img/pig__quadremesher.webp) |
| 100 062 triangles | 2 675 quads | 3 031 quads | 4 698 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/pig__quadremesher.webp) | ![openclare, as many quads](img/pig__openclare-matched.webp) |
| 4 698 quads | 4 615 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1009 | **71** | 100 | 59 | 64 | 77 | 17 | 40 | 96 | 77 |
| openclare-uniform | 1009 | **71** | 100 | 46 | 69 | 64 | 48 | 46 | 96 | 73 |
| quadremesher | 1975 | **68** | 100 | 67 | 59 | 85 | 56 | 42 | 0 | 31 |
| integer-grid | 903 | **34** | 0 | 42 | 57 | 41 | 35 | — | 61 | 59 |
| quadremesher-uniform | 853 | **24** | 0 | 54 | 53 | 0 | 0 | — | 41 | 33 |
| instant-meshes | 910 | **23** | 0 | 28 | 58 | 0 | 18 | 5 | 64 | 98 |
| quadriflow | 861 | **14** | 0 | 23 | 23 | 0 | 12 | 2 | 44 | 89 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1975 | **75** | 100 | 67 | 59 | 83 | 56 | 42 | 100 | 31 |
| openclare-matched | 1949 | **74** | 100 | 55 | 68 | 84 | 56 | 44 | 95 | 76 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3031 | **79** | 100 | 60 | 73 | 86 | 69 | 39 | 96 | 61 |
| openclare-uniform | 3049 | **74** | 100 | 52 | 70 | 67 | 77 | 21 | 93 | 71 |
| quadremesher-uniform | 2552 | **68** | 100 | 58 | 60 | 71 | 66 | 28 | 40 | 32 |
| quadremesher | 4698 | **46** | 0 | 68 | 64 | 86 | 79 | 36 | 0 | 35 |
| integer-grid | 2704 | **41** | 0 | 51 | 61 | 41 | 73 | — | 61 | 45 |
| instant-meshes | 3016 | **33** | 0 | 28 | 66 | 25 | 37 | 3 | 96 | 97 |
| quadriflow | 2675 | **21** | 0 | 23 | 28 | 5 | 57 | 2 | 57 | 84 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 4615 | **79** | 100 | 61 | 74 | 85 | 79 | 37 | 93 | 51 |
| quadremesher-as-made | 4698 | **50** | 0 | 67 | 64 | 83 | 79 | 36 | 100 | 35 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8141 | **82** | 100 | 64 | 71 | 88 | 89 | 34 | 93 | 32 |
| openclare-uniform | 8093 | **78** | 100 | 58 | 65 | 77 | 90 | 26 | 95 | 48 |
| quadremesher-uniform | 7070 | **74** | 100 | 61 | 67 | 74 | 87 | 39 | 53 | 32 |
| instant-meshes | 8116 | **63** | 80 | 34 | 75 | 67 | 55 | 4 | 94 | 92 |
| quadremesher | 9824 | **49** | 0 | 67 | 68 | 87 | 90 | — | 9 | 31 |
| integer-grid | 7189 | **48** | 0 | 58 | 56 | 76 | 89 | 37 | 59 | 13 |
| quadriflow | 6973 | **25** | 0 | 30 | 43 | 0 | 82 | 2 | 49 | 66 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 9951 | **80** | 100 | 63 | 70 | 89 | 91 | 34 | 95 | 34 |
| quadremesher-as-made | 9824 | **53** | 0 | 67 | 68 | 86 | 90 | — | 100 | 31 |
