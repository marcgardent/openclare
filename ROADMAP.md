# Roadmap

*[Version française](ROADMAP.fr.md)*

What is left to do, feature by feature. What each feature does and how far it can be relied on
is in [Features](docs/FEATURES.md); the tickets are the
[issues](https://github.com/marcgardent/openclare/issues), cited by their number.

Rewritten on 31 October 2026 from the features as they are. The rule since 8 October 2026
stands: what improves the mesh comes before side features.

## 1. Experimental to stable

Each line is done when its feature meets the rule of [FEATURES.md](docs/FEATURES.md): tested,
true on the bench or on a known case, no issue *high* or *medium* left open against it. In this
order:

| Order | Features | Done when | Issues |
|---|---|---|---|
| 1 | *Ask For: Sizes*, *Analyse*, *Show Sizes*, *Density from Shape*; `--largest`, `--smallest`, `analyse` | the sizes proposed come from the user or from a measured rule, not a constant; the fingers of a body keep the quads round them that *Quads Around Thin Parts* asks | #253 |
| 2 | *Draw Curve*, *Cut Volume*, the marks of a curve; `--curves` | a curve drawn where quads can follow it is kept, every time, on a cube as on a sculpt; the sizes and the budget count the curves | #195, #196, #44 |
| 3 | *Quads Around Thin Parts*; `--quads-around` | the thin parts of the bench keep their volume at the three budgets without the bound on size going wrong | #38, #18 |
| 4 | The poles of the project and the *Poles* tool; `--poles`, `--poles-out` | measured on the bench, tried by artists, their remarks settled | #249 |
| 5 | *Follow Curvature*, *Fewer Irregular Vertices*, *Volume Fit*; the presets shipped | defaults tuned on the corpus; no reach folds a quad; the volume kept after subdivision | #13, #21, #33, #72, #39 |
| 6 | *Output: Triangles*, for *Simulation*; `--method triangles` | no triangle cuts through another on the models of [TRIANGLES.md](docs/TRIANGLES.md) | #226 |
| 7 | *Materials*, the lines between them; `--groups` | the borders between materials kept on a real model, the author's car first | #47 |
| 8 | The add-on on Windows and macOS | run in Blender on each, macOS notarized | #110 |

The other experimental features are tools to look into the chain (`field`, `tmesh`,
`quantize`, Rerun and the like) or research tracks (`--quality slow`): they stay experimental,
and change as the chain does.

## 2. Limits of the chain

What bounds every feature (listed in [FEATURES.md](docs/FEATURES.md#limits-of-the-chain)).
Real models look like these inputs more often than like a clean sculpt.

| Order | Track | Done when | Issues |
|---|---|---|---|
| 1 | **Small pieces** | every piece of the bench through the main chain, none empty | #34 |
| 2 | **What a mesh still loses of the shape**: deep pockets, details finer than the work mesh | the Hausdorff distance of the ogre at 3000 quads under 10 per mille | #36, #48 |
| 3 | **Dense sculpts** | a sculpt of millions of triangles remeshed in a time an artist waits for | #45 |
| 4 | **Robustness**: slivers, inversions, a bound on time | no input of the bench, or made to break it, gives an invalid mesh or runs past a stated limit | #31, #10, #27, #194 |
| 5 | **Structure**: narrow strips, base complex | fewer patches and irregular vertices at the three budgets | #5, #20 |

## 3. New features

Each to be specified and validated when its turn comes.

| Feature | What it is | Issues |
|---|---|---|
| **Quad Shape menu** | a sphere, cylinder or disc made of quads with their ideal poles | #159 |
| **Baking the sculpt onto the result** | normal map, height and ambient occlusion, read through the engine's own correspondence with the sculpt | #51 |
| **Improving a mesh from a dry run** | a run marked, the best of several kept, a part run again where it went wrong | #40 |
| **Clean triangles for 3D printing** | the same chain with six-fold stars instead of crosses | #70 |

## 4. Research

The papers, the Lean proofs and the experiments behind the quantization are on the
[research page](site/content/research.md) and in their issues (label *research-note*). What
they find comes into the chain through section 1 or 2, measured on the bench first.

## What is not planned

- The patch chain for hard surfaces (QuadWild): walls in the T-mesh do the job (#30).
- An exact integer-grid map and extraction: the direct route from the T-mesh replaced it (#1).
- Embedding the Rerun viewer in what is distributed: it stays a tool for those who work on the
  chain.
