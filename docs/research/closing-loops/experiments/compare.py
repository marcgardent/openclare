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


def whole(value):
    return None if value is None else f"{value:,}"


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
    out = [
        # (what, text to find in paper.tex, the paper's value, the engine's value)
        ("Twelve models, three budgets", None, None, None),
        ("spiral edges, mean of 36 runs", "4.65", "4.65%", percent(get(twelve, "mean", "loop_edges_spiral"))),
        ("… at 2,970 quads", "2.89", "2.89%", percent(get(twelve, "mean_by_budget", "2970", "loop_edges_spiral"))),
        ("… at 3,000 quads", "4.97", "4.97%", percent(get(twelve, "mean_by_budget", "3000", "loop_edges_spiral"))),
        ("… at 3,030 quads", "6.09", "6.09%", percent(get(twelve, "mean_by_budget", "3030", "loop_edges_spiral"))),
        ("spiral loops in the 36 meshes", "$102$", "102", whole(get(twelve, "spiral_loops_total"))),
        ("valid meshes", "all valid", "36 of 36",
         None if twelve is None else f"{twelve['valid']} of {twelve['runs']}"),
        ("closed-loop edges", "36.7", "36.7%", percent(get(twelve, "mean", "loop_edges_closed"), 1)),
        ("mean distance, ‰ of the diagonal", "0.377", "0.377", mille(get(twelve, "mean", "mean_distance"))),
        ("mean corner deviation, degrees", "4.90", "4.90", plain(get(twelve, "mean", "angle_deviation_mean"))),
        ("edges on helices of any pitch (DT-137)", None, "—", percent(get(twelve, "mean", "loop_edges_helix"))),
        ("helices of pitch above 4, per run (DT-137)", None, "—", plain(get(twelve, "mean", "loops_helix_missed"))),
        ("Table tab:dev (paper: single runs at 3,000, four engines; here one engine, means over 36 runs, "
         "at 3,000 in brackets)", None, None, None),
    ]
    for key, text, spiral, hausdorff, corners in [
        ("no_closing", "no closing &", "16.7%", "8.77", "4.75"),
        ("first_closing", "closing, one way kept", "9.7%", "8.57", "4.85"),
        ("leaning_aimed", "+ leaning ways, aimed", "9.1%", "8.58", "4.91"),
        ("longer_cap", "+ longer turns, deviation cap &", "6.0%", "8.68", "5.01"),
    ]:
        both = lambda measure, show: None if line(key, measure) is None else \
            f"{show(line(key, measure))} ({show(line(key, measure, True))})"
        out += [
            (f"{key}: spiral edges", text, spiral, both("loop_edges_spiral", percent)),
            (f"{key}: Hausdorff, ‰", text, hausdorff, both("hausdorff", lambda v: mille(v, 2))),
            (f"{key}: corner deviation, degrees", text, corners, both("angle_deviation_mean", plain)),
        ]
    out.append(("leaning alone (ways 0,1,2,3, no aimed repair): spiral edges", "$10.1\\%$ to $9.5",
                "9.5% (10.1% before, commit 452b346)",
                None if line("leaning", "loop_edges_spiral") is None else
                f"{percent(line('leaning', 'loop_edges_spiral'))} ({percent(line('leaning', 'loop_edges_spiral', True))})"))
    for key in ["longer", "cut_rounds", "active_set", "deviation_floor", "engine"]:
        out.append((f"{key}: spiral edges (not in the table)", None, "—",
                    None if line(key, "loop_edges_spiral") is None else
                    f"{percent(line(key, 'loop_edges_spiral'))} ({percent(line(key, 'loop_edges_spiral', True))})"))
    out += [
        ("Against other remeshers (bench at 3,000 quads)", None, None, None),
        ("models QuadriFlow produces a mesh for", "produces a mesh for $22$", "22",
         whole(get(current, "produce_a_mesh", "quadriflow"))),
        ("models AutoRemesher produces a mesh for", "AutoRemesher for $18$", "18",
         whole(get(current, "produce_a_mesh", "autoremesher"))),
        ("models all three mesh validly in every run", "On the $7$ models", "7",
         whole(None if valid is None else len(valid["models"]))),
        ("… openclare spiral edges", "$1.8\\%$ for openclare", "1.8%", percent(get(valid, "means", "openclare",
                                                                                    "loop_edges_spiral"), 1)),
        ("… QuadriFlow", "$38.8\\%$ for", "38.8%", percent(get(valid, "means", "quadriflow", "loop_edges_spiral"), 1)),
        ("… AutoRemesher", "$21.7\\%$ for AutoRemesher", "21.7%",
         percent(get(valid, "means", "autoremesher", "loop_edges_spiral"), 1)),
        ("… openclare without closing", "had $16.6\\%$", "16.6%",
         percent(get(no_closing, "valid_in_every_run", "loop_edges_spiral"), 1)),
        ("models all three produce a mesh for", "on the $15$ models", "15",
         whole(None if produced is None else len(produced["models"]))),
        ("… openclare spiral edges", "$4.2\\%$, $27.2", "4.2%",
         percent(get(produced, "means", "openclare", "loop_edges_spiral"), 1)),
        ("… QuadriFlow", "$27.2\\%$", "27.2%", percent(get(produced, "means", "quadriflow", "loop_edges_spiral"), 1)),
        ("… AutoRemesher", "$27.3\\%$. Without", "27.3%",
         percent(get(produced, "means", "autoremesher", "loop_edges_spiral"), 1)),
        ("… openclare without closing", "and $15.5\\%$", "15.5%",
         percent(get(no_closing, "all_produce_a_mesh", "loop_edges_spiral"), 1)),
        ("… helices of any pitch, openclare / QuadriFlow / AutoRemesher (DT-137)", None, "—",
         None if produced is None else " / ".join(percent(produced["means"][tool]["loop_edges_helix"], 1)
                                                  for tool in ["openclare", "quadriflow", "autoremesher"])),
        ("Male body, 10,590 quads", None, None, None),
        ("hand-made: spiral edges", "with $2.5\\%$ of spiral", "2.5%",
         percent(get(male, "handmade", "loop_edges_spiral"), 1)),
        ("hand-made: spiral loops", "in $3$ spiral loops", "3", whole(get(male, "handmade", "loops_spiral"))),
        ("hand-made: closed-loop edges", "$31.6\\%$ of its edges on closed", "31.6%",
         percent(get(male, "handmade", "loop_edges_closed"), 1)),
        ("openclare, closing: spiral edges", "give $1.6\\%$ of spiral", "1.6%",
         percent(get(body, "closing", "loop_edges_spiral"))),
        ("openclare, closing: spiral loops", "in $9$ spiral loops", "9", whole(get(body, "closing", "loops_spiral"))),
        ("openclare, closing: closed-loop edges", "$33.2\\%$", "33.2%",
         percent(get(body, "closing", "loop_edges_closed"), 1)),
        ("openclare, closing: mean distance, ‰", "$0.19$", "0.19", mille(get(body, "closing", "mean_distance"), 2)),
        ("openclare, closing: corner deviation", "deviation of $4.1$", "4.1",
         plain(get(body, "closing", "angle_deviation_mean"), 1)),
        ("hand-made: mean distance, ‰", "against $0.17$", "0.17", mille(get(male, "handmade", "mean_distance"), 2)),
        ("hand-made: corner deviation", "and $9.2$ degrees", "9.2",
         plain(get(male, "handmade", "angle_deviation_mean"), 1)),
        ("lean bound 0.05 alone: spiral edges", "$12.1\\%$ of spiral", "12.1%",
         percent(get(body, "lean_005", "loop_edges_spiral"), 1)),
        ("openclare, closing: helices of any pitch (DT-137)", None, "—",
         percent(get(body, "closing", "loop_edges_helix"), 2)),
        ("Figure fig:overview (caption; the meshes of figures/make_figures.sh)", None, None, None),
    ]
    meshes = get(figures, "meshes")
    for name, label, quads, spiral, loops, closed in [
        ("no_closing", "(a) no closing", "10{,}485", "26.8%", "18", "10.2%"),
        ("closing", "(b) closing", "10{,}748", "1.95%", "15", "32.6%"),
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
