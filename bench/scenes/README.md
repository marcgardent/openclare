# Scenes

The models of the bench, each as the scene the bench runs it from: a `.blend` that holds the
object to remesh and the settings of the add-on ([how](../README.md#scenes)). They are tracked
through Git LFS.

Most are a plain mesh, one object with its vertices and faces and nothing else: until
9 October 2026 they were OBJ files under `../corpus/`, which are gone; the scenes hold the same
vertices and faces, in single precision, which is how Blender stores them. Two are scenes made
by hand from files of their author, `venturi` and `ogre_hp` (see below).

The authors, sources and licences of all the models are in [CREDITS.md](../../CREDITS.md). The
models whose licence forbids passing them on are not here: they live under `../corpus-large/`,
out of git, with their scenes (`../corpus-large/scenes/`); `make blenderkit` downloads them
again and makes their scenes.

## Synthetic shapes

Made by `../make_corpus.py` (Blender) or by `openclare primitive`. No third-party rights.

| File | What it is |
|---|---|
| `blob.blend` | merged balls, genus 0 |
| `rock.blend` | a sphere with noise at two scales |
| `creature.blend` | a quadruped, a branching shape |
| `cage.blend` | a cube pierced along its three axes then softened, genus 5 |
| `suzanne.blend` | Blender's monkey head, open, in three pieces |
| `part.blend` | a pierced mechanical part, with the long thin triangles of CAD |
| `cube.blend`, `sphere.blend`, `torus.blend`, `lblock.blend`, `cylinder.blend` | yardsticks and test shapes |

## The ogre

`ogre.blend`: the mascot of the project, a sculpt by its author, 4.7 million triangles at first,
brought down to 200,000 by `openclare preprocess`.

The sculpt itself is the scene `../scenes/ogre_hp.blend`, made by `../make_scene.py` from its
author's file: 2.3 million vertices, with three guide curves and one sharp curve drawn on it,
each held on it by a Shrinkwrap modifier, and a vertex group painted as density.

## The car

`../scenes/venturi.blend`: a car modelled in Blender by the author of the project, as it is in
its file, made by `../make_scene.py`: half a body with its Mirror, Subdivision Surface, Edge
Split and Boolean modifiers, in thirteen materials. The add-on remeshes what those modifiers
give: 40,764 triangles, torn along the edges split for shading, with 334 edges of three faces
or more where the boolean cut.

## Keenan Crane's repository

Downloaded on 6 October 2026 from Keenan Crane's
[3D Model Repository](https://www.cs.cmu.edu/~kmcrane/Projects/ModelRepository/) (Carnegie
Mellon), which says: "you can do whatever you like with the data without asking for
permission". Converted by `../import_crane.py`.

| File | Origin | Processing |
|---|---|---|
| `spot.blend` | Spot, the cow | subdivided twice, then 100,000 triangles |
| `bob.blend` | Bob, a rubber-duck float, genus 1 | subdivided twice, then 100,000 triangles |
| `blub.blend` | Blub, the fish | subdivided twice, then 100,000 triangles |
| `pig.blend` | Origins of the Pig, body only | subdivided twice, then 100,000 triangles |
| `jerry.blend` | Jerry the Ogre, face at rest, an open surface | as it is |
| `rose.blend` | Desert Rose, 96 closed shells that cross each other | 150,000 triangles |
| `bridge.blend` | Pittsburgh Bridge, a raw scan with open borders, released into the public domain by its authors (Panoptic Studio, CMU) | as it is |

The characters come as coarse subdivision cages; they are subdivided into smooth dense
surfaces, which is what a remesher is given in practice.
