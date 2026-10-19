# Recherche

<p class="subtitle">Ce travail est né d'un projet de programmation, openclare, un remailleur en quads libre, et il est devenu une recherche quand l'ingénierie seule ne suffisait plus à dire quels résultats pouvaient être garantis et quels calculs pouvaient être épargnés.</p>

<article class="paper" aria-labelledby="paper-a">
<div class="paper-text">
<p class="tags"><span class="tag tag-ink">Article A · appliqué</span> <span class="tag tag-teal">Brouillon · non relu par des pairs</span></p>
<h2 id="paper-a">Fermer les boucles d'arêtes à la quantification : contraintes d'hélice pour la quantification d'un T-mesh</h2>
<p>Le remaillage automatique en quads produit souvent des boucles d'arêtes qui s'enroulent en hélice autour d'une partie de la forme au lieu de se refermer en anneaux. Bommes et al. retirent ces <em>q-hélices</em> d'un maillage fini ; nous les empêchons là où se décide la structure entière du maillage, dans la quantification d'un T-mesh.</p>
<p class="actions"><a class="button primary" href="../../docs/research/closing-loops/paper.fr.pdf">PDF · français · 20 p.</a> <a class="button" href="../../docs/research/closing-loops/paper.pdf" hreflang="en" lang="en">PDF · English · 18 p.</a> <a class="button plain" href="../../docs/research/closing-loops/paper.fr.tex">Source LaTeX</a></p>
<p class="note">La version française est une traduction faite au mieux ; la version anglaise fait foi. Son titre anglais : <em lang="en">Closing Edge Loops at Quantization Time: Helix Constraints for T-Mesh Quantization</em>.</p>
</div>
<figure><img src="../../docs/research/closing-loops/figures/male-overview.fr.png" alt="Figure 3 de l'article A : le corps masculin à 10 590 quads demandés, par openclare sans fermeture, par openclare avec fermeture, fait à la main et par QuadriFlow, boucles en spirale en vermillon" loading="lazy"><figcaption>Figure 3 · boucles en spirale en vermillon, boucles fermées en bleu foncé</figcaption></figure>
</article>

<article class="paper" aria-labelledby="paper-b">
<div class="paper-text">
<p class="tags"><span class="tag tag-ink">Article B · théorie</span> <span class="tag tag-hot">En préparation</span></p>
<h2 id="paper-b">Les tours d'un T-mesh : obstructions à la fermeture des boucles d'arêtes</h2>
<p>Le décalage d'un tour lu comme une holonomie de translation ; les conflits qu'imposent les singularités ; les tours que toute quantification dans des bornes laisse décalés ; et pourquoi choisir les tours à fermer est NP-difficile.</p>
<p>Son brouillon est la première note de recherche du projet, qui porte le titre de l'article A et est en cours de réécriture pour devenir cet article. Titre anglais prévu : <em lang="en">Turns of a T-mesh: obstructions to closing edge loops</em>.</p>
<p class="actions"><a class="button" href="../../docs/research/closing-helices/paper.pdf" hreflang="en" lang="en">Brouillon · PDF · anglais · 51 p.</a> <a class="button plain" href="../../docs/research/closing-helices/paper.tex">Source LaTeX</a></p>
</div>
</article>

## Des preuves vérifiées par machine

<!-- section: night -->

<p class="lead-number"><b>{{theorems}}</b><span>théorèmes en Lean 4 + Mathlib</span></p>

Les cœurs algébriques des résultats de la note (ses sections 3 et 8 à 10) sont vérifiés par l'assistant de preuve [Lean 4](https://lean-lang.org/) avec la bibliothèque Mathlib : **{{theorems}} théorèmes**, dans un projet Lake à côté de la note. Chaque théorème ne dépend que des trois axiomes standard de Lean (`propext`, `Classical.choice`, `Quot.sound`) ; il n'y a aucun `sorry`.

**Ce qui est vérifié.** Entre autres : le décalage d'un tour est une forme linéaire des longueurs d'arcs ; la marche complète des bandes trouve chaque bande ; Gauss–Bonnet pour les tours ; la somme de Burgers et l'ordre de première visite ; les anneaux, les dislocations, les escaliers et les échanges de deux sommets ; la règle du damier et la parité d'un tour sur une sphère (avec une seconde preuve, née de la formalisation) ; la formule de Fox–Burgers ; le lemme des poignées de main et la proposition des « arcs libres » ; des certificats de Farkas pour les tours qui ne peuvent jamais se fermer ; et le théorème des hélices forcées. Plusieurs résultats sont prouvés plus généralement que la note ne les énonce : pour des rotations et des dimensions quelconques, et pour des champs de tout ordre (maillages de triangles et d'hexagones).

**Ce qui est supposé.** Les faits topologiques sur lesquels la note s'appuie ne sont pas formalisés ; chacun est une hypothèse explicite du théorème qui l'utilise, jamais cachée. Parmi eux : qu'une courbe fermée simple sur une sphère borde un disque (Jordan–Schoenflies), l'application développante et l'holonomie comme morphisme du groupe fondamental, et deux faits classiques sur les graphes. Les bandes sur les surfaces à bords, et tout énoncé *mesuré*, sont hors de la formalisation.

**Ce qu'elle a trouvé.** Formaliser la note a trouvé onze lacunes ou corrections, dont aucune erreur de signe ou de convention : un facteur mal placé dans une étape du calcul de Fox (la conclusion restant juste), un théorème qui n'a besoin ni de convexité ni de longueurs entières, un énoncé de parité qui ne vaut que sur une sphère fermée (il tombe en défaut avec deux bords ou plus), et des propositions plus fortes qu'énoncées. Elles sont listées dans le [README des preuves](../../docs/research/closing-helices/lean/README.md) (en anglais).

Pour vérifier les preuves vous-même, depuis la racine du dépôt :

```sh
make proofs NOTE=closing-helices
```

Le premier lancement télécharge la chaîne d'outils de Lean et Mathlib déjà compilée, environ 10 Go en tout ; ensuite, construire les fichiers propres à la note prend environ 40 secondes et 1,3 Go de mémoire.

## Assistance par IA

Les deux articles et le logiciel qu'ils décrivent ont été faits par un auteur humain travaillant avec un système d'IA, qui n'en est pas auteur. Suivant la pratique recommandée par l'ACM et par Nature, chaque article dit comment le travail s'est partagé.

**L'auteur humain** a choisi le problème et ses buts : des boucles d'arêtes qui se ferment en anneaux, la sphère en quads aux huit pôles d'un cube comme cible et, en artiste, peu de sommets singuliers et peu de spirales plutôt que des quads parfaitement carrés, des quads seulement. Il a fixé la direction et l'ordre du travail, décidé quels résultats poursuivre et quels changements fusionner dans le moteur, autorisé les outils (LaTeX, Lean et Mathlib), et jugé les résultats à travers les mesures, les images et les brouillons qui lui ont été présentés.

**Le système d'IA**, Claude Opus 5.5 (Anthropic), utilisé via Claude Code en octobre 2026 avec plusieurs sous-agents en parallèle, a fait le reste : les changements du moteur et les scripts d'expérience, les expériences et le banc, les calculs exacts, la recherche bibliographique, les énoncés et les preuves, la formalisation en Lean et les brouillons du texte. L'existence des références a été vérifiée, et chaque entrée des bibliographies dit quelle part de l'ouvrage a été lue.

**Ce qui a été vérifié, et comment.** Les cœurs algébriques sont vérifiés par machine en Lean, avec les seuls axiomes standard de Lean. Les hypothèses topologiques que Lean prend pour acquises, et les preuves données seulement sur papier, ont été vérifiées par des agents d'IA qui les ont relues et testées sur les données, pas par un expert humain. Chaque chiffre mesuré peut être reproduit à partir des scripts et des branches que les articles nomment. Aucun des deux articles n'a été relu par des pairs ; des erreurs restent possibles, et l'auteur, qui les publie, est responsable de leur contenu.

Ce site a été fait de la même façon : ses pages ont été rédigées par le système d'IA et publiées par l'auteur du projet.
