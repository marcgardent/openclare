# marine

An extra model, with open borders; 247 198 triangles in the source.
Its licence does not allow passing it on: the model is not in the repository (`make blenderkit`).
Author, source and licence: [CREDITS.md](../../CREDITS.md).

No picture of this one is published.

## Marks

From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).

### 1000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| **quad-layout-adaptive** | **17** | 0 | — | 0 | 87 | 0 | — | 0 | 1 |
| quad-layout | **17** | 0 | — | 0 | 87 | 0 | — | 0 | 9 |

### 3000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quadremesher | **40** | 0 | 55 | 51 | 96 | 52 | — | 0 | 39 |
| quadremesher-uniform | **25** | 0 | 39 | 15 | 94 | 0 | — | 0 | 42 |
| quad-layout | **23** | 0 | 46 | 0 | 87 | 0 | — | 0 | 11 |
| **quad-layout-adaptive** | **23** | 0 | 44 | 0 | 87 | 0 | — | 0 | 2 |
| instant-meshes | **17** | 0 | 21 | 0 | 85 | 0 | 7 | 0 | 66 |
| integer-grid | **0** | 0 | — | — | — | — | — | — | — |
| quadriflow | **0** | 0 | — | — | — | — | — | — | 100 |

### 8000 quads

| Remesher | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Budget | Speed |
|---|---|---|---|---|---|---|---|---|---|
| quad-layout | **29** | 0 | 44 | 0 | 93 | 0 | — | 51 | 10 |
| **quad-layout-adaptive** | **27** | 0 | 41 | 0 | 93 | 0 | — | 40 | 1 |
