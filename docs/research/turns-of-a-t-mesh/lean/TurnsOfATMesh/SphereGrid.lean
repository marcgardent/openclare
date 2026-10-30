/-
The last step of DT-173: the quad graph of a T-mesh of a sphere is bipartite,
so the parity of a turn holds with no colouring hypothesis. When
`P − A + N = 2`, the quad mesh has Euler characteristic `2`
(`eulerQuad_eq_nodeCount`), so its vertices carry a colouring that changes along
every edge (`quad_bipartite`). Inside a patch that colouring is
`κ_P + i + j` at the vertex `(2i, 2j)`, `κ_P` its value at the corner `(0, 0)`;
it is a function of the classes, so it agrees on every gluing pair: `κ` colours
the quad graph (`bipartite_of_sphere`). With `Bipartite.lean` this gives
Proposition "Parity of a turn on a sphere" (`parity_of_turn_sphere`).
-/
module

public import TurnsOfATMesh.CoarseSurface
public import TurnsOfATMesh.CoarseEuler
public import TurnsOfATMesh.Bipartite

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

open Classical Matrix

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] {T : TMesh Patch α}

/-- Two different colours in `ZMod 2` differ by one. -/
theorem zmod_two_eq_add_one_of_ne {a b : ZMod 2} (h : a ≠ b) : b = a + 1 := by
  revert a b; decide

section Colour

variable {patches : List Patch} {allArcs : List α} (hP : ∀ P, P ∈ patches)
  (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
  (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P)
  (c : T.QuadV patches allArcs ℓ → ZMod 2)
  (hcol : ∀ e : T.QuadE patches allArcs ℓ, c (T.quadEnds e).1 ≠ c (T.quadEnds e).2)
include hP hA hne hbal hwh hcol

variable (T) in
/-- The colour `c` read on the points of the fine grid, `0` off the vertices. -/
noncomputable def colourAt (c : T.QuadV patches allArcs ℓ → ZMod 2) (p : Point Patch) : ZMod 2 :=
  if h : p ∈ T.points patches ℓ ∧ p.IsCoarse then c (QuadV.mk T p h) else 0

/-- Along an edge the colour changes by one. -/
theorem colourAt_edgeEnds {m : Point Patch} (hm : m ∈ T.points patches ℓ ∧ m.IsEdge) :
    T.colourAt c (edgeEnds m).2 = T.colourAt c (edgeEnds m).1 + 1 := by
  obtain ⟨h₁, h₂⟩ := edgeEnds_mem hm
  obtain ⟨e, he⟩ := quadAdj_of_edge hP hA hne hbal hwh hm
    (u := QuadV.mk T _ h₁) (v := QuadV.mk T _ h₂) rfl
  have hc := hcol e
  unfold colourAt
  rw [dite_eq_left h₁, dite_eq_left h₂]
  rcases Sym2.eq_iff.mp he with ⟨ha, hb⟩ | ⟨ha, hb⟩
  · rw [ha, hb] at hc; exact zmod_two_eq_add_one_of_ne hc
  · rw [ha, hb] at hc; exact zmod_two_eq_add_one_of_ne (Ne.symm hc)

/-- **Inside a patch the colour is `κ_P + i + j`** at the vertex `(2i, 2j)`, `κ_P`
the colour of the corner `(0, 0)`. -/
theorem colourAt_coarse (P : Patch) (i j : ℕ) (hi : i ≤ T.width ℓ P) (hj : j ≤ T.height ℓ P) :
    T.colourAt c ⟨P, 2 * i, 2 * j⟩ = c (T.quadBase hP allArcs ℓ P) + i + j := by
  have hmem : ∀ x y, x ≤ 2 * T.width ℓ P → y ≤ 2 * T.height ℓ P →
      (⟨P, x, y⟩ : Point Patch) ∈ T.points patches ℓ :=
    fun x y hx hy => (mem_points hP).mpr ⟨hx, hy⟩
  have row : ∀ i, i ≤ T.width ℓ P →
      T.colourAt c ⟨P, 2 * i, 0⟩ = c (T.quadBase hP allArcs ℓ P) + i := by
    intro i hi
    induction i with
    | zero =>
      simp only [Nat.mul_zero, Nat.cast_zero, add_zero, colourAt]
      rw [dite_eq_left ⟨hmem 0 0 (Nat.zero_le _) (Nat.zero_le _), rfl, rfl⟩]; rfl
    | succ i ih =>
      have h := colourAt_edgeEnds hP hA hne hbal hwh c hcol (m := ⟨P, 2 * i + 1, 0⟩)
        ⟨hmem _ _ (by omega) (Nat.zero_le _), by simp only [Point.IsEdge]; omega⟩
      have hends : edgeEnds (⟨P, 2 * i + 1, 0⟩ : Point Patch) =
          (⟨P, 2 * i, 0⟩, ⟨P, 2 * (i + 1), 0⟩) := by
        unfold edgeEnds; rw [ite_eq_left (by dsimp only; omega)]; simp; omega
      rw [hends] at h
      rw [h, ih (by omega)]; push_cast; ring
  induction j with
  | zero => simpa using row i hi
  | succ j ih =>
    have h := colourAt_edgeEnds hP hA hne hbal hwh c hcol (m := ⟨P, 2 * i, 2 * j + 1⟩)
      ⟨hmem _ _ (by omega) (by omega), by simp only [Point.IsEdge]; omega⟩
    have hends : edgeEnds (⟨P, 2 * i, 2 * j + 1⟩ : Point Patch) =
        (⟨P, 2 * i, 2 * j⟩, ⟨P, 2 * i, 2 * (j + 1)⟩) := by
      unfold edgeEnds; rw [ite_eq_right (by dsimp only; omega)]; simp; omega
    rw [hends] at h
    rw [h, ih (by omega)]; push_cast; ring

/-- The colour of a vertex of a patch is `pointColour` for the colours of the corners. -/
theorem colourAt_eq_pointColour {p : Point Patch} (hp : p ∈ T.points patches ℓ ∧ p.IsCoarse) :
    T.colourAt c p = pointColour (fun P => c (T.quadBase hP allArcs ℓ P)) p := by
  obtain ⟨P, x, y⟩ := p
  obtain ⟨hp₁, hx₂, hy₂⟩ := hp
  obtain ⟨hx, hy⟩ := (mem_points hP).mp hp₁
  dsimp only at hx hy hx₂ hy₂
  obtain ⟨i, rfl⟩ : ∃ i, x = 2 * i := ⟨x / 2, by omega⟩
  obtain ⟨j, rfl⟩ : ∃ j, y = 2 * j := ⟨y / 2, by omega⟩
  rw [colourAt_coarse hP hA hne hbal hwh c hcol P i j (by omega) (by omega), pointColour]
  simp only [Nat.mul_div_cancel_left _ two_pos]
  ring

end Colour

/-- **The quad graph of a T-mesh of a sphere is bipartite**: at positive balanced
whole lengths, every side holding an arc, the T-mesh connected and
`P − A + N = 2`, some constants `κ_P` colour it. -/
theorem bipartite_of_sphere [Fintype Patch] [Fintype α] {allArcs : List α}
    (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a) (hne : ∀ σ, T.arcs σ ≠ [])
    (hbal : T.Balanced ℓ) (hc : T.Connected)
    (hχ : (Fintype.card Patch : ℤ) - Fintype.card α + T.nodeCount = 2) :
    T.Bipartite allArcs ℓ := by
  let patches := (Finset.univ : Finset Patch).toList
  have hP : ∀ P, P ∈ patches := fun P => Finset.mem_toList.mpr (Finset.mem_univ P)
  have hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P :=
    fun P => ⟨sideLen_pos hℓ hne _, sideLen_pos hℓ hne _⟩
  obtain ⟨c, hcol⟩ := quad_bipartite (patches := patches) hP hA hℓ hne hbal hc
    (by rw [eulerQuad_eq_nodeCount hP hA hℓ hne hbal]; exact hχ)
  refine ⟨fun P => c (T.quadBase hP allArcs ℓ P), fun pq hpq h₁ h₂ => ?_⟩
  obtain ⟨b, t, ht, hp, hq⟩ := mem_gluings hpq
  have hp' : pq.1 ∈ T.points patches ℓ ∧ pq.1.IsCoarse :=
    ⟨(mem_points hP).mpr (hp ▸ arcPoint_inGrid hbal _ _ ht), h₁⟩
  have hq' : pq.2 ∈ T.points patches ℓ ∧ pq.2.IsCoarse :=
    ⟨(mem_points hP).mpr (hq ▸ arcPoint_inGrid hbal _ _ (flip_le ht)), h₂⟩
  rw [← colourAt_eq_pointColour hP hA hne hbal hwh c hcol hp',
    ← colourAt_eq_pointColour hP hA hne hbal hwh c hcol hq', colourAt, colourAt,
    dite_eq_left hp', dite_eq_left hq']
  congr 1
  exact Subtype.ext (vertexOf_eq_iff.mpr (Relation.EqvGen.rel _ _ hpq))

/-- Balanced whole lengths, in the sense of the matrix `B`, are `Balanced`. -/
theorem balanced_of_mulVec [Fintype α] {ℓ : α → ℕ}
    (h : T.balance *ᵥ (fun a => (ℓ a : ℤ)) = 0) : T.Balanced ℓ := by
  have h' := (T.balance_mulVec_eq_zero_iff (K := ℤ) (fun a => (ℓ a : ℤ))).mp
    (by rw [show T.balance.map (Int.cast : ℤ → ℤ) = T.balance from by ext; simp]; exact h)
  intro P j
  have := h' P j
  unfold sideLen
  have e : ∀ σ : Side Patch,
      ((((T.arcs σ).map ℓ).sum : ℕ) : ℤ) = ((T.arcs σ).map fun a => (ℓ a : ℤ)).sum :=
    fun σ => by rw [Nat.cast_list_sum, List.map_map]; rfl
  exact_mod_cast (e (P, j, 0)).trans (this.trans (e (P, j, 1)).symm)

/-- **Parity of a turn on a sphere**, at one positive balanced whole `ℓ`: a turn
whose walk is followed from a station `k + ½` of its start has
`α_γ(ℓ) ≡ shift_γ(ℓ) (mod 2)`. -/
theorem parity_of_turn_sphere_at [Fintype Patch] [Fintype α] (hT : T.Oriented)
    {allArcs : List α} (hA : ∀ a, a ∈ allArcs) (hne : ∀ σ, T.arcs σ ≠ []) (hc : T.Connected)
    (hχ : (Fintype.card Patch : ℤ) - Fintype.card α + T.nodeCount = 2) (γ : T.Turn)
    {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a) (hbal : T.Balanced ℓ) (k : ℕ)
    (hk : T.Follows (rat ℓ) γ.start γ.path ((k : ℚ) + 1 / 2)) :
    ((evalForm γ.lengthForm (fun a => (ℓ a : ℤ)) : ℤ) : ZMod 2) =
      ((evalForm γ.shiftForm (fun a => (ℓ a : ℤ)) : ℤ) : ZMod 2) := by
  obtain ⟨κ, hκ⟩ := bipartite_of_sphere hA hℓ hne hbal hc hχ
  exact T.parity_of_turn_of_colouring hT allArcs hA ℓ κ hκ γ k hk

/-- **Proposition "Parity of a turn on a sphere"**: on an oriented connected T-mesh
with `P − A + N = 2`, every side holding an arc, admitting positive balanced
lengths, a turn whose walk is followed from some station at every positive
balanced whole `ℓ` has `α_γ(ℓ) ≡ shift_γ(ℓ) (mod 2)` at every balanced integer
`ℓ`. -/
theorem parity_of_turn_sphere [Fintype Patch] [Fintype α] (hT : T.Oriented)
    {allArcs : List α} (hA : ∀ a, a ∈ allArcs) (hne : ∀ σ, T.arcs σ ≠ []) (hc : T.Connected)
    (hχ : (Fintype.card Patch : ℤ) - Fintype.card α + T.nodeCount = 2) (γ : T.Turn)
    (hreal : ∃ ℓ : α → ℝ, T.balance.map (Int.cast : ℤ → ℝ) *ᵥ ℓ = 0 ∧ ∀ a, 0 < ℓ a)
    (hstation : ∀ ℓ : α → ℕ, T.balance *ᵥ (fun a => (ℓ a : ℤ)) = 0 → (∀ a, 0 < ℓ a) →
      ∃ k : ℕ, T.Follows (rat ℓ) γ.start γ.path ((k : ℚ) + 1 / 2)) :
    ∀ ℓ : α → ℤ, T.balance *ᵥ ℓ = 0 →
      ((evalForm γ.lengthForm ℓ : ℤ) : ZMod 2) = ((evalForm γ.shiftForm ℓ : ℤ) : ZMod 2) :=
  T.parity_of_turn_tmesh_any_sign hT allArcs hA γ hreal fun ℓ hB hpos =>
    ⟨bipartite_of_sphere hA hpos hne (balanced_of_mulVec hB) hc hχ, hstation ℓ hB hpos⟩

end TMesh

end TurnsOfATMesh
