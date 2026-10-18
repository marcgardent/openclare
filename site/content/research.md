# Closing Edge Loops at Quantization Time

<p class="subtitle">Helix constraints for T-mesh quantization. Research note of the openclare project, 18 October 2026. A draft, not peer reviewed.</p>

<p class="actions"><a class="button primary" href="../../docs/research/closing-helices/paper.pdf">Read the note (PDF)</a> <a class="button" href="../../docs/research/closing-helices/paper.tex">LaTeX source</a> <a class="button" href="theory.md">The theory in plain words</a></p>

## Abstract

Automatic quad remeshers produce edge loops that wind round a part of the shape as helices instead of closing into rings, a defect that makes such meshes awkward to edit by hand. Bommes et al. identified these *q-helices* and remove them after meshing, by grid-preserving operators applied to the finished quad mesh. We take the opposite route and prevent them where the integer structure of the mesh is decided: in the quantization of a T-mesh. A straight walk across the rectangles of a T-mesh that comes back to its start defines a *turn*; we show that the pitch of the corresponding helix is a linear form in the integer lengths of the arcs, so that closing the helix is a linear constraint on the quantization. We add such constraints to a minimum-deviation-flow quantizer through a least-change projection and targeted repair walks, select among a few ways of finding turns by the share of spiral edges they leave, and guard against folds and budget drift. On our bench the share of edges on spiral loops at 3,000 quads drops from 16.7% to 6.0% on twelve models; on a human body remeshed at the face count of the hand-made base mesh it drops from 12.1% to 1.1%, below the 2.5% of the hand-made mesh.

We relate turns to the twist of flat cylinders in the theory of Jenkins–Strebel differentials, show with the holonomy of the flat metric that a pair of singularities of valences three and five joined by a staircase of arcs between two turns forbids closing both, and prove that on a sphere the shift of a turn has the parity of its length, so that a helix of odd pitch closes only if its length changes. Exact computations on the lattice of closed quantizations of the human body show that whole lengths closing nearly every ring exist, but not at the size asked for. Tested on the T-mesh of that body, the propositions hold on every turn and annulus they apply to; the conflicts between two turns are not, as we had guessed, pairs on one trace, but staircases between a pair of valences three and five, exchanges of two vertices of valence three, or caps, and the one turn that cannot close on its own is held open by a separatrix that winds round a wrist. On seven other models the Burgers sum reproduces the shift of 3,600 of 3,604 rings, yet moving a singular vertex in the parametrization barely moves the shift: the map moves the other cone points instead.

With Fox's free differential calculus we write any combination of shifts as a sum over the singular vertices of twisted winding numbers times Burgers vectors; winding numbers alone do not suffice, on the body as in principle, but they do modulo two, which gives a checkerboard rule for any combination of turns. Turned around, the obstructions bound from below the number of helices of any quantization: on the body, the separation of the singularities alone forces one to nine of the turns held for closing to stay helices, and this is attained, where the engine leaves 13 to 123. We list the open problems that a complete treatment would need to solve.

## Machine-checked proofs

The algebraic cores of the note's results (its Sections 3 and 8 to 10) are checked by the [Lean 4](https://lean-lang.org/) proof assistant with the Mathlib library: **187 theorems**, in a Lake project beside the note. Every theorem depends only on Lean's three standard axioms (`propext`, `Classical.choice`, `Quot.sound`); there is no `sorry`.

**What is checked.** Among others: the shift of a turn is a linear form in the arc lengths; the complete walk of the strips finds every strip; Gauss–Bonnet for turns; the Burgers sum and the order of first visit; annuli, dislocations, staircases and exchanges of two vertices; the checkerboard rule and the parity of a turn on a sphere (with a second proof that came out of the formalization); the Fox–Burgers formula; the handshake lemma and the "free arcs" proposition; Farkas certificates for turns that can never close; and the theorem on forced helices. Several results are proved more generally than the note states them: for any rotations and dimensions, and for fields of any order (triangle and hexagon meshes).

**What is assumed.** The topological facts the note relies on are not formalized; each one is an explicit hypothesis of the theorem that uses it, never hidden. They include that a simple closed curve on a sphere bounds a disc (Jordan–Schoenflies), the developing map and holonomy as a homomorphism of the fundamental group, and two standard graph facts. Strips on surfaces with borders, and every *measured* statement, are outside the formalization.

**What it found.** Formalizing the note found eleven gaps or corrections, none of them a sign or convention error: a misplaced factor in one Fox-calculus step (with the right conclusion), a theorem that needs neither convexity nor whole lengths, a parity statement that holds on a closed sphere only (it fails with two or more borders), and propositions that are stronger than stated. They are listed in the [proofs' README](../../docs/research/closing-helices/lean/README.md).

To check the proofs yourself, from the root of the repository:

```sh
make proofs NOTE=closing-helices
```

The first run downloads the Lean toolchain and Mathlib already compiled, about 10 GB in all; after that, building the note's own files takes about 40 seconds and 1.3 GB of memory.

## AI assistance

The note states how the work was divided between its human author and an AI system. In its own words:

> This note and the software it describes were made by a human author working with an AI system. Following the practice recommended by ACM and by Nature, we state how the work was divided. No AI system is an author of this note.
>
> **The human author** chose the problem and its aims: edge loops that close into rings, the quad sphere with the eight poles of a cube as a target, and, as an artist, a mesh with few singular vertices and few spirals rather than perfectly square quads, always made of quads only. He set the direction of the research and the order of the work, decided which results to pursue, which changes to merge into the engine and which to drop, authorized the tools (LaTeX, Lean and Mathlib), and judged the results through the measurements, pictures and drafts presented to him.
>
> **The AI system**, Claude Opus 5.5 (Anthropic), used through Claude Code in October 2026, with several sub-agents of the same model working in parallel, did the rest. It wrote the changes to the engine and the experiment scripts, ran the experiments and the bench, carried out the exact computations (SymPy, HiGHS through SciPy), searched the literature, stated and proved the propositions, wrote the Lean formalization and drafted the text of this note. References were checked for existence, and the note attached to each entry of the bibliography says how much of the work was read: the abstract only, a part, or all of it.
>
> **What was checked, and how.** The algebraic cores of Sections 8 to 10 are machine-checked in Lean, using only Lean's standard axioms. The topological statements that Lean takes as hypotheses, and the proofs given only on paper, were checked by AI agents rereading them and testing them numerically on the data, not by a human expert. Every measured figure can be reproduced from the scripts and branches named in the note. The note is a draft and has not been peer reviewed. Errors remain possible, and the human author, who publishes the note, is responsible for its content.

This website was built the same way: its pages were drafted by the AI system and published by the author of the project.
