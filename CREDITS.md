# openclare — credits for the models

Every 3D model the project uses for its measurements and its pictures, with its author, its
source and its licence. No model enters the bench without its line here.

- **In the repository**: the model is under `bench/scenes/`, as the scene the bench runs it
  from (a `.blend`, tracked through Git LFS; see [bench/README.md](bench/README.md#scenes)),
  and the comparison meshes made from it under `bench/reference/`.
- **Out of the repository**: the licence does not allow passing the model on. It is kept under
  `bench/corpus-large/`, which is not tracked, with its scene and everything made from it. The
  figures measured on it are in the results. The "Source" column says how to get it again.

## Models by the author of the project

| Model | Author | Year | Source | Licence | Where |
|---|---|---|---|---|---|
| `ogre` | Marc Gardent | 2026 | original sculpt, 4.7 million triangles, brought down to 200,000 by `openclare preprocess`; the mascot of the project | © 2026 Marc Gardent, [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/deed.en) | in the repository; the original out of it. `assets/ogre.obj` is the same brought down to 19,480 triangles, which the tests of the engine remesh |
| `venturi` | Marc Gardent | 2026 | a car modelled in Blender after the Venturi 500 LM. The bench runs it from the scene `bench/scenes/venturi.blend`, made by `bench/make_scene.py`: the object as it is in its author's file, with its four modifiers (Mirror, Subdivision Surface, Edge Split, Boolean) and its thirteen materials; 40,764 triangles as the add-on sees it. Until 9 October 2026 it was `bench/corpus/venturi.obj`, exported by `bench/export_object.py` without the last two modifiers, a closed surface of 39,820 triangles, with its materials in `venturi.groups` | © 2026 Marc Gardent, [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/deed.en) | in the repository |
| `ogre_hp` | Marc Gardent | 2026 | the sculpt the `ogre` comes from, as it is: 2.3 million vertices, 4.7 million triangles. The scene `bench/scenes/ogre_hp.blend`, made by `bench/make_scene.py` from its author's file, holds it with the three guide curves and the sharp curve drawn on it and a vertex group painted as density | © 2026 Marc Gardent, [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/deed.en) | in the repository |

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
| `watermill`, `towermill`, `stonemill`, `windmill` | *Free Windmills Set*, LapaModels; made with a generative model | [Fab](https://fab.com/s/d87ad94cbd8e), the file `free_windmills_set.blend`, fetched on 12 October 2026 | [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/deed.en) | in the repository | the four objects of the file (`mesh`, `mesh.001`, `mesh.002`, `mesh.003`), each as it is and without its textures: 14,934, 14,998, 14,942 and 14,962 triangles |
| `terrain` | *Mountain Range Terrain*, Bipro Fx | [BlenderKit](https://www.blenderkit.com/), `asset_base_id:43a14c49-93bb-49e7-baec-6397eae071c5`, published on 7 July 2026, fetched on 8 October 2026 | BlenderKit Royalty Free: may be used in a project, may not be passed on as it is | **out of the repository** | taken as it is by `bench/make_scenes.py` (every mesh object of the file, modifiers applied, in one mesh), 543,557 triangles |
| `mahaveer` | *Mahaveer Dev*, Avi Panchal | BlenderKit, `asset_base_id:b6564fca-81c7-4873-945a-287fe143b7e6`, published on 29 December 2025, fetched on 8 October 2026 | BlenderKit Royalty Free | **out of the repository** | exported as it is, 418,472 triangles, 164 pieces |
| `marine` | *Marine Armor*, Grudgebearer iZerat; a fan creation after a video game | BlenderKit, `asset_base_id:22029966-6ee8-48d1-8d6b-d962162e09fa`, published on 9 January 2024, fetched on 8 October 2026 | BlenderKit Royalty Free | **out of the repository** | exported as it is, 247,198 triangles |
| `f16` | *F-16 fighter jet*, Joe Naaman | BlenderKit, `asset_base_id:2b0e1d40-8487-4b92-8594-6141f2ec6162`, published on 8 May 2025, fetched on 9 October 2026 | BlenderKit Royalty Free | **out of the repository** | exported as it is, 261,232 triangles, 296 pieces |
| `camera` | *Vintage 35mm Film Camera*, Hesoyam 3dart | BlenderKit, `asset_base_id:3ef12fa4-29a0-463f-b338-7ad3f81b654b`, published on 12 September 2026, fetched on 9 October 2026 | BlenderKit Royalty Free | **out of the repository** | exported as it is, 34,600 triangles, 42 pieces |
| `wolf` | *Stylized Wolf Warrior-Rigged-Animated*, Amin khedri | BlenderKit, `asset_base_id:c63283ec-7419-4bff-8f6f-13cf70bd1b14`, published on 8 February 2026, fetched on 9 October 2026 | BlenderKit Royalty Free | **out of the repository** | exported as it is, 14,797 triangles |

## Shapes made by the project

No third-party rights. They are made by scripts of the repository and follow its licence
(GPL-3.0-or-later).

| Model | Made by | What it is |
|---|---|---|
| `blob`, `rock`, `creature`, `cage`, `part` | `bench/make_corpus.py` (Blender) | merged balls; noisy sphere; quadruped; softened cube with holes; mechanical part with holes |
| `cube`, `sphere`, `torus`, `lblock`, `cylinder` | `openclare primitive` | yardsticks and test shapes |

The scene of each model that is an OBJ file, those by others included, is made from that file
by `bench/make_scenes.py` (`make scenes`), which changes nothing in it: the same vertices and
the same faces.

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
3. Give it a line in `MODELS` of `bench/run.py`, and make its scene
   ([bench/README.md](bench/README.md#scenes)): `bench/make_scene.py` for an object of a `.blend`, with its
   modifiers, its curves or its painted density; or, for a mesh that comes as an OBJ file,
   `make scenes` once the file is put under `bench/corpus/` (it need not be tracked).
4. For a BlenderKit model: add its `asset_base_id` to `BLENDERKIT` in `bench/run.py`; then
   `make blenderkit` downloads the file and makes its scene, under
   `bench/corpus-large/scenes/`:

```bash
python3 bench/fetch_blenderkit.py --models name
```
