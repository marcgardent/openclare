#!/usr/bin/env bash
# Makes again the rendered figures of the paper (male-overview.png,
# male-wrist.png, male-eye.png, and their French copies male-*.fr.png) from
# the repository. See README.md.
#
#     docs/research/closing-loops/figures/make_figures.sh
#
# Environment:
#   SOURCE  the male body as the bench gives it to every remesher (default
#           bench/out/human_male/source.obj, see README.md for how to make it)
#   WORK    where the meshes, their measures and the panels go (default
#           target/figures); meshes already there are reused, delete them
#           to make them again
#
# Needs: cargo, blender (5.x), python3 with Pillow. turn.tex and closing.tex
# are drawn by hand and are not made here.
set -euo pipefail

FIGURES=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$FIGURES/../../../.." && pwd)
SOURCE=${SOURCE:-$ROOT/bench/out/human_male/source.obj}
WORK=${WORK:-$ROOT/target/figures}
QUADS=10590
OPENCLARE=$ROOT/target/release/openclare
# Memory caps (KB) for the engine and for Blender.
ulimit -v 24000000

mkdir -p "$WORK/panels"
cd "$ROOT"
echo "engine commit $(git rev-parse --short HEAD), source $(md5sum "$SOURCE" | cut -d' ' -f1)"
cargo build --release

# The four meshes.
if [ ! -f "$WORK/no_closing.obj" ]; then
    "$OPENCLARE" remesh "$SOURCE" "$WORK/no_closing.obj" --preset organic --quads $QUADS --no-close-loops
fi
if [ ! -f "$WORK/closing.obj" ]; then
    "$OPENCLARE" remesh "$SOURCE" "$WORK/closing.obj" --preset organic --quads $QUADS
fi
cp bench/reference/handmade/human_male.obj "$WORK/handmade.obj"
if [ ! -f "$WORK/quadriflow.obj" ]; then
    # Seed 0, scaled by 100 about the origin: see quadriflow_scaled.py.
    blender -b --factory-startup --python "$FIGURES/quadriflow_scaled.py" -- \
        "$SOURCE" "$WORK/quadriflow.obj" $QUADS 0 100
fi

# What `openclare measure` says of each, for the README.
for mesh in no_closing closing handmade quadriflow; do
    "$OPENCLARE" measure "$SOURCE" "$WORK/$mesh.obj" --target $QUADS --json > "$WORK/$mesh.json"
done

# One panel per mesh and view, then the figures.
for view in overview wrist eye; do
    for mesh in no_closing closing handmade quadriflow; do
        blender -b --factory-startup --python "$FIGURES/render_loops.py" -- \
            "$WORK/$mesh.obj" "$WORK/panels/${view}_$mesh.png" $view
    done
    python3 "$FIGURES/compose.py" "$FIGURES/male-$view.png" "$WORK/numbers-$view.json" \
        "$WORK/panels/${view}_no_closing.png" "$WORK/no_closing.obj" "openclare, no closing" \
        "$WORK/panels/${view}_closing.png" "$WORK/closing.obj" "openclare, closing" \
        "$WORK/panels/${view}_handmade.png" "$WORK/handmade.obj" "hand-made" \
        "$WORK/panels/${view}_quadriflow.png" "$WORK/quadriflow.obj" "QuadriFlow"
    # The same panels with French labels, for the French translation of the paper.
    python3 "$FIGURES/compose.py" --lang fr "$FIGURES/male-$view.fr.png" "$WORK/numbers-$view.fr.json" \
        "$WORK/panels/${view}_no_closing.png" "$WORK/no_closing.obj" "openclare, sans fermeture" \
        "$WORK/panels/${view}_closing.png" "$WORK/closing.obj" "openclare, avec fermeture" \
        "$WORK/panels/${view}_handmade.png" "$WORK/handmade.obj" "fait à la main" \
        "$WORK/panels/${view}_quadriflow.png" "$WORK/quadriflow.obj" "QuadriFlow"
done
