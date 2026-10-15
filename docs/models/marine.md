# marine

An extra model, with open borders; 247 198 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/marine__source.webp) | ![AutoRemesher](img/marine__autoremesher.webp) | ![openclare](img/marine__openclare.webp) | ![Quad Remesher](img/marine__quadremesher.webp) |
| 247 198 triangles | 870 quads | 8 922 quads | 214 018 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/marine__quadremesher.webp) | ![openclare, as many quads](img/marine__openclare-matched.webp) |
| 214 018 quads | 201 657 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 205185 | **39** | 0 | 55 | 49 | 93 | 48 | — | 0 | 36 |
| quadremesher-uniform | 10623 | **19** | 0 | — | 12 | 84 | 0 | — | 0 | 47 |
| instant-meshes | 204 | **19** | 0 | 14 | 0 | 39 | 100 | 0 | 0 | 70 |
| **openclare** | 7967 | **17** | 0 | — | 0 | 86 | 0 | — | 0 | 26 |
| openclare-uniform | 8170 | **17** | 0 | — | 0 | 85 | 0 | — | 0 | 36 |
| autoremesher | 1110 | **15** | 0 | — | 0 | 39 | 0 | — | 56 | 93 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 193727 | **46** | 0 | 62 | 74 | 79 | 40 | — | 78 | 1 |
| quadremesher-as-made | 205185 | **33** | 0 | 42 | 49 | 48 | 48 | — | 100 | 36 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 214018 | **39** | 0 | 55 | 51 | 91 | 52 | — | 0 | 39 |
| quadremesher-uniform | 11340 | **24** | 0 | 39 | 15 | 82 | 0 | — | 0 | 42 |
| **openclare** | 8922 | **22** | 0 | 45 | 0 | 81 | 0 | — | 0 | 32 |
| openclare-uniform | 9111 | **22** | 0 | 45 | 0 | 80 | 0 | — | 0 | 40 |
| instant-meshes | 1225 | **11** | 0 | 21 | 0 | 43 | 0 | 7 | 0 | 66 |
| autoremesher | 870 | **10** | 0 | 20 | 0 | 40 | 0 | — | 0 | 95 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 201657 | **47** | 0 | 62 | 75 | 79 | 43 | — | 77 | 9 |
| quadremesher-as-made | 214018 | **34** | 0 | 46 | 51 | 48 | 52 | — | 100 | 39 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 246063 | **40** | 0 | 54 | 57 | 88 | 58 | — | 0 | 37 |
| openclare-uniform | 12356 | **25** | 0 | 48 | 13 | 81 | 0 | — | 0 | 42 |
| quadremesher-uniform | 14915 | **25** | 0 | 40 | 24 | 78 | 0 | — | 0 | 37 |
| **openclare** | 12000 | **24** | 0 | 47 | 10 | 80 | 0 | — | 0 | 32 |
| autoremesher | 2388 | **14** | 0 | 33 | 0 | 43 | 0 | — | 0 | 92 |
| instant-meshes | 4146 | **11** | 0 | 20 | 0 | 44 | 0 | 11 | 0 | 61 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 231978 | **48** | 0 | 63 | 75 | 78 | 49 | — | 77 | 7 |
| quadremesher-as-made | 246063 | **37** | 0 | 49 | 57 | 48 | 58 | — | 100 | 37 |
