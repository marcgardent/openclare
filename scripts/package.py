#!/usr/bin/env python3
"""The two archives of a release, for one system, from a library built for it.

    python3 scripts/package.py PLATFORM LIBRARY [IMPORT_LIBRARY]

    python3 scripts/package.py linux-x64 target/release/libopenclare_capi.so
    python3 scripts/package.py windows-x64 target/release/openclare_capi.dll target/release/openclare_capi.dll.lib
    python3 scripts/package.py macos-arm64 target/aarch64-apple-darwin/release/libopenclare_capi.dylib

writes into dist/:

- openclare-<version>-<platform>.zip, the Blender add-on (Community edition, as
  Blender installs it from disk): the files of blender/openclare, presets
  included, a manifest whose `platforms` names this system only, and the
  library in lib/;
- openclare-capi-<version>-<platform>.tar.gz, the library for C and other
  hosts: lib/ (the library; on Windows also its import library), include/
  openclare.h, LICENSE and README.md, in one folder of the same name.

The library is taken as it is: built, stripped and signed beforehand
(.github/workflows/release.yml). Plain Python, so that it runs the same on
the three systems.
"""

import argparse
import io
import re
import sys
import tarfile
import time
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ADDON = ROOT / "blender" / "openclare"
MANIFEST = ADDON / "blender_manifest.toml"
CAPI = ROOT / "crates" / "openclare-capi"
DIST = ROOT / "dist"

# The name the add-on loads the library by on each system (engine.py).
LIBRARIES = {
    "linux-x64": "libopenclare_capi.so",
    "windows-x64": "openclare_capi.dll",
    "macos-arm64": "libopenclare_capi.dylib",
    "macos-x64": "libopenclare_capi.dylib",
}
IMPORT_LIBRARY = "openclare_capi.dll.lib"

PLATFORMS_LINE = re.compile(r"^platforms = .*$", re.MULTILINE)
VERSION_LINE = re.compile(r'^version = "(\d+\.\d+\.\d+)"', re.MULTILINE)


def addon_files():
    """The files of the add-on to ship, as (path in the archive, path here):
    all of blender/openclare but the library built here and Python's caches."""
    for path in sorted(ADDON.rglob("*")):
        inside = path.relative_to(ADDON)
        if path.is_dir() or inside.parts[0] == "lib" or "__pycache__" in inside.parts or path.suffix == ".pyc":
            continue
        yield inside.as_posix(), path


def write_addon(platform, library, version, manifest):
    archive = DIST / f"openclare-{version}-{platform}.zip"
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as out:
        for inside, path in addon_files():
            if inside == "blender_manifest.toml":
                out.writestr(inside, manifest)
            else:
                out.write(path, inside)
        out.write(library, f"lib/{LIBRARIES[platform]}")
    return archive


def write_capi(platform, library, import_library, version):
    name = f"openclare-capi-{version}-{platform}"
    archive = DIST / f"{name}.tar.gz"
    readme = (CAPI / "README.md").read_text(encoding="utf-8")
    readme = readme.replace("# openclare C library", f"# openclare C library {version}, {platform}", 1)

    def add(out, inside, path=None, data=None, executable=False):
        info = tarfile.TarInfo(f"{name}/{inside}")
        if data is None:
            data = Path(path).read_bytes()
        info.size, info.mode = len(data), 0o755 if executable else 0o644
        info.mtime = int(time.time())
        out.addfile(info, io.BytesIO(data))

    with tarfile.open(archive, "w:gz") as out:
        add(out, f"lib/{LIBRARIES[platform]}", library, executable=not platform.startswith("windows"))
        if import_library:
            add(out, f"lib/{IMPORT_LIBRARY}", import_library)
        add(out, "include/openclare.h", CAPI / "include" / "openclare.h")
        add(out, "LICENSE", ROOT / "LICENSE")
        add(out, "README.md", data=readme.encode("utf-8"))
    return archive


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("platform", choices=sorted(LIBRARIES), help="the system, as Blender names it")
    parser.add_argument("library", type=Path, help="the shared library built for it")
    parser.add_argument("import_library", type=Path, nargs="?", help="on Windows, its import library (.dll.lib)")
    asked = parser.parse_args()

    if asked.library.name != LIBRARIES[asked.platform]:
        sys.exit(f"{asked.platform} loads {LIBRARIES[asked.platform]}, not {asked.library.name}")
    if asked.platform.startswith("windows") != (asked.import_library is not None):
        sys.exit("the import library comes with the Windows library, and only with it")
    for path in (asked.library, asked.import_library):
        if path is not None and not path.is_file():
            sys.exit(f"no file {path}")

    text = MANIFEST.read_text(encoding="utf-8")
    version = VERSION_LINE.search(text).group(1)
    if not PLATFORMS_LINE.search(text):
        sys.exit("the manifest of the add-on has no `platforms` line")
    manifest = PLATFORMS_LINE.sub(f'platforms = ["{asked.platform}"]', text, count=1)

    DIST.mkdir(exist_ok=True)
    for archive in (
        write_addon(asked.platform, asked.library, version, manifest),
        write_capi(asked.platform, asked.library, asked.import_library, version),
    ):
        print(f"written: {archive.relative_to(ROOT)} ({archive.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
