/-
Proposition "Parity of a turn on a sphere" on the `TMesh` (DT-173): if constants
`κ` colour the quad graph at whole lengths `ℓ` (`IsColouring`), the length and the
shift of a turn followed by the walk from a half-integer station have the same
parity (`parity_of_turn_of_colouring`).

The proof follows the vertex beside the station (`stationVertex`) round the walk.
Across a patch its colour changes by the length across (`colour_opp`). Across an
arc of sign `+1` it is glued to the vertex beside the new station, of the same
colour; across an arc of sign `-1` to the vertex just after it, of the other colour
(`colour_cross`). Along a side the colour changes at each station
(`colour_stationVertex`). On an oriented T-mesh the product of the signs of a turn
is `1`, so the arcs of sign `-1` are even in number (`negCount_turn`).

`parity_of_turn_tmesh_any_sign` lifts it to whole balanced lengths of any sign,
as `Checkerboard.parity_of_turn_any_sign` does. Assumed there, not proved: that the
quad graph is bipartite at every positive whole balanced `ℓ` (a sphere) and that
some half-integer station follows the turn.
-/
module

public import TurnsOfATMesh.Grid
public import TurnsOfATMesh.Checkerboard

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### Casts of the forms -/

theorem evalForm_lengthForm_ring [Fintype α] {K : Type*} [CommRing K] (l : List α) (ℓ : α → K) :
    evalForm (lengthForm l) ℓ = (l.map ℓ).sum := by
  induction l with
  | nil => simp [lengthForm]
  | cons b l ih => simp [lengthForm, evalForm_add, evalForm_single, ih]

theorem evalForm_cast [Fintype α] {K : Type*} [CommRing K] (c : α → ℤ) (ℓ : α → ℕ) :
    ((evalForm c (fun a => (ℓ a : ℤ)) : ℤ) : K) = evalForm c (fun a => (ℓ a : K)) := by
  simp [evalForm]

theorem posAt_rat (a : α) (l : List α) (ℓ : α → ℕ) : posAt a l (rat ℓ) = (posNat a l ℓ : ℚ) := by
  induction l with
  | nil => simp [posAt, posNat]
  | cons b l ih =>
    simp only [posAt, posNat]
    split_ifs
    · simp
    · simp [ih, rat]

/-! ### The colour along a side and across a patch -/

variable {ℓ : α → ℕ} {κ : Patch → ZMod 2}

theorem sidePoint_isCoarse (σ : Side Patch) {t : ℕ} (ht : t % 2 = 0) :
    (T.sidePoint ℓ σ t).IsCoarse := by
  unfold sidePoint Point.IsCoarse
  split_ifs <;> simp <;> omega

/-- Along a side, the colour of the vertex beside the station `k + ½` changes with
`k`. -/
theorem colour_stationVertex (σ : Side Patch) (k : ℕ) :
    pointColour κ (T.stationVertex ℓ σ k) = (k : ZMod 2) + pointColour κ (T.stationVertex ℓ σ 0) := by
  unfold stationVertex sidePoint pointColour
  split_ifs <;> simp <;> ring

/-- **Across a patch** the colour changes by the length across. -/
theorem colour_opp [Fintype α] (σ : Side Patch) (k : ℕ) :
    pointColour κ (T.stationVertex ℓ σ.opp k) =
      pointColour κ (T.stationVertex ℓ σ k) + evalForm (T.acrossForm σ) (fun a => (ℓ a : ZMod 2)) := by
  obtain ⟨P, j, s⟩ := σ
  rw [acrossForm, evalForm_lengthForm_ring, show (fun a => (ℓ a : ZMod 2)) = Nat.cast ∘ ℓ from rfl,
    ← List.map_map, ← Nat.cast_list_sum]
  fin_cases j <;> fin_cases s <;>
    simp [stationVertex, sidePoint, Side.opp, pointColour, height, width, sideLen] <;>
    first | ring1 | (ring_nf; simp [show (2 : ZMod 2) = 0 by decide])

theorem colour_succ (σ : Side Patch) (k : ℕ) :
    pointColour κ (T.stationVertex ℓ σ (k + 1)) = pointColour κ (T.stationVertex ℓ σ k) + 1 := by
  rw [T.colour_stationVertex σ (k + 1), T.colour_stationVertex σ k]
  push_cast
  ring

/-! ### Across an arc -/

theorem other_ends_zero (a : α) : T.other a (T.ends a 0) = T.ends a 1 := by simp [other]

theorem other_ends_one (a : α) : T.other a (T.ends a 1) = T.ends a 0 := by
  simp [other, T.ends_ne a]

/-- The two points of a gluing pair at an even fine position have the same
colour. -/
theorem colour_glued {allArcs : List α} (hall : ∀ a, a ∈ allArcs)
    (hκ : T.IsColouring allArcs ℓ κ) (a : α) {t : ℕ} (ht : t ≤ 2 * ℓ a) (ht2 : t % 2 = 0) :
    pointColour κ (T.glued ℓ a t).1 = pointColour κ (T.glued ℓ a t).2 := by
  refine hκ _ ?_ ?_ ?_
  · unfold gluings
    simp only [List.mem_flatMap, List.mem_map, List.mem_range]
    exact ⟨a, hall a, t, by omega, rfl⟩
  · simp only [glued]
    exact T.sidePoint_isCoarse _ (by omega)
  · simp only [glued]
    split_ifs <;> exact T.sidePoint_isCoarse _ (by omega)

/-- The arcs of sign `-1` of a list, counted mod 2. -/
def negCount (l : List α) : ZMod 2 := (l.map fun a => if T.sign a = 1 then 0 else 1).sum

theorem negCount_cons (a : α) (l : List α) :
    T.negCount (a :: l) = (if T.sign a = 1 then 0 else 1) + T.negCount l := by
  simp [negCount]

/-- **Across an arc** from the station `k + ½` of `τ`, the walk enters the other
side at a station `k' + ½`, and the colour beside it changes by `1` exactly when
the arc has sign `-1`. -/
theorem colour_cross [Fintype α] {allArcs : List α} (hall : ∀ a, a ∈ allArcs)
    (hκ : T.IsColouring allArcs ℓ κ) {τ : Side Patch} {a : α} {k : ℕ}
    (h : arcAt (rat ℓ) (T.arcs τ) ((k : ℚ) + 1 / 2) = some a) :
    ∃ k' : ℕ, (T.crossAt (rat ℓ) τ a).apply ((k : ℚ) + 1 / 2) = (k' : ℚ) + 1 / 2 ∧
      pointColour κ (T.stationVertex ℓ (T.other a τ) k') =
        pointColour κ (T.stationVertex ℓ τ k) + (if T.sign a = 1 then 0 else 1) := by
  have hℓ : ∀ b, (0 : ℚ) ≤ rat ℓ b := fun b => by simp [rat]
  obtain ⟨ha, h₁, h₂⟩ := (arcAt_eq_some hℓ (T.arcs_nodup τ) _ a).mp h
  have hcross : (T.crossAt (rat ℓ) τ a).apply ((k : ℚ) + 1 / 2) =
      if T.sign a = 1 then (k : ℚ) + 1 / 2 + posNat a (T.arcs (T.other a τ)) ℓ - posNat a (T.arcs τ) ℓ
      else posNat a (T.arcs τ) ℓ + posNat a (T.arcs (T.other a τ)) ℓ + ℓ a - ((k : ℚ) + 1 / 2) := by
    unfold crossAt
    split_ifs <;> simp [Motion.apply, posAt_rat, rat] <;> ring
  rw [hcross]
  rw [posAt_rat] at h₁ h₂
  simp only [rat] at h₂
  generalize hpdef : posNat a (T.arcs τ) ℓ = p at h₁ h₂ ⊢
  generalize hp'def : posNat a (T.arcs (T.other a τ)) ℓ = p' at ⊢
  have hp : p ≤ k := by
    by_contra hc
    have : (k : ℚ) + 1 ≤ p := by exact_mod_cast (Nat.lt_of_not_le hc)
    linarith
  have hk : k < p + ℓ a := by
    by_contra hc
    have : (p : ℚ) + ℓ a ≤ k := by exact_mod_cast (Nat.le_of_not_lt hc)
    linarith
  rcases T.sign_isSign a with hs | hs
  · obtain ⟨k', hk'⟩ : ∃ k', k' + p = k + p' := ⟨k + p' - p, by omega⟩
    have hq : (k' : ℚ) + p = k + p' := by exact_mod_cast hk'
    refine ⟨k', by simp only [hs, ↓reduceIte]; linarith, ?_⟩
    simp only [hs, ↓reduceIte, add_zero]
    have hg := T.colour_glued hall hκ a (t := 2 * (k - p)) (by omega) (by omega)
    rcases (T.mem_arcs a τ).mp ha with h0 | h0
    · rw [h0, other_ends_zero] at hp'def ⊢
      rw [h0] at hpdef
      simp only [glued, hs, ↓reduceIte, hpdef, hp'def] at hg
      rw [stationVertex, stationVertex, show 2 * k' = 2 * p' + 2 * (k - p) by omega, ← hg,
        show 2 * p + 2 * (k - p) = 2 * k by omega]
    · rw [h0, other_ends_one] at hp'def ⊢
      rw [h0] at hpdef
      simp only [glued, hs, ↓reduceIte, hpdef, hp'def] at hg
      rw [stationVertex, stationVertex, show 2 * k' = 2 * p' + 2 * (k - p) by omega, hg,
        show 2 * p + 2 * (k - p) = 2 * k by omega]
  · obtain ⟨k', hk'⟩ : ∃ k', k' + k + 1 = p + p' + ℓ a := ⟨p + p' + ℓ a - k - 1, by omega⟩
    have hq : (k' : ℚ) + k + 1 = p + p' + ℓ a := by exact_mod_cast hk'
    refine ⟨k', by simp only [hs, reduceCtorEq, ↓reduceIte]; linarith, ?_⟩
    simp only [hs, reduceCtorEq, ↓reduceIte]
    suffices hsv : pointColour κ (T.stationVertex ℓ τ k) =
        pointColour κ (T.stationVertex ℓ (T.other a τ) (k' + 1)) by
      rw [hsv, colour_succ, add_assoc, show (1 : ZMod 2) + 1 = 0 by decide, add_zero]
    rcases (T.mem_arcs a τ).mp ha with h0 | h0
    · have hg := T.colour_glued hall hκ a (t := 2 * (k - p)) (by omega) (by omega)
      rw [h0, other_ends_zero] at hp'def ⊢
      rw [h0] at hpdef
      simp only [glued, hs, reduceCtorEq, ↓reduceIte, hpdef, hp'def] at hg
      rw [stationVertex, stationVertex, show 2 * (k' + 1) = 2 * p' + 2 * ℓ a - 2 * (k - p) by omega,
        ← hg, show 2 * p + 2 * (k - p) = 2 * k by omega]
    · have hg := T.colour_glued hall hκ a (t := 2 * (p + ℓ a - k)) (by omega) (by omega)
      rw [h0, other_ends_one] at hp'def ⊢
      rw [h0] at hpdef
      simp only [glued, hs, reduceCtorEq, ↓reduceIte, hpdef, hp'def] at hg
      rw [stationVertex, stationVertex, show 2 * (k' + 1) = 2 * p' + 2 * (p + ℓ a - k) by omega,
        hg, show 2 * p + 2 * ℓ a - 2 * (p + ℓ a - k) = 2 * k by omega]

/-! ### Along a path, and round a turn -/

/-- **Along a path** followed by the walk from the station `k + ½` of `σ`, the walk
reaches a station `k' + ½` of the end side, and the colour beside it is the colour
at the start plus the length of the path plus the number of arcs of sign `-1`. -/
theorem colour_along [Fintype α] {allArcs : List α} (hall : ∀ a, a ∈ allArcs)
    (hκ : T.IsColouring allArcs ℓ κ) :
    ∀ (l : List α) (σ : Side Patch) (k : ℕ), T.Follows (rat ℓ) σ l ((k : ℚ) + 1 / 2) →
      ∃ k' : ℕ, (Motion.composeAll (T.crossingsAt (rat ℓ) σ l)).apply ((k : ℚ) + 1 / 2) =
          (k' : ℚ) + 1 / 2 ∧
        pointColour κ (T.stationVertex ℓ (T.endSide σ l) k') =
          pointColour κ (T.stationVertex ℓ σ k) +
            evalForm (T.pathLengthForm σ l) (fun a => (ℓ a : ZMod 2)) + T.negCount l
  | [], σ, k, _ =>
    ⟨k, by simp [crossingsAt, Motion.apply, Motion.id], by simp [endSide, pathLengthForm, negCount]⟩
  | a :: l, σ, k, ⟨ha, hl⟩ => by
    obtain ⟨k₁, h₁, c₁⟩ := T.colour_cross hall hκ ha
    rw [h₁] at hl
    obtain ⟨k', h', c'⟩ := colour_along hall hκ l _ k₁ hl
    refine ⟨k', ?_, ?_⟩
    · simp only [crossingsAt, Motion.composeAll_cons, Motion.apply_comp, h₁, h']
    · rw [endSide, c', c₁, colour_opp, pathLengthForm, evalForm_add, negCount_cons]
      ring

theorem map_lin_crossings [Fintype α] (σ : Side Patch) (l : List α) :
    (T.crossings σ l).map Motion.lin = l.map T.sign := by
  induction l generalizing σ with
  | nil => rfl
  | cons a l ih => simp [crossings, crossForm_lin, ih]

theorem prod_sign (l : List α) :
    (l.map T.sign).prod = if T.negCount l = 0 then 1 else -1 := by
  induction l with
  | nil => simp [negCount]
  | cons a l ih =>
    rw [List.map_cons, List.prod_cons, ih, negCount_cons]
    generalize T.negCount l = n
    rcases T.sign_isSign a with h | h <;> rw [h] <;> fin_cases n <;> decide

/-- **On an oriented T-mesh a turn crosses an even number of arcs of sign `-1`**:
the product of its signs is the linear part of its composite, `1`. -/
theorem negCount_turn [Fintype α] (hT : T.Oriented) (γ : T.Turn) : T.negCount γ.path = 0 := by
  have h := γ.composite_lin hT
  rw [Turn.composite, Motion.composeAll_lin, List.prod_reverse, map_lin_crossings, prod_sign] at h
  by_contra hn
  simp only [hn, ↓reduceIte] at h
  exact absurd h (by decide)

/-- **Proposition "Parity of a turn on a sphere"** on the `TMesh`: on an oriented
T-mesh, if the constants `κ` colour the quad graph at the whole lengths `ℓ`, the
length and the shift of a turn whose walk from the station `k + ½` of its first
side crosses its arcs are congruent mod 2. -/
theorem parity_of_turn_of_colouring [Fintype α] (hT : T.Oriented) (allArcs : List α)
    (hall : ∀ a, a ∈ allArcs) (ℓ : α → ℕ) (κ : Patch → ZMod 2)
    (hκ : T.IsColouring allArcs ℓ κ) (γ : T.Turn) (k : ℕ)
    (hk : T.Follows (rat ℓ) γ.start γ.path ((k : ℚ) + 1 / 2)) :
    ((evalForm γ.lengthForm (fun a => (ℓ a : ℤ)) : ℤ) : ZMod 2) =
      ((evalForm γ.shiftForm (fun a => (ℓ a : ℤ)) : ℤ) : ZMod 2) := by
  obtain ⟨k', h', c'⟩ := T.colour_along hall hκ γ.path γ.start k hk
  rw [(γ.shift_linear hT (rat ℓ)).1, Motion.apply_translation, Turn.shift] at h'
  rw [γ.closes, T.colour_stationVertex _ k', T.colour_stationVertex _ k,
    T.negCount_turn hT γ] at c'
  have hs : (k' : ℤ) = k + evalForm γ.shiftForm (fun a => (ℓ a : ℤ)) := by
    have e : ((evalForm γ.shiftForm (fun a => (ℓ a : ℤ)) : ℤ) : ℚ) = evalForm γ.shiftForm (rat ℓ) :=
      evalForm_cast _ _
    have : (k' : ℚ) = k + ((evalForm γ.shiftForm (fun a => (ℓ a : ℤ)) : ℤ) : ℚ) := by
      rw [e]; linarith
    exact_mod_cast this
  have hs2 := congrArg (Int.cast : ℤ → ZMod 2) hs
  push_cast at hs2
  rw [evalForm_cast] at hs2
  rw [evalForm_cast, evalForm_cast, Turn.lengthForm]
  linear_combination hs2 - c'

/-! ### Lengths of any sign -/

open Matrix

/-- An integer linear form as an additive map of the integer lengths. -/
def formHom [Fintype α] (c : α → ℤ) : (α → ℤ) →+ ℤ where
  toFun := evalForm c
  map_zero' := by simp [evalForm]
  map_add' x y := by simp [evalForm, mul_add, Finset.sum_add_distrib]

/-- **Proposition "Parity of a turn on a sphere" on the `TMesh`, whole balanced
lengths of any sign.** Some real lengths positive on every arc balance every
patch, and at every whole balanced `ℓ` positive on every arc the quad graph is
bipartite and the walk from some station `k + ½` of the first side follows the
turn. Then the length and the shift of the turn are congruent mod 2 at every
whole balanced `ℓ`. -/
theorem parity_of_turn_tmesh_any_sign [Fintype α] [Fintype Patch] (hT : T.Oriented)
    (allArcs : List α) (hall : ∀ a, a ∈ allArcs) (γ : T.Turn)
    (hreal : ∃ ℓ : α → ℝ, T.balance.map (Int.cast : ℤ → ℝ) *ᵥ ℓ = 0 ∧ ∀ a, 0 < ℓ a)
    (hbip : ∀ ℓ : α → ℕ, T.balance *ᵥ (fun a => (ℓ a : ℤ)) = 0 → (∀ a, 0 < ℓ a) →
      T.Bipartite allArcs ℓ ∧ ∃ k : ℕ, T.Follows (rat ℓ) γ.start γ.path ((k : ℚ) + 1 / 2)) :
    ∀ ℓ : α → ℤ, T.balance *ᵥ ℓ = 0 →
      ((evalForm γ.lengthForm ℓ : ℤ) : ZMod 2) = ((evalForm γ.shiftForm ℓ : ℤ) : ZMod 2) := by
  let S : AddSubgroup (α → ℤ) :=
    { carrier := {ℓ | T.balance *ᵥ ℓ = 0}
      add_mem' := fun {x y} (hx : T.balance *ᵥ x = 0) (hy : T.balance *ᵥ y = 0) => by
        simp [Matrix.mulVec_add, hx, hy]
      zero_mem' := Matrix.mulVec_zero T.balance
      neg_mem' := fun {x} (hx : T.balance *ᵥ x = 0) => by simp [Matrix.mulVec_neg, hx] }
  obtain ⟨p, hpB, hp⟩ := exists_positive_whole T.balance hreal
  intro ℓ hℓ
  rw [← even_sub_iff_zmod_two]
  refine even_of_even_on_positive S (formHom γ.lengthForm - formHom γ.shiftForm) p hpB hp
    (fun m (hm : T.balance *ᵥ m = 0) hmpos => ?_) ℓ hℓ
  have hcast : (fun a => ((m a).toNat : ℤ)) = m := funext fun a => Int.toNat_of_nonneg (hmpos a).le
  obtain ⟨⟨κ, hκ⟩, k, hk⟩ := hbip (fun a => (m a).toNat) (by rw [hcast]; exact hm)
    (fun a => by have := hmpos a; omega)
  have h := T.parity_of_turn_of_colouring hT allArcs hall _ κ hκ γ k hk
  rw [hcast] at h
  rw [AddMonoidHom.sub_apply, even_sub_iff_zmod_two]
  exact h

end TMesh

end TurnsOfATMesh
