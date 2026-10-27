"""Checks the levels of evidence of the annotated version of the paper.

Every theorem, proposition, lemma, corollary, remark, question and conjecture
of paper.tex sits in an `evidence` box with a known level, or is preceded by a
comment `% evidence: none` saying why it claims nothing; every Lean name a box
cites (\\lean{...}) is declared in lean/; a box at a Lean level cites at least
one. Run from anywhere: python3 docs/research/turns-of-a-t-mesh/evidence.py
"""

import re
import sys
from pathlib import Path

HERE = Path(__file__).parent
LEVELS = {"lean", "core", "data", "measured", "prose", "cited", "open"}
STATEMENTS = ["theorem", "proposition", "lemma", "corollary", "remark", "question", "conjecture"]


def declarations():
    """The full names of the declarations of the Lean sources, namespaces followed."""
    names = set()
    for path in sorted((HERE / "lean").rglob("*.lean")):
        if ".lake" in path.parts:
            continue
        spaces = []
        for line in path.read_text().splitlines():
            if m := re.match(r"\s*namespace\s+(\S+)", line):
                spaces.append(m.group(1))
            elif m := re.match(r"\s*end\s+(\S+)\s*$", line):
                if spaces and spaces[-1] == m.group(1):
                    spaces.pop()
            elif m := re.match(
                r"\s*(?:@\[[^\]]*\]\s*)?(?:private\s+|protected\s+|noncomputable\s+)*"
                r"(?:theorem|lemma|def|abbrev|structure|inductive|instance|class)\s+([^\s:({\[]+)",
                line,
            ):
                prefix = ".".join(spaces)
                names.add(f"{prefix}.{m.group(1)}" if prefix else m.group(1))
    return names


def declared(name, names):
    return any(full == name or full.endswith("." + name) for full in names)


def main():
    text = (HERE / "paper.tex").read_text()
    body = text[text.index(r"\begin{document}"):]
    names = declarations()
    errors = []
    # The boxes, their levels and their Lean names.
    boxes = list(re.finditer(r"\\begin\{evidence\}(?:\[[^\]]*\])?\{(\w+)\}\{", body))
    for box in boxes:
        level = box.group(1)
        end = body.index(r"\end{evidence}", box.end())
        tag = body[box.end():body.index("\n", box.end())]
        cited = re.findall(r"\\lean\{([^}]*)\}", tag)
        if level not in LEVELS:
            errors.append(f"unknown level {level!r}")
        if level in ("lean", "core") and not cited:
            errors.append(f"a {level} box cites no Lean name: {tag[:60]}")
        for name in cited:
            if not declared(name, names):
                errors.append(f"not declared in lean/: {name}")
        if r"\begin{evidence}" in body[box.end():end]:
            errors.append(f"a box inside a box: {tag[:60]}")
    # Every statement inside a box, or said to claim nothing.
    spans = [(box.start(), body.index(r"\end{evidence}", box.end())) for box in boxes]
    statements = 0
    for m in re.finditer(r"\\begin\{(%s)\}" % "|".join(STATEMENTS), body):
        statements += 1
        if any(start < m.start() < end for start, end in spans):
            continue
        before = body[:m.start()].rstrip().rsplit("\n", 1)[-1]
        if not before.startswith("% evidence: none"):
            line = text[: text.index(r"\begin{document}")].count("\n") + body[: m.start()].count("\n") + 1
            errors.append(f"line {line}: a {m.group(1)} without a level")
    for error in errors:
        print("evidence:", error)
    counts = {level: sum(1 for box in boxes if box.group(1) == level) for level in sorted(LEVELS)}
    print(f"evidence: {statements} statements, {len(boxes)} boxes {counts}, {len(names)} Lean declarations")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
