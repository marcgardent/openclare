# lab : le registre et le lanceur des calculs d'openclare

État : **brouillon à valider**. Aucun code n'est écrit avant que Marc valide ce document.
Version anglaise de référence : `SPEC.md`.

Amendement du 2026-11-08, **brouillon à valider** : lab devient un programme à part, installé sur la
machine, qui récupère le dépôt, extrait le commit demandé et y lance la tâche ; une mémoire par machine
dans `~/.openclare-lab`, réunie dans git sur une branche de cette machine (§1, §2, §3, §4, §4 bis, §5,
§6, §7.3, §8.1, §10 bis, §11, §12, §13, §14, §15, §17).

Amendement R1 du 2026-11-13, **validé par Marc le même jour** : un job est un calcul, chronométré et
jugé par lab ; un batch est une grille de jobs, déclarée en `[[batch]]` avec des axes et développée
par lab, qui l'exécute dans un pool, une fiche par job, chaque job coupé à son délai, les versions des
outils dans chaque fiche ; d'abord sans fenêtre (§1, §3, §5, §6 bis, §14, §15, §17). La grille dans la
fenêtre, les relances, les dépendances en cache et les rapports sont R2 à R4 (§15). Dans ce qui suit,
la « tâche » des sections plus anciennes est un job ; l'interface dit « job » et « batch ».

Amendement R2 du 2026-11-13, demandé par Marc le même jour (« finis tes jalons ») : le moteur d'un
commit et la source d'une scène sont des dépendances, `[[dependency]]`, faites une fois par clé dans
`<lab>/cache/` et partagées par tous les batches qui en ont besoin (§3, §5, §6 bis, §15, §17).

Amendement R3 du 2026-11-13, demandé le même jour : la fenêtre met un batch en file comme un seul
élément et une séquence peut nommer des batches ; un écran Batches montre la grille, dont on choisit
des cases à remettre en file ; un job déjà fait est sauté sauf si on le redemande ; une estimation par
job (§6 bis, §11, §14, §15).

Amendement R4 du 2026-11-13, demandé le même jour : le `report` d'un batch, un court script qui lit les
sorties de ses jobs et fait le tableau, relancé en quelques secondes sans rien recalculer (§6 bis,
§14, §15).

## 1. Objet

Tout calcul long du projet (bancs, suites de tests, constructions et oracle Lean, expériences des
papiers) se lance depuis une seule application de bureau, qui garde une fiche par exécution : ce qui
a tourné, sur quel commit, sur quelle machine, sous quelle charge, combien de temps, et quelle
version de quel papier elle étaye. Les temps cessent de vivre dans des journaux, des descriptions de
PR et des notes de mémoire.

L'application fait six choses : elle tient la base des exécutions (§4), le catalogue des tâches (§6),
une file qui les lance une par une (§8), une estimation de progression tirée de l'historique (§10), un
moniteur de la machine avec un indicateur « calme » (§9), et une vue d'historique avec export (§11).

Hors champ : lancer d'ici une tâche sur une autre machine (chaque machine fait tourner son lab, et
leurs fiches se réunissent dans git, §4 bis), lancer deux éléments de la file à la fois (les jobs d'un
batch tournent côte à côte dans son pool, §6 bis), programmer des tâches
récurrentes, lire les résultats d'une exécution (les résumés des scripts restent ce qu'ils sont).

## 2. Nom et emplacement

- Dossier `lab/` à la racine du dépôt, son propre espace de travail Cargo (son `Cargo.toml`, son
  `Cargo.lock` et son `target/`), hors de l'espace de travail du moteur : les constructions du moteur
  et la CI ne changent pas.
- Paquet `openclare-lab`, binaire `lab`. Installé sur la machine par `make lab-install` à la racine du
  dépôt (§14) ; `cargo run --release` depuis `lab/` pendant son développement. Le `make install` de la
  racine reste celui du greffon Blender.
- lab possède un **miroir** du dépôt, `~/.openclare-lab/repo.git`, récupéré depuis GitHub
  (`marcgardent/openclare`, ou le réglage `remote`) avant chaque file et à la demande. Il n'a plus besoin
  du clone d'où il a été lancé : le premier démarrage crée le miroir, depuis ce clone quand il y en a un
  (rapide, hors ligne, ses objets LFS liés), sinon depuis GitHub. Le catalogue qu'il montre est le
  `lab/jobs.toml` de `main` dans le miroir ; `--catalogue <fichier>` en lit un autre, le temps d'en écrire un.
- Une tâche tourne sur une **référence** : un tag (`v0.7.0`), une branche (`main`, `worktree-borders`),
  une pull request (`pull/301`) ou un hash. lab la résout dans le miroir et extrait ce commit dans
  `~/.openclare-lab/checkouts/<hash court>/`, un worktree détaché du miroir, un par commit, réutilisé par
  toute exécution sur ce commit. Le catalogue dont la tâche est rendue est le `lab/jobs.toml` de ce
  commit ; un commit antérieur à lab prend le catalogue de `main`.
- La porte de secours pour une expérience non commise : une tâche lancée dans un dossier donné à la main
  (checkout **local**). Sa fiche est hors tag, `dirty` si l'arbre diffère de HEAD, et orpheline si son
  commit n'est pas poussé (§10 bis).

## 3. Fichiers

| Fichier | Suivi | Écrit par | Contenu |
|---|---|---|---|
| `lab/jobs.toml` | oui | Marc à la main | le catalogue (§6), lu au commit où la tâche tourne |
| `lab/runs.jsonl` | oui | l'application, à la demande de Marc (« Copier l'historique dans le dépôt ») | l'instantané publié de l'historique |
| `<lab>/runs.jsonl` | non | l'application | la mémoire vivante de cette machine, une exécution par ligne |
| `<lab>/logs/<id>.log` | non | la tâche (sa sortie standard et d'erreur) | le journal de chaque exécution |
| `<lab>/out/<id>/` | non | le job, par `{out}` | les fichiers d'un job d'un batch (§6 bis) |
| `<lab>/cache/<dépendance>/<clé>/` | non | le job d'une dépendance | ce qu'une dépendance a fait, partagé par les batches qui en ont besoin (§6 bis, R2) |
| `<lab>/refused.jsonl` | non | l'application | les fiches qui nomment encore un chemin local (section 4) |
| `<lab>/state.json` | non | l'application | la file en cours et l'exécution en cours, pour la reprise |
| `<lab>/settings.toml` | non | l'application | dépôt distant, nom de la machine, politique de push, seuils, alerte de threads, politiques (§12) |
| `<lab>/repo.git/` | non | git | le miroir du dépôt (§2) |
| `<lab>/checkouts/<hash court>/` | non | git, les préparations, les tâches | un worktree détaché par commit exécuté (§2), avec son `target/` et son `.lake/` |
| `<lab>/machines/<machine>.jsonl` | non | l'application | la mémoire de chaque autre machine, telle que récupérée en dernier (§4 bis) |

`<lab>` est `~/.openclare-lab`, un seul dossier pour tout ce que lab garde sur la machine ;
`--data <dossier>` le déplace (les tests utilisent `tests/fixtures`). Le premier démarrage trouve les
`~/.local/share/openclare-lab` et `~/.config/openclare-lab` de la version précédente et déplace leurs
fichiers dans `<lab>` une fois. L'application montre le chemin dans sa boîte « À propos ».

## 4. Stockage

**Format : JSON Lines, une exécution par ligne, triées par heure de début.** Quelques centaines de
lignes tiennent en mémoire et se chargent en millisecondes ; une ligne se lit avec `jq`, en Python
sans bibliothèque, et dans un diff. SQLite par `rusqlite` apporterait des index et des transactions
dont ce volume n'a pas besoin, et un fichier binaire que git ne sait ni comparer ni fusionner.
Modifier un commentaire réécrit le fichier de façon atomique (écrit à côté, puis renommé) : le diff
git de l'instantané est cette seule ligne.

**Le fichier vivant est hors de l'arbre de travail ; git en garde un instantané.** Si la base
vivante était `lab/runs.jsonl` d'un checkout, ce checkout porterait sans cesse une
modification locale : `git pull` refuserait de passer chaque fois qu'une PR change le fichier, et le
drapeau `dirty` de toute exécution lancée là serait vrai à cause de la base elle-même. L'application
écrit donc `<lab>/runs.jsonl`, un seul fichier pour la machine quel que soit le checkout ou le
worktree où tourne une tâche, et « Copier l'historique dans le dépôt » écrit l'union des mémoires des machines (§4 bis),
triée, dans `lab/runs.jsonl` du checkout que Marc choisit, à commettre avec la PR qui cite les exécutions.
`lab/.gitattributes` déclare `runs.jsonl merge=union` : deux branches qui ajoutent chacune des
exécutions fusionnent sans conflit ; le chargement garde ensuite une ligne par identifiant (celle dont
`edited` est le plus récent). L'instantané suivi sert aussi de sauvegarde du fichier vivant ;
l'application garde en plus les cinq dernières versions du fichier vivant (`runs.jsonl.1` à `.5`).

**Aucun chemin absolu n'est écrit dans une fiche.** Les chemins sont relatifs à la racine du checkout
où la tâche a tourné, et ce checkout est nommé par la référence demandée (`v0.7.0`, `main`, `pull/301`, un hash), ou
`local` pour un dossier donné à la main (§2). Avant d'écrire, l'application remplace la racine du checkout par `{repo}` et
le dossier personnel par `~` dans la commande, l'environnement et les chemins, puis refuse d'écrire une
fiche qui correspond encore aux motifs de dossier personnel de `hygiene.yml` (Linux, macOS et Windows) ou qui
contient `/tmp/`, et montre le champ fautif ; cette fiche est gardée dans `<lab>/refused.jsonl`,
jamais copiée dans le dépôt. L'environnement enregistré est celui des variables que
l'application pose (§7), pas tout l'environnement hérité.

## 4 bis. Une mémoire par machine, réunies dans git

- Chaque machine a sa mémoire vivante, `<lab>/runs.jsonl`, et sa **branche** dans le dépôt,
  `lab/runs-<machine>` (`<machine>` est le réglage `machine`, le nom d'hôte par défaut). La branche
  porte un seul fichier, `runs.jsonl`, copie de la mémoire vivante, et n'a aucune histoire commune avec
  `main` (une branche orpheline, comme `gh-pages`). Seule cette machine y écrit : chaque push est un
  fast-forward, jamais de conflit ni de course entre deux machines.
- **Push** : après chaque exécution, selon le réglage `push` (`ask`, par défaut : un bouton dans la barre
  du haut avec le nombre de fiches pas encore poussées ; `auto` ; `never`). Ce qui est poussé est l'union
  de la branche et de la mémoire vivante : une mémoire effacée sur la machine ne perd rien. Le push
  demande une clé en écriture sur le dépôt : celle de Marc sur ses machines, une clé de déploiement
  limitée au dépôt sur une machine qui tourne la nuit (Q18).
- **Lecture** : à chaque ouverture et après chaque récupération, lab lit les branches `lab/runs-*` du
  miroir dans `<lab>/machines/` et en fait l'union avec la mémoire vivante de cette machine : une ligne
  par identifiant, le `edited` le plus récent gagne, la règle que le §4 a déjà. Un commentaire ou un
  papier modifié sur une autre machine arrive donc ici à la récupération suivante ; la même fiche
  modifiée sur deux machines garde la modification la plus récente. Une fiche d'une autre machine
  modifiée ici s'ajoute à la mémoire de cette machine, et arrive partout comme la plus récente.
- **Publication** : « Copier l'historique dans le dépôt » écrit cette union, triée, dans `lab/runs.jsonl`
  d'un checkout que Marc choisit, à commettre avec la PR qui cite les exécutions ; `main` ne change
  toujours que par PR.
- Les tags et les résumés du dépôt (§13) sont lus dans le miroir (`git tag`,
  `git show main:<fichier>`, `git log -1 main -- <fichier>` ; les branches du dépôt distant sont celles du
  miroir), jamais dans un arbre de travail. L'écran État ne dépend donc d'aucun checkout : le même sur
  chaque machine après la même récupération.

## 5. Modèle de données

Les types Rust sont ceux de `SPEC.md` §5, identiques ; seuls les commentaires sont traduits ici.

- `Run` : une exécution, une ligne de `runs.jsonl`. Champs : version du format, identifiant
  (`20261106-221403-forced-whole-lower`), nom de la tâche au moment de l'exécution, origine
  (`App`, `Seeded { from }` pour l'historique amorcé, `Manual` pour une saisie à la main), file
  d'origine, début et fin (RFC 3339 avec décalage), durée de la tâche seule (préparation exclue),
  statut, préparation, commande telle qu'exécutée (marques remplies, `{repo}` gardé), variables
  posées par l'application, paramètres choisis, checkout (la référence demandée, ou `local`, §2), dossier de
  travail relatif à ce checkout, git (commit, `dirty`, référence, `pushed` : le commit est atteignable
  depuis une branche ou un tag du dépôt distant, §10 bis), threads, plafond mémoire, machine, charge, verdict de calme,
  commentaire (modifiable après coup), papiers (modifiables après coup), journal, fichiers produits,
  date de dernière modification du commentaire ou des papiers. Depuis R1 : la case (`cell`, absente
  pour un `[[job]]` : le lancement du batch, la valeur de chaque axe, la largeur du pool), le délai
  sous lequel le job a tourné (`timeout_seconds`), les versions des outils (`tools` : `blender`,
  `rustc`, `python`, `lean`) ; les trois sont omis de la ligne quand ils sont vides, si bien que les
  fiches d'avant se relisent et se réécrivent à l'identique. Depuis R2 : les clés des dépendances
  d'une case (`dependencies` : `{"engine": "3f2a…", "source": "9c01…"}`), et pour le job d'une
  dépendance la clé qu'il a faite (`cache`) ; omis eux aussi quand ils sont vides.
  Git, threads, machine et charge sont facultatifs : une fiche amorcée ou saisie n'a que ce qui est
  connu.
- `Status` : `Ok`, `Failed { code, signal }`, `Stopped` (arrêté par Marc), `Interrupted`
  (l'application s'est terminée pendant l'exécution, §8.5), `TimedOut` (tué par lab à son délai,
  §6 bis ; `seconds` est alors le temps jusqu'à l'arrêt).
- `Prepare` : commande, durée, statut, et la raison d'un saut (`skipped_because`).
- `Git` : commit (40 caractères hexadécimaux), `dirty` (des fichiers suivis diffèrent de HEAD,
  `lab/runs.jsonl` exclu), branche.
- `Threads` : le réglage et le nombre effectif, ce que la tâche utilise (§7).
- `Machine` : nom d'hôte, processeur, cœurs, threads, mémoire, GPU (nom, mémoire), noyau, et le
  libellé des réglages, par exemple « i7-7700K 4c/8t 2017, 62 GB ».
- `Sample` : moyennes de charge 1/5/15, CPU % de toute la machine (100 = tous les threads occupés),
  CPU % **étranger** (les processus qui ne sont pas ceux de l'exécution), mémoire utilisée, VRAM
  utilisée et GPU % quand ils se lisent.
- `Load` : l'échantillon au début, à la fin, et le résumé pendant (`During` : nombre d'échantillons,
  moyenne et pic du CPU étranger, load1 maximal, pics de mémoire et de VRAM).
- `Calm` : `before` (machine calme sur la fenêtre avant le début, §9), `during` (charge étrangère
  sous les seuils pendant l'exécution), `forced` (Marc l'a lancée quand même, §7.2),
  `timing_valid` (`before && during && !forced`), `reason` en mots (« CPU étranger 34 % à 23:12 (lake) »).
- `Paper` : `A` (closing-loops) ou `B` (turns-of-a-t-mesh) ; `PaperRef` : un papier et un libellé
  (« v0.7.0 », « 2026-11-06 », « tab:forced »).
- `FileRef` : chemin, taille, SHA-256 jusqu'à 1 Go.
- `Job` : une tâche du catalogue, une table `[[job]]` de `jobs.toml` : nom unique, description,
  catégorie (`Build`, `TestSuite`, `QuickBench`, `Bench`, `LeanBuild`, `LeanOracle`, `Experiment`),
  commande avec marques, préparation, choix du checkout (`Ask`, `Reference(nom)`), dossier,
  paramètres avec leurs valeurs par défaut (modifiables par élément de file), environnement propre,
  usage des threads (`Setting` ou `Fixed(n)`), plafond mémoire, « le temps compte », motifs des
  fichiers produits, papier par défaut, étiquettes, délai (`timeout`, aucun par défaut pour un
  `[[job]]`, Q20).
- `Batch` : une table `[[batch]]` : tous les champs d'un `Job` (son délai 30 minutes s'il n'en donne
  pas), et ses axes dans l'ordre écrit, chacun un nom et ses valeurs ; une valeur peut porter ses
  propres marques (`"cage"`, `{"preset": "hard-surface"}`).
- `Dependency` (R2) : une table `[[dependency]]` : les champs d'un `Job` sauf `prepare`, `summary` et
  `outputs`, plus `key` (les chemins du checkout dont le résultat dépend, marques permises), `tools`
  (les outils dont la version entre dans la clé) et `provides` (le fichier que `{<nom>}` désigne dans
  le dossier du cache). Un `Batch` gagne `needs`, les dépendances de ses cases.
- `Sequence` : une liste ordonnée et nommée de tâches, une table `[[sequence]]` : les étapes d'un
  NIGHT.md dans l'ordre où il les donne.

## 6. Le catalogue

`lab/jobs.toml`, modifié dans l'écran Catalogue ou à la main ; l'application le relit quand le
fichier change sur le disque. L'exemple `forced-whole-lower` de `SPEC.md` §6 montre le format.

Marques : `{threads}` (le nombre effectif de threads), `{repo}` (la racine du checkout où tourne la
tâche), `{commit}` (son hash court), `{ref}` (la référence demandée), et les `{paramètres}` propres à la
tâche. Une marque inconnue est
une erreur montrée dans le catalogue, pas au lancement. `checkout` est la référence sur laquelle la tâche
tourne, `main` quand le champ est absent, ou `"ask"` : Marc la choisit sur l'élément de la file.

Le catalogue est amorcé avec les tâches que le dépôt a aujourd'hui : le tableau de `SPEC.md` §6 les
donne une par une, avec la durée **écrite** dans les documents (pas mesurée par l'application ;
« ? » quand rien n'est écrit). En résumé :

- constructions : `build-release`, `addon` ;
- suites de tests : `test-release` (plafond 40 Go, ~15 min selon Marc), `addon-test` ;
- bancs : `quick-bench` (préparation `cargo build --release && make addon`), `bench` (budget et
  outils en paramètres ; c'est aussi la comparaison aux autres remailleurs), `remeasure`, `scores`,
  `three-budgets` (les douze modèles à 2970, 3000 et 3030 quads, métriques et valences ; `{engine}`
  un commit à construire, ou vide pour le moteur construit ici ; checkout demandé) ;
- Lean : `lean-build`, `lean-oracle-seed-1`, `lean-oracle-doubles` (branche `worktree-borders`,
  PR #271, `--borders`, ~18 h d'un cœur au total), `lean-check-export` ;
- papier B : `export-male-body` (8 threads fixes, plafond 24 Go), les sept scripts rapides de
  l'étape 3 de NIGHT.md, `forced-whole-lower` (l'expérience des longueurs forcées, 1 à 4 h),
  `forced-sound`, `holonomy`, `strict`, `certify`, `conjugacy`, `parity`, `gadget` ;
- papier A : `twelve-models`, `ablation`, `male-body`, `figures`, `structure`, `against-remeshers`
  (lit seulement `bench/results/results.json`), `compare`, et les étapes de `exact/NIGHT.md`.

Séquences amorcées : `paper-B-data` (NIGHT.md, étapes 1 à 4 dans l'ordre), `paper-A-exact`
(exact/NIGHT.md, 1 à 4), `before-merge` (`build-release`, `test-release`, `quick-bench`).

## 6 bis. Batches (amendement R1)

Un **job** est un calcul : une commande, un groupe de processus, un temps, un journal, une fiche. Un
**batch** est une grille de jobs : un gabarit de commande et des axes, que lab développe en un job
par case. Un script qui fait 150 calculs dans un seul `[[job]]` est une boîte noire pour lab : pas de
progression, un seul temps pour tout, et le premier calcul qui s'éternise fait perdre les autres (le
job `pole-pairs-115`, 2026-11-13). Un batch est le même travail, avec lab pour pool et pour chrono.

### Le catalogue

L'exemple est celui de `SPEC.md` §6b, le batch `pole-pairs-115-remesh` de `lab/jobs.toml` : douze
modèles, trois budgets, `ref = ["main", "12a6675"]`, un remaillage et sa mesure par job, un thread
moteur, `timeout = "30m"`.

- Un `[[batch]]` a tous les champs d'un `[[job]]` (§6), plus `axes` et `timeout`.
- **Axes.** Chaque axe est une marque de la commande, avec la liste de ses valeurs (textes ou
  nombres). Une valeur peut être une table : son `value` est celui de la case, ses autres clés
  remplacent, pour cette case seule, les `parameters` du batch de même nom (`{preset}` ; une clé qui
  n'est pas un paramètre est un problème du catalogue). La grille est le produit des axes, développé
  dans l'ordre où ils sont écrits, le dernier variant le plus vite : l'axe comparé va en dernier,
  pour que ses valeurs tournent côte à côte sous la même charge.
- **`ref`** est le seul axe qui a un sens : la référence de chaque case, résolue et extraite comme au
  §2, un checkout par commit. Un batch sans cet axe tourne sur son `checkout`.
- **La préparation** peut nommer les marques intégrées et les paramètres, pas les autres axes : elle
  tourne une fois par checkout, pas par case.
- **Un seul catalogue pour toute la grille** : celui d'où le batch est tiré (`main` du miroir, ou
  `--catalogue`), pas le `jobs.toml` du commit de chaque case : une comparaison entre commits lance la
  même commande sur chacun.
- **`{out}`**, nouvelle marque intégrée : `<lab>/out/<id>/`, un dossier vidé avant le début du job.
  Chaque fichier qui s'y trouve va dans les `outputs` de la fiche, avec taille et SHA-256. Comme
  `{repo}`, la fiche garde `{out}` tel quel dans `command`, pour que deux exécutions de la même case
  aient la même clé de commande (§10).
- Un **`[[job]]` existant est un batch sans axe** : une case, lancée comme aujourd'hui, sa fiche sans
  `cell`. Un calcul dont les étapes dépendent les unes des autres (l'une lit le résultat d'une autre,
  un seul flux aléatoire, une seule optimisation) reste un seul job ; les jobs qui étaient des batches
  dans un script sont découpés, un job par unité, leur résumé refait par le rapport du batch (R4).
  Fait le 2026-11-13 :

  | Batch | Un job par | Son rapport |
  |---|---|---|
  | `lean-oracle-doubles`, `lean-oracle-seed-1` | instance (`index`) ; la génération, un seul flux aléatoire, est la préparation | — |
  | `lean-check-export` | fichier de voie (`index`) | — |
  | `lattice`, `holonomy`, `conjugacy`, `certify` | voie | `experiments/merge.py` |
  | `forced-bounds`, `forced-whole-lower`, `forced-valid-bounds`, `forced-valid` | famille (8 tenues, les bandes de la voie 2) : bornes, plafond et taille d'une famille se lisent l'une l'autre | `merge.py` |
  | `strict` | ligne de la table des conflits | `merge.py` |
  | `twelve-models`, `ablation`, `end-to-end`, `end-to-end-time-5`, `end-to-end-time-300` | (ligne ou variante ×) modèle × budget | `closing-loops/experiments/collect.py`, puis le script relit les exécutions |
  | `end-to-end-male-body`, `male-body`, `exact-remesh`, `forced-valid-remesh`, `most-closable` | variante, exécution ou voie | `collect.py` |
  | `three-budgets`, `pole-pairs-115-remesh`, `pole-pairs-115-tmesh` | modèle × budget × ref (ou pairs) | `bench/before_after.py` |
  | `pole-pairs-115-male-body` | côté (`ref`) : l'export, puis conflicts dessus | — |
  | `quick-bench`, `bench`, `remeasure` | modèle, modèle × outil, budget | `bench/run.py --merge` (bench) |

  `rings` est coupé en deux jobs : les anneaux avec leur arrondi (l'arrondi lit le sous-ensemble que
  les anneaux ont choisi), et `local-search`, qui ne lit que l'export. Restent un seul job, par la
  règle ou parce qu'ils durent quelques secondes : les compilations et suites de tests, `lean-build`,
  `export-male-body`, `census`, `conflicts` (sa table regroupe les voies), `pairs`, `combs`, `fox`
  (ses relations mettent toutes les voies ensemble), `forced-sound` (une famille), `parity`, `gadget`
  (un seul flux aléatoire), `scores`, `figures`, `structure`, `against-remeshers`, `compare`,
  `exact-relaxation`. Les batches des papiers lisent l'export du corps masculin dans `scratch/`, hors
  de leur clé : `skip_done = false`.

### Dépendances (amendement R2)

Une **dépendance** est un calcul dont beaucoup de jobs se servent et dont le résultat est le même
pour beaucoup d'entre eux : le moteur d'un commit, la source d'une scène. Déclarée une fois en
`[[dependency]]`, elle est faite une fois par **clé**, gardée dans `<lab>/cache/<nom>/<clé>/`, et
partagée par tous les batches et tous les lancements qui en ont besoin. Un second lancement du témoin
ne compile ni n'exporte rien.

```toml
[[dependency]]
name = "engine"
description = "The engine's command line, release."
category = "build"
command = "cargo build --release && cp target/release/openclare {out}/"
key = ["crates", "Cargo.toml", "Cargo.lock"]
tools = ["rustc"]
provides = "openclare"

[[dependency]]
name = "source"
description = "A scene of the bench as the add-on sees it, exported by Blender."
category = "quick-bench"
command = "blender -b --factory-startup bench/scenes/{model}.blend --python-exit-code 1 --python bench/blender_run.py -- --out {out} --no-remesh"
key = ["bench/scenes/{model}.blend", "bench/blender_run.py", "bench/blender_engine.py", "blender/openclare"]
tools = ["blender"]
provides = "source.obj"
threads = 1

[[batch]]
name = "pole-pairs-115-remesh"
needs = ["engine", "source"]
command = "{engine} remesh {source} {out}/mesh.obj --preset {preset} --quads {budget} && {engine} measure {source} {out}/mesh.obj --target {budget} --json > {out}/measure.json"
# … le reste comme plus haut, sans la préparation
```

- Une `[[dependency]]` a les champs d'un `[[job]]` sauf `prepare`, `summary` et `outputs` (ses
  fichiers sont ceux de son `{out}`), plus `key`, `tools` et `provides`. Ses threads sont le réglage
  sauf mention contraire ; elle n'a pas de délai sauf si elle en donne un. Elle ne se met pas seule en
  file et ne figure pas dans une séquence : c'est un batch qui en a besoin qui la fait.
- **Ses marques** : les intégrées, `{out}`, ses paramètres, et celles de la case qui en a besoin
  (`{model}` ci-dessus). Un batch qui a besoin d'une dépendance dont une marque manque à ses cases est
  un problème du catalogue.
- **La clé** : les 16 premiers chiffres hexadécimaux d'un SHA-256 sur le nom de la dépendance, sa
  commande rendue (`{out}` et `{repo}` gardés), pour chaque chemin de `key` rendu les objets git sous
  lui au commit du checkout (`git ls-tree -r` ; pour un dossier local, le texte de `git diff HEAD` sur
  ces chemins en plus, les nouveaux fichiers non suivis n'étant pas vus), et les versions des `tools`
  nommés (§6 bis, étape 2). Un chemin qui ne désigne rien à ce commit fait échouer la dépendance.
  Ainsi deux commits dont les sources du moteur sont les mêmes partagent un moteur, et une scène qui
  n'a pas changé n'est pas réexportée, quel que soit le commit.
- **`needs`**, dans un batch : les dépendances de ses cases. `{engine}` dans la commande est le chemin
  du fichier `provides` dans le dossier du cache du moteur de cette case. La fiche garde `{engine}`
  non rendu, comme `{out}`, pour que les clés de commande de deux commits restent égales, et donne les
  clés dans `dependencies`.
- **Quand** : après la lecture des outils et avant les préparations (étape 3 ci-dessous). Les
  dépendances de toutes les cases sont rendues et regroupées par clé ; celles déjà dans le cache sont
  dites et sautées (« engine 3f2a… en cache ») ; les autres tournent dans l'ordre de `needs`, un pool
  par dépendance (`réglage / threads` côte à côte), chacune un job avec sa fiche : `job` le nom de la
  dépendance, `cell` ses marques, `cache` sa clé, son journal dans `logs/`. Son `{out}` est
  `<lab>/out/<id>/` ; quand elle finit `Ok`, le dossier est déplacé en `<lab>/cache/<nom>/<clé>/` (un
  dossier qu'un autre lancement y a mis d'abord l'emporte, le nôtre est retiré), et ses fichiers sont
  notés là. Une dépendance en échec fait échouer les cases qui en ont besoin sans les lancer, une
  fiche chacune, comme une préparation en échec ; les autres cases continuent.
- Rien ne vide le cache tout seul (un moteur pèse environ 20 Mo, une source quelques Mo) ; le dossier
  se retire à la main (Q23).

### L'exécution d'un batch

1. **Développer** la grille ; un axe restreint à la main (`--axis model=ogre,pig`) ne garde que ces
   valeurs. Résoudre chaque `ref` (une référence inconnue tourne sur `main`, comme au §8.1).
2. **Outils**, lus une fois par checkout : `blender --version`, `rustc --version` et
   `python3 --version` lancés dans le checkout, leur numéro de version gardé ; Lean est la chaîne
   nommée par le fichier `lean-toolchain` du checkout, lu et non lancé (elan téléchargerait une chaîne
   absente). Un outil introuvable est omis. Chaque fiche reçoit la table de son checkout, celle d'un
   `[[job]]` aussi : rien ne notait jusqu'ici que Blender est en 5.3.0.
3. **Dépendances** (R2, ci-dessus), puis **préparations**, un checkout après l'autre, avant tout job : celle du §7.3, une fois par
   checkout (`.lab-prepared`), avec tout le réglage de threads (pas les `threads` du job). Son temps
   et son statut vont dans le `prepare` de chaque job de ce checkout. Une préparation en échec fait
   échouer les jobs de son checkout sans les lancer, une fiche chacun ; les autres checkouts
   continuent.
4. **Le pool.** `threads` est ce qu'un job utilise. Le pool lance `max(1, réglage / threads)` jobs
   côte à côte (de jour, le réglage 6 et un thread moteur : six jobs), dans l'ordre de la grille,
   chacun lancé comme au §8.1 : son groupe de processus, son plafond mémoire, son journal
   `<lab>/logs/<id>.log`. La limite de confort (§7.1) se vérifie sur la largeur du pool fois
   `threads`. Un batch dont **le temps compte tourne un job à la fois** (Marc, 2026-11-13) : six jobs
   côte à côte sur 4 cœurs et 8 threads se ralentissent, et le moniteur les compte comme à nous, si
   bien que leurs temps passeraient pour calmes.
5. **Une fiche par job, écrite à sa fin** : `job` est le nom du batch, `parameters` les marques de la
   case, `checkout` sa référence, `cell` le lancement, les valeurs des axes et la largeur du pool.
   L'id est l'heure du début, le nom du batch et les valeurs de la case
   (`20261113-221403-pole-pairs-115-remesh-ogre-3000-12a6675`), avec `-2`, `-3` s'il est pris. Un
   batch qui meurt perd les jobs en cours, jamais ceux déjà faits.
6. **Délai**, par job : `timeout` dans le batch (`"90s"`, `"30m"`, `"2h"`), 30 minutes s'il manque ;
   `--timeout` le remplace pour un lancement. Au-delà, lab arrête le job comme le fait Stop (§8.3,
   SIGTERM puis SIGKILL au groupe) : statut `TimedOut`. Un job en échec ou en dépassement est une case
   en échec ; le batch continue. Les politiques du §8.4 s'appliquent entre les éléments d'une file,
   pas dans un batch.
7. **Calme**, par job, selon le §9, le CPU étranger étant celui des processus hors du batch (les autres
   jobs du pool sont les nôtres) ; load1 n'est pas regardé pendant le batch, comme après une de nos
   exécutions. Un batch dont le temps compte lance son pool quand le moniteur a une fenêtre pleine
   (60 s), sinon ses premiers jobs n'auraient pas de verdict. Le temps d'un job dans un pool de six est
   un temps dans un pool de six : `cell.pool` le dit. Un batch dont le temps compte a un pool de un,
   les deux côtés d'une comparaison toujours entrelacés (Q21).

**Les threads dans le code, pas dans le pool** (Marc, 2026-11-13). Un job reçoit les threads qu'il
peut utiliser et son code s'en sert : lab pose `OPENCLARE_THREADS`, `RAYON_NUM_THREADS`,
`RUST_TEST_THREADS` et `CARGO_BUILD_JOBS` à ce nombre (§7.1), et tout batch qui lance le moteur prend
le réglage de threads (`threads` absent), un job à la fois quand son temps compte. Les scripts qui
appellent le moteur transmettent `OPENCLARE_THREADS` (ils le retiraient jusque-là, si bien que le
moteur prenait toute la machine) ; `end_to_end.py --cpus` le borne aux processeurs donnés. HiGHS
suit le même réglage (`HIGHS_THREADS`, plus forcé à 1 dans le catalogue) : les batches Python des
papiers prennent eux aussi le réglage de threads. Seules les vérifications Lean, une instance par
processus, restent sur un thread.

**Les temps dans les papiers** (Marc, 2026-11-13). Un papier cite le temps de chaque unité (une voie,
une famille, un modèle à un budget), d'un job lancé seul (`cell.pool` 1) sur une machine calme
(`timing_valid`), avec le matériel et ses threads (`threads.effective`) ; et pour une étape, la somme
de ces temps, qui ne dépend pas du nombre de jobs lancés à la fois. Jamais le temps mural d'un pool
parallèle. Les résumés fusionnés donnent les deux : les `seconds` propres des unités, et
`cells_seconds_sum` à côté de `wall_seconds` (le plus long job).
8. **La fin** : une ligne par statut (`72 jobs: 71 ok, 1 timed out`), les cases en échec nommées avec
   leur journal ; code de sortie 0 quand tous les jobs sont `Ok`, 1 sinon. Stop (Ctrl-C sans fenêtre)
   arrête les jobs en cours, notés `Stopped` ; les jobs pas encore lancés n'ont pas de fiche.

Sans fenêtre (§14), `lab run <batch>` écrit une ligne par job qui commence et qui finit
(`[37/72] ok 41.2 s  model=ogre budget=3000 ref=12a6675`), pas les journaux, qui sont plusieurs à la
fois. En R1, la fenêtre liste les batches dans le catalogue et leurs fiches dans l'historique ; R3 les
met en file (ci-dessous).

### La grille, les relances et la file (amendement R3)

- **Un batch dans la file.** L'écran File liste les batches à côté des jobs, et une séquence peut
  nommer des batches comme des jobs. Un batch est un élément de la file : à son tour, le lanceur
  l'exécute comme ci-dessus (dépendances, préparations, pool), puis passe à l'élément suivant. Son
  élément affiche `37/72` pendant qu'il tourne ; les règles du §8.4 s'appliquent après lui quand l'un
  de ses jobs n'a pas fini `Ok`. Arrêter termine les jobs en cours (`Stopped`), comme Ctrl-C sans
  fenêtre ; les jobs non commencés n'ont pas de fiche. Les lignes du lancement, celles qu'écrit
  `lab run`, vont dans `logs/<lancement>.log` : le journal que l'écran File montre pour lui.
- **L'écran Batches**, un sixième onglet : les batches à gauche ; pour celui choisi, sa grille. Une
  ligne par job, numérotée dans l'ordre de la grille : la valeur de chaque axe, son état, sa dernière
  exécution (une grille en tableau croisé se lisait comme un job par ligne). Une case
  montre son état : en cours (écoulé et estimation), en attente dans la file (son estimation), sa
  dernière fiche (`ok 41 s`, `failed`, `timed out`, le temps grisé quand il ne compte pas, §7.2), ou
  `—`, jamais lancée. Le survol donne son exécution, son commit, son calme et son estimation ; un clic la
  choisit, et la dernière case cliquée montre sa fiche et la fin de son journal sous la grille. La
  grille d'un lancement s'affiche dès son début ; au-dessus, ce que le lancement fait maintenant (les
  checkouts, une dépendance en cours, une préparation, l'attente du moniteur, les jobs) et chaque
  dépendance, en cache, en cours ou faite. Le journal de l'écran File suit le job qui tourne, une
  dépendance ou une préparation comprise, pas les lignes du lancement. Au-dessus de la grille, au repos
  comme pendant un lancement, la table des jobs dont le batch a besoin (R2) : une ligne par dépendance
  et valeur voulue (`source creature`), une colonne par référence, chacune sa dernière fiche et si son
  résultat est en cache ; un clic ouvre la fiche dans l'Historique. Pendant qu'une dépendance se fait,
  le panneau de l'écran File la nomme, avec son temps et la médiane de celles faites avant, puis le
  compte des jobs. Un job dont le journal reste muet 30 s (une compilation sous cmake, un solveur) reçoit
  de lab une ligne toutes les 30 s : depuis quand il tourne, depuis quand son journal se tait, les
  processus de son groupe (`cc1plus ×4, cmake`).
- **Choisir et relancer.** On choisit les cases une à une, ou « en échec », « jamais lancées »,
  « toutes », « aucune » ; un délai pour ce lancement (celui du batch par défaut) ; « Mettre la
  sélection en file » met le batch en file réduit à ces cases. Sans fenêtre, `--axis` réduit la grille
  comme en R1.
- **Un job déjà fait est sauté.** Une case dont la dernière fiche `Ok` a la même clé de commande (§10),
  le même commit, les mêmes versions d'outils et les mêmes clés de dépendances ne retourne pas : pas
  de nouvelle fiche, la ligne dit `done before` avec l'identifiant de cette fiche, et la grille
  continue de la montrer. Une fiche dont le temps ne compte pas est faite tout de même : son résultat
  est là. « Again » (`--again` sans fenêtre) les relance quand même. Un batch dont les jobs lisent des
  fichiers que leur commande, leur commit et leurs dépendances ne nomment pas (l'export du corps
  masculin dans `scratch/`) dit `skip_done = false` : ses jobs tournent toujours.
- **Une estimation par job** : la médiane des temps des fiches `Ok` du même batch à la même clé de
  commande et au même commit, sinon à la même clé de commande, sinon aux mêmes valeurs des axes.
  `lab run` la donne sur la ligne d'un job qui commence (`≈ 41 s`), et au début le total du lancement
  divisé par la largeur du pool ; la grille donne celle de chaque case et le temps restant du batch
  en cours.
- **Données** : un élément de file gagne `batch: Option<BatchChoice>`, `BatchChoice { cells:
  Vec<BTreeMap<String, String>> /* vide : toutes */, timeout: Option<Timeout>, again: bool }` ; il
  exécute le batch du catalogue que montre la fenêtre.

### Rapports (amendement R4)

- Un batch peut avoir un **`report`** : une commande, lancée dans un checkout comme un job, qui lit
  les fiches de ses cases et leurs fichiers, et écrit un tableau. Elle ne recalcule rien : relancée,
  elle donne le même tableau en quelques secondes. Marques : les intégrées, les paramètres du batch,
  `{records}` et `{out}` ; pas les axes.

  ```toml
  report = "python3 bench/before_after.py {records} --side ref --out {out}"
  ```

- **`lab report <batch>`** prend, pour chaque case de la grille, sa dernière fiche (`--launch <id>` :
  de ce lancement seulement), et les écrit dans `<lab>/reports/<batch>/<heure>/records.json` : le
  batch, ses axes, et par case ses valeurs, l'identifiant de sa fiche, son statut, ses secondes, si le
  temps compte, son commit, sa référence, et les chemins absolus de son dossier `{out}` et de ses
  fichiers (`null` pour une case jamais lancée). Puis il lance le rapport dans le checkout de ce
  dossier : le `checkout` du batch, ou `--ref <référence ou dossier>` ; son `{out}` est ce même dossier,
  sa sortie dans `report.log`, que `lab report` affiche. Un rapport qui dure plus de 10 minutes est
  arrêté. Pas de fiche : un rapport se refait à volonté à partir des fiches. Un lancement d'un batch
  qui a un rapport finit par lui, sans fenêtre comme dans la fenêtre (son tableau dans le journal du
  lancement) : une nuit de séquence laisse ses tableaux et ses résumés fusionnés prêts.
- L'écran Batches a un bouton **Report** à côté de la grille d'un batch qui en a un ; le tableau
  s'affiche sous la grille quand il est fait.
- Un seul script sert aux comparaisons, `bench/before_after.py` : l'axe comparé (`--side`, `ref` par
  défaut), le fichier que chaque job a écrit (`--file`, `measure.json` par défaut), les clés
  comparées, leur moyenne ou leur somme sur les autres axes sauf un (`--by`, `budget` par défaut) ; le
  temps est celui de lab, avec combien de ces temps comptent.

## 7. Règles que l'application fait respecter

### 7.1 Threads

- Un **réglage de threads** global, dans la barre du haut. Chaque tâche déclare son usage :
  `setting` (elle reçoit le réglage) ou `Fixed(n)` (elle en utilise n quel que soit le réglage,
  comme l'export qui prend toute la machine). Le nombre **effectif** est celui que les règles
  contrôlent.
- Pour chaque tâche, l'application pose `OPENCLARE_THREADS`, `RAYON_NUM_THREADS`,
  `RUST_TEST_THREADS`, `CARGO_BUILD_JOBS` et `HIGHS_THREADS` (depuis le 2026-11-13 : les scripts des
  papiers le donnent à HiGHS) au nombre effectif, et remplit `{threads}` (pour `-P`, `--jobs`,
  `JOBS=`). L'environnement propre d'une tâche les remplace.
- **Limite de confort.** Au-delà de 6 threads (réglable, §12), l'élément de file affiche une alerte
  ambre : la machine rame pendant ce temps. Elle ne bloque jamais : Marc décide quand il lance.
  L'application ne voit pas les threads des autres sessions de la machine : la charge étrangère du
  moniteur (§9) les montre.
- La file lance une tâche à la fois : un banc de comparaison et la suite de tests ne se chevauchent
  jamais quand les deux partent de l'application.

### 7.2 Temps qui compte

Une tâche `timing_matters = true` est de celles dont la durée entre dans un papier. Ce qui rend son
temps honnête, c'est une machine au repos, pas l'heure : Marc lance quand il sait qu'il ne s'en sert
pas, de jour comme de nuit. Proposition (Q2) :

- Elle **part tout de suite**, comme toute tâche (décision du 2026-11-09 : lab avertit, il n'empêche
  jamais un Run de démarrer). Une exécution commencée sur une machine non calme a `before = false`,
  donc `timing_valid = false`, sa raison notée (« pas calme au départ : CPU étranger 18 % »), et le
  bandeau de messages le dit.
- Avec `wait = true` sous `[calm]` dans les réglages, elle attend en tête de file que la machine soit
  calme (§9) : l'élément affiche « attend le calme depuis 15:04 (CPU étranger 18 %) », elle part quand
  la machine a été calme sur toute la fenêtre, et les autres tâches de la file ne l'attendent que si
  elles viennent après elle. « Lancer quand même » la lance tout de suite : `forced = true`, donc
  `timing_valid = false`, l'exécution compte, son temps non.
- Toute exécution, quelle que soit la tâche, reçoit son verdict `Calm` ; un temps n'est rapporté que
  d'une exécution `timing_valid = true`.
- Les écrans disent de quel cas il s'agit. Un chronomètre ⏱ se tient à côté du nom d'une tâche dont le
  temps compte, partout où la tâche est montrée (État, File, Historique, Catalogue, l'exécution en
  cours, l'attente du calme) ; une tâche sans lui tourne sous n'importe quelle charge et sa durée est un
  ordre de grandeur, pas une mesure. Pour une tâche au chronomètre, la durée d'une exécution n'est en
  clair que si `timing_valid` : grisée quand son calme n'a pas été mesuré (exécution lue dans le dépôt ou
  saisie à la main), barrée quand la machine n'était pas calme ou que l'exécution a été forcée, la raison
  au survol.

### 7.3 Préparation avant la tâche

- La préparation tourne avant la tâche, dans le même checkout, avec le même réglage ; sa durée et son
  statut sont gardés dans `prepare`, à part de la durée de la tâche.
- La même préparation sur le même checkout tourne une fois pour toutes les files : elle écrit sa commande
  dans `.lab-prepared` à la racine du checkout, et est sautée tant que ce fichier la nomme
  (`skipped_because = "faite pour forced-bounds le 2026-11-06"`). Un checkout local, qui peut changer
  entre deux files, la fait tourner une fois par file.
- Une préparation échouée fait échouer la tâche ; sa commande ne tourne pas.
- Ainsi `cargo build --release` précède toujours un banc : `cargo test --release` ne refait pas
  `target/release/openclare`, et les tâches de banc portent la construction dans leur préparation.

### 7.4 Plafond mémoire

`memory_limit_gb` pose `RLIMIT_AS` (souple et dur) dans l'enfant avant `exec`, comme `ulimit -v`. Les
valeurs amorcées suivent les règles du projet : 32 Go pour le moteur, Blender et Lean, 40 Go pour
`cargo test`, les valeurs de NIGHT.md quand il en donne.

## 8. Le lanceur

### 8.1 Lancement

L'application récupère le miroir, résout la référence, ajoute le checkout de ce commit s'il n'existe pas
(`git worktree add --detach <lab>/checkouts/<hash court> <hash>`), écrit `state.json` (la file, l'élément
lancé), prend l'échantillon de début (§9), lit git (`rev-parse HEAD`, `status --porcelain
--untracked-files=no`, la référence comme `branch`, `pushed` par `branch -r --contains` et
`tag --contains`), puis lance `bash -c "<commande>"` dans ce checkout :

- dans un nouveau groupe de processus (`setpgid(0, 0)` dans `pre_exec`), avec le plafond mémoire ;
- sortie standard et d'erreur redirigées vers `<lab>/logs/<id>.log`, ouvert par l'application et
  passé à l'enfant : le journal continue de s'écrire si l'application meurt ;
- entrée standard sur `/dev/null` ;
- l'environnement hérité, plus les variables du §7.1 et celles de la tâche.

Une référence que le miroir ne connaît pas (une branche supprimée après sa fusion, une faute de frappe)
n'arrête pas l'élément : il tourne sur `main`, le bandeau de messages et le journal disent quelle
référence était demandée et quel commit a tourné, la fiche nomme `main`, et l'élément de la file
affichait « unknown, main instead » avant. lab avertit, il n'empêche jamais une exécution de démarrer
(2026-11-09).

Le journal défile dans l'application : les 2 000 dernières lignes, rafraîchies toutes les demi-secondes,
couleurs ANSI retirées, recherche dans le journal.

### 8.2 Fin d'exécution

Quand le shell se termine : échantillon de fin, statut tiré du code de retour ou du signal, fichiers
produits (les motifs de la tâche, plus les fichiers suivis que `git status` montre modifiés entre début
et fin, chacun avec taille et SHA-256), fiche ajoutée à `runs.jsonl`, élément suivant lancé. Les
processus du groupe encore vivants après la fin du shell sont notés dans le journal et tués (§8.3).

### 8.3 Arrêt

Le bouton Arrêter envoie SIGTERM au groupe de processus, puis SIGKILL au groupe après 10 secondes
(réglable). La fiche reçoit `Stopped`. Le sort du reste de la file est un réglage (Q3) : **suspendre et
demander** (proposé par défaut : « Continuer la file / Annuler le reste »), ou annuler le reste.

### 8.4 Échec

Une tâche échouée (code non nul, ou signal que l'application n'a pas envoyé) : même réglage que l'arrêt,
séparé (Q3) : suspendre et demander, continuer, ou annuler le reste.

### 8.5 Fermeture de la fenêtre, plantage

- Fermer la fenêtre pendant une exécution demande : « Arrêter l'exécution et quitter » ou « Garder la
  fenêtre ouverte » (par défaut). Pas de mode détaché : la fiche a besoin de l'application vivante (Q4).
- Si l'application meurt (plantage, déconnexion, redémarrage), `state.json` dit quelle exécution était
  en cours. Au démarrage suivant l'application cherche le groupe de processus : s'il a disparu,
  l'exécution est notée `Interrupted`, sa fin à la dernière modification du journal ; s'il vit encore,
  l'application propose de l'attendre (fin connue, code de retour non : `Interrupted` aussi) ou de le
  tuer. Le reste de la file s'affiche suspendu.

## 9. Moniteur et indicateur de calme

Un fil d'échantillonnage lit toutes les 2 secondes (réglable, 1 à 5) : CPU % total et par thread,
moyennes de charge 1/5/15 et mémoire dans /proc (Linux seulement ; `sysinfo` n'apporterait rien ici) ; VRAM et GPU % par
`nvidia-smi --query-gpu=memory.used,memory.total,utilization.gpu --format=csv,noheader,nounits` (toutes
les 4 secondes ; « n/a » si la commande manque ou échoue) ; et le CPU de chaque processus. Les processus
de l'exécution sont ceux de son groupe et leurs descendants (par pid parent, ce qui attrape aussi les
enfants sortis du groupe) ; le **CPU étranger** est le reste. Les 10 dernières minutes restent en
mémoire pour les courbes et la fenêtre de l'indicateur.

L'indicateur, toujours visible dans la barre du haut :

| État | Quand | Texte |
|---|---|---|
| vert **machine calme** | aucune exécution à nous, et sur les 60 dernières s : CPU étranger moyen < 5 %, chaque échantillon < 25 %, load1 < 1,0 | « calme depuis 14 min » |
| ambre **occupée par notre run** | une exécution à nous, et sur les 60 dernières s : CPU étranger moyen < 5 %, chaque échantillon < 25 % | « forced-whole-lower, 4 threads » |
| rouge **chargée par autre chose** | les seuils étrangers sont franchis, exécution ou non ; ou aucune exécution et load1 ≥ 1,0 | « lake 212 %, cargo 95 % » (les trois processus étrangers les plus lourds) |

Pendant les 60 secondes qui suivent l'ouverture, l'indicateur est gris (« mesure en cours ») : la fenêtre
n'est pas pleine.

La moyenne de charge décroît en minutes et compte notre propre exécution : juste après la fin d'une
exécution à nous, load1 reste haut un moment alors que rien d'autre ne tourne. Load1 n'est donc contrôlé
que si aucune exécution à nous ne s'est terminée dans les 5 dernières minutes (réglable) ; sinon le CPU
étranger décide seul. Les CPU % sont des pourcentages de toute la machine (8 threads : 12,5 % est un
thread occupé).

Seuils, fenêtre et période sont dans `settings.toml` et modifiables dans l'écran Machine ; les valeurs
ci-dessus sont une première proposition, à régler sur de vraies exécutions (Q5).

## 10. Estimation de la progression

La **clé de commande** d'une exécution est sa commande rendue, l'environnement de sa tâche et son dossier
(le checkout exclu). L'estimation d'une tâche est la médiane des durées de ses exécutions `Ok` :

1. de même clé et de même commit ;
2. sinon de même clé, à tout commit ;
3. sinon sur toutes les exécutions de la tâche, résultats du dépôt compris (section 13) ;
4. sinon « pas d'estimation ».

Une estimation dit toujours sur quoi elle repose : « ≈ 2 h 38 min 24 · 1 exécution, charge 0,43 ». Les
exécutions sous charge servent, et le disent : ce sont souvent les seules (Marc, 2026-11-06).

La file montre, pour chaque élément, son estimation, le **cumul** depuis le début de la file et son heure
de fin prévue. Un élément sans estimation fait lire le cumul « au moins …, plus n tâches sans
estimation ». Pendant une exécution : écoulé / estimé et une barre ; au-delà, « dépasse l'estimation de
12 min ». Une exécution terminée est un point de plus pour l'estimation suivante.

## 10 bis. Versions : sur un tag ou hors tag

- Une exécution se range par **un seul hash** : celui qu'elle note. C'est le commit du checkout pour une
  exécution lancée par lab, le commit écrit dans le résumé pour un résultat du dépôt ; pour un résultat qui
  n'en note aucun, le commit qui l'a apporté dans git (`git log -1 -- <fichier>`).
- Ce hash est celui d'un tag (`git tag -l 'v*'` dans le miroir) : l'exécution est **sur ce tag** et peut
  étayer un papier. Sinon elle est **hors tag**, montrée avec son hash, et ne peut pas étayer un papier.
- Ce hash n'est atteignable depuis aucune branche ni aucun tag du dépôt distant (`pushed = false`) :
  l'exécution est **orpheline**, montrée « orphan <hash> », et ne peut pas non plus étayer un papier :
  personne ne peut extraire ce qui a tourné. Une exécution d'un checkout local est orpheline tant que
  son commit n'est pas poussé.
- Une exécution dont la durée n'a pas été mesurée affiche « no time ».

## 11. Écrans

Une fenêtre, une barre du haut et cinq onglets. Libellés en anglais comme le code ; des libellés
français sont possibles par un réglage (Q11). Disposition, d'après la maquette :

### Barre du haut

Le miroir (heure de la dernière récupération, un bouton Fetch), les fiches pas encore poussées (un bouton
Push, §4 bis), l'indicateur de calme, CPU et load1
d'un coup d'œil, le réglage de threads (de 1 à 8, ambre au-delà de la limite de confort).

### État (l'écran d'ouverture)

- Une ligne par tâche, groupées par papier, le chronomètre ⏱ à côté d'une tâche dont le temps compte,
  avec sa **dernière exécution** (Last run) : marquée de son tag (« v0.7.0 ») ou « off-tag <hash> », puis
  sa date, sa durée (« no time » si non mesurée ; grisée ou barrée quand le temps de la tâche compte et
  que celui-ci n'est pas valide, §7.2), la charge au début et le fichier où elle a été lue ; ou
  « never run ».
- En tête, les comptes : dernière exécution sur un tag, hors tag, orpheline, jamais lancée.
- Les exécutions montrées sont l'union des mémoires des machines (§4 bis) et des résumés du dépôt (§13),
  lus dans le miroir : l'écran ne dépend d'aucun checkout. Un filtre **Machine** garde celles d'une
  machine.
- Un filtre **Tag** : « All runs » (la dernière exécution de chaque tâche, où qu'elle ait tourné), ou un
  tag : seule la dernière exécution de chaque tâche sur ce tag, les autres « no run on this tag ».
- Les tâches cochées ici, ou toutes celles jamais lancées, partent dans la file d'un clic.

### File

- **À gauche, le catalogue** : tâches groupées par catégorie, une recherche, pour chacune une case, son
  estimation, le chronomètre ⏱ si son temps compte ; les séquences au-dessus (en cocher une ajoute ses
  tâches dans l'ordre).
- **Au milieu, la file** : les éléments dans l'ordre, réordonnés par une **poignée** de glisser-déposer,
  chacun avec ses paramètres (modifiables), sa référence (un tag, une branche, une pull request, un hash, ou un
  dossier local ; si l'exécution sera sur un tag, hors tag ou orpheline ; si ce commit a déjà un checkout
  préparé), ses threads effectifs, son estimation, son cumul et son heure de fin (section 10) ; dessous : le commentaire et la version de papier (A ou B, un libellé suggéré d'après les
  libellés passés) appliqués à toutes les exécutions de la file, les politiques d'arrêt et d'échec, l'heure de fin de la file, Lancer.
- **À droite, l'exécution en cours** : tâche, élément n sur m, écoulé / estimé avec la barre, heures de
  fin de la tâche et de la file, Arrêter, le journal qui défile.

### Historique

- Un tableau de toutes les exécutions : date, tâche (avec le chronomètre ⏱ quand son temps compte),
  durée (grisée ou barrée quand ce temps n'est pas valide, §7.2), commit (court, marque de
  modification), threads, calme (✓, ✗ avec la raison en info-bulle, « forcée »), statut, commentaire,
  papiers. Tri par
  toute colonne ; filtres par tâche, commit, papier et libellé, dates, statut, « temps valides
  seulement ».
- Un panneau de détail de l'exécution choisie : commande entière, environnement, paramètres, checkout et
  dossier, machine, charge au début, pendant et à la fin, fichiers produits avec leurs empreintes, le
  journal (dans le panneau, ou dans l'éditeur du système) ; le commentaire et les papiers s'y modifient.
- « Exporter la sélection » : tableau Markdown ou CSV, vers le presse-papiers ou un fichier. Le Markdown
  porte la ligne de matériel qu'il faut aux papiers, tirée des exécutions : « Intel Core i7-7700K
  (4 cœurs, 8 threads, 2017, 62 Go), au repos (load1 0,12 au début, CPU étranger sous 5 %) », et dit
  quand la sélection mêle des machines, des charges ou des commits.
- « Saisir une exécution passée » : un formulaire pour une exécution que l'application n'a pas lancée
  (un temps tiré d'une description de PR) : `source = Manual`, les champs que Marc connaît, le reste vide.
- « Copier l'historique dans le dépôt » (§4).

### Catalogue

La liste des tâches et des séquences, un formulaire pour modifier celle choisie (chaque champ de `Job`),
la commande rendue avec le réglage courant en aperçu, « Dupliquer », « Nouvelle », « Supprimer »
(l'historique garde les exécutions d'une tâche supprimée). Enregistrer réécrit `jobs.toml` en gardant
l'ordre des tâches ; les commentaires du fichier sont perdus quand il est enregistré depuis l'application
(Q10).

### Machine

CPU total et par thread (barres et courbe sur 10 minutes), charge 1/5/15, mémoire utilisée / totale, VRAM
et GPU %, les processus les plus lourds marqués « à nous » ou « étranger », l'indicateur avec sa raison,
les seuils modifiables ; les checkouts du miroir avec leur taille, et un bouton Clean qui retire ceux hors
de tout tag avec leur `target/` et leur `.lake/` (rien n'est retiré tout seul, Q16).

## 12. Réglages

`<lab>/settings.toml` : le dépôt distant (`marcgardent/openclare`), le nom de la machine (le nom d'hôte
par défaut : la branche `lab/runs-<machine>` et les exports), la politique de push (`ask`, `auto`,
`never`), la limite de confort des threads (6,
Q1), seuils
et fenêtre du calme, si une tâche au chronomètre attend le calme (`wait`, non par défaut), période
d'échantillonnage, délai entre SIGTERM et SIGKILL, politiques d'arrêt et
d'échec, libellé de la machine pour les exports (proposé : « i7-7700K 4c/8t 2017, 62 GB », comparé à ce qui
est détecté : l'application avertit si le modèle de processeur ou la mémoire diffèrent).

## 13. Les résultats du dépôt dans l'historique

À chaque ouverture et après chaque récupération, lab lit les résumés que `main` contient dans le
miroir (`git show`, `git log -1 main -- <fichier>`, jamais un arbre de travail) et montre chacun comme une exécution
« du dépôt », jamais recopiée dans `runs.jsonl`. Une tâche nomme son résumé dans `jobs.toml` (`summary`, un
chemin, avec `#clé` pour un enregistrement niché dans un autre fichier, comme l'export dans les résumés des
papiers). Un résumé donne : le hash qu'il note (sinon le commit qui l'a apporté, section 10 bis), la date de
ce commit (l'heure n'est pas connue), `wall_seconds` ou `seconds`, le processeur et la mémoire, les moyennes
de charge au début et à la fin. Son calme ne peut pas se savoir (rien d'autre n'a été échantillonné) :
`timing_valid = false`, raison « non mesuré (du dépôt) ». `bench/results/*.json` et quelques autres ne notent
aucune durée : « no time ».

## 14. Choix techniques

- Rust 1.95 (la chaîne d'ici), édition 2024.
- `eframe` / `egui` 0.36 avec `egui_extras` (tableaux) et `egui_plot` (courbes) : le mode immédiat convient
  à un tableau de bord redessiné chaque seconde, et il est mûr sous Linux (X11 et Wayland). Le cache cargo
  d'ici contient `egui` 0.36.2, `egui_extras` 0.36.2 et `egui_plot` 0.37.0, mais pas `eframe` : la
  première construction le télécharge avec ses dépendances depuis crates.io (Q12).
- CPU, mémoire, charge et processus lus dans /proc ; `nix` 0.31 (groupes de processus, signaux,
  `setrlimit`), `serde`, `serde_json`, `toml`, `chrono`, `sha2`, `glob`, `dirs`. Le GPU par la commande
  `nvidia-smi`, sans liaison NVML.
- Fils : l'interface ; l'échantillonnage ; le lanceur (lancement, attente, fin d'exécution) ; une
  surveillance de `jobs.toml` (date de modification lue toutes les 2 s, sans crate `notify`). État partagé
  derrière un `Mutex`, interface redessinée chaque seconde et à chaque événement.
- Modules purs sans egui (fiches, catalogue, rendu des commandes, estimation, verdict de calme, export),
  avec tests unitaires ; l'interface les appelle.
- Constructions légères : `CARGO_BUILD_JOBS=4` pendant le développement.

**Installation.** `make lab-install` à la racine du dépôt : `cargo install --path lab --root ~/.local`
met le binaire dans `~/.local/bin/lab` ; `lab/openclare-lab.desktop` va dans
`~/.local/share/applications/` et `site/assets/icon.svg` dans
`~/.local/share/icons/hicolor/scalable/apps/openclare-lab.svg`, KDE liste alors « openclare lab » dans
son menu ; `update-desktop-database` tourne s'il est présent. `make lab-uninstall` retire les trois. Rien
n'est installé hors du dossier personnel.

**Sans fenêtre, batches (R1).** `lab run <batch> [--axis <nom>=<valeur>,…]… [--timeout <durée>]
[--threads <n>]` lance un batch (§6 bis) ; `--ref` donne la référence d'un batch sans axe `ref` ;
`--axis` et `--timeout` ne valent que pour un batch, le délai d'un job étant son `timeout` dans
`jobs.toml`.

**Sans fenêtre.** `lab run <tâche> [--ref <référence>] [--force] [--threads <n>]` lance une tâche sans
fenêtre, avec le même lanceur, le même moniteur et les mêmes fiches, son journal sur la sortie standard,
et sort avec le statut de la tâche ; la référence est par défaut celle de la tâche, `--force` la lance
bien que la machine ne soit pas calme quand le réglage `wait` la fait attendre (son temps ne compte alors
pas). `lab fetch` et `lab status` font
ce que leur nom dit ; `lab push` récupère puis pousse la mémoire vivante vers la branche de cette machine
(§4 bis), quoi que dise le réglage `push`, et `lab run … --push` le fait après l'exécution. Un timer
systemd ou un shell sur une autre machine peuvent les appeler (Q4 du §17 répondue). La fenêtre reste
l'endroit où l'on regarde.

**CI.** Un workflow `lab.yml`, lancé seulement quand `lab/**` ou lui-même change (même filtre `paths` et
même tâche `changes` que `lean.yml`) : `cargo build --release --locked` et `cargo test --locked` dans
`lab/`, sur `ubuntu-24.04` avec les bibliothèques système qu'il faut à eframe. Il coûte quelques minutes
du temps de GitHub, rien à la machine, et garde l'application constructible quand Rust ou une dépendance
bouge. `hygiene.yml` couvre déjà `lab/runs.jsonl`.

## 15. Jalons

Chacun est un ou quelques commits sur la branche, puis un arrêt pour la revue de Marc.

| | Contenu | Vérifié par |
|---|---|---|
| M1 | l'application s'ouvre, lit `jobs.toml` (amorcé avec le §6) et `runs.jsonl`, montre le catalogue et le tableau d'historique ; `settings.toml` | l'ouvrir ; tests unitaires de lecture et de rendu |
| M1b (facultatif) | `lab seed` depuis les résumés de résultats | les lignes amorcées contre les fichiers JSON |
| M2 | file, lancement et arrêt, journal qui défile, préparation, plafond mémoire, fiches écrites, `state.json` et reprise | une file de `sleep 5`, `false`, un `sleep 60` arrêté, l'application tuée pendant une exécution |
| M3 | échantillonnage, écran Machine, indicateur de calme, `Calm` dans les fiches, les règles du §7 | l'indicateur observé pendant qu'un `yes > /dev/null` tourne hors de l'application |
| M4 | estimations et heures de fin | tests unitaires sur des historiques inventés ; une file de `sleep` lancée deux fois |
| M5 | détail de l'historique, modification, export Markdown et CSV, saisie d'une exécution passée, copie dans le dépôt, `lab/README.md` et `README.fr.md`, CI ; la PR sur le mot de Marc | un export collé dans un tableau de brouillon |

État au 2026-11-06 : jalons M1 à M5 faits, avec l'écran État et la règle des tags (sections 10 bis, 11,
13) demandés après M2, et les résumés du dépôt lus à chaque ouverture à la place de M1b. Pas fait :
la modification du catalogue dans l'application (modifier `jobs.toml` à la main), les libellés français
(Q11).

L'amendement du 2026-11-08 en ajoute trois, une PR chacun, ce document d'abord :

| | Contenu | Vérifié par |
|---|---|---|
| M6 | `~/.openclare-lab` comme dossier unique, les fichiers des anciens dossiers déplacés une fois ; `make lab-install` et `lab-uninstall`, l'entrée de bureau et l'icône | l'entrée du menu KDE ouvre lab avec l'historique toujours là |
| M7 | le miroir, un checkout par commit, préparations une fois par checkout, `{ref}`, `pushed`, exécutions orphelines ; le catalogue, les tags et les résumés lus dans le miroir, plus aucun clone nécessaire ; les checkouts et le bouton Clean dans l'écran Machine ; `lab run`, `fetch`, `status` sans fenêtre | une tâche en file sur `v0.7.0` et sur `main` ; `lab run` depuis un shell, fenêtre fermée |
| M8 | les branches `lab/runs-<machine>`, push selon la politique, `lab push` et `lab run --push`, lecture par union ; le filtre Machine de l'écran État | deux mémoires (un second nom de machine avec `--data`) poussées et relues ; l'écran identique depuis les deux dossiers |

État au 2026-11-08 : M6, M7 et M8 faits ; `lab push` est passé de M7 à M8, où sont les branches qu'il
pousse.

L'amendement R1 du 2026-11-13 commence la réforme des jobs et des batches (§6 bis), quatre jalons,
chacun sa revue, le batch du ticket #115 servant de témoin à chacun :

| | Contenu | Vérifié par |
|---|---|---|
| R1 | `[[batch]]`, la grille développée en jobs, le pool, une fiche par job, un délai par job, les versions des outils ; sans fenêtre | `lab run pole-pairs-115-remesh` : 72 fiches, ogre à 3 000 quads sur 12a6675 en dépassement s'il passe 30 minutes, les autres faits |
| R2 | les dépendances comme jobs, en cache par clé : le moteur d'un commit, la source d'une scène, partagés entre batches | un second lancement ne compile ni n'exporte rien |
| R3 | la grille dans la fenêtre (lignes, colonnes, l'état d'une case, son journal), la sélection de cases et leur relance ; un job déjà fait (même clé de commande, commit et outils) sauté sauf si on force ; une estimation par job | ogre seul relancé avec un délai plus long |
| R4 | les rapports : un script court lit les sorties des jobs et fait le tableau, relancé en quelques secondes | le tableau avant et après de #115, rien recalculé |

État au 2026-11-13 : R1 codé (`src/batch.rs`, `[[batch]]` dans `src/catalogue.rs`, `src/tools.rs`),
tests unitaires sur des grilles inventées ; vérifié à la main sur 4 jobs du témoin, les 72 laissés à
Marc. R2 codé (`[[dependency]]` dans `src/catalogue.rs`, le cache dans `src/batch.rs`), tests unitaires
sur une dépendance inventée ; le témoin réduit à blob à 3000 quads sur main et 12a6675 : le premier
lancement a compilé deux moteurs et exporté une source (la scène est la même aux deux commits), le
second n'a rien compilé ni exporté. R3 codé (`src/app/batches.rs`, l'élément batch de `src/runner.rs`, le
saut et les estimations dans `src/batch.rs`) ; un test unitaire passe un élément batch de la file sur des
cases choisies, puis à nouveau (sautées) ; l'écran Batches regardé sur les fiches du témoin et sur un
batch inventé en cours. Le témoin de R3, ogre relancé seul avec un délai plus long depuis la grille, est
laissé à Marc. R4 codé (`src/report.rs`, `lab report`, le bouton Report, `bench/before_after.py`) ; un
test unitaire fait deux fois le rapport d'un batch inventé, même tableau et pas de fiche ; le rapport du
témoin fait à partir des fiches des essais de R1 et R2 (blob et ogre à 3000 quads). Le tableau complet
de #115 attend le lancement des 72 jobs par Marc.

## 16. Ce qui ne peut pas être vérifié ici

- **VRAM** : `nvidia-smi` répond sur cette machine (GeForce RTX 5070, 12 227 Mio, lu le 2026-11-06), la
  requête du §9 fonctionne donc ; le GPU % par processus n'est pas lu, la charge GPU n'est donc pas
  partagée entre nous et l'étranger.
- **Threads d'une tâche** : l'application pose les variables ; que chaque script les respecte n'est pas
  vérifié (Q8, Q9).
- **Coût de l'application** : mesuré le 2026-11-06 à 15 % d'un thread (environ 2 % de la machine), fenêtre
  ouverte sur l'onglet Machine, machine chargée par d'autres travaux ; il compte comme CPU étranger.
- **Seuils du calme** : une première proposition, à régler sur de vraies exécutions.
- **Wayland ou X11** : la session de Marc n'est pas vérifiée ; eframe gère les deux.

## 17. Questions ouvertes pour Marc

1. **Limite de confort** : une alerte ambre au-delà de 6 threads, jamais bloquante : suffisant ?
2. **Temps qui compte, machine pas calme** : attendre le calme en tête de file (proposé), ou seulement
   avertir et lancer ? Répondu le 2026-11-09 : avertir et lancer, le temps ne comptant pas ; l'attente
   est le réglage `wait` du calme, non par défaut (§7.2).
3. **Reste de la file après Arrêter, après un échec** : suspendre et demander (proposé pour les deux),
   annuler, continuer ?
4. **Fermer la fenêtre pendant une exécution** : demander (proposé) sans mode détaché, ou un lanceur sans
   fenêtre `lab run` qui survit à la fenêtre (un jalon plus tard) ?
5. **Seuils du calme** : CPU étranger moyen < 5 % et pic < 25 % sur 60 s, load1 < 1,0 : bons ordres de
   grandeur ?
6. **Veille** : la machine se met-elle en veille quand personne n'y touche ? Si oui, l'application doit la tenir
   éveillée (`systemd-inhibit`) pendant une file de plusieurs heures.
7. **Estimations tirées d'exécutions chargées** : montrées mais hors de l'heure de fin (proposé), ou
   utilisées avec une marque ?
8. **Threads du banc** : combien de threads utilise `bench/run.py` (Blender, le moteur, les autres
   remailleurs) ? `OPENCLARE_THREADS` est-il respecté dans Blender par le greffon ?
9. **Threads de Lake** : `lake build` ne prend pas `-j` à ma connaissance ; faut-il plafonner la
   construction Lean, et comment ?
10. **jobs.toml** : modifié surtout à la main (l'application garde alors ses commentaires en l'éditant par
    `toml_edit`), ou surtout dans l'application (simple réécriture) ?
11. **Langue de l'interface** : anglais comme le code, ou français ?
12. **Téléchargement de crates** : la première construction tire `eframe` et ses dépendances de crates.io.
    Permis ?
13. **Base vivante hors du dépôt, instantané dans git** (§4) : d'accord, ou la base elle-même suivie dans le
    checkout d'attache ?
14. **Nom** : `lab` pour le dossier et le binaire, `openclare-lab` pour le paquet ?

Répondues par l'amendement du 2026-11-08 : Q4 (`lab run` sans fenêtre, M7) et Q13 (la mémoire vivante
hors du dépôt, réunie dans git, §4 bis). Nouvelles :

15. **Push** : `ask` par défaut, un bouton avec le nombre de fiches pas encore poussées ; ou `auto` après
    chaque exécution ?
16. **Checkouts gardés** : ceux des tags toujours, les cinq derniers autres commits, le reste retiré avec
    son `target/` et son `.lake/` à la fin d'une file : d'accord ? Un `.lake/packages` partagé par un lien
    entre checkouts de même toolchain, pour épargner le cache Mathlib ?
17. **Nom de la machine** : le nom d'hôte, ou un nom posé à la main (`i7-7700K`) ?
18. **Machine de nuit** : une clé de déploiement limitée au dépôt, en écriture, pour ses push ?
19. **Où tout habite** : le miroir et les checkouts dans `~/.openclare-lab` à côté de la mémoire, plusieurs
    Go ; ou la mémoire seule là et les checkouts ailleurs (`--checkouts <dossier>`) ?

Répondues le 2026-11-08 : Q15 `ask` ; Q16 tout gardé, un bouton Clean dans l'écran Machine ; Q17 le nom
d'hôte ; Q18 pas de machine de nuit, la clé de Marc seule ; Q19 tout dans `~/.openclare-lab`.

Décidé le 2026-11-13 (la proposition de réforme) : remaillage et mesure font un seul job ; un job est
coupé après 30 minutes par défaut, chaque batch peut changer ce délai ; « job » et « batch » dans
l'interface ; la réforme dans une session et une branche à elle. Nouvelles, répondues le même jour
comme proposé :

20. **Délai d'un `[[job]]`** : aucun par défaut, car beaucoup des jobs d'aujourd'hui sont des batches
    entiers dans un script (`lean-oracle-doubles`, environ 18 h d'un cœur ; `forced-whole-lower`, 1 à
    4 h) que 30 minutes tueraient.
21. **Pool et temps** : six jobs à un thread sur 4 cœurs et 8 threads se ralentissent l'un l'autre ;
    le pool remplit le réglage, `cell.pool` est noté, et une comparaison reste juste parce que ses deux
    côtés tournent entrelacés dans le même pool. Revu par Marc plus tard le 2026-11-13 : un job à la
    fois pour un batch dont le temps compte ; les autres remplissent le pool.
22. **Version de Lean** : la chaîne nommée par `lean-toolchain`, rien n'est lancé.

Nouvelle avec R2 :

23. **Le cache** : gardé jusqu'à ce qu'on le retire à la main (proposé ; un moteur pèse environ 20 Mo
    par commit), ou nettoyé comme les checkouts, par un bouton de l'écran Machine ?
