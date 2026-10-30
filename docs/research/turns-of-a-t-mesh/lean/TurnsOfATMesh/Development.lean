/-
The development of a turn is a translation (DT-134, third brick): on an oriented
T-mesh at whole lengths `ℓ`, the composite of the changes of chart along a turn,
`Turn.development` of `Holonomy.lean`, is the translation by
`-α_γ(ℓ) e₁ + shift_γ(ℓ) e₂` in the frame `(e₁, e₂) = (inward, acrossDir)` of its
first side (`Turn.development_eq`), Section "Flat metric and holonomy" of the note.

The proof follows the frame of the side entered along the path. Across a patch the
first corner moves by the length across (`origin_opp`); across an arc the change
of chart takes the inward direction to the inward direction and, on an oriented
T-mesh, the direction along the side to `sign a` times the direction along the
other side (`crossLin_acrossDir`), the offsets crossing as `crossAt` says
(`crossTrans_eq`). Along a path the composite takes the frame of the first side to
that of the side entered last, the first corner going to the offset the crossings
of `TMesh.lean` compose to, minus the length of the path inward
(`development_apply_origin`).
-/
module

public import TurnsOfATMesh.Holonomy
public import TurnsOfATMesh.Bipartite

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### The frame across a patch -/

/-- The length of a list of arcs, as an integer form evaluated at whole lengths. -/
theorem evalForm_lengthForm_nat [Fintype α] (l : List α) (ℓ : α → ℕ) :
    evalForm (lengthForm l) (fun a => (ℓ a : ℤ)) = ((l.map ℓ).sum : ℕ) := by
  rw [evalForm_lengthForm_ring, Nat.cast_list_sum, List.map_map]
  rfl

/-- **Across a patch** the first corner of the opposite side is the first corner of
the side moved inward by the length across. -/
theorem origin_opp [Fintype α] (ℓ : α → ℕ) (σ : Side Patch) :
    T.origin ℓ σ.opp =
      T.origin ℓ σ + ((evalForm (T.acrossForm σ) (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) * inward σ := by
  have hc : ∀ n : ℕ, ((n : ℤ) : GaussianInt) = ⟨n, 0⟩ := fun n => by ext <;> simp
  obtain ⟨P, j, s⟩ := σ
  fin_cases j <;> fin_cases s <;>
    simp only [origin, inward, acrossForm, Side.opp, evalForm_lengthForm_nat, height, width,
      sideLen, hc] <;> ext <;> simp [gi]

/-! ### The linear part of a change of chart -/

theorem crossLin_isQuarter (τ τ' : Side Patch) : IsQuarter (crossLin τ τ') := by
  rcases isQuarter_inward τ with h | h | h | h <;>
  rcases isQuarter_inward τ' with h' | h' | h' | h' <;>
    simp only [crossLin, h, h'] <;> unfold IsQuarter <;> simp [conj, gi] <;> decide

/-- The linear part of the change of chart from `τ` to `τ'` takes the direction
along `τ` to `-orient τ · orient τ'` times the direction along `τ'`. -/
theorem crossLin_acrossDir_orient (τ τ' : Side Patch) :
    crossLin τ τ' * acrossDir τ = ((-(τ.orient * τ'.orient) : ℤ) : GaussianInt) * acrossDir τ' := by
  obtain ⟨P, j, s⟩ := τ
  obtain ⟨Q, j', s'⟩ := τ'
  fin_cases j <;> fin_cases s <;> fin_cases j' <;> fin_cases s' <;>
    simp [crossLin, inward, acrossDir, conj, Side.orient] <;> ext <;> simp [gi]

/-- **The geometric content of `Oriented`**: across an arc `a` of the side `τ`, the
linear part of the change of chart takes the direction along `τ` to `sign a` times
the direction along the other side. -/
theorem crossLin_acrossDir [Fintype α] (hT : T.Oriented) {τ : Side Patch} {a : α} (ha : a ∈ T.arcs τ) :
    crossLin τ (T.other a τ) * acrossDir τ = (T.sign a : GaussianInt) * acrossDir (T.other a τ) := by
  have hopp : τ.opp.opp = τ := by
    obtain ⟨P, j, s⟩ := τ
    simp only [Side.opp, Prod.mk.injEq, true_and]
    fin_cases s <;> decide
  have h := T.sign_eq_of_oriented hT (σ := τ.opp) (a := a) (by rw [hopp]; exact ha)
  rw [hopp, Side.orient_opp] at h
  rw [crossLin_acrossDir_orient, h, neg_mul]

/-! ### The change of chart on the side -/

theorem crossMotion_lin (ℓ : α → ℕ) (τ : Side Patch) (a : α) :
    (T.crossMotion ℓ τ a).lin = crossLin τ (T.other a τ) := rfl

theorem apply_eq (f : Motion GaussianInt GaussianInt) (x : GaussianInt) :
    f.apply x = f.lin * x + f.trans := rfl

/-- **Across an arc on the side**: the point of `τ` at the offset `x` goes to the
point of the other side at the offset `sign a · x + crossTrans`. -/
theorem crossMotion_side [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ) (x : ℤ) :
    (T.crossMotion ℓ τ a).apply (T.origin ℓ τ + x * acrossDir τ) =
      T.origin ℓ (T.other a τ) +
        ((T.sign a * x + T.crossTrans ℓ τ a : ℤ) : GaussianInt) * acrossDir (T.other a τ) := by
  have h := T.crossLin_acrossDir hT ha
  rw [apply_eq, crossMotion_lin]
  simp only [crossMotion]
  push_cast
  linear_combination (x : GaussianInt) * h

/-- The position on a side at whole lengths is the form `posForm`. -/
theorem evalForm_posForm_nat [Fintype α] (a : α) (l : List α) (ℓ : α → ℕ) :
    evalForm (posForm a l) (fun b => (ℓ b : ℤ)) = posNat a l ℓ := by
  induction l with
  | nil => simp [posForm, posNat]
  | cons b l ih =>
    simp only [posForm, posNat]
    split_ifs
    · simp
    · rw [evalForm_add, evalForm_single, ih]; push_cast; rfl

/-- **The translation of the change of chart is that of the crossing of the offsets**:
`crossTrans` is the form of `crossForm` at whole lengths. -/
theorem crossTrans_eq [Fintype α] (ℓ : α → ℕ) (τ : Side Patch) (a : α) :
    T.crossTrans ℓ τ a = evalForm (T.crossForm τ a).trans (fun b => (ℓ b : ℤ)) := by
  unfold crossTrans crossForm
  split_ifs
  · simp [evalForm_sub, evalForm_posForm_nat]
  · simp [evalForm_add, evalForm_posForm_nat, evalForm_single]

/-- The same, read in the rationals of the walk: `crossTrans` is the translation of
`crossAt` at `rat ℓ`. -/
theorem crossTrans_rat [Fintype α] (ℓ : α → ℕ) (τ : Side Patch) (a : α) :
    (T.crossTrans ℓ τ a : ℚ) = (T.crossAt (rat ℓ) τ a).trans := by
  unfold crossTrans crossAt
  split_ifs
  · simp [posAt_rat]
  · simp [posAt_rat, rat]

/-! ### Along a path -/

/-- Along a path the composite of the changes of chart takes the inward direction of
the first side to that of the side entered last. -/
theorem composeAll_crossMotions_inward (ℓ : α → ℕ) (σ : Side Patch) (l : List α) :
    (Motion.composeAll (T.crossMotions ℓ σ l)).lin * inward σ = inward (T.endSide σ l) := by
  induction l generalizing σ with
  | nil => simp [crossMotions, endSide]
  | cons a l ih =>
    simp only [crossMotions, endSide, Motion.composeAll_cons, Motion.comp_lin, crossMotion_lin]
    have h := crossLin_inward σ.opp (T.other a σ.opp)
    rw [inward_opp] at h
    rw [← ih, mul_assoc]
    linear_combination (Motion.composeAll (T.crossMotions ℓ (T.other a σ.opp) l)).lin * (-1) * h

/-- Along a path the composite of the changes of chart takes the direction along the
first side to the sign of the crossings of the offsets times the direction along the
side entered last. -/
theorem composeAll_crossMotions_acrossDir [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (σ : Side Patch) (l : List α) (hl : T.IsPath σ l) :
    (Motion.composeAll (T.crossMotions ℓ σ l)).lin * acrossDir σ =
      ((Motion.composeAll (T.crossings σ l)).lin : GaussianInt) * acrossDir (T.endSide σ l) := by
  induction l generalizing σ with
  | nil => simp [crossMotions, crossings, endSide]
  | cons a l ih =>
    simp only [crossMotions, crossings, endSide, Motion.composeAll_cons, Motion.comp_lin,
      crossMotion_lin, crossForm_lin]
    have h := T.crossLin_acrossDir hT hl.1
    rw [acrossDir_opp] at h
    have ih' := ih _ hl.2
    push_cast
    linear_combination (Motion.composeAll (T.crossMotions ℓ (T.other a σ.opp) l)).lin * h +
      (T.sign a : GaussianInt) * ih'

/-- **The invariant along a path**: the composite of the changes of chart takes the
first corner of the first side to the point of the side entered last at the offset
the crossings of the offsets compose to, moved back inward by the length of the
path. -/
theorem development_apply_origin [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) (σ : Side Patch)
    (l : List α) (hl : T.IsPath σ l) :
    (Motion.composeAll (T.crossMotions ℓ σ l)).apply (T.origin ℓ σ) =
      T.origin ℓ (T.endSide σ l) +
        ((evalForm (Motion.composeAll (T.crossings σ l)).trans (fun a => (ℓ a : ℤ)) : ℤ) :
          GaussianInt) * acrossDir (T.endSide σ l) -
        ((evalForm (T.pathLengthForm σ l) (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) *
          inward (T.endSide σ l) := by
  induction l generalizing σ with
  | nil => simp [crossMotions, crossings, endSide, pathLengthForm, apply_eq]
  | cons a l ih =>
    set τ' := T.other a σ.opp
    have ih' := ih τ' hl.2
    have I1 := T.composeAll_crossMotions_inward ℓ τ' l
    have I2 := T.composeAll_crossMotions_acrossDir hT ℓ τ' l hl.2
    have hL := crossLin_inward σ.opp τ'
    rw [inward_opp] at hL
    have ho := T.origin_opp ℓ σ
    rw [apply_eq] at ih' ⊢
    simp only [crossMotions, crossings, endSide, pathLengthForm, Motion.composeAll_cons,
      Motion.comp_lin, Motion.comp_trans, crossMotion_lin, evalForm_add, evalForm_smul,
      ← crossTrans_eq, smul_eq_mul]
    simp only [crossMotion]
    push_cast
    set K := (Motion.composeAll (T.crossMotions ℓ τ' l)).lin
    set L := crossLin σ.opp τ'
    linear_combination ih' + (T.crossTrans ℓ σ.opp a : GaussianInt) * I2 - K * L * ho +
      ((evalForm (T.acrossForm σ) (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) * K * hL -
      ((evalForm (T.acrossForm σ) (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) * I1

/-! ### The development of a turn -/

namespace Turn

variable {T}

/-- The development of a turn keeps the directions: its linear part is `1`. -/
theorem development_lin (γ : T.Turn) (ℓ : α → ℕ) : (γ.development ℓ).lin = 1 := by
  have h := T.composeAll_crossMotions_inward ℓ γ.start γ.path
  rw [γ.closes] at h
  have hq := mul_conj_of_isQuarter (isQuarter_inward γ.start)
  rw [development]
  linear_combination conj (inward γ.start) * h - ((Motion.composeAll
    (T.crossMotions ℓ γ.start γ.path)).lin - 1) * hq

/-- The translation of the development of a turn: `-α_γ(ℓ) e₁ + shift_γ(ℓ) e₂`. -/
theorem development_trans [Fintype α] (hT : T.Oriented) (γ : T.Turn) (ℓ : α → ℕ) :
    (γ.development ℓ).trans =
      -((evalForm γ.lengthForm (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) * inward γ.start +
        ((evalForm γ.shiftForm (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) * acrossDir γ.start := by
  have h := T.development_apply_origin hT ℓ γ.start γ.path γ.isPath
  have hlin := γ.development_lin ℓ
  rw [development] at hlin
  rw [apply_eq, hlin, γ.closes] at h
  rw [development, lengthForm, shiftForm, composite]
  linear_combination h

/-- **The development of a turn is a translation**: on an oriented T-mesh at whole
lengths `ℓ`, the composite of the changes of chart along a turn is the translation
by `-α_γ(ℓ) e₁ + shift_γ(ℓ) e₂`, `e₁ = inward`, `e₂ = acrossDir` of its first side. -/
theorem development_eq [Fintype α] (hT : T.Oriented) (γ : T.Turn) (ℓ : α → ℕ) :
    γ.development ℓ = Motion.translation
      (-((evalForm γ.lengthForm (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) * inward γ.start +
        ((evalForm γ.shiftForm (fun a => (ℓ a : ℤ)) : ℤ) : GaussianInt) * acrossDir γ.start) := by
  ext1
  · exact γ.development_lin ℓ
  · exact γ.development_trans hT ℓ

end Turn

end TMesh

end TurnsOfATMesh
