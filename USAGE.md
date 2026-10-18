# Advanced usage

What the Blender add-on does is described in the [README](README.md). This page is for the rest:
the command line, the settings behind the panel, the library, and the tools to look inside a run.

## The command line

```bash
cargo build --release
```

```bash
./target/release/openclare remesh input.obj output.obj --quads 5000
```

To try it with no mesh at hand (`torus`, `sphere`, `cube`, `plane`, `lblock`, `cylinder`):

```bash
./target/release/openclare primitive torus torus.obj
```

`openclare --help` lists every command and option.

## Presets and saved settings

A preset is a named set of settings; each setting can still be changed on top of it.

```bash
./target/release/openclare presets
```

```bash
./target/release/openclare remesh input.obj output.obj --preset animation --quads 8000 --alignment 0.1
```

To save the settings of a run and use them again as a preset of your own:

```bash
./target/release/openclare remesh input.obj output.obj --preset organic --singularity-reach 4 --save-config mine.txt
```

```bash
./target/release/openclare remesh other.obj output.obj --config mine.txt
```

## A number of quads, or their sizes

`--quads` asks for a number of quads, and that number is kept. The shape says where they are
smaller: where it bends more than it typically does, and round its thin parts.

The number is a consequence, though: what one usually means is how coarse the mesh may be where
nothing happens, and how fine where the shape is thin or bends. `--largest` and `--smallest` ask
for that, as edge lengths in the units of the mesh. Each point gets the size its shape calls for
between the two, and the run says how many quads that made and how much of the surface would
have called for quads smaller than the smallest.

```bash
./target/release/openclare remesh input.obj output.obj --largest 0.25 --smallest 0.05
```

`--most-quads` (200,000 by default) is the most that are made: where the shape calls for more,
every quad is made larger, and the run says so.

`analyse` looks at the shape once and prints, for a range of smallest quads, how many quads each
would make, with a pair of sizes to start from: what a pair of sizes costs is known before
anything is meshed. Counting a pair takes a few milliseconds once the shape is analysed.

```bash
./target/release/openclare analyse input.obj
```

## How the size follows the shape

`--adaptivity` says how much the size of the quads follows the curvature: 0 for quads of one size,
1 for the error to the surface evened out. A painted density can be given too: a file of one
weight per vertex of the input, from 0 (least dense) to 1 (most dense). `--density-contrast` sets
how far apart the two are: at 4, the default, the places painted at 1 have four times the quads
per unit of area.

```bash
./target/release/openclare remesh input.obj output.obj --quads 3000 --adaptivity 0.7 --density weights.txt --density-contrast 8
```

Thin parts keep their volume: where the shape is too thin for the quads (a finger, the tip of an
ear), they are made small enough for a given number of them to go round it. That number is set
with `--quads-around`: 6 in the `organic` and `animation` presets, 0 (thickness is not looked at)
in `hard-surface`.

```bash
./target/release/openclare remesh input.obj output.obj --quads 3000 --quads-around 8
```

![The hand of the ogre at 3000 quads](docs/images/thin_parts_hand.png)

A separate piece that is thin all along (a ring, a strap, a wire) gets the quads its thickness
calls for out of the number asked for: the other pieces have that many fewer, and keep half of
the number between them whatever is called for. The run says how many were moved.

The mesh also keeps the volume of the shape. Vertices sitting on the surface give faces that run
under it wherever it bulges; a last stage moves them a little off the surface, along their
normals, for the faces to run through it rather than under it. `--volume-fit` sets how far, from
0 (vertices on the surface) to 1 (the default).

```bash
./target/release/openclare remesh input.obj output.obj --quads 3000 --volume-fit 0
```

## Guide curves

Curves drawn on the model become lines of edges of the result. From the command line they go in
as a text file, `--curves curves.txt`: one point per line as `x y z`, a blank line between two
curves, and a line saying `closed` for one that joins its first point.

![The ogre with and without guide curves](docs/images/guide_curves.png)

## Symmetry

`--symmetry x` (or `y`, `z`; `symmetry = x` in a saved configuration) says that the plane through
the origin of the model, across that axis, is a plane of symmetry. The plane is drawn into the
model as a line of edges, the whole model is remeshed, and the half on the positive side is kept
and mirrored: the result is its own mirror image, with a loop of edges on the plane. A model that
is not on both sides of the plane, or whose mirror image is more than 2 % of its size away from
it somewhere, is refused. Guide curves, a painted density and groups are not taken with symmetry
yet.

```bash
./target/release/openclare remesh head.obj quads.obj --preset organic --quads 3000 --symmetry x
```

## Inside the chain

To see the T-mesh of a model, its cuts written as lines on the surface:

```bash
./target/release/openclare tmesh input.obj --quads 3000 --simplify 2 --traces cuts.obj
```

And to quantize it:

```bash
./target/release/openclare quantize input.obj --quads 3000 --simplify 2
```

`openclare remesh --notes` prints what the stages remark on: how many triangles folded and where,
how many guide edges were let go, why a piece went to the other chain.

### Watching the chain at work

Each stage of the chain can show what it makes: the work mesh and its curvature, the size asked
of the quads, the cross field and its singularities before and after simplification, the cuts and
nodes of the layout, the whole lengths, the quads before and after relaxation. All of it is
recorded in a `.rrd` file to open in the viewer of [Rerun](https://rerun.io). Two timelines, to
pick at the bottom of the window:

- `step`, the one it opens on: one stage at a time. Each stage takes the place of the one before,
  on the surface it is read on; the left and right arrows go from one to the next, and the last
  shows the final mesh alone.
- `stack`: everything shown so far stays, to lay one stage over another (the cuts over the final
  mesh, say), hiding the layers that are not wanted.

```bash
make view MODEL=ogre
make view MODEL=venturi QUADS=8000 OPTIONS='--groups bench/out/venturi/source.groups'
```

The stage `map` shows what the layout is cut from: its grid lines (`u-lines`, `v-lines`), the
triangles it folds or flattens (`folded-or-flat`), those that folded when every line was put on a
grid line (`folded-with-every-line`), and the guide lines kept and let go.

A model that is very small or very large (less than 1 or more than 1000 across) is shown at a
power of ten times its size, which a note of the recording gives.

`make view` builds what it takes, remeshes the model while recording, then opens the viewer. The
first build is long (Rerun comes with several hundred dependencies) and needs Rust 1.95. None of
it weighs on the rest: the library knows one interface only, `Recorder`, and its silent
implementation `Silent`. Rerun lives in two crates apart, built on demand only:

| Crate | What it is | Built with |
|---|---|---|
| `openclare-rerun` | the `Recorder` that writes a `.rrd`; gives `openclare remesh` its `--record` option | `cargo build --release -p openclare-cli --features rerun` |
| `openclare-viewer` | the Rerun viewer built in, executable `openclare-view` | `cargo build --release -p openclare-viewer` |

```bash
target/release/openclare remesh model.obj quads.obj --quads 3000 --record model.rrd
```

```bash
target/release/openclare-view model.rrd
```

A `.rrd` is read by the viewer of the version that wrote it, here 0.36.

## Measuring a result

```bash
./target/release/openclare measure source.obj quads.obj --target 3000
```

The marks from 0 to 100 the bench gives, for any mesh against its source:

```bash
python3 scripts/scores.py mesh source.obj quads.obj --target 3000
```

## As a library

```rust
use openclare::{RemeshConfig, Target};

let mut config = RemeshConfig::for_organic(Target::QuadCount(5000));
config.alignment = 0.1;
let (quads, report) = config.remesh(&triangles)?;
```

Sizes instead of a number: `Target::SizeRange { largest, smallest }`. `openclare::Analysis::of`
measures a shape once, then `quads`, `below` and `proposal` answer at once for any pair of sizes.

## From another host

The add-on calls the engine through a C interface (`crates/openclare-capi`), which any other host
can use: one call, flat arrays in, quads out, settings as the text of a saved configuration, and a
function of the host's called as each stage begins, which can stop the run. Blender's own arrays
are read where they are, without a copy on the Python side.
