#!/usr/bin/env python3
"""The turns every quantization in P leaves shifted, τ_P(H) (Section "Turns
every quantization within bounds leaves shifted", Table `tab:forced`; the
paragraph "The heuristic leaves most of the gap" of the closing-loops
paper). For each family H, the held turns of a way or the strips of its
whole lengths leaning less than 0.12 (each counted once up to sign), and
three sets P, each inside the one before:

1. `bounds`: balanced real lengths ℓ ≥ b (P_eng). Exact, by implicit hitting
   sets (Parker and Ryan): cores of H that cannot be closed together, each
   found by the duals of an elastic program and a deletion filter; a least
   set X meeting them (an integer program); done when H minus X closes.
2. `cap`: also the engine's cap on the mean deviation from the target,
   mean |ℓ - t| ≤ D, in real lengths. The same loop, stopped after
   --budget seconds: then the least hitting set of the cores found is a
   lower bound and the best closable complement found an upper bound.
3. `size`: also whole lengths and the engine's window of sizes, |Q(ℓ) -
   wanted| ≤ allowed, Q the number of quads. Lower bound: the one of `cap`
   and, with --whole-lower, the implicit hitting sets in whole lengths
   within the cap (no-good cuts on the sets HiGHS proves cannot close,
   stopped at the first one it cannot decide within --seconds). Upper bound:
   whole lengths closing the complement of the set the `cap` step left
   shifted, with the window to first order round a reference; if they leave
   more shifted than the lower bound, a program over whole lengths and a
   binary per turn, the fewest left shifted. Every solution is checked
   exactly (balances, bounds, cap, exact number of quads), and the window
   linearized again at it if its count falls outside (three rounds at most).

A single number is exact (lower = upper). With --sound every arc the
engine's lengths keep open is held at length one or more too (optima that
bring other arcs to zero can break the mesh). The cap, wanted and allowed
sizes are read from the export's call file (`most_deviation`, `wanted`,
`allowed`) when it has one, else (the drafts' old export, which has none)
those of its engine (0.474, 10,590, 211): --cap, --wanted, --allowed set them.

    python3 docs/research/turns-of-a-t-mesh/experiments/forced.py [<export>] \\
        [--steps bounds,cap,size] [--strips-ways 2] [--seconds 900] [--budget 1800] [--whole-lower] [--jobs 4]

Seconds a family for `bounds` (45 at most on the drafts' old export); up to
--budget for `cap` and several times --seconds for `size`; --jobs runs that
many families at a time, in processes of their own. Writes
results/forced.json, and the whole lengths found under --out.
"""

import argparse
import json
import time

import tmesh  # first: it sets the threads of the linear algebra

import numpy as np
from scipy.optimize import Bounds, LinearConstraint
from scipy.sparse import coo_matrix, csr_matrix, diags, hstack, identity, vstack

# The engine of the drafts' old export (male6, written before the export had
# call files, so without these numbers): the plain search's deviation 0.316
# times 1.5, and its window of 2 % round the 10,590 quads asked for. Only a
# fallback: an export of today's engine carries its own.
OLD_EXPORT = {"cap": 0.474, "wanted": 10590, "allowed": 211}


def say(text):
    print(text, flush=True)


class Family:
    """The turns of one family, each once up to sign, and the set P."""

    def __init__(self, way, name, forms, cap, wanted, allowed, sound=False):
        self.way, self.name = way, name
        seen, unique = set(), []
        for shift in forms:
            k = tmesh.key(shift)
            if k not in seen:
                seen.add(k)
                unique.append(shift)
        self.forms = unique
        self.m = len(unique)
        self.n = way.n
        self.S = tmesh.matrix(unique, way.n)
        self.lower = way.lower.copy()
        if sound:
            self.lower = np.maximum(self.lower, np.minimum(1, way.lengths).astype(float))
        self.cap, self.wanted, self.allowed = cap, wanted, allowed

    def engine_shifted(self):
        values = self.S @ self.way.lengths
        return [k for k in range(self.m) if round(float(values[k])) != 0]

    # --- the constraints of P ---------------------------------------------------------------

    def blocks(self, Y=(), cap=True, window=None, binaries=False, ranges=None):
        """Constraints over [x (n), e (n, with the cap), z (m, with binaries)]."""
        way, n, m = self.way, self.n, self.m
        ne = n if cap else 0
        nz = m if binaries else 0
        width = n + ne + nz

        def row(*parts):
            return hstack(list(parts)).tocsr()

        def zeros(rows, columns):
            return coo_matrix((rows, columns))

        constraints = [LinearConstraint(row(way.B, zeros(way.B.shape[0], ne + nz)), 0, 0)]
        if len(Y):
            S = self.S[list(Y)]
            constraints.append(LinearConstraint(row(S, zeros(S.shape[0], ne + nz)), 0, 0))
        if cap:
            eye = identity(n)
            t = way.target
            constraints += [
                LinearConstraint(row(eye, -eye, zeros(n, nz)), -np.inf, t),
                LinearConstraint(row(-eye, -eye, zeros(n, nz)), -np.inf, -t),
                LinearConstraint(csr_matrix(np.concatenate([np.zeros(n), np.ones(n), np.zeros(nz)])[None, :]),
                                 -np.inf, self.cap * n),
            ]
        if window is not None:
            # Q(x0) + g(x0) . (x - x0) within the window; g(x0) . x0 = 2 Q(x0).
            x0, low, high = window
            g = way.quads_gradient(x0)
            q = way.quads(x0)
            constraints.append(LinearConstraint(csr_matrix(np.concatenate([g, np.zeros(ne + nz)])[None, :]),
                                                low + q, high + q))
        if binaries:
            # z_k = 0 forces the shift of turn k to zero, within its range over P.
            low, high = ranges
            constraints += [
                LinearConstraint(row(self.S, zeros(m, ne), -diags(np.maximum(high, 0) + 1e-6)), -np.inf, 0),
                LinearConstraint(row(self.S, zeros(m, ne), -diags(np.minimum(low, 0) - 1e-6)), 0, np.inf),
            ]
        upper = way.target + self.cap * n + 1 if cap else np.full(n, np.inf)
        lb = np.concatenate([self.lower, np.zeros(ne + nz)])
        ub = np.concatenate([upper, np.full(ne, np.inf), np.ones(nz)])
        return constraints, lb, ub, width

    def lp(self, cost, Y=()):
        constraints, lb, ub, width = self.blocks(Y)
        c = np.concatenate([cost, np.zeros(width - self.n)])
        result, _ = tmesh.run_milp(c, constraints, Bounds(lb, ub), np.zeros(width))
        return None if result.x is None else result.fun

    def ranges(self):
        """The least and largest value of each shift over P with the cap."""
        low, high = np.zeros(self.m), np.zeros(self.m)
        for k in range(self.m):
            s = np.asarray(self.S[k].todense()).ravel()
            low[k] = self.lp(s)
            high[k] = -self.lp(-s)
        return low, high

    def window(self, reference, margin):
        return (reference, self.wanted - self.allowed + margin, self.wanted + self.allowed - margin)

    def least_shifted(self, ranges, integer, window=None, seconds=None):
        """The program over x and a binary per turn: the fewest turns left
        shifted. Sign-definite turns (a range not containing 0) are fixed."""
        constraints, lb, ub, width = self.blocks(window=window, binaries=True, ranges=ranges)
        low, high = ranges
        first = 2 * self.n
        for k in range(self.m):
            if low[k] > 1e-9 or high[k] < -1e-9:
                lb[first + k] = 1
        cost = np.concatenate([np.zeros(first), np.ones(self.m)])
        integrality = np.concatenate([np.full(self.n, 1 if integer else 0), np.zeros(self.n), np.ones(self.m)])
        result, took = tmesh.run_milp(cost, constraints, Bounds(lb, ub), integrality, seconds, mip_rel_gap=0)
        return result, took

    def whole(self, Y, cap=True, window=None, seconds=None):
        """Whole lengths in P closing Y, the least deviation as a guide,
        stopped at the first solution."""
        constraints, lb, ub, width = self.blocks(Y, cap=cap, window=window)
        if cap:
            cost = np.concatenate([np.zeros(self.n), np.ones(self.n)])
            more = {"mip_rel_gap": 1.0}
        else:
            cost, more = np.zeros(width), {}
        integrality = np.concatenate([np.ones(self.n), np.zeros(width - self.n)])
        result, took = tmesh.run_milp(cost, constraints, Bounds(lb, ub), integrality, seconds, **more)
        x = None if result.x is None else np.round(result.x[:self.n]).astype(np.int64)
        return x, result, took

    def check(self, x):
        """Exact checks of whole lengths: balances, bounds, cap, window."""
        way = self.way
        x = np.asarray(x, dtype=np.int64)
        deviation = way.deviation(x)
        quads = way.quads(x)
        out = {"balanced": all(sum(c * int(x[a]) for a, c in row.items()) == 0 for row in way.rows),
               "bounds": bool((x >= self.lower).all()),
               "deviation": deviation, "cap": deviation <= self.cap + 1e-12,
               "quads": quads, "window": abs(quads - self.wanted) <= self.allowed,
               "shifted": [k for k in range(self.m) if sum(c * int(x[a]) for a, c in self.forms[k].items()) != 0]}
        out["admissible"] = out["balanced"] and out["bounds"] and out["cap"] and out["window"]
        return out

    # --- implicit hitting sets ------------------------------------------------------------

    def elastic(self, Y, cap):
        """min Σ |shift_k(x)| over k in Y, x in P: (value, duals of the shift rows)."""
        way, n, k = self.way, self.n, len(Y)
        S = self.S[list(Y)] if k else csr_matrix((0, n))
        ne = n if cap else 0
        A_eq = vstack([hstack([way.B, coo_matrix((way.B.shape[0], ne + 2 * k))]),
                       hstack([S, coo_matrix((k, ne)), -identity(k), identity(k)])]).tocsr()
        cost = np.concatenate([np.zeros(n + ne), np.ones(2 * k)])
        bounds = list(zip(self.lower, [None] * n)) + [(0, None)] * (ne + 2 * k)
        arguments = {}
        if cap:
            eye = identity(n)
            A_ub = vstack([hstack([eye, -eye]), hstack([-eye, -eye]),
                           csr_matrix(np.concatenate([np.zeros(n), np.ones(n)])[None, :])]).tocsr()
            arguments = {"A_ub": hstack([A_ub, coo_matrix((A_ub.shape[0], 2 * k))]).tocsr(),
                         "b_ub": np.concatenate([way.target, -way.target, [self.cap * n]])}
        r = tmesh.run_linprog(cost, A_eq=A_eq, b_eq=np.zeros(A_eq.shape[0]), bounds=bounds, **arguments)
        if r.status != 0:
            raise RuntimeError(f"elastic program: {r.message}")
        return r.fun, r.eqlin.marginals[way.B.shape[0]:]

    def closable(self, Y, cap):
        return self.elastic(Y, cap)[0] <= 1e-7

    def core(self, Y, cap):
        _, duals = self.elastic(Y, cap)
        core = [Y[i] for i in range(len(Y)) if abs(duals[i]) > 1e-9]
        for k in list(core):
            trial = [j for j in core if j != k]
            if not self.closable(trial, cap):
                core = trial
        return core

    def hitting_sets(self, cap, budget=None, log=say):
        """τ by implicit hitting sets: (lower, upper, a least set X, cores,
        number of disjoint cores)."""
        clock = time.monotonic()
        m = self.m
        cores, pool = [], list(range(m))
        while not self.closable(pool, cap):
            c = self.core(pool, cap)
            cores.append(c)
            pool = [k for k in pool if k not in c]
        disjoint = len(cores)
        upper, best = m, list(range(m))
        while True:
            while True:
                X = greedy(m, cores)
                rest = [k for k in range(m) if k not in X]
                if self.closable(rest, cap):
                    break
                cores.append(self.core(rest, cap))
            if len(X) < upper:
                upper, best = len(X), X
            X = least_hitting_set(m, cores)
            rest = [k for k in range(m) if k not in X]
            log(f"   {len(cores)} cores: least hitting set {len(X)}, closable found {upper} "
                f"({time.monotonic() - clock:.0f} s)")
            if self.closable(rest, cap):
                return len(X), len(X), X, cores, disjoint
            cores.append(self.core(rest, cap))
            if budget is not None and time.monotonic() - clock > budget:
                return len(X), upper, best, cores, disjoint


def least_hitting_set(m, cores, seconds=600):
    if not cores:
        return []
    rows = np.zeros((len(cores), m))
    for i, c in enumerate(cores):
        rows[i, c] = 1
    result, _ = tmesh.run_milp(np.ones(m), [LinearConstraint(rows, 1, np.inf)], Bounds(0, 1), np.ones(m), seconds,
                               mip_rel_gap=0)
    if result.x is None:
        raise RuntimeError(f"hitting set: {result.message}")
    return sorted(int(k) for k in np.flatnonzero(result.x > 0.5))


def greedy(m, cores):
    X, left = set(), list(cores)
    while left:
        count = np.zeros(m)
        for c in left:
            count[c] += 1
        k = int(np.argmax(count))
        X.add(k)
        left = [c for c in left if k not in c]
    return sorted(X)


def whole_hitting_sets(family, cores, seconds, budget, log=say):
    """τ over whole lengths within the cap (no window), by hitting sets of
    the bounds' cores (valid for every smaller P) and of no-goods: the
    complement of a least hitting set X is closed in whole numbers if HiGHS
    can; if it proves it cannot, "drop one more of H minus X" is added. The
    lower bound is certified up to the first set HiGHS cannot decide."""
    clock = time.monotonic()
    rows = [list(c) for c in cores]
    certified, undecided, history = 0, False, []
    while time.monotonic() - clock < budget:
        X = least_hitting_set(family.m, rows)
        if not undecided:
            certified = len(X)
        Y = [k for k in range(family.m) if k not in X]
        x, result, took = family.whole(Y, seconds=seconds)
        status = "closed" if x is not None else ("cannot close" if result.status == 2 else "undecided")
        history.append({"size": len(X), "status": status, "seconds": round(took, 1)})
        log(f"   whole, within the cap: |X| = {len(X)}: {status} ({time.monotonic() - clock:.0f} s)")
        if x is not None:
            return certified, X, x, history
        if status == "undecided":
            undecided = True
        rows.append(Y)
    return certified, None, None, history


def run_family(family, steps, arguments, log=say):
    out = {"family": family.name, "turns": family.m, "engine_left_shifted": len(family.engine_shifted())}
    log(f"== way {family.way.number}, {family.name}: {family.m} turns; the engine leaves "
        f"{out['engine_left_shifted']} shifted")
    clock = time.monotonic()
    lower, upper = 0, family.m
    if "bounds" in steps or "size" in steps:
        low, high, X, cores, disjoint = family.hitting_sets(cap=False, log=log)
        bounds_cores = cores
        out["bounds"] = {"tau": low, "shifted": X, "cores": len(cores), "disjoint_cores": disjoint,
                         "core_sizes": sorted(len(c) for c in cores), "seconds": round(time.monotonic() - clock, 1)}
        log(f"   bounds: τ = {low} ({len(cores)} cores, {disjoint} disjoint, {out['bounds']['seconds']} s)")
        lower = low
    if "cap" in steps or "size" in steps:
        tick = time.monotonic()
        low, high, X, cores, disjoint = family.hitting_sets(cap=True, budget=arguments.budget, log=log)
        out["cap"] = {"lower": max(low, lower), "upper": high, "shifted": X, "cores": len(cores),
                      "disjoint_cores": disjoint, "seconds": round(time.monotonic() - tick, 1)}
        lower, upper = max(low, lower), high
        log(f"   bounds and cap, real lengths: {lower}–{upper} ({out['cap']['seconds']} s)")
    if "size" in steps:
        tick = time.monotonic()
        size = {"lower": lower}
        if arguments.whole_lower:
            certified, X, x, history = whole_hitting_sets(family, bounds_cores, arguments.seconds,
                                                          arguments.budget, log)
            size["whole_within_the_cap"] = {"certified_lower": certified, "history": history,
                                            "closed_leaving": None if X is None else len(X)}
            size["lower"] = max(size["lower"], certified)
        best, rounds = None, []

        def keep(x, kind, result, took):
            nonlocal best
            check = family.check(x)
            rounds.append({"program": kind, "found": True, "shifted": len(check["shifted"]),
                           "quads": check["quads"], "deviation": round(check["deviation"], 4),
                           "admissible": check["admissible"], "dual_bound": getattr(result, "mip_dual_bound", None),
                           "message": result.message, "seconds": round(took, 1)})
            log(f"   whole, window ({kind}): {len(check['shifted'])} shifted, {check['quads']} quads, deviation "
                f"{check['deviation']:.4f}, admissible {check['admissible']} ({took:.0f} s)")
            if check["admissible"] and (best is None or len(check["shifted"]) < len(best[1]["shifted"])):
                best = (x, check)
            return check["admissible"]

        # First the complement of the least set of the cap step, closed in
        # whole lengths within the window: exact when it leaves no more.
        Y = [k for k in range(family.m) if k not in out["cap"]["shifted"]]
        reference = family.way.target
        for _ in range(3):
            x, result, took = family.whole(Y, window=family.window(reference, arguments.margin),
                                           seconds=arguments.seconds)
            if x is None:
                rounds.append({"program": "complement", "found": False, "message": result.message,
                               "seconds": round(took, 1)})
                break
            if keep(x, "complement", result, took):
                break
            reference = x.astype(float)
        if best is None or len(best[1]["shifted"]) > size["lower"]:
            # Else the program over a binary per turn, the fewest left shifted.
            ranges = family.ranges()
            size["sign_definite_under_the_cap"] = [k for k in range(family.m)
                                                   if ranges[0][k] > 1e-9 or ranges[1][k] < -1e-9]
            reference = family.way.target if best is None else best[0].astype(float)
            for _ in range(3):
                window = family.window(reference, arguments.margin)
                result, took = family.least_shifted(ranges, integer=True, window=window, seconds=arguments.seconds)
                if result.x is None:
                    rounds.append({"program": "fewest", "found": False, "message": result.message,
                                   "seconds": round(took, 1)})
                    break
                x = np.round(result.x[:family.n]).astype(np.int64)
                if keep(x, "fewest", result, took):
                    break
                reference = x.astype(float)
        size["rounds"] = rounds
        if best is not None:
            size["upper"] = len(best[1]["shifted"])
            size["quads"], size["deviation"] = best[1]["quads"], best[1]["deviation"]
            name = f"way-{family.way.number}-{family.name}-size.txt"
            (arguments.out / name).write_text(tmesh.lengths_text(best[0], family.way.scale))
            size["lengths_file"] = name
        else:
            size["upper"] = family.m
        size["seconds"] = round(time.monotonic() - tick, 1)
        out["size"] = size
        log(f"   cap, size, whole: {size['lower']}–{size['upper']} ({size['seconds']} s)")
    out["seconds"] = round(time.monotonic() - clock, 1)
    return out


def family_job(job):
    """One family, in a process of its own when --jobs asks for several."""
    path, name, cap, wanted, allowed, steps, arguments = job
    way = tmesh.Way(path)
    forms = tmesh.held_forms(way) if name == "held" else [s["shift"] for s in way.strips(0.12)]
    family = Family(way, name, forms, cap, wanted, allowed, arguments.sound)
    return {"way": way.number, **run_family(family, steps, arguments)}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "forced")
    parser.add_argument("--steps", default="bounds,cap,size", help="bounds, cap, size (comma-separated)")
    parser.add_argument("--families", default="held,strips", help="held, strips (comma-separated)")
    parser.add_argument("--strips-ways", type=lambda text: [int(k) for k in text.split(",")], default=[2],
                        help="the ways whose strips are a family too (default 2, the way the engine kept)")
    parser.add_argument("--cap", type=float, help="mean deviation allowed (default: the export's, else 0.474)")
    parser.add_argument("--wanted", type=int, help="quads asked for (default: the export's, else 10,590)")
    parser.add_argument("--allowed", type=int, help="quads off allowed (default: the export's, else 211)")
    parser.add_argument("--seconds", type=float, default=900.0, help="time limit of each integer program")
    parser.add_argument("--budget", type=float, default=1800.0, help="time limit of each hitting-set loop with a cap")
    parser.add_argument("--margin", type=float, default=60.0, help="quads kept off each end of the window")
    parser.add_argument("--whole-lower", action="store_true", help="lower bound in whole lengths within the cap")
    parser.add_argument("--sound", action="store_true", help="keep open every arc the engine keeps open")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    call = folder.parent / f"{folder.name}.json"
    engine = json.loads(call.read_text()) if call.exists() and folder.name.startswith("call-") else {}
    cap = arguments.cap or engine.get("most_deviation") or OLD_EXPORT["cap"]
    wanted = arguments.wanted or engine.get("wanted") or OLD_EXPORT["wanted"]
    allowed = arguments.allowed or engine.get("allowed") or OLD_EXPORT["allowed"]
    steps = set(arguments.steps.split(","))
    families = arguments.families.split(",")
    say(f"P: cap {cap}, window {wanted} ± {allowed}{', sound' if arguments.sound else ''}; steps {sorted(steps)}")
    jobs = []
    for way in ways:
        if "held" in families:
            jobs.append((way.path, "held", cap, wanted, allowed, steps, arguments))
        if "strips" in families and way.number in arguments.strips_ways and way.helices is not None:
            jobs.append((way.path, "strips", cap, wanted, allowed, steps, arguments))
    rows = tmesh.parallel(family_job, jobs, arguments.jobs)
    summary = {
        "experiment": "turns every quantization in P leaves shifted (Table tab:forced)",
        **tmesh.record(arguments, folder, ways, started, clock, cap=cap, wanted=wanted, allowed=allowed,
                       sound=arguments.sound, steps=sorted(steps), seconds_per_program=arguments.seconds,
                       budget_per_loop=arguments.budget, margin=arguments.margin),
        "table": [{"way": row["way"], "family": row["family"], "turns": row["turns"],
                   "bounds": row.get("bounds", {}).get("tau"),
                   "bounds_cap": [row["cap"]["lower"], row["cap"]["upper"]] if "cap" in row else None,
                   "cap_size_whole": [row["size"]["lower"], row["size"]["upper"]] if "size" in row else None,
                   "engine_left_shifted": row["engine_left_shifted"]} for row in rows],
        "per_family": rows,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
