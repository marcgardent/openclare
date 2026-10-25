/-
Quarter-turns of the plane, over the reals.

The plane of a patch is `ℂ`, with `e₁ = 1` (the walking direction of a turn)
and `e₂ = I` (along its first side); the quarter-turn `R(a, b) = (-b, a)` of the
note is multiplication by `I`. A motion `x ↦ R^j x + t` is a `Motion ℂ ℂ`. The
along and across components of a translation `t` are `t.re` and `t.im`; the
shift of a turn is `t.im` (`⟨e₂, t⟩`). Lengths are real numbers here; the
statements about whole lengths are in `Checkerboard.lean`.
-/
module

public import TurnsOfATMesh.Motion
public import Mathlib.RingTheory.RootsOfUnity.Complex

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

open Complex Motion

/-! ### Gauss–Bonnet: when quarter-turns compose to the identity -/

/-- A product of quarter-turns `R^{j₁} ⋯ R^{jₘ}` is the identity exactly when
`j₁ + ⋯ + jₘ` is a multiple of four (Lemma "Gauss–Bonnet for turns" gives
`∑ (4 - k) = 4` on a disc and `0` on an annulus). -/
theorem quarter_turns_prod_eq_one_iff (js : List ℤ) :
    (js.map fun j => I ^ j).prod = 1 ↔ (4 : ℤ) ∣ js.sum := by
  have h : (js.map fun j => I ^ j).prod = I ^ js.sum := by
    induction js with
    | nil => simp
    | cons j js ih => simp [ih, zpow_add₀ I_ne_zero]
  rw [h]
  exact_mod_cast isPrimitiveRoot_I.zpow_eq_one_iff_dvd js.sum

/-! ### Forms `±d₁ ± d₂` -/

/-- `x = ε₁ d₁ + ε₂ d₂` with signs `ε₁, ε₂ ∈ {+1, -1}`. -/
def IsDiagonalForm (x : ℝ) (d : ℂ) : Prop :=
  ∃ ε₁ ε₂ : ℝ, (ε₁ = 1 ∨ ε₁ = -1) ∧ (ε₂ = 1 ∨ ε₂ = -1) ∧ x = ε₁ * d.re + ε₂ * d.im

theorem isDiagonalForm_iff (x : ℝ) (d : ℂ) :
    IsDiagonalForm x d ↔ x = d.re + d.im ∨ x = d.re - d.im ∨ x = -d.re + d.im ∨
      x = -d.re - d.im := by
  constructor
  · rintro ⟨ε₁, ε₂, h₁ | h₁, h₂ | h₂, rfl⟩ <;> subst h₁ h₂ <;> simp <;> ring_nf <;> simp
  · rintro (h | h | h | h)
    · exact ⟨1, 1, by norm_num, by norm_num, by rw [h]; ring⟩
    · exact ⟨1, -1, by norm_num, by norm_num, by rw [h]; ring⟩
    · exact ⟨-1, 1, by norm_num, by norm_num, by rw [h]; ring⟩
    · exact ⟨-1, -1, by norm_num, by norm_num, by rw [h]; ring⟩

/-- `(1 - R^{±1}) (±d)`: both components are of the form `±d₁ ± d₂`, and the
squared length is twice that of `d`. -/
theorem one_sub_quarter_mul (r σ : ℂ) (hr : r = I ∨ r = -I) (hσ : σ = 1 ∨ σ = -1) (d : ℂ) :
    IsDiagonalForm ((1 - r) * (σ * d)).re d ∧ IsDiagonalForm ((1 - r) * (σ * d)).im d ∧
      normSq ((1 - r) * (σ * d)) = 2 * normSq d := by
  rcases hr with rfl | rfl <;> rcases hσ with rfl | rfl <;>
    refine ⟨?_, ?_, ?_⟩ <;>
    simp only [normSq_apply, mul_re, mul_im, sub_re, sub_im, one_re, one_im, I_re, I_im,
      neg_re, neg_im] <;>
    (try simp only [isDiagonalForm_iff]) <;>
    first
    | ring1
    | (left; ring1)
    | (right; left; ring1)
    | (right; right; left; ring1)
    | (right; right; right; ring1)

/-! ### Proposition "A dislocation between two turns" -/

/-- The Burgers sum of a vertex of valence three at `p₃` (`R₃ = R`) and one of
valence five at `p₅` (`R₅ = R⁻¹ = -I`), in the two orders, and for the other
orientation of the border, which inverts the rotations: in all four cases it is
`(1 - R^{±1})(±d)` with `d = p₅ - p₃`, as the proof in the note says. -/
theorem dislocation_burgers (p₃ p₅ : ℂ) :
    burgers [(I, p₃), (-I, p₅)] = (1 - -I) * (p₅ - p₃) ∧
    burgers [(-I, p₅), (I, p₃)] = (1 - I) * (-(p₅ - p₃)) ∧
    burgers [(-I, p₃), (I, p₅)] = (1 - I) * (p₅ - p₃) ∧
    burgers [(I, p₅), (-I, p₃)] = (1 - -I) * (-(p₅ - p₃)) := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;>
    simp only [burgers, List.map_cons, List.map_nil, List.reverse_cons, List.reverse_nil,
      List.nil_append, List.prod_cons, List.prod_nil, mul_one, smul_eq_mul, add_zero] <;>
    ring_nf <;> simp only [I_sq] <;> ring

/-- **Proposition "A dislocation between two turns" (i).** Whatever the order of the
two vertices and the orientation, both components of the Burgers vector `B` of
the pair are `±d₁ ± d₂` and `|B|² = 2|d|²`. -/
theorem dislocation (p₃ p₅ : ℂ) (B : ℂ)
    (hB : B = burgers [(I, p₃), (-I, p₅)] ∨ B = burgers [(-I, p₅), (I, p₃)] ∨
      B = burgers [(-I, p₃), (I, p₅)] ∨ B = burgers [(I, p₅), (-I, p₃)]) :
    IsDiagonalForm B.re (p₅ - p₃) ∧ IsDiagonalForm B.im (p₅ - p₃) ∧
      normSq B = 2 * normSq (p₅ - p₃) := by
  obtain ⟨h1, h2, h3, h4⟩ := dislocation_burgers p₃ p₅
  have key := one_sub_quarter_mul
  rcases hB with rfl | rfl | rfl | rfl
  · have := key _ _ (Or.inr rfl) (Or.inl rfl) (p₅ - p₃); rw [one_mul] at this; rw [h1]; exact this
  · have := key _ _ (Or.inl rfl) (Or.inr rfl) (p₅ - p₃); rw [← neg_eq_neg_one_mul] at this
    rw [h2]; exact this
  · have := key _ _ (Or.inl rfl) (Or.inl rfl) (p₅ - p₃); rw [one_mul] at this; rw [h3]; exact this
  · have := key _ _ (Or.inr rfl) (Or.inr rfl) (p₅ - p₃); rw [← neg_eq_neg_one_mul] at this
    rw [h4]; exact this

/-- The two turns can close together only on a diagonal: the across component of
`B` vanishes exactly when `ε₁ d₁ + ε₂ d₂ = 0`. -/
theorem dislocation_closes_iff (x : ℝ) (d : ℂ) (ε₁ ε₂ : ℝ) (h : x = ε₁ * d.re + ε₂ * d.im) :
    x = 0 ↔ ε₁ * d.re + ε₂ * d.im = 0 := by rw [h]

/-- **Proposition "A dislocation between two turns" (ii).** If the two vertices lie
on one straight line of the field, `d = δ u` with `u ∈ {±e₁, ±e₂}`, both
components of `(1 - R^{±1})(±d)` have absolute value `|δ|`. -/
theorem dislocation_straight (r σ : ℂ) (hr : r = I ∨ r = -I) (hσ : σ = 1 ∨ σ = -1)
    (δ : ℝ) (u : ℂ) (hu : u = 1 ∨ u = -1 ∨ u = I ∨ u = -I) :
    |((1 - r) * (σ * (δ * u))).re| = |δ| ∧ |((1 - r) * (σ * (δ * u))).im| = |δ| := by
  rcases hr with rfl | rfl <;> rcases hσ with rfl | rfl <;> rcases hu with rfl | rfl | rfl | rfl <;>
    simp [abs_neg]

/-! ### Proposition "Staircases" -/

/-- **Proposition "Staircases", algebraic core.** If `d = ∑ᵢ ℓᵢ vᵢ` and every step
`vᵢ` climbs the diagonal, `ε₁ vᵢ₁ + ε₂ vᵢ₂ = 1`, then `ε₁ d₁ + ε₂ d₂ = ∑ᵢ ℓᵢ`.
Neither `vᵢ ∈ {±e₁, ±e₂}` nor `ε₁, ε₂ ∈ {±1}` is needed. -/
theorem staircase {ι : Type*} (s : Finset ι) (ε₁ ε₂ : ℝ) (ℓ : ι → ℝ) (v : ι → ℂ)
    (hv : ∀ i ∈ s, ε₁ * (v i).re + ε₂ * (v i).im = 1) :
    ε₁ * (∑ i ∈ s, (ℓ i : ℂ) * v i).re + ε₂ * (∑ i ∈ s, (ℓ i : ℂ) * v i).im = ∑ i ∈ s, ℓ i := by
  rw [re_sum, im_sum, Finset.mul_sum, Finset.mul_sum, ← Finset.sum_add_distrib]
  refine Finset.sum_congr rfl fun i hi => ?_
  have := hv i hi
  simp only [mul_re, mul_im, ofReal_re, ofReal_im, zero_mul, sub_zero, add_zero]
  linear_combination ℓ i * this

/-- Both turns are rings only if every arc of the staircase has length zero. -/
theorem staircase_rings {ι : Type*} (s : Finset ι) (ℓ : ι → ℝ) (hℓ : ∀ i ∈ s, 0 ≤ ℓ i)
    (s₁ s₂ : ℝ) (hdiff : s₁ - s₂ = ∑ i ∈ s, ℓ i) (h₁ : s₁ = 0) (h₂ : s₂ = 0) :
    ∀ i ∈ s, ℓ i = 0 := by
  have : ∑ i ∈ s, ℓ i = 0 := by rw [← hdiff, h₁, h₂, sub_zero]
  exact (Finset.sum_eq_zero_iff_of_nonneg hℓ).mp this

/-- A form with non-negative coefficients, one of them positive on an arc held at
length at least one, is positive at non-negative lengths: so if a combination of
shifts equals it, the shifts cannot all vanish (Remark "Sign-definite turns",
Propositions "Staircases" and "A separatrix that winds"). -/
theorem nonneg_form_pos {ι : Type*} (s : Finset ι) (c ℓ : ι → ℝ) (hc : ∀ a ∈ s, 0 ≤ c a)
    (hℓ : ∀ a ∈ s, 0 ≤ ℓ a) (a₀ : ι) (ha₀ : a₀ ∈ s) (hc₀ : 0 < c a₀) (hℓ₀ : 1 ≤ ℓ a₀) :
    0 < ∑ a ∈ s, c a * ℓ a := by
  have hle : c a₀ * ℓ a₀ ≤ ∑ a ∈ s, c a * ℓ a :=
    Finset.single_le_sum (f := fun a => c a * ℓ a) (fun a ha => mul_nonneg (hc a ha) (hℓ a ha)) ha₀
  nlinarith

/-! ### Proposition "Exchanging two vertices" -/

/-- **Proposition "Exchanging two vertices".** Two rotations by `R^{±1}` (valence
three or five) about `p` and `p'`: `ρ_p ∘ ρ_{p'}⁻¹` is the translation by
`(1 - R^{±1})(p - p')`, whose components are `±d₁ ± d₂`, `d = p - p'`. -/
theorem exchange_quarter (u : ℂˣ) (hu : (u : ℂ) = I ∨ (u : ℂ) = -I) (p p' : ℂ) :
    comp (rotAbout (u : ℂ) p) (rotAbout (↑u⁻¹ : ℂ) p') = translation ((1 - u) * (p - p')) ∧
      IsDiagonalForm ((1 - u) * (p - p')).re (p - p') ∧
      IsDiagonalForm ((1 - u) * (p - p')).im (p - p') := by
  refine ⟨?_, ?_⟩
  · rw [exchange]; congr 1; simp only [smul_eq_mul]; ring
  · have := one_sub_quarter_mul (u : ℂ) 1 hu (Or.inl rfl) (p - p')
    rw [one_mul] at this
    exact ⟨this.1, this.2.1⟩

end TurnsOfATMesh
