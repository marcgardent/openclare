# openclare

Retopologie automatique en quads, alignée sur un champ de directions, écrite en Rust.
Objectif : une alternative open source à Quad Remesher, meilleure que le remaillage quad de
Blender. (Les crates et l'exécutable portent encore le nom provisoire `retopo`.)

![Tore, cube à arêtes vives et Suzanne remaillés](docs/images/apercu.png)

## État

Deux chaînes de remaillage, de bout en bout (triangles en entrée, quads en sortie) :

- **`integer-grid`**, la chaîne cible, encore provisoire. Champ de croix aligné sur la courbure,
  simplification des singularités, paramétrisation à grille entière, extraction. Sur le corpus,
  ses arêtes suivent la forme deux fois mieux que celles de Blender et presque aussi bien que
  celles de Quad Remesher, avec une vraie structure globale. Elle n'est pas encore fiable : un
  maillage sur quatre garde quelques faces non quads, et le nombre de faces demandé n'est tenu
  qu'à 10 % près.
- **`instant-meshes`**, le plancher de comparaison : robuste et rapide, mais sans structure et
  avec beaucoup de sommets irréguliers.

La suite de la chaîne cible est en construction, étape par étape : le **T-mesh**, qui découpe
la surface en rectangles à partir des singularités, puis la **quantification**, qui donne à
chaque côté un nombre entier d'arêtes. Les deux sont valides sur tout le corpus, et le nombre de
quads demandé est tenu à 4 % près. Elles n'entrent pas encore dans `remesh` ; elles se regardent
avec `retopo tmesh` et `retopo quantize`.

Le cahier des charges, les mesures et la feuille de route sont dans
[ALGORITHME.md](ALGORITHME.md).

## Utilisation

```bash
cargo build --release
```

```bash
./target/release/retopo remesh entree.obj sortie.obj --quads 5000
```

Pour essayer sans maillage sous la main (`torus`, `sphere`, `cube`, `plane`) :

```bash
./target/release/retopo primitive torus tore.obj
```

### Préréglages

Un préréglage est un jeu de paramètres nommé ; chaque paramètre reste réglable par-dessus.

```bash
./target/release/retopo presets
```

```bash
./target/release/retopo remesh entree.obj sortie.obj --preset animation --quads 8000 --alignment 0.1
```

Pour enregistrer les réglages d'un lancement et les réutiliser comme préréglage personnel :

```bash
./target/release/retopo remesh entree.obj sortie.obj --preset organic --singularity-reach 4 --save-config perso.txt
```

```bash
./target/release/retopo remesh autre.obj sortie.obj --config perso.txt
```

`retopo --help` liste toutes les options. Pour mesurer un résultat et le comparer à Blender et à
Quad Remesher, voir [bench/README.md](bench/README.md).

Comme bibliothèque :

```rust
use retopo::{RemeshConfig, Target};

let mut config = RemeshConfig::for_organic(Target::QuadCount(5000));
config.alignment = 0.1;
let (quads, report) = config.remesh(&triangles)?;
```

Pour voir le T-mesh d'un modèle, ses coupes écrites comme des lignes sur la surface :

```bash
./target/release/retopo tmesh entree.obj --quads 3000 --simplify 2 --traces coupes.obj
```

Et pour le quantifier :

```bash
./target/release/retopo quantize entree.obj --quads 3000 --simplify 2
```

## Organisation

| Module | Rôle |
|---|---|
| `mesh` | conteneurs (`TriMesh`, `PolyMesh`), connectivité, OBJ, subdivision, primitives de test |
| `geometry` | normales, aires, détection des arêtes caractéristiques |
| `hierarchy` | graphe multi-résolution pour résoudre du grossier au fin |
| `field` | champ d'orientation (4-RoSy), champ de position (4-PoSy), contraintes, singularités |
| `extract` | du champ ou de la carte entière au graphe de quads, puis aux faces |
| `relax` | lissage final contraint à la surface et aux arêtes vives |
| `spatial` | BVH pour la projection sur la surface source |
| `pipeline` | enchaînement complet de la chaîne Instant Meshes |
| `remesher` | interface commune aux chaînes, fabrique qui les crée par nom, et la chaîne `integer-grid` |
| `preprocess` | étape de prétraitement : nettoyage, décimation, remaillage isotrope, courbures |
| `crossfield` | étape du champ de croix : une croix par face, alignement sur la courbure, singularités |
| `singularity` | étape de simplification : annulation des paires de singularités inutiles |
| `parametrization` | étape de paramétrisation : graphe de coupe, carte (u, v) sans couture |
| `tmesh` | étape de structure globale : traces depuis les singularités, patchs rectangulaires |
| `quantization` | étape de quantification : longueurs entières, connectivité du maillage quad |
| `linalg` | algèbre linéaire creuse (enveloppe de la bibliothèque `faer`) |
| `config` | configuration unique d'un remaillage, préréglages, enregistrement et relecture |
| `curvature` | courbures et directions principales |
| `metrics` | mesures de qualité d'un maillage quad par rapport à sa source |

La chaîne actuelle est décrite dans
[docs/REFERENCE_INSTANT_MESHES.md](docs/REFERENCE_INSTANT_MESHES.md).

## Tests

```bash
cargo test
```

Les tests vérifient des invariants plutôt que des valeurs figées : théorème de Poincaré-Hopf sur
les singularités du champ, caractéristique d'Euler conservée, maillage fermé et variété, sommets
sur la surface, coins et arêtes vives retrouvés exactement, déterminisme.

## Licence

GPL-3.0-or-later, voir [LICENSE](LICENSE).
