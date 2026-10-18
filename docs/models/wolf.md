# wolf

An extra model, with open borders; 14 797 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/wolf__source.webp) | ![AutoRemesher](img/wolf__autoremesher.webp) | ![openclare](img/wolf__openclare.webp) | ![Quad Remesher](img/wolf__quadremesher.webp) |
| 14 797 triangles | 625 quads | 2 394 quads | 14 984 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/wolf__quadremesher.webp) | ![openclare, as many quads](img/wolf__openclare-matched.webp) |
| 14 984 quads | 13 116 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 20 % | 11 % | 69 % | 0 % | 40 | 86 |
| Quad Remesher | 16 % | 7 % | 77 % | 0 % | 70 | 170 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 738 | **64** | 100 | 84 | 75 | 9 | 75 | 40 | 65 | 0 | 56 |
| **openclare** | 738 | **64** | 100 | 84 | 75 | 9 | 75 | 40 | 65 | 0 | 59 |
| quadremesher | 7098 | **48** | 0 | 92 | 88 | 55 | 48 | — | 52 | 0 | 19 |
| integer-grid | 744 | **35** | 0 | 85 | 72 | 2 | 51 | — | — | 0 | 57 |
| quadremesher-uniform | 803 | **32** | 0 | 84 | 70 | 5 | 0 | — | — | 21 | 12 |
| instant-meshes | 866 | **26** | 0 | 58 | 60 | 0 | 0 | 6 | — | 46 | 100 |
| autoremesher | 180 | **14** | 0 | 56 | 5 | 0 | 0 | — | 21 | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 6016 | **75** | 99 | 81 | 90 | 44 | 80 | — | 39 | 39 | 0 |
| quadremesher-as-made | 7098 | **44** | 0 | 82 | 88 | 22 | 48 | — | — | 100 | 19 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2394 | **64** | 95 | 80 | 87 | 6 | 59 | — | 58 | 19 | 23 |
| quadremesher | 14984 | **55** | 0 | 92 | 92 | 61 | 98 | — | 65 | 0 | 20 |
| integer-grid | 2420 | **45** | 0 | 85 | 90 | 16 | 74 | — | — | 23 | 24 |
| openclare-uniform | 2390 | **42** | 0 | 79 | 87 | 6 | 58 | — | 57 | 19 | 23 |
| quadremesher-uniform | 2308 | **35** | 0 | 87 | 81 | 12 | 7 | — | — | 8 | 17 |
| instant-meshes | 2774 | **32** | 0 | 53 | 80 | 6 | 10 | 9 | — | 70 | 100 |
| autoremesher | 625 | **20** | 0 | 55 | 52 | 0 | 0 | — | — | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 13116 | **77** | 98 | 84 | 91 | 50 | 100 | — | 30 | 50 | 0 |
| quadremesher-as-made | 14984 | **54** | 0 | 85 | 92 | 35 | 98 | — | — | 100 | 20 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 6750 | **72** | 98 | 83 | 89 | 50 | 87 | — | 29 | 38 | 0 |
| **openclare** | 6830 | **71** | 98 | 81 | 88 | 52 | 82 | — | 27 | 42 | 0 |
| quadremesher | 28909 | **54** | 0 | 87 | 92 | 70 | 100 | — | — | 0 | 9 |
| integer-grid | 6579 | **51** | 0 | 86 | 93 | 37 | 92 | — | — | 29 | 0 |
| instant-meshes | 7656 | **40** | 0 | 48 | 94 | 19 | 55 | 4 | — | 83 | 97 |
| quadremesher-uniform | 6170 | **39** | 0 | 83 | 87 | 11 | 52 | — | — | 8 | 9 |
| autoremesher | 2119 | **39** | 0 | 64 | 83 | 14 | 58 | — | 64 | 0 | 77 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 25282 | **78** | 98 | 76 | 98 | 56 | 100 | — | 35 | 50 | 0 |
| quadremesher-as-made | 28909 | **54** | 0 | 79 | 92 | 44 | 100 | — | — | 100 | 9 |
