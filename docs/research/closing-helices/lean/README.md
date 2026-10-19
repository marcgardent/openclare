# Machine-checked cores of "Closing Edge Loops at Quantization Time"

This Lake project checks with Lean 4 and Mathlib the **algebraic cores** of the
results of the research note `../paper.tex` (Sections 3, 7 and 9, and
Appendix F): 187 `theorem` declarations, many of them small lemmas and a few
restating a hypothesis (`dislocation_closes_iff`, `staircase_rings`); the
count is not evidence, the table below names the main ones. The topological facts the note uses (that a loop develops into a given
motion, that a curve bounds a disc, that the quad graph of a sphere is
bipartite, ...) are not formalized: each one is an explicit hypothesis of the
theorem that uses it, never a hidden `sorry`. Every theorem depends only on
Lean's standard axioms (`propext`, `Classical.choice`, `Quot.sound`).

## Replay

```sh
make proofs NOTE=closing-helices      # from the root of the repository
```

or, in this folder, `lake exe cache get && lake build` (with `~/.elan/bin` on
the `PATH` if `lake` is not). The project pins Mathlib `v4.35.0` and the toolchain
`leanprover/lean4:v4.35.0` (`lean-toolchain`, `lake-manifest.json`). The first run
downloads the toolchain (about 3 GB in `~/.elan`) and Mathlib compiled (about
7 GB in `.lake`, which git ignores); after that, building the note's own files
takes about 40 seconds and 1.3 GB of memory.

## What is proved

The plane of a patch is `ℂ` (real lengths) or `ℤ[i]` (whole lengths), with
`e₁ = 1` the walking direction, `e₂ = i` along the first side, and the
quarter-turn `R(a, b) = (-b, a)` the multiplication by `i`, as in
`quantization/tubes.rs` (`turn_expression`). A motion `x ↦ a • x + t` is a
`Motion A V`; `comp f g` is `f ∘ g` and `composeAll [ρ₁, …, ρₘ]` is
`ρₘ ∘ ⋯ ∘ ρ₁`.

| Statement of the note | Lean | Assumed (hypotheses) |
|---|---|---|
| Prop. "The shift of a turn is linear", the additivity only | `Motion.composeAll_trans_add`, `Motion.composeAll_trans` | the crossing maps have linear parts independent of the lengths and translations additive in them; the integer coefficients and the reading as a helix (with its corridor) are not formalized |
| Prop. "Completeness at given lengths", in a model only | `stripsEquiv` (in a model where the strips are *defined* as the cycles of a successor map built from a permutation `walk` of the stations, so the bijection holds by construction; the station walk of the gluing data, `stationWalk`, is not shown to be that permutation), `exists_station`, `sameCycle_station_iff`; `realWalk_iterate_half`, `realWalk_iterate_open` (the walk from `k + ½`, or from any point of `(k, k + 1)`, follows the unit edges); `glueLine_half`, `glueLine_open`, `oddCount_motion` (a quarter-turn with a whole translation keeps nodes, edge midpoints and quad centres apart) | the patches are integer rectangles with positive sides (`hdepth`) glued along their arcs by `t ↦ ±t + c`, `c` whole, mapping the unit edges of the sides one to one (a closed surface); that the station walk is a permutation and that the model is the quad mesh |
| Lemma "Gauss–Bonnet for turns" | `gauss_bonnet_turn` (`∑ (4 - k) = 4 χ` inside a region whose border turns by zero), `gauss_bonnet`, `turn_border_sum`, `sum_deg`, `sum_corners` | that `γ̄`, on the grid refined by two, bounds a disc (or an annulus): a complex of quads with Euler characteristic `χ` whose interior vertices have a cycle as link and border vertices a path (`IsRegion`) |
| … the rotation part | `quarter_turns_prod_eq_one_iff` (`R^{j₁}⋯R^{jₘ} = I ⇔ 4 ∣ ∑ jᵢ`) | — |
| Lemma "One self-crossing" | `one_crossing_on_walk` (totals `3, 3, 2`), `one_crossing_on_segment` (totals `2, 2, 4`), from `gauss_bonnet_corners` (`4 χ - a + b` with `a` convex and `b` concave corners on the border) | the regions cut open at the crossing are discs, the crossing doubled on the border of the third; how many convex and concave corners each region has (`a - b = 1, 1, 2` on the walk, `2, 2, 0` on the closing segment), the note's case analysis |
| Prop. "The shift is set by the singularities" (a) | `Motion.composeAll_rotations`, `Motion.burgers_eq_sum` (`B = ∑ Qᵢ (I - Rᵢ) pᵢ`, `Qᵢ = Rₘ⋯Rᵢ₊₁`), `Motion.translation_inv` (the sign) | that `ĥ(γ̄)` is the composition of the rotations round the cone points |
| … the order of first visit | `Forest.glue_eq_preorder`, `Forest.glue_rotations`; `Forest.order_of_first_visit_matters` (last visit gives another vector); all in `FirstVisit.lean`, namespace `Forest` | the recursion `G_f = G_{f_r} ∘ ⋯ ∘ G_{f_1} ∘ ρ_v` read off the cut tree |
| Prop. (b), annuli | `Motion.annulus_holonomy` (`ĥ(γ̄₁ κ γ̄₂⁻¹ κ⁻¹)` is the translation by `T₁ - M T₂`), `Motion.conj_translation` | the homomorphism `ĥ` and the holonomies of the two turns |
| Prop. "A dislocation between two turns" (i), (ii) | `dislocation_burgers` (the four orders and orientations), `dislocation` (`±d₁ ± d₂`, `‖B‖² = 2‖d‖²`), `dislocation_closes_iff`, `dislocation_straight` | as in (b) |
| Prop. "Staircases" | `staircase`, `staircase_rings`, `nonneg_form_pos` | `d = ∑ ℓᵢ vᵢ` along the staircase (development along homotopic paths) |
| Prop. "Exchanging two vertices" | `Motion.exchange` (any rotation), `exchange_quarter` | the homotopy `λ ≃ λ_c λ_{c'}⁻¹` |
| Prop. "A separatrix that winds", the conjugation step | `Motion.conj_translation` | the free homotopy |
| Parity of `(I - R^{±1}) p`, `(I - R²) p` | `parity_one_sub_quarter`, `one_sub_half_turn`, `quarter_mul_parity`, `colour_rotAbout` | — |
| Prop. "Checkerboard rule" | `checkerboard`, `oddColours_translate` | the Burgers sum of Prop. (b) |
| Prop. "Parity of a turn on a sphere" | `parity_chain`, `parity_of_turn` (the induction of the note's proof); `colour_hol_trans` (a second proof) | a proper 2-colouring of the quad graph; for the second proof, that the turn is a word in the loops round the cone points (closed genus-zero surface) |
| Prop. "Fox–Burgers formula" | `fox_burgers`, `fox_fundamental`, `fox_burgers_combination`, `twisted_relation`, `combination_translate`, `insert_relation`; on the free group: `holFG`, `foxFG` | the presentation of `π₁(Σ°)` by the loops `λᵢ` |
| Corollary "Checkerboard rule for any combination" | `checkerboard_word`, `checkerboard_combination`, `colours_flip`, `relation_colours`, `winding_constant` | as above; the relation `ϱ = λ₁⋯λₙ` has trivial holonomy |
| Prop. "Winding numbers do not suffice" | `commutator_holonomy`, `winding_numbers_do_not_suffice` | — |
| Lemma "Handshake" | `handshake`, `column_even`, `no_whole_solution` | — |
| Prop. "Free arcs" | `free_arcs` (unique whole extension), `free_arcs_basis` (`z_f` with entries in `{0, ±1, ±2}`), `rows_independent` (full row rank), `solve_unicyclic` (the whole image of `B` on `T ∪ {f}` is the lattice of even sums), `exists_closesUnbalanced` (the edge closing an unbalanced cycle exists), `free_arcs_of_cycle` (as stated: a spanning tree and an unbalanced cycle) | that the graph is connected, as the existence of a spanning tree grown by leaves (`IsTree`) |
| … bidirected graphs | `IsTree.exists_switching`, `IsTree.eq_zero`, `IsTree.solve`, `closesUnbalanced_of_pair`, `pair_ne_zero`, `balanced_cycle_of_not_closesUnbalanced`, `Cycle.isBalanced_of_graph` (a cycle of a balanced graph is balanced) | — |
| Remark "Sign-definite turns", Farkas certificates | `sign_definite_certificate` (`B ℓ = 0`, `shift_γ(ℓ) = 0`, `ℓ ≥ b` infeasible ⇔ `∑ u_γ shift_γ + λᵀ B = c ≥ 0` with `c ⬝ b > 0`), `forced_zero_iff` (arcs zero in every solution), `farkas_int` (whole multipliers), `farkas`, `farkas_bounds` (any linearly ordered field, by Fourier–Motzkin) | — |
| Remark "Whole solutions exist at some size" | `whole_solution_of_real`, `clear_denominators` | — |
| Theorem "Forced shifts" | `forced_helices`, `meets_infeasible`, `card_ge_of_disjoint`, `tauP_ge_of_disjoint` | — |
| Corollary "Chains of dislocations" | `sign_change_between`, `chain_of_dislocations` | the identities `s_{i-1} - s_i = εᵢ Fᵢ` with `Fᵢ > 0` |

Not formalized: the topology behind the hypotheses, weeks of work with much of
it missing from Mathlib. That a simple closed curve on a sphere bounds a disc on
each side (Jordan–Schoenflies) and that the refined quads inside form a disc in
the combinatorial sense (`IsRegion`); the developing map, holonomy as a
homomorphism of the fundamental group and its presentation by the loops round
the cone points; which corners of the regions of Lemma "One self-crossing" are
convex (the totals are proved from them). Two standard graph facts, days of work: a finite graph is connected
exactly when it has a spanning tree grown by leaves, and Harary's converse (a
graph all of whose cycles are balanced has a switching; only the direction the
proposition needs is proved). Strips on surfaces with borders (the walk is then
a partial bijection, whose open paths `stripsEquiv` does not cover). And every
measured statement.

## Gaps and corrections found in the note

1. **Fox calculus, inserting the relation.** Inserting a conjugate `g ϱ g⁻¹` of
   the relation in `w_r` *after a prefix `a`* adds `ρ(a g ∂ϱ/∂xᵢ)ᵀ f_r` to `Wᵢ`,
   not `ρ(g ∂ϱ/∂xᵢ)ᵀ f_r`; the note's formula is the case of an insertion at
   the front (`insert_relation`). The conclusion (the sum does not change) is
   right.
2. **Theorem "Forced shifts" needs neither convexity nor whole lengths.** The
   proof holds for any set `P` of lengths and any `ℓ ∈ P`
   (`forced_helices`); convexity only makes `τ_P` computable by linear
   programming. So the theorem applies verbatim to the non-convex set of
   lengths with a given number of quads, the open problem "Lower bounds with
   the size": what is open there is computing or certifying `τ_P`, not the
   bound. Also, `τ_P(H)` is a minimum over an empty set when `P` is empty;
   the bound `τ_P(H) ≥ k` needs `P` non-empty (`tauP_ge_of_disjoint`).
3. **"Genus zero" must mean closed (a sphere).** Prop. "Parity of a turn on a
   sphere" and Corollary "Parity" (a) fail on a sphere with two or more
   border loops: an open cylinder of three quads round has a ring of three
   quads. The proofs use `H₁(S²; ℤ/2) = 0` and the presentation of `π₁(Σ°)` by
   the `λᵢ` with one relation, both of a closed sphere. On the bench, this
   matters for models with borders (suzanne, whose eye sockets carry rings of
   23 quads) and with handles (bob, part, cage). The note now gives the rule on
   any surface (Theorem "Parity of a turn on any surface", proved on paper).
4. **Prop. "Parity of a turn on a sphere", frame.** The frame `(e₁, e₂)` of the
   first side is that of the first patch up to a quarter-turn, not only up to
   signs (when the first side is horizontal the coordinates swap). Harmless for
   the parity; `parity_of_turn` takes any quarter-turn.
5. **Prop. "Staircases" is more general than stated**: the identity needs only
   `ε₁ vᵢ₁ + ε₂ vᵢ₂ = 1` for each step, not `vᵢ ∈ {±e₁, ±e₂}` nor `εⱼ = ±1`
   (`staircase`).
6. **Prop. "Free arcs"** uses an edge closing an unbalanced fundamental cycle of
   a spanning tree; it exists because the sign of a cycle is a homomorphism on
   the cycle space, of which the fundamental cycles are a basis. Not stated.
   Now proved (`exists_closesUnbalanced`), with switchings in place of the cycle
   space: signs `w` on the nodes balancing the tree exist, and an edge they do
   not balance closes an unbalanced cycle.
7. **Prop. "Checkerboard rule"** does not say which colour is "white"; either
   works, the number of vertices of odd valence in the annulus being even
   (`oddColours_translate`).

8. **Prop. "Free arcs" is stronger than stated.** On a spanning tree and an
   edge closing an unbalanced cycle, a demand `d` is `B x` for a whole `x`
   exactly when `∑_r d_r` is even (`solve_unicyclic`): Lemma "Handshake" is the
   only obstruction to whole solutions of `B x = d`, not only a sufficient one.
   The whole image of `B_T` has index 2 (its determinant is `±2`, not `±1`),
   which is the one pivot of greatest common divisor 2 the note measures. The
   entries of the `z_f` are bounded by `∑_r |d_r| = 2` along the leaves, a
   shorter proof than the note's case analysis.
9. **Remark "Sign-definite turns": whole multipliers need whole bounds.** The
   multipliers can be taken whole when the bounds `b` are whole as well as `B`
   and the shifts (`farkas_int`); with real bounds only the rational case
   scales.
10. **Prop. "Completeness at given lengths": "that comes back to the side it
   left" is always true on a closed surface.** The walk on the half-integer
   stations is a permutation (the gluing maps unit edges one to one), so every
   strip is a cycle of it and comes back to every station it crosses; every
   strip meets a side of a patch (`exists_station`), and two stations lie on
   the same strip exactly when the walk takes one to the other
   (`sameCycle_station_iff`). Each oriented strip is one cycle; "each strip is
   found twice" also needs the surface oriented, so that the two directions are
   different cycles. On a surface with borders, strips reaching a border are
   open paths of a partial bijection.
11. **Remark "Whole solutions exist at some size"** holds as argued; the proof
   here goes through Farkas' lemma over the rationals (`whole_solution_of_real`)
   rather than a vertex of the polyhedron, which exists since `ℓ ≥ b` makes it
   pointed.

No sign or convention error was found: the four orders and orientations of
Prop. "A dislocation" (`dislocation_burgers`), the order `Qᵢ = Rₘ⋯Rᵢ₊₁`
of Prop. "The shift is set by the singularities", the Fox derivatives of the
commutator and `(R - I)² = -2R` all check.

A second proof of the parity of a turn came out of the formalization
(`colour_hol_trans`): on a closed sphere every loop is a word in the loops
round the cone points, each a quarter-turn about a whole point for whole
lengths, and such rotations keep the colour `p₁ + p₂ mod 2`, so the
translation of every loop has `T₁ + T₂` even. It needs neither the bipartite
quad graph nor positive lengths.

## What generalizes

* **Any rotations, any dimension.** Motions, the Burgers sum, the order of
  first visit, annuli, exchanges, the Fox–Burgers formula and twisted windings
  are proved for linear parts in an arbitrary ring `A` acting on a module `V`
  (`Motion.lean`, `FirstVisit.lean`, `Fox.lean`), so they hold for the
  rotations of order 6 of triangular and hexagonal meshes, for cone metrics of
  any angles, and for motions of space.
* **Dislocations** depend on the order `n` of the rotation: `|1 - ζₙ| = 2 sin(π/n)`,
  so `‖B‖ = √2 ‖d‖` for quads, `‖B‖ = ‖d‖` for a pair of valences 5 and 7 on a
  triangle mesh; the "diagonal" forms `±d₁ ± d₂` are particular to quads.
* **Staircases**: any steps on which the functional of the shift is `1`.
* **Parity** becomes a congruence modulo `(1 - z)²` (`burgers_congr`):
  `τ(w) ≡ (1 - z) ∑ᵢ εᵢ(w) jᵢ pᵢ`. For quads `(1 - z)² = -2z`, a congruence
  modulo 2 (the checkerboard); for rotations of order 3, `(1 - z)² = -3z`,
  three colours modulo 3; order 2, modulo 4; for order 6 (triangle and hex
  meshes) `1 - z` is a unit and there is **no** colouring rule
  (`one_sub_sq_order_*`, `one_sub_unit_order_six`). In general `1 - ζₙ` is a
  unit of `ℤ[ζₙ]` exactly when `n` is not a prime power.
* **Forced helices and chains** use nothing of the geometry: any family of
  functions, any set `P`.
* **Handshake**: any integer matrix whose columns have even sums.
* **Free arcs, Farkas**: any bidirected graph; Farkas over any linearly ordered
  field.

## Beyond quads

A field of order `n` turns the chart by a power of `ζₙ = exp(2πi/n)` round each
cone point of valence `k`, by `ζₙ^{n-k}`. Proved for every `n`
(`BeyondQuads.lean`, `GaussBonnet.lean`, `Congruence.lean`):

* **Gauss–Bonnet, the rotation part**: `ζₙ^{j₁} ⋯ ζₙ^{jₘ} = 1 ⇔ n ∣ ∑ jᵢ`
  (`rotations_prod_eq_one_iff`).
* **Gauss–Bonnet, the count**, for faces with `p` corners and regular valence
  `q`, `p q = 2p + 2q` (`gauss_bonnet_general`):
  `2 ∑_{inside} (q - deg) + ∑_{border} (q - 2 corners) = 2 q χ`. Quads
  (`p = q = 4`): `∑ (4 - k) = 4χ`; triangles (`p = 3`, `q = 6`, fields of order 6):
  `∑ (6 - k) + ∑_{border} (3 - corners) = 6χ` (`gauss_bonnet_triangles`);
  hexagons (`p = 6`, `q = 3`, fields of order 3) (`gauss_bonnet_hexagons`). A
  closed sphere holds total index `2q`: 8 for quads, 12 for triangles, 6 for
  hexagons.
* **Dislocations**: `‖(1 - ζₙ) d‖ = 2 sin(π/n) ‖d‖` (`dislocation_norm`):
  `√2 ‖d‖` for quads (`dislocation_norm_four`), `√3 ‖d‖` for order 3
  (`dislocation_norm_three`), `‖d‖` for order 6 (`dislocation_norm_six`).
* **Parity**: the congruence modulo `(1 - z)²` above (`burgers_congr`) and its
  four cases.

## Invariants the engine can assert

Quantities of `quantization/tubes.rs`: `Turn.shift` (the across component, an
`Expression` in the arc lengths), `Turn.around` (the length along, at the real
lengths); `shift_along` computes both components and keeps one. `ℓ` below is
the whole lengths of the integer search, `B` the balance matrix.

1. **Integer coefficients.** Every coefficient of a shift expression is a whole
   number (`Motion.composeAll_trans_add`): `c.fract() == 0`.
2. **Parity of a turn** (closed surfaces of genus zero only, any whole balanced
   `ℓ`, of any sign): with `along` the other component of `shift_along`,
   `(value(along, ℓ) + value(shift, ℓ)) % 2 == 0`. In particular a ring has an
   even number of quads round it, and every closed edge loop of the output mesh
   of a closed genus-zero surface has an even length. With borders or handles
   the sum has instead the parity `Ω_ℓ([γ̄])` of the note's Theorem "Parity of
   a turn on any surface" (on a surface of genus zero, the total length of the
   borders on one side of the turn); a ring of 23 quads round an eye socket of
   Suzanne is correct, not a bug. That theorem is proved on paper, not here.
3. **Handshake.** Each column of `B` has two entries `±1`, or a single one for
   an arc on a border of the surface (one half of the arc lies on a patch rim),
   or none. So for every whole `x` and every connected component `C` of the
   graph of the balances, `∑_{r ∈ C} (B x)_r` has the parity of the sum of `x`
   over the border arcs of `C`; on a closed surface `∑_r (B x)_r` is even, and a
   balance system with an odd total right-hand side has no whole solution.
   Conversely, when the graph of the balances is connected with an unbalanced
   cycle (as on the male body), one with an even total always has one
   (`solve_unicyclic`). An assertion that counts the halves of each arc on the
   patch rims, as the engine's does, is right with borders too. Only the
   closed case is proved here (`handshake`); the case with borders is the
   note's Lemma "Handshake", proved on paper.
4. **Burgers sum** (where the developed cone points are known, as in the
   research export): for a turn whose curve bounds a disc,
   `T = ∑ᵢ Qᵢ (I - Rᵢ) pᵢ` in the order of first visit, `Qᵢ = Rₘ⋯Rᵢ₊₁`,
   `Rᵢ = R^{4-k(cᵢ)}`, exactly in integers at whole lengths; the indices
   `4 - k` inside add up to 4; both components of `T` are congruent modulo 2 to
   the number of vertices of odd valence inside on points of odd colour.
5. **Cylinders.** Two disjoint parallel turns with no singular vertex between
   them have the same shift at every balanced `ℓ`.
6. **Staircases.** If `s₁ - s₂` equals the length of a staircase
   `∑ ℓ_{aᵢ}` and one of its arcs is held at length at least one, the two turns
   are never both closed: the projection should never hold both.
7. **Sign-definite turns.** A turn whose shift equals, modulo the balances, a
   form with non-negative coefficients and a positive one on an arc held at
   length at least one is never closed (`nonneg_form_pos`): never hold it.
8. **Forced helices.** For any `k` pairwise disjoint sets of held turns that are
   each infeasible, at least `k` held turns are left shifted at `ℓ`
   (`forced_helices`, `card_ge_of_disjoint`); along a chain of `m + 1` parallel
   turns separated by dislocations with `q` changes of sign, at least `m - q`
   are left shifted (`chain_of_dislocations`).
9. **Combinations** (closed surfaces of genus zero, or curves that all lie in
   one planar region without border). For whole multipliers,
   `∑ᵣ uᵣ sᵣ(ℓ) ≡ ∑ᵢ wᵢ jᵢ χᵢ (mod 2)`, `wᵢ` the winding numbers of the weighted
   cycle round the cone points, `jᵢ = 4 - k(cᵢ)`, `χᵢ` their colours
   (`checkerboard_combination`). With borders or handles the colours may depend
   on the paths; the general rule is `∑ᵣ uᵣ (αᵣ + sᵣ) ≡ ∑ᵣ uᵣ Ω_ℓ([γ̄ᵣ]) (mod 2)`
   (the note's Theorem "Parity of a turn on any surface").

## Files

* `Motion.lean`: motions, compositions, the Burgers sum, annuli, exchanges.
* `FirstVisit.lean`: the cut tree and the order of first visit.
* `QuarterTurn.lean`: quarter-turns of `ℂ`; dislocations, staircases.
* `Fox.lean`: words, holonomy, Fox derivatives, twisted windings, commutators.
* `Congruence.lean`: the Fox–Burgers formula modulo `(1 - z)²`; orders 2, 3, 4, 6.
* `Checkerboard.lean`: colours in `ℤ[i]`, the checkerboard rules, parity of a turn.
* `Balances.lean`: the handshake lemma.
* `Bidirected.lean`: trees, switchings and unbalanced cycles of a bidirected
  graph; Proposition "Free arcs".
* `Farkas.lean`: Farkas' lemma by Fourier–Motzkin elimination; Remarks
  "Sign-definite turns" and "Whole solutions exist at some size".
* `Completeness.lean`: Proposition "Completeness at given lengths".
* `GaussBonnet.lean`: discrete Gauss–Bonnet for quads, triangles and hexagons;
  Lemmas "Gauss–Bonnet for turns" and "One self-crossing" (the counts).
* `BeyondQuads.lean`: fields of any order: rotations and dislocations.
* `Forced.lean`: forced helices and chains of dislocations.
