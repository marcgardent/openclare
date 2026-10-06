# Corpus

Les modèles sources du banc de mesure. Ils sont versionnés parce que les mires conservées
(`../reference/`) n'ont de sens que face à ces fichiers exacts.

## Formes de synthèse

Générées par `../make_corpus.py` (Blender) ou par `retopo primitive`. Aucun droit de tiers.

| Fichier | Contenu |
|---|---|
| `blob.obj` | boules fusionnées, genre 0 |
| `rock.obj` | sphère bruitée à deux échelles |
| `creature.obj` | quadrupède, forme à branches |
| `cage.obj` | cube percé selon ses trois axes puis adouci, genre 5 |
| `suzanne.obj` | la tête de singe de Blender, ouverte, en trois pièces |
| `part.obj` | pièce mécanique percée, triangles effilés de type CAO |
| `cube.obj`, `sphere.obj`, `torus.obj` | étalons |

## L'ogre

`ogre.obj` : la mascotte du projet, sculpture fournie par l'auteur du projet, 4,7 millions de
triangles à l'origine, ramenée à 200 000 par `retopo preprocess`. L'original est dans `../corpus-large/`,
hors git.

## Dépôt de Keenan Crane

Téléchargés le 6 octobre 2026 depuis le
[3D Model Repository](https://www.cs.cmu.edu/~kmcrane/Projects/ModelRepository/) de Keenan Crane
(Carnegie Mellon), qui indique : « you can do whatever you like with the data without asking for
permission ». Convertis par `../import_crane.py`.

| Fichier | Origine | Traitement |
|---|---|---|
| `spot.obj` | Spot, la vache | subdivisé deux fois, puis 100 000 triangles |
| `bob.obj` | Bob, bouée canard, genre 1 | subdivisé deux fois, puis 100 000 triangles |
| `blub.obj` | Blub, le poisson | subdivisé deux fois, puis 100 000 triangles |
| `pig.obj` | Origins of the Pig, corps seul | subdivisé deux fois, puis 100 000 triangles |
| `jerry.obj` | Jerry the Ogre, visage au repos, surface ouverte | tel quel |
| `rose.obj` | Desert Rose, 96 coques fermées qui se traversent | 150 000 triangles |
| `bridge.obj` | Pittsburgh Bridge, scan brut à bords ouverts, versé au domaine public par ses auteurs (Panoptic Studio, CMU) | tel quel |

Les personnages sont fournis sous forme de cages de subdivision grossières ; ils sont subdivisés
en surfaces lisses et denses, qui sont ce qu'on donne en pratique à un remailleur.
