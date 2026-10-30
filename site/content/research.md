# Research

<p class="subtitle">This work began as a programming project, openclare, an open-source quad remesher, and became research when engineering alone could no longer tell which results could be guaranteed and which computations could be saved.</p>

<article class="paper" aria-labelledby="paper-a">
<div class="paper-text">
<p class="tags"><span class="tag tag-ink">Paper A · applied</span> <span class="tag tag-teal">Draft · not peer reviewed</span></p>
<h2 id="paper-a">Closing Edge Loops at Quantization Time: Helix Constraints for T-Mesh Quantization</h2>
<p>Automatic quad remeshing often produces edge loops that wind round a part of the shape as helices instead of closing into rings. Bommes et al. remove such <em>q-helices</em> from a finished mesh; we prevent them where the integer structure of the mesh is decided, in the quantization of a T-mesh.</p>
<p class="actions"><a class="button primary" href="../../docs/research/closing-loops/paper.pdf">PDF · English · 18 p.</a> <a class="button" href="../../docs/research/closing-loops/paper.fr.pdf" hreflang="fr" lang="fr">PDF · français · 20 p.</a> <a class="button plain" href="../../docs/research/closing-loops/paper.tex">LaTeX source</a></p>
<p class="note">The French version is a best-effort translation; the English one is authoritative.</p>
</div>
<figure><img src="../../docs/research/closing-loops/figures/male-overview.png" alt="Figure 3 of paper A: the male body at 10,590 quads asked for, by openclare without closing, by openclare with closing, by hand and by QuadriFlow, spiral loops in vermillion" loading="lazy"><figcaption>Figure 3 · spiral loops in vermillion, closed loops in dark blue</figcaption></figure>
</article>

<article class="paper" aria-labelledby="paper-b">
<div class="paper-text">
<p class="tags"><span class="tag tag-ink">Paper B · theory</span> <span class="tag tag-teal">Draft · not peer reviewed</span></p>
<h2 id="paper-b">Turns of a T-Mesh: Obstructions to Closing Edge Loops</h2>
<p>In the quantization of a T-mesh, an edge loop closes into a ring when a <em>turn</em>, a straight walk across the patches that returns to its first side, has zero <em>shift</em>, an integer linear form in the arc lengths; otherwise it winds as a helix. We study why turns cannot all be closed.</p>
<p class="actions"><a class="button primary" href="../../docs/research/turns-of-a-t-mesh/paper.pdf">PDF · English · 26 p.</a> <a class="button" href="../../docs/research/turns-of-a-t-mesh/paper.fr.pdf" hreflang="fr" lang="fr">PDF · français · 29 p.</a> <a class="button plain" href="../../docs/research/turns-of-a-t-mesh/paper.tex">LaTeX source</a></p>
<p class="note">The French version is a best-effort translation; the English one is authoritative.</p>
</div>
</article>

## The method

The papers follow four rules taken from software engineering: how the work is checked, not how far it has gone. Where each rule stands changes from day to day; it is followed in the repository's [issues](https://github.com/marcgardent/openclare/issues).

**1. Lean first, prose last.** The proofs in the [Lean 4](https://lean-lang.org/) proof assistant, with the Mathlib library, are the reference, and the paper explains them. Only Lean's three standard axioms are used, and no proof is left open.

**2. Known cases, edge cases, random cases.** A statement is tried on examples whose answer is known, on its limits, and on random inputs. The random inputs are built without the engine and checked against the Lean definitions, statement by statement ([property tests](../../docs/research/turns-of-a-t-mesh/experiments/property/README.md)).

**3. Tests that must fail.** A theorem that still holds when the definition it certifies is altered says too little; a hypothesis that can be removed was not needed.

**4. Continuous integration.** Every change to the proofs or to the tests is rebuilt on a pinned Mathlib, and a proof left open fails the build.

**What is assumed.** The topological facts the note relies on are not formalized; each one is an explicit hypothesis of the theorem that uses it, never hidden: that a simple closed curve on a sphere bounds a disc (Jordan–Schoenflies), the developing map and holonomy as a homomorphism of the fundamental group, and two standard graph facts. Strips on surfaces with borders, and every *measured* statement, are outside the formalization.

**What it found.** Formalizing the note found eleven gaps or corrections, none of them a sign or convention error; among them, a parity statement that holds on a closed sphere only. They are listed in the [proofs' README](../../docs/research/turns-of-a-t-mesh/lean/README.md).

To check the proofs yourself, from the root of the repository:

```sh
make proofs NOTE=turns-of-a-t-mesh
```

The first run downloads the Lean toolchain and Mathlib already compiled, about 10 GB in all; after that, building the note's own files takes about 40 seconds and 1.3 GB of memory.

## AI assistance

Both papers and the software they describe were made by a human author working with an AI system, which is not an author. Following the practice recommended by ACM and by Nature, each paper states how the work was divided.

**The human author** chose the problem and its aims: edge loops that close into rings, the quad sphere with the eight poles of a cube as a target and, as an artist, few singular vertices and few spirals rather than perfectly square quads, quads only. He set the direction and the order of the work, decided which results to pursue and which changes to merge into the engine, authorized the tools (LaTeX, Lean and Mathlib), and judged the results through the measurements, pictures and drafts presented to him.

**The AI system**, Claude Opus 5.5 (Anthropic), used through Claude Code in October 2026 with several sub-agents in parallel, did the rest: the changes to the engine and the experiment scripts, the experiments and the bench, the exact computations, the search of the literature, the statements and proofs, the Lean formalization and the drafts of the text. References were checked for existence, and each entry of the bibliographies says how much of the work was read.

**What was checked, and how.** The algebraic cores are machine-checked in Lean, with Lean's standard axioms only. The topological hypotheses Lean takes as given, and the proofs given only on paper, were checked by AI agents rereading them and testing them on the data, not by a human expert. Every measured figure can be reproduced from the scripts and branches the papers name. Neither paper has been peer reviewed; errors remain possible, and the author, who publishes them, is responsible for their content.

This website was built the same way: its pages were drafted by the AI system and published by the author of the project.
