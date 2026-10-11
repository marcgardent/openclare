#!/usr/bin/env python3
"""A quick look at what a change does: one run of the add-on per model, several at a time.

    python3 bench/quick.py [--models camera,wolf] [--target 3000] [--jobs 4]

For each model, what the bench has stored for `openclare` and what the engine
as it is built now gives: faces, validity, Hausdorff and mean distance (per
mille of the diagonal), seconds, and the quads added for thin pieces. Nothing
stored is touched: this is for the work in hand, in a couple of minutes, where
the bench proper (bench/run.py) runs every tool at every budget and takes an
hour. Its files go to bench/out/quick.
"""

import argparse
import json
from concurrent.futures import ThreadPoolExecutor

from run import DEFAULT_TARGET, MODELS, OUTPUT, RESULTS, TOOLS, available_models, measure


def shown(metrics):
    if not metrics:
        return "nothing"
    valid = "valid" if metrics["valid"] else "invalid"
    return (f"{metrics['faces']:6d} faces, {valid:7}, Hausdorff {1000 * metrics['hausdorff']:5.1f}, "
            f"mean {1000 * metrics['mean_distance']:5.2f}")


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--models", default=",".join(available_models()))
    parser.add_argument("--target", type=int, default=DEFAULT_TARGET)
    parser.add_argument("--jobs", type=int, default=4, help="runs at a time; each may take a few gigabytes")
    options = parser.parse_args()
    wanted = options.models.split(",")
    suffix = "" if options.target == DEFAULT_TARGET else f"-{options.target}"
    stored_path = RESULTS / f"results{suffix}.json"
    stored = json.loads(stored_path.read_text())["runs"] if stored_path.exists() else {}

    def one(entry):
        model, _, sharp, boundary = entry
        directory = OUTPUT / "quick" / model
        directory.mkdir(parents=True, exist_ok=True)
        path, seconds, failure, _, engine = TOOLS["openclare"](model, sharp, boundary, options.target, [1], directory)[1]
        before = (stored.get(model, {}).get("openclare") or [{}])[0]
        now = shown(measure(model, path, options.target)) if path else failure
        added = f", {engine['quads_for_thin_pieces']} quads for {engine['thin_pieces']} thin pieces" if engine else ""
        return (f"{model:10} stored {shown(before.get('metrics'))}, {before.get('seconds', 0):5.1f} s\n"
                f"{'':10} now    {now}, {seconds:5.1f} s{added}")

    with ThreadPoolExecutor(options.jobs) as pool:
        for lines in pool.map(one, [entry for entry in MODELS if entry[0] in wanted]):
            print(lines, flush=True)


if __name__ == "__main__":
    main()
