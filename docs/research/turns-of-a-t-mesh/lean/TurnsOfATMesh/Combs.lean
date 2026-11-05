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

theorem IsWhole.mul {z z' : ℂ} (h : IsWhole z) (h' : IsWhole z') : IsWhole (z * z') := by
  obtain ⟨m, n, rfl⟩ := h
  obtain ⟨m', n', rfl⟩ := h'
  exact ⟨m * m' - n * n', m * n' + n * m', by
    push_cast; linear_combination ((n : ℂ) * n') * Complex.I_sq⟩

theorem IsWhole.sub {z z' : ℂ} (h : IsWhole z) (h' : IsWhole z') : IsWhole (z - z') := by
  obtain ⟨m, n, rfl⟩ := h
  obtain ⟨m', n', rfl⟩ := h'
  exact ⟨m - m', n - n', by push_cast; ring⟩

theorem isWhole_unit {z : ℂ} (h : z = 1 ∨ z = Complex.I ∨ z = -1 ∨ z = -Complex.I) :
    IsWhole z := by
  rcases h with rfl | rfl | rfl | rfl
  · exact ⟨1, 0, by simp⟩
  · exact ⟨0, 1, by simp⟩
  · exact ⟨-1, 0, by simp⟩
  · exact ⟨0, -1, by simp⟩

/-- For an odd valence, `(1 - R)ᵀ (1 - R) = 2`. -/
theorem conj_mul_one_sub {R : ℂ} (h : R = Complex.I ∨ R = -Complex.I) :
    conj (1 - R) * (1 - R) = 2 := by
  rcases h with rfl | rfl <;>
    simp only [map_sub, map_one, map_neg, Complex.conj_I, neg_neg] <;>
    linear_combination (-1 : ℂ) * Complex.I_sq

section Forms

variable {ι : Type*} [Fintype ι] [DecidableEq ι]

/-- The form of the twisted class `w ⊗ v` of a word `w = λ_{c₀} λ_{c₁}^{±1}`
whose rotation is trivial is `⟨v, (1 - R₀)(q₀ - q₁)⟩`. -/
theorem form_pair (u : ι → ℂˣ) (q : ι → ℂ) (c₀ c₁ : ι) (s : Bool)
    (hrot : (u c₀ : ℂ) * letterLin u (c₁, s) = 1) (v : ℂ) :
    form u (twistedWinding {()} u (fun _ => 1) (fun _ => v) (fun _ => [(c₀, true), (c₁, s)])) q =
      dotC v ((1 - (u c₀ : ℂ)) * (q c₀ - q c₁)) := by
  have h := fox_burgers_combination {()} u q (fun _ => 1) (fun _ => v)
    (fun _ => [(c₀, true), (c₁, s)])
  rw [Finset.sum_singleton, one_mul] at h
  rw [form, ← h]
  congr 1
  have h₀ : letterLin u (c₀, true) = (u c₀ : ℂ) := rfl
  simp only [hol_cons, hol_nil, comp_trans, rotAbout_trans, rotAbout_lin, Motion.id_trans,
    smul_eq_mul, h₀]
  linear_combination (-(q c₁)) * hrot

/-- The common part of a dislocation and an exchange. -/
theorem comb_pair (u : ι → ℂˣ) (q : ι → ℂ) (c₀ c₁ : ι)
    (s : Bool) (hrot : (u c₀ : ℂ) * letterLin u (c₁, s) = 1)
    (hR₀ : (u c₀ : ℂ) = Complex.I ∨ (u c₀ : ℂ) = -Complex.I) (e : ℂ)
    (he : e = 1 ∨ e = Complex.I ∨ e = -1 ∨ e = -Complex.I) (len : ℝ)
    (hlen : dotC e (q c₁ - q c₀) = len) :
    let W := twistedWinding {()} u (fun _ => 1) (fun _ => e * (1 - (u c₀ : ℂ)))
      (fun _ => [(c₀, true), (c₁, s)])
    rhoW u [(c₀, true), (c₁, s)] = 1 ∧ W ∈ cycles u ∧ IsWhole (e * (1 - (u c₀ : ℂ))) ∧
      conj (1 - (u c₀ : ℂ)) * (e * (1 - (u c₀ : ℂ))) = 2 * e ∧ form u W q = -2 * len := by
  intro W
  have hrho : rhoW u [(c₀, true), (c₁, s)] = 1 := by
    simpa [rhoW, letterLin] using hrot
  have hn := conj_mul_one_sub hR₀
  refine ⟨hrho, ?_, ?_, ?_, ?_⟩
  · exact twisted_relation {()} u q _ _ _ fun _ _ => by rw [hol_lin, hrho]
  · refine (isWhole_unit he).mul (IsWhole.sub ⟨1, 0, by simp⟩ ?_)
    rcases hR₀ with h | h <;> rw [h]
    · exact ⟨0, 1, by simp⟩
    · exact ⟨0, -1, by simp⟩
  · linear_combination e * hn
  · rw [form_pair u q c₀ c₁ s hrot, ← hlen, dotC, dotC]
    have key : conj (e * (1 - (u c₀ : ℂ))) * ((1 - (u c₀ : ℂ)) * (q c₀ - q c₁)) =
        ((-2 : ℝ) : ℂ) * (conj e * (q c₁ - q c₀)) := by
      rw [map_mul]; push_cast; linear_combination (conj e * (q c₀ - q c₁)) * hn
    rw [key, Complex.re_ofReal_mul]

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
      conj (1 - (u c₀ : ℂ)) * (e * (1 - (u c₀ : ℂ))) = 2 * e ∧ form u W q = -2 * len :=
  comb_pair u q c₀ c₁ true (by
    simp only [letterLin, hR₁]
    exact mul_inv_cancel₀ (Units.ne_zero _)) hR₀ e he len hlen

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
      conj (1 - (u c₀ : ℂ)) * (e * (1 - (u c₀ : ℂ))) = 2 * e ∧ form u W q = -2 * len :=
  comb_pair u q c₀ c₁ false (by
    simp only [letterLin, Units.val_inv_eq_inv_val, hR₁]
    exact mul_inv_cancel₀ (Units.ne_zero _)) hR₀ e he len hlen

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

/-- The ends of an arc at `v`, modulo 2: a loop at `v` counts twice. -/
def ends (a : α) (v : ι) : ZMod 2 :=
  (if foot₀ a = v then 1 else 0) + (if foot₁ a = v then 1 else 0)

/-- The degree modulo 2 of `v` in a set of arcs. -/
def deg (S : Finset α) (v : ι) : ZMod 2 := ∑ a ∈ S, ends foot₀ foot₁ a v

theorem zmod_two_add_self (t : ZMod 2) : t + t = 0 := by fin_cases t <;> decide

/-- The ends of the arc of a step are where it leaves and where it arrives. -/
theorem ends_step (s : α × Bool) (v : ι) :
    ends foot₀ foot₁ s.1 v =
      (if src foot₀ foot₁ s = v then 1 else 0) + (if tgt foot₀ foot₁ s = v then 1 else 0) := by
  obtain ⟨a, b⟩ := s
  cases b <;> simp [ends, src, tgt, add_comm]

theorem deg_erase {S : Finset α} {a : α} (ha : a ∈ S) (v : ι) :
    deg foot₀ foot₁ S v = ends foot₀ foot₁ a v + deg foot₀ foot₁ (S.erase a) v :=
  (Finset.add_sum_erase S _ ha).symm

/-- A walk with distinct arcs from `y` to `x` has odd degree at `x` and `y` only
(none if `x = y`). -/
theorem deg_walk {x y : ι} {w : List (α × Bool)} (hw : IsWalk foot₀ foot₁ y w x)
    (hnd : (w.map Prod.fst).Nodup) (v : ι) :
    deg foot₀ foot₁ (w.map Prod.fst).toFinset v =
      (if y = v then 1 else 0) + (if x = v then 1 else 0) := by
  induction hw with
  | nil x => simp [deg, zmod_two_add_self]
  | cons s w hsrc hw ih =>
    rw [List.map_cons, List.nodup_cons] at hnd
    rw [List.map_cons, List.toFinset_cons, deg,
      Finset.sum_insert (fun h => hnd.1 (List.mem_toFinset.mp h)), ← deg, ih hnd.2, ends_step,
      hsrc]
    linear_combination zmod_two_add_self (if tgt foot₀ foot₁ s = v then (1 : ZMod 2) else 0)

/-- A set of arcs whose only vertices of odd degree are `x ≠ y` holds a walk from
`y` to `x` with distinct arcs. -/
theorem trail (S : Finset α) : ∀ x y : ι, x ≠ y →
    (∀ v, deg foot₀ foot₁ S v = (if x = v then 1 else 0) + (if y = v then 1 else 0)) →
    ∃ w : List (α × Bool), IsWalk foot₀ foot₁ y w x ∧ (w.map Prod.fst).Nodup ∧
      ∀ s ∈ w, s.1 ∈ S := by
  induction S using Finset.strongInduction with
  | H S ih =>
  intro x y hxy hdeg
  have hy : deg foot₀ foot₁ S y = 1 := by rw [hdeg]; simp [hxy]
  obtain ⟨a, ha, hay⟩ : ∃ a ∈ S, foot₀ a = y ∨ foot₁ a = y := by
    by_contra hno
    push Not at hno
    have h0 : deg foot₀ foot₁ S y = 0 :=
      Finset.sum_eq_zero fun a h => by simp [ends, (hno a h).1, (hno a h).2]
    rw [h0] at hy
    exact zero_ne_one hy
  let s : α × Bool := (a, decide (foot₀ a = y))
  have hsrc : src foot₀ foot₁ s = y := by
    by_cases h : foot₀ a = y
    · simp [s, src, h]
    · simp [s, src, h]; exact hay.resolve_left h
  have hdeg' : ∀ v, deg foot₀ foot₁ (S.erase a) v =
      (if x = v then 1 else 0) + (if tgt foot₀ foot₁ s = v then 1 else 0) := by
    intro v
    have h1 := deg_erase foot₀ foot₁ ha v
    have h2 : ends foot₀ foot₁ a v =
        (if y = v then 1 else 0) + (if tgt foot₀ foot₁ s = v then 1 else 0) := by
      rw [← hsrc]; exact ends_step foot₀ foot₁ s v
    rw [hdeg] at h1
    linear_combination -h1 - h2 -
      zmod_two_add_self (if tgt foot₀ foot₁ s = v then (1 : ZMod 2) else 0)
  by_cases hzx : tgt foot₀ foot₁ s = x
  · refine ⟨[s], IsWalk.cons s [] hsrc (by rw [hzx]; exact IsWalk.nil x), by simp, ?_⟩
    intro t ht
    rw [List.mem_singleton] at ht
    rw [ht]; exact ha
  · obtain ⟨w, hw, hnd, hmem⟩ :=
      ih (S.erase a) (Finset.erase_ssubset ha) x _ (Ne.symm hzx) hdeg'
    refine ⟨s :: w, IsWalk.cons s w hsrc hw, ?_, ?_⟩
    · rw [List.map_cons, List.nodup_cons]
      refine ⟨fun h => ?_, hnd⟩
      obtain ⟨t, ht, hta⟩ := List.mem_map.mp h
      have := hmem t ht
      rw [hta] at this
      exact Finset.notMem_erase a S this
    · intro t ht
      rcases List.mem_cons.mp ht with rfl | ht
      · exact ha
      · exact Finset.mem_of_mem_erase (hmem t ht)

/-- **Veblen.** A set of arcs with even degree at every vertex (a cycle modulo 2
of the graph of feet) is the union of closed walks with distinct arcs: every arc
of `S` is walked exactly once, and no other. -/
theorem veblen (S : Finset α) (hS : ∀ v, deg foot₀ foot₁ S v = 0) :
    ∃ walks : List (List (α × Bool)),
      (∀ w ∈ walks, w ≠ [] ∧ ∃ x, IsWalk foot₀ foot₁ x w x) ∧
      ∀ a, (walks.flatten.map Prod.fst).count a = if a ∈ S then 1 else 0 := by
  induction S using Finset.strongInduction with
  | H S ih =>
  rcases S.eq_empty_or_nonempty with rfl | ⟨a, ha⟩
  · exact ⟨[], by simp, by simp⟩
  have hdeg₁ : ∀ v, deg foot₀ foot₁ (S.erase a) v = ends foot₀ foot₁ a v := by
    intro v
    have h1 := deg_erase foot₀ foot₁ ha v
    rw [hS v] at h1
    linear_combination -h1 - zmod_two_add_self (ends foot₀ foot₁ a v)
  by_cases hloop : foot₀ a = foot₁ a
  · -- A loop is a closed walk of length one.
    obtain ⟨walks, hw, hc⟩ := ih (S.erase a) (Finset.erase_ssubset ha) fun v => by
      rw [hdeg₁, ends, hloop]; exact zmod_two_add_self _
    refine ⟨[(a, true)] :: walks, ?_, ?_⟩
    · intro w hw'
      rcases List.mem_cons.mp hw' with rfl | hw'
      · refine ⟨by simp, foot₀ a, IsWalk.cons _ _ (by simp [src]) ?_⟩
        simp only [tgt, ite_true]
        rw [← hloop]
        exact IsWalk.nil _
      · exact hw w hw'
    · intro b
      rw [List.flatten_cons, List.map_append, List.count_append, hc b]
      by_cases hb : b = a
      · subst hb; simp [ha]
      · simp [hb, Ne.symm hb, Finset.mem_erase]
  · -- Otherwise the arc closes a walk back from its second foot to its first.
    obtain ⟨t, ht, hnd, hmem⟩ := trail foot₀ foot₁ (S.erase a) (foot₀ a) (foot₁ a) hloop
      fun v => hdeg₁ v
    set T := (t.map Prod.fst).toFinset with hTdef
    have hT : T ⊆ S.erase a := by
      intro b hb
      obtain ⟨s, hs, rfl⟩ := List.mem_map.mp (List.mem_toFinset.mp hb)
      exact hmem s hs
    have hss : S.erase a \ T ⊂ S := Finset.sdiff_subset.trans_ssubset (Finset.erase_ssubset ha)
    have hdeg'' : ∀ v, deg foot₀ foot₁ (S.erase a \ T) v = 0 := by
      intro v
      have h := Finset.sum_sdiff (f := fun b => ends foot₀ foot₁ b v) hT
      have hw := deg_walk foot₀ foot₁ ht hnd v
      rw [← hTdef] at hw
      have h' : deg foot₀ foot₁ (S.erase a \ T) v + deg foot₀ foot₁ T v =
          deg foot₀ foot₁ (S.erase a) v := h
      rw [hdeg₁ v, ends] at h'
      linear_combination h' - hw
    obtain ⟨walks, hw, hc⟩ := ih _ hss hdeg''
    refine ⟨((a, true) :: t) :: walks, ?_, ?_⟩
    · intro w hw'
      rcases List.mem_cons.mp hw' with rfl | hw'
      · exact ⟨List.cons_ne_nil _ _, foot₀ a, IsWalk.cons _ _ (by simp [src]) (by simpa [tgt] using ht)⟩
      · exact hw w hw'
    · intro b
      have hcount : (t.map Prod.fst).count b = if b ∈ T then 1 else 0 := by
        split_ifs with h
        · exact List.count_eq_one_of_mem hnd (List.mem_toFinset.mp h)
        · exact List.count_eq_zero_of_not_mem fun h' => h (List.mem_toFinset.mpr h')
      rw [List.flatten_cons, List.map_append, List.count_append, hc b, List.map_cons,
        List.count_cons, hcount]
      by_cases hba : b = a
      · subst hba
        have hbT : b ∉ T := fun h => Finset.notMem_erase b S (hT h)
        simp [hbT, ha]
      · by_cases hbT : b ∈ T
        · have hbS := Finset.mem_of_mem_erase (hT hbT)
          simp [hbT, hbS, Ne.symm hba]
        · simp [hbT, hba, Ne.symm hba, Finset.mem_erase]

/-- A sum over a list is the sum of the multiplicities. -/
theorem sum_map_count [Fintype α] (ℓ : α → ℝ) (l : List α) :
    (l.map ℓ).sum = ∑ a, (l.count a : ℝ) * ℓ a := by
  induction l with
  | nil => simp
  | cons b l ih =>
    simp only [List.map_cons, List.sum_cons, ih, List.count_cons, beq_iff_eq]
    push_cast
    simp only [add_mul, Finset.sum_add_distrib, ite_mul, one_mul, zero_mul, Finset.sum_ite_eq,
      Finset.mem_univ, ite_true]
    ring

/-- The readings of walks add up to the sum over all their steps. -/
theorem sum_reading (ℓ : α → ℝ) (walks : List (List (α × Bool))) :
    (walks.map (reading ℓ)).sum = ((walks.flatten.map Prod.fst).map ℓ).sum := by
  induction walks with
  | nil => simp
  | cons w ws ih => simp [reading, ih, List.map_append, List.sum_append, Function.comp_def]

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
  obtain ⟨walks, hw, hc⟩ := veblen foot₀ foot₁ _ hcycle
  have hodd : ∀ w ∈ walks, ∀ s ∈ w, Odd (c s.1) := by
    intro w hw' s hs
    have hpos : 0 < (walks.flatten.map Prod.fst).count s.1 :=
      List.count_pos_iff.mpr (List.mem_map.mpr ⟨s, List.mem_flatten.mpr ⟨w, hw', hs⟩, rfl⟩)
    rw [hc] at hpos
    split_ifs at hpos with h
    · simpa using h
    · omega
  refine ⟨walks, fun w hw' => ⟨(hw w hw').1, (hw w hw').2, fun s hs => ⟨hodd w hw' s hs, ?_⟩⟩,
    fun ℓ => ?_⟩
  · have hne : c s.1 ≠ 0 := fun h => by
      have := hodd w hw' s hs
      rw [h] at this
      exact Nat.not_odd_zero this
    obtain ⟨h0, h1⟩ := hfeet s.1 hne
    unfold src
    split_ifs
    · exact h0
    · exact h1
  · rw [sum_reading, sum_map_count, ← Finset.sum_add_distrib]
    refine Finset.sum_congr rfl fun a _ => ?_
    have hn : c a = (if a ∈ Finset.univ.filter fun a => Odd (c a) then 1 else 0) + 2 * (c a / 2) := by
      simp only [Finset.mem_filter, Finset.mem_univ, true_and]
      split_ifs with h
      · rw [Nat.odd_iff] at h; omega
      · rw [Nat.not_odd_iff_even, Nat.even_iff] at h; omega
    rw [hc a]
    conv_lhs => rw [hn]
    push_cast
    ring

/-- **Proposition "Certificates are sums of combs", real weights.** Every
non-negative form is the non-negative combination of the comb readings `2ℓₐ`
of the arcs with two distinct feet, with weights `cₐ/2`, and of the lengths
`ℓₐ` (winding separatrices) of the arcs whose two feet are one vertex. -/
theorem sum_of_combs_real [Fintype α] (c : α → ℝ) (ℓ : α → ℝ) :
    ∑ a, c a * ℓ a =
      ∑ a ∈ Finset.univ.filter (fun a => foot₀ a ≠ foot₁ a), c a / 2 * (2 * ℓ a) +
        ∑ a ∈ Finset.univ.filter (fun a => foot₀ a = foot₁ a), c a * ℓ a := by
  rw [← Finset.sum_filter_add_sum_filter_not Finset.univ (fun a => foot₀ a ≠ foot₁ a)]
  congr 1
  · exact Finset.sum_congr rfl fun a _ => by ring
  · simp only [not_not]

end Walks

end Combs

end TurnsOfATMesh
