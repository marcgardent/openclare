# Editing loops: specification

*[Version française](LOOP_EDITING.fr.md)*

**State:** validated on 27 November 2026; milestones 1 to 3 written (add-on 0.29.0), and Move of milestone 5, brought forward (0.30.0): a drag on a loop away from its ends moves it, so Redraw will be Ctrl + drag; the others not yet. Milestone 2 departs from section 1 and 5 in three ways, for now: the analysis is the one *Analyse* made, not started by itself; with *Sizes* the one size before it is the *Largest*; hard edges do not make a row red.

Today a loop is asked for by drawing a curve: *Draw Curve* or *Cut Volume* makes a curve
object, child of the mesh and held on it by a Shrinkwrap, which the engine lays into the work
mesh as a line of edges (`GuideCurve`). The artist edits it as a Blender curve, through its
control points, and sees only after a remesh whether the quad mesh followed it. This
specification is for a tool that edits the loops themselves: put them where the artist wants
them, on the model, see the quads along them while drawing, and move them about until the
result is right.

## 1. The quad and its size

A **quad** is a face of four vertices of the result. Its **size** is the length of its edges,
`h`.

The retopo project of a source asks for a size at each point of its surface, `h(p)`: the one
*Show Sizes* colours the object with and the engine meshes with. It follows from the settings
of the project, by the analysis of the engine (`engine.Analysis`):

- with *Sizes*, from *Largest* and *Smallest* and the shape (`Analysis.at_vertices`);
- with *Quads*, from the number asked for and the shape (`Analysis.at_vertices_for`).

`h(p)` is read at the vertices of the source and taken between them along the surface. The
analysis is kept while the shape and the settings stay the same (`ANALYSES`), so it is
measured once; on a sculpt of millions of faces that takes seconds, and is done away from the
viewport. Until it is ready the size is one for the whole surface, the edge of as many equal
squares as the quads asked for, `sqrt(area / quads)` (decision D1).

`h(p)` is what the project asks for, not what the engine makes: the quads of the result are
near that size, not of it.

## 2. What a loop is

A **loop** is a line on the surface of the **source** (the sculpt), open or closed, along which
the quad mesh must have a line of edges. It has the marks a curve has today: *Sharp*, *Crease*,
*Seam*. It is a hard constraint, like a border or a change of material: a remesh holds it or
says, for that loop, that it did not (section 5).

The loop lives on the source because the result is made again at each remesh and the source is
what the engine reads. It is **seen and picked on both**:

- on the source, drawn as a strip of quads (section 5);
- on the result, drawn over it the same way, and taken from it: an edge loop of the quad mesh
  can be made a loop of the source in one click (gesture *Pick*, section 3).

The tool lays every point on the source, even while the artist looks at the result: rays are
cast against the source's tree whether it is shown or not. The two surfaces are close, so the
point lands under the mouse.

**Storage (decision D2).** The loops are data of the source mesh, `mesh.openclare_loops`, as the
poles are (`poles.py`): each loop a list of points in the space of the object, laid on the
surface, `closed`, its marks, and the state of the last remesh. Not curve objects: ten loops are
not ten objects in the outliner, no Shrinkwrap is evaluated against a sculpt of millions of
faces, and the tool edits points, not Bezier handles. Points that stray from the surface after
the artist sculpts are laid on it again at the next remesh (nearest point). Blender's undo
covers this data in Object Mode: each gesture is one step.

The curves drawn so far keep working: the add-on still reads them (`curves_of`) and sends them
with the loops. *Convert Curves to Loops* turns them into loops, one per stretch, marks kept.

## 3. Gestures

One tool, *Loops*, in the toolbar (T) in Object Mode, on the source or on a result of it (it
then works on its source, as the *Poles* tool does). The loop under the mouse is lit; what a
click would do is shown before it is done, as the snap circle of *Draw Curve* is.

| Gesture | Input | What it does |
|---|---|---|
| **Draw** | left drag | a new loop along the stroke, with everything *Draw Curve* has: the rope, back along the stroke to undo, *Ridge*/*Valley* snap, F for the radius. Let go with the mouse on its start (not the point on the rope, which lags): closed; a ring round the start fills while letting go would close it. The quads along it are drawn as it goes. |
| **Ring** | Alt + left drag | a closed loop all round a limb, as *Cut Volume* makes it. |
| **Pick** | Shift + click on an edge of the result | the edge loop of the quad mesh through that edge becomes a loop: walked through vertices of four edges, stopped at a pole or a border, closed if it comes back. Each Shift + click past a pole carries it on to the next one. |
| **Move** | left drag on a loop, away from its ends | the points within the radius, along the loop, slide on the surface with the mouse, the farther the less (the falloff of Blender's proportional editing); the loop stays on the surface and keeps its spacing. The radius is a circle round the mouse, shown with *Move* as soon as the mouse is over a loop; the wheel changes it. |
| **Redraw** | Ctrl + left drag starting on a loop and ending on it | the stretch between the two places is replaced by the stroke. |
| **Join, Close** | a stroke or a Move that ends on an end | an end let go on the end of another loop joins the two; on the loop's own start, closes it. |
| **Cut** | C + click on a loop | an open loop becomes two; a closed one is opened there. |
| **Delete** | X + click on a loop | the loop goes. A stretch goes by cutting twice and deleting the middle. |
| **Marks** | the header of the tool | *Sharp*, *Crease*, *Seam* for the loops to come, and for the loop under the mouse with M. |

The rules of these gestures (where a stroke joins, what Redraw replaces, what Cut leaves, where
the quads along a loop fall) go in a module that knows nothing of Blender, as `trace.py` does
for a stroke, so that they are tested without a window. The surface, the BVH tree kept while
the shape stays the same, the rope, the snap, the ribbon and the cutter of *Cut Volume* are
those of `draw.py`, moved where both tools reach them.

**Decision D3.** *Loops* takes over *Draw Curve* and *Cut Volume*: its Draw and Ring are theirs.
Proposed: the two tools are removed from the toolbar once *Loops* draws (milestone 3), and the
curve objects stay readable.

## 4. What goes to the engine

Nothing new at first. A loop is a `GuideCurve` (`points`, `closed`, `sharp`) in
`Guides::curves`, as a curve is now; the crease and the seam stay marks the add-on puts on the
result from the index of the curve of each edge (`curve_edges`). The add-on sends the loops and
the curve objects in one list, in that order, and keeps the index of each. The size of
section 1 is the engine's own; the tool reads it and sends nothing more.

Two additions to the report, small and in the engine:

- per curve, the places where the line is broken (`Laid::gaps` is counted for all curves now;
  `Laid::curve` already knows the curve of each stretch);
- per curve, whether it was let go, and why (`guides_let_go` and `line_causes` count pieces
  now, not curves).

What the engine does with a loop is not in this specification: holding every loop, always, is
the work of #44, #47 and #196 and of the branch `claude/curve-on-edges` (a loop through a vertex
of the work mesh). The tool's part is to never let a loop go unseen.

## 5. What is shown

**The quads along a loop.** Each loop is drawn as one row of quads on either side of it, laid on
the surface: edges along the loop on it and at `h` either side, edges across it every `h` along
it, `h` read where they fall. The artist sees, while drawing, how many quads the loop takes and
how close to it the next line of edges runs. For a closed loop, the number of edges round it,
the length divided by `h` along it, is written beside it.

Where the quads cannot be laid, they are drawn in red:

- a bend tighter than `h` (the row inside the bend folds);
- two loops, or a loop and a border or a hard edge, closer than `h` (their rows overlap): the
  engine would need a smaller quad there, which #196 is to ask for;
- an open end, which needs a pole where the line stops, marked by a dot.

These quads are what the size asks for, not the engine's: after a remesh, the result's own
edges along the loop are what tell whether it was held.

**The state of each loop**, after a remesh, in the *Loops* panel (a row per loop: name, marks,
state) and on the strip: **held** (n edges along it), **broken** (k places, drawn as red dots
where the line breaks), **let go** (dashed, red, with the cause). The report of the remesh says
how many loops were held, broken and let go, as it says today how many edges run along the
curves.

Loops are drawn in the colour of their marks (`CURVE_COLOURS`).

## 6. Milestones

One at a time, each its own review, each with `make version`, `FEATURES.md`, `USE_CASES.md` and
the experimental flask.

1. **Loops as data.** `mesh.openclare_loops`, drawn as lines on the source and on its results;
   sent to the engine with the curves; *Convert Curves to Loops*. Done when, on the cube of #195
   and on the ogre, a converted curve gives the same edges as the curve did.
2. **The size and the quads along the loops.** `h(p)` of section 1, from the analysis or uniform
   until it is ready; the strips of quads, the count round a closed loop and the red of
   section 5. Done when, on a sphere of known radius `r` with a size asked for, the count round
   its equator is `2πr / h`, and two loops drawn closer than `h` are red where they are.
3. **Draw and Ring.** The strokes of *Draw Curve* and *Cut Volume* make loops; D3 applied.
   Brought forward by Marc on 27 November 2026, once he had drawn a curve where he wanted a loop.
4. **Pick and Delete.** An edge loop of the result made a loop of the source, and taken away.
   With the states of section 5 (and the two additions of section 4) from here, since a picked
   loop is only worth something if the next remesh says whether it held it. Done when a loop
   picked on a result of the ogre is held by *Remesh Again*, or is said not to be.
5. **Move and Redraw.** Done when the module of rules covers them in tests, and a loop moved
   on the ogre stays on the surface with its spacing.
6. **Cut, Join, Close, Marks.**

## 7. Out of scope, for later

- The number of edges along a loop asked for (12 round an eye), which needs the engine: the
  count of section 5 says what the size gives, it does not ask for another.
- The number of quads between two loops (an edge ring), the same.
- A v2 with a setting of several loops: the rows either side held too, not only the line in the middle (asked by Marc on 27 November 2026). Until then only the loop itself goes to the engine; the rows are what the size asks for.
- A loop drawn on one side mirrored on the other under *Symmetry*; until then, the loops go
  through the cut as the curves do.

## 8. Decisions for review

- **D1**: the quad size of the project is the engine's size field `h(p)`, uniform until the
  analysis is ready (section 1), not one size set by hand for the whole project.
- **D2**: loops as data of the mesh, not curve objects (section 2).
- **D3**: *Loops* replaces *Draw Curve* and *Cut Volume* at milestone 3 (section 3).
- **D4**: the order of the milestones; Draw first, then Pick (changed by Marc on 27 November
  2026: Pick came before Draw).
- **D5**: the keys of section 3.
