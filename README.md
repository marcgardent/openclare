# openclare

Automatic quad retopology for Blender: a mesh of triangles in, a mesh of quads out, its edges
following the shape. Open source, written in Rust. The aim: an open alternative to the commercial
remeshers, better than QuadriFlow, the one built into Blender.

## Three remeshers side by side

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/dashboard/gallery-dark.webp">
  <img alt="openclare, QuadriFlow and AutoRemesher on the ogre, the pig, a mechanical part and a generated building, each with its number of quads and its mark out of 100" src="docs/images/dashboard/gallery-light.webp">
</picture>

Each row at the same number of quads, asked of every remesher. The mark weighs
validity, alignment on the shape, the shape of the quads, the distance to the model, the
irregular vertices and the speed.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/dashboard/marks-dark.svg">
  <img alt="Average mark out of 100 of the three remeshers over the organic models, the mechanical parts and the assembled and generated models, at 1,000, 3,000 and 8,000 quads asked for" src="docs/images/dashboard/marks-light.svg">
</picture>

Every model, every remesher, in pictures: [docs/models](docs/models/README.md). The marks in
full: [SCORES.md](bench/results/SCORES.md).

## The Blender add-on

This is the Community edition of the add-on.

### Installing

One archive holds everything: the add-on and its engine, compiled. There is nothing else to
install, no Python package, no program beside it, and nothing is downloaded when it runs.

1. Take the archive of your system from the [releases](https://github.com/marcgardent/openclare/releases)
   (`openclare-<version>-<platform>.zip`, the platform being `linux-x64`, `windows-x64`,
   `macos-arm64` or `macos-x64`), or build it ([CONTRIBUTING.md](CONTRIBUTING.md)).
2. In Blender 4.2 or later: *Edit > Preferences > Get Extensions*, the arrow at the top right,
   *Install from Disk*, and pick the archive.

A panel named *openclare* appears in the Properties editor, on the tab of the mesh (Object Data).
The Linux build runs wherever Blender does (glibc 2.28 or later). The Windows and macOS builds
pass the tests of the C interface on their system but have not yet been run in a Blender there;
the macOS one is signed ad hoc, not notarized.

Each release also has `openclare-capi-<version>-<platform>.tar.gz`: the engine as a C library,
with its header, for other programs ([crates/openclare-capi/README.md](crates/openclare-capi/README.md)).

### Using it

Select a mesh, set what follows, press *Remesh to Quads*. The result is a new object,
`<name>.quads`, at the same place as the original, which is left as it is. What is remeshed is
the object as it is seen, modifiers applied. The engine runs in the background and Blender stays
free: the panel shows a bar of its progress, with a button to stop it (Esc stops it too), and
the status bar each stage as it begins. It takes about eight seconds for the 200,000 triangles
of the ogre at 3000 quads.

| Setting | What it does |
|---|---|
| Ask For | *Number*: a number of quads, kept; the shape says where they are smaller. *Sizes*: the largest and the smallest quad; the shape says which where, and the number of quads is what that makes |
| Quads | With *Number*: the number of quads asked for. A part that is thin all along (a ring, a strap, a wire) is given the quads its thickness calls for out of that number |
| Largest, Smallest | With *Sizes*: the edges of the largest and of the smallest quads, in the units of the scene. Flat and thick parts get the largest; thin parts and tight bends get smaller ones, down to the smallest. Set from the size of the object when left at 0. The run says how many quads the shape called for |
| Density | A vertex group of the object, painted in Weight Paint: quads are smallest where the weight is 1, largest where it is 0 |
| Factor | How many times more quads a weight of 1 gets than a weight of 0, for the same area |
| Materials | Each quad gets the material of the surface it lies on, and the borders between materials are kept as lines of edges where that can be done |
| Shape: presets | The menu in the header of the *Shape* block sets every setting below: *Organic*, *Animation*, *Hard Surface*, and those saved there with *+*. A saved preset can be removed; those of the add-on cannot |
| Adaptive Size | How much the size of the quads follows the curvature: at 0 % they are all the same size; towards 100 % they get smaller where the shape bends and larger where it is flat |
| Quads Around Thin Parts | The fewest quads that go round a thin part, a finger or the tip of an ear, for it to keep its volume. 0 does not look at thickness |
| Hard Edges | On by default. Edges of the object sharper than the angle, and still sharp at the size of the quads asked for, are followed by the quads wherever they can, and kept crisp; the fine creases of a sculpt are not |
| Show Hard Edges (eye beside the angle) | Shows over the object in the 3D viewport what *Hard Edges* does: in solid cyan the edges the quads will follow, in faint orange those sharper than the angle that are left out, too small for the quads asked for. Found by the engine's own code at the size of the quads asked for, again a moment after a setting or the object changes; nothing is written into the mesh or the file. While a change is being computed the lines are dimmed and a ring turns beside the eye; once they are up to date, a tick, and below it how many edges are followed and left out |
| Follow Curvature | How strongly the edges are drawn along the directions in which the shape bends, against how smoothly they flow |
| Fewer Irregular Vertices | Irregular vertices closer than this many quads to one of the opposite kind are cancelled with it: fewer of them, for meshes meant to deform |
| Volume Fit | How far the vertices leave the surface at the very end, for the mesh to keep the volume of the shape |

With the result selected, the same panel says what it was remeshed from and how the run went.
The arrow beside the name of the original selects it; *Remesh Again* remeshes the original with
the settings it has now, in place of the result, which keeps its name and place.

## Further

- [USAGE.md](USAGE.md): the command line, sizes and their analysis, the library, the C interface
  for other hosts, and watching the chain at work.
- [CONTRIBUTING.md](CONTRIBUTING.md): building, testing, the bench, the layout of the code.
- [ALGORITHM.md](ALGORITHM.md): how it works, stage by stage, and what each milestone delivered.
- [ROADMAP.md](ROADMAP.md): what is left to do. Tickets are the
  [issues](https://github.com/marcgardent/openclare/issues).
- [CREDITS.md](CREDITS.md): the authors, sources and licences of the 3D models and tools used.

## Licence

The code: GPL-3.0-or-later, see [LICENSE](LICENSE). The 3D models by the author of the project
(the ogre, the car): CC BY-SA 4.0. Those by others each keep their licence, all listed in
[CREDITS.md](CREDITS.md).
