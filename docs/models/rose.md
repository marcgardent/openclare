# rose

An organic model; 140 336 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/rose__source.webp) | ![QuadriFlow (Blender)](img/rose__quadriflow.webp) | ![openclare](img/rose__quad-layout-adaptive.webp) | ![Quad Remesher](img/rose__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 26339 | **32** | 0 | 58 | 55 | 44 | 36 | — | 0 | 36 |
| quadremesher-uniform | 1271 | **13** | 0 | 36 | 16 | 21 | 0 | — | 0 | 46 |
| integer-grid | 596 | **9** | 0 | 25 | 20 | 7 | 0 | — | 0 | 53 |
| quad-layout | 1773 | **8** | 0 | 23 | 0 | 20 | 0 | — | 0 | 46 |
| **quad-layout-adaptive** | 1343 | **6** | 0 | 23 | 0 | 9 | 0 | — | 0 | 41 |
| quadriflow | 494 | **5** | 0 | 18 | 6 | 0 | 0 | 8 | 0 | 82 |
| instant-meshes | 530 | **4** | 0 | 17 | 0 | 0 | 0 | 6 | 0 | 88 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 26548 | **41** | 0 | 56 | 62 | 77 | 41 | 19 | 97 | 17 |
| quadremesher-as-made | 26339 | **24** | 0 | 44 | 55 | 0 | 36 | — | 100 | 36 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 35874 | **33** | 0 | 60 | 55 | 38 | 51 | — | 0 | 45 |
| quadremesher-uniform | 2689 | **21** | 0 | 37 | 32 | 15 | 0 | — | 59 | 42 |
| quad-layout | 3242 | **21** | 0 | 32 | 3 | 39 | 0 | — | 68 | 47 |
| **quad-layout-adaptive** | 3568 | **18** | 0 | 34 | 0 | 55 | 0 | 9 | 24 | 38 |
| integer-grid | 2060 | **14** | 0 | 29 | 39 | 12 | 0 | — | 0 | 26 |
| quadriflow | 2085 | **8** | 0 | 18 | 18 | 11 | 0 | 10 | 0 | 75 |
| instant-meshes | 2226 | **8** | 0 | 15 | 22 | 4 | 0 | 12 | 0 | 82 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 36124 | **43** | 0 | 59 | 66 | 73 | 53 | 20 | 97 | 7 |
| quadremesher-as-made | 35874 | **27** | 0 | 50 | 55 | 0 | 51 | — | 100 | 45 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout | 8743 | **62** | 100 | 46 | 46 | 72 | 24 | 19 | 63 | 43 |
| **quad-layout-adaptive** | 8417 | **60** | 100 | 48 | 37 | 74 | 0 | 13 | 79 | 31 |
| quadremesher | 53874 | **33** | 0 | 60 | 57 | 28 | 65 | — | 0 | 45 |
| integer-grid | 6566 | **22** | 0 | 37 | 49 | 11 | 15 | — | 28 | 0 |
| quadremesher-uniform | 6896 | **22** | 0 | 45 | 42 | 7 | 0 | — | 45 | 50 |
| instant-meshes | 6932 | **15** | 0 | 18 | 40 | 2 | 0 | 7 | 47 | 86 |
| quadriflow | 6758 | **13** | 0 | 19 | 26 | 6 | 0 | 3 | 38 | 71 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 53646 | **46** | 0 | 59 | 69 | 77 | 65 | 20 | 98 | 8 |
| quadremesher-as-made | 53874 | **29** | 0 | 50 | 57 | 0 | 65 | — | 100 | 45 |
