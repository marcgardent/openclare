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

Proved after this interface:

* The crossing of an edge on a side takes its ends to the points they are glued
  to (`crossMotion_glued`, `crossEdge_same`), so a step round a vertex stays at
  the vertex (`aroundStep_same`).
* The holonomy round a vertex met by `k` faces is the rotation `R^{4-k}` about it
  (`holonomyAround_eq`); no orientation is needed.
* The steps of a round start at the points of `round` (`roundSteps_round`).
* **The holonomy of the border of a round is the development of its turn**
  (`borderHolonomy_round`): on an oriented T-mesh at whole lengths, every side of
  positive length and the lengths balanced, `borderHolonomy ℓ 4` of the steps of
  a round is the composite of the `crossMotions` of the arcs it crosses, in half
  quads, which is `Turn.development` of its turn. Assumed: every node strictly
  inside the first side `σ` is met by four faces. It is used only when the
  closing segment along `σ` has the region on its left outside the rectangle of
  `σ`: the chart then runs below `σ`, and at each node the four crossings round
  it compose to the identity (`vc_node`).

The proof reads each step in the frame of a side (`framePt`, `dir`), where every
square to test is four integer conditions. A step round a vertex walked back
crosses back by the inverse change of chart (`aroundStep_reverse`). The
crossings of a list of steps compose run by run (`chain`, `chain_range`).
`Motion.composeAll_telescope` is the telescope of charts the next brick needs.
-/
module

public import TurnsOfATMesh.Development
public import TurnsOfATMesh.CutDisc
public import TurnsOfATMesh.GridSurface

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

/-! ### Points, frames and squares -/

@[simp] theorem pointAt_fine (p : Point Patch) : pointAt p.patch (fine p) = p := by
  obtain ⟨P, x, y⟩ := p
  simp [pointAt, fine]

theorem inRect_iff (ℓ : α → ℕ) (P : Patch) (z : GaussianInt) :
    T.inRect ℓ P z = true ↔
      0 ≤ z.re ∧ z.re ≤ 2 * T.width ℓ P ∧ 0 ≤ z.im ∧ z.im ≤ 2 * T.height ℓ P := by
  simp [inRect, and_assoc]

theorem fine_pointAt_of_inRect {ℓ : α → ℕ} {P : Patch} {z : GaussianInt}
    (h : T.inRect ℓ P z = true) : fine (pointAt P z) = z := by
  rw [inRect_iff] at h
  exact fine_pointAt P h.1 h.2.2.1

/-- The length of the rectangle of the side `τ` along it, in quads. -/
def lenAlong (ℓ : α → ℕ) (τ : Side Patch) : ℕ :=
  if τ.2.1 = 0 then T.width ℓ τ.1 else T.height ℓ τ.1

/-- The length of the rectangle of the side `τ` across it, in quads. -/
def lenAcross (ℓ : α → ℕ) (τ : Side Patch) : ℕ :=
  if τ.2.1 = 0 then T.height ℓ τ.1 else T.width ℓ τ.1

/-- The point at the fine position `s` along the side `τ` and `r` inward from it, in
half quads in the frame of its patch. -/
def framePt (ℓ : α → ℕ) (τ : Side Patch) (s r : ℤ) : GaussianInt :=
  2 * T.origin ℓ τ + (s : GaussianInt) * acrossDir τ + (r : GaussianInt) * inward τ

theorem fine_sidePoint_eq_framePt (ℓ : α → ℕ) (τ : Side Patch) (t : ℕ) :
    fine (T.sidePoint ℓ τ t) = T.framePt ℓ τ t 0 := by
  rw [fine_sidePoint, framePt]; push_cast; ring

/-- **The rectangle in the frame of a side.** -/
theorem inRect_framePt (ℓ : α → ℕ) (τ : Side Patch) (s r : ℤ) :
    T.inRect ℓ τ.1 (T.framePt ℓ τ s r) = true ↔
      0 ≤ s ∧ s ≤ 2 * T.lenAlong ℓ τ ∧ 0 ≤ r ∧ r ≤ 2 * T.lenAcross ℓ τ := by
  obtain ⟨P, j, e⟩ := τ
  rw [inRect_iff]
  fin_cases j <;> fin_cases e <;>
    simp [framePt, origin, acrossDir, inward, lenAlong, lenAcross, gi] <;> omega

theorem gi_mul_acrossDir (τ : Side Patch) : gi * acrossDir τ = (τ.orient : GaussianInt) * inward τ := by
  obtain ⟨P, j, e⟩ := τ
  fin_cases j <;> fin_cases e <;> simp [acrossDir, inward, Side.orient, gi_mul_gi]

theorem gi_mul_inward (τ : Side Patch) : gi * inward τ = -(τ.orient : GaussianInt) * acrossDir τ := by
  obtain ⟨P, j, e⟩ := τ
  fin_cases j <;> fin_cases e <;> simp [acrossDir, inward, Side.orient, gi_mul_gi]

theorem orient_sq (τ : Side Patch) : (τ.orient : GaussianInt) * τ.orient = 1 := by
  have := Side.orient_mul_self τ
  exact_mod_cast this

/-! ### The crossing of an arc in half quads -/

/-- **Across an arc in the frame of its side**: the point at the fine position `s`
along `τ` and `r` inward goes to the point at `sign a · s + 2 crossTrans` along the
other side and `r` outward. -/
theorem crossMotion_framePt [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ) (s r : ℤ) :
    (fineMotion (T.crossMotion ℓ τ a)).apply (T.framePt ℓ τ s r) =
      T.framePt ℓ (T.other a τ) (T.sign a * s + 2 * T.crossTrans ℓ τ a) (-r) := by
  have h0 := T.crossMotion_side hT ℓ ha 0
  have hA := T.crossLin_acrossDir hT ha
  have hI := crossLin_inward τ (T.other a τ)
  simp only [Int.cast_zero, zero_mul, add_zero, mul_zero, zero_add] at h0
  rw [apply_eq] at h0 ⊢
  rw [crossMotion_lin] at h0
  simp only [fineMotion_lin, fineMotion_trans, crossMotion_lin, framePt]
  push_cast
  linear_combination 2 * h0 + (s : GaussianInt) * hA + (r : GaussianInt) * hI

theorem framePt_congr (ℓ : α → ℕ) (τ : Side Patch) {s s' r r' : ℤ} (hs : s = s') (hr : r = r') :
    T.framePt ℓ τ s r = T.framePt ℓ τ s' r' := by rw [hs, hr]

theorem sign_cases (a : α) : T.sign a = 1 ∨ T.sign a = -1 := T.sign_isSign a

/-- **The crossing of an edge on a side takes its ends to the points they are glued
to**: from the first side of the arc to the second. -/
theorem crossMotion_glued [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) (a : α) {t : ℕ}
    (ht : t ≤ 2 * ℓ a) :
    (fineMotion (T.crossMotion ℓ (T.ends a 0) a)).apply (fine (T.glued ℓ a t).1) =
      fine (T.glued ℓ a t).2 := by
  have ha : a ∈ T.arcs (T.ends a 0) := (T.mem_arcs a _).mpr (Or.inl rfl)
  simp only [glued]
  rw [fine_sidePoint_eq_framePt, fine_sidePoint_eq_framePt, crossMotion_framePt T hT ℓ ha,
    other_ends_zero]
  apply framePt_congr _ _ _ _ neg_zero
  unfold crossTrans
  rw [other_ends_zero]
  rcases T.sign_cases a with hs | hs <;> simp [hs] <;> omega

/-- The same, from the second side of the arc back to the first. -/
theorem crossMotion_glued_back [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) (a : α) {t : ℕ}
    (ht : t ≤ 2 * ℓ a) :
    (fineMotion (T.crossMotion ℓ (T.ends a 1) a)).apply (fine (T.glued ℓ a t).2) =
      fine (T.glued ℓ a t).1 := by
  have ha : a ∈ T.arcs (T.ends a 1) := (T.mem_arcs a _).mpr (Or.inr rfl)
  simp only [glued]
  rw [fine_sidePoint_eq_framePt, fine_sidePoint_eq_framePt, crossMotion_framePt T hT ℓ ha,
    other_ends_one]
  apply framePt_congr _ _ _ _ neg_zero
  unfold crossTrans
  rw [other_ends_one]
  rcases T.sign_cases a with hs | hs <;> simp [hs] <;> omega

/-! ### The arc under a fine edge -/

/-- The arc holding the fine edge `[t, t + 1]` of a side is the arc whose interval
`[2 pos, 2 pos + 2 ℓ_a]` holds it. -/
theorem arcOn_eq_some (ℓ : α → ℕ) {l : List α} (hl : l.Nodup) (t : ℕ) (a : α) :
    arcOn ℓ l t = some a ↔ a ∈ l ∧ 2 * posNat a l ℓ ≤ t ∧ t < 2 * posNat a l ℓ + 2 * ℓ a := by
  induction l generalizing t with
  | nil => simp [arcOn]
  | cons b l ih =>
    rw [List.nodup_cons] at hl
    by_cases hab : b = a
    · subst hab
      simp only [arcOn, posNat, ↓reduceIte, mul_zero, zero_le, zero_add, List.mem_cons, true_or,
        true_and]
      split_ifs with h
      · simp [h]
      · rw [ih hl.2]
        constructor
        · rintro ⟨hb, -⟩; exact absurd hb hl.1
        · intro h'; exact absurd h' h
    · simp only [arcOn, posNat, hab, ↓reduceIte, List.mem_cons, Ne.symm hab, false_or]
      split_ifs with h
      · simp only [Option.some.injEq, hab, false_iff, not_and, not_lt]
        intro _ hlt
        omega
      · rw [ih hl.2]
        constructor <;> rintro ⟨hm, h₁, h₂⟩ <;> exact ⟨hm, by omega, by omega⟩

/-! ### The side of a fine edge -/

/-- An edge of a side lies on that side, at the fine position of its lower end. -/
theorem onSide_sidePoint (ℓ : α → ℕ) (τ : Side Patch) (hH : 0 < T.lenAcross ℓ τ) (s : ℕ) :
    T.onSide ℓ (T.sidePoint ℓ τ s) (T.sidePoint ℓ τ (s + 1)) = some (τ, s) ∧
      T.onSide ℓ (T.sidePoint ℓ τ (s + 1)) (T.sidePoint ℓ τ s) = some (τ, s) := by
  obtain ⟨P, j, e⟩ := τ
  fin_cases j <;> fin_cases e <;>
    simp [onSide, sidePoint, lenAcross] at hH ⊢ <;> omega

/-- The fine edge from `p` to `q = p + d` found on a side lies on it: `p` and `q` are its
points at the position found and the next. -/
theorem onSide_some (ℓ : α → ℕ) {p q : Point Patch} {d : GaussianInt} (hd : IsQuarter d)
    (hq : q.patch = p.patch) (hpq : fine q = fine p + d) {τ : Side Patch} {t : ℕ}
    (h : T.onSide ℓ p q = some (τ, t)) :
    τ.1 = p.patch ∧ (p = T.sidePoint ℓ τ t ∧ q = T.sidePoint ℓ τ (t + 1) ∨
      p = T.sidePoint ℓ τ (t + 1) ∧ q = T.sidePoint ℓ τ t) := by
  obtain ⟨P, x, y⟩ := p
  obtain ⟨Q, x', y'⟩ := q
  simp only at hq
  subst hq
  have hre := congrArg Zsqrtd.re hpq
  have him := congrArg Zsqrtd.im hpq
  simp only [fine_mk] at hre him
  unfold onSide at h
  dsimp only at h
  split_ifs at h <;> simp only [Option.some.injEq, Prod.mk.injEq] at h <;>
    obtain ⟨rfl, rfl⟩ := h <;>
    rcases hd with rfl | rfl | rfl | rfl <;> simp [gi] at hre him <;> simp [sidePoint] <;> omega

/-- The change of chart across an edge of a side is the crossing of the arc holding it. -/
theorem crossEdge_side (ℓ : α → ℕ) (τ : Side Patch) (hH : 0 < T.lenAcross ℓ τ) {s : ℕ} {a : α}
    (ha : arcOn ℓ (T.arcs τ) s = some a) :
    T.crossEdge ℓ (T.sidePoint ℓ τ s) (T.sidePoint ℓ τ (s + 1)) =
        some ((T.other a τ).1, fineMotion (T.crossMotion ℓ τ a)) ∧
      T.crossEdge ℓ (T.sidePoint ℓ τ (s + 1)) (T.sidePoint ℓ τ s) =
        some ((T.other a τ).1, fineMotion (T.crossMotion ℓ τ a)) := by
  obtain ⟨h1, h2⟩ := T.onSide_sidePoint ℓ τ hH s
  constructor
  · simp only [crossEdge, h1, ha, Option.map_some]
  · simp only [crossEdge, h2, ha, Option.map_some]

/-- A point of an arc goes, across the arc, to a point it is glued to. -/
theorem same_crossMotion_sidePoint [Fintype α] (hT : T.Oriented) (allArcs : List α)
    (hall : ∀ a, a ∈ allArcs) (ℓ : α → ℕ) {τ : Side Patch} {a : α} (ha : a ∈ T.arcs τ) {u : ℕ}
    (hu₁ : 2 * posNat a (T.arcs τ) ℓ ≤ u) (hu₂ : u ≤ 2 * posNat a (T.arcs τ) ℓ + 2 * ℓ a) :
    ∃ u' : ℕ, (fineMotion (T.crossMotion ℓ τ a)).apply (fine (T.sidePoint ℓ τ u)) =
        fine (T.sidePoint ℓ (T.other a τ) u') ∧
      T.Same allArcs ℓ (T.sidePoint ℓ (T.other a τ) u') (T.sidePoint ℓ τ u) := by
  rcases (T.mem_arcs a τ).mp ha with h0 | h0
  · subst h0
    set t := u - 2 * posNat a (T.arcs (T.ends a 0)) ℓ
    have hg := T.crossMotion_glued hT ℓ a (t := t) (by omega)
    have e1 : T.sidePoint ℓ (T.ends a 0) u = (T.glued ℓ a t).1 := by
      simp only [glued]; congr 1; omega
    refine ⟨if T.sign a = 1 then 2 * posNat a (T.arcs (T.ends a 1)) ℓ + t
      else 2 * posNat a (T.arcs (T.ends a 1)) ℓ + 2 * ℓ a - t, ?_, ?_⟩
    · rw [e1, hg, other_ends_zero]; rfl
    · rw [e1, other_ends_zero]
      exact T.same_symm _ _ (T.same_of_glued allArcs ℓ a (hall a) t (by omega))
  · subst h0
    set q := posNat a (T.arcs (T.ends a 1)) ℓ
    set t := if T.sign a = 1 then u - 2 * q else 2 * q + 2 * ℓ a - u
    have ht : t ≤ 2 * ℓ a := by simp only [t]; split_ifs <;> omega
    have hg := T.crossMotion_glued_back hT ℓ a ht
    have e1 : T.sidePoint ℓ (T.ends a 1) u = (T.glued ℓ a t).2 := by
      simp only [glued, t]; congr 1; split_ifs <;> omega
    refine ⟨2 * posNat a (T.arcs (T.ends a 0)) ℓ + t, ?_, ?_⟩
    · rw [e1, hg, other_ends_one]; rfl
    · rw [e1, other_ends_one]
      exact T.same_of_glued allArcs ℓ a (hall a) t ht

/-- **The change of chart across a fine edge takes its ends to the points they are
glued to** (the identity inside a rectangle): the image of `p` in the frame of the
patch across is one vertex with `p`, and so is the image of `q`. -/
theorem crossEdge_same [Fintype α] (hT : T.Oriented) (allArcs : List α) (hall : ∀ a, a ∈ allArcs)
    (ℓ : α → ℕ) {p q : Point Patch} {d : GaussianInt} (hd : IsQuarter d) (hq : q.patch = p.patch)
    (hpq : fine q = fine p + d) {Q : Patch} {C : Motion GaussianInt GaussianInt}
    (h : T.crossEdge ℓ p q = some (Q, C)) :
    T.Same allArcs ℓ (pointAt Q (C.apply (fine p))) p ∧
      T.Same allArcs ℓ (pointAt Q (C.apply (fine q))) q := by
  unfold crossEdge at h
  cases hs : T.onSide ℓ p q with
  | none =>
    rw [hs] at h
    simp only [Option.some.injEq, Prod.mk.injEq] at h
    obtain ⟨rfl, rfl⟩ := h
    have hid : ∀ z : GaussianInt, (Motion.id : Motion GaussianInt GaussianInt).apply z = z :=
      fun z => by simp [Motion.apply, Motion.id]
    refine ⟨?_, ?_⟩
    · rw [hid, pointAt_fine]; exact Relation.EqvGen.refl _
    · rw [hid, ← hq, pointAt_fine]; exact Relation.EqvGen.refl _
  | some τt =>
    obtain ⟨τ, t⟩ := τt
    rw [hs] at h
    obtain ⟨a, ha, he⟩ := Option.map_eq_some_iff.mp h
    simp only [Prod.mk.injEq] at he
    obtain ⟨rfl, rfl⟩ := he
    obtain ⟨hP, hpts⟩ := T.onSide_some ℓ hd hq hpq hs
    obtain ⟨hmem, h₁, h₂⟩ := (arcOn_eq_some ℓ (T.arcs_nodup τ) t a).mp ha
    have key : ∀ u, t ≤ u → u ≤ t + 1 →
        T.Same allArcs ℓ (pointAt (T.other a τ).1
          ((fineMotion (T.crossMotion ℓ τ a)).apply (fine (T.sidePoint ℓ τ u)))) (T.sidePoint ℓ τ u) := by
      intro u hu₁ hu₂
      obtain ⟨u', e, hsame⟩ := T.same_crossMotion_sidePoint hT allArcs hall ℓ hmem
        (u := u) (by omega) (by omega)
      rw [e, ← sidePoint_patch (T := T) ℓ (T.other a τ) u', pointAt_fine]
      exact hsame
    rcases hpts with ⟨rfl, rfl⟩ | ⟨rfl, rfl⟩
    · exact ⟨key t le_rfl (by omega), key (t + 1) (by omega) le_rfl⟩
    · exact ⟨key (t + 1) (by omega) le_rfl, key t le_rfl (by omega)⟩

/-- **A step round a vertex stays at the vertex**: the point reached is one vertex of
the quad mesh with the point left. -/
theorem aroundStep_same [Fintype α] (hT : T.Oriented) (allArcs : List α) (hall : ∀ a, a ∈ allArcs)
    (ℓ : α → ℕ) {ρ : GaussianInt} {v : Point Patch} {d : GaussianInt} (hd : IsQuarter d)
    {v' : Point Patch} {d' : GaussianInt} {C : Motion GaussianInt GaussianInt}
    (h : T.aroundStep ℓ ρ v d = some (v', d', C)) : T.Same allArcs ℓ v' v := by
  unfold aroundStep at h
  dsimp only at h
  split_ifs at h with h1 h2
  · simp only [Option.some.injEq, Prod.mk.injEq] at h
    obtain ⟨rfl, -, -⟩ := h
    exact Relation.EqvGen.refl _
  · obtain ⟨⟨Q, C'⟩, hc, h'⟩ := Option.bind_eq_some_iff.mp h
    simp only at h'
    split_ifs at h' with h3
    simp only [Option.some.injEq, Prod.mk.injEq] at h'
    obtain ⟨rfl, -, -⟩ := h'
    exact (T.crossEdge_same hT allArcs hall ℓ hd (p := v) (q := pointAt v.patch (fine v + d)) rfl
      (T.fine_pointAt_of_inRect h2) hc).1

/-! ### The holonomy round a vertex -/

/-- **One step round a vertex** moves the point by its change of chart, which turns the
edge left by `ρ`. -/
theorem aroundStep_apply {ℓ : α → ℕ} {ρ : GaussianInt} {v : Point Patch} {d : GaussianInt}
    {v' : Point Patch} {d' : GaussianInt} {C : Motion GaussianInt GaussianInt}
    (h : T.aroundStep ℓ ρ v d = some (v', d', C)) :
    C.apply (fine v) = fine v' ∧ d' = ρ * C.lin * d := by
  unfold aroundStep at h
  dsimp only at h
  split_ifs at h with h1 h2
  · simp only [Option.some.injEq, Prod.mk.injEq] at h
    obtain ⟨rfl, rfl, rfl⟩ := h
    simp [Motion.apply, Motion.id]
  · obtain ⟨⟨Q, C'⟩, hc, h'⟩ := Option.bind_eq_some_iff.mp h
    simp only at h'
    split_ifs at h' with h3
    simp only [Option.some.injEq, Prod.mk.injEq] at h'
    obtain ⟨h₁, h₂, h₃⟩ := h'
    subst h₁ h₂ h₃
    have hin : T.inRect ℓ Q (C'.apply (fine v)) = true := by
      simp only [square, List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true] at h3
      exact h3.1
    exact ⟨(T.fine_pointAt_of_inRect hin).symm, by ring⟩

/-- **The invariant of the walk round a vertex**: the composite of its changes of chart
takes the point left to the point reached, and the edge left, turned by `ρ` at each
step, to the edge reached. -/
theorem around_apply (ℓ : α → ℕ) (ρ : GaussianInt) :
    ∀ (fuel : ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch) (d₀ : GaussianInt)
      (L : List (Motion GaussianInt GaussianInt)), T.around ℓ ρ fuel v d v₀ d₀ = some L →
      (Motion.composeAll L).apply (fine v) = fine v₀ ∧
        d₀ = ρ ^ L.length * (Motion.composeAll L).lin * d
  | 0, _, _, _, _, _, h => by simp [around] at h
  | fuel + 1, v, d, v₀, d₀, L, h => by
    unfold around at h
    obtain ⟨⟨v', d', C⟩, hs, h'⟩ := Option.bind_eq_some_iff.mp h
    obtain ⟨ha, hd⟩ := T.aroundStep_apply hs
    simp only at h'
    split_ifs at h' with heq
    · simp only [Option.some.injEq] at h'
      subst h'
      obtain ⟨rfl, rfl⟩ := heq
      simp [ha, hd]
    · obtain ⟨L', hL', rfl⟩ := Option.map_eq_some_iff.mp h'
      obtain ⟨ia, id'⟩ := around_apply ℓ ρ fuel v' d' v₀ d₀ L' hL'
      refine ⟨?_, ?_⟩
      · rw [Motion.composeAll_cons, Motion.apply_comp, ha, ia]
      · rw [id', hd, Motion.composeAll_cons, Motion.comp_lin, List.length_cons]
        ring

/-- The walk round a vertex meets at least one face. -/
theorem around_ne_nil (ℓ : α → ℕ) (ρ : GaussianInt) :
    ∀ (fuel : ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch) (d₀ : GaussianInt)
      (L : List (Motion GaussianInt GaussianInt)), T.around ℓ ρ fuel v d v₀ d₀ = some L → L ≠ []
  | 0, _, _, _, _, _, h => by simp [around] at h
  | fuel + 1, v, d, v₀, d₀, L, h => by
    unfold around at h
    obtain ⟨⟨v', d', C⟩, hs, h'⟩ := Option.bind_eq_some_iff.mp h
    simp only at h'
    split_ifs at h' with heq
    · simp only [Option.some.injEq] at h'
      subst h'
      simp
    · obtain ⟨L', hL', rfl⟩ := Option.map_eq_some_iff.mp h'
      simp

/-- A vertex is met by at least one face. -/
theorem valenceAround_pos {ℓ : α → ℕ} {fuel : ℕ} {v : Point Patch} {d : GaussianInt} {k : ℕ}
    (h : T.valenceAround ℓ fuel v d = some k) : 0 < k := by
  unfold valenceAround aroundVertex at h
  obtain ⟨L, hL, rfl⟩ := Option.map_eq_some_iff.mp h
  exact List.length_pos_of_ne_nil (T.around_ne_nil ℓ gi fuel v d v d L hL)

/-- `R^k R^{4-k} = 1`. -/
theorem gi_pow_mul_quarter (k : ℕ) : gi ^ k * quarter (4 - (k : ℤ)) = 1 := by
  have h4 : giUnit ^ (4 : ℤ) = 1 := by
    rw [show (4 : ℤ) = ((2 * 2 : ℕ) : ℤ) from rfl, zpow_natCast, pow_mul, giUnit_sq]; simp
  have hk : (gi ^ k : GaussianInt) = ((giUnit ^ (k : ℤ) : GaussianIntˣ) : GaussianInt) := by
    rw [zpow_natCast, Units.val_pow_eq_pow_val, giUnit_val]
  rw [hk, quarter, ← Units.val_mul, ← zpow_add, show (k : ℤ) + (4 - k) = 4 by ring, h4,
    Units.val_one]

/-- **The holonomy round a vertex met by `k` faces is the rotation `R^{4-k}` about it**:
the composite of the changes of chart of the walk round it, back to its first frame. -/
theorem holonomyAround_eq {ℓ : α → ℕ} {fuel : ℕ} {v : Point Patch} {d : GaussianInt}
    (hd : IsQuarter d) {H : Motion GaussianInt GaussianInt} {k : ℕ}
    (hH : T.holonomyAround ℓ fuel v d = some H) (hk : T.valenceAround ℓ fuel v d = some k) :
    H = Motion.rotAbout (quarter (4 - (k : ℤ))) (fine v) := by
  unfold holonomyAround valenceAround aroundVertex at *
  obtain ⟨L, hL, rfl⟩ := Option.map_eq_some_iff.mp hH
  rw [hL] at hk
  simp only [Option.map_some, Option.some.injEq] at hk
  subst hk
  obtain ⟨ha, hdd⟩ := T.around_apply ℓ gi fuel v d v d L hL
  have hu := mul_conj_of_isQuarter hd
  have hlin : (Motion.composeAll L).lin = quarter (4 - (L.length : ℤ)) := by
    have h1 : gi ^ L.length * (Motion.composeAll L).lin = 1 := by
      linear_combination (-(conj d)) * hdd - (gi ^ L.length * (Motion.composeAll L).lin - 1) * hu
    have h2 := gi_pow_mul_quarter L.length
    linear_combination quarter (4 - (L.length : ℤ)) * h1 - (Motion.composeAll L).lin * h2
  apply Motion.eq_of_apply (fine v)
  · simpa using hlin
  · rw [ha, Motion.apply_rotAbout]; simp

/-- The linear part of the holonomy round a vertex met by `k` faces is `R^{4-k}`. -/
theorem holonomyAround_lin {ℓ : α → ℕ} {fuel : ℕ} {v : Point Patch} {d : GaussianInt}
    (hd : IsQuarter d) {H : Motion GaussianInt GaussianInt} {k : ℕ}
    (hH : T.holonomyAround ℓ fuel v d = some H) (hk : T.valenceAround ℓ fuel v d = some k) :
    H.lin = quarter (4 - (k : ℤ)) := by
  rw [T.holonomyAround_eq hd hH hk]; rfl

/-! ### The steps of a round -/

theorem alongSide_ne_nil (ℓ : α → ℕ) (σ : Side Patch) (s t : ℕ) : T.alongSide ℓ σ s t ≠ [] := by
  unfold alongSide
  split_ifs <;> simp [List.range_succ]

/-- `curveSteps` and `Grid.curve` follow the same recursion. -/
theorem curveSteps_curve (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) :
    ∀ (fuel : ℕ) (acc : List (Step Patch)) (arcs : List α) (accP : List (Point Patch))
      (p : Side Patch × ℚ), acc.map Prod.fst = accP →
      (T.curveSteps ℓ σ k fuel acc arcs p).map (fun r => r.1.map Prod.fst) =
        (T.curve ℓ σ k fuel accP p).map List.dropLast
  | 0, _, _, _, _, _ => by simp [curveSteps, curve]
  | fuel + 1, acc, arcs, accP, p, hacc => by
    unfold curveSteps curve step
    cases h : arcAt (rat ℓ) (T.arcs p.1.opp) p.2 with
    | none => simp [h]
    | some a =>
      simp only [h, Option.map_some]
      split_ifs with hq
      · simp only [Option.map_some, Option.some.injEq, List.map_append, hacc]
        rw [List.dropLast_append_of_ne_nil (T.alongSide_ne_nil ℓ σ _ _)]
        simp [acrossSteps, alongSteps, Function.comp_def, List.map_dropLast]
      · apply curveSteps_curve
        simp [hacc, acrossSteps, Function.comp_def]

/-- **The steps of a round are the points of the round**: their starts are the points
of `round` without its last, the start station once more. -/
theorem roundSteps_round (patches : List Patch) (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) :
    (T.roundSteps patches ℓ σ k).map (fun r => r.1.map Prod.fst) =
      (T.round patches ℓ σ k).map List.dropLast :=
  T.curveSteps_curve ℓ σ k _ [] [] [] _ rfl

/-! ### Directions and points in the frame of a side -/

/-- The direction `x` along the side `τ` and `y` inward. -/
def dir (τ : Side Patch) (x y : ℤ) : GaussianInt :=
  (x : GaussianInt) * acrossDir τ + (y : GaussianInt) * inward τ

/-- The point of the fine grid at the fine position `s` along `τ` and `r` inward. -/
def fpt (ℓ : α → ℕ) (τ : Side Patch) (s r : ℤ) : Point Patch := pointAt τ.1 (T.framePt ℓ τ s r)

@[simp] theorem fpt_patch (ℓ : α → ℕ) (τ : Side Patch) (s r : ℤ) : (T.fpt ℓ τ s r).patch = τ.1 := rfl

theorem framePt_add_dir (ℓ : α → ℕ) (τ : Side Patch) (s r x y : ℤ) :
    T.framePt ℓ τ s r + dir τ x y = T.framePt ℓ τ (s + x) (r + y) := by
  simp only [framePt, dir]; push_cast; ring

theorem pointAt_framePt_add_dir (ℓ : α → ℕ) (τ : Side Patch) (s r x y : ℤ) :
    pointAt τ.1 (T.framePt ℓ τ s r + dir τ x y) = T.fpt ℓ τ (s + x) (r + y) := by
  rw [framePt_add_dir]; rfl

theorem gi_mul_dir (τ : Side Patch) (x y : ℤ) :
    gi * dir τ x y = dir τ (-(τ.orient * y)) (τ.orient * x) := by
  have h1 := gi_mul_acrossDir τ
  have h2 := gi_mul_inward τ
  simp only [dir]; push_cast
  linear_combination (x : GaussianInt) * h1 + (y : GaussianInt) * h2

theorem neg_gi_mul_dir (τ : Side Patch) (x y : ℤ) :
    -gi * dir τ x y = dir τ (τ.orient * y) (-(τ.orient * x)) := by
  rw [neg_mul, gi_mul_dir]; simp only [dir]; push_cast; ring

@[simp] theorem dir_zero_one (τ : Side Patch) : dir τ 0 1 = inward τ := by simp [dir]

theorem dir_inj {τ : Side Patch} {x y x' y' : ℤ} (h : dir τ x y = dir τ x' y') : x = x' ∧ y = y' := by
  obtain ⟨P, j, e⟩ := τ
  have hre := congrArg Zsqrtd.re h
  have him := congrArg Zsqrtd.im h
  fin_cases j <;> fin_cases e <;> simp [dir, acrossDir, inward, gi] at hre him <;> omega

theorem orient_cases (τ : Side Patch) : τ.orient = 1 ∨ τ.orient = -1 := by
  unfold Side.orient; split_ifs <;> simp

theorem square_gi (ℓ : α → ℕ) (τ : Side Patch) (s r x y : ℤ) :
    square (T.framePt ℓ τ s r) (dir τ x y) gi =
      [T.framePt ℓ τ s r, T.framePt ℓ τ (s + x) (r + y),
        T.framePt ℓ τ (s + x + -(τ.orient * y)) (r + y + τ.orient * x),
        T.framePt ℓ τ (s + -(τ.orient * y)) (r + τ.orient * x)] := by
  simp only [square, gi_mul_dir, framePt_add_dir]

theorem square_neg_gi (ℓ : α → ℕ) (τ : Side Patch) (s r x y : ℤ) :
    square (T.framePt ℓ τ s r) (dir τ x y) (-gi) =
      [T.framePt ℓ τ s r, T.framePt ℓ τ (s + x) (r + y),
        T.framePt ℓ τ (s + x + τ.orient * y) (r + y + -(τ.orient * x)),
        T.framePt ℓ τ (s + τ.orient * y) (r + -(τ.orient * x))] := by
  simp only [square, neg_gi_mul_dir, framePt_add_dir]

theorem fine_fpt {ℓ : α → ℕ} {τ : Side Patch} {s r : ℤ} (h : T.inRect ℓ τ.1 (T.framePt ℓ τ s r) = true) :
    fine (T.fpt ℓ τ s r) = T.framePt ℓ τ s r := T.fine_pointAt_of_inRect h

theorem fpt_nat (ℓ : α → ℕ) (τ : Side Patch) (n : ℕ) : T.fpt ℓ τ n 0 = T.sidePoint ℓ τ n := by
  have h := pointAt_fine (T.sidePoint ℓ τ n)
  rwa [sidePoint_patch, fine_sidePoint_eq_framePt] at h

/-! ### Crossing an arc, in the frames of its sides -/

theorem other_other {τ : Side Patch} {a : α} (ha : a ∈ T.arcs τ) : T.other a (T.other a τ) = τ := by
  rcases (T.mem_arcs a τ).mp ha with h | h <;> subst h <;>
    simp [other_ends_zero, other_ends_one]

theorem mem_arcs_other {τ : Side Patch} {a : α} (ha : a ∈ T.arcs τ) : a ∈ T.arcs (T.other a τ) := by
  rcases (T.mem_arcs a τ).mp ha with h | h <;> subst h
  · rw [other_ends_zero]; exact (T.mem_arcs a _).mpr (Or.inr rfl)
  · rw [other_ends_one]; exact (T.mem_arcs a _).mpr (Or.inl rfl)

theorem other_ne {τ : Side Patch} {a : α} (ha : a ∈ T.arcs τ) : T.other a τ ≠ τ := by
  rcases (T.mem_arcs a τ).mp ha with h | h <;> subst h
  · rw [other_ends_zero]; exact (T.ends_ne a).symm
  · rw [other_ends_one]; exact T.ends_ne a

/-- On an oriented T-mesh the sign of an arc is fixed by the orientations of its sides. -/
theorem sign_eq_orient (hT : T.Oriented) {τ : Side Patch} {a : α} (ha : a ∈ T.arcs τ) :
    T.sign a = -(τ.orient * (T.other a τ).orient) := by
  rw [hT a]
  rcases (T.mem_arcs a τ).mp ha with h | h <;> subst h
  · rw [other_ends_zero]
  · rw [other_ends_one]; ring

theorem sign_mul_self (a : α) : T.sign a * T.sign a = 1 := by
  rcases T.sign_cases a with h | h <;> rw [h] <;> rfl

theorem crossMotion_lin_dir [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ) (x y : ℤ) :
    (fineMotion (T.crossMotion ℓ τ a)).lin * dir τ x y = dir (T.other a τ) (T.sign a * x) (-y) := by
  have hA := T.crossLin_acrossDir hT ha
  have hI := crossLin_inward τ (T.other a τ)
  simp only [fineMotion_lin, crossMotion_lin, dir]
  push_cast
  linear_combination (x : GaussianInt) * hA + (y : GaussianInt) * hI

/-- Back across the arc: the positions cross back. -/
theorem img_img {τ : Side Patch} {a : α} (ha : a ∈ T.arcs τ) (ℓ : α → ℕ) (x : ℤ) :
    T.sign a * (T.sign a * x + 2 * T.crossTrans ℓ τ a) + 2 * T.crossTrans ℓ (T.other a τ) a = x := by
  unfold crossTrans
  rw [T.other_other ha]
  rcases T.sign_cases a with hs | hs <;> simp [hs] <;> ring

/-- The positions of an arc on one side cross to the positions of the arc on the other. -/
theorem img_range {τ : Side Patch} {a : α} (ℓ : α → ℕ) {x : ℤ}
    (h₁ : 2 * (posNat a (T.arcs τ) ℓ : ℤ) ≤ x) (h₂ : x ≤ 2 * (posNat a (T.arcs τ) ℓ : ℤ) + 2 * ℓ a) :
    2 * (posNat a (T.arcs (T.other a τ)) ℓ : ℤ) ≤ T.sign a * x + 2 * T.crossTrans ℓ τ a ∧
      T.sign a * x + 2 * T.crossTrans ℓ τ a ≤
        2 * (posNat a (T.arcs (T.other a τ)) ℓ : ℤ) + 2 * ℓ a := by
  unfold crossTrans
  rcases T.sign_cases a with hs | hs <;> simp [hs] <;> omega

/-- **Crossing an arc there and back is the identity.** -/
theorem crossMotion_inv [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ) :
    Motion.comp (fineMotion (T.crossMotion ℓ (T.other a τ) a)) (fineMotion (T.crossMotion ℓ τ a)) =
      Motion.id := by
  have ha' := T.mem_arcs_other ha
  apply Motion.eq_of_apply (T.framePt ℓ τ 0 0)
  · have h1 := T.crossMotion_lin_dir hT ℓ ha 1 0
    have h2 := T.crossMotion_lin_dir hT ℓ ha' (T.sign a * 1) (-0)
    rw [T.other_other ha, ← mul_assoc, T.sign_mul_self] at h2
    have hu := mul_conj_of_isQuarter (isQuarter_acrossDir τ)
    simp only [dir, Int.cast_one, one_mul, Int.cast_zero, zero_mul, add_zero, neg_zero] at h1 h2
    simp only [Motion.comp_lin, Motion.id_lin]
    set A := (fineMotion (T.crossMotion ℓ τ a)).lin
    set B := (fineMotion (T.crossMotion ℓ (T.other a τ) a)).lin
    linear_combination conj (acrossDir τ) * (B * h1 + h2) - (B * A - 1) * hu
  · rw [Motion.apply_comp, T.crossMotion_framePt hT ℓ ha, T.crossMotion_framePt hT ℓ ha',
      T.other_other ha]
    simp only [mul_zero, zero_add, neg_zero]
    rw [show T.sign a * (2 * T.crossTrans ℓ τ a) + 2 * T.crossTrans ℓ (T.other a τ) a = 0 by
      have := T.img_img ha ℓ 0; simpa using this]
    simp [Motion.apply, Motion.id]

/-- The edge of a side from the position `x` to `x + 1` is crossed by the arc holding it. -/
theorem crossEdge_framePt (ℓ : α → ℕ) {τ : Side Patch} (hH : 0 < T.lenAcross ℓ τ) {a : α}
    (ha : a ∈ T.arcs τ) {x : ℤ} (hx₀ : 0 ≤ x) (hx₁ : 2 * (posNat a (T.arcs τ) ℓ : ℤ) ≤ x)
    (hx₂ : x + 1 ≤ 2 * (posNat a (T.arcs τ) ℓ : ℤ) + 2 * ℓ a) :
    T.crossEdge ℓ (T.fpt ℓ τ x 0) (T.fpt ℓ τ (x + 1) 0) =
        some ((T.other a τ).1, fineMotion (T.crossMotion ℓ τ a)) ∧
      T.crossEdge ℓ (T.fpt ℓ τ (x + 1) 0) (T.fpt ℓ τ x 0) =
        some ((T.other a τ).1, fineMotion (T.crossMotion ℓ τ a)) := by
  obtain ⟨n, rfl⟩ := Int.eq_ofNat_of_zero_le hx₀
  have hn : ((n : ℤ) + 1) = ((n + 1 : ℕ) : ℤ) := by push_cast; ring
  rw [hn, fpt_nat, fpt_nat]
  exact T.crossEdge_side ℓ τ hH ((arcOn_eq_some ℓ (T.arcs_nodup τ) n a).mpr ⟨ha, by omega, by omega⟩)

/-! ### Computing the walks step by step -/

theorem aroundStep_in {ℓ : α → ℕ} {ρ : GaussianInt} {v : Point Patch} {d : GaussianInt}
    (h : (square (fine v) d ρ).all (T.inRect ℓ v.patch) = true) :
    T.aroundStep ℓ ρ v d = some (v, ρ * d, Motion.id) := by
  unfold aroundStep; dsimp only; rw [ite_eq_left h]

theorem aroundStep_cross {ℓ : α → ℕ} {ρ : GaussianInt} {v : Point Patch} {d : GaussianInt}
    {Q : Patch} {C : Motion GaussianInt GaussianInt}
    (h₁ : ¬ (square (fine v) d ρ).all (T.inRect ℓ v.patch) = true)
    (h₂ : T.inRect ℓ v.patch (fine v + d) = true)
    (hc : T.crossEdge ℓ v (pointAt v.patch (fine v + d)) = some (Q, C)) :
    T.aroundStep ℓ ρ v d =
      if (square (C.apply (fine v)) (C.lin * d) ρ).all (T.inRect ℓ Q) then
        some (pointAt Q (C.apply (fine v)), ρ * (C.lin * d), C) else none := by
  unfold aroundStep; dsimp only; rw [ite_eq_right h₁, ite_eq_left h₂, hc]; rfl

theorem turnAt_stop (ℓ : α → ℕ) (f : ℕ) (v : Point Patch) (d : GaussianInt) :
    T.turnAt ℓ (f + 1) v d v d = some [] := by simp [turnAt]

theorem turnAt_step {ℓ : α → ℕ} {f : ℕ} {v v₀ v' : Point Patch} {d d₀ d' : GaussianInt}
    {C : Motion GaussianInt GaussianInt} (hne : ¬ (v = v₀ ∧ d = d₀))
    (h : T.aroundStep ℓ (-gi) v d = some (v', d', C)) :
    T.turnAt ℓ (f + 1) v d v₀ d₀ = (T.turnAt ℓ f v' d' v₀ d₀).map (C :: ·) := by
  rw [turnAt, ite_eq_right hne, h]; rfl

theorem normalize_self {ℓ : α → ℕ} {s : Step Patch}
    (h : (square (fine s.1) s.2 gi).all (T.inRect ℓ s.1.patch) = true) : T.normalize ℓ s = some s := by
  unfold normalize; dsimp only; rw [ite_eq_left h]

theorem normalize_cross {ℓ : α → ℕ} {s : Step Patch} {Q : Patch} {C : Motion GaussianInt GaussianInt}
    (h₁ : ¬ (square (fine s.1) s.2 gi).all (T.inRect ℓ s.1.patch) = true)
    (hc : T.crossEdge ℓ s.1 (pointAt s.1.patch (fine s.1 + s.2)) = some (Q, C)) :
    T.normalize ℓ s = some (pointAt Q (C.apply (fine s.1)), C.lin * s.2) := by
  unfold normalize; dsimp only; rw [ite_eq_right h₁, hc]; rfl

theorem vertexCrossings_eq {ℓ : α → ℕ} {fuel : ℕ} {s₁ s₂ : Step Patch} {u v₀ : Point Patch}
    {d d₀ : GaussianInt} (h₁ : T.normalize ℓ s₁ = some (u, d)) (h₂ : T.normalize ℓ s₂ = some (v₀, d₀)) :
    T.vertexCrossings ℓ fuel s₁ s₂ = T.turnAt ℓ fuel (pointAt u.patch (fine u + d)) (gi * d) v₀ d₀ := by
  unfold vertexCrossings; rw [h₁, h₂]; rfl

/-! ### The crossings along a list of steps -/

/-- The crossings between consecutive steps of a list, in order. -/
def chain (ℓ : α → ℕ) (fuel : ℕ) : List (Step Patch) → Option (List (Motion GaussianInt GaussianInt))
  | s₁ :: s₂ :: ss =>
    (T.vertexCrossings ℓ fuel s₁ s₂).bind fun c => (chain ℓ fuel (s₂ :: ss)).map (c ++ ·)
  | _ => some []

theorem mapM_zip_chain (ℓ : α → ℕ) (fuel : ℕ) (y : Step Patch) :
    ∀ (x : Step Patch) (l : List (Step Patch)),
      (((x :: l).zip (l ++ [y])).mapM fun p => T.vertexCrossings ℓ fuel p.1 p.2).map List.flatten =
        T.chain ℓ fuel (x :: l ++ [y])
  | x, [] => by
    simp only [List.nil_append, List.zip_cons_cons, List.zip_nil_right, List.mapM_cons,
      List.mapM_nil, chain, List.cons_append]
    cases T.vertexCrossings ℓ fuel x y <;> simp
  | x, z :: l => by
    have ih := mapM_zip_chain ℓ fuel y z l
    simp only [List.cons_append, List.zip_cons_cons, List.mapM_cons, chain] at ih ⊢
    rw [← ih]
    cases T.vertexCrossings ℓ fuel x z <;> cases (List.zip (z :: l) (l ++ [y])).mapM
      (fun p => T.vertexCrossings ℓ fuel p.1 p.2) <;> simp

/-- The holonomy of the border is the composite of the crossings of its closed list of steps. -/
theorem borderHolonomy_eq_chain (ℓ : α → ℕ) (fuel : ℕ) (s : Step Patch) (ss : List (Step Patch)) :
    T.borderHolonomy ℓ fuel (s :: ss) = (T.chain ℓ fuel (s :: ss ++ [s])).map Motion.composeAll := by
  rw [← mapM_zip_chain, borderHolonomy, Option.map_map]
  rfl

theorem chain_append_of {ℓ : α → ℕ} {fuel : ℕ} :
    ∀ (l₁ : List (Step Patch)) (x : Step Patch) (l₂ : List (Step Patch))
      (c₁ c₂ : List (Motion GaussianInt GaussianInt)),
      T.chain ℓ fuel (l₁ ++ [x]) = some c₁ → T.chain ℓ fuel (x :: l₂) = some c₂ →
      T.chain ℓ fuel (l₁ ++ x :: l₂) = some (c₁ ++ c₂)
  | [], x, l₂, c₁, c₂, h₁, h₂ => by
    simp only [List.nil_append, chain, Option.some.injEq] at h₁ ⊢
    rw [← h₁, h₂]; rfl
  | [y], x, l₂, c₁, c₂, h₁, h₂ => by
    simp only [List.cons_append, List.nil_append, chain] at h₁ ⊢
    obtain ⟨c, hc, h⟩ := Option.bind_eq_some_iff.mp h₁
    simp only [Option.map_some, Option.some.injEq, List.append_nil] at h
    subst h
    rw [hc, h₂]; rfl
  | y :: z :: l₁, x, l₂, c₁, c₂, h₁, h₂ => by
    simp only [List.cons_append, chain] at h₁ ⊢
    obtain ⟨c, hc, h⟩ := Option.bind_eq_some_iff.mp h₁
    obtain ⟨c', hc', rfl⟩ := Option.map_eq_some_iff.mp h
    have ih := chain_append_of (z :: l₁) x l₂ c' c₂ (by simpa using hc') h₂
    simp only [List.cons_append] at ih
    rw [hc, Option.bind_some, ih, Option.map_some, List.append_assoc]

/-- **The crossings along a run of steps**, each vertex carrying the frame of a step to the
frame of the next: the composite carries the frame of the first step to the frame after
the last. -/
theorem chain_range {ℓ : α → ℕ} {fuel : ℕ} (f : ℕ → Step Patch) (F : ℕ → Motion GaussianInt GaussianInt)
    (nx : Step Patch) (G : Motion GaussianInt GaussianInt) :
    ∀ n : ℕ,
      (∀ i, i + 1 < n + 1 → ∃ cs, T.vertexCrossings ℓ fuel (f i) (f (i + 1)) = some cs ∧
        Motion.comp (Motion.composeAll cs) (F i) = F (i + 1)) →
      (∃ cs, T.vertexCrossings ℓ fuel (f n) nx = some cs ∧
        Motion.comp (Motion.composeAll cs) (F n) = G) →
      ∃ cs, T.chain ℓ fuel ((List.range (n + 1)).map f ++ [nx]) = some cs ∧
        Motion.comp (Motion.composeAll cs) (F 0) = G
  | 0, _, ⟨cs, h, hG⟩ => ⟨cs, by simp [chain, h], hG⟩
  | n + 1, hstep, hlast => by
    obtain ⟨c₁, h₁, e₁⟩ := chain_range f F (f (n + 1)) (F (n + 1)) n
      (fun i hi => hstep i (by omega)) (hstep n (by omega))
    obtain ⟨c₂, h₂, e₂⟩ := hlast
    refine ⟨c₁ ++ c₂, ?_, ?_⟩
    · rw [List.range_succ, List.map_append, List.map_singleton, List.append_assoc,
        List.singleton_append]
      exact T.chain_append_of _ _ _ c₁ c₂ h₁ (by simp [chain, h₂])
    · rw [Motion.composeAll_append, Motion.comp_assoc, e₁, e₂]

/-! ### A step round a vertex, walked back -/

theorem square_map (C : Motion GaussianInt GaussianInt) (z d ρ : GaussianInt) :
    (square z d ρ).map C.apply = square (C.apply z) (C.lin * d) ρ := by
  simp only [square, List.map_cons, List.map_nil, apply_eq]
  refine List.cons_eq_cons.mpr ⟨rfl, List.cons_eq_cons.mpr ⟨by ring,
    List.cons_eq_cons.mpr ⟨by ring, List.cons_eq_cons.mpr ⟨by ring, rfl⟩⟩⟩⟩

theorem square_last (z d ρ : GaussianInt) (f : GaussianInt → Bool) (h : (square z d ρ).all f = true) :
    f (z + ρ * d) = true := by
  simp only [square, List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true] at h
  exact h.2.2.2

theorem square_second (z d ρ : GaussianInt) (f : GaussianInt → Bool) (h : (square z d ρ).all f = true) :
    f (z + d) = true := by
  simp only [square, List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true] at h
  exact h.2.1

theorem square_first (z d ρ : GaussianInt) (f : GaussianInt → Bool) (h : (square z d ρ).all f = true) :
    f z = true := by
  simp only [square, List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true] at h
  exact h.1

/-- The fine edge from `v` along `d` found on the side `τ`, in the frame of that side. -/
theorem onSide_framePt (ℓ : α → ℕ) {v : Point Patch} {d : GaussianInt} (hd : IsQuarter d)
    (h₂ : T.inRect ℓ v.patch (fine v + d) = true) {τ : Side Patch} {t : ℕ}
    (hs : T.onSide ℓ v (pointAt v.patch (fine v + d)) = some (τ, t)) :
    τ.1 = v.patch ∧ ∃ x e : ℤ, (e = 1 ∨ e = -1) ∧ fine v = T.framePt ℓ τ x 0 ∧ d = dir τ e 0 ∧
      (t : ℤ) ≤ x ∧ x ≤ t + 1 ∧ (t : ℤ) ≤ x + e ∧ x + e ≤ t + 1 := by
  have hq := T.fine_pointAt_of_inRect h₂
  obtain ⟨hP, hpts⟩ := T.onSide_some ℓ hd (p := v) (q := pointAt v.patch (fine v + d)) rfl hq hs
  refine ⟨hP, ?_⟩
  rcases hpts with ⟨hv, hw⟩ | ⟨hv, hw⟩
  · refine ⟨t, 1, Or.inl rfl, by rw [hv, fine_sidePoint_eq_framePt], ?_, le_rfl, by omega,
      by omega, le_rfl⟩
    have e1 := congrArg fine hw
    rw [hq, hv, fine_sidePoint_eq_framePt, fine_sidePoint_eq_framePt] at e1
    have e2 := T.framePt_add_dir ℓ τ t 0 1 0
    push_cast at e1 e2
    linear_combination e1 - e2
  · refine ⟨t + 1, -1, Or.inr rfl, by rw [hv, fine_sidePoint_eq_framePt]; push_cast; rfl, ?_,
      by omega, le_rfl, by omega, by omega⟩
    have e1 := congrArg fine hw
    rw [hq, hv, fine_sidePoint_eq_framePt, fine_sidePoint_eq_framePt] at e1
    have e2 := T.framePt_add_dir ℓ τ (t + 1) 0 (-1) 0
    push_cast at e1 e2
    simp only [add_neg_cancel_right] at e2
    linear_combination e1 - e2

/-- **A step round a vertex, walked back.** If the step `ρ`-wise from the edge `(v, d)`
crosses into the frame of the face entered, the step `(-ρ)`-wise from the image of the
edge crosses back, by the inverse change of chart; the face left must lie in the
rectangle of `v`. -/
theorem aroundStep_reverse [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) {ρ : GaussianInt} (hρ : ρ = gi ∨ ρ = -gi)
    {v : Point Patch} {d : GaussianInt} (hd : IsQuarter d)
    (hinv : (square (fine v) d (-ρ)).all (T.inRect ℓ v.patch) = true)
    {v' : Point Patch} {d' : GaussianInt} {C : Motion GaussianInt GaussianInt}
    (h : T.aroundStep ℓ ρ v d = some (v', d', C)) :
    ∃ C', T.aroundStep ℓ (-ρ) v' (C.lin * d) = some (v, -ρ * d, C') ∧ Motion.comp C' C = Motion.id := by
  unfold aroundStep at h
  dsimp only at h
  split_ifs at h with h1 h2
  · simp only [Option.some.injEq, Prod.mk.injEq] at h
    obtain ⟨rfl, rfl, rfl⟩ := h
    exact ⟨Motion.id, by rw [Motion.id_lin, one_mul]; exact T.aroundStep_in hinv, by simp⟩
  obtain ⟨⟨Q, C'⟩, hc, h'⟩ := Option.bind_eq_some_iff.mp h
  simp only at h'
  split_ifs at h' with h3
  simp only [Option.some.injEq, Prod.mk.injEq] at h'
  obtain ⟨h₁, h₂', h₃⟩ := h'
  subst h₁ h₂' h₃
  have hc' := hc
  unfold crossEdge at hc
  cases hs : T.onSide ℓ v (pointAt v.patch (fine v + d)) with
  | none =>
    rw [hs] at hc
    simp only [Option.some.injEq, Prod.mk.injEq] at hc
    obtain ⟨rfl, rfl⟩ := hc
    exact absurd (by simpa [Motion.apply, Motion.id] using h3) h1
  | some τt =>
    obtain ⟨τ, t⟩ := τt
    rw [hs] at hc
    obtain ⟨a, ha, he⟩ := Option.map_eq_some_iff.mp hc
    simp only [Prod.mk.injEq] at he
    obtain ⟨rfl, rfl⟩ := he
    obtain ⟨hmem, ht₁, ht₂⟩ := (arcOn_eq_some ℓ (T.arcs_nodup τ) t a).mp ha
    obtain ⟨hP, x, e, he, hz, rfl, hx₁, hx₂, hx₃, hx₄⟩ := T.onSide_framePt ℓ hd h2 hs
    set D := fineMotion (T.crossMotion ℓ τ a)
    have hmem' := T.mem_arcs_other hmem
    -- The face left lies inward of `τ`: `-ρ` turns the edge inward.
    have hin : -ρ * dir τ e 0 = inward τ := by
      have hl := square_last _ _ _ _ hinv
      rw [hz, ← hP] at hl
      rcases hρ with rfl | rfl
      · rw [neg_gi_mul_dir, framePt_add_dir, inRect_framePt] at hl
        rw [neg_gi_mul_dir, ← dir_zero_one]
        congr 1 <;> rcases orient_cases τ with ho | ho <;> rcases he with rfl | rfl <;>
          simp_all
      · rw [neg_neg, gi_mul_dir, framePt_add_dir, inRect_framePt] at hl
        rw [neg_neg, gi_mul_dir, ← dir_zero_one]
        congr 1 <;> rcases orient_cases τ with ho | ho <;> rcases he with rfl | rfl <;>
          simp_all
    -- The image of the edge, on `(T.other a τ)`.
    have hDz : D.apply (fine v) = T.framePt ℓ (T.other a τ) (T.sign a * x + 2 * T.crossTrans ℓ τ a) 0 := by
      rw [hz, T.crossMotion_framePt hT ℓ hmem, neg_zero]
    have hDd : D.lin * dir τ e 0 = dir (T.other a τ) (T.sign a * e) 0 := by
      rw [T.crossMotion_lin_dir hT ℓ hmem, neg_zero]
    have hv' : fine (pointAt (T.other a τ).1 (D.apply (fine v))) = D.apply (fine v) :=
      T.fine_pointAt_of_inRect (square_first _ _ _ _ h3)
    have hr := T.img_range (τ := τ) (a := a) ℓ (x := x) (by omega) (by omega)
    have hr' := T.img_range (τ := τ) (a := a) ℓ (x := x + e) (by omega) (by omega)
    obtain ⟨hp1, hp2⟩ := hr
    obtain ⟨hp3, hp4⟩ := hr'
    have hsg := T.sign_cases a
    -- Back across the edge of `(T.other a τ)`.
    have hback : T.crossEdge ℓ (pointAt (T.other a τ).1 (D.apply (fine v)))
        (pointAt (pointAt (T.other a τ).1 (D.apply (fine v))).patch
          (fine (pointAt (T.other a τ).1 (D.apply (fine v))) + D.lin * dir τ e 0)) =
        some ((T.other a (T.other a τ)).1, fineMotion (T.crossMotion ℓ (T.other a τ) a)) := by
      rw [hv', hDz, hDd, pointAt_patch, pointAt_framePt_add_dir, add_zero]
      change T.crossEdge ℓ (T.fpt ℓ (T.other a τ) _ 0) (T.fpt ℓ (T.other a τ) _ 0) = _
      have hy : T.sign a * (x + e) + 2 * T.crossTrans ℓ τ a =
          T.sign a * x + 2 * T.crossTrans ℓ τ a + T.sign a * e := by ring
      rcases hsg with hs1 | hs1 <;> rcases he with rfl | rfl <;> rw [hs1] at hp1 hp2 hp3 hp4 ⊢
      · have := (T.crossEdge_framePt ℓ (hH (T.other a τ)) hmem' (x := 1 * x + 2 * T.crossTrans ℓ τ a)
          (by omega) (by omega) (by omega)).1
        simpa using this
      · have := (T.crossEdge_framePt ℓ (hH (T.other a τ)) hmem' (x := 1 * x + 2 * T.crossTrans ℓ τ a + 1 * -1)
          (by omega) (by omega) (by omega)).2
        simpa [add_assoc] using this
      · have := (T.crossEdge_framePt ℓ (hH (T.other a τ)) hmem' (x := -1 * x + 2 * T.crossTrans ℓ τ a + -1 * 1)
          (by omega) (by omega) (by omega)).2
        simpa [add_assoc] using this
      · have := (T.crossEdge_framePt ℓ (hH (T.other a τ)) hmem' (x := -1 * x + 2 * T.crossTrans ℓ τ a)
          (by omega) (by omega) (by omega)).1
        simpa using this
    set D' := fineMotion (T.crossMotion ℓ (T.other a τ) a)
    have hinvD : Motion.comp D' D = Motion.id := T.crossMotion_inv hT ℓ hmem
    refine ⟨D', ?_, hinvD⟩
    have hD'z : D'.apply (D.apply (fine v)) = fine v := by
      rw [← Motion.apply_comp, hinvD]; simp [Motion.apply, Motion.id]
    have hD'l : D'.lin * (D.lin * dir τ e 0) = dir τ e 0 := by
      rw [← mul_assoc, ← Motion.comp_lin, hinvD, Motion.id_lin, one_mul]
    have h1' : ¬ (square (fine (pointAt (T.other a τ).1 (D.apply (fine v)))) (D.lin * dir τ e 0)
        (-ρ)).all (T.inRect ℓ (T.other a τ).1) = true := by
      intro hc
      have hl := square_last _ _ _ _ hc
      have e1 : D.apply (fine v) + -ρ * (D.lin * dir τ e 0) = D.apply (T.framePt ℓ τ x 1) := by
        have e2 := T.framePt_add_dir ℓ τ x 0 0 1
        rw [zero_add, add_zero, dir_zero_one] at e2
        rw [← e2, ← hin, ← hz, apply_eq, apply_eq]; ring
      rw [hv', e1, T.crossMotion_framePt hT ℓ hmem, inRect_framePt] at hl
      omega
    have h2' : T.inRect ℓ (T.other a τ).1
        (fine (pointAt (T.other a τ).1 (D.apply (fine v))) + D.lin * dir τ e 0) = true := by
      rw [hv']; exact square_second _ _ _ _ h3
    rw [T.aroundStep_cross h1' h2' hback, hv', hD'z, hD'l, T.other_other hmem, hP, pointAt_fine,
      ite_eq_left hinv]

theorem square_rot_all {ρ : GaussianInt} (hρ : ρ * ρ = -1) (z e : GaussianInt) (f : GaussianInt → Bool) :
    (square z (ρ * e) (-ρ)).all f = (square z e ρ).all f := by
  simp only [square, List.all_cons, List.all_nil, Bool.and_true]
  rw [show -ρ * (ρ * e) = e by linear_combination (-e) * hρ, show z + ρ * e + e = z + e + ρ * e by ring]
  cases f z <;> cases f (z + ρ * e) <;> cases f (z + e + ρ * e) <;> cases f (z + e) <;> rfl

/-- After a step round a vertex, the face entered lies in the rectangle of the frame
reached: it is on the side `-ρ` of the edge reached. -/
theorem aroundStep_right_face {ℓ : α → ℕ} {ρ : GaussianInt} (hρ : ρ * ρ = -1) {v : Point Patch}
    {d : GaussianInt} {v' : Point Patch} {d' : GaussianInt} {C : Motion GaussianInt GaussianInt}
    (h : T.aroundStep ℓ ρ v d = some (v', d', C)) :
    (square (fine v') d' (-ρ)).all (T.inRect ℓ v'.patch) = true := by
  unfold aroundStep at h
  dsimp only at h
  split_ifs at h with h1 h2
  · simp only [Option.some.injEq, Prod.mk.injEq] at h
    obtain ⟨rfl, rfl, rfl⟩ := h
    rw [square_rot_all hρ]; exact h1
  · obtain ⟨⟨Q, C'⟩, hc, h'⟩ := Option.bind_eq_some_iff.mp h
    simp only at h'
    split_ifs at h' with h3
    simp only [Option.some.injEq, Prod.mk.injEq] at h'
    obtain ⟨h₁, h₂, h₃⟩ := h'
    subst h₁ h₂ h₃
    rw [pointAt_patch, T.fine_pointAt_of_inRect (square_first _ _ _ _ h3), square_rot_all hρ]
    exact h3

/-! ### The station arithmetic of the walk -/

theorem station_floor (k : ℕ) : ((2 * ((k : ℚ) + 1 / 2) - 1) / 2).floor.toNat = k := by
  have h : (2 * ((k : ℚ) + 1 / 2) - 1) / 2 = ((k : ℤ) : ℚ) := by push_cast; ring
  rw [h, Rat.floor_intCast, Int.toNat_natCast]

theorem crossAt_apply_eq [Fintype α] (ℓ : α → ℕ) (τ : Side Patch) (a : α) (x : ℚ) :
    (T.crossAt (rat ℓ) τ a).apply x = (T.sign a : ℚ) * x + (T.crossTrans ℓ τ a : ℚ) := by
  rw [T.crossTrans_rat ℓ τ a]
  unfold crossAt
  split_ifs with h
  · simp [Motion.apply, h]
  · rcases T.sign_cases a with h' | h'
    · exact absurd h' h
    · simp [Motion.apply, h']

/-- **One crossing of the walk from a half-integer station**: the arc found holds the
station in its interior, and the walk enters the other side at a half-integer station,
the fine positions crossing as the points of the arc do. -/
theorem station_step [Fintype α] (ℓ : α → ℕ) {ω : Side Patch} {a : α} {k : ℕ}
    (h : arcAt (rat ℓ) (T.arcs ω) ((k : ℚ) + 1 / 2) = some a) :
    a ∈ T.arcs ω ∧ 2 * (posNat a (T.arcs ω) ℓ : ℤ) + 1 ≤ 2 * k + 1 ∧
      (2 * k + 1 : ℤ) + 1 ≤ 2 * posNat a (T.arcs ω) ℓ + 2 * ℓ a ∧
      ∃ k' : ℕ, (T.crossAt (rat ℓ) ω a).apply ((k : ℚ) + 1 / 2) = (k' : ℚ) + 1 / 2 ∧
        (2 * k' + 1 : ℤ) = T.sign a * (2 * k + 1) + 2 * T.crossTrans ℓ ω a := by
  have hℓ : ∀ b, (0 : ℚ) ≤ rat ℓ b := fun b => by simp [rat]
  obtain ⟨ha, h₁, h₂⟩ := (arcAt_eq_some hℓ (T.arcs_nodup ω) _ a).mp h
  rw [posAt_rat] at h₁ h₂
  simp only [rat] at h₂
  have hp : posNat a (T.arcs ω) ℓ ≤ k := by
    by_contra hc
    have : (k : ℚ) + 1 ≤ posNat a (T.arcs ω) ℓ := by exact_mod_cast (Nat.lt_of_not_le hc)
    linarith
  have hk : k < posNat a (T.arcs ω) ℓ + ℓ a := by
    by_contra hc
    have : (posNat a (T.arcs ω) ℓ : ℚ) + ℓ a ≤ k := by exact_mod_cast (Nat.le_of_not_lt hc)
    linarith
  refine ⟨ha, by omega, by omega, ?_⟩
  rw [T.crossAt_apply_eq]
  have hr := T.img_range (τ := ω) (a := a) ℓ (x := 2 * k + 1) (by omega) (by omega)
  obtain ⟨k', hk'⟩ : ∃ k' : ℕ, (2 * k' + 1 : ℤ) = T.sign a * (2 * k + 1) + 2 * T.crossTrans ℓ ω a := by
    unfold crossTrans at hr ⊢
    rcases T.sign_cases a with hs | hs <;> simp only [hs] at hr ⊢ <;> simp at hr ⊢
    · exact ⟨k + posNat a (T.arcs (T.other a ω)) ℓ - posNat a (T.arcs ω) ℓ, by omega⟩
    · exact ⟨posNat a (T.arcs ω) ℓ + posNat a (T.arcs (T.other a ω)) ℓ + ℓ a - k - 1, by omega⟩
  refine ⟨k', ?_, hk'⟩
  have hq : ((2 * k' + 1 : ℤ) : ℚ) = ((T.sign a * (2 * k + 1) + 2 * T.crossTrans ℓ ω a : ℤ) : ℚ) := by
    rw [hk']
  push_cast at hq
  linarith

/-! ### The steps of a round in the frames of the sides -/

theorem range_succ_reverse_dropLast (n : ℕ) :
    (List.range (n + 1)).reverse.dropLast = (List.range n).map fun j => n - j := by
  apply List.ext_getElem
  · simp
  · intro i h₁ h₂
    simp only [List.getElem_dropLast, List.getElem_reverse, List.getElem_range, List.getElem_map,
      List.length_range]
    omega

theorem range_succ_dropLast (n : ℕ) : (List.range (n + 1)).dropLast = List.range n := by
  rw [List.range_succ, List.dropLast_concat]

/-- The steps across a rectangle in the frame of the side entered. -/
theorem acrossSteps_eq (ℓ : α → ℕ) (τ : Side Patch) (k : ℕ) :
    T.acrossSteps ℓ τ k =
      (List.range (2 * T.lenAcross ℓ τ)).map fun j : ℕ => (T.fpt ℓ τ (2 * k + 1) (j : ℤ), inward τ) := by
  obtain ⟨P, j, e⟩ := τ
  unfold acrossSteps across
  fin_cases j <;> fin_cases e <;>
    simp only [Fin.zero_eta, Fin.isValue, ↓reduceIte, Fin.mk_one, one_ne_zero, ← List.map_dropLast,
      List.map_map, range_succ_reverse_dropLast, range_succ_dropLast, lenAcross] <;>
    apply List.map_congr_left <;> intro i hi <;> rw [List.mem_range] at hi <;>
    simp [fpt, framePt, origin, acrossDir, inward, pointAt, gi, height, width] <;> omega

/-- The steps along a side in the frame of that side, forwards. -/
theorem alongSteps_eq_le (ℓ : α → ℕ) (σ : Side Patch) {s t : ℕ} (h : s ≤ t) :
    T.alongSteps ℓ σ s t =
      (List.range (t - s)).map fun i : ℕ => (T.fpt ℓ σ ((s : ℤ) + 1 * (i : ℤ)) 0, dir σ 1 0) := by
  unfold alongSteps alongSide
  rw [ite_eq_left h, ite_eq_left h]
  simp only [← List.map_dropLast, List.map_map,
    range_succ_dropLast]
  apply List.map_congr_left
  intro i _
  simp only [Function.comp_apply, Prod.mk.injEq]
  refine ⟨?_, by simp [dir]⟩
  rw [show ((s : ℤ) + 1 * i) = ((s + i : ℕ) : ℤ) by push_cast; ring, fpt_nat]

/-- The steps along a side in the frame of that side, backwards. -/
theorem alongSteps_eq_gt (ℓ : α → ℕ) (σ : Side Patch) {s t : ℕ} (h : t < s) :
    T.alongSteps ℓ σ s t =
      (List.range (s - t)).map fun i : ℕ => (T.fpt ℓ σ ((s : ℤ) + -1 * (i : ℤ)) 0, dir σ (-1) 0) := by
  unfold alongSteps alongSide
  rw [ite_eq_right (by omega), ite_eq_right (by omega)]
  simp only [← List.map_dropLast, List.map_map,
    range_succ_dropLast]
  apply List.map_congr_left
  intro i hi
  rw [List.mem_range] at hi
  simp only [Function.comp_apply, Prod.mk.injEq]
  refine ⟨?_, by simp [dir]⟩
  rw [show ((s : ℤ) + -1 * i) = ((s - i : ℕ) : ℤ) by omega, fpt_nat]

/-! ### The crossings at the vertices of a round -/

/-- Decides whether the corners of a square, in the frame of the side `o` names, lie in its
rectangle. -/
local macro "rect_by " o:term : tactic =>
  `(tactic| (simp only [List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true,
      inRect_framePt]; rcases orient_cases $o with ho | ho <;> simp only [ho] <;> omega))

theorem lenAlong_pos {ℓ : α → ℕ} (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) (τ : Side Patch) :
    0 < T.lenAlong ℓ τ := by
  have h := hH (τ.1, τ.2.1 + 1, τ.2.2)
  obtain ⟨P, j, e⟩ := τ
  fin_cases j <;> simp_all [lenAlong, lenAcross]

theorem lenAcross_opp (ℓ : α → ℕ) (τ : Side Patch) : T.lenAcross ℓ τ.opp = T.lenAcross ℓ τ := rfl
theorem lenAlong_opp (ℓ : α → ℕ) (τ : Side Patch) : T.lenAlong ℓ τ.opp = T.lenAlong ℓ τ := rfl

theorem fine_fpt_of {ℓ : α → ℕ} {τ : Side Patch} {s r : ℤ} (h₁ : 0 ≤ s) (h₂ : s ≤ 2 * T.lenAlong ℓ τ)
    (h₃ : 0 ≤ r) (h₄ : r ≤ 2 * T.lenAcross ℓ τ) : fine (T.fpt ℓ τ s r) = T.framePt ℓ τ s r :=
  T.fine_fpt ((T.inRect_framePt ℓ τ s r).mpr ⟨h₁, h₂, h₃, h₄⟩)

/-- Inside a rectangle, between two steps of the dual line across it, the chart crosses
one interior edge: the identity. -/
theorem vc_across (ℓ : α → ℕ) {τ : Side Patch} {s j : ℤ} (hs₁ : 1 ≤ s)
    (hs₂ : s + 1 ≤ 2 * T.lenAlong ℓ τ) (hj₀ : 0 ≤ j) (hj : j + 2 ≤ 2 * T.lenAcross ℓ τ) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ τ s j, inward τ) (T.fpt ℓ τ s (j + 1), inward τ) =
      some [Motion.id] := by
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := τ) (s := s) (r := j) (by omega) (by omega) hj₀ (by omega)
  have f2 := T.fine_fpt_of (ℓ := ℓ) (τ := τ) (s := s) (r := j + 1) (by omega) (by omega) (by omega)
    (by omega)
  have n1 : T.normalize ℓ (T.fpt ℓ τ s j, inward τ) = some (T.fpt ℓ τ s j, inward τ) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f1, ← dir_zero_one, square_gi]
    rect_by τ
  have n2 : T.normalize ℓ (T.fpt ℓ τ s (j + 1), inward τ) = some (T.fpt ℓ τ s (j + 1), inward τ) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f2, ← dir_zero_one, square_gi]
    rect_by τ
  rw [T.vertexCrossings_eq n1 n2, fpt_patch, f1, ← dir_zero_one, pointAt_framePt_add_dir, gi_mul_dir,
    add_zero, mul_one, mul_zero]
  have hstep : T.aroundStep ℓ (-gi) (T.fpt ℓ τ s (j + 1)) (dir τ (-τ.orient) 0) =
      some (T.fpt ℓ τ s (j + 1), dir τ 0 1, Motion.id) := by
    rw [T.aroundStep_in]
    · rw [neg_gi_mul_dir]
      rcases orient_cases τ with ho | ho <;> simp [ho]
    · simp only [fpt_patch]
      rw [f2, square_neg_gi]
      rect_by τ
  rw [T.turnAt_step _ hstep, turnAt_stop]
  · rfl
  · rintro ⟨-, h⟩
    have := (dir_inj h).2
    omega

/-- **Out of a rectangle through a side**: from the end of the dual line on the side `ω`,
at the fine position `s` inside the arc `a`, the chart turns clockwise across the edge
of `ω` and crosses the arc, into the frame of the other side, where the walk goes on
inward. -/
theorem exit_step [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ) (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {ω : Side Patch} {a : α}
    (ha : a ∈ T.arcs ω) {s : ℤ} (hs₁ : 2 * (posNat a (T.arcs ω) ℓ : ℤ) + 1 ≤ s)
    (hs₂ : s + 1 ≤ 2 * (posNat a (T.arcs ω) ℓ : ℤ) + 2 * ℓ a) :
    T.aroundStep ℓ (-gi) (T.fpt ℓ ω s 0) (dir ω ω.orient 0) =
      some (T.fpt ℓ (T.other a ω) (T.sign a * s + 2 * T.crossTrans ℓ ω a) 0, inward (T.other a ω),
        fineMotion (T.crossMotion ℓ ω a)) := by
  have hl := posNat_add_le ℓ ha
  have hl' := posNat_add_le ℓ (T.mem_arcs_other ha)
  rw [← sideLen, hbal] at hl hl'
  have hA := hH ω
  have hA' := hH (T.other a ω)
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := ω) (s := s) (r := 0) (by omega) (by omega) le_rfl
    (by omega)
  have hsg := T.sign_eq_orient hT ha
  have hr := T.img_range (τ := ω) (a := a) ℓ (x := s) (by omega) (by omega)
  have hr' := T.img_range (τ := ω) (a := a) ℓ (x := s + ω.orient) (by
    rcases orient_cases ω with ho | ho <;> rw [ho] <;> omega) (by
    rcases orient_cases ω with ho | ho <;> rw [ho] <;> omega)
  have h1 : ¬ (square (fine (T.fpt ℓ ω s 0)) (dir ω ω.orient 0) (-gi)).all
      (T.inRect ℓ (T.fpt ℓ ω s 0).patch) = true := by
    intro h
    have hl := square_last _ _ _ _ h
    rw [f1, fpt_patch, neg_gi_mul_dir, framePt_add_dir, inRect_framePt] at hl
    rcases orient_cases ω with ho | ho <;> simp [ho] at hl
  have h2 : T.inRect ℓ (T.fpt ℓ ω s 0).patch (fine (T.fpt ℓ ω s 0) + dir ω ω.orient 0) = true := by
    rw [f1, fpt_patch, framePt_add_dir, inRect_framePt]
    rcases orient_cases ω with ho | ho <;> simp only [ho] <;> omega
  have hc : T.crossEdge ℓ (T.fpt ℓ ω s 0)
      (pointAt (T.fpt ℓ ω s 0).patch (fine (T.fpt ℓ ω s 0) + dir ω ω.orient 0)) =
      some ((T.other a ω).1, fineMotion (T.crossMotion ℓ ω a)) := by
    rw [f1, fpt_patch, pointAt_framePt_add_dir, add_zero]
    rcases orient_cases ω with ho | ho <;> rw [ho]
    · exact (T.crossEdge_framePt ℓ hA ha (x := s) (by omega) (by omega) (by omega)).1
    · have := (T.crossEdge_framePt ℓ hA ha (x := s + -1) (by omega) (by omega) (by omega)).2
      simpa using this
  rw [T.aroundStep_cross h1 h2 hc, f1, T.crossMotion_framePt hT ℓ ha, T.crossMotion_lin_dir hT ℓ ha,
    neg_zero, square_neg_gi, ite_eq_left]
  · rw [neg_gi_mul_dir]
    congr 2
    rw [hsg]
    rcases orient_cases ω with ho | ho <;> rcases orient_cases (T.other a ω) with ho' | ho' <;>
      simp [ho, ho']
  · simp only [List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true, inRect_framePt]
    rw [hsg] at hr hr' ⊢
    rcases orient_cases ω with ho | ho <;> rcases orient_cases (T.other a ω) with ho' | ho' <;>
      simp only [ho, ho'] at hr hr' ⊢ <;> omega

theorem framePt_opp (ℓ : α → ℕ) (τ : Side Patch) (s r : ℤ) :
    T.framePt ℓ τ.opp s r = T.framePt ℓ τ s (2 * T.lenAcross ℓ τ - r) := by
  obtain ⟨P, j, e⟩ := τ
  fin_cases j <;> fin_cases e <;> ext <;>
    simp [framePt, origin, acrossDir, inward, Side.opp, lenAcross, gi] <;> ring

theorem dir_opp (τ : Side Patch) (x y : ℤ) : dir τ.opp x y = dir τ x (-y) := by
  simp only [dir, acrossDir_opp, inward_opp]; push_cast; ring

theorem fpt_opp (ℓ : α → ℕ) (τ : Side Patch) (s : ℤ) :
    T.fpt ℓ τ.opp s 0 = T.fpt ℓ τ s (2 * T.lenAcross ℓ τ) := by
  unfold fpt; rw [framePt_opp, sub_zero]; rfl

/-- Two sides of one rectangle share no point at an odd fine position. -/
theorem framePt_side_ne (ℓ : α → ℕ) {τ τ' : Side Patch} (hP : τ.1 = τ'.1) (hne : τ ≠ τ')
    {s s' : ℤ} (hs : s % 2 = 1) (hw : 0 < T.width ℓ τ.1) (hh : 0 < T.height ℓ τ.1) :
    T.framePt ℓ τ s 0 ≠ T.framePt ℓ τ' s' 0 := by
  obtain ⟨P, j, e⟩ := τ
  obtain ⟨P', j', e'⟩ := τ'
  simp only at hP
  subst hP
  intro h
  have hre := congrArg Zsqrtd.re h
  have him := congrArg Zsqrtd.im h
  fin_cases j <;> fin_cases e <;> fin_cases j' <;> fin_cases e' <;>
    simp [framePt, origin, acrossDir, inward, gi] at hre him hne hw hh <;> omega

theorem width_pos {ℓ : α → ℕ} (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) (P : Patch) :
    0 < T.width ℓ P := by simpa [lenAcross] using hH (P, 1, 0)

theorem height_pos {ℓ : α → ℕ} (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) (P : Patch) :
    0 < T.height ℓ P := by simpa [lenAcross] using hH (P, 0, 0)

/-- Points at odd positions of two distinct sides are distinct. -/
theorem fpt_ne {ℓ : α → ℕ} (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) {τ τ' : Side Patch}
    (hne : τ ≠ τ') {s s' : ℤ} (hs : s % 2 = 1) (h₁ : 0 ≤ s) (h₂ : s ≤ 2 * T.lenAlong ℓ τ)
    (h₁' : 0 ≤ s') (h₂' : s' ≤ 2 * T.lenAlong ℓ τ') : T.fpt ℓ τ s 0 ≠ T.fpt ℓ τ' s' 0 := by
  intro h
  have hP : τ.1 = τ'.1 := by simpa using congrArg Point.patch h
  have hf := congrArg fine h
  rw [T.fine_fpt_of h₁ h₂ le_rfl (by omega), T.fine_fpt_of h₁' h₂' le_rfl (by omega)] at hf
  exact T.framePt_side_ne ℓ hP hne hs (T.width_pos hH _) (T.height_pos hH _) hf

/-- The last step of the dual line across a rectangle, from `τ`: it ends on the opposite
side, where the chart turns clockwise from the edge on its left. -/
theorem vc_last (ℓ : α → ℕ) {τ : Side Patch} (hH : 0 < T.lenAcross ℓ τ) {s : ℤ} (hs₁ : 1 ≤ s)
    (hs₂ : s + 1 ≤ 2 * T.lenAlong ℓ τ) {nx : Step Patch} {v₀ : Point Patch} {d₀ : GaussianInt}
    (hn : T.normalize ℓ nx = some (v₀, d₀)) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ τ s (2 * T.lenAcross ℓ τ - 1), inward τ) nx =
      T.turnAt ℓ 4 (T.fpt ℓ τ.opp s 0) (dir τ.opp τ.opp.orient 0) v₀ d₀ := by
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := τ) (s := s) (r := 2 * T.lenAcross ℓ τ - 1) (by omega)
    (by omega) (by omega) (by omega)
  have n1 : T.normalize ℓ (T.fpt ℓ τ s (2 * T.lenAcross ℓ τ - 1), inward τ) =
      some (T.fpt ℓ τ s (2 * T.lenAcross ℓ τ - 1), inward τ) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f1, ← dir_zero_one, square_gi]
    rect_by τ
  rw [T.vertexCrossings_eq n1 hn, fpt_patch, f1, ← dir_zero_one, pointAt_framePt_add_dir, gi_mul_dir,
    fpt_opp, dir_opp, Side.orient_opp]
  congr 2 <;> simp

/-- **From one rectangle to the next**: at the end of the dual line across a rectangle,
the chart crosses the arc the walk crosses, by its change of chart, into the frame of
the first step across the next rectangle. -/
theorem vc_transition [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ.opp) {s : ℤ} (hodd : s % 2 = 1)
    (hs₁ : 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 1 ≤ s)
    (hs₂ : s + 1 ≤ 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 2 * ℓ a) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ τ s (2 * T.lenAcross ℓ τ - 1), inward τ)
        (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
          inward (T.other a τ.opp)) =
      some [fineMotion (T.crossMotion ℓ τ.opp a)] := by
  have hl := posNat_add_le ℓ ha
  have hl' := posNat_add_le ℓ (T.mem_arcs_other ha)
  rw [← sideLen, hbal] at hl hl'
  have hlo : T.lenAlong ℓ τ.opp = T.lenAlong ℓ τ := rfl
  have hr₁ := T.img_range (τ := τ.opp) (a := a) ℓ (x := s - 1) (by omega) (by omega)
  have hr₂ := T.img_range (τ := τ.opp) (a := a) ℓ (x := s + 1) (by omega) (by omega)
  have hs' : 1 ≤ T.sign a * s + 2 * T.crossTrans ℓ τ.opp a ∧
      T.sign a * s + 2 * T.crossTrans ℓ τ.opp a + 1 ≤ 2 * T.lenAlong ℓ (T.other a τ.opp) := by
    rcases T.sign_cases a with h | h <;> rw [h] at hr₁ hr₂ ⊢ <;> constructor <;> omega
  have hH' := hH (T.other a τ.opp)
  have f2 := T.fine_fpt_of (ℓ := ℓ) (τ := T.other a τ.opp)
    (s := T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) (r := 0) (by omega) (by omega) le_rfl (by omega)
  have n2 : T.normalize ℓ (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
      inward (T.other a τ.opp)) =
      some (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
        inward (T.other a τ.opp)) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f2, ← dir_zero_one, square_gi]
    rect_by (T.other a τ.opp)
  rw [T.vc_last ℓ (hH τ) (by omega) (by omega) n2, T.turnAt_step _ (T.exit_step hT ℓ hH hbal ha hs₁ hs₂),
    turnAt_stop]
  · rfl
  · rintro ⟨h, -⟩
    exact T.fpt_ne hH (T.other_ne ha).symm hodd (by omega) (by omega) (by omega) (by omega) h

/-- The crossings along the dual line across a rectangle, from the station `k + ½` of
`τ`: the identity at each vertex inside, then whatever happens at its end. -/
theorem chain_across (ℓ : α → ℕ) {τ : Side Patch} (hH : 0 < T.lenAcross ℓ τ) {k : ℕ}
    (hk : 2 * (k : ℤ) + 2 ≤ 2 * T.lenAlong ℓ τ) {nx : Step Patch} {M : Motion GaussianInt GaussianInt}
    (hlast : ∃ cs, T.vertexCrossings ℓ 4 (T.fpt ℓ τ (2 * k + 1) (2 * T.lenAcross ℓ τ - 1), inward τ) nx =
      some cs ∧ Motion.composeAll cs = M) :
    ∃ cs, T.chain ℓ 4 (T.acrossSteps ℓ τ k ++ [nx]) = some cs ∧ Motion.composeAll cs = M := by
  rw [acrossSteps_eq]
  obtain ⟨n, hn⟩ : ∃ n, 2 * T.lenAcross ℓ τ = n + 1 := ⟨2 * T.lenAcross ℓ τ - 1, by omega⟩
  rw [hn]
  obtain ⟨cs, h, e⟩ := T.chain_range (ℓ := ℓ) (fuel := 4)
    (fun j : ℕ => (T.fpt ℓ τ (2 * k + 1) (j : ℤ), inward τ)) (fun _ => Motion.id) nx M n
    (fun i hi => ⟨[Motion.id], by
      have := T.vc_across ℓ (τ := τ) (s := 2 * k + 1) (j := i) (by omega) (by omega) (by omega)
        (by omega)
      simpa using this, by simp⟩)
    (by
      obtain ⟨cs, h, e⟩ := hlast
      refine ⟨cs, ?_, by simpa using e⟩
      rw [show ((n : ℕ) : ℤ) = 2 * T.lenAcross ℓ τ - 1 by omega]
      exact h)
  exact ⟨cs, h, by simpa using e⟩

/-- **A step along a side, the region outside the rectangle**: its face on the left lies
across the side, in the frame of the arc holding the edge. -/
theorem normalize_below [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {σ : Side Patch} {b : α}
    (hb : b ∈ T.arcs σ) {y ε : ℤ} (hε : ε = -σ.orient)
    (h₁ : 2 * (posNat b (T.arcs σ) ℓ : ℤ) ≤ y) (h₂ : y ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b)
    (h₃ : 2 * (posNat b (T.arcs σ) ℓ : ℤ) ≤ y + ε)
    (h₄ : y + ε ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b) :
    T.normalize ℓ (T.fpt ℓ σ y 0, dir σ ε 0) =
      some (T.fpt ℓ (T.other b σ) (T.sign b * y + 2 * T.crossTrans ℓ σ b) 0,
        dir (T.other b σ) (T.other b σ).orient 0) := by
  have hl := posNat_add_le ℓ hb
  rw [← sideLen, hbal] at hl
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := y) (r := 0) (by omega) (by omega) le_rfl
    (by have := hH σ; omega)
  have hsg := T.sign_eq_orient hT hb
  rw [T.normalize_cross (Q := (T.other b σ).1) (C := fineMotion (T.crossMotion ℓ σ b))]
  · simp only
    rw [f1, T.crossMotion_framePt hT ℓ hb, T.crossMotion_lin_dir hT ℓ hb, neg_zero]
    congr 3
    rw [hsg, hε]
    rcases orient_cases σ with ho | ho <;> rcases orient_cases (T.other b σ) with ho' | ho' <;>
      simp [ho, ho']
  · simp only [fpt_patch]
    intro h
    have hl := square_last _ _ _ _ h
    rw [f1, gi_mul_dir, framePt_add_dir, inRect_framePt, hε] at hl
    rcases orient_cases σ with ho | ho <;> simp [ho] at hl
  · simp only [fpt_patch]
    rw [f1, pointAt_framePt_add_dir, add_zero]
    rcases orient_cases σ with ho | ho <;> rw [ho] at hε <;> subst hε
    · have := (T.crossEdge_framePt ℓ (hH σ) hb (x := y + -1) (by omega) (by omega) (by omega)).2
      simpa using this
    · exact (T.crossEdge_framePt ℓ (hH σ) hb (x := y) (by omega) (by omega) (by omega)).1

/-- **Arrival through `σ`, the closing segment outside the rectangle**: the chart is
already in the frame of the face on the left of the first step along `σ`. -/
theorem vc_arrive_out [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ.opp) {s : ℤ} (hodd : s % 2 = 1)
    (hs₁ : 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 1 ≤ s)
    (hs₂ : s + 1 ≤ 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 2 * ℓ a) {ε : ℤ}
    (hε : ε = -(T.other a τ.opp).orient) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ τ s (2 * T.lenAcross ℓ τ - 1), inward τ)
        (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
          dir (T.other a τ.opp) ε 0) = some [] := by
  have hl := posNat_add_le ℓ ha
  rw [← sideLen, hbal] at hl
  have hlo : T.lenAlong ℓ τ.opp = T.lenAlong ℓ τ := rfl
  have ha' := T.mem_arcs_other ha
  have hr₁ := T.img_range (τ := τ.opp) (a := a) ℓ (x := s - 1) (by omega) (by omega)
  have hr₂ := T.img_range (τ := τ.opp) (a := a) ℓ (x := s + 1) (by omega) (by omega)
  have hn := T.normalize_below hT ℓ hH hbal ha' hε
    (y := T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) (by
      rcases T.sign_cases a with h | h <;> rw [h] at hr₁ hr₂ ⊢ <;> omega) (by
      rcases T.sign_cases a with h | h <;> rw [h] at hr₁ hr₂ ⊢ <;> omega) (by
      rcases T.sign_cases a with h | h <;> rcases orient_cases (T.other a τ.opp) with ho | ho <;>
        rw [ho] at hε <;> subst hε <;> rw [h] at hr₁ hr₂ ⊢ <;> omega) (by
      rcases T.sign_cases a with h | h <;> rcases orient_cases (T.other a τ.opp) with ho | ho <;>
        rw [ho] at hε <;> subst hε <;> rw [h] at hr₁ hr₂ ⊢ <;> omega)
  rw [T.other_other ha, T.img_img ha] at hn
  rw [T.vc_last ℓ (hH τ) (by omega) (by omega) hn, turnAt_stop]

/-- **Arrival through `σ`, the closing segment inside the rectangle**: the chart crosses
the arc into the frame of `σ`, then one interior edge (a turn to the right). -/
theorem vc_arrive_in [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ.opp) {s : ℤ} (hodd : s % 2 = 1)
    (hs₁ : 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 1 ≤ s)
    (hs₂ : s + 1 ≤ 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 2 * ℓ a) {ε : ℤ}
    (hε : ε = (T.other a τ.opp).orient) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ τ s (2 * T.lenAcross ℓ τ - 1), inward τ)
        (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
          dir (T.other a τ.opp) ε 0) = some [fineMotion (T.crossMotion ℓ τ.opp a), Motion.id] := by
  have hl := posNat_add_le ℓ ha
  have hl' := posNat_add_le ℓ (T.mem_arcs_other ha)
  rw [← sideLen, hbal] at hl hl'
  have hlo : T.lenAlong ℓ τ.opp = T.lenAlong ℓ τ := rfl
  have hr₁ := T.img_range (τ := τ.opp) (a := a) ℓ (x := s - 1) (by omega) (by omega)
  have hr₂ := T.img_range (τ := τ.opp) (a := a) ℓ (x := s + 1) (by omega) (by omega)
  have hs' : 1 ≤ T.sign a * s + 2 * T.crossTrans ℓ τ.opp a ∧
      T.sign a * s + 2 * T.crossTrans ℓ τ.opp a + 1 ≤ 2 * T.lenAlong ℓ (T.other a τ.opp) := by
    rcases T.sign_cases a with h | h <;> rw [h] at hr₁ hr₂ ⊢ <;> constructor <;> omega
  have hH' := hH (T.other a τ.opp)
  have f2 := T.fine_fpt_of (ℓ := ℓ) (τ := T.other a τ.opp)
    (s := T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) (r := 0) (by omega) (by omega) le_rfl (by omega)
  have n2 : T.normalize ℓ (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
      dir (T.other a τ.opp) ε 0) =
      some (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
        dir (T.other a τ.opp) ε 0) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f2, square_gi, hε]
    rect_by (T.other a τ.opp)
  have hin : T.aroundStep ℓ (-gi)
      (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0) (inward (T.other a τ.opp)) =
      some (T.fpt ℓ (T.other a τ.opp) (T.sign a * s + 2 * T.crossTrans ℓ τ.opp a) 0,
        dir (T.other a τ.opp) ε 0, Motion.id) := by
    rw [T.aroundStep_in]
    · rw [← dir_zero_one, neg_gi_mul_dir, hε]; simp
    · simp only [fpt_patch]
      rw [f2, ← dir_zero_one, square_neg_gi]
      rect_by (T.other a τ.opp)
  rw [T.vc_last ℓ (hH τ) (by omega) (by omega) n2, T.turnAt_step _ (T.exit_step hT ℓ hH hbal ha hs₁ hs₂),
    T.turnAt_step _ hin, turnAt_stop]
  · rfl
  · rintro ⟨-, h⟩
    rw [← dir_zero_one] at h
    have := (dir_inj h).2
    omega
  · rintro ⟨h, -⟩
    exact T.fpt_ne hH (T.other_ne ha).symm hodd (by omega) (by omega) (by omega) (by omega) h

/-- Along `σ`, the region inside the rectangle: one interior edge at each vertex. -/
theorem vc_along_in (ℓ : α → ℕ) (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) {σ : Side Patch}
    {y ε : ℤ} (hε : ε = σ.orient) (h₁ : 0 ≤ y) (h₂ : y ≤ 2 * T.lenAlong ℓ σ) (h₃ : 0 ≤ y + ε)
    (h₄ : y + ε ≤ 2 * T.lenAlong ℓ σ) (h₅ : 0 ≤ y + ε + ε) (h₆ : y + ε + ε ≤ 2 * T.lenAlong ℓ σ) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ σ y 0, dir σ ε 0) (T.fpt ℓ σ (y + ε) 0, dir σ ε 0) =
      some [Motion.id] := by
  have hH' := hH σ
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := y) (r := 0) h₁ h₂ le_rfl (by omega)
  have f2 := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := y + ε) (r := 0) h₃ h₄ le_rfl (by omega)
  have n1 : T.normalize ℓ (T.fpt ℓ σ y 0, dir σ ε 0) = some (T.fpt ℓ σ y 0, dir σ ε 0) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f1, square_gi, hε]
    rect_by σ
  have n2 : T.normalize ℓ (T.fpt ℓ σ (y + ε) 0, dir σ ε 0) = some (T.fpt ℓ σ (y + ε) 0, dir σ ε 0) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f2, square_gi, hε]
    rect_by σ
  have hin : T.aroundStep ℓ (-gi) (T.fpt ℓ σ (y + ε) 0) (dir σ 0 1) =
      some (T.fpt ℓ σ (y + ε) 0, dir σ ε 0, Motion.id) := by
    rw [T.aroundStep_in]
    · rw [neg_gi_mul_dir, hε]; simp
    · simp only [fpt_patch]
      rw [f2, square_neg_gi, hε]
      rect_by σ
  rw [T.vertexCrossings_eq n1 n2, fpt_patch, f1, pointAt_framePt_add_dir, gi_mul_dir, add_zero,
    mul_zero, neg_zero, hε, Side.orient_mul_self, ← hε, T.turnAt_step _ hin, turnAt_stop]
  · rfl
  · rintro ⟨-, h⟩
    have := (dir_inj h).1
    rcases orient_cases σ with ho | ho <;> rw [ho] at hε <;> omega

/-- At the start of the round, the region inside the rectangle: the turn onto the first
step is a turn to the left, no crossing. -/
theorem vc_close_in (ℓ : α → ℕ) (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) {σ : Side Patch}
    {x ε : ℤ} (hε : ε = σ.orient) (h₁ : 1 ≤ x) (h₂ : x + 1 ≤ 2 * T.lenAlong ℓ σ) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ σ (x - ε) 0, dir σ ε 0) (T.fpt ℓ σ x 0, inward σ) = some [] := by
  have hH' := hH σ
  have hε' : ε = 1 ∨ ε = -1 := by rw [hε]; exact orient_cases σ
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := x - ε) (r := 0) (by omega) (by omega) le_rfl
    (by omega)
  have f2 := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := x) (r := 0) (by omega) (by omega) le_rfl (by omega)
  have n1 : T.normalize ℓ (T.fpt ℓ σ (x - ε) 0, dir σ ε 0) = some (T.fpt ℓ σ (x - ε) 0, dir σ ε 0) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f1, square_gi, hε]
    rect_by σ
  have n2 : T.normalize ℓ (T.fpt ℓ σ x 0, inward σ) = some (T.fpt ℓ σ x 0, inward σ) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f2, ← dir_zero_one, square_gi]
    rect_by σ
  rw [T.vertexCrossings_eq n1 n2, fpt_patch, f1, pointAt_framePt_add_dir, gi_mul_dir, sub_add_cancel,
    add_zero, mul_zero, neg_zero, hε, Side.orient_mul_self, dir_zero_one, turnAt_stop]

/-- Across an arc of `σ`, the positions of the arc on `σ` land in the rectangle across. -/
theorem img_in [Fintype α] (ℓ : α → ℕ) (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ)
    {σ : Side Patch} {b : α} (hb : b ∈ T.arcs σ) {x : ℤ} (h₁ : 2 * (posNat b (T.arcs σ) ℓ : ℤ) ≤ x)
    (h₂ : x ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b) :
    0 ≤ T.sign b * x + 2 * T.crossTrans ℓ σ b ∧
      T.sign b * x + 2 * T.crossTrans ℓ σ b ≤ 2 * T.lenAlong ℓ (T.other b σ) := by
  have hl' := posNat_add_le ℓ (T.mem_arcs_other hb)
  rw [← sideLen, hbal] at hl'
  have := T.img_range (τ := σ) (a := b) ℓ h₁ h₂
  omega

/-- Along `σ`, the region outside the rectangle, at a vertex at an odd position: the
chart is in the frame of the patch across, where the vertex is inside an arc, and
crosses one interior edge there. -/
theorem vc_along_out_odd [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {σ : Side Patch} {b : α}
    (hb : b ∈ T.arcs σ) {y ε : ℤ} (hε : ε = -σ.orient)
    (h₁ : 2 * (posNat b (T.arcs σ) ℓ : ℤ) ≤ y) (h₂ : y ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b)
    (h₃ : 2 * (posNat b (T.arcs σ) ℓ : ℤ) ≤ y + ε)
    (h₄ : y + ε ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b)
    (h₅ : 2 * (posNat b (T.arcs σ) ℓ : ℤ) ≤ y + ε + ε)
    (h₆ : y + ε + ε ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ σ y 0, dir σ ε 0) (T.fpt ℓ σ (y + ε) 0, dir σ ε 0) =
      some [Motion.id] := by
  have n1 := T.normalize_below hT ℓ hH hbal hb hε h₁ h₂ h₃ h₄
  have n2 := T.normalize_below hT ℓ hH hbal hb hε h₃ h₄ h₅ h₆
  have hsg := T.sign_eq_orient hT hb
  have i1 := T.img_in ℓ hbal hb h₁ h₂
  have i2 := T.img_in ℓ hbal hb h₃ h₄
  have i3 := T.img_in ℓ hbal hb h₅ h₆
  have hH' := hH (T.other b σ)
  have hI : ∀ x, T.sign b * (x + ε) + 2 * T.crossTrans ℓ σ b =
      T.sign b * x + 2 * T.crossTrans ℓ σ b + (T.other b σ).orient := by
    intro x
    rw [hsg, hε]
    rcases orient_cases σ with ho | ho <;> rcases orient_cases (T.other b σ) with ho' | ho' <;>
      simp [ho, ho'] <;> ring
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := T.other b σ) (s := T.sign b * y + 2 * T.crossTrans ℓ σ b)
    (r := 0) i1.1 i1.2 le_rfl (by omega)
  have f2 := T.fine_fpt_of (ℓ := ℓ) (τ := T.other b σ)
    (s := T.sign b * (y + ε) + 2 * T.crossTrans ℓ σ b) (r := 0) i2.1 i2.2 le_rfl (by omega)
  rw [hI] at f2 n2
  have hin : T.aroundStep ℓ (-gi)
      (T.fpt ℓ (T.other b σ) (T.sign b * y + 2 * T.crossTrans ℓ σ b + (T.other b σ).orient) 0)
      (dir (T.other b σ) 0 1) =
      some (T.fpt ℓ (T.other b σ) (T.sign b * y + 2 * T.crossTrans ℓ σ b + (T.other b σ).orient) 0,
        dir (T.other b σ) (T.other b σ).orient 0, Motion.id) := by
    rw [T.aroundStep_in]
    · rw [neg_gi_mul_dir]; simp
    · simp only [fpt_patch]
      rw [f2, square_neg_gi]
      have := hI (y + ε)
      rw [hI y] at this
      simp only [List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true, inRect_framePt]
      rcases orient_cases (T.other b σ) with ho | ho <;> simp only [ho] at this i3 ⊢ <;> omega
  rw [T.vertexCrossings_eq n1 n2, fpt_patch, f1, pointAt_framePt_add_dir, gi_mul_dir, add_zero,
    mul_zero, neg_zero, Side.orient_mul_self, T.turnAt_step _ hin, turnAt_stop]
  · rfl
  · rintro ⟨-, h⟩
    have := (dir_inj h).1
    rcases orient_cases (T.other b σ) with ho | ho <;> rw [ho] at this <;> omega

/-- At the start of the round, the region outside the rectangle: from the frame of the
patch across `σ`, the chart crosses one interior edge there, then the arc back into the
frame of `σ`. -/
theorem vc_close_out [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {σ : Side Patch} {b : α}
    (hb : b ∈ T.arcs σ) {x ε : ℤ} (hε : ε = -σ.orient) (hodd : x % 2 = 1)
    (h₁ : 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 1 ≤ x) (h₂ : x + 1 ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b) :
    T.vertexCrossings ℓ 4 (T.fpt ℓ σ (x - ε) 0, dir σ ε 0) (T.fpt ℓ σ x 0, inward σ) =
      some [Motion.id, fineMotion (T.crossMotion ℓ (T.other b σ) b)] := by
  have hε' : ε = 1 ∨ ε = -1 := by
    rcases orient_cases σ with ho | ho <;> rw [ho] at hε <;> omega
  have hl := posNat_add_le ℓ hb
  rw [← sideLen, hbal] at hl
  have hH' := hH (T.other b σ)
  have hHσ := hH σ
  have n1 := T.normalize_below hT ℓ hH hbal hb hε (y := x - ε) (by omega) (by omega) (by omega)
    (by omega)
  have f0 := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := x) (r := 0) (by omega) (by omega) le_rfl
    (by omega)
  have n2 : T.normalize ℓ (T.fpt ℓ σ x 0, inward σ) = some (T.fpt ℓ σ x 0, inward σ) := by
    apply normalize_self
    simp only [fpt_patch]
    rw [f0, ← dir_zero_one, square_gi]
    rcases orient_cases σ with ho | ho <;> rw [ho] at hε <;>
      simp only [List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true, inRect_framePt, ho] <;>
      omega
  have hsg := T.sign_eq_orient hT hb
  have hI : ∀ y, T.sign b * (y + ε) + 2 * T.crossTrans ℓ σ b =
      T.sign b * y + 2 * T.crossTrans ℓ σ b + (T.other b σ).orient := by
    intro y
    rw [hsg, hε]
    rcases orient_cases σ with ho | ho <;> rcases orient_cases (T.other b σ) with ho' | ho' <;>
      simp [ho, ho'] <;> ring
  have i0 := T.img_in ℓ hbal hb (x := x - ε) (by omega) (by omega)
  have i1 := T.img_in ℓ hbal hb (x := x) (by omega) (by omega)
  have i2 := T.img_in ℓ hbal hb (x := x + ε) (by omega) (by omega)
  have hr₁ := T.img_range (τ := σ) (a := b) ℓ (x := x - 1) (by omega) (by omega)
  have hr₂ := T.img_range (τ := σ) (a := b) ℓ (x := x + 1) (by omega) (by omega)
  have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := T.other b σ)
    (s := T.sign b * (x - ε) + 2 * T.crossTrans ℓ σ b) (r := 0) i0.1 i0.2 le_rfl (by omega)
  have hx : T.sign b * (x - ε) + 2 * T.crossTrans ℓ σ b + (T.other b σ).orient =
      T.sign b * x + 2 * T.crossTrans ℓ σ b := by
    have := hI (x - ε); rw [sub_add_cancel] at this; exact this.symm
  have f2 := T.fine_fpt_of (ℓ := ℓ) (τ := T.other b σ) (s := T.sign b * x + 2 * T.crossTrans ℓ σ b)
    (r := 0) i1.1 i1.2 le_rfl (by omega)
  have hin : T.aroundStep ℓ (-gi) (T.fpt ℓ (T.other b σ) (T.sign b * x + 2 * T.crossTrans ℓ σ b) 0)
      (dir (T.other b σ) 0 1) =
      some (T.fpt ℓ (T.other b σ) (T.sign b * x + 2 * T.crossTrans ℓ σ b) 0,
        dir (T.other b σ) (T.other b σ).orient 0, Motion.id) := by
    rw [T.aroundStep_in]
    · rw [neg_gi_mul_dir]; simp
    · simp only [fpt_patch]
      rw [f2, square_neg_gi]
      have := hI x
      simp only [List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true, inRect_framePt]
      rcases orient_cases (T.other b σ) with ho | ho <;> simp only [ho] at this i2 ⊢ <;> omega
  have hb' := T.mem_arcs_other hb
  have hodd' : (T.sign b * x + 2 * T.crossTrans ℓ σ b) % 2 = 1 := by
    rcases T.sign_cases b with h | h <;> rw [h] <;> omega
  have hex := T.exit_step hT ℓ hH hbal hb' (s := T.sign b * x + 2 * T.crossTrans ℓ σ b) (by
    rcases T.sign_cases b with h | h <;> rw [h] at hr₁ hr₂ ⊢ <;> omega) (by
    rcases T.sign_cases b with h | h <;> rw [h] at hr₁ hr₂ ⊢ <;> omega)
  rw [T.other_other hb, T.img_img hb] at hex
  have hne : ∀ d d' : GaussianInt,
      ¬ (T.fpt ℓ (T.other b σ) (T.sign b * x + 2 * T.crossTrans ℓ σ b) 0 = T.fpt ℓ σ x 0 ∧ d = d') := by
    rintro d d' ⟨h, -⟩
    exact T.fpt_ne hH (T.other_ne hb) hodd' i1.1 i1.2 (by omega) (by omega) h
  rw [T.vertexCrossings_eq n1 n2, fpt_patch, f1, pointAt_framePt_add_dir, gi_mul_dir, add_zero,
    mul_zero, neg_zero, Side.orient_mul_self, hx, T.turnAt_step (hne _ _) hin,
    T.turnAt_step (hne _ _) hex, turnAt_stop]
  rfl

/-! ### At a node of the closing segment, the region outside the rectangle -/

theorem crossEdge_lin_quarter {ℓ : α → ℕ} {p q : Point Patch} {Q : Patch}
    {C : Motion GaussianInt GaussianInt} (h : T.crossEdge ℓ p q = some (Q, C)) : IsQuarter C.lin := by
  unfold crossEdge at h
  cases hs : T.onSide ℓ p q with
  | none =>
    rw [hs] at h
    simp only [Option.some.injEq, Prod.mk.injEq] at h
    rw [← h.2]; exact IsQuarter.one
  | some τt =>
    rw [hs] at h
    obtain ⟨a, -, he⟩ := Option.map_eq_some_iff.mp h
    simp only [Prod.mk.injEq] at he
    rw [← he.2, fineMotion_lin, crossMotion_lin]
    exact crossLin_isQuarter _ _

theorem aroundStep_quarter {ℓ : α → ℕ} {ρ : GaussianInt} (hρ : IsQuarter ρ) {v : Point Patch}
    {d : GaussianInt} (hd : IsQuarter d) {v' : Point Patch} {d' : GaussianInt}
    {C : Motion GaussianInt GaussianInt} (h : T.aroundStep ℓ ρ v d = some (v', d', C)) : IsQuarter d' := by
  unfold aroundStep at h
  dsimp only at h
  split_ifs at h with h1 h2
  · simp only [Option.some.injEq, Prod.mk.injEq] at h
    rw [← h.2.1]; exact hρ.mul hd
  · obtain ⟨⟨Q, C'⟩, hc, h'⟩ := Option.bind_eq_some_iff.mp h
    simp only at h'
    split_ifs at h' with h3
    simp only [Option.some.injEq, Prod.mk.injEq] at h'
    rw [← h'.2.1]
    exact hρ.mul ((T.crossEdge_lin_quarter hc).mul hd)

theorem around_some {ℓ : α → ℕ} {ρ : GaussianInt} {f : ℕ} {v v₀ : Point Patch} {d d₀ : GaussianInt}
    {L : List (Motion GaussianInt GaussianInt)} (h : T.around ℓ ρ (f + 1) v d v₀ d₀ = some L) :
    ∃ v' d' C, T.aroundStep ℓ ρ v d = some (v', d', C) ∧
      ((v' = v₀ ∧ d' = d₀) ∧ L = [C] ∨
        ¬(v' = v₀ ∧ d' = d₀) ∧ ∃ L', T.around ℓ ρ f v' d' v₀ d₀ = some L' ∧ L = C :: L') := by
  unfold around at h
  obtain ⟨⟨v', d', C⟩, hs, h'⟩ := Option.bind_eq_some_iff.mp h
  refine ⟨v', d', C, hs, ?_⟩
  simp only at h'
  split_ifs at h' with heq
  · exact Or.inl ⟨heq, (Option.some.inj h').symm⟩
  · obtain ⟨L', hL', rfl⟩ := Option.map_eq_some_iff.mp h'
    exact Or.inr ⟨heq, L', hL', rfl⟩

theorem around_fuel {ℓ : α → ℕ} {ρ : GaussianInt} {f : ℕ} {v v₀ : Point Patch} {d d₀ : GaussianInt}
    {L : List (Motion GaussianInt GaussianInt)} (h : T.around ℓ ρ f v d v₀ d₀ = some L) :
    ∃ f', f = f' + 1 := by
  cases f with
  | zero => simp [around] at h
  | succ f' => exact ⟨f', rfl⟩

/-- **A walk round a vertex met by four faces**, step by step. -/
theorem around_four {ℓ : α → ℕ} {ρ : GaussianInt} {f : ℕ} {v : Point Patch} {d : GaussianInt}
    {L : List (Motion GaussianInt GaussianInt)} (h : T.around ℓ ρ f v d v d = some L)
    (hl : L.length = 4) :
    ∃ v₁ d₁ M₁ v₂ d₂ M₂ v₃ d₃ M₃ M₄, T.aroundStep ℓ ρ v d = some (v₁, d₁, M₁) ∧
      T.aroundStep ℓ ρ v₁ d₁ = some (v₂, d₂, M₂) ∧ T.aroundStep ℓ ρ v₂ d₂ = some (v₃, d₃, M₃) ∧
      ¬(v₃ = v ∧ d₃ = d) ∧ T.aroundStep ℓ ρ v₃ d₃ = some (v, d, M₄) ∧ L = [M₁, M₂, M₃, M₄] := by
  obtain ⟨f, rfl⟩ := T.around_fuel h
  obtain ⟨v₁, d₁, M₁, s1, h1⟩ := T.around_some h
  rcases h1 with ⟨-, rfl⟩ | ⟨-, L₁, h1, rfl⟩
  · simp at hl
  obtain ⟨f, rfl⟩ := T.around_fuel h1
  obtain ⟨v₂, d₂, M₂, s2, h2⟩ := T.around_some h1
  rcases h2 with ⟨-, rfl⟩ | ⟨-, L₂, h2, rfl⟩
  · simp at hl
  obtain ⟨f, rfl⟩ := T.around_fuel h2
  obtain ⟨v₃, d₃, M₃, s3, h3⟩ := T.around_some h2
  rcases h3 with ⟨-, rfl⟩ | ⟨n3, L₃, h3, rfl⟩
  · simp at hl
  obtain ⟨f, rfl⟩ := T.around_fuel h3
  obtain ⟨v₄, d₄, M₄, s4, h4⟩ := T.around_some h3
  rcases h4 with ⟨⟨rfl, rfl⟩, rfl⟩ | ⟨-, L₄, h4, rfl⟩
  · exact ⟨v₁, d₁, M₁, v₂, d₂, M₂, v₃, d₃, M₃, M₄, s1, s2, s3, n3, s4, rfl⟩
  · have := T.around_ne_nil ℓ ρ _ _ _ _ _ _ h4
    simp only [List.length_cons] at hl
    exact absurd (List.length_eq_zero_iff.mp (by omega)) this

/-- **At a node of `σ` met by four faces, the region outside the rectangle**: from the
frame of the arc holding the edge before the node to the frame of the arc holding the
edge after it, the chart crosses the edge below the node. The composite carries the
first frame to the second: the four crossings round the node compose to the identity
(`holonomyAround_eq`), and two of them are these frames. -/
theorem vc_node [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    {σ : Side Patch} {m : ℤ}
    (hm₁ : 0 < m) (hm₂ : m < T.lenAlong ℓ σ) {ε : ℤ} (hε : ε = -σ.orient) {fuel' : ℕ}
    (hval : T.valenceAround ℓ fuel' (T.fpt ℓ σ (2 * m) 0) (inward σ) = some 4)
    {b₁ b₂ : α} (hb₁ : b₁ ∈ T.arcs σ) (hb₂ : b₂ ∈ T.arcs σ)
    (h₁ : 2 * (posNat b₁ (T.arcs σ) ℓ : ℤ) ≤ 2 * m - ε)
    (h₂ : 2 * m - ε ≤ 2 * (posNat b₁ (T.arcs σ) ℓ : ℤ) + 2 * ℓ b₁)
    (h₃ : 2 * (posNat b₁ (T.arcs σ) ℓ : ℤ) ≤ 2 * m)
    (h₄ : 2 * m ≤ 2 * (posNat b₁ (T.arcs σ) ℓ : ℤ) + 2 * ℓ b₁)
    (h₅ : 2 * (posNat b₂ (T.arcs σ) ℓ : ℤ) ≤ 2 * m)
    (h₆ : 2 * m ≤ 2 * (posNat b₂ (T.arcs σ) ℓ : ℤ) + 2 * ℓ b₂)
    (h₇ : 2 * (posNat b₂ (T.arcs σ) ℓ : ℤ) ≤ 2 * m + ε)
    (h₈ : 2 * m + ε ≤ 2 * (posNat b₂ (T.arcs σ) ℓ : ℤ) + 2 * ℓ b₂) :
    ∃ cs, T.vertexCrossings ℓ 4 (T.fpt ℓ σ (2 * m - ε) 0, dir σ ε 0) (T.fpt ℓ σ (2 * m) 0, dir σ ε 0) =
      some cs ∧ Motion.comp (Motion.composeAll cs) (fineMotion (T.crossMotion ℓ σ b₁)) =
        fineMotion (T.crossMotion ℓ σ b₂) := by
  set C₁ := fineMotion (T.crossMotion ℓ σ b₁)
  set C₂ := fineMotion (T.crossMotion ℓ σ b₂)
  set v := T.fpt ℓ σ (2 * m) 0
  have hε' : ε = 1 ∨ ε = -1 := by
    rcases orient_cases σ with ho | ho <;> rw [ho] at hε <;> omega
  have hHσ := hH σ
  have fz := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := 2 * m) (r := 0) (by omega) (by omega) le_rfl
    (by omega)
  -- The walk round the node, and its holonomy.
  obtain ⟨L, hL, hlen⟩ : ∃ L, T.around ℓ gi fuel' v (inward σ) v (inward σ) = some L ∧ L.length = 4 := by
    unfold valenceAround aroundVertex at hval
    obtain ⟨L, hL, hl⟩ := Option.map_eq_some_iff.mp hval
    exact ⟨L, hL, hl⟩
  have hhol : Motion.composeAll L = Motion.id := by
    have := T.holonomyAround_eq (isQuarter_inward σ) (H := Motion.composeAll L)
      (by unfold holonomyAround aroundVertex; rw [hL]; rfl) hval
    rw [this]
    ext <;> simp [quarter]
  obtain ⟨v₁, d₁, M₁, v₂, d₂, M₂, v₃, d₃, M₃, M₄, s1, s2, s3, n3, s4, rfl⟩ := T.around_four hL hlen
  -- Step 1: inside the rectangle.
  have e1 : T.aroundStep ℓ gi v (inward σ) = some (v, dir σ ε 0, Motion.id) := by
    rw [T.aroundStep_in]
    · rw [← dir_zero_one, gi_mul_dir, hε]; simp
    · simp only [v, fpt_patch]
      rw [fz, ← dir_zero_one, square_gi]
      rect_by σ
  rw [e1] at s1
  simp only [Option.some.injEq, Prod.mk.injEq] at s1
  obtain ⟨rfl, rfl, rfl⟩ := s1
  -- Step 2: across the edge after the node, by `C₂`.
  have hc₂ : T.crossEdge ℓ v (pointAt v.patch (fine v + dir σ ε 0)) = some ((T.other b₂ σ).1, C₂) := by
    simp only [v, fpt_patch]
    rw [fz, pointAt_framePt_add_dir, add_zero]
    rcases hε' with rfl | rfl
    · exact (T.crossEdge_framePt ℓ hHσ hb₂ (x := 2 * m) (by omega) (by omega) (by omega)).1
    · have := (T.crossEdge_framePt ℓ hHσ hb₂ (x := 2 * m + -1) (by omega) (by omega) (by omega)).2
      simpa using this
  have g2₁ : ¬ (square (fine v) (dir σ ε 0) gi).all (T.inRect ℓ v.patch) = true := by
    intro h
    have hl := square_last _ _ _ _ h
    simp only [v, fpt_patch] at hl
    rw [fz, gi_mul_dir, framePt_add_dir, inRect_framePt, hε] at hl
    rcases orient_cases σ with ho | ho <;> simp [ho] at hl
  have g2₂ : T.inRect ℓ v.patch (fine v + dir σ ε 0) = true := by
    simp only [v, fpt_patch]
    rw [fz, framePt_add_dir, inRect_framePt]
    omega
  rw [T.aroundStep_cross g2₁ g2₂ hc₂] at s2
  split_ifs at s2 with g2₃
  have s2' : T.aroundStep ℓ gi v (dir σ ε 0) =
      some (pointAt (T.other b₂ σ).1 (C₂.apply (fine v)), gi * (C₂.lin * dir σ ε 0), C₂) := by
    rw [T.aroundStep_cross g2₁ g2₂ hc₂, ite_eq_left g2₃]
  simp only [Option.some.injEq, Prod.mk.injEq] at s2
  obtain ⟨rfl, rfl, rfl⟩ := s2
  -- The directions stay quarter-turns.
  have hq₂ : IsQuarter (gi * (C₂.lin * dir σ ε 0)) :=
    T.aroundStep_quarter (Or.inr (Or.inl rfl))
      (T.aroundStep_quarter (Or.inr (Or.inl rfl)) (isQuarter_inward σ) e1) s2'
  have hq₃ := T.aroundStep_quarter (Or.inr (Or.inl rfl)) hq₂ s3
  have hgi : gi * gi = -1 := gi_mul_gi
  -- Step 4 walked back: the crossing of the edge before the node, by `C₁`.
  obtain ⟨Y, r4, hY⟩ := T.aroundStep_reverse hT ℓ hH (Or.inl rfl) hq₃
    (T.aroundStep_right_face hgi s3) s4
  have ha4 := (T.aroundStep_apply s4).2
  have hY4 : M₄.lin * d₃ = dir σ (-ε) 0 := by
    have : M₄.lin * d₃ = -gi * inward σ := by linear_combination gi * ha4 + M₄.lin * d₃ * hgi
    rw [this, ← dir_zero_one, neg_gi_mul_dir, hε]; simp
  rw [hY4] at r4
  have hc₁ : T.crossEdge ℓ v (pointAt v.patch (fine v + dir σ (-ε) 0)) =
      some ((T.other b₁ σ).1, C₁) := by
    simp only [v, fpt_patch]
    rw [fz, pointAt_framePt_add_dir, add_zero]
    rcases hε' with rfl | rfl
    · have := (T.crossEdge_framePt ℓ hHσ hb₁ (x := 2 * m + -1) (by omega) (by omega) (by omega)).2
      simpa using this
    · exact (T.crossEdge_framePt ℓ hHσ hb₁ (x := 2 * m) (by omega) (by omega) (by omega)).1
  have g4₁ : ¬ (square (fine v) (dir σ (-ε) 0) (-gi)).all (T.inRect ℓ v.patch) = true := by
    intro h
    have hl := square_last _ _ _ _ h
    simp only [v, fpt_patch] at hl
    rw [fz, neg_gi_mul_dir, framePt_add_dir, inRect_framePt, hε] at hl
    rcases orient_cases σ with ho | ho <;> simp [ho] at hl
  have g4₂ : T.inRect ℓ v.patch (fine v + dir σ (-ε) 0) = true := by
    simp only [v, fpt_patch]
    rw [fz, framePt_add_dir, inRect_framePt]
    omega
  rw [T.aroundStep_cross g4₁ g4₂ hc₁] at r4
  split_ifs at r4 with g4₃
  simp only [Option.some.injEq, Prod.mk.injEq] at r4
  obtain ⟨hv₃, hd₃, rfl⟩ := r4
  -- Step 3 walked back: the crossing below the node.
  obtain ⟨X, r3, hX⟩ := T.aroundStep_reverse hT ℓ hH (Or.inl rfl) hq₂
    (T.aroundStep_right_face hgi s2') s3
  have ha3 := (T.aroundStep_apply s3).2
  -- The two steps of the border at the node, read in the frames of `C₁` and `C₂`.
  have hn1 : T.normalize ℓ (T.fpt ℓ σ (2 * m - ε) 0, dir σ ε 0) =
      some (pointAt (T.other b₁ σ).1 (C₁.apply (T.framePt ℓ σ (2 * m - ε) 0)), C₁.lin * dir σ ε 0) := by
    have f1 := T.fine_fpt_of (ℓ := ℓ) (τ := σ) (s := 2 * m - ε) (r := 0) (by omega) (by omega) le_rfl
      (by omega)
    rw [T.normalize_cross (Q := (T.other b₁ σ).1) (C := C₁)]
    · simp only; rw [f1]
    · simp only [fpt_patch]
      intro h
      have hl := square_last _ _ _ _ h
      rw [f1, gi_mul_dir, framePt_add_dir, inRect_framePt, hε] at hl
      rcases orient_cases σ with ho | ho <;> simp [ho] at hl
    · simp only [fpt_patch]
      rw [f1, pointAt_framePt_add_dir, add_zero, sub_add_cancel]
      rcases hε' with rfl | rfl
      · have := (T.crossEdge_framePt ℓ hHσ hb₁ (x := 2 * m - 1) (by omega) (by omega) (by omega)).1
        simpa using this
      · have := (T.crossEdge_framePt ℓ hHσ hb₁ (x := 2 * m) (by omega) (by omega) (by omega)).2
        simpa using this
  have hn2 : T.normalize ℓ (T.fpt ℓ σ (2 * m) 0, dir σ ε 0) =
      some (pointAt (T.other b₂ σ).1 (C₂.apply (fine v)), C₂.lin * dir σ ε 0) := by
    exact T.normalize_cross (s := (v, dir σ ε 0)) g2₁ hc₂
  -- The frame of the first: its point is the image of `2m - ε`, inside the rectangle across.
  have hC₁ : C₁.apply (T.framePt ℓ σ (2 * m - ε) 0) = C₁.apply (fine v) + C₁.lin * dir σ (-ε) 0 := by
    have e : T.framePt ℓ σ (2 * m - ε) 0 = fine v + dir σ (-ε) 0 := by
      rw [fz, framePt_add_dir]; exact T.framePt_congr ℓ σ (by ring) (by ring)
    rw [e, apply_eq, apply_eq]; ring
  have hfin : fine (pointAt (T.other b₁ σ).1 (C₁.apply (T.framePt ℓ σ (2 * m - ε) 0))) =
      C₁.apply (T.framePt ℓ σ (2 * m - ε) 0) := by
    rw [hC₁]; exact T.fine_pointAt_of_inRect (square_second _ _ _ _ g4₃)
  have hstart : pointAt (pointAt (T.other b₁ σ).1 (C₁.apply (T.framePt ℓ σ (2 * m - ε) 0))).patch
      (fine (pointAt (T.other b₁ σ).1 (C₁.apply (T.framePt ℓ σ (2 * m - ε) 0))) + C₁.lin * dir σ ε 0) =
      v₃ := by
    rw [pointAt_patch, hfin, hC₁, ← hv₃]
    congr 1
    have : dir σ (-ε) 0 = -dir σ ε 0 := by simp [dir]
    rw [this]; ring
  have hdir : gi * (C₁.lin * dir σ ε 0) = M₃.lin * (gi * (C₂.lin * dir σ ε 0)) := by
    have : dir σ (-ε) 0 = -dir σ ε 0 := by simp [dir]
    rw [this] at hd₃
    linear_combination hd₃ - gi * ha3 - (M₃.lin * (gi * (C₂.lin * dir σ ε 0))) * hgi
  refine ⟨[X], ?_, ?_⟩
  · rw [T.vertexCrossings_eq hn1 hn2, hstart, hdir, T.turnAt_step _ r3]
    · rw [show -gi * (gi * (C₂.lin * dir σ ε 0)) = C₂.lin * dir σ ε 0 by
        linear_combination (-(C₂.lin * dir σ ε 0)) * hgi, turnAt_stop]
      rfl
    · rintro ⟨h, hd⟩
      apply n3
      have e3 : d₃ = gi * (C₂.lin * dir σ ε 0) := by rw [ha3, mul_assoc, hd]
      rw [h, e3] at s4
      rw [s3] at s4
      simp only [Option.some.injEq, Prod.mk.injEq] at s4
      exact ⟨s4.1, s4.2.1⟩
  · have hhol' : Motion.comp (Motion.comp M₄ M₃) C₂ = Motion.id := by
      simpa [Motion.comp_assoc] using hhol
    calc Motion.comp (Motion.composeAll [X]) C₁
        = Motion.comp (Motion.comp X C₁) (Motion.comp (Motion.comp M₄ M₃) C₂) := by
          rw [hhol', Motion.comp_id]; simp
      _ = Motion.comp X (Motion.comp (Motion.comp C₁ M₄) (Motion.comp M₃ C₂)) := by
          simp only [Motion.comp_assoc]
      _ = C₂ := by rw [hY, Motion.id_comp, ← Motion.comp_assoc, hX, Motion.id_comp]

/-! ### Along the closing segment -/

theorem arcOn_exists (ℓ : α → ℕ) : ∀ (l : List α) (t : ℕ), t < 2 * (l.map ℓ).sum →
    ∃ a, arcOn ℓ l t = some a
  | [], t, h => by simp at h
  | b :: l, t, h => by
    simp only [arcOn]
    split_ifs with hb
    · exact ⟨b, rfl⟩
    · simp only [List.map_cons, List.sum_cons] at h
      exact arcOn_exists ℓ l (t - 2 * ℓ b) (by omega)

/-- The arc of `σ` holding the fine edge `[x, x + 1]`, at a position inside the side. -/
theorem arc_of_edge (ℓ : α → ℕ) (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ)
    (σ : Side Patch) {x : ℤ} (h₁ : 0 ≤ x) (h₂ : x + 1 ≤ 2 * T.lenAlong ℓ σ) :
    ∃ b, arcOn ℓ (T.arcs σ) x.toNat = some b ∧ b ∈ T.arcs σ ∧
      2 * (posNat b (T.arcs σ) ℓ : ℤ) ≤ x ∧ x + 1 ≤ 2 * (posNat b (T.arcs σ) ℓ : ℤ) + 2 * ℓ b := by
  obtain ⟨b, hb⟩ := arcOn_exists ℓ (T.arcs σ) x.toNat (by
    have := hbal σ; rw [sideLen] at this; omega)
  obtain ⟨hm, r₁, r₂⟩ := (arcOn_eq_some ℓ (T.arcs_nodup σ) _ b).mp hb
  exact ⟨b, hb, hm, by omega, by omega⟩

/-- **The crossings along the closing segment, the region inside the rectangle**: the
identity. -/
theorem chain_along_in (ℓ : α → ℕ) (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ) {σ : Side Patch}
    {p ε : ℤ} {N : ℕ} (hε : ε = σ.orient) (hN : 0 < N) (hp₁ : 1 ≤ p) (hp₂ : p + 1 ≤ 2 * T.lenAlong ℓ σ)
    (hx₁ : 1 ≤ p + ε * N) (hx₂ : p + ε * N + 1 ≤ 2 * T.lenAlong ℓ σ) :
    ∃ cs, T.chain ℓ 4 ((List.range N).map (fun i : ℕ => (T.fpt ℓ σ (p + ε * i) 0, dir σ ε 0)) ++
      [(T.fpt ℓ σ (p + ε * N) 0, inward σ)]) = some cs ∧ Motion.composeAll cs = Motion.id := by
  have hε' : ε = 1 ∨ ε = -1 := by rw [hε]; exact orient_cases σ
  obtain ⟨n, rfl⟩ : ∃ n, N = n + 1 := ⟨N - 1, by omega⟩
  obtain ⟨cs, h, e⟩ := T.chain_range (ℓ := ℓ) (fuel := 4)
    (fun i : ℕ => (T.fpt ℓ σ (p + ε * i) 0, dir σ ε 0)) (fun _ => Motion.id)
    (T.fpt ℓ σ (p + ε * ((n + 1 : ℕ) : ℤ)) 0, inward σ) Motion.id n
    (fun i hi => ⟨[Motion.id], by
      have e1 : p + ε * ((i + 1 : ℕ) : ℤ) = p + ε * i + ε := by push_cast; ring
      rw [e1]
      exact T.vc_along_in ℓ hH hε (by rcases hε' with rfl | rfl <;> omega)
        (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
        (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
        (by rcases hε' with rfl | rfl <;> omega), by simp⟩)
    ⟨[], by
      have e1 : p + ε * ((n + 1 : ℕ) : ℤ) = p + ε * n + ε := by push_cast; ring
      rw [e1]
      have := T.vc_close_in ℓ hH hε (x := p + ε * n + ε) (by rcases hε' with rfl | rfl <;> omega)
        (by rcases hε' with rfl | rfl <;> omega)
      simpa using this, by simp⟩
  exact ⟨cs, h, by simpa using e⟩

/-- **The crossings along the closing segment, the region outside the rectangle**: every
node of `σ` on the way is met by four faces. The composite carries the frame of the arc the
walk came back through to the frame of `σ`. -/
theorem chain_along_out [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {σ : Side Patch} {fuel' : ℕ}
    (hσ : ∀ m : ℤ, 0 < m → m < T.lenAlong ℓ σ →
      T.valenceAround ℓ fuel' (T.fpt ℓ σ (2 * m) 0) (inward σ) = some 4)
    {p ε : ℤ} {N : ℕ} (hε : ε = -σ.orient) (hN : 0 < N) (hp : p % 2 = 1) (hp₁ : 1 ≤ p)
    (hp₂ : p + 1 ≤ 2 * T.lenAlong ℓ σ) (hx : (p + ε * N) % 2 = 1) (hx₁ : 1 ≤ p + ε * N)
    (hx₂ : p + ε * N + 1 ≤ 2 * T.lenAlong ℓ σ) {a : α} (ha : a ∈ T.arcs σ)
    (ha₁ : 2 * (posNat a (T.arcs σ) ℓ : ℤ) + 1 ≤ p) (ha₂ : p + 1 ≤ 2 * (posNat a (T.arcs σ) ℓ : ℤ) + 2 * ℓ a) :
    ∃ cs, T.chain ℓ 4 ((List.range N).map (fun i : ℕ => (T.fpt ℓ σ (p + ε * i) 0, dir σ ε 0)) ++
      [(T.fpt ℓ σ (p + ε * N) 0, inward σ)]) = some cs ∧
      Motion.comp (Motion.composeAll cs) (fineMotion (T.crossMotion ℓ σ a)) = Motion.id := by
  have hε' : ε = 1 ∨ ε = -1 := by
    rcases orient_cases σ with ho | ho <;> rw [ho] at hε <;> omega
  -- The arc under the edge of each step, and its frame.
  let bArc : ℕ → α := fun i => (arcOn ℓ (T.arcs σ) (min (p + ε * i) (p + ε * i + ε)).toNat).getD a
  have hbArc : ∀ i : ℕ, i < N → bArc i ∈ T.arcs σ ∧
      2 * (posNat (bArc i) (T.arcs σ) ℓ : ℤ) ≤ min (p + ε * i) (p + ε * i + ε) ∧
      min (p + ε * i) (p + ε * i + ε) + 1 ≤ 2 * (posNat (bArc i) (T.arcs σ) ℓ : ℤ) + 2 * ℓ (bArc i) := by
    intro i hi
    obtain ⟨b, hb, hm, r₁, r₂⟩ := T.arc_of_edge ℓ hbal σ (x := min (p + ε * i) (p + ε * i + ε))
      (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
    have : bArc i = b := by simp only [bArc, hb, Option.getD_some]
    rw [this]
    exact ⟨hm, r₁, r₂⟩
  have hb0 : bArc 0 = a := by
    have : arcOn ℓ (T.arcs σ) (min (p + ε * ((0 : ℕ) : ℤ)) (p + ε * ((0 : ℕ) : ℤ) + ε)).toNat = some a :=
      (arcOn_eq_some ℓ (T.arcs_nodup σ) _ a).mpr ⟨ha, by rcases hε' with rfl | rfl <;> omega,
        by rcases hε' with rfl | rfl <;> omega⟩
    simp only [bArc, this, Option.getD_some]
  obtain ⟨n, rfl⟩ : ∃ n, N = n + 1 := ⟨N - 1, by omega⟩
  obtain ⟨cs, h, e⟩ := T.chain_range (ℓ := ℓ) (fuel := 4)
    (fun i : ℕ => (T.fpt ℓ σ (p + ε * i) 0, dir σ ε 0))
    (fun i => fineMotion (T.crossMotion ℓ σ (bArc i)))
    (T.fpt ℓ σ (p + ε * ((n + 1 : ℕ) : ℤ)) 0, inward σ) Motion.id n
    (fun i hi => by
      obtain ⟨hm, r₁, r₂⟩ := hbArc i (by omega)
      obtain ⟨hm', r₁', r₂'⟩ := hbArc (i + 1) (by omega)
      have e1 : p + ε * ((i + 1 : ℕ) : ℤ) = p + ε * i + ε := by push_cast; ring
      rw [e1]
      rw [e1] at r₁' r₂'
      by_cases hodd : (p + ε * i + ε) % 2 = 1
      · -- The vertex is inside an arc: the frames agree.
        have hsame : bArc (i + 1) = bArc i := by
          have : arcOn ℓ (T.arcs σ) (min (p + ε * ((i + 1 : ℕ) : ℤ)) (p + ε * ((i + 1 : ℕ) : ℤ) + ε)).toNat =
              some (bArc i) := by
            rw [e1]
            exact (arcOn_eq_some ℓ (T.arcs_nodup σ) _ _).mpr ⟨hm,
              by rcases hε' with rfl | rfl <;> omega, by rcases hε' with rfl | rfl <;> omega⟩
          simp only [bArc] at this ⊢
          rw [this, Option.getD_some]
        refine ⟨[Motion.id], ?_, by rw [hsame]; simp⟩
        exact T.vc_along_out_odd hT ℓ hH hbal hm hε (by rcases hε' with rfl | rfl <;> omega)
          (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
          (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
          (by rcases hε' with rfl | rfl <;> omega)
      · -- The vertex is a node of `σ`, met by four faces.
        obtain ⟨m, hm2⟩ : ∃ m, p + ε * i + ε = 2 * m := ⟨(p + ε * i + ε) / 2, by omega⟩
        have hv := hσ m (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
        have := T.vc_node hT ℓ hH (σ := σ) (m := m) (by rcases hε' with rfl | rfl <;> omega)
          (by rcases hε' with rfl | rfl <;> omega) hε hv hm hm'
          (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
          (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
          (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
          (by rcases hε' with rfl | rfl <;> omega) (by rcases hε' with rfl | rfl <;> omega)
        rw [show p + ε * i = 2 * m - ε by omega, show 2 * m - ε + ε = 2 * m by ring]
        exact this)
    (by
      obtain ⟨hm, r₁, r₂⟩ := hbArc n (by omega)
      have e1 : p + ε * ((n + 1 : ℕ) : ℤ) = p + ε * n + ε := by push_cast; ring
      rw [e1]
      have := T.vc_close_out hT ℓ hH hbal hm hε (x := p + ε * n + ε)
        (by rcases hε' with rfl | rfl <;> push_cast at hx <;> omega)
        (by rcases hε' with rfl | rfl <;> push_cast at hx <;> omega)
        (by rcases hε' with rfl | rfl <;> push_cast at hx <;> omega)
      rw [add_sub_cancel_right] at this
      refine ⟨_, this, ?_⟩
      simp only [Motion.composeAll_cons, Motion.composeAll_nil, Motion.id_comp, Motion.comp_id]
      exact T.crossMotion_inv hT ℓ hm)
  refine ⟨cs, h, ?_⟩
  simpa [hb0] using e

/-! ### The holonomy of the border of a round -/

/-- **The end of a round with a closing segment**: across the last rectangle, through `σ`
at the fine position `p`, then along `σ` by `ε` back to the start `p + ε N`. -/
theorem chain_close_seg [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ.opp) {k' : ℕ} (hs₁ : 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 1 ≤ 2 * k' + 1)
    (hs₂ : 2 * (k' : ℤ) + 1 + 1 ≤ 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 2 * ℓ a) {fuel' : ℕ}
    (hσ : ∀ m : ℤ, 0 < m → m < T.lenAlong ℓ (T.other a τ.opp) →
      T.valenceAround ℓ fuel' (T.fpt ℓ (T.other a τ.opp) (2 * m) 0) (inward (T.other a τ.opp)) = some 4)
    {p : ℤ} (hp : p = T.sign a * (2 * k' + 1) + 2 * T.crossTrans ℓ τ.opp a)
    {ε : ℤ} (hε : ε = 1 ∨ ε = -1) {N : ℕ} (hN : 0 < N) (hx₁ : 1 ≤ p + ε * N)
    (hx₂ : p + ε * N + 1 ≤ 2 * T.lenAlong ℓ (T.other a τ.opp)) (hxodd : (p + ε * N) % 2 = 1) :
    ∃ cs, T.chain ℓ 4 (T.acrossSteps ℓ τ k' ++
      ((List.range N).map (fun i : ℕ => (T.fpt ℓ (T.other a τ.opp) (p + ε * i) 0,
        dir (T.other a τ.opp) ε 0)) ++
      [(T.fpt ℓ (T.other a τ.opp) (p + ε * N) 0, inward (T.other a τ.opp))])) = some cs ∧
      Motion.composeAll cs = fineMotion (T.crossMotion ℓ τ.opp a) := by
  have hlo : T.lenAlong ℓ τ.opp = T.lenAlong ℓ τ := rfl
  have hl := posNat_add_le ℓ ha
  rw [← sideLen, hbal] at hl
  have hk2 : 2 * (k' : ℤ) + 2 ≤ 2 * T.lenAlong ℓ τ := by omega
  have ha' := T.mem_arcs_other ha
  have hr₁ := T.img_range (τ := τ.opp) (a := a) ℓ (x := 2 * k' + 1 - 1) (by omega) (by omega)
  have hr₂ := T.img_range (τ := τ.opp) (a := a) ℓ (x := 2 * k' + 1 + 1) (by omega) (by omega)
  have hpa : 2 * (posNat a (T.arcs (T.other a τ.opp)) ℓ : ℤ) + 1 ≤ p ∧
      p + 1 ≤ 2 * (posNat a (T.arcs (T.other a τ.opp)) ℓ : ℤ) + 2 * ℓ a := by
    subst hp; rcases T.sign_cases a with h | h <;> rw [h] at hr₁ hr₂ ⊢ <;> constructor <;> omega
  have hl' := posNat_add_le ℓ ha'
  rw [← sideLen, hbal] at hl'
  have hpodd : p % 2 = 1 := by subst hp; rcases T.sign_cases a with h | h <;> rw [h] <;> omega
  have hodd : (2 * (k' : ℤ) + 1) % 2 = 1 := by omega
  obtain ⟨n, rfl⟩ : ∃ n, N = n + 1 := ⟨N - 1, by omega⟩
  have hD : Motion.comp (fineMotion (T.crossMotion ℓ (T.other a τ.opp) a))
      (fineMotion (T.crossMotion ℓ τ.opp a)) = Motion.id := T.crossMotion_inv hT ℓ ha
  have hD' : Motion.comp (fineMotion (T.crossMotion ℓ τ.opp a))
      (fineMotion (T.crossMotion ℓ (T.other a τ.opp) a)) = Motion.id := by
    have := T.crossMotion_inv hT ℓ ha'
    rwa [T.other_other ha] at this
  rw [List.range_succ_eq_map, List.map_cons, List.cons_append]
  have hg0 : (T.fpt ℓ (T.other a τ.opp) (p + ε * ((0 : ℕ) : ℤ)) 0, dir (T.other a τ.opp) ε 0) =
      (T.fpt ℓ (T.other a τ.opp) (T.sign a * (2 * k' + 1) + 2 * T.crossTrans ℓ τ.opp a) 0,
        dir (T.other a τ.opp) ε 0) := by
    rw [hp]; simp
  by_cases hin : ε = (T.other a τ.opp).orient
  · -- The closing segment runs with the region inside the rectangle.
    obtain ⟨c₁, h₁, e₁⟩ := T.chain_across ℓ (hH τ) hk2 (nx := (T.fpt ℓ (T.other a τ.opp)
      (p + ε * ((0 : ℕ) : ℤ)) 0, dir (T.other a τ.opp) ε 0))
      ⟨_, by rw [hg0]; exact T.vc_arrive_in hT ℓ hH hbal ha hodd hs₁ hs₂ hin, rfl⟩
    obtain ⟨c₂, h₂, e₂⟩ := T.chain_along_in ℓ hH (σ := T.other a τ.opp) (p := p) (N := n + 1) hin
      (by omega) (by omega) (by omega) hx₁ hx₂
    rw [List.range_succ_eq_map, List.map_cons, List.cons_append] at h₂
    refine ⟨c₁ ++ c₂, T.chain_append_of _ _ _ c₁ c₂ h₁ h₂, ?_⟩
    rw [Motion.composeAll_append, e₂, Motion.id_comp, e₁]
    simp
  · -- The closing segment runs with the region outside the rectangle.
    have hout : ε = -(T.other a τ.opp).orient := by
      rcases orient_cases (T.other a τ.opp) with ho | ho <;> rw [ho] at hin ⊢ <;> omega
    obtain ⟨c₁, h₁, e₁⟩ := T.chain_across ℓ (hH τ) hk2 (nx := (T.fpt ℓ (T.other a τ.opp)
      (p + ε * ((0 : ℕ) : ℤ)) 0, dir (T.other a τ.opp) ε 0))
      ⟨_, by rw [hg0]; exact T.vc_arrive_out hT ℓ hH hbal ha hodd hs₁ hs₂ hout, rfl⟩
    obtain ⟨c₂, h₂, e₂⟩ := T.chain_along_out hT ℓ hH hbal hσ (p := p) (N := n + 1) hout (by omega)
      hpodd (by omega) (by omega) hxodd hx₁ hx₂ ha' hpa.1 hpa.2
    rw [List.range_succ_eq_map, List.map_cons, List.cons_append] at h₂
    refine ⟨c₁ ++ c₂, T.chain_append_of _ _ _ c₁ c₂ h₁ h₂, ?_⟩
    rw [Motion.composeAll_append, e₁, Motion.composeAll_nil, Motion.comp_id]
    calc Motion.composeAll c₂
        = Motion.comp (Motion.composeAll c₂) (Motion.comp (fineMotion (T.crossMotion ℓ (T.other a τ.opp) a))
            (fineMotion (T.crossMotion ℓ τ.opp a))) := by rw [hD, Motion.comp_id]
      _ = fineMotion (T.crossMotion ℓ τ.opp a) := by rw [← Motion.comp_assoc, e₂, Motion.id_comp]

/-- **The end of a round**: across the last rectangle, through `σ` at the station `kq + ½`,
then along `σ` back to the start station `k + ½` (nothing when `kq = k`). The crossings
compose to the change of chart across the last arc. -/
theorem chain_close [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {τ : Side Patch} {a : α}
    (ha : a ∈ T.arcs τ.opp) {k' : ℕ} (hs₁ : 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 1 ≤ 2 * k' + 1)
    (hs₂ : 2 * (k' : ℤ) + 1 + 1 ≤ 2 * (posNat a (T.arcs τ.opp) ℓ : ℤ) + 2 * ℓ a) {kq : ℕ}
    (hkq : (2 * kq + 1 : ℤ) = T.sign a * (2 * k' + 1) + 2 * T.crossTrans ℓ τ.opp a) {k : ℕ}
    (hk : k < T.lenAlong ℓ (T.other a τ.opp)) {fuel' : ℕ}
    (hσ : ∀ m : ℤ, 0 < m → m < T.lenAlong ℓ (T.other a τ.opp) →
      T.valenceAround ℓ fuel' (T.fpt ℓ (T.other a τ.opp) (2 * m) 0) (inward (T.other a τ.opp)) = some 4) :
    ∃ cs, T.chain ℓ 4 (T.acrossSteps ℓ τ k' ++ T.alongSteps ℓ (T.other a τ.opp) (2 * kq + 1) (2 * k + 1) ++
      [(T.fpt ℓ (T.other a τ.opp) (2 * k + 1) 0, inward (T.other a τ.opp))]) = some cs ∧
      Motion.composeAll cs = fineMotion (T.crossMotion ℓ τ.opp a) := by
  have hlo : T.lenAlong ℓ τ.opp = T.lenAlong ℓ τ := rfl
  have hl := posNat_add_le ℓ ha
  rw [← sideLen, hbal] at hl
  have hl' := posNat_add_le ℓ (T.mem_arcs_other ha)
  rw [← sideLen, hbal] at hl'
  have hr := T.img_range (τ := τ.opp) (a := a) ℓ (x := 2 * k' + 1) (by omega) (by omega)
  have hkq' : kq < T.lenAlong ℓ (T.other a τ.opp) := by
    have hr₁ := T.img_range (τ := τ.opp) (a := a) ℓ (x := 2 * k' + 1 + 1) (by omega) (by omega)
    have hr₀ := T.img_range (τ := τ.opp) (a := a) ℓ (x := 2 * k' + 1 - 1) (by omega) (by omega)
    rcases T.sign_cases a with h | h <;> rw [h] at hr₀ hr₁ hkq <;> omega
  have hk2 : 2 * (k' : ℤ) + 2 ≤ 2 * T.lenAlong ℓ τ := by omega
  have hodd : (2 * (k' : ℤ) + 1) % 2 = 1 := by omega
  have hσ' := hσ
  rcases lt_trichotomy kq k with hlt | rfl | hgt
  · rw [T.alongSteps_eq_le ℓ _ (by omega), List.append_assoc]
    obtain ⟨cs, h, e⟩ := T.chain_close_seg hT ℓ hH hbal ha hs₁ hs₂ hσ'
      (p := ((2 * kq + 1 : ℕ) : ℤ)) (by push_cast; exact hkq) (ε := 1) (Or.inl rfl)
      (N := 2 * k + 1 - (2 * kq + 1)) (by omega) (by push_cast; omega) (by push_cast; omega)
      (by push_cast; omega)
    rw [show ((2 * kq + 1 : ℕ) : ℤ) + 1 * ((2 * k + 1 - (2 * kq + 1) : ℕ) : ℤ) = 2 * k + 1 by
      push_cast; omega] at h
    exact ⟨cs, h, e⟩
  · rw [T.alongSteps_eq_le ℓ _ le_rfl, Nat.sub_self, List.range_zero, List.map_nil, List.append_nil]
    refine T.chain_across ℓ (hH τ) hk2 ⟨[fineMotion (T.crossMotion ℓ τ.opp a)], ?_, by simp⟩
    have := T.vc_transition hT ℓ hH hbal ha hodd hs₁ hs₂
    rwa [← hkq] at this
  · rw [T.alongSteps_eq_gt ℓ _ (by omega), List.append_assoc]
    obtain ⟨cs, h, e⟩ := T.chain_close_seg hT ℓ hH hbal ha hs₁ hs₂ hσ'
      (p := ((2 * kq + 1 : ℕ) : ℤ)) (by push_cast; exact hkq) (ε := -1) (Or.inr rfl)
      (N := 2 * kq + 1 - (2 * k + 1)) (by omega) (by push_cast; omega) (by push_cast; omega)
      (by push_cast; omega)
    rw [show ((2 * kq + 1 : ℕ) : ℤ) + -1 * ((2 * kq + 1 - (2 * k + 1) : ℕ) : ℤ) = 2 * k + 1 by
      push_cast; omega] at h
    exact ⟨cs, h, e⟩

theorem crossMotions_append (ℓ : α → ℕ) :
    ∀ (σ : Side Patch) (l : List α) (a : α),
      T.crossMotions ℓ σ (l ++ [a]) = T.crossMotions ℓ σ l ++ [T.crossMotion ℓ (T.endSide σ l).opp a]
  | _, [], _ => rfl
  | σ, b :: l, a => by
    simp only [List.cons_append, crossMotions, endSide, crossMotions_append ℓ _ l a]

theorem endSide_append : ∀ (σ : Side Patch) (l : List α) (a : α),
    T.endSide σ (l ++ [a]) = T.other a (T.endSide σ l).opp
  | _, [], _ => rfl
  | σ, b :: l, a => by simp only [List.cons_append, endSide, endSide_append _ l a]

@[simp] theorem fineMotion_id : fineMotion (Motion.id : Motion GaussianInt GaussianInt) = Motion.id := by
  ext <;> simp [fineMotion]

theorem acrossSteps_cons (ℓ : α → ℕ) {τ : Side Patch} (hH : 0 < T.lenAcross ℓ τ) (k : ℕ) :
    ∃ tl, T.acrossSteps ℓ τ k = (T.fpt ℓ τ (2 * k + 1) 0, inward τ) :: tl := by
  rw [acrossSteps_eq]
  obtain ⟨n, hn⟩ : ∃ n, 2 * T.lenAcross ℓ τ = n + 1 := ⟨2 * T.lenAcross ℓ τ - 1, by omega⟩
  rw [hn, List.range_succ_eq_map, List.map_cons]
  exact ⟨_, by rw [Nat.cast_zero]⟩

/-- The composite along the path so far, one more arc crossed. -/
theorem fineMotion_crossMotions_append (ℓ : α → ℕ) (σ : Side Patch) (l : List α) (a : α) :
    fineMotion (Motion.composeAll (T.crossMotions ℓ σ (l ++ [a]))) =
      Motion.comp (fineMotion (T.crossMotion ℓ (T.endSide σ l).opp a))
        (fineMotion (Motion.composeAll (T.crossMotions ℓ σ l))) := by
  rw [crossMotions_append, Motion.composeAll_append, fineMotion_comp]
  simp

/-- **The induction along the walk**: the crossings of the steps so far compose to the
changes of chart of the arcs crossed so far. -/
theorem curveSteps_holonomy [Fintype α] (hT : T.Oriented) (ℓ : α → ℕ)
    (hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ) {σ : Side Patch} {k : ℕ}
    (hk : k < T.lenAlong ℓ σ) {fuel' : ℕ}
    (hσ : ∀ m : ℤ, 0 < m → m < T.lenAlong ℓ σ →
      T.valenceAround ℓ fuel' (T.fpt ℓ σ (2 * m) 0) (inward σ) = some 4) :
    ∀ (fuel : ℕ) (acc : List (Step Patch)) (arcs : List α) (τ : Side Patch) (k' : ℕ)
      (steps : List (Step Patch)) (path : List α),
      T.curveSteps ℓ σ k fuel acc arcs (τ, (k' : ℚ) + 1 / 2) = some (steps, path) →
      τ = T.endSide σ arcs.reverse →
      (∃ rest, acc ++ [(T.fpt ℓ τ (2 * k' + 1) 0, inward τ)] =
        (T.fpt ℓ σ (2 * k + 1) 0, inward σ) :: rest) →
      (∃ cs, T.chain ℓ 4 (acc ++ [(T.fpt ℓ τ (2 * k' + 1) 0, inward τ)]) = some cs ∧
        Motion.composeAll cs = fineMotion (Motion.composeAll (T.crossMotions ℓ σ arcs.reverse))) →
      T.borderHolonomy ℓ 4 steps = some (fineMotion (Motion.composeAll (T.crossMotions ℓ σ path)))
  | 0, _, _, _, _, _, _, h, _, _, _ => by simp [curveSteps] at h
  | fuel + 1, acc, arcs, τ, k', steps, path, h, hτ, ⟨rest, hrest⟩, ⟨c₁, hc₁, e₁⟩ => by
    unfold curveSteps at h
    simp only [station_floor] at h
    cases harc : arcAt (rat ℓ) (T.arcs τ.opp) ((k' : ℚ) + 1 / 2) with
    | none => rw [harc] at h; simp at h
    | some a =>
      rw [harc] at h
      obtain ⟨ha, hs₁, hs₂, k₂, hk₂, hk₂'⟩ := T.station_step ℓ harc
      simp only [hk₂, station_floor] at h
      obtain ⟨tl, htl⟩ := T.acrossSteps_cons ℓ (hH τ) k'
      have hpath : T.crossMotions ℓ σ (a :: arcs).reverse =
          T.crossMotions ℓ σ (arcs.reverse ++ [a]) := by rw [List.reverse_cons]
      split_ifs at h with hq
      · -- Back through `σ`: the end of the round.
        simp only [Option.some.injEq, Prod.mk.injEq] at h
        obtain ⟨rfl, rfl⟩ := h
        obtain ⟨c₂, h₂, e₂⟩ := T.chain_close hT ℓ hH hbal ha hs₁ hs₂ hk₂' (k := k) (by rw [hq]; exact hk)
          (fuel' := fuel') (by rw [hq]; exact hσ)
        rw [hq, htl] at h₂
        have hsteps : acc ++ ((T.fpt ℓ τ (2 * k' + 1) 0, inward τ) :: tl) ++
            T.alongSteps ℓ σ (2 * k₂ + 1) (2 * k + 1) =
            (T.fpt ℓ σ (2 * k + 1) 0, inward σ) :: (rest ++ tl ++ T.alongSteps ℓ σ (2 * k₂ + 1) (2 * k + 1)) := by
          rw [show acc ++ ((T.fpt ℓ τ (2 * k' + 1) 0, inward τ) :: tl) =
            acc ++ [(T.fpt ℓ τ (2 * k' + 1) 0, inward τ)] ++ tl by simp, hrest]
          simp
        rw [htl, hsteps, borderHolonomy_eq_chain]
        have hlist : (T.fpt ℓ σ (2 * k + 1) 0, inward σ) ::
            (rest ++ tl ++ T.alongSteps ℓ σ (2 * k₂ + 1) (2 * k + 1)) ++ [(T.fpt ℓ σ (2 * k + 1) 0, inward σ)] =
            acc ++ (T.fpt ℓ τ (2 * k' + 1) 0, inward τ) ::
              (tl ++ T.alongSteps ℓ σ (2 * k₂ + 1) (2 * k + 1) ++ [(T.fpt ℓ σ (2 * k + 1) 0, inward σ)]) := by
          rw [show acc ++ (T.fpt ℓ τ (2 * k' + 1) 0, inward τ) ::
              (tl ++ T.alongSteps ℓ σ (2 * k₂ + 1) (2 * k + 1) ++ [(T.fpt ℓ σ (2 * k + 1) 0, inward σ)]) =
            (acc ++ [(T.fpt ℓ τ (2 * k' + 1) 0, inward τ)]) ++
              (tl ++ T.alongSteps ℓ σ (2 * k₂ + 1) (2 * k + 1) ++ [(T.fpt ℓ σ (2 * k + 1) 0, inward σ)]) by
            simp, hrest]
          simp
        rw [hlist, T.chain_append_of _ _ _ c₁ c₂ hc₁ (by simpa using h₂), Option.map_some,
          Motion.composeAll_append, e₁, e₂, hpath, fineMotion_crossMotions_append, ← hτ]
      · -- Across the arc into the next rectangle.
        apply curveSteps_holonomy hT ℓ hH hbal hk hσ fuel (acc ++ T.acrossSteps ℓ τ k') (a :: arcs) (T.other a τ.opp) k₂
          steps path h
        · rw [List.reverse_cons, endSide_append, ← hτ]
        · refine ⟨rest ++ tl ++ [(T.fpt ℓ (T.other a τ.opp) (2 * k₂ + 1) 0, inward (T.other a τ.opp))], ?_⟩
          rw [htl, show acc ++ ((T.fpt ℓ τ (2 * k' + 1) 0, inward τ) :: tl) =
            acc ++ [(T.fpt ℓ τ (2 * k' + 1) 0, inward τ)] ++ tl by simp, hrest]
          simp
        · have h₂ := T.chain_across ℓ (hH τ) (k := k') (by
            have hl := posNat_add_le ℓ ha
            rw [← sideLen, hbal] at hl
            have hlo : T.lenAlong ℓ τ.opp = T.lenAlong ℓ τ := rfl
            omega) (nx := (T.fpt ℓ (T.other a τ.opp) (2 * k₂ + 1) 0, inward (T.other a τ.opp)))
            ⟨_, by rw [hk₂']; exact T.vc_transition hT ℓ hH hbal ha (by omega) hs₁ hs₂, rfl⟩
          obtain ⟨c₂, h₂, e₂⟩ := h₂
          rw [htl] at h₂
          refine ⟨c₁ ++ c₂, ?_, ?_⟩
          · rw [htl, List.append_assoc, List.cons_append]
            exact T.chain_append_of _ _ _ c₁ c₂ hc₁ h₂
          · rw [Motion.composeAll_append, e₁, e₂, hpath, fineMotion_crossMotions_append, ← hτ]
            simp

/-- **The holonomy of the border of a round is the development of its turn.** On an
oriented T-mesh at whole lengths `ℓ`, every side of positive length and the lengths
balanced, the chart continued along the closed curve of the round from the station `k + ½`
of `σ`, through the region on its left, is the composite of the changes of chart of the arcs
the round crosses, in half quads: `fineMotion (Turn.development γ ℓ)` for the turn `γ` of
the round (`Turn.development` is this composite by definition). Assumed: every node strictly
inside `σ` is met by four faces (needed only when the closing segment along `σ` runs with
the region outside the rectangle of `σ`). -/
theorem borderHolonomy_round [Fintype α] (hT : T.Oriented) {ℓ : α → ℕ}
    (hpos : ∀ τ : Side Patch, 0 < T.sideLen ℓ τ)
    (hbal : ∀ τ : Side Patch, T.sideLen ℓ τ.opp = T.sideLen ℓ τ) {σ : Side Patch} {k fuel' : ℕ}
    (hσ : ∀ m : ℕ, 0 < m → m < T.sideLen ℓ σ →
      T.valenceAround ℓ fuel' (T.sidePoint ℓ σ (2 * m)) (inward σ) = some 4)
    (hk : k < T.sideLen ℓ σ) {patches : List Patch} {steps : List (Step Patch)} {path : List α}
    (h : T.roundSteps patches ℓ σ k = some (steps, path)) :
    T.borderHolonomy ℓ 4 steps = some (fineMotion (Motion.composeAll (T.crossMotions ℓ σ path))) := by
  have hbal' : ∀ τ : Side Patch, T.sideLen ℓ τ = T.lenAlong ℓ τ := by
    intro τ
    obtain ⟨P, j, e⟩ := τ
    fin_cases j <;> fin_cases e <;> simp only [lenAlong, width, height, Fin.isValue, Fin.zero_eta,
      Fin.mk_one, ↓reduceIte, one_ne_zero] <;> first | exact hbal (P, 0, 0) | exact hbal (P, 1, 0)
  have hH : ∀ τ : Side Patch, 0 < T.lenAcross ℓ τ := by
    intro τ
    obtain ⟨P, j, e⟩ := τ
    fin_cases j <;> simp [lenAcross, width, height, hpos]
  have hσ' : ∀ m : ℤ, 0 < m → m < T.lenAlong ℓ σ →
      T.valenceAround ℓ fuel' (T.fpt ℓ σ (2 * m) 0) (inward σ) = some 4 := by
    intro m hm₁ hm₂
    obtain ⟨n, rfl⟩ := Int.eq_ofNat_of_zero_le hm₁.le
    rw [show 2 * (n : ℤ) = ((2 * n : ℕ) : ℤ) by push_cast; ring, fpt_nat]
    exact hσ n (by omega) (by rw [hbal']; omega)
  exact T.curveSteps_holonomy hT ℓ hH hbal' (by rw [← hbal']; exact hk) hσ' _ [] [] σ k steps path h rfl
    ⟨[], rfl⟩ ⟨[], rfl, by simp [crossMotions]⟩

end TMesh

namespace Motion

variable {A V : Type*} [Ring A] [AddCommGroup V] [Module A V]

/-- **A telescope of charts**: if the charts `M i` intertwine the motions `C i` and `J i`
(`M (i + 1) ∘ C i = J i ∘ M i`), the composite of the `C i` is carried to that of the
`J i`: `M n ∘ (C (n-1) ∘ ⋯ ∘ C 0) = (J (n-1) ∘ ⋯ ∘ J 0) ∘ M 0`. -/
theorem composeAll_telescope_comp (M C J : ℕ → Motion A V) :
    ∀ n : ℕ, (∀ i, i < n → comp (M (i + 1)) (C i) = comp (J i) (M i)) →
      comp (M n) (composeAll ((List.range n).map C)) = comp (composeAll ((List.range n).map J)) (M 0)
  | 0, _ => by simp
  | n + 1, h => by
    have ih := composeAll_telescope_comp M C J n (fun i hi => h i (by omega))
    simp only [List.range_succ, List.map_append, List.map_singleton, composeAll_append,
      composeAll_cons, composeAll_nil, id_comp]
    rw [← comp_assoc, h n (by omega), comp_assoc, ih, comp_assoc]

/-- **A telescope of charts that starts and ends at the identity**: the two composites
agree. -/
theorem composeAll_telescope (M C J : ℕ → Motion A V) (n : ℕ)
    (h : ∀ i, i < n → comp (M (i + 1)) (C i) = comp (J i) (M i)) (h₀ : M 0 = id) (hn : M n = id) :
    composeAll ((List.range n).map C) = composeAll ((List.range n).map J) := by
  have := composeAll_telescope_comp M C J n h
  rwa [h₀, hn, id_comp, comp_id] at this

end Motion

end TurnsOfATMesh
