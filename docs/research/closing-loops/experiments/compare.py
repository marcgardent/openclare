#!/usr/bin/env python3
"""Each number of the applied experiments of the paper next to the value the
summaries in results/ give: the table "paper says / engine gives" of
README.md, in Markdown, on the standard output.

    python3 docs/research/closing-loops/experiments/compare.py [--paper paper.tex]

A number is found in the paper by its text; the line printed is where it is
in paper.tex now (the first match after the line given, so that the table
follows the paper when it is edited). A summary that is missing gives "—".
"""

import argparse
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"


def load(name):
    path = RESULTS / f"{name}.json"
    return json.loads(path.read_text()) if path.exists() else None


def get(data, *keys):
    for key in keys:
        if data is None:
            return None
        data = data[key] if isinstance(data, list) else data.get(key)
    return data


def percent(value, digits=2):
    return None if value is None else f"{100 * value:.{digits}f}%"


def mille(value, digits=3):
    return None if value is None else f"{1000 * value:.{digits}f}"


def plain(value, digits=2):
    return None if value is None else f"{value:.{digits}f}"


def unit(value):
    """A share to the unit of a per cent, as the paper gives a share whose
    interval is several points wide (#222); halves rounded up."""
    from decimal import ROUND_HALF_UP, Decimal

    return None if value is None else f"{Decimal(str(100 * value)).quantize(Decimal('1'), ROUND_HALF_UP)}%"


def interval(spread):
    if spread is None:
        return None
    low, high = (unit(value)[:-1] for value in spread["interval_95"])
    return f"{low}–{high}%"


def whole(value):
    return None if value is None else f"{value:,}"


def counts(subset):
    against = get(subset, "openclare_against")
    return None if against is None else ", ".join(
        f"{c['fewer']}/{c['more']}/{c['as_many']}" for c in against.values())


def rows():
    twelve, ablation, remeshers, male = load("twelve_models"), load("ablation"), load("against_remeshers"), \
        load("male_body")
    figures = load("figures")
    lines = {line["key"]: line for line in get(ablation, "lines") or []}

    def line(key, measure, at3000=False):
        return get(lines.get(key), "mean_at_3000" if at3000 else "mean", measure)

    current = get(remeshers, "current")
    valid, produced = get(current, "subsets", "valid_in_every_run"), get(current, "subsets", "all_produce_a_mesh")
    no_closing = get(remeshers, "no_closing", "means")
    body = get(male, "openclare")
    spread = get(twelve, "spiral")
    out = [
        # (what, text to find in paper.tex, the paper's value, the engine's value)
        ("Twelve models, 21 budgets (252 runs)", None, None, None),
        ("spiral edges, mean", "leaves $5\\%$ of the edges", "5%", unit(get(spread, "mean"))),
        ("… its 95% interval", "($2$ to", "2–11%", interval(spread)),
        ("… median", "median of $1.7", "1.7%", percent(get(spread, "median"), 1)),
        ("… runs with none", "none on $42", "42%", unit(get(spread, "share_of_runs_at_zero"))),
        ("… models with some", "each of the twelve models", "12",
         whole(get(spread, "models_above_zero"))),
        ("spiral loops in the 252 meshes", "$802$", "802", whole(get(twelve, "spiral_loops_total"))),
        ("valid meshes", "all valid but one", "251 of 252",
         None if twelve is None else f"{twelve['valid']} of {twelve['runs']}"),
        ("closed-loop edges", "$36\\%$ of their", "36%", unit(get(twelve, "mean", "loop_edges_closed"))),
        ("mean distance, ‰ of the diagonal", "$0.38$", "0.38", mille(get(twelve, "mean", "mean_distance"), 2)),
        ("mean corner deviation, degrees", "$5.1$ degrees", "5.1",
         plain(get(twelve, "mean", "angle_deviation_mean"), 1)),
        ("edges on helices of any pitch (DT-137)", None, "—", percent(get(twelve, "mean", "loop_edges_helix"))),
        ("helices of pitch above 4, per run (DT-137)", None, "—", plain(get(twelve, "mean", "loops_helix_missed"))),
        ("Table tab:dev: mean, interval, runs with none, lower/higher than the line above, Hausdorff, corners",
         None, None, None),
    ]
    for key, text, said in [
        ("no_closing", "no closing &", "19% · 13–27% · 6% · — · 8.16 · 4.91"),
        ("first_closing", "closing: lean bound", "11% · 7–17% · 15% · 190/0 · 8.25 · 4.97"),
        ("leaning", "+ lean bound $0.12$ &", "10% · 6–15% · 16% · 72/0 · 8.25 · 4.99"),
        ("leaning_aimed", "+ aimed repair &", "10% · 5–14% · 19% · 77/34 · 8.25 · 5.01"),
        ("longer", "+ turns up to $6$ sides &", "7% · 4–11% · 28% · 102/0 · 8.27 · 5.06"),
        ("longer_cap", "+ deviation cap, without its floor &", "8% · 4–14% · 26% · 0/23 · 8.26 · 5.02"),
        ("cut_rounds", "+ constraint rounds &", "7% · 3–13% · 32% · 90/0 · 8.25 · 5.06"),
        ("active_set", "+ active set &", "8% · 3–14% · 31% · 82/84 · 8.16 · 5.09"),
        ("deviation_floor", "+ floor of the cap &", "7% · 3–12% · 34% · 35/0 · 8.25 · 5.14"),
        ("fold_guard", "+ fold guard &", "7% · 3–12% · 34% · 0/1 · 8.25 · 5.14"),
        ("engine", "the engine &", "5% · 2–11% · 42% · 95/0 · 8.35 · 5.12"),
    ]:
        data = lines.get(key)
        spiral, against = get(data, "spiral"), get(data, "against_previous_line")
        value = None if data is None else " · ".join([
            unit(spiral["mean"]), interval(spiral), unit(spiral["share_of_runs_at_zero"]),
            f"{against['lower']}/{against['higher']}" if against else "—",
            mille(line(key, "hausdorff"), 2), plain(line(key, "angle_deviation_mean"))])
        out.append((key, text, said, value))
    out += [
        ("Against other remeshers (bench at 3,000 quads)", None, None, None),
        ("models QuadriFlow produces a mesh for", "produces a mesh for $22$", "22",
         whole(get(current, "produce_a_mesh", "quadriflow"))),
        ("models AutoRemesher produces a mesh for", "AutoRemesher for $18$", "18",
         whole(get(current, "produce_a_mesh", "autoremesher"))),
        ("models all three mesh validly in every run", "On the $7$ models", "7",
         whole(None if valid is None else len(valid["models"]))),
        ("… openclare spiral edges", "is $2.2\\%$ for", "2.2%", percent(get(valid, "means", "openclare",
                                                                                    "loop_edges_spiral"), 1)),
        ("… QuadriFlow", "$38.8\\%$ for", "38.8%", percent(get(valid, "means", "quadriflow", "loop_edges_spiral"), 1)),
        ("… AutoRemesher", "$21.7\\%$ for AutoRemesher", "21.7%",
         percent(get(valid, "means", "autoremesher", "loop_edges_spiral"), 1)),
        ("… openclare without closing", "openclare has $15.3\\%$", "15.3%",
         percent(get(no_closing, "valid_in_every_run", "loop_edges_spiral"), 1)),
        ("models all three produce a mesh for", "on the $15$ models", "15",
         whole(None if produced is None else len(produced["models"]))),
        ("… openclare spiral edges", "$2.9\\%$, $27.2", "2.9%",
         percent(get(produced, "means", "openclare", "loop_edges_spiral"), 1)),
        ("… QuadriFlow", "$27.2\\%$", "27.2%", percent(get(produced, "means", "quadriflow", "loop_edges_spiral"), 1)),
        ("… AutoRemesher", "$27.3\\%$. Without", "27.3%",
         percent(get(produced, "means", "autoremesher", "loop_edges_spiral"), 1)),
        ("… openclare without closing", "and $15.2\\%$", "15.2%",
         percent(get(no_closing, "all_produce_a_mesh", "loop_edges_spiral"), 1)),
        ("… openclare fewer/more/as many, 7 models: QuadriFlow, AutoRemesher", "on $6$ of the $7$ models",
         "6/0/1, 5/0/2", counts(valid)),
        ("… the same, 15 models", "fewer than QuadriFlow on $13$", "13/0/2, 11/1/3", counts(produced)),
        ("… helices of any pitch, openclare / QuadriFlow / AutoRemesher (DT-137)", None, "—",
         None if produced is None else " / ".join(percent(produced["means"][tool]["loop_edges_helix"], 1)
                                                  for tool in ["openclare", "quadriflow", "autoremesher"])),
        ("Male body, 10,590 quads", None, None, None),
        ("hand-made: spiral edges", "with $2.5\\%$ of spiral", "2.5%",
         percent(get(male, "handmade", "loop_edges_spiral"), 1)),
        ("hand-made: spiral loops", "in $3$ spiral loops", "3", whole(get(male, "handmade", "loops_spiral"))),
        ("hand-made: closed-loop edges", "$31.6\\%$ of its edges on closed", "31.6%",
         percent(get(male, "handmade", "loop_edges_closed"), 1)),
        ("openclare, closing: spiral edges", "it has $1.46\\%$ of spiral", "1.46%",
         percent(get(body, "closing", "loop_edges_spiral"))),
        ("openclare, closing: spiral loops", "edges, in $11$ spiral loops", "11", whole(get(body, "closing", "loops_spiral"))),
        ("openclare, closing: closed-loop edges", "It has $36.5\\%$", "36.5%",
         percent(get(body, "closing", "loop_edges_closed"), 1)),
        ("openclare, closing: mean distance, ‰", "$0.19$", "0.19", mille(get(body, "closing", "mean_distance"), 2)),
        ("openclare, closing: corner deviation", "deviation of $4.3$", "4.3",
         plain(get(body, "closing", "angle_deviation_mean"), 1)),
        ("hand-made: mean distance, ‰", "against $0.17$", "0.17", mille(get(male, "handmade", "mean_distance"), 2)),
        ("hand-made: corner deviation", "and $9.2$ degrees", "9.2",
         plain(get(male, "handmade", "angle_deviation_mean"), 1)),
        ("lean bound 0.05 alone: spiral edges", "the body has $1.67\\%$ of spiral", "1.67%",
         percent(get(body, "lean_005", "loop_edges_spiral"))),
        ("openclare, closing: helices of any pitch (DT-137)", None, "—",
         percent(get(body, "closing", "loop_edges_helix"), 2)),
        ("Figure fig:overview (caption; the meshes of figures/make_figures.sh)", None, None, None),
    ]
    meshes = get(figures, "meshes")
    for name, label, quads, spiral, loops, closed in [
        ("no_closing", "(a) no closing", "10{,}625", "23.4%", "21", "17.2%"),
        ("closing", "(b) closing", "10{,}484", "1.46%", "11", "36.5%"),
        ("handmade", "(c) hand-made", "10{,}570", "2.55%", "3", "31.6%"),
        ("quadriflow", "(d) QuadriFlow", "8{,}844", "35.7%", "43", "2.7%"),
    ]:
        mesh = get(meshes, name)
        out += [
            (f"{label}: quads", f"${quads}$", quads.replace("{,}", ","), whole(get(mesh, "quads"))),
            (f"{label}: spiral edges (loops)", f"${spiral[:-1]}\\%$ (${loops}$", f"{spiral} ({loops})",
             None if mesh is None else f"{percent(mesh['loop_edges_spiral'])} ({mesh['loops_spiral']})"),
            (f"{label}: closed-loop edges", f"${closed[:-1]}\\%$", closed, percent(get(mesh, "loop_edges_closed"), 1)),
        ]
    out += [
        ("(d) QuadriFlow: inverted faces", "has $3$ inverted", "3", whole(get(meshes, "quadriflow", "inverted_faces"))),
        ("(d) QuadriFlow: Hausdorff, ‰", "$31$\\textperthousand", "31",
         mille(get(meshes, "quadriflow", "hausdorff"), 1)),
    ]
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--paper", type=Path, default=HERE.parent / "paper.tex")
    arguments = parser.parse_args()
    paper = arguments.paper.read_text().splitlines()
    # Searched from the start of Section "Experiments", whose numbers these are.
    start = next((n for n, text in enumerate(paper) if "\\label{sec:experiments}" in text), 0)
    print("| | line | paper | engine |")
    print("|---|---|---|---|")
    for what, text, said, value in rows():
        if said is None:
            print(f"| **{what}** | | | |")
            continue
        where = next((str(n + 1) for n in range(start, len(paper)) if text and text in paper[n]), "")
        print(f"| {what} | {where} | {said} | {value or '—'} |")


if __name__ == "__main__":
    main()
