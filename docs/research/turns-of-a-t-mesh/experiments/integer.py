"""Whole lengths as a lattice: an exact integer basis of the whole solutions of
linear equations by elimination, and its LLL reduction with Babai's nearest
plane. Used by `lattice.py` (Section "Whole lengths and size" of the turns
paper) and by `../../closing-loops/experiments/exact/lattice_rounding.py`
(the rounding of the rings, Section "An exact look at the closing system").
"""

import time
from math import gcd

import numpy as np


class Lattice:
    """The whole solutions of equations imposed one at a time: every arc
    length is an integer combination of free parameters, x = M y, and each
    equation eliminates one parameter. A parameter with coefficient ±1 is
    solved for directly; otherwise unimodular changes of parameters (Euclid's
    algorithm on the coefficients) bring a unit coefficient first. Every step
    maps the whole solutions one to one, so at the end they are M Z^d."""

    def __init__(self, n):
        self.expression = [{i: 1} for i in range(n)]
        self.users = {i: {i} for i in range(n)}
        self.euclid_steps = 0
        self.non_unit_equations = 0
        # The greatest common divisor of each equation as it was met, where
        # it was not 1: an equation whose coefficients, once the ones before
        # hold, are all even is a condition of parity.
        self.divisors = []

    def evaluate(self, row):
        out = {}
        for arc, c in row:
            for j, v in self.expression[arc].items():
                w = out.get(j, 0) + c * v
                if w:
                    out[j] = w
                else:
                    out.pop(j, None)
        return out

    def substitute(self, j, replacement):
        """y_j := Σ replacement[k] y_k (j not in the replacement)."""
        for arc in list(self.users[j]):
            expression = self.expression[arc]
            c = expression.pop(j)
            for k, v in replacement.items():
                w = expression.get(k, 0) + c * v
                if w:
                    if k not in expression:
                        self.users[k].add(arc)
                    expression[k] = w
                elif k in expression:
                    del expression[k]
                    self.users[k].discard(arc)
        del self.users[j]

    def impose(self, row):
        """Adds Σ c x = 0 (row: [(arc, c)]); False if the ones before imply it."""
        e = self.evaluate([(int(a), int(round(c))) for a, c in row])
        if not e:
            return False
        g = 0
        for v in e.values():
            g = gcd(g, v)
        if g != 1:
            self.divisors.append(g)
        e = {k: v // g for k, v in e.items()}
        if not any(abs(v) == 1 for v in e.values()):
            self.non_unit_equations += 1
        while not any(abs(v) == 1 for v in e.values()):
            # The two smallest coefficients a, b: y_j = y_j' - q y_k turns the
            # coefficient b of y_k into b - q a.
            (j, a), (k, b) = sorted(e.items(), key=lambda t: abs(t[1]))[:2]
            q = b // a
            for arc in list(self.users[j]):
                expression = self.expression[arc]
                w = expression.get(k, 0) - q * expression[j]
                if w:
                    if k not in expression:
                        self.users[k].add(arc)
                    expression[k] = w
                elif k in expression:
                    del expression[k]
                    self.users[k].discard(arc)
            w = b - q * a
            if w:
                e[k] = w
            else:
                del e[k]
            self.euclid_steps += 1
        j = min((k for k, v in e.items() if abs(v) == 1), key=lambda k: len(self.users[k]))
        s = e.pop(j)
        self.substitute(j, {k: -s * v for k, v in e.items()})
        return True

    @property
    def dimension(self):
        return len(self.users)

    def largest_entry(self):
        return max((abs(v) for expression in self.expression for v in expression.values()), default=0)

    def largest_entry_digits(self):
        """The number of decimal digits of the largest entry (which can grow
        past what is worth printing)."""
        largest = self.largest_entry()
        return len(str(largest)) if largest < 10 ** 18 else int(largest.bit_length() * 0.30103) + 1

    def basis(self):
        """The basis vectors, as lists of [arc, value]."""
        parameters = sorted(self.users)
        index = {p: c for c, p in enumerate(parameters)}
        columns = [[] for _ in parameters]
        for arc, expression in enumerate(self.expression):
            for p, v in expression.items():
                columns[index[p]].append([arc, v])
        return columns


class Reducer:
    """LLL reduction (δ = 0.99) of the rows of an integer basis for the inner
    product Σ w_i u_i v_i, and Babai's nearest plane. Gram–Schmidt in floating
    point, recomputed for each row as the reduction reaches it."""

    def __init__(self, basis, weight):
        self.B = np.array(basis, dtype=np.int64)
        self.root = np.sqrt(np.asarray(weight, dtype=float))
        d, m = self.B.shape
        self.star = np.zeros((d, m))
        self.mu = np.zeros((d, d))
        self.norm = np.zeros(d)

    def orthogonalize(self, k):
        v = self.B[k] * self.root
        if k:
            c = (self.star[:k] @ v) / self.norm[:k]
            self.mu[k, :k] = c
            v = v - c @ self.star[:k]
            # Twice, for stability.
            again = (self.star[:k] @ v) / self.norm[:k]
            self.mu[k, :k] += again
            v = v - again @ self.star[:k]
        self.star[k] = v
        self.norm[k] = v @ v

    def size_reduce(self, k):
        q = np.zeros(k, dtype=np.int64)
        mu = self.mu
        for j in range(k - 1, -1, -1):
            r = int(np.rint(mu[k, j]))
            if r:
                q[j] = r
                mu[k, :j] -= r * mu[j, :j]
                mu[k, j] -= r
        if q.any():
            self.B[k] -= q @ self.B[:k]
            self.orthogonalize(k)

    def reduce(self, delta=0.99, log=None, every=60.0):
        d = len(self.B)
        self.orthogonalize(0)
        k = 1
        if d > 1:
            self.orthogonalize(1)
        swaps, last = 0, time.monotonic()
        while k < d:
            self.size_reduce(k)
            if self.norm[k] >= (delta - self.mu[k, k - 1] ** 2) * self.norm[k - 1]:
                k += 1
                if k < d:
                    self.orthogonalize(k)
            else:
                self.B[[k - 1, k]] = self.B[[k, k - 1]]
                swaps += 1
                self.orthogonalize(k - 1)
                self.orthogonalize(k)
                k = max(k - 1, 1)
            if log is not None and time.monotonic() - last > every:
                last = time.monotonic()
                log(f"   LLL: row {k} of {d}, {swaps} swaps, largest entry {np.abs(self.B).max()}")
        return swaps

    def nearest_plane(self, target):
        """Babai: integer coefficients c with c @ B near `target` (weighted)."""
        d = len(self.B)
        for k in range(d):
            self.orthogonalize(k)
        v = np.asarray(target, dtype=float) * self.root
        c = np.zeros(d, dtype=np.int64)
        for k in range(d - 1, -1, -1):
            r = int(np.rint((self.star[k] @ v) / self.norm[k]))
            c[k] = r
            v = v - r * self.B[k] * self.root
        return c
