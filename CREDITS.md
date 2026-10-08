# openclare — credits for the models

Every 3D model the project uses for its measurements and its pictures, with its author, its
source and its licence. No model enters the bench without its line here.

- **In the repository**: the file is tracked under `bench/corpus/`, and the comparison meshes
  made from it under `bench/reference/`.
- **Out of the repository**: the licence does not allow passing the model on. It is kept under
  `bench/corpus-large/`, which is not tracked, with everything made from it. The figures
  measured on it are in the results. The "Source" column says how to get it again.

## Models by the author of the project

| Model | Author | Year | Source | Licence | Where |
|---|---|---|---|---|---|
| `ogre` | Marc Gardent | 2026 | original sculpt, 4.7 million triangles, brought down to 200,000 by `retopo preprocess`; the mascot of the project | © 2026 Marc Gardent, [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/deed.en) | in the repository; the original out of it |

Convention for those to come: **Marc Gardent, 2026, CC BY-SA 4.0**.

CC BY-SA 4.0 (attribution, share alike) is to a work what the GPL is to software: it may be
reused and transformed, provided the author is credited and what is made from it stays as free.
Creative Commons declared it compatible with the GPLv3 on 8 October 2015, one way only: what is
made from a work under BY-SA 4.0 may be placed under the GPLv3, not the reverse. Decided on
8 October 2026.

## Models by others

| Model | Title and author | Source | Licence | Where | Processing |
|---|---|---|---|---|---|
| `spot` | *Spot*, Keenan Crane | Keenan Crane's [3D Model Repository](https://www.cs.cmu.edu/~kmcrane/Projects/ModelRepository/) (Carnegie Mellon), downloaded on 6 October 2026 | the repository says: "you can do whatever you like with the data without asking for permission" | in the repository | subdivided twice, then 100,000 triangles |
| `bob` | *Bob*, Keenan Crane | same | same | in the repository | subdivided twice, then 100,000 triangles |
| `blub` | *Blub*, Keenan Crane | same | same | in the repository | subdivided twice, then 100,000 triangles |
| `pig` | *Origins of the Pig*, Keenan Crane | same | same | in the repository | body only, subdivided twice, then 100,000 triangles |
| `jerry` | *Jerry the Ogre*, Keenan Crane | same | same | in the repository | face at rest, as it is |
| `rose` | *Desert Rose*, Keenan Crane | same | same | in the repository | 150,000 triangles |
| `bridge` | *Pittsburgh Bridge*, scan by the Panoptic Studio (Carnegie Mellon) | same | released into the public domain by its authors | in the repository | as it is |
| `suzanne` | *Suzanne*, Blender's monkey head, Blender Foundation | Blender's "Monkey" primitive, exported by `bench/make_corpus.py` | ships with Blender, software under the GNU GPL | in the repository | subdivided |
| `terrain` | *Mountain Range Terrain*, Bipro Fx | [BlenderKit](https://www.blenderkit.com/), `asset_base_id:43a14c49-93bb-49e7-baec-6397eae071c5`, published on 7 July 2026, fetched on 8 October 2026 | BlenderKit Royalty Free: may be used in a project, may not be passed on as it is | **out of the repository** | exported as it is by `bench/import_blenderkit.py`, 543,557 triangles |
| `mahaveer` | *Mahaveer Dev*, Avi Panchal | BlenderKit, `asset_base_id:b6564fca-81c7-4873-945a-287fe143b7e6`, published on 29 December 2025, fetched on 8 October 2026 | BlenderKit Royalty Free | **out of the repository** | exported as it is, 418,472 triangles, 164 pieces |
| `marine` | *Marine Armor*, Grudgebearer iZerat; a fan creation after a video game | BlenderKit, `asset_base_id:22029966-6ee8-48d1-8d6b-d962162e09fa`, published on 9 January 2024, fetched on 8 October 2026 | BlenderKit Royalty Free | **out of the repository** | exported as it is, 247,198 triangles |

## Shapes made by the project

No third-party rights. They are made by scripts of the repository and follow its licence
(GPL-3.0-or-later).

| Model | Made by | What it is |
|---|---|---|
| `blob`, `rock`, `creature`, `cage`, `part` | `bench/make_corpus.py` (Blender) | merged balls; noisy sphere; quadruped; softened cube with holes; mechanical part with holes |
| `cube`, `sphere`, `torus`, `lblock`, `cylinder` | `retopo primitive` | yardsticks and test shapes |

## Comparison meshes

| What | Tool | Terms |
|---|---|---|
| `bench/reference/quadremesher/` | Quad Remesher 1.4.1, Exoside | trial licence, non-commercial use; run from Blender 5.2.2 on 6, 7 and 8 October 2026 |
| "quadriflow" results | QuadriFlow, built into Blender | run again at every measurement, nothing is kept |

## Pictures

The pages of `docs/models/` show renders of the models and of what the remeshers make of them.
For the models under the BlenderKit Royalty Free licence, pictures are allowed; passing on the
models themselves is not. That holds for `marine` too, a fan creation after a video game: its
picture is published since 8 October 2026, by decision of the author of the project, with the
credit above.

## Adding a model

1. Check its licence. If it does not allow redistribution, the model goes under
   `bench/corpus-large/` and its name into `LOCAL_MODELS` in `bench/run.py`.
2. Add its line to this file: title, author, exact source (link or identifier), date fetched,
   licence, processing applied.
3. For a BlenderKit model: in Blender, search for `asset_base_id:<identifier>` with the
   BlenderKit add-on, then export the file obtained:

```bash
blender -b model.blend --python bench/import_blenderkit.py -- bench/corpus-large/name.obj
```
