# Experiments of "Closing edge loops": the exact look at the closing system

Scripts that make again the numbers of Section `sec:exact` of the paper (`../../paper.tex`, "An
exact look at the closing system"), its Tables `tab:exact` and `tab:rounding`, and the rows of
Appendix `app:provenance` that point to it. They read the export of the male body that the
turns paper reads too, with its module `tmesh.py`
(`../../../turns-of-a-t-mesh/experiments/`, whose README says how an export is made and read).

> **No number here yet.** The scripts were checked on 2026-10-28 against the export of the papers'
> first version (below, "Checked"); those runs are not kept. The scripts are run on the commit the
> paper will cite (`NIGHT.md`), and their summaries then go to `results/`.

The conventions are those of `../README.md` and of the turns paper's scripts: run from the root of
a clone; the export as first argument (default `scratch/data-package/turns-of-a-t-mesh/male-body`);
`--out` for the raw output (default `scratch/data-package/closing-loops/<group>/`), `--summary`
for the summary (default `results/<group>.json`); the engine, where a script runs it, as
`--openclare <binary>` and `--commit`. HiGHS (through SciPy) runs on one thread (`HIGHS_THREADS`),
and each summary records it with the versions, the machine and its load.

## The paragraphs and their commands

`B=docs/research/turns-of-a-t-mesh/experiments`, `A=docs/research/closing-loops/experiments/exact`.

| Paragraph or table | Command | Output |
|---|---|---|
| Opening: 1,295 nodes (1,034 T-junctions, 261 singular: 133, 127, 1), 2,069 arcs, 776 patches, 1,552 balances, 1,299 turns, 28 to 36 bounds | `python3 $B/census.py <export>` | `../../../turns-of-a-t-mesh/experiments/results/census.json` |
| Opening: on all 1,299 turns the crossing maps give the exported shift | `python3 $B/holonomy.py <export> --no-annuli` | `.../results/holonomy.json` |
| "The conflict is real but local": all held turns close with ℓ ≥ 0, not within the bounds; the least giving up of the bounds (one arc, in ways 0, 2 and 4) | `python3 $A/relaxation.py <export>` | `results/relaxation.json` |
| Same paragraph: the wrist turn, alone, in four ways, its certificate −(ℓ1921 + ℓ1937), arc 1937 0.08 long; the clusters at eyes, hands and feet | `python3 $B/conflicts.py <export>` | `.../results/conflicts.json` |
| Table `tab:exact` (held, kept by the projection, closable together); "In whole numbers, the same mixed-integer program did not finish within fifteen minutes" | `python3 $A/most_closable.py <export> --whole-most 900 --jobs 4` | `results/most_closable.json` |
| "Whole lengths are not what is missing": the closable sets closed in whole numbers (about two minutes a way for three ways, the fourth not within ten), 1 to 4 held turns left shifted against 13 to 44, the turns not held, deviation 4 % to 62 % higher | `python3 $A/most_closable.py <export> --whole 600 --jobs 4` (the same run as above, both options) | `results/most_closable.json`, `<out>/call-<name>/way-<k>-integer.txt`, `aimed-<k>.txt` |
| Same paragraph: 517 free arcs | `python3 $B/census.py <export>` (`kernel_dimension`), `python3 $B/lattice.py <export>` | |
| "Walking every strip of the mesh of way 2": 230 ways round, 168 shifted, 222 leaning less than 0.12, 218 (109 strips) closable together; no whole solution in ten or thirty minutes; 171 arcs set to zero, 23 distinct certificates, whole lengths in a second once they are given; mean deviation at least 0.817; a subset of 190 at 0.399 | `python3 $A/rings.py <export> --without-zeros 1800` | `results/rings.json`, `<out>/rings.json`, `<out>/subset.json` |
| Same paragraph: LLL, Aardal–Hurkens–Lenstra, 1,200 seconds: 0.619, 10,568 quads, none below 0.536; rounding costs 0.137 to 0.220 | `python3 $A/lattice_rounding.py <export> --seconds 1200` | `results/lattice_rounding.json`, `<out>/call-<name>.txt` |
| Table `tab:rounding`; the meshes of the whole lengths (12.2 % against 8.8 %, 3.7 % against 1.1 %, the third breaking the structure); the 190 rings given to the engine (2.5 % against 1.1 %, 156 of the 230 ways round shifted); the local search's mesh (0.020 against 0.011) | `python3 $A/remesh.py --openclare <binary> --commit <hash>` (after the scripts whose files it reads) | `results/remesh.json` |
| "The heuristic leaves most of the gap": exactly 1, 5, 1, 4, 1, 6, 1 and 9 against 13, 39, 16, 44, 49, 115, 44 and 123; with the cap and the window (10,379 to 10,801 quads), still exactly one in ways 0 and 2; of the 16 turns of way 2, one forced, 15 left by the heuristic | `python3 $B/forced.py <export> --whole-lower --jobs 4` | `.../results/forced.json` |
| Same paragraph: the local search closes 38 more strips | `python3 $A/local_search.py <export> --seconds 1200` | `results/local_search.json`, `<out>/call-<name>.txt` |
| "Closing the most rings, end to end": the slow quality mode against the engine and other draws, over the 36 runs; the limit of 5 seconds a solve; the work limit; the male body | `python3 $A/end_to_end.py --openclare <binary> --commit <hash> --male-body` (variants and their order in `NIGHT.md`) | `results/end_to_end.json` |

## What each script does

- `relaxation.py`: per way, two linear programs (all held turns closed, lengths ≥ 0, then ≥ the
  bounds) and the least Σ s with ℓ ≥ b - s, which names the bounds in the way.
- `most_closable.py`: per way, the program of the table (one binary per held turn, big M, real
  lengths, every bound kept; `--seconds`, 300 by default, where the paper's scripts had 300); the
  nearest lengths closing the set found (weighted L1, the papers' weight 1/(t + 1)), written for
  `OPENCLARE_AIMED`; with `--whole`, the set closed in whole numbers nearest the target, written
  for `OPENCLARE_LENGTHS`, with what it leaves shifted and its deviation against the engine's; with
  `--whole-most`, the program of the table in whole numbers.
- `rings.py`: one round of the constraint rounds by hand: the strips of the way's whole lengths
  (`way-<k>-helices.json`) leaning less than 0.12, the closed ones then the longest, each kept if
  the kept ones still close in real lengths within the bounds; the implicit zeros of their cone
  (one linear program) and a Farkas certificate for each (least Σ|u|), grouped; whole lengths with
  the zeros as upper bounds (and, with `--without-zeros`, without); the least plain mean deviation;
  the most rings closable within `--subset-cap` 0.40 with the number of quads held to first order
  (a program stopped after `--subset-seconds`; `--subset greedy` for the greedy choice the paper's
  text describes: on the papers' export it keeps 140, the program 181 within 90 s and the paper's
  190 within its time).
- `lattice_rounding.py`: the integer basis of the whole lengths closing the subset with its
  implicit zeros imposed first, its LLL reduction (plain inner product), Babai's nearest plane,
  and the program over the reduced basis (Aardal, Hurkens and Lenstra) with the number of quads
  to first order, stopped after `--seconds`, its proven bound turned into a mean deviation (the
  arcs forced to zero counted at their target).
- `local_search.py`: the fewest strips (or held turns) left shifted by whole lengths within
  `--radius` of the engine's, with the engine's cap and window, checked exactly.
- `remesh.py`: the engine runs of the section, each with its turns exported, measured with
  `openclare measure`; from each run's export, way by way, the turns closed in real lengths and
  those still shifted after the integer search, and for the lengths ranked first the strips of
  quads they make. Its variants are listed in its docstring.
- `end_to_end.py`: the twelve models at three budgets (and the male body) for each variant of the
  engine and of its slow quality mode, with `OPENCLARE_SLOW_LOG`: the solves of HiGHS with and
  without a solution (by status), the rounds that returned their starting lengths or spiralled more
  than the engine, the pieces whose first lengths are the selector's or a draw's. `--cpus` fixes
  the processors the engine (and its HiGHS) may use; `--jobs 1` runs one model at a time.

## What the scripts cannot make again, and why

- **The engines.** The paper's engine of the export (branch `rd-exact`, before `c8bf2be`) and the
  engine of the end-to-end experiment (branch `complete-helices` with the hooks of branch
  `maxclose-rd`) are not rebuilt. The hooks of both branches are on main since DT-089 (ALGORITHM.md,
  M6, "Experiment options", last paragraph), and the max-closable selection is the slow quality
  mode (#79, `RingSelector`, HiGHS in `crates/openclare-highs`). `export.py --engine before-cuts`
  and `remesh.py --engine before-cuts` switch off the stages the export's engine did not have, on
  today's engine: the numbers come back with a new T-mesh, not the old ones.
- **The paragraph "Closing the most rings, end to end"**: the slow mode adds its lengths to the
  candidates, with the corridor inequalities always on. What it cannot run: the selector's lengths
  in place of the engine's (4.77 % and 5.08 %), three lengths drawn at random within the cap and
  budget (4.34 %), the rounds without the corridor inequalities (3.15 %), and the counts of strips
  closed by the program and at the new lengths (27.3 of 38.2, 22.1 against 21.8, 24.6 and 24.0),
  which the hooks of `maxclose-rd` wrote and the slow log does not. The engine of the paper (4.54 %)
  is replaced by `draws-0`, and the 12 other draws of version 0.7.0 are a variant of their own.
- **The paragraph "Acting before quantization"**: the prototypes (moving singular vertices,
  rotating the field) are patches outside the engine, against commit `5bc16c6`; not scripted.
- **Times and limits** (DT-090): the 10-, 15- and 30-minute stops, the 1,200-second run and its
  bound, and the 5-second limit depend on the machine and its load; the night states both.
- **Witnesses and orders**: which held turns, rings and arcs are named depends on the export, and
  so does the subset found within a time limit.

## Long runs

| Script | Limit (one HiGHS thread) | On the papers' export, under load |
|---|---|---|
| `relaxation.py` | — | 1.4 s |
| `most_closable.py` | `--seconds 300`; `--whole 600` and `--whole-most 900` a way | the table in 14 s; `--whole` hit 100 s in way 2 under load with 1 of 52 held turns left shifted |
| `rings.py` | `--subset-seconds 600`, `--without-zeros 1800` | 91 s without them |
| `lattice_rounding.py` | `--seconds 1200` | LLL 87 s in the first version; not run on the male body here (run on blob) |
| `local_search.py` | `--seconds 1200` | no solution within 90 s; run on blob |
| `remesh.py` | — | not run (no engine built today): one to three minutes a run |
| `end_to_end.py` | the engine's: 15 (work) or 5 / 300 s (time) a solve | not run (no engine built today): hours |

## Checked on 2026-10-28

On the export of the papers' first version (see the turns paper's README), the scripts give back:
Table `tab:exact` entirely (`most_closable.py`); the bounds in the way, one arc (1937) in ways 0, 2
and 4 (`relaxation.py`); 230, 168, 222, 218 and 109, the 171 implicit zeros, the 23 distinct
certificates, whole lengths in under a second once the zeros are given, and 0.817 (`rings.py`);
in way 2, the set closed in whole numbers leaving 1 held turn shifted and 5 turns not held against
the engine's 11 (`most_closable.py --whole`, stopped at 100 s). Not given back: the subset of 190
(the program found 181 within 90 s, the greedy choice 140), and every number that needs the engine
or a long limit (`tab:rounding`, the meshes, the 0.619 and 0.536 of the lattice, the local search,
the end-to-end runs); `lattice_rounding.py` and `local_search.py` were run end to end on the export
of blob instead.
