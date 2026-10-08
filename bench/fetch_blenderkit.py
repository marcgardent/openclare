#!/usr/bin/env python3
"""Downloads the BlenderKit models of the bench that cannot be redistributed.

    python3 bench/fetch_blenderkit.py [--models terrain,marine] [--refresh]

They are listed in `BLENDERKIT` of run.py, by `asset_base_id`. Each is
downloaded as its .blend into bench/corpus-large/blenderkit/ and exported to
bench/corpus-large/<name>.obj by import_blenderkit.py; neither directory is
tracked. Needs `blender` on the PATH for the export.

Only free assets can be fetched this way, without an account. The licence of
each is printed; BlenderKit's Royalty Free licence does not allow passing a
model on, even modified: keep these files, and what is remeshed from them,
out of the repository. See CREDITS.md.
"""

import argparse
import json
import shutil
import subprocess
import sys
import urllib.request
import uuid

from run import BENCH, BLENDERKIT, LOCAL, source_path

API = "https://www.blenderkit.com/api/v1/search/?query=asset_base_id:"
AGENT = {"User-Agent": "openclare-bench"}


def fetch_json(url):
    with urllib.request.urlopen(urllib.request.Request(url, headers=AGENT), timeout=60) as response:
        return json.load(response)


def download(url, path):
    partial = path.with_suffix(".part")
    with urllib.request.urlopen(urllib.request.Request(url, headers=AGENT), timeout=60) as response:
        total = int(response.headers.get("Content-Length") or 0)
        with open(partial, "wb") as out:
            shutil.copyfileobj(response, out, 1 << 20)
    if total and partial.stat().st_size != total:
        partial.unlink()
        sys.exit(f"{path.name}: download cut short")
    partial.replace(path)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--models", default=",".join(BLENDERKIT))
    parser.add_argument("--refresh", action="store_true", help="download and export again what is already there")
    options = parser.parse_args()

    blends = LOCAL / "blenderkit"
    blends.mkdir(parents=True, exist_ok=True)
    for name in options.models.split(","):
        if name not in BLENDERKIT:
            sys.exit(f"unknown BlenderKit model '{name}' (known: {', '.join(BLENDERKIT)})")
        results = fetch_json(API + BLENDERKIT[name]).get("results") or []
        if not results:
            sys.exit(f"{name}: BlenderKit knows no asset {BLENDERKIT[name]}")
        asset = results[0]
        print(f"{name}: \"{asset['name']}\" by {asset['author']['fullName']}, licence {asset['license']}", flush=True)
        if not (asset.get("isFree") and asset.get("canDownload")):
            sys.exit(f"{name}: not a free asset, it cannot be fetched without an account")

        blend = blends / f"{name}.blend"
        if options.refresh or not blend.exists():
            link = next(f["downloadUrl"] for f in asset["files"] if f["fileType"] == "blend")
            # The server answers with the address of the file itself.
            address = fetch_json(f"{link}?scene_uuid={uuid.uuid4()}")["filePath"]
            print(f"  downloading {blend.relative_to(BENCH.parent)}", flush=True)
            download(address, blend)
        else:
            print(f"  {blend.relative_to(BENCH.parent)} is already there")

        target = source_path(name)
        if options.refresh or not target.exists():
            done = subprocess.run(
                ["blender", "-b", str(blend), "--python", str(BENCH / "import_blenderkit.py"), "--", str(target)],
                capture_output=True,
                text=True,
            )
            summary = [line for line in done.stdout.splitlines() if line.startswith("BLENDERKIT wrote")]
            if done.returncode != 0 or not summary:
                sys.exit(f"{name}: Blender could not export {blend}")
            print("  " + summary[0].removeprefix("BLENDERKIT "))
        else:
            print(f"  {target.relative_to(BENCH.parent)} is already there")


if __name__ == "__main__":
    main()
