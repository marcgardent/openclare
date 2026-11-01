# The retopology pipeline in Blender

<p class="subtitle">From a dense sculpt or scan to a light mesh of quads, with the detail baked back: the steps, and the page of the Blender manual for each tool.</p>

A retopology rebuilds a mesh: the source keeps the detail, the new mesh carries the topology that animation, UVs and subdivision need. The steps below go in order; each tool links to its page in the [Blender manual](https://docs.blender.org/manual/en/latest/). The engines Blender ships for the automatic route are described in [Remeshing built into Blender](blender.md).

## 1. Prepare the source

A source with doubled vertices, holes or parts that cross each other misleads every tool after it.

- [Merge by Distance](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/mesh/merge.html) and the [Clean Up](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/mesh/cleanup.html) menu: doubled vertices, loose geometry, degenerate faces, holes.
- [Voxel Remesh](https://docs.blender.org/manual/en/latest/modeling/meshes/retopology.html): welds overlapping parts into one closed shell.
- [Decimate](https://docs.blender.org/manual/en/latest/modeling/modifiers/generate/decimate.html): lightens a scan of millions of triangles, so that the viewport and snapping stay fast.
- [Snap to Symmetry](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/mesh/snap_symmetry.html): makes a nearly symmetric source exactly symmetric, so that one half can be mirrored.

## 2. Plan the flow

Decide where the loops go before making them: round the eyes and the mouth, along the limbs, at the joints.

- [Face Sets](https://docs.blender.org/manual/en/latest/sculpt_paint/sculpting/editing/face_sets.html): paint the regions of the source in Sculpt Mode, one per part.
- [Mark Seam](https://docs.blender.org/manual/en/latest/modeling/meshes/uv/unwrapping/seams.html): the cuts of the future UVs, when they decide where loops must run.

## 3. Make the quads

**Automatic.** [QuadriFlow Remesh](https://docs.blender.org/manual/en/latest/modeling/meshes/retopology.html) is the only engine in Blender whose quads follow the shape; add-ons, openclare among them, run others: see [the tools](retopology.md). The result is a start, to be corrected by hand in step 4.

**By hand**, on top of the source:

- the [Retopology overlay](https://docs.blender.org/manual/en/latest/editors/3dview/display/overlays.html), which shows the new mesh through the source;
- [snapping](https://docs.blender.org/manual/en/latest/editors/3dview/controls/snapping.html) to faces (Face Project, Face Nearest), so that each new vertex lands on the surface;
- the [Mirror](https://docs.blender.org/manual/en/latest/modeling/modifiers/generate/mirror.html) modifier with Clipping, to make one half;
- [Poly Build](https://docs.blender.org/manual/en/latest/modeling/meshes/tools/poly_build.html), to place vertices and quads one by one;
- [Extrude](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/vertex/extrude_vertices.html), [Loop Cut](https://docs.blender.org/manual/en/latest/modeling/meshes/tools/loop.html) and the [Knife](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/mesh/knife_topology_tool.html), to grow and split strips;
- [Grid Fill](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/face/grid_fill.html) and [Bridge Edge Loops](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/edge/bridge_edge_loops.html), to fill a hole or join two loops with quads;
- [Triangles to Quads](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/face/triangles_quads.html), to pair the triangles left over.

## 4. Fit and relax

- the [Shrinkwrap](https://docs.blender.org/manual/en/latest/modeling/modifiers/deform/shrinkwrap.html) modifier: puts every vertex back on the source after edits;
- [Smooth Vertices](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/vertex/smooth_vertices.html): evens the quads out; Shrinkwrap brings them back to the surface;
- [Edge Slide](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/edge/edge_slide.html): moves a loop along the surface without changing its shape.

## 5. Check

- [Select All by Trait](https://docs.blender.org/manual/en/latest/modeling/meshes/selecting/all_by_trait.html): Non Manifold for holes and doubled edges, Faces by Sides for the triangles and n-gons left;
- [Select Loops](https://docs.blender.org/manual/en/latest/modeling/meshes/selecting/loops.html): Alt-click a loop and see where it goes, round once or in a spiral;
- [Mesh Analysis](https://docs.blender.org/manual/en/latest/modeling/meshes/mesh_analysis.html): distortion, sharp angles, intersections;
- the [Subdivision Surface](https://docs.blender.org/manual/en/latest/modeling/modifiers/generate/subdivision_surface.html) modifier: what the mesh looks like smoothed, and where poles pinch it.

## 6. Unwrap

- [Mark Seam](https://docs.blender.org/manual/en/latest/modeling/meshes/uv/unwrapping/seams.html) then [Unwrap](https://docs.blender.org/manual/en/latest/modeling/meshes/editing/uv.html): the UVs of the new mesh, on which the detail is baked.

## 7. Carry the detail back

- [Bake](https://docs.blender.org/manual/en/latest/render/cycles/baking.html) in Cycles, Selected to Active: the source's detail into a normal map on the new mesh;
- the [Multiresolution](https://docs.blender.org/manual/en/latest/modeling/modifiers/generate/multiresolution.html) modifier: subdivide the new mesh and Reshape it onto the source, to keep sculpting;
- the [Data Transfer](https://docs.blender.org/manual/en/latest/modeling/modifiers/modify/data_transfer.html) modifier: carries vertex colours, weights or UVs from the source to the new mesh.

A fact is wrong or a link broken: open an [issue](https://github.com/marcgardent/openclare/issues).
