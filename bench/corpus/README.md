# Corpus

The source models of the bench. They are tracked because the references kept (`../reference/`)
only mean something against these exact files.

The authors, sources and licences of all the models are in [CREDITS.md](../../CREDITS.md). The
models whose licence forbids passing them on are not here: they live under `../corpus-large/`,
out of git, and `make blenderkit` downloads them again.

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
brought down to 200,000 by `openclare preprocess`. The original is in `../corpus-large/`, out of
git.

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
