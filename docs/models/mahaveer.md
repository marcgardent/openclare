# mahaveer

An extra model, with open borders; 418 472 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare |
|---|---|---|
| ![Source](img/mahaveer__source.webp) | ![AutoRemesher](img/mahaveer__autoremesher.webp) | ![openclare](img/mahaveer__openclare.webp) |
| 418 472 triangles | 1 325 quads | 3 322 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 532 | **63** | 77 | 96 | 81 | 17 | 65 | — | — | 0 | 37 |
| instant-meshes | 562 | **35** | 0 | 76 | 58 | 0 | 100 | 25 | — | 0 | 58 |
| autoremesher | 636 | **20** | 0 | 94 | 0 | 0 | 0 | — | — | 0 | 92 |
| openclare-uniform | 2115 | **13** | 0 | 70 | 0 | 0 | 0 | — | 2 | 0 | 39 |
| **openclare** | 1958 | **4** | 0 | 22 | 0 | 0 | 0 | — | 2 | 0 | 41 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 2080 | **78** | 94 | 100 | 100 | 38 | 100 | — | — | 0 | 28 |
| openclare-uniform | 2838 | **40** | 0 | 91 | 40 | 16 | 0 | 72 | 49 | 78 | 45 |
| instant-meshes | 1699 | **37** | 0 | 86 | 90 | 0 | 47 | 28 | — | 0 | 52 |
| **openclare** | 3322 | **32** | 0 | 100 | 15 | 16 | 3 | — | — | 57 | 39 |
| autoremesher | 1325 | **20** | 0 | 91 | 0 | 9 | 0 | — | — | 0 | 93 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7958 | **61** | 0 | 100 | 98 | 37 | 77 | — | — | 98 | 41 |
| integer-grid | 6148 | **54** | 0 | 100 | 100 | 40 | 100 | — | — | 7 | 0 |
| openclare-uniform | 6577 | **51** | 0 | 100 | 85 | 40 | 68 | — | — | 29 | 42 |
| instant-meshes | 5345 | **43** | 0 | 88 | 98 | 18 | 71 | 25 | — | 0 | 53 |
| autoremesher | 2996 | **27** | 0 | 91 | 30 | 14 | 10 | — | — | 0 | 73 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |
