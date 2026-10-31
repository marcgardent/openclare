# Cas d'usage

À quoi sert openclare, noté au fur et à mesure : qui en a besoin, ce que l'usage demande à
l'outil, ce qui y répond, et où cela en est. L'édition Community est l'outil de retopologie ;
l'édition PRO est le pipeline bâti dessus, du remaillage à l'export.
La version anglaise est [USE_CASES.md](USE_CASES.md).

**État :** *fait* est dans le greffon de `main`, *expérimental* quand [FEATURES.fr.md](FEATURES.fr.md) le
dit ; *revue* est dans une pull request ; *prévu* a un ticket.

## Retopologie

| Usage | Qui | Ce qu'il demande | Ce qui y répond | Édition | État |
|---|---|---|---|---|---|
| Une sculpture faite maillage à animer | artistes de personnages | des quads, peu de sommets irréguliers, des boucles autour des yeux, de la bouche, des articulations | le remaillage ; les courbes tracées sur le modèle, la boucle autour d'un membre avec Cut Volume | Community | fait ; les courbes expérimentales |
| Une pièce mécanique en quads propres | modeleurs hard surface | ses arêtes vives gardées comme lignes d'arêtes | Hard Edges, Marked Sharp | Community | fait |
| Un maillage prêt pour la subdivision | modeleurs | les arêtes vives qui restent vives sous un Subdivision Surface | des poids de crease sur les arêtes vives, et sur les arêtes le long d'une courbe selon la courbe | Community | fait ; par courbe expérimental |
| Des coutures où l'artiste les veut | textureurs | des UV coupées le long de lignes choisies | les courbes marquées Seam : les arêtes qui les suivent marquées comme coutures | Community | fait, expérimental |
| Un scan ou un maillage généré par IA rendu utilisable | numérisation, génération | une surface fermée et propre en quads, à partir d'une surface bruitée | le remaillage, qui répare les bords déchirés et les arêtes à plus de deux faces | Community | fait |
| Plus de détail là où il compte | tous | des quads plus petits sur le visage, les mains | densité peinte ; Density from Shape, tailles analysées et montrées | Community | fait ; Density from Shape expérimental (#253) |
| Un maillage où peindre des couleurs de sommets | textureurs, artistes de jeu | des sommets répartis régulièrement, pour que la peinture ait la même résolution partout plutôt que des taches là où la sculpture est grossière et du gaspillage là où elle est dense | le remaillage en quads d'une seule taille (Adaptive Size à 0), ou Output : Triangles, For : Simulation, pour des triangles réguliers | Community | fait |
| Des pôles où l'artiste les veut | artistes de personnages | un pôle écarté d'une articulation ou du visage, épinglé, les autres placés autour | les pôles du dernier résultat gardés sur l'objet, ceux touchés donnés au remaillage suivant ; l'outil pour les déplacer | Community | fait, expérimental (#249) |
| Un modèle symétrique | artistes de personnages | les mêmes quads des deux côtés | Symmetry | Community | fait |
| Des matériaux tenus à part | tous | chaque quad du matériau qu'il recouvre, une ligne d'arêtes entre deux | Materials | Community | fait |
| Des primitives aux bons pôles | modeleurs | une sphère, un cylindre ou un disque en quads | menu Quad Shape | Community | prévu (#159) |

## Triangles

| Usage | Qui | Ce qu'il demande | Ce qui y répond | Édition | État |
|---|---|---|---|---|---|
| Simulation (tissu, corps mous) | animateurs, VFX | des triangles d'une même taille, proches de l'équilatéral | Output : Triangles, For : Simulation, les triangles d'openclare | Community | fait, expérimental (#226) ; comparés : [TRIANGLES.fr.md](TRIANGLES.fr.md) |
| Impression 3D | makers | une surface de triangles fermée, propre, fidèle à la source | Output : Triangles, For : Shape, le Decimate de Blender (Collapse), le plus fidèle ; croisements signalés | Community | fait ; comparés : [TRIANGLES.fr.md](TRIANGLES.fr.md) |
| Maillages de collision | développeurs de jeux | une surface de triangles légère, fidèle à la forme | Output : Triangles, For : Shape, le Decimate de Blender (Collapse) | Community | fait ; comparés : [TRIANGLES.fr.md](TRIANGLES.fr.md) |

## Pipeline

| Usage | Qui | Ce qu'il demande | Ce qui y répond | Édition | État |
|---|---|---|---|---|---|
| Un asset prêt pour le jeu d'un seul geste | artistes de jeu | quads, UV, cartes cuites, niveaux de détail, export | le pipeline bâti sur le remaillage | PRO | prévu |
| Une sculpture reportée sur un Multires | sculpteurs | le remaillage comme base d'une pile Multires reconstruite depuis la source | Multires depuis la source | PRO | prévu |
| Beaucoup d'objets à la fois | studios | une collection remaillée en une fois | remaillage par lot | PRO | prévu |

## Hors de Blender

| Usage | Qui | Ce qu'il demande | Ce qui y répond | Édition | État |
|---|---|---|---|---|---|
| Le moteur dans un autre programme | développeurs | une bibliothèque à l'interface stable | la bibliothèque C et son en-tête, dans chaque version publiée | open source | fait |
| Remaillage par script ou par lot | pipelines, recherche | une ligne de commande | `openclare remesh` et les autres commandes ([USAGE.md](../USAGE.md)) | open source | fait |
