#!/usr/bin/env python3
"""Check the built site: every relative link, picture and anchor of every
page must lead to something inside it. Links to other sites are not
followed. Exits with 1 and lists what is broken, if anything is.

    python3 site/check_links.py [site/_build]
"""

import html.parser
import posixpath
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit


class Page(html.parser.HTMLParser):
    def __init__(self):
        super().__init__()
        self.links = []
        self.ids = set()

    def handle_starttag(self, tag, attributes):
        attributes = dict(attributes)
        if "id" in attributes:
            self.ids.add(attributes["id"])
        for name in ("href", "src"):
            if attributes.get(name) is not None:
                self.links.append((tag, attributes[name]))
        if attributes.get("srcset"):
            for candidate in attributes["srcset"].split(","):
                self.links.append((tag, candidate.strip().split(" ")[0]))


def main():
    root = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).resolve().parent / "_build")
    pages = {}
    for path in sorted(root.rglob("*.html")):
        parser = Page()
        parser.feed(path.read_text(encoding="utf-8"))
        pages[path.relative_to(root).as_posix()] = parser
    broken = []
    external = set()
    for name, parser in pages.items():
        for tag, link in parser.links:
            parts = urlsplit(link)
            if parts.scheme or link.startswith("//"):
                external.add(link)
                continue
            if not link:
                broken.append((name, tag, "(empty)"))
                continue
            target = posixpath.normpath(posixpath.join(posixpath.dirname(name), unquote(parts.path))) if parts.path else name
            if target.startswith(".."):
                broken.append((name, tag, f"{link} (outside the site)"))
                continue
            if not (root / target).exists():
                broken.append((name, tag, link))
                continue
            if (root / target).is_dir():
                target = posixpath.join(target, "index.html")
            if parts.fragment and target in pages and unquote(parts.fragment) not in pages[target].ids:
                broken.append((name, tag, f"{link} (no such anchor)"))
    for name, tag, link in broken:
        print(f"broken: {name}: <{tag}> {link}")
    print(f"checked {len(pages)} pages; {len(broken)} broken; {len(external)} links to other sites not followed")
    sys.exit(1 if broken else 0)


if __name__ == "__main__":
    main()
