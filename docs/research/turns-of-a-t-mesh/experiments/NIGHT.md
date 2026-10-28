# The night of the turns paper's data (and the export both papers read)

What a night run executes, in order, from the root of the main checkout on the commit the papers
cite (version 0.7.0, `e13fadb`, or a later commit whose engine is the same: `git diff --quiet e13fadb
HEAD -- crates Cargo.toml Cargo.lock`), on an idle machine, after `cargo build --release`. The
exact section of the closing-loops paper follows (`../../closing-loops/experiments/exact/NIGHT.md`);
it needs stage 1 here.

The papers' numbers are made on the engine of v0.7.0 as published: the `as-is` export. Where an
example of the drafts (the wrist, the right eye, the set of 15 turns...) is not found on it, the
text is rewritten from what the scripts find. The `before-cuts` export is an aside: the stages of
the engine of the drafts' first export switched off, to compare with that export; its summaries
stay under `scratch/` and are never the papers' numbers.

```sh
BIN=target/release/openclare
FROZEN=e13fadb
B=docs/research/turns-of-a-t-mesh/experiments
D=scratch/data-package/turns-of-a-t-mesh
SOURCE=scratch/torso/source.obj        # the male body; MD5 f1fe33002b731d1d41846daed6c59f5e,
                                       # the same file as bench/out/human_male/source.obj
export HIGHS_THREADS=1                 # every HiGHS solve of the scripts on one thread (recorded)
```

## 1. The export (about 2 minutes, the whole machine)

```sh
ulimit -S -v 24000000
python3 $B/export.py --openclare $BIN --commit $FROZEN --engine as-is --source $SOURCE
ulimit -S -v 16000000
E=$D/male-body                         # the papers' export from here on (the published engine)
```

It writes `$E/export/` (8 ways expected: `record.json` lists them), `mesh.obj`, `run.log`,
`measure.json` and `record.json`.

## 2. The Lean check (about 15 minutes, 3 jobs)

```sh
make lean-check EXPORT=$E/export JOBS=3
```

## 3. The quick scripts (15 minutes in all, one thread each)

```sh
python3 $B/census.py $E
python3 $B/lattice.py $E --seconds 600          # a way whose coefficients grow stops at 600 s
python3 $B/conflicts.py $E                       # about 3 minutes
python3 $B/pairs.py $E
python3 $B/combs.py $E
python3 $B/fox.py $E --relations 2              # about 6 minutes
python3 $B/forced.py $E --steps bounds --summary $B/results/forced-bounds.json   # about 2 minutes
```

## 4. The long scripts (one HiGHS thread per process, 4 processes)

The limits are the papers' (120 to 1,800 seconds) and are recorded in each summary; run nothing
else meanwhile.

```sh
python3 $B/forced.py $E --whole-lower --seconds 900 --budget 1800 --jobs 4    # 1 to 4 hours (9 families)
python3 $B/forced.py $E --ways 2 --families held --sound --seconds 900 \
    --summary $B/results/forced-sound.json --out $D/forced-sound              # under an hour
python3 $B/holonomy.py $E --jobs 4                                              # about an hour
python3 $B/strict.py $E                                                         # 30 to 60 minutes
# The certificates of the table conflicts.py found, for the Lean check: four processes of two
# ways each (certify.py takes 20 to 70 minutes of processor time in all).
for part in "0 1" "2 3" "4 5" "6 7"; do
  set -- $part
  mkdir -p $E/export/certificates/part-$1
  python3 docs/research/turns-of-a-t-mesh/lean/Check/certify.py \
      $E/export/call-*/way-$1.json $E/export/call-*/way-$2.json \
      --claims $D/conflicts/claims.json --out $E/export/certificates/part-$1/certificates.json &
done; wait
make lean-check EXPORT=$E/export JOBS=3          # again: now with the table's certificates
```

(`export.py` may list fewer than 8 ways if the published engine tries fewer on this body: the
loop then names only the ways `record.json` lists.)

## 5. Aside: the engine of the drafts' first export (optional, 20 minutes)

For comparison with the export of the drafts only; the summaries stay in `scratch/`.

```sh
ulimit -S -v 24000000
python3 $B/export.py --openclare $BIN --commit $FROZEN --engine before-cuts --source $SOURCE
ulimit -S -v 16000000
C=$D/male-body-before-cuts
S=$D/before-cuts                        # its summaries: not results/
python3 $B/census.py $C --out $S/census --summary $S/census.json
python3 $B/conflicts.py $C --out $S/conflicts --summary $S/conflicts.json
python3 $B/pairs.py $C --table $S/conflicts.json --out $S/pairs --summary $S/pairs.json
python3 $B/combs.py $C --table $S/conflicts.json --out $S/combs --summary $S/combs.json
python3 $B/fox.py $C --table $S/conflicts.json --out $S/fox --summary $S/fox.json
python3 $B/lattice.py $C --seconds 600 --out $S/lattice --summary $S/lattice.json
python3 $B/forced.py $C --steps bounds --out $S/forced --summary $S/forced-bounds.json
```

## Times and order

| Stage | Duration (estimated, idle i7-7700K) | Priority |
|---|---|---|
| 1. export | 2 minutes | first |
| 2. Lean check | 15 minutes | high |
| 3. quick scripts | 15 minutes | high |
| 4. `forced.py` | 1 to 4 hours | high (Table `tab:forced`) |
| 4. `holonomy.py` | 1 hour | medium |
| 4. `strict.py` | 30 to 60 minutes | medium |
| 4. `certify.py` and the Lean check again | 1 hour | medium |
| 4. `forced.py --sound` | under an hour | low |
| 5. aside, `before-cuts` | 20 minutes | lowest |

The summaries of stages 3 and 4 go to `results/`, to be committed with the commit and the machine
they name; the raw output stays under `scratch/data-package/`.
