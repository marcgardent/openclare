#!/usr/bin/env python3
"""Table `tab:exact` and the whole closing of paragraph "Whole lengths are not
what is missing" (Section "An exact look at the closing system"): for each
way, the turns it holds, those its projection kept (the export's `kept`),
and the most held turns closable together in real lengths with every bound
of separation kept (a mixed-integer program, one binary per turn, big M).
Then, for the turns found closable together:

- `aimed-<way>.txt`: the lengths nearest the target closing them, in the
  weighted L1 sense (a linear program), for the engine to aim its integer
  search at (`OPENCLARE_AIMED`; Table `tab:rounding`, row "linear program");
- with --whole S: the same set closed exactly in whole numbers, nearest the
  target (HiGHS, stopped after S seconds), written as
  `way-<way>-integer.txt` for the engine to mesh (`OPENCLARE_LENGTHS`), with
  the held and the other turns it leaves shifted and its deviation against
  the engine's lengths;
- with --whole-most S: the program of the table itself over whole lengths
  (the papers': "did not finish within fifteen minutes").

    python3 docs/research/closing-loops/experiments/exact/most_closable.py [<export>] \\
        [--seconds 300] [--whole 600] [--whole-most 900]

The files for the engine go under --out/call-<name>/ (the T-mesh's name; an
export without one, as the drafts' old export, cannot be given back to today's
engine). Seconds a way for the table; --whole and --whole-most are long
(their limit a way, one thread). Writes results/most_closable.json.
"""

import argparse
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "turns-of-a-t-mesh" / "experiments"))
import tmesh  # noqa: E402  (first: it sets the threads of the linear algebra)

import numpy as np  # noqa: E402


def way_job(job):
    """One way, in a process of its own when --jobs asks for several."""
    path, arguments, files = job
    way = tmesh.Way(path)
    forms = tmesh.held_forms(way)
    held_numbers = sorted(way.held_index)
    chosen, result, seconds = way.most_closable(forms, time_limit=arguments.seconds)
    row = {"way": way.number, "held": len(forms), "kept_by_the_projection": len(way.kept),
           "closable_together": len(chosen), "status": result.message, "seconds": round(seconds, 1),
           "dual_bound": getattr(result, "mip_dual_bound", None)}
    x_aimed = None
    if chosen:
        _, x_aimed, _ = way.nearest([forms[k] for k in chosen])
        if x_aimed is not None:
            aimed = x_aimed / way.scale
            (files / f"aimed-{way.number}.txt").write_text(
                " ".join(f"{v:.12g}" for v in aimed) + "\n" + " ".join(str(held_numbers[k]) for k in chosen)
                + "\n")
            row["aimed"] = {"weighted_deviation": way.weighted_deviation(x_aimed),
                            "deviation": way.deviation(x_aimed),
                            "file": f"aimed-{way.number}.txt"}
    if arguments.whole and chosen:
        result_whole, x, took = way.nearest([forms[k] for k in chosen], integer=True, time_limit=arguments.whole)
        closing = {"seconds": round(took, 1), "message": result_whole.message, "found": x is not None}
        if x is not None:
            x = np.rint(x).astype(np.int64)
            others = [way.shift[k] for k in range(len(way.turns)) if not way.turns[k]["held"]]
            engine = way.lengths
            closing.update(
                held_left_shifted=way.shifted(forms, x), engine_held_left_shifted=way.shifted(forms, engine),
                not_held_left_shifted=way.shifted(others, x),
                engine_not_held_left_shifted=way.shifted(others, engine),
                weighted_deviation=way.weighted_deviation(x),
                engine_weighted_deviation=way.weighted_deviation(engine),
                deviation=way.deviation(x), engine_deviation=way.deviation(engine),
                quads=way.quads(x), engine_quads=way.quads(engine),
                file=f"way-{way.number}-integer.txt")
            closing["deviation_higher_by"] = round(
                closing["weighted_deviation"] / closing["engine_weighted_deviation"] - 1, 3)
            (files / f"way-{way.number}-integer.txt").write_text(" ".join(str(int(v)) for v in x) + "\n")
        row["whole_closing"] = closing
    if arguments.whole_most:
        chosen_whole, result_most, took = way.most_closable(forms, time_limit=arguments.whole_most, integer=True)
        row["whole_most_closable"] = {"found": len(chosen_whole) if result_most.x is not None else None,
                                      "dual_bound": getattr(result_most, "mip_dual_bound", None),
                                      "message": result_most.message, "seconds": round(took, 1)}
    print(f"way {way.number}: {row['held']} held, {row['kept_by_the_projection']} kept by the projection, "
          f"{row['closable_together']} closable together ({row['status']}, {row['seconds']} s)"
          + (f"; whole: {row['whole_closing']}" if "whole_closing" in row else "")
          + (f"; whole program: {row['whole_most_closable']}" if "whole_most_closable" in row else ""),
          flush=True)
    return row


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "most_closable", paper="closing-loops")
    parser.add_argument("--seconds", type=float, default=300.0, help="time limit of the program of the table")
    parser.add_argument("--whole", type=float, help="close the set in whole numbers, this many seconds a way")
    parser.add_argument("--whole-most", type=float, help="the program of the table in whole numbers, seconds")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    call = folder.name if folder.name.startswith("call-") else None
    files = arguments.out / call if call else arguments.out
    files.mkdir(parents=True, exist_ok=True)
    rows = tmesh.parallel(way_job, [(way.path, arguments, files) for way in ways], arguments.jobs)
    summary = {
        "experiment": "the most held turns closable together (Table tab:exact)",
        **tmesh.record(arguments, folder, ways, started, clock, seconds_per_program=arguments.seconds,
                       whole_seconds=arguments.whole, whole_most_seconds=arguments.whole_most,
                       files_for_the_engine=str(files.name) if call else None),
        "table": [{key: row[key] for key in ("way", "held", "kept_by_the_projection", "closable_together")}
                  for row in rows],
        "per_way": rows,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
