"""Checks the levels of evidence of the statements of the paper.

Every theorem, proposition, lemma, corollary, remark, question and conjecture
of paper.tex and of its translation paper.fr.tex sits in an `evidence` box with a known level, or is preceded by a
comment `% evidence: none` saying why it claims nothing; every Lean name a box
cites (\\lean{...}) is declared in lean/; a box at a Lean level cites at least
one; a box at the level `tested` (a conjecture checked by computer, on the
random T-meshes of experiments/property) says on how many instances. Run from
anywhere: python3 docs/research/turns-of-a-t-mesh/evidence.py [--json PATH]

The fields of a tag. A tag is a list of fields separated by `;` (`\\,;` in
French); besides the Lean names and free text, three fields name what else
checks the statement, each in both languages, and the French box carries the
same ones as the English:
- `tested:` (`testé~:`), the property tests that also ran on it;
- `examples: \\lean{name}, ...` (`exemples~:`), worked examples in Lean, a
  declaration checked by `decide`, `#guard` or a proof on a fixed instance;
- `rendered from: \\lean{name}, ...` (`rendu depuis~:`), the statement's
  text rendered from those Lean sources (DT-179).

The progress of the paper. `--json PATH` writes, for the statements of
paper.tex, one step per row of PROGRESS: its names in English and French, N
out of M statements and their labels (`kind-k` for the k-th statement when it
has no \\label), then whether the paper is a preprint (PREPRINT), the levels
of the statements (`none` for one that claims nothing) and their total. The
website reads it (site/build.py); paper.fr.tex must give the same.
"""

import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).parent
LEVELS = {"lean", "core", "data", "measured", "tested", "prose", "cited", "open"}
STATEMENTS = ["theorem", "proposition", "lemma", "corollary", "remark", "question", "conjecture"]
FIELDS = {
    "tested": r"(?:tested|testé)",
    "examples": r"(?:examples|exemples)",
    "rendered": r"(?:rendered from|rendu depuis)",
}
# The steps from a statement written to a preprint, in order: a key, its
# names, and which statements it counts (s: the statement's level and fields).
PROGRESS = [
    ("lean", "Lean written", "Lean écrit", lambda s: s["level"] in ("lean", "core")),
    ("examples", "Edge cases", "Cas limites", lambda s: bool(s["examples"])),
    ("tested", "Random tests", "Tests aléatoires", lambda s: s["level"] == "tested" or s["tested"]),
    ("mutation", "Mutation screening", "Tests de mutation", lambda s: False),
    ("rendered", "Prose from Lean", "Prose tirée du Lean", lambda s: bool(s["rendered"])),
]
# Paper B is a draft until the roadmap of DT-135 is done.
PREPRINT = False


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


def field(tag, key):
    """The text of one field of a tag, or None when it has none."""
    m = re.search(r"(?:^|;)\s*%s\s*~?:([^;]*)" % FIELDS[key], tag)
    return m.group(1) if m else None


def check(path, names):
    """The errors of one paper, and its statements: label, level and fields."""
    text = path.read_text()
    body = text[text.index(r"\begin{document}"):]
    offset = text[: text.index(r"\begin{document}")].count("\n")
    errors = []
    # The boxes, their levels and their Lean names.
    boxes = list(re.finditer(r"\\begin\{evidence\}(?:\[[^\]]*\])?\{(\w+)\}\{", body))
    fields = []
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
        tested, examples, rendered = (field(tag, key) for key in ("tested", "examples", "rendered"))
        for key, text in (("examples", examples), ("rendered", rendered)):
            if text is not None and not re.search(r"\\lean\{", text):
                errors.append(f"a field {key} that cites no Lean name: {tag[:60]}")
        fields.append({
            "level": level,
            "tested": tested is not None,
            "examples": re.findall(r"\\lean\{([^}]*)\}", examples or ""),
            "rendered": re.findall(r"\\lean\{([^}]*)\}", rendered or ""),
        })
    # Every statement inside a box, or said to claim nothing.
    spans = [(box.start(), body.index(r"\end{evidence}", box.end())) for box in boxes]
    statements = []
    pattern = r"\\begin\{(%s)\}(?:\[[^\]]*\])?\s*(?:\\label\{([^}]*)\})?" % "|".join(STATEMENTS)
    for m in re.finditer(pattern, body):
        label = m.group(2) or f"{m.group(1)}-{len(statements) + 1}"
        inside = [i for i, (start, end) in enumerate(spans) if start < m.start() < end]
        if inside:
            statements.append({"label": label, **fields[inside[0]]})
            continue
        statements.append({"label": label, "level": "none", "tested": False, "examples": [], "rendered": []})
        before = body[:m.start()].rstrip().rsplit("\n", 1)[-1]
        if not before.startswith("% evidence: none"):
            errors.append(f"line {offset + body[: m.start()].count(chr(10)) + 1}: a {m.group(1)} without a level")
    levels = [box.group(1) for box in boxes]
    return errors, statements, levels


def progress(statements):
    """The steps of the paper towards a preprint, as --json writes them."""
    steps = []
    for key, name_en, name_fr, counts in PROGRESS:
        labels = [s["label"] for s in statements if counts(s)]
        steps.append({"key": key, "name_en": name_en, "name_fr": name_fr,
                      "n": len(labels), "m": len(statements), "labels": labels})
    levels = {}
    for s in statements:
        levels[s["level"]] = levels.get(s["level"], 0) + 1
    return {"paper": "Turns of a T-mesh", "steps": steps, "preprint": PREPRINT,
            "levels": dict(sorted(levels.items())), "statements": len(statements)}


def main():
    arguments = sys.argv[1:]
    if arguments and (len(arguments) != 2 or arguments[0] != "--json"):
        sys.exit("usage: evidence.py [--json PATH]")
    names = declarations()
    failed = False
    papers = [HERE / "paper.tex"] + [path for path in [HERE / "paper.fr.tex"] if path.exists()]
    levels, statements = {}, {}
    for path in papers:
        errors, statements[path.name], levels[path.name] = check(path, names)
        for error in errors:
            print(f"evidence: {path.name}: {error}")
        counts = {level: levels[path.name].count(level) for level in sorted(LEVELS)}
        print(f"evidence: {path.name}: {len(statements[path.name])} statements, {len(levels[path.name])} boxes {counts}")
        failed |= bool(errors)
    # The translation gives each statement the level, the label and the
    # fields of the original.
    if len(papers) == 2 and levels["paper.tex"] != levels["paper.fr.tex"]:
        print("evidence: paper.tex and paper.fr.tex give different levels, in order")
        failed = True
    if len(papers) == 2 and statements["paper.tex"] != statements["paper.fr.tex"]:
        print("evidence: paper.tex and paper.fr.tex give different labels or fields, in order")
        failed = True
    print(f"evidence: {len(names)} Lean declarations")
    result = progress(statements["paper.tex"])
    print("evidence: " + " → ".join(f"{s['name_en']} {s['n']}/{s['m']}" for s in result["steps"])
          + f" → Preprint: {'yes' if result['preprint'] else 'no'}")
    if arguments:
        Path(arguments[1]).write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
