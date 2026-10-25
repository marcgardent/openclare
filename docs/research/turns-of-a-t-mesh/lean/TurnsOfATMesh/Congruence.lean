/-
What survives of the Fox–Burgers formula modulo `(1 - z)²`, for rotations that
are powers of one rotation `z`.

This is the general form of the checkerboard rule. Take the rotations to be
`uᵢ = z^{jᵢ}` in a commutative ring `A` (for quad meshes `z` is the quarter-turn
and `jᵢ = 4 - k(cᵢ)`). Then every Fox derivative is congruent to its augmentation
modulo `1 - z`, and `1 - z^j` to `j (1 - z)` modulo `(1 - z)²`, so

  `τ(w) ≡ (1 - z) ∑ᵢ εᵢ(w) jᵢ pᵢ  (mod (1 - z)²)`,

with `εᵢ(w)` the exponent sums (the winding numbers). What this says depends on
the order of `z`:
* order 4 (quad meshes): `(1 - z)² = -2z`, the congruence is modulo 2, and
  `(1 - z) q` modulo 2 only depends on `q` modulo `1 - z`, the colour of the
  checkerboard (`Checkerboard.lean`);
* order 3: `(1 - z)² = -3z`, a congruence modulo 3 (three colours);
* order 2: `(1 - z)² = 4`, a congruence modulo 4;
* order 6 (rotations of a triangular or hexagonal grid): `1 - z` is a unit and
  the congruence says nothing: there is no colouring rule.
-/
module

public import TurnsOfATMesh.Fox
public import Mathlib.Tactic.LinearCombination
public import Mathlib.GroupTheory.OrderOfElement

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

variable {ι A V : Type*} [CommRing A] [AddCommGroup V] [Module A V]

/-- `z^k ≡ 1 (mod 1 - z)`. -/
theorem zpow_congr_one (z : Aˣ) (k : ℤ) : ∃ c : A, ((z ^ k : Aˣ) : A) = 1 + c * (1 - z) := by
  have hz : (z : A) * ↑z⁻¹ = 1 := Units.mul_inv z
  induction k using Int.induction_on with
  | zero => exact ⟨0, by simp⟩
  | succ k ih =>
    obtain ⟨c, hc⟩ := ih
    refine ⟨c * z - 1, ?_⟩
    rw [zpow_add_one, Units.val_mul, hc]; ring
  | pred k ih =>
    obtain ⟨c, hc⟩ := ih
    refine ⟨(↑z⁻¹ : A) + c * ↑z⁻¹, ?_⟩
    rw [zpow_sub_one, Units.val_mul, hc]
    linear_combination hz

/-- `1 - z^k ≡ k (1 - z) (mod (1 - z)²)`. -/
theorem one_sub_zpow_congr (z : Aˣ) (k : ℤ) :
    ∃ c : A, 1 - ((z ^ k : Aˣ) : A) = (k : A) * (1 - z) + c * (1 - z) ^ 2 := by
  have hz : (z : A) * ↑z⁻¹ = 1 := Units.mul_inv z
  induction k using Int.induction_on with
  | zero => exact ⟨0, by simp⟩
  | succ k ih =>
    obtain ⟨c, hc⟩ := ih
    obtain ⟨c₀, hc₀⟩ := zpow_congr_one z (k : ℤ)
    refine ⟨c + c₀, ?_⟩
    rw [zpow_add_one, Units.val_mul]
    push_cast at hc ⊢
    linear_combination hc + (1 - (z : A)) * hc₀
  | pred k ih =>
    obtain ⟨c, hc⟩ := ih
    obtain ⟨c₁, hc₁⟩ := zpow_congr_one z (-(k : ℤ) - 1)
    refine ⟨c - c₁, ?_⟩
    rw [zpow_sub_one, Units.val_mul] at hc₁ ⊢
    push_cast at hc ⊢
    linear_combination hc - (1 - (z : A)) * hc₁ - ((z ^ (-(k : ℤ)) : Aˣ) : A) * hz

section Powers

variable [DecidableEq ι] (z : Aˣ) (j : ι → ℤ)

/-- The rotations `uᵢ = z^{jᵢ}`. -/
def powers : ι → Aˣ := fun i => z ^ j i

theorem letterLin_powers (x : ι × Bool) :
    letterLin (powers z j) x = ((z ^ (if x.2 then j x.1 else -j x.1) : Aˣ) : A) := by
  obtain ⟨i, b⟩ := x
  cases b <;> simp [letterLin, powers, zpow_neg]

/-- Every Fox derivative is congruent to its augmentation, the exponent sum,
modulo `1 - z`. -/
theorem fox_congr (i : ι) (w : List (ι × Bool)) :
    ∃ c : A, fox (powers z j) i w = (expSum i w : A) + c * (1 - z) := by
  induction w with
  | nil => exact ⟨0, by simp [expSum]⟩
  | cons x w ih =>
    obtain ⟨k, b⟩ := x
    obtain ⟨c, hc⟩ := ih
    obtain ⟨c₀, hc₀⟩ := zpow_congr_one z (if b then j k else -j k)
    obtain ⟨c₁, hc₁⟩ := zpow_congr_one z (-j i)
    have hL := letterLin_powers z j (k, b)
    simp only at hL
    refine ⟨(if k = i then (if b then 0 else -c₁) else 0) + c₀ * expSum i w + c +
      c₀ * c * (1 - z), ?_⟩
    simp only [fox, expSum, hL, hc, hc₀]
    by_cases hki : k = i
    · subst hki
      cases b
      · simp only [ite_true, Bool.false_eq_true, ite_false, powers, ← zpow_neg, hc₁]
        push_cast; ring
      · simp only [ite_true]; push_cast; ring
    · simp only [hki, ite_false]; push_cast; ring

/-- The rotation of a word is `z` to the power `∑ᵢ εᵢ(w) jᵢ`. -/
theorem rhoW_powers [Fintype ι] (w : List (ι × Bool)) :
    rhoW (powers z j) w = ((z ^ (∑ i, expSum i w * j i) : Aˣ) : A) := by
  induction w with
  | nil => simp [expSum]
  | cons x w ih =>
    obtain ⟨k, b⟩ := x
    rw [rhoW_cons, ih, letterLin_powers, ← Units.val_mul, ← zpow_add]
    congr 2
    simp only [expSum, add_mul, Finset.sum_add_distrib]
    congr 1
    rw [Finset.sum_eq_single k (fun i _ hik => by simp [Ne.symm hik]) (by simp)]
    cases b <;> simp

/-- **Checkerboard rule, general form.** For rotations `uᵢ = z^{jᵢ}`,
`τ(w) ≡ (1 - z) ∑ᵢ εᵢ(w) jᵢ pᵢ  (mod (1 - z)²)`. -/
theorem burgers_congr [Fintype ι] (p : ι → V) (w : List (ι × Bool)) :
    ∃ v : V, (hol (powers z j) p w).trans =
      (1 - (z : A)) • ∑ i, ((expSum i w * j i : ℤ) : A) • p i + (1 - (z : A)) ^ 2 • v := by
  choose c hc using fun i => fox_congr z j i w
  choose c' hc' using fun i => one_sub_zpow_congr z (j i)
  refine ⟨∑ i, ((expSum i w : A) * c' i + c i * (j i : A) + c i * c' i * (1 - z)) • p i, ?_⟩
  rw [fox_burgers, Finset.smul_sum, Finset.smul_sum, ← Finset.sum_add_distrib]
  refine Finset.sum_congr rfl fun i _ => ?_
  have e : p i - ((powers z j i : Aˣ) : A) • p i = (1 - ((z ^ j i : Aˣ) : A)) • p i := by
    rw [sub_smul, one_smul]; rfl
  rw [e, ← mul_smul, ← mul_smul, ← mul_smul, ← add_smul, hc i, hc' i]
  congr 1
  push_cast
  ring

/-- If the rotation `z` has order exactly `n` and a word has trivial rotation,
`n` divides `∑ᵢ εᵢ(w) jᵢ`: the Gauss–Bonnet condition read on the winding
numbers. -/
theorem sum_dvd_of_rhoW_eq_one [Fintype ι] (w : List (ι × Bool))
    (h : rhoW (powers z j) w = 1) : (orderOf z : ℤ) ∣ ∑ i, expSum i w * j i := by
  rw [rhoW_powers, Units.val_eq_one] at h
  exact orderOf_dvd_iff_zpow_eq_one.mpr h

end Powers

/-! ### What `(1 - z)²` is for rotations of order 2, 3, 4 and 6 -/

theorem one_sub_sq_order_two (z : A) (hz : z = -1) : (1 - z) ^ 2 = 4 := by
  subst hz; ring

theorem one_sub_sq_order_three (z : A) (hz : z ^ 2 + z + 1 = 0) : (1 - z) ^ 2 = -3 * z := by
  linear_combination hz

theorem one_sub_sq_order_four (z : A) (hz : z ^ 2 = -1) : (1 - z) ^ 2 = -2 * z := by
  linear_combination hz

/-- For a rotation by a sixth of a turn (`z² - z + 1 = 0`), `1 - z` is a unit, of
inverse `z`: every vector is `(1 - z) d` for some `d`, so the congruence of
`burgers_congr` is empty and there is no colouring rule on triangular or
hexagonal grids. -/
theorem one_sub_unit_order_six (z : A) (hz : z ^ 2 - z + 1 = 0) (v : V) :
    (1 - z) * z = 1 ∧ (1 - z) • (z • v) = v := by
  have h : (1 - z) * z = 1 := by linear_combination -hz
  exact ⟨h, by rw [← mul_smul, h, one_smul]⟩

end TurnsOfATMesh
