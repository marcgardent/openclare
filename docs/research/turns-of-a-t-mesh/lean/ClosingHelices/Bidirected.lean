/-
The balances as a bidirected graph: Proposition "Free arcs".

The matrix `B` of the balances has one row per balance (a node of the graph) and
one column per arc (an edge); every column has exactly two non-zero entries, at
two distinct nodes, each `+1` or `-1` (`IsBidirected`).

A minimal theory of such graphs, enough for the proposition:

* a **tree** on a set of nodes `S` with edges `T` (`IsTree`) is grown from one
  node by adding leaves, one new node and one edge joining it to the tree at a
  time. Every finite tree grows so (remove its leaves in the reverse order) and
  everything grown so is a tree; a spanning tree is a tree on every node, and a
  finite graph has one exactly when it is connected;
* a **switching** of a tree (`IsSwitching`) is a choice of signs `w` on its
  nodes for which every edge of the tree has `∑_r w r B r e = 0`. Every tree has
  one (`IsTree.exists_switching`), and the graph is **balanced** if one choice
  of signs does it for every edge (`IsBalancedGraph`), which is Harary's
  characterization of a signed graph all of whose cycles are balanced;
* an edge `f` **closes an unbalanced cycle** with a tree `T` (`ClosesUnbalanced`)
  if no non-zero values on `T ∪ {f}` are balanced at every node. Otherwise the
  fundamental cycle of `f` carries balanced values `±1`
  (`balanced_cycle_of_not_closesUnbalanced`), which is the note's definition of a
  balanced cycle.

The proofs follow the note: a leaf of the tree fixes the value of its edge, and
the unbalanced cycle fixes the last one. They work with integer values
throughout: the switching `w` takes the place of the determinant, since
`∑_r w r d_r` and `∑_r d_r` have the same parity and `∑_r w r B r f = ±2`.
-/
module

public import ClosingHelices.Balances
public import Mathlib.Data.Matrix.Basic
public import Mathlib.LinearAlgebra.LinearIndependent.Defs
public import Mathlib.Algebra.Order.BigOperators.Group.Finset
public import Mathlib.Algebra.Order.Ring.Abs
public import Mathlib.Data.Fintype.BigOperators
public import Mathlib.Logic.Equiv.Fin.Rotate
public import Mathlib.Tactic.Linarith
public import Mathlib.Tactic.Ring
public import Mathlib.Tactic.LinearCombination
public import Mathlib.Tactic.Push
public import Mathlib.Tactic.FieldSimp
public import Mathlib.Tactic.Positivity
public import Mathlib.Data.Rat.Cast.Order

@[expose] public section

set_option linter.unusedSectionVars false

namespace ClosingHelices

namespace Bidirected

open Finset Matrix

/-- `a` is `+1` or `-1`. -/
def IsSign (a : ℤ) : Prop := a = 1 ∨ a = -1

theorem IsSign.mul_self {a : ℤ} (h : IsSign a) : a * a = 1 := by
  rcases h with rfl | rfl <;> rfl

theorem IsSign.abs {a : ℤ} (h : IsSign a) : |a| = 1 := by
  rcases h with rfl | rfl <;> rfl

theorem IsSign.ne_zero {a : ℤ} (h : IsSign a) : a ≠ 0 := by
  rcases h with rfl | rfl <;> decide

theorem IsSign.mul {a b : ℤ} (ha : IsSign a) (hb : IsSign b) : IsSign (a * b) := by
  rcases ha with rfl | rfl <;> rcases hb with rfl | rfl <;> simp [IsSign]

theorem IsSign.neg {a : ℤ} (ha : IsSign a) : IsSign (-a) := by
  rcases ha with rfl | rfl <;> simp [IsSign]

/-- `a ≡ 1 (mod 2)` for a sign, so `w r * d - d` is even. -/
theorem IsSign.even_mul_sub {a : ℤ} (h : IsSign a) (d : ℤ) : Even (a * d - d) := by
  rcases h with rfl | rfl
  · simp
  · exact ⟨-d, by ring⟩

/-- A sum of two signs is `0` or `±2`. -/
theorem IsSign.add_eq {a b : ℤ} (ha : IsSign a) (hb : IsSign b) :
    a + b = 0 ∨ a + b = 2 ∨ a + b = -2 := by
  rcases ha with rfl | rfl <;> rcases hb with rfl | rfl <;> simp

variable {V E : Type*} [Fintype V] [DecidableEq V] [Fintype E] [DecidableEq E]

/-- Every column of `B` has exactly two non-zero entries, at two distinct nodes,
each `+1` or `-1`: the incidence matrix of a bidirected graph. -/
def IsBidirected (B : Matrix V E ℤ) : Prop :=
  ∀ e, ∃ u v, u ≠ v ∧ IsSign (B u e) ∧ IsSign (B v e) ∧ ∀ r, r ≠ u → r ≠ v → B r e = 0

/-- The columns of a bidirected incidence matrix have even sums (Lemma "Handshake"). -/
theorem IsBidirected.column_even {B : Matrix V E ℤ} (hB : IsBidirected B) (e : E) :
    Even (∑ r, B r e) := by
  obtain ⟨u, v, huv, hu, hv, h0⟩ := hB e
  exact ClosingHelices.column_even (fun r => B r e) u v huv h0 hu hv

/-- `∑_r w r B r e` for signs `w`: `0` or `±2`. -/
theorem IsBidirected.pair_eq {B : Matrix V E ℤ} (hB : IsBidirected B) (w : V → ℤ)
    (hw : ∀ v, IsSign (w v)) (e : E) :
    ∑ r, w r * B r e = 0 ∨ ∑ r, w r * B r e = 2 ∨ ∑ r, w r * B r e = -2 := by
  obtain ⟨u, v, huv, hu, hv, h0⟩ := hB e
  rw [Finset.sum_eq_add_of_mem u v (mem_univ _) (mem_univ _) huv
    (fun r _ h => by rw [h0 r h.1 h.2, mul_zero])]
  exact ((hw u).mul hu).add_eq ((hw v).mul hv)

/-- The sum of the entries of `B` in the column `e`, in absolute value: `2`. -/
theorem IsBidirected.sum_abs {B : Matrix V E ℤ} (hB : IsBidirected B) (e : E) :
    ∑ r, |B r e| = 2 := by
  obtain ⟨u, v, huv, hu, hv, h0⟩ := hB e
  rw [Finset.sum_eq_add_of_mem u v (mem_univ _) (mem_univ _) huv
    (fun r _ h => by rw [h0 r h.1 h.2, abs_zero]), hu.abs, hv.abs]
  rfl

/-! ### Trees -/

/-- `IsTree B S T`: the edges `T` form a tree on the nodes `S`, grown from one node by
adding leaves. A leaf `v` is joined to a node `u` of the tree by an edge `e` whose
column is `±1` at `v` and at `u` and zero elsewhere. -/
inductive IsTree (B : Matrix V E ℤ) : Finset V → Finset E → Prop
  | node (v : V) : IsTree B {v} ∅
  | leaf {S : Finset V} {T : Finset E} (v u : V) (e : E) :
      IsTree B S T → v ∉ S → u ∈ S → e ∉ T → IsSign (B v e) → IsSign (B u e) →
      (∀ r, r ≠ v → r ≠ u → B r e = 0) → IsTree B (insert v S) (insert e T)

variable {B : Matrix V E ℤ}

/-- An edge of a tree has its column supported on the nodes of the tree. -/
theorem IsTree.support {S : Finset V} {T : Finset E} (hT : IsTree B S T) :
    ∀ e ∈ T, ∀ r ∉ S, B r e = 0 := by
  induction hT with
  | node v => simp
  | leaf v u e hT hv hu he hBv hBu h0 ih =>
    intro e' he' r hr
    rw [mem_insert, not_or] at hr
    rcases mem_insert.mp he' with rfl | he'
    · exact h0 r hr.1 (fun h => hr.2 (h ▸ hu))
    · exact ih e' he' r hr.2

/-- Signs on the nodes of a tree for which every edge of the tree is balanced:
`∑_{r ∈ S} w r B r e = 0`. -/
def IsSwitching (B : Matrix V E ℤ) (S : Finset V) (T : Finset E) (w : V → ℤ) : Prop :=
  (∀ v ∈ S, IsSign (w v)) ∧ ∀ e ∈ T, ∑ r ∈ S, w r * B r e = 0

/-- Every tree has a switching. -/
theorem IsTree.exists_switching {S : Finset V} {T : Finset E} (hT : IsTree B S T) :
    ∃ w, IsSwitching B S T w := by
  induction hT with
  | node v => exact ⟨fun _ => 1, by simp [IsSign], by simp⟩
  | @leaf S T v u e hT hv hu he hBv hBu h0 ih =>
    obtain ⟨w, hws, hwe⟩ := ih
    have hsupp := hT.support
    let w' := Function.update w v (-(w u * B u e * B v e))
    have hw'S : ∀ r ∈ S, w' r = w r := fun r hr =>
      Function.update_of_ne (by rintro rfl; exact hv hr) _ _
    have hw'v : w' v = -(w u * B u e * B v e) := Function.update_self _ _ _
    refine ⟨w', fun r hr => ?_, fun e' he' => ?_⟩
    · rcases mem_insert.mp hr with rfl | hr
      · rw [hw'v]; exact (((hws u hu).mul hBu).mul hBv).neg
      · rw [hw'S r hr]; exact hws r hr
    · rw [sum_insert hv, sum_congr rfl fun r hr => by rw [hw'S r hr]]
      rcases mem_insert.mp he' with h | he'
      · rw [h, sum_eq_single u (fun r hr hru => by
            rw [h0 r (fun h => hv (h ▸ hr)) hru, mul_zero]) (fun h => absurd hu h), hw'v]
        have := hBv.mul_self
        linear_combination (-(w u * B u e)) * this
      · rw [hsupp e' he' v hv, mul_zero, zero_add]
        exact hwe e' he'

/-- A switching of a tree is one of every smaller tree it was grown from. -/
theorem IsSwitching.of_leaf {S : Finset V} {T : Finset E} {v : V} {e : E} {w : V → ℤ}
    (hT : IsTree B S T) (hv : v ∉ S) (hw : IsSwitching B (insert v S) (insert e T) w) :
    IsSwitching B S T w := by
  refine ⟨fun r hr => hw.1 r (mem_insert_of_mem hr), fun e' he' => ?_⟩
  have := hw.2 e' (mem_insert_of_mem he')
  rwa [sum_insert hv, hT.support e' he' v hv, mul_zero, zero_add] at this

/-- **Injectivity on a tree.** Values on the edges of a tree that are balanced at every
node are zero: a leaf forces its edge to zero, and so on. -/
theorem IsTree.eq_zero {S : Finset V} {T : Finset E} (hT : IsTree B S T) (y : E → ℤ)
    (hy : ∀ e ∉ T, y e = 0) (h : B *ᵥ y = 0) : y = 0 := by
  induction hT generalizing y with
  | node v => funext e; exact hy e (notMem_empty e)
  | @leaf S T v u e hT hv hu he hBv hBu h0 ih =>
    have hsupp := hT.support
    have hv0 : (B *ᵥ y) v = B v e * y e := by
      simp only [Matrix.mulVec, dotProduct]
      refine sum_eq_single e (fun e' _ he'e => ?_) (fun h => absurd (mem_univ e) h)
      by_cases he'T : e' ∈ T
      · rw [hsupp e' he'T v hv, zero_mul]
      · rw [hy e' (by simp [he'e, he'T]), mul_zero]
    have hye : y e = 0 := by
      rw [h] at hv0
      exact (mul_eq_zero.mp hv0.symm).resolve_left hBv.ne_zero
    refine ih y (fun e' he' => ?_) h
    by_cases h' : e' = e
    · rw [h', hye]
    · exact hy e' (by simp [h', he'])

/-- **Solving on a tree.** On a tree with a switching `w`, every integer demand `d` on
its nodes with `∑_r w r d_r = 0` is `B y` for integer values `y` on its edges, and
`2 |y e| ≤ ∑_r |d_r|`. -/
theorem IsTree.solve {S : Finset V} {T : Finset E} (hT : IsTree B S T) (w : V → ℤ)
    (hw : IsSwitching B S T w) (d : V → ℤ) (hd : ∀ r ∉ S, d r = 0)
    (hwd : ∑ r ∈ S, w r * d r = 0) :
    ∃ y : E → ℤ, (∀ e ∉ T, y e = 0) ∧ B *ᵥ y = d ∧ ∀ e, 2 * |y e| ≤ ∑ r, |d r| := by
  induction hT generalizing d with
  | node v =>
    refine ⟨0, fun _ _ => rfl, ?_, fun e => by simp [Finset.sum_nonneg]⟩
    funext r
    simp only [Matrix.mulVec_zero, Pi.zero_apply]
    by_cases hr : r = v
    · subst hr
      have hs := hw.1 r (mem_singleton_self r)
      rw [sum_singleton] at hwd
      exact ((mul_eq_zero.mp hwd).resolve_left hs.ne_zero).symm
    · exact (hd r (by simpa using hr)).symm
  | @leaf S T v u e hT hv hu he hBv hBu h0 ih =>
    have hsupp := hT.support
    have hwS := hw.of_leaf hT hv
    have hwe : ∑ r ∈ insert v S, w r * B r e = 0 := hw.2 e (mem_insert_self _ _)
    have huv : u ≠ v := fun h => hv (h ▸ hu)
    set c := d v * B v e with hc
    set d' : V → ℤ := fun r => d r - c * B r e with hd'
    have hd'v : d' v = 0 := by
      simp only [hd', hc]; linear_combination (-(d v)) * hBv.mul_self
    have hd'S : ∀ r ∉ S, d' r = 0 := by
      intro r hr
      by_cases hrv : r = v
      · rw [hrv, hd'v]
      · have hru : r ≠ u := fun h => hr (h ▸ hu)
        simp only [hd', hd r (by simp [hrv, hr]), h0 r hrv hru, mul_zero, sub_zero]
    have hwd' : ∑ r ∈ S, w r * d' r = 0 := by
      have h1 : ∑ r ∈ insert v S, w r * d' r = ∑ r ∈ S, w r * d' r := by
        rw [sum_insert hv, hd'v, mul_zero, zero_add]
      have h2 : ∑ r ∈ insert v S, w r * d' r =
          ∑ r ∈ insert v S, w r * d r - c * ∑ r ∈ insert v S, w r * B r e := by
        rw [mul_sum, ← sum_sub_distrib]
        refine sum_congr rfl fun r _ => ?_
        simp only [hd']; ring
      rw [← h1, h2, hwd, hwe, mul_zero, sub_zero]
    obtain ⟨y', hy'T, hy'B, hy'bound⟩ := ih hwS d' hd'S hwd'
    refine ⟨y' + fun e' => if e' = e then c else 0, fun e' he' => ?_, ?_, ?_⟩
    · have h1 : e' ≠ e := fun h => he' (h ▸ mem_insert_self _ _)
      have h2 : e' ∉ T := fun h => he' (mem_insert_of_mem h)
      simp [h1, hy'T e' h2]
    · funext r
      have := congrFun hy'B r
      simp only [Matrix.mulVec, dotProduct, Pi.add_apply, mul_add, sum_add_distrib,
        mul_ite, mul_zero, sum_ite_eq', mem_univ, ite_true] at this ⊢
      rw [this]; simp only [hd']; ring
    · -- `2 |c| ≤ ∑ |d|` and `∑ |d'| ≤ ∑ |d|`.
      have hcabs : |c| = |d v| := by rw [hc, abs_mul, hBv.abs, mul_one]
      have hsumS : ∑ r ∈ S, |d r| ≤ ∑ r ∈ univ \ {v}, |d r| :=
        sum_le_sum_of_subset_of_nonneg (fun r hr => by
          simp only [mem_sdiff, mem_univ, mem_singleton, true_and]
          exact fun h => hv (h ▸ hr)) (fun _ _ _ => abs_nonneg _)
      have hsplit : ∑ r, |d r| = |d v| + ∑ r ∈ univ \ {v}, |d r| := by
        rw [← sum_sdiff (s₁ := {v}) (subset_univ _), sum_singleton, add_comm]
      have hdv : |d v| ≤ ∑ r ∈ S, |d r| := by
        rw [sum_insert hv] at hwd
        have hwv : |w v * d v| = |d v| := by rw [abs_mul, (hw.1 v (mem_insert_self _ _)).abs,
          one_mul]
        have : w v * d v = -∑ r ∈ S, w r * d r := by linarith
        rw [← hwv, this, abs_neg]
        refine (abs_sum_le_sum_abs _ _).trans (le_of_eq (sum_congr rfl fun r hr => ?_))
        rw [abs_mul, (hwS.1 r hr).abs, one_mul]
      have hc2 : 2 * |c| ≤ ∑ r, |d r| := by rw [hcabs, hsplit]; linarith
      have hd'sum : ∑ r, |d' r| ≤ ∑ r, |d r| := by
        have hterm : ∀ r, |d' r| ≤ |d r| + |c| * |B r e| -
            (if r = v then 2 * |d v| else 0) := by
          intro r
          by_cases hrv : r = v
          · rw [hrv, hd'v, ite_eq_left rfl, hBv.abs, hcabs, abs_zero]; linarith
          · rw [ite_eq_right hrv, sub_zero]
            simp only [hd']
            calc |d r - c * B r e| ≤ |d r| + |c * B r e| := abs_sub _ _
              _ = |d r| + |c| * |B r e| := by rw [abs_mul]
        have hcol : ∑ r, |B r e| = 2 := by
          rw [Finset.sum_eq_add_of_mem v u (mem_univ _) (mem_univ _) huv.symm
            (fun r _ h => by rw [h0 r h.1 h.2, abs_zero]), hBv.abs, hBu.abs]
          rfl
        calc ∑ r, |d' r| ≤ ∑ r, (|d r| + |c| * |B r e| - (if r = v then 2 * |d v| else 0)) :=
              sum_le_sum fun r _ => hterm r
          _ = ∑ r, |d r| + |c| * ∑ r, |B r e| - 2 * |d v| := by
              rw [sum_sub_distrib, sum_add_distrib, ← mul_sum, sum_ite_eq', ite_eq_left (mem_univ _)]
          _ = ∑ r, |d r| := by rw [hcol, hcabs]; ring
      intro e'
      by_cases he' : e' = e
      · rw [he']
        simp only [Pi.add_apply, hy'T e he, zero_add]
        exact hc2
      · simp only [Pi.add_apply, ite_eq_right he', add_zero]
        exact (hy'bound e').trans hd'sum

/-! ### Unbalanced cycles -/

/-- Values on the edges of `T` balanced at every node are zero. -/
def Independent (B : Matrix V E ℤ) (T : Finset E) : Prop :=
  ∀ y : E → ℤ, (∀ e ∉ T, y e = 0) → B *ᵥ y = 0 → y = 0

/-- The edge `f` closes an unbalanced cycle with the tree `T`: no non-zero values on
`T ∪ {f}` are balanced at every node. -/
def ClosesUnbalanced (B : Matrix V E ℤ) (T : Finset E) (f : E) : Prop :=
  Independent B (insert f T)

/-- The graph is balanced: one choice of signs on the nodes balances every edge. -/
def IsBalancedGraph (B : Matrix V E ℤ) : Prop :=
  ∃ w : V → ℤ, (∀ v, IsSign (w v)) ∧ ∀ e, ∑ r, w r * B r e = 0

/-- `∑_r w r (B y)_r = ∑_e y_e ∑_r w r B r e`. -/
theorem sum_mul_mulVec (w : V → ℤ) (y : E → ℤ) :
    ∑ r, w r * (B *ᵥ y) r = ∑ e, y e * ∑ r, w r * B r e := by
  simp only [Matrix.mulVec, dotProduct, mul_sum]
  rw [sum_comm]
  exact sum_congr rfl fun _ _ => sum_congr rfl fun _ _ => by ring

/-- If a switching `w` of a spanning tree does not balance the edge `f`, then `f`
closes an unbalanced cycle. -/
theorem closesUnbalanced_of_pair {T : Finset E} (hT : IsTree B univ T) (w : V → ℤ)
    (hw : IsSwitching B univ T w) (f : E) (hf : ∑ r, w r * B r f ≠ 0) :
    ClosesUnbalanced B T f := by
  intro y hy hBy
  have hfT : f ∉ T := fun h => hf (hw.2 f h)
  have h1 : ∑ r, w r * (B *ᵥ y) r = y f * ∑ r, w r * B r f := by
    rw [sum_mul_mulVec]
    refine sum_eq_single f (fun e _ hef => ?_) (fun h => absurd (mem_univ f) h)
    by_cases heT : e ∈ T
    · rw [hw.2 e heT, mul_zero]
    · rw [hy e (by simp [hef, heT]), zero_mul]
  rw [hBy] at h1
  simp only [Pi.zero_apply, mul_zero, sum_const_zero] at h1
  have hyf : y f = 0 := (mul_eq_zero.mp h1.symm).resolve_right hf
  refine hT.eq_zero y (fun e he => ?_) hBy
  by_cases hef : e = f
  · rw [hef, hyf]
  · exact hy e (by simp [hef, he])

/-- Conversely, if `f` closes an unbalanced cycle, no switching of the tree balances it. -/
theorem pair_ne_zero {T : Finset E} (hT : IsTree B univ T) (w : V → ℤ)
    (hw : IsSwitching B univ T w) (f : E) (hfT : f ∉ T) (hf : ClosesUnbalanced B T f) :
    ∑ r, w r * B r f ≠ 0 := by
  intro h0
  obtain ⟨y, hyT, hyB, -⟩ := hT.solve w hw (fun r => -B r f) (fun r hr => absurd (mem_univ r) hr)
    (by simp only [mul_neg, sum_neg_distrib, h0, neg_zero])
  have := hf (y + fun e => if e = f then 1 else 0) (fun e he => ?_) ?_
  · have := congrFun this f
    simp [hyT f hfT] at this
  · have h1 : e ≠ f := fun h => he (h ▸ mem_insert_self _ _)
    have h2 : e ∉ T := fun h => he (mem_insert_of_mem h)
    simp [h1, hyT e h2]
  · funext r
    have := congrFun hyB r
    simp only [Matrix.mulVec, dotProduct, Pi.add_apply, mul_add, sum_add_distrib,
      mul_ite, mul_one, mul_zero, sum_ite_eq', mem_univ, ite_true] at this ⊢
    rw [this]; simp

/-- **A balanced fundamental cycle carries balanced values `±1`.** If `f` does not close
an unbalanced cycle with the spanning tree `T`, there are values in `{0, ±1}`
on `T ∪ {f}`, `1` on `f`, balanced at every node: the note's definition of a
balanced cycle. -/
theorem balanced_cycle_of_not_closesUnbalanced {T : Finset E} (hB : IsBidirected B)
    (hT : IsTree B univ T) (f : E) (hfT : f ∉ T) (hf : ¬ ClosesUnbalanced B T f) :
    ∃ y : E → ℤ, (∀ e ∉ insert f T, y e = 0) ∧ y f = 1 ∧ B *ᵥ y = 0 ∧ ∀ e, |y e| ≤ 1 := by
  obtain ⟨w, hw⟩ := hT.exists_switching
  have h0 : ∑ r, w r * B r f = 0 := by
    by_contra h
    exact hf (closesUnbalanced_of_pair hT w hw f h)
  obtain ⟨y, hyT, hyB, hbound⟩ := hT.solve w hw (fun r => -B r f)
    (fun r hr => absurd (mem_univ r) hr) (by simp only [mul_neg, sum_neg_distrib, h0, neg_zero])
  refine ⟨y + fun e => if e = f then 1 else 0, fun e he => ?_, ?_, ?_, fun e => ?_⟩
  · have h1 : e ≠ f := fun h => he (h ▸ mem_insert_self _ _)
    have h2 : e ∉ T := fun h => he (mem_insert_of_mem h)
    simp [h1, hyT e h2]
  · simp [hyT f hfT]
  · funext r
    have := congrFun hyB r
    simp only [Matrix.mulVec, dotProduct, Pi.add_apply, mul_add, sum_add_distrib,
      mul_ite, mul_one, mul_zero, sum_ite_eq', mem_univ, ite_true] at this ⊢
    rw [this]; simp
  · have hsum : ∑ r, |(fun r => -B r f) r| = 2 := by
      simp only [abs_neg]; exact hB.sum_abs f
    have := hbound e
    rw [hsum] at this
    by_cases hef : e = f
    · simp [hef, hyT f hfT]
    · simp only [Pi.add_apply, ite_eq_right hef, add_zero]; linarith

/-- **Existence of the edge** (the step the note's proof left implicit). If the graph
is not balanced, some edge closes an unbalanced cycle with any spanning tree. -/
theorem exists_closesUnbalanced {T : Finset E} (hT : IsTree B univ T)
    (hG : ¬ IsBalancedGraph B) : ∃ f ∉ T, ClosesUnbalanced B T f := by
  obtain ⟨w, hw⟩ := hT.exists_switching
  by_contra h
  push Not at h
  apply hG
  refine ⟨w, fun v => hw.1 v (mem_univ v), fun e => ?_⟩
  by_cases heT : e ∈ T
  · exact hw.2 e heT
  · by_contra hne
    exact h e heT (closesUnbalanced_of_pair hT w hw e hne)

/-! ### Proposition "Free arcs" -/

section FreeArcs

variable {T : Finset E} {f : E}

/-- **Solving on a spanning tree and an unbalanced cycle.** Every integer demand `d`
with `∑_r d_r` even is `B y` for exactly one integer `y` supported on `T ∪ {f}`,
and `|y e| ≤ ∑_r |d_r|`. The image of `B` restricted to `T ∪ {f}` is thus the
lattice of vectors of even sum, of index 2: the determinant is `±2`, not `±1`. -/
theorem solve_unicyclic (hB : IsBidirected B) (hT : IsTree B univ T) (hfT : f ∉ T)
    (hf : ClosesUnbalanced B T f) (d : V → ℤ) (hd : Even (∑ r, d r)) :
    ∃ y : E → ℤ, (∀ e ∉ insert f T, y e = 0) ∧ B *ᵥ y = d ∧ ∀ e, |y e| ≤ ∑ r, |d r| := by
  obtain ⟨w, hw⟩ := hT.exists_switching
  have hws : ∀ v, IsSign (w v) := fun v => hw.1 v (mem_univ v)
  have hs := pair_ne_zero hT w hw f hfT hf
  set s := ∑ r, w r * B r f with hsdef
  have hs2 : s = 2 ∨ s = -2 := by
    rcases hB.pair_eq w hws f with h | h | h
    · exact absurd h hs
    · exact Or.inl h
    · exact Or.inr h
  -- `∑ w d` is even, so `c = (∑ w d) / s` is whole.
  have hwd : Even (∑ r, w r * d r) := by
    have : ∑ r, w r * d r = ∑ r, d r + ∑ r, (w r * d r - d r) := by
      rw [← sum_add_distrib]; exact sum_congr rfl fun _ _ => by ring
    rw [this]
    exact hd.add (Finset.even_sum _ fun r _ => (hws r).even_mul_sub (d r))
  obtain ⟨m, hm⟩ := hwd
  set c := if s = 2 then m else -m with hc
  have hcs : c * s = ∑ r, w r * d r := by
    rcases hs2 with h | h <;> simp [hc, h, hm] <;> ring
  have hcabs : 2 * |c| ≤ ∑ r, |d r| := by
    have h1 : |∑ r, w r * d r| ≤ ∑ r, |d r| :=
      (abs_sum_le_sum_abs _ _).trans (le_of_eq (sum_congr rfl fun r _ => by
        rw [abs_mul, (hws r).abs, one_mul]))
    have h2 : |∑ r, w r * d r| = 2 * |c| := by
      rw [← hcs, abs_mul]; rcases hs2 with h | h <;> rw [h] <;> simp [mul_comm]
    linarith
  set d' : V → ℤ := fun r => d r - c * B r f with hd'
  have hwd' : ∑ r ∈ univ, w r * d' r = 0 := by
    have : ∑ r, w r * d' r = ∑ r, w r * d r - c * s := by
      rw [hsdef, mul_sum, ← sum_sub_distrib]
      exact sum_congr rfl fun r _ => by simp only [hd']; ring
    rw [this, hcs, sub_self]
  obtain ⟨y', hy'T, hy'B, hy'bound⟩ :=
    hT.solve w hw d' (fun r hr => absurd (mem_univ r) hr) hwd'
  have hd'sum : ∑ r, |d' r| ≤ ∑ r, |d r| + 2 * |c| := by
    calc ∑ r, |d' r| ≤ ∑ r, (|d r| + |c| * |B r f|) := sum_le_sum fun r _ => by
            simp only [hd']
            calc |d r - c * B r f| ≤ |d r| + |c * B r f| := abs_sub _ _
              _ = |d r| + |c| * |B r f| := by rw [abs_mul]
      _ = ∑ r, |d r| + 2 * |c| := by rw [sum_add_distrib, ← mul_sum, hB.sum_abs f]; ring
  refine ⟨y' + fun e => if e = f then c else 0, fun e he => ?_, ?_, fun e => ?_⟩
  · have h1 : e ≠ f := fun h => he (h ▸ mem_insert_self _ _)
    have h2 : e ∉ T := fun h => he (mem_insert_of_mem h)
    simp [h1, hy'T e h2]
  · funext r
    have := congrFun hy'B r
    simp only [Matrix.mulVec, dotProduct, Pi.add_apply, mul_add, sum_add_distrib,
      mul_ite, mul_zero, sum_ite_eq', mem_univ, ite_true] at this ⊢
    rw [this]; simp only [hd']; ring
  · by_cases hef : e = f
    · rw [hef]
      simp only [Pi.add_apply, ite_true, hy'T f hfT, zero_add]
      have := abs_nonneg c; linarith
    · simp only [Pi.add_apply, ite_eq_right hef, add_zero]
      have := hy'bound e
      have := abs_nonneg c
      linarith

/-- Values supported on `T ∪ {f}` with the same image under `B` are equal. -/
theorem unique_unicyclic (hf : ClosesUnbalanced B T f) (y y' : E → ℤ)
    (hy : ∀ e ∉ insert f T, y e = 0) (hy' : ∀ e ∉ insert f T, y' e = 0)
    (h : B *ᵥ y = B *ᵥ y') : y = y' := by
  have := hf (y - y') (fun e he => by simp [hy e he, hy' e he])
    (by rw [Matrix.mulVec_sub, h, sub_self])
  exact sub_eq_zero.mp this

/-- **Proposition "Free arcs", the extension.** Let `T` be a spanning tree and `f` an
edge closing an unbalanced cycle with it. Every integer assignment of the other
edges `F` extends to exactly one integer assignment of all the edges balanced
at every node. -/
theorem free_arcs (hB : IsBidirected B) (hT : IsTree B univ T) (hfT : f ∉ T)
    (hf : ClosesUnbalanced B T f) (x : E → ℤ) :
    ∃! z : E → ℤ, B *ᵥ z = 0 ∧ ∀ e ∉ insert f T, z e = x e := by
  -- `x₀`: `x` on the free edges, zero on `T ∪ {f}`.
  set x₀ : E → ℤ := fun e => if e ∈ insert f T then 0 else x e with hx₀
  have hd : Even (∑ r, (-(B *ᵥ x₀)) r) := by
    have := handshake B hB.column_even x₀
    simpa only [Pi.neg_apply, sum_neg_distrib, even_neg] using this
  obtain ⟨y, hyT, hyB, -⟩ := solve_unicyclic hB hT hfT hf _ hd
  refine ⟨x₀ + y, ⟨by rw [Matrix.mulVec_add, hyB, add_neg_cancel], fun e he => ?_⟩, ?_⟩
  · simp only [Pi.add_apply, hx₀, ite_eq_right he, hyT e he, add_zero]
  · rintro z ⟨hz, hzx⟩
    have h1 : z - x₀ = y := by
      refine unique_unicyclic hf _ _ (fun e he => ?_) hyT ?_
      · simp only [Pi.sub_apply, hx₀, ite_eq_right he, hzx e he, sub_self]
      · rw [Matrix.mulVec_sub, hz, hyB, zero_sub]
    rw [← h1, add_sub_cancel]

/-- **Proposition "Free arcs", the basis.** The vectors `z_g`, `g` a free edge, are
balanced, supported on `T ∪ {g}`, with `z_g(g) = 1` and entries in `{0, ±1, ±2}`;
every integer balanced assignment `x` is `∑_{g free} x_g z_g`. The integer
balanced assignments thus form a lattice isomorphic to `ℤ^F`. -/
theorem free_arcs_basis (hB : IsBidirected B) (hT : IsTree B univ T) (hfT : f ∉ T)
    (hf : ClosesUnbalanced B T f) :
    ∃ z : E → E → ℤ,
      (∀ g ∉ insert f T, B *ᵥ z g = 0 ∧ z g g = 1 ∧
        (∀ e, e ∉ insert f T → e ≠ g → z g e = 0) ∧ ∀ e, |z g e| ≤ 2) ∧
      ∀ x : E → ℤ, B *ᵥ x = 0 → x = ∑ g ∈ (insert f T)ᶜ, x g • z g := by
  have hex : ∀ g, ∃ y : E → ℤ, (∀ e ∉ insert f T, y e = 0) ∧
      B *ᵥ y = (fun r => -B r g) ∧ ∀ e, |y e| ≤ 2 := by
    intro g
    have hsum : ∑ r, |(fun r => -B r g) r| = 2 := by
      simp only [abs_neg]; exact hB.sum_abs g
    have heven : Even (∑ r, (fun r => -B r g) r) := by
      simpa only [sum_neg_distrib, even_neg] using hB.column_even g
    obtain ⟨y, h1, h2, h3⟩ := solve_unicyclic hB hT hfT hf _ heven
    exact ⟨y, h1, h2, fun e => (h3 e).trans hsum.le⟩
  choose y hyT hyB hybound using hex
  set z : E → E → ℤ := fun g => y g + fun e => if e = g then 1 else 0 with hz
  have hzB : ∀ g, B *ᵥ z g = 0 := by
    intro g; funext r
    have := congrFun (hyB g) r
    simp only [hz, Matrix.mulVec, dotProduct, Pi.add_apply, mul_add, sum_add_distrib,
      mul_ite, mul_one, mul_zero, sum_ite_eq', mem_univ, ite_true] at this ⊢
    rw [this]; simp
  refine ⟨z, fun g hg => ⟨hzB g, ?_, fun e he heg => ?_, fun e => ?_⟩, fun x hx => ?_⟩
  · simp [hz, hyT g g hg]
  · simp [hz, heg, hyT g e he]
  · by_cases heg : e = g
    · subst heg; simp [hz, hyT e e hg]
    · simp only [hz, Pi.add_apply, ite_eq_right heg, add_zero]; exact hybound g e
  · -- `x - ∑ x_g z_g` is balanced and supported on `T ∪ {f}`, hence zero.
    set s := ∑ g ∈ (insert f T)ᶜ, x g • z g with hs
    have hsB : B *ᵥ s = 0 := by
      rw [hs, Matrix.mulVec_sum]
      exact sum_eq_zero fun g _ => by rw [Matrix.mulVec_smul, hzB g, smul_zero]
    have hsx : ∀ e ∉ insert f T, s e = x e := by
      intro e he
      rw [hs, Finset.sum_apply, sum_eq_single e]
      · simp [hz, hyT e e he]
      · intro g hg hge
        have : z g e = 0 := by
          simp [hz, Ne.symm hge, hyT g e he]
        simp [this]
      · intro h; exact absurd (mem_compl.mpr he) h
    have := hf (x - s) (fun e he => by simp [hsx e he]) (by rw [Matrix.mulVec_sub, hx, hsB,
      sub_self])
    exact (sub_eq_zero.mp this)

/-- **Proposition "Free arcs", full row rank.** The rows of `B` are linearly
independent over `ℚ`. -/
theorem rows_independent (hB : IsBidirected B) (hT : IsTree B univ T) (hfT : f ∉ T)
    (hf : ClosesUnbalanced B T f) :
    LinearIndependent ℚ (fun v : V => fun e : E => (B v e : ℚ)) := by
  rw [Fintype.linearIndependent_iff]
  intro g hg v
  -- `2 e_v` has an even sum, so it is `B y` for a whole `y`.
  obtain ⟨y, -, hyB, -⟩ := solve_unicyclic hB hT hfT hf (fun r => if r = v then 2 else 0)
    (by simp)
  have h1 : ∑ r, g r * ((B *ᵥ y) r : ℚ) = 0 := by
    have : ∀ r, g r * ((B *ᵥ y) r : ℚ) = ∑ e, g r * (B r e : ℚ) * (y e : ℚ) := by
      intro r
      simp only [Matrix.mulVec, dotProduct, Int.cast_sum, Int.cast_mul, mul_sum]
      exact sum_congr rfl fun _ _ => by ring
    simp only [this]
    rw [sum_comm]
    refine sum_eq_zero fun e _ => ?_
    have := congrFun hg e
    simp only [Finset.sum_apply, Pi.smul_apply, smul_eq_mul, Pi.zero_apply] at this
    rw [← sum_mul, this, zero_mul]
  rw [hyB] at h1
  simp only [Int.cast_ite, Int.cast_ofNat, Int.cast_zero, mul_ite, mul_zero,
    sum_ite_eq', mem_univ, ite_true] at h1
  linarith

end FreeArcs

/-! ### Cycles -/

/-- A cycle of the graph: nodes `node i` and edges `edge i`, `i : Fin k`, all distinct,
the edge `edge i` joining `node i` to `node (i + 1)` (indices modulo `k`). -/
structure Cycle (B : Matrix V E ℤ) (k : ℕ) where
  node : Fin k → V
  edge : Fin k → E
  node_injective : Function.Injective node
  edge_injective : Function.Injective edge
  two_le : 2 ≤ k
  start : ∀ i, IsSign (B (node i) (edge i))
  finish : ∀ i, IsSign (B (node (finRotate k i)) (edge i))
  zero : ∀ i r, r ≠ node i → r ≠ node (finRotate k i) → B r (edge i) = 0

/-- The note's definition: a cycle is balanced if values `±1` on its edges have a
zero balance at each node. -/
def Cycle.IsBalanced {k : ℕ} (C : Cycle B k) : Prop :=
  ∃ y : E → ℤ, (∀ i, IsSign (y (C.edge i))) ∧ (∀ e, e ∉ Set.range C.edge → y e = 0) ∧
    B *ᵥ y = 0

/-- In a cycle, `i + 1 ≠ i`. -/
theorem finRotate_ne {k : ℕ} (hk : 2 ≤ k) (i : Fin k) : finRotate k i ≠ i := by
  obtain ⟨n, rfl⟩ : ∃ n, k = Nat.succ n := ⟨k - 1, by omega⟩
  intro h
  have := congrArg Fin.val h
  rw [coe_finRotate] at this
  split_ifs at this with hi
  · rw [hi, Fin.val_last] at this; omega
  · omega

/-- **Every cycle of a balanced graph is balanced** (one direction of Harary's
theorem). So a graph with an unbalanced cycle is not balanced, and
`exists_closesUnbalanced` gives the edge of Proposition "Free arcs". -/
theorem Cycle.isBalanced_of_graph {k : ℕ} (C : Cycle B k) (hG : IsBalancedGraph B) :
    C.IsBalanced := by
  obtain ⟨w, hws, hwe⟩ := hG
  have hk : 2 ≤ k := C.two_le
  set ρ := finRotate k
  have hne : ∀ i, C.node i ≠ C.node (ρ i) := fun i h =>
    finRotate_ne hk i (C.node_injective h).symm
  -- The switching balances `edge i` at its two ends.
  have hpair : ∀ i, w (C.node i) * B (C.node i) (C.edge i) +
      w (C.node (ρ i)) * B (C.node (ρ i)) (C.edge i) = 0 := by
    intro i
    have := hwe (C.edge i)
    rwa [Finset.sum_eq_add_of_mem (C.node i) (C.node (ρ i)) (mem_univ _) (mem_univ _) (hne i)
      (fun r _ h => by rw [C.zero i r h.1 h.2, mul_zero])] at this
  -- The value on `edge i` is `w (node i) * B (node i) (edge i)`.
  set y : E → ℤ := fun e => if h : e ∈ Set.range C.edge then
    w (C.node h.choose) * B (C.node h.choose) e else 0 with hy
  have hyedge : ∀ i, y (C.edge i) = w (C.node i) * B (C.node i) (C.edge i) := by
    intro i
    have h : C.edge i ∈ Set.range C.edge := ⟨i, rfl⟩
    have hc : h.choose = i := C.edge_injective h.choose_spec
    simp only [hy, dite_eq_left h, hc]
  have hyoff : ∀ e, e ∉ Set.range C.edge → y e = 0 := fun e he => by simp only [hy, dite_eq_right he]
  refine ⟨y, fun i => by rw [hyedge]; exact (hws _).mul (C.start i), hyoff, ?_⟩
  funext r
  simp only [Matrix.mulVec, dotProduct, Pi.zero_apply]
  -- Reindex the sum by the edges of the cycle.
  have hre : ∑ e, B r e * y e = ∑ i, B r (C.edge i) * y (C.edge i) := by
    rw [← Finset.sum_image (f := fun e => B r e * y e) (fun i _ j _ h => C.edge_injective h)]
    refine (sum_subset (subset_univ _) fun e _ he => ?_).symm
    rw [hyoff e (fun ⟨i, hi⟩ => he (mem_image.mpr ⟨i, mem_univ _, hi⟩)), mul_zero]
  rw [hre]
  simp only [hyedge]
  by_cases hr : ∃ j, r = C.node j
  · obtain ⟨j, rfl⟩ := hr
    set p := ρ.symm j
    have hpj : ρ p = j := ρ.apply_symm_apply j
    have hpne : p ≠ j := fun h => finRotate_ne hk p (hpj.trans h.symm)
    rw [Finset.sum_eq_add_of_mem j p (mem_univ _) (mem_univ _) hpne.symm (fun i _ h => by
      rw [C.zero i (C.node j) (fun h' => h.1 (C.node_injective h').symm)
        (fun h' => h.2 (by rw [← ρ.symm_apply_apply i, ← C.node_injective h'])), zero_mul])]
    have h1 := (C.start j).mul_self
    have h2 := (C.finish p).mul_self
    have h3 := hpair p
    rw [hpj] at h2 h3
    linear_combination (w (C.node j)) * h1 + B (C.node j) (C.edge p) * h3 -
      w (C.node j) * h2
  · push Not at hr
    exact sum_eq_zero fun i _ => by
      rw [C.zero i r (hr i) (hr (ρ i)), zero_mul]

/-- A graph with an unbalanced cycle is not balanced. -/
theorem not_isBalancedGraph_of_cycle {k : ℕ} (C : Cycle B k) (hC : ¬ C.IsBalanced) :
    ¬ IsBalancedGraph B := fun hG => hC (C.isBalanced_of_graph hG)

/-- **Proposition "Free arcs", as stated.** Let the graph have a spanning tree `T`
(it is connected) and an unbalanced cycle. Then some edge `f` closes an
unbalanced cycle with `T`, and every integer assignment of the other edges extends
to exactly one integer balanced assignment. -/
theorem free_arcs_of_cycle {T : Finset E} {k : ℕ} (hB : IsBidirected B)
    (hT : IsTree B univ T) (C : Cycle B k) (hC : ¬ C.IsBalanced) :
    ∃ f ∉ T, ClosesUnbalanced B T f ∧ ∀ x : E → ℤ,
      ∃! z : E → ℤ, B *ᵥ z = 0 ∧ ∀ e ∉ insert f T, z e = x e := by
  obtain ⟨f, hfT, hf⟩ := exists_closesUnbalanced hT (not_isBalancedGraph_of_cycle C hC)
  exact ⟨f, hfT, hf, free_arcs hB hT hfT hf⟩

end Bidirected

end ClosingHelices
