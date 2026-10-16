# windmill

An extra model; 14 962 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/windmill__source.webp) | ![AutoRemesher](img/windmill__autoremesher.webp) | ![openclare](img/windmill__openclare.webp) | ![Quad Remesher](img/windmill__quadremesher.webp) |
| 14 962 triangles | 1 003 quads | 349 quads | 17 287 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/windmill__quadremesher.webp) | ![openclare, as many quads](img/windmill__openclare-matched.webp) |
| 17 287 quads | 753 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 0 % | 11 % | 89 % | 0 % | 8 | 18 |
| Quad Remesher | 8 % | 11 % | 82 % | 0 % | 46 | 180 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 6933 | **51** | 0 | 88 | 91 | 69 | 64 | — | — | 0 | 0 |
| openclare-uniform | 141 | **14** | 0 | 31 | 0 | 0 | 100 | 0 | 0 | 0 | 100 |
| autoremesher | 370 | **12** | 0 | 55 | 0 | 0 | 0 | — | — | 0 | 92 |
| **openclare** | 164 | **9** | 0 | 8 | 0 | 0 | 77 | 0 | 6 | 0 | 97 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 17287 | **54** | 0 | 90 | 98 | 75 | 98 | — | 39 | 0 | 0 |
| quadremesher-uniform | 2787 | **45** | 0 | 75 | 85 | 46 | 13 | — | — | 72 | 0 |
| instant-meshes | 1886 | **20** | 0 | 48 | 58 | 7 | 0 | 6 | — | 0 | 100 |
| openclare-uniform | 207 | **19** | 0 | 23 | 0 | 0 | 100 | 4 | 44 | 0 | 100 |
| autoremesher | 1003 | **17** | 0 | 56 | 28 | 0 | 5 | — | — | 0 | 93 |
| **openclare** | 349 | **8** | 0 | 26 | 0 | 0 | 0 | 0 | 25 | 0 | 88 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — | — |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 17287 | **58** | 0 | 74 | 98 | 65 | 98 | — | — | 100 | 0 |
| openclare-matched | 753 | **18** | 0 | 38 | 34 | 0 | 0 | 1 | 44 | 0 | 51 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 36766 | **58** | 0 | 89 | 100 | 79 | 100 | — | — | 0 | 0 |
| **openclare** | 439 | **33** | 100 | 25 | 19 | 0 | 0 | 0 | 32 | 0 | 58 |
| autoremesher | 2364 | **29** | 0 | 63 | 65 | 0 | 51 | — | — | 0 | 71 |
| openclare-uniform | 249 | **24** | 0 | 43 | 0 | 0 | 100 | 10 | 60 | 0 | 76 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |
