# Corpus

The models of the bench that are a plain mesh, as OBJ files. The bench does not read them: it
runs on **scenes**, `.blend` files that hold the object to remesh and the settings of the
add-on ([how](../README.md#scenes)). `make scenes` (`../make_scenes.py`) makes the scene of each
of these files, `../scenes/<model>.blend`: one mesh object with the same vertices and the same
faces, in single precision, which is how Blender stores them.

The scenes are to be tracked with Git LFS, and these OBJ files to go once they are. Until
then the OBJ files stay tracked, because the references kept (`../reference/`) only mean
something against these exact models, and the scenes are made from them on each machine.

The authors, sources and licences of all the models are in [CREDITS.md](../../CREDITS.md). The
models whose licence forbids passing them on are not here: they live under `../corpus-large/`,
out of git, with their scenes (`../corpus-large/scenes/`); `make blenderkit` downloads them
again and makes their scenes.

Two models are scenes made by hand, with no OBJ here to make them from: `venturi` and `ogre_hp`
(see below). `venturi.obj` and `venturi.groups` are the car as the bench took it until
9 October 2026, exported without two of its modifiers; nothing reads them any more.

## Synthetic shapes

Made by `../make_corpus.py` (Blender) or by `openclare primitive`. No third-party rights.

| File | What it is |
|---|---|
| `blob.obj` | merged balls, genus 0 |
| `rock.obj` | a sphere with noise at two scales |
| `creature.obj` | a quadruped, a branching shape |
| `cage.obj` | a cube pierced along its three axes then softened, genus 5 |
| `suzanne.obj` | Blender's monkey head, open, in three pieces |
| `part.obj` | a pierced mechanical part, with the long thin triangles of CAD |
| `cube.obj`, `sphere.obj`, `torus.obj`, `lblock.obj`, `cylinder.obj` | yardsticks and test shapes |

## The ogre

`ogre.obj`: the mascot of the project, a sculpt by its author, 4.7 million triangles at first,
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
| `spot.obj` | Spot, the cow | subdivided twice, then 100,000 triangles |
| `bob.obj` | Bob, a rubber-duck float, genus 1 | subdivided twice, then 100,000 triangles |
| `blub.obj` | Blub, the fish | subdivided twice, then 100,000 triangles |
| `pig.obj` | Origins of the Pig, body only | subdivided twice, then 100,000 triangles |
| `jerry.obj` | Jerry the Ogre, face at rest, an open surface | as it is |
| `rose.obj` | Desert Rose, 96 closed shells that cross each other | 150,000 triangles |
| `bridge.obj` | Pittsburgh Bridge, a raw scan with open borders, released into the public domain by its authors (Panoptic Studio, CMU) | as it is |

The characters come as coarse subdivision cages; they are subdivided into smooth dense
surfaces, which is what a remesher is given in practice.
