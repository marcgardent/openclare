# cage

An organic model; 26 656 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/cage__source.webp) | ![QuadriFlow (Blender)](img/cage__quadriflow.webp) | ![AutoRemesher](img/cage__autoremesher.webp) | ![openclare](img/cage__openclare.webp) | ![Quad Remesher](img/cage__quadremesher.webp) |
| 26 656 triangles | 2 885 quads | 4 039 quads | 2 968 quads | 5 809 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/cage__quadremesher.webp) | ![openclare, as many quads](img/cage__openclare-matched.webp) |
| 5 809 quads | 5 890 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 18 % | 38 % | 42 % | 0 % | 20 | 86 |
| AutoRemesher | 4 % | 30 % | 66 % | 0 % | 24 | 110 |
| openclare | 62 % | 18 % | 20 % | 0 % | 5 | 69 |
| Quad Remesher | 55 % | 28 % | 17 % | 0 % | 10 | 92 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 970 | **86** | 100 | 87 | 75 | 65 | 92 | 90 | — | 88 | 68 |
| **openclare** | 1001 | **84** | 100 | 86 | 75 | 79 | 84 | 79 | 52 | 100 | 83 |
| openclare-uniform | 1001 | **82** | 100 | 82 | 76 | 49 | 84 | 74 | — | 100 | 100 |
| quadremesher-uniform | 934 | **77** | 100 | 86 | 70 | 36 | 82 | 77 | — | 74 | 66 |
| quadremesher | 3224 | **76** | 100 | 88 | 78 | 86 | 91 | 77 | 39 | 0 | 60 |
| quadriflow | 1043 | **68** | 100 | 66 | 65 | 57 | 69 | 31 | 14 | 83 | 100 |
| instant-meshes | 974 | **60** | 100 | 55 | 53 | 43 | 0 | 20 | — | 90 | 100 |
| autoremesher | 1720 | **26** | 0 | 61 | 47 | 0 | 70 | — | 13 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 3168 | **90** | 100 | 90 | 84 | 79 | 95 | 86 | — | 93 | 79 |
| quadremesher-as-made | 3224 | **89** | 100 | 88 | 78 | 86 | 91 | 77 | — | 100 | 60 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3040 | **90** | 100 | 86 | 85 | 79 | 95 | 77 | — | 95 | 86 |
| integer-grid | 2874 | **88** | 100 | 85 | 80 | 77 | 97 | 85 | — | 83 | 48 |
| **openclare** | 2968 | **88** | 100 | 90 | 84 | 79 | 95 | 90 | 54 | 96 | 78 |
| quadremesher-uniform | 2785 | **87** | 100 | 89 | 79 | 81 | 96 | 64 | — | 71 | 67 |
| quadremesher | 5809 | **79** | 100 | 91 | 80 | 89 | 98 | 81 | 50 | 0 | 65 |
| instant-meshes | 2944 | **73** | 100 | 65 | 68 | 71 | 43 | 21 | — | 93 | 100 |
| quadriflow | 2891 | **73** | 100 | 64 | 71 | 77 | 88 | 33 | 15 | 85 | 94 |
| autoremesher | 4039 | **37** | 0 | 60 | 55 | 73 | 76 | — | 3 | 0 | 79 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 5890 | **94** | 100 | 91 | 87 | 94 | 97 | 92 | — | 94 | 60 |
| quadremesher-as-made | 5809 | **91** | 100 | 91 | 80 | 89 | 98 | 81 | — | 100 | 65 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8057 | **95** | 100 | 91 | 89 | 94 | 97 | 97 | 91 | 97 | 49 |
| openclare-uniform | 8112 | **94** | 100 | 89 | 89 | 94 | 98 | 80 | — | 94 | 64 |
| integer-grid | 7765 | **92** | 100 | 88 | 82 | 90 | 99 | 90 | — | 88 | 20 |
| quadremesher-uniform | 7510 | **89** | 100 | 90 | 83 | 88 | 96 | 74 | — | 76 | 61 |
| quadremesher | 11009 | **80** | 100 | 91 | 84 | 91 | 97 | 71 | 50 | 0 | 57 |
| instant-meshes | 8050 | **79** | 100 | 61 | 78 | 80 | 73 | 11 | — | 98 | 100 |
| quadriflow | 7932 | **74** | 100 | 63 | 71 | 87 | 95 | 8 | 9 | 97 | 58 |
| autoremesher | 5928 | **47** | 0 | 77 | 63 | 85 | 90 | — | 29 | 0 | 71 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 11166 | **95** | 100 | 92 | 90 | 96 | 97 | 99 | — | 94 | 45 |
| quadremesher-as-made | 11009 | **92** | 100 | 91 | 84 | 91 | 97 | 71 | — | 100 | 57 |
