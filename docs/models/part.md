# part

A mechanical model, with hard edges; 406 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/part__source.webp) | ![QuadriFlow (Blender)](img/part__quadriflow.webp) | ![AutoRemesher](img/part__autoremesher.webp) | ![openclare](img/part__openclare.webp) | ![Quad Remesher](img/part__quadremesher.webp) |
| 406 triangles | 2 843 quads | 1 855 quads | 2 978 quads | 2 815 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/part__quadremesher.webp) | ![openclare, as many quads](img/part__openclare-matched.webp) |
| 2 815 quads | 2 780 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 51 % | 26 % | 22 % | 0 % | 7 | 90 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 84 % | 0 % | 16 % | 0 % | 0 | 48 |
| Quad Remesher | 56 % | 24 % | 20 % | 0 % | 7 | 91 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1018 | **95** | 100 | 81 | 100 | 97 | 100 | 93 | 100 | 93 | 100 |
| quadremesher-uniform | 924 | **91** | 100 | 80 | 100 | 95 | 100 | 68 | — | 70 | 76 |
| openclare-uniform | 976 | **90** | 100 | 69 | 100 | 70 | 100 | 93 | 100 | 90 | 91 |
| quadremesher | 1013 | **87** | 100 | 79 | 100 | 97 | 100 | 64 | 50 | 95 | 73 |
| integer-grid | 959 | **85** | 100 | 67 | 100 | 66 | 100 | 60 | — | 84 | 49 |
| quadriflow | 943 | **84** | 100 | 63 | 100 | 84 | 100 | 76 | 68 | 77 | 100 |
| instant-meshes | 1036 | **74** | 100 | 54 | 97 | 59 | 47 | 39 | — | 86 | 100 |
| autoremesher | 348 | **9** | 0 | 21 | 32 | 0 | 0 | — | — | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 1018 | **96** | 100 | 81 | 100 | 97 | 100 | 93 | 100 | 98 | 100 |
| quadremesher-as-made | 1013 | **93** | 100 | 79 | 100 | 97 | 100 | 64 | — | 100 | 73 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2978 | **97** | 100 | 83 | 100 | 99 | 100 | 97 | 100 | 97 | 80 |
| openclare-uniform | 3049 | **94** | 100 | 73 | 100 | 98 | 100 | 97 | 100 | 93 | 72 |
| integer-grid | 2976 | **94** | 100 | 75 | 100 | 98 | 100 | 97 | — | 97 | 66 |
| quadremesher-uniform | 2815 | **92** | 100 | 80 | 100 | 98 | 100 | 82 | — | 75 | 74 |
| quadremesher | 2815 | **87** | 100 | 80 | 100 | 98 | 100 | 82 | 50 | 75 | 75 |
| instant-meshes | 2938 | **83** | 100 | 59 | 100 | 81 | 83 | 39 | — | 92 | 100 |
| quadriflow | 2890 | **80** | 80 | 66 | 100 | 89 | 100 | 79 | 50 | 85 | 76 |
| autoremesher | 1855 | **41** | 0 | 47 | 75 | 64 | 87 | — | — | 0 | 97 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 2780 | **97** | 100 | 84 | 100 | 99 | 100 | 97 | 100 | 95 | 80 |
| quadremesher-as-made | 2815 | **94** | 100 | 80 | 100 | 98 | 100 | 82 | — | 100 | 75 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8098 | **97** | 100 | 84 | 100 | 99 | 100 | 99 | 100 | 95 | 57 |
| openclare-uniform | 8028 | **95** | 100 | 73 | 100 | 99 | 100 | 99 | 100 | 99 | 57 |
| integer-grid | 7824 | **94** | 100 | 74 | 100 | 99 | 100 | 98 | — | 91 | 43 |
| quadremesher-uniform | 7353 | **92** | 100 | 80 | 100 | 99 | 100 | 90 | — | 68 | 76 |
| quadremesher | 7353 | **87** | 100 | 80 | 100 | 99 | 100 | 90 | 55 | 68 | 76 |
| instant-meshes | 7732 | **87** | 100 | 62 | 100 | 90 | 100 | 39 | — | 87 | 80 |
| quadriflow | 7983 | **76** | 60 | 63 | 100 | 90 | 100 | 85 | 50 | 99 | 47 |
| autoremesher | 4954 | **45** | 0 | 58 | 66 | 77 | 100 | — | — | 0 | 73 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 7279 | **97** | 100 | 83 | 100 | 99 | 100 | 99 | 100 | 96 | 52 |
| quadremesher-as-made | 7353 | **95** | 100 | 80 | 100 | 99 | 100 | 90 | — | 100 | 76 |
