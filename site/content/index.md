# openclare

<div class="hero">
<p class="lede">Automatic quad retopology for Blender. A mesh of triangles in, a mesh of quads out, its edges following the shape and its edge loops closing round it.</p>
<p>openclare is a field-aligned quad remeshing engine: a cross field follows the curvature, a T-mesh lays out the patches, and an integer quantization decides how many quads go where. The engine is written in Rust; it ships as a Blender add-on with the engine inside, and as a command-line tool. Open source, GPL-3.0-or-later.</p>
<p class="actions"><a class="button primary" href="get.md">Get it</a> <a class="button" href="results.md">See the results</a> <a class="button" href="theory.md">How it works</a></p>
</div>

## Loops that close

An artist selects an edge loop with one Alt-click and expects it to go round an arm once and close. Automatic remeshers often make it a helix instead, winding from shoulder to wrist. openclare closes those loops while it decides the integer structure of the mesh, not afterwards.

![A cylinder remeshed by openclare, before and after closing its loops](../../docs/glossary/cylinder_result.png)

*A cylinder, hard-surface preset. Before: spiral loops (red), about 700 patches. After: every loop is a ring, and the layout is that of a cube, 6 patches.*

![The male body before and after closing loops](../../docs/glossary/male_result.png)

*A human body at 10,590 quads, the face count of the base mesh an artist made for it. Spiral loops in yellow to red, closed loops in blue.*

## The numbers

<div class="stats">
<div class="stat"><span class="value">4.5%</span><span class="label">of the edges on spiral loops, 12 models at three budgets near 3,000 quads; 16.7% with no closing</span></div>
<div class="stat"><span class="value">1.2–1.6%</span><span class="label">spiral edges on the male body, against 2.5% for the mesh made by hand (12.1% before closing)</span></div>
<div class="stat"><span class="value">{{mark:3000:organic:openclare}} / 100</span><span class="label">average mark on {{count:organic}} organic models at 3,000 quads; AutoRemesher {{mark:3000:organic:autoremesher}}, QuadriFlow {{mark:3000:organic:quadriflow}}</span></div>
<div class="stat"><span class="value">{{mark:3000:mechanical:openclare}} / 100</span><span class="label">average mark on the mechanical parts at 3,000 quads</span></div>
<div class="stat"><span class="value">187</span><span class="label">theorems of the research note checked by the Lean proof assistant</span></div>
</div>

Measured by the project's bench, the night of 18 October 2026, on {{count:all}} models; how the marks are made is on the [results](results.md) page. The marks weigh validity, alignment on the shape, the shape of the quads, the distance to the model, irregular vertices, the structure of the layout, closed edge loops and the number of quads asked for.

## Three remeshers side by side

<figure class="wide"><picture><source media="(prefers-color-scheme: dark)" srcset="../../docs/images/dashboard/gallery-dark.webp"><img src="../../docs/images/dashboard/gallery-light.webp" alt="openclare, QuadriFlow and AutoRemesher on the ogre, the pig, a mechanical part and a generated building, each with its number of quads and its mark out of 100" loading="lazy"></picture><figcaption>Each row at the same number of quads, asked of every remesher. The stone mill is from the <em>Free Windmills Set</em> by LapaModels, CC BY 4.0.</figcaption></figure>

<figure class="wide"><picture><source media="(prefers-color-scheme: dark)" srcset="../../docs/images/dashboard/marks-dark.svg"><img src="../../docs/images/dashboard/marks-light.svg" alt="Average mark out of 100 of the three remeshers over the organic models, the mechanical parts and the assembled and generated models, at 1,000, 3,000 and 8,000 quads asked for" loading="lazy"></picture><figcaption>Average marks over the bench at 1,000, 3,000 and 8,000 quads asked for.</figcaption></figure>

## Simple rather than regular

An artist would rather have few poles and few spirals than perfectly square quads. A *simplicity* setting removes nearby pairs of poles and accepts quads a little less square in exchange. openclare makes quads only: never a triangle, never an n-gon.

![The male body remeshed at simplicity 0, 0.25, 0.5 and 1](../../docs/glossary/simplicity.png)

*The male body at simplicity 0, 0.25 (the presets), 0.5 and 1: 303, 259, 213 and 172 poles.*

## Read on

<div class="cards">
<a class="card" href="theory.md"><strong>Theory</strong><span>Why edge loops spiral, and the rules that close them, in plain words.</span></a>
<a class="card" href="../../docs/GLOSSARY.md"><strong>Glossary</strong><span>Poles, cross fields, T-meshes, turns and shifts, with pictures.</span></a>
<a class="card" href="results.md"><strong>Results</strong><span>Marks per budget and per kind of model, and one page per model.</span></a>
<a class="card" href="research.md"><strong>Research note</strong><span>The paper, its machine-checked proofs, and how the work was done.</span></a>
<a class="card" href="get.md"><strong>Get it</strong><span>The Blender add-on and the command line.</span></a>
</div>
