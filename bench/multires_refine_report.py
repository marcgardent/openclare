"""The report of the batch multires-refine (milestone 6 of the Multires
specification): a cage remeshed plain, and remeshed again with the density
Refine the Cage paints from its top level, each taken to L4, compared, from
the records lab wrote for its cells and the two runs each job left in its
folder (plain/, refined/).

    python3 bench/multires_refine_report.py <records.json> [--out DIR]

The criterion, per row (a model at a quad count): at L4, the missing
detail's 99.9th percentile, what a level misses in a few places only (a
fingertip, a hollow under an arm), is lower once refined, and its p99 not
worse (within 0.002 point, the third digit the bench prints). The cage
faces told to want more quads are shown, not judged: the refinement fixes
small faces and may flag a few large ones just above the target, so their
area rose in two draws of three while the local misses fell in all three
(Marc, 2026-11-19).

Writes report.md into `--out` (the folder of records.json by default) and
prints it.
"""

import argparse
import json
from pathlib import Path

from multires_advice_report import cage_areas

WORSE_BY = 0.002


def at_top(run):
    """What a run of multires.py --advice says of its top level: quads of
    the cage, missing p99 and p99.9, stretched edges, the share of the
    cage's area told to want more quads."""
    detail = json.loads((run / "detail.json").read_text())
    top = detail["levels"][-1]
    verdict = json.loads((run / f"advice-{top['level']}.json").read_text())["verdict"]
    areas = cage_areas(detail["cage"])
    share = sum(a for a, v in zip(areas, verdict) if v == 2) / sum(areas)
    return {
        "quads": detail["levels"][0]["faces"], "p99": top["missing_p99"], "p999": top["missing_p999"],
        "stretched": top["stretched"], "denser": verdict.count(2), "share": share,
    }


def text(said):
    return (f"{said['quads']} q, {said['p99']:.3f} / {said['p999']:.2f}, {said['stretched']} str., "
            f"{said['denser']} faces ({said['share']:.1%})")


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("records")
    parser.add_argument("--out")
    options = parser.parse_args()
    records = json.loads(Path(options.records).read_text())

    lines = [
        f"# {records['batch']}: Refine the Cage against milestone 6",
        "",
        "At L4: quads of the cage, missing p99 / p99.9 (% of the diagonal), stretched edges, and the cage "
        "faces told to want more quads with their share of the cage's area.",
        "",
        "| row | plain | refined | verdict |",
        "|---|---|---|---|",
    ]
    met = judged = 0
    for cell in records["cells"]:
        row = cell["values"]["row"]
        try:
            if cell.get("status") != "ok":
                raise OSError
            plain, refined = (at_top(Path(cell["out"]) / name) for name in ("plain", "refined"))
        except (OSError, ValueError, KeyError):
            lines.append(f"| {row} | {cell.get('status') or 'never run'} | | |")
            continue
        fine = refined["p999"] < plain["p999"] and refined["p99"] <= plain["p99"] + WORSE_BY
        judged += 1
        met += fine
        lines.append(f"| {row} | {text(plain)} | {text(refined)} | {'met' if fine else 'not met ✗'} |")
    lines += ["", f"{met} of {judged} rows meet the milestone."]
    report = "\n".join(lines) + "\n"
    out = Path(options.out) if options.out else Path(options.records).parent
    (out / "report.md").write_text(report)
    print(report, end="")


if __name__ == "__main__":
    main()
