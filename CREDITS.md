# openclare — crédits des modèles

Tous les modèles 3D dont le projet se sert pour ses mesures et ses images, avec leur auteur, leur
source et leur licence. Un modèle n'entre pas dans le banc de mesure sans sa ligne ici.

- **Dans le dépôt** : le fichier est versionné sous `bench/corpus/`, et les maillages de
  comparaison qui en sont tirés sous `bench/reference/`.
- **Hors dépôt** : la licence ne permet pas de redistribuer le modèle. Il est rangé sous
  `bench/corpus-large/`, qui n'est pas versionné, avec tout ce qui en est tiré. Les chiffres
  mesurés, eux, figurent dans les résultats. La colonne « Source » dit comment le retrouver.

## Modèles de l'auteur du projet

| Modèle | Auteur | Année | Source | Licence | Où |
|---|---|---|---|---|---|
| `ogre` | Marc Gardent | 2026 | sculpture originale, 4,7 millions de triangles, ramenée à 200 000 par `retopo preprocess` ; la mascotte du projet | © 2026 Marc Gardent, [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/deed.fr) | dans le dépôt ; l'original hors dépôt |

Convention pour les prochains : **Marc Gardent, 2026, CC BY-SA 4.0**.

La CC BY-SA 4.0 (attribution, partage dans les mêmes conditions) est pour une œuvre ce que la
GPL est pour un logiciel : on peut la réutiliser et la transformer, à condition de citer
l'auteur et de garder la même liberté pour ce qu'on en tire. Creative Commons l'a déclarée
compatible avec la GPLv3 le 8 octobre 2015, dans un seul sens : ce qu'on tire d'une œuvre sous
BY-SA 4.0 peut être placé sous GPLv3, pas l'inverse. Décision du 8 octobre 2026.

## Modèles de tiers

| Modèle | Titre et auteur | Source | Licence | Où | Traitement |
|---|---|---|---|---|---|
| `spot` | *Spot*, Keenan Crane | [3D Model Repository](https://www.cs.cmu.edu/~kmcrane/Projects/ModelRepository/) de Keenan Crane (Carnegie Mellon), téléchargé le 6 octobre 2026 | le dépôt indique : « you can do whatever you like with the data without asking for permission » | dans le dépôt | subdivisé deux fois, puis 100 000 triangles |
| `bob` | *Bob*, Keenan Crane | idem | idem | dans le dépôt | subdivisé deux fois, puis 100 000 triangles |
| `blub` | *Blub*, Keenan Crane | idem | idem | dans le dépôt | subdivisé deux fois, puis 100 000 triangles |
| `pig` | *Origins of the Pig*, Keenan Crane | idem | idem | dans le dépôt | corps seul, subdivisé deux fois, puis 100 000 triangles |
| `jerry` | *Jerry the Ogre*, Keenan Crane | idem | idem | dans le dépôt | visage au repos, tel quel |
| `rose` | *Desert Rose*, Keenan Crane | idem | idem | dans le dépôt | 150 000 triangles |
| `bridge` | *Pittsburgh Bridge*, scan du Panoptic Studio (Carnegie Mellon) | idem | versé au domaine public par ses auteurs | dans le dépôt | tel quel |
| `suzanne` | *Suzanne*, la tête de singe de Blender, Blender Foundation | primitive « Monkey » de Blender, exportée par `bench/make_corpus.py` | livrée avec Blender, logiciel sous licence GNU GPL | dans le dépôt | subdivisée |
| `terrain` | *Mountain Range Terrain*, Bipro Fx | [BlenderKit](https://www.blenderkit.com/), `asset_base_id:43a14c49-93bb-49e7-baec-6397eae071c5`, publié le 7 juillet 2026, récupéré le 8 octobre 2026 | Royalty Free de BlenderKit : utilisable dans un projet, pas redistribuable tel quel | **hors dépôt** | exporté tel quel par `bench/import_blenderkit.py`, 543 557 triangles |

## Formes produites par le projet

Aucun droit de tiers. Elles sont produites par des scripts du dépôt et suivent sa licence
(GPL-3.0-or-later).

| Modèle | Produit par | Contenu |
|---|---|---|
| `blob`, `rock`, `creature`, `cage`, `part` | `bench/make_corpus.py` (Blender) | boules fusionnées ; sphère bruitée ; quadrupède ; cube percé adouci ; pièce mécanique percée |
| `cube`, `sphere`, `torus`, `lblock`, `cylinder` | `retopo primitive` | étalons et formes d'essai |

## Maillages de comparaison

| Quoi | Outil | Conditions |
|---|---|---|
| `bench/reference/quadremesher/` | Quad Remesher 1.4.1, Exoside | licence d'essai, usage non commercial ; lancé depuis Blender 5.2.2 les 6, 7 et 8 octobre 2026 |
| résultats « quadriflow » | QuadriFlow, intégré à Blender | relancé à chaque mesure, rien n'est conservé |

## Ajouter un modèle

1. Vérifier sa licence. Si elle n'autorise pas la redistribution, le modèle va sous
   `bench/corpus-large/` et son nom dans `LOCAL_MODELS` de `bench/run.py`.
2. Ajouter sa ligne dans ce fichier : titre, auteur, source précise (lien ou identifiant), date
   de récupération, licence, traitement appliqué.
3. Pour un modèle BlenderKit : dans Blender, chercher `asset_base_id:<identifiant>` avec le
   module BlenderKit, puis exporter le fichier obtenu :

```bash
blender -b modele.blend --python bench/import_blenderkit.py -- bench/corpus-large/nom.obj
```
