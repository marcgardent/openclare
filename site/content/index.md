<div class="hero">
<div class="hero-text">
<p class="kicker">Free add-on for Blender · open source</p>
<h1>Clean retopology, in one click, in Blender.</h1>
<p class="lede">Quads that follow the shape, and edge loops that close round it. Free and open source.</p>
<p class="actions"><a class="button primary" href="get.md">Download for Blender</a> <a class="button" href="#see-it-in-3d">See it in 3D</a></p>
</div>
<figure class="pair"><div class="pair-images"><img src="../../docs/models/img/human_male__source.webp" alt="A sculpted human body, {{shot:human_male:Source}}" width="600" height="600"><img src="../../docs/models/img/human_male__openclare.webp" alt="The same body remeshed by openclare, {{shot:human_male:openclare}}" width="600" height="600"></div><figcaption><span>{{shot:human_male:Source}}</span><span class="to">→ {{shot:human_male:openclare}}</span></figcaption></figure>
</div>

## What it does for you

Select a mesh, ask for a number of quads, press *Remesh to Quads*. The original is left as it is.

<div class="cards uses">
<div class="card" data-art="sculpt"><strong>A sculpt ready to animate</strong><span>Quads with few poles, and loops round the eyes, the mouth and the joints, drawn on the model where you want them.</span></div>
<div class="card" data-art="mechanical"><strong>A mechanical part with crisp edges</strong><span>Hard edges and marked sharp edges kept as lines of edges, creased for a Subdivision Surface.</span></div>
<div class="card" data-art="scan"><strong>A scan or an AI mesh made usable</strong><span>A clean, closed surface of quads from a noisy one.</span></div>
<div class="card" data-art="detail"><strong>Detail where it matters</strong><span>Smaller quads on the face and the hands, painted or taken from the shape.</span></div>
<div class="card" data-art="seams"><strong>Seams where you want them</strong><span>Curves drawn on the model become seams, ready for the UVs.</span></div>
<div class="card" data-art="triangles"><strong>Triangles too</strong><span>Even triangles for cloth simulation, or Blender's own Decimate for printing and collisions.</span></div>
</div>

## Loops that close

Select an edge loop with Alt-click: it should go round the arm once and close. Automatic remeshers often make it a spiral instead, winding from the shoulder to the wrist. openclare closes those loops, and keeps few poles rather than perfectly square quads.

![The male body at 10,590 quads, by openclare without closing loops, by openclare with closing, by hand and by QuadriFlow](../../docs/research/closing-loops/figures/male-overview.png)

*A human body at 10,590 quads: by openclare without closing loops, with closing, by hand and by QuadriFlow. Spiral loops in vermillion, closed loops in dark blue.*

## See it in 3D

Turn round the results of openclare and of two other remeshers, quad by quad.

<div class="viewer" data-models="ogre pig part stonemill human_male"></div>

## Measured in public

<!-- section: night -->

Every version runs on a public bench of {{count:all}} models, against QuadriFlow, built into Blender, and AutoRemesher.

<div class="stats">
<div class="stat"><span class="value">{{mark:3000:organic:openclare}}<small> / 100</small></span><span class="label">average mark on {{count:organic}} organic models at 3,000 quads; AutoRemesher {{mark:3000:organic:autoremesher}}, QuadriFlow {{mark:3000:organic:quadriflow}}</span></div>
<div class="stat"><span class="value">{{mark:3000:mechanical:openclare}}<small> / 100</small></span><span class="label">average mark on the mechanical parts at 3,000 quads</span></div>
</div>

Every model, every mark, and how they are made: the [results](results.md).

## Get it

One archive for Linux, Windows or macOS, installed from Blender's preferences. Nothing else to install, nothing downloaded while it runs.

<p class="actions"><a class="button primary" href="get.md">Download for Blender</a></p>

## Read on

<div class="cards">
<a class="card" href="articles.md"><strong>Articles</strong><span>The retopology tools of Blender in 2026, the results of the bench, the theory and the research.</span></a>
<a class="card" href="../../docs/GLOSSARY.md"><strong>Glossary</strong><span>Poles, edge loops, spirals and the other words of retopology, with pictures.</span></a>
<a class="card" href="get.md"><strong>Get it</strong><span>The add-on, setting by setting, and the command line.</span></a>
</div>
