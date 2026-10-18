# openclare

<div class="hero">
<p class="lede">La retopologie automatique en quads pour Blender. Un maillage de triangles en entrée, un maillage de quads en sortie, dont les arêtes suivent la forme et dont les boucles d'arêtes se referment autour d'elle.</p>
<p>openclare est un remailleur en quads guidé par un champ : un champ de croix suit la courbure, un T-mesh dispose les patchs, et une quantification entière décide combien de quads vont où. Le moteur est écrit en Rust ; il est livré comme extension Blender, moteur compris, et comme outil en ligne de commande. Logiciel libre, GPL-3.0-or-later.</p>
<p class="actions"><a class="button primary" href="get.fr.md">L'obtenir</a> <a class="button" href="results.fr.md">Voir les résultats</a> <a class="button" href="theory.fr.md">Comment ça marche</a></p>
</div>

## Des boucles qui se ferment

Un artiste sélectionne une boucle d'arêtes d'un Alt-clic et s'attend à ce qu'elle fasse une fois le tour d'un bras et se referme. Les remailleurs automatiques en font souvent une hélice, qui s'enroule de l'épaule au poignet. openclare ferme ces boucles au moment où il décide de la structure entière du maillage, pas après coup.

![Un cylindre remaillé par openclare, avant et après la fermeture de ses boucles](../../docs/glossary/cylinder_result.png)

*Un cylindre, préréglage surface dure. Avant : des boucles en spirale (rouge), environ 700 patchs. Après : chaque boucle est un anneau, et la disposition est celle d'un cube, 6 patchs.*

![Le corps masculin avant et après la fermeture des boucles](../../docs/glossary/male_result.png)

*Un corps humain à 10 590 quads, le nombre de faces du maillage de base qu'un artiste a fait pour lui. Boucles en spirale du jaune au rouge, boucles fermées en bleu.*

## Les chiffres

<div class="stats">
<div class="stat"><span class="value">4,5 %</span><span class="label">des arêtes sur des boucles en spirale, 12 modèles à trois budgets autour de 3 000 quads ; 16,7 % sans fermeture</span></div>
<div class="stat"><span class="value">1,2–1,6 %</span><span class="label">d'arêtes en spirale sur le corps masculin, contre 2,5 % pour le maillage fait à la main (12,1 % avant fermeture)</span></div>
<div class="stat"><span class="value">{{mark:3000:organic:openclare}} / 100</span><span class="label">note moyenne sur {{count:organic}} modèles organiques à 3 000 quads ; AutoRemesher {{mark:3000:organic:autoremesher}}, QuadriFlow {{mark:3000:organic:quadriflow}}</span></div>
<div class="stat"><span class="value">{{mark:3000:mechanical:openclare}} / 100</span><span class="label">note moyenne sur les pièces mécaniques à 3 000 quads</span></div>
<div class="stat"><span class="value">187</span><span class="label">théorèmes de la note de recherche vérifiés par l'assistant de preuve Lean</span></div>
</div>

Mesuré par le banc d'essai du projet, la nuit du 18 octobre 2026, sur {{count:all}} modèles ; la façon dont les notes sont faites est sur la page des [résultats](results.fr.md). Les notes pèsent la validité, l'alignement sur la forme, la forme des quads, la distance au modèle, les sommets irréguliers, la structure de la disposition, les boucles d'arêtes fermées et le nombre de quads demandé.

## Trois remailleurs côte à côte

<figure class="wide"><picture><source media="(prefers-color-scheme: dark)" srcset="../../docs/images/dashboard/gallery-dark.webp"><img src="../../docs/images/dashboard/gallery-light.webp" alt="openclare, QuadriFlow et AutoRemesher sur l'ogre, le cochon, une pièce mécanique et un bâtiment généré, chacun avec son nombre de quads et sa note sur 100" loading="lazy"></picture><figcaption>Chaque ligne au même nombre de quads, demandé à chaque remailleur. Le moulin de pierre vient du <em>Free Windmills Set</em> de LapaModels, CC BY 4.0. Légendes de l'image en anglais.</figcaption></figure>

<figure class="wide"><picture><source media="(prefers-color-scheme: dark)" srcset="../../docs/images/dashboard/marks-dark.svg"><img src="../../docs/images/dashboard/marks-light.svg" alt="Note moyenne sur 100 des trois remailleurs sur les modèles organiques, les pièces mécaniques et les modèles assemblés et générés, à 1 000, 3 000 et 8 000 quads demandés" loading="lazy"></picture><figcaption>Notes moyennes sur le banc à 1 000, 3 000 et 8 000 quads demandés.</figcaption></figure>

## Simple plutôt que régulier

Un artiste préfère peu de pôles et peu de spirales à des quads parfaitement carrés. Un réglage de *simplicité* retire les paires de pôles voisines et accepte en échange des quads un peu moins carrés. openclare ne fait que des quads : jamais de triangle, jamais de n-gone.

![Le corps masculin remaillé à simplicité 0, 0,25, 0,5 et 1](../../docs/glossary/simplicity.png)

*Le corps masculin à simplicité 0, 0,25 (les préréglages), 0,5 et 1 : 303, 259, 213 et 172 pôles.*

## Pour aller plus loin

<div class="cards">
<a class="card" href="theory.fr.md"><strong>Théorie</strong><span>Pourquoi les boucles d'arêtes spiralent, et les règles qui les ferment, en mots simples.</span></a>
<a class="card" href="../../docs/GLOSSARY.fr.md"><strong>Glossaire</strong><span>Pôles, champs de croix, T-mesh, tours et décalages, en images.</span></a>
<a class="card" href="results.fr.md"><strong>Résultats</strong><span>Les notes par budget et par type de modèle, et une page par modèle.</span></a>
<a class="card" href="research.fr.md"><strong>Note de recherche</strong><span>L'article, ses preuves vérifiées par machine, et la façon dont le travail a été fait.</span></a>
<a class="card" href="get.fr.md"><strong>L'obtenir</strong><span>L'extension Blender et la ligne de commande.</span></a>
</div>
