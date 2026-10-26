# Experiments of "Closing edge loops": the applied figures

Scripts that make again every applied number and figure of the paper (`../paper.tex`, Section
`sec:experiments`, Table `tab:dev`, Figures `fig:overview` and `fig:closeups`, Appendix
`app:provenance`), on one engine given to them. Section `sec:exact` is not covered here.

> **Provisional numbers.** Every number below and in `results/` comes from the engine of commit
> **f992bac** (version 0.6.2; its engine code is that of 524b759), run once to show that the
> scripts work. f992bac is *not* the frozen commit: the engine is to be improved before one is
> frozen, and the scripts are then run again on it. The paper's text has not been changed.

Each script runs from the root of a clone, takes the engine as `--openclare <binary>` (default
`target/release/openclare`) and the commit it was built from as `--commit` (default the clone's),
writes its raw output under `--out` (default `scratch/data-package/closing-loops/<group>/`, not
committed; a run already there is read back, not made again) and its summary to `results/`
(committed). The sources are the bench's (`bench/out/<model>/source.obj`, `--sources` to read them
elsewhere); their MD5 sums are kept in the summaries. `compare.py` puts the paper's numbers next
to the summaries' (the table below).

## The groups of figures

Times: wall-clock, on an Intel Core i7-7700K (4 cores, 8 threads, 2017), 62.7 GiB of memory,
**shared with other jobs**: the load average (on 8 threads) is given with each. They give orders
of magnitude only.

| Figures | Script and command | Engine | Output | Time (load) |
|---|---|---|---|---|
| Twelve models, three budgets (`sec:experiments`, 36 runs) | `python3 docs/research/closing-loops/experiments/twelve_models.py` | f992bac | `results/twelve_models.json` | 153 s, 2 runs at a time (load 12 to 15) |
| Table `tab:dev` as an ablation (one engine, one stage per line, 10 lines × 36 runs) | `python3 docs/research/closing-loops/experiments/ablation.py --check scratch/data-package/closing-loops/twelve_models` | f992bac | `results/ablation.json` | first pass 8 lines, 288 runs: 54 min; the two split lines, 72 runs: 14 min; 2 runs at a time (load 16 rising to 71) |
| Against other remeshers (bench at 3,000 quads) | `python3 docs/research/closing-loops/experiments/against_remeshers.py --git 5bc16c6 --git b0fba91 --no-closing-run` | bench results of 524b759 (= f992bac's engine) and of the night of 2026-10-25 for the other tools; the line without closing run on f992bac | `results/against_remeshers.json` | reading: under 1 s; the line without closing, 15 Blender runs: 257 s (load 18 to 27) |
| Male body against the hand-made mesh | `python3 docs/research/closing-loops/experiments/male_body.py` | f992bac | `results/male_body.json` | 116 s with closing, 168 s without, 150 s lean 0.05 alone (load 17 to 26) |
| Figures `fig:overview`, `fig:closeups` | `WORK=scratch/data-package/closing-loops/figures OPENCLARE=target/release/openclare COMMIT=f992bac docs/research/closing-loops/figures/make_figures.sh`, then `python3 docs/research/closing-loops/experiments/figure_numbers.py scratch/data-package/closing-loops/figures` | f992bac | `../figures/male-*.png`, `../figures/README.md`, `results/figures.json` | 6 min 31 s with the two openclare meshes reused from `male_body.py` (QuadriFlow and 12 renders; load about 25) |
| The table below | `python3 docs/research/closing-loops/experiments/compare.py` | — | Markdown on the standard output | under 1 s |

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

Each line is the engine of f992bac with the switches of ALGORITHM.md (M6, "Experiment options"),
one more stage on per line; means over the twelve models at the three budgets (36 runs per line,
all valid), and at 3,000 quads alone. The paper's table gives single runs at 3,000 quads of four
different engines. The last line sets no option; its 36 meshes are those of `twelve_models.py`
byte for byte.

| Line | Switches | Paper's row | Spiral edges | at 3,000 | Spiral loops | Helices, any pitch | Closed | Hausdorff ‰ | Mean distance ‰ | Corners, deg |
|---|---|---|---|---|---|---|---|---|---|---|
| no closing | `--no-close-loops` | no closing (16.7%) | 19.81% | 18.18% | 278 | 12.93% | 26.3% | 8.35 | 0.373 | 4.90 |
| first closing | ways `0,1`; every `NO_` switch | closing, one way kept by spirals (9.7%) | 10.14% | 9.47% | 172 | 6.22% | 33.1% | 8.43 | 0.374 | 5.01 |
| + leaning ways | ways `0,1,2,3`; every `NO_` switch | (caption: 10.1% → 9.5% at 452b346) | 9.08% | 7.83% | 156 | 5.68% | 33.7% | 8.33 | 0.373 | 5.07 |
| + aimed repair | ways `0,1,2,3`; `NO_` all but `AIMED_REPAIR` | + leaning ways, aimed repair (9.1%) | 7.86% | 6.59% | 154 | 4.82% | 34.3% | 8.12 | 0.372 | 5.10 |
| + longer turns | all ways; `NO_` all but `AIMED_REPAIR` | — | 6.63% | 5.83% | 139 | 4.38% | 35.0% | 8.32 | 0.375 | 5.15 |
| + deviation cap, without its floor | `NO_DEVIATION_FLOOR`, `NO_CUT_ROUNDS`, `NO_ACTIVE_SET`, `NO_FOLD_GUARD` | + longer turns, deviation cap (6.0%) | 8.43% | 6.25% | 151 | 5.38% | 33.8% | 8.29 | 0.375 | 5.10 |
| + constraint rounds | `NO_DEVIATION_FLOOR`, `NO_ACTIVE_SET`, `NO_FOLD_GUARD` | — (not in the table) | 7.26% | 5.10% | 138 | 4.83% | 34.4% | 8.30 | 0.374 | 5.14 |
| + active set | `NO_DEVIATION_FLOOR`, `NO_FOLD_GUARD` | — (not in the table) | 7.58% | 5.46% | 146 | 5.14% | 34.3% | 8.27 | 0.381 | 5.04 |
| + floor of the cap | `NO_FOLD_GUARD` | — (not in the table) | 5.85% | 5.46% | 133 | 4.02% | 35.4% | 8.38 | 0.382 | 5.10 |
| + fold guard: the engine | none | (4.65% over three budgets) | 5.85% | 5.46% | 133 | 4.02% | 35.4% | 8.38 | 0.382 | 5.10 |

Every row of the paper's table has its switches, but two rows add two stages at once; one switch
per line splits them (leaning ways, then aimed repair; longer turns, then the deviation cap). The
stages the table leaves out (constraint rounds, active set, floor of the cap, fold guard) have
switches too. On f992bac the fold guard changes none of the 36 meshes.

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

What the lines say on f992bac: each of the first stages takes spiral edges off (19.8% → 10.1% →
9.1% → 7.9% → 6.6%); the deviation cap without its floor gives back more than the longer turns
took (6.6% → 8.4%); the constraint rounds help (→ 7.3%), the active set does not (→ 7.6%), and the
floor of the cap brings the engine to 5.85%. Over the lines the Hausdorff distance stays within
about 4% (8.12 to 8.43‰) and the corner deviation within about 5% (4.90 to 5.15 degrees). The
independent measure (helices of any pitch) falls with the spiral share on every line but one
(+ active set, both up).

## Paper says / engine of f992bac gives

Made by `compare.py` from the summaries of `results/` (provisional, f992bac). "line" is the line of
`../paper.tex` where the paper's number is; in the Table `tab:dev` rows the engine's value is the
mean over 36 runs, at 3,000 quads in brackets (the paper: single runs at 3,000 of older engines).
The abstract (lines 70 to 74) repeats 4.65%, 16.7%, 1.8%, 38.8%, 21.7%, 1.6% and 2.5%, and
Appendix `app:provenance` (line 956) 4.65%.

| | line | paper | engine |
|---|---|---|---|
| **Twelve models, three budgets** | | | |
| spiral edges, mean of 36 runs | 533 | 4.65% | 5.85% |
| … at 2,970 quads | 533 | 2.89% | 4.86% |
| … at 3,000 quads | 534 | 4.97% | 5.46% |
| … at 3,030 quads | 534 | 6.09% | 7.23% |
| spiral loops in the 36 meshes | 534 | 102 | 133 |
| valid meshes | 535 | 36 of 36 | 36 of 36 |
| closed-loop edges | 535 | 36.7% | 35.4% |
| mean distance, ‰ of the diagonal | 536 | 0.377 | 0.382 |
| mean corner deviation, degrees | 537 | 4.90 | 5.10 |
| edges on helices of any pitch (DT-137) |  | — | 4.02% |
| helices of pitch above 4, per run (DT-137) |  | — | 0.06 |
| **Table tab:dev (paper: single runs at 3,000, four engines; here one engine, means over 36 runs, at 3,000 in brackets)** | | | |
| no_closing: spiral edges | 551 | 16.7% | 19.81% (18.18%) |
| no_closing: Hausdorff, ‰ | 551 | 8.77 | 8.35 (8.62) |
| no_closing: corner deviation, degrees | 551 | 4.75 | 4.90 (4.88) |
| first_closing: spiral edges | 552 | 9.7% | 10.14% (9.47%) |
| first_closing: Hausdorff, ‰ | 552 | 8.57 | 8.43 (8.46) |
| first_closing: corner deviation, degrees | 552 | 4.85 | 5.01 (4.99) |
| leaning_aimed: spiral edges | 553 | 9.1% | 7.86% (6.59%) |
| leaning_aimed: Hausdorff, ‰ | 553 | 8.58 | 8.12 (7.94) |
| leaning_aimed: corner deviation, degrees | 553 | 4.91 | 5.10 (5.09) |
| longer_cap: spiral edges | 554 | 6.0% | 8.43% (6.25%) |
| longer_cap: Hausdorff, ‰ | 554 | 8.68 | 8.29 (8.07) |
| longer_cap: corner deviation, degrees | 554 | 5.01 | 5.10 (5.02) |
| leaning alone (ways 0,1,2,3, no aimed repair): spiral edges | 561 | 9.5% (10.1% before, commit 452b346) | 9.08% (7.83%) |
| longer: spiral edges (not in the table) |  | — | 6.63% (5.83%) |
| cut_rounds: spiral edges (not in the table) |  | — | 7.26% (5.10%) |
| active_set: spiral edges (not in the table) |  | — | 7.58% (5.46%) |
| deviation_floor: spiral edges (not in the table) |  | — | 5.85% (5.46%) |
| engine: spiral edges (not in the table) |  | — | 5.85% (5.46%) |
| **Against other remeshers (bench at 3,000 quads)** | | | |
| models QuadriFlow produces a mesh for | 573 | 22 | 22 |
| models AutoRemesher produces a mesh for | 574 | 18 | 18 |
| models all three mesh validly in every run | 574 | 7 | 7 |
| … openclare spiral edges | 575 | 1.8% | 2.2% |
| … QuadriFlow | 575 | 38.8% | 38.8% |
| … AutoRemesher | 576 | 21.7% | 21.7% |
| … openclare without closing | 578 | 16.6% | 15.3% |
| models all three produce a mesh for | 576 | 15 | 15 |
| … openclare spiral edges | 577 | 4.2% | 3.3% |
| … QuadriFlow | 577 | 27.2% | 27.2% |
| … AutoRemesher | 577 | 27.3% | 27.3% |
| … openclare without closing | 578 | 15.5% | 15.2% |
| … helices of any pitch, openclare / QuadriFlow / AutoRemesher (DT-137) |  | — | 1.9% / 30.5% / 21.8% |
| **Male body, 10,590 quads** | | | |
| hand-made: spiral edges | 587 | 2.5% | 2.5% |
| hand-made: spiral loops | 587 | 3 | 3 |
| hand-made: closed-loop edges | 588 | 31.6% | 31.6% |
| openclare, closing: spiral edges | 590 | 1.6% | 1.65% |
| openclare, closing: spiral loops | 591 | 9 | 16 |
| openclare, closing: closed-loop edges | 593 | 33.2% | 36.6% |
| openclare, closing: mean distance, ‰ | 594 | 0.19 | 0.20 |
| openclare, closing: corner deviation | 594 | 4.1 | 4.3 |
| hand-made: mean distance, ‰ | 595 | 0.17 | 0.17 |
| hand-made: corner deviation | 595 | 9.2 | 9.2 |
| lean bound 0.05 alone: spiral edges | 601 | 12.1% | 4.1% |
| openclare, closing: helices of any pitch (DT-137) |  | — | 0.26% |
| **Figure fig:overview (caption; the meshes of figures/make_figures.sh)** | | | |
| (a) no closing: quads | 631 | 10,485 | 10,625 |
| (a) no closing: spiral edges (loops) | 632 | 26.8% (18) | 23.39% (21) |
| (a) no closing: closed-loop edges | 634 | 10.2% | 17.2% |
| (b) closing: quads | 631 | 10,748 | 10,520 |
| (b) closing: spiral edges (loops) | 632 | 1.95% (15) | 1.65% (16) |
| (b) closing: closed-loop edges | 634 | 32.6% | 36.6% |
| (c) hand-made: quads | 631 | 10,570 | 10,570 |
| (c) hand-made: spiral edges (loops) | 632 | 2.55% (3) | 2.55% (3) |
| (c) hand-made: closed-loop edges | 588 | 31.6% | 31.6% |
| (d) QuadriFlow: quads | 631 | 8,844 | 8,844 |
| (d) QuadriFlow: spiral edges (loops) | 633 | 35.7% (43) | 35.72% (43) |
| (d) QuadriFlow: closed-loop edges | 634 | 2.7% | 2.7% |
| (d) QuadriFlow: inverted faces | 635 | 3 | 3 |
| (d) QuadriFlow: Hausdorff, ‰ | 636 | 31 | 31.3 |

## Conclusions of the paper the f992bac numbers no longer support

Against the f992bac numbers (provisional; to be checked again on the frozen commit):

1. **The headline 4.65%** (abstract, lines 70, 533, 563, 956): the engine gives 5.85% (4.86%, 5.46%,
   7.23% at the three budgets), 133 spiral loops instead of 102, a corner deviation of 5.10 degrees
   instead of 4.90. Closed loops (35.4% against 36.7%) and the distance (0.382 against 0.377‰)
   barely move; all 36 meshes are still valid.
2. **Table `tab:dev`**: "from 16.7% of spiral edges without closing to 6.0%" becomes 19.8% to 5.85%
   (36 runs; 18.2% to 5.46% at 3,000). The sentence "the longer turns and the deviation cap trade
   a little of both for a third fewer spiral edges" (line 542) is not supported: on one engine the
   longer turns take a sixth off (7.86% → 6.63%) and the cap without its floor adds more than that
   back (→ 8.43%, above the line before it); only with the constraint rounds and the floor of the
   cap does the engine reach 5.85%. "Hausdorff within about 2%" (lines 540 to 541) becomes about 4%. The caption's warning that each line "adds to the one above only roughly" is
   no longer needed: on one commit each line adds exactly one stage.
3. **Against other remeshers**: openclare's 1.8% and 4.2% become 2.2% and 3.3%; QuadriFlow and
   AutoRemesher are unchanged (the bench reran them, same meshes); without closing 15.3% and 15.2%
   (16.6% and 15.5%). The conclusion (openclare far below both) holds. The caveat that the bench
   "used an engine a few changes away from the cited one, which give 4.54% instead of 4.65%" (line
   571) is obsolete: the bench's openclare runs are of this engine. "A measure of structure the
   selection does not use is left to future work" (lines 582 to 583) is obsolete too: on the 15 models,
   edges on helices of any pitch are 1.9% for openclare, 30.5% for QuadriFlow and 21.8% for
   AutoRemesher (0.5%, 42.9% and 20.4% on the 7), and openclare has no helix the spiral count
   misses; the ranking holds on the independent measure.
4. **Male body**: 1.65% of spiral edges, but in 16 spiral loops, not 9. "Below the hand-made mesh,
   but by a few loops only" (line 591) no longer holds as written: openclare has fewer spiral edges
   (1.65% against 2.55%) but more spiral loops (16 against 3). Closed loops 36.6% (33.2%), distance
   0.20‰ (0.19), corners 4.3 degrees (4.1). The provenance "commits 1b77f7d to 4db7b42" and "one
   such run" go: one run of one engine.
5. **The lean bound** (lines 598 to 601): with the lean bound 0.05 alone the body has 4.1% of spiral
   edges, not 12.1%. The lean bound still matters (4.1% against 1.65%), but "one leg was a single
   helix from hip to ankle" is not reproduced as a number (the mesh of this run was not inspected
   for it).
6. **Figure `fig:overview`** (caption, lines 627 to 636, and text, lines 603 to 609): (a) 10,625 quads, 23.4% in 21 loops,
   17.2% closed; (b) 10,520 quads, 1.65% in 16 loops, 36.6% closed; (c) and (d) unchanged. Without
   closing, helices wind round both legs, a band across the upper chest, the head and the outer
   side of the left arm, not the neck. With closing, the spiral loops left are not "short ones at
   the mouth, the forehead and the fingers": they are on the top of the head, the left ear, the
   fingers and toes, and one long loop runs down the outer side of the left arm.
7. **Figure `fig:closeups`**: "at the left wrist it leaves spiral loops on the thumb and the back of
   the hand where the mesh without closing has closed ones" (lines 610 to 614 and the caption, lines 643 to 652) no
   longer holds: both meshes have spiral loops on the thumb, the outer forearm and the fingers, and
   closing removes those on the back of the hand. "Near the right eye both openclare meshes keep
   short spiral loops, while closing turns a loop across the forehead into a ring" no longer holds:
   neither has a spiral loop near the eye, and the spiral loop across the temple and forehead
   becomes a loop that stops at irregular vertices, not a ring.
8. **Limitations** (lines 886 to 888): "the comparison with other remeshers ... used earlier
   engines, and the male body was not remeshed with the cited commit" goes once the frozen commit
   is rerun with these scripts.
