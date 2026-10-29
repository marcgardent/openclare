# Results

<p class="subtitle">Marks from 0 to 100 for every remesher on every model of the bench, at 1,000, 3,000 and 8,000 quads asked for. From the night bench of 18 October 2026.</p>

The bench runs openclare and two other remeshers on {{count:all}} models: organic shapes (sculpts, scans, characters), mechanical parts, two calibration shapes, and an *extra* group of assembled and generated models that are not clean surfaces. Every model has its page, with pictures of what each remesher makes of it, where its licence allows.

The remeshers: **openclare** as the Blender add-on runs it; **QuadriFlow**, built into Blender; **AutoRemesher** 1.2.0 by Jeremy Hu. Marks are absolute: a remesher keeps its mark whichever others it is compared with.

## Average marks

{{averages}}

## Model by model

<div class="viewer" data-models="all"></div>

{{models}}

{{withheld}}

## How the marks are made

| Criterion | Full marks for | 100 at | 0 at | Weight |
|---|---|---|---|--:|
| Validity | manifold, no flipped or collapsed face, no hole the source lacks; all quads, or nearly | every run valid | no run valid | 25 |
| Alignment | edges along the principal curvature directions | 0° off | 22.5° off (edges at random) | 20 |
| Quad shape | corners at right angles | 0° mean deviation | 20° | 15 |
| Fidelity | no point of the source far from the mesh, thin parts kept | Hausdorff distance 0; thin parts on the mesh | 5% of the bounding diagonal; thin parts half a quad edge away | 15 |
| Regularity | no irregular vertex beyond what the topology imposes | none | 10 per 100 quads | 10 |
| Structure | few patches in the base complex: edge loops that run long | — | one patch per quad | 5 |
| Edge loops | loops that close round the shape, none that spiral round it | 2% of the edges or fewer on spirals; a quarter of the loops closed | a fifth of the edges on spirals; no loop closed | 15 |
| Budget | the number of quads asked for | exact | 25% off | 10 |
| Speed | an answer within a second | 1 s | 60 s (logarithmic) | 0 |

The overall mark is the weighted mean of the criteria that exist for the model: a sphere has no direction to align with, a mesh with open borders has no topological minimum. Edge loops averages its two halves, and is left out where the mesh is not a manifold. Speed is shown in the full report but does not count: it is not a matter of quality, and the timings were taken on a loaded machine. A remesher that gave no mesh at all scores 0. The *Quads* column is not a mark: it is how many quads the mesh has, the mean over the models.

The full report, with every criterion for every model and remesher, is [bench/results/SCORES.md](../../bench/results/SCORES.md) in the repository; the authors, sources and licences of the models are in [CREDITS.md](../../CREDITS.md).
