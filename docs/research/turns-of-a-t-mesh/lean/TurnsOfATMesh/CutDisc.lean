/-
The geometric half of Proposition "The shift is set by the singularities":
a disc of quads cut along a forest, its border, and the holonomy of the border
as the composite of the rotations round its cone points in the order of first
visit.

The model does not describe the disc `D` topologically. It describes the cut
disc `Ω` by the walk round its border, counter-clockwise (`Ω` on the left, the
forest on the right), as a plane polyline of unit steps in `ℤ[i]`: a direction
is a power `R^n` of the quarter-turn (`quarter n`), and at a border vertex
where `c` face corners meet, the walk turns by `R^{2 - c}` (`c = 1`: a convex
corner of a square, a left turn). The forest is stored first-child
next-sibling (`Cut`), so the matching of its two copies of each tree edge is
laminar by construction: each tree edge `f` is met twice, first along `e` from
its near end `u` to its far end `v`, and, after the walk round the subtree at
`v`, along `e'` from `v` back to `u`.

* The cone rotation at `v` is `R^{4 - k}`, `k` the number of face corners at
  `v`, summed over its copies on the border (`Cut.corners`): a definition.
* The jump `G_f` of a tree edge (`Cut.jump`) is the motion taking the copy `e`
  onto the copy `e'` reversed (the start of `e` to the end of `e'`), the change
  of chart across `f` from the side met first to the side met second.
* The holonomy of the border `γ̄` of `D` (`CutDisc.holonomy`) is the composite,
  in border order, of the jumps of the roots of the forest's trees.

Proved: the walk round a vertex (`rootJumps_lin`, `rootJumps_apply`), the
recursion `G_f = G_{f_r} ∘ ⋯ ∘ G_{f_1} ∘ ρ_v` (`jump_eq`), and the holonomy as
`Forest.glue` of the cone rotations, hence their composite in preorder, with
the Burgers sum as translation (`CutDisc.holonomy_eq_glue`,
`CutDisc.holonomy_eq_preorder`, `CutDisc.holonomy_eq_burgers`). That a region of
a T-mesh is such a cut disc, and that `holonomy` is the development of the
turn, is outside this file.
-/
module

public import TurnsOfATMesh.FirstVisit
public import TurnsOfATMesh.Holonomy

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

open Motion

/-! ### Directions -/

/-- The direction `R^n`, `R` the quarter-turn. -/
def quarter (n : ℤ) : GaussianInt := ((giUnit ^ n : GaussianIntˣ) : GaussianInt)

theorem quarter_add (m n : ℤ) : quarter (m + n) = quarter m * quarter n := by
  simp [quarter, zpow_add]

theorem quarter_two : quarter 2 = -1 := by
  rw [quarter, show (2 : ℤ) = ((2 : ℕ) : ℤ) from rfl, zpow_natCast, giUnit_sq]; simp

theorem isQuarter_quarter (n : ℤ) : IsQuarter (quarter n) := IsQuarter.zpow_giUnit n

/-- Two motions with the same linear part that agree at one point are equal. -/
theorem Motion.eq_of_apply {f g : Motion GaussianInt GaussianInt} (x : GaussianInt)
    (hl : f.lin = g.lin) (ha : f.apply x = g.apply x) : f = g := by
  ext : 1
  · exact hl
  · simpa [Motion.apply, hl] using ha

/-! ### Forests joined one after the other -/

namespace Forest

/-- The trees of `f`, then those of `g`, as siblings. -/
def append {M : Type*} : Forest M → Forest M → Forest M
  | nil, g => g
  | node ρ ch sib, g => node ρ ch (append sib g)

theorem map_append {M N : Type*} (h : M → N) (f g : Forest M) :
    map h (append f g) = append (map h f) (map h g) := by
  induction f with
  | nil => rfl
  | node ρ ch sib _ ihs => simp [append, map, ihs]

theorem preorder_append {M : Type*} (f g : Forest M) :
    preorder (append f g) = preorder f ++ preorder g := by
  induction f with
  | nil => rfl
  | node ρ ch sib _ ihs => simp [append, preorder, ihs]

theorem glue_append {A V : Type*} [Ring A] [AddCommGroup V] [Module A V]
    (f g : Forest (Motion A V)) : glue (append f g) = comp (glue g) (glue f) := by
  rw [glue_eq_preorder, glue_eq_preorder, glue_eq_preorder, preorder_append, composeAll_append]

end Forest

/-! ### The cut disc seen from one vertex -/

/-- The forest at a vertex `v`, in the order of the walk: `last c` is `c` face
corners before the walk leaves `v` (back along the parent's tree edge, or along
`γ̄` at a root); `edge c below next` is `c` face corners, then a tree edge `f`
leaving `v` with the forest `below` at its far end, then the next edges at `v`. -/
inductive Cut
  | last (c : ℕ) : Cut
  | edge (c : ℕ) (below next : Cut) : Cut
  deriving DecidableEq, Repr

namespace Cut

/-- The face corners at the vertex, summed over its copies: `k`. -/
def corners : Cut → ℕ
  | last c => c
  | edge c _ next => c + corners next

/-- The walk round the vertex from the copy `p`, arrived at along the direction
`R^n`: the copy it leaves from and the direction `R^{n'}` it leaves along. -/
def walk : Cut → GaussianInt → ℤ → GaussianInt × ℤ
  | last c, p, n => (p, n + 2 - c)
  | edge c below next, p, n =>
    let w := walk below (p + quarter (n + 2 - c)) (n + 2 - c)
    walk next (w.1 + quarter w.2) w.2

/-- The border of the cut disc walked round the vertex, as unit steps
`(start, direction)`: the two copies of each tree edge and everything between. -/
def border : Cut → GaussianInt → ℤ → List (GaussianInt × GaussianInt)
  | last _, _, _ => []
  | edge c below next, p, n =>
    let n₁ := n + 2 - c
    let w := walk below (p + quarter n₁) n₁
    (p, quarter n₁) :: border below (p + quarter n₁) n₁ ++
      (w.1, quarter w.2) :: border next (w.1 + quarter w.2) w.2

/-- **The jump of the tree edge** `f = edge c below _` leaving the copy `p`
arrived at along `R^n`: `e` runs from `p` to `q = p + R^{n₁}`, `e'` from the
copy `w.1` where the walk round `below` ends, along `R^{w.2}`. The jump takes
`p` to the end of `e'` and turns `R^{n₁}` into `-R^{w.2}`. -/
def jump (c : ℕ) (below : Cut) (p : GaussianInt) (n : ℤ) : Motion GaussianInt GaussianInt :=
  let n₁ := n + 2 - c
  let w := walk below (p + quarter n₁) n₁
  ⟨quarter (w.2 - n₁ + 2), w.1 + quarter w.2 - quarter (w.2 - n₁ + 2) * p⟩

/-- The composite `G_{f_r} ∘ ⋯ ∘ G_{f_1}` of the jumps of the edges leaving the
vertex, in the order of the walk. -/
def rootJumps : Cut → GaussianInt → ℤ → Motion GaussianInt GaussianInt
  | last _, _, _ => Motion.id
  | edge c below next, p, n =>
    let w := walk below (p + quarter (n + 2 - c)) (n + 2 - c)
    comp (rootJumps next (w.1 + quarter w.2) w.2) (jump c below p n)

/-- The cone points below the vertex, as a forest: at the far end `v` of each
tree edge, its rotation `R^{4 - k}` and its first copy `q`. -/
def cones : Cut → GaussianInt → ℤ → Forest (GaussianInt × GaussianInt)
  | last _, _, _ => .nil
  | edge c below next, p, n =>
    let n₁ := n + 2 - c
    let q := p + quarter n₁
    let w := walk below q n₁
    .node (quarter (4 - corners below), q) (cones below q n₁) (cones next (w.1 + quarter w.2) w.2)

/-! ### The walk round one vertex -/

theorem jump_lin (c : ℕ) (below : Cut) (p : GaussianInt) (n : ℤ) :
    (jump c below p n).lin =
      quarter ((walk below (p + quarter (n + 2 - c)) (n + 2 - c)).2 - (n + 2 - c) + 2) := rfl

/-- The jump takes the start of `e` to the end of `e'`. -/
theorem jump_apply_start (c : ℕ) (below : Cut) (p : GaussianInt) (n : ℤ) :
    (jump c below p n).apply p =
      (walk below (p + quarter (n + 2 - c)) (n + 2 - c)).1 +
        quarter (walk below (p + quarter (n + 2 - c)) (n + 2 - c)).2 := by
  simp [jump, Motion.apply]

/-- The jump takes the end of `e` to the start of `e'`. -/
theorem jump_apply_end (c : ℕ) (below : Cut) (p : GaussianInt) (n : ℤ) :
    (jump c below p n).apply (p + quarter (n + 2 - c)) =
      (walk below (p + quarter (n + 2 - c)) (n + 2 - c)).1 := by
  simp only [jump, Motion.apply, smul_eq_mul]
  have h : quarter ((walk below (p + quarter (n + 2 - c)) (n + 2 - c)).2 - (n + 2 - c) + 2) *
      quarter (n + 2 - c) =
      -quarter (walk below (p + quarter (n + 2 - c)) (n + 2 - c)).2 := by
    rw [← quarter_add, show ∀ a b : ℤ, a - b + 2 + b = a + 2 by intros; ring, quarter_add,
      quarter_two, mul_neg_one]
  linear_combination h

/-- **The walk round a vertex, linear part.** The jumps of the edges leaving a
vertex compose to `R^{n' - n + k - 2}`: the walk arrives along `R^n`, leaves
along `R^{n'}`, and turns by `R^{2 - c}` at each copy. -/
theorem rootJumps_lin (C : Cut) (p : GaussianInt) (n : ℤ) :
    (rootJumps C p n).lin = quarter ((walk C p n).2 - n + corners C - 2) := by
  induction C generalizing p n with
  | last c =>
    simp only [rootJumps, walk, corners, Motion.id_lin]
    rw [show n + 2 - (c : ℤ) - n + c - 2 = 0 by ring]; rfl
  | edge c below next _ ihn =>
    simp only [rootJumps, walk, corners, comp_lin, ihn, jump_lin, ← quarter_add]
    congr 1; push_cast; ring

/-- **The walk round a vertex, translation part.** The jumps of the edges
leaving a vertex carry the copy it is arrived at to the copy it is left from. -/
theorem rootJumps_apply (C : Cut) (p : GaussianInt) (n : ℤ) :
    (rootJumps C p n).apply p = (walk C p n).1 := by
  induction C generalizing p n with
  | last c => simp [rootJumps, walk, Motion.apply]
  | edge c below next _ ihn =>
    simp only [rootJumps, walk, apply_comp, jump_apply_start, ihn]

/-- **The jump of an edge is the rotation at its far vertex composed with the
jumps below:** `G_f = G_{f_r} ∘ ⋯ ∘ G_{f_1} ∘ ρ_v`, `ρ_v` the rotation by
`R^{4 - k}` about the first copy `q` of `v`. Both take `q` to the start of `e'`,
and the corners at the `k` copies of `v` turn the walk by `k` quarters, so the
`4 - k` that remain are the rotation. -/
theorem jump_eq (c : ℕ) (below : Cut) (p : GaussianInt) (n : ℤ) :
    jump c below p n =
      comp (rootJumps below (p + quarter (n + 2 - c)) (n + 2 - c))
        (rotAbout (quarter (4 - corners below)) (p + quarter (n + 2 - c))) := by
  apply Motion.eq_of_apply (p + quarter (n + 2 - c))
  · rw [jump_lin, comp_lin, rootJumps_lin, rotAbout_lin, ← quarter_add]
    congr 1; ring
  · rw [jump_apply_end, apply_comp, apply_rotAbout, sub_self, smul_zero, zero_add,
      rootJumps_apply]

/-- **The chart continued along `γ̄`.** At a vertex `x` of `γ̄` with `k` face
corners, `γ̄` turns by `R^{2 - k}` in `D`; the jumps at `x` carry that direction
to the one along which the border of `Ω` leaves `x`, and the copy it arrives at
to the copy it leaves from (`rootJumps_apply`). -/
theorem rootJumps_continues (C : Cut) (p : GaussianInt) (n : ℤ) :
    (rootJumps C p n).lin * quarter (n + 2 - corners C) = quarter (walk C p n).2 := by
  rw [rootJumps_lin, ← quarter_add]; congr 1; ring

/-- **The jumps at a vertex are the gluing of its forest of cone rotations**:
`Forest.glue` with `ρ_v` at each far end `v`. -/
theorem rootJumps_eq_glue (C : Cut) (p : GaussianInt) (n : ℤ) :
    rootJumps C p n = Forest.glue ((cones C p n).map fun c => rotAbout c.1 c.2) := by
  induction C generalizing p n with
  | last c => rfl
  | edge c below next ihb ihn =>
    simp only [rootJumps, cones, Forest.map, Forest.glue, ← ihb, ← ihn, ← jump_eq]

end Cut

/-! ### The cut disc -/

/-- A disc of quads cut along a forest, by the walk round the border of the cut
disc `Ω`: the vertices `x₁, …, x_m` of `γ̄` in order, each with the forest
hanging from it and followed by the edge of `γ̄` leaving it; the walk starts at
the copy `start` of `x₁`, arrived at along `R^dir`. -/
structure CutDisc where
  /-- The copy of the first vertex of `γ̄` in the plane. -/
  start : GaussianInt
  /-- The direction `R^dir` of the edge of `γ̄` arriving at it. -/
  dir : ℤ
  /-- The vertices of `γ̄`, in order. -/
  vertices : List Cut
  deriving DecidableEq, Repr

namespace CutDisc

/-- The walk along `γ̄` from the copy `p`, arrived at along `R^n`: the vertex,
then the edge of `γ̄` leaving it. -/
def walkFrom : List Cut → GaussianInt → ℤ → GaussianInt × ℤ
  | [], p, n => (p, n)
  | C :: Cs, p, n =>
    let w := C.walk p n
    walkFrom Cs (w.1 + quarter w.2) w.2

/-- The border of `Ω` from the copy `p`, as unit steps `(start, direction)`. -/
def borderFrom : List Cut → GaussianInt → ℤ → List (GaussianInt × GaussianInt)
  | [], _, _ => []
  | C :: Cs, p, n =>
    let w := C.walk p n
    C.border p n ++ (w.1, quarter w.2) :: borderFrom Cs (w.1 + quarter w.2) w.2

/-- The composite of the jumps of the roots, in border order. -/
def holonomyFrom : List Cut → GaussianInt → ℤ → Motion GaussianInt GaussianInt
  | [], _, _ => Motion.id
  | C :: Cs, p, n =>
    let w := C.walk p n
    comp (holonomyFrom Cs (w.1 + quarter w.2) w.2) (C.rootJumps p n)

/-- The cone points of the forest, its trees in border order. -/
def conesFrom : List Cut → GaussianInt → ℤ → Forest (GaussianInt × GaussianInt)
  | [], _, _ => .nil
  | C :: Cs, p, n =>
    let w := C.walk p n
    Forest.append (C.cones p n) (conesFrom Cs (w.1 + quarter w.2) w.2)

variable (D : CutDisc)

/-- The border of `Ω`: a list the program can compare with a tree of faces. -/
def border : List (GaussianInt × GaussianInt) := borderFrom D.vertices D.start D.dir

/-- The border of `Ω` closes: the walk returns to its start, along the same direction. -/
def Closes : Prop :=
  (walkFrom D.vertices D.start D.dir).1 = D.start ∧
    quarter (walkFrom D.vertices D.start D.dir).2 = quarter D.dir

instance : Decidable D.Closes := by unfold Closes; infer_instance

/-- **The holonomy of the border `γ̄` of `D`**: the chart continued along `γ̄`
crosses the forest at its roots, by their jumps. -/
def holonomy : Motion GaussianInt GaussianInt := holonomyFrom D.vertices D.start D.dir

/-- The cone points `(R_v, p_v)` of the forest, in the forest of the walk. -/
def cones : Forest (GaussianInt × GaussianInt) := conesFrom D.vertices D.start D.dir

theorem holonomyFrom_eq_glue (Cs : List Cut) (p : GaussianInt) (n : ℤ) :
    holonomyFrom Cs p n = Forest.glue ((conesFrom Cs p n).map fun c => rotAbout c.1 c.2) := by
  induction Cs generalizing p n with
  | nil => rfl
  | cons C Cs ih =>
    simp only [holonomyFrom, conesFrom, Forest.map_append, Forest.glue_append, ← ih,
      ← Cut.rootJumps_eq_glue]

/-- **Proposition "The shift is set by the singularities" (a), geometric half.**
The holonomy of the border of the cut disc is `Forest.glue` of the rotations
round its cone points. -/
theorem holonomy_eq_glue : D.holonomy = Forest.glue (D.cones.map fun c => rotAbout c.1 c.2) :=
  holonomyFrom_eq_glue _ _ _

/-- **The order of first visit.** The holonomy of the border is `ρ_m ∘ ⋯ ∘ ρ_1`,
the rotations round the cone points in the order the walk first reaches them. -/
theorem holonomy_eq_preorder :
    D.holonomy = composeAll (rotations (Forest.preorder D.cones)) := by
  rw [holonomy_eq_glue, Forest.glue_eq_preorder, Forest.preorder_map]; rfl

/-- **The Burgers vector.** The holonomy of the border is `x ↦ R_m ⋯ R_1 x + B_D`,
`B_D = ∑ᵢ Qᵢ (1 - Rᵢ) pᵢ` over the cone points in the order of first visit,
`Qᵢ = R_m ⋯ R_{i+1}`. -/
theorem holonomy_eq_burgers :
    D.holonomy = ⟨((Forest.preorder D.cones).map Prod.fst).reverse.prod,
      burgers (Forest.preorder D.cones)⟩ := by
  rw [holonomy_eq_preorder, composeAll_rotations]

/-- The Burgers vector in indexed form. -/
theorem holonomy_trans :
    D.holonomy.trans = ∑ i : Fin (Forest.preorder D.cones).length,
      (((Forest.preorder D.cones).drop (i + 1)).map Prod.fst).reverse.prod •
        (((Forest.preorder D.cones).get i).2 -
          ((Forest.preorder D.cones).get i).1 • ((Forest.preorder D.cones).get i).2) := by
  rw [holonomy_eq_burgers, burgers_eq_sum]

/-! ### An example: three squares round a corner of a cube -/

/-- The squares `[0,1]×[0,1]`, `[-1,0]×[0,1]`, `[-1,0]×[-1,0]` round the vertex
`v = 0` of valence three, cut along the edge from `v` to the middle `x₀` of a
side of the hexagon `γ̄`; the walk starts at the copy `-i` of `x₀`. -/
def threeSquares : CutDisc :=
  ⟨⟨0, -1⟩, 0, [.edge 1 (.last 3) (.last 1), .last 1, .last 2, .last 1, .last 2, .last 1]⟩

theorem threeSquares_closes : threeSquares.Closes := by decide

theorem threeSquares_border : threeSquares.border =
    [(⟨0, -1⟩, gi), (0, 1), (1, gi), (⟨1, 1⟩, -1), (gi, -1), (⟨-1, 1⟩, -gi), (-1, -gi),
      (⟨-1, -1⟩, 1)] := by decide

/-- One cone point, `v = 0`, with rotation `R^{4 - 3} = R`. -/
theorem threeSquares_cones : Forest.preorder threeSquares.cones = [(gi, 0)] := by decide

/-- The holonomy of the hexagon is the quarter-turn about `v`. -/
theorem threeSquares_holonomy : threeSquares.holonomy = rotAbout gi 0 := by
  have h : threeSquares.holonomy.lin = gi ∧ threeSquares.holonomy.trans = 0 := by decide
  ext : 1
  · exact h.1
  · rw [h.2]; simp

end CutDisc

end TurnsOfATMesh
