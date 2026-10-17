# The bench

Compares any quad remesher with its source, with the metrics of the specification
([ALGORITHM.md](../ALGORITHM.md), section 0). The latest figures are in
[results/RESULTS.md](results/RESULTS.md).

## Running it

```bash
cargo build --release
make addon
make scenes
```

```bash
python3 bench/run.py
```

That runs every tool at one budget, and takes the better part of an hour. For the work in hand
there is a quick look, one run of the add-on per model, four at a time, next to what is stored
and without touching it: five minutes for the whole corpus, one for a few models.

```bash
make quick MODELS=camera,wolf
```

It needs `blender` on the `PATH`: `openclare` runs in it, through the add-on, and so do
QuadriFlow and Quad Remesher. `cargo build --release` builds the command line, `make addon` the
engine the add-on loads (`blender/openclare/lib/`): do both again before measuring, after any
change to the engine. `make scenes` makes the scenes that are missing (see below); the bench
does it too when it finds one missing. A full campaign takes a quarter of an hour or so per
budget, most of it QuadriFlow. Options: `--models blob,cube`, `--tools quadriflow`,
`--target 5000`, `--seeds 1,2,3`. A partial run updates the results that are there; each budget
has its own files (`RESULTS.md` for 3000 quads, `RESULTS-8000.md`, and so on).

## Scenes

A model of the bench is a **scene**: a `.blend` that holds one object to remesh and the
settings of the add-on, saved with the scene (`scene.openclare`: the object, the preset, the
number of quads, the adaptive size, the vertex group painted as density and its factor, the
collections of guide curves and of sharp curves, what to export). The file alone says how the
model is to be run, and it is run as a user of the add-on would run it: the object as it is
seen, modifiers applied, by the button of the panel.

| Where | What |
|---|---|
| `scenes/<model>.blend` | the scenes of the corpus |
| `corpus-large/scenes/<model>.blend` | those of the models that cannot be passed on (not tracked) |
| `out/<model>/source.obj` | the object of the scene as the add-on sees it, in triangles: what every result is measured against, and what the tools that do not read a scene are given (not tracked; made again when the scene is newer) |
| `out/<model>/source.groups` | the material of each of those triangles, when the scene has several |

There are two kinds of scenes:

- **A plain mesh**, made from a file as it is by `make_scenes.py` (`make scenes`): one mesh
  object named after the model, with the vertices and faces of the file it was made from, 3000
  quads, the preset `hard-surface` for the models `run.py` says have hard edges and `organic`
  for the others, an adaptive size of 50 %. For a BlenderKit model, every mesh object of the
  file downloaded, modifiers and place in the scene applied, in one mesh.
- **A scene made by hand** from a file of its author, by `make_scene.py`: the object as it is in
  that file, with its modifiers, its materials and its vertex groups, what its modifiers use,
  and the curves drawn on it. `venturi` (a car: Mirror, Subdivision Surface, Edge Split and
  Boolean modifiers, thirteen materials) and `ogre_hp` (a sculpt of 2.3 million vertices, three
  guide curves and a sharp one, each held on it by a Shrinkwrap, and a painted density) are
  such scenes.

The scenes are binary files, tracked through Git LFS (`git lfs install` once, on a machine that
has it). Those that are a plain mesh were made from OBJ files the repository had until
9 October 2026; `make scenes` still makes the scene of an OBJ file put under `corpus/`, which is
how a model that comes as one is added.

To make a scene from a `.blend` of your own:

```bash
blender -b --factory-startup --python bench/make_scene.py -- my-file.blend bench/scenes/mine.blend \
    --object Body --guides "openclare guide curves" --sharp "openclare sharp curves" \
    --density Group --factor 4 --preset organic --quads 12000 --adaptive 100
```

Only `--object` is needed; the collections are those the *Curves* panel of the add-on fills.
Or open the file in Blender, set the panel as you want it, and save: any `.blend` whose scene
has an object chosen in the panel (or a mesh active) is a scene. To run one, in Blender without
interface:

```bash
blender -b --factory-startup bench/scenes/mine.blend --python-exit-code 1 \
    --python bench/blender_run.py -- --out bench/out/mine --quads 8000
```

It writes the result (`<object>.obj`), what the run said (`<object>.json`: the settings, the
time, the engine's report) and `source.obj`, and prints one line starting with `OPENCLARE` that
says the same. For the bench to run it with the other tools, give it a line in `MODELS` of
`run.py` (and in [CREDITS.md](../CREDITS.md)).

The add-on stores positions in single precision, as Blender does; the OBJ files of the corpus
were in double precision. A model run from its scene is therefore not, to the last digit, the
model that was run from its OBJ, and the chain is sensitive to that: at 3000 quads the pig went
from 3031 quads to 3054 and the ogre from 3019 to 2949, with the same number of irregular
vertices or two more. On the same numbers the add-on and the command line make the same mesh.

## Memory

The engine runs inside Blender, and a run that goes wrong can take all the memory of the
machine: on 9 October 2026 a run on the car grew to 53 GB. So every process the bench starts
is capped (`MEMORY_CAP` in `run.py`: 12 GB of address space, 24 GB for `ogre_hp`), and
`blender_run.py` caps itself when run by hand (`--memory`). A run that hits the cap fails, and
is reported as `out of memory`; the bench goes on.

To measure one mesh, whatever made it:

```bash
./target/release/openclare measure source.obj quads.obj --target 3000
```

## Files

| File | What it does |
|---|---|
| `run.py` | runs each tool on each model, measures, writes `results/` |
| `blender_run.py` | Blender script: remeshes the object of a scene with the add-on, as its button does; writes the result, the object as the add-on saw it, and what the run said |
| `make_scenes.py` | Blender script: makes the scenes that are a plain mesh, from `corpus/<model>.obj` or from a BlenderKit file (`make scenes`) |
| `make_scene.py` | Blender script: makes a scene from an object of a `.blend`, with its modifiers, its curves and its painted density |
| `export_object.py` | Blender script: writes one object of a `.blend` as an OBJ and the material of each triangle, for the command line |
| `tmesh.py` | builds the T-mesh of each model and writes `results/TMESH.md` |
| `creases.py` | Blender script: subdivides each result with and without its crease weights and measures both; writes `results/CREASES.md` |
| `fetch_blenderkit.py` | downloads the BlenderKit models of the bench, which cannot be redistributed, and makes their scenes (`make blenderkit`) |
| `../scripts/scores.py` | marks from 0 to 100 per model, remesher and criterion; writes `results/SCORES.md` (`make scores`) |
| `acceptance.py` | checks the results against the acceptance thresholds, writes `results/ACCEPTANCE.md` |
| `quantize.py` | quantizes the T-mesh of each model and writes `results/QUANTIZATION.md` |
| `quadriflow.py` | Blender script: QuadriFlow on an OBJ (`out/<model>/source.obj`), once per seed |
| `quadremesher.py` | Blender script: Quad Remesher on an OBJ |
| `make_references.py` | makes and stores the Quad Remesher references while a licence is available |
| `scenes/` | the scenes the bench runs on (see above) |
| `scenes/` | [where each model comes from](scenes/README.md) |
| `corpus/` | not tracked: where an OBJ file is put for `make scenes` to make its scene |
| `corpus-large/` | the models whose licence forbids passing them on, and their scenes; not tracked |
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
  `openclare-matched` is openclare asked for that same number, to compare at equal quad
  count.

It has no seed: one run per model. The references come from a trial licence: non-commercial
use.

## Adding a tool

One line in the `TOOLS` dictionary of `run.py`. For a chain of this repository, run from the
command line on `out/<model>/source.obj`: `"name": openclare("name")`, where the name is the
one `openclare remesh --method` knows. A chain that does not use the seed is declared with
`seeded=False` and is run once.

The three chains of the repository are there: `instant-meshes` (the floor), `integer-grid`
(the provisional one) and `quad-layout`, the one the project is about. That last one goes by
the name of the project in the reports, under two settings:

- `openclare` (and `openclare-matched`): the add-on, in Blender, on the scene, with the
  settings saved in it; the bench only sets the number of quads. On the scenes that are a
  plain mesh that is quads sized by the shape (adaptive size 50 %); on the others it is what
  their author set, curves, painted density and materials included. With each run the bench
  stores what the engine says of the lines it was to keep (`engine` in `results*.json`: edges
  along the guide curves or the borders between materials, edges along the sharp curves, faces
  across a border, vertices welded and edges taken apart while mending, pieces on which the
  lines were let go).
- `openclare-uniform`: the command line, quads of one size. It is given the materials of the
  scene, not its curves nor its painted density.

(Until 9 October 2026 they were called `quad-layout-adaptive` and `quad-layout`, and
`openclare-matched` was `quad-layout-matched`; and both ran from the command line on OBJ
files.)

## Settings of the comparison

- QuadriFlow: `use_mesh_symmetry=False` (on by default, it would distort everything),
  `use_preserve_sharp` on models with hard edges, `use_preserve_boundary` on open models.
- AutoRemesher: standalone CLI ([huxingyi/autoremesher](https://github.com/huxingyi/autoremesher)), `--sharp-edge 90.0` on models with hard edges, `--sharp-edge 180.0` on organic models.
- The floor: `--crease-angle 30` on models with hard edges.
- Quad Remesher: detection of hard edges on models with hard edges only, symmetry off.
- The same face budget and the same seeds for all, and the same input: the object of the scene
  as the add-on sees it. Only `openclare` takes more from the scene (curves, density).
- Blender is started with `--factory-startup` everywhere: none of the add-ons installed on the
  machine is loaded, and the add-on run is the one of the repository.

## The ogre

The scene `ogre` comes from a sculpt of 4.7 million triangles, brought down to 200,000 by
`openclare preprocess <sculpt.obj> <ogre.obj> --triangles 200000`, before the bench had scenes.
The sculpt itself, at 2.3 million vertices, is the scene `ogre_hp`.

It is 0.055 per mille of the diagonal away from the original on average, and 4.5 per mille at
worst (on the sharpest details). The sculpt itself is the scene `ogre_hp`, with the curves and
the density its author drew on it.

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
  trip through FBX and its licence manager, `openclare`'s is the engine's alone, as the add-on
  reports it: without the modifiers Blender applies first, nor the mesh it builds after. An
  order of magnitude only.
- **Scenes made by hand.** The other tools are given their object, not their curves nor their
  painted density: on those models the comparison is not on equal terms.
- **Open source, closed result.** If a tool fills borders in, validity does not say so; the
  Hausdorff distance shows it.
