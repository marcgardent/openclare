#!/usr/bin/env python3
"""Markdown tables of the conflict-graph study, from the outputs of
analyse.py and supplement.py and the exported instances.

Usage: tables.py <results.json> <supplement.json> <instance directory>
"""

import json
import os
import sys

import numpy as np


def level(record, name):
    for entry in record["levels"]:
        if entry["level"] == name:
            return entry
    return None


def classes(entry):
    if entry is None:
        return "-"
    if not entry["pairs"]:
        return "empty"
    c = entry["classes"]
    names = [("bipartite", "Bip"), ("chordal", "Ch"), ("interval", "Int"), ("comparability", "Co"), ("cocomparability", "coCo"), ("perfect", "Perf")]
    out = [short for key, short in names if c.get(key)]
    if c.get("perfect") is False:
        hole = c.get("odd_hole")
        anti = c.get("odd_antihole")
        if hole:
            out.append(f"C{len(hole)} hole")
        if anti and len(anti) > 5:
            out.append(f"anti-C{len(anti)}")
    return ",".join(out) if out else "none"


def lp_mark(entry, key):
    lp = entry[key]
    opt = entry["optimum"]
    if opt is None:
        return f"{lp['value']:.0f}/?"
    if not lp["fractional"] and abs(lp["value"] - opt) < 1e-6:
        return "int"
    return f"{lp['value']:.0f} ({len(lp['fractional'])} frac)"


def ring_places(instance_path):
    with open(instance_path) as f:
        d = json.load(f)
    middles = np.array(d["arc_middles"])
    low, high = middles.min(axis=0), middles.max(axis=0)
    span = np.where(high - low > 0, high - low, 1)
    places = []
    for ring in d["rings"]:
        arcs = sorted({arc for row, _ in ring["walk"] for arc, _ in row} | {arc for arc, _ in ring["shift"]})
        points = (middles[arcs] - low) / span
        places.append((points.mean(axis=0), np.ptp(points, axis=0) if len(points) else np.zeros(3)))
    side = np.sqrt(d["quads"])
    return places, side


def main():
    results = json.load(open(sys.argv[1]))
    supplement = {r["instance"]: r for r in json.load(open(sys.argv[2]))} if os.path.exists(sys.argv[2]) else {}
    directory = sys.argv[3]

    print("### Per piece: sizes and the levels without walks\n")
    print("| piece | arcs | rings | closed by engine | S: forced / pairs / higher | B: forced / pairs / higher cores (sizes) |")
    print("|---|---|---|---|---|---|")
    for r in results:
        s, b = level(r, "S"), level(r, "B")
        if s is None:
            continue
        hs = [len(c) for c in b["higher_cores"]]
        print(f"| {r['instance']} | {r['arcs']} | {r['rings']} | {len(r['reference_closed'])} | {len(s['forced'])} / {len(s['pairs'])} / {len(s['higher_cores'])} | "
              f"{len(b['forced'])} / {len(b['pairs'])} / {len(hs)}{' (' + str(min(hs)) + '-' + str(max(hs)) + ')' if hs else ''} |")

    print("\n### Per piece: level W (box and walks), the conflict graph\n")
    print("| piece | forced | pairs | rings in conflicts | components | classes | higher cores (opt / probe) | V pairs | optimum (rings) | edge LP | clique LP | clique LP unit |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|")
    for r in results:
        w = level(r, "W")
        if w is None:
            continue
        sup = supplement.get(r["instance"])
        probe, vpairs = "-", "-"
        if sup:
            sw = level(sup, "W")
            sv = level(sup, "V")
            probe = f"{sw['probe_failures']}/20, sizes {sorted(len(c) for c in sw['probe_higher_cores'])}" if sw else "-"
            vpairs = f"{len(sv['forced'])}f {len(sv['pairs'])}p" if sv else "-"
        unit = w["lp_clique_unit"]
        unit_mark = "int" if not unit["fractional"] else f"{unit['value']:.1f} ({len(unit['fractional'])} frac)"
        print(f"| {r['instance']} | {len(w['forced'])} | {len(w['pairs'])} | {w['conflict_vertices']} | {w['components']} | {classes(w)} | "
              f"{[len(c) for c in w['higher_cores']]} / {probe} | {vpairs} | {w['optimum']:.0f} ({w['optimum_count']}) | {lp_mark(w, 'lp_edge_weight')} | {lp_mark(w, 'lp_clique_weight')} | {unit_mark} |")

    print("\n### Per piece: level Q (everything the solver poses), exact optimum and solvers\n")
    print("| piece | forced | pairs | higher cores | classes | optimum, real x (rings) | clique LP | clique+cores LP | big-M LP | MILP real x | MILP whole x | CPU: pairs + hitting | CPU: MILP real / whole |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for r in results:
        q = level(r, "Q")
        if q is None:
            continue
        mip = r["mip"]
        def mipv(m):
            if "value" not in m:
                return "none"
            if m["status"] == 0:
                return f"{m['value']:.0f}"
            return f"{m['value']:.0f} (bound {m.get('dual_bound', float('nan')):.0f})"
        hs = [len(c) for c in q["higher_cores"]]
        print(f"| {r['instance']} | {len(q['forced'])} | {len(q['pairs'])} | {len(hs)}{' (' + str(min(hs)) + '-' + str(max(hs)) + ')' if hs else ''} | {classes(q)} | "
              f"{q['optimum']:.0f} ({q['optimum_count']}) | {lp_mark(q, 'lp_clique_weight')} | {lp_mark(q, 'lp_clique_cores_weight')} | "
              f"{mip['lp_relaxation']['value']:.0f} ({mip['lp_relaxation']['z_fractional']} frac) | {mipv(mip['mip_real'])} | {mipv(mip['mip_whole'])} | "
              f"{q['pair_seconds']:.1f} + {q['hitting_seconds']:.1f} s | {mip['mip_real']['cpu_seconds']:.1f} / {mip['mip_whole']['cpu_seconds']:.1f} s |")

    if supplement:
        print("\n### Whole lengths at level W\n")
        print("| piece | forced real / whole | pairs real / whole | higher cores (whole) | optimum real / whole | perfect (whole) | MILPs (CPU s, unknown) |")
        print("|---|---|---|---|---|---|---|")
        for r in results:
            sup = supplement.get(r["instance"])
            if not sup or "whole_W" not in sup:
                continue
            w = level(r, "W")
            h = sup["whole_W"]
            opt = f"{h['optimum']:.0f}" if h["optimum"] is not None else "?"
            print(f"| {r['instance']} | {len(w['forced'])} / {len(h['forced'])} | {len(w['pairs'])} / {len(h['pairs'])} | {[len(c) for c in h['higher_cores']]} | "
                  f"{w['optimum']:.0f} / {opt} | {h['perfect']} | {h['milps']['count']} ({h['milps']['seconds']:.0f} s, {h['milps']['unknown']}) |")

    print("\n### Rings of the odd holes and antiholes at level W\n")
    for r in results:
        w = level(r, "W")
        if w is None or w["classes"].get("perfect") is not False:
            continue
        places, side = ring_places(os.path.join(directory, r["instance"] + ".json"))
        for kind in ("odd_hole", "odd_antihole"):
            cycle = w["classes"].get(kind)
            if not cycle:
                continue
            print(f"- {r['instance']} {kind.replace('_', ' ')} of {len(cycle)}: " + "; ".join(
                f"ring {k} (round {r['weights'][k]:.0f} = {r['weights'][k] / side:.1f} sides, at {np.round(places[k][0], 2).tolist()}, spread {np.round(places[k][1], 2).tolist()})" for k in cycle))


if __name__ == "__main__":
    main()
