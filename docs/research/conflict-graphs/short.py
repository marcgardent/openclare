#!/usr/bin/env python3
"""Level W restricted to the short rings (round at most K sides of the mesh,
the side the square root of the quads asked for): random weightings by
implicit hitting sets, pairs first, to find cores of three rings or more
among them; and the clique LP of their pairwise conflict graph.

Usage: short.py <results.json> <instance directory> <K> [piece...]
"""

import json
import os
import sys

import numpy as np

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyse  # noqa: E402
import graphs  # noqa: E402

TRIALS = 30


def main():
    results = {r["instance"]: r for r in json.load(open(sys.argv[1]))}
    directory, most = sys.argv[2], float(sys.argv[3])
    pieces = sys.argv[4:] or list(results)
    rng = np.random.default_rng(2)
    for piece in pieces:
        r = results[piece]
        w = [entry for entry in r["levels"] if entry["level"] == "W"]
        if not w:
            continue
        w = w[0]
        inst = analyse.Instance(os.path.join(directory, piece + ".json"))
        side = np.sqrt(json.load(open(os.path.join(directory, piece + ".json")))["quads"])
        keep = [k for k in range(inst.m) if inst.weight[k] <= most * side and k not in w["forced"]]
        out = set(range(inst.m)) - set(keep)
        pairs = [tuple(p) for p in w["pairs"] if p[0] in keep and p[1] in keep]
        cores = list(pairs)
        higher, failures, fractional = [], 0, 0
        graph = {k: set() for k in keep}
        for a, b in pairs:
            graph[a].add(b)
            graph[b].add(a)
        touched = sorted({k for e in pairs for k in e})
        cliques = [(c, 1) for c in graphs.maximal_cliques(graphs.induced(graph, touched)) if len(c) > 1] if pairs else []
        for trial in range(TRIALS):
            weight = rng.exponential(size=inst.m) if trial else np.ones(inst.m)
            value, frac, _ = analyse.lp_value(inst.m, weight, cliques, out)
            fractional += bool(frac)
            first = True
            for _ in range(100):
                chosen, _ = analyse.best_set(inst.m, weight, cores, out)
                if chosen is None or inst.closable("W", chosen):
                    break
                if first:
                    failures += 1
                    first = False
                core = inst.core_within("W", chosen)
                higher.append(core)
                cores.append(tuple(core))
        places = []
        for core in higher:
            places.append([(k, round(float(inst.weight[k] / side), 2), [round(c, 2) for c in inst.at[k]]) for k in core])
        print(
            f"{piece}: {len(keep)} rings of at most {most} sides, {len(pairs)} pairs; {failures} of {TRIALS} weightings need "
            f"more than pairs, cores {[len(c) for c in higher]} {places}; clique LP fractional for {fractional} of {TRIALS}",
            flush=True,
        )


if __name__ == "__main__":
    main()
