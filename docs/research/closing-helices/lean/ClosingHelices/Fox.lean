/-
The Fox–Burgers formula (Proposition "Fox–Burgers formula") and Proposition
"Winding numbers do not suffice".

Words in the letters `xᵢ^{±1}` are lists of `(i, b)`, `b = true` for `xᵢ` and
`false` for `xᵢ⁻¹`, as in Mathlib's `FreeGroup`. The letter `xᵢ` is mapped to
the rotation about `pᵢ` by the unit `uᵢ` (the holonomy of the loop `λᵢ`), so a
word has a holonomy, a motion. The Fox derivative `∂w/∂xᵢ` is used only through
its image `ρ(∂w/∂xᵢ)` in the ring `A`, which is what the formula needs; it is
defined by the rules `∂xⱼ/∂xᵢ = δᵢⱼ`, `∂xⱼ⁻¹/∂xᵢ = -δᵢⱼ xⱼ⁻¹` and
`∂(uv)/∂xᵢ = ∂u/∂xᵢ + u ∂v/∂xᵢ`. The augmentation `ε(∂w/∂xᵢ)` is the exponent
sum `expSum i w`.

The ring `A` and the module `V` are arbitrary, so the formula holds for any
rotations (not only quarter-turns) and in any dimension.
-/
module

public import ClosingHelices.Motion
public import Mathlib.GroupTheory.FreeGroup.Basic
public import Mathlib.Algebra.BigOperators.Ring.Finset
public import Mathlib.Basic.Complex.BigOperators

@[expose] public section

set_option linter.unusedSectionVars false

namespace ClosingHelices

open Motion

variable {ι A V : Type*} [Ring A] [AddCommGroup V] [Module A V]
variable (u : ι → Aˣ) (p : ι → V)

/-- The linear part of a letter: `uᵢ` or `uᵢ⁻¹`. -/
def letterLin : ι × Bool → A
  | (i, true) => u i
  | (i, false) => ↑(u i)⁻¹

/-- The holonomy of a word: `ĥ(x w) = ĥ(x) ∘ ĥ(w)`, a letter being the rotation by
`uᵢ^{±1}` about `pᵢ` (the inverse of a rotation about `p` is the inverse
rotation about `p`, `Motion.inv_rotAbout`). -/
def hol : List (ι × Bool) → Motion A V
  | [] => Motion.id
  | x :: w => comp (rotAbout (letterLin u x) (p x.1)) (hol w)

/-- The rotation `ρ(w)` of a word, the product of the linear parts of its letters. -/
def rhoW (w : List (ι × Bool)) : A := (w.map (letterLin u)).prod

/-- The image `ρ(∂w/∂xᵢ)` of the Fox derivative. -/
def fox [DecidableEq ι] (i : ι) : List (ι × Bool) → A
  | [] => 0
  | (j, b) :: w =>
      (if j = i then (if b then 1 else -↑(u i)⁻¹) else 0) + letterLin u (j, b) * fox i w

/-- The exponent sum of `xᵢ` in `w`, which is the augmentation `ε(∂w/∂xᵢ)`. -/
def expSum [DecidableEq ι] (i : ι) : List (ι × Bool) → ℤ
  | [] => 0
  | (j, b) :: w => (if j = i then (if b then 1 else -1) else 0) + expSum i w

variable {u p}

@[simp] theorem hol_nil : hol u p [] = Motion.id := rfl
@[simp] theorem hol_cons (x : ι × Bool) (w : List (ι × Bool)) :
    hol u p (x :: w) = comp (rotAbout (letterLin u x) (p x.1)) (hol u p w) := rfl

@[simp] theorem rhoW_nil : rhoW u ([] : List (ι × Bool)) = 1 := rfl
@[simp] theorem rhoW_cons (x : ι × Bool) (w : List (ι × Bool)) :
    rhoW u (x :: w) = letterLin u x * rhoW u w := by simp [rhoW]
theorem rhoW_append (w w' : List (ι × Bool)) : rhoW u (w ++ w') = rhoW u w * rhoW u w' := by
  simp [rhoW]

/-- The linear part of the holonomy of a word is its rotation `ρ(w)`. -/
theorem hol_lin (w : List (ι × Bool)) : (hol u p w).lin = rhoW u w := by
  induction w with
  | nil => rfl
  | cons x w ih => simp [ih]

theorem hol_append (w w' : List (ι × Bool)) :
    hol u p (w ++ w') = comp (hol u p w) (hol u p w') := by
  induction w with
  | nil => ext <;> simp [comp, Motion.id]
  | cons x w ih => simp [ih, comp_assoc]

/-! ### Words and compositions of rotations

`hol (x :: w)` applies the head `x` last, `composeAll (f :: fs)` applies it first:
the holonomy of a word is the composition of the rotations of its letters taken
from the right. -/

/-- The holonomy of a word is `composeAll` of the rotations of its letters,
reversed. -/
theorem hol_eq_composeAll (w : List (ι × Bool)) :
    hol u p w = composeAll ((w.map fun x => rotAbout (letterLin u x) (p x.1)).reverse) := by
  induction w with
  | nil => rfl
  | cons x w ih => simp [ih, composeAll_append]

/-- The same with the cone points `(Rᵢ, pᵢ)` of the letters, read from the right:
the word `xₘ ⋯ x₁` is `ρₘ ∘ ⋯ ∘ ρ₁`. -/
theorem hol_eq_rotations (w : List (ι × Bool)) :
    hol u p w = composeAll (rotations ((w.map fun x => (letterLin u x, p x.1)).reverse)) := by
  rw [hol_eq_composeAll, rotations, List.map_reverse, List.map_map]
  rfl

/-- The translation of the holonomy of a word is the Burgers sum of its letters,
read from the right. -/
theorem hol_trans_eq_burgers (w : List (ι × Bool)) :
    (hol u p w).trans = burgers ((w.map fun x => (letterLin u x, p x.1)).reverse) := by
  rw [hol_eq_rotations, composeAll_rotations]

/-- The word `xₘ ⋯ x₁` of a list `[i₁, …, iₘ]` of indices, each letter positive: its
head is the last element of the list. -/
def simpleWord (l : List ι) : List (ι × Bool) := (l.map fun i => (i, true)).reverse

/-- The holonomy of `xₘ ⋯ x₁` is `ρₘ ∘ ⋯ ∘ ρ₁`, the composition of Proposition "The
shift is set by the singularities" (a) with the cone points `(Rᵢ, pᵢ)` in the
order of the list. -/
theorem hol_simpleWord (l : List ι) :
    hol u p (simpleWord l) = composeAll (rotations (l.map fun i => ((u i : A), p i))) := by
  rw [hol_eq_rotations, simpleWord, List.map_reverse, List.reverse_reverse, List.map_map]
  rfl

theorem rhoW_simpleWord (l : List ι) :
    rhoW u (simpleWord l) = (l.map fun i => (u i : A)).reverse.prod := by
  simp [rhoW, simpleWord, List.map_reverse, Function.comp_def, letterLin]

theorem simpleWord_append (a b : List ι) :
    simpleWord (a ++ b) = simpleWord b ++ simpleWord a := by
  simp [simpleWord]

section Fox

variable [DecidableEq ι]

@[simp] theorem fox_nil (i : ι) : fox u i [] = 0 := rfl

theorem fox_append (i : ι) (w w' : List (ι × Bool)) :
    fox u i (w ++ w') = fox u i w + rhoW u w * fox u i w' := by
  induction w with
  | nil => simp
  | cons x w ih =>
    obtain ⟨j, b⟩ := x
    simp only [List.cons_append, fox, ih, rhoW_cons, mul_add, mul_assoc]
    abel

/-- **Proposition "Fox–Burgers formula", first identity.** For every word `w`,
`τ(w) = ∑ᵢ ρ(∂w/∂xᵢ) (1 - Rᵢ) pᵢ`. -/
theorem fox_burgers [Fintype ι] (w : List (ι × Bool)) :
    (hol u p w).trans = ∑ i, fox u i w • (p i - (u i : A) • p i) := by
  induction w with
  | nil => simp [Motion.id]
  | cons x w ih =>
    obtain ⟨j, b⟩ := x
    simp only [hol_cons, comp_trans, rotAbout_trans, ih, fox, add_smul, Finset.sum_add_distrib,
      mul_smul, ← Finset.smul_sum]
    congr 1
    rw [Finset.sum_eq_single j (fun i _ hij => by simp [Ne.symm hij]) (by simp)]
    cases b <;> (simp [letterLin, smul_sub, ← mul_smul]; try abel)

/-- **Fundamental formula of Fox calculus**, under `ρ`:
`∑ᵢ ρ(∂w/∂xᵢ) (Rᵢ - 1) = ρ(w) - 1`. -/
theorem fox_fundamental [Fintype ι] (w : List (ι × Bool)) :
    ∑ i, fox u i w * ((u i : A) - 1) = rhoW u w - 1 := by
  induction w with
  | nil => simp
  | cons x w ih =>
    obtain ⟨j, b⟩ := x
    simp only [fox, add_mul, Finset.sum_add_distrib, mul_assoc, ← Finset.mul_sum, ih, rhoW_cons]
    rw [Finset.sum_eq_single j (fun i _ hij => by simp [Ne.symm hij]) (by simp)]
    cases b <;> (simp [letterLin, mul_sub]; try abel)

/-- A letter followed by its inverse changes neither the holonomy nor the Fox
derivatives: they are defined on the free group. -/
theorem hol_cancel (x : ι × Bool) (w : List (ι × Bool)) :
    hol u p (x :: (x.1, !x.2) :: w) = hol u p w := by
  obtain ⟨j, b⟩ := x
  have key : comp (rotAbout (letterLin u (j, b)) (p j))
      (rotAbout (letterLin u (j, !b)) (p j)) = (Motion.id : Motion A V) := by
    cases b
    · have := comp_inv (rotAbout (↑(u j)⁻¹ : A) (p j)) ((u j)⁻¹) rfl
      rw [inv_rotAbout, inv_inv] at this
      simpa [letterLin] using this
    · have := comp_inv (rotAbout (u j : A) (p j)) (u j) rfl
      rw [inv_rotAbout] at this
      simpa [letterLin] using this
  simp only [hol_cons]
  rw [← comp_assoc, key]
  ext <;> simp [comp, Motion.id]

theorem fox_cancel (i : ι) (x : ι × Bool) (w : List (ι × Bool)) :
    fox u i (x :: (x.1, !x.2) :: w) = fox u i w := by
  obtain ⟨j, b⟩ := x
  cases b <;> by_cases h : j = i <;>
    simp [fox, letterLin, h, mul_add, ← mul_assoc]

theorem hol_step {w w' : List (ι × Bool)} (h : FreeGroup.Red.Step w w') :
    hol u p w = hol u p w' := by
  cases h with
  | not => rw [hol_append, hol_append, hol_cancel]

theorem fox_step (i : ι) {w w' : List (ι × Bool)} (h : FreeGroup.Red.Step w w') :
    fox u i w = fox u i w' := by
  cases h with
  | not =>
    rename_i L₁ L₂ x b
    rw [fox_append, fox_append]
    have := fox_cancel (u := u) i (x, b) L₂
    simp only at this
    rw [this]

/-- The holonomy as a function on the free group. -/
def holFG (u : ι → Aˣ) (p : ι → V) : FreeGroup ι → Motion A V :=
  Quot.lift (hol u p) fun _ _ h => hol_step h

/-- The Fox derivative, under `ρ`, as a function on the free group. -/
def foxFG (u : ι → Aˣ) (i : ι) : FreeGroup ι → A :=
  Quot.lift (fox u i) fun _ _ h => fox_step i h

/-! ### Inserting a conjugate of the relation -/

/-- The inverse word. -/
def invWord (w : List (ι × Bool)) : List (ι × Bool) :=
  (w.map fun x => (x.1, !x.2)).reverse

theorem invWord_cons (x : ι × Bool) (g : List (ι × Bool)) :
    invWord (x :: g) = invWord g ++ [(x.1, !x.2)] := by
  simp [invWord]

theorem cons_invWord (x : ι × Bool) (g w : List (ι × Bool)) :
    x :: g ++ invWord (x :: g) ++ w = x :: (g ++ invWord g ++ ((x.1, !x.2) :: w)) := by
  simp [invWord_cons]

theorem hol_mul_invWord (g w : List (ι × Bool)) :
    hol u p (g ++ invWord g ++ w) = hol u p w := by
  induction g generalizing w with
  | nil => rfl
  | cons x g ih => rw [cons_invWord, hol_cons, ih, ← hol_cons, hol_cancel]

theorem rhoW_mul_invWord (g : List (ι × Bool)) : rhoW u g * rhoW u (invWord g) = 1 := by
  have := congrArg Motion.lin (hol_mul_invWord (u := u) (p := fun _ => (0 : A)) g [])
  rw [List.append_nil, hol_append, comp_lin, hol_lin, hol_lin] at this
  exact this

theorem fox_mul_invWord (i : ι) (g w : List (ι × Bool)) :
    fox u i (g ++ invWord g ++ w) = fox u i w := by
  induction g generalizing w with
  | nil => rfl
  | cons x g ih =>
    rw [cons_invWord]
    obtain ⟨j, b⟩ := x
    have h1 := fox_cancel (u := u) i (j, b) w
    simp only [fox] at h1 ⊢
    rw [ih]
    exact h1

/-! ### The Fox–Burgers formula is the Burgers sum -/

/-- The Fox–Burgers sum of a word is the Burgers sum of its letters, read from the
right: both are the translation of its holonomy. -/
theorem fox_burgers_eq_burgers [Fintype ι] (w : List (ι × Bool)) :
    ∑ i, fox u i w • (p i - (u i : A) • p i) =
      burgers ((w.map fun x => (letterLin u x, p x.1)).reverse) := by
  rw [← fox_burgers, hol_trans_eq_burgers]

theorem fox_eq_zero_of_forall_ne (i : ι) (w : List (ι × Bool)) (h : ∀ x ∈ w, x.1 ≠ i) :
    fox u i w = 0 := by
  induction w with
  | nil => rfl
  | cons x w ih =>
    obtain ⟨j, b⟩ := x
    have hj : j ≠ i := h (j, b) (by simp)
    simp [fox, hj, ih fun y hy => h y (by simp [hy])]

/-- In `xₘ ⋯ x₁`, with `i` once in the list, `ρ(∂w/∂xᵢ)` is the rotation of the
letters left of `xᵢ`, `Rₘ ⋯ Rₖ₊₁`. -/
theorem fox_simpleWord_split (a b : List ι) (i : ι) (ha : i ∉ a) (hb : i ∉ b) :
    fox u i (simpleWord (a ++ i :: b)) = (b.map fun j => (u j : A)).reverse.prod := by
  have hzero : ∀ l : List ι, i ∉ l → fox u i (simpleWord l) = 0 := fun l hl =>
    fox_eq_zero_of_forall_ne i _ fun x hx => by
      simp only [simpleWord, List.mem_reverse, List.mem_map] at hx
      obtain ⟨j, hj, rfl⟩ := hx
      exact fun h => hl (h ▸ hj)
  rw [show a ++ i :: b = a ++ [i] ++ b by simp, simpleWord_append, simpleWord_append,
    fox_append, fox_append, hzero b hb, hzero a ha, rhoW_simpleWord]
  simp [simpleWord, fox]

/-- **The Fox–Burgers formula on `xₘ ⋯ x₁`, the coefficients.** For a list
`[i₁, …, iₘ]` of distinct indices, `ρ(∂(xₘ ⋯ x₁)/∂x_{iₖ}) = Qₖ = Rₘ ⋯ Rₖ₊₁`, the
coefficient of the `k`-th cone point in `Motion.burgers_eq_sum`. -/
theorem fox_simpleWord (l : List ι) (hl : l.Nodup) (k : ℕ) (hk : k < l.length) :
    fox u l[k] (simpleWord l) = ((l.drop (k + 1)).map fun i => (u i : A)).reverse.prod := by
  have hsplit : l = l.take k ++ l[k] :: l.drop (k + 1) := by
    rw [← List.drop_eq_getElem_cons hk, List.take_append_drop]
  have hnd := hl
  rw [hsplit, List.nodup_append, List.nodup_cons] at hnd
  have ha : l[k] ∉ l.take k := fun h => hnd.2.2 _ h _ (List.mem_cons_self) rfl
  have h := fox_simpleWord_split (u := u) _ _ _ ha hnd.2.1.1
  rwa [← hsplit] at h

/-- **The Fox–Burgers formula on `xₘ ⋯ x₁` is Proposition "The shift is set by the
singularities" (a).** The Fox–Burgers sum of the word is the Burgers sum
`∑ₖ Qₖ (pₖ - Rₖ pₖ)` of the cone points in the order of the list (`Motion.burgers_map`),
`Qₖ = Rₘ ⋯ Rₖ₊₁`; by `fox_simpleWord`, `Qₖ` is the image of the Fox derivative
when the indices are distinct. -/
theorem fox_burgers_simple [Fintype ι] (l : List ι) :
    ∑ i, fox u i (simpleWord l) • (p i - (u i : A) • p i) =
      ∑ k : Fin l.length, ((l.drop (k + 1)).map fun i => (u i : A)).reverse.prod •
        (p l[k] - (u l[k] : A) • p l[k]) := by
  rw [← fox_burgers, hol_simpleWord, composeAll_rotations, burgers_map]

/-- Inserting a conjugate `g ϱ g⁻¹` of a relation with trivial holonomy does not
change the holonomy. -/
theorem hol_insert_relation (a g ϱ b : List (ι × Bool)) (hϱ : hol u p ϱ = Motion.id) :
    hol u p (a ++ (g ++ ϱ ++ invWord g) ++ b) = hol u p (a ++ b) := by
  have hgg : comp (hol u p g) (hol u p (invWord g)) = Motion.id := by
    have := hol_mul_invWord (u := u) (p := p) g []
    rwa [List.append_nil, hol_append] at this
  simp only [hol_append, hϱ, comp_id, hgg]

/-- **Inserting a conjugate `g ϱ g⁻¹` of a relation `ϱ` with trivial holonomy**
(the paragraph after the Fox–Burgers formula). Inserted after a prefix `a` of the
word, it leaves the holonomy unchanged and adds `ρ(a) ρ(g) ρ(∂ϱ/∂xᵢ)` to the image
of each Fox derivative. The note writes `ρ(g ∂ϱ/∂xᵢ)`, which is the case of an
insertion at the front (`a` empty); an insertion after `a` is the insertion of
the conjugate by `a g` at the front. -/
theorem insert_relation (i : ι) (a g ϱ b : List (ι × Bool)) (hϱ : hol u p ϱ = Motion.id) :
    hol u p (a ++ (g ++ ϱ ++ invWord g) ++ b) = hol u p (a ++ b) ∧
      fox u i (a ++ (g ++ ϱ ++ invWord g) ++ b) =
        fox u i (a ++ b) + rhoW u a * rhoW u g * fox u i ϱ := by
  have hrϱ : rhoW u ϱ = 1 := by rw [← hol_lin (p := p), hϱ]; rfl
  have hfg : fox u i g + rhoW u g * fox u i (invWord g) = 0 := by
    have := fox_mul_invWord (u := u) i g []
    rwa [List.append_nil, fox_append] at this
  have hrg := rhoW_mul_invWord (u := u) g
  refine ⟨hol_insert_relation a g ϱ b hϱ, ?_⟩
  simp only [fox_append, rhoW_append, hrϱ, mul_one, hrg]
  have : fox u i g + rhoW u g * fox u i ϱ + rhoW u g * fox u i (invWord g) =
      rhoW u g * fox u i ϱ := by
    rw [add_right_comm, hfg, zero_add]
  rw [this, mul_assoc]
  abel

/-- **The Fox–Burgers sum does not change** when a conjugate of a relation with
trivial holonomy is inserted, although the Fox derivatives do (`insert_relation`):
both sums are the translation of the same holonomy. -/
theorem fox_sum_insert_relation [Fintype ι] (a g ϱ b : List (ι × Bool))
    (hϱ : hol u p ϱ = Motion.id) :
    ∑ i, fox u i (a ++ (g ++ ϱ ++ invWord g) ++ b) • (p i - (u i : A) • p i) =
      ∑ i, fox u i (a ++ b) • (p i - (u i : A) • p i) := by
  rw [← fox_burgers, ← fox_burgers, hol_insert_relation a g ϱ b hϱ]

/-! ### Proposition "Winding numbers do not suffice" -/

/-- The commutator word `xₐ x_b xₐ⁻¹ x_b⁻¹`. -/
def commutatorWord (a b : ι) : List (ι × Bool) := [(a, true), (b, true), (a, false), (b, false)]

theorem expSum_commutator (a b i : ι) : expSum i (commutatorWord a b) = 0 := by
  by_cases ha : a = i <;> by_cases hb : b = i <;> simp [commutatorWord, expSum, ha, hb]

end Fox

/-- **Proposition "Winding numbers do not suffice", holonomy of the commutator.**
When the two rotations commute, `λₐ λ_b λₐ⁻¹ λ_b⁻¹` is the translation by
`(1 - Rₐ)(1 - R_b)(pₐ - p_b)`. -/
theorem commutator_holonomy {A V : Type*} [CommRing A] [AddCommGroup V] [Module A V]
    [DecidableEq ι] (u : ι → Aˣ) (p : ι → V) (a b : ι) :
    hol u p (commutatorWord a b) =
      translation (((1 - (u a : A)) * (1 - (u b : A))) • (p a - p b)) := by
  have ha : (u a : A) * ↑(u a)⁻¹ = 1 := Units.mul_inv _
  have hb : (u b : A) * ↑(u b)⁻¹ = 1 := Units.mul_inv _
  ext
  · simp only [commutatorWord, hol_cons, hol_nil, comp_lin, rotAbout_lin, letterLin,
      Motion.id_lin, translation_lin, mul_one]
    calc (u a : A) * (u b * (↑(u a)⁻¹ * ↑(u b)⁻¹))
        = ((u a : A) * ↑(u a)⁻¹) * ((u b : A) * ↑(u b)⁻¹) := by ring
      _ = 1 := by rw [ha, hb, one_mul]
  · simp only [commutatorWord, hol_cons, hol_nil, comp_trans, rotAbout_lin, rotAbout_trans,
      letterLin, Motion.id_trans, translation_trans, smul_zero, add_zero]
    have e1 : (u a : A) * (u b * ↑(u a)⁻¹) = u b := by
      rw [mul_left_comm, ha, mul_one]
    have e2 : (u a : A) * (u b * (↑(u a)⁻¹ * ↑(u b)⁻¹)) = 1 := by
      rw [← mul_assoc (u b : A), ← mul_assoc, e1, hb]
    simp only [smul_sub, smul_add, ← mul_smul, sub_smul, mul_sub, sub_mul, one_mul, mul_one,
      e1, e2, one_smul]
    abel

/-- In the plane, with odd valences (`R = ±I`), the commutator has translation of
squared length `4 |pₐ - p_b|²` while its exponent sums are all zero, like those
of the empty word, whose translation is zero: no function of the winding numbers
gives the translation of the loops with trivial rotation. -/
theorem winding_numbers_do_not_suffice [DecidableEq ι] (u : ι → ℂˣ) (p : ι → ℂ) (a b : ι)
    (hua : (u a : ℂ) = Complex.I ∨ (u a : ℂ) = -Complex.I)
    (hub : (u b : ℂ) = Complex.I ∨ (u b : ℂ) = -Complex.I) :
    Complex.normSq (hol u p (commutatorWord a b)).trans = 4 * Complex.normSq (p a - p b) ∧
      (∀ i, expSum i (commutatorWord a b) = expSum i []) ∧
      (hol u p ([] : List (ι × Bool))).trans = 0 := by
  refine ⟨?_, fun i => by rw [expSum_commutator]; rfl, rfl⟩
  rw [commutator_holonomy, translation_trans, smul_eq_mul, Complex.normSq_mul,
    Complex.normSq_mul]
  have h : ∀ z : ℂ, (z = Complex.I ∨ z = -Complex.I) → Complex.normSq (1 - z) = 2 := by
    rintro z (rfl | rfl) <;> simp [Complex.normSq_apply] <;> norm_num
  rw [h _ hua, h _ hub]; ring

/-! ### Twisted windings in the plane -/

section Twisted

open Complex ComplexConjugate

variable [DecidableEq ι] [Fintype ι]

/-- `⟨f, v⟩`, the Euclidean product of the plane. -/
def dotC (f v : ℂ) : ℝ := (conj f * v).re

/-- The twisted winding `Wᵢ = ∑ᵣ cᵣ ρ(∂wᵣ/∂xᵢ)ᵀ fᵣ` of a weighted cycle, the
transpose of multiplication by `z` being multiplication by `conj z`. -/
def twistedWinding {κ : Type*} (s : Finset κ) (u : ι → ℂˣ) (c : κ → ℝ) (f : κ → ℂ)
    (w : κ → List (ι × Bool)) (i : ι) : ℂ :=
  ∑ r ∈ s, (c r : ℂ) * (conj (fox u i (w r)) * f r)

/-- **Proposition "Fox–Burgers formula", combinations.** For multipliers `cᵣ`,
words `wᵣ` and directions `fᵣ`,
`∑ᵣ cᵣ ⟨fᵣ, τ(wᵣ)⟩ = ∑ᵢ ⟨Wᵢ, (1 - Rᵢ) pᵢ⟩`. -/
theorem fox_burgers_combination {κ : Type*} (s : Finset κ) (u : ι → ℂˣ) (p : ι → ℂ)
    (c : κ → ℝ) (f : κ → ℂ) (w : κ → List (ι × Bool)) :
    ∑ r ∈ s, c r * dotC (f r) (hol u p (w r)).trans =
      ∑ i, dotC (twistedWinding s u c f w i) (p i - (u i : ℂ) * p i) := by
  simp only [dotC, twistedWinding, fox_burgers, smul_eq_mul, map_sum, map_mul, conj_conj,
    conj_ofReal, Finset.mul_sum, Finset.sum_mul, re_sum]
  rw [Finset.sum_comm]
  refine Finset.sum_congr rfl fun i _ => Finset.sum_congr rfl fun r _ => ?_
  rw [← re_ofReal_mul]
  congr 1
  ring

/-- **Inserting the relation in one word of a combination.** Replacing the word
`a b` of the turn `r₀` by `a g ϱ g⁻¹ b`, `ϱ` with trivial holonomy, changes the
twisted windings `Wᵢ` but not `∑ᵢ ⟨Wᵢ, (1 - Rᵢ) pᵢ⟩`. -/
theorem combination_insert_relation {κ : Type*} [DecidableEq κ] (s : Finset κ) (u : ι → ℂˣ)
    (p : ι → ℂ) (c : κ → ℝ) (f : κ → ℂ) (w : κ → List (ι × Bool)) (r₀ : κ)
    (a g ϱ b : List (ι × Bool)) (hw : w r₀ = a ++ b) (hϱ : hol u p ϱ = Motion.id) :
    ∑ i, dotC (twistedWinding s u c f (Function.update w r₀ (a ++ (g ++ ϱ ++ invWord g) ++ b)) i)
        (p i - (u i : ℂ) * p i) =
      ∑ i, dotC (twistedWinding s u c f w i) (p i - (u i : ℂ) * p i) := by
  rw [← fox_burgers_combination, ← fox_burgers_combination]
  refine Finset.sum_congr rfl fun r _ => ?_
  by_cases h : r = r₀
  · subst h
    rw [Function.update_self, hol_insert_relation a g ϱ b hϱ, hw]
  · rw [Function.update_of_ne h]

/-- **Proposition "Fox–Burgers formula", second identity.** If every word has
trivial rotation, `∑ᵢ (1 - Rᵢ)ᵀ Wᵢ = 0`. -/
theorem twisted_relation {κ : Type*} (s : Finset κ) (u : ι → ℂˣ) (p : ι → ℂ)
    (c : κ → ℝ) (f : κ → ℂ) (w : κ → List (ι × Bool))
    (hrot : ∀ r ∈ s, (hol u p (w r)).lin = 1) :
    ∑ i, conj (1 - (u i : ℂ)) * twistedWinding s u c f w i = 0 := by
  simp only [twistedWinding, Finset.mul_sum]
  rw [Finset.sum_comm]
  refine Finset.sum_eq_zero fun r hr => ?_
  have h := fox_fundamental (u := u) (w r)
  rw [← hol_lin (p := p), hrot r hr, sub_self] at h
  have h' : ∑ i, conj (fox u i (w r)) * conj (1 - (u i : ℂ)) =
      -conj (∑ i, fox u i (w r) * ((u i : ℂ) - 1)) := by
    rw [map_sum, ← Finset.sum_neg_distrib]
    refine Finset.sum_congr rfl fun i _ => ?_
    simp only [map_mul, map_sub, map_one]; ring
  rw [h, map_zero, neg_zero] at h'
  calc ∑ i, conj (1 - (u i : ℂ)) * ((c r : ℂ) * (conj (fox u i (w r)) * f r))
      = (c r : ℂ) * (∑ i, conj (fox u i (w r)) * conj (1 - (u i : ℂ))) * f r := by
        rw [Finset.mul_sum, Finset.sum_mul]
        refine Finset.sum_congr rfl fun i _ => ?_
        ring
    _ = 0 := by rw [h']; ring

/-- Hence the combination does not change when all the `pᵢ` are moved by one
translation `t`: it is a combination of separations. -/
theorem combination_translate {κ : Type*} (s : Finset κ) (u : ι → ℂˣ) (p : ι → ℂ)
    (c : κ → ℝ) (f : κ → ℂ) (w : κ → List (ι × Bool))
    (hrot : ∀ r ∈ s, (hol u p (w r)).lin = 1) (t : ℂ) :
    ∑ i, dotC (twistedWinding s u c f w i) ((p i + t) - (u i : ℂ) * (p i + t)) =
      ∑ i, dotC (twistedWinding s u c f w i) (p i - (u i : ℂ) * p i) := by
  have h := twisted_relation s u p c f w hrot
  have hsplit : ∀ i, dotC (twistedWinding s u c f w i) ((p i + t) - (u i : ℂ) * (p i + t)) =
      dotC (twistedWinding s u c f w i) (p i - (u i : ℂ) * p i) +
        (conj (conj (1 - (u i : ℂ)) * twistedWinding s u c f w i) * t).re := by
    intro i
    simp only [dotC, map_mul, conj_conj, ← add_re]
    congr 1
    ring
  simp only [hsplit, Finset.sum_add_distrib]
  rw [← re_sum, ← Finset.sum_mul, ← map_sum, h, map_zero, zero_mul, zero_re, add_zero]

end Twisted

end ClosingHelices
