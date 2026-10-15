# camera

An extra model, with hard edges, with open borders; 34 600 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/camera__source.webp) | ![AutoRemesher](img/camera__autoremesher.webp) | ![openclare](img/camera__openclare.webp) | ![Quad Remesher](img/camera__quadremesher.webp) |
| 34 600 triangles | 2 222 quads | 3 445 quads | 12 621 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/camera__quadremesher.webp) | ![openclare, as many quads](img/camera__openclare-matched.webp) |
| 12 621 quads | 12 579 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 42 % | 10 % | 36 % | 12 % | 3 | 90 |
| Quad Remesher | — | — | — | — | — | — |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 9385 | **45** | 0 | 82 | 60 | 76 | 61 | — | 0 | 56 |
| quadremesher-uniform | 3134 | **34** | 0 | 81 | 44 | 58 | 8 | — | 0 | 60 |
| integer-grid | 843 | **34** | 0 | 63 | 50 | 54 | 3 | — | 37 | 25 |
| autoremesher | 988 | **27** | 0 | 58 | 0 | 32 | 0 | — | 95 | 100 |
| **openclare** | 1670 | **25** | 0 | 73 | 0 | 48 | 0 | 71 | 0 | 74 |
| openclare-uniform | 1462 | **23** | 0 | 78 | 1 | 36 | 7 | — | 0 | 76 |
| instant-meshes | 508 | **18** | 0 | 58 | 8 | 29 | 0 | 16 | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 34 edges laid along the guide curves and the borders between materials, lines let go on 14 pieces.

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 9385 | **50** | 0 | 85 | 60 | 71 | 61 | — | 100 | 56 |
| openclare-matched | 8871 | **50** | 0 | 86 | 58 | 68 | 65 | — | 78 | 37 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 12621 | **47** | 0 | 81 | 64 | 84 | 65 | — | 0 | 61 |
| openclare-uniform | 3105 | **42** | 0 | 77 | 15 | 61 | 42 | — | 86 | 47 |
| quadremesher-uniform | 5458 | **42** | 0 | 83 | 59 | 66 | 41 | — | 0 | 64 |
| **openclare** | 3445 | **34** | 0 | 76 | 0 | 62 | 39 | — | 41 | 73 |
| integer-grid | 1109 | **28** | 0 | 62 | 37 | 32 | 37 | — | 0 | 0 |
| instant-meshes | 2216 | **27** | 0 | 62 | 40 | 45 | 0 | 24 | 0 | 97 |
| autoremesher | 2222 | **18** | 0 | 68 | 0 | 25 | 0 | — | 0 | 100 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 119 edges laid along the guide curves and the borders between materials, lines let go on 15 pieces.

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 12621 | **53** | 0 | 84 | 64 | 82 | 65 | — | 100 | 61 |
| openclare-matched | 12579 | **50** | 0 | 85 | 49 | 68 | 76 | — | 99 | 53 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 20326 | **51** | 0 | 85 | 71 | 89 | 77 | — | 0 | 52 |
| integer-grid | 7017 | **50** | 0 | 82 | 70 | 53 | 76 | — | 51 | 0 |
| openclare-uniform | 7811 | **49** | 0 | 85 | 34 | 56 | 72 | — | 91 | 43 |
| **openclare** | 8042 | **49** | 0 | 85 | 30 | 56 | 68 | — | 98 | 39 |
| quadremesher-uniform | 10881 | **49** | 0 | 86 | 72 | 78 | 67 | — | 0 | 56 |
| instant-meshes | 6839 | **37** | 0 | 71 | 53 | 42 | 22 | 35 | 42 | 75 |
| autoremesher | 4114 | **17** | 0 | 61 | 6 | 18 | 2 | — | 0 | 83 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | 100 |

Lines of the scene, as openclare reports them: 178 edges laid along the guide curves and the borders between materials, lines let go on 16 pieces.

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 20326 | **57** | 0 | 85 | 71 | 88 | 77 | — | 100 | 52 |
| openclare-matched | 19734 | **53** | 0 | 85 | 58 | 70 | 85 | — | 88 | 25 |
