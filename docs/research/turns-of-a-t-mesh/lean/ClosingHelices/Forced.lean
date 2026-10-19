/-
Theorem "Forced helices" and Corollary "Chains of dislocations".

A turn is anything with a shift: a function `shift γ` of the lengths. Nothing
here uses that the shifts are linear, that the lengths are whole, or that the
set `P` of lengths is convex: the theorem holds for any set of lengths, for
instance one that also fixes the number of quads (a quadratic, non-convex
condition). Convexity only matters to compute or certify `τ_P(H)` by linear
programming.
-/
module

public import Mathlib.Data.Finset.Card
public import Mathlib.Order.ConditionallyCompleteLattice.Basic
public import Mathlib.Order.Lattice.Nat
public import Mathlib.Basic.Real.Basic
public import Mathlib.Algebra.Order.Field.Basic
public import Mathlib.Tactic.Linarith
public import Mathlib.Tactic.Ring

@[expose] public section

namespace ClosingHelices

open Classical

section Forced

variable {ι L : Type*} [DecidableEq ι] (shift : ι → L → ℝ) (P : Set L)

/-- The turns of `S` can be closed together by lengths in `P`. -/
def Closable (S : Finset ι) : Prop := ∃ ℓ ∈ P, ∀ γ ∈ S, shift γ ℓ = 0

/-- `τ_P(H)`: the fewest turns of `H` to let go for the others to close together in
`P` (`0` if `P` is empty, a case the note leaves undefined). -/
noncomputable def tauP (H : Finset ι) : ℕ :=
  sInf {n | ∃ X ⊆ H, X.card = n ∧ Closable shift P (H \ X)}

/-- The turns of `H` that `ℓ` leaves shifted. -/
noncomputable def shifted (H : Finset ι) (ℓ : L) : Finset ι := H.filter fun γ => shift γ ℓ ≠ 0

/-- **Theorem "Forced helices", first part.** For every `ℓ ∈ P`, at least `τ_P(H)`
turns of `H` have a non-zero shift. -/
theorem forced_helices (H : Finset ι) (ℓ : L) (hℓ : ℓ ∈ P) :
    tauP shift P H ≤ (shifted shift H ℓ).card := by
  apply Nat.sInf_le
  refine ⟨shifted shift H ℓ, Finset.filter_subset _ _, rfl, ℓ, hℓ, fun γ hγ => ?_⟩
  rw [Finset.mem_sdiff, shifted, Finset.mem_filter] at hγ
  by_contra h
  exact hγ.2 ⟨hγ.1, h⟩

/-- A set whose removal leaves a closable set meets every infeasible subset. -/
theorem meets_infeasible (H X S : Finset ι) (hS : S ⊆ H) (hX : Closable shift P (H \ X))
    (hinf : ¬ Closable shift P S) : (S ∩ X).Nonempty := by
  by_contra h
  rw [Finset.not_nonempty_iff_eq_empty] at h
  apply hinf
  obtain ⟨ℓ, hℓ, hclose⟩ := hX
  refine ⟨ℓ, hℓ, fun γ hγ => hclose γ ?_⟩
  rw [Finset.mem_sdiff]
  refine ⟨hS hγ, fun hγX => ?_⟩
  have : γ ∈ S ∩ X := Finset.mem_inter.mpr ⟨hγ, hγX⟩
  rw [h] at this
  exact absurd this (Finset.notMem_empty γ)

/-- **Theorem "Forced helices", second part.** If `H` contains `k` pairwise disjoint
subsets, none closable within `P`, every set of turns whose removal leaves a
closable set has at least `k` turns; so `τ_P(H) ≥ k` when `P` is not empty. -/
theorem card_ge_of_disjoint (H X : Finset ι) {k : ℕ} (S : Fin k → Finset ι)
    (hS : ∀ j, S j ⊆ H) (hdisj : ∀ j j', j ≠ j' → Disjoint (S j) (S j'))
    (hinf : ∀ j, ¬ Closable shift P (S j)) (hX : Closable shift P (H \ X)) : k ≤ X.card := by
  have hmeet : ∀ j, (S j ∩ X).Nonempty := fun j => meets_infeasible shift P H X (S j) (hS j) hX (hinf j)
  choose f hf using hmeet
  have hinj : Function.Injective f := by
    intro j j' hjj'
    by_contra hne
    have h1 := (Finset.mem_inter.mp (hf j)).1
    have h2 := (Finset.mem_inter.mp (hf j')).1
    rw [hjj'] at h1
    exact Finset.disjoint_left.mp (hdisj j j' hne) h1 h2
  have hmaps : ∀ j ∈ (Finset.univ : Finset (Fin k)), f j ∈ X :=
    fun j _ => (Finset.mem_inter.mp (hf j)).2
  have := Finset.card_le_card_of_injOn f hmaps (fun a _ b _ h => hinj h)
  simpa using this

theorem tauP_ge_of_disjoint (H : Finset ι) (hP : P.Nonempty) {k : ℕ} (S : Fin k → Finset ι)
    (hS : ∀ j, S j ⊆ H) (hdisj : ∀ j j', j ≠ j' → Disjoint (S j) (S j'))
    (hinf : ∀ j, ¬ Closable shift P (S j)) : k ≤ tauP shift P H := by
  have hne : {n | ∃ X ⊆ H, X.card = n ∧ Closable shift P (H \ X)}.Nonempty := by
    obtain ⟨ℓ, hℓ⟩ := hP
    exact ⟨H.card, H, le_refl _, rfl, ℓ, hℓ, fun γ hγ => by simp at hγ⟩
  have hmem := Nat.sInf_mem hne
  obtain ⟨X, -, hcard, hX⟩ := hmem
  unfold tauP
  rw [← hcard]
  exact card_ge_of_disjoint shift P H X S hS hdisj hinf hX

end Forced

/-! ### Corollary "Chains of dislocations" -/

section Chain

variable (m : ℕ) (s ε F : ℕ → ℝ)

/-- Between two rings `γᵢ`, `γⱼ` of a chain, `i < j`, the signs `εₖ` change. -/
theorem sign_change_between (hε : ∀ k, ε k = 1 ∨ ε k = -1)
    (hF : ∀ k, 1 ≤ k → k ≤ m → 0 < F k)
    (hs : ∀ k, 1 ≤ k → k ≤ m → s (k - 1) - s k = ε k * F k)
    (i j : ℕ) (hij : i < j) (hjm : j ≤ m) (hi : s i = 0) (hj : s j = 0) :
    ∃ k, i + 1 ≤ k ∧ k + 1 ≤ j ∧ ε (k + 1) ≠ ε k := by
  by_contra hno
  push Not at hno
  -- all the signs from i + 1 to j are equal
  have hconst : ∀ n, i + 1 ≤ n → n ≤ j → ε n = ε (i + 1) := by
    intro n hn1 hn2
    induction n with
    | zero => omega
    | succ n ih =>
      rcases Nat.lt_or_ge (i + 1) (n + 1) with h | h
      · rw [hno n (by omega) (by omega), ih (by omega) (by omega)]
      · have : n + 1 = i + 1 := by omega
        rw [this]
  have hε2 : ε (i + 1) * ε (i + 1) = 1 := by
    rcases hε (i + 1) with h | h <;> rw [h] <;> norm_num
  -- ε (i+1) (s i - s n) > 0 for i < n ≤ j
  have hpos : ∀ n, i + 1 ≤ n → n ≤ j → 0 < ε (i + 1) * (s i - s n) := by
    intro n hn1 hn2
    induction n with
    | zero => omega
    | succ n ih =>
      have hstep := hs (n + 1) (by omega) (by omega)
      simp only [Nat.add_sub_cancel] at hstep
      have hFn := hF (n + 1) (by omega) (by omega)
      have hεn := hconst (n + 1) hn1 hn2
      rcases Nat.lt_or_ge (i + 1) (n + 1) with h | h
      · have ih' := ih (by omega) (by omega)
        have : ε (i + 1) * (s i - s (n + 1)) =
            ε (i + 1) * (s i - s n) + ε (i + 1) * ε (n + 1) * F (n + 1) := by
          rw [mul_assoc, ← hstep]; ring
        rw [this, hεn, hε2, one_mul]
        linarith
      · have hn : n = i := by omega
        subst hn
        rw [hstep, ← mul_assoc, hε2, one_mul]
        exact hFn
  have := hpos j (by omega) le_rfl
  rw [hi, hj, sub_zero, mul_zero] at this
  exact lt_irrefl 0 this

/-- The number of sign changes `εₖ₊₁ ≠ εₖ` with `1 ≤ k < n`. -/
noncomputable def changesBelow (n : ℕ) : ℕ :=
  ((Finset.Ico 1 n).filter fun k => ε (k + 1) ≠ ε k).card

/-- **Corollary "Chains of dislocations".** If `s_{i-1} - s_i = εᵢ Fᵢ` with
`εᵢ = ±1` and `Fᵢ > 0` for `i = 1, …, m`, at most `q + 1` of `s₀, …, sₘ` vanish,
`q` the number of sign changes among `ε₁, …, εₘ`: at least `m - q` of the
`m + 1` turns are helices. -/
theorem chain_of_dislocations (hε : ∀ k, ε k = 1 ∨ ε k = -1)
    (hF : ∀ k, 1 ≤ k → k ≤ m → 0 < F k)
    (hs : ∀ k, 1 ≤ k → k ≤ m → s (k - 1) - s k = ε k * F k) :
    ((Finset.range (m + 1)).filter fun i => s i = 0).card ≤ changesBelow ε m + 1 := by
  classical
  -- map a ring i to the number of sign changes below it: strictly increasing
  have hmono : ∀ i j, i < j → j ≤ m → s i = 0 → s j = 0 →
      changesBelow ε i < changesBelow ε j := by
    intro i j hij hjm hi hj
    obtain ⟨k, hk1, hk2, hk⟩ := sign_change_between m s ε F hε hF hs i j hij hjm hi hj
    apply Finset.card_lt_card
    refine ⟨fun x hx => ?_, fun hsub => ?_⟩
    · rw [Finset.mem_filter, Finset.mem_Ico] at hx ⊢
      exact ⟨⟨hx.1.1, by omega⟩, hx.2⟩
    · have : k ∈ (Finset.Ico 1 j).filter fun k => ε (k + 1) ≠ ε k := by
        rw [Finset.mem_filter, Finset.mem_Ico]; exact ⟨⟨by omega, by omega⟩, hk⟩
      have := hsub this
      rw [Finset.mem_filter, Finset.mem_Ico] at this
      omega
  have hle : ∀ i ≤ m, changesBelow ε i ≤ changesBelow ε m := by
    intro i hi
    apply Finset.card_le_card
    intro x hx
    rw [Finset.mem_filter, Finset.mem_Ico] at hx ⊢
    exact ⟨⟨hx.1.1, by omega⟩, hx.2⟩
  have hmaps : ∀ i ∈ (Finset.range (m + 1)).filter (fun i => s i = 0),
      changesBelow ε i ∈ Finset.range (changesBelow ε m + 1) := by
    intro i hi
    rw [Finset.mem_filter, Finset.mem_range] at hi
    rw [Finset.mem_range]
    have := hle i (by omega)
    omega
  have hinj : Set.InjOn (changesBelow ε) ((Finset.range (m + 1)).filter fun i => s i = 0) := by
    intro i hi j hj hij
    simp only [Finset.coe_filter, Finset.mem_range, Set.mem_ofPred_eq] at hi hj
    by_contra hne
    rcases Nat.lt_or_gt_of_ne hne with h | h
    · exact absurd hij (ne_of_lt (hmono i j h (by omega) hi.2 hj.2))
    · exact absurd hij.symm (ne_of_lt (hmono j i h (by omega) hj.2 hi.2))
  have := Finset.card_le_card_of_injOn (changesBelow ε) hmaps hinj
  simpa using this

end Chain

end ClosingHelices
