#!/usr/bin/env python3
"""Conflicts between held turns (Section "Conflicts between turns", Table
`tab:iis`; the first paragraphs of Section "An exact look at the closing
system" of the closing-loops paper): for each way, the held turns that
cannot be closed even alone within the bounds of separation, and irreducible
infeasible sets of held turns, found as the papers found them: a deletion
filter over the held turns in the engine's order, the first turn of each set
found then dropped and the search repeated (at most --most sets a way). For
each set, an exact Farkas certificate (`../lean/Check/certify.py`: the least
Σ|u| with c · b = 1, checked in rational arithmetic; also scaled to coprime
whole numbers), its bounded arcs, and
for each turn left out, whole lengths closing the others, checked exactly
(real lengths, a vertex of the linear program, times their common
denominator).
The sets are then grouped across ways (the same turns, up to sign: a row of
the table).

    python3 docs/research/turns-of-a-t-mesh/experiments/conflicts.py [<export>] [--most 12]

Writes results/conflicts.json, and under --out `claims.json`, the rows in
the form `../lean/Check/certify.py --claims` reads, so that `make
lean-check` checks the table on the same export (README, "Table tab:iis").
About a minute a way on the male body.
"""

import argparse
import json
import math
import time

import tmesh  # first: it sets the threads of the linear algebra


def text(fraction):
    return str(fraction) if fraction.denominator != 1 else str(fraction.numerator)


def primitive(u):
    """Multipliers scaled to coprime whole numbers."""
    scale = math.lcm(*(x.denominator for x in u))
    whole = [int(x * scale) for x in u]
    divisor = math.gcd(*whole) or 1
    return [w // divisor for w in whole]


def deletion_filter(way, alive):
    """An irreducible infeasible subset of the turns `alive` (places in the
    way file), every proper subset closable."""
    subset = list(alive)
    for k in list(subset):
        trial = [j for j in subset if j != k]
        if way.closable([way.shift[j] for j in trial]) is None:
            subset = trial
    return subset


def describe(way, turns):
    """The certificate of a set and what closes it without each of its turns."""
    found = way.certificate(turns)
    if found is None:
        return {"turns": turns, "certificate": None}
    u, c = found
    combination = {a: x for a, x in c.items()}
    irreducible, how = [], []
    for t in turns:
        lengths, found_by = way.whole([s for s in turns if s != t])
        irreducible.append(lengths is not None)
        how.append(found_by)
    return {
        "turns": [way.number_of[t] for t in turns],
        "multipliers": [text(x) for x in u],
        "primitive_multipliers": primitive(u),
        "form": {str(a): text(x) for a, x in sorted(combination.items())},
        "form_arcs": len(combination),
        "bounded": sorted(a for a in combination if way.b[a] > 0),
        "bounded_target": {str(a): round(float(way.target[a]), 3) for a in sorted(combination) if way.b[a] > 0},
        "exact": True,
        "closable_without_each": irreducible,
        "whole_lengths_found_by": how,
        "around": [round(float(way.turns[t]["around"]), 2) for t in turns],
        "real_shift": [round(way.value(way.shift[t], way.target), 3) for t in turns],
        "keys": [tmesh.key(way.shift[t]) for t in turns],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "conflicts")
    parser.add_argument("--most", type=int, default=12, help="sets searched at most per way")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    per_way, found = [], []
    for way in ways:
        held = [way.held_index[h] for h in sorted(way.held_index)]
        tick = time.monotonic()
        everything_nonnegative = way.closable([way.shift[k] for k in held], lower=[0] * way.n) is not None
        everything = way.closable([way.shift[k] for k in held]) is not None
        alone = [k for k in held if way.closable([way.shift[k]]) is None]
        sets, alive = [], list(held)
        while len(sets) < arguments.most and way.closable([way.shift[k] for k in alive]) is None:
            subset = deletion_filter(way, alive)
            sets.append(describe(way, subset))
            alive = [k for k in alive if k != subset[0]]
        rest = way.closable([way.shift[k] for k in alive]) is not None
        per_way.append({
            "way": way.number, "held": len(held),
            "held_closable_with_lengths_nonnegative": everything_nonnegative,
            "held_closable_within_the_bounds": everything,
            "not_closable_alone": [way.number_of[k] for k in alone],
            "sets": len(sets), "rest_closable": rest, "seconds": round(time.monotonic() - tick, 1),
        })
        for s in sets:
            found.append((way.number, s))
        print(f"way {way.number}: {len(held)} held, closable with ℓ ≥ 0: {everything_nonnegative}, within the "
              f"bounds: {everything}; alone not closable: {[way.number_of[k] for k in alone]}; "
              f"{len(sets)} sets: {[s['turns'] for s in sets]} ({per_way[-1]['seconds']} s)", flush=True)

    # The rows of the table: the same set of turns, up to sign, in several ways.
    rows = {}
    for number, s in found:
        if s.get("certificate", True) is None:
            continue
        key = tuple(sorted(tuple(k) for k in s["keys"]))
        row = rows.setdefault(key, {"ways": {}, "first": s, "first_way": number})
        row["ways"][number] = s["turns"]
    table = []
    for key, row in rows.items():
        s = row["first"]
        table.append({
            "ways": sorted(row["ways"]),
            "size": len(s["turns"]),
            "turns": s["turns"],
            "turns_in_each_way": {str(k): v for k, v in sorted(row["ways"].items())},
            "multipliers": s["multipliers"],
            "primitive_multipliers": s["primitive_multipliers"],
            "bounded": s["bounded"],
            "bounded_target": s["bounded_target"],
            "form": s["form"],
            "form_arcs": s["form_arcs"],
            "exact_check": "yes",
            "closable_without_each": f"{sum(s['closable_without_each'])} of {len(s['turns'])}",
            "around": s["around"],
            "real_shift": s["real_shift"],
        })
    table.sort(key=lambda row: (row["size"], -len(row["ways"]), row["ways"]))
    for row in table:
        print(f"ways {row['ways']}, {row['size']} turns {row['turns']}: u {row['multipliers']}, bounded "
              f"{row['bounded']}, form of {row['form_arcs']} arcs; closable without each: "
              f"{row['closable_without_each']}")

    claims = {
        "about": "Rows found by conflicts.py on this export: each set of held turns, its multipliers "
                 "(those of the first way listed) and bounded arcs; turns are held indices.",
        "export": {way.path.name: way.sha for way in ways},
        "ways": {str(way.number): {"held": len(way.held)} for way in ways},
        "sets": [{"row": i + 1, "ways": row["turns_in_each_way"], "multipliers": row["multipliers"],
                  "bounded": row["bounded"], "exact": "yes"} for i, row in enumerate(table)],
    }
    (arguments.out / "claims.json").write_text(json.dumps(claims, indent=1) + "\n")
    summary = {
        "experiment": "irreducible infeasible sets of held turns (Table tab:iis)",
        **tmesh.record(arguments, folder, ways, started, clock, most_sets_per_way=arguments.most),
        "per_way": per_way,
        "table": table,
        "claims_for_certify": str((arguments.out / "claims.json").name),
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
