# venturi

An extra model; 40 764 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/venturi__source.webp) | ![AutoRemesher](img/venturi__autoremesher.webp) | ![openclare](img/venturi__openclare.webp) | ![Quad Remesher](img/venturi__quadremesher.webp) |
| 40 764 triangles | 4 046 quads | 3 828 quads | 14 193 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/venturi__quadremesher.webp) | ![openclare, as many quads](img/venturi__openclare-matched.webp) |
| 14 193 quads | 14 442 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 11 % | 15 % | 74 % | 0 % | 46 | 104 |
| Quad Remesher | 6 % | 3 % | 29 % | 61 % | 18 | 112 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 12157 | **59** | 0 | 100 | 88 | 84 | 100 | — | — | 0 | 53 |
| quadremesher-uniform | 8690 | **54** | 0 | 100 | 85 | 70 | 83 | — | — | 0 | 67 |
| integer-grid | 742 | **28** | 0 | 56 | 89 | 15 | 0 | — | — | 0 | 50 |
| openclare-uniform | 589 | **27** | 0 | 64 | 52 | 29 | 0 | — | 29 | 0 | 47 |
| **openclare** | 758 | **23** | 0 | 52 | 36 | 30 | 0 | — | 33 | 3 | 41 |
| autoremesher | 2390 | **23** | 0 | 96 | 0 | 18 | 0 | — | — | 0 | 100 |
| instant-meshes | 639 | **23** | 0 | 66 | 43 | 11 | 0 | 28 | — | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

Lines of the scene, as openclare reports them: 34 edges laid along the guide curves and the borders between materials, 300 faces left across a border, lines let go on 5 pieces.

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 12459 | **72** | 100 | 77 | 100 | 52 | 69 | — | 13 | 90 | 0 |
| quadremesher-as-made | 12157 | **65** | 0 | 97 | 88 | 82 | 100 | — | — | 100 | 53 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2700 | **69** | 97 | 76 | 95 | 32 | 72 | — | 30 | 60 | 33 |
| quadremesher | 14193 | **59** | 0 | 100 | 94 | 84 | 100 | — | 56 | 0 | 58 |
| quadremesher-uniform | 9910 | **57** | 0 | 100 | 92 | 79 | 88 | — | — | 0 | 65 |
| **openclare** | 3828 | **49** | 98 | 53 | 69 | 36 | 0 | — | 18 | 0 | 24 |
| integer-grid | 2412 | **46** | 0 | 76 | 99 | 25 | 80 | — | — | 22 | 24 |
| instant-meshes | 2489 | **34** | 0 | 74 | 66 | 31 | 0 | 41 | — | 32 | 99 |
| autoremesher | 4046 | **20** | 0 | 74 | 0 | 26 | 0 | — | — | 0 | 87 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 30 edges laid along the guide curves and the borders between materials, 904 faces left across a border, lines let go on 6 pieces.

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 14193 | **66** | 0 | 98 | 94 | 83 | 100 | — | — | 100 | 58 |
| openclare-matched | 14442 | **55** | 0 | 79 | 100 | 50 | 82 | — | — | 93 | 0 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 6950 | **79** | 99 | 84 | 100 | 24 | 100 | — | — | 48 | 0 |
| openclare-uniform | 7624 | **78** | 99 | 87 | 100 | 27 | 100 | — | 40 | 81 | 2 |
| **openclare** | 8945 | **63** | 100 | 74 | 96 | 29 | 34 | — | 11 | 53 | 2 |
| quadremesher | 19704 | **61** | 0 | 100 | 100 | 84 | 100 | — | 63 | 0 | 60 |
| quadremesher-uniform | 14699 | **60** | 0 | 100 | 98 | 82 | 100 | — | — | 0 | 57 |
| instant-meshes | 7693 | **50** | 0 | 78 | 86 | 57 | 23 | 45 | — | 85 | 91 |
| autoremesher | 7212 | **30** | 0 | 85 | 0 | 34 | 0 | — | — | 61 | 85 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

Lines of the scene, as openclare reports them: 98 edges laid along the guide curves and the borders between materials, 1504 faces left across a border, lines let go on 6 pieces.

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 19704 | **67** | 0 | 98 | 100 | 83 | 100 | — | — | 100 | 60 |
| openclare-matched | 19024 | **55** | 0 | 88 | 100 | 68 | 100 | — | 16 | 86 | 0 |
