"""What the experiment scripts of both papers share about the systems the
engine exports: finding the way files of an export, reading one way, its
matrices, its linear programs, and the record of how a summary was made.

An export is the directory the engine writes with `OPENCLARE_EXPORT_TURNS=<dir>`
(ALGORITHM.md, M6, "Experiment options"): for each T-mesh `call-<name>.json`,
`call-<name>.txt`, and in `call-<name>/` a `way-<k>.json` and a
`way-<k>-helices.json` per way of closing. The export the papers' first
version used (the hook of branch `rd-exact`, engine before commit `c8bf2be`)
has the way files directly in the directory, without call files; both are
read. A model in several pieces has a T-mesh per piece: the one with the most
arcs is taken unless `--call` names another.

The exact arithmetic (Farkas certificates, rows of the balances, whole
lengths checked) is that of `../lean/Check/certify.py`, whose `Way` this
module extends: the scripts and the Lean check read the same numbers.

HiGHS is called through SciPy with `threads` set (by default 1, or
`$HIGHS_THREADS`): SciPy passes the option on verbatim, and every summary
records it with the versions of SciPy and HiGHS.
"""

import json
import os
import sys
import time
import warnings
from pathlib import Path

# One thread for the linear algebra too (set before NumPy is first imported:
# the scripts import this module first).
for _name in ("OPENBLAS_NUM_THREADS", "OMP_NUM_THREADS", "MKL_NUM_THREADS"):
    os.environ.setdefault(_name, "1")

import numpy as np  # noqa: E402
import scipy  # noqa: E402
from scipy.optimize import Bounds, LinearConstraint, linprog, milp  # noqa: E402
from scipy.sparse import coo_matrix, hstack, identity, vstack  # noqa: E402

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
sys.path.insert(0, str(HERE.parent / "lean" / "Check"))
sys.path.insert(0, str(ROOT / "docs" / "research" / "closing-loops" / "experiments"))
import certify  # noqa: E402
import common  # noqa: E402

DATA = ROOT / "scratch" / "data-package"
# Where export.py writes the export of the male body by default (the engine
# as it is; `male-body-before-cuts` with the stages of the papers' export).
EXPORT = DATA / "turns-of-a-t-mesh" / "male-body"
THREADS = int(os.environ.get("HIGHS_THREADS", "1"))


# --- HiGHS -------------------------------------------------------------------------------


def options(time_limit=None, **more):
    """Options for `milp`/`linprog`: the time limit, if any, and the number of
    threads, passed to HiGHS as they are."""
    out = {"threads": THREADS, **more}
    if time_limit is not None:
        out["time_limit"] = float(time_limit)
    return out


warnings.filterwarnings("ignore", message="Unrecognized options detected")


def solver():
    """The versions of the solver, for the record."""
    try:
        from scipy.optimize._highspy import _core
        highs = f"{_core.HIGHS_VERSION_MAJOR}.{_core.HIGHS_VERSION_MINOR}.{_core.HIGHS_VERSION_PATCH}"
    except (ImportError, AttributeError):
        highs = "unknown"
    return {"scipy": scipy.__version__, "highs": highs, "highs_threads": THREADS}


def run_milp(cost, constraints, bounds, integrality, time_limit=None, **more):
    """`milp` with the threads fixed; returns the result and the seconds it took."""
    clock = time.monotonic()
    result = milp(cost, constraints=constraints, bounds=bounds, integrality=integrality,
                  options=options(time_limit, **more))
    return result, time.monotonic() - clock


def run_linprog(cost, **arguments):
    return linprog(cost, method="highs", options=options(), **arguments)


def parallel(function, items, jobs):
    """`function` on each item, `jobs` processes at a time (each with its own
    HiGHS, of THREADS threads); the results in the order of the items."""
    items = list(items)
    if jobs <= 1 or len(items) <= 1:
        return [function(item) for item in items]
    import multiprocessing

    with multiprocessing.get_context("fork").Pool(min(jobs, len(items))) as pool:
        return pool.map(function, items, chunksize=1)


# --- export directories ----------------------------------------------------------------


def calls(directory):
    """The T-meshes of an export: {name: folder holding its way files}. The
    folder itself, named "", when it holds way files (the old layout)."""
    directory = Path(directory)
    if (directory / "export").is_dir() and not any(directory.glob("way-*.json")):
        directory = directory / "export"
    found = {}
    if any(directory.glob("way-[0-9]*.json")):
        found[""] = directory
    for folder in sorted(directory.glob("call-*")):
        if folder.is_dir() and any(folder.glob("way-[0-9]*.json")):
            found[folder.name] = folder
    return found


def call_arcs(name):
    """The number of arcs a call name gives (`call-<arcs>-<patches>-<length>`)."""
    try:
        return int(name.split("-")[1])
    except (IndexError, ValueError):
        return 0


def ways(directory, call=None):
    """The way files of one T-mesh of an export: (folder, {way: path}). With
    several T-meshes, the one `call` names, else the one with the most arcs."""
    found = calls(directory)
    if not found:
        raise SystemExit(f"no way-<k>.json in {directory} (an OPENCLARE_EXPORT_TURNS directory)")
    if call is not None:
        key = call if call.startswith("call-") else f"call-{call}"
        if key not in found:
            raise SystemExit(f"no T-mesh {key} in {directory}: {sorted(found)}")
    elif "" in found:
        key = ""
    else:
        key = max(found, key=call_arcs)
    folder = found[key]
    paths = {}
    for path in folder.glob("way-*.json"):
        stem = path.stem[len("way-"):]
        if stem.isdigit():
            paths[int(stem)] = path
    return folder, dict(sorted(paths.items()))


def arguments(parser, group, paper="turns-of-a-t-mesh"):
    """The options every script on an export takes."""
    parser.add_argument("export", nargs="?", type=Path, default=EXPORT,
                        help=f"the export directory (default {EXPORT.relative_to(ROOT)})")
    parser.add_argument("--call", help="the T-mesh, if the export has several (default: the one with the most arcs)")
    parser.add_argument("--ways", type=lambda text: [int(k) for k in text.split(",")],
                        help="only these ways, as 0,2,5 (default: all)")
    parser.add_argument("--out", type=Path, default=DATA / paper / group, help="where raw output goes")
    parser.add_argument("--jobs", type=int, default=1,
                        help="processes at a time, where the script splits its work (one HiGHS thread each)")
    parser.add_argument("--summary", type=Path,
                        default=ROOT / "docs" / "research" / paper / "experiments"
                        / ("exact" if paper == "closing-loops" else "") / "results" / f"{group}.json",
                        help="the summary written")


def load(arguments):
    """The ways asked for, read."""
    folder, paths = ways(arguments.export, arguments.call)
    asked = arguments.ways if arguments.ways is not None else list(paths)
    missing = [k for k in asked if k not in paths]
    if missing:
        raise SystemExit(f"no way {missing} in {folder}")
    arguments.out.mkdir(parents=True, exist_ok=True)
    return folder, [Way(paths[k]) for k in asked]


def record(arguments, folder, ways_read, started, clock, **more):
    """The head of a summary: the export read (its files' SHA-256), the
    machine, the solver, the time."""
    try:
        export = str(Path(folder).resolve().relative_to(ROOT))
    except ValueError:
        export = Path(folder).name
    return {
        "export": export,
        "export_record": export_record(folder),
        "ways": {way.number: way.sha for way in ways_read},
        "machine": {**started, "load_average_at_end": [round(x, 2) for x in os.getloadavg()],
                    "processors_allowed": len(os.sched_getaffinity(0))},
        "solver": solver(),
        "wall_seconds": round(time.monotonic() - clock, 1),
        **more,
    }


def export_record(folder):
    """What export.py wrote of how the export was made, if it did."""
    for candidate in (Path(folder), Path(folder).parent, Path(folder).parent.parent):
        path = candidate / "record.json"
        if path.exists():
            return json.loads(path.read_text())
    return None


def sha256(path):
    import hashlib

    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write(path, data):
    common.write_json(Path(path), data)
    try:
        shown = Path(path).resolve().relative_to(ROOT)
    except ValueError:
        shown = path
    print(f"summary: {shown}")


# --- one way ---------------------------------------------------------------------------------


def matrix(forms, n):
    """The rows of `forms` ({arc: coefficient} or [[arc, coefficient]]) as a sparse matrix."""
    data, rows, columns = [], [], []
    for i, form in enumerate(forms):
        for arc, coefficient in (form.items() if isinstance(form, dict) else form):
            rows.append(i)
            columns.append(int(arc))
            data.append(float(coefficient))
    return coo_matrix((data, (rows, columns)), shape=(len(forms), n)).tocsr()


def form(expression):
    """An expression of the export ([[arc, coefficient]]) as {arc: whole coefficient}."""
    out = {}
    for arc, coefficient in expression:
        out[int(arc)] = out.get(int(arc), 0) + int(round(coefficient))
    return {arc: c for arc, c in out.items() if c}


def key(shift):
    """A form up to sign, to tell the same turn found twice."""
    items = tuple(sorted(shift.items()))
    negated = tuple(sorted((a, -c) for a, c in shift.items()))
    return min(items, negated)


class Way(certify.Way):
    """One way of closing of an export. Turns are numbered by their place in
    the way file; `held` lists the held ones in that order and
    `held_index[h]` gives the place of the turn the engine held `h`-th (the
    numbers of the papers' tables). Lengths: `target`, the real lengths at
    the way's scale (what the engine's deviation is measured from); `engine`,
    the way's whole lengths; `b`, the bounds of separation."""

    def __init__(self, path):
        super().__init__(path)
        self.path = Path(path)
        self.number = int(self.path.stem[len("way-"):])
        with open(path) as data:
            self.data = json.load(data)
        self.scale = float(self.data["scale"])
        self.real = np.asarray(self.data["real"], float)
        self.target = self.scale * self.real
        self.lower = np.asarray(self.b, float)
        self.lengths = np.asarray(self.engine, dtype=np.int64)
        self.turns = self.data["turns"]
        self.kept = [k for k, turn in enumerate(self.turns) if turn["kept"]]
        self.number_of = {k: h for h, k in self.held_index.items()}
        self._helices = None
        rows = self.data["stations"]
        # Each patch has two balances, one per pair of opposite sides; the
        # positive part of a balance is one side.
        self.side_u = matrix([{a: 1 for a, c in rows[k] if c > 0} for k in range(0, len(rows), 2)], self.n)
        self.side_v = matrix([{a: 1 for a, c in rows[k + 1] if c > 0} for k in range(0, len(rows), 2)], self.n)

    @property
    def helices(self):
        """The strips of quads of the way's whole lengths (`way-<k>-helices.json`)."""
        if self._helices is None:
            path = self.path.with_name(f"way-{self.number}-helices.json")
            self._helices = json.loads(path.read_text()) if path.exists() else None
        return self._helices

    def value(self, shift, x):
        return float(sum(c * x[a] for a, c in shift.items()))

    def quads(self, x):
        """The number of quads of lengths `x`: each patch is the product of its
        two sides (exact for whole lengths)."""
        x = np.asarray(x)
        if x.dtype.kind in "iu":
            return int(((self.side_u.astype(np.int64) @ x) * (self.side_v.astype(np.int64) @ x)).sum())
        return float(((self.side_u @ x) * (self.side_v @ x)).sum())

    def quads_gradient(self, x):
        return self.side_u.T @ (self.side_v @ x) + self.side_v.T @ (self.side_u @ x)

    def deviation(self, x):
        """The engine's deviation: the mean |x - target| over the arcs."""
        return float(np.abs(np.asarray(x, float) - self.target).mean())

    def weighted_deviation(self, x):
        """Σ |x - t| / (t + 1), the objective of the papers' first programs."""
        return float((np.abs(np.asarray(x, float) - self.target) / (self.target + 1.0)).sum())

    def lean(self, k):
        """How far turn `k` leans: its real shift over its length round."""
        turn = self.turns[k]
        if "lean" in turn:
            return float(turn["lean"])
        return abs(self.value(self.shift[k], self.real)) / max(turn["around"], 1e-12)

    def strips(self, most_lean=0.12):
        """The ways round of the strips of the whole lengths, as forms, with
        how far each leans at the target and its pitch; `most_lean` keeps
        those leaning less (None: all)."""
        out = []
        for strip in self.helices or []:
            shift = form(strip["shift"])
            lean = abs(self.value(shift, self.target)) / max(strip["around"], 1e-12)
            if most_lean is None or lean <= most_lean:
                out.append({"shift": shift, "around": strip["around"], "pitch": strip["pitch"], "lean": lean})
        return out

    def shifted(self, forms, x, tolerance=0.5):
        """How many of `forms` lengths `x` leave shifted."""
        return sum(1 for shift in forms if abs(self.value(shift, x)) > tolerance)

    def closable(self, forms, lower=None, upper=None):
        """Real lengths with the balances, `lower ≤ ℓ ≤ upper` and every form
        of `forms` zero, or None."""
        lower = self.lower if lower is None else np.asarray(lower, float)
        A = vstack([self.B, matrix(forms, self.n)]).tocsr() if forms else self.B
        r = run_linprog(np.zeros(self.n), A_eq=A, b_eq=np.zeros(A.shape[0]),
                        bounds=list(zip(lower, [None] * self.n if upper is None else upper)))
        return r.x if r.status == 0 else None

    def nearest(self, forms, lower=None, weighted=True, time_limit=None, integer=False, extra=()):
        """The lengths nearest the target closing `forms`, in the weighted
        (1/(t+1)) or plain L1 sense; `extra` adds rows to the program over
        [x, p, m] (x = t + p - m). Returns (result, x or None, seconds)."""
        n = self.n
        lower = self.lower if lower is None else np.asarray(lower, float)
        weight = 1.0 / (self.target + 1.0) if weighted else np.ones(n)
        equal = vstack([self.B, matrix(forms, n)]).tocsr() if forms else self.B
        eye = identity(n)
        constraints = [LinearConstraint(hstack([equal, coo_matrix((equal.shape[0], 2 * n))]).tocsr(), 0, 0),
                       LinearConstraint(hstack([eye, -eye, eye]).tocsr(), self.target, self.target), *extra]
        bounds = Bounds(np.concatenate([lower, np.zeros(2 * n)]), np.full(3 * n, np.inf))
        integrality = np.concatenate([np.full(n, 1 if integer else 0), np.zeros(2 * n)])
        result, seconds = run_milp(np.concatenate([np.zeros(n), weight, weight]), constraints, bounds,
                                   integrality, time_limit)
        return result, (None if result.x is None else result.x[:n]), seconds

    def whole(self, turns, seconds=0.0):
        """Whole lengths in P (balances, ℓ ≥ b) closing the turns at the places
        `turns`, checked in exact arithmetic, and how they were found: as real
        lengths (a vertex of the linear program, read as fractions) times
        their common denominator (Remark "Whole solutions exist at some
        size"), or, with `seconds`, first by HiGHS within that time. None if
        none closes."""
        import math
        from fractions import Fraction

        n = self.n
        A = vstack([self.B, self.H(turns)]).tocsr() if turns else self.B
        if seconds > 0:
            result, _ = run_milp(np.ones(n), [LinearConstraint(A, 0, 0)], Bounds(self.lower, np.inf), np.ones(n),
                                 seconds)
            if result.x is not None:
                x = [int(round(v)) for v in result.x]
                if self.closes(turns, x):
                    return x, "solver"
        r = run_linprog(np.ones(n), A_eq=A, b_eq=np.zeros(A.shape[0]), bounds=list(zip(self.lower, [None] * n)))
        if r.status != 0:
            return None, None
        candidates = [[Fraction(float(t)).limit_denominator(q) for t in r.x] for q in (12, 60, 1000, 10 ** 6)]
        for fx in candidates + [self.vertex(turns, r.x)]:
            if fx is None:
                continue
            denominator = math.lcm(*(t.denominator for t in fx))
            x = [int(t * denominator) for t in fx]
            if self.closes(turns, x):
                return x, "denominator"
        return None, None

    def most_closable(self, forms, lower=None, time_limit=None, integer=False, extra_lower=None):
        """The most of `forms` closable together within the bounds: one binary
        per form, big M (the papers' program). Returns (chosen places,
        result, seconds)."""
        n, m = self.n, len(forms)
        lower = self.lower if lower is None else np.asarray(lower, float)
        big = 1000.0 + 4.0 * float(np.sum(self.target)) ** 0.5
        shifts = matrix(forms, n)
        z = identity(m) * big
        constraints = [
            LinearConstraint(hstack([self.B, coo_matrix((self.B.shape[0], m))]).tocsr(), 0, 0),
            LinearConstraint(hstack([shifts, z]).tocsr(), -np.inf, big),
            LinearConstraint(hstack([-shifts, z]).tocsr(), -np.inf, big),
        ]
        bounds = Bounds(np.concatenate([lower, np.zeros(m)]), np.concatenate([np.full(n, np.inf), np.ones(m)]))
        integrality = np.concatenate([np.full(n, 1 if integer else 0), np.ones(m)])
        result, seconds = run_milp(np.concatenate([np.zeros(n), -np.ones(m)]), constraints, bounds, integrality,
                                   time_limit)
        chosen = [] if result.x is None else [k for k in range(m) if result.x[n + k] > 0.5]
        return chosen, result, seconds


def held_forms(way):
    """The shifts of the held turns, in the order the engine held them."""
    return [way.shift[way.held_index[h]] for h in sorted(way.held_index)]


def lengths_text(x, scale=None):
    """Lengths as `OPENCLARE_LENGTHS` reads them in `call-<name>.txt`: the scale
    on the first line (empty for the plain search's), then one per arc."""
    head = "" if scale is None else repr(float(scale))
    return head + "\n" + "\n".join(str(int(v)) for v in x) + "\n"
