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
| DT-003 | Pièce sans singularité ni bord : pas de T-mesh | J5 | moyenne | ouvert |
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
| DT-014 | Visualisation des champs et des structures | J2 | basse | ouvert |
| DT-015 | Code encore nommé `retopo` | J4 bis | basse | ouvert |
| DT-016 | Documents en français, à traduire | J0 | basse | ouvert |
| DT-017 | Corpus : sculptures de l'auteur à ajouter avant la fin de la licence d'essai | J0 | moyenne | ouvert |
| DT-018 | Parties plus fines qu'un quad mal rendues à bas budget | J7 | moyenne | ouvert (atténué au J8) |
| DT-019 | Relaxation rudimentaire : la forme des quads plafonne | J7 | haute | **fermé au J8** |
| DT-020 | Complexe de base doublé par les jonctions en T | J7 | moyenne | ouvert |
| DT-021 | Scan bruité (rock) : le champ suit le bruit | J7 | moyenne | ouvert |
| DT-022 | Mires de comparaison incomplètes à 1000 et 8000 quads | J7 | basse | **fermé au J8** |
| DT-023 | Relaxation : coût et tailles de quads | J8 | basse | ouvert |
| DT-024 | Courbes guides (rang 4) non commencées | J8 | moyenne | ouvert (réduit au J8) |
| DT-025 | Densité : portée des singularités et lecture de la courbure encore à l'échelle nominale | J8 | basse | ouvert |
| DT-026 | Densité peinte : lecture du Weight Paint de Blender à faire (pont Blender) | J8 | moyenne | ouvert |
| DT-028 | Symétrie : absente, Quad Remesher l'a | J8 | moyenne | ouvert, planifié au J9 bis |
| DT-029 | Quads étirés sur la paroi d'un cylindre | J9 | moyenne | ouvert |
| DT-030 | Chaîne par patchs (QuadWild) non écrite | J9 | basse | ouvert |
| DT-031 | Triangles d'aire quasi nulle laissés par le prétraitement | J9 | moyenne | ouvert |
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

### DT-015 — Code encore nommé `retopo`

- **Né au** jalon 4 bis. Décision du 7 octobre 2026 : renommage au jalon 10.
- **Constat** : crates, exécutable et scripts du banc portent l'ancien nom.

### DT-016 — Documents en français, à traduire

- **Né au** jalon 0, par décision (« pour les docs on traduira à la fin »).
- **Constat** : `ALGORITHME.md`, `BACKLOG.md`, les `README.md` et `docs/` sont en français ; le
  code et ses commentaires sont en anglais.

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

### DT-026 — Densité peinte : lecture du Weight Paint de Blender à faire (pont Blender)

- **Né au** jalon 8.
- **Fait côté moteur** : la densité peinte est un poids par sommet, de 0 (densité minimale) à 1
  (densité maximale), avec un réglage de contraste (`density_contrast`). Convention fixée le
  7 octobre 2026 pour coller au Weight Paint : un sommet non peint vaut 0 et sert de niveau de
  base, il n'y a pas de valeur neutre à connaître.
- **Reste** : rien ne lit encore un groupe de sommets de Blender ; en ligne de commande, les
  poids viennent d'un fichier texte (`--density`).
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
