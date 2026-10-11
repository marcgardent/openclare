# marine

An extra model, with open borders; 247 198 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | openclare | Quad Remesher |
|---|---|---|
| ![Source](img/marine__source.webp) | ![openclare](img/marine__openclare.webp) | ![Quad Remesher](img/marine__quadremesher.webp) |
| 247 198 triangles | 7 390 quads | 214 018 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/marine__quadremesher.webp) | ![openclare, as many quads](img/marine__openclare-matched.webp) |
| 214 018 quads | 199 778 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 205185 | **39** | 0 | 55 | 49 | 93 | 48 | — | 0 | 36 |
| quadremesher-uniform | 10623 | **19** | 0 | — | 12 | 84 | 0 | — | 0 | 47 |
| instant-meshes | 204 | **19** | 0 | 14 | 0 | 39 | 100 | 0 | 0 | 70 |
| **openclare** | 7390 | **17** | 0 | — | 0 | 85 | 0 | — | 0 | 0 |
| openclare-uniform | 7648 | **17** | 0 | — | 0 | 85 | 0 | — | 0 | 9 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 191140 | **46** | 0 | 63 | 73 | 79 | 41 | — | 73 | 0 |
| quadremesher-as-made | 205185 | **33** | 0 | 42 | 49 | 48 | 48 | — | 100 | 36 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 214018 | **39** | 0 | 55 | 51 | 91 | 52 | — | 0 | 39 |
| quadremesher-uniform | 11340 | **24** | 0 | 39 | 15 | 82 | 0 | — | 0 | 42 |
| openclare-uniform | 7648 | **22** | 0 | 44 | 0 | 80 | 0 | — | 0 | 9 |
| **openclare** | 7390 | **22** | 0 | 44 | 0 | 80 | 0 | — | 0 | 0 |
| instant-meshes | 1225 | **11** | 0 | 21 | 0 | 43 | 0 | 7 | 0 | 66 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 199778 | **47** | 0 | 63 | 74 | 79 | 44 | — | 73 | 0 |
| quadremesher-as-made | 214018 | **34** | 0 | 46 | 51 | 48 | 52 | — | 100 | 39 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 246063 | **40** | 0 | 54 | 57 | 88 | 58 | — | 0 | 37 |
| quadremesher-uniform | 14915 | **25** | 0 | 40 | 24 | 78 | 0 | — | 0 | 37 |
| openclare-uniform | 10117 | **22** | 0 | 42 | 0 | 83 | 0 | — | 0 | 6 |
| **openclare** | 10134 | **21** | 0 | 38 | 0 | 82 | 0 | — | 0 | 0 |
| instant-meshes | 4146 | **11** | 0 | 20 | 0 | 44 | 0 | 11 | 0 | 61 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 229651 | **48** | 0 | 64 | 75 | 78 | 52 | — | 73 | 0 |
| quadremesher-as-made | 246063 | **37** | 0 | 49 | 57 | 48 | 58 | — | 100 | 37 |
