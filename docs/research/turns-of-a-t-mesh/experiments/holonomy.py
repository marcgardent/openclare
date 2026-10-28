#!/usr/bin/env python3
"""The statements of Section "Turns and singularities" checked on an export
(its paragraph "On the male body"), for every turn of every way:

- crossing maps: the maps of the patches crossed, composed along the walk,
  are a translation whose across component is the exported shift, modulo
  the balances (each convention of sign counted);
- parity (Proposition "Parity"): at the whole lengths, around + shift is even,
  for every turn and every way round of the strips of the whole lengths;
- curves: self-crossings, and the regions a curve cuts the surface into with
  the total index of each (Lemma "Gauss–Bonnet": a simple curve splits the
  singular vertices 4 + 4; one self-crossing, (3, 3, 2) or (2, 2, 4));
- discs (Proposition "Burgers sums" (a)): on each side of a simple turn, the
  holonomy against the product of the rotations about its singular
  vertices, developed in the disc, in the order of their first visit and of
  their last visit by the contour of the disc; the checkerboard rule;
- annuli between two disjoint simple turns: their index (0), the checkerboard
  rule for s1 - s2; pairs that split the singular vertices alike have equal
  handed shifts (Proposition "Burgers sums" (c)); annuli holding exactly one
  vertex of valence 3 and one of valence 5: identity (i) of Proposition
  "Dislocations", and whether a straight chain or a staircase of arcs joins
  the pair.

    python3 docs/research/turns-of-a-t-mesh/experiments/holonomy.py [<export>] [--no-annuli]

The counts are per way and summed over the ways (with repeats, as the
paper's). Minutes a way for the turns and discs; the annuli, a pair of
simple turns at a time, take most of the time (an hour or more for the eight
ways of the male body). Writes results/holonomy.json.
"""

import argparse
import itertools
import time
from collections import Counter

import tmesh  # first: it sets the threads of the linear algebra
from geometry import (DIRECTION, Dense, Layout, contour_order, dense, develop, developed_nodes, form_add,
                      form_scale, form_value, shift_form, staircases, straight, vector_sub)


def across(M, side):
    """⟨t, d⟩, d the direction of the first side of the walk."""
    d = DIRECTION[side]
    return M.t0 * d[0] + M.t1 * d[1]


def white_odd(L, nodes, places, x):
    """The odd-valence singular nodes, and those on white points (p1 + p2 odd)."""
    odd = [n for n in nodes if L.singular(n) and L.nodes[n]["angle"] % 2 == 1]
    return odd, [n for n in odd if (form_value(places[n][0], x) + form_value(places[n][1], x)) % 2 != 0]


def turns_and_discs(L, tally, extremes):
    x = L.whole
    n = len(L.arcs)
    simple = {}
    for tid, turn in enumerate(L.turns):
        M = L.walk(turn)
        s = dense(shift_form(turn), n)
        tally["turns"] += 1
        tally["crossing maps: a translation"] += M.k == 0
        a = across(M, turn["side"])
        hand = 1 if turn["side"] in (0, 1) else -1
        for sign in (1, -1):
            tally[f"crossing maps: shift = {'+' if sign > 0 else '-'}⟨t, d⟩"] += L.same_mod_balances(sign * a, s)
            tally[f"crossing maps: shift = {'+' if sign > 0 else '-'}h⟨t, d⟩"] += L.same_mod_balances(
                sign * hand * a, s)
        tally["parity: around + shift even"] += int(M.t0 @ x + M.t1 @ x) % 2 == 0
        crossings = L.crossings_between(tid, tid)
        regions, sides, find, node_cell = L.split([tid])
        keys = [k for k in regions if k is not None]
        indices = tuple(sorted(sum(L.index(n) for n in regions[k]) for k in keys))
        extremes["self-crossings, most"] = max(extremes.get("self-crossings, most", 0), crossings)
        extremes["regions of one curve, most"] = max(extremes.get("regions of one curve, most", 0), len(keys))
        tally[f"curves: regions = self-crossings + 2"] += len(keys) == crossings + 2
        if crossings == 0:
            tally["curves: simple"] += 1
            tally["curves: simple, indices 4 + 4"] += indices == (4, 4)
            tally["curves: simple, held"] += bool(turn["held"])
        elif crossings == 1:
            tally[f"curves: one self-crossing, indices {indices}"] += 1
        else:
            tally["curves: two or more self-crossings"] += 1
        tally["held"] += bool(turn["held"])
        if crossings or len(keys) != 2:
            continue
        simple[tid] = {k: frozenset(n for n in regions[k] if L.singular(n)) for k in keys}
        whole_shift = int(s @ x)
        for root in keys:
            singular = [n for n in regions[root] if L.singular(n)]
            tally["discs"] += 1
            extremes["singular vertices in a disc, least"] = min(extremes.get("singular vertices in a disc, least",
                                                                              10 ** 9), len(singular))
            extremes["singular vertices in a disc, most"] = max(extremes.get("singular vertices in a disc, most", 0),
                                                                len(singular))
            by_first, by_last, first = contour_order(L, turn, root, find, L.exact)
            places = {v: (dense(first[v][0], n), dense(first[v][1], n)) for v in first}
            for name, order in (("first", by_first), ("last", by_last)):
                matches = []
                for sign, reverse in itertools.product((1, -1), (False, True)):
                    sequence = list(reversed(order)) if reverse else order
                    composed = Dense.identity(n)
                    for v in sequence:
                        composed = Dense.rotation_about(sign * L.index(v), *places[v]).after(composed)
                    for inverse, candidate in ((False, composed), (True, composed.inverse())):
                        if candidate.k == 0 and L.same_mod_balances(candidate.t0, M.t0) \
                                and L.same_mod_balances(candidate.t1, M.t1):
                            matches.append((sign, reverse, inverse))
                # The paper's convention: rotations by +(4 - valence), in the
                # order of the visits, the product or its inverse (the two
                # sides of the curve).
                tally[f"discs: Burgers sum, order of {name} visit"] += any(
                    m[0] == 1 and not m[1] for m in matches)
                tally[f"discs: some convention, order of {name} visit"] += bool(matches)
            motion = develop(L, root, find)
            places = developed_nodes(L, root, find, motion)
            odd, white = white_odd(L, regions[root], places, x)
            tally["discs: checkerboard rule"] += (len(white) - whole_shift) % 2 == 0
    return simple


def strips_parity(way, tally):
    for strip in way.helices or []:
        tally["strips: ways round"] += 1
        tally["strips: around + pitch even"] += (round(strip["around"]) + round(strip["pitch"])) % 2 == 0


def annuli(L, simple, tally, limit=None):
    """Pairs of disjoint simple turns bounding an annulus (as parity.py of the
    papers' first version selected them)."""
    x = L.whole
    for t1, t2 in itertools.islice(itertools.combinations(sorted(simple), 2), limit):
        small1 = min(simple[t1].values(), key=len)
        small2 = min(simple[t2].values(), key=len)
        if not (small1 <= small2 or small2 <= small1 or not (small1 & small2)):
            continue
        if L.crossings_between(t1, t2):
            continue
        regions, sides, find, node_cell = L.split([t1, t2])
        keys = [k for k in regions if k is not None]
        if len(keys) != 3:
            continue
        middle = [k for k in keys if k in sides[0] and k in sides[1]]
        if len(middle) != 1:
            continue
        root = middle[0]
        singular = [n for n in regions[root] if L.singular(n)]
        f1, f2 = shift_form(L.turns[t1]), shift_form(L.turns[t2])
        if L.same_mod_balances(f1, f2) and L.same_mod_balances(f1, form_scale(f2, -1)):
            continue
        tally["annuli"] += 1
        tally["annuli: index 0"] += sum(L.index(n) for n in regions[root]) == 0
        places = developed_nodes(L, root, find, develop(L, root, find))
        odd, white = white_odd(L, regions[root], places, x)
        tally["annuli: checkerboard rule"] += (len(white) - (form_value(f1, x) - form_value(f2, x))) % 2 == 0
        if not singular:
            tally["annuli without a singular vertex"] += 1
            tally["annuli without a singular vertex: same shift up to sign"] += (
                L.same_mod_balances(f1, f2) or L.same_mod_balances(f1, form_scale(f2, -1)))


def parallel_pairs(L, tally, limit=None):
    """Pairs of disjoint simple turns splitting the singular vertices alike
    (cylinders.py of the papers' first version): equal handed shifts."""
    part = {}
    for tid in range(len(L.turns)):
        if L.crossings_between(tid, tid):
            continue
        regions, sides, find, node_cell = L.split([tid])
        keys = [k for k in regions if k is not None]
        if len(keys) == 2:
            part[tid] = min((frozenset(v for v in regions[k] if L.singular(v)) for k in keys),
                            key=lambda s: (len(s), sorted(s)))
    for t1, t2 in itertools.islice(itertools.combinations(sorted(part), 2), limit):
        if part[t1] != part[t2] or L.crossings_between(t1, t2):
            continue
        f1, f2 = shift_form(L.turns[t1]), shift_form(L.turns[t2])
        h1 = 1 if L.turns[t1]["side"] in (0, 1) else -1
        h2 = 1 if L.turns[t2]["side"] in (0, 1) else -1
        tally["parallel pairs, no singular vertex between"] += 1
        tally["parallel pairs: handed shifts equal"] += L.same_mod_balances(form_scale(f1, h1), form_scale(f2, h2))


def dislocations(L, simple_sides, tally, found, limit=None):
    """Annuli holding exactly one vertex of valence 3 and one of valence 5
    (annuli35.py of the papers' first version), each pair of shifts once."""
    seen = set()
    for t1, t2 in itertools.islice(itertools.combinations(sorted(simple_sides), 2), limit):
        candidates = [a - b for a in simple_sides[t1] for b in simple_sides[t2] if b <= a and len(a - b) == 2]
        if not candidates:
            continue
        pair = candidates[0]
        if sorted(L.nodes[v]["angle"] for v in pair) != [3, 5]:
            continue
        if L.crossings_between(t1, t2):
            continue
        regions, sides, find, node_cell = L.split([t1, t2])
        keys = [k for k in regions if k is not None]
        middle = [k for k in keys if k in sides[0] and k in sides[1]]
        if len(keys) != 3 or len(middle) != 1:
            continue
        root = middle[0]
        singular = sorted((v for v in regions[root] if L.singular(v)), key=lambda v: L.nodes[v]["angle"])
        if [L.nodes[v]["angle"] for v in singular] != [3, 5]:
            continue
        f1, f2 = shift_form(L.turns[t1]), shift_form(L.turns[t2])
        key = (tuple(sorted(f1.items())), tuple(sorted(f2.items())))
        if key in seen:
            continue
        seen.add(key)
        tally["dislocations: annuli with one 3-5 pair"] += 1
        places = developed_nodes(L, root, find, develop(L, root, find))
        c3, c5 = singular
        d = vector_sub(places[c5], places[c3])
        identity = None
        for e1, e2, sg in itertools.product((1, -1), repeat=3):
            difference = form_add(f1, form_scale(f2, -sg))
            if L.same_mod_balances(form_add(form_scale(d[0], e1), form_scale(d[1], e2)), difference):
                identity = sg
        tally["dislocations: identity (i)"] += identity is not None
        if identity is None:
            continue
        difference = form_add(f1, form_scale(f2, -identity))
        paths = staircases(L, c3, c5, 8)
        lines = [arcs for _, arcs in paths if straight(L, c3, arcs)]
        equal = [arcs for _, arcs in paths
                 if any(L.same_mod_balances(form_scale(difference, s), {a: 1 for a in arcs}) for s in (1, -1))]
        tally["dislocations: the pair on one straight chain of arcs"] += bool(lines)
        tally["dislocations: a staircase whose length is the difference"] += bool(equal)
        found.append({"way": L.way.number, "turns": [L.way.number_of.get(t, None) if L.turns[t]["held"] else None
                                                     for t in (t1, t2)],
                      "places": [t1, t2], "pair": [c3, c5],
                      "d_at_exact_lengths": [round(form_value(d[0], L.exact), 3), round(form_value(d[1], L.exact), 3)],
                      "straight_chains": lines[:3], "staircases_equal": equal[:3]})


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "holonomy")
    parser.add_argument("--no-annuli", action="store_true", help="skip the pairs of turns (most of the time)")
    parser.add_argument("--pairs", type=int, help="only the first pairs of simple turns (a quick try)")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    total, extremes, rows, pairs = Counter(), {}, [], []
    for way in ways:
        tick = time.monotonic()
        L = Layout(way)
        tally, local = Counter(), {}
        simple = turns_and_discs(L, tally, local)
        strips_parity(way, tally)
        if not arguments.no_annuli:
            annuli(L, simple, tally, arguments.pairs)
            parallel_pairs(L, tally, arguments.pairs)
            sides = {t: list(v.values()) for t, v in simple.items()}
            dislocations(L, sides, tally, pairs, arguments.pairs)
        for key, value in local.items():
            extremes[key] = (min if "least" in key else max)(extremes.get(key, value), value)
        tally = Counter({key: int(value) for key, value in tally.items()})
        total.update(tally)
        rows.append({"way": way.number, "counts": dict(sorted(tally.items())), "extremes": local,
                     "seconds": round(time.monotonic() - tick, 1)})
        print(f"way {way.number}: {dict(sorted(tally.items()))} ({rows[-1]['seconds']} s)", flush=True)
    summary = {
        "experiment": "turns and singularities, checked on every turn (Section 'Turns and singularities')",
        **tmesh.record(arguments, folder, ways, started, clock, annuli=not arguments.no_annuli,
                       pairs_limit=arguments.pairs),
        "total": dict(sorted(total.items())),
        "extremes": extremes,
        "dislocation_annuli": pairs,
        "per_way": rows,
    }
    for key, value in sorted(total.items()):
        print(f"{key}: {value}")
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
