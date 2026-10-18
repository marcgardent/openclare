# ogre_hp

An extra model; 4 686 394 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare |
|---|---|---|
| ![Source](img/ogre_hp__source.webp) | ![AutoRemesher](img/ogre_hp__autoremesher.webp) | ![openclare](img/ogre_hp__openclare.webp) |
| 4 686 394 triangles | 1 283 quads | 3 048 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | 10 % | 17 % | 73 % | 0 % | 12 | 51 |
| openclare | 21 % | 4 % | 76 % | 0 % | 6 | 105 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 1017 | **82** | 100 | 56 | 97 | 44 | 100 | 66 | 100 | 93 | 0 |
| **openclare** | 999 | **70** | 100 | 44 | 89 | 63 | 36 | 32 | 56 | 100 | 0 |
| integer-grid | 743 | **65** | 100 | 56 | 85 | 23 | 100 | 51 | — | 0 | 0 |
| instant-meshes | 940 | **48** | 80 | 20 | 86 | 0 | 31 | 3 | — | 76 | 0 |
| autoremesher | 334 | **18** | 0 | 25 | 59 | 0 | 36 | — | 17 | 0 | 31 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 12 edges laid along the guide curves and the borders between materials, 4 along the sharp curves, lines let go on 1 pieces.

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3048 | **84** | 100 | 70 | 100 | 71 | 90 | 39 | 82 | 94 | 0 |
| openclare-uniform | 2979 | **82** | 100 | 62 | 100 | 49 | 100 | 38 | 89 | 97 | 1 |
| integer-grid | 2515 | **70** | 98 | 61 | 93 | 11 | 100 | — | — | 35 | 1 |
| instant-meshes | 2836 | **51** | 60 | 28 | 100 | 0 | 73 | 4 | — | 78 | 0 |
| autoremesher | 1283 | **31** | 0 | 39 | 86 | 19 | 82 | — | 18 | 0 | 37 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 7946 | **87** | 100 | 72 | 100 | 78 | 100 | 39 | 81 | 97 | 0 |
| **openclare** | 8037 | **85** | 100 | 82 | 100 | 91 | 100 | 21 | 42 | 98 | 0 |
| integer-grid | 6805 | **80** | 99 | 70 | 96 | 59 | 100 | — | — | 40 | 0 |
| autoremesher | 4422 | **44** | 0 | 59 | 100 | 33 | 100 | — | — | 0 | 31 |
| instant-meshes | 7700 | **44** | 0 | 38 | 100 | 20 | 92 | 3 | — | 85 | 0 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 68 edges laid along the guide curves and the borders between materials, 13 along the sharp curves.
