# windmill

An extra model; 14 962 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare |
|---|---|---|
| ![Source](img/windmill__source.webp) | ![AutoRemesher](img/windmill__autoremesher.webp) | ![openclare](img/windmill__openclare.webp) |
| 14 962 triangles | 1 003 quads | 432 quads |

## Edge loops

At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on each, those of borders and of faces that are not quads left out; "—" where the mesh is not a manifold.

| | closed | spiral | stops | border to border | spirals | longest that stops |
|---|---|---|---|---|---|---|
| Modelled by hand | — | — | — | — | — | — |
| QuadriFlow (Blender) | — | — | — | — | — | — |
| AutoRemesher | — | — | — | — | — | — |
| openclare | 0 % | 4 % | 96 % | 0 % | 8 | 17 |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 207 | **33** | 100 | 12 | 0 | 0 | 44 | 0 | 37 | 0 | 100 |
| openclare-uniform | 136 | **17** | 0 | 32 | 0 | 0 | 100 | 0 | 23 | 0 | 100 |
| autoremesher | 370 | **12** | 0 | 55 | 0 | 0 | 0 | — | — | 0 | 92 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 432 | **37** | 100 | 27 | 34 | 0 | 0 | 0 | 45 | 0 | 60 |
| openclare-uniform | 135 | **36** | 100 | 30 | 0 | 0 | 100 | 0 | 0 | 0 | 91 |
| instant-meshes | 1886 | **20** | 0 | 48 | 58 | 7 | 0 | 6 | — | 0 | 100 |
| autoremesher | 1003 | **17** | 0 | 56 | 28 | 0 | 5 | — | — | 0 | 93 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | — |
| integer-grid | — | **0** | 0 | — | — | — | — | — | — | — | — |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-uniform | 253 | **42** | 100 | 50 | 5 | 0 | 100 | 12 | 12 | 0 | 70 |
| **openclare** | 646 | **34** | 100 | 16 | 30 | 0 | 0 | 0 | 41 | 0 | 50 |
| autoremesher | 2364 | **29** | 0 | 63 | 65 | 0 | 51 | — | — | 0 | 71 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |
