# Figures of paper B (turns of a T-mesh, the theory)

What each figure shows and the statement it illustrates (names and labels of preprint v1, whose
numbering paper B may change).

| File | What it shows | Statement |
|---|---|---|
| `holonomy.tex` | a cap D (six singular vertices, valences 3, 3, 3, 5, 3, 3) cut along a tree T into a disc and developed; a turn γ round it comes back one row lower; the Burgers vector B_D = Σ Q_i (I − R_i) p_i = (−11, −1) of the vertices of D has the length 11 of γ along and its shift −1 across; a parallel turn γ′ bounds with γ an annulus without singular vertex and has the same shift | Proposition 3, "The shift is set by the singularities" (`prop:burgers`), (a) and (c); Lemma 1 (index 4 in a disc) |
| `dislocation.tex` | a vertex of valence 3 and one of valence 5 between a ring γ₁ and a helix γ₂ of a band; (a) joined along one trace by a chain of two arcs, δ = 2; (b) by a staircase of two arcs; in both s(γ₂) − s(γ₁) = ℓ_b + ℓ_c = 2, so both are rings only if the two vertices merge | Proposition 4, "A dislocation between two turns" (`prop:dislocation`) (ii) and (i), (iii); Proposition 7, "Staircases" (`prop:staircase`) |
| `comb.tex` | the comb σ_a of an arc a: from the foot c₀ (valence 3) of one end of a, along a, to the foot c₁ (valence 5) of the other; developed along it, ⟨e_a, p_c₁ − p_c₀⟩ = ℓ_a = 9 | Section "Combs", the comb σ_a and Lemma "Combs" (`lem:comb`) |
| `separatrix.tex` | (a) sketch of a cylinder (a wrist) where a separatrix σ of a vertex v of valence 5 goes once round and stops on another separatrix τ of v, δ = 2 above v; (b) the band cut open along τ: the turn γ beside σ comes back δ = ℓ_e + ℓ_f = 2 higher, a ring only if σ came back onto v | Proposition 9, "A separatrix that winds" (`prop:winding`); the left wrist of the male body |

`comb.tex`, `separatrix.tex` (b) and `dislocation.tex` (b) draw one and the same T-mesh: the
staircase between the two vertices, the separatrix that winds and the comb of a are three readings
of the shift 2 of the upper turn.

The `.fr.tex` files are the French copies, for `paper.fr.tex`: same drawings, labels translated
after `docs/GLOSSARY.fr.md`.

## Using them

Each file is a `tikzpicture` only, to `\input` inside a `figure`. They need

```latex
\usepackage{tikz}
\usetikzlibrary{arrows.meta,decorations.pathreplacing}
```

and the French copies `\usepackage{icomma}` as well (the decimal comma), as in
`closing-loops/paper.fr.tex`. Sizes: `dislocation.tex` 15.6 cm by 6.1 cm (text width, two
panels), `separatrix.tex` 12.3 by 4.8, `holonomy.tex` 9.8 by 6.2, `comb.tex` 6.7 by 3.2.

## How to read them

Each surface is cut open into one plane chart, as in paper A's `turn.tex`: every node is at a
whole position, the faint grid is the quad mesh at these lengths, and the cut sides are glued in
pairs, so that a vertex may be drawn several times (its angle split between its copies). In
`dislocation.tex`, `comb.tex` and `separatrix.tex` the band is cut along a trace of the vertex of
valence 5; its right side is glued to its left side, at the same height below that vertex and two
units higher above it, and the empty corner next to the vertex of valence 3 is not part of the
surface: its two sides are glued by a quarter-turn about that vertex. In `holonomy.tex` the right
side is glued to the left side one unit lower, and the top of the block, from c₆ to the right, is
glued round the border of the L-shaped top (each c_i is drawn twice, once at its first visit p_i on
the walk round T). Hollow circles: where a turn starts and where it comes back. Colours as in paper
A: helices vermillion, rings dark blue; vertices of valence 3 orange triangles, of valence 5 purple
pentagons; what is measured (Burgers vector, gap δ, comb) green.

The comment at the top of each file gives the gluings and the arithmetic of every number drawn.
`check.py` (Python 3, standard library) builds the same surfaces and checks the valences, the
lengths and shifts of the turns (by walking the strips of the quad mesh), the Burgers sum, the
staircase, the winding separatrix and the comb identity:

```sh
python3 docs/research/closing-helices/figures/check.py
```
