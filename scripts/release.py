#!/usr/bin/env python3
"""The version of openclare: raised in a change, tagged on `main` for a release.

    python3 scripts/release.py version fix      0.1.0 becomes 0.1.1
    python3 scripts/release.py version minor    0.1.0 becomes 0.2.0
    python3 scripts/release.py version major    0.1.0 becomes 1.0.0
    python3 scripts/release.py tag              tag `main` with the version it holds
    python3 scripts/release.py tag --push       and send the tag to `origin`

The number is kept in Cargo.toml (every crate takes it from the workspace), in
Cargo.lock and in the manifest of the Blender add-on. `main` changes only
through pull requests: the number is raised on the branch of a change, with
the change (a change of the add-on raises it, so that Blender sees a new
version), and goes to `main` when it is merged. A release is then a tag
`v<version>` on `main`, made from a working tree without changes; nothing
leaves this machine without `--push`. Once the tag is on GitHub, the add-on is
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


def raise_version(part):
    current = version_in(CARGO, IN_CARGO)
    if version_in(MANIFEST, IN_MANIFEST) != current:
        sys.exit(f"Cargo.toml says {current}, the manifest of the add-on {version_in(MANIFEST, IN_MANIFEST)}")
    version = next_of(current, part)
    for path, pattern in ((CARGO, IN_CARGO), (MANIFEST, IN_MANIFEST)):
        path.write_text(pattern.sub(rf"\g<1>{version}\g<3>", path.read_text(), count=1))
    # Cargo.lock names the crates of the workspace with their version. Nothing
    # else in it moves, and nothing is fetched.
    locked = subprocess.run(["cargo", "update", "--workspace", "--offline", "--quiet"], cwd=ROOT)
    if locked.returncode != 0:
        git("checkout", "--", str(CARGO), str(MANIFEST))
        sys.exit("cargo could not write Cargo.lock again: nothing was changed")
    print(f"{current} -> {version}, in Cargo.toml, Cargo.lock and the manifest of the add-on: commit it with the change")


def tag(push):
    if git("rev-parse", "--abbrev-ref", "HEAD") != BRANCH:
        sys.exit(f"a release is tagged on `{BRANCH}`: this is `{git('rev-parse', '--abbrev-ref', 'HEAD')}`")
    if git("status", "--porcelain", "--untracked-files=no"):
        sys.exit("a release is tagged from a working tree without changes: commit or put them aside first")
    git("fetch", "--quiet", "origin", BRANCH)
    if git("rev-parse", "HEAD") != git("rev-parse", f"origin/{BRANCH}"):
        sys.exit(f"`{BRANCH}` here is not `origin/{BRANCH}`: pull first")
    version = version_in(CARGO, IN_CARGO)
    if version_in(MANIFEST, IN_MANIFEST) != version:
        sys.exit(f"Cargo.toml says {version}, the manifest of the add-on {version_in(MANIFEST, IN_MANIFEST)}")
    name = f"v{version}"
    if git("tag", "--list", name):
        sys.exit(f"the tag {name} is already there: raise the version in a change first (make version)")
    git("tag", "--annotate", name, "--message", f"openclare {version}")
    print(f"{BRANCH} tagged {name}")
    if push:
        subprocess.run(["git", "push", "origin", name], cwd=ROOT, check=True)
        print(f"sent to `origin`: the add-on of {name} is being built on GitHub")
    else:
        print(f"to have it built on GitHub:  git push origin {name}")
        print(f"to take it back:             git tag -d {name}")


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    asked = parser.add_subparsers(dest="action", required=True)
    raising = asked.add_parser("version", help="raise the version in the sources")
    raising.add_argument("part", type=str.lower, choices=PARTS, help="the number to raise; those after it go back to 0")
    tagging = asked.add_parser("tag", help="tag main with its version")
    tagging.add_argument("--push", action="store_true", help="send the tag to `origin`")
    asked = parser.parse_args()
    if asked.action == "version":
        raise_version(asked.part)
    else:
        tag(asked.push)


if __name__ == "__main__":
    main()
