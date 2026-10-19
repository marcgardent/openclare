/-
Farkas' lemma by Fourier–Motzkin elimination: Remarks "Sign-definite turns;
Farkas certificates" and "Whole solutions exist at some size".

Mathlib proves Farkas' lemma over the reals for closed cones (by Hahn–Banach), but
the note needs it over the rationals too, to make the multipliers whole and to
scale a real solution into a whole one. So it is proved here from scratch, over
any linearly ordered field, by eliminating the variables one after the other.

A constraint `a ⬝ x ≤ β` is a pair `(a, β)`. It is **derivable** from a set of
constraints if it is a non-negative combination of them (`Derivable`). A finite
system has no solution exactly when `0 ≤ β` with `β < 0` is derivable
(`infeasible_iff_derivable`); that is, exactly when non-negative multipliers
`λ` give `∑ λᵢ aᵢ = 0` and `∑ λᵢ βᵢ < 0` (`farkas`).
-/
module

public import Mathlib.Data.Matrix.ColumnRowPartitioned
public import Mathlib.Data.Matrix.Mul
public import Mathlib.Algebra.Order.BigOperators.Group.Finset
public import Mathlib.Algebra.Order.Field.Basic
public import Mathlib.Data.Rat.Cast.Order
public import Mathlib.Basic.Real.Basic
public import Mathlib.Algebra.BigOperators.Ring.Finset
public import Mathlib.Data.Fintype.BigOperators
public import Mathlib.Algebra.Order.BigOperators.GroupWithZero.Finset
public import Mathlib.Data.Nat.Cast.Field
public import Mathlib.Data.Rat.BigOperators
public import Mathlib.Tactic.Linarith
public import Mathlib.Tactic.Positivity
public import Mathlib.Tactic.FieldSimp
public import Mathlib.Tactic.Ring
public import Mathlib.Tactic.LinearCombination

@[expose] public section

set_option linter.unusedSectionVars false

namespace ClosingHelices

namespace Farkas

open Finset Matrix

variable {K : Type*} [Field K] [LinearOrder K] [IsStrictOrderedRing K]
variable {V : Type*} [Fintype V] [DecidableEq V]

/-- The constraints derivable from `S`: non-negative combinations of them. -/
inductive Derivable (S : Set ((V → K) × K)) : (V → K) × K → Prop
  | mem {c} : c ∈ S → Derivable S c
  | zero : Derivable S 0
  | add {c d} : Derivable S c → Derivable S d → Derivable S (c + d)
  | smul {c} (t : K) : 0 ≤ t → Derivable S c → Derivable S (t • c)

/-- `x` satisfies every constraint of `S`. -/
def Solves (S : Set ((V → K) × K)) (x : V → K) : Prop := ∀ c ∈ S, c.1 ⬝ᵥ x ≤ c.2

/-- A derivable constraint holds at every solution. -/
theorem Derivable.sound {S : Set ((V → K) × K)} {c : (V → K) × K} (h : Derivable S c)
    {x : V → K} (hx : Solves S x) : c.1 ⬝ᵥ x ≤ c.2 := by
  induction h with
  | mem hc => exact hx _ hc
  | zero => simp
  | add _ _ ih₁ ih₂ =>
    simp only [Prod.fst_add, Prod.snd_add, add_dotProduct]; linarith
  | @smul c t ht _ ih =>
    simp only [Prod.smul_fst, Prod.smul_snd, smul_dotProduct, smul_eq_mul]
    exact mul_le_mul_of_nonneg_left ih ht

theorem Derivable.mono {S S' : Set ((V → K) × K)} (hS : ∀ c ∈ S, Derivable S' c)
    {c : (V → K) × K} (h : Derivable S c) : Derivable S' c := by
  induction h with
  | mem hc => exact hS _ hc
  | zero => exact .zero
  | add _ _ ih₁ ih₂ => exact .add ih₁ ih₂
  | smul t ht _ ih => exact .smul t ht ih

/-! ### One step of Fourier–Motzkin elimination -/

/-- Eliminating the variable `j`: the constraints without `j`, and for every pair of a
constraint `p` with a positive coefficient on `j` and a constraint `n` with a
negative one, the combination `(-n_j) p + p_j n`, without `j`. -/
noncomputable def elim (j : V) (S : Finset ((V → K) × K)) : Finset ((V → K) × K) :=
  S.filter (fun c => c.1 j = 0) ∪
    ((S.filter (fun c => 0 < c.1 j)) ×ˢ (S.filter (fun c => c.1 j < 0))).image
      (fun pn => (-pn.2.1 j) • pn.1 + pn.1.1 j • pn.2)

theorem mem_elim {j : V} {S : Finset ((V → K) × K)} {c : (V → K) × K} :
    c ∈ elim j S ↔ (c ∈ S ∧ c.1 j = 0) ∨ ∃ p ∈ S, ∃ n ∈ S, 0 < p.1 j ∧ n.1 j < 0 ∧
      c = (-n.1 j) • p + p.1 j • n := by
  simp only [elim, mem_union, mem_filter, mem_image, mem_product]
  constructor
  · rintro (h | ⟨⟨p, n⟩, ⟨⟨hp, hpj⟩, hn, hnj⟩, rfl⟩)
    · exact Or.inl h
    · exact Or.inr ⟨p, hp, n, hn, hpj, hnj, rfl⟩
  · rintro (h | ⟨p, hp, n, hn, hpj, hnj, rfl⟩)
    · exact Or.inl h
    · exact Or.inr ⟨(p, n), ⟨⟨hp, hpj⟩, hn, hnj⟩, rfl⟩

/-- Every constraint of the eliminated system is derivable from the system. -/
theorem elim_derivable (j : V) (S : Finset ((V → K) × K)) :
    ∀ c ∈ elim j S, Derivable (S : Set ((V → K) × K)) c := by
  intro c hc
  rcases mem_elim.mp hc with ⟨hcS, -⟩ | ⟨p, hp, n, hn, hpj, hnj, rfl⟩
  · exact .mem hcS
  · exact .add (.smul _ (by linarith) (.mem hp)) (.smul _ hpj.le (.mem hn))

/-- The eliminated system no longer involves `j`. -/
theorem elim_support {j : V} {W : Finset V} {S : Finset ((V → K) × K)}
    (hS : ∀ c ∈ S, ∀ v ∉ insert j W, c.1 v = 0) : ∀ c ∈ elim j S, ∀ v ∉ W, c.1 v = 0 := by
  intro c hc v hv
  rcases mem_elim.mp hc with ⟨hcS, hcj⟩ | ⟨p, hp, n, hn, hpj, hnj, rfl⟩
  · by_cases hvj : v = j
    · rw [hvj, hcj]
    · exact hS c hcS v (by simp [hvj, hv])
  · simp only [Prod.fst_add, Prod.smul_fst, Pi.add_apply, Pi.smul_apply, smul_eq_mul]
    by_cases hvj : v = j
    · rw [hvj]; ring
    · rw [hS p hp v (by simp [hvj, hv]), hS n hn v (by simp [hvj, hv])]; ring

/-- Between finitely many lower bounds, each below every upper bound, and the upper
bounds, there is a value. -/
theorem exists_between_finsets (L U : Finset K) (h : ∀ l ∈ L, ∀ u ∈ U, l ≤ u) :
    ∃ t, (∀ l ∈ L, l ≤ t) ∧ ∀ u ∈ U, t ≤ u := by
  rcases L.eq_empty_or_nonempty with rfl | hL
  · rcases U.eq_empty_or_nonempty with rfl | hU
    · exact ⟨0, by simp, by simp⟩
    · exact ⟨U.min' hU, by simp, fun u hu => U.min'_le u hu⟩
  · exact ⟨L.max' hL, fun l hl => L.le_max' l hl, fun u hu => (L.max'_le hL _ fun l hl => h l hl u hu)⟩

/-- `a ⬝ (x with the value t at j) = a ⬝ x + a_j (t - x_j)`. -/
theorem dotProduct_update (a x : V → K) (j : V) (t : K) :
    a ⬝ᵥ Function.update x j t = a ⬝ᵥ x + a j * (t - x j) := by
  have : Function.update x j t = x + Pi.single j (t - x j) := by
    funext v
    by_cases hv : v = j
    · subst hv; simp
    · simp [hv]
  rw [this, dotProduct_add, dotProduct_single]

/-- **A solution of the eliminated system extends to one of the system**: choose the
value of `j` between the bounds the constraints with `j` put on it. -/
theorem solves_of_elim (j : V) (S : Finset ((V → K) × K)) (x : V → K)
    (hx : Solves (elim j S : Set ((V → K) × K)) x) :
    ∃ x', Solves (S : Set ((V → K) × K)) x' := by
  -- `r c`: the value of `c` at `x` without its term in `j`.
  set r : (V → K) × K → K := fun c => c.1 ⬝ᵥ x - c.1 j * x j with hr
  set P := S.filter (fun c => 0 < c.1 j)
  set N := S.filter (fun c => c.1 j < 0)
  obtain ⟨t, hlow, hup⟩ := exists_between_finsets (N.image fun n => (n.2 - r n) / n.1 j)
    (P.image fun p => (p.2 - r p) / p.1 j) (by
      simp only [mem_image]
      rintro _ ⟨n, hn, rfl⟩ _ ⟨p, hp, rfl⟩
      simp only [N, P, mem_filter] at hn hp
      have hc := hx _ (mem_elim.mpr (Or.inr ⟨p, hp.1, n, hn.1, hp.2, hn.2, rfl⟩))
      simp only [Prod.fst_add, Prod.snd_add, Prod.smul_fst, Prod.smul_snd, add_dotProduct,
        smul_dotProduct, smul_eq_mul] at hc
      -- `(n₂ - r n) / n_j ≤ (p₂ - r p) / p_j`
      have hpj := hp.2
      have hnj := hn.2
      set q := (n.2 - r n) / n.1 j with hq
      have hqn : q * n.1 j = n.2 - r n := div_mul_cancel₀ _ hnj.ne
      rw [le_div_iff₀ hpj]
      have : (-n.1 j) * (q * p.1 j) ≤ (-n.1 j) * (p.2 - r p) := by
        simp only [hr] at hqn ⊢
        nlinarith
      exact le_of_mul_le_mul_left this (by linarith))
  refine ⟨Function.update x j t, fun c hc => ?_⟩
  rw [dotProduct_update]
  rcases lt_trichotomy (c.1 j) 0 with hcj | hcj | hcj
  · have := hlow _ (mem_image.mpr ⟨c, mem_filter.mpr ⟨hc, hcj⟩, rfl⟩)
    rw [div_le_iff_of_neg hcj] at this
    simp only [hr] at this
    nlinarith
  · rw [hcj, zero_mul, add_zero]
    exact hx c (mem_elim.mpr (Or.inl ⟨hc, hcj⟩))
  · have := hup _ (mem_image.mpr ⟨c, mem_filter.mpr ⟨hc, hcj⟩, rfl⟩)
    rw [le_div_iff₀ hcj] at this
    simp only [hr] at this
    nlinarith

/-! ### Farkas' lemma -/

/-- **Fourier–Motzkin.** A finite system with no solution derives `0 ≤ β` with `β < 0`. -/
theorem derivable_of_infeasible (W : Finset V) :
    ∀ S : Finset ((V → K) × K), (∀ c ∈ S, ∀ v ∉ W, c.1 v = 0) →
      (¬ ∃ x, Solves (S : Set ((V → K) × K)) x) →
      ∃ β < 0, Derivable (S : Set ((V → K) × K)) (0, β) := by
  induction W using Finset.induction_on with
  | empty =>
    intro S hS hinf
    have hzero : ∀ c ∈ S, c.1 = 0 := fun c hc => funext fun v => hS c hc v (notMem_empty v)
    have : ¬ Solves (S : Set ((V → K) × K)) 0 := fun h => hinf ⟨0, h⟩
    simp only [Solves, not_forall, not_le, dotProduct_zero] at this
    obtain ⟨c, hc, hneg⟩ := this
    refine ⟨c.2, hneg, ?_⟩
    have : c = (0, c.2) := Prod.ext (hzero c hc) rfl
    exact this ▸ .mem hc
  | @insert j W hj ih =>
    intro S hS hinf
    have h1 := elim_support hS
    have h2 : ¬ ∃ x, Solves (elim j S : Set ((V → K) × K)) x := by
      rintro ⟨x, hx⟩
      exact hinf (solves_of_elim j S x hx)
    obtain ⟨β, hβ, hder⟩ := ih (elim j S) h1 h2
    exact ⟨β, hβ, hder.mono (elim_derivable j S)⟩

/-- A finite system has no solution exactly when it derives `0 ≤ β` with `β < 0`. -/
theorem infeasible_iff_derivable (S : Finset ((V → K) × K)) :
    (¬ ∃ x, Solves (S : Set ((V → K) × K)) x) ↔
      ∃ β < 0, Derivable (S : Set ((V → K) × K)) (0, β) := by
  refine ⟨derivable_of_infeasible univ S (fun c _ v hv => absurd (mem_univ v) hv), ?_⟩
  rintro ⟨β, hβ, hder⟩ ⟨x, hx⟩
  have := hder.sound hx
  simp only [zero_dotProduct] at this
  linarith

variable {ι : Type*} [Fintype ι] [DecidableEq ι]

/-- A constraint derivable from the rows of `(A, b)` is `∑ λᵢ (Aᵢ, bᵢ)` with `λ ≥ 0`. -/
theorem Derivable.exists_coeffs (A : ι → V → K) (b : ι → K) {c : (V → K) × K}
    (h : Derivable (Set.range fun i => (A i, b i)) c) :
    ∃ μ : ι → K, (∀ i, 0 ≤ μ i) ∧ c.1 = ∑ i, μ i • A i ∧ c.2 = ∑ i, μ i * b i := by
  induction h with
  | mem hc =>
    obtain ⟨i, rfl⟩ := hc
    refine ⟨Pi.single i 1, fun k => ?_, ?_, ?_⟩
    · by_cases hk : k = i
      · subst hk; simp
      · simp [hk]
    · rw [Finset.sum_eq_single i (fun k _ hk => by simp [hk]) (by simp)]; simp
    · rw [Finset.sum_eq_single i (fun k _ hk => by simp [hk]) (by simp)]; simp
  | zero => exact ⟨0, fun _ => le_rfl, by simp, by simp⟩
  | add _ _ ih₁ ih₂ =>
    obtain ⟨μ, hμ, h1, h2⟩ := ih₁
    obtain ⟨ν, hν, h3, h4⟩ := ih₂
    refine ⟨μ + ν, fun i => add_nonneg (hμ i) (hν i), ?_, ?_⟩
    · simp only [Prod.fst_add, h1, h3, Pi.add_apply, add_smul, sum_add_distrib]
    · simp only [Prod.snd_add, h2, h4, Pi.add_apply, add_mul, sum_add_distrib]
  | smul t ht _ ih =>
    obtain ⟨μ, hμ, h1, h2⟩ := ih
    refine ⟨t • μ, fun i => mul_nonneg ht (hμ i), ?_, ?_⟩
    · simp only [Prod.smul_fst, h1, smul_sum, Pi.smul_apply, smul_eq_mul, mul_smul]
    · simp only [Prod.smul_snd, h2, smul_eq_mul, mul_sum, Pi.smul_apply, mul_assoc]

/-- **Farkas' lemma** (over any linearly ordered field). The system `Aᵢ ⬝ x ≤ bᵢ`
has no solution exactly when non-negative multipliers `μ` give `∑ μᵢ Aᵢ = 0`
and `∑ μᵢ bᵢ < 0`. -/
theorem farkas (A : ι → V → K) (b : ι → K) :
    (¬ ∃ x : V → K, ∀ i, A i ⬝ᵥ x ≤ b i) ↔
      ∃ μ : ι → K, (∀ i, 0 ≤ μ i) ∧ ∑ i, μ i • A i = 0 ∧ ∑ i, μ i * b i < 0 := by
  classical
  set S := univ.image fun i => (A i, b i) with hS
  have hSr : (S : Set ((V → K) × K)) = Set.range fun i => (A i, b i) := by
    rw [hS, coe_image, coe_univ, Set.image_univ]
  have hsol : ∀ x, Solves (S : Set ((V → K) × K)) x ↔ ∀ i, A i ⬝ᵥ x ≤ b i := by
    intro x; rw [hSr]
    exact ⟨fun h i => h _ ⟨i, rfl⟩, fun h _ ⟨i, hi⟩ => hi ▸ h i⟩
  constructor
  · intro hinf
    obtain ⟨β, hβ, hder⟩ := (infeasible_iff_derivable S).mp fun ⟨x, hx⟩ =>
      hinf ⟨x, (hsol x).mp hx⟩
    rw [hSr] at hder
    obtain ⟨μ, hμ, h1, h2⟩ := hder.exists_coeffs A b
    exact ⟨μ, hμ, h1.symm, h2 ▸ hβ⟩
  · rintro ⟨μ, hμ, h1, h2⟩ ⟨x, hx⟩
    have : ∑ i, μ i * (A i ⬝ᵥ x) ≤ ∑ i, μ i * b i :=
      sum_le_sum fun i _ => mul_le_mul_of_nonneg_left (hx i) (hμ i)
    have h0 : ∑ i, μ i * (A i ⬝ᵥ x) = (∑ i, μ i • A i) ⬝ᵥ x := by
      rw [sum_dotProduct]; simp only [smul_dotProduct, smul_eq_mul]
    rw [h0, h1, zero_dotProduct] at this
    linarith

/-! ### Remark "Sign-definite turns; Farkas certificates" -/

variable {κ α : Type*} [Fintype κ] [DecidableEq κ] [Fintype α] [DecidableEq α]

/-- **Farkas with lower bounds.** `M ℓ = 0`, `ℓ ≥ b` has no solution exactly when a
combination `c = yᵀ M` of the rows is a non-negative form with `c ⬝ b > 0`. -/
theorem farkas_bounds (M : Matrix κ α K) (b : α → K) :
    (¬ ∃ ℓ : α → K, M *ᵥ ℓ = 0 ∧ ∀ a, b a ≤ ℓ a) ↔
      ∃ y : κ → K, (∀ a, 0 ≤ (y ᵥ* M) a) ∧ 0 < (y ᵥ* M) ⬝ᵥ b := by
  constructor
  · intro hinf
    -- Rows `M_k ℓ ≤ 0`, `-M_k ℓ ≤ 0`, `-ℓ_a ≤ -b_a`.
    let A : κ ⊕ κ ⊕ α → α → K := Sum.elim (fun k => M k) (Sum.elim (fun k => -M k)
      (fun a => -Pi.single a 1))
    let β : κ ⊕ κ ⊕ α → K := Sum.elim 0 (Sum.elim 0 (fun a => -b a))
    have hA : ¬ ∃ x : α → K, ∀ i, A i ⬝ᵥ x ≤ β i := by
      rintro ⟨x, hx⟩
      refine hinf ⟨x, funext fun k => ?_, fun a => ?_⟩
      · have h1 := hx (Sum.inl k)
        have h2 := hx (Sum.inr (Sum.inl k))
        simp only [A, β, Sum.elim_inl, Sum.elim_inr, Pi.zero_apply, neg_dotProduct] at h1 h2
        simp only [Matrix.mulVec, Pi.zero_apply]
        exact le_antisymm h1 (by linarith)
      · have := hx (Sum.inr (Sum.inr a))
        simp only [A, β, Sum.elim_inr, neg_dotProduct, single_dotProduct, one_mul] at this
        linarith
    obtain ⟨μ, hμ, h1, h2⟩ := (farkas A β).mp hA
    refine ⟨fun k => μ (Sum.inl k) - μ (Sum.inr (Sum.inl k)), ?_⟩
    -- `yᵀ M = ν`, the multipliers of the bounds.
    have hc : ∀ a, ((fun k => μ (Sum.inl k) - μ (Sum.inr (Sum.inl k))) ᵥ* M) a =
        μ (Sum.inr (Sum.inr a)) := by
      intro a
      have := congrFun h1 a
      simp only [Fintype.sum_sum_type, Finset.sum_apply, Pi.smul_apply, smul_eq_mul, A,
        Sum.elim_inl, Sum.elim_inr, Pi.neg_apply, Pi.zero_apply, mul_neg, sum_neg_distrib] at this
      rw [Finset.sum_eq_single a (fun a' _ ha' => by simp [ha'.symm])
        (by simp)] at this
      simp only [Pi.single_eq_same, mul_one] at this
      simp only [vecMul, dotProduct, sub_mul, sum_sub_distrib]
      linarith
    refine ⟨fun a => by rw [hc]; exact hμ _, ?_⟩
    simp only [dotProduct, hc]
    simp only [Fintype.sum_sum_type, β, Sum.elim_inl, Sum.elim_inr, Pi.zero_apply, mul_zero,
      sum_const_zero, zero_add, mul_neg, sum_neg_distrib] at h2
    linarith
  · rintro ⟨y, hy, hpos⟩ ⟨ℓ, hℓ, hb⟩
    have h0 : (y ᵥ* M) ⬝ᵥ ℓ = 0 := by rw [← dotProduct_mulVec, hℓ, dotProduct_zero]
    have : (y ᵥ* M) ⬝ᵥ b ≤ (y ᵥ* M) ⬝ᵥ ℓ :=
      sum_le_sum fun a _ => mul_le_mul_of_nonneg_left (hb a) (hy a)
    linarith

variable {ρ η : Type*} [Fintype ρ] [DecidableEq ρ] [Fintype η] [DecidableEq η]

/-- **Remark "Sign-definite turns", Farkas certificates.** With balances `B` and the
shifts `H` of a set of turns, the system `B ℓ = 0`, `shift_γ(ℓ) = 0` for every
turn, `ℓ ≥ b` has no solution exactly when a combination `∑ u_γ shift_γ` of the
shifts equals, modulo the balances (`+ λᵀ B`), a form `c ≥ 0` with `c ⬝ b > 0`. -/
theorem sign_definite_certificate (B : Matrix ρ α K) (H : Matrix η α K) (b : α → K) :
    (¬ ∃ ℓ : α → K, B *ᵥ ℓ = 0 ∧ H *ᵥ ℓ = 0 ∧ ∀ a, b a ≤ ℓ a) ↔
      ∃ (u : η → K) (lam : ρ → K), (∀ a, 0 ≤ (u ᵥ* H + lam ᵥ* B) a) ∧
        0 < (u ᵥ* H + lam ᵥ* B) ⬝ᵥ b := by
  have hsys : (∃ ℓ : α → K, B *ᵥ ℓ = 0 ∧ H *ᵥ ℓ = 0 ∧ ∀ a, b a ≤ ℓ a) ↔
      ∃ ℓ : α → K, Matrix.fromRows B H *ᵥ ℓ = 0 ∧ ∀ a, b a ≤ ℓ a := by
    refine exists_congr fun ℓ => ?_
    rw [fromRows_mulVec]
    constructor
    · rintro ⟨h1, h2, h3⟩; exact ⟨by rw [h1, h2]; funext k; cases k <;> rfl, h3⟩
    · rintro ⟨h1, h3⟩
      refine ⟨funext fun r => ?_, funext fun γ => ?_, h3⟩
      · exact congrFun h1 (Sum.inl r)
      · exact congrFun h1 (Sum.inr γ)
  rw [hsys, farkas_bounds]
  constructor
  · rintro ⟨y, hy, hpos⟩
    refine ⟨fun γ => y (Sum.inr γ), fun r => y (Sum.inl r), ?_⟩
    have : (fun γ => y (Sum.inr γ)) ᵥ* H + (fun r => y (Sum.inl r)) ᵥ* B =
        y ᵥ* Matrix.fromRows B H := by
      funext a
      simp only [vecMul, dotProduct, Pi.add_apply, Fintype.sum_sum_type, fromRows_apply_inl,
        fromRows_apply_inr]
      ring
    rw [this]; exact ⟨hy, hpos⟩
  · rintro ⟨u, lam, hc, hpos⟩
    refine ⟨Sum.elim lam u, ?_⟩
    have : Sum.elim lam u ᵥ* Matrix.fromRows B H = u ᵥ* H + lam ᵥ* B := by
      funext a
      simp only [vecMul, dotProduct, Pi.add_apply, Fintype.sum_sum_type, fromRows_apply_inl,
        fromRows_apply_inr, Sum.elim_inl, Sum.elim_inr]
      ring
    rw [this]; exact ⟨hc, hpos⟩

/-- **Arcs forced to zero.** An arc `a` has length zero in every solution of
`M ℓ = 0`, `ℓ ≥ 0` exactly when some combination `c = yᵀ M` is a non-negative
form with `c_a > 0` (the implicit equalities of the cone). -/
theorem forced_zero_iff (M : Matrix κ α K) (a : α) :
    (∀ ℓ : α → K, M *ᵥ ℓ = 0 → (∀ a', 0 ≤ ℓ a') → ℓ a = 0) ↔
      ∃ y : κ → K, (∀ a', 0 ≤ (y ᵥ* M) a') ∧ 0 < (y ᵥ* M) a := by
  constructor
  · intro h
    have hinf : ¬ ∃ ℓ : α → K, M *ᵥ ℓ = 0 ∧ ∀ a', (Pi.single a (1 : K) : α → K) a' ≤ ℓ a' := by
      rintro ⟨ℓ, hℓ, hb⟩
      have hnn : ∀ a', 0 ≤ ℓ a' := fun a' => by
        have := hb a'
        by_cases ha : a' = a
        · subst ha; simp at this; linarith
        · simpa [ha] using this
      have h1 := hb a
      rw [Pi.single_eq_same, h ℓ hℓ hnn] at h1
      exact absurd h1 (by norm_num)
    obtain ⟨y, hy, hpos⟩ := (farkas_bounds M _).mp hinf
    rw [dotProduct_single, mul_one] at hpos
    exact ⟨y, hy, hpos⟩
  · rintro ⟨y, hy, hpos⟩ ℓ hℓ hnn
    have h0 : (y ᵥ* M) ⬝ᵥ ℓ = 0 := by rw [← dotProduct_mulVec, hℓ, dotProduct_zero]
    have hterm := (Finset.sum_eq_zero_iff_of_nonneg (fun a' _ => mul_nonneg (hy a') (hnn a'))).mp
      h0 a (mem_univ a)
    exact (mul_eq_zero.mp hterm).resolve_left hpos.ne'

/-! ### Whole multipliers, whole solutions -/

/-- Clearing denominators: a rational vector times a positive whole number is whole. -/
theorem clear_denominators {ι : Type*} [Fintype ι] (v : ι → ℚ) :
    ∃ N : ℕ, 0 < N ∧ ∃ z : ι → ℤ, ∀ i, (z i : ℚ) = N * v i := by
  classical
  refine ⟨∏ i, (v i).den, Finset.prod_pos fun i _ => (v i).den_pos, fun i =>
    (v i).num * ((∏ j, (v j).den) / (v i).den : ℕ), fun i => ?_⟩
  have hdvd : (v i).den ∣ ∏ j, (v j).den := Finset.dvd_prod_of_mem _ (mem_univ i)
  have hden : ((v i).den : ℚ) ≠ 0 := by exact_mod_cast (v i).den_ne_zero
  rw [Int.cast_mul, Int.cast_natCast, Nat.cast_div hdvd hden]
  conv_rhs => rw [← Rat.num_div_den (v i)]
  field_simp

/-- The cast of `yᵀ M` for whole `y` and `M`. -/
theorem cast_vecMul {L : Type*} [Ring L] (M : Matrix κ α ℤ) (y : κ → ℤ) (a : α) :
    (((y ᵥ* M) a : ℤ) : L) = ((fun k => (y k : L)) ᵥ* M.map (Int.cast : ℤ → L)) a := by
  simp [vecMul, dotProduct]

/-- **Whole multipliers.** For whole `M` and `b`, the system `M ℓ = 0`, `ℓ ≥ b` has no
real solution exactly when whole multipliers `y` make `c = yᵀ M ≥ 0` with
`c ⬝ b > 0`. -/
theorem farkas_int (M : Matrix κ α ℤ) (b : α → ℤ) :
    (¬ ∃ ℓ : α → ℝ, M.map (Int.cast : ℤ → ℝ) *ᵥ ℓ = 0 ∧ ∀ a, (b a : ℝ) ≤ ℓ a) ↔
      ∃ y : κ → ℤ, (∀ a, 0 ≤ (y ᵥ* M) a) ∧ 0 < (y ᵥ* M) ⬝ᵥ b := by
  constructor
  · intro hinf
    -- No real solution, hence no rational one.
    have hQ : ¬ ∃ ℓ : α → ℚ, M.map (Int.cast : ℤ → ℚ) *ᵥ ℓ = 0 ∧ ∀ a, (b a : ℚ) ≤ ℓ a := by
      rintro ⟨ℓ, hℓ, hb⟩
      refine hinf ⟨fun a => (ℓ a : ℝ), funext fun k => ?_, fun a => ?_⟩
      swap
      · show (b a : ℝ) ≤ (ℓ a : ℝ)
        exact_mod_cast hb a
      have := congrFun hℓ k
      simp only [Matrix.mulVec, dotProduct, Matrix.map_apply, Pi.zero_apply] at this ⊢
      have := congrArg (Rat.cast : ℚ → ℝ) this
      simpa only [Rat.cast_sum, Rat.cast_mul, Rat.cast_intCast, Rat.cast_zero] using this
    obtain ⟨y, hy, hpos⟩ := (farkas_bounds _ _).mp hQ
    obtain ⟨N, hN, z, hz⟩ := clear_denominators y
    have hzc : ∀ a, (((z ᵥ* M) a : ℤ) : ℚ) = N * (y ᵥ* M.map (Int.cast : ℤ → ℚ)) a := by
      intro a
      rw [cast_vecMul]
      simp only [hz, vecMul, dotProduct, mul_sum, Matrix.map_apply, mul_assoc]
    refine ⟨z, fun a => ?_, ?_⟩
    · have : (0 : ℚ) ≤ (((z ᵥ* M) a : ℤ) : ℚ) := by
        rw [hzc]; exact mul_nonneg (by positivity) (hy a)
      exact_mod_cast this
    · have : (0 : ℚ) < ((((z ᵥ* M) ⬝ᵥ b) : ℤ) : ℚ) := by
        have h1 : ((((z ᵥ* M) ⬝ᵥ b) : ℤ) : ℚ) =
            N * ((y ᵥ* M.map (Int.cast : ℤ → ℚ)) ⬝ᵥ fun a => (b a : ℚ)) := by
          simp only [dotProduct, Int.cast_sum, Int.cast_mul, hzc, mul_sum, mul_assoc]
        rw [h1]; exact mul_pos (by exact_mod_cast hN) hpos
      exact_mod_cast this
  · rintro ⟨y, hy, hpos⟩
    rw [farkas_bounds]
    refine ⟨fun k => (y k : ℝ), fun a => ?_, ?_⟩
    · rw [← cast_vecMul]; exact_mod_cast hy a
    · have : ((((y ᵥ* M) ⬝ᵥ b) : ℤ) : ℝ) =
          ((fun k => (y k : ℝ)) ᵥ* M.map (Int.cast : ℤ → ℝ)) ⬝ᵥ fun a => (b a : ℝ) := by
        simp only [dotProduct, Int.cast_sum, Int.cast_mul, cast_vecMul]
      rw [← this]; exact_mod_cast hpos

/-- **Remark "Whole solutions exist at some size".** For whole `M` and whole bounds
`b ≥ 0`, if `M ℓ = 0`, `ℓ ≥ b` has a real solution, it has a whole one: a rational
solution times the common denominator of its entries. -/
theorem whole_solution_of_real (M : Matrix κ α ℤ) (b : α → ℤ) (hb : ∀ a, 0 ≤ b a)
    (h : ∃ ℓ : α → ℝ, M.map (Int.cast : ℤ → ℝ) *ᵥ ℓ = 0 ∧ ∀ a, (b a : ℝ) ≤ ℓ a) :
    ∃ L : α → ℤ, M *ᵥ L = 0 ∧ ∀ a, b a ≤ L a := by
  -- A rational solution: otherwise a rational certificate, cast to the reals,
  -- would forbid the real one.
  have hQ : ∃ ℓ : α → ℚ, M.map (Int.cast : ℤ → ℚ) *ᵥ ℓ = 0 ∧ ∀ a, (b a : ℚ) ≤ ℓ a := by
    by_contra hQ
    obtain ⟨y, hy, hpos⟩ := (farkas_bounds _ _).mp hQ
    obtain ⟨N, hN, z, hz⟩ := clear_denominators y
    apply (farkas_int M b).mpr _ h
    have hzc : ∀ a, (((z ᵥ* M) a : ℤ) : ℚ) = N * (y ᵥ* M.map (Int.cast : ℤ → ℚ)) a := by
      intro a
      rw [cast_vecMul]
      simp only [hz, vecMul, dotProduct, mul_sum, Matrix.map_apply, mul_assoc]
    refine ⟨z, fun a => ?_, ?_⟩
    · have : (0 : ℚ) ≤ (((z ᵥ* M) a : ℤ) : ℚ) := by
        rw [hzc]; exact mul_nonneg (by positivity) (hy a)
      exact_mod_cast this
    · have : (0 : ℚ) < ((((z ᵥ* M) ⬝ᵥ b) : ℤ) : ℚ) := by
        have h1 : ((((z ᵥ* M) ⬝ᵥ b) : ℤ) : ℚ) =
            N * ((y ᵥ* M.map (Int.cast : ℤ → ℚ)) ⬝ᵥ fun a => (b a : ℚ)) := by
          simp only [dotProduct, Int.cast_sum, Int.cast_mul, hzc, mul_sum, mul_assoc]
        rw [h1]; exact mul_pos (by exact_mod_cast hN) hpos
      exact_mod_cast this
  obtain ⟨ℓ, hℓ, hbℓ⟩ := hQ
  obtain ⟨N, hN, L, hL⟩ := clear_denominators ℓ
  refine ⟨L, funext fun k => ?_, fun a => ?_⟩
  · have : (((M *ᵥ L) k : ℤ) : ℚ) = N * (M.map (Int.cast : ℤ → ℚ) *ᵥ ℓ) k := by
      simp only [Matrix.mulVec, dotProduct, Int.cast_sum, Int.cast_mul, hL, mul_sum,
        Matrix.map_apply]
      exact sum_congr rfl fun _ _ => by ring
    rw [hℓ] at this
    simpa using this
  · have h1 : (b a : ℚ) ≤ N * ℓ a := by
      have : (1 : ℚ) ≤ N := by exact_mod_cast hN
      have hb0 : (0 : ℚ) ≤ b a := by exact_mod_cast hb a
      nlinarith [hbℓ a]
    rw [← hL] at h1
    exact_mod_cast h1

end Farkas

end ClosingHelices
