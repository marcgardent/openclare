# Certificates of the check

Inputs of the check of Theorem "Forced shifts" and Remark "Sign-definite turns"
(`../README.md`), each tied to the way files it was made from by their SHA-256:

- `male-body.claims.json`: the claims of the papers on the male body (export
  before commit `c8bf2be`), transcribed by hand: Table `tab:iis` of
  `../../../paper.tex` (turns, multipliers, bounded arcs, exact check), the
  held turns and `τ_P(H)` of each way (Table `tab:forced`, column "bounds", and
  `../../../../closing-loops/paper.tex`, "exactly 1, 5, 1, 4, 1, 6, 1 and 9").
  The turns of a row in its other ways come from the scripts behind the table.
- `male-body.json`: written from it by
  `python3 ../certify.py <male>/way-{0..7}.json --claims male-body.claims.json --out male-body.json`
  (SciPy 1.16.3, HiGHS; on 2026-10-26 one way per run, four at a time, and the
  `ways` merged: the same data), all eight ways; the export itself is not kept in
  git.
- `fixtures.json`: the same for the two fixtures (`../fixtures`), without
  claims: `python3 ../certify.py ../fixtures/blob/way-*.json ../fixtures/torus/way-0.json --out fixtures.json`.

`certify.py` finds the certificates; `Main.lean` trusts none of them and checks
each with the library's definitions.
