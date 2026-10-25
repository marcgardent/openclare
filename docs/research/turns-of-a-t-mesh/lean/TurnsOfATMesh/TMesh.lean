/-
A combinatorial T-mesh, from which the balances and the shifts are derived:
Proposition "The shift of a turn is linear", and the chain from it to Lemma
"Handshake", Proposition "Free arcs", Remark "Sign-definite turns" and Theorem
"Forced shifts" on one object.

* **The T-mesh** (`TMesh`). Each patch has two pairs of opposite sides; a side
  is a list of arcs, in order from its first corner, the two sides of a pair
  listed from corresponding corners in the same direction, so that crossing a
  patch keeps the offset. Every arc lies on exactly two sides (`ends`), with a
  sign `ε = ±1` saying whether the two sides run along it in the same direction:
  a closed surface, borders left out. Lengths `ℓ` are functions of the arcs;
  positions on a side are partial sums of them, integer linear forms in `ℓ`
  (`posForm`, a form being a vector `α → ℤ` evaluated by `evalForm`).

* **Balances.** The matrix `B` has one row per pair of opposite sides of a
  patch, `B ℓ = 0` saying that the two sides have equal length
  (`balance_mulVec_eq_zero_iff`). Its columns have even sums
  (`balance_column_even`), so Lemma "Handshake" holds on every T-mesh, and it is
  the incidence matrix of a bidirected graph when no arc lies on two opposite
  sides of one patch (`balance_isBidirected`), so Proposition "Free arcs" does.

* **Turns.** Crossing a patch keeps the offset; crossing the arc `a` from a side
  to the other side holding it is `x ↦ ε x + c(ℓ)`, `c` an integer linear form
  in `ℓ` (`crossForm`; at given lengths, `crossAt`). A turn (`Turn`) is a list of
  arcs crossed in turn from a first side and back to it. When the patches carry
  frames of one orientation of the surface (`Oriented`), the composite of its
  crossings is a translation, by the integer linear form `shift` of the lengths
  (`Turn.shift_linear`): the coefficients do not depend on `ℓ`.

* **The walk and its corridor.** At lengths `ℓ ≥ 0`, the walk of Definition
  "Turn" from an offset `x` of a side goes to the opposite side at the same
  offset and crosses the arc whose interior holds `x` there (`step`, `walk`).
  The corridor of a turn is the set of offsets from which the walk crosses the
  arcs of the turn (`Turn.corridor`); it is cut out by strict linear inequalities
  with integer coefficients in `(x, ℓ)` (`Turn.mem_corridor_iff`). From an offset
  `o` of the corridor the walk comes back to the first side at `o + shift(ℓ)`
  (`Turn.comes_back`), and winds `t` times as a helix while `o + j shift(ℓ)`
  stays in the corridor (`Turn.winds`).

* **Wiring.** With the shifts of these turns and `P` the balanced lengths above
  the bounds of separation, Theorem "Forced shifts" (`forced_helices`) and
  Remark "Sign-definite turns" (`sign_definite_certificate`) apply to the matrix
  `B` of the T-mesh and the matrix of the shift forms: `not_closable_iff`,
  `forced_of_certificates`.

The crossings are motions `Motion ℤ (α → ℤ)` of `Basic.lean`, their linear part
the sign and their translation a form, composed by `Motion.composeAll`; reading
them at `ℓ` commutes with composing them (`evalMotion_composeAll`), which is the
proof of linearity. A torus of one patch glued with a twist (`twistedTorus`)
checks that the definitions are not vacuous: its vertical turn has shift
`ℓ_{c₁}`.

Not modelled: the surface and the cross field behind the T-mesh, the geometry of
the patches (only the offset across the walk is followed, not the length of the
turn), borders, the strips of the quad mesh (`Completeness.lean` relates walk and
strips in its own model at whole lengths) and the link with the holonomy of
`Fox.lean`. The orientation is a hypothesis on the signs of the gluing
(`Oriented`).
-/
module

public import TurnsOfATMesh.Motion
public import TurnsOfATMesh.Bidirected
public import TurnsOfATMesh.Farkas
public import TurnsOfATMesh.Forced

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

open Finset Matrix

/-! ### Integer linear forms in the lengths -/

section Forms

variable {α : Type*} [Fintype α] {K : Type*} [CommRing K]

/-- The value `∑ₐ cₐ ℓₐ` of the integer linear form `c` at the lengths `ℓ`. -/
def evalForm (c : α → ℤ) (ℓ : α → K) : K := ∑ a, (c a : K) * ℓ a

@[simp] theorem evalForm_zero (ℓ : α → K) : evalForm (0 : α → ℤ) ℓ = 0 := by
  simp [evalForm]

theorem evalForm_add (c d : α → ℤ) (ℓ : α → K) :
    evalForm (c + d) ℓ = evalForm c ℓ + evalForm d ℓ := by
  simp [evalForm, add_mul, sum_add_distrib]

theorem evalForm_sub (c d : α → ℤ) (ℓ : α → K) :
    evalForm (c - d) ℓ = evalForm c ℓ - evalForm d ℓ := by
  simp [evalForm, sub_mul, sum_sub_distrib]

theorem evalForm_smul (n : ℤ) (c : α → ℤ) (ℓ : α → K) :
    evalForm (n • c) ℓ = n * evalForm c ℓ := by
  simp [evalForm, mul_sum, mul_assoc]

theorem evalForm_single [DecidableEq α] (b : α) (ℓ : α → K) :
    evalForm (Pi.single b 1) ℓ = ℓ b := by
  simp [evalForm, Pi.single_apply]

/-- A motion `x ↦ ε x + c` whose translation `c` is an integer linear form, read at
the lengths `ℓ`. -/
def evalMotion (f : Motion ℤ (α → ℤ)) (ℓ : α → K) : Motion K K := ⟨(f.lin : K), evalForm f.trans ℓ⟩

theorem evalMotion_comp (f g : Motion ℤ (α → ℤ)) (ℓ : α → K) :
    evalMotion (Motion.comp f g) ℓ = Motion.comp (evalMotion f ℓ) (evalMotion g ℓ) := by
  ext
  · simp [evalMotion]
  · simp only [evalMotion, Motion.comp_trans, evalForm_add, evalForm_smul, smul_eq_mul]

theorem evalMotion_composeAll (fs : List (Motion ℤ (α → ℤ))) (ℓ : α → K) :
    evalMotion (Motion.composeAll fs) ℓ = Motion.composeAll (fs.map (evalMotion · ℓ)) := by
  induction fs with
  | nil => ext <;> simp [evalMotion, Motion.id]
  | cons f fs ih => simp [evalMotion_comp, ih]

theorem evalMotion_apply (f : Motion ℤ (α → ℤ)) (ℓ : α → K) (x : K) :
    (evalMotion f ℓ).apply x = f.lin * x + evalForm f.trans ℓ := by
  simp [evalMotion, Motion.apply]

end Forms

/-! ### Sides, positions and lengths on a side -/

namespace TMesh

/-- A side of a patch: the patch, the pair of opposite sides it belongs to, and
which side of the pair it is. In the frame `(u, v)` of the patch, the pair `0`
holds the sides `v = 0` and `v = h`, listed along `u`, and the pair `1` the sides
`u = 0` and `u = w`, listed along `v`. -/
abbrev Side (Patch : Type*) := Patch × Fin 2 × Fin 2

namespace Side

variable {Patch : Type*}

/-- The balance of a side: its patch and its pair. -/
def row (σ : Side Patch) : Patch × Fin 2 := (σ.1, σ.2.1)

/-- The opposite side. -/
def opp (σ : Side Patch) : Side Patch := (σ.1, σ.2.1, σ.2.2 + 1)

/-- The coefficient of the side in its balance: `+1` for the first side of the
pair, `-1` for the second. -/
def sign (σ : Side Patch) : ℤ := if σ.2.2 = 0 then 1 else -1

/-- `+1` if the side is listed in the direction of the boundary of its patch
(anticlockwise in the frame `(u, v)`), `-1` otherwise: `+1` on `v = 0` and
`u = w`, `-1` on `v = h` and `u = 0`. -/
def orient (σ : Side Patch) : ℤ := if σ.2.1 = σ.2.2 then 1 else -1

theorem orient_opp (σ : Side Patch) : σ.opp.orient = -σ.orient := by
  obtain ⟨P, j, s⟩ := σ
  fin_cases j <;> fin_cases s <;> rfl

theorem orient_mul_self (σ : Side Patch) : σ.orient * σ.orient = 1 := by
  unfold orient; split_ifs <;> rfl

theorem isSign_sign (σ : Side Patch) : Bidirected.IsSign σ.sign := by
  unfold sign Bidirected.IsSign; split_ifs <;> simp

theorem opp_row (σ : Side Patch) : σ.opp.row = σ.row := rfl

end Side

variable {α : Type*} [DecidableEq α]

/-- The form of the total length of the arcs of a list. -/
def lengthForm : List α → α → ℤ
  | [] => 0
  | b :: l => Pi.single b 1 + lengthForm l

/-- The form of the position of the arc `a` on a side whose arcs are `l`: the total
length of the arcs before it. -/
def posForm (a : α) : List α → α → ℤ
  | [] => 0
  | b :: l => if b = a then 0 else Pi.single b 1 + posForm a l

theorem lengthForm_apply (l : List α) (a : α) : lengthForm l a = l.count a := by
  induction l with
  | nil => rfl
  | cons b l ih =>
    rw [lengthForm, Pi.add_apply, ih, List.count_cons, Pi.single_apply]
    by_cases h : b = a
    · subst h; simp [add_comm]
    · simp [h, Ne.symm h]

variable {K : Type*} [Field K] [LinearOrder K] [IsStrictOrderedRing K]

/-- The position of the arc `a` on a side whose arcs are `l`, at the lengths `ℓ`. -/
def posAt (a : α) : List α → (α → K) → K
  | [], _ => 0
  | b :: l, ℓ => if b = a then 0 else ℓ b + posAt a l ℓ

/-- The arc of a side (whose arcs are `l`) whose interior holds the offset `x`, at
the lengths `ℓ`: `none` when `x` is a node or off the side. -/
def arcAt (ℓ : α → K) : List α → K → Option α
  | [], _ => none
  | b :: l, x => if 0 < x ∧ x < ℓ b then some b else arcAt ℓ l (x - ℓ b)

variable [Fintype α]

theorem evalForm_lengthForm (l : List α) (ℓ : α → K) :
    evalForm (lengthForm l) ℓ = (l.map ℓ).sum := by
  induction l with
  | nil => simp [lengthForm]
  | cons b l ih => simp [lengthForm, evalForm_add, evalForm_single, ih]

/-- The position on a side is the integer linear form `posForm`. -/
theorem evalForm_posForm (a : α) (l : List α) (ℓ : α → K) :
    evalForm (posForm a l) ℓ = posAt a l ℓ := by
  induction l with
  | nil => simp [posForm, posAt]
  | cons b l ih =>
    simp only [posForm, posAt]
    split_ifs
    · simp
    · rw [evalForm_add, evalForm_single, ih]

omit [Fintype α] in
theorem posAt_nonneg {ℓ : α → K} (hℓ : ∀ a, 0 ≤ ℓ a) (a : α) (l : List α) :
    0 ≤ posAt a l ℓ := by
  induction l with
  | nil => simp [posAt]
  | cons b l ih =>
    simp only [posAt]
    split_ifs
    · exact le_rfl
    · exact add_nonneg (hℓ b) ih

omit [Fintype α] in
/-- **The arc under an offset.** At lengths `ℓ ≥ 0`, the arcs of a side cover
disjoint open intervals: the arc found under `x` is the arc `a` of the side with
`pos a < x < pos a + ℓ a`. -/
theorem arcAt_eq_some {ℓ : α → K} (hℓ : ∀ a, 0 ≤ ℓ a) {l : List α} (hl : l.Nodup) (x : K)
    (a : α) : arcAt ℓ l x = some a ↔ a ∈ l ∧ posAt a l ℓ < x ∧ x < posAt a l ℓ + ℓ a := by
  induction l generalizing x with
  | nil => simp [arcAt]
  | cons b l ih =>
    rw [List.nodup_cons] at hl
    by_cases hab : b = a
    · subst hab
      have hpos : posAt b (b :: l) ℓ = 0 := by simp [posAt]
      rw [hpos, zero_add]
      simp only [arcAt, List.mem_cons, true_or, true_and]
      split_ifs with h
      · simp [h]
      · rw [ih hl.2]
        constructor
        · rintro ⟨hb, -⟩; exact absurd hb hl.1
        · intro h'; exact absurd h' h
    · have hpos : posAt a (b :: l) ℓ = ℓ b + posAt a l ℓ := by simp [posAt, hab]
      rw [hpos]
      simp only [arcAt, List.mem_cons, Ne.symm hab, false_or]
      split_ifs with h
      · simp only [Option.some.injEq, hab, false_iff, not_and, not_lt]
        intro _ hlt
        have := posAt_nonneg hℓ a l
        linarith [h.2]
      · rw [ih hl.2]
        constructor <;> rintro ⟨hm, h₁, h₂⟩ <;> exact ⟨hm, by linarith, by linarith⟩

end TMesh

/-! ### The T-mesh -/

open TMesh in
/-- **A T-mesh of a closed surface.** The arcs of each side in order from its first
corner (the two sides of a pair from corresponding corners, in the same
direction); each arc lies on exactly the two distinct sides `ends a 0` and
`ends a 1`, once on each; `sign a = 1` if the two sides run along it in the same
direction, `-1` otherwise. -/
structure TMesh (Patch α : Type*) where
  /-- The arcs of a side, in order from its first corner. -/
  arcs : Side Patch → List α
  arcs_nodup : ∀ σ, (arcs σ).Nodup
  /-- The two sides an arc lies on. -/
  ends : α → Fin 2 → Side Patch
  ends_ne : ∀ a, ends a 0 ≠ ends a 1
  mem_arcs : ∀ a σ, a ∈ arcs σ ↔ σ = ends a 0 ∨ σ = ends a 1
  /-- The gluing of the arc on the offsets: `+1` if its two sides run along it in
  the same direction, `-1` if in opposite directions. -/
  sign : α → ℤ
  sign_isSign : ∀ a, Bidirected.IsSign (sign a)

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### The balances -/

/-- **The balance matrix** `B`: one row per pair of opposite sides of a patch,
`B ℓ = 0` when the two sides have equal length. -/
def balance : Matrix (Patch × Fin 2) α ℤ := fun r a =>
  lengthForm (T.arcs (r.1, r.2, 0)) a - lengthForm (T.arcs (r.1, r.2, 1)) a

/-- The number of times an arc lies on a side: once on each of its two sides. -/
theorem count_arcs (a : α) (σ : Side Patch) :
    (T.arcs σ).count a = (if T.ends a 0 = σ then 1 else 0) + (if T.ends a 1 = σ then 1 else 0) := by
  by_cases h : a ∈ T.arcs σ
  · rw [List.count_eq_one_of_mem (T.arcs_nodup σ) h]
    rcases (T.mem_arcs a σ).mp h with rfl | rfl
    · simp [Ne.symm (T.ends_ne a)]
    · simp [T.ends_ne a]
  · rw [List.count_eq_zero_of_not_mem h]
    have h' := (T.mem_arcs a σ).not.mp h
    push Not at h'
    simp [Ne.symm h'.1, Ne.symm h'.2]

/-- The two entries of a side `e` in the rows of its patch. -/
theorem single_side (e : Side Patch) (r : Patch × Fin 2) :
    ((if e = (r.1, r.2, 0) then 1 else 0) - (if e = (r.1, r.2, 1) then 1 else 0) : ℤ) =
      if r = e.row then e.sign else 0 := by
  obtain ⟨P, j, s⟩ := e
  obtain ⟨Q, k⟩ := r
  simp only [Side.row, Side.sign, Prod.mk.injEq]
  by_cases hPQ : P = Q <;> by_cases hjk : j = k
  · subst hPQ; subst hjk; fin_cases s <;> simp
  · simp [hjk, Ne.symm hjk]
  · simp [hPQ, Ne.symm hPQ]
  · simp [hPQ, Ne.symm hPQ]

/-- **The column of an arc**: its coefficient `±1` in the balance of each of its two
sides. -/
theorem balance_apply (r : Patch × Fin 2) (a : α) :
    T.balance r a = (if r = (T.ends a 0).row then (T.ends a 0).sign else 0) +
      (if r = (T.ends a 1).row then (T.ends a 1).sign else 0) := by
  rw [balance, lengthForm_apply, lengthForm_apply, T.count_arcs, T.count_arcs,
    ← single_side, ← single_side]
  push_cast
  ring

/-- `B ℓ` is, for each pair of opposite sides, the length of the first side minus
the length of the second. -/
theorem balance_mulVec [Fintype α] {K : Type*} [CommRing K] (ℓ : α → K) (r : Patch × Fin 2) :
    (T.balance.map (Int.cast : ℤ → K) *ᵥ ℓ) r =
      ((T.arcs (r.1, r.2, 0)).map ℓ).sum - ((T.arcs (r.1, r.2, 1)).map ℓ).sum := by
  have h : ∀ l : List α, ∑ a, ((lengthForm l a : ℤ) : K) * ℓ a = (l.map ℓ).sum := by
    intro l
    induction l with
    | nil => simp [lengthForm]
    | cons b l ih => simp [lengthForm, add_mul, sum_add_distrib, Pi.single_apply, ih]
  simp only [Matrix.mulVec, dotProduct, Matrix.map_apply, balance, Int.cast_sub, sub_mul,
    sum_sub_distrib, h]

/-- The balanced lengths are those for which the two sides of every pair have equal
length. -/
theorem balance_mulVec_eq_zero_iff [Fintype α] {K : Type*} [CommRing K] (ℓ : α → K) :
    T.balance.map (Int.cast : ℤ → K) *ᵥ ℓ = 0 ↔
      ∀ P j, ((T.arcs (P, j, 0)).map ℓ).sum = ((T.arcs (P, j, 1)).map ℓ).sum := by
  constructor
  · intro h P j
    have := congrFun h (P, j)
    rw [balance_mulVec, Pi.zero_apply, sub_eq_zero] at this
    exact this
  · intro h
    funext r
    rw [balance_mulVec, Pi.zero_apply, sub_eq_zero]
    exact h r.1 r.2

section Columns

variable [Fintype Patch] [Fintype α]

/-- The column of an arc sums to the sum of its two signs. -/
theorem balance_column_sum (a : α) :
    ∑ r, T.balance r a = (T.ends a 0).sign + (T.ends a 1).sign := by
  simp [balance_apply, sum_add_distrib]

/-- **Every column of `B` has an even sum**, on every T-mesh of a closed surface:
`±1 ± 1`. -/
theorem balance_column_even (a : α) : Even (∑ r, T.balance r a) := by
  rw [balance_column_sum]
  rcases (T.ends a 0).isSign_sign with h₀ | h₀ <;> rcases (T.ends a 1).isSign_sign with h₁ | h₁ <;>
    rw [h₀, h₁] <;> decide

omit [Fintype Patch] [Fintype α] in
/-- No arc lies on the two opposite sides of one pair of a patch (as when no patch
borders itself): the two sides of every arc lie in different balances. -/
def NoArcAcross : Prop := ∀ a, (T.ends a 0).row ≠ (T.ends a 1).row

/-- **`B` is the incidence matrix of a bidirected graph** when no arc lies on two
opposite sides of a patch: every column has exactly two entries `±1`, in the
balances of the two sides of the arc. -/
theorem balance_isBidirected (h : T.NoArcAcross) : Bidirected.IsBidirected T.balance := by
  intro a
  refine ⟨(T.ends a 0).row, (T.ends a 1).row, h a, ?_, ?_, fun r h₀ h₁ => ?_⟩
  · simpa [balance_apply, h a] using (T.ends a 0).isSign_sign
  · simpa [balance_apply, Ne.symm (h a)] using (T.ends a 1).isSign_sign
  · simp [balance_apply, h₀, h₁]

/-- **Lemma "Handshake" on a T-mesh of a closed surface**: `∑_r (B x)_r` is even. -/
theorem handshake (x : α → ℤ) : Even (∑ r, (T.balance *ᵥ x) r) :=
  TurnsOfATMesh.handshake T.balance T.balance_column_even x

/-- **Proposition "Free arcs" on a T-mesh** with no arc across a patch, a spanning
tree of the graph of its balances and an unbalanced cycle. -/
theorem free_arcs (h : T.NoArcAcross) {Tr : Finset α} {k : ℕ}
    (hT : Bidirected.IsTree T.balance univ Tr) (C : Bidirected.Cycle T.balance k)
    (hC : ¬ C.IsBalanced) :
    ∃ f ∉ Tr, Bidirected.ClosesUnbalanced T.balance Tr f ∧ ∀ x : α → ℤ,
      ∃! z : α → ℤ, T.balance *ᵥ z = 0 ∧ ∀ e ∉ insert f Tr, z e = x e :=
  Bidirected.free_arcs_of_cycle (T.balance_isBidirected h) hT C hC

end Columns

/-! ### Crossings -/

/-- The other side the arc `a` lies on, seen from the side `τ`. -/
def other (a : α) (τ : Side Patch) : Side Patch := if T.ends a 0 = τ then T.ends a 1 else T.ends a 0

/-- **Crossing the arc `a`** from the side `τ` to the other side holding it, on the
offsets: `x ↦ x - p + p'` if the two sides run along `a` in the same direction,
`x ↦ p + p' + ℓ_a - x` otherwise, `p` and `p'` the positions of `a` on the two
sides; as a motion `x ↦ ε x + c` with `c` an integer linear form in `ℓ`. -/
def crossForm (τ : Side Patch) (a : α) : Motion ℤ (α → ℤ) :=
  if T.sign a = 1 then ⟨1, posForm a (T.arcs (T.other a τ)) - posForm a (T.arcs τ)⟩
  else ⟨-1, posForm a (T.arcs τ) + posForm a (T.arcs (T.other a τ)) + Pi.single a 1⟩

variable [Fintype α] {K : Type*} [Field K] [LinearOrder K] [IsStrictOrderedRing K]

/-- The same crossing at the lengths `ℓ`. -/
def crossAt (ℓ : α → K) (τ : Side Patch) (a : α) : Motion K K :=
  if T.sign a = 1 then ⟨1, posAt a (T.arcs (T.other a τ)) ℓ - posAt a (T.arcs τ) ℓ⟩
  else ⟨-1, posAt a (T.arcs τ) ℓ + posAt a (T.arcs (T.other a τ)) ℓ + ℓ a⟩

theorem evalMotion_crossForm (ℓ : α → K) (τ : Side Patch) (a : α) :
    evalMotion (T.crossForm τ a) ℓ = T.crossAt ℓ τ a := by
  unfold crossForm crossAt
  split_ifs
  · ext <;> simp [evalMotion, evalForm_sub, evalForm_posForm]
  · ext <;> simp [evalMotion, evalForm_add, evalForm_posForm, evalForm_single]

theorem crossForm_lin (τ : Side Patch) (a : α) : (T.crossForm τ a).lin = T.sign a := by
  unfold crossForm
  split_ifs with h
  · exact h.symm
  · rcases T.sign_isSign a with h' | h'
    · exact absurd h' h
    · exact h'.symm

/-! ### Paths of arcs and turns -/

/-- The side the walk enters after crossing the arcs `l` in turn from the side `σ`:
each time across the patch to the opposite side, then across the arc. -/
def endSide : Side Patch → List α → Side Patch
  | σ, [] => σ
  | σ, a :: l => endSide (T.other a σ.opp) l

/-- The arcs `l` can be crossed in turn from the side `σ`: each lies on the side
opposite to the side entered. -/
def IsPath : Side Patch → List α → Prop
  | _, [] => True
  | σ, a :: l => a ∈ T.arcs σ.opp ∧ IsPath (T.other a σ.opp) l

/-- The crossing maps along a path, the first applied first. -/
def crossings : Side Patch → List α → List (Motion ℤ (α → ℤ))
  | _, [] => []
  | σ, a :: l => T.crossForm σ.opp a :: crossings (T.other a σ.opp) l

/-- The crossing maps along a path at the lengths `ℓ`. -/
def crossingsAt (ℓ : α → K) : Side Patch → List α → List (Motion K K)
  | _, [] => []
  | σ, a :: l => T.crossAt ℓ σ.opp a :: crossingsAt ℓ (T.other a σ.opp) l

theorem crossingsAt_eq (ℓ : α → K) (σ : Side Patch) (l : List α) :
    T.crossingsAt ℓ σ l = (T.crossings σ l).map (evalMotion · ℓ) := by
  induction l generalizing σ with
  | nil => rfl
  | cons a l ih => simp [crossingsAt, crossings, ih, evalMotion_crossForm]

/-- **A turn**: arcs crossed in turn from a first side, back to it. -/
structure Turn (T : TMesh Patch α) where
  /-- The first side, entered by the walk. -/
  start : Side Patch
  /-- The arcs crossed, in order. -/
  path : List α
  isPath : T.IsPath start path
  closes : T.endSide start path = start

/-- **Oriented gluing.** The frames `(u, v)` of the patches are those of one
orientation of the surface. The boundaries of two patches then run along a shared
arc in opposite directions, so the sign of the arc is fixed by the sides it lies
on: `ε = -orient σ · orient σ'`. -/
def Oriented : Prop := ∀ a, T.sign a = -((T.ends a 0).orient * (T.ends a 1).orient)

/-- On an oriented T-mesh, crossing the patch from `σ` and then the arc `a` to the
side `σ'` has sign `orient σ · orient σ'`. -/
theorem sign_eq_of_oriented (hT : T.Oriented) {σ : Side Patch} {a : α}
    (ha : a ∈ T.arcs σ.opp) : T.sign a = σ.orient * (T.other a σ.opp).orient := by
  rw [hT a, other]
  rcases (T.mem_arcs a σ.opp).mp ha with h | h
  · rw [ite_eq_left h.symm, ← h, Side.orient_opp]; ring
  · rw [ite_eq_right (by rw [h]; exact T.ends_ne a), ← h, Side.orient_opp]; ring

/-- The composite of the crossings along a path from `σ` to `σ'` has sign
`orient σ · orient σ'`. -/
theorem composeAll_crossings_lin (hT : T.Oriented) (σ : Side Patch) (l : List α)
    (hl : T.IsPath σ l) :
    (Motion.composeAll (T.crossings σ l)).lin = σ.orient * (T.endSide σ l).orient := by
  induction l generalizing σ with
  | nil => simp [crossings, endSide, Side.orient_mul_self]
  | cons a l ih =>
    simp only [crossings, endSide, Motion.composeAll_cons, Motion.comp_lin]
    rw [ih _ hl.2, crossForm_lin, T.sign_eq_of_oriented hT hl.1]
    have := (T.other a σ.opp).orient_mul_self
    linear_combination σ.orient * (T.endSide (T.other a σ.opp) l).orient * this

namespace Turn

variable {T}

/-- The composite of the crossings of a turn, `x ↦ ε x + c` with `c` an integer
linear form in the lengths. -/
def composite (γ : T.Turn) : Motion ℤ (α → ℤ) := Motion.composeAll (T.crossings γ.start γ.path)

/-- **The shift of a turn**, as an integer linear form in the lengths. -/
def shiftForm (γ : T.Turn) : α → ℤ := γ.composite.trans

/-- The shift of a turn at the lengths `ℓ`. -/
def shift (γ : T.Turn) (ℓ : α → K) : K := evalForm γ.shiftForm ℓ

/-- On an oriented T-mesh the composite of the crossings of a turn keeps the
direction of the offsets: the walk re-enters its first side inward. -/
theorem composite_lin (hT : T.Oriented) (γ : T.Turn) : γ.composite.lin = 1 := by
  rw [composite, T.composeAll_crossings_lin hT _ _ γ.isPath, γ.closes, Side.orient_mul_self]

/-- **Proposition "The shift of a turn is linear".** On an oriented T-mesh, the
composite of the crossings of a turn at the lengths `ℓ` is the translation by
`shift γ ℓ = ∑ₐ cₐ ℓₐ`, with integer coefficients `c = shiftForm γ` that do not
depend on `ℓ`. -/
theorem shift_linear (hT : T.Oriented) (γ : T.Turn) (ℓ : α → K) :
    Motion.composeAll (T.crossingsAt ℓ γ.start γ.path) = Motion.translation (γ.shift ℓ) ∧
      γ.shift ℓ = ∑ a, (γ.shiftForm a : K) * ℓ a := by
  refine ⟨?_, rfl⟩
  rw [crossingsAt_eq, ← evalMotion_composeAll]
  ext
  · have h := γ.composite_lin hT
    rw [composite] at h
    simp [evalMotion, h]
  · rfl

end Turn

/-! ### The walk at given lengths, and the corridor -/

/-- One move of the walk of Definition "Turn" at the lengths `ℓ`: from the side `σ`
at the offset `x`, across the patch to the opposite side at the same offset, then
across the arc whose interior holds `x` there. `none` if `x` is a node there or
off the side. -/
def step (ℓ : α → K) (p : Side Patch × K) : Option (Side Patch × K) :=
  (arcAt ℓ (T.arcs p.1.opp) p.2).map fun a => (T.other a p.1.opp, (T.crossAt ℓ p.1.opp a).apply p.2)

/-- `n` moves of the walk. -/
def walk (ℓ : α → K) : ℕ → Side Patch × K → Option (Side Patch × K)
  | 0, p => some p
  | n + 1, p => (T.step ℓ p).bind (walk ℓ n)

theorem walk_add (ℓ : α → K) (n m : ℕ) (p : Side Patch × K) :
    T.walk ℓ (n + m) p = (T.walk ℓ n p).bind (T.walk ℓ m) := by
  induction n generalizing p with
  | zero => simp [walk]
  | succ n ih =>
    rw [Nat.succ_add, walk, walk, Option.bind_assoc]
    congr 1
    funext q
    exact ih q

/-- The walk from the offset `x` of the side `σ` crosses the arcs `l` in turn. -/
def Follows (ℓ : α → K) : Side Patch → List α → K → Prop
  | _, [], _ => True
  | σ, a :: l, x => arcAt ℓ (T.arcs σ.opp) x = some a ∧
      Follows ℓ (T.other a σ.opp) l ((T.crossAt ℓ σ.opp a).apply x)

/-- **The walk follows the composite of the crossings**: from an offset whose walk
crosses the arcs `l`, it reaches the end side at the image of the offset by the
composite. -/
theorem walk_of_follows (ℓ : α → K) (σ : Side Patch) (l : List α) (x : K)
    (h : T.Follows ℓ σ l x) :
    T.walk ℓ l.length (σ, x) =
      some (T.endSide σ l, (Motion.composeAll (T.crossingsAt ℓ σ l)).apply x) := by
  induction l generalizing σ x with
  | nil => simp [walk, endSide, crossingsAt, Motion.apply, Motion.id]
  | cons a l ih =>
    obtain ⟨ha, hl⟩ := h
    simp only [List.length_cons, walk, step, ha, Option.map_some, Option.bind_some, endSide,
      crossingsAt, Motion.composeAll_cons, Motion.apply_comp]
    exact ih _ _ hl

/-- The inequalities of the corridor along a path, the offset having gone through
the motion `g` so far: each `(e, f)` stands for `0 < e x + f(ℓ)`, two per arc
(the offset is past the start of the arc and before its end). -/
def corridorIneqs : Side Patch → List α → Motion ℤ (α → ℤ) → List (ℤ × (α → ℤ))
  | _, [], _ => []
  | σ, a :: l, g =>
    (g.lin, g.trans - posForm a (T.arcs σ.opp)) ::
      (-g.lin, posForm a (T.arcs σ.opp) + Pi.single a 1 - g.trans) ::
        corridorIneqs (T.other a σ.opp) l (Motion.comp (T.crossForm σ.opp a) g)

/-- At lengths `ℓ ≥ 0`, the walk from `g(x)` follows a path exactly when `x` meets
the inequalities of the corridor. -/
theorem follows_iff {ℓ : α → K} (hℓ : ∀ a, 0 ≤ ℓ a) (σ : Side Patch) (l : List α)
    (hl : T.IsPath σ l) (g : Motion ℤ (α → ℤ)) (x : K) :
    T.Follows ℓ σ l ((evalMotion g ℓ).apply x) ↔
      ∀ q ∈ T.corridorIneqs σ l g, 0 < (q.1 : K) * x + evalForm q.2 ℓ := by
  induction l generalizing σ g with
  | nil => simp [Follows, corridorIneqs]
  | cons a l ih =>
    obtain ⟨ha, hl⟩ := hl
    have hcross : (T.crossAt ℓ σ.opp a).apply ((evalMotion g ℓ).apply x) =
        (evalMotion (Motion.comp (T.crossForm σ.opp a) g) ℓ).apply x := by
      rw [evalMotion_comp, Motion.apply_comp, evalMotion_crossForm]
    simp only [Follows, corridorIneqs, List.forall_mem_cons]
    rw [hcross, ih _ hl, arcAt_eq_some hℓ (T.arcs_nodup _)]
    simp only [ha, true_and, evalMotion_apply, evalForm_sub, evalForm_add, evalForm_posForm,
      evalForm_single, Int.cast_neg, ← and_assoc]
    constructor <;> rintro ⟨⟨h₁, h₂⟩, h₃⟩ <;> exact ⟨⟨by linarith, by linarith⟩, h₃⟩

namespace Turn

variable {T}

/-- **The corridor** of a turn at the lengths `ℓ`: the offsets of its first side from
which the walk crosses the arcs of the turn. -/
def corridor (γ : T.Turn) (ℓ : α → K) : Set K := {x | T.Follows ℓ γ.start γ.path x}

/-- The inequalities of the corridor of a turn. -/
def corridorIneqs (γ : T.Turn) : List (ℤ × (α → ℤ)) := T.corridorIneqs γ.start γ.path Motion.id

/-- **The corridor is cut out by linear inequalities**: at lengths `ℓ ≥ 0`, `x` lies
in it exactly when `0 < e x + f(ℓ)` for every `(e, f)` of a finite list fixed by
the turn, with `e = ±1` and `f` an integer linear form. -/
theorem mem_corridor_iff {ℓ : α → K} (hℓ : ∀ a, 0 ≤ ℓ a) (γ : T.Turn) (x : K) :
    x ∈ γ.corridor ℓ ↔ ∀ q ∈ γ.corridorIneqs, 0 < (q.1 : K) * x + evalForm q.2 ℓ := by
  have h := T.follows_iff hℓ γ.start γ.path γ.isPath Motion.id x
  simp only [evalMotion_apply, Motion.id_lin, Motion.id_trans, Int.cast_one, one_mul,
    evalForm_zero, add_zero] at h
  exact h

/-- **The walk comes back shifted** (Proposition "The shift of a turn is linear",
second part): on an oriented T-mesh, from an offset `o` of the corridor the walk
comes back to the first side at `o + shift γ ℓ`. -/
theorem comes_back (hT : T.Oriented) (γ : T.Turn) {ℓ : α → K} {o : K} (ho : o ∈ γ.corridor ℓ) :
    T.walk ℓ γ.path.length (γ.start, o) = some (γ.start, o + γ.shift ℓ) := by
  rw [T.walk_of_follows ℓ _ _ _ ho, γ.closes, (γ.shift_linear hT ℓ).1,
    Motion.apply_translation]

/-- It closes after one round exactly when the shift is zero. -/
theorem comes_back_iff (hT : T.Oriented) (γ : T.Turn) {ℓ : α → K} {o : K}
    (ho : o ∈ γ.corridor ℓ) :
    T.walk ℓ γ.path.length (γ.start, o) = some (γ.start, o) ↔ γ.shift ℓ = 0 := by
  rw [γ.comes_back hT ho]
  simp

/-- **Helices**: if `o + j shift γ ℓ` lies in the corridor for `j < t`, the walk
from `o` winds `t` times round and comes back at `o + t shift γ ℓ`. -/
theorem winds (hT : T.Oriented) (γ : T.Turn) {ℓ : α → K} {o : K} (t : ℕ)
    (ho : ∀ j < t, o + j * γ.shift ℓ ∈ γ.corridor ℓ) :
    T.walk ℓ (t * γ.path.length) (γ.start, o) = some (γ.start, o + t * γ.shift ℓ) := by
  induction t with
  | zero => simp [walk]
  | succ t ih =>
    rw [Nat.succ_mul, walk_add, ih fun j hj => ho j (by omega), Option.bind_some,
      γ.comes_back hT (ho t (by omega))]
    push_cast
    ring_nf

end Turn

/-! ### Wiring: forced shifts and Farkas certificates on a T-mesh -/

section Wiring

variable [Fintype Patch] {ι : Type*} [DecidableEq ι] (turn : ι → T.Turn)

/-- `P`: the balanced real lengths above the bounds of separation `b`. -/
def bounded (b : α → ℝ) : Set (α → ℝ) :=
  {ℓ | T.balance.map (Int.cast : ℤ → ℝ) *ᵥ ℓ = 0 ∧ ∀ a, b a ≤ ℓ a}

/-- The shifts of a family of turns, as functions of the real lengths. -/
noncomputable def turnShift (γ : ι) (ℓ : α → ℝ) : ℝ := (turn γ).shift ℓ

/-- The matrix `H` of the shift forms of the turns of `S`. -/
noncomputable def shiftMatrix (S : Finset ι) : Matrix S α ℝ := fun γ a => ((turn γ).shiftForm a : ℝ)

/-- The turns of `S` close together within `P` exactly when the linear system
`B ℓ = 0`, `H ℓ = 0`, `ℓ ≥ b` has a solution: the shifts are linear forms. -/
theorem closable_iff (b : α → ℝ) (S : Finset ι) :
    Closable (turnShift T turn) (T.bounded b) S ↔
      ∃ ℓ : α → ℝ, T.balance.map (Int.cast : ℤ → ℝ) *ᵥ ℓ = 0 ∧
        T.shiftMatrix turn S *ᵥ ℓ = 0 ∧ ∀ a, b a ≤ ℓ a := by
  constructor
  · rintro ⟨ℓ, ⟨hB, hb⟩, hS⟩
    refine ⟨ℓ, hB, funext fun γ => ?_, hb⟩
    have := hS γ γ.2
    simpa [turnShift, Turn.shift, evalForm, shiftMatrix, Matrix.mulVec, dotProduct] using this
  · rintro ⟨ℓ, hB, hH, hb⟩
    refine ⟨ℓ, ⟨hB, hb⟩, fun γ hγ => ?_⟩
    have := congrFun hH ⟨γ, hγ⟩
    simpa [turnShift, Turn.shift, evalForm, shiftMatrix, Matrix.mulVec, dotProduct] using this

/-- **Remark "Sign-definite turns" on a T-mesh**: the turns of `S` cannot be closed
together within `P` exactly when a combination of their shift forms equals, modulo
the balances of the T-mesh, a form `c ≥ 0` with `c ⬝ b > 0`. -/
theorem not_closable_iff (b : α → ℝ) (S : Finset ι) :
    ¬ Closable (turnShift T turn) (T.bounded b) S ↔
      ∃ (u : S → ℝ) (lam : Patch × Fin 2 → ℝ),
        (∀ a, 0 ≤ (u ᵥ* T.shiftMatrix turn S + lam ᵥ* T.balance.map (Int.cast : ℤ → ℝ)) a) ∧
          0 < (u ᵥ* T.shiftMatrix turn S + lam ᵥ* T.balance.map (Int.cast : ℤ → ℝ)) ⬝ᵥ b := by
  rw [closable_iff]
  exact Farkas.sign_definite_certificate _ _ b

/-- **Theorem "Forced shifts" on a T-mesh, from Farkas certificates.** If `H`
contains `k` pairwise disjoint sets of turns, each with a certificate of Remark
"Sign-definite turns", then every balanced `ℓ ≥ b` leaves at least `k` turns of
`H` shifted. -/
theorem forced_of_certificates (b : α → ℝ) (H : Finset ι) {k : ℕ} (S : Fin k → Finset ι)
    (hS : ∀ j, S j ⊆ H) (hdisj : ∀ j j', j ≠ j' → Disjoint (S j) (S j'))
    (hcert : ∀ j, ∃ (u : S j → ℝ) (lam : Patch × Fin 2 → ℝ),
      (∀ a, 0 ≤ (u ᵥ* T.shiftMatrix turn (S j) + lam ᵥ* T.balance.map (Int.cast : ℤ → ℝ)) a) ∧
        0 < (u ᵥ* T.shiftMatrix turn (S j) + lam ᵥ* T.balance.map (Int.cast : ℤ → ℝ)) ⬝ᵥ b)
    (ℓ : α → ℝ) (hℓ : ℓ ∈ T.bounded b) :
    k ≤ (shifted (turnShift T turn) H ℓ).card :=
  (tauP_ge_of_disjoint _ _ H ⟨ℓ, hℓ⟩ S hS hdisj
    fun j => (T.not_closable_iff turn b (S j)).mpr (hcert j)).trans
    (forced_helices _ _ H ℓ hℓ)

end Wiring

/-! ### An example: a torus of one patch, glued with a twist

The bottom side holds the arcs `c₁, c₂`, the top side `c₂, c₁`, both glued in the
same direction; the left and right sides hold one arc `e`. The vertical turn
crossing `c₂` comes back shifted by `ℓ_{c₁}`, from the offsets `0 < x < ℓ_{c₂}`. -/

/-- The twisted torus: arcs `0 = e`, `1 = c₁`, `2 = c₂`. -/
def twistedTorus : TMesh Unit (Fin 3) where
  arcs σ := if σ.2.1 = 0 then (if σ.2.2 = 0 then [1, 2] else [2, 1]) else [0]
  arcs_nodup := by decide
  ends a k := if a = 0 then ((), 1, k) else ((), 0, k)
  ends_ne := by decide
  mem_arcs := by decide
  sign _ := 1
  sign_isSign _ := Or.inl rfl

theorem twistedTorus_oriented : twistedTorus.Oriented := by unfold Oriented; decide

/-- The vertical turn of the twisted torus, from the bottom side across `c₂`. -/
def twistedTorus.turn : twistedTorus.Turn := ⟨((), 0, 0), [2], ⟨by decide, trivial⟩, by decide⟩

/-- Its shift is `ℓ_{c₁}`. -/
theorem twistedTorus.shiftForm_turn : twistedTorus.turn.shiftForm = Pi.single 1 1 := by
  funext a; fin_cases a <;> rfl

/-- From the offset `x` of the bottom side, `0 < x < ℓ_{c₂}`, the walk comes back to
the bottom side at `x + ℓ_{c₁}`. -/
theorem twistedTorus.walk_turn {K : Type*} [Field K] [LinearOrder K] [IsStrictOrderedRing K]
    (ℓ : Fin 3 → K) {x : K} (h₀ : 0 < x) (h₁ : x < ℓ 2) :
    twistedTorus.walk ℓ 1 (((), 0, 0), x) = some (((), 0, 0), x + ℓ 1) := by
  have hx : x ∈ twistedTorus.turn.corridor ℓ := by
    simp [Turn.corridor, Follows, twistedTorus.turn, twistedTorus, Side.opp, arcAt, h₀, h₁]
  have h := twistedTorus.turn.comes_back twistedTorus_oriented hx
  rwa [Turn.shift, twistedTorus.shiftForm_turn, evalForm_single] at h

end TMesh

end TurnsOfATMesh
