/-
Parity at whole lengths: the checkerboard rules and the parity of a turn.

For whole lengths the developed positions are points of `ℤ²`, here the Gaussian
integers `ℤ[i]`, with the quarter-turn `R` the multiplication by `i`. The
colour of a point `p` is the parity of `p₁ + p₂`, an element of `ZMod 2`.
-/
module

public import ClosingHelices.Congruence
public import Mathlib.NumberTheory.Zsqrtd.GaussianInt
public import Mathlib.Data.ZMod.Basic
public import Mathlib.Algebra.CharP.Two

@[expose] public section

set_option linter.unusedSectionVars false

namespace ClosingHelices

open Motion

/-- `i`, the quarter-turn `R(a, b) = (-b, a)` of `ℤ²`. -/
def gi : GaussianInt := Zsqrtd.sqrtd

@[simp] theorem gi_re : gi.re = 0 := rfl
@[simp] theorem gi_im : gi.im = 1 := rfl

theorem gi_mul_gi : gi * gi = -1 := by ext <;> simp [gi]

/-- The quarter-turn as a unit. -/
def giUnit : GaussianIntˣ := ⟨gi, -gi, by rw [mul_neg, gi_mul_gi, neg_neg],
  by rw [neg_mul, gi_mul_gi, neg_neg]⟩

@[simp] theorem giUnit_val : (giUnit : GaussianInt) = gi := rfl

theorem giUnit_sq : giUnit ^ 2 = -1 := by
  ext : 1
  simp [sq, gi_mul_gi]


/-- The colour of a point of `ℤ²`: the parity of `p₁ + p₂`. -/
def colour (p : GaussianInt) : ZMod 2 := ((p.re + p.im : ℤ) : ZMod 2)

theorem zmod_two_neg (a : ZMod 2) : -a = a := by fin_cases a <;> rfl

theorem zmod_two_two : (2 : ZMod 2) = 0 := rfl

theorem colour_add (p q : GaussianInt) : colour (p + q) = colour p + colour q := by
  simp only [colour, Zsqrtd.re_add, Zsqrtd.im_add]; push_cast; ring

theorem colour_neg (p : GaussianInt) : colour (-p) = colour p := by
  simp only [colour, Zsqrtd.re_neg, Zsqrtd.im_neg]; push_cast
  rw [← neg_add, zmod_two_neg]

theorem colour_sub (p q : GaussianInt) : colour (p - q) = colour p + colour q := by
  rw [sub_eq_add_neg, colour_add, colour_neg]

/-! ### Single vertices -/

/-- `(1 - R) (a, b) = (a + b, b - a)`. -/
theorem one_sub_gi_mul (p : GaussianInt) :
    ((1 - gi) * p).re = p.re + p.im ∧ ((1 - gi) * p).im = p.im - p.re := by
  constructor <;> (simp [gi]; try ring)

/-- Both components of `(1 - R) p` and of `(1 - R⁻¹) p` have the colour of `p`:
the parity of `p₁ + p₂` (proof of Proposition "Checkerboard rule"). -/
theorem parity_one_sub_quarter (p : GaussianInt) (r : GaussianInt) (hr : r = gi ∨ r = -gi) :
    (((1 - r) * p).re : ZMod 2) = colour p ∧ (((1 - r) * p).im : ZMod 2) = colour p := by
  rcases hr with rfl | rfl <;> constructor <;> simp [colour, gi] <;> simp [add_comm]

/-- `(1 - R²) p = 2 p` is even. -/
theorem one_sub_half_turn (p : GaussianInt) :
    (1 - gi * gi) * p = 2 * p ∧ (((1 - gi * gi) * p).re : ZMod 2) = 0 ∧
      (((1 - gi * gi) * p).im : ZMod 2) = 0 := by
  rw [gi_mul_gi]
  refine ⟨by ring, ?_, ?_⟩ <;> simp <;> ring_nf <;> simp [zmod_two_two]

/-! ### Quarter-turns keep colours -/

/-- `a` is one of `1, R, R², R³`. -/
def IsQuarter (a : GaussianInt) : Prop := a = 1 ∨ a = gi ∨ a = -1 ∨ a = -gi

theorem IsQuarter.mul {a b : GaussianInt} (ha : IsQuarter a) (hb : IsQuarter b) :
    IsQuarter (a * b) := by
  unfold IsQuarter at *
  rcases ha with rfl | rfl | rfl | rfl <;> rcases hb with rfl | rfl | rfl | rfl <;>
    simp [gi_mul_gi]

theorem IsQuarter.one : IsQuarter 1 := Or.inl rfl

theorem IsQuarter.list_prod (l : List GaussianInt) (h : ∀ a ∈ l, IsQuarter a) :
    IsQuarter l.prod := by
  induction l with
  | nil => exact IsQuarter.one
  | cons a l ih =>
    rw [List.prod_cons]
    exact (h a (by simp)).mul (ih fun b hb => h b (by simp [hb]))

theorem colour_quarter_mul {a : GaussianInt} (ha : IsQuarter a) (x : GaussianInt) :
    colour (a * x) = colour x := by
  unfold IsQuarter at ha
  rcases ha with rfl | rfl | rfl | rfl <;> simp [colour, gi] <;> simp [add_comm]

/-- Quarter-turns keep the property "both components have parity `σ`". -/
theorem quarter_mul_parity {a : GaussianInt} (ha : IsQuarter a) (x : GaussianInt) (σ : ZMod 2)
    (hre : (x.re : ZMod 2) = σ) (him : (x.im : ZMod 2) = σ) :
    ((a * x).re : ZMod 2) = σ ∧ ((a * x).im : ZMod 2) = σ := by
  unfold IsQuarter at ha
  rcases ha with rfl | rfl | rfl | rfl <;> simp [gi] <;> simp [hre, him]

/-- A rotation by a quarter-turn about a whole point keeps the colour: the colour
of a vertex does not depend on the path it is developed along. -/
theorem colour_rotAbout {a : GaussianInt} (ha : IsQuarter a) (q x : GaussianInt) :
    colour ((rotAbout a q).apply x) = colour x := by
  rw [apply_rotAbout, smul_eq_mul, colour_add, colour_quarter_mul ha, colour_sub, add_assoc,
    CharTwo.add_self_eq_zero, add_zero]

/-! ### Proposition "Checkerboard rule" -/

/-- The sum of the colours of the vertices of odd valence (rotation `R^{±1}`). -/
def oddColours : List (GaussianInt × GaussianInt) → ZMod 2
  | [] => 0
  | c :: cs => (if c.1 = gi ∨ c.1 = -gi then colour c.2 else 0) + oddColours cs

/-- The number of vertices of odd valence, modulo 2. -/
def oddCount : List (GaussianInt × GaussianInt) → ZMod 2
  | [] => 0
  | c :: cs => (if c.1 = gi ∨ c.1 = -gi then 1 else 0) + oddCount cs

/-- **Proposition "Checkerboard rule".** Both components of the Burgers sum of
quarter-turns about whole points have the parity of the number of vertices of
odd valence on points of odd colour. -/
theorem checkerboard (cs : List (GaussianInt × GaussianInt)) (hq : ∀ c ∈ cs, IsQuarter c.1) :
    ((burgers cs).re : ZMod 2) = oddColours cs ∧ ((burgers cs).im : ZMod 2) = oddColours cs := by
  induction cs with
  | nil => simp [burgers, oddColours]
  | cons c cs ih =>
    obtain ⟨ihre, ihim⟩ := ih fun d hd => hq d (by simp [hd])
    have hQ : IsQuarter ((cs.map Prod.fst).reverse.prod) :=
      IsQuarter.list_prod _ fun a ha => by
        rw [List.mem_reverse, List.mem_map] at ha
        obtain ⟨d, hd, rfl⟩ := ha
        exact hq d (by simp [hd])
    have hterm : (((cs.map Prod.fst).reverse.prod * (c.2 - c.1 * c.2)).re : ZMod 2) =
          (if c.1 = gi ∨ c.1 = -gi then colour c.2 else 0) ∧
        (((cs.map Prod.fst).reverse.prod * (c.2 - c.1 * c.2)).im : ZMod 2) =
          (if c.1 = gi ∨ c.1 = -gi then colour c.2 else 0) := by
      apply quarter_mul_parity hQ
      · have hc := hq c (by simp)
        have e : c.2 - c.1 * c.2 = (1 - c.1) * c.2 := by ring
        rw [e]
        by_cases hodd : c.1 = gi ∨ c.1 = -gi
        · rw [ite_eq_left_of_eq_true _ _ (eq_true hodd)]; exact (parity_one_sub_quarter c.2 c.1 hodd).1
        · rw [ite_eq_right_of_eq_false _ _ (eq_false hodd)]
          unfold IsQuarter at hc
          rcases hc with h | h | h | h
          · simp [h]
          · exact absurd (Or.inl h) hodd
          · rw [h]; simp; ring_nf; simp [zmod_two_two]
          · exact absurd (Or.inr h) hodd
      · have hc := hq c (by simp)
        have e : c.2 - c.1 * c.2 = (1 - c.1) * c.2 := by ring
        rw [e]
        by_cases hodd : c.1 = gi ∨ c.1 = -gi
        · rw [ite_eq_left_of_eq_true _ _ (eq_true hodd)]; exact (parity_one_sub_quarter c.2 c.1 hodd).2
        · rw [ite_eq_right_of_eq_false _ _ (eq_false hodd)]
          unfold IsQuarter at hc
          rcases hc with h | h | h | h
          · simp [h]
          · exact absurd (Or.inl h) hodd
          · rw [h]; simp; ring_nf; simp [zmod_two_two]
          · exact absurd (Or.inr h) hodd
    simp only [burgers, oddColours, smul_eq_mul, Zsqrtd.re_add, Zsqrtd.im_add, Int.cast_add,
      hterm.1, hterm.2, ihre, ihim]
    constructor <;> trivial

/-- The count does not depend on the origin of the development, as long as the
number of vertices of odd valence is even, which Gauss–Bonnet gives (their
indices `4 - k` are odd and add up to `0` or `4` with the even ones). -/
theorem oddColours_translate (cs : List (GaussianInt × GaussianInt)) (t : GaussianInt) :
    oddColours (cs.map fun c => (c.1, c.2 + t)) = oddColours cs + oddCount cs * colour t := by
  induction cs with
  | nil => simp [oddColours, oddCount]
  | cons c cs ih =>
    simp only [List.map_cons, oddColours, oddCount, ih, colour_add]
    split_ifs <;> ring

/-! ### Proposition "Parity of a turn on a sphere", algebraic core -/

/-- A motion `x ↦ R^k x + t` with `t` whole changes the colour by that of `t`. -/
theorem colour_apply (f : Motion GaussianInt GaussianInt) (hf : IsQuarter f.lin)
    (x : GaussianInt) : colour (f.apply x) = colour x + colour f.trans := by
  rw [Motion.apply, smul_eq_mul, colour_add, colour_quarter_mul hf]

/-- The induction of the proof of Proposition "Parity of a turn on a sphere". Let
`Dₖ` develop the coordinates of the `k`-th patch crossed into those of the first,
and let the colouring of the quad graph (a proper 2-colouring, which exists on a
surface of genus zero; this is the hypothesis) be `y₁ + y₂ + κₖ` in the
coordinates of the `k`-th patch. If consecutive patches share a vertex `w`, of
coordinates `y` and `y'` with `Dₖ y = Dₖ₊₁ y'`, then `κₖ = κ₀ + π(Dₖ)`. -/
theorem parity_chain (N : ℕ) (D : ℕ → Motion GaussianInt GaussianInt) (κ : ℕ → ZMod 2)
    (hq : ∀ k, IsQuarter (D k).lin) (h0 : (D 0).trans = 0)
    (hstep : ∀ k < N, ∃ y y', (D k).apply y = (D (k + 1)).apply y' ∧
      colour y + κ k = colour y' + κ (k + 1)) :
    ∀ k ≤ N, κ k = κ 0 + colour (D k).trans := by
  intro k hk
  induction k with
  | zero => simp [h0, colour]
  | succ k ih =>
    obtain ⟨y, y', hy, hc⟩ := hstep k (by omega)
    have h1 := congrArg colour hy
    rw [colour_apply _ (hq k), colour_apply _ (hq (k + 1))] at h1
    have ih' := ih (by omega)
    linear_combination -hc + h1 + ih'

/-- **Proposition "Parity of a turn on a sphere", algebraic core.** Back in the
first patch (`κ_N = κ₀`), the translation `T = D_N` of the turn has
`T₁ + T₂` even; read in the frame of the first side (a quarter-turn of the
patch frame), its components are the length `α` and the shift `s`, so
`α ≡ s (mod 2)`. -/
theorem parity_of_turn (N : ℕ) (D : ℕ → Motion GaussianInt GaussianInt) (κ : ℕ → ZMod 2)
    (hq : ∀ k, IsQuarter (D k).lin) (h0 : (D 0).trans = 0)
    (hstep : ∀ k < N, ∃ y y', (D k).apply y = (D (k + 1)).apply y' ∧
      colour y + κ k = colour y' + κ (k + 1))
    (hback : κ N = κ 0) (a : GaussianInt) (ha : IsQuarter a) (α s : ℤ)
    (hframe : a * (D N).trans = ⟨α, s⟩) :
    (α : ZMod 2) = s := by
  have h := parity_chain N D κ hq h0 hstep N le_rfl
  rw [hback] at h
  have hT : colour (D N).trans = 0 := by
    have := congrArg (· - κ 0) h
    simpa using this.symm
  have : colour (a * (D N).trans) = 0 := by rw [colour_quarter_mul ha, hT]
  rw [hframe, colour] at this
  simp only at this
  push_cast at this
  rw [← sub_eq_zero, sub_eq_add_neg, zmod_two_neg]
  exact this

/-! ### Parity of every loop, from the colours -/

/-- Every holonomy of a word keeps colours, the letters being quarter-turns
about whole points. -/
theorem colour_hol {ι : Type*} (j : ι → ℤ) (p : ι → GaussianInt) (w : List (ι × Bool))
    (x : GaussianInt) : colour ((hol (powers giUnit j) p w).apply x) = colour x := by
  induction w generalizing x with
  | nil => simp [Motion.apply, Motion.id]
  | cons y w ih =>
    have hq : ∀ k : ℤ, IsQuarter ((giUnit ^ k : GaussianIntˣ) : GaussianInt) := by
      intro k
      have h4 : giUnit ^ (4 : ℤ) = 1 := by
        rw [show (4 : ℤ) = ((2 * 2 : ℕ) : ℤ) from rfl, zpow_natCast, pow_mul, giUnit_sq]; simp
      rw [← Int.emod_add_mul_ediv k 4, zpow_add, zpow_mul, h4, one_zpow, mul_one]
      have hr : k % 4 = 0 ∨ k % 4 = 1 ∨ k % 4 = 2 ∨ k % 4 = 3 := by omega
      rcases hr with h | h | h | h <;> rw [h] <;> unfold IsQuarter <;>
        simp [zpow_ofNat, pow_succ, gi_mul_gi]
    have hy : IsQuarter (letterLin (powers giUnit j) y) := by
      obtain ⟨i, b⟩ := y
      cases b <;> simp only [letterLin, powers, ← zpow_neg] <;> exact hq _
    rw [hol_cons, apply_comp, colour_rotAbout hy, ih]

/-- **Proposition "Parity of a turn on a sphere", from the holonomy.** On a surface
of genus zero every loop is a word in the loops round the cone points, so for
whole lengths its holonomy is a product of quarter-turns about whole points.
Its translation then has `T₁ + T₂` even: for a turn, `α ≡ s (mod 2)`. This is a
second proof, which needs neither the bipartite quad graph nor positive lengths. -/
theorem colour_hol_trans {ι : Type*} (j : ι → ℤ) (p : ι → GaussianInt) (w : List (ι × Bool)) :
    colour (hol (powers giUnit j) p w).trans = 0 := by
  have := colour_hol j p w 0
  simpa [Motion.apply, colour] using this

/-! ### Corollary "Checkerboard rule for any combination" -/

section Words

variable {ι : Type*} [DecidableEq ι] [Fintype ι]

theorem re_sum' {κ : Type*} (s : Finset κ) (f : κ → GaussianInt) :
    (∑ k ∈ s, f k).re = ∑ k ∈ s, (f k).re := by
  classical
  induction s using Finset.induction_on with
  | empty => simp
  | insert a s ha ih => rw [Finset.sum_insert ha, Finset.sum_insert ha, Zsqrtd.re_add, ih]

theorem im_sum' {κ : Type*} (s : Finset κ) (f : κ → GaussianInt) :
    (∑ k ∈ s, f k).im = ∑ k ∈ s, (f k).im := by
  classical
  induction s using Finset.induction_on with
  | empty => simp
  | insert a s ha ih => rw [Finset.sum_insert ha, Finset.sum_insert ha, Zsqrtd.im_add, ih]

/-- **Corollary "Checkerboard rule for any combination", for one word.** With the
rotations `Rᵢ = R^{jᵢ}` (`jᵢ = 4 - k(cᵢ)`), both components of `τ(w)` are congruent
modulo 2 to `∑ᵢ εᵢ(w) jᵢ χᵢ`, `εᵢ(w)` the exponent sums (winding numbers) and `χᵢ`
the colours: only vertices of odd valence count. -/
theorem checkerboard_word (j : ι → ℤ) (p : ι → GaussianInt) (w : List (ι × Bool)) :
    (((hol (powers giUnit j) p w).trans.re : ℤ) : ZMod 2) =
        ∑ i, ((expSum i w * j i : ℤ) : ZMod 2) * colour (p i) ∧
      (((hol (powers giUnit j) p w).trans.im : ℤ) : ZMod 2) =
        ∑ i, ((expSum i w * j i : ℤ) : ZMod 2) * colour (p i) := by
  obtain ⟨v, hv⟩ := burgers_congr giUnit j p w
  rw [hv]
  simp only [smul_eq_mul, giUnit_val]
  have hsq : (1 - gi) ^ 2 = ⟨0, -2⟩ := by ext <;> simp [sq, gi]
  rw [hsq]
  set q := ∑ i, ((expSum i w * j i : ℤ) : GaussianInt) * p i with hq
  have hqre : ((q.re : ℤ) : ZMod 2) + ((q.im : ℤ) : ZMod 2) =
      ∑ i, ((expSum i w * j i : ℤ) : ZMod 2) * colour (p i) := by
    rw [hq, re_sum', im_sum']
    push_cast
    rw [← Finset.sum_add_distrib]
    refine Finset.sum_congr rfl fun i _ => ?_
    simp [colour]
    ring
  constructor
  · rw [← hqre]; simp [gi]; ring_nf; try simp [zmod_two_two]
  · rw [← hqre]; simp [gi]; ring_nf
    (try simp only [zmod_two_two, add_zero, mul_zero])

/-- A component `±τ₁` or `±τ₂` of a translation, chosen by `k : Fin 4`. -/
def component (k : Fin 4) (t : GaussianInt) : ℤ :=
  match k with
  | 0 => t.re
  | 1 => t.im
  | 2 => -t.re
  | 3 => -t.im

/-- **Corollary "Checkerboard rule for any combination".** For whole multipliers
`cᵣ`, words `wᵣ` and components `±e₁, ±e₂` (shift or length of each turn),
`∑ᵣ cᵣ sᵣ ≡ ∑ᵢ wᵢ jᵢ χᵢ (mod 2)`, with `wᵢ = ∑ᵣ cᵣ εᵢ(wᵣ)` the winding numbers of
the weighted cycle. -/
theorem checkerboard_combination {κ : Type*} (s : Finset κ) (j : ι → ℤ) (p : ι → GaussianInt)
    (c : κ → ℤ) (dir : κ → Fin 4) (w : κ → List (ι × Bool)) :
    ((∑ r ∈ s, c r * component (dir r) (hol (powers giUnit j) p (w r)).trans : ℤ) : ZMod 2) =
      ∑ i, ((∑ r ∈ s, c r * expSum i (w r) : ℤ) : ZMod 2) * (j i : ZMod 2) * colour (p i) := by
  push_cast
  simp only [Finset.sum_mul]
  rw [Finset.sum_comm]
  refine Finset.sum_congr rfl fun r _ => ?_
  have h := checkerboard_word j p (w r)
  have hc : ((component (dir r) (hol (powers giUnit j) p (w r)).trans : ℤ) : ZMod 2) =
      ∑ i, ((expSum i (w r) * j i : ℤ) : ZMod 2) * colour (p i) := by
    unfold component
    split <;> (try simp only [Int.cast_neg, zmod_two_neg]) <;> first | exact h.1 | exact h.2
  rw [hc, Finset.mul_sum]
  refine Finset.sum_congr rfl fun i _ => ?_
  push_cast; ring

theorem orderOf_giUnit : orderOf giUnit = 4 := by
  have h := orderOf_eq_prime_pow (p := 2) (n := 1) (x := giUnit) ?_ ?_
  · simpa using h
  · rw [pow_one, giUnit_sq]
    intro h2
    have := congrArg (fun u : GaussianIntˣ => ((u : GaussianInt)).re) h2
    simp at this
  · rw [show (2 : ℕ) ^ (1 + 1) = 2 * 2 from rfl, pow_mul, giUnit_sq]
    simp

/-- A word with trivial rotation has `∑ᵢ εᵢ jᵢ ≡ 0 (mod 4)`, hence even: moving
the origin of the development by a whole vector, which flips every colour,
does not change the right-hand side of the corollary. -/
theorem colours_flip (j : ι → ℤ) (w : List (ι × Bool))
    (h : rhoW (powers giUnit j) w = 1) (p : ι → GaussianInt) (t : GaussianInt) :
    ∑ i, ((expSum i w * j i : ℤ) : ZMod 2) * colour (p i + t) =
      ∑ i, ((expSum i w * j i : ℤ) : ZMod 2) * colour (p i) := by
  have hd := sum_dvd_of_rhoW_eq_one giUnit j w h
  rw [orderOf_giUnit] at hd
  have h2 : ((∑ i, expSum i w * j i : ℤ) : ZMod 2) = 0 := by
    rw [ZMod.intCast_zmod_eq_zero_iff_dvd]
    exact (Int.dvd_trans (by norm_num) hd)
  simp only [colour_add, mul_add, Finset.sum_add_distrib, ← Finset.sum_mul]
  rw [← Int.cast_sum, h2, zero_mul, add_zero]

/-- The relation `ϱ = λ₁ ⋯ λₙ` (each letter once) has trivial holonomy, so the
colours satisfy `∑ᵢ jᵢ χᵢ ≡ 0`: adding a constant to all the winding numbers does
not change the right-hand side of the corollary. -/
theorem relation_colours (j : ι → ℤ) (p : ι → GaussianInt) (ϱ : List (ι × Bool))
    (hϱ : ∀ i, expSum i ϱ = 1) (htrans : (hol (powers giUnit j) p ϱ).trans = 0) :
    ∑ i, (j i : ZMod 2) * colour (p i) = 0 := by
  have h := (checkerboard_word j p ϱ).1
  rw [htrans] at h
  simp only [hϱ, one_mul, Zsqrtd.re_zero, Int.cast_zero] at h
  exact h.symm

theorem winding_constant (j : ι → ℤ) (p : ι → GaussianInt) (e : ι → ℤ) (q : ℤ)
    (hrel : ∑ i, (j i : ZMod 2) * colour (p i) = 0) :
    ∑ i, ((e i + q : ℤ) : ZMod 2) * (j i : ZMod 2) * colour (p i) =
      ∑ i, ((e i : ℤ) : ZMod 2) * (j i : ZMod 2) * colour (p i) := by
  push_cast
  simp only [add_mul, Finset.sum_add_distrib, mul_assoc, ← Finset.mul_sum, hrel, mul_zero,
    add_zero]

end Words

end ClosingHelices
