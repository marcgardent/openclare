# Éditer les loops : cahier des charges

*[English version](LOOP_EDITING.md)*

**État :** validé le 27 novembre 2026 ; jalons 1 à 3 écrits (greffon 0.29.0) ; puis, à la demande de Marc, un outil par action, avec Move et Delete avancés et Smooth ajouté (0.30.0) ; les autres pas encore. Le jalon 2 s'écarte des sections 1 et 5 sur trois points, pour l'instant : l'analyse est celle qu'a faite *Analyse*, pas lancée d'elle-même ; avec *Sizes*, la taille unique d'avant elle est le *Largest* ; les arêtes dures ne rendent pas une rangée rouge.

Aujourd'hui, une loop se demande en traçant une courbe : *Draw Curve* ou *Cut Volume* crée un
objet courbe, enfant du maillage et tenu dessus par un Shrinkwrap, que le moteur pose dans le
maillage de travail comme une ligne d'arêtes (`GuideCurve`). L'artiste la modifie comme une
courbe de Blender, par ses points de contrôle, et ne voit qu'après un remesh si le maillage de
quads l'a suivie. Ce cahier des charges décrit un outil qui édite les loops elles-mêmes : les
mettre où l'artiste les veut, sur le modèle, voir les quads le long d'elles pendant le tracé, et
les déplacer jusqu'à ce que le résultat soit juste.

## 1. Le quad et sa taille

Un **quad** est une face à quatre sommets du résultat. Sa **taille** est la longueur de ses
arêtes, `h`.

Le projet de retopologie d'une source demande une taille en chaque point de sa surface,
`h(p)` : celle dont *Show Sizes* colore l'objet et avec laquelle le moteur maille. Elle découle
des réglages du projet, par l'analyse du moteur (`engine.Analysis`) :

- avec *Sizes*, de *Largest*, de *Smallest* et de la forme (`Analysis.at_vertices`) ;
- avec *Quads*, du nombre demandé et de la forme (`Analysis.at_vertices_for`).

`h(p)` se lit aux sommets de la source et se prend entre eux le long de la surface. L'analyse
est gardée tant que la forme et les réglages ne changent pas (`ANALYSES`) : elle se mesure une
fois ; sur une sculpture de millions de faces cela prend des secondes, faites hors de la vue 3D.
Tant qu'elle n'est pas prête, la taille est une seule pour toute la surface, l'arête d'autant de
carrés égaux que de quads demandés, `sqrt(aire / quads)` (décision D1).

`h(p)` est ce que le projet demande, pas ce que le moteur fait : les quads du résultat sont
proches de cette taille, pas de cette taille.

## 2. Ce qu'est une loop

Une **loop** est une ligne sur la surface de la **source** (la sculpture), ouverte ou fermée, le
long de laquelle le maillage de quads doit avoir une ligne d'arêtes. Elle porte les marques
d'une courbe d'aujourd'hui : *Sharp*, *Crease*, *Seam*. C'est une contrainte dure, comme un bord
ou un changement de matériau : un remesh la tient, ou dit, pour cette loop, qu'il ne l'a pas
tenue (section 5).

La loop vit sur la source, parce que le résultat est refait à chaque remesh et que la source est
ce que lit le moteur. Elle est **vue et prise sur les deux** :

- sur la source, dessinée en bande de quads (section 5) ;
- sur le résultat, dessinée de même, et prise sur lui : une loop d'arêtes du maillage de quads
  devient une loop de la source en un clic (geste *Pick*, section 3).

L'outil pose tout point sur la source, même quand l'artiste regarde le résultat : les rayons
partent vers l'arbre de la source, affichée ou non. Les deux surfaces sont proches, le point
tombe sous la souris.

**Stockage (décision D2).** Les loops sont des données du maillage source,
`mesh.openclare_loops`, comme les pôles (`poles.py`) : chaque loop une liste de points dans
l'espace de l'objet, posés sur la surface, `closed`, ses marques et l'état du dernier remesh.
Pas d'objets courbes : dix loops ne font pas dix objets dans l'outliner, aucun Shrinkwrap n'est
évalué contre une sculpture de millions de faces, et l'outil édite des points, pas des poignées
de Bézier. Des points écartés de la surface parce que l'artiste a sculpté ensuite y sont reposés
au remesh suivant (point le plus proche). L'annulation de Blender couvre ces données en Object
Mode : chaque geste est un pas.

Les courbes déjà tracées marchent toujours : le greffon les lit encore (`curves_of`) et les
envoie avec les loops. *Convert Curves to Loops* les change en loops, une par tronçon, marques
gardées.

## 3. Les gestes

Un outil par action, côte à côte dans la barre d'outils (T) en Object Mode, sur le maillage
sélectionné et actif : aucune touche de modification, aucun raccourci à apprendre au-delà de ceux
des outils de Blender (F pour un rayon, Échap pour défaire un glisser). Ce qu'un appui ferait se
voit avant d'être fait : le cercle d'accroche de *Draw Loop*, le cercle de pinceau de *Move
Loop* et *Smooth Loop*, la loop en rouge sous la souris pour *Delete Loop*.

| Outil | Entrée | Ce qu'il fait |
|---|---|---|
| **Draw Loop** | glisser | une nouvelle loop le long du trait, avec tout ce qu'a *Draw Curve* : la corde, le retour le long du trait pour défaire, l'accroche *Ridge*/*Valley*, F pour le rayon. Depuis un bout d'une loop ouverte, la reprend. Lâché avec la souris sur son départ (pas le point au bout de la corde, qui traîne) : fermée ; un anneau autour du départ se remplit tant que lâcher la fermerait. Marques des loops à venir dans l'en-tête. |
| **Ring Loop** | glisser à travers un membre | une loop fermée tout autour, comme la fait *Cut Volume*. Marques dans l'en-tête. |
| **Move Loop** | glisser une loop | le point attrapé suit la souris sur la surface ; les points dans le rayon, le long de la loop, le suivent d'autant moins qu'ils sont loin (l'atténuation de l'édition proportionnelle de Blender) ; la loop reste sur la surface et garde son espacement. |
| **Smooth Loop** | frotter les loops | les points sous le pinceau vont vers la moyenne de la loop autour d'eux, sur la largeur du pinceau, d'autant plus qu'ils sont près de son centre, de la *Strength* de l'en-tête à chaque passage : les courbes aussi larges que le pinceau et plus petites s'en vont, comme sous le pinceau *Smooth* de Blender. Une loop fermée frottée tout autour se resserre un peu, comme un maillage sous ce pinceau. Déplacée ou lissée, une loop est reposée sur la surface entre ses points aussi : un tronçon qui passerait à travers le volume, par-dessus une arête, reçoit les points de la surface sous lui. Les bouts d'une loop ouverte restent. |
| **Delete Loop** | clic sur une loop | la loop s'en va. |
| **Cut Loop** | clic sur une loop | une loop ouverte en devient deux ; une fermée s'ouvre là. Un tronçon s'en va en coupant deux fois et en supprimant le milieu. |
| **Redraw Loop** | glisser d'une loop jusqu'à elle | le tronçon entre les deux endroits est remplacé par le trait. |
| **Pick Loop** | clic sur une arête du résultat | la loop d'arêtes du maillage de quads par cette arête devient une loop : parcourue par les sommets à quatre arêtes, arrêtée à un pôle ou à un bord, fermée si elle revient. Chaque clic au-delà d'un pôle la prolonge jusqu'au suivant. |
| **Join, Close** | un trait ou un Move qui finit sur un bout | un bout lâché sur le bout d'une autre loop joint les deux ; sur le départ de la loop elle-même, la ferme. |

Le pinceau de *Move Loop* et *Smooth Loop* est un cercle autour de la souris avec le nom de
l'outil ; F règle son rayon, la molette le change pendant le glisser ; Échap ou le clic droit
remet les loops en place. Chaque action est une étape d'annulation. Dans le panneau *Loops*,
chaque loop a sa ligne : son compte d'arêtes, ses marques (*Sharp*, *Seam*, *Crease*) réglées
là, et une croix qui la supprime ; *Clear* les supprime toutes.

Les règles de ces gestes (où un trait se joint, ce que Redraw remplace, ce que Cut laisse, où
tombent les quads le long d'une loop) vont dans un module qui ignore Blender, comme `trace.py`
pour un trait, pour être testées sans fenêtre. La surface, l'arbre BVH gardé tant que la forme ne
change pas, la corde, l'accroche, le ruban et le découpeur de *Cut Volume* sont ceux de
`draw.py`, déplacés là où les deux outils les atteignent.

**Décision D3.** Les outils des loops reprennent *Draw Curve* et *Cut Volume* : *Draw Loop* et
*Ring Loop* sont les leurs. Les deux outils quittent la barre d'outils dès que les loops se
tracent (jalon 3), et les objets courbes restent lus.

## 4. Ce qui va au moteur

Rien de neuf d'abord. Une loop est une `GuideCurve` (`points`, `closed`, `sharp`) dans
`Guides::curves`, comme une courbe aujourd'hui ; le crease et le seam restent des marques que le
greffon pose sur le résultat d'après l'indice de la courbe de chaque arête (`curve_edges`). Le
greffon envoie loops et objets courbes dans une seule liste, dans cet ordre, et garde l'indice
de chacun. La taille de la section 1 est celle du moteur ; l'outil la lit et n'envoie rien de
plus.

Deux ajouts au rapport, petits et dans le moteur :

- par courbe, les endroits où la ligne est rompue (`Laid::gaps` compte aujourd'hui pour toutes
  les courbes ; `Laid::curve` connaît déjà la courbe de chaque tronçon) ;
- par courbe, si elle a été lâchée, et pourquoi (`guides_let_go` et `line_causes` comptent
  aujourd'hui des morceaux, pas des courbes).

Ce que le moteur fait d'une loop n'est pas dans ce cahier : tenir toute loop, toujours, est le
travail de #44, #47 et #196 et de la branche `claude/curve-on-edges` (une loop par un sommet du
maillage de travail). La part de l'outil est de ne jamais laisser une loop se perdre sans le
dire.

## 5. Ce qui est montré

**Les quads le long d'une loop.** Chaque loop est dessinée comme une rangée de quads de chaque
côté, posée sur la surface : des arêtes le long de la loop, sur elle et à `h` de part et d'autre,
des arêtes en travers tous les `h`, `h` lu là où elles tombent. L'artiste voit, pendant le
tracé, combien de quads la loop prend et à quelle distance passe la ligne d'arêtes voisine. Pour
une loop fermée, le nombre d'arêtes autour, la longueur divisée par `h` le long d'elle, est écrit
à côté.

Là où les quads ne peuvent pas se poser, ils sont dessinés en rouge :

- un virage plus serré que `h` (la rangée intérieure se replie) ;
- deux loops, ou une loop et un bord ou une arête dure, à moins de `h` (leurs rangées se
  chevauchent) : le moteur y demanderait un quad plus petit, ce que #196 doit demander ;
- un bout ouvert, qui demande un pôle là où la ligne s'arrête, marqué d'un point.

Ces quads sont ce que demande la taille, pas ceux du moteur : après un remesh, ce sont les arêtes
du résultat le long de la loop qui disent si elle a été tenue.

**L'état de chaque loop**, après un remesh, dans le panneau *Loops* (une ligne par loop : nom,
marques, état) et sur la bande : **tenue** (n arêtes le long), **rompue** (k endroits, en points
rouges là où la ligne casse), **lâchée** (tiretée, rouge, avec la cause). Le rapport du remesh
dit combien de loops sont tenues, rompues, lâchées, comme il dit aujourd'hui combien d'arêtes
suivent les courbes.

Les loops sont dessinées à la couleur de leurs marques (`CURVE_COLOURS`).

## 6. Jalons

Un à la fois, chacun sa revue, chacun avec `make version`, `FEATURES.md`, `USE_CASES.md` et la
fiole expérimentale.

1. **Les loops comme données.** `mesh.openclare_loops`, dessinées en lignes sur la source et ses
   résultats ; envoyées au moteur avec les courbes ; *Convert Curves to Loops*. Fini quand, sur
   le cube de #195 et sur l'ogre, une courbe convertie donne les mêmes arêtes que la courbe.
2. **La taille et les quads le long des loops.** `h(p)` de la section 1, par l'analyse, ou
   uniforme en attendant ; les bandes de quads, le compte autour d'une loop fermée et le rouge
   de la section 5. Fini quand, sur une sphère de rayon `r` connu avec une taille demandée, le
   compte autour de l'équateur est `2πr / h`, et que deux loops tracées à moins de `h` sont
   rouges là où elles le sont.
3. **Draw et Ring.** Les traits de *Draw Curve* et *Cut Volume* font des loops ; D3 appliquée.
   Avancé par Marc le 27 novembre 2026, après avoir tracé une courbe là où il voulait une loop.
4. **Pick.** Une loop d'arêtes du résultat faite loop de la source (Delete écrit avec les outils, 0.30.0). Avec
   les états de la section 5 (et les deux ajouts de la section 4) dès ici : une loop prise ne
   vaut que si le remesh suivant dit s'il l'a tenue. Fini quand une loop prise sur un résultat
   de l'ogre est tenue par *Remesh Again*, ou dite non tenue.
5. **Redraw.** Fini quand le module de règles le couvre en tests (Move et Smooth écrits avec les
   outils, 0.30.0).
6. **Cut, Join, Close.** (Les marques d'une loop se règlent dans le panneau depuis 0.30.0.)

## 7. Hors champ, pour plus tard

- Le nombre d'arêtes le long d'une loop demandé (12 autour d'un œil), qui demande le moteur : le
  compte de la section 5 dit ce que donne la taille, il n'en demande pas un autre.
- Le nombre de quads entre deux loops (un anneau d'arêtes), de même.
- Une v2 avec un réglage de plusieurs loops : les rangées de part et d'autre tenues elles aussi, pas seulement la ligne du milieu (demandé par Marc le 27 novembre 2026). D'ici là, seule la loop va au moteur ; les rangées sont ce que demande la taille.
- Une loop tracée d'un côté reflétée de l'autre sous *Symmetry* ; d'ici là, les loops passent
  la coupe comme les courbes.

## 8. Décisions à valider

- **D1** : la taille de quad du projet est le champ de tailles du moteur `h(p)`, uniforme tant
  que l'analyse n'est pas prête (section 1), et non une taille unique réglée à la main pour tout
  le projet.
- **D2** : les loops comme données du maillage, pas des objets courbes (section 2).
- **D3** : les outils des loops remplacent *Draw Curve* et *Cut Volume* au jalon 3 (section 3).
- **D4** : l'ordre des jalons ; Draw d'abord, puis Pick (changé par Marc le 27 novembre 2026 :
  Pick venait avant Draw).
- **D5** : un outil par action, sans touche de modification (Marc, 27 novembre 2026 : « ce n'est pas un pipeline exigeant qui demande plein de raccourcis, surtout que c'est un plugin »). Avant : un seul outil *Loops*, avec Ctrl, Maj et des touches pour ses gestes ; avant encore, Alt, que *Emulate 3 Button Mouse* prend pour tourner la vue.
