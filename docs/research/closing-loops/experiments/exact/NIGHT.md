# The night of the exact section of the closing-loops paper

What a night run executes for Section `sec:exact`, in order, from the root of the main checkout on
the commit the paper cites, on an idle machine, after `cargo build --release` and after stage 1
(the exports) of `../../../turns-of-a-t-mesh/experiments/NIGHT.md`, whose stages 3 and 4 make the
numbers this section shares with the turns paper (census, conflicts, `forced.py`).

```sh
BIN=target/release/openclare
FROZEN=e13fadb
B=docs/research/turns-of-a-t-mesh/experiments
A=docs/research/closing-loops/experiments/exact
E=scratch/data-package/turns-of-a-t-mesh/male-body-before-cuts   # the papers' export
SOURCE=scratch/torso/source.obj
export HIGHS_THREADS=1
```

## 1. Quick (a few minutes, one thread)

```sh
python3 $A/relaxation.py $E
```

## 2. Time-limited programs (one HiGHS thread per process, at most 4 processes at a time)

The limits are those the paper states (DT-090: run on the idle machine, the thread count is in
each summary).

```sh
# Table tab:exact (300 s a way), the whole closing of each closable set (600 s a way: "about two
# minutes a way ... the fourth did not finish within ten minutes") and the table's program in
# whole numbers (900 s a way: "did not finish within fifteen minutes"). 8 ways, 4 at a time:
# about 1.5 hours.
python3 $A/most_closable.py $E --whole 600 --whole-most 900 --jobs 4
# The rings of way 2 (the subset's program 600 s; whole lengths without the zeros 1,800 s: "no
# whole solution in ten or thirty minutes"), then their rounding in the lattice (1,200 s), then
# the local search (1,200 s): about 1.5 hours in one lane, alongside the line above.
python3 $A/rings.py $E --subset-seconds 600 --without-zeros 1800
python3 $A/lattice_rounding.py $E --seconds 1200
python3 $A/local_search.py $E --seconds 1200
```

## 3. The engine's meshes (the whole machine, one run at a time, about 30 minutes)

After stage 2, whose files they read (`aimed-<way>.txt`, `way-<way>-integer.txt`,
`call-<name>.txt`): Table `tab:rounding`, the meshes of the whole closings way by way, of the
rounded rings and of the local search.

```sh
ulimit -S -v 24000000
python3 $A/remesh.py --openclare $BIN --commit $FROZEN --engine before-cuts --source $SOURCE
ulimit -S -v 16000000
```

## 4. The slow quality mode end to end (hours)

Deterministic variants first, two runs at a time; then the time-limited ones alone, one run at a
time on four fixed processors (the engine does not set HiGHS's threads: the processors it may
use are fixed instead, and stated in the summary).

```sh
python3 $A/end_to_end.py --openclare $BIN --commit $FROZEN --variants draws-0,draws-3,engine,slow-draws-0 \
    --jobs 2 --male-body                                   # 2 to 4 hours
python3 $A/end_to_end.py --openclare $BIN --commit $FROZEN --variants slow-time-5 \
    --jobs 1 --cpus 0-3 --male-body \
    --summary $A/results/end_to_end-time-5.json            # 1 to 2 hours
python3 $A/end_to_end.py --openclare $BIN --commit $FROZEN --variants slow-time-300 \
    --jobs 1 --cpus 0-3 \
    --summary $A/results/end_to_end-time-300.json          # hours; lowest priority
```

## Times and order

| Stage | Duration (estimated, idle i7-7700K) | Priority |
|---|---|---|
| 1. `relaxation.py` | seconds | high |
| 2. `most_closable.py` | 1.5 hours with 4 processes | high (Table `tab:exact`, the 10- and 15-minute statements) |
| 2. `rings.py`, `lattice_rounding.py`, `local_search.py` | 1.5 hours in one lane | high (the 1,200-second run and its bound) |
| 3. `remesh.py` | 30 minutes | high (Table `tab:rounding`) |
| 4. deterministic variants | 2 to 4 hours | medium |
| 4. `slow-time-5` | 1 to 2 hours | high (the 5-second statement, DT-090) |
| 4. `slow-time-300` | hours | low |

The summaries go to `results/`, to be committed with the commit and the machine they name; the
raw output stays under `scratch/data-package/closing-loops/`.
