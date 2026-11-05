#!/usr/bin/env python3
"""The homotopies of the mechanisms of the conflicts (Section "Conflicts
between turns": Propositions "Exchanging two vertices", "A dislocation between
two turns" with "Staircases", and "A separatrix that winds"), decided in the
free group of the surface without its singular vertices.

- The words: the chart of `fox.py` (the surface cut along a tree of the
  nodes), each crossing of a tree arc a word in one letter per singular
  vertex but the root. The word map is checked to be an isomorphism from the
  fundamental group onto the free group on those letters: the small loop round
  every node that is not singular reduces to the empty word (these are the
  relations of the presentation by the crossings of the tree), and the loop
  round every singular vertex but the root to a conjugate of its own letter
  (so the map is onto, hence one to one, a free group of finite rank being
  Hopfian).
- A closed curve crossing the arcs away from the nodes has the word of its
  crossings: a turn's curve, and a path along arcs pushed off them to one side
  (to the left of every arc, or to the right of every arc: across an arc or a
  T-junction the two are homotopic, so they differ only at singular vertices).
- A pair of turns (rows of `pairs.py` with an exchange or a dislocation):
  the loops λ = γ̄1 κ γ̄2^{±1} κ^{-1} of Proposition "Exchanging two vertices"
  for every path κ inside the chart between a chord of γ1 and a chord of γ2
  whose walking directions in the chart are equal or opposite (κ is then
  trivial in the fundamental group, and carries one walking direction onto
  the other, γ2 walked backwards if they are opposite). The loop of the
  mechanism: round c, along the staircase σ (pushed to its left) to c', round
  c' backwards for an exchange and the same way for a 3–5 pair, back along σ.
  A loop conjugate to it passes if its rotation is trivial and the across
  component of its translation is, modulo the balances, ± the combination
  of the certificate. The class in homology of λ does not depend on κ: where
  it differs from the mechanism's, no κ at all gives the mechanism. Control:
  the mechanism's loop with c' walked round the other way.
- Sets of three turns with multipliers ±1: the products of their loops in
  both cyclic orders, each walked either way, the paths between them inside
  the chart; only the orientations with the mechanism's class in homology.
- A turn not closable alone with a winding separatrix σ of v stopping on τ
  (Proposition "A separatrix that winds"): the loop along σ from v to where
  it stops and back along τ to v, pushed to the left or to the right, against
  the turn's curve.

Two words are freely homotopic loops if and only if their cyclic reductions
are cyclic permutations of one another; a loop and its inverse are both tried.
Nothing here is floating point but the identities modulo the balances (a basis
of the balanced lengths), which `pairs.py` already checks.

    python3 docs/research/turns-of-a-t-mesh/experiments/conjugacy.py [<export>] \\
        [--pairs docs/research/turns-of-a-t-mesh/experiments/results/pairs.json]

Ten minutes on an i7-7700K loaded by other jobs, most of it on the sets of
three turns (thousands of loops each). Writes results/conjugacy.json.
"""

import argparse
import itertools
import json
import time
from pathlib import Path

import tmesh  # first: it sets the threads of the linear algebra

import numpy as np
from fox import Fox
from geometry import DIRECTION, Layout, dense, rotate, shift_form


# --- the free group: words are lists of (letter, ±1) -------------------------------------


def reduce(word):
    out = []
    for letter in word:
        if out and out[-1][0] == letter[0] and out[-1][1] == -letter[1]:
            out.pop()
        else:
            out.append(letter)
    return out


def cyclic(word):
    """The cyclic reduction of a word."""
    w = reduce(word)
    i, j = 0, len(w) - 1
    while i < j and w[i][0] == w[j][0] and w[i][1] == -w[j][1]:
        i, j = i + 1, j - 1
    return w[i:j + 1]


def inverse(word):
    return [(v, -e) for v, e in reversed(word)]


def conjugate(a, b):
    """Whether the words a and b are conjugate in the free group: their
    cyclic reductions are cyclic permutations of one another."""
    a, b = cyclic(a), cyclic(b)
    if len(a) != len(b):
        return False
    text = "".join(f",{v}{'+' if e == 1 else '-'}" for v, e in a)
    return "".join(f",{v}{'+' if e == 1 else '-'}" for v, e in b) + "," in text + text + ","


def letters(word):
    return "".join(f"x{v}" + ("" if e == 1 else "⁻¹") for v, e in word)


# --- crossings and their words ------------------------------------------------------------


class Words:
    def __init__(self, L, fox):
        self.L, self.fox = L, fox
        self.rim_before, self.rim_after = {}, {}
        for p, rim in enumerate(L.rims):
            for k, (half, side) in enumerate(rim):
                self.rim_before[half] = rim[k - 1][0]
                self.rim_after[half] = rim[(k + 1) % len(rim)][0]

    def of(self, crossings):
        """The word of a sequence of crossings (patch, half of its rim)."""
        return self.fox.turn_word({"crossings": [(p, h, None, 0.0) for p, h in crossings]})

    def next_ccw(self, half):
        """The half-arc leaving the same node next counter-clockwise."""
        return self.rim_before[half] ^ 1

    def round(self, half):
        """The crossings of the small counter-clockwise loop round the node
        `half` leaves, from the corner between `half` and the next half
        counter-clockwise."""
        L, out, h = self.L, [], half
        for _ in range(len(L.nodes[L.tail(half)]["halves"])):
            out.append((L.place[h][0], self.rim_before[h]))
            h = self.next_ccw(h)
        assert h == half
        return out

    def clockwise(self, half):
        """The half-arc leaving the same node next clockwise."""
        return self.rim_after[half ^ 1]

    def turning(self, incoming, outgoing, left):
        """The crossings at a node between two halves of a path pushed to the
        left (clockwise from the incoming half reversed to the outgoing one)
        or to the right (counter-clockwise)."""
        L, out, back = self.L, [], incoming ^ 1
        h = self.clockwise(back) if left else self.next_ccw(back)
        while h != outgoing:
            out.append((L.place[h][0], h) if left else (L.place[h ^ 1][0], h ^ 1))
            if h == back:
                raise ValueError("a path cannot turn back onto the half it came along")
            h = self.clockwise(h) if left else self.next_ccw(h)
        return out

    def path(self, halves, left, closed):
        """The crossings of a path of half-arcs pushed to one side; a closed
        path turns at its first node too."""
        out = []
        pairs = list(zip(halves, halves[1:]))
        if closed:
            pairs.append((halves[-1], halves[0]))
        for a, b in pairs:
            out += self.turning(a, b, left)
        return out


def halves_of(L, start, arcs):
    """The half-arcs of a path of arcs from the node `start`."""
    out, node = [], start
    for a in arcs:
        n0, n1 = L.arcs[a]["nodes"]
        h = 2 * a if n0 == node else 2 * a + 1
        assert L.tail(h) == node, (a, node)
        out.append(h)
        node = L.head(h)
    return out, node


def check_presentation(L, W):
    """The word map is an isomorphism onto the free group on the letters of
    the singular vertices but the root."""
    trivial, own, bad = 0, 0, []
    root = W.fox.order[0]
    for v, node in enumerate(L.nodes):
        word = cyclic(W.of(W.round(node["halves"][0])))
        if not L.singular(v):
            trivial += not word
            if word:
                bad.append(v)
        elif v != root:
            if len(word) == 1 and word[0][0] == v:
                own += 1
            else:
                bad.append(v)
    singular = sum(L.singular(v) for v in range(len(L.nodes)))
    return {"nodes": len(L.nodes), "not_singular_trivial": trivial, "singular_own_letter": own,
            "singular": singular, "root": root, "failures": bad,
            "isomorphism": not bad and own == singular - 1}


# --- the turns' loops ---------------------------------------------------------------------


def chords_of(L, turn):
    """For each chord of a turn: (patch, side entered, side left)."""
    out = []
    cs, _ = L.chords(turn)
    for patch, a, b in cs:
        out.append((patch, int(a) % 4, int(b) % 4, a, b))
    return out


def loop_from(W, turn, i):
    """The word of a turn's curve from a point of its chord i (the chord
    before crossing i)."""
    c = turn["crossings"]
    return W.of([(p, h) for (p, h, q, into) in c[i:] + c[:i]])


def walking(L, W, turn, i):
    """The walking direction of chord i in the frame of the chart, as a
    quarter turn k (DIRECTION[k]), or None for a chord that turns."""
    patch, s_in, s_out, a, b = chords_of(L, turn)[i]
    if (s_out - s_in) % 4 != 2:
        return None
    return (s_in + 1 + W.fox.charts[patch].k) % 4


def pair_loops(L, W, ids):
    """λ = γ̄1 κ γ̄2^{∓1} κ^{-1} for every path κ inside the chart from a
    chord of γ1 to a chord of γ2 whose walking directions in the chart are
    equal (γ2 then walked backwards: the upper sign) or opposite, with the
    across direction of γ1 in the chart; κ is trivial in the fundamental
    group, the chart being a disc. Distinct cyclic words."""
    t1, t2 = (L.turns[t] for t in ids)
    chords = []
    for t in (t1, t2):
        chords.append([(i, walking(L, W, t, i), loop_from(W, t, i)) for i in range(len(t["crossings"]))])
    seen, out = set(), []
    for i, k1, a in chords[0]:
        if k1 is None:
            continue
        for j, k2, b in chords[1]:
            if k2 is None or (k1 - k2) % 2:
                continue
            same = k1 == k2
            # γ̄1 then γ̄2^{-1} (equal directions) or γ̄2 walked backwards
            # (opposite): the later crossings on the left, as fox.py writes.
            word = reduce((inverse(b) if same else b) + a)
            key = tuple(cyclic(word))
            if key in seen:
                continue
            seen.add(key)
            out.append({"chords": [i, j], "kind": "parallel" if same else "antiparallel", "word": word,
                        "across": rotate(1, DIRECTION[k1])})
    return out


def exponents(word):
    """The exponent sum of each letter (the class in homology)."""
    out = {}
    for v, e in word:
        out[v] = out.get(v, 0) + e
    return {v: e for v, e in out.items() if e}


def set_loops(L, W, ids, mu):
    """For three turns: the products, in both cyclic orders, of their loops
    from chords whose walking directions in the chart lie on one axis, each
    loop walked forwards or backwards (the first forwards), the paths between
    them inside the chart. Only the orientations whose class in homology is
    that of the mechanism's loop or of its inverse are formed."""
    loops = []
    for t in ids:
        turn = L.turns[t]
        loops.append([(i, walking(L, W, turn, i), loop_from(W, turn, i)) for i in range(len(turn["crossings"]))])
    wanted = [exponents(mu), exponents(inverse(mu))]
    out, seen = [], set()
    for signs in itertools.product((1, -1), repeat=len(ids) - 1):
        sigma = (1,) + signs
        total = {}
        for (_, _, w), e in zip((entry[0] for entry in loops), sigma):
            for v, x in exponents(w if e == 1 else inverse(w)).items():
                total[v] = total.get(v, 0) + x
        if {v: x for v, x in total.items() if x} not in wanted:
            continue
        for chosen in itertools.product(*loops):
            k = [c[1] for c in chosen]
            if None in k or any((x - k[0]) % 2 for x in k):
                continue
            words = [c[2] if e == 1 else inverse(c[2]) for c, e in zip(chosen, sigma)]
            for order in ((0, 1, 2), (0, 2, 1)):
                # The later loops on the left, as fox.py writes.
                word = reduce([x for r in reversed(order) for x in words[r]])
                key = tuple(cyclic(word))
                if key in seen:
                    continue
                seen.add(key)
                out.append({"chords": [c[0] for c in chosen], "kind": f"order {order}, signs {sigma}",
                            "word": word, "across": rotate(1, DIRECTION[k[0]])})
    return out


def mechanism_loop(L, W, start, arcs, same_way):
    """Round the vertex `start`, along the staircase (pushed to its left) to
    its other end, round that vertex (backwards for an exchange, the same
    way for a 3–5 pair), back along the staircase."""
    halves, end = halves_of(L, start, arcs)
    transport = W.of(W.path(halves, left=True, closed=False))
    # The corner left of the staircase at its end: the half after the last
    # half in the rim of the patch on its left.
    P, k = L.place[halves[-1]]
    after = L.rims[P][(k + 1) % len(L.rims[P])][0]
    round_start = W.of(W.round(halves[0]))
    round_end = W.of(W.round(after))
    if not same_way:
        round_end = inverse(round_end)
    # Later on the left: round the start, the staircase, round the end, back.
    return reduce(inverse(transport) + round_end + transport + round_start), end


def separatrix_loop(L, W, found):
    """The loop along σ from v to where it stops on τ, back along τ to v,
    pushed to the left and to the right: {side: word}."""
    from pairs import traces

    chains = traces(L)
    halves, end = halves_of(L, found["vertex"], chains[found["sigma"]])
    back, start = halves_of(L, end, list(reversed(found["delta_arcs"])))
    assert start == found["vertex"]
    through = [L.tail(h) for h in halves + back][1:]
    return {side: W.of(W.path(halves + back, side == "left", closed=True)) for side in ("left", "right")}, \
        [v for v in through if L.singular(v)]


def decide(a, b):
    """Whether a is conjugate to b, to its inverse, or to neither (None)."""
    if conjugate(a, b):
        return "conjugate"
    if conjugate(a, inverse(b)):
        return "conjugate to the inverse"
    return None


def describe(L, W, row):
    ids = [L.held[h] for h in row["turns"]]
    out = {"row": row["row"], "way": row["way"], "turns": row["turns"]}
    if len(ids) == 1:
        found = row.get("separatrices") or []
        if not found:
            out["mechanism"] = "none found"
            return out
        s = found[0]
        turn = cyclic(W.fox.turn_word(L.turns[ids[0]]))
        loops, singular = separatrix_loop(L, W, s)
        out.update(mechanism="winding separatrix", vertex=s["vertex"], sigma=s["sigma"], tau=s["tau"],
                   singular_vertices_passed=singular, turn_word=letters(turn),
                   sides={side: {"word": letters(cyclic(w)), "homotopic": decide(w, turn)}
                          for side, w in loops.items()})
        out["passes"] = any(side["homotopic"] for side in out["sides"].values())
        return out
    stair = row["staircases"][0] if row.get("staircases") else None
    if stair is None or len(ids) > 3 or any(abs(int(u)) != 1 for u in row["primitive_multipliers"]):
        out["mechanism"] = row.get("mechanism", "none found")
        if stair is not None:
            out["not_tried"] = "more than three turns, or multipliers other than ±1"
        return out
    kind = "dislocation" if sorted(stair["valences"]) == [3, 5] else "exchange"
    mu, end = mechanism_loop(L, W, stair["from"], stair["arcs"], same_way=kind == "dislocation")
    control, _ = mechanism_loop(L, W, stair["from"], stair["arcs"], same_way=kind != "dislocation")
    assert end == stair["to"]
    n = len(L.arcs)
    combination = sum((int(u) * dense(shift_form(L.turns[t]), n) for t, u in zip(ids, row["primitive_multipliers"])),
                      np.zeros(n, np.int64))
    # The class in homology of λ does not depend on κ: the orientations of
    # the turns' loops for which it is that of the mechanism's loop.
    turns = [W.fox.turn_word(L.turns[t]) for t in ids]
    wanted = [exponents(mu), exponents(inverse(mu))]
    out["orientations_with_the_class_of_the_mechanism"] = [
        list(signs) for signs in itertools.product((1, -1), repeat=len(ids) - 1)
        if exponents([x for w, e in zip(turns, (1,) + signs) for x in (w if e == 1 else inverse(w))]) in wanted]
    loops = pair_loops(L, W, ids) if len(ids) == 2 else set_loops(L, W, ids, mu)
    homotopic, controls = [], 0
    for entry in loops:
        controls += bool(decide(entry["word"], control))
        verdict = decide(entry["word"], mu)
        if not verdict:
            continue
        # The across component of the translation of λ against the
        # combination of the certificate, modulo the balances.
        M = W.fox.motion_of_word(entry["word"])
        component = entry["across"][0] * M.t0 + entry["across"][1] * M.t1
        sign = next((s for s in (1, -1) if L.same_mod_balances(component, s * combination)), 0)
        homotopic.append({"chords": entry["chords"], "kind": entry["kind"], "rotation": int(M.k % 4),
                          "combination_sign": sign, "word": letters(cyclic(entry["word"])), "homotopic": verdict})
    out.update(mechanism=kind, staircase={"from": stair["from"], "to": stair["to"], "arcs": stair["arcs"]},
               mechanism_word=letters(cyclic(mu)), loops_tried=len(loops), homotopic=homotopic,
               homotopic_to_the_control=controls,
               passes=any(h["rotation"] == 0 and h["combination_sign"] for h in homotopic))
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "conjugacy")
    parser.add_argument("--pairs", type=Path, default=tmesh.HERE / "results" / "pairs.json",
                        help="the summary of pairs.py")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    table = json.loads(arguments.pairs.read_text())
    folder, paths = tmesh.ways(arguments.export, arguments.call)
    for number, sha in table["ways"].items():
        if int(number) not in paths or tmesh.sha256(paths[int(number)]) != sha:
            raise SystemExit(f"{arguments.pairs} was not made from this export (way {number})")
    conflicts = json.loads((tmesh.HERE / "results" / "conflicts.json").read_text())["table"]
    number_of = {(int(next(iter(r["turns_in_each_way"]))), tuple(r["turns"])): i + 1 for i, r in enumerate(conflicts)}
    layouts, presentations, rows, ways_read = {}, {}, [], []
    for row in table["rows"]:
        number = row["way"]
        if number not in layouts:
            way = tmesh.Way(paths[number])
            ways_read.append(way)
            L = Layout(way)
            fox = Fox(L)
            if not fox.conventions():
                raise SystemExit(f"way {number}: no convention of the recursion holds")
            W = Words(L, fox)
            presentations[number] = check_presentation(L, W)
            print(f"way {number}: {presentations[number]}", flush=True)
            if not presentations[number]["isomorphism"]:
                raise SystemExit(f"way {number}: the word map is not an isomorphism")
            layouts[number] = (L, W)
        L, W = layouts[number]
        row = dict(row, row=number_of[(number, tuple(row["turns"]))])
        out = describe(L, W, row)
        rows.append(out)
        print(f"row {out['row']} (way {number}, turns {out['turns']}): {out['mechanism']}"
              + (f", passes {out['passes']}" if "passes" in out else "")
              + (f", {out['loops_tried']} loops tried, orientations with the mechanism's class in homology "
                 f"{out['orientations_with_the_class_of_the_mechanism']}" if "loops_tried" in out else ""), flush=True)
    summary = {
        "experiment": "the homotopies of the mechanisms of the conflicts, by conjugacy in the free group",
        **tmesh.record(arguments, folder, ways_read, started, clock, table=arguments.pairs.name),
        "presentations": presentations,
        "rows": rows,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
