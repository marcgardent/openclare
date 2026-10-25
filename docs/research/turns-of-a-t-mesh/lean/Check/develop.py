"""Words of the turns in the loops round the cone points, for the check of the Fox–Burgers
formula (`Main.lean`, `Fox.hol`). Used by `convert.py`; Python standard library only.

At the whole lengths of the export (balanced), each patch is the rectangle of its frame
`(u, v)`, its rim from the first corner along `+u, +v, -u, -v` (engine sides 0 to 3). A
point is a Gaussian integer `(x, y)`; a motion `x ↦ Rᵏ x + t`, `R` the quarter-turn.

* **One chart.** A breadth-first spanning tree of the patches from patch 0 (across arcs)
  gives every patch `P` the motion `D_P` from its frame to the frame of patch 0. On a
  closed surface of genus zero the arcs it does not cross form a spanning tree `T` of the
  nodes, and the surface cut along `T` is a disc `Ω` with the chart `D`.
* **Jumps.** A loop based in `Ω` that crosses the arc of a half `h` of `T`, from the patch
  `P` holding `h` into the patch `Q` holding its twin, continues the chart `D_P` into `Q`:
  there the continued chart is `J_h ∘ D_Q` with `J_h = D_P ∘ c ∘ D_Q⁻¹`, `c` the change of
  frame from `Q` to `P`. The holonomy (the chart continued along the loop is `ĥ ∘ D`, so
  that `ĥ(αβ) = ĥ(α) ∘ ĥ(β)`, the convention of the paper) of a loop is the composite of
  the jumps of its crossings, the first crossing applied last.
* **Words.** Rooted at a singular node, each arc `f` of `T` leads away from the root to a
  node `v`. Its generator `G_f` is a jump across `f`, and the recursion of the proof of
  Proposition "The shift is set by the singularities" (`FirstVisit.lean`) is
  `G_f = G_{f_r} ∘ ⋯ ∘ G_{f_1} ∘ ρ_v`, `f_1, …, f_r` the arcs of `T` leaving `v` in the order
  of a walk round `T`, `ρ_v` the rotation by `R^{4 - k(v)}` about a developed corner `p_v`
  of `v` (the identity at a node that is not singular). Which jump is `G_f` and in which
  order the walk meets the arcs are not taken for granted: the conventions are tried, and
  one is kept only if at every node `ρ_v` is exactly a rotation by `R^{4 - k(v)}` about a
  corner of `v`, at the whole lengths and, within `10⁻⁶`, at the lengths the engine
  walked. Unfolding the recursion writes every `G_f` as a word in the letters `x_v`, so
  every turn's holonomy too.

Out of scope: surfaces of other genus (the arcs off the tree of patches are not a tree).
"""

from fractions import Fraction

DIRECTION = [(1, 0), (0, 1), (-1, 0), (0, -1)]


def rot(k, p):
    x, y = p
    for _ in range(k % 4):
        x, y = -y, x
    return (x, y)


def add(p, q):
    return (p[0] + q[0], p[1] + q[1])


def sub(p, q):
    return (p[0] - q[0], p[1] - q[1])


class Motion:
    """x ↦ Rᵏ x + t."""

    def __init__(self, k=0, t=(0, 0)):
        self.k, self.t = k % 4, t

    def after(self, other):
        return Motion(self.k + other.k, add(rot(self.k, other.t), self.t))

    def inverse(self):
        k = -self.k % 4
        return Motion(k, rot(k, (-self.t[0], -self.t[1])))

    def close(self, other, tol):
        return self.k == other.k and all(abs(a - b) <= tol for a, b in zip(self.t, other.t))


class Unsupported(Exception):
    pass


class Chart:
    def __init__(self, export, lengths, exact):
        self.export = export
        self.lengths = lengths
        self.tol = 0 if exact else 1e-6
        arcs, rims = export["arcs"], export["patches"]
        self.place = {}
        self.start = []
        for p, rim in enumerate(rims):
            at, starts = (0, 0), []
            for k, (half, side) in enumerate(rim):
                self.place[half] = (p, k)
                starts.append(at)
                d = DIRECTION[side]
                at = add(at, (d[0] * lengths[half // 2], d[1] * lengths[half // 2]))
            if any(abs(c) > self.tol for c in at):
                raise Unsupported(f"patch {p} does not close at these lengths")
            self.start.append(starts)
        # The charts, along a breadth-first tree of the patches.
        self.D = {0: Motion()}
        crossed, queue = set(), [0]
        while queue:
            p = queue.pop(0)
            for half, _ in rims[p]:
                q = self.place[half ^ 1][0]
                if q not in self.D:
                    self.D[q] = self.D[p].after(self.change(half ^ 1))
                    crossed.add(half // 2)
                    queue.append(q)
        self.tree = [a for a in range(len(arcs)) if a not in crossed]
        if len(self.tree) != len(export["nodes"]) - 1:
            raise Unsupported(f"{len(self.tree)} arcs off the tree of patches for {len(export['nodes'])} nodes: "
                              "not a sphere")

    def change(self, half):
        """The change of frame from the patch holding `half` to the patch holding its twin."""
        p, k = self.place[half]
        q, k2 = self.place[half ^ 1]
        rims = self.export["patches"]
        j = (rims[q][k2][1] + 2 - rims[p][k][1]) % 4
        a = self.start[p][k]
        b = self.start[q][(k2 + 1) % len(rims[q])]
        return Motion(j, sub(b, rot(j, a)))

    def jump(self, half):
        """J_h: crossing the arc of `half` from the patch holding it."""
        P, Q = self.place[half][0], self.place[half ^ 1][0]
        return self.D[P].after(self.change(half ^ 1)).after(self.D[Q].inverse())

    def corners(self, node):
        """The developed places of the corners of `node` in the chart of `Ω`."""
        if not hasattr(self, "_corners"):
            self._corners = {}
            for p, rim in enumerate(self.export["patches"]):
                for k, (half, _) in enumerate(rim):
                    v = self.export["arcs"][half // 2]["nodes"][half % 2]
                    self._corners.setdefault(v, []).append(self.D[p].after(Motion(0, self.start[p][k])).t)
        return self._corners.get(node, [])


def rotation_centre(X, k, tol):
    """p with X = (x ↦ Rᵏ (x - p) + p), or None."""
    if X.k != k % 4:
        return None
    if k % 4 == 0:
        return (0, 0) if all(abs(c) <= tol for c in X.t) else None
    # t = (1 - Rᵏ) p: p = t / (1 - Rᵏ) in the Gaussian rationals.
    a = rot(k, (1, 0))
    one_minus = (1 - a[0], -a[1])
    norm = one_minus[0] ** 2 + one_minus[1] ** 2
    t = X.t
    num = (t[0] * one_minus[0] + t[1] * one_minus[1], t[1] * one_minus[0] - t[0] * one_minus[1])
    if tol == 0:
        return (Fraction(num[0], norm), Fraction(num[1], norm))
    return (num[0] / norm, num[1] / norm)


class Words:
    def __init__(self, export, chart, convention):
        self.export, self.chart = export, chart
        arcs, nodes = export["arcs"], export["nodes"]
        twin, reverse, after = convention
        tree = set(chart.tree)
        adj = {v: [] for v in range(len(nodes))}
        for a in tree:
            n0, n1 = arcs[a]["nodes"]
            adj[n0].append(a)
            adj[n1].append(a)
        # The root: the first singular node.
        root = next(v for v, n in enumerate(nodes) if n["kind"] == "singularity")
        away = {root: None}
        order = [root]
        for v in order:
            for a in adj[v]:
                n0, n1 = arcs[a]["nodes"]
                w = n1 if n0 == v else n0
                if w not in away:
                    away[w] = 2 * a if n0 == v else 2 * a + 1
                    order.append(w)
        self.root, self.away, self.order = root, away, order
        self.places = {}
        self.word = {}
        self.ok = True
        self.failure = None
        for v in reversed(order):
            h = away[v]
            if h is None:
                continue
            halves = nodes[v]["halves"]
            back = h ^ 1
            i = halves.index(back)
            kids = [g for g in halves[i + 1:] + halves[:i] if g // 2 in tree]
            if reverse:
                kids = kids[::-1]
            comp, word = Motion(), []
            for g in kids:
                comp = self.generator(g, twin).after(comp)
                word = self.word[g] + word
            G = self.generator(h, twin)
            X = comp.inverse().after(G) if not after else G.after(comp.inverse())
            k = 4 - nodes[v]["angle"] if nodes[v]["kind"] == "singularity" else 0
            p = rotation_centre(X, k, chart.tol)
            if p is None or (k % 4 and not any(abs(p[0] - c[0]) <= chart.tol and abs(p[1] - c[1]) <= chart.tol
                                                for c in chart.corners(v))):
                self.ok = False
                self.failure = (v, X.k, k % 4)
                return
            if k % 4:
                self.places[v] = (int(p[0]), int(p[1])) if chart.tol == 0 else p
                word = word + [(v, 1)] if not after else [(v, 1)] + word
            self.word[h] = word

    def generator(self, h, twin):
        """G_f for the half `h` of `f` away from the root."""
        return self.chart.jump(h ^ 1 if twin else h)

    def jump_word(self, half, twin):
        """The word of the jump across the arc of `half` from the patch holding it."""
        h = half if self.away.get(self.head(half)) == half else half ^ 1
        w = self.word[h]
        # G_f is the jump from the patch of h ^ 1 (twin) or of h: the jump of `half` is G_f
        # or its inverse.
        same = (half == (h ^ 1)) if twin else (half == h)
        return w if same else [(v, -e) for v, e in reversed(w)]

    def head(self, half):
        return self.export["arcs"][half // 2]["nodes"][1 - half % 2]


def reduce(word):
    out = []
    for x in word:
        if out and out[-1][0] == x[0] and out[-1][1] == -x[1]:
            out.pop()
        else:
            out.append(x)
    return out


CONVENTIONS = [(twin, reverse, after) for twin in (True, False) for reverse in (False, True)
               for after in (False, True)]


def holonomy(export, walked):
    """The data of the Fox check, or raises Unsupported."""
    nodes = export["nodes"]
    whole = Chart(export, export["lengths"], exact=True)
    real = Chart(export, walked, exact=False)
    tried = []
    for convention in CONVENTIONS:
        W = Words(export, whole, convention)
        if W.ok and Words(export, real, convention).ok:
            break
        tried.append((convention, W.failure))
    else:
        raise Unsupported(f"no convention of the recursion holds: {tried}")
    twin = convention[0]
    tree = set(whole.tree)
    cones = sorted(W.places)
    index = {v: i for i, v in enumerate(cones)}
    turns = []
    for turn in export.get("turns", []):
        word = []
        for (_, half, _, _) in turn["crossings"]:
            if half // 2 in tree:
                word += W.jump_word(half, twin)
        turns.append({"word": [[index[v], e] for v, e in reduce(word)],
                      "chart": whole.D[turn["patch"]].k})
    jumps = []
    for a in sorted(tree):
        J = whole.jump(2 * a)
        jumps.append({"word": [[index[v], e] for v, e in reduce(W.jump_word(2 * a, twin))],
                      "motion": [J.k, J.t[0], J.t[1]]})
    return {
        "convention": {"generator": "G_f the jump from the patch holding the half of f towards the root"
                       if twin else "G_f the jump from the patch holding the half of f away from the root",
                       "children": "reversed" if convention[1] else "after the parent, as the node lists its halves",
                       "rotation": "applied last" if convention[2] else "applied first"},
        "tried": len(tried) + 1,
        "root": W.root,
        "cones": [{"node": v, "valence": nodes[v]["angle"], "at": list(W.places[v])} for v in cones],
        "turns": turns,
        "jumps": jumps,
    }
