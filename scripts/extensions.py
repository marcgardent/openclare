#!/usr/bin/env python3
"""The extension repository of openclare: a listing that Blender reads, so
that it offers each new release of the add-on (#344).

    python3 scripts/extensions.py --out site/_build/extensions
    python3 scripts/extensions.py --out DIR --archives DIR_OF_ZIPS

The first form takes the latest published release on GitHub (never a draft),
downloads its add-on archives, one per system, checks them against its
SHA256SUMS and copies them into `--out`. The second takes archives already
on disk, for the tests or for a build not yet released. Either way `--out`
then holds the archives, `index.json` and `index.html`.

`index.json` is Blender's Extension Listing API v1, as
`blender --command extension server-generate` writes it: one entry per
archive, each the manifest of the archive without its `[build]` table, with
the archive's relative URL, size and SHA-256. Blender keeps the entries of
its own system, so one URL serves Linux, Windows and macOS. `index.html`
lists the archives, each a link that can be dropped into Blender.

The user adds the repository once: Preferences, Get Extensions,
Repositories, +, Add Remote Repository, the URL of `index.json`. Standard
library only: it runs in the build of the site (.github/workflows/pages.yml),
with no Blender there.
"""

import argparse
import hashlib
import html
import json
import os
import shutil
import sys
import tomllib
import urllib.parse
import urllib.request
import zipfile
from pathlib import Path

REPOSITORY = "marcgardent/openclare"
API = "https://api.github.com"
# The add-on's archives in a release: openclare-<version>-<platform>.zip. The
# C library's archives (openclare-capi-…) are not extensions.
PREFIX = "openclare-"
NOT_ADDON = "openclare-capi-"
SUMS = "SHA256SUMS"


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as file:
        for block in iter(lambda: file.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def manifest_of(archive):
    """The manifest at the root of an extension's archive."""
    with zipfile.ZipFile(archive) as extension:
        try:
            text = extension.read("blender_manifest.toml").decode("utf-8")
        except KeyError:
            sys.exit(f"{archive.name}: no blender_manifest.toml at the root of the archive")
    return tomllib.loads(text)


def entry(archive):
    """The listing's entry for one archive, as server-generate makes it."""
    manifest = manifest_of(archive)
    for key in ("id", "version", "platforms"):
        if key not in manifest:
            sys.exit(f"{archive.name}: the manifest has no `{key}`")
    listed = {key: value for key, value in manifest.items() if key != "build"}
    listed["archive_url"] = "./" + urllib.parse.quote(archive.name)
    listed["archive_size"] = archive.stat().st_size
    listed["archive_hash"] = "sha256:" + sha256(archive)
    return listed


def addon_archives(folder):
    return sorted(
        path for path in Path(folder).glob("*.zip") if path.name.startswith(PREFIX) and not path.name.startswith(NOT_ADDON)
    )


def listing(archives):
    """index.json: every archive, all of one id and one version."""
    entries = [entry(archive) for archive in archives]
    if not entries:
        sys.exit("no add-on archive (openclare-<version>-<platform>.zip) to list")
    versions = {(item["id"], item["version"]) for item in entries}
    if len(versions) != 1:
        sys.exit(f"the archives are not of one release: {sorted(versions)}")
    platforms = [platform for item in entries for platform in item["platforms"]]
    if len(platforms) != len(set(platforms)):
        sys.exit(f"two archives for one system: {sorted(platforms)}")
    return {"version": "v1", "blocklist": [], "data": entries}


def drop_link(item):
    """The archive's link that Blender takes when it is dropped on it."""
    query = {"repository": "./index.json", "blender_version_min": item["blender_version_min"]}
    if "blender_version_max" in item:
        query["blender_version_max"] = item["blender_version_max"]
    query["platforms"] = ",".join(item["platforms"])
    return item["archive_url"] + "?" + urllib.parse.urlencode(query)


def page(index):
    """index.html: what the repository holds, and how to add it."""
    first = index["data"][0]
    rows = "\n".join(
        f'<tr><td>{html.escape(", ".join(item["platforms"]))}</td>'
        f'<td><a href="{html.escape(drop_link(item))}">{html.escape(urllib.parse.unquote(item["archive_url"][2:]))}</a></td>'
        f'<td>{item["archive_size"] / 1e6:.1f} MB</td></tr>'
        for item in sorted(index["data"], key=lambda item: item["platforms"])
    )
    name, version = html.escape(first["name"]), html.escape(first["version"])
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{name} extension repository</title>
<style>
body {{ font-family: system-ui, sans-serif; max-width: 46rem; margin: 2rem auto; padding: 0 1rem; line-height: 1.5; }}
code {{ background: #eee; padding: 0 .25em; }}
table {{ border-collapse: collapse; }}
td, th {{ padding: .25rem 1rem .25rem 0; text-align: left; }}
</style>
</head>
<body>
<h1>{name} {version}</h1>
<p>{html.escape(first.get("tagline", ""))}. Blender {html.escape(first["blender_version_min"])} or later.</p>
<h2>Add the repository to Blender</h2>
<p><em>Edit › Preferences › Get Extensions › Repositories</em>, <em>+</em>, <em>Add Remote Repository</em>,
and paste the address of <a href="index.json">index.json</a>, this page's address with
<code>index.json</code> in place of <code>index.html</code>. Blender then installs the archive of its own
system from <em>Get Extensions</em>, and offers each new release.</p>
<h2>Or drop one archive into Blender</h2>
<table>
<tr><th>System</th><th>Archive</th><th>Size</th></tr>
{rows}
</table>
</body>
</html>
"""


def write(out, archives):
    out.mkdir(parents=True, exist_ok=True)
    for archive in archives:
        if archive.parent.resolve() != out.resolve():
            shutil.copyfile(archive, out / archive.name)
    copied = [out / archive.name for archive in archives]
    index = listing(copied)
    (out / "index.json").write_text(json.dumps(index, indent=1) + "\n")
    (out / "index.html").write_text(page(index))
    return index


def fetch(url, token=None, accept=None):
    request = urllib.request.Request(url, headers={"User-Agent": "openclare-extensions"})
    if accept:
        request.add_header("Accept", accept)
    if token and url.startswith(API):
        request.add_header("Authorization", f"Bearer {token}")
    with urllib.request.urlopen(request, timeout=120) as response:
        return response.read()


def from_release(repository, folder, token):
    """The add-on's archives of the latest published release, checked against
    its SHA256SUMS, downloaded into `folder`."""
    release = json.loads(fetch(f"{API}/repos/{repository}/releases/latest", token, "application/vnd.github+json"))
    assets = {asset["name"]: asset["browser_download_url"] for asset in release["assets"]}
    if SUMS not in assets:
        sys.exit(f"release {release['tag_name']}: no {SUMS}")
    sums = {}
    for line in fetch(assets[SUMS]).decode().splitlines():
        digest, _, name = line.strip().partition("  ")
        if name:
            sums[name.lstrip("*")] = digest
    names = [name for name in assets if name.endswith(".zip") and name.startswith(PREFIX) and not name.startswith(NOT_ADDON)]
    if not names:
        sys.exit(f"release {release['tag_name']}: no add-on archive")
    folder.mkdir(parents=True, exist_ok=True)
    for name in sorted(names):
        path = folder / name
        path.write_bytes(fetch(assets[name]))
        if sums.get(name) != sha256(path):
            sys.exit(f"release {release['tag_name']}: {name} does not match {SUMS}")
    print(f"release {release['tag_name']}: {len(names)} add-on archives, checked against {SUMS}")
    return [folder / name for name in sorted(names)]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--out", required=True, help="the folder of the repository: archives, index.json, index.html")
    parser.add_argument("--archives", help="take the add-on archives of this folder instead of the latest release")
    parser.add_argument("--repository", default=os.environ.get("GITHUB_REPOSITORY", REPOSITORY), help="owner/name on GitHub")
    arguments = parser.parse_args()
    out = Path(arguments.out)
    if arguments.archives:
        archives = addon_archives(arguments.archives)
    else:
        token = os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN")
        archives = from_release(arguments.repository, out, token)
    index = write(out, archives)
    first = index["data"][0]
    systems = ", ".join(platform for item in index["data"] for platform in item["platforms"])
    print(f"written: {out / 'index.json'}, {first['id']} {first['version']} for {systems}")


if __name__ == "__main__":
    main()
