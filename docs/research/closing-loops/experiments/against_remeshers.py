#!/usr/bin/env python3
"""The comparison with other remeshers (Section "Experiments", "Against other
remeshers"), read from the bench's results at 3,000 quads; nothing of the
bench is run again.

    python3 docs/research/closing-loops/experiments/against_remeshers.py \
        [--results bench/results/results.json] [--git <commit>]... [--no-closing-run]

The figures, with the paper's definitions: a tool "produces a mesh" for a
model if one of its runs gives a mesh whose loops can be measured (a share of
spiral edges); it "meshes validly in every run" if every run of it gives a
valid mesh. Per model and tool, the median over the runs (five seeds for
QuadriFlow, one run for openclare and AutoRemesher), as the bench reports it;
then the mean over the models of the subset. Besides the share of spiral
edges, the independent measures of structure (DT-137): edges on helices of
any pitch, helices of pitch above four the spiral count misses, edges on
closed loops.

--git <commit> reads `bench/results/results.json` as it is at that commit
too (the paper's figures came from 5bc16c6, and without closing from
b0fba91), to check that the definitions give the paper's numbers back.

--no-closing-run makes the line "without closing" on the engine given: the
add-on chain of the bench (bench/blender_run.py, on each model's scene) with
`OPENCLARE_CLOSE_WAYS=` (no way of closing tried: the plain search alone), on
the models of the subsets. It needs the add-on's library built from the same
commit (`make addon`; the engine of the add-on is that library, not
--openclare, which measures) and blender on the PATH.
"""

import argparse
import json
import os
import statistics
import subprocess
import time
from pathlib import Path

import common

TOOLS = ["openclare", "quadriflow", "autoremesher"]
KEYS = ["loop_edges_spiral", "loop_edges_helix", "loops_helix_missed", "loop_edges_closed"]


def measured(runs):
    return [run for run in runs if run.get("metrics") and run["metrics"].get("loop_edges_spiral") is not None]


def produced(results, model, tool):
    return bool(measured(results["runs"][model].get(tool, [])))


def valid_every_run(results, model, tool):
    runs = results["runs"][model].get(tool, [])
    return bool(runs) and all(run.get("metrics") and run["metrics"]["valid"] for run in runs)


def median(runs, key):
    values = [run["metrics"][key] for run in measured(runs) if run["metrics"].get(key) is not None]
    return statistics.median(values) if values else None


def subset_means(per_model, models, keys=KEYS):
    out = {}
    for key in keys:
        values = [per_model[model][key] for model in models if per_model[model].get(key) is not None]
        out[key] = statistics.mean(values) if values else None
    return out


def compare(results, tools=TOOLS):
    """The subsets and the means of the paper, from one results file."""
    models = list(results["runs"])
    every_valid = [model for model in models if all(valid_every_run(results, model, tool) for tool in tools)]
    all_produce = [model for model in models if all(produced(results, model, tool) for tool in tools)]
    out = {
        "models": len(models),
        "produce_a_mesh": {tool: sum(1 for model in models if produced(results, model, tool)) for tool in tools},
        "valid_in_every_run": {tool: sum(1 for model in models if valid_every_run(results, model, tool))
                               for tool in tools},
        "subsets": {},
    }
    for name, subset in (("valid_in_every_run", every_valid), ("all_produce_a_mesh", all_produce)):
        out["subsets"][name] = {"models": subset, "means": {}}
        for tool in tools:
            per_model = {model: {key: median(results["runs"][model].get(tool, []), key) for key in KEYS}
                         for model in subset}
            out["subsets"][name]["means"][tool] = subset_means(per_model, subset)
    return out


def at_commit(commit):
    done = subprocess.run(["git", "show", f"{commit}:bench/results/results.json"], cwd=common.ROOT,
                          capture_output=True, text=True, check=True)
    return json.loads(done.stdout)


def scene(model, places):
    """The scene the bench runs a model from (bench/run.py, scene_path)."""
    for place in places:
        if (place / f"{model}.blend").exists():
            return place / f"{model}.blend"
    raise SystemExit(f"no scene of {model} in {', '.join(map(str, places))}")


def no_closing_run(models, target, directory, places):
    """openclare as the bench runs it (the add-on, on the scene), with no way
    of closing tried; measured as the bench measures it."""
    runs = {}
    environment = {key: value for key, value in os.environ.items() if not key.startswith("OPENCLARE_")}
    environment["OPENCLARE_CLOSE_WAYS"] = ""
    for model in models:
        record = directory / f"{model}.json"
        if record.exists():
            runs[model] = json.loads(record.read_text())
            print(f"{model}: read back")
            continue
        own = directory / model
        own.mkdir(parents=True, exist_ok=True)
        clock = time.monotonic()
        done = subprocess.run(
            ["blender", "-b", "--factory-startup", str(scene(model, places)), "--python-exit-code", "1",
             "--python", str(common.ROOT / "bench" / "blender_run.py"), "--", "--out", str(own),
             "--quads", str(target), "--name", "openclare-no-closing_1", "--no-source"],
            env=environment, capture_output=True, text=True, preexec_fn=common.capped(common.MEMORY_LARGE))
        seconds = time.monotonic() - clock
        line = next((l for l in done.stdout.splitlines() if l.startswith("OPENCLARE ")), None)
        said = json.loads(line.removeprefix("OPENCLARE ")) if line else {"status": "error", "message": "no output"}
        run = {"seed": 1, "seconds": round(seconds, 2), "failure": None, "metrics": None}
        if said.get("status") != "ok":
            run["failure"] = said.get("message")
        else:
            mesh = Path(said["result"])
            run["metrics"] = common.measure(common.source(model), mesh, target, common.MEMORY_LARGE)
        common.write_json(record, run)
        runs[model] = run
        print(f"{model}: {run['failure'] or 'ok'} ({run['seconds']} s)", flush=True)
    return runs


def percent(value):
    return None if value is None else round(100 * value, 2)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    common.options(parser, "against_remeshers")
    parser.add_argument("--results", type=Path, default=common.ROOT / "bench" / "results" / "results.json")
    parser.add_argument("--git", action="append", default=[], help="also read the results at this commit")
    parser.add_argument("--no-closing-run", action="store_true",
                        help="make the line without closing on the engine given (Blender, the add-on)")
    parser.add_argument("--scenes", type=Path, nargs="+",
                        default=[common.ROOT / "bench" / "scenes", common.ROOT / "bench" / "corpus-large" / "scenes"],
                        help="where the models' scenes are (default those of bench/run.py)")
    arguments = parser.parse_args()
    common.use(arguments)

    results = json.loads(arguments.results.read_text())
    started, clock = common.machine(), time.monotonic()
    summary = {
        "experiment": "against other remeshers, read from the bench",
        "results": str(arguments.results.resolve().relative_to(common.ROOT)),
        "target": results["target"],
        "current": compare(results),
        "at_commits": {commit: compare(at_commit(commit)) for commit in arguments.git},
    }
    if arguments.no_closing_run:
        subsets = summary["current"]["subsets"]
        models = sorted(set(subsets["valid_in_every_run"]["models"]) | set(subsets["all_produce_a_mesh"]["models"]))
        runs = no_closing_run(models, results["target"], arguments.out, arguments.scenes)
        line = {}
        for name, subset in subsets.items():
            per_model = {model: {key: median([runs[model]], key) for key in KEYS} for model in subset["models"]}
            line[name] = subset_means(per_model, subset["models"])
        summary["no_closing"] = {
            **common.header(arguments, started, [round(x, 2) for x in os.getloadavg()], time.monotonic() - clock,
                           list(runs.values())),
            "how": "the add-on chain (bench/blender_run.py) with OPENCLARE_CLOSE_WAYS= (no way of closing)",
            "valid": sum(1 for run in runs.values() if run["metrics"] and run["metrics"]["valid"]),
            "runs": len(runs),
            "means": line,
            "per_model": {model: {key: run["metrics"].get(key) if run["metrics"] else None
                                  for key in ["valid", "quads", *KEYS]} for model, run in runs.items()},
        }
    else:
        summary["engine"] = {"commit": arguments.commit, "note": "nothing run; the bench's results are read"}
    common.write_json(arguments.summary, summary)

    for label, data in [("current", summary["current"]), *summary["at_commits"].items()]:
        print(f"{label}: produce a mesh {data['produce_a_mesh']}, valid in every run {data['valid_in_every_run']}")
        for name, subset in data["subsets"].items():
            print(f"  {name} ({len(subset['models'])}): "
                  + ", ".join(f"{tool} {percent(subset['means'][tool]['loop_edges_spiral'])}%" for tool in TOOLS))
    if "no_closing" in summary:
        for name, means in summary["no_closing"]["means"].items():
            print(f"  no closing, {name}: {percent(means['loop_edges_spiral'])}%")
    print(f"summary: {arguments.summary}")


if __name__ == "__main__":
    main()
