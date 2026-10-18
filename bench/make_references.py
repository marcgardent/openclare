#!/usr/bin/env python3
"""Computes and keeps the meshes of the local, unpublished challengers while they can be run.

    python3 bench/make_references.py [--targets 1000,3000,8000] [--models ogre,blob] [--tools name] [--refresh]

A challenger is a script in local/challengers (see UNPUBLISHED in run.py); its
meshes go to local/reference/<name>/, which is not tracked. Those already
there are kept unless --refresh is given.
"""

import argparse
import sys

from run import CHALLENGERS, MODELS, OUTPUT, UNPUBLISHED, available_models, challenger, ensure_corpus

parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
parser.add_argument("--targets", default="1000,3000,8000")
parser.add_argument("--models", default=",".join(available_models()))
parser.add_argument("--tools", default=",".join(name for name in CHALLENGERS if (UNPUBLISHED / "challengers" / f"{name}.py").exists()))
parser.add_argument("--refresh", action="store_true")
options = parser.parse_args()

tools = [name for name in options.tools.split(",") if name]
if not tools:
    sys.exit("no local challenger: put its script in local/challengers/<name>.py (see bench/README.md)")
wanted = options.models.split(",")
ensure_corpus(wanted)
for target in (int(t) for t in options.targets.split(",")):
    for model, _, sharp, boundary in MODELS:
        if model not in wanted:
            continue
        directory = OUTPUT / model
        directory.mkdir(parents=True, exist_ok=True)
        for name in tools:
            path, seconds, failure = challenger(name, options.refresh)(model, sharp, boundary, target, [0], directory)[0]
            label = f"{name} {model} {target}"
            print(f"{label:45} {'ok' if path else 'FAILED: ' + failure}  {seconds:.1f} s", flush=True)
