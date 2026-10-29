/-
The reduction of Theorem "Choosing the turns is NP-hard": for a graph, four
arcs per edge `e = uv` (`y_{u,e}`, `y_{v,e}`, `d_e`, `d'_e`), the balances
`y_{u,e} + y_{v,e} = d_e = d'_e`, the bounds `d_e, d'_e ≥ 1`, `y ≥ 0`, and for
each vertex the form `shift_u = ∑_{e ∋ u} y_{u,e}`. Closing `shift_u` and
`shift_v` on an edge `uv` forces `d_e = 0`, below its bound; and an independent
set is closed by lengths `0` and `1`. So the closable sets are exactly the
independent sets, and `τ_P(V)` is the vertex cover number of the graph. That
Independent Set and Vertex Cover are NP-hard is cited, not formalized; nor is
it formalized that the instance is a T-mesh (it is not known to be one).

The graph is given by its edges `E` with two ends each, `ends e = (u, v)`;
loops are excluded, multiple edges allowed.
-/
module

public import TurnsOfATMesh.Forced
public import Mathlib.Algebra.Order.BigOperators.Group.Finset
public import Mathlib.Data.Fintype.BigOperators
public import Mathlib.Tactic.Linarith

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace Gadget

variable {V E : Type*} [Fintype E] [DecidableEq V]

/-- The lengths of the four arcs of every edge. -/
structure Lengths (E : Type*) where
  /-- `y_{u,e}`, on the side of the first end. -/
  y₀ : E → ℝ
  /-- `y_{v,e}`, on the side of the second end. -/
  y₁ : E → ℝ
  d : E → ℝ
  d' : E → ℝ

variable (ends : E → V × V)

/-- The balances, the bounds `d, d' ≥ 1` and `y ≥ 0`: the set `P` of the lengths. -/
def Feasible (ℓ : Lengths E) : Prop :=
  (∀ e, ℓ.y₀ e + ℓ.y₁ e - ℓ.d e = 0) ∧ (∀ e, ℓ.y₀ e + ℓ.y₁ e - ℓ.d' e = 0) ∧
    (∀ e, ℓ.d e - ℓ.d' e = 0) ∧ (∀ e, 1 ≤ ℓ.d e) ∧ (∀ e, 1 ≤ ℓ.d' e) ∧
    (∀ e, 0 ≤ ℓ.y₀ e) ∧ (∀ e, 0 ≤ ℓ.y₁ e)

/-- The term of `shift_u` for the edge `e`: `y_{u,e}` if `u` is an end of `e`. -/
def term (ℓ : Lengths E) (u : V) (e : E) : ℝ :=
  (if (ends e).1 = u then ℓ.y₀ e else 0) + (if (ends e).2 = u then ℓ.y₁ e else 0)

/-- `shift_u = ∑_{e ∋ u} y_{u,e}`, a form with coefficients `0` and `1`. -/
def shift (ℓ : Lengths E) (u : V) : ℝ := ∑ e, term ends ℓ u e

/-- The shifts as the turns of `Forced.lean`: `shift γ ℓ`. -/
def shiftOf (u : V) (ℓ : Lengths E) : ℝ := shift ends ℓ u

/-- No edge has both ends in `Y`. -/
def Independent (Y : Finset V) : Prop := ∀ e, ¬ ((ends e).1 ∈ Y ∧ (ends e).2 ∈ Y)

/-- Every edge has an end in `X`. -/
def IsVertexCover (X : Finset V) : Prop := ∀ e, (ends e).1 ∈ X ∨ (ends e).2 ∈ X

theorem term_nonneg (ℓ : Lengths E) (hℓ : Feasible ℓ) (u : V) (e : E) : 0 ≤ term ends ℓ u e := by
  obtain ⟨-, -, -, -, -, hy₀, hy₁⟩ := hℓ
  unfold term
  split_ifs <;> linarith [hy₀ e, hy₁ e]

/-- If `shift_u(ℓ) = 0` with `ℓ` feasible, every `y_{u,e}` at `u` is zero. -/
theorem y_eq_zero_of_shift (ℓ : Lengths E) (hℓ : Feasible ℓ) (u : V) (hu : shift ends ℓ u = 0)
    (e : E) : ((ends e).1 = u → ℓ.y₀ e = 0) ∧ ((ends e).2 = u → ℓ.y₁ e = 0) := by
  have hterm : term ends ℓ u e = 0 :=
    (Finset.sum_eq_zero_iff_of_nonneg fun e _ => term_nonneg ends ℓ hℓ u e).mp hu e
      (Finset.mem_univ e)
  obtain ⟨-, -, -, -, -, hy₀, hy₁⟩ := hℓ
  unfold term at hterm
  constructor
  · intro h0
    simp only [h0, ite_true] at hterm
    split_ifs at hterm <;> linarith [hy₀ e, hy₁ e]
  · intro h1
    simp only [h1, ite_true] at hterm
    split_ifs at hterm <;> linarith [hy₀ e, hy₁ e]

/-- **Closing two adjacent vertices is impossible**: `y_{u,e} = y_{v,e} = 0` gives
`d_e = 0 < 1`. -/
theorem independent_of_closable (Y : Finset V) (ℓ : Lengths E) (hℓ : Feasible ℓ)
    (hY : ∀ u ∈ Y, shift ends ℓ u = 0) : Independent ends Y := by
  rintro e ⟨h0, h1⟩
  have hy0 := (y_eq_zero_of_shift ends ℓ hℓ _ (hY _ h0) e).1 rfl
  have hy1 := (y_eq_zero_of_shift ends ℓ hℓ _ (hY _ h1) e).2 rfl
  obtain ⟨hd, -, -, hd1, -, -, -⟩ := hℓ
  linarith [hd e, hd1 e]

/-- The lengths closing an independent set: `y = 0` on the side of a vertex of
`Y`, `1` on the other side (or on the first side when neither end is in `Y`),
`d = d' = 1`. Whole lengths. -/
def witness (Y : Finset V) : Lengths E where
  y₀ e := if (ends e).1 ∈ Y then 0 else 1
  y₁ e := if (ends e).1 ∈ Y then 1 else 0
  d _ := 1
  d' _ := 1

theorem witness_feasible (Y : Finset V) : Feasible (witness ends Y) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_⟩ <;> intro e <;> simp only [witness] <;> (try split_ifs) <;>
    norm_num

/-- **An independent set closes**, with the witness, when the graph has no loop. -/
theorem witness_closes (hloop : ∀ e, (ends e).1 ≠ (ends e).2) (Y : Finset V)
    (hY : Independent ends Y) : ∀ u ∈ Y, shift ends (witness ends Y) u = 0 := by
  intro u hu
  apply Finset.sum_eq_zero
  intro e _
  unfold term witness
  by_cases h0 : (ends e).1 = u
  · have hin : (ends e).1 ∈ Y := h0 ▸ hu
    have h1 : (ends e).2 ≠ u := fun h => hloop e (h0.trans h.symm)
    simp [h0, h1, hu]
  · by_cases h1 : (ends e).2 = u
    · have hnot : (ends e).1 ∉ Y := fun hin => hY e ⟨hin, h1 ▸ hu⟩
      simp [h0, h1, hnot]
    · simp [h0, h1]

/-- **Theorem "Choosing the turns is NP-hard", the reduction.** On the gadget of
a graph without loops, a set of vertices can be closed within `P` exactly when it
is independent. -/
theorem closable_iff_independent (hloop : ∀ e, (ends e).1 ≠ (ends e).2) (Y : Finset V) :
    Closable (shiftOf ends) {ℓ : Lengths E | Feasible ℓ} Y ↔ Independent ends Y := by
  constructor
  · rintro ⟨ℓ, hℓ, hY⟩
    exact independent_of_closable ends Y ℓ hℓ hY
  · intro hY
    exact ⟨witness ends Y, witness_feasible ends Y, witness_closes ends hloop Y hY⟩

variable [Fintype V]

theorem independent_sdiff_iff (X : Finset V) :
    Independent ends (Finset.univ \ X) ↔ IsVertexCover ends X := by
  unfold Independent IsVertexCover
  simp only [Finset.mem_sdiff, Finset.mem_univ, true_and]
  constructor
  · intro h e
    by_contra hc
    exact h e ⟨(not_or.mp hc).1, (not_or.mp hc).2⟩
  · rintro h e ⟨h0, h1⟩
    rcases h e with h' | h'
    · exact h0 h'
    · exact h1 h'

/-- **`τ_P(V)` is the vertex cover number** of the graph: the fewest vertices to
let go for the others to close together are the fewest vertices meeting every
edge. -/
theorem tauP_eq_vertex_cover (hloop : ∀ e, (ends e).1 ≠ (ends e).2) :
    tauP (shiftOf ends) {ℓ : Lengths E | Feasible ℓ} Finset.univ =
      sInf {n | ∃ X : Finset V, X.card = n ∧ IsVertexCover ends X} := by
  unfold tauP
  congr 1
  ext n
  simp only [Set.mem_ofPred_eq, Finset.subset_univ, true_and]
  constructor
  · rintro ⟨X, hX, hc⟩
    exact ⟨X, hX, (independent_sdiff_iff ends X).mp ((closable_iff_independent ends hloop _).mp hc)⟩
  · rintro ⟨X, hX, hc⟩
    exact ⟨X, hX, (closable_iff_independent ends hloop _).mpr ((independent_sdiff_iff ends X).mpr hc)⟩

end Gadget

end TurnsOfATMesh
