# openclare — cahier des charges du moteur de retopologie quad

**Statut : validé le 6 octobre 2026. Jalons 0 à 7 faits ; jalon 8 (relaxation, puis densité adaptative et densité peinte) réalisé, en attente de revue.**
Ce qui a été écarté, approché ou reporté en chemin est suivi ticket par ticket dans [BACKLOG.md](BACKLOG.md).
Le projet s'appelle **openclare** ; les crates portent encore le nom provisoire `retopo`. Les points marqués
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
6. Symétrie par rapport à un plan (ajoutée le 7 octobre 2026 ; jalon 9 bis).

Le rang 1 conditionne l'acceptation. La densité adaptative (rang 2) est **prévue par les
interfaces dès maintenant** (un champ de taille accompagne le maillage de calcul, uniforme pour
commencer) et **livrée juste après le premier résultat complet**, avant les rangs 3 à 5 : sur
l'ogre, c'est elle qui décide si les mains existent encore.

État au jalon 8 : les rangs 1, 2 et 5 sont livrés. Le rang 2 et le rang 5 passent par le même
champ de taille (fiche M1 bis) ; la densité peinte est remontée devant les rangs 3 et 4 parce
qu'elle ne demandait rien de plus. Le rang 3 existe dans la chaîne organique, sans garantie
(DT-002) ; le rang 4 n'est pas commencé (DT-024).

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
 M7  maillage par rectangle ────► connectivité de M6, sommets lus dans la carte de M4
     │                            (route directe ; M8, l'extraction, n'a plus lieu d'être)
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

### M1 bis — Champ de taille

- **Entrée** : maillage de calcul ; réglage d'adaptivité (de 0 à 1) ; en option, une densité
  peinte par sommet du maillage d'entrée.
- **Sortie** : par sommet du maillage de calcul, un facteur sur la longueur d'arête demandée
  (moins de 1 : quads plus petits). Il accompagne le maillage de calcul et vaut 1 partout tant
  qu'on ne demande rien.
- **Méthode** : deux sources qui se multiplient.
  - *La forme* (rang 2). Une corde de longueur l sur une surface de courbure k s'en écarte de
    k l² / 8 : pour une erreur égale partout, l varie comme 1/√k. L'adaptivité est la part de
    cette loi qu'on applique (0 : taille unique ; 1 : erreur égalisée), dans la limite d'un
    facteur 3 de part et d'autre.
  - *La densité peinte* (rang 5). Un poids par sommet de l'entrée, de 0 (le moins dense) à 1
    (le plus dense). Un seul réglage, le contraste, dit combien de fois plus de quads par unité
    d'aire reçoit un poids de 1 qu'un poids de 0 (4 par défaut : arêtes deux fois plus courtes).
    Seul l'écart entre les poids compte, puisque le nombre total de quads est fixé par
    ailleurs : un sommet non peint vaut 0 et sert simplement de niveau de base. Convention
    retenue le 7 octobre 2026 pour coller au Weight Paint de Blender. (Quad Remesher, lui, peint
    en couleurs de sommets un facteur de ×0,25 à ×4 autour d'une valeur neutre : vérifié dans le
    module installé, version 1.4.) Chaque sommet du maillage de calcul lit la peinture au point
    le plus proche de l'entrée.

  Le champ est ensuite lissé sur une arête et demie de quad, puis mis à l'échelle pour que le
  nombre total de quads reste celui demandé.
- **Comment il agit sur la chaîne** : à deux endroits, et il faut les deux.
  - *La carte* (M4) avance plus vite là où les quads doivent être plus petits : l'unité de la
    carte est la longueur d'arête locale. Tout l'aval (T-mesh, quantification, placement) vit
    dans la carte et n'a pas à connaître la taille.
  - *Le champ de croix* (M2) est calculé comme si les longueurs de la surface étaient divisées
    par la taille locale. Sans cela la carte ne peut suivre la taille qu'à moitié : un maillage
    quad ne change de densité qu'en créant des sommets irréguliers, et c'est le champ qui les
    place. En pratique, le transport d'une direction d'une face à sa voisine reçoit un terme de
    plus, proportionnel à la variation de la taille le long de l'arête traversée.
- **Point dur** : plus la taille varie, plus il faut de sommets irréguliers ; et une partie plus
  mince qu'un quad (une oreille) n'est pas vue par la courbure seule.
- **Test** : sans adaptivité ni peinture, facteur 1 partout ; le nombre de quads est conservé ;
  le maillage est mesurablement plus fin là où on l'a demandé.

**Réalisé au jalon 8** (`crates/retopo/src/sizing.rs`, interface `Sizer` et sa fabrique ;
terme de transport dans `crossfield/frames.rs` ; réglage `adaptivity` de la configuration ;
données peintes passées par `Guides`, à côté du maillage, et non dans la configuration).

![L'ogre à 3000 quads : taille unique, densité adaptative, densité peinte](docs/images/jalon8_densite.png)

Sur l'image : taille unique ; densité adaptative à 0,5 (le préréglage `organic`) ; densité
peinte, quatre fois plus de quads sur la tête et quatre fois moins sur les jambes que sur le
tronc (poids 1, 0 et 0,5, contraste 16). Les trois
maillages sont valides et tiennent leur budget à 2 % près.

**Résultat.** Médianes sur les 12 modèles organiques, densité adaptative à 0,5 (tableaux
complets dans `bench/results/ACCEPTANCE.md` ; dans le banc, `quad-layout` est la variante à
taille unique et `quad-layout-adaptive` celle du préréglage) :

| | 1000 quads | 3000 quads | 8000 quads |
|---|---|---|---|
| Modèles toujours valides : taille unique / adaptative / Quad Remesher défaut | 11 / **12** / 11 | 12 / **12** / 8 | 12 / **12** / 11 |
| Alignement (A) : taille unique / adaptative / Quad Remesher défaut | 9,1° / **8,4°** / 6,4° | 8,4° / **7,6°** / 6,4° | 8,7° / **7,8°** / 7,5° |
| Écart d'angle moyen (F) | 5,3° / **5,4°** / 8,2° | 5,4° / **5,0°** / 6,5° | 6,1° / **5,6°** / 6,4° |
| Hausdorff, ‰ (H) | 18,5 / **14,1** / 7,9 | 8,1 / **7,2** / 5,9 | 4,7 / **3,5** / 4,3 |
| Sommets irréguliers (S) | 42 / **42** / 72 | 50 / **54** / 80 | 67 / **78** / 72 |
| Écart au budget (N) | 0,8 % / **0,5 %** / 71 % | 1,4 % / **1,1 %** / 37 % | 0,8 % / **0,8 %** / 13 % |

- La densité adaptative améliore l'alignement et la fidélité à tous les budgets, et rend
  valide le dernier cas qui ne l'était pas (le cochon à 1000 quads).
- **Face à Quad Remesher par défaut** (lui aussi à taille adaptative) : à 8000 quads openclare
  est devant sur la fidélité et la forme, presque à égalité sur l'alignement. À 1000 et 3000
  quads Quad Remesher reste devant sur l'alignement et la fidélité, mais il n'y tient pas le
  budget demandé : il rend 71 % et 37 % de quads d'écart, là où openclare reste à 1 %.
- **La règle d'acceptation face à QuadriFlow** reste tenue en médiane avec la densité
  adaptative, à une exception : à 8000 quads, 78 sommets irréguliers contre 70 pour QuadriFlow.
  La variante à taille unique la tient aux trois budgets.

Vérifié par les tests (`crates/retopo/tests/sizing.rs`) : facteur 1 partout par défaut ;
sur un fuseau, pointes plus fines que la taille et d'autant plus que l'adaptivité est forte,
nombre de quads conservé ; un hémisphère peint à 1 et l'autre laissé à 0, avec un contraste de
16, donnent un champ de taille dans un rapport 4, sans saut à l'équateur ; peindre tout au même
poids ne change rien ; avec un contraste de 9, le maillage obtenu a des arêtes trois fois plus
courtes du côté peint, reste valide et tient son budget ; les entrées incohérentes sont
refusées.

Ce qui reste ouvert :

- **Plus de sommets irréguliers** : c'est le prix de la variation de taille (54 en médiane
  contre 50 à 3000 quads ; 88 contre 76 sur l'ogre).
- **Les parties plus minces qu'un quad** restent mal rendues : les oreilles de l'ogre font
  toujours son plus grand écart à la source. La courbure ne suffit pas à les voir (DT-018).
- **La portée de l'annulation des singularités et l'échelle de lecture de la courbure** se
  comptent encore en arêtes nominales, pas locales (DT-025).
- **Format de la densité peinte** : un fichier texte, un poids par sommet. La lecture du
  Weight Paint de Blender viendra avec le pont Blender (J10, DT-026).
- **Pièces mécaniques** : le préréglage `hard-surface` garde une taille unique.

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

**Réalisé au jalon 4** (`crates/retopo/src/parametrization/`, interface `Parametrizer` et sa
fabrique).

- **Graphe de coupe** : arbre couvrant des faces, dont le complément est élagué jusqu'aux
  singularités et aux bords. Le champ est peigné le long de l'arbre.
- **Résolution** : les conditions de couture sont imposées par lagrangien augmenté, ce qui garde
  un système symétrique défini positif. Une seule factorisation de Cholesky sert à toutes les
  itérations, et les coutures finissent exactes (écart de l'ordre de 1e-10).
- **Inversions** : première ligne de défense seulement, le raidissement. Les faces repliées et
  leurs voisines sont alourdies, puis on résout de nouveau.

**Jalon de vérité : passé.** Sur les 16 modèles du corpus, à 1000, 3000 et 8000 quads (champ
simplifié par M3) :

| | Résultat |
|---|---|
| Cas sans aucun triangle inversé | **48 sur 48** |
| Triangles inversés à la première résolution | 0 à 146 selon le modèle |
| Passes de raidissement nécessaires | 0 à 15 |
| Distorsion moyenne | de 0,001 (cube) à 0,48 (ogre à 1000 quads) ; 0 = copie rigide de chaque face |
| Temps, 50 000 triangles | 0,2 à 3,6 secondes |

Les cas les plus durs sont ceux où le quad demandé est gros devant les détails : l'ogre et la
créature à 1000 quads (15 et 14 passes), et le scan brut à 3000 (11 passes).

Vérifié par les tests : un plan est déplié rigidement ; un cube se déplie sans aucune distorsion,
ses 8 coins sur la coupe ; un tore se referme avec la bonne période (la circonférence du tube, à
10 % près) ; la surface coupée est toujours un disque ; le champ est peigné partout hors de la
coupe ; résultat déterministe.

Ce qui reste ouvert, et qu'il ne faut pas se cacher :

- **Ce jalon ne valide que la moitié du risque.** La carte est sans couture, mais ses
  translations sont encore réelles. Les rendre entières (J4 bis, puis J7) ajoute des contraintes,
  donc de la distorsion : des inversions peuvent revenir. Le verdict définitif sur la lignée
  tombera là.
- **Faces très étirées** : certaines ne sont pas inversées mais écrasées (étirement jusqu'à 1000
  près de singularités). Elles sont valides, pas belles.
- **La distorsion ne dépend pas du budget à ce stade** : la taille du quad ne fait que mettre la
  carte à l'échelle. Elle n'entrera en jeu qu'avec les entiers.
- **Bords et arêtes vives** : depuis le jalon 5, ils sont tenus sur des isolignes de la carte
  (une coordonnée constante le long de chacun), sans inversion sur le corpus. Les placer sur des
  lignes *entières* viendra avec la quantification.
- **Lignes de défense 2 et 3** (contraintes d'*Integer-Grid Maps*, construction à garantie) non
  écrites : le raidissement a suffi sur le corpus.

### Jalon 4 bis — premier résultat complet (provisoire)

La première chaîne complète existe : `integer-grid`. Elle enchaîne prétraitement, champ de
croix, simplification des singularités, paramétrisation entière, extraction et relaxation.

- **Carte entière par arrondi glouton** (`parametrization/integer.rs`) : on résout sans entiers,
  on arrondit les quantités les plus proches d'un entier, on résout de nouveau, jusqu'à
  épuisement. Les quantités entières sont les coordonnées des singularités, les translations des
  coutures qu'aucune singularité ne fixe, et la ligne de grille de chaque arête caractéristique.
- **Extraction** (`extract/grid.rs`) : chaque point de la surface appartient au point de grille
  le plus proche ; les triangles sont raffinés, les sommets d'une même cellule fusionnés, et deux
  cellules voisines reliées. C'est l'extraction d'Instant Meshes nourrie d'une grille globalement
  cohérente. Elle n'a jamais à suivre une ligne à travers une couture, et tolère quelques
  triangles inversés ; ce n'est pas l'extraction exacte du module M8.
- **Réparation locale** : les rares faces non quads sont fusionnées par paires ou recoupées.

Mesuré à 3000 quads sur les 12 modèles organiques (Desert Rose hors acceptation) :

| | QuadriFlow | Quad Remesher (uniforme) | **openclare** | Plancher |
|---|---|---|---|---|
| Maillages valides (V) | 36 sur 60 | 10 sur 12 | **9 sur 12** | 47 sur 60 |
| Alignement (A), médiane | 17,5° | 7,7° | **8,8°** | 16,6° |
| Sommets irréguliers (S), médiane | 66 | 68 | **50** | 156 |
| Écart d'angle moyen (F), médiane | 8,6° | 7,2° | **8,0°** | 6,0° |
| Écart au budget (N), médiane | 4 % | 10 % | **11 %** | 2 % |
| Temps, médiane | 1,9 s | 12 s | **6,5 s** | 1,2 s |

Structure (C), sur les modèles où notre maillage est valide : toujours moins de patchs que
QuadriFlow, et le plus souvent moins que Quad Remesher (blob 951 contre 2697 et 1350 ; cage 427
contre 1943 et 1001 ; Bob 714 contre 2903 et 2568). Le tore sort en grille parfaite : aucun
sommet irrégulier, un seul patch.

Face aux seuils d'acceptation :

| Seuil | Verdict |
|---|---|
| A : au moins 2° de mieux que QuadriFlow | **tenu sur les 11 modèles comparables** |
| C : pas plus de patchs que QuadriFlow | **tenu partout où il est mesurable** (8 modèles) |
| H : Hausdorff au plus 1,2 fois celui de QuadriFlow | tenu sur 10 modèles sur 11 |
| S : au plus 1,5 fois QuadriFlow | tenu sur 9 sur 11 (pas sur le rocher ni la créature) |
| F : écart d'angle pas plus élevé | tenu sur 8 sur 11 |
| V : tous les maillages valides | **non tenu** : 9 sur 12 |
| N : écart au budget d'au plus 5 % | **non tenu** : 1 sur 12, il manque 8 à 18 % des faces |

**Verdict sur la lignée.** Les deux jalons de vérité sont passés sur le fond : la chaîne produit
de vrais maillages quad, structurés, mieux alignés que ceux de QuadriFlow sur tous les modèles
et proches de ceux de Quad Remesher. Elle n'est pas encore acceptable : la validité et le budget
ne sont pas tenus, et ce sont précisément les deux points que la quantification (J5, J6) est
faite pour régler.

Ce qui ne va pas, et pourquoi :

- **Trois maillages invalides** (ogre, Blub, cochon) : il y reste 2 à 6 faces non quads, isolées,
  que la réparation locale ne sait pas apparier. Aucun trou, aucune face inversée.
- **La carte garde des triangles inversés après arrondi** (6 à 80 selon le modèle), que le
  raidissement ne résout plus : l'arrondi glouton peut pousser deux singularités sur le même
  point de grille. C'est la limite connue de cette méthode.
- **Impasse de parité, contournée.** Arrondies dans le désordre, les dernières singularités se
  retrouvent forcées sur des demi-entiers. La carte est donc calculée sur une grille deux fois
  plus grossière puis doublée : les choix libres tombent sur des lignes paires. C'est moins fin,
  et c'est une cause du budget non tenu et de quads étirés.
- **Arêtes vives** : sur le cube, les contraintes de lignes de grille ne se concilient pas avec le
  reste ; la chaîne se replie alors sur le champ seul (8 sommets irréguliers, mais des lignes
  décalées des arêtes). La pièce percée, elle, passe.
- **Vitesse** : 2 à 10 secondes, dominées par les résolutions répétées de l'arrondi.

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
- **Arithmétique du traçage** : tranchée au jalon 5, voir ci-dessous.

**Réalisé au jalon 5** (`crates/retopo/src/tmesh/`, interface `LayoutBuilder` et sa fabrique).

![Les coupes du T-mesh sur l'ogre et sur Suzanne, à 3000 quads](docs/images/tmesh.png)

- **Marche dans la carte** (`walk.rs`) : une trace avance triangle par triangle. Dans un
  triangle la carte est linéaire, donc la trace est un segment ; en traversant une couture, sa
  direction tourne de la transition de cette couture ; en passant par un sommet, elle continue
  dans le triangle dont le secteur contient sa direction. Le partage des directions entre les
  secteurs autour d'un sommet est décidé par des tests de signe exacts, un par arête : aucune
  direction n'est comptée deux fois ni oubliée. Une singularité d'indice k émet bien 4 − k traces.
- **Course** (`motorcycle.rs`) : toutes les traces partent en même temps ; à chaque pas, c'est
  la moins avancée qui traverse son triangle. Elle s'arrête sur le premier tronçon déjà posé
  dans ce triangle. C'est une approximation de la course idéale, à un triangle près : le
  résultat peut différer du « motorcycle graph » exact, mais c'est toujours une partition
  valide, et c'est tout ce que demande la suite.
- **Rencontres** : en travers (jonction en T), nez à nez sur la même ligne (les deux traces
  s'arrêtent, nœud de valence 2), ou exactement sur un sommet du maillage (chaque sommet retient
  la première trace qui l'a traversé). Ce dernier cas n'est pas un détail : sur un cube, toutes
  les traces courent le long d'arêtes du maillage.
- **Bords ouverts** : un bord est une coupe présente dès le départ. Les traces s'y arrêtent ;
  là où il tourne vers l'intérieur de la surface, il émet des traces comme une singularité.
  Cela exige des bords posés sur des isolignes : la paramétrisation sans couture (M4) impose
  désormais que chaque arête caractéristique, bord ou arête vive, garde une coordonnée
  constante, sans la forcer à être entière. Si cela replie la carte, elle revient à la version
  sans cette contrainte et le signale.
- **Patchs** : chaque nœud ordonne ses arcs dans le sens trigonométrique ; un patch se lit en
  suivant son contour, patch à gauche. Un angle intérieur d'un quart de tour est un coin, un
  demi-tour est une jonction en T sur un côté. Un patch est accepté s'il a exactement quatre
  coins et rien d'autre.
- **Arithmétique** : nombres flottants ordinaires, avec une seule tolérance de 1e-6 unité de
  grille (la carte est exacte à 1e-10 près, un triangle mesure quelques dixièmes d'unité). En
  dessous, une trace qui frôle un sommet passe par ce sommet, et deux traces sur des lignes
  voisines sont sur la même ligne. Ni prédicats exacts ni rationnels : sur le corpus, ce n'est
  pas nécessaire.

**Résultat.** Sur les 16 modèles du corpus, à 1000, 3000 et 8000 quads (champ simplifié par M3,
arêtes vives à 30° pour les deux pièces mécaniques ; tableau complet dans
`bench/results/TMESH.md`, produit par `bench/tmesh.py`) :

| | Résultat |
|---|---|
| Partitions valides (tous les patchs rectangulaires, aire couverte exacte, Euler) | **48 sur 48** |
| Traces perdues, traces superposées | 0 |
| Triangles inversés dans la carte, bords hors isolignes | 0 |
| Écart d'aire entre les rectangles et la carte | de l'ordre de 1e-10 en valeur relative |
| Patchs, 12 modèles organiques à 3000 quads | 60 à 340, médiane 185 (environ 3 par singularité) |
| Patchs plus étroits qu'un demi-quad, mêmes modèles | 27 % (36 % à 1000 quads, 22 % à 8000) |
| Temps du traçage, 50 000 triangles | moins de 0,15 seconde |

Cas d'école : le cube donne ses 6 faces, 20 nœuds (8 coins, 12 milieux d'arêtes où les traces
se rencontrent nez à nez) et 24 arcs ; le plan ouvert donne un seul rectangle bordé par son
contour ; le tore, sans singularité, n'est pas découpé du tout.

Vérifié par les tests (`crates/retopo/tests/tmesh.rs`) : sur une sphère, un cube à arêtes vives,
un plan ouvert, un tore avec et sans singularités, chaque patch a quatre coins et des côtés
opposés égaux, chaque demi-arc borde exactement un patch, les contours se referment, la somme
des aires des rectangles est l'aire de la carte, et nœuds − arcs + patchs vaut la caractéristique
d'Euler de la surface ; résultat déterministe.

Ce qui reste ouvert :

- **Un quart des patchs sont des lamelles.** Deux singularités presque alignées laissent entre
  leurs traces une bande plus étroite qu'un demi-quad. C'est attendu avec une carte à
  translations réelles, et c'est précisément le travail de la quantification (M6) : donner à ces
  bandes une largeur entière, zéro le plus souvent, ce qui les referme. Le nombre de patchs
  d'aujourd'hui n'est donc pas celui du maillage final.
- **Les arêtes vives ne sont pas des arcs.** Elles sont sur des isolignes de la carte, et sur
  le cube les traces les suivent d'elles-mêmes, mais rien n'oblige en général une arête vive à
  porter un arc, donc rien ne permettra à M6 de lui donner une coordonnée entière. À traiter
  comme les bords (une coupe présente dès le départ), au plus tard avec la chaîne durci (J9).
- **Une pièce sans singularité ni bord** (un tore) n'a pas de T-mesh : la quantification devra
  traiter ses deux périodes à part.
- **La course est approchée** (à un triangle près) et les traces ne sont pas raccourcies : le
  T-mesh n'est pas le plus simple possible. Lyon et al. 2021 montrent qu'on peut faire mieux en
  choisissant quelles traces prolonger ; à regarder si M6 peine.
- **Aucune garantie formelle** de terminaison ou de validité hors du corpus : une trace qui
  dépasse 20 fois le côté de la carte est abandonnée et comptée, la partition est alors déclarée
  non valide. Cela ne s'est jamais produit.

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

**Réalisé au jalon 6** (`crates/retopo/src/quantization/`, interface `Quantizer` et sa fabrique).

- **Le problème** (`flow.rs`) : une bande de quads traverse un rectangle d'un côté au côté
  opposé. Les longueurs des arcs de deux côtés opposés doivent donc avoir la même somme : c'est
  une conservation de flot, sur un réseau dont les arêtes n'ont pas de sens (une bande n'a ni
  début ni fin). C'est exactement le flot bidirigé à déviation minimale de Bi-MDF. Sur un bord
  ouvert, les bandes s'arrêtent librement.
- **Le solveur, approché** : on part de zéro partout (équilibré) et on applique des marches
  fermées qui gardent chaque rectangle équilibré : en traversant un rectangle, deux arcs
  bougent dans le même sens ; en rebroussant chemin sur le même côté, ils bougent en sens
  opposés. Une marche qui fait baisser le coût est appliquée, par pas de 32, 16, … puis 1,
  jusqu'à ce qu'on n'en trouve plus. Les marches sont cherchées comme des cycles négatifs
  (Bellman-Ford). Le coût d'un arc est l'écart au carré à sa longueur réelle, rapporté à cette
  longueur : une arête de plus ou de moins compte sur un arc court, pas sur un long.
- **Ce qui ne doit pas se refermer** (`lattice.rs`) : un arc de longueur zéro fusionne ses deux
  bouts, un rectangle sans largeur replie ses deux grands côtés l'un sur l'autre. Pour savoir
  si le résultat est sain, on **construit la connectivité du maillage quad** que ces longueurs
  représentent, et on la vérifie : variété, même topologie que la surface, et pour seuls sommets
  irréguliers les singularités du champ, chacune avec sa valence (règle assouplie au jalon 7
  pour les bords, voir plus bas). Chaque
  défaut est ramené à ce qui s'est refermé à cet endroit ; l'arc correspondant reçoit une
  longueur minimale de 1, et on résout de nouveau.
- **Le budget** : les longueurs réelles sont multipliées par un facteur commun, ajusté
  (proportion, puis dichotomie) jusqu'à tomber à 2 % du nombre de quads demandé. Un léger
  décalage fixe par arc évite que des arcs égaux basculent tous ensemble : sans lui, un cube
  passe de 22 à 24 arêtes par côté d'un coup, soit 15 % de quads.

**Résultat.** Sur 15 modèles du corpus (le tore n'a pas de T-mesh), à 1000, 3000 et 8000 quads
(tableau complet dans `bench/results/QUANTIZATION.md`, produit par `bench/quantize.py`) :

| | Résultat |
|---|---|
| Quantifications valides (côtés opposés égaux, maillage quad sain) | **45 sur 45** |
| Écart au nombre de quads demandé | médiane 1,3 %, pire cas 3,9 % |
| Arcs de longueur zéro, modèles organiques | 16 à 25 % selon le budget |
| Rectangles refermés (sans aire), modèles organiques | 19 à 31 % : les lamelles du jalon 5 |
| Arcs maintenus ouverts pour garder la structure saine | 29 au total sur les 12 modèles organiques à 3000 quads (39 avant l'assouplissement du jalon 7) |
| Écart moyen à la longueur réelle | 0,3 arête |
| Temps | moins de 0,15 s à 1000 et 3000 quads ; 1 s au pire à 8000 |

Le seuil **N** (budget tenu à 5 % près), manqué au jalon 4 bis (11 %), est tenu ici. Et la
connectivité obtenue est celle d'un maillage tout en quads, variété, sans autre sommet irrégulier
que les singularités : le seuil **V** est tenu sur le papier, il reste à le confirmer avec des
positions (J7).

Vérifié par les tests : sur de petits réseaux, le solveur retrouve l'optimum exact (comparé à
l'énumération complète) dans 58 cas tirés au hasard sur 60, et n'est jamais déséquilibré ; une
borne inférieure est respectée ; les bandes s'arrêtent sur les bords. Sur une sphère, un cube,
un plan ouvert et un tore à seize singularités : côtés opposés égaux, maillage quad sain, bon
nombre de quads. Le plan à 400 quads donne exactement sa grille de 20 par 20. À très bas budget
(60 quads), les lamelles se referment sans fusionner de singularités ni pincer le tube.

Ce qui reste ouvert :

- **Le solveur est approché.** Il juge chaque marche sur ses déplacements pris un à un, et peut
  se tromper quand une marche repasse par le même arc : deux cas sur soixante au test. Le
  raffinement exact de Bi-MDF (couplage) n'est pas écrit ; rien sur le corpus ne le réclame.
- **Les arcs maintenus ouverts** donnent des bandes de quads étroites (un quad de large pour
  une largeur réelle bien moindre). C'est le prix de la fidélité au champ : accepter au
  contraire de fusionner deux singularités opposées proches (Lyon et al. 2021) donnerait un
  maillage plus simple. À voir avec les résultats du jalon 7.
- **Budget** : le pas le plus fin est une bande entière, soit 2 à 4 % des quads à 1000 quads.
- **Une pièce sans singularité ni bord** (un tore) n'a pas de T-mesh, donc rien à quantifier :
  ses deux périodes devront être arrondies à part.
- **Arêtes vives** : toujours pas des arcs (voir M5), donc pas encore sur des lignes entières.

### M7 et M8 — Du T-mesh quantifié au maillage quad

Le cahier des charges prévoyait deux modules : M7, une carte à grille entière contrainte par les
longueurs quantifiées, sans triangle inversé ; M8, l'extraction exacte de ses isolignes (principe
de QEx). **Décision du 7 octobre 2026 : route directe.** Ces deux modules ne sont pas écrits
(ticket DT-001) ; ils sont remplacés par ce qui suit.

- **Entrée** : maillage de calcul, carte sans couture, T-mesh, quantification.
- **Sortie** : le maillage quad, sommets posés sur le maillage de calcul.
- **Méthode** : la connectivité est celle que la quantification a construite et vérifiée
  (M6). Un sommet situé sur un arc est placé le long de la trace dont l'arc est un morceau. Un
  sommet intérieur à un rectangle est placé en partant du bord du rectangle et en marchant dans
  la carte sans couture vers l'intérieur. La grille entière de chaque rectangle est ainsi
  étirée sur son étendue réelle.
- **Point dur** : là où les longueurs entières s'écartent des réelles, les lignes de la grille
  se brisent aux frontières entre rectangles ; la relaxation (M9) l'atténue.
- **Test** : 100 % quads, variété ; nombre de faces égal à celui de la quantification ; sommets
  sur la surface ; valences conformes aux indices du champ.

**Réalisé au jalon 7** (`crates/retopo/src/meshing/`, interface `Mesher` et sa fabrique ; chaîne
`quad-layout` dans `remesher.rs`, devenue celle des préréglages).

![L'ogre à 3000 quads : QuadriFlow, openclare, Quad Remesher](docs/images/jalon7_ogre.png)

- **Lecture de la carte** : le T-mesh garde, pour chaque trace, son chemin dans la carte
  (triangle par triangle) et, pour chaque arc, la portion de trace qu'il couvre. Placer un point
  revient à retrouver un point d'une trace, puis à marcher en ligne droite dans la carte, avec
  la même mécanique que le traçage. Sur le corpus, aucun sommet n'a dû être placé autrement.
- **Sommets fusionnés** : un sommet né de la fusion de plusieurs points (lamelle refermée) est
  mis à leur moyenne, sauf s'il contient une singularité ou un coin de bord, qui ne bouge pas.
- **Relaxation** : deux passes seulement. Le placement brut est déjà valide partout ; la
  relaxation améliore la forme des quads, mais au-delà de quelques passes elle érode les
  détails fins et dégrade la fidélité (ticket DT-019).
- **Deux règles de la quantification assouplies** au vu des premiers maillages, toutes deux
  pour les bords ouverts. Une singularité que les longueurs amènent sur un bord devient un coin
  du bord, au lieu d'être tenue à une fraction d'arête de lui. Deux coins de bord presque
  confondus (une marche d'escalier minuscule dans la carte) peuvent fusionner. Sans cela, des
  rangées de quads écrasés longeaient les bords. La décision de garder séparées les
  *singularités* entre elles est inchangée.
- **Repli** : une surface dont une pièce n'a ni singularité ni bord (un tore) n'a pas de T-mesh
  complet ; la chaîne passe alors la main à `integer-grid` et le signale (ticket DT-003).

**Résultat.** Sur les 12 modèles organiques, à 3000 quads, médianes (tableaux complets dans
`bench/results/RESULTS.md`) :

| | QuadriFlow | openclare, J4 bis | **openclare, J7** | Quad Remesher, taille uniforme |
|---|---|---|---|---|
| Modèles toujours valides (V) | 5 sur 12 | 9 sur 12 | **12 sur 12** | 10 sur 12 |
| Alignement (A) | 17,5° | 8,8° | **8,6°** | 7,8° |
| Sommets irréguliers (S) | 66 | 50 | **50** | 67,5 |
| Patchs du complexe de base (C) | 2697 | 951 | **1886** | 2009 |
| Écart d'angle moyen (F) | 8,6° | 8,0° | **7,8°** | 7,2° |
| Hausdorff, ‰ de la diagonale (H) | 17,7 | 9,3 | **9,1** | 9,6 |
| Écart au budget (N) | 4,1 % | 11,1 % | **1,4 %** | 10,3 % |
| Temps (T) | 1,9 s | 6,5 s | **2,2 s** | 12,3 s |

**La règle d'acceptation est tenue à 3000 quads** : validité jamais moins bonne que celle de
QuadriFlow, médianes meilleures sur S, C, A et F, sans dégrader H ni N. Les deux seuils que le
jalon 4 bis manquait, V et N, sont tenus.

Aux autres budgets : 12 modèles valides sur 12 à 8000 quads (budget tenu à 1,6 % près), 11 sur
12 à 1000 quads. Les mires de comparaison n'y étaient pas complètes ; elles l'ont été au jalon 8
(voir M9).

Ce qu'il ne faut pas se cacher :

- **Modèle par modèle, tout n'est pas tenu.** À 3000 quads, les seuils A, H, N et V sont tenus
  sur les 11 modèles où QuadriFlow donne un maillage (il échoue sur bridge, où le nôtre est
  valide). F (forme des quads) ne l'est pas sur rock, creature et spot ; S pas sur
  rock et creature ; C pas sur rock. Le scan brut (rock) est notre plus mauvais cas : QuadriFlow
  y ignore le bruit de surface, notre champ le suit.
- **Quad Remesher reste devant** sur l'alignement (de 1° environ) et sur la forme des quads.
- **Le complexe de base a doublé** par rapport au jalon 4 bis (1886 contre 951) : les jonctions
  en T du T-mesh décalent les lignes de part et d'autre (ticket DT-020).
- **Parties plus fines qu'un quad** : à 1000 quads, deux faces du cochon (oreilles) sont comptées
  inversées, et les oreilles de l'ogre perdent leur volume (ticket DT-018).
- **Bandes étroites** là où deux singularités proches sont tenues séparées (ticket DT-005).

### M9 — Relaxation et projection

- **Entrée** : maillage quad, source dense d'origine.
- **Sortie** : positions finales sur la source.
- **Méthode** : lissage tangentiel et projection au point le plus proche, lignes caractéristiques
  et coins préservés. Déjà en place et réutilisé tel quel. Le lancer de rayon le long de la
  normale est écarté : il rate la surface ou touche la face arrière. Une optimisation de forme
  plus poussée (type ARAP) est envisageable ensuite.
- **Test** : métriques H et F ; aucune face inversée.

**Réalisé au jalon 8** (`crates/retopo/src/relax.rs`, interface `Relaxer` et sa fabrique : deux
implémentations, `laplacian`, celle d'origine, et `squaring`, la nouvelle, qu'utilise la chaîne
`quad-layout`).

![L'ogre à 3000 quads après le jalon 8 : QuadriFlow, openclare, Quad Remesher](docs/images/jalon8_ogre.png)

Le jalon 7 avait montré que le lissage d'origine (chaque sommet vers la moyenne de ses voisins)
plafonnait : au-delà de deux passes il usait les parties fines et finissait par inverser une
face. La nouvelle relaxation change trois choses.

- **Ce vers quoi on tire.** Chaque quad propose, pour ses quatre coins, le rectangle qui lui
  ressemble le plus : même centre, même plan, côtés égaux à la moyenne de ses côtés opposés. Un
  sommet va vers la moyenne de ce que proposent ses quads, avec un quart de lissage ordinaire
  pour égaliser les tailles. On corrige ainsi les angles sans toucher aux dimensions : rien ne
  pousse le maillage à rétrécir.
- **Comment on bouge.** Le déplacement est rendu tangent à la surface avant la projection au
  point le plus proche : un sommet glisse, il ne s'enfonce pas.
- **Deux garde-fous.** Un sommet ne s'éloigne jamais de plus d'une demi-arête de l'endroit où
  la carte l'avait placé : c'est ce qui protège les pointes et les parties minces. Et un
  déplacement qui aplatirait ou retournerait un quad n'est pas fait.

Avec ces garde-fous, soixante passes améliorent encore la forme sans rien abîmer, là où
l'ancien lissage devait s'arrêter à deux.

**Résultat.** Médianes sur les 12 modèles organiques (tableaux complets et détail par modèle
dans `bench/results/ACCEPTANCE.md`, produit par `bench/acceptance.py`) :

| À 3000 quads | QuadriFlow | openclare, J7 | **openclare, J8** | Quad Remesher, taille uniforme | Quad Remesher, défaut |
|---|---|---|---|---|---|
| Modèles toujours valides (V) | 5 sur 12 | 12 sur 12 | **12 sur 12** | 10 sur 12 | 8 sur 12 |
| Écart d'angle moyen (F) | 8,6° | 7,8° | **5,4°** | 7,2° | 6,5° |
| Hausdorff, ‰ de la diagonale (H) | 17,7 | 9,1 | **8,1** | 9,6 | 5,9 |
| Alignement (A) | 17,5° | 8,6° | **8,4°** | 7,7° | 6,4° |
| Temps (T) | 1,9 s | 2,2 s | **2,2 s** | 12,3 s | 14,5 s |

Sur la forme des quads, openclare passe devant Quad Remesher ; sur la fidélité, devant sa
version à taille uniforme (sa version par défaut, à taille adaptative, reste devant). Les seuils
F et H sont désormais tenus sur les 12 modèles, un par un, et non plus seulement en médiane.

**La règle d'acceptation est maintenant établie aux trois budgets**, QuadriFlow et Quad
Remesher ayant été mesurés sur tout le corpus à 1000 et 8000 quads (ticket DT-022, fermé) :

| Médianes, 12 modèles organiques | 1000 quads | 3000 quads | 8000 quads |
|---|---|---|---|
| Modèles toujours valides : openclare / QuadriFlow | 11 / 4 | 12 / 5 | 12 / 3 |
| Alignement (A) : openclare / QuadriFlow / Quad Remesher uniforme | 9,1° / 18,0° / 8,1° | 8,4° / 17,5° / 7,7° | 8,7° / 15,8° / 8,6° |
| Sommets irréguliers (S) : openclare / QuadriFlow | 42 / 62 | 50 / 66 | 67 / 70 |
| Patchs du complexe de base (C) : openclare / QuadriFlow | 553 / 891 | 1886 / 2697 | 4436 / 7342 |
| Écart d'angle moyen (F) : openclare / QuadriFlow / Quad Remesher uniforme | 5,3° / 9,7° / 8,8° | 5,4° / 8,6° / 7,2° | 6,1° / 6,4° / 6,8° |
| Hausdorff, ‰ (H) : openclare / QuadriFlow | 18,5 / 38,9 | 8,1 / 17,7 | 4,7 / 6,9 |
| Écart au budget (N) : openclare / QuadriFlow | 0,8 % / 5,9 % | 1,4 % / 4,1 % | 0,8 % / 3,0 % |
| Temps (T) : openclare / QuadriFlow | 1,4 s / 1,3 s | 2,2 s / 1,9 s | 6,8 s / 4,1 s |

En médiane, tout est meilleur que QuadriFlow aux trois budgets, sauf le temps. Modèle par
modèle, les seuils manqués sont : à 1000 quads, S sur rock et creature, H sur cage, V sur pig
(une face comptée inversée sur une oreille, DT-018) ; à 3000, S sur rock et creature, C sur
rock ; à 8000, S et C sur rock, S et F sur creature, F sur spot, C sur jerry. À 8000 quads
l'avance sur la forme des quads se réduit (6,1° contre 6,4°) : soixante passes ne suffisent
plus tout à fait quand le maillage est plus fin (DT-023).

Ce qui reste ouvert :

- **L'alignement ne bouge presque pas** (8,4° contre 7,7° pour Quad Remesher) : il se joue en
  amont, dans le champ et le T-mesh, pas dans la relaxation.
- **Les tailles des quads sont un peu moins égales** qu'avec l'ancien lissage (variation d'aire
  de 0,34 à 0,38 sur blob) : les rectangles gardent les tailles que la quantification leur donne.
- **Coût** : 1,7 s sur l'ogre à 8000 quads, soit un sixième du temps total. À ce budget
  openclare est plus lent que QuadriFlow (6,8 s contre 4,1 s) (ticket DT-023).
- **Les seuils S et C** restent manqués sur rock et creature (DT-021) : la relaxation ne change
  pas la connectivité.
- **La seconde partie du jalon 8**, la densité adaptative, est faite depuis : voir M1 bis.

---

## 4. Jalons

Chaque jalon produit quelque chose de vérifiable et se termine par **un arrêt pour revue**.

| Jalon | Contenu | Ce qu'on peut vérifier à la fin |
|---|---|---|
| J0 | Banc de mesure, corpus, fabrique de chaînes | **Fait.** Chiffres de QuadriFlow, de Quad Remesher et du plancher ; seuils adoptés |
| J1 | Prétraitement (M1) | **Fait.** Maillages de calcul et courbures, tests analytiques |
| J2 | Champ de croix (M2) | **Fait.** Champs visualisés, singularités comptées |
| J3 | Singularités (M3), durée bornée | **Fait.** Gain mesuré sur S : critère de sortie atteint |
| J4 | Paramétrisation sans couture (M4) | **Fait.** Jalon de vérité passé : zéro inversion sur tout le corpus |
| J4 bis | Arrondi glouton des translations (façon MIQ) et extraction ; configuration unique et préréglages | **Fait.** Premier résultat complet, provisoire, mesuré face à QuadriFlow et à Quad Remesher |
| J5 | T-mesh (M5) | **Fait.** Partitions en rectangles valides sur tout le corpus, bords ouverts compris |
| J6 | Quantification (M6) | **Fait.** Contraintes satisfaites, budget tenu à 4 % près, connectivité du maillage quad construite et saine sur tout le corpus |
| J7 | Maillage quad depuis le T-mesh quantifié, par la route directe (à la place de M7 et M8) | **Fait.** Résultat complet ; règle d'acceptation tenue à 3000 quads. L'arrondi glouton reste comme repli (DT-009) |
| J8 | Relaxation ; densité adaptative, puis contraintes de rang 3 à 5 | **Fait, en attente de revue** : relaxation, densité adaptative (rang 2) et densité peinte (rang 5). Règle d'acceptation tenue aux trois budgets, forme des quads devant Quad Remesher. Reste le rang 4, les courbes guides (DT-024) ; le rang 3 relève du jalon 9 |
| J9 | Chaîne durci : traçage, patchs, remplissage par motifs | Pièces mécaniques comparées à QuadriFlow |
| J9 bis | Symétrie : un modèle symétrique par rapport à un plan donne un maillage symétrique (DT-028) | Écart entre le maillage et son reflet nul ; boucle d'arêtes sur le plan de symétrie |
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
| `Mesher` | maillage quad depuis la structure quantifiée | une grille par rectangle, lue dans la carte (M7) | remplissage par motifs |
| `Relaxer` | placement final (M9) | commune | commune |

Écrites à ce jour : `Remesher` (trois chaînes : `instant-meshes`, `integer-grid` et `quad-layout`),
`Preprocessor`, `CrossFieldSolver`, `SingularityOptimizer`, `Parametrizer` (deux
implémentations : `seamless` et `integer-grid`) et `LayoutBuilder` (une implémentation :
`motorcycle`, qui enchaîne la paramétrisation sans couture et le traçage du T-mesh),
`Quantizer` (une implémentation : `walk-cancelling`) et `Mesher` (une implémentation :
`patch-grid`), `Relaxer` (deux implémentations : `laplacian` et `squaring`) et `Sizer` (une
implémentation : `curvature`, pour le champ de taille). La chaîne
`quad-layout` les enchaîne ; c'est celle des préréglages. Toutes les étapes du tableau ont
maintenant leur interface.

Les deux chaînes ne diffèrent donc que par deux étapes, `LayoutBuilder` et `Mesher`, plus le
réglage du champ de croix. Les types échangés entre étapes (maillage de calcul, champ de croix,
structure à longueurs réelles puis entières, maillage quad) sont ceux des fiches de la section 3.

Règle de conduite : une interface est écrite avec sa première implémentation, pas avant, pour
qu'elle soit dictée par un besoin réel. Son contrat, lui, est déjà fixé par les fiches.

### Stack

- **Langage** : Rust, dans l'espace de travail existant.
- **Maillage** : tableaux plats et table des demi-arêtes opposées (`Topology`), à étendre avec
  des données par coin pour (u, v).
- **Algèbre linéaire creuse** : `faer` (Rust pur, licence MIT), enveloppé dans `linalg.rs` pour que
  le reste du code ne le nomme jamais. Sa factorisation de Cholesky creuse sert à la
  paramétrisation ; le champ de croix s'en passe (gradient conjugué). `faer` n'offre pas en haut
  niveau de factorisation pour systèmes symétriques indéfinis, d'où la formulation retenue en M4.
- **Requêtes spatiales** : le BVH existant.
- **Blender** : utilisé seulement par le banc de mesure, jamais comme dépendance.

### Configuration et préréglages

Dans l'esprit de Blender : un préréglage n'est qu'un jeu de paramètres nommé. L'utilisateur garde
la main sur chaque réglage, peut enregistrer les siens, et le projet en fournit quelques-uns,
documentés.

En place depuis le jalon 4 bis (`crates/retopo/src/config.rs`) :

```rust
pub struct RemeshConfig {
    pub method: Method,              // quelle chaîne
    pub target: Target,              // nombre de quads ou longueur d'arête
    pub alignment: f64,              // attraction vers la courbure (M2)
    pub singularity_reach: f64,      // paires annulées en deçà de tant d'arêtes de quad (M3)
    pub singularity_tolerance: f64,  // ce qu'une annulation a le droit de coûter (M3)
    pub crease_angle: Option<f64>,   // arêtes vives : None, ou seuil dièdre en degrés
    pub adaptivity: f64,             // taille des quads selon la forme : 0 unique, jusqu'à 1 (M1 bis)
    pub density_contrast: f64,       // densité peinte : rapport entre un poids de 1 et un poids de 0
    pub seed: u64,
}
```

Ce que l'utilisateur dessine sur un modèle précis n'est pas un réglage et ne va pas dans la
configuration : cela accompagne le maillage, dans `Guides` (aujourd'hui la densité peinte ;
demain les courbes guides).

- **Tous les champs sont publics** et modifiables un à un.
- **Enregistrer un préréglage**, c'est écrire la configuration dans un petit fichier texte
  (`clé = valeur`, une ligne par champ) ; le relire redonne la même configuration. Un fichier
  peut être édité à la main, et ce qu'il ne mentionne pas garde sa valeur par défaut.
- **Trois préréglages fournis**, listés par `retopo presets` :

| Préréglage | Ce qu'il privilégie | Réglages actuels |
|---|---|---|
| `organic` | fidélité à la sculpture | alignement 0,05 ; annulation jusqu'à 2 arêtes de quad ; adaptivité 0,5 |
| `animation` | peu de sommets irréguliers, pour bien se déformer | comme `organic`, annulation jusqu'à 3 arêtes |
| `hard-surface` | arêtes vives | arêtes de plus de 30° préservées ; alignement 0,02 ; taille unique |

Ces valeurs sont un point de départ, à régler sur le corpus. `hard-surface` tourne pour
l'instant sur la chaîne organique, en attendant la chaîne durci (J9).

Écarts avec la proposition d'origine, et pourquoi :

- `f64` partout au lieu de `f32` : c'est le type de tout le cœur.
- `preserve_sharp_edges` et `sharp_angle_threshold` fondus en `crease_angle: Option<f64>` : pas
  de seuil sans préservation, ni l'inverse.
- `singularity_reach` ajouté : le jalon 3 a montré que le réglage principal de l'annulation est
  une distance, la tolérance n'étant qu'un garde-fou.
- `method` ajouté : le durci aura sa propre chaîne.
- `adaptivity` et `density_contrast` ajoutés au jalon 8 avec la densité ; les fichiers
  enregistrés avant restent lisibles.

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
| Traçage non robuste (M5, M8) | T-mesh ou maillage incohérent | pour le T-mesh, le risque ne s'est pas réalisé sur le corpus (J5) ; arithmétique exacte en réserve. Portage de libQEx pour l'extraction |
| Solveur de couplage trop lourd à écrire (M6) | pas de quantification optimale | le solveur approché suffit sur le corpus (J6) ; le couplage reste en réserve |
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

Prises :

- objectif : mieux que le remaillage quad de Blender ; Quad Remesher est un palier, pas un plafond ;
- priorité : organique d'abord, durci ensuite ;
- lignées : paramétrisation et isolignes pour l'organique, patchs pour le durci ;
- architecture modulaire à fabriques ;
- arithmétique du traçage (M5) : nombres flottants et une tolérance unique, suffisants sur le corpus ;
- route du jalon 7 : directe ; la carte à grille entière et l'extraction exacte sont mises au
  backlog (DT-001) ;
- deux singularités proches restent séparées (DT-005) ;
- le code sera renommé `openclare` au jalon 10 (DT-015) ;
- tout ce qui est écarté, approché ou reporté fait l'objet d'un ticket numéroté dans BACKLOG.md ;
- densité : un seul champ de taille pour la densité adaptative et la densité peinte ; cette
  dernière se donne en poids de 0 (densité minimale) à 1 (densité maximale), avec un contraste
  réglable, et se peindra en Weight Paint dans Blender (DT-026) ;
- symétrie : inscrite à la feuille de route (jalon 9 bis, DT-028) ;
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
  (J12 à J14) ;
- une configuration unique `RemeshConfig` et trois préréglages (animation, organique, durci),
  introduits avec la première chaîne complète (J4 bis).
