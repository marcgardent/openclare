# mahaveer

An extra model, with open borders; 418 472 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

## At 3000 quads asked for

| Source | AutoRemesher | openclare | Quad Remesher |
|---|---|---|---|
| ![Source](img/mahaveer__source.webp) | ![AutoRemesher](img/mahaveer__autoremesher.webp) | ![openclare](img/mahaveer__openclare.webp) | ![Quad Remesher](img/mahaveer__quadremesher.webp) |
| 418 472 triangles | 1 325 quads | 3 322 quads | 46 202 quads |

## At equal quad count

Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.

| Quad Remesher | openclare, as many quads |
|---|---|
| ![Quad Remesher](img/mahaveer__quadremesher.webp) | ![openclare, as many quads](img/mahaveer__openclare-matched.webp) |
| 46 202 quads | 45 905 quads |

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 532 | **63** | 77 | 96 | 81 | 17 | 65 | — | — | 0 | 37 |
| quadremesher | 41821 | **42** | 0 | 52 | 71 | 79 | 76 | — | — | 0 | 31 |
| instant-meshes | 562 | **35** | 0 | 76 | 58 | 0 | 100 | 25 | — | 0 | 58 |
| quadremesher-uniform | 2688 | **23** | 0 | 54 | 47 | 28 | 0 | — | — | 0 | 33 |
| autoremesher | 636 | **20** | 0 | 94 | 0 | 0 | 0 | — | — | 0 | 92 |
| openclare-uniform | 2115 | **13** | 0 | 70 | 0 | 0 | 0 | — | 2 | 0 | 39 |
| **openclare** | 1958 | **4** | 0 | 22 | 0 | 0 | 0 | — | 2 | 0 | 41 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 41444 | **60** | 0 | 100 | 100 | 50 | 100 | — | 50 | 96 | 14 |
| quadremesher-as-made | 41821 | **41** | 0 | 57 | 71 | 36 | 76 | — | — | 100 | 31 |

### 3000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| integer-grid | 2080 | **78** | 94 | 100 | 100 | 38 | 100 | — | — | 0 | 28 |
| quadremesher | 46202 | **45** | 0 | 62 | 74 | 75 | 83 | — | — | 0 | 35 |
| openclare-uniform | 2838 | **40** | 0 | 91 | 40 | 16 | 0 | 72 | 49 | 78 | 45 |
| instant-meshes | 1699 | **37** | 0 | 86 | 90 | 0 | 47 | 28 | — | 0 | 52 |
| quadremesher-uniform | 4324 | **36** | 0 | 75 | 73 | 54 | 0 | — | — | 0 | 34 |
| **openclare** | 3322 | **32** | 0 | 100 | 15 | 16 | 3 | — | — | 57 | 39 |
| autoremesher | 1325 | **20** | 0 | 91 | 0 | 9 | 0 | — | — | 0 | 93 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 45905 | **60** | 0 | 100 | 100 | 48 | 100 | — | 50 | 97 | 12 |
| quadremesher-as-made | 46202 | **44** | 0 | 62 | 74 | 37 | 83 | — | — | 100 | 35 |

### 8000 quads

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **openclare** | 7958 | **61** | 0 | 100 | 98 | 37 | 77 | — | — | 98 | 41 |
| integer-grid | 6148 | **54** | 0 | 100 | 100 | 40 | 100 | — | — | 7 | 0 |
| quadremesher-uniform | 8791 | **52** | 0 | 93 | 95 | 55 | 20 | — | — | 60 | 40 |
| openclare-uniform | 6577 | **51** | 0 | 100 | 85 | 40 | 68 | — | — | 29 | 42 |
| quadremesher | 55181 | **50** | 0 | 75 | 82 | 69 | 93 | — | — | 0 | 36 |
| instant-meshes | 5345 | **43** | 0 | 88 | 98 | 18 | 71 | 25 | — | 0 | 53 |
| autoremesher | 2996 | **27** | 0 | 91 | 30 | 14 | 10 | — | — | 0 | 73 |
| quadriflow | — | **0** | 0 | — | — | — | — | — | — | — | 100 |

At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.

| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| openclare-matched | 53768 | **59** | 0 | 100 | 100 | 46 | 100 | — | 50 | 90 | 0 |
| quadremesher-as-made | 55181 | **49** | 0 | 72 | 82 | 38 | 93 | — | — | 100 | 36 |
