# Remeshing built into Blender

<p class="subtitle">Three engines ship with Blender: OpenVDB voxels, an octree with dual contouring, and QuadriFlow. Where each one is, what its settings do, and what it cannot do.</p>

Two of the three rebuild the surface on a grid that follows the axes, not the shape: they clean up a mesh, they do not give it edge loops. Only QuadriFlow aligns its quads with the shape. The names, defaults and shortcuts below are read from Blender 5.2.2 itself; the behaviour is the [manual's](https://docs.blender.org/manual/en/latest/modeling/meshes/retopology.html).

## Voxel Remesh: OpenVDB

The mesh is sampled on a grid of voxels with OpenVDB, and a new surface is built on it: closed, in one piece, of quads of one size.

- **Where**: *Properties ‣ Data ‣ Remesh*, mode **Voxel**, button **Voxel Remesh**, in Object Mode or Sculpt Mode. In Sculpt Mode, **R** sets the voxel size in the viewport and **Ctrl R** remeshes. As a modifier: *Remesh*, mode **Voxel**.
- **Voxel Size** (0.1): the size of a cell, so of a quad; smaller keeps finer details.
- **Adaptivity** (0): above 0, fewer faces where detail is not needed, but triangles appear and Fix Poles is turned off. Keep it at 0 for quads only.
- **Fix Poles** (off): fewer poles, a better flow.
- **Preserve Volume** and **Preserve Attributes** (off): project onto the original to keep its volume; carry its attributes (masks, face sets, colours) over.
- **For**: welding overlapping parts of a sculpt, closing holes, starting a sculpt again on even faces.
- **Limits**: details thinner than a voxel are lost and sharp edges are rounded. It reads the original mesh only, not its modifiers or shape keys, and refuses an object with a Multiresolution modifier.

## Remesh modifier: an octree and dual contouring

The three older modes of the *Remesh* modifier build the surface on an octree, by dual contouring.

- **Where**: *Add Modifier ‣ Generate ‣ Remesh*, mode **Blocks**, **Smooth** or **Sharp**. It works live; apply it to keep the result.
- **Blocks** leaves the cubes as they are, **Smooth** smooths the surface, **Sharp** keeps the edges and corners.
- **Octree Depth** (4): the resolution; each step about four times the faces.
- **Scale** (0.9): the size of the model over the size of the grid; lower is coarser.
- **Sharpness** (1, Sharp only): lower filters noise, higher follows the edges of the input more closely.
- **Remove Disconnected** (on) with **Threshold** (1): drops pieces smaller than this ratio of the largest.
- **For**: a closed, even shell; Sharp for a hard-surface part.
- **Limits**: a grid along the axes, as with voxels. The input must have some thickness: put a *Solidify* modifier above a flat mesh.

## QuadriFlow Remesh

QuadriFlow (Huang and others, 2018) follows Instant Meshes: an orientation field on the surface, then quads aligned with it, with constraints solved as flows to limit singularities. Its quads follow the shape.

- **Where**: *Properties ‣ Data ‣ Remesh*, mode **Quad**, button **QuadriFlow Remesh**, which opens its settings.
- **Mode**: **Faces** (by default, **Number of Faces** 4,000, approximate), **Ratio** (of the current number of faces) or **Edge Length**.
- **Use Mesh Symmetry** (on): a symmetric result, along the axes set in the mesh's symmetry options.
- **Preserve Sharp** and **Preserve Mesh Boundary** (off): keep the sharp features, keep the open borders.
- **Preserve Attributes** (off): reproject the attributes onto the new mesh.
- **Smooth Normals** (off): smooth shading on the result.
- **Seed** (0): another seed, another layout of quads.
- **For**: the only one of the three that resembles a retopology.
- **Limits**: one size of quad everywhere; slower than voxels. It does not clean up self-intersections, so on a messy sculpt run a Voxel Remesh first.

## Around them

- **Decimate** modifier: fewer faces without a new mesh. **Collapse** merges vertices taking the shape into account (Ratio, Symmetry, Triangulate), **Un-Subdivide** undoes a subdivision on a grid, **Planar** dissolves flat areas (Angle Limit).
- **Dyntopo**, in Sculpt Mode: adds and removes triangles under the brush as you sculpt (Detailing: Relative, Constant, Brush or Manual). It makes triangles, not a retopology; **R** and **Ctrl R** drive it instead of the voxels while it is on.
- **Poly Build**, in Edit Mode: a manual tool; Ctrl-click adds a vertex and a face, drag on an edge extrudes it into a quad, Shift-click deletes. **Create Quads** leaves quads.
- **Retopology overlay** (*Overlays ‣ Mesh Edit Mode*, with an **Offset** towards the camera), **face snapping** and the **Shrinkwrap** modifier keep a hand-made mesh on the sculpt.

## In practice

Voxel Remesh to clean up, QuadriFlow to orient, then Poly Build and the snapping for the places that need a hand. The add-ons sold for Blender often run these same engines, or Instant Meshes and QuadWild: see [the engines](engines.md) and [the tools](retopology.md).
