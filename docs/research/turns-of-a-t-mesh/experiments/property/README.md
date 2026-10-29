# Property tests: T-meshes that do not come from the engine

The measured statements of the turns paper (`../../paper.tex`) and the exact checks of
`../../lean/Check/Main.lean` have so far run on one kind of data: the T-meshes the engine
exports. The scripts here make T-meshes without the engine, write them in the engine's export
format, and let the same Lean oracle check the papers' definitions on them, exactly. A
statement that holds on thousands of random surfaces is corroborated, not proved; one that
fails is refuted, and the instance is kept (DT-178, milestone 1).

## What an instance is

1. **A polygon mesh of a closed surface** (`quadmesh.py`): a grid glued into a torus, a cube,
   the convex hull of random points on a sphere, or its dual; then a few random moves that
   replace two adjacent quads by a triangle and a pentagon.
2. **One step of Catmull–Clark**: an all-quad mesh whose irregular vertices are the centres of
   the faces that are not quads and the vertices whose degree is not four (the triangle and
   pentagon of a move give two 3–5 pairs).
3. **The motorcycle graph** (`tracing.py`, after Eppstein et al. 2008): a trace leaves every
   irregular vertex along each of its edges, goes straight on, and stops at the first vertex
   that is irregular or already traced; the regions are the rectangles of the T-mesh, in the
   engine's conventions (nodes with their half-arcs counter-clockwise, half `2a` from the first
   node with its patch on the left, rims from the first corner with sides 0 to 3). A region
   that is not a rectangle makes the instance unusable, and the summary counts it.
4. **Lengths** (`walks.py`): the quad mesh's own, plus random strips of quads and random
   vectors of the lattice of balanced lengths (the null space of the balances), for the whole
   lengths; the same in eighths for the real lengths, which are then exact in floating point.
5. **Turns and strips** (`walks.py`, `go_round` and `shift_along` of
   `crates/openclare/src/quantization/tubes.rs` ported line by line in exact arithmetic): the
   turns walked at the real lengths from the middle of every side, held as the engine holds
   them, and every strip of quads of the whole lengths.
6. **The export** (`export.py`): `call-<name>/way-0.json` and `way-0-helices.json`, read by
   `Check/convert.py` and `Check/Main.lean` as the engine's own.

## Running it

```sh
python3 docs/research/turns-of-a-t-mesh/experiments/property/generate.py --count 50 --seed 1
python3 docs/research/turns-of-a-t-mesh/lean/Check/check_exports.py \
    scratch/data-package/property/seed-1 scratch/data-package/property/seed-1/lean-check --jobs 2
```

The first takes seconds and writes `summary.json` (families, seeds, genus, sizes, turns shifted,
rejections) beside the instances; the second needs the Lean project built (`make proofs
NOTE=turns-of-a-t-mesh` once) and takes about three seconds per instance. What `Main.lean`
checks on each: the `TMesh` hypotheses, `NoArcAcross` and `Oriented`; the balances; every
shift form against the paper's definition, coefficient by coefficient modulo the balances; the
corridors and the walk coming back at `offset + shift` at the walked, real and whole lengths,
from every half-integer station; and on spheres the words of the turns, their holonomy and
the Fox–Burgers sum.

## Seed 1, 50 instances, the Lean oracle (2026-10-29)

33 spheres (cubes with 0 to 2 moves, hulls and duals of 6 to 12 points) and 17 tori (grids
with 1 to 4 moves, long tubes with one move), 6 to 60 patches, 0 rejected: 50 passed, 0 failed
(`results/seed-1.json`, the summary and the counts of the check). 3,851 turns, 259 of them
shifted at the real lengths, on 7 instances; 2,109 strips of the whole lengths, 43 shifted. On
the spheres the Fox–Burgers formula held on every turn and the recursion of the words found its
convention (`G_f` the jump towards the root, children reversed, rotation first) on all 33.

## The property tests (`properties.py`, milestone 2)

The topology the paper assumes is computed here instead (`grid.py`, `topology.py`): the quad
mesh at the whole lengths, refined by two so that the dual line of a strip and the sides of the
rectangles are both paths of it; the closed curve of every strip (the walk, then the closing
segment along the first side); the regions it cuts and their Euler characteristic; the cone
points strictly inside; the 2-colouring of the quad graph; the developed positions of the cone
points as linear forms in the lengths, along one chart; the feet of the arcs; the traces that
stop on a trace of their own vertex. Each statement is then checked, exactly, on every object
its hypothesis fits:

| Statement (paper) | Checked on | Seed 1 |
|---|---|---|
| Gauss–Bonnet, Lemma 1 (and the borders lemma at Θ = 0) | every region of every simple strip: Σ (4 − k) = 4 χ | 2,803 of 2,803 |
| One self-crossing, Lemma 2 | spheres: regions (3, 3, 2) or (2, 2, 4) by where the crossing is | 2 of 2 |
| Parity on a sphere, and rings even | every strip of a sphere | 812 of 812; 808 of 808 |
| Parity on any surface | every strip where the quad graph is bipartite; two strips bounding an annulus | 1,190 of 1,190; 1,592 of 1,592 |
| Same shift across an annulus without cone point, Prop. 2 (c) | pairs of disjoint simple strips | 731 of 731 |
| Checkerboard rule | pairs bounding an annulus on a sphere | 1,592 of 1,592 |
| Dislocation (i) | annuli holding exactly a 3–5 pair | none found yet |
| Burgers pairing | spheres without valence 4k: full rank of the cone positions on the balanced lengths | 33 of 33; tori: 11 of 17 (a period is expected there) |
| Combs: the feet | arcs whose two feet exist | 415 of 2,310 |
| A separatrix that winds | a strip beside the trace with the shift δ | 0 of 30 found |

The two last lines are counts, not verdicts. The feet are undefined on four arcs in five here:
in these meshes two traces often stop on the same vertex from opposite sides, which the lemma's
hypothesis excludes (the paper's own export has about forty such arcs). The strip beside a
winding separatrix was never the turn of the statement on these tori: both strips beside it
are rings; whether the loop of the statement is freely homotopic to a turn at all is not checked
here, and that is the hypothesis to compute next (word reduction, DT-139).

```sh
python3 docs/research/turns-of-a-t-mesh/experiments/property/properties.py --count 50 --seed 1 --pairs 60
```

reads nothing from `generate.py`'s output: it makes the same instances again from the seed, and
writes `results/properties-seed-1.json` with the first failures of each statement.

## Not yet here

Annuli holding exactly one 3–5 pair, borders (collars, the border term of the Burgers sum),
genus two, the free homotopy of the winding separatrix, and the certificates of conflicts (the
odd part of a certificate as a cycle of the graph of feet, DT-142) are the next milestones of
DT-178.
