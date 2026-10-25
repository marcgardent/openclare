/-
Lemma "Gauss–Bonnet for turns", in a discrete form.

A turn at whole lengths follows its strip at half-integer offsets. On the grid
refined by two (every quad cut into four), the curve `γ̄` of the turn, the walk
and its closing segment at depth `½` inside the first patch, runs along edges of
the refined mesh, through none of the old vertices: every vertex on it, and
every new vertex, has valence four. When `γ̄` is simple and bounds a disc `D`,
`D` is a union of quads of the refined mesh: a 2-complex whose interior
vertices have as many edges as quad corners (their link is a cycle) and whose
border vertices have one edge more than corners (their link is a path), with
Euler characteristic `χ = 1` (`0` for an annulus). These are the topological
inputs, taken as hypotheses (`IsQuadRegion`); what is proved is the counting
(`gauss_bonnet`):

  `∑_{v inside} (4 - deg v) + ∑_{v on the border} (2 - corners v) = 4 χ`.

Along `γ̄` a vertex has two quad corners inside `D` where the curve goes straight,
one at a convex corner (`+π/2`) and three at a concave one (`-π/2`). The closing
segment adds one of each, or none when the shift is zero, so the border terms
cancel and the indices `4 - k` of the singular vertices inside add up to `4χ`:
`4` on a disc, `0` on an annulus (`gauss_bonnet_turn`). The new vertices of the
refined mesh are regular, so the sum is over the singular vertices of the T-mesh.

The counting holds for any faces with `p` corners and regular valence `q` with
`p q = 2 p + 2 q` (`gauss_bonnet_general`): quads (`p = q = 4`), triangles
(`p = 3`, `q = 6`) and hexagons (`p = 6`, `q = 3`), the three regular tilings of
the plane, so for the fields of order 4, 6 and 3.

The count with convex and concave corners (`gauss_bonnet_corners`) gives the
totals of Lemma "One self-crossing" from the corners of its three regions
(`one_crossing_on_walk`, `one_crossing_on_segment`).

Not formalized: that a simple closed curve on a sphere bounds a disc on each side
(Jordan–Schoenflies), that the refined quads inside form a complex with the local
structure above, and, for Lemma "One self-crossing", which corners of its regions
are convex (the note's "direct check of the four directions").
-/
module

public import Mathlib.Algebra.BigOperators.Group.Finset.Basic
public import Mathlib.Algebra.BigOperators.Ring.Finset
public import Mathlib.Data.Fintype.BigOperators
public import Mathlib.Data.Fintype.Prod
public import Mathlib.Tactic.LinearCombination
public import Mathlib.Tactic.Linarith
public import Mathlib.Tactic.Ring

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace GaussBonnet

open Finset

variable {Vx Ed Fc : Type*} [Fintype Vx] [DecidableEq Vx] [Fintype Ed] [Fintype Fc] {p : ℕ}

/-- The valence of `v`: the ends of edges at `v` (an edge with both ends at `v`
counts twice). -/
def deg (ends : Ed → Fin 2 → Vx) (v : Vx) : ℕ := #{x : Ed × Fin 2 | ends x.1 x.2 = v}

/-- The number of face corners at `v`. -/
def corners (corner : Fc → Fin p → Vx) (v : Vx) : ℕ := #{x : Fc × Fin p | corner x.1 x.2 = v}

/-- Handshake: the valences add up to twice the number of edges. -/
theorem sum_deg (ends : Ed → Fin 2 → Vx) : ∑ v, deg ends v = 2 * Fintype.card Ed := by
  have := card_eq_sum_card_fiberwise (s := (univ : Finset (Ed × Fin 2)))
    (t := (univ : Finset Vx)) (f := fun x => ends x.1 x.2) (fun _ _ => mem_univ _)
  rw [card_univ, Fintype.card_prod, Fintype.card_fin, mul_comm] at this
  exact this.symm

/-- The corners add up to `p` times the number of faces. -/
theorem sum_corners (corner : Fc → Fin p → Vx) :
    ∑ v, corners corner v = p * Fintype.card Fc := by
  have := card_eq_sum_card_fiberwise (s := (univ : Finset (Fc × Fin p)))
    (t := (univ : Finset Vx)) (f := fun x => corner x.1 x.2) (fun _ _ => mem_univ _)
  rw [card_univ, Fintype.card_prod, Fintype.card_fin, mul_comm] at this
  exact this.symm

/-- A region made of faces with `p` corners, with border vertices `border`: an
interior vertex has as many edges as corners (its link is a cycle), a border
vertex one edge more (its link is a path), and the Euler characteristic is `χ`. -/
structure IsRegion (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin p → Vx) (border : Finset Vx)
    (χ : ℤ) : Prop where
  interior : ∀ v ∉ border, deg ends v = corners corner v
  boundary : ∀ v ∈ border, deg ends v = corners corner v + 1
  euler : (Fintype.card Vx : ℤ) - Fintype.card Ed + Fintype.card Fc = χ

/-- **Discrete Gauss–Bonnet**, for faces with `p` corners and regular valence `q`,
`p q = 2 p + 2 q`:
`2 ∑_{inside} (q - deg v) + ∑_{border} (q - 2 corners v) = 2 q χ`. -/
theorem gauss_bonnet_general (q : ℕ) (hpq : p * q = 2 * p + 2 * q)
    (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin p → Vx) (border : Finset Vx) (χ : ℤ)
    (h : IsRegion ends corner border χ) :
    2 * ∑ v ∈ borderᶜ, ((q : ℤ) - deg ends v) + ∑ v ∈ border, ((q : ℤ) - 2 * corners corner v) =
      2 * q * χ := by
  have hD : ∑ v, (deg ends v : ℤ) = 2 * Fintype.card Ed := by exact_mod_cast sum_deg ends
  have hC : ∑ v, (corners corner v : ℤ) = p * Fintype.card Fc := by
    exact_mod_cast sum_corners corner
  rw [← sum_add_sum_compl border] at hD hC
  have h1 : ∑ v ∈ borderᶜ, (deg ends v : ℤ) = ∑ v ∈ borderᶜ, (corners corner v : ℤ) :=
    sum_congr rfl fun v hv => by rw [h.interior v (mem_compl.mp hv)]
  have h2 : ∑ v ∈ border, (deg ends v : ℤ) = ∑ v ∈ border, (corners corner v : ℤ) + #border := by
    rw [card_eq_sum_ones, Nat.cast_sum, ← sum_add_distrib]
    exact sum_congr rfl fun v hv => by rw [h.boundary v hv]; push_cast; ring
  have h5 : (Fintype.card Vx : ℤ) = #borderᶜ + #border := by
    rw [card_compl]; push_cast [card_le_univ border]; ring
  have h6 : (p : ℤ) * q = 2 * p + 2 * q := by exact_mod_cast hpq
  have hχ := h.euler
  simp only [sum_sub_distrib, sum_const, nsmul_eq_mul, ← mul_sum]
  linear_combination (2 * (q : ℤ)) * hχ + (-2 * (q : ℤ)) * h5 + ((q : ℤ) - 2) * h1 +
    (q : ℤ) * h2 + (-(q : ℤ)) * hD + ((q : ℤ) - 2) * hC + (Fintype.card Fc : ℤ) * h6

/-- **Discrete Gauss–Bonnet for quads**:
`∑_{inside} (4 - deg v) + ∑_{border} (2 - corners v) = 4 χ`. -/
theorem gauss_bonnet (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin 4 → Vx) (border : Finset Vx)
    (χ : ℤ) (h : IsRegion ends corner border χ) :
    ∑ v ∈ borderᶜ, ((4 : ℤ) - deg ends v) + ∑ v ∈ border, ((2 : ℤ) - corners corner v) =
      4 * χ := by
  have := gauss_bonnet_general 4 (by norm_num) ends corner border χ h
  have e : ∑ v ∈ border, ((4 : ℤ) - 2 * corners corner v) =
      2 * ∑ v ∈ border, ((2 : ℤ) - corners corner v) := by
    rw [mul_sum]; exact sum_congr rfl fun _ _ => by ring
  push_cast at this
  rw [e] at this
  linarith

/-- **Lemma "Gauss–Bonnet for turns"**, discrete. If the border turns as much left as
right (two corners inside at every border vertex but as many with one as with
three), the indices `4 - deg` inside add up to `4 χ`: `4` on a disc bounded by a
turn, `0` on an annulus between two turns. -/
theorem gauss_bonnet_turn (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin 4 → Vx)
    (border : Finset Vx) (χ : ℤ) (h : IsRegion ends corner border χ)
    (hturn : ∑ v ∈ border, ((2 : ℤ) - corners corner v) = 0) :
    ∑ v ∈ borderᶜ, ((4 : ℤ) - deg ends v) = 4 * χ := by
  have := gauss_bonnet ends corner border χ h
  linarith

/-- The border of a turn: straight (two corners) except at most one convex corner
(one) and one concave corner (three); then it turns by zero. -/
theorem turn_border_sum (corner : Fc → Fin 4 → Vx) (border : Finset Vx) (a b : Vx)
    (hab : a ≠ b) (ha : a ∈ border) (hb : b ∈ border) (hca : corners corner a = 1)
    (hcb : corners corner b = 3)
    (hstraight : ∀ v ∈ border, v ≠ a → v ≠ b → corners corner v = 2) :
    ∑ v ∈ border, ((2 : ℤ) - corners corner v) = 0 := by
  rw [Finset.sum_eq_add_of_mem a b ha hb hab fun v hv h => by
    rw [hstraight v hv h.1 h.2]; norm_num, hca, hcb]
  norm_num

/-- **A border with corners.** If every border vertex has one, two or three corners
inside (a convex corner, straight, a concave corner), with `a` convex and `b`
concave corners, the indices inside add up to `4 χ - a + b`. Lemma "One
self-crossing" is this count on the three regions of the curve cut open at the
crossing (each a disc, the crossing doubled on the border of the third region),
once the corners are known: `a - b = 1, 1, 2` when the crossing is on the walk,
giving `3, 3, 2`; on the closing segment, `(a, b) = (2, 0)` for each lobe and
`(2, 2)` for the third region, giving `2, 2, 4` (`one_crossing_on_walk`,
`one_crossing_on_segment`). -/
theorem gauss_bonnet_corners (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin 4 → Vx)
    (border : Finset Vx) (χ : ℤ) (h : IsRegion ends corner border χ)
    (hc : ∀ v ∈ border, corners corner v = 1 ∨ corners corner v = 2 ∨ corners corner v = 3) :
    ∑ v ∈ borderᶜ, ((4 : ℤ) - deg ends v) =
      4 * χ - #{v ∈ border | corners corner v = 1} + #{v ∈ border | corners corner v = 3} := by
  have hb : ∑ v ∈ border, ((2 : ℤ) - corners corner v) =
      #{v ∈ border | corners corner v = 1} - #{v ∈ border | corners corner v = 3} := by
    rw [card_filter, card_filter, Nat.cast_sum, Nat.cast_sum, ← sum_sub_distrib]
    refine sum_congr rfl fun v hv => ?_
    rcases hc v hv with h1 | h1 | h1 <;> simp [h1]
  have := gauss_bonnet ends corner border χ h
  rw [hb] at this
  linarith

/-- One region of Lemma "One self-crossing": a disc (`χ = 1`) whose border vertices
have one, two or three corners inside, with `e` more convex corners (one corner
inside) than concave ones (three corners inside), holds total index `4 - e`. -/
theorem disc_index_of_excess (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin 4 → Vx)
    (border : Finset Vx) (h : IsRegion ends corner border 1)
    (hc : ∀ v ∈ border, corners corner v = 1 ∨ corners corner v = 2 ∨ corners corner v = 3)
    (e : ℕ) (he : #{v ∈ border | corners corner v = 1} = #{v ∈ border | corners corner v = 3} + e) :
    ∑ v ∈ borderᶜ, ((4 : ℤ) - deg ends v) = 4 - e := by
  rw [gauss_bonnet_corners ends corner border 1 h hc, he]
  push_cast
  ring

section OneCrossing

variable {Vx₁ Ed₁ Fc₁ Vx₂ Ed₂ Fc₂ Vx₃ Ed₃ Fc₃ : Type*}
  [Fintype Vx₁] [DecidableEq Vx₁] [Fintype Ed₁] [Fintype Fc₁]
  [Fintype Vx₂] [DecidableEq Vx₂] [Fintype Ed₂] [Fintype Fc₂]
  [Fintype Vx₃] [DecidableEq Vx₃] [Fintype Ed₃] [Fintype Fc₃]

/-- **Lemma "One self-crossing", the crossing on the walk.** Two lobes and a third
region, each a disc of quads of the refined mesh (`IsRegion` with `χ = 1`), whose
border vertices have one (convex), two (straight) or three (concave) corners
inside; the border of the third region passes the crossing twice and holds it
as two vertices. If each lobe has one convex corner more than concave ones and
the third region two more, the indices `4 - k` inside add up to `3`, `3` and `2`.
In the note, the excess is the convex corner at the crossing (one per lobe, two
for the third region); the two corners of the closing segment, one convex and
one concave on each region that sees them, cancel. Which corners are convex is
the hypothesis. -/
theorem one_crossing_on_walk
    (ends₁ : Ed₁ → Fin 2 → Vx₁) (corner₁ : Fc₁ → Fin 4 → Vx₁) (border₁ : Finset Vx₁)
    (ends₂ : Ed₂ → Fin 2 → Vx₂) (corner₂ : Fc₂ → Fin 4 → Vx₂) (border₂ : Finset Vx₂)
    (ends₃ : Ed₃ → Fin 2 → Vx₃) (corner₃ : Fc₃ → Fin 4 → Vx₃) (border₃ : Finset Vx₃)
    (h₁ : IsRegion ends₁ corner₁ border₁ 1) (h₂ : IsRegion ends₂ corner₂ border₂ 1)
    (h₃ : IsRegion ends₃ corner₃ border₃ 1)
    (hc₁ : ∀ v ∈ border₁, corners corner₁ v = 1 ∨ corners corner₁ v = 2 ∨ corners corner₁ v = 3)
    (hc₂ : ∀ v ∈ border₂, corners corner₂ v = 1 ∨ corners corner₂ v = 2 ∨ corners corner₂ v = 3)
    (hc₃ : ∀ v ∈ border₃, corners corner₃ v = 1 ∨ corners corner₃ v = 2 ∨ corners corner₃ v = 3)
    (he₁ : #{v ∈ border₁ | corners corner₁ v = 1} = #{v ∈ border₁ | corners corner₁ v = 3} + 1)
    (he₂ : #{v ∈ border₂ | corners corner₂ v = 1} = #{v ∈ border₂ | corners corner₂ v = 3} + 1)
    (he₃ : #{v ∈ border₃ | corners corner₃ v = 1} = #{v ∈ border₃ | corners corner₃ v = 3} + 2) :
    ∑ v ∈ border₁ᶜ, ((4 : ℤ) - deg ends₁ v) = 3 ∧ ∑ v ∈ border₂ᶜ, ((4 : ℤ) - deg ends₂ v) = 3 ∧
      ∑ v ∈ border₃ᶜ, ((4 : ℤ) - deg ends₃ v) = 2 := by
  refine ⟨?_, ?_, ?_⟩
  · rw [disc_index_of_excess ends₁ corner₁ border₁ h₁ hc₁ 1 he₁]; norm_num
  · rw [disc_index_of_excess ends₂ corner₂ border₂ h₂ hc₂ 1 he₂]; norm_num
  · rw [disc_index_of_excess ends₃ corner₃ border₃ h₃ hc₃ 2 he₃]; norm_num

/-- **Lemma "One self-crossing", the crossing on the closing segment.** As
`one_crossing_on_walk`, with two convex corners more than concave ones on each
lobe and as many of each on the third region: the indices inside add up to `2`,
`2` and `4`. In the note, a lobe has two convex corners, at the crossing and at
an end of the closing segment, and the third region two convex corners (the
crossing, doubled) and two concave ones (the ends of the segment, seen from the
other side). -/
theorem one_crossing_on_segment
    (ends₁ : Ed₁ → Fin 2 → Vx₁) (corner₁ : Fc₁ → Fin 4 → Vx₁) (border₁ : Finset Vx₁)
    (ends₂ : Ed₂ → Fin 2 → Vx₂) (corner₂ : Fc₂ → Fin 4 → Vx₂) (border₂ : Finset Vx₂)
    (ends₃ : Ed₃ → Fin 2 → Vx₃) (corner₃ : Fc₃ → Fin 4 → Vx₃) (border₃ : Finset Vx₃)
    (h₁ : IsRegion ends₁ corner₁ border₁ 1) (h₂ : IsRegion ends₂ corner₂ border₂ 1)
    (h₃ : IsRegion ends₃ corner₃ border₃ 1)
    (hc₁ : ∀ v ∈ border₁, corners corner₁ v = 1 ∨ corners corner₁ v = 2 ∨ corners corner₁ v = 3)
    (hc₂ : ∀ v ∈ border₂, corners corner₂ v = 1 ∨ corners corner₂ v = 2 ∨ corners corner₂ v = 3)
    (hc₃ : ∀ v ∈ border₃, corners corner₃ v = 1 ∨ corners corner₃ v = 2 ∨ corners corner₃ v = 3)
    (he₁ : #{v ∈ border₁ | corners corner₁ v = 1} = #{v ∈ border₁ | corners corner₁ v = 3} + 2)
    (he₂ : #{v ∈ border₂ | corners corner₂ v = 1} = #{v ∈ border₂ | corners corner₂ v = 3} + 2)
    (he₃ : #{v ∈ border₃ | corners corner₃ v = 1} = #{v ∈ border₃ | corners corner₃ v = 3}) :
    ∑ v ∈ border₁ᶜ, ((4 : ℤ) - deg ends₁ v) = 2 ∧ ∑ v ∈ border₂ᶜ, ((4 : ℤ) - deg ends₂ v) = 2 ∧
      ∑ v ∈ border₃ᶜ, ((4 : ℤ) - deg ends₃ v) = 4 := by
  refine ⟨?_, ?_, ?_⟩
  · rw [disc_index_of_excess ends₁ corner₁ border₁ h₁ hc₁ 2 he₁]; norm_num
  · rw [disc_index_of_excess ends₂ corner₂ border₂ h₂ hc₂ 2 he₂]; norm_num
  · rw [disc_index_of_excess ends₃ corner₃ border₃ h₃ hc₃ 0 he₃]; norm_num

end OneCrossing

/-- **Triangles**, fields of order 6: `∑_{inside} (6 - deg v) + ∑_{border} (3 - corners v) = 6 χ`. -/
theorem gauss_bonnet_triangles (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin 3 → Vx)
    (border : Finset Vx) (χ : ℤ) (h : IsRegion ends corner border χ) :
    ∑ v ∈ borderᶜ, ((6 : ℤ) - deg ends v) + ∑ v ∈ border, ((3 : ℤ) - corners corner v) =
      6 * χ := by
  have := gauss_bonnet_general 6 (by norm_num) ends corner border χ h
  have e : ∑ v ∈ border, ((6 : ℤ) - 2 * corners corner v) =
      2 * ∑ v ∈ border, ((3 : ℤ) - corners corner v) := by
    rw [mul_sum]; exact sum_congr rfl fun _ _ => by ring
  push_cast at this
  rw [e] at this
  linarith

/-- **Hexagons**, fields of order 3:
`2 ∑_{inside} (3 - deg v) + ∑_{border} (3 - 2 corners v) = 6 χ`. -/
theorem gauss_bonnet_hexagons (ends : Ed → Fin 2 → Vx) (corner : Fc → Fin 6 → Vx)
    (border : Finset Vx) (χ : ℤ) (h : IsRegion ends corner border χ) :
    2 * ∑ v ∈ borderᶜ, ((3 : ℤ) - deg ends v) + ∑ v ∈ border, ((3 : ℤ) - 2 * corners corner v) =
      6 * χ := by
  have := gauss_bonnet_general 3 (by norm_num) ends corner border χ h
  push_cast at this
  linarith

end GaussBonnet

end TurnsOfATMesh
