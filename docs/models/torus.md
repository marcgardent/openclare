# torus

A calibration model; 9 216 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/torus__source.webp) | ![QuadriFlow (Blender)](img/torus__quadriflow.webp) | ![AutoRemesher](img/torus__autoremesher.webp) | ![openclare](img/torus__openclare.webp) | ![Quad Remesher](img/torus__quadremesher.webp) |
| 9 216 triangles | 2 855 quads | 2 672 quads | 2 952 quads | 2 824 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/torus__quadremesher.webp) | ![openclare, as many quads](img/torus__openclare-matched.webp) |
| 2 824 quads | 2 800 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 0 % | 44 % | 56 % | 0 % | 7 | 411 |
| AutoRemesher | 0 % | 100 % | 0 % | 0 % | 6 | 0 |
| openclare | 100 % | 0 % | 0 % | 0 % | 0 | 0 |
| Quad Remesher | 10 % | 81 % | 9 % | 0 % | 10 | 108 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 1008 | **99** | 100 | 100 | 100 | 97 | 100 | 100 | 100 | 97 | 89 |
| openclare-uniform | 1008 | **99** | 100 | 100 | 100 | 97 | 100 | 100 | 100 | 97 | 96 |
| quadremesher | 920 | **97** | 100 | 100 | 100 | 95 | 100 | 100 | 100 | 68 | 64 |
| integer-grid | 920 | **96** | 100 | 100 | 100 | 97 | 100 | 100 | — | 68 | 100 |
| quadremesher-uniform | 920 | **96** | 100 | 100 | 100 | 95 | 100 | 100 | — | 68 | 77 |
| autoremesher | 728 | **90** | 100 | 100 | 100 | 89 | 100 | 100 | 100 | 0 | 100 |
| instant-meshes | 1054 | **87** | 100 | 95 | 97 | 92 | 65 | 12 | — | 78 | 100 |
| quadriflow | 1028 | **74** | 100 | 61 | 100 | 94 | 100 | 1 | 0 | 89 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 920 | **100** | 100 | 100 | 100 | 97 | 100 | 100 | 100 | 100 | 100 |
| quadremesher-as-made | 920 | **99** | 100 | 100 | 100 | 95 | 100 | 100 | — | 100 | 64 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 2952 | **99** | 100 | 100 | 100 | 99 | 100 | 100 | 100 | 94 | 89 |
| openclare-uniform | 2952 | **99** | 100 | 100 | 100 | 99 | 100 | 100 | 100 | 94 | 78 |
| integer-grid | 2720 | **96** | 100 | 100 | 100 | 99 | 100 | 100 | — | 63 | 83 |
| quadremesher-uniform | 2824 | **93** | 100 | 100 | 100 | 97 | 100 | 20 | — | 77 | 74 |
| quadremesher | 2824 | **88** | 100 | 100 | 100 | 97 | 100 | 20 | 50 | 77 | 62 |
| instant-meshes | 3078 | **85** | 100 | 57 | 100 | 97 | 100 | 0 | — | 90 | 100 |
| autoremesher | 2672 | **80** | 100 | 84 | 100 | 97 | 100 | 100 | 0 | 56 | 100 |
| quadriflow | 2976 | **60** | 20 | 85 | 87 | 96 | 100 | 0 | 0 | 97 | 89 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 2800 | **100** | 100 | 100 | 100 | 99 | 100 | 100 | 100 | 97 | 67 |
| quadremesher-as-made | 2824 | **95** | 100 | 100 | 100 | 97 | 100 | 20 | — | 100 | 62 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8024 | **100** | 100 | 100 | 100 | 99 | 100 | 100 | 100 | 99 | 60 |
| openclare-uniform | 8024 | **100** | 100 | 100 | 100 | 99 | 100 | 100 | 100 | 99 | 62 |
| autoremesher | 7544 | **98** | 100 | 100 | 100 | 98 | 100 | 100 | 100 | 77 | 81 |
| integer-grid | 7280 | **96** | 100 | 100 | 100 | 99 | 100 | 100 | — | 64 | 54 |
| instant-meshes | 8038 | **95** | 100 | 100 | 100 | 99 | 100 | 1 | — | 98 | 100 |
| quadremesher-uniform | 7401 | **93** | 100 | 100 | 100 | 99 | 100 | 19 | — | 70 | 57 |
| quadremesher | 7401 | **87** | 100 | 100 | 100 | 99 | 100 | 19 | 50 | 70 | 55 |
| quadriflow | 9147 | **60** | 40 | 76 | 100 | 99 | 100 | 1 | 0 | 43 | 59 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 7467 | **100** | 100 | 100 | 100 | 99 | 100 | 100 | 100 | 96 | 62 |
| quadremesher-as-made | 7401 | **95** | 100 | 100 | 100 | 99 | 100 | 19 | — | 100 | 55 |
