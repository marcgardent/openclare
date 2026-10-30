/-
Theorem "Parity of a turn on any surface", algebraic core. The parity of a
translation, `π(t) = t₁ + t₂ mod 2` (`colour`), is a homomorphism on the motions
with quarter-turn linear parts and whole translations: `π` of a composite is the
sum of the `π` of its factors. It vanishes on the rotations about whole points
(the loops round the cone points), it is `L mod 2` on the translation by `L`
along an axis (a border component without corner), and `α + s mod 2` on the
translation of a turn. So `Ω = π ∘ ĥ` is a homomorphism of the fundamental group
to `ℤ/2` that kills the loops round the cone points. That it then factors through
`H₁(Σ; ℤ/2)`, and that its value on a class is the parity of the length of any
cycle of the quad graph in that class, is the topology the note assumes.
-/
module

public import TurnsOfATMesh.Checkerboard

@[expose] public section

set_option linter.unusedSectionVars false

namespace TurnsOfATMesh

open Motion

/-- **`π` is a homomorphism**: the translation of `f ∘ g` has the colour of
`f.trans` plus that of `g.trans`, the linear part of `f` being a quarter-turn. -/
theorem colour_comp_trans (f g : Motion GaussianInt GaussianInt) (hf : IsQuarter f.lin) :
    colour (comp f g).trans = colour f.trans + colour g.trans := by
  rw [comp_trans, colour_add, smul_eq_mul, colour_quarter_mul hf]

/-- Along a list of motions with quarter-turn linear parts, `π` of the composite
is the sum of the `π` of the translations. -/
theorem colour_composeAll_trans (fs : List (Motion GaussianInt GaussianInt))
    (hq : ∀ f ∈ fs, IsQuarter f.lin) :
    colour (composeAll fs).trans = (fs.map fun f => colour f.trans).sum := by
  induction fs with
  | nil => simp [Motion.id, colour]
  | cons f fs ih =>
    have hlin : IsQuarter (composeAll fs).lin := by
      rw [composeAll_lin]
      apply IsQuarter.list_prod
      intro a ha
      rw [List.mem_reverse, List.mem_map] at ha
      obtain ⟨g, hg, rfl⟩ := ha
      exact hq g (List.mem_cons_of_mem f hg)
    rw [composeAll_cons, colour_comp_trans _ _ hlin, ih (fun g hg => hq g (List.mem_cons_of_mem f hg)),
      List.map_cons, List.sum_cons, add_comm]

/-- `π` vanishes on a rotation by a quarter-turn about a whole point: the loops
round the cone points. -/
theorem colour_rotAbout_trans {a : GaussianInt} (ha : IsQuarter a) (q : GaussianInt) :
    colour (rotAbout a q).trans = 0 := by
  rw [rotAbout_trans, colour_sub, smul_eq_mul, colour_quarter_mul ha, CharTwo.add_self_eq_zero]

/-- On the translation by `L` along an axis of the field, the holonomy of a
border component without corner, `π` is `L mod 2`. -/
theorem colour_border (L : ℤ) (e : GaussianInt) (he : IsQuarter e) :
    colour (translation (L * e) : Motion GaussianInt GaussianInt).trans = (L : ZMod 2) := by
  unfold IsQuarter at he
  rcases he with rfl | rfl | rfl | rfl <;> simp [colour, gi]

/-- On the translation `(α, s)` of a turn, read in the frame of its first side,
`π` is `α + s mod 2`: Theorem "Parity of a turn on any surface" reads
`α + s ≡ Ω([γ̄])`. -/
theorem colour_turn (α s : ℤ) : colour (⟨α, s⟩ : GaussianInt) = (α : ZMod 2) + s := by
  simp [colour]

/-- **The core of the theorem.** The holonomy of a loop is a composite of motions
with quarter-turn linear parts: rotations about whole points for the loops round
the cone points, which contribute nothing, translations along axes for the border
components, which contribute their lengths, and any others for the handles. The
parity of its translation is the sum of these contributions. -/
theorem parity_any_surface (fs : List (Motion GaussianInt GaussianInt))
    (hq : ∀ f ∈ fs, IsQuarter f.lin) (α s : ℤ) (hT : (composeAll fs).trans = ⟨α, s⟩) :
    (α : ZMod 2) + s = (fs.map fun f => colour f.trans).sum := by
  rw [← colour_turn, ← hT, colour_composeAll_trans fs hq]

end TurnsOfATMesh
