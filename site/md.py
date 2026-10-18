"""A small Markdown converter for the subset the openclare documents use.

Blocks: ATX headings, paragraphs, bullet and numbered lists (nested by
indentation), pipe tables, fenced code, block quotes, horizontal rules and raw
HTML blocks. Inline: code spans, images, links, autolinks, strong and
emphasis. Nothing else: no reference links, no setext headings, no footnotes.

`render(text, resolve)` returns the HTML and the list of headings. `resolve`
is called with every link and image target, `resolve(target, image)`, and
returns the target to write, or None for an image that must not be shown (it
is then replaced by its alternative text).
"""

import html
import re
import unicodedata

BULLET = re.compile(r"^( *)([-*+]) +(.*)$")
NUMBERED = re.compile(r"^( *)(\d{1,9})[.)] +(.*)$")
HEADING = re.compile(r"^(#{1,6}) +(.*?) *#*$")
FENCE = re.compile(r"^( *)(```+|~~~+) *([\w+-]*)")
RULE = re.compile(r"^ {0,3}([-*_])( *\1){2,} *$")
TABLE_DELIMITER = re.compile(r"^ *\|? *:?-+:? *(\| *:?-+:? *)*\|? *$")
HTML_BLOCK = re.compile(r"^ {0,3}<(/?[a-zA-Z][a-zA-Z0-9-]*|!--)")


def slug(text, taken):
    """An id for a heading: lower case, accents kept, words joined by hyphens,
    unique within the page."""
    text = re.sub(r"<[^>]+>", "", text)
    text = html.unescape(text).lower()
    text = unicodedata.normalize("NFC", text)
    text = re.sub(r"[^\w\s-]", "", text).strip()
    text = re.sub(r"[\s_]+", "-", text) or "section"
    candidate, number = text, 1
    while candidate in taken:
        number += 1
        candidate = f"{text}-{number}"
    taken.add(candidate)
    return candidate


class Converter:
    def __init__(self, resolve=None):
        self.resolve = resolve or (lambda target, image: target)
        self.headings = []
        self.ids = set()

    # ---------------------------------------------------------------- inline

    def inline(self, text):
        stash = []

        def keep(fragment):
            stash.append(fragment)
            return f"\x00{len(stash) - 1}\x00"

        def code(match):
            return keep(f"<code>{html.escape(match.group(2).strip(' ') or match.group(2), quote=False)}</code>")

        text = re.sub(r"(`+)(.+?)\1", code, text)
        text = re.sub(r"\\([\\`*_{}\[\]()#+\-.!|<>])", lambda m: keep(html.escape(m.group(1))), text)

        def image(match):
            alt, target = match.group(1), match.group(2)
            title = match.group(3)
            resolved = self.resolve(target, True)
            if resolved is None:
                return keep(f'<span class="withheld">{html.escape(alt)}</span>')
            extra = f' title="{html.escape(title)}"' if title else ""
            return keep(
                f'<img src="{html.escape(resolved)}" alt="{html.escape(alt)}" loading="lazy" decoding="async"{extra}>'
            )

        text = re.sub(r'!\[([^\]]*)\]\(\s*<?([^)\s>]+)>?(?:\s+"([^"]*)")?\s*\)', image, text)

        def link(match):
            label, target = match.group(1), match.group(2)
            resolved = self.resolve(target, False)
            inner = self.inline_resolved(label, stash)
            external = resolved.startswith(("http://", "https://"))
            rel = ' rel="noopener"' if external else ""
            return keep(f'<a href="{html.escape(resolved)}"{rel}>{inner}</a>')

        # A label may hold an image (and the image is already stashed) or
        # brackets of its own, one level deep.
        text = re.sub(r"\[((?:[^\[\]]|\[[^\[\]]*\])*)\]\(\s*<?([^)\s>]+)>?\s*\)", link, text)
        text = re.sub(
            r"<(https?://[^>\s]+)>",
            lambda m: keep(f'<a href="{html.escape(m.group(1))}" rel="noopener">{html.escape(m.group(1))}</a>'),
            text,
        )
        # Inline HTML the documents use on purpose.
        text = re.sub(r"</?(?:br|sup|sub|kbd|abbr|span|em|strong)\b[^>]*>", lambda m: keep(m.group(0)), text)
        return self.finish(text, stash)

    def inline_resolved(self, text, stash):
        """Strong and emphasis on text whose links and code are already in
        `stash`, without restoring them (the caller does)."""
        text = html.escape(text, quote=False)
        return self.emphasis(text)

    @staticmethod
    def emphasis(text):
        text = re.sub(r"\*\*(?=\S)(.+?)(?<=\S)\*\*", r"<strong>\1</strong>", text)
        text = re.sub(r"(?<![\w*])__(?=\S)(.+?)(?<=\S)__(?![\w*])", r"<strong>\1</strong>", text)
        text = re.sub(r"(?<![\w*])\*(?=[^\s*])(.+?)(?<=[^\s*])\*(?![\w*])", r"<em>\1</em>", text)
        text = re.sub(r"(?<![\w])_(?=\S)(.+?)(?<=\S)_(?![\w])", r"<em>\1</em>", text)
        return text

    def finish(self, text, stash):
        text = html.escape(text, quote=False)
        # An entity written in the source (&nbsp;, &rarr;) stays an entity.
        text = re.sub(r"&amp;(#?\w+;)", r"&\1", text)
        text = self.emphasis(text)
        while "\x00" in text:
            text = re.sub(r"\x00(\d+)\x00", lambda m: stash[int(m.group(1))], text)
        return text

    # ---------------------------------------------------------------- blocks

    def blocks(self, lines):
        out = []
        index = 0
        paragraph = []

        def flush():
            if paragraph:
                out.append(("p", " ".join(line.strip() for line in paragraph)))
                paragraph.clear()

        while index < len(lines):
            line = lines[index]
            stripped = line.strip()
            if not stripped:
                flush()
                index += 1
                continue
            fence = FENCE.match(line)
            if fence:
                flush()
                marker = fence.group(2)
                language = fence.group(3)
                body = []
                index += 1
                while index < len(lines) and not lines[index].strip().startswith(marker):
                    body.append(lines[index])
                    index += 1
                index += 1
                out.append(("code", language, "\n".join(body)))
                continue
            heading = HEADING.match(line)
            if heading:
                flush()
                out.append(("h", len(heading.group(1)), heading.group(2)))
                index += 1
                continue
            if RULE.match(line) and not paragraph:
                out.append(("hr",))
                index += 1
                continue
            if HTML_BLOCK.match(line) and not paragraph:
                body = []
                while index < len(lines) and lines[index].strip():
                    body.append(lines[index])
                    index += 1
                out.append(("html", "\n".join(body)))
                continue
            if stripped.startswith(">"):
                flush()
                body = []
                while index < len(lines) and lines[index].strip().startswith(">"):
                    body.append(re.sub(r"^ *> ?", "", lines[index]))
                    index += 1
                out.append(("quote", body))
                continue
            if "|" in line and index + 1 < len(lines) and TABLE_DELIMITER.match(lines[index + 1]) and "-" in lines[index + 1]:
                flush()
                rows = [line, lines[index + 1]]
                index += 2
                while index < len(lines) and "|" in lines[index] and lines[index].strip():
                    rows.append(lines[index])
                    index += 1
                out.append(("table", rows))
                continue
            bullet, numbered = BULLET.match(line), NUMBERED.match(line)
            starts_list = bullet or numbered
            # A list interrupts a paragraph only from its first item: a line
            # that wraps onto "4. ..." stays in the paragraph.
            if starts_list and paragraph and numbered and numbered.group(2) != "1":
                starts_list = None
            if starts_list:
                flush()
                index = self.list_block(lines, index, out)
                continue
            paragraph.append(line)
            index += 1
        flush()
        return out

    def list_block(self, lines, index, out):
        first = BULLET.match(lines[index]) or NUMBERED.match(lines[index])
        ordered = NUMBERED.match(lines[index]) is not None and not BULLET.match(lines[index])
        indent = len(first.group(1))
        start = int(first.group(2)) if ordered else 1
        items = []
        loose = False
        while index < len(lines):
            line = lines[index]
            marker = (NUMBERED if ordered else BULLET).match(line)
            if not marker or len(marker.group(1)) != indent:
                break
            content_indent = len(line) - len(marker.group(3))
            body = [marker.group(3)]
            index += 1
            while index < len(lines):
                line = lines[index]
                if not line.strip():
                    # A blank line: the item goes on only if what follows is
                    # indented under it.
                    following = index + 1
                    while following < len(lines) and not lines[following].strip():
                        following += 1
                    if following < len(lines) and len(lines[following]) - len(lines[following].lstrip()) >= content_indent:
                        body.append("")
                        index += 1
                        loose = True
                        continue
                    break
                leading = len(line) - len(line.lstrip())
                other = BULLET.match(line) or NUMBERED.match(line)
                if other and leading <= indent:
                    break
                if leading >= content_indent:
                    body.append(line[content_indent:])
                elif other:
                    body.append(line.strip())
                elif HEADING.match(line) or FENCE.match(line):
                    break
                else:
                    body.append(line.strip())
                index += 1
            items.append(body)
            # A blank line between items makes the list loose.
            if index < len(lines) and not lines[index].strip():
                following = index + 1
                while following < len(lines) and not lines[following].strip():
                    following += 1
                again = following < len(lines) and (NUMBERED if ordered else BULLET).match(lines[following])
                if again and len(again.group(1)) == indent:
                    loose = True
                    index = following
        out.append(("list", ordered, start, items, loose))
        return index

    def render_blocks(self, blocks):
        parts = []
        position = 0
        while position < len(blocks):
            block = blocks[position]
            kind = block[0]
            if kind == "h":
                level, text = block[1], block[2]
                inner = self.inline(text)
                identifier = slug(inner, self.ids)
                self.headings.append((level, inner, identifier))
                parts.append(f'<h{level} id="{identifier}">{inner}</h{level}>')
            elif kind == "p":
                text = block[1]
                figure = re.fullmatch(r"!\[[^\]]*\]\([^)]*\)", text)
                if figure:
                    caption = ""
                    following = blocks[position + 1] if position + 1 < len(blocks) else None
                    if following and following[0] == "p" and re.fullmatch(r"\*[^*].*[^*]\*", following[1]):
                        caption = f"<figcaption>{self.inline(following[1][1:-1])}</figcaption>"
                        position += 1
                    parts.append(f"<figure>{self.inline(text)}{caption}</figure>")
                else:
                    parts.append(f"<p>{self.inline(text)}</p>")
            elif kind == "code":
                language = f' class="language-{block[1]}"' if block[1] else ""
                parts.append(f"<pre><code{language}>{html.escape(block[2], quote=False)}</code></pre>")
            elif kind == "hr":
                parts.append("<hr>")
            elif kind == "html":
                parts.append(block[1])
            elif kind == "quote":
                parts.append(f"<blockquote>{self.render_blocks(self.blocks(block[1]))}</blockquote>")
            elif kind == "table":
                parts.append(self.table(block[1]))
            elif kind == "list":
                parts.append(self.list_html(*block[1:]))
            position += 1
        return "\n".join(parts)

    @staticmethod
    def cells(row):
        row = row.strip()
        if row.startswith("|"):
            row = row[1:]
        if row.endswith("|") and not row.endswith("\\|"):
            row = row[:-1]
        # Split on pipes outside code spans and not escaped.
        cells, current, in_code = [], "", False
        position = 0
        while position < len(row):
            character = row[position]
            if character == "\\" and position + 1 < len(row) and row[position + 1] == "|":
                current += "|"
                position += 2
                continue
            if character == "`":
                in_code = not in_code
            if character == "|" and not in_code:
                cells.append(current.strip())
                current = ""
            else:
                current += character
            position += 1
        cells.append(current.strip())
        return cells

    def table(self, rows):
        header = self.cells(rows[0])
        aligns = []
        for cell in self.cells(rows[1]):
            if cell.startswith(":") and cell.endswith(":"):
                aligns.append("center")
            elif cell.endswith(":"):
                aligns.append("right")
            else:
                aligns.append(None)

        def attribute(column):
            align = aligns[column] if column < len(aligns) else None
            return f' style="text-align:{align}"' if align else ""

        head = "".join(f"<th{attribute(c)}>{self.inline(cell)}</th>" for c, cell in enumerate(header))
        body = []
        for row in rows[2:]:
            cells = self.cells(row)
            cells += [""] * (len(header) - len(cells))
            body.append("<tr>" + "".join(f"<td{attribute(c)}>{self.inline(cell)}</td>" for c, cell in enumerate(cells)) + "</tr>")
        return (
            f'<div class="table-wrap"><table><thead><tr>{head}</tr></thead>'
            f'<tbody>{"".join(body)}</tbody></table></div>'
        )

    def list_html(self, ordered, start, items, loose):
        rendered = []
        for body in items:
            blocks = self.blocks(body)
            if not loose and blocks and blocks[0][0] == "p":
                inner = self.inline(blocks[0][1])
                rest = self.render_blocks(blocks[1:])
                rendered.append(f"<li>{inner}{rest}</li>")
            else:
                rendered.append(f"<li>{self.render_blocks(blocks)}</li>")
        tag = "ol" if ordered else "ul"
        attribute = f' start="{start}"' if ordered and start != 1 else ""
        return f"<{tag}{attribute}>" + "".join(rendered) + f"</{tag}>"


def render(text, resolve=None):
    """HTML of a Markdown text, and its headings as (level, html, id)."""
    converter = Converter(resolve)
    lines = text.replace("\r\n", "\n").replace("\t", "    ").split("\n")
    body = converter.render_blocks(converter.blocks(lines))
    return body, converter.headings
