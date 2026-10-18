/-
The balances of the patches: Lemma "Handshake".

The matrix `B` of the balances has one column per arc. When each column sums to
an even number (each arc lies on a side of the two patches it borders, so its
column has two entries `±1`, or cancels), the sum of the entries of `B x` is
even for every whole `x`.
-/
module

public import Mathlib.Data.Matrix.Mul
public import Mathlib.Algebra.Group.Int.Even
public import Mathlib.Algebra.BigOperators.Ring.Finset
public import Mathlib.Algebra.Ring.Parity
public import Mathlib.Algebra.Ring.Int.Parity

@[expose] public section

namespace ClosingHelices

variable {ρ α : Type*} [Fintype ρ] [Fintype α]

/-- **Lemma "Handshake".** If every column of `B` has an even sum, then
`∑_r (B x)_r` is even for every whole `x`. -/
theorem handshake (B : Matrix ρ α ℤ) (hcol : ∀ a, Even (∑ r, B r a)) (x : α → ℤ) :
    Even (∑ r, (B.mulVec x) r) := by
  simp only [Matrix.mulVec, dotProduct]
  rw [Finset.sum_comm]
  refine Finset.even_sum _ fun a _ => ?_
  rw [← Finset.sum_mul]
  exact (hcol a).mul_right _

/-- A column with exactly two non-zero entries, each `±1`, has an even sum. -/
theorem column_even (c : ρ → ℤ) (r₁ r₂ : ρ) (hne : r₁ ≠ r₂)
    (hzero : ∀ r, r ≠ r₁ → r ≠ r₂ → c r = 0)
    (h₁ : c r₁ = 1 ∨ c r₁ = -1) (h₂ : c r₂ = 1 ∨ c r₂ = -1) : Even (∑ r, c r) := by
  rw [Finset.sum_eq_add_of_mem r₁ r₂ (Finset.mem_univ _) (Finset.mem_univ _) hne
    (fun r _ h => hzero r h.1 h.2)]
  rcases h₁ with h₁ | h₁ <;> rcases h₂ with h₂ | h₂ <;> rw [h₁, h₂] <;> decide

/-- Hence a system `B x = d` with `∑_r d_r` odd has real solutions perhaps, but no
whole one. -/
theorem no_whole_solution (B : Matrix ρ α ℤ) (hcol : ∀ a, Even (∑ r, B r a)) (d : ρ → ℤ)
    (hd : Odd (∑ r, d r)) : ¬ ∃ x : α → ℤ, B.mulVec x = d := by
  rintro ⟨x, hx⟩
  have := handshake B hcol x
  rw [hx] at this
  exact (Int.not_even_iff_odd.mpr hd) this

end ClosingHelices
