# Experiments of "Closing edge loops": the applied figures

Scripts that make again every applied number and figure of the paper (`../paper.tex`, Section
`sec:experiments`, Table `tab:dev`, Figures `fig:overview` and `fig:closeups`, Appendix
`app:provenance`), on one engine given to them. Section `sec:exact` has its own scripts, in
`exact/` (its README and NIGHT.md).

> **No number here yet.** The scripts were run once on the engine of f992bac to check that they
> work; those numbers are not kept, the engine having changed since (version 0.7.0). The scripts
> are run on the commit the paper will cite, and their summaries then go to `results/`.

Each script runs from the root of a clone, takes the engine as `--openclare <binary>` (default
`target/release/openclare`) and the commit it was built from as `--commit` (default the clone's),
writes its raw output under `--out` (default `scratch/data-package/closing-loops/<group>/`, not
committed; a run already there is read back, not made again) and its summary to `results/`
(committed). The sources are the bench's (`bench/out/<model>/source.obj`, `--sources` to read them
elsewhere); their MD5 sums are kept in the summaries. `compare.py` puts the paper's numbers next
to the summaries' (the table "paper says / engine gives").

## The groups of figures

| Figures | Script and command | Output |
|---|---|---|
| Twelve models, three budgets (`sec:experiments`, 36 runs) | `python3 docs/research/closing-loops/experiments/twelve_models.py` | `results/twelve_models.json` |
| Table `tab:dev` as an ablation (one engine, one stage per line, 11 lines × 36 runs) | `python3 docs/research/closing-loops/experiments/ablation.py --check scratch/data-package/closing-loops/twelve_models` | `results/ablation.json` |
| Against other remeshers (bench at 3,000 quads) | `python3 docs/research/closing-loops/experiments/against_remeshers.py --git 5bc16c6 --git b0fba91 --no-closing-run` | `results/against_remeshers.json` |
| Male body against the hand-made mesh | `python3 docs/research/closing-loops/experiments/male_body.py` | `results/male_body.json` |
| Figures `fig:overview`, `fig:closeups` | `WORK=scratch/data-package/closing-loops/figures OPENCLARE=target/release/openclare COMMIT=<commit> docs/research/closing-loops/figures/make_figures.sh`, then `python3 docs/research/closing-loops/experiments/figure_numbers.py scratch/data-package/closing-loops/figures` | `../figures/male-*.png`, `../figures/README.md`, `results/figures.json` |
| The paper's numbers next to the summaries' | `python3 docs/research/closing-loops/experiments/compare.py` | Markdown on the standard output |

What each script does and why:

- `twelve_models.py`: the protocol of Appendix `app:provenance` (12 models, presets organic or
  hard-surface, budgets 2,970, 3,000 and 3,030, `openclare remesh` then `openclare measure
  --json`), and the means of `loop_edges_spiral`, `loop_edges_closed`, `loops_spiral`,
  `mean_distance`, `angle_deviation_mean`, and of the measures of structure the engine's
  selection does not look at (DT-137): `loop_edges_helix`, `loops_helix_missed`,
  `base_complex_patches`, `irregular_vertices`.
- `ablation.py`: the stages of the closing turned off with the experiment options of
  `crates/openclare/src/quantization/experiment.rs` (ALGORITHM.md, M6, "Experiment options"), one
  more stage on per line, on the twelve models at the three budgets. The last line sets no option
  and is checked byte for byte against the plain runs of `twelve_models.py`.
- `against_remeshers.py`: reads `bench/results/results.json` and nothing else, with the paper's
  definitions: a tool *produces a mesh* for a model if a run gives a mesh whose loops can be
  measured; it *meshes validly in every run* if all its runs give valid meshes; per model, the
  bench's median over the runs (five seeds for QuadriFlow, one run otherwise); then the mean over
  the models. `--git 5bc16c6 --git b0fba91` reads the files the paper used: they give back
  exactly the paper's 22, 18, 7, 15, 1.8%, 38.8%, 21.7%, 4.2%, 27.2%, 27.3%, and 16.6% and 15.5%
  without closing, which checks the definitions. The current bench has no run without closing:
  `--no-closing-run` makes it, as the bench runs openclare (the add-on, in Blender, on the model's
  scene; `make addon` first) with `OPENCLARE_CLOSE_WAYS=` (no way of closing tried). That variable
  gives the same meshes as `--no-close-loops` (checked byte for byte on blob, suzanne and spot), and
  the add-on chain of f992bac gives back the measures of the bench's openclare mesh of 524b759
  (checked on blob: same quads, spiral and closed shares, Hausdorff).
- `male_body.py`: the male body at 10,590 quads, preset organic, with closing, without it, and
  with the lean bound 0.05 alone (`OPENCLARE_CLOSE_WAYS=0,1,4,5`, the paper's "commit 452b346"
  experiment), and the hand-made mesh `bench/reference/handmade/human_male.obj` measured the same
  way. It also copies the bench's own run of the add-on at the hand-made count
  (`openclare-as-handmade`), which on this engine is the same mesh as the command line's.
- `figures/make_figures.sh`: unchanged but for taking the binary (`OPENCLARE`, `COMMIT`); its
  meshes, measures and panels go to `WORK`; `figure_numbers.py` copies the measures to
  `results/figures.json`.

## Table `tab:dev`: what one engine can and cannot ablate

Each line is the engine of one commit with the switches of ALGORITHM.md (M6, "Experiment
options"), one more stage on per line, on the twelve models at the three budgets (36 runs per
line). The paper's table gives single runs at 3,000 quads of four different engines. Every line
but the last sets `OPENCLARE_EXTRA_DRAWS=0`: the engine's own lengths only, as before version
0.7.0; the last line sets no option, and its 36 meshes must be those of `twelve_models.py` byte for
byte (`--check`).

| Line | Switches | Paper's row |
|---|---|---|
| no closing | `--no-close-loops` | no closing |
| first closing | ways `0,1`; every `NO_` switch | closing, one way kept by spirals |
| + leaning ways | ways `0,1,2,3`; every `NO_` switch | (caption: 452b346) |
| + aimed repair | ways `0,1,2,3`; `NO_` all but `AIMED_REPAIR` | + leaning ways, aimed repair |
| + longer turns | all ways; `NO_` all but `AIMED_REPAIR` | — |
| + deviation cap, without its floor | `NO_DEVIATION_FLOOR`, `NO_CUT_ROUNDS`, `NO_ACTIVE_SET`, `NO_FOLD_GUARD` | + longer turns, deviation cap |
| + constraint rounds | `NO_DEVIATION_FLOOR`, `NO_ACTIVE_SET`, `NO_FOLD_GUARD` | — (not in the table) |
| + active set | `NO_DEVIATION_FLOOR`, `NO_FOLD_GUARD` | — (not in the table) |
| + floor of the cap | `NO_FOLD_GUARD` | — (not in the table) |
| + fold guard | none but `OPENCLARE_EXTRA_DRAWS=0` | (the engine of the paper) |
| + other draws: the engine | none | — (version 0.7.0) |

Every row of the paper's table has its switches, but two rows add two stages at once; one switch
per line splits them (leaning ways, then aimed repair; longer turns, then the deviation cap). The
stages the table leaves out (constraint rounds, active set, floor of the cap, fold guard) have
switches too.

What a single commit cannot reproduce, because no switch turns it off: the engines of 9f3e455,
3658abc, 197f0ca and 452b346 themselves. Every line here runs with the rest of today's engine,
which those commits did not have and which is always on: the field, T-mesh and work mesh of
today (among them DT-067, DT-155), the pull of the folds in the cost (1b77f7d), the plain lengths
and the next candidates meshed where the chosen lengths fold a face over (d37bfad, 7ae552f), no
closing on lengths that break the structure (2f55a99), held turns told apart by their shift
expression (4db7b42), the parity and non-negative-combination pruning (8908d6a), the shifts read
at the parametrization's lengths (524b759), and the three candidates kept. So the paper's
single-run numbers per version (16.7%, 9.7%, 9.1%, 6.0%) and the moved baseline of the caption
(10.1% to 9.5% at 452b346) cannot be made again on one commit; the lines above replace them.
