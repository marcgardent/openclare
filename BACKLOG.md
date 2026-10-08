# openclare — backlog de dette technique

Un ticket par raccourci, approximation, repli ou action du cahier des charges
([ALGORITHME.md](ALGORITHME.md)) qui n'a pas été faite. Rien n'est supprimé : un ticket soldé
passe à l'état *fermé*, avec le jalon qui l'a soldé. Les numéros ne sont jamais réutilisés.

Ce fichier ne contient pas la feuille de route (les jalons à venir sont dans ALGORITHME.md,
section 4), seulement ce qu'on a laissé derrière soi en avançant.

**Gravité** : *haute* = limite un résultat visé par la règle d'acceptation ; *moyenne* = limite
la qualité ou la robustesse hors du corpus ; *basse* = propreté, confort, garantie théorique.

## Vue d'ensemble

| Ticket | Sujet | Né au | Gravité | État |
|---|---|---|---|---|
| DT-001 | Carte à grille entière contrainte et extraction exacte, écartées | J7 | moyenne | ouvert |
| DT-002 | Arêtes vives absentes du T-mesh | J5 | haute (durci) | **fermé au J9** |
| DT-003 | Pièce sans singularité ni bord : pas de T-mesh | J5 | moyenne | **fermé (Q1)** |
| DT-004 | Solveur de quantification approché | J6 | basse | ouvert |
| DT-005 | Arcs maintenus ouverts : bandes de quads étroites | J6 | moyenne | ouvert |
| DT-006 | Budget de quads : pas plus fin qu'une bande | J6 | basse | ouvert |
| DT-007 | Course du T-mesh approchée, T-mesh non simplifié | J5 | basse | ouvert |
| DT-008 | Traçage en nombres flottants, sans garantie formelle | J5 | basse | ouvert |
| DT-009 | Chaîne `integer-grid` provisoire encore dans le dépôt | J4 bis | basse | ouvert |
| DT-010 | Lignes de défense 2 et 3 contre les inversions non écrites | J4 | moyenne | ouvert |
| DT-011 | Faces très étirées dans la carte sans couture | J4 | basse | ouvert |
| DT-012 | Préréglage `hard-surface` sur la chaîne organique | J4 bis | haute (durci) | **fermé au J9** |
| DT-013 | Valeurs des préréglages non réglées sur le corpus | J4 bis | moyenne | ouvert |
| DT-014 | Visualisation des champs et des structures | J2 | basse | **fermé (T1, T2)**, reste DT-035 |
| DT-015 | Code encore nommé `retopo` | J4 bis | basse | **closed at J10** |
| DT-016 | Documents en français, à traduire | J0 | basse | ouvert (new text in English since 8 October 2026; `CREDITS.md` translated) |
| DT-017 | Corpus : sculptures de l'auteur à ajouter avant la fin de la licence d'essai | J0 | moyenne | ouvert |
| DT-018 | Parties plus fines qu'un quad mal rendues à bas budget | J7 | haute | ouvert (doigts traités au Q2 ; restent les feuilles minces) |
| DT-019 | Relaxation rudimentaire : la forme des quads plafonne | J7 | haute | **fermé au J8** |
| DT-020 | Complexe de base doublé par les jonctions en T | J7 | moyenne | ouvert |
| DT-021 | Scan bruité (rock) : le champ suit le bruit | J7 | moyenne | ouvert |
| DT-022 | Mires de comparaison incomplètes à 1000 et 8000 quads | J7 | basse | **fermé au J8** |
| DT-023 | Relaxation : coût et tailles de quads | J8 | basse | ouvert |
| DT-024 | Courbes guides (rang 4) non commencées | J8 | moyenne | **closed (9 October 2026)**, DT-044 remains |
| DT-025 | Densité : portée des singularités et lecture de la courbure encore à l'échelle nominale | J8 | basse | ouvert (portée locale faite au Q2 ; reste la courbure) |
| DT-026 | Densité peinte : lecture du Weight Paint de Blender à faire (pont Blender) | J8 | moyenne | **closed at J10** |
| DT-028 | Symétrie : absente, Quad Remesher l'a | J8 | basse | ouvert, après la qualité |
| DT-029 | Quads étirés sur la paroi d'un cylindre | J9 | moyenne | ouvert |
| DT-030 | Chaîne par patchs (QuadWild) non écrite | J9 | basse | ouvert |
| DT-031 | Triangles d'aire quasi nulle laissés par le prétraitement | J9 | moyenne | ouvert |
| DT-032 | Modèles faits de nombreuses pièces détachées : toute la chaîne se replie | J9 | haute | **fermé (Q1)**, reste DT-034 |
| DT-034 | Petites pièces : quelques-unes passent encore par le repli, ou n'en sortent pas | Q1 | moyenne | ouvert |
| DT-035 | Visualisation Rerun : ce qui n'est pas montré, version en retard | T2 | basse | ouvert |
| DT-036 | Poches et cavités profondes refermées par le maillage | Q2 | moyenne | ouvert |
| DT-037 | Volume : le maillage est inscrit dans la forme | Q2 | haute | **closed (Q2 bis)**, DT-039 remains |
| DT-038 | Parties fines : limites de la borne de taille | Q2 | moyenne | ouvert |
| DT-039 | Fitting: limits, and the volume after subdivision | Q2 bis | medium | open |
| DT-040 | Improving a mesh from a dry run: tried for density, the rest to do | idea of the author | medium | open |
| DT-041 | Blender add-on, first version: what it does not do | J10 | medium | open |
| DT-042 | UV layers unfolded from the quad mesh: limits | J10 | medium | open |
| DT-043 | Creases: limits | J12 | low | open |
| DT-044 | Guide curves: not kept everywhere | guide curves | high | open |
| DT-033 | Terrains et scans : aucun outil n'aligne, le champ n'a rien à suivre | J9 | moyenne | ouvert |
| DT-027 | Garde-fous de temps de calcul sur les entrées pathologiques | J8 | basse | ouvert |

## Tickets

### DT-001 — Carte à grille entière contrainte et extraction exacte, écartées

- **Né au** jalon 7, par décision du 7 octobre 2026 (route « directe »).
- **Ce qui était prévu** : M7 (résoudre une carte (u, v) à transitions entières, contrainte par
  les longueurs quantifiées, sans triangle inversé) puis M8 (extraction exacte, principe de QEx).
- **Ce qui est fait à la place** : la connectivité du maillage vient directement de la
  quantification ; chaque sommet est placé en lisant la carte sans couture dans son rectangle,
  puis relaxé.
- **Ce qu'on y perd** : la carte entière répartit la déformation sur toute la surface ; la
  route directe étire chaque rectangle séparément, ce qui laisse des cassures aux frontières
  entre rectangles, que seule la relaxation atténue. Pas d'UV globales cohérentes avec le
  maillage (utile pour J10).
- **À reprendre si** la qualité mesurée au jalon 7 (métriques A et F) reste derrière Quad
  Remesher à cause de ces cassures, ou au plus tard pour l'export des UV (J10).

### DT-002 — Arêtes vives absentes du T-mesh

- **Né au** jalon 5.
- **Constat** : les arêtes vives sont tenues sur des isolignes de la carte, mais ne sont pas des
  coupes du T-mesh. Rien n'oblige donc une arête vive à porter un arc, et la quantification ne
  peut pas lui donner une coordonnée entière. Sur le cube, les traces les suivent d'elles-mêmes ;
  ce n'est pas garanti ailleurs.
- **Conséquence** : sur une pièce mécanique, des arêtes du maillage peuvent ne pas tomber sur
  l'arête vive ; seule la relaxation les y ramène.
- **À faire** : traiter les arêtes vives comme les bords ouverts (coupes présentes dès le
  départ, coins qui émettent des traces). Au plus tard au jalon 9.
- **Fermé au jalon 9** : bords et arêtes vives sont des « murs » du T-mesh. Le maillage a une
  arête le long de chaque arête vive et un sommet sur chaque coin, vérifié à 1e-6 près sur un
  bloc en L ; un cylindre garde ses deux bords circulaires.

### DT-003 — Pièce sans singularité ni bord : pas de T-mesh

- **Né au** jalon 5.
- **Constat** : un tore dont le champ n'a pas de singularité n'a aucune trace, donc ni T-mesh
  ni quantification.
- **Repli actuel** (jalon 7) : dès qu'une pièce de la surface n'a ni singularité ni bord, toute
  la chaîne `quad-layout` passe la main à `integer-grid` (DT-009), et le rapport le signale.
  Un modèle à plusieurs pièces dont une seule est un tore perd donc tout le bénéfice.
- **À faire** : quantifier les deux périodes à part, ou poser une coupe arbitraire ; traiter
  chaque pièce séparément.
- **Fermé au chantier Q1** : une pièce sans singularité ni mur est découpée à partir d'un de
  ses sommets, pris au hasard, d'où partent quatre traces. Le tore ressort en une seule grille
  régulière par la chaîne principale, sans repli. Ce cas, que je croyais marginal, est en
  réalité courant : sur un personnage de jeu, 92 pièces sur 164 étaient des anneaux.

### DT-004 — Solveur de quantification approché

- **Né au** jalon 6.
- **Ce qui était prévu** : Bi-MDF avec raffinement exact par couplage.
- **Ce qui est fait** : recherche locale par marches améliorantes. Sur de petits réseaux, elle
  trouve l'optimum exact dans 58 cas sur 60.
- **À reprendre si** des quantifications visiblement sous-optimales apparaissent.

### DT-005 — Arcs maintenus ouverts : bandes de quads étroites

- **Né au** jalon 6. Décision du 7 octobre 2026 : on garde les singularités séparées.
- **Constat** : pour ne pas fusionner deux singularités, certains arcs reçoivent une longueur
  minimale de 1 alors que leur longueur réelle est bien moindre (29 arcs sur les 12 modèles
  organiques à 3000 quads). Il en sort des bandes d'un quad de large, trop étroites.
- **Piste** : autoriser la fusion de deux singularités opposées proches (Lyon et al. 2021),
  en option réglable par préréglage.
- **Constaté au jalon 7** : ces bandes donnent des quads écrasés que deux passes de relaxation
  n'ouvrent pas tout à fait (quelques quads non convexes sur jerry). Les mêmes bandes le long
  des bords ont été supprimées en laissant une singularité devenir un coin de bord et deux
  coins de bord voisins fusionner ; entre singularités, la règle reste.

### DT-006 — Budget de quads : pas plus fin qu'une bande

- **Né au** jalon 6.
- **Constat** : le nombre de quads se règle par un facteur d'échelle commun ; le plus petit
  changement possible est une bande entière, soit 2 à 4 % à 1000 quads. Écart médian 1,4 %,
  pire cas 3,9 %.
- **Piste** : terme de budget dans l'objectif (Marinov et al. 2026).

### DT-007 — Course du T-mesh approchée, T-mesh non simplifié

- **Né au** jalon 5.
- **Constat** : les traces avancent triangle par triangle, la moins avancée d'abord ; l'ordre
  des rencontres peut différer de la course idéale à un triangle près. Aucune trace n'est
  raccourcie ni prolongée pour simplifier la partition.
- **Conséquence** : partition valide mais pas la plus simple ; un quart des rectangles sont des
  lamelles que la quantification referme.
- **Piste** : choix des traces à prolonger (Lyon et al. 2021).

### DT-008 — Traçage en nombres flottants, sans garantie formelle

- **Né au** jalon 5.
- **Ce qui était envisagé** : prédicats exacts ou rationnels.
- **Ce qui est fait** : flottants et tolérance unique de 1e-6. Aucun échec sur le corpus (48 cas).
  Une trace perdue rend la partition invalide, et c'est signalé.
- **À reprendre si** un modèle hors corpus fait échouer le traçage.

### DT-009 — Chaîne `integer-grid` provisoire encore dans le dépôt

- **Né au** jalon 4 bis. Le cahier des charges prévoyait de retirer l'arrondi glouton au jalon 7.
- **Constat** : elle sert encore de repli (DT-003) et de point de comparaison. Depuis le jalon 7
  elle n'est plus la chaîne des préréglages.
- **À faire** : la retirer, avec son extraction par cellules, quand la nouvelle chaîne la
  remplace partout.

### DT-010 — Lignes de défense 2 et 3 contre les inversions non écrites

- **Né au** jalon 4.
- **Ce qui était prévu** : contraintes de validité d'*Integer-Grid Maps*, puis construction à
  garantie (Campen et al. 2019) en dernier recours.
- **Ce qui est fait** : le raidissement seul. Zéro triangle inversé sur le corpus.
- **Conséquence** : hors corpus, une inversion irréductible invaliderait le T-mesh sans repli.

### DT-011 — Faces très étirées dans la carte sans couture

- **Né au** jalon 4.
- **Constat** : près de certaines singularités, des triangles sont valides mais écrasés
  (étirement jusqu'à 1000).
- **Conséquence** : la carte y est lue avec peu de précision ; à surveiller dans le placement
  des sommets (J7).

### DT-012 — Préréglage `hard-surface` sur la chaîne organique

- **Né au** jalon 4 bis.
- **Constat** : le préréglage existe, mais il passe par la chaîne organique, faute de chaîne
  durci. Voir aussi DT-002.
- **Fermé au jalon 9** : la chaîne unique garantit maintenant les arêtes vives ; le préréglage
  n'a plus besoin d'une autre chaîne. La chaîne par patchs prévue à l'origine est suivie à part
  (DT-030).

### DT-013 — Valeurs des préréglages non réglées sur le corpus

- **Né au** jalon 4 bis.
- **Constat** : `organic`, `animation` et `hard-surface` ont des valeurs raisonnables, pas des
  valeurs mesurées.
- **À faire** : une campagne de réglage quand la chaîne complète sera stable.

### DT-014 — Visualisation des champs et des structures

- **Né au** jalon 2. Reporté par décision (« nous verrons plus tard »).
- **Constat** : les champs, traces et patchs ne se regardent que par des OBJ de lignes et des
  rendus faits à la main.
- **Chantier T1 (8 octobre 2026)** : les maillages finaux ont maintenant de vrais rendus
  (Blender, `scripts/render.py`) et une page par modèle (`make pages`). Les étapes
  intermédiaires (champ, traces, patchs) restent sans visualisation.
- **Chantier T2 (8 octobre 2026)** : les étapes intermédiaires se regardent dans Rerun
  (`make view MODEL=ogre`). **Fermé** ; ce qui manque encore est suivi par DT-035.

### DT-015 — Code encore nommé `retopo`

- **Né au** jalon 4 bis. Décision du 7 octobre 2026 : renommage au jalon 10.
- **Constat** : crates, exécutable et scripts du banc portent l'ancien nom.
- **Closed at milestone 10 (8 October 2026)**: crates, executable, scripts and documents are
  named `openclare`. The folder of the repository on the author's machine keeps its name.

### DT-016 — Documents en français, à traduire

- **Né au** jalon 0, par décision (« pour les docs on traduira à la fin »).
- **Constat** : `ALGORITHME.md`, `BACKLOG.md`, les `README.md` et `docs/` sont en français ; le
  code et ses commentaires sont en anglais.
- **8 October 2026**: the author asks for documents to be written in English from now on, so
  that there is less to translate at the end. `CREDITS.md` is translated. What is added to the
  other documents is written in English; what they already hold in French (`ALGORITHME.md`,
  this file, the `README.md` files) is still to translate.

### DT-017 — Corpus : sculptures de l'auteur à ajouter

- **Né au** jalon 0. Réponse du moment : « oui, plus tard ».
- **Constat** : les mires Quad Remesher ne peuvent être produites que tant que la licence d'essai
  est valide. Tout modèle ajouté après son expiration n'aura pas de mire.
- **À faire** : fournir les sculptures, puis lancer `bench/make_references.py`.

### DT-018 — Parties plus fines qu'un quad mal rendues à bas budget

- **Né au** jalon 7.
- **Constat** : quand une partie de la forme est plus mince que le quad demandé (oreilles du
  cochon à 1000 quads), les sommets de ses deux faces se retrouvent presque au même endroit.
  Deux faces du cochon sont comptées inversées à 1000 quads : c'est le seul cas non valide des
  36 mesurés. L'ogre garde à 3000 quads un Hausdorff de 3 % de la diagonale (QuadriFlow : 3,7 %),
  probablement pour la même raison ; à vérifier.
- **Piste** : densité adaptative (rang 2 des contraintes, J8) ; à défaut, détection des parties
  minces et placement des sommets sur leur ligne médiane.
- **Remarque de l'auteur, 8 octobre 2026** : les doigts et les oreilles de l'ogre sont un vrai
  problème pour tous les remailleurs. L'exigence à tenir : garder au moins le volume de ces
  parties. C'est devenu le chantier de qualité Q2.
- **Au chantier Q2 (8 octobre 2026)** : une mesure d'épaisseur borne maintenant la taille des
  quads pour qu'il en passe six autour d'une partie fine (`quads_around`), et les singularités
  d'un doigt ne sont plus annulées. Sur l'ogre à 3000 quads, l'écart sur les parties fines
  passe de 30 à 9 % d'une arête de quad ; les mains ont des doigts. **Reste ouvert** : les
  feuilles minces (oreilles, tissus), que la mesure du tour laisse de côté à dessein et dont
  le bord reste grossier ; et les détails plus petits qu'un quad (la queue de `spot` à 1000
  quads est moins bien rendue qu'avant).
- **Au jalon 8** : avec la densité adaptative du préréglage, le cochon est valide à 1000 quads.
  Mais les oreilles de l'ogre restent son plus grand écart à la source : la courbure seule ne
  voit pas qu'une partie est mince. Il faudrait une mesure d'épaisseur locale dans le champ de
  taille.

### DT-019 — Relaxation rudimentaire : la forme des quads plafonne

- **Né au** jalon 7.
- **Constat** : la relaxation est un lissage laplacien suivi d'une projection. Deux passes
  améliorent la forme ; davantage érode les détails fins (le Hausdorff de l'ogre passe de 27 à
  46 ‰ entre 0 et 25 passes) et finit par inverser une face. La métrique F reste derrière
  QuadriFlow sur rock, creature et spot, et derrière Quad Remesher partout.
- **Conséquence** : c'était le premier frein à la qualité perçue.
- **Fermé au jalon 8** : nouvelle relaxation `squaring` (chaque quad tiré vers son rectangle,
  déplacement tangent, sommet tenu à une demi-arête de son point de départ, déplacements qui
  retournent un quad refusés). À 3000 quads, l'écart d'angle moyen passe de 7,8° à 5,4°
  (Quad Remesher : 7,2°) et le Hausdorff de 9,1 à 8,1 ‰. Le seuil F est tenu sur les 12 modèles.
  L'ancien lissage reste disponible (`laplacian`) pour les deux autres chaînes.

### DT-020 — Complexe de base doublé par les jonctions en T

- **Né au** jalon 7.
- **Constat** : 1886 patchs en médiane contre 951 pour l'arrondi glouton du jalon 4 bis. Deux
  rectangles voisins du T-mesh n'ont pas forcément leurs lignes en face l'une de l'autre ; les
  séparatrices du maillage final ne se rejoignent donc pas. Le seuil C reste tenu face à
  QuadriFlow (sauf sur rock), pas la visée (la moitié de QuadriFlow).
- **Piste** : simplification du T-mesh (DT-007), ou terme d'alignement des jonctions dans la
  quantification.

### DT-021 — Scan bruité (rock) : le champ suit le bruit

- **Né au** jalon 7 (visible depuis le jalon 2).
- **Constat** : sur le scan brut, 50 sommets irréguliers contre 8 pour QuadriFlow, alignement à
  16°, forme des quads moins bonne. Le champ s'aligne sur une courbure qui est en partie du bruit.
- **Piste** : lire la courbure à une échelle plus large sur les surfaces bruitées, ou baisser
  l'alignement là où la confiance est faible ; réglage des préréglages (DT-013).

### DT-022 — Mires de comparaison incomplètes à 1000 et 8000 quads

- **Né au** jalon 7.
- **Constat** : QuadriFlow n'a été lancé sur tout le corpus qu'à 3000 quads ; à 8000 il n'y a
  qu'un modèle, à 1000 aucun. La règle d'acceptation n'est donc établie qu'à 3000 quads.
- **Fermé au jalon 8** : tous les outils ont été lancés sur tout le corpus à 1000 et 8000
  quads (les mires Quad Remesher existaient déjà). La règle d'acceptation est tenue en médiane
  aux trois budgets ; le détail par modèle est dans `bench/results/ACCEPTANCE.md`. Réserve : ces
  deux campagnes ont tourné pendant que je compilais et testais, leurs temps de calcul sont donc
  un peu surestimés.

### DT-023 — Relaxation : coût et tailles de quads

- **Né au** jalon 8.
- **Constat** : soixante passes, chacune avec une projection sur la source par sommet : 1,7 s
  sur l'ogre à 8000 quads, un sixième du temps total. Par ailleurs les quads gardent les tailles
  que la quantification leur donne ; la variation d'aire monte un peu (0,34 à 0,38 sur blob).
- **Constaté aussi** : à 8000 quads les soixante passes ne suffisent plus tout à fait (écart
  d'angle moyen de 6,1° contre 5,4° à 3000 quads), et openclare y est plus lent que QuadriFlow
  (6,8 s contre 4,1 s en médiane).
- **Pistes** : nombre de passes selon la taille du maillage, ou du grossier au fin ; ne
  projeter qu'une passe sur plusieurs ; arrêt quand plus rien ne bouge ;
  projection sur le maillage de calcul plutôt que sur la source ; part de lissage plus forte
  là où deux rectangles voisins ont des densités différentes.

### DT-024 — Courbes guides (rang 4) non commencées

- **Né au** jalon 8 sous le titre « Densité adaptative et contraintes de rang 3 à 5 non
  commencées » ; **réduit** dans le même jalon.
- **Fait depuis** : la densité adaptative (rang 2) et la densité peinte (rang 5), par un même
  champ de taille (fiche M1 bis). Quad Remesher par défaut garde un avantage sur la fidélité à
  1000 et 3000 quads, mais il y dépasse le budget demandé de 71 % et 37 %.
- **Reste** : le rang 4, les courbes guides tracées sur la surface. Le rang 3 (arêtes vives)
  est suivi par DT-002 et DT-012.
- **Piste** : une courbe guide est une arête caractéristique que l'utilisateur dessine ; elle
  entrerait par `Guides`, comme la densité peinte, et serait traitée comme une arête vive.
- **Closed on 9 October 2026**: this is what was done, with curves drawn in Blender and held on
  the model by a Shrinkwrap, as the author proposed (fiche "Guide curves" of the
  specification). What is not reliable yet is DT-044.
- **Constat du 7 octobre 2026** : Quad Remesher (module Blender 1.4, vérifié) n'a pas de
  courbes guides. Il guide les boucles d'arêtes par ce que le maillage porte déjà : frontières
  entre matériaux, normales scindées, arêtes vives détectées par angle. Une variante bien moins
  coûteuse du rang 4 serait donc des **arêtes marquées par l'utilisateur** dans Blender, qui
  passeraient par le même mécanisme que les arêtes vives du jalon 9.
- **Objection du 7 octobre 2026** : marquer des arêtes est à peu près impossible sur un
  maillage à haute densité, et des courbes libres risquent de donner une interface lourde.
  (Depuis le jalon 9 le mécanisme existe côté moteur : une ligne à suivre est un « mur ». Il
  reste à faire entrer par `Guides` des murs désignés par l'utilisateur.)
  La question est donc d'abord celle du geste dans Blender. Pistes à discuter, de la plus
  naturelle sur une sculpture à la plus lourde : les frontières entre *Face Sets* du mode
  Sculpt (on peint des zones, leurs frontières deviennent des boucles) ; un trait d'annotation
  dessiné sur la surface ; des objets courbes. Rien n'est décidé.

### DT-025 — Densité : portée des singularités et lecture de la courbure encore à l'échelle nominale

- **Né au** jalon 8.
- **Constat** : là où les quads sont plus petits, deux réglages continuent de se compter en
  arêtes nominales : la portée de l'annulation des paires de singularités (M3), qui annule donc
  trop dans les zones denses et pas assez dans les zones lâches, et l'échelle à laquelle le
  champ de croix lit la courbure (M2).
- **Conséquence** : dans une zone peinte très dense, des détails que les quads pourraient suivre
  ne sont pas vus par le champ.
- **À faire** : rendre ces deux longueurs locales.
- **Au chantier Q2** : la portée de l'annulation se compte maintenant en quads locaux (le
  chemin entre deux singularités est mesuré dans la métrique du champ de taille). Reste
  l'échelle de lecture de la courbure.

### DT-026 — Densité peinte : lecture du Weight Paint de Blender à faire (pont Blender)

- **Né au** jalon 8.
- **Fait côté moteur** : la densité peinte est un poids par sommet, de 0 (densité minimale) à 1
  (densité maximale), avec un réglage de contraste (`density_contrast`). Convention fixée le
  7 octobre 2026 pour coller au Weight Paint : un sommet non peint vaut 0 et sert de niveau de
  base, il n'y a pas de valeur neutre à connaître.
- **Reste** : rien ne lit encore un groupe de sommets de Blender ; en ligne de commande, les
  poids viennent d'un fichier texte (`--density`).
- **Closed at milestone 10 (8 October 2026)**: the add-on lets the user pick a vertex group of
  the object and a factor (how many times more quads a weight of 1 gets than a weight of 0),
  and hands its weights to the engine.
- **À faire** au jalon 10, avec le pont Blender (pas commencé) : lire le groupe de sommets
  choisi par l'utilisateur et le passer tel quel au moteur.

### DT-027 — Garde-fous de temps de calcul sur les entrées pathologiques

- **Né au** jalon 8, sur Desert Rose (hors acceptation).
- **Constat** : sur une surface où la carte ne se déplie pas, des traces tournaient sur place
  pendant plus d'un quart d'heure ; leur nombre de pas est maintenant borné par leur longueur
  permise, et une carte repliée renvoie directement à la chaîne de repli. Mais ce repli
  (`integer-grid`) met encore plus de deux minutes sur ce modèle à 8000 quads, et la chaîne n'a
  pas de limite de temps globale.
- **À faire** : avec le prétraitement de réparation (J11) ; d'ici là, borner aussi le repli.

### DT-028 — Symétrie : absente, Quad Remesher l'a

- **Né au** jalon 8, en relisant les options du module Blender de Quad Remesher (1.4).
- **Constat** : Quad Remesher propose une symétrie selon X, Y ou Z. Le cahier des charges
  d'openclare ne la prévoit pas, et rien ne garantit aujourd'hui qu'un modèle symétrique
  donne un maillage symétrique.
- **Piste** : remailler une moitié avec un bord posé sur le plan de symétrie, puis la refléter ;
  les bords ouverts sont déjà pris en charge.
- **Décision du 7 octobre 2026** : voulue. Inscrite à la feuille de route comme jalon 9 bis.
- **Décision du 8 octobre 2026** : c'est une fonctionnalité annexe, qui ne fait pas progresser
  la qualité ; elle passe après les travaux de qualité.

### DT-029 — Quads étirés sur la paroi d'un cylindre

- **Né au** jalon 9.
- **Constat** : sur un cylindre à fonds plats, les quads de la paroi sont 1,3 fois plus larges
  que hauts. Le nombre de lignes qui longent la paroi est celui qui sort des deux disques, et
  un disque, dont les anneaux rétrécissent vers le centre, en veut moins que la paroi. La
  carte tranche entre les deux.
- **Pistes** : placer les quatre singularités de chaque disque plus près du bord (la zone en
  anneaux est alors mince) ; ou pondérer la carte par l'aire concernée de chaque côté.

### DT-030 — Chaîne par patchs (QuadWild) non écrite

- **Né au** jalon 9, par décision du 7 octobre 2026.
- **Ce qui était prévu** : une seconde chaîne pour les surfaces dures, sur le principe de
  QuadWild : tracer des patchs à partir du champ sans passer par une carte, puis les remplir
  par des motifs.
- **Ce qui est fait à la place** : les arêtes vives sont des coupes du T-mesh de la chaîne
  existante (fiche M5). Sur les quatre pièces du corpus, arêtes et coins sont retrouvés
  exactement.
- **Ce qu'on y perd** : la chaîne par patchs ne dépend pas d'une carte sans triangle inversé ;
  c'était aussi le repli prévu si la paramétrisation échouait (section 6). Ce repli n'existe
  donc pas : aujourd'hui, c'est `integer-grid` qui en tient lieu (DT-009).
- **À reprendre si** des pièces mécaniques réelles font échouer la carte ou le T-mesh.

### DT-031 — Triangles d'aire quasi nulle laissés par le prétraitement

- **Né au** jalon 9 ; le défaut date du jalon 1.
- **Constat** : pour ne pas ouvrir de fente, le remaillage isotrope garde des triangles plats
  le long des arêtes vives (aire de l'ordre de 1e-18). Ils faisaient s'effondrer la carte sans
  couture dans certaines configurations (cylindre, préréglage `hard-surface`).
- **Même famille** : un triangle dont deux côtés sont sur la même arête vive (il coupe un
  léger angle de la ligne) est forcément aplati dans la carte ; il faisait renoncer la carte à
  tenir les arêtes vives, et faussait le décompte des angles autour des sommets.
- **Traité en aval** au jalon 9 : la paramétrisation, le décompte des triangles inversés et le
  calcul des angles autour d'un mur s'en accommodent.
- **À faire** : les éliminer à la source, par bascule ou fusion d'arêtes qui respecte les
  arêtes vives.

### DT-032 — Modèles faits de nombreuses pièces détachées : toute la chaîne se replie

- **Né au** jalon 9, sur deux personnages de jeu récupérés de BlenderKit (voir CREDITS.md).
- **Constat** : `mahaveer` est fait de 164 pièces ouvertes détachées (vêtements, bijoux),
  `marine` de 820, avec en plus 305 arêtes à trois faces. Le cahier des charges dit que
  plusieurs pièces séparées sont « acceptées, chaque pièce est traitée seule » : ce n'est pas
  ce que fait le code. Tout est traité d'un bloc ; il suffit qu'une pièce n'ait pas de T-mesh
  valide pour que le modèle entier parte dans la chaîne de repli, qui met 84 secondes sur
  `mahaveer` et rend un maillage non valide (17 pièces sur 164, budget manqué de 28 %).
  `marine` est refusé d'emblée, avec un diagnostic clair, pour ses arêtes non variété : c'est
  le comportement prévu en attendant la réparation (J11).
- **Conséquence** : les modèles assemblés, courants dans la production, ne passent pas.
- **À faire** : traiter chaque pièce pour elle-même (budget réparti au prorata de l'aire,
  repli pièce par pièce), ignorer ou regrouper les pièces plus petites qu'un quad. La fusion
  des pièces qui se traversent en une seule peau reste l'affaire du jalon 11.
- **Fermé au chantier Q1** : chaque pièce est maillée pour elle-même, avec sa part des quads
  (au prorata de l'aire, pondérée par la densité peinte, avec un minimum par pièce), un
  maillage de calcul à sa mesure, et son propre repli si besoin. Sur `mahaveer` : 15 secondes
  au lieu de 84, 150 pièces sur 164 par la chaîne principale, budget tenu à 1,8 %. Ce qui
  reste est suivi par DT-034.

### DT-033 — Terrains et scans : aucun outil n'aligne, le champ n'a rien à suivre

- **Né au** jalon 9, sur un terrain de montagne de 543 557 triangles (BlenderKit).
- **Constat** : openclare y donne un maillage valide, au budget, avec trois fois moins de
  sommets irréguliers que Quad Remesher (56 contre 167) et une fidélité comparable. Mais
  l'alignement est à 18,6°, proche du hasard (22,5°), comme pour tous les outils mesurés
  (QuadriFlow 20,2°, Quad Remesher 18,6°) : un relief érodé n'a pas de directions de courbure
  stables à l'échelle d'un quad. Même famille que le scan `rock` (DT-021).
- **Piste** : sur ces surfaces, la métrique d'alignement ne dit pas grand-chose ; ce qui
  compterait serait de suivre les lignes de crête et de vallée, à détecter comme des lignes à
  suivre plutôt que par la courbure point par point.

### DT-034 — Petites pièces : quelques-unes passent encore par le repli, ou n'en sortent pas

- **Né au** chantier Q1, sur `mahaveer` (164 pièces) et sur Desert Rose (96 coques).
- **Constat** : sur `mahaveer` à 8000 quads, 6 pièces passent encore par la chaîne de repli et
  8 n'en sortent avec aucune face ; le maillage garde 9 faces qui ne sont pas des quads. Sur
  la rose, 12 coques sur 96 passent par le repli. Les causes, que le rapport détaille
  maintenant pièce par pièce : la carte se replie (le plus souvent), pas de longueurs entières
  saines, un bord hors des isolignes, une trace perdue.
- **Par ailleurs** : une pièce dont la juste part est inférieure à un quad reçoit quand même
  le minimum (6 quads pour une boule, 16 pour un anneau). Sur un modèle à très nombreuses
  petites pièces, ces minimums pèsent sur le budget. **Décidé le 8 octobre 2026** : on garde
  ce minimum par défaut, et le réglage `skip_pieces_under` permet de laisser de côté les
  pièces dont la part est inférieure à un seuil.
- **À faire** : reprendre ces causes une à une ; rendre le repli lui-même plus sûr sur les
  très petites pièces, ou s'en passer.

### DT-035 — Visualisation Rerun : ce qui n'est pas montré, version en retard

- **Né au** chantier T2.
- **Ce qui n'est pas montré** :
  - les patchs de la structure globale comme des régions colorées : on voit leurs bords (les
    coupes), pas leur surface ;
  - la carte sans couture (ses isolignes, ses triangles repliés), alors que « la carte se
    replie » est la première cause de repli (DT-034) ;
  - les traces perdues et les chevauchements, que le T-mesh compte sans garder leur position ;
  - la comparaison entre remailleurs : un enregistrement montre une seule exécution.
- **Approximations** : la longueur entière d'un arc est écrite au milieu de la corde entre ses
  deux nœuds, pas au milieu de l'arc ; les quads sont affichés en ombrage lissé.
- **Une étape à la fois** (demandé le 8 octobre 2026, fait) : le long de la ligne de temps
  `step`, chaque étape efface la précédente ; la ligne `stack` garde tout. Reste que les
  étapes d'un modèle en plusieurs pièces défilent pièce après pièce, sans vue d'ensemble
  d'une même étape sur toutes les pièces.
- **Version** : Rerun 0.36.3, la dernière qui se construit avec Rust 1.95 ; la plus récente
  (0.38.1) demande Rust 1.96. La visionneuse de la 0.36 impose de plus des versions exactes
  de `wasm-bindgen` et de ses voisines, fixées dans `Cargo.lock`.
- **Poids** : `Cargo.lock` passe à un millier de paquets, que seules les deux crates Rerun
  utilisent. Première construction en mode release : environ 5 minutes pour `openclare` avec
  `--features rerun` (31 Mo au lieu de 17), environ 17 minutes pour `openclare-view` (265 Mo).
  Les informations de débogage des dépendances ont été retirées du profil release pour
  cela : avec elles, `openclare-view` pesait 1,5 Go.
- **Visionneuse embarquée non vérifiée à l'œil** : `openclare-view` démarre et reste ouvert sans
  erreur, mais c'est dans la visionneuse web de Rerun que l'enregistrement a été regardé.
- **Temps mesurés** : pendant un enregistrement, le temps passé à écrire les calques d'une
  étape est compté dans l'étape suivante.

### DT-036 — Poches et cavités profondes refermées par le maillage

- **Né au** chantier Q2, en cherchant d'où vient le Hausdorff de l'ogre.
- **Constat** : à 3000 quads, le point de la source le plus éloigné du maillage (1,6 arête de
  quad) n'est pas sur un doigt mais dans deux petites poches derrière les yeux, de 0,15 de
  large et 0,35 de profondeur, que le maillage referme. Quad Remesher, au double du budget,
  les suit (0,36 arête). Tant qu'elles sont là, le Hausdorff de l'ogre (27 ‰) ne dit rien de
  ses mains.
- **Piste** : c'est le pendant en creux des parties fines. La même mesure, rayons lancés vers
  l'extérieur, donnerait la largeur d'une cavité ; reste à décider si on veut la suivre ou la
  boucher proprement.

### DT-037 — Volume : le maillage est inscrit dans la forme

- **Né au** chantier Q2. Priorité haute : l'auteur tient la préservation des volumes pour très
  importante.
- **Constat** : les sommets du maillage sont posés sur la surface. Partout où elle est bombée,
  les faces passent donc en dessous, et le volume baisse : 99,6 % du volume de l'ogre à 8000
  quads, 98,8 % à 3000, 96,5 % à 1000 ; entre 97 et 98 % à 1000 quads sur les autres modèles
  fermés. Sur une partie fine c'est bien plus : un doigt à six faces n'a que 83 % de la
  section du doigt, à quatre faces 64 %. Une subdivision de Catmull-Clark, usage normal d'un
  tel maillage, retire encore du volume.
- **Piste** : après la relaxation, ajuster les sommets le long de la normale pour que le
  maillage passe au milieu de la surface plutôt qu'en dessous (moindres carrés sur la distance
  de la source au maillage, et non des sommets à la source) ; en option, viser la surface
  limite de la subdivision. La mesure `volume_ratio` du banc est là pour le vérifier.
- **Closed at track Q2 bis (8 October 2026)**: a last stage, fitting, does this
  (`fitting.rs`, setting `volume_fit`). Median volume kept over the bench: 100.0 % at the three
  budgets; the ogre 99.8 % at 1000 quads and 100.0 % above. What it leaves open is DT-039.

### DT-038 — Parties fines : limites de la borne de taille

- **Né au** chantier Q2.
- **Pas de plafond sur la part du budget** que prennent les parties fines : sur un modèle fait
  surtout de parties fines, le reste devient très grossier. Le seul garde-fou est le plancher
  de taille (un quart de la taille demandée).
- **Le plancher est réglé sur un cas** : à un sixième, une ou deux faces de `jerry` se
  retournaient sur ses oreilles à 1000 quads ; à un quart, non. Rien ne garantit que ce soit
  vrai partout.
- **Plus de sommets irréguliers** : l'ogre passe de 88 à 155 à 3000 quads, `jerry` de 102 à
  162. Le préréglage `animation`, qui veut peu de sommets irréguliers, garde pour l'instant le
  même réglage que `organic` ; à revoir avec DT-013.
- **Les notes ne comptent pas les parties fines** : le barème de `SCORES.md` voit ce qu'elles
  coûtent (régularité) et pas ce qu'elles rapportent. Décision à prendre : ajouter un critère,
  ou l'intégrer à la fidélité.
- **L'épaisseur n'est pas montrée** dans l'enregistrement Rerun ; on n'y voit que son effet
  sur le champ de taille.
- **La mesure** : sept rayons en travers et quatre cordes par sommet, médiane sur le
  voisinage. Elle coûte de 0,3 à 0,5 seconde. Au bout d'un doigt, l'épaisseur en travers est
  surestimée d'un tiers environ (le rayon central file le long du doigt).
- **Préréglage `hard-surface`** : la borne y est désactivée, sans essai. Une pièce mécanique
  avec des tiges fines en aurait peut-être besoin.

### DT-039 — Fitting: limits, and the volume after subdivision

- **Born at** track Q2 bis.
- **After subdivision the volume is lost again.** Fitting makes the quad mesh keep the volume
  of the shape. A Catmull-Clark subdivision, the normal use of such a mesh, rounds it off and
  shrinks it. To keep the volume there, the fit should aim the limit surface of the
  subdivision at the source, not the faces of the mesh. **Measured at milestone 12**
  (`bench/creases.py` subdivides with Blender): the ogre at 1000 quads keeps 99.8 % of its
  volume as a quad mesh and 96.4 % once subdivided twice; 98.8 % at 3000 quads. The other
  closed organic models lose 2 to 3 % at 1000 quads the same way.
- **Rims stay inscribed**: vertices on hard edges, corners and borders do not move. The rim of
  a cylinder is still a polygon inside its circle; it could slide outwards in the plane of the
  cap.
- **The two guards are tuned on the bench**: a vertex goes no farther than a quarter of its
  shortest edge, and a face tilted by more than 20 degrees is put back. The second came from
  one face of Desert Rose; nothing says these values hold everywhere.
- **Only quads are looked at**: a face that is not a quad (the fallback chain leaves a few)
  asks for nothing, and its corners follow their other faces.
- **Vertices off the surface**: by design, but a later stage that expects them on the source
  (a shrinkwrap in Blender, say) will undo the fit. To say in the documentation of the add-on.

### DT-040 — Improving a mesh from a dry run

- **Born** on 8 October 2026, from an idea of the author: run the chain once to mark the mesh,
  turn the marks into one or several weights, and run again.
- **Tried for density** (a script outside the repository): after each round, a density is
  painted where the mesh was far from the source, and the next round uses it. On the ogre and
  the pig at 3000 quads, three rounds, contrast 4:

  | | Ogre, round 0 → best round | Pig, round 0 → best round |
  |---|---|---|
  | Distance to the mesh, 99th centile, share of a quad edge | 26.9 → 22.4 % | 17.4 → 15.3 % |
  | Mean angle deviation | 6.3 → 7.6 degrees | 5.3 → 6.2 degrees |
  | Irregular vertices | 155 → 183 | 102 → 104 |
  | Hausdorff distance | 28.4 → 25.6 ‰ | 10.0 → 8.2 ‰, then 11.8 the round after |

- **What it says**: density driven by the error gains 10 to 15 % on the worst distances, costs
  a degree of quad shape, and swings from one round to the next. Curvature and thickness
  already put the density about where it belongs, and every round redraws the whole mesh, so
  the marks of one round only half predict the next.
- **Where the idea should pay**, none of it written yet:
  - *keep the best of several rounds*: results vary from run to run, the marks exist in the
    library (`metrics::measure`), so marking a few runs and keeping the best is nearly free;
  - *a validity loop*: when a round gives an inverted face or a piece through the fallback,
    run again with what went wrong corrected there (smaller quads, another tolerance);
  - *a weight for alignment*: where edges follow the shape badly, pull the cross field harder
    towards curvature there. Alignment is the largest gap left with Quad Remesher.
- **Decided on 8 October 2026**: a quality track of its own, after the first version of the
  Blender add-on, starting with the best of several rounds and the validity loop.

### DT-041 — Blender add-on, first version: what it does not do

- **Born at** milestone 10, first step (8 October 2026), by decision: the least that runs
  first.
- **Blender waits while the engine runs**, and cannot be made to stop it. Since 8 October 2026
  the engine tells its host each stage as it begins and stops if the host says so
  (`Recorder::begin`, `on_stage` in the C interface), and the add-on shows the stage in the
  status bar, on the cursor and in the Info log, asking Blender to redraw each time. What is
  missing is a way for the user to say stop: Blender reads no key while an operator runs. The
  operator would have to run the engine aside, on a copy of the mesh (reading Blender's
  arrays in place from another thread is not safe while the user can change the scene).
- **The redraw in the middle of a run is a workaround** (`wm.redraw_timer`), which Blender's
  documentation advises against. Seen to work on Blender 5.2.2 on Linux (a picture of the
  window taken during a run shows the stage in the status bar); not tried elsewhere.
- **Few settings shown**: preset, quads, adaptive size, a vertex group for density and its
  factor (added on 8 October 2026 at the author's request). Quads round thin parts and volume
  fitting exist in the engine and pass through the interface as text; the panel does not show
  them.
- **Vertex group weights are read one vertex at a time** in Python: Blender offers no array of
  them. About 0.2 second for 100,000 vertices, so a couple of seconds per million.
- **Export, added on 8 October 2026** at the author's request: a section of the panel with
  boxes to tick for the two UV layers and for marking sharp the edges that follow a hard edge
  of the original. The limits of the UV layers are in DT-042.
- **Still to do**: reading the edges *marked* sharp in Blender as hard edges to keep. Today
  only the angle between faces makes a hard edge (preset *Hard surface*, 30 degrees); the
  engine has no way yet to be given a list of edges.
- **Linux, 64-bit Intel or AMD only.** The archive carries one native library; Windows and
  macOS need their own builds, and a way to make them (continuous integration).
- **Reading in place relies on how Blender lays a mesh out in memory**: positions as a packed
  array of three floats, corner vertices and triangle corners as packed integers. Checked on
  Blender 5.2.2 against the documented accessors; true since Blender 4.0 as far as its
  release notes say, not checked on 4.2 to 5.1. If a version changed it the add-on would read
  nonsense rather than fail; so the add-on compares a few values with the documented accessors
  first, and copies the arrays out the documented way if they do not match. That other way is
  tested, but only by forcing it.
- **Tried without interface only.** `make addon-test` runs the operator in a Blender started
  with `-b`; the archive is accepted by Blender's own validation. Nobody has installed it and
  pressed the button yet.
- **The result is a bare mesh**: no material, no vertex group, no custom attribute carried
  over from the original.
- **In the local coordinates of the object**: a non-uniform scale on the object is not applied
  before remeshing, so quads that are square in local space are not in the scene.

### DT-042 — UV layers unfolded from the quad mesh: limits

- **Born at** milestone 10 (8 October 2026), with the Export section of the add-on.
- **What they are**: both layers are read off the quad mesh itself, by laying one quad on a
  unit square and unfolding its neighbours next to it (`uv.rs`). It works on any mesh of
  quads, pieces through the fallback chain included; it does not use the map or the patches
  the chain worked with.
- **Not what was first described.** The atlas was announced as one rectangular island per
  rectangular patch of the layout. It is made of islands grown from the quads instead, each
  kept within a square of its own: far fewer islands (53 on the ogre at 8000 quads, where the
  layout has several hundred patches), but with ragged outlines.
- **Every quad takes the same room**, whatever its size on the surface. Where quads are four
  times smaller (thin parts, painted density), the texture is four times finer there, and
  coarser elsewhere.
- **The atlas is filled at 50 to 60 %** (60 % on the ogre at 3000 quads). Islands are odd
  shapes fitted cell by cell; a model in hundreds of pieces falls to 25 % (`marine`).
- **Many seams**: about a third of the edges on the ogre (1034 of 5898 at 3000 quads). Islands
  are cut where the unfolding meets itself, which leaves cuts inside them. Seams placed where
  they are least seen (creases, hidden sides) is another problem, not started.
- **The grid layer overlaps itself**, by nature, and its coordinates leave the unit square
  (they count quads). Fine for a pattern that repeats; a renderer that expects coordinates
  between 0 and 1 needs the texture set to repeat.
- **Faces that are not quads** (the fallback chain leaves a few) get a small island of their
  own.
- **Seen as a drawing of the coordinates only**: the two layers of the ogre were read back
  from Blender and plotted; nobody has yet looked at a texture on the model.

### DT-043 — Creases: limits

- **Born at** milestone 12.
- **Little effect on organic shapes.** The rule finds few folds there (124 edges on the ogre at
  3000 quads) and the subdivided mesh is hardly closer to the source for it. Either such
  shapes have few folds worth a crease, or the rule misses them; the bench cannot tell which.
  A picture of a subdivided nail or eyelid, with and without, would.
- **A fold is only creased where edges run along it.** Nothing here moves edges onto a fold:
  with the organic preset a fold the cross field did not follow is crossed by edges, and no
  edge gets its weight. Hard edges (preset *Hard surface*) and guide curves are what put
  edges on a line.
- **Thresholds set by hand**: no weight under 30 degrees, full weight from 70, at least 60 %
  of the turn close to the edge. Tried on a lens and on the bench, not tuned.
- **A partial weight is Blender's**: the engine gives a number from 0 to 1 and Blender turns
  it into a sharpness for the subdivision in its own way. Another host may read the same
  number otherwise.
- **Borders get no weight**: how a border is subdivided is a setting of the modifier.
- **The `part` model at 1000 quads** stays at 15 per mille after subdivision, creases or not:
  its holes have few quads round them and subdivision shrinks them.
- **No setting**: the weights are always computed (it costs a few queries per edge) and the
  add-on has a box to write them or not. The thresholds are not exposed.

### DT-044 — Guide curves: not kept everywhere

- **Born** with guide curves, 9 October 2026. Priority high: a curve that is let go is a
  feature that did not work for the user who drew it.
- **Which curves are kept is not predictable enough.** On the ogre at 3000 quads a line down
  the front or the back is kept, two rings out of three round the belly and the arms are, and
  the profile of the face and a line through the crotch are not. During the work the same
  curve went from kept to let go and back with changes that had nothing to do with it. What
  decides is one or two triangles of the work mesh along the line: a single fold of the map
  that stiffening cannot undo is enough to let a curve go.
- **Small loops need enough quads**: a loop under about nine quad edges round is let go (the
  eyes of the ogre at 3000 quads; kept at 8000).
- **Curves that touch are one**: letting go is decided curve by curve, but two curves that
  cross hang together and go together. A long curve with one bad stretch is lost whole, where
  it could be kept up to that stretch.
- **The fallback chain ignores curves**, and the fallback of the fallback is to do the piece
  again without them.
- **A curve is cut into the work mesh by its crossings**: its own bends inside a triangle are
  lost, which is nothing on a dense sculpt and too much on a model of a few large triangles.
  An open curve loses up to a triangle at each end.
- **Cost**: each curve tried alone is one more solve of the map, about half a second each on
  the ogre.
- **Only curve objects** are read by the add-on: not the edges of a mesh object, not
  annotation strokes, not face set borders. The edges along the curves come back in the
  report, but the add-on does not mark them on the result.
- **Not measured on the bench**: no model of the bench has curves, so no metric says whether
  the edges along a curve are where they should be; the tests check it on a sphere and on a
  flat sheet.
- **Where to look next**: the triangles along a laid line. Evening the mesh out again along
  the line (collapsing the short edges crossings leave), rather than making up for them in the
  map and in the layout as now, should make the outcome depend less on luck.
