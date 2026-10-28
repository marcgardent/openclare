#!/usr/bin/env python3
"""What an export holds, counted: the T-mesh (nodes by kind and valence, arcs,
patches, balances), the balances as a bidirected graph, and for each way of
closing its turns (found, held, kept by the projection), its bounds of
separation, its whole lengths (quads, deviation, held turns left shifted)
and the strips of quads they make (ways round, shifted, leaning less than
0.12). The paragraph "Data" of the turns paper and the opening of Section
"An exact look at the closing system" of the closing-loops paper, and the
first two columns of their tables `tab:exact` and `tab:forced`.

    python3 docs/research/turns-of-a-t-mesh/experiments/census.py [<export>] [--call <name>]

Seconds. Writes results/census.json.
"""

import argparse
import time
from collections import Counter

import tmesh  # first: it sets the threads of the linear algebra

import numpy as np


def graph(way):
    """The balances as a bidirected graph: a node per balance, an edge per
    arc between the two balances it appears in, with the signs it has there."""
    columns = [[] for _ in range(way.n)]
    for r, row in enumerate(way.rows):
        for a, c in row.items():
            columns[a].append((r, c))
    signs = Counter()
    odd = 0
    for column in columns:
        if len(column) != 2 or any(abs(c) != 1 for _, c in column):
            odd += 1
            continue
        signs["+/+" if column[0][1] > 0 and column[1][1] > 0 else "-/-" if column[0][1] < 0 and column[1][1] < 0
              else "+/-"] += 1
    # Components, and whether some cycle is unbalanced: give each balance a
    # sign so that every edge reads +/-; an edge that cannot is on an
    # unbalanced cycle.
    rows = len(way.rows)
    sign, component, unbalanced = [0] * rows, [-1] * rows, set()
    adjacent = [[] for _ in range(rows)]
    for column in columns:
        if len(column) == 2:
            (r0, c0), (r1, c1) = column
            adjacent[r0].append((r1, c0 * c1))
            adjacent[r1].append((r0, c0 * c1))
    count = 0
    for root in range(rows):
        if component[root] >= 0:
            continue
        component[root], sign[root] = count, 1
        stack = [root]
        while stack:
            r = stack.pop()
            for s, product in adjacent[r]:
                wanted = -product * sign[r]
                if component[s] < 0:
                    component[s], sign[s] = count, wanted
                    stack.append(s)
                elif sign[s] != wanted:
                    unbalanced.add(count)
        count += 1
    dense = way.B.toarray()
    rank = int(np.linalg.matrix_rank(dense))
    return {
        "arcs_in_two_balances_with_signs": dict(signs),
        "arcs_otherwise": odd,
        "components": count,
        "components_with_an_unbalanced_cycle": len(unbalanced),
        "rank": rank,
        "full_row_rank": rank == rows,
        "kernel_dimension": way.n - rank,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "census")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    first = ways[0]
    data = first.data
    nodes = data.get("nodes")
    tmesh_counts = {"arcs": first.n, "patches": len(data["stations"]) // 2, "balances": len(data["stations"])}
    if nodes is not None:
        kinds = Counter(node["kind"] for node in nodes)
        valences = Counter(node["angle"] for node in nodes if node["kind"] == "singularity")
        tmesh_counts.update(nodes=len(nodes), kinds=dict(kinds),
                            singular_by_valence={str(k): v for k, v in sorted(valences.items())},
                            index_sum=sum(4 - node["angle"] for node in nodes))
    rows = []
    for way in ways:
        held = tmesh.held_forms(way)
        strips = way.helices
        row = {
            "way": way.number,
            "turns_found": len(way.turns),
            "held": len(way.held),
            "kept_by_the_projection": len(way.kept),
            "bounds_of_separation": int((way.lower > 0).sum()),
            "scale": way.scale,
            "quads": way.quads(way.lengths),
            "deviation": way.deviation(way.lengths),
            "held_left_shifted": way.shifted(held, way.lengths),
            "found_left_shifted": way.shifted([way.shift[k] for k in range(len(way.turns))], way.lengths),
            "spirals": way.data.get("spirals"),
        }
        if strips is not None:
            leaning = way.strips(0.12)
            row.update(strips_ways_round=len(strips),
                       strips_shifted=sum(1 for strip in strips if abs(strip["pitch"]) >= 0.5),
                       strips_leaning_below_0_12=len(leaning),
                       strips_leaning_below_0_12_up_to_sign=len({tmesh.key(s["shift"]) for s in leaning}),
                       strips_leaning_below_0_12_shifted=sum(1 for s in leaning if abs(s["pitch"]) >= 0.5))
        rows.append(row)
        print(f"way {way.number}: {row['turns_found']} turns, {row['held']} held, {row['kept_by_the_projection']} "
              f"kept; {row['bounds_of_separation']} bounds; {row['quads']} quads, {row['held_left_shifted']} held "
              f"left shifted" + (f"; {row['strips_ways_round']} ways round, {row['strips_shifted']} shifted, "
                                 f"{row['strips_leaning_below_0_12']} leaning < 0.12" if strips is not None else ""))
    summary = {
        "experiment": "census of an export",
        **tmesh.record(arguments, folder, ways, started, clock),
        "tmesh": tmesh_counts,
        "balances_as_a_graph": graph(first),
        "ways_read": len(rows),
        "turns_found_total": sum(row["turns_found"] for row in rows),
        "held_total": sum(row["held"] for row in rows),
        "bounds_of_separation_range": [min(row["bounds_of_separation"] for row in rows),
                                       max(row["bounds_of_separation"] for row in rows)],
        "strips_ways_round_total": sum(row.get("strips_ways_round", 0) for row in rows),
        "per_way": rows,
    }
    print(f"T-mesh {tmesh_counts}; {summary['turns_found_total']} turns found, {summary['held_total']} held; "
          f"balances: {summary['balances_as_a_graph']}")
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
