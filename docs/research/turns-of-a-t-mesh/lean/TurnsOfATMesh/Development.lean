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

end TMesh

end TurnsOfATMesh
