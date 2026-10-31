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
across a tree edge. A telescope of charts then reads the holonomy of the border
as the composite, in border order, of the jumps across the cuts, the edges of the
region off the tree: the `G_f` of the note. The border turns round its own
vertices only, so the cuts it crosses are those with an end on the curve.

Here: the face beside an edge and the face on the left of a step (`faceBeside`,
`faceAt`); the crossings of the chart continued along a curve with the faces
they leave and enter (`turnAtF`, `vertexCrossingsF`, `borderCrossings`, the
definitions of `Border.lean` with the faces written down); the inverse of a
motion whose linear part is a quarter-turn (`qinv`); the jump of a crossing
through charts (`jump`).

Proved:
* `borderCrossings_holonomy`: `borderHolonomy ℓ fuel steps` is the composite of
  the motions of `borderCrossings ℓ fuel steps`, for all `ℓ`, `fuel`, `steps`.
* `borderCrossings_chain`: round a closed curve `s :: ss` of unit steps
  (`IsQuarter t.2` for every step `t`) with `normalize ℓ s = some s'`, the faces of
  `borderCrossings ℓ fuel (s :: ss) = some cs` chain:
  `faceAt s' :: cs.map (·.2.2) = cs.map (·.1) ++ [faceAt s']`. Each crossing enters
  the face the next one leaves, the first leaves the face on the left of the first
  step read in its own frame, and the last enters it. The unit steps are needed
  where `normalize` reads a step across a side: the end of the step may then lie
  one off the rectangle, and the face is the same only because the corner of the
  face is cut to `0` too (`faceAt_turn`).
* `qinv_comp`, `comp_qinv`: `qinv M` is the inverse of `M` when `M.lin` is a
  quarter-turn.
* `jump_tree`: across a crossing where the charts agree (`M g ∘ C = M f`) the
  jump is the identity.
* `borderHolonomy_jumps`: with charts `M` whose linear parts are quarter-turns,
  the identity on `faceAt s'`, the holonomy of the border is the composite, in
  border order, of the jumps of all its crossings (the telescope, by induction
  along the chain: `facesChain_telescope`).
* `borderHolonomy_cuts`: and of the jumps across the cuts alone
  (`cutCrossings tree cs`), when the charts agree across every crossing `tree`
  holds for.

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

/-! ### The crossings carry the changes of chart of `Border.lean` -/

theorem turnAtF_map (ℓ : α → ℕ) :
    ∀ (fuel : ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch) (d₀ : GaussianInt),
      (T.turnAtF ℓ fuel v d v₀ d₀).map (List.map (·.2.1)) = T.turnAt ℓ fuel v d v₀ d₀
  | 0, _, _, _, _ => rfl
  | fuel + 1, v, d, v₀, d₀ => by
    rw [turnAtF, turnAt]
    split_ifs
    · rfl
    · cases h : T.aroundStep ℓ (-gi) v d with
      | none => rfl
      | some r =>
        obtain ⟨v', d', C⟩ := r
        simp only [Option.bind_some, ← turnAtF_map ℓ fuel v' d' v₀ d₀, Option.map_map]
        cases T.turnAtF ℓ fuel v' d' v₀ d₀ <;> rfl

theorem vertexCrossingsF_map (ℓ : α → ℕ) (fuel : ℕ) (s₁ s₂ : Step Patch) :
    (T.vertexCrossingsF ℓ fuel s₁ s₂).map (List.map (·.2.1)) = T.vertexCrossings ℓ fuel s₁ s₂ := by
  unfold vertexCrossingsF vertexCrossings
  cases T.normalize ℓ s₁ with
  | none => rfl
  | some ud =>
    cases T.normalize ℓ s₂ with
    | none => rfl
    | some vd => exact T.turnAtF_map ℓ fuel _ _ _ _

/-- `mapM` of two functions related by `map (map g)`. -/
theorem mapM_map_map {β γ δ : Type*} (F : β → Option (List γ)) (G : β → Option (List δ))
    (g : γ → δ) (hFG : ∀ x, (F x).map (List.map g) = G x) :
    ∀ l : List β, (l.mapM F).map (List.map (List.map g)) = l.mapM G
  | [] => rfl
  | x :: l => by
    simp only [List.mapM_cons, ← hFG x, ← mapM_map_map F G g hFG l]
    cases F x <;> cases l.mapM F <;> rfl

/-- **The holonomy of the border is the composite of its crossings.** -/
theorem borderCrossings_holonomy (ℓ : α → ℕ) (fuel : ℕ) (steps : List (Step Patch)) :
    T.borderHolonomy ℓ fuel steps =
      (T.borderCrossings ℓ fuel steps).map fun cs => Motion.composeAll (cs.map (·.2.1)) := by
  cases steps with
  | nil => rfl
  | cons s ss =>
    have key := mapM_map_map (fun p : Step Patch × Step Patch => T.vertexCrossingsF ℓ fuel p.1 p.2)
      (fun p => T.vertexCrossings ℓ fuel p.1 p.2) (fun c : Crossing Patch => c.2.1)
      (fun p => T.vertexCrossingsF_map ℓ fuel p.1 p.2) ((s :: ss).zip (ss ++ [s]))
    rw [borderHolonomy, borderCrossings, ← key, Option.map_map, Option.map_map]
    congr 1
    funext css
    simp [Function.comp, List.map_flatten]

/-! ### The faces of the crossings chain -/

/-- The crossings `cs` run from the face `f₀` to the face `f₁`: each enters the face
the next one leaves, the first leaves `f₀` and the last enters `f₁` (`cs = []`:
`f₀ = f₁`). -/
def FacesChain (f₀ : Face Patch) (cs : List (Crossing Patch)) (f₁ : Face Patch) : Prop :=
  f₀ :: cs.map (·.2.2) = cs.map (·.1) ++ [f₁]

theorem FacesChain.append {f₀ f₁ f₂ : Face Patch} {cs₁ cs₂ : List (Crossing Patch)}
    (h₁ : FacesChain f₀ cs₁ f₁) (h₂ : FacesChain f₁ cs₂ f₂) : FacesChain f₀ (cs₁ ++ cs₂) f₂ := by
  unfold FacesChain at *
  rw [List.map_append, List.map_append, ← List.cons_append, h₁, List.append_assoc,
    List.singleton_append, h₂, List.append_assoc]

/-- The crossings of `turnAtF` run from the face on the left of the state it
starts from to the face on the left of the state it stops at. -/
theorem turnAtF_chain (ℓ : α → ℕ) :
    ∀ (fuel : ℕ) (v : Point Patch) (d : GaussianInt) (v₀ : Point Patch) (d₀ : GaussianInt)
      (cs : List (Crossing Patch)), T.turnAtF ℓ fuel v d v₀ d₀ = some cs →
      FacesChain (faceAt (v, d)) cs (faceAt (v₀, d₀))
  | 0, _, _, _, _, _, h => by simp at h
  | fuel + 1, v, d, v₀, d₀, cs, h => by
    rw [turnAtF] at h
    split_ifs at h with hs
    · obtain ⟨rfl, rfl⟩ := hs
      cases Option.some.inj h
      rfl
    · obtain ⟨⟨v', d', C⟩, -, h'⟩ := Option.bind_eq_some_iff.mp h
      obtain ⟨cs', hcs', rfl⟩ := Option.map_eq_some_iff.mp h'
      have ih := turnAtF_chain ℓ fuel v' d' v₀ d₀ cs' hcs'
      unfold FacesChain at ih ⊢
      simp only [List.map_cons, List.cons_append, ih]

/-- The face on the left of the edge a quarter-turn to the left of a unit step,
from its end, is the face on the left of the step, even when that end is read off
the rectangle (one of its coordinates `-1`, cut to `0`, as is the corner of the
face). -/
theorem faceAt_turn (u : Point Patch) {d : GaussianInt} (hd : IsQuarter d) :
    faceAt (pointAt u.patch (fine u + d), gi * d) = faceAt (u, d) := by
  obtain ⟨P, x, y⟩ := u
  rcases hd with rfl | rfl | rfl | rfl <;>
    simp [faceAt, faceBeside, lowerLeft, pointAt, fine, gi] <;> omega

theorem normalize_quarter {ℓ : α → ℕ} {s : Step Patch} {u : Point Patch} {d : GaussianInt}
    (h : T.normalize ℓ s = some (u, d)) (hs : IsQuarter s.2) : IsQuarter d := by
  unfold normalize at h
  dsimp only at h
  split_ifs at h
  · cases Option.some.inj h
    exact hs
  · obtain ⟨⟨Q, C⟩, hc, he⟩ := Option.map_eq_some_iff.mp h
    simp only [Prod.mk.injEq] at he
    rw [← he.2]
    exact (T.crossEdge_lin_quarter hc).mul hs

theorem vertexCrossingsF_chain {ℓ : α → ℕ} {fuel : ℕ} {s₁ s₂ s₁' s₂' : Step Patch}
    {cs : List (Crossing Patch)} (hq : IsQuarter s₁.2) (h₁ : T.normalize ℓ s₁ = some s₁')
    (h₂ : T.normalize ℓ s₂ = some s₂') (h : T.vertexCrossingsF ℓ fuel s₁ s₂ = some cs) :
    FacesChain (faceAt s₁') cs (faceAt s₂') := by
  obtain ⟨u, d⟩ := s₁'
  obtain ⟨v₀, d₀⟩ := s₂'
  unfold vertexCrossingsF at h
  simp only [h₁, h₂, Option.bind_some] at h
  have := T.turnAtF_chain ℓ fuel _ _ v₀ d₀ cs h
  rwa [faceAt_turn u (T.normalize_quarter h₁ hq)] at this

theorem vertexCrossingsF_normalize {ℓ : α → ℕ} {fuel : ℕ} {s₁ s₂ : Step Patch}
    {cs : List (Crossing Patch)} (h : T.vertexCrossingsF ℓ fuel s₁ s₂ = some cs) :
    (∃ s₁', T.normalize ℓ s₁ = some s₁') ∧ ∃ s₂', T.normalize ℓ s₂ = some s₂' := by
  unfold vertexCrossingsF at h
  obtain ⟨s₁', h₁, h'⟩ := Option.bind_eq_some_iff.mp h
  obtain ⟨s₂', h₂, -⟩ := Option.bind_eq_some_iff.mp h'
  exact ⟨⟨s₁', h₁⟩, s₂', h₂⟩

/-- The crossings between consecutive steps of a list of unit steps, then from its
last step to `y`, run from the face on the left of its first step to the face on
the left of `y`. -/
theorem mapM_zip_facesChain (ℓ : α → ℕ) (fuel : ℕ) {y y' : Step Patch}
    (hy : T.normalize ℓ y = some y') :
    ∀ (x : Step Patch) (l : List (Step Patch)) (x' : Step Patch) (css : List (List (Crossing Patch))),
      (∀ t ∈ x :: l, IsQuarter t.2) → T.normalize ℓ x = some x' →
      ((x :: l).zip (l ++ [y])).mapM (fun p => T.vertexCrossingsF ℓ fuel p.1 p.2) = some css →
      FacesChain (faceAt x') css.flatten (faceAt y')
  | x, [], x', css, hq, hx, h => by
    simp only [List.nil_append, List.zip_cons_cons, List.zip_nil_right, List.mapM_cons,
      List.mapM_nil] at h
    cases hc : T.vertexCrossingsF ℓ fuel x y with
    | none => simp [hc] at h
    | some c =>
      simp only [hc] at h
      cases Option.some.inj h
      simpa using T.vertexCrossingsF_chain (hq x (by simp)) hx hy hc
  | x, z :: l, x', css, hq, hx, h => by
    simp only [List.cons_append, List.zip_cons_cons, List.mapM_cons] at h
    cases hc : T.vertexCrossingsF ℓ fuel x z with
    | none => simp [hc] at h
    | some c =>
      cases hr : ((z :: l).zip (l ++ [y])).mapM (fun p => T.vertexCrossingsF ℓ fuel p.1 p.2) with
      | none => simp [hc, hr] at h
      | some css' =>
        simp only [hc, hr] at h
        cases Option.some.inj h
        obtain ⟨z', hz⟩ := (T.vertexCrossingsF_normalize hc).2
        have ih := mapM_zip_facesChain ℓ fuel hy z l z' css'
          (fun t ht => hq t (List.mem_cons_of_mem _ ht)) hz hr
        rw [List.flatten_cons]
        exact (T.vertexCrossingsF_chain (hq x (by simp)) hx hz hc).append ih

/-- **The crossings of the border chain**: round a closed curve of unit steps, each
crossing enters the face the next one leaves, and the crossings start from and come
back to the face on the left of the first step, read in its own frame. -/
theorem borderCrossings_chain {ℓ : α → ℕ} {fuel : ℕ} {s s' : Step Patch} {ss : List (Step Patch)}
    {cs : List (Crossing Patch)} (hq : ∀ t ∈ s :: ss, IsQuarter t.2)
    (hs : T.normalize ℓ s = some s') (h : T.borderCrossings ℓ fuel (s :: ss) = some cs) :
    faceAt s' :: cs.map (·.2.2) = cs.map (·.1) ++ [faceAt s'] := by
  unfold borderCrossings at h
  obtain ⟨css, hcss, rfl⟩ := Option.map_eq_some_iff.mp h
  exact T.mapM_zip_facesChain ℓ fuel hs s ss s' css hq hs hcss

/-! ### Jumps through charts -/

/-- **`qinv M` undoes `M`**, `M` a motion whose linear part is a quarter-turn. -/
theorem qinv_comp {M : Motion GaussianInt GaussianInt} (hq : IsQuarter M.lin) :
    Motion.comp (qinv M) M = Motion.id := by
  have h := mul_conj_of_isQuarter hq
  refine Motion.ext ?_ ?_
  · simp only [Motion.comp_lin, qinv_lin, Motion.id_lin]
    rw [mul_comm, h]
  · simp only [Motion.comp_trans, qinv_lin, qinv_trans, Motion.id_trans, smul_eq_mul]
    ring

/-- **`M` undoes `qinv M`**, `M` a motion whose linear part is a quarter-turn. -/
theorem comp_qinv {M : Motion GaussianInt GaussianInt} (hq : IsQuarter M.lin) :
    Motion.comp M (qinv M) = Motion.id := by
  have h := mul_conj_of_isQuarter hq
  refine Motion.ext ?_ ?_
  · simp only [Motion.comp_lin, qinv_lin, Motion.id_lin, h]
  · simp only [Motion.comp_trans, qinv_trans, Motion.id_trans, smul_eq_mul]
    rw [mul_neg, ← mul_assoc, h, one_mul, add_neg_cancel]

/-- **Across a tree edge the jump is the identity**: when the charts agree across the
crossing (`M g ∘ C = M f`). -/
theorem jump_tree (M : Face Patch → Motion GaussianInt GaussianInt) (c : Crossing Patch)
    (hq : IsQuarter (M c.1).lin) (h : Motion.comp (M c.2.2) c.2.1 = M c.1) :
    jump M c = Motion.id := by
  rw [jump, ← Motion.comp_assoc, h, comp_qinv hq]

/-- A crossing seen through the charts: `M g ∘ C = J ∘ M f`, `J` its jump. -/
theorem jump_comp (M : Face Patch → Motion GaussianInt GaussianInt) (c : Crossing Patch)
    (hq : IsQuarter (M c.1).lin) :
    Motion.comp (jump M c) (M c.1) = Motion.comp (M c.2.2) c.2.1 := by
  rw [jump, Motion.comp_assoc, Motion.comp_assoc, qinv_comp hq, Motion.comp_id]

/-- **The telescope along a chain of crossings**: from `f₀` to `f₁`, the composite
of the changes of chart, read through the charts at both ends, is the composite of
the jumps. -/
theorem facesChain_telescope (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) {f₁ : Face Patch} :
    ∀ (cs : List (Crossing Patch)) {f₀ : Face Patch}, FacesChain f₀ cs f₁ →
      Motion.comp (M f₁) (Motion.composeAll (cs.map (·.2.1))) =
        Motion.comp (Motion.composeAll (cs.map (jump M))) (M f₀)
  | [], f₀, h => by
    unfold FacesChain at h
    simp only [List.map_nil, List.nil_append, List.cons.injEq, and_true] at h
    subst h
    simp
  | c :: cs, f₀, h => by
    unfold FacesChain at h
    simp only [List.map_cons, List.cons_append, List.cons.injEq] at h
    obtain ⟨rfl, h⟩ := h
    have ih := facesChain_telescope M hq cs (f₀ := c.2.2) h
    simp only [List.map_cons, Motion.composeAll_cons]
    rw [← Motion.comp_assoc, ih, Motion.comp_assoc, ← jump_comp M c (hq _), Motion.comp_assoc]

/-- **The holonomy of the border is the composite of the jumps** of its crossings,
through charts `M` that are the identity on the face on the left of its first step
(the telescope). -/
theorem borderHolonomy_jumps (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) {ℓ : α → ℕ} {fuel : ℕ} {s s' : Step Patch}
    {ss : List (Step Patch)} {cs : List (Crossing Patch)} (hd : ∀ t ∈ s :: ss, IsQuarter t.2)
    (hs : T.normalize ℓ s = some s') (h₀ : M (faceAt s') = Motion.id)
    (h : T.borderCrossings ℓ fuel (s :: ss) = some cs) :
    T.borderHolonomy ℓ fuel (s :: ss) = some (Motion.composeAll (cs.map (jump M))) := by
  have ht := facesChain_telescope M hq cs (T.borderCrossings_chain hd hs h)
  rw [h₀, Motion.id_comp, Motion.comp_id] at ht
  rw [borderCrossings_holonomy, h, Option.map_some, ht]

/-- Dropping identities from a composite. -/
theorem composeAll_filter_id {X : Type*} (g : X → Motion GaussianInt GaussianInt) (p : X → Bool) :
    ∀ l : List X, (∀ x ∈ l, p x = false → g x = Motion.id) →
      Motion.composeAll ((l.filter p).map g) = Motion.composeAll (l.map g)
  | [], _ => rfl
  | x :: l, h => by
    have ih := composeAll_filter_id g p l (fun y hy => h y (List.mem_cons_of_mem _ hy))
    cases hp : p x
    · simp [hp, ih, h x (by simp) hp]
    · simp [hp, ih]

/-- **The holonomy of the border is the composite of the jumps across the cuts**, in
border order, through charts `M` that are the identity on the face on the left of the
first step and agree across every crossing `tree` holds for. -/
theorem borderHolonomy_cuts (M : Face Patch → Motion GaussianInt GaussianInt)
    (hq : ∀ f, IsQuarter (M f).lin) (tree : Face Patch → Face Patch → Bool) {ℓ : α → ℕ}
    {fuel : ℕ} {s s' : Step Patch} {ss : List (Step Patch)} {cs : List (Crossing Patch)}
    (hd : ∀ t ∈ s :: ss, IsQuarter t.2) (hs : T.normalize ℓ s = some s')
    (h₀ : M (faceAt s') = Motion.id) (h : T.borderCrossings ℓ fuel (s :: ss) = some cs)
    (htree : ∀ c ∈ cs, tree c.1 c.2.2 = true → Motion.comp (M c.2.2) c.2.1 = M c.1) :
    T.borderHolonomy ℓ fuel (s :: ss) =
      some (Motion.composeAll ((cutCrossings tree cs).map (jump M))) := by
  rw [T.borderHolonomy_jumps M hq hd hs h₀ h, cutCrossings,
    composeAll_filter_id (jump M) (fun c => !tree c.1 c.2.2) cs]
  intro c hc hcut
  simp only [Bool.not_eq_false'] at hcut
  exact jump_tree M c (hq _) (htree c hc hcut)

end TMesh

end TurnsOfATMesh
