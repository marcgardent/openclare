#!/usr/bin/env python3
"""Computes and stores Quad Remesher results while a licence is available.

    python3 bench/make_references.py [--targets 1000,3000,8000] [--models ogre,blob] [--refresh]

Results already in bench/reference are kept unless --refresh is given.
"""

import argparse

from run import MODELS, OUTPUT, available_models, ensure_corpus, quadremesher, quadremesher_reference

parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
parser.add_argument("--targets", default="1000,3000,8000")
parser.add_argument("--models", default=",".join(available_models()))
parser.add_argument("--refresh", action="store_true")
options = parser.parse_args()

wanted = options.models.split(",")
ensure_corpus(wanted)
for target in (int(t) for t in options.targets.split(",")):
    for model, _, sharp, boundary in MODELS:
        if model not in wanted:
            continue
        directory = OUTPUT / model
        directory.mkdir(parents=True, exist_ok=True)
        for uniform in (False, True):
            runs = quadremesher(uniform, options.refresh)(model, sharp, boundary, target, [0], directory)
            path, seconds, failure = runs[0]
            name = quadremesher_reference(model, target, uniform).name
            print(f"{name:45} {'ok' if path else 'FAILED: ' + failure}  {seconds:.1f} s", flush=True)
