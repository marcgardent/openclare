# Machine-checked cores of "Turns of a T-mesh: obstructions to closing edge loops"

This Lake project checks with Lean 4 and Mathlib the **algebraic cores** of the
results of the research note `../paper.tex` (Sections 3 to 8 and Appendix A;
Section 10 there lists what is checked, statement by statement), and of the
two propositions of its applied companion,
[`../../closing-loops/paper.tex`](../../closing-loops/paper.tex), "The shift of
a turn is linear" (also stated in `../paper.tex`) and "Completeness at given
lengths": 698 `theorem` and `lemma` declarations under this folder (`Check/Main.lean`
included), many of them small lemmas and a few
restating a hypothesis (`dislocation_closes_iff`, `staircase_rings`); the
count is not evidence, the table below names the main ones. The topological facts the note uses (that a loop develops into a given
motion, that a curve bounds a disc, ...) are not formalized: each one is an explicit hypothesis of the
theorem that uses it, never a hidden `sorry`. Every theorem depends only on
Lean's standard axioms (`propext`, `Classical.choice`, `Quot.sound`).

## Replay

```sh
make proofs NOTE=turns-of-a-t-mesh      # from the root of the repository
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

`Basic.lean` holds what the files share: `Motion` and its group law, and the
quarter-turn group, stated once in `ℤ[i]` (`gi`, `giUnit` of order four,
`IsQuarter a` for `a ∈ {1, R, R², R³}`). The files on whole lengths
(`Checkerboard.lean`) use it as it is. The files on real lengths
(`QuarterTurn.lean`, `FirstVisit.lean`, the twisted windings of `Fox.lean`)
keep `I`, `-I` in `ℂ`, the image of `gi` (`toComplex_gi`): real lengths need
`V = ℂ`, and taking the linear parts in `ℤ[i]` would mean a module structure of
`ℂ` over `ℤ[i]` and every statement of those files restated, for no gain in the
proofs. `BeyondQuads.lean` keeps the rotation `ζₙ` of any order `n`. All files
share the namespace `TurnsOfATMesh`.

`TMesh.lean` is the one object the argument runs on. A `TMesh` is
combinatorial: each patch has two pairs of opposite sides, a side is the list of
its arcs in order from its first corner (the two sides of a pair listed from
corresponding corners, in the same direction), and every arc lies on exactly two
sides with a sign `ε = ±1`, the gluing of a closed surface. The balance matrix
`B` and the shift of every turn are derived from it, not assumed: `B` is then
an instance of `Balances.lean` and `Bidirected.lean`, and the shifts and `B`
are the inputs of `Forced.lean` and `Farkas.lean` (`TMesh.not_closable_iff`,
`TMesh.forced_of_certificates`). The other files keep their own abstractions:
a turn as a list of motions (`Motion`, `Fox`, `Checkerboard`), a region as
finite sets with counts (`GaussBonnet`), the gluing data at whole lengths
(`Completeness`).

| Statement of the note | Lean | Assumed (hypotheses) |
|---|---|---|
| A T-mesh, its balances | `TMesh` (patches, sides as lists of arcs, gluing with signs); `TMesh.balance` (`B`), `balance_mulVec_eq_zero_iff` (`B ℓ = 0` ⇔ opposite sides of equal length), `balance_apply` (the column of an arc: `±1` in the balance of each of its two sides), `balance_column_even` (every column has an even sum), `balance_isBidirected` (`B` is bidirected) | a closed surface: every arc on exactly two sides, once on each, borders left out; for `balance_isBidirected`, no arc on two opposite sides of one patch (`NoArcAcross`, as when no patch borders itself) |
| Prop. "The shift of a turn is linear" | `TMesh.Turn.shift_linear` (the composite of the crossings of a turn at the lengths `ℓ` is the translation by `∑ₐ cₐ ℓₐ`, `c = shiftForm` integer and independent of `ℓ`), from `crossForm` (crossing an arc is `x ↦ ε x + c(ℓ)`, `c` an integer linear form), `evalMotion_composeAll` and `composite_lin` (the composite keeps the direction of the offsets); `Turn.mem_corridor_iff` (the corridor, the offsets from which the walk crosses the arcs of the turn, is cut out by strict linear inequalities with integer coefficients in `(x, ℓ)`, at `ℓ ≥ 0`), from `arcAt_eq_some` (the arcs of a side cover disjoint open intervals); `Turn.comes_back` (from an offset `o` of the corridor the walk comes back to the first side at `o + shift(ℓ)`), `comes_back_iff` (it closes after one round exactly when the shift is zero), `Turn.winds` (helices: `t` rounds while `o + j shift(ℓ)` stays in the corridor); the additivity alone for any motions: `Motion.composeAll_trans_add`, `Motion.composeAll_trans` | an oriented gluing: the frames of the patches are those of one orientation of the surface (`Oriented`, a condition on the signs of the arcs); that the walk from a half-integer is a dual line of the quad mesh is Prop. "Completeness" below, in its own model; the sentence on a corridor without half-integers is not formalized |
| The development of a turn in the plane, `T_γ(ℓ)` (DT-134, third brick) | `Holonomy.lean`: the frame of a side (`inward` the walking direction into the patch, `acrossDir` the direction of increasing offsets, `origin` its first corner), the change of chart across an arc as a motion of `ℤ[i]` (`crossLin`, the quarter-turn taking `inward τ` to `-inward τ'`; `crossTrans`, the crossing of the offsets read in `ℤ`; `crossMotion`), `crossMotions` along a path and `Turn.development`; `Development.lean`: `Turn.development_eq` (on an oriented T-mesh, at whole lengths, the development of a turn is the translation by `-α_γ(ℓ) inward + shift_γ(ℓ) acrossDir` in the frame of the first side), from `crossLin_acrossDir` (the geometric content of `Oriented`: the linear part takes `acrossDir τ` to `sign a · acrossDir τ'`), `crossMotion_side`, `crossTrans_eq`, and the invariant of the frame along a path (`composeAll_crossMotions_inward`, `composeAll_crossMotions_acrossDir`, `development_apply_origin`) | that the development is the inverse of the holonomy `ĥ(γ̄)` of the flat metric, and that this holonomy is the product of the rotations round the cone points: `Check/Main.lean` compares the development with `Fox.hol` of the word of the turn (`develop.py`) on every sphere, 3,865 turns of 34 spheres agree |
| Prop. "Completeness at given lengths", in a model only | `completeness_at_given_lengths` (on a closed surface the strips are in bijection with the cycles of the station walk, and two stations lie on one cycle exactly when the real walk from the half-integer of one reaches that of the other), from `stationWalk_injective` and `stationPerm` (the walk on unit edges of the gluing data is a permutation of the stations), `sameCycle_stationPerm_iff` (with `realWalk_stations`) and `stripsEquiv` (strips *defined* as the cycles of a successor map built from a permutation `walk` of the stations), `exists_station`, `sameCycle_station_iff`; `realWalk_iterate_half`, `realWalk_iterate_open` (the walk from `k + ½`, or from any point of `(k, k + 1)`, follows the unit edges); `glueLine_half`, `glueLine_open` (a gluing `t ↦ ±t + c` takes midpoints of unit edges to midpoints, open unit edges onto open unit edges) | the patches are integer rectangles glued along their arcs by `t ↦ ±t + c`, `c` whole; a closed surface (`ClosedGluing`): opposite sides of a patch have the same length, `opp` is one to one, crossing an arc takes a unit edge of a side to a unit edge of a side and no two onto the same one (`ClosedGluing.of_involutive`: both hold when the opposite of the opposite side is the side and crossing an arc back returns to the start); columns of positive depth (`hdepth`); that crossing a patch keeps the offset and that the darts and `next` are the quads and strips of the mesh |
| Lemma "Gauss–Bonnet for turns" | `gauss_bonnet_turn` (`∑ (4 - k) = 4 χ` inside a region whose border turns by zero), `gauss_bonnet`, `turn_border_sum`, `sum_deg`, `sum_corners` | that `γ̄`, on the grid refined by two, bounds a disc (or an annulus): a complex of quads with Euler characteristic `χ` whose interior vertices have a cycle as link and border vertices a path (`IsRegion`) |
| … the rotation part | `quarter_turns_prod_eq_one_iff` (`R^{j₁}⋯R^{jₘ} = I ⇔ 4 ∣ ∑ jᵢ`) | — |
| Lemma "One self-crossing" | `one_crossing_on_walk` (totals `3, 3, 2`), `one_crossing_on_segment` (totals `2, 2, 4`), from `gauss_bonnet_corners` (`4 χ - a + b` with `a` convex and `b` concave corners on the border) | the regions cut open at the crossing are discs, the crossing doubled on the border of the third; how many convex and concave corners each region has (`a - b = 1, 1, 2` on the walk, `2, 2, 0` on the closing segment), the note's case analysis |
| Prop. "The shift is set by the singularities" (a) | `CutDisc.lean`: a disc cut along a forest modelled by the walk round the border of the cut disc (`Cut`, first-child next-sibling, the matching of the two copies of each tree edge laminar by construction; `corners`, the face corners met at a vertex; `walk`, `border`; `jump`, the change of chart across a tree edge; `rootJumps`; `cones`), `Cut.jump_eq` (the recursion `G_f = G_{f_r} ∘ ⋯ ∘ G_{f_1} ∘ ρ_v`, `ρ_v` the rotation by `R^{4−k}` about the first-visit position), `CutDisc.holonomy_eq_glue`, `holonomy_eq_preorder` (the holonomy of the border is the composite of the cone rotations in the order of first visit), `holonomy_eq_burgers`, `holonomy_trans`; before it `Motion.composeAll_rotations`, `Motion.burgers_eq_sum`, `Motion.translation_inv`; a worked example (`threeSquares`) by `decide`; that the holonomy of the border of a round is the development of the turn is `borderHolonomy_round` (`Border.lean`, next row), that through the charts of a dual tree it is the composite of the jumps across the cuts is `borderHolonomy_cuts` (`Region.lean`), and that these jumps glue the cone rotations of a forest of cuts, so that the development of the turn is their composite in the order of first visit, is `development_eq_cones` (`CutForest.lean`), which closes the chain at whole lengths for the region on the left of a round (the rows after) | that the forest of cuts `Check/Main.lean` builds hangs from the region (`hangs`) and that its roots are the cut crossings of the border, decided on every region (1,852 regions of 35 spheres), not proved; the setting of `borderHolonomy_round` (oriented, whole balanced lengths, the nodes of the first side regular) |
| The chart continued along the border of a region (DT-134, fifth brick) | `Border.lean`: `fine`, `fineMotion` (a motion of the quads read in half quads), `crossEdge` (the change of chart across a fine edge: the identity inside a rectangle, the `crossMotion` of the arc holding it on a side), `aroundStep`, `valenceAround`, `holonomyAround` (the walk round a vertex, face to face), `normalize`, `turnAt`, `vertexCrossings`, `borderHolonomy` (the chart continued along a closed curve of steps through the region on its left), `acrossSteps`, `alongSteps`, `curveSteps`, `roundSteps`; `crossMotion_glued` (the crossing of an edge on a side takes its ends to the points they are glued to), `crossEdge_same`, `aroundStep_same`, `holonomyAround_eq` (the holonomy round a vertex met by `k` faces is `R^{4−k}` about it), `roundSteps_round`, `borderHolonomy_round` (on an oriented T-mesh at whole balanced lengths, every side of positive length, the holonomy of the border of a round is the composite of the `crossMotions` of the arcs it crosses, `Turn.development` of its turn), from `aroundStep_reverse` (a step round a vertex walked back crosses back) and `vc_node`; `Motion.composeAll_telescope` | that every node strictly inside the first side is met by four faces, used only when the closing segment has the region on its left outside the rectangle; `Check/Main.lean` evaluates the definitions on every round and every vertex: on seed 1, 19,584 of 19,584 rounds (17,296 rings, 902 closing segments inside, 1,386 outside, none outside with an irregular node) and 21,593 of 21,593 vertices of the quad graph |
| The jumps across the cuts of a region (DT-134, sixth brick) | `Region.lean`: `faceBeside`, `faceAt` (the face on the left of a step), `Crossing` (a face, a change of chart, a face), `turnAtF`, `vertexCrossingsF`, `borderCrossings` (the crossings of `borderHolonomy` with the faces they leave and enter), `qinv`, `jump` (`M g ∘ C ∘ (M f)⁻¹`, a crossing seen through the charts `M`), `cutCrossings`; `borderCrossings_holonomy` (the holonomy of the border is the composite of its crossings), `borderCrossings_chain` (round a closed curve of unit steps the faces chain: `faceAt s' :: cs.map (·.2.2) = cs.map (·.1) ++ [faceAt s']`, `s'` the first step read in its own frame), `qinv_comp`, `comp_qinv`, `jump_tree` (the jump is the identity where the charts agree), `borderHolonomy_jumps` (with charts of quarter-turn linear parts, the identity on the first face, the holonomy of the border is the composite of the jumps of its crossings, in border order), `borderHolonomy_cuts` (of the jumps across the cuts alone, when the charts agree across the crossings of the tree) | that the steps are unit steps (`borderCrossings_chain` needs it where `normalize` reads a step across a side) and that the charts agree across the tree, which `discOf` builds them to do; `Check/Main.lean` evaluates the definitions on both regions of every simple curve on the spheres of seed 1: 1,852 of 1,852 regions crossed face by face with the faces chained, 124,632 crossings (111,168 across the tree, every one with agreeing charts; 13,464 across the cuts), and in 1,852 of 1,852 regions the composite of the jumps across the cuts equals the holonomy of the border and the development (`2 T` on the left, `−2 T` on the right). The border turns round its own vertices only: in every region the cuts crossed are those with an end on the curve, each crossed once (13,464 of the 1,889,092 cuts of the 1,852 regions) |
| The forest of cuts of a region (DT-134, seventh brick) | `CutForest.lean`: `aroundF`, `aroundVertexF` (the walk round a vertex with the faces it leaves and enters), `CutForest` (first-child next-sibling: a cut by its crossing, its far end with the direction to its near end, the valence `k` of the far end, the cuts below it, the next cuts), `roots`, `cones` (`R^{4−k}` about the far end developed in the chart of the face before the cut), `size`, `hangs` (at each node: a unit direction, the face before the cut on the left of `(v, −i d)`, the walk clockwise round the far end back in `k` crossings, the last the cut crossed back by the inverse change of chart, the cut crossings before it the roots below, the charts agreeing across the tree crossings); `aroundF_around`, `aroundVertexF_chain`, `holonomyAround_cw` (clockwise round `v` in `k` crossings: `R^{k}` about `v`), `rotAbout_conj`, `jump_reverse`, `jump_node` (`G_e = G_{e_r} ∘ ⋯ ∘ G_{e_1} ∘ ρ_v` on the grid), `roots_glue` (the jumps across the roots compose to `Forest.glue` of the cone rotations), `borderHolonomy_forest` (the holonomy of the border is their composite in preorder), `development_eq_cones` (so is the development of the turn of a round, in half quads) | that the forest hangs and that its roots are the cut crossings of the border, both decided by `Check/Main.lean`: it builds the forest of both regions of every simple curve on the spheres of seed 1 from the cut crossings and the library's walks round the far ends; in 1,852 of 1,852 regions the forest hangs, its cone points in preorder (`k ≠ 4`) are the word of the walk round the cut trees, its nodes are the interior vertices, and the composite of its cone rotations equals the development (`2 T` on the left, `−2 T` on the right); 1,889,092 nodes, 17,289 with `k ≠ 4`, forests at most 129 deep, no region with an empty forest |
| … the order of first visit | `Forest.glue_eq_preorder`, `Forest.glue_rotations`; `Forest.order_of_first_visit_matters` (last visit gives another vector); all in `FirstVisit.lean`, namespace `Forest` | the recursion `G_f = G_{f_r} ∘ ⋯ ∘ G_{f_1} ∘ ρ_v` read off the cut tree |
| Prop. (b), annuli | `Motion.annulus_holonomy` (`ĥ(γ̄₁ κ γ̄₂⁻¹ κ⁻¹)` is the translation by `T₁ - M T₂`), `Motion.conj_translation` | the homomorphism `ĥ` and the holonomies of the two turns |
| Prop. "A dislocation between two turns" (i), (ii) | `dislocation_burgers` (the four orders and orientations), `dislocation` (`±d₁ ± d₂`, `‖B‖² = 2‖d‖²`), `dislocation_closes_iff`, `dislocation_straight` | as in (b) |
| Prop. "Staircases" | `staircase`, `staircase_rings`, `nonneg_form_pos` | `d = ∑ ℓᵢ vᵢ` along the staircase (development along homotopic paths) |
| Prop. "Exchanging two vertices" | `Motion.exchange` (any rotation), `exchange_quarter` | the homotopy `λ ≃ λ_c λ_{c'}⁻¹` |
| Prop. "A separatrix that winds", the conjugation step | `Motion.conj_translation` | the free homotopy |
| Parity of `(I - R^{±1}) p`, `(I - R²) p` | `parity_one_sub_quarter`, `one_sub_half_turn`, `quarter_mul_parity`, `colour_rotAbout` | — |
| Prop. "Checkerboard rule" | `checkerboard`, `oddColours_translate` | the Burgers sum of Prop. (b) |
| Prop. "Parity of a turn on a sphere" | `TMesh.parity_of_turn_of_colouring` on the T-mesh (`Bipartite.lean`, its own row below); before it, `parity_of_turn_any_sign` (whole balanced lengths of any sign), from `parity_chain`, `parity_of_turn` (the induction of the note's proof at positive lengths), `even_of_even_on_positive` (`ℓ = (ℓ + N p) - N p`) and `exists_positive_whole` (positive whole balanced lengths from real ones, by `Farkas.whole_solution_of_real`); `colour_hol_trans` (a second proof) | at every positive whole balanced `ℓ`, a proper 2-colouring of the quad graph with the development of the turn (`parity_of_turn`'s hypotheses); that the length and the shift of the turn are whole linear forms in `ℓ` (the turn crosses the same patches); real positive balanced lengths; for the second proof, that the turn is a word in the loops round the cone points (closed genus-zero surface) |
| Prop. "Fox–Burgers formula" | `fox_burgers`, `fox_fundamental`, `fox_burgers_combination`, `twisted_relation`, `combination_translate`, `insert_relation`; on the free group: `holFG`, `foxFG` | the presentation of `π₁(Σ°)` by the loops `λᵢ` |
| … inserting the relation leaves the sum unchanged | `fox_sum_insert_relation`, `combination_insert_relation` (the `Wᵢ` change, `∑ᵢ ⟨Wᵢ, (I - Rᵢ) pᵢ⟩` does not), `hol_insert_relation` | as above |
| … for a simple turn it is Prop. "The shift is set by the singularities" (a) | `hol_eq_composeAll` (`Fox.hol` applies the head of the word last, `composeAll` first: the list is reversed), `hol_trans_eq_burgers`, `fox_burgers_eq_burgers` (any word); on `simpleWord l = xₘ ⋯ x₁`: `hol_simpleWord`, `fox_burgers_simple` (the Fox–Burgers sum is `∑ₖ Qₖ (pₖ - Rₖ pₖ)`, `Motion.burgers_map`), `fox_simpleWord` (`ρ(∂w/∂x_{iₖ}) = Qₖ = Rₘ⋯Rₖ₊₁` when the indices are distinct) | as above; the letters are the rotations about the developed cone points |
| Corollary "Checkerboard rule for any combination" | `checkerboard_word`, `checkerboard_combination`, `colours_flip`, `relation_colours`, `winding_constant` | as above; the relation `ϱ = λ₁⋯λₙ` has trivial holonomy |
| Prop. "Winding numbers do not suffice" | `commutator_holonomy`, `winding_numbers_do_not_suffice` | — |
| Lemma "Handshake" | `handshake`, `column_even`, `no_whole_solution`; on a T-mesh, `TMesh.handshake` | on a T-mesh, a closed surface |
| Prop. "Free arcs" | on a T-mesh, `TMesh.free_arcs`; for any bidirected `B`: `free_arcs` (unique whole extension), `free_arcs_basis` (`z_f` with entries in `{0, ±1, ±2}`), `rows_independent` (full row rank), `solve_unicyclic` (the whole image of `B` on `T ∪ {f}` is the lattice of even sums), `exists_closesUnbalanced` (the edge closing an unbalanced cycle exists), `free_arcs_of_cycle` (as stated: a spanning tree and an unbalanced cycle) | that the graph is connected, as the existence of a spanning tree grown by leaves (`IsTree`); on a T-mesh, `NoArcAcross` |
| … bidirected graphs | `IsTree.exists_switching`, `IsTree.eq_zero`, `IsTree.solve`, `closesUnbalanced_of_pair`, `pair_ne_zero`, `balanced_cycle_of_not_closesUnbalanced`, `Cycle.isBalanced_of_graph` (a cycle of a balanced graph is balanced) | — |
| Remark "Sign-definite turns", Farkas certificates | on a T-mesh, `TMesh.not_closable_iff` (turns of the T-mesh that cannot close together within the balanced `ℓ ≥ b` ⇔ a certificate on its `B` and the matrix of their shift forms), from `TMesh.closable_iff` (closing is a linear system, the shifts being linear forms); for any matrices: `sign_definite_certificate` (`B ℓ = 0`, `shift_γ(ℓ) = 0`, `ℓ ≥ b` infeasible ⇔ `∑ u_γ shift_γ + λᵀ B = c ≥ 0` with `c ⬝ b > 0`), `forced_zero_iff` (arcs zero in every solution), `farkas_int` (whole multipliers), `farkas`, `farkas_bounds` (any linearly ordered field, by Fourier–Motzkin) | — |
| Remark "Whole solutions exist at some size" | `whole_solution_of_real`, `clear_denominators` | — |
| Theorem "Forced shifts" | `forced_helices`, `meets_infeasible`, `card_ge_of_disjoint`, `tauP_ge_of_disjoint` (any functions `shift γ`); on a T-mesh, `TMesh.forced_of_certificates` (`k` disjoint sets of its turns, each with a Farkas certificate: every balanced `ℓ ≥ b` leaves at least `k` of them shifted) | — |
| Corollary "Chains of dislocations" | `sign_change_between`, `chain_of_dislocations` | the identities `s_{i-1} - s_i = εᵢ Fᵢ` with `Fᵢ > 0` |
| Corollary "Parity" (a)–(c) | `ring_even`, `odd_pitch_odd_length`, `repair_keeps_length_cannot_close`, `odd_change_of_pitch_changes_length`, from `parity_mod` (`Consequences.lean`) | `α ≡ s (mod 2)`, the conclusion of `parity_of_turn_any_sign` |
| Prop. "Whole lengths and size without a cap" | `closable_at_any_size`, `smul_mem_system` (scaling a real solution of a homogeneous system with bounds `ℓ ≥ b ≥ 0` by `√(q / Q(ℓ))`); the whole case is `whole_solution_of_real` | the number of quads `Q` homogeneous of degree two; the infimum over `P_Y` taken on paper |
| Lemma "Combs", the identity | `comb_reading`, `dotC_sum` | the feet exist; every step of the comb but the arc is orthogonal to `e_a` (the perpendicular traces develop straight) |
| Prop. "Burgers sum with borders and handles" | `fox_general` (`τ(w) = ∑ᵢ ρ(∂w/∂xᵢ) τ(xᵢ)` for letters that are any motions, `holGen`, `letterMotion`), `holGen_rotations` (rotations about the cone points give back `hol`), `commutator_motion` (`(1 - B) a - (1 - A) b` for commuting linear parts) (`Periods.lean`) | the presentation of `π₁(Σ°)` by the loops round the cone points, the border components and the handles |
| Prop. "Two crossing turns on a torus" | `commutator_translation` (`(1 - R^j) T`), `commutator_translations` (two translations commute, the Burgers sum of the cut disc is zero) | the border of the disc cut along the two curves is their commutator |
| Prop. "Collars" | `collar`, `quarter_mul_quarter` (a translation along an axis conjugated by a quarter-turn motion is a translation along an axis; `α > 0` picks `e₁`) | the holonomy of the turn is conjugate to that of the border component (an annulus without cone point) |
| The quad mesh at whole lengths, on the `TMesh` (DT-134, first brick) | `Grid.lean`: `Point`, `Face`, `sideLen`, `width`, `height`, `posNat`, `sidePoint`, `glued` (the gluing of an arc, point by point), `gluings`, `Same` and `Vertex` (the vertices as the classes the gluings generate), `points`, `faces`, `corners`, `faceEdges`, `across` (the dual line from a station), `alongSide`, `curve` and `round` (one round of the walk at the whole lengths as a closed curve of the fine grid), `Turn.lengthForm` and `Turn.length` (the length of a turn, the length across each patch crossed), `Point.IsCoarse`, `pointColour`, `IsColouring` (decidable), `Bipartite`, `stationVertex`; `same_of_glued` | definitions, computable, that `Check/Main.lean` evaluates on every export: the vertices by union-find, the edges as classes of midpoints, the Euler characteristic, the valences, the regions of every simple round with Lemma "Gauss–Bonnet for turns" on each, the constants of the colouring by propagation and `IsColouring` decided; the holonomy of a curve is not yet related to `Turn.shift` |
| Prop. "Parity of a turn on a sphere", on the `TMesh` (DT-173) | `Bipartite.lean`: `parity_of_turn_of_colouring` (constants `κ` colouring the quad graph at whole `ℓ`, `IsColouring`, give `α ≡ shift (mod 2)` for every turn followed by the walk from a half-integer station: the vertex beside the station is followed round the walk, `colour_opp` across a patch, `colour_cross` across an arc, `colour_along`, and `negCount_turn`, the arcs of sign `-1` even in number on an oriented T-mesh), `parity_of_turn_tmesh_any_sign` (whole balanced lengths of any sign) | that some half-integer station follows the turn at `ℓ`; the colouring is given by `SphereGrid.lean` on a sphere (the rows below), and `Check/Main.lean` still decides `IsColouring` on every export (the 35 spheres are, 7 of 15 tori) |
| A closed quad surface with χ = 2 is bipartite (DT-173) | `SphereBipartite.lean`: `QuadSurface` (finite vertices, edges and faces, each edge in exactly two slots of faces, vertices and faces connected), `bipartite_of_euler` (with `V − E + F = 2` a colouring that changes along every edge), from `bd1_comp_bd2` (`∂₁ ∘ ∂₂ = 0` over `ZMod 2`), `card_faces_le` (a 2-chain without boundary is constant), `card_vertices_le`, `range_bd2_eq_ker_bd1` (every cycle bounds, by rank–nullity), `total_eq_zero_of_bd1` (every cycle is even) | nothing topological: linear algebra over `ZMod 2` on the chains |
| The quad mesh of a `TMesh` is such a surface (DT-173) | `Coarse.lean` (the vertices, edges and faces of the quad mesh read off the fine grid by parity: `Point.IsCoarse`, `Point.IsEdge`, `Point.IsCentre`, `edgeEnds`, `centreCorner`, `centreEdge`, `eulerQuad`); `CoarseEuler.lean`: `eulerQuad_eq_nodeCount` (`V − E + F = P − A + N`, by `ncard_classes` on each parity); `CoarseSurface.lean`: `quadSurface` (the `QuadSurface` of the quad mesh: `quadEnds` independent of the midpoint chosen, `quad_two_slots`, `quad_connected` and `quad_faceConnected` when the T-mesh is `Connected`), `card_quadSurface`, `quad_bipartite` | positive balanced whole lengths, every side holding an arc, the T-mesh connected |
| Prop. "Parity of a turn on a sphere", complete (DT-173) | `SphereGrid.lean`: `bipartite_of_sphere` (a connected T-mesh with `P − A + N = 2` has `Bipartite` at every positive balanced whole `ℓ`: the colouring of the classes read at the corner of each rectangle is `κ_P + i + j`), `parity_of_turn_sphere_at` (at one such `ℓ`, for a turn followed from a station), `parity_of_turn_sphere` (whole balanced lengths of any sign) | the sphere as `P − A + N = 2` and connected, oriented; the turn followed by the walk from some station at every positive balanced whole `ℓ`; real positive balanced lengths |
| The Euler characteristic of the quad mesh (DT-134) | `GridEuler.lean`: `euler_eq_nodeCount` (`V − E + F = P − A + N` at positive balanced whole lengths, every side holding an arc), `nodeCount` (the nodes of the T-mesh without lengths: the ends of the arcs as junctions, identified by the gluing of the arcs and at the corners, `NodeStep`), `numNodes_eq`, `same_of_inner` (a point inside a patch is a vertex alone), `same_of_arcInner` (a point inside an arc is one vertex with its partner, and nothing else), `isEnd_of_same`, `ncard_classes`; the edges counted as classes of midpoints in the grid at `2ℓ`, since two edges may join one pair of vertices (the square torus of one patch at `ℓ = 1`: 8 edges on 4 pairs, which the pair count of the programs missed until this theorem) | that the union-find of `Check/Main.lean` computes these quantities |
| Theorem "Parity of a turn on any surface" | `parity_any_surface`, `colour_comp_trans` (`π(t) = t₁ + t₂ mod 2` is a homomorphism on the motions with quarter-turn linear parts), `colour_composeAll_trans`, `colour_rotAbout_trans` (it vanishes on the loops round the cone points), `colour_border` (it is `L mod 2` on a border component), `colour_turn` (`α + s` on a turn) (`AnySurface.lean`) | that `π ∘ ĥ` then factors through `H₁(Σ; ℤ/2)`, and that its value on a class is the parity of the length of any cycle of the quad graph in it |
| Theorem "Choosing the turns is NP-hard", the reduction | `Gadget.closable_iff_independent` (on the gadget of a graph without loops, `Closable` of `Forced.lean` ⇔ independent set), `Gadget.tauP_eq_vertex_cover` (`τ_P(V)` is the vertex cover number), from `independent_of_closable`, `witness_closes` (the lengths `0` and `1`) (`Gadget.lean`) | NP-hardness of Independent Set and Vertex Cover and the bounds on approximation are cited; that the gadget is a T-mesh is not claimed |

Not formalized: the topology behind the hypotheses, weeks of work with much of
it missing from Mathlib. That a simple closed curve on a sphere bounds a disc on
each side (Jordan–Schoenflies) and that the refined quads inside form a disc in
the combinatorial sense (`IsRegion`); the developing map, holonomy as a
homomorphism of the fundamental group and its presentation by the loops round
the cone points; which corners of the regions of Lemma "One self-crossing" are
convex (the totals are proved from them). Two standard graph facts, days of work: a finite graph is connected
exactly when it has a spanning tree grown by leaves, and Harary's converse (a
graph all of whose cycles are balanced has a switching; only the direction the
proposition needs is proved). For the T-mesh of `TMesh.lean`: that it comes from
a surface with a cross field (it is taken as combinatorial data), its
orientation (`Oriented` is a hypothesis on the signs), borders, the length
`α_γ` of a turn, the 2-dimensional crossings `x ↦ R^k x + t` (only their
component across the walk is modelled), and the holonomy `hol` of `Fox.lean`,
to which the note ties the shift by `shift_γ(ℓ) = -⟨e₂, ĥ(γ̄)⟩` (Section "Flat
metric and holonomy"). Strips on surfaces with borders (the walk is then
a partial bijection, whose open paths `stripsEquiv` does not cover), and a model
of the quad mesh from which the stations, the depths of the columns and the
gluing data of `completeness_at_given_lengths` would be read rather than
assumed. And every
measured statement.

## Gaps and corrections found in the note

1. **Fox calculus, inserting the relation.** Inserting a conjugate `g ϱ g⁻¹` of
   the relation in `w_r` *after a prefix `a`* adds `ρ(a g ∂ϱ/∂xᵢ)ᵀ f_r` to `Wᵢ`,
   not `ρ(g ∂ϱ/∂xᵢ)ᵀ f_r`; the note's formula is the case of an insertion at
   the front (`insert_relation`). The conclusion (the sum does not change) is
   right (`fox_sum_insert_relation`, `combination_insert_relation`).
2. **Theorem "Forced shifts" needs neither convexity nor whole lengths.** The
   proof holds for any set `P` of lengths and any `ℓ ∈ P`
   (`forced_helices`); convexity only makes `τ_P` computable by linear
   programming. So the theorem applies verbatim to the non-convex set of
   lengths with a given number of quads, an open problem of the note: what is
   open there is computing or certifying `τ_P`, not the
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
   stations is a permutation (the gluing maps unit edges one to one:
   `stationWalk_injective`, `stationPerm`), so every
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
  (`Basic.lean`, `Motion.lean`, `FirstVisit.lean`, `Fox.lean`), so they hold
  for the rotations of order 6 of triangular and hexagonal meshes, for cone
  metrics of any angles, and for motions of space.
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
   number (`TMesh.Turn.shift_linear`): `c.fract() == 0`.
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

## Checked against the engine's exports

A definition that does not describe the engine's object would make the theorems
true but beside the point. `Check/` tests `TMesh.lean` on real T-meshes; it is
not part of the library, which `lake build` builds unchanged.

```sh
make lean-check EXPORT=<dir>      # from the root of the repository
```

`<dir>` is a directory the engine wrote with `OPENCLARE_EXPORT_TURNS=<dir>`.
`Check/convert.py` turns each way of closing (`way-<n>.json`) into the model
(its docstring gives the mapping, read from `quantization/tubes.rs` and
`export.rs` and checked on the data: rim side 0 is `(P, 0, 0)`, side 1 is
`(P, 1, 1)`, sides 2 and 3 are `(P, 0, 1)` and `(P, 1, 0)` listed in reverse);
`Check/Main.lean`, run by `lake env lean --run`, builds the `TMesh` from it,
discharging the proof fields by checks on the data, and compares with the
library's definitions, in exact arithmetic:

* the fields of `TMesh`, `NoArcAcross` and `Oriented`, every violation listed;
* `balance` with the engine's balances, row by row (the rows `(P, 1)` are the
  engine's negated);
* `Turn.shiftForm` with the engine's shift, coefficient by coefficient and
  modulo the balances (`shiftForm − shift = λᵀ B`, `λ` found by the program and
  the identity checked against `balance`);
* at the lengths the engine walked: its offset lies in `Turn.corridor`, `walk`
  passes within `10⁻⁶` of each crossing the engine recorded and comes back at
  the offset plus `Turn.shift`; at the real lengths the same from the same
  share of the side, and at the whole lengths from every half-integer of the
  corridor.

Surfaces with borders are out of the model's scope and reported as skipped.
Results on 2026-10-25 (an i7-7700K, 4 cores, shared with other jobs, load
about 6): the male body at 10,590 quads exported before commit `c8bf2be` (the
data of the note, 8 ways), and exports from commit `300a058` of the male body
(10,580 quads, 8 ways) and of blob, spot, rock and torus at 3,000 quads; 35
ways, 3,841 turns:

| Check | Agree |
|---|---|
| fields of `TMesh`, `NoArcAcross`, `Oriented` | all 35, no violation |
| balances | 30,970 of 30,970 rows |
| turns of the `TMesh` (`IsPath`, `closes`) | 3,841 of 3,841 |
| `shiftForm` = engine shift, coefficient by coefficient | **0** of 3,841 |
| `shiftForm` = engine shift modulo the balances, `λ` whole | 3,841 of 3,841 |
| walked lengths: offset in the corridor, crossings, come-back | 3,841 of 3,841 each |
| real lengths: come-back (2,680 turns keep the share in their corridor) | 2,680 of 2,680 |
| whole lengths: come-back from each half-integer of the corridor (1,333 turns have one) | 12,873 of 12,873 |

Suzanne, which has borders, was skipped. The run takes 20 minutes of processor
time, 8 minutes with 3 jobs.

Two findings. **The engine's shift is another representative of the same
form.** The engine measures positions round the rim from the first corner
(`tubes::rims`), so a position on sides 2 and 3 carries the lengths of sides 0
and 1; the library measures each side from its own first corner. The two forms
differ by a whole combination of the balances, so they agree at every
balanced length (the whole lengths; the walked lengths, balanced to `10⁻⁷`,
to within `10⁻⁶`), never as vectors. The shift of a turn is defined modulo the
balances, and what the library proves from it (Remark "Sign-definite turns",
Theorem "Forced shifts") is stated modulo the balances. **The real shift depends on the representative.**
The real lengths the engine ranks turns by are its exact lengths with a jitter
of ±0.5 % per arc (`quantization/mod.rs`), which leaves the balances off by up
to 0.39; there the two representatives differ by up to 6.6 on the male body
of the note and 1.2 on the fresh export, and by more than 0.05 (the real shift
above which the engine holds a turn) for 2,572 of the 3,841 turns.

**Forced shifts and the Fox–Burgers formula.** Two more checks run on the same
exports.

* *Certificates* (Theorem "Forced shifts", Remark "Sign-definite turns").
  `Check/certify.py` (SciPy, run by hand) finds, for each way, the cores of held
  turns that cannot be closed within `P = {B ℓ = 0, ℓ ≥ b}` with a Farkas
  certificate `(u, c)` each, the disjoint ones, a least set `X` meeting them and
  whole lengths closing the other held turns, and, from the claims of a paper,
  the certificates of its table with lengths closing each set without each of
  its turns; it writes them to `Check/certificates/` (see the README there).
  Its numbers are floats read as fractions, or, when that fails (large cores,
  vertices with large denominators), solved again exactly over the fractions.
  `Main.lean` trusts none of it: it solves `λ` from the claimed `c`, checks
  `Σ u shiftForm + λᵀ B = c ≥ 0` and `c · b > 0` with `balance` and
  `shiftForm` (the right side of `TMesh.not_closable_iff`), then bounds
  `τ_P(H)` below by the disjoint certified cores (`forced_of_certificates`) and
  by the least set meeting the certified cores (`meets_infeasible`, a branch
  and bound stopped after 100,000 nodes), above by the closing lengths, and
  compares with the papers.
* *Fox–Burgers* (genus zero). `Check/develop.py` cuts the surface along the
  arcs off a breadth-first tree of the patches (a spanning tree of the nodes on
  a sphere), develops it at the whole lengths, and writes each jump across the
  cut as a word in the loops round the cone points by the recursion of
  `FirstVisit.lean`, keeping a convention only if every node's rotation is
  exactly `R^{4-k}` about one of its developed corners. `Main.lean` evaluates
  `Fox.hol` over `ℤ[i]` (`giUnit` powers): each jump must be the change of
  chart, each turn a translation by `α e₁ - shift e₂` (`e₁` the walking
  direction, `e₂` that of increasing offsets, turned by the chart; `α` the
  length of the turn, computed on the `TMesh`), and equal to the sum of
  Proposition "Fox–Burgers formula" (`fox`). The convention found everywhere so
  far: `G_f` is the jump from the patch holding the half of `f` towards the
  root, children taken in the reverse of the order the node lists its halves,
  the rotation applied first. This is the sign of the paper's Section "Flat
  metric and holonomy" (`ĥ(γ̄)` the translation by `α e₁ - shift e₂`); the
  along component `-α` written in issue #141 is not what the data give.

Results on 2026-10-26, all checks above, on the male body of the papers
(exported before commit `c8bf2be`, 10,590 quads) and on exports from commit
`524b759` (the engine with DT-162) of the male body (10,520 quads) and of blob,
spot, rock and torus at about 3,000 quads. Every check passes on every way:

| Export | Ways | Turns | Balance rows | Held | Certificates valid | `τ_P(H)` per way | Fox jumps, turns |
|---|---|---|---|---|---|---|---|
| male body, papers | 8 | 1,299 | 12,416 | 765 | 51 of 51 | 1, 5, 1, 4, 1, 6, 1, 9 | 10,352, 1,299 |
| male body, `524b759` | 8 | 1,708 | 12,896 | 1,020 | 1,636 of 1,636 | 1, 5, 3, 12, 5, 20–30, 8, 28–37 | 10,728, 1,708 |
| blob, `524b759` | 6 | 136 | 792 | 70 | 8 of 8 | 0, 0, 1, 0, 1, 2 | 666, 136 |
| spot, `524b759` | 6 | 484 | 2,628 | 249 | 31 of 31 | 0, 0, 1, 2, 6, 7 | 2,190, 484 |
| rock, `524b759` | 6 | 226 | 2,232 | 169 | 78 of 78 | 0, 0, 0, 0, 4, 4 | 1,866, 226 |
| torus, `524b759` | T-mesh only | 0 | 6 | 0 | | | not a sphere |

A single number is exact (lower bound = upper bound). Blob, spot and rock
export ways 0, 1, 3, 4, 5, 7: ways 2 and 6 hold the same turns as another and
are not run. The torus holds no turn at `524b759` (its four turns close at the
lengths of the parametrization, DT-162), so it has no way; its T-mesh is
checked. In ways 5 and 7 of the fresh male body, of 734 and 845 cores, the
branch and bound stops before the least set meeting the cores (30 and 37 by
HiGHS in `certify.py`, not checked): the bounds are the disjoint cores and the
closing lengths. On all 3,853 turns the checks of the first table agree as on
2026-10-25: the shift forms modulo the balances only, the walked lengths
everywhere, the real lengths on the 2,692 turns that keep their share, the whole
lengths on 13,261 half-integers; the real shift differs between the two
representatives by up to 6.6 on the male body of the papers and 1.2 on the
fresh one. Suzanne at `524b759` (10 T-meshes, all with borders) is skipped.

Against the papers, on their male body (the export of v0.7.0): the held turns
(58, 92, 64, 102, 125, 219, 131, 229) agree, and `τ_P(H)` = 1, 5, 3, 12, 5 and
8 in ways 0 to 4 and 6 is checked exactly (in way 3, 10 of the 21 cores are
disjoint and the least set meeting them is 12); in ways 5 and 7 the paper's 30
and 37 lie within the bounds checked, 20 to 30 and 28 to 37 (disjoint certified
cores below, whole lengths closing all but that many above; the least set
meeting the cores is not searched past 100,000 nodes). Table `tab:iis`: its 41
rows give 68 certificates over the ways, all valid with the paper's
multipliers and bounded arcs, and every set closes, at exact whole lengths,
without each of its turns (230 of 230). The Fox–Burgers formula holds on all
1,708 turns of the eight ways.

Each check fails on corrupted data (way 1 of the papers' body: a multiplier of a
core or of a set of the table, a closing length, a developed place, each
changed by one).

Runtimes, on the i7-7700K (4 cores, 8 threads, 62 GB) shared with a virtual
machine of 6 cores and desktop use, load 4 to 9. `certify.py`: 50 minutes of
processor time on the male body of the papers (1 to 21 minutes a way), 22
minutes with four ways at a time; on the fresh exports 3 minutes for blob,
spot and rock, 67 minutes for the male body, of which 27 and 29 in ways 5 and
7. `make lean-check`: 6 minutes with 3 jobs on the male body of the papers (13
minutes of processor time), 13 minutes on the fresh one (28 minutes, 9 each
for the 734 and 845 certificates of ways 5 and 7), 2 minutes on blob, spot, rock and torus.

**When it runs.**
- **CI**, on every change of this folder, runs the check on two small exports kept in
  `Check/fixtures` (torus and blob, frozen at commit `300a058`): a change of the
  library that no longer describes the engine's T-mesh fails there.
- **By hand**, `make lean-check EXPORT=<dir>`:
  - on fresh exports, the male body included, whenever the engine's T-mesh or
    quantization changes;
  - at each rerun of the papers on their frozen commit, whose exports and log go to
    the papers' data package (DT-088) and whose counts replace the table above,
    with the commit.

Exports and logs are not kept in git.

## Files

* `Basic.lean`: motions and their group law; the quarter-turn group in `ℤ[i]`.
* `Motion.lean`: compositions of motions, the Burgers sum, annuli, exchanges.
* `FirstVisit.lean`: the cut tree and the order of first visit.
* `QuarterTurn.lean`: quarter-turns of `ℂ` (the group of `Basic.lean` read in
  `ℂ`); dislocations, staircases.
* `Fox.lean`: words, holonomy, Fox derivatives, twisted windings, commutators;
  the Fox–Burgers sum as the Burgers sum of `Motion.lean`.
* `Congruence.lean`: the Fox–Burgers formula modulo `(1 - z)²`; orders 2, 3, 4, 6.
* `Checkerboard.lean`: colours in `ℤ[i]`, the checkerboard rules, parity of a turn
  at lengths of any sign.
* `Balances.lean`: the handshake lemma.
* `Bidirected.lean`: trees, switchings and unbalanced cycles of a bidirected
  graph; Proposition "Free arcs".
* `Farkas.lean`: Farkas' lemma by Fourier–Motzkin elimination; Remarks
  "Sign-definite turns" and "Whole solutions exist at some size".
* `Completeness.lean`: Proposition "Completeness at given lengths" (of the
  companion note, `../../closing-loops`).
* `GaussBonnet.lean`: discrete Gauss–Bonnet for quads, triangles and hexagons;
  Lemmas "Gauss–Bonnet for turns" and "One self-crossing" (the counts).
* `BeyondQuads.lean`: fields of any order: rotations and dislocations.
* `Forced.lean`: forced helices and chains of dislocations.
* `TMesh.lean`: the combinatorial T-mesh, its balance matrix, turns, their
  shifts and corridors; Lemma "Handshake", Proposition "Free arcs", Remark
  "Sign-definite turns" and Theorem "Forced shifts" on it; an example, a torus
  of one patch glued with a twist (`twistedTorus`).
* `Check/`, not in the library: `convert.py`, `Main.lean` and
  `check_exports.py`, the check against the engine's exports above.
