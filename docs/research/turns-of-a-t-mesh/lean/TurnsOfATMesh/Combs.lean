/-
Lemma "Combs", the forms, and Proposition "Certificates are sums of combs", the
decomposition.

The identity of Lemma "Combs" is `comb_reading` (`Consequences.lean`) and
`Pairing.Comb.reading`. Here, in the chart of the comb, two feet of odd
valence developed at `q₀`, `q₁` with `⟨e_a, q₁ - q₀⟩ = ℓ_a` give the twisted
class of `λ_{c₀} λ_{c₁}^{±1}`, of trivial rotation, whose form against a whole
`v` with `(1 - R₀)ᵀ v = 2 e_a` is `-2 ℓ_a`: a dislocation when the valences
differ (`comb_dislocation`), an exchange when they are equal
(`comb_exchange`).

The decomposition is combinatorial. The arcs `α` have feet `foot₀`, `foot₁`
(the graph of feet, a multigraph whose loops are the arcs with one foot); a
closed walk of combs is a closed walk of that graph (`IsWalk`), its reading the
sum of the lengths of its arcs. When the odd part of a whole non-negative form
`∑ cₐ ℓₐ` has even degree at every vertex (a cycle modulo 2), it splits into
closed walks with distinct arcs (Veblen, `veblen`), and the form is the sum of
their readings and of `⌊cₐ/2⌋ · 2ℓₐ` (`sum_of_combs`). That the odd part of a
certificate is a cycle is the hypothesis of the proposition, not proved here.
-/
module

public import TurnsOfATMesh.Pairing

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace Combs

open Complex ComplexConjugate Motion Pairing

/-! ### Lemma "Combs", the forms of dislocations and exchanges -/

/-- A whole point of the plane: integer coordinates. -/
def IsWhole (z : ℂ) : Prop := ∃ m n : ℤ, z = m + n * Complex.I

section Forms

variable {ι : Type*} [Fintype ι] [DecidableEq ι]

/-- The form of the twisted class `w ⊗ v` of a word `w = λ_{c₀} λ_{c₁}^{±1}`
whose rotation is trivial is `⟨v, (1 - R₀)(q₀ - q₁)⟩`. -/
theorem form_pair (u : ι → ℂˣ) (q : ι → ℂ) (c₀ c₁ : ι) (s : Bool)
    (hrot : (u c₀ : ℂ) * letterLin u (c₁, s) = 1) (v : ℂ) :
    form u (twistedWinding {()} u (fun _ => 1) (fun _ => v) (fun _ => [(c₀, true), (c₁, s)])) q =
      dotC v ((1 - (u c₀ : ℂ)) * (q c₀ - q c₁)) := by
  sorry

/-- **Lemma "Combs", a dislocation.** Feet `c₀ ≠ c₁` of odd valence and
different valences (`R₁ = R₀⁻¹`, `R₀ = ±I`), developed in the chart of the comb
at `q`, with `⟨e_a, q_{c₁} - q_{c₀}⟩ = ℓ_a` for a unit axis `e_a`: the loop
`λ_{c₀} λ_{c₁}` has trivial rotation, its class is a cycle, and for the whole
`v = e_a (1 - R₀)`, with `(1 - R₀)ᵀ v = 2 e_a`, the form of `λ_{c₀} λ_{c₁} ⊗ v`
is `-2 ℓ_a`. -/
theorem comb_dislocation (u : ι → ℂˣ) (q : ι → ℂ) (c₀ c₁ : ι)
    (hR₀ : (u c₀ : ℂ) = Complex.I ∨ (u c₀ : ℂ) = -Complex.I)
    (hR₁ : (u c₁ : ℂ) = (u c₀ : ℂ)⁻¹) (e : ℂ)
    (he : e = 1 ∨ e = Complex.I ∨ e = -1 ∨ e = -Complex.I) (len : ℝ)
    (hlen : dotC e (q c₁ - q c₀) = len) :
    let W := twistedWinding {()} u (fun _ => 1) (fun _ => e * (1 - (u c₀ : ℂ)))
      (fun _ => [(c₀, true), (c₁, true)])
    rhoW u [(c₀, true), (c₁, true)] = 1 ∧ W ∈ cycles u ∧ IsWhole (e * (1 - (u c₀ : ℂ))) ∧
      conj (1 - (u c₀ : ℂ)) * (e * (1 - (u c₀ : ℂ))) = 2 * e ∧ form u W q = -2 * len := by
  sorry

/-- **Lemma "Combs", an exchange.** The same with equal valences (`R₁ = R₀`):
the loop is `λ_{c₀} λ_{c₁}⁻¹`. -/
theorem comb_exchange (u : ι → ℂˣ) (q : ι → ℂ) (c₀ c₁ : ι)
    (hR₀ : (u c₀ : ℂ) = Complex.I ∨ (u c₀ : ℂ) = -Complex.I)
    (hR₁ : (u c₁ : ℂ) = u c₀) (e : ℂ)
    (he : e = 1 ∨ e = Complex.I ∨ e = -1 ∨ e = -Complex.I) (len : ℝ)
    (hlen : dotC e (q c₁ - q c₀) = len) :
    let W := twistedWinding {()} u (fun _ => 1) (fun _ => e * (1 - (u c₀ : ℂ)))
      (fun _ => [(c₀, true), (c₁, false)])
    rhoW u [(c₀, true), (c₁, false)] = 1 ∧ W ∈ cycles u ∧ IsWhole (e * (1 - (u c₀ : ℂ))) ∧
      conj (1 - (u c₀ : ℂ)) * (e * (1 - (u c₀ : ℂ))) = 2 * e ∧ form u W q = -2 * len := by
  sorry

end Forms

/-! ### Closed walks of combs -/

section Walks

variable {α ι : Type*} [DecidableEq α] [DecidableEq ι] (foot₀ foot₁ : α → ι)

/-- Where a step leaves: an arc walked forward (`true`) from `foot₀` to `foot₁`,
backward from `foot₁` to `foot₀`. -/
def src (s : α × Bool) : ι := if s.2 then foot₀ s.1 else foot₁ s.1

/-- Where a step arrives. -/
def tgt (s : α × Bool) : ι := if s.2 then foot₁ s.1 else foot₀ s.1

/-- A walk of combs from `x` to `y`: each arc leaves from the foot where the
previous one arrived. -/
inductive IsWalk : ι → List (α × Bool) → ι → Prop
  | nil (x : ι) : IsWalk x [] x
  | cons {x y : ι} (s : α × Bool) (w : List (α × Bool)) :
      src foot₀ foot₁ s = x → IsWalk (tgt foot₀ foot₁ s) w y → IsWalk x (s :: w) y

/-- The reading `∑ᵢ ℓ_{aᵢ}` of a walk. -/
def reading (ℓ : α → ℝ) (w : List (α × Bool)) : ℝ := (w.map fun s => ℓ s.1).sum

/-- The degree modulo 2 of `v` in a set of arcs: the ends of its arcs at `v`, a
loop counting twice. -/
def deg (S : Finset α) (v : ι) : ZMod 2 :=
  ∑ a ∈ S, ((if foot₀ a = v then 1 else 0) + (if foot₁ a = v then 1 else 0))

/-- **Veblen.** A set of arcs with even degree at every vertex (a cycle modulo 2
of the graph of feet) is the union of closed walks with distinct arcs: every arc
of `S` is walked exactly once, and no other. -/
theorem veblen (S : Finset α) (hS : ∀ v, deg foot₀ foot₁ S v = 0) :
    ∃ walks : List (List (α × Bool)),
      (∀ w ∈ walks, w ≠ [] ∧ ∃ x, IsWalk foot₀ foot₁ x w x) ∧
      ∀ a, (walks.flatten.map Prod.fst).count a = if a ∈ S then 1 else 0 := by
  sorry

/-- **Proposition "Certificates are sums of combs", whole weights.** Let
`c = ∑ cₐ ℓₐ`, `cₐ ∈ ℕ`, be a form whose arcs (`cₐ ≠ 0`) have feet of odd
valence (`odd`) and whose odd part (the arcs with `cₐ` odd) is a cycle modulo 2
of the graph of feet. Then `c` is the sum of the comb readings `2ℓₐ`, with the
whole weights `⌊cₐ/2⌋`, and of the readings of closed walks of combs through
feet of odd valence. -/
theorem sum_of_combs (c : α → ℕ) [Fintype α] (odd : ι → Prop)
    (hfeet : ∀ a, c a ≠ 0 → odd (foot₀ a) ∧ odd (foot₁ a))
    (hcycle : ∀ v, deg foot₀ foot₁ (Finset.univ.filter fun a => Odd (c a)) v = 0) :
    ∃ walks : List (List (α × Bool)),
      (∀ w ∈ walks, w ≠ [] ∧ (∃ x, IsWalk foot₀ foot₁ x w x) ∧
        ∀ s ∈ w, Odd (c s.1) ∧ odd (src foot₀ foot₁ s)) ∧
      ∀ ℓ : α → ℝ, ∑ a, (c a : ℝ) * ℓ a =
        (walks.map (reading ℓ)).sum + ∑ a, ((c a / 2 : ℕ) : ℝ) * (2 * ℓ a) := by
  sorry

/-- **Proposition "Certificates are sums of combs", real weights.** Every
non-negative form is the non-negative combination of the comb readings `2ℓₐ`
of the arcs with two distinct feet, with weights `cₐ/2`, and of the lengths
`ℓₐ` (winding separatrices) of the arcs whose two feet are one vertex. -/
theorem sum_of_combs_real [Fintype α] (c : α → ℝ) (ℓ : α → ℝ) :
    ∑ a, c a * ℓ a =
      ∑ a ∈ Finset.univ.filter (fun a => foot₀ a ≠ foot₁ a), c a / 2 * (2 * ℓ a) +
        ∑ a ∈ Finset.univ.filter (fun a => foot₀ a = foot₁ a), c a * ℓ a := by
  sorry

end Walks

end Combs

end TurnsOfATMesh
