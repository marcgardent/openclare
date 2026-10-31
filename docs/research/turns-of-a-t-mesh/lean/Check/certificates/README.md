# Certificates of the check

Inputs of the check of Theorem "Forced shifts" and Remark "Sign-definite turns"
(`../README.md`), each tied to the way files it was made from by their SHA-256:

- `male-body.claims.json`: the claims of the papers on the male body (export
  of the engine of v0.7.0, e13fadb, as-is): the rows of Table `tab:iis` of
  `../../../paper.tex` (turns, multipliers, bounded arcs, exact check, and the
  same in each way a row is in), written by `../../../experiments/conflicts.py`;
  the held turns and `τ_P(H)` of each way (Table `tab:forced`, column "bounds",
  and `../../../../closing-loops/paper.tex`, "The heuristic leaves most of the
  gap"), from `../../../experiments/forced.py --steps bounds`.
- `male-body.json`: written from it by `python3 ../certify.py` on the eight
  ways, two at a time in four runs, the `ways` merged (SciPy 1.16.3, HiGHS);
  the export itself is not kept in git. `τ_P(H)` is checked exactly in ways 0
  to 4 and 6; in ways 5 and 7 the certified cores bound it, `20 ≤ τ ≤ 30` and
  `28 ≤ τ ≤ 37`, and the claim is checked to lie within.
- `fixtures.json`: the same for the two fixtures (`../fixtures`), without
  claims: `python3 ../certify.py ../fixtures/blob/way-*.json ../fixtures/torus/way-0.json --out fixtures.json`.

`certify.py` finds the certificates; `Main.lean` trusts none of them and checks
each with the library's definitions.
