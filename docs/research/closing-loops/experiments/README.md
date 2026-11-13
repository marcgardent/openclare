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
| Table `tab:dev` as an ablation (one engine, one stage per line, 11 lines × 252 runs) | `python3 docs/research/closing-loops/experiments/ablation.py` | `results/ablation.json` |
| Twelve models, 21 budgets (`sec:experiments`, 252 runs): the last line of the ablation, read back | `python3 docs/research/closing-loops/experiments/twelve_models.py --out scratch/data-package/closing-loops/ablation/engine` | `results/twelve_models.json` |
| Against other remeshers (bench at 3,000 quads), Table `tab:remeshers` | `python3 docs/research/closing-loops/experiments/against_remeshers.py --git 5bc16c6 --git b0fba91 --no-closing-from docs/research/closing-loops/experiments/results/against_remeshers.json` (first made with `--no-closing-run`) | `results/against_remeshers.json` |
| Male body against the hand-made mesh | `python3 docs/research/closing-loops/experiments/male_body.py` | `results/male_body.json` |
| Figures `fig:overview`, `fig:closeups` | `WORK=scratch/data-package/closing-loops/figures OPENCLARE=target/release/openclare COMMIT=<commit> docs/research/closing-loops/figures/make_figures.sh`, then `python3 docs/research/closing-loops/experiments/figure_numbers.py scratch/data-package/closing-loops/figures` | `../figures/male-*.png`, `../figures/README.md`, `results/figures.json` |
| Measures the selection does not use (DT-137), on the bench's stored meshes at 3,000 quads | `python3 -I docs/research/closing-loops/experiments/structure.py [--meshes bench/out]` | `results/structure.json` |
| The paper's numbers next to the summaries' | `python3 docs/research/closing-loops/experiments/compare.py` | Markdown on the standard output |

What each script does and why:

- `twelve_models.py`: the protocol of Appendix `app:provenance` (12 models, presets organic or
  hard-surface, the 21 budgets 2,900 to 3,100 by 10, `openclare remesh` then `openclare measure
  --json`), and the means of `loop_edges_spiral`, `loop_edges_closed`, `loops_spiral`,
  `mean_distance`, `angle_deviation_mean`, and of the measures of structure the engine's
  selection does not look at (DT-137): `loop_edges_helix`, `loops_helix_missed`,
  `base_complex_patches`, `irregular_vertices`; and how the share of spiral edges is spread
  (`common.spread`, below).
- `ablation.py`: the stages of the closing turned off with the experiment options of
  `crates/openclare/src/quantization/experiment.rs` (ALGORITHM.md, M6, "Experiment options"), one
  more stage on per line, on the twelve models at the 21 budgets, with the spread of each line
  and its paired counts against the line above (`common.paired`). The last line sets no option:
  its runs are those of `twelve_models.py`, which reads them back.
- `against_remeshers.py`: reads `bench/results/results.json` and nothing else, with the paper's
  definitions: a tool *produces a mesh* for a model if a run gives a mesh whose loops can be
  measured; it *meshes validly in every run* if all its runs give valid meshes; per model, the
  bench's median over the runs (five seeds for QuadriFlow, one run otherwise); then the mean over
  the models. The tools are openclare, QuadriFlow, AutoRemesher, QuadWild-BiMDF and Instant Meshes
  (the last two run on the 27 models whose meshes may be passed on); a results file is compared on
  the tools it has, and the script prints the rows of Table `tab:remeshers` in both languages. `--git 5bc16c6 --git b0fba91` reads the files the paper used: they give back
  exactly the paper's 22, 18, 7, 15, 1.8%, 38.8%, 21.7%, 4.2%, 27.2%, 27.3%, and 16.6% and 15.5%
  without closing, which checks the definitions. The current bench has no run without closing:
  `--no-closing-run` makes it, as the bench runs openclare (the add-on, in Blender, on the model's
  scene; `make addon` first) with `OPENCLARE_CLOSE_WAYS=` (no way of closing tried). That variable
  gives the same meshes as `--no-close-loops` (checked byte for byte on blob, suzanne and spot), and
  the add-on chain of f992bac gives back the measures of the bench's openclare mesh of 524b759
  (checked on blob: same quads, spiral and closed shares, Hausdorff). The summary keeps those runs
  (`no_closing`, `per_model`, engine e13fadb); `--no-closing-from` takes the means again from them
  on the current subsets, nothing run, which is how the line was made once the subset of models
  where every tool produces a mesh went from 15 to 14 (the two new tools were not run on terrain).
- `male_body.py`: the male body at 10,590 quads, preset organic, with closing, without it, and
  with the lean bound 0.05 alone (`OPENCLARE_CLOSE_WAYS=0,1,4,5`, the paper's "commit 452b346"
  experiment), and the hand-made mesh `bench/reference/handmade/human_male.obj` measured the same
  way. It also copies the bench's own run of the add-on at the hand-made count
  (`openclare-as-handmade`), which on this engine is the same mesh as the command line's.
- `figures/make_figures.sh`: unchanged but for taking the binary (`OPENCLARE`, `COMMIT`); its
  meshes, measures and panels go to `WORK`; `figure_numbers.py` copies the measures to
  `results/figures.json`.

## Run as lab batches

The scripts loop over independent runs, and lab runs such a loop better as a `[[batch]]`, one job
per run, each with its own empty folder `{out}` (lab/SPEC.md, §6b). The options that pick one
cell: `twelve_models.py --models bob --budgets 2900`, `ablation.py --lines no_closing --models bob
--budgets 2900`, `exact/end_to_end.py --variants engine --models bob --budgets 2900` (`--models
human_male` for the male body), `exact/remesh.py --variants whole-3` (or `engine-3`, or any other
single variant), `exact/most_closable.py --ways 3`; each job also takes `--out {out} --summary
{out}/summary.json`, so that it writes nothing in the checkout. A script lays its runs out under
`--out` as it would in a run of the whole grid (`<model>-<quads>.*`, `<line>/…`, `<variant>/…`,
`call-<name>/…`). The merge is `collect.py`: it copies the files of every cell that ended ok from
the `records.json` of `lab report` into the script's default `--out`, at the same relative paths
(the jobs' `summary.json` left out), refusing a file of other bytes unless `--force`, and says
which cells are missing. The script then run unchanged on the whole grid reads every run back
(`<name>.json`, `record.json`, `way-<way>.json`) and writes the summary, nothing run again; for
example `python3 docs/research/closing-loops/experiments/collect.py <records.json> --into
scratch/data-package/closing-loops/twelve_models && python3
docs/research/closing-loops/experiments/twelve_models.py --jobs 1 --openclare <the jobs' engine>`.
The summary's engine is the binary given to the merge: `builds_of_the_runs` lists the builds the
runs read back say made them, and the script warns when they differ.

## Means over budgets, and comparisons run by run (#222)

The share of spiral edges is chaotic on one model: one per cent of the budget moves it as much as
a change of the method. Each model is therefore run at 21 budgets, 2,900 to 3,100 quads by 10
(`common.BUDGETS`), and a mean is given with:

- `common.spread`: the mean, the median, the share of runs with none, the number of models with
  some, and the 95 % bootstrap interval of the mean. The bootstrap draws the models with
  replacement, then the runs of each drawn model: the runs of one model are not independent.
- `common.paired`, between two lines: on how many runs (same model, same budget) the share is
  lower, higher, the same, and the two-sided sign test on those where it changes.

A share is given to the unit of its interval. Most stages can only lower the share, since they add
candidates and the engine keeps the one with the fewest spiral edges: their paired counts say how
often they find a better candidate, not a gain beyond that construction. Against the other
remeshers, `against_remeshers.py` counts the models where openclare has fewer spiral edges, more,
as many.

## Table `tab:dev`: what one engine can and cannot ablate

Each line is the engine of one commit with the switches of ALGORITHM.md (M6, "Experiment
options"), one more stage on per line, on the twelve models at the 21 budgets (252 runs per
line). The paper's table gives single runs at 3,000 quads of four different engines. Every line
but the last sets `OPENCLARE_EXTRA_DRAWS=0`: the engine's own lengths only, as before version
0.7.0; the last line sets no option. With three budgets (the protocol before #222), its 36
meshes were checked byte for byte against those of `twelve_models.py` (`--check`).

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

## Measures the selection does not use (DT-137)

The engine keeps, among its candidates, the one with the fewest spiral edges, and the paper
compares the tools by that same share, which sees no helix of pitch above four (`k_max = 4`).
`structure.py` measures the structure of the meshes the bench stored at 3,000 quads
(`bench/out/<model>/3000/<tool>_<seed>.obj`, the runs of `bench/results/results.json`; nothing is
remeshed) by three measures no step of the engine uses, in Python with the standard library only,
one thread, no code shared with the engine:

- **q-helices** of Bommes, Lempfer and Kobbelt (Eurographics 2011, Section 2.4) of every pitch: a
  strip of quads along which walking `tau` steps is the same, in position and orientation, as `h`
  side-steps; found, as in their Section 3, where the orthogonal strip through a quad first meets
  the strip again, then extended as far as the property holds; minimal ones only (`h <= tau`, the
  smallest `h`). Counted: those whose property holds over a whole turn (winding number `l / tau >=
  2`) and whose band between windings is checked to be a regular grid, a criterion of ours
  grounded in the paper's remark that the property implies a regular region round the helix. Read
  at a single place, the
  property also holds wherever a strip comes back beside itself once, the same way round, round
  singularities whose turns cancel: these *meetings of any winding* number in the hundreds per mesh
  and are kept in the JSON (`any_winding`) but say little.
- **The base complex after extraction**: a separatrix from each irregular vertex along each of its
  edges, straight on through regular vertices, until an irregular vertex, a border, or a vertex it
  already passed (a T-junction); patches are the regions of faces between separatrices.
- **Singularities**: inner vertices of valence other than four, by valence in the JSON.

Each (quad, direction) of a strip and each edge of a separatrix is counted once, so the double
count of #262 cannot occur here (an edge on two of the engine's loops where two fans of two quads
pinch at one vertex, in some Instant Meshes outputs; the engine's loops stop there since).

**Checks.** The hand-made cases (`--check`, run before every measure): a 20 x 10 torus grid
without shift (no helix, one patch), shifted by 1 (one helix of pitch 1, turn 10, 20 windings),
by 4 (four interleaved helices of pitch 4) and a 21 x 10 grid shifted by 7 (seven helices of pitch
7, which the spiral count cannot see), and a cube of 6 x 6 quads per side (eight corners of valence
3, six patches, no T-junction, no helix): all give what they must. On the bench meshes, the count
of irregular vertices equals the engine's `irregular_vertices` on all 395 meshes, and the patches
without the stop on itself equal the engine's `base_complex_patches` on all 345 meshes where the
bench stored it. Not checked by a hand-made case: a T-junction (no small mesh with a known nonzero
count was built), and a helix whose band holds a singularity.

**Results** (median over the models, each model the median over its runs, quartiles in
parentheses; spiral edges are the bench's stored share, over the meshes whose loops the engine
can follow; the engine's helices are its own count of every pitch, `loops_helix`, of which
`loops_helix_missed` above four, which the selection does not use either):

All the models each tool meshes:

| Tool | Models (meshes) | Spiral edges, % | q-helices per mesh | q-helices of pitch > 4 | Meetings of any winding (pitch > 4) | Engine's helices (pitch > 4) | Base-complex patches | T-junctions | Singularities |
|---|---|---|---|---|---|---|---|---|---|
| openclare | 33 (33) | 1.5 (0.0–8.7) | 2 (0–6) | 0 of 376 | 5533 (1702) | 92 (0) | 1256 (231–2058) | 5 (0–11) | 90 (40–192) |
| quadriflow | 22 (110) | 16.4 (9.6–34.7) | 2 (1–4) | 12 of 358 | 15636 (7133) | 589 (146) | 2202 (992–2601) | 13 (2–28) | 59 (25–110) |
| autoremesher | 33 (33) | 15.6 (6.3–31.4) | 3 (0–6) | 0 of 225 | 3311 (1371) | 48 (4) | 850 (563–1180) | 6 (2–18) | 84 (26–129) |
| quadwild-bimdf | 27 (27) | 12.3 (3.4–21.7) | 2 (0–6) | 2 of 216 | 23803 (16260) | 117 (5) | 1720 (298–2794) | 14 (2–24) | 56 (24–183) |
| instant-meshes-jakob | 27 (27) | 6.7 (4.9–10.0) | 0 (0–3) | 0 of 48 | 7083 (4381) | 29 (0) | 2597 (2180–2950) | 7 (4–15) | 158 (104–224) |
| instant-meshes | 33 (165) | 5.4 (3.3–9.6) | 0 (0–2) | 0 of 170 | 40989 (20468) | 186 (4) | 2366 (1697–2896) | 6 (2–10) | 204 (136–287) |

The 21 models every tool meshes (blob, blub, bob, cage, creature, cube, cylinder, jerry, lblock, ogre, part, pig, rock, rose, sphere, spot, stonemill, suzanne, torus, towermill, watermill):

| Tool | Models (meshes) | Spiral edges, % | q-helices per mesh | q-helices of pitch > 4 | Meetings of any winding (pitch > 4) | Engine's helices (pitch > 4) | Base-complex patches | T-junctions | Singularities |
|---|---|---|---|---|---|---|---|---|---|
| openclare | 21 (21) | 0.0 (0.0–7.9) | 1 (0–3) | 0 of 89 | 2679 (1184) | 53 (0) | 861 (66–1913) | 6 (0–9) | 56 (20–154) |
| quadriflow | 21 (105) | 17.6 (9.6–35.4) | 3 (1–5) | 12 of 358 | 15597 (7100) | 589 (146) | 2205 (1489–2605) | 14 (2–28) | 52 (24–106) |
| autoremesher | 21 (21) | 20.8 (7.1–33.2) | 1 (0–5) | 0 of 51 | 1630 (1025) | 40 (4) | 950 (602–1180) | 8 (4–18) | 68 (18–99) |
| quadwild-bimdf | 21 (21) | 10.3 (1.7–20.2) | 2 (0–5) | 2 of 169 | 19603 (13748) | 91 (3) | 1643 (122–2647) | 13 (0–26) | 50 (18–145) |
| instant-meshes-jakob | 21 (21) | 7.3 (4.7–9.2) | 0 (0–3) | 0 of 28 | 5769 (3561) | 23 (0) | 2867 (2379–3000) | 7 (4–16) | 137 (97–192) |
| instant-meshes | 21 (105) | 5.2 (3.0–9.6) | 0 (0–1) | 0 of 88 | 27695 (14585) | 109 (4) | 2856 (1997–2986) | 4 (2–10) | 160 (111–214) |

**How often a helix has a pitch above four.** Of the 1,393 q-helices on the 395 meshes, 14 have
a pitch above four (largest 12), in 10 meshes, all from QuadriFlow (12) and QuadWild-BiMDF (2),
none from openclare. The engine's own helix count, which asks a pass of the loop beside every
edge along a turn but no regular band, finds more: 159 of 1,061 helices above four, in 57 of the
356 meshes it can measure, 49 of them QuadriFlow's; none in openclare's 28. So the cut-off
`k_max = 4` hides no helix of openclare's meshes at 3,000 quads by either count, and hides some of
QuadriFlow's.

**What these measures do not settle.** On the 21 models every tool meshes, openclare has fewer
q-helices than QuadriFlow (median 1 against 3 per mesh) but not fewer than AutoRemesher (1) or
either Instant Meshes (0), which have the most singularities and a base complex of nearly one patch
per quad. openclare and AutoRemesher have the fewest patches (medians 861 and 950), with wide
spreads; the T-junctions are few for every tool (medians 4 to 14). The measures are not ported to
the engine (`openclare measure`); that is left.
