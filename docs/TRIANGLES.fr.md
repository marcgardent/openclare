# Les triangles face au Decimate de Blender

*Output : Triangles* (DT-179) remaille un objet en triangles proches de l'équilatéral, pour la
simulation, l'impression 3D et les maillages de collision ([cas d'usage](USE_CASES.fr.md)).
Blender réduit déjà un maillage en triangles avec son modificateur *Decimate*. Cette page mesure
ce qui les sépare, pour choisir le bon outil selon le travail.

![La tête de Spot à 6 000 triangles : openclare Triangles à gauche, Decimate Collapse à droite](images/triangles/spot-openclare-collapse.jpg)

## En bref

- **La forme des triangles : openclare.** Ses triangles sont proches de l'équilatéral partout : le plus petit angle
  d'un triangle vaut 48° à 54° en moyenne, contre 29° à 43° pour *Collapse* ; le pire 1 % reste entre 20° et 44° sur
  la plupart des modèles (*Collapse* : 1° à 13°), et descend à 6° et 7° sur l'ogre et jerry. Une simulation de tissu
  ou de corps mou, ou une surface régulière à imprimer, demande cela.
- **La fidélité à la forme : Decimate Collapse.** À nombre de triangles égal, il est 1,6 à 20 fois plus proche de la
  source (distance de Hausdorff) : il garde des sommets là où la forme en a besoin, et étire ses triangles pour cela.
  openclare les répartit régulièrement ; sa *taille adaptative* réduit l'écart (l'ogre à 100 % au lieu de 50 % :
  3,27 % au lieu de 4,53 % selon `openclare measure`), sans le combler.
- **Fermé, manifold, comme la source.** Aucun des deux n'ouvre un modèle fermé. openclare a fait quelques triangles
  qui en traversent un autre sur trois modèles (2 à 4), là où *Collapse* n'en a fait aucun ; son rapport dit
  combien, car cela compte pour une imprimante.
- **Le temps.** *Collapse* prend moins d'une seconde ; openclare 1 à 10 secondes.
- ***Planar* et *Un-Subdivide*** ne visent pas un nombre de triangles. Sur des sculptures, ils n'en retirent presque
  pas : *Planar* garde 6 000 à 97 000 triangles des modèles ci-dessous, *Un-Subdivide* presque tous, faute d'avoir été
  faits par subdivision.

## Mesures

Douze modèles du banc, chacun à 6 000 triangles demandés (*Collapse* réglé au ratio qui donne 6 000). Dans chaque
case, **openclare / Collapse**. Sur un i7-7700K (4 cœurs, 8 fils, 62 Go), avec d'autres travaux en cours (charge 6 à
8) : les temps donnent un ordre de grandeur, pas un banc d'essai.

| Modèle | Source | Triangles | Angle minimal, moyenne | Angle minimal, pire 1 % | Rapport des arêtes | Hausdorff | Croisements | Temps |
|---|---|---|---|---|---|---|---|---|
| creature | 78 124 | 6 060 / 5 998 | 53° / 29° | 42° / 5° | 1,17 / 2,37 | 0,19 / 0,03 % | 0 / 0 | 1,6 / 0,4 s |
| blob | 124 004 | 6 048 / 6 000 | 54° / 43° | 44° / 13° | 1,14 / 1,54 | 0,78 / 0,08 % | 0 / 0 | 1,6 / 0,5 s |
| suzanne | 15 744 | 6 449 / 6 000 | 52° / 32° | 12° / 6° | 1,24 / 2,13 | 0,62 / 0,05 % | 81 / 138 | 2,3 / 0,0 s |
| spot | 100 250 | 6 040 / 6 000 | 53° / 34° | 37° / 10° | 1,17 / 1,91 | 0,73 / 0,07 % | 0 / 0 | 2,4 / 0,9 s |
| pig | 100 062 | 6 586 / 6 000 | 51° / 31° | 20° / 7° | 1,23 / 2,07 | 0,73 / 0,18 % | 2 / 0 | 5,5 / 0,5 s |
| ogre | 199 764 | 6 228 / 6 000 | 48° / 34° | 6° / 7° | 1,39 / 1,89 | 3,48 / 0,22 % | 4 / 0 | 10,5 / 1,0 s |
| bob | 102 756 | 5 966 / 6 000 | 52° / 34° | 41° / 10° | 1,18 / 1,88 | 0,54 / 0,05 % | 0 / 0 | 1,6 / 0,5 s |
| jerry | 39 856 | 6 576 / 6 000 | 48° / 30° | 7° / 5° | 1,39 / 2,23 | 1,61 / 0,11 % | 168 / 88 | 3,1 / 0,2 s |
| blub | 106 640 | 6 454 / 6 000 | 51° / 31° | 23° / 7° | 1,21 / 2,11 | 0,55 / 0,05 % | 4 / 0 | 5,2 / 0,5 s |
| rock | 81 920 | 5 844 / 6 000 | 53° / 38° | 43° / 13° | 1,15 / 1,66 | 0,44 / 0,27 % | 0 / 0 | 1,1 / 0,3 s |
| cage | 26 656 | 6 126 / 6 000 | 50° / 32° | 26° / 1° | 1,25 / 3,06 | 0,80 / 0,04 % | 0 / 0 | 1,3 / 0,1 s |
| part | 406 | 6 408 / 406 | 51° / 3° | 30° / 1° | 1,21 / 29,65 | 0,00 / 0,00 % | 0 / 0 | 1,5 / 0,0 s |

- **Angle minimal :** le plus petit angle de chaque triangle ; sa moyenne, et la valeur sous laquelle tombe le pire
  1 %. Un triangle équilatéral a 60°.
- **Rapport des arêtes :** la plus longue arête d'un triangle sur sa plus courte, en moyenne. 1 est équilatéral.
- **Hausdorff :** le plus grand écart entre les deux surfaces, dans un sens ou dans l'autre, en part de la diagonale
  de la boîte du modèle. Mesuré depuis les sommets de chacune vers l'autre, jusqu'à 40 000 sommets de la source.
- **Croisements :** triangles qui en traversent un autre avec lequel ils ne partagent aucun sommet. *suzanne* et
  *jerry* ont des bords ouverts et des croisements à eux dans la source, que toutes les méthodes gardent.
- **part** a 406 triangles : *Collapse* n'a rien à retirer, et openclare en fait 6 408 réguliers.

## Où la mesure s'arrête

- Un seul nombre de triangles, 6 000. Plus fin ou plus grossier, l'équilibre peut bouger.
- openclare avec les réglages de son preset *Default* (*taille adaptative* 50 %) ; l'effet de 100 % n'est mesuré que
  sur deux modèles.
- Aucune simulation n'a tourné et rien n'a été imprimé : la forme des triangles et les croisements sont ce que ces
  usages demandent, pas un essai de ces usages.

## Refaire la mesure

Les mesures sont dans [bench/results/triangles-vs-decimate-6000.json](../bench/results/triangles-vs-decimate-6000.json),
faites par [bench/triangles_vs_decimate.py](../bench/triangles_vs_decimate.py) :

```bash
blender -b --factory-startup --python bench/triangles_vs_decimate.py -- blender bench/results/triangles-vs-decimate-6000.json 6000 bench/out/blob/source.obj
```
