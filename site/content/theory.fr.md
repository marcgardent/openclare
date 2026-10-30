# Fermer les boucles d'arêtes : la théorie expliquée simplement

*[English version](theory.md)*

Cette page raconte, pour les artistes 3D et les développeurs curieux, ce que les deux articles de recherche,
[Closing Edge Loops at Quantization Time](../../docs/research/closing-loops/paper.fr.pdf) (appliqué) et
[Turns of a T-Mesh](../../docs/research/turns-of-a-t-mesh/paper.pdf) (théorie, en anglais), ont trouvé en apprenant
à openclare à fermer ses boucles d'arêtes : l'idée, les règles qui en sont sorties, ce que cela a
changé dans le moteur, et ce qui n'a pas marché. Elle se lit en un quart d'heure environ. Les
mots en *italique* sont définis, avec d'autres images, dans le [glossaire](../../docs/GLOSSARY.fr.md).

Chaque affirmation ci-dessous est de l'une de ces trois sortes, et nous disons laquelle :

- **démontré** : une preuve existe, et pour la plupart un ordinateur l'a vérifiée (section 8) ;
- **mesuré** : calculé sur de vrais maillages, exactement là où nous le disons ;
- **conjecturé** ou **réfuté** : une hypothèse, encore ouverte ou montrée fausse.

La note est un brouillon et n'a pas été relue par des pairs.

## 1. Le problème que voit un artiste

Demandez à un remailleur automatique un maillage de quads d'un bras, et regardez les lignes
d'arêtes qui en font le tour. Un artiste attend de chacune qu'elle soit un *anneau* : elle fait
le tour une fois et se referme sur elle-même, si bien qu'on peut la sélectionner d'un Alt-clic, la
faire glisser ou couper le maillage le long d'elle. Bien souvent, c'est une *hélice* : elle fait
le tour, revient une rangée plus haut, et remonte tout le bras comme le filet d'une vis.

![Un anneau et une hélice sur un tube, et le tube ouvert à plat](../../docs/glossary/ring_helix.png)

*À gauche, un anneau : il revient à son départ. À droite, une hélice de pas 1 : elle revient une
rangée plus haut. En dessous, les mêmes tubes ouverts et mis à plat.*

Une hélice n'est pas fausse géométriquement : les quads peuvent être carrés et suivre
parfaitement la forme. Elle est fausse pour la personne qui doit retoucher le maillage. Une seule
spirale peut faire d'une jambe entière une boucle unique de la hanche à la cheville, et c'est
exactement ce qu'openclare faisait, avant ce travail, sur le corps masculin qui nous sert de
témoin.

Voici un cylindre remaillé par openclare, avant et après le travail décrit ici.

![Un cylindre remaillé par openclare, avant et après la fermeture des boucles](../../docs/glossary/cylinder_result.png)

*Avant : les boucles spiralent (en rouge), sur environ 700 patchs. Après : toutes les boucles sont
des anneaux (en bleu), et le découpage est celui d'un cube, 6 patchs.*

Et le corps masculin, remaillé à 10 590 quads, le nombre de quads du maillage de base qu'un
artiste a modélisé à la main pour lui.

![Le corps masculin avant et après la fermeture des boucles](../../docs/glossary/male_result.png)

*Boucles en spirale du jaune au rouge, boucles fermées en bleu, d'un passage plus ancien du moteur.
Avec le moteur d'aujourd'hui : 26,8 % des arêtes sur des spirales sans fermeture, 1,95 % avec,
contre 2,55 % pour le maillage fait à la main.*

Le problème est bien connu. Bommes et ses collègues ont nommé ces hélices *q-helices* en 2011 et
les retiraient d'un maillage fini par des retouches locales. openclare prend le chemin inverse :
il les empêche plus tôt, au moment où le maillage décide combien de quads vont où.

## 2. Comment openclare construit un maillage de quads

Pour voir où naissent les hélices, suivons les cinq étapes du moteur.

**1. Un champ de croix.** En chaque point de la surface, openclare calcule une croix à quatre
bras à angle droit : les directions que les arêtes doivent suivre. Les croix suivent la courbure
de la forme. En quelques points, la croix tourne d'un quart de tour quand on en fait le tour : ce
sont les *singularités*, et chacune devient un *pôle* du maillage, un sommet où se rejoignent 3
ou 5 arêtes au lieu de 4.

![Un champ de croix qui suit une courbe, et une singularité](../../docs/glossary/cross_field.png)

*À gauche : les croix suivent la courbure. À droite : après un tour autour d'une singularité, la
croix revient tournée d'un quart de tour ; trois lignes en partent, et elle devient un pôle de
valence 3.*

**2. Des lignes depuis les pôles.** De chaque pôle, openclare suit les lignes du champ qui en
partent (les *séparatrices*) jusqu'à ce que chacune rencontre une autre ligne ou un bord.

**3. Un patchwork de rectangles.** Ces lignes découpent la surface en rectangles, les *patchs*.
Leurs côtés sont faits de morceaux de ligne, les *arcs*. Ce patchwork s'appelle un *maillage en
T*, parce qu'une ligne qui s'arrête sur une autre dessine un T. Sur le corps masculin, il compte
776 patchs et 2 069 arcs.

![Séparatrices, traces, nœuds, arcs et patchs d'un petit maillage de quads](../../docs/glossary/tmesh.png)

*Les lignes qui partent des pôles découpent la surface en rectangles de quads.*

**4. Des longueurs entières.** Chaque arc doit maintenant recevoir un nombre entier d'arêtes de
quad. Le champ demande des longueurs réelles, comme 3,4 ou 5,7 ; arrondir chacune séparément casse
les rectangles, dont les côtés opposés doivent rester égaux. Choisir des nombres entiers qui
gardent chaque rectangle rectangle, au plus près de ce que demande le champ, s'appelle la
*quantification*.

![Longueurs réelles, arrondi simple et quantification de trois patchs](../../docs/glossary/quantization.png)

*À gauche, ce que demande le champ. Au milieu, l'arrondi simple : le côté gauche a 5 arêtes, le
côté droit 2 + 2, les quads ne tombent pas juste. À droite, la quantification : 5 = 2 + 3.*

**5. Des quads.** Chaque rectangle, maintenant à côtés entiers, est rempli d'une grille de quads.

C'est à l'étape 4 que se décide la structure entière du maillage et, nous l'avons découvert, si
chaque boucle se ferme ou spirale. Avant ce travail, openclare choisissait les longueurs entières
pour leur cohérence et leur fidélité au champ seulement ; les boucles se fermaient ou spiralaient
au hasard.

## 3. L'idée clé : un tour, et son décalage

Partez d'un patch, traversez-le tout droit, entrez dans le patch voisin, et continuez tout droit.
Il arrive que vous reveniez dans le patch de départ, par le même côté, dans la même direction. Ce
parcours est un *tour*. La bande de quads qui le suit est soit un anneau, soit une hélice.

L'endroit exact où vous revenez le long de ce côté est le *décalage* du tour : 0 pour un anneau,
le pas de l'hélice sinon.

![Un tour à travers les patchs d'un tube, qui revient avec un décalage 0 puis 1](../../docs/glossary/turn_shift.png)

*Un tour traverse les patchs autour d'un tube. En haut, il revient à son départ : décalage 0, un
anneau. En bas, une rangée plus haut : décalage 1, une hélice de pas 1.*

Le premier résultat de la note (**démontré**) est que le décalage est une simple somme de
longueurs d'arcs, avec des coefficients entiers qui ne dépendent que des patchs traversés, pas des
longueurs elles-mêmes. Quelque chose comme :

> décalage = longueur de l'arc 3 + longueur de l'arc 7 − longueur de l'arc 12

Donc **fermer une boucle, c'est annuler un nombre**, et ce nombre est une simple somme des
longueurs mêmes que choisit la quantification. Fermer les boucles devient une contrainte qu'on
peut demander à la quantification de respecter, comme l'égalité des côtés des rectangles.

Le deuxième résultat (**démontré**, et vérifié par ordinateur) répond à la question « comment
trouver tous les tours ? ». Partez du milieu de chaque arête de quad, sur chaque côté de chaque
patch : vous rencontrez toutes les bandes de quads du maillage final, sans en manquer une. Sur le
corps masculin, ce parcours complet trouve environ 220 tours pour chaque variante qu'essaie le
moteur (section 6), et 12 à 24 d'entre eux reviennent décalés alors que le champ lui-même les
décale de moins d'une demi-rangée : des anneaux gâchés par l'arrondi. Les parcours par
échantillons qu'utilisait le moteur n'en trouvaient que 4 à 10 (**mesuré**).

## 4. D'où viennent les décalages

Pourquoi un tour revient-il décalé ? La réponse est empruntée à la physique des cristaux.

**Le vecteur de Burgers.** Dans un cristal parfait, faites 6 pas à droite, 6 vers le haut, 6 à
gauche et 6 vers le bas : vous êtes revenu au point de départ. Si le cristal a un défaut, une
demi-rangée d'atomes en trop, le même parcours manque son départ d'un pas. Ce pas manquant est le
*vecteur de Burgers* du défaut.

Un maillage de quads est un cristal de quads, et ses défauts sont ses pôles. Un pôle de valence 3
à côté d'un pôle de valence 5, une *paire 3/5*, est exactement un défaut de cristal : un parcours
autour d'elle manque son départ d'une arête en travers et d'une arête en long.

![Le circuit de Burgers dans un cristal et autour d'une paire 3/5 d'un maillage de quads](../../docs/glossary/burgers.png)

*À gauche, un cristal avec une demi-rangée en trop : le parcours en carré manque d'un pas. À
droite, le même parcours autour d'une paire 3/5 d'un maillage de quads. Les deux parcours sont
calculés, pas dessinés à la main.*

La note démontre que **le décalage d'un tour est le vecteur de Burgers des pôles situés d'un côté
de ce tour**, additionnés dans un ordre précis (celui où un parcours autour d'eux les rencontre
pour la première fois). Ce n'est pas le champ qui spirale ; ce sont les pôles qui poussent les
rangées. **Mesuré :** sur le corps masculin, elle tient sur les 1 436 régions testées, sans exception.

**L'escalier.** Placez une paire 3/5 entre deux tours d'un membre. La différence de leurs
décalages est alors la longueur d'un *escalier* : un chemin d'arêtes du 5 au 3, qui avance dans
deux directions à un quart de tour l'une de l'autre.

![Deux tours d'un tube avec une paire 3/5 entre eux et l'escalier qui relie la paire](../../docs/glossary/staircase.png)

*Le tour 1 est un anneau, le tour 2 revient 2 rangées plus haut, et 2 est la longueur de
l'escalier du 5 au 3. Décalages mesurés sur le maillage.*

Les deux tours ne peuvent être des anneaux que si l'escalier est de longueur nulle, c'est-à-dire
si les deux pôles fusionnent. Or openclare garde les pôles à au moins une arête l'un de l'autre,
pour qu'ils ne s'écrasent pas l'un sur l'autre. Alors l'un des deux tours au moins doit être une
hélice (**démontré**).

**La séparatrice qui s'enroule.** Une boucle du corps masculin, autour du poignet gauche, ne peut
jamais se fermer, quelles que soient les longueurs. La raison : une ligne part d'un pôle de
valence 5, fait une fois le tour du poignet (6,99 arêtes) et vient se poser sur une autre ligne du
même pôle, à 0,3 arête de lui. Le tour du poignet revient décalé d'exactement cet écart. Comme cet
écart contient un arc qui doit garder au moins une arête, le tour du poignet est une hélice de pas
au moins 1 pour tout choix admissible de longueurs (**démontré** ; l'identité **mesurée** sur le
maillage).

![Une séparatrice qui fait une fois le tour du poignet et se pose à côté de son propre pôle](../../docs/glossary/winding_separatrix.png)

*La ligne quitte le pôle, fait le tour du poignet et se pose juste à côté de son propre pôle ; le
tour du poignet hérite de l'écart.*

Quand deux tours du corps masculin ne peuvent pas se fermer ensemble, nous avions supposé que ce
serait toujours une paire 3/5 posée sur une même ligne entre eux. Cette hypothèse est
**réfutée** : elle tenait dans 0 des 13 conflits examinés. Ils se rangent en six configurations :
un escalier entre une paire 3/5, trois échanges de deux pôles de valence 3 entre des tours qui se
croisent, et deux boucles plus longues, dont une autour d'une calotte comme un bout de doigt.

**Les peignes.** Il existe une façon plus simple de lire chaque conflit. Prenez n'importe quel arc
du patchwork : les deux lignes qui le bornent partent chacune d'un pôle. Ensemble, elles forment
un *peigne*, d'un pôle, à travers l'arc, jusqu'à l'autre. La longueur de l'arc est exactement
l'écart entre ces deux pôles, mesuré en travers de la direction de l'arc (**démontré**,
**mesuré** sur les 2 069 arcs du corps masculin). Presque chaque arc est donc une minuscule paire
3/5 ou un échange de deux pôles, et toute preuve que des tours ne peuvent pas se fermer ensemble
est une somme de peignes (**démontré**). Les deux boucles plus longues ci-dessus sont des chaînes
fermées de six et de quatre peignes.

## 5. Des règles que personne ne peut enfreindre

Certaines contraintes valent pour tout maillage de quads, quel que soit le logiciel et quel que
soit le modeleur.

**Compter les pôles.** Donnez à chaque pôle un *indice* : +1 pour la valence 3, −1 pour la
valence 5. Sur toute surface fermée en forme de boule, les indices font 8 au total : une sphère en
quads a besoin d'au moins huit pôles de valence 3, les coins d'un cube (**démontré**, classique).
Le corps masculin a 133 pôles de valence 3, 127 de valence 5 et un de valence 2 (indice +2) :
133 − 127 + 2 = 8. Et un anneau coupe toujours une telle surface en deux moitiés d'indice 4
chacune : un bout de doigt au-delà d'un anneau contient l'équivalent de quatre pôles de valence 3.
Les trous changent le compte : le côté d'un anneau qui contient un trou compte 4 de moins par
trou, si bien qu'un anneau qui longe un trou peut n'avoir aucun pôle de son côté. Un tel *collier*
autour d'un trou aux bords droits se ferme toujours (**démontré** ; **mesuré** sur les 40
colliers autour des yeux de Suzanne).

![La QuadSphere : un cube gonflé en sphère](../../docs/glossary/quadsphere.png)

*La QuadSphere : 8 pôles de valence 3, 6 patchs, toutes les boucles des anneaux. openclare la
produit désormais sur une sphère.*

**Parité : les anneaux ont un nombre pair de quads.** Sur une surface fermée en forme de boule,
coloriez les sommets des quads en deux couleurs de sorte que deux voisins diffèrent toujours ;
c'est toujours possible. En faisant le tour d'un anneau, les couleurs alternent, donc **tout
anneau a un nombre pair de quads** (**démontré**). Une conséquence qui compte pour le moteur : une
hélice de pas impair ne peut devenir un anneau que si le nombre de quads de son tour change d'un
nombre impair. Une petite réparation qui garde ce nombre ne peut jamais la fermer. **Mesuré** sur
les 1 299 tours du corps masculin.

![Un anneau sur une sphère fermée aux coins bicolores, et un anneau de 5 quads autour d'un trou](../../docs/glossary/parity.png)

*À gauche, sur une surface fermée les couleurs alternent : ici un anneau de 16 quads. À droite,
autour d'un trou les couleurs peuvent se heurter, et un anneau de 5 quads devient possible.*

**La surprise : les bords la cassent.** Nous avions d'abord énoncé la règle de parité pour toute
surface en forme de sphère. La vérification des preuves par ordinateur (section 8) a montré
qu'elle exige une surface *fermée*. Et elle est fausse en pratique : sur Suzanne, la tête de singe
de Blender, dont les yeux sont des trous, openclare a produit un anneau de **23 quads** autour
d'une orbite (**mesuré**). Avec d'autres longueurs, le même anneau en a 24. La règle corrigée
(**démontrée**, **mesurée** sur 660 tours sur 660 de Suzanne, de jerry et d'un dôme) dit qu'avec
des bords, la longueur plus le décalage d'un tour a la parité de la longueur de bord située d'un
côté de ce tour.

**La règle du damier.** Entre deux tours parallèles, coloriez les points entiers du plan en
damier et comptez les pôles de valence 3 ou 5 posés sur des points blancs. Si ce nombre est
impair, les deux tours ne peuvent pas être tous deux des anneaux (**démontré** ; **mesuré** sur
30 148 paires de tours du corps masculin).

![La règle du damier pour une paire 3/5 entre deux tours](../../docs/glossary/checkerboard.png)

*Un quart de tour autour d'un point entier garde les couleurs. Au milieu : un pôle sur un point
blanc, les deux tours ne peuvent pas être tous deux des anneaux. À droite : zéro, cette règle ne
l'interdit pas.*

**Les hélices forcées.** Retournées, les obstructions donnent un plancher : si un groupe de tours
ne peut jamais se fermer tout entier, au moins l'un d'eux est une hélice dans *tout* maillage de
quads qui garde ces pôles séparés. Avec plusieurs groupes disjoints, les hélices s'additionnent.
Par exemple, un membre avec trois paires 3/5 empilées dans le même sens a au moins trois hélices
parmi ses quatre tours (**démontré**).

![Quatre tours d'un membre séparés par trois paires 3/5, de décalages 0, 2, 4 et 6](../../docs/glossary/forced_helices.png)

*Chaque paire 3/5 ajoute la longueur de son escalier au décalage, toujours dans le même sens : un
seul des quatre tours au plus peut être un anneau.*

Nous ne connaissons pas de borne inférieure antérieure de ce genre. Sur le corps masculin
(**mesuré**, calculé exactement avec un solveur d'optimisation), les pôles n'imposent que **1 à 9
hélices** parmi les
tours retenus pour la fermeture, selon la variante, là où le moteur en laissait **13 à 123**. L'une
des hélices forcées est celle du poignet ; les autres sont aux yeux, aux mains et aux pieds.

![Hélices forcées contre celles que laisse le moteur, pour les huit variantes du corps masculin](../../docs/glossary/forced_vs_engine.png)

*Pour chacune des huit variantes qu'essaie le moteur, les hélices qu'aucune quantification ne peut
éviter (rouge foncé) contre celles que le moteur laissait (rouge clair).*

C'est la leçon la plus utile de la théorie : **les pôles ne sont pas les principaux coupables**.
La plupart des hélices que laisse le moteur pourraient être fermées par d'autres longueurs. Ce qui
les garde ouvertes, c'est le coût (des quads moins carrés), la taille demandée, et l'heuristique
qui choisit quels tours fermer.

## 6. Ce que cela a changé dans openclare

Le moteur trouve maintenant les tours de chaque patch, demande à la quantification d'annuler leurs
décalages, répare les quelques-uns que l'arrondi laisse décalés, et essaie huit *variantes* de
choix des tours à retenir, en gardant celle qui laisse le moins de spirales. Après chaque
recherche, il parcourt toutes les bandes du maillage (le parcours complet de la section 3), ajoute
les anneaux que le champ aurait fermés, et cherche à nouveau. Enfin il vérifie qu'aucun quad ne se
retourne, et se rabat sur d'autres longueurs si c'est le cas.

Les chiffres, tous **mesurés** sur le banc d'essai, sont des parts d'arêtes situées sur des boucles
en spirale (plus c'est bas, mieux c'est). La part d'un seul modèle est chaotique : changer le
nombre de quads de 1 % la déplace autant qu'une vraie correction. Les changements sont donc jugés
sur 12 modèles à trois budgets, 2 970, 3 000 et 3 030 quads.

![Part d'arêtes en spirale : le lot de 12 modèles et le corps masculin](../../docs/glossary/spiral_results.png)

*La première version de la fermeture et aujourd'hui, sur le lot de 12 modèles à trois budgets et
sur le corps masculin.*

- **Lot de 12 modèles.** Sans aucune fermeture, 16,7 % des arêtes étaient sur des spirales (à
  3 000 quads) ; la première version de la fermeture l'a ramené à 6,0 %. Sur trois budgets, la
  première version laisse 7,55 % et le moteur d'aujourd'hui **4,54 %**, avec 36,3 % des arêtes
  sur des boucles fermées au lieu de 34,1 %, aucun maillage invalide, et une distance moyenne à la
  source plus grande de 1,2 %.
- **Corps masculin, 10 590 quads.** Avec le moteur d'aujourd'hui, 26,8 % sans fermeture et
  **1,95 %** avec ([figures de l'article](../../docs/research/closing-loops/figures/README.md), un seul passage) ; le maillage fait à la main a
  2,55 %. Les moteurs plus anciens donnaient 12,1 % sans fermeture, 2,47 % avec la première version
  de la fermeture, puis 1,15 % à 1,59 %. Un seul modèle est chaotique : c'est une illustration, la
  preuve est le lot ci-dessus.
- **Cylindre.** **Aucune spirale et 6 patchs** à 2 990, 3 000 et 5 000 quads, contre 26 % à 32 %
  d'arêtes en spirale et environ 700 patchs auparavant.
- **Face aux autres remailleurs** (banc complet de 31 modèles à 3 000 quads, première version de
  la fermeture) : 12,3 % d'arêtes en spirale pour openclare (17,3 % sans fermeture), 21,9 % pour
  QuadriFlow, 25,3 % pour AutoRemesher.
- **Banc de nuit du 18 octobre**, modèles organiques, notes sur 100 (avant fermeture, première
  version, aujourd'hui) : globalement 72,9 → 86,0 → **86,2** à 3 000 quads ; la note des boucles
  43 → 76 → **77** à 3 000 et 33 → 68 → **74** à 1 000. Les grands modèles ont un peu reculé à
  1 000 et 8 000 quads, et un modèle de 373 pièces a régressé ; il a son propre ticket.

**Simplicité.** Un levier de plus est venu du côté de l'artiste plutôt que de la théorie : un
artiste préfère peu de pôles et peu de spirales à des quads parfaitement carrés. Le curseur de
*simplicité*, de 0 à 1, retire les paires 3/5 proches et accepte en échange des quads un peu moins
carrés.

![Le corps masculin remaillé à la simplicité 0, 0,25, 0,5 et 1](../../docs/glossary/simplicity.png)

*Le corps masculin à la simplicité 0, 0,25 (les préréglages), 0,5 et 1 : 303, 259, 213 et 172
pôles.*

## 7. Les portes que nous avons fermées

La recherche, c'est surtout des idées qui échouent. Celles-ci ont échoué, et la note dit pourquoi.

- **Déplacer les pôles, ou détordre le champ cylindre par cylindre,** avant la quantification.
  Puisque les pôles fixent les décalages, pourquoi ne pas les déplacer jusqu'à annuler chaque
  décalage ? Des prototypes du moteur ont essayé les deux, et aucun n'a fait baisser les
  spirales : les décalages de deux tours voisins diffèrent du vecteur de Burgers des pôles
  situés entre eux, si bien que fermer un tour par un changement local reporte son décalage sur
  le suivant. La théorie tient toujours exactement : le champ fixe où les pôles tombent dans la
  carte mise à plat, pas où ils sont sur la surface.
- **Fermer tous les anneaux d'un coup.** Sur le corps masculin, nous avons demandé les 218 tours
  en forme d'anneau (109 bandes) d'une variante. C'est possible en théorie, mais cela force 171
  arcs à une longueur nulle et donne 3 960 quads au lieu d'environ 10 500, avec des quads bien
  moins carrés que ce qu'accepte le moteur (un écart moyen d'au moins 0,817 arête par arc, contre
  un plafond de 0,474). Un sous-ensemble de 190 anneaux tient ; des longueurs entières qui les
  ferment tous les 190 existent et ont été trouvées, mais ce maillage spirale sur 2,5 % de ses
  arêtes contre 1,1 % pour le moteur : fermer certains anneaux déplace les bandes, et d'autres
  boucles spiralent à leur place.
- **« Les conflits sont des paires 3/5 sur une même ligne »** : réfuté, 0 sur 13 (section 4).
  Une première conjecture, « les conflits sont toujours les bords d'une même région autour d'un
  amas de pôles », est elle aussi réfutée telle qu'énoncée.
- **Conjecture 2, « les conflits sont des sommes de dislocations locales ».** Lue strictement
  (escaliers, calottes et lignes qui s'enroulent seulement), elle est **fausse** : deux conflits du
  corps masculin, à l'œil gauche et au pied droit, sont prouvés ne pas être de telles sommes, en
  arithmétique exacte. Lue largement, elle est **vraie mais vide** : presque chaque arc est
  lui-même une minuscule dislocation (2 046 des 2 069 arcs du corps masculin relient deux pôles de
  valence impaire par les lignes qui les bornent), donc toute obstruction peut s'écrire ainsi, ce
  qui n'apprend rien sur les pôles.
- **Choisir les anneaux de façon optimale.** Un solveur exact qui ferme le plus d'anneaux
  possible, en s'assurant que chaque anneau fermé reste la même bande, fournit d'excellents
  candidats supplémentaires : ajoutés à ceux du moteur, ils font passer le lot de 12 modèles de
  4,54 % à **2,32 %** et 2,70 % sur deux essais, jamais pire, alors que tirer simplement trois
  candidats de plus au hasard donne 4,12 % à 4,34 %. Mais cela ne marche que si le solveur
  termine. Avec 5 secondes par résolution, à peu près ce que le moteur peut se permettre, la
  moitié des pièces n'a pas eu de solution et le gain a disparu (4,10 %). Sur le corps masculin,
  chaque tour de résolution spiralait plus que le moteur (3,45 % à 5,59 % contre 1,59 %). Une
  version rapide dans le moteur n'a pas fermé plus d'anneaux qu'avant. Pas adopté, et une leçon :
  fermer le plus d'anneaux n'est pas, à lui seul, le chemin vers moins de spirales.

## 8. Comment savons-nous que c'est vrai

**Des preuves vérifiées par la machine.** Le cœur algébrique de chaque résultat ci-dessus est
démontré en [Lean](../../docs/research/turns-of-a-t-mesh/lean/README.md), un assistant de preuve : un programme qui n'accepte une preuve
que si chaque étape découle des précédentes. Ces preuves n'utilisent que les axiomes standard de Lean, ne laissent
aucune preuve ouverte, et des maillages aléatoires les mettent à l'épreuve ; [la
méthode](research.fr.md#la-méthode) dit comment, et ce qui n'est pas encore fait. Elles couvrent le
décalage comme somme de longueurs, la complétude du parcours, la somme de Burgers et son ordre,
les dislocations et les escaliers, la règle du damier, la parité des anneaux (avec deux preuves
différentes), le décompte des pôles sur une grille, la structure des longueurs entières, les
certificats qui prouvent qu'un groupe de tours ne peut pas se fermer, et les hélices forcées.
Plusieurs d'entre elles valent aussi au-delà des quads, pour les maillages de triangles et
d'hexagones.

**Ce qui est supposé, pas démontré par l'ordinateur.** La topologie : qu'une courbe fermée sur
une sphère la coupe en deux disques, comment une boucle se déplie sur le plan, comment les boucles
autour des pôles engendrent toutes les autres. Chacun de ces faits est écrit comme une hypothèse
explicite du théorème qui l'utilise. Les formaliser prendrait des semaines, et une bonne part
manque à Mathlib. Les résultats les plus récents, sur les surfaces à bords et à anses, sur les
peignes et sur la difficulté de choisir les anneaux, sont démontrés sur papier (et les deux
premiers mesurés), pas encore en Lean.

**Les failles trouvées par l'ordinateur.** Aucune erreur de signe, mais plusieurs failles,
désormais corrigées :

- « genre zéro » devait vouloir dire *fermée* : la règle de parité est fausse avec des bords, ce
  qui a mené à l'anneau de 23 quads autour de l'œil de Suzanne ;
- le théorème des hélices forcées n'a besoin ni de convexité ni de longueurs entières, il vaut
  donc aussi quand le nombre de quads est fixé ;
- pour les équations des rectangles, la seule condition de parité que la note avait trouvée est
  la *seule* obstruction aux solutions entières ;
- quelques hypothèses manquantes dans les preuves (une arête qui doit exister, des bornes entières
  pour des certificats entiers), et une seconde preuve, plus simple, de la règle de parité.

**Des vérifications numériques.** Plus de 30 000 configurations, sur des données réelles : la
règle du damier sur 30 148 paires de tours et 1 436 régions du corps masculin, 13 000
configurations aléatoires, les 1 299 tours
du corps masculin pour la formule générale, 660 tours de modèles à bords. L'arithmétique exacte,
rationnelle ou entière, est utilisée là où un calcul en virgule flottante pourrait tromper.

**Ce que personne n'a vérifié.** Aucun mathématicien humain n'a encore relu les preuves. Les
preuves sur papier et les hypothèses de Lean ont été relues par des agents d'IA et testées sur les
données, ce qui n'est pas la même chose.

## 9. Questions ouvertes

- **Quels tours fermer ?** C'est la question centrale. Fermer des anneaux change les longueurs,
  les longueurs changent les bandes, et les nouvelles bandes sont d'autres parcours. Existe-t-il
  un petit ensemble de tours dont la fermeture garantit que toutes les boucles du maillage se
  ferment ?
- **Des longueurs entières à la vitesse du moteur.** Les solveurs exacts trouvent des longueurs
  entières qui ferment les anneaux demandés, mais en quelques minutes. La quantification peut-elle
  le faire en une fraction de seconde, en tenant exactement le nombre de quads ?
- **Des planchers à taille donnée.** Le théorème des hélices forcées vaut quand le nombre de quads
  est fixé, mais nous ne savons pas encore calculer le plancher dans ce cas.
- **Quand un groupe de tours peut-il se fermer ensemble**, lu directement sur les pôles ? Tout
  conflit est une somme de peignes, mais presque tout l'est, ce qui en dit peu ; et deux conflits
  du corps masculin (l'œil gauche et le pied droit) ne sont prouvablement pas faits d'escaliers et
  de calottes seulement. La réponse utile se trouve quelque part entre les deux.
- **Existe-t-il un algorithme parfait et rapide ? Non** (**démontré**, sauf si P = NP). Choisir
  quels anneaux fermer est au moins aussi difficile qu'une énigme célèbre : dans un réseau de
  personnes, trouver le plus grand groupe dont personne ne connaît personne. Pour tout réseau de
  ce genre, on sait construire un petit patchwork de rectangles dans lequel les groupes d'anneaux
  qui peuvent se fermer ensemble sont exactement ces groupes. Aucune méthode ne peut donc toujours
  trouver le meilleur choix rapidement, sauf si P = NP, une question ouverte depuis 1971 dont la
  plupart des chercheurs pensent que la réponse est non. On ne peut même pas bien l'approcher :
  aucune méthode rapide ne peut promettre de laisser moins d'environ 1,4 fois le plus petit nombre
  possible d'hélices, sauf si P = NP. C'est le pire des cas, sur des patchworks construits exprès ;
  que les patchworks des vrais maillages soient aussi difficiles est une **conjecture**. Sur le
  corps masculin, des solveurs exacts trouvent encore le plus petit nombre d'hélices forcées en
  moins d'une demi-minute quand seul l'écartement des pôles compte ; avec les limites du moteur
  sur la déformation et la taille, ils ne donnent que des fourchettes. Le choix du moteur restera
  une heuristique, et la recherche doit viser de bons choix, pas des choix parfaits.

## 10. Qui a fait quoi

openclare et cette note ont été faits par un auteur humain travaillant avec un système d'IA.
L'auteur humain a choisi le problème et les objectifs : des boucles d'arêtes qui se ferment en
anneaux, la sphère en quads aux huit pôles d'un cube et, en artiste, peu de pôles et peu de
spirales plutôt que des quads parfaitement carrés, toujours uniquement des quads. Il a fixé la
direction et l'ordre du travail, et décidé ce qu'il fallait poursuivre, ce qu'il fallait fusionner
dans le moteur et ce qu'il fallait abandonner. Claude Opus 5.5 (Anthropic), via Claude Code, avec
plusieurs agents travaillant en parallèle, a écrit les changements du moteur et les scripts, mené
les expériences et le banc d'essai, fait les calculs exacts, énoncé et démontré les propositions,
écrit les preuves Lean et rédigé la note et cette page. Aucun système d'IA n'est auteur de la
note ; l'auteur humain, qui la publie, est responsable de son contenu.

## Pour aller plus loin

- L'article A, appliqué : [paper.fr.pdf](../../docs/research/closing-loops/paper.fr.pdf) (en anglais : [paper.pdf](../../docs/research/closing-loops/paper.pdf)), source [paper.fr.tex](../../docs/research/closing-loops/paper.fr.tex).
- L'article B, théorie (en anglais) : [paper.pdf](../../docs/research/turns-of-a-t-mesh/paper.pdf), source [paper.tex](../../docs/research/turns-of-a-t-mesh/paper.tex).
- Les preuves vérifiées par la machine : [lean/README.md](../../docs/research/turns-of-a-t-mesh/lean/README.md).
- Le glossaire, avec toutes les images : [GLOSSARY.fr.md](../../docs/GLOSSARY.fr.md).
- Les images sont dessinées par [make_figures.py](../../docs/glossary/make_figures.py).
