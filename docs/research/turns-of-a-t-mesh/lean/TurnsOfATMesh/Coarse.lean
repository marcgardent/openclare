/-
The quad mesh of a T-mesh at whole lengths as a closed surface of quads
(DT-173): its vertices, edges and faces are the points of the fine grid of
`Grid.lean` read by parity. A point with both coordinates even is a vertex of
the quad mesh (`Point.IsCoarse`); a point with exactly one odd coordinate is
the midpoint of an edge of the quad mesh, whose two ends are the vertices
beside it (`Point.IsEdge`, `edgeEnds`); a point with both coordinates odd is
the centre of a face, whose four corners and four edges are the points around
it (`Point.IsCentre`, `centreCorner`, `centreEdge`). All of them up to `Same`,
as `Vertex`. The gluings keep the parity of the coordinates
(`GridEuler.parity_of_same`), so the three kinds never mix.

This is the interface of the last step of DT-173. `CoarseEuler.lean` proves
that `eulerQuad`, the Euler characteristic of this quad mesh, is `P − A + N`
as `GridEuler.euler_eq_nodeCount` does for the fine grid; `CoarseSurface.lean`
proves that the quad mesh meets `QuadSurface` of `SphereBipartite.lean`
(every edge in exactly two faces, connected); `SphereGrid.lean` then gets
`Bipartite` from `bipartite_of_euler` when `P − A + N = 2`, and with
`Bipartite.lean` the parity of every turn of a sphere, with no colouring
hypothesis.
-/
module

public import TurnsOfATMesh.GridSurface
public import TurnsOfATMesh.SphereBipartite

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

namespace TMesh

variable {Patch α : Type*} [DecidableEq Patch] [DecidableEq α] (T : TMesh Patch α)

/-- A point of the fine grid is the midpoint of an edge of the quad mesh when
exactly one of its coordinates is odd. -/
def Point.IsEdge (p : Point Patch) : Prop := (p.x + p.y) % 2 = 1

/-- A point of the fine grid is the centre of a face of the quad mesh when both
its coordinates are odd. -/
def Point.IsCentre (p : Point Patch) : Prop := p.x % 2 = 1 ∧ p.y % 2 = 1

instance (p : Point Patch) : Decidable p.IsEdge := by unfold Point.IsEdge; infer_instance

instance (p : Point Patch) : Decidable p.IsCentre := by unfold Point.IsCentre; infer_instance

/-- The two ends of the edge of the quad mesh with midpoint `m`: the vertices
beside it, along `u` when `m.x` is odd, along `v` otherwise. -/
def edgeEnds (m : Point Patch) : Point Patch × Point Patch :=
  if m.x % 2 = 1 then (⟨m.patch, m.x - 1, m.y⟩, ⟨m.patch, m.x + 1, m.y⟩)
  else (⟨m.patch, m.x, m.y - 1⟩, ⟨m.patch, m.x, m.y + 1⟩)

/-- The four corners of the face of the quad mesh with centre `c`,
counter-clockwise in the frame of its rectangle from the lower left. -/
def centreCorner (c : Point Patch) : Fin 4 → Point Patch :=
  ![⟨c.patch, c.x - 1, c.y - 1⟩, ⟨c.patch, c.x + 1, c.y - 1⟩,
    ⟨c.patch, c.x + 1, c.y + 1⟩, ⟨c.patch, c.x - 1, c.y + 1⟩]

/-- The four edges of the face with centre `c`, as their midpoints: the edge `i`
joins the corners `i` and `i + 1`. -/
def centreEdge (c : Point Patch) : Fin 4 → Point Patch :=
  ![⟨c.patch, c.x, c.y - 1⟩, ⟨c.patch, c.x + 1, c.y⟩,
    ⟨c.patch, c.x, c.y + 1⟩, ⟨c.patch, c.x - 1, c.y⟩]

/-- The vertices of the quad mesh: the classes of the points with both
coordinates even. -/
noncomputable def numQuadVertices (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) : ℕ :=
  (T.vertexOf allArcs ℓ '' {p | p ∈ T.points patches ℓ ∧ p.IsCoarse}).ncard

/-- The edges of the quad mesh: the classes of the points with one odd coordinate. -/
noncomputable def numQuadEdges (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) : ℕ :=
  (T.vertexOf allArcs ℓ '' {p | p ∈ T.points patches ℓ ∧ p.IsEdge}).ncard

/-- The faces of the quad mesh: the classes of the points with both coordinates
odd (each one alone, inside a rectangle). -/
noncomputable def numQuadFaces (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) : ℕ :=
  (T.vertexOf allArcs ℓ '' {p | p ∈ T.points patches ℓ ∧ p.IsCentre}).ncard

/-- **The Euler characteristic of the quad mesh** at `ℓ`, `V − E + F`. -/
noncomputable def eulerQuad (patches : List Patch) (allArcs : List α) (ℓ : α → ℕ) : ℤ :=
  (T.numQuadVertices patches allArcs ℓ : ℤ) - T.numQuadEdges patches allArcs ℓ +
    T.numQuadFaces patches allArcs ℓ

/-! ### What is proved here -/

theorem isEdge_centreEdge (c : Point Patch) (hc : c.IsCentre) (i : Fin 4) :
    (centreEdge c i).IsEdge := by
  obtain ⟨hx, hy⟩ := hc
  fin_cases i <;> simp [centreEdge, Point.IsEdge] <;> omega

theorem isCoarse_centreCorner (c : Point Patch) (hc : c.IsCentre) (i : Fin 4) :
    (centreCorner c i).IsCoarse := by
  obtain ⟨hx, hy⟩ := hc
  fin_cases i <;> simp [centreCorner, Point.IsCoarse] <;> omega

theorem isCoarse_edgeEnds (m : Point Patch) (hm : m.IsEdge) :
    (edgeEnds m).1.IsCoarse ∧ (edgeEnds m).2.IsCoarse := by
  unfold edgeEnds Point.IsEdge at *
  split_ifs <;> simp [Point.IsCoarse] <;> omega

/-- The edge `i` of a face joins its corners `i` and `i + 1`. -/
theorem edgeEnds_centreEdge (c : Point Patch) (hc : c.IsCentre) (i : Fin 4) :
    edgeEnds (centreEdge c i) = (centreCorner c i, centreCorner c (i + 1)) ∨
      edgeEnds (centreEdge c i) = (centreCorner c (i + 1), centreCorner c i) := by
  obtain ⟨P, x, y⟩ := c
  obtain ⟨hx, hy⟩ := hc
  simp only at hx hy
  obtain ⟨k, rfl⟩ : ∃ k, x = 2 * k + 1 := ⟨x / 2, by omega⟩
  obtain ⟨l, rfl⟩ : ∃ l, y = 2 * l + 1 := ⟨y / 2, by omega⟩
  fin_cases i <;> simp [centreEdge, centreCorner, edgeEnds, Nat.add_mod]

end TMesh

end TurnsOfATMesh
