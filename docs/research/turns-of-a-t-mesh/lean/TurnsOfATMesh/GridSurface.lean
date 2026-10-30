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
public import Mathlib.Data.Sym.Sym2

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

/-! ### The ends of an edge -/

/-- The lower end of the unit edge of the grid at `ℓ` whose midpoint, in the grid at
`double ℓ`, is `m`. -/
def lowEnd (m : Point Patch) : Point Patch := ⟨m.patch, m.x / 2, m.y / 2⟩

/-- The upper end of the unit edge whose midpoint is `m`. -/
def highEnd (m : Point Patch) : Point Patch := ⟨m.patch, (m.x + 1) / 2, (m.y + 1) / 2⟩

/-- The ends of a face edge are the two ends of its midpoint. -/
theorem ends_midpoint {f : Face Patch} {e : Point Patch × Point Patch} (he : e ∈ faceEdges f) :
    s(e.1, e.2) = s(lowEnd (midpoint e), highEnd (midpoint e)) := by
  obtain ⟨P, i, j⟩ := f
  simp only [faceEdges, List.mem_cons, List.not_mem_nil, or_false] at he
  rcases he with rfl | rfl | rfl | rfl <;>
    simp only [lowEnd, highEnd, midpoint, Sym2.eq_iff, Point.mk.injEq, true_and] <;> omega

theorem lowEnd_sidePoint (ℓ : α → ℕ) (σ : Side Patch) (u : ℕ) :
    lowEnd (T.sidePoint (double ℓ) σ u) = T.sidePoint ℓ σ (u / 2) ∧
      highEnd (T.sidePoint (double ℓ) σ u) = T.sidePoint ℓ σ ((u + 1) / 2) := by
  obtain ⟨P, j, k⟩ := σ
  fin_cases j <;> fin_cases k <;>
    simp only [sidePoint, lowEnd, highEnd, width_double, height_double] <;>
    simp <;> omega

theorem lowEnd_arcPoint (ℓ : α → ℕ) (a : α) (i : Fin 2) (s : ℕ) :
    lowEnd (T.arcPoint (double ℓ) a i s) = T.arcPoint ℓ a i (s / 2) ∧
      highEnd (T.arcPoint (double ℓ) a i s) = T.arcPoint ℓ a i ((s + 1) / 2) := by
  unfold arcPoint
  rw [posNat_double, (lowEnd_sidePoint ℓ _ _).1, (lowEnd_sidePoint ℓ _ _).2]
  constructor <;> congr 1 <;> omega

variable (T) in
/-- The two ends of the edge whose midpoint is `m`, as vertices of the quad mesh. -/
def endsOf (allArcs : List α) (ℓ : α → ℕ) (m : Point Patch) : Sym2 (T.Vertex allArcs ℓ) :=
  s(T.vertexOf allArcs ℓ (lowEnd m), T.vertexOf allArcs ℓ (highEnd m))

/-- A function that agrees on the two points of every gluing pair agrees on a vertex. -/
theorem eq_of_same {allArcs : List α} {ℓ : α → ℕ} {β : Type*} {g : Point Patch → β}
    (hg : ∀ p q, (p, q) ∈ T.gluings allArcs ℓ → g p = g q) {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : g p = g q := by
  induction h with
  | rel x y hxy => exact hg x y hxy
  | refl => rfl
  | symm x y _ ih => exact ih.symm
  | trans x y z _ _ ih₁ ih₂ => exact ih₁.trans ih₂

/-- The two ends of an edge across an arc are glued to the two ends on the other side. -/
theorem endsOf_glued {allArcs : List α} (hA : ∀ a, a ∈ allArcs) (ℓ : α → ℕ) (a : α) {t : ℕ}
    (ht : t ≤ 2 * double ℓ a) :
    T.endsOf allArcs ℓ (T.arcPoint (double ℓ) a 0 t) =
      T.endsOf allArcs ℓ (T.arcPoint (double ℓ) a 1 (T.flip (double ℓ) a t)) := by
  simp only [double] at ht
  have g : ∀ u, u ≤ 2 * ℓ a →
      T.vertexOf allArcs ℓ (T.arcPoint ℓ a 0 u) = T.vertexOf allArcs ℓ (T.arcPoint ℓ a 1 (T.flip ℓ a u)) := by
    intro u hu
    have := T.same_of_glued allArcs ℓ a (hA a) u hu
    rw [glued_eq ℓ a hu] at this
    exact vertexOf_eq_iff.mpr this
  unfold endsOf
  rw [(lowEnd_arcPoint ℓ a 0 t).1, (lowEnd_arcPoint ℓ a 0 t).2, (lowEnd_arcPoint ℓ a 1 _).1,
    (lowEnd_arcPoint ℓ a 1 _).2, g (t / 2) (by omega), g ((t + 1) / 2) (by omega)]
  unfold flip double
  split_ifs
  · rfl
  · rw [Sym2.eq_swap]
    congr 3 <;> omega

/-- **An edge determines its two ends**: two face edges that are one edge of the quad
mesh join the same two vertices. -/
theorem ends_eq_of_edgeOf_eq {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    {f f' : Face Patch} {e e' : Point Patch × Point Patch} (he : e ∈ faceEdges f)
    (he' : e' ∈ faceEdges f') (h : T.edgeOf allArcs ℓ e = T.edgeOf allArcs ℓ e') :
    s(T.vertexOf allArcs ℓ e.1, T.vertexOf allArcs ℓ e.2) =
      s(T.vertexOf allArcs ℓ e'.1, T.vertexOf allArcs ℓ e'.2) := by
  have hmap : ∀ {f : Face Patch} {e : Point Patch × Point Patch}, e ∈ faceEdges f →
      s(T.vertexOf allArcs ℓ e.1, T.vertexOf allArcs ℓ e.2) = T.endsOf allArcs ℓ (midpoint e) :=
    fun he => by rw [endsOf, ← Sym2.map_mk, ← Sym2.map_mk, ends_midpoint he]
  rw [hmap he, hmap he']
  refine eq_of_same (fun p q hpq => ?_) (vertexOf_eq_iff.mp h)
  obtain ⟨a, t, ht, rfl, rfl⟩ := mem_gluings hpq
  exact endsOf_glued hA ℓ a ht

/-! ### Every edge lies in exactly two slots -/

/-- The edge in the slot `i` of a face. -/
def faceEdge (f : Face Patch) (i : Fin 4) : Point Patch × Point Patch :=
  (faceEdges f)[i.val]'(by simp [faceEdges])

theorem faceEdge_mem (f : Face Patch) (i : Fin 4) : faceEdge f i ∈ faceEdges f :=
  List.getElem_mem _

instance (ℓ : α → ℕ) (p : Point Patch) : Decidable (T.IsInner ℓ p) := by
  unfold IsInner; infer_instance

variable (T) in
/-- The slots `(f, i)` of the faces whose edge has the midpoint `m`. -/
def slotsAt (patches : List Patch) (ℓ : α → ℕ) (m : Point Patch) : Finset (Face Patch × Fin 4) :=
  ((T.faces patches ℓ).toFinset ×ˢ Finset.univ).filter fun s => midpoint (faceEdge s.1 s.2) = m

/-- The two points of a gluing pair are one vertex, on either side. -/
theorem same_partner {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} {a : α} (i : Fin 2)
    {s : ℕ} (hs : s ≤ 2 * ℓ a) :
    T.Same allArcs ℓ (T.arcPoint ℓ a i s) (T.arcPoint ℓ a (i + 1) (T.flip ℓ a s)) := by
  fin_cases i
  · have := T.same_of_glued allArcs ℓ a (hA a) s hs
    rwa [glued_eq ℓ a hs] at this
  · have := T.same_of_glued allArcs ℓ a (hA a) _ (flip_le (T := T) hs)
    rw [glued_eq ℓ a (flip_le hs), flip_flip hs] at this
    exact T.same_symm _ _ this

section Slots

variable {patches : List Patch} (hP : ∀ P, P ∈ patches) {ℓ : α → ℕ}
  (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P)
include hP hwh

/-- A midpoint inside a patch is in two slots, a midpoint on a side in one. -/
theorem card_slotsAt {m : Point Patch} (hm : T.InGrid (double ℓ) m) (hodd : (m.x + m.y) % 2 = 1) :
    (T.slotsAt patches ℓ m).card = if T.IsInner (double ℓ) m then 2 else 1 := by
  obtain ⟨P, X, Y⟩ := m
  have hw := hwh P
  simp only [InGrid, width_double, height_double] at hm hodd
  have hext : ∀ t : Finset (Face Patch × Fin 4), (∀ Q a b k, (⟨Q, a, b⟩, k) ∈ t ↔
      (a < 2 * T.width ℓ Q ∧ b < 2 * T.height ℓ Q) ∧
        midpoint (faceEdge ⟨Q, a, b⟩ k) = ⟨P, X, Y⟩) → T.slotsAt patches ℓ ⟨P, X, Y⟩ = t := by
    intro t ht
    ext ⟨⟨Q, a, b⟩, k⟩
    rw [ht]
    simp [slotsAt, mem_faces hP]
  have if_side : ∀ {x y : ℕ}, ¬ (0 < x ∧ x < 2 * (2 * T.width ℓ P) ∧ 0 < y ∧ y < 2 * (2 * T.height ℓ P)) →
      (if T.IsInner (double ℓ) ⟨P, x, y⟩ then 2 else 1) = 1 := fun h => by
    simp only [IsInner, width_double, height_double]; simp [h]
  have if_inner : ∀ {x y : ℕ}, (0 < x ∧ x < 2 * (2 * T.width ℓ P) ∧ 0 < y ∧ y < 2 * (2 * T.height ℓ P)) →
      (if T.IsInner (double ℓ) ⟨P, x, y⟩ then 2 else 1) = 2 := fun h => by
    simp only [IsInner, width_double, height_double]; simp [h]
  rcases Nat.even_or_odd' X with ⟨i, rfl | rfl⟩ <;> rcases Nat.even_or_odd' Y with ⟨j, rfl | rfl⟩
  · omega
  · -- A vertical edge: slot 1 of the face on its left, slot 3 of the face on its right.
    by_cases h0 : i = 0
    · subst h0
      rw [if_side (by omega), Finset.card_eq_one]
      refine ⟨(⟨P, 0, j⟩, 3), hext _ fun Q a b k => ?_⟩
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;>
        (simp [faceEdge, faceEdges, midpoint, *]; try omega)
    by_cases h1 : i = 2 * T.width ℓ P
    · rw [if_side (by omega), Finset.card_eq_one]
      refine ⟨(⟨P, i - 1, j⟩, 1), hext _ fun Q a b k => ?_⟩
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;>
        simp [faceEdge, faceEdges, midpoint, *] <;> omega
    · rw [if_inner (by omega), Finset.card_eq_two]
      refine ⟨(⟨P, i - 1, j⟩, 1), (⟨P, i, j⟩, 3), by simp, hext _ fun Q a b k => ?_⟩
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;>
        simp [faceEdge, faceEdges, midpoint, *] <;> omega
  · -- A horizontal edge: slot 0 of the face above it, slot 2 of the face below it.
    by_cases h0 : j = 0
    · subst h0
      rw [if_side (by omega), Finset.card_eq_one]
      refine ⟨(⟨P, i, 0⟩, 0), hext _ fun Q a b k => ?_⟩
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;>
        (simp [faceEdge, faceEdges, midpoint, *]; try omega)
    by_cases h1 : j = 2 * T.height ℓ P
    · rw [if_side (by omega), Finset.card_eq_one]
      refine ⟨(⟨P, i, j - 1⟩, 2), hext _ fun Q a b k => ?_⟩
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;>
        simp [faceEdge, faceEdges, midpoint, *] <;> omega
    · rw [if_inner (by omega), Finset.card_eq_two]
      refine ⟨(⟨P, i, j⟩, 0), (⟨P, i, j - 1⟩, 2), by simp, hext _ fun Q a b k => ?_⟩
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;>
        simp [faceEdge, faceEdges, midpoint, *] <;> omega
  · omega

end Slots

open Classical in
/-- **Every edge lies in exactly two slots of faces**: an edge inside a patch in the
two faces on either side, an edge on a side in one face of each of the two patches
the arc joins. -/
theorem card_slots {patches : List Patch} {allArcs : List α} (hP : ∀ P, P ∈ patches)
    (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {f₀ : Face Patch}
    {e₀ : Point Patch × Point Patch} (hf₀ : f₀ ∈ T.faces patches ℓ) (he₀ : e₀ ∈ faceEdges f₀) :
    (((T.faces patches ℓ).toFinset ×ˢ Finset.univ).filter fun s : Face Patch × Fin 4 =>
      T.edgeOf allArcs ℓ (faceEdge s.1 s.2) = T.edgeOf allArcs ℓ e₀).card = 2 := by
  have hbal₂ := balanced_double hbal
  have hwh₂ : ∀ P, 0 < T.width (double ℓ) P ∧ 0 < T.height (double ℓ) P := fun P => by
    rw [width_double, height_double]; have := hwh P; omega
  set m₀ := midpoint e₀
  have hm₀ : T.InGrid (double ℓ) m₀ ∧ (m₀.x + m₀.y) % 2 = 1 := by
    have : m₀ ∈ midpoint '' {e | e ∈ (T.faces patches ℓ).flatMap faceEdges} :=
      ⟨e₀, List.mem_flatMap.mpr ⟨f₀, hf₀, he₀⟩, rfl⟩
    rwa [midpoint_edges hP hwh] at this
  -- The slots of the edge are the slots of the midpoints of its class.
  have hfilter : ∀ S : Point Patch → Prop, (∀ m, T.Same allArcs (double ℓ) m m₀ ↔ S m) →
      (((T.faces patches ℓ).toFinset ×ˢ Finset.univ).filter fun s : Face Patch × Fin 4 =>
        T.edgeOf allArcs ℓ (faceEdge s.1 s.2) = T.edgeOf allArcs ℓ e₀) =
      ((T.faces patches ℓ).toFinset ×ˢ Finset.univ).filter fun s => S (midpoint (faceEdge s.1 s.2)) :=
    fun S hS => Finset.filter_congr fun s _ => by rw [edgeOf, edgeOf, vertexOf_eq_iff, hS]
  by_cases hi : T.IsInner (double ℓ) m₀
  · rw [hfilter (· = m₀) fun m => ⟨fun h => same_of_inner hi (T.same_symm _ _ h),
      fun h => h ▸ Relation.EqvGen.refl _⟩]
    have hU : (((T.faces patches ℓ).toFinset ×ˢ Finset.univ).filter fun s : Face Patch × Fin 4 =>
        midpoint (faceEdge s.1 s.2) = m₀) = T.slotsAt patches ℓ m₀ := by
      ext x; simp [slotsAt]
    have c := card_slotsAt hP hwh hm₀.1 hm₀.2
    simp only [hi, ↓reduceIte] at c
    rw [← hU] at c
    convert c using 2; ext; simp only [Finset.mem_filter]
  obtain ⟨a, i, s, hs, hm⟩ := exists_arcPoint hne hbal₂ hm₀.1 hi
  have hs₁ : s % 2 = 1 := by have := hm₀.2; rwa [hm, arcPoint_parity] at this
  have hs₂ : 0 < s ∧ s < 2 * double ℓ a := by omega
  set m₁ := T.arcPoint (double ℓ) a (i + 1) (T.flip (double ℓ) a s)
  have h01 : m₀ ≠ m₁ := fun h => by
    have := (arcPoint_inj hbal₂ hwh₂ hs₂.1 hs₂.2 (flip_le hs) (hm.symm.trans h)).2.1
    fin_cases i <;> simp at this
  have hU : (((T.faces patches ℓ).toFinset ×ˢ Finset.univ).filter fun s : Face Patch × Fin 4 =>
      midpoint (faceEdge s.1 s.2) = m₀ ∨ midpoint (faceEdge s.1 s.2) = m₁) =
      T.slotsAt patches ℓ m₀ ∪ T.slotsAt patches ℓ m₁ := by
    ext x; simp [slotsAt, and_or_left]
  have hdisj : Disjoint (T.slotsAt patches ℓ m₀) (T.slotsAt patches ℓ m₁) :=
    Finset.disjoint_left.mpr fun _ h₁ h₂ =>
      h01 ((Finset.mem_filter.mp h₁).2.symm.trans (Finset.mem_filter.mp h₂).2)
  have c := Finset.card_union_of_disjoint hdisj
  rw [card_slotsAt hP hwh hm₀.1 hm₀.2, card_slotsAt hP hwh (m := m₁)
    (arcPoint_inGrid hbal₂ _ _ (flip_le hs)) (by rw [arcPoint_parity, flip_parity _ _ hs, hs₁]),
    ← hU] at c
  have hi₁ : ¬ T.IsInner (double ℓ) m₁ := arcPoint_not_inner _ _ _ _
  simp only [hi, hi₁, ↓reduceIte] at c
  rw [hfilter (fun m => m = m₀ ∨ m = m₁) fun m => ⟨fun h => ?_, fun h => ?_⟩]
  · convert c using 2; ext; simp only [Finset.mem_filter]
  · rw [hm] at h ⊢
    exact same_of_arcInner' hA hbal₂ hwh₂ hs₂.1 hs₂.2 (T.same_symm _ _ h)
  · rcases h with rfl | rfl
    · exact Relation.EqvGen.refl _
    · rw [hm]; exact T.same_symm _ _ (same_partner hA i hs)

end TMesh

end TurnsOfATMesh
