# Résultats

<p class="subtitle">Des notes de 0 à 100 pour chaque remailleur sur chaque modèle du banc, à 1 000, 3 000 et 8 000 quads demandés. D'après le banc de la nuit du 18 octobre 2026.</p>

Le banc fait tourner openclare et deux autres remailleurs sur {{count:all}} modèles : des formes organiques (sculptures, numérisations, personnages), des pièces mécaniques, deux formes d'étalonnage, et un groupe *extra* de modèles assemblés et générés qui ne sont pas des surfaces propres. Chaque modèle a sa page, avec les images de ce que chaque remailleur en fait, là où sa licence le permet.

Les remailleurs : **openclare** tel que l'extension Blender le lance ; **QuadriFlow**, intégré à Blender ; **AutoRemesher** 1.2.0 de Jeremy Hu. Les notes sont absolues : un remailleur garde sa note quels que soient ceux auxquels on le compare.

Au-delà du banc, l'[état du marché](market.fr.md) : les outils de retopologie pour Blender en 2026, leurs moteurs et où ils sont vendus.

## Notes moyennes

{{averages}}

## Modèle par modèle

<div class="viewer" data-models="all"></div>

{{models}}

{{withheld}}

## Comment les notes sont faites

| Critère | Note pleine pour | 100 à | 0 à | Poids |
|---|---|---|---|--:|
| Validité | variété, aucune face retournée ni écrasée, aucun trou que la source n'a pas ; que des quads, ou presque | chaque essai valide | aucun essai valide | 25 |
| Alignement | les arêtes le long des directions principales de courbure | 0° d'écart | 22,5° d'écart (arêtes au hasard) | 20 |
| Forme des quads | des coins à angle droit | 0° d'écart moyen | 20° | 15 |
| Fidélité | aucun point de la source loin du maillage, parties fines gardées | distance de Hausdorff nulle ; parties fines sur le maillage | 5 % de la diagonale de la boîte englobante ; parties fines à une demi-arête de quad | 15 |
| Régularité | aucun sommet irrégulier au-delà de ce qu'impose la topologie | aucun | 10 pour 100 quads | 10 |
| Structure | peu de patchs dans le complexe de base : des boucles d'arêtes qui courent loin | — | un patch par quad | 5 |
| Boucles d'arêtes | des boucles qui se ferment autour de la forme, aucune qui spirale autour | 2 % des arêtes ou moins en spirale ; un quart des boucles fermées | un cinquième des arêtes en spirale ; aucune boucle fermée | 15 |
| Budget | le nombre de quads demandé | exact | 25 % d'écart | 10 |
| Vitesse | une réponse en une seconde | 1 s | 60 s (logarithmique) | 0 |

La note globale est la moyenne pondérée des critères qui existent pour le modèle : une sphère n'a pas de direction à suivre, un maillage à bords ouverts n'a pas de minimum topologique. Le critère des boucles fait la moyenne de ses deux moitiés, et il est laissé de côté là où le maillage n'est pas une variété. La vitesse figure dans le rapport complet mais ne compte pas : ce n'est pas une question de qualité, et les temps ont été pris sur une machine chargée. Un remailleur qui n'a rendu aucun maillage a 0. La colonne *Quads* n'est pas une note : c'est le nombre de quads du maillage, en moyenne sur les modèles.

Le rapport complet, avec chaque critère pour chaque modèle et chaque remailleur, est [bench/results/SCORES.md](../../bench/results/SCORES.md) dans le dépôt (en anglais) ; les auteurs, sources et licences des modèles sont dans [CREDITS.md](../../CREDITS.md).
