# Chaîne de référence (Instant Meshes)

> Ce document décrit la chaîne **déjà écrite**, conservée comme plancher de comparaison.
> La cible du projet est décrite dans [ALGORITHME.md](../ALGORITHME.md), qui remplace la feuille de
> route en fin de page.

## Le problème

À partir d'une surface triangulée quelconque (sculpture, scan, CAO), produire un maillage de quads :

1. dont les arêtes **suivent les formes** (courbures, arêtes vives, bords) ;
2. de **taille maîtrisée** (nombre de faces cible) ;
3. avec le **moins possible de sommets irréguliers** (valence ≠ 4), placés là où la forme l'exige ;
4. qui **reste sur la surface** d'origine.

Toutes les méthodes sérieuses décomposent ça en deux questions : *dans quelle direction vont les
arêtes ?* puis *où passent-elles ?* C'est la famille « field-aligned », et c'est l'ossature de ce
projet.

```
triangles ─► nettoyage ─► hiérarchie ─► champ          ─► champ        ─► extraction ─► relaxation ─► quads
             subdivision               d'orientation      de position
```

## 1. Champ d'orientation (`field::orientation`)

En chaque sommet, on cherche une **croix** : quatre directions tangentes à 90° les unes des
autres, sans distinction entre elles (symétrie 4-RoSy). On la stocke par un seul vecteur unitaire
tangent `q` ; les trois autres bras sont ses rotations autour de la normale.

Le champ doit être **lisse** : deux sommets voisins doivent porter presque la même croix. La
difficulté est la symétrie : pour comparer deux croix, il faut d'abord choisir quels bras se
correspondent. `rosy::compat_orientation` prend la paire de bras la mieux alignée *en 3D*
(énergie « extrinsèque »), ce qui a un effet de bord précieux : le champ s'aligne spontanément
sur les plis et les directions de courbure, sans qu'on les calcule.

Résolution par balayages de Gauss-Seidel : chaque croix est remplacée par la moyenne de ses
voisines (chacune ramenée au bras compatible), reprojetée dans le plan tangent et normalisée.

Là où le champ ne peut pas être lisse apparaissent des **singularités** : en faisant le tour
d'un triangle, la croix a tourné d'un quart de tour. Ce sont les futurs sommets de valence 3
(indice +¼) et 5 (indice −¼). Leur somme est imposée par la topologie (Poincaré-Hopf) :
`#valence3 − #valence5 = 4 χ`, soit 8 pour une sphère et 0 pour un tore. Le test
`orientation_field_obeys_poincare_hopf` le vérifie.

## 2. Hiérarchie multi-résolution (`hierarchy`)

Un lissage local ne propage l'information que d'un voisinage par passe et se fige dans de mauvais
minima locaux, avec des dizaines de singularités inutiles. On construit donc une pyramide : à
chaque niveau, les sommets voisins sont fusionnés deux à deux (les normales les plus proches
d'abord) jusqu'à n'en garder qu'un. On résout au niveau le plus grossier, on recopie la solution
au niveau plus fin, on raffine, et ainsi de suite. Résultat : sur une sphère, on obtient les 8
singularités minimales.

## 3. Champ de position (`field::position`)

Avec la croix et une longueur d'arête `s`, chaque sommet porte une **grille carrée** de pas `s`
dans son plan tangent, représentée par un de ses points `o`. Lisser ce champ, c'est faire
coïncider les grilles voisines pour qu'elles décrivent une seule grille globale, dont les nœuds
seront les sommets des quads.

Même principe que pour l'orientation, avec une symétrie par translations entières : pour comparer
deux grilles, `rosy::compat_position` cherche les deux nœuds les plus proches l'un de l'autre
autour du milieu de l'arête (`middle_point`, le point à la fois proche des deux sommets et dans
leurs deux plans tangents, pour neutraliser la courbure). Après chaque moyenne, `o` est ramené au
nœud le plus proche de son sommet.

## 4. Contraintes (`field::constraints`)

Les bords ouverts et les arêtes vives (angle dièdre au-dessus d'un seuil) imposent :

- une **direction** : la croix a un bras le long de la ligne ;
- un **point** : la grille passe par la ligne, tout en restant libre de glisser le long d'elle ;
- aux **coins** (ligne qui tourne brusquement, ou plus de deux lignes qui se rencontrent) : un
  nœud de la grille est épinglé sur le coin.

Les contraintes sont transportées vers les niveaux grossiers de la hiérarchie.

## 5. Extraction (`extract`)

Pour chaque arête du maillage d'entrée, on calcule le décalage entier entre les nœuds de grille
de ses deux extrémités (`rosy::compat_position_index`) :

- `(0, 0)` : même nœud, les deux sommets sont **fusionnés** (union-find, les correspondances les
  plus fiables d'abord, sans jamais fusionner deux groupes déjà reliés par une arête) ;
- `(±1, 0)` ou `(0, ±1)` : nœuds adjacents, c'est une **arête** du maillage de quads ;
- autre chose (diagonale, saut) : ignoré.

On obtient un graphe plongé sur la surface. Après nettoyage (sommets parasites, jonctions en T,
arêtes pendantes), on trie les voisins de chaque sommet dans le sens trigonométrique autour de la
normale. « Arriver par `u`, repartir par le voisin qui précède `u` » est alors une permutation des
arêtes orientées dont les cycles sont exactement les faces. Les cycles qui repassent par un même
sommet sont découpés à leurs points de pincement.

Le résultat est **quad-dominant** : des quads, plus des triangles et pentagones là où la grille a
des défauts. Pour des **quads purs**, on résout à l'échelle `2s` puis on coupe chaque face autour
de son centre (un n-gone donne n quads).

## 6. Relaxation (`relax`)

Lissage laplacien suivi d'une projection : les sommets libres sur la surface source (BVH,
`spatial`), les sommets proches d'une ligne caractéristique sur cette ligne, les coins restent
fixes. C'est ce qui rend les arêtes vives nettes dans le résultat.

## Limites connues

- **Singularités de position.** Le champ de position est résolu localement : rien ne garantit
  qu'il soit globalement cohérent. Là où il ne l'est pas, on obtient des paires triangle /
  pentagone, donc des paires de sommets valence 3 / valence 5 qui ne servent à rien (3 à 6 % des
  sommets aujourd'hui). C'est la limite structurelle d'Instant Meshes et l'écart principal avec
  Quad Remesher.
- **Arêtes vives.** Les normales sont moyennées de part et d'autre d'un pli : le champ y est moins
  fiable qu'ailleurs, et il reste parfois un défaut le long d'une arête.
- **Densité uniforme.** Pas encore de taille adaptée à la courbure.
- **Entrée.** Supposée à peu près variété et orientée de façon cohérente ; pas de soudure des
  sommets dupliqués, pas de réparation.
- **Trous.** Sur des maillages ouverts ou très bruités, une boucle de plus de 8 arêtes reste
  ouverte (le rapport les compte : `open_loops`).

## Feuille de route

Par ordre d'impact sur la qualité.

1. **Paramétrisation globale à entiers** pour supprimer les singularités de position. Deux voies :
   - *QuadriFlow* : garder nos deux champs et rendre les décalages entiers cohérents par un flot
     à coût minimal. Chemin le plus court depuis le code actuel.
   - *QuadWild* : tracer les séparatrices du champ de croix pour découper la surface en patchs,
     puis choisir le nombre de quads par côté (quantification par programme entier ou BiMDF).
     Plus lourd, mais c'est ce qui donne les boucles d'arêtes propres attendues en modélisation.
2. **Champ de croix guidé** : directions de courbure principale comme contraintes souples,
   traits dessinés par l'utilisateur, solveur intrinsèque (Knöppel et al.) en alternative, fusion
   des paires de singularités proches.
3. **Arêtes vives propres** : normales séparées de chaque côté d'un pli, groupes de lissage,
   coutures UV et limites de matériaux comme lignes caractéristiques.
4. **Densité adaptative** : champ de taille piloté par la courbure ou peint.
5. **Symétrie** : détection d'un plan de symétrie et résultat symétrique.
6. **Robustesse** : soudure, entrée non variété, composantes minuscules, remplissage des trous.
7. **Performance** : Gauss-Seidel parallèle par coloration de graphe (rayon), SIMD.
8. **Intégration** : add-on Blender, API C, formats PLY / glTF.
9. **Mesure** : banc d'essai comparant nombre de singularités, distorsion des quads et distance
   de Hausdorff avec Instant Meshes, QuadriFlow et QuadWild.

## Références

Les articles sont en accès libre sur les pages de leurs auteurs.

| Article | Ce qu'on en tire | Code de référence (licence) |
|---|---|---|
| Jakob, Tarini, Panozzo, Sorkine-Hornung, *Instant Field-Aligned Meshes*, SIGGRAPH Asia 2015 | étapes 1 à 5, implémentées ici | `wjakob/instant-meshes` (BSD-3) |
| Huang, Zhou, Niessner, Shewchuk, Guibas, *QuadriFlow*, SGP 2018 | cohérence globale par flot | `hjwdzh/QuadriFlow` (MIT) |
| Pietroni, Nuvoli, Alderighi, Cignoni, Tarini, *Reliable Feature-Line Driven Quad-Remeshing*, SIGGRAPH 2021 | patchs guidés par les arêtes vives | `nicopietroni/quadwild` (GPL-3) |
| Heistermann, Warnett, Bommes, *Min-Deviation-Flow in Bi-directed Graphs for T-Mesh Quantization*, SIGGRAPH 2023 | quantification sans solveur commercial | `cgg-bern/quadwild-bimdf` (GPL-3) |
| Bommes, Zimmer, Kobbelt, *Mixed-Integer Quadrangulation*, SIGGRAPH 2009 | formulation de référence | CoMISo (GPL) |
| Bommes, Campen, Ebke, Alliez, Kobbelt, *Integer-Grid Maps for Reliable Quad Meshing*, SIGGRAPH 2013 | garanties de validité | — |
| Ebke, Bommes, Campen, Kobbelt, *QEx: Robust Quad Mesh Extraction*, SIGGRAPH Asia 2013 | extraction robuste | `libQEx` (GPL-3) |
| Campen, Bommes, Kobbelt, *Quantized Global Parametrization*, SIGGRAPH Asia 2015 | quantification robuste | — |
| Knöppel, Crane, Pinkall, Schröder, *Globally Optimal Direction Fields*, SIGGRAPH 2013 | champ de croix par valeurs propres | Directional (MPL-2) |
| Bommes et al., *Quad-Mesh Generation and Processing: A Survey*, 2013 | vue d'ensemble | — |
| Vaxman et al., *Directional Field Synthesis, Design, and Processing*, 2016 | vue d'ensemble des champs | — |

Le code de cette chaîne est écrit d'après l'article d'Instant Meshes. Le projet est sous
GPL-3.0-or-later ; la politique de réutilisation du code existant est décrite dans
[ALGORITHME.md](../ALGORITHME.md).
