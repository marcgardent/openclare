/-
Proposition "Completeness at given lengths" on the `TMesh` (DT-099): the gluing
data of `Completeness.lean` read off the T-mesh at whole lengths instead of
assumed.

`Completeness.lean` takes the gluing on unit edges as data: for each side `τ` and
unit edge `k`, the side it is glued to, a sign and a whole translation, and the
walk `realWalk` reads them at `⌊t⌋`. Here they come from the T-mesh. A side is the
list of its arcs, and at whole lengths the nodes sit at the partial sums of their
lengths, whole numbers; so every point of the open unit edge `(k, k + 1)` lies in
one arc, the arc `arcOfEdge` finds from the partial sums (`arcAt_of_unit_edge`),
and the walk of Definition "Turn" (`TMesh.step`, which looks for the arc whose
interior holds the offset) crosses it as `t ↦ ε t + c` with `ε` its sign and `c`
the whole `crossTrans` (`step_eq_realWalk`). At balanced whole lengths the data
form a closed gluing (`closedGluing`): the opposite of the opposite side is the
side, and crossing an arc back returns to the unit edge one started from
(`crossArc_back`). So `completeness_at_given_lengths` applies to the T-mesh, with
the walk of the T-mesh itself from the half-integers (`completeness_tmesh`).

Still assumed, as in `Completeness.lean`: the depths of the columns, and that the
darts and `next` built on them are the quads and strips of the quad mesh.
-/
module

public import TurnsOfATMesh.Completeness
public import TurnsOfATMesh.Border

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

open Completeness

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α]

/-! ### The arc over a unit edge -/

/-- The arc of a side (whose arcs are `l`) that holds the unit edge `[k, k + 1]`, at
whole lengths: the nodes are the partial sums of the lengths. `none` off the
side. -/
def arcOfEdge (ℓ : α → ℕ) : List α → ℤ → Option α
  | [], _ => none
  | b :: l, k => if 0 ≤ k ∧ k < ℓ b then some b else arcOfEdge ℓ l (k - ℓ b)

/-- A whole number lies below a point of the open unit edge `(k, k + 1)` exactly when
it is at most `k`. -/
theorem lt_of_unit_edge {k m : ℤ} {t : ℝ} (h₁ : (k : ℝ) < t) (h₂ : t < k + 1) :
    (m : ℝ) < t ↔ m ≤ k := by
  constructor
  · intro h
    have : (m : ℝ) < k + 1 := h.trans h₂
    have : m < k + 1 := by exact_mod_cast this
    omega
  · intro h
    exact lt_of_le_of_lt (by exact_mod_cast h) h₁

/-- A point of the open unit edge `(k, k + 1)` lies below a whole number exactly when
`k` does. -/
theorem unit_edge_lt {k m : ℤ} {t : ℝ} (h₁ : (k : ℝ) < t) (h₂ : t < k + 1) :
    t < m ↔ k < m := by
  constructor
  · intro h
    exact_mod_cast h₁.trans h
  · intro h
    have : k + 1 ≤ m := h
    exact lt_of_lt_of_le h₂ (by exact_mod_cast this)

/-- **The arc over an open unit edge.** At whole lengths every point `t` of the open
unit edge `(k, k + 1)` of a side lies in the interior of the arc that holds the
unit edge: the walk of Definition "Turn" crosses the arc `arcOfEdge` reads from the
partial sums. -/
theorem arcAt_of_unit_edge (ℓ : α → ℕ) (l : List α) (k : ℤ) {t : ℝ} (h₁ : (k : ℝ) < t)
    (h₂ : t < k + 1) : arcAt (fun b => (ℓ b : ℝ)) l t = arcOfEdge ℓ l k := by
  induction l generalizing k t with
  | nil => rfl
  | cons b l ih =>
    have e₁ : (0 : ℝ) < t ↔ 0 ≤ k := by simpa using lt_of_unit_edge (m := 0) h₁ h₂
    have e₂ : t < (ℓ b : ℝ) ↔ k < ℓ b := by simpa using unit_edge_lt (m := (ℓ b : ℤ)) h₁ h₂
    simp only [arcAt, arcOfEdge, e₁, e₂]
    split_ifs
    · rfl
    · exact ih (k - ℓ b) (by push_cast; linarith) (by push_cast; linarith)

/-- Every unit edge of a side lies in one of its arcs. -/
theorem exists_arcOfEdge (ℓ : α → ℕ) (l : List α) {k : ℤ} (h₀ : 0 ≤ k)
    (h₁ : k < (l.map ℓ).sum) : ∃ a, arcOfEdge ℓ l k = some a := by
  induction l generalizing k with
  | nil => simp at h₁; omega
  | cons b l ih =>
    simp only [List.map_cons, List.sum_cons, Nat.cast_add] at h₁
    simp only [arcOfEdge]
    split_ifs with h
    · exact ⟨b, rfl⟩
    · exact ih (by omega) (by omega)

/-- The arc found over the unit edge `k` of a side is the arc `a` of the side with
`pos a ≤ k < pos a + ℓ a`, the position `pos a` the sum of the lengths before it. -/
theorem arcOfEdge_eq_some (ℓ : α → ℕ) {l : List α} (hl : l.Nodup) (k : ℤ) (a : α) :
    arcOfEdge ℓ l k = some a ↔ a ∈ l ∧ (posNat a l ℓ : ℤ) ≤ k ∧ k < posNat a l ℓ + ℓ a := by
  induction l generalizing k with
  | nil => simp [arcOfEdge]
  | cons b l ih =>
    rw [List.nodup_cons] at hl
    by_cases hab : b = a
    · subst hab
      simp only [arcOfEdge, posNat, ite_true, List.mem_cons, true_or, true_and, Nat.cast_zero,
        zero_add]
      split_ifs with h
      · simp [h]
      · rw [ih hl.2]
        constructor
        · rintro ⟨hb, -⟩; exact absurd hb hl.1
        · intro h'; exact absurd h' h
    · simp only [arcOfEdge, posNat, hab, ite_false, List.mem_cons, Ne.symm hab, false_or,
        Nat.cast_add]
      split_ifs with h
      · simp only [Option.some.injEq, hab, false_iff, not_and, not_lt]
        intro _ _
        omega
      · rw [ih hl.2]
        constructor <;> rintro ⟨hm, h₁, h₂⟩ <;> exact ⟨hm, by omega, by omega⟩

/-! ### The gluing data of the T-mesh on unit edges -/

variable (T : TMesh Patch α) [Fintype α] (ℓ : α → ℕ)

/-- The side the unit edge `k` of `τ` is glued to: the other side of the arc
holding it. -/
def edgeTarget (τ : Side Patch) (k : ℤ) : Side Patch := (arcOfEdge ℓ (T.arcs τ) k).elim τ (T.other · τ)

/-- The sign of the gluing of the unit edge `k` of `τ`: that of the arc holding it. -/
def edgeSign (τ : Side Patch) (k : ℤ) : ℤ := (arcOfEdge ℓ (T.arcs τ) k).elim 1 T.sign

/-- The whole translation of the gluing of the unit edge `k` of `τ`: that of the
crossing of the arc holding it, `crossTrans`. -/
def edgeShift (τ : Side Patch) (k : ℤ) : ℤ := (arcOfEdge ℓ (T.arcs τ) k).elim 0 (T.crossTrans ℓ τ)

variable {T ℓ}

/-- The sign of the gluing of a unit edge is `±1`. -/
theorem edgeSign_isSign (τ : Side Patch) (k : ℤ) :
    T.edgeSign ℓ τ k = 1 ∨ T.edgeSign ℓ τ k = -1 := by
  unfold edgeSign
  cases arcOfEdge ℓ (T.arcs τ) k with
  | none => exact Or.inl rfl
  | some a => exact T.sign_isSign a

/-- Across the arc `a` holding it, the unit edge `k` of `τ` goes to the other side of
`a`, by `glueEdge` with the sign of `a` and `crossTrans`. -/
theorem crossArc_eq {τ : Side Patch} {k : ℤ} {a : α} (h : arcOfEdge ℓ (T.arcs τ) k = some a) :
    crossArc (T.edgeTarget ℓ) (T.edgeSign ℓ) (T.edgeShift ℓ) (τ, k) =
      (T.other a τ, glueEdge (T.sign a) (T.crossTrans ℓ τ a) k) := by
  simp [crossArc, edgeTarget, edgeSign, edgeShift, h]

/-- The position of an arc at whole lengths, read in the reals, is `posNat`. -/
theorem posAt_natCast (a : α) (l : List α) :
    posAt a l (fun b => (ℓ b : ℝ)) = (posNat a l ℓ : ℝ) := by
  induction l with
  | nil => simp [posAt, posNat]
  | cons b l ih =>
    simp only [posAt, posNat]
    split_ifs
    · simp
    · simp [ih]

/-- At whole lengths, crossing the arc `a` from the side `τ` is `t ↦ ε t + c` with
`ε` the sign of `a` and `c = crossTrans` whole. -/
theorem crossAt_apply_natCast (τ : Side Patch) (a : α) (t : ℝ) :
    (T.crossAt (fun b => (ℓ b : ℝ)) τ a).apply t = glueLine (T.sign a) (T.crossTrans ℓ τ a) t := by
  unfold crossAt crossTrans glueLine Motion.apply
  split_ifs with h
  · simp [h, posAt_natCast]
  · have h' : T.sign a = -1 := (T.sign_isSign a).resolve_left h
    simp [h', posAt_natCast]

/-- **One move of the walk of the T-mesh is the move of `realWalk`** on the data read
off it, from any point of an open unit edge that lies on the opposite side: the
arc crossed is derived from the partial sums, not read at `⌊t⌋` by definition. -/
theorem step_eq_realWalk (σ : Side Patch) (k : ℤ) {t : ℝ} (h₁ : (k : ℝ) < t) (h₂ : t < k + 1)
    {a : α} (ha : arcOfEdge ℓ (T.arcs σ.opp) k = some a) :
    T.step (fun b => (ℓ b : ℝ)) (σ, t) =
      some (realWalk Side.opp (T.edgeTarget ℓ) (T.edgeSign ℓ) (T.edgeShift ℓ) (σ, t)) := by
  have hfl : ⌊t⌋ = k := by rw [Int.floor_eq_iff]; exact ⟨h₁.le, h₂⟩
  simp only [step, realWalk, arcAt_of_unit_edge ℓ _ k h₁ h₂, ha, Option.map_some, hfl,
    edgeTarget, edgeSign, edgeShift, Option.elim, crossAt_apply_natCast]

/-! ### A closed gluing -/

variable [Fintype Patch]

/-- The opposite of the opposite side is the side. -/
theorem opp_involutive : Function.Involutive (Side.opp : Side Patch → Side Patch) := by
  rintro ⟨P, j, s⟩
  simp only [Side.opp, Prod.mk.injEq, true_and]
  revert s
  decide

/-- At balanced lengths opposite sides have the same length. -/
theorem sideLen_opp (hbal : T.Balanced ℓ) (σ : Side Patch) : T.sideLen ℓ σ.opp = T.sideLen ℓ σ := by
  obtain ⟨P, j, s⟩ := σ
  fin_cases s
  · exact (hbal P j).symm
  · exact hbal P j

/-- **Crossing an arc back.** The unit edge `k` of `τ`, in the arc `a`, is glued to a
unit edge of the other side of `a`, in the same arc, and crossing `a` from there
returns to `k`. -/
theorem crossArc_back {τ : Side Patch} {k : ℤ} {a : α} (h : arcOfEdge ℓ (T.arcs τ) k = some a) :
    arcOfEdge ℓ (T.arcs (T.other a τ)) (glueEdge (T.sign a) (T.crossTrans ℓ τ a) k) = some a ∧
      crossArc (T.edgeTarget ℓ) (T.edgeSign ℓ) (T.edgeShift ℓ)
        (T.other a τ, glueEdge (T.sign a) (T.crossTrans ℓ τ a) k) = (τ, k) := by
  obtain ⟨hmem, hk₀, hk₁⟩ := (arcOfEdge_eq_some ℓ (T.arcs_nodup τ) k a).mp h
  have hmem' := T.mem_arcs_other hmem
  have hback := T.other_other hmem
  have h' : arcOfEdge ℓ (T.arcs (T.other a τ)) (glueEdge (T.sign a) (T.crossTrans ℓ τ a) k) =
      some a := by
    rw [arcOfEdge_eq_some ℓ (T.arcs_nodup _)]
    refine ⟨hmem', ?_⟩
    unfold glueEdge crossTrans
    split_ifs <;> omega
  refine ⟨h', ?_⟩
  rw [crossArc_eq h', hback]
  congr 1
  unfold glueEdge crossTrans
  rw [hback]
  split_ifs <;> omega

/-- **The gluing of the T-mesh at balanced whole lengths is closed**: the stations are
the unit edges of the sides, of `sideLen` each. -/
theorem closedGluing (hbal : T.Balanced ℓ) :
    ClosedGluing (T.sideLen ℓ) Side.opp (T.edgeTarget ℓ) (T.edgeSign ℓ) (T.edgeShift ℓ) := by
  refine ClosedGluing.of_involutive opp_involutive (sideLen_opp hbal) (fun τ k h₀ h₁ => ?_)
    (fun τ k h₀ h₁ => ?_)
  · obtain ⟨a, ha⟩ := exists_arcOfEdge ℓ (T.arcs τ) h₀ h₁
    obtain ⟨h', -⟩ := crossArc_back ha
    rw [crossArc_eq ha]
    obtain ⟨hm, hk₀, hk₁⟩ := (arcOfEdge_eq_some ℓ (T.arcs_nodup _) _ a).mp h'
    have := posNat_add_le ℓ hm
    unfold sideLen
    dsimp only
    constructor <;> omega
  · obtain ⟨a, ha⟩ := exists_arcOfEdge ℓ (T.arcs τ) h₀ h₁
    rw [crossArc_eq ha]
    exact (crossArc_back ha).2

/-! ### Completeness on the T-mesh -/

variable (T ℓ) in
/-- The depth of the column above a station: the length of the sides of the other
pair of its patch, the number of quads across the patch along the station. -/
def columnDepth (s : Σ σ : Side Patch, Fin (T.sideLen ℓ σ)) : ℕ := T.sideLen ℓ (s.1.1, s.1.2.1 + 1, 0)

/-- The columns have positive depth when every arc is positive and every side holds
an arc. -/
theorem columnDepth_pos (hℓ : ∀ a, 0 < ℓ a) (hne : ∀ σ, T.arcs σ ≠ [])
    (s : Σ σ : Side Patch, Fin (T.sideLen ℓ σ)) : 0 < T.columnDepth ℓ s :=
  sideLen_pos hℓ hne _

/-- **The walk of the T-mesh from a half-integer is the station walk**: from the
midpoint `k + ½` of a unit edge of a side, at balanced whole lengths, the walk of
Definition "Turn" is defined at every step and visits the midpoints of the stations
the station walk of the closed gluing gives. -/
theorem walk_stations (hbal : T.Balanced ℓ) (n : ℕ) (s : Σ σ : Side Patch, Fin (T.sideLen ℓ σ)) :
    T.walk (fun b => (ℓ b : ℝ)) n (s.1, (s.2.val : ℝ) + 1 / 2) =
      some (((stationPerm (T.closedGluing hbal) ^ n) s).1,
        (((stationPerm (T.closedGluing hbal) ^ n) s).2.val : ℝ) + 1 / 2) := by
  have hc := T.closedGluing hbal
  induction n generalizing s with
  | zero => rfl
  | succ n ih =>
    obtain ⟨a, ha⟩ := exists_arcOfEdge ℓ (T.arcs s.1.opp) (k := s.2.val) (by positivity)
      (by have := s.2.isLt; have h := sideLen_opp hbal s.1; unfold sideLen at this h; omega)
    have hstep := T.step_eq_realWalk (ℓ := ℓ) s.1 s.2.val (t := (s.2.val : ℝ) + 1 / 2)
      (by push_cast; linarith) (by push_cast; linarith) ha
    have h1 := realWalk_stations (edgeSign_isSign (T := T) (ℓ := ℓ)) hc.mapsStations 1 s
    simp only [Function.iterate_one] at h1
    rw [← stationPerm_pow hc 1, pow_one] at h1
    rw [walk, hstep, Option.bind_some, h1]
    exact (ih (stationPerm hc s)).trans (by rw [pow_succ, Equiv.Perm.mul_apply])

/-- **Proposition "Completeness at given lengths" on the T-mesh**, at balanced whole
lengths. The strips (the cycles of the successor `next` on the quads above the
stations, the columns of `depth s > 0` quads) are in bijection with the cycles of
the station walk of the T-mesh, and two stations lie on one cycle exactly when the
walk of Definition "Turn" from the midpoint of the one reaches the midpoint of the
other. -/
theorem completeness_tmesh (hbal : T.Balanced ℓ)
    (depth : (Σ σ : Side Patch, Fin (T.sideLen ℓ σ)) → ℕ) (hdepth : ∀ s, 0 < depth s) :
    ∃ e : Quotient (Equiv.Perm.SameCycle.setoid (stationPerm (T.closedGluing hbal))) ≃
        Quotient (Equiv.Perm.SameCycle.setoid
          (nextPerm (stationPerm (T.closedGluing hbal)) depth hdepth)),
      (∀ s, e ⟦s⟧ = ⟦Completeness.station depth hdepth s⟧) ∧
      ∀ s s', (stationPerm (T.closedGluing hbal)).SameCycle s s' ↔
        ∃ n : ℕ, T.walk (fun b => (ℓ b : ℝ)) n (s.1, (s.2.val : ℝ) + 1 / 2) =
          some (s'.1, (s'.2.val : ℝ) + 1 / 2) := by
  refine ⟨stripsEquiv (walk := stationPerm (T.closedGluing hbal)) (hdepth := hdepth),
    fun _ => rfl, fun s s' => ?_⟩
  simp only [walk_stations hbal, Option.some.injEq, Prod.mk.injEq, add_left_inj, Nat.cast_inj]
  constructor
  · intro h
    obtain ⟨n, hn⟩ := h.exists_nat_pow_eq
    exact ⟨n, by rw [hn], by rw [hn]⟩
  · rintro ⟨n, h1, h2⟩
    exact ⟨n, by rw [zpow_natCast]; exact Sigma.ext h1 ((Fin.heq_ext_iff (by rw [h1])).mpr h2)⟩

end TMesh

end TurnsOfATMesh
