# The night of the turns paper's data (and the export both papers read)

What a night run executes, in order, from the root of the main checkout on the commit the papers
cite (version 0.7.0, `e13fadb`, or a later commit whose engine is the same: `git diff --quiet e13fadb
HEAD -- crates Cargo.toml Cargo.lock`), on an idle machine, after `cargo build --release`. The
exact section of the closing-loops paper follows (`../../closing-loops/experiments/exact/NIGHT.md`);
it needs stage 1 here.

```sh
BIN=target/release/openclare
FROZEN=e13fadb
B=docs/research/turns-of-a-t-mesh/experiments
D=scratch/data-package/turns-of-a-t-mesh
SOURCE=scratch/torso/source.obj        # the male body; MD5 f1fe33002b731d1d41846daed6c59f5e,
                                       # the same file as bench/out/human_male/source.obj
export HIGHS_THREADS=1                 # every HiGHS solve of the scripts on one thread (recorded)
```

## 1. The two exports (about 2 minutes each, the whole machine)

The papers describe the engine of their first export, before the constraint rounds. `before-cuts`
switches off, on the engine of `e13fadb`, the stages it did not have (no constraint rounds, no
active set, no floor of the cap, no fold guard, no other draws): it is the export the papers'
numbers are made again on. `as-is` is the engine as users get it: it answers whether the
statements hold for it too.

```sh
ulimit -S -v 24000000
python3 $B/export.py --openclare $BIN --commit $FROZEN --engine before-cuts --source $SOURCE
python3 $B/export.py --openclare $BIN --commit $FROZEN --engine as-is --source $SOURCE
ulimit -S -v 16000000
E=$D/male-body-before-cuts             # the papers' export from here on
N=$D/male-body                         # the engine as it is
```

Each writes `<dir>/export/` (8 ways expected: `record.json` lists them), `mesh.obj`, `run.log`,
`measure.json` and `record.json`.

## 2. The Lean check of both exports (about 15 minutes each, 3 jobs)

```sh
make lean-check EXPORT=$E/export JOBS=3
make lean-check EXPORT=$N/export JOBS=3
```

## 3. The quick scripts on the papers' export (10 minutes in all, one thread each)

```sh
python3 $B/census.py $E
python3 $B/lattice.py $E --seconds 600          # way 7 stops at 600 s
python3 $B/conflicts.py $E                       # about 3 minutes
python3 $B/pairs.py $E
python3 $B/combs.py $E
python3 $B/forced.py $E --steps bounds --summary $B/results/forced-bounds.json   # about 2 minutes
```

## 4. The long scripts on the papers' export (one HiGHS thread per process, 4 processes)

The limits are the papers' (120 to 1,800 seconds) and are recorded in each summary; run nothing
else meanwhile.

```sh
python3 $B/forced.py $E --whole-lower --seconds 900 --budget 1800 --jobs 4    # 1 to 4 hours (9 families)
python3 $B/forced.py $E --ways 2 --families held --sound --seconds 900 \
    --summary $B/results/forced-sound.json --out $D/forced-sound              # under an hour
python3 $B/holonomy.py $E --jobs 4                                              # about an hour
# The certificates of Table tab:iis, with the rows conflicts.py found, for the Lean check: four
# processes of two ways each (certify.py takes 20 to 70 minutes of processor time in all).
for part in "0 1" "2 3" "4 5" "6 7"; do
  set -- $part
  mkdir -p $E/export/certificates/part-$1
  python3 docs/research/turns-of-a-t-mesh/lean/Check/certify.py \
      $E/export/call-*/way-$1.json $E/export/call-*/way-$2.json \
      --claims $D/conflicts/claims.json --out $E/export/certificates/part-$1/certificates.json &
done; wait
make lean-check EXPORT=$E/export JOBS=3          # again: now with the table's certificates
```

## 5. The engine as it is (quick scripts only, 15 minutes)

```sh
R=$B/results/as-is
python3 $B/census.py $N --out $D/as-is/census --summary $R/census.json
python3 $B/conflicts.py $N --out $D/as-is/conflicts --summary $R/conflicts.json
python3 $B/pairs.py $N --table $R/conflicts.json --out $D/as-is/pairs --summary $R/pairs.json
python3 $B/combs.py $N --table $R/conflicts.json --out $D/as-is/combs --summary $R/combs.json
python3 $B/lattice.py $N --seconds 600 --out $D/as-is/lattice --summary $R/lattice.json
python3 $B/forced.py $N --steps bounds --out $D/as-is/forced --summary $R/forced-bounds.json
```

## Times and order

| Stage | Duration (estimated, idle i7-7700K) | Priority |
|---|---|---|
| 1. exports | 4 minutes | first |
| 2. Lean check | 30 minutes | high |
| 3. quick scripts | 10 minutes | high |
| 4. `forced.py` | 1 to 4 hours | high (Table `tab:forced`) |
| 4. `holonomy.py` | 1 hour | medium |
| 4. `certify.py` and the Lean check again | 1 hour | medium |
| 4. `forced.py --sound` | under an hour | low |
| 5. as-is | 15 minutes | low |

The summaries go to `results/` (and `results/as-is/`), to be committed with the commit and the
machine they name; the raw output stays under `scratch/data-package/`.
