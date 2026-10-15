# Contributing

## Building

```bash
cargo build --release
```

The Blender add-on, for the system it is built on (`dist/openclare-<version>-<system>.zip`):

```bash
make addon
```

For Windows, from Linux, with the Rust target `x86_64-pc-windows-gnu` and MinGW-w64 installed:

```bash
make addon-windows
```

The engine built that way can be tested under Wine:

```bash
make test-windows WINE=/path/to/wine64
```

`make help` lists every target.

## Testing

```bash
cargo test
```

The add-on, in a Blender without interface (it needs `blender` on the `PATH`):

```bash
make addon-test
```

The tests check invariants rather than frozen values: the Poincaré-Hopf theorem on the
singularities of the field, the Euler characteristic kept, a closed manifold mesh, vertices on the
surface, corners and hard edges found exactly, determinism. Where a test bounds a quality measure,
the comment says what was measured and when: the bound says when the engine got clearly worse,
not that it is good.

Tests are themselves put to the proof by breaking the engine on purpose, one small change at a
time, and counting the changes no test notices (ticket DT-058 says what is still unchecked). A new
test is worth its place when it notices a change the others let through.

## The bench

What each change does to the meshes is measured on a corpus of models against QuadriFlow, Quad
Remesher and AutoRemesher: [bench/README.md](bench/README.md) says how to run it, how a tool is
added (one function and one line in `bench/run.py`) and how a model is. The shortcuts:

| Command | What it does |
|---|---|
| `make quick MODELS=ogre,camera` | one run of the add-on per model, next to what the bench stored, in a few minutes |
| `python3 bench/run.py` | the full bench at one budget (`--target`, `--models`, `--tools`) |
| `make remeasure` | the meshes of the results measured again, without remeshing: after a change to the measures |
| `make scores` | the marks from 0 to 100 of every remesher on every model ([SCORES.md](bench/results/SCORES.md)) |
| `make pages` | one page of pictures and marks per model ([docs/models](docs/models/README.md)) |
| `make dashboard` | the pictures and the chart at the top of the README (`docs/images/dashboard`) |
| `make autoremesher` | installs AutoRemesher into `~/.local/bin` |

Rebuild before measuring: `cargo test` does not rebuild `target/release/openclare`, and the add-on
loads the engine from `blender/openclare/lib`, which `make addon` writes.

## Layout of the code

| Module | What it does |
|---|---|
| `mesh` | containers (`TriMesh`, `PolyMesh`), connectivity, OBJ, subdivision, test primitives |
| `geometry` | normals, areas, detection of feature edges |
| `hierarchy` | multi-resolution graph, to solve from coarse to fine |
| `field` | orientation field (4-RoSy), position field (4-PoSy), constraints, singularities |
| `extract` | from the field or the integer map to the graph of quads, then to faces |
| `sizing` | the size of the quads: what the shape calls for, read for a number of quads or between a largest and a smallest |
| `relax` | relaxation stage: final placement, quads drawn towards rectangles, on the surface and on hard edges |
| `spatial` | BVH, for projecting onto the source surface and casting rays through it |
| `pipeline` | the whole Instant Meshes chain |
| `remesher` | the interface the chains share, the factory that makes them by name, the `quad-layout` and `integer-grid` chains, and the analysis of a shape |
| `preprocess` | preprocessing stage: cleaning, decimation, isotropic remeshing, curvature, guide curves laid into the mesh |
| `crossfield` | cross field stage: one cross per face, alignment on curvature, singularities |
| `singularity` | simplification stage: cancelling the pairs of singularities that are not needed |
| `parametrization` | parametrization stage: cut graph, seamless (u, v) map |
| `tmesh` | layout stage: traces from the singularities, rectangular patches |
| `quantization` | quantization stage: whole lengths, connectivity of the quad mesh |
| `meshing` | meshing stage: one grid of quads per rectangle, vertices read off the map |
| `linalg` | sparse linear algebra (a wrapper round the `faer` library) |
| `config` | the one configuration of a run, presets, saving and reading back |
| `curvature` | curvatures and principal directions |
| `metrics` | quality measures of a quad mesh against its source |
| `fitting` | fitting stage: the vertices leave the surface for the mesh to keep the volume |
| `creases` | crease weight per edge, for subdivision to keep the hard edges and folds of the source |
| `uv` | two layers of texture coordinates unfolded from the quad mesh: the grid and the atlas |
| `bake` | the relief of a detailed surface read onto the texture of a plainer one |
| `thickness` | thickness of the shape under each vertex, measured by rays through its inside |
| `recorder` | the `Recorder` interface through which the stages show their work, and `Silent`, which listens to nothing |
| crate `openclare-capi` | C interface of the engine: a shared library, for Blender and other hosts |
| crate `openclare-cli` | the command line |
| `blender/openclare` | the Blender add-on, in Python, over the C interface |

The specification and what each milestone delivered are in [ALGORITHM.md](ALGORITHM.md); what is
left to do, in [ROADMAP.md](ROADMAP.md). The chain the project started from is described in
[docs/REFERENCE_INSTANT_MESHES.md](docs/REFERENCE_INSTANT_MESHES.md).

## How work is done here

- **Everything in the repository is in English**: code, comments, documents, commit messages,
  tickets.
- **The core library takes no heavy dependency.** A heavy one (Rerun, say) goes into a crate
  apart, behind an interface the core knows.
- **Tickets** are the GitHub issues, titled `DT-nnn — subject` with `nnn` the number of the issue,
  and a label `severity: high`, `medium` or `low`. Every shortcut, approximation, fallback or
  thing left undone gets one; findings and measures go into its comments; a ticket done is
  closed, not deleted.
- **One change at a time**, on a branch, merged once reviewed, with what was measured in its
  description.

## Releasing

```bash
make release version=FIX
```

`MINOR` or `MAJOR` instead raise that number of the version. The new number is written in the
sources, committed on `main` and tagged `v<version>`; `PUSH=1` sends the tag too. Once the tag is
on GitHub, the add-on is built for Linux (against glibc 2.28, as Blender itself) and for Windows,
and attached to a release of that tag (`.github/workflows/release.yml`).

## Licence

Contributions are under the licence of the project, GPL-3.0-or-later ([LICENSE](LICENSE)). The 3D
models by the author of the project are under CC BY-SA 4.0; those by others keep their own
licence, all listed in [CREDITS.md](CREDITS.md).
