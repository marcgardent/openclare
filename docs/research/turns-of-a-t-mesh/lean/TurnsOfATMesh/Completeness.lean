/-
Proposition "Completeness at given lengths", in a discrete model of the T-mesh.

At whole lengths `ℓ` balancing every patch, each patch is an integer rectangle and
the quad mesh is its unit grid. The model has two layers.

* **Along a side.** An arc is glued to the arc across it by a quarter-turn and a
  whole translation; restricted to the line of the side, that is `t ↦ ε t + c`
  with `ε = ±1` and `c` whole (`glueLine`), the same map on every unit edge
  `[k, k + 1]` of the arc since nodes sit at whole positions. Such a map sends
  the midpoint `k + ½` of a unit edge to the midpoint of a unit edge
  (`glueLine_half`) and the open unit edge onto an open unit edge
  (`glueLine_open`). So the walk of Definition "Turn" from a half-integer
  `k + ½`, taken with the lengths `ℓ`, crosses the sides exactly at the
  midpoints the discrete walk on unit edges gives (`realWalk_iterate_half`), and
  so does the walk from any point of the open unit edge
  (`realWalk_iterate_open`): the walk is the strip's.

* **Strips.** A station is a unit edge of a side, entered from that side; the walk
  `walk` crosses the patch to the opposite side at the same offset and passes the
  arc to the next station. A quad crossed by a strip is a dart: a station and the
  row `m < depth` of the quad in the column above it. The strip moves to the next
  quad of the column, or at the last row through the opposite side to the next
  station (`next`). The strips of the quad mesh are the cycles of `next`. Every
  strip meets a station (`exists_station`), two stations lie on the same strip
  exactly when the walk takes one to the other (`sameCycle_station_iff`), so the
  strips are the cycles of the walk (`stripsEquiv`): walking from every
  half-integer of every side finds every strip, each as one cycle of the walk.

* **Together.** On a closed surface (`ClosedGluing`: opposite sides of a patch
  have the same length, and no two unit edges are glued onto the same one) the
  walk on unit edges is a permutation of the stations (`stationPerm`), and
  `completeness_at_given_lengths` takes it as the `walk` of the strips: the
  strips are in bijection with the cycles of the real walk from the
  half-integers. The model still assumes that crossing a patch keeps the offset,
  and that the darts and `next` built on it are the quads and strips of the mesh.
-/
module

public import Mathlib.GroupTheory.Perm.Cycle.Basic
public import Mathlib.Algebra.Order.Floor.Ring
public import Mathlib.Basic.Real.Basic
public import Mathlib.Algebra.Order.Archimedean.Real.Basic
public import Mathlib.Data.Fintype.Sigma
public import Mathlib.Tactic.Linarith
public import Mathlib.Tactic.Ring
public import Mathlib.Tactic.Positivity

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace Completeness

/-! ### Along a side: half-integers and unit edges -/

/-- The gluing of an arc on the line of a side: `t ↦ ε t + c`. -/
def glueLine (ε c : ℤ) (t : ℝ) : ℝ := ε * t + c

/-- The unit edge `[k, k + 1]` is glued onto the unit edge `[k', k' + 1]`. -/
def glueEdge (ε c k : ℤ) : ℤ := if ε = 1 then k + c else c - 1 - k

/-- The midpoint of a unit edge goes to the midpoint of a unit edge. -/
theorem glueLine_half {ε : ℤ} (hε : ε = 1 ∨ ε = -1) (c k : ℤ) :
    glueLine ε c (k + 1 / 2) = glueEdge ε c k + 1 / 2 := by
  rcases hε with rfl | rfl
  · simp [glueLine, glueEdge]; ring
  · simp [glueLine, glueEdge]; ring

/-- The open unit edge goes onto an open unit edge: a point at a non-whole offset
never meets a node, and the unit edge it reaches does not depend on the point. -/
theorem glueLine_open {ε : ℤ} (hε : ε = 1 ∨ ε = -1) (c k : ℤ) {t : ℝ}
    (h₁ : (k : ℝ) < t) (h₂ : t < k + 1) :
    (glueEdge ε c k : ℝ) < glueLine ε c t ∧ glueLine ε c t < glueEdge ε c k + 1 := by
  rcases hε with rfl | rfl
  · simp only [glueLine, glueEdge, ↓reduceIte, Int.cast_add, Int.cast_one, one_mul]
    constructor <;> linarith
  · rw [glueLine, glueEdge, ite_eq_right (by decide : ¬ ((-1 : ℤ) = 1))]
    push_cast
    constructor <;> linarith

variable {Side : Type*} (opp : Side → Side) (target : Side → ℤ → Side) (sign shift : Side → ℤ → ℤ)

/-- One move of the walk of Definition "Turn" at whole lengths: from the side `σ` at
offset `t`, across the patch to the opposite side at the same offset, then across
the arc holding the unit edge `⌊t⌋` of that side. -/
noncomputable def realWalk (p : Side × ℝ) : Side × ℝ :=
  (target (opp p.1) ⌊p.2⌋, glueLine (sign (opp p.1) ⌊p.2⌋) (shift (opp p.1) ⌊p.2⌋) p.2)

/-- The same move on unit edges. -/
def edgeWalk (p : Side × ℤ) : Side × ℤ :=
  (target (opp p.1) p.2, glueEdge (sign (opp p.1) p.2) (shift (opp p.1) p.2) p.2)

/-- The second half of the move: the unit edge `k` of the side `τ` goes across the
arc holding it to a unit edge of the side `target τ k`. -/
def crossArc (p : Side × ℤ) : Side × ℤ :=
  (target p.1 p.2, glueEdge (sign p.1 p.2) (shift p.1 p.2) p.2)

theorem edgeWalk_eq (p : Side × ℤ) :
    edgeWalk opp target sign shift p = crossArc target sign shift (opp p.1, p.2) := rfl

variable {opp target sign shift}

/-- **The walk from a half-integer is the walk on unit edges**, at every step. -/
theorem realWalk_iterate_half (hs : ∀ σ k, sign σ k = 1 ∨ sign σ k = -1) (n : ℕ) (σ : Side)
    (k : ℤ) : (realWalk opp target sign shift)^[n] (σ, (k : ℝ) + 1 / 2) =
      (((edgeWalk opp target sign shift)^[n] (σ, k)).1,
        (((edgeWalk opp target sign shift)^[n] (σ, k)).2 : ℝ) + 1 / 2) := by
  induction n generalizing σ k with
  | zero => rfl
  | succ n ih =>
    rw [Function.iterate_succ_apply, Function.iterate_succ_apply]
    have hfl : ⌊(k : ℝ) + 1 / 2⌋ = k := by
      rw [Int.floor_eq_iff]; constructor <;> linarith
    have : realWalk opp target sign shift (σ, (k : ℝ) + 1 / 2) =
        ((edgeWalk opp target sign shift (σ, k)).1,
          ((edgeWalk opp target sign shift (σ, k)).2 : ℝ) + 1 / 2) := by
      simp only [realWalk, edgeWalk, hfl, glueLine_half (hs _ _)]
    rw [this, ih]

/-- **Any point of the open unit edge follows the same strip**: the walk from
`t ∈ (k, k + 1)` stays inside the open unit edges the walk on unit edges gives. -/
theorem realWalk_iterate_open (hs : ∀ σ k, sign σ k = 1 ∨ sign σ k = -1) (n : ℕ) (σ : Side)
    (k : ℤ) (t : ℝ) (h₁ : (k : ℝ) < t) (h₂ : t < k + 1) :
    ((realWalk opp target sign shift)^[n] (σ, t)).1 =
        ((edgeWalk opp target sign shift)^[n] (σ, k)).1 ∧
      (((edgeWalk opp target sign shift)^[n] (σ, k)).2 : ℝ) <
        ((realWalk opp target sign shift)^[n] (σ, t)).2 ∧
      ((realWalk opp target sign shift)^[n] (σ, t)).2 <
        ((edgeWalk opp target sign shift)^[n] (σ, k)).2 + 1 := by
  induction n generalizing σ k t with
  | zero => exact ⟨rfl, h₁, h₂⟩
  | succ n ih =>
    rw [Function.iterate_succ_apply, Function.iterate_succ_apply]
    have hfl : ⌊t⌋ = k := by rw [Int.floor_eq_iff]; exact ⟨h₁.le, h₂⟩
    have h := glueLine_open (hs (opp σ) k) (shift (opp σ) k) k h₁ h₂
    simp only [realWalk, edgeWalk, hfl]
    exact ih _ _ _ h.1 h.2

/-! ### Strips -/

section Strips

variable {St : Type*} [Fintype St] [DecidableEq St] (walk : Equiv.Perm St)
  (depth : St → ℕ) (hdepth : ∀ s, 0 < depth s)

/-- A quad crossed by a strip: a station and the row of the quad above it. -/
abbrev Dart := Σ s : St, Fin (depth s)

/-- The quad of row `0` above a station. -/
def station (s : St) : Dart depth := ⟨s, ⟨0, hdepth s⟩⟩

/-- The next quad of the strip: the next row of the column, or after the last row,
through the opposite side, the first row above the next station. -/
def next (d : Dart depth) : Dart depth :=
  if h : d.2.val + 1 < depth d.1 then ⟨d.1, ⟨d.2.val + 1, h⟩⟩ else station depth hdepth (walk d.1)

variable {walk depth hdepth}

theorem next_iterate_lt (s : St) (i : ℕ) (hi : i < depth s) :
    (next walk depth hdepth)^[i] (station depth hdepth s) = ⟨s, ⟨i, hi⟩⟩ := by
  induction i with
  | zero => rfl
  | succ i ih =>
    rw [Function.iterate_succ_apply', ih (by omega)]
    simp [next, hi]

theorem next_iterate_depth (s : St) :
    (next walk depth hdepth)^[depth s] (station depth hdepth s) =
      station depth hdepth (walk s) := by
  obtain ⟨i, hi⟩ : ∃ i, depth s = i + 1 := ⟨depth s - 1, by have := hdepth s; omega⟩
  rw [hi, Function.iterate_succ_apply', next_iterate_lt s i (by omega)]
  simp [next, hi]

theorem next_injective : Function.Injective (next walk depth hdepth) := by
  rintro ⟨s, m⟩ ⟨s', m'⟩ h
  simp only [next] at h
  split_ifs at h with h1 h2 h2
  · simp only [Sigma.mk.injEq] at h
    obtain ⟨rfl, h⟩ := h
    simp only [heq_eq_eq, Fin.mk.injEq] at h
    congr; exact Fin.ext (by omega)
  · simp only [station, Sigma.mk.injEq] at h
    obtain ⟨rfl, h⟩ := h
    simp at h
  · simp only [station, Sigma.mk.injEq] at h
    obtain ⟨rfl, h⟩ := h
    simp at h
  · simp only [station, Sigma.mk.injEq, EmbeddingLike.apply_eq_iff_eq] at h
    obtain ⟨rfl, -⟩ := h
    congr; exact Fin.ext (by omega)

variable (walk depth hdepth)

/-- The strips: `next` is a permutation of the darts. -/
noncomputable def nextPerm : Equiv.Perm (Dart depth) :=
  Equiv.ofBijective _ (Finite.injective_iff_bijective.mp (next_injective (walk := walk)
    (hdepth := hdepth)))

variable {walk depth hdepth}

theorem nextPerm_pow (n : ℕ) (d : Dart depth) :
    (nextPerm walk depth hdepth ^ n) d = (next walk depth hdepth)^[n] d := by
  rw [Equiv.Perm.coe_pow]; rfl

/-- Every quad of a strip lies above a station: every strip meets a side. -/
theorem exists_station (d : Dart depth) :
    ∃ s, (nextPerm walk depth hdepth).SameCycle (station depth hdepth s) d :=
  ⟨d.1, d.2.val, by
    rw [zpow_natCast, nextPerm_pow, next_iterate_lt d.1 d.2.val d.2.isLt]⟩

theorem walk_iterate_to_next (n : ℕ) (s : St) :
    ∃ N, (next walk depth hdepth)^[N] (station depth hdepth s) =
      station depth hdepth ((walk : St → St)^[n] s) := by
  induction n generalizing s with
  | zero => exact ⟨0, rfl⟩
  | succ n ih =>
    obtain ⟨N, hN⟩ := ih (walk s)
    refine ⟨N + depth s, ?_⟩
    rw [Function.iterate_add_apply, next_iterate_depth, hN, Function.iterate_succ_apply]

theorem next_iterate_to_walk (N : ℕ) : ∀ s s' : St,
    (next walk depth hdepth)^[N] (station depth hdepth s) = station depth hdepth s' →
      ∃ n, (walk : St → St)^[n] s = s' := by
  induction N using Nat.strong_induction_on with
  | _ N ih =>
    intro s s' h
    by_cases hN : N < depth s
    · rw [next_iterate_lt s N hN] at h
      simp only [station, Sigma.mk.injEq] at h
      obtain ⟨rfl, h⟩ := h
      simp only [heq_eq_eq, Fin.mk.injEq] at h
      exact ⟨0, rfl⟩
    · obtain ⟨M, rfl⟩ : ∃ M, N = M + depth s := ⟨N - depth s, by omega⟩
      rw [Function.iterate_add_apply, next_iterate_depth] at h
      obtain ⟨n, hn⟩ := ih M (by have := hdepth s; omega) (walk s) s' h
      exact ⟨n + 1, by rw [Function.iterate_succ_apply, hn]⟩

/-- **Two stations lie on the same strip exactly when the walk takes one to the
other.** -/
theorem sameCycle_station_iff (s s' : St) :
    (nextPerm walk depth hdepth).SameCycle (station depth hdepth s) (station depth hdepth s') ↔
      walk.SameCycle s s' := by
  constructor
  · intro h
    obtain ⟨N, hN⟩ := h.exists_nat_pow_eq
    rw [nextPerm_pow] at hN
    obtain ⟨n, hn⟩ := next_iterate_to_walk N s s' hN
    exact ⟨n, by rw [zpow_natCast, Equiv.Perm.coe_pow, hn]⟩
  · intro h
    obtain ⟨n, hn⟩ := h.exists_nat_pow_eq
    rw [Equiv.Perm.coe_pow] at hn
    obtain ⟨N, hN⟩ := walk_iterate_to_next (walk := walk) (hdepth := hdepth) n s
    exact ⟨N, by rw [zpow_natCast, nextPerm_pow, hN, hn]⟩

/-- **Proposition "Completeness at given lengths".** The strips of the quad mesh (the
cycles of `next`) are in bijection with the cycles of the walk on the stations,
the half-integers of the sides. -/
noncomputable def stripsEquiv :
    Quotient (Equiv.Perm.SameCycle.setoid walk) ≃
      Quotient (Equiv.Perm.SameCycle.setoid (nextPerm walk depth hdepth)) :=
  Equiv.ofBijective
    (Quotient.map (station depth hdepth) fun s s' h => (sameCycle_station_iff s s').mpr h)
    ⟨by
      rintro ⟨s⟩ ⟨s'⟩ h
      exact Quotient.sound ((sameCycle_station_iff s s').mp (Quotient.exact h)),
    by
      rintro ⟨d⟩
      obtain ⟨s, hs⟩ := exists_station (walk := walk) (hdepth := hdepth) d
      exact ⟨⟦s⟧, Quotient.sound hs⟩⟩

end Strips

/-! ### The two layers together -/

section TMesh

variable {Side : Type*} [Fintype Side] [DecidableEq Side] {len : Side → ℕ}
  {opp : Side → Side} {target : Side → ℤ → Side} {sign shift : Side → ℤ → ℤ}

/-- The walk on unit edges maps a station (a unit edge `k < len σ` of a side) to a
station: the gluing maps the unit edges of the sides onto one another. -/
def MapsStations (len : Side → ℕ) (opp : Side → Side) (target : Side → ℤ → Side)
    (sign shift : Side → ℤ → ℤ) : Prop :=
  ∀ σ (k : ℤ), 0 ≤ k → k < len σ →
    0 ≤ (edgeWalk opp target sign shift (σ, k)).2 ∧
      (edgeWalk opp target sign shift (σ, k)).2 < len (edgeWalk opp target sign shift (σ, k)).1

/-- The walk on the stations `Σ σ, Fin (len σ)`. -/
def stationWalk (h : MapsStations len opp target sign shift) :
    (Σ σ : Side, Fin (len σ)) → Σ σ : Side, Fin (len σ) := fun s =>
  ⟨(edgeWalk opp target sign shift (s.1, s.2.val)).1,
    ⟨(edgeWalk opp target sign shift (s.1, s.2.val)).2.toNat, by
      have := h s.1 s.2.val (by positivity) (by exact_mod_cast s.2.isLt)
      omega⟩⟩

theorem stationWalk_iterate (h : MapsStations len opp target sign shift) (n : ℕ)
    (s : Σ σ : Side, Fin (len σ)) :
    ((((stationWalk h)^[n] s).1, (((stationWalk h)^[n] s).2.val : ℤ)) : Side × ℤ) =
      (edgeWalk opp target sign shift)^[n] (s.1, s.2.val) := by
  induction n generalizing s with
  | zero => rfl
  | succ n ih =>
    rw [Function.iterate_succ_apply, Function.iterate_succ_apply, ih]
    congr 1
    have := h s.1 s.2.val (by positivity) (by exact_mod_cast s.2.isLt)
    simp only [stationWalk]
    ext
    · rfl
    · simp only; omega

/-- **The real walk from the half-integer `k + ½` of a station visits the sides at
the midpoints of the stations of the walk**, step after step. On a closed surface
the station walk is a permutation (`stationPerm`), and
`completeness_at_given_lengths` composes this with `stripsEquiv`. -/
theorem realWalk_stations (hs : ∀ σ k, sign σ k = 1 ∨ sign σ k = -1)
    (h : MapsStations len opp target sign shift) (n : ℕ) (s : Σ σ : Side, Fin (len σ)) :
    (realWalk opp target sign shift)^[n] (s.1, (s.2.val : ℝ) + 1 / 2) =
      (((stationWalk h)^[n] s).1, (((stationWalk h)^[n] s).2.val : ℝ) + 1 / 2) := by
  have h1 := realWalk_iterate_half (opp := opp) (target := target) (shift := shift) hs n s.1
    s.2.val
  have h2 := stationWalk_iterate h n s
  push_cast at h1
  rw [h1, ← h2]
  simp

/-- **The gluing of a closed surface at whole lengths.** Crossing a patch takes the
unit edge `k` of a side to the unit edge `k` of the opposite side; it maps the
stations one to one when opposite sides have the same length, which is what
lengths balancing every patch say (`len_opp`), and two sides never have the same
opposite side (`opp_injective`). Crossing an arc takes a station to a station:
every unit edge of a side lies in one arc, glued whole onto the arc across it, on
a side of the neighbouring patch (`crossArc_station`); and two unit edges are
never glued onto the same unit edge (`crossArc_injective`). On a T-mesh with
whole lengths on a closed surface both maps are their own inverses: the opposite
of the opposite side is the side, and crossing an arc back undoes `t ↦ ε t + c`
(`ClosedGluing.of_involutive`). -/
structure ClosedGluing (len : Side → ℕ) (opp : Side → Side) (target : Side → ℤ → Side)
    (sign shift : Side → ℤ → ℤ) : Prop where
  opp_injective : Function.Injective opp
  len_opp : ∀ σ, len (opp σ) = len σ
  crossArc_station : ∀ τ (k : ℤ), 0 ≤ k → k < len τ →
    0 ≤ (crossArc target sign shift (τ, k)).2 ∧
      (crossArc target sign shift (τ, k)).2 < len (crossArc target sign shift (τ, k)).1
  crossArc_injective : ∀ τ τ' (k k' : ℤ), 0 ≤ k → k < len τ → 0 ≤ k' → k' < len τ' →
    crossArc target sign shift (τ, k) = crossArc target sign shift (τ', k') → (τ, k) = (τ', k')

/-- The gluing of a closed surface, from the two involutions of a T-mesh: the
opposite of the opposite side is the side, and crossing an arc then crossing back
returns to the unit edge one started from. -/
theorem ClosedGluing.of_involutive (hopp : Function.Involutive opp)
    (hlen : ∀ σ, len (opp σ) = len σ)
    (hstation : ∀ τ (k : ℤ), 0 ≤ k → k < len τ →
      0 ≤ (crossArc target sign shift (τ, k)).2 ∧
        (crossArc target sign shift (τ, k)).2 < len (crossArc target sign shift (τ, k)).1)
    (hback : ∀ τ (k : ℤ), 0 ≤ k → k < len τ →
      crossArc target sign shift (crossArc target sign shift (τ, k)) = (τ, k)) :
    ClosedGluing len opp target sign shift where
  opp_injective := hopp.injective
  len_opp := hlen
  crossArc_station := hstation
  crossArc_injective τ τ' k k' h₀ h₁ h₀' h₁' h := by
    rw [← hback τ k h₀ h₁, ← hback τ' k' h₀' h₁', h]

theorem ClosedGluing.mapsStations (hc : ClosedGluing len opp target sign shift) :
    MapsStations len opp target sign shift := fun σ k h₀ h₁ =>
  hc.crossArc_station (opp σ) k h₀ (by rw [hc.len_opp]; exact h₁)

/-- **On a closed surface the walk maps no two stations to the same station.** -/
theorem stationWalk_injective (hc : ClosedGluing len opp target sign shift) :
    Function.Injective (stationWalk hc.mapsStations) := by
  rintro ⟨σ, k⟩ ⟨σ', k'⟩ h
  have e : edgeWalk opp target sign shift (σ', k'.val) =
      edgeWalk opp target sign shift (σ, k.val) :=
    (stationWalk_iterate hc.mapsStations 1 ⟨σ', k'⟩).symm.trans
      ((congrArg (fun t => (t.1, (t.2.val : ℤ))) h).symm.trans
        (stationWalk_iterate hc.mapsStations 1 ⟨σ, k⟩))
  rw [edgeWalk_eq, edgeWalk_eq] at e
  have := hc.crossArc_injective (opp σ') (opp σ) k'.val k.val (by positivity)
    (by rw [hc.len_opp]; exact_mod_cast k'.isLt) (by positivity)
    (by rw [hc.len_opp]; exact_mod_cast k.isLt) e
  simp only [Prod.mk.injEq, Nat.cast_inj] at this
  obtain ⟨h1, h2⟩ := this
  obtain rfl := hc.opp_injective h1
  rw [Fin.val_inj] at h2
  rw [h2]

/-- The walk on the stations of a closed surface, as a permutation. -/
noncomputable def stationPerm (hc : ClosedGluing len opp target sign shift) :
    Equiv.Perm (Σ σ : Side, Fin (len σ)) :=
  Equiv.ofBijective _ (Finite.injective_iff_bijective.mp (stationWalk_injective hc))

theorem stationPerm_pow (hc : ClosedGluing len opp target sign shift) (n : ℕ)
    (s : Σ σ : Side, Fin (len σ)) :
    (stationPerm hc ^ n) s = (stationWalk hc.mapsStations)^[n] s := by
  rw [Equiv.Perm.coe_pow]; rfl

/-- **Two stations lie on one cycle of the station walk exactly when the real walk
from the half-integer of the one reaches the half-integer of the other.** -/
theorem sameCycle_stationPerm_iff (hs : ∀ σ k, sign σ k = 1 ∨ sign σ k = -1)
    (hc : ClosedGluing len opp target sign shift) (s s' : Σ σ : Side, Fin (len σ)) :
    (stationPerm hc).SameCycle s s' ↔
      ∃ n : ℕ, (realWalk opp target sign shift)^[n] (s.1, (s.2.val : ℝ) + 1 / 2) =
        (s'.1, (s'.2.val : ℝ) + 1 / 2) := by
  constructor
  · intro h
    obtain ⟨n, hn⟩ := h.exists_nat_pow_eq
    exact ⟨n, by rw [realWalk_stations hs hc.mapsStations, ← stationPerm_pow hc, hn]⟩
  · rintro ⟨n, hn⟩
    rw [realWalk_stations hs hc.mapsStations, ← stationPerm_pow hc] at hn
    simp only [Prod.mk.injEq, add_left_inj, Nat.cast_inj] at hn
    obtain ⟨h1, h2⟩ := hn
    exact ⟨n, by rw [zpow_natCast]; exact Sigma.ext h1 ((Fin.heq_ext_iff (by rw [h1])).mpr h2)⟩

/-- **Proposition "Completeness at given lengths"**, on a closed surface at whole
lengths. A quad above a station lies in the column of `depth s > 0` quads between
its side and the opposite side; the strips are the cycles of the successor `next`
built from the station walk `stationPerm`. They are in bijection with the cycles
of the station walk, the strip of the cycle of `s` being the strip through `s`
(`stripsEquiv`), and these cycles are those of the real walk with the lengths `ℓ`
from the half-integers `k + ½` of all sides: walking from every half-integer of
every side finds every strip, each as one cycle of the walk. -/
theorem completeness_at_given_lengths (hs : ∀ σ k, sign σ k = 1 ∨ sign σ k = -1)
    (hc : ClosedGluing len opp target sign shift) (depth : (Σ σ : Side, Fin (len σ)) → ℕ)
    (hdepth : ∀ s, 0 < depth s) :
    ∃ e : Quotient (Equiv.Perm.SameCycle.setoid (stationPerm hc)) ≃
        Quotient (Equiv.Perm.SameCycle.setoid (nextPerm (stationPerm hc) depth hdepth)),
      (∀ s, e ⟦s⟧ = ⟦station depth hdepth s⟧) ∧
      ∀ s s', (stationPerm hc).SameCycle s s' ↔
        ∃ n : ℕ, (realWalk opp target sign shift)^[n] (s.1, (s.2.val : ℝ) + 1 / 2) =
          (s'.1, (s'.2.val : ℝ) + 1 / 2) :=
  ⟨stripsEquiv (walk := stationPerm hc) (hdepth := hdepth), fun _ => rfl,
    sameCycle_stationPerm_iff hs hc⟩

end TMesh

end Completeness

end TurnsOfATMesh
