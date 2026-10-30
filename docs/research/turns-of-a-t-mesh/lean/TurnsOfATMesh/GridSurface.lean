/-
The quad mesh of a T-mesh at whole lengths (`Grid.lean`) is a closed surface made
of quads, in the sense of `QuadSurface`: the faces have four distinct corners and
four distinct edges (`corners_nodup`, `faceEdges_nodup`), an edge determines its two
ends (`ends_eq_of_edgeOf_eq`), every edge lies in exactly two slots of faces
(`card_slots`), and the vertices and the faces are connected when the T-mesh is
(`vertex_connected`, `face_connected`).

A vertex is the class of a point (`vertexOf`), an edge the class of its midpoint in
the grid at `double ℓ` (`edgeOf`), as `GridEuler.lean` counts them. Distinct corners
and edges need one more hypothesis, `NoAdjacentArc`: an arc folded at the corner of
one patch glues two corners of the face at that corner.
-/
module

public import TurnsOfATMesh.GridEuler

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-- No arc lies on two adjacent sides of one patch. -/
def NoAdjacentArc : Prop :=
  ∀ a, (T.ends a 0).1 = (T.ends a 1).1 → (T.ends a 0).2.1 = (T.ends a 1).2.1

/-- The edge of the quad mesh of a face edge: the class of its midpoint in the grid
at `double ℓ`. -/
def edgeOf (allArcs : List α) (ℓ : α → ℕ) (e : Point Patch × Point Patch) :
    T.Vertex allArcs (double ℓ) :=
  T.vertexOf allArcs (double ℓ) (midpoint e)

/-- Two points of one patch at least a side apart, across or along. -/
def Far (ℓ : α → ℕ) (p q : Point Patch) : Prop :=
  p.x + 2 * T.width ℓ p.patch ≤ q.x ∨ q.x + 2 * T.width ℓ p.patch ≤ p.x ∨
    p.y + 2 * T.height ℓ p.patch ≤ q.y ∨ q.y + 2 * T.height ℓ p.patch ≤ p.y

variable {T}

theorem sidePoint_patch (ℓ : α → ℕ) (σ : Side Patch) (u : ℕ) : (T.sidePoint ℓ σ u).patch = σ.1 := by
  unfold sidePoint; split_ifs <;> rfl

/-- The points of two opposite sides are a side apart. -/
theorem far_sidePoint (ℓ : α → ℕ) {σ τ : Side Patch} (h₁ : σ.1 = τ.1) (h₂ : σ.2.1 = τ.2.1)
    (hne : σ ≠ τ) (u v : ℕ) : T.Far ℓ (T.sidePoint ℓ σ u) (T.sidePoint ℓ τ v) := by
  obtain ⟨P, j, k⟩ := σ
  obtain ⟨Q, j', k'⟩ := τ
  simp only at h₁ h₂
  subst h₁ h₂
  unfold Far
  rw [sidePoint_patch]
  fin_cases j <;> fin_cases k <;> fin_cases k' <;> simp_all [sidePoint]

/-- A point inside an arc, on either of its sides, is one vertex with its partner
on the other side, and with no other point. -/
theorem same_of_arcInner' {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hbal : T.Balanced ℓ) (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {a : α} {i : Fin 2}
    {s : ℕ} (hs₀ : 0 < s) (hs : s < 2 * ℓ a) {q : Point Patch}
    (h : T.Same allArcs ℓ (T.arcPoint ℓ a i s) q) :
    q = T.arcPoint ℓ a i s ∨ q = T.arcPoint ℓ a (i + 1) (T.flip ℓ a s) := by
  fin_cases i
  · exact same_of_arcInner hbal hwh hs₀ hs h
  · obtain ⟨hf₀, hf⟩ := flip_inner (T := T) hs₀ hs
    have hg := T.same_of_glued allArcs ℓ a (hA a) _ hf.le
    rw [glued_eq ℓ a hf.le, flip_flip hs.le] at hg
    rcases same_of_arcInner hbal hwh hf₀ hf (Relation.EqvGen.trans _ _ _ hg h) with h' | h'
    · exact Or.inr h'
    · exact Or.inl (by rw [h', flip_flip hs.le]; rfl)

/-- **Two points of one patch that are one vertex** are a side apart, or both have
even coordinates. -/
theorem far_or_coarse {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hT : T.NoAdjacentArc) (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {p q : Point Patch} (hp : T.InGrid ℓ p)
    (h : T.Same allArcs ℓ p q) (hpq : q.patch = p.patch) (hne' : q ≠ p) :
    T.Far ℓ p q ∨ (p.x % 2 = 0 ∧ p.y % 2 = 0 ∧ q.x % 2 = 0 ∧ q.y % 2 = 0) := by
  have hq : T.InGrid ℓ q := (inGrid_of_same hbal h).mp hp
  have hpar := parity_of_same h
  -- A point with two odd coordinates is inside its patch.
  have odd_inner : ∀ r : Point Patch, T.InGrid ℓ r → r.x % 2 = 1 → r.y % 2 = 1 → T.IsInner ℓ r :=
    fun r hr hx hy => by unfold InGrid at hr; unfold IsInner; omega
  by_cases hodd : (p.x + p.y) % 2 = 1
  · left
    by_cases hi : T.IsInner ℓ p
    · exact absurd (same_of_inner hi h) hne'
    obtain ⟨a, i, s, hs, rfl⟩ := exists_arcPoint hne hbal hp hi
    rw [arcPoint_parity] at hodd
    rcases same_of_arcInner' hA hbal hwh (by omega) (by omega) h with rfl | rfl
    · exact absurd rfl hne'
    have h₁ : (T.ends a (i + 1)).1 = (T.ends a i).1 := by
      simpa only [arcPoint, sidePoint_patch] using hpq
    have h₂ : (T.ends a i).2.1 = (T.ends a (i + 1)).2.1 := by
      fin_cases i
      · exact hT a h₁.symm
      · exact (hT a h₁).symm
    have h₃ : T.ends a i ≠ T.ends a (i + 1) := fun h => by
      have := ends_inj h; fin_cases i <;> simp at this
    exact far_sidePoint ℓ h₁.symm h₂ h₃ _ _
  · right
    by_cases hp₁ : p.x % 2 = 1 ∧ p.y % 2 = 1
    · exact absurd (same_of_inner (odd_inner p hp hp₁.1 hp₁.2) h) hne'
    by_cases hq₁ : q.x % 2 = 1 ∧ q.y % 2 = 1
    · exact absurd (same_of_inner (odd_inner q hq hq₁.1 hq₁.2) (T.same_symm _ _ h)).symm hne'
    omega

/-! ### Four distinct corners, four distinct edges -/

section Distinct

variable {patches : List Patch} {allArcs : List α} (hP : ∀ P, P ∈ patches)
  (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hT : T.NoAdjacentArc) (hne : ∀ σ, T.arcs σ ≠ [])
  (hbal : T.Balanced ℓ) (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P)
include hP hA hT hne hbal hwh

/-- **The four corners of a face are four distinct vertices.** -/
theorem corners_nodup {f : Face Patch} (hf : f ∈ T.faces patches ℓ) :
    ((corners f).map (T.vertexOf allArcs ℓ)).Nodup := by
  rw [mem_faces hP] at hf
  obtain ⟨P, i, j⟩ := f
  have hw := hwh P
  refine List.Nodup.map_on (fun p hp q hq hpq => ?_) (by simp [corners])
  by_contra hne'
  simp only [corners, List.mem_cons, List.not_mem_nil, or_false] at hp hq
  rcases hp with rfl | rfl | rfl | rfl <;> rcases hq with rfl | rfl | rfl | rfl <;>
    first
    | exact hne' rfl
    | have := far_or_coarse hA hT hne hbal hwh (by unfold InGrid; dsimp only at hf ⊢; omega)
        (vertexOf_eq_iff.mp hpq) rfl (by simp only [ne_eq, Point.mk.injEq]; omega)
      simp only [Far] at this hf; omega

/-- **The four edges of a face are four distinct edges.** -/
theorem faceEdges_nodup {f : Face Patch} (hf : f ∈ T.faces patches ℓ) :
    ((faceEdges f).map (T.edgeOf allArcs ℓ)).Nodup := by
  rw [mem_faces hP] at hf
  obtain ⟨P, i, j⟩ := f
  have hw := hwh P
  have hbal₂ := balanced_double hbal
  have hwh₂ : ∀ P, 0 < T.width (double ℓ) P ∧ 0 < T.height (double ℓ) P := fun P => by
    rw [width_double, height_double]; have := hwh P; omega
  refine List.Nodup.map_on (fun p hp q hq hpq => ?_) (by simp [faceEdges])
  by_contra hne'
  simp only [faceEdges, List.mem_cons, List.not_mem_nil, or_false] at hp hq
  rcases hp with rfl | rfl | rfl | rfl <;> rcases hq with rfl | rfl | rfl | rfl <;>
    first
    | exact hne' rfl
    | have := far_or_coarse hA hT hne hbal₂ hwh₂
        (by unfold InGrid midpoint; simp only [width_double, height_double] at hf ⊢; omega)
        (vertexOf_eq_iff.mp hpq) rfl (by simp only [midpoint, ne_eq, Point.mk.injEq]; omega)
      simp only [Far, midpoint, width_double, height_double] at this hf; omega

end Distinct

end TMesh

end TurnsOfATMesh
