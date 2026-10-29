"""Checks the Lean T-mesh against every way of closing an engine export holds.

    python3 check_exports.py <export directory> <work directory> [--jobs N]

Finds every `way-<n>.json` under the export directory (written by the engine
with `OPENCLARE_EXPORT_TURNS`), converts each with `convert.py` into the work
directory, and runs `Main.lean` on them, compiled (`.lake/build/bin/check`,
from `lake build check`) when it is there, else with `lake env lean --run`,
`--jobs` files at a time (one process each, about 3 GB of memory
interpreted). Exports out of
the scope of the Lean model (borders, patches that are not rectangles) are
reported as skipped. Exits with 1 if any check fails. Python standard library
only; run from anywhere, with `lake` on the PATH.

The certificates of Theorem "Forced shifts" (written by `certify.py`) are read
from `Check/certificates/*.json` (but the `*.claims.json` they were made from)
and from any `certificates.json` under the export directory; each applies to
the way file whose SHA-256 it names.
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
    kept = os.path.join(HERE, "certificates")
    certificates = sorted(os.path.join(kept, f) for f in os.listdir(kept)
                          if f.endswith(".json") and not f.endswith(".claims.json"))
    for root, _, files in os.walk(args.export):
        if "certificates.json" in files:
            certificates.append(os.path.join(root, "certificates.json"))
    os.makedirs(args.work, exist_ok=True)
    converted, skipped = [], []
    for source in found:
        relative = os.path.relpath(source, args.export)
        target = os.path.join(os.path.abspath(args.work), relative.replace(os.sep, "--"))
        run = subprocess.run([sys.executable, os.path.join(HERE, "convert.py"), source, target]
                             + certificates, capture_output=True, text=True)
        print(run.stdout.strip() or run.stderr.strip())
        if run.returncode == 3:
            skipped.append(relative)
        elif run.returncode != 0:
            sys.exit(f"conversion failed: {source}")
        else:
            converted.append(target)

    # The program compiled (`lake build check`) when it is there, else interpreted.
    binary = os.path.join(PROJECT, ".lake", "build", "bin", "check")
    command = [binary] if os.path.exists(binary) else ["lake", "env", "lean", "--run", "Check/Main.lean"]

    def check(target):
        return subprocess.run(command + [target],
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
