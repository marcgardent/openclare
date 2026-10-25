# Fixtures of the check

Two small T-meshes exported by the engine at commit `300a058`, so that CI runs
the check on every change of the Lean folder (`.github/workflows/lean.yml`):

- `torus`: the bench scene `torus`, 3,000 quads, preset organic; one way, 4 turns.
- `blob`: the bench scene `blob`, 3,000 quads, preset organic; six ways, 136 turns.

Written with `OPENCLARE_EXPORT_TURNS=<directory> openclare remesh <source.obj>
<out.obj> --preset organic --quads 3000`; only the `way-<n>.json` files are kept.
They are frozen: the engine may change, these files test the Lean side. The full
check, on the male body and on fresh exports, is run with `make lean-check` (see
`../README.md`).
