# mahaveer

An extra model, with open borders; 418 472 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | openclare | Quad Remesher |
|---|---|---|
| ![Source](img/mahaveer__source.webp) | ![openclare](img/mahaveer__openclare.webp) | ![Quad Remesher](img/mahaveer__quadremesher.webp) |
| 418 472 triangles | 2 976 quads | 46 202 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/mahaveer__quadremesher.webp) | ![openclare, as many quads](img/mahaveer__openclare-matched.webp) |
| 46 202 quads | 44 429 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 41821 | **28** | 0 | 33 | 35 | 79 | 26 | — | 0 | 31 |
| integer-grid | 532 | **24** | 0 | 62 | 44 | 17 | 15 | — | 0 | 37 |
| instant-meshes | 562 | **22** | 0 | 49 | 25 | 0 | 77 | 25 | 0 | 58 |
| quadremesher-uniform | 2688 | **14** | 0 | 35 | 15 | 28 | 0 | — | 0 | 33 |
| openclare-uniform | 2097 | **4** | 0 | 20 | 0 | 0 | 0 | — | 0 | 49 |
| **openclare** | 2042 | **4** | 0 | 18 | 0 | 0 | 0 | — | 0 | 26 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 40296 | **42** | 0 | 58 | 64 | 52 | 71 | — | 85 | 0 |
| quadremesher-as-made | 41821 | **24** | 0 | 37 | 35 | 36 | 26 | — | 100 | 31 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 2080 | **37** | 0 | 68 | 62 | 38 | 69 | — | 0 | 28 |
| quadremesher | 46202 | **30** | 0 | 40 | 38 | 75 | 33 | — | 0 | 35 |
| quadremesher-uniform | 4324 | **25** | 0 | 48 | 37 | 54 | 0 | — | 0 | 34 |
| openclare-uniform | 3028 | **21** | 0 | 32 | 0 | 24 | 0 | — | 96 | 39 |
| **openclare** | 2976 | **21** | 0 | 32 | 0 | 24 | 0 | — | 97 | 24 |
| instant-meshes | 1699 | **20** | 0 | 56 | 52 | 0 | 0 | 28 | 0 | 52 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 44429 | **43** | 0 | 58 | 64 | 50 | 74 | — | 85 | 0 |
| quadremesher-as-made | 46202 | **27** | 0 | 40 | 38 | 37 | 33 | — | 100 | 35 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 6148 | **40** | 0 | 73 | 63 | 40 | 73 | — | 7 | 0 |
| openclare-uniform | 7883 | **38** | 0 | 44 | 35 | 66 | 29 | — | 94 | 33 |
| quadremesher-uniform | 8791 | **37** | 0 | 60 | 56 | 55 | 0 | — | 60 | 40 |
| **openclare** | 7762 | **36** | 0 | 45 | 33 | 67 | 15 | — | 88 | 22 |
| quadremesher | 55181 | **33** | 0 | 49 | 44 | 69 | 43 | — | 0 | 36 |
| instant-meshes | 5345 | **26** | 0 | 57 | 58 | 18 | 21 | 25 | 0 | 53 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 52590 | **44** | 0 | 59 | 66 | 51 | 78 | — | 81 | 0 |
| quadremesher-as-made | 55181 | **30** | 0 | 46 | 44 | 38 | 43 | — | 100 | 36 |
