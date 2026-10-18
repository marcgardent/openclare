#!/usr/bin/env python3
"""Conflicts between rings in openclare's ring-selection problem.

Reads the JSON files written by `OPENCLARE_EXPORT_RINGS=<prefix> openclare
remesh ...` (one per piece: the problem the slow quality mode poses in its
first round) and, for each, at several levels of constraints:

- which rings cannot be closed even alone (forced shifts);
- the pairwise conflict graph (an edge: the two rings cannot be closed
  together), and its classes (bipartite, chordal, interval, comparability,
  co-comparability, perfect by odd hole / odd antihole search);
- the exact largest weight of rings closable together, by an implicit
  hitting-set loop (Parker and Ryan) whose cores are found by a deletion
  filter: the cores of three or more rings are the conflicts that pairs do
  not capture;
- the LP relaxations: the big-M relaxation of the solver's model, the edge
  and clique relaxations of the conflict graph, and the clique relaxation
  with the higher cores; are their optima integral, how far from the
  integer optimum;
- the mixed-integer program itself, with real and with whole lengths, and
  the CPU time of every step.

Levels (each adds to the one before, except S which is the research note's P,
and V which is S with the walks):
  S  balances, lengths at least the engine's bounds of separation, no upper bound
  V  S, and the conditions that keep each closed ring the same walk (no box)
  B  balances, lengths within the box the slow mode allows (target +- 4,
     the reference, the bounds of separation)
  W  B, and the conditions that keep each closed ring the same walk
  C  W, and the cap on the mean deviation from the targets
  Q  C, and the number of quads within 80% of the window, to first order
     around the reference (as the HiGHS selector poses it)

Usage: analyse.py <out.json> <instance.json>...
"""

import itertools
import json
import os
import sys
import time

import numpy as np
from scipy import sparse
from scipy.optimize import Bounds, LinearConstraint, linprog, milp

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import graphs  # noqa: E402

LEVELS = ["S", "B", "W", "C", "Q"]
BUDGET_SHARE = 0.8
INTEGRAL = 1e-6


def rows_matrix(rows, n):
    data, row_index, col = [], [], []
    for k, row in enumerate(rows):
        for arc, c in row:
            row_index.append(k)
            col.append(arc)
            data.append(c)
    return sparse.csr_matrix((data, (row_index, col)), shape=(len(rows), n))


class Instance:
    def __init__(self, path):
        with open(path) as f:
            d = json.load(f)
        self.name = os.path.basename(path)[: -len(".json")]
        self.n = n = d["arcs"]
        self.target = np.array(d["target"], float)
        self.reference = np.array(d["reference"], float)
        self.separation = np.array(d["separation"], float)
        self.lower = np.array(d["lower"], float)
        self.upper = np.array(d["upper"], float)
        self.balance = rows_matrix(d["balances"], n)
        self.rings = d["rings"]
        self.m = len(self.rings)
        self.weight = np.array([r["weight"] for r in self.rings], float)
        self.at = [r["at"] for r in self.rings]
        self.real_shift = [r["real_shift"] for r in self.rings]
        self.shift = rows_matrix([r["shift"] for r in self.rings], n)
        self.walk = [rows_matrix([w[0] for w in r["walk"]], n) for r in self.rings]
        self.walk_constant = [np.array([w[1] for w in r["walk"]], float) for r in self.rings]
        self.most = d["most_deviation"] * n
        # Quads to first order around the reference.
        gradient = np.zeros(n)
        quads = 0.0
        for first, second in d["sides"]:
            a = self.reference[first].sum()
            b = self.reference[second].sum()
            quads += a * b
            gradient[first] += b
            gradient[second] += a
        base = quads - gradient @ self.reference
        room = BUDGET_SHARE * d["quads_allowed"]
        self.quads_row = sparse.csr_matrix(gradient.reshape(1, n))
        self.quads_range = (d["quads"] - room - base, d["quads"] + room - base)
        self.lp_count = 0
        self.lp_seconds = 0.0
        self.memo = {}

    # The polyhedron of a level, for lengths x (and deviations u from C on).
    def system(self, level, chosen):
        n = self.n
        with_cap = level in ("C", "Q")
        size = 2 * n if with_cap else n
        pad = lambda block: sparse.hstack([block, sparse.csr_matrix((block.shape[0], size - n))]) if with_cap else block
        eq = [pad(self.balance)]
        if chosen:
            eq.append(pad(self.shift[chosen]))
        a_eq = sparse.vstack(eq).tocsr()
        b_eq = np.zeros(a_eq.shape[0])
        ub, b_ub = [], []
        if level in ("V", "W", "C", "Q"):
            for k in chosen:
                if self.walk[k].shape[0]:
                    # expression . x + constant >= 0
                    ub.append(pad(-self.walk[k]))
                    b_ub.append(self.walk_constant[k])
        if with_cap:
            eye = sparse.identity(n, format="csr")
            ub.append(sparse.hstack([eye, -eye]))
            b_ub.append(self.target)
            ub.append(sparse.hstack([-eye, -eye]))
            b_ub.append(-self.target)
            ub.append(sparse.hstack([sparse.csr_matrix((1, n)), sparse.csr_matrix(np.ones((1, n)))]))
            b_ub.append([self.most])
        if level == "Q":
            ub.append(pad(self.quads_row))
            b_ub.append([self.quads_range[1]])
            ub.append(pad(-self.quads_row))
            b_ub.append([-self.quads_range[0]])
        a_ub = sparse.vstack(ub).tocsr() if ub else None
        b_ub = np.concatenate([np.atleast_1d(np.asarray(b, float)) for b in b_ub]) if ub else None
        if level in ("S", "V"):
            bounds = [(s, None) for s in self.separation]
        else:
            bounds = list(zip(self.lower, self.upper))
        if with_cap:
            bounds += [(0, None)] * n
        return size, a_eq, b_eq, a_ub, b_ub, bounds

    def closable(self, level, chosen):
        key = (level, frozenset(chosen))
        if key in self.memo:
            return self.memo[key]
        chosen = sorted(chosen)
        size, a_eq, b_eq, a_ub, b_ub, bounds = self.system(level, chosen)
        clock = time.process_time()
        result = linprog(np.zeros(size), A_ub=a_ub, b_ub=b_ub, A_eq=a_eq, b_eq=b_eq, bounds=bounds, method="highs")
        self.lp_seconds += time.process_time() - clock
        self.lp_count += 1
        if result.status not in (0, 2):
            print(f"  {self.name} level {level}: LP status {result.status} ({result.message}) on {chosen}", file=sys.stderr)
        ok = result.status == 0
        self.memo[key] = ok
        return ok

    def core_within(self, level, chosen):
        """A minimal non-closable subset of `chosen` (deletion filter)."""
        core = list(chosen)
        for k in list(core):
            rest = [j for j in core if j != k]
            if not self.closable(level, rest):
                core = rest
        return sorted(core)

    def mip_model(self, level, whole, relax=False):
        """The big-M model of the HiGHS selector at a level (B to Q), with
        the objective the most weight of rings closed; x real or whole, z
        binary or (relaxed) in [0, 1]."""
        n, m = self.n, self.m
        size, a_eq, b_eq, a_ub, b_ub, bounds = self.system(level, [])
        lower = np.array([b[0] for b in bounds[:n]])
        upper = np.array([b[1] for b in bounds[:n]])
        extra = size - n
        total = size + m
        rows, low, high = [], [], []

        def widen(block, z_block=None):
            block = sparse.hstack([block, sparse.csr_matrix((block.shape[0], total - block.shape[1]))]).tocsr()
            if z_block is not None:
                block = block + z_block
            return block

        rows.append(widen(a_eq))
        low.append(b_eq)
        high.append(b_eq)
        if a_ub is not None:
            rows.append(widen(a_ub))
            low.append(np.full(a_ub.shape[0], -np.inf))
            high.append(b_ub)
        for k in range(m):
            s = self.shift[k].toarray().ravel()
            most_of = np.where(s > 0, s * upper, s * lower).sum()
            least_of = np.where(s > 0, s * lower, s * upper).sum()
            big = max(most_of, -least_of) + 1.0
            z = sparse.csr_matrix(([big], ([0], [size + k])), shape=(1, total))
            row = widen(self.shift[k])
            # s.x + big z <= big and -s.x + big z <= big
            rows.append(row + z)
            rows.append(-row + z)
            low += [[-np.inf], [-np.inf]]
            high += [[big], [big]]
            if level in ("W", "C", "Q") and self.walk[k].shape[0]:
                w = self.walk[k].toarray()
                least = np.where(w > 0, w * lower, w * upper).sum(axis=1)
                constant = self.walk_constant[k]
                slack = np.maximum(-(least + constant), 0.0)
                keep = slack > 0
                if keep.any():
                    block = sparse.csr_matrix(w[keep])
                    zb = sparse.csr_matrix(
                        (-slack[keep], (np.arange(keep.sum()), np.full(keep.sum(), size + k))), shape=(keep.sum(), total)
                    )
                    rows.append(widen(block, zb))
                    low.append(-constant[keep] - slack[keep])
                    high.append(np.full(keep.sum(), np.inf))
        a = sparse.vstack(rows).tocsr()
        constraint = LinearConstraint(a, np.concatenate([np.atleast_1d(x) for x in low]), np.concatenate([np.atleast_1d(x) for x in high]))
        lb = np.concatenate([lower, np.zeros(extra), np.zeros(m)])
        ub = np.concatenate([upper, np.full(extra, np.inf), np.ones(m)])
        integrality = np.concatenate([np.full(n, 1 if whole else 0), np.zeros(extra), np.full(m, 0 if relax else 1)])
        c = np.concatenate([np.zeros(size), -self.weight])
        return c, constraint, Bounds(lb, ub), integrality, size


def best_set(m, weight, cores, forced, time_limit=60):
    """Most weight of rings, none forced, no core whole (a small MILP)."""
    allowed = [k for k in range(m) if k not in forced]
    if not cores:
        return allowed, 0.0
    rows = []
    for core in cores:
        row = np.zeros(m)
        row[list(core)] = 1
        rows.append(row)
    a = np.array(rows)
    ub = np.array([len(c) - 1 for c in cores], float)
    lb = np.zeros(m)
    hi = np.ones(m)
    for k in forced:
        hi[k] = 0
    clock = time.process_time()
    result = milp(-weight, constraints=LinearConstraint(a, -np.inf, ub), bounds=Bounds(lb, hi), integrality=np.ones(m), options={"time_limit": time_limit})
    seconds = time.process_time() - clock
    if result.x is None:
        return None, seconds
    return [k for k in range(m) if result.x[k] > 0.5], seconds


def lp_value(m, weight, rows, forced):
    """max weight.z over 0 <= z <= 1, z = 0 on forced, rows: (set, bound)."""
    hi = np.ones(m)
    for k in forced:
        hi[k] = 0
    if rows:
        a = np.zeros((len(rows), m))
        for r, (members, _) in enumerate(rows):
            a[r, list(members)] = 1
        b = np.array([bound for _, bound in rows], float)
    else:
        a, b = None, None
    clock = time.process_time()
    result = linprog(-weight, A_ub=a, b_ub=b, bounds=list(zip(np.zeros(m), hi)), method="highs")
    seconds = time.process_time() - clock
    z = result.x
    fractional = [k for k in range(m) if INTEGRAL < z[k] < 1 - INTEGRAL]
    return -result.fun, fractional, seconds


def analyse_level(inst, level, out, verbose=True):
    m = inst.m
    clock = time.process_time()
    count0 = inst.lp_count
    record = {"level": level}
    forced = [k for k in range(m) if not inst.closable(level, [k])]
    record["forced"] = forced
    free = [k for k in range(m) if k not in forced]
    all_together = inst.closable(level, free)
    record["all_free_closable"] = all_together
    conflicts = []
    if not all_together:
        # Pairs known compatible: within a closable set found greedily.
        covered = set()
        order = sorted(free, key=lambda k: -inst.weight[k])
        for seed in order:
            chosen = [seed]
            for k in order:
                if k != seed and inst.closable(level, chosen + [k]):
                    chosen.append(k)
            for a, b in itertools.combinations(sorted(chosen), 2):
                covered.add((a, b))
            if len(covered) == len(free) * (len(free) - 1) // 2:
                break
            # A few seeds suffice to cover most pairs; the rest are tested.
            if seed == order[min(4, len(order) - 1)]:
                break
        for a, b in itertools.combinations(free, 2):
            if (a, b) not in covered and not inst.closable(level, [a, b]):
                conflicts.append((a, b))
    record["pairs"] = conflicts
    record["pair_seconds"] = time.process_time() - clock
    record["pair_lps"] = inst.lp_count - count0

    graph = {k: set() for k in free}
    for a, b in conflicts:
        graph[a].add(b)
        graph[b].add(a)
    touched = sorted({k for e in conflicts for k in e})
    record["graph_vertices"] = len(free)
    record["graph_edges"] = len(conflicts)
    record["conflict_vertices"] = len(touched)
    comps = [c for c in graphs.components(graph) if len(c) > 1]
    record["components"] = [len(c) for c in comps]
    record["max_degree"] = max((len(graph[k]) for k in free), default=0)
    if conflicts:
        cls = graphs.classes(graphs.induced(graph, touched))
    else:
        cls = {k: True for k in ("forest", "bipartite", "chordal", "comparability", "cocomparability", "interval", "perfect")}
        cls.update(odd_hole=None, odd_antihole=None)
    record["classes"] = cls

    # Exact optimum by implicit hitting sets, pairs first.
    clock = time.process_time()
    count0 = inst.lp_count
    cores = [tuple([a, b]) for a, b in conflicts]
    higher = []
    iterations = 0
    first_closable = None
    best = None
    master_seconds = 0.0
    while True:
        iterations += 1
        chosen, seconds = best_set(m, inst.weight, cores, set(forced))
        master_seconds += seconds
        if chosen is None:
            break
        ok = inst.closable(level, chosen)
        if first_closable is None:
            first_closable = ok
        if ok:
            best = chosen
            break
        core = inst.core_within(level, chosen)
        higher.append(core)
        cores.append(tuple(core))
        if iterations > 300 or time.process_time() - clock > 1800:
            break
    record["pairwise_optimum_closable"] = first_closable
    record["higher_cores"] = higher
    record["hitting_iterations"] = iterations
    record["hitting_seconds"] = time.process_time() - clock
    record["hitting_master_seconds"] = master_seconds
    record["hitting_lps"] = inst.lp_count - count0
    record["exact"] = best is not None
    record["optimum"] = float(inst.weight[best].sum()) if best is not None else None
    record["optimum_count"] = len(best) if best is not None else None
    record["best"] = best

    # LP relaxations of the conflict structure.
    edge_rows = [((a, b), 1) for a, b in conflicts]
    cliques = graphs.maximal_cliques(graphs.induced(graph, touched)) if conflicts else []
    clique_rows = [(c, 1) for c in cliques if len(c) > 1]
    core_rows = [(c, len(c) - 1) for c in higher]
    for name, rows in (("edge", edge_rows), ("clique", clique_rows), ("clique_cores", clique_rows + core_rows)):
        for weighting, w in (("weight", inst.weight), ("unit", np.ones(m))):
            value, fractional, seconds = lp_value(m, w, rows, forced)
            record[f"lp_{name}_{weighting}"] = {"value": value, "fractional": fractional, "seconds": seconds}
    # The unit-weight integer optimum, for the gaps of the unit relaxations.
    if record["exact"]:
        unit_cores = [tuple(c) for c in cores]
        chosen, _ = best_set(m, np.ones(m), unit_cores, set(forced))
        if chosen is not None and inst.closable(level, chosen):
            record["optimum_unit"] = len(chosen)
        else:
            record["optimum_unit"] = None
    out.append(record)
    if verbose:
        print(
            f"  {level}: forced {len(forced)}, free {len(free)}, all closable {all_together}, "
            f"pairs {len(conflicts)} on {len(touched)} rings, components {record['components']}, "
            f"higher cores {[len(c) for c in higher]}, optimum {record['optimum']} ({record['optimum_count']} rings), "
            f"edge LP {record['lp_edge_weight']['value']:.1f} frac {len(record['lp_edge_weight']['fractional'])}, "
            f"clique LP {record['lp_clique_weight']['value']:.1f} frac {len(record['lp_clique_weight']['fractional'])}, "
            f"classes " + ",".join(k for k in ("bipartite", "chordal", "interval", "comparability", "perfect") if cls[k]),
            flush=True,
        )
    return record


def solve_mip(inst, level, whole, relax, time_limit):
    c, constraint, bounds, integrality, size = inst.mip_model(level, whole, relax)
    clock = time.process_time()
    wall = time.time()
    if relax and not whole:
        result = milp(c, constraints=constraint, bounds=bounds, integrality=np.zeros_like(integrality))
    else:
        result = milp(c, constraints=constraint, bounds=bounds, integrality=integrality, options={"time_limit": time_limit})
    seconds = time.process_time() - clock
    out = {"status": int(result.status), "message": str(result.message), "cpu_seconds": seconds, "wall_seconds": time.time() - wall}
    if result.x is not None:
        out["value"] = float(-result.fun)
        z = result.x[size:]
        out["z_fractional"] = int(((z > INTEGRAL) & (z < 1 - INTEGRAL)).sum())
        if not relax:
            # Closed for real: a closed ring needs a zero shift and its walk.
            x = result.x[: inst.n]
            closed = []
            for k in range(inst.m):
                if abs(inst.shift[k] @ x) < 1e-6 and (inst.walk[k].shape[0] == 0 or (inst.walk[k] @ x + inst.walk_constant[k] >= -1e-6).all() or level in ("S", "B")):
                    closed.append(k)
            out["closed_check"] = float(inst.weight[closed].sum())
    bound = getattr(result, "mip_dual_bound", None)
    if bound is not None:
        out["dual_bound"] = float(-bound)
    return out


def main():
    target = sys.argv[1]
    results = []
    if os.path.exists(target):
        with open(target) as f:
            results = json.load(f)
    done = {r["instance"] for r in results}
    for path in sys.argv[2:]:
        inst = Instance(path)
        if inst.name in done:
            continue
        print(f"{inst.name}: {inst.n} arcs, {inst.balance.shape[0]} balances, {inst.m} rings", flush=True)
        clock = time.process_time()
        record = {"instance": inst.name, "arcs": inst.n, "balances": inst.balance.shape[0], "rings": inst.m}
        record["walk_conditions"] = int(sum(w.shape[0] for w in inst.walk))
        record["weights"] = inst.weight.tolist()
        record["at"] = inst.at
        record["real_shift"] = inst.real_shift
        record["reference_closed"] = [k for k in range(inst.m) if abs(inst.shift[k] @ inst.reference) < 0.5]
        levels = []
        for level in LEVELS:
            if inst.m == 0:
                break
            analyse_level(inst, level, levels)
        record["levels"] = levels
        # The solver's model itself, at the full level, real and whole, and
        # its LP relaxation.
        if inst.m:
            mips = {}
            for name, whole, relax in (("lp_relaxation", False, True), ("mip_real", False, False), ("mip_whole", True, False)):
                mips[name] = solve_mip(inst, "Q", whole, relax, time_limit=120)
                print(f"  Q {name}: {mips[name]}", flush=True)
            record["mip"] = mips
        record["lp_count"] = inst.lp_count
        record["lp_seconds"] = inst.lp_seconds
        record["cpu_seconds"] = time.process_time() - clock
        results.append(record)
        with open(target, "w") as f:
            json.dump(results, f, indent=1)


if __name__ == "__main__":
    main()
