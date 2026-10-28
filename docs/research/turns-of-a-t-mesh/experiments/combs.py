#!/usr/bin/env python3
"""Combs on an export (Section "Combs", paragraph "On the male body"): the
feet of every arc and Lemma "Combs" (the arc is the offset, across it, of
the singular vertices at the feet of the traces perpendicular to it at its
ends: developed along foot, trace, arc, trace, foot, every step but the arc
runs across it), the valences of the feet, the arcs whose feet coincide and
the arcs ending on a separatrix connection (a trace joining two singular
vertices: the foot taken is where it was traced from); then,
for every certificate of the table of `conflicts.py` (each row in each of its
ways, primitive multipliers), a non-negative representative of the
combination modulo the balances (`../lean/Check/certify.py`, exact) and
whether its odd part is a cycle of the graph of feet (every singular vertex
met an even number of times by the arcs with an odd coefficient).

    python3 docs/research/turns-of-a-t-mesh/experiments/combs.py [<export>] [--way 1] \\
        [--table docs/research/turns-of-a-t-mesh/experiments/results/conflicts.json]

Seconds to a minute. Writes results/combs.json.
"""

import argparse
import json
import math
import time
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path

import tmesh  # first: it sets the threads of the linear algebra
from geometry import Layout, directions_at


class Feet:
    def __init__(self, L):
        self.L = L
        self.trace_arcs = defaultdict(list)
        for a, arc in enumerate(L.arcs):
            self.trace_arcs[arc["trace"]].append(a)
        for t in self.trace_arcs:
            self.trace_arcs[t].sort(key=lambda a: L.arcs[a]["span"][0])
        self.origin = {t: L.arcs[arcs[0]]["nodes"][0] for t, arcs in self.trace_arcs.items()}
        self.ambiguous = {}
        self.connections = {}

    def foot_path(self, node, arc):
        """From an end `node` of `arc`: the singular vertex where the trace
        perpendicular to the arc at that node starts, and the arcs from it to
        the node."""
        L = self.L
        if L.singular(node):
            return node, []
        t = L.arcs[arc]["trace"]
        perpendicular = sorted({L.arcs[h // 2]["trace"] for h in L.nodes[node]["halves"]
                                if h // 2 != arc and L.arcs[h // 2]["trace"] != t})
        if len(perpendicular) != 1:
            self.ambiguous[arc] = perpendicular
        chosen = perpendicular[0]
        end = L.arcs[self.trace_arcs[chosen][-1]]["nodes"][1]
        if L.singular(end):
            # A separatrix connection: the foot is where it was traced from.
            self.connections[arc] = [self.origin[chosen], end]
        path = []
        for b in self.trace_arcs[chosen]:
            path.append(b)
            if L.arcs[b]["nodes"][1] == node:
                break
        return self.origin[chosen], path

    def comb(self, arc):
        n0, n1 = self.L.arcs[arc]["nodes"]
        c0, p0 = self.foot_path(n0, arc)
        c1, p1 = self.foot_path(n1, arc)
        return c0, c1, p0, p1

    def develop(self, start, arcs):
        """The directions of a path of arcs from `start` (mod 4, the first
        step 0) and its end node."""
        L = self.L
        node, out, dirs = start, [], None
        for a in arcs:
            h = 2 * a if L.arcs[a]["nodes"][0] == node else 2 * a + 1
            if L.tail(h) != node:
                return None, None
            if dirs is None:
                dirs = directions_at(L, node, h, 0)
            out.append((a, dirs[h]))
            node = L.head(h)
            dirs = directions_at(L, node, h ^ 1, dirs[h] + 2)
        return out, node


def boundary(form, feet):
    """The singular vertices met an odd number of times by the arcs with an
    odd coefficient (a loop at one vertex counts twice)."""
    odd = Counter()
    for a, c in form.items():
        if c.denominator == 1 and c.numerator % 2:
            c0, c1 = feet[a]
            odd[c0] += 1
            odd[c1] += 1
    return sorted(v for v, k in odd.items() if k % 2)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "combs")
    parser.add_argument("--way", type=int, default=1, help="the way whose T-mesh the lemma is checked on")
    parser.add_argument("--table", type=Path, default=tmesh.HERE / "results" / "conflicts.json",
                        help="the summary of conflicts.py")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, paths = tmesh.ways(arguments.export, arguments.call)
    way = tmesh.Way(paths[arguments.way])
    L = Layout(way)
    F = Feet(L)
    feet, kinds, holds, same = {}, Counter(), 0, []
    for a in range(way.n):
        c0, c1, p0, p1 = F.comb(a)
        steps, end = F.develop(c0, p0 + [a] + list(reversed(p1)))
        ok = steps is not None and end == c1 and all(
            d % 2 != dict(steps)[a] % 2 for b, d in steps if b != a)
        holds += ok
        feet[a] = (c0, c1)
        if c0 == c1:
            same.append(a)
            kinds["the same foot"] += 1
        else:
            kinds["valences " + "-".join(str(v) for v in sorted((L.nodes[c0]["angle"], L.nodes[c1]["angle"])))] += 1
    odd_feet = sum(1 for a, (c0, c1) in feet.items()
                   if c0 != c1 and L.nodes[c0]["angle"] % 2 and L.nodes[c1]["angle"] % 2)
    lemma = {"way": way.number, "arcs": way.n, "comb_lemma_holds": holds,
             "two_distinct_feet_of_odd_valence": odd_feet, "feet_by_valence": dict(kinds),
             "arcs_whose_feet_coincide": same,
             "arcs_with_more_than_one_perpendicular_trace": {str(a): t for a, t in F.ambiguous.items()},
             "arcs_ending_on_a_separatrix_connection": {str(a): ends for a, ends in F.connections.items()}}
    print(f"way {way.number}: comb lemma on {holds} of {way.n} arcs; {odd_feet} with two distinct feet of odd "
          f"valence; feet coincide on {same}; perpendicular trace not unique at {list(F.ambiguous)}; ending on a "
          f"separatrix connection: {F.connections}", flush=True)

    certificates = []
    if arguments.table.exists():
        table = json.loads(arguments.table.read_text())
        layouts = {way.number: (way, feet)}
        for row in table["table"]:
            for number, turns in row["turns_in_each_way"].items():
                number = int(number)
                if number not in layouts:
                    other = tmesh.Way(paths[number])
                    layouts[number] = (other, None)
                other, _ = layouts[number]
                ids = [other.held_index[h] for h in turns]
                found = other.certificate(ids)
                if found is None:
                    certificates.append({"way": number, "turns": turns, "certificate": None})
                    continue
                u, _ = found
                scale = math.lcm(*(x.denominator for x in u))
                whole = [int(x * scale) for x in u]
                divisor = math.gcd(*whole) or 1
                primitive = [w // divisor for w in whole]
                exact = other.certificate(ids, [Fraction(x) for x in primitive])
                c = exact[1] if exact else None
                integral = c is not None and all(x.denominator == 1 for x in c.values())
                # The feet are those of the way checked: the T-mesh is the same in every way.
                odd_boundary = boundary(c, feet) if integral else None
                certificates.append({"way": number, "turns": turns, "multipliers": [str(x) for x in primitive],
                                     "arcs": len(c) if c else None, "whole": integral,
                                     "odd_part_is_a_cycle_of_the_graph_of_feet": odd_boundary == [],
                                     "odd_boundary": odd_boundary})
        print(f"{len(certificates)} certificates; odd part a cycle of the graph of feet in "
              f"{sum(1 for c in certificates if c.get('odd_part_is_a_cycle_of_the_graph_of_feet'))}", flush=True)
    summary = {
        "experiment": "combs: the feet of every arc, and the odd parts of the certificates",
        **tmesh.record(arguments, folder, [way], started, clock, table=arguments.table.name),
        "lemma": lemma,
        "certificates": certificates,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
