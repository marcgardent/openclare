# marine

An extra model, with open borders; 247 198 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare |
|---|---|---|
| ![Source](img/marine__source.webp) | ![AutoRemesher](img/marine__autoremesher.webp) | ![openclare](img/marine__openclare.webp) |
| 247 198 triangles | 870 quads | 9 054 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| instant-meshes | 204 | **20** | 0 | 22 | 0 | 39 | 100 | 0 | — | 0 | 70 |
| **openclare** | 8094 | **20** | 0 | — | 13 | 86 | 0 | — | — | 0 | 25 |
| openclare-uniform | 8052 | **18** | 0 | — | 6 | 85 | 0 | — | — | 0 | 34 |
| autoremesher | 1110 | **15** | 0 | — | 0 | 39 | 0 | — | — | 56 | 94 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 9054 | **32** | 0 | 72 | 22 | 81 | 0 | — | — | 0 | 25 |
| openclare-uniform | 8941 | **30** | 0 | 67 | 19 | 80 | 0 | — | — | 0 | 34 |
| instant-meshes | 1225 | **14** | 0 | 33 | 7 | 43 | 0 | 7 | — | 0 | 66 |
| autoremesher | 870 | **13** | 0 | 31 | 0 | 40 | 0 | — | — | 0 | 100 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 12272 | **32** | 0 | 75 | 46 | 81 | 0 | — | 10 | 0 | 34 |
| **openclare** | 11937 | **31** | 0 | 71 | 42 | 81 | 0 | — | 10 | 0 | 21 |
| autoremesher | 2388 | **18** | 0 | 51 | 0 | 43 | 0 | — | — | 0 | 97 |
| instant-meshes | 4146 | **17** | 0 | 32 | 26 | 44 | 0 | 11 | — | 0 | 61 |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — | — |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |
