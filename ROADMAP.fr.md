# Feuille de route

*[English version](ROADMAP.md)*

Ce qui reste à faire, fonction par fonction. Ce que fait chaque fonction et jusqu'où on peut s'y
fier est dans [Fonctions](docs/FEATURES.fr.md) ; les tickets sont les
[issues](https://github.com/marcgardent/openclare/issues), citées par leur numéro.

Réécrite le 31 octobre 2026 à partir des fonctions telles qu'elles sont. La règle du 8 octobre
2026 tient : ce qui améliore le maillage passe avant les fonctions annexes.

## 1. D'expérimental à stable

Chaque ligne est faite quand sa fonction remplit la règle de [FEATURES.fr.md](docs/FEATURES.fr.md) :
testée, juste sur le banc ou sur un cas connu, plus aucune issue *high* ou *medium* ouverte
contre elle. Dans cet ordre :

| Ordre | Fonctions | Fait quand | Issues |
|---|---|---|---|
| 1 | *Ask For: Sizes*, *Analyse*, *Show Sizes*, *Density from Shape* ; `--largest`, `--smallest`, `analyse` | les tailles proposées viennent de l'utilisateur ou d'une règle mesurée, pas d'une constante ; les doigts d'un corps gardent autour d'eux les quads que demande *Quads Around Thin Parts* | #253 |
| 2 | *Draw Curve*, *Cut Volume*, les marques d'une courbe ; `--curves` | une courbe tracée là où des quads peuvent la suivre est gardée, à chaque fois, sur un cube comme sur une sculpture ; les tailles et le budget comptent les courbes | #195, #196, #44 |
| 3 | *Quads Around Thin Parts* ; `--quads-around` | les parties fines du banc gardent leur volume aux trois budgets sans que la borne sur la taille dérape | #38, #18 |
| 4 | Les pôles du projet et l'outil *Poles* ; `--poles`, `--poles-out` | mesurés sur le banc, essayés par des artistes, leurs remarques réglées | #249 |
| 5 | *Follow Curvature*, *Fewer Irregular Vertices*, *Volume Fit* ; les préréglages livrés | valeurs par défaut réglées sur le corpus ; aucune portée ne plie de quad ; le volume gardé après subdivision | #13, #21, #33, #72, #39 |
| 6 | *Output: Triangles*, pour *Simulation* ; `--method triangles` | aucun triangle n'en traverse un autre sur les modèles de [TRIANGLES.fr.md](docs/TRIANGLES.fr.md) | #226 |
| 7 | *Materials*, les lignes entre eux ; `--groups` | les frontières entre matériaux gardées sur un vrai modèle, la voiture de l'auteur d'abord | #47 |
| 8 | Le greffon sous Windows et macOS | lancé dans Blender sur chacun, macOS notarisé | #110 |

Les autres fonctions expérimentales sont des outils pour regarder dans la chaîne (`field`,
`tmesh`, `quantize`, Rerun et les autres) ou des pistes de recherche (`--quality slow`) : elles
restent expérimentales, et changent avec la chaîne.

## 2. Limites de la chaîne

Ce qui borne toutes les fonctions (la liste est dans
[FEATURES.fr.md](docs/FEATURES.fr.md#limites-de-la-chaîne)). Les vrais modèles ressemblent à ces
entrées plus souvent qu'à une sculpture propre.

| Ordre | Piste | Fait quand | Issues |
|---|---|---|---|
| 1 | **Petits morceaux** | chaque morceau du banc passe par la chaîne principale, aucun vide | #34 |
| 2 | **Ce qu'un maillage perd encore de la forme** : poches profondes, détails plus fins que le maillage de travail | la distance de Hausdorff de l'ogre à 3000 quads sous 10 pour mille | #36, #48 |
| 3 | **Sculptures denses** | une sculpture de millions de triangles remaillée dans un temps qu'un artiste attend | #45 |
| 4 | **Robustesse** : triangles plats, inversions, borne sur le temps | aucune entrée du banc, ou faite pour le casser, ne donne un maillage invalide ni ne dépasse une limite annoncée | #31, #10, #27, #194 |
| 5 | **Structure** : bandes étroites, complexe de base | moins de patchs et de sommets irréguliers aux trois budgets | #5, #20 |

## 3. Nouvelles fonctions

Chacune à spécifier et valider quand son tour vient.

| Fonction | Ce que c'est | Issues |
|---|---|---|
| **Menu Quad Shape** | une sphère, un cylindre ou un disque en quads avec leurs pôles idéaux | #159 |
| **Cuire la sculpture sur le résultat** | carte de normales, hauteur et occlusion ambiante, lues par la correspondance du moteur avec la sculpture | #51 |
| **Améliorer un maillage à partir d'un premier essai** | un essai noté, le meilleur de plusieurs gardé, une partie refaite là où elle a raté | #40 |
| **Triangles propres pour l'impression 3D** | la même chaîne avec des étoiles à six branches au lieu de croix | #70 |

## 4. Recherche

Les papiers, les preuves Lean et les expériences derrière la quantification sont sur la
[page recherche](site/content/research.fr.md) et dans leurs issues (étiquette *research-note*).
Ce qu'ils trouvent entre dans la chaîne par la section 1 ou 2, mesuré d'abord sur le banc.

## Ce qui n'est pas prévu

- La chaîne de patchs pour les surfaces dures (QuadWild) : les murs du T-mesh font le travail
  (#30).
- Une carte exacte sur grille entière et son extraction : la route directe depuis le T-mesh
  l'a remplacée (#1).
- Embarquer la visionneuse Rerun dans ce qui est distribué : elle reste un outil pour ceux qui
  travaillent sur la chaîne.
