# rose

An organic model; 140 336 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/rose__source.webp) | ![QuadriFlow (Blender)](img/rose__quadriflow.webp) | ![openclare](img/rose__openclare.webp) | ![Quad Remesher](img/rose__quadremesher.webp) |
| 140 336 triangles | 2 208 quads | 3 579 quads | 35 874 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/rose__quadremesher.webp) | ![openclare, as many quads](img/rose__openclare-matched.webp) |
| 35 874 quads | 35 959 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 26339 | **32** | 0 | 58 | 55 | 44 | 36 | — | 0 | 36 |
| quadremesher-uniform | 1271 | **13** | 0 | 36 | 16 | 21 | 0 | — | 0 | 46 |
| integer-grid | 511 | **9** | 0 | 30 | 14 | 2 | 0 | — | 0 | 56 |
| openclare-uniform | 1597 | **6** | 0 | 16 | 0 | 18 | 0 | — | 0 | 50 |
| **openclare** | 1377 | **5** | 0 | 20 | 0 | 7 | 0 | — | 0 | 34 |
| quadriflow | 494 | **5** | 0 | 18 | 6 | 0 | 0 | 8 | 0 | 82 |
| instant-meshes | 530 | **4** | 0 | 17 | 0 | 0 | 0 | 6 | 0 | 88 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 26395 | **42** | 0 | 57 | 61 | 80 | 40 | 18 | 99 | 16 |
| quadremesher-as-made | 26339 | **24** | 0 | 44 | 55 | 0 | 36 | — | 100 | 36 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 35874 | **33** | 0 | 60 | 55 | 38 | 51 | — | 0 | 45 |
| openclare-uniform | 3111 | **22** | 0 | 33 | 4 | 33 | 0 | — | 85 | 47 |
| quadremesher-uniform | 2689 | **21** | 0 | 37 | 32 | 15 | 0 | — | 59 | 42 |
| **openclare** | 3579 | **18** | 0 | 35 | 0 | 55 | 0 | — | 23 | 35 |
| integer-grid | 1923 | **13** | 0 | 28 | 34 | 10 | 0 | — | 0 | 32 |
| quadriflow | 2085 | **8** | 0 | 18 | 18 | 11 | 0 | 10 | 0 | 75 |
| instant-meshes | 2226 | **8** | 0 | 15 | 22 | 4 | 0 | 12 | 0 | 82 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 35959 | **44** | 0 | 59 | 66 | 74 | 53 | 22 | 99 | 12 |
| quadremesher-as-made | 35874 | **27** | 0 | 50 | 55 | 0 | 51 | — | 100 | 45 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 8711 | **60** | 100 | 46 | 47 | 64 | 24 | 17 | 64 | 42 |
| **openclare** | 8484 | **35** | 0 | 48 | 37 | 74 | 0 | 12 | 76 | 25 |
| quadremesher | 53874 | **33** | 0 | 60 | 57 | 28 | 65 | — | 0 | 45 |
| integer-grid | 6584 | **22** | 0 | 37 | 49 | 13 | 13 | — | 29 | 0 |
| quadremesher-uniform | 6896 | **22** | 0 | 45 | 42 | 7 | 0 | — | 45 | 50 |
| instant-meshes | 6932 | **15** | 0 | 18 | 40 | 2 | 0 | 7 | 47 | 86 |
| quadriflow | 6758 | **13** | 0 | 19 | 26 | 6 | 0 | 3 | 38 | 71 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 54069 | **46** | 0 | 59 | 69 | 77 | 66 | 18 | 99 | 7 |
| quadremesher-as-made | 53874 | **29** | 0 | 50 | 57 | 0 | 65 | — | 100 | 45 |
