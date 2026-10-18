# camera

An extra model, with hard edges, with open borders; 34 600 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare |
|---|---|---|
| ![Source](img/camera__source.webp) | ![AutoRemesher](img/camera__autoremesher.webp) | ![openclare](img/camera__openclare.webp) |
| 34 600 triangles | 2 222 quads | 3 445 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 50 % | 6 % | 33 % | 11 % | 82 | 72 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 843 | **53** | 0 | 98 | 88 | 54 | 53 | — | — | 37 | 25 |
| **openclare** | 1671 | **42** | 0 | 100 | 23 | 46 | 46 | 74 | 63 | 0 | 77 |
| openclare-uniform | 1626 | **41** | 0 | 100 | 17 | 37 | 49 | 75 | 67 | 0 | 77 |
| autoremesher | 988 | **34** | 0 | 90 | 0 | 32 | 0 | — | — | 95 | 100 |
| instant-meshes | 508 | **29** | 0 | 90 | 39 | 29 | 0 | 16 | — | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 33 edges laid along the guide curves and the borders between materials, lines let go on 15 pieces.

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 3276 | **57** | 0 | 100 | 41 | 62 | 88 | — | 83 | 63 | 44 |
| **openclare** | 3445 | **54** | 0 | 100 | 28 | 62 | 91 | — | 88 | 41 | 72 |
| integer-grid | 1109 | **46** | 0 | 96 | 73 | 32 | 87 | — | — | 0 | 0 |
| instant-meshes | 2216 | **42** | 0 | 97 | 77 | 45 | 30 | 24 | — | 0 | 97 |
| autoremesher | 2222 | **28** | 0 | 100 | 17 | 25 | 0 | — | — | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 123 edges laid along the guide curves and the borders between materials, lines let go on 16 pieces.

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 7017 | **61** | 0 | 100 | 100 | 53 | 100 | — | — | 51 | 0 |
| **openclare** | 8045 | **61** | 0 | 100 | 65 | 56 | 100 | — | — | 98 | 40 |
| openclare-uniform | 7720 | **61** | 0 | 100 | 71 | 56 | 100 | — | — | 86 | 41 |
| instant-meshes | 6839 | **53** | 0 | 100 | 91 | 42 | 72 | 35 | — | 42 | 75 |
| autoremesher | 4114 | **34** | 0 | 95 | 36 | 18 | 52 | — | — | 0 | 97 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 182 edges laid along the guide curves and the borders between materials, lines let go on 17 pieces.
