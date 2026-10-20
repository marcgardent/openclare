#!/usr/bin/env python3
"""The version of openclare: raised in a change, released from `main` on GitHub.

    python3 scripts/release.py version fix      0.1.0 becomes 0.1.1
    python3 scripts/release.py version minor    0.1.0 becomes 0.2.0
    python3 scripts/release.py version major    0.1.0 becomes 1.0.0
    python3 scripts/release.py release          draft a release of `main` on GitHub

The number is kept in Cargo.toml (every crate takes it from the workspace), in
Cargo.lock and in the manifest of the Blender add-on. `main` changes only
through pull requests: the number is raised on the branch of a change, with
the change (a change of the add-on raises it, so that Blender sees a new
version), and goes to `main` when it is merged.

A release is made on GitHub, by .github/workflows/release.yml run on `main`
with `release=true`: from the Actions tab, or by `release` here, which asks
GitHub for it through the `gh` command. It builds the add-on and the C library
for the four systems and makes of them a DRAFT release `v<version>` of the
commit, to read and publish by hand (Zenodo archives every published release).
Nothing is tagged here: the tag is made on GitHub when the draft is published.
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


def release():
    """Asks GitHub for a draft release of `origin/main`, with the version it holds."""
    git("fetch", "--quiet", "origin", BRANCH)
    held = git("show", f"origin/{BRANCH}:{MANIFEST.relative_to(ROOT).as_posix()}")
    found = IN_MANIFEST.search(held)
    if not found:
        sys.exit(f"no version found in the manifest of the add-on on origin/{BRANCH}")
    name = f"v{found.group(2)}"
    if git("ls-remote", "--tags", "origin", f"refs/tags/{name}"):
        sys.exit(f"{name} is already released: raise the version in a change first (make version)")
    asked = ["gh", "workflow", "run", "release.yml", "--ref", BRANCH, "-f", "release=true"]
    try:
        subprocess.run(asked, cwd=ROOT, check=True)
    except FileNotFoundError:
        sys.exit("no `gh` here: run the workflow `release` from the Actions tab of the repository, release ticked")
    except subprocess.CalledProcessError:
        sys.exit("GitHub did not start the workflow (see above)")
    print(f"building {name} on GitHub: gh run watch, or the Actions tab. Once done, the draft {name} is")
    print("under Releases: read it, then publish it there (Zenodo archives it), or delete it.")


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    asked = parser.add_subparsers(dest="action", required=True)
    raising = asked.add_parser("version", help="raise the version in the sources")
    raising.add_argument("part", type=str.lower, choices=PARTS, help="the number to raise; those after it go back to 0")
    asked.add_parser("release", help="draft a release of origin/main on GitHub (needs gh)")
    asked = parser.parse_args()
    if asked.action == "version":
        raise_version(asked.part)
    else:
        release()


if __name__ == "__main__":
    main()
