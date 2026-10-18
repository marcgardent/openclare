/-
Beyond quads: the statements for a field of any order `n`.

A field of order `n` (4 for quads, 6 for triangle and hexagonal meshes, 3 for the
order-three fields of hexagon-dominant meshes) turns the chart by a power of the
rotation `ζₙ = exp(2πi/n)` round each cone point. The Burgers sum, the order of first
visit, annuli, exchanges and the Fox–Burgers formula are proved for any ring
(`Motion.lean`, `FirstVisit.lean`, `Fox.lean`); the congruence modulo `(1 - z)²` in
`Congruence.lean`; discrete Gauss–Bonnet for triangles and hexagons in
`GaussBonnet.lean`. What is left, stated here:

* the rotation part of Gauss–Bonnet: `ζₙ^{j₁} ⋯ ζₙ^{jₘ} = 1` exactly when `n` divides
  `∑ jᵢ` (`rotations_prod_eq_one_iff`), so a curve whose holonomy is a
  translation encloses indices `n - k` adding up to a multiple of `n`;
* dislocations: the Burgers vector `(1 - ζₙ) d` of a dislocation has length
  `2 sin(π/n) ‖d‖` (`dislocation_norm`): `√2 ‖d‖` for quads, `√3 ‖d‖` for order 3,
  `‖d‖` for order 6 (a pair of valences five and seven on a triangle mesh).
-/
module

public import Mathlib.RingTheory.RootsOfUnity.Complex
public import Mathlib.Analysis.SpecialFunctions.Trigonometric.Basic
public import Mathlib.Analysis.Complex.Trigonometric

@[expose] public section

namespace ClosingHelices

namespace BeyondQuads

open Complex

/-- The rotation of order `n`, `ζₙ = exp(2πi/n)`. -/
noncomputable def zeta (n : ℕ) : ℂ := exp (2 * Real.pi * I / n)

/-- **Gauss–Bonnet, the rotation part, for a field of order `n`.** A product of
rotations `ζₙ^{j₁} ⋯ ζₙ^{jₘ}` is the identity exactly when `n` divides `∑ jᵢ`. -/
theorem rotations_prod_eq_one_iff (n : ℕ) (hn : n ≠ 0) (js : List ℤ) :
    (js.map fun j => zeta n ^ j).prod = 1 ↔ (n : ℤ) ∣ js.sum := by
  have hz : zeta n ≠ 0 := exp_ne_zero _
  have h : (js.map fun j => zeta n ^ j).prod = zeta n ^ js.sum := by
    induction js with
    | nil => simp
    | cons j js ih => simp [ih, zpow_add₀ hz]
  rw [h]
  exact (isPrimitiveRoot_exp n hn).zpow_eq_one_iff_dvd js.sum

/-- `|1 - ζₙ| = 2 sin(π/n)`. -/
theorem norm_one_sub_zeta (n : ℕ) (hn : n ≠ 0) : ‖1 - zeta n‖ = 2 * Real.sin (Real.pi / n) := by
  have hn' : (0 : ℝ) < n := by exact_mod_cast Nat.pos_of_ne_zero hn
  have h1 : zeta n = exp (I * ((2 * Real.pi / n : ℝ) : ℂ)) := by
    rw [zeta]; congr 1; push_cast; ring
  rw [norm_sub_rev, h1, norm_exp_I_mul_ofReal_sub_one]
  have h2 : 2 * Real.pi / n / 2 = Real.pi / n := by field_simp
  rw [h2, Real.norm_eq_abs, abs_of_nonneg]
  have : 0 ≤ Real.sin (Real.pi / n) := Real.sin_nonneg_of_nonneg_of_le_pi (by positivity)
    (div_le_self Real.pi_pos.le (by exact_mod_cast Nat.one_le_iff_ne_zero.mpr hn))
  positivity

/-- **Dislocations of order `n`.** The Burgers vector `(1 - ζₙ) d` of a pair of
rotations `ζₙ^{±1}` about two points `d` apart has length `2 sin(π/n) ‖d‖`. -/
theorem dislocation_norm (n : ℕ) (hn : n ≠ 0) (d : ℂ) :
    ‖(1 - zeta n) * d‖ = 2 * Real.sin (Real.pi / n) * ‖d‖ := by
  rw [norm_mul, norm_one_sub_zeta n hn]

/-- Quads: `‖B‖ = √2 ‖d‖`. -/
theorem dislocation_norm_four (d : ℂ) : ‖(1 - zeta 4) * d‖ = √2 * ‖d‖ := by
  rw [dislocation_norm 4 (by norm_num)]
  push_cast
  rw [Real.sin_pi_div_four]; ring

/-- Order 3: `‖B‖ = √3 ‖d‖`. -/
theorem dislocation_norm_three (d : ℂ) : ‖(1 - zeta 3) * d‖ = √3 * ‖d‖ := by
  rw [dislocation_norm 3 (by norm_num)]
  push_cast
  rw [Real.sin_pi_div_three]; ring

/-- Order 6 (valences five and seven on a triangle mesh): `‖B‖ = ‖d‖`. -/
theorem dislocation_norm_six (d : ℂ) : ‖(1 - zeta 6) * d‖ = ‖d‖ := by
  rw [dislocation_norm 6 (by norm_num)]
  push_cast
  rw [Real.sin_pi_div_six]; ring

end BeyondQuads

end ClosingHelices
