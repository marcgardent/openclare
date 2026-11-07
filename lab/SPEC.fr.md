# lab : le registre et le lanceur des calculs d'openclare

État : **brouillon à valider**. Aucun code n'est écrit avant que Marc valide ce document.
Version anglaise de référence : `SPEC.md`.

## 1. Objet

Tout calcul long du projet (bancs, suites de tests, constructions et oracle Lean, expériences des
papiers) se lance depuis une seule application de bureau, qui garde une fiche par exécution : ce qui
a tourné, sur quel commit, sur quelle machine, sous quelle charge, combien de temps, et quelle
version de quel papier elle étaye. Les temps cessent de vivre dans des journaux, des descriptions de
PR et des notes de mémoire.

L'application fait six choses : elle tient la base des exécutions (§4), le catalogue des tâches (§6),
une file qui les lance une par une (§8), une estimation de progression tirée de l'historique (§10), un
moniteur de la machine avec un indicateur « calme » (§9), et une vue d'historique avec export (§11).

Hors champ : lancer sur une autre machine, lancer deux tâches à la fois, programmer des tâches
récurrentes, lire les résultats d'une exécution (les résumés des scripts restent ce qu'ils sont).

## 2. Nom et emplacement

- Dossier `lab/` à la racine du dépôt, son propre espace de travail Cargo (son `Cargo.toml`, son
  `Cargo.lock` et son `target/`), hors de l'espace de travail du moteur : les constructions du moteur
  et la CI ne changent pas.
- Paquet `openclare-lab`, binaire `lab`. Ouvert par `cargo run --release` depuis `lab/`.
- Le **checkout d'attache** est le clone où se trouve `lab/`. Le catalogue y est lu. Les tâches
  tournent dans ce checkout ou dans l'un de ses worktrees (`git worktree list`).

## 3. Fichiers

| Fichier | Suivi | Écrit par | Contenu |
|---|---|---|---|
| `lab/jobs.toml` | oui | l'application et Marc à la main | le catalogue (§6) |
| `lab/runs.jsonl` | oui | l'application, à la demande de Marc (« Copier l'historique dans le dépôt ») | l'instantané publié de l'historique |
| `<data>/runs.jsonl` | non | l'application | la base vivante, une exécution par ligne |
| `<data>/logs/<id>.log` | non | la tâche (sa sortie standard et d'erreur) | le journal de chaque exécution |
| `<data>/refused.jsonl` | non | l'application | les fiches qui nomment encore un chemin local (section 4) |
| `<data>/state.json` | non | l'application | la file en cours et l'exécution en cours, pour la reprise |
| `<config>/settings.toml` | non | l'application | seuils, alerte de threads, politiques, libellé de la machine (§12) |

`<data>` est `$XDG_DATA_HOME/openclare-lab` (par défaut `~/.local/share/openclare-lab`), `<config>`
est `$XDG_CONFIG_HOME/openclare-lab` (par défaut `~/.config/openclare-lab`). L'application montre les
deux chemins dans sa boîte « À propos ».

## 4. Stockage

**Format : JSON Lines, une exécution par ligne, triées par heure de début.** Quelques centaines de
lignes tiennent en mémoire et se chargent en millisecondes ; une ligne se lit avec `jq`, en Python
sans bibliothèque, et dans un diff. SQLite par `rusqlite` apporterait des index et des transactions
dont ce volume n'a pas besoin, et un fichier binaire que git ne sait ni comparer ni fusionner.
Modifier un commentaire réécrit le fichier de façon atomique (écrit à côté, puis renommé) : le diff
git de l'instantané est cette seule ligne.

**Le fichier vivant est hors de l'arbre de travail ; git en garde un instantané.** Si la base
vivante était `lab/runs.jsonl` du checkout d'attache, le checkout principal porterait sans cesse une
modification locale : `git pull` refuserait de passer chaque fois qu'une PR change le fichier, et le
drapeau `dirty` de toute exécution lancée là serait vrai à cause de la base elle-même. L'application
écrit donc `<data>/runs.jsonl`, un seul fichier pour la machine quel que soit le checkout ou le
worktree où tourne une tâche, et « Copier l'historique dans le dépôt » écrit le fichier entier, trié,
dans `lab/runs.jsonl` du checkout que Marc choisit, à commettre avec la PR qui cite les exécutions.
`lab/.gitattributes` déclare `runs.jsonl merge=union` : deux branches qui ajoutent chacune des
exécutions fusionnent sans conflit ; le chargement garde ensuite une ligne par identifiant (celle dont
`edited` est le plus récent). L'instantané suivi sert aussi de sauvegarde du fichier vivant ;
l'application garde en plus les cinq dernières versions du fichier vivant (`runs.jsonl.1` à `.5`).

**Aucun chemin absolu n'est écrit dans une fiche.** Les chemins sont relatifs à la racine du checkout
où la tâche a tourné, et ce checkout est nommé relativement au checkout d'attache (`.` ou
`.claude/worktrees/nom`). Avant d'écrire, l'application remplace la racine du checkout par `{repo}` et
le dossier personnel par `~` dans la commande, l'environnement et les chemins, puis refuse d'écrire une
fiche qui correspond encore aux motifs de dossier personnel de `hygiene.yml` (Linux, macOS et Windows) ou qui
contient `/tmp/`, et montre le champ fautif ; cette fiche est gardée dans `<data>/refused.jsonl`,
jamais copiée dans le dépôt. L'environnement enregistré est celui des variables que
l'application pose (§7), pas tout l'environnement hérité.

## 5. Modèle de données

Les types Rust sont ceux de `SPEC.md` §5, identiques ; seuls les commentaires sont traduits ici.

- `Run` : une exécution, une ligne de `runs.jsonl`. Champs : version du format, identifiant
  (`20261106-221403-forced-whole-lower`), nom de la tâche au moment de l'exécution, origine
  (`App`, `Seeded { from }` pour l'historique amorcé, `Manual` pour une saisie à la main), file
  d'origine, début et fin (RFC 3339 avec décalage), durée de la tâche seule (préparation exclue),
  statut, préparation, commande telle qu'exécutée (marques remplies, `{repo}` gardé), variables
  posées par l'application, paramètres choisis, checkout (`.` ou `.claude/worktrees/nom`), dossier de
  travail relatif à ce checkout, git, threads, plafond mémoire, machine, charge, verdict de calme,
  commentaire (modifiable après coup), papiers (modifiables après coup), journal, fichiers produits,
  date de dernière modification du commentaire ou des papiers.
  Git, threads, machine et charge sont facultatifs : une fiche amorcée ou saisie n'a que ce qui est
  connu.
- `Status` : `Ok`, `Failed { code, signal }`, `Stopped` (arrêté par Marc), `Interrupted`
  (l'application s'est terminée pendant l'exécution, §8.5).
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
  commande avec marques, préparation, choix du checkout (`Home`, `Ask`, `Branch(nom)`), dossier,
  paramètres avec leurs valeurs par défaut (modifiables par élément de file), environnement propre,
  usage des threads (`Setting` ou `Fixed(n)`), plafond mémoire, « le temps compte », motifs des
  fichiers produits, papier par défaut, étiquettes.
- `Sequence` : une liste ordonnée et nommée de tâches, une table `[[sequence]]` : les étapes d'un
  NIGHT.md dans l'ordre où il les donne.

## 6. Le catalogue

`lab/jobs.toml`, modifié dans l'écran Catalogue ou à la main ; l'application le relit quand le
fichier change sur le disque. L'exemple `forced-whole-lower` de `SPEC.md` §6 montre le format.

Marques : `{threads}` (le nombre effectif de threads), `{repo}` (la racine du checkout où tourne la
tâche), `{commit}` (son hash court), et les `{paramètres}` propres à la tâche. Une marque inconnue est
une erreur montrée dans le catalogue, pas au lancement.

Le catalogue est amorcé avec les tâches que le dépôt a aujourd'hui : le tableau de `SPEC.md` §6 les
donne une par une, avec la durée **écrite** dans les documents (pas mesurée par l'application ;
« ? » quand rien n'est écrit). En résumé :

- constructions : `build-release`, `addon` ;
- suites de tests : `test-release` (plafond 40 Go, ~15 min selon Marc), `addon-test` ;
- bancs : `quick-bench` (préparation `cargo build --release && make addon`), `bench` (budget et
  outils en paramètres ; c'est aussi la comparaison aux autres remailleurs), `remeasure`, `scores` ;
- Lean : `lean-build`, `lean-oracle-seed-1`, `lean-oracle-doubles` (branche `worktree-borders`,
  PR #271, `--borders`, ~18 h d'un cœur au total), `lean-check-export` ;
- papier B : `export-male-body` (8 threads fixes, plafond 24 Go), les sept scripts rapides de
  l'étape 3 de NIGHT.md, `forced-whole-lower` (l'expérience des longueurs forcées, 1 à 4 h),
  `forced-sound`, `holonomy`, `strict`, `certify`, `conjugacy`, `parity`, `gadget` ;
- papier A : `twelve-models`, `ablation`, `male-body`, `figures`, `structure`, `against-remeshers`
  (lit seulement `bench/results/results.json`), `compare`, et les étapes de `exact/NIGHT.md`.

Séquences amorcées : `paper-B-data` (NIGHT.md, étapes 1 à 4 dans l'ordre), `paper-A-exact`
(exact/NIGHT.md, 1 à 4), `before-merge` (`build-release`, `test-release`, `quick-bench`).

## 7. Règles que l'application fait respecter

### 7.1 Threads

- Un **réglage de threads** global, dans la barre du haut. Chaque tâche déclare son usage :
  `setting` (elle reçoit le réglage) ou `Fixed(n)` (elle en utilise n quel que soit le réglage,
  comme l'export qui prend toute la machine). Le nombre **effectif** est celui que les règles
  contrôlent.
- Pour chaque tâche, l'application pose `OPENCLARE_THREADS`, `RAYON_NUM_THREADS`,
  `RUST_TEST_THREADS` et `CARGO_BUILD_JOBS` au nombre effectif, et remplit `{threads}` (pour `-P`,
  `--jobs`, `JOBS=`). L'environnement propre d'une tâche les remplace (comme `HIGHS_THREADS = "1"`).
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

- En tête de file, elle **attend que la machine soit calme** (§9) : l'élément affiche « attend le calme
  depuis 15:04 (CPU étranger 18 %) ». Elle part quand la machine a été calme sur toute la fenêtre. Les
  autres tâches de la file ne l'attendent que si elles viennent après elle.
- « Lancer quand même » la lance tout de suite. La fiche a alors `forced = true`, donc
  `timing_valid = false` : l'exécution compte, son temps non.
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
- La même préparation sur le même commit (propre) tourne une fois par file : la seconde est sautée avec
  `skipped_because = "faite pour forced-bounds à 22:03"`.
- Une préparation échouée fait échouer la tâche ; sa commande ne tourne pas.
- Ainsi `cargo build --release` précède toujours un banc : `cargo test --release` ne refait pas
  `target/release/openclare`, et les tâches de banc portent la construction dans leur préparation.

### 7.4 Plafond mémoire

`memory_limit_gb` pose `RLIMIT_AS` (souple et dur) dans l'enfant avant `exec`, comme `ulimit -v`. Les
valeurs amorcées suivent les règles du projet : 32 Go pour le moteur, Blender et Lean, 40 Go pour
`cargo test`, les valeurs de NIGHT.md quand il en donne.

## 8. Le lanceur

### 8.1 Lancement

L'application écrit `state.json` (la file, l'élément lancé), prend l'échantillon de début (§9), lit git
(`rev-parse HEAD`, `status --porcelain --untracked-files=no`, branche), puis lance
`bash -c "<commande>"` :

- dans un nouveau groupe de processus (`setpgid(0, 0)` dans `pre_exec`), avec le plafond mémoire ;
- sortie standard et d'erreur redirigées vers `<data>/logs/<id>.log`, ouvert par l'application et
  passé à l'enfant : le journal continue de s'écrire si l'application meurt ;
- entrée standard sur `/dev/null` ;
- l'environnement hérité, plus les variables du §7.1 et celles de la tâche.

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
- Ce hash est celui d'un tag (`git tag -l 'v*'`) : l'exécution est **sur ce tag** et peut étayer un papier.
  Sinon elle est **hors tag**, montrée avec son hash, et ne peut pas étayer un papier.
- Une exécution dont la durée n'a pas été mesurée affiche « no time ».

## 11. Écrans

Une fenêtre, une barre du haut et cinq onglets. Libellés en anglais comme le code ; des libellés
français sont possibles par un réglage (Q11). Disposition, d'après la maquette :

### Barre du haut

Checkout d'attache (branche, commit court, marque de modification), l'indicateur de calme, CPU et load1
d'un coup d'œil, le réglage de threads (de 1 à 8, ambre au-delà de la limite de confort).

### État (l'écran d'ouverture)

- Une ligne par tâche, groupées par papier, le chronomètre ⏱ à côté d'une tâche dont le temps compte,
  avec sa **dernière exécution** (Last run) : marquée de son tag (« v0.7.0 ») ou « off-tag <hash> », puis
  sa date, sa durée (« no time » si non mesurée ; grisée ou barrée quand le temps de la tâche compte et
  que celui-ci n'est pas valide, §7.2), la charge au début et le fichier où elle a été lue ; ou
  « never run ».
- En tête, les comptes : dernière exécution sur un tag, hors tag, jamais lancée.
- Un filtre **Tag** : « All runs » (la dernière exécution de chaque tâche, où qu'elle ait tourné), ou un
  tag : seule la dernière exécution de chaque tâche sur ce tag, les autres « no run on this tag ».
- Les tâches cochées ici, ou toutes celles jamais lancées, partent dans la file d'un clic.

### File

- **À gauche, le catalogue** : tâches groupées par catégorie, une recherche, pour chacune une case, son
  estimation, le chronomètre ⏱ si son temps compte ; les séquences au-dessus (en cocher une ajoute ses
  tâches dans l'ordre).
- **Au milieu, la file** : les éléments dans l'ordre, réordonnés par une **poignée** de glisser-déposer,
  chacun avec ses paramètres (modifiables), son checkout (d'attache ou un worktree, et si l'exécution sera
  sur un tag ou hors tag), ses threads effectifs, son estimation, son cumul et son heure de fin (section 10) ; dessous : le commentaire et la version de papier (A ou B, un libellé suggéré d'après les
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
les seuils modifiables.

## 12. Réglages

`<config>/settings.toml` : la limite de confort des threads (6, Q1), seuils
et fenêtre du calme, période d'échantillonnage, délai entre SIGTERM et SIGKILL, politiques d'arrêt et
d'échec, libellé de la machine pour les exports (proposé : « i7-7700K 4c/8t 2017, 62 GB », comparé à ce qui
est détecté : l'application avertit si le modèle de processeur ou la mémoire diffèrent).

## 13. Les résultats du dépôt dans l'historique

À chaque ouverture, lab lit les résumés que le dépôt contient déjà et montre chacun comme une exécution
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
   avertir et lancer ?
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
