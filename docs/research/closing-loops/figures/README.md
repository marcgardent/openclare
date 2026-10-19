# Figures of "Closing Edge Loops at Quantization Time"

What each figure shows, how to make it again, and the numbers measured on it.

| File | What it is | Made by |
|---|---|---|
| `turn.tex` | (a) a band of a T-mesh on a cylinder, a turn γ, its corridor, the shift s_γ(ℓ) = ℓ_b − ℓ_c + ℓ_d = 1; (b) the strip it makes, a helix of pitch 1 | hand (TikZ) |
| `closing.tex` | the same turn before (ℓ_d = 2, s = 1, a helix) and after the lengths close it (ℓ_d = 1, s = 0, three rings) | hand (TikZ) |
| `male-overview.png` | the male body at 10,590 quads asked for, four meshes, edge loops coloured by kind | `make_figures.sh` |
| `male-wrist.png` | close-up of the left wrist and hand (the body's left, +X), same four meshes | `make_figures.sh` |
| `male-eye.png` | close-up of the right eye (the body's right, −X), same four meshes | `make_figures.sh` |

## The TikZ figures

Each file is a `tikzpicture` only, to `\input` inside a `figure`. They need

```latex
\usepackage{tikz}
\usetikzlibrary{arrows.meta,decorations.pathreplacing}
```

They follow Section 3 of the paper: the band is cut open along the first side of P_1, and P_1 is
drawn again, dashed, past the right side of P_4, where it is glued. Every node is at a whole
position and the faint grid is the quad mesh at these lengths. The walk at offset o crosses
P_1 → P_2 → P_3 → P_4; measured from the first corner of each patch its offset grows by ℓ_b,
drops by ℓ_c, grows by ℓ_d, so it comes back at o + s_γ(ℓ). The corridor is 1 < o < 4 (the walk
must cross the arc [0, 4] of the side of P_1 and P_2, and [1, 4] at the next two sides); with
s = 1 the strips at o = 1.5, 2.5, 3.5 are one helix that then leaves the corridor. In
`closing.tex` the column of P_4 sits one unit higher after quantization: ℓ_d and the arc above
P_4 on the same side are one shorter, no node passes another (the T-mesh is the same, only its
lengths changed), and s = 1 − 2 + 1 = 0: three closed rings. Colours: corridor light blue, helix
vermillion (#D55E00), rings dark blue (#0B3C8C), as in the renders.

## The renders

Four meshes of the male body of Blender's Essentials library (CC0), all drawn with the same
orthographic camera per view:

- (a) openclare without closing: `openclare remesh <source> a.obj --preset organic --quads 10590 --no-close-loops`
- (b) openclare with closing (the default): `openclare remesh <source> b.obj --preset organic --quads 10590`
- (c) the hand-made base mesh, `bench/reference/handmade/human_male.obj`
- (d) QuadriFlow, Blender's `bpy.ops.object.quadriflow_remesh` with the parameters of
  `bench/quadriflow.py` (`use_mesh_symmetry=False`, no sharp or border preservation,
  `mode="FACES"`, `target_faces=10590`, seed 0), run by `quadriflow_scaled.py` on the source
  scaled by 100 about the origin and scaled back. Without the scaling Blender refuses the
  source: it reports any edge whose ends are within 1e-4 in every coordinate as "not manifold",
  and the 1.35-million-triangle sculpt, 1.7 units tall, has such edges (bench/quadriflow.py
  stops with "cancelled" for this reason). QuadriFlow does not depend on the scale.

Edge loops are sorted as `openclare measure` sorts them (Section 5): `loops.py` is a line-by-line
port of `Connectivity::edge_loops` in `crates/openclare/src/metrics.rs`, and gives exactly the
counts and shares of `openclare measure --json` on all four meshes (checked: same
`loops_closed`, `loops_spiral`, `loops_open`, and shares equal to 1e-15). Colours: closed loops
dark blue (#0B3C8C), spiral loops vermillion (#D55E00, Okabe–Ito) twice as wide with a dark rim,
so that they stand out from closed loops in greyscale too, loops that stop at an irregular vertex
grey. The 20 triangles of the hand-made mesh are on no loop and drawn grey. The surface is light
grey, shaded by a fixed light baked into vertex colours; Blender's Workbench renders it flat, so
the colours in the images are exactly these. The numbers under each panel are those of the whole
mesh, whatever the view.

### Numbers of each panel

Measured by `openclare measure <source> <mesh> --target 10590 --json` (engine 89232b0).
Distances are per mille of the diagonal of the source's bounding box.

| | (a) no closing | (b) closing | (c) hand-made | (d) QuadriFlow |
|---|---|---|---|---|
| faces (quads) | 10,485 (10,485) | 10,748 (10,748) | 10,590 (10,570 + 20 triangles) | 8,844 (8,844) |
| valid | yes | yes | no (triangles) | no (3 inverted faces) |
| irregular vertices | 249 | 249 | 476 | 224 |
| edges on spiral loops | **26.8%** (18 loops) | **1.95%** (15 loops) | **2.55%** (3 loops) | **35.7%** (43 loops) |
| edges on closed loops | 10.2% (143 loops) | 32.6% (287 loops) | 31.6% (321 loops) | 2.7% (31 loops) |
| mean distance to the source, ‰ | 0.183 | 0.190 | 0.172 | 1.356 |
| Hausdorff distance, ‰ | 3.97 | 4.38 | 6.47 | 31.3 |
| mean corner deviation, degrees | 4.50 | 4.32 | 9.24 | 8.79 |

The engine and QuadriFlow are deterministic here: two runs of `make_figures.sh` gave the same
three meshes byte for byte.

Against preprint v1 (Section 6, "Against the hand-made mesh"): v1 gives 1.6% of spiral edges in
9 spiral loops and 33.2% on closed loops for the engine, from single runs of commits b0d37fa to
8fd1932. The engine of these figures, 89232b0, gives 1.95% in 15 spiral loops and 32.6% on closed
loops: still below the hand-made mesh's 2.55%, by less. The difference is the newer engine (v1
already says that the later changes, a finer work mesh at thin walls, were not measured on this
body, and that earlier versions gave 1.1% to 2.4%); like the spiral share of any single model it
is chaotic (a 1% change of the budget moves it as much), so it is one run, not a trend. The hand-made
mesh's figures (2.55%, 3 loops, 31.6%) are those of v1. QuadriFlow made 8,844 quads for 10,590
asked for; v1 has no QuadriFlow figure on this body because the bench's run was refused (above).

Note on the wrist close-up: closing leaves spiral loops on the thumb and the back of the hand
where the mesh without closing had closed ones; the whole body still goes from 26.8% to 1.95%.

## Making them again

```sh
make essentials        # the male body's scene, from Blender's cache (see bench/fetch_essentials.py)
blender -b --factory-startup bench/corpus-large/scenes/human_male.blend --python-exit-code 1 \
    --python bench/blender_run.py -- --out bench/out/human_male --no-remesh
                       # bench/out/human_male/source.obj, as the bench gives it to every remesher
docs/research/closing-loops/figures/make_figures.sh
```

`make_figures.sh` builds the engine (`cargo build --release`), makes the four meshes into
`target/figures/` (or `$WORK`; meshes already there are reused), measures them, renders twelve
panels (`render_loops.py`, Blender 5.2.2, Workbench, in the background) and puts them together
(`compose.py`, Python with Pillow; Latin Modern Roman for the labels). The source used for these
figures had MD5 `f1fe33002b731d1d41846daed6c59f5e` (1,354,880 triangles); `SOURCE=<path>` picks
another copy.

Files: `loops.py` (the loop classification, also `python3 loops.py <mesh.obj> [--edges out.txt]`
for the kind of every edge), `render_loops.py` (one panel), `compose.py` (four panels, labels and
legend), `quadriflow_scaled.py` (QuadriFlow on the scaled source), `make_figures.sh` (all of it).
