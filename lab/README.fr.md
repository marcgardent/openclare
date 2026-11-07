# lab

Le registre et le lanceur des calculs d'openclare : chaque banc, suite de tests, construction Lean et
expérience des papiers se lance d'ici, et chaque exécution est notée avec son hash, la machine, la charge,
le verdict de calme et le papier qu'elle étaye. La spécification est `SPEC.fr.md` (anglais : `SPEC.md`).

```sh
cd lab && cargo run --release
```

lab trouve le dépôt où il se trouve. Il tourne sous Linux (il lit /proc) ; `nvidia-smi`, s'il est là,
donne le GPU.

## Les écrans

- **State** s'ouvre en premier : la dernière exécution de chaque tâche, marquée de son tag (`v0.7.0`) ou
  `off-tag <hash>`, sa date, sa durée (`no time` quand elle n'a pas été mesurée) et la charge à son début.
  Une exécution est sur un tag quand le seul hash qu'elle note est celui du tag ; une exécution hors tag
  ne peut pas étayer un papier. Le filtre Tag montre la dernière exécution de chaque tâche sur un seul
  tag. Les tâches cochées, ou toutes celles jamais lancées, partent dans la file d'un clic. Un
  chronomètre ⏱ à côté d'une tâche dit que son temps compte : elle attend une machine calme, et sa durée
  n'est en clair que si le temps de l'exécution est valide, grisée quand son calme n'a pas été mesuré,
  barrée quand la machine n'était pas calme. Une tâche sans chronomètre tourne sous n'importe quelle
  charge ; sa durée est un ordre de grandeur.
- **Queue** : tâches et séquences prises dans le catalogue, réordonnées par leur poignée, leurs
  paramètres et leur checkout modifiables ; chacune montre son estimation (la médiane des exécutions
  passées de la tâche, avec leur nombre et leur charge), le cumul et l'heure de fin prévue. Run les lance
  une à une, chacune dans son groupe de processus ; Stop envoie SIGTERM, puis SIGKILL. Une tâche dont le
  temps compte (⏱) attend une machine calme ; « Start anyway » la lance, son temps ne compte alors pas.
- **History** : toutes les exécutions notées par lab et tous les résultats que les résumés du dépôt
  contiennent, triables et filtrées ; le détail d'une exécution ; son commentaire et son papier
  modifiables ; les exécutions affichées copiées en Markdown (avec la ligne de matériel) ou en CSV ; une
  exécution passée saisie à la main ; l'historique copié dans le dépôt.
- **Catalogue** : les tâches de `jobs.toml` et les commandes telles qu'elles partiraient. Modifiez
  `jobs.toml` à la main : lab le relit dans les deux secondes.
- **Machine** : CPU total et par thread, charge, mémoire, VRAM, les processus les plus lourds (à nous ou
  étrangers), l'indicateur de calme et ses seuils.

L'indicateur de calme, dans la barre du haut : vert quand, sur les 60 dernières secondes, le CPU étranger
reste sous 5 % en moyenne et sous 25 % à chaque mesure, et load1 sous 1,0 (load1 n'est pas regardé pendant
5 minutes après une exécution à nous) ; ambre quand seule notre exécution occupe la machine ; rouge quand
autre chose la charge.

## Les fichiers

| Fichier | Suivi | Contenu |
|---|---|---|
| `lab/jobs.toml` | oui | le catalogue : commandes, préparations, threads, plafonds mémoire, le résumé que chaque tâche écrit |
| `lab/runs.jsonl` | oui | un instantané de l'historique, écrit par « Copy history to the repo » |
| `~/.local/share/openclare-lab/runs.jsonl` | non | l'historique vivant, une exécution par ligne |
| `~/.local/share/openclare-lab/logs/` | non | le journal de chaque exécution |
| `~/.local/share/openclare-lab/state.json` | non | la file en cours, pour reprendre une exécution laissée par une application morte |
| `~/.config/openclare-lab/settings.toml` | non | réglage de threads, limite de confort, seuils du calme, politiques |

Aucune fiche ne nomme un chemin local : le checkout s'écrit `{repo}`, le dossier personnel `~` ; une fiche
qui en nomme encore un va dans `refused.jsonl`.

## Options

`--data <dossier>` lit et écrit les exécutions ailleurs (`tests/fixtures` montre l'historique sur des
exécutions inventées) ; `--tab <state|queue|history|catalogue|machine>` ; `--queue <tâche ou séquence>`
(répétable) ; `--start` appuie sur Run ; `--run <id>` choisit une exécution ;
`--screenshot <fichier.ppm> [--after <secondes>]` écrit une image de la fenêtre elle-même, jamais de
l'écran, puis la ferme.

## Tests

```sh
cd lab && cargo test --release
```

Ils lancent de vrais processus (`sleep`, `false`, un shell qui ignore SIGTERM), lisent les tags et les
résumés de ce dépôt, et lisent /proc. La CI construit et teste lab quand `lab/` change
(`.github/workflows/lab.yml`).
