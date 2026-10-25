"""Checks the Lean T-mesh against every way of closing an engine export holds.

    python3 check_exports.py <export directory> <work directory> [--jobs N]

Finds every `way-<n>.json` under the export directory (written by the engine
with `OPENCLARE_EXPORT_TURNS`), converts each with `convert.py` into the work
directory, and runs `Main.lean` on them with `lake env lean --run`, `--jobs`
files at a time (one Lean process each, about 3 GB of memory). Exports out of
the scope of the Lean model (borders, patches that are not rectangles) are
reported as skipped. Exits with 1 if any check fails. Python standard library
only; run from anywhere, with `lake` on the PATH.
"""

import argparse
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
PROJECT = os.path.dirname(HERE)


def has_ways(folder):
    return os.path.isdir(folder) and any(
        name.startswith("way-") and name.endswith(".json") for name in os.listdir(folder))


def exports(directory, work):
    """The ways under `directory`, and the T-meshes (`call-<name>.json`) for which
    the engine wrote no way (as on surfaces with borders), those in `work` left
    out."""
    work = os.path.abspath(work)
    for root, folders, files in os.walk(directory):
        folders[:] = sorted(f for f in folders if os.path.abspath(os.path.join(root, f)) != work)
        for name in sorted(files):
            if name.startswith("way-") and name.endswith(".json") and not name.endswith("-helices.json"):
                yield os.path.join(root, name)
            elif name.startswith("call-") and name.endswith(".json") and not has_ways(
                    os.path.join(root, name[: -len(".json")])):
                yield os.path.join(root, name)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("export")
    parser.add_argument("work")
    parser.add_argument("--jobs", type=int, default=2)
    args = parser.parse_args()
    found = list(exports(args.export, args.work))
    if not found:
        sys.exit(f"no way-<n>.json under {args.export}")
    os.makedirs(args.work, exist_ok=True)
    converted, skipped = [], []
    for source in found:
        relative = os.path.relpath(source, args.export)
        target = os.path.join(os.path.abspath(args.work), relative.replace(os.sep, "--"))
        run = subprocess.run([sys.executable, os.path.join(HERE, "convert.py"), source, target],
                             capture_output=True, text=True)
        print(run.stdout.strip() or run.stderr.strip())
        if run.returncode == 3:
            skipped.append(relative)
        elif run.returncode != 0:
            sys.exit(f"conversion failed: {source}")
        else:
            converted.append(target)

    def check(target):
        return subprocess.run(["lake", "env", "lean", "--run", "Check/Main.lean", target],
                              cwd=PROJECT, capture_output=True, text=True)

    failed = 0
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for run in pool.map(check, converted):
            print(run.stdout, end="")
            if run.returncode != 0:
                print(run.stderr, end="")
                failed += 1
    print(f"{len(converted)} exports checked, {failed} failed, {len(skipped)} skipped (out of scope)")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
