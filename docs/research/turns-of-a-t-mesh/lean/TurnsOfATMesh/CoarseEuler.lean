/-
The Euler characteristic of the quad mesh of `Coarse.lean` is `P − A + N`
(`eulerQuad_eq_nodeCount`), as `GridEuler.euler_eq_nodeCount` proves for the
fine grid. The vertices, edges and faces of the quad mesh are the classes of
the points of the fine grid with both coordinates even, one odd, both odd; the
gluings keep each kind (`isCoarse_of_same`, `isEdge_of_same`,
`isCentre_of_same`), so `ncard_classes` counts each kind apart: per patch of
`w × h` quads, `(w − 1)(h − 1)` inner vertices, `w(h − 1) + (w − 1)h` inner
edges, `wh` faces; per arc of length `ℓ_a`, `ℓ_a − 1` vertices and `ℓ_a` edges
inside it; the ends of the arcs are all vertices. A patch adds `+1`, an arc
`−1`, a node `+1`.
-/
module

public import TurnsOfATMesh.Coarse

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] {T : TMesh Patch α}

/-! ### The kind of a point of a side or of an arc -/

theorem sidePoint_isCoarse_iff (ℓ : α → ℕ) (σ : Side Patch) (u : ℕ) :
    (T.sidePoint ℓ σ u).IsCoarse ↔ u % 2 = 0 := by
  unfold sidePoint Point.IsCoarse; split_ifs <;> simp

theorem sidePoint_not_isCentre (ℓ : α → ℕ) (σ : Side Patch) (u : ℕ) :
    ¬ (T.sidePoint ℓ σ u).IsCentre := by
  unfold sidePoint Point.IsCentre; split_ifs <;> simp

theorem arcPoint_isCoarse_iff (ℓ : α → ℕ) (a : α) (i : Fin 2) (s : ℕ) :
    (T.arcPoint ℓ a i s).IsCoarse ↔ s % 2 = 0 := by
  rw [arcPoint, sidePoint_isCoarse_iff]; omega

theorem arcPoint_isEdge_iff (ℓ : α → ℕ) (a : α) (i : Fin 2) (s : ℕ) :
    (T.arcPoint ℓ a i s).IsEdge ↔ s % 2 = 1 := by
  rw [Point.IsEdge, arcPoint_parity]

theorem isCoarse_of_isEnd {ℓ : α → ℕ} {p : Point Patch} (h : T.IsEnd ℓ p) : p.IsCoarse := by
  obtain ⟨a, i, rfl | rfl⟩ := h <;> simp [arcPoint_isCoarse_iff]

/-! ### The gluings keep the kind of a point -/

theorem isCoarse_of_same {allArcs : List α} {ℓ : α → ℕ} {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : p.IsCoarse ↔ q.IsCoarse := by
  refine same_iff_of_gluings (fun x y hxy => ?_) h
  obtain ⟨b, t, ht, rfl, rfl⟩ := mem_gluings hxy
  rw [arcPoint_isCoarse_iff, arcPoint_isCoarse_iff, flip_parity ℓ b ht]

theorem isEdge_of_same {allArcs : List α} {ℓ : α → ℕ} {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : p.IsEdge ↔ q.IsEdge := by
  unfold Point.IsEdge; rw [parity_of_same h]

theorem isCentre_of_same {allArcs : List α} {ℓ : α → ℕ} {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : p.IsCentre ↔ q.IsCentre := by
  refine same_iff_of_gluings (fun x y hxy => ?_) h
  obtain ⟨b, t, ht, rfl, rfl⟩ := mem_gluings hxy
  exact iff_of_false (sidePoint_not_isCentre _ _ _) (sidePoint_not_isCentre _ _ _)

/-! ### Counting points -/

/-- The points of `(0, 2m) × (0, 2n)` with both coordinates even. -/
theorem card_even_even (m n : ℕ) :
    ((Finset.Ioo 0 (2 * m)).filter (· % 2 = 0) ×ˢ (Finset.Ioo 0 (2 * n)).filter (· % 2 = 0)).card =
      (m - 1) * (n - 1) := by
  rw [Finset.card_product, card_filter_even, card_filter_even]

/-- The points of `(0, 2m) × (0, 2n)` with both coordinates odd. -/
theorem card_odd_odd (m n : ℕ) :
    ((Finset.Ioo 0 (2 * m)).filter (· % 2 = 1) ×ˢ (Finset.Ioo 0 (2 * n)).filter (· % 2 = 1)).card =
      m * n := by
  rw [Finset.card_product, card_filter_odd, card_filter_odd]

/-! ### The vertices, edges and faces of the quad mesh -/

section Count

variable [Fintype Patch] [Fintype α] {patches : List Patch} {allArcs : List α}
  (hP : ∀ P, P ∈ patches) (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hne : ∀ σ, T.arcs σ ≠ [])
  (hbal : T.Balanced ℓ) (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P)
include hP hA hne hbal hwh

theorem numQuadVertices_eq : T.numQuadVertices patches allArcs ℓ =
    ∑ P, (T.width ℓ P - 1) * (T.height ℓ P - 1) + ∑ a, (ℓ a - 1) + T.numNodes allArcs ℓ := by
  unfold numQuadVertices
  rw [ncard_classes (S := {p | p ∈ T.points patches ℓ ∧ p.IsCoarse}) hA hne hbal hwh
    ((T.points patches ℓ).finite_toSet.subset fun p hp => hp.1)
    (fun p hp => (mem_points hP).mp hp.1)
    (fun p q hp h => ⟨(mem_points hP).mpr ((inGrid_of_same hbal h).mp ((mem_points hP).mp hp.1)),
      (isCoarse_of_same h).mp hp.2⟩)]
  have hI : {p | p ∈ T.points patches ℓ ∧ p.IsCoarse} ∩ {p | T.IsInner ℓ p} =
      ↑(ptsOf fun P => (Finset.Ioo 0 (2 * T.width ℓ P)).filter (· % 2 = 0) ×ˢ
        (Finset.Ioo 0 (2 * T.height ℓ P)).filter (· % 2 = 0)) := by
    ext p
    simp only [Set.mem_inter_iff, Set.mem_ofPred_eq, mem_points hP, Finset.mem_coe, mem_ptsOf,
      Finset.mem_product, Finset.mem_filter, Finset.mem_Ioo, InGrid, IsInner, Point.IsCoarse]
    constructor <;> intro <;> omega
  have hX : {x : (_ : α) × ℕ | 0 < x.2 ∧ x.2 < 2 * ℓ x.1 ∧
      T.arcPoint ℓ x.1 0 x.2 ∈ {p | p ∈ T.points patches ℓ ∧ p.IsCoarse}} =
      ↑(Finset.univ.sigma fun a => (Finset.Ioo 0 (2 * ℓ a)).filter (· % 2 = 0)) := by
    ext ⟨a, t⟩
    simp only [Set.mem_ofPred_eq, mem_points hP, arcPoint_isCoarse_iff, Finset.coe_sigma,
      Set.mem_sigma_iff, Finset.coe_univ, Set.mem_univ, Finset.coe_filter, Finset.mem_Ioo,
      true_and]
    exact ⟨fun h => ⟨⟨h.1, h.2.1⟩, h.2.2.2⟩,
      fun h => ⟨h.1.1, h.1.2, arcPoint_inGrid hbal a 0 h.1.2.le, h.2⟩⟩
  have hE : {p | p ∈ T.points patches ℓ ∧ p.IsCoarse} ∩ {p | T.IsEnd ℓ p} = {p | T.IsEnd ℓ p} := by
    refine Set.inter_eq_right.mpr fun p hp => ⟨?_, isCoarse_of_isEnd hp⟩
    obtain ⟨a, i, rfl | rfl⟩ := hp <;> rw [mem_points hP]
    exacts [arcPoint_inGrid hbal a i (Nat.zero_le _), arcPoint_inGrid hbal a i le_rfl]
  rw [hI, hX, hE, Set.ncard_coe_finset, Set.ncard_coe_finset, card_ptsOf, Finset.card_sigma]
  simp only [card_even_even, card_filter_even, numNodes]

theorem numQuadEdges_eq : T.numQuadEdges patches allArcs ℓ =
    ∑ P, (T.width ℓ P * (T.height ℓ P - 1) + (T.width ℓ P - 1) * T.height ℓ P) + ∑ a, ℓ a := by
  unfold numQuadEdges
  rw [ncard_classes (S := {p | p ∈ T.points patches ℓ ∧ p.IsEdge}) hA hne hbal hwh
    ((T.points patches ℓ).finite_toSet.subset fun p hp => hp.1)
    (fun p hp => (mem_points hP).mp hp.1)
    (fun p q hp h => ⟨(mem_points hP).mpr ((inGrid_of_same hbal h).mp ((mem_points hP).mp hp.1)),
      (isEdge_of_same h).mp hp.2⟩)]
  have hI : {p | p ∈ T.points patches ℓ ∧ p.IsEdge} ∩ {p | T.IsInner ℓ p} =
      ↑(ptsOf fun P => (Finset.Ioo 0 (2 * T.width ℓ P) ×ˢ
        Finset.Ioo 0 (2 * T.height ℓ P)).filter fun q => (q.1 + q.2) % 2 = 1) := by
    ext p
    simp only [Set.mem_inter_iff, Set.mem_ofPred_eq, mem_points hP, Finset.mem_coe, mem_ptsOf,
      Finset.mem_product, Finset.mem_filter, Finset.mem_Ioo, InGrid, IsInner, Point.IsEdge]
    constructor <;> intro <;> omega
  have hX : {x : (_ : α) × ℕ | 0 < x.2 ∧ x.2 < 2 * ℓ x.1 ∧
      T.arcPoint ℓ x.1 0 x.2 ∈ {p | p ∈ T.points patches ℓ ∧ p.IsEdge}} =
      ↑(Finset.univ.sigma fun a => (Finset.Ioo 0 (2 * ℓ a)).filter (· % 2 = 1)) := by
    ext ⟨a, t⟩
    simp only [Set.mem_ofPred_eq, mem_points hP, arcPoint_isEdge_iff, Finset.coe_sigma,
      Set.mem_sigma_iff, Finset.coe_univ, Set.mem_univ, Finset.coe_filter, Finset.mem_Ioo,
      true_and]
    exact ⟨fun h => ⟨⟨h.1, h.2.1⟩, h.2.2.2⟩,
      fun h => ⟨h.1.1, h.1.2, arcPoint_inGrid hbal a 0 h.1.2.le, h.2⟩⟩
  have hE : {p | p ∈ T.points patches ℓ ∧ p.IsEdge} ∩ {p | T.IsEnd ℓ p} = ∅ := by
    rw [Set.eq_empty_iff_forall_notMem]
    rintro p ⟨⟨-, hp⟩, hpe⟩
    have := isCoarse_of_isEnd hpe
    unfold Point.IsEdge at hp; unfold Point.IsCoarse at this; omega
  rw [hI, hX, hE, Set.image_empty, Set.ncard_empty, Set.ncard_coe_finset, Set.ncard_coe_finset,
    card_ptsOf, Finset.card_sigma]
  simp only [card_filter_odd_sum, card_filter_odd, add_zero]

theorem numQuadFaces_eq : T.numQuadFaces patches allArcs ℓ = ∑ P, T.width ℓ P * T.height ℓ P := by
  unfold numQuadFaces
  rw [ncard_classes (S := {p | p ∈ T.points patches ℓ ∧ p.IsCentre}) hA hne hbal hwh
    ((T.points patches ℓ).finite_toSet.subset fun p hp => hp.1)
    (fun p hp => (mem_points hP).mp hp.1)
    (fun p q hp h => ⟨(mem_points hP).mpr ((inGrid_of_same hbal h).mp ((mem_points hP).mp hp.1)),
      (isCentre_of_same h).mp hp.2⟩)]
  have hI : {p | p ∈ T.points patches ℓ ∧ p.IsCentre} ∩ {p | T.IsInner ℓ p} =
      ↑(ptsOf fun P => (Finset.Ioo 0 (2 * T.width ℓ P)).filter (· % 2 = 1) ×ˢ
        (Finset.Ioo 0 (2 * T.height ℓ P)).filter (· % 2 = 1)) := by
    ext p
    simp only [Set.mem_inter_iff, Set.mem_ofPred_eq, mem_points hP, Finset.mem_coe, mem_ptsOf,
      Finset.mem_product, Finset.mem_filter, Finset.mem_Ioo, InGrid, IsInner, Point.IsCentre]
    constructor <;> intro <;> omega
  have hX : {x : (_ : α) × ℕ | 0 < x.2 ∧ x.2 < 2 * ℓ x.1 ∧
      T.arcPoint ℓ x.1 0 x.2 ∈ {p | p ∈ T.points patches ℓ ∧ p.IsCentre}} = ∅ := by
    rw [Set.eq_empty_iff_forall_notMem]
    rintro x ⟨-, -, -, hc⟩
    exact sidePoint_not_isCentre _ _ _ hc
  have hE : {p | p ∈ T.points patches ℓ ∧ p.IsCentre} ∩ {p | T.IsEnd ℓ p} = ∅ := by
    rw [Set.eq_empty_iff_forall_notMem]
    rintro p ⟨⟨-, hp⟩, hpe⟩
    have := isCoarse_of_isEnd hpe
    unfold Point.IsCentre at hp; unfold Point.IsCoarse at this; omega
  rw [hI, hX, hE, Set.image_empty, Set.ncard_empty, Set.ncard_empty, Set.ncard_coe_finset,
    card_ptsOf]
  simp only [card_odd_odd, add_zero]

end Count

/-- **The Euler characteristic of the quad mesh** of a T-mesh at positive
balanced lengths, every side holding an arc, is `P − A + N`: the patches minus
the arcs plus the nodes. -/
theorem eulerQuad_eq_nodeCount [Fintype Patch] [Fintype α] {patches : List Patch}
    {allArcs : List α} (hP : ∀ P, P ∈ patches) (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hℓ : ∀ a, 0 < ℓ a) (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ) :
    T.eulerQuad patches allArcs ℓ = (Fintype.card Patch : ℤ) - Fintype.card α + T.nodeCount := by
  have hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P :=
    fun P => ⟨sideLen_pos hℓ hne _, sideLen_pos hℓ hne _⟩
  have cw : ∀ P, ((T.width ℓ P - 1 : ℕ) : ℤ) = T.width ℓ P - 1 :=
    fun P => by have := (hwh P).1; omega
  have ch : ∀ P, ((T.height ℓ P - 1 : ℕ) : ℤ) = T.height ℓ P - 1 :=
    fun P => by have := (hwh P).2; omega
  have cl : ∀ a, ((ℓ a - 1 : ℕ) : ℤ) = ℓ a - 1 := fun a => by have := hℓ a; omega
  unfold eulerQuad
  rw [numQuadVertices_eq hP hA hne hbal hwh, numQuadEdges_eq hP hA hne hbal hwh,
    numQuadFaces_eq hP hA hne hbal hwh, numNodes_eq hA hℓ hne hbal]
  push_cast [cw, ch, cl]
  have hcard : (Fintype.card Patch : ℤ) = ∑ P : Patch,
      (((T.width ℓ P : ℤ) - 1) * (T.height ℓ P - 1) -
        (T.width ℓ P * (T.height ℓ P - 1) + (T.width ℓ P - 1) * T.height ℓ P) +
        T.width ℓ P * T.height ℓ P) := by
    rw [Finset.sum_congr rfl (g := fun _ => (1 : ℤ)) fun P _ => by ring]; simp
  have hcardα : (Fintype.card α : ℤ) = ∑ a : α, ((ℓ a : ℤ) - (ℓ a - 1)) := by
    rw [Finset.sum_congr rfl (g := fun _ => (1 : ℤ)) fun a _ => by ring]; simp
  rw [hcard, hcardα]
  simp only [Finset.sum_add_distrib, Finset.sum_sub_distrib]
  ring

end TMesh

end TurnsOfATMesh
