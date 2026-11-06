/-
Proposition "The Burgers pairing": on a closed surface of genus zero without
singular vertex of valence divisible by four, the forms of the twisted classes
span the dual of the balanced lengths.

The twisted homology `H₁(Σ°; ℝ²_ρ)` is read on the presentation of `π₁(Σ°)` by
the loops `λᵢ` round the singular vertices and one relation `ϱ`: a twisted
1-chain is a vector `W` of `ℂ = ℝ²` per generator (the twisted windings of
`Fox.lean`), a cycle when `∑ᵢ (1 - Rᵢ)ᵀ Wᵢ = 0` (`cycles`), a boundary when it
is `Wᵢ = ρ(∂ϱ/∂xᵢ)ᵀ f` (`boundaries`), and a class is a cycle modulo the
boundaries (`Homology`). The form of a class on the balanced lengths `ℓ ∈ L`
is `∑ᵢ ⟨Wᵢ, (1 - Rᵢ) pᵢ(ℓ)⟩`, the developed positions `pᵢ` linear in `ℓ`
(`pairing`). That this presentation is the fundamental group of the
punctured sphere, that the relation has trivial holonomy at balanced lengths,
and that every comb develops in the chart of the base point through loops
`λᵢ` (`Comb.Develops`) are the topological facts of the proof: hypotheses of
the theorems.
-/
module

public import TurnsOfATMesh.Consequences
public import Mathlib.LinearAlgebra.Dual.Lemmas
public import Mathlib.LinearAlgebra.Quotient.Basic
public import Mathlib.LinearAlgebra.Complex.Module

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace Pairing

open Complex ComplexConjugate Motion

variable {ι : Type*} [Fintype ι] [DecidableEq ι]

/-! ### Twisted cycles, boundaries and classes -/

/-- The twisted 1-cycles: `∑ᵢ (1 - Rᵢ)ᵀ Wᵢ = 0`, the transpose of the
multiplication by `z` being the multiplication by `conj z`. -/
def cycles (u : ι → ℂˣ) : Submodule ℝ (ι → ℂ) where
  carrier := {W | ∑ i, conj (1 - (u i : ℂ)) * W i = 0}
  add_mem' {W W'} hW hW' := by
    simp only [Set.mem_ofPred_eq, Pi.add_apply, mul_add, Finset.sum_add_distrib] at *
    rw [hW, hW', add_zero]
  zero_mem' := by simp
  smul_mem' c W hW := by
    simp only [Set.mem_ofPred_eq, Pi.smul_apply, Complex.real_smul] at *
    rw [show ∑ i, conj (1 - (u i : ℂ)) * ((c : ℂ) * W i) =
      (c : ℂ) * ∑ i, conj (1 - (u i : ℂ)) * W i by
        rw [Finset.mul_sum]; exact Finset.sum_congr rfl fun i _ => by ring, hW, mul_zero]

/-- The boundary of the 2-cell of the relation `ϱ` with coefficient `f`:
`Wᵢ = ρ(∂ϱ/∂xᵢ)ᵀ f`. -/
def boundaryMap (u : ι → ℂˣ) (ϱ : List (ι × Bool)) : ℂ →ₗ[ℝ] (ι → ℂ) where
  toFun f i := conj (fox u i ϱ) * f
  map_add' f f' := by ext i; simp [mul_add]
  map_smul' c f := by ext i; simp only [Pi.smul_apply, Complex.real_smul, RingHom.id_apply]; ring

/-- The twisted boundaries, the image of `boundaryMap`. -/
def boundaries (u : ι → ℂˣ) (ϱ : List (ι × Bool)) : Submodule ℝ (ι → ℂ) :=
  LinearMap.range (boundaryMap u ϱ)

/-- The twisted homology `H₁(Σ°; ℝ²_ρ)` of the presentation with relation `ϱ`:
the cycles modulo the boundaries. -/
abbrev Homology (u : ι → ℂˣ) (ϱ : List (ι × Bool)) : Type _ :=
  cycles u ⧸ (boundaries u ϱ).comap (cycles u).subtype

/-- A relation with trivial rotation has its boundaries among the cycles (the
fundamental formula of Fox calculus). -/
theorem boundaries_le_cycles (u : ι → ℂˣ) (ϱ : List (ι × Bool)) (hϱ : rhoW u ϱ = 1) :
    boundaries u ϱ ≤ cycles u := by
  rintro _ ⟨f, rfl⟩
  change ∑ i, conj (1 - (u i : ℂ)) * (conj (fox u i ϱ) * f) = 0
  have h := fox_fundamental (u := u) ϱ
  rw [hϱ, sub_self] at h
  have e : ∑ i, conj (1 - (u i : ℂ)) * (conj (fox u i ϱ) * f) =
      -(conj (∑ i, fox u i ϱ * ((u i : ℂ) - 1)) * f) := by
    rw [map_sum, Finset.sum_mul, ← Finset.sum_neg_distrib]
    refine Finset.sum_congr rfl fun i _ => ?_
    simp only [map_mul, map_sub, map_one]; ring
  rw [e, h, map_zero, zero_mul, neg_zero]

/-! ### The form of a class -/

/-- The form `∑ᵢ ⟨Wᵢ, (1 - Rᵢ) pᵢ⟩` of a twisted chain `W` at the developed
positions `p`. -/
def form (u : ι → ℂˣ) (W p : ι → ℂ) : ℝ := ∑ i, dotC (W i) (p i - (u i : ℂ) * p i)

variable {α : Type*}

/-- The form of a twisted chain on the balanced lengths `L`, the developed
position `pᵢ` of each singular vertex being linear in the lengths. -/
def formMap (u : ι → ℂˣ) (L : Submodule ℝ (α → ℝ)) (pos : ι → L →ₗ[ℝ] ℂ) :
    (ι → ℂ) →ₗ[ℝ] Module.Dual ℝ L :=
  LinearMap.mk₂ ℝ (fun W ℓ => form u W fun i => pos i ℓ)
    (fun W W' ℓ => by
      simp only [form, dotC, Pi.add_apply, map_add, add_mul, Complex.add_re,
        Finset.sum_add_distrib])
    (fun c W ℓ => by
      simp only [form, dotC, Pi.smul_apply, Complex.real_smul, map_mul, Complex.conj_ofReal,
        smul_eq_mul, Finset.mul_sum, mul_assoc, Complex.re_ofReal_mul])
    (fun W ℓ ℓ' => by
      simp only [form, dotC, map_add, ← Finset.sum_add_distrib, ← Complex.add_re]
      exact Finset.sum_congr rfl fun i _ => by ring_nf)
    (fun c W ℓ => by
      simp only [form, dotC, map_smul, Complex.real_smul, smul_eq_mul, Finset.mul_sum,
        ← Complex.re_ofReal_mul]
      exact Finset.sum_congr rfl fun i _ => by ring_nf)

theorem formMap_apply (u : ι → ℂˣ) (L : Submodule ℝ (α → ℝ)) (pos : ι → L →ₗ[ℝ] ℂ)
    (W : ι → ℂ) (ℓ : L) : formMap u L pos W ℓ = form u W fun i => pos i ℓ := rfl

/-- At balanced lengths the relation has trivial holonomy, so every boundary has
the zero form. -/
theorem boundaries_le_ker (u : ι → ℂˣ) (L : Submodule ℝ (α → ℝ)) (pos : ι → L →ₗ[ℝ] ℂ)
    (ϱ : List (ι × Bool)) (hrel : ∀ ℓ : L, hol u (fun i => pos i ℓ) ϱ = Motion.id) :
    boundaries u ϱ ≤ LinearMap.ker (formMap u L pos) := by
  rintro _ ⟨f, rfl⟩
  rw [LinearMap.mem_ker]
  refine LinearMap.ext fun ℓ => ?_
  rw [formMap_apply, LinearMap.zero_apply]
  have h := fox_burgers (u := u) (p := fun i => pos i ℓ) ϱ
  rw [hrel ℓ, Motion.id_trans] at h
  have e : form u (boundaryMap u ϱ f) (fun i => pos i ℓ) =
      (conj f * ∑ i, fox u i ϱ • (pos i ℓ - (u i : ℂ) • pos i ℓ)).re := by
    simp only [form, dotC, boundaryMap, LinearMap.coe_mk, AddHom.coe_mk, map_mul, conj_conj,
      Finset.mul_sum, Complex.re_sum, smul_eq_mul]
    exact Finset.sum_congr rfl fun i _ => by ring_nf
  rw [e, ← h, mul_zero, Complex.zero_re]

/-- **The Burgers pairing**: a twisted class to its form on the balanced lengths. -/
def pairing (u : ι → ℂˣ) (L : Submodule ℝ (α → ℝ)) (pos : ι → L →ₗ[ℝ] ℂ)
    (ϱ : List (ι × Bool)) (hrel : ∀ ℓ : L, hol u (fun i => pos i ℓ) ϱ = Motion.id) :
    Homology u ϱ →ₗ[ℝ] Module.Dual ℝ L :=
  Submodule.liftQ _ ((formMap u L pos).comp (cycles u).subtype) fun W hW => by
    rw [Submodule.mem_comap] at hW
    exact boundaries_le_ker u L pos ϱ hrel hW

theorem pairing_mk (u : ι → ℂˣ) (L : Submodule ℝ (α → ℝ)) (pos : ι → L →ₗ[ℝ] ℂ)
    (ϱ : List (ι × Bool)) (hrel : ∀ ℓ : L, hol u (fun i => pos i ℓ) ϱ = Motion.id)
    (W : cycles u) (ℓ : L) :
    pairing u L pos ϱ hrel (Submodule.Quotient.mk W) ℓ = form u W fun i => pos i ℓ := rfl

/-! ### Combs, developed in the chart of the base point -/

/-- The comb of an arc read in one chart: its feet `c₀`, `c₁`, the loops
`w₀`, `w₁` (words in the `λᵢ`) that carry the chart of the base point, where
`cⱼ` develops at `p_{cⱼ}`, to the chart of the comb, where it develops at
`ĥ(wⱼ) p_{cⱼ}`, and the direction `e_a` of the arc in that chart. -/
structure Comb (ι : Type*) where
  foot₀ : ι
  foot₁ : ι
  loop₀ : List (ι × Bool)
  loop₁ : List (ι × Bool)
  dir : ℂ

/-- The first foot developed in the chart of the comb. -/
def Comb.at₀ (C : Comb ι) (u : ι → ℂˣ) (p : ι → ℂ) : ℂ := (hol u p C.loop₀).apply (p C.foot₀)

/-- The second foot developed in the chart of the comb. -/
def Comb.at₁ (C : Comb ι) (u : ι → ℂˣ) (p : ι → ℂ) : ℂ := (hol u p C.loop₁).apply (p C.foot₁)

/-- The comb develops as in Lemma "Combs" at the length `len` of its arc: `e_a`
is a unit vector and the comb, from `c₀` to `c₁` in its chart, is a chain of
steps, one of them `len e_a` (the arc) and the others orthogonal to `e_a` (the
perpendicular traces, through no cone point). -/
def Comb.Develops (C : Comb ι) (u : ι → ℂˣ) (p : ι → ℂ) (len : ℝ) : Prop :=
  normSq C.dir = 1 ∧ ∃ (n : ℕ) (v : Fin n → ℂ) (k : Fin n), v k = len * C.dir ∧
    (∀ i, i ≠ k → dotC C.dir (v i) = 0) ∧ ∑ i, v i = C.at₁ u p - C.at₀ u p

/-- **Lemma "Combs", the identity, on a comb.** -/
theorem Comb.reading (C : Comb ι) (u : ι → ℂˣ) (p : ι → ℂ) (len : ℝ) (h : C.Develops u p len) :
    dotC C.dir (C.at₁ u p - C.at₀ u p) = len := by
  obtain ⟨he, n, v, k, hk, horth, hsum⟩ := h
  rw [← hsum]
  exact comb_reading C.dir he v k len hk horth

/-! ### Proposition "The Burgers pairing" -/

/-- A valence `k` not divisible by four gives a rotation `R = I^{4 - k}` other
than the identity. -/
theorem quarter_ne_one (k : ℤ) (hk : ¬ (4 : ℤ) ∣ k) : Complex.I ^ (4 - k) ≠ 1 := by
  intro h
  have hnonneg := Int.emod_nonneg (4 - k) (by norm_num : (4 : ℤ) ≠ 0)
  have hdecomp : 4 - k = 4 * ((4 - k) / 4) + (((4 - k) % 4).toNat : ℤ) := by
    rw [Int.toNat_of_nonneg hnonneg]; omega
  obtain ⟨m, hm⟩ : ∃ m, ((4 - k) % 4).toNat = m := ⟨_, rfl⟩
  have hlt : m < 4 := by omega
  rw [hdecomp, zpow_add₀ Complex.I_ne_zero, zpow_mul, show Complex.I ^ (4 : ℤ) = 1 by
    rw [show (4 : ℤ) = ((4 : ℕ) : ℤ) by rfl, zpow_natCast, Complex.I_pow_four], one_zpow,
    one_mul, zpow_natCast, hm] at h
  interval_cases m
  · omega
  · simp [Complex.ext_iff] at h
  · rw [Complex.I_sq] at h; norm_num at h
  · rw [pow_succ, Complex.I_sq] at h
    simp [Complex.ext_iff] at h

/-- The twisted class of the commutator `λₐ λ_b λₐ⁻¹ λ_b⁻¹` with the direction `f`:
`Wᵢ = ρ(∂w/∂xᵢ)ᵀ f`. -/
def commutatorCycle (u : ι → ℂˣ) (a b : ι) (f : ℂ) : ι → ℂ :=
  twistedWinding {()} u (fun _ => 1) (fun _ => f) (fun _ => commutatorWord a b)

theorem commutatorCycle_mem (u : ι → ℂˣ) (a b : ι) (f : ℂ) :
    commutatorCycle u a b f ∈ cycles u :=
  twisted_relation {()} u (fun _ => 0) _ _ _ fun _ _ => by
    rw [commutator_holonomy, translation_lin]

/-- **Proposition "Winding numbers do not suffice", as a form.** The form of the
class of the commutator is `⟨f, (1 - Rₐ)(1 - R_b)(pₐ - p_b)⟩`. -/
theorem form_commutatorCycle (u : ι → ℂˣ) (a b : ι) (f : ℂ) (p : ι → ℂ) :
    form u (commutatorCycle u a b f) p =
      dotC f ((1 - (u a : ℂ)) * (1 - (u b : ℂ)) * (p a - p b)) := by
  have h := fox_burgers_combination {()} u p (fun _ => 1) (fun _ => f)
    (fun _ => commutatorWord a b)
  rw [Finset.sum_singleton, one_mul, commutator_holonomy, translation_trans, smul_eq_mul] at h
  exact h.symm

/-- `⟨1, z⟩` and `⟨i, z⟩` are the two coordinates of `z`. -/
theorem eq_zero_of_dotC (z : ℂ) (h1 : dotC 1 z = 0) (hI : dotC Complex.I z = 0) : z = 0 := by
  simp only [dotC, map_one, one_mul, Complex.conj_I, neg_mul, Complex.neg_re,
    Complex.mul_re, Complex.I_re, Complex.I_im, zero_mul, one_mul, zero_sub, neg_neg] at h1 hI
  exact Complex.ext h1 hI

/-- When every cone point is at `q`, every holonomy fixes `q`. -/
theorem hol_apply_of_const (u : ι → ℂˣ) (p : ι → ℂ) (q : ℂ) (hp : ∀ i, p i = q)
    (w : List (ι × Bool)) : (hol u p w).apply q = q := by
  induction w with
  | nil => simp [Motion.apply, Motion.id]
  | cons x w ih => rw [hol_cons, apply_comp, ih, apply_rotAbout, hp, sub_self, smul_zero, zero_add]

/-- **Proposition "The Burgers pairing", onto, for rotations other than the
identity.** The singular vertices `ι` have rotations `Rᵢ ≠ I`; the twisted
homology is that of the presentation of `π₁(Σ°)` by the loops `λᵢ` with the
relation `ϱ`, of trivial holonomy at the balanced lengths `L` (`hrel`); and
every arc has a comb that develops in the chart of the base point through
loops `λᵢ` (`hcomb`: the surface is a sphere, so the `λᵢ` generate, and every
trace starts at a singular vertex, so every arc has feet). Then every linear
form on the balanced lengths is the form of a twisted class. -/
theorem pairing_surjective_of_ne_one [Fintype α] (u : ι → ℂˣ) (hu : ∀ i, (u i : ℂ) ≠ 1)
    (L : Submodule ℝ (α → ℝ)) (pos : ι → L →ₗ[ℝ] ℂ) (ϱ : List (ι × Bool))
    (hrel : ∀ ℓ : L, hol u (fun i => pos i ℓ) ϱ = Motion.id) (comb : α → Comb ι)
    (hcomb : ∀ (a : α) (ℓ : L), (comb a).Develops u (fun i => pos i ℓ) ((ℓ : α → ℝ) a)) :
    Function.Surjective (pairing u L pos ϱ hrel) := by
  -- A length on which every form vanishes is zero.
  have key : ∀ ℓ : L, (∀ W : cycles u, form u W (fun i => pos i ℓ) = 0) → ℓ = 0 := by
    intro ℓ hℓ
    set p : ι → ℂ := fun i => pos i ℓ
    -- The commutators put all the singular vertices at one point.
    have hsame : ∀ a b, p a = p b := by
      intro a b
      have hz : (1 - (u a : ℂ)) * (1 - (u b : ℂ)) * (p a - p b) = 0 := by
        refine eq_zero_of_dotC _ ?_ ?_
        · rw [← form_commutatorCycle]; exact hℓ ⟨_, commutatorCycle_mem u a b 1⟩
        · rw [← form_commutatorCycle]; exact hℓ ⟨_, commutatorCycle_mem u a b Complex.I⟩
      have ha : (1 : ℂ) - u a ≠ 0 := sub_ne_zero.mpr (hu a).symm
      have hb : (1 : ℂ) - u b ≠ 0 := sub_ne_zero.mpr (hu b).symm
      exact sub_eq_zero.mp ((mul_eq_zero.mp hz).resolve_left (mul_ne_zero ha hb))
    -- Every comb then reads zero.
    apply Subtype.ext
    funext a
    have hr := (comb a).reading u p _ (hcomb a ℓ)
    have hq : ∀ i, p i = p (comb a).foot₀ := fun i => hsame i _
    rw [Comb.at₁, Comb.at₀, hq (comb a).foot₁, hol_apply_of_const u p _ hq,
      hol_apply_of_const u p _ hq, sub_self] at hr
    rw [← hr]
    simp [dotC]
  -- Hence the image has no common zero but `0`, and is everything.
  have hco : (LinearMap.range (pairing u L pos ϱ hrel)).dualCoannihilator = ⊥ := by
    rw [eq_bot_iff]
    intro ℓ hℓ
    rw [Submodule.mem_dualCoannihilator] at hℓ
    rw [Submodule.mem_bot]
    exact key ℓ fun W => hℓ _ ⟨Submodule.Quotient.mk W, rfl⟩
  rw [← LinearMap.range_eq_top, ← Subspace.dualCoannihilator_dualAnnihilator_eq (W := LinearMap.range _),
    hco, Submodule.dualAnnihilator_bot]

/-- **Proposition "The Burgers pairing", onto.** On a closed surface of genus
zero, with no singular vertex of valence `kᵢ` divisible by four (the rotation
round `cᵢ` being `Rᵢ = I^{4 - kᵢ}`), and every trace starting at a singular
vertex (hypotheses as in `pairing_surjective_of_ne_one`), the Burgers pairing
is onto the dual of the balanced lengths. -/
theorem pairing_surjective [Fintype α] (u : ι → ℂˣ) (k : ι → ℤ)
    (hu : ∀ i, (u i : ℂ) = Complex.I ^ (4 - k i)) (hk : ∀ i, ¬ (4 : ℤ) ∣ k i)
    (L : Submodule ℝ (α → ℝ)) (pos : ι → L →ₗ[ℝ] ℂ) (ϱ : List (ι × Bool))
    (hrel : ∀ ℓ : L, hol u (fun i => pos i ℓ) ϱ = Motion.id) (comb : α → Comb ι)
    (hcomb : ∀ (a : α) (ℓ : L), (comb a).Develops u (fun i => pos i ℓ) ((ℓ : α → ℝ) a)) :
    Function.Surjective (pairing u L pos ϱ hrel) :=
  pairing_surjective_of_ne_one u (fun i => by rw [hu]; exact quarter_ne_one _ (hk i)) L pos ϱ
    hrel comb hcomb

/-- **Proposition "The Burgers pairing", valence divisible by four.** A singular
vertex with `Rᵢ = I` has `(I - Rᵢ) pᵢ = 0`: no form depends on its developed
position. -/
theorem form_update_of_eq_one (u : ι → ℂˣ) (W p : ι → ℂ) (i : ι) (hi : (u i : ℂ) = 1) (q : ℂ) :
    form u W (Function.update p i q) = form u W p := by
  refine Finset.sum_congr rfl fun j _ => ?_
  by_cases hj : j = i
  · subst hj; simp [hi]
  · rw [Function.update_of_ne hj]

end Pairing

end TurnsOfATMesh
