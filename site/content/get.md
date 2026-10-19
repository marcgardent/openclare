# Get openclare

<p class="subtitle">The source is on GitHub, at <a href="https://github.com/marcgardent/openclare">marcgardent/openclare</a>.</p>

## The Blender add-on

One archive holds everything: the add-on and its engine, compiled. There is nothing else to install, no Python package, no program beside it, and nothing is downloaded when it runs.

1. Take the archive of your system from the [releases](https://github.com/marcgardent/openclare/releases): `openclare-<version>-linux-x64.zip` or `openclare-<version>-windows-x64.zip`. Or build it yourself (below).
2. In Blender 4.2 or later: *Edit › Preferences › Get Extensions*, the arrow at the top right, *Install from Disk*, and pick the archive.

A tab named *openclare* appears in the sidebar of the 3D viewport (key N). Select a mesh, choose a preset (organic, animation, hard surface) and a number of quads, press *Remesh to Quads*. The result is a new object beside the original, which is left as it is.

The Linux build runs wherever Blender does (glibc 2.28 or later). The Windows build passes the tests of the engine but has not yet been run in a Blender for Windows; there is no build for macOS yet.

What the panel offers, setting by setting (guide curves, painted density, sizes instead of a number of quads, UV layers, sharp edges and creases, materials, baking the relief of the original), is in the [README](../../README.md).

### Building the add-on

With the [Rust toolchain](https://www.rust-lang.org/tools/install) installed, from the root of the repository:

```sh
make addon      # dist/openclare-<version>-linux-x64.zip, to install from Blender
make install    # or straight into the extensions of the Blender on the PATH
```

`make addon-windows` builds the Windows archive from Linux (it needs the Rust target `x86_64-pc-windows-gnu` and MinGW-w64). `make help` lists every shortcut.

## The command line

```sh
cargo build --release
./target/release/openclare remesh input.obj output.obj --quads 5000
./target/release/openclare remesh part.obj quads.obj --preset hard-surface --quads 3000
./target/release/openclare primitive torus torus.obj     # a shape to try it on
./target/release/openclare measure input.obj output.obj --target 5000
```

`openclare --help` lists every command and option. Presets and saved settings, sizes instead of a number of quads, guide curves, the Rust library and the C interface for other hosts are described in [USAGE.md](../../USAGE.md).

## Further

<div class="cards">
<a class="card" href="../../README.md"><strong>README</strong><span>The add-on, setting by setting.</span></a>
<a class="card" href="../../USAGE.md"><strong>USAGE.md</strong><span>The command line, the library, the C interface, and watching the chain at work.</span></a>
<a class="card" href="../../CONTRIBUTING.md"><strong>CONTRIBUTING.md</strong><span>Building, testing, the bench, the layout of the code.</span></a>
<a class="card" href="../../ALGORITHM.md"><strong>ALGORITHM.md</strong><span>How it works, stage by stage.</span></a>
<a class="card" href="../../CREDITS.md"><strong>CREDITS.md</strong><span>Authors, sources and licences of the 3D models used.</span></a>
</div>

## Licence

The code is under the GNU GPL, version 3 or later. The 3D models by the author of the project (the ogre, the car) are under CC BY-SA 4.0; those by others each keep their licence, listed in [CREDITS.md](../../CREDITS.md).
