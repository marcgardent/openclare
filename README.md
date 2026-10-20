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

This is the Community edition of the add-on: it remeshes an object to a number of quads, with
the presets of the shape.

### Installing

One archive holds everything: the add-on and its engine, compiled. There is nothing else to
install, no Python package, no program beside it, and nothing is downloaded when it runs.

1. Take the archive of your system from the [releases](https://github.com/marcgardent/openclare/releases)
   (`openclare-<version>-linux-x64.zip` or `openclare-<version>-windows-x64.zip`), or build it
   ([CONTRIBUTING.md](CONTRIBUTING.md)).
2. In Blender 4.2 or later: *Edit > Preferences > Get Extensions*, the arrow at the top right,
   *Install from Disk*, and pick the archive.

A panel named *openclare* appears in the Properties editor, on the tab of the mesh (Object Data).
The Linux build runs wherever Blender does (glibc 2.28 or later). The Windows build passes the
tests of the engine but has not yet been run in a Blender for Windows; there is no build for
macOS yet.

### Using it

Select a mesh, set what follows, press *Remesh to Quads*. The result is a new object,
`<name>.quads`, at the same place as the original, which is left as it is: a bare mesh of quads,
with no UV layer, material or marked edge. What is remeshed is the object as it is seen,
modifiers applied. The engine runs in the background and Blender stays free: the panel shows a
bar of its progress, with a button to stop it (Esc stops it too), and the status bar each stage
as it begins. It takes about eight seconds for the 200,000 triangles of the ogre at 3000 quads.

| Setting | What it does |
|---|---|
| Quads | The number of quads asked for, kept; the shape says where they are smaller. A part that is thin all along (a ring, a strap, a wire) is given the quads its thickness calls for out of that number |
| Shape: presets | The menu in the header of the *Shape* block sets every setting below: *Organic*, *Animation*, *Hard Surface*, and those saved there with *+*. A saved preset can be removed; those of the add-on cannot |
| Adaptive Size | How much the size of the quads follows the curvature: at 0 % they are all the same size; towards 100 % they get smaller where the shape bends and larger where it is flat |
| Quads Around Thin Parts | The fewest quads that go round a thin part, a finger or the tip of an ear, for it to keep its volume. 0 does not look at thickness |
| Hard Edges | Edges of the object sharper than the angle are followed by the quads wherever they can, and kept crisp: for mechanical parts |
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
