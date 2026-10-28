#!/usr/bin/env python3
"""The whole balanced lengths as a lattice, and what the shifts of the held
turns leave of it (Section "Whole lengths and size", paragraph after
Proposition "Free arcs"): for each way, the balances eliminated exactly in
whole numbers (rank, dimension of the lattice, equations whose coefficients
had a common divisor, steps of Euclid's algorithm), then the shifts of the
held turns (dimension left, Euclid steps, largest entry of the basis).

    python3 docs/research/turns-of-a-t-mesh/experiments/lattice.py [<export>] [--seconds 120]

The elimination of a way stops after --seconds (the coefficients can grow:
on the papers' body, way 7 did not finish); the summary says so. Seconds to
minutes a way. Writes results/lattice.json.
"""

import argparse
import time

import tmesh  # first: it sets the threads of the linear algebra
from integer import Lattice


def eliminate(way, seconds):
    clock = time.monotonic()
    lattice = Lattice(way.n)
    rank = sum(lattice.impose(list(row.items())) for row in way.rows)
    balances = {"rank": rank, "dimension": lattice.dimension, "divisors": list(lattice.divisors),
                "non_unit_equations": lattice.non_unit_equations, "euclid_steps": lattice.euclid_steps,
                "seconds": round(time.monotonic() - clock, 1)}
    before = (len(lattice.divisors), lattice.non_unit_equations, lattice.euclid_steps)
    rank, done = 0, 0
    forms = tmesh.held_forms(way)
    finished = True
    for shift in forms:
        if time.monotonic() - clock > seconds:
            finished = False
            break
        rank += lattice.impose(list(shift.items()))
        done += 1
    held = {"turns_imposed": done, "of": len(forms), "finished": finished, "rank_beyond_balances": rank,
            "dimension": lattice.dimension, "divisors": lattice.divisors[before[0]:],
            "non_unit_equations": lattice.non_unit_equations - before[1],
            "euclid_steps": lattice.euclid_steps - before[2], "largest_entry_digits": lattice.largest_entry_digits(),
            "seconds": round(time.monotonic() - clock, 1)}
    return balances, held


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    tmesh.arguments(parser, "lattice")
    parser.add_argument("--seconds", type=float, default=120.0, help="time limit of the elimination of a way")
    arguments = parser.parse_args()
    started, clock = tmesh.common.machine(), time.monotonic()
    folder, ways = tmesh.load(arguments)
    rows = []
    for way in ways:
        balances, held = eliminate(way, arguments.seconds)
        rows.append({"way": way.number, "arcs": way.n, "balances": balances, "held_turns": held})
        print(f"way {way.number}: balances rank {balances['rank']}, lattice {balances['dimension']}, divisors "
              f"{balances['divisors']}, Euclid {balances['euclid_steps']}; with the {held['turns_imposed']} of "
              f"{held['of']} held turns: dimension {held['dimension']}, Euclid {held['euclid_steps']}, largest "
              f"entry {held['largest_entry_digits']} digits{'' if held['finished'] else ' (stopped)'} "
              f"({held['seconds']} s)", flush=True)
    summary = {
        "experiment": "the lattice of whole balanced lengths and of the held turns",
        **tmesh.record(arguments, folder, ways, started, clock, seconds_per_way=arguments.seconds),
        "per_way": rows,
    }
    tmesh.write(arguments.summary, summary)


if __name__ == "__main__":
    main()
