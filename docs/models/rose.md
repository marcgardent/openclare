# rose

An organic model; 140 336 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/rose__source.webp) | ![QuadriFlow (Blender)](img/rose__quadriflow.webp) | ![AutoRemesher](img/rose__autoremesher.webp) | ![openclare](img/rose__openclare.webp) | ![Quad Remesher](img/rose__quadremesher.webp) |
| 140 336 triangles | 2 208 quads | 636 quads | 3 585 quads | 35 874 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/rose__quadremesher.webp) | ![openclare, as many quads](img/rose__openclare-matched.webp) |
| 35 874 quads | 36 253 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 3 % | 18 % | 80 % | 1 % | 82 | 38 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | — | — | — | — | — | — |
| Quad Remesher | — | — | — | — | — | — |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 26339 | **50** | 0 | 90 | 94 | 44 | 86 | — | — | 0 | 36 |
| quadremesher-uniform | 1271 | **23** | 0 | 56 | 48 | 21 | 0 | — | — | 0 | 46 |
| integer-grid | 511 | **17** | 0 | 46 | 46 | 2 | 0 | — | — | 0 | 56 |
| openclare-uniform | 1571 | **12** | 0 | 62 | 0 | 0 | 0 | — | 3 | 0 | 69 |
| quadriflow | 494 | **10** | 0 | 25 | 37 | 0 | 0 | 8 | 4 | 0 | 81 |
| instant-meshes | 530 | **10** | 0 | 27 | 29 | 0 | 0 | 6 | — | 0 | 88 |
| autoremesher | 157 | **6** | 0 | 29 | 0 | 0 | 0 | — | — | 0 | 100 |
| **openclare** | 1403 | **1** | 0 | — | 0 | 0 | 0 | — | 5 | 0 | 66 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 26552 | **59** | 0 | 87 | 100 | 83 | 88 | 28 | 48 | 97 | 26 |
| quadremesher-as-made | 26339 | **43** | 0 | 69 | 94 | 0 | 86 | — | — | 100 | 36 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 35874 | **51** | 0 | 93 | 94 | 38 | 100 | — | — | 0 | 45 |
| quadremesher-uniform | 2689 | **31** | 0 | 58 | 67 | 15 | 0 | — | — | 59 | 42 |
| openclare-uniform | 3364 | **27** | 0 | 53 | 33 | 44 | 0 | — | 19 | 51 | 77 |
| **openclare** | 3585 | **24** | 0 | 52 | 37 | 33 | 0 | — | — | 22 | 55 |
| integer-grid | 1923 | **24** | 0 | 44 | 69 | 10 | 19 | — | — | 0 | 32 |
| quadriflow | 2085 | **15** | 0 | 29 | 50 | 11 | 0 | 10 | 9 | 0 | 81 |
| instant-meshes | 2226 | **14** | 0 | 24 | 55 | 4 | 0 | 12 | — | 0 | 82 |
| autoremesher | 636 | **5** | 0 | 21 | 0 | 0 | 0 | — | — | 0 | 96 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 36253 | **61** | 0 | 91 | 100 | 82 | 100 | 28 | 50 | 96 | 19 |
| quadremesher-as-made | 35874 | **47** | 0 | 78 | 94 | 0 | 100 | — | — | 100 | 45 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 53874 | **50** | 0 | 93 | 96 | 28 | 100 | — | — | 0 | 45 |
| openclare-uniform | 8329 | **49** | 0 | 68 | 77 | 71 | 77 | 24 | 19 | 84 | 59 |
| **openclare** | 8368 | **43** | 0 | 69 | 68 | 73 | 27 | 24 | 20 | 82 | 37 |
| integer-grid | 6584 | **38** | 0 | 58 | 87 | 13 | 63 | — | — | 29 | 0 |
| quadremesher-uniform | 6896 | **33** | 0 | 70 | 79 | 7 | 0 | — | — | 45 | 50 |
| instant-meshes | 6932 | **22** | 0 | 28 | 76 | 2 | 0 | 7 | — | 47 | 86 |
| quadriflow | 6758 | **20** | 0 | 30 | 60 | 6 | 14 | 3 | 10 | 38 | 69 |
| autoremesher | 3828 | **13** | 0 | 29 | 42 | 0 | 5 | — | — | 0 | 85 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 53782 | **61** | 0 | 91 | 100 | 82 | 100 | 28 | 44 | 99 | 20 |
| quadremesher-as-made | 53874 | **47** | 0 | 77 | 96 | 0 | 100 | — | — | 100 | 45 |
