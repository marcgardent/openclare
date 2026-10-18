# venturi

An extra model; 40 764 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare |
|---|---|---|
| ![Source](img/venturi__source.webp) | ![AutoRemesher](img/venturi__autoremesher.webp) | ![openclare](img/venturi__openclare.webp) |
| 40 764 triangles | 4 046 quads | 3 828 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 11 % | 15 % | 74 % | 0 % | 46 | 104 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 742 | **28** | 0 | 56 | 89 | 15 | 0 | — | — | 0 | 50 |
| openclare-uniform | 589 | **27** | 0 | 64 | 52 | 29 | 0 | — | 29 | 0 | 47 |
| **openclare** | 758 | **23** | 0 | 52 | 36 | 30 | 0 | — | 33 | 3 | 41 |
| autoremesher | 2390 | **23** | 0 | 96 | 0 | 18 | 0 | — | — | 0 | 100 |
| instant-meshes | 639 | **23** | 0 | 66 | 43 | 11 | 0 | 28 | — | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

Lines of the scene, as openclare reports them: 34 edges laid along the guide curves and the borders between materials, 300 faces left across a border, lines let go on 5 pieces.

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2700 | **69** | 97 | 76 | 95 | 32 | 72 | — | 30 | 60 | 33 |
| **openclare** | 3828 | **49** | 98 | 53 | 69 | 36 | 0 | — | 18 | 0 | 24 |
| integer-grid | 2412 | **46** | 0 | 76 | 99 | 25 | 80 | — | — | 22 | 24 |
| instant-meshes | 2489 | **34** | 0 | 74 | 66 | 31 | 0 | 41 | — | 32 | 99 |
| autoremesher | 4046 | **20** | 0 | 74 | 0 | 26 | 0 | — | — | 0 | 87 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 30 edges laid along the guide curves and the borders between materials, 904 faces left across a border, lines let go on 6 pieces.

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 6950 | **79** | 99 | 84 | 100 | 24 | 100 | — | — | 48 | 0 |
| openclare-uniform | 7624 | **78** | 99 | 87 | 100 | 27 | 100 | — | 40 | 81 | 2 |
| **openclare** | 8945 | **63** | 100 | 74 | 96 | 29 | 34 | — | 11 | 53 | 2 |
| instant-meshes | 7693 | **50** | 0 | 78 | 86 | 57 | 23 | 45 | — | 85 | 91 |
| autoremesher | 7212 | **30** | 0 | 85 | 0 | 34 | 0 | — | — | 61 | 85 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

Lines of the scene, as openclare reports them: 98 edges laid along the guide curves and the borders between materials, 1504 faces left across a border, lines let go on 6 pieces.
