# ogre

An organic model; 199 764 triangles in the source.
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads

| Source | QuadriFlow (Blender) | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/ogre__source.webp) | ![QuadriFlow (Blender)](img/ogre__quadriflow.webp) | ![openclare](img/ogre__quad-layout-adaptive.webp) | ![Quad Remesher](img/ogre__quadremesher.webp) |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **70** | 100 | 53 | 65 | 43 | 62 | 44 | 99 | 70 |
| **quad-layout-adaptive** | **61** | 100 | 51 | 55 | 44 | 2 | 45 | 85 | 60 |
| quadremesher | **56** | 100 | 65 | 53 | 44 | 18 | 24 | 0 | 28 |
| integer-grid | **52** | 100 | 48 | 45 | 22 | 49 | 29 | 13 | 62 |
| instant-meshes | **39** | 80 | 16 | 52 | 0 | 0 | 8 | 73 | 75 |
| quadriflow | **29** | 60 | 7 | 29 | 0 | 28 | 0 | 56 | 73 |
| quadremesher-uniform | **19** | 0 | 53 | 38 | 0 | 0 | — | 20 | 31 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **69** | 100 | 46 | 68 | 45 | 78 | 11 | 92 | 65 |
| **quad-layout-adaptive** | **68** | 100 | 48 | 69 | 45 | 51 | 18 | 97 | 55 |
| quadremesher | **65** | 100 | 63 | 54 | 88 | 50 | 16 | 0 | 28 |
| instant-meshes | **45** | 80 | 17 | 63 | 8 | 30 | 3 | 83 | 76 |
| quadriflow | **33** | 20 | 14 | 40 | 27 | 67 | 3 | 83 | 67 |
| integer-grid | **32** | 0 | 44 | 50 | 25 | 70 | — | 29 | 48 |
| quadremesher-uniform | **24** | 0 | 52 | 43 | 21 | 9 | 17 | 22 | 31 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **76** | 100 | 52 | 69 | 81 | 71 | 23 | 98 | 35 |
| quad-layout | **75** | 100 | 50 | 69 | 74 | 76 | 22 | 98 | 42 |
| quadremesher | **68** | 100 | 62 | 60 | 90 | 74 | 22 | 0 | 23 |
| integer-grid | **41** | 0 | 49 | 57 | 55 | 76 | — | 52 | 14 |
| quadremesher-uniform | **38** | 0 | 57 | 58 | 49 | 52 | — | 38 | 22 |
| quadriflow | **34** | 0 | 21 | 42 | 45 | 80 | 5 | 88 | 63 |
| instant-meshes | **34** | 0 | 21 | 69 | 39 | 45 | 2 | 92 | 75 |
