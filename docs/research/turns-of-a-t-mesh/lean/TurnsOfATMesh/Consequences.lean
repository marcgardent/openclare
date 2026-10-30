/-
Corollary "Parity", Proposition "Whole lengths and size without a cap" and
Lemma "Combs": one-line consequences of statements checked elsewhere, stated
on their conclusions. The parity corollary reads `α ≡ s (mod 2)`, the
conclusion of `parity_of_turn_any_sign`; the size statement scales a real
solution of a homogeneous system with lower bounds, the whole case being
`whole_solution_of_real`; the comb identity is the sum of steps orthogonal to
`e_a` but one, of length `ℓ_a` along it.
-/
module

public import TurnsOfATMesh.Fox
public import TurnsOfATMesh.Farkas
public import Mathlib.Analysis.Real.Sqrt
public import Mathlib.Data.ZMod.Basic
public import Mathlib.Tactic.Linarith

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

open Matrix

/-! ### Corollary "Parity" -/

section Parity

/-- `α ≡ s (mod 2)` as the conclusion of Proposition "Parity of a turn on a
sphere" (`parity_of_turn_any_sign`), read as residues. -/
theorem parity_mod (α s : ℤ) (h : (α : ZMod 2) = s) : α % 2 = s % 2 :=
  (ZMod.intCast_eq_intCast_iff' α s 2).mp h

/-- **Corollary "Parity" (a).** A ring (shift zero) has an even length. -/
theorem ring_even (α s : ℤ) (h : (α : ZMod 2) = s) (hs : s = 0) : Even α := by
  have := parity_mod α s h
  rw [Int.even_iff]; omega

/-- **Corollary "Parity" (b).** A helix of odd pitch winds round an odd number of
quads per turn. -/
theorem odd_pitch_odd_length (α s : ℤ) (h : (α : ZMod 2) = s) (hs : Odd s) : Odd α := by
  have := parity_mod α s h
  rw [Int.odd_iff] at hs ⊢; omega

/-- **Corollary "Parity" (c), first half.** A repair that keeps the length of a
turn of odd shift cannot close it: at the lengths after the repair, `α' = α`
and `α' ≡ s'`, so `s'` is odd, not zero. -/
theorem repair_keeps_length_cannot_close (α s α' s' : ℤ) (h : (α : ZMod 2) = s)
    (h' : (α' : ZMod 2) = s') (hs : Odd s) (hlen : α' = α) : s' ≠ 0 := by
  have h1 := parity_mod α s h
  have h2 := parity_mod α' s' h'
  rw [Int.odd_iff] at hs; omega

/-- **Corollary "Parity" (c), second half.** Changing the pitch by an odd amount
changes the length. -/
theorem odd_change_of_pitch_changes_length (α s α' s' : ℤ) (h : (α : ZMod 2) = s)
    (h' : (α' : ZMod 2) = s') (hodd : Odd (s' - s)) : α' ≠ α := by
  have h1 := parity_mod α s h
  have h2 := parity_mod α' s' h'
  rw [Int.odd_iff] at hodd; omega

end Parity

/-! ### Proposition "Whole lengths and size without a cap" -/

section Size

variable {α κ : Type*} [Fintype α]

/-- Scaling keeps a homogeneous system with non-negative lower bounds: `c ℓ`
balances and closes what `ℓ` does, and stays above the bounds for `c ≥ 1`. -/
theorem smul_mem_system (M : Matrix κ α ℝ) (b : α → ℝ) (hb : ∀ a, 0 ≤ b a) (ℓ : α → ℝ)
    (hM : M *ᵥ ℓ = 0) (hℓ : ∀ a, b a ≤ ℓ a) (c : ℝ) (hc : 1 ≤ c) :
    M *ᵥ (c • ℓ) = 0 ∧ ∀ a, b a ≤ (c • ℓ) a := by
  refine ⟨by rw [Matrix.mulVec_smul, hM, smul_zero], fun a => ?_⟩
  rw [Pi.smul_apply, smul_eq_mul]
  calc b a ≤ ℓ a := hℓ a
    _ ≤ c * ℓ a := le_mul_of_one_le_left (le_trans (hb a) (hℓ a)) hc

/-- **Proposition "Whole lengths and size without a cap", the sizes.** The number
of quads `Q` is homogeneous of degree two in the lengths (a sum of products of
two side lengths). A set of turns closable in real lengths within the balances,
their closing forms (the rows of `M`) and bounds `ℓ ≥ b ≥ 0` is closable at
every size `q ≥ Q(ℓ)`: `ℓ` scaled by `√(q / Q(ℓ))`. -/
theorem closable_at_any_size (M : Matrix κ α ℝ) (b : α → ℝ) (hb : ∀ a, 0 ≤ b a)
    (Q : (α → ℝ) → ℝ) (hQ : ∀ (c : ℝ) (ℓ : α → ℝ), Q (c • ℓ) = c ^ 2 * Q ℓ)
    (ℓ : α → ℝ) (hM : M *ᵥ ℓ = 0) (hℓ : ∀ a, b a ≤ ℓ a) (hpos : 0 < Q ℓ) (q : ℝ) (hq : Q ℓ ≤ q) :
    ∃ ℓ' : α → ℝ, M *ᵥ ℓ' = 0 ∧ (∀ a, b a ≤ ℓ' a) ∧ Q ℓ' = q := by
  have hdiv : 0 ≤ q / Q ℓ := div_nonneg (le_trans hpos.le hq) hpos.le
  have hc : 1 ≤ Real.sqrt (q / Q ℓ) := by
    rw [← Real.sqrt_one]
    exact Real.sqrt_le_sqrt ((one_le_div hpos).mpr hq)
  obtain ⟨h1, h2⟩ := smul_mem_system M b hb ℓ hM hℓ _ hc
  refine ⟨_, h1, h2, ?_⟩
  rw [hQ, Real.sq_sqrt hdiv, div_mul_cancel₀ q hpos.ne']

end Size

/-! ### Lemma "Combs" -/

section Combs

open Complex ComplexConjugate

/-- `⟨e, ·⟩` is additive. -/
theorem dotC_sum {n : ℕ} (e : ℂ) (v : Fin n → ℂ) : dotC e (∑ i, v i) = ∑ i, dotC e (v i) := by
  simp [dotC, Finset.mul_sum, Complex.re_sum]

/-- **Lemma "Combs", the identity.** Developed along the comb, every step but the
one along the arc is orthogonal to `e_a` (it runs along a trace perpendicular
to the arc, through no cone point), and that one is `ℓ_a e_a`; so the
developed feet differ by `ℓ_a` along `e_a`. The content of the lemma is in
its hypotheses: that the feet exist and the perpendicular traces are straight. -/
theorem comb_reading {n : ℕ} (e : ℂ) (he : normSq e = 1) (v : Fin n → ℂ) (k : Fin n) (ℓ : ℝ)
    (hk : v k = ℓ * e) (horth : ∀ i, i ≠ k → dotC e (v i) = 0) :
    dotC e (∑ i, v i) = ℓ := by
  rw [dotC_sum, Finset.sum_eq_single k (fun i _ hi => horth i hi) (by simp), hk, dotC,
    mul_left_comm, ← Complex.normSq_eq_conj_mul_self, he]
  simp

end Combs

end TurnsOfATMesh
