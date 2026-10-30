/-
The Euler characteristic of the quad mesh of a T-mesh at whole lengths
(`Grid.lean`) does not depend on the lengths: at positive balanced lengths it is
`P − A + N`, the patches minus the arcs plus the nodes of the T-mesh.

Every point of the fine grid is inside a patch, inside an arc, or an end of an
arc (`arcPoint`). The gluings never touch a point inside a patch, glue a point
inside an arc only to its partner on the other side of the same arc, and keep
the ends of the arcs among themselves. So a patch adds `+1`, an arc `−1`, a node
`+1`. An edge of the quad mesh is counted as the class of its midpoint in the
grid refined once more (lengths `2ℓ`), where the same three cases apply.
-/
module

public import TurnsOfATMesh.Grid

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### Positions of the arcs on a side -/

theorem posNat_add_le (ℓ : α → ℕ) {a : α} {l : List α} (h : a ∈ l) :
    posNat a l ℓ + ℓ a ≤ (l.map ℓ).sum := by
  induction l with
  | nil => simp at h
  | cons b l ih =>
    simp only [posNat, List.map_cons, List.sum_cons]
    split_ifs with hb
    · subst hb; omega
    · have := ih ((List.mem_cons.mp h).resolve_left (Ne.symm hb)); omega

/-- Two arcs of a side cover intervals with disjoint interiors. -/
theorem posNat_disjoint (ℓ : α → ℕ) {l : List α} (hl : l.Nodup) {a b : α} (ha : a ∈ l)
    (hb : b ∈ l) (hab : a ≠ b) :
    posNat a l ℓ + ℓ a ≤ posNat b l ℓ ∨ posNat b l ℓ + ℓ b ≤ posNat a l ℓ := by
  induction l with
  | nil => simp at ha
  | cons c l ih =>
    rw [List.nodup_cons] at hl
    simp only [posNat]
    rcases List.mem_cons.mp ha with rfl | ha'
    · left; simp [hab]
    rcases List.mem_cons.mp hb with rfl | hb'
    · right; simp [Ne.symm hab]
    have hca : c ≠ a := fun h => hl.1 (h ▸ ha')
    have hcb : c ≠ b := fun h => hl.1 (h ▸ hb')
    simp only [hca, hcb, ite_false]
    rcases ih hl.2 ha' hb' with h | h <;> omega

/-- Every fine position of a side lies on one of its arcs. -/
theorem exists_arc_of_le (ℓ : α → ℕ) {l : List α} (hl : l.Nodup) (hne : l ≠ []) {u : ℕ}
    (hu : u ≤ 2 * (l.map ℓ).sum) :
    ∃ a ∈ l, 2 * posNat a l ℓ ≤ u ∧ u ≤ 2 * posNat a l ℓ + 2 * ℓ a := by
  induction l generalizing u with
  | nil => exact absurd rfl hne
  | cons c l ih =>
    rw [List.nodup_cons] at hl
    simp only [List.map_cons, List.sum_cons] at hu
    by_cases hc : u ≤ 2 * ℓ c
    · exact ⟨c, by simp, by simp [posNat], by simp [posNat]; omega⟩
    · have hl' : l ≠ [] := by rintro rfl; simp at hu; omega
      obtain ⟨a, ha, h₁, h₂⟩ := ih hl.2 hl' (u := u - 2 * ℓ c) (by omega)
      have hca : c ≠ a := fun h => hl.1 (h ▸ ha)
      exact ⟨a, by simp [ha], by simp [posNat, hca]; omega, by simp [posNat, hca]; omega⟩

/-! ### The sides of a rectangle at balanced lengths -/

/-- The lengths `ℓ` are balanced: opposite sides of every patch have one length. -/
def Balanced (ℓ : α → ℕ) : Prop := ∀ P j, T.sideLen ℓ (P, j, 0) = T.sideLen ℓ (P, j, 1)

variable {T}

theorem sideLen_eq {ℓ : α → ℕ} (hbal : T.Balanced ℓ) (σ : Side Patch) :
    T.sideLen ℓ σ = if σ.2.1 = 0 then T.width ℓ σ.1 else T.height ℓ σ.1 := by
  obtain ⟨P, j, k⟩ := σ
  fin_cases j <;> fin_cases k <;> simp [width, height, ← hbal P]

theorem sideLen_pos {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a) (hne : ∀ σ, T.arcs σ ≠ [])
    (σ : Side Patch) : 0 < T.sideLen ℓ σ := by
  obtain ⟨a, ha⟩ := List.exists_mem_of_ne_nil _ (hne σ)
  have := posNat_add_le ℓ ha
  have := hℓ a
  unfold sideLen; omega

/-- A point of the fine grid of its patch. -/
def InGrid (ℓ : α → ℕ) (p : Point Patch) : Prop :=
  p.x ≤ 2 * T.width ℓ p.patch ∧ p.y ≤ 2 * T.height ℓ p.patch

/-- A point inside its patch. -/
def IsInner (ℓ : α → ℕ) (p : Point Patch) : Prop :=
  0 < p.x ∧ p.x < 2 * T.width ℓ p.patch ∧ 0 < p.y ∧ p.y < 2 * T.height ℓ p.patch

theorem sidePoint_not_inner (ℓ : α → ℕ) (σ : Side Patch) (u : ℕ) :
    ¬ T.IsInner ℓ (T.sidePoint ℓ σ u) := by
  unfold sidePoint IsInner; split_ifs <;> simp

theorem sidePoint_inGrid {ℓ : α → ℕ} (hbal : T.Balanced ℓ) {σ : Side Patch} {u : ℕ}
    (hu : u ≤ 2 * T.sideLen ℓ σ) : T.InGrid ℓ (T.sidePoint ℓ σ u) := by
  rw [sideLen_eq hbal] at hu
  unfold sidePoint InGrid; split_ifs at hu ⊢ <;> simp_all

/-- The sum of the coordinates of a point of a side has the parity of its position. -/
theorem sidePoint_parity (ℓ : α → ℕ) (σ : Side Patch) (u : ℕ) :
    ((T.sidePoint ℓ σ u).x + (T.sidePoint ℓ σ u).y) % 2 = u % 2 := by
  unfold sidePoint; split_ifs <;> simp

/-- A point of a side other than its corners lies on no other side. -/
theorem sidePoint_inj {ℓ : α → ℕ} (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {σ τ : Side Patch} {u v : ℕ}
    (hu₀ : 0 < u) (hu : u < 2 * T.sideLen ℓ σ) (hv : v ≤ 2 * T.sideLen ℓ τ)
    (h : T.sidePoint ℓ σ u = T.sidePoint ℓ τ v) : σ = τ ∧ u = v := by
  rw [sideLen_eq hbal] at hu hv
  obtain ⟨P, j, k⟩ := σ
  obtain ⟨Q, j', k'⟩ := τ
  obtain ⟨hw, hh⟩ := hwh P
  fin_cases j <;> fin_cases k <;> fin_cases j' <;> fin_cases k' <;>
    simp [sidePoint, Point.mk.injEq] at h hu hv ⊢ <;> obtain ⟨rfl, h₁, h₂⟩ := h <;> first | omega | exact ⟨rfl, by omega⟩

/-! ### The points of the arcs -/

variable (T)

/-- The point at the fine position `s` along the arc `a` on its side `ends a i`,
counted in the direction of that side, `0 ≤ s ≤ 2 ℓ_a`. -/
def arcPoint (ℓ : α → ℕ) (a : α) (i : Fin 2) (s : ℕ) : Point Patch :=
  T.sidePoint ℓ (T.ends a i) (2 * posNat a (T.arcs (T.ends a i)) ℓ + s)

/-- The position on `ends a 1` of the point at `t` on `ends a 0`. -/
def flip (ℓ : α → ℕ) (a : α) (t : ℕ) : ℕ := if T.sign a = 1 then t else 2 * ℓ a - t

variable {T}

theorem mem_arcs_ends (a : α) (i : Fin 2) : a ∈ T.arcs (T.ends a i) :=
  (T.mem_arcs _ _).mpr (by fin_cases i <;> simp)

theorem ends_inj {a : α} {i j : Fin 2} (h : T.ends a i = T.ends a j) : i = j := by
  have := T.ends_ne a
  fin_cases i <;> fin_cases j <;> simp_all [eq_comm]

theorem arcPoint_inGrid {ℓ : α → ℕ} (hbal : T.Balanced ℓ) (a : α) (i : Fin 2) {s : ℕ}
    (hs : s ≤ 2 * ℓ a) : T.InGrid ℓ (T.arcPoint ℓ a i s) := by
  have := posNat_add_le ℓ (mem_arcs_ends (T := T) a i)
  exact sidePoint_inGrid hbal (by unfold sideLen; omega)

theorem arcPoint_not_inner (ℓ : α → ℕ) (a : α) (i : Fin 2) (s : ℕ) :
    ¬ T.IsInner ℓ (T.arcPoint ℓ a i s) := sidePoint_not_inner _ _ _

theorem arcPoint_parity (ℓ : α → ℕ) (a : α) (i : Fin 2) (s : ℕ) :
    ((T.arcPoint ℓ a i s).x + (T.arcPoint ℓ a i s).y) % 2 = s % 2 := by
  rw [arcPoint, sidePoint_parity]; omega

/-- **A point inside an arc is on no other arc**, and is not an end of an arc. -/
theorem arcPoint_inj {ℓ : α → ℕ} (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {a b : α} {i j : Fin 2} {s s' : ℕ}
    (hs₀ : 0 < s) (hs : s < 2 * ℓ a) (hs' : s' ≤ 2 * ℓ b)
    (h : T.arcPoint ℓ a i s = T.arcPoint ℓ b j s') : a = b ∧ i = j ∧ s = s' := by
  have ha := mem_arcs_ends (T := T) a i
  have hb := mem_arcs_ends (T := T) b j
  have h₁ := posNat_add_le ℓ ha
  have h₂ := posNat_add_le ℓ hb
  obtain ⟨hστ, huv⟩ := sidePoint_inj hbal hwh (by omega) (by unfold sideLen; omega)
    (by unfold sideLen; omega) h
  rw [← hστ] at hb huv
  by_cases hab : a = b
  · subst hab
    exact ⟨rfl, ends_inj hστ, by omega⟩
  · rcases posNat_disjoint ℓ (T.arcs_nodup _) ha hb hab with h | h <;> omega

theorem flip_le {ℓ : α → ℕ} {a : α} {t : ℕ} (ht : t ≤ 2 * ℓ a) : T.flip ℓ a t ≤ 2 * ℓ a := by
  unfold flip; split_ifs <;> omega

theorem flip_inner {ℓ : α → ℕ} {a : α} {t : ℕ} (ht₀ : 0 < t) (ht : t < 2 * ℓ a) :
    0 < T.flip ℓ a t ∧ T.flip ℓ a t < 2 * ℓ a := by
  unfold flip; split_ifs <;> omega

theorem flip_flip {ℓ : α → ℕ} {a : α} {t : ℕ} (ht : t ≤ 2 * ℓ a) :
    T.flip ℓ a (T.flip ℓ a t) = t := by
  unfold flip; split_ifs <;> omega

theorem flip_parity (ℓ : α → ℕ) (a : α) {t : ℕ} (ht : t ≤ 2 * ℓ a) :
    T.flip ℓ a t % 2 = t % 2 := by
  unfold flip; split_ifs <;> omega

theorem flip_end {ℓ : α → ℕ} {a : α} {t : ℕ} (ht : t = 0 ∨ t = 2 * ℓ a) :
    T.flip ℓ a t = 0 ∨ T.flip ℓ a t = 2 * ℓ a := by
  unfold flip; split_ifs <;> omega

theorem glued_eq (ℓ : α → ℕ) (a : α) {t : ℕ} (ht : t ≤ 2 * ℓ a) :
    T.glued ℓ a t = (T.arcPoint ℓ a 0 t, T.arcPoint ℓ a 1 (T.flip ℓ a t)) := by
  simp only [glued, arcPoint, flip]
  split_ifs
  · rfl
  · congr 2; omega

theorem mem_gluings {allArcs : List α} {ℓ : α → ℕ} {p q : Point Patch}
    (h : (p, q) ∈ T.gluings allArcs ℓ) : ∃ a t, t ≤ 2 * ℓ a ∧
      p = T.arcPoint ℓ a 0 t ∧ q = T.arcPoint ℓ a 1 (T.flip ℓ a t) := by
  simp only [gluings, List.mem_flatMap, List.mem_map, List.mem_range] at h
  obtain ⟨a, -, t, ht, hpq⟩ := h
  rw [glued_eq ℓ a (by omega), Prod.mk.injEq] at hpq
  exact ⟨a, t, by omega, hpq.1.symm, hpq.2.symm⟩

/-! ### The classes of `Same` -/

/-- A property that the two points of every gluing pair share is shared by the
points of one vertex. -/
theorem same_iff_of_gluings {allArcs : List α} {ℓ : α → ℕ} {S : Point Patch → Prop}
    (hS : ∀ p q, (p, q) ∈ T.gluings allArcs ℓ → (S p ↔ S q)) {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : S p ↔ S q := by
  induction h with
  | rel x y hxy => exact hS x y hxy
  | refl => rfl
  | symm x y _ ih => exact ih.symm
  | trans x y z _ _ ih₁ ih₂ => exact ih₁.trans ih₂

/-- **(a)** A point inside a patch is a vertex alone. -/
theorem same_of_inner {allArcs : List α} {ℓ : α → ℕ} {p q : Point Patch}
    (hp : T.IsInner ℓ p) (h : T.Same allArcs ℓ p q) : q = p := by
  have key : ∀ x y, (x, y) ∈ T.gluings allArcs ℓ → (x = p ↔ y = p) := by
    intro x y hxy
    obtain ⟨a, t, -, rfl, rfl⟩ := mem_gluings hxy
    constructor <;> rintro rfl <;> exact absurd hp (arcPoint_not_inner _ _ _ _)
  exact (same_iff_of_gluings key h).mp rfl

/-- **(b)** A point inside an arc is one vertex with its partner on the other side
of the arc, and with no other point. -/
theorem same_of_arcInner {allArcs : List α} {ℓ : α → ℕ} (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {a : α} {t : ℕ} (ht₀ : 0 < t)
    (ht : t < 2 * ℓ a) {q : Point Patch} (h : T.Same allArcs ℓ (T.arcPoint ℓ a 0 t) q) :
    q = T.arcPoint ℓ a 0 t ∨ q = T.arcPoint ℓ a 1 (T.flip ℓ a t) := by
  obtain ⟨hf₀, hf⟩ := flip_inner (T := T) ht₀ ht
  have key : ∀ x y, (x, y) ∈ T.gluings allArcs ℓ →
      ((x = T.arcPoint ℓ a 0 t ∨ x = T.arcPoint ℓ a 1 (T.flip ℓ a t)) ↔
        (y = T.arcPoint ℓ a 0 t ∨ y = T.arcPoint ℓ a 1 (T.flip ℓ a t))) := by
    intro x y hxy
    obtain ⟨b, t', ht', rfl, rfl⟩ := mem_gluings hxy
    have hft' := flip_le (T := T) ht'
    constructor
    · rintro (h | h)
      · obtain ⟨rfl, -, rfl⟩ := arcPoint_inj hbal hwh ht₀ ht ht' h.symm
        exact Or.inr rfl
      · obtain ⟨-, h01, -⟩ := arcPoint_inj hbal hwh hf₀ hf ht' h.symm
        exact absurd h01 (by decide)
    · rintro (h | h)
      · obtain ⟨-, h01, -⟩ := arcPoint_inj hbal hwh ht₀ ht hft' h.symm
        exact absurd h01 (by decide)
      · obtain ⟨rfl, -, h'⟩ := arcPoint_inj hbal hwh hf₀ hf hft' h.symm
        have := congrArg (T.flip ℓ a) h'
        rw [flip_flip ht.le, flip_flip ht'] at this
        exact Or.inl (by rw [this])
  exact (same_iff_of_gluings key h).mp (Or.inl rfl)

/-- An end of an arc. -/
def IsEnd (ℓ : α → ℕ) (p : Point Patch) : Prop :=
  ∃ a i, p = T.arcPoint ℓ a i 0 ∨ p = T.arcPoint ℓ a i (2 * ℓ a)

theorem not_isEnd {ℓ : α → ℕ} (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {a : α} {i : Fin 2} {t : ℕ} (ht₀ : 0 < t)
    (ht : t < 2 * ℓ a) : ¬ T.IsEnd ℓ (T.arcPoint ℓ a i t) := by
  rintro ⟨b, j, h | h⟩
  · have := (arcPoint_inj hbal hwh ht₀ ht (by omega) h).2.2; omega
  · obtain ⟨rfl, -, h⟩ := arcPoint_inj hbal hwh ht₀ ht le_rfl h; omega

/-- **(c)** The ends of the arcs are glued only to ends of arcs. -/
theorem isEnd_of_same {allArcs : List α} {ℓ : α → ℕ} (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : T.IsEnd ℓ p ↔ T.IsEnd ℓ q := by
  refine same_iff_of_gluings (fun x y hxy => ?_) h
  obtain ⟨b, t, ht, rfl, rfl⟩ := mem_gluings hxy
  by_cases hte : t = 0 ∨ t = 2 * ℓ b
  · have hfe := flip_end (T := T) hte
    exact ⟨fun _ => ⟨b, 1, hfe.imp (congrArg _) (congrArg _)⟩,
      fun _ => ⟨b, 0, hte.imp (congrArg _) (congrArg _)⟩⟩
  · obtain ⟨hf₀, hf⟩ := flip_inner (T := T) (ℓ := ℓ) (a := b) (t := t) (by omega) (by omega)
    exact iff_of_false (not_isEnd hbal hwh (by omega) (by omega)) (not_isEnd hbal hwh hf₀ hf)

theorem inGrid_of_same {allArcs : List α} {ℓ : α → ℕ} (hbal : T.Balanced ℓ) {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : T.InGrid ℓ p ↔ T.InGrid ℓ q := by
  refine same_iff_of_gluings (fun x y hxy => ?_) h
  obtain ⟨b, t, ht, rfl, rfl⟩ := mem_gluings hxy
  exact iff_of_true (arcPoint_inGrid hbal _ _ ht) (arcPoint_inGrid hbal _ _ (flip_le ht))

theorem parity_of_same {allArcs : List α} {ℓ : α → ℕ} {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : (p.x + p.y) % 2 = (q.x + q.y) % 2 := by
  have key := same_iff_of_gluings (S := fun r : Point Patch => (r.x + r.y) % 2 = (p.x + p.y) % 2)
    (fun x y hxy => by
      obtain ⟨b, t, ht, rfl, rfl⟩ := mem_gluings hxy
      simp only [arcPoint_parity, flip_parity ℓ b ht]) h
  exact (key.mp rfl).symm

end TMesh

end TurnsOfATMesh
