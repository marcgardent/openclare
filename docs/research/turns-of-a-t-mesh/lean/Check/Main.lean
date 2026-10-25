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
  offset plus the shift.

All arithmetic is exact (`ℤ`, `ℚ`; the engine's floats are read as the fractions they
are). `Turn.shiftForm` and `balance` are read only at the arcs of the sides they
involve; `shiftForm_eq_zero` and `balance_eq_zero` below prove that they are zero at
every other arc.
-/
import TurnsOfATMesh.TMesh
import Lean.Data.Json

open Lean TurnsOfATMesh TMesh

namespace Check

/-! ### Reading the converted export -/

structure RawTurn where
  index : Nat
  start : Nat
  path : Array Nat
  shift : Array (Nat × Int)
  offset : ℚ
  share : Option ℚ
  crossed : Array ℚ

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
    start := ← nat (← field j "start")
    path := ← (← arr (← field j "path")).mapM nat
    shift := ← form (← field j "shift")
    offset := ← rat (← field j "offset")
    share
    crossed := ← (← arr (← field j "crossed")).mapM rat }

def readRaw (j : Json) : Except String Raw := do
  let stations ← (← arr (← field j "stations")).mapM fun s => do
    return (← nat (← field s "patch"), ← nat (← field s "pair"), ← form (← field s "row"))
  return {
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

def guess (R : Nat) (cols : Array (Array (Nat × Int))) (d : Array Int) : Array ℚ := Id.run do
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
        let da : ℚ := d[a]!
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
def residual (cols : Array (Array (Nat × Int))) (lam : Array ℚ) (d : Array Int) : List Nat :=
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

/-! ### The check of one export -/

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
  -- The most `|shift ℓ - engine shift ℓ|` at the whole, walked and real lengths.
  let mut gaps : ℚ × ℚ × ℚ := (0, 0, 0)
  -- The turns for which that gap at the real lengths is beyond 0.05, the real shift
  -- above which the engine holds a turn.
  let mut apart := 0
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
    let engine := dense n t.shift
    exact := exact.add (c == engine) fun _ => s!"turn {t.index}: Lean {terms c}, engine {terms engine}"
    let d := (Array.range n).map fun a => c[a]! - engine[a]!
    let lam := guess (2 * P) cols d
    let bad := residual cols lam d
    modulo := modulo.add bad.isEmpty fun _ =>
      s!"turn {t.index}: Lean {terms c}, engine {terms engine}, no λ (residual at {bad.take 10})"
    if bad.isEmpty && lam.any (fun x => x.den != 1) then halves := halves + 1
    -- `γ.shift ℓ`, read through the coefficients (equal to `shiftForm` at every arc).
    let shiftAt (ℓ : Fin n → ℚ) : ℚ := evalForm (fun a => c[a.val]!) ℓ
    let sWhole := shiftAt whole
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
    if any then wholeTurns := wholeTurns + 1
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
  return ok && built.ok && modulo.ok && corridor.ok && passes.ok && back.ok && atReal.ok &&
    atWhole.ok

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
