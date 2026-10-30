/-
The seventh brick of DT-134: the jumps across the root cuts of a region are the
gluing motions of a forest of cuts, so the holonomy of the border is the
composite of the rotations round the cone points in the order of first visit.

`Region.lean` reads the holonomy of the border of a region through the charts
`M` of a dual spanning tree as the composite, in border order, of the jumps
`M g ∘ C ∘ (M f)⁻¹` across the cuts the border crosses: the cuts with an end on
the curve, the roots of the forest of cuts. Here the forest is written down
(`CutForest`, first-child next-sibling as `Forest` and `Cut`): each node is a
cut, by its crossing `c` from the face `f` before it to the face `g` after it,
clockwise round its near end; its far end `v` with the direction `d` from `v`
towards the near end, in the frame of `f` (`s = (v, d)`, so that `f` is the face
on the left of `(v, −i d)`); the number `k` of faces round `v`; and the cuts
leaving `v`, in the order the walk clockwise round `v` from the cut meets them.
The forest hangs from the region (`hangs`) when, at each node, the walk
clockwise round `v` from the edge a quarter-turn clockwise from the cut comes
back in `k` crossings, the last of them the cut crossed back from `g` to `f` by
the inverse change of chart, the cuts among the others being the roots of the
trees below, in order, and the charts agreeing across the tree edges among
them. All of it is decided by the program on each region.

Then, at a node: the composite of the jumps of the walk round `v`, through the
charts, is the rotation by `R^{k}` about the developed position `M f (v)`
(`holonomyAround_cw` conjugated by `M f`); the jumps across the tree edges are
the identity and the last jump is the inverse of `G_e`, so
`G_e = G_{e_r} ∘ ⋯ ∘ G_{e_1} ∘ ρ_v` with `ρ_v = R^{4−k}` about `M f (v)`, the
recursion of the note; unfolded over the forest, `Forest.glue` of the cone
rotations (`cones`), hence their composite in preorder, the order of first
visit (`Forest.glue_eq_preorder`). With `borderHolonomy_cuts` and
`borderHolonomy_round`: the development of the turn is that composite,
Proposition "The shift is set by the singularities" (a) at whole lengths.

Here: the walk round a vertex with faces (`aroundF`, `aroundVertexF`), the
forest (`CutForest`, `roots`, `cones`), the predicate `hangs`.

To prove (the brief of the brick):
* `aroundF_around`: `aroundF` carries the motions of `around`, and its faces
  chain from the face on the `−ρ` side of `(v, d)` back to it;
* `holonomyAround_cw`: the walk clockwise round `v` from `(v, d)` back to it, in
  `k` crossings, composes to `rotAbout (quarter k) (fine v)`;
* `rotAbout_conj`: `M ∘ rotAbout u p ∘ M⁻¹ = rotAbout u (M p)` for `M` with a
  quarter-turn linear part;
* `jump_reverse`: the jump of a crossing walked back, by the inverse change of
  chart, is `qinv` of the jump;
* `jump_node`: at a node of a forest that hangs,
  `jump M c = composeAll ((roots below).map (jump M)) ∘ rotAbout (quarter (4 − k)) (M f (v))`;
* `roots_glue`: for a forest that hangs, the composite of the jumps of its roots
  is `Forest.glue` of its cone rotations;
* `borderHolonomy_forest`: with the hypotheses of `borderHolonomy_cuts`, when
  the cut crossings of the border are the roots of a forest that hangs, the
  holonomy of the border is the composite of the cone rotations in preorder;
* `development_eq_cones`: with `borderHolonomy_round`, the development of the
  turn of a round, in half quads, is that composite.
-/
module

public import TurnsOfATMesh.Region

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

/-- Motions over types with decidable equality compare componentwise. -/
instance {A V : Type*} [DecidableEq A] [DecidableEq V] : DecidableEq (Motion A V) :=
  fun f g => decidable_of_iff (f.lin = g.lin ∧ f.trans = g.trans) (Motion.ext_iff).symm

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### The walk round a vertex, with faces -/

/-- `around` with the faces: each step of the walk `ρ`-wise round `v` from the
state `(v, d)` leaves the face on the `−ρ` side of the edge `(v, d)` and enters
the face on the `−ρ` side of the next state `(v', d')` (the face on the `ρ`
side of `(v, d)`, read in the frame entered). -/
def aroundF (ℓ : α → ℕ) (ρ : GaussianInt) :
    ℕ → Point Patch → GaussianInt → Point Patch → GaussianInt → Option (List (Crossing Patch))
  | 0, _, _, _, _ => none
  | fuel + 1, v, d, v₀, d₀ =>
    (T.aroundStep ℓ ρ v d).bind fun (v', d', C) =>
      let c : Crossing Patch := (faceBeside v d (-ρ), C, faceBeside v' d' (-ρ))
      if v' = v₀ ∧ d' = d₀ then some [c] else (aroundF ℓ ρ fuel v' d' v₀ d₀).map (c :: ·)

/-- The walk `ρ`-wise round `v` from the state `(v, d)` back to it, with faces:
clockwise for `ρ = −i`, from the face on the left of `(v, d)`. -/
def aroundVertexF (ℓ : α → ℕ) (ρ : GaussianInt) (fuel : ℕ) (v : Point Patch) (d : GaussianInt) :
    Option (List (Crossing Patch)) :=
  T.aroundF ℓ ρ fuel v d v d

/-! ### A forest of cuts -/

/-- A forest of cuts of a region, first-child next-sibling: `node c s k below
next` is a cut, by its crossing `c = (f, C, g)` from the face before it to the
face after it clockwise round its near end; its far end `v` with the direction
`d` from `v` towards the near end, in the frame of `f` (`s = (v, d)`); the
number `k` of faces round `v`; the forest `below` of the cuts leaving `v`, in
the order the walk clockwise round `v` from the cut meets them; and the next
cuts `next` at the same vertex. -/
inductive CutForest (Patch : Type*)
  | nil : CutForest Patch
  | node (c : Crossing Patch) (s : Step Patch) (k : ℕ) (below next : CutForest Patch) :
      CutForest Patch

namespace CutForest

variable {Patch : Type*}

/-- The crossings of the roots, in order. -/
def roots : CutForest Patch → List (Crossing Patch)
  | nil => []
  | node c _ _ _ next => c :: roots next

/-- The cone points of the forest, through the charts `M`: at each node the
rotation `R^{4−k}` and the far end developed in the chart of the face before
the cut, `M f (v)`, the position of its first visit. -/
def cones (M : Face Patch → Motion GaussianInt GaussianInt) :
    CutForest Patch → Forest (GaussianInt × GaussianInt)
  | nil => .nil
  | node c s k below next =>
    .node (quarter (4 - (k : ℤ)), (M c.1).apply (fine s.1)) (cones M below) (cones M next)

/-- The number of nodes. -/
def size : CutForest Patch → ℕ
  | nil => 0
  | node _ _ _ below next => size below + size next + 1

end CutForest

/-- **The forest hangs from the region**: at each node `(c, (v, d), k, below, next)`
with `c = (f, C, g)`, the face on the left of `(v, −i d)` is `f`; the walk
clockwise round `v` from `(v, −i d)` comes back in exactly `k` crossings; its
last crossing is the cut crossed back, from `g` to `f`, by a change of chart
inverse to `C`; among the others, the crossings off the tree are the roots of
`below`, in order, and across the tree edges the charts agree; and `below` and
`next` hang in turn. -/
def hangs (M : Face Patch → Motion GaussianInt GaussianInt) (ℓ : α → ℕ) (fuel : ℕ)
    (tree : Face Patch → Face Patch → Bool) : CutForest Patch → Bool
  | .nil => true
  | .node c s k below next =>
    decide (faceAt (s.1, -gi * s.2) = c.1) &&
    (match T.aroundVertexF ℓ (-gi) fuel s.1 (-gi * s.2) with
      | none => false
      | some cs =>
        decide (cs.length = k) &&
        (match cs.getLast? with
          | none => false
          | some (f', C', g') =>
            decide (f' = c.2.2 ∧ g' = c.1 ∧ Motion.comp C' c.2.1 = Motion.id)) &&
        decide (cutCrossings tree cs.dropLast = below.roots) &&
        (cs.dropLast.filter fun c' => tree c'.1 c'.2.2).all fun c' =>
          decide (Motion.comp (M c'.2.2) c'.2.1 = M c'.1)) &&
    hangs M ℓ fuel tree below && hangs M ℓ fuel tree next

/-! ### First facts -/

@[simp] theorem aroundF_zero (ℓ : α → ℕ) (ρ : GaussianInt) (v : Point Patch) (d : GaussianInt)
    (v₀ : Point Patch) (d₀ : GaussianInt) : T.aroundF ℓ ρ 0 v d v₀ d₀ = none := rfl

@[simp] theorem roots_nil : (CutForest.nil : CutForest Patch).roots = [] := rfl

@[simp] theorem roots_node (c : Crossing Patch) (s : Step Patch) (k : ℕ)
    (below next : CutForest Patch) : (CutForest.node c s k below next).roots = c :: next.roots := rfl

@[simp] theorem cones_nil (M : Face Patch → Motion GaussianInt GaussianInt) :
    (CutForest.nil : CutForest Patch).cones M = .nil := rfl

@[simp] theorem hangs_nil (M : Face Patch → Motion GaussianInt GaussianInt) (ℓ : α → ℕ) (fuel : ℕ)
    (tree : Face Patch → Face Patch → Bool) : T.hangs M ℓ fuel tree .nil = true := rfl

/-- The preorder of the cones is the order of first visit: a node before the
nodes below it, then the next cuts. -/
theorem preorder_cones (M : Face Patch → Motion GaussianInt GaussianInt) (c : Crossing Patch)
    (s : Step Patch) (k : ℕ) (below next : CutForest Patch) :
    Forest.preorder ((CutForest.node c s k below next).cones M) =
      (quarter (4 - (k : ℤ)), (M c.1).apply (fine s.1)) ::
        (Forest.preorder (below.cones M) ++ Forest.preorder (next.cones M)) := rfl

end TMesh

end TurnsOfATMesh
