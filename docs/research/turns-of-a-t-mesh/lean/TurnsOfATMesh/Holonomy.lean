/-
The development of a turn in the plane, on the `TMesh` at whole lengths
(DT-134, third brick): the change of chart across an arc as a motion of
`ℤ[i]`, the plane of a patch at whole lengths, and the composite of these
motions along the path of a turn, `Turn.development`. Section "Flat metric and
holonomy" of the note: in the frame `(e₁, e₂)` of the first side, `e₁` the
walking direction into the patch (`inward`) and `e₂` the direction of
increasing offsets along the side (`acrossDir`), the crossings develop the last
patch into the first and compose to `T_γ(ℓ) = -α_γ(ℓ) e₁ + shift_γ(ℓ) e₂`, the
inverse of the holonomy `ĥ(γ̄)` of the closed curve of the turn.

The frame of a patch is `(u, v)`, `u` along the sides of the pair `0`, `v` along
those of the pair `1`, as `TMesh.lean` says, read in `ℤ[i]` as `1` and `gi`; a
node at `(i, j)` quads is the Gaussian integer `⟨i, j⟩`. A side `σ` starts at its
`origin` and runs along `acrossDir σ`; `inward σ` points into the patch.

Crossing the arc `a` from the side `τ` of the patch `P` to the side `τ'` of the
patch `Q`: the same point of the surface has coordinates `x` in the frame of `P`
and `(crossMotion ℓ τ a).apply x` in the frame of `Q`. Its linear part is the
quarter-turn taking `inward τ` to `-inward τ'` (the walk leaves `P` through `τ`
and enters `Q` through `τ'`); on the side it is the crossing of the offsets of
`TMesh.lean` (`crossAt`), `x ↦ ε x + c` from the first corner of `τ` to the
first corner of `τ'`.

Everything here is a definition; the theorems (that the linear part takes
`acrossDir τ` to `sign a • acrossDir τ'` on an oriented T-mesh, and that the
development of a turn is the translation by `-α e₁ + shift e₂`) come in
`Development.lean`, and `Check/Main.lean` compares the development with the
holonomy of the words of the turns in the loops round the cone points.
-/
module

public import TurnsOfATMesh.Grid

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### The frame of a side -/

/-- The direction of increasing offsets along a side: `+u` on the pair `0`, `+v`
on the pair `1`. -/
def acrossDir (σ : Side Patch) : GaussianInt := if σ.2.1 = 0 then 1 else gi

/-- The walking direction from a side into its patch: `+v` from `v = 0`, `-v` from
`v = h`, `+u` from `u = 0`, `-u` from `u = w`. -/
def inward (σ : Side Patch) : GaussianInt :=
  if σ.2.1 = 0 then (if σ.2.2 = 0 then gi else -gi) else (if σ.2.2 = 0 then 1 else -1)

/-- The first corner of a side in the frame of its patch at the whole lengths `ℓ`:
`(0, 0)` for `v = 0` and `u = 0`, `(0, h)` for `v = h`, `(w, 0)` for `u = w`. -/
def origin (ℓ : α → ℕ) (σ : Side Patch) : GaussianInt :=
  if σ.2.1 = 0 then (if σ.2.2 = 0 then 0 else ⟨0, T.height ℓ σ.1⟩)
  else (if σ.2.2 = 0 then 0 else ⟨T.width ℓ σ.1, 0⟩)

/-- The conjugate, the inverse of a quarter-turn. -/
def conj (z : GaussianInt) : GaussianInt := ⟨z.re, -z.im⟩

/-! ### The change of chart across an arc -/

/-- The linear part of the change of chart from the side `τ` to the side `τ'`:
the quarter-turn taking `inward τ` to `-inward τ'`. -/
def crossLin (τ τ' : Side Patch) : GaussianInt := -inward τ' * conj (inward τ)

/-- The translation of the crossing of the offsets at whole lengths, as an integer:
`crossAt` read in `ℤ`. From the side `τ` across the arc `a`, the offset `x` from
the first corner of `τ` becomes `sign a · x + crossTrans`. -/
def crossTrans (ℓ : α → ℕ) (τ : Side Patch) (a : α) : ℤ :=
  if T.sign a = 1 then (posNat a (T.arcs (T.other a τ)) ℓ : ℤ) - posNat a (T.arcs τ) ℓ
  else (posNat a (T.arcs τ) ℓ : ℤ) + posNat a (T.arcs (T.other a τ)) ℓ + ℓ a

/-- **The change of chart across the arc `a` from the side `τ`**, from the frame of
the patch of `τ` to the frame of the patch of the other side `τ'`: the point of
`τ` at the offset `x`, `origin τ + x · acrossDir τ`, goes to the point of `τ'`
at the offset `sign a · x + crossTrans`, and the linear part turns `inward τ`
into `-inward τ'`. -/
def crossMotion (ℓ : α → ℕ) (τ : Side Patch) (a : α) : Motion GaussianInt GaussianInt :=
  let τ' := T.other a τ
  let L := crossLin τ τ'
  ⟨L, T.origin ℓ τ' + (T.crossTrans ℓ τ a : GaussianInt) * acrossDir τ' - L * T.origin ℓ τ⟩

/-- The changes of chart along a path of arcs from the side `σ`, the first applied
first, as `crossings` lists the crossings of the offsets: across the patch to
`σ.opp`, then across the arc. -/
def crossMotions (ℓ : α → ℕ) : Side Patch → List α → List (Motion GaussianInt GaussianInt)
  | _, [] => []
  | σ, a :: l => T.crossMotion ℓ σ.opp a :: crossMotions ℓ (T.other a σ.opp) l

/-- **The development of a turn** at the whole lengths `ℓ`: the composite of the
changes of chart along its path, which develops the last patch, the first one
again, into the first: `T_γ(ℓ)` of the note. -/
def Turn.development {T : TMesh Patch α} (γ : T.Turn) (ℓ : α → ℕ) :
    Motion GaussianInt GaussianInt :=
  Motion.composeAll (T.crossMotions ℓ γ.start γ.path)

/-! ### What is proved here -/

theorem inward_opp (σ : Side Patch) : inward σ.opp = -inward σ := by
  obtain ⟨P, j, s⟩ := σ
  fin_cases j <;> fin_cases s <;> simp [inward, Side.opp]

theorem acrossDir_opp (σ : Side Patch) : acrossDir σ.opp = acrossDir σ := by
  obtain ⟨P, j, s⟩ := σ
  fin_cases j <;> fin_cases s <;> simp [acrossDir, Side.opp]

theorem isQuarter_inward (σ : Side Patch) : IsQuarter (inward σ) := by
  unfold inward IsQuarter
  split_ifs <;> simp

theorem isQuarter_acrossDir (σ : Side Patch) : IsQuarter (acrossDir σ) := by
  unfold acrossDir IsQuarter
  split_ifs <;> simp

/-- The conjugate of a quarter-turn is its inverse. -/
theorem mul_conj_of_isQuarter {z : GaussianInt} (hz : IsQuarter z) : z * conj z = 1 := by
  rcases hz with rfl | rfl | rfl | rfl <;> ext <;> simp [conj, gi]

/-- The linear part of a change of chart takes `inward τ` to `-inward τ'`. -/
theorem crossLin_inward (τ τ' : Side Patch) : crossLin τ τ' * inward τ = -inward τ' := by
  rw [crossLin, mul_assoc, mul_comm (conj _), mul_conj_of_isQuarter (isQuarter_inward τ),
    mul_one]

end TMesh

end TurnsOfATMesh
