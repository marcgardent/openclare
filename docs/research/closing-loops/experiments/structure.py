#!/usr/bin/env python3
"""Measures of structure the engine's selection does not use (DT-137), on the
quad meshes the bench stored; nothing is remeshed.

    python3 -I docs/research/closing-loops/experiments/structure.py \
        [--meshes bench/out] [--results bench/results/results.json] \
        [--target 3000] [--tools ...] [--models ...] [--out <json>]
    python3 -I docs/research/closing-loops/experiments/structure.py --table
    python3 -I docs/research/closing-loops/experiments/structure.py --check
    python3 -I docs/research/closing-loops/experiments/structure.py --mesh <obj>...

The engine keeps, among its candidates, the one with the fewest edges on
spiral loops (paper, Section "Measuring loops"), and the paper compares the
tools by that share. Three measures here are computed from the mesh alone,
by a separate implementation that shares no code with the engine:

(a) q-helices (Bommes, Lempfer, Kobbelt, "Global Structure Optimization of
    Quadrilateral Meshes", Eurographics 2011, Section 2.4), of every pitch.
    A q-helix is a dual parametric line (a strip of quads) along which
    walking tau steps is the same, in position and orientation, as h
    side-steps to the left; h is its pitch, tau its turn length. As in their
    Section 3, candidates are found where the orthogonal dual line through a
    quad meets the line through it again (the first such meeting), then
    extended in both directions as far as the property holds. Only minimal
    q-helices are kept: h <= tau, and the smallest h where several hold. A
    strip that meets itself only back at the quad it left (a ring round a
    torus) is no helix.
    The paper states that the property implies a regular region without
    singularity round the helix. That holds where the property holds over a
    whole turn (winding number gamma = l / tau >= 2, l the length in quads),
    so that the side-steps sweep the band between two windings; read at a
    single place, the property also holds wherever a strip comes back beside
    itself the same way round once, round singularities whose turns cancel,
    and such meetings are counted in the hundreds per mesh. So the q-helices
    reported (`q_helices`) are those of gamma >= 2 whose band is checked to
    be a regular grid (each side-step commutes with the step along the
    strip). That criterion is ours, grounded in the paper's remark on
    regular regions; it is not the paper's identification (its Section 3
    uses winding >= 2 only to protect helices from its operators, a choice
    the authors call arbitrary). The literal reading is kept as
    `any_winding`.
(b) The base complex after extraction: from each irregular vertex, a
    separatrix along each of its edges, straight on through regular
    vertices, until it reaches an irregular vertex, a border, or a vertex it
    has already passed (it then ends on itself: a T-junction). The patches
    are the regions of faces the separatrices enclose. The same count
    without stopping a separatrix on itself is the engine's
    `base_complex_patches`, given as a check.
(c) The singularities: inner vertices of valence other than four, by
    valence.

Each half-edge of a strip, each edge of a separatrix is counted once, so the
double counting of DT-262 (an edge on two loops of the engine's loop
following, on degenerate meshes) cannot occur here. An edge used by more than
two faces, or twice the same way round, has no twin: strips and separatrices
stop there as at a border. A border is, as in the engine's measure, an edge
used by one face.

Paths are relative: the mesh folder is `--meshes`, or `$MESHES`, or
`bench/out` of this clone; the results recorded are relative to it. numpy is
not needed; the script uses the standard library only and one thread.
"""

import argparse
import json
import os
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
MESHES = Path(os.environ.get("MESHES", ROOT / "bench" / "out"))
RESULTS = ROOT / "bench" / "results" / "results.json"
OUT = HERE / "results" / "structure.json"
TOOLS = ["openclare", "quadriflow", "autoremesher", "quadwild-bimdf", "instant-meshes-jakob", "instant-meshes"]
# The pitch the engine's spiral count sees up to (k_max of the paper).
SPIRAL_ORDER = 4
# Measures stored by the bench, copied beside ours.
STORED = [
    "valid",
    "quads",
    "faces",
    "loop_edges_spiral",
    "loop_edges_helix",
    "loops_helix",
    "loops_helix_missed",
    "helix_pitch_max",
    "base_complex_patches",
    "irregular_vertices",
]
NONE = -1


def read_obj(path):
    """The faces of an OBJ, as tuples of vertex indices from 0."""
    vertices, faces = 0, []
    with open(path) as lines:
        for line in lines:
            if line.startswith("v "):
                vertices += 1
            elif line.startswith("f "):
                corners = []
                for word in line.split()[1:]:
                    index = int(word.split("/")[0])
                    corners.append(index - 1 if index > 0 else vertices + index)
                faces.append(tuple(corners))
    return vertices, faces


class Mesh:
    """Half-edges of a polygon mesh: half-edge `base[f] + k` runs from corner
    k of face f to corner k + 1."""

    def __init__(self, vertex_count, faces):
        self.faces = faces
        self.base, self.origin, self.face = [], [], []
        for f, polygon in enumerate(faces):
            self.base.append(len(self.origin))
            for v in polygon:
                self.origin.append(v)
                self.face.append(f)
        halves = len(self.origin)
        self.next = [0] * halves
        for f, polygon in enumerate(faces):
            n = len(polygon)
            for k in range(n):
                self.next[self.base[f] + k] = self.base[f] + (k + 1) % n
        directed = {}
        uses = {}
        for h in range(halves):
            a, b = self.origin[h], self.origin[self.next[h]]
            directed.setdefault((a, b), []).append(h)
            key = (a, b) if a < b else (b, a)
            uses[key] = uses.get(key, 0) + 1
        self.twin = [NONE] * halves
        for h in range(halves):
            a, b = self.origin[h], self.origin[self.next[h]]
            key = (a, b) if a < b else (b, a)
            back = directed.get((b, a), [])
            if uses[key] == 2 and len(back) == 1 and len(directed[(a, b)]) == 1:
                self.twin[h] = back[0]
        self.valence = [0] * vertex_count
        self.on_border = [False] * vertex_count
        for (a, b), count in uses.items():
            self.valence[a] += 1
            self.valence[b] += 1
            if count == 1:
                self.on_border[a] = self.on_border[b] = True
        self.is_quad = [len(polygon) == 4 for polygon in faces]
        self.used = [count > 0 for count in self.valence]

    def head(self, h):
        return self.origin[self.next[h]]

    def irregular(self, v):
        """As the engine's measure: four edges inside, three on a border."""
        return self.valence[v] != (3 if self.on_border[v] else 4)


# --- (c) singularities ---------------------------------------------------------


def singularities(mesh):
    by_valence = {}
    for v, valence in enumerate(mesh.valence):
        if mesh.used[v] and not mesh.on_border[v] and valence != 4:
            by_valence[valence] = by_valence.get(valence, 0) + 1
    border = sum(1 for v in range(len(mesh.valence)) if mesh.used[v] and mesh.on_border[v] and mesh.irregular(v))
    return {
        "inner": sum(by_valence.values()),
        "by_valence": {str(k): by_valence[k] for k in sorted(by_valence)},
        "on_border": border,
    }


# --- (b) base complex ----------------------------------------------------------


def base_complex(mesh, stop_on_itself=True):
    """Patches and T-junctions of the separatrices traced from every irregular
    vertex (see the module's docstring)."""
    halves = len(mesh.origin)
    separatrix = [False] * halves
    t_junctions = 0
    for start in range(halves):
        v0 = mesh.origin[start]
        if not mesh.irregular(v0):
            continue
        passed = {v0}
        h = start
        for _ in range(halves):
            separatrix[h] = True
            if mesh.twin[h] != NONE:
                separatrix[mesh.twin[h]] = True
            v = mesh.head(h)
            if mesh.irregular(v) or mesh.on_border[v]:
                break
            if stop_on_itself and v in passed:
                t_junctions += 1
                break
            passed.add(v)
            across = mesh.twin[mesh.next[h]]
            if across == NONE:
                break
            h = mesh.next[across]
    patch = [NONE] * len(mesh.faces)
    patches = 0
    for seed in range(len(mesh.faces)):
        if patch[seed] != NONE:
            continue
        patch[seed] = patches
        stack = [seed]
        while stack:
            f = stack.pop()
            for k in range(len(mesh.faces[f])):
                h = mesh.base[f] + k
                twin = mesh.twin[h]
                if separatrix[h] or twin == NONE:
                    continue
                g = mesh.face[twin]
                if patch[g] == NONE:
                    patch[g] = patches
                    stack.append(g)
        patches += 1
    return patches, t_junctions


# --- (a) q-helices -------------------------------------------------------------
#
# A state (f, k) is a quad f crossed from its edge k to the opposite edge
# k + 2, encoded 4 f + k. Its reverse is (f, k + 2). One step forward leaves
# through edge k + 2 into the next quad. A side-step to the left crosses edge
# k + 3 (the face lies left of its half-edges): the orthogonal strip through f
# entered by edge k + 1 and left by edge k + 3. So the d-state h side-steps to
# the left of (f, k) is read on the orthogonal strip walked from (f, k + 1):
# at each quad (g, j) of that walk, the d-state is (g, j + 3).


def reverse(s):
    return (s & ~3) | ((s + 2) & 3)


def forward(mesh, s):
    f, k = s >> 2, s & 3
    t = mesh.twin[mesh.base[f] + (k + 2) % 4]
    if t == NONE or not mesh.is_quad[mesh.face[t]]:
        return None
    g = mesh.face[t]
    return 4 * g + (t - mesh.base[g])


def left(mesh, s):
    """The d-state one side-step to the left of `s`, or None."""
    t = forward(mesh, (s & ~3) | ((s + 1) & 3))
    return None if t is None else (t & ~3) | ((t + 3) & 3)


def column(mesh, s, pitch):
    states = [s]
    for _ in range(pitch):
        s = left(mesh, s)
        if s is None:
            return None
        states.append(s)
    return states


def regular_band(mesh, order, closed, start, run, pitch, turn):
    """Whether the band between two windings is a regular grid: from each of
    the `run` places from `start`, `pitch` side-steps on one side reach the
    place `turn` further along the strip, and each side-step commutes with the
    step along the strip, so no singularity lies inside."""
    n = len(order)
    places = [(start + i) % n for i in range(run)]
    if closed and run == n:
        places.append(start)
    for way in (1, -1):
        columns = []
        for place in places:
            s = order[place] if way == 1 else reverse(order[place])
            states = column(mesh, s, pitch)
            meeting = order[(place + turn) % n] if closed or place + turn < n else None
            if states is None or meeting is None or states[-1] != (meeting if way == 1 else reverse(meeting)):
                break
            columns.append(states)
        else:
            if all(
                all(forward(mesh, a) == b for a, b in (zip(here, after) if way == 1 else zip(after, here)))
                for here, after in zip(columns, columns[1:])
            ):
                return True
    return False


def strips(mesh):
    """Every dual parametric line: its states in order, whether it closes,
    and per state its strip, position and orientation (+1 the way the strip
    was listed, -1 against it). A state met again is not followed again."""
    states = 4 * len(mesh.faces)
    strip_of = [NONE] * states
    position = [0] * states
    orientation = [0] * states
    listed = []
    for f, quad in enumerate(mesh.is_quad):
        if not quad:
            continue
        for k in (0, 1):
            start = 4 * f + k
            if strip_of[start] != NONE:
                continue
            ahead, closed = [start], False
            seen = {start, reverse(start)}
            s = forward(mesh, start)
            while s is not None:
                if s == start:
                    closed = True
                    break
                if s in seen or strip_of[s] != NONE:
                    break
                ahead.append(s)
                seen.update((s, reverse(s)))
                s = forward(mesh, s)
            behind = []
            if not closed:
                s = forward(mesh, reverse(start))
                while s is not None and s not in seen and strip_of[s] == NONE:
                    behind.append(reverse(s))
                    seen.update((s, reverse(s)))
                    s = forward(mesh, s)
            order = behind[::-1] + ahead
            index = len(listed)
            for i, s in enumerate(order):
                strip_of[s] = strip_of[reverse(s)] = index
                position[s] = position[reverse(s)] = i
                orientation[s], orientation[reverse(s)] = 1, -1
            listed.append((order, closed))
    return listed, strip_of, position, orientation


def helices(mesh):
    """The minimal q-helices: per helix, its strip, pitch, turn length,
    length in quads and winding number."""
    listed, strip_of, position, orientation = strips(mesh)
    # Per strip and position: the minimal (pitch, turn) of the meetings with
    # a place further along.
    found = [[None] * len(order) for order, _ in listed]
    for order, closed in listed:
        for sign in (1, -1):
            walk = order if sign == 1 else [reverse(s) for s in reversed(order)]
            # The d-state beside each quad of this walk, to its left.
            beside = [(s & ~3) | ((s + 3) & 3) for s in walk]
            n = len(walk)
            # For each place, the next place along the walk whose d-state is
            # on the same strip: the first meeting. Twice round if it closes.
            nearest = [None] * n
            last = {}
            for i in range(2 * n - 1 if closed else n - 1, -1, -1):
                line = strip_of[beside[i % n]]
                if line == NONE:
                    continue
                if i < n and line in last:
                    nearest[i] = last[line]
                last[line] = i
            for i in range(n):
                j = nearest[i]
                if j is None or j - i >= n:
                    continue
                s, t = beside[i], beside[j % n]
                if s == t or orientation[s] != orientation[t]:
                    continue
                line = strip_of[s]
                length = len(listed[line][0])
                # The meeting is kept once, at the place from which the
                # other is `turn` further the way the strip is listed.
                ahead = position[t] - position[s]
                if listed[line][1]:
                    ahead %= length
                    base, turn = (position[s], ahead) if ahead <= length - ahead else (position[t], length - ahead)
                else:
                    base, turn = (position[s], ahead) if ahead > 0 else (position[t], -ahead)
                pitch = j - i
                if turn == 0 or pitch > turn:
                    continue
                cell = found[line][base]
                if cell is None or (pitch, turn) < cell:
                    found[line][base] = (pitch, turn)

    result = []
    for line, (order, closed) in enumerate(listed):
        cells = found[line]
        n = len(cells)
        if n == 0 or all(c is None for c in cells):
            continue
        if closed and all(c == cells[0] for c in cells):
            pitch, turn = cells[0]
            band = regular_band(mesh, order, closed, 0, n, pitch, turn)
            result.append(
                {"strip": line, "start": 0, "pitch": pitch, "turn": turn, "quads": n, "winding": n / turn, "band": band}
            )
            continue
        # Runs of equal (pitch, turn); a closed strip is read from a place
        # where the run changes, so no run is cut in two.
        first = 0
        if closed:
            first = next(i for i in range(n) if cells[i] != cells[i - 1])
        i = 0
        while i < n:
            cell = cells[(first + i) % n]
            j = i
            while j + 1 < n and cells[(first + j + 1) % n] == cell:
                j += 1
            if cell is not None:
                pitch, turn = cell
                run = j - i + 1
                quads = min(run + turn, n)
                start = (first + i) % n
                band = run >= turn and regular_band(mesh, order, closed, start, run, pitch, turn)
                result.append(
                    {
                        "strip": line,
                        "start": start,
                        "pitch": pitch,
                        "turn": turn,
                        "quads": quads,
                        "winding": quads / turn,
                        "band": band,
                    }
                )
            i = j + 1
    return result, [(len(order), closed) for order, closed in listed]


# --- one mesh ------------------------------------------------------------------


def measure_mesh(path):
    vertex_count, faces = read_obj(path)
    mesh = Mesh(vertex_count, faces)
    quads = sum(mesh.is_quad)
    found, lines = helices(mesh)
    patches, t_junctions = base_complex(mesh)
    patches_full, _ = base_complex(mesh, stop_on_itself=False)

    def summary(chosen):
        by_pitch = {}
        for helix in chosen:
            by_pitch[helix["pitch"]] = by_pitch.get(helix["pitch"], 0) + 1
        # Quads on helices, each (quad, direction) once, over all of them.
        return {
            "helices": len(chosen),
            "by_pitch": {str(k): by_pitch[k] for k in sorted(by_pitch)},
            "pitch_above_4": sum(1 for h in chosen if h["pitch"] > SPIRAL_ORDER),
            "pitch_max": max((h["pitch"] for h in chosen), default=0),
        }

    def share(chosen):
        """The share of the quads crossed by strips, each way across counted
        once, that lie on these helices."""
        on = set()
        for helix in chosen:
            length, closed = lines[helix["strip"]]
            for i in range(helix["start"], helix["start"] + helix["quads"]):
                if closed or i < length:
                    on.add((helix["strip"], i % length))
        return len(on) / max(sum(length for length, _ in lines), 1)

    two_turns = [h for h in found if h["winding"] >= 2]
    kept = [h for h in two_turns if h["band"]]
    return {
        "faces": len(faces),
        "quads": quads,
        "strips": len(lines),
        "q_helices": dict(summary(kept), share=share(kept)),
        "two_turns_band_not_regular": len(two_turns) - len(kept),
        "any_winding": dict(summary(found), share=share(found)),
        "base_complex": {"patches": patches, "t_junctions": t_junctions, "patches_without_stop": patches_full},
        "singularities": singularities(mesh),
        "list": [[h["pitch"], h["turn"], h["quads"], round(h["winding"], 3)] for h in kept],
    }


# --- hand-made cases -----------------------------------------------------------


def torus(n, m, shift):
    """An n x m grid on a torus whose last row is glued to the first shifted
    by `shift` quads: its strips across the rows are gcd(n, shift) helices
    of pitch gcd(n, shift) (one helix of pitch 1 when they are coprime)."""

    def vertex(i, j):
        if j == m:
            i, j = i + shift, 0
        return (i % n) + n * j

    faces = [(vertex(i, j), vertex(i + 1, j), vertex(i + 1, j + 1), vertex(i, j + 1)) for j in range(m) for i in range(n)]
    return n * m, faces


def cube(n):
    """An n x n grid on each side of a cube, faces outward: eight corners of
    valence three, six patches, no helix."""
    index = {}

    def vertex(p):
        return index.setdefault(p, len(index))

    faces = []
    for axis in range(3):
        u, w = (axis + 1) % 3, (axis + 2) % 3
        for side, flip in ((0, True), (n, False)):
            for a in range(n):
                for b in range(n):
                    corners = []
                    for da, db in ((0, 0), (1, 0), (1, 1), (0, 1)):
                        p = [0, 0, 0]
                        p[axis], p[u], p[w] = side, a + da, b + db
                        corners.append(vertex(tuple(p)))
                    faces.append(tuple(corners[::-1]) if flip else tuple(corners))
    return len(index), faces


def check():
    """The hand-made cases and what they must give."""
    cases = [
        ("torus 20 x 10, no shift", torus(20, 10, 0), {"helices": 0, "by_pitch": {}, "patches": 1, "t": 0}),
        ("torus 20 x 10, shift 1", torus(20, 10, 1), {"helices": 1, "by_pitch": {"1": 1}, "patches": 1, "t": 0}),
        ("torus 20 x 10, shift 4", torus(20, 10, 4), {"helices": 4, "by_pitch": {"4": 4}, "patches": 1, "t": 0}),
        ("torus 21 x 10, shift 7", torus(21, 10, 7), {"helices": 7, "by_pitch": {"7": 7}, "patches": 1, "t": 0}),
        ("cube 6 x 6 per side", cube(6), {"helices": 0, "by_pitch": {}, "patches": 6, "t": 0}),
    ]
    report, failed = [], 0
    for name, (vertex_count, faces), expected in cases:
        mesh = Mesh(vertex_count, faces)
        found, _ = helices(mesh)
        found = [h for h in found if h["winding"] >= 2 and h["band"]]
        by_pitch = {}
        for helix in found:
            by_pitch[str(helix["pitch"])] = by_pitch.get(str(helix["pitch"]), 0) + 1
        patches, t_junctions = base_complex(mesh)
        got = {"helices": len(found), "by_pitch": by_pitch, "patches": patches, "t": t_junctions}
        corners = singularities(mesh)
        ok = got == expected
        failed += not ok
        report.append({"case": name, "expected": expected, "got": got, "singularities": corners, "ok": ok})
        turns = sorted({(h["turn"], round(h["winding"], 2)) for h in found})
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got}, singularities {corners['by_valence']}, (turn, winding) {turns}")
    return report, failed


# --- the bench -----------------------------------------------------------------


def spread(values):
    values = sorted(v for v in values if v is not None)
    if not values:
        return None
    if len(values) == 1:
        return {"median": values[0], "q1": values[0], "q3": values[0], "models": 1}
    q1, _, q3 = statistics.quantiles(values, n=4, method="inclusive")
    return {"median": statistics.median(values), "q1": q1, "q3": q3, "models": len(values)}


def per_model_values(record):
    """The figures of one mesh that the summary takes the median of."""
    return {
        "q_helices": record["q_helices"]["helices"],
        "q_helix_share": record["q_helices"]["share"],
        "q_helices_pitch_above_4": record["q_helices"]["pitch_above_4"],
        "any_winding": record["any_winding"]["helices"],
        "any_winding_pitch_above_4": record["any_winding"]["pitch_above_4"],
        "patches": record["base_complex"]["patches"],
        "t_junctions": record["base_complex"]["t_junctions"],
        "singularities": record["singularities"]["inner"],
        "loop_edges_spiral": record["stored"].get("loop_edges_spiral"),
        "loop_edges_helix": record["stored"].get("loop_edges_helix"),
    }


def summarize(records, tools, models=None):
    """Per tool: the medians over the models (each model the median over
    its runs), with the quartiles, and the totals of helices."""
    summary = {}
    for tool in tools:
        mine = [r for r in records if r["tool"] == tool and (models is None or r["model"] in models)]
        if not mine:
            continue
        by_model = {}
        for record in mine:
            by_model.setdefault(record["model"], []).append(per_model_values(record))
        medians = {
            model: {
                key: (statistics.median(vs) if (vs := [v[key] for v in values if v[key] is not None]) else None)
                for key in values[0]
            }
            for model, values in by_model.items()
        }
        pitches = {}
        for record in mine:
            for pitch, count in record["q_helices"]["by_pitch"].items():
                pitches[pitch] = pitches.get(pitch, 0) + count
        all_helices = sum(r["q_helices"]["helices"] for r in mine)
        above = sum(r["q_helices"]["pitch_above_4"] for r in mine)
        loose = sum(r["any_winding"]["helices"] for r in mine)
        loose_above = sum(r["any_winding"]["pitch_above_4"] for r in mine)
        summary[tool] = {
            "models": len(by_model),
            "meshes": len(mine),
            "per_model_median": {key: spread([m[key] for m in medians.values()]) for key in medians[next(iter(medians))]},
            "q_helices_by_pitch": {k: pitches[k] for k in sorted(pitches, key=int)},
            "q_helices": all_helices,
            "q_helices_pitch_above_4": above,
            "meshes_with_pitch_above_4": sum(1 for r in mine if r["q_helices"]["pitch_above_4"]),
            "two_turns_band_not_regular": sum(r["two_turns_band_not_regular"] for r in mine),
            "any_winding": loose,
            "any_winding_pitch_above_4": loose_above,
            "engine_helices": sum(r["stored"].get("loops_helix") or 0 for r in mine),
            "engine_helices_pitch_above_4": sum(r["stored"].get("loops_helix_missed") or 0 for r in mine),
            "meshes_with_engine_helix_pitch_above_4": sum(1 for r in mine if r["stored"].get("loops_helix_missed")),
        }
    return summary


def bench(arguments):
    results = json.loads(Path(arguments.results).read_text())
    meshes = Path(arguments.meshes)
    records = []
    for model in sorted(results["runs"]):
        if arguments.models and model not in arguments.models:
            continue
        for tool in arguments.tools:
            for run in results["runs"][model].get(tool, []):
                stored = run.get("mesh")
                if not stored:
                    continue
                # The bench records `bench/out/<model>/<budget>/<file>`.
                relative = Path(*Path(stored).parts[2:])
                path = meshes / relative
                if not path.exists():
                    continue
                record = measure_mesh(path)
                metrics = run.get("metrics") or {}
                record.update(model=model, tool=tool, seed=run.get("seed"), mesh=str(relative))
                record["stored"] = {key: metrics.get(key) for key in STORED}
                records.append(record)
                print(
                    f"{model:12} {tool:21} seed {run.get('seed')}: q-helices {record['q_helices']['helices']}"
                    f" (pitch > 4: {record['q_helices']['pitch_above_4']}), any winding"
                    f" {record['any_winding']['helices']}, patches {record['base_complex']['patches']},"
                    f" T {record['base_complex']['t_junctions']}, singularities {record['singularities']['inner']}",
                    flush=True,
                )

    # The engine's own counts, measured again here, as a check of the port.
    checks = {"irregular_vertices": [0, 0], "base_complex_patches": [0, 0]}
    for record in records:
        stored = record["stored"]
        if stored.get("irregular_vertices") is not None:
            checks["irregular_vertices"][0] += 1
            checks["irregular_vertices"][1] += stored["irregular_vertices"] == record["singularities"]["inner"]
        if stored.get("base_complex_patches") is not None:
            checks["base_complex_patches"][0] += 1
            checks["base_complex_patches"][1] += (
                stored["base_complex_patches"] == record["base_complex"]["patches_without_stop"]
            )

    summary = summarize(records, arguments.tools)
    # The models every tool meshes, for a comparison on the same models.
    common = sorted(set.intersection(*({r["model"] for r in records if r["tool"] == t} for t in arguments.tools)))
    summary_common = {"models": common, "tools": summarize(records, arguments.tools, set(common))}
    return records, checks, summary, summary_common


def dumps(out):
    """The results as JSON, one line per mesh measured."""
    records = out.pop("meshes_measured")
    head = json.dumps(out, indent=1)[:-2]
    lines = ",\n  ".join(json.dumps(record, separators=(",", ":")) for record in records)
    out["meshes_measured"] = records
    return f'{head},\n "meshes_measured": [\n  {lines}\n ]\n}}\n'


def table(path):
    """The Markdown tables of the README, from a results file."""
    data = json.loads(Path(path).read_text())
    common = data["summary_on_common_models"]
    return (
        "All the models each tool meshes:\n\n"
        + rows(data["summary"])
        + f"\n\nThe {len(common['models'])} models every tool meshes ({', '.join(common['models'])}):\n\n"
        + rows(common["tools"])
    )


def rows(summary):

    def cell(entry, scale=1, digits=0):
        if entry is None:
            return "—"
        f = lambda v: f"{v * scale:.{digits}f}"
        return f"{f(entry['median'])} ({f(entry['q1'])}–{f(entry['q3'])})"

    lines = [
        "| Tool | Models (meshes) | Spiral edges, % | q-helices per mesh | q-helices of pitch > 4 | Meetings of any winding (pitch > 4) | Engine's helices (pitch > 4) | Base-complex patches | T-junctions | Singularities |",
        "|---|---|---|---|---|---|---|---|---|---|",
    ]
    for tool, entry in summary.items():
        median = entry["per_model_median"]
        lines.append(
            f"| {tool} | {entry['models']} ({entry['meshes']}) | {cell(median['loop_edges_spiral'], 100, 1)}"
            f" | {cell(median['q_helices'])} | {entry['q_helices_pitch_above_4']} of {entry['q_helices']}"
            f" | {entry['any_winding']} ({entry['any_winding_pitch_above_4']})"
            f" | {entry['engine_helices']} ({entry['engine_helices_pitch_above_4']})"
            f" | {cell(median['patches'])} | {cell(median['t_junctions'])} | {cell(median['singularities'])} |"
        )
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--meshes", default=str(MESHES), help="the bench's mesh folder (bench/out)")
    parser.add_argument("--results", default=str(RESULTS), help="the bench's results (stored measures)")
    parser.add_argument("--target", type=int, default=3000)
    parser.add_argument("--tools", nargs="+", default=TOOLS)
    parser.add_argument("--models", nargs="+")
    parser.add_argument("--out", default=str(OUT))
    parser.add_argument("--check", action="store_true", help="the hand-made cases only")
    parser.add_argument("--mesh", nargs="+", help="measure these OBJ files and print the result")
    parser.add_argument("--table", action="store_true", help="print the README's table from --out and stop")
    arguments = parser.parse_args()

    if arguments.table:
        print(table(arguments.out))
        return 0
    if arguments.mesh:
        for path in arguments.mesh:
            print(json.dumps(dict(measure_mesh(path), mesh=Path(path).name)))
        return 0
    cases, failed = check()
    if arguments.check:
        return 1 if failed else 0
    if failed:
        print("hand-made cases fail: no bench measured", file=sys.stderr)
        return 1
    results = json.loads(Path(arguments.results).read_text())
    if results.get("target") != arguments.target:
        print(f"the results are at {results.get('target')} quads, not {arguments.target}", file=sys.stderr)
        return 1
    records, checks, summary, summary_common = bench(arguments)
    out = {
        "experiment": "measures of structure the selection does not use (DT-137)",
        "results": "bench/results/results.json",
        "meshes": "bench/out/<model>/<budget>/<tool>_<seed>.obj",
        "target": arguments.target,
        "spiral_order": SPIRAL_ORDER,
        "hand_made_cases": cases,
        "checks_against_the_engine": {k: {"meshes": v[0], "equal": v[1]} for k, v in checks.items()},
        "summary": summary,
        "summary_on_common_models": summary_common,
        "meshes_measured": records,
    }
    Path(arguments.out).parent.mkdir(parents=True, exist_ok=True)
    Path(arguments.out).write_text(dumps(out))
    print(json.dumps(checks))
    return 0


if __name__ == "__main__":
    sys.exit(main())
