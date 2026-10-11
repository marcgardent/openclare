# marine

An extra model, with open borders; 247 198 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | openclare | Quad Remesher |
|---|---|---|
| ![Source](img/marine__source.webp) | ![openclare](img/marine__openclare.webp) | ![Quad Remesher](img/marine__quadremesher.webp) |
| 247 198 triangles | 13 732 quads | 214 018 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/marine__quadremesher.webp) | ![openclare, as many quads](img/marine__openclare-matched.webp) |
| 214 018 quads | 226 774 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 205185 | **39** | 0 | 55 | 49 | 93 | 48 | — | 0 | 36 |
| **openclare** | 10665 | **19** | 0 | — | 12 | 85 | 0 | — | 0 | 34 |
| quadremesher-uniform | 10623 | **19** | 0 | — | 12 | 84 | 0 | — | 0 | 47 |
| instant-meshes | 204 | **19** | 0 | 14 | 0 | 39 | 100 | 0 | 0 | 70 |
| openclare-uniform | 10007 | **18** | 0 | — | 2 | 85 | 0 | — | 0 | 40 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 218256 | **47** | 0 | 63 | 75 | 79 | 46 | — | 75 | 9 |
| quadremesher-as-made | 205185 | **33** | 0 | 42 | 49 | 48 | 48 | — | 100 | 36 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 214018 | **39** | 0 | 55 | 51 | 91 | 52 | — | 0 | 39 |
| **openclare** | 13732 | **28** | 0 | 50 | 29 | 80 | 0 | — | 0 | 31 |
| openclare-uniform | 11505 | **25** | 0 | 49 | 13 | 81 | 0 | — | 0 | 39 |
| quadremesher-uniform | 11340 | **24** | 0 | 39 | 15 | 82 | 0 | — | 0 | 42 |
| instant-meshes | 1225 | **11** | 0 | 21 | 0 | 43 | 0 | 7 | 0 | 66 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 226774 | **48** | 0 | 63 | 75 | 80 | 49 | — | 76 | 7 |
| quadremesher-as-made | 214018 | **34** | 0 | 46 | 51 | 48 | 52 | — | 100 | 39 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 246063 | **40** | 0 | 54 | 57 | 88 | 58 | — | 0 | 37 |
| **openclare** | 19789 | **29** | 0 | 50 | 33 | 82 | 0 | — | 0 | 33 |
| openclare-uniform | 15453 | **28** | 0 | 51 | 28 | 84 | 0 | — | 0 | 41 |
| quadremesher-uniform | 14915 | **25** | 0 | 40 | 24 | 78 | 0 | — | 0 | 37 |
| instant-meshes | 4146 | **11** | 0 | 20 | 0 | 44 | 0 | 11 | 0 | 61 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 256346 | **49** | 0 | 64 | 77 | 78 | 55 | — | 83 | 5 |
| quadremesher-as-made | 246063 | **37** | 0 | 49 | 57 | 48 | 58 | — | 100 | 37 |
