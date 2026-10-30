/-
The chart continued along a curve of the fine grid (DT-134, fifth brick): the
objects that relate the fine grid of `Grid.lean` (points, faces, the gluing of
the arcs point by point) to the changes of chart of `Holonomy.lean`, and the
holonomy of the border of a region as the chart continued from face to face.

* `crossEdge` is the change of chart across one fine edge, from the frame of its
  patch to the frame of the patch across: the identity inside a rectangle, the
  `crossMotion` of the arc holding the edge on a side, in half quads (`fineMotion`).
  It is the definition `Check/Main.lean` evaluates to carry a chart along a dual
  spanning tree of a region.
* `aroundStep` and `aroundVertex` walk round a vertex of the fine grid from face
  to face (`gi` counter-clockwise, `-gi` clockwise), crossing the sides as they
  come; `valenceAround` counts the faces met and `holonomyAround` composes the
  changes of chart.
* A `Step` of a curve is a fine edge given by its start and its unit direction in
  the frame of a patch holding it. `vertexCrossings` continues the chart from the
  face on the left of a step, clockwise round the vertex the step ends at, to the
  face on the left of the next step; `borderHolonomy` composes these round a
  closed curve: the chart continued along the curve through the region on its
  left, back to the frame of its first step.
* `roundSteps` is one round of the walk as such steps, with the arcs it crosses,
  by the recursion of `Grid.curve`.

The theorems on these definitions (the crossing of an edge on a side takes its
ends to the points they are glued to, the holonomy round a vertex met by `k`
faces is the rotation `R^{4-k}` about it, and the holonomy of the border of a
round is the development of its turn) are proved after this interface.
-/
module

public import TurnsOfATMesh.Development
public import TurnsOfATMesh.CutDisc

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### Fine coordinates -/

/-- A point of the fine grid in the plane of its patch, in half quads. -/
def fine (p : Point Patch) : GaussianInt := ⟨p.x, p.y⟩

/-- A motion of the plane of a patch in quads, read in half quads: the
translation doubled. -/
def fineMotion (C : Motion GaussianInt GaussianInt) : Motion GaussianInt GaussianInt :=
  ⟨C.lin, 2 * C.trans⟩

/-- The point of the patch `P` at `z`, for `z` with non-negative coordinates. -/
def pointAt (P : Patch) (z : GaussianInt) : Point Patch := ⟨P, z.re.toNat, z.im.toNat⟩

/-- Whether `z` lies in the closed rectangle of `P` at the whole lengths `ℓ`, in
half quads. -/
def inRect (ℓ : α → ℕ) (P : Patch) (z : GaussianInt) : Bool :=
  0 ≤ z.re && z.re ≤ 2 * T.width ℓ P && 0 ≤ z.im && z.im ≤ 2 * T.height ℓ P

/-- The unit square beside the edge from `z` along `d`, on its side `ρ · d`
(`ρ = gi`: on its left; `ρ = -gi`: on its right), as its four corners from `z`. -/
def square (z d ρ : GaussianInt) : List GaussianInt := [z, z + d, z + d + ρ * d, z + ρ * d]

/-! ### The change of chart across a fine edge -/

/-- The side of its rectangle the fine edge from `p` to `q` lies on (`p` and `q`
in one patch, one apart), with the fine position of its lower end along the
side; `none` for an edge inside the rectangle. -/
def onSide (ℓ : α → ℕ) (p q : Point Patch) : Option (Side Patch × ℕ) :=
  let w := 2 * T.width ℓ p.patch
  let h := 2 * T.height ℓ p.patch
  if p.x = q.x ∧ p.x = 0 then some ((p.patch, 1, 0), min p.y q.y)
  else if p.x = q.x ∧ p.x = w then some ((p.patch, 1, 1), min p.y q.y)
  else if p.y = q.y ∧ p.y = 0 then some ((p.patch, 0, 0), min p.x q.x)
  else if p.y = q.y ∧ p.y = h then some ((p.patch, 0, 1), min p.x q.x)
  else none

/-- The arc of a side whose arcs are `l` holding the fine edge from the fine
position `t` to `t + 1` along it. -/
def arcOn (ℓ : α → ℕ) : List α → ℕ → Option α
  | [], _ => none
  | a :: l, t => if t < 2 * ℓ a then some a else arcOn ℓ l (t - 2 * ℓ a)

/-- **The change of chart across a fine edge**, from the frame of the patch of
`p` to the frame of the patch across the edge: the identity inside a rectangle;
on a side, `crossMotion` across the arc holding the edge, in half quads. With
the patch across. `none` on a side where no arc holds the edge. -/
def crossEdge (ℓ : α → ℕ) (p q : Point Patch) :
    Option (Patch × Motion GaussianInt GaussianInt) :=
  match T.onSide ℓ p q with
  | none => some (p.patch, Motion.id)
  | some (τ, t) => (arcOn ℓ (T.arcs τ) t).map fun a =>
      ((T.other a τ).1, fineMotion (T.crossMotion ℓ τ a))

/-! ### Round a vertex -/

/-- One step of the walk round the vertex `v` from the edge leaving it along
`d`, into the face on the side `ρ · d` of that edge (`ρ = gi`: counter-clockwise,
from the face on the right of the edge to the face on its left; `ρ = -gi`:
clockwise). Inside the rectangle, no change of chart; otherwise the edge lies on
a side and the chart crosses it, to the frame of the patch across, where the face
entered must lie. The vertex and the next edge leaving it, a quarter-turn `ρ`
further, in the frame entered, with the change of chart. -/
def aroundStep (ℓ : α → ℕ) (ρ : GaussianInt) (v : Point Patch) (d : GaussianInt) :
    Option (Point Patch × GaussianInt × Motion GaussianInt GaussianInt) :=
  let z := fine v
  if (square z d ρ).all (T.inRect ℓ v.patch) then some (v, ρ * d, Motion.id)
  else if T.inRect ℓ v.patch (z + d) then
    (T.crossEdge ℓ v (pointAt v.patch (z + d))).bind fun (Q, C) =>
      let z' := C.apply z
      let d' := C.lin * d
      if (square z' d' ρ).all (T.inRect ℓ Q) then some (pointAt Q z', ρ * d', C) else none
  else none

/-- The walk round a vertex, `ρ`-wise, from the edge `(v, d)` until the edge
`(v₀, d₀)` is reached, within `fuel` steps: the changes of chart of its steps. -/
def around (ℓ : α → ℕ) (ρ : GaussianInt) :
    ℕ → Point Patch → GaussianInt → Point Patch → GaussianInt →
      Option (List (Motion GaussianInt GaussianInt))
  | 0, _, _, _, _ => none
  | fuel + 1, v, d, v₀, d₀ =>
    (T.aroundStep ℓ ρ v d).bind fun (v', d', C) =>
      if v' = v₀ ∧ d' = d₀ then some [C] else (around ℓ ρ fuel v' d' v₀ d₀).map (C :: ·)

/-- The walk counter-clockwise round the vertex `v` from the face on the right
of the edge leaving `v` along `d` (a face of the rectangle of `v`), back to it. -/
def aroundVertex (ℓ : α → ℕ) (fuel : ℕ) (v : Point Patch) (d : GaussianInt) :
    Option (List (Motion GaussianInt GaussianInt)) :=
  T.around ℓ gi fuel v d v d

/-- The number of faces round the vertex `v`. -/
def valenceAround (ℓ : α → ℕ) (fuel : ℕ) (v : Point Patch) (d : GaussianInt) : Option ℕ :=
  (T.aroundVertex ℓ fuel v d).map List.length

/-- **The holonomy round the vertex `v`**: the composite of the changes of chart
of the walk round it, back to the frame of `v`. -/
def holonomyAround (ℓ : α → ℕ) (fuel : ℕ) (v : Point Patch) (d : GaussianInt) :
    Option (Motion GaussianInt GaussianInt) :=
  (T.aroundVertex ℓ fuel v d).map Motion.composeAll

/-! ### The chart continued along a curve -/

/-- A step of a curve of the fine grid: a fine edge, by its start and its unit
direction, in the frame of a patch holding it. -/
abbrev Step (Patch : Type*) := Point Patch × GaussianInt

/-- The step in the frame of the patch holding the face on its left: itself when
that face is in the rectangle of its start, else the step read across the side
it lies on. -/
def normalize (ℓ : α → ℕ) (s : Step Patch) : Option (Step Patch) :=
  let z := fine s.1
  if (square z s.2 gi).all (T.inRect ℓ s.1.patch) then some s
  else (T.crossEdge ℓ s.1 (pointAt s.1.patch (z + s.2))).map fun (Q, C) =>
    (pointAt Q (C.apply z), C.lin * s.2)

/-- The chart continued clockwise round the vertex `v` from the face on the left
of the edge `(v, d)`, crossing edge after edge, until the edge `(v₀, d₀)` is the
one leaving `v`, within `fuel` crossings: the changes of chart. -/
def turnAt (ℓ : α → ℕ) :
    ℕ → Point Patch → GaussianInt → Point Patch → GaussianInt →
      Option (List (Motion GaussianInt GaussianInt))
  | 0, _, _, _, _ => none
  | fuel + 1, v, d, v₀, d₀ =>
    if v = v₀ ∧ d = d₀ then some [] else
      (T.aroundStep ℓ (-gi) v d).bind fun (v', d', C) =>
        (turnAt ℓ fuel v' d' v₀ d₀).map (C :: ·)

/-- **The crossings between two consecutive steps** of a curve, the region on its
left: from the face on the left of `s₁`, clockwise round the vertex `s₁` ends at
(from the edge a quarter-turn to the left of `s₁`), to the face on the left of
`s₂`, both steps read in the frames of those faces. -/
def vertexCrossings (ℓ : α → ℕ) (fuel : ℕ) (s₁ s₂ : Step Patch) :
    Option (List (Motion GaussianInt GaussianInt)) :=
  (T.normalize ℓ s₁).bind fun (u, d) =>
    (T.normalize ℓ s₂).bind fun (v₀, d₀) =>
      T.turnAt ℓ fuel (pointAt u.patch (fine u + d)) (gi * d) v₀ d₀

/-- **The holonomy of the border**: the chart continued along the closed curve of
the steps through the region on its left, from the face on the left of the first
step round to it, as a motion of the frame of that face. -/
def borderHolonomy (ℓ : α → ℕ) (fuel : ℕ) : List (Step Patch) →
    Option (Motion GaussianInt GaussianInt)
  | [] => none
  | s :: ss =>
    ((s :: ss).zip (ss ++ [s]) |>.mapM fun (s₁, s₂) => T.vertexCrossings ℓ fuel s₁ s₂).map
      fun css => Motion.composeAll (List.flatten css)

/-! ### The steps of a round -/

/-- The steps of the dual line across a rectangle from the station `k + ½` of
`σ`: from each point of `across` to the next, along `inward σ`. -/
def acrossSteps (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) : List (Step Patch) :=
  (T.across ℓ σ k).dropLast.map fun p => (p, inward σ)

/-- The steps along the side `σ` from the fine position `s` to the fine position
`t`, along `acrossDir σ` or against it. -/
def alongSteps (ℓ : α → ℕ) (σ : Side Patch) (s t : ℕ) : List (Step Patch) :=
  (T.alongSide ℓ σ s t).dropLast.map fun p => (p, if s ≤ t then acrossDir σ else -acrossDir σ)

/-- One round of the walk from the station `k + ½` of `σ` as steps, with the
arcs it crosses in order, by the recursion of `Grid.curve`: across each
rectangle, then across the arc, until it comes back in through `σ`; then along
`σ` back to its start. -/
def curveSteps (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) :
    ℕ → List (Step Patch) → List α → Side Patch × ℚ → Option (List (Step Patch) × List α)
  | 0, _, _, _ => none
  | fuel + 1, acc, arcs, p =>
    let k' : ℕ := ((2 * p.2 - 1) / 2).floor.toNat
    let τ := p.1.opp
    match arcAt (rat ℓ) (T.arcs τ) p.2 with
    | none => none
    | some a =>
      let q : Side Patch × ℚ := (T.other a τ, (T.crossAt (rat ℓ) τ a).apply p.2)
      let acc := acc ++ T.acrossSteps ℓ p.1 k'
      if q.1 = σ then
        let kq : ℕ := ((2 * q.2 - 1) / 2).floor.toNat
        some (acc ++ T.alongSteps ℓ σ (2 * kq + 1) (2 * k + 1), (a :: arcs).reverse)
      else curveSteps ℓ σ k fuel acc (a :: arcs) q

/-- The steps and the arcs of one round from the station `k + ½` of `σ`. -/
def roundSteps (patches : List Patch) (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) :
    Option (List (Step Patch) × List α) :=
  T.curveSteps ℓ σ k (T.stations patches ℓ + 1) [] [] (station σ k)

/-! ### First lemmas -/

@[simp] theorem fine_mk (P : Patch) (x y : ℕ) : fine (⟨P, x, y⟩ : Point Patch) = ⟨x, y⟩ := rfl

theorem fine_pointAt (P : Patch) {z : GaussianInt} (hre : 0 ≤ z.re) (him : 0 ≤ z.im) :
    fine (pointAt P z) = z := by
  obtain ⟨a, b⟩ := z
  simp only [pointAt, fine]
  ext <;> simp_all

@[simp] theorem pointAt_patch (P : Patch) (z : GaussianInt) : (pointAt P z).patch = P := rfl

@[simp] theorem fineMotion_lin (C : Motion GaussianInt GaussianInt) :
    (fineMotion C).lin = C.lin := rfl

@[simp] theorem fineMotion_trans (C : Motion GaussianInt GaussianInt) :
    (fineMotion C).trans = 2 * C.trans := rfl

theorem fineMotion_comp (C D : Motion GaussianInt GaussianInt) :
    fineMotion (Motion.comp C D) = Motion.comp (fineMotion C) (fineMotion D) := by
  ext <;> simp [fineMotion, Motion.comp, mul_add, mul_left_comm]

/-- Half quads read a motion of the quads: `fineMotion C` at `2 x` is twice `C` at `x`. -/
theorem fineMotion_apply (C : Motion GaussianInt GaussianInt) (x : GaussianInt) :
    (fineMotion C).apply (2 * x) = 2 * C.apply x := by
  simp [fineMotion, Motion.apply, mul_add, mul_left_comm]

/-- A point of a side in half quads: twice the first corner of the side, plus the
fine position along `acrossDir`. -/
theorem fine_sidePoint (ℓ : α → ℕ) (σ : Side Patch) (t : ℕ) :
    fine (T.sidePoint ℓ σ t) = 2 * T.origin ℓ σ + (t : GaussianInt) * acrossDir σ := by
  obtain ⟨P, j, s⟩ := σ
  fin_cases j <;> fin_cases s <;> ext <;> simp [sidePoint, origin, acrossDir, fine]

end TMesh

end TurnsOfATMesh
