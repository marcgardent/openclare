# Banc de mesure

Compare n'importe quel remailleur quad à sa source, avec les métriques du cahier des charges
([ALGORITHME.md](../ALGORITHME.md), section 0). Les derniers chiffres sont dans
[results/RESULTATS.md](results/RESULTATS.md).

## Lancer

```bash
cargo build --release
```

```bash
python3 bench/run.py
```

Il faut `blender` dans le `PATH` : il sert à lancer QuadriFlow et à générer le corpus provisoire.
La campagne complète prend environ trois minutes. Options : `--models blob,cube`,
`--tools quadriflow`, `--target 5000`, `--seeds 1,2,3`.

Pour mesurer un seul maillage, quel que soit l'outil qui l'a produit :

```bash
./target/release/retopo measure source.obj quads.obj --target 3000
```

## Fichiers

| Fichier | Rôle |
|---|---|
| `run.py` | lance chaque outil sur chaque modèle, mesure, écrit `results/` |
| `quadriflow.py` | script Blender : QuadriFlow sur un OBJ, une fois par graine |
| `make_corpus.py` | script Blender : génère le corpus provisoire dans `corpus/` |
| `blender_io.py` | lecture et écriture OBJ sans conversion d'axes, pour que tous les outils voient les mêmes coordonnées |

Les métriques sont dans `crates/retopo/src/metrics.rs`, chacune testée sur un maillage dont le
score est connu (`crates/retopo/tests/metrics.rs`).

## Ajouter un outil

Une ligne dans le dictionnaire `TOOLS` de `run.py`. Pour une chaîne de ce dépôt :
`"nom": retopo("nom")`, où le nom est celui que connaît `retopo remesh --method`.

## Réglages de la comparaison

- QuadriFlow : `use_mesh_symmetry=False` (activé par défaut, il fausserait tout),
  `use_preserve_sharp` sur les modèles à arêtes vives, `use_preserve_boundary` sur les modèles
  ouverts.
- Plancher : `--crease-angle 30` sur les modèles à arêtes vives.
- Même budget de faces et mêmes graines pour tous.

## Limites connues

- **Corpus de synthèse.** Les formes sont générées, pas sculptées. Les conclusions devront être
  confirmées sur de vrais modèles.
- **Faces inversées.** Le test compare l'orientation de chaque face à celle de la source autour
  d'elle. Sur Suzanne (parois fines, pièces qui se traversent) le décompte est incertain à une ou
  deux faces près ; ailleurs il recoupe un test indépendant de la source.
- **Complexe de base (C).** Sature près du nombre de faces quand le maillage n'a pas de structure
  globale ; à lire avec le nombre de sommets irréguliers.
- **Alignement (A).** La courbure est lue à l'échelle d'un quad. Sur une forme bruitée elle n'a
  pas de direction stable et la métrique reste au niveau du hasard (22,5°).
- **Temps.** Celui de QuadriFlow inclut la conversion de maillage interne à Blender, celui de
  `retopo` exclut la lecture et l'écriture des fichiers : ordre de grandeur seulement.
- **Source ouverte, sortie fermée.** Si un outil rebouche des bords, la validité ne le signale
  pas ; c'est la distance de Hausdorff qui le montre.
