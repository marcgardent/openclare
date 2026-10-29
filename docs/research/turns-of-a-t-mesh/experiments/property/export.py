"""Writes an instance in the format of the engine's export
(`OPENCLARE_EXPORT_TURNS`, `quantization/export.rs`): `call-<name>/way-0.json`
with the T-mesh, the turns walked at the real lengths from the middle of each
side, and the whole lengths; `way-0-helices.json` with every strip of the
whole lengths. `Check/convert.py` and `Check/Main.lean` then read it as they
read the engine's. Fields the check does not read (`spirals`, `scale`, the
positions) are filled plausibly.
"""

import json
import os
from fractions import Fraction

from tracing import node_halves


def expression(form):
    return [[arc, c] for arc, c in sorted(form.items())]


def number(x):
    """A Fraction as the float the check reads back exactly (lengths are
    whole or in eighths)."""
    f = float(x)
    assert Fraction(f) == x, x
    return f


def way_json(tmesh, whole, real, turns, held, shares, most_lean, longest, name):
    quad = tmesh.quad
    halves = node_halves(tmesh)
    nodes = []
    for n, v in enumerate(tmesh.nodes):
        singular = quad.valence[v] != 4
        nodes.append({
            "at": list(quad.positions[v]),
            "kind": "singularity" if singular else "junction",
            "vertex": v if singular else -1,
            "halves": halves[n],
            "angle": quad.valence[v],
        })
    arcs = []
    for a, arc in enumerate(tmesh.arcs):
        p0, p1 = (quad.positions[arc["edges"][0][0]], quad.positions[arc["edges"][-1][1]])
        arcs.append({
            "nodes": arc["nodes"],
            "length": number(real[a]),
            "patches": arc["patches"],
            "border": False,
            "feature": False,
            "trace": a,
            "span": [0.0, number(real[a])],
            "sides": arc["sides"],
        })
    patches = [[[half, side] for half, side in patch["rim"]] for patch in tmesh.patches]
    stations = []
    for patch in tmesh.patches:
        for pair in range(2):
            row = [[half // 2, 1 if side < 2 else -1] for half, side in patch["rim"] if side % 2 == pair]
            stations.append(row)
    arcs_at = []
    for arc in tmesh.arcs:
        a, b = quad.positions[arc["edges"][0][0]], quad.positions[arc["edges"][-1][1]]
        arcs_at.append([(a[c] + b[c]) / 2 for c in range(3)])
    quads = 0
    for patch in tmesh.patches:
        sides = [Fraction(0)] * 4
        for half, side in patch["rim"]:
            sides[side] += whole[half // 2]
        quads += sides[0] * sides[1]
    out_turns = []
    for k, turn in enumerate(turns):
        shift = turn["shift"]
        held_index = next((i for i, h in enumerate(held) if h == shift), -1)
        real_shift = sum((c * real[arc] for arc, c in shift.items()), Fraction(0))
        at = quad.positions[tmesh.arcs[tmesh.patches[turn["patch"]]["rim"][0][0] // 2]["edges"][0][0]]
        out_turns.append({
            "shift": expression(shift),
            "along": expression(turn["along"]),
            "around": number(turn["around"]),
            "lean": abs(float(real_shift)) / float(turn["around"]),
            "patch": turn["patch"],
            "at": list(at),
            "held": held_index >= 0,
            "held_index": held_index,
            "kept": held_index >= 0,
            "side": turn["side"],
            "offset": number(turn["offset"]),
            "crossings": [[p, half, nxt, number(into)] for (p, half, nxt, into) in turn["crossings"]],
        })
    return {
        "call": name,
        "way": 0,
        "offsets": [float(s) for s in shares],
        "most_lean": most_lean,
        "longest": longest,
        "scale": 1.0,
        "quads": int(quads),
        "spirals": -1.0,
        "real": [number(x) for x in real],
        "lengths": [int(x) for x in whole],
        "lower": [0] * len(tmesh.arcs),
        "stations": stations,
        "arcs_at": arcs_at,
        "nodes": nodes,
        "arcs": arcs,
        "patches": patches,
        "turns": out_turns,
    }


def helices_json(strips, whole):
    out = []
    for strip in strips:
        pitch = sum((c * whole[arc] for arc, c in strip["shift"].items()), Fraction(0))
        out.append({"shift": expression(strip["shift"]), "around": number(strip["around"]),
                    "pitch": int(pitch), "field": int(pitch)})
    return out


def write(directory, name, way, helices):
    folder = os.path.join(directory, f"call-{name}")
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "way-0.json"), "w") as f:
        json.dump(way, f, separators=(",", ":"))
    with open(os.path.join(folder, "way-0-helices.json"), "w") as f:
        json.dump(helices, f, separators=(",", ":"))
    return folder
