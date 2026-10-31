#!/usr/bin/env python3
"""Runs every remesher on every model of the corpus and tabulates the metrics.

    python3 bench/run.py [--models a,b] [--tools a,b] [--target N] [--seeds 1,2,3]
    python3 bench/run.py --source MODEL

A model is a scene, a .blend that holds the object to remesh and the settings
of the add-on (bench/scenes; see bench/make_scene.py). `openclare` runs on it
through the add-on, in Blender; the other tools are given the same object as
the add-on sees it, exported as an OBJ. `--source` prints where that OBJ is,
after making it if need be.

Remeshers whose results are not to be published can be run too, on this
machine only: see `UNPUBLISHED` below.

Needs `blender` on the PATH, a release build of `openclare` and the add-on
built from it (`cargo build --release`, then `make addon`). Results go to
bench/results/.
"""

import argparse
import json
import os
import resource
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path

BENCH = Path(__file__).resolve().parent
ROOT = BENCH.parent
OPENCLARE = ROOT / "target" / "release" / "openclare"
AUTOREMESHER = (
    shutil.which("autoremesher")
    or (Path.home() / ".local" / "bin" / "autoremesher" if (Path.home() / ".local" / "bin" / "autoremesher").exists() else None)
)
# Open-source remeshers that bench/fetch_remeshers.py downloads, and builds,
# into local/remeshers (not tracked); an environment variable says where they
# are instead.
REMESHERS = ROOT / "local" / "remeshers"
INSTANT_MESHES_JAKOB = Path(os.environ.get("OPENCLARE_INSTANT_MESHES_JAKOB",
                                           REMESHERS / "instant-meshes-jakob" / "instant-meshes"))
# The directory of the release: both programs and their `config/`.
QUADWILD_BIMDF = Path(os.environ.get("OPENCLARE_QUADWILD_BIMDF", REMESHERS / "quadwild-bimdf"))
# The scenes: what the bench runs on.
SCENES = BENCH / "scenes"
# The models that are a plain mesh, as OBJ files: what bench/make_scenes.py
# makes their scenes from.
CORPUS = BENCH / "corpus"
OUTPUT = BENCH / "out"
RESULTS = BENCH / "results"
# Meshes that are not made by the bench: those modelled by hand.
REFERENCE = BENCH / "reference"
# Models whose licence does not allow passing them on: they, their scenes
# (in `scenes` there) and what is remeshed from them stay in this directory,
# which is not tracked. See CREDITS.md for where each one comes from and how
# to fetch it again.
LOCAL = BENCH / "corpus-large"
# Local, unpublished challengers: remeshers whose licence does not allow
# passing on what they make. Each is a Blender script,
# `local/challengers/<name>.py`, called as bench/quadriflow.py is (source OBJ,
# prefix of the output, quads, sharp 0|1, boundary 0|1, seeds) and printing a
# line "CHALLENGER <seed> <status> <seconds>" per mesh, written to
# `<prefix>_<seed>.obj`. It runs once per model and budget; its mesh is kept
# in `local/reference/<name>/<model>__<budget>.obj` (and its time next to
# it, `.seconds`) and read back from there afterwards, with or without the
# software. A directory of meshes there without a script is a challenger
# too. None of this is tracked, and their results go to `local/results/`,
# never to bench/results: nothing is published of a remesher that is not
# installed here.
UNPUBLISHED = ROOT / "local"
UNPUBLISHED_RESULTS = UNPUBLISHED / "results"
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
# The human bodies of Blender's Essentials library (CC0): a sculpt and the
# mesh modelled by hand on it, `make essentials` (bench/fetch_essentials.py)
# takes them from Blender's cache. Their sculpts are large: local too.
ESSENTIALS = {"human_male", "human_female"}
LOCAL_MODELS = set(BLENDERKIT) | ESSENTIALS
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
    # The human bodies of Blender's Essentials library: sculpts of 1.35 million
    # triangles, with the mesh of 10,590 faces modelled on them by hand (the
    # base level of their Multires), what the remeshers are held up against.
    ("human_male", "organic", False, False),
    ("human_female", "organic", False, False),
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
    # Four small buildings by LapaModels, made with a generative model:
    # 15,000 triangles each, closed, with dozens of holes through them
    # (railings, sails, eaves), two of them in several pieces, and a
    # surface that is all planks, tiles and stones. Free to pass on.
    ("watermill", "extra", False, False),
    ("towermill", "extra", False, False),
    ("stonemill", "extra", False, False),
    ("windmill", "extra", False, False),
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
    ("feature_edges_kept", "Sharp edges kept, % of their length (E)", "{:.1f}", False),
    ("loop_edges_closed", "Edges on closed loops, % (L)", "{:.1f}", False),
    ("loop_edges_spiral", "Edges on spiralling loops, % (L)", "{:.1f}", True),
    ("loops_spiral", "Spiralling loops (L)", "{:.0f}", True),
    ("loop_edges_helix", "Edges on helices of any pitch, % (L)", "{:.1f}", True),
    ("loops_helix_missed", "Helices of pitch above 4, which the spiral count misses (L)", "{:.0f}", True),
    ("loop_edges_open", "Edges on loops that stop at an irregular vertex, % (L)", "{:.1f}", True),
    ("longest_open_loop", "Longest loop that stops at an irregular vertex, edges (L)", "{:.0f}", True),
    ("loop_edges_border", "Edges on loops from a border to a border, % (L)", "{:.1f}", False),
    ("budget_error", "Off budget, % (N)", "{:.1f}", True),
    ("seconds", "Time, seconds (T)", "{:.2f}", True),
]
SCALE = {"hausdorff": 1000.0, "mean_distance": 1000.0, "budget_error": 100.0, "thin_distance": 100.0, "volume_ratio": 100.0,
         "feature_edges_kept": 100.0,
         "loop_edges_closed": 100.0, "loop_edges_spiral": 100.0, "loop_edges_helix": 100.0, "loop_edges_open": 100.0,
         "loop_edges_border": 100.0}
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


def run(command, timeout=1800, memory=MEMORY_CAP, cwd=None):
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
        return subprocess.run(command, capture_output=True, text=True, timeout=timeout, preexec_fn=capped, cwd=cwd)
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


def autoremesher(model, sharp, boundary, target, seeds, directory):
    """AutoRemesher (huxingyi/autoremesher): open-source automatic quad remeshing.
    Deterministic (no seed): one run. Returns {seed: (path or None, seconds, failure)}."""
    if not AUTOREMESHER:
        return {seeds[0]: (None, 0.0, "autoremesher not found (install AutoRemesher)")}
    path = directory / f"autoremesher_{seeds[0]}.obj"
    report_file = directory / f"autoremesher_{seeds[0]}_report.txt"
    path.unlink(missing_ok=True)
    report_file.unlink(missing_ok=True)
    cmd = []
    if not os.environ.get("DISPLAY") and shutil.which("xvfb-run"):
        cmd += ["xvfb-run", "-a"]
    cmd += [
        str(AUTOREMESHER),
        "--input", source_path(model),
        "--output", path,
        "--target-quads", target,
        "--sharp-edge", "90.0" if sharp else "180.0",
        "--report", report_file,
    ]
    done = run(cmd, memory=memory_cap(model))
    seconds = 0.0
    if report_file.exists():
        for line in report_file.read_text().splitlines():
            if "time:" in line.lower():
                try:
                    seconds = float(line.split(":")[-1].replace("seconds", "").strip())
                except ValueError:
                    pass
    if seconds == 0.0:
        for line in done.stdout.splitlines():
            if line.startswith("Total:") and "ms" in line:
                try:
                    seconds = float(line.removeprefix("Total:").replace("ms", "").strip()) / 1000.0
                except ValueError:
                    pass
    if path.exists() and path.stat().st_size > 0:
        return {seeds[0]: (path, seconds, None)}
    return {seeds[0]: (None, seconds, failure_of(done, "no mesh produced"))}


def not_fetched(name):
    return f"{name} not found (run bench/fetch_remeshers.py)"


def closest_to(target, made):
    """Of the meshes `made` ({path: faces}), the one whose number of faces is
    nearest the budget."""
    return min(made, key=lambda path: abs(made[path] - target))


def instant_meshes_jakob(model, sharp, boundary, target, seeds, directory):
    """Instant Meshes (wjakob/instant-meshes, Jakob et al. 2015), the original
    program, in batch mode, deterministic (`-d`): no seed, one run.

    Its pure quad output (the default; `-D` would leave triangles and
    pentagons) is its quad-dominant mesh with every face cut in quads around
    its centre: a quad gives four, a triangle three. It is asked for a
    quarter of the budget with `-f`. The count is approximate: if the mesh is
    more than 2 % off, it is run once more with the request scaled by how far
    off it was, and the nearer of the two is kept; the time is that of both.
    `-c` is the bench's crease angle on models with hard edges, `-b` aligns
    to the borders of open models. Returns {seed: (path or None, seconds,
    failure)}."""
    if not INSTANT_MESHES_JAKOB.exists():
        return {seeds[0]: (None, 0.0, not_fetched("instant-meshes-jakob"))}
    request, made, seconds, done = target / 4, {}, 0.0, None
    for attempt in range(2):
        path = directory / f"instant-meshes-jakob_{seeds[0]}_{attempt}.obj"
        path.unlink(missing_ok=True)
        command = [INSTANT_MESHES_JAKOB, "-d", "-f", round(request), "-o", path]
        if sharp:
            command += ["-c", CREASE_ANGLE]
        if boundary:
            command += ["-b"]
        start = time.monotonic()
        done = run([*command, source_path(model)], memory=memory_cap(model))
        seconds += time.monotonic() - start
        if done.returncode != 0 or not path.exists():
            break
        made[path] = faces_of(path)
        if abs(made[path] - target) <= 0.02 * target:
            break
        request *= target / made[path]
    if not made:
        return {seeds[0]: (None, seconds, failure_of(done, "no mesh produced"))}
    kept = directory / f"instant-meshes-jakob_{seeds[0]}.obj"
    shutil.move(closest_to(target, made), kept)
    for path in made:
        path.unlink(missing_ok=True)
    return {seeds[0]: (kept, seconds, None)}


def quadwild_bimdf(model, sharp, boundary, target, seeds, directory):
    """QuadWild with Bi-MDF quantization (cgg-bern/quadwild-bimdf, Heistermann
    et al. 2023), its release binaries, without Gurobi: no seed, one run.

    Two programs, as its README runs them. `quadwild` remeshes the input,
    traces the field and cuts the surface in patches, with the release's
    "Mechanical" setup on models with hard edges (sharp edges kept above
    the bench's crease angle, `alpha` 0.01) and its "Organic" one on the
    others (no sharp edges, `alpha` 0.02). It has no option for borders.
    `quad_from_patches` then quadrangulates the patches with `flow.txt`, the
    configuration its README recommends for edge flow.

    It takes no number of quads, only `scaleFact`, a factor on the size of
    its quads. The second program is run with 1, then with the factor scaled
    by the square root of how far off the number of quads was, up to three
    runs while it is more than 2 % off, and the nearest is kept; the time is
    that of every run. Its mesh may have faces other than quads: they are
    left for the validity measure to report. Returns {seed: (path or None,
    seconds, failure)}."""
    programs = [QUADWILD_BIMDF / "quadwild", QUADWILD_BIMDF / "quad_from_patches"]
    if not all(program.exists() for program in programs):
        return {seeds[0]: (None, 0.0, not_fetched("quadwild-bimdf"))}
    # It writes what it makes next to its input: a directory of its own.
    work = directory / "quadwild-bimdf"
    shutil.rmtree(work, ignore_errors=True)
    work.mkdir()
    source = work / "source.obj"
    shutil.copyfile(source_path(model), source)
    preparation = work / "prepare.txt"
    preparation.write_text(f"do_remesh 1\nsharp_feature_thr {CREASE_ANGLE if sharp else -1}\n"
                           f"alpha {0.01 if sharp else 0.02}\nscaleFact 1\n")
    # Its configuration files name others relative to the directory of the
    # release: both programs run from there, on absolute paths.
    start = time.monotonic()
    done = run([programs[0], source.resolve(), 2, preparation.resolve()], memory=memory_cap(model),
               cwd=QUADWILD_BIMDF)
    seconds = time.monotonic() - start
    patches = work / "source_rem_p0.obj"
    if not patches.exists():
        return {seeds[0]: (None, seconds, failure_of(done, "no patches produced"))}
    flow = (QUADWILD_BIMDF / "config" / "main_config" / "flow.txt").read_text().splitlines()
    result = work / "source_rem_p0_1_quadrangulation_smooth.obj"
    scale, made = 1.0, {}
    for attempt in range(3):
        configuration = work / "flow.txt"
        configuration.write_text("\n".join(f"scaleFact {scale:.6f}" if line.startswith("scaleFact ") else line
                                           for line in flow) + "\n")
        result.unlink(missing_ok=True)
        start = time.monotonic()
        # Release v0.0.2 crashes as it exits, once its meshes are written:
        # a run is judged by the mesh it leaves, not by its exit code.
        done = run([programs[1], patches.resolve(), 1, configuration.resolve()], memory=memory_cap(model),
                   cwd=QUADWILD_BIMDF)
        seconds += time.monotonic() - start
        if not result.exists() or result.stat().st_size == 0:
            break
        path = work / f"attempt_{attempt}.obj"
        shutil.move(result, path)
        made[path] = faces_of(path)
        if abs(made[path] - target) <= 0.02 * target:
            break
        scale *= (made[path] / target) ** 0.5
    if not made:
        return {seeds[0]: (None, seconds, failure_of(done, "no mesh produced"))}
    kept = directory / f"quadwild-bimdf_{seeds[0]}.obj"
    shutil.move(closest_to(target, made), kept)
    return {seeds[0]: (kept, seconds, None)}


def challenger_reference(name, model, target):
    """Where the mesh of a local challenger is kept (see UNPUBLISHED)."""
    return UNPUBLISHED / "reference" / name / f"{model}__{target}.obj"


def challenger(name, refresh=False):
    """A local challenger (see UNPUBLISHED), run once and kept: the seed is
    the first one of the bench. Returns {seed: (path or None, seconds,
    failure reason)}."""
    script = UNPUBLISHED / "challengers" / f"{name}.py"

    def tool(model, sharp, boundary, target, seeds, directory):
        path = challenger_reference(name, model, target)
        record = path.with_suffix(".seconds")
        if refresh or not path.exists():
            if not script.exists():
                return {seeds[0]: (None, 0.0, f"no mesh kept and no script for {name}")}
            prefix = directory / name
            fresh = Path(f"{prefix}_{seeds[0]}.obj")
            fresh.unlink(missing_ok=True)
            done = run(["blender", "-b", "--factory-startup", "--python", script, "--", source_path(model), prefix,
                        target, int(sharp), int(boundary), seeds[0]], memory=memory_cap(model))
            line = next((l for l in done.stdout.splitlines() if l.startswith("CHALLENGER ")), None)
            if line is None:
                return {seeds[0]: (None, 0.0, failure_of(done, "no output from Blender"))}
            _, _, status, seconds = line.split()
            if status != "ok" or not fresh.exists():
                return {seeds[0]: (None, float(seconds), status)}
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.move(fresh, path)
            record.write_text(seconds + "\n")
        seconds = float(record.read_text()) if record.exists() else 0.0
        return {seeds[0]: (path, seconds, None)}

    return tool


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
            # A chain that starts from a preset has its sharp edges from it.
            if sharp and not sharp_preset:
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
    """The chain the project is about, run as the add-on calls it, in
    Blender, on the scene of the model, with the settings saved in that
    scene (preset, adaptive size, curves, painted density,
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


# The meshes modelled by hand, for the models that have one.
HANDMADE = REFERENCE / "handmade"


def handmade(model, sharp, boundary, target, seeds, directory):
    """The mesh modelled by hand on the model, measured against its own
    number of quads: none for the models that have none."""
    path = HANDMADE / f"{model}.obj"
    if not path.exists():
        return {}
    return {seeds[0]: (path, None, None, faces_of(path))}


def openclare_as_handmade(model, sharp, boundary, target, seeds, directory):
    """openclare asked for as many quads as the mesh modelled by hand has."""
    path = HANDMADE / f"{model}.obj"
    if not path.exists():
        return {}
    count = faces_of(path)
    made = addon("openclare-as-handmade")(model, sharp, boundary, count, seeds, directory)
    return {seed: (path, seconds, failure, count, engine) for seed, (path, seconds, failure, _, engine) in made.items()}


# Every remesher under test. A new chain only needs a line here.
TOOLS = {
    "quadriflow": quadriflow,
    "autoremesher": autoremesher,
    # Two open-source remeshers of the literature, fetched apart
    # (bench/fetch_remeshers.py). Not to be confused with `instant-meshes`,
    # the chain of this repository built on the same method.
    "instant-meshes-jakob": instant_meshes_jakob,
    "quadwild-bimdf": quadwild_bimdf,
    "instant-meshes": openclare("instant-meshes"),
    "integer-grid": openclare("integer-grid", seeded=False),
    # The chain the project is about (`quad-layout` in the engine), under
    # two settings: quads of one size, from the command line, to compare like
    # with like against the tools that have nothing else; then as the add-on
    # runs it on the scene, with what the scene says: quads sized by the
    # shape, and the curves, the painted density and the materials of the
    # scenes that have them.
    "openclare-uniform": openclare(
        "quad-layout",
        seeded=False,
        name="openclare-uniform",
        options=["--adaptivity", "0", "--quads-around", "0"],
        sharp_preset="hard-surface",
    ),
    "openclare": addon("openclare"),
    # What a modeller made by hand, where there is such a mesh, and openclare
    # at its number of quads.
    "handmade": handmade,
    "openclare-as-handmade": openclare_as_handmade,
}


def local_challengers():
    """The local, unpublished challengers there are on this machine (see
    UNPUBLISHED), by name: a script, or meshes kept, or both."""
    names = {path.stem for path in (UNPUBLISHED / "challengers").glob("*.py")}
    reference = UNPUBLISHED / "reference"
    if reference.is_dir():
        names |= {path.name for path in reference.iterdir() if path.is_dir()}
    return {name: challenger(name) for name in sorted(names) if name not in TOOLS}


CHALLENGERS = local_challengers()


def kept_as(path):
    """How a mesh is named in the results: from the root of the repository
    where it is under it."""
    path = Path(path).resolve()
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def mesh_of_run(model, tool, run, target):
    """The mesh a run of the results was made of, if it is still there: where
    the results say, or for those older than that, where it is always kept
    (the hand-made ones, those of the local challengers) or was last left, if
    it has the number of faces measured then."""
    if run.get("mesh"):
        path = Path(run["mesh"])
        path = path if path.is_absolute() else ROOT / path
        return path if path.exists() else None
    if tool in CHALLENGERS:
        path = challenger_reference(tool, model, target)
    elif tool == "handmade":
        path = HANDMADE / f"{model}.obj"
    else:
        path = OUTPUT / model / f"{tool}_{run['seed']}.obj"
    faces = (run.get("metrics") or {}).get("faces")
    if not path.exists() or faces is None or faces_of(path) != faces:
        return None
    return path


def remeasure(raw, models, tools, target):
    """Measures again, in place, the meshes of the runs of `raw` that are
    still there; the others keep what was measured of them."""
    again = left = 0
    for model in models:
        for tool in tools:
            for run in raw.get(model, {}).get(tool, []):
                path = mesh_of_run(model, tool, run, target)
                if path is None:
                    left += run.get("metrics") is not None
                    continue
                run["metrics"] = measure(model, path, run.get("against") or target)
                run["mesh"] = kept_as(path)
                again += 1
        print(f"{model:10} measured again", flush=True)
    print(f"{again} meshes measured again, {left} no longer there and left as they were")


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
        f"Requested budget: {target} quads. Median over {len(seeds)} seeds; AutoRemesher, integer-grid, "
        "instant-meshes-jakob and quadwild-bimdf have no seed and run once. The first row is how many quads each tool made; below it, lower is better "
        "for every metric. \"—\" means it could not be measured.",
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
    parser.add_argument("--tools", default=",".join([*TOOLS, *CHALLENGERS]))
    parser.add_argument("--target", type=int, default=DEFAULT_TARGET)
    parser.add_argument("--seeds", default="1,2,3,4,5")
    parser.add_argument("--results", type=Path, default=RESULTS,
                        help="where to write the results (default bench/results), for a trial run kept apart")
    parser.add_argument("--source", metavar="MODEL", help="print where the OBJ of that model is, and run nothing")
    parser.add_argument(
        "--remeasure",
        action="store_true",
        help="run no remesher: measure again the meshes the results of this budget were made of, where they are kept",
    )
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
    options.results.mkdir(parents=True, exist_ok=True)

    every = {**TOOLS, **CHALLENGERS}
    unknown = [tool for tool in tools if tool not in every]
    if unknown:
        sys.exit(f"unknown tool: {', '.join(unknown)} (known: {', '.join(every)})")

    # One pair of files per budget; a partial run (some models, some tools)
    # updates them in place instead of discarding what was measured before.
    # The runs of the local challengers are kept apart, in local/results.
    suffix = "" if options.target == DEFAULT_TARGET else f"-{options.target}"

    def stored(directory):
        path = directory / f"results{suffix}.json"
        if path.exists():
            previous = json.loads(path.read_text())
            if previous.get("seeds") == seeds:
                return previous["runs"]
        return {}

    raw = stored(options.results)
    for model, by_tool in stored(UNPUBLISHED_RESULTS).items():
        raw.setdefault(model, {}).update(by_tool)

    if options.remeasure:
        remeasure(raw, models, tools, options.target)
    for model, _, sharp, boundary in MODELS:
        if model not in models or options.remeasure:
            continue
        # One directory per budget: the meshes of each are kept, to be
        # measured again without being made again.
        directory = OUTPUT / model / str(options.target)
        directory.mkdir(parents=True, exist_ok=True)
        raw.setdefault(model, {})
        for tool in tools:
            runs = []
            for seed, (path, seconds, failure, *own) in sorted(every[tool](model, sharp, boundary, options.target, seeds, directory).items()):
                # A tool may say what number of quads its mesh is to be
                # measured against, instead of the budget of the bench.
                against = own[0] if own and own[0] else options.target
                metrics = measure(model, path, against) if path else None
                runs.append({"seed": seed, "seconds": seconds, "failure": failure, "metrics": metrics, "against": against})
                if path:
                    runs[-1]["mesh"] = kept_as(path)
                # And what the engine says of the run, if it is ours.
                if len(own) > 1 and own[1]:
                    runs[-1]["engine"] = own[1]
            raw[model][tool] = runs
            summary = summarize(runs)
            print(f"{model:10} {tool:15} valid {summary['valid']}/{summary['runs']}  "
                  f"irregular {cell(summary, 'irregular_vertices', '{:.0f}'):>5}  "
                  f"patches {cell(summary, 'base_complex_patches', '{:.0f}'):>6}  "
                  f"alignment {cell(summary, 'alignment', '{:.2f}'):>6}", flush=True)

    def write(directory, kept, shown):
        """The runs of the tools `kept` into the data file of `directory`,
        and the report of the tools `shown` next to it. Every tool that has
        results is reported, in the order they are declared; where a tool
        has none for a model, its column says so."""
        runs = {model: {t: by_tool[t] for t in kept if t in by_tool} for model, by_tool in raw.items()}
        reported = [t for t in shown if any(t in by_tool for by_tool in raw.values())]
        results = {model: {t: summarize(raw[model].get(t, [])) for t in reported} for model in raw}
        directory.mkdir(parents=True, exist_ok=True)
        (directory / f"results{suffix}.json").write_text(
            json.dumps({"target": options.target, "seeds": seeds, "runs": runs, "summary": results}, indent=1))
        report_path = directory / f"RESULTS{suffix}.md"
        report_path.write_text(report(results, options.target, seeds, reported))
        print(f"\nwritten: {report_path}")

    write(options.results, list(TOOLS), list(TOOLS))
    if any(tool in by_tool for tool in CHALLENGERS for by_tool in raw.values()):
        write(UNPUBLISHED_RESULTS, list(CHALLENGERS), list(every))


if __name__ == "__main__":
    main()
