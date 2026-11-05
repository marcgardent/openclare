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
  sorry

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
  sorry

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
  sorry

/-! ### Proposition "The Burgers pairing" -/

/-- A valence `k` not divisible by four gives a rotation `R = I^{4 - k}` other
than the identity. -/
theorem quarter_ne_one (k : ℤ) (hk : ¬ (4 : ℤ) ∣ k) : Complex.I ^ (4 - k) ≠ 1 := by
  sorry

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
  sorry

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
    Function.Surjective (pairing u L pos ϱ hrel) := by
  sorry

/-- **Proposition "The Burgers pairing", valence divisible by four.** A singular
vertex with `Rᵢ = I` has `(I - Rᵢ) pᵢ = 0`: no form depends on its developed
position. -/
theorem form_update_of_eq_one (u : ι → ℂˣ) (W p : ι → ℂ) (i : ι) (hi : (u i : ℂ) = 1) (q : ℂ) :
    form u W (Function.update p i q) = form u W p := by
  sorry

end Pairing

end TurnsOfATMesh
