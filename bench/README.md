# The bench

Compares any quad remesher with its source, with the metrics of the specification
([ALGORITHM.md](../ALGORITHM.md), section 0). The latest figures are in
[results/RESULTS.md](results/RESULTS.md).

## Running it

```bash
cargo build --release
```

```bash
python3 bench/run.py
```

It needs `blender` on the `PATH`: it runs QuadriFlow and makes the provisional corpus. A full
campaign takes a quarter of an hour or so per budget, most of it QuadriFlow. Options:
`--models blob,cube`, `--tools quadriflow`, `--target 5000`, `--seeds 1,2,3`. A partial run
updates the results that are there; each budget has its own files (`RESULTS.md` for 3000
quads, `RESULTS-8000.md`, and so on).

To measure one mesh, whatever made it:

```bash
./target/release/openclare measure source.obj quads.obj --target 3000
```

## Files

| File | What it does |
|---|---|
| `run.py` | runs each tool on each model, measures, writes `results/` |
| `tmesh.py` | builds the T-mesh of each model and writes `results/TMESH.md` |
| `creases.py` | Blender script: subdivides each result with and without its crease weights and measures both; writes `results/CREASES.md` |
| `fetch_blenderkit.py` | downloads the BlenderKit models of the bench, which cannot be redistributed, and exports them (`make blenderkit`) |
| `import_blenderkit.py` | Blender script: exports a BlenderKit `.blend` as a source OBJ |
| `../scripts/scores.py` | marks from 0 to 100 per model, remesher and criterion; writes `results/SCORES.md` (`make scores`) |
| `acceptance.py` | checks the results against the acceptance thresholds, writes `results/ACCEPTANCE.md` |
| `quantize.py` | quantizes the T-mesh of each model and writes `results/QUANTIZATION.md` |
| `quadriflow.py` | Blender script: QuadriFlow on an OBJ, once per seed |
| `quadremesher.py` | Blender script: Quad Remesher on an OBJ |
| `make_references.py` | makes and stores the Quad Remesher references while a licence is available |
| `corpus/` | the source models, tracked ([where they come from](corpus/README.md)) |
| `corpus-large/` | what cannot or should not be in git: originals too heavy for it (the ogre at 4.7 million triangles) and models whose licence forbids passing them on; not tracked |
| `reference/` | the references kept, tracked ([notice](reference/README.md)) |
| `make_corpus.py` | Blender script: makes the synthetic shapes of the corpus |
| `import_crane.py`, `subdivide.py` | turn the models downloaded from Keenan Crane's repository into sources of the corpus |
| `blender_io.py` | OBJ in and out without axis conversion, so that every tool sees the same coordinates |

The metrics are in `crates/openclare/src/metrics.rs`, each tested on a mesh whose score is known
(`crates/openclare/tests/metrics.rs`).

## Quad Remesher

Quad Remesher (Exoside) is commercial software. Its results are computed once, with the Blender
add-on and a valid licence (the bench works round neither), then stored in `reference/`. The
bench reads these references back from then on, whether the software is still installed or
not. For a model or a budget with no reference and no licence, the columns stay empty.

- `quadremesher`: default settings (adaptive size at 50 %, quad count adjusted).
- `quadremesher-uniform`: constant size and exact quad count, to compare on equal terms with
  remeshers of one size.
- `quadremesher-as-made`: the mesh of `quadremesher`, measured against the number of quads it
  has rather than the budget. Left to itself it makes about twice the quads asked for;
  `quad-layout-matched` is openclare asked for that same number, to compare at equal quad
  count.

It has no seed: one run per model. The references come from a trial licence: non-commercial
use.

## Adding a tool

One line in the `TOOLS` dictionary of `run.py`. For a chain of this repository:
`"name": openclare("name")`, where the name is the one `openclare remesh --method` knows. A
chain that does not use the seed is declared with `seeded=False` and is run once.

The three chains of the repository are there: `instant-meshes` (the floor), `integer-grid`
(the provisional one) and `quad-layout` (the one the project is about), the last under two
settings: quads of one size, and sized by the shape.

## Settings of the comparison

- QuadriFlow: `use_mesh_symmetry=False` (on by default, it would distort everything),
  `use_preserve_sharp` on models with hard edges, `use_preserve_boundary` on open models.
- The floor: `--crease-angle 30` on models with hard edges.
- Quad Remesher: detection of hard edges on models with hard edges only, symmetry off.
- The same face budget and the same seeds for all.

## The ogre

`corpus/ogre.obj` comes from a sculpt of 4.7 million triangles, brought down to 200,000 by:

```bash
./target/release/openclare preprocess bench/corpus-large/ogre_hp.obj bench/corpus/ogre.obj --triangles 200000
```

It is 0.055 per mille of the diagonal away from the original on average, and 4.5 per mille at
worst (on the sharpest details).

## Known limits

- **A partly synthetic corpus.** Several shapes are generated (`make_corpus.py`), not sculpted.
  Conclusions drawn from them need confirming on real models.
- **Inverted faces.** The test compares the orientation of each face with that of the source
  around it. On Suzanne (thin walls, pieces that cross) the count is uncertain by a face or
  two; elsewhere it agrees with a test that does not use the source.
- **Base complex (C).** It saturates near the number of faces when the mesh has no global
  structure; to be read with the number of irregular vertices.
- **Alignment (A).** Curvature is read at the scale of a quad. On a noisy shape it has no
  stable direction and the metric stays at the level of chance (22.5 degrees).
- **Time.** QuadriFlow's includes Blender's internal mesh conversion, Quad Remesher's a round
  trip through FBX and its licence manager, `openclare`'s leaves out reading and writing the
  files: an order of magnitude only.
- **Open source, closed result.** If a tool fills borders in, validity does not say so; the
  Hausdorff distance shows it.
