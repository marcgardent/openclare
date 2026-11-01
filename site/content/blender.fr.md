# Le remaillage intégré à Blender

<p class="subtitle">Trois moteurs sont livrés avec Blender : les voxels d'OpenVDB, un octree en dual contouring, et QuadriFlow. Où trouver chacun, ce que font ses réglages, et ce qu'il ne sait pas faire.</p>

Deux des trois reconstruisent la surface sur une grille qui suit les axes, pas la forme : ils nettoient un maillage, ils ne lui donnent pas de boucles d'arêtes. Seul QuadriFlow aligne ses quads sur la forme. Les noms, valeurs par défaut et raccourcis ci-dessous sont lus dans Blender 5.2.2 lui-même ; le comportement est celui du [manuel](https://docs.blender.org/manual/fr/latest/modeling/meshes/retopology.html). Les noms de l'interface sont laissés en anglais.

## Voxel Remesh : OpenVDB

Le maillage est échantillonné sur une grille de voxels par OpenVDB, et une nouvelle surface est construite dessus : fermée, d'un seul tenant, en quads d'une seule taille.

- **Où** : *Properties ‣ Data ‣ Remesh*, mode **Voxel**, bouton **Voxel Remesh**, en Object Mode ou en Sculpt Mode. En Sculpt Mode, **R** règle la taille des voxels dans la vue et **Ctrl R** remaille. En modificateur : *Remesh*, mode **Voxel**.
- **Voxel Size** (0,1) : la taille d'une cellule, donc d'un quad ; plus petite, elle garde des détails plus fins.
- **Adaptivity** (0) : au-dessus de 0, moins de faces là où le détail n'est pas utile, mais des triangles apparaissent et Fix Poles est désactivé. La laisser à 0 pour n'avoir que des quads.
- **Fix Poles** (non) : moins de pôles, un meilleur flux.
- **Preserve Volume** et **Preserve Attributes** (non) : projeter sur l'original pour garder son volume ; reporter ses attributs (masques, face sets, couleurs).
- **Pour** : souder les pièces d'une sculpture qui se chevauchent, fermer les trous, repartir sur des faces régulières pour sculpter.
- **Limites** : les détails plus fins qu'un voxel disparaissent et les arêtes vives s'arrondissent. Il ne lit que le maillage d'origine, ni ses modificateurs ni ses shape keys, et refuse un objet qui porte un modificateur Multiresolution.

## Le modificateur Remesh : un octree en dual contouring

Les trois modes historiques du modificateur *Remesh* construisent la surface sur un octree, par dual contouring.

- **Où** : *Add Modifier ‣ Generate ‣ Remesh*, mode **Blocks**, **Smooth** ou **Sharp**. Il agit en direct ; l'appliquer pour garder le résultat.
- **Blocks** laisse les cubes tels quels, **Smooth** lisse la surface, **Sharp** garde les arêtes et les coins.
- **Octree Depth** (4) : la résolution ; chaque cran donne environ quatre fois plus de faces.
- **Scale** (0,9) : la taille du modèle rapportée à celle de la grille ; plus bas, c'est plus grossier.
- **Sharpness** (1, en Sharp seulement) : plus bas, il filtre le bruit ; plus haut, il suit de plus près les arêtes de l'entrée.
- **Remove Disconnected** (oui) avec **Threshold** (1) : retire les morceaux plus petits que cette part du plus grand.
- **Pour** : une coque fermée et régulière ; Sharp pour une pièce mécanique.
- **Limites** : une grille le long des axes, comme les voxels. L'entrée doit avoir de l'épaisseur : placer un modificateur *Solidify* au-dessus d'un maillage plat.

## QuadriFlow Remesh

QuadriFlow (Huang et al., 2018) suit Instant Meshes : un champ d'orientation sur la surface, puis des quads alignés sur lui, avec des contraintes résolues par flots pour limiter les singularités. Ses quads suivent la forme.

- **Où** : *Properties ‣ Data ‣ Remesh*, mode **Quad**, bouton **QuadriFlow Remesh**, qui ouvre ses réglages.
- **Mode** : **Faces** (par défaut, **Number of Faces** 4 000, approché), **Ratio** (du nombre de faces actuel) ou **Edge Length**.
- **Use Mesh Symmetry** (oui) : un résultat symétrique, selon les axes réglés dans les options de symétrie du maillage.
- **Preserve Sharp** et **Preserve Mesh Boundary** (non) : garder les arêtes vives, garder les bords ouverts.
- **Preserve Attributes** (non) : reprojeter les attributs sur le nouveau maillage.
- **Smooth Normals** (non) : un ombrage lisse sur le résultat.
- **Seed** (0) : une autre graine, une autre disposition des quads.
- **Pour** : le seul des trois qui ressemble à une retopologie.
- **Limites** : une seule taille de quad partout ; plus lent que les voxels. Il ne nettoie pas les auto-intersections : sur une sculpture en désordre, passer d'abord un Voxel Remesh.

## Autour d'eux

- Le modificateur **Decimate** : moins de faces sans nouveau maillage. **Collapse** fusionne des sommets en tenant compte de la forme (Ratio, Symmetry, Triangulate), **Un-Subdivide** défait une subdivision sur une grille, **Planar** dissout les zones planes (Angle Limit).
- **Dyntopo**, en Sculpt Mode : ajoute et retire des triangles sous le pinceau pendant qu'on sculpte (Detailing : Relative, Constant, Brush ou Manual). Il fait des triangles, pas une retopologie ; quand il est actif, **R** et **Ctrl R** le pilotent à la place des voxels.
- **Poly Build**, en Edit Mode : un outil manuel ; Ctrl-clic ajoute un sommet et une face, glisser sur une arête l'extrude en quad, Maj-clic supprime. **Create Quads** laisse des quads.
- La surcouche **Retopology** (*Overlays ‣ Mesh Edit Mode*, avec un **Offset** vers la caméra), l'**accrochage aux faces** et le modificateur **Shrinkwrap** gardent un maillage fait main sur la sculpture.

## En pratique

Voxel Remesh pour nettoyer, QuadriFlow pour orienter, puis Poly Build et l'accrochage là où il faut la main. Les greffons vendus pour Blender lancent souvent ces mêmes moteurs, ou Instant Meshes et QuadWild : voir [les moteurs](engines.fr.md) et [les outils](retopology.fr.md).
