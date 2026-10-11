#!/usr/bin/env python3
"""Runs every remesher on every model of the corpus and tabulates the metrics.

    python3 bench/run.py [--models a,b] [--tools a,b] [--target N] [--seeds 1,2,3]
    python3 bench/run.py --source MODEL

A model is a scene, a .blend that holds the object to remesh and the settings
of the add-on (bench/scenes; see bench/make_scene.py). `openclare` runs on it
through the add-on, in Blender; the other tools are given the same object as
the add-on sees it, exported as an OBJ. `--source` prints where that OBJ is,
after making it if need be.

Needs `blender` on the PATH, a release build of `openclare` and the add-on
built from it (`cargo build --release`, then `make addon`). Results go to
bench/results/.
"""

import argparse
import json
import resource
import statistics
import subprocess
import sys
from pathlib import Path

BENCH = Path(__file__).resolve().parent
ROOT = BENCH.parent
OPENCLARE = ROOT / "target" / "release" / "openclare"
# The scenes: what the bench runs on.
SCENES = BENCH / "scenes"
# The models that are a plain mesh, as OBJ files: what bench/make_scenes.py
# makes their scenes from.
CORPUS = BENCH / "corpus"
OUTPUT = BENCH / "out"
RESULTS = BENCH / "results"
# Outputs of tools that cannot be re-run at will (a licence that expires).
REFERENCE = BENCH / "reference"
# Models whose licence does not allow passing them on: they, their scenes
# (in `scenes` there) and what is remeshed from them stay in this directory,
# which is not tracked. See CREDITS.md for where each one comes from and how
# to fetch it again.
LOCAL = BENCH / "corpus-large"
# Those that come from BlenderKit, by their `asset_base_id`: `make blenderkit`
# (bench/fetch_blenderkit.py) downloads them again and makes their scenes.
BLENDERKIT = {
    "terrain": "43a14c49-93bb-49e7-baec-6397eae071c5",
    "mahaveer": "b6564fca-81c7-4873-945a-287fe143b7e6",
    "marine": "22029966-6ee8-48d1-8d6b-d962162e09fa",
    "f16": "2b0e1d40-8487-4b92-8594-6141f2ec6162",
    "camera": "3ef12fa4-29a0-463f-b338-7ad3f81b654b",
    "wolf": "c63283ec-7419-4bff-8f6f-13cf70bd1b14",
}
LOCAL_MODELS = set(BLENDERKIT)
# Scenes made by hand with bench/make_scene.py from files of the author of
# the project: an object with its modifiers and materials, curves drawn on
# it, a painted density. No OBJ makes them again.
AUTHORED = {"venturi", "ogre_hp"}

# name, category, has hard edges, has open boundaries
MODELS = [
    ("blob", "organic", False, False),
    ("rock", "organic", False, False),
    ("creature", "organic", False, False),
    ("cage", "organic", False, False),
    ("suzanne", "organic", False, True),
    # The project mascot: a real 4.7 million triangle sculpt, brought down to
    # 200 000 triangles by `openclare preprocess` (see bench/README.md).
    ("ogre", "organic", False, False),
    # From Keenan Crane's model repository (see bench/import_crane.py).
    ("spot", "organic", False, False),
    ("bob", "organic", False, False),
    ("blub", "organic", False, False),
    ("pig", "organic", False, False),
    ("jerry", "organic", False, True),
    ("rose", "organic", False, False),
    ("bridge", "organic", False, True),
    ("part", "mechanical", True, False),
    ("cube", "mechanical", True, False),
    # An edge that turns inwards, and hard edges that are closed curves.
    ("lblock", "mechanical", True, False),
    ("cylinder", "mechanical", True, False),
    # A car by the author of the project, modelled rather than sculpted, as
    # it is in its file: half a body mirrored, subdivided, its edges split
    # for shading and a boolean cut into it, in thirteen materials whose
    # borders the quad mesh is to follow. A closed surface as it is meant;
    # as it is seen, torn along its sharp edges (15,110 border edges) and
    # with 334 edges of three faces or more.
    ("venturi", "extra", False, False),
    # The ogre before it was brought down: the sculpt of 4.7 million
    # triangles, with three guide curves and a sharp one drawn on it, and a
    # painted density.
    ("ogre_hp", "extra", False, False),
    # Not redistributable (see LOCAL_MODELS): measured, but kept out of the
    # acceptance groups, which anyone must be able to reproduce.
    # A landscape from BlenderKit: half a million triangles, open, rugged.
    ("terrain", "extra", False, True),
    # Two game characters from BlenderKit, made of many separate pieces.
    ("mahaveer", "extra", False, True),
    ("marine", "extra", False, True),
    # Two hard-surface models from BlenderKit, assembled from many parts: a
    # fighter jet and a film camera.
    ("f16", "extra", True, True),
    ("camera", "extra", True, True),
    # A stylized game character from BlenderKit, low-poly and rigged.
    ("wolf", "extra", False, True),
    ("sphere", "calibration", False, False),
    ("torus", "calibration", False, False),
]
PRIMITIVES = {"cube", "sphere", "torus", "lblock", "cylinder"}
DEFAULT_TARGET = 3000
CREASE_ANGLE = "30"

# metric key, label, format, lower is better
METRICS = [
    ("faces", "Quads made", "{:.0f}", True),
    ("irregular_vertices", "Irregular vertices (S)", "{:.0f}", True),
    ("irregular_excess", "… of which above the minimum", "{:.0f}", True),
    ("base_complex_patches", "Base complex patches (C)", "{:.0f}", True),
    ("alignment", "Alignment, degrees (A)", "{:.2f}", True),
    ("angle_deviation_mean", "Mean angle deviation, degrees (F)", "{:.2f}", True),
    ("angle_deviation_p95", "Angle deviation, 95th percentile (F)", "{:.2f}", True),
    ("aspect_mean", "Mean aspect ratio (F)", "{:.3f}", True),
    ("area_variation", "Area variation (F)", "{:.3f}", True),
    ("hausdorff", "Hausdorff, ‰ of the diagonal (H)", "{:.2f}", True),
    ("mean_distance", "Mean distance, ‰ (H)", "{:.3f}", True),
    ("thin_distance", "Thin parts: mean distance, % of a quad edge (V)", "{:.1f}", True),
    ("volume_ratio", "Volume kept, % of the source (V)", "{:.2f}", False),
    ("budget_error", "Off budget, % (N)", "{:.1f}", True),
    ("seconds", "Time, seconds (T)", "{:.2f}", True),
]
SCALE = {"hausdorff": 1000.0, "mean_distance": 1000.0, "budget_error": 100.0, "thin_distance": 100.0, "volume_ratio": 100.0}
# The metrics the acceptance rule is about.
QUALITY = ["irregular_vertices", "base_complex_patches", "alignment", "angle_deviation_mean", "aspect_mean"]


# The most memory a process the bench starts may ask for, in bytes of address
# space: beyond it the process is refused and fails, and its run is reported
# as out of memory. Without it a run that goes wrong can take all the memory
# of the machine, and everything else running on it down with it (it
# happened: 53 GB on the car, on 9 October 2026).
MEMORY_CAP = 12_000_000 * 1024
# The models that need more room, each with its own cap: the sculpt of 4.7
# million triangles.
MEMORY_CAPS = {"ogre_hp": 24_000_000 * 1024}
# What a process says when it is refused memory: the engine (Rust), Python
# and numpy, Blender, C++, and the launcher of a Blender packaged as a snap.
OUT_OF_MEMORY = ("memory allocation of", "MemoryError", "Unable to allocate", "returns null", "bad_alloc",
                 "Cannot allocate memory", "cannot allocate memory", "pthread_create failed", "out of memory")


def memory_cap(model):
    return MEMORY_CAPS.get(model, MEMORY_CAP)


def run(command, timeout=1800, memory=MEMORY_CAP):
    """Runs a command to its end, with at most `memory` bytes of address
    space for it and what it starts."""

    def capped():
        # Never above a cap this process is itself under: that is refused.
        _, hard = resource.getrlimit(resource.RLIMIT_AS)
        limit = memory if hard == resource.RLIM_INFINITY else min(memory, hard)
        resource.setrlimit(resource.RLIMIT_AS, (limit, limit))
        # And no image of its memory kept by the system when it dies of it.
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))

    command = [str(c) for c in command]
    try:
        return subprocess.run(command, capture_output=True, text=True, timeout=timeout, preexec_fn=capped)
    except subprocess.TimeoutExpired as late:
        # Stopped, and a failure of that run like any other: with what it
        # had said so far.
        said = [text.decode(errors="replace") if isinstance(text, bytes) else text or ""
                for text in (late.stdout, late.stderr)]
        return subprocess.CompletedProcess(command, 1, said[0], f"{TIMED_OUT} after {timeout} s\n{said[1]}")


TIMED_OUT = "timed out"


def failure_of(done, otherwise):
    """Why a process that gave nothing failed: it took too long; or it ran
    out of memory, if it says so or if it was killed, which is how most end
    at the cap; `otherwise` if none of these."""
    if done.stderr.startswith(TIMED_OUT):
        return done.stderr.splitlines()[0]
    if any(sign in done.stdout or sign in done.stderr for sign in OUT_OF_MEMORY):
        return "out of memory"
    if done.returncode < 0:
        return f"killed by signal {-done.returncode}: out of memory, or a crash"
    return otherwise


def scene_path(model):
    """The scene of a model: the .blend the bench runs it from."""
    return (LOCAL / "scenes" if model in LOCAL_MODELS else SCENES) / f"{model}.blend"


def obj_path(model):
    """The OBJ the scene of a model is made from, if it is a plain mesh."""
    return CORPUS / f"{model}.obj"


def in_blender(model, directory, *arguments):
    """Runs bench/blender_run.py on the scene of a model, its files going to
    `directory`. Returns what it says: a dictionary whose "status" is "ok",
    or "error" with a "message"."""
    memory = memory_cap(model)
    done = run(["blender", "-b", "--factory-startup", scene_path(model), "--python-exit-code", 1,
                "--python", BENCH / "blender_run.py", "--", "--out", directory,
                "--memory", memory // 1024, *arguments], memory=memory)
    line = next((l for l in done.stdout.splitlines() if l.startswith("OPENCLARE ")), None)
    if line is None:
        return {"status": "error", "message": failure_of(done, "no output from Blender")}
    return json.loads(line.removeprefix("OPENCLARE "))


def source_path(model):
    """The model as the add-on sees it, modifiers applied, in triangles: the
    OBJ every result is measured against, and what the tools that do not
    read a scene are given. Exported from the scene into bench/out, and
    again when the scene is newer."""
    path, scene = OUTPUT / model / "source.obj", scene_path(model)
    if not path.exists() or path.stat().st_mtime < scene.stat().st_mtime:
        said = in_blender(model, path.parent, "--no-remesh")
        if said["status"] != "ok":
            sys.exit(f"{model}: {said['message']}")
    return path


def available_models():
    """Every model of the corpus, less those this machine has no scene of
    and nothing to make one from: the local ones not fetched, and those made
    by hand."""

    def can_be_made(name):
        if name in AUTHORED:
            return False
        return name not in LOCAL_MODELS or (LOCAL / "blenderkit" / f"{name}.blend").exists()

    return [name for name, *_ in MODELS if scene_path(name).exists() or can_be_made(name)]


def ensure_corpus(models):
    """Makes the scenes that are missing, and the OBJ files they are made
    from if those are missing too."""
    missing = [model for model in models if not scene_path(model).exists() and model not in AUTHORED]
    plain = [model for model in missing if model not in LOCAL_MODELS]
    if plain:
        CORPUS.mkdir(exist_ok=True)
        for model in plain:
            if model in PRIMITIVES and not obj_path(model).exists():
                run([OPENCLARE, "primitive", model, obj_path(model)])
        if any(not obj_path(model).exists() for model in plain):
            run(["blender", "-b", "--factory-startup", "--python", BENCH / "make_corpus.py", "--", CORPUS])
    if missing:
        run(["blender", "-b", "--factory-startup", "--python", BENCH / "make_scenes.py", "--",
             "--models", ",".join(missing)])
    missing = [model for model in models if not scene_path(model).exists()]
    if missing:
        sys.exit(f"missing scenes of the corpus: {', '.join(missing)}")


def quadriflow(model, sharp, boundary, target, seeds, directory):
    """Returns {seed: (path or None, seconds, failure reason)}."""
    prefix = directory / "quadriflow"
    done = run(["blender", "-b", "--factory-startup", "--python", BENCH / "quadriflow.py", "--", source_path(model),
                prefix, target, int(sharp), int(boundary), *seeds], memory=memory_cap(model))
    runs = {}
    for line in done.stdout.splitlines():
        if line.startswith("QUADRIFLOW "):
            _, seed, status, seconds = line.split()
            path = Path(f"{prefix}_{seed}.obj")
            runs[int(seed)] = (path if status == "ok" else None, float(seconds), None if status == "ok" else status)
    for seed in seeds:
        runs.setdefault(seed, (None, 0.0, failure_of(done, "no output from Blender")))
    return runs


def quadremesher_reference(model, target, uniform):
    setting = "uniform" if uniform else "default"
    kept = LOCAL / "reference" if model in LOCAL_MODELS else REFERENCE
    return kept / "quadremesher" / f"{model}__{target}__{setting}.obj"


def quadremesher(uniform, refresh=False):
    """Quad Remesher is deterministic (no seed) and needs a licence, so each
    result is computed once and kept in bench/reference; later runs read it
    back, with or without the software installed."""

    def tool(model, sharp, boundary, target, seeds, directory):
        path = quadremesher_reference(model, target, uniform)
        if path.exists() and not refresh:
            return {seeds[0]: (path, None, None)}
        path.parent.mkdir(parents=True, exist_ok=True)
        fresh = directory / path.name
        fresh.unlink(missing_ok=True)
        done = run(["blender", "-b", "--factory-startup", "--python", BENCH / "quadremesher.py", "--",
                    source_path(model), fresh, target, int(uniform), int(sharp)], memory=memory_cap(model))
        line = next((l for l in done.stdout.splitlines() if l.startswith("QUADREMESHER ")), None)
        if line is None:
            return {seeds[0]: (None, None, failure_of(done, "no output from Blender"))}
        _, status, seconds = line.split()
        if status != "ok":
            return {seeds[0]: (None, None, status)}
        fresh.replace(path)
        path.with_suffix(".seconds").write_text(seconds + "\n")
        return {seeds[0]: (path, None, None)}

    def timed(*arguments):
        runs = tool(*arguments)
        for seed, (path, _, failure) in runs.items():
            record = path.with_suffix(".seconds") if path else None
            seconds = float(record.read_text()) if record and record.exists() else 0.0
            runs[seed] = (path, seconds, failure)
        return runs

    return timed


def openclare(method, seeded=True, name=None, options=(), sharp_preset=None):
    """A chain of this repository, run from the command line on the model as
    an OBJ, under `name` if it is run with `options` that set it apart. One
    that does not use the seed runs once. On models with hard edges it
    starts from `sharp_preset`, if given."""

    def tool(model, sharp, boundary, target, seeds, directory):
        runs = {}
        for seed in seeds if seeded else seeds[:1]:
            path = directory / f"{name or method}_{seed}.obj"
            command = [OPENCLARE, "remesh", source_path(model), path]
            if sharp and sharp_preset:
                command += ["--preset", sharp_preset]
            command += ["--method", method, "--quads", target, "--seed", seed]
            # The materials of the model, where its scene has them.
            groups = source_path(model).with_suffix(".groups")
            if method == "quad-layout" and groups.exists():
                command += ["--groups", groups]
            command += list(options)
            if sharp:
                command += ["--crease-angle", CREASE_ANGLE]
            done = run(command, memory=memory_cap(model))
            seconds = next((float(l.split()[1]) for l in done.stdout.splitlines() if l.startswith("seconds")), 0.0)
            failed = done.returncode != 0
            said = (done.stderr.strip().splitlines() or ["no output"])[0]
            runs[seed] = (None if failed else path, seconds, failure_of(done, said) if failed else None)
        return runs

    return tool


# What the engine says of a run besides its mesh, kept with the results: the
# edges it laid along the guide curves and along the sharp ones (or along
# the borders between materials), the faces left across such a border, what
# it mended in the input, and on how many pieces the lines were let go.
ENGINE = ["guide_edges", "sharp_curve_edges", "faces_across_groups", "welded_vertices", "edges_taken_apart",
          "guides_let_go", "thin_pieces", "quads_for_thin_pieces"]


def addon(name):
    """The chain the project is about, run as its user runs it: by the
    add-on, in Blender, on the scene of the model, with the settings saved
    in that scene (preset, adaptive size, curves, painted density,
    materials). The number of quads alone is the bench's. It has no seed:
    one run. Returns {seed: (path or None, seconds, failure reason, None,
    what the engine says)}."""

    def tool(model, sharp, boundary, target, seeds, directory):
        # The model as an OBJ is made apart, if it is not there: `directory`
        # is not always where it is kept.
        source_path(model)
        said = in_blender(model, directory, "--quads", target, "--name", f"{name}_{seeds[0]}", "--no-source")
        if said["status"] != "ok":
            return {seeds[0]: (None, 0.0, said["message"], None, None)}
        engine = {key: said["report"][key] for key in ENGINE if key in said["report"]}
        return {seeds[0]: (Path(said["result"]), float(said["seconds"]), None, None, engine)}

    return tool


def faces_of(path):
    with open(path) as lines:
        return sum(1 for line in lines if line.startswith("f "))


# Quad Remesher, left to itself, makes about twice the quads it is asked for.
# These two set the budget aside to compare at equal quad count: its mesh as
# it is, and ours asked for as many quads as that mesh has. Both are measured
# against that number, not against the budget of the bench.
MATCHED = ("quadremesher-as-made", "openclare-matched")


def quadremesher_as_made(model, sharp, boundary, target, seeds, directory):
    """Returns {seed: (path or None, seconds, failure reason, quads to measure against)}."""
    path = quadremesher_reference(model, target, uniform=False)
    if not path.exists():
        return {seeds[0]: (None, 0.0, "no Quad Remesher reference", None)}
    record = path.with_suffix(".seconds")
    seconds = float(record.read_text()) if record.exists() else 0.0
    return {seeds[0]: (path, seconds, None, faces_of(path))}


def openclare_matched(model, sharp, boundary, target, seeds, directory):
    reference = quadremesher_reference(model, target, uniform=False)
    if not reference.exists():
        return {seeds[0]: (None, 0.0, "no Quad Remesher reference to match", None)}
    count = faces_of(reference)
    made = addon("openclare-matched")(model, sharp, boundary, count, seeds, directory)
    return {seed: (path, seconds, failure, count, engine) for seed, (path, seconds, failure, _, engine) in made.items()}


# Every remesher under test. A new chain only needs a line here.
TOOLS = {
    "quadriflow": quadriflow,
    "instant-meshes": openclare("instant-meshes"),
    "integer-grid": openclare("integer-grid", seeded=False),
    # The chain the project is about (`quad-layout` in the engine), under
    # two settings: quads of one size, from the command line, to compare like
    # with like against the tools that have nothing else; then as the add-on
    # runs it on the scene, with what the scene says: quads sized by the
    # shape, as Quad Remesher does by default, and the curves, the painted
    # density and the materials of the scenes that have them.
    "openclare-uniform": openclare(
        "quad-layout",
        seeded=False,
        name="openclare-uniform",
        options=["--adaptivity", "0", "--quads-around", "0"],
        sharp_preset="hard-surface",
    ),
    "openclare": addon("openclare"),
    # Commercial yardstick; needs the add-on and a licence (see bench/README.md).
    "quadremesher": quadremesher(uniform=False),
    "quadremesher-uniform": quadremesher(uniform=True),
    "quadremesher-as-made": quadremesher_as_made,
    "openclare-matched": openclare_matched,
}


def measure(model, path, target):
    done = run([OPENCLARE, "measure", source_path(model), path, "--target", target, "--json"], memory=memory_cap(model))
    if done.returncode != 0:
        return None
    return json.loads(done.stdout)


def median(values):
    values = [v for v in values if v is not None]
    return statistics.median(values) if values else None


def summarize(runs):
    measured = [r for r in runs if r.get("metrics")]
    summary = {
        "runs": len(runs),
        "produced": len(measured),
        "valid": sum(1 for r in measured if r["metrics"]["valid"]),
        "failures": sorted({r["failure"] for r in runs if r.get("failure")}),
    }
    for key, *_ in METRICS:
        values = [r["seconds"] if key == "seconds" else r["metrics"].get(key) for r in measured]
        summary[key] = median(values)
    return summary


def cell(summary, key, pattern):
    value = summary.get(key)
    return "—" if value is None else pattern.format(value * SCALE.get(key, 1.0))


def report(results, target, seeds, tools):
    lines = [
        "# Bench results",
        "",
        f"Requested budget: {target} quads. Median over {len(seeds)} seeds; Quad Remesher and integer-grid have "
        "no seed and run once. The first row is how many quads each tool made; below it, lower is better "
        "for every metric. \"—\" means it could not be measured.",
        "",
    ]
    if all(tool in tools for tool in MATCHED):
        lines += [
            "The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of "
            "`quadremesher` measured against the number of quads it has rather than the budget, and "
            "`openclare-matched` is `openclare` asked for that same number.",
            "",
        ]
    for model, category, *_ in MODELS:
        if model not in results:
            continue
        lines += [f"## {model} ({category})", "", "| | " + " | ".join(tools) + " |", "|---|" + "---|" * len(tools)]
        row = [f"{results[model][t]['valid']}/{results[model][t]['runs']}" for t in tools]
        lines.append("| **Valid meshes (V)** | " + " | ".join(row) + " |")
        for key, label, pattern, _ in METRICS:
            lines.append(f"| {label} | " + " | ".join(cell(results[model][t], key, pattern) for t in tools) + " |")
        failures = [f"{t}: {', '.join(results[model][t]['failures'])}" for t in tools if results[model][t]["failures"]]
        if failures:
            lines += ["", "Failures: " + "; ".join(failures) + "."]
        lines.append("")

    if len(tools) >= 2:
        lines += ["## Summary", "", "Number of models on which each tool has the best median, calibration "
                  "shapes excluded. Ties count for each.", ""]
        for category in ("organic", "mechanical"):
            names = [m for m, c, *_ in MODELS if c == category and m in results]
            if not names:
                continue
            lines += [f"### {category} ({len(names)} models)", "", "| Metric | " + " | ".join(tools) + " |",
                      "|---|" + "---|" * len(tools)]
            for key in QUALITY:
                label = next(l for k, l, *_ in METRICS if k == key)
                wins = dict.fromkeys(tools, 0)
                for model in names:
                    values = {t: results[model][t].get(key) for t in tools}
                    known = [v for v in values.values() if v is not None]
                    for t in tools:
                        if known and values[t] == min(known):
                            wins[t] += 1
                lines.append(f"| {label} | " + " | ".join(str(wins[t]) for t in tools) + " |")
            valid = [f"{sum(results[m][t]['valid'] for m in names)}/{sum(results[m][t]['runs'] for m in names)}" for t in tools]
            lines += ["| Valid meshes (V) | " + " | ".join(valid) + " |", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--models", default=",".join(available_models()))
    parser.add_argument("--tools", default=",".join(TOOLS))
    parser.add_argument("--target", type=int, default=DEFAULT_TARGET)
    parser.add_argument("--seeds", default="1,2,3,4,5")
    parser.add_argument("--source", metavar="MODEL", help="print where the OBJ of that model is, and run nothing")
    options = parser.parse_args()
    if options.source:
        if options.source not in [name for name, *_ in MODELS]:
            sys.exit(f"unknown model '{options.source}'")
        ensure_corpus([options.source])
        print(source_path(options.source))
        return
    models = options.models.split(",")
    tools = options.tools.split(",")
    seeds = [int(s) for s in options.seeds.split(",")]

    if not OPENCLARE.exists():
        sys.exit("build openclare first: cargo build --release")
    if not any((ROOT / "blender" / "openclare" / "lib").glob("*openclare_capi*")):
        sys.exit("build the add-on first: make addon")
    ensure_corpus(models)
    RESULTS.mkdir(exist_ok=True)

    # One pair of files per budget; a partial run (some models, some tools)
    # updates them in place instead of discarding what was measured before.
    suffix = "" if options.target == DEFAULT_TARGET else f"-{options.target}"
    data_path, report_path = RESULTS / f"results{suffix}.json", RESULTS / f"RESULTS{suffix}.md"
    raw = {}
    if data_path.exists():
        previous = json.loads(data_path.read_text())
        if previous.get("seeds") == seeds:
            raw = previous["runs"]

    for model, _, sharp, boundary in MODELS:
        if model not in models:
            continue
        directory = OUTPUT / model
        directory.mkdir(parents=True, exist_ok=True)
        raw.setdefault(model, {})
        for tool in tools:
            runs = []
            for seed, (path, seconds, failure, *own) in sorted(TOOLS[tool](model, sharp, boundary, options.target, seeds, directory).items()):
                # A tool may say what number of quads its mesh is to be
                # measured against, instead of the budget of the bench.
                against = own[0] if own and own[0] else options.target
                metrics = measure(model, path, against) if path else None
                runs.append({"seed": seed, "seconds": seconds, "failure": failure, "metrics": metrics, "against": against})
                # And what the engine says of the run, if it is ours.
                if len(own) > 1 and own[1]:
                    runs[-1]["engine"] = own[1]
            raw[model][tool] = runs
            summary = summarize(runs)
            print(f"{model:10} {tool:15} valid {summary['valid']}/{summary['runs']}  "
                  f"irregular {cell(summary, 'irregular_vertices', '{:.0f}'):>5}  "
                  f"patches {cell(summary, 'base_complex_patches', '{:.0f}'):>6}  "
                  f"alignment {cell(summary, 'alignment', '{:.2f}'):>6}", flush=True)

    # Report every tool that has results, in the order they are declared.
    reported = [t for t in TOOLS if all(t in by_tool for by_tool in raw.values())]
    results = {model: {t: summarize(raw[model][t]) for t in reported} for model in raw}
    data_path.write_text(json.dumps({"target": options.target, "seeds": seeds, "runs": raw, "summary": results}, indent=1))
    report_path.write_text(report(results, options.target, seeds, reported))
    print(f"\nwritten: {report_path}")


if __name__ == "__main__":
    main()
