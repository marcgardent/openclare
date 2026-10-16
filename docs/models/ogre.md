# ogre

An organic model; 199 764 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/ogre__source.webp) | ![QuadriFlow (Blender)](img/ogre__quadriflow.webp) | ![AutoRemesher](img/ogre__autoremesher.webp) | ![openclare](img/ogre__openclare.webp) | ![Quad Remesher](img/ogre__quadremesher.webp) |
| 199 764 triangles | 3 049 quads | 1 081 quads | 3 021 quads | 6 134 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/ogre__quadremesher.webp) | ![openclare, as many quads](img/ogre__openclare-matched.webp) |
| 6 134 quads | 6 178 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | 2 % | 10 % | 89 % | 0 % | 14 | 250 |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 11 % | 15 % | 73 % | 0 % | 25 | 131 |
| Quad Remesher | 8 % | 15 % | 77 % | 0 % | 20 | 199 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 994 | **84** | 100 | 83 | 100 | 51 | 100 | 49 | 64 | 98 | 73 |
| **openclare** | 980 | **80** | 100 | 78 | 94 | 64 | 54 | 45 | 76 | 92 | 67 |
| quadremesher | 2890 | **73** | 100 | 100 | 92 | 68 | 68 | 24 | 45 | 0 | 28 |
| integer-grid | 802 | **70** | 95 | 73 | 79 | 31 | 91 | — | — | 21 | 62 |
| instant-meshes | 932 | **51** | 80 | 25 | 90 | 0 | 44 | 8 | — | 73 | 76 |
| autoremesher | 311 | **41** | 87 | 37 | 69 | 0 | 34 | — | 15 | 0 | 100 |
| quadriflow | 891 | **38** | 60 | 11 | 64 | 0 | 78 | 0 | 27 | 56 | 71 |
| quadremesher-uniform | 799 | **31** | 0 | 83 | 75 | 0 | 0 | — | — | 20 | 31 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 2890 | **83** | 100 | 91 | 92 | 63 | 68 | 24 | — | 100 | 28 |
| openclare-matched | 2864 | **76** | 100 | 74 | 100 | 62 | 99 | 23 | 32 | 96 | 59 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 3021 | **79** | 100 | 75 | 100 | 65 | 100 | 22 | 36 | 97 | 56 |
| openclare-uniform | 3019 | **78** | 100 | 70 | 100 | 45 | 100 | 27 | 52 | 97 | 65 |
| quadremesher | 6134 | **75** | 100 | 97 | 94 | 87 | 100 | 16 | 30 | 0 | 28 |
| integer-grid | 2465 | **71** | 98 | 71 | 94 | 14 | 100 | — | — | 29 | 49 |
| instant-meshes | 2874 | **57** | 80 | 27 | 100 | 4 | 80 | 3 | — | 83 | 76 |
| quadriflow | 2935 | **41** | 20 | 22 | 76 | 13 | 100 | 3 | 37 | 83 | 71 |
| quadremesher-uniform | 2417 | **39** | 0 | 81 | 80 | 11 | 59 | 17 | — | 22 | 31 |
| autoremesher | 1081 | **33** | 0 | 43 | 86 | 19 | 72 | — | — | 0 | 95 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 6134 | **90** | 100 | 92 | 94 | 84 | 100 | 16 | — | 100 | 28 |
| openclare-matched | 6178 | **85** | 100 | 81 | 100 | 74 | 100 | 30 | 68 | 97 | 45 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 8038 | **88** | 100 | 81 | 100 | 88 | 100 | 34 | 71 | 98 | 36 |
| openclare-uniform | 8085 | **87** | 100 | 77 | 100 | 76 | 100 | 36 | 78 | 96 | 38 |
| quadremesher-uniform | 6761 | **83** | 99 | 88 | 98 | 56 | 100 | — | — | 38 | 22 |
| integer-grid | 6967 | **80** | 99 | 75 | 94 | 49 | 100 | — | — | 48 | 13 |
| quadremesher | 12717 | **78** | 100 | 97 | 100 | 87 | 100 | 22 | 38 | 0 | 23 |
| instant-meshes | 7836 | **45** | 0 | 33 | 100 | 34 | 95 | 2 | — | 92 | 77 |
| autoremesher | 4622 | **43** | 0 | 66 | 100 | 22 | 100 | — | 38 | 0 | 67 |
| quadriflow | 7763 | **43** | 0 | 33 | 79 | 51 | 100 | 5 | 28 | 88 | 62 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 12717 | **92** | 100 | 94 | 100 | 86 | 100 | 22 | — | 100 | 23 |
| openclare-matched | 12825 | **89** | 100 | 85 | 100 | 92 | 100 | 32 | 70 | 97 | 26 |
