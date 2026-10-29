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

## Seed 1, 50 instances (2026-10-29)

38 spheres (cubes with 0 to 2 moves, hulls and duals of 6 to 12 points) and 12 tori (grids
with 1 to 4 moves), 6 to 60 patches, 0 rejected: 50 passed, 0 failed (`results/seed-1.json`,
the summary and the counts of the check). 5,138 turns, 238 of them shifted at the real lengths,
on 6 instances; 1,825 strips of the whole lengths, 31 shifted. On the spheres the Fox–Burgers
formula held on every turn and the recursion of the words found its convention (`G_f` the jump
towards the root, children reversed, rotation first) on all 38.

## Not yet here

Shifted turns are rare: the lattice of balanced lengths of these T-meshes is small (3 to 18
dimensions) and most turns bound caps; a family of tubes with dislocations inserted on purpose
is the next generator. The topology computed (Euler characteristic of a region, H₁ mod 2, free
reduction of words, feet of arcs) and the property test of each statement of the paper are the
next milestones of DT-178; surfaces with borders and of higher genus too.
