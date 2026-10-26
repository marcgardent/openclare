"""Finds the certificates that `Main.lean` checks for Theorem "Forced shifts" and Remark
"Sign-definite turns": for each way of closing an export, the least number `τ_P(H)` of
held turns that every balanced length `ℓ ≥ b` (the bounds of separation) leaves shifted,
with its witnesses, and, given the claims of a paper, the certificates of its table.

    python3 certify.py <way-n.json>... --out <file> [--claims <claims.json>] [--note <text>]

Needs SciPy (HiGHS); `Main.lean` trusts none of it. Every number written is exact and
checked here in exact arithmetic before it is written; `Main.lean` checks it again with
the library's definitions. For each way:

* `tau`: by implicit hitting sets (Parker and Ryan), cores of held turns that cannot be
  closed together, each with a Farkas certificate `(u, c)`: `c = Σ u_γ shift_γ + λᵀ B`,
  `c ≥ 0`, `c · b > 0` (`c` sparse; `λ` is left to `Main.lean`); the indices of pairwise
  disjoint cores; a least set `X` of held turns meeting every core, and whole lengths
  `ℓ ≥ b`, `B ℓ = 0` that close every held turn outside `X`.
* `sets`, with `--claims`: for each set of a table of certificates that holds this way, its
  turns, the multipliers `u` (those of the claims in the first way of the row; in the
  others the same up to the sign of each turn, the same `c` kept when possible), `c`,
  and, for each turn of the set, whole lengths in `P` that close the others
  (irreducibility), when they are found.

The bounds `b` used (the engine's, `max(lower, 0)`) are written out with the rest.

The output maps each way file, by its SHA-256, to these data; `convert.py` adds them to
the way it converts when the hash matches.
"""

import argparse
import hashlib
import json
import math
import os
import sys
import time
from fractions import Fraction

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, linprog, milp
from scipy.sparse import coo_matrix, csr_matrix, hstack, identity, vstack


def matrix(rows, n):
    data, r, c = [], [], []
    for i, row in enumerate(rows):
        for arc, coefficient in row:
            r.append(i)
            c.append(int(arc))
            data.append(float(coefficient))
    return coo_matrix((data, (r, c)), shape=(len(rows), n)).tocsr()


def form(row):
    out = {}
    for arc, c in row:
        out[int(arc)] = out.get(int(arc), 0) + int(round(c))
    return {a: c for a, c in out.items() if c}


def fraction(x):
    return [x.numerator, x.denominator]


def solve(equations, guess):
    """An exact solution of `equations`, each `({unknown: coefficient}, right side)` over
    the fractions, by elimination; the unknowns left free take the value of `guess`, as a
    fraction. None if there is none."""
    pivots, order = {}, []
    for form, rhs in equations:
        row = {v: Fraction(c) for v, c in form.items() if c}
        rhs = Fraction(rhs)
        while (hit := next((v for v in row if v in pivots), None)) is not None:
            f, (other, value) = row[hit], pivots[hit]
            for v, c in other.items():
                row[v] = row.get(v, 0) - f * c
                if not row[v]:
                    del row[v]
            rhs -= f * value
        if not row:
            if rhs:
                return None
            continue
        v = next(iter(row))
        p = row[v]
        pivots[v] = ({w: c / p for w, c in row.items()}, rhs / p)
        order.append(v)
    value = [Fraction(float(x)).limit_denominator(1000) for x in guess]
    for v in reversed(order):
        row, rhs = pivots[v]
        value[v] = rhs - sum((c * value[w] for w, c in row.items() if w != v), Fraction(0))
    return value


class Way:
    def __init__(self, path):
        with open(path, "rb") as f:
            raw = f.read()
        self.sha = hashlib.sha256(raw).hexdigest()
        s = json.loads(raw)
        self.n = n = len(s["arcs"])
        self.engine = [int(x) for x in s["lengths"]]
        self.b = [max(int(round(x)), 0) for x in s["lower"]]
        self.rows = [form(r) for r in s["stations"]]
        self.B = matrix([list(r.items()) for r in self.rows], n)
        turns = s["turns"]
        self.held = [k for k, t in enumerate(turns) if t["held"]]
        self.held_index = {t["held_index"]: k for k, t in enumerate(turns) if t["held"]}
        self.shift = [form(t["shift"]) for t in turns]
        # Columns of B: (row, coefficient) for each arc, for the exact solve of λᵀ B = d.
        self.cols = [[] for _ in range(n)]
        for r, row in enumerate(self.rows):
            for a, c in row.items():
                self.cols[a].append((r, c))

    def H(self, turns):
        return matrix([list(self.shift[k].items()) for k in turns], self.n)

    # --- exact arithmetic ------------------------------------------------------------------

    def combination(self, turns, u):
        d = [Fraction(0)] * self.n
        for k, w in zip(turns, u):
            for a, c in self.shift[k].items():
                d[a] += w * c
        return d

    def in_rowspace(self, d):
        """λ with λᵀ B = d exactly, or None: over each component of the graph of the
        balances, rows `c + k t` from a root, `t` fixed by an unbalanced cycle."""
        R = len(self.rows)
        at_row = [[] for _ in range(R)]
        for a, col in enumerate(self.cols):
            for r, _ in col:
                at_row[r].append(a)
        c, k, lam = [Fraction(0)] * R, [0] * R, [Fraction(0)] * R
        for root in range(R):
            if k[root]:
                continue
            k[root] = 1
            queue, i, t = [root], 0, None
            while i < len(queue):
                r = queue[i]
                i += 1
                for a in at_row[r]:
                    col = self.cols[a]
                    if len(col) != 2:
                        continue
                    (r0, s0), (r1, s1) = col
                    if k[r0] and not k[r1]:
                        c[r1] = s1 * (d[a] - s0 * c[r0])
                        k[r1] = -s1 * s0 * k[r0]
                        queue.append(r1)
                    elif k[r1] and not k[r0]:
                        c[r0] = s0 * (d[a] - s1 * c[r1])
                        k[r0] = -s0 * s1 * k[r1]
                        queue.append(r0)
                    elif t is None:
                        coefficient = s0 * k[r0] + s1 * k[r1]
                        if coefficient:
                            t = (d[a] - s0 * c[r0] - s1 * c[r1]) / coefficient
            for r in queue:
                lam[r] = c[r] + k[r] * (t or 0)
        for a, col in enumerate(self.cols):
            if sum((lam[r] * v for r, v in col), Fraction(0)) != d[a]:
                return None
        return lam

    def valid(self, turns, u, c):
        """`c - Σ u shift ∈ rows of B`, `c ≥ 0`, `c · b > 0`, exactly."""
        if any(x < 0 for x in c.values()):
            return False
        if sum((x * self.b[a] for a, x in c.items()), Fraction(0)) <= 0:
            return False
        combo = self.combination(turns, u)
        d = [-x for x in combo]
        for a, x in c.items():
            d[a] += x
        return self.in_rowspace(d) is not None

    def closes(self, turns, lengths):
        if any(x < b for x, b in zip(lengths, self.b)):
            return False
        if any(sum(c * lengths[a] for a, c in row.items()) for row in self.rows):
            return False
        return all(sum(c * lengths[a] for a, c in self.shift[k].items()) == 0 for k in turns)

    # --- linear programs -------------------------------------------------------------------

    def elastic(self, Y):
        """min Σ |shift_γ(ℓ)| over γ ∈ Y, ℓ ∈ P: (value, ℓ, duals of the shift rows)."""
        n, k = self.n, len(Y)
        S = self.H(Y) if k else csr_matrix((0, n))
        A = vstack([hstack([self.B, coo_matrix((self.B.shape[0], 2 * k))]),
                    hstack([S, -identity(k), identity(k)])]).tocsr()
        cost = np.concatenate([np.zeros(n), np.ones(2 * k)])
        bounds = list(zip(self.b, [None] * n)) + [(0, None)] * (2 * k)
        r = linprog(cost, A_eq=A, b_eq=np.zeros(A.shape[0]), bounds=bounds, method="highs")
        assert r.status == 0, r.message
        return r.fun, r.x[:n], r.eqlin.marginals[self.B.shape[0]:]

    def feasible(self, Y):
        return self.elastic(Y)[0] <= 1e-7

    def core(self, Y):
        _, _, d = self.elastic(Y)
        core = [Y[i] for i in range(len(Y)) if abs(d[i]) > 1e-9]
        assert not self.feasible(core)
        for k in list(core):
            trial = [j for j in core if j != k]
            if not self.feasible(trial):
                core = trial
        return core

    def certificate(self, turns, u=None):
        """Exact (u, c), u given or found (least Σ |u| with c · b = 1): c read as
        fractions, else λ solved again exactly (`solve`)."""
        n, m, R = self.n, len(turns), self.B.shape[0]
        St, Bt = self.H(turns).T.tocsr(), self.B.T.tocsr()
        if u is None:
            rows = hstack([St, -St, Bt]).tocsr()
            b = coo_matrix(np.asarray(self.b, float)[None, :])
            r = milp(np.concatenate([np.ones(2 * m), np.zeros(R)]),
                     constraints=[LinearConstraint(rows, 0, np.inf), LinearConstraint(b @ rows, 1, 1)],
                     bounds=Bounds(np.concatenate([np.zeros(2 * m), np.full(R, -np.inf)]), np.inf))
            if r.x is None:
                return None
            uf, lf = r.x[:m] - r.x[m:2 * m], r.x[2 * m:]
            candidates = [[Fraction(float(x)).limit_denominator(q) for x in uf] for q in (12, 60, 1000, 100000)]
        else:
            candidates = [u]
        b = np.asarray(self.b, float)
        Bb = Bt.T @ b
        for uq in candidates:
            combo = np.zeros(n)
            for a, x in enumerate(self.combination(turns, uq)):
                combo[a] = float(x)
            # c = combo + Bᵀ λ ≥ 0 with u fixed: the largest c · b, then the least Σ c
            # keeping it, for a sparse c.
            r = linprog(-Bb, A_ub=-Bt, b_ub=combo, bounds=[(None, None)] * R, method="highs")
            if r.status != 0 or combo @ b - r.fun <= 1e-9:
                continue
            best = combo @ b - r.fun
            r = linprog(np.asarray(Bt.sum(axis=0)).ravel(), A_ub=vstack([-Bt, csr_matrix(-Bb[None, :])]),
                        b_ub=np.concatenate([combo, [combo @ b - best * (1 - 1e-6)]]),
                        bounds=[(None, None)] * R, method="highs")
            if r.status != 0:
                continue
            cf = combo + Bt @ r.x
            for q in (12, 60, 1000, 100000):
                c = {a: Fraction(float(x)).limit_denominator(q) for a, x in enumerate(cf) if x > 1e-7}
                c = {a: x for a, x in c.items() if x != 0}
                if self.valid(turns, uq, c):
                    return uq, c
            # Else λ solved again exactly, c kept at zero where the program puts it.
            exact = self.combination(turns, uq)
            lam = solve([(dict(self.cols[a]), -exact[a]) for a in range(n) if cf[a] <= 1e-7], r.x)
            if lam is not None:
                for a in range(n):
                    exact[a] += sum((lam[s] * v for s, v in self.cols[a]), Fraction(0))
                c = {a: x for a, x in enumerate(exact) if x != 0}
                if self.valid(turns, uq, c):
                    return uq, c
        if u is not None:
            return None
        # Else u and λ solved again together, exactly: c kept at zero where the first
        # program puts it, and c · b = 1.
        cf = St @ uf + Bt @ lf

        def row(arcs):
            out = {}
            for a, weight in arcs:
                for i, k in enumerate(turns):
                    out[i] = out.get(i, 0) + weight * self.shift[k].get(a, 0)
                for s, v in self.cols[a]:
                    out[m + s] = out.get(m + s, 0) + weight * v
            return out
        equations = [(row([(a, 1)]), 0) for a in range(n) if cf[a] <= 1e-9]
        equations.append((row([(a, x) for a, x in enumerate(self.b) if x]), 1))
        x = solve(equations, np.concatenate([uf, lf]))
        if x is None:
            return None
        uq, c = x[:m], self.combination(turns, x[:m])
        for a in range(n):
            c[a] += sum((x[m + s] * v for s, v in self.cols[a]), Fraction(0))
        c = {a: v for a, v in enumerate(c) if v != 0}
        return (uq, c) if self.valid(turns, uq, c) else None

    def closing(self, turns, seconds=60):
        """Whole ℓ ∈ P closing `turns`, checked exactly: the nearest to the engine's
        lengths found within `seconds`, else any, else real lengths made whole by their
        common denominator (Remark "Whole solutions exist at some size"): the vertex of a
        linear program read as fractions, or solved again exactly."""
        n, k = self.n, len(turns)
        S = self.H(turns) if k else csr_matrix((0, n))
        A = vstack([hstack([self.B, coo_matrix((self.B.shape[0], 2 * n))]),
                    hstack([S, coo_matrix((k, 2 * n))]),
                    hstack([identity(n), -identity(n), identity(n)])]).tocsr()
        rhs = np.concatenate([np.zeros(self.B.shape[0] + k), np.asarray(self.engine, float)])
        cost = np.concatenate([np.zeros(n), np.ones(2 * n)])
        lower = np.concatenate([np.asarray(self.b, float), np.zeros(2 * n)])
        r = milp(cost, constraints=[LinearConstraint(A, rhs, rhs)], bounds=Bounds(lower, np.inf),
                 integrality=np.concatenate([np.ones(n), np.zeros(2 * n)]),
                 options={"time_limit": seconds})
        if r.x is not None:
            x = [int(round(v)) for v in r.x[:n]]
            if self.closes(turns, x):
                return x
        A = vstack([self.B, S]).tocsr()
        b = np.asarray(self.b, float)
        r = milp(np.ones(n), constraints=[LinearConstraint(A, 0, 0)], bounds=Bounds(b, np.inf),
                 integrality=np.ones(n), options={"time_limit": 2 * seconds})
        if r.x is not None:
            x = [int(round(v)) for v in r.x]
            if self.closes(turns, x):
                return x
        r = linprog(np.ones(n), A_eq=A, b_eq=np.zeros(A.shape[0]), bounds=list(zip(b, [None] * n)),
                    method="highs")
        if r.status != 0:
            return None
        candidates = [[Fraction(float(t)).limit_denominator(q) for t in r.x] for q in (12, 60, 1000, 10 ** 6)]
        for fx in candidates + [self.vertex(turns, r.x)]:
            if fx is None:
                continue
            den = math.lcm(*(t.denominator for t in fx))
            whole = [int(t * den) for t in fx]
            if self.closes(turns, whole):
                return whole
        return None

    def vertex(self, turns, x):
        """Exact ℓ with B ℓ = 0 and the shifts of `turns` zero, the arcs at their bound
        in `x` kept there, the others near `x` (`solve`), or None."""
        at_bound = [({a: 1}, self.b[a]) for a in range(self.n) if x[a] <= self.b[a] + 1e-9]
        return solve(at_bound + [(row, 0) for row in self.rows + [self.shift[k] for k in turns]], x)


def hitting_set(m, cores, seconds=600):
    if not cores:
        return []
    rows = np.zeros((len(cores), m))
    for i, c in enumerate(cores):
        rows[i, c] = 1
    r = milp(np.ones(m), constraints=[LinearConstraint(rows, 1, np.inf)], bounds=Bounds(0, 1),
             integrality=np.ones(m), options={"time_limit": seconds, "mip_rel_gap": 0})
    assert r.status == 0, r.message
    return sorted(int(k) for k in np.flatnonzero(r.x > 0.5))


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


def tau(W, log):
    """τ_P of the held turns by implicit hitting sets, on positions in `W.held`."""
    H = W.held
    m = len(H)
    cores, pool = [], list(range(m))
    while not W.feasible([H[k] for k in pool]):
        c = W.core([H[k] for k in pool])
        cores.append(sorted(H.index(t) for t in c))
        pool = [k for k in pool if H[k] not in c]
    disjoint = list(range(len(cores)))
    while True:
        while True:
            X = greedy(m, cores)
            rest = [H[k] for k in range(m) if k not in X]
            if W.feasible(rest):
                break
            cores.append(sorted(H.index(t) for t in W.core(rest)))
        X = hitting_set(m, cores)
        rest = [H[k] for k in range(m) if k not in X]
        log(f"   {len(cores)} cores, least hitting set {len(X)}")
        if W.feasible(rest):
            return cores, disjoint, X
        cores.append(sorted(H.index(t) for t in W.core(rest)))


def certify(path, claims, log):
    W = Way(path)
    name = os.path.basename(path)
    out = {"sha256": W.sha, "bounds": [[a, b] for a, b in enumerate(W.b) if b]}
    t0 = time.time()
    cores, disjoint, X = tau(W, log)
    H = W.held
    certified = []
    for core in cores:
        turns = [H[k] for k in core]
        found = W.certificate(turns)
        if found is None:
            sys.exit(f"{name}: no exact certificate for the core {turns}")
        u, c = found
        certified.append({"turns": turns, "u": [fraction(x) for x in u],
                          "c": [[a, x.numerator, x.denominator] for a, x in sorted(c.items())]})
    shifted = [H[k] for k in X]
    lengths = W.closing([t for t in H if t not in shifted])
    if lengths is None:
        sys.exit(f"{name}: no whole lengths close the held turns outside {shifted}")
    out["tau"] = {"cores": certified, "disjoint": disjoint, "shifted": shifted, "closing": lengths}
    log(f"{name}: tau = {len(X)}, {len(cores)} cores, {len(disjoint)} disjoint ({time.time() - t0:.0f} s)")
    if claims is not None:
        way = name[len("way-"):-len(".json")]
        out["claims"] = claims["ways"].get(way)
        sets = []
        for row in claims["sets"]:
            if way not in row["ways"]:
                continue
            turns = [W.held_index[h] for h in row["ways"][way]]
            first = next(iter(row["ways"]))
            given = row["multipliers"] if isinstance(row["multipliers"], list) else row.get("script")
            paper = {"multipliers": [fraction(Fraction(x)) for x in given] if way == first else None,
                     "stated": row["multipliers"] if isinstance(row["multipliers"], str) else None,
                     "bounded": row["bounded"], "exact": row["exact"], "first": first}
            found = None
            if way == first:
                found = W.certificate(turns, [Fraction(x) for x in given])
            else:
                # The same turns up to the sign of each: the same c when possible.
                base = [Fraction(x) for x in given]
                for signs in range(2 ** len(turns)):
                    u = [x if not (signs >> i) & 1 else -x for i, x in enumerate(base)]
                    found = W.certificate(turns, u)
                    if found:
                        break
            if found is None:
                log(f"   row {row['row']}: no certificate with the multipliers of the claims")
                sets.append({"row": row["row"], "turns": turns, "u": None, "c": None, "irreducible": [],
                             "paper": paper})
                continue
            u, c = found
            irreducible = []
            for t in turns:
                lengths = W.closing([s for s in turns if s != t], seconds=30)
                irreducible.append({"without": t, "closing": lengths})
            sets.append({"row": row["row"], "turns": turns, "u": [fraction(x) for x in u],
                         "c": [[a, x.numerator, x.denominator] for a, x in sorted(c.items())],
                         "irreducible": irreducible, "paper": paper})
            log(f"   row {row['row']}: turns {turns}, u {[str(x) for x in u]}, "
                f"bounded {[a for a in sorted(c) if W.b[a] > 0]}, "
                f"{sum(i['closing'] is not None for i in irreducible)} of {len(turns)} left out closed")
        out["sets"] = sets
    return name, out


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("ways", nargs="+")
    parser.add_argument("--out", required=True)
    parser.add_argument("--claims")
    parser.add_argument("--note", default="")
    args = parser.parse_args()
    claims = None
    if args.claims:
        with open(args.claims) as f:
            claims = json.load(f)
    result = {"note": args.note, "ways": {}}
    for path in args.ways:
        name, out = certify(path, claims, lambda s: print(s, flush=True))
        if claims is not None and claims["export"].get(name) != out["sha256"]:
            sys.exit(f"{path} is not the export of the claims")
        result["ways"][out["sha256"]] = dict(out, way=name)
    with open(args.out, "w") as f:
        json.dump(result, f, separators=(",", ":"))


if __name__ == "__main__":
    main()
