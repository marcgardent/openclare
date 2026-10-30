#!/usr/bin/env python3
"""Renders a TikZ figure of the paper as a PNG for the website.

    python3 docs/research/turns-of-a-t-mesh/figures/render.py [NAME ...]

Writes figures/<name>.png and figures/<name>.fr.png from <name>.tex and
<name>.fr.tex, each in a standalone document with the paper's TikZ libraries
and macros. Default: dislocation, the picture of paper B on the website.
Needs pdflatex and pdftoppm (poppler).
"""

import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESOLUTION = 220  # dots per inch: about 1,700 pixels wide for dislocation

PREAMBLE = r"""\documentclass[border=6pt]{standalone}
\usepackage[T1]{fontenc}
\usepackage[utf8]{inputenc}
\usepackage{lmodern}
\usepackage{amsmath,amssymb}
\usepackage{tikz}
\usetikzlibrary{arrows.meta,decorations.pathreplacing,patterns}
\newcommand{\Z}{\mathbb{Z}}
\newcommand{\R}{\mathbb{R}}
\newcommand{\shift}{s}
"""


def render(source, target):
    with tempfile.TemporaryDirectory() as work:
        work = Path(work)
        (work / "figure.tex").write_text(PREAMBLE + f"\\begin{{document}}\\input{{{source}}}\\end{{document}}\n")
        run = subprocess.run(
            ["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "figure.tex"],
            cwd=work, capture_output=True, text=True,
        )
        if run.returncode:
            sys.exit(f"{source.name}: pdflatex failed\n" + run.stdout[-2000:])
        subprocess.run(["pdftoppm", "-png", "-r", str(RESOLUTION), "-singlefile", "figure.pdf", "figure"], cwd=work, check=True)
        shutil.copyfile(work / "figure.png", target)
    print(f"written: {target.relative_to(HERE.parent.parent.parent.parent)}")


def main():
    for name in sys.argv[1:] or ["dislocation"]:
        for suffix in ("", ".fr"):
            render(HERE / f"{name}{suffix}.tex", HERE / f"{name}{suffix}.png")


if __name__ == "__main__":
    main()
