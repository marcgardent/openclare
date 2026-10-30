/-
The Euler characteristic of the quad mesh of a T-mesh at whole lengths
(`Grid.lean`) does not depend on the lengths: at positive balanced lengths, every
side holding an arc, it is `P − A + N`, the patches minus the arcs plus the nodes
of the T-mesh (`euler_eq_nodeCount`). The nodes are defined without lengths
(`nodeCount`): the ends of the arcs as junctions on the sides, identified by the
gluing of the arcs and at the corners of the patches.

Every point of the fine grid is inside a patch, inside an arc, or an end of an
arc (`arcPoint`). The gluings never touch a point inside a patch, glue a point
inside an arc only to its partner on the other side of the same arc, and keep
the ends of the arcs among themselves. So a patch adds `+1`, an arc `−1`, a node
`+1`. An edge of the quad mesh is counted as the class of its midpoint in the
grid refined once more (lengths `2ℓ`), where the same three cases apply.
Counting edges as unordered pairs of vertices instead undercounts at small
lengths: the square torus of one patch at `ℓ = 1` has 8 edges but 4 such pairs.
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
    simp [sidePoint, Point.mk.injEq] at h hu hv ⊢ <;> obtain ⟨rfl, h₁, h₂⟩ := h <;>
    first | omega | exact ⟨rfl, by omega⟩

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

/-! ### Every point of the grid is inside a patch or on an arc -/

theorem exists_side {ℓ : α → ℕ} (hbal : T.Balanced ℓ) {p : Point Patch} (hp : T.InGrid ℓ p)
    (hi : ¬ T.IsInner ℓ p) : ∃ σ u, u ≤ 2 * T.sideLen ℓ σ ∧ p = T.sidePoint ℓ σ u := by
  obtain ⟨P, x, y⟩ := p
  simp only [InGrid, IsInner] at hp hi
  by_cases hx₀ : x = 0
  · exact ⟨(P, 1, 0), y, by rw [sideLen_eq hbal]; simp; omega, by simp [sidePoint, hx₀]⟩
  by_cases hx₁ : x = 2 * T.width ℓ P
  · exact ⟨(P, 1, 1), y, by rw [sideLen_eq hbal]; simp; omega, by simp [sidePoint, hx₁]⟩
  by_cases hy₀ : y = 0
  · exact ⟨(P, 0, 0), x, by rw [sideLen_eq hbal]; simp; omega, by simp [sidePoint, hy₀]⟩
  by_cases hy₁ : y = 2 * T.height ℓ P
  · exact ⟨(P, 0, 1), x, by rw [sideLen_eq hbal]; simp; omega, by simp [sidePoint, hy₁]⟩
  exact absurd ⟨by omega, by omega, by omega, by omega⟩ hi

/-- A point of the grid not inside its patch is a point of an arc. -/
theorem exists_arcPoint {ℓ : α → ℕ} (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
    {p : Point Patch} (hp : T.InGrid ℓ p) (hi : ¬ T.IsInner ℓ p) :
    ∃ a i s, s ≤ 2 * ℓ a ∧ p = T.arcPoint ℓ a i s := by
  obtain ⟨σ, u, hu, rfl⟩ := exists_side hbal hp hi
  obtain ⟨a, ha, h₁, h₂⟩ := exists_arc_of_le ℓ (T.arcs_nodup σ) (hne σ) hu
  obtain ⟨i, rfl⟩ : ∃ i, σ = T.ends a i := by
    rcases (T.mem_arcs a σ).mp ha with h | h
    exacts [⟨0, h⟩, ⟨1, h⟩]
  exact ⟨a, i, u - 2 * posNat a (T.arcs (T.ends a i)) ℓ, by omega,
    by unfold arcPoint; congr 1; omega⟩

/-! ### Counting the classes -/

variable (T) in
/-- The vertex of the quad mesh at a point. -/
def vertexOf (allArcs : List α) (ℓ : α → ℕ) (p : Point Patch) : T.Vertex allArcs ℓ :=
  Quotient.mk (T.vertexSetoid allArcs ℓ) p

theorem vertexOf_eq_iff {allArcs : List α} {ℓ : α → ℕ} {p q : Point Patch} :
    T.vertexOf allArcs ℓ p = T.vertexOf allArcs ℓ q ↔ T.Same allArcs ℓ p q :=
  Quotient.eq

/-- **The classes of a set of points** closed under `Same`: one per point inside a
patch, one per point inside an arc on its first side, and the classes of its
ends of arcs. -/
theorem ncard_classes [Fintype α] {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {S : Set (Point Patch)} (hfin : S.Finite)
    (hgrid : ∀ p ∈ S, T.InGrid ℓ p) (hsat : ∀ p q, p ∈ S → T.Same allArcs ℓ p q → q ∈ S) :
    (T.vertexOf allArcs ℓ '' S).ncard =
      (S ∩ {p | T.IsInner ℓ p}).ncard +
        {x : (_ : α) × ℕ | 0 < x.2 ∧ x.2 < 2 * ℓ x.1 ∧ T.arcPoint ℓ x.1 0 x.2 ∈ S}.ncard +
        (T.vertexOf allArcs ℓ '' (S ∩ {p | T.IsEnd ℓ p})).ncard := by
  set A := T.vertexOf allArcs ℓ '' (S ∩ {p | T.IsInner ℓ p})
  set X := {x : (_ : α) × ℕ | 0 < x.2 ∧ x.2 < 2 * ℓ x.1 ∧ T.arcPoint ℓ x.1 0 x.2 ∈ S}
  set g := fun x : (_ : α) × ℕ => T.vertexOf allArcs ℓ (T.arcPoint ℓ x.1 0 x.2)
  set C := T.vertexOf allArcs ℓ '' (S ∩ {p | T.IsEnd ℓ p})
  have hXfin : X.Finite := (Finset.univ.sigma fun a => Finset.range (2 * ℓ a)).finite_toSet.subset
    (by rintro ⟨a, t⟩ ⟨-, ht, -⟩; simp [ht])
  have hsplit : T.vertexOf allArcs ℓ '' S = A ∪ g '' X ∪ C := by
    ext v; constructor
    · rintro ⟨p, hp, rfl⟩
      by_cases hi : T.IsInner ℓ p
      · exact Or.inl (Or.inl ⟨p, ⟨hp, hi⟩, rfl⟩)
      obtain ⟨a, i, s, hs, rfl⟩ := exists_arcPoint hne hbal (hgrid _ hp) hi
      by_cases he : s = 0 ∨ s = 2 * ℓ a
      · exact Or.inr ⟨_, ⟨hp, a, i, he.imp (congrArg _) (congrArg _)⟩, rfl⟩
      have hs₀ : 0 < s := by omega
      have hs₁ : s < 2 * ℓ a := by omega
      refine Or.inl (Or.inr ?_)
      fin_cases i
      · exact ⟨⟨a, s⟩, ⟨hs₀, hs₁, hp⟩, rfl⟩
      · obtain ⟨hf₀, hf₁⟩ := flip_inner (T := T) hs₀ hs₁
        have hsame : T.Same allArcs ℓ (T.arcPoint ℓ a 0 (T.flip ℓ a s)) (T.arcPoint ℓ a 1 s) := by
          have := T.same_of_glued allArcs ℓ a (hA a) (T.flip ℓ a s) hf₁.le
          rwa [glued_eq ℓ a hf₁.le, flip_flip hs] at this
        exact ⟨⟨a, T.flip ℓ a s⟩, ⟨hf₀, hf₁, hsat _ _ hp (T.same_symm _ _ hsame)⟩,
          vertexOf_eq_iff.mpr hsame⟩
    · rintro ((⟨p, ⟨hp, -⟩, rfl⟩ | ⟨x, ⟨-, -, hx⟩, rfl⟩) | ⟨p, ⟨hp, -⟩, rfl⟩)
      exacts [⟨p, hp, rfl⟩, ⟨_, hx, rfl⟩, ⟨p, hp, rfl⟩]
  have hAX : Disjoint A (g '' X) := by
    rw [Set.disjoint_left]
    rintro v ⟨p, ⟨-, hi⟩, rfl⟩ ⟨x, -, hx⟩
    have := same_of_inner hi (vertexOf_eq_iff.mp hx.symm)
    exact arcPoint_not_inner ℓ x.1 0 x.2 (by rw [this]; exact hi)
  have hABC : Disjoint (A ∪ g '' X) C := by
    rw [Set.disjoint_union_left]
    constructor
    · rw [Set.disjoint_left]
      rintro v ⟨p, ⟨-, hi⟩, rfl⟩ ⟨q, ⟨-, a, i, he⟩, hq⟩
      have := same_of_inner hi (vertexOf_eq_iff.mp hq.symm)
      subst this
      rcases he with he | he <;> exact arcPoint_not_inner _ _ _ _ (he ▸ hi)
    · rw [Set.disjoint_left]
      rintro v ⟨x, ⟨h₀, h₁, -⟩, rfl⟩ ⟨q, ⟨-, hqe⟩, hq⟩
      exact not_isEnd hbal hwh h₀ h₁ ((isEnd_of_same hbal hwh (vertexOf_eq_iff.mp hq)).mp hqe)
  have hAcard : A.ncard = (S ∩ {p | T.IsInner ℓ p}).ncard :=
    Set.InjOn.ncard_image fun p ⟨_, hp⟩ _ _ h =>
      (same_of_inner hp (vertexOf_eq_iff.mp h)).symm
  have hXcard : (g '' X).ncard = X.ncard := by
    refine Set.InjOn.ncard_image ?_
    rintro ⟨a, t⟩ ⟨h₀, h₁, -⟩ ⟨b, t'⟩ ⟨h₀', h₁', -⟩ h
    rcases same_of_arcInner hbal hwh h₀ h₁ (vertexOf_eq_iff.mp h) with h | h
    · obtain ⟨rfl, -, rfl⟩ := arcPoint_inj hbal hwh h₀' h₁' h₁.le h
      rfl
    · obtain ⟨-, h01, -⟩ := arcPoint_inj hbal hwh h₀' h₁' (flip_le h₁.le) h
      exact absurd h01 (by decide)
  have hSfin : (S ∩ {p | T.IsInner ℓ p}).Finite := hfin.subset Set.inter_subset_left
  have hEfin : (S ∩ {p | T.IsEnd ℓ p}).Finite := hfin.subset Set.inter_subset_left
  rw [hsplit, Set.ncard_union_eq hABC ((hSfin.image _).union (hXfin.image _)) (hEfin.image _),
    Set.ncard_union_eq hAX (hSfin.image _) (hXfin.image _), hAcard, hXcard]

/-! ### Counting points -/

theorem card_filter_odd (m : ℕ) : ((Finset.Ioo 0 (2 * m)).filter (· % 2 = 1)).card = m := by
  have : (Finset.Ioo 0 (2 * m)).filter (· % 2 = 1) =
      (Finset.range m).image fun i => 2 * i + 1 := by
    ext t
    simp only [Finset.mem_filter, Finset.mem_Ioo, Finset.mem_image, Finset.mem_range]
    constructor
    · rintro ⟨⟨h₀, h₁⟩, h₂⟩; exact ⟨t / 2, by omega, by omega⟩
    · rintro ⟨i, hi, rfl⟩; omega
  rw [this, Finset.card_image_of_injective _ (fun i j (h : 2 * i + 1 = 2 * j + 1) => by omega),
    Finset.card_range]

theorem card_filter_even (m : ℕ) : ((Finset.Ioo 0 (2 * m)).filter (· % 2 = 0)).card = m - 1 := by
  have : (Finset.Ioo 0 (2 * m)).filter (· % 2 = 0) =
      (Finset.range (m - 1)).image fun i => 2 * i + 2 := by
    ext t
    simp only [Finset.mem_filter, Finset.mem_Ioo, Finset.mem_image, Finset.mem_range]
    constructor
    · rintro ⟨⟨h₀, h₁⟩, h₂⟩; exact ⟨t / 2 - 1, by omega, by omega⟩
    · rintro ⟨i, hi, rfl⟩; omega
  rw [this, Finset.card_image_of_injective _ (fun i j (h : 2 * i + 2 = 2 * j + 2) => by omega),
    Finset.card_range]

/-- The points of `(0, 2m) × (0, 2n)` whose coordinates have an odd sum. -/
theorem card_filter_odd_sum (m n : ℕ) :
    ((Finset.Ioo 0 (2 * m) ×ˢ Finset.Ioo 0 (2 * n)).filter fun q => (q.1 + q.2) % 2 = 1).card =
      m * (n - 1) + (m - 1) * n := by
  have : ((Finset.Ioo 0 (2 * m) ×ˢ Finset.Ioo 0 (2 * n)).filter fun q => (q.1 + q.2) % 2 = 1) =
      ((Finset.Ioo 0 (2 * m)).filter (· % 2 = 1) ×ˢ (Finset.Ioo 0 (2 * n)).filter (· % 2 = 0)) ∪
        ((Finset.Ioo 0 (2 * m)).filter (· % 2 = 0) ×ˢ
          (Finset.Ioo 0 (2 * n)).filter (· % 2 = 1)) := by
    ext ⟨x, y⟩
    simp only [Finset.mem_filter, Finset.mem_product, Finset.mem_union, Finset.mem_Ioo]
    constructor <;> intro <;> omega
  rw [this, Finset.card_union_of_disjoint, Finset.card_product, Finset.card_product,
    card_filter_odd, card_filter_odd, card_filter_even, card_filter_even]
  rw [Finset.disjoint_left]
  rintro ⟨x, y⟩ h₁ h₂
  simp only [Finset.mem_filter, Finset.mem_product] at h₁ h₂
  omega

section Count

variable [Fintype Patch]

/-- The points `(P, x, y)` with `(x, y) ∈ f P`. -/
def ptsOf (f : Patch → Finset (ℕ × ℕ)) : Finset (Point Patch) :=
  (Finset.univ.sigma f).image fun q => ⟨q.1, q.2.1, q.2.2⟩

theorem mem_ptsOf {f : Patch → Finset (ℕ × ℕ)} {p : Point Patch} :
    p ∈ ptsOf f ↔ (p.x, p.y) ∈ f p.patch := by
  simp only [ptsOf, Finset.mem_image, Finset.mem_sigma, Finset.mem_univ, true_and]
  constructor
  · rintro ⟨⟨P, x, y⟩, h, rfl⟩; exact h
  · intro h; exact ⟨⟨p.patch, p.x, p.y⟩, h, rfl⟩

theorem card_ptsOf (f : Patch → Finset (ℕ × ℕ)) : (ptsOf f).card = ∑ P, (f P).card := by
  rw [ptsOf, Finset.card_image_of_injective, Finset.card_sigma]
  rintro ⟨P, x, y⟩ ⟨Q, x', y'⟩ h
  simp only [Point.mk.injEq] at h
  obtain ⟨rfl, rfl, rfl⟩ := h
  rfl

end Count

/-! ### The vertices, edges and faces of the quad mesh -/

/-- The lengths doubled: the grid refined once more, where the midpoints of the
edges are points. -/
def double (ℓ : α → ℕ) : α → ℕ := fun a => 2 * ℓ a

/-- The midpoint of an edge of the grid at `ℓ`, a point of the grid at `double ℓ`. -/
def midpoint (e : Point Patch × Point Patch) : Point Patch :=
  ⟨e.1.patch, e.1.x + e.2.x, e.1.y + e.2.y⟩

variable (T)

/-- The number of vertices of the quad mesh: the classes of the points of the grid. -/
noncomputable def numVertices (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) : ℕ :=
  (T.vertexOf allArcs ℓ '' {p | p ∈ T.points patches ℓ}).ncard

/-- The number of edges of the quad mesh: the edges of the faces, two being one
edge when their midpoints are one vertex of the grid at `double ℓ`. -/
noncomputable def numEdges (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) : ℕ :=
  ((fun e => T.vertexOf allArcs (double ℓ) (midpoint e)) ''
    {e | e ∈ (T.faces patches ℓ).flatMap faceEdges}).ncard

/-- The number of faces of the quad mesh. -/
noncomputable def numFaces (patches : List Patch) (ℓ : α → ℕ) : ℕ :=
  {f | f ∈ T.faces patches ℓ}.ncard

/-- The Euler characteristic of the quad mesh at `ℓ`. -/
noncomputable def euler (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) : ℤ :=
  (T.numVertices patches allArcs ℓ : ℤ) - T.numEdges patches allArcs ℓ + T.numFaces patches ℓ

/-- The number of nodes of the T-mesh at `ℓ`: the classes of the ends of the arcs. -/
noncomputable def numNodes (allArcs : List α) (ℓ : α → ℕ) : ℕ :=
  (T.vertexOf allArcs ℓ '' {p | T.IsEnd ℓ p}).ncard

variable {T}

theorem mem_points {patches : List Patch} (hP : ∀ P, P ∈ patches) {ℓ : α → ℕ} {p : Point Patch} :
    p ∈ T.points patches ℓ ↔ T.InGrid ℓ p := by
  obtain ⟨P, x, y⟩ := p
  simp [points, InGrid, hP]

theorem mem_faces {patches : List Patch} (hP : ∀ P, P ∈ patches) {ℓ : α → ℕ} {f : Face Patch} :
    f ∈ T.faces patches ℓ ↔ f.i < 2 * T.width ℓ f.patch ∧ f.j < 2 * T.height ℓ f.patch := by
  obtain ⟨P, i, j⟩ := f
  simp [faces, hP]

theorem sideLen_double (ℓ : α → ℕ) (σ : Side Patch) :
    T.sideLen (double ℓ) σ = 2 * T.sideLen ℓ σ := by
  unfold sideLen double
  induction T.arcs σ with
  | nil => rfl
  | cons b l ih => simp only [List.map_cons, List.sum_cons, ih]; ring

theorem posNat_double (ℓ : α → ℕ) (a : α) (l : List α) :
    posNat a l (double ℓ) = 2 * posNat a l ℓ := by
  induction l with
  | nil => rfl
  | cons b l ih => simp only [posNat, double, ih]; split_ifs <;> ring

theorem width_double (ℓ : α → ℕ) (P : Patch) : T.width (double ℓ) P = 2 * T.width ℓ P :=
  sideLen_double ℓ _

theorem height_double (ℓ : α → ℕ) (P : Patch) : T.height (double ℓ) P = 2 * T.height ℓ P :=
  sideLen_double ℓ _

theorem balanced_double {ℓ : α → ℕ} (hbal : T.Balanced ℓ) : T.Balanced (double ℓ) := by
  intro P j; rw [sideLen_double, sideLen_double, hbal P j]

/-- The midpoints of the edges of the faces are the points of the grid at
`double ℓ` whose coordinates have an odd sum. -/
theorem midpoint_edges {patches : List Patch} (hP : ∀ P, P ∈ patches) {ℓ : α → ℕ}
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) :
    midpoint '' {e | e ∈ (T.faces patches ℓ).flatMap faceEdges} =
      {p | T.InGrid (double ℓ) p ∧ (p.x + p.y) % 2 = 1} := by
  ext ⟨P, X, Y⟩
  simp only [Set.mem_image, Set.mem_ofPred_eq, List.mem_flatMap, InGrid, width_double,
    height_double]
  constructor
  · rintro ⟨e, ⟨⟨Q, i, j⟩, hf, he⟩, h⟩
    rw [mem_faces hP] at hf
    simp only [faceEdges, List.mem_cons, List.not_mem_nil, or_false] at he
    rcases he with rfl | rfl | rfl | rfl <;> simp only [midpoint, Point.mk.injEq] at h <;>
      obtain ⟨rfl, rfl, rfl⟩ := h <;> simp only at hf ⊢ <;> omega
  · rintro ⟨⟨hX, hY⟩, hodd⟩
    have hm : ∀ i j, (⟨P, i, j⟩ : Face Patch) ∈ T.faces patches ℓ ↔
        i < 2 * T.width ℓ P ∧ j < 2 * T.height ℓ P := fun i j => mem_faces hP
    obtain ⟨i, rfl | rfl⟩ := Nat.even_or_odd' X <;> obtain ⟨j, rfl | rfl⟩ := Nat.even_or_odd' Y
    · omega
    · by_cases hi : i < 2 * T.width ℓ P
      · exact ⟨(⟨P, i, j + 1⟩, ⟨P, i, j⟩), ⟨⟨P, i, j⟩, (hm i j).mpr ⟨hi, by omega⟩,
          by simp [faceEdges]⟩, by simp only [midpoint, Point.mk.injEq, true_and]; omega⟩
      · obtain ⟨k, rfl⟩ : ∃ k, i = k + 1 := ⟨i - 1, by have := hwh P; omega⟩
        exact ⟨(⟨P, k + 1, j⟩, ⟨P, k + 1, j + 1⟩), ⟨⟨P, k, j⟩, (hm k j).mpr ⟨by omega, by omega⟩,
          by simp [faceEdges]⟩, by simp only [midpoint, Point.mk.injEq, true_and]; omega⟩
    · by_cases hj : j < 2 * T.height ℓ P
      · exact ⟨(⟨P, i, j⟩, ⟨P, i + 1, j⟩), ⟨⟨P, i, j⟩, (hm i j).mpr ⟨by omega, hj⟩,
          by simp [faceEdges]⟩, by simp only [midpoint, Point.mk.injEq, true_and]; omega⟩
      · obtain ⟨k, rfl⟩ : ∃ k, j = k + 1 := ⟨j - 1, by have := hwh P; omega⟩
        exact ⟨(⟨P, i + 1, k + 1⟩, ⟨P, i, k + 1⟩), ⟨⟨P, i, k⟩, (hm i k).mpr ⟨by omega, by omega⟩,
          by simp [faceEdges]⟩, by simp only [midpoint, Point.mk.injEq, true_and]; omega⟩
    · omega

section Euler

variable [Fintype Patch] [Fintype α] {patches : List Patch} {allArcs : List α}
  (hP : ∀ P, P ∈ patches) (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hne : ∀ σ, T.arcs σ ≠ [])
  (hbal : T.Balanced ℓ) (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P)
include hP hA hne hbal hwh

theorem numVertices_eq : T.numVertices patches allArcs ℓ =
    ∑ P, (2 * T.width ℓ P - 1) * (2 * T.height ℓ P - 1) + ∑ a, (2 * ℓ a - 1) +
      T.numNodes allArcs ℓ := by
  unfold numVertices
  rw [ncard_classes hA hne hbal hwh (T.points patches ℓ).finite_toSet
    (fun p hp => (mem_points hP).mp hp)
    (fun p q hp h => (mem_points hP).mpr ((inGrid_of_same hbal h).mp ((mem_points hP).mp hp)))]
  have hI : {p | p ∈ T.points patches ℓ} ∩ {p | T.IsInner ℓ p} =
      ↑(ptsOf fun P => Finset.Ioo 0 (2 * T.width ℓ P) ×ˢ Finset.Ioo 0 (2 * T.height ℓ P)) := by
    ext p
    simp only [Set.mem_inter_iff, Set.mem_ofPred_eq, mem_points hP, Finset.mem_coe, mem_ptsOf,
      Finset.mem_product, Finset.mem_Ioo, InGrid, IsInner]
    constructor <;> intro <;> omega
  have hX : {x : (_ : α) × ℕ | 0 < x.2 ∧ x.2 < 2 * ℓ x.1 ∧
      T.arcPoint ℓ x.1 0 x.2 ∈ {p | p ∈ T.points patches ℓ}} =
      ↑(Finset.univ.sigma fun a => Finset.Ioo 0 (2 * ℓ a)) := by
    ext ⟨a, t⟩
    simp only [Set.mem_ofPred_eq, mem_points hP, Finset.coe_sigma, Set.mem_sigma_iff,
      Finset.coe_univ,       Set.mem_univ, Finset.coe_Ioo, Set.mem_Ioo, true_and]
    exact ⟨fun h => ⟨h.1, h.2.1⟩, fun h => ⟨h.1, h.2, arcPoint_inGrid hbal a 0 h.2.le⟩⟩
  have hE : {p | p ∈ T.points patches ℓ} ∩ {p | T.IsEnd ℓ p} = {p | T.IsEnd ℓ p} := by
    refine Set.inter_eq_right.mpr ?_
    rintro p ⟨a, i, rfl | rfl⟩ <;> rw [Set.mem_ofPred_eq, mem_points hP]
    exacts [arcPoint_inGrid hbal a i (Nat.zero_le _), arcPoint_inGrid hbal a i le_rfl]
  rw [hI, hX, hE, Set.ncard_coe_finset, Set.ncard_coe_finset, card_ptsOf, Finset.card_sigma]
  simp [numNodes]

theorem numEdges_eq : T.numEdges patches allArcs ℓ =
    ∑ P, (2 * T.width ℓ P * (2 * T.height ℓ P - 1) + (2 * T.width ℓ P - 1) * (2 * T.height ℓ P)) +
      ∑ a, 2 * ℓ a := by
  have hbal₂ := balanced_double hbal
  have hwh₂ : ∀ P, 0 < T.width (double ℓ) P ∧ 0 < T.height (double ℓ) P := fun P => by
    rw [width_double, height_double]; have := hwh P; omega
  unfold numEdges
  rw [← Set.image_image, midpoint_edges hP hwh]
  have hfin : {p | T.InGrid (double ℓ) p ∧ (p.x + p.y) % 2 = 1}.Finite := by
    rw [← midpoint_edges hP hwh]; exact (List.finite_toSet _).image _
  rw [ncard_classes hA hne hbal₂ hwh₂ hfin (fun p hp => hp.1)
    (fun p q hp h => ⟨(inGrid_of_same hbal₂ h).mp hp.1, by rw [← parity_of_same h]; exact hp.2⟩)]
  have hI : {p | T.InGrid (double ℓ) p ∧ (p.x + p.y) % 2 = 1} ∩ {p | T.IsInner (double ℓ) p} =
      ↑(ptsOf fun P => (Finset.Ioo 0 (2 * (2 * T.width ℓ P)) ×ˢ
        Finset.Ioo 0 (2 * (2 * T.height ℓ P))).filter fun q => (q.1 + q.2) % 2 = 1) := by
    ext p
    simp only [Set.mem_inter_iff, Set.mem_ofPred_eq, Finset.mem_coe, mem_ptsOf,
      Finset.mem_filter, Finset.mem_product, Finset.mem_Ioo, InGrid, IsInner, width_double,
      height_double]
    constructor <;> intro <;> omega
  have hX : {x : (_ : α) × ℕ | 0 < x.2 ∧ x.2 < 2 * double ℓ x.1 ∧
      T.arcPoint (double ℓ) x.1 0 x.2 ∈ {p | T.InGrid (double ℓ) p ∧ (p.x + p.y) % 2 = 1}} =
      ↑(Finset.univ.sigma fun a => (Finset.Ioo 0 (2 * (2 * ℓ a))).filter (· % 2 = 1)) := by
    ext ⟨a, t⟩
    simp only [Set.mem_ofPred_eq, arcPoint_parity, Finset.coe_sigma, Set.mem_sigma_iff,
      Finset.coe_univ,       Set.mem_univ, Finset.coe_filter, Finset.mem_Ioo, true_and, double]
    exact ⟨fun h => ⟨⟨h.1, h.2.1⟩, h.2.2.2⟩,
      fun h => ⟨h.1.1, h.1.2, arcPoint_inGrid hbal₂ a 0 (by unfold double; omega), h.2⟩⟩
  have hE : {p | T.InGrid (double ℓ) p ∧ (p.x + p.y) % 2 = 1} ∩ {p | T.IsEnd (double ℓ) p} = ∅ := by
    rw [Set.eq_empty_iff_forall_notMem]
    rintro p ⟨⟨-, hodd⟩, a, i, rfl | rfl⟩ <;> rw [arcPoint_parity] at hodd <;> omega
  rw [hI, hX, hE, Set.image_empty, Set.ncard_empty, Set.ncard_coe_finset, Set.ncard_coe_finset,
    card_ptsOf, Finset.card_sigma]
  simp only [card_filter_odd_sum, card_filter_odd, add_zero]

omit hA in
theorem numFaces_eq : T.numFaces patches ℓ = ∑ P, 2 * T.width ℓ P * (2 * T.height ℓ P) := by
  have hinj : Function.Injective fun f : Face Patch => (⟨f.patch, f.i, f.j⟩ : Point Patch) := by
    rintro ⟨P, i, j⟩ ⟨Q, i', j'⟩ h
    simp only [Point.mk.injEq] at h
    obtain ⟨rfl, rfl, rfl⟩ := h
    rfl
  have himg : (fun f : Face Patch => (⟨f.patch, f.i, f.j⟩ : Point Patch)) ''
      {f | f ∈ T.faces patches ℓ} =
      ↑(ptsOf fun P => Finset.range (2 * T.width ℓ P) ×ˢ Finset.range (2 * T.height ℓ P)) := by
    ext ⟨P, x, y⟩
    simp only [Set.mem_image, Set.mem_ofPred_eq, mem_faces hP, Finset.mem_coe, mem_ptsOf,
      Finset.mem_product, Finset.mem_range]
    constructor
    · rintro ⟨⟨Q, i, j⟩, h, he⟩
      simp only [Point.mk.injEq] at he
      obtain ⟨rfl, rfl, rfl⟩ := he
      exact h
    · intro h; exact ⟨⟨P, x, y⟩, h, rfl⟩
  unfold numFaces
  rw [← Set.ncard_image_of_injective _ hinj, himg, Set.ncard_coe_finset, card_ptsOf]
  simp

end Euler

/-- **The Euler characteristic of the quad mesh** of a T-mesh at positive balanced
lengths is `P − A + N`: the patches minus the arcs plus the nodes. -/
theorem euler_eq [Fintype Patch] [Fintype α] {patches : List Patch} {allArcs : List α}
    (hP : ∀ P, P ∈ patches) (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a)
    (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ) :
    T.euler patches allArcs ℓ =
      (Fintype.card Patch : ℤ) - Fintype.card α + T.numNodes allArcs ℓ := by
  have hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P :=
    fun P => ⟨sideLen_pos hℓ hne _, sideLen_pos hℓ hne _⟩
  have cw : ∀ P, ((2 * T.width ℓ P - 1 : ℕ) : ℤ) = 2 * T.width ℓ P - 1 :=
    fun P => by have := (hwh P).1; omega
  have ch : ∀ P, ((2 * T.height ℓ P - 1 : ℕ) : ℤ) = 2 * T.height ℓ P - 1 :=
    fun P => by have := (hwh P).2; omega
  have cl : ∀ a, ((2 * ℓ a - 1 : ℕ) : ℤ) = 2 * ℓ a - 1 := fun a => by have := hℓ a; omega
  unfold euler
  rw [numVertices_eq hP hA hne hbal hwh, numEdges_eq hP hA hne hbal hwh,
    numFaces_eq hP hne hbal hwh]
  push_cast [cw, ch, cl]
  have hcard : (Fintype.card Patch : ℤ) = ∑ P : Patch,
      ((2 * (T.width ℓ P : ℤ) - 1) * (2 * T.height ℓ P - 1) -
        (2 * T.width ℓ P * (2 * T.height ℓ P - 1) + (2 * T.width ℓ P - 1) * (2 * T.height ℓ P)) +
        2 * T.width ℓ P * (2 * T.height ℓ P)) := by
    rw [Finset.sum_congr rfl (g := fun _ => (1 : ℤ)) fun P _ => by ring]; simp
  have hcardα : (Fintype.card α : ℤ) = ∑ a : α, (2 * (ℓ a : ℤ) - (2 * ℓ a - 1)) := by
    rw [Finset.sum_congr rfl (g := fun _ => (1 : ℤ)) fun a _ => by ring]; simp
  rw [hcard, hcardα]
  simp only [Finset.sum_add_distrib, Finset.sum_sub_distrib]
  ring


/-! ### The nodes without lengths

A node is named by a side and the index `k` of a junction between its arcs:
`0` at its first corner, `k` between its `k`-th and `k+1`-th arcs, the number of
its arcs at its last corner. Two junctions are one node when the gluing of an arc
takes an end of it to the other, or when they are one corner of a patch. -/

/-- The length of the first `k` arcs of a list. -/
def pre (ℓ : α → ℕ) (l : List α) (k : ℕ) : ℕ := ((l.take k).map ℓ).sum

theorem pre_idxOf (ℓ : α → ℕ) {a : α} {l : List α} (h : a ∈ l) :
    pre ℓ l (l.idxOf a) = posNat a l ℓ ∧ pre ℓ l (l.idxOf a + 1) = posNat a l ℓ + ℓ a := by
  induction l with
  | nil => simp at h
  | cons b l ih =>
    by_cases hb : b = a
    · subst hb; simp [pre, posNat]
    · have ha := (List.mem_cons.mp h).resolve_left (Ne.symm hb)
      have hidx : (b :: l).idxOf a = l.idxOf a + 1 := by simp [hb]
      obtain ⟨ih₁, ih₂⟩ := ih ha
      simp only [pre] at ih₁ ih₂ ⊢
      rw [hidx]
      simp only [List.take_succ_cons, List.map_cons, List.sum_cons, posNat, hb, ite_false]
      omega

theorem pre_lt {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a) {l : List α} {k k' : ℕ} (hk : k < k')
    (hk' : k' ≤ l.length) : pre ℓ l k < pre ℓ l k' := by
  induction l generalizing k k' with
  | nil => simp at hk'; omega
  | cons b l ih =>
    obtain ⟨m', rfl⟩ : ∃ m', k' = m' + 1 := ⟨k' - 1, by omega⟩
    have := hℓ b
    rcases k with _ | m
    · simp [pre]; omega
    · have := ih (k := m) (k' := m') (by omega) (by simp at hk'; omega)
      simp only [pre, List.take_succ_cons, List.map_cons, List.sum_cons] at this ⊢
      omega

theorem pre_inj {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a) {l : List α} {k k' : ℕ} (hk : k ≤ l.length)
    (hk' : k' ≤ l.length) (h : pre ℓ l k = pre ℓ l k') : k = k' := by
  rcases lt_trichotomy k k' with hlt | rfl | hlt
  · exact absurd h (pre_lt hℓ hlt hk').ne
  · rfl
  · exact absurd h (pre_lt hℓ hlt hk).ne'

theorem pre_length (ℓ : α → ℕ) (l : List α) : pre ℓ l l.length = (l.map ℓ).sum := by
  simp [pre]

/-- A corner of a patch: the patch and its place `(u, v) ∈ {0, 1}²`. -/
abbrev Corner (Patch : Type*) := Patch × Fin 2 × Fin 2

/-- The first (`e = 0`) or last (`e = 1`) corner of a side. -/
def sideCorner (σ : Side Patch) (e : Fin 2) : Corner Patch :=
  if σ.2.1 = 0 then (σ.1, e, σ.2.2) else (σ.1, σ.2.2, e)

variable (T)

/-- The junction at the first (`e = 0`) or last (`e = 1`) corner of a side. -/
def cornerJunction (σ : Side Patch) (e : Fin 2) : Side Patch × ℕ :=
  (σ, if e = 0 then 0 else (T.arcs σ).length)

/-- The junction at the start (`e = 0`) or the end (`e = 1`) of the arc `a` on its
side `ends a i`, in the direction of that side. -/
def endJunction (a : α) (i : Fin 2) (e : Fin 2) : Side Patch × ℕ :=
  (T.ends a i, (T.arcs (T.ends a i)).idxOf a + e.val)

/-- The identifications of junctions, without lengths: the gluing of an arc takes
its ends on one side to its ends on the other, and the junctions at one corner of
a patch are one. -/
inductive NodeStep : Side Patch × ℕ → Side Patch × ℕ → Prop
  | glue (a : α) (e : Fin 2) :
      NodeStep (T.endJunction a 0 e) (T.endJunction a 1 (if T.sign a = 1 then e else e + 1))
  | corner (σ τ : Side Patch) (e e' : Fin 2) (h : sideCorner σ e = sideCorner τ e') :
      NodeStep (T.cornerJunction σ e) (T.cornerJunction τ e')

/-- **The number of nodes of the T-mesh**: the classes of the ends of the arcs,
as junctions, under `NodeStep`. It does not depend on lengths. -/
noncomputable def nodeCount : ℕ :=
  (Quot.mk T.NodeStep '' Set.range fun x : α × Fin 2 × Fin 2 => T.endJunction x.1 x.2.1 x.2.2).ncard

/-- The point of a junction at the lengths `ℓ`. -/
def junctionPoint (ℓ : α → ℕ) (j : Side Patch × ℕ) : Point Patch :=
  T.sidePoint ℓ j.1 (2 * pre ℓ (T.arcs j.1) j.2)

/-- The point of a corner at the lengths `ℓ`. -/
def cornerPoint (ℓ : α → ℕ) (c : Corner Patch) : Point Patch :=
  ⟨c.1, 2 * T.width ℓ c.1 * c.2.1.val, 2 * T.height ℓ c.1 * c.2.2.val⟩

variable {T}

theorem junctionPoint_endJunction (ℓ : α → ℕ) (a : α) (i e : Fin 2) :
    T.junctionPoint ℓ (T.endJunction a i e) = T.arcPoint ℓ a i (2 * ℓ a * e.val) := by
  obtain ⟨h₀, h₁⟩ := pre_idxOf ℓ (mem_arcs_ends (T := T) a i)
  unfold junctionPoint endJunction arcPoint
  fin_cases e
  · simp only [add_zero, mul_zero, h₀]
  · simp only [mul_one, h₁]; congr 1; ring

theorem junctionPoint_cornerJunction {ℓ : α → ℕ} (hbal : T.Balanced ℓ) (σ : Side Patch)
    (e : Fin 2) : T.junctionPoint ℓ (T.cornerJunction σ e) = T.cornerPoint ℓ (sideCorner σ e) := by
  have hL := sideLen_eq hbal σ
  unfold junctionPoint cornerJunction
  obtain ⟨P, j, k⟩ := σ
  fin_cases e <;> simp only [Fin.zero_eta, Fin.mk_one, ite_true, ite_false, one_ne_zero] <;>
    simp only [pre, List.take_zero, List.map_nil, List.sum_nil] <;>
    fin_cases j <;> fin_cases k <;>
    simp_all [sidePoint, cornerPoint, sideCorner, sideLen, width, height]

theorem cornerPoint_inj {ℓ : α → ℕ} (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P)
    {c c' : Corner Patch} (h : T.cornerPoint ℓ c = T.cornerPoint ℓ c') : c = c' := by
  obtain ⟨P, x, y⟩ := c
  obtain ⟨Q, x', y'⟩ := c'
  simp only [cornerPoint, Point.mk.injEq] at h
  obtain ⟨rfl, hx, hy⟩ := h
  obtain ⟨hw, hh⟩ := hwh P
  have := Fin.isLt x; have := Fin.isLt x'; have := Fin.isLt y; have := Fin.isLt y'
  have hx' : x.val = x'.val := by
    rcases Nat.lt_or_ge x.val x'.val with h | h <;> [skip; skip] <;>
      nlinarith [hx]
  have hy' : y.val = y'.val := by
    rcases Nat.lt_or_ge y.val y'.val with h | h <;> nlinarith [hy]
  rw [Fin.ext hx', Fin.ext hy']

theorem pre_le_length {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a) {l : List α} {k : ℕ} (hk : k ≤ l.length) :
    pre ℓ l k ≤ (l.map ℓ).sum := by
  rw [← pre_length]
  rcases hk.lt_or_eq with h | rfl
  exacts [(pre_lt hℓ h le_rfl).le, le_rfl]

theorem endJunction_valid (a : α) (i e : Fin 2) :
    (T.endJunction a i e).2 ≤ (T.arcs (T.endJunction a i e).1).length := by
  have := List.idxOf_lt_length_of_mem (mem_arcs_ends (T := T) a i)
  have := e.isLt
  simp only [endJunction]; omega

theorem isEnd_junctionPoint (ℓ : α → ℕ) (a : α) (i e : Fin 2) :
    T.IsEnd ℓ (T.junctionPoint ℓ (T.endJunction a i e)) := by
  rw [junctionPoint_endJunction]
  fin_cases e
  · exact ⟨a, i, Or.inl (by simp)⟩
  · exact ⟨a, i, Or.inr (by simp)⟩

/-- **Two junctions with one point are one node**, at positive balanced lengths. -/
theorem nodeStep_of_point {ℓ : α → ℕ} (hℓ : ∀ a, 0 < ℓ a) (hbal : T.Balanced ℓ)
    (hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P) {j j' : Side Patch × ℕ}
    (hj : j.2 ≤ (T.arcs j.1).length) (hj' : j'.2 ≤ (T.arcs j'.1).length)
    (h : T.junctionPoint ℓ j = T.junctionPoint ℓ j') : Relation.EqvGen T.NodeStep j j' := by
  have key : ∀ {σ τ : Side Patch} {k k' : ℕ}, k' ≤ (T.arcs τ).length → 0 < k →
      k < (T.arcs σ).length → T.junctionPoint ℓ (σ, k) = T.junctionPoint ℓ (τ, k') →
      σ = τ ∧ k = k' := by
    intro σ τ k k' hk' hk₀ hk h
    simp only [junctionPoint] at h
    have h₀ : pre ℓ (T.arcs σ) 0 < pre ℓ (T.arcs σ) k := pre_lt hℓ hk₀ hk.le
    rw [show pre ℓ (T.arcs σ) 0 = 0 by simp [pre]] at h₀
    have h₁ := pre_lt hℓ (l := T.arcs σ) hk le_rfl
    have h₂ := pre_le_length hℓ hk'
    rw [pre_length] at h₁
    obtain ⟨rfl, huv⟩ := sidePoint_inj hbal hwh (by omega) (by unfold sideLen; omega)
      (by unfold sideLen; omega) h
    exact ⟨rfl, pre_inj hℓ hk.le hk' (by omega)⟩
  have corner : ∀ {σ : Side Patch} {k : ℕ}, (k = 0 ∨ k = (T.arcs σ).length) →
      ∃ e, (σ, k) = T.cornerJunction σ e := by
    rintro σ k (rfl | rfl)
    exacts [⟨0, by simp [cornerJunction]⟩, ⟨1, by simp [cornerJunction]⟩]
  obtain ⟨σ, k⟩ := j
  obtain ⟨τ, k'⟩ := j'
  by_cases hk : 0 < k ∧ k < (T.arcs σ).length
  · obtain ⟨rfl, rfl⟩ := key hj' hk.1 hk.2 h
    exact Relation.EqvGen.refl _
  by_cases hk' : 0 < k' ∧ k' < (T.arcs τ).length
  · obtain ⟨rfl, rfl⟩ := key hj hk'.1 hk'.2 h.symm
    exact Relation.EqvGen.refl _
  obtain ⟨e, he⟩ := corner (σ := σ) (k := k) (by simp only at hj; omega)
  obtain ⟨e', he'⟩ := corner (σ := τ) (k := k') (by simp only at hj'; omega)
  rw [he, he'] at h ⊢
  rw [junctionPoint_cornerJunction hbal, junctionPoint_cornerJunction hbal] at h
  exact Relation.EqvGen.rel _ _ (NodeStep.corner σ τ e e' (cornerPoint_inj hwh h))

theorem glued_ends (ℓ : α → ℕ) (a : α) (e : Fin 2) :
    T.arcPoint ℓ a 1 (T.flip ℓ a (2 * ℓ a * e.val)) =
      T.arcPoint ℓ a 1 (2 * ℓ a * (if T.sign a = 1 then e else e + 1).val) := by
  congr 1
  unfold flip
  fin_cases e <;> split_ifs <;> simp

/-- The points of the two ends of a gluing step are one vertex. -/
theorem same_of_nodeStep {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hbal : T.Balanced ℓ) {j j' : Side Patch × ℕ} (h : T.NodeStep j j') :
    T.Same allArcs ℓ (T.junctionPoint ℓ j) (T.junctionPoint ℓ j') := by
  cases h with
  | glue a e =>
    have ht : 2 * ℓ a * e.val ≤ 2 * ℓ a := by fin_cases e <;> simp
    have := T.same_of_glued allArcs ℓ a (hA a) _ ht
    rwa [glued_eq ℓ a ht, glued_ends, ← junctionPoint_endJunction,
      ← junctionPoint_endJunction] at this
  | corner σ τ e e' hc =>
    rw [junctionPoint_cornerJunction hbal, junctionPoint_cornerJunction hbal, hc]
    exact Relation.EqvGen.refl _

/-- **The nodes at positive balanced lengths are the nodes without lengths.** -/
theorem numNodes_eq [Fintype α] {allArcs : List α} (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hℓ : ∀ a, 0 < ℓ a) (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ) :
    T.numNodes allArcs ℓ = T.nodeCount := by
  have hwh : ∀ P, 0 < T.width ℓ P ∧ 0 < T.height ℓ P :=
    fun P => ⟨sideLen_pos hℓ hne _, sideLen_pos hℓ hne _⟩
  set E := Set.range fun x : α × Fin 2 × Fin 2 => T.endJunction x.1 x.2.1 x.2.2
  have hEv : ∀ j ∈ E, j.2 ≤ (T.arcs j.1).length := by
    rintro _ ⟨⟨a, i, e⟩, rfl⟩; exact endJunction_valid a i e
  have hEnd : {p | T.IsEnd ℓ p} = T.junctionPoint ℓ '' E := by
    ext p
    constructor
    · rintro ⟨a, i, rfl | rfl⟩
      · exact ⟨T.endJunction a i 0, ⟨(a, i, 0), rfl⟩, by simp [junctionPoint_endJunction]⟩
      · exact ⟨T.endJunction a i 1, ⟨(a, i, 1), rfl⟩, by simp [junctionPoint_endJunction]⟩
    · rintro ⟨_, ⟨⟨a, i, e⟩, rfl⟩, rfl⟩
      exact isEnd_junctionPoint ℓ a i e
  -- The class of a point of a junction of `E` holds only points of junctions of `E`
  -- related to it.
  have hclosed : ∀ j₀ ∈ E, ∀ q, T.Same allArcs ℓ (T.junctionPoint ℓ j₀) q →
      ∃ j ∈ E, T.junctionPoint ℓ j = q ∧ Relation.EqvGen T.NodeStep j₀ j := by
    intro j₀ hj₀ q hq
    refine (same_iff_of_gluings (S := fun p => ∃ j ∈ E, T.junctionPoint ℓ j = p ∧
      Relation.EqvGen T.NodeStep j₀ j) (fun x y hxy => ?_) hq).mp
      ⟨j₀, hj₀, rfl, Relation.EqvGen.refl _⟩
    obtain ⟨b, t, ht, rfl, rfl⟩ := mem_gluings hxy
    by_cases hte : t = 2 * ℓ b * (0 : Fin 2).val ∨ t = 2 * ℓ b * (1 : Fin 2).val
    · obtain ⟨e, rfl⟩ : ∃ e : Fin 2, t = 2 * ℓ b * e.val := hte.elim (⟨0, ·⟩) (⟨1, ·⟩)
      have hstep := NodeStep.glue (T := T) b e
      rw [glued_ends, ← junctionPoint_endJunction, ← junctionPoint_endJunction]
      have hv₀ := endJunction_valid (T := T) b 0 e
      have hv₁ := endJunction_valid (T := T) b 1 (if T.sign b = 1 then e else e + 1)
      constructor
      · rintro ⟨j, hj, hjp, hr⟩
        exact ⟨_, ⟨(b, 1, if T.sign b = 1 then e else e + 1), rfl⟩, rfl,
          hr.trans _ _ _ ((nodeStep_of_point hℓ hbal hwh (hEv j hj) hv₀
          hjp).trans _ _ _ (Relation.EqvGen.rel _ _ hstep))⟩
      · rintro ⟨j, hj, hjp, hr⟩
        exact ⟨_, ⟨(b, 0, e), rfl⟩, rfl,
          hr.trans _ _ _ ((nodeStep_of_point hℓ hbal hwh (hEv j hj) hv₁
          hjp).trans _ _ _ (Relation.EqvGen.symm _ _ (Relation.EqvGen.rel _ _ hstep)))⟩
    · simp only [Fin.val_zero, mul_zero, Fin.val_one, mul_one] at hte
      obtain ⟨hf₀, hf⟩ := flip_inner (T := T) (ℓ := ℓ) (a := b) (t := t) (by omega) (by omega)
      refine iff_of_false ?_ ?_ <;> rintro ⟨_, ⟨⟨a, i, e⟩, rfl⟩, hjp, -⟩ <;>
        have := isEnd_junctionPoint (T := T) ℓ a i e <;> rw [hjp] at this
      exacts [not_isEnd hbal hwh (by omega) (by omega) this, not_isEnd hbal hwh hf₀ hf this]
  let φ : Quot T.NodeStep → T.Vertex allArcs ℓ :=
    Quot.lift (fun j => T.vertexOf allArcs ℓ (T.junctionPoint ℓ j))
      fun _ _ h => vertexOf_eq_iff.mpr (same_of_nodeStep hA hbal h)
  have himg : T.vertexOf allArcs ℓ '' {p | T.IsEnd ℓ p} = φ '' (Quot.mk T.NodeStep '' E) := by
    rw [hEnd, Set.image_image, Set.image_image]
  unfold numNodes nodeCount
  rw [himg]
  refine Set.InjOn.ncard_image ?_
  rintro _ ⟨j, hj, rfl⟩ _ ⟨j', hj', rfl⟩ h
  obtain ⟨j'', hj'', hp, hr⟩ := hclosed j hj _ (vertexOf_eq_iff.mp h)
  exact Quot.eqvGen_sound (hr.trans _ _ _ (nodeStep_of_point hℓ hbal hwh (hEv _ hj'')
    (hEv _ hj') hp))

/-- **The Euler characteristic of the quad mesh does not depend on the lengths**: at
positive balanced lengths it is the patches minus the arcs plus the nodes. -/
theorem euler_eq_nodeCount [Fintype Patch] [Fintype α] {patches : List Patch}
    {allArcs : List α} (hP : ∀ P, P ∈ patches) (hA : ∀ a, a ∈ allArcs) {ℓ : α → ℕ}
    (hℓ : ∀ a, 0 < ℓ a) (hne : ∀ σ, T.arcs σ ≠ []) (hbal : T.Balanced ℓ) :
    T.euler patches allArcs ℓ = (Fintype.card Patch : ℤ) - Fintype.card α + T.nodeCount := by
  rw [euler_eq hP hA hℓ hne hbal, numNodes_eq hA hℓ hne hbal]

end TMesh

end TurnsOfATMesh
