/-
The order of first visit in Proposition "The shift is set by the singularities".

The proof in the note cuts the disc along a tree `T` and finds, at each vertex
`v` reached by an edge `f`, the recursion `G_f = G_{f_r} ∘ ⋯ ∘ G_{f_1} ∘ ρ_v`,
`f_1, …, f_r` the edges leaving `v` in the order of the walk round `T`, and at
the root `ĥ(γ̄) = G_{f_r} ∘ ⋯ ∘ G_{f_1}`. Taking this recursion as the hypothesis
(it is the geometric part of the proof), unfolding it gives the composition of
the rotations in preorder: each vertex before the vertices below it, the
branches in the order of the walk. That is the order of first visit.

Trees are stored as forests in the first-child, next-sibling form.
-/
module

public import ClosingHelices.QuarterTurn

@[expose] public section

set_option linter.unusedSectionVars false

namespace ClosingHelices

open Motion

/-- A forest: `node ρ children siblings` is a tree whose root carries `ρ` (the
rotation round that vertex, the identity at a vertex that is not singular),
followed by its next siblings. -/
inductive Forest (M : Type*)
  | nil : Forest M
  | node (ρ : M) (children : Forest M) (siblings : Forest M) : Forest M

namespace Forest

variable {A V : Type*} [Ring A] [AddCommGroup V] [Module A V]

/-- The gluing motions: for sibling trees `f₁, …, f_r`, `G_{f_r} ∘ ⋯ ∘ G_{f_1}`, with
`G_f = G_{children} ∘ ρ` for one tree. -/
def glue : Forest (Motion A V) → Motion A V
  | nil => Motion.id
  | node ρ ch sib => comp (glue sib) (comp (glue ch) ρ)

/-- The preorder of a forest: the order of first visit of a walk round it. -/
def preorder {M : Type*} : Forest M → List M
  | nil => []
  | node ρ ch sib => ρ :: (preorder ch ++ preorder sib)

/-- The postorder, the order of last visit. -/
def postorder {M : Type*} : Forest M → List M
  | nil => []
  | node ρ ch sib => postorder ch ++ [ρ] ++ postorder sib

/-- **The order of first visit.** The recursion of the gluing motions unfolds
into the composition of the rotations in preorder. -/
theorem glue_eq_preorder (f : Forest (Motion A V)) : glue f = composeAll (preorder f) := by
  induction f with
  | nil => rfl
  | node ρ ch sib ihc ihs =>
    rw [glue, preorder, composeAll_cons, composeAll_append, ihc, ihs, comp_assoc]

/-- The forest with every label mapped by `g`. -/
def map {M N : Type*} (g : M → N) : Forest M → Forest N
  | nil => nil
  | node ρ ch sib => node (g ρ) (map g ch) (map g sib)

theorem preorder_map {M N : Type*} (g : M → N) (f : Forest M) :
    preorder (map g f) = (preorder f).map g := by
  induction f with
  | nil => rfl
  | node ρ ch sib ihc ihs => simp [map, preorder, ihc, ihs]

/-- With rotations about cone points at the nodes, the holonomy of the curve is
the translation by the Burgers sum taken in preorder, when the rotations
compose to the identity. -/
theorem glue_rotations (f : Forest (A × V))
    (h : ((preorder f).map Prod.fst).reverse.prod = 1) :
    glue (map (fun c => rotAbout c.1 c.2) f) = translation (burgers (preorder f)) := by
  rw [glue_eq_preorder, preorder_map, ← composeAll_rotations_of_prod_eq_one _ h]
  rfl

/-- The order matters: for a vertex `v` (a quarter-turn about `0`) and a vertex
`w` below it (a quarter-turn about `1`), the order of first visit and the order
of last visit give different translations (`1 - i` and `1 + i`). -/
theorem order_of_first_visit_matters :
    let f : Forest (Motion ℂ ℂ) :=
      node (rotAbout Complex.I 0) (node (rotAbout Complex.I 1) nil nil) nil
    (glue f).trans = 1 - Complex.I ∧ (composeAll (postorder f)).trans = 1 + Complex.I := by
  simp only [glue, postorder, comp, rotAbout, Motion.id, List.nil_append, List.cons_append,
    composeAll_cons, composeAll_nil, smul_eq_mul]
  constructor <;> (ring_nf; try simp [Complex.ext_iff])

end Forest

end ClosingHelices
