# Cahier des charges : moteur de retopologie quad

**Statut : validé le 6 octobre 2026. Jalon 0 réalisé, en attente de revue.** Les points marqués
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

Campagne complète dans [bench/results/RESULTATS.md](bench/results/RESULTATS.md) : 9 modèles,
3000 quads demandés, médiane sur 5 graines. Mode d'emploi et limites dans
[bench/README.md](bench/README.md). Sur les cinq modèles organiques :

| | QuadriFlow (Blender) | Plancher (Instant Meshes) |
|---|---|---|
| Maillages valides (V) | 18 sur 25 | 22 sur 25 |
| Sommets irréguliers (S) | 8 à 78 | 91 à 212 |
| Patchs du complexe de base (C), pour 3000 faces | 1943 à 2884 | 2319 à 3048 |
| Alignement (A), degrés | 8,1 à 23,0 | 8,0 à 23,2 |
| Écart d'angle moyen (F), degrés | 5,8 à 8,7 | 4,7 à 7,1 |
| Écart au budget (N) | 1 à 11 % | 2 à 3 % |

Ce que ces chiffres établissent :

- **Le plancher est bien en dessous sur les singularités** : 3 à 11 fois plus de sommets
  irréguliers que QuadriFlow. C'est l'écart à combler, et il était attendu.
- **QuadriFlow n'est pas toujours valide.** Il produit des faces repliées sur le tore (4 graines
  sur 5), sur Suzanne (5 sur 5) et sur le blob (2 sur 5), et une face d'aire nulle sur la pièce
  mécanique (1 sur 5). Constat recoupé par un second test indépendant de la source.
- **Aucun des deux n'a de structure globale** : le complexe de base compte presque autant de
  patchs que de faces, parce que les lignes d'arêtes tournent longtemps avant de rencontrer une
  singularité. Seule exception : le cube chez QuadriFlow (6 patchs, le minimum). C'est là que la
  chaîne cible doit faire la différence, puisque le T-mesh impose cette structure.
- **Sur l'alignement et la forme des quads, les deux sont proches.** Sur le rocher bruité, les
  deux sont au niveau du hasard (22,5°).

Deux réserves sur les métriques elles-mêmes :

- **C sature.** Près du nombre de faces, elle ne départage plus : un maillage avec davantage de
  singularités peut même obtenir un C un peu plus bas, ses lignes s'arrêtant plus tôt. Elle ne
  se lit qu'avec S.
- **A dépend de l'échelle** à laquelle on lit la courbure (ici celle d'un quad). Sur une forme
  bruitée, cette courbure n'a pas de direction stable et la métrique ne dit rien.

### Seuils proposés — *à trancher*

Sur chaque modèle organique, médiane sur 5 graines, à budget égal :

| Métrique | Seuil proposé |
|---|---|
| V | 5 maillages valides sur 5 |
| S | pas plus que QuadriFlow, et strictement moins sur au moins 3 modèles sur 5 |
| C | au plus la moitié de celui de QuadriFlow |
| A | au moins 2° de mieux, là où QuadriFlow fait mieux que 20° |
| F | écart d'angle moyen pas plus élevé |
| H | Hausdorff au plus 1,2 fois celui de QuadriFlow |
| N | écart au budget d'au plus 5 % |

### Corpus — *à trancher*

Le jalon 0 tourne sur un **corpus provisoire**, entièrement généré dans Blender pour ne rien
télécharger : cinq formes organiques (blob, rocher bruité, créature à pattes, cage de genre 5,
Suzanne), deux mécaniques (pièce percée, cube) et deux étalons (sphère, tore). Elles couvrent les
cas utiles, mais ce sont des formes de synthèse, pas de vraies sculptures.

Reste à choisir le corpus définitif : une dizaine de modèles réels sous licence libre (personnage,
tête, main, animal, scan bruité), ce qui suppose de les télécharger.

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

**Contraintes utilisateur, par rang.**

1. Budget de quads cible.
2. Préservation optionnelle des arêtes vives au-delà d'un angle.
3. Courbes guides tracées sur la surface.
4. Masque de densité locale peint sur les sommets.

Le rang 1 conditionne l'acceptation. Les rangs 2 à 4 sont prévus par l'architecture mais livrés
après le premier résultat de bout en bout.

**Deux usages, deux chaînes.** L'organique (sculptures, personnages) est servi par la première
chaîne. Le durci (pièces mécaniques, surfaces dures) le sera par la seconde. L'utilisateur choisit
la chaîne ; le logiciel ne suppose jamais qu'il n'en existe qu'une.

**Sortie.** Maillage 100 % quads, variété, fermé ou à bords nets, avec **le moins possible de
sommets irréguliers**. (« 100 % de quads réguliers » est impossible hors du tore : la topologie
impose des sommets irréguliers.)

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
- **À trancher** : champ par face ou par sommet. L'article de Knöppel et al. travaille par
  sommet ; la paramétrisation (M4) attend un champ par face avec singularités aux sommets. Je
  propose par face.

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
| J0 | Banc de mesure, corpus provisoire, fabrique de chaînes | **Fait, en attente de revue.** Chiffres de QuadriFlow et du plancher ; seuils proposés |
| J1 | Prétraitement (M1) | Maillages de calcul et courbures, tests analytiques |
| J2 | Champ de croix (M2) | Champs visualisés, singularités comptées |
| J3 | Singularités (M3), durée bornée | Gain mesuré sur S, ou décision de s'en tenir au réglage global |
| J4 | Paramétrisation sans couture (M4) | **Jalon de vérité** : zéro inversion sur le corpus, ou activation d'un repli |
| J5 | T-mesh (M5) | Partitions en rectangles visualisées |
| J6 | Quantification (M6) | Contraintes satisfaites, budget tenu |
| J7 | Carte entière et extraction (M7, M8) | **Premier résultat complet, comparé à QuadriFlow** |
| J8 | Relaxation, puis contraintes de rang 2 à 4 | Règle d'acceptation évaluée |
| J9 | Chaîne durci : traçage, patchs, remplissage par motifs | Pièces mécaniques comparées à QuadriFlow |

**Variante à trancher** : après J4, un arrondi glouton des translations (comme dans MIQ) suivi de
l'extraction donnerait un premier résultat complet plus tôt, avant d'écrire M5 et M6. C'est moins
robuste que la quantification, mais cela permet de se comparer à QuadriFlow bien avant J7.

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

## 7. Décisions attendues

1. **Seuils d'acceptation** proposés en section 0.
2. **Corpus définitif** : quels modèles réels, et l'autorisation de les télécharger (section 0).
3. **Champ par face ou par sommet** (M2) ; proposition : par face.
4. **Variante MIQ** pour un premier résultat complet après J4 (section 4).
5. **Arithmétique du traçage** (M5) ; peut attendre J4.

Déjà décidé : objectif (mieux que Blender), priorité (organique d'abord, durci ensuite), lignées
(paramétrisation et isolignes pour l'organique, patchs pour le durci), architecture modulaire à
fabriques, licence (GPL-3.0-or-later).
