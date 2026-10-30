"""Checks the levels of evidence of the statements of the paper.

Every theorem, proposition, lemma, corollary, remark, question and conjecture
of paper.tex and of its translation paper.fr.tex sits in an `evidence` box with a known level, or is preceded by a
comment `% evidence: none` saying why it claims nothing; every Lean name a box
cites (\\lean{...}) is declared in lean/; a box at a Lean level cites at least
one; a box at the level `tested` (a conjecture checked by computer, on the
random T-meshes of experiments/property) says on how many instances. Run from
anywhere: python3 docs/research/turns-of-a-t-mesh/evidence.py
"""

import re
import sys
from pathlib import Path

HERE = Path(__file__).parent
LEVELS = {"lean", "core", "data", "measured", "tested", "prose", "cited", "open"}
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


def check(path, names):
    """The errors of one paper, and its counts."""
    text = path.read_text()
    body = text[text.index(r"\begin{document}"):]
    offset = text[: text.index(r"\begin{document}")].count("\n")
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
        # A statement checked by computer says on how many instances (the
        # property tests of experiments/property), in either language.
        if level == "tested" and not re.search(r"\d[\d,{}\\ ]*\s*(random\s+)?instances", tag):
            errors.append(f"a tested box does not say on how many instances: {tag[:60]}")
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
            errors.append(f"line {offset + body[: m.start()].count(chr(10)) + 1}: a {m.group(1)} without a level")
    levels = [box.group(1) for box in boxes]
    return errors, statements, levels


def main():
    names = declarations()
    failed = False
    papers = [HERE / "paper.tex"] + [path for path in [HERE / "paper.fr.tex"] if path.exists()]
    levels = {}
    for path in papers:
        errors, statements, levels[path.name] = check(path, names)
        for error in errors:
            print(f"evidence: {path.name}: {error}")
        counts = {level: levels[path.name].count(level) for level in sorted(LEVELS)}
        print(f"evidence: {path.name}: {statements} statements, {len(levels[path.name])} boxes {counts}")
        failed |= bool(errors)
    # The translation gives each statement the level of the original.
    if len(papers) == 2 and levels["paper.tex"] != levels["paper.fr.tex"]:
        print("evidence: paper.tex and paper.fr.tex give different levels, in order")
        failed = True
    print(f"evidence: {len(names)} Lean declarations")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
