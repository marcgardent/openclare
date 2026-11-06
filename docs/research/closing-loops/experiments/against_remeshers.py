#!/usr/bin/env python3
"""The comparison with other remeshers (Section "Experiments", "Against other
remeshers"), read from the bench's results at 3,000 quads; nothing of the
bench is run again.

    python3 docs/research/closing-loops/experiments/against_remeshers.py \
        [--results bench/results/results.json] [--git <commit>]...
        [--no-closing-run | --no-closing-from <summary>]

The figures, with the paper's definitions: a tool "produces a mesh" for a
model if one of its runs gives a mesh whose loops can be measured (a share of
spiral edges); it "meshes validly in every run" if every run of it gives a
valid mesh; a model a tool was not run on counts as neither. Per model and
tool, the median over the runs (five seeds for QuadriFlow, one run for
openclare, AutoRemesher, QuadWild-BiMDF and Instant Meshes), as the bench
reports it; then the mean over the models of the subset. QuadWild-BiMDF and
Instant Meshes were run on the 27 models whose meshes may be passed on
(CREDITS.md), not on the other 6. Besides the share of spiral
edges, the independent measures of structure (DT-137): edges on helices of
any pitch, helices of pitch above four the spiral count misses, edges on
closed loops; and, for the loops that are neither, edges on open loops and
irregular vertices.

--git <commit> reads `bench/results/results.json` as it is at that commit
too (the paper's figures came from 5bc16c6, and without closing from
b0fba91, before the bench ran QuadWild-BiMDF and Instant Meshes), to check
that the definitions give the paper's numbers back; a results file is
compared on the tools it has.

--no-closing-run makes the line "without closing" on the engine given: the
add-on chain of the bench (bench/blender_run.py, on each model's scene) with
`OPENCLARE_CLOSE_WAYS=` (no way of closing tried: the plain search alone), on
the models of the subsets. It needs the add-on's library built from the same
commit (`make addon`; the engine of the add-on is that library, not
--openclare, which measures) and blender on the PATH.

--no-closing-from <summary> makes the same line from the runs a previous
summary recorded (its "no_closing" "per_model"), nothing run: the means are
taken again on the current subsets, which must be among its models.
"""

import argparse
import json
import os
import statistics
import subprocess
import time
from pathlib import Path

import common

TOOLS = ["openclare", "quadriflow", "autoremesher", "quadwild-bimdf", "instant-meshes-jakob"]
NAMES = {"openclare": "openclare", "quadriflow": "QuadriFlow", "autoremesher": "AutoRemesher",
         "quadwild-bimdf": "QuadWild-BiMDF", "instant-meshes-jakob": "Instant Meshes"}
KEYS = ["loop_edges_spiral", "loop_edges_helix", "loops_helix_missed", "loop_edges_closed", "loop_edges_open",
        "irregular_vertices"]


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


def compare(results):
    """The subsets and the means of the paper, from one results file."""
    models = list(results["runs"])
    tools = [tool for tool in TOOLS if any(tool in results["runs"][model] for model in models)]
    every_valid = [model for model in models if all(valid_every_run(results, model, tool) for tool in tools)]
    all_produce = [model for model in models if all(produced(results, model, tool) for tool in tools)]
    out = {
        "models": len(models),
        "run_on": {tool: sum(1 for model in models if tool in results["runs"][model]) for tool in tools},
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
            out["subsets"][name].setdefault("per_model", {})[tool] = {
                model: per_model[model]["loop_edges_spiral"] for model in subset}
        # Model by model, on how many openclare has fewer spiral edges than
        # each other tool, more, as many: a mean over seven models says
        # little by itself (#222).
        ours = out["subsets"][name]["per_model"][tools[0]]
        out["subsets"][name]["openclare_against"] = {
            tool: {"fewer": sum(ours[m] < theirs[m] for m in subset),
                   "more": sum(ours[m] > theirs[m] for m in subset),
                   "as_many": sum(ours[m] == theirs[m] for m in subset)}
            for tool in tools[1:] for theirs in [out["subsets"][name]["per_model"][tool]]}
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


def no_closing_from(path, models):
    """The runs without closing a previous summary recorded, as runs."""
    recorded = json.loads(path.read_text())["no_closing"]
    missing = [model for model in models if model not in recorded["per_model"]]
    if missing:
        raise SystemExit(f"{path} has no run without closing of {', '.join(missing)}")
    return recorded, {model: {"metrics": recorded["per_model"][model]} for model in models}


def percent(value):
    return None if value is None else round(100 * value, 2)


def table(summary, french=False):
    """The rows of Table tab:remeshers: models run, meshed and meshed validly
    in every run, then the spiral and closed shares on the two subsets."""
    current, no_closing = summary["current"], summary.get("no_closing")

    def share(value):
        if value is None:
            return "--"
        text = f"{100 * value:.1f}"
        return text.replace(".", ",") + "\\,\\%" if french else text + "\\%"

    def shares(means):
        return [share(means[name][key]) for name in ("valid_in_every_run", "all_produce_a_mesh")
                for key in ("loop_edges_spiral", "loop_edges_closed")]

    rows = []
    for tool in current["subsets"]["valid_in_every_run"]["means"]:
        both = {name: subset["means"][tool] for name, subset in current["subsets"].items()}
        rows.append([NAMES[tool], str(current["run_on"][tool]), str(current["produce_a_mesh"][tool]),
                     str(current["valid_in_every_run"][tool]), *shares(both)])
        if tool == "openclare" and no_closing:
            rows.append(["\\quad sans fermeture" if french else "\\quad without closing", "--", "--", "--",
                         *shares(no_closing["means"])])
    return [" & ".join(row) + " \\\\" for row in rows]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    common.options(parser, "against_remeshers")
    parser.add_argument("--results", type=Path, default=common.ROOT / "bench" / "results" / "results.json")
    parser.add_argument("--git", action="append", default=[], help="also read the results at this commit")
    parser.add_argument("--no-closing-run", action="store_true",
                        help="make the line without closing on the engine given (Blender, the add-on)")
    parser.add_argument("--no-closing-from", type=Path,
                        help="make the line without closing from the runs this summary recorded")
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
    subsets = summary["current"]["subsets"]
    models = sorted(set(subsets["valid_in_every_run"]["models"]) | set(subsets["all_produce_a_mesh"]["models"]))

    def line_on_subsets(runs):
        line = {}
        for name, subset in subsets.items():
            per_model = {model: {key: median([runs[model]], key) for key in KEYS} for model in subset["models"]}
            line[name] = subset_means(per_model, subset["models"])
        return line

    if arguments.no_closing_from:
        recorded, runs = no_closing_from(arguments.no_closing_from, models)
        summary["no_closing"] = {
            **recorded,
            "read_from": str(arguments.no_closing_from.resolve().relative_to(common.ROOT)),
            "means": line_on_subsets(runs),
        }
    elif arguments.no_closing_run:
        runs = no_closing_run(models, results["target"], arguments.out, arguments.scenes)
        line = line_on_subsets(runs)
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
        print(f"{label}: run on {data['run_on']}, produce a mesh {data['produce_a_mesh']}, "
              f"valid in every run {data['valid_in_every_run']}")
        for name, subset in data["subsets"].items():
            print(f"  {name} ({len(subset['models'])}): "
                  + ", ".join(f"{tool} {percent(means['loop_edges_spiral'])}%"
                              for tool, means in subset["means"].items())
                  + "; openclare fewer/more/as many than "
                  + ", ".join(f"{tool} {c['fewer']}/{c['more']}/{c['as_many']}"
                              for tool, c in subset["openclare_against"].items()))
    if "no_closing" in summary:
        for name, means in summary["no_closing"]["means"].items():
            print(f"  no closing, {name}: {percent(means['loop_edges_spiral'])}%")
    for french, paper in ((False, "paper.tex"), (True, "paper.fr.tex")):
        print(f"Table tab:remeshers, {paper}:")
        for row in table(summary, french):
            print(f"  {row}")
    print(f"summary: {arguments.summary}")


if __name__ == "__main__":
    main()
