#!/usr/bin/env python3
"""A release: the next version number, written where it is kept, committed and tagged.

    python3 scripts/release.py fix           0.1.0 becomes 0.1.1
    python3 scripts/release.py minor         0.1.0 becomes 0.2.0
    python3 scripts/release.py major         0.1.0 becomes 1.0.0
    python3 scripts/release.py minor --push  and send the commit and the tag to `origin`

The number is kept in Cargo.toml (every crate takes it from the workspace), in
Cargo.lock and in the manifest of the Blender add-on. The commit is made on
`main`, from a working tree without changes, and tagged `v<version>`. Nothing
leaves this machine without `--push`; once the tag is on GitHub, the add-on is
built there for Linux and for Windows (.github/workflows/release.yml).
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CARGO = ROOT / "Cargo.toml"
MANIFEST = ROOT / "blender" / "openclare" / "blender_manifest.toml"
BRANCH = "main"
PARTS = ("major", "minor", "fix")

# The line of each file that holds the number: the one of the workspace in
# Cargo.toml, which comes after `[workspace.package]`, and the first of the
# manifest, which has only one at the start of a line.
IN_CARGO = re.compile(r'(\[workspace\.package\]\s*\nversion = ")(\d+\.\d+\.\d+)(")')
IN_MANIFEST = re.compile(r'(^version = ")(\d+\.\d+\.\d+)(")', re.MULTILINE)


def git(*arguments):
    return subprocess.run(["git", *arguments], cwd=ROOT, check=True, capture_output=True, text=True).stdout.strip()


def version_in(path, pattern):
    found = pattern.search(path.read_text())
    if not found:
        sys.exit(f"no version found in {path.relative_to(ROOT)}")
    return found.group(2)


def next_of(version, part):
    major, minor, fix = (int(number) for number in version.split("."))
    if part == "major":
        return f"{major + 1}.0.0"
    if part == "minor":
        return f"{major}.{minor + 1}.0"
    return f"{major}.{minor}.{fix + 1}"


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("part", type=str.lower, choices=PARTS, help="the number to raise; those after it go back to 0")
    parser.add_argument("--push", action="store_true", help="send the commit and the tag to `origin`")
    asked = parser.parse_args()

    if git("rev-parse", "--abbrev-ref", "HEAD") != BRANCH:
        sys.exit(f"a release is made from `{BRANCH}`: this is `{git('rev-parse', '--abbrev-ref', 'HEAD')}`")
    if git("status", "--porcelain"):
        sys.exit("a release is made from a working tree without changes: commit or put them aside first")

    current = version_in(CARGO, IN_CARGO)
    if version_in(MANIFEST, IN_MANIFEST) != current:
        sys.exit(f"Cargo.toml says {current}, the manifest of the add-on {version_in(MANIFEST, IN_MANIFEST)}")
    version = next_of(current, asked.part)
    tag = f"v{version}"
    if git("tag", "--list", tag):
        sys.exit(f"the tag {tag} is already there")

    for path, pattern in ((CARGO, IN_CARGO), (MANIFEST, IN_MANIFEST)):
        path.write_text(pattern.sub(rf"\g<1>{version}\g<3>", path.read_text(), count=1))
    # Cargo.lock names the crates of the workspace with their version. Nothing
    # else in it moves, and nothing is fetched.
    locked = subprocess.run(["cargo", "update", "--workspace", "--offline", "--quiet"], cwd=ROOT)
    if locked.returncode != 0:
        git("checkout", "--", ".")
        sys.exit("cargo could not write Cargo.lock again: nothing was changed")

    git("commit", "--all", "--message", f"Release {tag}")
    git("tag", "--annotate", tag, "--message", f"openclare {version}")
    print(f"{current} -> {version}: committed on `{BRANCH}` and tagged {tag}")
    if asked.push:
        subprocess.run(["git", "push", "--atomic", "origin", BRANCH, tag], cwd=ROOT, check=True)
        print(f"sent to `origin`: the add-on of {tag} is being built on GitHub")
    else:
        print(f"to have it built on GitHub:  git push --atomic origin {BRANCH} {tag}")
        print(f"to take it back:             git tag -d {tag} && git reset --hard HEAD~1")


if __name__ == "__main__":
    main()
