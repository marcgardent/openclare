# Banc de mesure

Compare n'importe quel remailleur quad à sa source, avec les métriques du cahier des charges
([ALGORITHME.md](../ALGORITHME.md), section 0). Les derniers chiffres sont dans
[results/RESULTS.md](results/RESULTS.md).

## Lancer

```bash
cargo build --release
```

```bash
python3 bench/run.py
```

Il faut `blender` dans le `PATH` : il sert à lancer QuadriFlow et à générer le corpus provisoire.
La campagne complète prend environ cinq minutes. Options : `--models blob,cube`,
`--tools quadriflow`, `--target 5000`, `--seeds 1,2,3`. Un lancement partiel met à jour les résultats existants ;
chaque budget a ses fichiers (`RESULTS.md` pour 3000 quads, `RESULTS-8000.md`, etc.).

Pour mesurer un seul maillage, quel que soit l'outil qui l'a produit :

```bash
./target/release/retopo measure source.obj quads.obj --target 3000
```

## Fichiers

| Fichier | Rôle |
|---|---|
| `run.py` | lance chaque outil sur chaque modèle, mesure, écrit `results/` |
| `tmesh.py` | construit le T-mesh de chaque modèle et écrit `results/TMESH.md` |
| `fetch_blenderkit.py` | télécharge les modèles BlenderKit du banc, non redistribuables, et les exporte (`make blenderkit`) |
| `import_blenderkit.py` | script Blender : exporte un `.blend` de BlenderKit en OBJ source |
| `acceptance.py` | confronte les résultats aux seuils d'acceptation, écrit `results/ACCEPTANCE.md` |
| `quantize.py` | quantifie le T-mesh de chaque modèle et écrit `results/QUANTIZATION.md` |
| `quadriflow.py` | script Blender : QuadriFlow sur un OBJ, une fois par graine |
| `quadremesher.py` | script Blender : Quad Remesher sur un OBJ |
| `make_references.py` | produit et range les mires Quad Remesher tant qu'une licence est disponible |
| `corpus/` | les modèles sources, versionnés ([provenance](corpus/README.md)) |
| `corpus-large/` | originaux trop lourds pour git (l'ogre à 4,7 millions de triangles), non versionnés |
| `reference/` | les mires conservées, versionnées ([notice](reference/README.md)) |
| `make_corpus.py` | script Blender : génère les formes de synthèse du corpus |
| `import_crane.py`, `subdivide.py` | convertissent les modèles téléchargés du dépôt de Keenan Crane en sources du corpus |
| `blender_io.py` | lecture et écriture OBJ sans conversion d'axes, pour que tous les outils voient les mêmes coordonnées |

Les métriques sont dans `crates/retopo/src/metrics.rs`, chacune testée sur un maillage dont le
score est connu (`crates/retopo/tests/metrics.rs`).

## Quad Remesher

Quad Remesher (Exoside) est un logiciel commercial. Ses résultats sont calculés une fois, avec le
module Blender et une licence valide (le banc ne contourne ni l'un ni l'autre), puis rangés dans
`reference/`. Le banc relit ensuite ces mires, que le logiciel soit encore installé ou non. Pour
un modèle ou un budget sans mire et sans licence, les colonnes correspondantes restent vides.

- `quadremesher` : réglages par défaut (taille adaptative à 50 %, nombre de quads ajusté).
- `quadremesher-uniform` : taille constante et nombre de quads exact, pour comparer à armes
  égales avec des remailleurs à taille unique.

Il n'a pas de graine : une seule exécution par modèle. Les mires proviennent d'une licence
d'essai : usage non commercial.

## Ajouter un outil

Une ligne dans le dictionnaire `TOOLS` de `run.py`. Pour une chaîne de ce dépôt :
`"nom": retopo("nom")`, où le nom est celui que connaît `retopo remesh --method`. Une chaîne
qui n'utilise pas la graine se déclare avec `seeded=False` et n'est lancée qu'une fois.

Les trois chaînes du dépôt y sont : `instant-meshes` (le plancher), `integer-grid` (la version
provisoire) et `quad-layout` (la cible).

## Réglages de la comparaison

- QuadriFlow : `use_mesh_symmetry=False` (activé par défaut, il fausserait tout),
  `use_preserve_sharp` sur les modèles à arêtes vives, `use_preserve_boundary` sur les modèles
  ouverts.
- Plancher : `--crease-angle 30` sur les modèles à arêtes vives.
- Quad Remesher : détection des arêtes vives sur les modèles à arêtes vives seulement, symétrie désactivée.
- Même budget de faces et mêmes graines pour tous.

## L'ogre

`corpus/ogre.obj` vient d'une sculpture de 4,7 millions de triangles, ramenée à 200 000 par :

```bash
./target/release/retopo preprocess bench/corpus-large/ogre_hp.obj bench/corpus/ogre.obj --triangles 200000
```

L'écart à l'original est de 0,055 ‰ de la diagonale en moyenne et 4,5 ‰ au pire (sur les détails
les plus anguleux).

## Limites connues

- **Corpus de synthèse.** Les formes sont générées (`make_corpus.py`), pas sculptées. Les conclusions devront être
  confirmées sur de vrais modèles.
- **Faces inversées.** Le test compare l'orientation de chaque face à celle de la source autour
  d'elle. Sur Suzanne (parois fines, pièces qui se traversent) le décompte est incertain à une ou
  deux faces près ; ailleurs il recoupe un test indépendant de la source.
- **Complexe de base (C).** Sature près du nombre de faces quand le maillage n'a pas de structure
  globale ; à lire avec le nombre de sommets irréguliers.
- **Alignement (A).** La courbure est lue à l'échelle d'un quad. Sur une forme bruitée elle n'a
  pas de direction stable et la métrique reste au niveau du hasard (22,5°).
- **Temps.** Celui de QuadriFlow inclut la conversion de maillage interne à Blender, celui de
  Quad Remesher un aller-retour en FBX et son gestionnaire de licence, celui de `retopo` exclut la
  lecture et l'écriture des fichiers : ordre de grandeur seulement.
- **Source ouverte, sortie fermée.** Si un outil rebouche des bords, la validité ne le signale
  pas ; c'est la distance de Hausdorff qui le montre.
