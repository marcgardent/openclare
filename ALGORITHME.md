# Cahier des charges : moteur de retopologie quad

**Statut : validé le 6 octobre 2026. Jalons 0 à 2 faits ; jalon 3 réalisé, en attente de revue.** Les points marqués
*à trancher* attendent une décision ; ils sont regroupés en section 7.

Première chaîne, pour l'organique : **champ de croix → paramétrisation sans couture → T-mesh →
quantification entière → carte à grille entière → extraction**. C'est la lignée MIQ / Integer-Grid
Maps / QGP, retenue parce qu'elle a le plafond de qualité le plus haut sur les formes organiques.

Seconde chaîne, pour le durci (hard-surface) : **champ de croix → patchs → quantification →
remplissage par motifs**, la lignée QuadWild. Elle vient après la première, mais l'architecture la
prévoit dès maintenant : les deux partagent leurs étapes communes (section 5).

---

## 0. Objectif et critère d'acceptation

**Objectif : un résultat supérieur au remaillage quad de Blender.**
**Priorité : formes organiques d'abord ;** arêtes vives, courbes guides et densité locale ensuite.

### L'adversaire

Le remaillage quad de Blender est QuadriFlow, exposé par l'opérateur
`bpy.ops.object.quadriflow_remesh` (signature relevée sur Blender 5.2.2 LTS). QuadriFlow est
lui-même Instant Meshes modifié pour produire beaucoup moins de singularités. La chaîne déjà
écrite dans ce dépôt est un Instant Meshes : elle est donc attendue **en dessous** de l'adversaire,
et sert de plancher mesurable.

### Protocole

- Mêmes maillages d'entrée, même nombre de faces demandé (`mode='FACES'`, `target_faces=N`).
- `use_mesh_symmetry=False` : sa valeur par défaut est `True`, ce qui fausserait la comparaison.
- `use_preserve_sharp` et `use_preserve_boundary` activés quand le modèle a des arêtes vives ou
  des bords, pour comparer à l'adversaire dans sa meilleure configuration.
- QuadriFlow dépend d'une graine (`seed`) : on compare la médiane sur 5 graines.

### Métriques

| | Métrique | Définition | Rôle |
|---|---|---|---|
| V | Validité | 100 % de quads, variété, fermé si l'entrée l'est, aucune face inversée | **bloquante** |
| S | Sommets irréguliers | nombre de sommets intérieurs de valence ≠ 4, et excès sur le minimum topologique (4·\|χ\| pour une surface fermée) | qualité |
| C | Complexité du complexe de base | nombre de patchs obtenus en prolongeant les arêtes depuis chaque sommet irrégulier ; une spirale le fait exploser | qualité |
| A | Alignement | angle moyen entre arêtes et directions de courbure principale, pondéré par l'anisotropie \|k₁−k₂\| | qualité |
| F | Forme des quads | écart des angles à 90° (moyenne, 95ᵉ centile), rapport d'aspect, dispersion des aires | qualité |
| H | Fidélité | distance de Hausdorff et distance moyenne à la source, rapportées à la diagonale englobante | garde-fou |
| N | Budget | écart relatif au nombre de faces demandé | garde-fou |
| T | Temps | temps de calcul | information |

### Règle d'acceptation

Sur les modèles organiques du corpus : **V jamais moins bon que QuadriFlow, et médiane meilleure
sur S, C, A et F**, sans dégrader H ni N au-delà d'une marge.

### Résultats du jalon 0

Campagne complète dans [bench/results/RESULTS.md](bench/results/RESULTS.md) : 9 modèles,
3000 quads demandés, médiane sur 5 graines. Mode d'emploi et limites dans
[bench/README.md](bench/README.md).

Quad Remesher (version d'essai 1.4.1) sert de **mire** : ce n'est pas l'adversaire à battre pour
l'acceptation, mais le niveau visé à terme. Il est mesuré en réglage uniforme (taille de quad
constante, pour comparer à armes égales) et n'a pas de graine, donc une seule exécution.

Sur les cinq modèles organiques :

| | QuadriFlow (Blender) | Quad Remesher (uniforme) | Plancher (Instant Meshes) |
|---|---|---|---|
| Maillages valides (V) | 18 sur 25 | 5 sur 5 | 22 sur 25 |
| Alignement (A), degrés | 8,1 à 23,0 | 2,6 à 14,7 | 8,0 à 23,2 |
| Sommets irréguliers (S) | 8 à 78 | 24 à 101 | 91 à 212 |
| Patchs du complexe de base (C), pour 3000 faces | 1943 à 2884 | 1001 à 2531 | 2319 à 3048 |
| Écart d'angle moyen (F), degrés | 5,8 à 8,7 | 4,3 à 10,6 | 4,7 à 7,1 |
| Écart au budget (N) | 1 à 11 % | 7 à 16 % | 2 à 3 % |
| Temps (T), secondes | 1,3 à 4,2 | 3 à 17 | 0,3 à 1,5 |

Ce que ces chiffres établissent :

- **Ce qui distingue Quad Remesher, c'est l'alignement.** Sur les formes lisses (blob, créature,
  cage) ses arêtes sont à 2,6°, 4,6° et 5,3° des directions de courbure, contre 8,1° à 17,5° pour
  QuadriFlow : environ trois fois mieux. Ses boucles d'arêtes font le tour des trous et des
  membres ; celles de QuadriFlow forment une grille qui les ignore.
- **Quad Remesher ne minimise pas les singularités.** Il en a autant que QuadriFlow sur les formes
  lisses, et douze fois plus sur le rocher bruité (101 contre 8), où il suit le relief. Peu de
  sommets irréguliers n'est donc pas l'objectif en soi : c'est le prix qu'on accepte de payer
  pour l'alignement.
- **Sa structure est meilleure sans être simple** : environ moitié moins de patchs que QuadriFlow
  sur les formes lisses, mais encore un millier pour 3000 faces.
- **Il tient mal le budget** : 7 à 16 % d'écart en réglage uniforme, et jusqu'à presque le double
  de faces demandées en réglage par défaut (cage).
- **QuadriFlow n'est pas toujours valide.** Faces repliées sur le tore (4 graines sur 5), sur
  Suzanne (5 sur 5) et sur le blob (2 sur 5) ; une face d'aire nulle sur la pièce mécanique
  (1 sur 5). Constat recoupé par un second test indépendant de la source.
- **Le plancher est loin derrière sur les singularités** : 3 à 11 fois plus que QuadriFlow.
- **Sur le durci**, QuadriFlow et Quad Remesher font jeu égal : 8 sommets irréguliers et 6 patchs
  sur le cube (les minimums), 24 à 28 sommets irréguliers sur la pièce percée.

### Sur un vrai modèle : l'ogre

L'ogre, mascotte du projet, est une vraie sculpture de 4,7 millions de triangles. Le banc la
mesure à partir d'une source de 200 000 triangles produite par notre prétraitement. Résultats à
8000 quads demandés ([RESULTS-8000.md](bench/results/RESULTS-8000.md) ; 3000 et 30 000 quads sont
mesurés aussi) :

| | QuadriFlow | Quad Remesher (défaut) | Quad Remesher (uniforme) | Plancher |
|---|---|---|---|---|
| Maillages valides (V) | 0 sur 5 | 1 sur 1 | 0 sur 1 | 0 sur 5 |
| Alignement (A), degrés | 17,8 | 8,5 | 9,8 | 17,7 |
| Sommets irréguliers (S) | 162 | 338 | 322 | 455 |
| Hausdorff (H), ‰ | 27,3 | 5,2 | 25,6 | 30,7 |
| Écart au budget (N) | 3 % | 59 % | 16 % | 2 % |
| Temps (T), secondes | 4,3 | 23,5 | 24,0 | 2,8 |

- **La taille adaptative est ce qui sauve les détails fins.** En réglage par défaut, Quad Remesher
  garde les doigts et les oreilles (Hausdorff 5,2 ‰) ; en taille uniforme il les perd comme les
  autres (25,6 ‰). Le prix : 59 % de quads de plus que demandé.
- **Tout le monde échoue quelque part.** QuadriFlow sort 9 à 14 faces repliées à chaque graine.
  Notre plancher perd carrément les mains : les doigts se détachent en morceaux séparés. Quad
  Remesher en taille uniforme rend 4 faces qui ne sont pas des quads.
- **Un vrai modèle est bien plus dur que le corpus de synthèse** : même Quad Remesher y laisse
  plus de 300 sommets irréguliers et n'aligne qu'à 9°. QuadriFlow et le plancher y sont proches du
  hasard (22,5°).

Conséquence à trancher : la densité adaptative était au rang 4 des contraintes (section 2). Sur
un personnage, c'est elle qui décide si les mains existent encore. Elle mérite sans doute de
remonter, au moins comme réglage automatique piloté par la courbure.

### Sur le corpus élargi

Avec l'ogre et les sept modèles de Keenan Crane, le corpus compte 13 modèles organiques. À 3000
quads demandés, nombre de modèles où chaque outil a la meilleure médiane :

| | QuadriFlow | Plancher | Quad Remesher (défaut) | Quad Remesher (uniforme) |
|---|---|---|---|---|
| Alignement (A) | 0 | 0 | 13 | 0 |
| Sommets irréguliers (S) | 5 | 1 | 1 | 6 |
| Patchs du complexe de base (C) | 1 | 1 | 2 | 9 |
| Maillages valides (V) | 36 sur 65 | 47 sur 65 | 8 sur 13 | 10 sur 13 |

- **Quad Remesher gagne l'alignement sur les 13 modèles**, sans exception : c'est bien là qu'est
  l'écart à combler.
- **Personne n'est valide partout.** QuadriFlow refuse purement le scan brut (Pittsburgh Bridge)
  et replie des faces sur le cochon à chaque graine. Quad Remesher rend quelques faces non quads
  sur le scan, sur Jerry et sur Desert Rose.
- **Desert Rose (96 coques qui se traversent) met tout le monde en échec** ; nos métriques de
  validité y sont elles-mêmes peu fiables. **Décidé : elle ne compte pas dans l'acceptation** ;
  elle servira de cas de test au prétraitement de réparation (J11).

Deux réserves sur les métriques elles-mêmes :

- **C sature.** Près du nombre de faces, elle ne départage plus : un maillage avec davantage de
  singularités peut même obtenir un C un peu plus bas, ses lignes s'arrêtant plus tôt. Elle ne
  se lit qu'avec S.
- **A dépend de l'échelle** à laquelle on lit la courbure (ici celle d'un quad). Sur le rocher
  bruité, QuadriFlow et le plancher sont au niveau du hasard (22,5°).

### Seuils d'acceptation (adoptés)

Sur chaque modèle organique, médiane sur 5 graines, à budget égal. Deux niveaux : l'acceptation
(mieux que Blender) et la visée (le niveau de Quad Remesher). La visée est un palier, pas un
plafond : si on peut faire mieux que Quad Remesher, on ne s'arrête pas là.

| Métrique | Acceptation | Visée |
|---|---|---|
| V | 5 maillages valides sur 5 | idem |
| A | au moins 2° de mieux que QuadriFlow, là où il fait mieux que 20° | 6° ou moins sur les formes lisses |
| S | au plus 1,5 fois QuadriFlow | — |
| C | pas plus que QuadriFlow | la moitié de QuadriFlow |
| F | écart d'angle moyen pas plus élevé que QuadriFlow | idem |
| H | Hausdorff au plus 1,2 fois celui de QuadriFlow | idem |
| N | écart au budget d'au plus 5 % | idem |

L'alignement est en tête et l'exigence sur les sommets irréguliers est souple : la mire montre que
les deux se négocient l'un contre l'autre, et que c'est l'alignement qui fait la qualité perçue.

### Corpus

Le jalon 0 tourne sur un **corpus provisoire**, entièrement généré dans Blender pour ne rien
télécharger, et versionné dans `bench/corpus/` avec les mires Quad Remesher correspondantes
(`bench/reference/`, 3 budgets et 2 réglages par modèle) : cinq formes organiques (blob, rocher
bruité, créature à pattes, cage de genre 5, Suzanne), deux mécaniques (pièce percée, cube) et deux
étalons (sphère, tore). Elles couvrent les cas utiles, mais ce sont des formes de synthèse, pas de
vraies sculptures.

S'y ajoutent des modèles réels, détaillés dans [bench/corpus/README.md](bench/corpus/README.md) :

- **l'ogre**, mascotte du projet (voir plus haut). Son original de 4,7 millions de triangles pèse
  200 Mo : trop pour git, il est gardé hors versionnement dans `bench/corpus-large/`. Seule sa
  version à 200 000 triangles est versionnée ;
- **sept modèles du dépôt de Keenan Crane**, libres de tout usage : quatre personnages (Spot, Bob,
  Blub, le cochon), un visage ouvert (Jerry), une forme à 96 coques (Desert Rose) et un scan brut
  (Pittsburgh Bridge).

Le corpus compte ainsi 17 modèles, tous avec leurs mires Quad Remesher. Il manque encore de
vraies sculptures denses autres que l'ogre : les scans classiques (Stanford) ont des conditions
réservées à la recherche et ont été écartés.

---

## 1. Références

Chaque entrée a été contrôlée sur la page de l'éditeur ou des auteurs (lien).

### L'adversaire et le plancher

| Référence | Rôle ici |
|---|---|
| Huang, Zhou, Niessner, Shewchuk, Guibas, [*QuadriFlow: A Scalable and Robust Method for Quadrangulation*](https://onlinelibrary.wiley.com/doi/10.1111/cgf.13498), SGP 2018 | l'adversaire (Blender) |
| Jakob, Tarini, Panozzo, Sorkine-Hornung, [*Instant Field-Aligned Meshes*](https://dl.acm.org/doi/10.1145/2816795.2818078), SIGGRAPH Asia 2015 | le plancher (chaîne déjà écrite) |

### Champ de croix

| Référence | Rôle ici |
|---|---|
| Knöppel, Crane, Pinkall, Schröder, [*Globally Optimal Direction Fields*](https://publications.ri.cmu.edu/globally-optimal-direction-fields), SIGGRAPH 2013 | formulation retenue : énergie quadratique, problème aux valeurs propres creux, alignement sur la courbure |
| Bommes, Zimmer, Kobbelt, [*Mixed-Integer Quadrangulation*](https://cgg.unibe.ch/publication/031044/), SIGGRAPH 2009 | représentation angle + sauts de période ; les deux étapes (champ, puis paramétrisation) de la lignée |
| Vaxman, Campen, Diamanti, Panozzo, Bommes, Hildebrandt, Ben-Chen, [*Directional Field Synthesis, Design, and Processing*](https://cgg.unibe.ch/publication/016/), Eurographics STAR 2016 | panorama des représentations |

### Paramétrisation et validité

| Référence | Rôle ici |
|---|---|
| Bommes, Campen, Ebke, Alliez, Kobbelt, [*Integer-Grid Maps for Reliable Quad Meshing*](https://cgg.unibe.ch/publication/031197/), SIGGRAPH 2013 | définition de la carte à grille entière et conditions de validité |
| Campen, Shen, Zhou, Zorin, *Seamless Parametrization with Arbitrary Cones for Arbitrary Genus*, ACM TOG 39(1), 2019 | repli avec garantie si les inversions locales résistent |

### T-mesh et quantification

| Référence | Rôle ici |
|---|---|
| Campen, Bommes, Kobbelt, [*Quantized Global Parametrization*](https://cgg.unibe.ch/publication/031251/), SIGGRAPH Asia 2015 | colonne vertébrale : T-mesh, quantification, résultat valide garanti |
| Eppstein, Goodrich, Kim, Tamstorf, [*Motorcycle Graphs: Canonical Quad Mesh Partitioning*](https://disneyanimation.com/publications/motorcycle-graphs-canonical-quad-mesh-partitioning/), SGP 2008 | principe du « motorcycle graph » |
| Heistermann, Warnett, Bommes, [*Min-Deviation-Flow in Bi-directed Graphs for T-Mesh Quantization*](https://cgg.unibe.ch/publication/011282/), SIGGRAPH 2023 | solveur de quantification sans solveur commercial |
| Marinov, Lyon, Born, [*Target face number for quantized global parameterization*](https://www.researchgate.net/publication/403764812_Target_face_number_for_quantized_global_parameterization), CAGD 2026 | tenir le budget de quads |
| Lyon, Campen, Kobbelt, [*Simpler Quad Layouts using Relaxed Singularities*](https://onlinelibrary.wiley.com/doi/full/10.1111/cgf.14365), CGF 2021 | déplacer ou supprimer des singularités pendant la quantification |

### Extraction

| Référence | Rôle ici |
|---|---|
| Ebke, Bommes, Campen, Kobbelt, [*QEx: Robust Quad Mesh Extraction*](https://cgg.unibe.ch/publication/031204/), SIGGRAPH Asia 2013 | extraction robuste aux imprécisions numériques |
| Ray, [*On Quad Mesh Extraction From Messy Grid Preserving Maps*](https://arxiv.org/abs/2507.15404), 2025 | ce qui distingue une carte réelle d'une carte idéale |

### Prétraitement et passage à l'échelle

| Référence | Rôle ici |
|---|---|
| Garland, Heckbert, [*Surface Simplification Using Quadric Error Metrics*](https://dl.acm.org/doi/10.1145/258734.258849), SIGGRAPH 1997 | décimation |
| Botsch, Kobbelt, [*A Remeshing Approach to Multiresolution Modeling*](https://dl.acm.org/doi/10.1145/1057432.1057457), SGP 2004 | remaillage isotrope |
| Rusinkiewicz, [*Estimating Curvatures and Their Derivatives on Triangle Meshes*](https://gfx.cs.princeton.edu/pubs/Rusinkiewicz_2004_ECA/curvpaper.pdf), 3DPVT 2004 | courbures et directions principales (retenu) |
| Meyer, Desbrun, Schröder, Barr, [*Discrete Differential-Geometry Operators for Triangulated 2-Manifolds*](https://multires.caltech.edu/pubs/diffGeoOps.pdf), VisMath 2002 | alternative pour les courbures |
| Ebke, Schmidt, Campen, Kobbelt, [*Interactively Controlled Quad Remeshing of High Resolution 3D Models*](https://history.siggraph.org/learning/interactively-controlled-quad-remeshing-of-high-resolution-3d-models/), SIGGRAPH Asia 2016 | entrées de plusieurs millions de triangles, guides utilisateur |

### Seconde chaîne, pour le durci

| Référence | Rôle ici |
|---|---|
| Pietroni, Nuvoli, Alderighi, Cignoni, Tarini, [*Reliable Feature-Line Driven Quad-Remeshing*](http://vcg.isti.cnr.it/publication/2021/PNACT21/) (QuadWild), SIGGRAPH 2021 | base de la chaîne durci : arêtes vives comme bords de patchs. Moins bonne sur l'organique, d'où son rang de seconde chaîne. Sert aussi de repli si la paramétrisation échoue (section 6). |

### Codes existants et licences

| Code | Licence | Usage permis ici |
|---|---|---|
| [`hjwdzh/QuadriFlow`](https://github.com/hjwdzh/QuadriFlow) | MIT | portage possible ; sert surtout d'adversaire |
| [`wjakob/instant-meshes`](https://github.com/wjakob/instant-meshes) | BSD 3-Clause | portage possible |
| [`cgg-bern/libSatsuma`](https://github.com/cgg-bern/libSatsuma) (Bi-MDF) | MIT ; son couplage optionnel Blossom-V n'est pas libre | portage possible, **sauf Blossom-V** |
| [`komietty/metriko`](https://github.com/komietty/metriko) (chaîne QGP complète) | MPL-2.0 ; stade précoce, sans gestion des bords | portage possible |
| [`nicopietroni/quadwild`](https://github.com/nicopietroni/quadwild), `cgg-bern/quadwild-bimdf` | GPL-3.0 | portage possible |
| `libQEx` | GPL-3.0 | portage possible |
| CoMISo | GPL (version à vérifier avant tout portage) | portage possible sous réserve de la version |

**Le projet est sous licence GPL-3.0-or-later.** Le but : ne pas être limité dans la
réutilisation et la réécriture de code existant, tout en restant intégrable à Blender.

- **Réutilisation** : tout le code libre de la lignée devient portable, y compris QuadWild et
  libQEx (GPL-3.0), et metriko (la MPL-2.0 autorise la combinaison avec la GPL). CoMISo l'est
  aussi si sa version de GPL est compatible, ce qui reste à vérifier. Porter (traduire du C++
  vers Rust) crée une œuvre dérivée : on conserve les mentions de copyright d'origine et on cite
  la source dans le fichier.
- **Blender** : les binaires de Blender sont distribués en GPL v3 ou ultérieure, et sa plateforme
  d'extensions exige précisément GPL-3.0-or-later pour un add-on. L'intégration dans l'arbre
  source de Blender, dont le code propre est en GPL v2 ou ultérieure, reste une décision de la
  fondation.
- **Ce qui reste interdit** : le code non libre, soit Blossom-V et les solveurs commerciaux
  (Gurobi).
- **Contrepartie** : quiconque intègre cette bibliothèque dans un logiciel doit distribuer ce
  logiciel sous GPL. Elle ne pourra donc pas être embarquée dans un outil propriétaire.
- **Réversibilité** : tant qu'aucun code GPL n'a été porté, on peut encore changer de licence.
  Après le premier portage, les parties dérivées resteront GPL.

### Écarts avec la compilation d'origine

- QuadWild : le titre cité n'existait pas ; titre réel ci-dessus.
- Bi-MDF : attribué à tort à Lyon et Campen ; les auteurs sont Heistermann, Warnett et Bommes.
- « Le secret de ZRemesher » : ZRemesher et Quad Remesher sont propriétaires et non publiés. Toute
  affirmation sur leur fonctionnement est une **hypothèse**, pas une référence.
- « Lissage géodésique » (prétraitement) : terme non rattaché à une méthode ; retiré.

---

## 2. Spécifications fonctionnelles

**Entrée.** Maillage triangulaire dense non structuré, 10⁵ à 10⁷ triangles, OBJ ou PLY.

**Entrées acceptées.** Le cœur (du champ de croix à l'extraction) exige une surface orientée où
chaque arête a au plus deux faces, et le vérifie. La tolérance aux entrées sales est l'affaire du
prétraitement, qui est interchangeable :

| Situation | Traitement |
|---|---|
| Bords ouverts | acceptés |
| Plusieurs pièces séparées | acceptées, chaque pièce est traitée seule |
| Pièces qui se traversent | aujourd'hui traitées coque par coque ; la fusion en une peau extérieure est un prétraitement de réparation à venir (J11) |
| Arêtes à trois faces ou plus, orientation incohérente | refusées avec un diagnostic clair ; réparation à venir (J11) |

**Contraintes utilisateur, par rang.**

1. Budget de quads cible.
2. Densité adaptative automatique : quads plus petits là où la forme est fine ou très courbe.
3. Préservation optionnelle des arêtes vives au-delà d'un angle.
4. Courbes guides tracées sur la surface.
5. Masque de densité locale peint sur les sommets.

Le rang 1 conditionne l'acceptation. La densité adaptative (rang 2) est **prévue par les
interfaces dès maintenant** (un champ de taille accompagne le maillage de calcul, uniforme pour
commencer) et **livrée juste après le premier résultat complet**, avant les rangs 3 à 5 : sur
l'ogre, c'est elle qui décide si les mains existent encore.

**Deux usages, deux chaînes.** L'organique (sculptures, personnages) est servi par la première
chaîne. Le durci (pièces mécaniques, surfaces dures) le sera par la seconde. L'utilisateur choisit
la chaîne ; le logiciel ne suppose jamais qu'il n'en existe qu'une.

**Sortie.** Maillage 100 % quads, variété, fermé ou à bords nets, avec **le moins possible de
sommets irréguliers**. (« 100 % de quads réguliers » est impossible hors du tore : la topologie
impose des sommets irréguliers.)

**Sorties complémentaires, en fin de projet.** Avec le maillage :

- les **coutures** (seams), c'est-à-dire les arêtes le long desquelles la surface est découpée
  pour être dépliée ;
- les **coordonnées UV** par coin de face.

- les **plis** (creases) : un poids par arête, calculé automatiquement, pour qu'une subdivision
  du maillage garde nettes les arêtes vives de la source ;
- des **normales optimales** : les normales par coin qui reproduisent au mieux l'ombrage de la
  source sur le maillage allégé ;
- la **reconstitution Multires** : des niveaux de subdivision dont le déplacement est reprojeté
  depuis la sculpture d'origine, pour retrouver tout son détail au-dessus du maillage quad.

La chaîne organique calcule déjà une paramétrisation et un graphe de coupe pour construire le
maillage ; les UV et les coutures en découleront. Pour la chaîne durci, les bords de patchs
jouent le même rôle.

**Forme finale du logiciel.** Une bibliothèque à interface C, chargée depuis un greffon Python
(module `ctypes`), pour s'intégrer à Blender **sans copie de mémoire** : le greffon passe
directement les tableaux de sommets et de faces de Blender, et relit les résultats dans des
tableaux qu'il fournit.

---

## 3. Pipeline

```
 source dense
     │
 M1  prétraitement ─────────────► maillage de calcul + courbures
     │
 M2  champ de croix ────────────► une croix par face, singularités aux sommets
     │
 M3  simplification des singularités
     │
 M4  paramétrisation sans couture ► (u, v) par coin, transitions réelles
     │
 M5  T-mesh ────────────────────► patchs rectangulaires, longueurs réelles
     │
 M6  quantification ────────────► longueurs entières
     │
 M7  carte à grille entière ────► (u, v) par coin, transitions entières
     │
 M8  extraction ────────────────► maillage quad
     │
 M9  relaxation et projection ──► maillage quad final sur la source

 M0  banc de mesure : compare n'importe quel maillage quad à sa source
```

### M0 — Banc de mesure

- **Entrée** : un maillage source et un maillage quad, quel que soit l'outil qui l'a produit.
- **Sortie** : le tableau des métriques de la section 0.
- **Méthode** : Blender sans interface pour QuadriFlow ; l'exécutable `retopo` pour le plancher.
- **Test** : valeurs connues sur primitives (grille parfaite sur un tore : S = 0, F idéale).
- **Pourquoi en premier** : sans lui, « supérieur à Blender » n'est pas vérifiable.

### M1 — Prétraitement

- **Entrée** : triangles bruts.
- **Sortie** : un maillage de calcul variété, à triangles bien conformés ; par sommet, courbures
  k₁, k₂, directions principales et un indice de confiance.
- **Méthode** : nettoyage (soudure, triangles dégénérés) ; décimation QEM si l'entrée dépasse le
  budget ; remaillage isotrope ; courbures par la méthode de Rusinkiewicz. La décimation seule ne
  suffit pas : elle laisse des triangles effilés qui dégradent les solveurs des modules suivants.
- **Taille du maillage de calcul** : de 50 000 à 200 000 triangles, et au moins plusieurs
  triangles par quad demandé. *Le facteur exact est à mesurer.*
- **Point dur** : l'échelle à laquelle mesurer la courbure sur une sculpture bruitée.
- **Test** : angles minimaux des triangles ; distance à la source ; courbures comparées aux
  valeurs exactes sur sphère, cylindre et tore.

**Réalisé au jalon 1** (`crates/retopo/src/preprocess/`, interface `Preprocessor` et sa fabrique).
Mesuré sur le corpus pour un maillage de calcul de 50 000 triangles demandés :

| Modèle | Triangles | Plus petit angle, moyenne | … 1ᵉʳ centile | Hausdorff, ‰ | Temps |
|---|---|---|---|---|---|
| blob | 124 004 → 50 420 | 45,6° → 52,9° | 35,2° → 41,6° | 2,0 | 1,4 s |
| rocher | 81 920 → 56 638 | 55,8° → 54,8° | 48,0° → 42,7° | 1,3 | 1,1 s |
| créature | 78 124 → 51 162 | 43,3° → 53,3° | 32,3° → 42,0° | 0,6 | 1,1 s |
| cage | 26 656 → 48 600 | 40,8° → 52,8° | 28,5° → 41,5° | 4,4 | 0,9 s |
| Suzanne | 15 744 → 49 284 | 35,4° → 52,2° | 11,8° → 39,2° | 3,2 | 1,1 s |
| pièce (arêtes vives) | 406 → 53 090 | 3,3° → 49,7° | 0,5° → 29,2° | 0,3 | 1,5 s |
| cube (arêtes vives) | 3 072 → 50 592 | 45,0° → 46,7° | 45,0° → 39,0° | 0,0 | 0,8 s |

- Topologie conservée partout : même nombre de composantes, mêmes bords, aucune arête non variété.
- Arêtes vives et coins conservés exactement (volume du cube exact, les 8 coins présents).
- Passage à l'échelle : 1,3 million de triangles ramenés à 100 000 en 10 secondes.
- Courbures vérifiées sur le tore remaillé : valeur et direction justes sur plus de 97 % des
  sommets ; indice de confiance proche de zéro sur la sphère, comme attendu.

Limites constatées :

- **Arêtes vives non déclarées.** Sans angle de pli fourni, une arête vive est traitée comme une
  zone très courbe et s'érode d'environ une demi-longueur d'arête (c'est la cage, à 4,4 ‰).
- **Taille du maillage de calcul** tenue à 13 % près, pas exactement.
- **Entrées non variété** : laissées telles quelles, ni réparées ni refusées.
- **Cylindre** : pas de test sur cylindre, faute de primitive ; le tore couvre le même cas.

### M2 — Champ de croix

- **Entrée** : maillage de calcul, courbures et confiance ; contraintes dures éventuelles (arêtes
  vives, courbes guides).
- **Sortie** : une croix par face, les correspondances entre faces voisines, et l'indice de
  chaque sommet (singularités).
- **Méthode** : représentation complexe d'ordre 4 (z = e^{4iθ}), qui masque la symétrie par quart
  de tour ; énergie de lissage quadratique tenant compte du transport parallèle entre faces.
  - sans contrainte ni alignement : plus petit vecteur propre d'une matrice creuse ;
  - avec alignement souple sur la courbure, ou contraintes dures : système linéaire creux.
  L'alignement est pondéré par l'anisotropie |k₁−k₂| et la confiance : là où k₁ ≈ k₂, la direction
  de courbure n'a pas de sens et ne doit rien imposer.
- **Point dur** : le bruit de courbure crée des singularités parasites ; réglage du poids
  d'alignement.
- **Test** : somme des indices égale à 4·χ ; 8 singularités sur une sphère, 0 sur un tore ; écart
  angulaire à la direction de courbure sur un ellipsoïde ; déterminisme.
- **Décidé : champ par face.** L'article de Knöppel et al. travaille par sommet ; on adapte sa
  formulation aux faces, parce que la paramétrisation (M4) attend un champ par face avec les
  singularités aux sommets.

**Réalisé au jalon 2** (`crates/retopo/src/crossfield/`, interface `CrossFieldSolver` et sa
fabrique ; deux solveurs : `power`, celui de cette fiche, et `extrinsic`, celui d'Instant Meshes
lu par face, gardé comme point de comparaison).

Mesuré sur un maillage de calcul de 50 000 triangles, courbure lue à l'échelle d'un maillage de
3000 quads, poids d'alignement 0,05. Chaque case donne le nombre de singularités puis l'écart
d'alignement en degrés :

| Modèle | Notre champ | Quad Remesher (uniforme) | QuadriFlow |
|---|---|---|---|
| blob | 28 / 1,7 | 24 / 4,6 | 38 / 17,5 |
| créature | 38 / 3,4 | 46 / 5,3 | 24 / 12,9 |
| cage | 48 / 2,3 | 43 / 2,6 | 66 / 8,1 |
| rocher | 126 / 10,0 | 101 / 14,7 | 8 / 23,0 |
| ogre | 203 / 8,0 | 228 / 10,8 | 106 / 19,3 |
| Spot | 74 / 4,7 | 87 / 7,6 | 68 / 18,1 |
| Bob | 20 / 1,9 | 30 / 5,1 | 32 / 19,4 |
| Blub | 44 / 4,9 | 46 / 7,0 | 52 / 16,0 |
| cochon | 90 / 5,9 | 96 / 9,4 | 122 / 17,4 |

**Attention à ne pas surlire ce tableau.** Notre colonne mesure un *champ* ; les deux autres
mesurent un *maillage fini*, dont les arêtes s'écartent toujours un peu du champ qui l'a guidé
(distorsion de la paramétrisation, arrondi aux entiers). Le maillage qu'on tirera de ce champ
fera donc moins bien que ces chiffres. Ce que le tableau établit : le champ a autant de
singularités que Quad Remesher, pas davantage, et il suit la courbure de près. La matière
première est au niveau.

- **Vitesse** : 0,2 à 0,5 seconde pour 50 000 triangles.
- **Vérifié par les tests** : somme des indices égale à 4 χ pour les deux solveurs ; 8
  singularités sur la sphère ; sur le cube, exactement une par coin et toutes les croix alignées
  sur les axes ; champ parallèle aux bords sur une surface ouverte ; alignement sur l'ellipsoïde ;
  déterminisme ; refus d'un maillage non variété.
- **Solveur** : gradient conjugué appliqué directement depuis le maillage (chaque face a au plus
  trois voisines), sans bibliothèque d'algèbre creuse. `faer` reste prévu pour la
  paramétrisation (M4), dont le système est d'une autre nature.

Constats utiles pour la suite :

- **Le champ le plus lisse n'est pas celui qui a le moins de singularités.** Sur un tore épais,
  le champ le plus lisse en porte 16 (8 de chaque signe), parce qu'elles absorbent la courbure à
  moindre coût ; dès qu'on suit la courbure, elles disparaissent. C'est un levier pour M3.
- **Le poids d'alignement règle le compromis** entre nombre de singularités et alignement. De 0
  à 0,5, la créature passe de 18 singularités à 18° d'écart, à 44 singularités à 1,6°. La valeur
  0,05 est un bon milieu sur tout le corpus.
- **Singularités d'ordre supérieur** : sur l'ogre, 6 sommets ont un indice d'un demi-tour ou plus.
  Rares, mais M3 devra les scinder ou les accepter.

### M3 — Simplification des singularités

- **Entrée** : champ et singularités de M2.
- **Sortie** : un champ avec moins de singularités, mieux placées, vérifiant toujours 4·χ.
- **État honnête** : c'est le module le moins établi. Il n'existe pas d'algorithme de référence
  unique ; ce qui suit est une proposition à valider par l'expérience.
- **Méthode proposée**, en deux niveaux :
  1. *Réglage global* : plus le lissage domine l'alignement, moins il y a de singularités.
  2. *Annulation de paires* : pour une singularité +¼ et une −¼ proches, transférer l'indice de
     l'une à l'autre le long du plus court chemin d'arêtes (en modifiant les sauts de période le
     long du chemin), relisser, et ne garder le changement que si l'énergie de lissage n'augmente
     pas au-delà d'un seuil.
- **Alternative plus tard dans la chaîne** : Lyon et al. 2021 déplacent ou suppriment des
  singularités pendant la quantification.
- **Valences** : les indices ±¼ (valences 3 et 5) sont le cas général visé. Les autres ne sont
  pas interdits : coins concaves et bords en demandent.
- **Critère de sortie** : réduction mesurable de S et C sur le corpus sans dégrader A. Si ce
  n'est pas atteint dans le temps imparti, on garde le seul niveau 1 et on avance.

**Réalisé au jalon 3** (`crates/retopo/src/singularity/`, interface `SingularityOptimizer` et sa
fabrique ; deux implémentations : `pair-cancellation`, et `keep` qui ne touche à rien).

La méthode retenue est le niveau 2 de la fiche, avec deux précisions venues de l'expérience :

- **Le champ n'est relissé qu'autour de la paire annulée.** Relisser tout le champ à chaque
  essai le dégradait partout (jusqu'à 1° d'alignement perdu sans rien annuler) et coûtait
  plusieurs secondes. En local, le reste du champ reste exactement celui de M2.
- **Le réglage principal est une distance**, pas un seuil d'énergie : on annule les paires plus
  proches que quelques arêtes de quad. Une paire plus serrée qu'un ou deux quads est du bruit que
  le maillage ne pourrait de toute façon pas exprimer ; une paire éloignée décrit en général une
  vraie forme.

Les singularités d'ordre supérieur sont d'abord scindées en singularités simples.

Mesuré comme au jalon 2 (courbure lue à l'échelle de 3000 quads), paires annulées jusqu'à 2 arêtes
de quad. Chaque case : singularités / écart d'alignement en degrés.

| Modèle | Champ de M2 | Après M3 | Réglage global seul | Quad Remesher (uniforme) |
|---|---|---|---|---|
| ogre | 203 / 8,0 | **98 / 9,0** | 185 / 9,2 | 228 / 10,8 |
| rocher | 126 / 10,0 | **74 / 10,9** | 90 / 11,4 | 101 / 14,7 |
| cochon | 90 / 5,9 | **74 / 6,2** | 90 / 7,1 | 96 / 9,4 |
| Spot | 74 / 4,7 | 70 / 4,9 | 74 / 6,3 | 87 / 7,6 |
| blob | 28 / 1,7 | 26 / 1,8 | 28 / 1,9 | 24 / 4,6 |
| créature, cage, Bob, Blub | inchangés | inchangés | — | — |

« Réglage global seul » est le niveau 1 de la fiche : le même champ calculé avec un poids
d'alignement plus faible (0,02 au lieu de 0,05).

- **Critère de sortie atteint.** Sur les modèles chargés, M3 retire 18 à 52 % des singularités
  pour 0,3° à 1° d'alignement, et fait mieux que le réglage global sur les deux tableaux à la fois
  (ogre : moitié moins de singularités pour le même alignement).
- **Les modèles propres ne sont pas touchés** : là où il n'y a rien à retirer, le champ ressort
  identique, bit pour bit.
- **Vitesse** : 0,1 à 0,2 seconde.
- **Vérifié par les tests** : un tore épais perd ses 16 singularités inutiles ; une sphère garde
  les 8 que sa topologie impose ; la somme des indices est conservée ; résultat déterministe.

Ce qui reste ouvert :

- **L'effet sur la structure (C) n'est pas mesuré** : il faut un maillage pour cela, donc le
  jalon 4 bis.
- **Mêmes réserves qu'au jalon 2** : ces chiffres décrivent un champ, pas un maillage fini.
- **La portée est un réglage** : à 3 arêtes de quad l'ogre tombe à 58 singularités pour 9,7°. La
  bonne valeur se décidera sur des maillages, pas sur des champs.
- **L'ordre est glouton**, les paires les plus proches d'abord ; aucun déplacement de singularité
  isolée n'est tenté.

### M4 — Paramétrisation sans couture

- **Entrée** : maillage de calcul, champ de M3, taille cible h = √(aire / budget).
- **Sortie** : des coordonnées (u, v) par coin de triangle, dont le gradient suit la croix,
  dont les transitions à travers les coutures sont un quart de tour plus une translation (réelle à
  ce stade), **sans triangle inversé**.
- **Méthode** : graphe de coupe passant par les singularités ; mise en cohérence du champ sur le
  maillage découpé ; minimisation quadratique de l'écart entre le gradient de (u, v) et la croix,
  sous contraintes linéaires de transition. C'est un système linéaire creux.
- **Point dur, le premier du projet** : les inversions locales. La minimisation ne les exclut
  pas, et une seule suffit à invalider la suite.
  - ligne de défense 1 : détection, puis repondération locale et nouvelle résolution ;
  - ligne de défense 2 : contraintes de validité d'*Integer-Grid Maps* ;
  - repli : construction à garantie de Campen et al. 2019, nettement plus lourde.
- **Test** : zéro triangle inversé sur tout le corpus ; transitions exactes aux coutures ;
  distorsion mesurée ; sur un tore, la paramétrisation est la grille attendue.

### M5 — T-mesh

- **Entrée** : paramétrisation sans couture de M4.
- **Sortie** : une partition de la surface en patchs rectangulaires dans le domaine paramétrique,
  avec la longueur réelle de chaque arc.
- **Méthode** : « motorcycle graph ». De chaque singularité partent des traces le long des
  isolignes ; une trace s'arrête quand elle en rencontre une autre.
- **Point dur** : la robustesse numérique du traçage (traces passant par un sommet, intersections
  quasi simultanées).
- **Test** : chaque patch est un rectangle ; la partition couvre toute la surface ; cohérence
  avec la formule d'Euler.
- **À trancher** : l'arithmétique du traçage (assainissement de la paramétrisation à la manière
  de QEx, prédicats exacts, ou rationnels).

### M6 — Quantification

- **Entrée** : T-mesh et longueurs réelles.
- **Sortie** : une longueur entière par arc, telle que les côtés opposés de chaque patch aient la
  même somme, que la validité soit préservée, et que le total de quads approche le budget.
- **Méthode** : Bi-MDF. Solveur approché d'abord, raffinement exact ensuite. Budget de quads
  selon Marinov et al. 2026.
- **Points durs** : le raffinement exact repose sur un couplage pondéré dans un graphe général,
  à écrire en Rust. Bi-MDF a été évalué par ses auteurs sur QuadWild ; son application à notre
  T-mesh est à confirmer.
- **Test** : contraintes satisfaites exactement ; optimum retrouvé sur de petits T-mesh
  construits à la main ; écart au budget mesuré (métrique N).

### M7 — Carte à grille entière

- **Entrée** : maillage de calcul, champ, T-mesh quantifié.
- **Sortie** : (u, v) par coin, avec translations entières aux coutures, singularités sur des
  points entiers, sans triangle inversé.
- **Méthode** : même problème que M4, avec en plus une contrainte linéaire par arc du T-mesh
  imposant sa longueur entière.
- **Point dur** : le même que M4.
- **Test** : transitions entières exactes ; singularités sur la grille ; zéro inversion.

### M8 — Extraction

- **Entrée** : carte à grille entière.
- **Sortie** : le maillage quad dont les sommets sont les points entiers et les arêtes les
  segments d'isolignes entières.
- **Méthode** : principe de QEx. Son code de référence (libQEx, GPL-3.0) peut être porté, ce
  qui réduit le risque de ce module.
- **Point dur** : cas dégénérés et imprécisions numériques.
- **Test** : 100 % quads, variété ; nombre de faces égal à celui que prédit la quantification ;
  valences conformes aux indices du champ.

### M9 — Relaxation et projection

- **Entrée** : maillage quad, source dense d'origine.
- **Sortie** : positions finales sur la source.
- **Méthode** : lissage tangentiel et projection au point le plus proche, lignes caractéristiques
  et coins préservés. Déjà en place et réutilisé tel quel. Le lancer de rayon le long de la
  normale est écarté : il rate la surface ou touche la face arrière. Une optimisation de forme
  plus poussée (type ARAP) est envisageable ensuite.
- **Test** : métriques H et F ; aucune face inversée.

---

## 4. Jalons

Chaque jalon produit quelque chose de vérifiable et se termine par **un arrêt pour revue**.

| Jalon | Contenu | Ce qu'on peut vérifier à la fin |
|---|---|---|
| J0 | Banc de mesure, corpus, fabrique de chaînes | **Fait.** Chiffres de QuadriFlow, de Quad Remesher et du plancher ; seuils adoptés |
| J1 | Prétraitement (M1) | **Fait.** Maillages de calcul et courbures, tests analytiques |
| J2 | Champ de croix (M2) | **Fait.** Champs visualisés, singularités comptées |
| J3 | Singularités (M3), durée bornée | **Fait, en attente de revue.** Gain mesuré sur S : critère de sortie atteint |
| J4 | Paramétrisation sans couture (M4) | **Jalon de vérité** : zéro inversion sur le corpus, ou activation d'un repli |
| J4 bis | Arrondi glouton des translations (façon MIQ) et extraction | **Premier résultat complet, provisoire**, comparé à QuadriFlow et à Quad Remesher |
| J5 | T-mesh (M5) | Partitions en rectangles visualisées |
| J6 | Quantification (M6) | Contraintes satisfaites, budget tenu |
| J7 | Carte entière et extraction (M7, M8) | **Résultat complet définitif**, l'arrondi glouton est retiré |
| J8 | Relaxation ; densité adaptative, puis contraintes de rang 3 à 5 | Règle d'acceptation évaluée |
| J9 | Chaîne durci : traçage, patchs, remplissage par motifs | Pièces mécaniques comparées à QuadriFlow |
| J10 | Export des coutures et des UV ; bibliothèque à interface C et greffon Python sans copie mémoire | Remaillage lancé depuis Blender, UV et coutures relus dans la scène |
| J11 | Prétraitement de réparation : fusion des pièces qui se traversent, arêtes non variété | Desert Rose remaillée en une seule peau valide |
| J12 | Plis automatiques : un poids de pli par arête de sortie | Après subdivision, le maillage plissé est plus proche de la source que le même sans plis |
| J13 | Normales optimales par coin | Écart angulaire entre l'ombrage du maillage allégé et celui de la source |
| J14 | Reconstitution Multires : niveaux de subdivision avec déplacement reprojeté | À chaque niveau, l'écart à la sculpture d'origine diminue jusqu'à la retrouver |

Le jalon 4 bis est un raccourci assumé : un arrondi glouton des translations, comme dans MIQ,
donne un premier maillage complet bien avant le T-mesh et la quantification. Il est moins robuste
et sera remplacé au jalon 7 ; son intérêt est de savoir tôt où on en est.

---

## 5. Architecture et stack technique

### Architecture modulaire

Exigence : accueillir plusieurs chaînes (organique, durci, et le plancher actuel) sans réécrire
ce qui leur est commun.

**Une chaîne est un `Remesher`**, obtenu d'une fabrique à partir de son nom
(`remesher::create(Method)`). L'exécutable (`--method`) et le banc de mesure ne passent que par
elle : ajouter une chaîne revient à ajouter une variante de `Method` et une ligne dans le banc.
En place : `instant-meshes`. À venir : la chaîne organique et la chaîne durci.

**Une étape est une interface (trait)**, avec sa propre fabrique, pour que les chaînes se
composent des mêmes briques :

| Étape | Rôle | Chaîne organique | Chaîne durci |
|---|---|---|---|
| `Preprocessor` | maillage de calcul et courbures (M1) | commune | commune |
| `CrossFieldSolver` | champ de croix (M2) | alignement souple sur la courbure | contraintes dures sur les arêtes vives |
| `SingularityOptimizer` | simplification (M3) | commune | commune |
| `LayoutBuilder` | structure globale à longueurs réelles | paramétrisation et T-mesh (M4, M5) | traçage et patchs |
| `Quantizer` | longueurs entières (M6) | commune | commune |
| `Mesher` | maillage quad depuis la structure quantifiée | carte entière et extraction (M7, M8) | remplissage par motifs |
| `Relaxer` | placement final (M9) | commune | commune |

Écrites à ce jour : `Remesher`, `Preprocessor`, `CrossFieldSolver` et `SingularityOptimizer`.

Les deux chaînes ne diffèrent donc que par deux étapes, `LayoutBuilder` et `Mesher`, plus le
réglage du champ de croix. Les types échangés entre étapes (maillage de calcul, champ de croix,
structure à longueurs réelles puis entières, maillage quad) sont ceux des fiches de la section 3.

Règle de conduite : une interface est écrite avec sa première implémentation, pas avant, pour
qu'elle soit dictée par un besoin réel. Son contrat, lui, est déjà fixé par les fiches.

### Stack

- **Langage** : Rust, dans l'espace de travail existant.
- **Maillage** : tableaux plats et table des demi-arêtes opposées (`Topology`), à étendre avec
  des données par coin pour (u, v).
- **Algèbre linéaire creuse** : `faer`, en Rust pur, qui fournit les factorisations creuses de
  Cholesky (LLT, LDLT). *À valider en performance sur 200 000 inconnues.* Le problème aux valeurs
  propres de M2 se ramène à des résolutions répétées avec la même factorisation.
- **Requêtes spatiales** : le BVH existant.
- **Blender** : utilisé seulement par le banc de mesure, jamais comme dépendance.

### En vue de la bibliothèque C (J10)

Le passage en bibliothèque se fait à la fin, mais il coûtera peu si le cœur respecte dès
maintenant quelques règles :

- **Données en tableaux plats et contigus** (positions, indices), comme aujourd'hui : ce sont
  exactement les tampons que `ctypes` et Blender savent partager.
- **Entrées empruntées, jamais possédées** : les étapes lisent des tranches et n'exigent pas de
  prendre possession des données de l'appelant.
- **Pas d'état global** ; tout passe par les paramètres, pour qu'une session Blender puisse lancer
  plusieurs remaillages.
- **Erreurs rendues en valeurs**, jamais par arrêt brutal : une erreur ne doit pas faire tomber
  Blender.
- **Interface C mince**, dans une crate à part, posée par-dessus `Remesher` ; le cœur n'en sait
  rien.

### Ce qui est réutilisé du code existant

| Module existant | Réutilisé pour |
|---|---|
| `crates/retopo/src/mesh/` | conteneurs, connectivité, OBJ, subdivision, primitives de test |
| `crates/retopo/src/geometry.rs` | normales, aires, détection des arêtes vives |
| `crates/retopo/src/spatial/bvh.rs` | projection sur la source (M1, M9) |
| `crates/retopo/src/relax.rs` | M9 |
| `crates/retopo/src/field/orientation.rs` | détection des singularités, à adapter au champ par face |
| `crates/retopo/tests/pipeline.rs` | invariants : Poincaré-Hopf, Euler, variété, déterminisme |

Conservés uniquement comme plancher de comparaison, hors de la chaîne cible :
`field/position.rs`, `extract/`, `hierarchy.rs`, `pipeline.rs`.

---

## 6. Risques

| Risque | Conséquence | Repli |
|---|---|---|
| Inversions locales irréductibles (M4, M7) | pas de maillage valide | construction à garantie (Campen et al. 2019) ; en dernier recours, lignée par patchs à partir du champ de M3, puisque M1 à M3 et M6 sont communs aux deux, en portant QuadWild |
| Traçage non robuste (M5, M8) | T-mesh ou maillage incohérent | portage de libQEx pour l'extraction ; arithmétique exacte pour le T-mesh |
| Solveur de couplage trop lourd à écrire (M6) | pas de quantification optimale | solveur approché seul ; arrondi glouton façon MIQ |
| Pas d'algorithme établi pour M3 | singularités en excès | réglage global seul ; simplification pendant la quantification (Lyon et al. 2021) |
| Densité adaptative | distorsion, singularités supplémentaires | reportée au rang 4, hors critère d'acceptation |
| Ne pas battre QuadriFlow | objectif non atteint | mesuré dès J7, pas découvert à la fin |

Deux points à ne pas se cacher. QuadriFlow est rapide et robuste (validé par ses auteurs sur
17 791 objets, moins de dix secondes par modèle) : la cible est de le battre en **qualité**, pas
d'emblée en vitesse ni en robustesse. Et le volume de travail se compte en mois ; je n'ai pas de
base fiable pour le chiffrer plus finement.

---

## 7. Décisions

En attente :

1. **Corpus** : d'autres sculptures à ajouter plus tard, avant l'expiration de la licence d'essai
   de Quad Remesher.
2. **Arithmétique du traçage** (M5) ; peut attendre J4.

Prises :

- objectif : mieux que le remaillage quad de Blender ; Quad Remesher est un palier, pas un plafond ;
- priorité : organique d'abord, durci ensuite ;
- lignées : paramétrisation et isolignes pour l'organique, patchs pour le durci ;
- architecture modulaire à fabriques ;
- licence GPL-3.0-or-later ;
- seuils d'acceptation de la section 0 ;
- champ de croix par face ;
- densité adaptative prévue d'emblée, livrée juste après le premier résultat complet ;
- raccourci MIQ (jalon 4 bis) pour un premier résultat complet plus tôt ;
- export des coutures et des UV, et forme finale en bibliothèque C pilotée par un greffon Python
  sans copie mémoire ;
- sources en anglais, documents traduits à la fin ;
- cœur strict sur les entrées (surface orientée, deux faces par arête au plus), tolérance logée
  dans le prétraitement ; Desert Rose hors acceptation ;
- feuille de route étendue : plis automatiques, normales optimales, reconstitution Multires
  (J12 à J14).
