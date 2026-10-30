/-
The sixth brick of DT-134: from the holonomy of the border to the jumps across
the cuts.

A region of the fine grid is developed along a dual spanning tree of its faces:
each face `f` gets a chart `M f`, a motion from the frame of its patch to the
frame of the root, and across each tree edge the charts of the two faces agree
(`M g ∘ C = M f` for the change of chart `C` from `f` to `g`). The chart
continued along the border (`Border.lean`) crosses edges of the region one after
the other, each from a face `f` to a face `g`; seen through the charts, a
crossing is the jump `M g ∘ C ∘ (M f)⁻¹` of the frame of the root, the identity
across a tree edge. The telescope `Motion.composeAll_telescope` then reads the
holonomy of the border as the composite, in border order, of the jumps across
the cuts, the edges of the region off the tree: the `G_f` of the note.

Here: the face beside an edge and the face on the left of a step (`faceBeside`,
`faceAt`); the crossings of the chart continued along a curve with the faces
they leave and enter (`turnAtF`, `vertexCrossingsF`, `borderCrossings`, the
definitions of `Border.lean` with the faces written down); the inverse of a
motion whose linear part is a quarter-turn (`qinv`); the jump of a crossing
through charts (`jump`).

To prove (the brief of the brick):
* `borderCrossings_holonomy`: `borderHolonomy ℓ fuel steps` is the composite of
  the motions of `borderCrossings ℓ fuel steps`;
* `borderCrossings_chain`: each crossing enters the face the next one leaves
  (`List.Chain'`), the first leaves the face on the left of the first step read
  in the frame of that face (`normalize`), and the last enters it;
* `qinv_comp`, `comp_qinv`: `qinv M` is the inverse of `M` when `M.lin` is a
  quarter-turn;
* `jump_tree`: across a tree edge the jump is the identity;
* `borderHolonomy_jumps`: with `M` the identity on the face on the left of the
  first step, the holonomy of the border is the composite of the jumps of all
  its crossings (the telescope);
* `borderHolonomy_cuts`: and of the jumps across the cuts alone, when the charts
  agree across the other crossings.

That the jumps across the cuts, in border order, are the jumps of a cut disc of
the model (`CutDisc.lean`), hence compose to the rotations round the cone
points in the order of first visit, is the next brick.
-/
module

public import TurnsOfATMesh.Border

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### Faces beside an edge -/

/-- The lower-left corner of the unit square `square z d ρ`. -/
def lowerLeft (z d ρ : GaussianInt) : GaussianInt :=
  ⟨z.re + min (min 0 d.re) (min (ρ * d).re (d + ρ * d).re),
    z.im + min (min 0 d.im) (min (ρ * d).im (d + ρ * d).im)⟩

/-- The face of the fine grid on the `ρ` side of the edge from `v` along `d`, the
unit square `square (fine v) d ρ` by its lower-left corner, in the rectangle of
`v` (a face of the grid when that square lies in the rectangle). -/
def faceBeside (v : Point Patch) (d ρ : GaussianInt) : Face Patch :=
  let c := lowerLeft (fine v) d ρ
  ⟨v.patch, c.re.toNat, c.im.toNat⟩

/-- The face on the left of a step. -/
def faceAt (s : Step Patch) : Face Patch := faceBeside s.1 s.2 gi

/-! ### The crossings of the chart continued along a curve, with their faces -/

/-- A crossing of the chart continued through a region: the face it leaves, its
change of chart and the face it enters, each face by the frame of its own patch. -/
abbrev Crossing (Patch : Type*) := Face Patch × Motion GaussianInt GaussianInt × Face Patch

/-- `turnAt` with the faces: each step of the chart continued clockwise round `v`
from the state `(v, d)` leaves the face on the left of `(v, d)` and enters the
face on the left of the next state. -/
def turnAtF (ℓ : α → ℕ) :
    ℕ → Point Patch → GaussianInt → Point Patch → GaussianInt → Option (List (Crossing Patch))
  | 0, _, _, _, _ => none
  | fuel + 1, v, d, v₀, d₀ =>
    if v = v₀ ∧ d = d₀ then some [] else
      (T.aroundStep ℓ (-gi) v d).bind fun (v', d', C) =>
        (turnAtF ℓ fuel v' d' v₀ d₀).map ((faceAt (v, d), C, faceAt (v', d')) :: ·)

/-- `vertexCrossings` with the faces. -/
def vertexCrossingsF (ℓ : α → ℕ) (fuel : ℕ) (s₁ s₂ : Step Patch) :
    Option (List (Crossing Patch)) :=
  (T.normalize ℓ s₁).bind fun (u, d) =>
    (T.normalize ℓ s₂).bind fun (v₀, d₀) =>
      T.turnAtF ℓ fuel (pointAt u.patch (fine u + d)) (gi * d) v₀ d₀

/-- **The crossings of the border**: the chart continued along the closed curve of
the steps through the region on its left, crossing by crossing, each with the
face it leaves and the face it enters. -/
def borderCrossings (ℓ : α → ℕ) (fuel : ℕ) : List (Step Patch) → Option (List (Crossing Patch))
  | [] => none
  | s :: ss =>
    ((s :: ss).zip (ss ++ [s]) |>.mapM fun (s₁, s₂) => T.vertexCrossingsF ℓ fuel s₁ s₂).map
      List.flatten

/-! ### Jumps through the charts of a tree -/

/-- The inverse of a motion `x ↦ u x + t` whose linear part `u` is a quarter-turn:
`x ↦ ū x − ū t`. -/
def qinv (M : Motion GaussianInt GaussianInt) : Motion GaussianInt GaussianInt :=
  ⟨conj M.lin, -(conj M.lin * M.trans)⟩

/-- **The jump of a crossing** from `f` to `g` seen through the charts `M`, in the
frame of the root: `M g ∘ C ∘ (M f)⁻¹`. -/
def jump (M : Face Patch → Motion GaussianInt GaussianInt) (c : Crossing Patch) :
    Motion GaussianInt GaussianInt :=
  Motion.comp (M c.2.2) (Motion.comp c.2.1 (qinv (M c.1)))

/-- The crossings of a list across the cuts: those the predicate `tree` does not
hold for, by the faces they leave and enter. -/
def cutCrossings (tree : Face Patch → Face Patch → Bool) (cs : List (Crossing Patch)) :
    List (Crossing Patch) :=
  cs.filter fun c => !tree c.1 c.2.2

/-! ### First facts -/

@[simp] theorem faceBeside_patch (v : Point Patch) (d ρ : GaussianInt) :
    (faceBeside v d ρ).patch = v.patch := rfl

theorem lowerLeft_re (z d ρ : GaussianInt) :
    (lowerLeft z d ρ).re = z.re + min (min 0 d.re) (min (ρ * d).re (d + ρ * d).re) := rfl

theorem lowerLeft_im (z d ρ : GaussianInt) :
    (lowerLeft z d ρ).im = z.im + min (min 0 d.im) (min (ρ * d).im (d + ρ * d).im) := rfl

/-- The square seen from its second corner, a quarter-turn further, is the same
square: the face on the left of the edge `(v + d, gi d)` is the face on the left of
`(v, d)`. -/
theorem lowerLeft_next (z d : GaussianInt) : lowerLeft (z + d) (gi * d) gi = lowerLeft z d gi := by
  obtain ⟨a, b⟩ := d
  obtain ⟨x, y⟩ := z
  simp only [lowerLeft, Zsqrtd.ext_iff, Zsqrtd.re_mul, Zsqrtd.im_mul, Zsqrtd.re_add,
    Zsqrtd.im_add, gi_re, gi_im]
  constructor <;> omega

theorem faceAt_next (v : Point Patch) (d : GaussianInt) (h : T.inRect ℓ v.patch (fine v + d) = true) :
    faceAt (pointAt v.patch (fine v + d), gi * d) = faceAt (v, d) := by
  simp only [faceAt, faceBeside, pointAt_patch, T.fine_pointAt_of_inRect h, lowerLeft_next]

theorem qinv_lin (M : Motion GaussianInt GaussianInt) : (qinv M).lin = conj M.lin := rfl

theorem qinv_trans (M : Motion GaussianInt GaussianInt) :
    (qinv M).trans = -(conj M.lin * M.trans) := rfl

@[simp] theorem turnAtF_zero (ℓ : α → ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch)
    (d₀ : GaussianInt) : T.turnAtF ℓ 0 v d v₀ d₀ = none := rfl

theorem turnAtF_stop (ℓ : α → ℕ) (fuel : ℕ) (v : Point Patch) (d : GaussianInt) :
    T.turnAtF ℓ (fuel + 1) v d v d = some [] := by
  simp [turnAtF]

end TMesh

end TurnsOfATMesh
