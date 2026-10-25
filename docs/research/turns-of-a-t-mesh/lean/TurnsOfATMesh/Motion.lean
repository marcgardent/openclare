/-
Compositions of motions and the Burgers sum.

The note develops a loop patch after patch and gets a motion `x ↦ R x + t`
of the coordinates of its first patch. Everything here is stated for motions
`x ↦ a • x + t` (`Motion`, in `Basic.lean`) whose linear part `a` lies in an
arbitrary ring `A` acting on an additive group `V`: quarter-turns of the plane
(`A = V = ℂ` or `ℤ[i]`) are one instance, the rotations of a hexagonal or
triangular grid another, and nothing in this file uses that the rotations have
order four.

Conventions: `comp f g` is `f ∘ g` (`g` applied first), as in the note's
`ĥ(αβ) = ĥ(α) ∘ ĥ(β)`; `composeAll [ρ₁, …, ρₘ]` is `ρₘ ∘ ⋯ ∘ ρ₁` (the first
element of the list is applied first).
-/
module

public import TurnsOfATMesh.Basic
public import Mathlib.Algebra.BigOperators.Fin
public import Mathlib.Tactic.NoncommRing

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace Motion

variable {A V : Type*} [Ring A] [AddCommGroup V] [Module A V]

/-! ### Composing a list of motions -/

/-- `composeAll [f₁, …, fₘ] = fₘ ∘ ⋯ ∘ f₁`. -/
def composeAll : List (Motion A V) → Motion A V
  | [] => id
  | f :: fs => comp (composeAll fs) f

@[simp] theorem composeAll_nil : composeAll ([] : List (Motion A V)) = id := rfl
@[simp] theorem composeAll_cons (f : Motion A V) (fs : List (Motion A V)) :
    composeAll (f :: fs) = comp (composeAll fs) f := rfl

theorem composeAll_append (fs gs : List (Motion A V)) :
    composeAll (fs ++ gs) = comp (composeAll gs) (composeAll fs) := by
  induction fs with
  | nil => ext <;> simp [composeAll, comp, id]
  | cons f fs ih => simp [ih, comp_assoc]

/-- The linear part of `fₘ ∘ ⋯ ∘ f₁` is `lin fₘ * ⋯ * lin f₁`. -/
theorem composeAll_lin (fs : List (Motion A V)) :
    (composeAll fs).lin = (fs.map lin).reverse.prod := by
  induction fs with
  | nil => rfl
  | cons f fs ih => simp [ih]

/-- **Translation of a composition** (the algebraic core of Proposition
"The shift is set by the singularities"): the translation of `fₘ ∘ ⋯ ∘ f₁` is
`∑ᵢ Qᵢ • tᵢ`, where `tᵢ` is the translation of `fᵢ` and
`Qᵢ = lin fₘ * ⋯ * lin fᵢ₊₁` the product of the linear parts applied after it. -/
theorem composeAll_trans (fs : List (Motion A V)) :
    (composeAll fs).trans =
      ∑ i : Fin fs.length, ((fs.drop (i + 1)).map lin).reverse.prod • (fs.get i).trans := by
  induction fs with
  | nil => simp
  | cons f fs ih =>
    show _ = ∑ i : Fin (fs.length + 1), _
    rw [Fin.sum_univ_succ]
    simp only [composeAll_cons, comp_trans, ih, composeAll_lin, Fin.val_zero, Fin.val_succ,
      List.get_cons_zero, List.drop_succ_cons, zero_add, List.drop_zero]
    rw [add_comm]
    rfl

/-! ### The Burgers sum of a list of rotations -/

/-- The Burgers sum `B = ∑ᵢ Qᵢ (1 - Rᵢ) pᵢ`, `Qᵢ = Rₘ ⋯ Rᵢ₊₁`, of rotations `Rᵢ`
about points `pᵢ`, the list being in the order the rotations are applied. -/
def burgers : List (A × V) → V
  | [] => 0
  | c :: cs => ((cs.map Prod.fst).reverse.prod) • (c.2 - c.1 • c.2) + burgers cs

/-- The rotations `ρᵢ(x) = Rᵢ (x - pᵢ) + pᵢ` of a list of cone points. -/
def rotations (cs : List (A × V)) : List (Motion A V) := cs.map fun c => rotAbout c.1 c.2

/-- **Proposition "The shift is set by the singularities" (a), algebraic core.**
`ρₘ ∘ ⋯ ∘ ρ₁` has linear part `Rₘ ⋯ R₁` and translation the Burgers sum. -/
theorem composeAll_rotations (cs : List (A × V)) :
    composeAll (rotations cs) = ⟨(cs.map Prod.fst).reverse.prod, burgers cs⟩ := by
  induction cs with
  | nil => rfl
  | cons c cs ih =>
    simp only [rotations, List.map_cons, composeAll_cons] at ih ⊢
    rw [ih]
    ext <;> simp [burgers, add_comm]

/-- The same, in indexed form: `B = ∑ᵢ Qᵢ • (pᵢ - Rᵢ • pᵢ)`. -/
theorem burgers_eq_sum (cs : List (A × V)) :
    burgers cs = ∑ i : Fin cs.length,
      ((cs.drop (i + 1)).map Prod.fst).reverse.prod • ((cs.get i).2 - (cs.get i).1 • (cs.get i).2) := by
  induction cs with
  | nil => simp [burgers]
  | cons c cs ih =>
    show _ = ∑ i : Fin (cs.length + 1), _
    rw [Fin.sum_univ_succ]
    simp [burgers, ih]

/-- The indexed form for cone points given as the images of a list `l` by a map
`c`: `B = ∑ₖ Qₖ • (pₖ - Rₖ • pₖ)`, `(Rₖ, pₖ) = c lₖ`, `Qₖ = Rₘ ⋯ Rₖ₊₁`. -/
theorem burgers_map {ι : Type*} (c : ι → A × V) (l : List ι) :
    burgers (l.map c) = ∑ k : Fin l.length,
      ((l.drop (k + 1)).map fun i => (c i).1).reverse.prod • ((c l[k]).2 - (c l[k]).1 • (c l[k]).2) := by
  induction l with
  | nil => simp [burgers]
  | cons a l ih =>
    show _ = ∑ k : Fin (l.length + 1), _
    rw [Fin.sum_univ_succ]
    simp [burgers, ih, Function.comp_def]

/-- When the total rotation is trivial (Gauss–Bonnet, Lemma "Gauss–Bonnet for
turns"), the composition is the translation by the Burgers sum. -/
theorem composeAll_rotations_of_prod_eq_one (cs : List (A × V))
    (h : (cs.map Prod.fst).reverse.prod = 1) :
    composeAll (rotations cs) = translation (burgers cs) := by
  rw [composeAll_rotations, h]; rfl

/-- Moving every cone point by one translation `t` changes the Burgers sum by
`(1 - Rₘ ⋯ R₁) t`; when the total rotation is trivial it does not change it. -/
theorem burgers_translate (cs : List (A × V)) (t : V) :
    burgers (cs.map fun c => (c.1, c.2 + t)) =
      burgers cs + (t - (cs.map Prod.fst).reverse.prod • t) := by
  induction cs with
  | nil => simp [burgers]
  | cons c cs ih =>
    have hfst : ((cs.map fun c => (c.1, c.2 + t)).map Prod.fst) = cs.map Prod.fst := by
      simp [List.map_map, Function.comp_def]
    simp only [List.map_cons, burgers, hfst, ih, List.reverse_cons, List.prod_append,
      List.prod_cons, List.prod_nil, mul_one, smul_add, smul_sub, mul_smul]
    abel

/-! ### Orientation, annuli, exchanges -/

/-- Run the other way, a loop whose holonomy is the translation by `B` has the
translation by `-B` (the sign `±` of Proposition "The shift is set by the
singularities" (a)). -/
theorem translation_inv (B : V) :
    comp (translation B : Motion A V) (translation (-B)) = id ∧
      comp (translation (-B) : Motion A V) (translation B) = id := by
  constructor <;> ext <;> simp

/-- **Conjugating a translation.** `κ ∘ (x ↦ x + T) ∘ κ⁻¹` is the translation by
`M • T`, `M` the linear part of `κ`. -/
theorem conj_translation (κ : Motion A V) (u : Aˣ) (h : (u : A) = κ.lin) (T : V) :
    comp (comp κ (translation T)) (inv κ u h) = translation (κ.lin • T) := by
  ext
  · simp [inv, ← h]
  · simp [inv, ← h, smul_neg, ← mul_smul]

/-- **Proposition "The shift is set by the singularities" (b), algebraic core.**
If `ĥ(γ̄₁)` and `ĥ(γ̄₂)` are the translations by `T₁` and `T₂` and `ĥ(κ)` has linear
part `M`, the loop `λ = γ̄₁ κ γ̄₂⁻¹ κ⁻¹` has holonomy the translation by `T₁ - M T₂`:
the difference of the two translations read in the coordinates at the start of
`γ̄₁`. -/
theorem annulus_holonomy (T₁ T₂ : V) (κ : Motion A V) (u : Aˣ) (h : (u : A) = κ.lin) :
    comp (comp (comp (translation T₁) κ) (translation (-T₂))) (inv κ u h) =
      translation (T₁ - κ.lin • T₂) := by
  rw [comp_assoc, comp_assoc, ← comp_assoc κ, conj_translation]
  ext <;> simp [sub_eq_add_neg]

/-- **Proposition "Exchanging two vertices", algebraic core.** Two rotations by the
same `g` about `p` and `p'`: `ρ_p ∘ ρ_{p'}⁻¹` is the translation by
`(1 - g)(p - p')`. -/
theorem exchange (u : Aˣ) (p p' : V) :
    comp (rotAbout (u : A) p) (rotAbout (↑u⁻¹ : A) p') =
      translation (p - p' - (u : A) • (p - p')) := by
  ext
  · simp
  · simp [smul_sub, ← mul_smul]; abel

/-! ### The shift is linear in the lengths -/

/-- **Proposition "The shift of a turn is linear", algebraic core.** If the linear
parts of the crossing maps do not depend on the lengths `ℓ` and their
translations are additive in `ℓ`, so is the translation of their composition. -/
theorem composeAll_trans_add {L : Type*} [AddCommGroup L] (lins : List A)
    (ts : List (L →+ V)) (ℓ ℓ' : L) :
    (composeAll (List.zipWith (fun a t => (⟨a, t (ℓ + ℓ')⟩ : Motion A V)) lins ts)).trans =
      (composeAll (List.zipWith (fun a t => (⟨a, t ℓ⟩ : Motion A V)) lins ts)).trans +
      (composeAll (List.zipWith (fun a t => (⟨a, t ℓ'⟩ : Motion A V)) lins ts)).trans := by
  induction lins generalizing ts with
  | nil => simp
  | cons a lins ih =>
    cases ts with
    | nil => simp
    | cons t ts =>
      have hl : ∀ g : (L →+ V) → V,
          (List.zipWith (fun a t => (⟨a, g t⟩ : Motion A V)) lins ts).map lin =
            List.zipWith (fun a _ => a) lins ts := by
        intro g
        simp [List.map_zipWith]
      simp only [List.zipWith_cons_cons, composeAll_cons, comp_trans, ih, composeAll_lin]
      rw [hl (fun t => t (ℓ + ℓ')), hl (fun t => t ℓ), hl (fun t => t ℓ'), map_add, smul_add]
      abel

end Motion

end TurnsOfATMesh
