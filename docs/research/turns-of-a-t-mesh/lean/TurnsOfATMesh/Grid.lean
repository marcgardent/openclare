/-
The quad mesh of a T-mesh at whole lengths, defined on `TMesh` (DT-134): the
integer grid of each rectangle, glued along the arcs. It is built refined by
two, as the Lean model of Gauss–Bonnet reads it: a point at every half-integer
place of every rectangle, so that the dual line of a strip (at a half-integer
offset) and the sides of the rectangles are both paths of the fine grid.

Everything here is a computable definition on the data of the T-mesh and
whole lengths `ℓ : α → ℕ`: the points, the gluing pairs along the arcs, the
faces with their corners, the dual line across a rectangle and the closed
curve of one round of a walk. The vertices of the quad mesh are the classes of
the equivalence the gluings generate (`Same`, `Vertex`); the program of
`Check/Main.lean` computes them, the regions a curve cuts and their Euler
characteristics from these lists, and checks the statements of the note on
them. Theorems on these definitions: `GridEuler.lean` (the Euler
characteristic of the grid is `P − A + N`, independent of the lengths) and
`Bipartite.lean` (the parity of a turn from a colouring of the quad graph).
That a sphere's quad graph is bipartite is `SphereGrid.lean`, through
`Coarse.lean`, `CoarseEuler.lean`, `CoarseSurface.lean` and
`SphereBipartite.lean`; the program still decides it on every export.

Coordinates: in the frame `(u, v)` of a rectangle of width `w` (the length of
its side `(P, 0, 0)`) and height `h` (of `(P, 1, 0)`), the fine point `(x, y)`
has `0 ≤ x ≤ 2w`, `0 ≤ y ≤ 2h`. The side `(P, 0, 0)` runs along `v = 0`,
`(P, 0, 1)` along `v = h`, both listed along `+u`; `(P, 1, 0)` along `u = 0`,
`(P, 1, 1)` along `u = w`, both listed along `+v`, as `TMesh.lean` says.
-/
module

public import TurnsOfATMesh.TMesh

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

/-- A point of the fine grid: a rectangle and its coordinates in half quad
edges. -/
structure Point (Patch : Type*) where
  patch : Patch
  x : ℕ
  y : ℕ
deriving DecidableEq, Repr, Hashable

/-- A face of the fine grid: a rectangle and the lower-left corner `(i, j)` of
a unit square, `i < 2w`, `j < 2h`. -/
structure Face (Patch : Type*) where
  patch : Patch
  i : ℕ
  j : ℕ
deriving DecidableEq, Repr, Hashable

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-! ### Sides and arcs at whole lengths -/

/-- The length of a side, in quad edges. -/
def sideLen (ℓ : α → ℕ) (σ : Side Patch) : ℕ := ((T.arcs σ).map ℓ).sum

/-- The width of a rectangle: the length of its side `v = 0`. -/
def width (ℓ : α → ℕ) (P : Patch) : ℕ := T.sideLen ℓ (P, 0, 0)

/-- The height of a rectangle: the length of its side `u = 0`. -/
def height (ℓ : α → ℕ) (P : Patch) : ℕ := T.sideLen ℓ (P, 1, 0)

/-- The position of the arc `a` on a side whose arcs are `l`, in quad edges:
`posAt` over the natural numbers. -/
def posNat (a : α) : List α → (α → ℕ) → ℕ
  | [], _ => 0
  | b :: l, ℓ => if b = a then 0 else ℓ b + posNat a l ℓ

/-- The point of the side `σ` at the fine position `t` from its first corner,
`0 ≤ t ≤ 2 · sideLen σ`. -/
def sidePoint (ℓ : α → ℕ) (σ : Side Patch) (t : ℕ) : Point Patch :=
  if σ.2.1 = 0 then
    if σ.2.2 = 0 then ⟨σ.1, t, 0⟩ else ⟨σ.1, t, 2 * T.height ℓ σ.1⟩
  else
    if σ.2.2 = 0 then ⟨σ.1, 0, t⟩ else ⟨σ.1, 2 * T.width ℓ σ.1, t⟩

/-- **The gluing of an arc**: the points of its two sides at the fine position `t`
along it, `0 ≤ t ≤ 2 ℓ_a`, measured from its first end on `ends a 0`; on
`ends a 1` the same point is at `t` from the arc's start if the sides run along
it in the same direction, at `2 ℓ_a - t` otherwise. -/
def glued (ℓ : α → ℕ) (a : α) (t : ℕ) : Point Patch × Point Patch :=
  let σ := T.ends a 0
  let τ := T.ends a 1
  let p := 2 * posNat a (T.arcs σ) ℓ
  let q := 2 * posNat a (T.arcs τ) ℓ
  (T.sidePoint ℓ σ (p + t), T.sidePoint ℓ τ (if T.sign a = 1 then q + t else q + 2 * ℓ a - t))

/-! The patches and the arcs are enumerated by lists given to the definitions
(the program gives `List.finRange`), which keeps them computable. -/

/-- Every gluing pair of the fine grid, the arcs being `allArcs`. -/
def gluings (allArcs : List α) (ℓ : α → ℕ) : List (Point Patch × Point Patch) :=
  allArcs.flatMap fun a => (List.range (2 * ℓ a + 1)).map (T.glued ℓ a)

/-- Two points are one vertex of the quad mesh when the gluings connect them. -/
def Same (allArcs : List α) (ℓ : α → ℕ) : Point Patch → Point Patch → Prop :=
  Relation.EqvGen fun p q => (p, q) ∈ T.gluings allArcs ℓ

/-- The vertices of the quad mesh: the points up to the gluings. -/
def vertexSetoid (allArcs : List α) (ℓ : α → ℕ) : Setoid (Point Patch) :=
  ⟨T.Same allArcs ℓ, Relation.EqvGen.is_equivalence _⟩

/-- The vertices of the quad mesh at the whole lengths `ℓ`. -/
def Vertex (allArcs : List α) (ℓ : α → ℕ) := Quotient (T.vertexSetoid allArcs ℓ)

/-- Every point of the fine grid, the patches being `patches`. -/
def points (patches : List Patch) (ℓ : α → ℕ) : List (Point Patch) :=
  patches.flatMap fun P =>
    (List.range (2 * T.width ℓ P + 1)).flatMap fun x =>
      (List.range (2 * T.height ℓ P + 1)).map fun y => ⟨P, x, y⟩

/-- Every face of the fine grid. -/
def faces (patches : List Patch) (ℓ : α → ℕ) : List (Face Patch) :=
  patches.flatMap fun P =>
    (List.range (2 * T.width ℓ P)).flatMap fun i =>
      (List.range (2 * T.height ℓ P)).map fun j => ⟨P, i, j⟩

/-- The four corners of a face, counter-clockwise in the frame of its rectangle. -/
def corners (f : Face Patch) : List (Point Patch) :=
  [⟨f.patch, f.i, f.j⟩, ⟨f.patch, f.i + 1, f.j⟩, ⟨f.patch, f.i + 1, f.j + 1⟩, ⟨f.patch, f.i, f.j + 1⟩]

/-- The four edges of a face, as pairs of corners. -/
def faceEdges (f : Face Patch) : List (Point Patch × Point Patch) :=
  let a : Point Patch := ⟨f.patch, f.i, f.j⟩
  let b : Point Patch := ⟨f.patch, f.i + 1, f.j⟩
  let c : Point Patch := ⟨f.patch, f.i + 1, f.j + 1⟩
  let d : Point Patch := ⟨f.patch, f.i, f.j + 1⟩
  [(a, b), (b, c), (c, d), (d, a)]

/-! ### The curve of a walk -/

/-- **The dual line across a rectangle** from the station `k + ½` of the side
`σ`, as the fine points it passes, from `σ` to the opposite side: at the fine
coordinate `2k + 1` across, every half step along. -/
def across (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) : List (Point Patch) :=
  let P := σ.1
  let t := 2 * k + 1
  if σ.2.1 = 0 then
    let ys := List.range (2 * T.height ℓ P + 1)
    (if σ.2.2 = 0 then ys else ys.reverse).map fun y => ⟨P, t, y⟩
  else
    let xs := List.range (2 * T.width ℓ P + 1)
    (if σ.2.2 = 0 then xs else xs.reverse).map fun x => ⟨P, x, t⟩

/-- The points of the side `σ` from the fine position `s` to the fine position
`t`, both included, in either order: the closing segment of a turn, from the
station it came back at to the one it left from. -/
def alongSide (ℓ : α → ℕ) (σ : Side Patch) (s t : ℕ) : List (Point Patch) :=
  if s ≤ t then (List.range (t - s + 1)).map fun d => T.sidePoint ℓ σ (s + d)
  else (List.range (s - t + 1)).map fun d => T.sidePoint ℓ σ (s - d)

/-- The lengths as rationals, for the walk of `TMesh.lean`. -/
def rat (ℓ : α → ℕ) : α → ℚ := fun a => (ℓ a : ℚ)

/-- The station `k + ½` of the side `σ` as a start of the walk. -/
def station (σ : Side Patch) (k : ℕ) : Side Patch × ℚ := (σ, (k : ℚ) + 1 / 2)

/-- **One round of the walk** from the station `k + ½` of the side `σ` at the
whole lengths `ℓ`, as the fine points of its curve: across each rectangle, then
across the arc, until it comes back in through `σ`; then the closing segment
along `σ` back to its start. `none` if the walk stops (it never does on a
closed surface) or does not come back within `fuel` crossings. The first point
of the list is the start station and the last point is the same station again,
reached along the closing segment (for a ring, the station the walk comes back
to). -/
def curve (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) : ℕ → List (Point Patch) → Side Patch × ℚ →
    Option (List (Point Patch))
  | 0, _, _ => none
  | fuel + 1, acc, p =>
    -- `p.2` is a half-integer station of `p.1`: `2 p.2 = 2 k' + 1`.
    let k' : ℕ := ((2 * p.2 - 1) / 2).floor.toNat
    let line := T.across ℓ p.1 k'
    match T.step (rat ℓ) p with
    | none => none
    | some q =>
      let acc := acc ++ line.dropLast
      if q.1 = σ then
        let kq : ℕ := ((2 * q.2 - 1) / 2).floor.toNat
        some (acc ++ T.alongSide ℓ σ (2 * kq + 1) (2 * k + 1))
      else curve ℓ σ k fuel acc q

/-- The number of stations of the T-mesh, a bound on the crossings of one round. -/
def stations (patches : List Patch) (ℓ : α → ℕ) : ℕ :=
  (patches.flatMap fun P =>
    [T.sideLen ℓ (P, 0, 0), T.sideLen ℓ (P, 0, 1), T.sideLen ℓ (P, 1, 0), T.sideLen ℓ (P, 1, 1)]).sum

/-- The curve of one round from the station `k + ½` of `σ`. -/
def round (patches : List Patch) (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) : Option (List (Point Patch)) :=
  T.curve ℓ σ k (T.stations patches ℓ + 1) [] (station σ k)

/-! ### The length of a turn, and the 2-colouring of the quad graph (DT-173)

The objects the parity of a turn needs, as definitions on the `TMesh`. The
length `α_γ(ℓ)` of a turn is the sum, over the patches it crosses, of the
length across that patch (`acrossForm`). The colour of a vertex of the quad
graph (a point of the fine grid with both coordinates even) is `x/2 + y/2 +
κ_P mod 2` for constants `κ_P`, one per patch, chosen so that the colour agrees
on the two points of every gluing pair (`IsColouring`). On a sphere such
constants exist; on a torus, not always (a `3 × 3` grid). -/

/-- The length across a patch when it is entered through the side `σ`: the
length of the sides of the other pair, from their first side. At balanced
lengths both sides of the pair have that length. -/
def acrossForm (σ : Side Patch) : α → ℤ := lengthForm (T.arcs (σ.1, σ.2.1 + 1, 0))

/-- The length of a path of arcs from the side `σ`, as an integer linear form in
the lengths: the length across each patch entered. -/
def pathLengthForm : Side Patch → List α → α → ℤ
  | _, [] => 0
  | σ, a :: l => T.acrossForm σ + pathLengthForm (T.other a σ.opp) l

/-- **The length of a turn** `α_γ`, as an integer linear form in the lengths. -/
def Turn.lengthForm {T : TMesh Patch α} (γ : T.Turn) : α → ℤ := T.pathLengthForm γ.start γ.path

/-- The length of a turn at the lengths `ℓ`. -/
def Turn.length [Fintype α] {K : Type*} [CommRing K] {T : TMesh Patch α} (γ : T.Turn)
    (ℓ : α → K) : K := evalForm γ.lengthForm ℓ

/-- A point of the fine grid is a vertex of the quad graph when both its
coordinates are even. -/
def Point.IsCoarse (p : Point Patch) : Prop := p.x % 2 = 0 ∧ p.y % 2 = 0

instance (p : Point Patch) : Decidable p.IsCoarse := by unfold Point.IsCoarse; infer_instance

/-- The colour of a point of the fine grid under the constants `κ`: for a vertex
of the quad graph `(2i, 2j)` of the patch `P`, `i + j + κ_P mod 2`, so that
every edge of the quad graph inside a patch changes the colour. -/
def pointColour (κ : Patch → ZMod 2) (p : Point Patch) : ZMod 2 :=
  ((p.x / 2 : ℕ) : ZMod 2) + ((p.y / 2 : ℕ) : ZMod 2) + κ p.patch

/-- **`κ` colours the quad graph at `ℓ`**: the two points of every gluing pair of
vertices of the quad graph have the same colour. With the colour changing along
every edge inside a patch, the quad graph is then properly 2-coloured. -/
def IsColouring (allArcs : List α) (ℓ : α → ℕ) (κ : Patch → ZMod 2) : Prop :=
  ∀ pq ∈ T.gluings allArcs ℓ, pq.1.IsCoarse → pq.2.IsCoarse →
    pointColour κ pq.1 = pointColour κ pq.2

instance (allArcs : List α) (ℓ : α → ℕ) (κ : Patch → ZMod 2) :
    Decidable (T.IsColouring allArcs ℓ κ) := by unfold IsColouring; infer_instance

/-- **The quad graph at `ℓ` is bipartite**: some constants colour it. -/
def Bipartite (allArcs : List α) (ℓ : α → ℕ) : Prop := ∃ κ, T.IsColouring allArcs ℓ κ

/-- The vertex of the quad graph beside the station `k + ½` of the side `σ`: the
point of `σ` at the fine position `2k`, the corner of the quad the walk leaves
through. The parity proof follows this vertex round the walk: across each
patch its colour changes by the length across, across each arc it is glued to
a vertex of the same colour (`IsColouring`), and the walk comes back beside the
station `k + ½ + shift`. -/
def stationVertex (ℓ : α → ℕ) (σ : Side Patch) (k : ℕ) : Point Patch := T.sidePoint ℓ σ (2 * k)

/-! ### What is proved here -/

/-- The gluing pairs are symmetric as a relation: `Same` is the equivalence
generated by the gluings read either way. -/
theorem same_symm (allArcs : List α) (ℓ : α → ℕ) {p q : Point Patch}
    (h : T.Same allArcs ℓ p q) : T.Same allArcs ℓ q p :=
  Relation.EqvGen.symm p q h

/-- A glued pair is one vertex. -/
theorem same_of_glued (allArcs : List α) (ℓ : α → ℕ) (a : α) (ha : a ∈ allArcs) (t : ℕ)
    (ht : t ≤ 2 * ℓ a) : T.Same allArcs ℓ (T.glued ℓ a t).1 (T.glued ℓ a t).2 := by
  apply Relation.EqvGen.rel
  unfold gluings
  simp only [List.mem_flatMap, List.mem_map, List.mem_range]
  exact ⟨a, ha, t, by omega, rfl⟩

/-- The first end of an arc on its first side is the point at the arc's
position; the gluing of an arc starts there. -/
theorem glued_zero_fst (ℓ : α → ℕ) (a : α) :
    (T.glued ℓ a 0).1 = T.sidePoint ℓ (T.ends a 0) (2 * posNat a (T.arcs (T.ends a 0)) ℓ) := by
  simp [glued]

end TMesh

end TurnsOfATMesh
