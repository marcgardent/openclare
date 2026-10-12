# watermill

An extra model; 14 934 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/watermill__source.webp) | ![QuadriFlow (Blender)](img/watermill__quadriflow.webp) | ![openclare](img/watermill__openclare.webp) | ![Quad Remesher](img/watermill__quadremesher.webp) |
| 14 934 triangles | 2 731 quads | 2 243 quads | 22 120 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 22120 | **34** | 0 | 51 | 59 | 62 | 43 | — | 0 | 0 |
| quadremesher-uniform | 2829 | **26** | 0 | 34 | 44 | 24 | 0 | — | 77 | 0 |
| quadriflow | 2553 | **21** | 0 | 36 | 43 | 20 | 0 | 3 | 40 | 65 |
| **openclare** | 2243 | **17** | 0 | 31 | 46 | 11 | 18 | — | 0 | 0 |
| integer-grid | 2269 | **17** | 0 | 29 | 47 | 10 | 19 | — | 3 | 24 |
| openclare-uniform | 2241 | **17** | 0 | 31 | 46 | 11 | 16 | — | 0 | 0 |
| instant-meshes | 1736 | **10** | 0 | 28 | 15 | 13 | 0 | 9 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 22120 | **33** | 0 | 46 | 59 | 40 | 43 | — | 100 | 0 |
| openclare-matched | 21992 | **27** | 0 | 37 | 48 | 63 | 0 | 8 | 98 | 0 |
