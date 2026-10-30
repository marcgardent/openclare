/-
The seventh brick of DT-134: the jumps across the root cuts of a region are the
gluing motions of a forest of cuts, so the holonomy of the border is the
composite of the rotations round the cone points in the order of first visit.

`Region.lean` reads the holonomy of the border of a region through the charts
`M` of a dual spanning tree as the composite, in border order, of the jumps
`M g ∘ C ∘ (M f)⁻¹` across the cuts the border crosses: the cuts with an end on
the curve, the roots of the forest of cuts. Here the forest is written down
(`CutForest`, first-child next-sibling as `Forest` and `Cut`): each node is a
cut, by its crossing `c` from the face `f` before it to the face `g` after it,
clockwise round its near end; its far end `v` with the direction `d` from `v`
towards the near end, in the frame of `f` (`s = (v, d)`, so that `f` is the face
on the left of `(v, −i d)`); the number `k` of faces round `v`; and the cuts
leaving `v`, in the order the walk clockwise round `v` from the cut meets them.
The forest hangs from the region (`hangs`) when, at each node, `d` is a unit
step and the walk clockwise round `v` from the edge a quarter-turn clockwise
from the cut comes back in `k` crossings, the last of them the cut crossed back
from `g` to `f` by the inverse change of chart, the cuts among the others being
the roots of the trees below, in order, and the charts agreeing across the tree
edges among them. All of it is decided by the program on each region.

Then, at a node: the composite of the jumps of the walk round `v`, through the
charts, is the rotation by `R^{k}` about the developed position `M f (v)`
(`holonomyAround_cw` conjugated by `M f`); the jumps across the tree edges are
the identity and the last jump is the inverse of `G_e`, so
`G_e = G_{e_r} ∘ ⋯ ∘ G_{e_1} ∘ ρ_v` with `ρ_v = R^{4−k}` about `M f (v)`, the
recursion of the note; unfolded over the forest, `Forest.glue` of the cone
rotations (`cones`), hence their composite in preorder, the order of first
visit (`Forest.glue_eq_preorder`). With `borderHolonomy_cuts` and
`borderHolonomy_round`: the development of the turn is that composite,
Proposition "The shift is set by the singularities" (a) at whole lengths.

Here: the walk round a vertex with faces (`aroundF`, `aroundVertexF`), the
forest (`CutForest`, `roots`, `cones`), the predicate `hangs`.

Proved (standard axioms only):
* `aroundF_around`: `aroundF` carries the changes of chart of `around`;
  `aroundVertexF_chain`: the faces of the walk clockwise round `v` from `(v, d)`
  chain from `faceAt (v, d)` back to it.
* `holonomyAround_cw`: for a unit step `d`, the walk clockwise round `v` from
  `(v, d)` back to it in `k` crossings composes to `rotAbout (quarter k) (fine v)`.
* `rotAbout_conj`: `M ∘ rotAbout u p ∘ qinv M = rotAbout u (M p)` for `M` with a
  quarter-turn as linear part.
* `jump_reverse`: for `C` with a quarter-turn as linear part and `C' ∘ C = id`,
  `jump M (g, C', f) = qinv (jump M (f, C, g))`.
* `jump_node`: at a node of a forest that hangs, through charts with quarter-turns
  as linear parts,
  `jump M c = composeAll ((roots below).map (jump M)) ∘ rotAbout (quarter (4 − k)) (M f (v))`.
* `roots_glue`: for a forest that hangs, the composite of the jumps of its roots
  is `Forest.glue` of its cone rotations.
* `borderHolonomy_forest`: with the hypotheses of `borderHolonomy_cuts`, when the
  cut crossings of the border are the roots of a forest that hangs, the holonomy
  of the border is `composeAll (rotations (preorder (cones M F)))`.
* `development_eq_cones`: with the hypotheses of `borderHolonomy_round` as well,
  the development of the turn of the round, in half quads, is that composite.

That `d` is a unit step at every node is part of `hangs`, decided like the rest:
without it the walk round `v` may stop on a degenerate square.
-/
module

public import TurnsOfATMesh.Region

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

/-- Motions over types with decidable equality compare componentwise. -/
instance {A V : Type*} [DecidableEq A] [DecidableEq V] : DecidableEq (Motion A V) :=
  fun f g => decidable_of_iff (f.lin = g.lin ∧ f.trans = g.trans) (Motion.ext_iff).symm

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### The walk round a vertex, with faces -/

/-- `around` with the faces: each step of the walk `ρ`-wise round `v` from the
state `(v, d)` leaves the face on the `−ρ` side of the edge `(v, d)` and enters
the face on the `−ρ` side of the next state `(v', d')` (the face on the `ρ`
side of `(v, d)`, read in the frame entered). -/
def aroundF (ℓ : α → ℕ) (ρ : GaussianInt) :
    ℕ → Point Patch → GaussianInt → Point Patch → GaussianInt → Option (List (Crossing Patch))
  | 0, _, _, _, _ => none
  | fuel + 1, v, d, v₀, d₀ =>
    (T.aroundStep ℓ ρ v d).bind fun (v', d', C) =>
      let c : Crossing Patch := (faceBeside v d (-ρ), C, faceBeside v' d' (-ρ))
      if v' = v₀ ∧ d' = d₀ then some [c] else (aroundF ℓ ρ fuel v' d' v₀ d₀).map (c :: ·)

/-- The walk `ρ`-wise round `v` from the state `(v, d)` back to it, with faces:
clockwise for `ρ = −i`, from the face on the left of `(v, d)`. -/
def aroundVertexF (ℓ : α → ℕ) (ρ : GaussianInt) (fuel : ℕ) (v : Point Patch) (d : GaussianInt) :
    Option (List (Crossing Patch)) :=
  T.aroundF ℓ ρ fuel v d v d

/-! ### A forest of cuts -/

/-- A forest of cuts of a region, first-child next-sibling: `node c s k below
next` is a cut, by its crossing `c = (f, C, g)` from the face before it to the
face after it clockwise round its near end; its far end `v` with the direction
`d` from `v` towards the near end, in the frame of `f` (`s = (v, d)`); the
number `k` of faces round `v`; the forest `below` of the cuts leaving `v`, in
the order the walk clockwise round `v` from the cut meets them; and the next
cuts `next` at the same vertex. -/
inductive CutForest (Patch : Type*)
  | nil : CutForest Patch
  | node (c : Crossing Patch) (s : Step Patch) (k : ℕ) (below next : CutForest Patch) :
      CutForest Patch

namespace CutForest

variable {Patch : Type*}

/-- The crossings of the roots, in order. -/
def roots : CutForest Patch → List (Crossing Patch)
  | nil => []
  | node c _ _ _ next => c :: roots next

/-- The cone points of the forest, through the charts `M`: at each node the
rotation `R^{4−k}` and the far end developed in the chart of the face before
the cut, `M f (v)`, the position of its first visit. -/
def cones (M : Face Patch → Motion GaussianInt GaussianInt) :
    CutForest Patch → Forest (GaussianInt × GaussianInt)
  | nil => .nil
  | node c s k below next =>
    .node (quarter (4 - (k : ℤ)), (M c.1).apply (fine s.1)) (cones M below) (cones M next)

/-- The number of nodes. -/
def size : CutForest Patch → ℕ
  | nil => 0
  | node _ _ _ below next => size below + size next + 1

end CutForest

/-- Whether a Gaussian integer is a quarter-turn, decided. -/
instance : DecidablePred IsQuarter := fun a =>
  inferInstanceAs (Decidable (a = 1 ∨ a = gi ∨ a = -1 ∨ a = -gi))

/-- **The forest hangs from the region**: at each node `(c, (v, d), k, below, next)`
with `c = (f, C, g)`, `d` is a unit step; the face on the left of `(v, −i d)` is `f`; the walk
clockwise round `v` from `(v, −i d)` comes back in exactly `k` crossings; its
last crossing is the cut crossed back, from `g` to `f`, by a change of chart
inverse to `C`; among the others, the crossings off the tree are the roots of
`below`, in order, and across the tree edges the charts agree; and `below` and
`next` hang in turn. -/
def hangs (M : Face Patch → Motion GaussianInt GaussianInt) (ℓ : α → ℕ) (fuel : ℕ)
    (tree : Face Patch → Face Patch → Bool) : CutForest Patch → Bool
  | .nil => true
  | .node c s k below next =>
    decide (IsQuarter s.2) && decide (faceAt (s.1, -gi * s.2) = c.1) &&
    (match T.aroundVertexF ℓ (-gi) fuel s.1 (-gi * s.2) with
      | none => false
      | some cs =>
        decide (cs.length = k) &&
        (match cs.getLast? with
          | none => false
          | some (f', C', g') =>
            decide (f' = c.2.2 ∧ g' = c.1 ∧ Motion.comp C' c.2.1 = Motion.id)) &&
        decide (cutCrossings tree cs.dropLast = below.roots) &&
        (cs.dropLast.filter fun c' => tree c'.1 c'.2.2).all fun c' =>
          decide (Motion.comp (M c'.2.2) c'.2.1 = M c'.1)) &&
    hangs M ℓ fuel tree below && hangs M ℓ fuel tree next

/-! ### First facts -/

@[simp] theorem aroundF_zero (ℓ : α → ℕ) (ρ : GaussianInt) (v : Point Patch) (d : GaussianInt)
    (v₀ : Point Patch) (d₀ : GaussianInt) : T.aroundF ℓ ρ 0 v d v₀ d₀ = none := rfl

@[simp] theorem roots_nil : (CutForest.nil : CutForest Patch).roots = [] := rfl

@[simp] theorem roots_node (c : Crossing Patch) (s : Step Patch) (k : ℕ)
    (below next : CutForest Patch) : (CutForest.node c s k below next).roots = c :: next.roots := rfl

@[simp] theorem cones_nil (M : Face Patch → Motion GaussianInt GaussianInt) :
    (CutForest.nil : CutForest Patch).cones M = .nil := rfl

@[simp] theorem hangs_nil (M : Face Patch → Motion GaussianInt GaussianInt) (ℓ : α → ℕ) (fuel : ℕ)
    (tree : Face Patch → Face Patch → Bool) : T.hangs M ℓ fuel tree .nil = true := rfl

/-- The preorder of the cones is the order of first visit: a node before the
nodes below it, then the next cuts. -/
theorem preorder_cones (M : Face Patch → Motion GaussianInt GaussianInt) (c : Crossing Patch)
    (s : Step Patch) (k : ℕ) (below next : CutForest Patch) :
    Forest.preorder ((CutForest.node c s k below next).cones M) =
      (quarter (4 - (k : ℤ)), (M c.1).apply (fine s.1)) ::
        (Forest.preorder (below.cones M) ++ Forest.preorder (next.cones M)) := rfl

/-! ### The walk round a vertex with faces -/

/-- **`aroundF` carries the changes of chart of `around`.** -/
theorem aroundF_around (ℓ : α → ℕ) (ρ : GaussianInt) :
    ∀ (fuel : ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch) (d₀ : GaussianInt),
      (T.aroundF ℓ ρ fuel v d v₀ d₀).map (List.map (·.2.1)) = T.around ℓ ρ fuel v d v₀ d₀
  | 0, _, _, _, _ => rfl
  | fuel + 1, v, d, v₀, d₀ => by
    rw [aroundF, around]
    cases h : T.aroundStep ℓ ρ v d with
    | none => rfl
    | some r =>
      obtain ⟨v', d', C⟩ := r
      simp only [Option.bind_some]
      split_ifs
      · rfl
      · rw [← aroundF_around ℓ ρ fuel v' d' v₀ d₀, Option.map_map]
        cases T.aroundF ℓ ρ fuel v' d' v₀ d₀ <;> rfl

/-- The crossings of `aroundF` run from the face on the `−ρ` side of the state it
starts from to the face on the `−ρ` side of the state it stops at. -/
theorem aroundF_chain (ℓ : α → ℕ) (ρ : GaussianInt) :
    ∀ (fuel : ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch) (d₀ : GaussianInt)
      (cs : List (Crossing Patch)), T.aroundF ℓ ρ fuel v d v₀ d₀ = some cs →
      FacesChain (faceBeside v d (-ρ)) cs (faceBeside v₀ d₀ (-ρ))
  | 0, _, _, _, _, _, h => by simp at h
  | fuel + 1, v, d, v₀, d₀, cs, h => by
    rw [aroundF] at h
    obtain ⟨⟨v', d', C⟩, -, h'⟩ := Option.bind_eq_some_iff.mp h
    simp only at h'
    split_ifs at h' with hs
    · obtain ⟨rfl, rfl⟩ := hs
      cases Option.some.inj h'
      rfl
    · obtain ⟨cs', hcs', rfl⟩ := Option.map_eq_some_iff.mp h'
      have ih := aroundF_chain ℓ ρ fuel v' d' v₀ d₀ cs' hcs'
      unfold FacesChain at ih ⊢
      simp only [List.map_cons, List.cons_append, ih]

/-- **The faces of the walk clockwise round a vertex chain** from the face on the left
of the state it starts from back to it. -/
theorem aroundVertexF_chain {ℓ : α → ℕ} {fuel : ℕ} {v : Point Patch} {d : GaussianInt}
    {cs : List (Crossing Patch)} (h : T.aroundVertexF ℓ (-gi) fuel v d = some cs) :
    FacesChain (faceAt (v, d)) cs (faceAt (v, d)) := by
  have := T.aroundF_chain ℓ (-gi) fuel v d v d cs h
  rwa [neg_neg] at this

/-- The change of chart of a step round a vertex has a quarter-turn as linear part. -/
theorem aroundStep_lin_quarter {ℓ : α → ℕ} {ρ : GaussianInt} {v : Point Patch} {d : GaussianInt}
    {v' : Point Patch} {d' : GaussianInt} {C : Motion GaussianInt GaussianInt}
    (h : T.aroundStep ℓ ρ v d = some (v', d', C)) : IsQuarter C.lin := by
  unfold aroundStep at h
  dsimp only at h
  split_ifs at h with h1 h2
  · simp only [Option.some.injEq, Prod.mk.injEq] at h
    rw [← h.2.2]; exact IsQuarter.one
  · obtain ⟨⟨Q, C'⟩, hc, h'⟩ := Option.bind_eq_some_iff.mp h
    simp only at h'
    split_ifs at h' with h3
    simp only [Option.some.injEq, Prod.mk.injEq] at h'
    rw [← h'.2.2]
    exact T.crossEdge_lin_quarter hc

theorem around_lin_quarter (ℓ : α → ℕ) (ρ : GaussianInt) :
    ∀ (fuel : ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch) (d₀ : GaussianInt)
      (L : List (Motion GaussianInt GaussianInt)), T.around ℓ ρ fuel v d v₀ d₀ = some L →
      ∀ C ∈ L, IsQuarter C.lin
  | 0, _, _, _, _, _, h => by simp [around] at h
  | fuel + 1, v, d, v₀, d₀, L, h => by
    obtain ⟨v', d', C, hs, h'⟩ := T.around_some h
    rcases h' with ⟨-, rfl⟩ | ⟨-, L', hL', rfl⟩
    · simpa using T.aroundStep_lin_quarter hs
    · intro D hD
      rcases List.mem_cons.mp hD with rfl | hD
      · exact T.aroundStep_lin_quarter hs
      · exact around_lin_quarter ℓ ρ fuel v' d' v₀ d₀ L' hL' D hD

/-- **The holonomy clockwise round a vertex**: the walk clockwise round `v` from the
state `(v, d)` back to it, in `k` crossings, composes to the rotation `R^k` about `v`
(the inverse of the counter-clockwise `R^{4−k}` of `holonomyAround_eq`). -/
theorem holonomyAround_cw {ℓ : α → ℕ} {fuel : ℕ} {v : Point Patch} {d : GaussianInt}
    (hd : IsQuarter d) {cs : List (Motion GaussianInt GaussianInt)}
    (h : T.around ℓ (-gi) fuel v d v d = some cs) :
    Motion.composeAll cs = Motion.rotAbout (quarter (cs.length : ℤ)) (fine v) := by
  obtain ⟨ha, hdd⟩ := T.around_apply ℓ (-gi) fuel v d v d cs h
  have hu := mul_conj_of_isQuarter hd
  have h1 : (-gi) ^ cs.length * (Motion.composeAll cs).lin = 1 := by
    linear_combination (-(conj d)) * hdd - ((-gi) ^ cs.length * (Motion.composeAll cs).lin - 1) * hu
  have hq : quarter (cs.length : ℤ) = gi ^ cs.length := by
    rw [quarter, zpow_natCast, Units.val_pow_eq_pow_val, giUnit_val]
  have h2 : gi ^ cs.length * (-gi) ^ cs.length = 1 := by
    rw [← mul_pow, mul_neg, gi_mul_gi, neg_neg, one_pow]
  have hlin : (Motion.composeAll cs).lin = quarter (cs.length : ℤ) := by
    rw [hq]
    linear_combination gi ^ cs.length * h1 - (Motion.composeAll cs).lin * h2
  apply Motion.eq_of_apply (fine v)
  · simpa using hlin
  · rw [ha, Motion.apply_rotAbout]; simp

/-! ### Motions with a quarter-turn as linear part -/

theorem isQuarter_conj {z : GaussianInt} (hz : IsQuarter z) : IsQuarter (conj z) := by
  rcases hz with rfl | rfl | rfl | rfl
  · left; ext <;> simp [conj]
  · right; right; right; ext <;> simp [conj, gi]
  · right; right; left; ext <;> simp [conj]
  · right; left; ext <;> simp [conj, gi]

theorem isQuarter_of_mul_eq_one {a b : GaussianInt} (ha : IsQuarter a) (h : a * b = 1) :
    IsQuarter b := by
  have hb : b = conj a := by
    linear_combination conj a * h - b * mul_conj_of_isQuarter ha
  rw [hb]
  exact isQuarter_conj ha

/-- A left inverse of a motion with a quarter-turn as linear part is `qinv`. -/
theorem eq_qinv {X Y : Motion GaussianInt GaussianInt} (hY : IsQuarter Y.lin)
    (h : Motion.comp X Y = Motion.id) : X = qinv Y := by
  calc X = Motion.comp X (Motion.comp Y (qinv Y)) := by rw [comp_qinv hY, Motion.comp_id]
    _ = qinv Y := by rw [← Motion.comp_assoc, h, Motion.id_comp]

/-- **Conjugating a rotation**: `M ∘ rotAbout u p ∘ M⁻¹ = rotAbout u (M p)`, for `M`
with a quarter-turn as linear part. -/
theorem rotAbout_conj (M : Motion GaussianInt GaussianInt) (hq : IsQuarter M.lin) (u p : GaussianInt) :
    Motion.comp M (Motion.comp (Motion.rotAbout u p) (qinv M)) = Motion.rotAbout u (M.apply p) := by
  have h := mul_conj_of_isQuarter hq
  refine Motion.ext ?_ ?_
  · simp only [Motion.comp_lin, Motion.rotAbout_lin, qinv_lin]
    linear_combination u * h
  · simp only [Motion.comp_trans, Motion.rotAbout_lin, Motion.rotAbout_trans, qinv_trans,
      Motion.apply, smul_eq_mul]
    linear_combination (-(u * M.trans)) * h

theorem rotAbout_comp_rotAbout (a b p : GaussianInt) :
    Motion.comp (Motion.rotAbout a p) (Motion.rotAbout b p) = Motion.rotAbout (a * b) p := by
  refine Motion.ext rfl ?_
  simp only [Motion.comp_trans, Motion.rotAbout_lin, Motion.rotAbout_trans, smul_eq_mul]
  ring

theorem rotAbout_one (p : GaussianInt) : Motion.rotAbout (1 : GaussianInt) p = Motion.id := by
  exact Motion.ext rfl (by simp)

theorem quarter_four : quarter 4 = 1 := by
  rw [show (4 : ℤ) = 2 + 2 by norm_num, quarter_add, quarter_two]; norm_num

/-! ### The jumps of a node -/

theorem jump_lin_quarter (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) (c : Crossing Patch) (hC : IsQuarter c.2.1.lin) :
    IsQuarter (jump M c).lin := by
  simp only [jump, Motion.comp_lin, qinv_lin]
  exact (hq _).mul (hC.mul (isQuarter_conj (hq _)))

/-- **A crossing walked back**, by the inverse change of chart, jumps by the inverse. -/
theorem jump_reverse (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) {f g : Face Patch} {C C' : Motion GaussianInt GaussianInt}
    (hC : IsQuarter C.lin) (h : Motion.comp C' C = Motion.id) :
    jump M (g, C', f) = qinv (jump M (f, C, g)) := by
  apply eq_qinv (jump_lin_quarter M hq (f, C, g) hC)
  simp only [jump]
  rw [Motion.comp_assoc, Motion.comp_assoc, ← Motion.comp_assoc (qinv (M g)), qinv_comp (hq g),
    Motion.id_comp, ← Motion.comp_assoc C', h, Motion.id_comp, comp_qinv (hq f)]

/-- **The recursion at a node** (`Cut.jump_eq` on the grid): at a node of a forest that
hangs, the jump across the cut is the composite of the jumps across the cuts leaving
its far end `v`, after the cone rotation `R^{4−k}` about `v` developed in the chart of
the face before the cut. The walk clockwise round `v` composes, through the charts, to
`R^{k}` about that point (`holonomyAround_cw`, the telescope `facesChain_telescope`
from `f` back to `f`, `rotAbout_conj`); its crossings of the tree jump by the identity
and its last crossing is the cut walked back (`jump_reverse`). -/
theorem jump_node (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) {ℓ : α → ℕ} {fuel : ℕ} {tree : Face Patch → Face Patch → Bool}
    {c : Crossing Patch} {s : Step Patch} {k : ℕ} {below next : CutForest Patch}
    (h : T.hangs M ℓ fuel tree (.node c s k below next) = true) :
    jump M c = Motion.comp (Motion.composeAll (below.roots.map (jump M)))
      (Motion.rotAbout (quarter (4 - (k : ℤ))) ((M c.1).apply (fine s.1))) := by
  obtain ⟨f, C, g⟩ := c
  obtain ⟨v, d⟩ := s
  simp only [hangs, Bool.and_eq_true, decide_eq_true_eq] at h
  obtain ⟨⟨⟨⟨hd, hf⟩, hw⟩, -⟩, -⟩ := h
  dsimp only at hd hf hw ⊢
  cases hcs : T.aroundVertexF ℓ (-gi) fuel v (-gi * d) with
  | none => rw [hcs] at hw; exact absurd hw Bool.false_ne_true
  | some cs =>
    rw [hcs] at hw
    simp only [Bool.and_eq_true, decide_eq_true_eq] at hw
    obtain ⟨⟨⟨hlen, hlast⟩, hroots⟩, hall⟩ := hw
    cases hl : cs.getLast? with
    | none => rw [hl] at hlast; exact absurd hlast Bool.false_ne_true
    | some x =>
      obtain ⟨f', C', g'⟩ := x
      rw [hl] at hlast
      simp only [decide_eq_true_eq] at hlast
      obtain ⟨hg, hf₀, hinv⟩ := hlast
      subst f' g'
      -- The walk round `v`, through the charts: `R^{k}` about `M f (v)`.
      have hd' : IsQuarter (-gi * d) :=
        (show IsQuarter (-gi) from Or.inr (Or.inr (Or.inr rfl))).mul hd
      have ha : T.around ℓ (-gi) fuel v (-gi * d) v (-gi * d) = some (cs.map (·.2.1)) := by
        rw [← T.aroundF_around]
        unfold aroundVertexF at hcs
        rw [hcs]; rfl
      have hround : Motion.composeAll (cs.map (·.2.1)) =
          Motion.rotAbout (quarter (k : ℤ)) (fine v) := by
        rw [T.holonomyAround_cw hd' ha, List.length_map, hlen]
      have hchain := T.aroundVertexF_chain hcs
      rw [hf] at hchain
      have htel := facesChain_telescope M hq cs hchain
      have hjumps : Motion.composeAll (cs.map (jump M)) =
          Motion.rotAbout (quarter (k : ℤ)) ((M f).apply (fine v)) := by
        calc Motion.composeAll (cs.map (jump M))
            = Motion.comp (Motion.comp (Motion.composeAll (cs.map (jump M))) (M f)) (qinv (M f)) := by
              rw [Motion.comp_assoc, comp_qinv (hq f), Motion.comp_id]
          _ = Motion.comp (Motion.comp (M f) (Motion.rotAbout (quarter (k : ℤ)) (fine v)))
                (qinv (M f)) := by
              rw [← htel, hround]
          _ = _ := by rw [Motion.comp_assoc, rotAbout_conj _ (hq f)]
      -- The crossings before the last: the jumps across the tree are the identity.
      have hB : Motion.composeAll (cs.dropLast.map (jump M)) =
          Motion.composeAll (below.roots.map (jump M)) := by
        rw [← hroots, cutCrossings, composeAll_filter_id (jump M) (fun c => !tree c.1 c.2.2)]
        intro c hc hcut
        simp only [Bool.not_eq_false'] at hcut
        have hc' : c ∈ cs.dropLast.filter fun c' => tree c'.1 c'.2.2 :=
          List.mem_filter.mpr ⟨hc, hcut⟩
        exact jump_tree M c (hq _) (of_decide_eq_true (List.all_eq_true.mp hall c hc'))
      -- The last crossing is the cut walked back.
      have hmem : (g, C', f) ∈ cs := List.mem_of_getLast? hl
      have hC' : IsQuarter C'.lin :=
        T.around_lin_quarter ℓ (-gi) fuel v (-gi * d) v (-gi * d) _ ha C'
          (List.mem_map.mpr ⟨_, hmem, rfl⟩)
      have hC : IsQuarter C.lin := by
        refine isQuarter_of_mul_eq_one hC' ?_
        have := congrArg Motion.lin hinv
        simpa using this
      have hsplit := List.dropLast_append_getLast? (g, C', f) (by simpa using hl)
      rw [← hsplit, List.map_append, Motion.composeAll_append, hB, List.map_singleton,
        Motion.composeAll_cons, Motion.composeAll_nil, Motion.id_comp,
        jump_reverse M hq hC hinv] at hjumps
      -- `qinv J ∘ B = R^{k}`, so `J = B ∘ R^{4−k}`.
      have hJ := jump_lin_quarter M hq (f, C, g) hC
      have hk : quarter (k : ℤ) * quarter (4 - (k : ℤ)) = 1 := by
        rw [← quarter_add, show (k : ℤ) + (4 - k) = 4 by ring, quarter_four]
      calc jump M (f, C, g)
          = Motion.comp (Motion.comp (jump M (f, C, g)) (Motion.comp (qinv (jump M (f, C, g)))
              (Motion.composeAll (below.roots.map (jump M)))))
              (Motion.rotAbout (quarter (4 - (k : ℤ))) ((M f).apply (fine v))) := by
            rw [hjumps, Motion.comp_assoc, rotAbout_comp_rotAbout, hk, rotAbout_one, Motion.comp_id]
        _ = _ := by
            rw [← Motion.comp_assoc (jump M (f, C, g)), comp_qinv hJ, Motion.id_comp]

/-- **The roots of a forest that hangs glue its cone rotations**: the composite of
the jumps across its roots, in order, is `Forest.glue` of the rotations `R^{4−k}`
about the cone points, by induction with `jump_node`. -/
theorem roots_glue (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) {ℓ : α → ℕ} {fuel : ℕ} {tree : Face Patch → Face Patch → Bool} :
    ∀ F : CutForest Patch, T.hangs M ℓ fuel tree F = true →
      Motion.composeAll (F.roots.map (jump M)) =
        Forest.glue ((F.cones M).map fun c => Motion.rotAbout c.1 c.2)
  | .nil, _ => rfl
  | .node c s k below next, h => by
    have h' := h
    simp only [hangs, Bool.and_eq_true] at h'
    rw [roots_node, List.map_cons, Motion.composeAll_cons, roots_glue M hq next h'.2,
      T.jump_node M hq h, roots_glue M hq below h'.1.2]
    rfl

/-- **The holonomy of the border is the composite of the cone rotations in the order
of first visit**: with the hypotheses of `borderHolonomy_cuts`, when the cut crossings
of the border are the roots of a forest that hangs from the region. -/
theorem borderHolonomy_forest (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) (tree : Face Patch → Face Patch → Bool) {ℓ : α → ℕ}
    {fuel fuel' : ℕ} {s s' : Step Patch} {ss : List (Step Patch)} {cs : List (Crossing Patch)}
    (hd : ∀ t ∈ s :: ss, IsQuarter t.2) (hs : T.normalize ℓ s = some s')
    (h₀ : M (faceAt s') = Motion.id) (h : T.borderCrossings ℓ fuel (s :: ss) = some cs)
    (htree : ∀ c ∈ cs, tree c.1 c.2.2 = true → Motion.comp (M c.2.2) c.2.1 = M c.1)
    {F : CutForest Patch} (hroots : cutCrossings tree cs = F.roots)
    (hF : T.hangs M ℓ fuel' tree F = true) :
    T.borderHolonomy ℓ fuel (s :: ss) =
      some (Motion.composeAll (Motion.rotations (Forest.preorder (F.cones M)))) := by
  rw [T.borderHolonomy_cuts M hq tree hd hs h₀ h htree, hroots, T.roots_glue M hq F hF,
    Forest.glue_eq_preorder, Forest.preorder_map]
  rfl

/-- **Proposition "The shift is set by the singularities" (a) at whole lengths, for the
region on the left of a round.** The development of the turn of the round from the
station `k + ½` of `σ`, in half quads, is the composite of the rotations `R^{4−k_v}`
about the cone points `v` of a forest of cuts of the region, each at the position of
its first visit, in the order of first visit (the preorder of the forest).

Hypotheses of the setting: an oriented T-mesh at whole lengths, every side of positive
length and the lengths balanced, every node strictly inside `σ` met by four faces
(those of `borderHolonomy_round`); the steps of the round unit steps; charts `M` of the
faces with quarter-turns as linear parts, the identity on the face on the left of the
first step, agreeing across the crossings of the border that `tree` holds for.
Decided by the program on every region, not proved: that the forest it builds hangs
from the region (`hangs`) and that its roots are the cut crossings of the border. -/
theorem development_eq_cones [Fintype α] (hT : T.Oriented) {ℓ : α → ℕ}
    (hpos : ∀ τ : Side Patch, 0 < T.sideLen ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ.opp = T.sideLen ℓ τ) {σ : Side Patch} {k fuel' : ℕ}
    (hσ : ∀ m : ℕ, 0 < m → m < T.sideLen ℓ σ →
      T.valenceAround ℓ fuel' (T.sidePoint ℓ σ (2 * m)) (inward σ) = some 4)
    (hk : k < T.sideLen ℓ σ) {patches : List Patch} {s : Step Patch} {ss : List (Step Patch)}
    {path : List α} (h : T.roundSteps patches ℓ σ k = some (s :: ss, path))
    (M : Face Patch → Motion GaussianInt GaussianInt) (hq : ∀ f, IsQuarter (M f).lin)
    (tree : Face Patch → Face Patch → Bool) {s' : Step Patch} {cs : List (Crossing Patch)}
    (hd : ∀ t ∈ s :: ss, IsQuarter t.2) (hs : T.normalize ℓ s = some s')
    (h₀ : M (faceAt s') = Motion.id) (hc : T.borderCrossings ℓ 4 (s :: ss) = some cs)
    (htree : ∀ c ∈ cs, tree c.1 c.2.2 = true → Motion.comp (M c.2.2) c.2.1 = M c.1)
    {F : CutForest Patch} {fuel'' : ℕ} (hroots : cutCrossings tree cs = F.roots)
    (hF : T.hangs M ℓ fuel'' tree F = true) :
    fineMotion (Motion.composeAll (T.crossMotions ℓ σ path)) =
      Motion.composeAll (Motion.rotations (Forest.preorder (F.cones M))) := by
  have h1 := T.borderHolonomy_round hT hpos hbal hσ hk h
  rw [T.borderHolonomy_forest M hq tree hd hs h₀ hc htree hroots hF] at h1
  exact (Option.some.inj h1).symm

end TMesh

end TurnsOfATMesh
