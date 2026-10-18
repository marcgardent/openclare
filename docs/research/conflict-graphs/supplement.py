#!/usr/bin/env python3
"""Second pass over the exported ring systems (see analyse.py):

- level V (the research note's bounds of separation and the walks, no box):
  which conflicts come from the walks alone;
- random weights at levels V and W: is the best set under the pairwise
  conflicts closable for other weights too, or do cores of three or more
  rings appear (the hypergraph that pairs miss)?
- whole lengths at level W: singles, pairs and the optimum with x whole,
  each a small mixed-integer feasibility problem, against the same in real
  numbers.

Usage: supplement.py <out.json> <instance.json>...
"""

import itertools
import json
import os
import sys
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyse  # noqa: E402
import graphs  # noqa: E402

TRIALS = 20
WHOLE_ARCS = 1400
WHOLE_LIMIT = 30.0


def whole_closable(inst, level, chosen, memo, stats):
    key = (level, frozenset(chosen))
    if key in memo:
        return memo[key]
    size, a_eq, b_eq, a_ub, b_ub, bounds = inst.system(level, sorted(chosen))
    constraints = [LinearConstraint(a_eq, b_eq, b_eq)]
    if a_ub is not None:
        constraints.append(LinearConstraint(a_ub, -np.inf, b_ub))
    lb = np.array([b[0] for b in bounds], float)
    ub = np.array([np.inf if b[1] is None else b[1] for b in bounds], float)
    integrality = np.zeros(size)
    integrality[: inst.n] = 1
    clock = time.process_time()
    result = milp(np.zeros(size), constraints=constraints, bounds=Bounds(lb, ub), integrality=integrality, options={"time_limit": WHOLE_LIMIT})
    stats["seconds"] += time.process_time() - clock
    stats["count"] += 1
    if result.status == 0:
        ok = True
    elif result.status == 2:
        ok = False
    else:
        ok = None
        stats["unknown"] += 1
    memo[key] = ok
    return ok


def probe(inst, level, pairs, forced, rng):
    """Random weights: the cores of three or more rings found when the best
    set under the pairs (and the cores so far) is not closable."""
    cores = [tuple(p) for p in pairs]
    higher = []
    failures = 0
    for trial in range(TRIALS):
        weight = rng.exponential(size=inst.m) if trial else np.ones(inst.m)
        first = True
        for _ in range(100):
            chosen, _ = analyse.best_set(inst.m, weight, cores, set(forced))
            if chosen is None or inst.closable(level, chosen):
                break
            if first:
                failures += 1
                first = False
            core = inst.core_within(level, chosen)
            higher.append(core)
            cores.append(tuple(core))
    return higher, failures


def main():
    target = sys.argv[1]
    results = []
    if os.path.exists(target):
        with open(target) as f:
            results = json.load(f)
    done = {r["instance"] for r in results}
    rng = np.random.default_rng(1)
    for path in sys.argv[2:]:
        inst = analyse.Instance(path)
        if inst.name in done or inst.m == 0:
            continue
        print(f"{inst.name}: {inst.n} arcs, {inst.m} rings", flush=True)
        clock = time.process_time()
        record = {"instance": inst.name}
        out = []
        for level in ("V", "W"):
            analyse.analyse_level(inst, level, out)
        record["levels"] = out
        for entry in out:
            higher, failures = probe(inst, entry["level"], entry["pairs"], entry["forced"], rng)
            entry["probe_higher_cores"] = higher
            entry["probe_failures"] = failures
            print(f"  probe {entry['level']}: {failures} of {TRIALS} weightings need more than pairs, cores {[len(c) for c in higher]}", flush=True)
        # Whole lengths at level W.
        if inst.n <= WHOLE_ARCS:
            w = out[1]
            memo, stats = {}, {"seconds": 0.0, "count": 0, "unknown": 0}
            forced = [k for k in range(inst.m) if whole_closable(inst, "W", [k], memo, stats) is not True]
            free = [k for k in range(inst.m) if k not in forced]
            real_pairs = {tuple(p) for p in w["pairs"]}
            pairs = []
            for a, b in itertools.combinations(free, 2):
                if (a, b) in real_pairs or whole_closable(inst, "W", [a, b], memo, stats) is not True:
                    pairs.append((a, b))
            cores = [tuple(p) for p in pairs]
            higher = []
            best = None
            for _ in range(200):
                chosen, _ = analyse.best_set(inst.m, inst.weight, cores, set(forced))
                if chosen is None:
                    break
                ok = whole_closable(inst, "W", chosen, memo, stats)
                if ok:
                    best = chosen
                    break
                core = list(chosen)
                for k in list(core):
                    rest = [j for j in core if j != k]
                    if whole_closable(inst, "W", rest, memo, stats) is not True:
                        core = rest
                higher.append(core)
                cores.append(tuple(core))
            graph = {k: set() for k in free}
            for a, b in pairs:
                graph[a].add(b)
                graph[b].add(a)
            touched = sorted({k for e in pairs for k in e})
            cls = graphs.classes(graphs.induced(graph, touched)) if pairs else {"perfect": True, "odd_hole": None, "odd_antihole": None}
            record["whole_W"] = {
                "forced": forced,
                "pairs": pairs,
                "extra_pairs": [p for p in pairs if p not in real_pairs],
                "higher_cores": higher,
                "optimum": float(inst.weight[best].sum()) if best is not None else None,
                "best": best,
                "perfect": cls["perfect"],
                "odd_hole": cls["odd_hole"],
                "odd_antihole": cls["odd_antihole"],
                "milps": stats,
            }
            print(
                f"  whole W: forced {len(forced)} (real {len(w['forced'])}), pairs {len(pairs)} (real {len(real_pairs)}), "
                f"higher {[len(c) for c in higher]}, optimum {record['whole_W']['optimum']} (real {w['optimum']}), "
                f"perfect {cls['perfect']}, {stats}",
                flush=True,
            )
        record["cpu_seconds"] = time.process_time() - clock
        results.append(record)
        with open(target, "w") as f:
            json.dump(results, f, indent=1)


if __name__ == "__main__":
    main()
