"""Converts a T-mesh exported by the engine (`OPENCLARE_EXPORT_TURNS`, a file
`way-<n>.json`) into the combinatorial T-mesh of `TurnsOfATMesh/TMesh.lean`.

    python3 convert.py <way-n.json> <out.json>

Python standard library only. The conventions below are read from the engine
(`quantization/export.rs`, `quantization/tubes.rs`, `tmesh/mod.rs`) and checked
on the data where it allows it; any check that fails stops the conversion.

The engine
----------
* An arc has two half-arcs: half `2a` runs from `nodes[0]` to `nodes[1]`, half
  `2a + 1` back (`tmesh/mod.rs`, `origin(half) = nodes[half % 2]`), and
  `patches[half % 2]` is the patch whose rim holds that half. Checked: on every
  rim each half ends at the node the next one starts from.
* `patches[p]` is the rim of patch `p` from its first corner, as the turns read
  it (`tubes::rim_halves`): a list of `[half, side]`, the sides `0, 1, 2, 3` in
  that order, running counter-clockwise in the frame `(u, v)` of the patch:
  side 0 along `+u` (`v = 0`), 1 along `+v` (`u = w`), 2 along `-u` (`v = h`),
  3 along `-v` (`u = 0`) (`tubes::DIRECTION`). `null` for a patch that is not a
  rectangle. Checked: the sides come in that order, none empty, and agree with
  `arcs[a].sides` and `arcs[a].patches`.
* `stations`: one row per rectangle (in the order of `patches`, skipping the
  `null` ones) and pair `side % 2`, the arcs of sides 0 and 1 with `+1`, those of
  sides 2 and 3 with `-1`, not merged (`export::tmesh_fields`).
* A turn starts on side `side` of patch `patch`, `offset` along it in the
  direction of the rim (from the start of that side), walks across to the
  opposite side and crosses the half-arc it reaches there, and so on
  (`tubes::go_round`); `crossings` lists `[patch left, half crossed, patch
  entered, distance along that half]`. The turns of a way are walked at the
  lengths `arcs[a].length` (`tubes::turns`), here called `walked`. `shift` is
  the component across the walk, along `u` for sides 0 and 2 and along `v` for
  sides 1 and 3, of the translation that takes the frame of the first patch to
  itself after the turn (`tubes::shift_along`).

The Lean model
--------------
A side is `(P, j, s)`: pair `j = 0` holds `v = 0` (`s = 0`) and `v = h`
(`s = 1`), listed along `+u`; pair `j = 1` holds `u = 0` (`s = 0`) and `u = w`
(`s = 1`), listed along `+v`, all from the corner at the origin side. So, with
the side index `4 P + 2 j + s`:

    engine side 0 -> (P, 0, 0), in rim order
    engine side 1 -> (P, 1, 1), in rim order
    engine side 2 -> (P, 0, 1), rim order reversed
    engine side 3 -> (P, 1, 0), rim order reversed

The Lean row `(P, 0)` is the engine's row of pair 0; the Lean row `(P, 1)` is
side `u = 0` minus side `u = w`, the engine's row of pair 1 negated. An offset
of the engine on side 0 or 1 is the Lean offset; on side 2 or 3 the Lean offset
is the length of the side less it. The shift of the engine is along `+u` or
`+v`, the direction of the Lean offsets on both sides of a pair, so it is passed
as it is, with no change of sign. It measures positions round the rim from
the first corner, where the Lean model measures each side from its own first
corner: the two forms can only agree modulo the balances, which is what the
check finds.

The direction of an arc on a Lean side, relative to the arc's own direction
(`nodes[0]` to `nodes[1]`), is `+1` for an even half and `-1` for an odd one,
times `-1` on sides 2 and 3; `sign a` is the product of the two directions
(`+1`: the two sides run along the arc in the same direction).

Output
------
JSON with: `patches`, `arcs` (counts); `sides`, the arcs of each Lean side by
index `4 P + 2 j + s`; `holders`, per arc the Lean sides holding it with its
direction there (`ends` and `sign` are read from them, and the check reports
an arc held by other than two sides); `stations`, the engine's rows with their
`patch` and `pair`, merged; the lengths `whole`, `walked` and `real`, the last
two as exact fractions of the floats `[num, den]`; `turns`, each with its Lean
`start` side, `path` (arcs crossed), the engine's `shift`, the Lean offsets
`offset` (at `walked`) and `share` (at `real`, the same share of the side), and
`crossed`, the Lean offset on the side left at each crossing, from the
engine's `into`; `strips`, the shifts of every strip of quads of the whole
lengths (`way-<n>-helices.json`, when it is there).
"""

import json
import os
import sys
from fractions import Fraction


class Unsupported(Exception):
    """The export is out of the scope of the Lean model (borders, a patch that
    is not a rectangle): reported, not converted."""


# Engine side -> (pair j, s) of the Lean side, and whether the Lean listing
# reverses the rim.
LEAN_SIDE = {0: (0, 0, False), 1: (1, 1, False), 2: (0, 1, True), 3: (1, 0, True)}


def side_index(patch, side):
    j, s, _ = LEAN_SIDE[side]
    return 4 * patch + 2 * j + s


def fraction(x):
    f = Fraction(x)
    return [f.numerator, f.denominator]


def whole(c, what):
    if c != int(c):
        raise ValueError(f"{what}: coefficient {c} is not whole")
    return int(c)


def merged(row, what):
    out = {}
    for arc, c in row:
        out[arc] = out.get(arc, 0) + whole(c, what)
    return sorted([a, c] for a, c in out.items() if c != 0)


def convert(export, helices=None):
    arcs, rims = export["arcs"], export["patches"]
    n, count = len(arcs), len(rims)
    if any(rim is None for rim in rims):
        raise Unsupported(f"{sum(rim is None for rim in rims)} patches are not rectangles")
    border = [a for a, arc in enumerate(arcs) if arc["border"] or -1 in arc["sides"]]
    if border:
        raise Unsupported(f"{len(border)} arcs on a border")

    def start(half):
        return arcs[half // 2]["nodes"][half % 2]

    def end(half):
        return arcs[half // 2]["nodes"][1 - half % 2]

    sides = [[] for _ in range(4 * count)]
    # Per patch and engine side, the halves in rim order.
    rim_sides = []
    for p, rim in enumerate(rims):
        by_side = [[] for _ in range(4)]
        previous = 0
        for k, (half, side) in enumerate(rim):
            if side < previous or side > 3:
                raise ValueError(f"patch {p}: sides out of order on the rim")
            previous = side
            if end(half) != start(rim[(k + 1) % len(rim)][0]):
                raise ValueError(f"patch {p}: half {half} does not end where the next one starts")
            arc = arcs[half // 2]
            if arc["patches"][half % 2] != p or arc["sides"][half % 2] != side:
                raise ValueError(f"patch {p}: half {half} disagrees with arcs[{half // 2}]")
            by_side[side].append(half)
        if any(not halves for halves in by_side):
            raise ValueError(f"patch {p}: a side without arcs")
        rim_sides.append(by_side)
        for side, halves in enumerate(by_side):
            ordered = halves[::-1] if LEAN_SIDE[side][2] else halves
            sides[side_index(p, side)] = [half // 2 for half in ordered]

    holders = [[] for _ in range(n)]
    for p, by_side in enumerate(rim_sides):
        for side, halves in enumerate(by_side):
            for half in halves:
                direction = (1 if half % 2 == 0 else -1) * (-1 if LEAN_SIDE[side][2] else 1)
                holders[half // 2].append([side_index(p, side), direction])

    stations = []
    rows = iter(export["stations"])
    for p in range(count):
        for pair in range(2):
            stations.append({"patch": p, "pair": pair, "row": merged(next(rows), f"station of patch {p}")})
    if next(rows, None) is not None:
        raise ValueError("more stations than patches")

    walked = [Fraction(arc["length"]) for arc in arcs]

    def side_length(p, side, lengths):
        return sum((lengths[h // 2] for h in rim_sides[p][side]), Fraction(0))

    def lean_offset(p, side, offset, lengths):
        return side_length(p, side, lengths) - offset if LEAN_SIDE[side][2] else offset

    turns = []
    for k, turn in enumerate(export.get("turns", [])):
        p, side = turn["patch"], turn["side"]
        crossings = turn["crossings"]
        path = [half // 2 for (_, half, _, _) in crossings]
        patches = [turn["patch"]] + [c[2] for c in crossings]
        if [c[0] for c in crossings] != patches[:-1] or patches[-1] != p:
            raise ValueError(f"turn {k}: crossings do not chain")
        offset = Fraction(turn["offset"])
        length = side_length(p, side, walked)
        share = (offset / length).limit_denominator(20)
        if abs(float(offset / length - share)) > 1e-9:
            share = None
        elif LEAN_SIDE[side][2]:
            share = 1 - share
        # The Lean offset on the side left at each crossing, from `into`, the
        # distance along the half crossed in the direction of its rim.
        crossed = []
        for (q, half, _, into) in crossings:
            opposite = next(s for s in range(4) if half in rim_sides[q][s])
            before = Fraction(0)
            for h in rim_sides[q][opposite]:
                if h == half:
                    break
                before += walked[h // 2]
            crossed.append(fraction(lean_offset(q, opposite, before + Fraction(into), walked)))
        turns.append({
            "index": k,
            "start": side_index(p, side),
            "path": path,
            "shift": merged(turn["shift"], f"shift of turn {k}"),
            "held": turn.get("held", False),
            "offset": fraction(lean_offset(p, side, offset, walked)),
            "share": None if share is None else [share.numerator, share.denominator],
            "crossed": crossed,
        })

    out = {
        "patches": count,
        "arcs": n,
        "sides": sides,
        "holders": holders,
        "stations": stations,
        "whole": export["lengths"],
        "walked": [[f.numerator, f.denominator] for f in walked],
        "real": [fraction(x) for x in export["real"]],
        "turns": turns,
    }
    if helices is not None:
        out["strips"] = [{"shift": merged(s["shift"], "strip"), "pitch": whole(s["pitch"], "pitch")} for s in helices]
    return out


def main():
    source, target = sys.argv[1], sys.argv[2]
    with open(source) as f:
        export = json.load(f)
    helices = None
    sibling = source[: -len(".json")] + "-helices.json"
    if os.path.exists(sibling):
        with open(sibling) as f:
            helices = json.load(f)
    try:
        out = convert(export, helices)
    except Unsupported as reason:
        print(f"skipped {source}: {reason} (out of the scope of the Lean model)")
        sys.exit(3)
    with open(target, "w") as f:
        json.dump(out, f, separators=(",", ":"))
    print(f"converted {source}: {out['patches']} patches, {out['arcs']} arcs, {len(out['turns'])} turns")


if __name__ == "__main__":
    main()
