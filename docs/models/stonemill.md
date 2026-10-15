# stonemill

An extra model; 14 942 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/stonemill__source.webp) | ![QuadriFlow (Blender)](img/stonemill__quadriflow.webp) | ![AutoRemesher](img/stonemill__autoremesher.webp) | ![openclare](img/stonemill__openclare.webp) | ![Quad Remesher](img/stonemill__quadremesher.webp) |
| 14 942 triangles | 2 635 quads | 1 809 quads | 2 974 quads | 14 140 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/stonemill__quadremesher.webp) | ![openclare, as many quads](img/stonemill__openclare-matched.webp) |
| 14 140 quads | 14 370 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 2 % | 5 % | 92 % | 0 % | 12 | 92 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 2 % | 8 % | 90 % | 0 % | 12 | 105 |
| Quad Remesher | 2 % | 11 % | 87 % | 0 % | 37 | 189 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1013 | **65** | 100 | 41 | 55 | 42 | 72 | 5 | — | 95 | 90 |
| quadremesher | 5437 | **32** | 0 | 51 | 52 | 67 | 31 | 9 | 36 | 0 | 16 |
| **openclare** | 1003 | **31** | 0 | 35 | 33 | 56 | 9 | 2 | 25 | 99 | 66 |
| quadriflow | 938 | **23** | 0 | 33 | 37 | 12 | 0 | 1 | 40 | 75 | 100 |
| autoremesher | 703 | **11** | 0 | 24 | 40 | 0 | 0 | — | — | 0 | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3008 | **71** | 100 | 42 | 73 | 59 | 79 | 3 | — | 99 | 76 |
| **openclare** | 2974 | **41** | 0 | 42 | 64 | 64 | 45 | 3 | 33 | 97 | 65 |
| quadriflow | 2635 | **36** | 40 | 39 | 53 | 16 | 22 | 1 | 38 | 51 | 76 |
| quadremesher | 14140 | **33** | 0 | 49 | 60 | 60 | 45 | — | 23 | 0 | 11 |
| quadremesher-uniform | 2859 | **29** | 0 | 41 | 57 | 15 | 0 | — | — | 81 | 14 |
| integer-grid | 2538 | **27** | 0 | 38 | 56 | 0 | 59 | — | — | 38 | 45 |
| instant-meshes | 2664 | **27** | 20 | 36 | 52 | 0 | 7 | 3 | — | 55 | 100 |
| autoremesher | 1809 | **19** | 0 | 34 | 44 | 0 | 45 | — | — | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 14370 | **71** | 100 | 46 | 75 | 70 | 77 | 5 | — | 93 | 34 |
| quadremesher-as-made | 14140 | **31** | 0 | 43 | 60 | 29 | 45 | — | — | 100 | 11 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7867 | **75** | 100 | 50 | 78 | 66 | 84 | 20 | — | 93 | 48 |
| **openclare** | 8182 | **67** | 100 | 50 | 74 | 63 | 74 | 7 | 17 | 91 | 50 |
| quadriflow | 6712 | **42** | 40 | 41 | 53 | 58 | 49 | 3 | 30 | 36 | 53 |
| quadremesher | 33780 | **39** | 0 | 50 | 64 | 77 | 73 | 7 | 38 | 0 | 0 |
| autoremesher | 4327 | **21** | 0 | 39 | 46 | 0 | 72 | — | 2 | 0 | 73 |
