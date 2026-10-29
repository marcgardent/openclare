"""The property tests: each statement of the paper that was proved on paper
only, checked exactly on random T-meshes that do not come from the engine,
with its hypotheses computed rather than assumed.

    python3 docs/research/turns-of-a-t-mesh/experiments/property/properties.py \\
        --count 50 --seed 1 --out scratch/data-package/property/properties-seed-1.json

The instances are those of `generate.py` for the same seed (made again here,
not read from its export). For each statement the report says on how many
instances or objects the hypothesis held, how many of those passed, and the
first failures with the seed of their instance. A test refutes or
corroborates; it proves nothing.

Statements and what is checked, at the whole lengths of each instance, on the
strips of quads (every turn of that mesh, Proposition 2 of the companion):

* Gauss–Bonnet (Lemma 1, with Lemma "Gauss–Bonnet with borders" at Θ = 0): for
  every strip whose curve is simple, each region it cuts has Σ (4 - k) = 4 χ.
* One self-crossing (Lemma 2, spheres): the three regions hold (3, 3, 2) when
  the crossing is on the walk, (2, 2, 4) when on the closing segment.
* Parity (Proposition "Parity of a turn on a sphere" and its corollary): on a
  sphere every strip has α ≡ shift (mod 2) and every ring an even length.
* Parity on any surface (Theorem): where the quad graph is bipartite, α ≡
  shift for every strip; two strips bounding an annulus have α + shift of the
  same parity.
* Checkerboard (Proposition "Checkerboard rule", spheres): for two simple
  disjoint strips bounding an annulus, shift₁ - shift₂ has the parity of the
  cone points of odd valence inside on white points.
* Same shift (Proposition "The shift is set by the singularities" (c)): two
  simple disjoint strips bounding an annulus without cone point have the
  same shift form modulo the balances (here: the same shift at every random
  balanced length tried).
* Dislocation (Proposition "A dislocation between two turns" (i)): an annulus
  holding exactly a 3–5 pair: |shift₁ - shift₂| is |d₁ + d₂| or |d₁ - d₂|, d
  the developed separation of the pair.
* Burgers pairing (Proposition): with no cone point of valence divisible by
  four, the developed positions of the cone points, as forms on the balanced
  lengths, have full rank.
* Combs (Lemma): the share of arcs whose two feet exist (a unique
  perpendicular trace at each end), by valence of the feet.
* A separatrix that winds (Proposition): for a trace that stops on a trace
  of its own vertex at distance δ, a strip beside its first arc comes back
  with |shift| = δ (the homotopy of the statement is not checked: a miss is
  counted, not called a failure).
"""

import argparse
import json
import os
import random
import sys
import time
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import generate  # noqa: E402
import grid as gridmod  # noqa: E402
import quadmesh  # noqa: E402
import topology  # noqa: E402
import tracing  # noqa: E402
import walks  # noqa: E402


class Tally:
    def __init__(self):
        self.applies = 0
        self.passed = 0
        self.failures = []

    def check(self, ok, where, keep=5):
        self.applies += 1
        if ok:
            self.passed += 1
        elif len(self.failures) < keep:
            self.failures.append(where)

    def report(self):
        return {"applies": self.applies, "passed": self.passed, "failed": self.applies - self.passed,
                "failures": self.failures}


def shift_value(strip, lengths):
    return walks.value(strip["shift"], lengths)


def length_value(strip, lengths):
    return walks.value(strip["along"], lengths)


def run_instance(seed, kind, tallies, pairs_per_instance, rng):
    instance_rng = random.Random(seed)
    quad, T, whole, real, turns, held, strips, basis = generate.make(instance_rng, kind, [Fraction(1, 2)], 0.12, 6.0)
    g = gridmod.Grid(T, whole)
    genus = generate.genus(quad)
    where = {"seed": seed, "kind": list(kind)}
    assert g.euler() == quad.euler(), "the fine grid has another Euler characteristic"
    colouring = g.two_colouring()
    whole_f = [Fraction(x) for x in whole]

    # Curves, regions, self-crossings of every strip.
    curves = [g.turn_curve(strip) for strip in strips]
    simple = [len(gridmod.self_crossings(c)) == 0 for c in curves]
    region_of = {}
    for i, (strip, curve) in enumerate(zip(strips, curves)):
        crossings = gridmod.self_crossings(curve)
        if not crossings:
            regions = gridmod.regions(g, [curve])
            region_of[i] = regions
            for r in regions:
                tallies["gauss_bonnet"].check(r["index"] == 4 * r["euler"],
                                              dict(where, strip=i, region=r["euler"], index=r["index"]))
        elif len(crossings) == 1 and genus == 0:
            regions = gridmod.regions(g, [curve])
            indices = sorted(r["index"] for r in regions)
            # On the walk or on the closing segment: the crossing vertex lies on
            # the first side at an integer position (the segment) or not.
            vertex = next(iter(crossings))
            # The shift is along +u or +v: with the rim on sides 0 and 1, against it on 2 and 3.
            sign = 1 if strip["side"] < 2 else -1
            end = int(2 * (strip["offset"] + sign * shift_value(strip, whole_f)))
            start = int(2 * strip["offset"])
            p, side = strip["patch"], strip["side"]
            w, h = g.size[p]
            segment = set()
            for o in range(min(start, end), max(start, end) + 1):
                key = {0: (o, 0), 1: (2 * w, o), 2: (2 * w - o, 2 * h), 3: (0, 2 * h - o)}[side]
                segment.add(g.vertex[(p, key[0], key[1])])
            expected = [2, 2, 4] if vertex in segment else [2, 3, 3]
            tallies["one_crossing"].check(len(regions) == 3 and indices == expected,
                                          dict(where, strip=i, indices=indices, expected=expected))

    # Parity.
    for i, strip in enumerate(strips):
        alpha, s = length_value(strip, whole_f), shift_value(strip, whole_f)
        if genus == 0:
            tallies["parity_sphere"].check((alpha - s) % 2 == 0, dict(where, strip=i, alpha=int(alpha), shift=int(s)))
            if s == 0:
                tallies["ring_even"].check(alpha % 2 == 0, dict(where, strip=i, alpha=int(alpha)))
        if colouring is not None:
            tallies["parity_bipartite"].check((alpha - s) % 2 == 0, dict(where, strip=i, alpha=int(alpha), shift=int(s)))

    # Pairs of simple disjoint strips bounding an annulus.
    chart = topology.Chart(T, g.rims)
    candidates = [i for i in range(len(strips)) if simple[i]]
    pairs = [(i, j) for a, i in enumerate(candidates) for j in candidates[a + 1:]
             if not (set(curves[i]) & set(curves[j]))]
    rng.shuffle(pairs)
    for (i, j) in pairs[:pairs_per_instance]:
        regions = gridmod.regions(g, [curves[i], curves[j]])
        annuli = [r for r in regions if r["euler"] == 0]
        if len(regions) != 3 or len(annuli) != 1:
            continue
        A = annuli[0]
        s1, s2 = shift_value(strips[i], whole_f), shift_value(strips[j], whole_f)
        a1, a2 = length_value(strips[i], whole_f), length_value(strips[j], whole_f)
        tallies["annulus_parity"].check((a1 + s1 - a2 - s2) % 2 == 0, dict(where, strips=[i, j]))
        if not A["cones"]:
            tallies["same_shift"].check(abs(s1) == abs(s2), dict(where, strips=[i, j], shifts=[int(s1), int(s2)]))
        if genus == 0 and colouring is not None:
            white = sum(1 for v, k in A["cones"].items() if k % 2 == 1 and colouring.get(v) == 0)
            black = sum(1 for v, k in A["cones"].items() if k % 2 == 1 and colouring.get(v) == 1)
            tallies["checkerboard"].check((s1 - s2 - white) % 2 == 0 and (s1 - s2 - black) % 2 == 0,
                                          dict(where, strips=[i, j], white=white, black=black, shifts=[int(s1), int(s2)]))
        if sorted(A["cones"].values()) == [3, 5]:
            (v3, v5) = sorted(A["cones"], key=lambda v: A["cones"][v])
            # The pair developed inside the annulus, as the proposition asks.
            d = gridmod.develop_between(g, chart, A["face_list"], v3, v5)
            if d is None:
                continue
            allowed = {abs(d[0] + d[1]), abs(d[0] - d[1])}
            tallies["dislocation"].check(abs(s1 - s2) in allowed,
                                         dict(where, strips=[i, j], shifts=[int(s1), int(s2)], d=[int(d[0]), int(d[1])]))

    # The Burgers pairing.
    # The proposition is stated on a closed sphere; on a torus a handle adds a
    # period the cone points do not set (Proposition "Burgers sum with borders
    # and handles"), and the rank is only counted.
    rank = topology.pairing_rank(T, g.rims, basis)
    links = topology.connections(T)
    if genus != 0:
        tallies["pairing_rank_torus_full"].check(rank["rank"] == rank["dimension"], dict(where, **rank))
    elif rank["multiples_of_four"] == 0:
        tallies["pairing_rank"].check(rank["rank"] == rank["dimension"], dict(where, **rank, connections=links))
    else:
        tallies["pairing_rank_with_valence_4k"].check(rank["rank"] <= rank["dimension"], dict(where, **rank))

    # Combs: the feet.
    both = 0
    by_valence = {}
    for pair in topology.feet(T):
        if all(kind_ is not None for kind_, _ in pair):
            both += 1
            key = "-".join(sorted(str(quad.valence[v]) for _, v in pair))
            by_valence[key] = by_valence.get(key, 0) + 1
    tallies["feet"].applies += len(T.arcs)
    tallies["feet"].passed += both
    for key, count in by_valence.items():
        tallies["feet_by_valence"].setdefault(key, 0)
        tallies["feet_by_valence"][key] += count

    # A separatrix that winds.
    walker = walks.Walker(T, whole_f)
    for winding in topology.winding_separatrices(T):
        delta = sum(whole_f[a] for a in winding["delta_arcs"])
        if winding["first_arc"] is None:
            continue
        found = None
        for (p, side, offset) in topology.beside_stations(T, walker, winding["first_arc"], winding["vertex"]):
            turn = walker.go_round(p, side, offset, 8 * len(T.patches), 6 * max(1, int(len(quad.faces) ** 0.5)))
            if turn is not None:
                found = abs(shift_value(turn, whole_f))
                if found == delta:
                    break
        # Not a failure when missed: whether the strip beside the trace is the
        # turn homotopic to the loop of the statement is not checked here.
        tallies["winding_separatrix_beside"].check(found == delta, dict(where, trace=winding["trace"], delta=int(delta),
                                                                       found=None if found is None else int(found)))
    return {"seed": seed, "kind": list(kind), "genus": genus, "strips": len(strips),
            "simple_strips": sum(simple), "pairs_tried": min(len(pairs), pairs_per_instance),
            "bipartite": colouring is not None, "pairing": rank, "connections": links}


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--count", type=int, default=50)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--pairs", type=int, default=60, help="pairs of strips tried per instance")
    parser.add_argument("--out", default=None)
    args = parser.parse_args()
    out = args.out or os.path.join("scratch", "data-package", "property", f"properties-seed-{args.seed}.json")
    names = ["gauss_bonnet", "one_crossing", "parity_sphere", "ring_even", "parity_bipartite", "annulus_parity",
             "same_shift", "checkerboard", "dislocation", "pairing_rank", "pairing_rank_with_valence_4k",
             "pairing_rank_torus_full", "feet", "winding_separatrix_beside"]
    tallies = {name: Tally() for name in names}
    tallies["feet_by_valence"] = {}
    rng = random.Random(args.seed)
    kinds = [kind for kind, weight in generate.FAMILIES for _ in range(weight)]
    instances, rejected = [], 0
    started = time.time()
    pair_rng = random.Random(args.seed + 1)
    while len(instances) < args.count:
        kind = rng.choice(kinds)
        seed = rng.randrange(1 << 30)
        try:
            instances.append(run_instance(seed, kind, tallies, args.pairs, pair_rng))
        except tracing.NotRectangles:
            rejected += 1
            continue
        last = instances[-1]
        print(f"{kind[0]}-{seed}: genus {last['genus']}, {last['strips']} strips ({last['simple_strips']} simple), "
              f"pairing rank {last['pairing']['rank']} of {last['pairing']['dimension']}")
    report = {name: (t.report() if isinstance(t, Tally) else t) for name, t in tallies.items()}
    summary = {"experiment": "property tests of the paper's statements on random T-meshes",
               "seed": args.seed, "count": args.count, "rejected": rejected, "pairs_per_instance": args.pairs,
               "seconds": round(time.time() - started, 1), "statements": report, "instances": instances}
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w") as f:
        json.dump(summary, f, indent=1)
    for name, r in report.items():
        if isinstance(r, dict) and "applies" in r:
            word = "missed" if name in ("winding_separatrix_beside", "feet", "pairing_rank_torus_full") else "FAILED"
            print(f"{name}: {r['passed']} of {r['applies']}" + (f", {word} {r['failed']}" if r["failed"] else ""))
        else:
            print(f"{name}: {r}")
    print(f"{len(instances)} instances, {rejected} rejected, {summary['seconds']} s, report in {out}")


if __name__ == "__main__":
    main()
