"""Walks across the rectangles of a T-mesh at given lengths, in exact
arithmetic: the turns (`go_round` and `shift_along` of
`crates/openclare/src/quantization/tubes.rs`, ported line by line), the
strips of quads of whole lengths (the walks from every half-integer
station), and random balanced lengths, whole or rational, made by adding
strips to the lengths of the quad mesh.

Lengths are `Fraction`s; the shift of a turn is a dict `arc -> whole
coefficient`; a rim, as in the engine, lists its halves with their sides and
where each starts, as numbers and as expressions in the lengths.
"""

from fractions import Fraction

DIRECTION = [(1, 0), (0, 1), (-1, 0), (0, -1)]


def merged(terms):
    out = {}
    for arc, c in terms:
        out[arc] = out.get(arc, 0) + c
    return {arc: c for arc, c in out.items() if c != 0}


def turned(quarter_turns, pair):
    a, b = pair
    negated = lambda row: {arc: -c for arc, c in row.items()}
    k = quarter_turns % 4
    if k == 0:
        return (dict(a), dict(b))
    if k == 1:
        return (negated(b), dict(a))
    if k == 2:
        return (negated(a), negated(b))
    return (dict(b), negated(a))


class Rim:
    """A rectangle's rim from its first corner, at the lengths given."""

    def __init__(self, rim, lengths):
        self.halves = rim
        self.start = []
        self.start_expression = []
        self.side_start = [0] * 4
        at = (Fraction(0), Fraction(0))
        expression = ({}, {})
        for k, (half, side) in enumerate(rim):
            if k == 0 or side != rim[k - 1][1]:
                self.side_start[side] = k
            self.start.append(at)
            self.start_expression.append(expression)
            arc = half // 2
            d = DIRECTION[side]
            at = (at[0] + d[0] * lengths[arc], at[1] + d[1] * lengths[arc])
            u, v = dict(expression[0]), dict(expression[1])
            if d[0]:
                u[arc] = u.get(arc, 0) + d[0]
            if d[1]:
                v[arc] = v.get(arc, 0) + d[1]
            expression = (merged(u.items()), merged(v.items()))
        if at != (0, 0):
            raise ValueError(f"the rim does not close at these lengths: {at}")

    def side_range(self, side):
        begin = self.side_start[side]
        end = len(self.halves) if side == 3 else self.side_start[side + 1]
        return begin, end

    def side_length(self, side, lengths):
        begin, end = self.side_range(side)
        return sum((lengths[self.halves[k][0] // 2] for k in range(begin, end)), Fraction(0))


class Walker:
    def __init__(self, tmesh, lengths):
        self.T = tmesh
        self.lengths = lengths
        self.rims = [Rim(patch["rim"], lengths) for patch in tmesh.patches]
        # Where each half lies: its patch and its index on the rim.
        self.place = {}
        for p, rim in enumerate(self.rims):
            for k, (half, _) in enumerate(rim.halves):
                self.place[half] = (p, k)

    def go_round(self, patch, side, offset, steps, longest):
        """The walk from `offset` along `side` of `patch`, until it comes back
        in through that side the same way; None when it does not within
        `steps` crossings or `longest` quads along, or hits a node."""
        lengths = self.lengths
        start = (patch, side, offset)
        quarter_turns = 0
        around = Fraction(0)
        crossings, walked = [], []
        for _ in range(steps):
            rim = self.rims[patch]
            opposite = (side + 2) % 4
            begin, end = rim.side_range(opposite)
            opposite_length = rim.side_length(opposite, lengths)
            around += rim.side_length((side + 1) % 4, lengths)
            if around > longest:
                return None
            along = opposite_length - offset
            if along <= 0 or along >= opposite_length:
                return None
            before = Fraction(0)
            leaving = None
            for k in range(begin, end):
                length = lengths[rim.halves[k][0] // 2]
                if along < before + length:
                    leaving = (k, along - before)
                    break
                before += length
            k, into = leaving
            if into == 0:
                return None  # a node
            half = rim.halves[k][0]
            nxt, k_next = self.place[half ^ 1]
            next_rim = self.rims[nxt]
            next_side = next_rim.halves[k_next][1]
            next_begin = next_rim.side_start[next_side]
            before_twin = sum((lengths[next_rim.halves[m][0] // 2] for m in range(next_begin, k_next)), Fraction(0))
            crossings.append((patch, k, nxt, k_next))
            walked.append((patch, half, nxt, into))
            quarter_turns = (quarter_turns + (next_side + 2 + 4 - opposite) % 4) % 4
            patch, side, offset = nxt, next_side, before_twin + (lengths[half // 2] - into)
            if (patch, side) == start[:2] and quarter_turns == 0:
                first, second = self.shift_along(crossings)
                across = 0 if DIRECTION[start[1]][0] != 0 else 1
                shift, along_form = (first, second) if across == 0 else (second, first)
                # The shift is along +u or +v: the rim direction on sides 0 and
                # 1, against it on sides 2 and 3.
                sign = 1 if start[1] < 2 else -1
                if sign * (offset - start[2]) != value(shift, lengths):
                    raise AssertionError("the walk does not come back at offset + shift")
                return {"shift": shift, "along": along_form, "around": around, "patch": start[0],
                        "side": start[1], "offset": start[2], "crossings": walked}
        return None

    def shift_along(self, crossings):
        shift = ({}, {})
        for (patch, k, nxt, k_next) in crossings:
            rim, next_rim = self.rims[patch], self.rims[nxt]
            half, opposite = rim.halves[k]
            next_side = next_rim.halves[k_next][1]
            turn = (next_side + 2 + 4 - opposite) % 4
            here = rim.start_expression[k]
            there = (dict(next_rim.start_expression[k_next][0]), dict(next_rim.start_expression[k_next][1]))
            d = DIRECTION[next_side]
            arc = half // 2
            if d[0]:
                there[0][arc] = there[0].get(arc, 0) + d[0]
            if d[1]:
                there[1][arc] = there[1].get(arc, 0) + d[1]
            turned_here = turned(turn, here)
            turned_shift = turned(turn, shift)
            shift = tuple(merged(list(turned_shift[c].items()) + list(there[c].items())
                                 + [(i, -v) for i, v in turned_here[c].items()]) for c in range(2))
        return shift

    def turns_from_shares(self, shares, steps, longest):
        """The walks that come back, from each share of each side of each
        rectangle, every way once (by its start)."""
        out = []
        for p, rim in enumerate(self.rims):
            for side in range(4):
                length = rim.side_length(side, self.lengths)
                for share in shares:
                    turn = self.go_round(p, side, share * length, steps, longest)
                    if turn is not None:
                        out.append(turn)
        return out

    def strips(self, steps, longest):
        """Every strip of quads of whole lengths: the walks from the
        half-integer stations of every side, each strip once per direction,
        with the number of times it crosses each arc."""
        seen = set()
        out = []
        for p, rim in enumerate(self.rims):
            for side in range(4):
                length = self.rims[p].side_length(side, self.lengths)
                for k in range(int(length)):
                    offset = Fraction(2 * k + 1, 2)
                    if (p, side, offset) in seen:
                        continue
                    turn = self.go_round(p, side, offset, steps, longest)
                    if turn is None:
                        continue
                    # Mark every station the strip passes, so it is found once.
                    stations = [(p, side, offset)]
                    patch, s, o = p, side, offset
                    for (q, half, nxt, into) in turn["crossings"]:
                        next_rim = self.rims[nxt]
                        _, k_next = self.place[half ^ 1]
                        next_side = next_rim.halves[k_next][1]
                        before_twin = sum((self.lengths[next_rim.halves[m][0] // 2]
                                           for m in range(next_rim.side_start[next_side], k_next)), Fraction(0))
                        patch, s, o = nxt, next_side, before_twin + (self.lengths[half // 2] - into)
                        stations.append((patch, s, o))
                    if any(st in seen for st in stations):
                        continue
                    seen.update(stations)
                    crossed = {}
                    for (_, half, _, _) in turn["crossings"]:
                        crossed[half // 2] = crossed.get(half // 2, 0) + 1
                    turn["crossed"] = crossed
                    out.append(turn)
        return out


def value(expression, lengths):
    return sum((c * lengths[arc] for arc, c in expression.items()), Fraction(0))


def quad_lengths(tmesh):
    return [Fraction(len(arc["edges"])) for arc in tmesh.arcs]


def balance_rows(tmesh):
    """The balances, one row per rectangle and pair of opposite sides, as
    dicts `arc -> ±1` (an arc on both sides of the pair cancels out)."""
    rows = []
    for patch in tmesh.patches:
        for pair in range(2):
            rows.append(merged((half // 2, 1 if side < 2 else -1) for half, side in patch["rim"] if side % 2 == pair))
    return rows


def balanced_basis(tmesh):
    """A basis of the whole balanced lengths, as integer vectors: the
    rational null space of the balances, each vector scaled to whole
    entries. Strips are balanced, but not every balanced vector is a sum of
    strips: these move the nodes past one another and shift the turns."""
    import sympy

    n = len(tmesh.arcs)
    rows = balance_rows(tmesh)
    M = sympy.zeros(len(rows), n)
    for r, row in enumerate(rows):
        for arc, c in row.items():
            M[r, arc] = c
    basis = []
    for vector in M.nullspace():
        denominators = [sympy.Rational(x).q for x in vector]
        scale = sympy.ilcm(*denominators) if denominators else 1
        entries = [int(x * scale) for x in vector]
        g = 0
        for x in entries:
            g = sympy.igcd(g, x)
        basis.append([Fraction(x // g) for x in entries] if g else [Fraction(x) for x in entries])
    return basis


def random_lengths(rng, tmesh, strips, basis, whole_adds, real_moves):
    """Whole lengths: those of the quad mesh plus `whole_adds` random strips
    (each added once or twice) and random vectors of the balanced lattice
    with coefficients -1 or 1, kept while every arc stays at least one; real
    lengths: the whole ones plus `real_moves` random strips and basis vectors
    with weights in eighths between -1/4 and 1/4, exact in floating point,
    kept while every arc stays positive."""
    whole = quad_lengths(tmesh)
    strip_moves = [s["crossed"] for s in strips]
    basis_moves = [{a: c for a, c in enumerate(v) if c} for v in basis]
    # A strip added keeps every ring a ring; a vector of the lattice moves the
    # nodes past one another and shifts turns: drawn twice as often.
    moves = strip_moves + 2 * len(strip_moves) * basis_moves if basis_moves else strip_moves
    for _ in range(whole_adds):
        move = rng.choice(moves)
        times = rng.choice([-2, -1, 1, 1, 2, 3])
        candidate = list(whole)
        for arc, c in move.items():
            candidate[arc] += times * c
        if all(x >= 1 for x in candidate):
            whole = candidate
    real = list(whole)
    for _ in range(real_moves):
        move = rng.choice(moves)
        weight = Fraction(rng.choice([-2, -1, 1, 2]), 8)
        candidate = list(real)
        for arc, c in move.items():
            candidate[arc] += weight * c
        if all(x > 0 for x in candidate):
            real = candidate
    return whole, real
