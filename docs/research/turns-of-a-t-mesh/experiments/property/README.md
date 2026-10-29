# Property tests: T-meshes that do not come from the engine

The measured statements of the turns paper (`../../paper.tex`) and the exact checks of
`../../lean/Check/Main.lean` have so far run on one kind of data: the T-meshes the engine
exports. The scripts here make T-meshes without the engine, write them in the engine's export
format, and let the same Lean oracle check the papers' definitions on them, exactly; then they
compute the topology the paper assumes and check each statement on it; then they look for
conflicts and read their certificates. A statement that holds on thousands of random surfaces
is corroborated, not proved; one that fails is refuted, and the instance is kept (DT-178).

## What an instance is

1. **A polygon mesh of a closed surface** (`quadmesh.py`): a grid glued into a torus, a cube,
   the surface of a box (a sphere with a tube between two caps), the convex hull of random
   points on a sphere, or its dual; then a few random moves that replace two adjacent quads
   by a triangle and a pentagon.
2. **One step of Catmull–Clark**: an all-quad mesh whose irregular vertices are the centres of
   the faces that are not quads and the vertices whose degree is not four (the triangle and
   pentagon of a move give two 3–5 pairs).
3. **The motorcycle graph** (`tracing.py`, after Eppstein et al. 2008): a trace leaves every
   irregular vertex along each of its edges, goes straight on, and stops at the first vertex
   that is irregular or already traced; the regions are the rectangles of the T-mesh, in the
   engine's conventions (nodes with their half-arcs counter-clockwise, half `2a` from the first
   node with its patch on the left, rims from the first corner with sides 0 to 3). A region
   that is not a rectangle makes the instance unusable, and the summary counts it.
4. **Traced again at random lengths**: on the symmetric grids of the subdivision, traces
   collide head-on at the same vertices and the feet of the arcs (Lemma "Combs") are seldom
   defined. The quad mesh of that first T-mesh at random whole balanced lengths (`grid.py`)
   is traced again: its traces meet at generic places.
5. **Lengths** (`walks.py`): the quad mesh's own, plus random strips of quads and random
   vectors of the lattice of balanced lengths (the null space of the balances), for the whole
   lengths; the same in eighths for the real lengths, which are then exact in floating point.
6. **Turns and strips** (`walks.py`, `go_round` and `shift_along` of
   `crates/openclare/src/quantization/tubes.rs` ported line by line in exact arithmetic): the
   turns walked at the real lengths from the middle of every side, held as the engine holds
   them, and every strip of quads of the whole lengths.
7. **The export** (`export.py`): `call-<name>/way-0.json` and `way-0-helices.json`, read by
   `Check/convert.py` and `Check/Main.lean` as the engine's own.

## Running it

```sh
python3 docs/research/turns-of-a-t-mesh/experiments/property/generate.py --count 50 --seed 1
python3 docs/research/turns-of-a-t-mesh/lean/Check/check_exports.py \
    scratch/data-package/property/seed-1 scratch/data-package/property/seed-1/lean-check --jobs 2
python3 docs/research/turns-of-a-t-mesh/experiments/property/properties.py --count 50 --seed 1 --pairs 60
python3 docs/research/turns-of-a-t-mesh/experiments/property/certificates.py --count 50 --seed 1
```

The same four run in CI (`.github/workflows/lean.yml`) on 20 instances of seed 1 whenever
the Lean folder or this one changes; a statement that fails there fails the build. The
first takes seconds and writes `summary.json` (families, seeds, genus, sizes, turns shifted,
rejections) beside the instances; the second needs the Lean project built (`make proofs
NOTE=turns-of-a-t-mesh` once) and takes about three seconds per instance; the last two read
nothing from the export and make the same instances again from the seed, in under a minute.
What `Main.lean` checks on each instance: the `TMesh` hypotheses, `NoArcAcross` and
`Oriented`; the balances; every shift form against the paper's definition, coefficient by
coefficient modulo the balances; the corridors and the walk coming back at `offset + shift` at
the walked, real and whole lengths, from every half-integer station; and on spheres the words of
the turns, their holonomy and the Fox–Burgers sum.

## Seed 1, 50 instances, the Lean oracle (2026-10-29)

35 spheres (cubes, boxes, hulls and duals) and 15 tori, 6 to 61 patches, 0 rejected: 50
passed, 0 failed (`results/seed-1.json`, the summary and the counts of the check). 4,898 turns,
296 of them shifted at the real lengths; 2,657 strips of the whole lengths, 137 shifted; 15,572
half-integer stations walked. On the spheres the Fox–Burgers formula held on every turn and
the recursion of the words found its convention (`G_f` the jump towards the root, children
reversed, rotation first) on all 35.

## The property tests (`properties.py`, milestone 2)

The topology the paper assumes is computed here instead (`grid.py`, `topology.py`): the quad
mesh at the whole lengths, refined by two so that the dual line of a strip and the sides of the
rectangles are both paths of it; the closed curve of every strip (the walk, then the closing
segment along the first side); the regions it cuts and their Euler characteristic; the cone
points strictly inside; the 2-colouring of the quad graph; the developed positions of the cone
points as linear forms in the lengths, along one chart, or along a path of faces inside a
region; the feet of the arcs; the traces that stop on a trace of their own vertex. Each
statement is then checked, exactly, on every object its hypothesis fits:

| Statement (paper) | Checked on | Seed 1 |
|---|---|---|
| Gauss–Bonnet, Lemma 1 (and the borders lemma at Θ = 0) | every region of every simple strip: Σ (4 − k) = 4 χ | 3,833 of 3,833 |
| One self-crossing, Lemma 2 | spheres: regions (3, 3, 2) or (2, 2, 4) by where the crossing is | 42 of 42 |
| Parity on a sphere, and rings even | every strip of a sphere | 1,450 of 1,450; 1,410 of 1,410 |
| Parity on any surface | every strip where the quad graph is bipartite; two strips bounding an annulus | 2,027 of 2,027; 1,960 of 1,960 |
| Same shift across an annulus without cone point, Prop. 2 (c) | pairs of disjoint simple strips | 898 of 898 |
| Checkerboard rule | pairs bounding an annulus on a sphere | 1,960 of 1,960 |
| Dislocation (i) | annuli holding exactly a 3–5 pair, the pair developed inside the annulus | 5 of 5 |
| Burgers pairing | spheres without valence 4k: full rank of the cone positions on the balanced lengths | 35 of 35; tori: 11 of 15 (a period is expected there) |
| Combs: the feet | arcs whose two feet exist | 850 of 3,009 |
| A separatrix that winds | a strip beside the trace with the shift δ | 0 of 25 found |

The two last lines are counts, not verdicts. The feet are undefined on two arcs in three: two
traces often stop on the same vertex from opposite sides, which the lemma's hypothesis excludes
(the paper's own export has about forty such arcs). The strip beside a winding separatrix was
never the turn of the statement here; whether the loop of the statement is freely homotopic to
a turn at all is not checked, and that is the hypothesis to compute next (word reduction,
DT-139). `results/properties-seed-1.json` keeps the first failures of each statement.

## The Lean cores (milestone 3)

Seven statements proved on paper only had their algebraic core checked in Lean after the first
version of the paper, and the parity on any surface its homomorphism: `lean/README.md` maps
them (`Consequences.lean`, `Periods.lean`, `Gadget.lean`, `AnySurface.lean`).

## The certificates (`certificates.py`, milestone 4)

On each instance, with the bound `ℓ ≥ 1` on every arc, the strips of the whole lengths and the
turns walked at the real lengths with a non-zero shift form are the set `H`; the exact
machinery of `Check/certify.py` finds up to six cores (irreducible infeasible sets) per instance
and for each an exact Farkas certificate. Then, with the certificate scaled to whole numbers:
does every arc of its support have two feet of odd valence (the hypothesis of Proposition
"Certificates are sums of combs"), and is its odd part a cycle of the graph of feet, as the
paper observed on the 25 certificates of the body? When it is not, could any representative
modulo the balances be one: modulo 2, a question of linear algebra over GF(2), a necessary
condition.

Seed 1: 9 instances have conflicts (3 spheres, 6 tori), 50 cores of 1 to 3 turns, 50 exact
certificates. 17 meet the hypothesis; the odd part is a cycle for 4 of them and not for 13, of
which 6 on spheres (boxes with a tube), and for none of the 13 could any representative be one
even modulo 2. The observation of the body is not general: the typical certificate is a single
arc `ℓ_a` between two distinct feet of valence 3 and 5, the shift of a turn being the offset of
the pair (`results/certificates-seed-1.json`, the exports with the bound under
`scratch/data-package/property/bound-1-seed-1/`).

## Not yet here

Borders (collars, the border term of the Burgers sum), genus two, the free homotopy of the
winding separatrix, and the parity of a turn complete on the combinatorial T-mesh (DT-173).
