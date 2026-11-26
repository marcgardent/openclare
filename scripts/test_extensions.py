#!/usr/bin/env python3
"""The extension repository (scripts/extensions.py, #344), on made-up add-ons
of four systems. Without Blender: the listing, its checks and the page. With
`blender` on the PATH, besides: the listing against the one that
`blender --command extension server-generate` writes from the same archives,
and Blender itself, without interface and with a user configuration of its
own, adding the repository, installing the archive of its system, and
updating it to the next release. Run with python3 scripts/test_extensions.py
(make extensions-test)."""

import importlib.util
import json
import os
import pathlib
import shutil
import subprocess
import sys
import tempfile
import zipfile

HERE = pathlib.Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("extensions", HERE / "extensions.py")
extensions = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(extensions)

PLATFORMS = ["linux-x64", "windows-x64", "macos-arm64", "macos-x64"]
ID = "openclare_probe"


def addon(folder, version, platform, prefix="openclare-"):
    """A made-up add-on archive, its manifest shaped as openclare's, its
    module saying which system and version it was built for."""
    manifest = f'''schema_version = "1.0.0"

id = "{ID}"
version = "{version}"
name = "openclare probe"
tagline = "A made-up add-on for the tests of the repository"
maintainer = "test"
type = "add-on"
tags = ["Mesh"]

blender_version_min = "4.2.0"

license = ["SPDX:GPL-3.0-or-later"]
copyright = ["2026 test"]

# One system per archive, as make addon builds them.
platforms = ["{platform}"]

[build]
paths_exclude_pattern = ["__pycache__/", "*.pyc"]
'''
    module = f'BUILT = ("{platform}", "{version}")\n\n\ndef register():\n    pass\n\n\ndef unregister():\n    pass\n'
    path = pathlib.Path(folder) / f"{prefix}{version}-{platform}.zip"
    with zipfile.ZipFile(path, "w") as archive:
        archive.writestr("blender_manifest.toml", manifest)
        archive.writestr("__init__.py", module)
    return path


def release(folder, version):
    folder = pathlib.Path(folder)
    folder.mkdir(parents=True, exist_ok=True)
    for platform in PLATFORMS:
        addon(folder, version, platform)
    # The C library's archive of a release is not an extension.
    (folder / f"openclare-capi-{version}-linux-x64.zip").write_bytes(b"not an extension")
    return folder


def refused(call):
    try:
        call()
    except SystemExit:
        return True
    return False


def test_listing(root):
    archives = extensions.addon_archives(release(root / "in", "1.0.0"))
    assert [path.name for path in archives] == sorted(f"openclare-1.0.0-{platform}.zip" for platform in PLATFORMS)
    out = root / "out"
    index = extensions.write(out, archives)
    written = json.loads((out / "index.json").read_text())
    assert written == index and written["version"] == "v1" and written["blocklist"] == []
    assert sorted(item["platforms"][0] for item in written["data"]) == sorted(PLATFORMS)
    for item in written["data"]:
        archive = out / item["archive_url"][2:]
        assert archive.is_file(), archive
        assert item["archive_size"] == archive.stat().st_size
        assert item["archive_hash"] == "sha256:" + extensions.sha256(archive)
        assert item["id"] == ID and item["version"] == "1.0.0" and "build" not in item
        assert item["tags"] == ["Mesh"] and item["copyright"] == ["2026 test"]
    text = (out / "index.html").read_text()
    for platform in PLATFORMS:
        assert f"./openclare-1.0.0-{platform}.zip?repository=.%2Findex.json&amp;blender_version_min=4.2.0&amp;platforms={platform}" in text
    print("listing: four systems, sizes, hashes, the page's links")


def test_refusals(root):
    mixed = root / "mixed"
    mixed.mkdir()
    addon(mixed, "1.0.0", "linux-x64")
    addon(mixed, "1.0.1", "windows-x64")
    assert refused(lambda: extensions.listing(extensions.addon_archives(mixed)))
    twice = root / "twice"
    twice.mkdir()
    addon(twice, "1.0.0", "linux-x64")
    os.rename(twice / "openclare-1.0.0-linux-x64.zip", twice / "openclare-1.0.0-linux-x64-a.zip")
    addon(twice, "1.0.0", "linux-x64")
    assert refused(lambda: extensions.listing(extensions.addon_archives(twice)))
    assert refused(lambda: extensions.listing([]))
    print("refused: two versions, two archives for one system, no archive")


def blender(user, *arguments):
    """Blender without interface, with the user configuration in `user`."""
    environment = dict(os.environ, BLENDER_USER_RESOURCES=str(user))
    done = subprocess.run(
        ["blender", "--online-mode", "--command", "extension", *arguments],
        env=environment, capture_output=True, text=True, timeout=300,
    )
    return done.stdout + done.stderr


def installed(user):
    """(system, version) of the probe installed in `user`, or None."""
    found = list(pathlib.Path(user).glob(f"extensions/*/{ID}/__init__.py"))
    if not found:
        return None
    namespace = {}
    exec(found[0].read_text(), namespace)
    return namespace["BUILT"]


def this_system():
    machine = os.uname().machine if hasattr(os, "uname") else ""
    if sys.platform.startswith("linux"):
        return "linux-x64"
    if sys.platform == "darwin":
        return "macos-arm64" if machine == "arm64" else "macos-x64"
    return "windows-x64"


def test_against_blender(root):
    archives = extensions.addon_archives(release(root / "ours", "1.0.0"))
    ours = extensions.write(root / "ours", archives)
    theirs_folder = root / "theirs"
    theirs_folder.mkdir()
    for archive in archives:
        shutil.copyfile(archive, theirs_folder / archive.name)
    output = blender(root / "user-generate", "server-generate", f"--repo-dir={theirs_folder}")
    theirs = json.loads((theirs_folder / "index.json").read_text())
    by_system = lambda index: sorted(index["data"], key=lambda item: item["platforms"])
    assert by_system(ours) == by_system(theirs), (output, by_system(ours), by_system(theirs))
    print("against server-generate: the same entries")


def test_in_blender(root):
    system = this_system()
    served = root / "served"
    extensions.write(served, extensions.addon_archives(release(root / "first", "1.0.0")))
    user = root / "user"
    url = served.resolve().as_uri() + "/index.json"
    # --clear-all: our repository alone, extensions.blender.org not synced.
    blender(user, "repo-add", "--clear-all", "--name", "openclare probe", "--url", url, "probe")
    blender(user, "install", "--sync", ID)
    assert installed(user) == (system, "1.0.0"), installed(user)
    # The next release: the repository served again from its archives only.
    for path in served.iterdir():
        path.unlink()
    extensions.write(served, extensions.addon_archives(release(root / "second", "1.1.0")))
    output = blender(user, "update", "--sync")
    assert installed(user) == (system, "1.1.0"), (installed(user), output)
    print(f"in Blender: {system} installed from the repository, then updated to the next release")


def main():
    with tempfile.TemporaryDirectory() as folder:
        root = pathlib.Path(folder)
        for test in (test_listing, test_refusals):
            place = root / test.__name__
            place.mkdir()
            test(place)
        if shutil.which("blender"):
            for test in (test_against_blender, test_in_blender):
                place = root / test.__name__
                place.mkdir()
                test(place)
        else:
            print("no blender on the PATH: the comparison with server-generate and the run in Blender are skipped")


if __name__ == "__main__":
    main()
