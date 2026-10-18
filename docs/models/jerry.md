# jerry

An organic model, with open borders; 39 856 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/jerry__source.webp) | ![QuadriFlow (Blender)](img/jerry__quadriflow.webp) | ![AutoRemesher](img/jerry__autoremesher.webp) | ![openclare](img/jerry__openclare.webp) | ![Quad Remesher](img/jerry__quadremesher.webp) |
| 39 856 triangles | 3 015 quads | 1 102 quads | 3 012 quads | 6 872 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/jerry__quadremesher.webp) | ![openclare, as many quads](img/jerry__openclare-matched.webp) |
| 6 872 quads | 6 956 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 1 % | 82 % | 17 % | 1 | 155 |
| AutoRemesher | 0 % | 13 % | 87 % | 0 % | 6 | 64 |
| openclare | 3 % | 0 % | 87 % | 10 % | 0 | 103 |
| Quad Remesher | 3 % | 6 % | 85 % | 5 % | 10 | 282 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1020 | **76** | 100 | 52 | 100 | 41 | 100 | 52 | 59 | 92 | 87 |
| quadremesher | 3093 | **72** | 99 | 71 | 84 | 88 | 91 | — | 38 | 0 | 54 |
| **openclare** | 1007 | **70** | 100 | 51 | 90 | 58 | 57 | 2 | 51 | 97 | 73 |
| integer-grid | 867 | **62** | 100 | 41 | 81 | 9 | 92 | 24 | — | 47 | 69 |
| quadremesher-uniform | 876 | **60** | 98 | 56 | 74 | 32 | 0 | — | — | 50 | 42 |
| quadriflow | 900 | **55** | 80 | 28 | 87 | 22 | 73 | 20 | 50 | 60 | 100 |
| instant-meshes | 886 | **41** | 60 | 22 | 82 | 8 | 28 | 4 | — | 54 | 100 |
| autoremesher | 305 | **22** | 48 | 14 | 38 | 0 | 42 | — | 0 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 3093 | **85** | 99 | 67 | 84 | 83 | 91 | — | — | 100 | 54 |
| openclare-matched | 3035 | **73** | 100 | 54 | 100 | 61 | 89 | 9 | 51 | 92 | 59 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3012 | **77** | 100 | 55 | 100 | 64 | 94 | 25 | 53 | 98 | 55 |
| openclare-uniform | 3045 | **77** | 100 | 56 | 100 | 48 | 100 | 43 | 58 | 94 | 72 |
| quadremesher-uniform | 2573 | **74** | 100 | 62 | 88 | 66 | 80 | 19 | — | 43 | 53 |
| integer-grid | 2595 | **66** | 100 | 53 | 91 | 2 | 100 | 40 | — | 46 | 51 |
| quadriflow | 3122 | **62** | 80 | 32 | 81 | 38 | 100 | 29 | 50 | 84 | 87 |
| quadremesher | 6872 | **54** | 0 | 79 | 90 | 88 | 100 | — | 48 | 0 | 53 |
| instant-meshes | 2959 | **42** | 20 | 29 | 97 | 11 | 60 | 0 | — | 95 | 100 |
| autoremesher | 1102 | **23** | 0 | 31 | 57 | 0 | 74 | — | 18 | 0 | 92 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 6956 | **80** | 100 | 65 | 100 | 78 | 100 | 18 | 53 | 95 | 27 |
| quadremesher-as-made | 6872 | **61** | 0 | 77 | 90 | 86 | 100 | — | — | 100 | 53 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-uniform | 6947 | **83** | 99 | 68 | 98 | 75 | 100 | — | — | 47 | 48 |
| **openclare** | 8098 | **81** | 100 | 68 | 100 | 78 | 100 | 13 | 53 | 95 | 39 |
| openclare-uniform | 7970 | **80** | 100 | 63 | 100 | 62 | 100 | 32 | 58 | 98 | 48 |
| integer-grid | 7345 | **77** | 100 | 63 | 97 | 44 | 100 | 29 | — | 67 | 12 |
| quadremesher | 13714 | **75** | 100 | 82 | 99 | 92 | 100 | 15 | 40 | 0 | 45 |
| quadriflow | 7781 | **66** | 80 | 34 | 100 | 44 | 100 | 36 | 47 | 89 | 56 |
| instant-meshes | 7933 | **49** | 20 | 27 | 100 | 32 | 88 | 2 | — | 97 | 100 |
| autoremesher | 2740 | **38** | 0 | 47 | 87 | 25 | 100 | — | — | 0 | 78 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 13714 | **89** | 100 | 80 | 99 | 91 | 100 | 15 | — | 100 | 45 |
| openclare-matched | 13566 | **81** | 100 | 70 | 100 | 81 | 100 | 19 | 54 | 96 | 23 |
