#!/usr/bin/env python3
"""Check docs/retopology/tools/*.toml, docs/retopology/platforms.toml and
docs/retopology/taxonomy.toml against the schema of docs/retopology/README.md, and rewrite the indexes of
that README.

    python3 scripts/retopology.py [--check] [--compute]

With --check, nothing is written: the exit status says whether every file
follows the schema and the indexes are up to date. With --compute, the tags
that follow from the fields (licence, source, blender, install) are rewritten
in every file first.
"""

import argparse
import datetime
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FOLDER = ROOT / "docs" / "retopology"
TOOLS = FOLDER / "tools"
PLATFORMS = FOLDER / "platforms.toml"
TAXONOMY = FOLDER / "taxonomy.toml"
ENGINES = FOLDER / "engines.toml"
# Engine tags that name no single algorithm, so have no entry in engines.toml.
NOT_ENGINES = {"own", "proprietary", "simulation", "particles", "ml"}
ENGINE = {"id", "name", "family", "method", "output", "authors", "licence", "source", "year", "status", "last_commit",
          "papers", "strengths", "limits", "notes", "checked", "sources"}
README = FOLDER / "README.md"

KINDS = {"built-in", "add-on", "bridge", "standalone"}
STATUSES = {"active", "stale", "abandoned", "unknown"}
PLATFORM_KINDS = {"built-in", "official", "marketplace", "creator-store", "code-host", "donation", "vendor"}
TOP = {"name", "slug", "author", "tags", "kind", "method", "purpose", "status", "checked", "bench",
       "distribution", "blender", "papers", "sources", "notes", "brief"}
DISTRIBUTION = {"url", "channel", "source", "licence", "price", "also"}
STORE = {"url", "channel", "price"}
BLENDER = {"min", "max", "release"}
PAPER = {"title", "authors", "venue", "year", "doi"}
PLATFORM = {"id", "name", "url", "kind", "operator", "paid", "licence_rule", "seller_share", "install", "updates",
            "blender", "notes", "sources"}
SLUG = re.compile(r"^[a-z0-9]+(-[a-z0-9]+)*$")
VERSION = re.compile(r"^\d+\.\d+(\.\d+)?$")
RELEASE = re.compile(r"^\S+ \(\d{4}-\d{2}-\d{2}\)$")


class Checker:
    """Collects the ways one file breaks the schema, as sentences."""

    def __init__(self):
        self.found = []

    def unknown(self, table, allowed, where=""):
        self.found += [f"{where}{key}: unknown field" for key in sorted(set(table) - allowed)]

    def need(self, table, key, where=""):
        if not isinstance(table.get(key), str) or not table[key].strip():
            self.found.append(f"{where}{key}: missing or empty")
            return False
        return True

    def url(self, value, where):
        if not isinstance(value, str) or not value.startswith("https://"):
            self.found.append(f"{where}: not an https URL")

    def among(self, table, key, allowed, where=""):
        if self.need(table, key, where) and table[key] not in allowed:
            self.found.append(f"{where}{key}: {table[key]} is none of {', '.join(sorted(allowed))}")

    def sources(self, sources):
        if not sources:
            self.found.append("sources: at least one is needed")
        for i, source in enumerate(sources or []):
            # A key written after a [[sources]] header lands in that source.
            self.unknown(source, {"url", "what"}, f"sources[{i}].")
            self.url(source.get("url"), f"sources[{i}].url")
            self.need(source, "what", f"sources[{i}].")


def tags_of(entry, facet):
    """The values of one facet among the tags of a tool."""
    return [tag.split(":", 1)[1] for tag in entry.get("tags", []) if tag.startswith(facet + ":")]


OPEN = re.compile(r"^(A|L)?GPL|^MIT$|^BSD|^Apache|^MPL")


def licence_tag(dist):
    if OPEN.match(dist["licence"]):
        return "open-source"
    return "proprietary" if dist["licence"] == "proprietary" else "not-stated"


def source_tag(dist):
    if "source" in dist:
        return "public"
    return "with-download" if licence_tag(dist) == "open-source" else "closed"


def version(text):
    return tuple(int(part) for part in text.split("."))


def blender_tag(entry):
    """The newest Blender a tool states: built-in tools follow Blender."""
    blender = entry.get("blender", {})
    if entry.get("kind") == "built-in":
        return "5-x"
    newest = blender.get("max") or (blender.get("min") if version(blender.get("min", "0.0")) >= (5, 0) else None)
    if not newest:
        return "unstated"
    if version(newest) >= (5, 0):
        return "5-x"
    return "4-x" if version(newest) >= (4, 2) else "legacy"


def install_tag(entry):
    dist = entry.get("distribution", {})
    channels = {s.get("channel") for s in [dist] + dist.get("also", [])}
    if entry.get("kind") == "built-in":
        return "built-in"
    if entry.get("kind") == "standalone":
        return "external"
    return "get-extensions" if channels & {"extensions.blender.org", "remote-repository"} else "zip"


COMPUTED = {"licence": lambda e: licence_tag(e["distribution"]), "source": lambda e: source_tag(e["distribution"]),
            "blender": blender_tag, "install": install_tag}


def tool_problems(path, entry, platforms, taxonomy):
    c = Checker()
    c.unknown(entry, TOP)
    for key in ("name", "author", "method"):
        c.need(entry, key)
    if c.need(entry, "slug"):
        if not SLUG.match(entry["slug"]):
            c.found.append("slug: lower case, digits and - only")
        if entry["slug"] != path.stem:
            c.found.append(f"slug: {entry['slug']} is not the file name {path.stem}")
    c.among(entry, "kind", KINDS)
    c.among(entry, "status", STATUSES)
    tags = entry.get("tags")
    if not isinstance(tags, list) or not tags:
        c.found.append("tags: missing or empty")
        tags = []
    for tag in tags:
        facet, _, value = str(tag).partition(":")
        if value not in taxonomy.get(facet, {}):
            c.found.append(f"tags: {tag} is not in taxonomy.toml")
    if len(set(tags)) != len(tags):
        c.found.append("tags: a tag is given twice")
    needed = [f for f, values in taxonomy.items() if values.get("required")]
    # A tool that makes faces of its own says which; manual tools and helpers leave it to the user.
    if set(tags_of(entry, "mode")) - {"manual", "helper"}:
        needed.append("output")
    for facet in needed:
        if not tags_of(entry, facet):
            c.found.append(f"tags: no {facet}:… tag")
    # Some tags follow from the fields: licence, source, blender, install.
    dist = entry.get("distribution", {})
    if isinstance(dist, dict) and dist.get("licence"):
        for facet, compute in COMPUTED.items():
            value = compute(entry)
            if tags_of(entry, facet) != [value]:
                c.found.append(f"tags: {facet}:{value} expected from the fields")
    if not isinstance(entry.get("checked"), datetime.date):
        c.found.append("checked: missing, or not a TOML date")

    dist = entry.get("distribution")
    if not isinstance(dist, dict):
        c.found.append("distribution: missing")
    else:
        c.unknown(dist, DISTRIBUTION, "distribution.")
        if c.need(dist, "url", "distribution."):
            c.url(dist["url"], "distribution.url")
        c.among(dist, "channel", platforms, "distribution.")
        c.need(dist, "licence", "distribution.")
        c.need(dist, "price", "distribution.")
        if "source" in dist:
            c.url(dist["source"], "distribution.source")
        for i, store in enumerate(dist.get("also", [])):
            where = f"distribution.also[{i}]."
            c.unknown(store, STORE, where)
            if c.need(store, "url", where):
                c.url(store["url"], where + "url")
            c.among(store, "channel", platforms, where)

    blender = entry.get("blender", {})
    c.unknown(blender, BLENDER, "blender.")
    for key in ("min", "max"):
        if key in blender and not VERSION.match(str(blender[key])):
            c.found.append(f"blender.{key}: {blender[key]} is not a version like 4.2")
    if "release" in blender and not RELEASE.match(str(blender["release"])):
        c.found.append("blender.release: not like 1.2.0 (2026-01-31)")

    for i, paper in enumerate(entry.get("papers", [])):
        c.unknown(paper, PAPER, f"papers[{i}].")
        for key in ("title", "authors", "venue"):
            c.need(paper, key, f"papers[{i}].")
        if not isinstance(paper.get("year"), int):
            c.found.append(f"papers[{i}].year: missing, or not a number")
    c.sources(entry.get("sources"))
    return c.found


def engine_problems(engine, taxonomy):
    c = Checker()
    c.unknown(engine, ENGINE)
    for key in ("name", "family", "method", "authors", "licence"):
        c.need(engine, key)
    if engine.get("id") not in taxonomy["engine"] or engine.get("id") in NOT_ENGINES:
        c.found.append("id: not an engine value of taxonomy.toml")
    c.among(engine, "status", STATUSES)
    outputs = engine.get("output")
    if not isinstance(outputs, list) or not outputs or set(outputs) - set(taxonomy["output"]):
        c.found.append("output: missing, or a value not in the output facet")
    if "source" in engine:
        c.url(engine["source"], "source")
    for key in ("checked", "last_commit"):
        if (key == "checked" or key in engine) and not isinstance(engine.get(key), datetime.date):
            c.found.append(f"{key}: missing, or not a TOML date")
    for i, paper in enumerate(engine.get("papers", [])):
        c.unknown(paper, PAPER, f"papers[{i}].")
        for key in ("title", "authors", "venue"):
            c.need(paper, key, f"papers[{i}].")
    c.sources(engine.get("sources"))
    return [f"{engine.get('id', '?')}: {problem}" for problem in c.found]


def platform_problems(platform):
    c = Checker()
    c.unknown(platform, PLATFORM)
    for key in ("id", "name", "operator"):
        c.need(platform, key)
    # A vendor is every publisher's own site: it has no address of its own.
    if platform.get("kind") != "vendor" or "url" in platform:
        if c.need(platform, "url"):
            c.url(platform["url"], "url")
    c.among(platform, "kind", PLATFORM_KINDS)
    if not isinstance(platform.get("paid"), bool):
        c.found.append("paid: missing, or not true or false")
    c.sources(platform.get("sources"))
    return [f"{platform.get('id', '?')}: {problem}" for problem in c.found]


def tool_index(entries, platforms):
    """One row per tool, with the tags that matter most for choosing."""
    rows = ["| Tool | Mode | Output | Engine | Price | Where | Blender | Status |",
            "|---|---|---|---|---|---|---|---|"]
    for entry in sorted(entries, key=lambda e: e["name"].lower()):
        dist, blender = entry["distribution"], entry.get("blender", {})
        span = "–".join(dict.fromkeys(v for v in (blender.get("min"), blender.get("max")) if v)) or "—"
        if blender.get("min") and not blender.get("max"):
            span += "+"
        where = ", ".join(dict.fromkeys(platforms[s["channel"]]["name"] for s in [dist] + dist.get("also", [])))
        cell = lambda facet: ", ".join(tags_of(entry, facet)) or "—"
        name = entry["name"].replace("|", "\\|")
        rows.append(f"| [{name}](tools/{entry['slug']}.toml) | {cell('mode')} | {cell('output')} | "
                    f"{cell('engine')} | {cell('price')} | {where} | {span} | {entry['status']} |")
    return "\n".join(rows)


def tag_index(entries, taxonomy):
    """How many tools carry each tag, facet by facet."""
    rows = ["| Facet | Tag | Meaning | Tools |", "|---|---|---|---|"]
    for facet, values in taxonomy.items():
        for value, meaning in values.items():
            if value in ("required", "title"):
                continue
            count = sum(f"{facet}:{value}" in e.get("tags", []) for e in entries)
            rows.append(f"| {facet} | `{facet}:{value}` | {meaning} | {count} |")
    return "\n".join(rows)


def platform_index(platforms, entries):
    """One row per platform, with the number of tools found on it."""
    count = {key: 0 for key in platforms}
    for entry in entries:
        dist = entry["distribution"]
        for channel in dict.fromkeys(s["channel"] for s in [dist] + dist.get("also", [])):
            count[channel] += 1
    rows = ["| Platform | Kind | Paid add-ons | Tools here |", "|---|---|---|---|"]
    for key, p in platforms.items():
        name = f"[{p['name']}]({p['url']})" if "url" in p else p["name"]
        rows.append(f"| {name} | {p['kind']} | {'yes' if p['paid'] else 'no'} | {count[key]} |")
    return "\n".join(rows)


def compute_tags(path):
    """Rewrite the computed tags of one file, before its price tags."""
    text = path.read_text(encoding="utf-8")
    entry = tomllib.loads(text)
    match = re.search(r"^tags = \[(.*)\]$", text, flags=re.M)
    tags = [t for t in re.findall(r'"([^"]+)"', match.group(1)) if t.split(":")[0] not in COMPUTED]
    at = next((i for i, t in enumerate(tags) if t.startswith("price:")), len(tags))
    tags[at:at] = [f"{facet}:{compute(entry)}" for facet, compute in COMPUTED.items()]
    line = "tags = [" + ", ".join(f'"{t}"' for t in tags) + "]"
    path.write_text(text[: match.start()] + line + text[match.end():], encoding="utf-8")


def replace(text, marker, body):
    return re.sub(rf"(<!-- {marker}:start -->\n).*?(<!-- {marker}:end -->)",
                  lambda m: m.group(1) + body + "\n" + m.group(2), text, flags=re.S)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="write nothing, only check")
    parser.add_argument("--compute", action="store_true", help="rewrite the computed tags first")
    args = parser.parse_args()
    if args.compute and not args.check:
        for path in sorted(TOOLS.glob("*.toml")):
            compute_tags(path)

    failed = False
    try:
        listed = tomllib.loads(PLATFORMS.read_text(encoding="utf-8")).get("platform", [])
    except tomllib.TOMLDecodeError as error:
        print(f"platforms.toml: {error}")
        return 1
    platforms = {}
    for platform in listed:
        for problem in platform_problems(platform):
            print(f"platforms.toml: {problem}")
            failed = True
        if platform.get("id") in platforms:
            print(f"platforms.toml: {platform['id']}: listed twice")
            failed = True
        platforms[platform.get("id")] = platform

    try:
        taxonomy = tomllib.loads(TAXONOMY.read_text(encoding="utf-8"))["facet"]
    except (tomllib.TOMLDecodeError, KeyError) as error:
        print(f"taxonomy.toml: {error}")
        return 1

    try:
        engines = tomllib.loads(ENGINES.read_text(encoding="utf-8")).get("engine", [])
    except tomllib.TOMLDecodeError as error:
        print(f"engines.toml: {error}")
        return 1
    for engine in engines:
        for problem in engine_problems(engine, taxonomy):
            print(f"engines.toml: {problem}")
            failed = True
    missing = set(taxonomy["engine"]) - {"title", "required"} - NOT_ENGINES - {e.get("id") for e in engines}
    for value in sorted(missing):
        print(f"engines.toml: no entry for the engine tag {value}")
        failed = True

    entries = []
    for path in sorted(TOOLS.glob("*.toml")):
        try:
            entry = tomllib.loads(path.read_text(encoding="utf-8"))
        except tomllib.TOMLDecodeError as error:
            print(f"{path.name}: {error}")
            failed = True
            continue
        for problem in tool_problems(path, entry, platforms, taxonomy):
            print(f"{path.name}: {problem}")
            failed = True
        entries.append(entry)
    if failed:
        return 1

    text = README.read_text(encoding="utf-8")
    fresh = replace(text, "tools", tool_index(entries, platforms))
    fresh = replace(fresh, "tags", tag_index(entries, taxonomy))
    fresh = replace(fresh, "platforms", platform_index(platforms, entries))
    if fresh != text:
        if args.check:
            print("README.md: the indexes are out of date; run python3 scripts/retopology.py")
            return 1
        README.write_text(fresh, encoding="utf-8")
    print(f"{len(entries)} tools, {len(platforms)} platforms and {len(engines)} engines, all following the schema")
    return 0


if __name__ == "__main__":
    sys.exit(main())
