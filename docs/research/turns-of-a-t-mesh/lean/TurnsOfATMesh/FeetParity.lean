/-
Proposition "Feet modulo 2": the combinatorial and algebraic cores.

For a singular vertex `v`, `χ_v(a)` counts the feet of the arc `a` at `v`. Modulo 2
it is the coboundary of the node function `g_v`, the number of traces from `v`
through a node (zero at a vertex): at each end of an arc on the trace `τ`, the
foot is at `v` exactly when `g_v` plus `[τ leaves v]` is odd, and the two ends
cancel the second term. The developed parity of every vertex is then constant,
and the checkerboard rule for any combination makes every combination of shifts
even at `χ_v`. The balances read `χ_v` even (the corners of a patch on a trace
from `v` come in pairs, on paper), so every whole combination of shifts and
balances does: its odd part is a cycle of the graph of feet.
-/
module

public import TurnsOfATMesh.Checkerboard

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

/-! ### The feet of an arc, modulo 2 -/

section Feet

variable {Node Trace : Type*} [DecidableEq Node] [DecidableEq Trace]

/-- `[P]` in `ZMod 2`. -/
def ind (P : Prop) [Decidable P] : ZMod 2 := if P then 1 else 0

/-- The node function `g_v`: the traces through `n` traced from `v`, counted
modulo 2, zero at a vertex (a node that traces leave). -/
def fromVertex (origin : Trace → Node) (through : Node → Finset Trace) (isVertex : Node → Prop)
    [DecidablePred isVertex] (v n : Node) : ZMod 2 :=
  if isVertex n then 0 else (((through n).filter fun t => origin t = v).card : ZMod 2)

/-- The hypotheses at one end `n`, with foot `f`, of an arc on the trace `τ`:
either `n` is the vertex `τ` leaves, its own foot, or `n` is a junction met by
`τ` and exactly one other trace `ρ`, and the foot is the vertex `ρ` leaves. No
trace ends at a vertex. -/
def EndOfArc (origin : Trace → Node) (through : Node → Finset Trace) (isVertex : Node → Prop)
    (τ : Trace) (n f : Node) : Prop :=
  (isVertex n ∧ origin τ = n ∧ f = n) ∨
    (¬ isVertex n ∧ ∃ ρ, ρ ≠ τ ∧ through n = {τ, ρ} ∧ f = origin ρ)

/-- At one end of an arc on `τ`, the foot is at `v` exactly when `g_v(n) + [τ leaves v]`
is odd. -/
theorem foot_at_end (origin : Trace → Node) (through : Node → Finset Trace)
    (isVertex : Node → Prop) [DecidablePred isVertex] (τ : Trace) (n f v : Node)
    (h : EndOfArc origin through isVertex τ n f) :
    ind (f = v) = fromVertex origin through isVertex v n + ind (origin τ = v) := by
  sorry

/-- **Proposition "Feet modulo 2" (a), on an arc.** For an arc on the trace `τ`
with ends `n₀`, `n₁` and feet `f₀`, `f₁`, the number of its feet at `v` is the
coboundary of `g_v` modulo 2. -/
theorem feet_coboundary (origin : Trace → Node) (through : Node → Finset Trace)
    (isVertex : Node → Prop) [DecidablePred isVertex] (τ : Trace) (n₀ n₁ f₀ f₁ v : Node)
    (h₀ : EndOfArc origin through isVertex τ n₀ f₀) (h₁ : EndOfArc origin through isVertex τ n₁ f₁) :
    ind (f₀ = v) + ind (f₁ = v) =
      fromVertex origin through isVertex v n₀ + fromVertex origin through isVertex v n₁ := by
  sorry

end Feet

/-! ### Every combination of shifts is even at `χ_v` -/

section Combination

variable {ι : Type*} [DecidableEq ι] [Fintype ι]

/-- **Proposition "Feet modulo 2" (b), the algebraic step.** If every vertex of odd
valence (`jᵢ` odd) develops onto a point of one colour, every whole combination of
the components of the translations of words with trivial rotation is even. Applied
to integer lifts of the development modulo 2 at the lengths `χ_v`, under which every
vertex has the colour of the base point. -/
theorem combination_even_of_constant_colour {K : Type*} (s : Finset K) (j : ι → ℤ)
    (p : ι → GaussianInt) (c : K → ℤ) (dir : K → Fin 4) (w : K → List (ι × Bool))
    (hrot : ∀ r ∈ s, rhoW (powers giUnit j) (w r) = 1) (κ : ZMod 2)
    (hcol : ∀ i, Odd (j i) → colour (p i) = κ) :
    ((∑ r ∈ s, c r * component (dir r) (hol (powers giUnit j) p (w r)).trans : ℤ) : ZMod 2) = 0 := by
  sorry

end Combination

/-! ### Whole combinations of shifts and balances -/

section Assembly

variable {Arc Row K : Type*} [Fintype Arc] [Fintype Row] [Fintype K]

/-- **Proposition "Feet modulo 2" (c).** If every shift form and every balance reads
the lengths `χ` even, so does every whole combination of them: for `χ = χ_v`, the
arcs with an odd coefficient meet `v` an even number of times as feet. -/
theorem combination_reads_even (shift : K → Arc → ℤ) (B : Row → Arc → ℤ) (u : K → ℤ) (y : Row → ℤ)
    (χ : Arc → ℤ) (hs : ∀ r, ((∑ a, shift r a * χ a : ℤ) : ZMod 2) = 0)
    (hB : ∀ k, ((∑ a, B k a * χ a : ℤ) : ZMod 2) = 0) :
    ((∑ a, (∑ r, u r * shift r a + ∑ k, y k * B k a) * χ a : ℤ) : ZMod 2) = 0 := by
  sorry

end Assembly

end TurnsOfATMesh
