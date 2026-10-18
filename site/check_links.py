#!/usr/bin/env python3
"""Check the built site: every relative link, picture and anchor of every
page must lead to something inside it. Links to other sites are not
followed. The English and French glossaries must have the same entries
(the same anchors), and every link on a term of the glossary must lead to
the glossary of its page's language and carry the term's definition.
Exits with 1 and lists what is broken, if anything is.

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
        self.entries = set()  # anchors of the entries of a glossary
        self.terms = []  # (href, title) of the links on terms of the glossary

    def handle_starttag(self, tag, attributes):
        attributes = dict(attributes)
        classes = (attributes.get("class") or "").split()
        if "id" in attributes:
            self.ids.add(attributes["id"])
            if "entry" in classes:
                self.entries.add(attributes["id"])
        if tag == "a" and "term" in classes:
            self.terms.append((attributes.get("href") or "", attributes.get("title") or ""))
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
    glossaries = {language: pages.get(page) for language, page in (("en", "glossary.html"), ("fr", "fr/glossary.html"))}
    if all(glossaries.values()):
        english, french = glossaries["en"].entries, glossaries["fr"].entries
        if not english:
            broken.append(("glossary.html", "p", "no entry with an anchor"))
        for anchor in sorted(english - french):
            broken.append(("fr/glossary.html", "p", f"#{anchor} (in the English glossary only)"))
        for anchor in sorted(french - english):
            broken.append(("glossary.html", "p", f"#{anchor} (in the French glossary only)"))
    terms = 0
    for name, parser in pages.items():
        glossary = "fr/glossary.html" if name.startswith("fr/") else "glossary.html"
        for link, title in parser.terms:
            terms += 1
            parts = urlsplit(link)
            target = posixpath.normpath(posixpath.join(posixpath.dirname(name), unquote(parts.path)))
            if target != glossary or unquote(parts.fragment) not in pages[glossary].entries:
                broken.append((name, "a", f"{link} (a term, not to an entry of {glossary})"))
            elif not title.strip():
                broken.append((name, "a", f"{link} (a term with no definition)"))
    for name, tag, link in broken:
        print(f"broken: {name}: <{tag}> {link}")
    print(
        f"checked {len(pages)} pages, {terms} links to the glossary; {len(broken)} broken; "
        f"{len(external)} links to other sites not followed"
    )
    sys.exit(1 if broken else 0)


if __name__ == "__main__":
    main()
