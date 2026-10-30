/-
A closed surface made of quads with Euler characteristic `2` has a bipartite
1-skeleton (`QuadSurface.bipartite_of_euler`). Nothing topological: linear
algebra over `ZMod 2` on the chains of a finite combinatorial surface.

The boundaries `∂₂` (faces to edges) and `∂₁` (edges to vertices) compose to `0`
(`bd1_comp_bd2`). A 2-chain without boundary is constant on the faces, which are
connected through shared edges, so `∂₂` has rank at least `F − 1`
(`card_faces_le`). The vertices are connected by edges, so `∂₁` has rank at least
`V − 1` (`card_vertices_le`). With `V − E + F = 2` the two counts meet and every
cycle bounds (`range_bd2_eq_ker_bd1`). A face boundary has an even number of
edges, so every cycle does (`total_eq_zero_of_bd1`), and the parity of the number
of edges of a chain from a fixed vertex is a colouring.
-/
module

public import Mathlib.LinearAlgebra.FiniteDimensional.Lemmas
public import Mathlib.LinearAlgebra.Dimension.Constructions
public import Mathlib.Algebra.Field.ZMod
public import Mathlib.Data.Sym.Sym2
public import Mathlib.Logic.Relation

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

/-- A finite closed surface made of quads: vertices `V`, edges `E`, faces `F`.
The edge `faceEdge f i` joins the corners `i` and `i + 1` of the face `f`, every
edge lies in exactly two slots `(f, i)`, the vertices are connected by edges and
the faces through shared edges. -/
structure QuadSurface (V E F : Type*) [Fintype V] [Fintype E] [Fintype F]
    [DecidableEq V] [DecidableEq E] [DecidableEq F] where
  /-- The two ends of an edge. -/
  ends : E → V × V
  /-- The edge in the slot `i` of a face. -/
  faceEdge : F → Fin 4 → E
  /-- The corner `i` of a face. -/
  faceCorner : F → Fin 4 → V
  /-- The edge in the slot `i` joins the corners `i` and `i + 1`. -/
  faceEdge_ends : ∀ f i, s((ends (faceEdge f i)).1, (ends (faceEdge f i)).2) = s(faceCorner f i, faceCorner f (i + 1))
  /-- Every edge lies in exactly two slots. -/
  two_slots : ∀ e, (Finset.univ.filter fun fi : F × Fin 4 => faceEdge fi.1 fi.2 = e).card = 2
  /-- Any two vertices are joined by a chain of edges. -/
  connected : ∀ u v, Relation.ReflTransGen (fun a b => ∃ e, s((ends e).1, (ends e).2) = s(a, b)) u v
  /-- Any two faces are joined by a chain of faces sharing an edge. -/
  faceConnected :
    ∀ f g, Relation.ReflTransGen (fun a b => ∃ i j, faceEdge a i = faceEdge b j) f g

namespace QuadSurface

variable {V E F : Type*} [Fintype V] [Fintype E] [Fintype F]
  [DecidableEq V] [DecidableEq E] [DecidableEq F] (S : QuadSurface V E F)

/-- The boundary of an edge: its two ends. -/
def endsVec (e : E) : V → ZMod 2 := Pi.single (S.ends e).1 1 + Pi.single (S.ends e).2 1

/-- The boundary of 2-chains. -/
def bd2 : (F → ZMod 2) →ₗ[ZMod 2] (E → ZMod 2) where
  toFun x := ∑ fi : F × Fin 4, x fi.1 • Pi.single (S.faceEdge fi.1 fi.2) 1
  map_add' x y := by simp [add_smul, Finset.sum_add_distrib]
  map_smul' c x := by simp [Finset.smul_sum, smul_smul]

/-- The boundary of 1-chains. -/
def bd1 : (E → ZMod 2) →ₗ[ZMod 2] (V → ZMod 2) where
  toFun y := ∑ e, y e • S.endsVec e
  map_add' x y := by simp [add_smul, Finset.sum_add_distrib]
  map_smul' c x := by simp [Finset.smul_sum, smul_smul]

/-- The number of cells of a chain, mod 2. -/
def total (ι : Type*) [Fintype ι] : (ι → ZMod 2) →ₗ[ZMod 2] ZMod 2 where
  toFun y := ∑ i, y i
  map_add' x y := by simp [Finset.sum_add_distrib]
  map_smul' c x := by simp [Finset.mul_sum]

theorem endsVec_of_mk {e : E} {a b : V} (h : s((S.ends e).1, (S.ends e).2) = s(a, b)) :
    S.endsVec e = Pi.single a 1 + Pi.single b 1 := by
  obtain ⟨⟨p, q⟩, hpq⟩ : ∃ pq, S.ends e = pq := ⟨_, rfl⟩
  rw [hpq] at h
  rcases Sym2.eq_iff.1 h with ⟨rfl, rfl⟩ | ⟨rfl, rfl⟩ <;> simp [endsVec, hpq, add_comm]

theorem bd2_apply (x : F → ZMod 2) (e : E) :
    S.bd2 x e = ∑ fi ∈ Finset.univ.filter (fun fi : F × Fin 4 => S.faceEdge fi.1 fi.2 = e),
      x fi.1 := by
  simp [bd2, Finset.sum_apply, Pi.single_apply, Finset.sum_filter, eq_comm]

theorem add_self_vec {ι : Type*} (g : ι → ZMod 2) : g + g = 0 := by
  funext i; exact CharTwo.add_self_eq_zero (g i)

theorem quad_sum {ι : Type*} (a b c d : ι → ZMod 2) : a + b + (b + c) + (c + d) + (d + a) = 0 := by
  rw [show a + b + (b + c) + (c + d) + (d + a) = (a + a) + (b + b) + (c + c) + (d + d) by abel]
  simp only [add_self_vec, add_zero]

theorem tri_sum {ι : Type*} (a b c : ι → ZMod 2) : a + b + (b + c) + (a + c) = 0 := by
  rw [show a + b + (b + c) + (a + c) = (a + a) + (b + b) + (c + c) by abel]
  simp only [add_self_vec, add_zero]

theorem bd1_single (e : E) : S.bd1 (Pi.single e 1) = S.endsVec e := by
  simp [bd1, Pi.single_apply]

/-! ### `∂₁ ∘ ∂₂ = 0` -/

/-- The four edges of a face cover each corner twice. -/
theorem sum_endsVec_face (f : F) : ∑ i, S.endsVec (S.faceEdge f i) = 0 := by
  rw [Fin.sum_univ_four, S.endsVec_of_mk (S.faceEdge_ends f 0),
    S.endsVec_of_mk (S.faceEdge_ends f 1), S.endsVec_of_mk (S.faceEdge_ends f 2),
    S.endsVec_of_mk (S.faceEdge_ends f 3)]
  set c := S.faceCorner f
  have h : ((0 : Fin 4) + 1, (1 : Fin 4) + 1, (2 : Fin 4) + 1, (3 : Fin 4) + 1) = (1, 2, 3, 0) := by
    decide
  simp only [Prod.mk.injEq] at h
  rw [h.1, h.2.1, h.2.2.1, h.2.2.2]
  exact quad_sum _ _ _ _

theorem bd1_comp_bd2 : S.bd1 ∘ₗ S.bd2 = 0 := by
  refine LinearMap.ext fun x => ?_
  simp only [LinearMap.comp_apply, LinearMap.zero_apply, bd2, LinearMap.coe_mk, AddHom.coe_mk,
    map_sum, map_smul, bd1_single, Fintype.sum_prod_type, ← Finset.smul_sum, sum_endsVec_face,
    smul_zero, Finset.sum_const_zero]

/-! ### The 2-chains without boundary -/

theorem eq_of_bd2_eq_zero {x : F → ZMod 2} (hx : S.bd2 x = 0) {f g : F}
    (h : ∃ i j, S.faceEdge f i = S.faceEdge g j) : x f = x g := by
  obtain ⟨i, j, hij⟩ := h
  by_cases hfg : (f, i) = (g, j)
  · simp only [Prod.mk.injEq] at hfg; rw [hfg.1]
  have key : ∀ p q : ZMod 2, p + q = 0 → p = q := by decide
  have hsum := congrFun hx (S.faceEdge g j)
  rw [bd2_apply] at hsum
  obtain ⟨a, b, hab, hset⟩ := Finset.card_eq_two.1 (S.two_slots (S.faceEdge g j))
  have hf : (f, i) ∈ ({a, b} : Finset (F × Fin 4)) := by rw [← hset]; simp [hij]
  have hg : (g, j) ∈ ({a, b} : Finset (F × Fin 4)) := by rw [← hset]; simp
  rw [hset, Finset.sum_pair hab] at hsum
  simp only [Finset.mem_insert, Finset.mem_singleton] at hf hg
  rcases hf with hf | hf <;> rcases hg with hg | hg
  · exact absurd (hf.trans hg.symm) hfg
  · subst hf hg; exact key _ _ hsum
  · subst hf hg; exact (key _ _ hsum).symm
  · exact absurd (hf.trans hg.symm) hfg

/-- A 2-chain without boundary is constant. -/
theorem ker_bd2_le : LinearMap.ker S.bd2 ≤ Submodule.span (ZMod 2) {fun _ => 1} := by
  intro x hx
  rcases isEmpty_or_nonempty F with hF | ⟨⟨f₀⟩⟩
  · rw [Subsingleton.elim x 0]; exact Submodule.zero_mem _
  refine Submodule.mem_span_singleton.2 ⟨x f₀, funext fun f => ?_⟩
  have : x f₀ = x f := by
    induction S.faceConnected f₀ f with
    | refl => rfl
    | tail _ hab ih => exact ih.trans (S.eq_of_bd2_eq_zero hx hab)
  simp [this]

theorem finrank_ker_bd2_le : Module.finrank (ZMod 2) (LinearMap.ker S.bd2) ≤ 1 :=
  (Submodule.finrank_mono S.ker_bd2_le).trans ((finrank_span_le_card _).trans (by simp))

/-- `∂₂` has rank at least `F − 1`. -/
theorem card_faces_le :
    Fintype.card F ≤ Module.finrank (ZMod 2) (LinearMap.range S.bd2) + 1 := by
  have h := LinearMap.finrank_range_add_finrank_ker S.bd2
  rw [Module.finrank_fintype_fun_eq_card] at h
  have := S.finrank_ker_bd2_le
  omega

/-! ### The 1-chains -/

/-- A chain of edges from `v₀` to `v` has boundary `v₀ + v`. -/
theorem exists_bd1_eq (v₀ v : V) : ∃ y, S.bd1 y = Pi.single v₀ 1 + Pi.single v 1 := by
  induction S.connected v₀ v with
  | refl => exact ⟨0, by rw [map_zero, add_self_vec]⟩
  | @tail b c _ hbc ih =>
    obtain ⟨y, hy⟩ := ih
    obtain ⟨e, he⟩ := hbc
    refine ⟨y + Pi.single e 1, ?_⟩
    rw [map_add, hy, bd1_single, S.endsVec_of_mk he]
    calc _ = Pi.single v₀ 1 + Pi.single c 1 + (Pi.single b (1 : ZMod 2) + Pi.single b 1) := by abel
      _ = _ := by rw [add_self_vec, add_zero]

/-- Every 0-chain with an even number of vertices is a boundary. -/
theorem ker_total_le (v₀ : V) : LinearMap.ker (total V) ≤ LinearMap.range S.bd1 := by
  intro z hz
  have hz' : ∑ v, z v = 0 := hz
  have hsum : z = ∑ v, z v • (Pi.single v 1 + Pi.single v₀ 1) := by
    funext w
    simp [Finset.sum_apply, Pi.single_apply, Finset.sum_add_distrib, hz']
  rw [hsum]
  refine Submodule.sum_mem _ fun v _ => Submodule.smul_mem _ _ ?_
  obtain ⟨y, hy⟩ := S.exists_bd1_eq v₀ v
  exact ⟨y, hy.trans (add_comm _ _)⟩

/-- `∂₁` has rank at least `V − 1`. -/
theorem card_vertices_le :
    Fintype.card V ≤ Module.finrank (ZMod 2) (LinearMap.range S.bd1) + 1 := by
  rcases isEmpty_or_nonempty V with hV | ⟨⟨v₀⟩⟩
  · simp
  have h := LinearMap.finrank_range_add_finrank_ker (total V)
  rw [Module.finrank_fintype_fun_eq_card] at h
  have h1 : Module.finrank (ZMod 2) (LinearMap.range (total V)) ≤ 1 :=
    (Submodule.finrank_le _).trans (by simp)
  have h2 := Submodule.finrank_mono (S.ker_total_le v₀)
  omega

/-! ### Every cycle bounds -/

theorem range_bd2_le : LinearMap.range S.bd2 ≤ LinearMap.ker S.bd1 := by
  rintro _ ⟨x, rfl⟩
  exact LinearMap.congr_fun S.bd1_comp_bd2 x

/-- With Euler characteristic `2`, every cycle is a sum of face boundaries. -/
theorem range_bd2_eq_ker_bd1
    (hχ : (Fintype.card V : ℤ) - Fintype.card E + Fintype.card F = 2) :
    LinearMap.range S.bd2 = LinearMap.ker S.bd1 := by
  refine Submodule.eq_of_le_of_finrank_le S.range_bd2_le ?_
  have h := LinearMap.finrank_range_add_finrank_ker S.bd1
  rw [Module.finrank_fintype_fun_eq_card] at h
  have := S.card_faces_le
  have := S.card_vertices_le
  omega

theorem total_bd2 (x : F → ZMod 2) : total E (S.bd2 x) = 0 := by
  have h4 : ∀ a : ZMod 2, (4 : ℕ) • a = 0 := by decide
  simp only [bd2, LinearMap.coe_mk, AddHom.coe_mk, map_sum, map_smul]
  simp [total, Fintype.sum_prod_type, h4]

/-- With Euler characteristic `2`, every cycle has an even number of edges. -/
theorem total_eq_zero_of_bd1
    (hχ : (Fintype.card V : ℤ) - Fintype.card E + Fintype.card F = 2)
    {y : E → ZMod 2} (hy : S.bd1 y = 0) : total E y = 0 := by
  have : y ∈ LinearMap.range S.bd2 := by rw [S.range_bd2_eq_ker_bd1 hχ]; exact hy
  obtain ⟨x, rfl⟩ := this
  exact S.total_bd2 x

/-! ### The colouring -/

/-- A closed surface made of quads with Euler characteristic `2` has a bipartite
1-skeleton: some colouring of the vertices differs at the two ends of every edge. -/
theorem bipartite_of_euler
    (hχ : (Fintype.card V : ℤ) - Fintype.card E + Fintype.card F = 2) :
    ∃ c : V → ZMod 2, ∀ e, c (S.ends e).1 ≠ c (S.ends e).2 := by
  rcases isEmpty_or_nonempty V with hV | ⟨⟨v₀⟩⟩
  · exact ⟨fun _ => 0, fun e => isEmptyElim (S.ends e).1⟩
  choose y hy using S.exists_bd1_eq v₀
  refine ⟨fun v => total E (y v), fun e => ?_⟩
  rcases hab : S.ends e with ⟨a, b⟩
  intro hc
  have hcyc : S.bd1 (y a + Pi.single e 1 + y b) = 0 := by
    rw [map_add, map_add, hy, hy, bd1_single, S.endsVec_of_mk (a := a) (b := b) (by rw [hab])]
    exact tri_sum _ _ _
  have h := S.total_eq_zero_of_bd1 hχ hcyc
  simp only [map_add] at h
  simp only at hc
  rw [hc] at h
  have : total E (Pi.single e (1 : ZMod 2)) = 1 := by simp [total]
  rw [this] at h
  revert h; generalize total E (y b) = p; revert p; decide

end QuadSurface

end TurnsOfATMesh
