# pig

An organic model; 100 062 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/pig__source.webp) | ![QuadriFlow (Blender)](img/pig__quadriflow.webp) | ![AutoRemesher](img/pig__autoremesher.webp) | ![openclare](img/pig__openclare.webp) | ![Quad Remesher](img/pig__quadremesher.webp) |
| 100 062 triangles | 2 675 quads | 1 808 quads | 3 054 quads | 4 698 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/pig__quadremesher.webp) | ![openclare, as many quads](img/pig__openclare-matched.webp) |
| 4 698 quads | 4 684 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 10 % | 90 % | 0 % | 12 | 218 |
| AutoRemesher | 11 % | 6 % | 84 % | 0 % | 5 | 113 |
| openclare | 25 % | 9 % | 66 % | 0 % | 9 | 145 |
| Quad Remesher | 18 % | 24 % | 58 % | 0 % | 20 | 191 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1008 | **87** | 100 | 92 | 100 | 82 | 69 | 42 | 72 | 97 | 79 |
| quadremesher | 1975 | **84** | 100 | 100 | 98 | 85 | 100 | 42 | 82 | 0 | 31 |
| openclare-uniform | 990 | **81** | 100 | 69 | 100 | 66 | 97 | 44 | 53 | 96 | 82 |
| quadremesher-uniform | 853 | **66** | 91 | 84 | 92 | 0 | 50 | — | — | 41 | 33 |
| integer-grid | 877 | **51** | 0 | 65 | 96 | 44 | 99 | 48 | — | 51 | 62 |
| instant-meshes | 910 | **37** | 0 | 44 | 97 | 0 | 68 | 5 | — | 64 | 98 |
| autoremesher | 548 | **32** | 0 | 55 | 76 | 5 | 24 | — | 65 | 0 | 100 |
| quadriflow | 861 | **25** | 0 | 36 | 56 | 0 | 62 | 2 | 20 | 44 | 81 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1975 | **94** | 100 | 100 | 98 | 83 | 100 | 42 | — | 100 | 31 |
| openclare-matched | 1973 | **89** | 100 | 86 | 100 | 71 | 100 | 44 | 89 | 100 | 69 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3054 | **90** | 100 | 92 | 100 | 82 | 100 | 45 | 76 | 93 | 64 |
| quadremesher-uniform | 2552 | **84** | 100 | 91 | 100 | 71 | 100 | 28 | — | 40 | 32 |
| integer-grid | 2680 | **83** | 99 | 81 | 98 | 50 | 100 | — | — | 57 | 44 |
| openclare-uniform | 2949 | **83** | 100 | 80 | 100 | 54 | 100 | 47 | 61 | 93 | 74 |
| autoremesher | 1808 | **66** | 85 | 77 | 95 | 21 | 95 | — | 65 | 0 | 90 |
| quadremesher | 4698 | **58** | 0 | 100 | 100 | 86 | 100 | 36 | 50 | 0 | 35 |
| instant-meshes | 3016 | **46** | 0 | 43 | 100 | 25 | 87 | 3 | — | 96 | 97 |
| quadriflow | 2675 | **33** | 0 | 35 | 63 | 5 | 100 | 2 | 33 | 57 | 88 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 4684 | **87** | 100 | 95 | 100 | 86 | 100 | 39 | 48 | 99 | 49 |
| quadremesher-as-made | 4698 | **66** | 0 | 100 | 100 | 83 | 100 | 36 | — | 100 | 35 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7920 | **91** | 100 | 90 | 100 | 77 | 100 | 56 | 82 | 96 | 46 |
| **openclare** | 8148 | **90** | 100 | 99 | 100 | 89 | 100 | 47 | 58 | 93 | 39 |
| quadremesher-uniform | 7070 | **87** | 100 | 94 | 100 | 74 | 100 | 39 | — | 53 | 32 |
| instant-meshes | 8116 | **75** | 80 | 52 | 100 | 67 | 100 | 4 | — | 94 | 92 |
| quadremesher | 9824 | **62** | 0 | 100 | 100 | 87 | 100 | — | 63 | 9 | 31 |
| integer-grid | 7187 | **62** | 0 | 91 | 95 | 76 | 100 | 37 | — | 59 | 14 |
| autoremesher | 4682 | **49** | 0 | 78 | 100 | 68 | 100 | — | 20 | 0 | 63 |
| quadriflow | 6973 | **32** | 0 | 46 | 80 | 0 | 100 | 2 | 6 | 49 | 72 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 9969 | **88** | 100 | 97 | 100 | 89 | 100 | 45 | 50 | 94 | 36 |
| quadremesher-as-made | 9824 | **68** | 0 | 100 | 100 | 86 | 100 | — | — | 100 | 31 |
