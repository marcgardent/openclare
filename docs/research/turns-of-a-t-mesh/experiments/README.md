# Experiments of "Turns of a T-mesh": the measured statements

Scripts that make again every measured statement of the paper (`../paper.tex`) that comes from the
export of the male body: its paragraph "Data", the paragraphs "On the male body" of Sections "Turns
and singularities", "Conflicts between turns", "Any combination of turns" and "Combs", Tables
`tab:iis`, `tab:pairs` and `tab:forced`, and the numbers of Section "Whole lengths and size". The same export is the system of
Section "An exact look at the closing system" of the closing-loops paper, whose scripts
(`../../closing-loops/experiments/exact/`) read it with the module here.

> **No number here yet.** The scripts were checked on 2026-10-28 against the drafts' old export
> (`male6`, below, "Checked"); those runs are not kept. The papers' numbers are made on the engine
> of v0.7.0 as published (the `as-is` export of `export.py`, `NIGHT.md`), and their summaries then
> go to `results/`. Where an example of the drafts (the wrist, the right eye, the set of 15 turns)
> is not found on that export, the text is rewritten from what the scripts find.

Every script runs from the root of a clone and takes an export directory as its first argument
(default `scratch/data-package/turns-of-a-t-mesh/male-body`, which `export.py` makes). An export
is what the engine writes with `OPENCLARE_EXPORT_TURNS=<dir>` (ALGORITHM.md, M6, "Experiment
options"); the drafts' old export (`male6`), with its way files directly in the directory, is
read too. `--ways 0,2` keeps some ways, `--call` picks the T-mesh of a model in several pieces
(by default the one with the most arcs), `--out` takes the raw output (default
`scratch/data-package/turns-of-a-t-mesh/<group>/`, not committed), `--summary` the summary (default
`results/<group>.json`, committed after the night). Every summary records the SHA-256 of the way
files read, the export's `record.json` (engine commit and build, options, source MD5), the machine
and its load, and the versions of SciPy and HiGHS with the number of HiGHS threads.

**Threads.** HiGHS runs through SciPy (`scipy.optimize.milp` and `linprog`), with its option
`threads` set to 1 (`HIGHS_THREADS=<n>` for another number; SciPy passes the option on as it is),
and NumPy's linear algebra on one thread. `--jobs N` (where a script has it) runs N ways or
families at a time, each in a process of its own. A time limit stops a solve at the same point
only on the same machine under the same load: the long runs are for an idle machine (DT-090).

## The statements and their commands

| Statement | Command (`B=docs/research/turns-of-a-t-mesh/experiments`) | Output |
|---|---|---|
| The export itself (the published engine; `--engine before-cuts` only for the aside) | `python3 $B/export.py --openclare <binary> --commit <hash>` | `<out>/export/`, `mesh.obj`, `measure.json`, `record.json` |
| "Data": 261 singular vertices (133, 127, 1), 1,295 nodes, 2,069 arcs, 776 patches, 1,552 balances; 1,299 turns, 765 held; 28 to 36 bounds; Table `tab:forced`, columns "turns" and "left shifted by the engine"; the bidirected graph of the balances, its rank and the dimension 517 | `python3 $B/census.py <export>` | `results/census.json` |
| Section "Turns and singularities", "On the male body": the crossing maps give the shift of all 1,299 turns; 1,436 sides of the 718 simple turns, 4 to 257 singular vertices, the Burgers sum in the order of first visit, the order of last visit failing on 150 of 158 discs of way 2; the checkerboard rule on the discs and on the 30,148 annuli, of index 0; the 294 pairs with no singular vertex between them; the ten annuli with one 3–5 pair, never on one trace; parity on all turns and on the 1,838 ways round of the strips; 101 turns crossing themselves once (97 and 4), 480 up to 107 times, 356 of the 765 held turns simple | `python3 $B/holonomy.py <export> --jobs 4` | `results/holonomy.json` |
| Table `tab:iis` (sets, multipliers, bounded arcs, exact check, irreducibility in whole lengths); the wrist turn not closable alone in ways 0, 1, 2, 4, its form −(ℓ1921 + ℓ1937) and arc 1937 0.08 long; the three turns at the right eye; "every set found is infeasible in real numbers" | `python3 $B/conflicts.py <export>` | `results/conflicts.json`, `<out>/claims.json` |
| The certificates of `tab:iis` and τ_P(H) checked by Lean | `python3 docs/research/turns-of-a-t-mesh/lean/Check/certify.py <export>/export/call-*/way-[0-9].json --claims scratch/data-package/turns-of-a-t-mesh/conflicts/claims.json --out <export>/export/certificates.json`, then `make lean-check EXPORT=<export>/export` | Lean's report (`Check/README` of `../lean`) |
| Table `tab:pairs` (configurations A to F: windings, staircases, mechanisms); "A separatrix that winds" (separatrix 964 of vertex 246, 6.994 long, δ = ℓ1920 + ℓ1921 = 0.305, ℓ1920 ≡ ℓ1937); "Larger conflicts": the right eye's staircase of three arcs through arc 619, the primitive multipliers (7, 5, 2, 3) at the left hand | `python3 $B/pairs.py <export>` (after `conflicts.py`) | `results/pairs.json` |
| Section "Combs", "On the male body": Lemma "Combs" on all 2,069 arcs, 2,046 with two distinct feet of odd valence, arc 1676 on the separatrix connection of 204 and 210; the odd part of every certificate a cycle of the graph of feet | `python3 $B/combs.py <export>` (after `conflicts.py`) | `results/combs.json` |
| Section "Any combination of turns", "On the male body": every turn a word and its translation by Proposition "Fox–Burgers formula" (1,299 turns); Corollary "Checkerboard rule for any combination" on all turns and on the sets of `tab:iis`; the four turns with w18 = w22 + w101 + w65 whose translations break the relation in every frame; the twisted windings 10 ℓ1805 at the left hand; Section "Combs": the twisted homology of dimension 518, the pairing of rank 517, kernel 1 | `python3 $B/fox.py <export> --relations 2` (after `conflicts.py`) | `results/fox.json` |
| Section "Combs", "Question ... read loosely": read strictly, E and F separated at exact balanced lengths (−2.9967, −1.9973), the eyes separated, the left hand inside the cone | `python3 $B/strict.py <export>` (after `conflicts.py`) | `results/strict.json` |
| Section "Whole lengths and size": dimension 517, one row of greatest common divisor 2; lattices of dimension 363 to 469 with the held turns, Euclid's steps in ways 4, 5 and 6 | `python3 $B/lattice.py <export> --seconds 600` | `results/lattice.json` |
| Table `tab:forced`, columns "bounds", "bounds, cap", "cap, size, whole"; "exactly 1, 5, 1, 4, 1, 6, 1 and 9"; in way 7 nine turns with eight of 21 cores disjoint; at most 66 s per family; exactly one in ways 0 and 2 with the cap and the window, and the whole lengths that show it; way 3: 4 in real lengths, at least 5 in whole; the optimum 4 for way 2 keeping open every arc the engine keeps open | `python3 $B/forced.py <export> --whole-lower --jobs 4`; `python3 $B/forced.py <export> --ways 2 --families held --sound --summary $B/results/forced-sound.json` | `results/forced.json`, `results/forced-sound.json`, whole lengths under `<out>` |
| Paper A, Section "An exact look", and the rounding gap 0.399 / 0.536 quoted in "Whole lengths and size" | `../../closing-loops/experiments/exact/README.md` | |

## What each script does

- `tmesh.py` (a module): finds the way files of an export, reads a way (extending `Way` of
  `../lean/Check/certify.py`, whose exact arithmetic it uses: rows of the balances, Farkas
  certificates, whole lengths checked), its matrices, number of quads, deviation, strips, and the
  linear and integer programs every script poses; the record of a summary.
- `geometry.py` (a module): the geometry of the turns on the T-mesh the export carries: patch
  frames, crossing maps, turns as curves (chords of the unit square per patch), the regions they
  cut, development along a spanning tree of cells, the contour order of a disc, monotone paths of
  arcs, winding numbers of a weighted cycle.
- `integer.py` (a module): the lattice of whole solutions by exact elimination (Euclid's
  algorithm where no coefficient is a unit), and LLL with Babai's nearest plane.
- `export.py`: one run of the engine with the export on. By default the engine as published
  (`as-is`), on which the papers' numbers are made. `--engine before-cuts` sets the experiment
  options that switch off the stages the engine of the drafts' first export did not have (no
  constraint rounds, no active set, no floor of the cap, no fold guard, no other draws): an aside,
  to compare with that export, whose summaries stay in `scratch/` (`NIGHT.md`, stage 5).
- `census.py`: counts. The rank of the balances is computed (dense, floating point) and the graph
  facts (each arc in two balances with ±1, components, an unbalanced cycle) by a signed search.
- `holonomy.py`: for every turn of every way, the composed crossing maps (a translation; the
  exported shift modulo the balances, for each sign convention), the parity, the self-crossings
  and regions of its curve; for each disc of a simple turn, the Burgers sum in the order of first
  and of last visit (the convention of the paper and any convention), the checkerboard rule; then
  the pairs of simple turns: annuli (index, checkerboard), pairs splitting the singular vertices
  alike (handed shifts equal), annuli with one 3–5 pair (identity (i), straight chains,
  staircases). Identities modulo the balances are tested on a basis of the balanced lengths
  (floating point); the counts are summed over the ways, with repeats, as the paper's. The pairs
  are selected as the scripts of the first version selected them.
- `conflicts.py`: per way, the held turns closable with ℓ ≥ 0 and within the bounds, those not
  closable alone, and irreducible infeasible sets by a deletion filter over the held turns in the
  engine's order (the first turn of each set dropped, the search repeated, at most `--most` = 12
  sets a way), as the table was made; each set's exact certificate (least Σ|u| with c · b = 1, and
  primitive whole multipliers) and, for each turn left out, whole lengths closing the others,
  checked exactly. Sets of the same turns in several ways make one row, with the certificate of
  the first way listed (another way may give another representative modulo the balances, as row 6
  of the paper's table gives for row 4's turns).
- `pairs.py`: for each row of at most `--largest` turns, the crossings, the windings of the cycle
  Σ u γ round the regions, monotone staircases of at most `--depth` + 1 arcs between the singular
  vertices of the regions where it winds whose length equals the combination modulo the balances,
  and the mechanism; for a single turn, the winding separatrix. The homotopies of Propositions
  "Exchanging two vertices" and "A separatrix that winds" are not checked, as in the paper.
- `combs.py`: the feet of every arc (the vertices where the traces perpendicular to it at its ends
  start), the comb lemma checked by developing the path foot–arc–foot, and, for every certificate
  of `conflicts.py`'s table, the boundary modulo 2 of its odd part on the graph of feet.
- `fox.py`: the one chart of the surface cut along a tree of the nodes, the jumps across the tree
  written as words by the recursion of Proposition "Burgers sums" (a) (its conventions tried, one
  kept only if it holds at every node, as `../lean/Check/develop.py` does), each turn's word, its
  holonomy and Fox derivatives checked against the walk; the colours of the nodes and the
  checkerboard rule for every turn and every set (windings of Σ u γ with the turns as walked,
  zero on the region with the most nodes); the twisted windings of each set of at most
  `--largest` turns, the formula, and the 3–5 pairs whose contributions are a multiple of one arc
  length; the rank of the pairing on the twisted cycles (a basis of the balanced lengths, floating
  point); with `--relations`, a least-L1 relation among the windings of each turn and the others
  (whole coefficients, at most six terms), tested on the translations in every choice of
  quarter-turn frames.
- `strict.py`: the separation by shortest walks of the docstring, with the forms of the strict
  reading (diagonal walks between odd vertices and closed, closed axial walks, axial walks of a
  winding separatrix; `--loose` adds axial walks between two odd vertices, weight two), for the
  non-negative representative of least total of each set's certificate; a separating point made
  exact (free arcs rounded, the others solved in rational arithmetic) and checked again in
  integer arithmetic.
- `lattice.py`: the balances, then the held turns, eliminated in whole numbers per way, stopped
  after `--seconds` (in way 7 of the drafts' old export the coefficients grow without end).
- `forced.py`: τ_P(H) by implicit hitting sets for the bounds alone (exact), with the cap (exact or
  an interval after `--budget`), and with the cap, the window and whole lengths (an interval: the
  lower bound of the cap, raised with `--whole-lower` by hitting sets of no-goods in whole lengths
  within the cap; the upper bound by whole lengths closing the complement of the cap's least set,
  else by the program over a binary per turn, every solution checked exactly).

## What the scripts cannot make again, and why

- **The drafts' old export.** It was the hook of branch `rd-exact` on the engine before commit
  `c8bf2be`. The papers now describe the published engine of v0.7.0, whose export (`as-is`)
  has another field, T-mesh and work mesh, more stages (constraint rounds, active set, floor of the
  cap, fold guard, other draws) and so other turns: the turn numbers, arc numbers and vertex
  numbers of the drafts (wrist turn 42, arc 1937, vertex 246, configurations A to F...) do not
  survive. The tables are made again, with new numbers, and the text follows them. The hooks are
  on main since DT-089 (`OPENCLARE_EXPORT_TURNS`, with the same fields and more);
  `export.py --engine before-cuts` comes closest to the old engine on today's code, for comparison
  only.
- **Witnesses.** Where the paper quotes one solution among many (the whole lengths at 10,439 and
  10,436 quads, deviations 0.442 and 0.469, that show τ = 1 in ways 0 and 2), the scripts find
  their own; on the drafts' old export (`male6`) they found 10,731 and 10,638 quads, 0.430 and
  0.386.
- **Intervals.** The ranges of `tab:forced` depend on the time limits and on the machine; the paper
  says "within the time limit (120 to 1,800 seconds)". The night fixes them (`NIGHT.md`).
- **The relation of windings.** `fox.py --relations K` finds the K first relations among the
  winding numbers of single turns that the translations break, in the order of the turns; which
  four turns it names, and with which coefficients, depends on the export (on the drafts' old
  export `male6` it names first turn 18 of way 0 and turns 22, 101 and 65 of ways 0, 4 and 5: the paper's
  w18 = w22 + w101 + w65).
- **Other models.** Section "Borders and handles" (Suzanne, Jerry, Bob, part, cage, a dome; the
  314 and 586 sides holding a border, the 1,076 sides, the 660 turns, the 40 collars, Bob's pairs),
  the 40 random graphs of the reduction of Theorem "Choosing the turns is NP-hard" and the
  subdivided octahedra and icosahedra of Appendix "Rotations of other orders" do not come from
  the male body and are not scripted here. The "294" closed loops of the output meshes of
  Theorem "Parity of closed loops" are not either.

## Long runs

Times on an Intel Core i7-7700K (4 cores, 8 threads, 2017, 62 GB), loaded by other jobs (load 4 to
9) and the scripts under `nice`, one HiGHS thread:

| Script | On the drafts' old export (`male6`) |
|---|---|
| `census.py`, `combs.py`, `pairs.py` (rows of 2 to 4 turns) | seconds |
| `conflicts.py` | about 2 minutes for the eight ways (way 7: 50 s) |
| `lattice.py` | seconds for ways 0 to 6; way 7 stops at `--seconds` |
| `holonomy.py --no-annuli` | 2.5 minutes for the eight ways; the pairs of turns about 1 minute per 400 pairs: about an hour more for all eight ways (with `--jobs 4`, a quarter of that) |
| `fox.py` | 10 to 18 s a way; `--relations` adds about 4 minutes (the windings of every turn, then a linear program each) |
| `strict.py` | 8 s (configuration A, in the cone) to 75 s (F, separated) a set; the sets of four and more, minutes each |
| `forced.py --steps bounds` | 75 s for the nine families (way 7: 45 s) |
| `forced.py` (cap and size) | up to `--budget` (1,800 s) a family for the cap and up to 6 × `--seconds` (900 s) for the size: hours; ways 0 and 2 take 15 s |

## Checked on 2026-10-28

On the drafts' old export (`male6`: the engine before `c8bf2be`, 8 ways, 1,299 turns; its way files
have the SHA-256 listed in `../lean/Check/certificates/male-body.claims.json`), the scripts give
back the drafts' numbers: every number of "Data" and the columns "turns" and "left
shifted by the engine" of `tab:forced` (`census.py`); every count of "On the male body" of Section
"Turns and singularities" but the three that rest on pairs of turns (30,148, 294, ten), whose code
was only run on the first 400 pairs of way 0 (`holonomy.py`); every row of `tab:iis`, with the same
multipliers, bounded arcs and irreducibility (the last row closes without each of its 15 turns, as
the Lean check found), the wrist and the right eye (`conflicts.py`); the six configurations of
`tab:pairs`, with the same singular vertices, staircases, lengths and mechanisms, the winding
separatrix of the wrist, the right eye's staircase and the left hand's multipliers (`pairs.py`);
the comb lemma, 2,046, arc 1676 and the odd parts (`combs.py`); the Fox–Burgers formula and the
checkerboard rule on all 1,299 turns and on every set of the table (the set
of 15 turns included), the twisted windings 10 ℓ1805 at the left hand, 518 and 517 (`fox.py`), and
the relation w18 = w22 + w101 + w65 (`relations` of `fox.py`, run on windings read from a cache to
stay within today's time); E and F separated at −2.9967 and −1.9973 in exact arithmetic, and A
inside the cone (`strict.py`); the dimensions 517 and 363 to 469
and Euclid's steps (`lattice.py`); the column "bounds" of `tab:forced` and its 21 cores, eight
disjoint, in way 7; exactly one with the cap and the window in ways 0 and 2; at least 5 in whole
lengths in way 3 (`forced.py`, short limits). Not run: the other cells of the columns with the cap
and the size, which need the night's limits.
