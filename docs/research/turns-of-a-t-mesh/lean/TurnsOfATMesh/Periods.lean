/-
Period terms. The Fox–Burgers formula for any generators (Proposition "Burgers
sum with borders and handles": a border loop or a handle loop is a letter whose
motion is not a rotation about a cone point); the commutator of two motions
whose linear parts commute (the handle term of that proposition, and
Proposition "Two crossing turns on a torus"); and collars (Proposition
"Collars": a turn conjugate to a border loop has shift zero and the border's
length).
-/
module

public import TurnsOfATMesh.Fox
public import TurnsOfATMesh.QuarterTurn

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

open Motion

/-! ### The Fox–Burgers formula for any generators -/

section General

variable {ι A V : Type*} [Ring A] [AddCommGroup V] [Module A V]
variable (u : ι → Aˣ) (t : ι → V)

/-- The letter `xᵢ` as the motion `x ↦ uᵢ x + tᵢ` (the rotation about a cone
point `pᵢ` when `tᵢ = pᵢ - uᵢ pᵢ`, the translation by `Lⱼ` along an axis for a
border loop, any motion for a handle loop), and `xᵢ⁻¹` as its inverse. -/
def letterMotion : ι × Bool → Motion A V
  | (i, true) => ⟨u i, t i⟩
  | (i, false) => ⟨(↑(u i)⁻¹ : A), -((↑(u i)⁻¹ : A) • t i)⟩

/-- The holonomy of a word in any generators, the head of the word applied
last, as `hol`. -/
def holGen : List (ι × Bool) → Motion A V
  | [] => Motion.id
  | x :: w => comp (letterMotion u t x) (holGen w)

@[simp] theorem holGen_nil : holGen u t [] = Motion.id := rfl
@[simp] theorem holGen_cons (x : ι × Bool) (w : List (ι × Bool)) :
    holGen u t (x :: w) = comp (letterMotion u t x) (holGen u t w) := rfl

theorem letterMotion_lin (x : ι × Bool) : (letterMotion u t x).lin = letterLin u x := by
  obtain ⟨i, b⟩ := x
  cases b <;> rfl

/-- With `tᵢ = pᵢ - uᵢ pᵢ` the generators are the rotations about the cone
points and `holGen` is the `hol` of the Fox–Burgers formula. -/
theorem holGen_rotations (p : ι → V) (w : List (ι × Bool)) :
    holGen u (fun i => p i - (u i : A) • p i) w = hol u p w := by
  induction w with
  | nil => rfl
  | cons x w ih =>
    obtain ⟨i, b⟩ := x
    rw [holGen_cons, hol_cons, ih]
    congr 1
    cases b
    · ext
      · rfl
      · simp [letterMotion, letterLin, rotAbout, smul_sub, ← mul_smul]
    · rfl

/-- **Proposition "Burgers sum with borders and handles", algebraic core.** For
any generators, `τ(w) = ∑ᵢ ρ(∂w/∂xᵢ) τ(xᵢ)`: both sides satisfy
`φ(uv) = φ(u) + ρ(u) φ(v)` and agree on the letters. The cone points give the
Burgers sum, the other generators the period term. -/
theorem fox_general [DecidableEq ι] [Fintype ι] (w : List (ι × Bool)) :
    (holGen u t w).trans = ∑ i, fox u i w • t i := by
  induction w with
  | nil => simp [Motion.id]
  | cons x w ih =>
    obtain ⟨j, b⟩ := x
    simp only [holGen_cons, comp_trans, ih, fox, add_smul, Finset.sum_add_distrib, mul_smul,
      ← Finset.smul_sum, letterMotion_lin]
    congr 1
    rw [Finset.sum_eq_single j (fun i _ hij => by simp [Ne.symm hij]) (by simp)]
    cases b <;> simp [letterMotion]

end General

/-! ### Commutators: the handle term, and two crossing turns on a torus -/

section Commutator

variable {A V : Type*} [CommRing A] [AddCommGroup V] [Module A V]

/-- **The commutator of two motions with commuting linear parts** `x ↦ A x + a`
and `x ↦ B x + b` is the translation by `(1 - B) a - (1 - A) b` (the handle term
of Proposition "Burgers sum with borders and handles"). -/
theorem commutator_motion (f g : Motion A V) (uf ug : Aˣ) (hf : (uf : A) = f.lin)
    (hg : (ug : A) = g.lin) :
    comp (comp (comp f g) (inv f uf hf)) (inv g ug hg) =
      translation ((f.trans - g.lin • f.trans) - (g.trans - f.lin • g.trans)) := by
  have h1 : (↑uf * ↑ug * ↑uf⁻¹ : A) = ↑ug := by rw [mul_right_comm, Units.mul_inv, one_mul]
  have h2 : (↑uf * ↑ug * ↑uf⁻¹ * ↑ug⁻¹ : A) = 1 := by rw [h1, Units.mul_inv]
  ext
  · simp [inv, ← hf, ← hg, h2]
  · simp only [comp_trans, comp_lin, inv, ← hf, ← hg, smul_neg, ← mul_smul, h1, Units.mul_inv,
      one_smul, translation_trans]
    abel

/-- **Proposition "Two crossing turns on a torus", the general case.** If a simple
loop `κ` crossing the curve of a turn once has rotation `R^j`, the commutator of
the turn's translation `T` and `κ` is the translation by `(1 - R^j) T`: for
`j ≠ 0` the translation of the turn is set by the singular vertices inside the
cut disc, for `j = 0` it is a free period. -/
theorem commutator_translation (T : V) (g : Motion A V) (ug : Aˣ) (hg : (ug : A) = g.lin) :
    comp (comp (comp (translation T) g) (inv (translation T) 1 rfl)) (inv g ug hg) =
      translation (T - g.lin • T) := by
  refine (commutator_motion (translation T) g 1 ug rfl hg).trans ?_
  simp

/-- Two translations commute: the border of the disc cut along two crossing
turns of a torus has trivial holonomy, and the Burgers sum of the disc is zero. -/
theorem commutator_translations (S T : V) :
    comp (comp (comp (translation S) (translation T)) (inv (translation S) 1 rfl))
      (inv (translation T) 1 rfl) = (Motion.id : Motion A V) := by
  refine (commutator_motion (translation S) (translation T) 1 1 rfl rfl).trans ?_
  ext <;> simp [Motion.id]

end Commutator

/-! ### Collars -/

section Collar

open Complex

/-- The product of two of `1, I, -1, -I` is one of them. -/
theorem quarter_mul_quarter (a b : ℂ) (ha : a = 1 ∨ a = I ∨ a = -1 ∨ a = -I)
    (hb : b = 1 ∨ b = I ∨ b = -1 ∨ b = -I) :
    a * b = 1 ∨ a * b = I ∨ a * b = -1 ∨ a * b = -I := by
  rcases ha with rfl | rfl | rfl | rfl <;> rcases hb with rfl | rfl | rfl | rfl <;>
    simp [I_mul_I]

/-- **Proposition "Collars", algebraic core.** A border component without corner
has holonomy the translation by its length `L` along an axis `v` of the field.
A turn whose curve bounds with it an annulus without cone point has holonomy
conjugate to it by the motion `κ` along the cut: the translation by `κ.lin (L v)`,
along an axis. Read in the frame of the turn, with `α > 0` its length along the
walk (positive lengths), the axis is `e₁`: the shift is zero and `α = L`. -/
theorem collar (κ : Motion ℂ ℂ) (u : ℂˣ) (h : (u : ℂ) = κ.lin)
    (hκ : κ.lin = 1 ∨ κ.lin = I ∨ κ.lin = -1 ∨ κ.lin = -I)
    (L : ℝ) (hL : 0 < L) (v : ℂ) (hv : v = 1 ∨ v = I ∨ v = -1 ∨ v = -I) (α s : ℝ)
    (hT : (comp (comp κ (translation (L * v))) (inv κ u h)).trans = ⟨α, s⟩) (hα : 0 < α) :
    s = 0 ∧ α = L := by
  rw [conj_translation, translation_trans, smul_eq_mul, mul_left_comm] at hT
  rcases quarter_mul_quarter _ _ hκ hv with hw | hw | hw | hw <;> rw [hw] at hT <;>
    simp [Complex.ext_iff] at hT <;>
    exact ⟨by linarith [hT.1, hT.2], by linarith [hT.1, hT.2]⟩

end Collar

end TurnsOfATMesh
