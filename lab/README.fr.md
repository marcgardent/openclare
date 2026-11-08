# lab

Le registre et le lanceur des calculs d'openclare : chaque banc, suite de tests, construction Lean et
expérience des papiers se lance d'ici, et chaque exécution est notée avec son hash, la machine, la charge,
le verdict de calme et le papier qu'elle étaye. La spécification est `SPEC.fr.md` (anglais : `SPEC.md`).

```sh
make lab-install
```

depuis la racine du dépôt met le binaire dans `~/.local/bin/lab` et une entrée « openclare lab » dans le
menu du bureau (KDE, et tout ce qui lit `~/.local/share/applications`) ; `make lab-uninstall` le défait.
Pendant le développement de lab, `cd lab && cargo run --release`.

lab n'a besoin d'aucun clone. Il garde un **miroir** du dépôt dans `~/.openclare-lab/repo.git`, créé au
premier démarrage (depuis le clone où il est lancé quand il y en a un, ses objets LFS liés ; sinon depuis
GitHub par SSH) et récupéré avant chaque file et par le bouton Fetch. Chaque tâche tourne sur une
**référence**, un tag (`v0.7.0`), une branche (`main`), une pull request (`pull/301`) ou un hash, résolue
dans le miroir et extraite une fois dans `~/.openclare-lab/checkouts/<hash>/` ; un dossier de la machine
(`~/…`) est la porte de secours pour une expérience non commise, ses exécutions notées « local ». Le
catalogue montré est le `lab/jobs.toml` de `main`. Chaque machine garde sa propre mémoire et la pousse
vers sa branche `lab/runs-<machine>` du dépôt (le bouton Push de la barre du haut, ou `push = "auto"`
dans les réglages) ; chaque machine lit les branches des autres à chaque récupération, et les écrans
montrent toutes les exécutions, où qu'elles aient tourné. Il tourne sous Linux (il lit /proc) ;
`nvidia-smi`, s'il est là, donne le GPU.

## Les écrans

- **State** s'ouvre en premier : la dernière exécution de chaque tâche, marquée de son tag (`v0.7.0`) ou
  `off-tag <hash>`, sa date, sa durée (`no time` quand elle n'a pas été mesurée) et la charge à son début.
  Une exécution est sur un tag quand le seul hash qu'elle note est celui du tag ; sinon elle est hors tag,
  ou `orphan <hash>` quand aucune branche ni tag du dépôt distant n'atteint ce commit ; ni l'une ni
  l'autre ne peut étayer un papier. Le filtre Tag montre la dernière exécution de chaque tâche sur un seul
  tag ; le filtre Machine, les exécutions notées par une machine. Les tâches cochées, ou toutes celles
  jamais lancées, partent dans la file d'un clic. Un
  chronomètre ⏱ à côté d'une tâche dit que son temps compte : elle attend une machine calme, et sa durée
  n'est en clair que si le temps de l'exécution est valide, grisée quand son calme n'a pas été mesuré,
  barrée quand la machine n'était pas calme. Une tâche sans chronomètre tourne sous n'importe quelle
  charge ; sa durée est un ordre de grandeur.
- **Queue** : tâches et séquences prises dans le catalogue, réordonnées par leur poignée, leurs
  paramètres et leur référence modifiables (… liste les tags et les branches), chacune disant si elle
  sera sur un tag, hors tag ou orpheline et si son commit est déjà extrait ; chacune montre son estimation
  (la médiane des exécutions passées de la tâche, avec leur nombre et leur charge), le cumul et l'heure de
  fin prévue. Run récupère le miroir et les lance une à une, chacune dans son groupe de processus, la
  préparation une fois par checkout ; Stop envoie SIGTERM, puis SIGKILL. Une tâche dont le temps compte
  (⏱) attend une machine calme ; « Start anyway » la lance, son temps ne compte alors pas.
- **History** : toutes les exécutions notées par lab et tous les résultats que les résumés du dépôt
  contiennent, triables et filtrées ; le détail d'une exécution ; son commentaire et son papier
  modifiables ; les exécutions affichées copiées en Markdown (avec la ligne de matériel) ou en CSV ; une
  exécution passée saisie à la main ; l'historique de toutes les machines copié dans le dépôt.
- **Catalogue** : les tâches de `jobs.toml` et les commandes telles qu'elles partiraient. Modifiez
  `jobs.toml` à la main : celui du miroir est relu à la récupération suivante, un fichier donné par
  `--catalogue` dans les deux secondes.
- **Machine** : CPU total et par thread, charge, mémoire, VRAM, les processus les plus lourds (à nous ou
  étrangers), l'indicateur de calme et ses seuils ; les checkouts du miroir, leur taille à la demande, et
  Clean, qui retire ceux hors de tout tag avec ce qui y a été construit (rien n'est retiré tout seul).

L'indicateur de calme, dans la barre du haut : vert quand, sur les 60 dernières secondes, le CPU étranger
reste sous 5 % en moyenne et sous 25 % à chaque mesure, et load1 sous 1,0 (load1 n'est pas regardé pendant
5 minutes après une exécution à nous) ; ambre quand seule notre exécution occupe la machine ; rouge quand
autre chose la charge.

## Les fichiers

| Fichier | Suivi | Contenu |
|---|---|---|
| `lab/jobs.toml` | oui | le catalogue : commandes, préparations, threads, plafonds mémoire, le résumé que chaque tâche écrit |
| `lab/runs.jsonl` | oui | un instantané de l'historique, écrit par « Copy history to the repo » |
| `~/.openclare-lab/runs.jsonl` | non | l'historique vivant de cette machine, une exécution par ligne |
| `~/.openclare-lab/logs/` | non | le journal de chaque exécution |
| `~/.openclare-lab/state.json` | non | la file en cours, pour reprendre une exécution laissée par une application morte |
| `~/.openclare-lab/settings.toml` | non | le dépôt distant, le nom de la machine, la politique de push, réglage de threads, limite de confort, seuils du calme, politiques |
| `~/.openclare-lab/repo.git/` | non | le miroir du dépôt |
| `~/.openclare-lab/machines/<machine>.jsonl` | non | la mémoire de chaque machine, telle que sa branche `lab/runs-<machine>` l'avait à la dernière récupération |
| `~/.openclare-lab/checkouts/<hash>/` | non | un checkout détaché par commit lancé, avec son `target/`, son `.lake/` et `.lab-prepared`, les préparations faites dedans |

`~/.openclare-lab` est le seul dossier de tout ce que lab garde sur la machine. Le premier démarrage y
déplace une fois les fichiers des anciens dossiers, `~/.local/share/openclare-lab` et
`~/.config/openclare-lab`.

Aucune fiche ne nomme un chemin local : le checkout s'écrit `{repo}`, le dossier personnel `~` ; une fiche
qui en nomme encore un va dans `refused.jsonl`.

## Options

`--data <dossier>` utilise un autre dossier que `~/.openclare-lab` (`tests/fixtures` montre l'historique
sur des exécutions inventées) ; `--catalogue <fichier>` lit le catalogue dans ce fichier à la place de
celui du miroir ; `--tab <state|queue|history|catalogue|machine>` ; `--queue <tâche ou séquence>`
(répétable) ; `--start` appuie sur Run ; `--run <id>` choisit une exécution ;
`--screenshot <fichier.ppm> [--after <secondes>]` écrit une image de la fenêtre elle-même, jamais de
l'écran, puis la ferme.

## Sans fenêtre

```sh
lab run build-release --ref v0.7.0
```

lance une tâche avec le même lanceur, le même moniteur et les mêmes fiches, son journal sur la sortie
standard, et sort avec le statut de la tâche ; `--ref` est par défaut la référence de la tâche, `--force`
la lance bien que la machine ne soit pas calme (son temps ne compte alors pas), `--threads <n>` fixe le
nombre de threads, `--push` pousse la mémoire de cette machine vers sa branche après l'exécution, quel
que soit le réglage. Ctrl-C arrête l'exécution et la note arrêtée. `lab fetch` crée ou récupère le miroir
et lit les mémoires des autres machines ; `lab push` récupère puis pousse la mémoire de cette machine ;
`lab status` dit où en sont le miroir, les checkouts, les machines, la file et l'historique.

## Tests

```sh
cd lab && cargo test --release
```

Ils lancent de vrais processus (`sleep`, `false`, un shell qui ignore SIGTERM), lisent les tags et les
résumés de ce dépôt, et lisent /proc. La CI construit et teste lab quand `lab/` change
(`.github/workflows/lab.yml`).
