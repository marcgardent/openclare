# Fonctions

*[English version](FEATURES.md)*

Chaque fonction d'openclare, dans le greffon Blender, en ligne de commande et en bibliothèque, et
jusqu'où on peut s'y fier. Ce qui reste à faire pour chacune est dans la
[feuille de route](../ROADMAP.fr.md).

État au 31 octobre 2026, greffon 0.18.0. Dans le greffon, les fonctions expérimentales portent
l'icône d'une fiole, et *Experimental Features*, dans ses préférences, les cache toutes.

## Stable ou expérimental

Une fonction est **stable** quand tout ce qui suit est vrai :

- elle est sur `main`, et donc dans la prochaine version ;
- un test la couvre, celui du moteur ou celui du greffon lancé dans Blender ;
- elle fait ce qu'elle dit sur le banc, ou sur un cas dont on connaît la réponse ;
- aucune issue de sévérité *high* ou *medium* n'est ouverte contre elle.

Sinon elle est **expérimentale** : elle tourne et on peut s'en servir, mais elle peut changer,
et l'issue citée à côté dit ce qui lui manque. Les issues qui portent sur toute la chaîne plutôt
que sur une fonction (petits morceaux, poches profondes, sculptures de millions de triangles)
sont à part, dans [Limites de la chaîne](#limites-de-la-chaîne) : elles bornent toutes les
fonctions.

## Le greffon : le remaillage

Le panneau *openclare* de l'éditeur Properties, dans l'onglet du maillage.

| Fonction | État | Ce qui lui manque |
|---|---|---|
| *Remesh to Quads*, *Remesh Again*, *Stop*, *Select Original* | stable | |
| *Output: Quads* | stable | |
| *Output: Triangles*, pour *Simulation* | expérimental | quelques triangles en traversent un autre sur 3 modèles sur 12 ([TRIANGLES.fr.md](TRIANGLES.fr.md), #226) |
| *Output: Triangles*, pour *Shape* (le *Decimate* de Blender) | stable | ne suit ni arête vive, ni courbe, ni matériau, comme le panneau le dit |
| *Ask For: Number* | stable | |
| *Ask For: Sizes* (*Largest*, *Smallest*) | expérimental | tailles proposées à partir d'une constante cachée, doigts perdus sur un corps (#253) |
| *Analyse* | expérimental | la même chose (#253) |
| *Show Sizes* | expérimental | montre ces tailles (#253) |
| *Density from Shape* | expérimental | lit la forme avec ces tailles (#253) |
| *Adaptive Size* | stable | |
| *Quads Around Thin Parts* | expérimental | la borne sur la taille a des limites connues (#38) ; parties plus fines qu'un quad aux petits budgets (#18) |
| *Hard Edges*, *Angle* | stable | |
| *Marked Sharp* | stable | |
| *Shading* | stable | |
| *Follow Curvature* | expérimental | valeur par défaut pas réglée sur le corpus (#13) ; scans bruités et terrains (#21, #33) |
| *Fewer Irregular Vertices* | expérimental | une portée entre 3,2 et 3,7 plie un quad sur le corps d'homme (#72) |
| *Volume Fit* | expérimental | le volume après subdivision n'est pas gardé (#39) |
| *Density* (un groupe de sommets) et *Factor* | stable | |
| *Symmetry* | stable | |
| *Materials* : chaque quad prend le matériau sous lui | stable | |
| *Materials* : une ligne d'arêtes entre deux | expérimental | presque toutes lâchées sur un vrai modèle (#47) |
| *Sharp Edges*, *Creases* sur le résultat | stable | |
| *Shape Presets* : *Default* et ceux de l'utilisateur | stable | |

## Le greffon : courbes et pôles

Les outils de la vue 3D, en Object Mode.

| Fonction | État | Ce qui lui manque |
|---|---|---|
| *Draw Curve*, avec son aimantation (*Ridge*, *Valley*, rayon sur F) | expérimental | des courbes tracées sur un cube ne laissent aucune arête (#195) ; les courbes ne comptent pas dans les tailles et le budget (#196) ; pas gardées partout (#44) |
| *Cut Volume* | expérimental | fait les mêmes courbes (#195, #196, #44) |
| *Mesh*, *Sharp*, *Crease*, *Seam* d'une courbe | expérimental | une marque ne vaut que si la ligne d'arêtes qui la porte est là (#195) |
| *Select Curves*, *Deselect Curves* | stable | |
| Les pôles du dernier résultat, gardés sur l'objet et affichés ; ceux qu'on touche imposés au remaillage suivant | expérimental | nouveaux au 31 octobre 2026, ni mesurés sur le banc ni essayés par des artistes (#249) |
| L'outil *Poles* : glisser, ajouter (3, 5), épingler (P), retirer (X), *Free All*, *Clear* | expérimental | la même chose (#249) |

## Plateformes

| Fonction | État | Ce qui lui manque |
|---|---|---|
| Le greffon sous Linux | stable | |
| Le greffon sous Windows | stable | installé et lancé dans un Blender pour Windows le 31 octobre 2026 ; l'arrêt d'un calcul par Échap n'y a pas été essayé |
| Le greffon sous macOS | expérimental | compilé et testé comme bibliothèque, jamais lancé dans Blender ; signé ad hoc, pas notarisé (#110) |

## La ligne de commande

`openclare remesh` et les autres commandes ([USAGE.md](../USAGE.md)).

| Fonction | État | Ce qui lui manque |
|---|---|---|
| `remesh` avec `--quads`, `--seed`, `--notes` | stable | |
| `--preset`, `--config`, `--save-config` | stable | |
| Les préréglages livrés, `organic`, `animation`, `hard-surface` (`presets`) | expérimental | leurs valeurs ne sont pas réglées sur le corpus (#13) |
| `--adaptivity`, `--crease-angle`, `--density`, `--density-contrast`, `--symmetry` | stable | |
| `--alignment` | expérimental | comme *Follow Curvature* (#13, #21, #33) |
| `--singularity-reach`, `--singularity-tolerance` | expérimental | comme *Fewer Irregular Vertices* (#72) |
| `--volume-fit` | expérimental | comme *Volume Fit* (#39) |
| `--quads-around` | expérimental | comme *Quads Around Thin Parts* (#38, #18) |
| `--largest`, `--smallest`, `--most-quads`, `analyse` | expérimental | comme *Ask For: Sizes* (#253) |
| `--edge-length` | expérimental | plus ancien que les tailles, hors du banc |
| `--groups` : le groupe de chaque quad | stable | |
| `--groups` : une ligne d'arêtes entre deux | expérimental | comme *Materials* (#47) |
| `--curves` | expérimental | comme *Draw Curve* (#195, #196, #44) |
| `--poles`, `--poles-out` | expérimental | comme les pôles du greffon (#249) |
| `--edge-angle` | expérimental | arêtes vives du maillage de travail, désactivé par défaut, pas encore dans le greffon |
| `--method triangles` | expérimental | comme *Output: Triangles* (#226) |
| `--method integer-grid`, `--method instant-meshes` | expérimental | autres chaînes, gardées pour comparer (#9) |
| `--quality slow`, `--slow-work-limit`, `--slow-time-limit` | expérimental | une piste de recherche, en ligne de commande seulement (#138, #144) |
| `--skip-pieces-under`, `--no-close-loops` | expérimental | des outils pour regarder dans un calcul |
| `measure`, `layout`, `primitive` | stable | |
| `field`, `parametrize`, `tmesh`, `quantize`, `preprocess` | expérimental | les étapes de la chaîne une à une : leurs options suivent la chaîne |
| `--record`, `--record-still`, `openclare-view` (Rerun) | expérimental | une version de Rerun en retard, des étapes non montrées (#35) |

## Bibliothèques

| Fonction | État | Ce qui lui manque |
|---|---|---|
| La bibliothèque C, `openclare-capi`, dans chaque version publiée | stable | `quality = slow` y donne la qualité standard |
| La bibliothèque Rust, `openclare` | expérimental | version 0 : son interface change d'une version à l'autre |

## Limites de la chaîne

Ce qui borne toutes les fonctions, et qu'aucune ne peut régler seule. Sévérité *high* d'abord.

- Petits morceaux séparés : quelques-uns passent encore par la chaîne de repli, ou n'en
  sortent pas (#34).
- Les poches profondes et les cavités sont refermées par le maillage (#36).
- Sculptures de millions de triangles : l'essentiel du temps part dans le prétraitement (#45).
- Le maillage de travail a des triangles d'une seule taille : ce qui est plus fin est perdu en
  premier (#48).
- Des triangles de presque aucune aire laissés par le prétraitement (#31).
- Des bandes étroites de quads là où des arcs restent ouverts (#5), et un complexe de base
  doublé par les jonctions en T (#20).
- Inversions : deux des trois lignes de défense ne sont pas écrites (#10).
- Aucune borne sur le temps de calcul pour des entrées faites pour le casser (#27).
