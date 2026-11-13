#!/usr/bin/env python3
"""Gathers the files of a lab batch's jobs into one folder: the merge of a
split run (README, "Run as lab batches"). Each job of the batch ran a script
on one cell (one model and budget, one line, one variant, one way) with
`--out {out}`, its own empty folder; `lab report` lists the cells in
`records.json`. Every file of each cell that ended ok is copied into
--into at the same path relative to the cell's folder, so that --into is
the script's own --out, and the script run there on the whole grid reads
every run back and makes the summary, nothing run again.

    python3 docs/research/closing-loops/experiments/collect.py <records.json> \\
        --into scratch/data-package/closing-loops/twelve_models [--exclude summary.json] [--force]

A file already in --into with other bytes, or two cells giving one path
other bytes, stops the copy before anything is copied, unless --force (the
cell listed last wins). The summaries the jobs wrote in their folder
(`summary.json`) are left out by default (--exclude, a pattern on the path
or the name, may be given several times; --exclude '' copies everything).
Says what was copied, what was there already, and the cells missing (never
run, not ok, folder gone); exits 1 if a cell is missing or a file differs
(without --force), so that `collect.py … && <script>` does not make the
missing runs itself.
"""

import argparse
import filecmp
import fnmatch
import json
import shutil
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records", type=Path, help="the records.json of `lab report <batch>`")
    parser.add_argument("--into", type=Path, required=True, help="the folder the files go to (the script's --out)")
    parser.add_argument("--exclude", action="append",
                        help="leave out the files whose path or name matches (default: summary.json)")
    parser.add_argument("--force", action="store_true", help="replace a file that differs")
    arguments = parser.parse_args()
    excluded = [pattern for pattern in (arguments.exclude if arguments.exclude is not None else ["summary.json"])
                if pattern]

    records = json.loads(arguments.records.read_text())
    missing, plan, left_out, commits = [], {}, 0, set()
    for cell in records["cells"]:
        values = " ".join(f"{axis}={value}" for axis, value in cell["values"].items())
        if cell.get("run") is None:
            missing.append(f"{values}: never run")
            continue
        if cell.get("status") != "ok":
            missing.append(f"{values}: {cell.get('status')} ({cell['run']})")
            continue
        folder = Path(cell["out"])
        if not folder.is_dir():
            missing.append(f"{values}: no folder {folder}")
            continue
        commits.add(cell.get("commit"))
        for path in sorted(folder.rglob("*")):
            if not path.is_file():
                continue
            relative = path.relative_to(folder)
            if any(fnmatch.fnmatch(str(relative), pattern) or fnmatch.fnmatch(path.name, pattern)
                   for pattern in excluded):
                left_out += 1
                continue
            plan.setdefault(relative, []).append((values, path))

    copies, same, conflicts = [], 0, []
    for relative, sources in plan.items():
        values, path = sources[-1]
        others = [other for other, given in sources[:-1] if not filecmp.cmp(given, path, shallow=False)]
        if others:
            conflicts.append(f"{relative}: other bytes from {', '.join(others)} and {values}")
        target = arguments.into / relative
        if target.exists():
            if filecmp.cmp(target, path, shallow=False):
                same += 1
                continue
            conflicts.append(f"{relative}: {target} has other bytes than {values}'s")
        copies.append((relative, path, target))

    print(f"{records.get('batch')}: {len(records['cells'])} cells, {len(records['cells']) - len(missing)} ok; "
          f"{len(plan)} files, {same} already in {arguments.into}, {left_out} left out ({', '.join(excluded)})")
    if len(commits) > 1:
        print(f"warning: the cells ran on several commits: {', '.join(sorted(map(str, commits)))}")
    for line in conflicts:
        print(f"differs: {line}")
    if conflicts and not arguments.force:
        print("nothing copied: --force replaces the files that differ")
        sys.exit(1)
    for relative, path, target in copies:
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(path, target)
        print(f"copied {relative}")
    print(f"{len(copies)} copied into {arguments.into}")
    for line in missing:
        print(f"missing: {line}")
    if missing:
        sys.exit(1)


if __name__ == "__main__":
    main()
