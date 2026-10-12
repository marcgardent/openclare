# towermill

An extra model; 14 998 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/towermill__source.webp) | ![QuadriFlow (Blender)](img/towermill__quadriflow.webp) | ![openclare](img/towermill__openclare.webp) | ![Quad Remesher](img/towermill__quadremesher.webp) |
| 14 998 triangles | 2 732 quads | 2 285 quads | 16 264 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/towermill__quadremesher.webp) | ![openclare, as many quads](img/towermill__openclare-matched.webp) |
| 16 264 quads | 13 210 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3060 | **35** | 0 | 37 | 33 | 30 | 92 | 5 | 92 | 40 |
| quadremesher | 16264 | **35** | 0 | 53 | 56 | 64 | 50 | — | 0 | 0 |
| quadremesher-uniform | 2891 | **33** | 0 | 47 | 53 | 35 | 0 | — | 85 | 0 |
| quadriflow | 2796 | **27** | 0 | 42 | 43 | 34 | 0 | 3 | 73 | 78 |
| **openclare** | 2285 | **25** | 0 | 43 | 50 | 22 | 38 | — | 5 | 0 |
| integer-grid | 2285 | **25** | 0 | 43 | 50 | 22 | 38 | — | 5 | 41 |
| instant-meshes | 1850 | **9** | 0 | 29 | 0 | 18 | 0 | 6 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 13210 | **38** | 0 | 47 | 62 | 44 | 69 | — | 25 | 0 |
| quadremesher-as-made | 16264 | **36** | 0 | 47 | 56 | 51 | 50 | — | 100 | 0 |
