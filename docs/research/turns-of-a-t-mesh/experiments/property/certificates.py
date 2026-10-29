"""The certificates of conflicts on random T-meshes, and the parity behind
Proposition "Certificates are sums of combs" (DT-142): is the odd part of a
certificate a cycle of the graph of feet?

    python3 docs/research/turns-of-a-t-mesh/experiments/property/certificates.py \\
        --count 50 --seed 1 --out scratch/data-package/property/certificates-seed-1.json

On each instance of `generate.py` (made again from the seed), the set `P` is
the balanced lengths with every arc at least one, and `H` the strips of the
whole lengths (every turn of that mesh) and the turns walked at the real
lengths, those whose shift form is not zero, each form once up to sign. The
exact machinery of `Check/certify.py` finds the cores (irreducible infeasible
sets of `H`, by elastic filtering) and for each an exact Farkas certificate
`c = Σ u_γ shift_γ + λᵀ B ≥ 0`, `c · b > 0`, checked in exact arithmetic; up to
`--cores` cores per instance, each found after a turn of the previous one is
let go. Then, with `c` scaled to whole numbers:

* the hypothesis of the proposition: every arc of the support of `c` has two
  feet, both of odd valence;
* the parity: the arcs with odd `c_a` form a cycle modulo 2 of the graph of
  feet (every vertex of odd valence is a foot of an even number of them, an
  arc whose two feet coincide counting twice).

A certificate meeting the hypothesis whose odd part is not a cycle refutes
the parity the paper observed; the instance is written beside the report.
"""

import argparse
import json
import math
import os
import random
import sys
import time
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "lean", "Check"))

import certify  # noqa: E402
import export  # noqa: E402
import generate  # noqa: E402
import topology  # noqa: E402
import tracing  # noqa: E402
import walks  # noqa: E402


def whole_certificate(u, c):
    """`u` and `c` scaled by the least common denominator, as whole numbers."""
    denominators = [x.denominator for x in u] + [x.denominator for x in c.values()]
    scale = 1
    for d in denominators:
        scale = scale * d // math.gcd(scale, d)
    return [int(x * scale) for x in u], {a: int(x * scale) for a, x in c.items()}, scale


def boundary(arcs, feet):
    """The feet met an odd number of times by the arcs, as a set (a loop at
    one foot counts twice); None if an arc has no foot."""
    odd = set()
    for a in arcs:
        for kind, v in feet[a]:
            if kind is None:
                return None
            odd ^= {v}
    return odd


def odd_part_is_cycle(c_whole, feet, quad, rows):
    """The hypothesis (feet of odd valence on the support), the parity of the
    odd part, and whether any representative `c + λᵀ B` could have an odd
    part that is a cycle: modulo 2, the boundary of the odd part must lie in
    the span of the boundaries of the balance rows (those whose arcs all have
    feet); a necessary condition, solved over GF(2)."""
    support = [a for a, x in c_whole.items() if x != 0]
    hypothesis = True
    for a in support:
        for kind, v in feet[a]:
            if kind is None or quad.valence[v] % 2 == 0:
                hypothesis = False
    odd_arcs = [a for a, x in c_whole.items() if x % 2 == 1]
    odd_boundary = boundary(odd_arcs, feet)
    cycle = odd_boundary == set()
    # Gaussian elimination over GF(2) on the boundaries of the rows.
    index = {}
    for a in range(len(feet)):
        for kind, v in feet[a]:
            if kind is not None and v not in index:
                index[v] = len(index)
    pivots = {}
    for row in rows:
        b = boundary([a for a, c in row.items() if c % 2 == 1], feet)
        if b is None:
            continue
        vector = 0
        for v in b:
            vector |= 1 << index[v]
        while vector:
            top = vector.bit_length() - 1
            if top in pivots:
                vector ^= pivots[top]
            else:
                pivots[top] = vector
                break
    target = 0
    for v in (odd_boundary or set()):
        target |= 1 << index[v]
    while target:
        top = target.bit_length() - 1
        if top not in pivots:
            break
        target ^= pivots[top]
    representative_mod2 = odd_boundary is not None and target == 0
    return hypothesis, cycle, representative_mod2, len(support), len(odd_arcs)


def run_instance(seed, kind, work, cores_per_instance, shares):
    rng = random.Random(seed)
    quad, T, whole, real, turns, held, strips, basis = generate.make(rng, kind, shares, 0.12, 6.0)
    # H: every strip of the whole lengths (every turn of that mesh) and every
    # turn walked at the real lengths, with a non-zero shift form, each form
    # once up to sign.
    forms = []
    for turn in strips + turns:
        shift = turn["shift"]
        if not shift:
            continue
        negated = {a: -c for a, c in shift.items()}
        if shift not in forms and negated not in forms:
            forms.append(shift)
    name = f"{kind[0]}-{seed}"
    way = export.way_json(T, whole, real, strips + turns, forms, shares, 0.12, 6.0, name, lower=1)
    folder = export.write(work, name, way, export.helices_json(strips, whole))
    W = certify.Way(os.path.join(folder, "way-0.json"))
    feet = topology.feet(T)
    rows = walks.balance_rows(T)
    Y = list(W.held)
    out = {"seed": seed, "kind": list(kind), "genus": generate.genus(quad), "turns_in_H": len(Y),
           "feasible": W.feasible(Y) if Y else True, "cores": []}
    counterexamples = []
    while Y and not W.feasible(Y) and len(out["cores"]) < cores_per_instance:
        core = W.core(Y)
        found = W.certificate(core)
        record = {"turns": core, "size": len(core)}
        if found is None:
            record["certificate"] = None
        else:
            u, c = found
            u_whole, c_whole, scale = whole_certificate(u, c)
            hypothesis, cycle, representative, support, odd = odd_part_is_cycle(c_whole, feet, quad, rows)
            record.update({
                "multipliers": [str(x) for x in u], "arcs": sorted(c.keys()), "scale": scale,
                "support": support, "odd_arcs": odd, "feet_of_odd_valence_on_support": hypothesis,
                "odd_part_is_cycle": cycle, "some_representative_could_mod_2": representative,
                "exact": W.valid(core, u, c),
            })
            if hypothesis and not cycle and not representative:
                counterexamples.append(record)
        out["cores"].append(record)
        Y = [k for k in Y if k != core[0]]
    out["counterexamples"] = counterexamples
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--count", type=int, default=50)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--cores", type=int, default=6, help="cores sought per instance")
    parser.add_argument("--work", default=None, help="where the exports with the bound 1 are written")
    parser.add_argument("--out", default=None)
    parser.add_argument("--shares", default="0.5", help="shares of each side the turns start from")
    parser.add_argument("--families", default=None,
                        help="the families drawn from, as 'kind:size:...' separated by commas, e.g. "
                             "'hull:16,torus:12:9:4'; default: those of generate.py")
    args = parser.parse_args()
    out = args.out or os.path.join("scratch", "data-package", "property", f"certificates-seed-{args.seed}.json")
    work = args.work or os.path.join("scratch", "data-package", "property", f"bound-1-seed-{args.seed}")
    shares = [Fraction(s) for s in args.shares.split(",")]
    rng = random.Random(args.seed)
    if args.families:
        kinds = []
        for text in args.families.split(","):
            parts = text.split(":")
            kinds.append(tuple([parts[0]] + [int(p) for p in parts[1:]]))
    else:
        kinds = [kind for kind, weight in generate.FAMILIES for _ in range(weight)]
    instances, rejected = [], 0
    started = time.time()
    while len(instances) < args.count:
        kind = rng.choice(kinds)
        seed = rng.randrange(1 << 30)
        try:
            instances.append(run_instance(seed, kind, work, args.cores, shares))
        except tracing.NotRectangles:
            rejected += 1
            continue
        last = instances[-1]
        print(f"{kind[0]}-{seed}: genus {last['genus']}, {last['turns_in_H']} turns in H, "
              f"{'feasible' if last['feasible'] else str(len(last['cores'])) + ' cores'}"
              f"{', COUNTEREXAMPLE' if last['counterexamples'] else ''}")
    cores = [c for i in instances for c in i["cores"]]
    with_certificate = [c for c in cores if c.get("certificate", True) is not None and "arcs" in c]
    summary = {
        "experiment": "certificates of conflicts on random T-meshes: the odd part against the graph of feet",
        "seed": args.seed, "count": args.count, "rejected": rejected, "cores_per_instance": args.cores,
        "bound": "every arc at least one", "shares": [str(s) for s in shares],
        "families": [list(k) for k in dict.fromkeys(kinds)],
        "seconds": round(time.time() - started, 1),
        "instances_infeasible": sum(1 for i in instances if not i["feasible"]),
        "cores": len(cores), "certificates": len(with_certificate),
        "certificates_exact": sum(1 for c in with_certificate if c["exact"]),
        "hypothesis_met": sum(1 for c in with_certificate if c["feet_of_odd_valence_on_support"]),
        "hypothesis_met_and_cycle": sum(1 for c in with_certificate
                                        if c["feet_of_odd_valence_on_support"] and c["odd_part_is_cycle"]),
        "hypothesis_met_not_cycle": sum(1 for c in with_certificate
                                        if c["feet_of_odd_valence_on_support"] and not c["odd_part_is_cycle"]),
        "hypothesis_met_not_cycle_but_a_representative_could": sum(
            1 for c in with_certificate if c["feet_of_odd_valence_on_support"] and not c["odd_part_is_cycle"]
            and c["some_representative_could_mod_2"]),
        "cycle_without_hypothesis": sum(1 for c in with_certificate
                                        if not c["feet_of_odd_valence_on_support"] and c["odd_part_is_cycle"]),
        "counterexamples": sum(len(i["counterexamples"]) for i in instances),
        "counterexamples_genus_0": sum(len(i["counterexamples"]) for i in instances if i["genus"] == 0),
        "core_sizes": sorted(set(c["size"] for c in cores)),
        "instances": instances,
    }
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w") as f:
        json.dump(summary, f, indent=1)
    for key in ("instances_infeasible", "cores", "certificates", "certificates_exact", "hypothesis_met",
                "hypothesis_met_and_cycle", "hypothesis_met_not_cycle",
                "hypothesis_met_not_cycle_but_a_representative_could", "cycle_without_hypothesis",
                "counterexamples", "counterexamples_genus_0", "core_sizes"):
        print(f"{key}: {summary[key]}")
    print(f"{len(instances)} instances, {rejected} rejected, {summary['seconds']} s, report in {out}")


if __name__ == "__main__":
    main()
