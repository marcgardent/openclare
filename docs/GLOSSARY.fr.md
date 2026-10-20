# Glossaire

*[English version](GLOSSARY.md)*

Traduction française de [GLOSSARY.md](GLOSSARY.md). Les
objets géométriques et les termes d'openclare, de son banc d'essai et des deux articles de recherche,
[Closing Edge Loops at Quantization Time](research/closing-loops/paper.fr.tex) et
[Turns of a T-Mesh](research/turns-of-a-t-mesh/paper.tex). Le terme anglais suit chaque entrée entre parenthèses.
Les noms en `code` sont ceux du code source du moteur.

Les entrées suivent la chaîne : le maillage, le champ, la paramétrisation, le découpage, la
quantification, les boucles, puis la théorie des tours et les mesures.

Les images sont dessinées par `docs/glossary/make_figures.py`. Celles qui montrent un maillage de
quads avec des pôles sont calculées, pas dessinées à la main : les décalages qu'on y lit sont
mesurés en parcourant les boucles d'arêtes de ce maillage.

## 1. Maillages

**Maillage triangulaire** (*triangle mesh*). L'entrée : une surface faite de triangles, comme une
sculpture ou un scan.

**Maillage de quads** (*quad mesh*). La sortie : une surface faite uniquement de quadrilatères.
openclare ne produit jamais de triangles ni de n-gones.

**Valence** (*valence*). Le nombre d'arêtes qui se rejoignent en un sommet.

**Sommet régulier** (*regular vertex*). Un sommet intérieur de valence 4 dans un maillage de
quads, ou de valence 3 sur un bord.

**Sommet irrégulier, singularité, pôle** (*irregular vertex, singular vertex, pole*). Un sommet
intérieur dont la valence n'est pas 4. En pratique il est de valence 3 ou 5, rarement 2 ou 6. Le
flux des arêtes tourne autour de lui. Les artistes appellent ces sommets des pôles.

![Un sommet régulier, un pôle de valence 3 et un pôle de valence 5](glossary/valence_poles.png)

*Quatre arêtes se rejoignent en un sommet régulier ; trois ou cinq en un pôle, où le flux des
arêtes tourne.*

**Indice** (*index*). 4 moins la valence : +1 pour la valence 3, −1 pour la valence 5. Sur une
surface fermée, les indices ont pour somme 4 × (2 − 2g), où g est le genre (Gauss–Bonnet). Sur
une sphère, leur somme vaut 8 : un maillage de quads d'une sphère a donc au moins huit pôles de
valence 3.

**Genre** (*genus*). Le nombre d'anses d'une surface fermée : 0 pour une sphère, 1 pour un tore.

**Surface fermée, bord** (*closed surface, border*). Une surface sans arête de bord, comme une
balle. Suzanne et jerry ont des bords. Plusieurs théorèmes de la note ne valent que sur les
surfaces fermées de genre 0.

**Pièce** (*piece*). Une partie connexe de l'entrée. Les pièces sont maillées séparément et se
partagent le budget de quads.

**Maillage de travail** (*work mesh*). L'entrée remaillée en triangles réguliers avant toute autre
étape. Toutes les étapes suivantes calculent dessus.

**Complexe de base** (*base complex*). Le découpage d'un maillage de quads en les plus grands blocs
rectangulaires de quads, bordés par les lignes qui partent des pôles. Moins il y a de patchs,
plus la structure est simple. Une sphère en forme de cube en a 6.

**Boucle d'arêtes** (*edge loop*). Une ligne d'arêtes qui passe tout droit à chaque sommet
régulier. Elle s'arrête à un pôle ou à un bord, ou revient à son point de départ.

**Bande de quads** (*strip, quad strip*). Une rangée de quads reliés par leurs arêtes opposées :
les quads entre deux boucles d'arêtes voisines.

**Anneau, boucle fermée** (*ring, closed loop*). Une boucle d'arêtes ou une bande qui revient à son
départ et se referme sur elle-même.

**Spirale, hélice** (*spiral, helix*). Une boucle d'arêtes ou une bande qui revient à côté de son
départ, une rangée ou plus plus loin, et s'enroule autour de la partie comme le filet d'une vis.
C'est le principal défaut que combat openclare. Bommes et al. l'appellent une *q-helix*.

![Un anneau et une hélice sur un tube, et le tube ouvert à plat](glossary/ring_helix.png)

*Un anneau revient à son départ ; une hélice revient une rangée plus haut et continue de s'enrouler
comme un filet de vis. En bas, le même tube ouvert et mis à plat.*

![Un cylindre remaillé par openclare, avant et après la fermeture des boucles](glossary/cylinder_result.png)

*openclare sur un cylindre : avant, les boucles spiralent (rouge) ; après leur fermeture, chaque
boucle est un anneau (bleu).*

**Boucle ouverte** (*open loop*). Une boucle d'arêtes qui se termine à un pôle ou à un bord.

**Face retournée, pli** (*inverted face, fold*). Un quad orienté à l'inverse de la surface source.
Un seul suffit à rendre le maillage invalide.

**Face dégénérée** (*degenerate face*). Une face d'aire nulle.

## 2. Le champ de croix

**Directions principales de courbure** (*principal curvature directions*). En chaque point d'une
surface, les deux directions dans lesquelles elle se courbe le plus et le moins.

**Champ de croix** (*cross field*). En chaque point, une croix à quatre bras à angle droit qui
indique dans quel sens doivent courir les arêtes des quads. On l'appelle aussi champ 4-RoSy
(symétrie de rotation d'ordre quatre).

**Champ N-symétrique** (*N-RoSy field*). La même idée avec N bras. Un champ 6-RoSy, une étoile à
six bras, donne des maillages triangulaires et hexagonaux (DT-068).

**Champ en puissance** (*power field*). La façon dont openclare calcule le champ de croix (Knöppel
et al. 2013) : la croix s'écrit comme le nombre complexe e^{4iθ}, et le champ le plus lisse qui
suit aussi la courbure se résout comme un système linéaire.

**Confiance** (*confidence*). L'écart entre les deux courbures principales en un point. Là où
elles sont presque égales, comme sur une sphère, leurs directions ne veulent rien dire et ne sont
pas suivies (le plancher de confiance, `CONFIDENCE_FLOOR`).

**Singularité du champ** (*singularity of the field*). Un point autour duquel la croix tourne d'un
quart de tour ou plus. Elle devient un pôle du maillage.

![Un champ de croix qui suit une courbure, et une singularité](glossary/cross_field.png)

*À gauche : les croix suivent la courbure de la forme. À droite : en faisant une fois le tour d'une
singularité, la croix revient tournée d'un quart de tour ; trois lignes en partent et elle
devient un pôle de valence 3.*

**Saut de période** (*period jump*). Le nombre de quarts de tour dont la croix est renumérotée en
traversant une arête du maillage de travail.

**Annulation de paires, portée** (*pair cancellation, singularity reach*). Supprimer une singularité
de valence 3 et une de valence 5 proches l'une de l'autre, pour simplifier le maillage.
`singularity_reach` fixe à quelle distance elles peuvent être.

## 3. La paramétrisation

**Paramétrisation sans couture, à grille entière** (*seamless map, integer-grid map*). Une mise à
plat de la surface dans le plan, dans laquelle le champ de croix devient les directions
horizontale et verticale. À travers une coupure, elle ne change que d'un quart de tour et d'une
translation.

**Couture** (*seam*). Une coupure de la surface le long de laquelle la paramétrisation est ouverte à
plat.

**Point conique** (*cone point*). Un sommet singulier vu dans la paramétrisation à plat : l'angle
autour de lui est de 270° pour la valence 3 et de 450° pour la valence 5, au lieu de 360°.

**Métrique plate à singularités coniques** (*flat metric with cone points*). Un maillage de quads
vu comme une surface faite de carrés unité : plate partout sauf en ses pôles.

**Application développante** (*developing map*). La mise à plat obtenue en dépliant la surface
patch après patch dans le plan, le long d'un chemin. Des chemins différents peuvent ramener des
positions différentes.

**Holonomie** (*holonomy*). Ce que ramène un chemin fermé une fois déplié : une rotation d'un
nombre de quarts de tour et une translation. La translation d'un tour est son décalage.

## 4. Le découpage

**Séparatrice** (*separatrix*). Une ligne du champ qui part d'une singularité : 3 lignes partent
d'un sommet de valence 3 et 5 d'un sommet de valence 5. Les séparatrices deviennent des boucles
d'arêtes qui se terminent à des pôles.

**Trace** (*trace*). Une séparatrice suivie à travers la surface jusqu'à ce qu'elle rencontre une
autre trace ou un bord. On l'appelle *motocyclette* parce qu'elle s'arrête là où elle en heurte une
autre, comme les motos du film Tron.

**Fusion frontale** (*head-on merge*). Deux traces qui se rencontrent de face sont réunies en une
seule, au lieu que l'une s'arrête sur l'autre.

**T-mesh, maillage en T** (*T-mesh*). Le découpage formé par les traces : une partition de la
surface en rectangles dont les coins peuvent se trouver au milieu du côté d'un autre rectangle.

**Patch** (*patch*). Un rectangle du T-mesh. Il devient une grille de quads.

**Nœud** (*node*). Un point où des traces se rencontrent ou s'arrêtent : un sommet singulier, ou
une jonction en T.

**Jonction en T** (*T-junction*). Un nœud où une trace s'arrête sur une autre, si bien qu'il est un
coin de certains patchs et se trouve au milieu d'un côté d'un autre.

**Arc** (*arc*). Un morceau de trace entre deux nœuds : un côté, ou une partie d'un côté, d'un
patch. La quantification donne à chaque arc une longueur entière.

![Séparatrices, traces, nœuds, arcs et patchs d'un petit maillage de quads](glossary/tmesh.png)

*Les lignes qui partent des pôles courent jusqu'à heurter une autre ligne ou le bord, et découpent
la surface en rectangles de quads, les patchs. Calculé sur un petit maillage de quads avec deux
paires 3/5.*

**Mur** (*wall*). Un bord ou une arête vive que les traces ne peuvent pas traverser.

**Maillage par grille de patchs** (*patch grid meshing*). Transformer chaque patch, une fois que ses
côtés ont des longueurs entières, en sa grille de quads.

## 5. Quantification

**Quantification** (*quantization*). Choisir un nombre entier d'arêtes de quad pour chaque arc du
T-mesh, de sorte que les patchs se pavent de quads. C'est là que se décide la structure entière du
maillage, et avec elle ses boucles.

**Équilibre** (*balance*). Pour chaque patch, les deux côtés opposés doivent avoir la même longueur
totale. Chaque arc apparaît dans exactement deux lignes d'équilibre, avec un signe chacune, ce qui
fait de la matrice d'équilibre la matrice d'incidence d'un graphe bidirigé.

![Longueurs réelles, arrondi simple et quantification de trois patchs](glossary/quantization.png)

*Le champ demande des longueurs réelles ; arrondir chacune seule casse l'équilibre du patch de
gauche ; la quantification choisit des longueurs entières telles que les côtés opposés
s'égalent et que les quads tombent juste.*

**Longueurs visées, écart** (*target lengths, deviation*). Les longueurs réelles que demande le
champ, et l'écart entre elles et les longueurs entières choisies. Plus l'écart est grand, moins les
quads sont carrés.

**Plafond d'écart** (*deviation cap*). Le plus grand écart accepté pour fermer des boucles, par
rapport à la recherche simple : `DEVIATION_GROWTH` × simple + `DEVIATION_SLACK`. Le réglage de
simplicité le déplace.

**Recherche simple** (*plain search*). La recherche entière sans fermeture de boucles, à la manière
du flot d'écart minimal de Bi-MDF (Heistermann et al. 2023).

**Projection** (*projection*). Le plus petit changement des longueurs réelles qui ferme les tours
choisis, résolu par moindres carrés.

**Recherche entière** (*integer search*). La recherche de longueurs entières proches des longueurs
projetées qui respectent les équilibres.

**Variante** (*way*). L'une des huit façons de choisir quels tours fermer (`CLOSE_WAYS`). Elles sont
toutes essayées, et on garde celle qui laisse le moins de spirales.

**Marche de réparation, réparation visée** (*repair walk, aimed repair*). De petits changements des
longueurs entières qui tentent de fermer une hélice restante.

**Tour de coupe** (*cut round*). Après une recherche, on parcourt chaque bande des longueurs
entières, on ajoute aux tours à fermer les hélices que le champ aurait fermées, et on relance la
recherche.

**Ensemble actif** (*active set*). Dans la projection, les arcs qui sortiraient avec une longueur
négative sont maintenus à zéro et le système est résolu à nouveau.

**Candidat** (*candidate*). Un jeu admissible de longueurs entières, classé selon ses spirales.
Quand le jeu choisi produit un pli au maillage, on essaie les suivants avant les longueurs
simples.

**Garde contre les plis** (*fold guard*). La vérification qui maille d'autres longueurs quand celles
choisies replient un quad.

**Simplicité** (*simplicity*). Le réglage de 0 (quads les plus réguliers) à 1 (le moins de pôles et
de boucles). Les préréglages sont à 0,25. Il pilote la portée des singularités et le plafond
d'écart.

![Le corps masculin remaillé à la simplicité 0, 0,25, 0,5 et 1](glossary/simplicity.png)

*Le corps masculin à la simplicité 0, 0,25, 0,5 et 1 : le nombre de pôles baisse quand la
simplicité augmente ; les boucles en spirale sont dessinées du jaune au rouge.*

## 6. Les tours : la théorie de la note de recherche

**Tour** (*turn*). Une marche en ligne droite à travers les rectangles du T-mesh qui revient sur le
côté d'où elle est partie, dans le même sens. La bande de quads qu'elle suit est soit un anneau,
soit une hélice.

**Décalage** (*shift*). De combien, le long de son côté de départ, un tour revient décalé : 0 pour
un anneau, le pas de l'hélice sinon. C'est une forme linéaire des longueurs des arcs
(Proposition 1).

**Pas** (*pitch*). Le nombre de rangées dont une hélice avance à chaque tour : son décalage aux
longueurs entières.

![Un tour à travers les patchs d'un tube, qui revient avec un décalage 0 puis 1](glossary/turn_shift.png)

*Un tour marche tout droit à travers les patchs autour d'un tube et revient dans son premier patch :
à son départ, la bande est un anneau (décalage 0) ; une rangée plus haut, c'est une hélice de pas 1
(décalage 1).*

**Longueur du tour** (*around, along length*). La longueur de la marche d'un tour, sa composante le
long de la marche.

**Inclinaison** (*lean*). Le rapport entre le décalage réel d'un tour et sa longueur. Un tour que le
champ ferme presque (inclinaison ≤ 0,12) est un anneau du champ.

**Anneau du champ** (*ring of the field*). Un tour que le champ lui-même fermerait presque : il
devrait finir en anneau.

**Hélice indue** (*undue helix*). Un anneau du champ que les longueurs entières ont laissé en
hélice : c'est l'arrondi qui l'a créée, pas le champ.

**Station, départ à mi-entier** (*station, half-integer start*). Les milieux des arêtes de quad le
long d'un côté de patch (k + ½). Partir de tous trouve chaque bande du maillage (Proposition 2,
complétude).

**Marche complète** (*lattice walk, complete walk*). Partir de chaque station de chaque côté de
patch aux longueurs entières, ce qui liste toutes les bandes du maillage final.

**Vecteur de Burgers** (*Burgers vector*). Emprunté aux dislocations des cristaux : la translation
dont une boucle autour de quelques sommets singuliers manque de se fermer. Le décalage d'un tour
est la composante transverse du vecteur de Burgers des sommets singuliers d'un de ses côtés.

![Le circuit de Burgers dans un cristal et autour d'une paire 3/5 dans un maillage de quads](glossary/burgers.png)

*À gauche : dans un cristal avec une demi-rangée d'atomes en trop, 6 pas à droite, en haut, à
gauche et en bas manquent le départ d'un pas, le vecteur de Burgers. À droite : la même marche
autour d'une paire 3/5 d'un maillage de quads le manque d'une arête en travers et d'une en long.
Les deux marches sont calculées.*

**Dislocation, paire 3/5** (*dislocation, 3/5 pair*). Un sommet de valence 3 et un de valence 5
proches l'un de l'autre. Entre deux tours parallèles, une telle paire rend leurs décalages
différents de ±d₁±d₂, où d est l'écartement de la paire.

**Escalier** (*staircase*). Un chemin d'arcs qui suit deux directions à un quart de tour l'une de
l'autre, entre les deux sommets d'une paire. La différence des décalages des deux tours qui
l'entourent est la longueur de l'escalier : les deux ne peuvent être des anneaux que si la paire
fusionne.

![Deux tours d'un tube avec une paire 3/5 entre eux et l'escalier qui la relie](glossary/staircase.png)

*Un tube ouvert avec une paire 3/5 entre deux tours : le tour 1 est un anneau, le tour 2 revient
2 rangées plus haut, et la différence de leurs décalages est la longueur de l'escalier qui va du 5
au 3. Décalages mesurés sur le maillage.*

**Échange de deux sommets** (*exchange of two vertices*). Deux sommets singuliers de même valence
situés de part et d'autre de deux tours qui se croisent. Il agit sur les décalages comme une
dislocation.

**Calotte** (*cap*). Un disque fermé par un tour, comme un bout de doigt, qui contient des sommets
singuliers dont les indices ont pour somme 4, typiquement quatre de valence 3.

**Séparatrice qui s'enroule** (*winding separatrix*). Une séparatrice qui fait une fois le tour d'un
membre et retombe juste à côté de son propre sommet. Le tour autour du membre est alors tenu
ouvert par exactement cet écart, comme au poignet du corps masculin.

![Une séparatrice qui fait une fois le tour du poignet et retombe à côté de son propre pôle](glossary/winding_separatrix.png)

*Une séparatrice part d'un pôle de valence 5, fait une fois le tour du poignet et retombe sur une
autre ligne du même pôle, juste à côté de lui ; le tour autour du poignet revient décalé de cet
écart.*

**Gauss–Bonnet pour les tours** (*Gauss–Bonnet for turns*). Un tour simple qui borde un disque
entoure des singularités d'indice total 4. Deux tours disjoints qui bordent un anneau (une
couronne) en entourent un total de 0.

**Parité d'un tour** (*parity of a turn*). Sur une sphère fermée, la longueur plus le décalage d'un
tour est pair aux longueurs entières. Tout anneau a donc un nombre pair de quads, et une hélice de
pas impair ne peut se fermer que si sa longueur change. C'est faux sur les surfaces à bords.

![Un anneau sur une sphère fermée aux coins en deux couleurs, et un anneau de 5 quads autour d'un trou](glossary/parity.png)

*Sur une surface fermée, les coins des quads prennent deux couleurs, voisins toujours différents,
donc chaque anneau a un nombre pair de quads ; autour d'un trou, comme un œil, les couleurs se
heurtent et un anneau de 5 quads est possible.*

**Règle du damier** (*checkerboard rule*). Colorier les points entiers du plan en damier. Deux tours
parallèles ne peuvent être tous deux des anneaux que si un nombre pair de sommets singuliers de
valence impaire situés entre eux tombent sur des points blancs.

![La règle du damier pour une paire 3/5 entre deux tours](glossary/checkerboard.png)

*Un quart de tour autour d'un point entier garde les couleurs ; entre deux tours, si un nombre
impair des pôles de valence 3 ou 5 tombent sur des points blancs, les deux tours ne peuvent pas
être tous deux des anneaux.*

**Enroulement tordu, formule de Fox–Burgers** (*twisted winding, Fox–Burgers formula*). Toute
combinaison de décalages est une somme, sur les sommets singuliers, d'un enroulement à deux
composantes multiplié par leur terme de Burgers. Les enroulements viennent du calcul différentiel
libre de Fox. Les simples nombres d'enroulement ne suffisent pas.

**Tour de signe fixe** (*sign-definite turn*). Un tour dont le décalage garde le même signe quelles
que soient les longueurs. Il ne peut jamais se fermer sans écraser un arc : il ne faut donc pas
chercher à le fermer.

**Certificat de Farkas** (*Farkas certificate*). Une combinaison de décalages qui prouve qu'un
ensemble de tours ne peut pas se fermer en entier : elle est égale à une somme positive de
longueurs qui contient un arc maintenu à une longueur de 1 ou plus.

**Ensemble infaisable irréductible** (*irreducible infeasible set*). Un ensemble de tours qui ne
peuvent pas tous se fermer, alors que tout sous-ensemble plus petit le peut.

**Zéro implicite** (*implicit zero*). Un arc que toute solution réelle d'un système de fermeture met
à zéro. L'écraser rétrécit le maillage.

**Réseau des quantifications fermées** (*lattice of closed quantizations*). Toutes les longueurs
entières qui respectent les équilibres et ferment un ensemble de tours : un réseau entier, ker_ℤ.

**Forme normale de Hermite, LLL, Babai** (*Hermite normal form, LLL, Babai*). Des outils de la
géométrie des nombres : une base exacte d'un réseau, une base faite de vecteurs courts, et
l'arrondi vers un point proche du réseau.

**Hélices forcées** (*forced helices*). Le plus petit nombre de tours que toute quantification doit
laisser décalés, étant donnés les singularités et leur écartement. Sur le corps masculin il y en a
1 à 9 par variante, là où le moteur en laisse 13 à 123.

**Chaîne de dislocations** (*chain of dislocations*). Des tours parallèles emboîtés, séparés par des
dislocations de même orientation. m + 1 tels tours imposent au moins m hélices.

![Quatre tours d'un membre séparés par trois paires 3/5, de décalages 0, 2, 4 et 6](glossary/forced_helices.png)

*Trois paires 3/5 de même orientation le long d'un membre : chacune ajoute la longueur de son
escalier au décalage, si bien que les décalages montent (ici 0, 2, 4, 6) et, dans toute
quantification qui garde les paires écartées, au plus un des quatre tours peut être un anneau.
Décalages mesurés sur le maillage.*

**Différentielle de Jenkins–Strebel, torsion d'un cylindre** (*Jenkins–Strebel differential,
cylinder twist*). L'image classique d'une surface découpée en cylindres plats. Une hélice est un
cylindre dont la torsion n'est pas un nombre entier.

## 7. Les mesures du banc

**Arêtes en spirale** (*spiral loop edges*). La part des arêtes qui sont sur des boucles en spirale.
Plus c'est bas, mieux c'est. La valeur d'un seul modèle est chaotique, donc les changements se
jugent sur trois budgets.

![Le corps masculin avant et après la fermeture des boucles](glossary/male_result.png)

*Le corps masculin avant et après la fermeture des boucles : la plupart des spirales (du jaune au
rouge) deviennent des anneaux (bleu).*

**Arêtes en boucle fermée** (*closed loop edges*). La part des arêtes sur des boucles fermées. Plus
c'est haut, mieux c'est.

**Arêtes en hélice** (*helix loop edges*). La part des arêtes sur des hélices de tout pas : des
boucles qui longent leur propre tour suivant, dans le même sens, sur un tour entier, à quelque
nombre de quads que ce soit. Plus c'est bas, mieux c'est. Contrairement aux arêtes en spirale, le
moteur ne choisit pas ses maillages selon elle, et le pas n'est pas borné ; une boucle qui ne
revient le long d'elle-même que sur moins d'un tour n'y compte pas. La mesure donne aussi les
hélices de pas supérieur à 4, que le compte des spirales manque, et la part de chaque pas.

**Distance de Hausdorff** (*Hausdorff distance*). La plus grande distance entre le résultat et la
source, en millièmes de la diagonale. Elle est chaotique pour un seul modèle.

**Distance moyenne** (*mean distance*). La distance moyenne entre le résultat et la source.

**Distance des parties fines** (*thin-part distance*). La distance sur les parties fines (doigts,
oreilles, feuilles), en arêtes de quad.

**Arêtes vives gardées** (*sharp edges kept*). La part de la longueur des arêtes vives de la
source le long de laquelle le résultat a lui-même une arête vive. Plus c'est haut, mieux c'est.
Seuls comptent les plis qui se voient encore à la taille des quads : les triangles se coupent à
plus de 30°, la surface tourne encore d'autant sur une demi-arête de quad autour, et surtout à
l'arête. Le grain d'une sculpture ne compte pas, ni un tube de quelques quads de tour ; deux plis
à moins d'un demi-quad l'un de l'autre comptent pour un. Là où la source en a moins de 10 arêtes
de quad, comme une sculpture lisse, il n'y a pas de valeur.

**Rapport d'aspect** (*aspect*). À quel point les quads sont allongés : 1 veut dire carré.

**Écart d'angle** (*angle deviation*). L'écart entre les coins des quads et 90°.

**Alignement** (*alignment*). L'écart entre les arêtes et les directions principales de courbure.

**Écart au budget** (*budget error*). L'écart entre le nombre de quads produits et le nombre demandé.

**Validité** (*validity*). Fermé là où la source est fermée, variété (manifold), sans face retournée
ni dégénérée.

**QuadSphere** (*QuadSphere*). La réponse académique pour une sphère : le découpage d'un cube, avec
huit pôles de valence 3, six patchs et toutes les boucles fermées.

![La QuadSphere : un cube gonflé en sphère](glossary/quadsphere.png)

*La QuadSphere : un cube gonflé en sphère, avec 8 pôles de valence 3 à ses coins (leurs indices ont
pour somme 8), 6 patchs et chaque boucle un anneau.*

**Lot sur trois budgets** (*three-budget batch*). Les 12 modèles de la comparaison rapide, chacun
remaillé à 2 970, 3 000 et 3 030 quads, pour qu'un changement ne soit pas jugé sur un seul essai
chaotique.
