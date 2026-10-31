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
| Output | *Quads* (the default), or *Triangles*: triangles close to equilateral, of one size or as the shape asks, for simulation, 3D printing and collision meshes. The same settings apply but those of quads alone (Follow Curvature, Fewer Irregular Vertices, Volume Fit); the result says how many triangles cut through another one, which a printer cannot make sense of. How they compare with Blender's *Decimate*: [docs/TRIANGLES.md](docs/TRIANGLES.md) |
| For | With *Triangles*: *Simulation* (the default), the triangles of openclare, close to equilateral; or *Shape*, Blender's own *Decimate* in *Collapse* mode, which keeps closest to the shape at the same number of triangles and is the better one for printing and collision meshes: openclare uses it rather than make a worse one. Decimate reads the number, the symmetry and the painted density (kept denser); hard edges, curves and materials are not followed, and their settings are greyed |
| Ask For | *Number*: a number of quads, kept; the shape says where they are smaller. *Sizes*: the largest and the smallest quad; the shape says which where, and the number of quads is what that makes |
| Quads | With *Number*: the number of quads asked for. A part that is thin all along (a ring, a strap, a wire) is given the quads its thickness calls for out of that number |
| Largest, Smallest | With *Sizes*: the edges of the largest and of the smallest quads, in the units of the scene. Flat and thick parts get the largest; thin parts and tight bends get smaller ones, down to the smallest. Set from the size of the object when left at 0. The run says how many quads the shape called for |
| Analyse | With *Sizes*: measures what the shape asks for and proposes a largest and a smallest quad; then, as the sizes move, the panel tells at once about how many quads they make, and how much of the object is coarser than its shape asks |
| Show Sizes | After *Analyse*: colours the object with the size its quads will have, light where they are largest, dark where they are smallest, red where the shape calls for quads smaller than the smallest. A colour attribute of its own, taken away when unticked |
| Density | A vertex group of the object, painted in Weight Paint: quads are smallest where the weight is 1, largest where it is 0 |
| Factor | How many times more quads a weight of 1 gets than a weight of 0, for the same area |
| Density from Shape | Writes the density the shape calls for (Adaptive Size, thin parts) as a vertex group, to paint on top in Weight Paint. The group becomes the *Density*, and *Adaptive Size* is switched off so that the shape does not count twice |
| Materials | Each quad gets the material of the surface it lies on, and the borders between materials are kept as lines of edges where that can be done |
| Mark: Sharp, Creases | On by default. The edges of the result along a hard edge of the object are marked sharp, and given a crease weight that a *Subdivision Surface* modifier reads to keep them sharp. The edges along a curve take the marks of the curve |
| Symmetry | *None*, *X*, *Y* or *Z*: the quads come out the same on both sides of the plane through the origin of the object across that axis, in the object's own axes, with a loop of edges on the plane. The object must be the same on both sides; one that is not is refused, and said why |
| Shape: presets | The menu in the header of the *Shape* block saves every setting below with *+*, and sets them back from a preset saved there; a saved preset can be removed. The add-on ships one, *Default*, which sets them all back to their defaults: those suit sculpts, characters and mechanical parts alike |
| Adaptive Size | How much the size of the quads follows the curvature: at 0 % they are all the same size; towards 100 % they get smaller where the shape bends and larger where it is flat |
| Quads Around Thin Parts | The fewest quads that go round a thin part, a finger or the tip of an ear, for it to keep its volume. 0 does not look at thickness |
| Hard Edges | On by default. Edges of the object sharper than the angle, and still sharp at the size of the quads asked for, are followed by the quads wherever they can, and kept crisp; the fine creases of a sculpt are not |
| Show Hard Edges (eye beside the angle) | Shows over the object in the 3D viewport what *Hard Edges* does: in solid cyan the edges the quads will follow, in faint orange those sharper than the angle that are left out, too small for the quads asked for. Found by the engine's own code at the size of the quads asked for, again a moment after a setting or the object changes; nothing is written into the mesh or the file. While a change is being computed the lines are dimmed and a ring turns beside the eye; once they are up to date, a tick, and below it how many edges are followed and left out |
| Marked Sharp | Off by default. The edges marked sharp on the object (*Mark Sharp*, or a *Smooth by Angle* modifier) are followed by the quads and kept as hard edges, whatever their angle and size |
| Follow Curvature | How strongly the edges are drawn along the directions in which the shape bends, against how smoothly they flow |
| Fewer Irregular Vertices | Irregular vertices closer than this many quads to one of the opposite kind are cancelled with it: fewer of them, for meshes meant to deform |
| Volume Fit | How far the vertices leave the surface at the very end, for the mesh to keep the volume of the shape |
| Shading | How the faces of the result are shaded. *By Angle* (the default): smooth, but sharp across the edges sharper than the angle of *Hard Edges*, so a sculpt stays smooth and a mechanical part keeps its edges crisp. *Smooth*: every face smooth. *Flat*: every face flat |

An object scaled differently along its axes is remeshed as it is seen: its quads come out
square in the scene.

Some features are experimental: they run, but are not tested enough yet, or have a known issue.
They carry the icon of a flask, *Experimental* starts their tooltip, and an option of the
add-on's preferences, *Experimental Features* (on by default), hides them all: their settings,
the *Curves* and *Poles* blocks, and the tools *Draw Curve*, *Cut Volume* and *Poles*. Hidden,
a remesh gives their settings the default values, asks for a number of quads, makes triangles
with Blender's *Decimate*, and follows no curve and no pole placed by hand.

### Curves

Curves drawn on the model give the result a line of edges along each: a loop round an eye, a
limb, a muscle. Each curve also says what to mark the edges along it with: *Sharp*, a *Crease*
weight (1 keeps them sharp under a *Subdivision Surface* modifier), a *Seam*. With no mark, it is a
line of edges and nothing more.

- **Draw Curve**, in the toolbar of the 3D viewport (T) in Object Mode, draws on the active
  mesh, selected: hold the left button and draw. Going back along the stroke undoes it; letting
  go at its start closes it; pressing near an end of a curve takes it up again. Esc or the right
  button drops the stroke under way. The header of the tool sets the marks of the curves to come,
  the rope that steadies the hand, and a snap that keeps the stroke to a ridge or a valley near
  it, within a radius that F sets. S opens the menu of the snaps; V and R switch Valley and Ridge
  on and off, in the middle of a stroke too. While the tool is active, the circle of the snap
  follows the mouse, with the name of the snap beside it, dashed when there is none.
- **Cut Volume**, under the same button, draws a straight line across a limb: the curve is the
  loop round it, the hidden side too, shown while the line is drawn.
- A curve is the mesh's when it is its child. A drawn curve is a Bezier curve of few control
  points, a child of the model, held on it by a *Shrinkwrap*: it can be edited as any curve.
- The *openclare* panel of a curve (its Object Data tab) sets its marks, and its *Mesh*: set
  there, the curve is made a child of that mesh where it is. A curve made by hand works too.
- The *Curves* block of the mesh's panel lists its curves and their marks, and selects them.

With the result selected, the same panel says what it was remeshed from and how the run went:
how many quads, and, as a warning, what did not go as asked (far fewer quads than asked for, a
part that could not be meshed, hard edges let go, halves of a symmetric result that meet across
their quads).
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
