/-
The quad mesh of a T-mesh at positive balanced whole lengths is a `QuadSurface`
(DT-173): its vertices, edges and faces are those of `Coarse.lean`, the classes of
the points of the fine grid with both coordinates even, one odd, both odd
(`QuadV`, `QuadE`, `QuadF`; a centre is alone in its class, so a face is its
centre). An edge joins the vertices beside its midpoint, the same for every
midpoint of its class (`edgeVerts_eq`); every edge lies in two slots of faces
(`two_slots`); the vertices and the faces are connected when the T-mesh is. The
Euler characteristic of this structure is `eulerQuad` (`card_quadSurface`), so
`bipartite_of_euler` applies as soon as `eulerQuad = 2` (`quad_bipartite`).
-/
module

public import TurnsOfATMesh.Coarse

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

open Classical

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### The cells -/

/-- The vertices of the quad mesh: classes of the points with both coordinates even. -/
abbrev QuadV (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) :=
  ↥(T.vertexOf allArcs ℓ '' {p | p ∈ T.points patches ℓ ∧ p.IsCoarse})

/-- The edges of the quad mesh: classes of the points with one odd coordinate. -/
abbrev QuadE (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) :=
  ↥(T.vertexOf allArcs ℓ '' {p | p ∈ T.points patches ℓ ∧ p.IsEdge})

/-- The faces of the quad mesh: their centres, the points with both coordinates odd. -/
abbrev QuadF (patches : List Patch) (ℓ : α → ℕ) :=
  ↥{c : Point Patch | c ∈ T.points patches ℓ ∧ c.IsCentre}

noncomputable instance (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) :
    Fintype (T.QuadV patches allArcs ℓ) :=
  (((T.points patches ℓ).finite_toSet.subset fun _ h => h.1).image _).fintype

noncomputable instance (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) :
    Fintype (T.QuadE patches allArcs ℓ) :=
  (((T.points patches ℓ).finite_toSet.subset fun _ h => h.1).image _).fintype

noncomputable instance (patches : List Patch) (ℓ : α → ℕ) : Fintype (T.QuadF patches ℓ) :=
  ((T.points patches ℓ).finite_toSet.subset fun _ h => h.1).fintype

variable {T}

theorem mem_points' {patches : List Patch} {ℓ : α → ℕ} {p : Point Patch} :
    p ∈ T.points patches ℓ ↔ p.patch ∈ patches ∧ T.InGrid ℓ p := by
  obtain ⟨P, x, y⟩ := p
  simp only [points, InGrid, List.mem_flatMap, List.mem_map, List.mem_range, Point.mk.injEq]
  constructor
  · rintro ⟨Q, hQ, x', hx, y', hy, rfl, rfl, rfl⟩; exact ⟨hQ, by omega, by omega⟩
  · rintro ⟨hP, hx, hy⟩; exact ⟨P, hP, x, by omega, y, by omega, rfl, rfl, rfl⟩

theorem edgeEnds_mem {patches : List Patch} {ℓ : α → ℕ} {m : Point Patch}
    (hm : m ∈ T.points patches ℓ ∧ m.IsEdge) :
    ((edgeEnds m).1 ∈ T.points patches ℓ ∧ (edgeEnds m).1.IsCoarse) ∧
      ((edgeEnds m).2 ∈ T.points patches ℓ ∧ (edgeEnds m).2.IsCoarse) := by
  obtain ⟨h₁, h₂⟩ := isCoarse_edgeEnds m hm.2
  obtain ⟨hP, hx, hy⟩ := mem_points'.mp hm.1
  have he := hm.2
  obtain ⟨P, x, y⟩ := m
  dsimp only [InGrid, Point.IsEdge] at hP hx hy he
  refine ⟨⟨?_, h₁⟩, ⟨?_, h₂⟩⟩ <;> rw [mem_points'] <;> simp only [edgeEnds] <;> split_ifs <;>
    exact ⟨hP, by dsimp only [InGrid]; omega⟩

theorem centre_mem {patches : List Patch} {ℓ : α → ℕ} {c : Point Patch}
    (hc : c ∈ T.points patches ℓ ∧ c.IsCentre) (i : Fin 4) :
    (centreEdge c i ∈ T.points patches ℓ ∧ (centreEdge c i).IsEdge) ∧
      (centreCorner c i ∈ T.points patches ℓ ∧ (centreCorner c i).IsCoarse) := by
  refine ⟨⟨?_, isEdge_centreEdge c hc.2 i⟩, ⟨?_, isCoarse_centreCorner c hc.2 i⟩⟩ <;>
  · obtain ⟨hP, hx, hy⟩ := mem_points'.mp hc.1
    obtain ⟨hx', hy'⟩ := hc.2
    obtain ⟨P, x, y⟩ := c
    simp only [mem_points', InGrid] at *
    fin_cases i <;> simp [centreEdge, centreCorner, hP] <;> omega

/-- A centre of a face is inside its patch. -/
theorem isInner_centre {patches : List Patch} {ℓ : α → ℕ} {c : Point Patch}
    (hc : c ∈ T.points patches ℓ ∧ c.IsCentre) : T.IsInner ℓ c := by
  obtain ⟨-, hx, hy⟩ := mem_points'.mp hc.1
  obtain ⟨hx', hy'⟩ := hc.2
  unfold IsInner; omega

variable (T)

/-- The vertex of a point with both coordinates even. -/
def QuadV.mk {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ} (p : Point Patch)
    (hp : p ∈ T.points patches ℓ ∧ p.IsCoarse) : T.QuadV patches allArcs ℓ :=
  ⟨T.vertexOf allArcs ℓ p, p, hp, rfl⟩

/-- The edge of a point with one odd coordinate. -/
def QuadE.mk {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ} (m : Point Patch)
    (hm : m ∈ T.points patches ℓ ∧ m.IsEdge) : T.QuadE patches allArcs ℓ :=
  ⟨T.vertexOf allArcs ℓ m, m, hm, rfl⟩

variable {T}

/-- A midpoint of an edge, chosen in its class. -/
noncomputable def QuadE.rep {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ}
    (e : T.QuadE patches allArcs ℓ) : Point Patch :=
  ((Set.mem_image _ _ _).mp e.2).choose

theorem QuadE.rep_spec {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ}
    (e : T.QuadE patches allArcs ℓ) :
    (e.rep ∈ T.points patches ℓ ∧ e.rep.IsEdge) ∧ T.vertexOf allArcs ℓ e.rep = e.1 :=
  ((Set.mem_image _ _ _).mp e.2).choose_spec

variable (T)

/-- The two vertices beside the midpoint `m`, as an unordered pair. -/
def edgeVerts (allArcs : List α) (ℓ : α → ℕ) (m : Point Patch) : Sym2 (T.Vertex allArcs ℓ) :=
  s(T.vertexOf allArcs ℓ (edgeEnds m).1, T.vertexOf allArcs ℓ (edgeEnds m).2)

/-- The two ends of an edge: the vertices beside its chosen midpoint. -/
noncomputable def quadEnds {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ}
    (e : T.QuadE patches allArcs ℓ) : T.QuadV patches allArcs ℓ × T.QuadV patches allArcs ℓ :=
  (QuadV.mk T _ (edgeEnds_mem e.rep_spec.1).1, QuadV.mk T _ (edgeEnds_mem e.rep_spec.1).2)

/-- The edge in the slot `i` of a face. -/
def quadFaceEdge {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ}
    (f : T.QuadF patches ℓ) (i : Fin 4) : T.QuadE patches allArcs ℓ :=
  QuadE.mk T _ (centre_mem f.2 i).1

/-- The corner `i` of a face. -/
def quadFaceCorner {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ}
    (f : T.QuadF patches ℓ) (i : Fin 4) : T.QuadV patches allArcs ℓ :=
  QuadV.mk T _ (centre_mem f.2 i).2

variable {T}

/-! ### The ends of an edge -/

theorem edgeEnds_sidePoint (ℓ : α → ℕ) (σ : Side Patch) {u : ℕ} (hu : u % 2 = 1) :
    edgeEnds (T.sidePoint ℓ σ u) = (T.sidePoint ℓ σ (u - 1), T.sidePoint ℓ σ (u + 1)) := by
  obtain ⟨P, j, k⟩ := σ
  fin_cases j <;> fin_cases k <;> simp [sidePoint, edgeEnds, hu]

theorem edgeEnds_arcPoint (ℓ : α → ℕ) (a : α) (i : Fin 2) {s : ℕ} (hs : s % 2 = 1) :
    edgeEnds (T.arcPoint ℓ a i s) = (T.arcPoint ℓ a i (s - 1), T.arcPoint ℓ a i (s + 1)) := by
  unfold arcPoint
  rw [edgeEnds_sidePoint ℓ _ (by omega)]
  congr 2; omega

/-- **The ends of an edge do not depend on its midpoint**: two midpoints of one class
have the same two vertices beside them. -/
theorem edgeVerts_eq {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {m m' : Point Patch} (hm : T.InGrid ℓ m)
    (he : m.IsEdge) (h : T.Same allArcs ℓ m m') : T.edgeVerts allArcs ℓ m' = T.edgeVerts allArcs ℓ m := by
  by_cases hi : T.IsInner ℓ m
  · rw [same_of_inner hi h]
  obtain ⟨a, i, s, hs, rfl⟩ := exists_arcPoint hne hbal hm hi
  have hs₁ : s % 2 = 1 := by unfold Point.IsEdge at he; rwa [arcPoint_parity] at he
  rcases same_of_arcInner' hA hbal hwh (by omega) (by omega) h with rfl | rfl
  · rfl
  have g : ∀ u, u ≤ 2 * ℓ a → T.vertexOf allArcs ℓ (T.arcPoint ℓ a (i + 1) (T.flip ℓ a u)) =
      T.vertexOf allArcs ℓ (T.arcPoint ℓ a i u) :=
    fun u hu => (vertexOf_eq_iff.mpr (same_partner hA i hu)).symm
  have hf : T.flip ℓ a s % 2 = 1 := by rw [flip_parity ℓ a hs]; exact hs₁
  unfold edgeVerts
  rw [edgeEnds_arcPoint ℓ a _ hs₁, edgeEnds_arcPoint ℓ a _ hf, ← g (s - 1) (by omega),
    ← g (s + 1) (by omega)]
  unfold flip
  split_ifs
  · rfl
  · dsimp only
    rw [Sym2.eq_swap]
    congr 3; omega

theorem val_quadEnds {patches : List Patch} {allArcs : List α} {ℓ : α → ℕ}
    (e : T.QuadE patches allArcs ℓ) :
    Sym2.map Subtype.val s((T.quadEnds e).1, (T.quadEnds e).2) = T.edgeVerts allArcs ℓ e.rep :=
  rfl

theorem sym2_val_inj {β : Type*} {S : Set β} {a b c d : S}
    (h : s(a.1, b.1) = s(c.1, d.1)) : s(a, b) = s(c, d) :=
  Sym2.map.injective Subtype.val_injective (by simpa using h)

section Ends

variable {patches : List Patch} {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
  (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
  (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P)
include hA hne hbal hwh

/-- The ends of an edge are the two vertices beside any midpoint of its class. -/
theorem quadEnds_eq (e : T.QuadE patches allArcs ℓ) {m : Point Patch}
    (hm : m ∈ T.points patches ℓ ∧ m.IsEdge) (hme : T.vertexOf allArcs ℓ m = e.1) :
    Sym2.map Subtype.val s((T.quadEnds e).1, (T.quadEnds e).2) = T.edgeVerts allArcs ℓ m := by
  rw [val_quadEnds]
  exact edgeVerts_eq hA hne hbal hwh (mem_points'.mp hm.1).2 hm.2
    (vertexOf_eq_iff.mp (hme.trans e.rep_spec.2.symm))

/-- **The edge `i` of a face joins its corners `i` and `i + 1`.** -/
theorem quadFaceEdge_ends (f : T.QuadF patches ℓ) (i : Fin 4) :
    s((T.quadEnds (T.quadFaceEdge (allArcs := allArcs) f i)).1,
        (T.quadEnds (T.quadFaceEdge (allArcs := allArcs) f i)).2) =
      s(T.quadFaceCorner f i, T.quadFaceCorner f (i + 1)) := by
  apply Sym2.map.injective Subtype.val_injective
  rw [quadEnds_eq hA hne hbal hwh _ (centre_mem f.2 i).1 rfl]
  unfold edgeVerts
  rcases edgeEnds_centreEdge f.1 f.2.2 i with h | h <;> rw [h]
  · rfl
  · exact Sym2.eq_swap

end Ends

/-! ### Every edge lies in exactly two slots -/

variable (T) in
/-- The slots `(f, k)` of the faces whose edge `k` has the midpoint `m`. -/
noncomputable def quadSlotsAt (patches : List Patch) (ℓ : α → ℕ) (m : Point Patch) :
    Finset (T.QuadF patches ℓ × Fin 4) :=
  Finset.univ.filter fun s => centreEdge s.1.1 s.2 = m

/-- A midpoint inside a patch is in two slots, a midpoint on a side in one. -/
theorem card_quadSlotsAt {patches : List Patch} (hP : ∀ P, P ∈ patches) {ℓ : α → ℕ}
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {m : Point Patch} (hm : T.InGrid ℓ m)
    (he : m.IsEdge) : (T.quadSlotsAt patches ℓ m).card = if T.IsInner ℓ m then 2 else 1 := by
  obtain ⟨P, X, Y⟩ := m
  have hw := hwh P
  simp only [InGrid, Point.IsEdge] at hm he
  have hext : ∀ t : Finset (T.QuadF patches ℓ × Fin 4), (∀ Q a b hc k,
      ((⟨⟨Q, a, b⟩, hc⟩, k) ∈ t ↔ centreEdge ⟨Q, a, b⟩ k = ⟨P, X, Y⟩)) →
      T.quadSlotsAt patches ℓ ⟨P, X, Y⟩ = t := by
    intro t ht
    ext ⟨⟨⟨Q, a, b⟩, hc⟩, k⟩
    rw [ht]
    simp [quadSlotsAt]
  have hmk : ∀ x y, x % 2 = 1 → y % 2 = 1 → x ≤ 2 * T.width ℓ P → y ≤ 2 * T.height ℓ P →
      (⟨P, x, y⟩ : Point Patch) ∈ {c | c ∈ T.points patches ℓ ∧ c.IsCentre} :=
    fun x y hx hy hx' hy' => ⟨(mem_points hP).mpr ⟨hx', hy'⟩, hx, hy⟩
  have hin : ∀ {x y : ℕ}, (0 < x ∧ x < 2 * T.width ℓ P ∧ 0 < y ∧ y < 2 * T.height ℓ P) →
      (if T.IsInner ℓ ⟨P, x, y⟩ then 2 else 1) = 2 := fun h => ite_eq_left_iff.mpr (absurd h)
  have hout : ∀ {x y : ℕ}, ¬ (0 < x ∧ x < 2 * T.width ℓ P ∧ 0 < y ∧ y < 2 * T.height ℓ P) →
      (if T.IsInner ℓ ⟨P, x, y⟩ then 2 else 1) = 1 := fun h => ite_eq_right_iff.mpr (absurd · h)
  rcases Nat.even_or_odd' X with ⟨i, rfl | rfl⟩ <;> rcases Nat.even_or_odd' Y with ⟨j, rfl | rfl⟩
  · omega
  · -- A vertical edge: slot 3 of the face on its right, slot 1 of the face on its left.
    by_cases h0 : i = 0
    · subst h0
      rw [hout (by omega), Finset.card_eq_one]
      refine ⟨(⟨⟨P, 1, 2 * j + 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩, 3),
        hext _ fun Q a b hc k => ?_⟩
      obtain ⟨hc₁, hx, hy⟩ := hc
      obtain ⟨ha, hb⟩ := (mem_points hP).mp hc₁
      dsimp only at hx hy ha hb
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;> simp [centreEdge, *] <;> omega
    by_cases h1 : i = T.width ℓ P
    · rw [hout (by omega), Finset.card_eq_one]
      refine ⟨(⟨⟨P, 2 * i - 1, 2 * j + 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩,
        1), hext _ fun Q a b hc k => ?_⟩
      obtain ⟨hc₁, hx, hy⟩ := hc
      obtain ⟨ha, hb⟩ := (mem_points hP).mp hc₁
      dsimp only at hx hy ha hb
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;> simp [centreEdge, *] <;> omega
    · rw [hin (by omega), Finset.card_eq_two]
      refine ⟨(⟨⟨P, 2 * i + 1, 2 * j + 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩,
        3), (⟨⟨P, 2 * i - 1, 2 * j + 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩,
        1), by simp, hext _ fun Q a b hc k => ?_⟩
      obtain ⟨hc₁, hx, hy⟩ := hc
      obtain ⟨ha, hb⟩ := (mem_points hP).mp hc₁
      dsimp only at hx hy ha hb
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;> simp [centreEdge, *] <;> omega
  · -- A horizontal edge: slot 0 of the face above it, slot 2 of the face below it.
    by_cases h0 : j = 0
    · subst h0
      rw [hout (by omega), Finset.card_eq_one]
      refine ⟨(⟨⟨P, 2 * i + 1, 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩, 0),
        hext _ fun Q a b hc k => ?_⟩
      obtain ⟨hc₁, hx, hy⟩ := hc
      obtain ⟨ha, hb⟩ := (mem_points hP).mp hc₁
      dsimp only at hx hy ha hb
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;> simp [centreEdge, *] <;> omega
    by_cases h1 : j = T.height ℓ P
    · rw [hout (by omega), Finset.card_eq_one]
      refine ⟨(⟨⟨P, 2 * i + 1, 2 * j - 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩,
        2), hext _ fun Q a b hc k => ?_⟩
      obtain ⟨hc₁, hx, hy⟩ := hc
      obtain ⟨ha, hb⟩ := (mem_points hP).mp hc₁
      dsimp only at hx hy ha hb
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;> simp [centreEdge, *] <;> omega
    · rw [hin (by omega), Finset.card_eq_two]
      refine ⟨(⟨⟨P, 2 * i + 1, 2 * j + 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩,
        0), (⟨⟨P, 2 * i + 1, 2 * j - 1⟩, hmk _ _ (by omega) (by omega) (by omega) (by omega)⟩,
        2), by simp, hext _ fun Q a b hc k => ?_⟩
      obtain ⟨hc₁, hx, hy⟩ := hc
      obtain ⟨ha, hb⟩ := (mem_points hP).mp hc₁
      dsimp only at hx hy ha hb
      rcases eq_or_ne Q P with rfl | hQ <;> fin_cases k <;> simp [centreEdge, *] <;> omega
  · omega

/-- **Every edge lies in exactly two slots of faces**: an edge inside a patch in the
two faces on either side, an edge on a side in one face of each of the two patches
the arc joins. -/
theorem quad_two_slots {patches : List Patch} {allArcs : List α} (hP : ∀ P, P ∈ patches)
    (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) (e : T.QuadE patches allArcs ℓ) :
    (Finset.univ.filter fun fi : T.QuadF patches ℓ × Fin 4 =>
      T.quadFaceEdge fi.1 fi.2 = e).card = 2 := by
  obtain ⟨⟨hm₀, he₀⟩, hme⟩ := e.rep_spec
  set m₀ := e.rep
  have hg₀ := (mem_points hP).mp hm₀
  -- The slots of the edge are the slots of the midpoints of its class.
  have hfilter : ∀ S : Point Patch → Prop, (∀ m, T.Same allArcs ℓ m m₀ ↔ S m) →
      (Finset.univ.filter fun fi : T.QuadF patches ℓ × Fin 4 => T.quadFaceEdge fi.1 fi.2 = e) =
        Finset.univ.filter fun fi => S (centreEdge fi.1.1 fi.2) :=
    fun S hS => Finset.filter_congr fun fi _ => by
      rw [Subtype.ext_iff, ← hme, ← hS, ← vertexOf_eq_iff]; rfl
  by_cases hi : T.IsInner ℓ m₀
  · rw [hfilter (· = m₀) fun m => ⟨fun h => same_of_inner hi (T.same_symm _ _ h),
      fun h => h ▸ Relation.EqvGen.refl _⟩]
    have c := card_quadSlotsAt hP hwh hg₀ he₀
    simp only [hi, ↓reduceIte] at c
    convert c using 2; ext; simp [quadSlotsAt]
  obtain ⟨a, i, s, hs, hm⟩ := exists_arcPoint hne hbal hg₀ hi
  have hs₁ : s % 2 = 1 := by have := he₀; unfold Point.IsEdge at this; rwa [hm, arcPoint_parity] at this
  have hs₂ : 0 < s ∧ s < 2 * ℓ a := by omega
  set m₁ := T.arcPoint ℓ a (i + 1) (T.flip ℓ a s)
  have h01 : m₀ ≠ m₁ := fun h => by
    have := (arcPoint_inj hbal hwh hs₂.1 hs₂.2 (flip_le hs) (hm.symm.trans h)).2.1
    fin_cases i <;> simp at this
  have hU : (Finset.univ.filter fun fi : T.QuadF patches ℓ × Fin 4 =>
      centreEdge fi.1.1 fi.2 = m₀ ∨ centreEdge fi.1.1 fi.2 = m₁) =
      T.quadSlotsAt patches ℓ m₀ ∪ T.quadSlotsAt patches ℓ m₁ := by
    ext x; simp [quadSlotsAt]
  have hdisj : Disjoint (T.quadSlotsAt patches ℓ m₀) (T.quadSlotsAt patches ℓ m₁) :=
    Finset.disjoint_left.mpr fun _ h₁ h₂ =>
      h01 ((Finset.mem_filter.mp h₁).2.symm.trans (Finset.mem_filter.mp h₂).2)
  have c := Finset.card_union_of_disjoint hdisj
  rw [card_quadSlotsAt hP hwh hg₀ he₀, card_quadSlotsAt hP hwh (m := m₁)
    (arcPoint_inGrid hbal _ _ (flip_le hs))
    (by unfold Point.IsEdge; rw [arcPoint_parity, flip_parity _ _ hs, hs₁]), ← hU] at c
  have hi₁ : ¬ T.IsInner ℓ m₁ := arcPoint_not_inner _ _ _ _
  simp only [hi, hi₁, ↓reduceIte] at c
  rw [hfilter (fun m => m = m₀ ∨ m = m₁) fun m => ⟨fun h => ?_, fun h => ?_⟩]
  · convert c using 2; ext; simp only [Finset.mem_filter]
  · rw [hm] at h ⊢
    exact same_of_arcInner' hA hbal hwh hs₂.1 hs₂.2 (T.same_symm _ _ h)
  · rcases h with rfl | rfl
    · exact Relation.EqvGen.refl _
    · rw [hm]; exact T.same_symm _ _ (same_partner hA i hs)

end TMesh

end TurnsOfATMesh
