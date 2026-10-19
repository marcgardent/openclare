# Research

Two papers came out of openclare's work on edge loops. Both are drafts, not peer reviewed, under
CC BY 4.0; each has a best-effort French translation beside it (`paper.fr.pdf`), the English
version being authoritative.

| Folder | Paper | What it is |
|---|---|---|
| [`closing-loops/`](closing-loops/) | **A, applied.** *Closing Edge Loops at Quantization Time: Helix Constraints for T-Mesh Quantization* | How the engine closes edge loops while it quantizes a T-mesh, measured on the bench and on a human body |
| [`turns-of-a-t-mesh/`](turns-of-a-t-mesh/) | **B, theory.** *Turns of a T-Mesh: Obstructions to Closing Edge Loops* | Why some loops cannot all close: holonomy, conflicts between turns, forced shifts, NP-hardness; with the Lean proofs in `lean/` |

Each folder holds `paper.tex` and its PDF, the figures with the scripts that make them, and the
bibliography. Build with `make paper` (both, or `NOTE=<folder>` for one), check the proofs with
`make proofs`; [`turns-of-a-t-mesh/PUBLISHING.md`](turns-of-a-t-mesh/PUBLISHING.md) says how a
version is released.
