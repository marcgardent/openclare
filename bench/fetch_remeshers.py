#!/usr/bin/env python3
"""Downloads, and builds where need be, the open-source remeshers the bench
compares against that are not part of Blender.

    python3 bench/fetch_remeshers.py [--tools quadwild-bimdf,instant-meshes-jakob] [--jobs N]

Each comes from its official GitHub repository, at a pinned version, and
goes to `local/remeshers/<name>/` (not tracked: neither their sources nor
their binaries are part of this repository):

- `quadwild-bimdf`: QuadWild with Bi-MDF quantization (Heistermann et al.
  2023), the Linux binaries of its release v0.0.2, checked against their
  SHA-256. They are linked statically and built without Gurobi.
- `instant-meshes-jakob`: Instant Meshes (Jakob et al. 2015), the original,
  which has no release on GitHub: its source at a pinned commit, with the
  submodules that commit pins, built with CMake and the C++ compiler of the
  machine. It needs the X11 and OpenGL development files (its window is
  built in even though the bench runs it in batch mode).

`bench/run.py` looks for them there, or where the environment variables
named in `run.py` say.
"""

import argparse
import hashlib
import os
import shutil
import subprocess
import sys
import urllib.request
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REMESHERS = ROOT / "local" / "remeshers"
DOWNLOADS = REMESHERS / "downloads"

QUADWILD = {
    "repository": "https://github.com/cgg-bern/quadwild-bimdf",
    "tag": "v0.0.2",
    # The commit the tag points to.
    "commit": "1a24854abf7b17e5c9c0fba0f3c8c1930fd43957",
    "url": "https://github.com/cgg-bern/quadwild-bimdf/releases/download/v0.0.2/linux-binaries.zip",
    "file": "quadwild-bimdf-v0.0.2-linux-binaries.zip",
    "size": 5066088,
    "sha256": "f27d9f2ffebf4eafca4ca0ce45b21e5c6cc6fbee70dd8573f5e6dd7cd6b7ccc1",
}

INSTANT_MESHES = {
    "repository": "https://github.com/wjakob/instant-meshes.git",
    # The last commit of its master branch (3 November 2019).
    "commit": "7b3160864a2e1025af498c84cfed91cbfb613698",
}

# What a compiler and a CMake of today refuse in the sources of 2015-2019,
# each changed in place after checkout: (file, text, replaced by).
INSTANT_MESHES_PATCHES = [
    # CMake 4 no longer accepts the old behaviour of this policy.
    ("ext/nanogui/ext/glfw/CMakeLists.txt", "cmake_policy(SET CMP0042 OLD)", "cmake_policy(SET CMP0042 NEW)"),
    # The video mode extension of X11 is linked by name though nothing calls
    # it (GLFW only uses it as a fallback for gamma, and does not find it with
    # a recent CMake); its development link is often missing.
    ("ext/nanogui/CMakeLists.txt", "NANOGUI_EXTRA_LIBS GL Xxf86vm Xrandr", "NANOGUI_EXTRA_LIBS GL Xrandr"),
]
# GCC 13 and later make an error of a name the bundled TBB reuses; older
# CMake files are let through by the policy minimum.
INSTANT_MESHES_CMAKE = ["-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_POLICY_VERSION_MINIMUM=3.5", "-Wno-dev",
                        "-DCMAKE_CXX_FLAGS=-Wno-changes-meaning"]


def say(text):
    print(text, flush=True)


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as data:
        for block in iter(lambda: data.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def download(url, path, size, checksum):
    """Downloads `url` to `path`, unless it is there already, and checks it."""
    if not path.exists() or sha256(path) != checksum:
        path.parent.mkdir(parents=True, exist_ok=True)
        say(f"downloading {url}")
        partial = path.with_suffix(path.suffix + ".part")
        with urllib.request.urlopen(url) as response, open(partial, "wb") as out:
            shutil.copyfileobj(response, out)
        partial.replace(path)
    if path.stat().st_size != size or sha256(path) != checksum:
        sys.exit(f"{path.name}: not the expected file (size {path.stat().st_size}, SHA-256 {sha256(path)})")
    say(f"{path.name}: {size} bytes, SHA-256 checked")


def run(command, cwd=None):
    say("$ " + " ".join(str(part) for part in command))
    done = subprocess.run([str(part) for part in command], cwd=cwd)
    if done.returncode != 0:
        sys.exit(f"failed: {command[0]} (exit {done.returncode})")


def quadwild_bimdf(jobs):
    """The release binaries, unpacked with their configuration files."""
    archive = DOWNLOADS / QUADWILD["file"]
    download(QUADWILD["url"], archive, QUADWILD["size"], QUADWILD["sha256"])
    target = REMESHERS / "quadwild-bimdf"
    target.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(archive) as content:
        for member in content.namelist():
            # Nothing written outside the directory, whatever the archive says.
            if member.startswith("/") or ".." in Path(member).parts:
                sys.exit(f"{archive.name}: unexpected path {member}")
        content.extractall(target)
    for program in ("quadwild", "quad_from_patches"):
        (target / program).chmod(0o755)
    (target / "VERSION").write_text(f"{QUADWILD['repository']} {QUADWILD['tag']} {QUADWILD['commit']}\n")
    say(f"quadwild-bimdf {QUADWILD['tag']}: {target}")


def instant_meshes_jakob(jobs):
    """The source at the pinned commit, with its submodules, patched and built."""
    if not shutil.which("git") or not shutil.which("cmake"):
        sys.exit("instant-meshes-jakob needs git and cmake")
    target = REMESHERS / "instant-meshes-jakob"
    source, build = target / "source", target / "build"
    if not (source / ".git").exists():
        source.mkdir(parents=True, exist_ok=True)
        run(["git", "init", "-q"], cwd=source)
        run(["git", "remote", "add", "origin", INSTANT_MESHES["repository"]], cwd=source)
    run(["git", "fetch", "-q", "--depth", "1", "origin", INSTANT_MESHES["commit"]], cwd=source)
    # Back to the pinned state, patches undone, before applying them again.
    run(["git", "submodule", "foreach", "--recursive", "git checkout -q -- ."], cwd=source)
    run(["git", "checkout", "-q", "--force", INSTANT_MESHES["commit"]], cwd=source)
    run(["git", "submodule", "update", "--init", "--recursive", "--depth", "1"], cwd=source)
    for name, before, after in INSTANT_MESHES_PATCHES:
        path = source / name
        text = path.read_text()
        if before not in text:
            sys.exit(f"{name}: the text to patch is not there")
        path.write_text(text.replace(before, after))
    generator = ["-G", "Ninja"] if shutil.which("ninja") else []
    run(["cmake", "-S", source, "-B", build, *generator, *INSTANT_MESHES_CMAKE])
    run(["cmake", "--build", build, "--parallel", jobs])
    binary = target / "instant-meshes"
    shutil.copy2(build / "Instant Meshes", binary)
    (target / "VERSION").write_text(f"{INSTANT_MESHES['repository']} {INSTANT_MESHES['commit']}\n")
    say(f"instant-meshes-jakob {INSTANT_MESHES['commit'][:7]}: {binary}")


TOOLS = {"quadwild-bimdf": quadwild_bimdf, "instant-meshes-jakob": instant_meshes_jakob}


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--tools", default=",".join(TOOLS))
    parser.add_argument("--jobs", type=int, default=max(1, (os.cpu_count() or 2) // 2))
    options = parser.parse_args()
    tools = options.tools.split(",")
    unknown = [tool for tool in tools if tool not in TOOLS]
    if unknown:
        sys.exit(f"unknown tool: {', '.join(unknown)} (known: {', '.join(TOOLS)})")
    for tool in tools:
        TOOLS[tool](options.jobs)


if __name__ == "__main__":
    main()
