# mahaveer

An extra model, with open borders; 418 472 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | openclare | Quad Remesher |
|---|---|---|
| ![Source](img/mahaveer__source.webp) | ![openclare](img/mahaveer__quad-layout-adaptive.webp) | ![Quad Remesher](img/mahaveer__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **6** | 0 | 28 | 0 | 0 | 0 | — | 0 | 49 |
| **quad-layout-adaptive** | **5** | 0 | 26 | 0 | 0 | 0 | — | 0 | 38 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| integer-grid | **65** | 100 | 69 | 64 | 39 | 69 | 82 | 0 | 22 |
| quadremesher | **29** | 0 | 40 | 38 | 73 | 33 | — | 0 | 35 |
| quadremesher-uniform | **24** | 0 | 48 | 37 | 52 | 0 | — | 0 | 34 |
| quad-layout | **24** | 0 | 37 | 0 | 18 | 0 | 76 | 99 | 41 |
| **quad-layout-adaptive** | **24** | 0 | 37 | 0 | 18 | 0 | 76 | 97 | 29 |
| instant-meshes | **20** | 0 | 56 | 52 | 0 | 0 | 28 | 0 | 52 |
| quadriflow | **0** | 0 | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **39** | 0 | 48 | 33 | 69 | 30 | — | 93 | 33 |
| **quad-layout-adaptive** | **37** | 0 | 48 | 31 | 69 | 16 | — | 86 | 28 |
