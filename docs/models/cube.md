# cube

A mechanical model, with hard edges; 3 072 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | QuadriFlow (Blender) | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|---|
| ![Source](img/cube__source.webp) | ![QuadriFlow (Blender)](img/cube__quadriflow.webp) | ![AutoRemesher](img/cube__autoremesher.webp) | ![openclare](img/cube__openclare.webp) | ![Quad Remesher](img/cube__quadremesher.webp) |
| 3 072 triangles | 2 904 quads | 1 260 quads | 2 992 quads | 3 082 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/cube__quadremesher.webp) | ![openclare, as many quads](img/cube__openclare-matched.webp) |
| 3 082 quads | 3 082 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| QuadriFlow (Blender) | 95 % | 0 % | 5 % | 0 % | 0 | 22 |
| AutoRemesher | 83 % | 0 % | 17 % | 0 % | 0 | 58 |
| openclare | 96 % | 0 % | 4 % | 0 % | 0 | 23 |
| Quad Remesher | 96 % | 0 % | 4 % | 0 % | 0 | 23 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher | 1014 | **99** | 100 | 100 | 100 | 100 | 100 | 99 | 94 | 79 |
| quadremesher-uniform | 1014 | **99** | 100 | 100 | 100 | 100 | 100 | 99 | 94 | 73 |
| openclare-uniform | 1014 | **99** | 100 | 100 | 100 | 100 | 100 | 99 | 94 | 100 |
| **openclare** | 1014 | **99** | 100 | 99 | 98 | 100 | 100 | 99 | 94 | 100 |
| quadriflow | 1014 | **99** | 100 | 99 | 100 | 95 | 100 | 99 | 94 | 100 |
| instant-meshes | 900 | **91** | 100 | 94 | 90 | 100 | 91 | 81 | 60 | 100 |
| integer-grid | 999 | **84** | 100 | 93 | 87 | 49 | 100 | 0 | 100 | 74 |
| autoremesher | 573 | **47** | 0 | 86 | 77 | 53 | 77 | — | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 1014 | **100** | 100 | 100 | 100 | 100 | 100 | 99 | 100 | 79 |
| openclare-matched | 1014 | **99** | 100 | 99 | 98 | 100 | 100 | 99 | 100 | 100 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 2992 | **100** | 100 | 99 | 100 | 100 | 100 | 100 | 99 | 90 |
| **openclare** | 2992 | **99** | 100 | 98 | 98 | 100 | 100 | 100 | 99 | 71 |
| quadremesher | 3082 | **99** | 100 | 99 | 100 | 100 | 100 | 100 | 89 | 73 |
| quadremesher-uniform | 3082 | **99** | 100 | 99 | 100 | 100 | 100 | 100 | 89 | 73 |
| quadriflow | 2904 | **97** | 100 | 99 | 99 | 90 | 100 | 100 | 87 | 100 |
| instant-meshes | 2974 | **96** | 100 | 91 | 92 | 100 | 95 | 91 | 96 | 100 |
| integer-grid | 2992 | **94** | 100 | 97 | 96 | 82 | 100 | 63 | 99 | 60 |
| autoremesher | 1260 | **82** | 100 | 90 | 82 | 78 | 100 | 97 | 0 | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 3082 | **100** | 100 | 99 | 100 | 100 | 100 | 100 | 100 | 73 |
| openclare-matched | 3082 | **99** | 100 | 98 | 98 | 100 | 100 | 100 | 100 | 77 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 8066 | **100** | 100 | 99 | 100 | 100 | 100 | 100 | 97 | 63 |
| quadremesher | 7920 | **99** | 100 | 100 | 100 | 100 | 100 | 100 | 96 | 69 |
| quadremesher-uniform | 7920 | **99** | 100 | 100 | 100 | 100 | 100 | 100 | 96 | 74 |
| **openclare** | 8066 | **99** | 100 | 99 | 99 | 100 | 100 | 100 | 97 | 58 |
| quadriflow | 7920 | **99** | 100 | 98 | 98 | 98 | 100 | 100 | 96 | 67 |
| instant-meshes | 7802 | **97** | 100 | 96 | 95 | 100 | 99 | 95 | 90 | 100 |
| integer-grid | 8028 | **92** | 100 | 97 | 95 | 89 | 100 | 0 | 99 | 0 |
| autoremesher | 2774 | **54** | 0 | 88 | 70 | 90 | 96 | — | 0 | 93 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quadremesher-as-made | 7920 | **100** | 100 | 100 | 100 | 100 | 100 | 100 | 100 | 69 |
| openclare-matched | 7776 | **100** | 100 | 99 | 99 | 100 | 100 | 100 | 93 | 62 |
