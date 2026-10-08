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

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout | 1002 | **71** | 100 | 53 | 64 | 50 | 62 | 44 | 99 | 70 |
| **quad-layout-adaptive** | 963 | **64** | 100 | 50 | 53 | 64 | 2 | 45 | 85 | 64 |
| quadremesher | 2890 | **59** | 100 | 65 | 53 | 68 | 18 | 24 | 0 | 28 |
| integer-grid | 783 | **53** | 100 | 48 | 45 | 27 | 49 | 29 | 13 | 60 |
| instant-meshes | 932 | **39** | 80 | 16 | 52 | 0 | 0 | 8 | 73 | 76 |
| quadriflow | 891 | **29** | 60 | 7 | 29 | 0 | 28 | 0 | 56 | 71 |
| quadremesher-uniform | 799 | **19** | 0 | 53 | 38 | 0 | 0 | — | 20 | 31 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 2887 | **66** | 100 | 47 | 68 | 62 | 50 | 12 | 100 | 62 |
| quadremesher-as-made | 2890 | **64** | 100 | 58 | 53 | 63 | 18 | 24 | 100 | 28 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 3019 | **70** | 100 | 48 | 68 | 65 | 51 | 18 | 97 | 63 |
| quad-layout | 3059 | **68** | 100 | 45 | 67 | 45 | 78 | 11 | 92 | 62 |
| quadremesher | 6134 | **65** | 100 | 63 | 54 | 87 | 50 | 16 | 0 | 28 |
| instant-meshes | 2874 | **45** | 80 | 17 | 63 | 4 | 30 | 3 | 83 | 76 |
| quadriflow | 2935 | **31** | 20 | 14 | 40 | 13 | 67 | 3 | 83 | 67 |
| integer-grid | 2469 | **30** | 0 | 44 | 50 | 13 | 70 | — | 29 | 44 |
| quadremesher-uniform | 2417 | **22** | 0 | 52 | 43 | 11 | 9 | 17 | 22 | 31 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 6053 | **72** | 100 | 52 | 69 | 76 | 66 | 20 | 95 | 46 |
| quadremesher-as-made | 6134 | **71** | 100 | 59 | 54 | 84 | 50 | 16 | 100 | 28 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | 7955 | **77** | 100 | 52 | 68 | 87 | 71 | 23 | 98 | 39 |
| quad-layout | 7970 | **75** | 100 | 50 | 68 | 76 | 76 | 22 | 98 | 39 |
| quadremesher | 12717 | **68** | 100 | 62 | 60 | 87 | 74 | 22 | 0 | 23 |
| integer-grid | 7035 | **41** | 0 | 49 | 57 | 53 | 76 | — | 52 | 12 |
| quadremesher-uniform | 6761 | **39** | 0 | 57 | 58 | 56 | 52 | — | 38 | 22 |
| quadriflow | 7763 | **35** | 0 | 21 | 42 | 51 | 80 | 5 | 88 | 62 |
| instant-meshes | 7836 | **33** | 0 | 21 | 69 | 34 | 45 | 2 | 92 | 77 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|
| quad-layout-matched | 12832 | **77** | 100 | 55 | 71 | 93 | 79 | 18 | 96 | 30 |
| quadremesher-as-made | 12717 | **75** | 100 | 61 | 60 | 86 | 74 | 22 | 100 | 23 |
