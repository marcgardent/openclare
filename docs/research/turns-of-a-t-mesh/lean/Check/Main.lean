/-
Checks the T-mesh of `TurnsOfATMesh/TMesh.lean` against the engine's exports. Not a
part of the library: a program run on data, with the few lemmas it needs to read the
library's definitions fast.

    lake env lean --run Check/Main.lean <converted.json>...

Each file is a way of closing exported by the engine (`OPENCLARE_EXPORT_TURNS`) and
converted by `Check/convert.py`. For each, the program builds the `TMesh` (its proof
fields discharged by checks run on the data; it stops when they do not hold) and the
turns of the way, and compares, with the library's definitions:

* the hypotheses: sides without repeated arcs and every arc on exactly two distinct
  sides (the fields of `TMesh`), `NoArcAcross`, `Oriented`; every violation is listed;
* `balance` with the engine's balances (`stations`), row by row: the row `(P, 0)` is
  the engine's row of pair 0 (`v = 0` less `v = h`), the row `(P, 1)` the engine's row
  of pair 1 negated (the engine takes `u = w` less `u = 0`, the library the first side
  of the pair, `u = 0`, less the second);
* `Turn.shiftForm` with the engine's shift of every turn: coefficient by coefficient,
  and modulo the balances, `shiftForm - shift = λᵀ B` with `λ` found here and the
  identity checked exactly against `balance`;
* at the lengths the engine walked the turns at (`walked`): that its offset lies in
  `Turn.corridor` (`Follows`), that `walk` passes within `10⁻⁶` of where the engine
  says it crossed each arc, and that it comes back at the offset plus `Turn.shift`;
* at the real lengths, the offset at the same share of the first side, and at the
  engine's whole lengths every half-integer of the corridor: `walk` comes back at the
  offset plus the shift;
* Theorem "Forced shifts", when `certify.py` has certificates for the way: each Farkas
  certificate as `TMesh.not_closable_iff` states it, the bounds on `τ_P(H)` of the held
  turns they give (disjoint cores, `forced_of_certificates`; the least set meeting the
  cores, `meets_infeasible`; whole lengths of `P` closing the others), against the
  claims of the papers, and the sets of their table;
* the Fox–Burgers formula, on a sphere: the words of the turns in the loops round the
  cone points (`develop.py`), whose holonomy `Fox.hol` over `ℤ[i]` must be the
  translation by the length of the turn along its walk and minus its `shift` across it,
  and equal the sum of Proposition "Fox–Burgers formula" (`fox`);
* the quad mesh at the whole lengths, from the definitions of `Grid.lean` (DT-134): its
  vertices (the classes of the gluings, by union-find), its Euler characteristic against
  `∑ (4 - k)` over its cone points, one round of the walk from every station as a closed
  curve of the fine grid, and on every simple curve the regions it cuts, each with
  Lemma "Gauss–Bonnet for turns": `∑ (4 - k) = 4 χ`; the 2-colouring of the quad graph
  (`IsColouring`, constants `κ` found by propagation; a sphere must be bipartite) and,
  when it holds, `Turn.length ≡ Turn.shift (mod 2)` for every turn with a half-integer
  in its corridor;
* on a sphere, Proposition "The shift is set by the singularities" (a) on both regions of
  every simple curve: the cuts off a dual spanning tree form a forest whose trees each
  meet the curve at one vertex (a disc); the faces developed along the tree
  (`crossMotion`); the rotations `R^{4-k}` about the cone points in the order of first
  visit of the walk round the trees, whose composite and Burgers sum are compared with
  `Turn.development` of the turn of the curve and with the words of `develop.py`; the
  faces are developed by the library's `crossEdge` (`Border.lean`);
* the chart along the border of every round (`Border.lean`): the steps of the round
  against its points, the holonomy of its border (`borderHolonomy`) against the
  development of its turn, and at every vertex of the quad graph the walk round it
  (`valenceAround`) against the union-find, its holonomy against `R^{4-k}` about it and
  every point it meets in its class;
* on a sphere, the jumps across the cuts (`Region.lean`), on both regions of every
  simple curve: the border crossed face by face (`borderCrossings`, its faces chained
  as `borderCrossings_chain` states, every face in the region), the charts of the dual
  tree agreeing across every crossing of the tree, and the composite of the jumps
  across the cuts, in border order, against the holonomy of the border and the
  development of the turn (`borderHolonomy_cuts`); the region on the right of a curve
  is the region on the left of the curve walked back.

All arithmetic is exact (`ℤ`, `ℚ`; the engine's floats are read as the fractions they
are). `Turn.shiftForm` and `balance` are read only at the arcs of the sides they
involve; `shiftForm_eq_zero` and `balance_eq_zero` below prove that they are zero at
every other arc.
-/
import TurnsOfATMesh.TMesh
import TurnsOfATMesh.Fox
import TurnsOfATMesh.Grid
import TurnsOfATMesh.Holonomy
import TurnsOfATMesh.Border
import TurnsOfATMesh.Region
import Lean.Data.Json
import Std.Data.HashMap
import Std.Data.HashSet

open Lean TurnsOfATMesh TMesh

namespace Check

/-! ### Reading the converted export -/

structure RawTurn where
  index : Nat
  held : Bool
  start : Nat
  path : Array Nat
  shift : Array (Nat × Int)
  offset : ℚ
  share : Option ℚ
  crossed : Array ℚ

/-- A Farkas certificate of Remark "Sign-definite turns" for the turns `turns` (indices
of the export): the multipliers `u` of their shifts and the form `c` it claims for
`Σ u shiftForm + λᵀ B` (its nonzero terms); `λ` is left to the check. -/
structure RawCert where
  turns : Array Nat
  u : Array ℚ
  c : Array (Nat × ℚ)
  deriving Inhabited

/-- A set of a table of certificates in one way: the certificate found with the
multipliers of the paper (`none` if none was), whole lengths closing the set without each
of its turns (when found), and what the paper says of it. -/
structure RawSet where
  row : Nat
  turns : Array Nat
  cert : Option RawCert
  irreducible : Array (Nat × Option (Array ℚ))
  multipliers : Option (Array ℚ)
  stated : Option String
  bounded : Array Nat
  exact : String
  first : String

/-- The data of `certify.py` for one way: the bounds `b`, the claims of the paper, the
cores of held turns with their certificates, the disjoint ones, a set `X` of held turns
and whole lengths closing the others, and the sets of a table. -/
structure RawForced where
  source : String
  bounds : Array (Nat × Int)
  heldClaim : Option Nat
  tauClaim : Option Nat
  cores : Array RawCert
  disjoint : Array Nat
  shifted : Array Nat
  closing : Array ℚ
  sets : Array RawSet

/-- The words of `develop.py`: the cone points (valence and developed place, whole), each
turn's word with the rotation of the chart of its first patch, each jump across the cut
tree with its word and its motion `(k, x, y)`. -/
structure RawFox where
  convention : String
  cones : Array (Int × Int × Int)
  turns : Array (List (Nat × Bool) × Nat)
  jumps : Array (List (Nat × Bool) × Nat × Int × Int)

structure Raw where
  patches : Nat
  arcs : Nat
  sides : Array (Array Nat)
  holders : Array (Array (Nat × Int))
  stations : Array (Nat × Nat × Array (Nat × Int))
  whole : Array ℚ
  walked : Array ℚ
  real : Array ℚ
  turns : Array RawTurn
  lower : Array Int
  forced : Option RawForced
  fox : Option RawFox
  foxSkipped : String

def field (j : Json) (k : String) : Except String Json := j.getObjVal? k

def arr (j : Json) : Except String (Array Json) := j.getArr?

def nat (j : Json) : Except String Nat := j.getNat?

def int (j : Json) : Except String Int := j.getInt?

def rat (j : Json) : Except String ℚ := do
  let a ← arr j
  if a.size != 2 then throw "a fraction is [num, den]"
  return (((← int a[0]!) : ℚ) / ((← nat a[1]!) : ℚ))

def pair (j : Json) : Except String (Nat × Int) := do
  let a ← arr j
  if a.size != 2 then throw "a term is [arc, coefficient]"
  return (← nat a[0]!, ← int a[1]!)

def form (j : Json) : Except String (Array (Nat × Int)) := do (← arr j).mapM pair

def readTurn (j : Json) : Except String RawTurn := do
  let share ← match field j "share" with
    | .ok .null => pure none
    | .ok s => some <$> rat s
    | .error e => throw e
  return {
    index := ← nat (← field j "index")
    held := ← (← field j "held").getBool?
    start := ← nat (← field j "start")
    path := ← (← arr (← field j "path")).mapM nat
    shift := ← form (← field j "shift")
    offset := ← rat (← field j "offset")
    share
    crossed := ← (← arr (← field j "crossed")).mapM rat }

/-- A field that may be missing or `null`. -/
def opt {α : Type} (j : Json) (k : String) (f : Json → Except String α) : Except String (Option α) :=
  match j.getObjValD k with
  | .null => pure none
  | v => some <$> f v

def ints (j : Json) : Except String (Array ℚ) := do
  (← arr j).mapM fun x => do return ((← int x) : ℚ)

def readCert (j : Json) : Except String RawCert := do
  return {
    turns := ← (← arr (← field j "turns")).mapM nat
    u := ← (← arr (← field j "u")).mapM rat
    c := ← (← arr (← field j "c")).mapM fun t => do
      let t ← arr t
      return (← nat t[0]!, (((← int t[1]!) : ℚ) / ((← nat t[2]!) : ℚ))) }

def readSet (j : Json) : Except String RawSet := do
  let paper ← field j "paper"
  let cert ← match j.getObjValD "u" with
    | .null => pure none
    | _ => some <$> readCert j
  return {
    row := ← nat (← field j "row")
    turns := ← (← arr (← field j "turns")).mapM nat
    cert
    irreducible := ← (← arr (← field j "irreducible")).mapM fun i => do
      return (← nat (← field i "without"), ← opt i "closing" ints)
    multipliers := ← opt paper "multipliers" fun m => do (← arr m).mapM rat
    stated := ← opt paper "stated" fun s => s.getStr?
    bounded := ← (← arr (← field paper "bounded")).mapM nat
    exact := ← (← field paper "exact").getStr?
    first := ← (← field paper "first").getStr? }

def readForced (j : Json) : Except String RawForced := do
  let tau ← field j "tau"
  let claims := j.getObjValD "claims"
  return {
    source := ← (← field j "source").getStr?
    bounds := ← (← arr (← field j "bounds")).mapM pair
    heldClaim := ← opt claims "held" nat
    tauClaim := ← opt claims "tau" nat
    cores := ← (← arr (← field tau "cores")).mapM readCert
    disjoint := ← (← arr (← field tau "disjoint")).mapM nat
    shifted := ← (← arr (← field tau "shifted")).mapM nat
    closing := ← ints (← field tau "closing")
    sets := ← match j.getObjValD "sets" with
      | .null => pure #[]
      | s => do (← arr s).mapM readSet }

def word (j : Json) : Except String (List (Nat × Bool)) := do
  let w ← (← arr j).mapM fun x => do
    let x ← arr x
    return (← nat x[0]!, (← int x[1]!) == 1)
  return w.toList

def readFox (j : Json) : Except String RawFox := do
  let c ← field j "convention"
  return {
    convention := s!"{(← (← field c "generator").getStr?)}; children {(← (← field c "children").getStr?)}; rotation {(← (← field c "rotation").getStr?)}"
    cones := ← (← arr (← field j "cones")).mapM fun c => do
      let at_ ← arr (← field c "at")
      return (← int (← field c "valence"), ← int at_[0]!, ← int at_[1]!)
    turns := ← (← arr (← field j "turns")).mapM fun t => do
      return (← word (← field t "word"), ← nat (← field t "chart"))
    jumps := ← (← arr (← field j "jumps")).mapM fun t => do
      let m ← arr (← field t "motion")
      return (← word (← field t "word"), ← nat m[0]!, ← int m[1]!, ← int m[2]!) }

def readRaw (j : Json) : Except String Raw := do
  let stations ← (← arr (← field j "stations")).mapM fun s => do
    return (← nat (← field s "patch"), ← nat (← field s "pair"), ← form (← field s "row"))
  return {
    lower := ← (← arr (← field j "lower")).mapM int
    forced := ← opt j "certificates" readForced
    fox := ← opt j "fox" readFox
    foxSkipped := match j.getObjValD "fox_skipped" with
      | .str s => s
      | _ => ""
    patches := ← nat (← field j "patches")
    arcs := ← nat (← field j "arcs")
    sides := ← (← arr (← field j "sides")).mapM fun s => do (← arr s).mapM nat
    holders := ← (← arr (← field j "holders")).mapM form
    stations
    whole := ← (← arr (← field j "whole")).mapM fun x => do return ((← int x) : ℚ)
    walked := ← (← arr (← field j "walked")).mapM rat
    real := ← (← arr (← field j "real")).mapM rat
    turns := ← (← arr (← field j "turns")).mapM readTurn }

/-! ### Building the T-mesh -/

variable {P n : Nat}

/-- The side of index `4 P + 2 j + s`. -/
def sideOf (i : Nat) : Option (Side (Fin P)) :=
  if h : i / 4 < P then some (⟨i / 4, h⟩, ⟨i % 4 / 2, by omega⟩, ⟨i % 2, by omega⟩) else none

def sideIndex (σ : Side (Fin P)) : Nat := 4 * σ.1.val + 2 * σ.2.1.val + σ.2.2.val

def arcOf (a : Nat) : Option (Fin n) := if h : a < n then some ⟨a, h⟩ else none

/-- A `TMesh` from its data, when the data meet its fields; `mem_arcs` from two checks
linear in the size of the data. -/
def mkTMesh (arcs : Side (Fin P) → List (Fin n)) (ends : Fin n → Fin 2 → Side (Fin P))
    (sign : Fin n → ℤ) : Option (TMesh (Fin P) (Fin n)) :=
  if h₁ : ∀ σ, (arcs σ).Nodup then
    if h₂ : ∀ a, ends a 0 ≠ ends a 1 then
      if each : ∀ σ, ∀ a ∈ arcs σ, σ = ends a 0 ∨ σ = ends a 1 then
        if both : ∀ a, a ∈ arcs (ends a 0) ∧ a ∈ arcs (ends a 1) then
          if h₄ : ∀ a, sign a = 1 ∨ sign a = -1 then
            some { arcs, arcs_nodup := h₁, ends, ends_ne := h₂
                   mem_arcs := fun a σ => ⟨each σ a, by
                     rintro (rfl | rfl)
                     exacts [(both a).1, (both a).2]⟩
                   sign, sign_isSign := h₄ }
          else none
        else none
      else none
    else none
  else none

variable (T : TMesh (Fin P) (Fin n))

def decIsPath : (σ : Side (Fin P)) → (l : List (Fin n)) → Decidable (T.IsPath σ l)
  | _, [] => isTrue trivial
  | σ, a :: l =>
    have := decIsPath (T.other a σ.opp) l
    inferInstanceAs (Decidable (a ∈ T.arcs σ.opp ∧ T.IsPath (T.other a σ.opp) l))

def decFollows (ℓ : Fin n → ℚ) : (σ : Side (Fin P)) → (l : List (Fin n)) → (x : ℚ) →
    Decidable (T.Follows ℓ σ l x)
  | _, [], _ => isTrue trivial
  | σ, a :: l, x =>
    have := decFollows ℓ (T.other a σ.opp) l ((T.crossAt ℓ σ.opp a).apply x)
    inferInstanceAs (Decidable (arcAt ℓ (T.arcs σ.opp) x = some a ∧
      T.Follows ℓ (T.other a σ.opp) l ((T.crossAt ℓ σ.opp a).apply x)))

/-- Whether the walk from `x` follows `l`: `x ∈ γ.corridor ℓ` for a turn along `l`. -/
def follows (ℓ : Fin n → ℚ) (σ : Side (Fin P)) (l : List (Fin n)) (x : ℚ) : Bool :=
  @decide _ (decFollows T ℓ σ l x)

def mkTurn (start : Side (Fin P)) (path : List (Fin n)) : Option T.Turn :=
  haveI := decIsPath T start path
  if h₁ : T.IsPath start path then
    if h₂ : T.endSide start path = start then some ⟨start, path, h₁, h₂⟩ else none
  else none

instance : Decidable T.NoArcAcross :=
  inferInstanceAs (Decidable (∀ a, (T.ends a 0).row ≠ (T.ends a 1).row))

instance : Decidable T.Oriented :=
  inferInstanceAs (Decidable (∀ a, T.sign a = -((T.ends a 0).orient * (T.ends a 1).orient)))

/-! ### Where the forms vanish

The forms of the library are functions of all the arcs; these lemmas let the program
read them only where they can be nonzero. -/

theorem posForm_eq_zero {a b : Fin n} : ∀ {l : List (Fin n)}, b ∉ l → posForm a l b = 0
  | [], _ => rfl
  | c :: l, h => by
    rw [List.mem_cons, not_or] at h
    simp only [posForm]
    split_ifs
    · rfl
    · simp [h.1, posForm_eq_zero h.2]

theorem crossForm_trans_eq_zero {τ : Side (Fin P)} {a b : Fin n} (h₁ : b ∉ T.arcs τ)
    (h₂ : b ∉ T.arcs (T.other a τ)) (h₃ : b ≠ a) : (T.crossForm τ a).trans b = 0 := by
  unfold TMesh.crossForm
  split_ifs <;> simp [posForm_eq_zero h₁, posForm_eq_zero h₂, h₃]

/-- The arcs the crossings along `l` from `σ` involve: those crossed and those of the
sides they lie on. -/
def support : Side (Fin P) → List (Fin n) → List (Fin n)
  | _, [] => []
  | σ, a :: l => a :: (T.arcs σ.opp ++ T.arcs (T.other a σ.opp) ++ support (T.other a σ.opp) l)

theorem crossings_trans_eq_zero {b : Fin n} : ∀ {σ : Side (Fin P)} {l : List (Fin n)},
    b ∉ support T σ l → (Motion.composeAll (T.crossings σ l)).trans b = 0
  | _, [], _ => rfl
  | σ, a :: l, h => by
    simp only [support, List.mem_cons, List.mem_append, not_or] at h
    obtain ⟨h₃, ⟨h₁, h₂⟩, h₄⟩ := h
    simp only [TMesh.crossings, Motion.composeAll_cons, Motion.comp_trans, Pi.add_apply,
      Pi.smul_apply, crossings_trans_eq_zero h₄, crossForm_trans_eq_zero T h₁ h₂ h₃, smul_zero,
      add_zero]

/-- `shiftForm` vanishes off the support of the turn. -/
theorem shiftForm_eq_zero (γ : T.Turn) {b : Fin n} (h : b ∉ support T γ.start γ.path) :
    γ.shiftForm b = 0 :=
  crossings_trans_eq_zero T h

/-- A row of `balance` vanishes off the arcs of its two sides. -/
theorem balance_eq_zero (r : Fin P × Fin 2) {a : Fin n} (h₀ : a ∉ T.arcs (r.1, r.2, 0))
    (h₁ : a ∉ T.arcs (r.1, r.2, 1)) : T.balance r a = 0 := by
  simp [TMesh.balance, lengthForm_apply, List.count_eq_zero_of_not_mem h₀,
    List.count_eq_zero_of_not_mem h₁]

/-- The coefficients of `shiftForm`, read on the support (zero elsewhere,
`shiftForm_eq_zero`). -/
def coefficients (γ : T.Turn) : Array Int :=
  (support T γ.start γ.path).foldl (fun v a => v.set! a.val (γ.shiftForm a)) (Array.replicate n 0)

/-- The columns of `balance`: for each arc, its nonzero entries `(2 p + j, B_{(p, j) a})`,
read on the arcs of the two sides of each row (zero elsewhere, `balance_eq_zero`). -/
def columns : Array (Array (Nat × Int)) := Id.run do
  let mut cols : Array (Array (Nat × Int)) := Array.replicate n #[]
  for p in List.finRange P do
    for j in List.finRange 2 do
      for a in (T.arcs (p, j, 0) ++ T.arcs (p, j, 1)).eraseDups do
        let v := T.balance (p, j) a
        if v ≠ 0 then cols := cols.modify a.val (·.push (2 * p.val + j.val, v))
  return cols

/-! ### Modulo the balances

A guess of `λ` with `λᵀ B = d`, for `B` given by its columns, each with two entries
`±1` (the graph of the balances is bidirected, `balance_isBidirected`): over each
connected component the rows are `c + k t` from a root, and `t` is fixed by an
unbalanced cycle. Only a guess; `residual` checks it. -/

def guess (R : Nat) (cols : Array (Array (Nat × Int))) (d : Array ℚ) : Array ℚ := Id.run do
  let mut atRow : Array (Array Nat) := Array.replicate R #[]
  for a in [0:cols.size] do
    for (r, _) in cols[a]! do atRow := atRow.modify r (·.push a)
  let mut c : Array ℚ := Array.replicate R 0
  let mut k : Array Int := Array.replicate R 0
  let mut lam : Array ℚ := Array.replicate R 0
  for root in [0:R] do
    if k[root]! != 0 then continue
    k := k.set! root 1
    let mut queue : Array Nat := #[root]
    let mut i := 0
    let mut t : Option ℚ := none
    while i < queue.size do
      let r := queue[i]!
      i := i + 1
      for a in atRow[r]! do
        let col := cols[a]!
        if col.size != 2 then continue
        let (r₀, s₀) := col[0]!
        let (r₁, s₁) := col[1]!
        let da := d[a]!
        if k[r₀]! != 0 && k[r₁]! == 0 then
          c := c.set! r₁ (s₁ * (da - s₀ * c[r₀]!))
          k := k.set! r₁ (-(s₁ * s₀ * k[r₀]!))
          queue := queue.push r₁
        else if k[r₁]! != 0 && k[r₀]! == 0 then
          c := c.set! r₀ (s₀ * (da - s₁ * c[r₁]!))
          k := k.set! r₀ (-(s₀ * s₁ * k[r₁]!))
          queue := queue.push r₀
        else if t.isNone then
          let coefficient := s₀ * k[r₀]! + s₁ * k[r₁]!
          if coefficient != 0 then
            t := some ((da - s₀ * c[r₀]! - s₁ * c[r₁]!) / coefficient)
    for r in queue do lam := lam.set! r (c[r]! + k[r]! * t.getD 0)
  return lam

/-- The arcs `a` at which `(λᵀ B)_a ≠ d_a`. -/
def residual (cols : Array (Array (Nat × Int))) (lam : Array ℚ) (d : Array ℚ) : List Nat :=
  (List.range cols.size).filter fun a =>
    (cols[a]!.foldl (fun s (r, v) => s + lam[r]! * v) (0 : ℚ)) != d[a]!

/-! ### Reports -/

structure Count where
  checked : Nat := 0
  agree : Nat := 0
  failures : Array String := #[]

def Count.add (c : Count) (ok : Bool) (what : Unit → String) : Count :=
  { c with checked := c.checked + 1, agree := c.agree + (if ok then 1 else 0),
           failures := if ok then c.failures else c.failures.push (what ()) }

def Count.ok (c : Count) : Bool := c.failures.isEmpty

def Count.show (c : Count) (name : String) (shown : Nat := 10) : IO Unit := do
  IO.println s!"  {name}: {c.agree} of {c.checked}"
  for f in c.failures.toList.take shown do IO.println s!"    {f}"
  if c.failures.size > shown then IO.println s!"    ... {c.failures.size - shown} more"

/-- A form as a dense vector. -/
def dense (n : Nat) (f : Array (Nat × Int)) : Array Int :=
  f.foldl (fun v (a, c) => if a < v.size then v.modify a (· + c) else v) (Array.replicate n 0)

/-- The terms of a dense vector. -/
def terms (v : Array Int) : List (Nat × Int) :=
  (List.range v.size).filterMap fun a => if v[a]! = 0 then none else some (a, v[a]!)

def lengths (v : Array ℚ) : Fin n → ℚ := fun a => v[a.val]!

def approx (x : ℚ) : Float := Float.ofInt x.num / x.den.toFloat

def near (x y : ℚ) : Bool := x - y ≤ 1 / 1000000 && y - x ≤ 1 / 1000000

/-! ### Theorem "Forced shifts": certificates and `τ_P(H)`

A certificate `(u, c)` for a set `S` of turns is checked as `TMesh.not_closable_iff`
states it: with `λ` solved here from the `c` claimed, the vector
`Σ_γ u_γ shiftForm γ + λᵀ B` (the coefficients of `shiftForm` and the columns of
`balance`) is that `c`, `c ≥ 0` at every arc and `c · b > 0`. Then `S` cannot be closed
within `P = {B ℓ = 0, ℓ ≥ b}`. The bounds `τ_P(H)` of the held turns follow: `k` pairwise
disjoint certified cores give `τ_P(H) ≥ k` (`forced_of_certificates`); a set meeting every
certified core is needed (`meets_infeasible`), so the least such set is a lower bound too;
whole lengths of `P` closing every held turn outside a set `X` give `τ_P(H) ≤ |X|`. -/

/-- `Σ_k u_k · shiftForm (turns k)`, dense, from the coefficients of the turns. -/
def combine (n : Nat) (forms : Array (Option (Array Int))) (turns : Array Nat) (u : Array ℚ) :
    Except String (Array ℚ) := do
  let mut v : Array ℚ := Array.replicate n 0
  for k in [0:turns.size] do
    let some f := forms[turns[k]!]?.join | throw s!"turn {turns[k]!} is not a turn of the TMesh"
    for a in [0:n] do
      if f[a]! != 0 then v := v.modify a (· + u[k]! * f[a]!)
  return v

/-- `λᵀ B`, from the columns of `balance`. -/
def rowsTimes (cols : Array (Array (Nat × Int))) (lam : Array ℚ) : Array ℚ :=
  cols.map fun col => col.foldl (fun s (r, v) => s + lam[r]! * v) 0

/-- Checks a certificate; returns its form `c`. -/
def checkCert (R n : Nat) (cols : Array (Array (Nat × Int))) (forms : Array (Option (Array Int)))
    (b : Array ℚ) (cert : RawCert) : Except String (Array ℚ) := do
  if cert.turns.size != cert.u.size then throw "not one multiplier per turn"
  if !decide cert.turns.toList.Nodup then throw "a turn is repeated"
  let comb ← combine n forms cert.turns cert.u
  let claimed := cert.c.foldl (fun v (a, x) => if a < n then v.modify a (· + x) else v)
    (Array.replicate n (0 : ℚ))
  let d := (Array.range n).map fun a => claimed[a]! - comb[a]!
  let lam := guess R cols d
  let lamB := rowsTimes cols lam
  let c := (Array.range n).map fun a => comb[a]! + lamB[a]!
  if c != claimed then
    throw s!"no λ with Σ u shiftForm + λᵀ B = c (residual at {(residual cols lam d).take 5})"
  let negative := (List.range n).filter fun a => c[a]! < 0
  if !negative.isEmpty then throw s!"c < 0 at the arcs {negative.take 5}"
  let cb := (List.range n).foldl (fun s a => s + c[a]! * b[a]!) (0 : ℚ)
  if cb ≤ 0 then throw s!"c · b = {cb}, not positive"
  return c

/-- Whether `ℓ ∈ P` (`B ℓ = 0` row by row, from `balance`; `ℓ ≥ b`) closes `turns`. -/
def closesIn (rows : Array (Array (Nat × Int))) (forms : Array (Option (Array Int)))
    (b : Array ℚ) (ℓ : Array ℚ) (turns : List Nat) : Except String Unit := do
  if ℓ.size != b.size then throw "lengths: not one per arc"
  if let some a := (List.range b.size).find? fun a => ℓ[a]! < b[a]! then
    throw s!"ℓ < b at the arc {a}"
  if let some r := (List.range rows.size).find? fun r =>
      rows[r]!.foldl (fun s (a, v) => s + v * ℓ[a]!) (0 : ℚ) != 0 then
    throw s!"B ℓ ≠ 0 at the row {r}"
  for t in turns do
    let some f := forms[t]?.join | throw s!"turn {t} is not a turn of the TMesh"
    let s := (List.range b.size).foldl (fun s a => s + ((f[a]! : Int) : ℚ) * ℓ[a]!) (0 : ℚ)
    if s != 0 then throw s!"turn {t} shifted by {s}"

/-- Greedy pairwise disjoint cores: a lower bound on a set meeting them all. -/
def packing (cores : Array (Array Nat)) : Nat := Id.run do
  let mut used : Array Nat := #[]
  let mut k := 0
  for c in cores do
    if !c.any used.contains then
      used := used ++ c
      k := k + 1
  return k

/-- The least size of a set meeting every core, if below `best` (branch and bound), with
the nodes left of `fuel`; `none` when the search needs more than `fuel` nodes. -/
partial def leastHitting (cores : Array (Array Nat)) (chosen : List Nat) (best fuel : Nat) :
    Option (Nat × Nat) :=
  if fuel = 0 then none else
  let left := cores.filter fun c => !c.any chosen.contains
  if left.isEmpty then some (min best chosen.length, fuel - 1)
  else if chosen.length + packing left ≥ best then some (best, fuel - 1)
  else
    let c := left.foldl (fun c d => if d.size < c.size then d else c) left[0]!
    c.foldl (fun found e => found.bind fun (best, fuel) => leastHitting cores (e :: chosen) best fuel)
      (some (best, fuel - 1))

/-- The nodes `leastHitting` may visit: beyond, only the disjoint cores bound `τ_P(H)`
below. -/
def hittingFuel : Nat := 100000

def showQ (x : ℚ) : String := if x.den = 1 then s!"{x.num}" else s!"{x.num}/{x.den}"

def checkForced (n R : Nat) (cols rows : Array (Array (Nat × Int)))
    (forms : Array (Option (Array Int))) (held : Array Bool) (lower : Array Int)
    (f : RawForced) : IO Bool := do
  IO.println s!"  forced shifts, certificates of {f.source}:"
  let b : Array ℚ := f.bounds.foldl (fun v (a, x) => if a < n then v.set! a x else v)
    (Array.replicate n 0)
  let beyond := (List.range n).filter fun a => b[a]! != ((max lower[a]! 0 : Int) : ℚ)
  IO.println s!"    bounds b: {((List.range n).filter (b[·]! > 0)).length} arcs at least 1, {if beyond.isEmpty then "the engine's" else s!"DIFFERENT from the engine's at {beyond.length} arcs"}"
  let mut ok := beyond.isEmpty
  -- H: the held turns, counted up to sign.
  let H := (List.range held.size).filter (held[·]!)
  let sparse := H.map fun t => (t, ((forms[t]?.join).map fun f => terms f).getD [])
  let twins := sparse.foldl (fun k (t, f) => k + (sparse.filter fun (t', g) =>
    t < t' && (g == f || g == f.map fun (a, c) => (a, -c))).length) 0
  let heldOk := f.heldClaim.all (· == H.length)
  IO.println s!"    held turns H: {H.length}{match f.heldClaim with | some k => s!", paper {k}" | none => ""}; pairs with the same shift form up to sign: {twins}"
  ok := ok && heldOk
  -- The cores and their certificates.
  let mut valid : Array (Array Nat) := #[]
  let mut validIdx : Array Nat := #[]
  for k in [0:f.cores.size] do
    let cert := f.cores[k]!
    match checkCert R n cols forms b cert with
    | .ok _ =>
      if cert.turns.all (H.contains ·) then
        valid := valid.push cert.turns
        validIdx := validIdx.push k
      else IO.println s!"    core {k}: a turn is not held"
    | .error e => IO.println s!"    core {k} {cert.turns}: INVALID, {e}"
  let disjointOk := f.disjoint.all (validIdx.contains ·) &&
    (f.disjoint.toList.all fun i => f.disjoint.toList.all fun j =>
      i == j || !(f.cores[i]!.turns.any (f.cores[j]!.turns.contains ·)))
  let k := if disjointOk then f.disjoint.size else 0
  let hit := (leastHitting valid [] (valid.size + 1) hittingFuel).map (·.1)
  let h := hit.getD 0
  IO.println s!"    cores: {valid.size} of {f.cores.size} certificates valid; {f.disjoint.size} pairwise disjoint: {disjointOk}"
  -- The upper bound.
  let X := f.shifted.toList
  let upper ← match closesIn rows forms b f.closing (H.filter (!X.contains ·)) with
    | .ok () =>
      if X.all (H.contains ·) then pure (some X.length)
      else IO.println "    X: a turn is not held"; pure none
    | .error e => IO.println s!"    lengths closing H minus X: INVALID, {e}"; pure none
  let lowerB := max k h
  IO.println s!"    τ_P(H) ≥ {k} (disjoint certified cores), {match hit with | some h => s!"≥ {h} (least set meeting the certified cores)" | none => s!"least set meeting the certified cores not searched past {hittingFuel} nodes"}, ≤ {match upper with | some u => s!"{u} (whole lengths of P closing H minus X, |X| = {u})" | none => "? (no lengths checked)"}{match f.tauClaim with | some t => s!"; paper {t}" | none => ""}"
  if let some t := f.tauClaim then
    let agrees := lowerB ≤ t && upper.all (t ≤ ·)
    let exact := upper == some lowerB
    IO.println s!"    τ_P(H) = {t} of the paper: {if !agrees then "DISAGREES" else if exact then "checked exactly" else "within the bounds checked, not exactly"}"
    ok := ok && agrees
  ok := ok && valid.size == f.cores.size && disjointOk && upper.isSome
  -- The sets of the table.
  for st in f.sets do
    let head := s!"    row {st.row} ({st.turns.size} turns {st.turns.toList}):"
    let some cert := st.cert
      | IO.println s!"{head} no certificate with the paper's multipliers"; ok := false; continue
    match checkCert R n cols forms b cert with
    | .error e => IO.println s!"{head} certificate INVALID, {e}"; ok := false
    | .ok c =>
      let bounded := (List.range n).filter fun a => c[a]! > 0 && b[a]! ≥ 1
      let same := match st.multipliers with
        | some m => if m == cert.u then "the paper's" else "NOT the paper's"
        | none => match st.stated with
          | some s => s!"paper: {s}"
          | none => s!"way {st.first} of the row, up to sign"
      let boundedOk := bounded == st.bounded.toList
      let mut closed := 0
      for (t, ℓ) in st.irreducible do
        if let some ℓ := ℓ then
          if (closesIn rows forms b ℓ ((st.turns.toList.filter (· != t)))).toBool then closed := closed + 1
      IO.println s!"{head} valid; u = {cert.u.toList.map showQ} ({same}); bounded arcs {bounded}{if boundedOk then "" else s!", paper {st.bounded.toList}: DISAGREES"}; closable without each turn: {closed} of {st.turns.size} (paper: {st.exact})"
      ok := ok && boundedOk && (st.multipliers.all (· == cert.u))
  return ok

/-! ### The Fox–Burgers formula: holonomy of the words against the shift

The words of `develop.py` are read in the free group on the cone points, the letter
`x_i` mapped to the rotation by `R^{4 - k}` about the developed place `p_i` of the cone
point (`k` its valence, `R = giUnit`), over `A = V = ℤ[i]`, and `Fox.hol` gives their
holonomy. Each jump across the cut tree must be the motion of the chart; each turn's
holonomy a translation, by `α e₁ - shift e₂` turned by the chart of its first patch:
`e₁` the walking direction, `e₂` the direction of increasing offsets on the first side,
`shift` the library's `shift` at the whole lengths and `α` the length of the turn there,
the sum of the widths of the patches crossed (Section "Flat metric and holonomy":
`ĥ(γ̄)` is the inverse of `T_γ(ℓ) = -α e₁ + shift e₂`). The same holonomy is also
compared with `Turn.development` of `Holonomy.lean`, the composite of the library's
changes of chart along the path: `hol = R^k ∘ (development)⁻¹ ∘ R^{-k}`, rotation 1
and translation `-R^k · (development).trans`. -/

def gauss (x y : Int) : GaussianInt := ⟨x, y⟩

/-- `R^k` in `ℤ[i]`. -/
def quarter (k : Int) : GaussianInt := ((giUnit ^ k : GaussianIntˣ) : GaussianInt)

def showG (z : GaussianInt) : String := s!"({z.re}, {z.im})"

/-- `⟨f, z⟩`, the plane read as `ℤ[i]`. -/
def dot (f z : GaussianInt) : Int := f.re * z.re + f.im * z.im

/-- The direction of increasing offsets of a side: `+u` on the pair `0`, `+v` on `1`. -/
def acrossDir (σ : Side (Fin P)) : GaussianInt := if σ.2.1 = 0 then 1 else gi

/-- The walking direction from a side into its patch: `+v` from `v = 0`, `-v` from
`v = h`, `+u` from `u = 0`, `-u` from `u = w`. -/
def inward (σ : Side (Fin P)) : GaussianInt :=
  if σ.2.1 = 0 then (if σ.2.2 = 0 then gi else -gi) else (if σ.2.2 = 0 then 1 else -1)

/-- The length of a turn at the lengths `ℓ`: across each patch from a side to the
opposite one, the length of the sides of the other pair. -/
def along (ℓ : Fin n → ℚ) (start : Side (Fin P)) (path : List (Fin n)) : ℚ :=
  (path.foldl (fun (acc : Side (Fin P) × ℚ) a =>
    (T.other a acc.1.opp, acc.2 + ((T.arcs (acc.1.1, acc.1.2.1 + 1, 0)).map ℓ).sum)) (start, 0)).2

def checkFox (whole : Fin n → ℚ) (forms : Array (Option (Array Int)))
    (paths : Array (Option (Side (Fin P) × List (Fin n))))
    (devs : Array (Option (Motion GaussianInt GaussianInt))) (fox : RawFox) : IO Bool := do
  IO.println s!"  Fox–Burgers: {fox.cones.size} cone points, convention: {fox.convention}"
  let u : Nat → GaussianIntˣ := fun i => giUnit ^ (4 - fox.cones[i]!.1)
  let p : Nat → GaussianInt := fun i => gauss fox.cones[i]!.2.1 fox.cones[i]!.2.2
  let mut jumps : Count := {}
  let mut letters := 0
  for (w, k, x, y) in fox.jumps do
    let m := hol u p w
    jumps := jumps.add (m.lin == quarter k && m.trans == gauss x y) fun _ =>
      s!"jump: hol {showG m.lin}, {showG m.trans}, chart R^{k}, ({x}, {y})"
  jumps.show "jumps across the cut tree: hol of the word = the change of chart"
  let mut rotation : Count := {}
  let mut acrossC : Count := {}
  let mut alongC : Count := {}
  let mut opposite := 0
  let mut whole_ : Count := {}
  let mut foxSum : Count := {}
  let mut inverse : Count := {}
  for i in [0:fox.turns.size] do
    let (w, chart) := fox.turns[i]!
    let some (start, path) := paths[i]?.join | continue
    let some c := forms[i]?.join | continue
    letters := letters + w.length
    let s := evalForm (fun a => c[a.val]!) whole
    let α := along T whole start path
    if s.den != 1 || α.den != 1 then
      IO.println s!"    turn {i}: shift {s} or length {α} not whole"
      continue
    let m := hol u p w
    let M := quarter chart
    let e₁ := M * inward start
    let e₂ := M * acrossDir start
    if let some d := devs[i]?.join then
      inverse := inverse.add (m.lin == 1 && m.trans == -(M * d.trans)) fun _ =>
        s!"turn {i}: hol {showG m.lin}, {showG m.trans}; development {showG d.lin}, {showG d.trans}; R^{chart}"
    rotation := rotation.add (m.lin == 1) fun _ => s!"turn {i}: rotation {showG m.lin}"
    acrossC := acrossC.add (dot e₂ m.trans == -s.num) fun _ =>
      s!"turn {i}: ⟨e₂, τ⟩ = {dot e₂ m.trans}, shift {s}"
    alongC := alongC.add (dot e₁ m.trans == α.num) fun _ =>
      s!"turn {i}: ⟨e₁, τ⟩ = {dot e₁ m.trans}, length {α}"
    if dot e₁ m.trans == -α.num then opposite := opposite + 1
    whole_ := whole_.add (m.trans == (α.num : GaussianInt) * e₁ - (s.num : GaussianInt) * e₂) fun _ =>
      s!"turn {i}: τ {showG m.trans}"
    -- Proposition "Fox–Burgers formula", the sum over the cone points of the word
    -- (`fox` vanishes at the others, `fox_eq_zero_of_forall_ne`).
    let sum := (w.map (·.1)).eraseDups.foldl
      (fun t j => t + TurnsOfATMesh.fox u j w • (p j - ((u j : GaussianIntˣ) : GaussianInt) • p j)) 0
    foxSum := foxSum.add (sum == m.trans) fun _ => s!"turn {i}: Fox–Burgers sum {showG sum}, τ {showG m.trans}"
  IO.println s!"    {letters} letters in the words of the turns"
  rotation.show "turns: hol of the word has rotation 1"
  acrossC.show "turns: ⟨e₂, τ⟩ = -shift (shift: Turn.shift at the whole lengths)"
  alongC.show "turns: ⟨e₁, τ⟩ = α (the length of the turn)"
  IO.println s!"    (⟨e₁, τ⟩ = -α for {opposite})"
  whole_.show "turns: τ = α e₁ - shift e₂"
  foxSum.show "turns: τ = Σᵢ ρ(∂w/∂xᵢ) (1 - Rᵢ) pᵢ (Fox–Burgers sum, `fox`)"
  inverse.show "turns: holonomy of the word = inverse of the development, turned by the chart (hol = 1, -R^k · Turn.development.trans)"
  return jumps.ok && rotation.ok && acrossC.ok && alongC.ok && whole_.ok && foxSum.ok && inverse.ok

/-! ### The check of one export -/

/-! ### The quad mesh at the whole lengths (`Grid.lean`), its curves and regions

The library defines the fine grid (points, gluings, faces, the curve of one round
of a walk) on the `TMesh`; the program computes from these lists what a theorem
would state about them: the vertices (the classes of the gluings, by union-find),
the Euler characteristic of the mesh, the valence of every vertex of the quad
graph, and for every round of every station its regions, their Euler
characteristics and the cone points inside; then Lemma "Gauss–Bonnet for turns"
on each region of a simple curve: `∑ (4 - k) = 4 χ`. -/

/-- The root of `i` in a forest where every parent index is at most its child's. -/
partial def rootOf (par : Array Nat) (i : Nat) : Nat :=
  let p := par[i]!
  if p == i then i else rootOf par p

/-- Unites the classes of `i` and `j`, the larger root pointing to the smaller. -/
def unite (par : Array Nat) (i j : Nat) : Array Nat :=
  let ri := rootOf par i
  let rj := rootOf par j
  if ri == rj then par else par.set! (max ri rj) (min ri rj)

/-- The representative of every index at once: parents never exceed their children. -/
def representatives (par : Array Nat) : Array Nat := Id.run do
  let mut rep := Array.replicate par.size 0
  for i in [0:par.size] do
    let p := par[i]!
    rep := rep.set! i (if p == i then i else rep[p]!)
  return rep

/-- The midpoint of a fine edge, in the coordinates doubled once more
(`GridEuler.midpoint`): an edge is a class of midpoints, the edges along an arc
being glued in pairs. Two fine edges may join the same two vertices (the square
torus of one patch at ℓ = 1 has 8 edges on 4 pairs), so a pair of vertices is
not a key. -/
def mid (p q : Point (Fin P)) : Point (Fin P) := ⟨p.patch, p.x + q.x, p.y + q.y⟩

/-- The two points are the ends of one fine edge of a rectangle. -/
def adjacent (p q : Point (Fin P)) : Bool :=
  p.patch == q.patch && (p.x == q.x && (p.y + 1 == q.y || q.y + 1 == p.y) ||
    p.y == q.y && (p.x + 1 == q.x || q.x + 1 == p.x))

structure Region where
  faces : Nat
  euler : Int
  /-- The cone points strictly inside, by valence. -/
  cones : List Nat

/-- The regions the closed curves cut the fine grid into, with their Euler
characteristics and the cone points strictly inside them. `faceEdgesK` lists
the edge classes of every face and `faceCorners` its corners as representatives,
`facesAtEdge` the faces of every edge class, `valence` the number of fine faces
at every representative (the valence of a vertex of the quad graph) and `coarse`
whether a representative is a vertex of the quad graph. A curve is its vertices
with, at each, the class of the edge to the next. -/
def regionsOf (faceEdgesK : Array (Array Nat)) (faceCorners : Array (Array Nat))
    (facesAtEdge : Std.HashMap Nat (List Nat))
    (valence : Array Nat) (coarse : Array Bool) (curves : List (List (Nat × Nat))) : List Region := Id.run do
  let mut cut : Std.HashSet Nat := ∅
  let mut onCurve : Std.HashSet Nat := ∅
  for curve in curves do
    for (u, e) in curve do
      cut := cut.insert e
      onCurve := onCurve.insert u
  let mut par := Array.range faceEdgesK.size
  for (e, fs) in facesAtEdge.toList do
    if cut.contains e then continue
    match fs with
    | f :: rest => for g in rest do par := unite par f g
    | [] => pure ()
  let rep := representatives par
  -- Per component: faces, vertices, edges, cones.
  let mut facesIn : Std.HashMap Nat Nat := ∅
  let mut verts : Std.HashMap Nat (Std.HashSet Nat) := ∅
  let mut edges : Std.HashMap Nat (Std.HashSet Nat) := ∅
  for f in [0:faceEdgesK.size] do
    let r := rep[f]!
    facesIn := facesIn.insert r (facesIn.getD r 0 + 1)
    let mut vs := verts.getD r ∅
    let mut es := edges.getD r ∅
    for e in faceEdgesK[f]! do
      es := es.insert e
    for v in faceCorners[f]! do
      vs := vs.insert v
    verts := verts.insert r vs
    edges := edges.insert r es
  let mut out : List Region := []
  for (r, nf) in facesIn.toList do
    let vs := verts.getD r ∅
    let es := edges.getD r ∅
    let cones := vs.toList.filter fun v => coarse[v]! && valence[v]! != 4 && !onCurve.contains v
    out := ⟨nf, (vs.size : Int) - es.size + nf, cones.map fun v => valence[v]!⟩ :: out
  return out

/-! ### Proposition "The shift is set by the singularities" (a) on the regions of the curves

On a sphere a simple curve of a round cuts two regions. For the region on each side,
taken with the curve walked so that it lies on the left: a dual spanning tree of its
fine faces, breadth first from the face beside the first edge of the curve, and the
chart of every face, the frame of its patch developed into the frame of the root
across the tree (inside a rectangle the frame is shared; across an arc it is changed
by `crossMotion` of `Holonomy.lean`). The interior edges off the tree are the cuts;
the region is a disc in the combinatorial sense when they form a forest in its vertex
graph, every tree meeting the curve at exactly one vertex and every interior vertex
on a tree. Then the walk round each tree from that vertex, the tree on its right, the
trees in the order of the curve from its start, visits the cone points in the order of
first visit, each at the place the chart of the face it is first reached in gives it:
the rotations `rotAbout (R^{4-k}) p` there compose to the translation `B_D`, which the
proposition gives as `T_γ(ℓ)`, the development of the turn, with `D` on the left of
`γ̄`, and as its inverse with `D` on the right. -/

/-- The fine grid of a sphere as the walk round the cut trees reads it. `cw` maps a
corner `v` of a face and the edge of the face coming into `v` (counter-clockwise
round the face) to the next edge clockwise round `v`, through that face, with the face
and the index of the corner; `edgeEnds` gives the two ends of every edge class. -/
structure Fine (K : Nat) where
  facePatch : Array Nat
  cornerPts : Array (Array (Point (Fin K)))
  faceEdgesK : Array (Array Nat)
  faceCorners : Array (Array Nat)
  facesAtEdge : Std.HashMap Nat (List Nat)
  cw : Std.HashMap (Nat × Nat) (Nat × Nat × Nat)
  edgeEnds : Std.HashMap Nat (Nat × Nat)
  index : Std.HashMap (Point (Fin K)) Nat
  rep : Array Nat
  valence : Array Nat
  coarse : Array Bool

/-- The inverse of a motion whose linear part is a quarter-turn. -/
def invQ (m : Motion GaussianInt GaussianInt) : Motion GaussianInt GaussianInt :=
  ⟨TMesh.conj m.lin, -(TMesh.conj m.lin * m.trans)⟩

/-- Whether `z` in the frame of the patch `Q` is a point of the grid glued to `p`. -/
def sameAs (F : Fine P) (Q : Fin P) (z : GaussianInt) (p : Point (Fin P)) : Bool :=
  z.re ≥ 0 && z.im ≥ 0 &&
    match F.index[(⟨Q, z.re.toNat, z.im.toNat⟩ : Point (Fin P))]?, F.index[p]? with
    | some i, some j => F.rep[i]! == F.rep[j]!
    | _, _ => false

/-- The other end of the edge `e` from its end `v`. -/
def otherEnd (F : Fine P) (v e : Nat) : Nat :=
  let (a, b) := F.edgeEnds.getD e (v, v)
  if a == v then b else a

/-- The arcs crossed by one round of the walk from `p` at the whole lengths, until it
comes back in through `σ`, as `Grid.curve` stops. -/
def roundPath (ℓ : Fin n → ℕ) (σ : Side (Fin P)) :
    Nat → Side (Fin P) × ℚ → List (Fin n) → Option (List (Fin n))
  | 0, _, _ => none
  | fuel + 1, p, acc =>
    match arcAt (TMesh.rat ℓ) (T.arcs p.1.opp) p.2, T.step (TMesh.rat ℓ) p with
    | some a, some q => if q.1 = σ then some (a :: acc).reverse else roundPath ℓ σ fuel q (a :: acc)
    | _, _ => none

/-- A region of a curve: what makes it not a disc (empty for a disc), what went wrong
walking round its cut trees (empty if nothing did), the cone points in the order of
first visit as `(R^{4-k}, p)`, and counts for the report. -/
structure Disc where
  notDisc : String := ""
  walkFailed : String := ""
  word : List (GaussianInt × GaussianInt) := []
  faces : Nat := 0
  cuts : Nat := 0
  trees : Nat := 0
  inside : Nat := 0
  /-- Crossings of the dual tree where the change of chart does not take the corners
  of the edge to the points glued to them. -/
  badCharts : Nat := 0
  /-- The chart of every face of the region, by its index (`none` off the region). -/
  chart : Array (Option (Motion GaussianInt GaussianInt)) := #[]
  /-- The pairs of faces joined by an edge of the dual tree, by their indices, the
  smaller first. -/
  treePairs : Std.HashSet (Nat × Nat) := ∅
  /-- The cuts with an end on the curve. -/
  rootCuts : Nat := 0

/-- The region on the left of the curve `cyc` (its vertices, each with the edge to the
next), or on its right when `left` is false, from the face `root` beside its first
edge: the dual tree, the charts, the check of a disc and the word of first visits.
`dflt` is any point, for the arrays. -/
def discOf (F : Fine P) (ℓ : Fin n → ℕ) (dflt : Point (Fin P)) (cyc : Array (Nat × Nat))
    (left : Bool) (root : Nat) : Disc := Id.run do
  have : Inhabited (Point (Fin P)) := ⟨dflt⟩
  let m := cyc.size
  let curveEdges : Std.HashSet Nat := cyc.foldl (fun s ue => s.insert ue.2) ∅
  let onCurve : Std.HashSet Nat := cyc.foldl (fun s ue => s.insert ue.1) ∅
  let nF := F.faceEdgesK.size
  -- The dual spanning tree, breadth first across the edges off the curve, and the
  -- chart of every face, from the frame of its patch to the frame of the root.
  let mut chart : Array (Option (Motion GaussianInt GaussianInt)) := Array.replicate nF none
  chart := chart.set! root (some Motion.id)
  let mut queue : Array Nat := #[root]
  let mut treeEdges : Std.HashSet Nat := ∅
  let mut treePairs : Std.HashSet (Nat × Nat) := ∅
  let mut badCharts := 0
  let mut i := 0
  while i < queue.size do
    let f := queue[i]!
    i := i + 1
    let Mf := (chart[f]!).getD Motion.id
    let ks := F.faceEdgesK[f]!
    let cs := F.cornerPts[f]!
    for j in [0:4] do
      let e := ks[j]!
      if curveEdges.contains e then continue
      for g in F.facesAtEdge.getD e [] do
        if g == f || (chart[g]!).isSome then continue
        let p := cs[j]!
        let q := cs[(j + 1) % 4]!
        match T.crossEdge ℓ p q with
        | none => badCharts := badCharts + 1
        | some (Q, C) =>
          if Q.val != F.facePatch[g]! || !sameAs F Q (C.apply (fine p)) p ||
              !sameAs F Q (C.apply (fine q)) q then
            badCharts := badCharts + 1
          chart := chart.set! g (some (Motion.comp Mf (invQ C)))
          treeEdges := treeEdges.insert e
          treePairs := treePairs.insert (min f g, max f g)
          queue := queue.push g
  -- The border: every edge of the curve has exactly one face in the region.
  let mut why : Array String := #[]
  let border := curveEdges.toList.filter fun e =>
    ((F.facesAtEdge.getD e []).filter fun g => (chart[g]!).isSome).length != 1
  if !border.isEmpty then why := why.push s!"{border.length} curve edges without one face in the region"
  -- The cuts, the interior vertices and the cone points among them.
  let mut cuts : Std.HashSet Nat := ∅
  let mut inner : Std.HashSet Nat := ∅
  let mut inside := 0
  for f in queue do
    for e in F.faceEdgesK[f]! do
      if !curveEdges.contains e && !treeEdges.contains e then cuts := cuts.insert e
    for v in F.faceCorners[f]! do
      if !onCurve.contains v && !inner.contains v then
        inner := inner.insert v
        if F.coarse[v]! && F.valence[v]! != 4 then inside := inside + 1
  -- The vertex graph of the cuts: each component a tree with one vertex on the curve.
  let mut adj : Std.HashMap Nat (List Nat) := ∅
  for e in cuts.toList do
    let (a, b) := F.edgeEnds.getD e (0, 0)
    adj := adj.insert a (e :: adj.getD a [])
    adj := adj.insert b (e :: adj.getD b [])
  let mut seenV : Std.HashSet Nat := ∅
  let mut trees := 0
  let mut cyclic := 0
  let mut wrongBorder := 0
  for (v₀, _) in adj.toList do
    if seenV.contains v₀ then continue
    seenV := seenV.insert v₀
    trees := trees + 1
    let mut stack : List Nat := [v₀]
    let mut nv := 0
    let mut ends := 0
    let mut nb := 0
    while !stack.isEmpty do
      let v := stack.head!
      stack := stack.tail
      nv := nv + 1
      if onCurve.contains v then nb := nb + 1
      for e in adj.getD v [] do
        ends := ends + 1
        let w := otherEnd F v e
        if !seenV.contains w then
          seenV := seenV.insert w
          stack := w :: stack
    if ends / 2 + 1 != nv then cyclic := cyclic + 1
    if nb != 1 then wrongBorder := wrongBorder + 1
  let uncovered := (inner.toList.filter fun v => !adj.contains v).length
  if cyclic > 0 then why := why.push s!"{cyclic} components of the cuts with a cycle"
  if wrongBorder > 0 then why := why.push s!"{wrongBorder} cut trees not meeting the curve at one vertex"
  if uncovered > 0 then why := why.push s!"{uncovered} interior vertices off the cuts"
  let rootCuts := (cuts.toList.filter fun e =>
    let (a, b) := F.edgeEnds.getD e (0, 0)
    onCurve.contains a || onCurve.contains b).length
  let base : Disc := { faces := queue.size, cuts := cuts.size, trees, inside, badCharts, chart,
                       treePairs, rootCuts }
  if !why.isEmpty then return { base with notDisc := ", ".intercalate why.toList }
  -- The walk round the cut trees: at each vertex of the curve, clockwise from the edge
  -- it came in by to the edge it leaves by, round every tree met there, the trees on
  -- the right; at the start last on the left, first on the right, so that the walk
  -- begins in the root face.
  let stops : Array (Nat × Nat × Nat) :=
    if left then
      ((Array.range (m - 1)).map fun j => (cyc[j + 1]!.1, cyc[j]!.2, cyc[j + 1]!.2)).push
        (cyc[0]!.1, cyc[m - 1]!.2, cyc[0]!.2)
    else
      #[(cyc[0]!.1, cyc[0]!.2, cyc[m - 1]!.2)] ++ (Array.range (m - 1)).map fun j' =>
        let j := m - 1 - j'
        (cyc[j]!.1, cyc[j]!.2, cyc[j - 1]!.2)
  let mut fuel := 16 * nF + 8 * m + 64
  let mut word : Array (GaussianInt × GaussianInt) := #[]
  let mut seen : Std.HashSet Nat := ∅
  let mut walkFailed := ""
  for (b, ein, eout) in stops do
    if !walkFailed.isEmpty then break
    let mut cur := ein
    let mut atEnd := false
    while !atEnd do
      if fuel == 0 then
        walkFailed := "lost at a vertex of the curve"
        break
      fuel := fuel - 1
      match F.cw[(b, cur)]? with
      | none =>
        walkFailed := "no face clockwise at a vertex of the curve"
        break
      | some (e₂, _, _) =>
        if e₂ == eout then atEnd := true
        else if curveEdges.contains e₂ then
          walkFailed := "met the curve before the edge it leaves by"
          break
        else if cuts.contains e₂ then
          -- Round the branch through `e₂`, back to `b` along `e₂`.
          let mut v := otherEnd F b e₂
          let mut arrived := e₂
          while v != b && fuel > 0 do
            if !seen.contains v then
              seen := seen.insert v
              if F.coarse[v]! && F.valence[v]! != 4 then
                match F.cw[(v, arrived)]? with
                | some (_, f, j) =>
                  let at_ := ((chart[f]!).getD Motion.id).apply (fine (F.cornerPts[f]!)[j]!)
                  word := word.push (quarter (4 - (F.valence[v]! : Int)), at_)
                | none => walkFailed := "no face at a cone point"
            -- Clockwise round `v` to the next cut edge.
            let mut c := arrived
            let mut next := false
            while !next && fuel > 0 do
              fuel := fuel - 1
              match F.cw[(v, c)]? with
              | none => fuel := 0
              | some (e₃, _, _) =>
                c := e₃
                next := cuts.contains e₃
            arrived := c
            v := otherEnd F v c
          if fuel == 0 then
            walkFailed := "lost round a cut tree"
            break
          cur := arrived
        else cur := e₂
  if walkFailed.isEmpty && word.size != inside then
    walkFailed := s!"{word.size} cone points visited of {inside} inside"
  return { base with walkFailed, word := word.toList }

/-- The jumps across the cuts of a region (`Region.lean`): what went wrong with its
border (empty if nothing did: the steps are unit steps, each ends where the next
starts, the first face is the root, `borderCrossings` is `some`, its faces chain as
`borderCrossings_chain` states, and every face it meets has a chart), the crossings,
those across the tree with charts that do not agree (`M g ∘ C ≠ M f`), the composite
of the jumps across the cuts, and how the cut crossings, each named by its pair of
faces, nest. -/
structure CutsOf where
  why : String := ""
  crossings : Nat := 0
  treeCrossings : Nat := 0
  treeBad : Nat := 0
  cutCrossings : Nat := 0
  composite : Motion GaussianInt GaussianInt := Motion.id
  holonomy : Option (Motion GaussianInt GaussianInt) := none
  /-- Distinct pairs of faces among the cut crossings. -/
  distinct : Nat := 0
  /-- Every pair crossed exactly once. -/
  once : Bool := false
  /-- Every pair crossed exactly twice, once each way, the word well parenthesized. -/
  nested : Bool := false

/-- The definitions of the sixth brick evaluated on a region of `discOf`, with the
closed curve of `steps` round it (the region on their left) and the face `root` the
tree starts from. -/
def cutsOf (F : Fine P) (ℓ : Fin n → ℕ) (faceIndex : Std.HashMap (Face (Fin P)) Nat) (r : Disc)
    (steps : List (Step (Fin P))) (root : Face (Fin P)) : CutsOf := Id.run do
  let M : Face (Fin P) → Motion GaussianInt GaussianInt := fun f =>
    match faceIndex[f]? with
    | some i => (r.chart[i]?.join).getD Motion.id
    | none => Motion.id
  let charted : Face (Fin P) → Bool := fun f =>
    match faceIndex[f]? with
    | some i => (r.chart[i]?.join).isSome
    | none => false
  let tree : Face (Fin P) → Face (Fin P) → Bool := fun f g =>
    match faceIndex[f]?, faceIndex[g]? with
    | some i, some j => r.treePairs.contains (min i j, max i j)
    | _, _ => false
  let same : Motion GaussianInt GaussianInt → Motion GaussianInt GaussianInt → Bool :=
    fun A B => A.lin == B.lin && A.trans == B.trans
  let some s := steps.head? | return { why := "no steps" }
  have : Inhabited (Step (Fin P)) := ⟨s⟩
  let mut why : Array String := #[]
  if !(steps.all fun t => t.2 == 1 || t.2 == gi || t.2 == -1 || t.2 == -gi) then
    why := why.push "a step not a unit step"
  -- The curve closes: each step ends at the vertex the next starts at.
  let arr := steps.toArray
  let mut gaps := 0
  for i in [0:arr.size] do
    let (v, d) := arr[i]!
    let e := pointAt v.patch (fine v + d)
    match F.index[e]?, F.index[(arr[(i + 1) % arr.size]!).1]? with
    | some a, some b => if F.rep[a]! != F.rep[b]! then gaps := gaps + 1
    | _, _ => gaps := gaps + 1
  if gaps > 0 then why := why.push s!"{gaps} steps not ending where the next starts"
  let some s' := T.normalize ℓ s | return { why := ", ".intercalate (why.push "no frame for the first step").toList }
  let f₀ := faceAt s'
  if f₀ != root then why := why.push "the face on the left of the first step is not the root"
  if !charted f₀ || !same (M f₀) Motion.id then why := why.push "the root face without the identity chart"
  let some cs := T.borderCrossings ℓ 4 steps
    | return { why := ", ".intercalate (why.push "borderCrossings is none").toList }
  if !(f₀ :: cs.map (·.2.2) == cs.map (·.1) ++ [f₀]) then why := why.push "the faces do not chain"
  let outside := (cs.filter fun c => !charted c.1 || !charted c.2.2).length
  if outside > 0 then why := why.push s!"{outside} crossings meet a face off the region"
  let treeCs := cs.filter fun c => tree c.1 c.2.2
  let treeBad := (treeCs.filter fun c => !same (Motion.comp (M c.2.2) c.2.1) (M c.1)).length
  let cuts := cutCrossings tree cs
  -- The cut crossings by their pairs of faces, and the way each is crossed.
  let names := cuts.map fun c =>
    let i := faceIndex.getD c.1 0
    let j := faceIndex.getD c.2.2 0
    ((min i j, max i j), decide (i < j))
  let distinct := (Std.HashSet.ofList (names.map (·.1))).size
  let mut stack : List ((Nat × Nat) × Bool) := []
  let mut closed : Std.HashSet (Nat × Nat) := ∅
  let mut nested := true
  for (p, w) in names do
    match stack with
    | (q, w') :: rest =>
      if q == p && w' != w then
        stack := rest
        closed := closed.insert p
        continue
    | [] => pure ()
    if closed.contains p || stack.any (·.1 == p) then nested := false
    stack := (p, w) :: stack
  return { why := ", ".intercalate why.toList, crossings := cs.length, treeCrossings := treeCs.length,
           treeBad, cutCrossings := cuts.length, composite := Motion.composeAll (cuts.map (jump M)),
           holonomy := T.borderHolonomy ℓ 4 steps, distinct, once := distinct == names.length,
           nested := nested && stack.isEmpty }

/-- Proposition "The shift is set by the singularities" (a) on a sphere, for both
regions of every simple curve of a round: the check of a disc, the word of first
visits, and its composite against the development of the curve. The curve of the
round from the station `k + ½` of `σ` is the curve of the turn whose path is the arcs
that round crosses; its development `T = Turn.development` (`Holonomy.lean`) is a
translation in the frame of the patch of `σ`, in quads. The root face lies in that
patch, beside the first edge of the curve, so its chart is the identity: the composite
must be `x ↦ x + 2 T` in fine coordinates for the region on the left of the curve and
`x ↦ x - 2 T` for the region on its right (`B_D = T_γ` with `D` on the left, its
inverse with `D` on the right). Also the Burgers sum, and the holonomy of the word
`develop.py` gave the same turn (`ĥ(γ̄)`, the inverse of `T`), through the chart `R^k`
of its first patch. -/
def checkDiscs (ℓ : Fin n → ℕ) (fcs : Array (Face (Fin P)))
    (faceEdgesK faceCorners : Array (Array Nat)) (facesAtEdge : Std.HashMap Nat (List Nat))
    (index : Std.HashMap (Point (Fin P)) Nat) (rep valence : Array Nat) (coarse : Array Bool)
    (curves : List (List (Nat × Nat))) (curveSt : List (Side (Fin P) × Nat))
    (paths : Array (Option (Side (Fin P) × List (Fin n)))) (fox : Option RawFox) : IO Bool := do
  let mut cw : Std.HashMap (Nat × Nat) (Nat × Nat × Nat) := ∅
  let mut edgeEnds : Std.HashMap Nat (Nat × Nat) := ∅
  for f in [0:faceEdgesK.size] do
    let ks := faceEdgesK[f]!
    let cs := faceCorners[f]!
    for i in [0:4] do
      cw := cw.insert (cs[i]!, ks[(i + 3) % 4]!) (ks[i]!, f, i)
      edgeEnds := edgeEnds.insert ks[i]! (cs[i]!, cs[(i + 1) % 4]!)
  let F : Fine P := { facePatch := fcs.map (·.patch.val), cornerPts := fcs.map fun f => (corners f).toArray,
                      faceEdgesK, faceCorners, facesAtEdge, cw, edgeEnds, index, rep, valence, coarse }
  let mut faceIndex : Std.HashMap (Face (Fin P)) Nat := ∅
  for face in fcs do faceIndex := faceIndex.insert face faceIndex.size
  -- The turns of the export by their first side and path, for the words of `develop.py`.
  let mut byPath : Std.HashMap (Nat × List Nat) Nat := ∅
  for i in [0:paths.size] do
    if let some (σ, l) := paths[i]?.join then byPath := byPath.insert (sideIndex σ, l.map Fin.val) i
  let fu : Nat → GaussianIntˣ := fun i => match fox with
    | some fx => giUnit ^ (4 - fx.cones[i]!.1)
    | none => 1
  let fp : Nat → GaussianInt := fun i => match fox with
    | some fx => gauss fx.cones[i]!.2.1 fx.cones[i]!.2.2
    | none => 0
  let fuel := T.stations (List.finRange P) ℓ + 1
  let mut tested := 0
  let mut skipped := 0
  let mut noTurn := 0
  let mut rootBad := 0
  let mut ofExport := 0
  let mut letters := 0
  let mut cutsN := 0
  let mut treesN := 0
  let mut badCharts := 0
  let mut discC : Count := {}
  let mut wordC : Count := {}
  let mut compC : Count := {}
  let mut burgC : Count := {}
  let mut foxC : Count := {}
  let mut crossC : Count := {}
  let mut treeC : Count := {}
  let mut jumpC : Count := {}
  let mut crossingsN := 0
  let mut treeCrossingsN := 0
  let mut cutCrossingsN := 0
  let mut onceN := 0
  let mut nestedN := 0
  let mut allCutsN := 0
  let mut rootCutsN := 0
  let mut distinctN := 0
  let mut cutsOfDiscN := 0
  let mut rootCutsOfDiscN := 0
  for (c, (σ, k)) in curves.zip curveSt do
    let cyc := c.toArray
    if (Std.HashSet.ofList (c.map (·.1))).size != c.length then continue
    if cyc.isEmpty || cyc.any (fun ue => !facesAtEdge.contains ue.2) then
      skipped := skipped + 1
      continue
    let some path := roundPath T ℓ σ fuel (TMesh.station σ k) []
      | noTurn := noTurn + 1
        continue
    let some γ := mkTurn T σ path
      | noTurn := noTurn + 1
        continue
    let D := γ.development ℓ
    let known := byPath[(sideIndex σ, path.map Fin.val)]?
    if known.isSome then ofExport := ofExport + 1
    let x₀ := T.sidePoint ℓ σ (2 * k + 1)
    let d := TMesh.inward σ
    -- The curve as steps (`Border.lean`), the region on their left; walked back, the
    -- region on its right: `(v + d, -d)` from the first step back round.
    let stepsLeft := ((T.roundSteps (List.finRange P) ℓ σ k).map (·.1)).getD []
    let back : Step (Fin P) → Step (Fin P) := fun (v, e) => (pointAt v.patch (fine v + e), -e)
    let stepsRight := match stepsLeft with
      | [] => []
      | s :: ss => back s :: ss.reverse.map back
    for left in [true, false] do
      -- The face beside the first edge of the curve, from `x₀` into the patch of `σ`.
      let nrm := if left then gi * d else -(gi * d)
      let x := (x₀.x : Int) + min (min 0 d.re) (min nrm.re (d.re + nrm.re))
      let y := (x₀.y : Int) + min (min 0 d.im) (min nrm.im (d.im + nrm.im))
      let rootFace : Face (Fin P) := ⟨σ.1, x.toNat, y.toNat⟩
      let some root := faceIndex[rootFace]?
        | rootBad := rootBad + 1
          continue
      if !(faceEdgesK[root]!.contains cyc[0]!.2) then
        rootBad := rootBad + 1
        continue
      tested := tested + 1
      let r := discOf T F ℓ x₀ cyc left root
      let at_ := s!"station {k} of side {sideIndex σ}, {if left then "left" else "right"}"
      badCharts := badCharts + r.badCharts
      -- The jumps across the cuts (`Region.lean`).
      let u := cutsOf T F ℓ faceIndex r (if left then stepsLeft else stepsRight) rootFace
      crossC := crossC.add u.why.isEmpty fun _ => s!"{at_}: {u.why}"
      crossingsN := crossingsN + u.crossings
      treeCrossingsN := treeCrossingsN + u.treeCrossings
      cutCrossingsN := cutCrossingsN + u.cutCrossings
      treeC := treeC.add (u.why.isEmpty && u.treeBad == 0) fun _ =>
        s!"{at_}: {u.treeBad} of {u.treeCrossings} crossings of the tree with charts that do not agree"
      let fineD : GaussianInt := (if left then 2 else -2) * D.trans
      let holB := u.holonomy.getD ⟨0, 0⟩
      jumpC := jumpC.add (u.why.isEmpty && D.lin == 1 && u.composite.lin == 1 &&
          u.composite.trans == fineD && holB.lin == 1 && holB.trans == fineD) fun _ =>
        s!"{at_}: jumps across the cuts {showG u.composite.lin}, {showG u.composite.trans}; holonomy of the border {showG holB.lin}, {showG holB.trans}; development {showG D.lin}, {showG D.trans}"
      if u.why.isEmpty then
        if u.once then onceN := onceN + 1
        if u.nested then nestedN := nestedN + 1
        if u.distinct == r.cuts then allCutsN := allCutsN + 1
        if u.distinct == r.rootCuts then rootCutsN := rootCutsN + 1
        distinctN := distinctN + u.distinct
        cutsOfDiscN := cutsOfDiscN + r.cuts
        rootCutsOfDiscN := rootCutsOfDiscN + r.rootCuts
      discC := discC.add r.notDisc.isEmpty fun _ =>
        s!"{at_}: {r.faces} faces, {r.cuts} cuts, {r.trees} trees: {r.notDisc}"
      if !r.notDisc.isEmpty then continue
      cutsN := cutsN + r.cuts
      treesN := treesN + r.trees
      wordC := wordC.add r.walkFailed.isEmpty fun _ => s!"{at_}: {r.walkFailed}"
      if !r.walkFailed.isEmpty then continue
      letters := letters + r.word.length
      let comp := Motion.composeAll (Motion.rotations r.word)
      -- `B_D` of the proposition in fine units: `T` with the region on the left of the
      -- curve, `-T` with the region on its right.
      let developed : GaussianInt := (if left then 2 else -2) * D.trans
      compC := compC.add (D.lin == 1 && comp.lin == 1 && comp.trans == developed) fun _ =>
        s!"{at_}: composite {showG comp.lin}, {showG comp.trans}; development {showG D.lin}, {showG D.trans}; {r.word.length} cone points {r.word.map fun (u, p) => (showG u, showG p)}"
      burgC := burgC.add (Motion.burgers r.word == comp.trans) fun _ =>
        s!"{at_}: Burgers sum {showG (Motion.burgers r.word)}, composite {showG comp.trans}"
      if let (some i, some fx) := (known, fox) then
        if h : i < fx.turns.size then
          let (w, chart) := fx.turns[i]
          let mh := hol fu fp w
          foxC := foxC.add (mh.lin == 1 &&
              quarter chart * comp.trans == (if left then -2 else 2) * mh.trans) fun _ =>
            s!"{at_}, turn {i}: composite {showG comp.lin}, {showG comp.trans}; develop.py {showG mh.lin}, {showG mh.trans}, chart R^{chart}"
  IO.println s!"  regions of the simple curves on the sphere, both sides: {tested} (rounds of a turn of the export: {ofExport}; curves skipped {skipped}, without a turn {noTurn}; without a root face {rootBad})"
  discC.show "regions: the cuts (interior edges off a dual spanning tree) form a forest, each tree meeting the curve at one vertex, every interior vertex on it (a disc)"
  IO.println s!"    {cutsN} cut edges in {treesN} trees; changes of chart across the dual trees not taking an edge to the points glued to it: {badCharts}"
  wordC.show "regions: the walk round the cut trees visits every cone point inside (the word in the order of first visit)"
  IO.println s!"    {letters} letters in the words of the regions"
  compC.show "regions: composite of the rotations in first-visit order, region on the left, = Turn.development (Prop. 'The shift is set by the singularities' (a): B_D = T_γ with D on its left, the inverse with D on its right; fine units, 2 T)"
  burgC.show "regions: Burgers sum ∑ Qᵢ (1 - Rᵢ) pᵢ = translation of the composite (Motion.burgers)"
  if fox.isSome then
    foxC.show "regions: composite = inverse of the holonomy of the word of develop.py for the same turn, through the chart R^k of its first patch"
  crossC.show "cuts: the border of the region crossed face by face (borderCrossings ℓ 4 some, unit steps, closed, from the root face, the faces chain as borderCrossings_chain states, every face in the region)"
  IO.println s!"    {crossingsN} crossings: {treeCrossingsN} across the dual tree, {cutCrossingsN} across the cuts"
  treeC.show "cuts: across every crossing of the dual tree the charts agree, M g ∘ C = M f (the hypothesis of borderHolonomy_cuts)"
  jumpC.show "cuts: composite of the jumps across the cuts in border order = holonomy of the border = development (2 T on the left, -2 T on the right; borderHolonomy_cuts with borderHolonomy_round)"
  IO.println s!"    for the next brick, on the regions crossed: cut crossings with a distinct pair of faces each (each cut edge crossed once) {onceN}; nested words (each crossed twice, once each way) {nestedN}; distinct pairs crossed {distinctN} of {cutsOfDiscN} cuts, {rootCutsOfDiscN} with an end on the curve; regions where the pairs crossed are all the cuts {allCutsN}, the cuts with an end on the curve {rootCutsN}"
  return discC.ok && wordC.ok && compC.ok && burgC.ok && foxC.ok && badCharts == 0 && rootBad == 0 &&
    crossC.ok && treeC.ok && jumpC.ok

/-! ### The chart along the border of a round (`Border.lean`)

The definitions of the fifth brick of DT-134 evaluated on the quad mesh at the whole
lengths, and its theorems checked on them: the steps of every round against its points
(`roundSteps_round`); the holonomy of the border of every round, the chart continued
from face to face through the region on its left, against the development of its turn
(`borderHolonomy_round`), counted by where the closing segment along the first side
`σ` runs (none, a ring; with the region inside the rectangle of `σ`; outside, every
node strictly inside `σ` met by four faces, the hypothesis of the theorem; outside,
some node not, which the theorem does not cover); at every vertex of the quad graph,
the walk round it (`valenceAround`) against the valence of the union-find,
`holonomyAround` against `R^{4-k}` about it (`holonomyAround_eq`), and every point the
walk meets in its class (`aroundStep_same`). -/

/-- The checks of the fifth brick: see the section above. `pts`, `index`, `rep`,
`valence` and `coarse` are those of `checkGrid`. -/
def checkBorder (ℓ : Fin n → ℕ) (pts : Array (Point (Fin P)))
    (index : Std.HashMap (Point (Fin P)) Nat) (rep valence : Array Nat) (coarse : Array Bool) :
    IO Bool := do
  let patches := List.finRange P
  -- More steps than faces round any vertex: at most four corners of each corner and
  -- side of each patch meet at a vertex.
  let fuel := 16 * P + 16
  -- The vertices of the quad graph, by their representatives.
  let mut valC : Count := {}
  let mut holC : Count := {}
  let mut sameC : Count := {}
  for (v, i) in pts.zipIdx do
    if rep[i]! != i || !coarse[i]! then continue
    let z := TMesh.fine v
    -- A direction whose face on the right lies in the rectangle of `v`.
    let dirs : List GaussianInt := [1, gi, -1, -gi]
    match dirs.find? (fun d => (square z d (-gi)).all (T.inRect ℓ v.patch) &&
        T.inRect ℓ v.patch (z + d)) with
    | none => valC := valC.add false fun _ => s!"vertex {i}: no face of its rectangle at it"
    | some d =>
      let k := T.valenceAround ℓ fuel v d
      valC := valC.add (k == some valence[i]!) fun _ =>
        s!"vertex {i} at {repr v}: valenceAround {k}, union-find {valence[i]!}"
      match T.holonomyAround ℓ fuel v d, k with
      | some H, some k =>
        let R := Motion.rotAbout (TurnsOfATMesh.quarter (4 - (k : Int))) z
        holC := holC.add (H.lin == R.lin && H.trans == R.trans) fun _ =>
          s!"vertex {i} at {repr v}, {k} faces: holonomy {showG H.lin}, {showG H.trans}"
      | _, _ => holC := holC.add false fun _ => s!"vertex {i} at {repr v}: no walk round it"
      -- The walk round the vertex, point by point.
      let mut w := v
      let mut e := d
      let mut steps := 0
      let mut done := false
      while !done && steps < fuel do
        match T.aroundStep ℓ gi w e with
        | none =>
          sameC := sameC.add false fun _ => s!"vertex {i} at {repr v}: the walk stops at {repr w}"
          done := true
        | some (w', e', _) =>
          steps := steps + 1
          let same := match index[w']? with
            | some j => rep[j]! == i
            | none => false
          sameC := sameC.add same fun _ => s!"vertex {i} at {repr v}: the walk meets {repr w'}"
          w := w'
          e := e'
          if w == v && e == d then done := true
  -- One round from every station.
  let mut stepsC : Count := {}
  let mut ringC : Count := {}
  let mut inC : Count := {}
  let mut outC : Count := {}
  let mut irrC : Count := {}
  for Pi in patches do
    for j in (List.finRange 2) do
      for s in (List.finRange 2) do
        let σ : Side (Fin P) := (Pi, j, s)
        -- The hypothesis of `borderHolonomy_round`: every node strictly inside `σ` is met
        -- by four faces.
        let regular := (List.range (T.sideLen ℓ σ)).all fun m =>
          m == 0 || T.valenceAround ℓ fuel (T.sidePoint ℓ σ (2 * m)) (TMesh.inward σ) == some 4
        for k in [0:T.sideLen ℓ σ] do
          let at_ := s!"station {k} of side {sideIndex σ}"
          let rs := T.roundSteps patches ℓ σ k
          stepsC := stepsC.add (rs.map (·.1.map Prod.fst) == (T.round patches ℓ σ k).map List.dropLast)
            fun _ => s!"{at_}: the steps do not start at the points of the round"
          let some (steps, path) := rs | continue
          let D := fineMotion (Motion.composeAll (T.crossMotions ℓ σ path))
          let ok := match T.borderHolonomy ℓ 4 steps with
            | some H => H.lin == D.lin && H.trans == D.trans
            | none => false
          let what := fun (_ : Unit) =>
            s!"{at_}: holonomy of the border {(T.borderHolonomy ℓ 4 steps).map fun H => (showG H.lin, showG H.trans)}, development {showG D.lin}, {showG D.trans}"
          -- The station the round comes back to, `kq + ½`.
          let x := (Motion.composeAll (T.crossingsAt (TMesh.rat ℓ) σ path)).apply ((k : ℚ) + 1 / 2)
          let kq := ((2 * x - 1) / 2).floor.toNat
          if kq == k then ringC := ringC.add ok what
          else
            let ε : Int := if kq ≤ k then 1 else -1
            if ε == σ.orient then inC := inC.add ok what
            else if regular then outC := outC.add ok what
            else irrC := irrC.add ok what
  stepsC.show "border: the steps of a round start at its points (roundSteps_round)"
  ringC.show "border: holonomy of the border of a round = its development (borderHolonomy_round), rings"
  inC.show "border: the same, closing segment with the region inside the rectangle of σ"
  outC.show "border: the same, closing segment with the region outside, every node of σ met by four faces"
  irrC.show "border: the same, closing segment outside, a node of σ not met by four faces (not covered by the theorem)"
  valC.show "border: the walk round each vertex of the quad graph meets its valence (valenceAround)"
  holC.show "border: holonomy round each vertex of the quad graph = R^(4-k) about it (holonomyAround_eq)"
  sameC.show "border: every point the walk round a vertex meets is that vertex (aroundStep_same)"
  return stepsC.ok && ringC.ok && inC.ok && outC.ok && irrC.ok && valC.ok && holC.ok && sameC.ok

/-- The checks of the quad mesh at the whole lengths: see the section above.
`lengthShift` holds `(index, γ.length, γ.shift)` at the whole lengths for the turns
with a half-integer in their corridor, for the parity. -/
def checkGrid (whole : Array ℚ) (lengthShift : Array (Nat × ℚ × ℚ))
    (paths : Array (Option (Side (Fin P) × List (Fin n)))) (fox : Option RawFox) : IO Bool := do
  if whole.any (fun x => x.den ≠ 1 || x ≤ 0) then
    IO.println "  grid: an arc of length zero at the whole lengths, the quad mesh is not built"
    return true
  let ℓ : Fin n → ℕ := fun a => (whole[a.val]!).num.toNat
  let patches := List.finRange P
  let allArcs := List.finRange n
  -- The points and their representatives under the gluings.
  let pts := (T.points patches ℓ).toArray
  let mut index : Std.HashMap (Point (Fin P)) Nat := ∅
  let mut count := 0
  for p in pts do
    index := index.insert p count
    count := count + 1
  let mut par := Array.range pts.size
  let mut missing := 0
  for (p, q) in T.gluings allArcs ℓ do
    match index[p]?, index[q]? with
    | some i, some j => par := unite par i j
    | _, _ => missing := missing + 1
  let rep := representatives par
  if missing > 0 then
    IO.println s!"  grid: FAILED, {missing} gluing pairs name points off the grid"
    return false
  -- The edges as classes of midpoints: the midpoints of the fine edges of the
  -- faces, those of consecutive gluing pairs of an arc being one edge.
  let fcs := (T.faces patches ℓ).toArray
  let mut midIndex : Std.HashMap (Point (Fin P)) Nat := ∅
  for face in fcs do
    for (p, q) in faceEdges face do
      let m := mid p q
      if !midIndex.contains m then midIndex := midIndex.insert m midIndex.size
  let mut midPar := Array.range midIndex.size
  let mut missingEdges := 0
  for a in allArcs do
    for t in [0:2 * ℓ a] do
      let (p₀, q₀) := T.glued ℓ a t
      let (p₁, q₁) := T.glued ℓ a (t + 1)
      match midIndex[mid p₀ p₁]?, midIndex[mid q₀ q₁]? with
      | some i, some j => midPar := unite midPar i j
      | _, _ => missingEdges := missingEdges + 1
  let midRep := representatives midPar
  if missingEdges > 0 then
    IO.println s!"  grid: FAILED, {missingEdges} gluing pairs of edges name edges off the grid"
    return false
  -- The class of the fine edge from `p` to `q`, `none` unless they are adjacent in a rectangle.
  let edgeOf (p q : Point (Fin P)) : Option Nat :=
    if adjacent p q then (midIndex[mid p q]?).map fun i => midRep[i]! else none
  -- The faces, their edges as classes and corners as representatives, the valences.
  let mut faceEdgesK : Array (Array Nat) := Array.replicate fcs.size #[]
  let mut faceCorners : Array (Array Nat) := Array.replicate fcs.size #[]
  let mut facesAtEdge : Std.HashMap Nat (List Nat) := ∅
  let mut valence := Array.replicate pts.size 0
  let mut allEdges : Std.HashSet Nat := ∅
  let mut vertices : Std.HashSet Nat := ∅
  let mut f := 0
  for face in fcs do
    let mut ks : Array Nat := #[]
    let mut cs : Array Nat := #[]
    for (p, q) in faceEdges face do
      let u := rep[index.getD p 0]!
      let k := (edgeOf p q).getD 0
      ks := ks.push k
      cs := cs.push u
      facesAtEdge := facesAtEdge.insert k (f :: facesAtEdge.getD k [])
      allEdges := allEdges.insert k
      vertices := vertices.insert u
      valence := valence.set! u (valence[u]! + 1)
    faceEdgesK := faceEdgesK.set! f ks
    faceCorners := faceCorners.set! f cs
    f := f + 1
  let coarse : Array Bool := Id.run do
    let mut c := Array.replicate pts.size false
    let mut i := 0
    for p in pts do
      if p.x % 2 == 0 && p.y % 2 == 0 then c := c.set! (rep[i]!) true
      i := i + 1
    return c
  let euler : Int := (vertices.size : Int) - allEdges.size + fcs.size
  let mut byValence : Std.HashMap Nat Nat := ∅
  for v in vertices.toList do
    if coarse[v]! && valence[v]! != 4 then
      byValence := byValence.insert valence[v]! (byValence.getD valence[v]! 0 + 1)
  let index4 := (byValence.toList.map fun (k, c) => ((4 : Int) - k) * c).foldl (· + ·) 0
  IO.println s!"  grid: {vertices.size} vertices, {allEdges.size} edges, {fcs.size} faces, χ = {euler}; cone points by valence {byValence.toList.mergeSort (fun a b => a.1 ≤ b.1)}, ∑ (4 - k) = {index4}"
  let mut ok := index4 == 4 * euler
  if !ok then IO.println "    FAILED: ∑ (4 - k) ≠ 4 χ on the whole mesh"
  -- The 2-colouring (`IsColouring`): the constants `κ` propagated from patch to patch
  -- along the gluing pairs of vertices of the quad graph, `κ_Q = κ_P + c₀ p - c₀ q`.
  let c₀ (p : Point (Fin P)) : ZMod 2 := ((p.x / 2 : ℕ) : ZMod 2) + ((p.y / 2 : ℕ) : ZMod 2)
  let mut links : Array (Array (Nat × ZMod 2)) := Array.replicate P #[]
  for (p, q) in T.gluings allArcs ℓ do
    if p.IsCoarse ∧ q.IsCoarse then
      links := links.modify p.patch.val (·.push (q.patch.val, c₀ p - c₀ q))
      links := links.modify q.patch.val (·.push (p.patch.val, c₀ q - c₀ p))
  let mut kappa : Array (Option (ZMod 2)) := Array.replicate P none
  let mut conflict := false
  for root in [0:P] do
    if kappa[root]!.isSome then continue
    kappa := kappa.set! root (some 0)
    let mut queue : Array Nat := #[root]
    let mut i := 0
    while i < queue.size do
      let r := queue[i]!
      i := i + 1
      let kr := (kappa[r]!).getD 0
      for (s, d) in links[r]! do
        match kappa[s]! with
        | none =>
          kappa := kappa.set! s (some (kr + d))
          queue := queue.push s
        | some ks => if ks != kr + d then conflict := true
  let κ : Fin P → ZMod 2 := fun p => (kappa[p.val]!).getD 0
  let colouring := decide (T.IsColouring allArcs ℓ κ)
  IO.println s!"  bipartite: {!conflict} (IsColouring decided {colouring}), χ = {euler}"
  if colouring == conflict then
    IO.println "    FAILED: the propagation and IsColouring disagree"
    ok := false
  if euler == 2 && !colouring then
    IO.println "    FAILED: a sphere whose quad graph is not bipartite"
    ok := false
  -- The parity of a turn, `α_γ ≡ shift (mod 2)`, on a bipartite quad graph.
  let mut parity : Count := {}
  let mut odd := 0
  for (i, len, s) in lengthShift do
    let even := (len - s).den == 1 && (len - s).num % 2 == 0
    if colouring then
      parity := parity.add even fun _ => s!"turn {i}: length {len}, shift {s}"
    else if !even then odd := odd + 1
  parity.show "parity α ≡ shift (mod 2) at the whole lengths, bipartite instances"
  if !colouring then
    IO.println s!"    quad graph not bipartite, length - shift odd for {odd} of {lengthShift.size} turns (a report)"
  -- One round from every station, each curve once: its vertices as representatives,
  -- each with the class of the edge to the next point. Where the curve leaves a
  -- rectangle (`round` drops the point on the side, glued to the next point), that
  -- edge is from the point to the side, one more step in the direction of the line.
  let mut seen : Std.HashSet (List Nat) := ∅
  let mut curves : List (List (Nat × Nat)) := []
  -- The station of each curve, in the same order.
  let mut curveSt : List (Side (Fin P) × Nat) := []
  let mut offGrid := 0
  let mut rounds := 0
  let mut stopped := 0
  for Pi in patches do
    for j in (List.finRange 2) do
      for s in (List.finRange 2) do
        let σ : Side (Fin P) := (Pi, j, s)
        for k in [0:T.sideLen ℓ σ] do
          match T.round patches ℓ σ k with
          | none => stopped := stopped + 1
          | some pointsOf =>
            rounds := rounds + 1
            have : Inhabited (Point (Fin P)) := ⟨T.sidePoint ℓ σ 0⟩
            let arr := pointsOf.dropLast.toArray
            let m := arr.size
            let mut cyc : Array (Nat × Nat) := #[]
            let mut bad := 0
            for i in [0:m] do
              let p := arr[i]!
              let q := arr[(i + 1) % m]!
              let o := arr[(i + m - 1) % m]!
              let u := rep[index.getD p 0]!
              let e : Option Nat := match edgeOf p q with
                | some e => some e
                | none =>
                  if adjacent o p then
                    let s : Point (Fin P) := ⟨p.patch, p.x + p.x - o.x, p.y + p.y - o.y⟩
                    match index[s]?, index[q]? with
                    | some is, some iq => if rep[is]! == rep[iq]! then edgeOf p s else none
                    | _, _ => none
                  else none
              match e with
              | some e => cyc := cyc.push (u, e)
              | none => bad := bad + 1; cyc := cyc.push (u, midRep.size + bad)
            offGrid := offGrid + bad
            let key := ((cyc.map (·.1)).qsort (· < ·)).toList
            if !seen.contains key then
              seen := seen.insert key
              curves := cyc.toList :: curves
              curveSt := (σ, k) :: curveSt
  IO.println s!"  rounds of the walk from every station: {rounds}, {curves.length} distinct curves, {stopped} stopped"
  -- The chart along the border of every round, and round every vertex (`Border.lean`).
  let border ← checkBorder T ℓ pts index rep valence coarse
  -- Lemma "Gauss–Bonnet for turns" on the regions of every simple curve.
  let mut gb : Count := {}
  let mut regionsTotal := 0
  let mut simple := 0
  let mut byRegions : Std.HashMap Nat Nat := ∅
  for c in curves do
    let distinct := (Std.HashSet.ofList (c.map (·.1))).size
    if distinct != c.length then continue
    simple := simple + 1
    let rs := regionsOf faceEdgesK faceCorners facesAtEdge valence coarse [c]
    byRegions := byRegions.insert rs.length (byRegions.getD rs.length 0 + 1)
    for r in rs do
      regionsTotal := regionsTotal + 1
      let total := (r.cones.map fun k => (4 : Int) - k).foldl (· + ·) 0
      gb := gb.add (total == 4 * r.euler) fun _ =>
        s!"a region of {r.faces} faces with χ = {r.euler} holds index {total}"
  IO.println s!"  simple curves: {simple}; their regions: {regionsTotal} (curves by number of regions {byRegions.toList.mergeSort (fun a b => a.1 ≤ b.1)}); curve edges off the grid: {offGrid}"
  gb.show "Gauss–Bonnet on the fine grid: regions of simple curves with ∑ (4 - k) = 4 χ"
  -- Proposition "The shift is set by the singularities" (a) on the regions, on a sphere.
  let discs ← if euler == 2 then
      checkDiscs T ℓ fcs faceEdgesK faceCorners facesAtEdge index rep valence coarse curves
        curveSt paths fox
    else
      IO.println s!"  regions of the simple curves: not a sphere (χ = {euler}), no discs checked"
      pure true
  return ok && gb.ok && parity.ok && discs && border

def checkRaw (raw : Raw) : IO Bool := do
  let P := raw.patches
  let n := raw.arcs
  -- The sides, as lists of arcs.
  if raw.sides.size != 4 * P then throw <| IO.userError "sides: not four per patch"
  if raw.holders.size != n then throw <| IO.userError "holders: not one per arc"
  let sides : Array (List (Fin n)) ← raw.sides.mapM fun s =>
    s.toList.mapM fun a => match (arcOf a : Option (Fin n)) with
      | some a => pure a
      | none => throw <| IO.userError s!"arc {a} out of range"
  let arcs : Side (Fin P) → List (Fin n) := fun σ => sides[sideIndex σ]!
  -- The fields of `TMesh`, every violation listed.
  let repeated := (List.range (4 * P)).filter fun i => !(sides[i]!).Nodup
  let notTwo := (List.range n).filter fun a => (raw.holders[a]!).size != 2
  IO.println s!"  sides with an arc repeated: {repeated.length} {repeated.take 20}"
  IO.println s!"  arcs not on exactly two sides: {notTwo.length} {notTwo.take 20}"
  if !notTwo.isEmpty then
    IO.println "  FAILED: not the gluing of a closed surface, no TMesh"
    return false
  let ends : Array (Side (Fin P) × Side (Fin P) × Int) ← raw.holders.mapM fun h => do
    match (sideOf h[0]!.1 : Option (Side (Fin P))), (sideOf h[1]!.1 : Option (Side (Fin P))) with
    | some σ, some τ => pure (σ, τ, h[0]!.2 * h[1]!.2)
    | _, _ => throw <| IO.userError "side out of range"
  let some ends := (if h : ends.size = n then some (Vector.mk ends h) else none)
    | throw <| IO.userError "holders: not one per arc"
  let endsF : Fin n → Fin 2 → Side (Fin P) := fun a k => if k = 0 then ends[a].1 else ends[a].2.1
  let signF : Fin n → ℤ := fun a => ends[a].2.2
  let same := (List.finRange n).filter fun a => endsF a 0 = endsF a 1
  IO.println s!"  arcs whose two sides are one: {same.length} {(same.take 20).map Fin.val}"
  let some T := mkTMesh arcs endsF signF
    | IO.println "  FAILED: the data do not meet the fields of TMesh"; return false
  IO.println "  TMesh built: no side repeats an arc, every arc on exactly two distinct sides"
  -- `NoArcAcross` and `Oriented`, as the library states them.
  let across := (List.finRange n).filter fun a => (T.ends a 0).row = (T.ends a 1).row
  let unoriented := (List.finRange n).filter fun a =>
    T.sign a ≠ -((T.ends a 0).orient * (T.ends a 1).orient)
  IO.println s!"  NoArcAcross: {decide T.NoArcAcross}, violated by {across.length} arcs {(across.take 20).map Fin.val}"
  IO.println s!"  Oriented: {decide T.Oriented}, violated by {unoriented.length} arcs {(unoriented.take 20).map Fin.val}"
  let mut ok := decide T.NoArcAcross && decide T.Oriented
  -- The balances.
  let cols := columns T
  let mut bal : Count := {}
  if raw.stations.size != 2 * P then
    IO.println s!"  FAILED: {raw.stations.size} balances for {P} patches"
    ok := false
  let mut leanRows : Array (Array (Nat × Int)) := Array.replicate (2 * P) #[]
  for a in [0:n] do
    for (r, v) in cols[a]! do leanRows := leanRows.modify r (·.push (a, v))
  for (p, j, row) in raw.stations do
    let sgn : Int := if j = 0 then 1 else -1
    let engine := (row.map fun (a, c) => (a, sgn * c)).qsort (fun x y => x.1 < y.1)
    let lean := leanRows[2 * p + j]!
    bal := bal.add (p < P && j < 2 && lean == engine) fun _ =>
      s!"row ({p}, {j}): Lean {lean}, engine {engine}"
  bal.show "balances, B row (P, j) = engine row times +1 (j = 0) or -1 (j = 1)"
  ok := ok && bal.ok
  -- The turns.
  let whole := lengths raw.whole
  let walked := lengths raw.walked
  let real := lengths raw.real
  let mut built : Count := {}
  let mut exact : Count := {}
  let mut modulo : Count := {}
  let mut halves := 0
  let mut corridor : Count := {}
  let mut passes : Count := {}
  let mut back : Count := {}
  let mut atReal : Count := {}
  let mut shares := 0
  let mut atWhole : Count := {}
  let mut wholeTurns := 0
  -- Those turns as `(index, γ.length, γ.shift)` at the whole lengths, for the parity.
  let mut lengthShift : Array (Nat × ℚ × ℚ) := #[]
  -- The most `|shift ℓ - engine shift ℓ|` at the whole, walked and real lengths.
  let mut gaps : ℚ × ℚ × ℚ := (0, 0, 0)
  -- The turns for which that gap at the real lengths is beyond 0.05, the real shift
  -- above which the engine holds a turn.
  let mut apart := 0
  -- The coefficients of `shiftForm` and the walk of every turn built, by index.
  let mut forms : Array (Option (Array Int)) := Array.replicate raw.turns.size none
  let mut paths : Array (Option (Side (Fin P) × List (Fin n))) := Array.replicate raw.turns.size none
  -- `Turn.development` (`Holonomy.lean`) of every turn built, by index, when every
  -- length is a positive whole number (the quad mesh of `checkGrid` is then built).
  let positive := raw.whole.all fun x => x.den == 1 && x > 0
  let ℓN : Fin n → ℕ := fun a => (raw.whole[a.val]!).num.toNat
  let mut devs : Array (Option (Motion GaussianInt GaussianInt)) := Array.replicate raw.turns.size none
  let mut developed : Count := {}
  for t in raw.turns do
    let some start := (sideOf t.start : Option (Side (Fin P)))
      | throw <| IO.userError "start side out of range"
    let path ← t.path.toList.mapM fun a => match (arcOf a : Option (Fin n)) with
      | some a => pure a
      | none => throw <| IO.userError s!"arc {a} out of range"
    let some γ := mkTurn T start path
      | built := built.add false fun _ => s!"turn {t.index}: not a turn of the TMesh"
        continue
    built := built.add true fun _ => ""
    -- The shift form: exactly, and modulo the balances.
    let c := coefficients T γ
    if t.index < forms.size then
      forms := forms.set! t.index (some c)
      paths := paths.set! t.index (some (start, path))
    let engine := dense n t.shift
    exact := exact.add (c == engine) fun _ => s!"turn {t.index}: Lean {terms c}, engine {terms engine}"
    let d := (Array.range n).map fun a => ((c[a]! - engine[a]! : Int) : ℚ)
    let lam := guess (2 * P) cols d
    let bad := residual cols lam d
    modulo := modulo.add bad.isEmpty fun _ =>
      s!"turn {t.index}: Lean {terms c}, engine {terms engine}, no λ (residual at {bad.take 10})"
    if bad.isEmpty && lam.any (fun x => x.den != 1) then halves := halves + 1
    -- `γ.shift ℓ`, read through the coefficients (equal to `shiftForm` at every arc).
    let shiftAt (ℓ : Fin n → ℚ) : ℚ := evalForm (fun a => c[a.val]!) ℓ
    let sWhole := shiftAt whole
    -- The development at the whole lengths: the translation by `-α e₁ + shift e₂` in the
    -- frame of the first side, `α` the library's `Turn.length`.
    if positive then
      let dv := γ.development ℓN
      if t.index < devs.size then devs := devs.set! t.index (some dv)
      let α := γ.length whole
      let α' := Check.along T whole start path
      if α != α' then IO.println s!"    turn {t.index}: Turn.length {α}, along {α'}"
      let e₁ := TMesh.inward start
      let e₂ := TMesh.acrossDir start
      developed := developed.add (dv.lin == 1 &&
          dv.trans == -(α.num : GaussianInt) * e₁ + (sWhole.num : GaussianInt) * e₂) fun _ =>
        s!"turn {t.index}: development {showG dv.lin}, {showG dv.trans}; α {α}, shift {sWhole}, e₁ {showG e₁}, e₂ {showG e₂}"
    let engineAt (ℓ : Fin n → ℚ) : ℚ := evalForm (fun a => engine[a.val]!) ℓ
    let gap (ℓ : Fin n → ℚ) : ℚ := |shiftAt ℓ - engineAt ℓ|
    gaps := (max gaps.1 (gap whole), max gaps.2.1 (gap walked), max gaps.2.2 (gap real))
    if gap real > 1 / 20 then apart := apart + 1
    -- At the walked lengths: the engine's offset, its crossings, the come-back.
    let o := t.offset
    corridor := corridor.add (follows T walked start path o) fun _ =>
      s!"turn {t.index}: offset {o} not in the corridor"
    let mut p := (start, o)
    let mut along := t.crossed.size == path.length
    for k in [0:path.length] do
      if !near p.2 t.crossed[k]! then along := false
      match T.step walked p with
      | some q => p := q
      | none => along := false; break
    passes := passes.add along fun _ => s!"turn {t.index}: the walk is not where the engine crossed"
    back := back.add (T.walk walked path.length (start, o) == some (start, o + shiftAt walked))
      fun _ => s!"turn {t.index}: the walk does not come back at offset + shift"
    -- At the real lengths, the same share of the first side.
    if let some s := t.share then
      shares := shares + 1
      let o := s * ((T.arcs start).map real).sum
      if follows T real start path o then
        atReal := atReal.add (T.walk real path.length (start, o) == some (start, o + shiftAt real))
          fun _ => s!"turn {t.index}: at the real lengths the walk does not come back at offset + shift"
    -- At the whole lengths, every half-integer of the corridor.
    let L := ((T.arcs start).map whole).sum
    let mut k : Nat := 0
    let mut any := false
    while (k : ℚ) + 1 / 2 < L do
      let o : ℚ := k + 1 / 2
      if follows T whole start path o then
        any := true
        atWhole := atWhole.add (T.walk whole path.length (start, o) == some (start, o + sWhole))
          fun _ => s!"turn {t.index}: at the whole lengths from {o} the walk does not come back at offset + shift"
      k := k + 1
    if any then
      wholeTurns := wholeTurns + 1
      lengthShift := lengthShift.push (t.index, γ.length whole, sWhole)
  built.show "turns of the TMesh (IsPath, closes)"
  exact.show "shift forms, shiftForm = engine shift coefficient by coefficient" 2
  modulo.show "shift forms, shiftForm - engine shift = λᵀ B (checked exactly)"
  IO.println s!"    with λ not whole for {halves}"
  IO.println s!"    largest |shift - engine shift|: {gaps.1} at the whole lengths, {approx gaps.2.1} at the walked lengths, {approx gaps.2.2} at the real lengths, beyond 0.05 for {apart} turns"
  corridor.show "walked lengths: engine offset in the corridor"
  passes.show "walked lengths: walk where the engine crossed (within 1e-6)"
  back.show "walked lengths: walk comes back at offset + shift"
  IO.println s!"  real lengths: {atReal.checked} of {shares} turns hold the same share of their side in the corridor"
  atReal.show "real lengths: walk comes back at offset + shift"
  IO.println s!"  whole lengths: {wholeTurns} of {raw.turns.size} turns have a half-integer in their corridor"
  atWhole.show "whole lengths: half-integers of the corridors, walk comes back at offset + shift"
  if positive then
    developed.show "whole lengths: Turn.development = translation by -α e₁ + shift e₂ (Holonomy.lean)"
  else
    IO.println "  whole lengths: an arc of length zero, no development of the turns"
  ok := ok && built.ok && modulo.ok && corridor.ok && passes.ok && back.ok && atReal.ok &&
    atWhole.ok && developed.ok
  -- The quad mesh at the whole lengths, its curves and their regions (`Grid.lean`).
  let gridOk ← checkGrid T raw.whole lengthShift paths raw.fox
  ok := ok && gridOk
  -- Theorem "Forced shifts", with the certificates of `certify.py`.
  match raw.forced with
  | some f =>
    let good ← checkForced n (2 * P) cols leanRows forms (raw.turns.map (·.held)) raw.lower f
    ok := ok && good
  | none => IO.println "  forced shifts: no certificates for this export"
  -- The Fox–Burgers formula, at the whole lengths.
  let unbalanced := (List.range (2 * P)).filter fun r =>
    leanRows[r]!.foldl (fun s (a, v) => s + v * raw.whole[a]!) (0 : ℚ) != 0
  match raw.fox with
  | some fox =>
    IO.println s!"  whole lengths balanced (B ℓ = 0): {unbalanced.isEmpty}"
    let good ← checkFox T whole forms paths devs fox
    ok := ok && good && unbalanced.isEmpty
  | none => IO.println s!"  Fox–Burgers: out of scope, {raw.foxSkipped}"
  return ok

end Check

def main (files : List String) : IO UInt32 := do
  let mut ok := true
  for file in files do
    IO.println s!"{file}"
    let start ← IO.monoMsNow
    let text ← IO.FS.readFile file
    let raw ← match Json.parse text >>= Check.readRaw with
      | .ok raw => pure raw
      | .error e => throw <| IO.userError s!"{file}: {e}"
    let good ← Check.checkRaw raw
    IO.println s!"  {if good then "PASSED" else "FAILED"} in {(← IO.monoMsNow) - start} ms"
    ok := ok && good
  return if ok then 0 else 1
