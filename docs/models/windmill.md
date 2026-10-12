# windmill

An extra model; 14 962 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | openclare | Quad Remesher |
|---|---|---|
| ![Source](img/windmill__source.webp) | ![openclare](img/windmill__openclare.webp) | ![Quad Remesher](img/windmill__quadremesher.webp) |
| 14 962 triangles | 332 quads | 16 985 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/windmill__quadremesher.webp) | ![openclare, as many quads](img/windmill__openclare-matched.webp) |
| 16 985 quads | 599 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 16985 | **38** | 0 | 57 | 59 | 73 | 47 | — | 0 | 0 |
| quadremesher-uniform | 2759 | **32** | 0 | 49 | 47 | 45 | 0 | — | 68 | 0 |
| openclare-uniform | 207 | **11** | 0 | 15 | 0 | 0 | 81 | 4 | 0 | 100 |
| instant-meshes | 1911 | **10** | 0 | 32 | 13 | 7 | 0 | 4 | 0 | 100 |
| **openclare** | 332 | **4** | 0 | 18 | 0 | 0 | 0 | 0 | 0 | 84 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 16985 | **38** | 0 | 48 | 59 | 63 | 47 | — | 100 | 0 |
| openclare-matched | 599 | **33** | 100 | 22 | 2 | 0 | 0 | 1 | 0 | 49 |
